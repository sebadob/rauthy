use crate::database::DB;
use hiqlite::macros::params;
use rauthy_common::is_hiqlite;
use rauthy_common::utils::{deserialize, serialize};
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};

/// Records when the yearly sponsor reminder was last sent.
#[derive(Debug, Serialize, Deserialize)]
pub struct SponsorReminder {
    pub timestamp: i64,
}

impl SponsorReminder {
    /// Returns the unix timestamp of the last sent sponsor reminder (if any)
    pub async fn find() -> Option<i64> {
        let sql = "SELECT data FROM config WHERE id = 'sponsor_reminder'";
        let res = if is_hiqlite() {
            DB::hql().query_as(sql, params!()).await.ok()
        } else {
            DB::pg_query_one_row(sql, &[])
                .await
                .ok()
                .map(|r| r.get::<_, Vec<u8>>("data"))
        };

        res.and_then(|data| deserialize::<SponsorReminder>(&data).ok())
            .map(|slf| slf.timestamp)
    }

    pub async fn upsert(timestamp: i64) -> Result<(), ErrorResponse> {
        let data = serialize(&Self { timestamp })?;

        let sql = r#"
INSERT INTO config (id, data)
VALUES ('sponsor_reminder', $1)
ON CONFLICT(id)
DO UPDATE SET data = $1"#;

        if is_hiqlite() {
            DB::hql().execute(sql, params!(data)).await?;
        } else {
            DB::pg_execute(sql, &[&data]).await?;
        }

        Ok(())
    }
}
