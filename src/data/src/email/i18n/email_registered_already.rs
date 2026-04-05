use crate::language::Language;
use crate::rauthy_config::{RauthyConfig, VarsTemplate};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct I18nEmailRegisteredAlready<'a> {
    pub subject: &'a str,
    pub header: &'a str,
    pub text: &'a str,
    pub footer: Option<&'a str>,
    pub button_text_request_new: &'a str,
}

impl From<&'static VarsTemplate> for I18nEmailRegisteredAlready<'_> {
    fn from(tpl: &'static VarsTemplate) -> Self {
        Self {
            subject: tpl.subject.as_ref(),
            header: tpl.header.as_ref(),
            text: tpl
                .text
                .as_ref()
                .expect("Missing `text` in `email_registered_already` template"),
            footer: tpl.footer.as_deref(),
            button_text_request_new: tpl
                .button_text_request_new
                .as_ref()
                .expect("Missing `button_text_request_new` in `email_registered_already` template"),
        }
    }
}

impl I18nEmailRegisteredAlready<'_> {
    pub fn build(lang: &Language) -> Self {
        let tpls = &RauthyConfig::get().vars.templates.email_registered_already;
        match lang {
            Language::De => (&tpls.de).into(),
            Language::En => (&tpls.en).into(),
            Language::Ko => (&tpls.ko).into(),
            Language::Nb => (&tpls.nb).into(),
            Language::Ru => (&tpls.ru).into(),
            Language::Uk => (&tpls.uk).into(),
            Language::ZhHans => (&tpls.zhhans).into(),
        }
    }
}
