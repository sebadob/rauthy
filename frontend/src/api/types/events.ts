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
    | 'Test';

export interface EventResponse {
    id: string,
    timestamp: number,
    level: EventLevel,
    typ: EventType,
    ip?: string,
    data?: number,
    text?: string,
}
