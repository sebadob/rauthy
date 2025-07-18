export type PamGroupType =
    'immutable' |
    'host' |
    'user' |
    'generic' |
    'local';

export interface PamGroupResponse {
    id: number,
    name: string,
    typ: PamGroupType,
}

export interface PamUserResponse {
    id: number,
    name: string,
    gid: number,
    email: string,
    shell: string,
}

export interface PamUsernameCheckRequest {
    //// Validation: PATTERN_LINUX_USERNAME
    username: string,
    pow: string,
}