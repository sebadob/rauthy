use crate::database::DB;
use crate::entity::refresh_tokens::{
    SQL_HQL_IT_REVOKE_BY_USER_CLIENT, SQL_HQL_IT_REVOKE_BY_USER_CLIENT_OR_LEGACY,
    SQL_PG_IT_REVOKE_JTIS, SQL_PG_RT_DELETE_BY_USER_CLIENT_RET,
    SQL_PG_RT_DELETE_LEGACY_BY_USER_RET, SQL_PG_RTD_DELETE_BY_USER_CLIENT_RET,
    SQL_RT_DELETE_BY_USER_CLIENT, SQL_RT_DELETE_BY_USER_CLIENT_OR_LEGACY,
    SQL_RTD_DELETE_BY_USER_CLIENT,
};
use crate::rauthy_config::RauthyConfig;
use chrono::Utc;
use hiqlite::macros::params;
use rauthy_api_types::users::UserConsentResponse;
use rauthy_common::is_hiqlite;
use rauthy_derive::FromPgRow;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use tokio_postgres::IsolationLevel;

const SQL_FIND: &str = r#"
SELECT * FROM user_consents
WHERE user_id = $1 AND client_id = $2 AND resource = $3"#;
/// [`UserConsent::exists_fenced`] on hiqlite
const SQL_HQL_EXISTS: &str = r#"
SELECT 1 FROM user_consents
WHERE user_id = $1 AND client_id = $2 AND resource = $3"#;
/// [`UserConsent::exists_fenced`] on Postgres. `FOR SHARE` waits for a concurrent
/// [`UserConsent::revoke`] that already deleted the row, and then finds it gone.
const SQL_PG_EXISTS_FENCED: &str = r#"
SELECT 1 FROM user_consents
WHERE user_id = $1 AND client_id = $2 AND resource = $3
FOR SHARE"#;
const SQL_FIND_FOR_USER: &str =
    "SELECT * FROM user_consents WHERE user_id = $1 ORDER BY updated DESC";
const SQL_FIND_FOR_USER_RESOURCE: &str =
    "SELECT * FROM user_consents WHERE user_id = $1 AND resource = $2 ORDER BY updated DESC";
const SQL_UPSERT: &str = r#"
INSERT INTO user_consents
(user_id, client_id, resource, scopes, content_hash, created, updated, ip)
VALUES ($1, $2, $3, $4, $5, $6, $6, $7)
ON CONFLICT(user_id, client_id, resource) DO UPDATE
SET scopes = $4, content_hash = $5, updated = $6, ip = $7"#;
const SQL_DELETE: &str = r#"
DELETE FROM user_consents
WHERE user_id = $1 AND client_id = $2 AND resource = $3"#;
pub(crate) const SQL_CONSENT_DELETE_BY_CLIENT: &str =
    "DELETE FROM user_consents WHERE client_id = $1";
/// [`PgRevokeStep::LockTokens`]
const SQL_PG_RT_LOCK_BY_USER_CLIENT: &str = r#"
SELECT 1 FROM refresh_tokens WHERE user_id = $1 AND client_id = $2
FOR UPDATE"#;

/// A user's consent to a client, optionally scoped to a single resource. `resource` is empty for
/// a consent without one.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromPgRow)]
pub struct UserConsent {
    pub user_id: String,
    pub client_id: String,
    pub resource: String,
    /// comma separated
    pub scopes: String,
    pub content_hash: String,
    pub created: i64,
    pub updated: i64,
    pub ip: String,
}

impl UserConsent {
    pub async fn find(
        user_id: &str,
        client_id: &str,
        resource: &str,
    ) -> Result<Option<Self>, ErrorResponse> {
        let slf = if is_hiqlite() {
            DB::hql()
                .query_as_optional(SQL_FIND, params!(user_id, client_id, resource))
                .await?
        } else {
            DB::pg_query_opt(SQL_FIND, &[&user_id, &client_id, &resource]).await?
        };
        Ok(slf)
    }

    /// Whether the grant exists, ordered against a concurrent [`Self::revoke`]: once this returns
    /// `true`, a revoke of the grant that has not committed yet deletes every refresh token that
    /// was committed before this call. Used by the fence after the code exchange inserted its
    /// refresh token.
    ///
    /// Postgres: the revoke deletes the grant first and holds that row lock until it commits.
    /// `FOR SHARE` blocks on it, and after the commit finds the row gone. If the revoke has not
    /// deleted the grant yet, its refresh token delete runs later, in a statement that sees the
    /// already committed refresh token.
    /// hiqlite: a leader read after all committed logs were applied. The revoke is one raft log
    /// entry: either it is applied before the read and the grant is gone, or after the refresh
    /// token insert and deletes the token.
    pub async fn exists_fenced(
        user_id: &str,
        client_id: &str,
        resource: &str,
    ) -> Result<bool, ErrorResponse> {
        let exists = if is_hiqlite() {
            !DB::hql()
                .query_consistent(SQL_HQL_EXISTS, params!(user_id, client_id, resource))
                .await?
                .is_empty()
        } else {
            let cl = DB::pg().await?;
            let st = cl.prepare_cached(SQL_PG_EXISTS_FENCED).await?;
            cl.query_opt(&st, &[&user_id, &client_id, &resource])
                .await?
                .is_some()
        };
        Ok(exists)
    }

    pub async fn find_for_user(
        user_id: &str,
        resource: Option<&str>,
    ) -> Result<Vec<Self>, ErrorResponse> {
        let res = if let Some(resource) = resource {
            if is_hiqlite() {
                DB::hql()
                    .query_as(SQL_FIND_FOR_USER_RESOURCE, params!(user_id, resource))
                    .await?
            } else {
                DB::pg_query(SQL_FIND_FOR_USER_RESOURCE, &[&user_id, &resource], 0).await?
            }
        } else if is_hiqlite() {
            DB::hql()
                .query_as(SQL_FIND_FOR_USER, params!(user_id))
                .await?
        } else {
            DB::pg_query(SQL_FIND_FOR_USER, &[&user_id], 0).await?
        };
        Ok(res)
    }

    pub async fn upsert(
        user_id: &str,
        client_id: &str,
        resource: &str,
        scopes: &[String],
        content_hash: &str,
        ip: IpAddr,
    ) -> Result<(), ErrorResponse> {
        let scopes = scopes.join(",");
        let now = Utc::now().timestamp();
        let ip = ip.to_string();

        if is_hiqlite() {
            DB::hql()
                .execute(
                    SQL_UPSERT,
                    params!(user_id, client_id, resource, scopes, content_hash, now, ip),
                )
                .await?;
        } else {
            DB::pg_execute(
                SQL_UPSERT,
                &[
                    &user_id,
                    &client_id,
                    &resource,
                    &scopes,
                    &content_hash,
                    &now,
                    &ip,
                ],
            )
            .await?;
        }
        Ok(())
    }

    /// Revokes the consent together with all refresh tokens of the user for the client in a
    /// single transaction, and revokes the access tokens issued alongside those refresh tokens.
    /// Refresh tokens are deleted for the client as a whole, independent of `resource`. With
    /// `consent.enable`, legacy refresh tokens of the user without a `client_id` are deleted as
    /// well, because they are valid for any client, like `Client::delete` does. Without it, they
    /// stay, so that a revoke does not sign the user out of every client still on legacy tokens.
    pub async fn revoke(
        user_id: &str,
        client_id: &str,
        resource: &str,
    ) -> Result<ConsentRevoked, ErrorResponse> {
        let drop_legacy = RauthyConfig::get().vars.consent.enable;
        if is_hiqlite() {
            let stmts: Vec<_> = hql_revoke(drop_legacy)
                .iter()
                .map(|(sql, params)| {
                    let params = match params {
                        RevokeParams::UserClient => params!(user_id, client_id),
                        RevokeParams::UserClientResource => params!(user_id, client_id, resource),
                    };
                    (*sql, params)
                })
                .collect();
            let mut rows = Vec::with_capacity(HQL_REVOKE_ROWS.len());
            for res in DB::hql().txn(stmts).await? {
                rows.push(res?);
            }
            Ok(ConsentRevoked::from_hql_rows(rows))
        } else {
            let mut cl = DB::pg().await?;
            pg_revoke(&mut cl, drop_legacy, user_id, client_id, resource).await
        }
    }
}

/// The Postgres transaction of [`UserConsent::revoke`]: [`pg_revoke_steps`], then the access
/// tokens. Read committed is what the order of the steps relies on: each statement takes a new
/// snapshot, and sees what was committed while an earlier one waited for a lock.
async fn pg_revoke(
    cl: &mut deadpool_postgres::Object,
    drop_legacy: bool,
    user_id: &str,
    client_id: &str,
    resource: &str,
) -> Result<ConsentRevoked, ErrorResponse> {
    let txn = cl
        .build_transaction()
        .isolation_level(IsolationLevel::ReadCommitted)
        .start()
        .await?;
    let mut revoke = PgRevoke::default();
    for step in pg_revoke_steps(drop_legacy) {
        revoke
            .step(&txn, *step, user_id, client_id, resource)
            .await?;
    }
    revoke.revoke_access_tokens(&txn).await?;
    txn.commit().await?;
    Ok(revoke.revoked())
}

/// The result of [`UserConsent::revoke`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConsentRevoked {
    pub consent_deleted: bool,
    /// From `refresh_tokens` and `refresh_tokens_devices` together
    pub refresh_tokens_deleted: usize,
}

impl ConsentRevoked {
    /// From the rows each statement of [`hql_revoke`] affected, in order.
    fn from_hql_rows(rows: impl IntoIterator<Item = usize>) -> Self {
        let mut slf = Self {
            consent_deleted: false,
            refresh_tokens_deleted: 0,
        };
        for (rows, kind) in rows.into_iter().zip(HQL_REVOKE_ROWS) {
            match kind {
                RevokeRows::Consent => slf.consent_deleted |= rows > 0,
                RevokeRows::RefreshTokens => slf.refresh_tokens_deleted += rows,
                RevokeRows::None => {}
            }
        }
        slf
    }
}

/// A statement of the Postgres transaction of [`UserConsent::revoke`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PgRevokeStep {
    /// The grant
    Consent,
    /// `refresh_tokens_devices` of the client's devices
    DeviceTokens,
    /// `refresh_tokens` of the user without `client_id`
    LegacyTokens,
    /// Locks the `refresh_tokens` of the client, before [`Self::RefreshTokens`] deletes them
    LockTokens,
    /// `refresh_tokens` of the client
    RefreshTokens,
}

/// The statements of the Postgres transaction of [`UserConsent::revoke`], in order, run with read
/// committed. A refresh inserts its replacement for the client into `refresh_tokens` and only
/// then deletes its source (`delete_checked`); the code exchange inserts its refresh token and
/// only then checks the grant ([`UserConsent::exists_fenced`]). A check that blocks on a row the
/// revoke already holds finds it gone and discards the new token. A check that ran before leaves
/// a new token that is gone after the revoke only if it was committed before a later statement
/// of the revoke took its snapshot:
/// - The grant goes first, for the code exchange.
/// - `refresh_tokens_devices` and, with `consent.enable`, the legacy `refresh_tokens` are sources
///   in other rows than the `refresh_tokens` of the client. A refresh that deleted one of them
///   first committed its replacement before the later `refresh_tokens` delete took its snapshot,
///   so that delete sees it. One that blocks on a row the revoke deleted discards it.
/// - A source in the `refresh_tokens` of the client itself is locked by
///   [`PgRevokeStep::LockTokens`] first. The delete alone cannot cover it: its snapshot is taken
///   when it starts, and a refresh that deletes the source before the delete reaches it commits a
///   replacement the delete cannot see. The lock holds every source that was committed when it
///   took its snapshot and still exists when it reaches it, so a later rotation of those blocks
///   and discards its replacement. A source the lock finds deleted by a refresh that was running
///   when it started was replaced by that refresh; the replacement is not locked, but was
///   committed before the delete took its snapshot, so the delete sees it.
///
/// What remains open: the replacement of such a refresh is a live source the revoke does not
/// hold between the lock and the delete. A second refresh that rotates it and deletes it before
/// the delete reaches it, but commits after the delete took its snapshot, makes the delete skip
/// the deleted row and miss the second replacement, which survives the revoke. It takes two
/// rotations of the same token family in a row, the first running while the lock starts and the
/// second while the delete starts. Closing it would take a lock the refresh waits on before it
/// rotates, not a statement of this transaction.
///
/// This holds for this transaction only. A logout covers the device sources the same way, by
/// deleting `refresh_tokens_devices` before `refresh_tokens`; it does not lock the
/// `refresh_tokens` it deletes first.
///
/// `Client::delete` and, for the tables they share, `User::delete` take the same tables in the
/// same order, so that they and a revoke running at the same time are serialized instead of
/// deadlocking.
fn pg_revoke_steps(drop_legacy: bool) -> &'static [PgRevokeStep] {
    use PgRevokeStep::*;
    if drop_legacy {
        &[
            Consent,
            DeviceTokens,
            LegacyTokens,
            LockTokens,
            RefreshTokens,
        ]
    } else {
        &[Consent, DeviceTokens, LockTokens, RefreshTokens]
    }
}

/// The state of the Postgres transaction of [`UserConsent::revoke`].
#[derive(Debug, Default)]
struct PgRevoke {
    consent_rows: usize,
    rt_rows: usize,
    /// `access_token_jti` of the deleted refresh tokens
    jtis: Vec<String>,
}

impl PgRevoke {
    async fn step(
        &mut self,
        txn: &deadpool_postgres::Transaction<'_>,
        step: PgRevokeStep,
        user_id: &str,
        client_id: &str,
        resource: &str,
    ) -> Result<(), ErrorResponse> {
        let rows = match step {
            PgRevokeStep::Consent => {
                self.consent_rows +=
                    DB::pg_txn_append(txn, SQL_DELETE, &[&user_id, &client_id, &resource]).await?
                        as usize;
                return Ok(());
            }
            PgRevokeStep::LockTokens => {
                txn.query(SQL_PG_RT_LOCK_BY_USER_CLIENT, &[&user_id, &client_id])
                    .await?;
                return Ok(());
            }
            PgRevokeStep::DeviceTokens => {
                txn.query(
                    SQL_PG_RTD_DELETE_BY_USER_CLIENT_RET,
                    &[&user_id, &client_id],
                )
                .await?
            }
            PgRevokeStep::RefreshTokens => {
                txn.query(SQL_PG_RT_DELETE_BY_USER_CLIENT_RET, &[&user_id, &client_id])
                    .await?
            }
            PgRevokeStep::LegacyTokens => {
                txn.query(SQL_PG_RT_DELETE_LEGACY_BY_USER_RET, &[&user_id])
                    .await?
            }
        };
        self.rt_rows += rows.len();
        self.jtis
            .extend(rows.iter().filter_map(|r| r.get::<_, Option<String>>(0)));
        Ok(())
    }

    fn revoked(&self) -> ConsentRevoked {
        ConsentRevoked {
            consent_deleted: self.consent_rows > 0,
            refresh_tokens_deleted: self.rt_rows,
        }
    }

    /// The last statement: revokes the access tokens of the deleted refresh tokens.
    async fn revoke_access_tokens(
        &self,
        txn: &deadpool_postgres::Transaction<'_>,
    ) -> Result<(), ErrorResponse> {
        if !self.jtis.is_empty() {
            DB::pg_txn_append(txn, SQL_PG_IT_REVOKE_JTIS, &[&self.jtis]).await?;
        }
        Ok(())
    }
}

enum RevokeParams {
    UserClient,
    UserClientResource,
}

#[derive(Clone, Copy)]
enum RevokeRows {
    None,
    Consent,
    RefreshTokens,
}

/// The hiqlite transaction of [`UserConsent::revoke`], with the legacy refresh tokens of the user
/// (`client_id` NULL) if `drop_legacy`. The access tokens are revoked before the refresh tokens
/// they are looked up from are deleted. hiqlite applies the transaction as one raft log entry,
/// so no other write sees a part of it; the order mirrors [`pg_revoke_steps`].
fn hql_revoke(drop_legacy: bool) -> [(&'static str, RevokeParams); 4] {
    let (sql_it, sql_rt) = if drop_legacy {
        (
            SQL_HQL_IT_REVOKE_BY_USER_CLIENT_OR_LEGACY,
            SQL_RT_DELETE_BY_USER_CLIENT_OR_LEGACY,
        )
    } else {
        (
            SQL_HQL_IT_REVOKE_BY_USER_CLIENT,
            SQL_RT_DELETE_BY_USER_CLIENT,
        )
    };
    [
        (SQL_DELETE, RevokeParams::UserClientResource),
        (sql_it, RevokeParams::UserClient),
        (SQL_RTD_DELETE_BY_USER_CLIENT, RevokeParams::UserClient),
        (sql_rt, RevokeParams::UserClient),
    ]
}
const HQL_REVOKE_ROWS: [RevokeRows; 4] = [
    RevokeRows::Consent,
    RevokeRows::None,
    RevokeRows::RefreshTokens,
    RevokeRows::RefreshTokens,
];

impl UserConsent {
    pub fn into_response(self, client_name: Option<String>) -> UserConsentResponse {
        UserConsentResponse {
            scopes: self.scopes_vec(),
            client_id: self.client_id,
            client_name,
            resource: self.resource,
            created: self.created,
            updated: self.updated,
        }
    }
}

impl UserConsent {
    pub fn scopes_vec(&self) -> Vec<String> {
        self.scopes
            .split(',')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use tokio_postgres::types::ToSql;

    const MIGRATION_HQL: &str = include_str!("../../../../migrations/hiqlite/37_user_consents.sql");
    const MIGRATION_PG: &str =
        include_str!("../../../../migrations/postgres/V32__user_consents.sql");
    const RES: &str = "https://api.example.com";

    fn consent(
        client_id: &str,
        resource: &str,
        scopes: &str,
        created: i64,
        ts: i64,
    ) -> UserConsent {
        UserConsent {
            user_id: "u1".to_string(),
            client_id: client_id.to_string(),
            resource: resource.to_string(),
            scopes: scopes.to_string(),
            content_hash: format!("hash_{ts}"),
            created,
            updated: ts,
            ip: "127.0.0.1".to_string(),
        }
    }

    // (client_id, resource, scopes, ts), the last one updates the first
    const UPSERTS: [(&str, &str, &str, i64); 4] = [
        ("c1", "", "openid", 1),
        ("c1", RES, "openid", 2),
        ("c2", "", "openid", 3),
        ("c1", "", "openid,email", 4),
    ];

    /// `SQL_FIND_FOR_USER` after `UPSERTS`, most recently updated first
    fn expected_order() -> Vec<(String, String)> {
        [("c1", ""), ("c2", ""), ("c1", RES)]
            .into_iter()
            .map(|(c, r)| (c.to_string(), r.to_string()))
            .collect()
    }

    #[test]
    fn test_scopes_vec() {
        assert_eq!(
            consent("c1", "", "openid,email", 1, 1).scopes_vec(),
            vec!["openid".to_string(), "email".to_string()]
        );
        assert!(consent("c1", "", "", 1, 1).scopes_vec().is_empty());
    }

    /// The production SQL against the hiqlite migration.
    #[test]
    fn test_sql_sqlite() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.pragma_update(None, "foreign_keys", "ON").unwrap();
        conn.execute_batch("CREATE TABLE users (id TEXT NOT NULL PRIMARY KEY) STRICT;")
            .unwrap();
        conn.execute_batch(MIGRATION_HQL).unwrap();
        conn.execute("INSERT INTO users (id) VALUES ('u1'), ('u2')", [])
            .unwrap();

        // u2 holds rows with the same (client_id, resource) keys
        for user_id in ["u1", "u2"] {
            for (client_id, resource, scopes, ts) in UPSERTS {
                let hash = format!("hash_{ts}");
                let params =
                    rusqlite::params![user_id, client_id, resource, scopes, hash, ts, "127.0.0.1"];
                conn.execute(SQL_UPSERT, params).unwrap();
            }
        }

        let query = |sql: &str, params: &[&dyn rusqlite::ToSql]| -> Vec<UserConsent> {
            let mut stmt = conn.prepare(sql).unwrap();
            let rows = stmt.query(params).unwrap();
            serde_rusqlite::from_rows::<UserConsent>(rows)
                .map(|r| r.unwrap())
                .collect()
        };

        assert_eq!(
            query(SQL_FIND, &[&"u1", &"c1", &""]),
            vec![consent("c1", "", "openid,email", 1, 4)]
        );
        assert_eq!(query(SQL_FIND, &[&"u2", &"c1", &""])[0].user_id, "u2");
        assert!(query(SQL_FIND, &[&"u3", &"c1", &""]).is_empty());
        let keys = |v: Vec<UserConsent>| -> Vec<(String, String)> {
            v.into_iter().map(|c| (c.client_id, c.resource)).collect()
        };
        // the rows of u2 are left out
        assert_eq!(keys(query(SQL_FIND_FOR_USER, &[&"u1"])), expected_order());
        assert_eq!(
            query(SQL_FIND_FOR_USER_RESOURCE, &[&"u1", &RES]),
            vec![consent("c1", RES, "openid", 2, 2)]
        );
        let for_resource = query(SQL_FIND_FOR_USER_RESOURCE, &[&"u1", &""]);
        assert_eq!(for_resource.len(), 2);
        assert!(for_resource.iter().all(|c| c.user_id == "u1"));

        // idempotent, the resource is part of the key
        let delete = rusqlite::params!["u1", "c1", ""];
        assert_eq!(conn.execute(SQL_DELETE, delete).unwrap(), 1);
        assert_eq!(conn.execute(SQL_DELETE, delete).unwrap(), 0);
        assert_eq!(query(SQL_FIND_FOR_USER, &[&"u1"]).len(), 2);
        // the same key of u2 stays
        assert_eq!(keys(query(SQL_FIND_FOR_USER, &[&"u2"])), expected_order());

        conn.execute(SQL_CONSENT_DELETE_BY_CLIENT, ["c1"]).unwrap();
        assert_eq!(
            query(SQL_FIND_FOR_USER, &[&"u1"]),
            vec![consent("c2", "", "openid", 3, 3)]
        );

        conn.execute("DELETE FROM users WHERE id = 'u1'", [])
            .unwrap();
        assert!(query(SQL_FIND_FOR_USER, &[&"u1"]).is_empty());
        assert_eq!(query(SQL_FIND_FOR_USER, &[&"u2"]).len(), 1);
    }

    /// All hiqlite migrations, in order.
    fn sqlite_full_schema() -> rusqlite::Connection {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../migrations/hiqlite");
        let mut files = std::fs::read_dir(dir)
            .unwrap()
            .map(|e| e.unwrap().path())
            .map(|p| {
                let name = p.file_name().unwrap().to_str().unwrap();
                let nr: u32 = name.split('_').next().unwrap().parse().unwrap();
                (nr, p)
            })
            .collect::<Vec<_>>();
        files.sort();

        let conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.pragma_update(None, "foreign_keys", "ON").unwrap();
        for (_, path) in files {
            conn.execute_batch(&std::fs::read_to_string(path).unwrap())
                .unwrap();
        }
        conn
    }

    /// The hiqlite transaction of `UserConsent::revoke` against the full schema, with
    /// `consent.enable`: the legacy refresh tokens of the user go as well.
    #[test]
    fn test_revoke_sqlite() {
        revoke_sqlite(
            true,
            4,
            &["rt_c2", "rt_u2_c1", "rt_u2_legacy"],
            &["j_c1", "j_d1", "j_legacy"],
        );
    }

    /// Without `consent.enable`, the legacy refresh tokens stay valid for the other clients.
    #[test]
    fn test_revoke_sqlite_keeps_legacy() {
        revoke_sqlite(
            false,
            3,
            &["rt_c2", "rt_legacy", "rt_u2_c1", "rt_u2_legacy"],
            &["j_c1", "j_d1"],
        );
    }

    /// Revokes the consent of u1 for c1 twice. `rt_deleted` is the number of refresh token rows of
    /// the first revoke, `rts_left` and `jtis_revoked` the state afterwards.
    fn revoke_sqlite(
        drop_legacy: bool,
        rt_deleted: usize,
        rts_left: &[&str],
        jtis_revoked: &[&str],
    ) {
        let mut conn = sqlite_full_schema();
        conn.execute_batch(
            r#"
CREATE TEMP TABLE tu AS SELECT * FROM users LIMIT 1;
UPDATE tu SET id = 'u1', email = 'u1@example.com';
INSERT INTO users SELECT * FROM tu;
UPDATE tu SET id = 'u2', email = 'u2@example.com';
INSERT INTO users SELECT * FROM tu;
CREATE TEMP TABLE tc AS SELECT * FROM clients LIMIT 1;
UPDATE tc SET id = 'c1';
INSERT INTO clients SELECT * FROM tc;
UPDATE tc SET id = 'c2';
INSERT INTO clients SELECT * FROM tc;
INSERT INTO devices (id, client_id, user_id, created, access_exp, peer_ip, name)
VALUES ('d1', 'c1', 'u1', 1, 2, '127.0.0.1', 'd1'),
       ('d2', 'c2', 'u1', 1, 2, '127.0.0.1', 'd2'),
       ('d3', 'c1', 'u2', 1, 2, '127.0.0.1', 'd3');
INSERT INTO refresh_tokens (id, user_id, nbf, exp, access_token_jti, client_id)
VALUES ('rt_c1', 'u1', 1, 2, 'j_c1', 'c1'),
       ('rt_legacy', 'u1', 1, 2, 'j_legacy', NULL),
       ('rt_no_jti', 'u1', 1, 2, NULL, 'c1'),
       ('rt_c2', 'u1', 1, 2, 'j_c2', 'c2'),
       ('rt_u2_legacy', 'u2', 1, 2, 'j_u2_legacy', NULL),
       ('rt_u2_c1', 'u2', 1, 2, 'j_u2_c1', 'c1');
INSERT INTO refresh_tokens_devices (id, device_id, user_id, nbf, exp, access_token_jti)
VALUES ('rtd_d1', 'd1', 'u1', 1, 2, 'j_d1'),
       ('rtd_d2', 'd2', 'u1', 1, 2, 'j_d2'),
       ('rtd_u2_d3', 'd3', 'u2', 1, 2, 'j_u2_d3');
INSERT INTO issued_tokens (jti, user_id, exp)
VALUES ('j_c1', 'u1', 2), ('j_legacy', 'u1', 2), ('j_c2', 'u1', 2),
       ('j_u2_legacy', 'u2', 2), ('j_u2_c1', 'u2', 2), ('j_d1', 'u1', 2),
       ('j_d2', 'u1', 2), ('j_other', 'u1', 2), ('j_u2_d3', 'u2', 2);
"#,
        )
        .unwrap();
        let params = rusqlite::params!["u1", "c1", "", "openid", "hash", 1, "127.0.0.1"];
        conn.execute(SQL_UPSERT, params).unwrap();
        let params = rusqlite::params!["u1", "c2", "", "openid", "hash", 1, "127.0.0.1"];
        conn.execute(SQL_UPSERT, params).unwrap();

        assert_eq!(
            hql_revoke_on(&mut conn, drop_legacy, "u1", "c1", ""),
            ConsentRevoked {
                consent_deleted: true,
                refresh_tokens_deleted: rt_deleted,
            }
        );
        assert_eq!(
            hql_revoke_on(&mut conn, drop_legacy, "u1", "c1", ""),
            ConsentRevoked {
                consent_deleted: false,
                refresh_tokens_deleted: 0,
            }
        );

        let ids = |sql: &str| -> Vec<String> {
            let mut stmt = conn.prepare(sql).unwrap();
            stmt.query_map([], |r| r.get(0))
                .unwrap()
                .map(|r| r.unwrap())
                .collect()
        };
        assert_eq!(ids("SELECT id FROM refresh_tokens ORDER BY id"), rts_left);
        // the device of u2 for c1 stays
        assert_eq!(
            ids("SELECT id FROM refresh_tokens_devices ORDER BY id"),
            vec!["rtd_d2", "rtd_u2_d3"]
        );
        assert_eq!(
            ids("SELECT jti FROM issued_tokens WHERE revoked ORDER BY jti"),
            jtis_revoked
        );
        assert_eq!(
            ids("SELECT client_id FROM user_consents WHERE user_id = 'u1'"),
            vec!["c2"]
        );
    }

    /// The hiqlite transaction of `UserConsent::revoke` on `conn`, with its result.
    fn hql_revoke_on(
        conn: &mut rusqlite::Connection,
        drop_legacy: bool,
        user_id: &str,
        client_id: &str,
        resource: &str,
    ) -> ConsentRevoked {
        let txn = conn.transaction().unwrap();
        let rows = hql_revoke(drop_legacy)
            .iter()
            .map(|(sql, params)| match params {
                RevokeParams::UserClient => txn.execute(sql, [user_id, client_id]),
                RevokeParams::UserClientResource => {
                    txn.execute(sql, [user_id, client_id, resource])
                }
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        txn.commit().unwrap();
        ConsentRevoked::from_hql_rows(rows)
    }

    /// Consents of u1 with and without a resource, listed by `UserConsent::find_for_user` and
    /// revoked one by one: a consent without refresh tokens is reported as deleted, and the list
    /// ends up empty. The consent of u2 stays.
    #[test]
    fn test_revoke_list_sqlite() {
        for drop_legacy in [false, true] {
            let mut conn = sqlite_full_schema();
            conn.execute_batch(
                r#"
CREATE TEMP TABLE tu AS SELECT * FROM users LIMIT 1;
UPDATE tu SET id = 'u1', email = 'u1@example.com';
INSERT INTO users SELECT * FROM tu;
UPDATE tu SET id = 'u2', email = 'u2@example.com';
INSERT INTO users SELECT * FROM tu;
INSERT INTO refresh_tokens (id, user_id, nbf, exp, access_token_jti, client_id)
VALUES ('rt_c1', 'u1', 1, 2, 'j_c1', 'c1');
"#,
            )
            .unwrap();
            for (user_id, client_id, resource, ts) in [
                ("u1", "c1", "", 1),
                ("u1", "c1", RES, 2),
                ("u1", "c2", "", 3),
                ("u2", "c1", "", 4),
            ] {
                let params =
                    rusqlite::params![user_id, client_id, resource, "openid", "h", ts, "127.0.0.1"];
                conn.execute(SQL_UPSERT, params).unwrap();
            }

            let list = |conn: &rusqlite::Connection, resource: Option<&str>| {
                let mut stmt;
                let rows = if let Some(resource) = resource {
                    stmt = conn.prepare(SQL_FIND_FOR_USER_RESOURCE).unwrap();
                    stmt.query(["u1", resource]).unwrap()
                } else {
                    stmt = conn.prepare(SQL_FIND_FOR_USER).unwrap();
                    stmt.query(["u1"]).unwrap()
                };
                serde_rusqlite::from_rows::<UserConsent>(rows)
                    .map(|r| {
                        let c = r.unwrap();
                        (c.client_id, c.resource)
                    })
                    .collect::<Vec<_>>()
            };
            let key = |c: &str, r: &str| (c.to_string(), r.to_string());
            assert_eq!(
                list(&conn, None),
                vec![key("c2", ""), key("c1", RES), key("c1", "")]
            );
            assert_eq!(list(&conn, Some(RES)), vec![key("c1", RES)]);
            assert_eq!(list(&conn, Some("")), vec![key("c2", ""), key("c1", "")]);

            let revoke = |conn: &mut rusqlite::Connection, c: &str, r: &str| {
                hql_revoke_on(conn, drop_legacy, "u1", c, r)
            };
            // with the refresh token of c1
            assert_eq!(
                revoke(&mut conn, "c1", RES),
                ConsentRevoked {
                    consent_deleted: true,
                    refresh_tokens_deleted: 1,
                }
            );
            // without refresh tokens
            for (c, r) in [("c1", ""), ("c2", "")] {
                assert_eq!(
                    revoke(&mut conn, c, r),
                    ConsentRevoked {
                        consent_deleted: true,
                        refresh_tokens_deleted: 0,
                    },
                    "{c} {r}"
                );
            }
            assert!(list(&conn, None).is_empty());
            assert!(list(&conn, Some(RES)).is_empty());
            let u2: Vec<String> = conn
                .prepare("SELECT client_id FROM user_consents WHERE user_id = 'u2'")
                .unwrap()
                .query_map([], |r| r.get(0))
                .unwrap()
                .map(|r| r.unwrap())
                .collect();
            assert_eq!(u2, vec!["c1"]);
        }
    }

    /// The production SQL against the Postgres migration.
    /// Run with `RAUTHY_TEST_PG_URL=postgres://.. cargo test -p rauthy-data -- --ignored`.
    #[tokio::test]
    #[ignore = "needs Postgres via RAUTHY_TEST_PG_URL"]
    async fn test_sql_postgres() {
        use deadpool_postgres::{Config, Runtime};
        use tokio_postgres::NoTls;

        let url = std::env::var("RAUTHY_TEST_PG_URL").expect("RAUTHY_TEST_PG_URL is not set");
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let schema = format!("uc_test_{}_{nanos}", std::process::id());
        let cfg = Config {
            url: Some(url),
            ..Default::default()
        };
        let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
        let cl = pool.get().await.unwrap();
        cl.batch_execute(&format!(
            r#"
CREATE SCHEMA {schema};
SET search_path TO {schema};
CREATE TABLE users (id VARCHAR NOT NULL PRIMARY KEY);
INSERT INTO users (id) VALUES ('u1'), ('u2');
{MIGRATION_PG}"#
        ))
        .await
        .unwrap();

        // u2 holds rows with the same (client_id, resource) keys
        for user_id in ["u1", "u2"] {
            for (client_id, resource, scopes, ts) in UPSERTS {
                let hash = format!("hash_{ts}");
                let params: [&(dyn ToSql + Sync); 7] = [
                    &user_id,
                    &client_id,
                    &resource,
                    &scopes,
                    &hash,
                    &ts,
                    &"127.0.0.1",
                ];
                cl.execute(SQL_UPSERT, &params).await.unwrap();
            }
        }

        let cl = &cl;
        let query = async |sql: &str, params: &[&(dyn ToSql + Sync)]| -> Vec<UserConsent> {
            let rows = cl.query(sql, params).await.unwrap();
            rows.into_iter().map(UserConsent::from).collect()
        };

        assert_eq!(
            query(SQL_FIND, &[&"u1", &"c1", &""]).await,
            vec![consent("c1", "", "openid,email", 1, 4)]
        );
        assert_eq!(query(SQL_FIND, &[&"u2", &"c1", &""]).await[0].user_id, "u2");
        assert!(query(SQL_FIND, &[&"u3", &"c1", &""]).await.is_empty());
        let keys = |v: Vec<UserConsent>| -> Vec<(String, String)> {
            v.into_iter().map(|c| (c.client_id, c.resource)).collect()
        };
        // the rows of u2 are left out
        assert_eq!(
            keys(query(SQL_FIND_FOR_USER, &[&"u1"]).await),
            expected_order()
        );
        assert_eq!(
            query(SQL_FIND_FOR_USER_RESOURCE, &[&"u1", &RES]).await,
            vec![consent("c1", RES, "openid", 2, 2)]
        );
        let for_resource = query(SQL_FIND_FOR_USER_RESOURCE, &[&"u1", &""]).await;
        assert_eq!(for_resource.len(), 2);
        assert!(for_resource.iter().all(|c| c.user_id == "u1"));

        let delete: [&(dyn ToSql + Sync); 3] = [&"u1", &"c1", &""];
        assert_eq!(cl.execute(SQL_DELETE, &delete).await.unwrap(), 1);
        assert_eq!(cl.execute(SQL_DELETE, &delete).await.unwrap(), 0);
        // the same key of u2 stays
        assert_eq!(
            keys(query(SQL_FIND_FOR_USER, &[&"u2"]).await),
            expected_order()
        );

        cl.execute(SQL_CONSENT_DELETE_BY_CLIENT, &[&"c1"])
            .await
            .unwrap();
        assert_eq!(
            query(SQL_FIND_FOR_USER, &[&"u1"]).await,
            vec![consent("c2", "", "openid", 3, 3)]
        );

        cl.execute("DELETE FROM users WHERE id = 'u1'", &[])
            .await
            .unwrap();
        assert!(query(SQL_FIND_FOR_USER, &[&"u1"]).await.is_empty());
        assert_eq!(query(SQL_FIND_FOR_USER, &[&"u2"]).await.len(), 1);

        cl.batch_execute(&format!("DROP SCHEMA {schema} CASCADE"))
            .await
            .unwrap();
    }

    /// A schema with all Postgres migrations, u1 with a grant for c1, the device d1 of c1 with
    /// the device refresh token `rtd_src`, and a legacy refresh token of u1. u2 has the device d3
    /// of c1 with the device refresh token `rtd_u2_d3`.
    async fn pg_race_setup() -> (deadpool_postgres::Pool, String) {
        use deadpool_postgres::{Config, Runtime};
        use tokio_postgres::NoTls;

        let url = std::env::var("RAUTHY_TEST_PG_URL").expect("RAUTHY_TEST_PG_URL is not set");
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let schema = format!("uc_race_{}_{nanos}", std::process::id());
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

        let cl = pool.get().await.unwrap();
        cl.batch_execute(&format!(
            "CREATE SCHEMA {schema}; SET search_path TO {schema};"
        ))
        .await
        .unwrap();
        for (_, path) in files {
            cl.batch_execute(&std::fs::read_to_string(path).unwrap())
                .await
                .unwrap();
        }
        cl.batch_execute(
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
       ('d3', 'c1', 'u2', 1, 2, '127.0.0.1', 'd3');
INSERT INTO refresh_tokens_devices (id, device_id, user_id, nbf, exp, access_token_jti)
VALUES ('rtd_src', 'd1', 'u1', 1, 2, 'j_src'),
       ('rtd_u2_d3', 'd3', 'u2', 1, 2, 'j_u2_d3');
INSERT INTO refresh_tokens (id, user_id, nbf, exp, access_token_jti, client_id)
VALUES ('rt_legacy', 'u1', 1, 2, 'j_legacy', NULL);
INSERT INTO issued_tokens (jti, user_id, exp)
VALUES ('j_src', 'u1', 2), ('j_new', 'u1', 2), ('j_legacy', 'u1', 2), ('j_u2_d3', 'u2', 2);
INSERT INTO user_consents
(user_id, client_id, resource, scopes, content_hash, created, updated, ip)
VALUES ('u1', 'c1', '', 'openid', 'hash', 1, 1, '127.0.0.1');
"#,
        )
        .await
        .unwrap();
        (pool, schema)
    }

    async fn pg_conn(pool: &deadpool_postgres::Pool, schema: &str) -> deadpool_postgres::Object {
        let cl = pool.get().await.unwrap();
        cl.batch_execute(&format!("SET search_path TO {schema}"))
            .await
            .unwrap();
        cl
    }

    /// Waits until `task` finished, or the backend `pid` waits for a lock.
    async fn pg_wait_blocked_or_done<T>(
        monitor: &deadpool_postgres::Object,
        pid: i32,
        task: &tokio::task::JoinHandle<T>,
    ) {
        for _ in 0..500 {
            if task.is_finished() {
                return;
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
                return;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
        panic!("the concurrent statement neither finished nor blocked");
    }

    /// `RefreshToken::discard` on `cl`: the row deleted and its access token revoked in one
    /// transaction
    async fn pg_discard(cl: &mut deadpool_postgres::Object, id: &str) -> Result<(), ErrorResponse> {
        crate::entity::refresh_tokens::pg_discard_txn(
            cl,
            &crate::entity::refresh_tokens::SQL_RT_DISCARD,
            id,
        )
        .await
    }

    /// Runs the revoke of u1 for c1 in `txn` up to and including the `refresh_tokens` delete of
    /// the client, then `concurrent`, then the rest and the commit.
    async fn pg_revoke_around<F, Fut>(
        txn: deadpool_postgres::Transaction<'_>,
        drop_legacy: bool,
        concurrent: F,
    ) where
        F: FnOnce() -> Fut,
        Fut: Future<Output = ()>,
    {
        let steps = pg_revoke_steps(drop_legacy);
        let split = steps
            .iter()
            .position(|s| *s == PgRevokeStep::RefreshTokens)
            .unwrap()
            + 1;
        let mut revoke = PgRevoke::default();
        for step in &steps[..split] {
            revoke.step(&txn, *step, "u1", "c1", "").await.unwrap();
        }
        concurrent().await;
        for step in &steps[split..] {
            revoke.step(&txn, *step, "u1", "c1", "").await.unwrap();
        }
        revoke.revoke_access_tokens(&txn).await.unwrap();
        txn.commit().await.unwrap();
    }

    async fn pg_ids(cl: &deadpool_postgres::Object, sql: &str) -> Vec<String> {
        cl.query(sql, &[])
            .await
            .unwrap()
            .iter()
            .map(|r| r.get(0))
            .collect()
    }

    const INSERT_RT_NEW: &str = r#"
INSERT INTO refresh_tokens (id, user_id, nbf, exp, access_token_jti, client_id)
VALUES ('rt_new', 'u1', 1, 2, 'j_new', 'c1')"#;

    /// A device refresh of c1 runs while the consent is revoked: after the revoke deleted the
    /// `refresh_tokens` of c1, the refresh inserts its replacement there and deletes its source
    /// from `refresh_tokens_devices` (`RefreshTokenDevice::finish_rotation`), then the revoke
    /// commits. The replacement must not survive the revoke.
    /// Run with `RAUTHY_TEST_PG_URL=postgres://.. cargo test -p rauthy-data -- --ignored`.
    #[tokio::test]
    #[ignore = "needs Postgres via RAUTHY_TEST_PG_URL"]
    async fn test_revoke_device_refresh_race_postgres() {
        pg_rotation_race("DELETE FROM refresh_tokens_devices WHERE id = 'rtd_src'").await;
    }

    /// [`test_revoke_device_refresh_race_postgres`] for the refresh of a legacy refresh token
    /// (`client_id` NULL) with `consent.enable`. Its replacement is issued for c1, and it deletes
    /// its source from `refresh_tokens` (`RefreshToken::finish_rotation`).
    /// Run with `RAUTHY_TEST_PG_URL=postgres://.. cargo test -p rauthy-data -- --ignored`.
    #[tokio::test]
    #[ignore = "needs Postgres via RAUTHY_TEST_PG_URL"]
    async fn test_revoke_legacy_refresh_race_postgres() {
        pg_rotation_race("DELETE FROM refresh_tokens WHERE id = 'rt_legacy'").await;
    }

    /// A refresh token rotation into `rt_new` for c1, whose checked delete of the source is
    /// `source_delete`, runs after the revoke (with `consent.enable`) deleted the `refresh_tokens`
    /// of c1, and before it commits.
    async fn pg_rotation_race(source_delete: &'static str) {
        let (pool, schema) = pg_race_setup().await;
        let mut revoker = pg_conn(&pool, &schema).await;
        let refresher = pg_conn(&pool, &schema).await;
        let monitor = pg_conn(&pool, &schema).await;
        let pid: i32 = refresher
            .query_one("SELECT pg_backend_pid()", &[])
            .await
            .unwrap()
            .get(0);

        let mut rotation = None;
        let txn = revoker.transaction().await.unwrap();
        pg_revoke_around(txn, true, async || {
            // `TokenSet::from_user` inserts the replacement
            refresher.execute(INSERT_RT_NEW, &[]).await.unwrap();
            // `delete_checked` of the source
            let delete = tokio::spawn(async move {
                let rows = refresher.execute(source_delete, &[]).await.unwrap();
                (refresher, rows)
            });
            pg_wait_blocked_or_done(&monitor, pid, &delete).await;
            rotation = Some(delete);
        })
        .await;
        let (mut refresher, rows) = rotation.unwrap().await.unwrap();

        // `finish_rotation`
        let delete_checked = async {
            if rows > 0 {
                Ok(())
            } else {
                Err(ErrorResponse::new(
                    rauthy_error::ErrorResponseType::NotFound,
                    "Invalid Refresh Token",
                ))
            }
        };
        let res = crate::entity::refresh_tokens::finish_rotation_with(delete_checked, || {
            pg_discard(&mut refresher, "rt_new")
        })
        .await;

        assert!(res.is_err(), "the rotation must fail");
        assert!(
            pg_ids(&monitor, "SELECT id FROM refresh_tokens")
                .await
                .is_empty()
        );
        // the device of u2 for c1 stays
        assert_eq!(
            pg_ids(&monitor, "SELECT id FROM refresh_tokens_devices").await,
            vec!["rtd_u2_d3"]
        );
        assert_eq!(
            pg_ids(
                &monitor,
                "SELECT jti FROM issued_tokens WHERE revoked ORDER BY jti"
            )
            .await,
            vec!["j_legacy", "j_new", "j_src"]
        );

        monitor
            .batch_execute(&format!("DROP SCHEMA {schema} CASCADE"))
            .await
            .unwrap();
    }

    /// A code exchange of c1 runs while the consent is revoked: after the revoke deleted the
    /// `refresh_tokens` of c1, the exchange inserts its refresh token and the fence re-checks
    /// the grant with [`SQL_PG_EXISTS_FENCED`], then the revoke commits. The new refresh token
    /// must not survive the revoke.
    /// Run with `RAUTHY_TEST_PG_URL=postgres://.. cargo test -p rauthy-data -- --ignored`.
    #[tokio::test]
    #[ignore = "needs Postgres via RAUTHY_TEST_PG_URL"]
    async fn test_revoke_code_exchange_race_postgres() {
        let (pool, schema) = pg_race_setup().await;
        let mut revoker = pg_conn(&pool, &schema).await;
        let exchange = pg_conn(&pool, &schema).await;
        let monitor = pg_conn(&pool, &schema).await;
        let pid: i32 = exchange
            .query_one("SELECT pg_backend_pid()", &[])
            .await
            .unwrap()
            .get(0);

        let mut fence = None;
        let txn = revoker.transaction().await.unwrap();
        pg_revoke_around(txn, false, async || {
            // `TokenSet::from_user`
            exchange.execute(INSERT_RT_NEW, &[]).await.unwrap();
            // `fence_code_exchange`
            let check = tokio::spawn(async move {
                let granted = exchange
                    .query_opt(SQL_PG_EXISTS_FENCED, &[&"u1", &"c1", &""])
                    .await
                    .unwrap()
                    .is_some();
                (exchange, granted)
            });
            pg_wait_blocked_or_done(&monitor, pid, &check).await;
            fence = Some(check);
        })
        .await;
        let (mut exchange, granted) = fence.unwrap().await.unwrap();
        // `discard_unless_granted`
        if !granted {
            pg_discard(&mut exchange, "rt_new").await.unwrap();
        }

        assert!(!granted, "the fence must not pass");
        // without `consent.enable`, the legacy refresh token stays
        assert_eq!(
            pg_ids(&monitor, "SELECT id FROM refresh_tokens").await,
            vec!["rt_legacy"]
        );
        assert_eq!(
            pg_ids(
                &monitor,
                "SELECT jti FROM issued_tokens WHERE revoked ORDER BY jti"
            )
            .await,
            vec!["j_new", "j_src"]
        );

        monitor
            .batch_execute(&format!("DROP SCHEMA {schema} CASCADE"))
            .await
            .unwrap();
    }

    /// A refresh of `rt_src`, a refresh token of c1 itself, runs while the consent is revoked
    /// (with `consent.enable`). The statement of the revoke that takes the `refresh_tokens` of
    /// c1 has taken its snapshot and still waits for `rt_block`, which a third connection holds,
    /// when the refresh inserts its replacement and deletes `rt_src`, which that statement has
    /// not reached yet. The replacement must not survive the revoke. The access token of
    /// `rt_src` stays, as after a rotation that finished before the revoke.
    /// Run with `RAUTHY_TEST_PG_URL=postgres://.. cargo test -p rauthy-data -- --ignored`.
    #[tokio::test]
    #[ignore = "needs Postgres via RAUTHY_TEST_PG_URL"]
    async fn test_revoke_same_client_refresh_race_postgres() {
        let (pool, schema) = pg_race_setup().await;
        let mut revoker = pg_conn(&pool, &schema).await;
        let mut holder = pg_conn(&pool, &schema).await;
        let refresher = pg_conn(&pool, &schema).await;
        let monitor = pg_conn(&pool, &schema).await;
        // `rt_block` first, so that the revoke reaches it before `rt_src`
        monitor
            .batch_execute(
                r#"
INSERT INTO refresh_tokens (id, user_id, nbf, exp, access_token_jti, client_id)
VALUES ('rt_block', 'u1', 1, 2, 'j_block', 'c1');
INSERT INTO refresh_tokens (id, user_id, nbf, exp, access_token_jti, client_id)
VALUES ('rt_src', 'u1', 1, 2, 'j_rt_src', 'c1');
INSERT INTO issued_tokens (jti, user_id, exp) VALUES ('j_block', 'u1', 2), ('j_rt_src', 'u1', 2);
"#,
            )
            .await
            .unwrap();
        let pid = async |cl: &deadpool_postgres::Object| -> i32 {
            cl.query_one("SELECT pg_backend_pid()", &[])
                .await
                .unwrap()
                .get(0)
        };
        let revoker_pid = pid(&revoker).await;
        let refresher_pid = pid(&refresher).await;

        let hold = holder.transaction().await.unwrap();
        hold.execute(
            "SELECT 1 FROM refresh_tokens WHERE id = 'rt_block' FOR UPDATE",
            &[],
        )
        .await
        .unwrap();
        let revoke = tokio::spawn(async move {
            pg_revoke(&mut revoker, true, "u1", "c1", "")
                .await
                .map(|_| ())
        });
        pg_wait_blocked_or_done(&monitor, revoker_pid, &revoke).await;
        assert!(!revoke.is_finished(), "the revoke must wait for rt_block");

        // `TokenSet::from_user` inserts the replacement
        refresher.execute(INSERT_RT_NEW, &[]).await.unwrap();
        // `delete_checked` of the source
        let delete = tokio::spawn(async move {
            let rows = refresher
                .execute("DELETE FROM refresh_tokens WHERE id = 'rt_src'", &[])
                .await
                .unwrap();
            (refresher, rows)
        });
        pg_wait_blocked_or_done(&monitor, refresher_pid, &delete).await;
        hold.commit().await.unwrap();
        revoke.await.unwrap().unwrap();
        let (mut refresher, rows) = delete.await.unwrap();

        // `finish_rotation`
        let delete_checked = async {
            if rows > 0 {
                Ok(())
            } else {
                Err(ErrorResponse::new(
                    rauthy_error::ErrorResponseType::NotFound,
                    "Invalid Refresh Token",
                ))
            }
        };
        let _ = crate::entity::refresh_tokens::finish_rotation_with(delete_checked, || {
            pg_discard(&mut refresher, "rt_new")
        })
        .await;

        assert_eq!(
            pg_ids(&monitor, "SELECT id FROM refresh_tokens").await,
            Vec::<String>::new()
        );
        assert_eq!(
            pg_ids(
                &monitor,
                "SELECT jti FROM issued_tokens WHERE revoked ORDER BY jti"
            )
            .await,
            vec!["j_block", "j_legacy", "j_new", "j_src"]
        );

        monitor
            .batch_execute(&format!("DROP SCHEMA {schema} CASCADE"))
            .await
            .unwrap();
    }

    /// [`test_revoke_list_sqlite`] on Postgres, with the device of u2 for c1 (`rtd_u2_d3`)
    /// left alone.
    /// Run with `RAUTHY_TEST_PG_URL=postgres://.. cargo test -p rauthy-data -- --ignored`.
    #[tokio::test]
    #[ignore = "needs Postgres via RAUTHY_TEST_PG_URL"]
    async fn test_revoke_list_postgres() {
        for drop_legacy in [false, true] {
            let (pool, schema) = pg_race_setup().await;
            let mut cl = pg_conn(&pool, &schema).await;
            cl.batch_execute(&format!(
                r#"
DELETE FROM user_consents;
INSERT INTO refresh_tokens (id, user_id, nbf, exp, access_token_jti, client_id)
VALUES ('rt_c1', 'u1', 1, 2, 'j_c1', 'c1');
INSERT INTO issued_tokens (jti, user_id, exp) VALUES ('j_c1', 'u1', 2);
INSERT INTO user_consents
(user_id, client_id, resource, scopes, content_hash, created, updated, ip)
VALUES ('u1', 'c1', '', 'openid', 'h', 1, 1, '127.0.0.1'),
       ('u1', 'c1', '{RES}', 'openid', 'h', 2, 2, '127.0.0.1'),
       ('u1', 'c2', '', 'openid', 'h', 3, 3, '127.0.0.1'),
       ('u2', 'c1', '', 'openid', 'h', 4, 4, '127.0.0.1');
"#
            ))
            .await
            .unwrap();

            let list = async |cl: &deadpool_postgres::Object, resource: Option<&str>| {
                let rows = if let Some(resource) = resource {
                    cl.query(SQL_FIND_FOR_USER_RESOURCE, &[&"u1", &resource])
                        .await
                } else {
                    cl.query(SQL_FIND_FOR_USER, &[&"u1"]).await
                };
                rows.unwrap()
                    .into_iter()
                    .map(UserConsent::from)
                    .map(|c| (c.client_id, c.resource))
                    .collect::<Vec<_>>()
            };
            let key = |c: &str, r: &str| (c.to_string(), r.to_string());
            assert_eq!(
                list(&cl, None).await,
                vec![key("c2", ""), key("c1", RES), key("c1", "")]
            );
            assert_eq!(list(&cl, Some(RES)).await, vec![key("c1", RES)]);
            assert_eq!(
                list(&cl, Some("")).await,
                vec![key("c2", ""), key("c1", "")]
            );

            // rt_c1 and rtd_src, and rt_legacy with `drop_legacy`
            assert_eq!(
                pg_revoke(&mut cl, drop_legacy, "u1", "c1", RES)
                    .await
                    .unwrap(),
                ConsentRevoked {
                    consent_deleted: true,
                    refresh_tokens_deleted: if drop_legacy { 3 } else { 2 },
                }
            );
            // without refresh tokens
            for (c, r) in [("c1", ""), ("c2", "")] {
                assert_eq!(
                    pg_revoke(&mut cl, drop_legacy, "u1", c, r).await.unwrap(),
                    ConsentRevoked {
                        consent_deleted: true,
                        refresh_tokens_deleted: 0,
                    },
                    "{c} {r}"
                );
            }
            assert!(list(&cl, None).await.is_empty());
            assert!(list(&cl, Some(RES)).await.is_empty());
            assert_eq!(
                pg_ids(
                    &cl,
                    "SELECT client_id FROM user_consents WHERE user_id = 'u2'"
                )
                .await,
                vec!["c1"]
            );
            // the device of u2 for c1 stays, its access token is not revoked
            assert_eq!(
                pg_ids(&cl, "SELECT id FROM refresh_tokens_devices").await,
                vec!["rtd_u2_d3"]
            );
            let mut revoked = vec!["j_c1", "j_src"];
            if drop_legacy {
                revoked.insert(1, "j_legacy");
            }
            assert_eq!(
                pg_ids(
                    &cl,
                    "SELECT jti FROM issued_tokens WHERE revoked ORDER BY jti"
                )
                .await,
                revoked
            );

            cl.batch_execute(&format!("DROP SCHEMA {schema} CASCADE"))
                .await
                .unwrap();
        }
    }

    /// A transaction that takes the rows of u1 for c1 in [`test_user_delete_race_postgres`]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum PgTxnSide {
        /// `UserConsent::revoke` of u1 for c1, with `consent.enable`, the access tokens last
        Revoke,
        /// `Client::delete` of c1, with `consent.enable`
        ClientDelete,
        /// `User::delete` of u1
        UserDelete,
    }

    /// The state of the transaction of a [`PgTxnSide`] between its statements
    #[derive(Debug, Default)]
    struct PgSideState {
        revoke: PgRevoke,
        client_delete: crate::entity::refresh_tokens::PgDeletedJtis,
        user_delete: crate::entity::users::PgDelete,
    }

    impl PgTxnSide {
        fn len(self) -> usize {
            match self {
                Self::Revoke => pg_revoke_steps(true).len() + 1,
                Self::ClientDelete => pg_client_delete_len(),
                Self::UserDelete => crate::entity::users::PG_DELETE_STEPS.len(),
            }
        }

        /// Runs the statements `range` of the side in `txn`.
        async fn run(
            self,
            txn: &deadpool_postgres::Transaction<'_>,
            range: std::ops::Range<usize>,
            state: &mut PgSideState,
        ) -> Result<(), ErrorResponse> {
            for i in range {
                match self {
                    Self::Revoke => {
                        let steps = pg_revoke_steps(true);
                        if let Some(step) = steps.get(i) {
                            state.revoke.step(txn, *step, "u1", "c1", "").await?;
                        } else {
                            state.revoke.revoke_access_tokens(txn).await?;
                        }
                    }
                    Self::ClientDelete => {
                        pg_client_delete(txn, i..i + 1, &mut state.client_delete).await?;
                    }
                    Self::UserDelete => {
                        let step = crate::entity::users::PG_DELETE_STEPS[i];
                        state.user_delete.step(txn, step, "u1").await?;
                    }
                }
            }
            Ok(())
        }
    }

    /// `User::delete` of u1 runs at the same time as a consent revoke of u1 for c1, or as a
    /// `Client::delete` of c1: one of them runs its first statements, the other one all of its
    /// statements, then the first one the rest. They take the tables they share in the same
    /// order, so they are serialized and Postgres aborts neither as a deadlock, at every split.
    /// The rows of u1 both take first are dropped one by one (the consent, the device refresh
    /// token, the legacy refresh token), so that each later table is where they meet first once.
    /// Run with `RAUTHY_TEST_PG_URL=postgres://.. cargo test -p rauthy-data -- --ignored`.
    #[tokio::test]
    #[ignore = "needs Postgres via RAUTHY_TEST_PG_URL"]
    async fn test_user_delete_race_postgres() {
        for other in [PgTxnSide::Revoke, PgTxnSide::ClientDelete] {
            for (first, second) in [
                (PgTxnSide::UserDelete, other),
                (other, PgTxnSide::UserDelete),
            ] {
                for dropped in 0..=USER_DELETE_RACE_DROPS.len() {
                    for split in 1..=first.len() {
                        pg_user_delete_race(first, second, dropped, split).await;
                    }
                }
            }
        }
    }

    /// The rows of u1 that [`test_user_delete_race_postgres`] drops, in order
    const USER_DELETE_RACE_DROPS: [&str; 3] = [
        "DELETE FROM user_consents WHERE user_id = 'u1'",
        "DELETE FROM refresh_tokens_devices WHERE id = 'rtd_src'",
        "DELETE FROM refresh_tokens WHERE id = 'rt_legacy'",
    ];

    async fn pg_user_delete_race(
        first: PgTxnSide,
        second: PgTxnSide,
        dropped: usize,
        split: usize,
    ) {
        let case =
            format!("first: {first:?}, second: {second:?}, dropped: {dropped}, split: {split}");
        let (pool, schema) = pg_race_setup().await;
        let mut first_cl = pg_conn(&pool, &schema).await;
        let mut second_cl = pg_conn(&pool, &schema).await;
        let monitor = pg_conn(&pool, &schema).await;
        // `rt_legacy` after `rt_c1`, so that a single delete of all refresh tokens of u1 takes
        // `rt_c1` first, in the opposite order of the revoke and `Client::delete`
        monitor
            .batch_execute(
                r#"
DELETE FROM refresh_tokens WHERE id = 'rt_legacy';
INSERT INTO refresh_tokens (id, user_id, nbf, exp, access_token_jti, client_id)
VALUES ('rt_c1', 'u1', 1, 2, 'j_c1', 'c1');
INSERT INTO refresh_tokens (id, user_id, nbf, exp, access_token_jti, client_id)
VALUES ('rt_legacy', 'u1', 1, 2, 'j_legacy', NULL);
INSERT INTO issued_tokens (jti, user_id, exp) VALUES ('j_c1', 'u1', 2);
"#,
            )
            .await
            .unwrap();
        for sql in &USER_DELETE_RACE_DROPS[..dropped] {
            monitor.batch_execute(sql).await.unwrap();
        }
        let pid: i32 = second_cl
            .query_one("SELECT pg_backend_pid()", &[])
            .await
            .unwrap()
            .get(0);

        let txn = first_cl
            .build_transaction()
            .isolation_level(IsolationLevel::ReadCommitted)
            .start()
            .await
            .unwrap();
        let mut revoke = PgSideState::default();
        first.run(&txn, 0..split, &mut revoke).await.unwrap();
        let task = tokio::spawn(async move {
            let txn = second_cl
                .build_transaction()
                .isolation_level(IsolationLevel::ReadCommitted)
                .start()
                .await?;
            let mut revoke = PgSideState::default();
            second.run(&txn, 0..second.len(), &mut revoke).await?;
            txn.commit().await.map_err(ErrorResponse::from)
        });
        pg_wait_blocked_or_done(&monitor, pid, &task).await;
        let res_first = match first.run(&txn, split..first.len(), &mut revoke).await {
            Ok(()) => txn.commit().await.map_err(ErrorResponse::from),
            Err(err) => Err(err),
        };
        let res_second = task.await.unwrap();

        assert!(res_first.is_ok(), "{case}: {res_first:?}");
        assert!(res_second.is_ok(), "{case}: {res_second:?}");
        for sql in [
            "SELECT id FROM users WHERE id = 'u1'",
            "SELECT id FROM refresh_tokens WHERE user_id = 'u1'",
            "SELECT id FROM refresh_tokens_devices WHERE user_id = 'u1'",
            "SELECT client_id FROM user_consents WHERE user_id = 'u1'",
        ] {
            assert!(pg_ids(&monitor, sql).await.is_empty(), "{case}: {sql}");
        }
        // `User::delete` revokes every access token of u1, those of the dropped refresh tokens
        // included
        let mut revoked = vec!["j_c1", "j_legacy", "j_new", "j_src"];
        // `Client::delete` also deletes the device refresh token of u2 for c1
        if first == PgTxnSide::ClientDelete || second == PgTxnSide::ClientDelete {
            revoked.push("j_u2_d3");
        }
        assert_eq!(
            pg_ids(
                &monitor,
                "SELECT jti FROM issued_tokens WHERE revoked ORDER BY jti"
            )
            .await,
            revoked,
            "{case}"
        );

        monitor
            .batch_execute(&format!("DROP SCHEMA {schema} CASCADE"))
            .await
            .unwrap();
    }

    /// The number of statements of the Postgres transaction of `Client::delete` with
    /// `consent.enable`: `pg_delete_statements`, then the revoke of the access tokens
    fn pg_client_delete_len() -> usize {
        crate::entity::clients::pg_delete_statements(true).len() + 1
    }

    /// Runs the statements `range` of `Client::delete` of c1 (with `consent.enable`) in `txn`.
    async fn pg_client_delete(
        txn: &deadpool_postgres::Transaction<'_>,
        range: std::ops::Range<usize>,
        jtis: &mut crate::entity::refresh_tokens::PgDeletedJtis,
    ) -> Result<(), ErrorResponse> {
        let stmts = crate::entity::clients::pg_delete_statements(true);
        for i in range {
            match stmts.get(i) {
                Some((sql, true)) => jtis.delete(txn, sql, &[&"c1"]).await?,
                Some((sql, false)) => jtis.delete(txn, sql, &[]).await?,
                None => jtis.revoke(txn).await?,
            }
        }
        Ok(())
    }

    /// A consent revoke of u1 for c1 (with `consent.enable`) and a `Client::delete` of c1 run at
    /// the same time: one of them runs its first statements, the other one all of its
    /// statements, then the first one the rest. Both take their locks in the same order, so
    /// they are serialized and Postgres aborts neither as a deadlock, at every split. The rows
    /// both lock first are dropped one by one (the consent, then the device refresh token of u1),
    /// so that each later table is where they meet first once.
    /// Run with `RAUTHY_TEST_PG_URL=postgres://.. cargo test -p rauthy-data -- --ignored`.
    #[tokio::test]
    #[ignore = "needs Postgres via RAUTHY_TEST_PG_URL"]
    async fn test_revoke_client_delete_race_postgres() {
        let delete_stmts = pg_client_delete_len();
        let revoke_steps = pg_revoke_steps(true).len();
        for (with_consent, with_device) in [(true, true), (false, true), (false, false)] {
            for split in 1..=delete_stmts {
                pg_revoke_client_delete_race(with_consent, with_device, true, split).await;
            }
            for split in 1..=revoke_steps {
                pg_revoke_client_delete_race(with_consent, with_device, false, split).await;
            }
        }
    }

    async fn pg_revoke_client_delete_race(
        with_consent: bool,
        with_device: bool,
        delete_first: bool,
        split: usize,
    ) {
        let case = format!(
            "with_consent: {with_consent}, with_device: {with_device}, \
            delete_first: {delete_first}, split: {split}"
        );
        let (pool, schema) = pg_race_setup().await;
        let mut first = pg_conn(&pool, &schema).await;
        let mut second = pg_conn(&pool, &schema).await;
        let monitor = pg_conn(&pool, &schema).await;
        first
            .batch_execute(
                r#"
INSERT INTO refresh_tokens (id, user_id, nbf, exp, access_token_jti, client_id)
VALUES ('rt_c1', 'u1', 1, 2, 'j_c1', 'c1');
INSERT INTO issued_tokens (jti, user_id, exp) VALUES ('j_c1', 'u1', 2);
"#,
            )
            .await
            .unwrap();
        if !with_consent {
            first
                .batch_execute("DELETE FROM user_consents")
                .await
                .unwrap();
        }
        if !with_device {
            first
                .batch_execute("DELETE FROM refresh_tokens_devices WHERE id = 'rtd_src'")
                .await
                .unwrap();
        }
        let pid: i32 = second
            .query_one("SELECT pg_backend_pid()", &[])
            .await
            .unwrap()
            .get(0);
        let delete_len = pg_client_delete_len();

        let txn = first.transaction().await.unwrap();
        let (res_first, res_second) = if delete_first {
            let mut jtis = crate::entity::refresh_tokens::PgDeletedJtis::default();
            pg_client_delete(&txn, 0..split, &mut jtis).await.unwrap();
            let revoke = tokio::spawn(async move {
                pg_revoke(&mut second, true, "u1", "c1", "")
                    .await
                    .map(|_| ())
            });
            pg_wait_blocked_or_done(&monitor, pid, &revoke).await;
            let res = match pg_client_delete(&txn, split..delete_len, &mut jtis).await {
                Ok(()) => txn.commit().await.map_err(ErrorResponse::from),
                Err(err) => Err(err),
            };
            (res, revoke.await.unwrap())
        } else {
            let revoke_steps = pg_revoke_steps(true);
            let mut revoke = PgRevoke::default();
            for step in &revoke_steps[..split] {
                revoke.step(&txn, *step, "u1", "c1", "").await.unwrap();
            }
            let delete = tokio::spawn(async move {
                let txn = second.transaction().await?;
                let mut jtis = crate::entity::refresh_tokens::PgDeletedJtis::default();
                pg_client_delete(&txn, 0..delete_len, &mut jtis).await?;
                txn.commit().await.map_err(ErrorResponse::from)
            });
            pg_wait_blocked_or_done(&monitor, pid, &delete).await;
            let mut res = Ok(());
            for step in &revoke_steps[split..] {
                res = revoke.step(&txn, *step, "u1", "c1", "").await;
                if res.is_err() {
                    break;
                }
            }
            if res.is_ok() {
                res = match revoke.revoke_access_tokens(&txn).await {
                    Ok(()) => txn.commit().await.map_err(ErrorResponse::from),
                    Err(err) => Err(err),
                };
            }
            (res, delete.await.unwrap())
        };

        assert!(res_first.is_ok(), "{case}: {res_first:?}");
        assert!(res_second.is_ok(), "{case}: {res_second:?}");
        for sql in [
            "SELECT id FROM refresh_tokens",
            "SELECT id FROM refresh_tokens_devices",
            "SELECT client_id FROM user_consents",
            "SELECT id FROM clients WHERE id = 'c1'",
        ] {
            assert!(pg_ids(&monitor, sql).await.is_empty(), "{case}: {sql}");
        }
        // whichever of them deleted a refresh token revoked its access token
        let mut revoked = vec!["j_c1", "j_legacy", "j_src", "j_u2_d3"];
        if !with_device {
            revoked.retain(|jti| *jti != "j_src");
        }
        assert_eq!(
            pg_ids(
                &monitor,
                "SELECT jti FROM issued_tokens WHERE revoked ORDER BY jti"
            )
            .await,
            revoked,
            "{case}"
        );

        monitor
            .batch_execute(&format!("DROP SCHEMA {schema} CASCADE"))
            .await
            .unwrap();
    }

    /// The refresh tokens `RefreshToken::discard` and `RefreshTokenDevice::discard` drop in
    /// [`test_discard_revoke_race_postgres`]: the replacement `rt_new` for c1, and the device
    /// refresh token `rtd_src` of c1.
    const DISCARD_RACE_TOKENS: [(bool, &str); 2] = [(false, "rt_new"), (true, "rtd_src")];

    /// Runs the statements `range` of the discard of `id`, a device refresh token if `device`.
    async fn pg_discard_run(
        txn: &deadpool_postgres::Transaction<'_>,
        device: bool,
        id: &str,
        range: std::ops::Range<usize>,
        discard: &mut crate::entity::refresh_tokens::PgDiscard,
    ) -> Result<(), ErrorResponse> {
        let sql = if device {
            &crate::entity::refresh_tokens_devices::SQL_RTD_DISCARD
        } else {
            &crate::entity::refresh_tokens::SQL_RT_DISCARD
        };
        for step in &crate::entity::refresh_tokens::PG_DISCARD_STEPS[range] {
            discard.step(txn, sql, *step, id).await?;
        }
        Ok(())
    }

    const PG_DISCARD_LEN: usize = crate::entity::refresh_tokens::PG_DISCARD_STEPS.len();

    /// A discard of a refresh token of u1 for c1 (the replacement of a failed rotation, or a
    /// device refresh token) runs at the same time as a consent revoke of u1 for c1 (with
    /// `consent.enable`): one of them runs its first statements, the other one all of its
    /// statements, then the first one the rest. Both take `refresh_tokens` (or
    /// `refresh_tokens_devices`) before `issued_tokens`, so they are serialized and Postgres
    /// aborts neither as a deadlock, at every split.
    /// Run with `RAUTHY_TEST_PG_URL=postgres://.. cargo test -p rauthy-data -- --ignored`.
    #[tokio::test]
    #[ignore = "needs Postgres via RAUTHY_TEST_PG_URL"]
    async fn test_discard_revoke_race_postgres() {
        for (device, id) in DISCARD_RACE_TOKENS {
            for split in 1..=PG_DISCARD_LEN {
                pg_discard_revoke_race(device, id, true, split).await;
            }
            for split in 1..=PgTxnSide::Revoke.len() {
                pg_discard_revoke_race(device, id, false, split).await;
            }
        }
    }

    async fn pg_discard_revoke_race(
        device: bool,
        id: &'static str,
        discard_first: bool,
        split: usize,
    ) {
        let case = format!("id: {id}, discard_first: {discard_first}, split: {split}");
        let (pool, schema) = pg_race_setup().await;
        let mut first = pg_conn(&pool, &schema).await;
        let mut second = pg_conn(&pool, &schema).await;
        let monitor = pg_conn(&pool, &schema).await;
        monitor.execute(INSERT_RT_NEW, &[]).await.unwrap();
        let pid: i32 = second
            .query_one("SELECT pg_backend_pid()", &[])
            .await
            .unwrap()
            .get(0);

        let txn = first
            .build_transaction()
            .isolation_level(IsolationLevel::ReadCommitted)
            .start()
            .await
            .unwrap();
        let mut revoke = PgSideState::default();
        let mut discard = crate::entity::refresh_tokens::PgDiscard::default();
        let (first_len, second_side) = if discard_first {
            pg_discard_run(&txn, device, id, 0..split, &mut discard)
                .await
                .unwrap();
            (PG_DISCARD_LEN, false)
        } else {
            PgTxnSide::Revoke
                .run(&txn, 0..split, &mut revoke)
                .await
                .unwrap();
            (PgTxnSide::Revoke.len(), true)
        };
        let task = tokio::spawn(async move {
            let txn = second
                .build_transaction()
                .isolation_level(IsolationLevel::ReadCommitted)
                .start()
                .await?;
            if second_side {
                let mut discard = crate::entity::refresh_tokens::PgDiscard::default();
                pg_discard_run(&txn, device, id, 0..PG_DISCARD_LEN, &mut discard).await?;
            } else {
                let mut revoke = PgSideState::default();
                PgTxnSide::Revoke
                    .run(&txn, 0..PgTxnSide::Revoke.len(), &mut revoke)
                    .await?;
            }
            txn.commit().await.map_err(ErrorResponse::from)
        });
        pg_wait_blocked_or_done(&monitor, pid, &task).await;
        let res = if discard_first {
            pg_discard_run(&txn, device, id, split..first_len, &mut discard).await
        } else {
            PgTxnSide::Revoke
                .run(&txn, split..first_len, &mut revoke)
                .await
        };
        let res_first = match res {
            Ok(()) => txn.commit().await.map_err(ErrorResponse::from),
            Err(err) => Err(err),
        };
        let res_second = task.await.unwrap();

        assert!(res_first.is_ok(), "{case}: {res_first:?}");
        assert!(res_second.is_ok(), "{case}: {res_second:?}");
        assert!(
            pg_ids(
                &monitor,
                "SELECT id FROM refresh_tokens WHERE client_id = 'c1'"
            )
            .await
            .is_empty(),
            "{case}"
        );
        assert_eq!(
            pg_ids(&monitor, "SELECT id FROM refresh_tokens_devices").await,
            vec!["rtd_u2_d3"],
            "{case}"
        );
        assert_eq!(
            pg_ids(
                &monitor,
                "SELECT jti FROM issued_tokens WHERE revoked ORDER BY jti"
            )
            .await,
            vec!["j_legacy", "j_new", "j_src"],
            "{case}"
        );

        monitor
            .batch_execute(&format!("DROP SCHEMA {schema} CASCADE"))
            .await
            .unwrap();
    }

    /// A discard of a refresh token of u1 for c1 (`rt_c1`, or the device refresh token
    /// `rtd_src`) runs at the same time as a `Client::delete` of c1 or a `User::delete` of u1:
    /// one of them runs its first statements, the other one all of its statements, then the
    /// first one the rest. Whichever of them deletes a refresh token revokes its access token, so
    /// no access token of the deleted refresh tokens stays live, at every split, and Postgres
    /// aborts neither as a deadlock.
    /// Run with `RAUTHY_TEST_PG_URL=postgres://.. cargo test -p rauthy-data -- --ignored`.
    #[tokio::test]
    #[ignore = "needs Postgres via RAUTHY_TEST_PG_URL"]
    async fn test_discard_delete_race_postgres() {
        for side in [PgTxnSide::ClientDelete, PgTxnSide::UserDelete] {
            for (device, id) in [(false, "rt_c1"), (true, "rtd_src")] {
                for split in 1..=PG_DISCARD_LEN {
                    pg_discard_delete_race(side, device, id, true, split).await;
                }
                for split in 1..=side.len() {
                    pg_discard_delete_race(side, device, id, false, split).await;
                }
            }
        }
    }

    async fn pg_discard_delete_race(
        side: PgTxnSide,
        device: bool,
        id: &'static str,
        discard_first: bool,
        split: usize,
    ) {
        let case =
            format!("side: {side:?}, id: {id}, discard_first: {discard_first}, split: {split}");
        let (pool, schema) = pg_race_setup().await;
        let mut first = pg_conn(&pool, &schema).await;
        let mut second = pg_conn(&pool, &schema).await;
        let monitor = pg_conn(&pool, &schema).await;
        monitor
            .batch_execute(
                r#"
INSERT INTO refresh_tokens (id, user_id, nbf, exp, access_token_jti, client_id)
VALUES ('rt_c1', 'u1', 1, 2, 'j_c1', 'c1');
INSERT INTO issued_tokens (jti, user_id, exp) VALUES ('j_c1', 'u1', 2);
"#,
            )
            .await
            .unwrap();
        let pid: i32 = second
            .query_one("SELECT pg_backend_pid()", &[])
            .await
            .unwrap()
            .get(0);

        let txn = first
            .build_transaction()
            .isolation_level(IsolationLevel::ReadCommitted)
            .start()
            .await
            .unwrap();
        let mut state = PgSideState::default();
        let mut discard = crate::entity::refresh_tokens::PgDiscard::default();
        if discard_first {
            pg_discard_run(&txn, device, id, 0..split, &mut discard)
                .await
                .unwrap();
        } else {
            side.run(&txn, 0..split, &mut state).await.unwrap();
        }
        let task = tokio::spawn(async move {
            let txn = second
                .build_transaction()
                .isolation_level(IsolationLevel::ReadCommitted)
                .start()
                .await?;
            if discard_first {
                let mut state = PgSideState::default();
                side.run(&txn, 0..side.len(), &mut state).await?;
            } else {
                let mut discard = crate::entity::refresh_tokens::PgDiscard::default();
                pg_discard_run(&txn, device, id, 0..PG_DISCARD_LEN, &mut discard).await?;
            }
            txn.commit().await.map_err(ErrorResponse::from)
        });
        pg_wait_blocked_or_done(&monitor, pid, &task).await;
        let res = if discard_first {
            pg_discard_run(&txn, device, id, split..PG_DISCARD_LEN, &mut discard).await
        } else {
            side.run(&txn, split..side.len(), &mut state).await
        };
        let res_first = match res {
            Ok(()) => txn.commit().await.map_err(ErrorResponse::from),
            Err(err) => Err(err),
        };
        let res_second = task.await.unwrap();

        assert!(res_first.is_ok(), "{case}: {res_first:?}");
        assert!(res_second.is_ok(), "{case}: {res_second:?}");
        assert!(
            pg_ids(
                &monitor,
                "SELECT id FROM refresh_tokens WHERE user_id = 'u1'"
            )
            .await
            .is_empty(),
            "{case}"
        );
        assert!(
            pg_ids(
                &monitor,
                "SELECT id FROM refresh_tokens_devices WHERE user_id = 'u1'"
            )
            .await
            .is_empty(),
            "{case}"
        );
        // `Client::delete` also deletes the device refresh token of u2 for c1, and
        // `User::delete` revokes every access token of u1
        let mut revoked = vec!["j_c1", "j_legacy", "j_src"];
        match side {
            PgTxnSide::ClientDelete => revoked.push("j_u2_d3"),
            PgTxnSide::UserDelete => revoked.insert(2, "j_new"),
            PgTxnSide::Revoke => {}
        }
        assert_eq!(
            pg_ids(
                &monitor,
                "SELECT jti FROM issued_tokens WHERE revoked ORDER BY jti"
            )
            .await,
            revoked,
            "{case}"
        );

        monitor
            .batch_execute(&format!("DROP SCHEMA {schema} CASCADE"))
            .await
            .unwrap();
    }

    /// A refresh of u1 commits its access token and then its replacement refresh token (or
    /// device refresh token) while `User::delete` of u1 runs, after the delete removed the refresh
    /// tokens of u1 and before it deletes the user row. The replacement is either refused,
    /// because the user is gone, so its access token is never handed out, or it is deleted with
    /// its access token revoked. It is never removed by the cascade of the user row with its
    /// access token left live.
    /// Run with `RAUTHY_TEST_PG_URL=postgres://.. cargo test -p rauthy-data -- --ignored`.
    #[tokio::test]
    #[ignore = "needs Postgres via RAUTHY_TEST_PG_URL"]
    async fn test_user_delete_refresh_race_postgres() {
        for device in [false, true] {
            pg_user_delete_refresh_race(device).await;
        }
    }

    async fn pg_user_delete_refresh_race(device: bool) {
        use crate::entity::users::{PG_DELETE_STEPS, PgDelete, PgDeleteStep};
        use tokio_postgres::error::SqlState;

        let case = format!("device: {device}");
        let (pool, schema) = pg_race_setup().await;
        let mut first = pg_conn(&pool, &schema).await;
        let second = pg_conn(&pool, &schema).await;
        let monitor = pg_conn(&pool, &schema).await;
        let pid: i32 = second
            .query_one("SELECT pg_backend_pid()", &[])
            .await
            .unwrap()
            .get(0);

        let txn = first
            .build_transaction()
            .isolation_level(IsolationLevel::ReadCommitted)
            .start()
            .await
            .unwrap();
        let split = PG_DELETE_STEPS
            .iter()
            .position(|s| *s == PgDeleteStep::User)
            .unwrap();
        let mut delete = PgDelete::default();
        for step in &PG_DELETE_STEPS[..split] {
            delete.step(&txn, *step, "u1").await.unwrap();
        }
        let insert_replacement = if device {
            r#"
INSERT INTO refresh_tokens_devices (id, device_id, user_id, nbf, exp, access_token_jti)
VALUES ('rtd_r', 'd1', 'u1', 1, 2, 'j_r')"#
        } else {
            r#"
INSERT INTO refresh_tokens (id, user_id, nbf, exp, access_token_jti, client_id)
VALUES ('rt_r', 'u1', 1, 2, 'j_r', 'c1')"#
        };
        // the access token is issued before its refresh token is inserted
        second
            .batch_execute("INSERT INTO issued_tokens (jti, user_id, exp) VALUES ('j_r', 'u1', 2)")
            .await
            .unwrap();
        let task = tokio::spawn(async move { second.batch_execute(insert_replacement).await });
        pg_wait_blocked_or_done(&monitor, pid, &task).await;
        for step in &PG_DELETE_STEPS[split..] {
            delete.step(&txn, *step, "u1").await.unwrap();
        }
        txn.commit().await.unwrap();
        let res_insert = task.await.unwrap();

        assert!(delete.deleted.is_some(), "{case}");
        match res_insert {
            Err(err) => assert_eq!(
                err.code(),
                Some(&SqlState::FOREIGN_KEY_VIOLATION),
                "{case}: {err:?}"
            ),
            Ok(()) => assert!(
                pg_ids(&monitor, "SELECT jti FROM issued_tokens WHERE revoked")
                    .await
                    .contains(&"j_r".to_string()),
                "{case}: the replacement was removed with its access token live"
            ),
        }
        assert!(
            pg_ids(
                &monitor,
                "SELECT id FROM refresh_tokens WHERE user_id = 'u1' \
                UNION ALL SELECT id FROM refresh_tokens_devices WHERE user_id = 'u1'"
            )
            .await
            .is_empty(),
            "{case}"
        );

        monitor
            .batch_execute(&format!("DROP SCHEMA {schema} CASCADE"))
            .await
            .unwrap();
    }

    /// Two Postgres transactions of `User::delete` of u1 run at the same time: one of them runs
    /// its first statements, the other one all of its statements, then the first one the rest.
    /// Only one of them deletes the user row, and only that one gets the `picture_id` of the row
    /// as it is in the database. The other one ends with `PgDelete::deleted` as `None`. What
    /// `User::delete` does after its transaction is not covered here.
    /// Run with `RAUTHY_TEST_PG_URL=postgres://.. cargo test -p rauthy-data -- --ignored`.
    #[tokio::test]
    #[ignore = "needs Postgres via RAUTHY_TEST_PG_URL"]
    async fn test_user_delete_twice_postgres() {
        for split in 1..=PgTxnSide::UserDelete.len() {
            pg_user_delete_twice(split).await;
        }
    }

    async fn pg_user_delete_twice(split: usize) {
        use crate::entity::users::{PG_DELETE_STEPS, PgDelete};

        let case = format!("split: {split}");
        let (pool, schema) = pg_race_setup().await;
        let mut first = pg_conn(&pool, &schema).await;
        let mut second = pg_conn(&pool, &schema).await;
        let monitor = pg_conn(&pool, &schema).await;
        monitor
            .batch_execute("UPDATE users SET picture_id = 'p1' WHERE id = 'u1'")
            .await
            .unwrap();
        let pid: i32 = second
            .query_one("SELECT pg_backend_pid()", &[])
            .await
            .unwrap()
            .get(0);

        let txn = first
            .build_transaction()
            .isolation_level(IsolationLevel::ReadCommitted)
            .start()
            .await
            .unwrap();
        let mut delete = PgDelete::default();
        for step in &PG_DELETE_STEPS[..split] {
            delete.step(&txn, *step, "u1").await.unwrap();
        }
        let task = tokio::spawn(async move {
            let txn = second
                .build_transaction()
                .isolation_level(IsolationLevel::ReadCommitted)
                .start()
                .await?;
            let mut delete = PgDelete::default();
            for step in PG_DELETE_STEPS {
                delete.step(&txn, step, "u1").await?;
            }
            if delete.deleted.is_some() {
                txn.commit().await?;
            }
            Ok::<_, ErrorResponse>(delete.deleted)
        });
        pg_wait_blocked_or_done(&monitor, pid, &task).await;
        for step in &PG_DELETE_STEPS[split..] {
            delete.step(&txn, *step, "u1").await.unwrap();
        }
        if delete.deleted.is_some() {
            txn.commit().await.unwrap();
        }
        let deleted_second = task.await.unwrap().unwrap();

        let mut deleted = [delete.deleted, deleted_second];
        deleted.sort();
        assert_eq!(deleted, [None, Some(Some("p1".to_string()))], "{case}");
        assert!(
            pg_ids(&monitor, "SELECT id FROM users WHERE id = 'u1'")
                .await
                .is_empty(),
            "{case}"
        );
        assert_eq!(
            pg_ids(
                &monitor,
                "SELECT jti FROM issued_tokens WHERE revoked ORDER BY jti"
            )
            .await,
            vec!["j_legacy", "j_new", "j_src"],
            "{case}"
        );

        monitor
            .batch_execute(&format!("DROP SCHEMA {schema} CASCADE"))
            .await
            .unwrap();
    }
}
