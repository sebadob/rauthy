import type { PasswordPolicyResponse } from '#api/types/password_policy.ts';

export interface RegAccInitImmediate {
    enabled: boolean;
    // only present when `enabled` is true - the policy the password fields are validated against
    password_policy?: PasswordPolicyResponse;
}
