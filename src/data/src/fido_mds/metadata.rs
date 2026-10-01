use crate::database::DB;
use hiqlite::macros::FromRow;
use hiqlite::params;
use rauthy_common::is_hiqlite;
use rauthy_derive::FromPgRow;
use rauthy_error::ErrorResponse;

#[derive(Debug, Default, FromRow, FromPgRow)]
pub struct MdsMetadata {
    pub current_blob_no: i64,
    pub next_update: i64,
}

impl MdsMetadata {
    pub async fn find() -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM fido_mds_metadata";
        let slf = if is_hiqlite() {
            DB::hql().query_map_one(sql, params!()).await?
        } else {
            DB::pg_query_one(sql, &[]).await?
        };
        Ok(slf)
    }
}
