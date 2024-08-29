use crate::i18n::SsrJson;
use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nAuthorize<'a> {
    client_force_mfa: &'a str,
    email: &'a str,
    email_bad_format: &'a str,
    email_required: &'a str,
    email_sent_msg: &'a str,
    http_429: &'a str,
    invalid_credentials: &'a str,
    invalid_key_used: &'a str,
    login: &'a str,
    mfa_ack: &'a str,
    password: &'a str,
    password_forgotten: &'a str,
    password_request: &'a str,
    password_required: &'a str,
    provide_mfa: &'a str,
    request_expires: &'a str,
    sign_up: &'a str,
}

impl SsrJson for I18nAuthorize<'_> {
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

impl I18nAuthorize<'_> {
    fn build_en() -> Self {
        Self {
            client_force_mfa: r#"This login forces MFA to achieve higher security.
To get access, you need to log in to your account and add at least one additional Passkey"#,
            email: "E-Mail",
            email_bad_format: "Bad E-Mail format",
            email_required: "E-Mail is required",
            email_sent_msg: "If your E-Mail exists, a request has been sent",
            http_429: "Too many invalid inputs. Locked until:",
            invalid_credentials: "Invalid credentials",
            invalid_key_used: "Invalid Key",
            login: "Login",
            mfa_ack: "Acknowledged",
            password: "Password",
            password_forgotten: "Password forgotten?",
            password_request: "Request",
            password_required: "Password is required",
            provide_mfa: "Please login with your MFA device",
            request_expires: "Request expires",
            sign_up: "User Registration",
        }
    }

    fn build_de() -> Self {
        Self {
            client_force_mfa: r#"Dieser Login setzt MFA voraus für eine erhöhte Sicherheit.
Um Zugang zu bekommen, müssen Sie sie in Ihren Account einloggen und mindestens einen Passkey
hinzufügen."#,
            email: "E-Mail",
            email_bad_format: "Inkorrektes E-Mail Format",
            email_required: "E-Mail ist notwendig",
            email_sent_msg: "Sollte Ihre Adresse registriert sein, wurde eine Nachricht versandt",
            http_429: "Zu viele ungültige Versuche. Gesperrt bis:",
            invalid_credentials: "Ungültige Zugangsdaten",
            invalid_key_used: "Ungültiger Sicherheitsschlüssel",
            login: "Login",
            mfa_ack: "Bestätigt",
            password: "Password",
            password_forgotten: "Password vergessen?",
            password_request: "Anfordern",
            password_required: "Password ist notwendig",
            provide_mfa: "Bitte stellen Sie Ihr MFA Gerät zur Verfügung",
            request_expires: "Anfrage läuft ab",
            sign_up: "Benutzer Registrierung",
        }
    }

    fn build_zh_hans() -> Self {
        Self {
            client_force_mfa: r#"本次登陆强制使用多因子认证以增强安全性。
要完成登陆，请登入您的账户并添加一个登陆密钥。"#,
            email: "电子邮件地址",
            email_bad_format: "错误的电子邮件地址格式",
            email_required: "电子邮件地址必填。",
            email_sent_msg: "如果您的电子邮件存在，我们已发送请求邮件。",
            http_429: "过多无效输入，已锁定至：",
            invalid_credentials: "无效证明",
            invalid_key_used: "无效密钥",
            login: "登陆",
            mfa_ack: "已确认",
            password: "密码",
            password_forgotten: "忘记密码",
            password_request: "请求",
            password_required: "密码必填。",
            provide_mfa: "请使用MFA设备登陆",
            request_expires: "请求过期",
            sign_up: "用户注册",
        }
    }
}
