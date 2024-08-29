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
            Language::ZhHans => Self::build_zh_hans(),
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

    fn build_zh_hans() -> Self {
        Self {
            subject: "电子邮件更改请求",
            header: "电子邮件更改请求：",
            change_info: "有人请求更改您的账户的电子邮件地址。新邮箱为：",
            click_link: "如果您没有请求此更改，请点击下方的链接阻止此更改。",
            validity: "出于安全考虑，此链接仅在短时间内有效。",
            expires: "链接过期时间：",
            button_text: "阻止地址更改",
        }
    }
}
