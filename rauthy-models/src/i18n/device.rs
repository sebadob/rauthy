use crate::i18n::SsrJson;
use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nDevice<'a> {
    title: &'a str,
}

impl SsrJson for I18nDevice<'_> {
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

impl I18nDevice<'_> {
    fn build_en() -> Self {
        Self {
            title: "Device Authorization",
        }
    }

    fn build_de() -> Self {
        Self {
            title: "Gerät Authorisierung",
        }
    }
}
