// Copyright 2026 Sebastian Dobe <sebastiandobe@mailbox.org>

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use utoipa::ToSchema;

pub mod error_impls;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum ErrorResponseType {
    BadRequest,
    Blocked,
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
    /// RFC 6749 §5.2: the provided grant is invalid, expired, revoked, or was issued to
    /// another client. Serialized as the RFC error code `invalid_grant`. Used by the RFC 8693
    /// token exchange for a `subject_token` / `actor_token` that cannot be accepted.
    #[serde(rename = "invalid_grant")]
    InvalidGrant,
    /// RFC 7591 §3.2.2: the value of one or more client metadata fields is invalid or not
    /// allowed. Serialized as the RFC error code `invalid_client_metadata`. Used by the
    /// RFC 7592 self-update of a dynamic client that requests a grant type it does not have.
    #[serde(rename = "invalid_client_metadata")]
    InvalidClientMetadata,
    /// RFC 8707 §2: the requested `resource` is invalid, unknown, malformed, or not
    /// allowed for the client. Serialized as the RFC error code `invalid_target`.
    #[serde(rename = "invalid_target")]
    InvalidTarget,
    JwtToken,
    JoseError,
    MfaRequired,
    /// The request carried no session at all, so nothing inside it was ever validated. Its 401
    /// answers with the `x-session-expired` header, which tells a login page that its own session is
    /// gone and that it has to restart its flow, instead of reporting anything about credentials.
    NoSession,
    NotFound,
    PasswordExpired,
    PasswordRefresh,
    PreconditionRequired,
    Scim,
    SessionExpired,
    SessionTimeout,
    Timeout,
    TooManyRequests(i64),
    Unauthorized,
    NotAccepted,
    WWWAuthenticate(String),
    // Appended at the end to keep variant indices stable, in case an error is ever cached with
    // bincode, which encodes the variant index.
    /// OIDC Core §3.1.2.6: the authorization requires user consent that cannot be obtained.
    /// Serialized as the OIDC error code `consent_required`.
    #[serde(rename = "consent_required")]
    ConsentRequired,
    /// RFC 6749 §5.2 / RFC 8693 §2.2.2: the request is invalid, or the `subject_token` or
    /// `actor_token` is unacceptable based on policy. Serialized as the RFC error code
    /// `invalid_request`.
    #[serde(rename = "invalid_request")]
    InvalidRequest,
    /// The interactive sign-in form must ask for a password to complete the request.
    PasswordRequired,
    /// The credentials submitted through the interactive sign-in form were rejected.
    InvalidCredentials,
    /// RFC 6749 §5.2: the requested scope is invalid, unknown, malformed, or exceeds the scope
    /// granted by the resource owner. Serialized as the RFC error code `invalid_scope`.
    #[serde(rename = "invalid_scope")]
    InvalidScope,
}

impl Display for ErrorResponseType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

// This is the default `ErrorResponse` that could be the answer on almost every API endpoint in
// case something is wrong.<br>
// Except for input validations, every error will have this format and every possible error in the
// backend will be converted to this.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct ErrorResponse {
    pub timestamp: i64,
    pub error: ErrorResponseType,
    pub message: Cow<'static, str>,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "error: {}, timestamp: {}, message: {}",
            self.error, self.timestamp, self.message
        )
    }
}
