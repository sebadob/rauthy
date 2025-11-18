use crate::database::{Cache, DB};
use chrono::Utc;
use hiqlite_macros::params;
use rauthy_api_types::tos::ToSUserAcceptResponse;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
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
        ip: IpAddr,
        location: Option<String>,
    ) -> Result<(), ErrorResponse> {
        let accept_ts = Utc::now().timestamp();
        let location = format!("{} {}", ip, location.unwrap_or_default());

        let sql = r#"
INSERT INTO tos_user_accept (user_id, tos_ts, accept_ts, location)
VALUES ($1, $2, $3, $4)"#;
        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(&user_id, tos_ts, accept_ts, &location))
                .await?;
        } else {
            DB::pg_execute(sql, &[&user_id, &tos_ts, &accept_ts, &location]).await?;
        }

        DB::hql()
            .put(
                Cache::ToS,
                Self::cache_idx(&user_id),
                &Some(Self {
                    user_id,
                    tos_ts,
                    accept_ts,
                    location,
                }),
                Some(600),
            )
            .await?;

        Ok(())
    }

    pub async fn find_all(user_id: String) -> Result<Vec<Self>, ErrorResponse> {
        let sql = r#"
SELECT * FROM tos_user_accept
WHERE user_id = $1"#;

        let res = if is_hiqlite() {
            DB::hql().query_as(sql, params!(user_id)).await?
        } else {
            DB::pg_query(sql, &[&user_id], 0).await?
        };

        Ok(res)
    }

    pub async fn find_latest(user_id: String) -> Result<Option<Self>, ErrorResponse> {
        if let Some(slf) = DB::hql().get(Cache::ToS, Self::cache_idx(&user_id)).await? {
            return Ok(slf);
        }

        let cache_idx = Self::cache_idx(&user_id);
        let sql = r#"
SELECT * FROM tos_user_accept
WHERE user_id = $1
ORDER BY tos_ts DESC
LIMIT 1
"#;

        let slf = if is_hiqlite() {
            DB::hql().query_as_optional(sql, params!(user_id)).await?
        } else {
            DB::pg_query_opt(sql, &[&user_id]).await?
        };

        DB::hql()
            .put(Cache::ToS, cache_idx, &slf, Some(600))
            .await?;

        Ok(slf)
    }
}

impl ToSUserAccept {
    #[inline]
    fn cache_idx(user_id: &str) -> String {
        format!("uid_{user_id}")
    }
}

impl From<ToSUserAccept> for ToSUserAcceptResponse {
    fn from(value: ToSUserAccept) -> Self {
        Self {
            user_id: value.user_id,
            tos_ts: value.tos_ts,
            accept_ts: value.accept_ts,
            location: value.location,
        }
    }
}
