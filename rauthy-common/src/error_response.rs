use crate::constants::{APPLICATION_JSON, HEADER_DPOP_NONCE, HEADER_HTML, HEADER_RETRY_NOT_BEFORE};
use actix_multipart::MultipartError;
use actix_web::error::BlockingError;
use actix_web::http::header::{
    InvalidHeaderValue, ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_EXPOSE_HEADERS,
    WWW_AUTHENTICATE,
};
use actix_web::http::{header, StatusCode};
use actix_web::{HttpResponse, HttpResponseBuilder, ResponseError};
use cryptr::CryptrError;
use css_color::ParseColorError;
use derive_more::Display;
use image::ImageError;
use redhac::CacheError;
use rio_turtle::TurtleError;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use serde_json_path::ParseError;
use spow::pow::PowError;
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;
use time::OffsetDateTime;
use tracing::{debug, error, trace};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum ErrorResponseType {
    BadRequest,
    Connection,
    CSRFTokenError,
    Database,
    DatabaseIo,
    Disabled,
    // These String could be optimized in the future with borrowing
    // -> just not going down that rabbit hole for now
    DPoP(Option<String>),
    Encryption,
    UseDpopNonce((Option<String>, String)),
    Forbidden,
    Internal,
    JoseError,
    MfaRequired,
    NoSession,
    NotFound,
    PasswordExpired,
    PasswordRefresh,
    SessionExpired,
    SessionTimeout,
    TooManyRequests(i64),
    Unauthorized,
    WWWAuthenticate(String),
}

impl Display for ErrorResponseType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// This is the default `ErrorResponse` that could be the answer on almost every API endpoint in
// case something is wrong.<br>
// Except for input validations, every error will have this format and every possible error in the
// backend will be converted to this.
#[derive(Debug, Clone, Display, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[display(fmt = "error: {} message: {}", error, message)]
pub struct ErrorResponse {
    pub timestamp: i64,
    pub error: ErrorResponseType,
    pub message: Cow<'static, str>,
}

impl ErrorResponse {
    pub fn new<C>(error: ErrorResponseType, message: C) -> Self
    where
        C: Into<Cow<'static, str>>,
    {
        Self {
            timestamp: OffsetDateTime::now_utc().unix_timestamp(),
            error,
            message: message.into(),
        }
    }

    pub fn error_response_html(&self, body: String) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .append_header(HEADER_HTML)
            .body(body)
    }
}

impl ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
        match self.error {
            ErrorResponseType::BadRequest | ErrorResponseType::UseDpopNonce(_) => {
                StatusCode::BAD_REQUEST
            }
            ErrorResponseType::Forbidden => StatusCode::FORBIDDEN,
            ErrorResponseType::MfaRequired => StatusCode::NOT_ACCEPTABLE,
            ErrorResponseType::NotFound => StatusCode::NOT_FOUND,
            ErrorResponseType::Disabled
            | ErrorResponseType::CSRFTokenError
            | ErrorResponseType::DPoP(_)
            | ErrorResponseType::PasswordExpired
            | ErrorResponseType::SessionExpired
            | ErrorResponseType::SessionTimeout
            | ErrorResponseType::Unauthorized
            | ErrorResponseType::WWWAuthenticate(_) => StatusCode::UNAUTHORIZED,
            ErrorResponseType::TooManyRequests(_not_before_timestamp) => {
                StatusCode::TOO_MANY_REQUESTS
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        match &self.error {
            ErrorResponseType::TooManyRequests(not_before_timestamp) => {
                HttpResponseBuilder::new(status)
                    .insert_header((HEADER_RETRY_NOT_BEFORE, *not_before_timestamp))
                    .insert_header(HEADER_HTML)
                    // TODO we could possibly do a small `unsafe` call here to just take
                    // the content without cloning it -> more efficient, especially for blocked IPs
                    .body(self.message.clone())
            }

            ErrorResponseType::DPoP(header_origin) => {
                if let Some(origin) = header_origin {
                    HttpResponseBuilder::new(status)
                        .insert_header((WWW_AUTHENTICATE, "DPoP error=invalid_dpop_proof"))
                        .content_type(APPLICATION_JSON)
                        .insert_header((ACCESS_CONTROL_ALLOW_ORIGIN, origin.as_str()))
                        .body(serde_json::to_string(&self).unwrap())
                } else {
                    HttpResponseBuilder::new(status)
                        .insert_header((WWW_AUTHENTICATE, "DPoP error=invalid_dpop_proof"))
                        .content_type(APPLICATION_JSON)
                        .body(serde_json::to_string(&self).unwrap())
                }
            }

            ErrorResponseType::UseDpopNonce((header_origin, value)) => {
                if let Some(origin) = header_origin {
                    HttpResponseBuilder::new(status)
                        .insert_header((WWW_AUTHENTICATE, "DPoP error=use_dpop_nonce"))
                        .insert_header((HEADER_DPOP_NONCE, value.as_str()))
                        .content_type(APPLICATION_JSON)
                        .insert_header((ACCESS_CONTROL_ALLOW_ORIGIN, origin.as_str()))
                        .insert_header((ACCESS_CONTROL_EXPOSE_HEADERS, HEADER_DPOP_NONCE))
                        .content_type(APPLICATION_JSON)
                        .body(serde_json::to_string(&self).unwrap())
                } else {
                    HttpResponseBuilder::new(status)
                        .insert_header((WWW_AUTHENTICATE, "DPoP error=use_dpop_nonce"))
                        .insert_header((HEADER_DPOP_NONCE, value.as_str()))
                        .content_type(APPLICATION_JSON)
                        .body(serde_json::to_string(&self).unwrap())
                }
            }

            ErrorResponseType::WWWAuthenticate(msg) => HttpResponseBuilder::new(status)
                .insert_header((WWW_AUTHENTICATE, msg.as_str()))
                .content_type(APPLICATION_JSON)
                .body(serde_json::to_string(self).unwrap()),

            _ => {
                if status == StatusCode::UNAUTHORIZED {
                    HttpResponseBuilder::new(status)
                        .append_header((WWW_AUTHENTICATE, "OAuth"))
                        .content_type(APPLICATION_JSON)
                        .body(serde_json::to_string(self).unwrap())
                } else {
                    HttpResponseBuilder::new(status)
                        .content_type(APPLICATION_JSON)
                        .body(serde_json::to_string(self).unwrap())
                }
            }
        }
    }
}

impl From<std::io::Error> for ErrorResponse {
    fn from(value: std::io::Error) -> Self {
        trace!("{:?}", value);
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("IO Error: {}", value),
        )
    }
}

impl From<argon2::Error> for ErrorResponse {
    fn from(err: argon2::Error) -> Self {
        trace!("{:?}", err);
        ErrorResponse::new(
            ErrorResponseType::Internal,
            format!("Argon2Id Error: {}", err),
        )
    }
}

impl From<Box<bincode::ErrorKind>> for ErrorResponse {
    fn from(err: Box<bincode::ErrorKind>) -> Self {
        error!("Bincode Error: {:?}", err);
        ErrorResponse::new(
            ErrorResponseType::Internal,
            "Serialization Error".to_string(),
        )
    }
}

impl From<chrono::ParseError> for ErrorResponse {
    fn from(value: chrono::ParseError) -> Self {
        trace!("{:?}", value);
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            String::from("Unable to parse the correct DateTime"),
        )
    }
}

impl From<BlockingError> for ErrorResponse {
    fn from(value: BlockingError) -> Self {
        trace!("{:?}", value);
        ErrorResponse::new(
            ErrorResponseType::Internal,
            String::from("Database Pool is gone, please re-try later"),
        )
    }
}

impl From<CacheError> for ErrorResponse {
    fn from(e: CacheError) -> Self {
        trace!("{:?}", e);
        ErrorResponse::new(
            ErrorResponseType::Internal,
            format!("Internal Cache error: {:?}", e),
        )
    }
}

impl From<chacha20poly1305::Error> for ErrorResponse {
    fn from(e: chacha20poly1305::Error) -> Self {
        error!("{}", e);
        ErrorResponse::new(
            ErrorResponseType::Internal,
            "Internal Encryption Error".to_string(),
        )
    }
}

impl From<sqlx::Error> for ErrorResponse {
    fn from(err: sqlx::Error) -> Self {
        debug!("From<sqlx::Error>: {:?}", err);
        let (error, msg) = match err {
            sqlx::Error::Configuration(e) => (ErrorResponseType::Database, e.to_string()),
            sqlx::Error::Database(e) => {
                let s = e.to_string();
                if s.contains("duplicate key") || s.contains("UNIQUE") {
                    // basically returns http 400 on duplicate id column errors -> no distinct err type
                    (ErrorResponseType::BadRequest, s)
                } else {
                    (ErrorResponseType::Database, s)
                }
            }
            sqlx::Error::Io(e) => (ErrorResponseType::DatabaseIo, e.to_string()),
            sqlx::Error::RowNotFound => (ErrorResponseType::NotFound, "not found".to_string()),
            sqlx::Error::TypeNotFound { type_name } => (
                ErrorResponseType::NotFound,
                format!("Type not found: {}", type_name),
            ),
            sqlx::Error::ColumnNotFound(s) => (
                ErrorResponseType::NotFound,
                format!("Column not found: {}", s),
            ),
            sqlx::Error::PoolTimedOut => (
                ErrorResponseType::Internal,
                "Network error, please try again".to_string(),
            ),
            sqlx::Error::PoolClosed => (
                ErrorResponseType::Internal,
                "Network error, please try again".to_string(),
            ),
            e => {
                error!("Database Error: {:?}", e);
                (
                    ErrorResponseType::Internal,
                    "Internal error, please try again".to_string(),
                )
            }
        };

        ErrorResponse::new(error, msg)
    }
}

impl From<ParseColorError> for ErrorResponse {
    fn from(value: ParseColorError) -> Self {
        trace!("{:?}", value);
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Cannot parse input to valid CSS color".to_string(),
        )
    }
}

impl From<actix_multipart::MultipartError> for ErrorResponse {
    fn from(value: actix_multipart::MultipartError) -> Self {
        error!("From<actix_multipart::MultipartError>: {:?}", value);

        let text = match value {
            MultipartError::MissingField(_) => "MultipartError::MissingField",
            MultipartError::NoContentDisposition => "MultipartError::NoContentDisposition",
            MultipartError::NoContentType => "MultipartError::NoContentType",
            MultipartError::ParseContentType => "MultipartError::ParseContentType",
            MultipartError::Boundary => "MultipartError::Boundary",
            MultipartError::Nested => "MultipartError::Nested",
            MultipartError::Incomplete => "MultipartError::Incomplete",
            MultipartError::Parse(_) => "MultipartError::Parse",
            MultipartError::Payload(_) => "MultipartError::Payload",
            MultipartError::NotConsumed => "MultipartError::NotConsumed",
            MultipartError::Field { .. } => "MultipartError::Field",
            MultipartError::DuplicateField(_) => "MultipartError::DuplicateField",
            MultipartError::UnsupportedField(_) => "MultipartError::UnsupportedField",
            _ => "MultipartError::Unknown",
        };

        ErrorResponse::new(ErrorResponseType::BadRequest, text.to_string())
    }
}

impl From<FromUtf8Error> for ErrorResponse {
    fn from(value: FromUtf8Error) -> Self {
        trace!("{:?}", value);
        ErrorResponse::new(ErrorResponseType::Internal, value.to_string())
    }
}

impl From<validator::ValidationErrors> for ErrorResponse {
    fn from(value: validator::ValidationErrors) -> Self {
        trace!("{:?}", value);
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("Payload validation error: {:?}", value),
        )
    }
}

impl From<serde_json::Error> for ErrorResponse {
    fn from(value: Error) -> Self {
        trace!("{:?}", value);
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("Payload deserialization error: {:?}", value),
        )
    }
}
impl From<reqwest::header::ToStrError> for ErrorResponse {
    fn from(value: reqwest::header::ToStrError) -> Self {
        trace!("{:?}", value);
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!(
                "Request headers contained non ASCII characters: {:?}",
                value
            ),
        )
    }
}

impl From<reqwest::Error> for ErrorResponse {
    fn from(value: reqwest::Error) -> Self {
        trace!("{:?}", value);
        ErrorResponse::new(
            ErrorResponseType::Connection,
            format!("Cannot send out HTTP request: {:?}", value),
        )
    }
}

impl From<TurtleError> for ErrorResponse {
    fn from(value: TurtleError) -> Self {
        trace!("{:?}", value);
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("Cannot format / parse turtle data: {:?}", value),
        )
    }
}

impl From<oxiri::IriParseError> for ErrorResponse {
    fn from(value: oxiri::IriParseError) -> Self {
        trace!("{:?}", value);
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("Invalid iri given: {:?}", value),
        )
    }
}

impl From<CryptrError> for ErrorResponse {
    fn from(value: CryptrError) -> Self {
        trace!("{:?}", value);
        ErrorResponse::new(ErrorResponseType::Encryption, value.to_string())
    }
}

impl From<PowError> for ErrorResponse {
    fn from(value: PowError) -> Self {
        trace!("{:?}", value);
        ErrorResponse::new(ErrorResponseType::Forbidden, value.to_string())
    }
}

impl From<serde_json_path::ParseError> for ErrorResponse {
    fn from(value: ParseError) -> Self {
        trace!("{:?}", value);
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("JsonPath error: {}", value),
        )
    }
}

impl From<ImageError> for ErrorResponse {
    fn from(value: ImageError) -> Self {
        trace!("{:?}", value);
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("Cannot parse the image data: {}", value),
        )
    }
}

impl From<actix_web::http::header::ToStrError> for ErrorResponse {
    fn from(value: actix_web::http::header::ToStrError) -> Self {
        trace!("{:?}", value);
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!(
                "Request headers contained non ASCII characters: {:?}",
                value
            ),
        )
    }
}

impl From<header::InvalidHeaderValue> for ErrorResponse {
    fn from(value: InvalidHeaderValue) -> Self {
        trace!("{:?}", value);
        ErrorResponse::new(
            ErrorResponseType::Internal,
            format!("Cannot convert to HeaderValue: {:?}", value),
        )
    }
}
