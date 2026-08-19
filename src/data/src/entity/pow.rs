use crate::database::{Cache, DB};
use crate::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use spow::pow::Pow;

pub struct PowEntity;

impl PowEntity {
    pub async fn create() -> Result<Pow, ErrorResponse> {
        let cfg = &RauthyConfig::get().vars;
        let difficulty = if cfg.dev.dev_mode {
            10
        } else {
            cfg.pow.difficulty
        };

        let pow = Pow::with_difficulty(difficulty, cfg.pow.exp as u32)?;

        DB::hql()
            .put(
                Cache::PoW,
                pow.challenge.clone(),
                &pow,
                Some(cfg.pow.exp as i64),
            )
            .await?;

        Ok(pow)
    }

    /// Checks re-usages of PoWs and prevents a future re-use. The challenge is
    /// consumed atomically: `get_remove` claims it in one raft entry, so two
    /// concurrent requests cannot both consume it.
    pub async fn check_prevent_reuse(challenge: String) -> Result<(), ErrorResponse> {
        let client = DB::hql();

        let claimed: Option<Pow> = client.get_remove(Cache::PoW, challenge).await?;
        match claimed {
            Some(_) => Ok(()),
            None => Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "PoW not found",
            )),
        }
    }
}
