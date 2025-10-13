use crate::database::{Cache, DB};
use chrono::Utc;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ToS {
    pub ts: i64,
    pub author: String,
    pub is_html: bool,
    pub content: String,
}

impl From<tokio_postgres::Row> for ToS {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            ts: row.get("ts"),
            author: row.get("author"),
            is_html: row.get("is_html"),
            content: row.get("content"),
        }
    }
}

impl ToS {
    pub async fn create(
        author: String,
        is_html: bool,
        content: String,
    ) -> Result<(), ErrorResponse> {
        let ts = Utc::now().timestamp();

        let sql = r#"
INSERT INTO tos (ts, author, is_html, content)
VALUES ($1, $2, $3, $4)"#;

        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(ts, &author, is_html, &content))
                .await?;
        } else {
            DB::pg_execute(sql, &[&ts, &author, &is_html, &content]).await?;
        }

        let slf = Self {
            ts,
            author,
            is_html,
            content,
        };
        DB::hql().put(Cache::App, "tos", &slf, None).await?;

        Ok(())
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM tos";
        let res: Vec<Self> = if is_hiqlite() {
            DB::hql().query_as(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 0).await?
        };

        Ok(res)
    }

    pub async fn find_latest() -> Result<Option<Self>, ErrorResponse> {
        if let Some(slf) = DB::hql().get(Cache::App, "tos").await? {
            return Ok(slf);
        }

        let sql = "SELECT * FROM tos LIMIT 1 ORDER BY ts";
        let slf: Option<Self> = if is_hiqlite() {
            DB::hql().query_as_optional(sql, params!()).await?
        } else {
            DB::pg_query_opt(sql, &[]).await?
        };

        DB::hql().put(Cache::App, "tos", &slf, None).await?;

        Ok(slf)
    }
}
