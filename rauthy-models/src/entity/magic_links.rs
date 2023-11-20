use crate::app_state::AppState;
use crate::real_ip_from_req;
use actix_web::{web, HttpRequest};
use rauthy_common::constants::{PWD_CSRF_HEADER, PWD_RESET_COOKIE, UNSAFE_NO_RESET_BINDING};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::get_rand;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use tracing::warn;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MagicLinkUsage {
    EmailChange(String),
    PasswordReset,
    NewUser,
}

impl TryFrom<&String> for MagicLinkUsage {
    type Error = ErrorResponse;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for MagicLinkUsage {
    type Error = ErrorResponse;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (ty, v) = value.split_once('$').unwrap_or((value, ""));
        let slf = match ty {
            "password_reset" => MagicLinkUsage::PasswordReset,
            "new_user" => MagicLinkUsage::NewUser,
            "email_change" => MagicLinkUsage::EmailChange(v.to_string()),
            _ => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "Invalid string for MagicLinkUsage parsing".to_string(),
                ))
            }
        };

        Ok(slf)
    }
}

impl ToString for MagicLinkUsage {
    // For types with a value, `$` was chosen as the separating characters since it is URL safe.
    // It also makes splitting of the value quite easy.
    fn to_string(&self) -> String {
        match self {
            MagicLinkUsage::PasswordReset => "password_reset".to_string(),
            MagicLinkUsage::NewUser => "new_user".to_string(),
            MagicLinkUsage::EmailChange(email) => format!("email_change${}", email),
        }
    }
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MagicLink {
    pub id: String,
    pub user_id: String,
    pub csrf_token: String,
    pub cookie: Option<String>,
    pub exp: i64,
    pub used: bool,
    pub usage: String,
}

// CRUD
impl MagicLink {
    pub async fn create(
        data: &web::Data<AppState>,
        user_id: String,
        lifetime_minutes: i64,
        usage: MagicLinkUsage,
    ) -> Result<Self, ErrorResponse> {
        let id = get_rand(64);
        let exp = OffsetDateTime::now_utc().unix_timestamp() + lifetime_minutes * 60;
        let link = MagicLink {
            id,
            user_id,
            csrf_token: get_rand(48),
            cookie: None,
            exp,
            used: false,
            usage: usage.to_string(),
        };

        sqlx::query!(
            r#"insert into magic_links (id, user_id, csrf_token, exp, used, usage)
            values ($1, $2, $3, $4, $5, $6)"#,
            link.id,
            link.user_id,
            link.csrf_token,
            link.exp,
            false,
            link.usage,
        )
        .execute(&data.db)
        .await?;

        Ok(link)
    }

    pub async fn find(data: &web::Data<AppState>, id: &str) -> Result<Self, ErrorResponse> {
        let res = sqlx::query_as!(Self, "select * from magic_links where id = $1", id)
            .fetch_one(&data.db)
            .await?;

        Ok(res)
    }

    pub async fn find_by_user(
        data: &web::Data<AppState>,
        user_id: String,
    ) -> Result<MagicLink, ErrorResponse> {
        let res = sqlx::query_as!(
            Self,
            "select * from magic_links where user_id = $1",
            user_id
        )
        .fetch_one(&data.db)
        .await?;

        Ok(res)
    }

    pub async fn invalidate_all_email_change(
        data: &web::Data<AppState>,
        user_id: &str,
    ) -> Result<(), ErrorResponse> {
        sqlx::query!(
            "delete from magic_links where user_id = $1 and usage like 'email_change$%'",
            user_id,
        )
        .execute(&data.db)
        .await?;

        Ok(())
    }

    pub async fn save(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        sqlx::query!(
            "update magic_links set cookie = $1, exp = $2, used = $3 where id = $4",
            self.cookie,
            self.exp,
            self.used,
            self.id,
        )
        .execute(&data.db)
        .await?;

        Ok(())
    }
}

impl MagicLink {
    pub async fn invalidate(&mut self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        self.exp = OffsetDateTime::now_utc().unix_timestamp() - 10;
        self.save(data).await
    }

    pub fn validate(
        &self,
        user_id: &str,
        req: &HttpRequest,
        with_csrf: bool,
    ) -> Result<(), ErrorResponse> {
        // binding cookie
        if self.cookie.is_some() {
            let err = ErrorResponse::new(
                ErrorResponseType::Forbidden,
                String::from(
                    "The requested password reset link is already tied to another session",
                ),
            );

            let cookie_opt = req.cookie(PWD_RESET_COOKIE);
            if let Some(cookie) = cookie_opt {
                // the extracted cookie from the request starts with 'rauthy-pwd-reset='
                if !cookie.value().ends_with(self.cookie.as_ref().unwrap()) {
                    if *UNSAFE_NO_RESET_BINDING {
                        let ip = real_ip_from_req(req).unwrap_or_default();
                        warn!("UNSAFE_RESET_NO_BINDING is set to true -> ignoring invalid binding cookie from {}", ip);
                    } else {
                        return Err(err);
                    }
                }
            } else {
                if *UNSAFE_NO_RESET_BINDING {
                    let ip = real_ip_from_req(req).unwrap_or_default();
                    warn!("UNSAFE_RESET_NO_BINDING is set to true -> ignoring invalid binding cookie from {}", ip);
                } else {
                    return Err(err);
                }
            }
        }

        // csrf token
        if with_csrf {
            match req.headers().get(PWD_CSRF_HEADER) {
                None => {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::Unauthorized,
                        String::from("CSRF Token is missing"),
                    ));
                }
                Some(token) => {
                    if self.csrf_token != token.to_str().unwrap_or("") {
                        return Err(ErrorResponse::new(
                            ErrorResponseType::Unauthorized,
                            String::from("Invalid CSRF Token"),
                        ));
                    }
                }
            }
        }

        if self.user_id != user_id {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                String::from("The user id is invalid"),
            ));
        }

        if self.exp < OffsetDateTime::now_utc().unix_timestamp() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                String::from("This link has expired already"),
            ));
        }

        if self.used {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                String::from("The requested passwort reset link was already used"),
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdxMagicLinkPasswordUser {
    pub user_id: String,
    pub id: String,
}

impl IdxMagicLinkPasswordUser {
    pub fn from_magic_link(ml: &MagicLink) -> Self {
        Self {
            user_id: ml.user_id.clone(),
            id: ml.id.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::entity::magic_links::MagicLinkUsage;

    #[test]
    fn test_magic_link_usage_conversions() {
        let ml = MagicLinkUsage::NewUser;
        let s = ml.to_string();
        let ml_from = MagicLinkUsage::try_from(&s).unwrap();
        assert_eq!(ml, ml_from);

        let ml = MagicLinkUsage::PasswordReset;
        let s = ml.to_string();
        let ml_from = MagicLinkUsage::try_from(&s).unwrap();
        assert_eq!(ml, ml_from);

        let ml = MagicLinkUsage::EmailChange("admin@localhost.de".to_string());
        let s = ml.to_string();
        let ml_from = MagicLinkUsage::try_from(&s).unwrap();
        assert_eq!(ml, ml_from);
    }
}
