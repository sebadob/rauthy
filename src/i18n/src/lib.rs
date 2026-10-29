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
            Language::Es => todo!(),
            Language::Fr => todo!(),
            Language::Ja => todo!(),
            Language::Ko => todo!(),
            Language::Nb => todo!(),
            Language::Nl => todo!(),
            Language::Pt => todo!(),
            Language::Ru => todo!(),
            Language::Uk => todo!(),
            Language::ZhHans => todo!(),
        }
    }
}

impl From<&Language> for I18nAdmin<'_> {
    fn from(lang: &Language) -> Self {
        match lang {
            Language::De => languages::de::admin::build(),
            Language::En => languages::en::admin::build(),
            Language::Es => todo!(),
            Language::Fr => todo!(),
            Language::Ja => todo!(),
            Language::Ko => todo!(),
            Language::Nb => todo!(),
            Language::Nl => todo!(),
            Language::Pt => todo!(),
            Language::Ru => todo!(),
            Language::Uk => todo!(),
            Language::ZhHans => todo!(),
        }
    }
}
