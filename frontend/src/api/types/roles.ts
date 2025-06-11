export interface RoleRequest {
    /// Validation: PATTERN_ROLE_SCOPE
    role: string,
}

export interface RoleResponse {
    id: string,
    name: string,
}