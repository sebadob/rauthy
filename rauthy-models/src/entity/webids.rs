use crate::app_state::AppState;
use actix_web::web;
use rauthy_common::constants::CACHE_NAME_12HR;
use rauthy_common::error_response::ErrorResponse;
use redhac::{cache_get, cache_get_from, cache_get_value, cache_insert, AckLevel};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Debug)]
pub struct WebIdEntity {
    pub user_id: String,
    pub is_open: bool,
    pub data: Option<Vec<u8>>,
}

impl From<WebId> for WebIdEntity {
    fn from(value: WebId) -> Self {
        let data = value.data.map(|d| bincode::serialize(&d).unwrap());

        Self {
            user_id: value.user_id,
            is_open: value.is_open,
            data,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WebId {
    pub user_id: String,
    pub is_open: bool,
    // TODO is JSON value a "good" value type here or does turtle only have String anyway?
    // turtle works with triplets -> key and value are clear, what about predicate in this case?
    // is is always the same, or do we need to create some "turtle entry struct"?
    pub data: Option<HashMap<String, serde_json::Value>>,
}

impl WebId {
    pub async fn find(data: &web::Data<AppState>, user_id: String) -> Result<Self, ErrorResponse> {
        if let Some(web_id) = cache_get!(
            Self,
            CACHE_NAME_12HR.to_string(),
            Self::cache_idx(&user_id),
            &data.caches.ha_cache_config,
            false
        )
        .await?
        {
            return Ok(web_id);
        }

        let res = query_as!(
            WebIdEntity,
            "SELECT * FROM webids WHERE user_id = $1",
            user_id
        )
        .fetch_one(&data.db)
        .await?;

        let web_id = WebId::from(res);

        cache_insert(
            CACHE_NAME_12HR.to_string(),
            Self::cache_idx(&web_id.user_id),
            &data.caches.ha_cache_config,
            &web_id,
            AckLevel::Leader,
        )
        .await?;

        Ok(web_id)
    }

    pub async fn upsert(data: &web::Data<AppState>, web_id: WebId) -> Result<(), ErrorResponse> {
        let slf = WebIdEntity::from(web_id.clone());

        #[cfg(feature = "sqlite")]
        let q = query!(
            "INSERT OR REPLACE INTO webids (user_id, is_open, data) VALUES ($1, $2, $3)",
            slf.user_id,
            slf.is_open,
            slf.data,
        );
        #[cfg(not(feature = "sqlite"))]
        let q = query!(
            r#"INSERT INTO webids (user_id, is_open, data) VALUES ($1, $2, $3)
            ON CONFLICT(user_id) DO UPDATE SET is_open = $2, data = $3"#,
            slf.user_id,
            slf.is_open,
            slf.data,
        );
        q.execute(&data.db).await?;

        cache_insert(
            CACHE_NAME_12HR.to_string(),
            Self::cache_idx(&slf.user_id),
            &data.caches.ha_cache_config,
            &web_id,
            AckLevel::Leader,
        )
        .await?;

        Ok(())
    }
}

impl WebId {
    fn cache_idx(user_id: &str) -> String {
        format!("web_id_{}", user_id)
    }
}
impl From<WebIdEntity> for WebId {
    fn from(value: WebIdEntity) -> Self {
        let data = if let Some(data) = value.data {
            let d = bincode::deserialize(&data).unwrap();
            Some(d)
        } else {
            None
        };

        Self {
            user_id: value.user_id,
            is_open: value.is_open,
            data,
        }
    }
}
