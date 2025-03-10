use crate::database::DB;
use crate::entity::users::User;
use actix_web::http::header::{CACHE_CONTROL, CONTENT_TYPE};
use actix_web::{web, HttpResponse};
use futures_util::StreamExt;
use hiqlite::{params, Param};
use image::imageops::FilterType;
use image::ImageFormat;
use rauthy_common::is_hiqlite;
use rauthy_common::utils::new_store_id;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::Cursor;
use std::string::ToString;
use std::sync::LazyLock;
use tokio::fs;
use tracing::{debug, error, info};

const CACHE_CTRL_PICTURE: &str = "max-age=31104000, stale-while-revalidate=2592000";
const PICTURE_SIZE_PX: u32 = 192;

pub static PICTURE_STORAGE_TYPE: LazyLock<PictureStorage> = LazyLock::new(|| {
    let s = env::var("PICTURE_STORAGE_TYPE").unwrap_or_else(|_| "db".to_string());
    PictureStorage::from(s.as_str())
});
static PICTURE_PATH: LazyLock<String> =
    LazyLock::new(|| env::var("PICTURE_PATH").unwrap_or_else(|_| "./pictures".to_string()));
static PICTURE_S3_BUCKET: LazyLock<s3_simple::Bucket> = LazyLock::new(|| {
    let host = env::var("PIC_S3_URL")
        .expect("PIC_S3_URL not set")
        .parse::<Url>()
        .expect("invalid PIC_S3_URL");
    let name = env::var("PIC_S3_BUCKET").expect("PIC_S3_BUCKET not set");
    let region = s3_simple::Region(env::var("PIC_S3_REGION").expect("PIC_S3_REGION not set"));

    let key = env::var("PIC_S3_KEY").expect("PIC_S3_KEY not set");
    let secret = env::var("PIC_S3_SECRET").expect("PIC_S3_SECRET not set");
    let creds = s3_simple::Credentials::new(key, secret);

    let opts = s3_simple::BucketOptions {
        path_style: env::var("PIC_S3_PATH_STYLE")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .expect("Cannot parse PIC_S3_PATH_STYLE as bool"),
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
}

impl From<&str> for PictureStorage {
    fn from(value: &str) -> Self {
        match value {
            "db" => Self::DB,
            "file" => Self::File,
            "s3" => Self::S3,
            x => {
                panic!("Invalid picture storage: {}", x)
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
        }
    }
}

#[derive(Debug, Deserialize, sqlx::FromRow)]
pub struct UserPicture {
    pub id: String,
    pub content_type: String,
    pub storage: String,
    pub data: Option<Vec<u8>>,
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

        if is_hiqlite() {
            DB::client()
                .execute(
                    r#"
INSERT INTO pictures (id, content_type, storage, data)
VALUES ($1, $2, $3, $4)"#,
                    params!(&id, content_type, storage.as_str(), data),
                )
                .await?;
        } else {
            sqlx::query!(
                r#"
INSERT INTO pictures (id, content_type, storage, data)
VALUES ($1, $2, $3, $4)"#,
                &id,
                content_type,
                storage.as_str(),
                data
            )
            .execute(DB::conn())
            .await?;
        }

        Ok(id)
    }

    /// Deletes the UserPicture in the DB - does NOT delete the file on the storage.
    async fn delete(id: String) -> Result<(), ErrorResponse> {
        if is_hiqlite() {
            DB::client()
                .execute("DELETE FROM pictures WHERE id = $1", params!(id))
                .await?;
        } else {
            sqlx::query!("DELETE FROM pictures WHERE id = $1", id)
                .execute(DB::conn())
                .await?;
        }

        Ok(())
    }

    async fn find(id: String) -> Result<Self, ErrorResponse> {
        let slf = if is_hiqlite() {
            DB::client()
                .query_as_one("SELECT * FROM pictures WHERE id = $1", params!(id))
                .await?
        } else {
            sqlx::query_as!(Self, "SELECT * FROM pictures WHERE id = $1", id)
                .fetch_one(DB::conn())
                .await?
        };

        Ok(slf)
    }
}

impl UserPicture {
    #[inline]
    fn local_file_path(picture_id: &str) -> String {
        format!("{}/{}", PICTURE_PATH.as_str(), picture_id)
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
        let mut content_type = None;
        let mut is_img = false;
        let mut buf: Vec<u8> = Vec::with_capacity(32 * 1024);
        if let Some(part) = payload.next().await {
            let mut field = part?;

            match field.content_type() {
                Some(mime) => {
                    debug!("content_type: {:?}", mime);
                    let s = mime.as_ref();
                    match s {
                        "image/svg+xml" => {}
                        "image/jpeg" | "image/png" => is_img = true,
                        _ => {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "invalid content_type, expected one of: image/svg+xml, \
                                image/jpeg, image/png",
                            ));
                        }
                    }
                    content_type = Some(s.to_string());
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
            buf
        };

        // save img to db + storage
        let new_id = match *PICTURE_STORAGE_TYPE {
            PictureStorage::DB => {
                Self::insert(
                    content_type.unwrap(),
                    PICTURE_STORAGE_TYPE.clone(),
                    Some(bytes),
                )
                .await?
            }
            PictureStorage::File => {
                let id =
                    Self::insert(content_type.unwrap(), PICTURE_STORAGE_TYPE.clone(), None).await?;
                fs::write(Self::local_file_path(&id), bytes).await?;
                id
            }
            PictureStorage::S3 => {
                let id =
                    Self::insert(content_type.unwrap(), PICTURE_STORAGE_TYPE.clone(), None).await?;
                PICTURE_S3_BUCKET.put(&id, &bytes).await?;
                id
            }
        };

        user.picture_id = Some(new_id);
        user.save(None).await?;

        Ok(HttpResponse::Ok().body(user.picture_id.unwrap()))
    }

    pub async fn remove(id: String, user_id: String) -> Result<(), ErrorResponse> {
        let user = User::find(user_id).await?;
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
                    let path = Self::local_file_path(&picture_id);
                    if let Err(err) = fs::remove_file(&path).await {
                        error!("Error cleaning up local picture {}: {}", path, err)
                    }
                }
                PictureStorage::S3 => {
                    if let Err(err) = PICTURE_S3_BUCKET.delete(&picture_id).await {
                        error!("Error cleaning up s3 picture {}: {}", picture_id, err)
                    }
                }
            }

            Self::delete(slf.id).await?;
        }

        Ok(())
    }

    pub async fn download(id: String) -> Result<HttpResponse, ErrorResponse> {
        let slf = Self::find(id).await?;

        match PictureStorage::from(slf.storage.as_str()) {
            PictureStorage::DB => {
                if let Some(data) = slf.data {
                    Ok(HttpResponse::Ok()
                        .insert_header((CONTENT_TYPE, slf.content_type))
                        .insert_header((CACHE_CONTROL, CACHE_CTRL_PICTURE))
                        .body(data))
                } else {
                    error!("PictureStorage::DB but `data` is NULL: {:?}", slf);
                    Err(ErrorResponse::new(
                        ErrorResponseType::Internal,
                        "invalid storage type",
                    ))
                }
            }
            PictureStorage::File => {
                // TODO stream from file instead of loading into memory upfront
                let bytes = fs::read(Self::local_file_path(&slf.id)).await?;
                Ok(HttpResponse::Ok()
                    .insert_header((CONTENT_TYPE, slf.content_type))
                    .insert_header((CACHE_CONTROL, CACHE_CTRL_PICTURE))
                    .body(bytes))
            }
            PictureStorage::S3 => {
                let res = PICTURE_S3_BUCKET.get(&slf.id).await?;
                Ok(HttpResponse::Ok()
                    .insert_header((CONTENT_TYPE, slf.content_type))
                    .insert_header((CACHE_CONTROL, CACHE_CTRL_PICTURE))
                    .streaming(res.bytes_stream()))
            }
        }
    }

    /// Tests the Picture configuration. Should be executed at application startup.
    pub async fn test_config() -> Result<(), ErrorResponse> {
        match *PICTURE_STORAGE_TYPE {
            PictureStorage::DB => {
                info!("Using Database as User Picture Storage");
            }
            PictureStorage::File => {
                let nodes = env::var("HQL_NODES")
                    .unwrap()
                    .lines()
                    .filter_map(|l| {
                        let trim = l.trim();
                        if trim.is_empty() {
                            None
                        } else {
                            Some(trim)
                        }
                    })
                    .count();
                if nodes != 1 {
                    panic!("You can only use local file storage for User Pictures for a single instance");
                }

                // make sure the path exists and is available
                fs::create_dir_all(PICTURE_PATH.as_str()).await?;
                info!(
                    "Using local filesystem as User Picture Storage: {}",
                    PICTURE_PATH.as_str()
                );
            }
            PictureStorage::S3 => {
                // make sure the S3 bucket is configured properly
                PICTURE_S3_BUCKET
                    .list("", None)
                    .await
                    .expect("Cannot list User Picture S3 Bucket");
                info!(
                    "Using S3 bucket {} as User Picture Storage - connection test successful",
                    PICTURE_S3_BUCKET.name
                );
            }
        }

        Ok(())
    }
}
