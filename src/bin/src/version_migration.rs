use hiqlite::params;
use rauthy_common::is_hiqlite;
use rauthy_models::database::DB;
use rauthy_models::entity::password::PasswordPolicy;
use std::error::Error;
use tracing::warn;

/// If it's necessary to apply manual migrations between major versions, which are
/// not handled automatically by database migrations, put them here. This function
/// will be executed at startup after DB init and before the API start.
pub async fn manual_version_migrations() -> Result<(), Box<dyn Error>> {
    // TODO remove this block check after `0.28`.
    // 0.27.0 had a bug that could have inserted NULL for password policy on update.
    if is_hiqlite() {
        let mut row = DB::client()
            .query_raw_one(
                "SELECT data FROM config WHERE id = 'password_policy'",
                params!(),
            )
            .await?;
        if let Err(err) = row.try_get::<Vec<u8>>("data") {
            warn!(
                r#"

Error looking up PasswordPolicy - this is most probably a known 0.27.0 bug.
Inserting default Policy to fix it.
You should visit the Admin UI -> Config -> Password Policy and adjust it to your needs.

Error: {}
"#,
                err
            );
            PasswordPolicy {
                length_min: 14,
                length_max: 128,
                include_lower_case: Some(1),
                include_upper_case: Some(1),
                include_digits: Some(1),
                include_special: Some(1),
                valid_days: Some(180),
                not_recently_used: Some(3),
            }
            .save()
            .await
            .expect("Cannot fix default PasswordPolicy issue");
        }
    }

    // TODO remove this block with `0.29`
    if is_hiqlite() {
        DB::client()
            .execute(
                "UPDATE users SET language = 'zhhans' WHERE language = 'zh-Hans';",
                params!(),
            )
            .await?;
    }

    Ok(())
}
