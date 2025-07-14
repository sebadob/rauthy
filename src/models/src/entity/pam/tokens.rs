use crate::database::{Cache, DB};
use crate::entity::users::User;
use chrono::Utc;
use cryptr::utils::secure_random_alnum;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use std::ops::Add;
use utoipa::ToSchema;

// TODO we could think about a small daemon on each machine, have a shorter timeout, and the
//  possibility to refresh. This would require way more managing overhead though.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct PamToken {
    pub id: String,
    pub exp: i64,
    pub user_id: String,
    pub user_email: String,
    pub username: String,
    pub roles: Vec<String>,
    pub groups: Vec<String>,
}

impl Debug for PamToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PamAuthToken {{ id: {}(...), exp: {}, user_id: {}, user_email: {}, username: {}, \
            roles: {:?}, groups: {:?} }}",
            &self.id[..5],
            self.exp,
            self.user_id,
            self.user_email,
            self.username,
            self.roles,
            self.groups,
        )
    }
}

// TODO update the Cache index once testing is done and we are about to merge it into main.
//  if should have a dedicated cache and not live inside Cache::AuthCode. Not yet done to not break
//  the existing patch level for stress-free testing. The current Rauthy version is still in the
//  patch-phase.
impl PamToken {
    pub async fn new(user: User, username: String) -> Result<Self, ErrorResponse> {
        let lifetime_secs = 300;
        // let lifetime_secs = 16 * 3600;

        let roles = user.get_roles();
        let groups = user.get_groups();

        let slf = Self {
            id: secure_random_alnum(48),
            exp: Utc::now()
                .add(chrono::Duration::seconds(lifetime_secs))
                .timestamp(),
            user_id: user.id,
            user_email: user.email,
            username,
            roles,
            groups,
        };

        DB::hql()
            .put(Cache::AuthCode, slf.id.clone(), &slf, Some(lifetime_secs))
            .await?;

        Ok(slf)
    }

    pub async fn find(id: String) -> Result<Self, ErrorResponse> {
        match DB::hql().get(Cache::AuthCode, id).await? {
            Some(slf) => Ok(slf),
            None => Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Invalid PAM token",
            )),
        }
    }

    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        DB::hql().delete(Cache::AuthCode, self.id.clone()).await?;
        Ok(())
    }
}
