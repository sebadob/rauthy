use actix_web::http::header::{HeaderValue, ACCEPT_LANGUAGE};
use actix_web::HttpRequest;
use rauthy_common::constants::COOKIE_LOCALE;
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Language {
    En,
    De,
}

impl Language {
    fn all_available<'a>() -> [&'a str; 4] {
        ["en", "en-US", "de", "de-DE"]
    }

    pub fn as_str(&self) -> &str {
        match self {
            Language::En => "en",
            Language::De => "de",
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Self::En
    }
}

impl ToString for Language {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl From<&HeaderValue> for Language {
    fn from(value: &HeaderValue) -> Self {
        Language::from(value.to_str().unwrap_or_default())
    }
}

impl From<&str> for Language {
    fn from(value: &str) -> Self {
        match value {
            "de" | "de-DE" => Self::De,
            "en" | "en-US" => Self::En,
            _ => Self::default(),
        }
    }
}

impl TryFrom<&HttpRequest> for Language {
    type Error = ErrorResponse;

    fn try_from(value: &HttpRequest) -> Result<Self, Self::Error> {
        if let Some(cookie) = value.cookie(COOKIE_LOCALE) {
            debug!("locale cookie {:?}", cookie);
            return Ok(Language::from(cookie.value()));
        }

        if let Some(accept_lang) = value.headers().get(ACCEPT_LANGUAGE) {
            let accept_as_str = accept_lang.to_str().unwrap_or_default();
            let common_languages =
                accept_language::intersection(accept_as_str, &Language::all_available());
            debug!("common_languages: {:?}", common_languages);
            if !common_languages.is_empty() {
                return Ok(Language::from(common_languages.get(0).unwrap().as_str()));
            }
        }

        Err(ErrorResponse::new(
            ErrorResponseType::NotFound,
            "Could not find and extract  a locale cookie or the accept-language header".to_string(),
        ))
    }
}
