use crate::email::mailer::EMail;
use crate::email::smtp_oauth_token::SmtpOauthToken;
use crate::rauthy_config::RauthyConfig;
use lettre::message;
use rauthy_common::HTTP_CLIENT;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::Serialize;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

#[derive(Debug, Serialize)]
struct EmailAddr {
    address: String,
    name: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Recipient {
    email_address: EmailAddr,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum EmailContentType {
    Text,
    Html,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct EmailBody {
    content: String,
    content_type: EmailContentType,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MicrosoftEmail<'a> {
    bcc_recipients: Vec<Recipient>,
    body: EmailBody,
    cc_recipients: Vec<Recipient>,
    reply_to: Vec<&'a Recipient>,
    // sender: Recipient,
    subject: String,
    to_recipients: Vec<Recipient>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MicrosoftMessage<'a> {
    message: MicrosoftEmail<'a>,
    save_to_sent_items: bool,
}

pub async fn sender_microsoft_graph(mut rx: mpsc::Receiver<EMail>) {
    let from = {
        let from: message::Mailbox = RauthyConfig::get()
            .vars
            .email
            .smtp_from
            .as_ref()
            .parse()
            .expect("SMTP_FROM could not be parsed correctly");

        Recipient {
            email_address: EmailAddr {
                address: from.email.to_string(),
                name: from.name,
            },
        }
    };

    info!("Using Microsoft Graph API to send out E-Mails");

    loop {
        debug!("Listening for incoming send E-Mail requests");
        if let Some(req) = rx.recv().await {
            debug!("New E-Mail for address: {:?}", req.address);

            let body = if let Some(content) = req.html {
                EmailBody {
                    content,
                    content_type: EmailContentType::Html,
                }
            } else {
                EmailBody {
                    content: req.text,
                    content_type: EmailContentType::Text,
                }
            };

            let email = MicrosoftMessage {
                message: MicrosoftEmail {
                    bcc_recipients: Vec::default(),
                    body,
                    cc_recipients: Vec::default(),
                    reply_to: vec![&from],
                    subject: req.subject,
                    to_recipients: vec![Recipient {
                        email_address: EmailAddr {
                            address: req.address,
                            name: Some(req.recipient_name),
                        },
                    }],
                },
                save_to_sent_items: false,
            };

            if let Err(err) = send_email(&email).await {
                error!("{err:?}");

                // short timeout if sending fails, maybe the network just had a short hiccup
                tokio::time::sleep(Duration::from_millis(500)).await;

                if let Err(err) = send_email(&email).await {
                    // we want to panic if multiple sends fail so emails don't get lost
                    // silently
                    panic!("Could not send E-Mail even after retrying: {err:?}");
                }
            }
        } else {
            warn!("Received 'None' in email 'sender' - exiting");
            break;
        }
    }
}

async fn send_email(email: &MicrosoftMessage<'_>) -> Result<(), ErrorResponse> {
    let token = SmtpOauthToken::get()
        .await
        .expect("Error retrieving SMTP OAuth Token");

    warn!(
        "SendMail URI: {}",
        RauthyConfig::get()
            .vars
            .email
            .microsoft_graph_uri
            .as_ref()
            .unwrap()
    );

    let res = HTTP_CLIENT
        .get()
        .unwrap()
        .post(
            RauthyConfig::get()
                .vars
                .email
                .microsoft_graph_uri
                .as_ref()
                .unwrap(),
        )
        .bearer_auth(token.access_token)
        .json(&email)
        .send()
        .await?;

    let to = &email
        .message
        .to_recipients
        .first()
        .unwrap()
        .email_address
        .address;
    if res.status().is_success() {
        info!("E-Mail to '{to}' sent successfully!",);
        Ok(())
    } else {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        Err(ErrorResponse::new(
            ErrorResponseType::Connection,
            format!("Error sending E-Mail via Microsoft Graph to '{to}': HTTP {status} {body}"),
        ))
    }
}
