use crate::database::DB;
use chrono::Utc;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::Deserialize;
use std::fmt::{Debug, Formatter};

#[derive(Deserialize)]
pub struct RefreshToken {
    pub id: String,
    pub user_id: String,
    pub nbf: i64,
    pub exp: i64,
    pub scope: Option<String>,
    pub is_mfa: bool,
    pub session_id: Option<String>,
    pub access_token_jti: Option<String>,
}

impl Debug for RefreshToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RefreshToken {{ id: {}(...), user_id: {}, nbf: {}, exp: {}, scope: {:?}, is_mfa: {}, \
            session_id: {:?} }}",
            &self.id[..5],
            self.user_id,
            self.nbf,
            self.exp,
            self.scope,
            self.is_mfa,
            self.session_id.as_ref().map(|sid| &sid[..5]),
        )
    }
}

impl From<tokio_postgres::Row> for RefreshToken {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            user_id: row.get("user_id"),
            nbf: row.get("nbf"),
            exp: row.get("exp"),
            scope: row.get("scope"),
            is_mfa: row.get("is_mfa"),
            session_id: row.get("session_id"),
            access_token_jti: row.get("access_token_jti"),
        }
    }
}

// CRUD
impl RefreshToken {
    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        id: String,
        user_id: String,
        nbf: i64,
        exp: i64,
        scope: Option<String>,
        // TODO should we even save mfa for refresh tokens?
        //  even if the original token has been issued with mfa, the refresh
        //  token not really is, because it can be given without user interaction.
        is_mfa: bool,
        session_id: Option<String>,
        access_token_jti: Option<String>,
    ) -> Result<Self, ErrorResponse> {
        let rt = Self {
            id,
            user_id,
            nbf,
            exp,
            scope,
            is_mfa,
            session_id,
            access_token_jti,
        };

        rt.save().await?;
        Ok(rt)
    }

    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM refresh_tokens WHERE id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(self.id.clone())).await?;
        } else {
            DB::pg_execute(sql, &[&self.id]).await?;
        }
        Ok(())
    }

    pub async fn delete_by_sid(session_id: String) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM refresh_tokens WHERE session_id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(session_id)).await?;
        } else {
            DB::pg_execute(sql, &[&session_id]).await?;
        }
        Ok(())
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM refresh_tokens";
        let res = if is_hiqlite() {
            DB::hql().query_as(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 0).await?
        };
        Ok(res)
    }

    pub async fn find_by_user_id_jti(
        user_id: &str,
        access_token_jti: &str,
    ) -> Result<Option<Self>, ErrorResponse> {
        let sql = "SELECT * FROM refresh_tokens WHERE user_id = $1 AND access_token_jti = $2";

        let slf = if is_hiqlite() {
            DB::hql()
                .query_as_optional(sql, params!(user_id, access_token_jti))
                .await?
        } else {
            DB::pg_query_opt(sql, &[&user_id, &access_token_jti]).await?
        };

        Ok(slf)
    }

    pub async fn invalidate_all() -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp();
        let sql = "UPDATE refresh_tokens SET exp = $1 WHERE exp > $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(now)).await?;
        } else {
            DB::pg_execute(sql, &[&now]).await?;
        }

        Ok(())
    }

    pub async fn invalidate_for_user(user_id: &str) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM refresh_tokens WHERE user_id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(user_id)).await?;
        } else {
            DB::pg_execute(sql, &[&user_id]).await?;
        }
        Ok(())
    }

    pub async fn find(id: &str) -> Result<Self, ErrorResponse> {
        let now = Utc::now().timestamp();
        let sql = "SELECT * FROM refresh_tokens WHERE id = $1 AND exp > $2";

        let slf: Self = if is_hiqlite() {
            DB::hql()
                .query_as_one(sql, params!(id, now))
                .await
                .map_err(|_| {
                    ErrorResponse::new(ErrorResponseType::NotFound, "Refresh Token does not exist")
                })?
        } else {
            DB::pg_query_one(sql, &[&id, &now]).await.map_err(|_| {
                ErrorResponse::new(ErrorResponseType::NotFound, "Refresh Token does not exist")
            })?
        };

        Ok(slf)
    }

    pub async fn find_opt(id: &str) -> Result<Option<Self>, ErrorResponse> {
        let sql = "SELECT * FROM refresh_tokens WHERE id = $1";

        let slf = if is_hiqlite() {
            DB::hql().query_as_optional(sql, params!(id)).await?
        } else {
            DB::pg_query_opt(sql, &[&id]).await?
        };

        Ok(slf)
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        let sql = r#"
INSERT INTO refresh_tokens (id, user_id, nbf, exp, scope, is_mfa, session_id, access_token_jti)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
ON CONFLICT(id) DO UPDATE
SET user_id = $2, nbf = $3, exp = $4, scope = $5, session_id = $7, access_token_jti = $8"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        self.id.clone(),
                        self.user_id.clone(),
                        self.nbf,
                        self.exp,
                        self.scope.clone(),
                        self.is_mfa,
                        self.session_id.clone(),
                        self.access_token_jti.clone()
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &self.id,
                    &self.user_id,
                    &self.nbf,
                    &self.exp,
                    &self.scope,
                    &self.is_mfa,
                    &self.session_id,
                    &self.access_token_jti,
                ],
            )
            .await?;
        }

        Ok(())
    }
}
