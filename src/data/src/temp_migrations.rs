use crate::database::DB;
use hiqlite::macros::params;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use tracing::info;

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

    Ok(())
}
