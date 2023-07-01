use crate::app_state::AppState;
use crate::entity::users::User;
use actix_web::web;
use rauthy_common::constants::{ADMIN_ACCESS_SESSION_ONLY, ADMIN_FORCE_MFA, ROLE_ADMIN};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};

// Only used internally to provide a principal for authenticated endpoints
#[derive(Debug, Clone)]
pub struct Principal {
    pub user_id: String,
    pub email: Option<String>,
    pub has_mfa_active: bool,
    pub has_session: bool,
    pub has_token: bool,
    pub roles: Vec<String>,
}

impl Principal {
    pub fn get_from_req(principal: Option<Principal>) -> Result<Principal, ErrorResponse> {
        principal.ok_or_else(|| {
            ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                String::from("Only allowed as a logged in user"),
            )
        })
    }

    pub fn is_user_authorized_for_id(&self, user_id: &str) -> Result<(), ErrorResponse> {
        if !self.is_admin() && self.user_id != user_id {
            Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Access is forbidden with this user".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    pub fn is_admin(&self) -> bool {
        self.roles.contains(&*ROLE_ADMIN)
    }

    pub async fn validate_email(
        &self,
        email: &str,
        data: &web::Data<AppState>,
    ) -> Result<(), ErrorResponse> {
        if self.email.is_some() {
            if self.email.as_ref().unwrap().as_str() != email {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    String::from("Invalid email in path for JWT token user"),
                ));
            }
        } else {
            let user = User::find(data, self.user_id.clone()).await?;
            if user.email.as_str() != email {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    String::from("Invalid email in path for session user"),
                ));
            }
        }

        Ok(())
    }

    pub fn validate_id(&self, id: &str) -> Result<(), ErrorResponse> {
        if self.user_id != id {
            Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "You are not allowed to modify this user".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    pub fn validate_rauthy_admin(&self) -> Result<(), ErrorResponse> {
        if *ADMIN_FORCE_MFA && !self.has_mfa_active {
            return Err(ErrorResponse::new(
                ErrorResponseType::MfaRequired,
                "Rauthy admin access only allowed with MFA active".to_string(),
            ));
        }

        if *ADMIN_ACCESS_SESSION_ONLY && !self.has_session {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Rauthy admin access denied with JWT tokens".to_string(),
            ));
        }

        Ok(())
    }
}
