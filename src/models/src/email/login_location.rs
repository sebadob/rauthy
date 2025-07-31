use crate::email::mailer::EMail;
use crate::entity::theme::ThemeCssFull;
use crate::entity::users::User;
use crate::i18n_email::login_location::I18nEmailLoginLocation;
use crate::rauthy_config::RauthyConfig;
use askama::Template;
use std::time::Duration;
use tracing::error;

#[derive(Default, Template)]
#[template(path = "email/login_location.html")]
pub struct EMailLoginLocationHtml<'a> {
    pub lang: &'a str,
    pub theme_vars: String,
    pub email_sub_prefix: &'a str,
    pub ip: &'a str,
    pub user_agent: &'a str,
    pub location: &'a str,
    pub link_revoke: &'a str,
    pub link_account: &'a str,
    // i18n_email
    pub unknown_location: &'a str,
    pub if_invalid: &'a str,
    pub revoke_link: &'a str,
    pub account_link: &'a str,
}

#[derive(Default, Template)]
#[template(path = "email/login_location.txt")]
pub struct EmailLoginLocationTxt<'a> {
    pub email_sub_prefix: &'a str,
    pub ip: &'a str,
    pub user_agent: &'a str,
    pub location: &'a str,
    pub link_revoke: &'a str,
    pub link_account: &'a str,
    // i18n_email
    pub unknown_location: &'a str,
    pub if_invalid: &'a str,
    pub revoke_link: &'a str,
    pub account_link: &'a str,
}

pub async fn send_login_location(
    user: &User,
    ip: String,
    user_agent: String,
    location: Option<String>,
    revoke_code: String,
) {
    let pub_url = &RauthyConfig::get().pub_url_with_scheme;
    let link_revoke = format!(
        "{pub_url}/auth/v1/users/{}/revoke/{revoke_code}?ip={ip}",
        user.id
    );
    let link_account = format!("{pub_url}/auth/v1/account");
    let location = location.as_deref().unwrap_or_default();

    let i18n = I18nEmailLoginLocation::build(&user.language);
    let email_sub_prefix = &RauthyConfig::get().vars.email.sub_prefix;
    let text = EmailLoginLocationTxt {
        email_sub_prefix,
        ip: &ip,
        user_agent: &user_agent,
        location,
        link_revoke: &link_revoke,
        link_account: &link_account,
        unknown_location: i18n.unknown_location,
        if_invalid: i18n.if_invalid,
        revoke_link: i18n.revoke_link,
        account_link: i18n.account_link,
    };

    let theme_vars = ThemeCssFull::find_theme_variables_email()
        .await
        .unwrap_or_default();
    let html = EMailLoginLocationHtml {
        lang: user.language.as_str(),
        theme_vars,
        email_sub_prefix,
        ip: &ip,
        user_agent: &user_agent,
        location,
        link_revoke: &link_revoke,
        link_account: &link_account,
        unknown_location: i18n.unknown_location,
        if_invalid: i18n.if_invalid,
        revoke_link: i18n.revoke_link,
        account_link: i18n.account_link,
    };

    let req = EMail {
        recipient_name: user.email_recipient_name(),
        address: user.email.to_string(),
        subject: format!("{email_sub_prefix} - {}", i18n.subject),
        text: text
            .render()
            .expect("Template rendering: EMailLoginLocationTxt"),
        html: Some(
            html.render()
                .expect("Template rendering: EMailLoginLocationHtml"),
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
                "sending login from new location email",
            );
        }
    }
}
