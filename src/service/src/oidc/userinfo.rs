use crate::oidc::helpers;
use actix_web::HttpRequest;
use actix_web::http::header::{HeaderName, HeaderValue};
use rauthy_api_types::users::Userinfo;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_jwt::claims::{AddressClaim, JwtCommonClaims, JwtTokenType};
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::devices::DeviceEntity;
use rauthy_models::entity::user_attr::UserAttrValueEntity;
use rauthy_models::entity::users::User;
use rauthy_models::entity::users_values::UserValues;
use rauthy_models::entity::webids::WebId;
use rauthy_models::rauthy_config::RauthyConfig;
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

    if claims.sub.is_none() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Not a user token",
        ));
    }

    let scope = claims.scope.unwrap_or_else(|| Cow::from("openid"));
    let user = User::find(claims.sub.unwrap().to_string())
        .await
        .map_err(|_| {
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

        // scope: phone
        phone: None,

        // scope: webid
        webid,
    };

    let mut user_values = None;
    let mut user_values_fetched = false;

    let client = Client::find(claims.azp.to_string()).await.map_err(|_| {
        ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("client-not-found".to_string()),
            "The client has not been found",
        )
    })?;

    let (email, email_verified) = if let Some(cust_email_mapping) = client.cust_email_mapping {
        let attrs = UserAttrValueEntity::find_for_user(&user.id).await.unwrap();

        let attr = attrs.iter().find(|v| v.key == cust_email_mapping).unwrap();

        (String::from_utf8(attr.value.clone()).unwrap(), true)
    } else {
        (user.email.clone(), user.email_verified)
    };

    if scope.contains("email") {
        userinfo.email = Some(email.clone());
        userinfo.email_verified = Some(email_verified);
    }

    if scope.contains("profile") {
        userinfo.preferred_username = Some(email);
        userinfo.given_name = Some(user.given_name.clone());
        userinfo.family_name = user.family_name.clone();
        userinfo.locale = Some(user.language.to_string());

        user_values = UserValues::find(&user.id).await?;
        user_values_fetched = true;

        if let Some(values) = &user_values {
            if let Some(birthdate) = &values.birthdate {
                userinfo.birthdate = Some(birthdate.clone());
            }
        }
    }

    if scope.contains("address") {
        if !user_values_fetched {
            user_values = UserValues::find(&user.id).await?;
            user_values_fetched = true;
        }

        if let Some(values) = &user_values {
            userinfo.address = AddressClaim::try_build(&user, values).map(|claim| claim.into());
        }
    }

    if scope.contains("phone") {
        if !user_values_fetched {
            user_values = UserValues::find(&user.id).await?;
            // user_values_fetched = true;
        }

        if let Some(values) = &user_values {
            if let Some(phone) = &values.phone {
                userinfo.phone = Some(phone.clone());
            }
        }
    }

    Ok((userinfo, cors_header))
}
