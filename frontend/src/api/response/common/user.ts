import type {Language} from "$api/response/common/language.ts";

export type UserAccountTypeResponse =
    'New'
    | 'Password'
    | 'Passkey'
    | 'Federated'
    | 'FederatedPasskey'
    | 'FederatedPassword';

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
