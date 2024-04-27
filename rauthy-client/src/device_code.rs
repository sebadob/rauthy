use crate::rauthy_error::RauthyError;

#[derive(Debug)]
pub struct DeviceCode {
    pub issuer: String,
}

impl DeviceCode {
    pub async fn request() -> Result<Self, RauthyError> {
        todo!()
    }
}
