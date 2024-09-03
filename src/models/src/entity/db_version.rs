use crate::app_state::DbPool;
use rauthy_common::constants::RAUTHY_VERSION;
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
    pub async fn find(db: &DbPool) -> Option<Self> {
        let res = query!("SELECT data FROM config WHERE id = 'db_version'")
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

    pub async fn upsert(db: &DbPool, db_version: Option<Version>) -> Result<(), ErrorResponse> {
        let app_version = Self::app_version();
        if Some(&app_version) != db_version.as_ref() {
            let slf = Self {
                version: app_version,
            };
            let data = bincode::serialize(&slf)?;

            #[cfg(not(feature = "postgres"))]
            let q = query!(
                "INSERT OR REPLACE INTO config (id, data) VALUES ('db_version', $1)",
                data,
            );
            #[cfg(feature = "postgres")]
            let q = query!(
                r#"INSERT INTO config (id, data)
                VALUES ('db_version', $1)
                ON CONFLICT(id) DO UPDATE SET data = $1"#,
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
        let db_exists = query!("SELECT id FROM config LIMIT 1")
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
        // this check panics on purpose, and it is there to never forget to adjust this
        // version check before doing any major or minor release
        if app_version.major != 0 || app_version.minor != 26 {
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
        #[cfg(feature = "postgres")]
        let is_db_v0_15_0 = query!("SELECT * FROM pg_tables WHERE tablename = 'passkeys' LIMIT 1")
            .fetch_one(db)
            .await
            .is_err();
        #[cfg(not(feature = "postgres"))]
        let is_db_v0_15_0 = query!(
            "SELECT * FROM sqlite_master WHERE type = 'table' AND name = 'passkeys' LIMIT 1"
        )
        .fetch_one(db)
        .await
        .is_err();
        if is_db_v0_15_0 {
            panic!(
                "Your database is Rauthy v0.15.0. You need to upgrade to Rauthy v0.16 first.\n\
                Please check https://github.com/sebadob/rauthy/releases for additional information."
            );
        }

        // To check for any DB older than 0.15.0, we check for the existence of the 'clients' table
        // which is there since the very beginning.
        #[cfg(feature = "postgres")]
        let is_db_pre_v0_15_0 =
            query!("SELECT * FROM pg_tables WHERE tablename = 'clients' LIMIT 1")
                .fetch_one(db)
                .await
                .is_err();
        #[cfg(not(feature = "postgres"))]
        let is_db_pre_v0_15_0 =
            query!("SELECT * FROM sqlite_master WHERE type = 'table' AND name = 'clients' LIMIT 1")
                .fetch_one(db)
                .await
                .is_err();
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
