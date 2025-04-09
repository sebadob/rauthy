use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigEntity {
    pub id: String,
    pub data: Vec<u8>,
}

impl From<tokio_postgres::Row> for ConfigEntity {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            data: row.get("data"),
        }
    }
}
