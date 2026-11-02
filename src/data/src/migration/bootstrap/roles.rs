use crate::database::{Cache, DB};
use crate::entity::roles::Role as RoleEntity;
use crate::migration::bootstrap::types::Role;
use crate::migration::bootstrap::{bootstrap_data, find_existing};
use hiqlite::macros::params;
use rauthy_common::constants::IDX_ROLES;
use rauthy_common::is_hiqlite;
use rauthy_common::utils::new_store_id;
use rauthy_error::ErrorResponse;
use tracing::info;

pub async fn load() -> Result<Vec<Role>, ErrorResponse> {
    Ok(bootstrap_data!(Role, "roles"))
}

pub async fn bootstrap() -> Result<(), ErrorResponse> {
    let roles = bootstrap_data!(Role, "roles");

    let len = roles.len();
    for role in roles {
        insert(role).await?;
    }

    info!("Migrated {len} roles.");

    Ok(())
}

/// Creates missing and renames changed roles. Never deletes anything.
pub async fn reconcile(roles: Vec<Role>) -> Result<(), ErrorResponse> {
    let existing = RoleEntity::find_all().await?;
    let (mut created, mut updated) = (0, 0);

    for role in roles {
        match find_existing(&existing, role.id.as_deref(), &role.name, |e| {
            (&e.id, &e.name)
        }) {
            Some(e) if e.name == role.name => {}
            Some(e) => {
                // `update()` also renames the role on all its users
                let meta = e
                    .meta
                    .as_deref()
                    .and_then(|m| serde_json::from_slice(m).ok());
                RoleEntity::update(e.id.clone(), role.name, meta).await?;
                updated += 1;
            }
            None => {
                insert(role).await?;
                created += 1;
            }
        }
    }

    if created > 0 {
        // `insert()` bypasses the entity and therefore its cache
        DB::hql().delete(Cache::App, IDX_ROLES).await?;
    }
    info!("Reconciled roles: {created} created, {updated} updated.");

    Ok(())
}

async fn insert(role: Role) -> Result<(), ErrorResponse> {
    let sql = "INSERT INTO roles (id, name) VALUES ($1, $2)";
    let id = role.id.unwrap_or_else(new_store_id);

    if is_hiqlite() {
        DB::hql().execute(sql, params!(id, role.name)).await?;
    } else {
        DB::pg_execute(sql, &[&id, &role.name]).await?;
    }
    Ok(())
}
