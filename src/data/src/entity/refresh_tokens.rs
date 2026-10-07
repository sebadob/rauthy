use crate::database::DB;
use crate::entity::issued_tokens::IssuedToken;
use chrono::Utc;
use hiqlite::macros::{FromRow, params};
use rauthy_common::is_hiqlite;
use rauthy_derive::FromPgRow;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::Deserialize;
use std::fmt::{Debug, Formatter};

const SQL_DELETE_BY_USER_CLIENT: &str =
    "DELETE FROM refresh_tokens WHERE user_id = $1 AND client_id = $2";
pub(crate) const SQL_RT_DELETE_BY_CLIENT: &str = "DELETE FROM refresh_tokens WHERE client_id = $1";
const SQL_SAVE: &str = r#"
INSERT INTO refresh_tokens
(id, user_id, nbf, exp, scope, is_mfa, session_id, access_token_jti, client_id,
client_generation)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
ON CONFLICT(id) DO UPDATE
SET user_id = $2, nbf = $3, exp = $4, scope = $5, session_id = $7, access_token_jti = $8,
    client_id = $9, client_generation = $10"#;
pub(crate) const SQL_RT_DELETE_LEGACY: &str = "DELETE FROM refresh_tokens WHERE client_id IS NULL";

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

    /// Deletes all tokens of `client_id`, with `drop_legacy` also every legacy token
    /// (`client_id` NULL) of all users.
    pub(crate) async fn pg_delete_by_client_txn(
        txn: &deadpool_postgres::Transaction<'_>,
        client_id: &str,
        drop_legacy: bool,
    ) -> Result<(), ErrorResponse> {
        DB::pg_txn_append(txn, SQL_RT_DELETE_BY_CLIENT, &[&client_id]).await?;
        if drop_legacy {
            DB::pg_txn_append(txn, SQL_RT_DELETE_LEGACY, &[]).await?;
        }
        Ok(())
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
        let sql_1 = SQL_DELETE_BY_USER_CLIENT;
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
        let sql = SQL_SAVE;

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

    /// Rotation and revocation interleavings against a live Postgres, using the production SQL.
    /// Run with `RAUTHY_TEST_PG_URL=postgres://.. cargo test -p rauthy-data -- --ignored`.
    mod pg_interleaving {
        use super::super::*;
        use crate::entity::devices::DeviceEntity;
        use deadpool_postgres::{Client, Config, Runtime};
        use std::sync::atomic::{AtomicUsize, Ordering};
        use tokio_postgres::NoTls;

        const USER: &str = "u1";
        const CLIENT: &str = "c1";

        async fn connect(url: &str, schema: &str) -> Client {
            let cfg = Config {
                url: Some(url.to_string()),
                ..Default::default()
            };
            let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
            let cl = pool.get().await.unwrap();
            cl.batch_execute(&format!("SET search_path TO {schema}"))
                .await
                .unwrap();
            cl
        }

        async fn setup() -> Option<(String, String, Client)> {
            let url = std::env::var("RAUTHY_TEST_PG_URL").ok()?;
            let nanos = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            static SEQ: AtomicUsize = AtomicUsize::new(0);
            let seq = SEQ.fetch_add(1, Ordering::Relaxed);
            let schema = format!("rt_test_{}_{nanos}_{seq}", std::process::id());
            let cl = connect(&url, "public").await;
            cl.batch_execute(&format!(
                r#"
CREATE SCHEMA {schema};
SET search_path TO {schema};
CREATE TABLE refresh_tokens (
    id VARCHAR PRIMARY KEY, user_id VARCHAR NOT NULL, nbf BIGINT NOT NULL,
    exp BIGINT NOT NULL, scope VARCHAR, is_mfa BOOLEAN NOT NULL, session_id VARCHAR,
    access_token_jti VARCHAR, client_id VARCHAR, client_generation VARCHAR);
CREATE TABLE clients (id VARCHAR PRIMARY KEY, generation VARCHAR NOT NULL DEFAULT '');
CREATE TABLE devices (
    id VARCHAR PRIMARY KEY,
    client_id VARCHAR NOT NULL REFERENCES clients ON UPDATE CASCADE ON DELETE CASCADE,
    user_id VARCHAR, created BIGINT NOT NULL, access_exp BIGINT NOT NULL, refresh_exp BIGINT,
    peer_ip VARCHAR NOT NULL, name VARCHAR NOT NULL,
    client_generation VARCHAR NOT NULL DEFAULT '');
CREATE TABLE refresh_tokens_devices (
    id VARCHAR PRIMARY KEY, device_id VARCHAR NOT NULL, user_id VARCHAR NOT NULL,
    nbf BIGINT NOT NULL, exp BIGINT NOT NULL, scope VARCHAR, access_token_jti VARCHAR);
CREATE TABLE user_login_states (
    user_id VARCHAR NOT NULL, client_id VARCHAR NOT NULL, session_id VARCHAR);"#
            ))
            .await
            .unwrap();
            Some((url, schema, cl))
        }

        async fn teardown(cl: &Client, schema: &str) {
            cl.batch_execute(&format!("DROP SCHEMA {schema} CASCADE"))
                .await
                .unwrap();
        }

        async fn insert(cl: &Client, id: &str, user_id: &str, client_id: Option<&str>) {
            cl.execute(
                "INSERT INTO refresh_tokens (id, user_id, nbf, exp, is_mfa, client_id) \
                 VALUES ($1, $2, 0, 4102444800, false, $3)",
                &[&id, &user_id, &client_id],
            )
            .await
            .unwrap();
        }

        async fn client_delete_legacy(drop_legacy: bool) -> Vec<String> {
            let Some((url, schema, admin)) = setup().await else {
                panic!("RAUTHY_TEST_PG_URL is not set");
            };

            admin
                .execute(
                    "INSERT INTO user_login_states (user_id, client_id, session_id) \
                     VALUES ('u1', 'c1', NULL)",
                    &[],
                )
                .await
                .unwrap();
            insert(&admin, "legacy_u1", "u1", None).await;
            insert(&admin, "legacy_u2", "u2", None).await;
            insert(&admin, "c1_u1", "u1", Some("c1")).await;
            insert(&admin, "c2_u1", "u1", Some("c2")).await;

            // logout without token revocation drops the login states, keeps the tokens
            admin
                .execute("DELETE FROM user_login_states WHERE user_id = 'u1'", &[])
                .await
                .unwrap();

            let mut cl = connect(&url, &schema).await;
            let txn = cl.transaction().await.unwrap();
            RefreshToken::pg_delete_by_client_txn(&txn, "c1", drop_legacy)
                .await
                .unwrap();
            txn.commit().await.unwrap();

            let left: Vec<String> = admin
                .query("SELECT id FROM refresh_tokens ORDER BY id", &[])
                .await
                .unwrap()
                .iter()
                .map(|r| r.get(0))
                .collect();

            teardown(&admin, &schema).await;
            left
        }

        #[tokio::test]
        #[ignore = "needs Postgres via RAUTHY_TEST_PG_URL"]
        async fn test_client_delete_drops_unattributable_legacy() {
            assert_eq!(client_delete_legacy(true).await, vec!["c2_u1".to_string()]);
        }

        #[tokio::test]
        #[ignore = "needs Postgres via RAUTHY_TEST_PG_URL"]
        async fn test_client_delete_keeps_legacy_without_consent() {
            assert_eq!(
                client_delete_legacy(false).await,
                vec![
                    "c2_u1".to_string(),
                    "legacy_u1".to_string(),
                    "legacy_u2".to_string()
                ]
            );
        }

        async fn save_in(cl: &Client, rt: &RefreshToken) {
            cl.execute(
                SQL_SAVE,
                &[
                    &rt.id,
                    &rt.user_id,
                    &rt.nbf,
                    &rt.exp,
                    &rt.scope,
                    &rt.is_mfa,
                    &rt.session_id,
                    &rt.access_token_jti,
                    &rt.client_id,
                    &rt.client_generation,
                ],
            )
            .await
            .unwrap();
        }

        async fn load(cl: &Client, id: &str) -> RefreshToken {
            RefreshToken::from(
                cl.query_one("SELECT * FROM refresh_tokens WHERE id = $1", &[&id])
                    .await
                    .unwrap(),
            )
        }

        async fn insert_client(cl: &Client) -> String {
            let generation = crate::entity::clients::Client::new_generation();
            cl.execute(
                "INSERT INTO clients (id, generation) VALUES ($1, $2)",
                &[&CLIENT, &generation],
            )
            .await
            .unwrap();
            generation
        }

        fn issued(id: &str, generation: &str) -> RefreshToken {
            RefreshToken {
                id: id.to_string(),
                user_id: USER.to_string(),
                nbf: 0,
                exp: 4102444800,
                scope: None,
                is_mfa: false,
                session_id: None,
                access_token_jti: None,
                client_id: Some(CLIENT.to_string()),
                client_generation: Some(generation.to_string()),
            }
        }

        async fn find_generation(cl: &Client) -> Option<String> {
            cl.query_opt(crate::entity::clients::SQL_FIND_GENERATION, &[&CLIENT])
                .await
                .unwrap()
                .map(|r| r.get(0))
        }

        #[tokio::test]
        #[ignore = "needs Postgres via RAUTHY_TEST_PG_URL"]
        async fn test_find_generation() {
            let Some((_url, schema, admin)) = setup().await else {
                panic!("RAUTHY_TEST_PG_URL is not set");
            };

            assert_eq!(find_generation(&admin).await, None);
            let generation = insert_client(&admin).await;
            assert_eq!(find_generation(&admin).await, Some(generation));
            admin
                .execute("DELETE FROM clients WHERE id = $1", &[&CLIENT])
                .await
                .unwrap();
            assert_eq!(find_generation(&admin).await, None);

            teardown(&admin, &schema).await;
        }

        #[tokio::test]
        #[ignore = "needs Postgres via RAUTHY_TEST_PG_URL"]
        async fn test_stale_generation_after_client_recreation() {
            let Some((url, schema, admin)) = setup().await else {
                panic!("RAUTHY_TEST_PG_URL is not set");
            };

            // issuance loads the client, which is then deleted and recreated with the same id
            let gen_old = insert_client(&admin).await;
            let mut cl = connect(&url, &schema).await;
            let txn = cl.transaction().await.unwrap();
            RefreshToken::pg_delete_by_client_txn(&txn, CLIENT, true)
                .await
                .unwrap();
            txn.execute("DELETE FROM clients WHERE id = $1", &[&CLIENT])
                .await
                .unwrap();
            txn.commit().await.unwrap();
            let gen_new = insert_client(&admin).await;
            assert_ne!(gen_old, gen_new);

            // the stale issuance writes its token after the sweep
            save_in(&admin, &issued("rt_stale", &gen_old)).await;
            save_in(&admin, &issued("rt_fresh", &gen_new)).await;

            let current = find_generation(&admin).await.unwrap();
            assert_eq!(current, gen_new);
            assert!(
                !load(&admin, "rt_stale")
                    .await
                    .is_for_client_generation(&current)
            );
            assert!(
                load(&admin, "rt_fresh")
                    .await
                    .is_for_client_generation(&current)
            );

            teardown(&admin, &schema).await;
        }

        async fn delete_client(url: &str, schema: &str) {
            let mut cl = connect(url, schema).await;
            let txn = cl.transaction().await.unwrap();
            RefreshToken::pg_delete_by_client_txn(&txn, CLIENT, true)
                .await
                .unwrap();
            txn.execute("DELETE FROM clients WHERE id = $1", &[&CLIENT])
                .await
                .unwrap();
            txn.commit().await.unwrap();
        }

        async fn insert_device(cl: &Client, id: &str, generation: &str) -> DeviceEntity {
            let none: Option<i64> = None;
            let user = Some(USER.to_string());
            cl.execute(
                crate::entity::devices::SQL_INSERT,
                &[
                    &id,
                    &CLIENT,
                    &user,
                    &0i64,
                    &4102444800i64,
                    &none,
                    &"127.0.0.1",
                    &id,
                    &generation,
                ],
            )
            .await
            .unwrap();
            cl.execute(
                "INSERT INTO refresh_tokens_devices (id, device_id, user_id, nbf, exp) \
                 VALUES ($1, $2, $3, 0, 4102444800)",
                &[&format!("rtd_{id}"), &id, &USER],
            )
            .await
            .unwrap();
            DeviceEntity::from(
                cl.query_one("SELECT * FROM devices WHERE id = $1", &[&id])
                    .await
                    .unwrap(),
            )
        }

        #[tokio::test]
        #[ignore = "needs Postgres via RAUTHY_TEST_PG_URL"]
        async fn test_device_after_client_recreation() {
            let Some((url, schema, admin)) = setup().await else {
                panic!("RAUTHY_TEST_PG_URL is not set");
            };

            // the device code is checked against the client, which is then deleted and recreated
            let gen_x = insert_client(&admin).await;
            let checked = find_generation(&admin).await;
            assert_eq!(checked.as_deref(), Some(gen_x.as_str()));
            delete_client(&url, &schema).await;
            let gen_y = insert_client(&admin).await;
            assert_ne!(gen_x, gen_y);

            // the FK accepts the stale device against the new client row
            let stale = insert_device(&admin, "dev_stale", &gen_x).await;
            assert_eq!(stale.client_generation, gen_x);

            let current = find_generation(&admin).await.unwrap();
            assert_eq!(current, gen_y);
            assert!(!stale.is_for_client_generation(&current));

            let fresh = insert_device(&admin, "dev_fresh", &gen_y).await;
            assert!(fresh.is_for_client_generation(&current));

            teardown(&admin, &schema).await;
        }
    }
}
