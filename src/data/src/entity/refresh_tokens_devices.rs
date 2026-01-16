use crate::database::DB;
use chrono::Utc;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize)]
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

impl From<tokio_postgres::Row> for RefreshTokenDevice {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            device_id: row.get("device_id"),
            user_id: row.get("user_id"),
            nbf: row.get("nbf"),
            exp: row.get("exp"),
            scope: row.get("scope"),
            access_token_jti: row.get("access_token_jti"),
        }
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
            DB::hql().query_as(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 0).await?
        };
        Ok(res)
    }

    pub async fn invalidate_all() -> Result<(), ErrorResponse> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let sql = "UPDATE refresh_tokens_devices SET exp = $1 WHERE exp > $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(now)).await?;
        } else {
            DB::pg_execute(sql, &[&now]).await?;
        }

        Ok(())
    }

    // pub async fn invalidate_for_user(user_id: &str) -> Result<(), ErrorResponse> {
    //     let now = Utc::now().timestamp();
    //     let sql = "UPDATE refresh_tokens_devices SET exp = $1 WHERE exp > $1 AND user_id = $2";
    //     if is_hiqlite() {
    //         DB::hql().execute(sql, params!(now, user_id)).await?;
    //     } else {
    //         DB::pg_execute(sql, &[&now, &user_id]).await?;
    //     }
    //
    //     Ok(())
    // }

    pub async fn find(id: &str) -> Result<Self, ErrorResponse> {
        let now = Utc::now().timestamp();
        let sql = "SELECT * FROM refresh_tokens_devices WHERE id = $1 AND exp > $2";

        if is_hiqlite() {
            DB::hql()
                .query_as_one(sql, params!(id, now))
                .await
                .map_err(|_| {
                    ErrorResponse::new(
                        ErrorResponseType::NotFound,
                        "Device Refresh Token does not exist",
                    )
                })?
        } else {
            DB::pg_query_one(sql, &[&id, &now]).await.map_err(|_| {
                ErrorResponse::new(
                    ErrorResponseType::NotFound,
                    "Device Refresh Token does not exist",
                )
            })
        }
    }

    pub async fn find_opt(id: &str) -> Result<Option<Self>, ErrorResponse> {
        let sql = "SELECT * FROM refresh_tokens_devices WHERE id = $1";

        let slf = if is_hiqlite() {
            DB::hql().query_as_optional(sql, params!(id)).await?
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
                .query_as_optional(sql, params!(user_id, jti))
                .await?
        } else {
            DB::pg_query_opt(sql, &[&user_id, &jti]).await?
        };

        Ok(slf)
    }

    pub async fn invalidate_all_for_device(device_id: &str) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM refresh_tokens_devices WHERE device_id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(device_id)).await?;
        } else {
            DB::pg_execute(sql, &[&device_id]).await?;
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
