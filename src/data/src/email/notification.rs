use crate::email::mailer::{EMail, EmailType};
use crate::entity::theme::ThemeCssFull;
use crate::entity::users::User;
use crate::rauthy_config::RauthyConfig;
use askama::Template;
use rauthy_notify::{Notification, NotificationLevel};
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::error;

#[derive(Default, Template)]
#[template(path = "email/event.html")]
pub struct EMailEventHtml<'a> {
    pub lang: &'a str,
    pub theme_vars: String,
    pub head: &'a str,
    pub row_1: &'a str,
    pub row_2: &'a str,
}

#[derive(Default, Template)]
#[template(path = "email/event.txt")]
pub struct EMailEventTxt<'a> {
    pub head: &'a str,
    pub row_1: &'a str,
    pub row_2: &'a str,
}

pub async fn send_email_notification(
    recipient_name: String,
    address: String,
    tx_email: &mpsc::Sender<EMail>,
    notification: &Notification,
) {
    let text = EMailEventTxt {
        head: &notification.head,
        row_1: &notification.row_1,
        row_2: notification.row_2.as_deref().unwrap_or_default(),
    };

    let theme_vars = ThemeCssFull::find_theme_variables_email()
        .await
        .unwrap_or_default();
    let html = EMailEventHtml {
        lang: "en",
        theme_vars,
        head: text.head,
        row_1: text.row_1,
        row_2: text.row_2,
    };

    let req = EMail {
        typ: EmailType::Notification,
        recipient_name,
        address,
        subject: notification.head.to_string(),
        text: Some(text.render().expect("Template rendering: EMailEventTxt")),
        html: Some(html.render().expect("Template rendering: EMailEventHtml")),
    };

    let res = tx_email.send_timeout(req, Duration::from_secs(10)).await;
    match res {
        Ok(_) => {}
        Err(ref err) => {
            error!(?err, "sending Event E-Mail notification");
        }
    }
}

pub async fn send_email_passkey_removed(user: &User, passkey_name: &str) {
    let notification = passkey_removed_notification(passkey_name);
    send_email_notification(
        user.email_recipient_name(),
        user.email.clone(),
        &RauthyConfig::get().tx_email,
        &notification,
    )
    .await;
}

fn passkey_removed_notification(passkey_name: &str) -> Notification {
    Notification {
        level: NotificationLevel::Warning,
        head: "Passkey removed from your account".to_string(),
        row_1: format!("The passkey '{passkey_name}' was removed by an administrator."),
        row_2: Some(
            "If you did not expect this change, contact your administrator immediately."
                .to_string(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passkey_removed_email_identifies_the_key_and_warns_the_user() {
        let notification = passkey_removed_notification("security-key");

        assert_eq!(notification.level, NotificationLevel::Warning);
        assert!(notification.row_1.contains("security-key"));
        assert!(notification.row_2.unwrap().contains("immediately"));
    }
}
