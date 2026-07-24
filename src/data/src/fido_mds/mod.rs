//! FIDO Metadata Service (MDS) dataset.
//!
//! The upstream MDS blob is a signed JWT holding a few hundred authenticator entries. It is
//! transformed once, ahead of time, into the compact [`PreparedDataset`] persisted here: AAGUIDs
//! as raw bytes, the multi-valued metadata fields as bitmasks, the certification level as a single
//! ordered value, and the root certificates deduplicated into their own set. The prepared form is
//! shipped with the image and re-applied on startup; the same transform feeds the scheduled
//! refresh later on.
//!
//! Nothing reads this data yet. Enforcement (rejecting a passkey whose authenticator is not in, or
//! not certified strongly enough by, this dataset) is a separate, opt-in step.

pub mod db;
mod raw;

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

/// The bincode config used for the shipped dataset and any cached form.
const BINCODE: bincode::config::Configuration = bincode::config::standard();

/// A fully transformed MDS dataset, ready to be written to the database as-is.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreparedDataset {
    /// The monotonic MDS blob number (`no`). A higher value is newer.
    pub blob_no: i64,
    /// The `nextUpdate` date the blob advertises, kept verbatim for the scheduler.
    pub next_update: String,
    pub entries: Vec<PreparedEntry>,
    /// Every distinct root certificate referenced by `entries`, deduplicated by hash.
    pub certs: Vec<PreparedCert>,
}

impl PreparedDataset {
    #[inline]
    pub fn serialize(&self) -> Result<Vec<u8>, rauthy_error::ErrorResponse> {
        bincode::serde::encode_to_vec(self, BINCODE).map_err(|err| {
            rauthy_error::ErrorResponse::new(
                rauthy_error::ErrorResponseType::Internal,
                format!("Error serializing FIDO MDS dataset: {err}"),
            )
        })
    }

    #[inline]
    pub fn deserialize(bytes: &[u8]) -> Result<Self, rauthy_error::ErrorResponse> {
        bincode::serde::decode_from_slice(bytes, BINCODE)
            .map(|(slf, _)| slf)
            .map_err(|err| {
                rauthy_error::ErrorResponse::new(
                    rauthy_error::ErrorResponseType::Internal,
                    format!("Error deserializing FIDO MDS dataset: {err}"),
                )
            })
    }

    /// Transform a raw MDS JWT (as downloaded, `header.payload.signature`) into a prepared dataset.
    ///
    /// The signature is not verified: the blob is fetched over TLS from the alliance and shipped as
    /// trusted, pre-packaged data. Entries are kept only when they carry an AAGUID, have never been
    /// revoked or reported compromised, and ship at least one root certificate.
    #[inline]
    pub fn from_jwt(jwt: &str) -> Result<Self, rauthy_error::ErrorResponse> {
        raw::transform(jwt)
    }
}

/// One authenticator, keyed by its 16-byte AAGUID.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreparedEntry {
    pub aaguid: [u8; 16],
    pub description: String,
    /// Bitmask of [`KeyProtection`].
    pub key_protection: u32,
    /// Bitmask of [`AttachmentHint`].
    pub attachment_hint: u32,
    /// Bitmask of [`AttestationType`].
    pub attestation_types: u32,
    pub cert_level: CertLevel,
    /// Hashes into [`PreparedDataset::certs`]; never empty.
    pub cert_hashes: Vec<[u8; 32]>,
}

/// A distinct root certificate, keyed by the SHA-256 of its DER encoding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreparedCert {
    pub hash: [u8; 32],
    pub der: Vec<u8>,
}

/// How strongly an authenticator is FIDO-certified, as a single ordered value.
///
/// The MDS reports the level and a bare `FIDO_CERTIFIED` as separate status reports on the same
/// entry, and the reports are not stored in chronological order, so the level must be taken as the
/// maximum seen across all reports rather than read off the latest one. FIDO only ever recertifies
/// upward, so the maximum is the current level in practice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(u8)]
pub enum CertLevel {
    NotCertified = 0,
    /// Certified, but no level was ever reported (a single blob entry does this today).
    Certified = 1,
    L1 = 2,
    L1Plus = 3,
    L2 = 4,
    L2Plus = 5,
    L3 = 6,
    L3Plus = 7,
}

impl CertLevel {
    #[inline]
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

/// Key protection type. Order is wire-stable: never reorder, only append. `Unknown` is pinned to
/// bit 0 so the column can widen past 32 bits later without disturbing any existing value.
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum KeyProtection {
    Unknown = 1 << 0,
    Software = 1 << 1,
    Hardware = 1 << 2,
    Tee = 1 << 3,
    SecureElement = 1 << 4,
    RemoteHandle = 1 << 5,
}

impl KeyProtection {
    fn from_mds(s: &str) -> Option<Self> {
        Some(match s {
            "software" => Self::Software,
            "hardware" => Self::Hardware,
            "tee" => Self::Tee,
            "secure_element" => Self::SecureElement,
            "remote_handle" => Self::RemoteHandle,
            _ => return None,
        })
    }
}

/// Attachment hint. Order is wire-stable: never reorder, only append. `Unknown` is bit 0.
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum AttachmentHint {
    Unknown = 1 << 0,
    Internal = 1 << 1,
    External = 1 << 2,
    Wired = 1 << 3,
    Wireless = 1 << 4,
    Nfc = 1 << 5,
    Bluetooth = 1 << 6,
    Network = 1 << 7,
    WifiDirect = 1 << 8,
    SmartCard = 1 << 9,
}

impl AttachmentHint {
    fn from_mds(s: &str) -> Option<Self> {
        Some(match s {
            "internal" => Self::Internal,
            "external" => Self::External,
            "wired" => Self::Wired,
            "wireless" => Self::Wireless,
            "nfc" => Self::Nfc,
            "bluetooth" => Self::Bluetooth,
            "network" => Self::Network,
            "wifi_direct" => Self::WifiDirect,
            "smart-card" => Self::SmartCard,
            _ => return None,
        })
    }
}

/// Attestation type. Order is wire-stable: never reorder, only append. `Unknown` is bit 0.
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum AttestationType {
    Unknown = 1 << 0,
    BasicFull = 1 << 1,
    BasicSurrogate = 1 << 2,
    AttCa = 1 << 3,
    AnonCa = 1 << 4,
    Ecdaa = 1 << 5,
}

impl AttestationType {
    fn from_mds(s: &str) -> Option<Self> {
        Some(match s {
            "basic_full" => Self::BasicFull,
            "basic_surrogate" => Self::BasicSurrogate,
            "attca" => Self::AttCa,
            "anonca" => Self::AnonCa,
            "ecdaa" => Self::Ecdaa,
            _ => return None,
        })
    }
}
