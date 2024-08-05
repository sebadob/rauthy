use crate::oidc::validation;
use crate::oidc::validation::validate_token;
use actix_web::http::header::AUTHORIZATION;
use actix_web::{web, HttpRequest};
use rauthy_api_types::oidc::TokenInfo;
use rauthy_common::constants::DANGER_DISABLE_INTROSPECT_AUTH;
use rauthy_common::utils::base64_decode;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::clients::Client;
use rauthy_models::{JwtAccessClaims, JwtCommonClaims};

/// Returns [TokenInfo](crate::models::response::TokenInfo) for the
/// [/oidc/introspect endpoint](crate::handlers::post_token_info)
pub async fn get_token_info(
    data: &web::Data<AppState>,
    req: &HttpRequest,
    token: &str,
) -> Result<TokenInfo, ErrorResponse> {
    let claims_res = validation::validate_token::<JwtCommonClaims>(data, token).await;
    if claims_res.is_err() {
        return Ok(TokenInfo {
            active: false,
            scope: None,
            client_id: None,
            username: None,
            exp: None,
            cnf: None,
        });
    }
    let claims = claims_res.unwrap();

    let client_id = check_client_auth(data, req, claims.custom.azp).await?;

    // scope does not exist for ID tokens, for all others unwrap is safe
    let scope = claims.custom.scope;
    let username = claims.subject;
    let exp = claims.expires_at.unwrap().as_secs();
    let cnf = claims.custom.cnf;

    Ok(TokenInfo {
        active: true,
        scope,
        client_id: Some(client_id),
        username,
        exp: Some(exp),
        cnf,
    })
}

#[inline]
async fn check_client_auth(
    data: &web::Data<AppState>,
    req: &HttpRequest,
    client_id: String,
) -> Result<String, ErrorResponse> {
    if *DANGER_DISABLE_INTROSPECT_AUTH {
        return Ok(client_id);
    }

    let header_value = match req.headers().get(AUTHORIZATION) {
        None => {
            return Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "AUTHORIZATION header is missing",
            ));
        }
        Some(h) => h,
    };
    let header = header_value.to_str().unwrap_or_default();

    if let Some(token) = header.strip_prefix("Bearer ") {
        validate_token::<JwtAccessClaims>(data, token).await?;
        Ok(client_id)
    } else if let Some(basic) = header.strip_prefix("Basic ") {
        let bytes = base64_decode(basic)?;
        let decoded = String::from_utf8_lossy(&bytes);
        let (id, secret) = match decoded.split_once(':') {
            None => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Unauthorized,
                    "invalid AUTHORIZATION header: cannot split into client_id:client_secret",
                ));
            }
            Some(split) => split,
        };

        if id != client_id {
            return Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "'client_id' from token does not match the one from the AUTHORIZATION header",
            ));
        }

        let client = Client::find(data, client_id).await?;
        client.validate_secret(secret, req)?;
        Ok(client.id)
    } else {
        Err(ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            "invalid AUTHORIZATION header",
        ))
    }
}
