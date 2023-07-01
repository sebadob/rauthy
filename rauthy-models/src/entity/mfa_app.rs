use crate::app_state::AppState;
use crate::entity::users::User;
use crate::request::NewMfaAppRequest;
use actix_web::web;
use rauthy_common::constants::{
    CACHE_NAME_12HR, CACHE_NAME_MFA_APP_REQ, IDX_MFA_APP, MFA_REQ_LIFETIME,
};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::get_rand;
use redhac::{cache_del, cache_get, cache_get_from, cache_get_value, cache_put};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::ops::Add;
use tracing::warn;

#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct MfaApp {
    pub app_id: String,
    pub email: String,
    pub secret: String,
}

/// CRUD
impl MfaApp {
    pub async fn create(
        data: &web::Data<AppState>,
        new_mfa_app: MfaApp,
    ) -> Result<Self, ErrorResponse> {
        let mut user = User::find_by_email(data, new_mfa_app.email.clone()).await?;

        let mut txn = data.db.begin().await?;

        sqlx::query("insert into mfa_apps (app_id, email, secret) values ($1, $2, $3)")
            .bind(&new_mfa_app.app_id)
            .bind(&new_mfa_app.email)
            .bind(&new_mfa_app.secret)
            .execute(&mut txn)
            .await?;

        user.mfa_app = Some(new_mfa_app.app_id.clone());
        user.save(data, None, Some(&mut txn)).await?;

        txn.commit().await?;

        let idx = format!("{}{}", IDX_MFA_APP, &new_mfa_app.app_id);
        cache_put(
            CACHE_NAME_12HR.to_string(),
            idx,
            &data.caches.ha_cache_config,
            &new_mfa_app,
        )
        .await?;

        Ok(new_mfa_app)
    }

    pub async fn find(
        data: &web::Data<AppState>,
        app_id: String,
        email: String,
    ) -> Result<MfaApp, ErrorResponse> {
        let idx = format!("{}{}", IDX_MFA_APP, app_id);
        let res = cache_get!(
            MfaApp,
            CACHE_NAME_12HR.to_string(),
            idx.clone(),
            &data.caches.ha_cache_config,
            true
        )
        .await?;
        if res.is_some() {
            let mfa_app = res.unwrap();
            return if mfa_app.email == email && mfa_app.app_id == app_id {
                Ok(mfa_app)
            } else {
                warn!("Bad Request for MfaApp - Invalid AppId + E-Mail combination");
                Err(ErrorResponse::new(
                    ErrorResponseType::NotFound,
                    String::from("MfaApp NotFound"),
                ))
            };
        }

        let res = sqlx::query_as::<_, Self>("select * from mfa_apps where id = $1")
            .bind(&app_id)
            .fetch_one(&data.db)
            .await?;

        cache_put(
            CACHE_NAME_12HR.to_string(),
            idx,
            &data.caches.ha_cache_config,
            &res,
        )
        .await?;

        Ok(res)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaAppReg {
    pub app_id: String,
    pub code: String,
    pub email: String,
    pub exp: chrono::NaiveDateTime,
    pub challenge: String,
}

/// CRUD
impl MfaAppReg {
    pub async fn create(
        data: &web::Data<AppState>,
        email: String,
        req_data: NewMfaAppRequest,
    ) -> Result<MfaAppReg, ErrorResponse> {
        let exp = chrono::Local::now()
            .naive_local()
            .add(chrono::Duration::seconds(*MFA_REQ_LIFETIME as i64));
        let code = get_rand(8);
        let challenge = get_rand(40);
        let mfa_req = MfaAppReg {
            app_id: req_data.app_id.clone(),
            code,
            email,
            exp,
            challenge,
        };

        cache_put(
            CACHE_NAME_MFA_APP_REQ.to_string(),
            req_data.app_id,
            &data.caches.ha_cache_config,
            &mfa_req,
        )
        .await?;
        Ok(mfa_req)
    }

    pub async fn find(data: &web::Data<AppState>, app_id: String) -> Result<Self, ErrorResponse> {
        let res = cache_get!(
            MfaAppReg,
            CACHE_NAME_MFA_APP_REQ.to_string(),
            app_id,
            &data.caches.ha_cache_config,
            true
        )
        .await?;

        if res.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                String::from("The MfaAppReg could not be found"),
            ));
        }

        Ok(res.unwrap())
    }

    pub async fn delete(data: &web::Data<AppState>, app_id: String) -> Result<(), ErrorResponse> {
        cache_del(
            CACHE_NAME_MFA_APP_REQ.to_string(),
            app_id,
            &data.caches.ha_cache_config,
        )
        .await?;
        Ok(())
    }
}
