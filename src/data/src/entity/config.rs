use rauthy_derive::FromPgRow;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, FromPgRow)]
pub struct ConfigEntity {
    pub id: String,
    pub data: Vec<u8>,
}
