// Copyright 2024 Sebastian Dobe <sebastiandobe@mailbox.org>
#![forbid(unsafe_code)]
// needed because the lazy_static! initialization of constants grew quite a bit
#![recursion_limit = "512"]

use crate::constants::DB_TYPE;
use std::env;

mod compression;
pub mod constants;
pub mod password_hasher;
pub mod utils;

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
