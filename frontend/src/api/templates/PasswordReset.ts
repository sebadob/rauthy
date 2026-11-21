import type { PasswordPolicyResponse } from '#api/types/password_policy.ts';

export interface PasswordResetTemplate {
    csrf_token: string;
    magic_link_id: string;
    needs_mfa: boolean;
    password_policy: PasswordPolicyResponse;
    // seconds until the redirect after a successful reset, `0` disables it
    redirect_timeout: number;
    user_id: string;
}
