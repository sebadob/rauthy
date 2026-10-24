import type { JsonValue } from '$utils/jsonValue';

export interface RoleRequest {
    /// Validation: PATTERN_ROLE
    role: string;
    meta?: JsonValue;
}

export interface RoleResponse {
    id: string;
    name: string;
    meta?: JsonValue;
}
