// Copyright 2025 Sebastian Dobe <sebastiandobe@mailbox.org>

// needed because the lazy_static! initialization of constants grew quite a bit
#![recursion_limit = "512"]

use crate::constants::{DB_TYPE, DEV_MODE, RAUTHY_VERSION};
use reqwest::tls;
use std::env;
use std::sync::LazyLock;
use std::time::Duration;

pub mod compression;
pub mod constants;
pub mod password_hasher;
pub mod utils;

/// TODO make sure that in (almost) all places, this single client is being used for all outgoing
/// requests to reduce duplicate Client builds and the amount of duplicate TLS handshakes.
pub static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(10))
        .user_agent(format!("Rauthy v{}", RAUTHY_VERSION))
        .min_tls_version(tls::Version::TLS_1_2)
        .pool_idle_timeout(Duration::from_secs(600))
        // TODO make these 2 globally configurable
        .https_only(!*DEV_MODE)
        .danger_accept_invalid_certs(*DEV_MODE)
        .build()
        .unwrap()
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DbType {
    Sqlite,
    Postgres,
    Hiqlite,
}

impl DbType {
    fn from_str(db_url: Option<&str>) -> Self {
        let use_hiqlite = env::var("HIQLITE")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .expect("Cannot parse HIQLITE as bool");

        if use_hiqlite {
            return DbType::Hiqlite;
        }

        if let Some(db_url) = db_url {
            if db_url.starts_with("sqlite:") {
                panic!(
                    "SQLite support has been dropped with v0.27.0 - please migrate to Hiqlite:\n\
                https://github.com/sebadob/rauthy/blob/main/CHANGELOG.md#dropped-sqlx-sqlite-in-favor-of-hiqlite"
                )
            } else if db_url.starts_with("postgresql://") {
                Self::Postgres
            } else {
                panic!("You provided an unknown database type, please check the DATABASE_URL");
            }
        } else {
            panic!("HIQLITE is disabled and no DATABASE_URL given");
        }
    }
}

#[inline(always)]
pub fn is_hiqlite() -> bool {
    *DB_TYPE == DbType::Hiqlite
}

#[inline(always)]
pub fn is_sqlite() -> bool {
    *DB_TYPE == DbType::Sqlite
}

#[inline(always)]
pub fn is_postgres() -> bool {
    *DB_TYPE == DbType::Postgres
}
