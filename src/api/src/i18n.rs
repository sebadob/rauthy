use crate::AcceptEncoding;
use actix_web::body::BoxBody;
use actix_web::http::header::{CACHE_CONTROL, CONTENT_ENCODING, CONTENT_TYPE, ETAG};
use actix_web::http::StatusCode;
use actix_web::web::{Json, Path};
use actix_web::{get, post, HttpRequest, HttpResponse};
use rauthy_api_types::generic::{I18nContent, I18nRequest};
use rauthy_common::constants::{APPLICATION_JSON, BUILD_TIME};
use rauthy_error::ErrorResponse;
use rauthy_models::i18n::email::confirm_change_html::I18nEmailConfirmChangeHtml;
use rauthy_models::i18n::ui::account::I18nAccount;
use rauthy_models::i18n::ui::authorize::I18nAuthorize;
use rauthy_models::i18n::ui::device::I18nDevice;
use rauthy_models::i18n::ui::error::I18nError;
use rauthy_models::i18n::ui::index::I18nIndex;
use rauthy_models::i18n::ui::logout::I18nLogout;
use rauthy_models::i18n::ui::password_reset::I18nPasswordReset;
use rauthy_models::i18n::ui::register::I18nRegister;
use rauthy_models::i18n::ui::{I18n, SsrJson};
use rauthy_models::language::Language;

#[get("/i18n/{lang}")]
pub async fn get_i18n(lang: Path<String>, req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    let etag = BUILD_TIME.timestamp().to_string();

    if let Some(etag_remote) = req.headers().get("if-none-match") {
        let etag_remote = etag_remote.to_str().unwrap_or_default();
        if etag_remote == etag {
            return Ok(HttpResponse::new(StatusCode::from_u16(304).unwrap()));
        }
    }

    let lang = Language::from(lang.as_str());
    let enc = AcceptEncoding::from(req.headers());
    let body = match enc {
        AcceptEncoding::Br => BoxBody::new(I18n::build_br(&lang).await?),
        AcceptEncoding::Gzip => BoxBody::new(I18n::build_gzip(&lang).await?),
        AcceptEncoding::None => BoxBody::new(I18n::build_json(&lang)?),
    };

    Ok(HttpResponse::build(StatusCode::OK)
        .insert_header((CONTENT_TYPE, APPLICATION_JSON))
        .insert_header((CONTENT_ENCODING, enc.value()))
        .insert_header((
            CACHE_CONTROL,
            "max-age=31536000, stale-while-revalidate=2592000, public",
        ))
        .insert_header((ETAG, etag))
        .body(body))
}

// TODO remove this endpoint after i18n migration is complete.
// This is only needed for the old setup.
#[post("/i18n")]
pub async fn post_i18n(
    req: HttpRequest,
    // no validation needed for I18nRequest
    Json(payload): Json<I18nRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    let lang = Language::try_from(&req).unwrap_or_default();
    let body = match payload.content {
        I18nContent::Authorize => I18nAuthorize::build(&lang).as_json(),
        I18nContent::Account => I18nAccount::build(&lang).as_json(),
        I18nContent::Device => I18nDevice::build(&lang).as_json(),
        I18nContent::EmailChangeConfirm => I18nEmailConfirmChangeHtml::build(&lang).as_json(),
        // Just return some default values for local dev -> dynamically built during prod
        I18nContent::Error => {
            I18nError::build_with(&lang, StatusCode::NOT_FOUND, Some("<empty>")).as_json()
        }
        I18nContent::Index => I18nIndex::build(&lang).as_json(),
        I18nContent::Logout => I18nLogout::build(&lang).as_json(),
        I18nContent::PasswordReset => I18nPasswordReset::build(&lang).as_json(),
        I18nContent::Register => I18nRegister::build(&lang).as_json(),
    };

    Ok(HttpResponse::Ok()
        .insert_header((CONTENT_TYPE, APPLICATION_JSON))
        .body(body))
}
