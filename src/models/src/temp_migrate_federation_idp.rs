use crate::database::DB;
use crate::entity::users::User;
use hiqlite_macros::params;
use rauthy_common::constants::RAUTHY_VERSION;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use tracing::{error, info};

// TODO remove this whole file with v0.31+
pub async fn migrate_fed_ids() -> Result<(), ErrorResponse> {
    // TODO remove after v0.30
    if semver::Version::parse(RAUTHY_VERSION).unwrap().minor > 30 {
        panic!("Remove the `migrate_fed_ids()` after v0.30");
    }

    let sql = "SELECT * FROM users WHERE federation_uid LIKE '\"%\"'";
    let users: Vec<User> = if is_hiqlite() {
        DB::hql().query_as(sql, params!()).await?
    } else {
        DB::pg_query(sql, &[], 0).await?
    };

    if !users.is_empty() {
        info!(
            "Migration Auth Provider User IDs for {} users.",
            users.len()
        );
    }

    let sql = "UPDATE users SET federation_uid = $1 WHERE id = $2";
    for user in users {
        if user.federation_uid.is_none() {
            error!(
                "User {} with no `federation_uid` returned in migration query, this should never happen",
                user.email
            );
            continue;
        }

        let fed_id = user.federation_uid.unwrap();
        if fed_id.len() < 2 {
            error!(
                "User {} with too short `federation_uid` in migration query, this should never happen",
                user.email
            );
            continue;
        }

        assert_eq!(&fed_id[..1], "\"");
        assert_eq!(&fed_id[fed_id.len() - 1..], "\"");
        let stripped = &fed_id[1..fed_id.len() - 1];

        if is_hiqlite() {
            DB::hql().execute(sql, params!(stripped, user.id)).await?;
        } else {
            DB::pg_execute(sql, &[&stripped, &user.id]).await?;
        }
    }

    info!("`federation_uid`s migrated successfully");
    Ok(())
}
