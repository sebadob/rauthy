use crate::database::DB;
use chrono::Utc;
use hiqlite::{Param, params};
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserLoginState {
    // Unix Timestamp in Millis - used inside PK
    pub timestamp: i64,
    pub user_id: String,
    pub client_id: String,
    pub session_id: Option<String>,
}

impl From<tokio_postgres::Row> for UserLoginState {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            timestamp: row.get("timestamp"),
            user_id: row.get("user_id"),
            client_id: row.get("client_id"),
            session_id: row.get("session_id"),
        }
    }
}

/// CRUD
impl UserLoginState {
    pub async fn insert(
        user_id: String,
        client_id: String,
        session_id: Option<String>,
    ) -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp_millis();

        let sql_1 = r#"
INSERT INTO user_login_states(timestamp, user_id, client_id)
SELECT $1, $2, $3
WHERE NOT EXISTS (
    SELECT 1 FROM user_login_states
    WHERE user_id = $2 AND client_id = $3 AND session_id IS NULL
)"#;
        let sql_2 = r#"
INSERT INTO user_login_states(timestamp, user_id, client_id, session_id)
SELECT $1, $2, $3, $4
WHERE NOT EXISTS (
    SELECT 1 FROM user_login_states
    WHERE user_id = $2 AND client_id = $3 AND session_id = $4
)"#;

        if is_hiqlite() {
            let mut txn = Vec::with_capacity(session_id.as_ref().map(|_| 2).unwrap_or(1));
            txn.push((sql_1, params!(now - 1, &user_id, &client_id)));
            if session_id.is_some() {
                txn.push((sql_2, params!(now, user_id, client_id, session_id)));
            }

            DB::hql().txn(txn).await?;
        } else {
            let mut cl = DB::pg().await?;
            let txn = cl.transaction().await?;

            DB::pg_txn_append(&txn, sql_1, &[&(now - 1), &user_id, &client_id]).await?;
            DB::pg_txn_append(
                &txn,
                sql_2,
                &[&(now - 1), &user_id, &client_id, &session_id],
            )
            .await?;

            txn.commit().await?;
        }

        Ok(())
    }

    pub async fn find_all_without_session() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM user_login_states WHERE session_id IS NULL";
        let res = if is_hiqlite() {
            DB::hql().query_as(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 2).await?
        };

        Ok(res)
    }

    pub async fn find_by_client_without_session(
        client_id: String,
    ) -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM user_login_states WHERE client_id = $1 AND session_id IS NULL";
        let res = if is_hiqlite() {
            DB::hql().query_as(sql, params!(client_id)).await?
        } else {
            DB::pg_query(sql, &[&client_id], 1).await?
        };

        Ok(res)
    }

    pub async fn find_by_user(user_id: String) -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM user_login_states WHERE user_id = $1";
        let res = if is_hiqlite() {
            DB::hql().query_as(sql, params!(user_id)).await?
        } else {
            DB::pg_query(sql, &[&user_id], 2).await?
        };

        Ok(res)
    }

    pub async fn find_by_session(session_id: String) -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM user_login_states WHERE session_id = $1";
        let slf = if is_hiqlite() {
            DB::hql().query_as(sql, params!(session_id)).await?
        } else {
            DB::pg_query(sql, &[&session_id], 2).await?
        };

        Ok(slf)
    }

    pub async fn delete(self) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM user_login_states WHERE timestamp = $1 AND user_id = $2";
        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(self.timestamp, self.user_id))
                .await?;
        } else {
            DB::pg_execute(sql, &[&self.timestamp, &self.user_id]).await?;
        }

        Ok(())
    }

    pub async fn delete_all() -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM user_login_states";
        if is_hiqlite() {
            DB::hql().execute(sql, params!()).await?;
        } else {
            DB::pg_execute(sql, &[]).await?;
        }

        Ok(())
    }

    pub async fn delete_all_by_cid(cid: String) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM user_login_states WHERE client_id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(cid)).await?;
        } else {
            DB::pg_execute(sql, &[&cid]).await?;
        }

        Ok(())
    }

    pub async fn delete_all_by_uid(uid: String) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM user_login_states WHERE user_id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(uid)).await?;
        } else {
            DB::pg_execute(sql, &[&uid]).await?;
        }

        Ok(())
    }

    pub async fn delete_all_by_sid(sid: String) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM user_login_states WHERE session_id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(sid)).await?;
        } else {
            DB::pg_execute(sql, &[&sid]).await?;
        }

        Ok(())
    }
}
