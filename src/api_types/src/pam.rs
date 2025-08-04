use crate::cust_validation::validate_vec_linux_hostname;
use crate::users::WebauthnAuthFinishRequest;
use rauthy_common::regex::{RE_ALNUM, RE_LINUX_HOSTNAME, RE_LINUX_USERNAME};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum PamGroupType {
    Immutable,
    Host,
    User,
    Generic,
    Local,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PamGroupUserLink {
    pub uid: u32,
    pub gid: u32,
    pub wheel: bool,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PamGroupCreateRequest {
    //// Validation: `^[a-z][a-z0-9_-]{1,63}$`
    #[validate(regex(path = "*RE_LINUX_USERNAME", code = "^[a-z][a-z0-9_-]{1,61}$"))]
    pub name: String,
    pub typ: PamGroupType,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PamHostCreateRequest {
    /// Validation: `^[a-zA-Z0-9][a-zA-Z0-9-.]*[a-zA-Z0-9]$`
    #[validate(
        length(max = 61),
        regex(
            path = "*RE_LINUX_HOSTNAME",
            code = "^[a-zA-Z0-9][a-zA-Z0-9-.]*[a-zA-Z0-9]$"
        )
    )]
    pub hostname: String,
    pub gid: u32,
    pub force_mfa: bool,
    pub local_password_only: bool,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PamLoginRequest {
    /// Validation: `^[a-zA-Z0-9]{24}$`
    #[validate(
        length(min = 24, max = 24),
        regex(path = "*RE_ALNUM", code = "^[a-zA-Z0-9]{24}$")
    )]
    pub host_id: String,
    #[validate(length(min = 64, max = 64))]
    pub host_secret: String,
    //// Validation: `^[a-z][a-z0-9_-]{1,63}$`
    #[validate(regex(path = "*RE_LINUX_USERNAME", code = "^[a-z][a-z0-9_-]{1,61}$"))]
    pub username: String,
    /// Validation: Applies password policy - max 256 characters
    #[validate(length(max = 256))]
    pub password: Option<String>,
    /// PAM remote password - generated via account dashboard.
    /// Works for both password-only and MFA accounts.
    #[validate(length(min = 16, max = 128))]
    pub remote_password: Option<String>,
    #[validate(length(max = 64))]
    pub webauthn_code: Option<String>,
}

// TODO we could require machine id + secret here as well
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PamMfaStartRequest {
    //// Validation: `^[a-z][a-z0-9_-]{1,63}$`
    #[validate(regex(path = "*RE_LINUX_USERNAME", code = "^[a-z][a-z0-9_-]{1,61}$"))]
    pub username: String,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct PamMfaFinishRequest {
    #[validate(length(max = 32))]
    pub user_id: String,
    #[validate(nested)]
    pub data: WebauthnAuthFinishRequest,
}

#[derive(Debug, Deserialize, ToSchema)]
pub enum Getent {
    Users,
    UserId(u32),
    Username(String),
    Groups,
    Groupname(String),
    GroupId(u32),
    Hosts,
    Hostname(String),
    /// Validation: IpAddr
    #[schema(value_type = str)]
    HostIp(IpAddr),
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PamGetentRequest {
    /// Validation: `^[a-zA-Z0-9]{24}$`
    #[validate(
        length(min = 24, max = 24),
        regex(path = "*RE_ALNUM", code = "^[a-zA-Z0-9]{24}$")
    )]
    pub host_id: String,
    #[validate(length(min = 64, max = 64))]
    pub host_secret: String,
    pub getent: Getent,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PamHostUpdateRequest {
    /// Validation: `^[a-zA-Z0-9][a-zA-Z0-9-.]*[a-zA-Z0-9]$`
    #[validate(
        length(max = 61),
        regex(
            path = "*RE_LINUX_HOSTNAME",
            code = "^[a-zA-Z0-9][a-zA-Z0-9-.]*[a-zA-Z0-9]$"
        )
    )]
    pub hostname: String,
    pub gid: u32,
    pub force_mfa: bool,
    pub local_password_only: bool,
    pub notes: Option<String>,
    /// Validation: IpAddr
    #[schema(value_type = str)]
    pub ips: Vec<IpAddr>,
    /// Validation: `^[a-zA-Z0-9][a-zA-Z0-9-.]*[a-zA-Z0-9]$`
    #[validate(custom(function = "validate_vec_linux_hostname"))]
    pub aliases: Vec<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PamHostWhoamiRequest {
    #[validate(length(min = 64, max = 64))]
    pub host_secret: String,
}

/// Preflight request for PAM authentications to check, if the given user would be allowed to log
/// in to this client and if so, under which conditions.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PamPreflightRequest {
    /// Validation: `^[a-zA-Z0-9]{24}$`
    #[validate(
        length(min = 24, max = 24),
        regex(path = "*RE_ALNUM", code = "^[a-zA-Z0-9]{24}$")
    )]
    pub host_id: String,
    #[validate(length(min = 64, max = 64))]
    pub host_secret: String,
    //// Validation: `^[a-z][a-z0-9_-]{1,63}$`
    #[validate(regex(path = "*RE_LINUX_USERNAME", code = "^[a-z][a-z0-9_-]{1,61}$"))]
    pub username: String,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct PamUserCreateRequest {
    //// Validation: `^[a-z][a-z0-9_-]{1,63}$`
    #[validate(regex(path = "*RE_LINUX_USERNAME", code = "^[a-z][a-z0-9_-]{1,61}$"))]
    pub username: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PamUserUpdateRequest {
    #[validate(length(max = 24))]
    pub shell: String,
    pub groups: Vec<PamGroupUserLink>,
}

#[derive(Debug, Serialize, ToSchema)]
pub enum PamGetentResponse {
    Users(Vec<PamUserResponse>),
    User(PamUserResponse),
    Groups(Vec<PamGroupMembersResponse>),
    Group(PamGroupMembersResponse),
    Hosts(Vec<PamHostSimpleResponse>),
    Host(PamHostSimpleResponse),
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PamGroupResponse {
    pub id: u32,
    pub name: String,
    pub typ: PamGroupType,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PamGroupHostsCountResponse {
    pub gid: u32,
    pub count: u32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PamGroupMembersResponse {
    pub id: u32,
    pub name: String,
    pub typ: PamGroupType,
    pub members: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PamHostSimpleResponse {
    pub id: String,
    pub name: String,
    pub aliases: Vec<String>,
    #[schema(value_type = str)]
    pub addresses: Vec<IpAddr>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PamHostAccessResponse {
    pub hostname: String,
    pub force_mfa: bool,
    pub notes: Option<String>,
    #[schema(value_type = str)]
    pub ips: Vec<IpAddr>,
    pub aliases: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PamHostDetailsResponse {
    pub id: String,
    pub hostname: String,
    pub gid: u32,
    pub force_mfa: bool,
    pub notes: Option<String>,
    #[schema(value_type = str)]
    pub ips: Vec<IpAddr>,
    pub aliases: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PamHostSecretResponse {
    pub id: String,
    pub secret: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PamPasswordResponse {
    pub exp: i64,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PamPreflightResponse {
    pub login_allowed: bool,
    pub local_password_only: bool,
    pub mfa_required: bool,
}

#[derive(Serialize, ToSchema)]
pub struct PamUnlinkedEmailsResponse {
    pub emails: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PamUserResponse {
    pub id: u32,
    pub name: String,
    pub gid: u32,
    pub email: String,
    pub shell: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PamUserDetailsResponse {
    pub id: u32,
    pub name: String,
    pub gid: u32,
    pub email: String,
    pub shell: String,
    pub groups: Vec<PamGroupUserLink>,
}
