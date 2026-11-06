use crate::database::DB;
use crate::entity::refresh_tokens::{DiscardSql, RefreshToken, discard_txn, finish_rotation_with};
use hiqlite::macros::{FromRow, params};
use rauthy_common::is_hiqlite;
use rauthy_derive::FromPgRow;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use tokio_postgres::IsolationLevel;

/// The statements of [`RefreshTokenDevice::invalidate_with_refresh_tokens_for_user`], in order.
/// A refresh of a device token inserts its replacement into `refresh_tokens` and only then
/// deletes its source from `refresh_tokens_devices`, so the device tokens go first: a refresh
/// whose source is deleted before it finishes discards its replacement, and one that finished
/// before inserted the replacement early enough for the `refresh_tokens` delete to see it.
/// Like the consent revocation (`pg_revoke_steps`).
pub(crate) const SQL_INVALIDATE_FOR_USER: [&str; 2] = [
    "DELETE FROM refresh_tokens_devices WHERE user_id = $1",
    "DELETE FROM refresh_tokens WHERE user_id = $1",
];

/// [`SQL_INVALIDATE_FOR_USER`] with or without the device refresh tokens
fn sql_invalidate_for_user(devices: bool) -> &'static [&'static str] {
    if devices {
        &SQL_INVALIDATE_FOR_USER
    } else {
        &SQL_INVALIDATE_FOR_USER[1..]
    }
}

/// The statements of [`RefreshTokenDevice::discard`], like
/// [`SQL_RT_DISCARD`](crate::entity::refresh_tokens::SQL_RT_DISCARD).
pub(crate) const SQL_RTD_DISCARD: DiscardSql = DiscardSql {
    hql: [
        r#"
UPDATE issued_tokens SET revoked = true
WHERE jti = (SELECT access_token_jti FROM refresh_tokens_devices WHERE id = $1)"#,
        "DELETE FROM refresh_tokens_devices WHERE id = $1",
    ],
    pg_delete: "DELETE FROM refresh_tokens_devices WHERE id = $1 RETURNING access_token_jti",
};

#[derive(Serialize, Deserialize, FromRow, FromPgRow)]
pub struct RefreshTokenDevice {
    pub id: String,
    pub device_id: String,
    pub user_id: String,
    pub nbf: i64,
    pub exp: i64,
    pub scope: Option<String>,
    pub access_token_jti: Option<String>,
}

impl Debug for RefreshTokenDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RefreshTokenDevice {{ id: {}(...), device_id: {}, nbf: {}, exp: {}, scope: {:?}, \
            access_token_jti: {:?} }}",
            &self.id[..5],
            self.device_id,
            self.nbf,
            self.exp,
            self.scope,
            self.access_token_jti,
        )
    }
}

// CRUD
impl RefreshTokenDevice {
    pub async fn create(
        id: String,
        device_id: String,
        user_id: String,
        nbf: i64,
        exp: i64,
        scope: Option<String>,
        access_token_jti: Option<String>,
    ) -> Result<Self, ErrorResponse> {
        let rt = Self {
            id,
            device_id,
            user_id,
            nbf,
            exp,
            scope,
            access_token_jti,
        };

        rt.save().await?;
        Ok(rt)
    }

    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM refresh_tokens_devices WHERE id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(self.id.clone())).await?;
        } else {
            DB::pg_execute(sql, &[&self.id]).await?;
        }

        Ok(())
    }

    /// Return an error when no rows have been affected.
    pub async fn delete_checked(&self) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM refresh_tokens_devices WHERE id = $1";
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

    /// Deletes the device refresh token `id` and revokes the access token issued with it, in
    /// one transaction, like [`RefreshToken::discard`]. An `id` that is already gone is fine.
    pub async fn discard(id: &str) -> Result<(), ErrorResponse> {
        discard_txn(&SQL_RTD_DISCARD, id).await
    }

    /// Ends the rotation of this device refresh token into `replacement_id`, a `refresh_tokens`
    /// row: a refresh re-issues a device token as a regular one. The source is kept until the
    /// replacement exists. If it is gone by then (a consent revocation, a device deletion or a
    /// concurrent refresh), the replacement is discarded, like [`RefreshToken::finish_rotation`]
    /// does.
    pub async fn finish_rotation(&self, replacement_id: &str) -> Result<(), ErrorResponse> {
        finish_rotation_with(self.delete_checked(), || {
            RefreshToken::discard(replacement_id)
        })
        .await
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM refresh_tokens_devices";
        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 0).await?
        };
        Ok(res)
    }

    pub async fn invalidate_all() -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM refresh_tokens_devices";
        if is_hiqlite() {
            DB::hql().execute(sql, params!()).await?;
        } else {
            DB::pg_execute(sql, &[]).await?;
        }

        Ok(())
    }

    pub async fn invalidate_all_for_user(user_id: &str) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM refresh_tokens_devices WHERE user_id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(user_id)).await?;
        } else {
            DB::pg_execute(sql, &[&user_id]).await?;
        }

        Ok(())
    }

    /// Deletes all refresh tokens of `user_id` in one transaction: the device refresh tokens
    /// when `devices`, and the regular ones, in the order of [`SQL_INVALIDATE_FOR_USER`].
    pub async fn invalidate_with_refresh_tokens_for_user(
        user_id: &str,
        devices: bool,
    ) -> Result<(), ErrorResponse> {
        let stmts = sql_invalidate_for_user(devices);
        if is_hiqlite() {
            let stmts: Vec<_> = stmts.iter().map(|sql| (*sql, params!(user_id))).collect();
            for res in DB::hql().txn(stmts).await? {
                res?;
            }
        } else {
            let mut cl = DB::pg().await?;
            let txn = cl
                .build_transaction()
                .isolation_level(IsolationLevel::ReadCommitted)
                .start()
                .await?;
            for sql in stmts {
                DB::pg_txn_append(&txn, sql, &[&user_id]).await?;
            }
            txn.commit().await?;
        }
        Ok(())
    }

    /// Finds and delete the refresh token in an atomic operation.
    pub async fn find_delete(id: &str) -> Result<Self, ErrorResponse> {
        let sql = "DELETE FROM refresh_tokens_devices WHERE id = $1 RETURNING *";

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
        let sql = "SELECT * FROM refresh_tokens_devices WHERE id = $1";

        let slf = if is_hiqlite() {
            DB::hql().query_map_optional(sql, params!(id)).await?
        } else {
            DB::pg_query_opt(sql, &[&id]).await?
        };

        Ok(slf)
    }

    pub async fn find_by_user_id_jti(
        user_id: &str,
        jti: &str,
    ) -> Result<Option<Self>, ErrorResponse> {
        let sql =
            "SELECT * FROM refresh_tokens_devices WHERE user_id = $1 AND access_token_jti = $2";

        let slf = if is_hiqlite() {
            DB::hql()
                .query_map_optional(sql, params!(user_id, jti))
                .await?
        } else {
            DB::pg_query_opt(sql, &[&user_id, &jti]).await?
        };

        Ok(slf)
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        let sql = r#"
INSERT INTO refresh_tokens_devices
(id, device_id, user_id, nbf, exp, scope, access_token_jti)
VALUES ($1, $2, $3, $4, $5, $6, $7)
ON CONFLICT(id) DO UPDATE
SET device_id = $2, user_id = $3, nbf = $4, exp = $5, scope = $6, access_token_jti = $7"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        self.id.clone(),
                        self.device_id.clone(),
                        self.user_id.clone(),
                        self.nbf,
                        self.exp,
                        self.scope.clone(),
                        self.access_token_jti.clone()
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &self.id,
                    &self.device_id,
                    &self.user_id,
                    &self.nbf,
                    &self.exp,
                    &self.scope,
                    &self.access_token_jti,
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

    /// The statements without the device refresh tokens leave `refresh_tokens_devices` alone.
    #[test]
    fn test_sql_invalidate_for_user() {
        assert_eq!(sql_invalidate_for_user(true), &SQL_INVALIDATE_FOR_USER);
        assert_eq!(
            sql_invalidate_for_user(false),
            &["DELETE FROM refresh_tokens WHERE user_id = $1"]
        );
    }

    /// A refresh of the device token `rtd_src` of u1 runs while all refresh tokens of u1 are
    /// deleted (a logout or `DELETE /sessions/{user_id}` with `token_revoke_device_tokens`):
    /// after the first statement of [`SQL_INVALIDATE_FOR_USER`], the refresh inserts its
    /// replacement into `refresh_tokens` and deletes its source
    /// (`RefreshTokenDevice::finish_rotation`), then the rest runs and commits. The replacement
    /// must not survive.
    /// Run with `RAUTHY_TEST_PG_URL=postgres://.. cargo test -p rauthy-data -- --ignored`.
    #[tokio::test]
    #[ignore = "needs Postgres via RAUTHY_TEST_PG_URL"]
    async fn test_invalidate_for_user_device_refresh_race_postgres() {
        use deadpool_postgres::{Config, Runtime};
        use tokio_postgres::NoTls;

        let url = std::env::var("RAUTHY_TEST_PG_URL").expect("RAUTHY_TEST_PG_URL is not set");
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let schema = format!("rtd_race_{}_{nanos}", std::process::id());
        let cfg = Config {
            url: Some(url),
            ..Default::default()
        };
        let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

        let dir =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../migrations/postgres");
        let mut files = std::fs::read_dir(dir)
            .unwrap()
            .map(|e| e.unwrap().path())
            .map(|p| {
                let name = p.file_name().unwrap().to_str().unwrap();
                let nr: u32 = name[1..].split("__").next().unwrap().parse().unwrap();
                (nr, p)
            })
            .collect::<Vec<_>>();
        files.sort();

        let setup = pool.get().await.unwrap();
        setup
            .batch_execute(&format!(
                "CREATE SCHEMA {schema}; SET search_path TO {schema};"
            ))
            .await
            .unwrap();
        for (_, path) in files {
            setup
                .batch_execute(&std::fs::read_to_string(path).unwrap())
                .await
                .unwrap();
        }
        setup
            .batch_execute(
                r#"
CREATE TEMP TABLE tu AS SELECT * FROM users LIMIT 1;
UPDATE tu SET id = 'u1', email = 'u1@example.com';
INSERT INTO users SELECT * FROM tu;
UPDATE tu SET id = 'u2', email = 'u2@example.com';
INSERT INTO users SELECT * FROM tu;
CREATE TEMP TABLE tc AS SELECT * FROM clients LIMIT 1;
UPDATE tc SET id = 'c1';
INSERT INTO clients SELECT * FROM tc;
INSERT INTO devices (id, client_id, user_id, created, access_exp, peer_ip, name)
VALUES ('d1', 'c1', 'u1', 1, 2, '127.0.0.1', 'd1'),
       ('d2', 'c1', 'u2', 1, 2, '127.0.0.1', 'd2');
INSERT INTO refresh_tokens_devices (id, device_id, user_id, nbf, exp, access_token_jti)
VALUES ('rtd_src', 'd1', 'u1', 1, 2, 'j_src'),
       ('rtd_u2', 'd2', 'u2', 1, 2, 'j_u2');
INSERT INTO refresh_tokens (id, user_id, nbf, exp, access_token_jti, client_id)
VALUES ('rt_u1', 'u1', 1, 2, 'j_u1', 'c1'),
       ('rt_u2', 'u2', 1, 2, 'j_u2_rt', 'c1');
INSERT INTO issued_tokens (jti, user_id, exp)
VALUES ('j_src', 'u1', 2), ('j_new', 'u1', 2), ('j_u1', 'u1', 2);
"#,
            )
            .await
            .unwrap();
        drop(setup);

        let conn = async || {
            let cl = pool.get().await.unwrap();
            cl.batch_execute(&format!("SET search_path TO {schema}"))
                .await
                .unwrap();
            cl
        };
        let mut invalidator = conn().await;
        let refresher = conn().await;
        let monitor = conn().await;
        let pid: i32 = refresher
            .query_one("SELECT pg_backend_pid()", &[])
            .await
            .unwrap()
            .get(0);

        let txn = invalidator.transaction().await.unwrap();
        txn.execute(SQL_INVALIDATE_FOR_USER[0], &[&"u1"])
            .await
            .unwrap();

        // `TokenSet::from_user` inserts the replacement
        refresher
            .execute(
                r#"
INSERT INTO refresh_tokens (id, user_id, nbf, exp, access_token_jti, client_id)
VALUES ('rt_new', 'u1', 1, 2, 'j_new', 'c1')"#,
                &[],
            )
            .await
            .unwrap();
        // `delete_checked` of the source, which may block on the row lock of the invalidation
        let delete = tokio::spawn(async move {
            let rows = refresher
                .execute(
                    "DELETE FROM refresh_tokens_devices WHERE id = 'rtd_src'",
                    &[],
                )
                .await
                .unwrap();
            (refresher, rows)
        });
        let mut settled = false;
        for _ in 0..500 {
            if delete.is_finished() {
                settled = true;
                break;
            }
            let wait: Option<String> = monitor
                .query_one(
                    "SELECT wait_event_type FROM pg_stat_activity WHERE pid = $1",
                    &[&pid],
                )
                .await
                .unwrap()
                .get(0);
            if wait.as_deref() == Some("Lock") {
                settled = true;
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
        assert!(settled, "the source delete neither finished nor blocked");

        for sql in &SQL_INVALIDATE_FOR_USER[1..] {
            txn.execute(*sql, &[&"u1"]).await.unwrap();
        }
        txn.commit().await.unwrap();
        let (mut refresher, rows) = delete.await.unwrap();

        // `finish_rotation`
        let delete_checked = async {
            if rows > 0 {
                Ok(())
            } else {
                Err(ErrorResponse::new(
                    ErrorResponseType::NotFound,
                    "Invalid Refresh Token",
                ))
            }
        };
        let res = finish_rotation_with(delete_checked, || {
            crate::entity::refresh_tokens::pg_discard_txn(
                &mut refresher,
                &crate::entity::refresh_tokens::SQL_RT_DISCARD,
                "rt_new",
            )
        })
        .await;

        let ids = async |sql: &str| -> Vec<String> {
            let rows = monitor.query(sql, &[]).await.unwrap();
            rows.iter().map(|r| r.get(0)).collect()
        };
        // the tokens of u2 stay
        assert_eq!(
            ids("SELECT id FROM refresh_tokens ORDER BY id").await,
            vec!["rt_u2"]
        );
        assert_eq!(
            ids("SELECT id FROM refresh_tokens_devices").await,
            vec!["rtd_u2"]
        );
        // The access tokens of u1 are revoked by the callers with `IssuedToken::revoke_for_user`.
        assert!(res.is_err(), "the rotation must fail");

        monitor
            .batch_execute(&format!("DROP SCHEMA {schema} CASCADE"))
            .await
            .unwrap();
    }
}
