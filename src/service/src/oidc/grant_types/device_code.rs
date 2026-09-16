use crate::token_set::{AuthCodeFlow, AuthTime, DeviceCodeFlow, TokenNonce, TokenScopes, TokenSet};
use actix_web::HttpResponse;
use chrono::Utc;
use rauthy_api_types::oidc::{OAuth2ErrorResponse, OAuth2ErrorTypeResponse, TokenRequest};
use rauthy_common::utils::new_store_id;
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::devices::{DeviceAuthCode, DeviceEntity};
use rauthy_data::entity::users::User;
use rauthy_data::events::event::Event;
use rauthy_data::rauthy_config::RauthyConfig;
use std::borrow::Cow;
use std::net::IpAddr;
use std::ops::{Add, Sub};
use tracing::{debug, error, warn};

/// Return a [TokenSet](crate::models::response::TokenSet) for the `device_code` flow
#[tracing::instrument(skip_all, fields(client_id = payload.client_id))]
pub async fn grant_type_device_code(peer_ip: IpAddr, payload: TokenRequest) -> HttpResponse {
    let device_code = match &payload.device_code {
        None => {
            return HttpResponse::BadRequest().json(OAuth2ErrorResponse {
                error: OAuth2ErrorTypeResponse::InvalidRequest,
                error_description: Some(Cow::from("`device_code` is missing")),
            });
        }
        Some(dc) => dc,
    };

    let mut code = match DeviceAuthCode::find_by_device_code(device_code).await {
        Ok(Some(code)) => code,
        Ok(None) | Err(_) => {
            return HttpResponse::BadRequest().json(OAuth2ErrorResponse {
                error: OAuth2ErrorTypeResponse::ExpiredToken,
                error_description: Some(Cow::from("invalid `device_code` or request has expired")),
            });
        }
    };

    if Some(code.client_id.as_str()) != payload.client_id.as_deref() {
        return HttpResponse::BadRequest().json(OAuth2ErrorResponse {
            error: OAuth2ErrorTypeResponse::InvalidRequest,
            error_description: Some(Cow::from("Invalid `client_id`")),
        });
    }

    // We need to check the device_code again, because the `find_by_device_code` uses the
    // `user_code` as cache index under the hood for smaller footprints and the ability to find it
    // in both ways without duplicated data.
    //
    // The constant time comparison for both the device code and the client secret don't make
    // any sense in terms of security here, but I don't want any other brain-dead AI security report
    // about it.
    //
    // The device code is very short-lived, rate-limited and even deleted after 3 times rate-limit
    // abuse. The client secret comparison will never be reached until the full device code is valid.
    // Apart from that, this comparison happens in single digit nanoseconds and is practically
    // impossible to measure in this API. Scheduling an async task that is ready for work takes even
    // longer than that, and we won't even talk about network latency!
    //
    // This means from a security standpoint, constant time comparison does not make any difference.
    // We are doing it for best practice only, just don't annoy me with this anymore. Use some
    // common sense instead of blindly reporting AI findings and saying "that's a score of 7.4!".
    if !constant_time_eq::constant_time_eq(code.device_code.as_bytes(), device_code.as_bytes()) {
        return HttpResponse::BadRequest().json(OAuth2ErrorResponse {
            error: OAuth2ErrorTypeResponse::UnauthorizedClient,
            error_description: Some(Cow::from("Invalid `device_code`")),
        });
    }

    if !constant_time_eq::constant_time_eq(
        code.client_secret
            .as_ref()
            .map(|s| s.as_bytes())
            .unwrap_or_default(),
        payload
            .client_secret
            .as_ref()
            .map(|s| s.as_bytes())
            .unwrap_or_default(),
    ) {
        return HttpResponse::BadRequest().json(OAuth2ErrorResponse {
            error: OAuth2ErrorTypeResponse::UnauthorizedClient,
            error_description: Some(Cow::from("Invalid `client_secret`")),
        });
    }

    debug!("device oidc code poll request is valid");
    let mut error = OAuth2ErrorTypeResponse::AuthorizationPending;
    let mut error_description = Cow::default();

    // Check last_poll and make sure interval is being respected.
    // We allow it to be 500ms shorter than specified to not get into
    // possible problems with slightly inaccurate client implementations.
    let now = Utc::now();
    let interval = RauthyConfig::get().vars.device_grant.poll_interval as i64;
    let poll_thres = now
        .sub(chrono::Duration::seconds(interval))
        .add(chrono::Duration::milliseconds(500));
    if poll_thres < code.last_poll {
        warn!("device does not respect the poll interval");
        code.warnings += 1;
        if code.warnings >= 3 {
            warn!("deleting device oidc code request early because of not respected poll interval");
            error = OAuth2ErrorTypeResponse::AccessDenied;
            error_description = Cow::from("poll interval has not been respected");
            if let Err(err) = code.delete().await {
                // this should never happen
                error!(?err, "deleting DeviceAuthCode from the cache");
            }
        } else {
            error = OAuth2ErrorTypeResponse::SlowDown;
            error_description = Cow::from("must respect the poll interval");
        }
    }

    // check validation
    if let Some(verified_by) = &code.verified_by {
        let user = match User::find(verified_by.clone()).await {
            Ok(user) => user,
            Err(err) => {
                // at this point, this should never fail - only if the DB went down in the meantime
                error!("{:?}", err);
                return HttpResponse::InternalServerError().json(OAuth2ErrorResponse {
                    error: OAuth2ErrorTypeResponse::InvalidRequest,
                    error_description: Some(Cow::from(err.to_string())),
                });
            }
        };

        let client = match Client::find(code.client_id.clone()).await {
            Ok(client) => client,
            Err(err) => {
                // at this point, this should never fail - only if the DB went down in the meantime
                error!("{:?}", err);
                return HttpResponse::InternalServerError().json(OAuth2ErrorResponse {
                    error: OAuth2ErrorTypeResponse::InvalidRequest,
                    error_description: Some(Cow::from(err.to_string())),
                });
            }
        };

        let access_exp = now.add(chrono::Duration::seconds(
            client.access_token_lifetime as i64,
        ));
        let refresh_exp = if client.allow_refresh_token() {
            Some(
                access_exp
                    .add(chrono::Duration::seconds(48 * 3600))
                    .timestamp(),
            )
        } else {
            None
        };

        // Claim immediately before issuing tokens.
        let code: DeviceAuthCode = match DeviceAuthCode::find(code.user_code().to_string()).await {
            Ok(Some(code)) => code,
            Ok(None) => {
                return HttpResponse::BadRequest().json(OAuth2ErrorResponse {
                    error: OAuth2ErrorTypeResponse::ExpiredToken,
                    error_description: Some(Cow::from(
                        "invalid `device_code` or request has expired",
                    )),
                });
            }
            Err(err) => {
                error!(?err, "claiming DeviceAuthCode");
                return HttpResponse::InternalServerError().json(OAuth2ErrorResponse {
                    error: OAuth2ErrorTypeResponse::InvalidRequest,
                    error_description: Some(Cow::from("internal error")),
                });
            }
        };

        let id = new_store_id();
        let device = DeviceEntity {
            id: id.clone(),
            client_id: code.client_id,
            user_id: Some(user.id.clone()),
            created: now.timestamp(),
            access_exp: access_exp.timestamp(),
            refresh_exp,
            peer_ip: peer_ip.to_string(),
            // The very first name will just always be the id.
            // This is a better UX than asking for a custom name each time.
            // TODO add an optional `name` param to the initial device request?
            name: id.clone(),
        };
        if let Err(err) = device.insert().await {
            error!("{:?}", err);
            return HttpResponse::InternalServerError().json(OAuth2ErrorResponse {
                error: OAuth2ErrorTypeResponse::InvalidRequest,
                error_description: Some(Cow::from(err.to_string())),
            });
        }
        debug!("New Device with ID {id} has been created");

        let ts = match TokenSet::from_user(
            &user,
            &client,
            AuthTime::now(),
            None,
            code.nonce.map(TokenNonce),
            code.scopes.map(TokenScopes),
            None,
            // resource indicators are not supported for the device flow yet
            None,
            AuthCodeFlow::No,
            DeviceCodeFlow::Yes(id),
        )
        .await
        {
            Ok(ts) => ts,
            Err(err) => {
                error!(?err, "Building Device TokenSet");
                return HttpResponse::InternalServerError().json(OAuth2ErrorResponse {
                    error: OAuth2ErrorTypeResponse::InvalidRequest,
                    error_description: Some(Cow::from(err.to_string())),
                });
            }
        };

        if RauthyConfig::get().vars.events.generate_token_issued
            && let Err(err) = Event::token_issued("device_code", &client.id, Some(&user.email))
                .send()
                .await
        {
            error!(?err, "Cannot create device_code token issued event");
        }

        return HttpResponse::Ok().json(ts);
    }

    code.last_poll = now;
    if let Err(err) = code.save().await {
        // this should never happen
        error!(?err, "Error saving the DeviceAuthCode");
    }

    HttpResponse::BadRequest().json(OAuth2ErrorResponse {
        error,
        error_description: Some(error_description),
    })
}
