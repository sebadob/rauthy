use crate::app_state::AppState;
use crate::hiqlite::DB;
use actix_web::web;
use chrono::{DateTime, Utc};
use hiqlite::{params, Param};
use rauthy_common::is_hiqlite;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct RefreshTokenDevice {
    pub id: String,
    pub device_id: String,
    pub user_id: String,
    pub nbf: i64,
    pub exp: i64,
    pub scope: Option<String>,
}

// CRUD
impl RefreshTokenDevice {
    pub async fn create(
        data: &web::Data<AppState>,
        id: String,
        device_id: String,
        user_id: String,
        nbf: DateTime<Utc>,
        exp: DateTime<Utc>,
        scope: Option<String>,
    ) -> Result<Self, ErrorResponse> {
        let rt = Self {
            id,
            device_id,
            user_id,
            nbf: nbf.timestamp(),
            exp: exp.timestamp(),
            scope,
        };

        rt.save(data).await?;
        Ok(rt)
    }

    pub async fn delete(self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        if is_hiqlite() {
            DB::client()
                .execute(
                    "DELETE FROM refresh_tokens_devices WHERE id = $1",
                    params!(self.id),
                )
                .await?;
        } else {
            sqlx::query!("DELETE FROM refresh_tokens_devices WHERE id = $1", self.id)
                .execute(&data.db)
                .await?;
        }
        Ok(())
    }

    pub async fn find_all(data: &web::Data<AppState>) -> Result<Vec<Self>, ErrorResponse> {
        let res = if is_hiqlite() {
            DB::client()
                .query_as("SELECT * FROM refresh_tokens_devices", params!())
                .await?
        } else {
            sqlx::query_as!(Self, "SELECT * FROM refresh_tokens_devices")
                .fetch_all(&data.db)
                .await?
        };
        Ok(res)
    }

    pub async fn invalidate_all(data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        let now = OffsetDateTime::now_utc().unix_timestamp();

        if is_hiqlite() {
            DB::client()
                .execute(
                    "UPDATE refresh_tokens_devices SET exp = $1 WHERE exp > $1",
                    params!(now),
                )
                .await?;
        } else {
            sqlx::query!(
                "UPDATE refresh_tokens_devices SET exp = $1 WHERE exp > $1",
                now
            )
            .execute(&data.db)
            .await?;
        }

        Ok(())
    }

    pub async fn invalidate_for_user(
        data: &web::Data<AppState>,
        user_id: &str,
    ) -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp();

        if is_hiqlite() {
            DB::client()
                .execute(
                    "UPDATE refresh_tokens_devices SET exp = $1 WHERE exp > $1 AND user_id = $2",
                    params!(now, user_id),
                )
                .await?;
        } else {
            sqlx::query!(
                "UPDATE refresh_tokens_devices SET exp = $1 WHERE exp > $1 AND user_id = $2",
                now,
                user_id
            )
            .execute(&data.db)
            .await?;
        }

        Ok(())
    }

    pub async fn find(data: &web::Data<AppState>, id: &str) -> Result<Self, ErrorResponse> {
        let now = Utc::now().timestamp();

        if is_hiqlite() {
            DB::client()
                .query_as_one(
                    "SELECT * FROM refresh_tokens_devices WHERE id = $1 AND exp > $2",
                    params!(id, now),
                )
                .await
                .map_err(|_| {
                    ErrorResponse::new(
                        ErrorResponseType::NotFound,
                        "Device Refresh Token does not exist",
                    )
                })?
        } else {
            sqlx::query_as!(
                Self,
                "SELECT * FROM refresh_tokens_devices WHERE id = $1 AND exp > $2",
                id,
                now
            )
            .fetch_one(&data.db)
            .await
            .map_err(|_| {
                ErrorResponse::new(
                    ErrorResponseType::NotFound,
                    "Device Refresh Token does not exist",
                )
            })
        }
    }

    pub async fn invalidate_all_for_device(
        data: &web::Data<AppState>,
        device_id: &str,
    ) -> Result<(), ErrorResponse> {
        if is_hiqlite() {
            DB::client()
                .execute(
                    "DELETE FROM refresh_tokens_devices WHERE device_id = $1",
                    params!(device_id),
                )
                .await?;
        } else {
            sqlx::query!(
                "DELETE FROM refresh_tokens_devices WHERE device_id = $1",
                device_id
            )
            .execute(&data.db)
            .await?;
        }

        Ok(())
    }

    pub async fn invalidate_all_for_user(
        data: &web::Data<AppState>,
        user_id: &str,
    ) -> Result<(), ErrorResponse> {
        if is_hiqlite() {
            DB::client()
                .execute(
                    "DELETE FROM refresh_tokens_devices WHERE user_id = $1",
                    params!(user_id),
                )
                .await?;
        } else {
            sqlx::query!(
                "DELETE FROM refresh_tokens_devices WHERE user_id = $1",
                user_id
            )
            .execute(&data.db)
            .await?;
        }

        Ok(())
    }

    pub async fn save(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        if is_hiqlite() {
            DB::client()
                .execute(
                    r#"
INSERT INTO refresh_tokens_devices
(id, device_id, user_id, nbf, exp, scope)
VALUES ($1, $2, $3, $4, $5, $6)
ON CONFLICT(id) DO UPDATE
SET device_id = $2, user_id = $3, nbf = $4, exp = $5, scope = $6"#,
                    params!(
                        self.id.clone(),
                        self.device_id.clone(),
                        self.user_id.clone(),
                        self.nbf,
                        self.exp,
                        self.scope.clone()
                    ),
                )
                .await?;
        } else {
            sqlx::query!(
                r#"
INSERT INTO refresh_tokens_devices
(id, device_id, user_id, nbf, exp, scope)
VALUES ($1, $2, $3, $4, $5, $6)
ON CONFLICT(id) DO UPDATE
SET device_id = $2, user_id = $3, nbf = $4, exp = $5, scope = $6"#,
                self.id,
                self.device_id,
                self.user_id,
                self.nbf,
                self.exp,
                self.scope,
            )
            .execute(&data.db)
            .await?;
        }

        Ok(())
    }
}
