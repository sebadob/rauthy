use crate::database::DB;
use crate::entity::config::ConfigEntity;
use hiqlite_macros::params;
use rauthy_common::constants::RAUTHY_VERSION;
use rauthy_common::is_hiqlite;
use rauthy_common::utils::{deserialize, serialize};
use rauthy_error::ErrorResponse;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tracing::{debug, warn};

static LOWEST_COMPATIBLE_VERSION: &str = "0.30.0";

#[derive(Debug, Serialize, Deserialize)]
pub struct DbVersion {
    pub version: Version,
}

impl DbVersion {
    pub async fn find() -> Option<Self> {
        let sql = "SELECT * FROM config WHERE id = 'db_version'";
        let bytes: Vec<u8> = if is_hiqlite() {
            let config: ConfigEntity = DB::hql().query_as_optional(sql, params!()).await.ok()??;
            config.data
        } else {
            let config: ConfigEntity = DB::pg_query_opt(sql, &[]).await.ok()??;
            config.data
        };

        deserialize::<Self>(&bytes).ok()
    }

    pub async fn upsert(db_version: Option<Version>) -> Result<(), ErrorResponse> {
        let app_version = Self::app_version();
        if Some(&app_version) != db_version.as_ref() {
            let slf = Self {
                version: app_version,
            };
            let data = serialize(&slf)?;

            let sql = r#"
INSERT INTO config (id, data)
VALUES ('db_version', $1)
ON CONFLICT(id) DO UPDATE SET data = $1"#;

            if is_hiqlite() {
                DB::hql().execute(sql, params!(data)).await?;
            } else {
                DB::pg_execute(sql, &[&data]).await?;
            }
        }

        Ok(())
    }

    pub async fn check_app_version() -> Result<Option<Version>, ErrorResponse> {
        let app_version = Self::app_version();
        debug!("Current Rauthy Version: {app_version:?}");

        // check DB version for compatibility
        // We check the `config` table first instead of db version, because the db version does not
        // exist in early versions while the `config` does from the very beginning.
        let sql = "SELECT id FROM config LIMIT 1";
        let db_exists = if is_hiqlite() {
            DB::hql().query_raw(sql, params!()).await.is_ok()
        } else {
            DB::pg_query_one_row(sql, &[]).await.is_ok()
        };

        if !db_exists {
            return Ok(None);
        }

        let db_version = match Self::find().await {
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
        if app_version.major != 0 || app_version.minor != 35 {
            panic!(
                "\nDbVersion::check_app_version needs adjustment for the new RAUTHY_VERSION: \
                {RAUTHY_VERSION}\\n Also make sure that `LOWEST_COMPATIBLE_VERSION` is still \
                correctly set",
            );
        }

        // warn on prerelease usage
        if !app_version.pre.is_empty() {
            warn!(
                "!!! Caution: you are using a pre-release version: {} - DO NOT USE IN PRODUCTION !!!",
                app_version.pre.as_str()
            );
        }

        // check for the lowest DB version we can use with this App Version
        if let Some(db_version) = db_version {
            let lowest_compatible_version = Version::parse(LOWEST_COMPATIBLE_VERSION).unwrap();

            if db_version < &lowest_compatible_version {
                panic!(
                    "Your database is too old for this upgrade.\n\
                    Rauthy {app_version} needs at least a DB version {lowest_compatible_version}\n\
                    Please check https://github.com/sebadob/rauthy/releases for additional \
                    information.",
                );
            }

            return Ok(());
        }

        // check the DB version in another way if we did not find an existing DB version

        // from v0.16.0 on we did have the db_version inside the `config` table,
        // which is already checked above

        // the passkeys table was introduced with v0.15.0
        let is_db_v0_15_0 = if is_hiqlite() {
            DB::hql().query_raw("SELECT * FROM sqlite_master WHERE type = 'table' AND name = 'passkeys' LIMIT 1", params!()).await.is_err()
        } else {
            DB::pg_query_one_row(
                "SELECT * FROM pg_tables WHERE tablename = 'passkeys' LIMIT 1",
                &[],
            )
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
            DB::hql()
                .query_raw(
                    "SELECT * FROM sqlite_master WHERE type = 'table' AND name = 'clients' LIMIT 1",
                    params!(),
                )
                .await
                .is_err()
        } else {
            DB::pg_query_one_row(
                "SELECT * FROM pg_tables WHERE tablename = 'clients' LIMIT 1",
                &[],
            )
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
