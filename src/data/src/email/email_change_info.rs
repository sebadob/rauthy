use crate::email::email_ts_prettify;
use crate::email::i18n::change_info_new::I18nEmailChangeInfoNew;
use crate::email::mailer::EMail;
use crate::entity::magic_links::MagicLink;
use crate::entity::theme::ThemeCssFull;
use crate::entity::users::User;
use crate::rauthy_config::RauthyConfig;
use askama::Template;
use std::time::Duration;
use tracing::error;

#[derive(Default, Template)]
#[template(path = "email/change_info.html")]
pub struct EMailChangeInfoNewHtml<'a> {
    pub lang: &'a str,
    pub theme_vars: String,
    pub email_sub_prefix: &'a str,
    pub link: &'a str,
    pub exp: &'a str,
    // i18n_email
    pub header: &'a str,
    pub click_link: &'a str,
    pub validity: &'a str,
    pub expires: &'a str,
    pub button_text: &'a str,
}

#[derive(Default, Template)]
#[template(path = "email/change_info.txt")]
pub struct EMailChangeInfoNewTxt<'a> {
    pub email_sub_prefix: &'a str,
    pub link: &'a str,
    pub exp: &'a str,
    // i18n_email
    pub header: &'a str,
    pub click_link: &'a str,
    pub validity: &'a str,
    pub expires: &'a str,
}

pub async fn send_email_change_info_new(
    magic_link: &MagicLink,
    user: &User,
    user_tz: Option<&str>,
    new_email: String,
) {
    let link = format!(
        "{}users/{}/email_confirm/{}",
        RauthyConfig::get().issuer,
        magic_link.user_id,
        &magic_link.id,
    );
    let exp = email_ts_prettify(magic_link.exp, &user.language, user_tz);
    let theme_vars = ThemeCssFull::find_theme_variables_email()
        .await
        .unwrap_or_default();

    let i18n = I18nEmailChangeInfoNew::build(&user.language);
    let email_sub_prefix = &RauthyConfig::get().vars.email.sub_prefix;
    let text = EMailChangeInfoNewTxt {
        email_sub_prefix,
        link: &link,
        exp: &exp,
        header: i18n.header,
        click_link: i18n.click_link,
        validity: i18n.validity,
        expires: i18n.expires,
    };

    let html = EMailChangeInfoNewHtml {
        lang: user.language.as_str(),
        theme_vars,
        email_sub_prefix,
        link: &link,
        exp: &exp,
        header: i18n.header,
        click_link: i18n.click_link,
        validity: i18n.validity,
        expires: i18n.expires,
        button_text: i18n.button_text,
    };

    let req = EMail {
        recipient_name: user.email_recipient_name(),
        address: new_email.clone(),
        subject: format!("{} - {}", email_sub_prefix, i18n.subject),
        text: Some(
            text.render()
                .expect("Template rendering: EMailChangeInfoNewTxt"),
        ),
        html: Some(
            html.render()
                .expect("Template rendering: EMailChangeInfoNewHtml"),
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
                new_email, error = ?e,
                "sending magic link email request",
            );
        }
    }
}
