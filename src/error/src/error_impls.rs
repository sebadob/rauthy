use crate::{ErrorResponse, ErrorResponseType};
use actix_multipart::MultipartError;
use actix_web::error::BlockingError;
use actix_web::http::header::{
    ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_EXPOSE_HEADERS, InvalidHeaderValue,
    WWW_AUTHENTICATE,
};
use actix_web::http::{StatusCode, header};
use actix_web::{HttpResponse, HttpResponseBuilder, ResponseError, error};
use cryptr::CryptrError;
use cryptr::utils::secure_random_alnum;
use css_color::ParseColorError;
use deadpool::managed::{BuildError, PoolError};
use image::ImageError;
use openssl::error::ErrorStack;
use ring::error::{KeyRejected, Unspecified};
use rio_turtle::TurtleError;
use rsa::pkcs1::Error;
use s3_simple::S3Error;
use serde_json_path::ParseError;
use spow::pow::PowError;
use std::borrow::Cow;
use std::string::FromUtf8Error;
use svg_hush::FError;
use time::OffsetDateTime;
use tracing::{debug, error};
use validator::ValidationError;

const APPLICATION_JSON: &str = "application/json";
const HEADER_DPOP_NONCE: &str = "DPoP-Nonce";
const HEADER_HTML: (&str, &str) = ("content-type", "text/html;charset=utf-8");
const HEADER_RETRY_NOT_BEFORE: &str = "x-retry-not-before";

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
            ErrorResponseType::Blocked
            | ErrorResponseType::Forbidden
            | ErrorResponseType::PasswordRefresh => StatusCode::FORBIDDEN,
            ErrorResponseType::MfaRequired | ErrorResponseType::NotAccepted => {
                StatusCode::NOT_ACCEPTABLE
            }
            ErrorResponseType::NotFound => StatusCode::NOT_FOUND,
            ErrorResponseType::Disabled
            | ErrorResponseType::CSRFTokenError
            | ErrorResponseType::DPoP(_)
            | ErrorResponseType::JwtToken
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
            ErrorResponseType::Blocked => HttpResponseBuilder::new(status)
                .insert_header(HEADER_HTML)
                .body(self.message.clone()),

            ErrorResponseType::TooManyRequests(not_before_timestamp) => {
                HttpResponseBuilder::new(status)
                    .insert_header((HEADER_RETRY_NOT_BEFORE, *not_before_timestamp))
                    .insert_header(HEADER_HTML)
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
        debug!("{:?}", value);
        ErrorResponse::new(ErrorResponseType::BadRequest, format!("IO Error: {value}"))
    }
}

impl From<actix_web::Error> for ErrorResponse {
    fn from(err: actix_web::Error) -> Self {
        debug!("actix_web::Error: {:?}", err);
        ErrorResponse::new(
            ErrorResponseType::Internal,
            format!("actix_web::Error: {err}"),
        )
    }
}

impl From<actix_web::error::HttpError> for ErrorResponse {
    fn from(err: actix_web::error::HttpError) -> Self {
        debug!("HttpError: {err:?}");
        ErrorResponse::new(
            ErrorResponseType::Internal,
            format!("actix_web::error::HttpError: {err}"),
        )
    }
}

impl From<argon2::Error> for ErrorResponse {
    fn from(err: argon2::Error) -> Self {
        debug!("argon2::Error: {err:?}");
        ErrorResponse::new(
            ErrorResponseType::Internal,
            format!("Argon2Id Error: {err}"),
        )
    }
}

impl From<chrono::ParseError> for ErrorResponse {
    fn from(value: chrono::ParseError) -> Self {
        debug!("chrono::ParseError: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Unable to parse the correct DateTime",
        )
    }
}

impl From<BlockingError> for ErrorResponse {
    fn from(value: BlockingError) -> Self {
        debug!("BlockingError: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::Internal,
            "Database Pool is gone, please re-try later",
        )
    }
}

impl From<chacha20poly1305::Error> for ErrorResponse {
    fn from(e: chacha20poly1305::Error) -> Self {
        error!("chacha20poly1305::Error: {e}");
        ErrorResponse::new(ErrorResponseType::Internal, "Internal Encryption Error")
    }
}

impl From<deadpool::managed::BuildError> for ErrorResponse {
    fn from(e: BuildError) -> Self {
        debug!("deadpool::managed::BuildError: {e}");
        ErrorResponse::new(ErrorResponseType::DatabaseIo, e.to_string())
    }
}

impl From<deadpool::managed::PoolError<tokio_postgres::Error>> for ErrorResponse {
    fn from(e: PoolError<tokio_postgres::Error>) -> Self {
        debug!("PoolError<tokio_postgres::Error>: {e}");
        ErrorResponse::new(ErrorResponseType::DatabaseIo, e.to_string())
    }
}

impl From<tokio_postgres::Error> for ErrorResponse {
    fn from(e: tokio_postgres::Error) -> Self {
        debug!("tokio_postgres::Error: {e}");

        if let Some(code) = e.code() {
            let code = code.code();
            let (typ, msg) = match code {
                // unique constraint violation
                "23505" => (
                    ErrorResponseType::BadRequest,
                    Cow::from("ID exists already"),
                ),
                _ => (ErrorResponseType::Database, Cow::from(e.to_string())),
            };

            error!("SqlState({code}) -> {msg}");
            ErrorResponse::new(typ, msg)
        } else {
            let msg = e.to_string();

            // If we don't get a code, we can try to match by message.
            // This is usually quite brittle, but if these messages ever change, the integration
            // tests will catch them.
            if msg == "query returned an unexpected number of rows" {
                return Self::new(ErrorResponseType::NotFound, "No results found");
            }

            let id = secure_random_alnum(6);
            error!("db-err-id({id}): {msg}");
            Self::new(
                ErrorResponseType::Database,
                format!("Database error without inner error, check logs for:  db-err-id({id})"),
            )
        }
    }
}

impl From<hiqlite::Error> for ErrorResponse {
    fn from(value: hiqlite::Error) -> Self {
        debug!("hiqlite::Error: {value:?}");

        let (error, msg) = match value {
            hiqlite::Error::BadRequest(err) => (ErrorResponseType::BadRequest, err),
            hiqlite::Error::CheckIsLeaderError(err) => {
                (ErrorResponseType::Connection, err.to_string().into())
            }
            hiqlite::Error::ClientWriteError(err) => {
                (ErrorResponseType::Connection, err.to_string().into())
            }
            hiqlite::Error::Connect(err) => (ErrorResponseType::Connection, err.to_string().into()),
            hiqlite::Error::ConstraintViolation(err) => {
                (ErrorResponseType::BadRequest, err.to_string().into())
            }
            hiqlite::Error::LeaderChange(err) => {
                (ErrorResponseType::Connection, err.to_string().into())
            }
            hiqlite::Error::QueryReturnedNoRows(err) => {
                (ErrorResponseType::NotFound, err.to_string().into())
            }
            hiqlite::Error::Request(err) => (ErrorResponseType::Connection, err.to_string().into()),
            hiqlite::Error::S3(err) => (ErrorResponseType::Connection, err.to_string().into()),
            hiqlite::Error::Sqlite(err) => (ErrorResponseType::Database, err.to_string().into()),
            hiqlite::Error::Timeout(err) => (ErrorResponseType::Connection, err.to_string().into()),
            hiqlite::Error::Transaction(err) => {
                (ErrorResponseType::Database, err.to_string().into())
            }
            hiqlite::Error::WebSocket(err) => {
                (ErrorResponseType::Connection, err.to_string().into())
            }
            err => (ErrorResponseType::Database, err.to_string().into()),
        };

        ErrorResponse::new(error, msg)
    }
}

impl From<ParseColorError> for ErrorResponse {
    fn from(value: ParseColorError) -> Self {
        debug!("ParseColorError: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Cannot parse input to valid CSS color",
        )
    }
}

impl From<actix_multipart::MultipartError> for ErrorResponse {
    fn from(value: actix_multipart::MultipartError) -> Self {
        debug!("From<actix_multipart::MultipartError>: {value:?}");

        let text = match value {
            MultipartError::MissingField(_) => "MultipartError::MissingField",
            MultipartError::ContentDispositionMissing => {
                "MultipartError::ContentDispositionMissing"
            }
            MultipartError::ContentTypeIncompatible => "MultipartError::ContentTypeIncompatible",
            MultipartError::ContentDispositionNameMissing => {
                "MultipartError::ContentDispositionNameMissing"
            }
            MultipartError::ContentTypeMissing => "MultipartError::ContentTypeMissing",
            MultipartError::ContentTypeParse => "MultipartError::ContentTypeParse",
            MultipartError::BoundaryMissing => "MultipartError::BoundaryMissing",
            MultipartError::Nested => "MultipartError::Nested",
            MultipartError::Incomplete => "MultipartError::Incomplete",
            MultipartError::Parse(_) => "MultipartError::Parse",
            MultipartError::Payload(_) => "MultipartError::Payload",
            MultipartError::NotConsumed => "MultipartError::NotConsumed",
            MultipartError::Field { .. } => "MultipartError::Field",
            MultipartError::DuplicateField(_) => "MultipartError::DuplicateField",
            MultipartError::UnknownField(_) => "MultipartError::UnknownField",
            _ => "MultipartError::Unknown",
        };

        ErrorResponse::new(ErrorResponseType::BadRequest, text)
    }
}

impl From<error::PayloadError> for ErrorResponse {
    fn from(value: error::PayloadError) -> Self {
        debug!("PayloadError: {value:?}");
        ErrorResponse::new(ErrorResponseType::BadRequest, value.to_string())
    }
}

impl From<FromUtf8Error> for ErrorResponse {
    fn from(value: FromUtf8Error) -> Self {
        debug!("FromUtf8Error: {value:?}");
        ErrorResponse::new(ErrorResponseType::Internal, value.to_string())
    }
}

impl From<validator::ValidationErrors> for ErrorResponse {
    fn from(value: validator::ValidationErrors) -> Self {
        debug!("ValidationErrors: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("Payload validation error: {value:?}"),
        )
    }
}

impl From<validator::ValidationError> for ErrorResponse {
    fn from(value: ValidationError) -> Self {
        debug!("ValidationError: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("Payload validation error: {value:?}"),
        )
    }
}

impl From<serde_json::Error> for ErrorResponse {
    fn from(value: serde_json::Error) -> Self {
        debug!("serde_json::Error: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("Payload deserialization error: {value:?}"),
        )
    }
}

impl From<reqwest::header::ToStrError> for ErrorResponse {
    fn from(value: reqwest::header::ToStrError) -> Self {
        debug!("reqwest::header::ToStrError: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("Request headers contained non ASCII characters: {value:?}",),
        )
    }
}

impl From<reqwest::Error> for ErrorResponse {
    fn from(value: reqwest::Error) -> Self {
        debug!("reqwest::Error: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::Connection,
            format!("Cannot send out HTTP request: {value:?}"),
        )
    }
}

impl From<TurtleError> for ErrorResponse {
    fn from(value: TurtleError) -> Self {
        debug!("TurtleError: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("Cannot format / parse turtle data: {value:?}"),
        )
    }
}

impl From<oxiri::IriParseError> for ErrorResponse {
    fn from(value: oxiri::IriParseError) -> Self {
        debug!("oxiri::IriParseError: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("Invalid iri given: {value:?}"),
        )
    }
}

impl From<CryptrError> for ErrorResponse {
    fn from(value: CryptrError) -> Self {
        debug!("CryptrError: {value:?}");
        ErrorResponse::new(ErrorResponseType::Encryption, value.to_string())
    }
}

impl From<PowError> for ErrorResponse {
    fn from(value: PowError) -> Self {
        debug!("PowError: {value:?}");
        ErrorResponse::new(ErrorResponseType::Forbidden, value.to_string())
    }
}

impl From<serde_json_path::ParseError> for ErrorResponse {
    fn from(value: ParseError) -> Self {
        debug!("serde_json_path::ParseError: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("JsonPath error: {value}"),
        )
    }
}

impl From<serde_urlencoded::de::Error> for ErrorResponse {
    fn from(value: serde_urlencoded::de::Error) -> Self {
        debug!("serde_urlencoded::de::Error: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("URL encoded error: {value}"),
        )
    }
}

impl From<ImageError> for ErrorResponse {
    fn from(value: ImageError) -> Self {
        debug!("ImageError: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("Cannot parse the image data: {value}"),
        )
    }
}

impl From<actix_web::http::header::ToStrError> for ErrorResponse {
    fn from(value: actix_web::http::header::ToStrError) -> Self {
        debug!("ToStrError: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("Request headers contained non ASCII characters: {value:?}"),
        )
    }
}

impl From<header::InvalidHeaderValue> for ErrorResponse {
    fn from(value: InvalidHeaderValue) -> Self {
        debug!("header::InvalidHeaderValue: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::Internal,
            format!("Cannot convert to HeaderValue: {value:?}"),
        )
    }
}

impl From<std::fmt::Error> for ErrorResponse {
    fn from(value: std::fmt::Error) -> Self {
        debug!("{value:?}");
        ErrorResponse::new(ErrorResponseType::Internal, format!("fmt error: {value:?}"))
    }
}

impl From<svg_hush::FError> for ErrorResponse {
    fn from(value: FError) -> Self {
        debug!("svg_hush::FError: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("svg sanitization error: {value:?}"),
        )
    }
}

impl From<s3_simple::S3Error> for ErrorResponse {
    fn from(value: S3Error) -> Self {
        debug!("s3_simple::S3Error: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::Connection,
            format!("S3 error: {value:?}"),
        )
    }
}

impl From<rusqlite::Error> for ErrorResponse {
    fn from(value: rusqlite::Error) -> Self {
        debug!("rusqlite::Error: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::Database,
            format!("rusqlite error: {value:?}"),
        )
    }
}

impl From<rsa::Error> for ErrorResponse {
    fn from(value: rsa::Error) -> Self {
        debug!("rsa::Error: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("rsa error: {value:?}"),
        )
    }
}

impl From<rsa::pkcs1::Error> for ErrorResponse {
    fn from(value: Error) -> Self {
        debug!("rsa::pkcs1::Error: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("rsa::pkcs1::Error: {value:?}"),
        )
    }
}

impl From<rsa::pkcs8::Error> for ErrorResponse {
    fn from(value: rsa::pkcs8::Error) -> Self {
        debug!("rsa::pkcs8::Error: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("rsa::pkcs8::Error: {value:?}"),
        )
    }
}

impl From<ed25519_compact::Error> for ErrorResponse {
    fn from(value: ed25519_compact::Error) -> Self {
        debug!("ed25519_compact::Error: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("ed25519 error: {value:?}"),
        )
    }
}

impl From<openssl::error::ErrorStack> for ErrorResponse {
    fn from(value: ErrorStack) -> Self {
        debug!("openssl::error::ErrorStack: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("openssl::error::ErrorStack: {value:?}"),
        )
    }
}

impl From<ring::error::KeyRejected> for ErrorResponse {
    fn from(value: KeyRejected) -> Self {
        debug!("ring::error::KeyRejected: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("ring::error::KeyRejected: {value:?}"),
        )
    }
}

impl From<ring::error::Unspecified> for ErrorResponse {
    fn from(value: Unspecified) -> Self {
        debug!("ring::error::Unspecified: {value:?}");
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("ring::error::Unspecified: {value:?}"),
        )
    }
}
