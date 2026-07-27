// Copyright 2026 Sebastian Dobe <sebastiandobe@mailbox.org>

use std::sync::OnceLock;

pub mod compression;
pub mod constants;
pub mod logging;
pub mod markdown;
pub mod password_hasher;
pub mod regex;
pub mod sanitize_html;
pub mod utils;

pub static DB_TYPE: OnceLock<DbType> = OnceLock::new();
pub static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

#[derive(Debug, PartialEq, Eq)]
pub enum DbType {
    Postgres,
    Hiqlite,
}

#[inline(always)]
pub fn is_hiqlite() -> bool {
    DB_TYPE.get().unwrap() == &DbType::Hiqlite
}

#[inline(always)]
pub fn is_postgres() -> bool {
    DB_TYPE.get().unwrap() == &DbType::Postgres
}

#[inline]
pub fn http_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get().unwrap()
}
