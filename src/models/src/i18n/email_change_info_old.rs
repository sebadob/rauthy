use crate::i18n::SsrJson;
use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct I18nEmailChangeInfoOld<'a> {
    pub subject: &'a str,
    pub header: &'a str,
    pub change_info: &'a str,
    pub click_link: &'a str,
    pub validity: &'a str,
    pub expires: &'a str,
    pub button_text: &'a str,
}

impl SsrJson for I18nEmailChangeInfoOld<'_> {
    fn build(lang: &Language) -> Self {
        match lang {
            Language::En => Self::build_en(),
            Language::De => Self::build_de(),
        }
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl I18nEmailChangeInfoOld<'_> {
    fn build_en() -> Self {
        Self {
            subject: "E-Mail Change Request",
            header: "E-Mail change request for",
            change_info: "A change of your E-Mail address has been requested. New address:",
            click_link: "If you did not request this change, please click the Link below",
            validity: "This link is only valid for a short period of time for security reasons.",
            expires: "Link expires:",
            button_text: "Block Address Change",
        }
    }

    fn build_de() -> Self {
        Self {
            subject: "E-Mail Wechsel Bestätigung",
            header: "E-Mail Wechsel angefordert für",
            change_info: "Der Wechsel Ihrer E-Mail Adresse wurde angefordert. Neue Adresse:",
            click_link: "Sollten Sie diesen Wechsel nicht angefordert haben, klicken Sie bitte unten stehenden Link",
            validity: "Dieser Link ist aus Sicherheitsgründen nur für kurze Zeit gültig.",
            expires: "Link gültig bis:",
            button_text: "Adresswechsel Blockieren",
        }
    }
}
