use crate::entity::users::AccountType;
use serde::{Deserialize, Serialize};
use webauthn_rs::prelude::{
    AttestationCaList, AttestedPasskeyRegistration, AuthenticationResult, Credential, CredentialID,
    Passkey, PasskeyAuthentication, PasskeyRegistration, SecurityKey, SecurityKeyAuthentication,
    SecurityKeyRegistration, Uuid, Webauthn, WebauthnResult,
};
use webauthn_rs_proto::{
    CreationChallengeResponse, PublicKeyCredential, RegisterPublicKeyCredential,
    RequestChallengeResponse, ResidentKeyRequirement,
};

#[inline]
pub(super) fn requires_uv(account_type: AccountType, force_uv: bool) -> bool {
    force_uv
        || !matches!(
            account_type,
            AccountType::Password | AccountType::FederatedPassword
        )
}

// Keep the ceremony type in the server-side cache so finish uses the policy chosen at start.
#[derive(Debug, Serialize, Deserialize)]
pub(super) enum RegistrationState {
    Passkey(PasskeyRegistration),
    SecurityKey(SecurityKeyRegistration),
    AttestedPasskey(AttestedPasskeyRegistration),
    AttestedSecurityKey(SecurityKeyRegistration),
}

impl RegistrationState {
    pub(super) fn start(
        webauthn: &Webauthn,
        user_id: Uuid,
        email: &str,
        exclude_credentials: Option<Vec<CredentialID>>,
        require_uv: bool,
        allow_rk: bool,
        attestation_ca_list: Option<AttestationCaList>,
    ) -> WebauthnResult<(CreationChallengeResponse, Self)> {
        let (mut ccr, state) = match (require_uv, attestation_ca_list) {
            // With a CA list and UV required, use the dedicated passkey API. It enforces Direct
            // conveyance, a strict credProtect policy, and verifies the returned chain against
            // the MDS roots at finish time.
            (true, Some(ca_list)) => {
                let (ccr, state) = webauthn.start_attested_passkey_registration(
                    user_id,
                    email,
                    email,
                    exclude_credentials,
                    ca_list,
                    // None provides the most amount of compat. We should probably depend on
                    // `force_passkey_attachment` though.
                    None,
                )?;
                (ccr, Self::AttestedPasskey(state))
            }
            (true, None) => {
                let (ccr, state) = webauthn.start_passkey_registration(
                    user_id,
                    email,
                    email,
                    exclude_credentials,
                )?;
                (ccr, Self::Passkey(state))
            }
            (false, ca_list) => {
                let with_attestation = ca_list.is_some();

                // Without UV, the security key API is used with or without a CA list. A non-empty
                // list switches it to Direct conveyance and verifies the chain at finish time.
                let (mut ccr, state) = webauthn.start_securitykey_registration(
                    user_id,
                    email,
                    email,
                    exclude_credentials,
                    ca_list,
                    // None provides the most amount of compat. We should probably depend on
                    // `force_passkey_attachment` though.
                    None,
                )?;
                // webauthn-rs 0.5.5 requests UV-required credProtect even with UV preferred.
                // Chromium rejects that combination. This optional extension is not enforced
                // by the library; omit it for password-backed credentials.
                // https://github.com/kanidm/webauthn-rs/issues/490
                if let Some(extensions) = ccr.public_key.extensions.as_mut() {
                    extensions.cred_protect = None;
                }
                // Allow platform and hybrid authenticators as well as hardware security keys.
                ccr.public_key.hints = None;

                if with_attestation {
                    (ccr, Self::AttestedSecurityKey(state))
                } else {
                    (ccr, Self::SecurityKey(state))
                }
            }
        };

        // Resident keys are optional in both APIs. Preserve their UV policy and only change
        // the resident-key preference; neither choice requires attestation or residency.
        if let Some(selection) = ccr.public_key.authenticator_selection.as_mut() {
            selection.resident_key = Some(if allow_rk {
                ResidentKeyRequirement::Preferred
            } else {
                ResidentKeyRequirement::Discouraged
            });
        }
        Ok((ccr, state))
    }

    /// A copy of this state that finishes without an attestation CA list, keeping the underlying
    /// registration state and challenge unchanged. Attested ceremonies are re-wrapped as their
    /// plain counterparts; plain ceremonies round-trip unchanged.
    pub(super) fn into_plain_fallback(self) -> Self {
        let json_self = match self {
            // The plain passkey ceremony has no ca_list field at all.
            Self::AttestedPasskey(reg) => {
                let json = serde_json::to_value(reg).unwrap();
                let rs = json
                    .get("rs")
                    .expect("`rs` to be in AttestedPasskeyRegistration");
                serde_json::json!({ "Passkey": { "rs": rs } })
            }
            // The plain security-key ceremony carries a null ca_list.
            Self::AttestedSecurityKey(reg) => {
                let json = serde_json::to_value(reg).unwrap();
                let rs = json
                    .get("rs")
                    .expect("`rs` to be in SecurityKeyRegistration");
                serde_json::json!({
                    "SecurityKey": { "rs": rs, "ca_list": serde_json::Value::Null }
                })
            }
            _ => return self,
        };
        serde_json::from_value(json_self).expect("RegistrationState serialises")
    }

    pub(super) fn finish(
        &self,
        webauthn: &Webauthn,
        credential: &RegisterPublicKeyCredential,
    ) -> WebauthnResult<Passkey> {
        match self {
            Self::Passkey(state) => webauthn.finish_passkey_registration(credential, state),
            Self::SecurityKey(state) | Self::AttestedSecurityKey(state) => webauthn
                .finish_securitykey_registration(credential, state)
                .map(|key| Passkey::from(Credential::from(key))),
            Self::AttestedPasskey(state) => webauthn
                .finish_attested_passkey_registration(credential, state)
                .map(Passkey::from),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) enum AuthenticationState {
    Passkey(PasskeyAuthentication),
    SecurityKey(SecurityKeyAuthentication),
}

impl AuthenticationState {
    pub(super) fn start(
        webauthn: &Webauthn,
        passkeys: Vec<Passkey>,
        require_uv: bool,
    ) -> WebauthnResult<(RequestChallengeResponse, Self)> {
        if require_uv {
            webauthn
                .start_passkey_authentication(&passkeys)
                .map(|(rcr, state)| (rcr, Self::Passkey(state)))
        } else {
            let keys = passkeys
                .into_iter()
                .map(|key| SecurityKey::from(Credential::from(key)))
                .collect::<Vec<_>>();
            webauthn
                .start_securitykey_authentication(&keys)
                .map(|(mut rcr, state)| {
                    rcr.public_key.hints = None;
                    (rcr, Self::SecurityKey(state))
                })
        }
    }

    pub(super) fn finish(
        &self,
        webauthn: &Webauthn,
        credential: &PublicKeyCredential,
    ) -> WebauthnResult<AuthenticationResult> {
        match self {
            Self::Passkey(state) => webauthn.finish_passkey_authentication(credential, state),
            Self::SecurityKey(state) => {
                webauthn.finish_securitykey_authentication(credential, state)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde::de::DeserializeOwned;
    use std::collections::BTreeMap;
    use webauthn_rs::prelude::{AttestationCaListBuilder, Url, WebauthnBuilder};
    use webauthn_rs_proto::{CredentialProtectionPolicy, UserVerificationPolicy};

    fn webauthn() -> Webauthn {
        let origin = Url::parse("https://localhost:8080").unwrap();
        WebauthnBuilder::new("localhost", &origin)
            .unwrap()
            .build()
            .unwrap()
    }

    // This test is important to catch any major changes inside webauthn-rs, since we need to do
    // a json workaround to make optimistic attestation work.
    #[test]
    fn plain_fallback_workaround() {
        let webauthn = webauthn();

        let cert_hex = "308202DE3082023FA003020102020600EAB4000002300A06082A8648CE3D040304308191310B3009060355040613025553310B300906035504080C025641310F300D06035504070C06526573746F6E312D302B060355040A0C244944454D4941204964656E7469747920616E6420536563757269747920555341204C4C433135303306035504030C2C4944454D4941204964656E7469747920616E6420536563757269747920555341204C4C4320526F6F742043413020170D3234303931383232303030305A180F32303634303931393231353935395A308191310B3009060355040613025553310B300906035504080C025641310F300D06035504070C06526573746F6E312D302B060355040A0C244944454D4941204964656E7469747920616E6420536563757269747920555341204C4C433135303306035504030C2C4944454D4941204964656E7469747920616E6420536563757269747920555341204C4C4320526F6F7420434130819B301006072A8648CE3D020106052B8104002303818600040095C4D8B025762F1BB02BC4393CAFB4DFFC1200F4A941947A935D8FDA9A9F075A7B3372547D4C2F7A68ADA2128963611EA4F2DA7488B0CF68156AB3C1E8C15CFC7E01466BF9C9C518856C09C8F8917EA14942B7273ABD90986526EE7FC666140A573E6EAFC2FB34EA67A27BF172E85C7AE8DE7B5C47A4853E5ABC82A3686935F5E86222A33C303A301D0603551D0E041604143B56394BDDA8124D7BD5005C8A6F818811CA74C3300B0603551D0F0404030200FF300C0603551D13040530030101FF300A06082A8648CE3D04030403818C003081880242013E7DB6C915C5E21604196D9AF7C507545929E083584DAE41C51E1E50E911075C71176B2C7E2267F4D223E8369004E9CF660924D579606C18EA834B8077E91550A1024201B5F0303FA5D45FCBB314417B8FB4307870AF6D2FEA1926FACD8A39552BDA7F88033D9C82108294B3937575EF406978EB9DD75F7BF05FD569832452513D6D8627B1";
        let cert = hex::decode(cert_hex).unwrap();

        let mut builder = AttestationCaListBuilder::new();
        builder
            .insert_device_der(
                &cert,
                Uuid::new_v4(),
                "some description".to_string(),
                BTreeMap::new(),
            )
            .unwrap();
        let ca_list = builder.build();

        let (_ccr, state) = webauthn
            .start_attested_passkey_registration(
                Uuid::new_v4(),
                "batman@batcave.gotham",
                "Batman",
                None,
                ca_list.clone(),
                None,
            )
            .unwrap();
        // panics if it does not work
        RegistrationState::AttestedPasskey(state).into_plain_fallback();

        let (_ccr, state) = webauthn
            .start_securitykey_registration(
                Uuid::new_v4(),
                "batman@batcave.gotham",
                "Batman",
                None,
                Some(ca_list),
                None,
            )
            .unwrap();
        // panics if it does not work
        RegistrationState::AttestedSecurityKey(state).into_plain_fallback();
    }

    fn roundtrip<T: Serialize + DeserializeOwned>(value: &T) -> T {
        serde_json::from_str(&serde_json::to_string(value).unwrap()).unwrap()
    }

    #[rstest]
    #[case(AccountType::Password, false)]
    #[case(AccountType::FederatedPassword, false)]
    #[case(AccountType::New, true)]
    #[case(AccountType::Passkey, true)]
    #[case(AccountType::Federated, true)]
    #[case(AccountType::FederatedPasskey, true)]
    fn account_policy(#[case] account_type: AccountType, #[case] expected: bool) {
        assert_eq!(requires_uv(account_type.clone(), false), expected);
        assert!(requires_uv(account_type, true));
    }

    #[rstest]
    fn registration_options(
        #[values(false, true)] require_uv: bool,
        #[values(false, true)] allow_rk: bool,
    ) {
        let webauthn = webauthn();
        let user_id = Uuid::new_v4();
        let excluded = vec![vec![1, 2, 3].into()];
        let (ccr, state) = RegistrationState::start(
            &webauthn,
            user_id,
            "user@localhost",
            Some(excluded.clone()),
            require_uv,
            allow_rk,
            None,
        )
        .unwrap();
        let pk = &ccr.public_key;
        let selection = pk.authenticator_selection.as_ref().unwrap();
        let expected_policy = if require_uv {
            UserVerificationPolicy::Required
        } else {
            UserVerificationPolicy::Preferred
        };
        assert_eq!(selection.user_verification, expected_policy);
        assert_eq!(
            selection.resident_key,
            Some(if allow_rk {
                ResidentKeyRequirement::Preferred
            } else {
                ResidentKeyRequirement::Discouraged
            })
        );
        assert!(!selection.require_resident_key);
        assert!(selection.authenticator_attachment.is_none());
        assert!(pk.hints.is_none());
        assert_eq!(pk.exclude_credentials.as_ref().unwrap()[0].id, excluded[0]);
        assert_eq!(pk.user.id.as_ref(), user_id.as_bytes());

        let extensions = pk.extensions.as_ref().unwrap();
        if require_uv {
            assert_eq!(
                extensions
                    .cred_protect
                    .as_ref()
                    .unwrap()
                    .credential_protection_policy,
                CredentialProtectionPolicy::UserVerificationRequired
            );
        } else {
            assert!(extensions.cred_protect.is_none());
        }
        assert_eq!(extensions.cred_props, Some(true));
        // Check the cached policy as well as the browser request: the original bug changed
        // only the latter, leaving Required in the server's registration state.
        let state = serde_json::to_value(roundtrip(&state)).unwrap();
        let state = &state[if require_uv { "Passkey" } else { "SecurityKey" }];
        assert_eq!(
            state["rs"]["policy"],
            serde_json::to_value(expected_policy).unwrap()
        );
    }
}
