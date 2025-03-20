use crate::database::DB;
use chrono::Utc;
use hiqlite::{Param, params};
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserLoginState {
    pub user_id: String,
    pub client_id: String,
    pub session_id: Option<String>,
    pub login_ts: i64,
}

/// CRUD
impl UserLoginState {
    pub async fn insert(
        user_id: String,
        client_id: String,
        session_id: Option<String>,
    ) -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp();

        if is_hiqlite() {
            DB::client()
                .execute(
                    r#"
INSERT INTO user_login_states(user_id, client_id, session_id, login_ts)
VALUES ($1, $2, $3, $4)
ON CONFLICT (user_id, client_id, session_id)
DO NOTHING"#,
                    params!(user_id, client_id, session_id, now),
                )
                .await?;
        } else {
            sqlx::query!(
                r#"
INSERT INTO user_login_states(user_id, client_id, session_id, login_ts)
VALUES ($1, $2, $3, $4)
ON CONFLICT (user_id, client_id, session_id)
DO NOTHING"#,
                user_id,
                client_id,
                session_id,
                now
            )
            .execute(DB::conn())
            .await?;
        }

        Ok(())
    }

    pub async fn find_for_user(user_id: String) -> Result<Vec<Self>, ErrorResponse> {
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

    pub async fn find_for_session(session_id: String) -> Result<Option<Self>, ErrorResponse> {
        let slf = if is_hiqlite() {
            DB::client()
                .query_as_optional(
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
            .fetch_optional(DB::conn())
            .await?
        };

        Ok(slf)
    }

    pub async fn delete(self) -> Result<(), ErrorResponse> {
        if is_hiqlite() {
            DB::client()
                .execute(
                    r#"
DELETE FROM user_login_states
WHERE user_id = $1 AND client_id = $2 AND session_id = $3"#,
                    params!(self.client_id, self.user_id, self.session_id),
                )
                .await?;
        } else {
            sqlx::query!(
                r#"
DELETE FROM user_login_states
WHERE user_id = $1 AND client_id = $2 AND session_id = $3"#,
                self.client_id,
                self.user_id,
                self.session_id
            )
            .execute(DB::conn())
            .await?;
        }

        Ok(())
    }
}
