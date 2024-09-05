use crate::email;
use crate::email::EMail;
use crate::events::event::{Event, EventLevel, EventType};
use async_trait::async_trait;
use rauthy_error::ErrorResponse;
use rauthy_notify::matrix::NotifierMatrix;
use rauthy_notify::slack::NotifierSlack;
use rauthy_notify::{Notification, Notify};
use std::env;
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
            EventLevel::Info | EventLevel::Notice => info!("{}", event),
            EventLevel::Warning => warn!("{}", event),
            EventLevel::Critical => warn!("{}", event),
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

        if let Some((level, notifier)) = NOTIFIER_MATRIX.get() {
            if event.typ == EventType::Test || &event.level.value() >= level {
                if let Err(err) = notifier.notify(&notification).await {
                    error!("sending Event via Matrix Notifier: {:?}", err);
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

    pub async fn init_notifiers(tx_email: mpsc::Sender<EMail>) -> Result<(), ErrorResponse> {
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

        // Matrix
        if let Ok(user_id) = env::var("EVENT_MATRIX_USER_ID") {
            let level = env::var("EVENT_NOTIFY_LEVEL_MATRIX")
                .map(|level| {
                    level.parse::<EventLevel>().expect(
                        "Cannot parse EVENT_NOTIFY_LEVEL_MATRIX. Possible values: info, notice, warning, critical",
                    )
                })
                .unwrap_or(EventLevel::Notice);

            let server_url = env::var("EVENT_MATRIX_SERVER_URL")
                .unwrap_or_else(|_| "https://matrix-client.matrix.org".to_string());

            let room_id = env::var("EVENT_MATRIX_ROOM_ID")
                .expect("EVENT_MATRIX_USER_ID is given but no EVENT_MATRIX_ROOM_ID");

            let access_token = env::var("EVENT_MATRIX_ACCESS_TOKEN").ok();
            let user_password = env::var("EVENT_MATRIX_USER_PASSWORD").ok();

            if access_token.is_none() && user_password.is_none() {
                panic!("Specific one of: EVENT_MATRIX_ACCESS_TOKEN or EVENT_MATRIX_USER_PASSWORD");
            }

            let disable_tls_validation = env::var("EVENT_MATRIX_DANGER_DISABLE_TLS_VALIDATION")
                .unwrap_or_else(|_| "false".to_string())
                .parse::<bool>()
                .expect("Cannot parse EVENT_MATRIX_DANGER_DISABLE_TLS_VALIDATION to bool");
            let root_ca_path = env::var("EVENT_MATRIX_ROOT_CA_PATH").ok();

            match NotifierMatrix::try_new(
                server_url,
                &user_id,
                &room_id,
                access_token,
                user_password,
                disable_tls_validation,
                root_ca_path.as_deref(),
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
                    let no_panic = env::var("EVENT_MATRIX_ERROR_NO_PANIC")
                        .unwrap_or_else(|_| "false".to_string())
                        .parse::<bool>()
                        .expect("Cannot parse EVENT_MATRIX_ERROR_NO_PANIC to bool");

                    let msg = format!("Error creating the Matrix Notifier: {:?}", err.message);
                    if no_panic {
                        error!("{msg}");
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
