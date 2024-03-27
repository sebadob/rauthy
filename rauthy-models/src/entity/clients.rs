use crate::app_state::{AppState, DbTxn};
use crate::entity::clients_dyn::ClientDyn;
use crate::entity::jwk::JwkKeyPairAlg;
use crate::entity::scopes::Scope;
use crate::entity::users::User;
use crate::request::{DynamicClientRequest, EphemeralClientRequest, NewClientRequest};
use crate::response::DynamicClientResponse;
use crate::ListenScheme;
use actix_multipart::Multipart;
use actix_web::http::header;
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{web, HttpRequest};
use cryptr::{utils, EncKeys, EncValue};
use futures_util::StreamExt;
use rauthy_common::constants::{
    APPLICATION_JSON, CACHE_NAME_12HR, CACHE_NAME_EPHEMERAL_CLIENTS,
    DYN_CLIENT_DEFAULT_TOKEN_LIFETIME, DYN_CLIENT_SECRET_AUTO_ROTATE, ENABLE_EPHEMERAL_CLIENTS,
    EPHEMERAL_CLIENTS_ALLOWED_FLOWS, EPHEMERAL_CLIENTS_ALLOWED_SCOPES, EPHEMERAL_CLIENTS_FORCE_MFA,
    IDX_CLIENTS, IDX_CLIENT_LOGO, PROXY_MODE, RAUTHY_VERSION,
};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::{cache_entry_client, get_client_ip, get_rand};
use redhac::{
    cache_del, cache_get, cache_get_from, cache_get_value, cache_insert, cache_put, cache_remove,
    AckLevel,
};
use reqwest::header::CONTENT_TYPE;
use reqwest::{tls, Url};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use std::str::FromStr;
use std::sync::OnceLock;
use std::time::Duration;
use tracing::{debug, error, warn};
use utoipa::ToSchema;
use validator::Validate;

const RAUTHY_DEFAULT_LOGO: &str = "data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiIHN0YW5kYWxvbmU9Im5vIj8+CjwhRE9DVFlQRSBzdmcgUFVCTElDICItLy9XM0MvL0RURCBTVkcgMS4xLy9FTiIgImh0dHA6Ly93d3cudzMub3JnL0dyYXBoaWNzL1NWRy8xLjEvRFREL3N2ZzExLmR0ZCI+Cjxzdmcgd2lkdGg9IjEwMCUiIGhlaWdodD0iMTAwJSIgdmlld0JveD0iMCAwIDUxMiAxMzgiIHZlcnNpb249IjEuMSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIiB4bWxuczp4bGluaz0iaHR0cDovL3d3dy53My5vcmcvMTk5OS94bGluayIgeG1sOnNwYWNlPSJwcmVzZXJ2ZSIgeG1sbnM6c2VyaWY9Imh0dHA6Ly93d3cuc2VyaWYuY29tLyIgc3R5bGU9ImZpbGwtcnVsZTpldmVub2RkO2NsaXAtcnVsZTpldmVub2RkO3N0cm9rZS1saW5lY2FwOnJvdW5kO3N0cm9rZS1saW5lam9pbjpyb3VuZDtzdHJva2UtbWl0ZXJsaW1pdDoxLjU7Ij4KICAgIDxnIHRyYW5zZm9ybT0ibWF0cml4KDEsMCwwLDEsMCwtMTEpIj4KICAgICAgICA8ZyB0cmFuc2Zvcm09Im1hdHJpeCgxLDAsMCwxLDAsLTE3NikiPgogICAgICAgICAgICA8ZyB0cmFuc2Zvcm09Im1hdHJpeCgwLjkyMDMyNSwwLDAsMS44NDE1MSw0NS45Mjc5LDI2LjQ1OSkiPgogICAgICAgICAgICAgICAgPHJlY3QgeD0iMjcuNzQxIiB5PSIxNTEuNTciIHdpZHRoPSIyMDAuNTE3IiBoZWlnaHQ9IjEwLjE0OCIgc3R5bGU9ImZpbGw6cmdiKDQsNywxMSk7Ii8+CiAgICAgICAgICAgIDwvZz4KICAgICAgICAgICAgPGcgdHJhbnNmb3JtPSJtYXRyaXgoMS45MzQ3MiwwLDAsMS44MjczMiw4LjM1NjE4LDI4Ljc1MzMpIj4KICAgICAgICAgICAgICAgIDxyZWN0IHg9IjMzLjMwNyIgeT0iOTcuMTUiIHdpZHRoPSI5NC42OTMiIGhlaWdodD0iNTQuNDIiIHN0eWxlPSJmaWxsOnJnYig0LDcsMTEpO3N0cm9rZTpyZ2IoNCw3LDExKTtzdHJva2Utd2lkdGg6MS4wNnB4OyIvPgogICAgICAgICAgICA8L2c+CiAgICAgICAgICAgIDxnIHRyYW5zZm9ybT0ibWF0cml4KDEuODI3MzIsMCwwLDEuODI3MzIsLTE2MC44MjIsNzAuMTgwNikiPgogICAgICAgICAgICAgICAgPGcgdHJhbnNmb3JtPSJtYXRyaXgoNzIsMCwwLDcyLDIyNy4xNzQsMTIzLjQxNykiPgogICAgICAgICAgICAgICAgPC9nPgogICAgICAgICAgICAgICAgPHRleHQgeD0iMTI4Ljk4MnB4IiB5PSIxMjMuNDE3cHgiIHN0eWxlPSJmb250LWZhbWlseTonQ2FsaWJyaS1Cb2xkJywgJ0NhbGlicmknLCBzYW5zLXNlcmlmO2ZvbnQtd2VpZ2h0OjcwMDtmb250LXNpemU6NzJweDtmaWxsOndoaXRlOyI+cjx0c3BhbiB4PSIxNTIuOTk0cHggMTg4LjUzN3B4ICIgeT0iMTIzLjQxN3B4IDEyMy40MTdweCAiPmF1PC90c3Bhbj48L3RleHQ+CiAgICAgICAgICAgIDwvZz4KICAgICAgICAgICAgPGcgdHJhbnNmb3JtPSJtYXRyaXgoMSwwLDAsMS4wMTYxNywtMS40MjEwOWUtMTQsLTUuMjQ0OTIpIj4KICAgICAgICAgICAgICAgIDxwYXRoIGQ9Ik00NDAuOTM2LDMyMi42NDNMNDM5LjIwNCwzMjQuMjY2TDI1NS40ODIsMzI0LjI2NkwyNTUuNDgyLDMwNS43MjFMNDQwLjkzNiwzMDUuNzIxTDQ0MC45MzYsMzIyLjY0M1oiIHN0eWxlPSJmaWxsOnVybCgjX0xpbmVhcjEpOyIvPgogICAgICAgICAgICA8L2c+CiAgICAgICAgICAgIDxnIHRyYW5zZm9ybT0ibWF0cml4KDAuOTIwMTkxLDAsMCwxLjg0MTIxLDQ2LjI0NjQsLTkxLjMzODMpIj4KICAgICAgICAgICAgICAgIDxyZWN0IHg9IjI3Ljc0MSIgeT0iMTUxLjU3IiB3aWR0aD0iMjAwLjUxNyIgaGVpZ2h0PSIxMC4xNDgiIHN0eWxlPSJmaWxsOnVybCgjX0xpbmVhcjIpOyIvPgogICAgICAgICAgICA8L2c+CiAgICAgICAgICAgIDxnIHRyYW5zZm9ybT0ibWF0cml4KDEuOTc1OTgsMCwwLDEuODQ2MTksMTkwLjE4NywyNi4wNjIpIj4KICAgICAgICAgICAgICAgIDxyZWN0IHg9IjMzLjMwNyIgeT0iOTcuMTUiIHdpZHRoPSI5NC42OTMiIGhlaWdodD0iNTQuNDIiIHN0eWxlPSJmaWxsOnJnYig0Myw2NSwxMDcpOyIvPgogICAgICAgICAgICA8L2c+CiAgICAgICAgICAgIDxwYXRoIGQ9Ik00MzkuMjA0LDE4Ny43MzRMNDQwLjU1NywxODkuMDA3TDQ0MC41NTcsMjA2LjI3OUwyNTYsMjA2LjI3OUwyNTYsMTg3LjczNEw0MzkuMjA0LDE4Ny43MzRaIiBzdHlsZT0iZmlsbDpyZ2IoNDMsNjUsMTA3KTsiLz4KICAgICAgICAgICAgPGcgdHJhbnNmb3JtPSJtYXRyaXgoMS44MjczMiwwLDAsMS44MjczMiwtMTU0LjY2MSw3MC4xODA2KSI+CiAgICAgICAgICAgICAgICA8ZyB0cmFuc2Zvcm09Im1hdHJpeCg3MiwwLDAsNzIsMzIzLjA0NSwxMjMuNDE3KSI+CiAgICAgICAgICAgICAgICA8L2c+CiAgICAgICAgICAgICAgICA8dGV4dCB4PSIyMjYuNjQ2cHgiIHk9IjEyMy40MTdweCIgc3R5bGU9ImZvbnQtZmFtaWx5OidDYWxpYnJpLUJvbGQnLCAnQ2FsaWJyaScsIHNhbnMtc2VyaWY7Zm9udC13ZWlnaHQ6NzAwO2ZvbnQtc2l6ZTo3MnB4O2ZpbGw6d2hpdGU7Ij50aDx0c3BhbiB4PSIyODguOTQzcHggIiB5PSIxMjMuNDE3cHggIj55PC90c3Bhbj48L3RleHQ+CiAgICAgICAgICAgIDwvZz4KICAgICAgICAgICAgPGcgdHJhbnNmb3JtPSJtYXRyaXgoMiwwLDAsMiwwLDApIj4KICAgICAgICAgICAgICAgIDxwYXRoIGQ9Ik0yMTkuNjAyLDkzLjg2N0wyNTYsMTI4TDIxOS42MDIsMTYyLjEzM0wyMTkuNjAyLDkzLjg2N1oiIHN0eWxlPSJmaWxsOnJnYig0Myw2NSwxMDcpOyIvPgogICAgICAgICAgICA8L2c+CiAgICAgICAgICAgIDxnIHRyYW5zZm9ybT0ibWF0cml4KDIsMCwwLDEuOTU3MzksMCwzLjk5OTk3KSI+CiAgICAgICAgICAgICAgICA8cGF0aCBkPSJNMzYuMzk4LDkzLjg2N0wwLDkzLjg2N0wzNS45MDgsMTI4LjUyNEwwLDE2My42MTlMMzYuMzk4LDE2My42MTkiIHN0eWxlPSJmaWxsOnJnYig0LDcsMTEpOyIvPgogICAgICAgICAgICA8L2c+CiAgICAgICAgPC9nPgogICAgPC9nPgogICAgPGRlZnM+CiAgICAgICAgPGxpbmVhckdyYWRpZW50IGlkPSJfTGluZWFyMSIgeDE9IjAiIHkxPSIwIiB4Mj0iMSIgeTI9IjAiIGdyYWRpZW50VW5pdHM9InVzZXJTcGFjZU9uVXNlIiBncmFkaWVudFRyYW5zZm9ybT0ibWF0cml4KDE4NS40NTQsMCwwLDE4LjU0NDMsMjU1LjQ4MiwzMTQuOTk0KSI+PHN0b3Agb2Zmc2V0PSIwIiBzdHlsZT0ic3RvcC1jb2xvcjpyZ2IoNCw3LDExKTtzdG9wLW9wYWNpdHk6MSIvPjxzdG9wIG9mZnNldD0iMSIgc3R5bGU9InN0b3AtY29sb3I6cmdiKDQzLDY1LDEwNyk7c3RvcC1vcGFjaXR5OjEiLz48L2xpbmVhckdyYWRpZW50PgogICAgICAgIDxsaW5lYXJHcmFkaWVudCBpZD0iX0xpbmVhcjIiIHgxPSIwIiB5MT0iMCIgeDI9IjEiIHkyPSIwIiBncmFkaWVudFVuaXRzPSJ1c2VyU3BhY2VPblVzZSIgZ3JhZGllbnRUcmFuc2Zvcm09Im1hdHJpeCgyMDAuNTE3LDAsMCwxMC4xNDgzLDI3Ljc0MTQsMTU2LjY0NSkiPjxzdG9wIG9mZnNldD0iMCIgc3R5bGU9InN0b3AtY29sb3I6cmdiKDQsNywxMSk7c3RvcC1vcGFjaXR5OjEiLz48c3RvcCBvZmZzZXQ9IjEiIHN0eWxlPSJzdG9wLWNvbG9yOnJnYig0Myw2NSwxMDcpO3N0b3Atb3BhY2l0eToxIi8+PC9saW5lYXJHcmFkaWVudD4KICAgIDwvZGVmcz4KPC9zdmc+Cg==";

static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

/**
# OIDC Client

A few values here are saved as CSV Strings instead of having foreign keys and links to other
tables.
All deleting and modifying operations are a bit more expensive this way, but we gain a lot of
performance, when we do reads on clients, which we do most of the time.

`*_lifetime` values are meant to be in seconds.
 */
#[derive(Debug, Clone, PartialEq, Eq, FromRow, Deserialize, Serialize, ToSchema)]
pub struct Client {
    pub id: String,
    pub name: Option<String>,
    pub enabled: bool,
    pub confidential: bool,
    // The client secrets is saved as encrypted bytes and can only be decrypted inside this
    // application itself with the `ENC_KEYS` and `ENC_KEY_ACTIVE` environment variables.
    pub secret: Option<Vec<u8>>,
    pub secret_kid: Option<String>,
    pub redirect_uris: String,
    pub post_logout_redirect_uris: Option<String>,
    pub allowed_origins: Option<String>,
    pub flows_enabled: String,
    // Currently supported Algorithms: RS 256, 384, 512 and ES 256, 384, 512
    pub access_token_alg: String,
    // Currently supported Algorithms: RS 256, 384, 512 and ES 256, 384, 512
    pub id_token_alg: String,
    pub refresh_token: bool,
    pub auth_code_lifetime: i32,
    pub access_token_lifetime: i32,
    pub scopes: String,
    pub default_scopes: String,
    pub challenge: Option<String>,
    pub force_mfa: bool,
}

// CRUD
impl Client {
    pub fn get_cache_entry(id: &str) -> String {
        format!("client_{}", id)
    }

    pub async fn create(
        data: &web::Data<AppState>,
        mut client_req: NewClientRequest,
    ) -> Result<Self, ErrorResponse> {
        let kid = if client_req.confidential {
            let (_cleartext, enc) = Self::generate_new_secret()?;
            client_req.secret = Some(enc);
            Some(EncKeys::get_static().enc_key_active.clone())
        } else {
            None
        };
        let mut client = Client::from(client_req);
        client.secret_kid = kid;

        let rows =  sqlx::query!(
            r#"insert into clients (id, name, enabled, confidential, secret, secret_kid,
            redirect_uris, post_logout_redirect_uris, allowed_origins, flows_enabled, access_token_alg,
            id_token_alg, refresh_token, auth_code_lifetime, access_token_lifetime, scopes, default_scopes,
            challenge, force_mfa)
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)"#,
            client.id,
            client.name,
            client.enabled,
            client.confidential,
            client.secret,
            client.secret_kid,
            client.redirect_uris,
            client.post_logout_redirect_uris,
            client.allowed_origins,
            client.flows_enabled,
            client.access_token_alg,
            client.id_token_alg,
            client.refresh_token,
            client.auth_code_lifetime,
            client.access_token_lifetime,
            client.scopes,
            client.default_scopes,
            client.challenge,
            client.force_mfa,
        )
            .execute(&data.db)
            .await?
            .rows_affected();

        if rows == 0 {
            error!("Error inserting client - no rows affected");
        }

        // TODO remove all clients cache fully? -> admin UI only?
        // let mut clients = Client::find_all(data).await?;
        // clients.push(client.clone());
        // cache_insert(
        //     CACHE_NAME_12HR.to_string(),
        //     IDX_CLIENTS.to_string(),
        //     &data.caches.ha_cache_config,
        //     &clients,
        //     AckLevel::Leader,
        // )
        // .await?;

        Ok(client)
    }

    pub async fn create_dynamic(
        data: &web::Data<AppState>,
        client_req: DynamicClientRequest,
    ) -> Result<DynamicClientResponse, ErrorResponse> {
        let token_endpoint_auth_method = client_req
            .token_endpoint_auth_method
            .clone()
            .unwrap_or_else(|| "client_secret_basic".to_string());

        let client = Self::try_from_dyn_reg(client_req)?;

        let mut txn = data.db.begin().await?;

        sqlx::query!(
            r#"INSERT INTO clients (id, name, enabled, confidential, secret, secret_kid,
            redirect_uris, post_logout_redirect_uris, allowed_origins, flows_enabled, access_token_alg,
            id_token_alg, refresh_token, auth_code_lifetime, access_token_lifetime, scopes, default_scopes,
            challenge, force_mfa)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)"#,
            client.id,
            client.name,
            client.enabled,
            client.confidential,
            client.secret,
            client.secret_kid,
            client.redirect_uris,
            client.post_logout_redirect_uris,
            client.allowed_origins,
            client.flows_enabled,
            client.access_token_alg,
            client.id_token_alg,
            client.refresh_token,
            client.auth_code_lifetime,
            client.access_token_lifetime,
            client.scopes,
            client.default_scopes,
            client.challenge,
            client.force_mfa,
        )
            .execute(&mut *txn)
            .await?;

        let client_dyn = ClientDyn::create(
            &mut txn,
            client.id.clone(),
            token_endpoint_auth_method.clone(),
        )
        .await?;

        txn.commit().await?;

        // let mut clients = Client::find_all(data).await?;
        // clients.push(client.clone());
        // cache_insert(
        //     CACHE_NAME_12HR.to_string(),
        //     IDX_CLIENTS.to_string(),
        //     &data.caches.ha_cache_config,
        //     &clients,
        //     AckLevel::Leader,
        // )
        // .await?;

        DynamicClientResponse::build(data, client, client_dyn, true)
    }

    // Deletes a client
    pub async fn delete(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        sqlx::query!("delete from clients where id = $1", self.id,)
            .execute(&data.db)
            .await?;

        // let clients = Client::find_all(data)
        //     .await?
        //     .into_iter()
        //     .filter(|c| c.id != self.id)
        //     .collect::<Vec<Self>>();
        //
        // cache_insert(
        //     CACHE_NAME_12HR.to_string(),
        //     IDX_CLIENTS.to_string(),
        //     &data.caches.ha_cache_config,
        //     &clients,
        //     AckLevel::Quorum,
        // )
        // .await?;
        cache_remove(
            CACHE_NAME_12HR.to_string(),
            Client::get_cache_entry(&self.id),
            &data.caches.ha_cache_config,
            AckLevel::Leader,
        )
        .await?;

        // We only clean up the cache. The database uses foreign key a cascade.
        if self.is_dynamic() {
            ClientDyn::delete_from_cache(data, &self.id).await?;
        }

        Ok(())
    }

    // Returns a client by id without its secret.
    pub async fn find(data: &web::Data<AppState>, id: String) -> Result<Self, ErrorResponse> {
        let client = cache_get!(
            Client,
            CACHE_NAME_12HR.to_string(),
            cache_entry_client(&id),
            &data.caches.ha_cache_config,
            false
        )
        .await?;
        if let Some(client) = client {
            return Ok(client);
        }

        let client = sqlx::query_as::<_, Self>("select * from clients where id = $1")
            .bind(&id)
            .fetch_one(&data.db)
            .await?;

        cache_insert(
            CACHE_NAME_12HR.to_string(),
            Client::get_cache_entry(&client.id),
            &data.caches.ha_cache_config,
            &client,
            AckLevel::Leader,
        )
        .await?;

        Ok(client)
    }

    // Returns all existing clients with the secrets.
    pub async fn find_all(data: &web::Data<AppState>) -> Result<Vec<Self>, ErrorResponse> {
        // let clients = cache_get!(
        //     Vec<Client>,
        //     CACHE_NAME_12HR.to_string(),
        //     IDX_CLIENTS.to_string(),
        //     &data.caches.ha_cache_config,
        //     false
        // )
        // .await?;
        // if let Some(clients) = clients {
        //     return Ok(clients);
        // }

        let clients = sqlx::query_as("select * from clients")
            .fetch_all(&data.db)
            .await?;

        // cache_insert(
        //     CACHE_NAME_12HR.to_string(),
        //     IDX_CLIENTS.to_string(),
        //     &data.caches.ha_cache_config,
        //     &clients,
        //     AckLevel::Leader,
        // )
        // .await?;

        Ok(clients)
    }

    /// Accepts either a pre-registered client_id or a URL as such.
    /// If allowed, it will dynamically build an ephemeral client and cache it, it the client_id
    /// is a URL. Otherwise, it will do a classic fetch from the database.
    /// This function should be used in places where we would possibly accept an ephemeral client.
    pub async fn find_maybe_ephemeral(
        data: &web::Data<AppState>,
        id: String,
    ) -> Result<Self, ErrorResponse> {
        if !*ENABLE_EPHEMERAL_CLIENTS || Url::from_str(&id).is_err() {
            return Self::find(data, id).await;
        }

        if let Some(client) = cache_get!(
            Client,
            CACHE_NAME_EPHEMERAL_CLIENTS.to_string(),
            id.clone(),
            &data.caches.ha_cache_config,
            false
        )
        .await?
        {
            return Ok(client);
        }

        let client = Self::ephemeral_from_url(&id).await?;

        cache_put(
            CACHE_NAME_EPHEMERAL_CLIENTS.to_string(),
            id,
            &data.caches.ha_cache_config,
            &client,
        )
        .await?;

        Ok(client)
    }

    pub async fn find_logo(data: &web::Data<AppState>, id: &str) -> Result<String, ErrorResponse> {
        let idx = format!("{}{}", IDX_CLIENT_LOGO, id);
        let logo = cache_get!(
            String,
            CACHE_NAME_12HR.to_string(),
            idx.clone(),
            &data.caches.ha_cache_config,
            false
        )
        .await?;
        if let Some(logo) = logo {
            return Ok(logo);
        }

        let logo_opt = sqlx::query("select data from logos where client_id = $1")
            .bind(id)
            .fetch_optional(&data.db)
            .await?;

        let logo = match logo_opt {
            None => RAUTHY_DEFAULT_LOGO.to_string(),
            Some(row) => row.get("data"),
        };

        cache_put(
            CACHE_NAME_12HR.to_string(),
            idx,
            &data.caches.ha_cache_config,
            &logo,
        )
        .await?;

        Ok(logo)
    }

    pub async fn save_logo(
        data: &web::Data<AppState>,
        id: &str,
        logo: String,
    ) -> Result<(), ErrorResponse> {
        #[cfg(feature = "sqlite")]
        let q = sqlx::query!(
            "insert or replace into logos (client_id, data) values ($1, $2)",
            id,
            logo
        );
        #[cfg(not(feature = "sqlite"))]
        let q = sqlx::query!(
            r#"insert into logos (client_id, data) values ($1, $2)
                on conflict(client_id) do update set data = $2"#,
            id,
            logo
        );

        q.execute(&data.db).await?;

        let idx = format!("{}{}", IDX_CLIENT_LOGO, id);
        cache_put(
            CACHE_NAME_12HR.to_string(),
            idx,
            &data.caches.ha_cache_config,
            &logo,
        )
        .await?;

        Ok(())
    }

    pub async fn delete_logo(data: &web::Data<AppState>, id: &str) -> Result<(), ErrorResponse> {
        let idx = format!("{}{}", IDX_CLIENT_LOGO, id);
        cache_del(
            CACHE_NAME_12HR.to_string(),
            idx,
            &data.caches.ha_cache_config,
        )
        .await?;

        sqlx::query!("delete from logos where client_id = $1", id)
            .execute(&data.db)
            .await?;

        Ok(())
    }

    pub async fn save(
        &self,
        data: &web::Data<AppState>,
        txn: Option<&mut DbTxn<'_>>,
    ) -> Result<(), ErrorResponse> {
        let q = sqlx::query!(
            r#"update clients set name = $1, enabled = $2, confidential = $3, secret = $4,
            secret_kid = $5, redirect_uris = $6, post_logout_redirect_uris = $7, allowed_origins = $8,
            flows_enabled = $9, access_token_alg = $10, id_token_alg = $11, refresh_token = $12,
            auth_code_lifetime = $13, access_token_lifetime = $14, scopes = $15, default_scopes = $16,
            challenge = $17, force_mfa= $18 where id = $19"#,
            self.name,
            self.enabled,
            self.confidential,
            self.secret,
            self.secret_kid,
            self.redirect_uris,
            self.post_logout_redirect_uris,
            self.allowed_origins,
            self.flows_enabled,
            self.access_token_alg,
            self.id_token_alg,
            self.refresh_token,
            self.auth_code_lifetime,
            self.access_token_lifetime,
            self.scopes,
            self.default_scopes,
            self.challenge,
            self.force_mfa,
            self.id,
        );

        if let Some(txn) = txn {
            q.execute(&mut **txn).await?;
        } else {
            q.execute(&data.db).await?;
        }

        cache_put(
            CACHE_NAME_12HR.to_string(),
            Client::get_cache_entry(&self.id),
            &data.caches.ha_cache_config,
            &self,
        )
        .await?;

        let mut found_self = false;
        let mut clients = Client::find_all(data)
            .await?
            .into_iter()
            .map(|mut c| {
                if c.id == self.id {
                    found_self = true;
                    c = self.clone();
                }
                c
            })
            .collect::<Vec<Self>>();
        if !found_self {
            clients.push(self.clone());
        }
        cache_insert(
            CACHE_NAME_12HR.to_string(),
            IDX_CLIENTS.to_string(),
            &data.caches.ha_cache_config,
            &clients,
            AckLevel::Quorum,
        )
        .await?;

        Ok(())
    }

    pub async fn update_dynamic(
        data: &web::Data<AppState>,
        client_req: DynamicClientRequest,
        mut client_dyn: ClientDyn,
    ) -> Result<DynamicClientResponse, ErrorResponse> {
        let token_endpoint_auth_method = client_req
            .token_endpoint_auth_method
            .clone()
            .unwrap_or_else(|| "client_secret_basic".to_string());

        let mut new_client = Self::try_from_dyn_reg(client_req)?;
        let current = Self::find(data, client_dyn.id.clone()).await?;
        if !current.is_dynamic() {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Invalid request for non-dynamic client".to_string(),
            ));
        }

        // we need to keep some old and possibly user-modified values
        new_client.id = current.id;
        new_client.force_mfa = current.force_mfa;
        new_client.scopes = current.scopes;
        new_client.default_scopes = current.default_scopes;

        let mut txn = data.db.begin().await?;
        new_client.save(data, Some(&mut txn)).await?;
        client_dyn
            .update(data, &mut txn, token_endpoint_auth_method)
            .await?;
        txn.commit().await?;

        DynamicClientResponse::build(data, new_client, client_dyn, *DYN_CLIENT_SECRET_AUTO_ROTATE)
    }
}

impl Client {
    // TODO make a generic 'delete_from_csv' function out of this and re-use it in some other places
    pub fn delete_scope(&mut self, scope: &str) {
        // find the scope via index in the string
        // first entry: delete scope + ',' if it exists
        // last entry: delete scope + ',' in front if it exists
        // middle: delete scope + ',' in front if it exists
        // --> 2 cases: first entry or else
        let i_opt = self.scopes.find(scope);
        if i_opt.is_none() {
            return;
        }

        let i = i_opt.unwrap();
        let len = scope.bytes().len();
        if i == 0 {
            // the scope is the first entry
            if self.scopes.len() > len {
                let s = format!("{},", scope);
                self.scopes = self.scopes.replace(&s, "");
            } else {
                self.scopes = String::from("");
            }
        } else {
            // the scope is at the end or in the middle
            let s = format!(",{}", scope);
            self.scopes = self.scopes.replace(&s, "");
        }

        // check if the to-be-deleted scope is in the default scopes
        let i_opt = self.default_scopes.find(scope);
        if i_opt.is_none() {
            return;
        }

        let i = i_opt.unwrap();
        if i == 0 {
            // the scope is the first entry
            if self.default_scopes.len() > len {
                let s = format!("{},", scope);
                self.default_scopes = self.default_scopes.replace(&s, "");
            } else {
                self.default_scopes = String::from("");
            }
        } else {
            // the scope is at the end or in the middle
            let s = format!(",{}", scope);
            self.default_scopes = self.default_scopes.replace(&s, "");
        }
    }

    // Generates a new random 64 character long client secret and returns the cleartext and
    /// encrypted version
    /// # Panics
    /// The decryption depends on correctly set up `ENC_KEYS` and `ENC_KEY_ACTIVE` environment
    /// variables and panics, if this is not the case.
    #[inline]
    pub fn generate_new_secret() -> Result<(String, Vec<u8>), ErrorResponse> {
        let rnd = utils::secure_random_alnum(64);
        let enc = EncValue::encrypt(rnd.as_bytes())?.into_bytes().to_vec();
        Ok((rnd, enc))
    }

    #[inline]
    pub fn get_access_token_alg(&self) -> Result<JwkKeyPairAlg, ErrorResponse> {
        JwkKeyPairAlg::from_str(self.access_token_alg.as_str())
    }

    pub fn get_allowed_origins(&self) -> Option<Vec<String>> {
        self.allowed_origins.as_ref()?;
        let mut origins = Vec::new();
        self.allowed_origins
            .as_ref()
            .unwrap()
            .split(',')
            .for_each(|o| origins.push(o.trim().to_owned()));
        Some(origins)
    }

    pub fn get_challenges(&self) -> Option<Vec<String>> {
        self.challenge.as_ref()?;

        let mut res = Vec::new();
        self.challenge
            .as_ref()
            .unwrap()
            .split(',')
            .for_each(|c| res.push(c.trim().to_owned()));
        Some(res)
    }

    /// Decrypts the client secret (if it exists) and then returns it as clear text.
    pub fn get_secret_cleartext(&self) -> Result<Option<String>, ErrorResponse> {
        if let Some(secret) = self.secret.as_ref() {
            let bytes = EncValue::try_from(secret.clone())?.decrypt()?;
            let cleartext = String::from_utf8_lossy(bytes.as_ref()).to_string();
            Ok(Some(cleartext))
        } else {
            Ok(None)
        }
    }

    pub fn get_default_scopes(&self) -> Vec<String> {
        let mut res = Vec::new();
        self.default_scopes
            .split(',')
            .for_each(|s| res.push(s.trim().to_owned()));
        res
    }

    pub fn get_id_token_alg(&self) -> Result<JwkKeyPairAlg, ErrorResponse> {
        JwkKeyPairAlg::from_str(self.id_token_alg.as_str())
    }

    pub fn get_flows(&self) -> Vec<String> {
        let mut res = Vec::new();
        self.flows_enabled
            .split(',')
            .map(|f| f.trim().to_owned())
            .for_each(|f| res.push(f));
        res
    }

    pub fn get_post_logout_uris(&self) -> Option<Vec<String>> {
        self.post_logout_redirect_uris.as_ref()?;
        Some(
            self.post_logout_redirect_uris
                .as_ref()
                .unwrap()
                .split(',')
                .map(|i| i.trim().to_string())
                .collect(),
        )
    }

    pub fn get_redirect_uris(&self) -> Vec<String> {
        self.redirect_uris
            .split(',')
            .map(|i| i.trim().to_string())
            .collect()
    }

    pub fn get_scopes(&self) -> Vec<String> {
        let mut res = Vec::new();
        self.scopes
            .split(',')
            .for_each(|s| res.push(s.trim().to_owned()));
        res
    }

    pub fn get_scope_as_str(&self) -> String {
        self.scopes.replace(',', " ")
    }

    pub fn is_dynamic(&self) -> bool {
        self.id.starts_with("dyn$")
    }

    pub fn is_ephemeral(&self) -> bool {
        Url::from_str(&self.id).is_ok()
    }

    /// Sanitizes the current scopes and deletes everything, which does not exist in the `scopes`
    /// table in the database
    pub async fn sanitize_scopes(
        data: &web::Data<AppState>,
        scps: Vec<String>,
    ) -> Result<String, ErrorResponse> {
        let mut res = String::with_capacity(scps.len());
        Scope::find_all(data).await?.into_iter().for_each(|s| {
            if scps.contains(&s.name) {
                res.push_str(s.name.as_str());
                res.push(',');
            }
        });
        // remove the last comma
        if !res.is_empty() {
            res.remove(res.len() - 1);
        }
        // check for 'openid', which should always be there
        if res.is_empty() {
            res = "openid".to_string();
        } else if !res.contains("openid") {
            res = format!("openid,{}", res);
        }
        Ok(res)
    }

    /// Sanitizes the requested scopes on the authorization endpoint and matches them to the
    /// allowed scopes for this client.
    pub fn sanitize_login_scopes(
        &self,
        scopes: &Option<Vec<String>>,
    ) -> Result<Vec<String>, ErrorResponse> {
        if scopes.is_none() {
            return Ok(self
                .default_scopes
                .split(',')
                .map(|s| s.to_string())
                .collect());
        }

        let scopes = scopes.as_ref().unwrap();
        let mut res = Vec::with_capacity(scopes.len());

        // Always add the configured default scopes
        for s in self.default_scopes.split(',') {
            res.push(s.to_string());
        }

        for s in scopes {
            if self.default_scopes.contains(s) {
                continue;
            }

            if self.scopes.contains(s) {
                res.push(s.clone());
            }
        }

        Ok(res)
    }

    pub async fn upload_logo(
        data: &web::Data<AppState>,
        client_id: &str,
        mut payload: Multipart,
    ) -> Result<(), ErrorResponse> {
        let mut buf: Vec<u8> = Vec::with_capacity(1024);

        while let Some(item) = payload.next().await {
            let mut field = item?;
            // let content_type = field.content_disposition();

            while let Some(chunk) = field.next().await {
                let bytes = chunk?;
                buf.extend(bytes);
            }
        }

        let logo_str = match String::from_utf8(buf) {
            Ok(l) => l,
            Err(err) => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    format!("Cannot parse logo: {:?}", err),
                ));
            }
        };

        Self::save_logo(data, client_id, logo_str).await?;

        Ok(())
    }

    /// Validates the User's access to this client depending on the `force_mfa` setting.
    /// Do this check after a possible password hash to not leak information to unauthenticated users!
    ///
    /// The "rauthy" client is the exception for this check to makes logging into the account
    /// possible without MFA. The force MFA for the Rauthy admin UI is done in
    /// Principal::validate_admin_session() depending on the `ADMIN_FORCE_MFA` config variable.
    pub fn validate_mfa(&self, user: &User) -> Result<(), ErrorResponse> {
        if &self.id != "rauthy" && self.force_mfa && !user.has_webauthn_enabled() {
            Err(ErrorResponse::new(
                ErrorResponseType::MfaRequired,
                "MFA is required for this client".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    // Validates the `Origin` HTTP Header from an incoming request and compares it to the
    // `allowed_origins`. If the Origin is an external one and allowed by the config, it returns
    // the correct `ACCESS_CONTROL_ALLOW_ORIGIN` header which can then be inserted into the
    // HttpResponse.
    pub fn validate_origin(
        &self,
        r: &HttpRequest,
        listen_scheme: &ListenScheme,
        pub_url: &str,
    ) -> Result<Option<(HeaderName, HeaderValue)>, ErrorResponse> {
        let (is_ext, origin) = is_origin_external(r, listen_scheme, pub_url)?;
        if !is_ext {
            return Ok(None);
        }

        let err_msg = || {
            debug!("Origin err msg is being created");
            Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!(
                    "Coming from an external Origin '{}' which is not allowed",
                    origin
                ),
            ))
        };

        if self.allowed_origins.is_none() {
            debug!("Allowed origins is None");
            return err_msg();
        }

        let allowed_origins = self
            .allowed_origins
            .as_ref()
            .unwrap()
            .split(',')
            .filter(|&ao| {
                // in this case, we should accept http and https, so we just execute .ends_with
                if listen_scheme == &ListenScheme::HttpHttps {
                    ao.ends_with(origin)
                } else {
                    ao.eq(origin)
                }
            })
            .count();
        if allowed_origins == 0 {
            debug!("No match found for allowed origin");
            return err_msg();
        }

        Ok(Some((
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_str(origin).unwrap(),
        )))
    }

    pub fn validate_redirect_uri(&self, redirect_uri: &str) -> Result<(), ErrorResponse> {
        let matching_uris = self
            .get_redirect_uris()
            .iter()
            .filter(|uri| {
                (uri.ends_with('*') && redirect_uri.starts_with(uri.split_once('*').unwrap().0))
                    || uri.as_str().eq(redirect_uri)
                // if (uri.ends_with('*') && redirect_uri.starts_with(uri.split_once('*').unwrap().0))
                //     || uri.eq(redirect_uri)
                // {
                //     return true;
                // }
                // false
            })
            .count();
        if matching_uris == 0 {
            Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                String::from("Invalid redirect uri"),
            ))
        } else {
            Ok(())
        }
    }

    pub fn validate_code_challenge(
        &self,
        code_challenge: &Option<String>,
        code_challenge_method: &Option<String>,
    ) -> Result<(), ErrorResponse> {
        if self.challenge.is_some() {
            if code_challenge.is_none() {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    String::from("'code_challenge' is missing"),
                ));
            }

            if code_challenge_method.is_none() {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    String::from("'code_challenge_method' is missing"),
                ));
            }

            let method = code_challenge_method.as_ref().unwrap();
            if !self.challenge.as_ref().unwrap().contains(method) {
                Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    format!("code_challenge_method '{}' is not allowed", method),
                ))
            } else {
                Ok(())
            }
        } else if code_challenge.is_some() || code_challenge_method.is_some() {
            Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "'code_challenge' not enabled for this client".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    pub fn validate_challenge_method(
        &self,
        code_challenge_method: &str,
    ) -> Result<(), ErrorResponse> {
        if self.challenge.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                String::from("'code_challenge' not allowed"),
            ));
        }
        if code_challenge_method.is_empty()
            || !self
                .challenge
                .as_ref()
                .unwrap()
                .contains(code_challenge_method)
        {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!(
                    "code_challenge_method '{}' is not allowed",
                    code_challenge_method
                ),
            ));
        }
        Ok(())
    }

    pub fn validate_flow(&self, flow: &str) -> Result<(), ErrorResponse> {
        if flow.is_empty() || !self.flows_enabled.contains(flow) {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!("'{}' flow is not allowed for this client", flow),
            ));
        }
        Ok(())
    }

    pub fn validate_secret(&self, secret: &str, req: &HttpRequest) -> Result<(), ErrorResponse> {
        if !self.confidential {
            error!("Cannot validate 'client_secret' for public client");
            return Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                String::from("Cannot validate 'client_secret' for public client"),
            ));
        }

        let secret_enc = self.secret.as_ref().ok_or_else(|| {
            ErrorResponse::new(
                ErrorResponseType::Internal,
                format!("'{}' has no secret while being confidential", &self.id),
            )
        })?;
        let cleartext = EncValue::try_from(secret_enc.clone())?.decrypt()?;

        if cleartext.as_ref() != secret.as_bytes() {
            drop(cleartext);
            warn!(
                "Invalid login for client '{}' from '{}'",
                self.id,
                get_client_ip(req)
            );

            return Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                String::from("Invalid 'client_secret'"),
            ));
        }
        Ok(())
    }
}

impl Client {
    async fn ephemeral_from_url(value: &str) -> Result<Self, ErrorResponse> {
        let client = HTTP_CLIENT.get_or_init(|| {
            reqwest::Client::builder()
                .connect_timeout(Duration::from_secs(10))
                .timeout(Duration::from_secs(10))
                .user_agent(format!(
                    "Rauthy v{} Ephemeral Client Resolver",
                    RAUTHY_VERSION
                ))
                .min_tls_version(tls::Version::TLS_1_2)
                .pool_idle_timeout(Duration::from_secs(600))
                .build()
                .unwrap()
        });

        let res = client
            .get(value)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .send()
            .await
            .map_err(|err| {
                ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    format!(
                        "Cannot fetch ephemeral client data from {}: {:?}",
                        value, err
                    ),
                )
            })?;

        if !res.status().is_success() {
            let msg = format!("Cannot fetch ephemeral client information from {}", value);
            error!("{}", msg);
            return Err(ErrorResponse::new(ErrorResponseType::Connection, msg));
        }

        let body = match res.json::<EphemeralClientRequest>().await {
            Ok(b) => b,
            Err(err) => {
                let msg = format!(
                    "Cannot deserialize into EphemeralClientRequest from {}: {:?}",
                    value, err,
                );
                error!("{}", msg);
                return Err(ErrorResponse::new(ErrorResponseType::BadRequest, msg));
            }
        };
        body.validate()?;

        let slf = Self::from(body);
        if slf.id != value {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!(
                    "Client id from remote document {} does not match the given URL {}",
                    slf.id, value,
                ),
            ));
        }

        Ok(slf)
    }
}

impl From<EphemeralClientRequest> for Client {
    fn from(value: EphemeralClientRequest) -> Self {
        let scopes = EPHEMERAL_CLIENTS_ALLOWED_SCOPES.clone();

        Self {
            id: value.client_id,
            name: value.client_name,
            enabled: true,
            confidential: false,
            secret: None,
            secret_kid: None,
            redirect_uris: value.redirect_uris.join(","),
            post_logout_redirect_uris: value.post_logout_redirect_uris.map(|uris| uris.join(",")),
            allowed_origins: None,
            flows_enabled: EPHEMERAL_CLIENTS_ALLOWED_FLOWS.clone(),
            access_token_alg: value
                .access_token_signed_response_alg
                .unwrap_or_default()
                .to_string(),
            id_token_alg: value
                .id_token_signed_response_alg
                .unwrap_or_default()
                .to_string(),
            refresh_token: EPHEMERAL_CLIENTS_ALLOWED_FLOWS.contains("refresh_token"),
            auth_code_lifetime: 60,
            access_token_lifetime: value.default_max_age.unwrap_or(1800),
            scopes: scopes.clone(),
            default_scopes: scopes,
            challenge: Some("S256".to_string()),
            force_mfa: *EPHEMERAL_CLIENTS_FORCE_MFA,
        }
    }
}

#[derive(Debug)]
pub struct NewClient {
    pub id: String,
    pub secret: Option<Vec<u8>>,
    pub name: Option<String>,
    pub confidential: bool,
    pub redirect_uris: String,
    pub post_logout_redirect_uris: Option<String>,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            id: String::default(),
            name: None,
            enabled: true,
            confidential: false,
            secret: None,
            secret_kid: None,
            redirect_uris: String::default(),
            post_logout_redirect_uris: None,
            allowed_origins: None,
            flows_enabled: "authorization_code".to_string(),
            access_token_alg: "EdDSA".to_string(),
            id_token_alg: "EdDSA".to_string(),
            refresh_token: false,
            auth_code_lifetime: 60,
            access_token_lifetime: 1800,
            scopes: "openid,email,profile,groups".to_string(),
            default_scopes: "openid".to_string(),
            challenge: Some("S256".to_string()),
            force_mfa: false,
        }
    }
}

impl From<NewClientRequest> for Client {
    fn from(client: NewClientRequest) -> Self {
        let redirect_uris = client.redirect_uris.join(",");
        let post_logout_redirect_uris = client.post_logout_redirect_uris.map(|u| u.join(","));

        Self {
            id: client.id,
            secret: client.secret,
            name: client.name,
            confidential: client.confidential,
            redirect_uris,
            post_logout_redirect_uris,
            ..Default::default()
        }
    }
}

impl Client {
    fn try_from_dyn_reg(req: DynamicClientRequest) -> Result<Self, ErrorResponse> {
        let id = format!("dyn${}", get_rand(16));

        let confidential = req.token_endpoint_auth_method.as_deref() != Some("none");
        let (secret, secret_kid, _secret_plain) = if confidential {
            let (plain, enc) = Self::generate_new_secret()?;
            (
                Some(enc),
                Some(EncKeys::get_static().enc_key_active.clone()),
                Some(plain),
            )
        } else {
            (None, None, None)
        };

        let access_token_alg = req
            .token_endpoint_auth_signing_alg
            .unwrap_or_default()
            .to_string();
        let id_token_alg = req
            .id_token_signed_response_alg
            .unwrap_or_default()
            .to_string();
        let refresh_token = req
            .grant_types
            .iter()
            .map(|t| t.as_str())
            .collect::<Vec<&str>>()
            .contains(&"refresh_token");

        Ok(Self {
            id,
            name: req.client_name,
            enabled: true,
            confidential,
            secret,
            secret_kid,
            redirect_uris: req.redirect_uris.join(","),
            post_logout_redirect_uris: req.post_logout_redirect_uri,
            allowed_origins: None,
            flows_enabled: req.grant_types.join(","),
            access_token_alg,
            id_token_alg,
            refresh_token,
            access_token_lifetime: *DYN_CLIENT_DEFAULT_TOKEN_LIFETIME,
            challenge: confidential.then_some("S256".to_string()),
            force_mfa: false,
            ..Default::default()
        })
    }
}

/**
Checks if the HttpRequest's `Origin` Header is an external one, which needs to be validated with
the *Allowed-Origins* setting of the current client. Returns the origin as a `&str` if the Origin
is external and needs further validation.
 */
pub fn is_origin_external<'a>(
    req: &'a HttpRequest,
    listen_scheme: &'a ListenScheme,
    pub_url: &'a str,
) -> Result<(bool, &'a str), ErrorResponse> {
    let opt = req.headers().get(header::ORIGIN);
    if opt.is_none() {
        return Ok((false, ""));
    }
    let origin = opt.unwrap().to_str().unwrap_or("");
    debug!(origin, "External Origin header found:");

    let (scheme, url) = origin.split_once("://").ok_or_else(|| {
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Cannot parse ORIGIN header".to_string(),
        )
    })?;

    let scheme_ok = if *PROXY_MODE && scheme == "https" {
        true
    } else {
        match listen_scheme {
            ListenScheme::Http => scheme == "http",
            ListenScheme::Https => scheme == "https",
            ListenScheme::HttpHttps => scheme == "http" || scheme == "https",
        }
    };
    if !scheme_ok {
        warn!(pub_url, "Not matching scheme for HttpHeader::ORIGIN");
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "The scheme of the ORIGIN header does not match".to_string(),
        ));
    }

    debug!("pub_url: {}", pub_url);
    debug!("origin: {}", origin);

    if pub_url.eq(url) {
        return Ok((false, origin));
    }
    Ok((true, origin))
}

#[cfg(test)]
mod tests {
    use std::thread::JoinHandle;
    use std::time::Duration;
    use std::{env, thread};

    use actix_web::http::header;
    use actix_web::test::TestRequest;
    use actix_web::{App, HttpResponse, HttpServer};
    use pretty_assertions::assert_eq;
    use rauthy_common::constants::APPLICATION_JSON;
    use validator::Validate;

    use super::*;

    #[test]
    fn test_client_impl() {
        let mut client = Client {
            id: "123".to_string(),
            name: Some("Client123".to_string()),
            enabled: true,
            confidential: false,
            secret: None,
            secret_kid: None,
            redirect_uris: "".to_string(),
            post_logout_redirect_uris: None,
            allowed_origins: Some("http://localhost:8081,http://localhost:8082".to_string()),
            flows_enabled: "authorization_code,password".to_string(),
            access_token_alg: "EdDSA".to_string(),
            id_token_alg: "RS256".to_string(),
            refresh_token: true,
            auth_code_lifetime: 0,
            access_token_lifetime: 0,
            scopes: "openid,email,profile,groups".to_string(),
            default_scopes: "openid,email,profile,groups".to_string(),
            challenge: Some("S256,plain".to_string()),
            force_mfa: false,
        };

        assert_eq!(client.get_access_token_alg().unwrap(), JwkKeyPairAlg::EdDSA);
        assert_eq!(client.get_id_token_alg().unwrap(), JwkKeyPairAlg::RS256);

        assert_eq!(
            client.get_challenges(),
            Some(vec!["S256".to_string(), "plain".to_string()])
        );

        // scopes testing
        assert_eq!(
            client.get_scopes(),
            vec![
                "openid".to_string(),
                "email".to_string(),
                "profile".to_string(),
                "groups".to_string()
            ]
        );
        assert_eq!(
            client.get_scope_as_str(),
            "openid email profile groups".to_string()
        );
        assert_eq!(
            client.get_default_scopes(),
            vec![
                "openid".to_string(),
                "email".to_string(),
                "profile".to_string(),
                "groups".to_string()
            ]
        );
        // delete scope at the end
        client.delete_scope("groups");
        assert_eq!(
            client.get_scopes(),
            vec![
                "openid".to_string(),
                "email".to_string(),
                "profile".to_string(),
            ]
        );
        assert_eq!(
            client.get_scope_as_str(),
            "openid email profile".to_string()
        );
        assert_eq!(
            client.get_default_scopes(),
            vec![
                "openid".to_string(),
                "email".to_string(),
                "profile".to_string(),
            ]
        );
        // delete scope in the middle
        client.delete_scope("email");
        assert_eq!(
            client.get_scopes(),
            vec!["openid".to_string(), "profile".to_string(),]
        );
        assert_eq!(client.get_scope_as_str(), "openid profile".to_string());
        assert_eq!(
            client.get_default_scopes(),
            vec!["openid".to_string(), "profile".to_string(),]
        );
        // delete scope in the beginning
        client.delete_scope("openid");
        assert_eq!(client.get_scopes(), vec!["profile".to_string(),]);
        assert_eq!(client.get_scope_as_str(), "profile".to_string());
        assert_eq!(client.get_default_scopes(), vec!["profile".to_string(),]);

        assert_eq!(client.validate_challenge_method("S256"), Ok(()));
        assert_eq!(client.validate_challenge_method("plain"), Ok(()));
        assert!(client.validate_challenge_method("blabla").is_err());
        assert!(client.validate_challenge_method("").is_err());

        assert_eq!(client.validate_flow("authorization_code"), Ok(()));
        assert_eq!(client.validate_flow("password"), Ok(()));
        assert!(client.validate_flow("blabla").is_err());
        assert!(client.validate_flow("").is_err());

        // validate origin
        let listen_scheme = ListenScheme::Http;
        let pub_url = "localhost:8080";
        let origin = format!("{}://{}", listen_scheme, pub_url);

        // same origin first
        let req = TestRequest::default()
            .insert_header((header::ORIGIN, origin))
            .to_http_request();
        let res = client.validate_origin(&req, &listen_scheme, pub_url);
        assert!(res.is_ok());
        assert!(res.unwrap().is_none());

        // now other origins
        let req = TestRequest::default()
            .insert_header((header::ORIGIN, "http://localhost:8081"))
            .to_http_request();
        let res = client.validate_origin(&req, &listen_scheme, pub_url);
        assert!(res.is_ok());
        let header = res.unwrap().unwrap();
        assert_eq!(header.0, header::ACCESS_CONTROL_ALLOW_ORIGIN);
        assert_eq!(header.1, "http://localhost:8081");

        let req = TestRequest::default()
            .insert_header((header::ORIGIN, "http://localhost:8082"))
            .to_http_request();
        let res = client.validate_origin(&req, &listen_scheme, pub_url);
        assert!(res.is_ok());
        let header = res.unwrap().unwrap();
        assert_eq!(header.0, header::ACCESS_CONTROL_ALLOW_ORIGIN);
        assert_eq!(header.1, "http://localhost:8082");

        let req = TestRequest::default()
            .insert_header((header::ORIGIN, "http://localhost:8083"))
            .to_http_request();
        let res = client.validate_origin(&req, &listen_scheme, pub_url);
        assert!(res.is_err());
    }

    #[test]
    fn test_is_origin_external() {
        let pub_url = "localhost:8443";
        env::set_var("PROXY_MODE", "false");

        // err without ORIGIN header
        let req = TestRequest::default().to_http_request();
        let (is_ext, origin) = is_origin_external(&req, &ListenScheme::Http, pub_url).unwrap();
        assert_eq!(is_ext, false);
        assert_eq!(origin, "");

        // should return true -> url is external
        let req = TestRequest::default()
            .insert_header((header::ORIGIN, "http://localhost:8082"))
            .to_http_request();
        let (is_ext, origin) = is_origin_external(&req, &ListenScheme::Http, pub_url).unwrap();
        assert_eq!(is_ext, true);
        assert_eq!(origin, "http://localhost:8082");

        // different protocol
        let req = TestRequest::default()
            .insert_header((header::ORIGIN, "https://localhost:8443"))
            .to_http_request();
        let res = is_origin_external(&req, &ListenScheme::Http, pub_url);
        // scheme does not match
        assert!(res.is_err());

        // different protocol vice versa
        let req = TestRequest::default()
            .insert_header((header::ORIGIN, "http://localhost:8443"))
            .to_http_request();
        let res = is_origin_external(&req, &ListenScheme::Https, pub_url);
        // scheme does not match
        assert!(res.is_err());

        // should return false -> url is NOT external
        let req = TestRequest::default()
            .insert_header((header::ORIGIN, format!("http://{}", pub_url)))
            .to_http_request();
        let (is_ext, _origin) = is_origin_external(&req, &ListenScheme::Http, pub_url).unwrap();
        assert_eq!(is_ext, false);

        // should return false -> url is NOT external
        let req = TestRequest::default()
            .insert_header((header::ORIGIN, format!("https://{}", pub_url)))
            .to_http_request();
        let (is_ext, _origin) = is_origin_external(&req, &ListenScheme::Https, pub_url).unwrap();
        assert_eq!(is_ext, false);

        // should return false -> url is NOT external
        let req = TestRequest::default()
            .insert_header((header::ORIGIN, format!("http://{}", pub_url)))
            .to_http_request();
        let (is_ext, _origin) =
            is_origin_external(&req, &ListenScheme::HttpHttps, pub_url).unwrap();
        assert_eq!(is_ext, false);

        // should return false -> url is NOT external
        let req = TestRequest::default()
            .insert_header((header::ORIGIN, format!("https://{}", pub_url)))
            .to_http_request();
        let (is_ext, _origin) =
            is_origin_external(&req, &ListenScheme::HttpHttps, pub_url).unwrap();
        assert_eq!(is_ext, false);
    }

    #[test]
    fn test_from_ephemeral_client() {
        let example_client_res_resp = r#"{
          "@context": [ "https://www.w3.org/ns/solid/oidc-context.jsonld" ],

          "client_id": "https://decentphtos.example/webid#this",
          "client_name": "DecentPhotos",
          "redirect_uris": [ "https://decentphotos.example/callback" ],
          "post_logout_redirect_uris": [ "https://decentphotos.example/logout" ],
          "client_uri": "https://decentphotos.example/",
          "logo_uri": "https://decentphotos.example/logo.png",
          "tos_uri": "https://decentphotos.example/tos.html",
          "scope": "openid webid offline_access",
          "grant_types": [ "refresh_token", "authorization_code" ],
          "response_types": [ "code" ],
          "default_max_age": 3600,
          "require_auth_time": true
        }"#;
        let payload: EphemeralClientRequest =
            serde_json::from_str(example_client_res_resp).unwrap();
        // make sure our validation is good
        payload.validate().unwrap();

        // try build a client from it
        let client = Client::from(payload);
        println!("Client from EphemeralClientRequest:\n{:?}", client);

        assert_eq!(client.id.as_str(), "https://decentphtos.example/webid#this");
        assert_eq!(client.name.as_deref(), Some("DecentPhotos"));

        let uris = client.get_redirect_uris();
        assert_eq!(uris.len(), 1);
        assert_eq!(
            uris.get(0).unwrap().as_str(),
            "https://decentphotos.example/callback",
        );

        let uris = client.get_post_logout_uris().unwrap();
        assert_eq!(uris.len(), 1);
        assert_eq!(
            uris.get(0).unwrap().as_str(),
            "https://decentphotos.example/logout",
        );
    }

    #[tokio::test]
    async fn test_ephemeral_from_url() {
        let handle = serve_ephemeral_client();

        // make sure the http server starts and keeps running
        tokio::time::sleep(Duration::from_millis(100)).await;
        assert!(!handle.is_finished());

        // try to build up the whole client from the url
        let client_id = "http://127.0.0.1:10080/client";
        let client = Client::ephemeral_from_url(client_id).await.unwrap();

        // only id assertion here, the rest has been validated above in test_from_ephemeral_client()
        assert_eq!(client.id.as_str(), "http://127.0.0.1:10080/client");
    }

    fn serve_ephemeral_client() -> JoinHandle<()> {
        thread::spawn(move || {
            let actix_system = actix_web::rt::System::new();
            actix_system.block_on(async {
                HttpServer::new(|| {
                    App::new().route(
                        "/client",
                        web::get().to(|| async {
                            // Serves the example client response from the Solid OIDC primer
                            // https://solidproject.org/TR/oidc-primer
                            HttpResponse::Ok().content_type(APPLICATION_JSON).body(r#"{
                              "@context": [ "https://www.w3.org/ns/solid/oidc-context.jsonld" ],

                              "client_id": "http://127.0.0.1:10080/client",
                              "client_name": "DecentPhotos",
                              "redirect_uris": [ "https://decentphotos.example/callback" ],
                              "post_logout_redirect_uris": [ "https://decentphotos.example/logout" ],
                              "client_uri": "https://decentphotos.example/",
                              "logo_uri": "https://decentphotos.example/logo.png",
                              "tos_uri": "https://decentphotos.example/tos.html",
                              "scope": "openid webid offline_access",
                              "grant_types": [ "refresh_token", "authorization_code" ],
                              "response_types": [ "code" ],
                              "default_max_age": 60,
                              "require_auth_time": true
                            }"#,
                            )
                        }),
                    )
                })
                .bind(("127.0.0.1", 10080))
                .expect("port 10080 to be free for testing")
                .run()
                .await
                .expect("ephemeral client test http server to start") })
        })
    }
}
