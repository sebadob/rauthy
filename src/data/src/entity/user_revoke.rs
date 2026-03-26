use crate::database::DB;
use cryptr::utils::secure_random_alnum;
use hiqlite::macros::{FromRow, params};
use rauthy_common::is_hiqlite;
use rauthy_derive::FromPgRow;
use rauthy_error::ErrorResponse;
use tracing::debug;

#[derive(Debug, FromRow, FromPgRow)]
pub struct UserRevoke {
    pub user_id: String,
    pub code: String,
}

impl UserRevoke {
    #[inline]
    pub async fn find(user_id: String) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM user_revoke WHERE user_id = $1";
        let slf = if is_hiqlite() {
            DB::hql().query_map_one(sql, params!(user_id)).await?
        } else {
            DB::pg_query_one(sql, &[&user_id]).await?
        };
        Ok(slf)
    }

    #[inline]
    pub async fn find_or_upsert(user_id: String) -> Result<Self, ErrorResponse> {
        match Self::find(user_id.clone()).await {
            Ok(slf) => Ok(slf),
            Err(err) => {
                debug!("UserRevoke::find(): {err:?}");
                Self::upsert(user_id).await
            }
        }
    }

    pub async fn delete(user_id: String) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM user_revoke WHERE user_id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(user_id)).await?;
        } else {
            DB::pg_execute(sql, &[&user_id]).await?;
        }

        Ok(())
    }

    #[inline]
    pub async fn upsert(user_id: String) -> Result<Self, ErrorResponse> {
        let code = secure_random_alnum(48);
        let sql = r#"
INSERT INTO user_revoke (user_id, code)
VALUES ($1, $2)
ON CONFLICT (user_id) DO NOTHING
RETURNING *"#;

        let slf = if is_hiqlite() {
            let mut row = DB::hql()
                .execute_returning_one(sql, params!(user_id, code))
                .await?;
            Self::from(&mut row)
        } else {
            DB::pg_query_one(sql, &[&user_id, &code]).await?
        };

        Ok(slf)
    }
}
