//! Persistence for the FIDO MDS dataset.
//!
//! The prepared dataset is embedded in the binary and seeded on startup when the tables are still
//! empty, which covers both a fresh instance and an existing one upgrading into this version.
//! Keeping the shipped dataset fresh over time (a scheduled monthly refresh) is a separate step.

use crate::database::DB;
use crate::fido_mds::PreparedDataset;
use crate::rauthy_config::RauthyConfig;
use hiqlite::Params;
use hiqlite::macros::params;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use tracing::info;

/// The dataset shipped with the image, produced by the `fido-mds-prep` tool.
static EMBEDDED: &[u8] = include_bytes!("../../../../assets/fido_mds/dataset.bin");

const SQL_CERT: &str = "INSERT INTO fido_mds_certs (hash, der) VALUES ($1, $2)";
const SQL_ENTRY: &str = "INSERT INTO fido_mds_entries \
    (aaguid, description, key_protection, attachment_hint, attestation_types, cert_level) \
    VALUES ($1, $2, $3, $4, $5, $6)";
const SQL_JOIN: &str = "INSERT INTO fido_mds_entry_certs (aaguid, cert_hash) VALUES ($1, $2)";

/// Seed the embedded dataset when the entries table is empty. A no-op otherwise, so it is safe to
/// call on every startup. Only the primary node writes; followers receive it through Raft.
pub async fn seed_embedded() -> Result<(), ErrorResponse> {
    if !RauthyConfig::get().is_primary_node {
        return Ok(());
    }
    if entries_count().await? > 0 {
        return Ok(());
    }

    let dataset = PreparedDataset::deserialize(EMBEDDED)?;
    insert(&dataset).await?;

    info!(
        "Seeded FIDO MDS dataset no. {}: {} entries, {} root certs",
        dataset.blob_no,
        dataset.entries.len(),
        dataset.certs.len(),
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

/// Insert a full dataset. Certs and entries are written before the join rows so the foreign keys
/// resolve.
async fn insert(dataset: &PreparedDataset) -> Result<(), ErrorResponse> {
    if is_hiqlite() {
        let mut txn: Vec<(&str, Params)> =
            Vec::with_capacity(dataset.certs.len() + dataset.entries.len() * 2);

        for c in &dataset.certs {
            txn.push((SQL_CERT, params!(c.hash.to_vec(), c.der.clone())));
        }
        for e in &dataset.entries {
            txn.push((
                SQL_ENTRY,
                params!(
                    e.aaguid.to_vec(),
                    e.description.clone(),
                    e.key_protection as i64,
                    e.attachment_hint as i64,
                    e.attestation_types as i64,
                    e.cert_level.as_u8() as i64
                ),
            ));
            for h in &e.cert_hashes {
                txn.push((SQL_JOIN, params!(e.aaguid.to_vec(), h.to_vec())));
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

        for c in &dataset.certs {
            let hash = c.hash.as_slice();
            txn.execute(&st_cert, &[&hash, &c.der]).await?;
        }
        for e in &dataset.entries {
            let aaguid = e.aaguid.as_slice();
            let kp = e.key_protection as i64;
            let ah = e.attachment_hint as i64;
            let at = e.attestation_types as i64;
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
