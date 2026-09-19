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
    let ds: MdsDataset = fixture().parse().unwrap();

    assert_eq!(ds.blob_no, 271);
    // 2026-08-01T00:00:00Z
    assert_eq!(ds.next_update_ts, 1785542400);

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
    let ds: MdsDataset = fixture().parse().unwrap();
    // entries are sorted by aaguid, so the order is 001, 002, 006, 007
    assert_eq!(ds.entries[0].cert_level, MdsCertLevel::L1);
    assert_eq!(ds.entries[1].cert_level, MdsCertLevel::L2);
    assert_eq!(ds.entries[2].cert_level, MdsCertLevel::L3Plus);
    assert_eq!(ds.entries[3].cert_level, MdsCertLevel::NotCertified);
}

#[test]
fn cert_level_ordering_is_wire_stable() {
    // the ordering is what the operator's "at least L2" filter will compare against, and the
    // numbers are persisted, so neither may be reshuffled
    assert!(MdsCertLevel::NotCertified < MdsCertLevel::Certified);
    assert!(MdsCertLevel::Certified < MdsCertLevel::L1);
    assert!(MdsCertLevel::L1 < MdsCertLevel::L2);
    assert!(MdsCertLevel::L2 < MdsCertLevel::L3Plus);
    assert_eq!(MdsCertLevel::NotCertified.as_u8(), 0);
    assert_eq!(MdsCertLevel::L3Plus.as_u8(), 7);
}

#[test]
fn transform_folds_unrecognized_values_into_the_unknown_bit() {
    let ds: MdsDataset = fixture().parse().unwrap();
    let e = &ds.entries[3];
    assert!(e.key_protection.contains(KeyProtection::Hardware));
    assert!(e.key_protection.contains(KeyProtection::Unknown));
    assert!(!e.key_protection.contains(KeyProtection::Software));
    assert_eq!(
        e.key_protection.bits(),
        KeyProtection::Hardware as u32 | KeyProtection::Unknown as u32
    );
}

#[test]
fn unknown_is_pinned_to_the_low_bit_of_every_mask() {
    // persisted bit positions: `Unknown` must stay at bit 0 so the columns can widen past 32 bits
    assert_eq!(KeyProtection::Unknown as u32, 1);
    assert_eq!(AttachmentHint::Unknown as u32, 1);
    assert_eq!(AttestationType::Unknown as u32, 1);
}

#[test]
fn transform_deduplicates_shared_root_certificates() {
    let ds: MdsDataset = fixture().parse().unwrap();
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
    let a: MdsDataset = fixture().parse().unwrap();
    let b: MdsDataset = fixture().parse().unwrap();
    assert_eq!(a, b);

    let bytes = a.serialize().unwrap();
    let back = MdsDataset::deserialize(&bytes).unwrap();
    assert_eq!(a, back);
}

#[test]
fn aaguid_is_parsed_into_the_16_bytes_the_db_stores() {
    let ds: MdsDataset = fixture().parse().unwrap();
    assert_eq!(
        ds.entries[0].aaguid.as_bytes(),
        &[0xaa, 0xaa, 0xaa, 0xaa, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
    );
}
