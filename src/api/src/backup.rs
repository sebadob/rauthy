use crate::ReqPrincipal;
use actix_web::body::SizedStream;
use actix_web::http::header::CONTENT_DISPOSITION;
use actix_web::mime::APPLICATION_OCTET_STREAM;
use actix_web::web::Path;
use actix_web::{HttpResponse, get, post};
use bytes::Bytes;
use futures::channel::mpsc::Sender;
use futures::{SinkExt, StreamExt, TryStreamExt};
use rauthy_api_types::backup::{BackupListing, BackupListings};
use rauthy_data::database::DB;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use tokio::io::{AsyncRead, AsyncReadExt, BufReader};
use tokio::task;
use tracing::error;

/// The item type of the backup download body stream. An `Err` makes actix abort the response
/// body, so the client sees a failed download instead of a short file that looks complete.
type BackupChunk = Result<Bytes, String>;

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
    let len = file.metadata().await?.len();
    let rdr = BufReader::new(file);

    let (tx, rx) = futures::channel::mpsc::channel(1);

    task::spawn(pump_reader(rdr, tx, filename.clone(), len));

    // `SizedStream` sets a `Content-Length`, so any client can detect a short body on its own.
    Ok(HttpResponse::Ok()
        .content_type(APPLICATION_OCTET_STREAM)
        .insert_header((
            CONTENT_DISPOSITION,
            format!("attachment; filename=\"{filename}\""),
        ))
        .body(SizedStream::new(len, rx.into_stream())))
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
    let object_err = object.clone();

    let (mut tx, rx) = futures::channel::mpsc::channel(1);

    task::spawn(async move {
        while let Some(res) = rx_s3.next().await {
            match res {
                Ok(bytes) => {
                    if tx.send(Ok(Bytes::from(bytes))).await.is_err() {
                        // client hung up
                        break;
                    }
                }
                Err(err) => {
                    error!(?err, "Download S3 Backup error");
                    // Fail the body instead of ending it cleanly, which would hand out a
                    // truncated backup with a 200. The S3 error itself only goes to the log.
                    let _ = tx
                        .send(Err(format!(
                            "S3 backup {object_err} could not be read in full; \
                            see the server log"
                        )))
                        .await;
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

/// Streams `rdr` into `tx`, sending exactly `expected` bytes or ending with an `Err`.
///
/// A read error or an early EOF must not end the body like a regular EOF, because the client
/// would then receive a truncated backup with a 200.
async fn pump_reader<R>(mut rdr: R, mut tx: Sender<BackupChunk>, name: String, expected: u64)
where
    R: AsyncRead + Unpin,
{
    let mut buf = [0u8; 8 * 1024];
    let mut sent: u64 = 0;
    loop {
        // never send more than the declared `Content-Length`
        let room = match expected.checked_sub(sent) {
            Some(0) | None => break,
            Some(room) => room.min(buf.len() as u64) as usize,
        };

        match rdr.read(&mut buf[..room]).await {
            Ok(0) => {
                // EOF before the declared length
                error!("Local backup {name} ended after {sent} of {expected} bytes");
                let _ = tx
                    .send(Err(format!(
                        "Backup {name} ended after {sent} of {expected} bytes"
                    )))
                    .await;
                break;
            }
            Ok(len) => {
                if tx
                    .send(Ok(Bytes::copy_from_slice(&buf[..len])))
                    .await
                    .is_err()
                {
                    // client hung up
                    break;
                }
                sent += len as u64;
            }
            Err(err) => {
                error!(?err, "Error reading local backup {name}");
                let _ = tx
                    .send(Err(format!("Backup {name} could not be read: {err}")))
                    .await;
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use tokio::io::ReadBuf;

    /// Yields `head` and then fails, like a file whose backing storage dies mid-read.
    struct FailingReader {
        head: Vec<u8>,
    }

    impl AsyncRead for FailingReader {
        fn poll_read(
            mut self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<io::Result<()>> {
            if self.head.is_empty() {
                return Poll::Ready(Err(io::Error::other("backing storage went away")));
            }
            let take = self.head.len().min(buf.remaining());
            let chunk: Vec<u8> = self.head.drain(..take).collect();
            buf.put_slice(&chunk);
            Poll::Ready(Ok(()))
        }
    }

    async fn drain(rx: futures::channel::mpsc::Receiver<BackupChunk>) -> Vec<BackupChunk> {
        rx.collect::<Vec<_>>().await
    }

    #[tokio::test]
    async fn a_complete_read_streams_every_byte_and_no_error() {
        let (tx, rx) = futures::channel::mpsc::channel(1);
        let body = vec![7u8; 20 * 1024];
        let len = body.len() as u64;
        tokio::spawn(pump_reader(
            io::Cursor::new(body.clone()),
            tx,
            "good.sqlite".to_string(),
            len,
        ));

        let chunks = drain(rx).await;
        assert!(
            chunks.iter().all(Result::is_ok),
            "a healthy read must not produce a stream error"
        );
        let streamed: Vec<u8> = chunks
            .into_iter()
            .flat_map(|c| c.unwrap().to_vec())
            .collect();
        assert_eq!(streamed, body);
    }

    /// A body must never exceed the length the response declared, whatever the reader offers.
    #[tokio::test]
    async fn the_body_never_exceeds_the_declared_length() {
        let (tx, rx) = futures::channel::mpsc::channel(1);
        // The reader has more than the declared length: only the declared length may go out.
        tokio::spawn(pump_reader(
            io::Cursor::new(vec![3u8; 20 * 1024]),
            tx,
            "grew.sqlite".to_string(),
            12 * 1024,
        ));

        let chunks = drain(rx).await;
        assert!(chunks.iter().all(Result::is_ok), "no error is warranted");
        let streamed: usize = chunks.into_iter().map(|c| c.unwrap().len()).sum();
        assert_eq!(streamed, 12 * 1024);
    }

    /// And it must never fall short of it silently either.
    #[tokio::test]
    async fn a_body_shorter_than_declared_ends_in_an_error() {
        let (tx, rx) = futures::channel::mpsc::channel(1);
        tokio::spawn(pump_reader(
            io::Cursor::new(vec![4u8; 4 * 1024]),
            tx,
            "shrank.sqlite".to_string(),
            32 * 1024,
        ));

        let chunks = drain(rx).await;
        let err = chunks
            .last()
            .expect("some bytes went out first")
            .as_ref()
            .expect_err(
                "a short file must not end the body cleanly under a longer declared length",
            );
        assert!(
            err.contains("4096 of 32768"),
            "the shortfall must be stated: {err}"
        );
    }

    /// The regression this guards: a mid-stream read error used to end the body cleanly, so the
    /// client saw a 200 with a short file and no way to tell it was short.
    #[tokio::test]
    async fn a_read_failure_reaches_the_client_as_a_stream_error() {
        let (tx, rx) = futures::channel::mpsc::channel(1);
        tokio::spawn(pump_reader(
            FailingReader {
                head: vec![1u8; 4 * 1024],
            },
            tx,
            "truncated.sqlite".to_string(),
            64 * 1024,
        ));

        let chunks = drain(rx).await;
        let last = chunks.last().expect("the reader yielded some bytes first");
        let err = last
            .as_ref()
            .expect_err("a failed read must terminate the body with an error, not with EOF");
        assert!(
            err.contains("truncated.sqlite") && err.contains("backing storage went away"),
            "the cause must survive to the log and the stream: {err}"
        );
    }
}
