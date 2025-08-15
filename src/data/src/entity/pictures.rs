use crate::database::DB;
use crate::entity::logos::Logo;
use crate::entity::users::User;
use crate::rauthy_config::RauthyConfig;
use actix_web::http::StatusCode;
use actix_web::http::header::{CACHE_CONTROL, CONTENT_TYPE, HeaderName, HeaderValue};
use actix_web::{HttpResponse, HttpResponseBuilder, web};
use futures_util::StreamExt;
use hiqlite_macros::params;
use image::ImageFormat;
use image::imageops::FilterType;
use rauthy_common::is_hiqlite;
use rauthy_common::utils::new_store_id;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::string::ToString;
use std::sync::LazyLock;
use tokio::fs;
use tracing::{debug, error, info};

const CACHE_CTRL_PICTURE: &str = "max-age=31104000, stale-while-revalidate=2592000";
const PICTURE_SIZE_PX: u32 = 192;

pub static PICTURE_STORAGE_TYPE: LazyLock<PictureStorage> = LazyLock::new(|| {
    PictureStorage::from(RauthyConfig::get().vars.user_pictures.storage_type.as_ref())
});
static PICTURE_S3_BUCKET: LazyLock<s3_simple::Bucket> = LazyLock::new(|| {
    let cfg = &RauthyConfig::get().vars.user_pictures;
    let host = cfg
        .s3_url
        .as_ref()
        .expect("`user_pictures.s3_url` not set")
        .parse::<Url>()
        .expect("invalid `user_pictures.s3_url`");

    let name = cfg.bucket.clone().expect("`user_pictures.bucket` not set");
    let region = s3_simple::Region(cfg.region.clone().expect("PIC_S3_REGION not set"));

    let key = cfg.s3_key.clone().expect("`user_pictures.s3_key` not set");
    let secret = cfg
        .s3_secret
        .clone()
        .expect("`user_pictures.s3_secret` not set");
    let creds = s3_simple::Credentials::new(key, secret);

    let opts = s3_simple::BucketOptions {
        path_style: cfg.s3_path_style,
        list_objects_v2: true,
    };

    s3_simple::Bucket::new(host, name, region, creds, Some(opts))
        .expect("Cannot connect to Picture S3 Bucket")
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PictureStorage {
    DB,
    File,
    S3,
    Disabled,
}

impl From<&str> for PictureStorage {
    fn from(value: &str) -> Self {
        match value {
            "db" => Self::DB,
            "file" => Self::File,
            "s3" => Self::S3,
            "disabled" => Self::Disabled,
            x => {
                error!("Invalid picture storage: {x} - upload disabled");
                Self::Disabled
            }
        }
    }
}

impl PictureStorage {
    pub fn as_str(&self) -> &str {
        match self {
            Self::DB => "db",
            Self::File => "file",
            Self::S3 => "s3",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UserPicture {
    pub id: String,
    pub content_type: String,
    pub storage: String,
    pub data: Option<Vec<u8>>,
}

impl From<tokio_postgres::Row> for UserPicture {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            content_type: row.get("content_type"),
            storage: row.get("storage"),
            data: row.get("data"),
        }
    }
}

// CRUD
impl UserPicture {
    /// Inserts a new UserPicture for a user and returns the generated `id`.
    async fn insert(
        content_type: String,
        storage: PictureStorage,
        data: Option<Vec<u8>>,
    ) -> Result<String, ErrorResponse> {
        let id = new_store_id();
        let sql = r#"
INSERT INTO pictures (id, content_type, storage, data)
VALUES ($1, $2, $3, $4)"#;

        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(&id, content_type, storage.as_str(), data))
                .await?;
        } else {
            DB::pg_execute(sql, &[&id, &content_type, &storage.as_str(), &data]).await?;
        }

        Ok(id)
    }

    /// Deletes the UserPicture in the DB - does NOT delete the file on the storage.
    async fn delete(id: String) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM pictures WHERE id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(id)).await?;
        } else {
            DB::pg_execute(sql, &[&id]).await?;
        }

        Ok(())
    }

    async fn find(id: String) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM pictures WHERE id = $1";
        let slf = if is_hiqlite() {
            DB::hql().query_as_one(sql, params!(id)).await?
        } else {
            DB::pg_query_one(sql, &[&id]).await?
        };

        Ok(slf)
    }
}

impl UserPicture {
    #[inline]
    fn local_file_path(picture_id: &str, content_type: &str) -> String {
        format!(
            "{}/{picture_id}.{}",
            RauthyConfig::get().vars.user_pictures.path,
            Self::file_ending(content_type)
        )
    }

    #[inline]
    fn file_name(picture_id: &str, content_type: &str) -> String {
        format!("{picture_id}.{}", Self::file_ending(content_type))
    }

    #[inline]
    fn file_ending(content_type: &str) -> &str {
        if content_type.contains("svg") {
            "svg"
        } else {
            "webp"
        }
    }

    pub async fn upload(
        user_id: String,
        mut payload: actix_multipart::Multipart,
    ) -> Result<HttpResponse, ErrorResponse> {
        // make sure to always create a new picture with a new id for more efficient caching
        let mut user = User::find(user_id).await?;
        if let Some(picture_id) = user.picture_id {
            Self::remove(picture_id, user.id.clone()).await?;
        }

        // read image data into memory (actix web limits max body size)
        let mut content_type = "";
        let mut is_img = false;
        let mut buf: Vec<u8> = Vec::with_capacity(32 * 1024);
        if let Some(part) = payload.next().await {
            let mut field = part?;

            match field.content_type() {
                Some(mime) => {
                    debug!(content_type = ?mime);
                    let s = mime.as_ref();
                    match s {
                        "image/svg+xml" => {
                            content_type = "image/svg+xml";
                        }
                        "image/jpeg" | "image/png" | "image/webp" => {
                            is_img = true;
                            // all non-webp images will be converted to webp in the following lines
                            content_type = "image/webp";
                        }
                        _ => {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "invalid content_type, expected one of: image/svg+xml, \
                                image/jpeg, image/png, image/webp",
                            ));
                        }
                    }
                }
                None => {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::BadRequest,
                        "content_type is missing",
                    ));
                }
            }

            while let Some(chunk) = field.next().await {
                let bytes = chunk?;
                buf.extend(bytes);
            }
        }

        // make sure to parse, reduce in size, and convert image
        let bytes = if is_img {
            // if we have a "real" image: parse, reduce size, convert to webp
            let mut img = image::load_from_memory(&buf)?;
            web::block(move || async move {
                if img.width() > PICTURE_SIZE_PX || img.height() > PICTURE_SIZE_PX {
                    img =
                        img.resize_to_fill(PICTURE_SIZE_PX, PICTURE_SIZE_PX, FilterType::Lanczos3);
                }
                // most of these pictures end up somewhere between 25 and 35kB
                let mut buf = Cursor::new(Vec::with_capacity(32 * 1024));
                img.write_to(&mut buf, ImageFormat::WebP)?;
                Ok::<_, ErrorResponse>(buf.into_inner())
            })
            .await?
            .await?
        } else {
            Logo::sanitize_svg(&mut buf)?
        };

        // save img to db + storage
        let new_id = match *PICTURE_STORAGE_TYPE {
            PictureStorage::DB => {
                Self::insert(
                    content_type.to_string(),
                    PICTURE_STORAGE_TYPE.clone(),
                    Some(bytes),
                )
                .await?
            }
            PictureStorage::File => {
                let id = Self::insert(content_type.to_string(), PICTURE_STORAGE_TYPE.clone(), None)
                    .await?;
                fs::write(Self::local_file_path(&id, content_type), bytes).await?;
                id
            }
            PictureStorage::S3 => {
                let id = Self::insert(content_type.to_string(), PICTURE_STORAGE_TYPE.clone(), None)
                    .await?;
                PICTURE_S3_BUCKET
                    .put(&Self::file_name(&id, content_type), &bytes)
                    .await?;
                id
            }
            PictureStorage::Disabled => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "user picture upload is disabled",
                ));
            }
        };

        user.picture_id = Some(new_id);
        user.save(None).await?;

        Ok(HttpResponse::Ok().body(user.picture_id.unwrap()))
    }

    pub async fn remove(id: String, user_id: String) -> Result<(), ErrorResponse> {
        let mut user = User::find(user_id).await?;
        if let Some(picture_id) = user.picture_id {
            if id != picture_id {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "invalid picture_id for this user",
                ));
            }
            let slf = Self::find(id).await?;

            match PictureStorage::from(slf.storage.as_str()) {
                PictureStorage::DB => {
                    // no additional data exists
                }
                PictureStorage::File => {
                    let path = Self::local_file_path(&slf.id, &slf.content_type);
                    if let Err(err) = fs::remove_file(&path).await {
                        error!(path, ?err, "Error cleaning up local picture")
                    }
                }
                PictureStorage::S3 => {
                    if let Err(err) = PICTURE_S3_BUCKET
                        .delete(&Self::file_name(&slf.id, &slf.content_type))
                        .await
                    {
                        error!(object_id = slf.id, ?err, "Error cleaning up s3 picture")
                    }
                }
                PictureStorage::Disabled => unreachable!(),
            }

            Self::delete(slf.id).await?;

            user.picture_id = None;
            user.save(None).await?;
        }

        Ok(())
    }

    pub async fn download(
        id: String,
        cors_origin: Option<(HeaderName, HeaderValue)>,
    ) -> Result<HttpResponse, ErrorResponse> {
        let slf = Self::find(id).await?;

        let mut resp = HttpResponseBuilder::new(StatusCode::OK);
        resp.insert_header((CONTENT_TYPE, HeaderValue::from_str(&slf.content_type)?));
        resp.insert_header((CACHE_CONTROL, CACHE_CTRL_PICTURE));
        if let Some((n, v)) = cors_origin {
            resp.insert_header((n, v));
        }

        match PictureStorage::from(slf.storage.as_str()) {
            PictureStorage::DB => {
                if let Some(data) = slf.data {
                    Ok(resp.body(data))
                } else {
                    error!("PictureStorage::DB but `data` is NULL: {slf:?}");
                    Err(ErrorResponse::new(
                        ErrorResponseType::Internal,
                        "invalid storage type",
                    ))
                }
            }
            PictureStorage::File => {
                // TODO stream from file instead of loading into memory upfront
                let bytes = fs::read(Self::local_file_path(&slf.id, &slf.content_type)).await?;
                Ok(resp.body(bytes))
            }
            PictureStorage::S3 => {
                let res = PICTURE_S3_BUCKET
                    .get(&Self::file_name(&slf.id, &slf.content_type))
                    .await?;
                Ok(resp.streaming(res.bytes_stream()))
            }
            PictureStorage::Disabled => unreachable!(),
        }
    }

    /// Tests the Picture configuration. Should be executed at application startup.
    pub async fn test_config() -> Result<(), ErrorResponse> {
        match *PICTURE_STORAGE_TYPE {
            PictureStorage::DB => {
                info!("Using Database as User Picture Storage");
            }
            PictureStorage::File => {
                if RauthyConfig::get().is_ha_cluster {
                    panic!(
                        "You can only use local file storage for User Pictures for a single instance"
                    );
                }

                // make sure the path exists and is available
                let cfg = &RauthyConfig::get().vars.user_pictures;
                fs::create_dir_all(cfg.path.as_ref()).await?;
                info!(
                    path = cfg.path.as_ref(),
                    "Using local filesystem as User Picture Storage"
                );
            }
            PictureStorage::S3 => {
                // make sure the S3 bucket is configured properly
                PICTURE_S3_BUCKET
                    .list("", None)
                    .await
                    .expect("Cannot list User Picture S3 Bucket");
                info!(
                    bucket = PICTURE_S3_BUCKET.name,
                    "Using S3 bucket as User Picture Storage - connection test successful",
                );
            }
            PictureStorage::Disabled => {
                info!("User Picture upload disabled");
            }
        }

        Ok(())
    }
}
