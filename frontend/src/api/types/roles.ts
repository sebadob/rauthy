import type { JsonValue } from '#utils/jsonValue.ts';

export interface RoleRequest {
    /// Validation: PATTERN_ROLE_SCOPE
    role: string;
    meta?: JsonValue;
}

export interface RoleResponse {
    id: string;
    name: string;
    meta?: JsonValue;
}
