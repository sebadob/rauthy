use crate::database::{Cache, DB};
use chrono::{DateTime, Utc};
use hiqlite::macros::params;
use rauthy_api_types::users::DeviceResponse;
use rauthy_common::constants::DEVICE_KEY_LENGTH;
#[cfg(debug_assertions)]
use rauthy_common::constants::RAUTHY_VERSION;
use rauthy_common::is_hiqlite;
use rauthy_common::utils::get_rand;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

use crate::rauthy_config::RauthyConfig;
use rauthy_derive::FromPgRow;
use tracing::info;

#[derive(Debug, Deserialize, FromPgRow)]
pub struct DeviceEntity {
    pub id: String,
    pub client_id: String,
    pub user_id: Option<String>,
    pub created: i64,
    pub access_exp: i64,
    pub refresh_exp: Option<i64>,
    pub peer_ip: String,
    pub name: String,
    /// `Client::generation` of `client_id` when the device was authorized.
    #[serde(default)]
    pub client_generation: String,
}

impl DeviceEntity {
    pub fn is_for_client_generation(&self, generation: &str) -> bool {
        self.client_generation == generation
    }

    pub async fn insert(self) -> Result<(), ErrorResponse> {
        let sql = r#"
INSERT INTO devices
(id, client_id, user_id, created, access_exp, refresh_exp, peer_ip, name, client_generation)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#;

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
                        self.name,
                        self.client_generation
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
                    &self.client_generation,
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

    pub async fn delete_refresh_tokens(device_id: &str) -> Result<(), ErrorResponse> {
        let sql_tokens = "DELETE FROM refresh_tokens_devices WHERE device_id = $1";
        let sql_upd = "UPDATE devices SET refresh_exp = null WHERE id = $1";

        if is_hiqlite() {
            let mut txn = Vec::with_capacity(2);
            txn.push((sql_tokens, params!(device_id)));
            txn.push((sql_upd, params!(device_id)));

            for res in DB::hql().txn(txn).await? {
                res?;
            }
        } else {
            let mut cl = DB::pg().await?;
            let txn = cl.transaction().await?;

            DB::pg_txn_append(&txn, sql_tokens, &[&device_id]).await?;
            DB::pg_txn_append(&txn, sql_upd, &[&device_id]).await?;
            txn.commit().await?;
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
    pub nonce: Option<String>,
    // TODO we should probably save it hashed, even though it is only very short-lived
    //  saved additionally here to have fewer cache requests during client polling
    // TODO 2 we should not store the secret at all, and instead favor the additional cache lookups.
    //  The reason is simply client secret rotation. We should only save the information if the
    //  linked client is confidential. If so, we lookup its secret and on mismatch, even try the
    //  fallback secret which is saved after a rotation. Only this way we can make graceful secret
    //  rotation possible even with lots of devices.
    pub client_secret: Option<String>,
    // The warning counter will increase if a client does not stick to
    // the given interval and gets 'slow_down' from us. If this happens
    // too many times, the IP will be blacklisted.1
    pub warnings: u8,
    /// `Client::generation` of `client_id` when the code was created. Last on purpose: bincode
    /// ignores trailing bytes, so older versions can still decode a code with this field.
    pub client_generation: String,
}

/// `DeviceAuthCode` as cached before `client_generation` existed.
#[derive(Deserialize)]
#[cfg_attr(test, derive(Serialize))]
struct DeviceAuthCodeNoGeneration {
    client_id: String,
    device_code: String,
    verified_by: Option<String>,
    exp: DateTime<Utc>,
    last_poll: DateTime<Utc>,
    scopes: Option<String>,
    nonce: Option<String>,
    client_secret: Option<String>,
    warnings: u8,
}

impl DeviceAuthCode {
    /// DeviceAuthCode's live inside the cache only
    pub async fn new(
        scopes: Option<String>,
        client_id: String,
        client_generation: String,
        client_secret: Option<String>,
        nonce: Option<String>,
    ) -> Result<Self, ErrorResponse> {
        let now = Utc::now();
        let ttl = RauthyConfig::get().vars.device_grant.code_lifetime;
        let exp = now.add(ttl);
        let slf = Self {
            client_id,
            client_generation,
            device_code: get_rand(DEVICE_KEY_LENGTH as usize),
            verified_by: None,
            exp,
            last_poll: now,
            scopes,
            nonce,
            client_secret,
            warnings: 0,
        };

        DB::hql()
            .put(
                Cache::DeviceCode,
                slf.user_code().to_string(),
                &slf,
                Some(ttl.as_secs() as i64),
            )
            .await?;

        Ok(slf)
    }

    pub async fn find_by_device_code(device_code: &str) -> Result<Option<Self>, ErrorResponse> {
        let len = RauthyConfig::get().vars.device_grant.user_code_length as usize;
        match device_code.get(..len) {
            Some(key) => Self::find_pending(key.to_string()).await,
            None => Ok(None),
        }
    }

    pub async fn find(user_code: String) -> Result<Option<Self>, ErrorResponse> {
        let bytes = DB::hql()
            .get_remove_bytes(Cache::DeviceCode, user_code)
            .await?;
        Self::validate_expiry(bytes.as_deref().and_then(Self::decode)).await
    }

    pub async fn find_pending(user_code: String) -> Result<Option<Self>, ErrorResponse> {
        let bytes = DB::hql().get_bytes(Cache::DeviceCode, user_code).await?;
        Self::validate_expiry(bytes.as_deref().and_then(Self::decode)).await
    }

    /// Decodes a cached code, falling back to the layout without `client_generation`. Such codes
    /// get the empty one, which every client has until it is recreated.
    fn decode(bytes: &[u8]) -> Option<Self> {
        #[cfg(debug_assertions)]
        if !RAUTHY_VERSION.starts_with("0.37.") && !RAUTHY_VERSION.starts_with("0.38.") {
            todo!("Remove DeviceAuthCodeNoGeneration");
        }

        let config = bincode_next::config::legacy();
        if let Ok((slf, _)) = bincode_next::serde::decode_from_slice::<Self, _>(bytes, config) {
            return Some(slf);
        }

        let (code, _) =
            bincode_next::serde::decode_from_slice::<DeviceAuthCodeNoGeneration, _>(bytes, config)
                .ok()?;
        Some(Self {
            client_id: code.client_id,
            device_code: code.device_code,
            verified_by: code.verified_by,
            exp: code.exp,
            last_poll: code.last_poll,
            scopes: code.scopes,
            nonce: code.nonce,
            client_secret: code.client_secret,
            warnings: code.warnings,
            client_generation: String::new(),
        })
    }

    async fn validate_expiry(slf: Option<Self>) -> Result<Option<Self>, ErrorResponse> {
        match slf {
            Some(slf) => {
                if slf.exp < Utc::now() {
                    slf.delete().await?;
                    Ok(None)
                } else {
                    Ok(Some(slf))
                }
            }
            None => Ok(None),
        }
    }

    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        DB::hql()
            .delete(Cache::DeviceCode, self.user_code().to_string())
            .await?;
        Ok(())
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        let ttl = RauthyConfig::get()
            .vars
            .device_grant
            .code_lifetime
            .as_secs() as i64;
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
    /// `current` is the stored `Client::generation` of `client_id`, `None` if it does not exist.
    #[inline]
    pub fn is_for_client_generation(&self, current: Option<&str>) -> bool {
        current == Some(self.client_generation.as_str())
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_code_client_generation() {
        let now = Utc::now();
        let code = DeviceAuthCode {
            client_id: "client_1".to_string(),
            client_generation: "gen_a".to_string(),
            device_code: "code".to_string(),
            verified_by: Some("user_1".to_string()),
            exp: now,
            last_poll: now,
            scopes: None,
            nonce: None,
            client_secret: None,
            warnings: 0,
        };

        assert!(code.is_for_client_generation(Some("gen_a")));
        assert!(!code.is_for_client_generation(Some("gen_b")));
        assert!(!code.is_for_client_generation(Some("")));
        assert!(!code.is_for_client_generation(None));
    }

    fn encode<T: Serialize>(value: &T) -> Vec<u8> {
        bincode_next::serde::encode_to_vec(value, bincode_next::config::legacy()).unwrap()
    }

    #[test]
    fn test_device_code_decode_without_generation() {
        let now = Utc::now();
        let old = DeviceAuthCodeNoGeneration {
            client_id: "client_1".to_string(),
            device_code: "code".to_string(),
            verified_by: Some("user_1".to_string()),
            exp: now,
            last_poll: now,
            scopes: Some("openid".to_string()),
            nonce: None,
            client_secret: None,
            warnings: 2,
        };
        let code = DeviceAuthCode::decode(&encode(&old)).unwrap();
        assert_eq!(code.client_generation, "");
        assert_eq!(code.verified_by.as_deref(), Some("user_1"));
        assert_eq!(code.scopes.as_deref(), Some("openid"));
        assert_eq!(code.warnings, 2);
        assert!(code.is_for_client_generation(Some("")));
    }

    #[test]
    fn test_device_code_decode_current_and_by_older_versions() {
        let now = Utc::now();
        let code = DeviceAuthCode {
            client_id: "client_1".to_string(),
            device_code: "code".to_string(),
            verified_by: None,
            exp: now,
            last_poll: now,
            scopes: Some("openid".to_string()),
            nonce: None,
            client_secret: Some("secret".to_string()),
            warnings: 1,
            client_generation: "gen_a".to_string(),
        };
        let bytes = encode(&code);

        let decoded = DeviceAuthCode::decode(&bytes).unwrap();
        assert_eq!(decoded.client_generation, "gen_a");
        assert_eq!(decoded.client_secret.as_deref(), Some("secret"));

        let (old, _) = bincode_next::serde::decode_from_slice::<DeviceAuthCodeNoGeneration, _>(
            &bytes,
            bincode_next::config::legacy(),
        )
        .unwrap();
        assert_eq!(old.client_secret.as_deref(), Some("secret"));
        assert_eq!(old.warnings, 1);
    }

    #[test]
    fn test_device_client_generation() {
        let mut device: DeviceEntity = serde_json::from_value(serde_json::json!({
            "id": "dev_1",
            "client_id": "client_1",
            "user_id": "user_1",
            "created": 1,
            "access_exp": 2,
            "refresh_exp": 3,
            "peer_ip": "127.0.0.1",
            "name": "dev_1",
        }))
        .unwrap();
        assert_eq!(device.client_generation, "");
        assert!(device.is_for_client_generation(""));
        assert!(!device.is_for_client_generation("gen_a"));

        device.client_generation = "gen_a".to_string();
        assert!(device.is_for_client_generation("gen_a"));
        assert!(!device.is_for_client_generation("gen_b"));
        assert!(!device.is_for_client_generation(""));
    }
}
