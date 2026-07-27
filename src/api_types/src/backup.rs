use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct BackupListing {
    pub name: String,
    pub last_modified: i64,
    pub size: Option<u64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BackupListings {
    pub local: Vec<BackupListing>,
    pub s3: Vec<BackupListing>,
}
