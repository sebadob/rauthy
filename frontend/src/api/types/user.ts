import type {Language} from "./i18n";
import {PATTERN_ALNUM} from "$utils/patterns";

export interface UserPictureConfig {
    upload_allowed: boolean,
    content_len_limit: number,
}

export interface NewUserRequest {
    /// Validation: `email`
    email: string,
    /// Validation: PATTERN_USER_NAME
    given_name: string,
    /// Validation: PATTERN_USER_NAME
    family_name?: string,
    language: Language,
    /// Validation: PATTERN_GROUP
    groups?: string[],
    /// Validation: PATTERN_ROLE_SCOPE
    roles: string[],
    /// Unix timestamp in seconds
    user_expires?: number,
}

export interface UserValuesRequest {
    // Validation: PATTERN_DATE_STR
    birthdate?: string | null,
    // Validation: PATTERN_PHONE / 32
    phone?: string | null,
    // Validation: PATTERN_STREET / 48
    street?: string | null,
    // Validation: PATTERN_ALNUM / 24
    zip?: string | null,
    // Validation: PATTERN_CITY / 48
    city?: string | null,
    // Validation: PATTERN_CITY / 48
    country?: string | null,
}

export interface UpdateUserRequest {
    /// Validation: `email`
    email: string,
    /// Validation: PATTERN_USER_NAME
    given_name: string,
    /// Validation: PATTERN_USER_NAME
    family_name?: string,
    language?: Language,
    /// Validation: Applies password policy - max 256 characters
    password?: string,
    /// Validation: PATTERN_ROLE_SCOPE
    roles: string[],
    /// Validation: PATTERN_GROUP
    groups?: string[],
    enabled: boolean,
    allow_pam_logins: boolean,
    email_verified: boolean,
    /// Unix timestamp in seconds
    user_expires?: number,
    user_values?: UserValuesRequest,
}

export interface UpdateUserSelfRequest {
    // Validation: email
    email?: string | null,
    // Validation: PATTERN_USER_NAME / 32
    given_name?: string | null,
    // Validation: PATTERN_USER_NAME / 32
    family_name?: string | null,
    language?: Language,
    password_current?: string | null,
    mfa_code?: string | null,
    password_new?: string | null,
    user_values?: UserValuesRequest,
}

export interface UserAttrValueRequest {
    /// Validation: PATTERN_ATTR
    key: string,
    value: string,
}

export interface UserAttrValuesUpdateRequest {
    values: UserAttrValueRequest[],
}

export type UserAccountTypeResponse =
    'new'
    | 'password'
    | 'passkey'
    | 'federated'
    | 'federated_passkey'
    | 'federated_password';

export interface UserResponseSimple {
    id: string,
    email: string,
    given_name: string,
    family_name?: string,
    created_at: number,
    last_login?: number,
    picture_id?: string,
}

export interface UserResponse {
    id: string,
    email: string,
    given_name: string,
    family_name?: string,
    language: Language,
    roles: string[],
    groups?: string[],
    enabled: boolean,
    allow_pam_logins: boolean,
    email_verified: boolean,
    password_expires?: number,
    created_at: number,
    last_login?: number,
    last_failed_login?: number,
    failed_login_attempts?: number,
    user_expires?: number,
    account_type: UserAccountTypeResponse,
    webauthn_user_id?: string,
    user_values: UserValuesResponse,
    auth_provider_id?: string,
    federation_uid?: string,
    picture_id?: string,
}

export interface UserValuesResponse {
    birthdate?: string,
    phone?: string,
    street?: string,
    zip?: string,
    city?: string,
    country?: string,
}
