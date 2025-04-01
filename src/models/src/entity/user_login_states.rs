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

/// CRUD
impl UserLoginState {
    pub async fn insert(
        user_id: String,
        client_id: String,
        session_id: Option<String>,
    ) -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp_millis();

        if is_hiqlite() {
            DB::client()
                .txn([
                    (
                        r#"
INSERT INTO user_login_states(timestamp, user_id, client_id)
VALUES ($1, $2, $3)
ON CONFLICT (user_id, client_id, session_id)
DO NOTHING"#,
                        params!(now - 1, user_id.clone(), client_id.clone()),
                    ),
                    (
                        r#"
INSERT INTO user_login_states(timestamp, user_id, client_id, session_id)
VALUES ($1, $2, $3, $4)
ON CONFLICT (user_id, client_id, session_id)
DO NOTHING"#,
                        params!(now, user_id, client_id, session_id),
                    ),
                ])
                .await?;
        } else {
            let mut txn = DB::txn().await?;

            sqlx::query!(
                r#"
INSERT INTO user_login_states(timestamp, user_id, client_id)
VALUES ($1, $2, $3)
ON CONFLICT (user_id, client_id, session_id)
DO NOTHING"#,
                now - 1,
                user_id,
                client_id,
            )
            .execute(&mut *txn)
            .await?;

            sqlx::query!(
                r#"
INSERT INTO user_login_states(timestamp, user_id, client_id, session_id)
VALUES ($1, $2, $3, $4)
ON CONFLICT (user_id, client_id, session_id)
DO NOTHING"#,
                now,
                user_id,
                client_id,
                session_id,
            )
            .execute(&mut *txn)
            .await?;

            txn.commit().await?;
        }

        Ok(())
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
