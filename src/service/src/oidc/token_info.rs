use crate::oidc::validation;
use crate::oidc::validation::validate_token;
use actix_web::http::header::{AUTHORIZATION, HeaderName, HeaderValue};
use actix_web::{HttpRequest, web};
use rauthy_api_types::oidc::TokenInfo;
use rauthy_common::constants::DANGER_DISABLE_INTROSPECT_AUTH;
use rauthy_common::utils::base64_decode;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::clients::Client;
use rauthy_models::{JwtAccessClaims, JwtCommonClaims};
use tracing::error;

pub async fn get_token_info(
    data: &web::Data<AppState>,
    req: &HttpRequest,
    token: &str,
) -> Result<(TokenInfo, Option<(HeaderName, HeaderValue)>), ErrorResponse> {
    let claims_res = validation::validate_token::<JwtCommonClaims>(data, token, None).await;
    if claims_res.is_err() {
        return Ok((
            TokenInfo {
                active: false,
                ..Default::default()
            },
            None,
        ));
    }
    let claims = claims_res?;

    if claims.audiences.is_none() {
        error!("'aud' claim does not exist when it always should");
        return Ok((
            TokenInfo {
                active: false,
                ..Default::default()
            },
            None,
        ));
    }

    let client = check_client_auth(data, req, claims.custom.azp).await?;
    let cors_header = client.get_validated_origin_header(req)?;

    let aud_set = claims.audiences.unwrap().into_set();
    let aud = aud_set.into_iter().collect::<Vec<_>>().first().cloned();

    Ok((
        TokenInfo {
            active: true,
            sub: claims.subject,
            scope: claims.custom.scope,
            client_id: Some(client.id),
            aud,
            username: claims.custom.preferred_username,
            iat: claims.issued_at.map(|ts| ts.as_secs()),
            nbf: claims.invalid_before.map(|ts| ts.as_secs()),
            exp: claims.expires_at.map(|ts| ts.as_secs()),
            cnf: claims.custom.cnf,
        },
        cors_header,
    ))
}

#[inline]
async fn check_client_auth(
    data: &web::Data<AppState>,
    req: &HttpRequest,
    client_id: String,
) -> Result<Client, ErrorResponse> {
    let client = Client::find(client_id).await.map_err(|_| {
        ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("client-not-found".to_string()),
            "client does not exist anymore",
        )
    })?;

    if *DANGER_DISABLE_INTROSPECT_AUTH {
        return Ok(client);
    }

    let header_value = match req.headers().get(AUTHORIZATION) {
        None => {
            return Err(ErrorResponse::new(
                ErrorResponseType::WWWAuthenticate("authorization-header-missing".to_string()),
                "Authorization header is missing",
            ));
        }
        Some(h) => h,
    };
    let header = header_value.to_str().unwrap_or_default();

    if !client.enabled {
        return Err(ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("client-disabled".to_string()),
            "client has been disabled",
        ));
    }

    if let Some(token) = header.strip_prefix("Bearer ") {
        validate_token::<JwtAccessClaims>(data, token, None).await?;
        Ok(client)
    } else if let Some(basic) = header.strip_prefix("Basic ") {
        let bytes = base64_decode(basic)?;
        let decoded = String::from_utf8_lossy(&bytes);
        let (id, secret) = match decoded.split_once(':') {
            None => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::WWWAuthenticate("invalid-authorization-header".to_string()),
                    "invalid Authorization header: cannot split into client_id:client_secret",
                ));
            }
            Some(split) => split,
        };

        if id != client.id {
            return Err(ErrorResponse::new(
                ErrorResponseType::WWWAuthenticate("invalid-client-id".to_string()),
                "'client_id' from token does not match the one from the Authorization header",
            ));
        }

        client.validate_secret(secret, req)?;
        Ok(client)
    } else {
        Err(ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("invalid-authorization-header".to_string()),
            "invalid AUTHORIZATION header",
        ))
    }
}
