use crate::principal::PrincipalOidc;
use actix_web::dev::Payload;
use actix_web::error::{ErrorForbidden, ErrorUnauthorized};
use actix_web::http::header::AUTHORIZATION;
use actix_web::{Error, HttpRequest};
use std::future::Future;
use std::pin::Pin;

impl PrincipalOidc {
    pub fn is_admin(&self) -> Result<(), Error> {
        if self.is_admin {
            Ok(())
        } else {
            Err(ErrorForbidden("Admin access only"))
        }
    }

    pub fn is_user(&self) -> Result<(), Error> {
        if self.is_user {
            Ok(())
        } else {
            Err(ErrorForbidden("Access Forbidden"))
        }
    }

    pub fn has_any_group(&self, group: Vec<&str>) -> Result<(), Error> {
        for g in &self.groups {
            if group.contains(&g.as_str()) {
                return Ok(());
            }
        }
        Err(ErrorForbidden("Groups do not match"))
    }

    pub fn has_any_role(&self, roles: Vec<&str>) -> Result<(), Error> {
        for r in &self.roles {
            if roles.contains(&r.as_str()) {
                return Ok(());
            }
        }
        Err(ErrorForbidden("Roles do not match"))
    }
}

/// Actix-Web trait implementation to extract the Principal from the
impl actix_web::FromRequest for PrincipalOidc {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let header_value = req.headers().get(AUTHORIZATION).cloned();

        Box::pin(async move {
            match header_value {
                None => Err(ErrorUnauthorized("Unauthorized")),
                Some(value) => {
                    let (_bearer, access_token) = value
                        .to_str()
                        .unwrap_or_default()
                        .split_once("Bearer ")
                        .unwrap_or_default();

                    if access_token.is_empty() {
                        Err(ErrorUnauthorized("Unauthorized"))
                    } else {
                        match PrincipalOidc::from_token_validated(access_token).await {
                            Ok(p) => {
                                p.is_user()?;
                                Ok(p)
                            }
                            Err(_err) => Err(ErrorUnauthorized("Unauthorized")),
                        }
                    }
                }
            }
        })
    }
}
