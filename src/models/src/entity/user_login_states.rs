use crate::database::DB;
use chrono::Utc;
use hiqlite::{Param, params};
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use serde::Deserialize;

#[derive(Debug, Deserialize, sqlx::FromRow)]
pub struct UserLoginState {
    // Unix Timestamp in Millis - used inside PK
    pub timestamp: i64,
    pub user_id: String,
    pub client_id: String,
    pub session_id: Option<String>,
}

/// CRUD
impl UserLoginState {
    pub async fn insert(
        user_id: String,
        client_id: String,
        session_id: Option<String>,
    ) -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp_millis();

        if is_hiqlite() {
            let mut txn = Vec::with_capacity(session_id.as_ref().map(|_| 2).unwrap_or(1));
            txn.push((
                r#"
INSERT INTO user_login_states(timestamp, user_id, client_id)
SELECT $1, $2, $3
WHERE NOT EXISTS (
    SELECT 1 FROM user_login_states
    WHERE user_id = $2 AND client_id = $3 AND session_id IS NULL
)"#,
                params!(now - 1, user_id.clone(), client_id.clone()),
            ));
            if session_id.is_some() {
                txn.push((
                    r#"
INSERT INTO user_login_states(timestamp, user_id, client_id, session_id)
SELECT $1, $2, $3, $4
WHERE NOT EXISTS (
    SELECT 1 FROM user_login_states
    WHERE user_id = $2 AND client_id = $3 AND session_id = $4
)"#,
                    params!(now, user_id, client_id, session_id),
                ));
            }

            DB::client().txn(txn).await?;
        } else {
            let mut txn = DB::txn().await?;

            // TODO for some reason, the query! returns an error with the prepared statement type
            // for user_id, even though the query works just fine...
            //             sqlx::query!(
            //                 r#"
            // INSERT INTO user_login_states(timestamp, user_id, client_id)
            // SELECT $1, $2, $3
            // WHERE NOT EXISTS (
            //     SELECT 1 FROM user_login_states
            //     WHERE user_id = $2 AND client_id = $3 AND session_id IS NULL
            // )"#,
            //                 now - 1,
            //                 user_id,
            //                 client_id,
            //             )
            //             .execute(&mut *txn)
            //             .await?;

            sqlx::query(
                r#"
INSERT INTO user_login_states(timestamp, user_id, client_id)
SELECT $1, $2, $3
WHERE NOT EXISTS (
    SELECT 1 FROM user_login_states
    WHERE user_id = $2 AND client_id = $3 AND session_id IS NULL
)"#,
            )
            .bind(now - 1)
            .bind(user_id.clone())
            .bind(client_id.clone())
            .execute(&mut *txn)
            .await?;

            if session_id.is_some() {
                sqlx::query(
                    r#"
INSERT INTO user_login_states(timestamp, user_id, client_id, session_id)
SELECT $1, $2, $3, $4
WHERE NOT EXISTS (
    SELECT 1 FROM user_login_states
    WHERE user_id = $2 AND client_id = $3 AND session_id = $4
)"#,
                )
                .bind(now)
                .bind(user_id)
                .bind(client_id)
                .bind(session_id)
                .execute(&mut *txn)
                .await?;
            }

            txn.commit().await?;
        }

        Ok(())
    }

    pub async fn find_all_without_session() -> Result<Vec<Self>, ErrorResponse> {
        let res = if is_hiqlite() {
            DB::client()
                .query_as(
                    "SELECT * FROM user_login_states WHERE session_id IS NULL",
                    params!(),
                )
                .await?
        } else {
            sqlx::query_as!(
                Self,
                "SELECT * FROM user_login_states WHERE session_id IS NULL",
            )
            .fetch_all(DB::conn())
            .await?
        };

        Ok(res)
    }

    pub async fn find_by_client_without_session(
        client_id: String,
    ) -> Result<Vec<Self>, ErrorResponse> {
        let res = if is_hiqlite() {
            DB::client()
                .query_as(
                    "SELECT * FROM user_login_states WHERE client_id = $1 AND session_id IS NULL",
                    params!(client_id),
                )
                .await?
        } else {
            sqlx::query_as!(
                Self,
                "SELECT * FROM user_login_states WHERE client_id = $1 AND session_id IS NULL",
                client_id
            )
            .fetch_all(DB::conn())
            .await?
        };

        Ok(res)
    }

    pub async fn find_by_user(user_id: String) -> Result<Vec<Self>, ErrorResponse> {
        let res = if is_hiqlite() {
            DB::client()
                .query_as(
                    "SELECT * FROM user_login_states WHERE user_id = $1",
                    params!(user_id),
                )
                .await?
        } else {
            sqlx::query_as!(
                Self,
                "SELECT * FROM user_login_states WHERE user_id = $1",
                user_id
            )
            .fetch_all(DB::conn())
            .await?
        };

        Ok(res)
    }

    pub async fn find_by_session(session_id: String) -> Result<Vec<Self>, ErrorResponse> {
        let slf = if is_hiqlite() {
            DB::client()
                .query_as(
                    "SELECT * FROM user_login_states WHERE session_id = $1",
                    params!(session_id),
                )
                .await?
        } else {
            sqlx::query_as!(
                Self,
                "SELECT * FROM user_login_states WHERE session_id = $1",
                session_id
            )
            .fetch_all(DB::conn())
            .await?
        };

        Ok(slf)
    }

    pub async fn delete(self) -> Result<(), ErrorResponse> {
        if is_hiqlite() {
            DB::client()
                .execute(
                    "DELETE FROM user_login_states WHERE timestamp = $1 AND user_id = $2",
                    params!(self.timestamp, self.user_id),
                )
                .await?;
        } else {
            sqlx::query!(
                "DELETE FROM user_login_states WHERE timestamp = $1 AND user_id = $2",
                self.timestamp,
                self.user_id,
            )
            .execute(DB::conn())
            .await?;
        }

        Ok(())
    }

    pub async fn delete_all() -> Result<(), ErrorResponse> {
        if is_hiqlite() {
            DB::client()
                .execute("DELETE FROM user_login_states", params!())
                .await?;
        } else {
            sqlx::query!("DELETE FROM user_login_states")
                .execute(DB::conn())
                .await?;
        }

        Ok(())
    }

    pub async fn delete_all_by_cid(cid: String) -> Result<(), ErrorResponse> {
        if is_hiqlite() {
            DB::client()
                .execute(
                    "DELETE FROM user_login_states WHERE client_id = $1",
                    params!(cid),
                )
                .await?;
        } else {
            sqlx::query!("DELETE FROM user_login_states WHERE client_id = $1", cid)
                .execute(DB::conn())
                .await?;
        }

        Ok(())
    }

    pub async fn delete_all_by_uid(uid: String) -> Result<(), ErrorResponse> {
        if is_hiqlite() {
            DB::client()
                .execute(
                    "DELETE FROM user_login_states WHERE user_id = $1",
                    params!(uid),
                )
                .await?;
        } else {
            sqlx::query!("DELETE FROM user_login_states WHERE user_id = $1", uid)
                .execute(DB::conn())
                .await?;
        }

        Ok(())
    }

    pub async fn delete_all_by_sid(sid: String) -> Result<(), ErrorResponse> {
        if is_hiqlite() {
            DB::client()
                .execute(
                    "DELETE FROM user_login_states WHERE session_id = $1",
                    params!(sid),
                )
                .await?;
        } else {
            sqlx::query!("DELETE FROM user_login_states WHERE session_id = $1", sid)
                .execute(DB::conn())
                .await?;
        }

        Ok(())
    }
}
