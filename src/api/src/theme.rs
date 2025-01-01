use crate::AcceptEncoding;
use actix_web::body::BoxBody;
use actix_web::http::header::{CACHE_CONTROL, CONTENT_ENCODING, CONTENT_TYPE, ETAG};
use actix_web::http::StatusCode;
use actix_web::{get, web, HttpRequest, HttpResponse};
use rauthy_error::ErrorResponse;
use rauthy_models::entity::theme::ThemeCssFull;

#[get("/theme/{client_id}")]
pub async fn get_theme(
    path: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    let client_id = path.into_inner();
    let etag = ThemeCssFull::etag(&client_id).await?;

    if let Some(etag_remote) = req.headers().get("if-none-match") {
        let etag_remote = etag_remote.to_str().unwrap_or_default();
        if etag_remote == etag {
            return Ok(HttpResponse::new(StatusCode::from_u16(304).unwrap()));
        }
    }

    let enc = AcceptEncoding::from(req.headers());
    let body = match enc {
        AcceptEncoding::Br => BoxBody::new(ThemeCssFull::br(&client_id).await?),
        AcceptEncoding::Gzip => BoxBody::new(ThemeCssFull::gzip(&client_id).await?),
        AcceptEncoding::None => BoxBody::new(ThemeCssFull::plain(&client_id).await?),
    };

    Ok(HttpResponse::build(StatusCode::OK)
        .insert_header((CONTENT_TYPE, "text/css"))
        .insert_header((CONTENT_ENCODING, enc.value()))
        .insert_header((
            CACHE_CONTROL,
            "max-age=43200, stale-while-revalidate=2592000, public",
        ))
        .insert_header((ETAG, etag))
        .body(body))
}
