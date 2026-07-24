use super::*;
use rauthy_common::utils::base64_url_encode;

/// Wrap a JSON payload into a JWT shape the transform can read (`header.payload.signature`). The
/// header and signature are ignored, so they are just placeholders.
fn jwt(payload_json: &str) -> String {
    format!("aaaa.{}.zzzz", base64_url_encode(payload_json.as_bytes()))
}

/// A fixture that exercises every branch of the transform. Certificate values are arbitrary but
/// valid base64; `CERTA` (`Q0VSVEE=`) is deliberately shared between two kept entries.
fn fixture() -> String {
    let payload = r#"{
      "no": 271,
      "nextUpdate": "2026-08-01",
      "entries": [
        {
          "aaguid": "aaaaaaaa-0000-0000-0000-000000000001",
          "metadataStatement": {
            "description": "Kept L1, reports out of order",
            "keyProtection": ["hardware", "secure_element"],
            "attachmentHint": ["internal"],
            "attestationTypes": ["basic_full"],
            "attestationRootCertificates": ["Q0VSVEE="]
          },
          "statusReports": [
            { "status": "FIDO_CERTIFIED_L1" },
            { "status": "FIDO_CERTIFIED" }
          ]
        },
        {
          "aaguid": "aaaaaaaa-0000-0000-0000-000000000002",
          "metadataStatement": {
            "description": "Recertified upward L1 then L2",
            "keyProtection": ["hardware"],
            "attachmentHint": ["external", "nfc"],
            "attestationTypes": ["basic_full"],
            "attestationRootCertificates": ["Q0VSVEI="]
          },
          "statusReports": [
            { "status": "FIDO_CERTIFIED_L2" },
            { "status": "FIDO_CERTIFIED_L1" },
            { "status": "FIDO_CERTIFIED" }
          ]
        },
        {
          "aaguid": "aaaaaaaa-0000-0000-0000-000000000003",
          "metadataStatement": {
            "description": "Revoked, REVOKED reported before the older status",
            "keyProtection": ["hardware"],
            "attachmentHint": ["internal"],
            "attestationTypes": ["basic_full"],
            "attestationRootCertificates": ["Q0VSVEM="]
          },
          "statusReports": [
            { "status": "REVOKED" },
            { "status": "NOT_FIDO_CERTIFIED" }
          ]
        },
        {
          "aaguid": "aaaaaaaa-0000-0000-0000-000000000004",
          "metadataStatement": {
            "description": "No root certs",
            "keyProtection": ["software"],
            "attachmentHint": ["internal"],
            "attestationTypes": ["basic_surrogate"],
            "attestationRootCertificates": []
          },
          "statusReports": [{ "status": "FIDO_CERTIFIED" }]
        },
        {
          "metadataStatement": {
            "description": "No AAGUID (U2F)",
            "keyProtection": ["hardware"],
            "attachmentHint": ["external"],
            "attestationTypes": ["basic_full"],
            "attestationRootCertificates": ["Q0VSVEU="]
          },
          "statusReports": [{ "status": "FIDO_CERTIFIED_L1" }]
        },
        {
          "aaguid": "aaaaaaaa-0000-0000-0000-000000000006",
          "metadataStatement": {
            "description": "Shares CERTA with entry 1",
            "keyProtection": ["hardware"],
            "attachmentHint": ["internal"],
            "attestationTypes": ["basic_full"],
            "attestationRootCertificates": ["Q0VSVEE="]
          },
          "statusReports": [{ "status": "FIDO_CERTIFIED_L3plus" }]
        },
        {
          "aaguid": "aaaaaaaa-0000-0000-0000-000000000007",
          "metadataStatement": {
            "description": "Unknown key protection value, not certified",
            "keyProtection": ["hardware", "brand_new_protection"],
            "attachmentHint": ["internal"],
            "attestationTypes": ["basic_full"],
            "attestationRootCertificates": ["Q0VSVEc="]
          },
          "statusReports": [{ "status": "NOT_FIDO_CERTIFIED" }]
        },
        {
          "aaguid": "aaaaaaaa-0000-0000-0000-000000000008",
          "metadataStatement": {
            "description": "Compromised, even though also L2 certified",
            "keyProtection": ["hardware"],
            "attachmentHint": ["internal"],
            "attestationTypes": ["basic_full"],
            "attestationRootCertificates": ["Q0VSVEg="]
          },
          "statusReports": [
            { "status": "ATTESTATION_KEY_COMPROMISE" },
            { "status": "FIDO_CERTIFIED_L2" }
          ]
        }
      ]
    }"#;
    jwt(payload)
}

#[test]
fn transform_keeps_only_valid_aaguid_entries() {
    let ds = PreparedDataset::from_jwt(&fixture()).unwrap();

    assert_eq!(ds.blob_no, 271);
    assert_eq!(ds.next_update, "2026-08-01");

    // kept: entries 1, 2, 6, 7. dropped: 3 (revoked), 4 (no cert), 5 (no aaguid), 8 (compromised)
    assert_eq!(ds.entries.len(), 4);
    let descriptions: Vec<&str> = ds.entries.iter().map(|e| e.description.as_str()).collect();
    assert!(!descriptions.iter().any(|d| d.contains("Revoked")));
    assert!(!descriptions.iter().any(|d| d.contains("No root certs")));
    assert!(!descriptions.iter().any(|d| d.contains("No AAGUID")));
    assert!(!descriptions.iter().any(|d| d.contains("Compromised")));
}

#[test]
fn transform_derives_cert_level_as_the_max_across_reports() {
    let ds = PreparedDataset::from_jwt(&fixture()).unwrap();
    // entries are sorted by aaguid, so the order is 001, 002, 006, 007
    assert_eq!(ds.entries[0].cert_level, CertLevel::L1);
    assert_eq!(ds.entries[1].cert_level, CertLevel::L2);
    assert_eq!(ds.entries[2].cert_level, CertLevel::L3Plus);
    assert_eq!(ds.entries[3].cert_level, CertLevel::NotCertified);
}

#[test]
fn transform_folds_unrecognized_values_into_the_unknown_bit() {
    let ds = PreparedDataset::from_jwt(&fixture()).unwrap();
    let e = &ds.entries[3];
    assert_eq!(
        e.key_protection,
        KeyProtection::Hardware as u32 | KeyProtection::Unknown as u32
    );
    // the low bit is the stable Unknown marker
    assert_eq!(KeyProtection::Unknown as u32, 1);
}

#[test]
fn transform_deduplicates_shared_root_certificates() {
    let ds = PreparedDataset::from_jwt(&fixture()).unwrap();
    // CERTA, CERTB, CERTG across the four kept entries; CERTA is shared by 001 and 006
    assert_eq!(ds.certs.len(), 3);

    let e001 = &ds.entries[0];
    let e006 = &ds.entries[2];
    assert_eq!(e001.cert_hashes.len(), 1);
    assert_eq!(e006.cert_hashes.len(), 1);
    assert_eq!(e001.cert_hashes[0], e006.cert_hashes[0]);
    assert!(ds.certs.iter().any(|c| c.hash == e001.cert_hashes[0]));
}

#[test]
fn transform_is_deterministic_and_round_trips() {
    let a = PreparedDataset::from_jwt(&fixture()).unwrap();
    let b = PreparedDataset::from_jwt(&fixture()).unwrap();
    assert_eq!(a, b);

    let bytes = a.serialize().unwrap();
    let back = PreparedDataset::deserialize(&bytes).unwrap();
    assert_eq!(a, back);
}

#[test]
fn aaguid_is_parsed_to_16_bytes() {
    let ds = PreparedDataset::from_jwt(&fixture()).unwrap();
    assert_eq!(
        ds.entries[0].aaguid,
        [0xaa, 0xaa, 0xaa, 0xaa, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
    );
}
