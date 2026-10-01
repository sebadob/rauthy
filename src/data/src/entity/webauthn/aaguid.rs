//! AAGUID extraction from raw WebAuthn attestation objects.
//!
//! webauthn-rs does not expose the AAGUID for every attestation format, so we parse the raw
//! CBOR-encoded `attestationObject` ourselves and read the AAGUID from its `authData` field.
//!
//! The stored AAGUID is only meaningful if the attestation statement actually verified it, so
//! [`attested_aaguid`] gates the extraction on the attestation level webauthn-rs determined:
//! a non-`NULL` stored AAGUID means "attested device".

use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::Deserialize;
use webauthn_rs::prelude::ParsedAttestationData;

/// The `ATTESTED_DATA` flag bit of `authData.flags`. Only if this bit is set, the `authData`
/// contains an AAGUID after the sign count.
const FLAG_ATTESTED_DATA: u8 = 0x40;

/// Offset of the AAGUID within the `authData`: rpIdHash (32) + flags (1) + signCount (4).
const AAGUID_OFFSET: usize = 37;

/// Length of an AAGUID in bytes.
pub const AAGUID_LEN: usize = 16;

/// Minimal view of the CBOR-encoded `attestationObject` that is needed to reach the raw
/// `authData`. The other fields (`fmt`, `attStmt`) are ignored.
#[derive(Deserialize)]
struct AttestationObject<'a> {
    #[serde(rename = "authData")]
    auth_data: &'a [u8],
}

/// Extracts the 16-byte AAGUID from a raw, CBOR-encoded `attestationObject`.
///
/// Returns `None` if the object is not valid CBOR, does not contain an `authData`, or if the
/// `authData` is too short / does not have the `ATTESTED_DATA` flag set (in which case no
/// AAGUID is present). An all-zero AAGUID means "unknown" per the WebAuthn spec and is also
/// returned as `None`.
pub fn extract_aaguid(attestation_object: &[u8]) -> Result<Option<[u8; 16]>, ErrorResponse> {
    let att_obj =
        serde_cbor_2::from_slice::<AttestationObject<'_>>(attestation_object).map_err(|err| {
            ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!("Could not parse attestation object CBOR: {err:?}"),
            )
        })?;

    if att_obj.auth_data.len() < AAGUID_OFFSET + AAGUID_LEN {
        return Ok(None);
    }
    if att_obj.auth_data[32] & FLAG_ATTESTED_DATA == 0 {
        return Ok(None);
    }

    let aaguid: [u8; 16] = att_obj.auth_data[AAGUID_OFFSET..AAGUID_OFFSET + AAGUID_LEN]
        .try_into()
        .unwrap();
    // Per the WebAuthn spec, an all-zero AAGUID means "unknown", so treat it as absent.
    if aaguid == [0u8; 16] {
        return Ok(None);
    }

    Ok(Some(aaguid))
}

/// Returns the AAGUID only if the attestation statement actually verified it.
///
/// This is a deliberate safety net: for every variant other than `Basic`, `AttCa` and
/// `AnonCa` — which are the only attestations with a cryptographically verified attStmt —
/// the result is forced to `None`, even if the device sent an AAGUID. For `None` or
/// self-attestation (and every other unverified type) the AAGUID is just an unverified
/// device claim, so it must not be stored and the column stays `NULL`.
pub fn attested_aaguid(
    attestation_data: &ParsedAttestationData,
    attestation_object: &[u8],
) -> Result<Option<[u8; 16]>, ErrorResponse> {
    match attestation_data {
        ParsedAttestationData::Basic(_)
        | ParsedAttestationData::AttCa(_)
        | ParsedAttestationData::AnonCa(_) => extract_aaguid(attestation_object),
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const AAGUID: [u8; 16] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10,
    ];

    fn auth_data(flags: u8, aaguid: Option<&[u8; 16]>) -> Vec<u8> {
        let mut data = vec![0u8; AAGUID_OFFSET]; // rpIdHash (32) + flags (1) + signCount (4)
        data[32] = flags;
        if let Some(aaguid) = aaguid {
            data.extend_from_slice(aaguid);
        }
        data
    }

    /// Builds a minimal CBOR `attestationObject` with the spec layout: a map with
    /// `"fmt" => "none"`, `"attStmt" => {}` and `"authData"` as a CBOR byte string.
    fn attestation_object(auth_data: &[u8]) -> Vec<u8> {
        assert!(auth_data.len() < 256, "fixture assumes a one-byte length");
        let mut out = vec![0xA3]; // map(3)
        out.extend_from_slice(&[0x63]); // text(3): "fmt"
        out.extend_from_slice(b"fmt");
        out.extend_from_slice(&[0x64]); // text(4): "none"
        out.extend_from_slice(b"none");
        out.extend_from_slice(&[0x67]); // text(7): "attStmt"
        out.extend_from_slice(b"attStmt");
        out.push(0xA0); // map(0)
        out.extend_from_slice(&[0x68]); // text(8): "authData"
        out.extend_from_slice(b"authData");
        out.push(0x58); // byte string with one-byte length
        out.push(auth_data.len() as u8);
        out.extend_from_slice(auth_data);
        out
    }

    #[test]
    fn extract_with_attested_data_flag() {
        let obj = attestation_object(&auth_data(FLAG_ATTESTED_DATA, Some(&AAGUID)));
        assert_eq!(extract_aaguid(&obj).unwrap(), Some(AAGUID));
    }

    #[test]
    fn extract_without_attested_data_flag() {
        // without the flag bit, no AAGUID is present in the authData
        let obj = attestation_object(&auth_data(0x01, None));
        assert_eq!(extract_aaguid(&obj).unwrap(), None);
    }

    #[test]
    fn extract_short_auth_data() {
        // rpIdHash + flags + signCount only: no room for an AAGUID
        let obj = attestation_object(&auth_data(FLAG_ATTESTED_DATA, None));
        assert_eq!(extract_aaguid(&obj).unwrap(), None);
    }

    #[test]
    fn extract_all_zero_aaguid() {
        // per the WebAuthn spec, an all-zero AAGUID means "unknown"
        let obj = attestation_object(&auth_data(FLAG_ATTESTED_DATA, Some(&[0u8; 16])));
        assert_eq!(extract_aaguid(&obj).unwrap(), None);
    }

    #[test]
    fn attested_only_for_verified_attestation() {
        let obj = attestation_object(&auth_data(FLAG_ATTESTED_DATA, Some(&AAGUID)));

        for data in [
            ParsedAttestationData::Basic(Vec::new()),
            ParsedAttestationData::AttCa(Vec::new()),
            ParsedAttestationData::AnonCa(Vec::new()),
        ] {
            assert_eq!(attested_aaguid(&data, &obj).unwrap(), Some(AAGUID));
        }

        for data in [
            ParsedAttestationData::None,
            ParsedAttestationData::Self_,
            ParsedAttestationData::ECDAA,
            ParsedAttestationData::Uncertain,
        ] {
            assert_eq!(attested_aaguid(&data, &obj).unwrap(), None);
        }
    }

    #[test]
    fn extract_malformed_cbor() {
        assert!(extract_aaguid(b"not cbor").is_err());
        assert!(extract_aaguid(&[]).is_err());
    }

    #[test]
    fn extract_missing_auth_data() {
        // a valid CBOR map without an "authData" field
        let mut out = vec![0xA2]; // map(2)
        out.extend_from_slice(&[0x63]); // text(3): "fmt"
        out.extend_from_slice(b"fmt");
        out.extend_from_slice(&[0x64]); // text(4): "none"
        out.extend_from_slice(b"none");
        out.extend_from_slice(&[0x67]); // text(7): "attStmt"
        out.extend_from_slice(b"attStmt");
        out.push(0xA0); // map(0)
        assert!(extract_aaguid(&out).is_err());
    }
}
