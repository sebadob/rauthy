use std::time::Duration;
use askama::Template;
use crate::{email::{i18n::otp::I18nEmailOtp, mailer::EMail}, entity::{theme::ThemeCssFull, users::User}, rauthy_config::{RauthyConfig}};
use tracing::error;

#[derive(Default, Template)]
#[template(path = "email/otp.html")]
pub struct EmailOtpHtml<'a> {
    pub lang: &'a str,
    pub theme_vars: String,
    pub email_sub_prefix: &'a str,
    pub code: &'a str,
    pub exp: String,
    pub header: &'a str,
    pub text: &'a str,
    pub validity: &'a str,
    pub expires: &'a str,
}

#[derive(Default, Template)]
#[template(path = "email/otp.txt")]
pub struct EmailOtpTxt<'a> {
    pub email_sub_prefix: &'a str,
    pub code: &'a str,
    pub exp: String,
    pub header: &'a str,
    pub text: &'a str,
    pub validity: &'a str,
    pub expires: &'a str,
}

pub async fn send_email_otp(code: &str, user: &User) {
    let theme_vars = ThemeCssFull::find_theme_variables_email()
        .await
        .unwrap_or_default();

    let email_sub_prefix = &RauthyConfig::get().vars.email.sub_prefix;

    let (subject, text, html) = {
        let i18n = I18nEmailOtp::build(&user.language);
        let exp = RauthyConfig::get().vars.otp.exp.to_string();

        let text = EmailOtpTxt {
            email_sub_prefix,
            code: code,
            exp: exp.clone(),
            header: i18n.header,
            text: i18n.text.unwrap_or_default(),
            validity: i18n.validity.unwrap_or_default(),
            expires: i18n.expires,
        };

        let html = EmailOtpHtml {
            lang: user.language.as_str(),
            theme_vars,
            email_sub_prefix,
            code: code,
            exp: exp,
            header: i18n.header,
            text: i18n.text.unwrap_or_default(),
            validity: i18n.validity.unwrap_or_default(),
            expires: i18n.expires,
        };

        (i18n.subject, text, html)
    };

    let req = EMail {
        recipient_name: user.email_recipient_name(),
        address: user.email.to_string(),
        subject: format!("{email_sub_prefix} - {subject}"),
        text: Some(text.render().expect("Template rendering: EmailOtpTxt")),
        html: Some(html.render().expect("Template rendering: EmailOtpHtml")),
    };

    let res = RauthyConfig::get()
        .tx_email
        .send_timeout(req, Duration::from_secs(10))
        .await;
    match res {
        Ok(_) => {}
        Err(ref e) => {
            error!(
                user.email, error = ?e,
                "sending otp email",
            );
        }
    }
}