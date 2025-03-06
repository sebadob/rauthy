use crate::entity::theme::ThemeCssFull;
use chrono::Utc;
use rauthy_common::utils::new_store_id;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PictureStorage {
    DB,
    File,
    S3,
    Unknown,
}

impl From<&str> for PictureStorage {
    fn from(value: &str) -> Self {
        match value {
            "db" => Self::DB,
            "file" => Self::File,
            "s3" => Self::S3,
            x => {
                error!("Invalid Avatar storage location: {}", x);
                Self::Unknown
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
            Self::Unknown => "unknown",
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

impl UserPicture {
    /// Inserts a new Avatar for a user and returns the generated `id`.
    pub async fn insert(
        content_type: String,
        storage: PictureStorage,
        data: Option<Vec<u8>>,
    ) -> Result<String, ErrorResponse> {
        let id = new_store_id();
        let now = Utc::now().timestamp();

        todo!()
    }

    /// Deletes the Avatar in the DB - does NOT delete the file on the storage.
    pub async fn delete(id: String) -> Result<(), ErrorResponse> {
        todo!()
    }

    pub async fn find(id: String) -> Result<Self, ErrorResponse> {
        todo!()
    }
}
