use crate::i18n::SsrJson;
use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nIndex<'a> {
    register: &'a str,
    account_login: &'a str,
    admin_login: &'a str,
}

impl SsrJson for I18nIndex<'_> {
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

impl I18nIndex<'_> {
    fn build_en() -> Self {
        Self {
            register: "Register",
            account_login: "Account",
            admin_login: "Admin",
        }
    }

    fn build_de() -> Self {
        Self {
            register: "Registrieren",
            account_login: "Account",
            admin_login: "Admin",
        }
    }

    fn build_zh_hans() -> Self {
        Self {
            register: "注册",
            account_login: "登陆",
            admin_login: "管理",
        }
    }
}
