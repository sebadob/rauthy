use crate::entity::users::AccountType;
use serde::{Deserialize, Serialize};
use webauthn_rs::prelude::{
    AuthenticationResult, Credential, CredentialID, Passkey, PasskeyAuthentication,
    PasskeyRegistration, SecurityKey, SecurityKeyAuthentication, SecurityKeyRegistration, Uuid,
    Webauthn, WebauthnResult,
};
use webauthn_rs_proto::{
    CreationChallengeResponse, PublicKeyCredential, RegisterPublicKeyCredential,
    RequestChallengeResponse, ResidentKeyRequirement,
};

pub(super) fn requires_uv(account_type: AccountType, force_uv: bool) -> bool {
    force_uv
        || !matches!(
            account_type,
            AccountType::Password | AccountType::FederatedPassword
        )
}

// Keep the ceremony type in the server-side cache so finish uses the policy chosen at start.
#[derive(Serialize, Deserialize)]
pub(super) enum RegistrationState {
    Passkey(PasskeyRegistration),
    SecurityKey(SecurityKeyRegistration),
}

impl RegistrationState {
    pub(super) fn start(
        webauthn: &Webauthn,
        user_id: Uuid,
        email: &str,
        exclude_credentials: Option<Vec<CredentialID>>,
        require_uv: bool,
        allow_rk: bool,
    ) -> WebauthnResult<(CreationChallengeResponse, Self)> {
        let (mut ccr, state) = if require_uv {
            let (ccr, state) =
                webauthn.start_passkey_registration(user_id, email, email, exclude_credentials)?;
            (ccr, Self::Passkey(state))
        } else {
            let (mut ccr, state) = webauthn.start_securitykey_registration(
                user_id,
                email,
                email,
                exclude_credentials,
                None,
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
            (ccr, Self::SecurityKey(state))
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

    pub(super) fn finish(
        &self,
        webauthn: &Webauthn,
        credential: &RegisterPublicKeyCredential,
    ) -> WebauthnResult<Passkey> {
        match self {
            Self::Passkey(state) => webauthn.finish_passkey_registration(credential, state),
            Self::SecurityKey(state) => webauthn
                .finish_securitykey_registration(credential, state)
                .map(|key| Passkey::from(Credential::from(key))),
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
    use webauthn_rs::prelude::{Url, WebauthnBuilder};
    use webauthn_rs_proto::{CredentialProtectionPolicy, UserVerificationPolicy};

    fn webauthn() -> Webauthn {
        let origin = Url::parse("https://localhost:8080").unwrap();
        WebauthnBuilder::new("localhost", &origin)
            .unwrap()
            .build()
            .unwrap()
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
