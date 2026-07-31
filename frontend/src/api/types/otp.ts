import type { OtpKind } from "$mfa/otp/types";

export interface OtpResponse {
    id: String;
    name: String;
    /// Unix timestamp in seconds
    last_used: number;
    kind: OtpKind;
    is_active: boolean;
}
