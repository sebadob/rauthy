use crate::entity::api_keys::{AccessGroup, AccessRights, ApiKey};
use crate::entity::sessions::{Session, SessionState};
use actix_web::{web, HttpRequest};
use rauthy_common::constants::{ADMIN_FORCE_MFA, RAUTHY_ADMIN_ROLE};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use tracing::trace;

/// Used internally to check authentication and access rights for endpoints and actions
#[derive(Debug, Clone, Default)]
pub struct Principal {
    pub session: Option<Session>,
    pub api_key: Option<ApiKey>,
    pub roles: Vec<String>,
}

impl Principal {
    #[inline(always)]
    pub fn from_req(
        principal: web::ReqData<Option<Principal>>,
    ) -> Result<Principal, ErrorResponse> {
        principal.into_inner().ok_or_else(|| {
            ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Only allowed as a logged in user",
            )
        })
    }

    #[inline(always)]
    pub fn get_session(&self) -> Result<&Session, ErrorResponse> {
        if let Some(s) = &self.session {
            Ok(s)
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "No valid session",
            ))
        }
    }

    #[inline(always)]
    pub fn get_session_csrf_token(&self) -> Result<&str, ErrorResponse> {
        Ok(self.get_session()?.csrf_token.as_str())
    }

    #[inline(always)]
    pub fn is_admin(&self) -> bool {
        self.roles.contains(&*RAUTHY_ADMIN_ROLE)
    }

    #[inline(always)]
    pub fn is_user(&self, id: &str) -> Result<(), ErrorResponse> {
        if self.user_id() != Ok(id) {
            Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "You are not allowed to modify this user",
            ))
        } else {
            Ok(())
        }
    }

    #[inline(always)]
    pub fn has_mfa_active(&self) -> bool {
        self.session.as_ref().map(|s| s.is_mfa).unwrap_or(false)
    }

    #[inline(always)]
    pub fn user_id(&self) -> Result<&str, ErrorResponse> {
        self.session
            .as_ref()
            .and_then(|s| s.user_id.as_deref())
            .ok_or_else(|| {
                ErrorResponse::new(ErrorResponseType::NotFound, "No session for principal")
            })
    }

    #[inline(always)]
    pub fn validate_api_key(
        &self,
        access_group: AccessGroup,
        access_rights: AccessRights,
    ) -> Result<(), ErrorResponse> {
        if let Some(api_key) = &self.api_key {
            match api_key.validate_access(&access_group, &access_rights) {
                Ok(_) => Ok(()),
                Err(_) => Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    format!(
                        "Bad permissions for given ApiKey. Needed: {:?} / {:?}",
                        access_group, access_rights,
                    ),
                )),
            }
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "No API Key found",
            ))
        }
    }

    /// Validates the Principal's session to only allow authorized Rauthy admin access.
    #[inline(always)]
    pub fn validate_admin_session(&self) -> Result<(), ErrorResponse> {
        let _session = self.validate_session_auth()?;
        if !self.is_admin() {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Rauthy admin access only",
            ));
        }

        if *ADMIN_FORCE_MFA && !self.has_mfa_active() {
            return Err(ErrorResponse::new(
                ErrorResponseType::MfaRequired,
                "Rauthy admin access only allowed with MFA active",
            ));
        }

        Ok(())
    }

    /// Validates an ApiKey OR a valid admin session.
    /// If both are given, the ApiKey will have the higher priority since it is more specific.
    /// Returns an error with an invalid ApiKey even when a valid session exists.
    #[inline(always)]
    pub fn validate_api_key_or_admin_session(
        &self,
        access_group: AccessGroup,
        access_rights: AccessRights,
    ) -> Result<(), ErrorResponse> {
        match self.validate_api_key(access_group, access_rights) {
            Ok(_) => Ok(()),

            Err(err) => {
                if err.error == ErrorResponseType::Forbidden {
                    // in this case, return the Error from the ApiKey with the hint
                    // for the needed permissions in any case -> better DX and debugging
                    // without real security issues
                    Err(err)
                } else {
                    self.validate_admin_session()
                }
            }
        }
    }

    /// Validates the principal, that it is either an admin or the user matches the
    /// given `user_id`
    #[inline(always)]
    pub fn validate_user_or_admin(&self, user_id: &str) -> Result<(), ErrorResponse> {
        let session = self.validate_session_auth()?;
        if !self.is_admin() && session.user_id.as_deref() != Some(user_id) {
            Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Access is forbidden with this user",
            ))
        } else {
            Ok(())
        }
    }

    /// Validates the given user_id against this Principal.
    pub fn validate_user_session(&self, user_id: &str) -> Result<(), ErrorResponse> {
        let session = self.validate_session_auth()?;
        if session.user_id.as_deref() == Some(user_id) {
            Ok(())
        } else {
            trace!("Validating the session failed - was not in auth state");
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Unauthorized session or invalid user ID",
            ))
        }
    }

    #[inline(always)]
    pub fn validate_session_auth(&self) -> Result<&Session, ErrorResponse> {
        if let Some(session) = &self.session {
            if session.state()? == SessionState::Auth {
                Ok(session)
            } else {
                trace!("Validating the session failed - was not in auth state");
                Err(ErrorResponse::new(
                    ErrorResponseType::Unauthorized,
                    "Unauthorized Session",
                ))
            }
        } else {
            trace!("Validating the session failed - no session found");
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "No valid session",
            ))
        }
    }

    #[inline(always)]
    pub fn validate_session_auth_or_init(&self) -> Result<(), ErrorResponse> {
        if let Some(session) = &self.session {
            let state = session.state()?;
            if state == SessionState::Auth || state == SessionState::Init {
                Ok(())
            } else {
                trace!("Validating the session failed - was not in init or auth state");
                Err(ErrorResponse::new(
                    ErrorResponseType::Unauthorized,
                    "Unauthorized Session",
                ))
            }
        } else {
            trace!("Validating the session failed - no session found");
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "No valid session",
            ))
        }
    }

    #[inline(always)]
    pub fn validate_session_init(&self) -> Result<(), ErrorResponse> {
        if self
            .session
            .as_ref()
            .map(|s| s.state == SessionState::Init.as_str())
            .unwrap_or(false)
        {
            Ok(())
        } else {
            trace!("Validating the session failed - was not in init state");
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Session in Init state mandatory",
            ))
        }
    }

    #[inline(always)]
    pub fn validate_session_csrf_exception(&self, req: &HttpRequest) -> Result<(), ErrorResponse> {
        let s = self.get_session()?;
        s.validate_csrf(req)?;
        Ok(())
    }
}
