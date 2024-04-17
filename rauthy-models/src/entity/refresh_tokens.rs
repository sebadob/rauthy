use crate::app_state::AppState;
use actix_web::web;
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use tracing::error;

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
        nbf: OffsetDateTime,
        exp: OffsetDateTime,
        scope: Option<String>,
        is_mfa: bool,
    ) -> Result<Self, ErrorResponse> {
        let rt = Self {
            id,
            user_id,
            nbf: nbf.unix_timestamp(),
            exp: exp.unix_timestamp(),
            scope,
            is_mfa,
        };

        rt.save(data).await?;
        Ok(rt)
    }

    pub async fn delete(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        sqlx::query!("delete from refresh_tokens where id = $1", self.id)
            .execute(&data.db)
            .await?;
        Ok(())
    }

    pub async fn find_all(data: &web::Data<AppState>) -> Result<Vec<Self>, ErrorResponse> {
        let res = sqlx::query_as!(Self, "select * from refresh_tokens")
            .fetch_all(&data.db)
            .await?;
        Ok(res)
    }

    pub async fn invalidate_all(data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let tokens = Self::find_all(data).await?;

        for mut t in tokens {
            if t.exp > now {
                t.exp = now;
            }
            if let Err(err) = t.save(data).await {
                error!("Error invalidating refresh token: {}", err);
            }
        }

        Ok(())
    }

    pub async fn invalidate_for_user(
        data: &web::Data<AppState>,
        uid: &str,
    ) -> Result<(), ErrorResponse> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let tokens = Self::find_all(data).await?;

        for mut t in tokens {
            if t.user_id == uid && t.exp > now {
                t.exp = now;
            }
            t.save(data).await?;
        }

        Ok(())
    }

    pub async fn find(data: &web::Data<AppState>, id: &str) -> Result<Self, ErrorResponse> {
        match sqlx::query_as!(Self, "select * from refresh_tokens where id = $1", id)
            .fetch_one(&data.db)
            .await
        {
            Ok(res) => Ok(res),
            Err(_) => Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "Refresh Token does not exist".to_string(),
            )),
        }
    }

    pub async fn save(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        #[cfg(not(feature = "postgres"))]
        let q = sqlx::query!(
            r#"insert or replace into refresh_tokens (id, user_id, nbf, exp, scope, is_mfa)
                values ($1, $2, $3, $4, $5, $6)"#,
            self.id,
            self.user_id,
            self.nbf,
            self.exp,
            self.scope,
            self.is_mfa,
        );
        #[cfg(feature = "postgres")]
        let q = sqlx::query!(
            r#"insert into refresh_tokens (id, user_id, nbf, exp, scope, is_mfa)
                values ($1, $2, $3, $4, $5, $6)
                on conflict(id) do update set user_id = $2, nbf = $3, exp = $4, scope = $5"#,
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
        sqlx::query!("delete from refresh_tokens where user_id = $1", id)
            .execute(&data.db)
            .await?;

        Ok(())
    }
}
