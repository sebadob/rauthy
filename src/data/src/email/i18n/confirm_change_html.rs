use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct I18nEmailConfirmChangeHtml<'a> {
    pub title: &'a str,
    pub text_changed: &'a str,
    pub text_login: &'a str,
    pub to: &'a str,
}

impl I18nEmailConfirmChangeHtml<'_> {
    pub fn build(lang: &Language) -> Self {
        match lang {
            Language::De => Self::build_de(),
            Language::En => Self::build_en(),
            Language::Ko => Self::build_ko(),
            Language::Nb => Self::build_nb(),
            Language::Uk => Self::build_uk(),
            Language::ZhHans => Self::build_zh_hans(),
        }
    }
}

impl I18nEmailConfirmChangeHtml<'_> {
    fn build_de() -> Self {
        Self {
            title: "E-Mail Wechsel bestätigt",
            text_changed: "Ihre E-Mail Adresse wurde erfolgreich geändert von",
            text_login: "Sie können sich jetzt mit der neuen Adresse einloggen.",
            to: "zu",
        }
    }

    fn build_en() -> Self {
        Self {
            title: "E-Mail Change confirmed",
            text_changed: "Your E-Mail address has been changed from",
            text_login: "You can now log in using your new address.",
            to: "to",
        }
    }

    fn build_ko() -> Self {
        Self {
            title: "이메일 변경이 승인되었습니다.",
            text_changed: "이메일 주소가 다음으로부터 변경되었습니다",
            text_login: "이제 새로운 주소로 로그인할 수 있습니다.",
            to: "에서",
        }
    }

    fn build_nb() -> Self {
        Self {
            title: "E-postbytte bekreftet",
            text_changed: "Din epostadresse har blitt endret fra",
            text_login: "Du kan nå logge inn med din nye adresse.",
            to: "til",
        }
    }

    fn build_uk() -> Self {
        Self {
            title: "Зміну E-mail підтверджено",
            text_changed: "Вашу адресу E-mail було змінено з",
            text_login: "Тепер ви можете увійти, використовуючи вашу нову адресу.",
            to: "на",
        }
    }

    fn build_zh_hans() -> Self {
        Self {
            title: "电子邮件地址已更新",
            text_changed: "您的电子邮件地址已从",
            text_login: "您现在可以使用您的新地址进行登陆。",
            to: "更新为",
        }
    }
}
