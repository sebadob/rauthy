use actix_web::HttpRequest;
use actix_web::http::header::{AUTHORIZATION, HeaderName, HeaderValue};
use rauthy_api_types::oidc::TokenInfo;
use rauthy_common::utils::base64_decode_buf;
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::revoked_tokens::RevokedToken;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_jwt::claims::{JwtAccessClaims, JwtCommonClaims, JwtTokenType};
use rauthy_jwt::token::JwtToken;
use tracing::error;

pub async fn get_token_info(
    req: &HttpRequest,
    token: &str,
) -> Result<(String, Option<(HeaderName, HeaderValue)>), ErrorResponse> {
    let mut buf = Vec::with_capacity(512);
    if JwtToken::validate_claims_into(token, Some(JwtTokenType::Bearer), 0, &mut buf)
        .await
        .is_err()
    {
        return Ok((
            serde_json::to_string(&TokenInfo {
                active: false,
                ..Default::default()
            })?,
            None,
        ));
    }
    let claims = serde_json::from_slice::<JwtCommonClaims>(&buf)?;

    if let Some(sub) = claims.sub
        && let Some(jti) = claims.jti
    {
        RevokedToken::validate_not_revoked(sub, jti).await?;
    }

    if claims.aud.is_empty() {
        error!("'aud' claim does not exist when it always should");
        return Ok((
            serde_json::to_string(&TokenInfo {
                active: false,
                ..Default::default()
            })?,
            None,
        ));
    }
    let client_id = claims.azp.to_string();

    // serialize token already before checking client to be able to re-use `buf`
    let info = serde_json::to_string(&TokenInfo {
        active: true,
        sub: claims.sub,
        scope: claims.scope,
        client_id: Some(claims.azp),
        aud: Some(claims.aud.as_ref()),
        iat: Some(claims.iat),
        nbf: Some(claims.nbf),
        exp: Some(claims.exp),
        cnf: claims.cnf,
    })?;

    buf.clear();
    let client = check_client_auth(req, client_id, &mut buf).await?;
    client.validate_enabled()?;
    let cors_header = client.get_validated_origin_header(req)?;

    Ok((info, cors_header))
}

#[inline]
async fn check_client_auth(
    req: &HttpRequest,
    client_id: String,
    buf: &mut Vec<u8>,
) -> Result<Client, ErrorResponse> {
    debug_assert!(buf.is_empty());

    let client = Client::find(client_id).await.map_err(|_| {
        ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("client-not-found".to_string()),
            "client does not exist anymore",
        )
    })?;

    if RauthyConfig::get()
        .vars
        .access
        .danger_disable_introspect_auth
    {
        return Ok(client);
    }

    let Some(header_value) = req.headers().get(AUTHORIZATION) else {
        return Err(ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("authorization-header-missing".to_string()),
            "Authorization header is missing",
        ));
    };
    let header = header_value.to_str().unwrap_or_default();

    if !client.enabled {
        return Err(ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("client-disabled".to_string()),
            "client has been disabled",
        ));
    }

    if let Some(token) = header.strip_prefix("Bearer ") {
        JwtToken::validate_claims_into(token, Some(JwtTokenType::Bearer), 0, buf).await?;
        // Just make sure it deserializes fine
        serde_json::from_slice::<JwtAccessClaims>(buf)?;
        Ok(client)
    } else if let Some(basic) = header.strip_prefix("Basic ") {
        base64_decode_buf(basic, buf)?;
        let decoded = String::from_utf8_lossy(buf);
        let Some((id, secret)) = decoded.split_once(':') else {
            return Err(ErrorResponse::new(
                ErrorResponseType::WWWAuthenticate("invalid-authorization-header".to_string()),
                "invalid Authorization header: cannot split into client_id:client_secret",
            ));
        };

        if id != client.id {
            return Err(ErrorResponse::new(
                ErrorResponseType::WWWAuthenticate("invalid-client-id".to_string()),
                "'client_id' from token does not match the one from the Authorization header",
            ));
        }

        client.validate_secret(secret, req).await?;
        Ok(client)
    } else {
        Err(ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("invalid-authorization-header".to_string()),
            "invalid AUTHORIZATION header",
        ))
    }
}
