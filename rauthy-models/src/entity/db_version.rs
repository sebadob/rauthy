use crate::app_state::DbPool;
use rauthy_common::constants::RAUTHY_VERSION;
use rauthy_common::error_response::ErrorResponse;
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
    pub async fn find(db: &DbPool) -> Option<Self> {
        let res = query!("select data from config where id = 'db_version'")
            .fetch_optional(db)
            .await
            .ok()?;
        match res {
            Some(res) => {
                let data = res
                    .data
                    .expect("to get 'data' back from the AppVersion query");
                bincode::deserialize::<Self>(&data).ok()
            }
            None => None,
        }
    }

    pub async fn update(db: &DbPool, db_version: Option<Version>) -> Result<(), ErrorResponse> {
        let app_version = Self::app_version();
        if Some(&app_version) != db_version.as_ref() {
            let slf = Self {
                version: app_version,
            };
            let data = bincode::serialize(&slf)?;

            #[cfg(feature = "sqlite")]
            let q = query!(
                "insert or replace into config (id, data) values ('db_version', $1)",
                data,
            );
            #[cfg(not(feature = "sqlite"))]
            let q = query!(
                r#"insert into config (id, data) values ('db_version', $1)
                on conflict(id) do update set data = $1"#,
                data,
            );
            q.execute(db).await?;
        }

        Ok(())
    }

    pub async fn check_app_version(db: &DbPool) -> Result<Option<Version>, ErrorResponse> {
        let app_version = Self::app_version();
        debug!("Current Rauthy Version: {:?}", app_version);

        // check DB version for compatibility
        let db_exists = query!("select id from config limit 1")
            .fetch_one(db)
            .await
            .is_ok();
        let db_version = match Self::find(db).await {
            None => {
                debug!(" No Current DB Version found");
                if db_exists {
                    Self::is_db_compatible(db, &app_version, None).await?;
                }

                None
            }
            Some(db_version) => {
                debug!("Current DB Version: {:?}", db_version);

                if db_exists {
                    Self::is_db_compatible(db, &app_version, Some(&db_version.version)).await?;
                }

                Some(db_version.version)
            }
        };

        Ok(db_version)
    }

    /// Checks if we can use an existing (possibly older) db with this version of rauthy, or if
    /// the user may need to take action beforehand.
    async fn is_db_compatible(
        db: &DbPool,
        app_version: &Version,
        db_version: Option<&Version>,
    ) -> Result<(), ErrorResponse> {
        // this check panics on purpose and it is there to never forget to adjust this
        // version check before doing any major or minor release
        if app_version.major != 0 || app_version.minor != 16 {
            panic!(
                "\nDbVersion::check_app_version needs adjustment for the new RAUTHY_VERSION: {}",
                RAUTHY_VERSION
            );
        }

        // warn on prerelease usage
        if !app_version.pre.is_empty() {
            warn!("!!! Caution: you are using a prerelease version !!!");
        }

        // check for the lowest DB version we can use with this App Version
        if let Some(db_version) = db_version {
            if db_version.major != 0 || db_version.minor < 15 || db_version.minor > 16 {
                panic!(
                    "\nRauthy {} needs at least a DB version v0.15 and max v0.16",
                    app_version
                );
            }

            return Ok(());
        }

        // check the DB version in another way if we did not find an existing DB version

        // the passkeys table was introduced with v0.15.0
        #[cfg(not(feature = "sqlite"))]
        let passkeys_exist = query!(
            r#"select * from pg_tables
            where schemaname = 'rauthy' and tablename = 'passkeys' limit 1"#
        )
        .fetch_one(db)
        .await;
        #[cfg(feature = "sqlite")]
        let passkeys_exist = query!(
            "select * from sqlite_master where type = 'table' and name = 'passkeys' limit 1"
        )
        .fetch_one(db)
        .await;
        if passkeys_exist.is_err() {
            panic!("\nYou need to start at least rauthy v0.15 before you can upgrade");
        }

        Ok(())
    }
}

impl DbVersion {
    pub fn app_version() -> Version {
        Version::from_str(RAUTHY_VERSION).expect("bad format for RAUTHY_VERSION")
    }
}
