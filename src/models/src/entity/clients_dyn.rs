use crate::app_state::{AppState, DbTxn};
use crate::cache::{Cache, DB};
use crate::entity::clients::Client;
use actix_web::web;
use chrono::Utc;
use cryptr::EncValue;
use rauthy_common::constants::{
    CACHE_TTL_DYN_CLIENT, CACHE_TTL_IP_RATE_LIMIT, DYN_CLIENT_SECRET_AUTO_ROTATE,
};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow};
use std::net::IpAddr;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ClientDyn {
    pub id: String,
    pub created: i64,
    pub last_used: Option<i64>,
    pub registration_token: Vec<u8>,
    pub token_endpoint_auth_method: String,
}

impl ClientDyn {
    pub async fn create(
        txn: &mut DbTxn<'_>,
        id: String,
        token_endpoint_auth_method: String,
    ) -> Result<Self, ErrorResponse> {
        let (_secret_plain, registration_token) = Client::generate_new_secret()?;
        let created = Utc::now().timestamp();

        query!(
            r#"INSERT INTO
            clients_dyn (id, created, registration_token, token_endpoint_auth_method)
            VALUES ($1, $2, $3, $4)"#,
            id,
            created,
            registration_token,
            token_endpoint_auth_method,
        )
        .execute(&mut **txn)
        .await?;

        Ok(Self {
            id,
            created,
            last_used: None,
            registration_token,
            token_endpoint_auth_method,
        })
    }

    /// This only deletes a `ClientDyn` from the cache.
    /// The deletion at database level happens via the foreign key cascade.
    pub async fn delete_from_cache(id: &str) -> Result<(), ErrorResponse> {
        DB::client()
            .delete(Cache::App, ClientDyn::get_cache_entry(id))
            .await?;
        Ok(())
    }

    pub async fn find(data: &web::Data<AppState>, id: String) -> Result<Self, ErrorResponse> {
        let client = DB::client();

        if let Some(slf) = client
            .get(Cache::ClientDynamic, ClientDyn::get_cache_entry(&id))
            .await?
        {
            return Ok(slf);
        }

        let slf = query_as!(Self, "SELECT * FROM clients_dyn WHERE id = $1", id)
            .fetch_one(&data.db)
            .await?;

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

    pub async fn update(
        &mut self,
        txn: &mut DbTxn<'_>,
        token_endpoint_auth_method: String,
    ) -> Result<(), ErrorResponse> {
        self.token_endpoint_auth_method = token_endpoint_auth_method;
        self.last_used = Some(Utc::now().timestamp());

        if *DYN_CLIENT_SECRET_AUTO_ROTATE {
            let (_secret_plain, registration_token) = Client::generate_new_secret()?;
            self.registration_token = registration_token;
        }

        query!(
            r#"UPDATE clients_dyn
            SET registration_token = $1, token_endpoint_auth_method = $2, last_used = $3
            WHERE id = $4"#,
            self.registration_token,
            self.token_endpoint_auth_method,
            self.last_used,
            self.id,
        )
        .execute(&mut **txn)
        .await?;

        DB::client()
            .put(
                Cache::ClientDynamic,
                ClientDyn::get_cache_entry(&self.id),
                self,
                *CACHE_TTL_DYN_CLIENT,
            )
            .await?;

        Ok(())
    }

    pub async fn update_used(data: &web::Data<AppState>, id: &str) -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp();

        query!(
            "UPDATE clients_dyn SET last_used = $1 WHERE id = $2",
            now,
            id
        )
        .execute(&data.db)
        .await?;

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
        let client = DB::client();

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
