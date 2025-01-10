use crate::i18n::SsrJson;
use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nDevice {
    accept: &'static str,
    auto_redirect_account: &'static str,
    close_window: &'static str,
    decline: &'static str,
    desc: &'static str,
    desc_scopes: &'static str,
    err_too_short: &'static str,
    err_too_long: &'static str,
    invalid_input: &'static str,
    is_accepted: &'static str,
    is_declined: &'static str,
    submit: &'static str,
    title: &'static str,
    user_code: &'static str,
    wrong_or_expired: &'static str,
}

impl SsrJson for I18nDevice {
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

impl I18nDevice {
    fn build_en() -> Self {
        Self {
            accept: "Accept",
            auto_redirect_account: "You will be redirected to your account now",
            close_window: "You can close this window now.",
            decline: "Decline",
            desc: "Please enter the {{count}} characters user code from your device.",
            desc_scopes: "The device requests access to:",
            err_too_short: "Input too short",
            err_too_long: "Input too long",
            invalid_input: "Invalid Input",
            is_accepted: "The request has been accepted.",
            is_declined: "The request has been declined.",
            submit: "Submit",
            title: "Device Authorization",
            user_code: "User Code",
            wrong_or_expired: "Wrong or expired code",
        }
    }

    fn build_de() -> Self {
        Self {
            accept: "Akzeptieren",
            auto_redirect_account: "Automatische Weiterleitung zum Account folgt",
            close_window: "Dieses Fenster kann nun geschlossen werden.",
            decline: "Ablehnen",
            desc: "Bitte den {{count}}-stelligen vom Geräte angezeigten Benutzer Code eingeben.",
            desc_scopes: "Das Gerät fragt Zugang an zu:",
            err_too_short: "Eingabe zu kurz",
            err_too_long: "Eingabe zu lang",
            invalid_input: "Ungültige Eingabe",
            is_accepted: "Die Anfrage wurde akzeptiert",
            is_declined: "Die Anfrage wurde abgewiesen",
            submit: "Absenden",
            title: "Gerät Authorisierung",
            user_code: "Benutzer Code",
            wrong_or_expired: "Ungültiger oder abgelaufener Code",
        }
    }

    fn build_zh_hans() -> Self {
        Self {
            accept: "接受",
            auto_redirect_account: "您将被自动重定向至您的账户。",
            close_window: "您现在可以关闭此窗口了。",
            decline: "拒绝",
            desc: "请输入来自您的设备的{{count}}位用户代码。",
            desc_scopes: "此设备请求访问：",
            err_too_short: "输入过短",
            err_too_long: "输入过长",
            invalid_input: "无效输入",
            is_accepted: "请求已被接受。",
            is_declined: "请求已被拒绝。",
            submit: "提交",
            title: "设备授权",
            user_code: "用户代码",
            wrong_or_expired: "代码错误或已过期",
        }
    }

    fn build_ko() -> Self {
        Self {
            accept: "수락",
            auto_redirect_account: "지금 계정으로 리다이렉트될 예정입니다.",
            close_window: "이제 창을 닫아도 됩니다.",
            decline: "거절",
            desc: "기기에서 {{count}}자의 사용자 코드를 입력해 주세요.",
            desc_scopes: "다음 기기가 접근을 요청합니다:",
            err_too_short: "입력이 너무 짧습니다.",
            err_too_long: "입력이 너무 깁니다.",
            invalid_input: "유효하지 않은 입력입니다.",
            is_accepted: "요청이 수락되었습니다.",
            is_declined: "요청이 거절되었습니다.",
            submit: "제출",
            title: "기기 인증",
            user_code: "사용자 코드",
            wrong_or_expired: "잘못되거나 만료된 코드입니다.",
        }
    }
}
