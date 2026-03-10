use crate::database::DB;
use crate::migration::bootstrap::bootstrap_data;
use crate::migration::bootstrap::types::Role;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_common::utils::new_store_id;
use rauthy_error::ErrorResponse;
use tracing::info;

pub async fn bootstrap() -> Result<(), ErrorResponse> {
    let roles = bootstrap_data!(Role, "roles");

    let len = roles.len();
    let sql = "INSERT INTO roles (id, name) VALUES ($1, $2)";

    for role in roles {
        let id = role.id.unwrap_or_else(new_store_id);

        if is_hiqlite() {
            DB::hql().execute(sql, params!(id, role.name)).await?;
        } else {
            DB::pg_execute(sql, &[&id, &role.name]).await?;
        }
    }

    info!("Migrated {len} roles.");

    Ok(())
}
