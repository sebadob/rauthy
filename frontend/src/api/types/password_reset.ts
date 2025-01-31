export interface PasswordResetRequest {
    /// Validation: PATTERN_ALNUM_64
    magic_link_id: string,
    /// Validation: Applies password policy - max 256 characters
    password: string,
    /// Validation: PATTERN_ALNUM_48
    mfa_code?: string,
}