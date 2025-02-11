import type {CodeChallengeMethod} from "$api/types/authorize.ts";

export type AuthProviderType = 'Custom' | 'Github' | 'Google' | 'OIDC';

export interface ProviderRequest {
    /// Validation: PATTERN_CLIENT_NAME
    name: string,
    typ: AuthProviderType,
    enabled: boolean,

    /// Validation: PATTERN_URI
    issuer: string,
    /// Validation: PATTERN_URI
    authorization_endpoint: string,
    /// Validation: PATTERN_URI
    token_endpoint: string,
    /// Validation: PATTERN_URI
    userinfo_endpoint: string,

    danger_allow_insecure?: boolean,
    use_pkce: boolean,
    client_secret_basic: boolean,
    client_secret_post: boolean,

    /// Validation: PATTERN_URI
    client_id: string,
    /// Validation: max 256
    client_secret?: string,
    /// Validation: PATTERN_SCOPE_SPACE
    scope: string,
    /// Validation: PATTERN_PEM
    root_pem?: string,

    /// Validation: PATTERN_URI
    admin_claim_path?: string,
    /// Validation: PATTERN_URI
    admin_claim_value?: string,
    /// Validation: PATTERN_URI
    mfa_claim_path?: string,
    /// Validation: PATTERN_URI
    mfa_claim_value?: string,
}

export interface ProviderCallbackRequest {
    /// Validation: PATTERN_ALNUM
    state: string,
    /// Validation: PATTERN_URI
    code: string,
    /// Validation: PATTERN_ALNUM
    xsrf_token: string,
    /// Validation: PATTERN_URI
    pkce_verifier: string,
}

export interface ProviderLoginRequest {
    // values for the downstream client
    /// Validation: `email`
    email?: string,
    /// Validation: PATTERN_CLIENT_ID_EPHEMERAL
    client_id: string,
    /// Validation: PATTERN_URI
    redirect_uri: string,
    /// Validation: PATTERN_GROUP
    scopes?: string[],
    /// Validation: PATTERN_URI
    state?: string,
    /// Validation: PATTERN_URI
    nonce?: string,
    /// Validation: PATTERN_CODE_CHALLENGE
    code_challenge?: string,
    code_challenge_method?: CodeChallengeMethod,

    // values for the callback from upstream
    /// Validation: PATTERN_URI
    provider_id: string,
    /// Validation: PATTERN_URI
    pkce_challenge: string,
}

export interface ProviderResponse {
    id: string,
    name: string,
    typ: AuthProviderType,
    enabled: boolean,
    issuer: string,
    authorization_endpoint: string,
    token_endpoint: string,
    userinfo_endpoint: string,
    client_id: string,
    client_secret?: string,
    scope: string,
    admin_claim_path?: string,
    admin_claim_value?: string,
    mfa_claim_path?: string,
    mfa_claim_value?: string,
    danger_allow_insecure: boolean,
    use_pkce: boolean,
    client_secret_basic: boolean,
    client_secret_post: boolean,
    root_pem?: string,
}

export interface ProviderLinkedUserResponse {
    id: string,
    email: string,
}
