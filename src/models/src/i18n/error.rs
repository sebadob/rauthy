use crate::i18n::SsrJson;
use crate::language::Language;
use actix_web::http::StatusCode;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nError {
    error: String,
    error_text: &'static str,
    details: &'static str,
    details_text: Option<Cow<'static, str>>,
}

impl I18nError {
    pub fn build_with<C>(lang: &Language, status_code: StatusCode, details_text: Option<C>) -> Self
    where
        C: Into<Cow<'static, str>>,
    {
        match lang {
            Language::En => Self::build_en(status_code, details_text.map(|t| t.into())),
            Language::De => Self::build_de(status_code, details_text.map(|t| t.into())),
            Language::ZhHans => Self::build_zh_hans(status_code, details_text.map(|t| t.into())),
            Language::Ko => Self::build_ko(status_code, details_text.map(|t| t.into())),
        }
    }
}

impl SsrJson for I18nError {
    fn build(lang: &Language) -> Self {
        match lang {
            Language::En => Self::build_en(StatusCode::NOT_FOUND, None),
            Language::De => Self::build_de(StatusCode::NOT_FOUND, None),
            Language::ZhHans => Self::build_zh_hans(StatusCode::NOT_FOUND, None),
            Language::Ko => Self::build_ko(StatusCode::NOT_FOUND, None),
        }
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl I18nError {
    fn build_en(status_code: StatusCode, details_text: Option<Cow<'static, str>>) -> Self {
        let error_text = match status_code {
            StatusCode::BAD_REQUEST => {
                "Your request is malformed or incorrect, please see details."
            }
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                "Access denied - you are not allowed to use or access this resource."
            }
            StatusCode::INTERNAL_SERVER_ERROR => "Internal Server Error",
            _ => "The requested data could not be found",
        };

        Self {
            error: status_code.to_string(),
            error_text,
            details: "Show Details",
            details_text,
        }
    }

    fn build_de(status_code: StatusCode, details_text: Option<Cow<'static, str>>) -> Self {
        let error_text = match status_code {
            StatusCode::BAD_REQUEST => "Der Request ist falsch formuliert. Weitere Informationen in den Details.",
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => "Zugriff verweigert - das Nutzen oder der Zugriff auf diese Ressource ist nicht erlaubt.",
            StatusCode::INTERNAL_SERVER_ERROR => "Interner Server Fehler",
           _ =>  "Die angeforderte Seite konnte nicht gefunden werden.",
        };

        Self {
            error: status_code.to_string(),
            error_text,
            details: "Details Anzeigen",
            details_text,
        }
    }

    fn build_zh_hans(status_code: StatusCode, details_text: Option<Cow<'static, str>>) -> Self {
        let error_text = match status_code {
            StatusCode::BAD_REQUEST => "您的请求异常，请参见详情。",
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => "拒绝访问——您未被允许访问此资源。",
            StatusCode::INTERNAL_SERVER_ERROR => "内部服务器错误",
            _ => "找不到请求的资源",
        };

        Self {
            error: status_code.to_string(),
            error_text,
            details: "显示详情",
            details_text,
        }
    }

    fn build_ko(status_code: StatusCode, details_text: Option<Cow<'static, str>>) -> Self {
        let error_text = match status_code {
            StatusCode::BAD_REQUEST => {
                "요청이 손상되었거나 잘못되었습니다. 자세한 정보를 참고해 주세요."
            }
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                "접근 거부 - 이 리소스를 사용하거나 접근할 수 없습니다."
            }
            StatusCode::INTERNAL_SERVER_ERROR => "내부적인 서버 오류",
            _ => "요청 정보를 찾을 수 없습니다.",
        };

        Self {
            error: status_code.to_string(),
            error_text,
            details: "자세한 정보 표시",
            details_text,
        }
    }
}
