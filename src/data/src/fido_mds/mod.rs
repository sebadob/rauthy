//! FIDO Metadata Service (MDS) dataset.
//!
//! The upstream MDS blob is a signed JWT holding a few hundred authenticator entries. It is
//! transformed once, ahead of time, into the compact [`MdsDataset`] persisted here: AAGUIDs as
//! `Uuid`s, the multi-valued metadata fields as bitmasks, the certification level as a single
//! ordered value, and the root certificates deduplicated into their own set. The prepared form is
//! shipped with the image and re-applied on startup; the same transform feeds the scheduled
//! refresh later on.
//!
//! Validation against the dataset is opt-in via `webauthn.force_passkey_attestation`: when set,
//! registration passes the MDS root certificates to webauthn-rs as its attestation CA list, and
//! authentication re-verifies the stored attestation chain of the matched passkey against the
//! roots listed for its AAGUID.

use crate::fido_mds::mds_entry::MdsEntry;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use webauthn_rs::prelude::{AttestationCaList, AttestationCaListBuilder};

pub mod dataset;
pub mod masks;
pub mod mds_entry;
mod raw;

/// A distinct root certificate, keyed by the SHA-256 of its DER encoding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MdsCert {
    pub hash: [u8; 32],
    pub cert_der: Vec<u8>,
}

/// An MDS entry joined with its distinct root certificates, as read from the database.
#[derive(Debug, Clone, PartialEq)]
pub struct MdsAuthenticator {
    pub entry: MdsEntry,
    pub certs: Vec<MdsCert>,
}

/// Build the webauthn-rs CA list used to enforce attestation at registration.
///
/// Every distinct root of every authenticator is inserted; entries sharing a root merge into the
/// same CA, which keeps the AAGUID authority check correct for shared MDS roots.
pub fn build_ca_list(
    authenticators: &[MdsAuthenticator],
) -> Result<AttestationCaList, ErrorResponse> {
    let mut builder = AttestationCaListBuilder::new();
    for auth in authenticators {
        for cert in &auth.certs {
            if let Err(e) = builder.insert_device_der(
                cert.cert_der.as_slice(),
                auth.entry.aaguid,
                auth.entry.description.clone(),
                BTreeMap::new(),
            ) {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Internal,
                    format!("Failed to insert MDS root certificate: {e:?}"),
                ));
            }
        }
    }
    Ok(builder.build())
}
