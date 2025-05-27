export type UserAttrConfigTyp = 'email';

export interface UserAttrConfigEntity {
    name: string,
    desc?: string,
}

export interface UserAttrConfigRequest {
    /// Validation: PATTERN_ATTR
    name: string,
    /// Validation: PATTERN_ATTR_DESC
    desc?: string,
    default_value?: string,
    /// Currently ignored - will be implemented in a future version
    typ?: UserAttrConfigTyp,
    user_editable?: boolean,
}

export interface UserAttrConfigValueResponse {
    name: string,
    desc?: string,
    default_value?: string,
    /// Currently ignored - will be implemented in a future version
    typ?: UserAttrConfigTyp,
    user_editable?: boolean,
}

export interface UserAttrConfigResponse {
    values: UserAttrConfigValueResponse[],
}

export interface UserAttrValueResponse {
    key: string,
    value: string,
}

export interface UserAttrValuesResponse {
    values: UserAttrValueResponse[],
}
