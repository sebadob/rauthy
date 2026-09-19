//! Deserialization of the raw MDS JWT payload and its transform into an [`MdsDataset`].
//!
//! The JWT signature is deliberately not verified here. Transport security (the prep tool refuses
//! anything but `https`) protects the download against tampering in transit, but it is not the
//! same thing as cryptographic verification of the blob: it authenticates the server, not the
//! payload, and it says nothing about a compromised mirror or a modified local file passed with
//! `--source`. Verifying the blob's `x5c` chain against a pinned FIDO Alliance root would be that
//! guarantee, and is deliberately out of scope for this dataset-only change.

use crate::fido_mds::{
    AttachmentHintMask, AttestationTypeMask, KeyProtectionMask, MdsCert, MdsCertLevel, MdsDataset,
    MdsEntry,
};
use rauthy_common::utils::{base64_decode, base64_url_no_pad_decode};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::Deserialize;
use std::collections::BTreeMap;
use webauthn_rs::prelude::Uuid;

#[derive(Debug, Deserialize)]
struct RawBlob {
    no: i64,
    #[serde(rename = "nextUpdate")]
    next_update: String,
    entries: Vec<RawEntry>,
}

#[derive(Debug, Deserialize)]
struct RawEntry {
    aaguid: Option<Uuid>,
    #[serde(rename = "metadataStatement")]
    metadata_statement: RawMetadataStatement,
    #[serde(rename = "statusReports")]
    status_reports: Vec<RawStatusReport>,
}

#[derive(Debug, Deserialize)]
struct RawMetadataStatement {
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
struct RawStatusReport {
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

/// The `nextUpdate` field is a plain `YYYY-MM-DD` date, taken at midnight UTC.
fn next_update_ts(date: &str) -> Result<i64, ErrorResponse> {
    chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map(|d| d.and_time(chrono::NaiveTime::MIN).and_utc().timestamp())
        .map_err(|err| {
            ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!("Invalid FIDO MDS nextUpdate {date:?}: {err}"),
            )
        })
}

pub(super) fn transform(jwt: &str) -> Result<MdsDataset, ErrorResponse> {
    let payload_b64 = jwt.split('.').nth(1).ok_or_else(|| {
        ErrorResponse::new(ErrorResponseType::BadRequest, "FIDO MDS blob is not a JWT")
    })?;
    let payload = base64_url_no_pad_decode(payload_b64)?;
    let blob: RawBlob = serde_json::from_slice(&payload).map_err(|err| {
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("Error parsing FIDO MDS payload: {err}"),
        )
    })?;

    // deduplicate certs across all entries, keyed by their DER hash; BTreeMap keeps the output
    // deterministic so a regeneration from the same blob produces a byte-identical asset
    let mut cert_map: BTreeMap<[u8; 32], Vec<u8>> = BTreeMap::new();
    let mut entries = Vec::with_capacity(blob.entries.len());

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

        let mut cert_hashes = Vec::with_capacity(ms.attestation_root_certificates.len());
        for c in &ms.attestation_root_certificates {
            let cert_der = base64_decode(c)?;
            let hash: [u8; 32] = rauthy_common::sha256!(cert_der.as_slice())
                .try_into()
                .expect("SHA-256 digest is 32 bytes");
            cert_map.entry(hash).or_insert(cert_der);
            if !cert_hashes.contains(&hash) {
                cert_hashes.push(hash);
            }
        }

        // the reports are not chronologically ordered and the level arrives as a report separate
        // from the bare `FIDO_CERTIFIED`, so neither the first nor the last one can be trusted
        let cert_level = entry
            .status_reports
            .iter()
            .filter_map(|r| r.status.parse::<MdsCertLevel>().ok())
            .max()
            .unwrap_or(MdsCertLevel::NotCertified);

        entries.push(MdsEntry {
            aaguid,
            key_protection: KeyProtectionMask::from_mds(&ms.key_protection, &ms.description),
            attachment_hint: AttachmentHintMask::from_mds(&ms.attachment_hint, &ms.description),
            attestation_types: AttestationTypeMask::from_mds(
                &ms.attestation_types,
                &ms.description,
            ),
            cert_level,
            cert_hashes,
            description: ms.description,
        });
    }

    entries.sort_by_key(|e| e.aaguid);
    let certs = cert_map
        .into_iter()
        .map(|(hash, cert_der)| MdsCert { hash, cert_der })
        .collect();

    Ok(MdsDataset {
        blob_no: blob.no,
        next_update_ts: next_update_ts(&blob.next_update)?,
        entries,
        certs,
    })
}
