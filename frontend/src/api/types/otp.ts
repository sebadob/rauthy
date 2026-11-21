import type { OtpKind } from '#mfa/otp/types.ts';

export interface OtpResponse {
    id: string;
    name?: string;
    /// Unix timestamp in seconds
    last_used: number;
    kind: OtpKind;
    is_active: boolean;
}
