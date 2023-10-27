use crate::{Notification, Notify};
use async_trait::async_trait;
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
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
        debug!("Posting message to Slack");

        let color = notification.level.as_hex_color();
        let fields = vec![SlackMessageField::new(
            &notification.head,
            format!(
                "{}\n{}",
                notification.row_1,
                notification.row_2.as_deref().unwrap_or_default(),
            ),
        )];
        let msg = SlackMessageApi::new(color, fields);
        debug!("{:?}", msg);

        match Notification::client()
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
    pub fields: Vec<SlackMessageField<'a>>,
}

impl SlackMessageApi<'_> {
    pub fn new<'a>(color: &'a str, fields: Vec<SlackMessageField<'a>>) -> SlackMessageApi<'a> {
        SlackMessageApi { color, fields }
    }
}

/// Matches the SlackAPIs message fields
#[derive(Debug, Serialize)]
pub struct SlackMessageField<'a> {
    title: &'a str,
    value: String,
}

impl SlackMessageField<'_> {
    pub fn new(title: &str, value: String) -> SlackMessageField {
        SlackMessageField { title, value }
    }
}
