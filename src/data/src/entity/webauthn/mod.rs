use crate::fido_mds::masks::{AttachmentHintMask, KeyProtectionMask, MdsCertLevel};
use crate::fido_mds::mds_entry::MdsEntrySimple;
use crate::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use tracing::debug;

pub mod aaguid;
pub mod auth_data;
pub mod auth_req;
pub mod authenticate;
pub mod authenticate_rk;
mod ceremony;
mod mds_authenticator;
pub mod passkey;
pub mod register;
pub mod rk_token;

#[inline]
#[must_use]
fn force_attestation() -> bool {
    let config = &RauthyConfig::get().vars.webauthn;
    config.force_passkey_cert_level.is_some()
        || config.force_passkey_protection.is_some()
        || config.force_passkey_attachment.is_some()
}

async fn verify_attestation(
    aaguid: Option<&[u8]>,
    user_id: &str,
    pk_name: &str,
) -> Result<(), ErrorResponse> {
    if !force_attestation() {
        return Ok(());
    }

    // We handle the AAGUID in a way that whenever this exists, the device was verified during
    // registration. There can never be a situation where an entity has the AAGUID set without
    // being attested.
    let Some(aaguid) = &aaguid else {
        debug!(
            "FIDO attestation enforced but passkey has not been verified: {user_id} / {pk_name}",
        );
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "Missing FIDO authenticator attestation",
        ));
    };
    let mds = MdsEntrySimple::find(aaguid).await?;
    let config = &RauthyConfig::get().vars.webauthn;

    verify_mds_attestation(
        &mds,
        &config.force_passkey_cert_level,
        &config.force_passkey_protection,
        &config.force_passkey_attachment,
    )
}

fn verify_mds_attestation(
    mds: &MdsEntrySimple,
    force_passkey_cert_level: &Option<MdsCertLevel>,
    force_passkey_protection: &Option<KeyProtectionMask>,
    force_passkey_attachment: &Option<AttachmentHintMask>,
) -> Result<(), ErrorResponse> {
    debug!("Validating FIDO attestation for {mds}");

    // Note: It's important that we return 406 instead of 403 here. The UI matches on 406 to show
    // proper i18n for the issue. Also, do NOT update the error messages without also updating the
    // ui. It does string matching after the 406 to select the correct message to show.

    if let Some(level) = force_passkey_cert_level
        && &mds.cert_level < level
    {
        debug!(
            "FIDO certification level mismatch. Allowed: {} / Found: {}",
            level, mds.cert_level
        );
        return Err(ErrorResponse::new(
            ErrorResponseType::NotAccepted,
            "FIDO Authenticator certification level too low",
        ));
    }
    // The values reported by the authenticator must be a subset of the allowed ones (see
    // force_passkey_protection in config.toml).
    if let Some(allowed) = force_passkey_protection
        && !mds.key_protection.is_subset_of(*allowed)
    {
        debug!(
            "FIDO key protection mismatch. Allowed: {} / Found: {}",
            allowed, mds.key_protection
        );
        return Err(ErrorResponse::new(
            ErrorResponseType::NotAccepted,
            "FIDO Authenticator key protection not allowed",
        ));
    }
    // Same subset check as above for the attachment hints.
    if let Some(allowed) = force_passkey_attachment
        && !mds.attachment_hint.is_subset_of(*allowed)
    {
        debug!(
            "FIDO key attachment mismatch. Allowed: {} / Found: {}",
            allowed, mds.attachment_hint
        );
        return Err(ErrorResponse::new(
            ErrorResponseType::NotAccepted,
            "FIDO Authenticator attachment hint not allowed",
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fido_mds::masks::{
        AttachmentHint, AttachmentHintMask, AttestationTypeMask, KeyProtection, KeyProtectionMask,
    };

    fn mds_entry(
        key_protection: KeyProtectionMask,
        attachment_hint: AttachmentHintMask,
        cert_level: MdsCertLevel,
    ) -> MdsEntrySimple {
        MdsEntrySimple {
            aaguid: vec![0u8; 16],
            description: "test authenticator".to_string(),
            key_protection,
            attachment_hint,
            attestation_types: AttestationTypeMask::default(),
            cert_level,
        }
    }

    fn prot_mask(flags: &[KeyProtection]) -> KeyProtectionMask {
        let mut mask = KeyProtectionMask::default();
        for flag in flags {
            mask = mask.insert(*flag);
        }
        mask
    }

    fn att_mask(flags: &[AttachmentHint]) -> AttachmentHintMask {
        let mut mask = AttachmentHintMask::default();
        for flag in flags {
            mask = mask.insert(*flag);
        }
        mask
    }

    fn assert_forbidden(res: Result<(), ErrorResponse>, message: &str) {
        let err = res.unwrap_err();
        assert_eq!(err.error, ErrorResponseType::NotAccepted);
        assert_eq!(err.message, message);
    }

    #[test]
    fn no_forced_values_passes() {
        // With nothing forced in the config, any entry passes.
        let mds = mds_entry(
            prot_mask(&[KeyProtection::Software]),
            att_mask(&[AttachmentHint::Internal]),
            MdsCertLevel::NotCertified,
        );
        assert_eq!(verify_mds_attestation(&mds, &None, &None, &None), Ok(()));
    }

    #[test]
    fn cert_level_below_minimum_is_rejected() {
        let mds = mds_entry(
            KeyProtectionMask::default(),
            AttachmentHintMask::default(),
            MdsCertLevel::L1Plus,
        );
        assert_forbidden(
            verify_mds_attestation(&mds, &Some(MdsCertLevel::L2), &None, &None),
            "FIDO Authenticator certification level too low",
        );
    }

    #[test]
    fn cert_level_at_or_above_minimum_passes() {
        for level in [MdsCertLevel::L2, MdsCertLevel::L3Plus] {
            let mds = mds_entry(
                KeyProtectionMask::default(),
                AttachmentHintMask::default(),
                level,
            );
            assert_eq!(
                verify_mds_attestation(&mds, &Some(MdsCertLevel::L2), &None, &None),
                Ok(())
            );
        }
    }

    #[test]
    fn key_protection_subset_passes() {
        // The config.toml example: reporting a subset of the allowed values is fine.
        let mds = mds_entry(
            prot_mask(&[KeyProtection::Hardware, KeyProtection::Tee]),
            AttachmentHintMask::default(),
            MdsCertLevel::NotCertified,
        );
        let allowed = prot_mask(&[
            KeyProtection::Hardware,
            KeyProtection::Tee,
            KeyProtection::SecureElement,
        ]);
        assert_eq!(
            verify_mds_attestation(&mds, &None, &Some(allowed), &None),
            Ok(())
        );
    }

    #[test]
    fn key_protection_disallowed_value_is_rejected() {
        // The other half of the config.toml example: one disallowed value fails the check.
        let mds = mds_entry(
            prot_mask(&[KeyProtection::Hardware, KeyProtection::RemoteHandle]),
            AttachmentHintMask::default(),
            MdsCertLevel::NotCertified,
        );
        let allowed = prot_mask(&[
            KeyProtection::Hardware,
            KeyProtection::Tee,
            KeyProtection::SecureElement,
        ]);
        assert_forbidden(
            verify_mds_attestation(&mds, &None, &Some(allowed), &None),
            "FIDO Authenticator key protection not allowed",
        );
    }

    #[test]
    fn key_protection_empty_mask_passes() {
        // An empty mask is a subset of anything.
        let mds = mds_entry(
            KeyProtectionMask::default(),
            AttachmentHintMask::default(),
            MdsCertLevel::NotCertified,
        );
        assert_eq!(
            verify_mds_attestation(
                &mds,
                &None,
                &Some(prot_mask(&[KeyProtection::Hardware])),
                &None
            ),
            Ok(())
        );
    }

    #[test]
    fn key_protection_unknown_flag_is_rejected() {
        // Unrecognized MDS values fold into `Unknown`, which never appears in the config, so
        // the check fails closed.
        let mds = mds_entry(
            prot_mask(&[KeyProtection::Unknown]),
            AttachmentHintMask::default(),
            MdsCertLevel::NotCertified,
        );
        assert_forbidden(
            verify_mds_attestation(
                &mds,
                &None,
                &Some(prot_mask(&[KeyProtection::Hardware])),
                &None,
            ),
            "FIDO Authenticator key protection not allowed",
        );
    }

    #[test]
    fn attachment_hint_subset_passes() {
        let mds = mds_entry(
            KeyProtectionMask::default(),
            att_mask(&[AttachmentHint::External, AttachmentHint::Wired]),
            MdsCertLevel::NotCertified,
        );
        let allowed = att_mask(&[AttachmentHint::External, AttachmentHint::Wired]);
        assert_eq!(
            verify_mds_attestation(&mds, &None, &None, &Some(allowed)),
            Ok(())
        );
    }

    #[test]
    fn attachment_hint_disallowed_value_is_rejected() {
        let mds = mds_entry(
            KeyProtectionMask::default(),
            att_mask(&[AttachmentHint::External, AttachmentHint::Nfc]),
            MdsCertLevel::NotCertified,
        );
        let allowed = att_mask(&[AttachmentHint::External]);
        assert_forbidden(
            verify_mds_attestation(&mds, &None, &None, &Some(allowed)),
            "FIDO Authenticator attachment hint not allowed",
        );
    }

    #[test]
    fn cert_level_check_runs_before_protection_checks() {
        // Both checks fail here; the certification level error is reported first.
        let mds = mds_entry(
            prot_mask(&[KeyProtection::Software]),
            AttachmentHintMask::default(),
            MdsCertLevel::L1Plus,
        );
        assert_forbidden(
            verify_mds_attestation(
                &mds,
                &Some(MdsCertLevel::L2),
                &Some(prot_mask(&[KeyProtection::Hardware])),
                &None,
            ),
            "FIDO Authenticator certification level too low",
        );
    }
}
