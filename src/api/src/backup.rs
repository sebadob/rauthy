use crate::ReqPrincipal;
use actix_web::http::header::CONTENT_DISPOSITION;
use actix_web::mime::APPLICATION_OCTET_STREAM;
use actix_web::web::Path;
use actix_web::{HttpResponse, get, post};
use bytes::Bytes;
use futures::{SinkExt, StreamExt, TryStreamExt};
use rauthy_api_types::backup::{BackupListing, BackupListings};
use rauthy_data::database::DB;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use tokio::io::{AsyncReadExt, BufReader};
use tokio::task;
use tracing::error;

/// Show currently existing DB backups
///
/// This will only work if the configured database is a Hiqlite.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/backup",
    tag = "backup",
    responses(
        (status = 200, description = "Ok", body = BackupListings),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/backup")]
pub async fn get_backups(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;
    validate_hiqlite()?;

    let local = DB::hql()
        .backup_list_local()
        .await?
        .into_iter()
        .map(|l| BackupListing {
            name: l.name,
            last_modified: l.last_modified,
            size: l.size,
        })
        .collect::<Vec<_>>();

    let s3 = DB::hql()
        .backup_list_s3()
        .await?
        .into_iter()
        .map(|l| BackupListing {
            name: l.name,
            last_modified: l.last_modified,
            size: l.size,
        })
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(BackupListings { local, s3 }))
}

/// Trigger a one-of backup
///
/// This will only work if the configured database is a Hiqlite.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/backup",
    tag = "backup",
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/backup")]
pub async fn post_backup(principal: ReqPrincipal) -> Result<(), ErrorResponse> {
    principal.validate_admin_session()?;
    validate_hiqlite()?;

    DB::hql().backup().await?;

    Ok(())
}

#[inline]
fn validate_hiqlite() -> Result<(), ErrorResponse> {
    if RauthyConfig::get().vars.database.hiqlite {
        Ok(())
    } else {
        Err(ErrorResponse::new(
            ErrorResponseType::NotFound,
            "Backups can only be triggered for Hiqlite",
        ))
    }
}

/// Download a local backup
///
/// This will only work if the configured database is a Hiqlite.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/backup/local/{filename}",
    tag = "backup",
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/backup/local/{filename}")]
pub async fn get_backup_local(
    filename: Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;
    validate_hiqlite()?;

    let file = DB::hql().backup_file_local(&filename).await?;
    let mut rdr = BufReader::new(file);

    let (mut tx, rx) = futures::channel::mpsc::channel(1);

    task::spawn(async move {
        let mut buf = [0u8; 8 * 1024];
        while let Ok(len) = rdr.read(&mut buf).await {
            if len == 0 {
                break;
            }
            if tx
                .send(Ok::<Bytes, String>(Bytes::from(buf[..len].to_vec())))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    Ok(HttpResponse::Ok()
        .content_type(APPLICATION_OCTET_STREAM)
        .insert_header((
            CONTENT_DISPOSITION,
            format!("attachment; filename=\"{filename}\""),
        ))
        .streaming(rx.into_stream()))
}

/// Download an S3 backup
///
/// This will only work if the configured database is a Hiqlite and you have S3 backups configured.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/backup/s3/{object}",
    tag = "backup",
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/backup/s3/{object}")]
pub async fn get_backup_s3(
    object: Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;
    validate_hiqlite()?;

    let object = object.into_inner();
    let mut rx_s3 = DB::hql().backup_s3_stream(object.clone())?;

    let (mut tx, rx) = futures::channel::mpsc::channel(1);

    task::spawn(async move {
        while let Some(res) = rx_s3.next().await {
            match res {
                Ok(bytes) => {
                    if tx
                        .send(Ok::<Bytes, String>(Bytes::from(bytes)))
                        .await
                        .is_err()
                    {
                        break;
                    }
                }
                Err(err) => {
                    error!(?err, "Download S3 Backup error");
                    break;
                }
            }
        }
    });

    Ok(HttpResponse::Ok()
        .content_type(APPLICATION_OCTET_STREAM)
        .insert_header((
            CONTENT_DISPOSITION,
            format!("attachment; filename=\"{object}\""),
        ))
        .streaming(rx.into_stream()))
}
