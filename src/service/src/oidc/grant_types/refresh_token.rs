use crate::oidc::validation;
use crate::token_set::TokenSet;
use actix_web::http::header::{
    ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_METHODS, HeaderName, HeaderValue,
};
use actix_web::{HttpRequest, web};
use rauthy_api_types::oidc::TokenRequest;
use rauthy_common::constants::HEADER_DPOP_NONCE;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::clients::Client;
use std::str::FromStr;

#[tracing::instrument(skip_all, fields(client_id = req_data.client_id, username = req_data.username))]
pub async fn grant_type_refresh(
    data: &web::Data<AppState>,
    req: HttpRequest,
    req_data: TokenRequest,
) -> Result<(TokenSet, Vec<(HeaderName, HeaderValue)>), ErrorResponse> {
    if req_data.refresh_token.is_none() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "'refresh_token' is missing",
        ));
    }
    let (client_id, client_secret) = req_data.try_get_client_id_secret(&req)?;
    let client = Client::find_maybe_ephemeral(client_id).await?;

    let header_origin = client.get_validated_origin_header(&req)?;

    if client.confidential {
        let secret = client_secret.ok_or_else(|| {
            ErrorResponse::new(ErrorResponseType::BadRequest, "'client_secret' is missing")
        })?;
        client.validate_secret(&secret, &req).await?;
    }

    client.validate_flow("refresh_token")?;

    let refresh_token = req_data.refresh_token.unwrap();

    // validate common refresh token claims first and get the payload
    let (ts, dpop_none) =
        validation::validate_refresh_token(Some(client), &refresh_token, data, &req).await?;

    let mut headers = Vec::new();
    if let Some(h) = header_origin {
        headers.push(h);
        headers.push((
            ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("POST"),
        ));
        headers.push((
            ACCESS_CONTROL_ALLOW_CREDENTIALS,
            HeaderValue::from_static("true"),
        ));
    }
    if let Some(nonce) = dpop_none {
        headers.push((
            HeaderName::from_str(HEADER_DPOP_NONCE).unwrap(),
            HeaderValue::from_str(&nonce)?,
        ));
    }

    Ok((ts, headers))
}
