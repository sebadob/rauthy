use crate::i18n::SsrJson;
use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct I18nEmailConfirmChangeHtml<'a> {
    pub title: &'a str,
    pub text_changed: &'a str,
    pub text_login: &'a str,
    pub to: &'a str,
}

impl SsrJson for I18nEmailConfirmChangeHtml<'_> {
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

impl I18nEmailConfirmChangeHtml<'_> {
    fn build_en() -> Self {
        Self {
            title: "E-Mail Change confirmed",
            text_changed: "Your E-Mail address has been changed from",
            text_login: "You can now log in using your new address.",
            to: "to",
        }
    }

    fn build_de() -> Self {
        Self {
            title: "E-Mail Wechsel bestätigt",
            text_changed: "Ihre E-Mail Adresse wurde erfolgreich geändert von",
            text_login: "Sie können sich jetzt mit der neuen Adresse einloggen.",
            to: "zu",
        }
    }

    fn build_zh_hans() -> Self {
        Self {
            title: "电子邮件地址已更新",
            text_changed: "您的电子邮件地址已从",
            text_login: "您现在可以使用您的新地址进行登陆。",
            to: "更新为",
        }
    }
}
