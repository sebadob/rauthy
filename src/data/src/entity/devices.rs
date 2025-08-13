use crate::database::{Cache, DB};
use crate::entity::refresh_tokens_devices::RefreshTokenDevice;
use chrono::{DateTime, Utc};
use hiqlite_macros::params;
use rauthy_api_types::users::DeviceResponse;
use rauthy_common::constants::DEVICE_KEY_LENGTH;
use rauthy_common::is_hiqlite;
use rauthy_common::utils::get_rand;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

use crate::rauthy_config::RauthyConfig;
use tracing::info;

#[derive(Debug, Deserialize)]
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

impl From<tokio_postgres::Row> for DeviceEntity {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            client_id: row.get("client_id"),
            user_id: row.get("user_id"),
            created: row.get("created"),
            access_exp: row.get("access_exp"),
            refresh_exp: row.get("refresh_exp"),
            peer_ip: row.get("peer_ip"),
            name: row.get("name"),
        }
    }
}

impl DeviceEntity {
    pub async fn insert(self) -> Result<(), ErrorResponse> {
        let sql = r#"
INSERT INTO devices
(id, client_id, user_id, created, access_exp, refresh_exp, peer_ip, name)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        self.id,
                        self.client_id,
                        self.user_id,
                        self.created,
                        self.access_exp,
                        self.refresh_exp,
                        self.peer_ip,
                        self.name
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &self.id,
                    &self.client_id,
                    &self.user_id,
                    &self.created,
                    &self.access_exp,
                    &self.refresh_exp,
                    &self.peer_ip,
                    &self.name,
                ],
            )
            .await?;
        }

        Ok(())
    }

    pub async fn find(id: &str) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM devices WHERE id = $1";
        let slf = if is_hiqlite() {
            DB::hql().query_as_one(sql, params!(id)).await?
        } else {
            DB::pg_query_one(sql, &[&id]).await?
        };
        Ok(slf)
    }

    pub async fn find_for_user(user_id: &str) -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM devices WHERE user_id = $1";
        let res = if is_hiqlite() {
            DB::hql().query_as(sql, params!(user_id)).await?
        } else {
            DB::pg_query(sql, &[&user_id], 0).await?
        };
        Ok(res)
    }

    /// Deletes all devices where access and refresh token expirations are in the past
    pub async fn delete_expired() -> Result<(), ErrorResponse> {
        let exp = Utc::now()
            .sub(chrono::Duration::try_hours(1).unwrap())
            .timestamp();

        let sql = r#"
DELETE FROM devices
WHERE access_exp < $1 AND (refresh_exp < $1 OR refresh_exp is null)"#;

        let rows_affected = if is_hiqlite() {
            DB::hql().execute(sql, params!(exp)).await?
        } else {
            DB::pg_execute(sql, &[&exp]).await?
        };
        info!("Cleaned up {} expires devices", rows_affected);

        Ok(())
    }

    pub async fn invalidate(id: &str) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM devices WHERE id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(id)).await?;
        } else {
            DB::pg_execute(sql, &[&id]).await?;
        }

        // we don't need to manually clean up refresh_tokens because of FK cascades
        Ok(())
    }

    pub async fn revoke_refresh_tokens(device_id: &str) -> Result<(), ErrorResponse> {
        RefreshTokenDevice::invalidate_all_for_device(device_id).await?;

        let sql = "UPDATE devices SET refresh_exp = null WHERE id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(device_id)).await?;
        } else {
            DB::pg_execute(sql, &[&device_id]).await?;
        }

        Ok(())
    }

    pub async fn update_name(
        device_id: &str,
        user_id: &str,
        name: &str,
    ) -> Result<(), ErrorResponse> {
        let sql = "UPDATE devices SET name = $1 WHERE id = $2 AND user_id = $3";
        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(name, device_id, user_id))
                .await?;
        } else {
            DB::pg_execute(sql, &[&name, &device_id, &user_id]).await?;
        }

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
        let ttl = RauthyConfig::get().vars.device_grant.code_lifetime as i64;
        let exp = now.add(chrono::Duration::seconds(ttl));
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

        DB::hql()
            .put(
                Cache::DeviceCode,
                slf.user_code().to_string(),
                &slf,
                Some(ttl),
            )
            .await?;

        Ok(slf)
    }

    pub async fn find_by_device_code(device_code: &str) -> Result<Option<Self>, ErrorResponse> {
        let len = RauthyConfig::get().vars.device_grant.user_code_length as usize;
        let key = &device_code[..len];
        Self::find(key.to_string()).await
    }

    pub async fn find(user_code: String) -> Result<Option<Self>, ErrorResponse> {
        let slf: Option<Self> = DB::hql().get(Cache::DeviceCode, user_code).await?;
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
        DB::hql()
            .delete(Cache::DeviceCode, self.user_code().to_string())
            .await?;
        Ok(())
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        let ttl = RauthyConfig::get().vars.device_grant.code_lifetime as i64;
        DB::hql()
            .put(
                Cache::DeviceCode,
                self.user_code().to_string(),
                self,
                Some(ttl),
            )
            .await?;
        Ok(())
    }
}

impl DeviceAuthCode {
    /// Validates the given `user_code`
    #[inline]
    pub fn user_code(&self) -> &str {
        let len = RauthyConfig::get().vars.device_grant.user_code_length as usize;
        &self.device_code[..len]
    }

    pub fn verification_uri(&self) -> String {
        format!("{}/auth/v1/device", RauthyConfig::get().pub_url_with_scheme)
    }

    pub fn verification_uri_complete(&self) -> String {
        format!(
            "{}/auth/v1/device?code={}",
            RauthyConfig::get().pub_url_with_scheme,
            self.user_code()
        )
    }
}
