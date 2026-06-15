use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde_json_path::{JsonPath, ParseError};
use std::fmt::Write;

#[inline]
pub fn has_match(
    input: &serde_json::Value,
    buf: &mut String,
    path: &str,
    value_expected: &str,
) -> Result<bool, ErrorResponse> {
    let path = parse_path(path)?;

    for value in path.query(input).all() {
        // We actually need this allocation to String for better compatibility.
        // This way, we can accept not only string, but we would, for instance,
        // also interpret a given bool as string.
        buf.clear();
        if !value.is_string() {
            write!(buf, "\"{value}\"")
        } else {
            write!(buf, "{value}")
        }
        .expect("Write to never fail");

        if buf == value_expected {
            return Ok(true);
        }
    }

    Ok(false)
}

#[inline]
pub fn parse_path(path: &str) -> Result<JsonPath, ErrorResponse> {
    JsonPath::parse(path).map_err(|err| {
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("Cannot parse JSON path: {err}"),
        )
    })
}
