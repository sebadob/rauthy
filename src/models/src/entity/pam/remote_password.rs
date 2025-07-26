use crate::database::{Cache, DB};
use crate::rauthy_config::RauthyConfig;
use chrono::Utc;
use cryptr::utils::secure_random_alnum;
use rauthy_common::sha256;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PamRemotePassword {
    pub username: String,
    pub exp: i64,
    pub password: Vec<u8>,
}

impl PamRemotePassword {
    /// Returns `(Self, plain_password)`
    pub async fn create(username: String) -> Result<(Self, String), ErrorResponse> {
        let config = &RauthyConfig::get().vars.pam;
        let password_plain = secure_random_alnum(config.remote_password_len as usize);
        let now = Utc::now().timestamp();
        let ttl_secs = config.remote_password_ttl as i64;
        let exp = now + ttl_secs;

        let hash = sha256!(password_plain.as_bytes()).to_vec();

        let slf = Self {
            username,
            exp,
            password: hash,
        };

        DB::hql()
            .put(Cache::PAM, slf.username.clone(), &slf, Some(ttl_secs))
            .await?;

        Ok((slf, password_plain))
    }

    pub async fn get(username: String) -> Result<Self, ErrorResponse> {
        match DB::hql().get::<_, _, Self>(Cache::PAM, username).await? {
            None => Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "No PAM password exists",
            )),
            Some(slf) => {
                // this should never be possible, because the TTL is matched during insert
                if slf.exp < Utc::now().timestamp() {
                    Err(ErrorResponse::new(
                        ErrorResponseType::Unauthorized,
                        "Password has expired",
                    ))
                } else {
                    Ok(slf)
                }
            }
        }
    }

    pub fn compare_password(&self, password: &[u8]) -> Result<(), ErrorResponse> {
        if self.password.as_slice() == sha256!(password) {
            Ok(())
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Invalid Credentials",
            ))
        }
    }
}
