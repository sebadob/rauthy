use crate::database::DB;
use chrono::Utc;
use cryptr::utils::secure_random_alnum;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::ops::Add;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct IssuedToken {
    pub jti: String,
    pub user_id: Option<String>,
    pub sid: Option<String>,
    pub exp: i64,
    pub revoked: Option<bool>,
}

impl From<hiqlite::Row<'_>> for IssuedToken {
    fn from(mut row: hiqlite::Row<'_>) -> Self {
        Self {
            jti: row.get("jti"),
            user_id: row.get("user_id"),
            sid: row.get("sid"),
            exp: row.get("exp"),
            revoked: row.get("revoked"),
        }
    }
}

impl From<tokio_postgres::Row> for IssuedToken {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            jti: row.get("jti"),
            user_id: row.get("user_id"),
            sid: row.get("sid"),
            exp: row.get("exp"),
            revoked: row.get("revoked"),
        }
    }
}

impl IssuedToken {
    pub async fn cleanup_expired() -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM issued_tokens WHERE exp < $1";
        let now_plus_1 = Utc::now().add(chrono::Duration::minutes(1)).timestamp();

        let rows_affected = if is_hiqlite() {
            DB::hql().execute(sql, params!(now_plus_1)).await?
        } else {
            DB::pg_execute(sql, &[&now_plus_1]).await?
        };

        if rows_affected > 0 {
            debug!("Cleaned up {rows_affected} expired issued tokens");
        }

        Ok(())
    }

    pub async fn create(
        user_id: Option<&str>,
        sid: Option<&str>,
        exp: i64,
    ) -> Result<Self, ErrorResponse> {
        // Even though the chance is tiny, we want to handle PK
        // conflicts here to keep the `jti` somewhat small.
        loop {
            let slf = Self {
                jti: secure_random_alnum(12),
                user_id: user_id.map(String::from),
                sid: sid.map(String::from),
                exp,
                revoked: None,
            };
            if slf.insert().await.is_ok() {
                return Ok(slf);
            } else {
                debug!("Conflicting `jti` generated");
            }
        }
    }

    #[inline]
    async fn insert(&self) -> Result<(), ErrorResponse> {
        let sql = r#"
INSERT INTO issued_tokens (jti, user_id, sid, exp)
VALUES ($1, $2, $3, $4)
"#;
        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(&self.jti, &self.user_id, &self.sid, self.exp))
                .await?;
        } else {
            DB::pg_execute(sql, &[&self.jti, &self.user_id, &self.sid, &self.exp]).await?;
        }

        Ok(())
    }

    pub async fn revoke(jti: String) -> Result<(), ErrorResponse> {
        let sql = "UPDATE issued_tokens SET revoked = $1 WHERE jti = $2";

        if is_hiqlite() {
            DB::hql().execute(sql, params!(true, jti)).await?;
        } else {
            DB::pg_execute(sql, &[&true, &jti]).await?;
        }

        Ok(())
    }

    pub async fn revoke_all() -> Result<(), ErrorResponse> {
        let sql = "UPDATE issued_tokens SET revoked = $1";

        if is_hiqlite() {
            DB::hql().execute(sql, params!(true)).await?;
        } else {
            DB::pg_execute(sql, &[&true]).await?;
        }

        Ok(())
    }

    pub async fn revoke_for_user(user_id: &str) -> Result<(), ErrorResponse> {
        let sql = "UPDATE issued_tokens SET revoked = $1 WHERE user_id = $2";

        if is_hiqlite() {
            DB::hql().execute(sql, params!(true, user_id)).await?;
        } else {
            DB::pg_execute(sql, &[&true, &user_id]).await?;
        }

        Ok(())
    }

    pub async fn revoke_for_session(sid: &str) -> Result<(), ErrorResponse> {
        let sql = "UPDATE issued_tokens SET revoked = $1 WHERE sid = $2";

        if is_hiqlite() {
            DB::hql().execute(sql, params!(true, sid)).await?;
        } else {
            DB::pg_execute(sql, &[&true, &sid]).await?;
        }

        Ok(())
    }

    #[inline]
    pub async fn validate_not_revoked(jti: &str) -> Result<(), ErrorResponse> {
        let sql = "SELECT * FROM issued_tokens WHERE jti = $1";

        let opt: Option<Self> = if is_hiqlite() {
            DB::hql().query_map_optional(sql, params!(jti)).await?
        } else {
            DB::pg_query_opt(sql, &[&jti]).await?
        };

        if let Some(slf) = opt {
            if slf.revoked != Some(true) {
                Ok(())
            } else {
                Err(ErrorResponse::new(
                    ErrorResponseType::Unauthorized,
                    "token was revoked",
                ))
            }
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "token does not exist",
            ))
        }
    }
}
