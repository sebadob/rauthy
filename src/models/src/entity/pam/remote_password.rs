use crate::database::{Cache, DB};
use chrono::Utc;
use cryptr::utils::secure_random_alnum;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PamRemotePassword {
    pub username: String,
    pub exp: i64,
    pub password: String,
}

impl PamRemotePassword {
    pub async fn create(username: String) -> Result<Self, ErrorResponse> {
        // TODO length could be made configurable. For instance, when generating one via mobile,
        //  and having no copy & paste available, could be very annoying when having longer ones.
        let password = secure_random_alnum(16);
        // TODO making this configurable as well is probably a good idea.
        let now = Utc::now().timestamp();
        let ttl_secs = 180;
        let exp = now + ttl_secs;

        let slf = Self {
            username,
            exp,
            password,
        };

        DB::hql()
            .put(Cache::PAM, slf.username.clone(), &slf, Some(ttl_secs))
            .await?;

        Ok(slf)
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
}
