export interface UserMfaTokenRequest {
    /// Validation: max length 256
    password: string,
}

export interface MfaModTokenResponse {
    id: string,
    user_id: string,
    exp: number,
    ip: string,
}
