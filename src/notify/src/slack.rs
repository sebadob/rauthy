use crate::{Notification, Notify};
use async_trait::async_trait;
use rauthy_common::http_client;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::Serialize;
use tracing::{debug, error};

#[derive(Debug)]
pub struct NotifierSlack {
    webhook_url: String,
}

impl NotifierSlack {
    pub fn new(webhook_url: String) -> Self {
        Self { webhook_url }
    }
}

#[async_trait]
impl Notify for NotifierSlack {
    async fn notify(&self, notification: &Notification) -> Result<(), ErrorResponse> {
        debug!("Sending message to Slack");

        let color = notification.level.as_hex_color();
        let fields = vec![SlackMessageField::new(
            format!("*{}*", notification.head),
            format!(
                "{}\n{}",
                notification.row_1,
                notification.row_2.as_deref().unwrap_or_default(),
            ),
        )];
        let msg = SlackMessageApi::new(color, fields);
        debug!("{:?}", msg);

        match http_client()
            .post(&self.webhook_url)
            .json(&msg)
            .send()
            .await
        {
            Ok(_) => {
                debug!("Slack message sent successfully");
                Ok(())
            }
            Err(err) => {
                let e = format!("Unable to send message to Slack: {:?}", err);
                error!("{e}");
                Err(ErrorResponse::new(ErrorResponseType::Connection, e))
            }
        }
    }
}

/// Matches the Slack API
#[derive(Debug, Serialize)]
struct SlackMessageApi<'a> {
    pub color: &'a str,
    pub fields: Vec<SlackMessageField>,
}

impl SlackMessageApi<'_> {
    pub fn new(color: &str, fields: Vec<SlackMessageField>) -> SlackMessageApi {
        SlackMessageApi { color, fields }
    }
}

/// Matches the SlackAPIs message fields
#[derive(Debug, Serialize)]
pub struct SlackMessageField {
    title: String,
    value: String,
}

impl SlackMessageField {
    pub fn new(title: String, value: String) -> Self {
        Self { title, value }
    }
}
