import type { UserValuesRequest } from '$api/types/user';

export interface NewUserRegistrationRequest {
    /// Validation: `email`
    email: string;
    /// Validation: `[user_values.preferred_username] -> regex_rust`
    preferred_username?: string;
    /// Validation: PATTERN_USER_NAME
    given_name?: string;
    /// Validation: PATTERN_USER_NAME
    family_name?: string;
    user_values?: UserValuesRequest;
    /// Validation: PATTERN_URI
    pow: string;
    /// Validation: PATTERN_URI
    redirect_uri?: string;
}
