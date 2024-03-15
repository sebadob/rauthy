use crate::ReqPrincipal;
use actix_web::{post, HttpResponse};
use actix_web_validator::Json;
use rauthy_common::error_response::ErrorResponse;
use rauthy_models::entity::auth_provider::AuthProvider;
use rauthy_models::request::ProviderLookupRequest;

/// POST possible upstream auth provider config lookup
///
/// This will try to autoconfigure and build and upstream auth provider by the given issuer URL.
///
/// **Permissions**
/// - `rauthy_admin`
#[utoipa::path(
    post,
    path = "/providers/lookup",
    tag = "providers",
    responses(
        (status = 200, description = "OK", body = ProviderLookupResponse),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[post("/providers/lookup")]
pub async fn post_provider_lookup(
    // data: web::Data<AppState>,
    payload: Json<ProviderLookupRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let payload = payload.into_inner();
    let resp = AuthProvider::lookup_config(
        &payload.issuer,
        payload.danger_allow_http.unwrap_or(false),
        payload.danger_allow_insecure.unwrap_or(false),
    )
    .await?;

    Ok(HttpResponse::Ok().json(resp))
}
