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
export type AccessRights = 'read' | 'create' | 'update' | 'delete';

export interface ApiKeyAccess {
    group: AccessGroup,
    access_rights: AccessRights[],
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
