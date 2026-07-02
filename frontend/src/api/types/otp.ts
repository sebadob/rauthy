export interface OtpResponse {
    id: String;
    name: String;
    /// Unix timestamp in seconds
    last_used: number;
    kind: String;
    is_active: boolean;
}