use crate::provider::SCIM_TOKEN;
use crate::scim::types::ScimError;
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use http::HeaderMap;
use tracing::error;

#[cfg(feature = "axum")]
pub mod axum;
pub mod types;

pub static CONTENT_TYPE_SCIM: &str = "application/scim+json";

#[allow(unused)]
#[inline]
pub fn is_content_type_scim(headers: &HeaderMap) -> Result<(), ScimError> {
    let Some(ct_val) = headers.get(CONTENT_TYPE) else {
        return Err(ScimError::new(400, Some("Content-Type missing".into())));
    };

    let ct = ct_val.to_str().unwrap_or_default();
    if CONTENT_TYPE_SCIM == ct {
        Ok(())
    } else {
        error!("SCIM request with invalid Content-Type: {}", ct);
        Err(ScimError::new(400, Some("Invalid Content-Type".into())))
    }
}

#[allow(unused)]
#[inline]
pub fn validate_scim_token(headers: &HeaderMap) -> Result<(), ScimError> {
    let Some(auth_val) = headers.get(AUTHORIZATION) else {
        error!("SCIM request no token");
        return Err(ScimError::new(
            400,
            Some("Authorization header missing".into()),
        ));
    };
    let Some(token) = auth_val
        .to_str()
        .unwrap_or_default()
        .strip_prefix("Bearer ")
    else {
        error!("SCIM request with invalid Authorization header");
        return Err(ScimError::new(
            400,
            Some("Invalid Authorization Header".into()),
        ));
    };

    if token == SCIM_TOKEN.get().expect("OIDC Config not set up") {
        Ok(())
    } else {
        error!("SCIM request with invalid token");
        Err(ScimError::new(400, Some("Invalid Bearer Token".into())))
    }
}
