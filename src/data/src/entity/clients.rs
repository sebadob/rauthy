use crate::database::{Cache, DB};
use crate::entity::auth_providers::ProviderMfaLogin;
use crate::entity::clients_dyn::ClientDyn;
use crate::entity::clients_scim::ClientScim;
use crate::entity::jwk::JwkKeyPairAlg;
use crate::entity::scopes::Scope;
use crate::entity::users::User;
use crate::rauthy_config::RauthyConfig;
use actix_web::HttpRequest;
use actix_web::http::header;
use actix_web::http::header::{HeaderName, HeaderValue};
use chrono::Utc;
use cryptr::{EncKeys, EncValue, utils};
use deadpool_postgres::GenericClient;
use hiqlite::Params;
use hiqlite_macros::params;
use rauthy_api_types::clients::{
    ClientResponse, DynamicClientRequest, DynamicClientResponse, EphemeralClientRequest,
    NewClientRequest, ScimClientRequestResponse,
};
use rauthy_common::constants::{APPLICATION_JSON, CACHE_TTL_APP, SECRET_LEN_CLIENTS};
use rauthy_common::utils::{get_rand, real_ip_from_req};
use rauthy_common::{http_client, is_hiqlite};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use reqwest::Url;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use std::cmp::min;
use std::collections::HashSet;
use std::fmt::Write;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::str::FromStr;
use tracing::{debug, error, trace, warn};
use validator::Validate;

static SQL_SAVE: &str = r#"
UPDATE clients
SET name = $1, enabled = $2, confidential = $3, secret = $4, secret_kid = $5, redirect_uris = $6,
    post_logout_redirect_uris = $7, allowed_origins = $8, flows_enabled = $9, access_token_alg = $10,
    id_token_alg = $11, auth_code_lifetime = $12, access_token_lifetime = $13, scopes = $14,
    default_scopes = $15, challenge = $16, force_mfa= $17, client_uri = $18, contacts = $19,
    backchannel_logout_uri = $20, restrict_group_prefix = $21
WHERE id = $22"#;

/**
# OIDC Client

A few values here are saved as CSV Strings instead of having foreign keys and links to other
tables.
All deleting and modifying operations are a bit more expensive this way, but we gain a lot of
performance, when we do reads on clients, which we do most of the time.

`*_lifetime` values are meant to be in seconds.
 */
#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Client {
    pub id: String,
    pub name: Option<String>,
    pub enabled: bool,
    pub confidential: bool,
    pub secret: Option<Vec<u8>>,
    pub secret_kid: Option<String>,
    pub redirect_uris: String,
    pub post_logout_redirect_uris: Option<String>,
    pub allowed_origins: Option<String>,
    pub flows_enabled: String,
    // Currently supported Algorithms: RS 256, 384, 512 and EdDSA
    pub access_token_alg: String,
    // Currently supported Algorithms: RS 256, 384, 512 and EdDSA
    pub id_token_alg: String,
    pub auth_code_lifetime: i32,
    pub access_token_lifetime: i32,
    pub scopes: String,
    pub default_scopes: String,
    pub challenge: Option<String>,
    pub force_mfa: bool,
    pub client_uri: Option<String>,
    pub contacts: Option<String>,
    pub backchannel_logout_uri: Option<String>,
    pub restrict_group_prefix: Option<String>,
}

impl Debug for Client {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Client {{ id: {}, name: {:?}, endabled: {}, confidential: {}, secret: <hidden>, \
        redirect_uris: {}, post_logout_redirect_uris: {:?}, allowed_origins: {:?}, \
        flows_enabled: {}, access_token_alg: {}, id_token_alg: {}, auth_code_lifetime: {}, \
        access_token_lifetime: {}, scopes: {}, default_scopes: {}, challenge: {:?}, force_mfa: {}, \
        client_uri: {:?}, contacts: {:?}, backchannel_logout_uri: {:?}, restrict_group_prefix: {:?} \
        }}",
            self.id,
            self.name,
            self.enabled,
            self.confidential,
            self.redirect_uris,
            self.post_logout_redirect_uris,
            self.allowed_origins,
            self.flows_enabled,
            self.access_token_alg,
            self.id_token_alg,
            self.auth_code_lifetime,
            self.access_token_lifetime,
            self.scopes,
            self.default_scopes,
            self.challenge,
            self.force_mfa,
            self.client_uri,
            self.contacts,
            self.backchannel_logout_uri,
            self.restrict_group_prefix,
        )
    }
}

impl From<tokio_postgres::Row> for Client {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            enabled: row.get("enabled"),
            confidential: row.get("confidential"),
            secret: row.get("secret"),
            secret_kid: row.get("secret_kid"),
            redirect_uris: row.get("redirect_uris"),
            post_logout_redirect_uris: row.get("post_logout_redirect_uris"),
            allowed_origins: row.get("allowed_origins"),
            flows_enabled: row.get("flows_enabled"),
            access_token_alg: row.get("access_token_alg"),
            id_token_alg: row.get("id_token_alg"),
            auth_code_lifetime: row.get("auth_code_lifetime"),
            access_token_lifetime: row.get("access_token_lifetime"),
            scopes: row.get("scopes"),
            default_scopes: row.get("default_scopes"),
            challenge: row.get("challenge"),
            force_mfa: row.get("force_mfa"),
            client_uri: row.get("client_uri"),
            contacts: row.get("contacts"),
            backchannel_logout_uri: row.get("backchannel_logout_uri"),
            restrict_group_prefix: row.get("restrict_group_prefix"),
        }
    }
}

// CRUD
impl Client {
    #[inline]
    pub fn cache_idx(id: &str) -> String {
        format!("client_{id}")
    }

    // have less cloning
    pub async fn create(mut client_req: NewClientRequest) -> Result<Self, ErrorResponse> {
        let kid = if client_req.confidential {
            let (_cleartext, enc) = Self::generate_new_secret()?;
            client_req.secret = Some(enc);
            Some(EncKeys::get_static().enc_key_active.clone())
        } else {
            None
        };
        let mut client = Client::try_from(client_req)?;
        client.secret_kid = kid;

        let sql = r#"
INSERT INTO clients (id, name, enabled, confidential, secret, secret_kid, redirect_uris,
post_logout_redirect_uris, allowed_origins, flows_enabled, access_token_alg, id_token_alg,
auth_code_lifetime, access_token_lifetime, scopes, default_scopes, challenge, force_mfa,
client_uri, contacts, backchannel_logout_uri, restrict_group_prefix)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17,
$18, $19, $20, $21, $22)"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        &client.id,
                        &client.name,
                        client.enabled,
                        client.confidential,
                        &client.secret,
                        &client.secret_kid,
                        &client.redirect_uris,
                        &client.post_logout_redirect_uris,
                        &client.allowed_origins,
                        &client.flows_enabled,
                        &client.access_token_alg,
                        &client.id_token_alg,
                        client.auth_code_lifetime,
                        client.access_token_lifetime,
                        &client.scopes,
                        &client.default_scopes,
                        &client.challenge,
                        client.force_mfa,
                        &client.client_uri,
                        &client.contacts,
                        &client.backchannel_logout_uri,
                        &client.restrict_group_prefix
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &client.id,
                    &client.name,
                    &client.enabled,
                    &client.confidential,
                    &client.secret,
                    &client.secret_kid,
                    &client.redirect_uris,
                    &client.post_logout_redirect_uris,
                    &client.allowed_origins,
                    &client.flows_enabled,
                    &client.access_token_alg,
                    &client.id_token_alg,
                    &client.auth_code_lifetime,
                    &client.access_token_lifetime,
                    &client.scopes,
                    &client.default_scopes,
                    &client.challenge,
                    &client.force_mfa,
                    &client.client_uri,
                    &client.contacts,
                    &client.backchannel_logout_uri,
                    &client.restrict_group_prefix,
                ],
            )
            .await?;
        }

        Ok(client)
    }

    pub async fn create_dynamic(
        client_req: DynamicClientRequest,
        origin_header: Option<String>,
    ) -> Result<DynamicClientResponse, ErrorResponse> {
        let token_endpoint_auth_method = client_req
            .token_endpoint_auth_method
            .clone()
            .unwrap_or_else(|| "client_secret_basic".to_string());

        let client = Self::try_from_dyn_reg(client_req, origin_header)?;

        let created = Utc::now().timestamp();
        let (_secret_plain, registration_token) = Self::generate_new_secret()?;

        let sql_1 = r#"
INSERT INTO clients (id, name, enabled, confidential, secret, secret_kid, redirect_uris,
post_logout_redirect_uris, allowed_origins, flows_enabled, access_token_alg, id_token_alg,
auth_code_lifetime, access_token_lifetime, scopes, default_scopes, challenge, force_mfa,
client_uri, contacts, backchannel_logout_uri, restrict_group_prefix)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
$21, $22)"#;
        let sql_2 = r#"
INSERT INTO
clients_dyn (id, created, registration_token, token_endpoint_auth_method)
VALUES ($1, $2, $3, $4)"#;

        if is_hiqlite() {
            DB::hql()
                .txn([
                    (
                        sql_1,
                        params!(
                            &client.id,
                            &client.name,
                            client.enabled,
                            client.confidential,
                            &client.secret,
                            &client.secret_kid,
                            &client.redirect_uris,
                            &client.post_logout_redirect_uris,
                            &client.allowed_origins,
                            &client.flows_enabled,
                            &client.access_token_alg,
                            &client.id_token_alg,
                            client.auth_code_lifetime,
                            client.access_token_lifetime,
                            &client.scopes,
                            &client.default_scopes,
                            &client.challenge,
                            client.force_mfa,
                            &client.client_uri,
                            &client.contacts,
                            &client.backchannel_logout_uri,
                            &client.restrict_group_prefix
                        ),
                    ),
                    (
                        sql_2,
                        params!(
                            client.id.clone(),
                            created,
                            registration_token.clone(),
                            token_endpoint_auth_method.clone()
                        ),
                    ),
                ])
                .await?;
        } else {
            let mut cl = DB::pg().await?;
            let txn = cl.transaction().await?;

            DB::pg_txn_append(
                &txn,
                sql_1,
                &[
                    &client.id,
                    &client.name,
                    &client.enabled,
                    &client.confidential,
                    &client.secret,
                    &client.secret_kid,
                    &client.redirect_uris,
                    &client.post_logout_redirect_uris,
                    &client.allowed_origins,
                    &client.flows_enabled,
                    &client.access_token_alg,
                    &client.id_token_alg,
                    &client.auth_code_lifetime,
                    &client.access_token_lifetime,
                    &client.scopes,
                    &client.default_scopes,
                    &client.challenge,
                    &client.force_mfa,
                    &client.client_uri,
                    &client.contacts,
                    &client.backchannel_logout_uri,
                    &client.restrict_group_prefix,
                ],
            )
            .await?;
            DB::pg_txn_append(
                &txn,
                sql_2,
                &[
                    &client.id,
                    &created,
                    &registration_token,
                    &token_endpoint_auth_method,
                ],
            )
            .await?;

            txn.commit().await?;
        };

        let client_dyn = ClientDyn {
            id: client.id.clone(),
            created,
            last_used: None,
            registration_token,
            token_endpoint_auth_method,
        };

        client.into_dynamic_client_response(client_dyn, true)
    }

    // Deletes a client
    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM clients WHERE id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(&self.id)).await?;
        } else {
            DB::pg_execute(sql, &[&self.id]).await?;
        }

        self.delete_cache().await?;

        // We only clean up the cache. The database uses foreign key a cascade.
        if self.is_dynamic() {
            ClientDyn::delete_from_cache(&self.id).await?;
        }

        Ok(())
    }

    pub async fn delete_cache(&self) -> Result<(), ErrorResponse> {
        DB::hql()
            .delete(Cache::App, Self::cache_idx(&self.id))
            .await?;

        Ok(())
    }

    pub async fn delete_cache_for(id: &str) -> Result<(), ErrorResponse> {
        DB::hql().delete(Cache::App, Self::cache_idx(id)).await?;
        Ok(())
    }

    // Returns a client by id without its secret.
    pub async fn find(id: String) -> Result<Self, ErrorResponse> {
        let client = DB::hql();
        if let Some(slf) = client.get(Cache::App, Self::cache_idx(&id)).await? {
            return Ok(slf);
        };

        let sql = "SELECT * FROM clients WHERE id = $1";
        let slf: Self = if is_hiqlite() {
            client.query_as_one(sql, params!(id)).await?
        } else {
            DB::pg_query_one(sql, &[&id]).await?
        };

        client
            .put(Cache::App, Self::cache_idx(&slf.id), &slf, CACHE_TTL_APP)
            .await?;

        Ok(slf)
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM clients";
        let clients = if is_hiqlite() {
            DB::hql().query_as(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 2).await?
        };

        Ok(clients)
    }

    /// Finds all clients that match an entry in `ids` and have a configured `backchannel_logout_uri`.
    pub async fn find_all_bcl(ids: &[&str]) -> Result<Vec<Self>, ErrorResponse> {
        // Unfortunately, we cannot build the `IN` value upfront and use prepared statements,
        // which is why we will filter on the client side for now.
        // The
        let sql = "SELECT * FROM clients WHERE backchannel_logout_uri IS NOT NULL";
        let clients = if is_hiqlite() {
            DB::hql().query_as(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 0).await?
        }
        .into_iter()
        .filter(|c: &Self| ids.contains(&c.id.as_str()))
        .collect::<Vec<_>>();

        debug!(
            "Found {} clients with configured backchannel_logout_uri",
            clients.len()
        );

        Ok(clients)
    }

    /// Returns all registered `client_uri`s to be used during `USER_REG_OPEN_REDIRECT` checks.
    pub async fn find_all_client_uris() -> Result<Vec<String>, ErrorResponse> {
        let sql = "SELECT client_uri FROM clients WHERE client_uri IS NOT NULL";
        let uris = if is_hiqlite() {
            DB::hql()
                .query_raw(sql, params!())
                .await?
                .into_iter()
                .map(|mut r| r.get::<String>("client_uri"))
                .collect::<Vec<_>>()
        } else {
            DB::pg_query_rows(sql, &[], 2)
                .await?
                .into_iter()
                .map(|r| r.get::<_, String>("client_uri"))
                .collect::<Vec<_>>()
        };

        Ok(uris)
    }

    /// Accepts either a pre-registered client_id or a URL as such.
    /// If allowed, it will dynamically build an ephemeral client and cache it, it the client_id
    /// is a URL. Otherwise, it will do a classic fetch from the database.
    /// This function should be used in places where we would possibly accept an ephemeral client.
    pub async fn find_maybe_ephemeral(id: String) -> Result<Self, ErrorResponse> {
        if !RauthyConfig::get().vars.ephemeral_clients.enable || Url::from_str(&id).is_err() {
            return Self::find(id).await;
        }

        if let Some(slf) = DB::hql().get(Cache::ClientEphemeral, &id).await? {
            return Ok(slf);
        }

        let slf = Self::ephemeral_from_url(&id).await?;

        DB::hql()
            .put(
                Cache::ClientEphemeral,
                id,
                &slf,
                Some(RauthyConfig::get().vars.ephemeral_clients.cache_lifetime as i64),
            )
            .await?;

        Ok(slf)
    }

    /// This is an expensive query using `LIKE`, only use when necessary.
    pub async fn find_with_scope(scope_name: &str) -> Result<Vec<Self>, ErrorResponse> {
        let like = format!("%{scope_name}%");
        let sql = "SELECT * FROM clients WHERE scopes LIKE $1 OR default_scopes LIKE $1";

        let clients = if is_hiqlite() {
            DB::hql().query_as(sql, params!(like)).await?
        } else {
            DB::pg_query(sql, &[&like], 0).await?
        };

        Ok(clients)
    }

    pub fn save_txn_append(&self, txn: &mut Vec<(&str, Params)>) {
        let allowed_origins = self.allowed_origins.clone().filter(|o| !o.is_empty());
        let contacts = self.contacts.clone().filter(|c| !c.is_empty());
        let post_logout_redirect_uris = self
            .post_logout_redirect_uris
            .clone()
            .filter(|uris| !uris.is_empty());
        let backchannel_logout_uri = self
            .backchannel_logout_uri
            .clone()
            .filter(|uri| !uri.is_empty());

        txn.push((
            SQL_SAVE,
            params!(
                &self.name,
                self.enabled,
                self.confidential,
                &self.secret,
                &self.secret_kid,
                &self.redirect_uris,
                post_logout_redirect_uris,
                allowed_origins,
                &self.flows_enabled,
                &self.access_token_alg,
                &self.id_token_alg,
                self.auth_code_lifetime,
                self.access_token_lifetime,
                &self.scopes,
                &self.default_scopes,
                &self.challenge,
                self.force_mfa,
                &self.client_uri,
                contacts,
                backchannel_logout_uri,
                &self.restrict_group_prefix,
                &self.id
            ),
        ));
    }

    pub async fn save_txn(
        &self,
        txn: &deadpool_postgres::Transaction<'_>,
    ) -> Result<(), ErrorResponse> {
        let allowed_origins = self.allowed_origins.clone().filter(|o| !o.is_empty());
        let contacts = self.contacts.clone().filter(|c| !c.is_empty());
        let post_logout_redirect_uris = self
            .post_logout_redirect_uris
            .clone()
            .filter(|uris| !uris.is_empty());
        let backchannel_logout_uri = self
            .backchannel_logout_uri
            .clone()
            .filter(|uri| !uri.is_empty());

        DB::pg_txn_append(
            txn,
            SQL_SAVE,
            &[
                &self.name,
                &self.enabled,
                &self.confidential,
                &self.secret,
                &self.secret_kid,
                &self.redirect_uris,
                &post_logout_redirect_uris,
                &allowed_origins,
                &self.flows_enabled,
                &self.access_token_alg,
                &self.id_token_alg,
                &self.auth_code_lifetime,
                &self.access_token_lifetime,
                &self.scopes,
                &self.default_scopes,
                &self.challenge,
                &self.force_mfa,
                &self.client_uri,
                &contacts,
                &backchannel_logout_uri,
                &self.restrict_group_prefix,
                &self.id,
            ],
        )
        .await?;

        Ok(())
    }

    pub async fn save_cache(&self) -> Result<(), ErrorResponse> {
        DB::hql()
            .put(Cache::App, Client::cache_idx(&self.id), self, CACHE_TTL_APP)
            .await?;
        Ok(())
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        let allowed_origins = self.allowed_origins.clone().filter(|o| !o.is_empty());
        let contacts = self.contacts.clone().filter(|c| !c.is_empty());
        let post_logout_redirect_uris = self
            .post_logout_redirect_uris
            .clone()
            .filter(|uris| !uris.is_empty());
        let backchannel_logout_uri = self
            .backchannel_logout_uri
            .clone()
            .filter(|uri| !uri.is_empty());

        if is_hiqlite() {
            DB::hql()
                .execute(
                    SQL_SAVE,
                    params!(
                        self.name.clone(),
                        self.enabled,
                        self.confidential,
                        self.secret.clone(),
                        self.secret_kid.clone(),
                        self.redirect_uris.clone(),
                        post_logout_redirect_uris,
                        allowed_origins,
                        self.flows_enabled.clone(),
                        self.access_token_alg.clone(),
                        self.id_token_alg.clone(),
                        self.auth_code_lifetime,
                        self.access_token_lifetime,
                        self.scopes.clone(),
                        self.default_scopes.clone(),
                        self.challenge.clone(),
                        self.force_mfa,
                        self.client_uri.clone(),
                        contacts,
                        backchannel_logout_uri,
                        &self.restrict_group_prefix,
                        self.id.clone()
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                SQL_SAVE,
                &[
                    &self.name,
                    &self.enabled,
                    &self.confidential,
                    &self.secret,
                    &self.secret_kid,
                    &self.redirect_uris,
                    &post_logout_redirect_uris,
                    &allowed_origins,
                    &self.flows_enabled,
                    &self.access_token_alg,
                    &self.id_token_alg,
                    &self.auth_code_lifetime,
                    &self.access_token_lifetime,
                    &self.scopes,
                    &self.default_scopes,
                    &self.challenge,
                    &self.force_mfa,
                    &self.client_uri,
                    &contacts,
                    &backchannel_logout_uri,
                    &self.restrict_group_prefix,
                    &self.id,
                ],
            )
            .await?;
        }

        DB::hql()
            .put(Cache::App, Client::cache_idx(&self.id), self, CACHE_TTL_APP)
            .await?;

        Ok(())
    }

    pub async fn update_dynamic(
        client_req: DynamicClientRequest,
        mut client_dyn: ClientDyn,
    ) -> Result<DynamicClientResponse, ErrorResponse> {
        let token_endpoint_auth_method = client_req
            .token_endpoint_auth_method
            .clone()
            .unwrap_or_else(|| "client_secret_basic".to_string());

        let mut new_client = Self::try_from_dyn_reg(client_req, None)?;
        let current = Self::find(client_dyn.id.clone()).await?;
        if !current.is_dynamic() {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Invalid request for non-dynamic client",
            ));
        }

        // we need to keep some old and possibly user-modified values
        new_client.id = current.id;
        new_client.force_mfa = current.force_mfa;
        new_client.scopes = current.scopes;
        new_client.default_scopes = current.default_scopes;
        new_client.allowed_origins = current.allowed_origins;

        client_dyn.token_endpoint_auth_method = token_endpoint_auth_method;
        client_dyn.last_used = Some(Utc::now().timestamp());

        if RauthyConfig::get().vars.dynamic_clients.secret_auto_rotate {
            let (_secret_plain, registration_token) = Client::generate_new_secret()?;
            client_dyn.registration_token = registration_token;
        }

        let sql = r#"
UPDATE clients_dyn
SET registration_token = $1, token_endpoint_auth_method = $2, last_used = $3
WHERE id = $4"#;

        if is_hiqlite() {
            let mut txn = Vec::with_capacity(2);
            new_client.save_txn_append(&mut txn);
            txn.push((
                sql,
                params!(
                    client_dyn.registration_token.clone(),
                    client_dyn.token_endpoint_auth_method.clone(),
                    client_dyn.last_used,
                    client_dyn.id.clone()
                ),
            ));

            DB::hql().txn(txn).await?;
        } else {
            let mut cl = DB::pg().await?;
            let txn = cl.transaction().await?;

            new_client.save_txn(&txn).await?;

            DB::pg_txn_append(
                &txn,
                sql,
                &[
                    &client_dyn.registration_token,
                    &client_dyn.token_endpoint_auth_method,
                    &client_dyn.last_used,
                    &client_dyn.id,
                ],
            )
            .await?;

            txn.commit().await?;
        }

        new_client.save_cache().await?;
        let ttl = RauthyConfig::get().vars.dynamic_clients.rate_limit_sec as i64;
        DB::hql()
            .put(
                Cache::ClientDynamic,
                ClientDyn::get_cache_entry(&client_dyn.id),
                &client_dyn,
                Some(ttl),
            )
            .await?;

        new_client.into_dynamic_client_response(
            client_dyn,
            RauthyConfig::get().vars.dynamic_clients.secret_auto_rotate,
        )
    }

    pub async fn cache_current_secret(
        &self,
        cache_current_hours: Option<u8>,
    ) -> Result<(), ErrorResponse> {
        if let Some(hours) = cache_current_hours
            && let Some(plain) = self.get_secret_cleartext()?
        {
            // The original idea was to save secrets hashed. However, since the secrets are used as
            // a key, the HashMap inside the `hiqlite` cache handler will take care of this for us,
            // to they could not be read from memory at runtime.
            DB::hql()
                .put_bytes(
                    Cache::ClientSecret,
                    plain,
                    self.id.as_bytes().to_vec(),
                    Some(hours as i64),
                )
                .await?;
        }
        Ok(())
    }

    pub async fn validate_cached_secret(
        client_id: &str,
        secret: &str,
    ) -> Result<(), ErrorResponse> {
        match DB::hql().get_bytes(Cache::ClientSecret, secret).await? {
            None => {
                debug!("No cached secret found for client {client_id}");
                Err(ErrorResponse::new(
                    ErrorResponseType::Unauthorized,
                    "client_secret does not exist or is invalid",
                ))
            }
            Some(bytes) => {
                if bytes == client_id.as_bytes() {
                    debug!("Matching cached client_secret for {client_id}");
                    Ok(())
                } else {
                    debug!("Found cached client_secret, but for a different client.");
                    Err(ErrorResponse::new(
                        ErrorResponseType::Unauthorized,
                        "client_secret does not exist or is invalid",
                    ))
                }
            }
        }
    }
}

impl Client {
    pub fn allow_refresh_token(&self) -> bool {
        self.flows_enabled.contains("refresh_token")
    }

    // TODO make a generic 'delete_from_csv' function out of this and re-use it in some other places
    pub fn delete_scope(&mut self, scope: &str) {
        // find the scope via index in the string
        // first entry: delete scope + ',' if it exists
        // last entry: delete scope + ',' in front if it exists
        // middle: delete scope + ',' in front if it exists
        // --> 2 cases: first entry or else
        if let Some(i) = self.scopes.find(scope) {
            if i == 0 {
                // the scope is the first entry
                if self.scopes.len() > scope.len() {
                    let s = format!("{scope},");
                    self.scopes = self.scopes.replace(&s, "");
                } else {
                    self.scopes = String::default();
                }
            } else {
                // the scope is at the end or in the middle
                let s = format!(",{scope}");
                self.scopes = self.scopes.replace(&s, "");
            }
        }

        // check if the to-be-deleted scope is in the default scopes
        if let Some(i) = self.default_scopes.find(scope) {
            if i == 0 {
                // the scope is the first entry
                if self.default_scopes.len() > scope.len() {
                    let s = format!("{scope},");
                    self.default_scopes = self.default_scopes.replace(&s, "");
                } else {
                    self.default_scopes = String::default();
                }
            } else {
                // the scope is at the end or in the middle
                let s = format!(",{scope}");
                self.default_scopes = self.default_scopes.replace(&s, "");
            }
        }
    }

    /// Generates a new random 64 character long client secret and returns the cleartext and
    /// encrypted version
    /// # Panics
    /// The decryption depends on correctly set up `ENC_KEYS` and `ENC_KEY_ACTIVE` environment
    /// variables and panics, if this is not the case.
    pub fn generate_new_secret() -> Result<(String, Vec<u8>), ErrorResponse> {
        // CAUTION: DO NOT change the length of the `client_secret` here.
        // If you need to at some point, make sure to update `Self::validate_secret()` as well,
        // because it expects exactly 64 chars long secrets.
        let rnd = utils::secure_random_alnum(SECRET_LEN_CLIENTS);
        debug_assert_eq!(rnd.len(), SECRET_LEN_CLIENTS);
        let enc = EncValue::encrypt(rnd.as_bytes())?.into_bytes().to_vec();
        Ok((rnd, enc))
    }

    #[inline(always)]
    pub fn get_access_token_alg(&self) -> Result<JwkKeyPairAlg, ErrorResponse> {
        JwkKeyPairAlg::from_str(self.access_token_alg.as_str())
    }

    #[inline]
    pub fn get_allowed_origins(&self) -> Option<Vec<String>> {
        if let Some(origins) = &self.allowed_origins {
            let mut res = Vec::with_capacity(1);
            for o in origins.split(',') {
                if !o.is_empty() {
                    res.push(o.to_string());
                }
            }
            Some(res)
        } else {
            None
        }
    }

    #[inline]
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

    pub fn get_contacts(&self) -> Option<Vec<String>> {
        if let Some(contacts) = &self.contacts {
            let mut res = Vec::with_capacity(1);
            for c in contacts.split(',') {
                if !c.is_empty() {
                    res.push(c.to_string());
                }
            }
            Some(res)
        } else {
            None
        }
    }

    /// Decrypts the client secret (if it exists) and then returns it as clear text.
    #[inline]
    pub fn get_secret_cleartext(&self) -> Result<Option<String>, ErrorResponse> {
        if let Some(secret) = self.secret.as_ref() {
            let bytes = EncValue::try_from(secret.clone())?.decrypt()?;
            let cleartext = String::from_utf8_lossy(bytes.as_ref()).to_string();
            Ok(Some(cleartext))
        } else {
            Ok(None)
        }
    }

    #[inline]
    pub fn get_default_scopes(&self) -> Vec<String> {
        let mut res = Vec::new();
        self.default_scopes
            .split(',')
            .for_each(|s| res.push(s.trim().to_owned()));
        res
    }

    #[inline]
    pub fn get_id_token_alg(&self) -> Result<JwkKeyPairAlg, ErrorResponse> {
        JwkKeyPairAlg::from_str(self.id_token_alg.as_str())
    }

    #[inline]
    pub fn get_flows(&self) -> Vec<String> {
        let mut res = Vec::new();
        self.flows_enabled
            .split(',')
            .map(|f| f.trim().to_owned())
            .for_each(|f| res.push(f));
        res
    }

    #[inline]
    pub fn get_post_logout_uris(&self) -> Option<Vec<String>> {
        if let Some(uris) = &self.post_logout_redirect_uris {
            let mut res = Vec::with_capacity(1);
            for uri in uris.split(',') {
                if !uri.is_empty() {
                    res.push(uri.to_string());
                }
            }
            Some(res)
        } else {
            None
        }
    }

    #[inline]
    pub fn get_redirect_uris(&self) -> Vec<String> {
        if self.redirect_uris.is_empty() {
            Vec::default()
        } else {
            self.redirect_uris
                .split(',')
                .map(|i| i.trim().to_string())
                .collect()
        }
    }

    #[inline]
    pub fn get_scopes(&self) -> Vec<String> {
        let mut res = Vec::new();
        self.scopes
            .split(',')
            .for_each(|s| res.push(s.trim().to_owned()));
        res
    }

    #[inline]
    pub fn get_scope_as_str(&self) -> String {
        self.scopes.replace(',', " ")
    }

    #[inline]
    pub fn is_dynamic(&self) -> bool {
        self.id.starts_with("dyn$")
    }

    #[inline]
    pub fn is_ephemeral(&self) -> bool {
        // A non-dynamic client_id can never be a valid URL because of payload validation
        Url::from_str(&self.id).is_ok()
    }

    /// Sanitizes the current scopes and deletes everything, which does not exist in the `scopes`
    /// table in the database
    pub async fn sanitize_scopes(scps: Vec<String>) -> Result<String, ErrorResponse> {
        let mut res = String::with_capacity(scps.len());
        Scope::find_all().await?.into_iter().for_each(|s| {
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
            res = format!("openid,{res}");
        }
        Ok(res)
    }

    /// Sanitizes the requested scopes on the authorization endpoint and matches them to the
    /// allowed scopes for this client.
    #[inline]
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

        let matrix_enabled = RauthyConfig::get().vars.matrix.msc3861_enable;

        for s in scopes {
            if self.default_scopes.split(',').any(|d| d == s) {
                continue;
            }

            if self
                .scopes
                .split(',')
                .any(|allowed| Scope::matches(allowed, s, matrix_enabled))
            {
                res.push(s.clone());
            }
        }

        Ok(res)
    }

    /// Returns an error if the client is not enabled.
    #[inline]
    pub fn validate_enabled(&self) -> Result<(), ErrorResponse> {
        if self.enabled {
            Ok(())
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "client is disabled",
            ))
        }
    }

    /// Validates the User's access to this client depending on the `force_mfa` setting.
    /// Do this check after a possible password hash to not leak information to unauthenticated users!
    ///
    /// The "rauthy" client is the exception for this check to makes logging into the account
    /// possible without MFA. The force MFA for the Rauthy admin UI is done in
    /// Principal::validate_admin_session() depending on the `ADMIN_FORCE_MFA` config variable.
    #[inline]
    pub fn validate_mfa(
        &self,
        user: &User,
        provider_mfa_login: Option<ProviderMfaLogin>,
    ) -> Result<(), ErrorResponse> {
        let force_mfa = self.id != "rauthy" && self.force_mfa;
        let has_mfa =
            user.has_webauthn_enabled() || provider_mfa_login == Some(ProviderMfaLogin::Yes);

        if force_mfa && !has_mfa {
            trace!("MFA required for this client but the user has none");
            Err(ErrorResponse::new(
                ErrorResponseType::MfaRequired,
                "MFA is required for this client",
            ))
        } else {
            Ok(())
        }
    }

    // Validates the `Origin` HTTP Header from an incoming request and compares it to the
    // `allowed_origins`. If the Origin is an external one and allowed by the config, it returns
    // the correct `ACCESS_CONTROL_ALLOW_ORIGIN` header which can then be inserted into the
    // HttpResponse.
    #[inline]
    pub fn get_validated_origin_header(
        &self,
        req: &HttpRequest,
    ) -> Result<Option<(HeaderName, HeaderValue)>, ErrorResponse> {
        let pub_url_with_scheme = RauthyConfig::get().pub_url_with_scheme.as_str();
        let additional_allowed_origin_schemes = RauthyConfig::get()
            .vars
            .server
            .additional_allowed_origin_schemes
            .deref();
        self.get_validated_origin_header_with(
            req,
            pub_url_with_scheme,
            additional_allowed_origin_schemes,
        )
    }

    fn get_validated_origin_header_with(
        &self,
        req: &HttpRequest,
        pub_url_with_scheme: &str,
        additional_allowed_origin_schemes: &[String],
    ) -> Result<Option<(HeaderName, HeaderValue)>, ErrorResponse> {
        let origin = match extract_external_origin(
            req,
            pub_url_with_scheme,
            additional_allowed_origin_schemes,
        )? {
            Some(o) => o,
            None => {
                return Ok(None);
            }
        };

        let err_msg = || {
            debug!("Client request from invalid origin: {}", origin);
            Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!("Coming from an external Origin '{origin}' which is not allowed"),
            ))
        };

        if self.allowed_origins.is_none() {
            debug!("Request from external origin for client with no external origins allowed");
            return err_msg();
        }

        let has_any_match = self
            .allowed_origins
            .as_ref()
            .unwrap()
            .split(',')
            .any(|o| o == origin);
        if has_any_match {
            return Ok(Some((
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                HeaderValue::from_str(origin)?,
            )));
        }

        debug!(
            "No match found for allowed origin - external origin: {origin} / allowed: {:?}",
            self.allowed_origins
        );
        err_msg()
    }

    #[inline]
    pub fn validate_redirect_uri(&self, redirect_uri: &str) -> Result<(), ErrorResponse> {
        let has_any = self.get_redirect_uris().iter().any(|uri| {
            (uri.ends_with('*') && redirect_uri.starts_with(uri.split_once('*').unwrap().0))
                || uri.as_str().eq(redirect_uri)
        });

        if has_any {
            Ok(())
        } else {
            debug!(
                "Invalid `redirect_uri`: {} / expected one of: {}",
                redirect_uri, self.redirect_uris
            );
            Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Invalid redirect uri",
            ))
        }
    }

    #[inline]
    pub fn validate_post_logout_redirect_uri(
        &self,
        post_logout_redirect_uri: &str,
    ) -> Result<(), ErrorResponse> {
        let has_any = self
            .get_post_logout_uris()
            .unwrap_or_default()
            .iter()
            .any(|uri| {
                (uri.ends_with('*')
                    && post_logout_redirect_uri.starts_with(uri.split_once('*').unwrap().0))
                    || uri.as_str().eq(post_logout_redirect_uri)
            });

        if has_any {
            Ok(())
        } else {
            trace!("Invalid `post_logout_redirect_uri`");
            Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Invalid post_logout_redirect_uri",
            ))
        }
    }

    #[inline]
    pub fn validate_code_challenge(
        &self,
        code_challenge: &Option<String>,
        code_challenge_method: &Option<String>,
    ) -> Result<(), ErrorResponse> {
        if let Some(methods_allowed) = &self.challenge {
            if code_challenge.is_none() {
                trace!("'code_challenge' is missing");
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "'code_challenge' is missing",
                ));
            }

            let Some(method) = code_challenge_method else {
                trace!("'code_challenge_method' is missing");
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "'code_challenge_method' is missing",
                ));
            };

            if methods_allowed.contains(method) {
                Ok(())
            } else {
                trace!("given code_challenge_method is not allowed");
                Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    format!("code_challenge_method '{method}' is not allowed"),
                ))
            }
        } else if code_challenge.is_some() || code_challenge_method.is_some() {
            trace!("'code_challenge' not enabled for this client");
            Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "'code_challenge' not enabled for this client",
            ))
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn validate_challenge_method(
        &self,
        code_challenge_method: &str,
    ) -> Result<(), ErrorResponse> {
        if self.challenge.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "'code_challenge' not allowed",
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
                format!("code_challenge_method '{code_challenge_method}' is not allowed"),
            ));
        }
        Ok(())
    }

    #[inline]
    pub fn validate_flow(&self, flow: &str) -> Result<(), ErrorResponse> {
        if flow.is_empty() || !self.flows_enabled.contains(flow) {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!("'{flow}' flow is not allowed for this client"),
            ));
        }
        Ok(())
    }

    #[inline]
    pub async fn validate_secret(
        &self,
        secret: &str,
        req: &HttpRequest,
    ) -> Result<(), ErrorResponse> {
        if !self.confidential {
            error!("Cannot validate 'client_secret' for public client");
            return Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                "Cannot validate 'client_secret' for public client",
            ));
        }
        if secret.len() != SECRET_LEN_CLIENTS {
            error!("Invalid / too short secret given as `client_secret`");
            return Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Invalid 'client_secret'",
            ));
        }

        let secret_enc = self.secret.as_ref().ok_or_else(|| {
            ErrorResponse::new(
                ErrorResponseType::Internal,
                format!("'{}' has no secret while being confidential", &self.id),
            )
        })?;
        let cleartext = EncValue::try_from(secret_enc.clone())?.decrypt()?;

        // make sure this function is updated if the secret length ever changes
        debug_assert_eq!(cleartext.len(), SECRET_LEN_CLIENTS);

        let a = <&[u8; 64]>::try_from(cleartext.as_ref()).unwrap();
        let b = <&[u8; 64]>::try_from(secret.as_bytes()).unwrap();
        if constant_time_eq::constant_time_eq_64(a, b)
            || Client::validate_cached_secret(&self.id, secret)
                .await
                .is_ok()
        {
            return Ok(());
        }

        warn!(
            "Invalid login for client '{}' from '{}'",
            self.id,
            real_ip_from_req(req)?
        );

        Err(ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            "Invalid 'client_secret'",
        ))
    }

    #[inline]
    pub fn validate_user_groups(&self, user: &User) -> Result<(), ErrorResponse> {
        if let Some(prefix) = &self.restrict_group_prefix {
            if user.get_groups().iter().any(|g| g.starts_with(prefix)) {
                Ok(())
            } else {
                Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "You don't have access to this client, please contact your Administrator",
                ))
            }
        } else {
            Ok(())
        }
    }
}

impl Client {
    async fn ephemeral_from_url(value: &str) -> Result<Self, ErrorResponse> {
        let res = http_client()
            .get(value)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .send()
            .await
            .map_err(|err| {
                ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    format!("Cannot fetch ephemeral client data from {value}: {err:?}"),
                )
            })?;

        if !res.status().is_success() {
            let msg = format!("Cannot fetch ephemeral client information from {value}");
            error!("{msg}");
            return Err(ErrorResponse::new(ErrorResponseType::Connection, msg));
        }

        let body = match res.json::<EphemeralClientRequest>().await {
            Ok(b) => b,
            Err(err) => {
                let msg =
                    format!("Cannot deserialize into EphemeralClientRequest from {value}: {err:?}");
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
                    "Client id from remote document {} does not match the given URL {value}",
                    slf.id,
                ),
            ));
        }

        Ok(slf)
    }
}

impl Client {
    pub fn into_response(self, scim: Option<ClientScim>) -> ClientResponse {
        let redirect_uris = self.get_redirect_uris();
        let post_logout_redirect_uris = self.get_post_logout_uris();
        let allowed_origins = self.get_allowed_origins();
        let flows_enabled = self.get_flows();
        let scopes = self.get_scopes();
        let default_scopes = self.get_default_scopes();
        let challenges = self.get_challenges();
        let contacts = self.get_contacts();

        let access_token_alg = JwkKeyPairAlg::from_str(&self.access_token_alg)
            .expect("internal JwkKeyPairAlg conversion to always succeed")
            .into();
        let id_token_alg = JwkKeyPairAlg::from_str(&self.id_token_alg)
            .expect("internal JwkKeyPairAlg conversion to always succeed")
            .into();

        ClientResponse {
            id: self.id,
            name: self.name,
            enabled: self.enabled,
            confidential: self.confidential,
            redirect_uris,
            post_logout_redirect_uris,
            allowed_origins,
            flows_enabled,
            access_token_alg,
            id_token_alg,
            auth_code_lifetime: self.auth_code_lifetime,
            access_token_lifetime: self.access_token_lifetime,
            scopes,
            default_scopes,
            challenges,
            force_mfa: self.force_mfa,
            client_uri: self.client_uri,
            contacts,
            backchannel_logout_uri: self.backchannel_logout_uri,
            restrict_group_prefix: self.restrict_group_prefix,
            scim: scim.map(|scim| ScimClientRequestResponse {
                bearer_token: scim.bearer_token,
                base_uri: scim.base_uri,
                sync_groups: scim.sync_groups,
                group_sync_prefix: scim.group_sync_prefix,
            }),
        }
    }
}

impl From<EphemeralClientRequest> for Client {
    fn from(value: EphemeralClientRequest) -> Self {
        let scopes = RauthyConfig::get()
            .vars
            .ephemeral_clients
            .allowed_scopes
            .join(",");

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
            flows_enabled: RauthyConfig::get()
                .vars
                .ephemeral_clients
                .allowed_flows
                .join(","),
            access_token_alg: value
                .access_token_signed_response_alg
                .unwrap_or_default()
                .to_string(),
            id_token_alg: value
                .id_token_signed_response_alg
                .unwrap_or_default()
                .to_string(),
            auth_code_lifetime: 60,
            access_token_lifetime: value.default_max_age.unwrap_or(1800),
            scopes: scopes.clone(),
            default_scopes: scopes,
            challenge: Some("S256".to_string()),
            force_mfa: RauthyConfig::get().vars.ephemeral_clients.force_mfa,
            client_uri: value.client_uri,
            contacts: value.contacts.map(|c| c.join(",")),
            backchannel_logout_uri: None,
            restrict_group_prefix: None,
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
            auth_code_lifetime: 60,
            access_token_lifetime: 1800,
            scopes: "openid,email,profile,groups".to_string(),
            default_scopes: "openid".to_string(),
            challenge: Some("S256".to_string()),
            force_mfa: false,
            client_uri: None,
            contacts: None,
            backchannel_logout_uri: None,
            restrict_group_prefix: None,
        }
    }
}

impl TryFrom<NewClientRequest> for Client {
    type Error = ErrorResponse;

    fn try_from(client: NewClientRequest) -> Result<Self, Self::Error> {
        let mut redirect_uris = String::with_capacity(24);
        for uri in client.redirect_uris {
            let trimmed = uri.trim();
            if !trimmed.is_empty() {
                write!(redirect_uris, "{trimmed},")?;
            }
        }
        redirect_uris.pop();

        let post_logout_redirect_uris =
            if let Some(post_logout_redirect_uris) = client.post_logout_redirect_uris {
                let mut uris = String::with_capacity(24);
                for uri in post_logout_redirect_uris {
                    let trimmed = uri.trim();
                    if !trimmed.is_empty() {
                        write!(uris, "{trimmed},")?;
                    }
                }
                uris.pop();
                if uris.is_empty() { None } else { Some(uris) }
            } else {
                None
            };

        Ok(Self {
            id: client.id,
            secret: client.secret,
            name: client.name,
            confidential: client.confidential,
            redirect_uris,
            post_logout_redirect_uris,
            ..Default::default()
        })
    }
}

impl Client {
    fn try_from_dyn_reg(
        req: DynamicClientRequest,
        origin_header: Option<String>,
    ) -> Result<Self, ErrorResponse> {
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
        let allowed_origins = origin_header.and_then(|_| {
            let origins = req
                .redirect_uris
                .iter()
                .filter_map(|uri| {
                    Url::parse(uri).ok().and_then(|url| {
                        let origin = url.origin().unicode_serialization();
                        if origin != "null" {
                            Some(origin.trim_end_matches('/').to_string())
                        } else {
                            None
                        }
                    })
                })
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
                .join(",");

            (!origins.is_empty()).then_some(origins)
        });
        let scopes = RauthyConfig::get()
            .vars
            .dynamic_clients
            .allowed_scopes
            .join(",");
        let default_scopes = RauthyConfig::get()
            .vars
            .dynamic_clients
            .default_scopes
            .join(",");

        Ok(Self {
            id,
            name: req.client_name,
            enabled: true,
            confidential,
            secret,
            secret_kid,
            redirect_uris: req.redirect_uris.join(","),
            post_logout_redirect_uris: req.post_logout_redirect_uri.filter(|uri| !uri.is_empty()),
            allowed_origins,
            flows_enabled: req.grant_types.join(","),
            access_token_alg,
            id_token_alg,
            access_token_lifetime: min(
                RauthyConfig::get()
                    .vars
                    .dynamic_clients
                    .default_token_lifetime,
                i32::MAX as u32,
            ) as i32,
            scopes,
            default_scopes,
            challenge: (!confidential).then_some("S256".to_string()),
            force_mfa: false,
            client_uri: req.client_uri,
            contacts: req.contacts.map(|c| c.join(",")).filter(|c| !c.is_empty()),
            backchannel_logout_uri: req.backchannel_logout_uri,
            ..Default::default()
        })
    }

    pub fn into_dynamic_client_response(
        self,
        client_dyn: ClientDyn,
        map_registration_client_uri: bool,
    ) -> Result<DynamicClientResponse, ErrorResponse> {
        let contacts = self.get_contacts();

        let redirect_uris = self.get_redirect_uris();
        let grant_types = self.get_flows();
        let post_logout_redirect_uri = self.get_redirect_uris().first().cloned();

        let client_secret = self.get_secret_cleartext()?;
        let (registration_access_token, registration_client_uri) = if map_registration_client_uri {
            (
                Some(client_dyn.registration_token_plain()?),
                Some(ClientDyn::registration_client_uri(&client_dyn.id)),
            )
        } else {
            (None, None)
        };

        Ok(DynamicClientResponse {
            client_id: self.id,
            client_name: self.name,
            client_uri: self.client_uri,
            contacts,
            client_secret,
            // TODO check if we can make sure that a client will renew the secret properly -> let it expire then
            client_secret_expires_at: 0,
            redirect_uris,
            post_logout_redirect_uri,
            backchannel_logout_uri: self.backchannel_logout_uri,
            registration_access_token,
            registration_client_uri,
            grant_types,
            id_token_signed_response_alg: self.id_token_alg,
            token_endpoint_auth_method: client_dyn.token_endpoint_auth_method,
            token_endpoint_auth_signing_alg: self.access_token_alg,
        })
    }
}

#[inline]
fn extract_external_origin<'a>(
    req: &'a HttpRequest,
    pub_url_with_scheme: &'a str,
    additional_allowed_origin_schemes: &'a [String],
) -> Result<Option<&'a str>, ErrorResponse> {
    let opt = req.headers().get(header::ORIGIN);
    if opt.is_none() {
        return Ok(None);
    }
    let origin = opt.unwrap().to_str().unwrap_or("");
    debug!(origin, "Origin header found");

    // `Origin` will be present for same-origin requests other than `GET` / `HEAD`
    if origin == pub_url_with_scheme {
        return Ok(None);
    }

    let (scheme, _) = origin.split_once("://").unwrap_or((origin, ""));
    if scheme != "http" && scheme != "https" {
        let has_any_match = additional_allowed_origin_schemes
            .iter()
            .any(|allowed| allowed == scheme);
        if !has_any_match {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "invalid scheme for `Origin`",
            ));
        }
    }

    debug!(origin, "External origin");
    Ok(Some(origin))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::header;
    use actix_web::test::TestRequest;
    use pretty_assertions::assert_eq;

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
            allowed_origins: Some(
                "http://localhost:8082,sample://localhost,tauri://localhost".to_string(),
            ),
            flows_enabled: "authorization_code,password".to_string(),
            access_token_alg: "EdDSA".to_string(),
            id_token_alg: "RS256".to_string(),
            auth_code_lifetime: 0,
            access_token_lifetime: 0,
            scopes: "openid,email,profile,groups".to_string(),
            default_scopes: "openid,email,profile,groups".to_string(),
            challenge: Some("S256,plain".to_string()),
            force_mfa: false,
            client_uri: Some("http://localhost:1337".to_string()),
            contacts: Some("batman@localhost.de,@alfred:matrix.org".to_string()),
            backchannel_logout_uri: None,
            restrict_group_prefix: None,
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

        // contacts
        assert_eq!(
            client.get_contacts().expect("contacts to be set"),
            vec![
                "batman@localhost.de".to_string(),
                "@alfred:matrix.org".to_string(),
            ]
        );

        // validate origin
        let pub_url_scheme = "http://localhost:8081";
        let additional_themes = ["tauri".to_string()];
        let origin = "http://localhost:8081";

        // same origin first
        let req = TestRequest::default()
            .insert_header((header::ORIGIN, origin))
            .to_http_request();
        let res = client.get_validated_origin_header_with(
            &req,
            pub_url_scheme,
            additional_themes.as_slice(),
        );
        assert!(res.is_ok());
        assert!(res.unwrap().is_none());

        // now other origins
        let req = TestRequest::default()
            .insert_header((header::ORIGIN, "http://localhost:8081"))
            .to_http_request();
        let res = client.get_validated_origin_header_with(
            &req,
            pub_url_scheme,
            additional_themes.as_slice(),
        );
        assert!(res.is_ok());
        // same-origin
        assert!(res.unwrap().is_none());

        let req = TestRequest::default()
            .insert_header((header::ORIGIN, "http://localhost:8082"))
            .to_http_request();
        let res = client.get_validated_origin_header_with(
            &req,
            pub_url_scheme,
            additional_themes.as_slice(),
        );
        assert!(res.is_ok());
        let header = res.unwrap().unwrap();
        assert_eq!(header.0, header::ACCESS_CONTROL_ALLOW_ORIGIN);
        assert_eq!(header.1, "http://localhost:8082");

        let req = TestRequest::default()
            .insert_header((header::ORIGIN, "http://localhost:8083"))
            .to_http_request();
        let res = client.get_validated_origin_header_with(
            &req,
            pub_url_scheme,
            additional_themes.as_slice(),
        );
        assert!(res.is_err());

        let req = TestRequest::default()
            .insert_header((header::ORIGIN, "sample://localhost"))
            .to_http_request();
        let res = client.get_validated_origin_header_with(
            &req,
            pub_url_scheme,
            additional_themes.as_slice(),
        );
        assert!(res.is_err());

        let req = TestRequest::default()
            .insert_header((header::ORIGIN, "tauri://localhost"))
            .to_http_request();
        let res = client.get_validated_origin_header_with(
            &req,
            pub_url_scheme,
            additional_themes.as_slice(),
        );
        assert_eq!(
            res.unwrap().unwrap().1.to_str().unwrap(),
            "tauri://localhost"
        );
    }

    // TODO: Currently out-commented because of issues with static RauthyConfig init missing
    //  in unit tests. Should be added into integration tests.
    // #[tokio::test]
    // async fn test_ephemeral_from_url() {
    //     let handle = serve_ephemeral_client();
    //
    //     // make sure the http server starts and keeps running
    //     tokio::time::sleep(Duration::from_millis(100)).await;
    //     assert!(!handle.is_finished());
    //
    //     // try to build up the whole client from the url
    //     let client_id = "http://127.0.0.1:10080/client";
    //     let client = Client::ephemeral_from_url(client_id).await.unwrap();
    //
    //     // only id assertion here, the rest has been validated above in test_from_ephemeral_client()
    //     assert_eq!(client.id.as_str(), "http://127.0.0.1:10080/client");
    // }
    //
    // fn serve_ephemeral_client() -> JoinHandle<()> {
    //     thread::spawn(move || {
    //         let actix_system = actix_web::rt::System::new();
    //         actix_system.block_on(async {
    //             HttpServer::new(|| {
    //                 App::new().route(
    //                     "/client",
    //                     web::get().to(|| async {
    //                         // Serves the example client response from the Solid OIDC primer
    //                         // https://solidproject.org/TR/oidc-primer
    //                         HttpResponse::Ok().content_type(APPLICATION_JSON).body(r#"{
    //                           "@context": [ "https://www.w3.org/ns/solid/oidc-context.jsonld" ],
    //
    //                           "client_id": "http://127.0.0.1:10080/client",
    //                           "client_name": "DecentPhotos",
    //                           "redirect_uris": [ "https://decentphotos.example/callback" ],
    //                           "post_logout_redirect_uris": [ "https://decentphotos.example/logout" ],
    //                           "client_uri": "https://decentphotos.example/",
    //                           "logo_uri": "https://decentphotos.example/logo.png",
    //                           "tos_uri": "https://decentphotos.example/tos.html",
    //                           "scope": "openid webid offline_access",
    //                           "grant_types": [ "refresh_token", "authorization_code" ],
    //                           "response_types": [ "code" ],
    //                           "default_max_age": 60,
    //                           "require_auth_time": true
    //                         }"#,
    //                         )
    //                     }),
    //                 )
    //             })
    //                 .bind(("127.0.0.1", 10080))
    //                 .expect("port 10080 to be free for testing")
    //                 .run()
    //                 .await
    //                 .expect("ephemeral client test http server to start")
    //         })
    //     })
    // }

    #[test]
    fn test_delete_client_custom_scope() {
        let mut client = Client::default();
        client.scopes = "email,openid,profile,groups".to_string();
        client.default_scopes = "email,openid,cust_scope".to_string();

        client.delete_scope("profile");
        assert_eq!(&client.scopes, "email,openid,groups");
        assert_eq!(&client.default_scopes, "email,openid,cust_scope");

        client.delete_scope("cust_scope");
        assert_eq!(&client.scopes, "email,openid,groups");
        assert_eq!(&client.default_scopes, "email,openid");

        client.delete_scope("email");
        assert_eq!(&client.scopes, "openid,groups");
        assert_eq!(&client.default_scopes, "openid");

        client.delete_scope("groups");
        assert_eq!(&client.scopes, "openid");
        assert_eq!(&client.default_scopes, "openid");
    }
}
