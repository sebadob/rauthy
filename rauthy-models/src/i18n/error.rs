use crate::i18n::SsrJson;
use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nError<'a> {
    error: String,
    error_text: String,
    details: &'a str,
    details_text: String,
}

impl I18nError<'_> {
    pub fn build_with<'a>(
        lang: &Language,
        error: String,
        error_text: String,
        details_text: String,
    ) -> Self {
        match lang {
            Language::En => Self::build_en(error, error_text, details_text),
            Language::De => Self::build_de(error, error_text, details_text),
        }
    }
}

impl SsrJson for I18nError<'_> {
    fn build(lang: &Language) -> Self {
        match lang {
            Language::En => Self::build_en(String::default(), String::default(), String::default()),
            Language::De => Self::build_de(String::default(), String::default(), String::default()),
        }
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl I18nError<'_> {
    fn build_en(error: String, error_text: String, details_text: String) -> Self {
        Self {
            error,
            error_text,
            details: "Show Details",
            details_text,
        }
    }

    fn build_de(error: String, error_text: String, details_text: String) -> Self {
        Self {
            error,
            error_text,
            details: "Details Anzeigen",
            details_text,
        }
    }
}
