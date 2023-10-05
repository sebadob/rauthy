use crate::i18n::SsrJson;
use crate::language::Language;
use actix_web::http::StatusCode;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nError<'a> {
    error: String,
    error_text: String,
    details: &'a str,
    details_text: Option<String>,
}

impl I18nError<'_> {
    pub fn build_with<'a>(lang: &Language, status_code: u16, details_text: Option<String>) -> Self {
        match lang {
            Language::En => Self::build_en(status_code, details_text),
            Language::De => Self::build_de(status_code, details_text),
        }
    }
}

impl SsrJson for I18nError<'_> {
    fn build(lang: &Language) -> Self {
        match lang {
            Language::En => Self::build_en(404, None),
            Language::De => Self::build_de(404, None),
        }
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl I18nError<'_> {
    fn build_en(status_code: u16, details_text: Option<String>) -> Self {
        let error = StatusCode::from_u16(status_code)
            .unwrap_or_default()
            .to_string();
        let error_text = match status_code {
            400 => "Your request is malformed or incorrect, please see details.".to_string(),
            401 | 403 => {
                "Access denied - you are not allowed to use or access this resource.".to_string()
            }
            500 => "Internal Server Error".to_string(),
            _ => "The requested data could not be found".to_string(),
        };

        Self {
            error,
            error_text,
            details: "Show Details",
            details_text,
        }
    }

    fn build_de(status_code: u16, details_text: Option<String>) -> Self {
        let error = StatusCode::from_u16(status_code)
            .unwrap_or_default()
            .to_string();
        let error_text = match status_code {
            400 => "Der Request ist falsch formuliert. Weitere Informationen in den Details.".to_string(),
            401 | 403 => "Zugriff verweigert - das Nutzen oder der Zugriff auf diese Ressource ist nicht erlaubt.".to_string(),
            500 => "Interner Server Fehler".to_string(),
           _ =>  "Die angeforderte Seite konnte nicht gefunden werden.".to_string(),
        };

        Self {
            error,
            error_text,
            details: "Details Anzeigen",
            details_text,
        }
    }
}
