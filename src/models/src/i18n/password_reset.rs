use crate::i18n::account::I18nAccountMfa;
use crate::i18n::password_policy::I18nPasswordPolicy;
use crate::i18n::SsrJson;
use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nPasswordReset {
    password_policy: I18nPasswordPolicy,

    account_login: &'static str,
    bad_format: &'static str,
    fido_link: &'static str,
    generate: &'static str,
    mfa: I18nAccountMfa,
    new_acc_desc_1: &'static str,
    new_acc_desc_2: &'static str,
    new_account: &'static str,
    password_reset: &'static str,
    password: &'static str,
    passwordless: &'static str,
    password_confirm: &'static str,
    password_no_match: &'static str,
    required: &'static str,
    save: &'static str,
    success_1: &'static str,
    success_2: &'static str,
    success_3: &'static str,
    success_passkey_1: &'static str,
    success_passkey_2: &'static str,
}

impl SsrJson for I18nPasswordReset {
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

// batman@batcave.io
// 6f#h5b+eLZ9$9h5%hBQH,RH+

impl I18nPasswordReset {
    pub fn build_en() -> Self {
        Self {
            password_policy: I18nPasswordPolicy::build_en(),

            account_login: "Account Login",
            bad_format: "Bad Format",
            fido_link: "https://fidoalliance.org/fido2",
            generate: "Generate",
            mfa: I18nAccountMfa::build_en(),
            new_acc_desc_1:
                "You have the option between two account types: passwordless or traditional password",
            new_acc_desc_2: r#"The passwordless account is always preferred, because it provides
a way with stronger security. You will need at least one passkey (Yubikey, Apple Touch ID, Windows Hello,
...) to create such an account. Your device must embrace the FIDO2 standard. For more information
about this, you may follow this link: "#,
            new_account: "New Account",
            password_reset: "Password Reset",
            password: "Password",
            passwordless: "FIDO Passkey",
            password_confirm: "Password Confirm",
            password_no_match: "Passwords do not match",
            required: "Required",
            save: "Save",
            success_1: "The password has been updated successfully.",
            success_2: "You will be redirected shortly.",
            success_3: "If you are not being redirected, please click here:",
            success_passkey_1: "Your new passkey has been registered successfully.",
            success_passkey_2: r#"Please log into your account and register a second backup key as
soon as possible. With a passkey only account, you wil not be able to use a password reset via
E-Mail in case you lose your current key."#,
        }
    }

    pub fn build_de() -> Self {
        Self {
            password_policy: I18nPasswordPolicy::build_de(),

            account_login: "Account Login",
            bad_format: "Ungültiges Format",
            fido_link: "https://fidoalliance.org/fido2",
            generate: "Generieren",
            mfa: I18nAccountMfa::build_de(),
            new_acc_desc_1: r#"Sie haben die Option zwischen zwei Account Typen zu wählen: Passwortlos oder
traditionalles Passwort."#,
            new_acc_desc_2: r#"Der passwortlose Account Typ ist immer zu bevorzugen. Er bietet einen
sehr viel höheren Sicherheitsstandard als traditionelle Passwörter und gleichzeitig einen einfacheren
und schnelleren Login.
Dazu wird mindestens ein Passkey (Yubikey, Apple Touch ID, Windows Hello, ...) benötigt, welcher dem
FIDO2 Standard gerecht wird. Für weitere Informationen können Sie diesem Link folgen: "#,
            new_account: "Neuer Account",
            password_reset: "Passwort Zurücksetzen",
            password: "Passwort",
            passwordless: "FIDO Passkey",
            password_confirm: "Passwort bestätigen",
            password_no_match: "Passwörter stimmen nicht überein",
            required: "Notwendig",
            save: "Speichern",
            success_1: "Das Passwort wurde erfolgreich zurückgesetzt.",
            success_2: "Sie werden in Kürze weitergeleitet.",
            success_3: "Sollte Sie nicht weitergeleitet werden, klicken Sie bitte hier:",
            success_passkey_1: "Der neue Passkey wurde erfolgreich registriert.",
            success_passkey_2: r#"Bitte loggen Sie sich direkt in Ihren Account ein und registrieren
Sie mindestens einen weiteren Backup Passkey. Ein passwortloser Account kann nicht den Passwort
Reset via E-Mail nutzen für den Fall, dass der derzeitige Passkey abhanden kommt."#,
        }
    }

    pub fn build_zh_hans() -> Self {
        Self {
            password_policy: I18nPasswordPolicy::build_zh_hans(),

            account_login: "账户登录",
            bad_format: "格式错误",
            fido_link: "https://fidoalliance.org/fido2/?lang=zh-hans",
            generate: "生成",
            mfa: I18nAccountMfa::build_zh_hans(),
            new_acc_desc_1: "您可以在无密码账户和传统的密码账户之中选择其一。",
            new_acc_desc_2: r#"无密码账户应被优先考虑，因为其提供更强的安全性。
您需要至少一个支持FIDO2标准的通行密钥（Yubikey、Apple Touch ID或Windows Hello等）以完成账户创建。
获取更多信息："#,
            new_account: "新账户",
            password_reset: "密码重置",
            password: "密码",
            passwordless: "FIDO通行密钥",
            password_confirm: "密码确认",
            password_no_match: "密码不匹配",
            required: "必填",
            save: "保存",
            success_1: "密码已成功更新。",
            success_2: "您将被重定向。",
            success_3: "如果您未被重定向，请点击此链接：",
            success_passkey_1: "您的通行密钥已成功注册。",
            success_passkey_2: r#"请登入您的账户并尽快注册一个备份密钥。
对于仅密钥登陆的账户，在丢失您当前的密钥时，您无法通过电子邮件进行密码重置。"#,
        }
    }

    pub fn build_ko() -> Self {
        Self {
            password_policy: I18nPasswordPolicy::build_ko(),

            account_login: "계정 로그인",
            bad_format: "잘못된 형식",
            fido_link: "https://fidoalliance.org/fido2/?lang=ko",
            generate: "생성",
            mfa: I18nAccountMfa::build_ko(),
            new_acc_desc_1: "계정 종류는 비밀번호가 없는 계정 또는 기존의 비밀번호가 있는 계정 중 하나를 선택할 수 있습니다.",
            new_acc_desc_2: r#"비밀번호가 없는 계정은 더 강력한 보안 방법을 제공하기 때문에 항상 선호됩니다.
이러한 계정을 생성하려면 최소 하나의 패스키(Yubikey, Apple Touch ID, Windows Hello, ...)가 필요합니다. 기기가 FIDO2 표준을 지원해야 합니다.
더 자세한 사항은 다음 링크를 참고해 주세요: "#,
            new_account: "새 계정",
            password_reset: "비밀번호 초기화",
            password: "비밀번호",
            passwordless: "FIDO 패스키",
            password_confirm: "비밀번호 확인",
            password_no_match: "비밀번호가 일치하지 않습니다.",
            required: "필수",
            save: "저장",
            success_1: "비밀번호가 성공적으로 변경되었습니다.",
            success_2: "곧 리다이렉트됩니다.",
            success_3: "만약 리다이렉트가 되지 않으면, 여기를 클릭해 주세요:",
            success_passkey_1: "새로운 패스키가 성공적으로 등록되었습니다.",
            success_passkey_2: r#"계정에 로그인하여 가능한 한 빨리 두 번째 백업 키를 등록해 주세요.
패스키 전용 계정은 현재 패스키를 잃어버리면, 이메일을 통하여 비밀번호 초기화할 수 없습니다."#,
        }
    }
}
