use crate::database::DB;
use chrono::Utc;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::ops::Add;
use tracing::debug;

pub struct RevokedToken {
    pub jti: String,
    pub exp: i64,
}

impl From<hiqlite::Row<'_>> for RevokedToken {
    fn from(mut row: hiqlite::Row<'_>) -> Self {
        Self {
            jti: row.get("jti"),
            exp: row.get("exp"),
        }
    }
}

impl From<tokio_postgres::Row> for RevokedToken {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            jti: row.get("jti"),
            exp: row.get("exp"),
        }
    }
}

impl RevokedToken {
    pub async fn cleanup_expired() -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM revoked_tokens WHERE exp < $1";
        let now_plus_1 = Utc::now().add(chrono::Duration::minutes(1)).timestamp();

        let rows_affected = if is_hiqlite() {
            DB::hql().execute(sql, params!(now_plus_1)).await?
        } else {
            DB::pg_execute(sql, &[&now_plus_1]).await?
        };

        if rows_affected > 0 {
            debug!("Cleaned up {rows_affected} expired revoked tokens");
        }

        Ok(())
    }

    pub async fn upsert(self) -> Result<(), ErrorResponse> {
        let sql = r#"
INSERT INTO revoked_tokens (jti, exp)
VALUES ($1, $2)
ON CONFLICT (jti) DO UPDATE
SET exp = $2
"#;

        if is_hiqlite() {
            DB::hql().execute(sql, params!(self.jti, self.exp)).await?;
        } else {
            DB::pg_execute(sql, &[&self.jti, &self.exp]).await?;
        }

        Ok(())
    }

    #[inline]
    pub async fn validate_not_revoked(user_id: &str, jti: &str) -> Result<(), ErrorResponse> {
        let sql = "SELECT 1 FROM revoked_tokens WHERE user_id = $1 AND jti = $2";

        let opt: Option<Self> = if is_hiqlite() {
            DB::hql()
                .query_map_optional(sql, params!(user_id, jti))
                .await?
        } else {
            DB::pg_query_opt(sql, &[&user_id, &jti]).await?
        };

        if opt.is_none() {
            Ok(())
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "token was revoked",
            ))
        }
    }
}
