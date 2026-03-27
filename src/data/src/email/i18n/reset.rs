use crate::language::Language;
use crate::rauthy_config::{RauthyConfig, VarsTemplate};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct I18nEmailReset<'a> {
    pub subject: &'a str,
    pub header: &'a str,
    pub text: Option<&'a str>,
    pub click_link: &'a str,
    pub validity: &'a str,
    pub expires: &'a str,
    pub button_text: &'a str,
    pub footer: Option<&'a str>,
    pub button_text_request_new: Option<&'a str>,
}

impl From<&'static VarsTemplate> for I18nEmailReset<'_> {
    fn from(tpl: &'static VarsTemplate) -> Self {
        Self {
            subject: tpl.subject.as_ref(),
            header: tpl.header.as_ref(),
            text: tpl.text.as_deref(),
            click_link: tpl
                .click_link
                .as_ref()
                .expect("Missing `click_link` in `password_reset` template"),
            validity: tpl
                .validity
                .as_ref()
                .expect("Missing `validity` in `password_reset` template"),
            expires: tpl
                .expires
                .as_ref()
                .expect("Missing `expires` in `password_reset` template"),
            button_text: tpl
                .button
                .as_ref()
                .expect("Missing `button` in `password_reset` template"),
            footer: tpl.footer.as_deref(),
            button_text_request_new: tpl.button_text_request_new.as_deref(),
        }
    }
}

impl I18nEmailReset<'_> {
    pub fn build(lang: &Language) -> Self {
        let tpls = &RauthyConfig::get().vars.templates.password_reset;
        match lang {
            Language::De => (&tpls.de).into(),
            Language::En => (&tpls.en).into(),
            Language::Ko => (&tpls.ko).into(),
            Language::Nb => (&tpls.nb).into(),
            Language::Uk => (&tpls.uk).into(),
            Language::ZhHans => (&tpls.zhhans).into(),
        }
    }
}
