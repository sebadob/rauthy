use actix_web::HttpRequest;
use actix_web::http::header::{ACCEPT_LANGUAGE, HeaderValue};
use rauthy_common::constants::COOKIE_LOCALE;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use tracing::debug;

// Note: Updating this enum will require an update on the LANGUAGES constant for the frontend too
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, strum::EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    En,
    De,
    ZhHans,
    Ko,
}

impl Language {
    fn all_available<'a>() -> [&'a str; 7] {
        ["en", "en-US", "de", "de-DE", "zh", "zh-Hans", "ko"]
    }

    pub fn as_str(&self) -> &str {
        // must return results that work with serde::Deserialize from lowercase
        match self {
            Language::En => "en",
            Language::De => "de",
            Language::ZhHans => "zhhans",
            Language::Ko => "ko",
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Self::En
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
            "de" | "de-DE" => Self::De,
            "en" | "en-US" => Self::En,
            "zh" | "zhhans" | "zh-hans" | "zh-Hans" => Self::ZhHans,
            "ko" | "ko-KR" => Self::Ko,
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
            rauthy_api_types::generic::Language::En => Self::En,
            rauthy_api_types::generic::Language::De => Self::De,
            rauthy_api_types::generic::Language::ZhHans => Self::ZhHans,
            rauthy_api_types::generic::Language::Ko => Self::Ko,
        }
    }
}

impl From<Language> for rauthy_api_types::generic::Language {
    fn from(value: Language) -> Self {
        match value {
            Language::En => Self::En,
            Language::De => Self::De,
            Language::ZhHans => Self::ZhHans,
            Language::Ko => Self::Ko,
        }
    }
}
