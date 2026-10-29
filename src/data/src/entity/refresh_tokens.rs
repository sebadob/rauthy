use crate::database::DB;
use crate::entity::clients::Client;
use chrono::Utc;
use hiqlite::macros::{FromRow, params};
use rauthy_common::is_hiqlite;
use rauthy_derive::FromPgRow;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::Deserialize;
use std::fmt::{Debug, Formatter};
use tokio_postgres::IsolationLevel;

pub(crate) const SQL_RT_DELETE_BY_USER_CLIENT: &str =
    "DELETE FROM refresh_tokens WHERE user_id = $1 AND client_id = $2";
pub(crate) const SQL_RTD_DELETE_BY_USER_CLIENT: &str = r#"
DELETE FROM refresh_tokens_devices
WHERE user_id = $1
  AND device_id IN (SELECT id FROM devices WHERE client_id = $2)"#;

// With `consent.enable`, a consent revoke for `user_id` + `client_id` also covers the legacy
// refresh tokens of the user without a `client_id`, since those are valid for any client.
pub(crate) const SQL_RT_DELETE_BY_USER_CLIENT_OR_LEGACY: &str =
    "DELETE FROM refresh_tokens WHERE user_id = $1 AND (client_id = $2 OR client_id IS NULL)";
pub(crate) const SQL_PG_RT_DELETE_BY_USER_CLIENT_RET: &str = r#"
DELETE FROM refresh_tokens WHERE user_id = $1 AND client_id = $2
RETURNING access_token_jti"#;
/// The legacy refresh tokens of the user (`client_id` NULL), see
/// [`SQL_RT_DELETE_BY_USER_CLIENT_OR_LEGACY`].
pub(crate) const SQL_PG_RT_DELETE_LEGACY_BY_USER_RET: &str = r#"
DELETE FROM refresh_tokens WHERE user_id = $1 AND client_id IS NULL
RETURNING access_token_jti"#;
pub(crate) const SQL_PG_RTD_DELETE_BY_USER_CLIENT_RET: &str = r#"
DELETE FROM refresh_tokens_devices
WHERE user_id = $1
  AND device_id IN (SELECT id FROM devices WHERE client_id = $2)
RETURNING access_token_jti"#;
pub(crate) const SQL_PG_IT_REVOKE_JTIS: &str =
    "UPDATE issued_tokens SET revoked = true WHERE jti = ANY($1)";
/// Revokes the access tokens of the refresh tokens that the statements above delete. hiqlite
/// cannot return rows inside a transaction, so this runs first, in the same transaction.
pub(crate) const SQL_HQL_IT_REVOKE_BY_USER_CLIENT: &str = r#"
UPDATE issued_tokens SET revoked = true
WHERE jti IN (
    SELECT access_token_jti FROM refresh_tokens
    WHERE user_id = $1 AND client_id = $2
    UNION
    SELECT access_token_jti FROM refresh_tokens_devices
    WHERE user_id = $1
      AND device_id IN (SELECT id FROM devices WHERE client_id = $2)
)"#;
/// [`SQL_HQL_IT_REVOKE_BY_USER_CLIENT`] for [`SQL_RT_DELETE_BY_USER_CLIENT_OR_LEGACY`].
pub(crate) const SQL_HQL_IT_REVOKE_BY_USER_CLIENT_OR_LEGACY: &str = r#"
UPDATE issued_tokens SET revoked = true
WHERE jti IN (
    SELECT access_token_jti FROM refresh_tokens
    WHERE user_id = $1 AND (client_id = $2 OR client_id IS NULL)
    UNION
    SELECT access_token_jti FROM refresh_tokens_devices
    WHERE user_id = $1
      AND device_id IN (SELECT id FROM devices WHERE client_id = $2)
)"#;

/// The statements of [`RefreshToken::discard`], run in one transaction by [`discard_txn`].
pub(crate) const SQL_RT_DISCARD: DiscardSql = DiscardSql {
    hql: [
        r#"
UPDATE issued_tokens SET revoked = true
WHERE jti = (SELECT access_token_jti FROM refresh_tokens WHERE id = $1)"#,
        "DELETE FROM refresh_tokens WHERE id = $1",
    ],
    pg_delete: "DELETE FROM refresh_tokens WHERE id = $1 RETURNING access_token_jti",
};
/// [`PgDiscardStep::RevokeAccessToken`]
const SQL_PG_IT_REVOKE_JTI: &str = "UPDATE issued_tokens SET revoked = true WHERE jti = $1";

/// The statements that discard a refresh token of one table: [`SQL_RT_DISCARD`], or
/// [`SQL_RTD_DISCARD`](crate::entity::refresh_tokens_devices::SQL_RTD_DISCARD). All of them are
/// idempotent.
pub(crate) struct DiscardSql {
    /// hiqlite, in order: revokes the access token while the row still names it, then deletes
    /// the row. hiqlite cannot return rows inside a transaction, and applies the transaction as
    /// one raft log entry, so the order of the tables does not matter there.
    pub(crate) hql: [&'static str; 2],
    /// Postgres, [`PgDiscardStep::Delete`]: deletes the row and returns its `access_token_jti`
    pub(crate) pg_delete: &'static str,
}

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
    /// `Client::created_micros` of `client_id` when the token was issued.
    pub client_created_micros: Option<i64>,
}

impl Debug for RefreshToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RefreshToken {{ id: {}(...), user_id: {}, nbf: {}, exp: {}, scope: {:?}, is_mfa: {}, \
            session_id: {:?}, client_id: {:?}, client_created_micros: {:?} }}",
            &self.id[..5],
            self.user_id,
            self.nbf,
            self.exp,
            self.scope,
            self.is_mfa,
            self.session_id.as_ref().map(|sid| &sid[..5]),
            self.client_id,
            self.client_created_micros,
        )
    }
}

impl RefreshToken {
    /// `false` if the token was issued for another client, or for a deleted one with the same id.
    /// Legacy tokens without `client_id` match any client.
    pub fn is_for_client(&self, client: &Client) -> bool {
        match &self.client_id {
            None => true,
            Some(id) => *id == client.id && self.client_created_micros == client.created_micros,
        }
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
        client: &Client,
    ) -> Result<(), ErrorResponse> {
        let sql = r#"
INSERT INTO refresh_tokens
(id, user_id, nbf, exp, scope, is_mfa, session_id, access_token_jti, client_id,
client_created_micros)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
ON CONFLICT(id) DO UPDATE
SET user_id = $2, nbf = $3, exp = $4, scope = $5, session_id = $7, access_token_jti = $8,
    client_id = $9, client_created_micros = $10"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        id,
                        user_id,
                        nbf,
                        exp,
                        scope,
                        is_mfa,
                        session_id,
                        access_token_jti,
                        client.id.as_str(),
                        client.created_micros
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &id,
                    &user_id,
                    &nbf,
                    &exp,
                    &scope,
                    &is_mfa,
                    &session_id,
                    &access_token_jti,
                    &client.id,
                    &client.created_micros,
                ],
            )
            .await?;
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
        finish_rotation_with(self.delete_checked(), || Self::discard(replacement_id)).await
    }

    /// Deletes the refresh token `id` and revokes the access token issued with it, in one
    /// transaction, see [`discard_txn`]: either both happen or neither, so a failure leaves the
    /// row with its access token for a retry. An `id` that is already gone is fine.
    pub async fn discard(id: &str) -> Result<(), ErrorResponse> {
        discard_txn(&SQL_RT_DISCARD, id).await
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
        let deleted = if is_hiqlite() {
            let mut deleted = 0;
            for res in DB::hql()
                .txn(Self::hql_delete_by_user_client_stmts(user_id, client_id))
                .await?
            {
                deleted += res?;
            }
            deleted
        } else {
            let mut cl = DB::pg().await?;
            let txn = cl.transaction().await?;
            let deleted = Self::pg_delete_by_user_client_txn(&txn, user_id, client_id).await?;
            txn.commit().await?;
            deleted
        };
        Ok(deleted)
    }

    /// The hiqlite statements of [`Self::delete_by_user_client`] to compose into a larger
    /// transaction. Their results are the deleted rows.
    pub(crate) fn hql_delete_by_user_client_stmts(
        user_id: &str,
        client_id: &str,
    ) -> [(&'static str, hiqlite::Params); 2] {
        [
            (SQL_RT_DELETE_BY_USER_CLIENT, params!(user_id, client_id)),
            (SQL_RTD_DELETE_BY_USER_CLIENT, params!(user_id, client_id)),
        ]
    }

    /// [`Self::delete_by_user_client`] inside an existing transaction.
    pub(crate) async fn pg_delete_by_user_client_txn(
        txn: &deadpool_postgres::Transaction<'_>,
        user_id: &str,
        client_id: &str,
    ) -> Result<usize, ErrorResponse> {
        let mut deleted =
            DB::pg_txn_append(txn, SQL_RT_DELETE_BY_USER_CLIENT, &[&user_id, &client_id]).await?
                as usize;
        deleted += DB::pg_txn_append(txn, SQL_RTD_DELETE_BY_USER_CLIENT, &[&user_id, &client_id])
            .await? as usize;
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
}

/// Discards the refresh token `id` with `sql` in one transaction: deletes the row and revokes
/// the access token issued with it. If any statement fails, nothing is committed.
pub(crate) async fn discard_txn(sql: &DiscardSql, id: &str) -> Result<(), ErrorResponse> {
    if is_hiqlite() {
        for res in DB::hql().txn(sql.hql.map(|sql| (sql, params!(id)))).await? {
            res?;
        }
    } else {
        let mut cl = DB::pg().await?;
        pg_discard_txn(&mut cl, sql, id).await?;
    }
    Ok(())
}

/// The Postgres part of [`discard_txn`] on `cl`: [`PG_DISCARD_STEPS`] in one transaction.
pub(crate) async fn pg_discard_txn(
    cl: &mut deadpool_postgres::Object,
    sql: &DiscardSql,
    id: &str,
) -> Result<(), ErrorResponse> {
    let txn = cl
        .build_transaction()
        .isolation_level(IsolationLevel::ReadCommitted)
        .start()
        .await?;
    let mut discard = PgDiscard::default();
    for step in PG_DISCARD_STEPS {
        discard.step(&txn, sql, step, id).await?;
    }
    txn.commit().await?;
    Ok(())
}

/// A statement of the Postgres transaction of [`discard_txn`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PgDiscardStep {
    /// Deletes the row, see [`DiscardSql::pg_delete`]
    Delete,
    /// Revokes the access token the deleted row named
    RevokeAccessToken,
}

/// The statements of the Postgres transaction of [`discard_txn`], in order. The refresh token
/// table goes before `issued_tokens`, in the order `UserConsent::revoke` takes them, so that a
/// discard and a revoke of the same refresh token at the same time are serialized instead of
/// deadlocking. The access token is revoked in the same transaction as the delete.
///
/// The discard revokes the access token only if its own delete removed the row. If another
/// transaction deleted the row first, the delete returns nothing, and the access token is left
/// to that transaction. Some Postgres deletes of refresh tokens revoke the access tokens of the
/// rows they delete, each in its own transaction: `UserConsent::revoke` with its `PgRevoke`,
/// `Client::delete` and `User::delete` with [`PgDeletedJtis`]. The other deletes do NOT revoke
/// them, so when one of them deletes a row before a discard does, the access token the row named
/// stays valid until it expires: [`RefreshToken::delete`], [`RefreshToken::delete_checked`],
/// [`RefreshToken::find_delete`], [`RefreshToken::invalidate_all`],
/// [`RefreshToken::invalidate_for_user`], [`RefreshToken::delete_by_sid`],
/// [`RefreshToken::delete_by_user_client`], the device deletes
/// (`DeviceEntity::delete_refresh_tokens`, `DeviceEntity::invalidate`, and of
/// `RefreshTokenDevice` `delete`, `delete_checked`, `find_delete`, `invalidate_all`,
/// `invalidate_all_for_user` and `invalidate_with_refresh_tokens_for_user`), the FK cascade and
/// the expiry cleanup. `RefreshTokenDevice::discard` revokes like this discard does.
pub(crate) const PG_DISCARD_STEPS: [PgDiscardStep; 2] =
    [PgDiscardStep::Delete, PgDiscardStep::RevokeAccessToken];

/// The state of the Postgres transaction of [`discard_txn`]
#[derive(Debug, Default)]
pub(crate) struct PgDiscard {
    /// `access_token_jti` of the deleted row
    jti: Option<String>,
}

impl PgDiscard {
    pub(crate) async fn step(
        &mut self,
        txn: &deadpool_postgres::Transaction<'_>,
        sql: &DiscardSql,
        step: PgDiscardStep,
        id: &str,
    ) -> Result<(), ErrorResponse> {
        match step {
            PgDiscardStep::Delete => {
                self.jti = txn
                    .query_opt(sql.pg_delete, &[&id])
                    .await?
                    .and_then(|row| row.get::<_, Option<String>>(0));
            }
            PgDiscardStep::RevokeAccessToken => {
                if let Some(jti) = &self.jti {
                    DB::pg_txn_append(txn, SQL_PG_IT_REVOKE_JTI, &[jti]).await?;
                }
            }
        }
        Ok(())
    }
}

/// The `access_token_jti` of the refresh tokens that the statements of one Postgres transaction
/// deleted, revoked by its last statement. Each refresh token table goes before `issued_tokens`,
/// in the order [`PG_DISCARD_STEPS`] and `UserConsent::revoke` take them. Used only by
/// `Client::delete` and `User::delete`; `UserConsent::revoke` has its own `PgRevoke`. See
/// [`PG_DISCARD_STEPS`] for the deletes that do not revoke access tokens.
#[derive(Debug, Default)]
pub(crate) struct PgDeletedJtis(Vec<String>);

impl PgDeletedJtis {
    /// Runs `sql` in `txn`. A delete of refresh tokens returns their `access_token_jti` with
    /// `RETURNING access_token_jti`, which is kept; any other statement returns no rows.
    pub(crate) async fn delete(
        &mut self,
        txn: &deadpool_postgres::Transaction<'_>,
        sql: &str,
        params: &[&(dyn postgres_types::ToSql + Sync)],
    ) -> Result<(), ErrorResponse> {
        let rows = txn.query(sql, params).await?;
        self.0
            .extend(rows.iter().filter_map(|r| r.get::<_, Option<String>>(0)));
        Ok(())
    }

    /// The last statement: revokes the access tokens of the deleted refresh tokens.
    pub(crate) async fn revoke(
        &self,
        txn: &deadpool_postgres::Transaction<'_>,
    ) -> Result<(), ErrorResponse> {
        if !self.0.is_empty() {
            DB::pg_txn_append(txn, SQL_PG_IT_REVOKE_JTIS, &[&self.0]).await?;
        }
        Ok(())
    }
}

/// Ends a refresh token rotation whose source was kept until the replacement existed.
/// `delete_source` is the checked delete of the source. If it finds the source gone (a logout, a
/// consent revocation or a concurrent refresh consumed it after it was read), the replacement
/// may have been inserted after that deletion and escaped it, so `discard_replacement` removes
/// it and the rotation fails with the source's `NotFound`.
pub(crate) async fn finish_rotation_with<S, F, Fut>(
    delete_source: S,
    discard_replacement: F,
) -> Result<(), ErrorResponse>
where
    S: Future<Output = Result<(), ErrorResponse>>,
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<(), ErrorResponse>>,
{
    let err = match delete_source.await {
        Ok(()) => return Ok(()),
        Err(err) if matches!(err.error, ErrorResponseType::NotFound) => err,
        Err(err) => return Err(err),
    };
    discard_replacement().await?;
    Err(err)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::refresh_tokens_devices::SQL_RTD_DISCARD;
    use pretty_assertions::assert_eq;
    use std::cell::RefCell;
    use std::collections::HashMap;

    /// In-memory stand-in for `refresh_tokens(_devices)` and `issued_tokens`.
    #[derive(Default)]
    struct Db {
        // refresh token id -> `access_token_jti`
        rows: HashMap<String, String>,
        revoked: Vec<String>,
    }

    impl Db {
        fn insert(&mut self, id: &str, jti: &str) {
            self.rows.insert(id.to_string(), jti.to_string());
        }

        fn delete_checked(&mut self, id: &str) -> Result<(), ErrorResponse> {
            self.rows.remove(id).map(|_| ()).ok_or_else(|| {
                ErrorResponse::new(ErrorResponseType::NotFound, "Invalid Refresh Token")
            })
        }

        fn discard(&mut self, id: &str) {
            if let Some(jti) = self.rows.remove(id) {
                self.revoked.push(jti);
            }
        }

        /// `UserConsent::revoke`: removes the rows that exist right now
        fn revoke_all(&mut self) {
            let mut jtis: Vec<String> = std::mem::take(&mut self.rows).into_values().collect();
            jtis.sort();
            self.revoked.extend(jtis);
        }
    }

    async fn finish(
        db: &RefCell<Db>,
        source: &str,
        replacement: &str,
    ) -> Result<(), ErrorResponse> {
        let deleted = db.borrow_mut().delete_checked(source);
        finish_rotation_with(async { deleted }, || {
            db.borrow_mut().discard(replacement);
            async { Ok(()) }
        })
        .await
    }

    /// A revocation commits after the refresh read the source, but before the replacement was
    /// inserted. It removed the source, but could not see the replacement.
    #[tokio::test]
    async fn test_rotation_revoked_before_replacement() {
        let db = RefCell::new(Db::default());
        db.borrow_mut().insert("source", "j_source");

        db.borrow_mut().revoke_all();
        // `TokenSet::from_user` inserts the replacement
        db.borrow_mut().insert("replacement", "j_replacement");

        let err = finish(&db, "source", "replacement").await.unwrap_err();
        assert_eq!(err.error, ErrorResponseType::NotFound);
        assert!(db.borrow().rows.is_empty());
        assert_eq!(
            db.borrow().revoked,
            vec!["j_source".to_string(), "j_replacement".to_string()]
        );
    }

    #[tokio::test]
    async fn test_rotation_not_revoked() {
        let db = RefCell::new(Db::default());
        db.borrow_mut().insert("source", "j_source");
        db.borrow_mut().insert("replacement", "j_replacement");

        finish(&db, "source", "replacement").await.unwrap();
        assert_eq!(
            db.borrow().rows.keys().collect::<Vec<_>>(),
            vec!["replacement"]
        );
        assert!(db.borrow().revoked.is_empty());

        // any other error of the source delete is returned as is, the replacement stays
        let err = finish_rotation_with(
            async { Err(ErrorResponse::new(ErrorResponseType::Database, "down")) },
            || {
                db.borrow_mut().discard("replacement");
                async { Ok(()) }
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.error, ErrorResponseType::Database);
        assert_eq!(db.borrow().rows.len(), 1);
    }

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
    fn test_is_for_client() {
        let client = |id: &str, created_micros| Client {
            id: id.to_string(),
            created_micros,
            ..Default::default()
        };

        let mut rt: RefreshToken = serde_json::from_value(row(None)).unwrap();
        assert!(rt.is_for_client(&client("client_1", Some(1))));
        assert!(rt.is_for_client(&client("client_2", None)));

        rt.client_id = Some("client_1".to_string());
        assert!(rt.is_for_client(&client("client_1", None)));
        assert!(!rt.is_for_client(&client("client_1", Some(1))));
        assert!(!rt.is_for_client(&client("client_2", None)));

        rt.client_created_micros = Some(1);
        assert!(rt.is_for_client(&client("client_1", Some(1))));
        assert!(!rt.is_for_client(&client("client_1", Some(2))));
        assert!(!rt.is_for_client(&client("client_1", None)));
        assert!(!rt.is_for_client(&client("client_2", Some(1))));
    }

    const DISCARD_ROWS: &str = r#"
CREATE TEMP TABLE tu AS SELECT * FROM users LIMIT 1;
UPDATE tu SET id = 'u1', email = 'u1@example.com';
INSERT INTO users SELECT * FROM tu;
INSERT INTO refresh_tokens (id, user_id, nbf, exp, access_token_jti)
VALUES ('rt_1', 'u1', 1, 2, 'j_1'), ('rt_2', 'u1', 1, 2, 'j_2'),
       ('rt_no_jti', 'u1', 1, 2, NULL);
INSERT INTO issued_tokens (jti, user_id, exp)
VALUES ('j_1', 'u1', 2), ('j_2', 'u1', 2), ('j_other', 'u1', 2);
"#;
    const SELECT_RT_IDS: &str = "SELECT id FROM refresh_tokens ORDER BY id";
    const SELECT_RTD_IDS: &str = "SELECT id FROM refresh_tokens_devices ORDER BY id";
    const SELECT_REVOKED: &str = "SELECT jti FROM issued_tokens WHERE revoked ORDER BY jti";

    /// Rows for the fault injection of [`DISCARD_FAIL_SQLITE`] / [`DISCARD_FAIL_PG`]: a refresh
    /// token and a device refresh token whose discard fails in its second statement (`*_fail`),
    /// and a device refresh token that can be discarded. Run after [`DISCARD_ROWS`].
    const DISCARD_FAIL_ROWS: &str = r#"
INSERT INTO devices (id, client_id, user_id, created, access_exp, peer_ip, name)
VALUES ('d1', (SELECT id FROM clients ORDER BY id LIMIT 1), 'u1', 1, 2, '127.0.0.1', 'd1');
INSERT INTO refresh_tokens (id, user_id, nbf, exp, access_token_jti)
VALUES ('rt_fail', 'u1', 1, 2, 'j_fail');
INSERT INTO refresh_tokens_devices (id, device_id, user_id, nbf, exp, access_token_jti)
VALUES ('rtd_1', 'd1', 'u1', 1, 2, 'jd_1'), ('rtd_fail', 'd1', 'u1', 1, 2, 'jd_fail');
INSERT INTO issued_tokens (jti, user_id, exp)
VALUES ('j_fail', 'u1', 2), ('jd_1', 'u1', 2), ('jd_fail', 'u1', 2);
"#;
    /// The delete of a `*_fail` row aborts: the second statement of the hiqlite discard.
    const DISCARD_FAIL_SQLITE: &str = r#"
CREATE TRIGGER rt_fail BEFORE DELETE ON refresh_tokens
WHEN OLD.id LIKE '%_fail' BEGIN SELECT RAISE(ABORT, 'injected'); END;
CREATE TRIGGER rtd_fail BEFORE DELETE ON refresh_tokens_devices
WHEN OLD.id LIKE '%_fail' BEGIN SELECT RAISE(ABORT, 'injected'); END;
"#;
    /// The revoke of the access token of a `*_fail` row aborts: the second statement of the
    /// Postgres discard.
    const DISCARD_FAIL_PG: &str = r#"
CREATE FUNCTION fail_revoke() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF NEW.jti LIKE '%\_fail' THEN RAISE EXCEPTION 'injected'; END IF;
    RETURN NEW;
END $$;
CREATE TRIGGER it_fail BEFORE UPDATE ON issued_tokens
FOR EACH ROW EXECUTE FUNCTION fail_revoke();
"#;

    /// An in-memory database with all hiqlite migrations and [`DISCARD_ROWS`].
    fn sqlite_db() -> rusqlite::Connection {
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
        conn.execute_batch(DISCARD_ROWS).unwrap();
        conn
    }

    fn sqlite_ids(conn: &rusqlite::Connection, sql: &str) -> Vec<String> {
        let mut stmt = conn.prepare(sql).unwrap();
        stmt.query_map([], |r| r.get(0))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    }

    /// The statements of `RefreshToken::discard` against the full hiqlite schema.
    #[test]
    fn test_discard_sqlite() {
        let conn = sqlite_db();
        let ids = |sql: &str| sqlite_ids(&conn, sql);
        let exec = |sql: &str, id: &str| {
            conn.execute(sql, [id]).unwrap();
        };

        // the access token is revoked while the row still names it
        for (i, sql) in SQL_RT_DISCARD.hql.iter().enumerate() {
            exec(sql, "rt_2");
            let row = ids(SELECT_RT_IDS).contains(&"rt_2".to_string());
            let revoked = ids(SELECT_REVOKED).contains(&"j_2".to_string());
            assert!(
                row || revoked,
                "after statement {i}: row gone, jti not revoked"
            );
        }

        for sql in SQL_RT_DISCARD.hql {
            exec(sql, "rt_1");
        }
        assert_eq!(ids(SELECT_RT_IDS), vec!["rt_no_jti"]);
        assert_eq!(ids(SELECT_REVOKED), vec!["j_1", "j_2"]);

        // gone already, or without an access token
        for id in ["rt_1", "rt_no_jti", "rt_unknown"] {
            for sql in SQL_RT_DISCARD.hql {
                exec(sql, id);
            }
        }
        assert!(ids(SELECT_RT_IDS).is_empty());
        assert_eq!(ids(SELECT_REVOKED), vec!["j_1", "j_2"]);
    }

    /// The discard statements of both tables in one transaction, as `DB::hql().txn` runs them
    /// (hiqlite rolls the whole transaction back on the first failing statement): a delete that
    /// fails after the revoke leaves neither behind.
    #[test]
    fn test_discard_atomic_sqlite() {
        let mut conn = sqlite_db();
        conn.execute_batch(DISCARD_FAIL_ROWS).unwrap();
        conn.execute_batch(DISCARD_FAIL_SQLITE).unwrap();

        let mut discard = |sql: &DiscardSql, id: &str| -> rusqlite::Result<()> {
            let txn = conn.transaction()?;
            for sql in sql.hql {
                txn.execute(sql, [id])?;
            }
            txn.commit()
        };
        for (sql, id) in [(&SQL_RT_DISCARD, "rt_fail"), (&SQL_RTD_DISCARD, "rtd_fail")] {
            assert!(discard(sql, id).is_err(), "{id}: the delete must fail");
        }
        discard(&SQL_RTD_DISCARD, "rtd_1").unwrap();

        assert_eq!(
            sqlite_ids(&conn, SELECT_RT_IDS),
            vec!["rt_1", "rt_2", "rt_fail", "rt_no_jti"]
        );
        assert_eq!(sqlite_ids(&conn, SELECT_RTD_IDS), vec!["rtd_fail"]);
        assert_eq!(sqlite_ids(&conn, SELECT_REVOKED), vec!["jd_1"]);
    }

    /// A fresh schema `<prefix>_..` with all Postgres migrations and [`DISCARD_ROWS`], set as the
    /// `search_path` of the returned connection.
    async fn pg_db(prefix: &str) -> (deadpool_postgres::Object, String) {
        use deadpool_postgres::{Config, Runtime};
        use tokio_postgres::NoTls;

        let url = std::env::var("RAUTHY_TEST_PG_URL").expect("RAUTHY_TEST_PG_URL is not set");
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let schema = format!("{prefix}_{}_{nanos}", std::process::id());
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
        cl.batch_execute(DISCARD_ROWS).await.unwrap();
        (cl, schema)
    }

    async fn pg_ids(cl: &deadpool_postgres::Object, sql: &str) -> Vec<String> {
        let rows = cl.query(sql, &[]).await.unwrap();
        rows.iter().map(|r| r.get(0)).collect()
    }

    /// `PgDiscard`, the Postgres path of `RefreshToken::discard`, against all Postgres
    /// migrations: the row is deleted first, then its access token is revoked, in one
    /// transaction.
    /// Run with `RAUTHY_TEST_PG_URL=postgres://.. cargo test -p rauthy-data -- --ignored`.
    #[tokio::test]
    #[ignore = "needs Postgres via RAUTHY_TEST_PG_URL"]
    async fn test_discard_postgres() {
        let (mut cl, schema) = pg_db("rt_discard").await;

        let txn = cl.transaction().await.unwrap();
        let mut discard = PgDiscard::default();
        let ids = async |txn: &deadpool_postgres::Transaction<'_>, sql: &str| -> Vec<String> {
            let rows = txn.query(sql, &[]).await.unwrap();
            rows.iter().map(|r| r.get(0)).collect()
        };
        for (i, step) in PG_DISCARD_STEPS.into_iter().enumerate() {
            discard
                .step(&txn, &SQL_RT_DISCARD, step, "rt_2")
                .await
                .unwrap();
            let row = ids(&txn, SELECT_RT_IDS).await.contains(&"rt_2".to_string());
            let revoked = ids(&txn, SELECT_REVOKED).await.contains(&"j_2".to_string());
            match step {
                PgDiscardStep::Delete => {
                    assert!(!row && !revoked, "after statement {i}: {row} {revoked}")
                }
                PgDiscardStep::RevokeAccessToken => {
                    assert!(!row && revoked, "after statement {i}: {row} {revoked}")
                }
            }
        }
        txn.commit().await.unwrap();

        pg_discard_txn(&mut cl, &SQL_RT_DISCARD, "rt_1")
            .await
            .unwrap();
        assert_eq!(pg_ids(&cl, SELECT_RT_IDS).await, vec!["rt_no_jti"]);
        assert_eq!(pg_ids(&cl, SELECT_REVOKED).await, vec!["j_1", "j_2"]);

        // gone already, or without an access token
        for id in ["rt_1", "rt_no_jti", "rt_unknown"] {
            pg_discard_txn(&mut cl, &SQL_RT_DISCARD, id).await.unwrap();
        }
        assert!(pg_ids(&cl, SELECT_RT_IDS).await.is_empty());
        assert_eq!(pg_ids(&cl, SELECT_REVOKED).await, vec!["j_1", "j_2"]);

        cl.batch_execute(&format!("DROP SCHEMA {schema} CASCADE"))
            .await
            .unwrap();
    }

    /// `pg_discard_txn`, the Postgres path of `RefreshToken::discard` and
    /// `RefreshTokenDevice::discard`, with a revoke that fails after the delete: both roll back
    /// together.
    #[tokio::test]
    #[ignore = "needs Postgres via RAUTHY_TEST_PG_URL"]
    async fn test_discard_atomic_postgres() {
        let (mut cl, schema) = pg_db("rt_discard_atomic").await;
        cl.batch_execute(DISCARD_FAIL_ROWS).await.unwrap();
        cl.batch_execute(DISCARD_FAIL_PG).await.unwrap();

        for (sql, id) in [(&SQL_RT_DISCARD, "rt_fail"), (&SQL_RTD_DISCARD, "rtd_fail")] {
            let res = pg_discard_txn(&mut cl, sql, id).await;
            assert!(res.is_err(), "{id}: the revoke must fail");
        }
        pg_discard_txn(&mut cl, &SQL_RTD_DISCARD, "rtd_1")
            .await
            .unwrap();

        assert_eq!(
            pg_ids(&cl, SELECT_RT_IDS).await,
            vec!["rt_1", "rt_2", "rt_fail", "rt_no_jti"]
        );
        assert_eq!(pg_ids(&cl, SELECT_RTD_IDS).await, vec!["rtd_fail"]);
        assert_eq!(pg_ids(&cl, SELECT_REVOKED).await, vec!["jd_1"]);

        cl.batch_execute(&format!("DROP SCHEMA {schema} CASCADE"))
            .await
            .unwrap();
    }
}
