use crate::database::DB;
use chrono::Utc;
use hiqlite::macros::FromRow;
use hiqlite::params;
use rauthy_common::is_hiqlite;
use rauthy_derive::FromPgRow;
use rauthy_error::ErrorResponse;

#[derive(Debug, FromRow, FromPgRow)]
pub struct PasswordExpMail {
    pub user_id: String,
    pub mail_sent_ts: i64,
}

impl PasswordExpMail {
    pub async fn upsert_now(user_id: String) -> Result<(), ErrorResponse> {
        let sql = r#"
INSERT INTO pwd_exp_mails (user_id, mail_sent_ts)
VALUES ($1, $2)
ON CONFLICT (user_id) DO UPDATE SET mail_sent_ts = $2
"#;

        let now = Utc::now().timestamp();
        if is_hiqlite() {
            DB::hql().execute(sql, params!(user_id, now)).await?;
        } else {
            DB::pg_execute(sql, &[&user_id, &now]).await?;
        }

        Ok(())
    }

    pub async fn cleanup(older_than: chrono::DateTime<Utc>) -> Result<(), ErrorResponse> {
        let ts = older_than.timestamp();

        let sql = "DELETE FROM pwd_exp_mails WHERE mail_sent_ts < $1";

        if is_hiqlite() {
            DB::hql().execute(sql, params!(ts)).await?;
        } else {
            DB::pg_execute(sql, &[&ts]).await?;
        }

        Ok(())
    }

    pub async fn delete(user_id: String) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM pwd_exp_mails WHERE user_id = $1";

        if is_hiqlite() {
            DB::hql().execute(sql, params!(user_id)).await?;
        } else {
            DB::pg_execute(sql, &[&user_id]).await?;
        }

        Ok(())
    }

    // TODO probably use a combined join with users in a single query
    pub async fn find_since(
        younger_than: chrono::DateTime<Utc>,
    ) -> Result<Vec<Self>, ErrorResponse> {
        let ts = younger_than.timestamp();

        let sql = "SELECT * FROM pwd_exp_mails WHERE mail_sent_ts > $1";

        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!(ts)).await?
        } else {
            DB::pg_query(sql, &[&ts], 0).await?
        };

        Ok(res)
    }
}
