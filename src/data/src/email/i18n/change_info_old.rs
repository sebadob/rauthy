use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct I18nEmailChangeInfoOld<'a> {
    pub subject: &'a str,
    pub header: &'a str,
    pub change_info: &'a str,
    pub click_link: &'a str,
    pub validity: &'a str,
    pub expires: &'a str,
    pub button_text: &'a str,
}

impl I18nEmailChangeInfoOld<'_> {
    pub fn build(lang: &Language) -> Self {
        match lang {
            Language::De => Self::build_de(),
            Language::En => Self::build_en(),
            Language::Ko => Self::build_ko(),
            Language::Nb => Self::build_en(),
            Language::Uk => Self::build_uk(),
            Language::ZhHans => Self::build_zh_hans(),
        }
    }
}

impl I18nEmailChangeInfoOld<'_> {
    fn build_en() -> Self {
        Self {
            subject: "E-Mail Change Request",
            header: "E-Mail change request for",
            change_info: "A change of your E-Mail address has been requested. New address:",
            click_link: "If you did not request this change, please click the Link below",
            validity: "This link is only valid for a short period of time for security reasons.",
            expires: "Link expires:",
            button_text: "Block Address Change",
        }
    }

    fn build_de() -> Self {
        Self {
            subject: "E-Mail Wechsel Bestätigung",
            header: "E-Mail Wechsel angefordert für",
            change_info: "Der Wechsel Ihrer E-Mail Adresse wurde angefordert. Neue Adresse:",
            click_link: "Sollten Sie diesen Wechsel nicht angefordert haben, klicken Sie bitte unten stehenden Link",
            validity: "Dieser Link ist aus Sicherheitsgründen nur für kurze Zeit gültig.",
            expires: "Link gültig bis:",
            button_text: "Adresswechsel Blockieren",
        }
    }

    fn build_uk() -> Self {
        Self {
            subject: "Запит на зміну адреси E-mail",
            header: "Запит на зміну E-mail для",
            change_info: "Було надіслано запит на зміну вашої адреси E-mail. Нова адреса:",
            click_link: "Якщо ви не надсилали цей запит, будь ласка, натисніть посилання нижче",
            validity: "З міркувань безпеки це посилання дійсне лише протягом короткого часу.",
            expires: "Посилання дійсне до:",
            button_text: "Заблокувати зміну адреси",
        }
    }

    fn build_zh_hans() -> Self {
        Self {
            subject: "电子邮件更改请求",
            header: "电子邮件更改请求：",
            change_info: "有人请求更改您的账户的电子邮件地址。新邮箱为：",
            click_link: "如果您没有请求此更改，请点击下方的链接阻止此更改。",
            validity: "出于安全考虑，此链接仅在短时间内有效。",
            expires: "链接过期时间：",
            button_text: "阻止地址更改",
        }
    }

    fn build_ko() -> Self {
        Self {
            subject: "이메일 변경 요청",
            header: "이메일 변경 요청:",
            change_info: "이메일 주소 변경 요청이 있습니다. 새로운 주소:",
            click_link: "만약 이 변경을 요청한 적 없으면, 아래의 링크를 클릭해 주세요.",
            validity: "이 링크는 보안상의 이유로 짧은 시간 동안에만 유효합니다.",
            expires: "링크 만료일:",
            button_text: "주소 변경 차단",
        }
    }
}
