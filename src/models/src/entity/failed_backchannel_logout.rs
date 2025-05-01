use crate::database::DB;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FailedBackchannelLogout {
    pub client_id: String,
    // both `sub` and `sid` may be empty but cannot be NULL because Postgres requires
    // NOT NULL if used in a primary key
    pub sub: String,
    pub sid: String,
    pub retry_count: i32,
}

impl From<tokio_postgres::Row> for FailedBackchannelLogout {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            client_id: row.get("client_id"),
            sub: row.get("sub"),
            sid: row.get("sid"),
            retry_count: row.get("retry_count"),
        }
    }
}

impl FailedBackchannelLogout {
    pub async fn upsert(
        client_id: String,
        sub: Option<String>,
        sid: Option<String>,
    ) -> Result<(), ErrorResponse> {
        let sub = sub.unwrap_or_default();
        let sid = sid.unwrap_or_default();

        let sql = r#"
INSERT INTO failed_backchannel_logouts (client_id, sub, sid, retry_count)
VALUES ($1, $2, $3, 0)
ON CONFLICT (client_id, sub, sid)
DO UPDATE SET retry_count = retry_count + 1"#;

        if is_hiqlite() {
            DB::hql().execute(sql, params!(client_id, sub, sid)).await?;
        } else {
            DB::pg_execute(sql, &[&client_id, &sub, &sid]).await?;
        }

        Ok(())
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM failed_backchannel_logouts";
        let res = if is_hiqlite() {
            DB::hql().query_as(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 0).await?
        };

        Ok(res)
    }

    pub async fn delete(self) -> Result<(), ErrorResponse> {
        let sql = r#"
DELETE FROM failed_backchannel_logouts
WHERE client_id = $1 AND sub = $2 AND sid = $3"#;

        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(self.client_id, self.sub, self.sid))
                .await?;
        } else {
            DB::pg_execute(sql, &[&self.client_id, &self.sub, &self.sid]).await?;
        }

        Ok(())
    }

    pub async fn delete_all_by_client(client_id: String) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM failed_backchannel_logouts WHERE client_id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(client_id)).await?;
        } else {
            DB::pg_execute(sql, &[&client_id]).await?;
        }

        Ok(())
    }
}
