use crate::database::DB;
use crate::entity::issued_tokens::IssuedToken;
use chrono::Utc;
use hiqlite::macros::{FromRow, params};
use rauthy_common::is_hiqlite;
use rauthy_derive::FromPgRow;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::Deserialize;
use std::fmt::{Debug, Formatter};

#[derive(Deserialize, FromRow, FromPgRow)]
pub struct RefreshToken {
    pub id: String,
    pub user_id: String,
    pub nbf: i64,
    pub exp: i64,
    pub scope: Option<String>,
    pub is_mfa: bool,
    pub session_id: Option<String>,
    pub access_token_jti: Option<String>,
    pub client_id: Option<String>,
    /// `Client::generation` of `client_id` when the token was issued.
    pub client_generation: Option<String>,
}

impl Debug for RefreshToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RefreshToken {{ id: {}(...), user_id: {}, nbf: {}, exp: {}, scope: {:?}, is_mfa: {}, \
            session_id: {:?}, client_id: {:?}, client_generation: {:?} }}",
            &self.id[..5],
            self.user_id,
            self.nbf,
            self.exp,
            self.scope,
            self.is_mfa,
            self.session_id.as_ref().map(|sid| &sid[..5]),
            self.client_id,
            self.client_generation,
        )
    }
}

impl RefreshToken {
    /// Legacy tokens without `client_id` match any generation, all others only the one of the
    /// client they were issued for.
    pub fn is_for_client_generation(&self, generation: &str) -> bool {
        self.client_id.is_none() || self.client_generation.as_deref() == Some(generation)
    }
}

// CRUD
impl RefreshToken {
    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        id: String,
        user_id: String,
        nbf: i64,
        exp: i64,
        scope: Option<String>,
        // TODO should we even save mfa for refresh tokens?
        //  even if the original token has been issued with mfa, the refresh
        //  token not really is, because it can be given without user interaction.
        is_mfa: bool,
        session_id: Option<String>,
        access_token_jti: Option<String>,
        client_id: String,
        client_generation: String,
    ) -> Result<Self, ErrorResponse> {
        let rt = Self {
            id,
            user_id,
            nbf,
            exp,
            scope,
            is_mfa,
            session_id,
            access_token_jti,
            client_id: Some(client_id),
            client_generation: Some(client_generation),
        };

        rt.save().await?;
        Ok(rt)
    }

    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM refresh_tokens WHERE id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(self.id.clone())).await?;
        } else {
            DB::pg_execute(sql, &[&self.id]).await?;
        }
        Ok(())
    }

    /// Return an error when no rows have been affected.
    pub async fn delete_checked(&self) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM refresh_tokens WHERE id = $1";
        let rows_affected = if is_hiqlite() {
            DB::hql().execute(sql, params!(self.id.clone())).await?
        } else {
            DB::pg_execute(sql, &[&self.id]).await?
        };

        if rows_affected > 0 {
            Ok(())
        } else {
            // This check is important to prevent concurrent token usage
            Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "Invalid Refresh Token",
            ))
        }
    }

    pub async fn finish_rotation(&self, replacement_id: &str) -> Result<(), ErrorResponse> {
        let err = match self.delete_checked().await {
            Ok(()) => return Ok(()),
            Err(err) if matches!(err.error, ErrorResponseType::NotFound) => err,
            Err(err) => return Err(err),
        };

        // Logout or another refresh consumed the source. Remove this rotation's replacement,
        // which has a different ID and may still exist even though the source does not.
        match Self::find_delete(replacement_id).await {
            Ok(replacement) => {
                if let Some(jti) = replacement.access_token_jti {
                    IssuedToken::revoke(jti).await?;
                }
            }
            Err(cleanup) if matches!(cleanup.error, ErrorResponseType::NotFound) => {}
            Err(cleanup) => return Err(cleanup),
        }
        Err(err)
    }

    pub async fn delete_by_sid(session_id: String) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM refresh_tokens WHERE session_id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(session_id)).await?;
        } else {
            DB::pg_execute(sql, &[&session_id]).await?;
        }
        Ok(())
    }

    /// Deletes all refresh tokens for `user_id` + `client_id` from both `refresh_tokens` and
    /// `refresh_tokens_devices` and returns the total number of deleted rows.
    /// Legacy rows with `client_id` NULL are deliberately not matched.
    pub async fn delete_by_user_client(
        user_id: &str,
        client_id: &str,
    ) -> Result<usize, ErrorResponse> {
        let sql_1 = "DELETE FROM refresh_tokens WHERE user_id = $1 AND client_id = $2";
        let sql_2 = r#"
DELETE FROM refresh_tokens_devices
WHERE user_id = $1
  AND device_id IN (SELECT id FROM devices WHERE client_id = $2)"#;

        let mut deleted = 0;
        if is_hiqlite() {
            for res in DB::hql()
                .txn([
                    (sql_1, params!(user_id, client_id)),
                    (sql_2, params!(user_id, client_id)),
                ])
                .await?
            {
                deleted += res?;
            }
        } else {
            let mut cl = DB::pg().await?;
            let txn = cl.transaction().await?;
            deleted += DB::pg_txn_append(&txn, sql_1, &[&user_id, &client_id]).await? as usize;
            deleted += DB::pg_txn_append(&txn, sql_2, &[&user_id, &client_id]).await? as usize;
            txn.commit().await?;
        }
        Ok(deleted)
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM refresh_tokens";
        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 0).await?
        };
        Ok(res)
    }

    pub async fn find_by_user_id_jti(
        user_id: &str,
        access_token_jti: &str,
    ) -> Result<Option<Self>, ErrorResponse> {
        let sql = "SELECT * FROM refresh_tokens WHERE user_id = $1 AND access_token_jti = $2";

        let slf = if is_hiqlite() {
            DB::hql()
                .query_map_optional(sql, params!(user_id, access_token_jti))
                .await?
        } else {
            DB::pg_query_opt(sql, &[&user_id, &access_token_jti]).await?
        };

        Ok(slf)
    }

    pub async fn invalidate_all() -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp();
        let sql = "DELETE FROM refresh_tokens";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(now)).await?;
        } else {
            DB::pg_execute(sql, &[&now]).await?;
        }

        Ok(())
    }

    pub async fn invalidate_for_user(user_id: &str) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM refresh_tokens WHERE user_id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(user_id)).await?;
        } else {
            DB::pg_execute(sql, &[&user_id]).await?;
        }
        Ok(())
    }

    pub async fn find_delete(id: &str) -> Result<Self, ErrorResponse> {
        let sql = "DELETE FROM refresh_tokens WHERE id = $1 RETURNING *";

        let slf = if is_hiqlite() {
            DB::hql()
                .execute_returning_map_one(sql, params!(id))
                .await?
        } else {
            DB::pg_query_one(sql, &[&id]).await?
        };

        Ok(slf)
    }

    pub async fn find_opt(id: &str) -> Result<Option<Self>, ErrorResponse> {
        let sql = "SELECT * FROM refresh_tokens WHERE id = $1";

        let slf = if is_hiqlite() {
            DB::hql().query_map_optional(sql, params!(id)).await?
        } else {
            DB::pg_query_opt(sql, &[&id]).await?
        };

        Ok(slf)
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        let sql = r#"
INSERT INTO refresh_tokens
(id, user_id, nbf, exp, scope, is_mfa, session_id, access_token_jti, client_id,
client_generation)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
ON CONFLICT(id) DO UPDATE
SET user_id = $2, nbf = $3, exp = $4, scope = $5, session_id = $7, access_token_jti = $8,
    client_id = $9, client_generation = $10"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        self.id.clone(),
                        self.user_id.clone(),
                        self.nbf,
                        self.exp,
                        self.scope.clone(),
                        self.is_mfa,
                        self.session_id.clone(),
                        self.access_token_jti.clone(),
                        self.client_id.clone(),
                        self.client_generation.clone()
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &self.id,
                    &self.user_id,
                    &self.nbf,
                    &self.exp,
                    &self.scope,
                    &self.is_mfa,
                    &self.session_id,
                    &self.access_token_jti,
                    &self.client_id,
                    &self.client_generation,
                ],
            )
            .await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn row(client_id: Option<&str>) -> serde_json::Value {
        let mut row = serde_json::json!({
            "id": "rt_1234567890",
            "user_id": "user_1",
            "nbf": 1,
            "exp": 2,
            "scope": "openid",
            "is_mfa": false,
            "session_id": "sid_1234567890",
            "access_token_jti": "jti_1",
        });
        if let Some(cid) = client_id {
            row["client_id"] = serde_json::Value::String(cid.to_string());
        }
        row
    }

    #[test]
    fn test_client_id_mapping() {
        let rt: RefreshToken = serde_json::from_value(row(Some("client_1"))).unwrap();
        assert_eq!(rt.client_id.as_deref(), Some("client_1"));
        assert!(format!("{rt:?}").contains("client_id: Some(\"client_1\")"));

        // missing key -> None
        let rt: RefreshToken = serde_json::from_value(row(None)).unwrap();
        assert_eq!(rt.client_id, None);

        // explicit NULL -> None
        let mut null_row = row(None);
        null_row["client_id"] = serde_json::Value::Null;
        let rt: RefreshToken = serde_json::from_value(null_row).unwrap();
        assert_eq!(rt.client_id, None);
    }

    #[test]
    fn test_is_for_client_generation() {
        let mut rt: RefreshToken = serde_json::from_value(row(None)).unwrap();
        assert!(rt.is_for_client_generation("gen_a"));
        assert!(rt.is_for_client_generation(""));

        rt.client_id = Some("client_1".to_string());
        assert!(!rt.is_for_client_generation("gen_a"));
        assert!(!rt.is_for_client_generation(""));

        rt.client_generation = Some("gen_a".to_string());
        assert!(rt.is_for_client_generation("gen_a"));
        assert!(!rt.is_for_client_generation("gen_b"));
        assert!(!rt.is_for_client_generation(""));

        rt.client_generation = Some(String::new());
        assert!(rt.is_for_client_generation(""));
        assert!(!rt.is_for_client_generation("gen_a"));
    }
}
