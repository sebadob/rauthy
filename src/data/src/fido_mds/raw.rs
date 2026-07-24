//! Deserialization of the raw MDS JWT payload and its transform into a [`PreparedDataset`].

use crate::fido_mds::{
    AttachmentHint, AttestationType, CertLevel, KeyProtection, PreparedCert, PreparedDataset,
    PreparedEntry,
};
use rauthy_common::utils::{base64_decode, base64_url_no_pad_decode};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::Deserialize;
use std::collections::BTreeMap;
use tracing::warn;

#[derive(Debug, Deserialize)]
struct MdsBlob {
    no: i64,
    #[serde(rename = "nextUpdate")]
    next_update: String,
    entries: Vec<MdsEntry>,
}

#[derive(Debug, Deserialize)]
struct MdsEntry {
    aaguid: Option<String>,
    #[serde(rename = "metadataStatement")]
    metadata_statement: MdsMetadataStatement,
    #[serde(rename = "statusReports")]
    status_reports: Vec<MdsStatusReport>,
}

#[derive(Debug, Deserialize)]
struct MdsMetadataStatement {
    #[serde(default)]
    description: String,
    #[serde(rename = "attachmentHint", default)]
    attachment_hint: Vec<String>,
    #[serde(rename = "attestationRootCertificates", default)]
    attestation_root_certificates: Vec<String>,
    #[serde(rename = "attestationTypes", default)]
    attestation_types: Vec<String>,
    #[serde(rename = "keyProtection", default)]
    key_protection: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct MdsStatusReport {
    status: String,
}

/// A status that permanently disqualifies an authenticator, regardless of any other report.
fn is_terminal_bad(status: &str) -> bool {
    matches!(
        status,
        "REVOKED"
            | "ATTESTATION_KEY_COMPROMISE"
            | "USER_VERIFICATION_BYPASS"
            | "USER_KEY_REMOTE_COMPROMISE"
            | "USER_KEY_PHYSICAL_COMPROMISE"
    )
}

/// Derive the certification level as the maximum seen across all status reports.
///
/// The reports are not chronologically ordered and the level arrives as a separate report from the
/// bare `FIDO_CERTIFIED`, so neither the first nor the last report can be trusted to carry it.
fn cert_level(reports: &[MdsStatusReport]) -> CertLevel {
    let mut max = CertLevel::NotCertified;
    let mut saw_certified = false;

    for r in reports {
        let level = match r.status.as_str() {
            "FIDO_CERTIFIED" => {
                saw_certified = true;
                continue;
            }
            "FIDO_CERTIFIED_L1" => CertLevel::L1,
            "FIDO_CERTIFIED_L1plus" => CertLevel::L1Plus,
            "FIDO_CERTIFIED_L2" => CertLevel::L2,
            "FIDO_CERTIFIED_L2plus" => CertLevel::L2Plus,
            "FIDO_CERTIFIED_L3" => CertLevel::L3,
            "FIDO_CERTIFIED_L3plus" => CertLevel::L3Plus,
            _ => continue,
        };
        if level > max {
            max = level;
        }
    }

    if max == CertLevel::NotCertified && saw_certified {
        CertLevel::Certified
    } else {
        max
    }
}

/// OR the recognized MDS values into a bitmask, folding anything unrecognized into `unknown` and
/// logging it so a new spec value gets noticed and added in a later version.
fn bitmask<F>(values: &[String], parse: F, unknown: u32, field: &str, description: &str) -> u32
where
    F: Fn(&str) -> Option<u32>,
{
    let mut mask = 0u32;
    for v in values {
        match parse(v) {
            Some(bit) => mask |= bit,
            None => {
                mask |= unknown;
                warn!("Unknown FIDO MDS {field} value {v:?} for {description:?}");
            }
        }
    }
    mask
}

fn parse_aaguid(s: &str) -> Result<[u8; 16], ErrorResponse> {
    let bytes = hex::decode(s.replace('-', "")).map_err(|err| {
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("Invalid FIDO MDS AAGUID {s:?}: {err}"),
        )
    })?;
    bytes.try_into().map_err(|_| {
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("FIDO MDS AAGUID {s:?} is not 16 bytes"),
        )
    })
}

pub(super) fn transform(jwt: &str) -> Result<PreparedDataset, ErrorResponse> {
    // header.payload.signature; the signature is intentionally not verified (see PreparedDataset)
    let payload_b64 = jwt.split('.').nth(1).ok_or_else(|| {
        ErrorResponse::new(ErrorResponseType::BadRequest, "FIDO MDS blob is not a JWT")
    })?;
    let payload = base64_url_no_pad_decode(payload_b64)?;
    let blob: MdsBlob = serde_json::from_slice(&payload).map_err(|err| {
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("Error parsing FIDO MDS payload: {err}"),
        )
    })?;

    // deduplicate certs across all entries, keyed by their DER hash; BTreeMap keeps the output
    // deterministic so a regeneration from the same blob produces a byte-identical asset
    let mut cert_map: BTreeMap<[u8; 32], Vec<u8>> = BTreeMap::new();
    let mut entries = Vec::new();

    for entry in blob.entries {
        // reachable from a WebAuthn registration only via an AAGUID; the rest are U2F/UAF
        let Some(aaguid) = entry.aaguid else {
            continue;
        };
        // never revoked or reported compromised
        if entry
            .status_reports
            .iter()
            .any(|r| is_terminal_bad(&r.status))
        {
            continue;
        }
        let ms = entry.metadata_statement;
        // must be chain-validatable, so at least one root cert is required
        if ms.attestation_root_certificates.is_empty() {
            continue;
        }

        let aaguid = parse_aaguid(&aaguid)?;

        let mut cert_hashes = Vec::with_capacity(ms.attestation_root_certificates.len());
        for c in &ms.attestation_root_certificates {
            let der = base64_decode(c)?;
            let hash: [u8; 32] = rauthy_common::sha256!(der.as_slice())
                .try_into()
                .expect("SHA-256 digest is 32 bytes");
            cert_map.entry(hash).or_insert(der);
            if !cert_hashes.contains(&hash) {
                cert_hashes.push(hash);
            }
        }

        entries.push(PreparedEntry {
            aaguid,
            key_protection: bitmask(
                &ms.key_protection,
                |s| KeyProtection::from_mds(s).map(|k| k as u32),
                KeyProtection::Unknown as u32,
                "keyProtection",
                &ms.description,
            ),
            attachment_hint: bitmask(
                &ms.attachment_hint,
                |s| AttachmentHint::from_mds(s).map(|a| a as u32),
                AttachmentHint::Unknown as u32,
                "attachmentHint",
                &ms.description,
            ),
            attestation_types: bitmask(
                &ms.attestation_types,
                |s| AttestationType::from_mds(s).map(|a| a as u32),
                AttestationType::Unknown as u32,
                "attestationTypes",
                &ms.description,
            ),
            cert_level: cert_level(&entry.status_reports),
            cert_hashes,
            description: ms.description,
        });
    }

    entries.sort_by_key(|e| e.aaguid);
    let certs = cert_map
        .into_iter()
        .map(|(hash, der)| PreparedCert { hash, der })
        .collect();

    Ok(PreparedDataset {
        blob_no: blob.no,
        next_update: blob.next_update,
        entries,
        certs,
    })
}
