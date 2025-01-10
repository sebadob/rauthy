use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nPasswordPolicy {
    password_policy: &'static str,
    length_min: &'static str,
    length_max: &'static str,
    lowercase_min: &'static str,
    uppercase_min: &'static str,
    digits_min: &'static str,
    special_min: &'static str,
    not_recent: &'static str,
}

impl I18nPasswordPolicy {
    pub fn build(lang: &Language) -> Self {
        match lang {
            Language::En => Self::build_en(),
            Language::De => Self::build_de(),
            Language::ZhHans => Self::build_zh_hans(),
            Language::Ko => Self::build_ko(),
        }
    }

    pub fn build_en() -> Self {
        Self {
            password_policy: "Password Policy",
            length_min: "Length min",
            length_max: "Length max",
            lowercase_min: "Lowercase letters min",
            uppercase_min: "Uppercase letters min",
            digits_min: "Digits min",
            special_min: "Special characters min",
            not_recent: "Not one of last recent passwords",
        }
    }

    pub fn build_de() -> Self {
        Self {
            password_policy: "Passwort Regeln",
            length_min: "Länge min",
            length_max: "Länge max",
            lowercase_min: "Kleinbuchstaben min",
            uppercase_min: "Großbuchstaben min",
            digits_min: "Ziffern min",
            special_min: "Spezielle Buchstaben min",
            not_recent: "Keins der letzten Passwörter",
        }
    }

    pub(crate) fn build_zh_hans() -> Self {
        Self {
            password_policy: "密码要求",
            length_min: "最小长度",
            length_max: "最长长度",
            lowercase_min: "最少小写字母",
            uppercase_min: "最少大写字母",
            digits_min: "最少数字",
            special_min: "最少特殊字符",
            not_recent: "不是最近使用过的密码之一",
        }
    }

    pub fn build_ko() -> Self {
        Self {
            password_policy: "비밀번호 정책",
            length_min: "최소 자수",
            length_max: "최대 자수",
            lowercase_min: "최소 소문자 자수",
            uppercase_min: "최소 대문자 자수",
            digits_min: "최소 숫자 자수",
            special_min: "최소 특수문자 자수",
            not_recent: "최근 사용한 비밀번호 중 하나가 아닌 것",
        }
    }
}
