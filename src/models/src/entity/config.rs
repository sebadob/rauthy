use serde::Deserialize;
use sqlx::FromRow;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Debug, Clone, FromRow, Deserialize, PostgresMapper)]
#[pg_mapper(table = "config")]
pub struct ConfigEntity {
    pub id: String,
    pub data: Vec<u8>,
}
