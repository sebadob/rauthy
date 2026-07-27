import type { PasswordPolicyResponse } from '$api/types/password_policy';

export interface PasswordResetTemplate {
    csrf_token: string;
    magic_link_id: string;
    needs_mfa: boolean;
    password_policy: PasswordPolicyResponse;
    user_id: string;
}
