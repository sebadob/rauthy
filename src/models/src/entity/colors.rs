use crate::app_state::AppState;
use crate::cache::{Cache, DB};
use actix_web::web;
use rauthy_api_types::clients::ColorsRequest;
use rauthy_common::constants::CACHE_TTL_APP;
use rauthy_error::ErrorResponse;
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
        sqlx::query!("DELETE FROM colors WHERE client_id = $1", client_id,)
            .execute(&data.db)
            .await?;

        DB::client()
            .delete(Cache::App, Self::cache_idx(client_id))
            .await?;

        Ok(())
    }

    pub async fn find(
        data: &web::Data<AppState>,
        client_id: &str,
    ) -> Result<Colors, ErrorResponse> {
        let idx = Self::cache_idx(client_id);
        let client = DB::client();

        if let Some(slf) = client.get(Cache::App, &idx).await? {
            return Ok(slf);
        }

        let res = sqlx::query_as!(Self, "SELECT * FROM colors WHERE client_id = $1", client_id)
            .fetch_optional(&data.db)
            .await?;
        let colors = match res {
            None => Colors::default(),
            Some(entity) => entity.colors()?,
        };

        client.put(Cache::App, idx, &colors, CACHE_TTL_APP).await?;

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

        #[cfg(not(feature = "postgres"))]
        let q = sqlx::query!(
            "INSERT OR REPLACE INTO colors (client_id, data) values ($1, $2)",
            client_id,
            col_bytes,
        );
        #[cfg(feature = "postgres")]
        let q = sqlx::query!(
            r#"INSERT INTO colors (client_id, data) values ($1, $2)
                ON CONFLICT(client_id) DO UPDATE SET data = $2"#,
            client_id,
            col_bytes,
        );
        q.execute(&data.db).await?;

        DB::client()
            .put(Cache::App, Self::cache_idx(client_id), &cols, CACHE_TTL_APP)
            .await?;

        Ok(())
    }
}

impl ColorEntity {
    #[inline]
    pub fn colors(&self) -> Result<Colors, ErrorResponse> {
        Colors::from_bytes(self.data.as_slice())
    }

    #[inline]
    fn cache_idx(client_id: &str) -> String {
        format!("colors_{}", client_id)
    }
}

// TODO String -> Cow<_> would be a nice improvement in default cases
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
