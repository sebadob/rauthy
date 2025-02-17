export interface ScopeRequest {
    /// Validation: PATTERN_GROUP
    scope: string,
    /// Validation: PATTERN_ATTR
    attr_include_access?: string[],
    /// Validation: PATTERN_ATTR
    attr_include_id?: string[],
}

export interface ScopeResponse {
    id: string,
    name: string,
    attr_include_access?: string[],
    attr_include_id?: string[],
}