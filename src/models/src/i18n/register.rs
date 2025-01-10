use crate::i18n::SsrJson;
use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nRegister {
    domain_allowed: &'static str,
    domain_err: &'static str,
    domain_restricted: &'static str,
    email: &'static str,
    email_bad_format: &'static str,
    email_check: &'static str,
    family_name: &'static str,
    given_name: &'static str,
    regex_name: &'static str,
    register: &'static str,
    required: &'static str,
    success: &'static str,
    user_reg: &'static str,
}

impl SsrJson for I18nRegister {
    fn build(lang: &Language) -> Self {
        match lang {
            Language::En => Self::build_en(),
            Language::De => Self::build_de(),
            Language::ZhHans => Self::build_zh_hans(),
            Language::Ko => Self::build_ko(),
        }
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl I18nRegister {
    fn build_en() -> Self {
        Self {
            domain_allowed: "Allowed domain:",
            domain_err: "E-Mail domain not allowed",
            domain_restricted: "E-Mail domains are restricted",
            email: "E-Mail",
            email_bad_format: "Bad E-Mail format",
            email_check: "Please check your E-Mail inbox",
            family_name: "Family Name",
            given_name: "Given Name",
            regex_name: "Name should have 2 - 32 non-special characters",
            register: "Register",
            required: "Required",
            success: "Registration successful",
            user_reg: "User Registration",
        }
    }

    fn build_de() -> Self {
        Self {
            domain_allowed: "Erlaubte Domain:",
            domain_err: "E-Mail Domain ist nicht erlaubt",
            domain_restricted: "E-Mail Domains sind beschränkt",
            email: "E-Mail",
            email_bad_format: "Ungültiges E-Mail Format",
            email_check: "Bitte prüfen Sie Ihren E-Mail Posteingang",
            family_name: "Nachname",
            given_name: "Vorname",
            regex_name: "Name mit 2 - 32 Buchstaben ohne Sonderzeichen",
            register: "Registrieren",
            required: "Notwendig",
            success: "Registrierung erfolgreich",
            user_reg: "Benutzer Registrierung",
        }
    }

    fn build_zh_hans() -> Self {
        Self {
            domain_allowed: "允许的域名：",
            domain_err: "此电子邮件域名不被允许",
            domain_restricted: "电子邮件域名被限制",
            email: "电子邮件",
            email_bad_format: "错误的电子邮件地址格式",
            email_check: "请检查您的电子邮件收件箱",
            family_name: "姓氏",
            given_name: "名字",
            regex_name: "名字应有2-32个非特殊字符。",
            register: "注册",
            required: "必填",
            success: "注册成功",
            user_reg: "用户注册",
        }
    }

    fn build_ko() -> Self {
        Self {
            domain_allowed: "허용된 도메인:",
            domain_err: "허용되지 않은 이메일의 도메인입니다.",
            domain_restricted: "이메일의 도메인이 제한되어 있습니다.",
            email: "이메일",
            email_bad_format: "잘못된 이메일 형식입니다.",
            email_check: "이메일 보관함을 확인해 주세요.",
            family_name: "성",
            given_name: "이름",
            regex_name: "이름은 특수문자를 제외한 2자에서 32자이어야 합니다.",
            register: "가입",
            required: "필수 항목입니다.",
            success: "성공적으로 가입되었습니다.",
            user_reg: "사용자 가입",
        }
    }
}
