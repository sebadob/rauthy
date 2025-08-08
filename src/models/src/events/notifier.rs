use crate::email;
use crate::email::EMail;
use crate::events::event::{Event, EventLevel, EventType};
use crate::rauthy_config::RauthyConfig;
use async_trait::async_trait;
use rauthy_error::ErrorResponse;
use rauthy_notify::matrix::NotifierMatrix;
use rauthy_notify::slack::NotifierSlack;
use rauthy_notify::{Notification, Notify};
use std::sync::OnceLock;
use tokio::sync::mpsc;
use tracing::{error, info, warn};

static NOTIFIER_EMAIL: OnceLock<(i16, NotifierEmail)> = OnceLock::new();
static NOTIFIER_MATRIX: OnceLock<(i16, NotifierMatrix)> = OnceLock::new();
static NOTIFIER_SLACK: OnceLock<(i16, NotifierSlack)> = OnceLock::new();

pub struct EventNotifier;

impl EventNotifier {
    /// Sends an `Event` to configured notifiers like for instance E-Mail or Matrix.
    pub async fn send(event: &Event) -> Result<(), ErrorResponse> {
        match event.level {
            EventLevel::Info | EventLevel::Notice => info!("{event}"),
            EventLevel::Warning => warn!("{event}"),
            EventLevel::Critical => warn!("{event}"),
        }

        let notification = Notification::from(event);

        // if there will be more notifier targets in the future, a macro may make sense for a
        // cleaner code

        if let Some((level, notifier)) = NOTIFIER_EMAIL.get()
            && (event.typ == EventType::Test || &event.level.value() >= level)
            && let Err(err) = notifier.notify(&notification).await
        {
            error!(?err, "sending Event via E-Mail Notifier");
            // TODO implement some retry mechanism
        }

        if let Some((level, notifier)) = NOTIFIER_MATRIX.get()
            && (event.typ == EventType::Test || &event.level.value() >= level)
            && let Err(err) = notifier.notify(&notification).await
        {
            error!(?err, "sending Event via Matrix Notifier");
            // TODO implement some retry mechanism
        }

        if let Some((level, notifier)) = NOTIFIER_SLACK.get()
            && (event.typ == EventType::Test || &event.level.value() >= level)
            && let Err(err) = notifier.notify(&notification).await
        {
            error!(?err, "sending Event via Slack Notifier");
            // TODO implement some retry mechanism
        }

        Ok(())
    }

    pub async fn init_notifiers(tx_email: mpsc::Sender<EMail>) -> Result<(), ErrorResponse> {
        let vars = &RauthyConfig::get().vars.events;

        // E-Mail
        if let Some(email) = vars.email.clone() {
            let level = vars.notify_level_email.clone();
            info!("E-Mail Event Notification's will be sent to {email} with level: {level:?}");

            let notifier = NotifierEmail {
                notification_recipient_name: "Rauthy Admin".to_string(),
                notification_email: email,
                tx_email,
            };
            NOTIFIER_EMAIL
                .set((level.value(), notifier))
                .expect("init_notifiers should only be called once");
        }

        // Slack
        if let Some(url) = vars.slack_webhook.clone() {
            let level = vars.notify_level_slack.clone();
            info!("Event Notification's will be sent to Slack with level: {level:?}");

            let notifier = NotifierSlack::new(url);
            NOTIFIER_SLACK
                .set((level.value(), notifier))
                .expect("init_notifiers should only be called once");
        }

        // Matrix
        if let Some(user_id) = vars.matrix_user_id.as_ref() {
            let level = vars.notify_level_matrix.clone();
            let server_url = vars.matrix_server_url.to_string();
            let room_id = vars
                .matrix_room_id
                .as_ref()
                .expect("`event.matrix_room_id` not set");

            let access_token = vars.matrix_access_token.clone();
            let user_password = vars.matrix_user_password.clone();
            if access_token.is_none() && user_password.is_none() {
                panic!(
                    "Specific one of: `event.matrix_access_token` or `event.matrix_user_password`"
                );
            }

            match NotifierMatrix::try_new(
                server_url,
                user_id,
                room_id,
                access_token,
                user_password,
                vars.matrix_danger_disable_tls_validation,
                vars.matrix_root_ca_path.as_deref(),
            )
            .await
            {
                Ok(notifier) => {
                    NOTIFIER_MATRIX
                        .set((level.value(), notifier))
                        .expect("init_notifiers should only be called once");

                    info!("Event Notifications will be sent to Matrix");
                }
                Err(err) => {
                    let msg = format!("Error creating the Matrix Notifier: {:?}", err.message);
                    if vars.matrix_error_no_panic {
                        error!(error = msg);
                    } else {
                        panic!("{msg}");
                    }
                }
            };
        }

        Ok(())
    }
}

#[derive(Debug)]
struct NotifierEmail {
    notification_recipient_name: String,
    notification_email: String,
    tx_email: mpsc::Sender<EMail>,
}

#[async_trait]
impl Notify for NotifierEmail {
    async fn notify(&self, notification: &Notification) -> Result<(), ErrorResponse> {
        email::send_email_notification(
            self.notification_recipient_name.clone(),
            self.notification_email.clone(),
            &self.tx_email,
            notification,
        )
        .await;
        Ok(())
    }
}
