use crate::database::DB;
use chrono::Utc;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct ToSUserAccept {
    pub user_id: String,
    pub tos_ts: i64,
    pub accept_ts: i64,
    pub location: String,
}

impl From<tokio_postgres::Row> for ToSUserAccept {
    fn from(row: Row) -> Self {
        Self {
            user_id: row.get("user_id"),
            tos_ts: row.get("tos_ts"),
            accept_ts: row.get("accept_ts"),
            location: row.get("location"),
        }
    }
}

impl ToSUserAccept {
    pub async fn create(
        user_id: String,
        tos_ts: i64,
        location: String,
    ) -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp();

        let sql = r#"
INSERT INTO tos_user_accept (user_id, tos_ts, accept_ts, location)
VALUES ($1, $2, $3, $4)"#;
        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(user_id, tos_ts, now, location))
                .await?;
        } else {
            DB::pg_execute(sql, &[&user_id, &tos_ts, &now, &location]).await?;
        }

        Ok(())
    }

    pub async fn find(user_id: String) -> Result<Option<Self>, ErrorResponse> {
        let sql = "SELECT * FROM tos_user_accept WHERE user_id = $1";

        let slf = if is_hiqlite() {
            DB::hql().query_as_optional(sql, params!(user_id)).await?
        } else {
            DB::pg_query_opt(sql, &[&user_id]).await?
        };

        Ok(slf)
    }
}
