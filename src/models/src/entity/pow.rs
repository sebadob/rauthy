use crate::database::{Cache, DB};
use rauthy_common::constants::{POW_DIFFICULTY, POW_EXP};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use spow::pow::Pow;

pub struct PowEntity;

impl PowEntity {
    pub async fn create() -> Result<Pow, ErrorResponse> {
        let pow = Pow::with_difficulty(*POW_DIFFICULTY, *POW_EXP)?;

        DB::hql()
            .put(
                Cache::PoW,
                pow.challenge.clone(),
                &pow,
                Some(*POW_EXP as i64),
            )
            .await?;

        Ok(pow)
    }

    /// Checks re-usages of PoWs and prevents a future re-use
    pub async fn check_prevent_reuse(challenge: String) -> Result<(), ErrorResponse> {
        let client = DB::hql();

        let opt: Option<Pow> = client.get(Cache::PoW, challenge).await?;
        let pow = match opt {
            Some(pow) => pow,
            None => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::NotFound,
                    "PoW not found",
                ));
            }
        };

        client.delete(Cache::PoW, pow.challenge).await?;
        Ok(())
    }
}
