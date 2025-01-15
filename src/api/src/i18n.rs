use crate::AcceptEncoding;
use actix_web::body::BoxBody;
use actix_web::http::header::{CACHE_CONTROL, CONTENT_ENCODING, CONTENT_TYPE, ETAG};
use actix_web::http::StatusCode;
use actix_web::web::Path;
use actix_web::{get, HttpRequest, HttpResponse};
use rauthy_common::constants::{APPLICATION_JSON, BUILD_TIME};
use rauthy_error::ErrorResponse;
use rauthy_models::i18n::ui::I18n;
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
