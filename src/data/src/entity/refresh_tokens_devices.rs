use crate::database::DB;
use hiqlite::macros::{FromRow, params};
use rauthy_common::is_hiqlite;
use rauthy_derive::FromPgRow;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

#[derive(Serialize, Deserialize, FromRow, FromPgRow)]
pub struct RefreshTokenDevice {
    pub id: String,
    pub device_id: String,
    pub user_id: String,
    pub nbf: i64,
    pub exp: i64,
    pub scope: Option<String>,
    pub access_token_jti: Option<String>,
}

impl Debug for RefreshTokenDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RefreshTokenDevice {{ id: {}(...), device_id: {}, nbf: {}, exp: {}, scope: {:?}, \
            access_token_jti: {:?} }}",
            &self.id[..5],
            self.device_id,
            self.nbf,
            self.exp,
            self.scope,
            self.access_token_jti,
        )
    }
}

// CRUD
impl RefreshTokenDevice {
    pub async fn create(
        id: String,
        device_id: String,
        user_id: String,
        nbf: i64,
        exp: i64,
        scope: Option<String>,
        access_token_jti: Option<String>,
    ) -> Result<Self, ErrorResponse> {
        let rt = Self {
            id,
            device_id,
            user_id,
            nbf,
            exp,
            scope,
            access_token_jti,
        };

        rt.save().await?;
        Ok(rt)
    }

    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM refresh_tokens_devices WHERE id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(self.id.clone())).await?;
        } else {
            DB::pg_execute(sql, &[&self.id]).await?;
        }

        Ok(())
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM refresh_tokens_devices";
        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 0).await?
        };
        Ok(res)
    }

    pub async fn invalidate_all() -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM refresh_tokens_devices";
        if is_hiqlite() {
            DB::hql().execute(sql, params!()).await?;
        } else {
            DB::pg_execute(sql, &[]).await?;
        }

        Ok(())
    }

    pub async fn invalidate_all_for_user(user_id: &str) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM refresh_tokens_devices WHERE user_id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(user_id)).await?;
        } else {
            DB::pg_execute(sql, &[&user_id]).await?;
        }

        Ok(())
    }

    /// Finds and delete the refresh token in an atomic operation.
    pub async fn find_delete(id: &str) -> Result<Self, ErrorResponse> {
        let sql = "DELETE FROM refresh_tokens_devices WHERE id = $1 RETURNING *";

        let slf = if is_hiqlite() {
            DB::hql()
                .execute_returning_map_one(sql, params!(id))
                .await?
        } else {
            DB::pg_query_one(sql, &[&id]).await?
        };

        Ok(slf)
    }

    pub async fn find_opt(id: &str) -> Result<Option<Self>, ErrorResponse> {
        let sql = "SELECT * FROM refresh_tokens_devices WHERE id = $1";

        let slf = if is_hiqlite() {
            DB::hql().query_map_optional(sql, params!(id)).await?
        } else {
            DB::pg_query_opt(sql, &[&id]).await?
        };

        Ok(slf)
    }

    pub async fn find_by_user_id_jti(
        user_id: &str,
        jti: &str,
    ) -> Result<Option<Self>, ErrorResponse> {
        let sql =
            "SELECT * FROM refresh_tokens_devices WHERE user_id = $1 AND access_token_jti = $2";

        let slf = if is_hiqlite() {
            DB::hql()
                .query_map_optional(sql, params!(user_id, jti))
                .await?
        } else {
            DB::pg_query_opt(sql, &[&user_id, &jti]).await?
        };

        Ok(slf)
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        let sql = r#"
INSERT INTO refresh_tokens_devices
(id, device_id, user_id, nbf, exp, scope, access_token_jti)
VALUES ($1, $2, $3, $4, $5, $6, $7)
ON CONFLICT(id) DO UPDATE
SET device_id = $2, user_id = $3, nbf = $4, exp = $5, scope = $6, access_token_jti = $7"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        self.id.clone(),
                        self.device_id.clone(),
                        self.user_id.clone(),
                        self.nbf,
                        self.exp,
                        self.scope.clone(),
                        self.access_token_jti.clone()
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &self.id,
                    &self.device_id,
                    &self.user_id,
                    &self.nbf,
                    &self.exp,
                    &self.scope,
                    &self.access_token_jti,
                ],
            )
            .await?;
        }

        Ok(())
    }
}
