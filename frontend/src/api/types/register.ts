export interface NewUserRegistrationRequest {
    /// Validation: `email`
    email: string,
    /// Validation: PATTERN_USER_NAME
    given_name: string,
    /// Validation: PATTERN_USER_NAME
    family_name?: string,
    /// Validation: PATTERN_URI
    pow: string,
    /// Validation: PATTERN_URI
    redirect_uri?: string,
}