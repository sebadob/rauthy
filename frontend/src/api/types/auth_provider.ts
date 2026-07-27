import type { CodeChallengeMethod } from '$api/types/authorize.ts';

export type AuthProviderType = 'auto' | 'custom' | 'github' | 'google' | 'oidc';

export interface ProviderRequest {
    /// Validation: PATTERN_CLIENT_NAME
    name: string;
    typ: AuthProviderType;
    enabled: boolean;

    /// Validation: PATTERN_URI
    issuer: string;
    /// Validation: PATTERN_URI
    authorization_endpoint: string;
    /// Validation: PATTERN_URI
    token_endpoint: string;
    /// Validation: PATTERN_URI
    userinfo_endpoint: string;

    use_pkce: boolean;
    client_secret_basic: boolean;
    client_secret_post: boolean;
    auto_onboarding: boolean;
    auto_link: boolean;

    /// Validation: PATTERN_URI
    client_id: string;
    /// Validation: max 256
    client_secret?: string;
    /// Validation: PATTERN_SCOPE_SPACE
    scope: string;

    /// Validation: PATTERN_URI
    admin_claim_path?: string;
    /// Validation: PATTERN_URI
    admin_claim_value?: string;
    /// Validation: PATTERN_URI
    mfa_claim_path?: string;
    /// Validation: PATTERN_URI
    mfa_claim_value?: string;
}

export interface ProviderCallbackRequest {
    /// Validation: PATTERN_ALNUM
    state: string;
    /// Validation: PATTERN_URI
    code: string;
    /// Validation: PATTERN_ALNUM
    xsrf_token: string;
    /// Validation: PATTERN_URI
    pkce_verifier: string;

    /// Validation: PATTERN_ALNUM
    iss_atproto?: string;
}

export interface ProviderLoginRequest {
    // values for the downstream client
    /// Validation: `email`
    email?: string;
    /// Validation: PATTERN_CLIENT_ID
    client_id: string;
    /// Validation: PATTERN_URI
    redirect_uri: string;
    /// Validation: PATTERN_ROLE_SCOPE
    scopes?: string[];
    /// Validation: PATTERN_URI
    state?: string;
    /// Validation: PATTERN_URI
    nonce?: string;
    /// Validation: PATTERN_CODE_CHALLENGE
    code_challenge?: string;
    code_challenge_method?: CodeChallengeMethod;

    // values for the callback from upstream
    /// Validation: PATTERN_URI
    provider_id: string;
    /// Validation: PATTERN_URI
    pkce_challenge: string;

    pow: string;

    /// Validation: PATTERN_ATPROTO_ID
    handle?: string;
}

export interface ProviderLookupRequest {
    /// Validation: PATTERN_URI
    issuer?: string;
    /// Validation: PATTERN_URI
    metadata_url?: string;
}

export interface ProviderResponse {
    id: string;
    name: string;
    typ: AuthProviderType;
    enabled: boolean;
    issuer: string;
    authorization_endpoint: string;
    token_endpoint: string;
    userinfo_endpoint: string;
    client_id: string;
    client_secret?: string;
    scope: string;
    admin_claim_path?: string;
    admin_claim_value?: string;
    mfa_claim_path?: string;
    mfa_claim_value?: string;
    use_pkce: boolean;
    client_secret_basic: boolean;
    client_secret_post: boolean;
    auto_onboarding: boolean;
    auto_link: boolean;
}

export interface ProviderLinkedUserResponse {
    id: string;
    email: string;
}

export interface ProviderLookupResponse {
    issuer: string;
    authorization_endpoint: string;
    token_endpoint: string;
    userinfo_endpoint: string;
    scope: string;
    use_pkce: boolean;
    client_secret_basic: boolean;
    client_secret_post: boolean;
}
