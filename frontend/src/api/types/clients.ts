import type { JwkKeyPairAlg } from './oidc';

export const AuthFlowDeviceCode = 'urn:ietf:params:oauth:grant-type:device_code';
export type AuthFlow =
    | 'authorization_code'
    | 'client_credentials'
    | 'password'
    | 'refresh_token'
    | 'urn:ietf:params:oauth:grant-type:device_code';
export type CodeChallengeMethod = 'plain' | 'S256';

export interface NewClientRequest {
    /// Validation: PATTERN_LOWERCASE
    id: string;
    /// Validation: PATTERN_CLIENT_NAME
    name?: string;
    confidential: boolean;
    /// Validation: PATTERN_URI
    redirect_uris: string[];
    /// Validation: PATTERN_URI
    post_logout_redirect_uris?: string[];
}

export interface ScimClientRequestResponse {
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    bearer_token: string;
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    base_uri: string;
    sync_groups: boolean;
    /// Validation: `^[a-z0-9-_/,:*]{2,64}$`
    group_sync_prefix?: string;
}

export interface UpdateClientRequest {
    /// Validation: PATTERN_CLIENT_ID_EPHEMERAL
    id: string;
    /// Validation: PATTERN_CLIENT_NAME
    name?: string;
    confidential: boolean;
    /// Validation: PATTERN_URI
    redirect_uris: string[];
    /// Validation: PATTERN_URI
    post_logout_redirect_uris?: string[];
    /// Validation: PATTERN_ORIGIN
    allowed_origins?: string[];
    enabled: boolean;
    flows_enabled: AuthFlow[];
    access_token_alg: JwkKeyPairAlg;
    id_token_alg: JwkKeyPairAlg;
    /// Validation: `10 <= auth_code_lifetime <= 300`
    auth_code_lifetime: number;
    /// Validation: `10 <= access_token_lifetime <= 86400`
    access_token_lifetime: number;
    /// Validation: `PATTERN_ROLE_SCOPE
    scopes: string[];
    /// Validation: PATTERN_ROLE_SCOPE
    default_scopes: string[];
    /// Validation: `Vec<^(plain|S256)$>`
    challenges?: CodeChallengeMethod[];
    force_mfa: boolean;
    /// Validation: PATTERN_URI
    client_uri?: string;
    /// Validation: PATTERN_CONTACT
    contacts?: string[];
    /// Validation: PATTERN_URI
    backchannel_logout_uri?: string;
    /// Validation: PATTERN_GROUP
    restrict_group_prefix?: string;
    scim?: ScimClientRequestResponse;
}

export interface ClientSecretRequest {
    /// Validation: Value between 1 and 24
    cache_current_hours?: number;
}

export interface ClientResponse {
    id: string;
    name?: string;
    enabled: boolean;
    confidential: boolean;
    redirect_uris: string[];
    post_logout_redirect_uris?: string[];
    allowed_origins?: string[];
    flows_enabled: string[];
    access_token_alg: JwkKeyPairAlg;
    id_token_alg: JwkKeyPairAlg;
    auth_code_lifetime: number;
    access_token_lifetime: number;
    scopes: string[];
    default_scopes: string[];
    challenges?: string[];
    force_mfa: boolean;
    client_uri?: string;
    contacts?: string[];
    backchannel_logout_uri?: string;
    restrict_group_prefix?: string;
    scim?: ScimClientRequestResponse;
}

export interface ClientSecretResponse {
    id: string;
    confidential: boolean;
    secret?: string;
}
