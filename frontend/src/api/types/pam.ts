export type PamGroupType =
    'immutable' |
    'host' |
    'user' |
    'generic' |
    'local';

export interface PamGroupUserLink {
    uid: number,
    gid: number,
    wheel: boolean,
}

export interface PamGroupCreateRequest {
    //// Validation: PATTERN_LINUX_USERNAME
    name: string,
    typ: PamGroupType,
}

export interface PamHostCreateRequest {
    /// Validation: `PATTERN_LINUX_HOSTNAME, min 2, max 63
    hostname: string,
    gid: number,
    force_mfa: boolean,
}

export interface PamHostUpdateRequest {
    /// Validation: `PATTERN_LINUX_HOSTNAME, min 2, max 63
    hostname: string,
    gid: number,
    force_mfa: boolean,
    notes?: string,
    /// Validation: IpAddr
    ips: string[],
    /// Validation: `PATTERN_LINUX_HOSTNAME, min 2, max 63
    aliases: string[],
}

export interface PamUserCreateRequest {
    /// Validation: PATTERN_LINUX_USERNAME
    username: string,
    email: string,
}

export interface PamUsernameCheckRequest {
    /// Validation: PATTERN_LINUX_USERNAME
    username: string,
    pow: string,
}

export interface PamUserUpdateRequest {
    /// Validation: max length 24
    shell: string,
    groups: PamGroupUserLink[],
}

export interface PamGroupResponse {
    id: number,
    name: string,
    typ: PamGroupType,
}

export interface PamGroupHostsCountResponse {
    gid: number,
    count: number,
}

export interface PamHostSimpleResponse {
    id: string,
    name: string,
    aliases: string[],
    addresses: string[],
}

export interface PamHostDetailsResponse {
    id: string,
    hostname: string,
    gid: number,
    secret: string,
    force_mfa: boolean,
    notes?: string,
    ips: string[],
    aliases: string[],
}

export interface PamHostSecretResponse {
    id: string,
    secret: string,
}

export interface PamUnlinkedEmailsResponse {
    emails: string[],
}

export interface PamUserResponse {
    id: number,
    name: string,
    gid: number,
    email: string,
    shell: string,
}

export interface PamUserDetailsResponse {
    id: number,
    name: string,
    gid: number,
    email: string,
    shell: string,
    groups: PamGroupUserLink[],
}
