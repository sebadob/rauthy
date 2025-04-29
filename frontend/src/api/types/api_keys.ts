export type AccessGroup = 'Blacklist'
    | 'Clients'
    | 'Events'
    | 'Generic'
    | 'Groups'
    | 'Roles'
    | 'Secrets'
    | 'Sessions'
    | 'Scopes'
    | 'UserAttributes'
    | 'Users';
export type AccessRight = 'read' | 'create' | 'update' | 'delete';

export interface ApiKeyAccess {
    group: AccessGroup,
    access_rights: AccessRight[],
}

export interface ApiKeyRequest {
    /// Validation: PATTERN_API_KEY, min 2, max 24
    name: string,
    /// Unix timestamp in seconds
    exp?: number,
    access: ApiKeyAccess[],
}

export interface ApiKeyResponse {
    name: string,
    /// Unix timestamp in seconds
    created: number,
    /// Unix timestamp in seconds
    expires?: number,
    access: ApiKeyAccess[],
}

export interface ApiKeysResponse {
    keys: ApiKeyResponse[],
}
