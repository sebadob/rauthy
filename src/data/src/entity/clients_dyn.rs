use crate::database::{Cache, DB};
use crate::rauthy_config::RauthyConfig;
use chrono::Utc;
use cryptr::EncValue;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientDyn {
    pub id: String,
    pub created: i64,
    pub last_used: Option<i64>,
    pub registration_token: Vec<u8>,
    pub token_endpoint_auth_method: String,
}

impl From<tokio_postgres::Row> for ClientDyn {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            created: row.get("created"),
            last_used: row.get("last_used"),
            registration_token: row.get("registration_token"),
            token_endpoint_auth_method: row.get("token_endpoint_auth_method"),
        }
    }
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

        let sql = "SELECT * FROM clients_dyn WHERE id = $1";
        let slf: Self = if is_hiqlite() {
            DB::hql().query_as_one(sql, params!(id)).await?
        } else {
            DB::pg_query_one(sql, &[&id]).await?
        };

        let ttl = RauthyConfig::get().vars.dynamic_clients.rate_limit_sec as i64;
        client
            .put(
                Cache::ClientDynamic,
                ClientDyn::get_cache_entry(&slf.id),
                &slf,
                Some(ttl),
            )
            .await?;

        Ok(slf)
    }

    pub async fn update_used(id: &str) -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp();
        let sql = "UPDATE clients_dyn SET last_used = $1 WHERE id = $2";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(now, id)).await?;
        } else {
            DB::pg_execute(sql, &[&id]).await?;
        }

        Ok(())
    }
}

impl ClientDyn {
    #[inline]
    pub fn get_cache_entry(id: &str) -> String {
        format!("client_dyn_{id}")
    }

    /// Returns an Err(_) if the IP is currently existing inside the cache.
    /// If not, the IP will be cached with an Ok(()).
    pub async fn rate_limit_ip(ip: IpAddr) -> Result<(), ErrorResponse> {
        let client = DB::hql();

        let ttl = RauthyConfig::get().vars.dynamic_clients.rate_limit_sec as i64;
        let ts: Option<i64> = client.get(Cache::IpRateLimit, ip.to_string()).await?;
        match ts {
            Some(ts) => {
                let retry_at = ts + ttl;
                return Err(ErrorResponse::new(
                    ErrorResponseType::TooManyRequests(retry_at),
                    format!("You hit a rate limit. You may try again at: {retry_at}"),
                ));
            }
            None => {
                let now = Utc::now().timestamp();
                client
                    .put(Cache::IpRateLimit, ip.to_string(), &now, Some(ttl))
                    .await?;
            }
        }

        Ok(())
    }

    pub fn registration_client_uri(id: &str) -> String {
        format!("{}clients_dyn/{}", RauthyConfig::get().issuer, id)
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
