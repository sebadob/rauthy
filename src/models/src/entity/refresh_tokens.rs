use crate::app_state::AppState;
use actix_web::web;
use chrono::{DateTime, Utc};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct RefreshToken {
    pub id: String,
    pub user_id: String,
    pub nbf: i64,
    pub exp: i64,
    pub scope: Option<String>,
    pub is_mfa: bool,
}

// CRUD
impl RefreshToken {
    pub async fn create(
        data: &web::Data<AppState>,
        id: String,
        user_id: String,
        nbf: DateTime<Utc>,
        exp: DateTime<Utc>,
        scope: Option<String>,
        // TODO should we even save mfa for refresh tokens?
        // even if the original token has been issued with mfa, the refresh
        // token not really is, because it can be given without user interaction.
        is_mfa: bool,
    ) -> Result<Self, ErrorResponse> {
        let rt = Self {
            id,
            user_id,
            nbf: nbf.timestamp(),
            exp: exp.timestamp(),
            scope,
            is_mfa,
        };

        rt.save(data).await?;
        Ok(rt)
    }

    pub async fn delete(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        sqlx::query!("DELETE FROM refresh_tokens WHERE id = $1", self.id)
            .execute(&data.db)
            .await?;
        Ok(())
    }

    pub async fn find_all(data: &web::Data<AppState>) -> Result<Vec<Self>, ErrorResponse> {
        let res = sqlx::query_as!(Self, "SELECT * FROM refresh_tokens")
            .fetch_all(&data.db)
            .await?;
        Ok(res)
    }

    pub async fn invalidate_all(data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp();

        sqlx::query!("UPDATE refresh_tokens SET exp = $1 WHERE exp > $1", now,)
            .execute(&data.db)
            .await?;

        Ok(())
    }

    pub async fn invalidate_for_user(
        data: &web::Data<AppState>,
        user_id: &str,
    ) -> Result<(), ErrorResponse> {
        sqlx::query!("DELETE FROM refresh_tokens WHERE user_id = $1", user_id)
            .execute(&data.db)
            .await?;
        Ok(())
    }

    pub async fn find(data: &web::Data<AppState>, id: &str) -> Result<Self, ErrorResponse> {
        let now = Utc::now().timestamp();
        match sqlx::query_as!(
            Self,
            "SELECT * FROM refresh_tokens WHERE id = $1 AND exp > $2",
            id,
            now
        )
        .fetch_one(&data.db)
        .await
        {
            Ok(res) => Ok(res),
            Err(_) => Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "Refresh Token does not exist",
            )),
        }
    }

    pub async fn save(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        #[cfg(not(feature = "postgres"))]
        let q = sqlx::query!(
            r#"INSERT OR REPLACE INTO refresh_tokens (id, user_id, nbf, exp, scope, is_mfa)
                VALUES ($1, $2, $3, $4, $5, $6)"#,
            self.id,
            self.user_id,
            self.nbf,
            self.exp,
            self.scope,
            self.is_mfa,
        );
        #[cfg(feature = "postgres")]
        let q = sqlx::query!(
            r#"INSERT INTO refresh_tokens (id, user_id, nbf, exp, scope, is_mfa)
                VALUES ($1, $2, $3, $4, $5, $6)
                ON CONFLICT(id) DO UPDATE SET user_id = $2, nbf = $3, exp = $4, scope = $5"#,
            self.id,
            self.user_id,
            self.nbf,
            self.exp,
            self.scope,
            self.is_mfa,
        );

        q.execute(&data.db).await?;

        Ok(())
    }
}

impl RefreshToken {
    pub async fn invalidate_all_for_user(
        data: &web::Data<AppState>,
        id: &str,
    ) -> Result<(), ErrorResponse> {
        sqlx::query!("DELETE FROM refresh_tokens WHERE user_id = $1", id)
            .execute(&data.db)
            .await?;

        Ok(())
    }
}
