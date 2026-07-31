use crate::{
    email::{
        i18n::otp::I18nEmailOtp,
        mailer::{EMail, EmailType},
    },
    entity::{theme::ThemeCssFull, users::User},
    rauthy_config::RauthyConfig,
};
use askama::Template;
use std::time::Duration;
use tracing::error;

#[derive(Default, Template)]
#[template(path = "email/otp.html")]
pub struct EmailOtpHtml<'a> {
    pub lang: &'a str,
    pub theme_vars: String,
    pub email_sub_prefix: &'a str,
    pub code: &'a str,
    pub header: &'a str,
    pub text: &'a str,
}

#[derive(Default, Template)]
#[template(path = "email/otp.txt")]
pub struct EmailOtpTxt<'a> {
    pub email_sub_prefix: &'a str,
    pub code: &'a str,
    pub header: &'a str,
    pub text: &'a str,
}

pub async fn send_email_otp(code: &str, user: &User) {
    let theme_vars = ThemeCssFull::find_theme_variables_email()
        .await
        .unwrap_or_default();

    let email_sub_prefix = &RauthyConfig::get().vars.email.sub_prefix;

    let (subject, text, html) = {
        let i18n = I18nEmailOtp::build(&user.language);

        let text = EmailOtpTxt {
            email_sub_prefix,
            code,
            header: i18n.header,
            text: i18n.text.unwrap_or_default(),
        };

        let html = EmailOtpHtml {
            lang: user.language.as_str(),
            theme_vars,
            email_sub_prefix,
            code,
            header: i18n.header,
            text: i18n.text.unwrap_or_default(),
        };

        (i18n.subject, text, html)
    };

    let req = EMail {
        typ: EmailType::OtpRequest,
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
