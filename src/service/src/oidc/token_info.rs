use actix_web::HttpRequest;
use actix_web::http::header::{AUTHORIZATION, HeaderName, HeaderValue};
use rauthy_api_types::oidc::TokenInfo;
use rauthy_common::utils::base64_decode_buf;
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::issued_tokens::IssuedToken;
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

    if let Some(jti) = claims.jti {
        IssuedToken::validate_not_revoked(jti).await?;
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

    let claims_client = find_enabled_client(client_id).await?;

    if RauthyConfig::get()
        .vars
        .access
        .danger_disable_introspect_auth
    {
        return Ok(claims_client);
    }

    let Some(header_value) = req.headers().get(AUTHORIZATION) else {
        return Err(ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("authorization-header-missing".to_string()),
            "Authorization header is missing",
        ));
    };
    let header = header_value.to_str().unwrap_or_default();

    if let Some(token) = header.strip_prefix("Bearer ") {
        JwtToken::validate_claims_into(token, Some(JwtTokenType::Bearer), 0, buf).await?;
        let claims = serde_json::from_slice::<JwtAccessClaims>(buf)?;

        // If a different client was used for authentication, make sure it exists and is enabled.
        if claims.common.azp != claims_client.id {
            let client = find_enabled_client(claims.common.azp.to_string()).await?;
            // no need to validate the secret - the valid token was the authentication
            Ok(client)
        } else {
            Ok(claims_client)
        }
    } else if let Some(basic) = header.strip_prefix("Basic ") {
        base64_decode_buf(basic, buf)?;
        let decoded = String::from_utf8_lossy(buf);
        let Some((id_header, secret)) = decoded.split_once(':') else {
            return Err(ErrorResponse::new(
                ErrorResponseType::WWWAuthenticate("invalid-authorization-header".to_string()),
                "invalid Authorization header: cannot split into client_id:client_secret",
            ));
        };

        let client = if id_header != claims_client.id {
            find_enabled_client(id_header.to_string()).await?
        } else {
            claims_client
        };
        client.validate_secret(secret, req).await?;

        Ok(client)
    } else {
        Err(ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("invalid-authorization-header".to_string()),
            "invalid AUTHORIZATION header",
        ))
    }
}

#[inline]
async fn find_enabled_client(id: String) -> Result<Client, ErrorResponse> {
    let client = Client::find(id).await.map_err(|_| {
        ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("client-not-found".to_string()),
            "client does not exist anymore",
        )
    })?;
    if client.enabled {
        Ok(client)
    } else {
        Err(ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("client-disabled".to_string()),
            "client has been disabled",
        ))
    }
}
