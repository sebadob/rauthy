use crate::database::{Cache, DB};
use crate::entity::clients::Client;
use crate::rauthy_config::RauthyConfig;
use chrono::Utc;
use rauthy_common::utils::get_rand;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::fmt::{Debug, Formatter};
use std::ops::Add;
use std::time::Duration;
use utoipa::ToSchema;

#[derive(Deserialize)]
struct AuthCodeOld {
    id: String,
    exp: i64,
    client_id: String,
    user_id: String,
    session_id: Option<String>,
    challenge: Option<String>,
    challenge_method: Option<String>,
    nonce: Option<String>,
    scopes: Vec<String>,
    resource: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct AuthCode {
    pub id: String,
    pub exp: i64,
    pub client_id: String,
    /// The exact URI used during authorization
    pub redirect_uri: String,
    pub user_id: String,
    pub session_id: Option<String>,
    pub challenge: Option<String>,
    pub challenge_method: Option<String>,
    pub nonce: Option<String>,
    pub scopes: Vec<String>,
    /// RFC 8707 resource indicator chosen at the authorization request, carried through
    /// to the token exchange so the issued access token can be audience-restricted.
    /// No `serde` skip/default attributes here: auth codes are cached with bincode (a
    /// positional, non-self-describing format), so the field must always be present.
    pub resource: Option<String>,
    /// The opaque `state` from the authorization request, bound to this code so that consumers
    /// can verify a presented `state` belongs to exactly this code. No `serde` skip/default
    /// attributes here: auth codes are cached with bincode (a positional, non-self-describing
    /// format), so the field must always be present.
    ///
    /// Note: This is only used for Forward Auth, because in this case, we are also our own client.
    /// We will not save the state during normal logins, because it does not make any sense. We will
    /// not get anything from the client we could compare it to.
    pub state: Option<String>,
}

impl Debug for AuthCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AuthCode {{ id: {}(...), exp: {}, client_id: {}, user_id: {}, scopes: {:?} }}",
            &self.id[..5],
            self.exp,
            self.client_id,
            self.user_id,
            self.scopes
        )
    }
}

// CRUD
impl AuthCode {
    // Deletes an Authorization Code from the cache
    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        DB::hql().delete(Cache::AuthCode, self.id.clone()).await?;
        Ok(())
    }

    // Claims an Authorization code from the cache
    pub async fn find_remove(id: String) -> Result<Option<Self>, ErrorResponse> {
        #[cfg(debug_assertions)]
        if !rauthy_common::constants::RAUTHY_VERSION.starts_with("0.37.") {
            todo!("Cleanup AuthCode::find_remove() and remove AuthCodeOld");
        }

        // TODO this versioning is only necessary during the 0.37 release.
        //  Remove it afterwards.
        let Some(bytes) = DB::hql().get_remove_bytes(Cache::AuthCode, id).await? else {
            return Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "`auth_code` not found",
            ));
        };

        match bincode::serde::decode_from_slice::<Self, _>(&bytes, bincode::config::legacy()) {
            Ok((slf, _)) => Ok(Some(slf)),
            Err(_) => {
                // This might be an old auth code during a migration.
                let Ok((code_old, _)) = bincode::serde::decode_from_slice::<AuthCodeOld, _>(
                    &bytes,
                    bincode::config::legacy(),
                ) else {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::NotFound,
                        "auth_code not found",
                    ));
                };

                Ok(Some(Self {
                    id: code_old.id,
                    exp: code_old.exp,
                    client_id: code_old.client_id,
                    // This is not an Option on purpose to prevent another migration being necessary
                    redirect_uri: String::default(),
                    user_id: code_old.user_id,
                    session_id: code_old.session_id,
                    challenge: code_old.challenge,
                    challenge_method: code_old.challenge_method,
                    nonce: code_old.nonce,
                    scopes: code_old.scopes,
                    resource: code_old.resource,
                    state: None,
                }))
            }
        }

        // Ok(DB::hql().get_remove(Cache::AuthCode, id).await?)
    }

    // Saves an Authorization Code
    pub async fn save(&self, ttl: i64) -> Result<(), ErrorResponse> {
        DB::hql()
            .put(Cache::AuthCode, self.id.clone(), self, Some(ttl))
            .await?;
        Ok(())
    }
}

impl AuthCode {
    #[inline]
    pub fn build_location_header(&self, state: Option<&str>) -> Result<String, ErrorResponse> {
        let append_char = if self.redirect_uri.contains('?') {
            '&'
        } else {
            '?'
        };
        let mut loc = format!("{}{}code={}", self.redirect_uri, append_char, self.id);
        if let Some(state) = state {
            write!(
                loc,
                "&state={}",
                percent_encoding::percent_encode(
                    state.as_bytes(),
                    percent_encoding::NON_ALPHANUMERIC
                )
            )?;
        };
        Ok(loc)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        user_id: String,
        client_id: String,
        redirect_uri: String,
        session_id: Option<String>,
        challenge: Option<String>,
        challenge_method: Option<String>,
        nonce: Option<String>,
        scopes: Vec<String>,
        resource: Option<String>,
        state: Option<String>,
        lifetime: Duration,
    ) -> Self {
        debug_assert!(!redirect_uri.is_empty());
        debug_assert!(lifetime.as_secs() > 0);

        let id = get_rand(64);
        let exp = Utc::now().add(lifetime).timestamp();

        Self {
            id,
            exp,
            client_id,
            redirect_uri,
            user_id,
            session_id,
            challenge,
            challenge_method,
            nonce,
            scopes,
            resource,
            state,
        }
    }

    /// CAUTION: DO NOT use this reset in any other case than after accepting updated ToS!
    pub async fn danger_save_reset_exp(
        &mut self,
        auth_code_lifetime: i64,
    ) -> Result<(), ErrorResponse> {
        self.exp = Utc::now()
            .add(chrono::Duration::seconds(auth_code_lifetime))
            .timestamp();

        self.save(auth_code_lifetime).await
    }

    #[inline(always)]
    pub fn validate_redirect_uri_exact(
        &self,
        client: &Client,
        redirect_uri: &str,
    ) -> Result<(), ErrorResponse> {
        // The `client.validate_redirect_uri()` already prevents an empty URI, this makes it obvious.
        debug_assert!(!self.redirect_uri.is_empty());

        // Technically, this additional validation is not necessary, but it does not hurt either,
        // and it just an additional defense. It's a very inexpensive operation.
        client.validate_redirect_uri(redirect_uri)?;

        if self.redirect_uri == redirect_uri {
            Ok(())
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Invalid `redirect_uri`",
            ))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct AuthCodeToSAwait {
    pub auth_code: String,
    pub await_code: String,
    pub auth_code_lifetime: i32,
    pub header_loc: String,
    pub header_origin: Option<String>,
    pub needs_user_update: bool,
}

// CRUD
impl AuthCodeToSAwait {
    pub async fn find_remove(code: &str) -> Result<Option<Self>, ErrorResponse> {
        Ok(DB::hql()
            .get_remove(Cache::AuthCode, Self::cache_idx(code))
            .await?)
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        DB::hql()
            .put(
                Cache::AuthCode,
                Self::cache_idx(&self.await_code),
                &self,
                Some(RauthyConfig::get().vars.tos.accept_timeout.as_secs() as i64),
            )
            .await?;

        Ok(())
    }
}

impl AuthCodeToSAwait {
    #[inline]
    fn cache_idx(await_code: &str) -> String {
        // Note: We don't want to simply index the await
        // codes by id to never have an accidental misuse.
        format!("tos_aw_{await_code}")
    }

    #[inline]
    pub fn generate_code() -> String {
        get_rand(64)
    }
}
