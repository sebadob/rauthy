// Copyright 2024 Sebastian Dobe <sebastiandobe@mailbox.org>
#![forbid(unsafe_code)]
// needed because the lazy_static! initialization of constants grew quite a bit
#![recursion_limit = "256"]

use crate::constants::DB_TYPE;
use std::env;
use std::str::FromStr;

pub mod constants;
pub mod password_hasher;
pub mod utils;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DbType {
    Sqlite,
    Postgres,
    Hiqlite,
}

impl FromStr for DbType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let use_hiqlite = env::var("HIQLITE")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .expect("Cannot parse HIQLITE as bool");

        let res = if use_hiqlite {
            Self::Hiqlite
        } else if s.starts_with("sqlite:") {
            Self::Sqlite
        } else if s.starts_with("postgresql://") {
            Self::Postgres
        } else {
            panic!("You provided an unknown database type, please check the DATABASE_URL");
        };
        Ok(res)
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
