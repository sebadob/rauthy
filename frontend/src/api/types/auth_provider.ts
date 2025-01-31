import type {CodeChallengeMethod} from "$api/types/authorize.ts";

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