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
            click_link: tpl.click_link.as_ref(),
            validity: tpl.validity.as_ref(),
            expires: tpl.expires.as_ref(),
            button_text: tpl.button.as_ref(),
            footer: tpl.footer.as_deref(),
            button_text_request_new: tpl.button_text_request_new.as_deref(),
        }
    }
}

impl I18nEmailReset<'_> {
    pub fn build(lang: &Language) -> Self {
        let tpls = &RauthyConfig::get().vars.templates;
        match lang {
            Language::De => Self::from(&tpls.password_reset_de),
            Language::En => Self::from(&tpls.password_reset_en),
            Language::Ko => Self::from(&tpls.password_reset_ko),
            Language::Nb => Self::from(&tpls.password_reset_nb),
            Language::Uk => Self::from(&tpls.password_reset_uk),
            Language::ZhHans => Self::from(&tpls.password_reset_zhhans),
        }
    }
}
