export interface AtprotoLoginRequest {
    /// Validation: PATTERN_ATPROTO_ID
    at_id: string,
    pow: string,
    /// Validation: PATTERN_URI
    redirect_uri: string,
    /// Validation: PATTERN_URI
    state?: string,
    /// Validation: PATTERN_URI
    nonce?: string,
    /// Validation: PATTERN_CODE_CHALLENGE
    code_challenge?: string,
    /// Validation: `plain|S256`
    code_challenge_method?: CodeChallengeMethod,
    /// Validation: PATTERN_CODE_CHALLENGE
    pkce_challenge?: string,
}

export interface AtprotoCallbackRequest {
    /// Validation: PATTERN_ALNUM
    state: string,
    /// Validation: PATTERN_URI
    code: string,
    /// Validation: PATTERN_ALNUM
    iss: string,
    /// Validation: PATTERN_ALNUM
    xsrf_token: string,
    /// Validation: PATTERN_URI
    pkce_verifier: string,
}
