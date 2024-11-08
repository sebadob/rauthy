use crate::oidc::{helpers, validation};
use actix_web::{web, HttpRequest};
use rauthy_api_types::users::Userinfo;
use rauthy_common::constants::{ENABLE_WEB_ID, USERINFO_STRICT};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::devices::DeviceEntity;
use rauthy_models::entity::users::User;
use rauthy_models::entity::users_values::UserValues;
use rauthy_models::entity::webids::WebId;
use rauthy_models::{AddressClaim, JwtCommonClaims, JwtTokenType};

/// Returns the 'userInfo' for the [/oidc/userinfo endpoint](crate::handlers::get_userinfo)<br>
pub async fn get_userinfo(
    data: &web::Data<AppState>,
    req: HttpRequest,
) -> Result<Userinfo, ErrorResponse> {
    // get bearer token
    let bearer = helpers::get_bearer_token_from_header(req.headers())?;

    let claims = validation::validate_token::<JwtCommonClaims>(data, &bearer).await?;
    if claims.custom.typ != JwtTokenType::Bearer {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Token Type must be 'Bearer'",
        ));
    }

    let scope = claims.custom.scope.unwrap_or_else(|| "openid".to_string());
    let uid = claims.subject.ok_or_else(|| {
        ErrorResponse::new(
            ErrorResponseType::Internal,
            "Token without 'sub' - could not extract the Principal",
        )
    })?;
    let user = User::find(uid).await.map_err(|_| {
        ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("user-not-found".to_string()),
            "The user has not been found".to_string(),
        )
    })?;

    // reject the request if user has been disabled, even when the token is still valid
    if !user.enabled || user.check_expired().is_err() {
        return Err(ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("user-disabled".to_string()),
            "The user has been disabled".to_string(),
        ));
    }

    if *USERINFO_STRICT {
        // if the token has been issued to a device, make sure it still exists and is valid
        if let Some(device_id) = claims.custom.did {
            // just make sure it still exists
            DeviceEntity::find(&device_id).await.map_err(|_| {
                ErrorResponse::new(
                    ErrorResponseType::WWWAuthenticate("user-device-not-found".to_string()),
                    "The user device has not been found".to_string(),
                )
            })?;
        }

        // make sure the original client still exists and is enabled
        // skip this check if the client is ephemeral
        if !(claims.custom.azp.starts_with("http://") || claims.custom.azp.starts_with("https://"))
        {
            let client = Client::find(claims.custom.azp).await.map_err(|_| {
                ErrorResponse::new(
                    ErrorResponseType::WWWAuthenticate("client-not-found".to_string()),
                    "The client has not been found".to_string(),
                )
            })?;

            if !client.enabled {
                return Err(ErrorResponse::new(
                    ErrorResponseType::WWWAuthenticate("client-disabled".to_string()),
                    "The client has been disabled".to_string(),
                ));
            }
        }
    }

    let roles = user.get_roles();
    let groups = scope.contains("groups").then(|| user.get_groups());
    let webid =
        (*ENABLE_WEB_ID && scope.contains("webid")).then(|| WebId::resolve_webid_uri(&user.id));

    let mut userinfo = Userinfo {
        id: user.id.clone(),
        sub: user.id.clone(),
        name: format!("{} {}", &user.given_name, &user.family_name),
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

    if scope.contains("email") {
        userinfo.email = Some(user.email.clone());
        userinfo.email_verified = Some(user.email_verified);
    }

    let mut user_values = None;
    let mut user_values_fetched = false;

    if scope.contains("profile") {
        userinfo.preferred_username = Some(user.email.clone());
        userinfo.given_name = Some(user.given_name.clone());
        userinfo.family_name = Some(user.family_name.clone());
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

    Ok(userinfo)
}
