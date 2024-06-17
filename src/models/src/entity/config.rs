use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct ConfigEntity {
    pub id: String,
    pub data: Vec<u8>,
}
