use crate::oidc::validation;
use actix_web::web;
use rauthy_api_types::oidc::TokenInfo;
use rauthy_error::ErrorResponse;
use rauthy_models::app_state::AppState;
use rauthy_models::JwtCommonClaims;

/// Returns [TokenInfo](crate::models::response::TokenInfo) for the
/// [/oidc/tokenInfo endpoint](crate::handlers::post_token_info)
pub async fn get_token_info(
    data: &web::Data<AppState>,
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
    // scope does not exist for ID tokens, for all others unwrap is safe
    let scope = claims.custom.scope;
    let client_id = claims.custom.azp;
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
