// Copyright 2026 Sebastian Dobe <sebastiandobe@mailbox.org>

use crate::admin::I18nAdmin;
use crate::common::I18nCommon;
use rauthy_common::language::Language;

pub mod admin;
pub mod common;
pub mod languages;

pub fn build_all() {
    let mut admin = Vec::with_capacity(16);
    let mut common = Vec::with_capacity(16);

    for lang in Language::as_iter() {
        admin.push((lang, I18nAdmin::from(&lang)));
        common.push((lang, I18nCommon::from(&lang)));
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i18n_ui_build_all() {
        build_all();
    }
}
