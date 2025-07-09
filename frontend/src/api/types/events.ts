export type EventLevel = 'info' | 'notice' | 'warning' | 'critical';
export type EventType = 'InvalidLogins'
    | 'IpBlacklisted'
    | 'IpBlacklistRemoved'
    | 'JwksRotated'
    | 'NewUserRegistered'
    | 'NewRauthyAdmin'
    | 'NewRauthyVersion'
    | 'PossibleBruteForce'
    | 'RauthyStarted'
    | 'RauthyHealthy'
    | 'RauthyUnhealthy'
    | 'SecretsMigrated'
    | 'UserEmailChange'
    | 'UserPasswordReset'
    | 'Test'
    | 'BackchannelLogoutFailed'
    | 'ScimTaskFailed'
    | 'ForcedLogout'
    | 'UserLoginRevoke'
    | 'SuspiciousApiScan'
    ;

export interface EventsRequest {
    /// Unix timestamp in seconds
    from: number,
    /// Unix timestamp in seconds
    until?: number,
    level: EventLevel,
    typ?: EventType,
}

export interface EventResponse {
    id: string,
    timestamp: number,
    level: EventLevel,
    typ: EventType,
    ip?: string,
    data?: number,
    text?: string,
}
