export interface SessionResponse {
    id: string,
    user_id?: string,
    is_mfa: boolean,
    state: 'Open' | 'Init' | 'Auth' | 'LoggedOut' | 'Unknown',
    exp: number,
    last_seen: number,
    remote_ip?: string,
}