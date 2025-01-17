use crate::oidc::validation;
use actix_web::web;
use rauthy_api_types::oidc::LogoutRequest;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::colors::ColorEntity;
use rauthy_models::entity::sessions::Session;
use rauthy_models::html_templates::LogoutHtml;
use rauthy_models::language::Language;
use rauthy_models::{JwtIdClaims, JwtTokenType};

/// Returns the Logout HTML Page for [GET /oidc/logout](crate::handlers::get_logout)
pub async fn get_logout_html(
    logout_request: LogoutRequest,
    session: &Session,
    data: &web::Data<AppState>,
    lang: &Language,
) -> Result<String, ErrorResponse> {
    let colors = ColorEntity::find_rauthy().await?;

    if logout_request.id_token_hint.is_none() {
        return Ok(LogoutHtml::build(&session.csrf_token, false, &colors, lang));
    }

    // check if the provided token hint is a valid
    let token_raw = logout_request.id_token_hint.unwrap();
    let claims = validation::validate_token::<JwtIdClaims>(data, &token_raw).await?;

    // check if it is an ID token
    if JwtTokenType::Id != claims.custom.typ {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "The provided token is not an ID token",
        ));
    }

    // from here on, the token_hint contains a valid ID token -> skip the logout confirmation
    if logout_request.post_logout_redirect_uri.is_some() {
        // unwrap is safe since the token is valid already
        let client_id = claims.custom.azp;
        let client = Client::find(client_id).await?;
        if client.post_logout_redirect_uris.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Given 'post_logout_redirect_uri' is not allowed",
            ));
        }

        let target = logout_request.post_logout_redirect_uri.unwrap();
        let uri_vec = client.get_post_logout_uris();
        let valid_redirect = uri_vec.as_ref().unwrap().iter().filter(|uri| {
            if uri.ends_with('*') && target.starts_with(uri.split_once('*').unwrap().0) {
                return true;
            }
            if target.eq(*uri) {
                return true;
            }
            false
        });
        if valid_redirect.count() == 0 {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Given 'post_logout_redirect_uri' is not allowed",
            ));
        }
        // redirect uri is valid at this point
    }

    Ok(LogoutHtml::build(&session.csrf_token, true, &colors, lang))
}
