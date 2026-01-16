use actix_web::HttpRequest;
use actix_web::http::header;
use rauthy_api_types::oidc::TokenRevocationRequest;
use rauthy_common::utils::base64_decode;
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::refresh_tokens::RefreshToken;
use rauthy_data::entity::refresh_tokens_devices::RefreshTokenDevice;
use rauthy_data::entity::revoked_tokens::RevokedToken;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_jwt::claims::{JwtCommonClaims, JwtTokenType};
use rauthy_jwt::token::JwtToken;

#[inline(always)]
pub async fn handle_token_revocation(
    req: HttpRequest,
    payload: TokenRevocationRequest,
) -> Result<(), ErrorResponse> {
    let (client_id, client_secret) = {
        if let Some(h) = req.headers().get(header::AUTHORIZATION) {
            let decoded =
                if let Some(auth_header) = h.to_str().unwrap_or_default().strip_prefix("Basic ") {
                    String::from_utf8(base64_decode(auth_header)?)?
                } else {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::Unauthorized,
                        "unexpected AUTHORIZATION header",
                    ));
                };
            let Some((client_id, client_secret)) = decoded.split_once(':') else {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Unauthorized,
                    "Bad Authorization header",
                ));
            };
            (client_id.to_string(), client_secret.to_string())
        } else if let Some(client_id) = payload.client_id {
            (client_id, payload.client_secret.unwrap_or_default())
        } else {
            return Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "client_id neither in Authorization header nor in body",
            ));
        }
    };

    let client = Client::find_maybe_ephemeral(client_id).await?;
    if client.confidential {
        client.validate_secret(&client_secret, &req).await?;
    }

    let is_refresh_token = payload.token_type_hint.as_deref() == Some("refresh_token");

    let mut buf: Vec<u8> = Vec::with_capacity(256);
    if is_refresh_token {
        JwtToken::validate_claims_into(&payload.token, Some(JwtTokenType::Refresh), 10, &mut buf)
            .await?;
    } else {
        JwtToken::validate_claims_into(
            &payload.token,
            Some(JwtTokenType::Bearer),
            10,
            buf.as_mut(),
        )
        .await?;
    }
    let claims: JwtCommonClaims = serde_json::from_slice(&buf)?;

    if claims.azp != client.id {
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "mismatch in aud / azp - client_id",
        ));
    }

    let mut revoked_token: Option<RevokedToken> = None;
    if is_refresh_token {
        // by default, the `nbf` for refresh tokens will be
        // exactly 60 seconds before the access token expires
        let disable_nbf = RauthyConfig::get().vars.access.disable_refresh_token_nbf;
        let (_, validation_str) = payload.token.split_at(payload.token.len() - 49);

        if claims.did.is_some() {
            if let Some(rt) = RefreshTokenDevice::find_opt(validation_str).await? {
                rt.delete().await?;

                if let Some(jti) = rt.access_token_jti {
                    revoked_token = Some(RevokedToken {
                        jti,
                        exp: if disable_nbf {
                            rt.exp + 10
                        } else {
                            rt.nbf + 70
                        },
                    });
                }
            }
        } else if let Some(rt) = RefreshToken::find_opt(validation_str).await? {
            rt.delete().await?;

            if let Some(jti) = rt.access_token_jti {
                revoked_token = Some(RevokedToken {
                    jti,
                    exp: if disable_nbf {
                        rt.exp + 10
                    } else {
                        rt.nbf + 70
                    },
                });
            }
        }
    } else {
        let Some(jti) = claims.jti else {
            // We can only revoke new access tokens (at least Rauthy v0.34)
            // which contain the `jti` claim.
            return Ok(());
        };

        revoked_token = Some(RevokedToken {
            jti: jti.to_string(),
            exp: claims.exp + 10,
        });

        if let Some(user_id) = claims.sub {
            if claims.did.is_some() {
                if let Some(rt) = RefreshTokenDevice::find_by_user_id_jti(user_id, jti).await? {
                    rt.delete().await?;
                }
            } else if let Some(rt) = RefreshToken::find_by_user_id_jti(user_id, jti).await? {
                rt.delete().await?;
            }
        }
    };

    if let Some(token) = revoked_token {
        token.upsert_or_delete().await?;
    }

    Ok(())
}
