use crate::ReqPrincipal;
use actix_web::http::header::AUTHORIZATION;
use actix_web::web::Path;
use actix_web::{HttpRequest, HttpResponse, delete, get, post, put};
use actix_web_lab::extract::Json;
use chrono::Utc;
use rauthy_api_types::pam::{
    Getent, PamGetentRequest, PamGroupCreateRequest, PamGroupHostsCountResponse,
    PamGroupMembersResponse, PamGroupResponse, PamHostCreateRequest, PamHostDetailsResponse,
    PamHostSecretResponse, PamHostSimpleResponse, PamHostUpdateRequest, PamLoginRequest,
    PamMfaFinishRequest, PamMfaStartRequest, PamPreflightRequest, PamPreflightResponse,
    PamUnlinkedEmailsResponse, PamUserCreateRequest, PamUserDetailsResponse, PamUserResponse,
    PamUserUpdateRequest, PamUsernameCheckRequest,
};
use rauthy_api_types::users::MfaPurpose;
use rauthy_common::utils::real_ip_from_req;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::entity::failed_login_counter::FailedLoginCounter;
use rauthy_models::entity::pam::group_user_links::PamGroupUserLink;
use rauthy_models::entity::pam::groups::PamGroup;
use rauthy_models::entity::pam::hosts::PamHost;
use rauthy_models::entity::pam::tokens::PamToken;
use rauthy_models::entity::pam::users::PamUser;
use rauthy_models::entity::users::User;
use rauthy_models::entity::webauthn;
use rauthy_models::entity::webauthn::WebauthnServiceReq;
use serde::Serialize;
use spow::pow::Pow;
use std::cmp::max;
use std::time::Duration;
use tracing::{info, warn};
use utoipa::ToSchema;
use validator::Validate;

#[post("/pam/check")]
pub async fn post_username_check(
    principal: ReqPrincipal,
    Json(payload): Json<PamUsernameCheckRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;

    // The PoW is requested because if you had an instance with open registration and enabled
    // PAM account creation rights by default, this could be abused for username enumeration
    // otherwise.
    Pow::validate(&payload.pow)?;

    if ["root", "admin"].contains(&payload.username.as_str()) {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Blacklisted username",
        ));
    }

    match PamUser::find_by_name(payload.username).await {
        Ok(_) => Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "username already taken",
        )),
        Err(_) => Ok(HttpResponse::Ok().finish()),
    }
}

#[get("/pam/emails_unlinked")]
pub async fn get_pam_emails_unlinked(
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;

    let emails = PamUser::find_emails_unlinked().await?;

    Ok(HttpResponse::Ok().json(PamUnlinkedEmailsResponse { emails }))
}

#[post("/pam/getent")]
pub async fn post_getent(
    Json(payload): Json<PamGetentRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    info!("getent {:?}", payload.getent);
    payload.validate()?;

    let host = PamHost::find_simple(payload.host_id).await?;
    host.validate_secret(payload.host_secret)?;

    let resp = match payload.getent {
        Getent::Users => {
            // TODO somehow limit users to only the ones that are allowed to log in
            let users = PamUser::find_all()
                .await?
                .into_iter()
                .map(PamUserResponse::from)
                .collect::<Vec<_>>();
            PamGetentResponse::Users(users)
        }
        Getent::Username(name) => {
            let user = PamUser::find_by_name(name).await?;
            PamGetentResponse::User(PamUserResponse::from(user))
        }
        Getent::UserId(id) => {
            let user = PamUser::find_by_id(id).await?;
            PamGetentResponse::User(PamUserResponse::from(user))
        }

        Getent::Groups => {
            // TODO we should probably have a default config or pool that can be assigned to
            // machines, with default groups and so on
            let groups = PamGroup::find_all().await?;
            let mut res = Vec::with_capacity(groups.len());
            for group in groups {
                // TODO this could be made more efficient when we build it up manually
                //  and fetch all users in advance, only once.
                res.push(group.build_members_response().await?);
            }
            PamGetentResponse::Groups(res)
        }
        Getent::Groupname(name) => {
            let group = PamGroup::find_by_name(name).await?;
            PamGetentResponse::Group(group.build_members_response().await?)
        }
        Getent::GroupId(id) => {
            let group = PamGroup::find_by_id(id).await?;
            PamGetentResponse::Group(group.build_members_response().await?)
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

#[get("/pam/groups")]
pub async fn get_pam_groups(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let groups = PamGroup::find_all()
        .await?
        .into_iter()
        .map(PamGroupResponse::from)
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(groups))
}

#[post("/pam/groups")]
pub async fn post_pam_groups(
    principal: ReqPrincipal,
    Json(payload): Json<PamGroupCreateRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;
    payload.validate()?;

    let group = PamGroup::insert(payload.name, payload.typ.into()).await?;

    Ok(HttpResponse::Ok().json(group))
}

#[get("/pam/groups/{id}/hosts_count")]
pub async fn get_pam_group_hosts_count(
    id: Path<u32>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let gid = id.into_inner();
    let count = PamHost::count_with_group(gid).await? as u32;

    Ok(HttpResponse::Ok().json(PamGroupHostsCountResponse { gid, count }))
}

#[delete("/pam/groups/{id}")]
pub async fn delete_pam_group(
    id: Path<u32>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    PamGroup::delete(id.into_inner()).await?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/pam/hosts")]
pub async fn get_hosts(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let hosts = PamHost::find_all_simple()
        .await?
        .into_iter()
        .map(PamHostSimpleResponse::from)
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(hosts))
}

#[post("/pam/hosts")]
pub async fn post_hosts(
    principal: ReqPrincipal,
    Json(payload): Json<PamHostCreateRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;
    payload.validate()?;

    let host = PamHost::insert(payload.hostname, payload.gid, payload.force_mfa).await?;

    Ok(HttpResponse::Ok().json(PamHostSimpleResponse::from(host)))
}

#[get("/pam/hosts/{id}")]
pub async fn get_host_details(
    id: Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let host = PamHost::find_by_id_full(id.into_inner()).await?;

    Ok(HttpResponse::Ok().json(PamHostDetailsResponse::from(host)))
}

#[post("/pam/hosts/{id}/secret")]
pub async fn post_host_secret(
    id: Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let host = PamHost::find_simple(id.into_inner()).await?;

    Ok(HttpResponse::Ok().json(PamHostSecretResponse {
        id: host.id,
        secret: host.secret,
    }))
}

#[put("/pam/hosts/{id}/secret")]
pub async fn put_host_secret(
    id: Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let id = id.into_inner();
    let secret = PamHost::rotate_secret(id.clone()).await?;

    Ok(HttpResponse::Ok().json(PamHostSecretResponse { id, secret }))
}

#[put("/pam/hosts/{id}")]
pub async fn put_host(
    id: Path<String>,
    principal: ReqPrincipal,
    Json(payload): Json<PamHostUpdateRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;
    payload.validate()?;

    PamHost::update(id.into_inner(), payload).await?;

    Ok(HttpResponse::Ok().finish())
}

#[delete("/pam/hosts/{id}")]
pub async fn delete_host(
    id: Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    PamHost::delete(id.into_inner()).await?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/pam/login")]
pub async fn post_login(
    req: HttpRequest,
    Json(payload): Json<PamLoginRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;
    if payload.password.is_none() && payload.webauthn_code.is_none() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "at least one of password or webauthn_code must be given",
        ));
    }

    let host = PamHost::find_simple(payload.host_id).await?;
    host.validate_secret(payload.host_secret)?;

    let pam_user = PamUser::find_by_name(payload.username).await?;
    if !host.is_login_allowed(&pam_user).await {
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "Not allowed to log in to this host",
        ));
    }

    let mut user = User::find_by_email(pam_user.email.clone()).await?;
    user.check_expired()?;
    user.check_enabled()?;

    let ip = real_ip_from_req(&req)?;
    if let Some(password) = payload.password {
        match user.validate_password(password).await {
            Ok(_) => {
                user.last_login = Some(Utc::now().timestamp());
                user.last_failed_login = None;
                user.failed_login_attempts = None;
                user.save(None).await?;

                FailedLoginCounter::reset(ip.to_string()).await?;

                info!("New PAM login for user {}", user.email);
            }
            Err(err) => {
                user.last_failed_login = Some(Utc::now().timestamp());
                user.failed_login_attempts = Some(user.failed_login_attempts.unwrap_or(0) + 1);
                user.save(None).await?;

                let failed_logins = FailedLoginCounter::increase(ip.to_string()).await?;

                warn!(?err, "Failed PAM login for user {}", user.email);

                let sec = max(failed_logins, 5) as u64;
                tokio::time::sleep(Duration::from_secs(sec)).await;

                return Err(err);
            }
        }
    }

    if let Some(code) = payload.webauthn_code {
        let svc_req = WebauthnServiceReq::find(code).await?;
        if user.id != svc_req.user_id {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "User ID mismatch",
            ));
        }
        svc_req.delete().await?;
    } else if host.force_mfa {
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "This host requires MFA",
        ));
    }

    let token = PamToken::new(user, pam_user).await?;
    Ok(HttpResponse::Ok().json(token))
}

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

#[post("/pam/mfa/finish")]
pub async fn post_mfa_finish(
    req: HttpRequest,
    Json(payload): Json<PamMfaFinishRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;

    let resp = webauthn::auth_finish(payload.user_id, &req, payload.data).await?;
    Ok(resp.into_response())
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

#[post("/pam/preflight")]
pub async fn post_preflight(
    Json(payload): Json<PamPreflightRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;

    let host = PamHost::find_simple(payload.host_id).await?;
    host.validate_secret(payload.host_secret)?;

    let pam_user = PamUser::find_by_name(payload.username).await?;
    if !host.is_login_allowed(&pam_user).await {
        return Ok(HttpResponse::Ok().json(PamPreflightResponse {
            login_allowed: false,
            mfa_required: host.force_mfa,
        }));
    }

    let user = User::find_by_email(pam_user.email).await?;
    user.check_expired()?;
    user.check_enabled()?;

    if host.force_mfa && !user.has_webauthn_enabled() {
        // TODO maybe send a different status code here to better differentiate
        return Ok(HttpResponse::Ok().json(PamPreflightResponse {
            login_allowed: false,
            mfa_required: true,
        }));
    }

    Ok(HttpResponse::Ok().json(PamPreflightResponse {
        login_allowed: true,
        mfa_required: user.has_webauthn_enabled() || host.force_mfa,
    }))
}

#[get("/pam/users")]
pub async fn get_pam_users(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let users = PamUser::find_all()
        .await?
        .into_iter()
        .map(PamUserResponse::from)
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(users))
}

#[post("/pam/users")]
pub async fn post_pam_users(
    principal: ReqPrincipal,
    Json(payload): Json<PamUserCreateRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;
    payload.validate()?;

    let user = User::find_by_email(payload.email).await?;
    let pam_user = PamUser::insert(payload.username, user.email).await?;

    Ok(HttpResponse::Ok().json(PamUserResponse::from(pam_user)))
}

#[get("/pam/users/{uid}")]
pub async fn get_pam_user(
    uid: Path<u32>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;

    let uid = uid.into_inner();
    let pam_user = PamUser::find_by_id(uid).await?;
    let user = User::find_by_email(pam_user.email).await?;

    principal.validate_user_or_admin(&user.id)?;

    let groups = PamGroupUserLink::find_for_user(pam_user.id)
        .await?
        .into_iter()
        .map(rauthy_api_types::pam::PamGroupUserLink::from)
        .collect::<Vec<_>>();

    let resp = PamUserDetailsResponse {
        id: pam_user.id,
        name: pam_user.name,
        gid: pam_user.gid,
        email: user.email,
        shell: pam_user.shell,
        groups,
    };

    Ok(HttpResponse::Ok().json(resp))
}

#[put("/pam/users/{uid}")]
pub async fn put_pam_user(
    uid: Path<u32>,
    principal: ReqPrincipal,
    Json(payload): Json<PamUserUpdateRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;
    payload.validate()?;

    let uid = uid.into_inner();
    let pam_user = PamUser::find_by_id(uid).await?;
    if pam_user.shell != payload.shell {
        PamUser::update_shell(pam_user.id, payload.shell).await?;
    }
    PamUser::update_groups(pam_user.id, payload.groups).await?;

    Ok(HttpResponse::Ok().finish())
}

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
