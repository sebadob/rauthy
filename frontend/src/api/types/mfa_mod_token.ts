export interface UserMfaTokenRequest {
    /// Validation: max length 256
    password?: string,
    /// 48 character mfa code from WebauthnServiceRequest
    mfa_code?: string,
}

export interface MfaModTokenResponse {
    id: string,
    user_id: string,
    exp: number,
    ip: string,
}
