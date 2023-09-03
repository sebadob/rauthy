use crate::app_state::AppState;
use crate::request::ColorsRequest;
use actix_web::web;
use rauthy_common::constants::CACHE_NAME_12HR;
use rauthy_common::error_response::ErrorResponse;
use redhac::{cache_del, cache_get, cache_get_from, cache_get_value, cache_put};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, PartialEq, Eq, FromRow, Deserialize, Serialize, ToSchema)]
pub struct ColorEntity {
    pub client_id: String,
    pub data: Vec<u8>,
}

// CRUD
impl ColorEntity {
    pub async fn delete(data: &web::Data<AppState>, client_id: &str) -> Result<(), ErrorResponse> {
        sqlx::query!("delete from colors where client_id = $1", client_id,)
            .execute(&data.db)
            .await?;

        cache_del(
            CACHE_NAME_12HR.to_string(),
            Self::cache_idx(client_id),
            &data.caches.ha_cache_config,
        )
        .await?;

        Ok(())
    }

    pub async fn find(
        data: &web::Data<AppState>,
        client_id: &str,
    ) -> Result<Colors, ErrorResponse> {
        let idx = Self::cache_idx(client_id);
        if let Some(colors) = cache_get!(
            Colors,
            CACHE_NAME_12HR.to_string(),
            idx.clone(),
            &data.caches.ha_cache_config,
            false
        )
        .await?
        {
            return Ok(colors);
        }

        let res = sqlx::query_as!(Self, "select * from colors where client_id = $1", client_id)
            .fetch_optional(&data.db)
            .await?;
        let colors = match res {
            None => Colors::default(),
            Some(entity) => entity.colors()?,
        };

        cache_put(
            CACHE_NAME_12HR.to_string(),
            idx,
            &data.caches.ha_cache_config,
            &colors,
        )
        .await?;

        Ok(colors)
    }

    pub async fn find_rauthy(data: &web::Data<AppState>) -> Result<Colors, ErrorResponse> {
        Self::find(data, "rauthy").await
    }

    pub async fn update(
        data: &web::Data<AppState>,
        client_id: &str,
        req: ColorsRequest,
    ) -> Result<(), ErrorResponse> {
        let cols = Colors::from(req);
        let col_bytes = cols.as_bytes();

        #[cfg(feature = "sqlite")]
        let q = sqlx::query!(
            "insert or replace into colors (client_id, data) values ($1, $2)",
            client_id,
            col_bytes,
        );
        #[cfg(not(feature = "sqlite"))]
        let q = sqlx::query!(
            r#"insert into colors (client_id, data) values ($1, $2)
                on conflict(client_id) do update set data = $2"#,
            client_id,
            col_bytes,
        );

        q.execute(&data.db).await?;

        cache_put(
            CACHE_NAME_12HR.to_string(),
            Self::cache_idx(client_id),
            &data.caches.ha_cache_config,
            &cols,
        )
        .await?;

        Ok(())
    }
}

impl ColorEntity {
    pub fn colors(&self) -> Result<Colors, ErrorResponse> {
        Colors::from_bytes(self.data.as_slice())
    }

    fn cache_idx(client_id: &str) -> String {
        format!("colors_{}", client_id)
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, FromRow, ToSchema)]
pub struct Colors {
    pub act1: String,
    pub act1a: String,
    pub act2: String,
    pub act2a: String,
    pub acnt: String,
    pub acnta: String,
    pub ok: String,
    pub err: String,
    pub glow: String,
    pub gmid: String,
    pub ghigh: String,
    pub text: String,
    pub bg: String,
}

impl Default for Colors {
    fn default() -> Self {
        // These are rauthy's default colors
        Self {
            act1: "#6b3d99".to_string(),
            act1a: "#714d99".to_string(),
            act2: "#388c51".to_string(),
            act2a: "#4d8c62".to_string(),
            acnt: "#3d5d99".to_string(),
            acnta: "#36486b".to_string(),
            ok: "#43993d".to_string(),
            err: "#993d49".to_string(),
            glow: "#545454".to_string(),
            gmid: "#b2b2b2".to_string(),
            ghigh: "#f2f2f2".to_string(),
            text: "#383838".to_string(),
            bg: "#f7f7f7".to_string(),
        }
    }
}

impl From<ColorsRequest> for Colors {
    fn from(value: ColorsRequest) -> Self {
        Self {
            act1: value.act1,
            act1a: value.act1a,
            act2: value.act2,
            act2a: value.act2a,
            acnt: value.acnt,
            acnta: value.acnta,
            ok: value.ok,
            err: value.err,
            glow: value.glow,
            gmid: value.gmid,
            ghigh: value.ghigh,
            text: value.text,
            bg: value.bg,
        }
    }
}

impl From<Colors> for Vec<u8> {
    fn from(value: Colors) -> Self {
        bincode::serialize(&value).unwrap()
    }
}

impl Colors {
    pub fn as_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ErrorResponse> {
        Ok(bincode::deserialize::<Self>(bytes)?)
    }
}
