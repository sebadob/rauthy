export type MfaPurpose =
    | { Login: string }
    | 'MfaModToken'
    | 'PasswordNew'
    | 'PasswordReset'
    | 'Test';
export type WebauthnAdditionalData = undefined | WebauthnLoginFinishResponse | WebauthnServiceReq;

export interface WebauthnAuthStartRequest {
    purpose: MfaPurpose;
}

export interface WebauthnAuthFinishRequest {
    code: string;
    data: PublicKeyCredential;
}

export interface WebauthnAuthStartResponse {
    code: string;
    rcr: WebauthnRequestChallengeResponse;
    user_id: string;
    exp: number;
}

export interface WebauthnLoginFinishResponse {
    loc: String;
}

export interface WebauthnServiceReq {
    code: string;
    user_id: string;
}

export interface WebauthnRegStartRequest {
    /// Validation: PATTERN_USER_NAME / 32
    passkey_name: string;
    /// Validation: PATTERN_ALNUM_64
    magic_link_id?: string;
    /// Validation: length === 32
    mfa_mod_token_id?: string;
}

export interface WebauthnRegFinishRequest {
    /// Validation: RE_USER_NAME / 32
    passkey_name: string;
    data: RegisterPublicKeyCredential;
    /// Validation: RE_ALNUM_64
    magic_link_id?: string;
}

// ######################################################################
// The following types all match the `webauthn-rs` types from the backend.
// Most of them exist in the Browser context already, but with ArrayBuffer
// types instead of Base64UrlSafeData for backend communication.
// We need to convert and translate them between backend and browser,
// because all data will be sent as JSON which does not support bytes.
// ######################################################################

type AuthenticatorTransport = 'Usb' | 'Nfc' | 'Ble' | 'Internal' | 'Hybrid' | 'Test' | 'Unknown';
type AttestationFormat =
    | 'packed'
    | 'Packed'
    | 'tpm'
    | 'Tpm'
    | 'TPM'
    | 'android-key'
    | 'AndroidKey'
    | 'android-safetynet'
    | 'AndroidSafetyNet'
    | 'AndroidSafetynet'
    | 'fido-u2f'
    | 'FIDOU2F'
    | 'apple'
    | 'AppleAnonymous'
    | 'none'
    | 'None';
type Base64UrlSafeData = string;
type CredentialProtectionPolicy =
    | 'UserVerificationOptional'
    | 'UserVerificationOptionalWithCredentialIDList'
    | 'UserVerificationRequired';
type Mediation = 'Conditional';
type UserVerificationPolicy = 'required' | 'preferred' | 'discouraged';
type PublicKeyCredentialHints = 'securitykey' | 'clientdevice' | 'hybrid';

export interface WebauthnRequestChallengeResponse {
    publicKey: PublicKeyCredentialRequestOptions;
    mediation?: Mediation;
}

export interface PublicKeyCredentialRequestOptions {
    challenge: Base64UrlSafeData;
    timeout?: number;
    rp_id: string;
    allow_credentials: AllowCredentials[];
    user_verification: UserVerificationPolicy;
    hints?: PublicKeyCredentialHints[];
    extensions?: RequestAuthenticationExtensions;
}

export interface AllowCredentials {
    type: string;
    id: Base64UrlSafeData;
    transports?: AuthenticatorTransport[];
}

export interface RequestAuthenticationExtensions {
    appid?: string;
    /// ⚠️  - Browsers do not support this!
    uvm?: boolean;
    /// ⚠️  - Browsers do not support this!
    /// <https://bugs.chromium.org/p/chromium/issues/detail?id=1023225>
    hmac_get_secret?: HmacGetSecretInput;
}

export interface HmacGetSecretInput {
    /// Retrieve a symmetric secrets from the authenticator with this input.
    output1: Base64UrlSafeData;
    /// Rotate the secret in the same operation.
    output2?: Base64UrlSafeData;
}

export interface PublicKeyCredential {
    id: string;
    rawId: Base64UrlSafeData;
    response: AuthenticatorAssertionResponseRaw;
    extensions: AuthenticationExtensionsClientOutputs;
    type: string;
}

export interface AuthenticatorAssertionResponseRaw {
    authenticatorData: Base64UrlSafeData;
    clientDataJSON: Base64UrlSafeData;
    signature: Base64UrlSafeData;
    userHandle?: Base64UrlSafeData;
}

export interface CreationChallengeResponse {
    publicKey: PublicKeyCredentialCreationOptions;
}

export interface PublicKeyCredentialCreationOptions {
    rp: RelyingParty;
    user: User;
    challenge: Base64UrlSafeData;
    pubKeyCredParams: PubKeyCredParams[];
    timeout?: number;
    excludeCredentials?: PublicKeyCredentialDescriptor[];
    authenticatorSelection?: AuthenticatorSelectionCriteria;
    hints?: PublicKeyCredentialHints[];
    attestation?: AttestationConveyancePreference;
    attestationFormats?: AttestationFormat[];
    extensions?: RequestRegistrationExtensions;
}

export interface RelyingParty {
    name: string;
    id: string;
}

export interface User {
    /// The user's id in base64 form. This MUST be a unique id, and
    /// must NOT contain personally identifying information, as this value can NEVER
    /// be changed.
    id: Base64UrlSafeData;
    /// A detailed name for the account, such as an email address. This value
    /// **can** change, so **must not** be used as a primary key.
    name: string;
    /// The user's preferred name for display. This value **can** change, so
    /// **must not** be used as a primary key.
    display_name: string;
}

export interface PubKeyCredParams {
    type: string;
    alg: number;
}

export interface PublicKeyCredentialDescriptor {
    type: string;
    id: Base64UrlSafeData;
    transports?: AuthenticatorTransport[];
}

export interface RequestRegistrationExtensions {
    credProtect?: CredProtect;
    /// ⚠️  - Browsers do not support this!
    uvm?: boolean;
    /// ⚠️  - This extension result is always unsigned, and only indicates if the
    /// browser *requests* a residentKey to be created. It has no bearing on the
    /// true rk state of the credential.
    credProps?: boolean;
    /// CTAP2.1 Minumum pin length
    minPinLength?: boolean;
    /// ⚠️  - Browsers support the *creation* of the secret, but not the retrieval of it.
    /// CTAP2.1 create hmac secret
    hmacCreateSecret?: boolean;
}

export interface CredProtect {
    credentialProtectionPolicy: CredentialProtectionPolicy;
    /// Whether it is better for the authenticator to fail to create a
    /// credential rather than ignore the protection policy
    /// If no value is provided, the client treats it as `false`.
    enforceCredentialProtectionPolicy?: boolean;
}

export interface RegisterPublicKeyCredential {
    /// The id of the PublicKey credential, likely in base64.
    /// This is NEVER actually used in a real registration, because the true
    /// credential ID is taken from the attestation data.
    id: string;
    /// The id of the credential, as binary.
    ///
    /// This is NEVER actually used in a real registration, because the true
    // credential ID is taken from the attestation data.
    rawId: Base64UrlSafeData;
    /// <https://w3c.github.io/webauthn/#dom-publickeycredential-response>
    response: AuthenticatorAttestationResponseRaw;
    type: string;
    extensions: RegistrationExtensionsClientOutputs;
}

export interface AuthenticatorAttestationResponseRaw {
    /// <https://w3c.github.io/webauthn/#dom-authenticatorattestationresponse-attestationobject>
    attestationObject: Base64UrlSafeData;
    /// <https://w3c.github.io/webauthn/#dom-authenticatorresponse-clientdatajson>
    clientDataJSON: Base64UrlSafeData;
    /// <https://w3c.github.io/webauthn/#dom-authenticatorattestationresponse-gettransports>
    transports?: AuthenticatorTransport[];
}

export interface RegistrationExtensionsClientOutputs {
    /// Indicates whether the client used the provided appid extension
    appid?: boolean;
    /// Indicates if the client believes it created a resident key. This
    /// property is managed by the webbrowser, and is NOT SIGNED and CAN NOT be trusted!
    cred_props?: CredProps;
    /// Indicates if the client successfully applied a HMAC Secret
    hmac_secret?: boolean;
    /// Indicates if the client successfully applied a credential protection policy.
    cred_protect?: CredentialProtectionPolicy;
    /// Indicates the current minimum PIN length
    min_pin_length?: number;
}

export interface CredProps {
    /// A user agent supplied hint that this credential *may* have created a resident key. It is
    /// retured from the user agent, not the authenticator meaning that this is an unreliable
    /// signal.
    ///
    /// Note that this extension is UNSIGNED and may have been altered by page javascript.
    rk: boolean;
}
