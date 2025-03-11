use crate::oidc::grant_types::authorization_code::grant_type_authorization_code;
use crate::oidc::grant_types::client_credentials::grant_type_credentials;
use crate::oidc::grant_types::password::grant_type_password;
use crate::oidc::grant_types::refresh_token::grant_type_refresh;
use crate::token_set::TokenSet;
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{HttpRequest, web};
use rauthy_api_types::oidc::TokenRequest;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;

pub use grant_types::device_code::grant_type_device_code;

pub mod authorize;
mod grant_types;
pub mod helpers;
pub mod logout;
pub mod token_info;
pub mod userinfo;
pub mod validation;

/// Main entrance function for returning a whole new [TokenSet](crate::models::response::TokenSet)
pub async fn get_token_set(
    req_data: TokenRequest,
    data: &web::Data<AppState>,
    req: HttpRequest,
) -> Result<(TokenSet, Vec<(HeaderName, HeaderValue)>), ErrorResponse> {
    match req_data.grant_type.as_str() {
        "authorization_code" => grant_type_authorization_code(data, req, req_data).await,
        "client_credentials" => grant_type_credentials(data, req, req_data).await,
        "password" => grant_type_password(data, req, req_data).await,
        "refresh_token" => grant_type_refresh(data, req, req_data).await,
        _ => Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Invalid 'grant_type'",
        )),
    }
}
