use crate::email::email_ts_prettify;
use crate::email::i18n::reset_info::I18nEmailResetInfo;
use crate::email::mailer::EMail;
use crate::entity::theme::ThemeCssFull;
use crate::entity::users::User;
use crate::rauthy_config::RauthyConfig;
use askama::Template;
use std::time::Duration;
use tracing::error;

#[derive(Default, Template)]
#[template(path = "email/reset_info.html")]
pub struct EMailResetInfoHtml<'a> {
    pub lang: &'a str,
    pub theme_vars: String,
    pub email_sub_prefix: &'a str,
    pub link: &'a str,
    pub exp: &'a str,
    // i18n_email
    pub expires_1: &'a str,
    pub expires_2: &'a str,
    pub update: &'a str,
    pub button_text: &'a str,
}

#[derive(Default, Template)]
#[template(path = "email/reset_info.txt")]
pub struct EmailResetInfoTxt<'a> {
    pub email_sub_prefix: &'a str,
    pub link: &'a str,
    pub exp: &'a str,
    // i18n_email
    pub expires_1: &'a str,
    pub expires_2: &'a str,
    pub update: &'a str,
}

pub async fn send_pwd_reset_info(user: &User) {
    let exp = email_ts_prettify(user.password_expires.unwrap(), &user.language, None);
    let link = format!(
        "{}/auth/v1/account",
        RauthyConfig::get().pub_url_with_scheme
    );

    let i18n = I18nEmailResetInfo::build(&user.language);
    let email_sub_prefix = &RauthyConfig::get().vars.email.sub_prefix;
    let text = EmailResetInfoTxt {
        email_sub_prefix,
        link: &link,
        exp: &exp.to_string(),
        expires_1: i18n.expires_1,
        expires_2: i18n.expires_2,
        update: i18n.update,
    };

    let theme_vars = ThemeCssFull::find_theme_variables_email()
        .await
        .unwrap_or_default();
    let html = EMailResetInfoHtml {
        lang: user.language.as_str(),
        theme_vars,
        email_sub_prefix,
        link: &link,
        exp: &exp.to_string(),
        expires_1: i18n.expires_1,
        expires_2: i18n.expires_2,
        update: i18n.update,
        button_text: i18n.button_text,
    };

    let req = EMail {
        recipient_name: user.email_recipient_name(),
        address: user.email.to_string(),
        subject: format!("{email_sub_prefix} - {}", i18n.subject),
        text: text
            .render()
            .expect("Template rendering: EmailResetInfoTxt"),
        html: Some(
            html.render()
                .expect("Template rendering: EmailResetInfoHtml"),
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
                user.email, error = ?e,
                "sending password reset info",
            );
        }
    }
}
