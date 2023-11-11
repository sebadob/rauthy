use crate::app_state::AppState;
use actix_web::web;
use rauthy_common::constants::{CACHE_NAME_12HR, PUB_URL_WITH_SCHEME};
use rauthy_common::error_response::ErrorResponse;
use redhac::{cache_get, cache_get_from, cache_get_value, cache_insert, AckLevel};
use rio_api::{model::Triple, parser::TriplesParser};
use rio_turtle::{NTriplesParser, TurtleError};
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use sqlx::{query, query_as};
use std::{fmt::Display, str::FromStr};
use utoipa::ToSchema;

#[derive(Debug)]
pub struct WebIdEntity {
    pub user_id: String,
    pub is_open: bool,
    pub data: Option<Vec<u8>>,
}

impl From<WebId> for WebIdEntity {
    fn from(value: WebId) -> Self {
        Self {
            user_id: value.user_id,
            is_open: value.is_open,
            data: value.data.map(|data| data.serialized.into_bytes()),
        }
    }
}

/// An rdf graph that is backed by valid n-triples serialized data.
#[derive(Debug, Clone, SerializeDisplay, DeserializeFromStr)]
pub struct NTriplesGraph {
    serialized: String,
}

impl NTriplesGraph {
    /// Call the function for each given triple.
    pub fn for_each_triple<'s, E, F>(&'s self, mut f: F) -> Result<(), E>
    where
        F: for<'a> FnMut(Triple<'a>) -> Result<(), E>,
    {
        struct WE<IE>(Option<IE>);
        impl<IE> From<TurtleError> for WE<IE> {
            fn from(_v: TurtleError) -> Self {
                Self(None)
            }
        }

        NTriplesParser::new(self.serialized.as_bytes())
            .parse_all(&mut move |t| f(t).map_err(|e| WE(Some(e))))
            .map_err(|e| e.0.expect("Must be only callback error."))
    }
}

impl Display for NTriplesGraph {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.serialized.fmt(f)
    }
}

impl TryFrom<Vec<u8>> for NTriplesGraph {
    type Error = TurtleError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        NTriplesParser::new(bytes.as_slice()).parse_all(&mut |_| Ok::<_, TurtleError>(()))?;
        Ok(Self {
            serialized: String::from_utf8(bytes).expect("Checked to be valid."),
        })
    }
}

impl From<NTriplesGraph> for String {
    #[inline]
    fn from(v: NTriplesGraph) -> String {
        v.serialized
    }
}

impl FromStr for NTriplesGraph {
    type Err = TurtleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.to_owned().into_bytes().try_into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WebId {
    pub user_id: String,
    pub is_open: bool,
    pub data: Option<NTriplesGraph>,
}

impl WebId {
    /// Resolve webid uri
    #[inline]
    pub fn resolve_webid_uri(user_id: &str) -> String {
        format!(
            "{}/auth/v1/users/{}/webid#me",
            *PUB_URL_WITH_SCHEME, user_id
        )
    }

    /// Resolve webid card uri.
    #[inline]
    pub fn resolve_webid_card_uri(user_id: &str) -> String {
        format!("{}/auth/v1/users/{}/webid", *PUB_URL_WITH_SCHEME, user_id)
    }

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
        Self {
            user_id: value.user_id,
            is_open: value.is_open,
            data: value.data.and_then(|data| data.try_into().ok()),
        }
    }
}
