use crate::email::mailer::EMail;
use crate::entity::theme::ThemeCssFull;
use crate::entity::users::User;
use crate::i18n_email::confirm_change::I18nEmailConfirmChange;
use crate::rauthy_config::RauthyConfig;
use askama::Template;
use std::time::Duration;
use tracing::error;

#[derive(Default, Template)]
#[template(path = "email/confirm_change.html")]
pub struct EMailConfirmChangeHtml<'a> {
    pub lang: &'a str,
    pub theme_vars: String,
    pub email_sub_prefix: &'a str,
    pub header: &'a str,
    pub msg: &'a str,
    pub email_changed_to: &'a str,
    pub changed_by_admin: &'a str,
}

#[derive(Default, Template)]
#[template(path = "email/confirm_change.txt")]
pub struct EMailConfirmChangeTxt<'a> {
    pub email_sub_prefix: &'a str,
    pub header: &'a str,
    pub msg: &'a str,
    pub email_changed_to: &'a str,
    pub changed_by_admin: &'a str,
}

pub async fn send_email_confirm_change(
    user: &User,
    email_addr: &str,
    email_changed_to: &str,
    was_admin_action: bool,
) {
    let i18n = I18nEmailConfirmChange::build(&user.language);
    let email_sub_prefix = &RauthyConfig::get().vars.email.sub_prefix;
    let text = EMailConfirmChangeTxt {
        email_sub_prefix,
        header: i18n.subject,
        msg: i18n.msg,
        email_changed_to,
        changed_by_admin: if was_admin_action {
            i18n.msg_from_admin
        } else {
            ""
        },
    };

    let theme_vars = ThemeCssFull::find_theme_variables_email()
        .await
        .unwrap_or_default();
    let html = EMailConfirmChangeHtml {
        lang: user.language.as_str(),
        theme_vars,
        email_sub_prefix,
        header: i18n.subject,
        msg: i18n.msg,
        email_changed_to,
        changed_by_admin: text.changed_by_admin,
    };

    let req = EMail {
        recipient_name: user.email_recipient_name(),
        address: email_addr.to_string(),
        subject: format!("{email_sub_prefix} - {}", i18n.subject),
        text: text
            .render()
            .expect("Template rendering: EMailConfirmChangeTxt"),
        html: Some(
            html.render()
                .expect("Template rendering: EMailConfirmChangeHtml"),
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
                email_addr, error = ?e,
                "sending email change confirm",
            );
        }
    }
}
