use crate::app_state::AppState;
use crate::cache::{Cache, DB};
use crate::entity::refresh_tokens_devices::RefreshTokenDevice;
use actix_web::web;
use chrono::{DateTime, Utc};
use rauthy_api_types::users::DeviceResponse;
use rauthy_common::constants::{
    CACHE_TTL_DEVICE_CODE, DEVICE_GRANT_CODE_LIFETIME, DEVICE_GRANT_USER_CODE_LENGTH,
    DEVICE_KEY_LENGTH, PUB_URL_WITH_SCHEME,
};
use rauthy_common::utils::get_rand;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow};
use std::ops::{Add, Sub};
use tracing::info;

#[derive(Debug, FromRow)]
pub struct DeviceEntity {
    pub id: String,
    pub client_id: String,
    pub user_id: Option<String>,
    pub created: i64,
    pub access_exp: i64,
    pub refresh_exp: Option<i64>,
    pub peer_ip: String,
    pub name: String,
}

impl DeviceEntity {
    pub async fn insert(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        query!(
            r#"INSERT INTO devices
            (id, client_id, user_id, created, access_exp, refresh_exp, peer_ip, name)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
            self.id,
            self.client_id,
            self.user_id,
            self.created,
            self.access_exp,
            self.refresh_exp,
            self.peer_ip,
            self.name,
        )
        .execute(&data.db)
        .await?;
        Ok(())
    }

    pub async fn find(data: &web::Data<AppState>, id: &str) -> Result<Self, ErrorResponse> {
        let slf = query_as!(Self, "SELECT * FROM devices WHERE id = $1", id)
            .fetch_one(&data.db)
            .await?;
        Ok(slf)
    }

    pub async fn find_for_user(
        data: &web::Data<AppState>,
        user_id: &str,
    ) -> Result<Vec<Self>, ErrorResponse> {
        let res = query_as!(Self, "SELECT * FROM devices WHERE user_id = $1", user_id)
            .fetch_all(&data.db)
            .await?;
        Ok(res)
    }

    /// Deletes all devices where access and refresh token expirations are in the past
    pub async fn delete_expired(data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        let exp = Utc::now()
            .sub(chrono::Duration::try_hours(1).unwrap())
            .timestamp();

        let res = query!(
            r#"DELETE FROM devices
            WHERE access_exp < $1 AND (refresh_exp < $1 OR refresh_exp is null)"#,
            exp
        )
        .execute(&data.db)
        .await?;
        info!("Cleaned up {} expires devices", res.rows_affected());

        Ok(())
    }

    pub async fn invalidate(data: &web::Data<AppState>, id: &str) -> Result<(), ErrorResponse> {
        query!("DELETE FROM devices WHERE id = $1", id)
            .execute(&data.db)
            .await?;
        // we don't need to manually clean up refresh_tokens because of FK cascades
        Ok(())
    }

    pub async fn revoke_refresh_tokens(
        data: &web::Data<AppState>,
        device_id: &str,
    ) -> Result<(), ErrorResponse> {
        RefreshTokenDevice::invalidate_all_for_device(data, device_id).await?;
        query!(
            "UPDATE devices SET refresh_exp = null WHERE id = $1",
            device_id,
        )
        .execute(&data.db)
        .await?;
        Ok(())
    }

    pub async fn update_name(
        data: &web::Data<AppState>,
        device_id: &str,
        user_id: &str,
        name: &str,
    ) -> Result<(), ErrorResponse> {
        query!(
            "UPDATE devices SET name = $1 WHERE id = $2 AND user_id = $3",
            name,
            device_id,
            user_id
        )
        .execute(&data.db)
        .await?;
        Ok(())
    }
}

impl From<DeviceEntity> for DeviceResponse {
    fn from(value: DeviceEntity) -> Self {
        Self {
            id: value.id,
            client_id: value.client_id,
            user_id: value.user_id,
            created: value.created,
            access_exp: value.access_exp,
            refresh_exp: value.refresh_exp,
            peer_ip: value.peer_ip,
            name: value.name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAuthCode {
    pub client_id: String,
    pub device_code: String,
    /// Will be Some(user_id) once a user has been validated the auth request
    pub verified_by: Option<String>,
    /// We need the additional `exp` here because a verification from a
    /// user will reset the lifetime, which means without the additional
    /// check here, it could be possible that a code lives longer than
    /// allowed.
    pub exp: DateTime<Utc>,
    pub last_poll: DateTime<Utc>,
    pub scopes: Option<String>,
    // saved additionally here to have fewer cache requests during client polling
    pub client_secret: Option<String>,
    // The warnings counter will increase, if a client does not stick to
    // the given interval and gets 'slow_down' from us. If this happens
    // too many times, the IP will be blacklisted.1
    pub warnings: u8,
}

impl DeviceAuthCode {
    /// DeviceAuthCode's live inside the cache only
    pub async fn new(
        scopes: Option<String>,
        client_id: String,
        client_secret: Option<String>,
    ) -> Result<Self, ErrorResponse> {
        let now = Utc::now();
        let exp = now.add(chrono::Duration::seconds(
            *DEVICE_GRANT_CODE_LIFETIME as i64,
        ));
        let slf = Self {
            client_id,
            device_code: get_rand(DEVICE_KEY_LENGTH as usize),
            verified_by: None,
            exp,
            last_poll: now,
            scopes,
            client_secret,
            warnings: 0,
        };

        DB::client()
            .put(
                Cache::DeviceCode,
                slf.user_code().to_string(),
                &slf,
                *CACHE_TTL_DEVICE_CODE,
            )
            .await?;

        Ok(slf)
    }

    pub async fn find_by_device_code(device_code: &str) -> Result<Option<Self>, ErrorResponse> {
        let key = &device_code[..(*DEVICE_GRANT_USER_CODE_LENGTH as usize)];
        Self::find(key.to_string()).await
    }

    pub async fn find(user_code: String) -> Result<Option<Self>, ErrorResponse> {
        let slf: Option<Self> = DB::client().get(Cache::DeviceCode, user_code).await?;
        match slf {
            None => Ok(None),
            Some(slf) => {
                if slf.exp < Utc::now() {
                    slf.delete().await?;
                    Ok(None)
                } else {
                    Ok(Some(slf))
                }
            }
        }
    }

    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        DB::client()
            .delete(Cache::DeviceCode, self.user_code().to_string())
            .await?;
        Ok(())
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        DB::client()
            .put(
                Cache::DeviceCode,
                self.user_code().to_string(),
                self,
                *CACHE_TTL_DEVICE_CODE,
            )
            .await?;
        Ok(())
    }
}

impl DeviceAuthCode {
    /// Validates the given `user_code`
    #[inline]
    pub fn user_code(&self) -> &str {
        &self.device_code[..(*DEVICE_GRANT_USER_CODE_LENGTH as usize)]
    }

    pub fn verification_uri(&self) -> String {
        // TODO config var if we should host at / as well for better UX ?
        format!("{}/auth/v1/device", *PUB_URL_WITH_SCHEME)
    }

    pub fn verification_uri_complete(&self) -> String {
        format!(
            "{}/auth/v1/device?code={}",
            *PUB_URL_WITH_SCHEME,
            self.user_code()
        )
    }
}
