use actix_web::HttpRequest;
use actix_web::http::header::{ACCEPT_LANGUAGE, HeaderValue};
use rauthy_common::constants::COOKIE_LOCALE;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use tracing::debug;

// Note: Updating this enum will require an update on the LANGUAGES constant for the frontend too
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize, strum::EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    De,
    #[default]
    En,
    Es,
    Fr,
    Ja,
    Ko,
    Nb,
    Nl,
    Pt,
    Ru,
    Uk,
    ZhHans,
}

impl Language {
    pub fn all_available() -> [&'static str; 25] {
        [
            "en", "en-US", "de", "de-DE", "es", "es-ES", "fr", "fr-FR", "ja", "ja-JP", "ko",
            "ko-KR", "nb", "nb-NO", "no-NO", "nl", "nl-NL", "pt", "pt-PT", "ru", "ru-RU", "uk",
            "uk-UA", "zh", "zh-Hans",
        ]
    }

    pub fn as_str(&self) -> &'static str {
        // must return results that work with serde::Deserialize from lowercase
        match self {
            Language::De => "de",
            Language::En => "en",
            Language::Es => "es",
            Language::Fr => "fr",
            Language::Ja => "ja",
            Language::Ko => "ko",
            Language::Nb => "nb",
            Language::Nl => "nl",
            Language::Pt => "pt",
            Language::Ru => "ru",
            Language::Uk => "uk",
            Language::ZhHans => "zhhans",
        }
    }

    /// The BCP 47 tag for `<html lang>`, which browsers, screen readers and
    /// spell checkers read. It names the region or script where the bare code
    /// would mean another variant: a bare `pt` is Brazilian Portuguese to
    /// browsers and CLDR, and `zhhans` is not a valid tag. The frontend only
    /// reads the first two characters back, so this changes nothing else.
    pub fn html_lang(&self) -> &'static str {
        match self {
            Language::De => "de-DE",
            Language::En => "en-US",
            Language::Es => "es-ES",
            Language::Fr => "fr-FR",
            Language::Ja => "ja-JP",
            Language::Ko => "ko-KR",
            Language::Nb => "nb-NO",
            Language::Nl => "nl-NL",
            Language::Pt => "pt-PT",
            Language::Ru => "ru-RU",
            Language::Uk => "uk-UA",
            Language::ZhHans => "zh-Hans",
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&HeaderValue> for Language {
    fn from(value: &HeaderValue) -> Self {
        Language::from(value.to_str().unwrap_or_default())
    }
}

impl From<String> for Language {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&str> for Language {
    fn from(value: &str) -> Self {
        match value {
            "en" | "en-US" => Self::En,
            "de" | "de-DE" => Self::De,
            "es" | "es-ES" => Self::Es,
            "fr" | "fr-FR" => Self::Fr,
            "ja" | "ja-JP" => Self::Ja,
            "ko" | "ko-KR" => Self::Ko,
            "nb" | "nb-NO" | "no-NO" => Self::Nb,
            "nl" | "nl-NL" => Self::Nl,
            "pt" | "pt-PT" => Self::Pt,
            "ru" | "ru-RU" => Self::Ru,
            "uk" | "uk-UA" => Self::Uk,
            "zh" | "zhhans" | "zh-hans" | "zh-Hans" => Self::ZhHans,
            _ => Self::default(),
        }
    }
}

impl TryFrom<&HttpRequest> for Language {
    type Error = ErrorResponse;

    fn try_from(value: &HttpRequest) -> Result<Self, Self::Error> {
        // Do not use ApiCookie::from_req here since this cookie is non-sensitive and
        // set via the UI and JS
        if let Some(cookie) = value.cookie(COOKIE_LOCALE) {
            debug!(?cookie);
            return Ok(Language::from(cookie.value()));
        }

        if let Some(accept_lang) = value.headers().get(ACCEPT_LANGUAGE) {
            let accept_as_str = accept_lang.to_str().unwrap_or_default();
            let common_languages =
                accept_language::intersection(accept_as_str, &Language::all_available());
            debug!(?common_languages);
            if !common_languages.is_empty() {
                return Ok(Language::from(common_languages.first().unwrap().as_str()));
            }
        }

        Err(ErrorResponse::new(
            ErrorResponseType::NotFound,
            "Could not find and extract  a locale cookie or the accept-language header",
        ))
    }
}

impl From<rauthy_api_types::generic::Language> for Language {
    fn from(value: rauthy_api_types::generic::Language) -> Self {
        match value {
            rauthy_api_types::generic::Language::De => Self::De,
            rauthy_api_types::generic::Language::En => Self::En,
            rauthy_api_types::generic::Language::Es => Self::Es,
            rauthy_api_types::generic::Language::Fr => Self::Fr,
            rauthy_api_types::generic::Language::Ja => Self::Ja,
            rauthy_api_types::generic::Language::Ko => Self::Ko,
            rauthy_api_types::generic::Language::Nb => Self::Nb,
            rauthy_api_types::generic::Language::Nl => Self::Nl,
            rauthy_api_types::generic::Language::Pt => Self::Pt,
            rauthy_api_types::generic::Language::Ru => Self::Ru,
            rauthy_api_types::generic::Language::Uk => Self::Uk,
            rauthy_api_types::generic::Language::ZhHans => Self::ZhHans,
        }
    }
}

impl From<Language> for rauthy_api_types::generic::Language {
    fn from(value: Language) -> Self {
        match value {
            Language::De => Self::De,
            Language::En => Self::En,
            Language::Es => Self::Es,
            Language::Fr => Self::Fr,
            Language::Ja => Self::Ja,
            Language::Ko => Self::Ko,
            Language::Nb => Self::Nb,
            Language::Nl => Self::Nl,
            Language::Pt => Self::Pt,
            Language::Ru => Self::Ru,
            Language::Uk => Self::Uk,
            Language::ZhHans => Self::ZhHans,
        }
    }
}
