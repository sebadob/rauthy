use crate::database::{Cache, DB};
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum EmailRateLimit {
    RegisteredAlready { email: String },
}

impl EmailRateLimit {
    pub async fn limit(&self) -> Result<(), ErrorResponse> {
        DB::hql()
            .put(Cache::EmailRateLimit, self.key(), &(), Some(3600))
            .await?;

        Ok(())
    }

    pub async fn is_limited(&self) -> Result<bool, ErrorResponse> {
        if DB::hql()
            .get::<_, _, ()>(Cache::EmailRateLimit, self.key())
            .await?
            .is_some()
        {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl EmailRateLimit {
    fn key(&self) -> String {
        match self {
            Self::RegisteredAlready { email } => format!("reg_{}", email),
        }
    }
}
