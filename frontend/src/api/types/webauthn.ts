export interface PasskeyResponse {
    name: string,
    /// Unix timestamp in seconds
    registered: number,
    /// Unix timestamp in seconds
    last_used: number,
    user_verified?: boolean,
}

export interface WebauthnDeleteRequest {
    /// 32 chars long MfaModToken.id
    mfa_mod_token_id?: string,
}