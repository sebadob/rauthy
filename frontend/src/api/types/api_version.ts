export interface AppVersionResponse {
    current: string,
    last_check?: number,
    latest?: string,
    latest_url?: string,
    update_available: boolean,
}