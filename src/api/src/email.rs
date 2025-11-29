use crate::ReqPrincipal;
use actix_web::web::{Json, Path};
use actix_web::{HttpResponse, get, post};
use chrono::Utc;
use rauthy_api_types::email_jobs::{EmailContentType, EmailJobRequest, EmailJobResponse};
use rauthy_common::sanitize_html::sanitize_html;
use rauthy_data::entity::email_jobs::{EmailJob, EmailJobStatus};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use validator::Validate;

/// Get all EmailJobs
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/email",
    tag = "email",
    responses(
        (status = 200, description = "Ok", body = [EmailJobResponse]),
        (status = 400, description = "BadRequest"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[get("/email")]
pub async fn get_email_jobs(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    // TODO maybe accept a limit here?

    let resp = EmailJob::find_all()
        .await?
        .into_iter()
        .map(EmailJobResponse::from)
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(resp))
}

/// Send out a custom email to users
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/email",
    tag = "email",
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[post("/email")]
pub async fn post_send_email(
    principal: ReqPrincipal,
    Json(mut payload): Json<EmailJobRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;
    payload.validate()?;

    if payload.content_type == EmailContentType::HTML {
        payload.body = sanitize_html(&payload.body);
    }

    let job = EmailJob::insert(payload).await?;
    if job.scheduled.is_none() || job.scheduled.unwrap_or_default() <= Utc::now().timestamp() + 5 {
        job.spawn_task();
    }

    Ok(HttpResponse::Ok().finish())
}

/// Cancel a long-running E-Mail job early
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/email/cancel/{id}",
    tag = "email",
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[post("/email/cancel/{id}")]
pub async fn post_email_job_cancel(
    principal: ReqPrincipal,
    id: Path<i64>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let mut job = EmailJob::find(id.into_inner()).await?;
    if job.status != EmailJobStatus::Open {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Cannot cancel a non-open job",
        ));
    }
    job.cancel().await?;

    Ok(HttpResponse::Ok().finish())
}
