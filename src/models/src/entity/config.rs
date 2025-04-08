use serde::Deserialize;
use sqlx::FromRow;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Debug, Clone, FromRow, Deserialize, PostgresMapper)]
#[pg_mapper(table = "config")]
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
