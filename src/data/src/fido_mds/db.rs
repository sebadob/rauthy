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
use crate::fido_mds::MdsDataset;
use crate::rauthy_config::RauthyConfig;
use hiqlite::Params;
use hiqlite::macros::params;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use tracing::{info, warn};

/// The dataset shipped with the image, produced by the `fido-mds-prep` tool.
static MDS_DATASET: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/fido_mds_dataset.bin"));

const SQL_CERT: &str = r#"
INSERT INTO fido_mds_certs (hash, cert_der) VALUES ($1, $2)
ON CONFLICT (hash) DO UPDATE SET cert_der = $2"#;

const SQL_ENTRY: &str = r#"
INSERT INTO fido_mds_entries
(aaguid, description, key_protection, attachment_hint, attestation_types, cert_level)
VALUES ($1, $2, $3, $4, $5, $6)
ON CONFLICT (aaguid) DO UPDATE SET
description = $2, key_protection = $3, attachment_hint = $4, attestation_types = $5,
cert_level = $6"#;

// all columns are the primary key, so there is nothing to update on a re-apply
const SQL_JOIN: &str = r#"
INSERT INTO fido_mds_entry_certs (aaguid, cert_hash) VALUES ($1, $2)
ON CONFLICT (aaguid, cert_hash) DO NOTHING"#;

const SQL_COUNT: &str = "SELECT COUNT(*) AS count FROM fido_mds_entries";

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
        let count: i64 = if is_hiqlite() {
            DB::hql()
                .query_raw(SQL_COUNT, params!())
                .await?
                .remove(0)
                .get("count")
        } else {
            DB::pg_query_rows(SQL_COUNT, &[], 1)
                .await?
                .remove(0)
                .get("count")
        };
        Ok(count)
    }

    async fn insert(&self) -> Result<(), ErrorResponse> {
        if is_hiqlite() {
            let mut txn: Vec<(&str, Params)> =
                Vec::with_capacity(self.certs.len() + self.entries.len() * 2);

            for c in &self.certs {
                txn.push((SQL_CERT, params!(c.hash.to_vec(), c.cert_der.clone())));
            }
            for e in &self.entries {
                txn.push((
                    SQL_ENTRY,
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
                    txn.push((SQL_JOIN, params!(e.aaguid.as_bytes().to_vec(), h.to_vec())));
                }
            }

            for res in DB::hql().txn(txn).await? {
                res?;
            }
        } else {
            let mut cl = DB::pg().await?;
            let txn = cl.transaction().await?;

            let st_cert = txn.prepare_cached(SQL_CERT).await?;
            let st_entry = txn.prepare_cached(SQL_ENTRY).await?;
            let st_join = txn.prepare_cached(SQL_JOIN).await?;

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
