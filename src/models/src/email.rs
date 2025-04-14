use crate::app_state::AppState;
use crate::entity::magic_links::MagicLink;
use crate::entity::theme::ThemeCssFull;
use crate::entity::users::User;
use crate::i18n_email::change_info_new::I18nEmailChangeInfoNew;
use crate::i18n_email::confirm_change::I18nEmailConfirmChange;
use crate::i18n_email::password_new::I18nEmailPasswordNew;
use crate::i18n_email::reset::I18nEmailReset;
use crate::i18n_email::reset_info::I18nEmailResetInfo;
use actix_web::web;
use chrono::DateTime;
use lettre::message::{MultiPart, SinglePart};
use lettre::transport::smtp::authentication;
use lettre::{AsyncSmtpTransport, AsyncTransport, message};
use rauthy_common::constants::{
    EMAIL_SUB_PREFIX, SMTP_FROM, SMTP_PASSWORD, SMTP_URL, SMTP_USERNAME,
};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_notify::Notification;
use rinja_actix::Template;
use std::env;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tracing::{debug, error, info, warn};

#[derive(Debug)]
pub struct EMail {
    pub recipient_name: String,
    pub address: String,
    pub subject: String,
    pub text: String,
    pub html: Option<String>,
}

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
}

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
        text: text.render().expect("Template rendering: EMailEventTxt"),
        html: Some(html.render().expect("Template rendering: EMailEventHtml")),
    };

    let res = tx_email.send_timeout(req, Duration::from_secs(10)).await;
    match res {
        Ok(_) => {}
        Err(ref err) => {
            error!("Error sending Event E-Mail notification: {:?}", err);
        }
    }
}

pub async fn send_email_change_info_new(
    data: &web::Data<AppState>,
    magic_link: &MagicLink,
    user: &User,
    new_email: String,
) {
    let link = format!(
        "{}/users/{}/email_confirm/{}",
        data.issuer, magic_link.user_id, &magic_link.id,
    );
    let exp = email_ts_prettify(magic_link.exp);
    let theme_vars = ThemeCssFull::find_theme_variables_email()
        .await
        .unwrap_or_default();

    let i18n = I18nEmailChangeInfoNew::build(&user.language);
    let text = EMailChangeInfoNewTxt {
        email_sub_prefix: &EMAIL_SUB_PREFIX,
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
        email_sub_prefix: &EMAIL_SUB_PREFIX,
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
        subject: format!("{} - {}", *EMAIL_SUB_PREFIX, i18n.subject),
        text: text
            .render()
            .expect("Template rendering: EMailChangeInfoNewTxt"),
        html: Some(
            html.render()
                .expect("Template rendering: EMailChangeInfoNewHtml"),
        ),
    };

    let tx = &data.tx_email;
    let res = tx.send_timeout(req, Duration::from_secs(10)).await;
    match res {
        Ok(_) => {}
        Err(ref e) => {
            error!(
                "Error sending magic link email request for user '{}': {:?}",
                new_email, e
            );
        }
    }
}

pub async fn send_email_confirm_change(
    data: &web::Data<AppState>,
    user: &User,
    email_addr: &str,
    email_changed_to: &str,
    was_admin_action: bool,
) {
    let i18n = I18nEmailConfirmChange::build(&user.language);
    let text = EMailConfirmChangeTxt {
        email_sub_prefix: &EMAIL_SUB_PREFIX,
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
        email_sub_prefix: &EMAIL_SUB_PREFIX,
        header: i18n.subject,
        msg: i18n.msg,
        email_changed_to,
        changed_by_admin: text.changed_by_admin,
    };

    let req = EMail {
        recipient_name: user.email_recipient_name(),
        address: email_addr.to_string(),
        subject: format!("{} - {}", *EMAIL_SUB_PREFIX, i18n.subject),
        text: text
            .render()
            .expect("Template rendering: EMailConfirmChangeTxt"),
        html: Some(
            html.render()
                .expect("Template rendering: EMailConfirmChangeHtml"),
        ),
    };

    let tx = &data.tx_email;
    let res = tx.send_timeout(req, Duration::from_secs(10)).await;
    match res {
        Ok(_) => {}
        Err(ref e) => {
            error!(
                "Error sending email change confirm for user '{}': {:?}",
                email_addr, e
            );
        }
    }
}

pub async fn send_pwd_reset(data: &web::Data<AppState>, magic_link: &MagicLink, user: &User) {
    let link = format!(
        "{}/users/{}/reset/{}?type={}",
        data.issuer, magic_link.user_id, &magic_link.id, magic_link.usage,
    );
    let exp = email_ts_prettify(magic_link.exp);
    let theme_vars = ThemeCssFull::find_theme_variables_email()
        .await
        .unwrap_or_default();

    let (subject, text, html) = if user.password.is_none() {
        let i18n = I18nEmailPasswordNew::build(&user.language);
        let text = EmailResetTxt {
            email_sub_prefix: &EMAIL_SUB_PREFIX,
            link: &link,
            exp: &exp,
            header: i18n.header,
            click_link: i18n.click_link,
            text: i18n.text.unwrap_or_default(),
            validity: i18n.validity,
            expires: i18n.expires,
            footer: i18n.text.unwrap_or_default(),
        };

        let html = EMailResetHtml {
            lang: user.language.as_str(),
            theme_vars,
            email_sub_prefix: &EMAIL_SUB_PREFIX,
            link: &link,
            exp: &exp,
            header: i18n.header,
            click_link: i18n.click_link,
            text: i18n.text.unwrap_or_default(),
            validity: i18n.validity,
            expires: i18n.expires,
            button_text: i18n.button_text,
            footer: i18n.text.unwrap_or_default(),
        };

        (i18n.subject, text, html)
    } else {
        let i18n = I18nEmailReset::build(&user.language);
        let text = EmailResetTxt {
            email_sub_prefix: &EMAIL_SUB_PREFIX,
            link: &link,
            exp: &exp,
            header: i18n.header,
            click_link: i18n.click_link,
            text: i18n.text.unwrap_or_default(),
            validity: i18n.validity,
            expires: i18n.expires,
            footer: i18n.text.unwrap_or_default(),
        };

        let html = EMailResetHtml {
            lang: user.language.as_str(),
            theme_vars,
            email_sub_prefix: &EMAIL_SUB_PREFIX,
            link: &link,
            exp: &exp,
            header: i18n.header,
            click_link: i18n.click_link,
            text: i18n.text.unwrap_or_default(),
            validity: i18n.validity,
            expires: i18n.expires,
            button_text: i18n.button_text,
            footer: i18n.text.unwrap_or_default(),
        };

        (i18n.subject, text, html)
    };

    let req = EMail {
        recipient_name: user.email_recipient_name(),
        address: user.email.to_string(),
        subject: format!("{} - {}", *EMAIL_SUB_PREFIX, subject),
        text: text.render().expect("Template rendering: EmailResetTxt"),
        html: Some(html.render().expect("Template rendering: EmailResetHtml")),
    };

    let tx = &data.tx_email;
    let res = tx.send_timeout(req, Duration::from_secs(10)).await;
    match res {
        Ok(_) => {}
        Err(ref e) => {
            error!(
                "Error sending magic link email request for user '{}': {:?}",
                user.email, e
            );
        }
    }
}

pub async fn send_pwd_reset_info(data: &web::Data<AppState>, user: &User) {
    let exp = email_ts_prettify(user.password_expires.unwrap());
    let link = format!("{}/auth/v1/account", data.public_url);

    let i18n = I18nEmailResetInfo::build(&user.language);
    let text = EmailResetInfoTxt {
        email_sub_prefix: &EMAIL_SUB_PREFIX,
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
        email_sub_prefix: &EMAIL_SUB_PREFIX,
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
        subject: format!("{} - {}", *EMAIL_SUB_PREFIX, i18n.subject),
        text: text
            .render()
            .expect("Template rendering: EmailResetInfoTxt"),
        html: Some(
            html.render()
                .expect("Template rendering: EmailResetInfoHtml"),
        ),
    };

    let tx = &data.tx_email;
    let res = tx.send_timeout(req, Duration::from_secs(10)).await;
    match res {
        Ok(_) => {}
        Err(ref e) => {
            error!(
                "Error sending magic link email request for user '{}': {:?}",
                user.email, e
            );
        }
    }
}

pub async fn sender(mut rx: Receiver<EMail>, test_mode: bool) {
    debug!("E-Mail sender started");

    // to make the integration tests not panic, results are taken and just thrown away
    // not the nicest approach for now, but it works
    if test_mode || SMTP_URL.is_none() {
        if SMTP_URL.is_none() {
            error!("SMTP_URL is not configured, cannot send out any E-Mails!");
        }

        loop {
            let req = rx.recv().await;
            if req.is_some() {
                debug!(
                    "New E-Mail for address: {:?}",
                    req.as_ref().unwrap().address
                );
            } else {
                warn!("Received 'None' in email 'sender' - exiting");
                return;
            }
        }
    }

    let mailer = {
        let smtp_url = SMTP_URL.as_deref().unwrap();
        let smtp_port = env::var("SMTP_PORT")
            .map(|rl| {
                rl.parse::<u16>()
                    .expect("SMTP_PORT cannot be parsed to u16 - bad format")
            })
            .ok();
        let smtp_insecure = env::var("SMTP_DANGER_INSECURE")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .expect("Cannot parse SMTP_DANGER_INSECURE to bool");

        let mut retries = 0;
        let retries_max = env::var("SMTP_CONNECT_RETRIES")
            .unwrap_or_else(|_| "3".to_string())
            .trim()
            .parse::<u16>()
            .expect("Cannot parse SMTP_CONNECT_RETRIES to u16");

        let mut conn = if smtp_insecure {
            conn_test_smtp_insecure(smtp_url, smtp_port).await
        } else {
            connect_test_smtp(smtp_url, smtp_port).await
        };

        while let Err(err) = conn {
            error!("{:?}", err);

            if retries >= retries_max {
                panic!("SMTP connection retries exceeded");
            }
            retries += 1;
            tokio::time::sleep(Duration::from_secs(5)).await;

            conn = if smtp_insecure {
                conn_test_smtp_insecure(smtp_url, smtp_port).await
            } else {
                connect_test_smtp(smtp_url, smtp_port).await
            }
        }
        conn.unwrap()
    };

    let from: message::Mailbox = SMTP_FROM
        .parse()
        .expect("SMTP_FROM could not be parsed correctly");

    loop {
        debug!("Listening for incoming send E-Mail requests");
        if let Some(req) = rx.recv().await {
            debug!("New E-Mail for address: {:?}", req.address);

            let to = format!("{} <{}>", req.recipient_name, req.address);

            let email = if let Some(html) = req.html {
                lettre::Message::builder()
                    .from(from.clone())
                    .to(to.parse().unwrap())
                    .subject(req.subject)
                    .multipart(MultiPart::alternative_plain_html(req.text, html))
            } else {
                lettre::Message::builder()
                    .from(from.clone())
                    .to(to.parse().unwrap())
                    .subject(req.subject)
                    .singlepart(SinglePart::plain(req.text))
            };

            match email {
                Ok(addr) => match mailer.send(addr).await {
                    Ok(_) => info!("E-Mail to '{}' sent successfully!", req.address),
                    Err(e) => error!("Could not send E-Mail: {:?}", e),
                },
                Err(_) => error!("Error building the E-Mail to '{}'", req.address),
            }
        } else {
            warn!("Received 'None' in email 'sender' - exiting");
            break;
        }
    }
}

async fn connect_test_smtp(
    smtp_url: &str,
    smtp_port: Option<u16>,
) -> Result<AsyncSmtpTransport<lettre::Tokio1Executor>, ErrorResponse> {
    let creds = authentication::Credentials::new(SMTP_USERNAME.clone(), SMTP_PASSWORD.clone());

    // always try fully wrapped TLS first
    let mut builder = AsyncSmtpTransport::<lettre::Tokio1Executor>::relay(smtp_url)
        .expect("Connection Error with 'SMTP_URL'");
    if let Some(port) = smtp_port {
        builder = builder.port(port);
    }
    let mut conn = builder
        .credentials(creds.clone())
        .timeout(Some(Duration::from_secs(10)))
        .build();

    match conn.test_connection().await {
        Ok(true) => {
            info!("Successfully connected to {} via TLS", smtp_url);
            return Ok(conn);
        }
        Ok(false) => {
            warn!(
                "Could not connect to {} via TLS. Trying downgrade to STARTTLS",
                smtp_url,
            );
        }
        Err(err) => {
            warn!("Could not connect to {} via TLS: {:?}", smtp_url, err,);
        }
    }

    // only if full TLS fails, try STARTTLS
    builder = AsyncSmtpTransport::<lettre::Tokio1Executor>::starttls_relay(smtp_url)
        .expect("Connection Error with 'SMTP_URL'");
    if let Some(port) = smtp_port {
        builder = builder.port(port);
    }
    conn = builder
        .credentials(creds)
        .timeout(Some(Duration::from_secs(10)))
        .build();

    match conn.test_connection().await {
        Ok(true) => {
            info!("Successfully connected to {} via STARTTLS", smtp_url);
            return Ok(conn);
        }
        Ok(false) => {
            error!("Could not connect to {} via STARTTLS either", smtp_url);
        }
        Err(err) => {
            warn!(
                "Could not connect to {} via STARTTLS either: {:?}",
                smtp_url, err,
            );
        }
    }

    Err(ErrorResponse::new(
        ErrorResponseType::Internal,
        format!(
            "Could not connect to {} - neither TLS nor STARTTLS worked",
            smtp_url
        ),
    ))
}

async fn conn_test_smtp_insecure(
    smtp_url: &str,
    smtp_port: Option<u16>,
) -> Result<AsyncSmtpTransport<lettre::Tokio1Executor>, ErrorResponse> {
    let port = smtp_port.unwrap_or(1025);

    let conn = AsyncSmtpTransport::<lettre::Tokio1Executor>::builder_dangerous(smtp_url)
        .port(port)
        .build();
    match conn.test_connection().await {
        Ok(true) => {
            warn!(
                "Successfully connected to INSECURE SMTP relay {}:{}",
                smtp_url, port
            );
            Ok(conn)
        }
        Ok(false) => {
            error!(
                "Could not connect to insecure SMTP relay on {}:{}",
                smtp_url, port
            );
            Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                "Could not connect to localhost SMTP relay",
            ))
        }
        Err(err) => {
            error!(
                "Could not connect to insecure SMTP relay on {}:{} -> {:?}",
                smtp_url, port, err
            );
            Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                "Could not connect to localhost SMTP relay",
            ))
        }
    }
}

/// Prettifies unix timestamps for E-Mails in a better readable format for end users
#[inline]
fn email_ts_prettify(ts: i64) -> String {
    let dt = DateTime::from_timestamp(ts, 0).unwrap_or_default();
    let fmt = dt.format("%d/%m/%Y %H:%M:%S");
    format!("{} UTC", fmt)
}
