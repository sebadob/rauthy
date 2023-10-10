use crate::app_state::AppState;
use crate::entity::magic_links::MagicLink;
use crate::entity::users::User;
use crate::i18n::email_change_info_new::I18nEmailChangeInfoNew;
use crate::i18n::email_confirm_change::I18nEmailConfirmChange;
use crate::i18n::email_reset::I18nEmailReset;
use crate::i18n::email_reset_info::I18nEmailResetInfo;
use crate::i18n::SsrJson;
use actix_web::web;
use askama_actix::Template;
use lettre::message::{MultiPart, SinglePart};
use lettre::transport::smtp::authentication;
use lettre::{AsyncSmtpTransport, AsyncTransport};
use rauthy_common::constants::{
    EMAIL_SUB_PREFIX, SMTP_FROM, SMTP_PASSWORD, SMTP_URL, SMTP_USERNAME,
};
use std::time::Duration;
use time::OffsetDateTime;
use tokio::sync::mpsc::Receiver;
use tracing::{debug, error, info, warn};

#[derive(Debug)]
pub struct EMail {
    pub address: String,
    pub subject: String,
    pub text: String,
    pub html: Option<String>,
}

// TODO implement
// #[derive(Default, Template)]
// #[template(path = "email/change_info_old.html")]
// pub struct EMailChangeInfoOldHtml<'a> {
//     pub pub_url: &'a str,
//     pub link: &'a str,
//     // i18n
//     pub header: &'a str,
//     pub click_link: &'a str,
//     pub button_text: &'a str,
// }
//
// #[derive(Default, Template)]
// #[template(path = "email/change_info_old.txt")]
// pub struct EMailChangeInfoOldTxt<'a> {
//     pub pub_url: &'a str,
//     pub link: &'a str,
//     // i18n
//     pub header: &'a str,
//     pub click_link: &'a str,
// }

#[derive(Default, Template)]
#[template(path = "email/change_info_new.html")]
pub struct EMailChangeInfoNewHtml<'a> {
    pub email_sub_prefix: &'a str,
    pub link: &'a str,
    pub exp: &'a str,
    // i18n
    pub header: &'a str,
    pub click_link: &'a str,
    pub validity: &'a str,
    pub expires: &'a str,
    pub button_text: &'a str,
}

#[derive(Default, Template)]
#[template(path = "email/change_info_new.txt")]
pub struct EMailChangeInfoNewTxt<'a> {
    pub email_sub_prefix: &'a str,
    pub link: &'a str,
    pub exp: &'a str,
    // i18n
    pub header: &'a str,
    pub click_link: &'a str,
    pub validity: &'a str,
    pub expires: &'a str,
}

#[derive(Default, Template)]
#[template(path = "email/confirm_change.html")]
pub struct EMailConfirmChangeHtml<'a> {
    pub email_sub_prefix: &'a str,
    pub header: &'a str,
    pub msg: &'a str,
    pub email_changed_to: &'a str,
}

#[derive(Default, Template)]
#[template(path = "email/confirm_change.txt")]
pub struct EMailConfirmChangeTxt<'a> {
    pub email_sub_prefix: &'a str,
    pub header: &'a str,
    pub msg: &'a str,
    pub email_changed_to: &'a str,
}

#[derive(Default, Template)]
#[template(path = "email/reset.html")]
pub struct EMailResetHtml<'a> {
    pub email_sub_prefix: &'a str,
    pub link: &'a str,
    pub exp: &'a str,
    // i18n
    pub header: &'a str,
    pub click_link: &'a str,
    pub validity: &'a str,
    pub expires: &'a str,
    pub button_text: &'a str,
}

#[derive(Default, Template)]
#[template(path = "email/reset.txt")]
pub struct EmailResetTxt<'a> {
    pub email_sub_prefix: &'a str,
    pub link: &'a str,
    pub exp: &'a str,
    // i18n
    pub header: &'a str,
    pub click_link: &'a str,
    pub validity: &'a str,
    pub expires: &'a str,
}

#[derive(Default, Template)]
#[template(path = "email/reset_info.html")]
pub struct EMailResetInfoHtml<'a> {
    pub email_sub_prefix: &'a str,
    pub link: &'a str,
    pub exp: &'a str,
    // i18n
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
    // i18n
    pub expires_1: &'a str,
    pub expires_2: &'a str,
    pub update: &'a str,
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
    let exp = OffsetDateTime::from_unix_timestamp(magic_link.exp)
        .unwrap()
        .to_string();

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
        address: new_email.clone(),
        subject: format!("{} - {}", *EMAIL_SUB_PREFIX, i18n.subject,),
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
    if res.is_err() {}
}

pub async fn send_email_confirm_change(
    data: &web::Data<AppState>,
    user: &User,
    email_addr: &str,
    email_changed_to: &str,
) {
    let i18n = I18nEmailConfirmChange::build(&user.language);
    let text = EMailConfirmChangeTxt {
        email_sub_prefix: &EMAIL_SUB_PREFIX,
        header: i18n.subject,
        msg: i18n.msg,
        email_changed_to,
    };

    let html = EMailConfirmChangeHtml {
        email_sub_prefix: &EMAIL_SUB_PREFIX,
        header: i18n.subject,
        msg: i18n.msg,
        email_changed_to,
    };

    let req = EMail {
        address: email_addr.to_string(),
        subject: format!("{} - {}", *EMAIL_SUB_PREFIX, i18n.subject,),
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
    if res.is_err() {}
}

pub async fn send_pwd_reset(data: &web::Data<AppState>, magic_link: &MagicLink, user: &User) {
    let link = format!(
        "{}/users/{}/reset/{}?type={}",
        data.issuer, magic_link.user_id, &magic_link.id, magic_link.usage,
    );
    let exp = OffsetDateTime::from_unix_timestamp(magic_link.exp)
        .unwrap()
        .to_string();

    let i18n = I18nEmailReset::build(&user.language);
    let text = EmailResetTxt {
        email_sub_prefix: &EMAIL_SUB_PREFIX,
        link: &link,
        exp: &exp,
        header: i18n.header,
        click_link: i18n.click_link,
        validity: i18n.validity,
        expires: i18n.expires,
    };

    let html = EMailResetHtml {
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
        address: user.email.to_string(),
        subject: format!("{} - {}", *EMAIL_SUB_PREFIX, i18n.subject,),
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
    if res.is_err() {}
}

pub async fn send_pwd_reset_info(data: &web::Data<AppState>, user: &User) {
    let exp = OffsetDateTime::from_unix_timestamp(user.password_expires.unwrap())
        .expect("Corrupt user password expiry timestamp");
    let link = format!("{}/auth/v1/account.html", data.public_url);

    let i18n = I18nEmailResetInfo::build(&user.language);
    let text = EmailResetInfoTxt {
        email_sub_prefix: &EMAIL_SUB_PREFIX,
        link: &link,
        exp: &exp.to_string(),
        expires_1: i18n.expires_1,
        expires_2: i18n.expires_2,
        update: i18n.update,
    };

    let html = EMailResetInfoHtml {
        email_sub_prefix: &EMAIL_SUB_PREFIX,
        link: &link,
        exp: &exp.to_string(),
        expires_1: i18n.expires_1,
        expires_2: i18n.expires_2,
        update: i18n.update,
        button_text: i18n.button_text,
    };

    let req = EMail {
        address: user.email.to_string(),
        subject: format!("{} - {}", *EMAIL_SUB_PREFIX, i18n.subject,),
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
    if res.is_err() {}
}

pub async fn sender(mut rx: Receiver<EMail>, test_mode: bool) {
    debug!("E-Mail sender started");

    // to make the integration tests not panic, results are taken and just thrown away
    // not the nicest approach for now, but it works
    if test_mode {
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

    let creds = authentication::Credentials::new(SMTP_USERNAME.clone(), SMTP_PASSWORD.clone());
    let mailer = AsyncSmtpTransport::<lettre::Tokio1Executor>::relay(&SMTP_URL)
        .expect("Connection Error with 'SMTP_URL'")
        .credentials(creds)
        .build();

    loop {
        debug!("Listening for incoming send E-Mail requests");
        if let Some(req) = rx.recv().await {
            debug!("New E-Mail for address: {:?}", req.address);

            let to = format!("{} <{}>", req.subject, req.address);

            let email = if let Some(html) = req.html {
                lettre::Message::builder()
                    .from(
                        SMTP_FROM
                            .parse()
                            .expect("SMTP_FROM could not be parsed correctly"),
                    )
                    .to(to.parse().unwrap())
                    .subject(req.subject)
                    .multipart(MultiPart::alternative_plain_html(req.text, html))
            } else {
                lettre::Message::builder()
                    .from(
                        SMTP_FROM
                            .parse()
                            .expect("SMTP_FROM could not be parsed correctly"),
                    )
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
