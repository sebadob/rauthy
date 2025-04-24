export type SessionState = 'Init' | 'Auth' | 'LoggedOut' | 'Unknown';

export interface SessionResponse {
    id: string,
    user_id?: string,
    is_mfa: boolean,
    state: SessionState,
    exp: number,
    last_seen: number,
    remote_ip?: string,
}

export interface SessionInfoResponse {
    id: string,
    csrf_token?: string,
    user_id?: string,
    roles?: string,
    groups?: string,
    exp: string,
    timeout: string,
    state: SessionState,
}
