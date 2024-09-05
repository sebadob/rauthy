use crate::token_set::{AuthCodeFlow, AuthTime, DeviceCodeFlow, DpopFingerprint, TokenSet};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{web, HttpRequest};
use rauthy_api_types::oidc::TokenRequest;
use rauthy_common::constants::HEADER_DPOP_NONCE;
use rauthy_common::password_hasher::HashPassword;
use rauthy_common::utils::real_ip_from_req;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::clients_dyn::ClientDyn;
use rauthy_models::entity::dpop_proof::DPoPProof;
use rauthy_models::entity::users::User;
use std::str::FromStr;
use time::OffsetDateTime;
use tracing::{info, warn};

#[tracing::instrument(skip_all, fields(client_id = req_data.client_id, username = req_data.username))]
pub async fn grant_type_password(
    data: &web::Data<AppState>,
    req: HttpRequest,
    req_data: TokenRequest,
) -> Result<(TokenSet, Vec<(HeaderName, HeaderValue)>), ErrorResponse> {
    if req_data.username.is_none() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Missing 'username'",
        ));
    }
    if req_data.password.is_none() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Missing 'password",
        ));
    }

    let (client_id, client_secret) = req_data.try_get_client_id_secret(&req)?;
    let email = req_data.username.as_ref().unwrap();
    let password = req_data.password.unwrap();

    let client = Client::find(data, client_id).await?;
    let header_origin = client.validate_origin(&req, &data.listen_scheme, &data.public_url)?;
    if client.confidential {
        let secret = client_secret.ok_or_else(|| {
            ErrorResponse::new(ErrorResponseType::BadRequest, "Missing 'client_secret'")
        })?;
        client.validate_secret(&secret, &req)?;
    }
    client.validate_flow("password")?;

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
    if let Some(h) = header_origin {
        headers.push(h);
    }

    // This Error must be the same if user does not exist AND passwords do not match to prevent
    // username enumeration
    let mut user = User::find_by_email(data, String::from(email)).await?;
    user.check_enabled()?;
    user.check_expired()?;

    match user.validate_password(data, password.clone()).await {
        Ok(_) => {
            user.last_login = Some(OffsetDateTime::now_utc().unix_timestamp());
            user.last_failed_login = None;
            user.failed_login_attempts = None;

            // check if the password hash should be upgraded
            let hash_uptodate = user.is_argon2_uptodate(&data.argon2_params)?;
            if !hash_uptodate {
                info!("Updating Argon2ID params for user '{}'", &user.email);
                let new_hash = HashPassword::hash_password(password).await?;
                // let new_hash = User::new_password_hash(&password, params).await?;
                user.password = Some(new_hash);
            }

            user.save(data, None, None).await?;

            // update timestamp if it is a dynamic client
            if client.is_dynamic() {
                ClientDyn::update_used(data, &client.id).await?;
            }

            let ts = TokenSet::from_user(
                &user,
                data,
                &client,
                AuthTime::now(),
                dpop_fingerprint,
                None,
                None,
                AuthCodeFlow::No,
                DeviceCodeFlow::No,
            )
            .await?;
            Ok((ts, headers))
        }
        Err(err) => {
            warn!(
                "False Login attempt from Host: '{}' for user: '{}'",
                real_ip_from_req(&req)?,
                user.email
            );

            user.last_failed_login = Some(OffsetDateTime::now_utc().unix_timestamp());
            user.failed_login_attempts = Some(&user.failed_login_attempts.unwrap_or(0) + 1);

            user.save(data, None, None).await?;

            // TODO add expo increasing sleeps after failed login attempts here?
            Err(err)
        }
    }
}
