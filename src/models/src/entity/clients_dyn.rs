use crate::app_state::AppState;
use crate::database::{Cache, DB};
use actix_web::web;
use chrono::Utc;
use cryptr::EncValue;
use hiqlite::{Param, params};
use rauthy_common::constants::{CACHE_TTL_DYN_CLIENT, CACHE_TTL_IP_RATE_LIMIT};
use rauthy_common::is_hiqlite;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, query, query_as};
use std::net::IpAddr;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Debug, Serialize, Deserialize, FromRow, PostgresMapper)]
#[pg_mapper(table = "clients_dyn")]
pub struct ClientDyn {
    pub id: String,
    pub created: i64,
    pub last_used: Option<i64>,
    pub registration_token: Vec<u8>,
    pub token_endpoint_auth_method: String,
}

impl ClientDyn {
    /// This only deletes a `ClientDyn` from the cache.
    /// The deletion at database level happens via the foreign key cascade.
    pub async fn delete_from_cache(id: &str) -> Result<(), ErrorResponse> {
        DB::hql()
            .delete(Cache::App, ClientDyn::get_cache_entry(id))
            .await?;
        Ok(())
    }

    pub async fn find(id: String) -> Result<Self, ErrorResponse> {
        let client = DB::hql();

        if let Some(slf) = client
            .get(Cache::ClientDynamic, ClientDyn::get_cache_entry(&id))
            .await?
        {
            return Ok(slf);
        }

        let slf: Self = if is_hiqlite() {
            DB::hql()
                .query_as_one("SELECT * FROM clients_dyn WHERE id = $1", params!(id))
                .await?
        } else {
            DB::pg_query_one("SELECT * FROM clients_dyn WHERE id = $1", &[&id]).await?
        };

        client
            .put(
                Cache::ClientDynamic,
                ClientDyn::get_cache_entry(&slf.id),
                &slf,
                *CACHE_TTL_DYN_CLIENT,
            )
            .await?;

        Ok(slf)
    }

    pub async fn update_used(id: &str) -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp();

        if is_hiqlite() {
            DB::hql()
                .execute(
                    "UPDATE clients_dyn SET last_used = $1 WHERE id = $2",
                    params!(now, id),
                )
                .await?;
        } else {
            DB::pg_execute(
                "UPDATE clients_dyn SET last_used = $1 WHERE id = $2",
                &[&id],
            )
            .await?;
        }

        Ok(())
    }
}

impl ClientDyn {
    #[inline]
    pub fn get_cache_entry(id: &str) -> String {
        format!("client_dyn_{}", id)
    }

    /// Returns an Err(_) if the IP is currently existing inside the cache.
    /// If not, the IP will be cached with an Ok(()).
    pub async fn rate_limit_ip(ip: IpAddr) -> Result<(), ErrorResponse> {
        let client = DB::hql();

        let ts: Option<i64> = client.get(Cache::IPRateLimit, ip.to_string()).await?;
        match ts {
            Some(ts) => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::TooManyRequests(ts),
                    format!("You hit a rate limit. You may try again at: {}", ts),
                ));
            }
            None => {
                let now = Utc::now().timestamp();
                client
                    .put(
                        Cache::IPRateLimit,
                        ip.to_string(),
                        &now,
                        *CACHE_TTL_IP_RATE_LIMIT,
                    )
                    .await?;
            }
        }

        Ok(())
    }

    pub fn registration_client_uri(data: &web::Data<AppState>, id: &str) -> String {
        format!("{}/clients_dyn/{}", data.issuer, id)
    }

    pub fn registration_token_plain(&self) -> Result<String, ErrorResponse> {
        let bytes = EncValue::try_from(self.registration_token.clone())?.decrypt()?;
        Ok(String::from_utf8_lossy(bytes.as_ref()).to_string())
    }

    pub fn validate_token(&self, bearer: &str) -> Result<(), ErrorResponse> {
        if self.registration_token_plain()? != bearer {
            Err(ErrorResponse::new(
                ErrorResponseType::WWWAuthenticate("invalid_token".to_string()),
                "Invalid registration_token",
            ))
        } else {
            Ok(())
        }
    }
}
