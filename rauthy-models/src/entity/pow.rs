use crate::app_state::AppState;
use actix_web::web;
use rauthy_common::constants::{CACHE_NAME_POW, POW_DIFFICULTY, POW_EXP};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use redhac::{cache_del, cache_get, cache_get_from, cache_get_value, cache_put};
use spow::pow::Pow;

pub struct PowEntity;

impl PowEntity {
    pub async fn create(data: &web::Data<AppState>) -> Result<Pow, ErrorResponse> {
        let pow = Pow::with_difficulty(*POW_DIFFICULTY, *POW_EXP)?;

        cache_put(
            CACHE_NAME_POW.to_string(),
            pow.challenge.clone(),
            &data.caches.ha_cache_config,
            &pow,
        )
        .await?;

        Ok(pow)
    }

    /// Checks re-usages of PoWs and prevents a future re-use
    pub async fn check_prevent_reuse(
        data: &web::Data<AppState>,
        challenge: String,
    ) -> Result<(), ErrorResponse> {
        let pow = match cache_get!(
            Pow,
            CACHE_NAME_POW.to_string(),
            challenge,
            &data.caches.ha_cache_config,
            true
        )
        .await?
        {
            Some(pow) => pow,
            None => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::NotFound,
                    "PoW not found".to_string(),
                ));
            }
        };

        cache_del(
            CACHE_NAME_POW.to_string(),
            pow.challenge,
            &data.caches.ha_cache_config,
        )
        .await?;
        Ok(())
    }
}
