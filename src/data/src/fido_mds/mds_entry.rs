use crate::database::DB;
use crate::fido_mds::masks::{
    AttachmentHintMask, AttestationTypeMask, KeyProtectionMask, MdsCertLevel,
};
use hiqlite::macros::FromRow;
use hiqlite::params;
use rauthy_common::is_hiqlite;
use rauthy_derive::FromPgRow;
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

#[derive(Debug, FromRow, FromPgRow)]
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

impl MdsEntrySimple {
    pub async fn find(aaguid: &[u8]) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM fido_mds_entries WHERE aaguid = $1";
        let slf = if is_hiqlite() {
            DB::hql().query_map_one(sql, params!(aaguid)).await?
        } else {
            DB::pg_query_one(sql, &[&aaguid]).await?
        };
        Ok(slf)
    }
}

impl Display for MdsEntrySimple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MdsEntrySimple {{ aaguid: {:?}, description: {}, key_protection: {}, \
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
