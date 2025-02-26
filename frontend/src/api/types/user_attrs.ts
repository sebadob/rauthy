export interface UserAttrConfigEntity {
    name: string,
    desc?: string,
}

export interface UserAttrConfigRequest {
    /// Validation: PATTERN_ATTR
    name: string,
    /// Validation: PATTERN_ATTR_DESC
    desc?: string,
}

export interface UserAttrConfigValueResponse {
    name: string,
    desc?: string,
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
