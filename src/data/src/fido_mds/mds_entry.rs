use crate::database::{Cache, DB};
use crate::fido_mds::masks::{
    AttachmentHintMask, AttestationTypeMask, KeyProtectionMask, MdsCertLevel,
};
use hiqlite::macros::FromRow;
use hiqlite::params;
use rauthy_common::constants::IDX_WEBAUTHN;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use webauthn_rs::prelude::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MdsEntry {
    pub aaguid: Uuid,
    pub description: String,
    pub key_protection: KeyProtectionMask,
    pub attachment_hint: AttachmentHintMask,
    pub attestation_types: AttestationTypeMask,
    pub cert_level: MdsCertLevel,
    pub cert_hashes: Vec<[u8; 32]>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MdsEntrySimple {
    pub aaguid: Vec<u8>,
    pub description: String,
    #[column(from_i64)]
    pub key_protection: KeyProtectionMask,
    #[column(from_i64)]
    pub attachment_hint: AttachmentHintMask,
    #[column(from_i64)]
    pub attestation_types: AttestationTypeMask,
    #[column(from_i64)]
    pub cert_level: MdsCertLevel,
}

impl From<tokio_postgres::Row> for MdsEntrySimple {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            aaguid: row.get("aaguid"),
            description: row.get("description"),
            key_protection: row.get::<_, i64>("key_protection").into(),
            attachment_hint: row.get::<_, i64>("attachment_hint").into(),
            attestation_types: row.get::<_, i64>("attestation_types").into(),
            cert_level: (row.get::<_, i16>("cert_level") as i64).into(),
        }
    }
}

impl MdsEntrySimple {
    #[inline]
    fn cache_idx(aaguid: &Uuid) -> String {
        format!("{IDX_WEBAUTHN}_MDS_{aaguid}")
    }

    pub async fn find(aaguid: &[u8]) -> Result<Self, ErrorResponse> {
        let uuid = Uuid::from_slice(aaguid).unwrap_or_default();
        if let Some(slf) = DB::hql()
            .get(Cache::Webauthn, Self::cache_idx(&uuid))
            .await?
        {
            return Ok(slf);
        }

        let sql = "SELECT * FROM fido_mds_entries WHERE aaguid = $1";
        let slf: Self = if is_hiqlite() {
            DB::hql().query_map_one(sql, params!(aaguid)).await?
        } else {
            DB::pg_query_one(sql, &[&aaguid]).await?
        };

        DB::hql()
            .put(Cache::Webauthn, Self::cache_idx(&uuid), &slf, Some(3600))
            .await?;

        Ok(slf)
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM fido_mds_entries";

        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!()).await?
        } else {
            // at the time of implementation, we have 340 valid entries in the dataset
            DB::pg_query(sql, &[], 384).await?
        };

        Ok(res)
    }
}

impl Display for MdsEntrySimple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MdsEntrySimple {{ aaguid: {}, description: {}, key_protection: {}, \
             attachment_hint: {}, attestation_types: {}, cert_level: {} }}",
            Uuid::from_slice(&self.aaguid).unwrap_or_default(),
            self.description,
            self.key_protection,
            self.attachment_hint,
            self.attestation_types,
            self.cert_level
        )
    }
}
