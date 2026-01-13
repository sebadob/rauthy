use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct I18nEmailConfirmChange<'a> {
    pub subject: &'a str,
    pub msg: &'a str,
    pub msg_from_admin: &'a str,
}

impl I18nEmailConfirmChange<'_> {
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

impl I18nEmailConfirmChange<'_> {
    fn build_de() -> Self {
        Self {
            subject: "E-Mail Wechsel bestätigt für",
            msg: "Ihre E-Mail Adresse wurde erfolgreich geändert zu:",
            msg_from_admin: "Diese Änderung wurde durch einen Administrator durchgeführt.",
        }
    }

    fn build_en() -> Self {
        Self {
            subject: "E-Mail Change confirmed for",
            msg: "Your E-Mail address has been changed successfully to:",
            msg_from_admin: "This action was done by an Administrator.",
        }
    }

    fn build_nb() -> Self {
        Self {
            subject: "Epostbytte bekreftet for",
            msg: "Din epostadresse har blitt endret til:",
            msg_from_admin: "Denne endringen ble gjort av en administrator.",
        }
    }

    fn build_ko() -> Self {
        Self {
            subject: "이메일 변경이 승인되었습니다:",
            msg: "이메일 주소가 다음 주소로 성공적으로 변경되었습니다:",
            msg_from_admin: "이 작업은 관리자가 수행했습니다.",
        }
    }

    fn build_uk() -> Self {
        Self {
            subject: "Зміну E-mail підтверджено для",
            msg: "Вашу адресу E-mail було успішно змінено на:",
            msg_from_admin: "Цю дію було виконано адміністратором.",
        }
    }

    fn build_zh_hans() -> Self {
        Self {
            subject: "电子邮件地址已更新：",
            msg: "您的电子邮件地址已成功更新为：",
            msg_from_admin: "此操作由管理员完成。",
        }
    }
}
