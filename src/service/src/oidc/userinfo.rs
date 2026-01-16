use crate::oidc::helpers;
use actix_web::HttpRequest;
use actix_web::http::header::{HeaderName, HeaderValue};
use rauthy_api_types::users::Userinfo;
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::devices::DeviceEntity;
use rauthy_data::entity::revoked_tokens::RevokedToken;
use rauthy_data::entity::users::User;
use rauthy_data::entity::users_values::UserValues;
use rauthy_data::entity::webids::WebId;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_jwt::claims::{AddressClaim, JwtCommonClaims, JwtTokenType};
use std::borrow::Cow;

pub async fn get_userinfo(
    req: HttpRequest,
) -> Result<(Userinfo, Option<(HeaderName, HeaderValue)>), ErrorResponse> {
    let bearer = helpers::get_bearer_token_from_header(req.headers())?;

    let mut buf: Vec<u8> = Vec::with_capacity(256);
    rauthy_jwt::token::JwtToken::validate_claims_into(
        &bearer,
        Some(JwtTokenType::Bearer),
        0,
        buf.as_mut(),
    )
    .await?;
    let claims = serde_json::from_slice::<JwtCommonClaims>(&buf)?;

    let Some(sub) = claims.sub else {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Not a user token",
        ));
    };
    if let Some(jti) = claims.jti
        && RevokedToken::validate_not_revoked(sub, jti).await.is_err()
    {
        return Err(ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("token-revoked".to_string()),
            "The token has been revoked",
        ));
    }

    let scope = claims.scope.unwrap_or_else(|| Cow::from("openid"));
    let user = User::find(sub.to_string()).await.map_err(|_| {
        ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("user-not-found".to_string()),
            "The user has not been found",
        )
    })?;

    // reject the request if user has been disabled, even when the token is still valid
    if !user.enabled || user.check_expired().is_err() {
        return Err(ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("user-disabled".to_string()),
            "The user has been disabled",
        ));
    }

    let cors_header = if RauthyConfig::get().vars.access.userinfo_strict {
        // if the token has been issued to a device, make sure it still exists and is valid
        if let Some(device_id) = claims.did {
            // make sure it still exists
            DeviceEntity::find(device_id).await.map_err(|_| {
                ErrorResponse::new(
                    ErrorResponseType::WWWAuthenticate("user-device-not-found".to_string()),
                    "The user device has not been found",
                )
            })?;
        }

        // make sure the original client still exists and is enabled
        // skip this check if the client is ephemeral
        if !(claims.azp.starts_with("http://") || claims.azp.starts_with("https://")) {
            let client = Client::find(claims.azp.to_string()).await.map_err(|_| {
                ErrorResponse::new(
                    ErrorResponseType::WWWAuthenticate("client-not-found".to_string()),
                    "The client has not been found",
                )
            })?;
            if !client.enabled {
                return Err(ErrorResponse::new(
                    ErrorResponseType::WWWAuthenticate("client-disabled".to_string()),
                    "The client has been disabled",
                ));
            }
            client.get_validated_origin_header(&req)?
        } else {
            None
        }
    } else {
        None
    };

    let roles = user.get_roles();
    let groups = scope.contains("groups").then(|| user.get_groups());
    let webid = (RauthyConfig::get().vars.ephemeral_clients.enable_web_id
        && scope.contains("webid"))
    .then(|| WebId::resolve_webid_uri(&user.id));

    let mut userinfo = Userinfo {
        id: user.id.clone(),
        sub: user.id.clone(),
        name: user.email_recipient_name(),
        roles,
        mfa_enabled: user.has_webauthn_enabled(),

        // scope: address
        address: None,

        // scope: email
        email: None,
        email_verified: None,

        // scope: groups
        groups,

        // scope: profile
        preferred_username: None,
        given_name: None,
        family_name: None,
        locale: None,
        birthdate: None,
        picture: None,
        zoneinfo: None,

        // scope: phone
        phone: None,

        // scope: webid
        webid,
    };

    let has_email = scope.contains("email");
    let has_profile = scope.contains("profile");
    let has_addr = scope.contains("address");
    let has_phone = scope.contains("phone");

    if (has_profile || has_addr || has_phone)
        && let Some(values) = UserValues::find(&user.id).await?
    {
        if has_addr {
            userinfo.address = AddressClaim::try_build(&user, &values).map(|claim| claim.into());
        }

        if has_profile {
            userinfo.birthdate = values.birthdate;
            userinfo.zoneinfo = values.tz;

            if let Some(username) = values.preferred_username {
                userinfo.preferred_username = Some(username);
            } else if RauthyConfig::get()
                .vars
                .user_values
                .preferred_username
                .email_fallback
            {
                userinfo.preferred_username = userinfo.email.clone();
            }
        }

        if has_phone {
            userinfo.phone = values.phone;
        }
    }

    if has_email {
        userinfo.email = Some(user.email);
        userinfo.email_verified = Some(user.email_verified);
    }

    if has_profile {
        userinfo.given_name = Some(user.given_name);
        userinfo.family_name = user.family_name;
        userinfo.locale = Some(user.language.to_string());
        userinfo.picture = user.picture_id;
    }

    Ok((userinfo, cors_header))
}
