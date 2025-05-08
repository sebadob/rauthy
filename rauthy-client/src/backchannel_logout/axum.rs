use crate::backchannel_logout::logout_token::LogoutToken;
use crate::rauthy_error::RauthyError;
use axum::body::Bytes;
use axum::extract::{FromRequest, Request};
use tracing::error;

impl<S> FromRequest<S> for LogoutToken
where
    S: Send + Sync,
{
    type Rejection = RauthyError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Bytes::from_request(req, state).await {
            Ok(body) => {
                let s = String::from_utf8_lossy(body.as_ref());
                LogoutToken::from_str_validated(s.as_ref()).await
            }
            Err(err) => {
                error!(
                    "Error extracting `logout_token` from request body: {:?}",
                    err
                );
                Err(RauthyError::BadRequest(
                    "Cannot extract `logout_token` from request body",
                ))
            }
        }
    }
}
