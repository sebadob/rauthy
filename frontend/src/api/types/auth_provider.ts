export interface ProviderLoginRequest {
    // values for the downstream client
    /// Validation: `email`
    email?: string,
    /// Validation: `^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]{2,128}$`
    client_id: string,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    redirect_uri: string,
    /// Validation: `Vec<^[a-z0-9-_/,:*]{2,64}$>`
    scopes?: string[],
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    state?: string,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    nonce?: string,
    /// Validation: `[a-zA-Z0-9-._~]{43,128}`
    code_challenge?: string,
    /// Validation: `[a-zA-Z0-9]`
    code_challenge_method?: string,

    // values for the callback from upstream
    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    provider_id: string,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    pkce_challenge: string,
}