// Rauthy - OpenID Connect and Single Sign-On Identity & Access Management
// Copyright (C) 2023 Sebastian Dobe <sebastiandobe@mailbox.org>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use async_trait::async_trait;
use rauthy_common::constants::RAUTHY_VERSION;
use rauthy_common::error_response::ErrorResponse;
use reqwest::tls;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use std::time::Duration;

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
    pub fn client() -> &'static reqwest::Client {
        HTTP_CLIENT.get_or_init(|| {
            reqwest::Client::builder()
                .connect_timeout(Duration::from_secs(10))
                .timeout(Duration::from_secs(10))
                .user_agent(format!("Rauthy v{} Notifier", RAUTHY_VERSION))
                .min_tls_version(tls::Version::TLS_1_2)
                .pool_idle_timeout(Duration::from_secs(300))
                .build()
                .unwrap()
        })
    }
}

#[async_trait]
pub trait Notify {
    // TODO think about adding smt like notify_all with batch messages
    async fn notify(&self, notification: &Notification) -> Result<(), ErrorResponse>;
}
