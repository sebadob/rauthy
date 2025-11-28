use crate::email::mailer::EMail;
use crate::entity::theme::ThemeCssFull;
use askama::Template;
use rauthy_notify::Notification;
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
