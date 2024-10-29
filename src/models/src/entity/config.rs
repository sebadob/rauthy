use serde::Deserialize;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Deserialize)]
pub struct ConfigEntity {
    pub id: String,
    pub data: Vec<u8>,
}
