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

    //     pub async fn replace(blob_no: i64, next_update: i64) -> Result<(), ErrorResponse> {
    //         let sql_delete = "DELETE FROM fido_mds_metadata";
    //         let sql_insert = r#"
    // INSERT INTO fido_mds_metadata (current_blob_no, next_update)
    // VALUES ($1, $2)
    // "#;
    //
    //         if is_hiqlite() {
    //             let mut txn = Vec::with_capacity(2);
    //             txn.push((sql_delete, params!()));
    //             txn.push((sql_insert, params!(blob_no, next_update)));
    //             DB::hql().txn(txn).await?;
    //         } else {
    //             let mut cl = DB::pg().await?;
    //             let txn = cl.transaction().await?;
    //             DB::pg_txn_append(&txn, sql_delete, &[]).await?;
    //             DB::pg_txn_append(&txn, sql_delete, &[&blob_no, &next_update]).await?;
    //             txn.commit().await?;
    //         }
    //
    //         Ok(())
    //     }
}
