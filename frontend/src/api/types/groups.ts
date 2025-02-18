export interface NewGroupRequest {
    /// Validation: PATTERN_GROUP
    group: string,
}

export interface GroupResponse {
    id: string,
    name: string,
}
