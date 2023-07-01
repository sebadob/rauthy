use crate::app_state::AppState;
use actix_web::web;
use rauthy_common::constants::{CACHE_NAME_POW, POW_EXP_DUR, POW_IT};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::get_rand;
use redhac::{cache_del, cache_get, cache_get_from, cache_get_value, cache_put};
use ring::digest;
use serde::{Deserialize, Serialize};
use std::ops::Add;
use time::OffsetDateTime;
use tokio::time::Instant;
use tracing::{debug, error};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pow {
    pub challenge: String,
    pub it: u64,
    pub exp: i64,
}

/// CRUD
impl Pow {
    pub async fn create(data: &web::Data<AppState>) -> Result<Self, ErrorResponse> {
        let pow = Self::default();
        pow.save(data).await?;
        Ok(pow)
    }

    pub async fn delete(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        cache_del(
            CACHE_NAME_POW.to_string(),
            self.idx(),
            &data.caches.ha_cache_config,
        )
        .await?;
        Ok(())
    }

    pub async fn find(data: &web::Data<AppState>, challenge: &str) -> Result<Self, ErrorResponse> {
        match cache_get!(
            Self,
            CACHE_NAME_POW.to_string(),
            Pow::get_idx(challenge),
            &data.caches.ha_cache_config,
            true
        )
        .await?
        {
            Some(pow) => Ok(pow),
            None => Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "PoW not found".to_string(),
            )),
        }
    }

    pub async fn save(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        cache_put(
            CACHE_NAME_POW.to_string(),
            self.idx(),
            &data.caches.ha_cache_config,
            &self,
        )
        .await?;
        Ok(())
    }
}

impl Pow {
    pub fn idx(&self) -> String {
        Self::get_idx(&self.challenge)
    }

    pub fn get_idx(challenge: &str) -> String {
        format!("pow_{}", challenge)
    }

    pub fn is_valid(&self) -> bool {
        self.exp < OffsetDateTime::now_utc().unix_timestamp()
    }

    pub async fn validate(&self, response: &str) -> Result<(), ErrorResponse> {
        let now = Instant::now();

        // expensive computation
        let challenge = self.challenge.clone();
        let it = self.it;
        let hash = web::block(move || {
            let mut hash = challenge;

            for _ in 0..it {
                let s256 = digest::digest(&digest::SHA256, hash.as_bytes());
                hash = hex::encode(s256);
            }
            debug!(
                "PoW Calculation time taken: {}ms",
                now.elapsed().as_millis()
            );
            hash
        })
        .await?;

        // check challenge verifier
        if hash != response {
            let err = String::from("PoW verification error");
            error!("{}", err);
            debug!("Local s256 hash: {:?}", hash);
            debug!("PoW Response: {:?}", response);
            return Err(ErrorResponse::new(ErrorResponseType::BadRequest, err));
        }

        Ok(())
    }
}

impl Default for Pow {
    fn default() -> Self {
        Self {
            challenge: get_rand(24),
            it: *POW_IT,
            exp: OffsetDateTime::now_utc().add(*POW_EXP_DUR).unix_timestamp(),
        }
    }
}
