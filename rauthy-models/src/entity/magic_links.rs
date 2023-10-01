use actix_web::{web, HttpRequest};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

use rauthy_common::constants::{PWD_CSRF_HEADER, PWD_RESET_COOKIE};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::get_rand;

use crate::app_state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MagicLinkUsage {
    PasswordReset,
    NewUser,
}

impl TryFrom<&str> for MagicLinkUsage {
    type Error = ErrorResponse;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let slf = match value {
            "password_reset" => MagicLinkUsage::PasswordReset,
            "new_user" => MagicLinkUsage::NewUser,
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

impl MagicLinkUsage {
    pub fn as_str(&self) -> &str {
        match self {
            MagicLinkUsage::PasswordReset => "password_reset",
            MagicLinkUsage::NewUser => "new_user",
        }
    }
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MagicLinkPassword {
    pub id: String,
    pub user_id: String,
    pub csrf_token: String,
    pub cookie: Option<String>,
    pub exp: i64,
    pub used: bool,
    pub usage: String,
}

// CRUD
impl MagicLinkPassword {
    pub async fn create(
        data: &web::Data<AppState>,
        user_id: String,
        lifetime_minutes: i64,
        usage: MagicLinkUsage,
    ) -> Result<Self, ErrorResponse> {
        let id = get_rand(64);
        let exp = OffsetDateTime::now_utc().unix_timestamp() + lifetime_minutes * 60;
        let link = MagicLinkPassword {
            id,
            user_id,
            csrf_token: get_rand(48),
            cookie: None,
            exp,
            used: false,
            usage: usage.as_str().to_string(),
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
    ) -> Result<MagicLinkPassword, ErrorResponse> {
        let res = sqlx::query_as!(
            Self,
            "select * from magic_links where user_id = $1",
            user_id
        )
        .fetch_one(&data.db)
        .await?;

        Ok(res)
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

impl MagicLinkPassword {
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
                    return Err(err);
                }
            } else {
                return Err(err);
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
    pub fn from_magic_link(ml: &MagicLinkPassword) -> Self {
        Self {
            user_id: ml.user_id.clone(),
            id: ml.id.clone(),
        }
    }
}
