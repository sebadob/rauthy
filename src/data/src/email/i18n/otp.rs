use crate::language::Language;
use crate::rauthy_config::{RauthyConfig, VarsTemplate};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct I18nEmailOtp<'a> {
    pub subject: &'a str,
    pub header: &'a str,
    pub text: Option<&'a str>,
    pub validity: Option<&'a str>,
    pub expires: &'a str,
}

impl From<&'static VarsTemplate> for I18nEmailOtp<'_> {
    fn from(tpl: &'static VarsTemplate) -> Self {
        Self {
            subject: tpl.subject.as_ref(),
            header: tpl.header.as_ref(),
            text: tpl.text.as_deref(),
            validity: tpl.validity.as_deref(),
            expires: tpl
                .expires
                .as_ref()
                .expect("Missing `expires` in `otp` template"),
        }
    }
}

impl I18nEmailOtp<'_> {
    pub fn build(lang: &Language) -> Self {
        let tpls = &RauthyConfig::get().vars.templates.email_otp;
        match lang {
            Language::De => (&tpls.de).into(),
            Language::En => (&tpls.en).into(),
            Language::Fr => (&tpls.fr).into(),
            Language::Ko => (&tpls.ko).into(),
            Language::Nb => (&tpls.nb).into(),
            Language::Ru => (&tpls.ru).into(),
            Language::Uk => (&tpls.uk).into(),
            Language::ZhHans => (&tpls.zhhans).into(),
        }
    }
}
