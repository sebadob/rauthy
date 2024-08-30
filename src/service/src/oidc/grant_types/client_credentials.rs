use crate::token_set::{DpopFingerprint, TokenSet};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{web, HttpRequest};
use rauthy_api_types::oidc::TokenRequest;
use rauthy_common::constants::HEADER_DPOP_NONCE;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::clients_dyn::ClientDyn;
use rauthy_models::entity::dpop_proof::DPoPProof;
use std::str::FromStr;

#[tracing::instrument(skip_all, fields(client_id = req_data.client_id, username = req_data.username))]
pub async fn grant_type_credentials(
    data: &web::Data<AppState>,
    req: HttpRequest,
    req_data: TokenRequest,
) -> Result<(TokenSet, Vec<(HeaderName, HeaderValue)>), ErrorResponse> {
    if req_data.client_secret.is_none() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "'client_secret' is missing",
        ));
    }

    let (client_id, client_secret) = req_data.try_get_client_id_secret(&req)?;
    let client = Client::find(data, client_id).await?;
    if !client.confidential {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "'client_credentials' flow is allowed for confidential clients only",
        ));
    }
    if !client.enabled {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "client is disabled",
        ));
    }
    let secret = client_secret.ok_or_else(|| {
        ErrorResponse::new(ErrorResponseType::BadRequest, "'client_secret' is missing")
    })?;
    client.validate_secret(&secret, &req)?;
    client.validate_flow("client_credentials")?;
    let header_origin = client.validate_origin(&req, &data.listen_scheme, &data.public_url)?;

    let mut headers = Vec::new();
    let dpop_fingerprint =
        if let Some(proof) = DPoPProof::opt_validated_from(&req, &header_origin).await? {
            if let Some(nonce) = &proof.claims.nonce {
                headers.push((
                    HeaderName::from_str(HEADER_DPOP_NONCE).unwrap(),
                    HeaderValue::from_str(nonce).unwrap(),
                ));
            }
            Some(DpopFingerprint(proof.jwk_fingerprint()?))
        } else {
            None
        };
    // We do not push the origin header, because client credentials should never be used from
    // any browser at all

    // update timestamp if it is a dynamic client
    if client.is_dynamic() {
        ClientDyn::update_used(data, &client.id).await?;
    }

    let ts = TokenSet::for_client_credentials(data, &client, dpop_fingerprint).await?;
    Ok((ts, headers))
}
