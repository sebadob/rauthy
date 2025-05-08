use jwt_simple::prelude::Deserialize;

pub mod logout_token;

#[cfg(feature = "axum")]
pub mod axum;

#[derive(Debug, Default, Deserialize)]
pub struct LogoutRequest {
    pub logout_token: String,
}
