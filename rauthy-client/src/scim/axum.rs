use crate::scim::types::{
    ScimError, ScimGroup, ScimListQuery, ScimListResponse, ScimPatchOp, ScimUser,
    SCIM_SCHEMA_GROUP, SCIM_SCHEMA_PATCH_OP, SCIM_SCHEMA_USER,
};
use crate::scim::{is_content_type_scim, validate_scim_token, CONTENT_TYPE_SCIM};
use axum::extract::{FromRequest, FromRequestParts, Query, Request};
use axum::response::IntoResponse;
use axum::response::Response;
use http::header::CONTENT_TYPE;
use http::request::Parts;
use tracing::error;

impl IntoResponse for ScimError {
    #[inline]
    fn into_response(self) -> Response {
        Response::builder()
            .status(self.status)
            .header(CONTENT_TYPE, CONTENT_TYPE_SCIM)
            .body(serde_json::to_string(&self).unwrap())
            .unwrap()
            .into_response()
    }
}

impl IntoResponse for ScimListResponse {
    #[inline]
    fn into_response(self) -> Response {
        Response::builder()
            .status(200)
            .header(CONTENT_TYPE, CONTENT_TYPE_SCIM)
            .body(serde_json::to_string(&self).unwrap())
            .unwrap()
            .into_response()
    }
}

impl IntoResponse for ScimGroup {
    #[inline]
    fn into_response(self) -> Response {
        Response::builder()
            .status(200)
            .header(CONTENT_TYPE, CONTENT_TYPE_SCIM)
            .body(serde_json::to_string(&self).unwrap())
            .unwrap()
            .into_response()
    }
}

impl IntoResponse for ScimUser {
    #[inline]
    fn into_response(self) -> Response {
        Response::builder()
            .status(200)
            .header(CONTENT_TYPE, CONTENT_TYPE_SCIM)
            .body(serde_json::to_string(&self).unwrap())
            .unwrap()
            .into_response()
    }
}

impl<S> FromRequest<S> for ScimUser
where
    S: Send + Sync,
{
    type Rejection = ScimError;

    #[inline]
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        validate_scim_token(req.headers())?;
        is_content_type_scim(req.headers())?;

        match axum::extract::Json::<ScimUser>::from_request(req, state).await {
            Ok(user) => {
                if user.schemas.iter().any(|s| s == SCIM_SCHEMA_USER) {
                    Ok(user.0)
                } else {
                    Err(ScimError::new(
                        400,
                        Some("Invalid Schema for ScimUser".into()),
                    ))
                }
            }
            Err(err) => {
                error!("Cannot extract ScimUser from request body: {:?}", err);
                Err(ScimError::new(
                    400,
                    Some(format!("Cannot extract ScimUser from request body: {:?}", err).into()),
                ))
            }
        }
    }
}

impl<S> FromRequest<S> for ScimGroup
where
    S: Send + Sync,
{
    type Rejection = ScimError;

    #[inline]
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        validate_scim_token(req.headers())?;
        is_content_type_scim(req.headers())?;

        match axum::extract::Json::<ScimGroup>::from_request(req, state).await {
            Ok(group) => {
                if group.schemas.iter().any(|s| s == SCIM_SCHEMA_GROUP) {
                    Ok(group.0)
                } else {
                    Err(ScimError::new(
                        400,
                        Some("Invalid Schema for ScimGroup".into()),
                    ))
                }
            }
            Err(err) => {
                error!("Cannot extract ScimGroup from request body: {:?}", err);
                Err(ScimError::new(
                    400,
                    Some(format!("Cannot extract ScimGroup from request body: {:?}", err).into()),
                ))
            }
        }
    }
}

impl<S> FromRequest<S> for ScimPatchOp
where
    S: Send + Sync,
{
    type Rejection = ScimError;

    #[inline]
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        println!("in from request for ScimPatchOp");

        validate_scim_token(req.headers())?;
        is_content_type_scim(req.headers())?;

        match axum::extract::Json::<ScimPatchOp>::from_request(req, state).await {
            Ok(op) => {
                if op.schemas.iter().any(|s| s == SCIM_SCHEMA_PATCH_OP) {
                    Ok(op.0)
                } else {
                    Err(ScimError::new(
                        400,
                        Some("Invalid Schema for ScimPatchOp".into()),
                    ))
                }
            }
            Err(err) => {
                error!("Cannot extract ScimPatchOp from request body: {:?}", err);
                Err(ScimError::new(
                    400,
                    Some(format!("Cannot extract ScimPatchOp from request body: {:?}", err).into()),
                ))
            }
        }
    }
}

impl<S> FromRequestParts<S> for ScimListQuery
where
    S: Send + Sync,
{
    type Rejection = ScimError;

    #[inline]
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        validate_scim_token(&parts.headers)?;

        let params = match Query::<ScimListQuery>::from_request_parts(parts, state).await {
            Ok(query) => query.0,
            Err(_) => ScimListQuery::default(),
        };
        Ok(params)
    }
}

// impl<S> FromRequestParts<S> for ScimSearchRequest
// where
//     S: Send + Sync,
// {
//     type Rejection = ScimError;
//
//     #[inline]
//     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         validate_scim_token(&parts.headers)?;
//
//         let params = match Query::<ScimSearchRequest>::from_request_parts(parts, state).await {
//             Ok(params) => params.0,
//             Err(_) => ScimSearchRequest::default(),
//         };
//         Ok(params)
//     }
// }
