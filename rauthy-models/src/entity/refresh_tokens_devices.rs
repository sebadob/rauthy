use crate::app_state::AppState;
use actix_web::web;
use chrono::{DateTime, Utc};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
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

    pub async fn delete(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        sqlx::query!("DELETE FROM refresh_tokens_devices WHERE id = $1", self.id)
            .execute(&data.db)
            .await?;
        Ok(())
    }

    pub async fn find_all(data: &web::Data<AppState>) -> Result<Vec<Self>, ErrorResponse> {
        let res = sqlx::query_as!(Self, "SELECT * FROM refresh_tokens_devices")
            .fetch_all(&data.db)
            .await?;
        Ok(res)
    }

    pub async fn invalidate_all(data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query!(
            "UPDATE refresh_tokens_devices SET exp = $1 WHERE exp > $1",
            now
        )
        .execute(&data.db)
        .await?;

        Ok(())
    }

    pub async fn invalidate_for_user(
        data: &web::Data<AppState>,
        user_id: &str,
    ) -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp();

        sqlx::query!(
            "UPDATE refresh_tokens_devices SET exp = $1 WHERE exp > $1 AND user_id = $2",
            now,
            user_id
        )
        .execute(&data.db)
        .await?;

        Ok(())
    }

    pub async fn find(data: &web::Data<AppState>, id: &str) -> Result<Self, ErrorResponse> {
        let now = Utc::now().timestamp();
        match sqlx::query_as!(
            Self,
            "SELECT * FROM refresh_tokens_devices WHERE id = $1 AND exp > $2",
            id,
            now
        )
        .fetch_one(&data.db)
        .await
        {
            Ok(res) => Ok(res),
            Err(_) => Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "Device Refresh Token does not exist".to_string(),
            )),
        }
    }

    pub async fn invalidate_all_for_device(
        data: &web::Data<AppState>,
        device_id: &str,
    ) -> Result<(), ErrorResponse> {
        sqlx::query!(
            "DELETE FROM refresh_tokens_devices WHERE device_id = $1",
            device_id
        )
        .execute(&data.db)
        .await?;

        Ok(())
    }

    pub async fn invalidate_all_for_user(
        data: &web::Data<AppState>,
        user_id: &str,
    ) -> Result<(), ErrorResponse> {
        sqlx::query!(
            "DELETE FROM refresh_tokens_devices WHERE user_id = $1",
            user_id
        )
        .execute(&data.db)
        .await?;

        Ok(())
    }

    pub async fn save(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        #[cfg(not(feature = "postgres"))]
        let q = sqlx::query!(
            r#"INSERT OR REPLACE INTO refresh_tokens_devices
                (id, device_id, user_id, nbf, exp, scope)
                VALUES ($1, $2, $3, $4, $5, $6)"#,
            self.id,
            self.device_id,
            self.user_id,
            self.nbf,
            self.exp,
            self.scope,
        );
        #[cfg(feature = "postgres")]
        let q = sqlx::query!(
            r#"INSERT INTO refresh_tokens_devices
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
        );

        q.execute(&data.db).await?;

        Ok(())
    }
}
