use crate::email::email_ts_prettify;
use crate::email::i18n::password_new::I18nEmailPasswordNew;
use crate::email::i18n::reset::I18nEmailReset;
use crate::email::mailer::EMail;
use crate::entity::magic_links::MagicLink;
use crate::entity::theme::ThemeCssFull;
use crate::entity::users::User;
use crate::rauthy_config::RauthyConfig;
use askama::Template;
use std::time::Duration;
use tracing::error;

#[derive(Default, Template)]
#[template(path = "email/reset.html")]
pub struct EMailResetHtml<'a> {
    pub lang: &'a str,
    pub theme_vars: String,
    pub email_sub_prefix: &'a str,
    pub link: &'a str,
    pub exp: &'a str,
    // i18n_email
    pub header: &'a str,
    pub click_link: &'a str,
    pub text: &'a str,
    pub validity: &'a str,
    pub expires: &'a str,
    pub button_text: &'a str,
    pub footer: &'a str,
    pub link_request_new: &'a str,
    pub button_text_request_new: &'a str,
}

#[derive(Default, Template)]
#[template(path = "email/reset.txt")]
pub struct EmailResetTxt<'a> {
    pub email_sub_prefix: &'a str,
    pub link: &'a str,
    pub exp: &'a str,
    // i18n_email
    pub header: &'a str,
    pub click_link: &'a str,
    pub text: &'a str,
    pub validity: &'a str,
    pub expires: &'a str,
    pub footer: &'a str,
    pub link_request_new: &'a str,
}

pub async fn send_pwd_reset(magic_link: &MagicLink, user: &User, user_tz: Option<&str>) {
    let link = format!(
        "{}users/{}/reset/{}?type={}",
        RauthyConfig::get().issuer,
        magic_link.user_id,
        &magic_link.id,
        magic_link.usage,
    );
    let exp = email_ts_prettify(magic_link.exp, &user.language, user_tz);
    let theme_vars = ThemeCssFull::find_theme_variables_email()
        .await
        .unwrap_or_default();

    let is_new_user = user.password.is_none() && !user.has_webauthn_enabled();
    let link_request_new: Option<String>;

    let email_sub_prefix = &RauthyConfig::get().vars.email.sub_prefix;
    let (subject, text, html) = if is_new_user {
        let i18n = I18nEmailPasswordNew::build(&user.language);
        let text = EmailResetTxt {
            email_sub_prefix,
            link: &link,
            exp: &exp,
            header: i18n.header,
            click_link: i18n.click_link,
            text: i18n.text.unwrap_or_default(),
            validity: i18n.validity,
            expires: i18n.expires,
            footer: i18n.footer.unwrap_or_default(),
            link_request_new: "",
        };

        let html = EMailResetHtml {
            lang: user.language.as_str(),
            theme_vars,
            email_sub_prefix,
            link: &link,
            exp: &exp,
            header: i18n.header,
            click_link: i18n.click_link,
            text: i18n.text.unwrap_or_default(),
            validity: i18n.validity,
            expires: i18n.expires,
            button_text: i18n.button_text,
            footer: i18n.footer.unwrap_or_default(),
            link_request_new: "",
            button_text_request_new: "",
        };

        (i18n.subject, text, html)
    } else {
        let i18n = I18nEmailReset::build(&user.language);
        link_request_new = Some(format!(
            "{}users/password_reset?email_hint={}",
            RauthyConfig::get().issuer,
            user.email
        ));

        let text = EmailResetTxt {
            email_sub_prefix,
            link: &link,
            exp: &exp,
            header: i18n.header,
            click_link: i18n.click_link,
            text: i18n.text.unwrap_or_default(),
            validity: i18n.validity,
            expires: i18n.expires,
            footer: i18n.footer.unwrap_or_default(),
            link_request_new: link_request_new.as_ref().unwrap(),
        };

        let html = EMailResetHtml {
            lang: user.language.as_str(),
            theme_vars,
            email_sub_prefix,
            link: &link,
            exp: &exp,
            header: i18n.header,
            click_link: i18n.click_link,
            text: i18n.text.unwrap_or_default(),
            validity: i18n.validity,
            expires: i18n.expires,
            button_text: i18n.button_text,
            footer: i18n.footer.unwrap_or_default(),
            link_request_new: link_request_new.as_ref().unwrap(),
            button_text_request_new: i18n.button_text_request_new.unwrap_or_default(),
        };

        (i18n.subject, text, html)
    };

    let req = EMail {
        recipient_name: user.email_recipient_name(),
        address: user.email.to_string(),
        subject: format!("{email_sub_prefix} - {subject}"),
        text: Some(text.render().expect("Template rendering: EmailResetTxt")),
        html: Some(html.render().expect("Template rendering: EmailResetHtml")),
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
                "sending password reset email",
            );
        }
    }
}
