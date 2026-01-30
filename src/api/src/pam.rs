use crate::ReqPrincipal;
use actix_web::http::header::AUTHORIZATION;
use actix_web::mime::TEXT_PLAIN;
use actix_web::web::Path;
use actix_web::{HttpRequest, HttpResponse, delete, get, post, put};
use actix_web_lab::extract::Json;
use chrono::Utc;
use rauthy_api_types::pam::{
    Getent, PamGetentRequest, PamGetentResponse, PamGroupCreateRequest, PamGroupHostsCountResponse,
    PamGroupMembersResponse, PamGroupResponse, PamHostAccessResponse, PamHostCreateRequest,
    PamHostDetailsResponse, PamHostSecretResponse, PamHostSimpleResponse, PamHostUpdateRequest,
    PamHostWhoamiRequest, PamLoginRequest, PamMfaFinishRequest, PamMfaStartRequest,
    PamPasswordResponse, PamPreflightRequest, PamPreflightResponse, PamSshAuthKeyRequest,
    PamSshAuthKeyResponse, PamUnlinkedEmailsResponse, PamUserCreateRequest, PamUserDetailsResponse,
    PamUserResponse, PamUserUpdateRequest,
};
use rauthy_api_types::users::{MfaPurpose, WebauthnAuthStartResponse};
use rauthy_common::constants::{PAM_WHEEL_ID, PAM_WHEEL_NAME};
use rauthy_common::utils::base64_decode;
use rauthy_data::entity::api_keys::{AccessGroup, AccessRights};
use rauthy_data::entity::browser_id::BrowserId;
use rauthy_data::entity::pam::authorized_keys::AuthorizedKey;
use rauthy_data::entity::pam::group_user_links::PamGroupUserLink;
use rauthy_data::entity::pam::groups::{PamGroup, PamGroupType};
use rauthy_data::entity::pam::hosts::PamHost;
use rauthy_data::entity::pam::remote_password::PamRemotePassword;
use rauthy_data::entity::pam::tokens::PamToken;
use rauthy_data::entity::pam::users::PamUser;
use rauthy_data::entity::users::User;
use rauthy_data::entity::webauthn;
use rauthy_data::entity::webauthn::{WebauthnAdditionalData, WebauthnServiceReq};
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::cmp::max;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;
use tracing::{debug, info, warn};
use validator::Validate;

/// GET all user emails that are not yet linked to a PAM account
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/pam/emails_unlinked",
    tag = "pam",
    responses(
        (status = 200, description = "Ok", body = PamUnlinkedEmailsResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[get("/pam/emails_unlinked")]
pub async fn get_pam_emails_unlinked(
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Pam, AccessRights::Read)?;

    let emails = PamUser::find_emails_unlinked().await?;

    Ok(HttpResponse::Ok().json(PamUnlinkedEmailsResponse { emails }))
}

/// `getent` request from PAM hosts
///
/// **Permissions**
/// - validated PAM host
#[utoipa::path(
    post,
    path = "/pam/getent",
    tag = "pam",
    request_body = PamGetentRequest,
    responses(
        (status = 200, description = "Ok", body = PamGetentResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[post("/pam/getent")]
pub async fn post_getent(
    Json(payload): Json<PamGetentRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    debug!("getent {:?}", payload.getent);
    payload.validate()?;

    let host = PamHost::find_simple(payload.host_id).await?;
    host.validate_secret(payload.host_secret.as_bytes())?;

    let resp = match payload.getent {
        Getent::Users => {
            let links = PamGroupUserLink::find_for_group(host.gid)
                .await?
                .into_iter()
                .map(|l| l.uid)
                .collect::<BTreeSet<u32>>();

            let users = PamUser::find_all()
                .await?
                .into_iter()
                .filter_map(|u| {
                    if links.contains(&u.id) {
                        // No need to include the SSH keys and do additional lookups for a get all
                        Some(u.build_response(None))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            PamGetentResponse::Users(users)
        }
        Getent::Username(name) => {
            let user = PamUser::find_by_name(name).await?;
            let authorized_keys = if validate_authorized_keys_enabled().is_ok() {
                Some(AuthorizedKey::find_by_uid(user.id).await?)
            } else {
                None
            };
            PamGetentResponse::User(user.build_response(authorized_keys))
        }
        Getent::UserId(id) => {
            let user = PamUser::find_by_id(id).await?;
            let authorized_keys = if validate_authorized_keys_enabled().is_ok() {
                Some(AuthorizedKey::find_by_uid(user.id).await?)
            } else {
                None
            };
            PamGetentResponse::User(user.build_response(authorized_keys))
        }

        Getent::Groups => {
            let mut groups = PamGroup::find_all()
                .await?
                .into_iter()
                .filter_map(|g| match g.typ {
                    PamGroupType::Immutable | PamGroupType::Host => None,
                    _ => Some((
                        g.id,
                        PamGroupMembersResponse {
                            id: g.id,
                            name: g.name,
                            typ: g.typ.into(),
                            members: Vec::with_capacity(8),
                        },
                    )),
                })
                .collect::<BTreeMap<u32, PamGroupMembersResponse>>();

            let users = PamUser::find_all()
                .await?
                .into_iter()
                .map(|u| (u.id, u))
                .collect::<BTreeMap<u32, PamUser>>();

            let links = PamGroupUserLink::find_all().await?;

            for link in links {
                if let Some(group) = groups.get_mut(&link.gid)
                    && let Some(user) = users.get(&link.uid)
                {
                    group.members.push(user.name.clone());
                }
            }

            let mut res = groups.into_values().collect::<Vec<_>>();
            res.push(host.build_wheel_group_response().await?);

            PamGetentResponse::Groups(res)
        }
        Getent::Groupname(name) => {
            let group = if name == PAM_WHEEL_NAME {
                host.build_wheel_group_response().await?
            } else {
                PamGroup::find_by_name(name)
                    .await?
                    .build_members_response()
                    .await?
            };
            PamGetentResponse::Group(group)
        }
        Getent::GroupId(id) => {
            let group = if id == PAM_WHEEL_ID {
                host.build_wheel_group_response().await?
            } else {
                PamGroup::find_by_id(id)
                    .await?
                    .build_members_response()
                    .await?
            };
            PamGetentResponse::Group(group)
        }

        Getent::Hosts => {
            let hosts = PamHost::find_in_group_full(host.gid)
                .await?
                .into_iter()
                .map(PamHostSimpleResponse::from)
                .collect::<Vec<_>>();
            PamGetentResponse::Hosts(hosts)
        }
        Getent::Hostname(name) => {
            let host = PamHost::find_by_alias_full(name).await?;
            PamGetentResponse::Host(PamHostSimpleResponse::from(host))
        }
        Getent::HostIp(ip) => {
            let host = PamHost::find_by_ip_full(ip).await?;
            PamGetentResponse::Host(PamHostSimpleResponse::from(host))
        }
    };

    Ok(HttpResponse::Ok().json(resp))
}

/// GET PAM groups
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/pam/groups",
    tag = "pam",
    responses(
        (status = 200, description = "Ok", body = [PamGroupResponse]),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[get("/pam/groups")]
pub async fn get_pam_groups(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Pam, AccessRights::Read)?;

    let groups = PamGroup::find_all()
        .await?
        .into_iter()
        .map(PamGroupResponse::from)
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(groups))
}

/// Create a new PAM group
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/pam/groups",
    tag = "pam",
    request_body = PamGroupCreateRequest,
    responses(
        (status = 200, description = "Ok", body = PamGroupResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[post("/pam/groups")]
pub async fn post_pam_groups(
    principal: ReqPrincipal,
    Json(payload): Json<PamGroupCreateRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Pam, AccessRights::Create)?;
    payload.validate()?;

    let group = PamGroup::insert(payload.name, payload.typ.into()).await?;

    Ok(HttpResponse::Ok().json(PamGroupResponse::from(group)))
}

/// GET linked hosts count in group
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/pam/groups/{id}/hosts_count",
    tag = "pam",
    responses(
        (status = 200, description = "Ok", body = PamGroupHostsCountResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[get("/pam/groups/{id}/hosts_count")]
pub async fn get_pam_group_hosts_count(
    id: Path<u32>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Pam, AccessRights::Read)?;

    let gid = id.into_inner();
    let count = PamHost::count_with_group(gid).await? as u32;

    Ok(HttpResponse::Ok().json(PamGroupHostsCountResponse { gid, count }))
}

/// DELETE a PAM group
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    delete,
    path = "/pam/groups/{id}",
    tag = "pam",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[delete("/pam/groups/{id}")]
pub async fn delete_pam_group(
    id: Path<u32>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Pam, AccessRights::Delete)?;

    PamGroup::delete(id.into_inner()).await?;

    Ok(HttpResponse::Ok().finish())
}

/// GET PAM all hosts
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/pam/hosts",
    tag = "pam",
    responses(
        (status = 200, description = "Ok", body = [PamHostSimpleResponse]),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[get("/pam/hosts")]
pub async fn get_hosts(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Pam, AccessRights::Read)?;

    let hosts = PamHost::find_all_simple()
        .await?
        .into_iter()
        .map(PamHostSimpleResponse::from)
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(hosts))
}

/// Create a new PAM host
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/pam/hosts",
    tag = "pam",
    request_body = PamHostCreateRequest,
    responses(
        (status = 200, description = "Ok", body = PamHostSimpleResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[post("/pam/hosts")]
pub async fn post_hosts(
    principal: ReqPrincipal,
    Json(payload): Json<PamHostCreateRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Pam, AccessRights::Create)?;
    payload.validate()?;

    let host = PamHost::insert(
        payload.hostname,
        payload.gid,
        payload.force_mfa,
        payload.local_password_only,
    )
    .await?;

    Ok(HttpResponse::Ok().json(PamHostSimpleResponse::from(host)))
}

/// GET all PAM hosts "this" session has access to
///
/// **Permissions**
/// - any authenticated session
#[utoipa::path(
    get,
    path = "/pam/hosts/user_access",
    tag = "pam",
    responses(
        (status = 200, description = "Ok", body = [PamHostAccessResponse]),
        (status = 401, description = "Unauthorized"),
    ),
)]
#[get("/pam/hosts/user_access")]
pub async fn get_hosts_user_access(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;

    let user = PamUser::find_by_user_id(principal.user_id()?.to_string()).await?;
    let links = PamGroupUserLink::find_for_user(user.id)
        .await?
        .into_iter()
        .map(|link| link.gid)
        .collect::<Vec<_>>();

    // finding all of them here because this will probably be cached in the future
    let hosts = PamHost::find_all_full()
        .await?
        .into_iter()
        .filter_map(|h| {
            if links.contains(&h.gid) {
                Some(PamHostAccessResponse::from(h))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(hosts))
}

/// GET detailed information for a PAM host
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/pam/hosts/{id}",
    tag = "pam",
    responses(
        (status = 200, description = "Ok", body = PamHostDetailsResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[get("/pam/hosts/{id}")]
pub async fn get_host_details(
    id: Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Pam, AccessRights::Read)?;

    let host = PamHost::find_by_id_full(id.into_inner()).await?;

    Ok(HttpResponse::Ok().json(PamHostDetailsResponse::from(host)))
}

/// Retrieve the secret for a PAM host
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/pam/hosts/{id}/secret",
    tag = "pam",
    responses(
        (status = 200, description = "Ok", body = PamHostSecretResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[post("/pam/hosts/{id}/secret")]
pub async fn post_host_secret(
    id: Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Pam, AccessRights::Read)?;

    let host = PamHost::find_simple(id.into_inner()).await?;

    let secret = host.secret();
    Ok(HttpResponse::Ok().json(PamHostSecretResponse {
        id: host.id,
        secret,
    }))
}

/// Create a new random secret for a PAM host
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    put,
    path = "/pam/hosts/{id}/secret",
    tag = "pam",
    responses(
        (status = 200, description = "Ok", body = PamHostSecretResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[put("/pam/hosts/{id}/secret")]
pub async fn put_host_secret(
    id: Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Pam, AccessRights::Update)?;

    let id = id.into_inner();
    let secret = PamHost::rotate_secret(id.clone()).await?;

    Ok(HttpResponse::Ok().json(PamHostSecretResponse { id, secret }))
}

/// Update a PAM host
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    put,
    path = "/pam/hosts/{id}",
    tag = "pam",
    request_body = PamHostUpdateRequest,
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[put("/pam/hosts/{id}")]
pub async fn put_host(
    id: Path<String>,
    principal: ReqPrincipal,
    Json(payload): Json<PamHostUpdateRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Pam, AccessRights::Update)?;
    payload.validate()?;

    PamHost::update(id.into_inner(), payload).await?;

    Ok(HttpResponse::Ok().finish())
}

/// DELETE a PAM host
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    delete,
    path = "/pam/hosts/{id}",
    tag = "pam",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[delete("/pam/hosts/{id}")]
pub async fn delete_host(
    id: Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Pam, AccessRights::Delete)?;

    PamHost::delete(id.into_inner()).await?;

    Ok(HttpResponse::Ok().finish())
}

/// whoami request for a PAM host
///
/// **Permissions**
/// - any authenticated PAM host
#[utoipa::path(
    post,
    path = "/pam/hosts/{id}/whoami",
    tag = "pam",
    request_body = PamHostWhoamiRequest,
    responses(
        (status = 200, description = "Ok", body = PamHostDetailsResponse),
        (status = 401, description = "Unauthorized"),
    ),
)]
#[post("/pam/hosts/{id}/whoami")]
pub async fn post_host_whoami(
    id: Path<String>,
    Json(payload): Json<PamHostWhoamiRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;

    let host = PamHost::find_by_id_full(id.into_inner()).await?;
    host.validate_secret(payload.host_secret.as_bytes())?;
    Ok(HttpResponse::Ok().json(PamHostDetailsResponse::from(host)))
}

/// Login via a PAM host
///
/// **Permissions**
/// - any authenticated PAM host
#[utoipa::path(
    post,
    path = "/pam/login",
    tag = "pam",
    request_body = PamLoginRequest,
    responses(
        (status = 200, description = "Ok", body = PamToken),
        (status = 401, description = "Unauthorized"),
    ),
)]
#[post("/pam/login")]
pub async fn post_login(
    Json(payload): Json<PamLoginRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;
    if payload.password.is_none()
        && payload.webauthn_code.is_none()
        && payload.remote_password.is_none()
        && payload.danger_auth_checked_locally != Some(true)
    {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "at least one of password, remote_password or webauthn_code must be given",
        ));
    }

    let host = PamHost::find_simple(payload.host_id).await?;
    host.validate_secret(payload.host_secret.as_bytes())?;

    let pam_user = PamUser::find_by_name(payload.username).await?;
    if !host.is_user_in_group(&pam_user).await {
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "Not allowed to log in to this host",
        ));
    }

    let mut user = User::find_by_email(pam_user.email.clone()).await?;
    user.check_expired()?;
    user.check_enabled()?;

    #[inline]
    async fn pwd_login_fail(user: &mut User, err: ErrorResponse) -> Result<(), ErrorResponse> {
        user.last_failed_login = Some(Utc::now().timestamp());
        user.failed_login_attempts = Some(user.failed_login_attempts.unwrap_or(0) + 1);
        user.save(None).await?;

        // Note: We don't want to increase the FailedLoginCounter for the IP here, because
        // this could be abused for DoS and block remote hosts with enough failed login atempts.

        warn!(?err, "Failed PAM login for user {}", user.email);

        let sec = max(user.failed_login_attempts.unwrap_or(1), 5) as u64;
        tokio::time::sleep(Duration::from_secs(sec)).await;

        Err(err)
    }

    if let Some(password) = payload.password {
        if !host.local_password_only && user.has_webauthn_enabled() {
            pwd_login_fail(
                &mut user,
                ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "MFA User and local_password_only not set",
                ),
            )
            .await?;
        }
        if let Err(err) = user.validate_password(password).await {
            pwd_login_fail(&mut user, err).await?;
        }
    } else if let Some(code) = payload.webauthn_code {
        let svc_req = WebauthnServiceReq::find(code).await?;
        if user.id != svc_req.user_id {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "User ID mismatch",
            ));
        }
        svc_req.delete().await?;
    } else if let Some(password) = payload.remote_password {
        if host.force_mfa && !user.has_webauthn_enabled() {
            pwd_login_fail(
                &mut user,
                ErrorResponse::new(ErrorResponseType::Forbidden, "This host requires MFA"),
            )
            .await?;
        }

        if let Err(err) = PamRemotePassword::get(pam_user.name.clone())
            .await?
            .compare_password(password.as_bytes())
        {
            pwd_login_fail(&mut user, err).await?;
        }
    } else if payload.danger_auth_checked_locally == Some(true) {
        // noop
        // This option is necessary for account management requests and checking authorization
        // after a local authentication was successful already. This is the case e.g. for SSH
        // public key logins, where `sshd` already validates the correctness of the key.
    } else {
        unreachable!();
    }

    // TODO should we even do this user update here? The PAM user belongs to this user, but on the
    //  other hand, these metrics are for all non-PAM logins.
    user.last_login = Some(Utc::now().timestamp());
    user.last_failed_login = None;
    user.failed_login_attempts = None;
    user.save(None).await?;

    info!("New PAM login for user {}", user.email);

    let token = PamToken::new(user, pam_user).await?;

    Ok(HttpResponse::Ok().json(token))
}

/// Start a Passkey authentication ceremony for a PAM host
///
/// **Permissions**
/// - any authenticated PAM host
#[utoipa::path(
    post,
    path = "/pam/mfa/start",
    tag = "pam",
    request_body = PamMfaStartRequest,
    responses(
        (status = 200, description = "Ok", body = WebauthnAuthStartResponse),
        (status = 401, description = "Unauthorized"),
    ),
)]
#[post("/pam/mfa/start")]
pub async fn post_mfa_start(
    Json(payload): Json<PamMfaStartRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;

    let pam_user = PamUser::find_by_name(payload.username).await?;
    let user = User::find_by_email(pam_user.email).await?;

    let resp = webauthn::auth_start(user.id, MfaPurpose::PamLogin).await?;
    Ok(HttpResponse::Ok().json(resp))
}

/// Finish a Passkey authentication ceremony for a PAM host
///
/// **Permissions**
/// - any authenticated PAM host
#[utoipa::path(
    post,
    path = "/pam/mfa/finish",
    tag = "pam",
    request_body = PamMfaFinishRequest,
    responses(
        (status = 200, description = "Ok", body = WebauthnAdditionalData),
        (status = 401, description = "Unauthorized"),
    ),
)]
#[post("/pam/mfa/finish")]
pub async fn post_mfa_finish(
    req: HttpRequest,
    Json(payload): Json<PamMfaFinishRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;

    let resp = webauthn::auth_finish(
        payload.user_id,
        &req,
        BrowserId::default(),
        None,
        payload.data,
    )
    .await?;
    Ok(resp.into_response())
}

/// Create a new random Remote PAM Password for "this" session
///
/// **Permissions**
/// - any authenticated session with a PAM user
#[utoipa::path(
    post,
    path = "/pam/password",
    tag = "pam",
    responses(
        (status = 200, description = "Ok", body = PamPasswordResponse),
        (status = 401, description = "Unauthorized"),
    ),
)]
#[post("/pam/password")]
pub async fn post_pam_password(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;

    let pam_user = PamUser::find_by_user_id(principal.user_id()?.to_string()).await?;
    let (pwd, password_plain) = PamRemotePassword::create(pam_user.name).await?;

    Ok(HttpResponse::Ok().json(PamPasswordResponse {
        exp: pwd.exp,
        username: pwd.username,
        password: password_plain,
    }))
}

/// Preflight request for a PAM host
///
/// **Permissions**
/// - any authenticated PAM host
#[utoipa::path(
    post,
    path = "/pam/preflight",
    tag = "pam",
    request_body = PamPreflightRequest,
    responses(
        (status = 200, description = "Ok", body = PamPreflightResponse),
        (status = 401, description = "Unauthorized"),
    ),
)]
#[post("/pam/preflight")]
pub async fn post_preflight(
    Json(payload): Json<PamPreflightRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;

    let host = PamHost::find_simple(payload.host_id).await?;
    host.validate_secret(payload.host_secret.as_bytes())?;

    let pam_user = PamUser::find_by_name(payload.username).await?;
    if !host.is_user_in_group(&pam_user).await {
        return Ok(HttpResponse::Ok().json(PamPreflightResponse {
            login_allowed: false,
            local_password_only: host.local_password_only,
            mfa_required: host.force_mfa,
        }));
    }

    let user = User::find_by_email(pam_user.email).await?;
    user.check_expired()?;
    user.check_enabled()?;

    if host.force_mfa && host.local_password_only && !user.has_webauthn_enabled() {
        return Ok(HttpResponse::Ok().json(PamPreflightResponse {
            login_allowed: false,
            local_password_only: false,
            mfa_required: true,
        }));
    }

    Ok(HttpResponse::Ok().json(PamPreflightResponse {
        login_allowed: true,
        local_password_only: host.local_password_only,
        mfa_required: user.has_webauthn_enabled() || host.force_mfa,
    }))
}

/// GET all PAM users
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/pam/users",
    tag = "pam",
    responses(
        (status = 200, description = "Ok", body = [PamUserResponse]),
        (status = 401, description = "Unauthorized"),
    ),
)]
#[get("/pam/users")]
pub async fn get_pam_users(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Pam, AccessRights::Read)?;

    let users = PamUser::find_all()
        .await?
        .into_iter()
        .map(|u| u.build_response(None))
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(users))
}

/// Create a new PAM user
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/pam/users",
    tag = "pam",
    request_body = PamUserCreateRequest,
    responses(
        (status = 200, description = "Ok", body = PamUserResponse),
        (status = 401, description = "Unauthorized"),
    ),
)]
#[post("/pam/users")]
pub async fn post_pam_users(
    principal: ReqPrincipal,
    Json(payload): Json<PamUserCreateRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Pam, AccessRights::Create)?;
    payload.validate()?;

    let user = User::find_by_email(payload.email).await?;
    let pam_user = PamUser::insert(payload.username, user.email).await?;

    Ok(HttpResponse::Ok().json(pam_user.build_response(None)))
}

/// GET the PAM user for "this" session
///
/// **Permissions**
/// - any authenticated session
#[utoipa::path(
    get,
    path = "/pam/users/self",
    tag = "pam",
    responses(
        (status = 200, description = "Ok", body = PamUserResponse),
        (status = 401, description = "Unauthorized"),
    ),
)]
#[get("/pam/users/self")]
pub async fn get_pam_user_self(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;

    let pam_user = PamUser::find_by_user_id(principal.user_id()?.to_string()).await?;
    let authorized_keys = if validate_authorized_keys_enabled().is_ok() {
        Some(AuthorizedKey::find_by_uid(pam_user.id).await?)
    } else {
        None
    };

    Ok(HttpResponse::Ok().json(pam_user.build_response(authorized_keys)))
}

/// Add an `authorized_key` for the currently logged-in user
///
/// **Permissions**
/// - any authenticated session
#[utoipa::path(
    post,
    path = "/pam/users/self/authorized_keys",
    tag = "pam",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized"),
    ),
)]
#[post("/pam/users/self/authorized_keys")]
pub async fn post_pam_user_self_authorized_keys(
    principal: ReqPrincipal,
    payload: Json<PamSshAuthKeyRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;
    validate_authorized_keys_enabled()?;
    payload.validate()?;

    let pam_user = PamUser::find_by_user_id(principal.user_id()?.to_string()).await?;
    AuthorizedKey::insert(pam_user.id, &payload.data).await?;

    Ok(HttpResponse::Ok().finish())
}

/// Add an `authorized_key` for the currently logged-in user
///
/// **Permissions**
/// - any authenticated session
#[utoipa::path(
    delete,
    path = "/pam/users/self/authorized_keys/{ts_added}",
    tag = "pam",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized"),
    ),
)]
#[delete("/pam/users/self/authorized_keys/{ts_added}")]
pub async fn delete_pam_user_self_authorized_keys(
    principal: ReqPrincipal,
    ts_added: Path<i64>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;
    validate_authorized_keys_enabled()?;

    let pam_user = PamUser::find_by_user_id(principal.user_id()?.to_string()).await?;
    AuthorizedKey::find_by_uid_ts(pam_user.id, ts_added.into_inner())
        .await?
        .delete()
        .await?;

    Ok(HttpResponse::Ok().finish())
}

/// GET `AuthorizedKeys` for a user
///
/// May require `host_id:host_secret` as basic Authorization header depending on config.
#[utoipa::path(
    get,
    path = "/pam/users/authorized_keys/{username}",
    tag = "pam",
    responses(
        (status = 200, description = "Ok"),
        (status = 404, description = "NotFound"),
    ),
)]
#[get("/pam/users/authorized_keys/{username}")]
pub async fn get_pam_users_authorized_keys(
    username: Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    validate_authorized_keys_enabled()?;

    if RauthyConfig::get().vars.pam.authorized_keys.auth_required {
        let Some(head) = req.headers().get(AUTHORIZATION) else {
            return Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "'Authorization' header missing",
            ));
        };
        let Some(b64) = head.to_str().unwrap_or_default().strip_prefix("Basic ") else {
            return Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "'Authorization' invalid - expected 'Basic' authentication",
            ));
        };
        let bytes = base64_decode(b64)?;
        let data = String::from_utf8_lossy(bytes.as_ref());
        let Some((host_id, host_secret)) = data.split_once(":") else {
            return Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "'Authorization' invalid format",
            ));
        };

        let host = PamHost::find_simple(host_id.to_string()).await?;
        host.validate_secret(host_secret.as_bytes())?;
    }

    let keys = AuthorizedKey::find_by_username(username.into_inner()).await?;
    let text = AuthorizedKey::fmt_authorized_keys(&keys)?;

    Ok(HttpResponse::Ok().content_type(TEXT_PLAIN).body(text))
}

/// GET detailed information for a PAM user
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/pam/users/{uid}",
    tag = "pam",
    responses(
        (status = 200, description = "Ok", body = PamUserDetailsResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[get("/pam/users/{uid}")]
pub async fn get_pam_user(
    uid: Path<u32>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Pam, AccessRights::Read)?;

    let pam_user = PamUser::find_by_id(uid.into_inner()).await?;

    let groups = PamGroupUserLink::find_for_user(pam_user.id)
        .await?
        .into_iter()
        .map(rauthy_api_types::pam::PamGroupUserLink::from)
        .collect::<Vec<_>>();

    let authorized_keys = if validate_authorized_keys_enabled().is_ok() {
        Some(
            AuthorizedKey::find_by_uid(pam_user.id)
                .await?
                .into_iter()
                .map(PamSshAuthKeyResponse::from)
                .collect::<Vec<_>>(),
        )
    } else {
        None
    };

    let resp = PamUserDetailsResponse {
        id: pam_user.id,
        name: pam_user.name,
        gid: pam_user.gid,
        email: pam_user.email,
        shell: pam_user.shell,
        groups,
        authorized_keys,
    };

    Ok(HttpResponse::Ok().json(resp))
}

/// Update a PAM user
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    put,
    path = "/pam/users/{uid}",
    tag = "pam",
    request_body = PamUserUpdateRequest,
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[put("/pam/users/{uid}")]
pub async fn put_pam_user(
    uid: Path<u32>,
    principal: ReqPrincipal,
    Json(payload): Json<PamUserUpdateRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Pam, AccessRights::Update)?;
    payload.validate()?;

    let uid = uid.into_inner();
    let pam_user = PamUser::find_by_id(uid).await?;
    if pam_user.shell != payload.shell {
        PamUser::update_shell(pam_user.id, payload.shell).await?;
    }
    PamUser::update_groups(pam_user.id, payload.groups).await?;

    Ok(HttpResponse::Ok().finish())
}

/// Update a PAM user
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    delete,
    path = "/pam/users/{uid}/authorized_key/{ts_added}",
    tag = "pam",
    request_body = PamUserUpdateRequest,
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[delete("/pam/users/{uid}/authorized_key/{ts_added}")]
pub async fn delete_pam_user_authorized_key(
    path: Path<(u32, i64)>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Pam, AccessRights::Update)?;

    let (pam_uid, ts_added) = path.into_inner();
    AuthorizedKey::find_by_uid_ts(pam_uid, ts_added)
        .await?
        .delete()
        .await?;

    Ok(HttpResponse::Ok().finish())
}

/// Validation endpoint for PAM tokens from a PAM host
///
/// **Permissions**
/// - any authenticated PAM host
#[utoipa::path(
    get,
    path = "/pam/validate/{user_id}",
    tag = "pam",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[get("/pam/validate/{user_id}")]
pub async fn get_validate_user(
    user_id: Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    let user_id = user_id.into_inner();

    let Some(header) = req.headers().get(AUTHORIZATION) else {
        return Err(ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            "Token missing",
        ));
    };
    let s = header.to_str().unwrap_or_default();
    let Some(token_id) = s.strip_prefix("PamToken ") else {
        return Err(ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            "Invalid Authorization header",
        ));
    };
    let token = PamToken::find(token_id.to_string()).await?;
    if token.user_id != user_id {
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "User ID mismatch",
        ));
    }
    // TODO maybe additional token validation or rely on cache ttl?

    let user = User::find(user_id).await?;
    user.check_enabled()?;
    user.check_expired()?;

    Ok(HttpResponse::Ok().finish())
}

#[inline]
fn validate_authorized_keys_enabled() -> Result<(), ErrorResponse> {
    if RauthyConfig::get()
        .vars
        .pam
        .authorized_keys
        .authorized_keys_enable
    {
        Ok(())
    } else {
        Err(ErrorResponse::new(
            ErrorResponseType::NotFound,
            "SSH authorized_keys disabled",
        ))
    }
}
