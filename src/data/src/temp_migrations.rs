use crate::database::DB;
use crate::entity::roles::Role;
use hiqlite::macros::params;
use rauthy_common::constants::RAUTHY_ADMIN_GROUP_PREFIX;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use tracing::{info, warn};

pub async fn apply_temp_migrations() -> Result<(), ErrorResponse> {
    // cleanup possibly lingering PAM user groups
    let sql = r#"
DELETE FROM pam_groups
WHERE typ = 'user' AND NOT EXISTS (
    SELECT 1 FROM pam_users
    WHERE pam_users.name = pam_groups.name
)"#;
    let rows_affected = if is_hiqlite() {
        DB::hql().execute(sql, params!()).await?
    } else {
        DB::pg_execute(sql, &[]).await?
    };
    if rows_affected > 0 {
        info!("Cleaned up {rows_affected} lingering PAM Groups from and older cleanup bug");
    }

    warn_existing_group_admin_roles().await?;

    Ok(())
}

/// #1538: the delegated group-admin feature reads roles named `rauthy_admin:<prefix>`
/// as group admins. This is non-breaking unless such a role already existed before the
/// upgrade, in which case its holders silently gain group-admin rights. We warn about
/// any matching role on each startup for the whole `v0.36` cycle so operators can spot
/// an unintended collision; roles created intentionally afterwards can ignore it.
async fn warn_existing_group_admin_roles() -> Result<(), ErrorResponse> {
    let found = Role::find_all()
        .await?
        .into_iter()
        .filter(|r| r.name.starts_with(RAUTHY_ADMIN_GROUP_PREFIX))
        .map(|r| r.name)
        .collect::<Vec<_>>();
    if !found.is_empty() {
        warn!(
            "Found custom roles matching the delegated group-admin scheme \
            `rauthy_admin:<prefix>`: {found:?}. Their holders are now group admins for \
            the matching groups (see #1538). If you created these intentionally, you \
            can safely ignore this warning."
        );
    }
    Ok(())
}
