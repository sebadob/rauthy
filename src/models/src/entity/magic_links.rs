use crate::api_cookie::ApiCookie;
use crate::database::DB;
use actix_web::HttpRequest;
use hiqlite::{Param, params};
use rauthy_common::constants::{PASSWORD_RESET_COOKIE_BINDING, PWD_CSRF_HEADER, PWD_RESET_COOKIE};
use rauthy_common::is_hiqlite;
use rauthy_common::utils::{get_rand, real_ip_from_req};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt::{Debug, Display, Formatter};
use time::OffsetDateTime;
use tracing::warn;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MagicLinkUsage {
    EmailChange(String),
    PasswordReset(Option<String>),
    NewUser(Option<String>),
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
            "email_change" => MagicLinkUsage::EmailChange(v.to_string()),
            "new_user" => {
                if !v.is_empty() {
                    MagicLinkUsage::NewUser(Some(v.to_string()))
                } else {
                    MagicLinkUsage::NewUser(None)
                }
            }
            "password_reset" => {
                if !v.is_empty() {
                    MagicLinkUsage::PasswordReset(Some(v.to_string()))
                } else {
                    MagicLinkUsage::PasswordReset(None)
                }
            }
            _ => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "Invalid string for MagicLinkUsage parsing",
                ));
            }
        };

        Ok(slf)
    }
}

impl Display for MagicLinkUsage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // For types with a value, `$` was chosen as the separating characters since it is URL safe.
        // It also makes splitting of the value quite easy.
        match self {
            MagicLinkUsage::EmailChange(email) => write!(f, "email_change${}", email),
            MagicLinkUsage::NewUser(redirect_uri) => {
                if let Some(uri) = redirect_uri {
                    write!(f, "new_user${}", uri)
                } else {
                    write!(f, "new_user")
                }
            }
            MagicLinkUsage::PasswordReset(redirect_uri) => {
                if let Some(uri) = redirect_uri {
                    write!(f, "password_reset${}", uri)
                } else {
                    write!(f, "password_reset")
                }
            }
        }
    }
}

#[derive(Clone, FromRow, Serialize, Deserialize)]
pub struct MagicLink {
    pub id: String,
    pub user_id: String,
    pub csrf_token: String,
    pub cookie: Option<String>,
    pub exp: i64,
    pub used: bool,
    pub usage: String,
}

impl Debug for MagicLink {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}(...), user_id: {}, csrf_token: {}(...), cookie: {:?}, exp: {}, used: {}, usage: {}",
            &self.id[..5],
            self.user_id,
            &self.csrf_token[..5],
            self.cookie,
            self.exp,
            self.used,
            self.usage
        )
    }
}

// CRUD
impl MagicLink {
    pub async fn create(
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

        if is_hiqlite() {
            DB::hql()
                .execute(
                    r#"
INSERT INTO magic_links (id, user_id, csrf_token, exp, used, usage)
VALUES ($1, $2, $3, $4, $5, $6)"#,
                    params!(
                        link.id.clone(),
                        link.user_id.clone(),
                        link.csrf_token.clone(),
                        link.exp,
                        false,
                        link.usage.clone()
                    ),
                )
                .await?;
        } else {
            sqlx::query!(
                r#"
INSERT INTO magic_links (id, user_id, csrf_token, exp, used, usage)
VALUES ($1, $2, $3, $4, $5, $6)"#,
                link.id,
                link.user_id,
                link.csrf_token,
                link.exp,
                false,
                link.usage,
            )
            .execute(DB::conn_sqlx())
            .await?;
        }

        Ok(link)
    }

    pub async fn delete_all_pwd_reset_for_user(user_id: String) -> Result<(), ErrorResponse> {
        if is_hiqlite() {
            DB::hql()
                .execute(
                    "DELETE FROM magic_links WHERE user_id = $1 AND usage LIKE 'password_reset%'",
                    params!(user_id),
                )
                .await?;
        } else {
            sqlx::query!(
                "DELETE FROM magic_links WHERE user_id = $1 AND usage LIKE 'password_reset%'",
                user_id,
            )
            .execute(DB::conn_sqlx())
            .await?;
        };

        Ok(())
    }

    pub async fn find(id: &str) -> Result<Self, ErrorResponse> {
        let res = if is_hiqlite() {
            DB::hql()
                .query_as_one("SELECT * FROM magic_links WHERE id = $1", params!(id))
                .await?
        } else {
            sqlx::query_as!(Self, "SELECT * FROM magic_links WHERE id = $1", id)
                .fetch_one(DB::conn_sqlx())
                .await?
        };

        Ok(res)
    }

    pub async fn find_by_user(user_id: String) -> Result<MagicLink, ErrorResponse> {
        let res = if is_hiqlite() {
            DB::hql()
                .query_as_one(
                    "SELECT * FROM magic_links WHERE user_id = $1",
                    params!(user_id),
                )
                .await?
        } else {
            sqlx::query_as!(
                Self,
                "SELECT * FROM magic_links WHERE user_id = $1",
                user_id
            )
            .fetch_one(DB::conn_sqlx())
            .await?
        };

        Ok(res)
    }

    pub async fn invalidate_all_email_change(user_id: &str) -> Result<(), ErrorResponse> {
        if is_hiqlite() {
            DB::hql()
                .execute(
                    "DELETE FROM magic_links WHERE user_id = $1 AND usage LIKE 'email_change$%'",
                    params!(user_id),
                )
                .await?;
        } else {
            sqlx::query!(
                "DELETE FROM magic_links WHERE user_id = $1 AND usage LIKE 'email_change$%'",
                user_id,
            )
            .execute(DB::conn_sqlx())
            .await?;
        };

        Ok(())
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        if is_hiqlite() {
            DB::hql()
                .execute(
                    "UPDATE magic_links SET cookie = $1, exp = $2, used = $3 WHERE id = $4",
                    params!(self.cookie.clone(), self.exp, self.used, self.id.clone()),
                )
                .await?;
        } else {
            sqlx::query!(
                "UPDATE magic_links SET cookie = $1, exp = $2, used = $3 WHERE id = $4",
                self.cookie,
                self.exp,
                self.used,
                self.id,
            )
            .execute(DB::conn_sqlx())
            .await?;
        }

        Ok(())
    }
}

impl MagicLink {
    /// Sets the magic link as being expired and used.
    pub async fn invalidate(&mut self) -> Result<(), ErrorResponse> {
        self.exp = OffsetDateTime::now_utc().unix_timestamp() - 10;
        self.used = true;
        self.save().await
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
                "The requested password reset link is already tied to another session",
            );

            let cookie_opt = ApiCookie::from_req(req, PWD_RESET_COOKIE);
            if let Some(cookie) = cookie_opt {
                // the extracted cookie from the request starts with 'rauthy-pwd-reset='
                if !cookie.ends_with(self.cookie.as_ref().unwrap()) {
                    if *PASSWORD_RESET_COOKIE_BINDING {
                        return Err(err);
                    } else {
                        let ip = real_ip_from_req(req)?;
                        warn!(
                            "PASSWORD_RESET_COOKIE_BINDING disabled -> ignoring invalid binding cookie from {}",
                            ip
                        );
                    }
                }
            } else if *PASSWORD_RESET_COOKIE_BINDING {
                return Err(err);
            } else {
                let ip = real_ip_from_req(req)?;
                warn!(
                    "PASSWORD_RESET_COOKIE_BINDING disabled -> ignoring invalid binding cookie from {}",
                    ip
                );
            }
        }

        // csrf token
        if with_csrf {
            match req.headers().get(PWD_CSRF_HEADER) {
                None => {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::Unauthorized,
                        "CSRF Token is missing",
                    ));
                }
                Some(token) => {
                    if self.csrf_token != token.to_str().unwrap_or("") {
                        return Err(ErrorResponse::new(
                            ErrorResponseType::Unauthorized,
                            "Invalid CSRF Token",
                        ));
                    }
                }
            }
        }

        if self.user_id != user_id {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Invalid User ID",
            ));
        }

        if self.exp < OffsetDateTime::now_utc().unix_timestamp() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "This link has expired already",
            ));
        }

        if self.used {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "The requested link has been used already",
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
        let ml = MagicLinkUsage::NewUser(None);
        let s = ml.to_string();
        let ml_from = MagicLinkUsage::try_from(&s).unwrap();
        assert_eq!(ml, ml_from);

        let ml = MagicLinkUsage::NewUser(Some("custom.uri.com".to_string()));
        let s = ml.to_string();
        let ml_from = MagicLinkUsage::try_from(&s).unwrap();
        assert_eq!(ml, ml_from);

        let ml = MagicLinkUsage::PasswordReset(None);
        let s = ml.to_string();
        let ml_from = MagicLinkUsage::try_from(&s).unwrap();
        assert_eq!(ml, ml_from);

        let ml = MagicLinkUsage::PasswordReset(Some("custom.uri.com".to_string()));
        let s = ml.to_string();
        let ml_from = MagicLinkUsage::try_from(&s).unwrap();
        assert_eq!(ml, ml_from);

        let ml = MagicLinkUsage::EmailChange("admin@localhost.de".to_string());
        let s = ml.to_string();
        let ml_from = MagicLinkUsage::try_from(&s).unwrap();
        assert_eq!(ml, ml_from);
    }
}
