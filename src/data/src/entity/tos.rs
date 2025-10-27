use crate::database::{Cache, DB};
use chrono::Utc;
use hiqlite_macros::params;
use rauthy_api_types::tos::{ToSLatestResponse, ToSResponse};
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};

static CACHE_IDX: &str = "tos_latest";

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
        DB::hql()
            .put(Cache::ToS, CACHE_IDX, &Some(&slf), None)
            .await?;

        Ok(())
    }

    pub async fn find(ts: i64) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM tos WHERE ts = $1";
        let res = if is_hiqlite() {
            DB::hql().query_as_one(sql, params!(ts)).await?
        } else {
            DB::pg_query_one(sql, &[&ts]).await?
        };
        Ok(res)
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
        if let Some(slf) = DB::hql().get(Cache::ToS, CACHE_IDX).await? {
            return Ok(slf);
        }

        let sql = "SELECT * FROM tos ORDER BY ts DESC LIMIT 1";
        let slf: Option<Self> = if is_hiqlite() {
            DB::hql().query_as_optional(sql, params!()).await?
        } else {
            DB::pg_query_opt(sql, &[]).await?
        };

        DB::hql().put(Cache::ToS, CACHE_IDX, &slf, None).await?;

        Ok(slf)
    }
}

impl From<ToS> for ToSResponse {
    fn from(tos: ToS) -> Self {
        Self {
            ts: tos.ts,
            author: tos.author,
            is_html: tos.is_html,
            content: tos.content,
        }
    }
}

impl From<ToS> for ToSLatestResponse {
    fn from(tos: ToS) -> Self {
        Self {
            ts: tos.ts,
            is_html: tos.is_html,
            content: tos.content,
        }
    }
}
