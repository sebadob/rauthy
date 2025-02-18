export interface UserAttrConfigValueResponse {
    name: string,
    desc?: string,
}

export interface UserAttrConfigResponse {
    values: UserAttrConfigValueResponse[],
}