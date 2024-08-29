use crate::i18n::SsrJson;
use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nLogout<'a> {
    logout: &'a str,
    confirm_msg: &'a str,
    cancel: &'a str,
}

impl SsrJson for I18nLogout<'_> {
    fn build(lang: &Language) -> Self {
        match lang {
            Language::En => Self::build_en(),
            Language::De => Self::build_de(),
            Language::ZhHans => Self::build_zh_hans(),
        }
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl I18nLogout<'_> {
    fn build_en() -> Self {
        Self {
            logout: "Logout",
            confirm_msg: "Do you really want to logout and end your session?",
            cancel: "Cancel",
        }
    }

    fn build_de() -> Self {
        Self {
            logout: "Logout",
            confirm_msg:
                "Sind Sie sicher, dass Sie sich ausloggen und die Session beenden möchten?",
            cancel: "Abbrechen",
        }
    }

    fn build_zh_hans() -> Self {
        Self {
            logout: "退出登录",
            confirm_msg: "您确定要退出登录并结束会话吗？",
            cancel: "取消",
        }
    }
}
