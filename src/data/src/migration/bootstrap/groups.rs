use crate::database::{Cache, DB};
use crate::entity::groups::Group as GroupEntity;
use crate::migration::bootstrap::types::Group;
use crate::migration::bootstrap::{bootstrap_data, find_existing};
use hiqlite::macros::params;
use rauthy_common::constants::IDX_GROUPS;
use rauthy_common::is_hiqlite;
use rauthy_common::utils::new_store_id;
use rauthy_error::ErrorResponse;
use tracing::info;

pub async fn load() -> Result<Vec<Group>, ErrorResponse> {
    Ok(bootstrap_data!(Group, "groups"))
}

pub async fn bootstrap() -> Result<(), ErrorResponse> {
    let groups = bootstrap_data!(Group, "groups");

    let len = groups.len();
    for group in groups {
        insert(group).await?;
    }

    info!("Migrated {len} groups.");

    Ok(())
}

/// Creates missing and renames changed groups. Never deletes anything.
pub async fn reconcile(groups: Vec<Group>) -> Result<(), ErrorResponse> {
    let existing = GroupEntity::find_all().await?;
    let (mut created, mut updated) = (0, 0);

    for group in groups {
        match find_existing(&existing, group.id.as_deref(), &group.name, |e| {
            (&e.id, &e.name)
        }) {
            Some(e) if e.name == group.name => {}
            Some(e) => {
                // `update()` also renames the group on all its users
                let meta = e
                    .meta
                    .as_deref()
                    .and_then(|m| serde_json::from_slice(m).ok());
                GroupEntity::update(e.id.clone(), group.name, meta).await?;
                updated += 1;
            }
            None => {
                insert(group).await?;
                created += 1;
            }
        }
    }

    if created > 0 {
        // `insert()` bypasses the entity and therefore its cache
        DB::hql().delete(Cache::App, IDX_GROUPS).await?;
    }
    info!("Reconciled groups: {created} created, {updated} updated.");

    Ok(())
}

async fn insert(group: Group) -> Result<(), ErrorResponse> {
    let sql = "INSERT INTO groups (id, name) VALUES ($1, $2)";
    let id = group.id.unwrap_or_else(new_store_id);

    if is_hiqlite() {
        DB::hql().execute(sql, params!(id, group.name)).await?;
    } else {
        DB::pg_execute(sql, &[&id, &group.name]).await?;
    }
    Ok(())
}
