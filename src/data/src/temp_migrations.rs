use crate::database::{Cache, DB};
use crate::entity::user_attr::UserAttrValueEntity;
use hiqlite_macros::params;
use rauthy_common::constants::{IDX_GROUPS, IDX_ROLES};
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use tracing::info;

pub async fn apply_temp_migrations() -> Result<(), ErrorResponse> {
    let sql = "SELECT * FROM user_attr_values";
    let values: Vec<UserAttrValueEntity> = if is_hiqlite() {
        DB::hql().query_as(sql, params!()).await?
    } else {
        DB::pg_query(sql, &[], 0).await?
    };

    let mut migrated = 0;

    for value in values {
        let json = serde_json::from_slice::<serde_json::Value>(&value.value)?;
        let serde_json::Value::String(s) = json else {
            continue;
        };

        // This is the easiest method to check if the String is actually another JSON obj.
        let Ok(json) = serde_json::from_str::<serde_json::Value>(s.as_str()) else {
            // If this fails, we actually have a String here - nothing to do.
            continue;
        };
        let value_json = serde_json::to_vec(&json)?;

        let sql = "UPDATE user_attr_values SET value = $1 WHERE user_id = $2 AND key = $3";
        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(value_json, value.user_id, value.key))
                .await?;
        } else {
            DB::pg_execute(sql, &[&value_json, &value.user_id, &value.key]).await?;
        }
        migrated += 1;
    }

    if migrated > 0 {
        info!("Migrated {} user attr values to proper JSON", migrated);
    }

    // because of changes on groups and roles, we must clear caches
    DB::hql().delete(Cache::App, IDX_GROUPS).await?;
    DB::hql().delete(Cache::App, IDX_ROLES).await?;

    Ok(())
}
