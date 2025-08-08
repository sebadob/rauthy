use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct I18nEmailResetInfo<'a> {
    pub subject: &'a str,
    pub expires_1: &'a str,
    pub expires_2: &'a str,
    pub update: &'a str,
    pub button_text: &'a str,
}

impl I18nEmailResetInfo<'_> {
    pub fn build(lang: &Language) -> Self {
        match lang {
            Language::En => Self::build_en(),
            Language::De => Self::build_de(),
            Language::ZhHans => Self::build_zh_hans(),
            Language::Ko => Self::build_ko(),
        }
    }
}

impl I18nEmailResetInfo<'_> {
    fn build_en() -> Self {
        Self {
            subject: "Password is about to expire",
            expires_1: "Your password for",
            expires_2: "is about to expire:",
            update: "You can update it here:",
            button_text: "Update Password",
        }
    }

    fn build_de() -> Self {
        Self {
            subject: "Passwort läuft demnächst ab",
            expires_1: "Ihr Passwort für",
            expires_2: " läuft demnächst ab:",
            update: "Sie können es hier erneuern:",
            button_text: "Passwort Erneuern",
        }
    }

    fn build_zh_hans() -> Self {
        Self {
            subject: "您的密码即将过期",
            expires_1: "",
            expires_2: "的密码即将过期：",
            update: "您可以在此处更新密码：",
            button_text: "更新密码",
        }
    }

    fn build_ko() -> Self {
        Self {
            subject: "비밀번호가 곧 만료됩니다.",
            expires_1: "",
            expires_2: "의 비밀번호가 곧 만료됩니다:",
            update: "다음에서 변경할 수 있습니다:",
            button_text: "비밀번호 변경",
        }
    }
}
