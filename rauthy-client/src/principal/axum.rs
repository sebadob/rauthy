use crate::principal::PrincipalOidc;
use axum::extract::OptionalFromRequestParts;
use axum::response::IntoResponse;
use axum::{body::Body, extract::FromRequestParts, http::request::Parts, response::Response};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

impl PrincipalOidc {
    pub fn is_admin(&self) -> Result<(), Response> {
        if self.is_admin {
            Ok(())
        } else {
            Err(Response::builder()
                .status(403)
                .body(Body::from("Admin access only"))
                .unwrap()
                .into_response())
        }
    }

    pub fn is_user(&self) -> Result<(), Response> {
        if self.is_user {
            Ok(())
        } else {
            Err(Response::builder()
                .status(403)
                .body(Body::from("Access Forbidden"))
                .unwrap()
                .into_response())
        }
    }

    pub fn has_any_group(&self, group: Vec<&str>) -> Result<(), Response> {
        for g in &self.groups {
            if group.contains(&g.as_str()) {
                return Ok(());
            }
        }
        Err(Response::builder()
            .status(403)
            .body(Body::from("Groups do not match"))
            .unwrap()
            .into_response())
    }

    pub fn has_any_role(&self, roles: Vec<&str>) -> Result<(), Response> {
        for r in &self.roles {
            if roles.contains(&r.as_str()) {
                return Ok(());
            }
        }
        Err(Response::builder()
            .status(403)
            .body(Body::from("Roles do not match"))
            .unwrap()
            .into_response())
    }
}

impl<S> FromRequestParts<S> for PrincipalOidc
where
    S: Send + Sync,
{
    type Rejection = Response<Body>;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            <TypedHeader<Authorization<Bearer>> as FromRequestParts<S>>::from_request_parts(
                parts, state,
            )
            .await
            .map_err(|_| Response::builder().status(401).body(Body::empty()).unwrap())?;

        match PrincipalOidc::from_token_validated(bearer.token()).await {
            Ok(p) => {
                p.is_user()?;
                Ok(p)
            }
            Err(_err) => Err(Response::builder().status(401).body(Body::empty()).unwrap()),
        }
    }
}

impl<S> OptionalFromRequestParts<S> for PrincipalOidc
where
    S: Send + Sync,
{
    type Rejection = Response<Body>;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        // Extract the token from the authorization header
        if let Ok(TypedHeader(Authorization(bearer))) =
            <TypedHeader<Authorization<Bearer>> as FromRequestParts<S>>::from_request_parts(
                parts, state,
            )
            .await
        {
            if let Ok(p) = PrincipalOidc::from_token_validated(bearer.token()).await {
                if p.is_user().is_ok() {
                    return Ok(Some(p));
                }
            }
        }

        Ok(None)
    }
}
