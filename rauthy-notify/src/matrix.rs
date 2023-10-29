use crate::{Notification, Notify};
use async_trait::async_trait;
use matrix_sdk::config::SyncSettings;
use matrix_sdk::matrix_auth::{MatrixSession, MatrixSessionTokens};
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use matrix_sdk::ruma::{OwnedDeviceId, OwnedRoomId, OwnedUserId};
use matrix_sdk::{AuthSession, Client, RoomState, SessionMeta};
use rauthy_common::constants::RAUTHY_VERSION;
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use std::str::FromStr;
use std::time::Duration;
use tokio::time;
use tracing::{debug, error, info, warn};

#[derive(Debug)]
pub struct NotifierMatrix {
    user_id: OwnedUserId,
    room_id: OwnedRoomId,
    access_token: Option<String>,
    user_password: Option<String>,
    device_id: OwnedDeviceId,
    client: Client,
}

impl NotifierMatrix {
    pub async fn try_new(
        user_id: &str,
        room_id: &str,
        access_token: Option<String>,
        user_password: Option<String>,
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

        let client = Client::builder()
            .server_name(user_id.server_name())
            .user_agent(rauthy_notifier)
            .handle_refresh_tokens()
            .disable_ssl_verification() // TODO enable again afterwards
            .build()
            .await
            .map_err(|err| {
                ErrorResponse::new(
                    ErrorResponseType::Internal,
                    format!("Cannot build Matrix Notification Client: {:?}", err),
                )
            })?;

        // build self
        let slf = Self {
            user_id,
            room_id,
            access_token,
            user_password,
            device_id,
            client,
        };

        // try login and check access and config
        let mut retry = 0;
        while retry < 3 {
            match slf.login().await {
                Ok(_) => break,
                Err(err) => {
                    if retry < 3 {
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

        let mut rx = slf.client.subscribe_to_session_changes();
        tokio::spawn(async move {
            while let Ok(state_change) = rx.recv().await {
                info!("Matrix Session State change: {:?}", state_change);
                // TODO do we need to monitor these and re-login manually, or does the client do it already?
            }
        });

        // all good
        Ok(slf)
    }

    async fn login(&self) -> Result<(), ErrorResponse> {
        // check if we maybe are logged in already
        if self.logged_in_check_sync().await.is_ok() {
            return Ok(());
        }

        if let Some(_session) = self.client.session() {
            if self.logged_in_check_sync().await.is_ok() {
                return Ok(());
            }
        } else if let Some(access_token) = &self.access_token {
            let session = AuthSession::Matrix(MatrixSession {
                meta: SessionMeta {
                    user_id: self.user_id.clone(),
                    device_id: self.device_id.clone(),
                },
                tokens: MatrixSessionTokens {
                    access_token: access_token.clone(),
                    refresh_token: None,
                },
            });

            if self.client.restore_session(session).await.is_ok()
                && self.logged_in_check_sync().await.is_ok()
            {
                return Ok(());
            }
        } else if let Some(password) = &self.user_password {
            if self
                .client
                .matrix_auth()
                .login_username(self.user_id.clone(), password)
                .await
                .is_ok()
                && self.logged_in_check_sync().await.is_ok()
            {
                return Ok(());
            }
        }

        Err(ErrorResponse::new(
            ErrorResponseType::Connection,
            "Unable to Login to Matrix".to_string(),
        ))
    }

    async fn logged_in_check_sync(&self) -> Result<(), ErrorResponse> {
        if self.client.logged_in() && self.client.sync_once(SyncSettings::default()).await.is_ok() {
            info!("Successfully logged in to matrix");
            Ok(())
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Connection,
                "Matrix Notifications Client is not logged in".to_string(),
            ))
        }
    }

    async fn check_room_config(&self) -> Result<(), ErrorResponse> {
        if let Some(room) = self.client.get_room(&self.room_id) {
            debug!("Matrix Notification Room found");

            if room.state() == RoomState::Joined {
                return Ok(());
            }

            warn!("Not joined to the Matrix Notification Room. Trying to join now");
            if room.join().await.is_ok() {
                return Ok(());
            } else {
                panic!("Could not join the Matrix Notification Room");
            }
        }

        Err(ErrorResponse::new(
            ErrorResponseType::Connection,
            "Matrix Notifications Room could not be found - check EVENT_MATRIX_ROOM_ID".to_string(),
        ))
    }
}

#[async_trait]
impl Notify for NotifierMatrix {
    async fn notify(&self, notification: &Notification) -> Result<(), ErrorResponse> {
        debug!("Sending message to Matrix");

        let build_msg = || {
            let msg = if let Some(row_2) = &notification.row_2 {
                format!(
                    "**{}**\n{}\n{}",
                    notification.head, notification.row_1, row_2
                )
            } else {
                format!("**{}**\n{}", notification.head, notification.row_1)
            };
            RoomMessageEventContent::text_markdown(msg.clone())
        };

        if let Some(room) = self.client.get_room(&self.room_id) {
            if let Err(err) = room.send(build_msg(), None).await {
                error!("Sending Matrix Notification: {:?} - retrying once", err);

                tokio::time::sleep(Duration::from_secs(1)).await;

                if self.logged_in_check_sync().await.is_err() {
                    error!(
                        "Matrix Notification Client is not logged in anymore when it was.\
                        This should never happen."
                    );
                    if let Err(err) = self.logged_in_check_sync().await {
                        error!("Unable to log back in again to Matrix: {:?}", err)
                    } else {
                        let _ = self.logged_in_check_sync().await;
                        room.send(build_msg(), None).await.map_err(|err| {
                            ErrorResponse::new(
                                ErrorResponseType::Connection,
                                format!("Matrix connection error: {:?}", err),
                            )
                        })?;
                    }
                }
            }

            Ok(())
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Connection,
                "Matrix Notification Room went away - check your config".to_string(),
            ))
        }
    }
}
