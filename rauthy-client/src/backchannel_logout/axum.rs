use crate::backchannel_logout::logout_token::LogoutToken;
use crate::backchannel_logout::LogoutRequest;
use crate::rauthy_error::RauthyError;
use axum::extract::{FromRequest, Request};
use tracing::error;

impl<S> FromRequest<S> for LogoutToken
where
    S: Send + Sync,
{
    type Rejection = RauthyError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Form::<LogoutRequest>::from_request(req, state).await {
            Ok(req_data) => LogoutToken::from_str_validated(&req_data.logout_token).await,
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
