use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct I18nEmailChangeInfoNew<'a> {
    pub subject: &'a str,
    pub header: &'a str,
    pub click_link: &'a str,
    pub validity: &'a str,
    pub expires: &'a str,
    pub button_text: &'a str,
}

impl I18nEmailChangeInfoNew<'_> {
    pub fn build(lang: &Language) -> Self {
        match lang {
            Language::En => Self::build_en(),
            Language::De => Self::build_de(),
            Language::ZhHans => Self::build_zh_hans(),
            Language::Ko => Self::build_ko(),
        }
    }
}

impl I18nEmailChangeInfoNew<'_> {
    fn build_en() -> Self {
        Self {
            subject: "E-Mail Change Request",
            header: "E-Mail change request for",
            click_link: "Click the link below to confirm your E-Mail address.",
            validity: "This link is only valid for a short period of time for security reasons.",
            expires: "Link expires:",
            button_text: "Confirm E-Mail",
        }
    }

    fn build_de() -> Self {
        Self {
            subject: "E-Mail Wechsel Anfrage",
            header: "E-Mail Wechsel angefordert für",
            click_link: "Klicken Sie auf den unten stehenden Link die E-Mail Adresse zu bestätigen.",
            validity: "Dieser Link ist aus Sicherheitsgründen nur für kurze Zeit gültig.",
            expires: "Link gültig bis:",
            button_text: "E-Mail Bestätigen",
        }
    }

    fn build_zh_hans() -> Self {
        Self {
            subject: "电子邮件更改请求",
            header: "电子邮件更改请求：",
            click_link: "点击下方链接以确认您的电子邮件地址。",
            validity: "出于安全考虑，此链接仅在短时间内有效。",
            expires: "链接过期时间：",
            button_text: "确认电子邮件地址",
        }
    }

    fn build_ko() -> Self {
        Self {
            subject: "이메일 변경 요청",
            header: "이메일 변경 요청:",
            click_link: "이메일 주소를 승인하려면 아래에 있는 링크를 클릭해 주세요.",
            validity: "이 링크는 보안상의 이유로 짧은 시간 동안에만 유효합니다.",
            expires: "링크 만료일:",
            button_text: "이메일 승인",
        }
    }
}
