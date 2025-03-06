use crate::database::DB;
use crate::entity::users::User;
use actix_web::http::header::{CACHE_CONTROL, CONTENT_TYPE};
use actix_web::HttpResponse;
use futures_util::StreamExt;
use hiqlite::{params, Param};
use rauthy_common::is_hiqlite;
use rauthy_common::utils::new_store_id;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::env;
use std::string::ToString;
use std::sync::LazyLock;
use tokio::fs;
use tracing::{debug, error, info};

const CACHE_CTRL_PICTURE: &str = "max-age=31104000, stale-while-revalidate=2592000";
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
    /// Inserts a new Avatar for a user and returns the generated `id`.
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

    /// Deletes the Avatar in the DB - does NOT delete the file on the storage.
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
    pub async fn upload(
        user_id: String,
        mut payload: actix_multipart::Multipart,
    ) -> Result<HttpResponse, ErrorResponse> {
        // TODO
        // - delete possibly existing picture
        // - create a new DB entry with a new ID for proper caching
        // - insert into DB
        // - push to storage
        // - update user.picture_id

        let mut user = User::find(user_id).await?;
        if let Some(picture_id) = user.picture_id {
            Self::remove(picture_id, user.id.clone()).await?;
        }

        let mut content_type = None;
        let mut is_img = false;
        let mut buf: Vec<u8> = Vec::with_capacity(16 * 1024);
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

        if is_img {
            // if we have a "real" image: parse, reduce size, convert to webp
            todo!()
        }

        let new_id = match *PICTURE_STORAGE_TYPE {
            PictureStorage::DB => {
                Self::insert(
                    // cannot be None at this point - checked in field receive already
                    content_type.unwrap(),
                    PICTURE_STORAGE_TYPE.clone(),
                    Some(buf),
                )
                .await?
            }
            PictureStorage::File => {
                todo!()
            }
            PictureStorage::S3 => {
                todo!()
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
                    todo!("delete PictureStorage::File")
                }
                PictureStorage::S3 => {
                    todo!("delete PictureStorage::S3")
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
                todo!("download PictureStorage::File")
            }
            PictureStorage::S3 => {
                todo!("download PictureStorage::S3")
            }
        }
    }

    /// Tests the Picture configuration. Should be executed at application startup.
    pub async fn test_config() -> Result<(), ErrorResponse> {
        match *PICTURE_STORAGE_TYPE {
            PictureStorage::DB => {
                // no need to test anything
                info!("Using Database as User Picture Storage");
            }
            PictureStorage::File => {
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
