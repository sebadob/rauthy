use crate::database::DB;
use chrono::{DateTime, Utc};
use hiqlite::{Param, params};
use rauthy_common::is_hiqlite;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt::{Debug, Formatter};

#[derive(FromRow, Serialize, Deserialize)]
pub struct RefreshToken {
    pub id: String,
    pub user_id: String,
    pub nbf: i64,
    pub exp: i64,
    pub scope: Option<String>,
    pub is_mfa: bool,
    pub session_id: Option<String>,
}

impl Debug for RefreshToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}(...), user_id: {}, nbf: {}, exp: {}, scope: {:?}, is_mfa: {}, session_id: {:?}",
            &self.id[..5],
            self.user_id,
            self.nbf,
            self.exp,
            self.scope,
            self.is_mfa,
            self.session_id
        )
    }
}

// CRUD
impl RefreshToken {
    pub async fn create(
        id: String,
        user_id: String,
        nbf: DateTime<Utc>,
        exp: DateTime<Utc>,
        scope: Option<String>,
        // TODO should we even save mfa for refresh tokens?
        // even if the original token has been issued with mfa, the refresh
        // token not really is, because it can be given without user interaction.
        is_mfa: bool,
        session_id: Option<String>,
    ) -> Result<Self, ErrorResponse> {
        let rt = Self {
            id,
            user_id,
            nbf: nbf.timestamp(),
            exp: exp.timestamp(),
            scope,
            is_mfa,
            session_id,
        };

        rt.save().await?;
        Ok(rt)
    }

    pub async fn delete(self) -> Result<(), ErrorResponse> {
        if is_hiqlite() {
            DB::hql()
                .execute("DELETE FROM refresh_tokens WHERE id = $1", params!(self.id))
                .await?;
        } else {
            sqlx::query!("DELETE FROM refresh_tokens WHERE id = $1", self.id)
                .execute(DB::conn_sqlx())
                .await?;
        }
        Ok(())
    }

    pub async fn delete_by_sid(session_id: String) -> Result<(), ErrorResponse> {
        if is_hiqlite() {
            DB::hql()
                .execute(
                    "DELETE FROM refresh_tokens WHERE session_id = $1",
                    params!(session_id),
                )
                .await?;
        } else {
            sqlx::query!(
                "DELETE FROM refresh_tokens WHERE session_id = $1",
                session_id
            )
            .execute(DB::conn_sqlx())
            .await?;
        }
        Ok(())
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let res = if is_hiqlite() {
            DB::hql()
                .query_as("SELECT * FROM refresh_tokens", params!())
                .await?
        } else {
            sqlx::query_as!(Self, "SELECT * FROM refresh_tokens")
                .fetch_all(DB::conn_sqlx())
                .await?
        };
        Ok(res)
    }

    pub async fn invalidate_all() -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp();

        if is_hiqlite() {
            DB::hql()
                .execute(
                    "UPDATE refresh_tokens SET exp = $1 WHERE exp > $1",
                    params!(now),
                )
                .await?;
        } else {
            sqlx::query!("UPDATE refresh_tokens SET exp = $1 WHERE exp > $1", now)
                .execute(DB::conn_sqlx())
                .await?;
        }

        Ok(())
    }

    pub async fn invalidate_for_user(user_id: &str) -> Result<(), ErrorResponse> {
        if is_hiqlite() {
            DB::hql()
                .execute(
                    "DELETE FROM refresh_tokens WHERE user_id = $1",
                    params!(user_id),
                )
                .await?;
        } else {
            sqlx::query!("DELETE FROM refresh_tokens WHERE user_id = $1", user_id)
                .execute(DB::conn_sqlx())
                .await?;
        }
        Ok(())
    }

    pub async fn find(id: &str) -> Result<Self, ErrorResponse> {
        let now = Utc::now().timestamp();

        let slf = if is_hiqlite() {
            DB::hql()
                .query_as_one::<Self, _>(
                    "SELECT * FROM refresh_tokens WHERE id = $1 AND exp > $2",
                    params!(id, now),
                )
                .await
                .map_err(|_| {
                    ErrorResponse::new(ErrorResponseType::NotFound, "Refresh Token does not exist")
                })?
        } else {
            sqlx::query_as!(
                Self,
                "SELECT * FROM refresh_tokens WHERE id = $1 AND exp > $2",
                id,
                now
            )
            .fetch_one(DB::conn_sqlx())
            .await
            .map_err(|_| {
                ErrorResponse::new(ErrorResponseType::NotFound, "Refresh Token does not exist")
            })?
        };
        Ok(slf)
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        if is_hiqlite() {
            DB::hql()
                .execute(
                    r#"
INSERT INTO refresh_tokens (id, user_id, nbf, exp, scope, is_mfa, session_id)
VALUES ($1, $2, $3, $4, $5, $6, $7)
ON CONFLICT(id) DO UPDATE SET user_id = $2, nbf = $3, exp = $4, scope = $5, session_id = $7"#,
                    params!(
                        self.id.clone(),
                        self.user_id.clone(),
                        self.nbf,
                        self.exp,
                        self.scope.clone(),
                        self.is_mfa,
                        self.session_id.clone()
                    ),
                )
                .await?;
        } else {
            sqlx::query!(
                r#"
INSERT INTO refresh_tokens (id, user_id, nbf, exp, scope, is_mfa, session_id)
VALUES ($1, $2, $3, $4, $5, $6, $7)
ON CONFLICT(id) DO UPDATE SET user_id = $2, nbf = $3, exp = $4, scope = $5, session_id = $7"#,
                self.id,
                self.user_id,
                self.nbf,
                self.exp,
                self.scope,
                self.is_mfa,
                self.session_id,
            )
            .execute(DB::conn_sqlx())
            .await?;
        }

        Ok(())
    }
}

impl RefreshToken {
    pub async fn invalidate_all_for_user(user_id: &str) -> Result<(), ErrorResponse> {
        if is_hiqlite() {
            DB::hql()
                .execute(
                    "DELETE FROM refresh_tokens WHERE user_id = $1",
                    params!(user_id),
                )
                .await?;
        } else {
            sqlx::query!("DELETE FROM refresh_tokens WHERE user_id = $1", user_id)
                .execute(DB::conn_sqlx())
                .await?;
        }

        Ok(())
    }
}
