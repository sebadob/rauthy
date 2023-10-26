use crate::entity::api_keys::ApiKey;
use crate::entity::sessions::Session;
use actix_web::web;
use rauthy_common::constants::{ADMIN_FORCE_MFA, ROLE_ADMIN};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};

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
                String::from("Only allowed as a logged in user"),
            )
        })
    }

    #[inline(always)]
    pub fn is_user_authorized_for_id(&self, user_id: &str) -> Result<(), ErrorResponse> {
        let self_uid = self.session.as_ref().and_then(|s| s.user_id.as_deref());
        if !self.is_admin() && self_uid != Some(user_id) {
            Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Access is forbidden with this user".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    #[inline(always)]
    pub fn is_admin(&self) -> bool {
        for role in &self.roles {
            if role == &*ROLE_ADMIN {
                return true;
            }
        }
        false
    }

    // #[inline(always)]
    // pub async fn validate_email(
    //     &self,
    //     email: &str,
    //     data: &web::Data<AppState>,
    // ) -> Result<(), ErrorResponse> {
    //     // let email = self.session.and_then(|s| s.)
    //
    //     if self.email.is_some() {
    //         if self.email.as_ref().unwrap().as_str() != email {
    //             return Err(ErrorResponse::new(
    //                 ErrorResponseType::Forbidden,
    //                 String::from("Invalid email in path for JWT token user"),
    //             ));
    //         }
    //     } else if self.has_session {
    //         let user = User::find(data, self.user_id.as_ref().unwrap().clone()).await?;
    //         if user.email.as_str() != email {
    //             return Err(ErrorResponse::new(
    //                 ErrorResponseType::Forbidden,
    //                 String::from("Invalid email in path for session user"),
    //             ));
    //         }
    //     }
    //
    //     Ok(())
    // }

    #[inline(always)]
    pub fn user_id(&self) -> Result<&str, ErrorResponse> {
        self.session
            .as_ref()
            .and_then(|s| s.user_id.as_deref())
            .ok_or_else(|| {
                ErrorResponse::new(
                    ErrorResponseType::NotFound,
                    "No session for principal".to_string(),
                )
            })
    }

    #[inline(always)]
    pub fn validate_id(&self, id: &str) -> Result<(), ErrorResponse> {
        if self.user_id() != Ok(id) {
            Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "You are not allowed to modify this user".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    #[inline(always)]
    pub fn validate_rauthy_admin(&self) -> Result<(), ErrorResponse> {
        if !self.is_admin() {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Rauthy admin access only".to_string(),
            ));
        }

        let has_mfa_active = self.session.as_ref().map(|s| s.is_mfa).unwrap_or(false);
        if *ADMIN_FORCE_MFA && !has_mfa_active {
            return Err(ErrorResponse::new(
                ErrorResponseType::MfaRequired,
                "Rauthy admin access only allowed with MFA active".to_string(),
            ));
        }

        // TODO CAUTION - config variable is currently ignored
        // if *ADMIN_ACCESS_SESSION_ONLY && !self.has_session {
        //     return Err(ErrorResponse::new(
        //         ErrorResponseType::Forbidden,
        //         "Rauthy admin access denied with JWT tokens".to_string(),
        //     ));
        // }

        Ok(())
    }
}
