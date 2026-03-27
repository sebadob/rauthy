use crate::database::DB;
use crate::migration::bootstrap::bootstrap_data;
use crate::migration::bootstrap::types::Group;
use hiqlite::macros::params;
use rauthy_common::is_hiqlite;
use rauthy_common::utils::new_store_id;
use rauthy_error::ErrorResponse;
use tracing::info;

pub async fn bootstrap() -> Result<(), ErrorResponse> {
    let groups = bootstrap_data!(Group, "groups");

    let len = groups.len();
    let sql = "INSERT INTO groups (id, name) VALUES ($1, $2)";

    for group in groups {
        let id = group.id.unwrap_or_else(new_store_id);

        if is_hiqlite() {
            DB::hql().execute(sql, params!(id, group.name)).await?;
        } else {
            DB::pg_execute(sql, &[&id, &group.name]).await?;
        }
    }

    info!("Migrated {len} groups.");

    Ok(())
}
