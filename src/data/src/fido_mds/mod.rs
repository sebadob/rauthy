//! FIDO Metadata Service (MDS) dataset.
//!
//! The upstream MDS blob is a signed JWT holding a few hundred authenticator entries. It is
//! transformed once, ahead of time, into the compact [`MdsDataset`] persisted here: AAGUIDs as
//! `Uuid`s, the multi-valued metadata fields as bitmasks, the certification level as a single
//! ordered value, and the root certificates deduplicated into their own set. The prepared form is
//! shipped with the image and re-applied on startup; the same transform feeds the scheduled
//! refresh later on.
//!
//! Nothing reads this data yet. Enforcement (rejecting a passkey whose authenticator is not in, or
//! not certified strongly enough by, this dataset) is a separate, opt-in step.

use rauthy_common::utils::{deserialize, serialize};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tracing::warn;
use webauthn_rs::prelude::Uuid;

mod db;
mod raw;

#[cfg(test)]
mod tests;

/// A fully transformed MDS dataset, ready to be written to the database as-is.
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

/// One authenticator, keyed by its AAGUID.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MdsEntry {
    pub aaguid: Uuid,
    pub description: String,
    pub key_protection: KeyProtectionMask,
    pub attachment_hint: AttachmentHintMask,
    pub attestation_types: AttestationTypeMask,
    pub cert_level: MdsCertLevel,
    /// Hashes into [`MdsDataset::certs`]; never empty.
    pub cert_hashes: Vec<[u8; 32]>,
}

/// A distinct root certificate, keyed by the SHA-256 of its DER encoding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MdsCert {
    pub hash: [u8; 32],
    pub cert_der: Vec<u8>,
}

/// How strongly an authenticator is FIDO-certified, as a single ordered value.
///
/// The MDS reports the level and a bare `FIDO_CERTIFIED` as separate status reports on the same
/// entry, and the reports are not stored in chronological order, so the level must be taken as the
/// maximum seen across all reports rather than read off the latest one. FIDO only ever recertifies
/// upward, so the maximum is the current level in practice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(u8)]
pub enum MdsCertLevel {
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

impl MdsCertLevel {
    #[inline]
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

/// Parses the certification-carrying MDS status values. Any other status, `NOT_FIDO_CERTIFIED`
/// and the revocation statuses included, is not a level and is rejected here.
impl FromStr for MdsCertLevel {
    type Err = ErrorResponse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "FIDO_CERTIFIED" => Self::Certified,
            "FIDO_CERTIFIED_L1" => Self::L1,
            "FIDO_CERTIFIED_L1plus" => Self::L1Plus,
            "FIDO_CERTIFIED_L2" => Self::L2,
            "FIDO_CERTIFIED_L2plus" => Self::L2Plus,
            "FIDO_CERTIFIED_L3" => Self::L3,
            "FIDO_CERTIFIED_L3plus" => Self::L3Plus,
            _ => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    format!("Not a FIDO MDS certification level: {s:?}"),
                ));
            }
        })
    }
}

/// Defines a bitmask newtype over an MDS flag enum, so the masks cannot be mixed up with each
/// other or with a plain integer. `$field` is the MDS JSON field name, used in the log line when
/// the spec has grown a value we do not know yet.
macro_rules! mds_bitmask {
    ($mask:ident, $flag:ident, $field:literal) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $mask(u32);

        impl $mask {
            #[inline]
            pub fn contains(self, flag: $flag) -> bool {
                self.0 & flag as u32 != 0
            }

            #[inline]
            pub fn bits(self) -> u32 {
                self.0
            }

            /// ORs the MDS string values together, folding anything unrecognized into `Unknown`
            /// and logging it so a new spec value gets noticed and added in a later version.
            pub(crate) fn from_mds(values: &[String], description: &str) -> Self {
                let mut bits = 0u32;
                for v in values {
                    match v.parse::<$flag>() {
                        Ok(flag) => bits |= flag as u32,
                        Err(_) => {
                            bits |= $flag::Unknown as u32;
                            warn!(
                                "Unknown FIDO MDS {} value {v:?} for {description:?}",
                                $field
                            );
                        }
                    }
                }
                Self(bits)
            }
        }

        impl From<u32> for $mask {
            #[inline]
            fn from(bits: u32) -> Self {
                Self(bits)
            }
        }
    };
}

/// Key protection type. Order is wire-stable: never reorder, only append. `Unknown` is pinned to
/// bit 0 so the column can widen past 32 bits later without disturbing any existing value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum KeyProtection {
    Unknown = 1 << 0,
    Software = 1 << 1,
    Hardware = 1 << 2,
    Tee = 1 << 3,
    SecureElement = 1 << 4,
    RemoteHandle = 1 << 5,
}

mds_bitmask!(KeyProtectionMask, KeyProtection, "keyProtection");

impl FromStr for KeyProtection {
    type Err = ErrorResponse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "software" => Self::Software,
            "hardware" => Self::Hardware,
            "tee" => Self::Tee,
            "secure_element" => Self::SecureElement,
            "remote_handle" => Self::RemoteHandle,
            _ => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    format!("Unknown FIDO MDS keyProtection: {s:?}"),
                ));
            }
        })
    }
}

/// Attachment hint. Order is wire-stable: never reorder, only append. `Unknown` is bit 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

mds_bitmask!(AttachmentHintMask, AttachmentHint, "attachmentHint");

impl FromStr for AttachmentHint {
    type Err = ErrorResponse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "internal" => Self::Internal,
            "external" => Self::External,
            "wired" => Self::Wired,
            "wireless" => Self::Wireless,
            "nfc" => Self::Nfc,
            "bluetooth" => Self::Bluetooth,
            "network" => Self::Network,
            "wifi_direct" => Self::WifiDirect,
            "smart-card" => Self::SmartCard,
            _ => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    format!("Unknown FIDO MDS attachmentHint: {s:?}"),
                ));
            }
        })
    }
}

/// Attestation type. Order is wire-stable: never reorder, only append. `Unknown` is bit 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum AttestationType {
    Unknown = 1 << 0,
    BasicFull = 1 << 1,
    BasicSurrogate = 1 << 2,
    AttCa = 1 << 3,
    AnonCa = 1 << 4,
    Ecdaa = 1 << 5,
}

mds_bitmask!(AttestationTypeMask, AttestationType, "attestationTypes");

impl FromStr for AttestationType {
    type Err = ErrorResponse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "basic_full" => Self::BasicFull,
            "basic_surrogate" => Self::BasicSurrogate,
            "attca" => Self::AttCa,
            "anonca" => Self::AnonCa,
            "ecdaa" => Self::Ecdaa,
            _ => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    format!("Unknown FIDO MDS attestationTypes: {s:?}"),
                ));
            }
        })
    }
}
