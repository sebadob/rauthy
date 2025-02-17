import type {Language} from "./language";

export type UserAccountTypeResponse =
    'new'
    | 'password'
    | 'passkey'
    | 'federated'
    | 'federated_passkey'
    | 'federated_password';

export interface UserResponse {
    id: string,
    email: string,
    given_name: string,
    family_name?: string,
    language: Language,
    roles: string[],
    groups?: string[],
    enabled: boolean,
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
}

export interface UserValuesResponse {
    birthdate?: string,
    phone?: string,
    street?: string,
    zip?: number,
    city?: string,
    country?: string,
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

export interface UserValuesRequest {
    // Validation: PATTERN_DATE_STR
    birthdate?: string | null,
    // Validation: PATTERN_PHONE / 32
    phone?: string | null,
    // Validation: PATTERN_STREET / 48
    street?: string | null,
    // Validation: min 1000 max 9999999
    zip?: number | null,
    // Validation: PATTERN_CITY / 48
    city?: string | null,
    // Validation: PATTERN_CITY / 48
    country?: string | null,
}