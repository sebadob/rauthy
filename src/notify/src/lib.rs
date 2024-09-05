// Copyright 2024 Sebastian Dobe <sebastiandobe@mailbox.org>

#![forbid(unsafe_code)]

use async_trait::async_trait;
use rauthy_common::constants::RAUTHY_VERSION;
use rauthy_error::ErrorResponse;
use reqwest::tls;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use std::time::Duration;
use tracing::info;

pub mod matrix;
pub mod slack;

static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NotificationLevel {
    Info,
    Notice,
    Warning,
    Critical,
}

impl NotificationLevel {
    pub fn as_hex_color(&self) -> &str {
        match self {
            NotificationLevel::Info => "#388c51",
            NotificationLevel::Notice => "#3d5d99",
            NotificationLevel::Warning => "#c29a4f",
            NotificationLevel::Critical => "#993d49",
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            NotificationLevel::Info => "INFO",
            NotificationLevel::Notice => "NOTICE",
            NotificationLevel::Warning => "WARNING",
            NotificationLevel::Critical => "CRITICAL",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub level: NotificationLevel,
    pub head: String,
    pub row_1: String,
    pub row_2: Option<String>,
}

impl Notification {
    pub async fn client() -> &'static reqwest::Client {
        if let Some(client) = HTTP_CLIENT.get() {
            client
        } else {
            let client = Self::build_client(false, None).await;
            let _ = HTTP_CLIENT.set(client);
            HTTP_CLIENT.get().unwrap()
        }
    }

    pub async fn build_client(
        disable_tls_validation: bool,
        root_ca_path: Option<&str>,
    ) -> reqwest::Client {
        let mut builder = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(10))
            .user_agent(format!("Rauthy v{} Notifier", RAUTHY_VERSION))
            .min_tls_version(tls::Version::TLS_1_2)
            .pool_idle_timeout(Duration::from_secs(600))
            .danger_accept_invalid_certs(disable_tls_validation);

        if disable_tls_validation {
            builder = builder.danger_accept_invalid_certs(true);
        }

        if let Some(path) = root_ca_path {
            let pem_file = tokio::fs::read(path)
                .await
                .expect("Cannot read Event Notifier Root CA PEM file");
            let root_cert = reqwest::tls::Certificate::from_pem(pem_file.as_slice())
                .expect("Cannot build Root TLS from Event Notifier Root CA PEM file");
            builder = builder.add_root_certificate(root_cert);

            info!(
                "Custom Root CA from {} was added to the Event Notifier Client",
                path
            );
        }

        builder.build().unwrap()
    }
}

#[async_trait]
pub trait Notify {
    async fn notify(&self, notification: &Notification) -> Result<(), ErrorResponse>;
}
