//! Persistence for the FIDO MDS dataset.
//!
//! The prepared dataset is embedded in the binary and seeded on startup when the tables are still
//! empty, which covers both a fresh instance and an existing one upgrading into this version.
//! Keeping the shipped dataset fresh over time (a scheduled monthly refresh) is a separate step.
//!
//! The asset is produced by `just fido-mds-prep` and is not checked into the repository. When it
//! is missing at compile time, `build.rs` substitutes an empty placeholder and seeding becomes a
//! no-op, so a fresh checkout still builds and the prep tool itself can be built to create it.

use crate::database::DB;
use crate::fido_mds::mds_entry::MdsEntry;
use crate::fido_mds::raw;
use crate::rauthy_config::RauthyConfig;
use hiqlite::{Params, params};
use rauthy_common::is_hiqlite;
use rauthy_common::utils::{deserialize, serialize};
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tracing::{info, warn};

/// The dataset shipped with the image, produced by the `fido-mds-prep` tool.
static MDS_DATASET: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/fido_mds_dataset.bin"));

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MdsDataset {
    /// The monotonic MDS blob number (`no`). A higher value is newer.
    pub blob_no: i64,
    /// The `nextUpdate` the blob advertises, as a unix timestamp in seconds, for the scheduler.
    pub next_update_ts: i64,
    pub entries: Vec<MdsEntry>,
    /// Every distinct root certificate referenced by `entries`, deduplicated by hash.
    pub certs: Vec<MdsCert>,
}

/// A distinct root certificate, keyed by the SHA-256 of its DER encoding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MdsCert {
    pub hash: [u8; 32],
    pub cert_der: Vec<u8>,
}

impl MdsDataset {
    #[inline]
    pub fn serialize(&self) -> Result<Vec<u8>, ErrorResponse> {
        serialize(self)
    }

    #[inline]
    pub fn deserialize(bytes: &[u8]) -> Result<Self, ErrorResponse> {
        deserialize(bytes)
    }
}

/// Parses a raw MDS blob (as downloaded, `header.payload.signature`) into a prepared dataset.
///
/// The JWT signature is not verified here, see [`raw`] for what that does and does not buy us.
impl FromStr for MdsDataset {
    type Err = ErrorResponse;

    #[inline]
    fn from_str(jwt: &str) -> Result<Self, Self::Err> {
        raw::transform(jwt)
    }
}

impl MdsDataset {
    /// Seed the embedded dataset when the entries table is empty. A no-op otherwise, so it is safe
    /// to call on every startup. Only the primary node writes; followers receive it through Raft.
    pub async fn seed_embedded() -> Result<(), ErrorResponse> {
        if !RauthyConfig::get().is_primary_node {
            return Ok(());
        }
        if MDS_DATASET.is_empty() {
            warn!(
                "This build contains no FIDO MDS dataset. Run `just fido-mds-prep` and rebuild to \
                 include one."
            );
            return Ok(());
        }
        if Self::entries_count().await? > 0 {
            return Ok(());
        }

        let slf = Self::deserialize(MDS_DATASET)?;
        slf.insert().await?;

        info!(
            "Seeded FIDO MDS dataset no. {}: {} entries, {} root certs",
            slf.blob_no,
            slf.entries.len(),
            slf.certs.len(),
        );
        Ok(())
    }

    async fn entries_count() -> Result<i64, ErrorResponse> {
        let sql = "SELECT COUNT(*) AS count FROM fido_mds_entries";

        let count: i64 = if is_hiqlite() {
            DB::hql()
                .query_raw(sql, params!())
                .await?
                .remove(0)
                .get("count")
        } else {
            DB::pg_query_rows(sql, &[], 1).await?.remove(0).get("count")
        };
        Ok(count)
    }

    async fn insert(&self) -> Result<(), ErrorResponse> {
        let sql_cert = r#"
INSERT INTO fido_mds_certs (hash, cert_der) VALUES ($1, $2)
ON CONFLICT (hash) DO UPDATE SET cert_der = $2
"#;
        let sql_entry = r#"
INSERT INTO fido_mds_entries (
    aaguid, description, key_protection, attachment_hint, attestation_types, cert_level
)
VALUES ($1, $2, $3, $4, $5, $6)
ON CONFLICT (aaguid) DO UPDATE SET description = $2, key_protection = $3, attachment_hint = $4,
attestation_types = $5, cert_level = $6
"#;
        let sql_join = r#"
INSERT INTO fido_mds_entry_certs (aaguid, cert_hash)
VALUES ($1, $2)
ON CONFLICT (aaguid, cert_hash) DO NOTHING
"#;

        if is_hiqlite() {
            let mut txn: Vec<(&str, Params)> =
                Vec::with_capacity(self.certs.len() + self.entries.len() * 2);

            for c in &self.certs {
                txn.push((sql_cert, params!(c.hash.to_vec(), c.cert_der.clone())));
            }
            for e in &self.entries {
                txn.push((
                    sql_entry,
                    params!(
                        e.aaguid.as_bytes().to_vec(),
                        e.description.clone(),
                        e.key_protection.bits() as i64,
                        e.attachment_hint.bits() as i64,
                        e.attestation_types.bits() as i64,
                        e.cert_level.as_u8() as i64
                    ),
                ));
                for h in &e.cert_hashes {
                    txn.push((sql_join, params!(e.aaguid.as_bytes().to_vec(), h.to_vec())));
                }
            }

            for res in DB::hql().txn(txn).await? {
                res?;
            }
        } else {
            let mut cl = DB::pg().await?;
            let txn = cl.transaction().await?;

            let st_cert = txn.prepare_cached(sql_cert).await?;
            let st_entry = txn.prepare_cached(sql_entry).await?;
            let st_join = txn.prepare_cached(sql_join).await?;

            for c in &self.certs {
                let hash = c.hash.as_slice();
                txn.execute(&st_cert, &[&hash, &c.cert_der]).await?;
            }
            for e in &self.entries {
                let aaguid = e.aaguid.as_bytes().as_slice();
                let kp = e.key_protection.bits() as i64;
                let ah = e.attachment_hint.bits() as i64;
                let at = e.attestation_types.bits() as i64;
                let lvl = e.cert_level.as_u8() as i16;
                txn.execute(&st_entry, &[&aaguid, &e.description, &kp, &ah, &at, &lvl])
                    .await?;
                for h in &e.cert_hashes {
                    let hash = h.as_slice();
                    txn.execute(&st_join, &[&aaguid, &hash]).await?;
                }
            }

            txn.commit().await?;
        }

        Ok(())
    }
}
