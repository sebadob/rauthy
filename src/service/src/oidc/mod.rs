use crate::oidc::grant_types::authorization_code::grant_type_authorization_code;
use crate::oidc::grant_types::client_credentials::grant_type_credentials;
use crate::oidc::grant_types::password::grant_type_password;
use crate::oidc::grant_types::refresh_token::grant_type_refresh;
use crate::token_set::TokenSet;
use actix_web::HttpRequest;
use actix_web::http::header::{HeaderName, HeaderValue};
use rauthy_api_types::oidc::TokenRequest;
use rauthy_error::{ErrorResponse, ErrorResponseType};

pub use grant_types::device_code::grant_type_device_code;

pub mod auth_providers;
pub mod authorize;
pub mod bcl_logout_token;
mod grant_types;
pub mod helpers;
pub mod logout;
pub mod token_info;
pub mod token_revocation;
pub mod userinfo;
pub mod validation;

/// Main entrance function for returning a whole new [TokenSet](crate::models::response::TokenSet)
pub async fn get_token_set(
    req_data: TokenRequest,
    req: HttpRequest,
) -> Result<(TokenSet, Vec<(HeaderName, HeaderValue)>), ErrorResponse> {
    match req_data.grant_type.as_str() {
        "authorization_code" => grant_type_authorization_code(req, req_data).await,
        "client_credentials" => grant_type_credentials(req, req_data).await,
        "password" => grant_type_password(req, req_data).await,
        "refresh_token" => grant_type_refresh(req, req_data).await,
        _ => Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Invalid 'grant_type'",
        )),
    }
}
