use crate::token_set::{AuthCodeFlow, AuthTime, DeviceCodeFlow, TokenScopes, TokenSet};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use rauthy_api_types::oidc::{OAuth2ErrorResponse, OAuth2ErrorTypeResponse, TokenRequest};
use rauthy_common::constants::DEVICE_GRANT_POLL_INTERVAL;
use rauthy_common::utils::new_store_id;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::devices::{DeviceAuthCode, DeviceEntity};
use rauthy_models::entity::users::User;
use std::borrow::Cow;
use std::net::IpAddr;
use std::ops::{Add, Sub};
use tracing::{debug, error, warn};

/// Return a [TokenSet](crate::models::response::TokenSet) for the `device_code` flow
#[tracing::instrument(skip_all, fields(client_id = payload.client_id))]
pub async fn grant_type_device_code(
    data: &web::Data<AppState>,
    peer_ip: IpAddr,
    payload: TokenRequest,
) -> HttpResponse {
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

    // We need to check the device_code again, because the `find_by_device_code` uses
    // the `user_code` as cache index under the hood for smaller footprints and the
    // ability to find it in both ways without duplicated data.
    if &code.device_code != device_code {
        return HttpResponse::BadRequest().json(OAuth2ErrorResponse {
            error: OAuth2ErrorTypeResponse::UnauthorizedClient,
            error_description: Some(Cow::from("Invalid `device_code`")),
        });
    }

    if code.client_secret != payload.client_secret {
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
    let poll_thres = now
        .sub(chrono::Duration::seconds(
            *DEVICE_GRANT_POLL_INTERVAL as i64,
        ))
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
                error!("Error deleting DeviceAuthCode from the cache: {:}", err);
            }
        } else {
            error = OAuth2ErrorTypeResponse::SlowDown;
            error_description = Cow::from("must respect the poll interval");
        }
    }

    // check validation
    if let Some(verified_by) = &code.verified_by {
        let user = match User::find(data, verified_by.clone()).await {
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

        let client = match Client::find(data, code.client_id.clone()).await {
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

        if let Err(err) = code.delete().await {
            // should really never happen - in cache only
            error!("Error deleting DeviceAuthCode: {:?}", err);
        }

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
            name: id,
        };
        if let Err(err) = device.insert(data).await {
            error!("{:?}", err);
            return HttpResponse::InternalServerError().json(OAuth2ErrorResponse {
                error: OAuth2ErrorTypeResponse::InvalidRequest,
                error_description: Some(Cow::from(err.to_string())),
            });
        }
        debug!("New Device has been created: {:?}", device);

        let ts = match TokenSet::from_user(
            &user,
            data,
            &client,
            AuthTime::now(),
            None,
            None,
            code.scopes.map(TokenScopes),
            AuthCodeFlow::No,
            DeviceCodeFlow::Yes(device.id),
        )
        .await
        {
            Ok(ts) => ts,
            Err(err) => {
                error!("Building Device TokenSet: {:?}", err);
                return HttpResponse::InternalServerError().json(OAuth2ErrorResponse {
                    error: OAuth2ErrorTypeResponse::InvalidRequest,
                    error_description: Some(Cow::from(err.to_string())),
                });
            }
        };

        return HttpResponse::Ok().json(ts);
    }

    code.last_poll = now;
    if let Err(err) = code.save().await {
        // this should never happen
        error!("Error saving the DeviceAuthCode: {:?}", err);
    }

    HttpResponse::BadRequest().json(OAuth2ErrorResponse {
        error,
        error_description: Some(error_description),
    })
}
