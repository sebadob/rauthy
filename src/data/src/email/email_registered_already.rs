use crate::email::i18n::email_registered_already::I18nEmailRegisteredAlready;
use crate::email::mailer::EMail;
use crate::entity::theme::ThemeCssFull;
use crate::entity::users::User;
use crate::rauthy_config::RauthyConfig;
use askama::Template;
use std::time::Duration;
use tracing::error;

#[derive(Default, Template)]
#[template(path = "email/email_registered_already.html")]
pub struct EmailRegisteredAlreadyHtml<'a> {
    pub lang: &'a str,
    pub theme_vars: String,
    pub email_sub_prefix: &'a str,
    pub link_request_new: &'a str,
    // i18n_email
    pub header: &'a str,
    pub text: &'a str,
    pub footer: &'a str,
    pub button_text_request_new: &'a str,
}

#[derive(Default, Template)]
#[template(path = "email/email_registered_already.txt")]
pub struct EmailRegisteredAlreadyTxt<'a> {
    pub email_sub_prefix: &'a str,
    pub link_request_new: &'a str,
    // i18n_email
    pub header: &'a str,
    pub text: &'a str,
    pub footer: &'a str,
}

/// This is sent out when you have an open user registration and the email exists in the DB already.
/// In this case, we want to basically do nothing but inform the user about it. We don't want to
/// show this information in the UI directly to prevent a username enumeration using the
/// registration endpoint.
pub async fn send_email_registered_already(user: &User) {
    let link_request_new = format!(
        "{}users/password_reset?email_hint={}",
        RauthyConfig::get().issuer,
        user.email
    );
    let theme_vars = ThemeCssFull::find_theme_variables_email()
        .await
        .unwrap_or_default();

    let i18n = I18nEmailRegisteredAlready::build(&user.language);
    let email_sub_prefix = &RauthyConfig::get().vars.email.sub_prefix;
    let text = EmailRegisteredAlreadyTxt {
        email_sub_prefix,
        link_request_new: &link_request_new,
        header: i18n.header,
        text: i18n.text,
        footer: i18n.footer.unwrap_or_default(),
    };

    let html = EmailRegisteredAlreadyHtml {
        lang: user.language.as_str(),
        theme_vars,
        email_sub_prefix,
        link_request_new: &link_request_new,
        header: i18n.header,
        text: i18n.text,
        footer: i18n.footer.unwrap_or_default(),
        button_text_request_new: i18n.button_text_request_new,
    };

    let req = EMail {
        recipient_name: user.email_recipient_name(),
        address: user.email.clone(),
        subject: format!("{} - {}", email_sub_prefix, i18n.subject),
        text: Some(
            text.render()
                .expect("Template rendering: EmailRegisteredAlreadyTxt"),
        ),
        html: Some(
            html.render()
                .expect("Template rendering: EmailRegisteredAlreadyHtml"),
        ),
    };

    let res = RauthyConfig::get()
        .tx_email
        .send_timeout(req, Duration::from_secs(10))
        .await;
    match res {
        Ok(_) => {}
        Err(ref e) => {
            error!(
                error = ?e,
                "sending email registered already info",
            );
        }
    }
}
