use crate::constants::{APPLICATION_JSON, HEADER_HTML, HEADER_RETRY_NOT_BEFORE};
use crate::utils::build_csp_header;
use actix_multipart::MultipartError;
use actix_web::error::BlockingError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, HttpResponseBuilder, ResponseError};
use css_color::ParseColorError;
use derive_more::Display;
use redhac::CacheError;
use serde::{Deserialize, Serialize};
use std::string::FromUtf8Error;
use time::OffsetDateTime;
use tracing::{debug, error};
use utoipa::ToSchema;

#[derive(Debug, Clone, Display, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum ErrorResponseType {
    BadRequest,
    CSRFTokenError,
    Database,
    DatabaseIo,
    Disabled,
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
    pub message: String,
}

impl ErrorResponse {
    pub fn new(error: ErrorResponseType, message: String) -> Self {
        Self {
            timestamp: OffsetDateTime::now_utc().unix_timestamp(),
            error,
            message,
        }
    }

    pub fn error_response_html(&self, body: String, nonce: &str) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .append_header(HEADER_HTML)
            .append_header(build_csp_header(nonce))
            .body(body)
    }
}

impl ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
        match self.error {
            ErrorResponseType::BadRequest => StatusCode::BAD_REQUEST,
            ErrorResponseType::Forbidden => StatusCode::FORBIDDEN,
            ErrorResponseType::MfaRequired => StatusCode::NOT_ACCEPTABLE,
            ErrorResponseType::NotFound => StatusCode::NOT_FOUND,
            ErrorResponseType::Disabled
            | ErrorResponseType::CSRFTokenError
            | ErrorResponseType::PasswordExpired
            | ErrorResponseType::SessionExpired
            | ErrorResponseType::SessionTimeout
            | ErrorResponseType::Unauthorized => StatusCode::UNAUTHORIZED,
            ErrorResponseType::TooManyRequests(_not_before_timestamp) => {
                StatusCode::TOO_MANY_REQUESTS
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self.error {
            ErrorResponseType::TooManyRequests(not_before_timestamp) => {
                HttpResponseBuilder::new(self.status_code())
                    .insert_header((HEADER_RETRY_NOT_BEFORE, not_before_timestamp))
                    .insert_header(HEADER_HTML)
                    .body(serde_json::to_string(&self.message).unwrap())
            }
            _ => HttpResponseBuilder::new(self.status_code())
                .content_type(APPLICATION_JSON)
                .body(serde_json::to_string(&self).unwrap()),
        }
    }
}

impl From<argon2::Error> for ErrorResponse {
    fn from(err: argon2::Error) -> Self {
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
    fn from(_: chrono::ParseError) -> Self {
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            String::from("Unable to parse the correct DateTime"),
        )
    }
}

impl From<BlockingError> for ErrorResponse {
    fn from(_e: BlockingError) -> Self {
        ErrorResponse::new(
            ErrorResponseType::Internal,
            String::from("Database Pool is gone, please re-try later"),
        )
    }
}

impl From<CacheError> for ErrorResponse {
    fn from(e: CacheError) -> Self {
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
            sqlx::Error::Database(e) => (ErrorResponseType::Database, e.to_string()),
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
    fn from(_: ParseColorError) -> Self {
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
        ErrorResponse::new(ErrorResponseType::Internal, value.to_string())
    }
}

impl From<validator::ValidationErrors> for ErrorResponse {
    fn from(value: validator::ValidationErrors) -> Self {
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("Payload validation error: {:?}", value),
        )
    }
}
