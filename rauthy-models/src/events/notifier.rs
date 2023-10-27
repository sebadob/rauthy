use crate::email;
use crate::email::EMail;
use crate::events::event::{Event, EventLevel, EventType};
use async_trait::async_trait;
use rauthy_common::constants::DEV_MODE;
use rauthy_common::error_response::ErrorResponse;
use rauthy_notify::slack::NotifierSlack;
use rauthy_notify::{Notification, Notify};
use std::env;
use std::sync::OnceLock;
use tokio::sync::mpsc;
use tracing::{error, info, warn};

static NOTIFIER_EMAIL: OnceLock<(i16, NotifierEmail)> = OnceLock::new();
static NOTIFIER_SLACK: OnceLock<(i16, NotifierSlack)> = OnceLock::new();

pub struct EventNotifier;

impl EventNotifier {
    pub async fn send(event: &Event) -> Result<(), ErrorResponse> {
        match event.level {
            EventLevel::Info | EventLevel::Notice => info!("{}", event),
            EventLevel::Warning => warn!("{}", event),
            EventLevel::Critical => warn!("{}", event),
        }

        if *DEV_MODE {
            warn!(
                "Running in DEV_MODE -> skipping sending out events. New Event to be sent:\n{:?}",
                event
            );
            return Ok(());
        }

        let notification = Notification::from(event);

        // if there will be more notifier targets in the future, a macro may make sense for a
        // cleaner code

        if let Some((level, notifier)) = NOTIFIER_EMAIL.get() {
            if event.typ == EventType::Test || &event.level.value() >= level {
                if let Err(err) = notifier.notify(&notification).await {
                    error!("sending Event via E-Mail Notifier: {:?}", err);
                    // TODO implement some retry mechanism
                }
            }
        }

        if let Some((level, notifier)) = NOTIFIER_SLACK.get() {
            if event.typ == EventType::Test || &event.level.value() >= level {
                if let Err(err) = notifier.notify(&notification).await {
                    error!("sending Event via Slack Notifier: {:?}", err);
                    // TODO implement some retry mechanism
                }
            }
        }

        Ok(())
    }

    pub fn init_notifiers(tx_email: mpsc::Sender<EMail>) -> Result<(), ErrorResponse> {
        // E-Mail
        if let Ok(email) = env::var("EVENT_EMAIL") {
            let level = env::var("EVENT_NOTIFY_LEVEL_EMAIL")
                .map(|level| {
                    level.parse::<EventLevel>().expect(
                        "Cannot parse EVENT_NOTIFY_LEVEL_EMAIL. Possible values: info, notice, warning, critical",
                    )
                })
                .unwrap_or(EventLevel::Warning);
            info!(
                "E-Mail Event Notification's will be sent to {} with level: {:?}",
                email, level
            );

            let notifier = NotifierEmail {
                notification_email: email,
                tx_email,
            };
            NOTIFIER_EMAIL
                .set((level.value(), notifier))
                .expect("init_notifiers should only be called once");
        }

        // Slack
        if let Ok(url) = env::var("EVENT_SLACK_WEBHOOK") {
            let level = env::var("EVENT_NOTIFY_LEVEL_SLACK")
                .map(|level| {
                    level.parse::<EventLevel>().expect(
                        "Cannot parse EVENT_NOTIFY_LEVEL_SLACK. Possible values: info, notice, warning, critical",
                    )
                })
                .unwrap_or(EventLevel::Notice);
            info!(
                "Event Notification's will be sent to Slack with level: {:?}",
                level
            );

            let notifier = NotifierSlack::new(url);
            NOTIFIER_SLACK
                .set((level.value(), notifier))
                .expect("init_notifiers should only be called once");
        }

        // TODO Matrix

        Ok(())
    }
}

#[derive(Debug)]
struct NotifierEmail {
    notification_email: String,
    tx_email: mpsc::Sender<EMail>,
}

#[async_trait]
impl Notify for NotifierEmail {
    async fn notify(&self, notification: &Notification) -> Result<(), ErrorResponse> {
        email::send_email_notification(
            self.notification_email.clone(),
            &self.tx_email,
            notification,
        )
        .await;
        Ok(())
    }
}
