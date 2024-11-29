use actix_web::{
    get,
    http::{
        header::{self, HeaderValue, LOCATION},
        StatusCode,
    },
    post,
    web::{self, Json, Query},
    HttpRequest, HttpResponse,
};
use rauthy_api_types::atproto;
use rauthy_common::constants::HEADER_HTML;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::{
    app_state::AppState,
    entity::{atproto::Callback, colors::Colors},
    language::Language,
    templates::{atproto::AuthorizeHtml, FrontendAction},
};

use crate::{map_auth_step, ReqPrincipal};

#[utoipa::path(
    get,
    path = "/atproto/client_metadata",
    tag = "atproto",
    responses(
        (status = 200, description = "OK"),
    ),
)]
#[get("/atproto/client_metadata")]
pub async fn get_client_metadata(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    let atproto = data.atproto.read().await;

    Ok(HttpResponse::Ok()
        .insert_header((
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_str("*").unwrap(),
        ))
        .json(&atproto.client_metadata))
}

/// OIDC Authorization HTML
///
/// Starts the authorization_code flow. Log in with username / password.<br>
/// If one does not exist, a new session will be opened with the 'Init' state and set's a cookie.
#[utoipa::path(
    get,
    path = "/atproto/login",
    tag = "atproto",
    // params(AuthRequest),
    responses(
        (status = 200, description = "If the params match the allowed settings, returns the pre-rendered HTML",),
        // (status = 400, description = "If any params do not match the backend config", body = ErrorResponse),
    ),
)]
#[get("/atproto/login")]
pub async fn get_login(// data: web::Data<AppState>,
    // req: HttpRequest,
    // req_data: actix_web_validator::Query<AuthRequest>,
    // principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    let colors = Colors {
        act1: "#6b3d99".to_string(),
        act1a: "#714d99".to_string(),
        act2: "#3575dd".to_string(),
        act2a: "#3b82f6".to_string(),
        acnt: "#388c51".to_string(),
        acnta: "#4d8c62".to_string(),
        ..Default::default()
    };
    let lang = Language::default();

    // let (client, origin_header) = match validation::validate_auth_req_param(
    //     &data,
    //     &req,
    //     &req_data.client_id,
    //     &req_data.redirect_uri,
    //     &req_data.code_challenge,
    //     &req_data.code_challenge_method,
    // )
    // .await
    // {
    //     Ok(res) => res,
    //     Err(err) => {
    //         let status = err.status_code();
    //         let body = Error1Html::build(&colors, &lang, status, Some(err.message));
    //         return Ok(ErrorHtml::response(body, status));
    //     }
    // };

    // // check prompt and max_age to possibly force a new session
    // let mut force_new_session = if req_data
    //     .prompt
    //     .as_ref()
    //     .map(|p| p.as_str() == "login")
    //     .unwrap_or(false)
    // {
    //     true
    // } else if let Some(max_age) = req_data.max_age {
    //     if let Some(session) = &principal.session {
    //         let session_created = session.exp - *SESSION_LIFETIME as i64;
    //         Utc::now().timestamp() > session_created + max_age
    //     } else {
    //         true
    //     }
    // } else {
    //     false
    // };

    // // check if the user needs to do the Webauthn login each time
    // let mut action = FrontendAction::None;
    // if let Ok(mfa_cookie) = WebauthnCookie::parse_validate(&ApiCookie::from_req(&req, COOKIE_MFA)) {
    //     if let Ok(user) = User::find_by_email(mfa_cookie.email.clone()).await {
    //         // we need to check this, because a user could deactivate MFA in another browser or
    //         // be deleted while still having existing mfa cookies somewhere else
    //         if user.has_webauthn_enabled() {
    //             action = FrontendAction::MfaLogin(mfa_cookie.email);

    //             // if the user must do another MFA login anyway, we do never force a new session creation,
    //             // because the authentication happens each time anyway
    //             force_new_session = false;
    //         }
    //     }
    // }

    // // check for `prompt=no-prompt`
    // if !force_new_session
    //     && req_data
    //         .prompt
    //         .as_ref()
    //         .map(|p| p.as_str() == "none")
    //         .unwrap_or(false)
    // {
    //     let status = StatusCode::UNAUTHORIZED;
    //     let body = Error1Html::build(&colors, &lang, status, Some("login_required"));
    //     return Ok(ErrorHtml::response(body, status));
    // }

    // let auth_providers_json = AuthProviderTemplate::get_all_json_template().await?;
    // let tpl_data = Some(format!(
    //     "{}\n{}\n{}",
    //     client.name.unwrap_or_default(),
    //     client.client_uri.unwrap_or_default(),
    //     *OPEN_USER_REG,
    // ));

    // // if the user is still authenticated and everything is valid -> immediate refresh
    // if !force_new_session && principal.validate_session_auth().is_ok() {
    //     let csrf = principal.get_session_csrf_token()?;
    //     let body = AuthorizeHtml::build(
    //         &tpl_data,
    //         csrf,
    //         FrontendAction::Refresh,
    //         &colors,
    //         &lang,
    //         auth_providers_json,
    //     );

    //     if let Some(o) = origin_header {
    //         return Ok(HttpResponse::Ok()
    //             .insert_header(o)
    //             .insert_header(HEADER_HTML)
    //             .body(body));
    //     }
    //     return Ok(HttpResponse::Ok().append_header(HEADER_HTML).body(body));
    // }
    // // check if we can re-use a still valid session or need to create a new one
    // let session = if let Some(session) = &principal.session {
    //     match principal.validate_session_auth_or_init() {
    //         Ok(_) => session.clone(),
    //         Err(_) => Session::new(*SESSION_LIFETIME, Some(real_ip_from_req(&req)?)),
    //     }
    // } else {
    //     Session::new(*SESSION_LIFETIME, Some(real_ip_from_req(&req)?))
    // };

    // if let Err(err) = session.save().await {
    //     let status = err.status_code();
    //     let body = Error1Html::build(&colors, &lang, status, Some(err.message));
    //     return Ok(ErrorHtml::response(body, status));
    // }

    let body = AuthorizeHtml::build(
        &None,
        "crsf_token",
        FrontendAction::None,
        &colors,
        &lang,
        None,
    );

    // let cookie = session.client_cookie();
    // if let Some(o) = origin_header {
    //     // TODO is 'Access-Control-Allow-Credentials: true' needed as well?
    //     return Ok(HttpResponse::Ok()
    //         .cookie(cookie)
    //         .insert_header(o)
    //         .insert_header(HEADER_HTML)
    //         .body(body));
    // }

    // if *EXPERIMENTAL_FED_CM_ENABLE {
    //     Ok(HttpResponse::build(StatusCode::OK)
    //         .cookie(session.client_cookie_fed_cm())
    //         .cookie(cookie)
    //         .insert_header(HEADER_HTML)
    //         .body(body))
    // } else {
    Ok(HttpResponse::build(StatusCode::OK)
        // .cookie(cookie)
        .insert_header(HEADER_HTML)
        // .insert_header((
        //     header::ACCESS_CONTROL_ALLOW_ORIGIN,
        //     HeaderValue::from_str("*").unwrap(),
        // ))
        .body(body))
    // }
}

#[utoipa::path(
    post,
    path = "/atproto/login",
    tag = "atproto",
    responses(
        (status = 202, description = "Accepted"),
    ),
)]
#[post("/atproto/login")]
pub async fn post_login(
    data: web::Data<AppState>,
    payload: Json<atproto::LoginRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth_or_init()?;

    let payload = payload.into_inner();
    let (cookie, xsrf_token, location) = Callback::login_start(&data, payload).await?;

    Ok(HttpResponse::Accepted()
        .insert_header((LOCATION, location))
        // .insert_header((
        //     header::ACCESS_CONTROL_ALLOW_ORIGIN,
        //     HeaderValue::from_str("*").unwrap(),
        // ))
        .cookie(cookie)
        .body(xsrf_token))
}

#[utoipa::path(
    post,
    path = "/atproto/callback",
    tag = "atproto",
    responses(
        (status = 200, description = "OK"),
    ),
)]
#[post("/atproto/callback")]
pub async fn post_callback(
    data: web::Data<AppState>,
    req: HttpRequest,
    payload: Query<atproto::CallbackRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth_or_init()?;

    let payload = payload.into_inner();
    let session = principal.get_session()?;
    let (auth_step, cookie) =
        Callback::login_finish(&data, &req, &payload, session.clone()).await?;

    let mut resp = map_auth_step(auth_step, &req).await?;
    resp.add_cookie(&cookie).map_err(|err| {
        ErrorResponse::new(
            ErrorResponseType::Internal,
            format!("Error adding cookie after map_auth_step: {}", err),
        )
    })?;

    // resp.headers_mut().append(
    //     header::ACCESS_CONTROL_ALLOW_ORIGIN,
    //     HeaderValue::from_str("*").unwrap(),
    // );
    Ok(resp)
}
