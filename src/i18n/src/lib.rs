// Copyright 2026 Sebastian Dobe <sebastiandobe@mailbox.org>

use ahash::{HashMap, HashMapExt};
use rauthy_common::language::Language;
use std::str::FromStr;
use std::sync::LazyLock;

pub use crate::admin::I18nAdmin;
pub use crate::common::I18nCommon;

pub mod admin;
pub mod common;
pub mod languages;

pub static I18N_ADMIN_HASHES: LazyLock<HashMap<Language, &'static str>> =
    LazyLock::new(|| parse_compile_time_hashes(include_str!("../i18n_hashes/admin")));

pub static I18N_COMMON_HASHES: LazyLock<HashMap<Language, &'static str>> =
    LazyLock::new(|| parse_compile_time_hashes(include_str!("../i18n_hashes/common")));

pub fn build_all_admin() -> Vec<(Language, I18nAdmin<'static>)> {
    let mut admin = Vec::with_capacity(16);
    for lang in Language::as_iter() {
        admin.push((lang, I18nAdmin::from(&lang)));
    }
    admin
}

pub fn build_all_common() -> Vec<(Language, I18nCommon<'static>)> {
    let mut common = Vec::with_capacity(16);
    for lang in Language::as_iter() {
        common.push((lang, I18nCommon::from(&lang)));
    }
    common
}

impl From<&Language> for I18nCommon<'_> {
    fn from(lang: &Language) -> Self {
        match lang {
            Language::De => languages::de::common::build(),
            Language::En => languages::en::common::build(),
            Language::Es => languages::es::common::build(),
            Language::Fr => languages::fr::common::build(),
            Language::Ja => languages::ja::common::build(),
            Language::Ko => languages::ko::common::build(),
            Language::Nb => languages::nb::common::build(),
            Language::Nl => languages::nl::common::build(),
            Language::Pt => languages::pt::common::build(),
            Language::Ru => languages::ru::common::build(),
            Language::Uk => languages::uk::common::build(),
            Language::ZhHans => languages::zh::common::build(),
        }
    }
}

impl From<&Language> for I18nAdmin<'_> {
    fn from(lang: &Language) -> Self {
        match lang {
            Language::De => languages::de::admin::build(),
            Language::En => languages::en::admin::build(),
            Language::Es => languages::es::admin::build(),
            Language::Fr => languages::fr::admin::build(),
            Language::Ja => languages::ja::admin::build(),
            Language::Ko => languages::ko::admin::build(),
            Language::Nb => languages::nb::admin::build(),
            Language::Nl => languages::nl::admin::build(),
            Language::Pt => languages::pt::admin::build(),
            Language::Ru => languages::ru::admin::build(),
            Language::Uk => languages::uk::admin::build(),
            Language::ZhHans => languages::zh::admin::build(),
        }
    }
}

fn parse_compile_time_hashes(input: &'static str) -> HashMap<Language, &'static str> {
    let mut map = HashMap::with_capacity(16);
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let (key, _) = line.split_once('_').unwrap();
        let lang = Language::from_str(key).unwrap();
        map.insert(lang, line);
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i18n_ui_build_all() {
        build_all_admin();
        build_all_common();

        I18N_ADMIN_HASHES.get(&Language::De).unwrap();
        I18N_COMMON_HASHES.get(&Language::De).unwrap();
    }
}
