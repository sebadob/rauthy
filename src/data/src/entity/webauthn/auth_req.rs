use crate::database::{Cache, DB};
use crate::rauthy_config::RauthyConfig;
use rauthy_common::utils::get_rand;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct WebauthnServiceReq {
    pub code: String,
    pub user_id: String,
}

// CRUD
impl WebauthnServiceReq {
    pub fn new(user_id: String) -> Self {
        Self {
            code: get_rand(48),
            user_id,
        }
    }

    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        DB::hql().delete(Cache::Webauthn, self.code.clone()).await?;
        Ok(())
    }

    pub async fn find(code: String) -> Result<Self, ErrorResponse> {
        let res = DB::hql().get(Cache::Webauthn, code).await?;
        match res {
            None => Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "Webauthn Service Request Data not found",
            )),
            Some(res) => Ok(res),
        }
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        let ttl = Some(RauthyConfig::get().vars.webauthn.data_exp.as_secs() as i64);
        DB::hql()
            .put(Cache::Webauthn, self.code.clone(), self, ttl)
            .await?;
        Ok(())
    }
}
