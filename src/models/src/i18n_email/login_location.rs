use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct I18nEmailLoginLocation<'a> {
    pub subject: &'a str,
    pub unknown_location: &'a str,
    pub if_invalid: &'a str,
    pub revoke_link: &'a str,
    pub account_link: &'a str,
}

impl I18nEmailLoginLocation<'_> {
    pub fn build(lang: &Language) -> Self {
        match lang {
            Language::En => Self::build_en(),
            Language::De => Self::build_de(),
            Language::ZhHans => Self::build_zh_hans(),
            Language::Ko => Self::build_ko(),
        }
    }
}

impl I18nEmailLoginLocation<'_> {
    fn build_en() -> Self {
        Self {
            subject: "Security Warning",
            unknown_location: "Login from unknown location",
            if_invalid: "If this is an invalid login, you should revoke it immediately and update \
                your credentials!",
            revoke_link: "Revoke Access",
            account_link: "Account Dashboard",
        }
    }

    fn build_de() -> Self {
        Self {
            subject: "Sicherheitswarnung",
            unknown_location: "Login von unbekannter Adresse",
            if_invalid: "Sollte es sicher hierbei um einen ungültigen Login handeln, sollte dieser\
                sofort widerrufen werden und Login-Daten erneuert werden!",
            revoke_link: "Zugriff Entziehen",
            account_link: "Account Dashboard",
        }
    }

    fn build_zh_hans() -> Self {
        Self {
            subject: "Security Warning",
            unknown_location: "Login from unknown location",
            if_invalid: "If this is an invalid login, you should revoke it immediately and update \
                your credentials!",
            revoke_link: "Revoke Access",
            account_link: "Account Dashboard",
        }
    }

    fn build_ko() -> Self {
        Self {
            subject: "Security Warning",
            unknown_location: "Login from unknown location",
            if_invalid: "If this is an invalid login, you should revoke it immediately and update \
                your credentials!",
            revoke_link: "Revoke Access",
            account_link: "Account Dashboard",
        }
    }
}
