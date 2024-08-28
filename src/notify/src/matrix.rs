use async_trait::async_trait;
use rauthy_common::constants::RAUTHY_VERSION;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use ruma::client::http_client;
use ruma::events::room::message::RoomMessageEventContent;
use ruma::{OwnedDeviceId, OwnedRoomId, OwnedUserId, TransactionId};
use std::str::FromStr;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time;
use tracing::{debug, error, info, warn};

use crate::{Notification, Notify};

#[derive(Debug)]
pub struct NotifierMatrix {
    user_id: OwnedUserId,
    room_id: OwnedRoomId,
    user_password: Option<String>,
    device_id: OwnedDeviceId,

    client: ruma::Client<http_client::Reqwest>,
    access_token: Mutex<Option<String>>,
    refresh_token: Mutex<Option<String>>,
}

impl NotifierMatrix {
    pub async fn try_new(
        server_url: String,
        user_id: &str,
        room_id: &str,
        access_token: Option<String>,
        user_password: Option<String>,
        disable_tls_validation: bool,
        root_ca_path: Option<&str>,
    ) -> Result<Self, ErrorResponse> {
        // init variables
        let user_id = OwnedUserId::from_str(user_id).map_err(|err| {
            ErrorResponse::new(
                ErrorResponseType::Internal,
                format!("Cannot parse the EVENT_MATRIX_USER_ID: {:?}", err),
            )
        })?;
        let room_id = OwnedRoomId::from_str(room_id).map_err(|err| {
            ErrorResponse::new(
                ErrorResponseType::Internal,
                format!("Cannot parse the EVENT_MATRIX_ROOM_ID: {:?}", err),
            )
        })?;

        let rauthy_notifier = format!("Rauthy v{} Notifier", RAUTHY_VERSION);
        let device_id = OwnedDeviceId::from(rauthy_notifier.as_str());

        let http_client = Notification::build_client(disable_tls_validation, root_ca_path).await;

        let client = ruma::Client::builder()
            .homeserver_url(server_url.to_string())
            .access_token(access_token.clone())
            .http_client(http_client)
            .await?;

        if disable_tls_validation {
            warn!("Matrix Event Notifier client has TLS validation disabled");
        };

        let slf = Self {
            user_id,
            room_id,
            user_password,
            device_id,

            client,
            access_token: Mutex::new(access_token),
            refresh_token: Mutex::new(None),
        };

        // try login and check access and config
        let mut retry = 0;
        let limit = 3;
        while retry < limit {
            match slf.login().await {
                Ok(_) => break,
                Err(err) => {
                    if retry < limit {
                        warn!("{:?} - retry in 5 seconds", err);
                        retry += 1;
                        time::sleep(Duration::from_secs(5)).await;
                    } else {
                        return Err(err);
                    }
                }
            }
        }
        slf.logged_in_check_sync().await?;
        slf.check_room_config().await?;

        Ok(slf)
    }

    async fn login(&self) -> Result<(), ErrorResponse> {
        // check if we maybe are logged in already
        if self.logged_in_check_sync().await.is_ok() {
            return Ok(());
        }

        let refresh_token = self.refresh_token.lock().await.clone();
        if let Some(token) = refresh_token {
            let res = self
                .client
                .send_request(ruma::api::client::session::refresh_token::v3::Request::new(
                    token,
                ))
                .await?;

            {
                let mut lock = self.access_token.lock().await;
                *lock = Some(res.access_token);
            }
            {
                let mut lock = self.refresh_token.lock().await;
                *lock = res.refresh_token;
            }

            if self.logged_in_check_sync().await.is_ok() {
                return Ok(());
            }
        }

        if self.user_password.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::Connection,
                "Cannot connect to Matrix with auth token and EVENT_MATRIX_USER_PASSWORD is empty",
            ));
        }

        let session = self
            .client
            .log_in(
                self.user_id.as_str(),
                self.user_password.as_deref().unwrap_or_default(),
                Some(&self.device_id),
                Some("Rauthy Events"),
            )
            .await?;

        {
            let mut lock = self.access_token.lock().await;
            *lock = Some(session.access_token);
        }
        {
            let mut lock = self.refresh_token.lock().await;
            *lock = session.refresh_token;
        }

        if self.logged_in_check_sync().await.is_ok() {
            return Ok(());
        }

        Err(ErrorResponse::new(
            ErrorResponseType::Connection,
            "Unable to Login to Matrix".to_string(),
        ))
    }

    async fn logged_in_check_sync(&self) -> Result<(), ErrorResponse> {
        let resp = self
            .client
            .send_request(ruma::api::client::account::whoami::v3::Request::new())
            .await
            .map_err(|err| {
                ErrorResponse::new(
                    ErrorResponseType::Connection,
                    format!("Matrix Notifications Client is not logged in: {:?}", err),
                )
            })?;

        if resp.user_id != self.user_id {
            return Err(ErrorResponse::new(
                ErrorResponseType::Connection,
                "Matrix Notifications Client is not logged in".to_string(),
            ));
        };

        debug!("logged in to Matrix");
        Ok(())
    }

    async fn check_room_config(&self) -> Result<(), ErrorResponse> {
        let resp = self
            .client
            .send_request(ruma::api::client::membership::joined_rooms::v3::Request::new())
            .await?;

        debug!(
            "Looking for Matrix room id {} in joined rooms: {:?}",
            self.room_id, resp.joined_rooms
        );
        let room = resp.joined_rooms.into_iter().find(|id| id == &self.room_id);

        if let Some(room) = room {
            assert_eq!(room, self.room_id);
            debug!("Matrix Notification Room found");
        } else {
            warn!("Not joined to the Matrix Notification Room. Trying to join now");
            let resp = self
                .client
                .send_request(
                    ruma::api::client::membership::join_room_by_id::v3::Request::new(
                        self.room_id.clone(),
                    ),
                )
                .await?;
            assert_eq!(resp.room_id, self.room_id);
        }

        Ok(())
    }
}

#[async_trait]
impl Notify for NotifierMatrix {
    async fn notify(&self, notification: &Notification) -> Result<(), ErrorResponse> {
        debug!("Sending message to Matrix");

        let send_msg = || {
            let msg = if let Some(row_2) = &notification.row_2 {
                format!(
                    "**{}**\n{}\n{}",
                    notification.head, notification.row_1, row_2
                )
            } else {
                format!("**{}**\n{}", notification.head, notification.row_1)
            };

            self.client.send_request(
                ruma::api::client::message::send_message_event::v3::Request::new(
                    self.room_id.clone(),
                    TransactionId::new(),
                    &RoomMessageEventContent::text_markdown(msg),
                )
                .expect("Matrix message to always build fine"),
            )
        };

        match send_msg().await {
            Ok(res) => {
                debug!("Successfully pushed matrix event with id {}", res.event_id);
                Ok(())
            }
            Err(err) => {
                error!("Error pushing matrix event: {:?}", err);

                let retries = 3;
                for i in 1..=3 {
                    match self.login().await {
                        Ok(_) => {
                            info!("Logged in to Matrix again");
                            break;
                        }
                        Err(err) => {
                            if i == retries {
                                return Err(ErrorResponse::new(
                                    ErrorResponseType::Connection,
                                    format!("Unable to log in to Matrix after {} retries", retries),
                                ));
                            }

                            error!("Error logging in to Matrix on {}. attempt: {:?}\nRetrying in 3 seconds", i, err);
                            time::sleep(Duration::from_secs(3)).await;
                            continue;
                        }
                    }
                }

                match send_msg().await {
                    Ok(_) => Ok(()),
                    Err(err) => Err(ErrorResponse::new(
                        ErrorResponseType::Connection,
                        format!("Unable to post message to matrix after re-login: {}", err),
                    )),
                }
            }
        }
    }
}
