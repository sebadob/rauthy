use crate::database::DB;
use crate::rauthy_config::RauthyConfig;
use chrono::Utc;
use hiqlite_macros::params;
use rauthy_api_types::pam::PamSshAuthKeyResponse;
use rauthy_common::utils::base64_encode;
use rauthy_common::{is_hiqlite, sha256};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::fmt::Write;
use std::ops::{Add, Sub};
use tokio_postgres::Row;

#[derive(Debug)]
pub struct AuthorizedKey {
    pub pam_uid: u32,
    pub ts_added: i64,
    pub expires: Option<i64>,
    pub typ: String,
    pub data: String,
    pub comment: String,
}

impl From<hiqlite::Row<'_>> for AuthorizedKey {
    fn from(mut row: hiqlite::Row<'_>) -> Self {
        Self {
            // type cast is safe because Rauthy controls the input
            pam_uid: row.get::<i64>("pam_uid") as u32,
            ts_added: row.get("ts_added"),
            expires: row.get("expires"),
            typ: row.get("typ"),
            data: row.get("data"),
            comment: row.get("comment"),
        }
    }
}

impl From<tokio_postgres::Row> for AuthorizedKey {
    fn from(row: Row) -> Self {
        Self {
            // type cast is safe because Rauthy controls the input
            pam_uid: row.get::<_, i64>("pam_uid") as u32,
            ts_added: row.get("ts_added"),
            expires: row.get("expires"),
            typ: row.get("typ"),
            data: row.get("data"),
            comment: row.get("comment"),
        }
    }
}

impl AuthorizedKey {
    pub async fn insert(
        pam_uid: u32,
        // The raw data string copied from a generated public key -> will be parsed into components
        data_raw: &str,
    ) -> Result<(), ErrorResponse> {
        let (typ, data, comment) = Self::parse_raw_key(data_raw)?;

        let config = &RauthyConfig::get().vars.pam.authorized_keys;

        if config.blacklist_used_keys {
            let blacklist_hash = Self::blacklist_hash(typ, data);
            let sql = "SELECT 1 FROM ssh_auth_keys_used WHERE used_key_hash = $1 LIMIT 1";
            let count = if is_hiqlite() {
                DB::hql()
                    .query_raw(sql, params!(blacklist_hash))
                    .await?
                    .len()
            } else {
                DB::pg_query_rows(sql, &[&blacklist_hash], 0).await?.len()
            };
            if count > 0 {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "This key was used before",
                ));
            }
        }

        let expires = if config.forced_key_expiry_days > 0 {
            Some(
                Utc::now()
                    .add(chrono::Duration::days(config.forced_key_expiry_days as i64))
                    .timestamp(),
            )
        } else {
            None
        };

        let sql = r#"
INSERT INTO ssh_auth_keys (pam_uid, ts_added, expires, typ, data, comment)
VALUES ($1, $2, $3, $4, $5, $6)"#;

        let now = Utc::now().timestamp();
        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(pam_uid, now, expires, typ, data, comment))
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[&(pam_uid as i64), &now, &expires, &typ, &data, &comment],
            )
            .await?;
        }

        Ok(())
    }

    /// Note: If you already know the `PamUser.id`, use `find_by_pam_uid()`
    pub async fn find_by_username(username: String) -> Result<Vec<Self>, ErrorResponse> {
        let sql = r#"
SELECT * FROM ssh_auth_keys WHERE pam_uid = (
    SELECT id FROM pam_users WHERE name = $1
)
"#;

        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!(username)).await?
        } else {
            DB::pg_query(sql, &[&username], 1).await?
        };

        Ok(res)
    }

    pub async fn find_by_uid(pam_uid: u32) -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM ssh_auth_keys WHERE pam_uid = $1";

        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!(pam_uid)).await?
        } else {
            DB::pg_query(sql, &[&(pam_uid as i64)], 1).await?
        };

        Ok(res)
    }

    pub async fn find_by_uid_ts(pam_uid: u32, ts_added: i64) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM ssh_auth_keys WHERE pam_uid = $1 AND ts_added = $2";

        let slf = if is_hiqlite() {
            DB::hql()
                .query_map_one(sql, params!(pam_uid, ts_added))
                .await?
        } else {
            DB::pg_query_one(sql, &[&(pam_uid as i64), &ts_added]).await?
        };

        Ok(slf)
    }

    pub async fn find_expired(expired_days_ago: u16) -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM ssh_auth_keys WHERE expires IS NOT NULL AND expires < $1";

        let now = Utc::now()
            .sub(chrono::Duration::days(expired_days_ago as i64))
            .timestamp();
        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!(now)).await?
        } else {
            DB::pg_query(sql, &[&now], 1).await?
        };

        Ok(res)
    }

    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        let blacklist = RauthyConfig::get()
            .vars
            .pam
            .authorized_keys
            .blacklist_used_keys
            && self.expires.is_some();

        // a conflict may happen if config for blacklisting has changed in the meantime
        let sql_blacklist = r#"
INSERT INTO ssh_auth_keys_used (used_key_hash, ts_added)
VALUES ($1, $2)
ON CONFLICT (used_key_hash) DO NOTHING
"#;
        let sql_delete = "DELETE FROM ssh_auth_keys WHERE pam_uid = $1 AND ts_added = $2";

        if is_hiqlite() {
            let mut txn = Vec::with_capacity(2);

            if blacklist {
                let hash = Self::blacklist_hash(&self.typ, &self.data);
                let now = Utc::now().timestamp();
                txn.push((sql_blacklist, params!(hash, now)));
            }
            txn.push((sql_delete, params!(self.pam_uid, self.ts_added)));

            DB::hql().txn(txn).await?;
        } else {
            let mut cl = DB::pg().await?;
            let txn = cl.transaction().await?;

            if blacklist {
                let hash = Self::blacklist_hash(&self.typ, &self.data);
                let now = Utc::now().timestamp();
                DB::pg_txn_append(&txn, sql_blacklist, &[&hash, &now]).await?;
            }
            DB::pg_txn_append(&txn, sql_delete, &[&(self.pam_uid as i64), &self.ts_added]).await?;

            txn.commit().await?;
        }

        Ok(())
    }

    pub async fn cleanup_blacklist() -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM ssh_auth_keys_used WHERE ts_added < $1";

        let days = RauthyConfig::get()
            .vars
            .pam
            .authorized_keys
            .blacklist_cleanup_days as i64;
        let threshold = Utc::now().sub(chrono::Duration::days(days)).timestamp();

        if is_hiqlite() {
            DB::hql().execute(sql, params!(threshold)).await?;
        } else {
            DB::pg_execute(sql, &[&threshold]).await?;
        }

        Ok(())
    }

    pub async fn expire_all_keys_by_uid(pam_uid: u32) -> Result<(), ErrorResponse> {
        let sql = "UPDATE ssh_auth_keys SET expires = $1 WHERE pam_uid = $2";

        if is_hiqlite() {
            DB::hql().execute(sql, params!(pam_uid)).await?;
        } else {
            DB::pg_execute(sql, &[&(pam_uid as i64)]).await?;
        }
        Ok(())
    }
}

impl AuthorizedKey {
    /// Creates a blacklist hash from a given key.
    fn blacklist_hash(typ: &str, key: &str) -> String {
        let s = format!("{typ} {key}");
        base64_encode(sha256!(s.as_bytes()))
    }

    // TODO depending on how the connections will be made later on, maybe also an
    //  `append_*` fn would be nice and accept an existing String.
    pub fn fmt_authorized_keys(keys: &[Self]) -> Result<String, ErrorResponse> {
        let include_comments = RauthyConfig::get()
            .vars
            .pam
            .authorized_keys
            .include_comments;
        let now = Some(Utc::now().timestamp());
        let mut f = String::with_capacity(keys.len() * 80);

        for slf in keys {
            if slf.expires < now {
                continue;
            }
            if include_comments && !slf.comment.is_empty() {
                writeln!(f, "{} {} {}", slf.typ, slf.data, slf.comment)?;
            } else {
                writeln!(f, "{} {}", slf.typ, slf.data)?;
            }
        }

        Ok(f)
    }

    /// Parses a raw SSH public key into `(typ, key, comment)`
    fn parse_raw_key(data: &str) -> Result<(&str, &str, &str), ErrorResponse> {
        let (typ, rest) = data.split_once(" ").unwrap_or_default();
        if !typ.starts_with("ssh-") || typ.len() > 24 {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Invalid SSH key format (key type)",
            ));
        }

        let (data, comment) = rest.split_once(" ").unwrap_or((rest, ""));
        if data.len() < 48 {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Invalid SSH key format (key data)",
            ));
        }

        Ok((typ, data, comment))
    }
}

impl From<AuthorizedKey> for PamSshAuthKeyResponse {
    fn from(value: AuthorizedKey) -> Self {
        Self {
            ts_added: value.ts_added,
            expires: value.expires,
            typ: value.typ,
            data: value.data,
            comment: value.comment,
        }
    }
}
