use crate::database::DB;
use hiqlite::{params, Param};
use rauthy_common::constants::RAUTHY_VERSION;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use semver::Version;
use serde::{Deserialize, Serialize};
use sqlx::query;
use std::str::FromStr;
use tracing::{debug, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct DbVersion {
    pub version: Version,
}

impl DbVersion {
    pub async fn find() -> Result<Option<Self>, ErrorResponse> {
        if is_hiqlite() {
            let mut rows = DB::client()
                .query_raw("SELECT data FROM config WHERE id = 'db_version'", params!())
                .await?;
            if rows.is_empty() {
                return Ok(None);
            }
            let bytes: Vec<u8> = rows.remove(0).get("data");
            let version = bincode::deserialize::<Self>(&bytes)?;
            Ok(Some(version))
        } else {
            let res = query!("SELECT data FROM config WHERE id = 'db_version'")
                .fetch_optional(DB::conn())
                .await?;
            match res {
                Some(record) => {
                    let data = record
                        .data
                        .expect("to get 'data' back from the AppVersion query");
                    Ok(Some(bincode::deserialize::<Self>(&data)?))
                }
                None => Ok(None),
            }
        }
    }

    pub async fn upsert(db_version: Option<Version>) -> Result<(), ErrorResponse> {
        let app_version = Self::app_version();
        if Some(&app_version) != db_version.as_ref() {
            let slf = Self {
                version: app_version,
            };
            let data = bincode::serialize(&slf)?;

            if is_hiqlite() {
                DB::client()
                    .execute(
                        r#"INSERT INTO config (id, data)
                        VALUES ('db_version', $1)
                        ON CONFLICT(id) DO UPDATE SET data = $1"#,
                        params!(data),
                    )
                    .await?;
            } else {
                query!(
                    r#"INSERT INTO config (id, data)
                    VALUES ('db_version', $1)
                    ON CONFLICT(id) DO UPDATE SET data = $1"#,
                    data,
                )
                .execute(DB::conn())
                .await?;
            }
        }

        Ok(())
    }

    pub async fn check_app_version() -> Result<Option<Version>, ErrorResponse> {
        let app_version = Self::app_version();
        debug!("Current Rauthy Version: {:?}", app_version);

        // check DB version for compatibility
        // We check the `config` table first instead of db version, because the db version does not
        // exist in early versions while the `config` does from the very beginning.
        let db_exists = if is_hiqlite() {
            DB::client()
                .query_raw("SELECT id FROM config LIMIT 1", params!())
                .await
                .is_ok()
        } else {
            query!("SELECT id FROM config LIMIT 1")
                .fetch_one(DB::conn())
                .await
                .is_ok()
        };

        if !db_exists {
            return Ok(None);
        }

        let db_version = match Self::find().await? {
            None => {
                debug!("No Current DB Version found");
                Self::is_db_compatible(&app_version, None).await?;
                None
            }
            Some(db_version) => {
                debug!("Current DB Version: {:?}", db_version);
                Self::is_db_compatible(&app_version, Some(&db_version.version)).await?;
                Some(db_version.version)
            }
        };

        Ok(db_version)
    }

    /// Checks if we can use an existing (possibly older) db with this version of rauthy, or if
    /// the user may need to take action beforehand.
    async fn is_db_compatible(
        app_version: &Version,
        db_version: Option<&Version>,
    ) -> Result<(), ErrorResponse> {
        // this check panics on purpose, and it is there to never forget to adjust this
        // version check before doing any major or minor release
        if app_version.major != 0 || app_version.minor != 28 {
            panic!(
                "\nDbVersion::check_app_version needs adjustment for the new RAUTHY_VERSION: {}",
                RAUTHY_VERSION
            );
        }

        // warn on prerelease usage
        if !app_version.pre.is_empty() {
            warn!(
                "!!! Caution: you are using a prerelease version: {} !!!",
                app_version.pre.as_str()
            );
        }

        // check for the lowest DB version we can use with this App Version
        if let Some(db_version) = db_version {
            let lowest_compatible_version = Version::parse("0.20.0").unwrap();

            if db_version < &lowest_compatible_version {
                panic!(
                    "Your database is too old for this upgrade.\n\
                    Rauthy {} needs at least a DB version {}\n\
                    Please check https://github.com/sebadob/rauthy/releases for additional information.",
                    app_version, lowest_compatible_version,
                );
            }

            return Ok(());
        }

        // check the DB version in another way if we did not find an existing DB version

        // from v0.16.0 on we did have the db_version inside the `config` table,
        // which is already checked above

        // the passkeys table was introduced with v0.15.0
        let is_db_v0_15_0 = if is_hiqlite() {
            DB::client()
                .query_raw(
                    "SELECT * FROM sqlite_master WHERE type = 'table' AND name = 'passkeys' LIMIT 1",
                    params!(),
                )
                .await
                .is_err()
        } else {
            query!("SELECT * FROM pg_tables WHERE tablename = 'passkeys' LIMIT 1")
                .fetch_one(DB::conn())
                .await
                .is_err()
        };
        if is_db_v0_15_0 {
            panic!(
                "Your database is Rauthy v0.15.0. You need to upgrade to Rauthy v0.16 first.\n\
                Please check https://github.com/sebadob/rauthy/releases for additional information."
            );
        }

        // To check for any DB older than 0.15.0, we check for the existence of the 'clients' table
        // which is there since the very beginning.
        let is_db_pre_v0_15_0 = if is_hiqlite() {
            DB::client()
                .query_raw(
                    "SELECT * FROM sqlite_master WHERE type = 'table' AND name = 'clients' LIMIT 1",
                    params!(),
                )
                .await
                .is_err()
        } else {
            query!("SELECT * FROM pg_tables WHERE tablename = 'clients' LIMIT 1")
                .fetch_one(DB::conn())
                .await
                .is_err()
        };
        if is_db_pre_v0_15_0 {
            panic!(
                "Your database is older than Rauthy v0.15.0. You need to upgrade to Rauthy v0.15 first.\n\
                Please check https://github.com/sebadob/rauthy/releases for additional information."
            );
        }

        // Since we did not find the clients table, we can assume, that the DB is really empty.
        Ok(())
    }
}

impl DbVersion {
    pub fn app_version() -> Version {
        Version::from_str(RAUTHY_VERSION).expect("bad format for RAUTHY_VERSION")
    }
}
