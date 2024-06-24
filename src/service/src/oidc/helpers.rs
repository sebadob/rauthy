use actix_web::http::header::HeaderMap;
use rauthy_common::constants::TOKEN_BEARER;
use rauthy_error::{ErrorResponse, ErrorResponseType};

#[inline(always)]
pub fn get_bearer_token_from_header(headers: &HeaderMap) -> Result<String, ErrorResponse> {
    let bearer = headers
        .get("Authorization")
        .ok_or_else(|| ErrorResponse::new(ErrorResponseType::Unauthorized, "Bearer Token missing"));
    if bearer.is_err() {
        return Err(ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            "Authorization header missing",
        ));
    }

    let head_val = bearer?
        .to_str()
        .map_err(|_| {
            ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Malformed Authorization Header. Could not extract token.",
            )
        })?
        .to_string();

    let (p, bearer) = head_val.split_once(' ').ok_or(("ERR", "")).map_err(|_| {
        ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            "Malformed Authorization Header. Could not extract token.",
        )
    })?;
    if p.ne(TOKEN_BEARER) || bearer.is_empty() {
        return Err(ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            "No bearer token given",
        ));
    }
    Ok(bearer.to_string())
}
