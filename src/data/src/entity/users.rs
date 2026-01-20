use crate::database::{Cache, DB};
use crate::email::email_change_confirm::send_email_confirm_change;
use crate::email::email_change_info::send_email_change_info_new;
use crate::email::password_reset::send_pwd_reset;
use crate::entity::continuation_token::ContinuationToken;
use crate::entity::groups::Group;
use crate::entity::magic_links::{MagicLink, MagicLinkUsage};
use crate::entity::password::PasswordPolicy;
use crate::entity::password::RecentPasswordsEntity;
use crate::entity::pictures::UserPicture;
use crate::entity::refresh_tokens::RefreshToken;
use crate::entity::roles::Role;
use crate::entity::sessions::Session;
use crate::entity::theme::ThemeCssFull;
use crate::entity::tos::ToS;
use crate::entity::tos_user_accept::ToSUserAccept;
use crate::entity::users_values::UserValues;
use crate::entity::webauthn::{PasskeyEntity, WebauthnServiceReq};
use crate::events::event::Event;
use crate::html::templates::{HtmlTemplate, UserEmailChangeConfirmHtml};
use crate::language::Language;
use crate::rauthy_config::RauthyConfig;
use actix_web::HttpRequest;
use argon2::PasswordHash;
use chrono::Utc;
use core::str::Split;
use hiqlite::Params;
use hiqlite_macros::params;
use rauthy_api_types::PatchOp;
use rauthy_api_types::generic::SearchParamsIdx;
use rauthy_api_types::users::{
    NewUserRegistrationRequest, NewUserRequest, UpdateUserRequest, UpdateUserSelfRequest,
    UserAccountTypeResponse, UserResponse, UserResponseSimple, UserValuesRequest,
    UserValuesResponse,
};
use rauthy_common::constants::{
    CACHE_TTL_APP, CACHE_TTL_USER, IDX_USER_COUNT, IDX_USERS, RAUTHY_ADMIN_ROLE,
};
use rauthy_common::is_hiqlite;
use rauthy_common::password_hasher::{ComparePasswords, HashPassword};
use rauthy_common::utils::{new_store_id, real_ip_from_req};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::cmp::max;
use std::default::Default;
use std::fmt::{Debug, Formatter};
use std::ops::Add;
use time::OffsetDateTime;
use tracing::{debug, error, trace};

static SQL_SAVE: &str = r#"
UPDATE USERS SET
email = $1, given_name = $2, family_name = $3, password = $4, roles = $5, groups = $6, enabled = $7,
email_verified = $8, password_expires = $9, last_login = $10, last_failed_login = $11,
failed_login_attempts = $12, language = $13, webauthn_user_id = $14, user_expires = $15,
auth_provider_id = $16, federation_uid = $17, picture_id = $18
WHERE id = $19"#;

#[derive(Debug, Clone, PartialEq)]
pub enum AccountType {
    // New -> neither password nor a passkey has been set yet
    New,
    Password,
    Passkey,
    Federated,
    /// Federated + Local Password
    FederatedPassword,
    /// Federated + Local Passkey
    FederatedPasskey,
}

impl From<AccountType> for UserAccountTypeResponse {
    fn from(value: AccountType) -> Self {
        match value {
            AccountType::New => Self::New,
            AccountType::Password => Self::Password,
            AccountType::Passkey => Self::Passkey,
            AccountType::Federated => Self::Federated,
            AccountType::FederatedPasskey => Self::FederatedPasskey,
            AccountType::FederatedPassword => Self::FederatedPassword,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub given_name: String,
    pub family_name: Option<String>,
    pub password: Option<String>,
    pub roles: String,
    pub groups: Option<String>,
    pub enabled: bool,
    pub email_verified: bool,
    pub password_expires: Option<i64>,
    pub created_at: i64,
    pub last_login: Option<i64>,
    pub last_failed_login: Option<i64>,
    pub failed_login_attempts: Option<i64>,
    pub language: Language,
    pub webauthn_user_id: Option<String>,
    pub user_expires: Option<i64>,
    pub auth_provider_id: Option<String>,
    pub federation_uid: Option<String>,
    pub picture_id: Option<String>,
}

impl Debug for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "User {{ id: {}, email: {}, given_name: {}, family_name: {:?}, password: <hidden>, \
        roles: {}, groups: {:?}, enabled: {}, email_verified: {}, password_expires: {:?}, \
        created_at: {}, last_login: {:?}, last_failed_login: {:?}, failed_login_attempts: {:?}, \
        language: {}, webauthn_user_id: {:?}, user_expires: {:?}, auth_provider_id: {:?}, \
        federation_uid: {:?}, picture_id: {:?} }}",
            self.id,
            self.email,
            self.given_name,
            self.family_name,
            self.roles,
            self.groups,
            self.enabled,
            self.email_verified,
            self.password_expires,
            self.created_at,
            self.last_login,
            self.last_failed_login,
            self.failed_login_attempts,
            self.language,
            self.webauthn_user_id,
            self.user_expires,
            self.auth_provider_id,
            self.federation_uid,
            self.picture_id,
        )
    }
}

impl From<tokio_postgres::Row> for User {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            email: row.get("email"),
            given_name: row.get("given_name"),
            family_name: row.get("family_name"),
            password: row.get("password"),
            roles: row.get("roles"),
            groups: row.get("groups"),
            enabled: row.get("enabled"),
            email_verified: row.get("email_verified"),
            password_expires: row.get("password_expires"),
            created_at: row.get("created_at"),
            last_login: row.get("last_login"),
            last_failed_login: row.get("last_failed_login"),
            failed_login_attempts: row.get("failed_login_attempts"),
            language: Language::from(row.get::<_, String>("language")),
            webauthn_user_id: row.get("webauthn_user_id"),
            user_expires: row.get("user_expires"),
            auth_provider_id: row.get("auth_provider_id"),
            federation_uid: row.get("federation_uid"),
            picture_id: row.get("picture_id"),
        }
    }
}

// CRUD
impl User {
    pub async fn invalidate_cache(user_id: &str, email: &str) -> Result<(), ErrorResponse> {
        let client = DB::hql();

        let idx = format!("{IDX_USERS}_{user_id}");
        client.delete(Cache::User, idx).await?;

        let idx = format!("{IDX_USERS}_{email}");
        client.delete(Cache::User, idx).await?;

        Ok(())
    }

    pub async fn count() -> Result<i64, ErrorResponse> {
        let client = DB::hql();

        if let Some(count) = client.get(Cache::App, IDX_USER_COUNT).await? {
            return Ok(count);
        }

        let sql = "SELECT COUNT (*) AS count FROM users";
        let count: i64 = if is_hiqlite() {
            client
                .query_raw(sql, params!())
                .await?
                .remove(0)
                .get("count")
        } else {
            DB::pg_query_rows(sql, &[], 1).await?.remove(0).get("count")
        };

        client
            .put(Cache::App, IDX_USER_COUNT, &count, CACHE_TTL_APP)
            .await?;

        Ok(count)
    }

    async fn count_inc() -> Result<(), ErrorResponse> {
        let mut count = Self::count().await?;
        // theoretically, we could have overlaps here, but we don't really care
        // -> used for dynamic pagination only
        count += 1;
        DB::hql()
            .put(Cache::App, IDX_USER_COUNT, &count, CACHE_TTL_APP)
            .await?;
        Ok(())
    }

    async fn count_dec() -> Result<(), ErrorResponse> {
        let mut count = Self::count().await?;
        // theoretically, we could have overlaps here, but we don't really care
        // -> used for dynamic pagination only
        count -= 1;
        DB::hql()
            .put(Cache::App, IDX_USER_COUNT, &count, CACHE_TTL_APP)
            .await?;
        Ok(())
    }

    pub async fn create(
        new_user: User,
        post_reset_redirect_uri: Option<String>,
        user_tz: Option<&str>,
    ) -> Result<Self, ErrorResponse> {
        let slf = Self::insert(new_user).await?;

        let magic_link = MagicLink::create(
            slf.id.clone(),
            RauthyConfig::get().vars.lifetimes.magic_link_pwd_first as i64,
            MagicLinkUsage::NewUser(post_reset_redirect_uri),
        )
        .await?;
        send_pwd_reset(&magic_link, &slf, user_tz).await;

        Ok(slf)
    }

    pub async fn create_federated(new_user: User) -> Result<Self, ErrorResponse> {
        Self::insert(new_user).await
    }

    pub async fn create_from_new(new_user_req: NewUserRequest) -> Result<User, ErrorResponse> {
        let tz = new_user_req.tz.clone();
        let new_user = User::from_new_user_req(new_user_req).await?;
        let user = User::create(new_user, None, tz.as_deref()).await?;

        if tz.is_some() && tz.as_deref() != Some("UTC") && tz.as_deref() != Some("Etc/UTC") {
            UserValues::insert(
                user.id.clone(),
                UserValuesRequest {
                    birthdate: None,
                    phone: None,
                    street: None,
                    zip: None,
                    city: None,
                    country: None,
                    tz,
                },
                None,
            )
            .await?;
        }

        Ok(user)
    }

    /// Inserts a user from the open registration endpoint into the database.
    pub async fn create_from_reg(
        req_data: NewUserRegistrationRequest,
        lang: Language,
    ) -> Result<User, ErrorResponse> {
        // pre-uniqueness checks for better UX and error handling
        User::validate_email_free(req_data.email.clone()).await?;
        if let Some(preferred_username) = &req_data.preferred_username {
            UserValues::validate_preferred_username_free(preferred_username.clone()).await?;
        }

        let mut new_user = Self {
            email: req_data.email.to_lowercase(),
            given_name: req_data.given_name.unwrap_or_default(),
            family_name: req_data.family_name,
            ..Default::default()
        };
        new_user.language = lang;
        let new_user = User::create(
            new_user,
            req_data.redirect_uri,
            req_data
                .user_values
                .as_ref()
                .and_then(|uv| uv.tz.as_deref()),
        )
        .await?;

        if let Some(uv) = req_data.user_values {
            UserValues::insert(new_user.id.clone(), uv, req_data.preferred_username).await?;
        }

        Ok(new_user)
    }

    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        Session::delete_by_user(&self.id).await?;

        if let Some(picture_id) = &self.picture_id {
            UserPicture::remove(picture_id.clone(), self.id.clone()).await?;
        }

        let sql = "DELETE FROM users WHERE id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(&self.id)).await?;
        } else {
            DB::pg_execute(sql, &[&self.id]).await?;
        }

        Self::invalidate_cache(&self.id, &self.email).await?;
        Self::count_dec().await?;

        Ok(())
    }

    pub async fn exists(id: String) -> Result<(), ErrorResponse> {
        let idx = format!("{IDX_USERS}_{id}");

        let opt: Option<Self> = DB::hql().get(Cache::User, idx).await?;
        if opt.is_some() {
            return Ok(());
        }

        let sql = "SELECT 1 FROM users WHERE id = $1";
        let user_exists = if is_hiqlite() {
            DB::hql().query_raw(sql, params!(id)).await?.is_empty()
        } else {
            DB::pg_query_rows(sql, &[&id], 1).await?.is_empty()
        };

        if !user_exists {
            return Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "user does not exist",
            ));
        }

        Ok(())
    }

    pub async fn find(id: String) -> Result<Self, ErrorResponse> {
        let idx = format!("{IDX_USERS}_{id}");
        let client = DB::hql();

        if let Some(slf) = client.get(Cache::User, &idx).await? {
            return Ok(slf);
        }

        let sql = "SELECT * FROM users WHERE id = $1";
        let slf: Self = if is_hiqlite() {
            client.query_as_one(sql, params!(id)).await?
        } else {
            DB::pg_query_one(sql, &[&id]).await?
        };

        client.put(Cache::User, idx, &slf, CACHE_TTL_USER).await?;
        Ok(slf)
    }

    pub async fn find_by_email(email: String) -> Result<User, ErrorResponse> {
        let email = email.to_lowercase();

        let idx = format!("{IDX_USERS}_{email}");
        let client = DB::hql();

        if let Some(slf) = client.get(Cache::User, &idx).await? {
            return Ok(slf);
        }

        let sql = "SELECT * FROM users WHERE email = $1";
        let slf = if is_hiqlite() {
            client.query_as_one(sql, params!(email)).await?
        } else {
            DB::pg_query_one(sql, &[&email]).await?
        };

        client.put(Cache::User, idx, &slf, CACHE_TTL_USER).await?;
        Ok(slf)
    }

    pub async fn find_by_federation(
        auth_provider_id: &str,
        federation_uid: &str,
    ) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM users WHERE auth_provider_id = $1 AND federation_uid = $2";
        let slf = if is_hiqlite() {
            DB::hql()
                .query_as_one(sql, params!(auth_provider_id, federation_uid))
                .await?
        } else {
            DB::pg_query_one(sql, &[&auth_provider_id, &federation_uid]).await?
        };

        Ok(slf)
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM users ORDER BY created_at ASC";
        let res = if is_hiqlite() {
            DB::hql().query_as(sql, params!()).await?
        } else {
            // for big instances, fetching the count from the cache upfront is a speed improvement
            // because we only need a single memory allocation for the internal `Vec<_>`
            let count = Self::count().await?;
            DB::pg_query(sql, &[], count as usize).await?
        };

        Ok(res)
    }

    pub async fn find_all_simple() -> Result<Vec<UserResponseSimple>, ErrorResponse> {
        let sql = r#"
SELECT id, email, given_name, family_name, created_at, last_login, picture_id
FROM users
ORDER BY created_at ASC"#;

        let res = if is_hiqlite() {
            DB::hql().query_as(sql, params!()).await?
        } else {
            // for big instances, fetching the count from the cache upfront is a speed improvement
            // because we only need a single memory allocation for the internal `Vec<_>`
            let count = Self::count().await?;
            DB::pg_query(sql, &[], count as usize).await?
        };

        Ok(res)
    }

    pub async fn find_batch(
        after_email: &str,
        from_created_at: i64,
        batch_size: u16,
    ) -> Result<Vec<Self>, ErrorResponse> {
        let sql = r#"
SELECT * FROM users
WHERE created_at >= $1 AND email > $2
ORDER BY created_at ASC, email ASC
LIMIT $3"#;

        let batch_size = batch_size as i64;
        let res = if is_hiqlite() {
            DB::hql()
                .query_as(sql, params!(from_created_at, after_email, batch_size))
                .await?
        } else {
            DB::pg_query(
                sql,
                &[&from_created_at, &after_email, &batch_size],
                batch_size as usize,
            )
            .await?
        };

        Ok(res)
    }

    /// This is a very expensive query using `LIKE`, use only when necessary.
    pub async fn find_with_group(group_name: &str) -> Result<Vec<Self>, ErrorResponse> {
        let like = format!("%{group_name}%");
        let sql = "SELECT * FROM users WHERE groups LIKE $1";

        let res = if is_hiqlite() {
            DB::hql().query_as(sql, params!(like)).await?
        } else {
            let count = Self::count().await? as usize;
            DB::pg_query(sql, &[&like], count).await?
        };

        Ok(res)
    }

    /// This is a very expensive query using `LIKE`, use only when necessary.
    pub async fn find_with_role(role_name: &str) -> Result<Vec<Self>, ErrorResponse> {
        let like = format!("%{role_name}%");
        let sql = "SELECT * FROM users WHERE roles LIKE $1";

        let res = if is_hiqlite() {
            DB::hql().query_as(sql, params!(like)).await?
        } else {
            let count = Self::count().await? as usize;
            DB::pg_query(sql, &[&like], count).await?
        };

        Ok(res)
    }

    pub async fn find_expired() -> Result<Vec<Self>, ErrorResponse> {
        let now = Utc::now().add(chrono::Duration::seconds(10)).timestamp();
        let sql = "SELECT * FROM users WHERE user_expires < $1";

        let res = if is_hiqlite() {
            DB::hql().query_as(sql, params!(now)).await?
        } else {
            let count = Self::count().await? as usize;
            DB::pg_query(sql, &[&now], count).await?
        };

        Ok(res)
    }

    pub async fn find_for_fed_cm_validated(user_id: String) -> Result<Self, ErrorResponse> {
        // We will stick to the WWW-Authenticate header for now and use duplicated code from
        // some OAuth2 api for now until the spec has settled on an error behavior.
        debug!("Looking up FedCM user_id {user_id}");
        let slf = Self::find(user_id).await.map_err(|_| {
            debug!("FedCM user not found");
            ErrorResponse::new(
                ErrorResponseType::WWWAuthenticate("user-not-found".to_string()),
                "The user has not been found",
            )
        })?;

        // reject the request if user has been disabled, even when the token is still valid
        if !slf.enabled || slf.check_expired().is_err() {
            debug!("FedCM user is disabled");
            return Err(ErrorResponse::new(
                ErrorResponseType::WWWAuthenticate("user-disabled".to_string()),
                "The user has been disabled",
            ));
        }

        Ok(slf)
    }

    #[tracing::instrument]
    pub async fn find_paginated(
        continuation_token: Option<ContinuationToken>,
        page_size: i64,
        offset: i64,
        backwards: bool,
    ) -> Result<(Vec<UserResponseSimple>, Option<ContinuationToken>), ErrorResponse> {
        let size_hint = page_size as usize;

        let res = if let Some(token) = continuation_token {
            if backwards {
                let sql = r#"
SELECT id, email, given_name, family_name, created_at, last_login, picture_id
FROM users
WHERE created_at <= $1 AND id != $2
ORDER BY created_at DESC
LIMIT $3
OFFSET $4"#;

                let mut res: Vec<UserResponseSimple> = if is_hiqlite() {
                    DB::hql()
                        .query_as(sql, params!(token.ts, token.id, page_size, offset))
                        .await?
                } else {
                    DB::pg_query(sql, &[&token.ts, &token.id, &page_size, &offset], size_hint)
                        .await?
                };
                res.reverse();
                res
            } else {
                let sql = r#"
SELECT id, email, given_name, family_name, created_at, last_login, picture_id
FROM users
WHERE created_at >= $1 AND id != $2
ORDER BY created_at ASC
LIMIT $3
OFFSET $4"#;

                if is_hiqlite() {
                    DB::hql()
                        .query_as(sql, params!(token.ts, token.id, page_size, offset))
                        .await?
                } else {
                    DB::pg_query(sql, &[&token.ts, &token.id, &page_size, &offset], size_hint)
                        .await?
                }
            }
        } else if backwards {
            // backwards without any continuation token will simply
            // serve the last elements without any other conditions
            let sql = r#"
SELECT id, email, given_name, family_name, created_at, last_login, picture_id
FROM users
ORDER BY created_at DESC
LIMIT $1
OFFSET $2"#;

            let mut res = if is_hiqlite() {
                DB::hql().query_as(sql, params!(page_size, offset)).await?
            } else {
                DB::pg_query(sql, &[&page_size, &offset], size_hint).await?
            };
            res.reverse();
            res
        } else {
            let sql = r#"
SELECT id, email, given_name, family_name, created_at, last_login, picture_id
FROM users
ORDER BY created_at ASC
LIMIT $1
OFFSET $2"#;

            if is_hiqlite() {
                DB::hql().query_as(sql, params!(page_size, offset)).await?
            } else {
                DB::pg_query(sql, &[&page_size, &offset], size_hint).await?
            }
        };

        let token = res
            .last()
            .map(|entry| ContinuationToken::new(entry.id.clone(), entry.created_at));

        Ok((res, token))
    }

    pub async fn insert(new_user: User) -> Result<Self, ErrorResponse> {
        let lang = new_user.language.as_str();
        let sql = r#"
INSERT INTO users
(id, email, given_name, family_name, roles, groups, enabled, email_verified, created_at,
last_login, language, user_expires, auth_provider_id, federation_uid, picture_id)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        &new_user.id,
                        &new_user.email,
                        &new_user.given_name,
                        &new_user.family_name,
                        &new_user.roles,
                        &new_user.groups,
                        new_user.enabled,
                        new_user.email_verified,
                        new_user.created_at,
                        new_user.last_login,
                        lang,
                        new_user.user_expires,
                        &new_user.auth_provider_id,
                        &new_user.federation_uid,
                        &new_user.picture_id
                    ),
                )
                .await
                .map_err(|err| {
                    let err = ErrorResponse::from(err);
                    if err.message.contains("UNIQUE") {
                        ErrorResponse::new(
                            ErrorResponseType::NotAccepted,
                            "UNIQUE constraint on: 'email'",
                        )
                    } else {
                        err
                    }
                })?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &new_user.id,
                    &new_user.email,
                    &new_user.given_name,
                    &new_user.family_name,
                    &new_user.roles,
                    &new_user.groups,
                    &new_user.enabled,
                    &new_user.email_verified,
                    &new_user.created_at,
                    &new_user.last_login,
                    &lang,
                    &new_user.user_expires,
                    &new_user.auth_provider_id,
                    &new_user.federation_uid,
                    &new_user.picture_id,
                ],
            )
            .await
            .map_err(|err| {
                if err.message.contains("UNIQUE") {
                    ErrorResponse::new(
                        ErrorResponseType::NotAccepted,
                        "UNIQUE constraint on: 'email'",
                    )
                } else {
                    err
                }
            })?;
        }

        Self::count_inc().await?;

        Ok(new_user)
    }

    /// Caution: Results will only contain values necessary for SCIM syncs.
    pub async fn find_for_scim_sync(
        last_created_ts: i64,
        limit: u16,
    ) -> Result<Vec<(Self, UserValues)>, ErrorResponse> {
        let sql = r#"
SELECT u.id AS user_id, email, email_verified, given_name, family_name, roles, groups, enabled,
    created_at, language, picture_id, birthdate, phone, street, zip, city, country
FROM users u
LEFT JOIN users_values uv ON u.id = uv.id
WHERE u.created_at > $1
ORDER BY created_at ASC
LIMIT $2"#;

        let mut res = Vec::with_capacity(limit as usize);

        if is_hiqlite() {
            let rows = DB::hql()
                .query_raw(sql, params!(last_created_ts, limit))
                .await?;
            for mut row in rows {
                let user = Self {
                    id: row.get("user_id"),
                    email: row.get("email"),
                    given_name: row.get("given_name"),
                    family_name: row.get("family_name"),
                    password: None,
                    roles: row.get("roles"),
                    groups: row.get("groups"),
                    enabled: row.get("enabled"),
                    email_verified: row.get("email_verified"),
                    password_expires: None,
                    created_at: row.get("created_at"),
                    last_login: None,
                    last_failed_login: None,
                    failed_login_attempts: None,
                    language: Language::from(row.get::<String>("language")),
                    webauthn_user_id: None,
                    user_expires: None,
                    auth_provider_id: None,
                    federation_uid: None,
                    picture_id: row.get("picture_id"),
                };
                let values = UserValues {
                    id: user.id.clone(),
                    birthdate: row.get("birthdate"),
                    phone: row.get("phone"),
                    street: row.get("street"),
                    zip: row.get("zip"),
                    city: row.get("city"),
                    country: row.get("country"),
                    preferred_username: row.get("preferred_username"),
                    tz: row.get("tz"),
                };
                res.push((user, values));
            }
        } else {
            let rows = DB::pg_query_rows(sql, &[&last_created_ts, &(limit as i32)], limit as usize)
                .await?;
            for row in rows {
                let user = Self {
                    id: row.get("user_id"),
                    email: row.get("email"),
                    given_name: row.get("given_name"),
                    family_name: row.get("family_name"),
                    password: None,
                    roles: row.get("roles"),
                    groups: row.get("groups"),
                    enabled: row.get("enabled"),
                    email_verified: row.get("email_verified"),
                    password_expires: None,
                    created_at: row.get("created_at"),
                    last_login: None,
                    last_failed_login: None,
                    failed_login_attempts: None,
                    language: Language::from(row.get::<_, String>("language")),
                    webauthn_user_id: None,
                    user_expires: None,
                    auth_provider_id: None,
                    federation_uid: None,
                    picture_id: row.get("picture_id"),
                };
                let values = UserValues {
                    id: user.id.clone(),
                    birthdate: row.get("birthdate"),
                    phone: row.get("phone"),
                    street: row.get("street"),
                    zip: row.get("zip"),
                    city: row.get("city"),
                    country: row.get("country"),
                    preferred_username: row.get("preferred_username"),
                    tz: row.get("tz"),
                };
                res.push((user, values));
            }
        }

        Ok(res)
    }

    pub async fn provider_unlink(user_id: String) -> Result<Self, ErrorResponse> {
        // we need to find the user first and validate that it has been set up properly
        // to work without a provider
        let mut slf = Self::find(user_id).await?;
        if slf.password.is_none() && !slf.has_webauthn_enabled() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "You must have at least a password or passkey set up before you can \
                remove a provider link",
            ));
        }

        slf.auth_provider_id = None;
        slf.federation_uid = None;
        slf.save(None).await?;

        Ok(slf)
    }

    /// Appends multiple necessary transaction queries to update a user to the given `Vec<_>`.
    ///
    /// CAUTION:
    /// You MUST clear the updated users from the cache after a successful txn commit!
    /// Either clear the whole `Cache::User` cache or use `User::invalidate_cache()`.
    ///
    /// CAUTION:
    /// DO NOT use this function to update a user's `email` or `enabled` state, as this would
    /// need additional cache cleanup and E-Mail handling!
    pub fn save_txn_append(self, txn: &mut Vec<(&str, Params)>) {
        txn.push((
            SQL_SAVE,
            params!(
                self.email,
                self.given_name,
                self.family_name,
                self.password,
                self.roles,
                self.groups,
                self.enabled,
                self.email_verified,
                self.password_expires,
                self.last_login,
                self.last_failed_login,
                self.failed_login_attempts,
                self.language.as_str().to_string(),
                self.webauthn_user_id,
                self.user_expires,
                self.auth_provider_id,
                self.federation_uid,
                self.picture_id,
                self.id
            ),
        ));
    }

    /// CAUTION:
    /// You MUST clear the updated users from the cache after a successful txn commit!
    /// Either clear the whole `Cache::User` cache or use `User::invalidate_cache()`.
    ///
    /// CAUTION:
    /// DO NOT use this function to update a user's `email` or `enabled` state!
    pub async fn save_txn(
        &self,
        txn: &deadpool_postgres::Transaction<'_>,
    ) -> Result<(), ErrorResponse> {
        let lang = self.language.as_str();

        DB::pg_txn_append(
            txn,
            SQL_SAVE,
            &[
                &self.email,
                &self.given_name,
                &self.family_name,
                &self.password,
                &self.roles,
                &self.groups,
                &self.enabled,
                &self.email_verified,
                &self.password_expires,
                &self.last_login,
                &self.last_failed_login,
                &self.failed_login_attempts,
                &lang,
                &self.webauthn_user_id,
                &self.user_expires,
                &self.auth_provider_id,
                &self.federation_uid,
                &self.picture_id,
                &self.id,
            ],
        )
        .await?;

        Ok(())
    }

    pub async fn save(&self, old_email: Option<String>) -> Result<(), ErrorResponse> {
        if old_email.is_some() {
            User::is_email_free(self.email.clone()).await?;
        }

        let lang = self.language.as_str();
        let client = DB::hql();

        if is_hiqlite() {
            client
                .execute(
                    SQL_SAVE,
                    params!(
                        &self.email,
                        &self.given_name,
                        &self.family_name,
                        &self.password,
                        &self.roles,
                        &self.groups,
                        self.enabled,
                        self.email_verified,
                        self.password_expires,
                        self.last_login,
                        self.last_failed_login,
                        self.failed_login_attempts,
                        lang,
                        &self.webauthn_user_id,
                        self.user_expires,
                        &self.auth_provider_id,
                        &self.federation_uid,
                        &self.picture_id,
                        &self.id
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                SQL_SAVE,
                &[
                    &self.email,
                    &self.given_name,
                    &self.family_name,
                    &self.password,
                    &self.roles,
                    &self.groups,
                    &self.enabled,
                    &self.email_verified,
                    &self.password_expires,
                    &self.last_login,
                    &self.last_failed_login,
                    &self.failed_login_attempts,
                    &lang,
                    &self.webauthn_user_id,
                    &self.user_expires,
                    &self.auth_provider_id,
                    &self.federation_uid,
                    &self.picture_id,
                    &self.id,
                ],
            )
            .await?;
        }

        if !self.enabled {
            Session::invalidate_for_user(&self.id).await?;
            RefreshToken::invalidate_for_user(&self.id).await?;
        }

        if let Some(email) = old_email {
            let idx = format!("{IDX_USERS}_{email}");
            client.delete(Cache::User, idx).await?;
        }

        let idx = format!("{IDX_USERS}_{}", self.id);
        client.put(Cache::User, idx, self, CACHE_TTL_USER).await?;

        let idx = format!("{IDX_USERS}_{}", self.email);
        client.put(Cache::User, idx, self, CACHE_TTL_USER).await?;

        Ok(())
    }

    /// Caution: Uses regex / LIKE on the database -> very costly query
    pub async fn search(
        idx: &SearchParamsIdx,
        q: &str,
        limit: i64,
    ) -> Result<Vec<UserResponseSimple>, ErrorResponse> {
        let q = format!("%{q}%");
        let size_hint = max(limit, 1) as usize;

        let res = match idx {
            SearchParamsIdx::Id | SearchParamsIdx::UserId => {
                let sql = r#"
SELECT id, email, given_name, family_name, created_at, last_login, picture_id
FROM users
WHERE id LIKE $1
ORDER BY created_at ASC
LIMIT $2"#;

                if is_hiqlite() {
                    DB::hql().query_as(sql, params!(q, limit)).await?
                } else {
                    DB::pg_query(sql, &[&q, &limit], size_hint).await?
                }
            }
            SearchParamsIdx::Email => {
                let sql = r#"
SELECT id, email, given_name, family_name, created_at, last_login, picture_id
FROM users
WHERE email LIKE $1
ORDER BY created_at ASC
LIMIT $2"#;

                if is_hiqlite() {
                    DB::hql().query_as(sql, params!(q, limit)).await?
                } else {
                    DB::pg_query(sql, &[&q, &limit], size_hint).await?
                }
            }
            _ => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "supported search idx for users: id / user_id, email",
                ));
            }
        };

        Ok(res)
    }

    pub async fn set_email_verified(
        user_id: String,
        email_verified: bool,
    ) -> Result<(), ErrorResponse> {
        let sql = "UPDATE users SET email_verified = $1 WHERE id = $2";
        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(email_verified, user_id))
                .await?;
        } else {
            DB::pg_execute(sql, &[&email_verified, &user_id]).await?;
        }
        Ok(())
    }

    /// Builds an `UpdateUserRequest` solely from data inside the DB. This data can then be used
    /// for `PatchOp`s. Build `UpdateUserRequest` to be able to re-use all other existing
    /// functionality and checks.
    ///
    /// Returns `(UpdateUserRequest, preferred_username)`
    pub async fn upd_req_from_db(
        user_id: String,
    ) -> Result<(UpdateUserRequest, Option<String>), ErrorResponse> {
        let u = Self::find(user_id).await?;

        let roles = u.get_roles();
        let groups = Some(u.get_groups());
        let language = Some(rauthy_api_types::generic::Language::from(u.language));

        let (user_values, preferred_username) = if let Some(uv) = UserValues::find(&u.id).await? {
            (
                Some(UserValuesRequest {
                    birthdate: uv.birthdate,
                    phone: uv.phone,
                    street: uv.street,
                    zip: uv.zip,
                    city: uv.city,
                    country: uv.country,
                    tz: uv.tz,
                }),
                uv.preferred_username,
            )
        } else {
            (None, None)
        };

        Ok((
            UpdateUserRequest {
                email: u.email,
                given_name: if u.given_name.is_empty() {
                    None
                } else {
                    Some(u.given_name)
                },
                family_name: u.family_name,
                language,
                // Must be None here to avoid triggering the "new password set" logic for each user
                // update. This req is being used for `PatchOp`s and if the PatchOp contains a
                // `password`, it will be updated later on.
                password: None,
                roles,
                groups,
                enabled: u.enabled,
                email_verified: u.email_verified,
                user_expires: u.user_expires,
                user_values,
            },
            preferred_username,
        ))
    }

    /// Returns `(UpdateUserRequest, preferred_username)`
    pub async fn patch(
        user_id: String,
        payload: PatchOp,
    ) -> Result<(UpdateUserRequest, Option<String>), ErrorResponse> {
        let (mut upd_req, preferred_username) = User::upd_req_from_db(user_id).await?;
        let mut uv = upd_req.user_values.unwrap_or_default();

        for put in payload.put {
            match put.key.as_str() {
                "email" => upd_req.email = put.value.as_str().unwrap_or_default().to_string(),
                "password" => {
                    upd_req.password = Some(put.value.as_str().unwrap_or_default().to_string())
                }
                "given_name" => upd_req.given_name = put.value.as_str().map(String::from),
                "family_name" => upd_req.family_name = put.value.as_str().map(String::from),
                "language" => {
                    let lang = rauthy_api_types::generic::Language::from(Language::from(
                        put.value.as_str().unwrap_or_default(),
                    ));
                    upd_req.language = Some(lang)
                }
                "roles" => {
                    upd_req.roles = put
                        .value
                        .as_array()
                        .map(|arr| {
                            arr.iter()
                                .map(|v| v.as_str().unwrap_or_default().to_string())
                                .collect()
                        })
                        .unwrap_or_default()
                }
                "groups" => {
                    upd_req.groups = Some(
                        put.value
                            .as_array()
                            .map(|arr| {
                                arr.iter()
                                    .map(|v| v.as_str().unwrap_or_default().to_string())
                                    .collect()
                            })
                            .unwrap_or_default(),
                    )
                }
                "enabled" => upd_req.enabled = put.value.as_bool().unwrap_or(true),
                "email_verified" => upd_req.email_verified = put.value.as_bool().unwrap_or(true),
                "user_expires" => upd_req.user_expires = put.value.as_i64(),
                key => {
                    if let Some(key) = key.strip_prefix("user_values.") {
                        match key {
                            "birthdate" => uv.birthdate = put.value.as_str().map(String::from),
                            "phone" => uv.phone = put.value.as_str().map(String::from),
                            "street" => uv.street = put.value.as_str().map(String::from),
                            "zip" => uv.zip = put.value.as_str().map(String::from),

                            "city" => uv.city = put.value.as_str().map(String::from),
                            "country" => uv.country = put.value.as_str().map(String::from),
                            "tz" => uv.tz = put.value.as_str().map(String::from),
                            v => {
                                return Err(ErrorResponse::new(
                                    ErrorResponseType::BadRequest,
                                    format!("Invalid patch op put value for user: user_values.{v}"),
                                ));
                            }
                        }
                    } else {
                        return Err(ErrorResponse::new(
                            ErrorResponseType::BadRequest,
                            format!("Invalid patch op put value for user: {key}"),
                        ));
                    }
                }
            }
        }

        for key in payload.del {
            match key.as_str() {
                "given_name" => upd_req.given_name = None,
                "family_name" => upd_req.family_name = None,
                "language" => upd_req.language = None,
                "roles" => upd_req.roles = Vec::default(),
                "groups" => upd_req.groups = None,
                "user_expires" => upd_req.user_expires = None,
                "user_values" => uv = UserValuesRequest::default(),
                key => {
                    if let Some(key) = key.strip_prefix("user_values.") {
                        match key {
                            "birthdate" => uv.birthdate = None,
                            "phone" => uv.phone = None,
                            "street" => uv.street = None,
                            "zip" => uv.zip = None,
                            "city" => uv.city = None,
                            "country" => uv.country = None,
                            "tz" => uv.tz = None,
                            v => {
                                return Err(ErrorResponse::new(
                                    ErrorResponseType::BadRequest,
                                    format!("Invalid patch op del value for user: user_values.{v}"),
                                ));
                            }
                        }
                    } else {
                        return Err(ErrorResponse::new(
                            ErrorResponseType::BadRequest,
                            format!("Invalid patch op del value for user: {key}"),
                        ));
                    }
                }
            }
        }

        if uv != UserValuesRequest::default() {
            upd_req.user_values = Some(uv);
        } else {
            upd_req.user_values = None;
        }

        Ok((upd_req, preferred_username))
    }

    pub async fn update(
        id: String,
        mut upd_user: UpdateUserRequest,
        user: Option<User>,
        preferred_username: Option<String>,
    ) -> Result<(User, Option<UserValues>, bool), ErrorResponse> {
        let mut user = match user {
            None => User::find(id).await?,
            Some(user) => user,
        };
        upd_user.email = upd_user.email.to_lowercase();
        let old_email = if user.email != upd_user.email {
            Some(user.email.clone())
        } else {
            None
        };

        user.email = upd_user.email;
        user.given_name = upd_user.given_name.unwrap_or_default();
        user.family_name = upd_user.family_name;

        if let Some(lang) = upd_user.language {
            user.language = lang.into();
        }

        if let Some(password) = &upd_user.password {
            user.apply_password_rules(password).await?;
        }

        let is_admin_before_update = user.is_admin();
        user.roles = Role::sanitize(upd_user.roles).await?;
        user.groups = Group::sanitize(upd_user.groups).await?;

        user.enabled = upd_user.enabled;
        user.email_verified = upd_user.email_verified;
        user.user_expires = upd_user.user_expires;

        user.save(old_email.clone()).await?;

        if upd_user.password.is_some() {
            RauthyConfig::get()
                .tx_events
                .send_async(Event::user_password_reset(
                    format!("Reset done by admin for user {}", user.email),
                    None,
                ))
                .await
                .unwrap();
        }

        if let Some(old_email) = old_email.as_ref() {
            // if the user was saved successfully and the email was changed, invalidate all existing
            // sessions with the old address and send out notifications to the users addresses
            Session::invalidate_for_user(&user.id).await?;

            // send out confirmation E-Mails to both addresses
            send_email_confirm_change(&user, &user.email, &user.email, true).await;
            send_email_confirm_change(&user, old_email, &user.email, true).await;

            let event_text = format!("Change by admin: {old_email} -> {}", user.email);
            RauthyConfig::get()
                .tx_events
                .send_async(Event::user_email_change(event_text, None))
                .await
                .unwrap();
        }

        let is_new_admin = !is_admin_before_update && user.is_admin();

        // finally, update the custom users values
        let user_values = if let Some(values) = upd_user.user_values {
            UserValues::upsert(user.id.clone(), values).await?
        } else if let Some(preferred_username) = preferred_username {
            UserValues::upsert(user.id.clone(), UserValuesRequest::default()).await?;
            Some(UserValues {
                id: user.id.clone(),
                preferred_username: Some(preferred_username),
                ..Default::default()
            })
        } else {
            UserValues::delete(user.id.clone()).await?;
            None
        };

        Ok((user, user_values, is_new_admin))
    }

    pub async fn update_language(&self) -> Result<(), ErrorResponse> {
        let lang = self.language.as_str();
        let sql = "UPDATE users SET language = $1 WHERE id = $2";

        if is_hiqlite() {
            DB::hql().execute(sql, params!(lang, &self.id)).await?;
        } else {
            DB::pg_execute(sql, &[&lang, &self.id]).await?;
        }

        Ok(())
    }

    /// Updates a user from himself. This is needed for the account page to make each user able to
    /// update its own data.
    pub async fn update_self_req(
        id: String,
        upd_user: UpdateUserSelfRequest,
    ) -> Result<(User, Option<UserValues>, bool), ErrorResponse> {
        let user = User::find(id.clone()).await?;

        let mut password = None;
        if let Some(pwd_new) = upd_user.password_new {
            if let Some(pwd_curr) = upd_user.password_current {
                user.validate_password(pwd_curr).await?;
            } else if let Some(mfa_code) = upd_user.mfa_code {
                let svc_req = WebauthnServiceReq::find(mfa_code).await?;
                if svc_req.user_id != user.id {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::Forbidden,
                        "User ID does not match",
                    ));
                }
                svc_req.delete().await?;
            } else {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "Cannot set a new password without the current one",
                ));
            }
            password = Some(pwd_new);
        }

        let email_updated = if let Some(email) = upd_user.email.map(|email| email.to_lowercase()) {
            // if the email should be updated, we do not do it directly -> send out confirmation
            // email to old AND new address
            if email != user.email {
                // invalidate possibly other existing MagicLinks of the same type
                MagicLink::invalidate_all_email_change(&user.id).await?;

                let ml = MagicLink::create(
                    user.id.clone(),
                    60,
                    MagicLinkUsage::EmailChange(email.clone()),
                )
                .await?;

                send_email_change_info_new(
                    &ml,
                    &user,
                    upd_user.user_values.as_ref().and_then(|v| v.tz.as_deref()),
                    email,
                )
                .await;

                true
            } else {
                false
            }
        } else {
            false
        };

        let groups = if user.groups.is_some() {
            Some(user.get_groups())
        } else {
            None
        };
        let req = UpdateUserRequest {
            // never update the email directly here, only via email confirmation action from the user
            email: user.email.clone(),
            given_name: upd_user.given_name,
            family_name: upd_user.family_name,
            language: upd_user.language,
            password,
            roles: user.get_roles(),
            groups,
            enabled: user.enabled,
            email_verified: user.email_verified,
            user_expires: user.user_expires,
            user_values: upd_user.user_values,
        };

        let preferred_username = UserValues::find_preferred_username(&id).await?;

        // a user cannot become a new admin from a self-req
        let (user, user_values, _is_new_admin) =
            User::update(id, req, Some(user), preferred_username).await?;

        Ok((user, user_values, email_updated))
    }

    /// Converts a user account from as password account type to passkey only with all necessary
    /// checks included.
    pub async fn convert_to_passkey(id: String) -> Result<(), ErrorResponse> {
        let mut user = User::find(id.clone()).await?;

        if user.account_type() != AccountType::Password {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Only AccountType::Password can be converted",
            ));
        }

        if !user.has_webauthn_enabled() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Account type conversion can only happen with at least one active Passkey",
            ));
        }

        let pks = PasskeyEntity::find_for_user_with_uv(&user.id).await?;
        if pks.is_empty() {
            return Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "Could not find any passkeys with active User Verification".to_string(),
            ));
        }

        user.password = None;
        user.password_expires = None;

        user.save(None).await?;
        Ok(())
    }

    pub async fn validate_email_free(email: String) -> Result<(), ErrorResponse> {
        let sql = "SELECT 1 FROM users WHERE email = $1";

        let is_free = if is_hiqlite() {
            DB::hql().query_raw_one(sql, params!(email)).await.is_err()
        } else {
            DB::pg_query_one_row(sql, &[&email]).await.is_err()
        };

        if is_free {
            Ok(())
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::NotAccepted,
                "UNIQUE constraint on: 'email'",
            ))
        }
    }
}

impl User {
    #[inline]
    pub fn account_type(&self) -> AccountType {
        if self.federation_uid.is_some() {
            if self.password.is_some() {
                AccountType::FederatedPassword
            } else if self.has_webauthn_enabled() {
                AccountType::FederatedPasskey
            } else {
                AccountType::Federated
            }
        } else if self.password.is_some() {
            AccountType::Password
        } else if self.has_webauthn_enabled() {
            AccountType::Passkey
        } else {
            AccountType::New
        }
    }

    pub async fn apply_password_rules(&mut self, plain_pwd: &str) -> Result<(), ErrorResponse> {
        let rules = PasswordPolicy::find().await?;

        if plain_pwd.len() < rules.length_min as usize {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!("Minimum password length is {}", rules.length_min),
            ));
        }
        if plain_pwd.len() > rules.length_max as usize {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!("Maximum password length is {}", rules.length_max),
            ));
        }

        let mut count_lower = 0;
        let mut count_upper = 0;
        let mut count_digit = 0;
        let mut count_special = 0;

        plain_pwd.chars().for_each(|c| {
            if c.is_lowercase() {
                count_lower += 1;
            } else if c.is_uppercase() {
                count_upper += 1;
            } else if c.is_ascii_digit() {
                count_digit += 1;
            } else if !c.is_alphanumeric() {
                count_special += 1;
            }
        });

        let lower_req = rules.include_lower_case.unwrap_or(0);
        if lower_req > count_lower {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!(
                    "New password does not include the minimum lower character count: {lower_req}"
                ),
            ));
        }

        let upper_req = rules.include_upper_case.unwrap_or(0);
        if upper_req > count_upper {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!(
                    "New password does not include the minimum upper character count: {upper_req}"
                ),
            ));
        }

        let digit_req = rules.include_digits.unwrap_or(0);
        if digit_req > count_digit {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!("New password does not include the minimum digit count: {digit_req}"),
            ));
        }

        let special_req = rules.include_special.unwrap_or(0);
        if special_req > count_special {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!(
                    "New password does not include the minimum special character count: {special_req}"
                ),
            ));
        }

        let new_hash = HashPassword::hash_password(plain_pwd.to_string()).await?;
        let mut new_recent = Vec::new();

        if let Some(recent_req) = rules.not_recently_used {
            match RecentPasswordsEntity::find(&self.id).await {
                Ok(mut most_recent) => {
                    let mut iteration = 1;
                    for old_hash in most_recent.passwords.split('\n') {
                        if ComparePasswords::is_match(plain_pwd.to_string(), old_hash.to_string())
                            .await?
                        {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                format!(
                                    "The new password must not be one of the last {recent_req} used \
                                    passwords"
                                ),
                            ));
                        }

                        new_recent.push(old_hash);

                        // check if we have more recent passwords than needed
                        iteration += 1;
                        if iteration == recent_req {
                            break;
                        }
                    }

                    most_recent.passwords = format!("{new_hash}\n{}", new_recent.join("\n"));
                    most_recent.save().await?;
                }

                Err(_) => {
                    RecentPasswordsEntity::create(&self.id, new_hash.clone()).await?;
                }
            }
        }

        if RauthyConfig::get().vars.webauthn.no_password_exp && self.has_webauthn_enabled() {
            self.password_expires = None;
        } else {
            let password_expires = rules.valid_days.map(|d| {
                OffsetDateTime::now_utc()
                    .add(::time::Duration::days(d as i64))
                    .unix_timestamp()
            });
            self.password_expires = password_expires;
        }

        self.password = Some(new_hash);

        Ok(())
    }

    #[inline]
    pub fn check_enabled(&self) -> Result<(), ErrorResponse> {
        if !self.enabled {
            trace!("The user is not enabled");
            return Err(ErrorResponse::new(
                ErrorResponseType::Disabled,
                "User is not enabled",
            ));
        }
        Ok(())
    }

    #[inline]
    pub fn check_expired(&self) -> Result<(), ErrorResponse> {
        if let Some(ts) = self.user_expires
            && Utc::now().timestamp() > ts
        {
            trace!("User has expired");
            return Err(ErrorResponse::new(
                ErrorResponseType::Disabled,
                "User has expired",
            ));
        }
        Ok(())
    }

    pub async fn confirm_email_address(
        req: HttpRequest,
        user_id: String,
        confirm_id: String,
    ) -> Result<String, ErrorResponse> {
        let mut ml = MagicLink::find(&confirm_id).await?;
        ml.validate(&user_id, &req, false)?;

        let usage = MagicLinkUsage::try_from(&ml.usage)?;
        let new_email = match usage {
            MagicLinkUsage::NewUser(_) | MagicLinkUsage::PasswordReset(_) => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "The Magic Link is not meant to be used to confirm an E-Mail address",
                ));
            }
            MagicLinkUsage::EmailChange(email) => email,
        };

        let mut user = Self::find(user_id).await?;

        // save data
        let old_email = user.email;
        user.email = new_email;
        user.email_verified = true;
        user.save(Some(old_email.clone())).await?;
        ml.invalidate().await?;

        // finally, invalidate all existing sessions with the old email
        Session::invalidate_for_user(&user.id).await?;

        // send out confirmation E-Mails to both addresses
        send_email_confirm_change(&user, &user.email, &user.email, false).await;
        send_email_confirm_change(&user, &old_email, &user.email, false).await;

        let event_text = format!("{old_email} -> {}", user.email);
        let ip = real_ip_from_req(&req).ok();
        RauthyConfig::get()
            .tx_events
            .send_async(Event::user_email_change(event_text, ip))
            .await
            .unwrap();

        // build response HTML
        let lang = Language::try_from(&req).unwrap_or_default();
        let html = UserEmailChangeConfirmHtml::build(
            &lang,
            ThemeCssFull::find_theme_ts_rauthy().await?,
            &[
                HtmlTemplate::EmailOld(old_email),
                HtmlTemplate::EmailNew(user.email),
            ],
        );

        Ok(html)
    }

    pub fn delete_group(&mut self, group: &str) {
        if self.groups.is_none() {
            return;
        }
        let old_groups = self.groups.as_ref().unwrap();

        let i_opt = old_groups.find(group);
        if i_opt.is_none() {
            return;
        }

        let i = i_opt.unwrap();
        if i == 0 {
            // the group is the first entry
            if old_groups.len() > group.len() {
                let g = format!("{group},");
                self.groups = Some(old_groups.replace(&g, ""));
            } else {
                self.groups = None;
            }
        } else {
            // the role is at the end or in the middle
            let g = format!(",{group}");
            self.groups = Some(old_groups.replace(&g, ""));
        }
    }

    pub fn delete_role(&mut self, role: &str) {
        // find the role via index in the string
        // first entry: delete role + ',' if it exists
        // last entry: delete role + ',' in front if it exists
        // middle: delete role + ',' in front if it exists
        // --> 2 cases: first entry or else
        let i_opt = self.roles.find(role);
        if i_opt.is_none() {
            return;
        }

        let i = i_opt.unwrap();
        if i == 0 {
            // the role is the first entry
            if self.roles.len() > role.len() {
                let r = format!("{role},");
                self.roles = self.roles.replace(&r, "");
            } else {
                self.roles = String::from("");
            }
        } else {
            // the role is at the end or in the middle
            let r = format!(",{role}");
            self.roles = self.roles.replace(&r, "");
        }
    }

    pub fn email_recipient_name(&self) -> String {
        if let Some(n) = &self.family_name {
            format!("{} {n}", self.given_name)
        } else {
            self.given_name.to_string()
        }
    }

    pub async fn from_new_user_req(new_user: NewUserRequest) -> Result<Self, ErrorResponse> {
        let roles = Role::sanitize(new_user.roles).await?;
        let groups = Group::sanitize(new_user.groups).await?;

        let user = Self {
            email: new_user.email.to_lowercase(),
            email_verified: false,
            given_name: new_user.given_name.unwrap_or_default(),
            family_name: new_user.family_name,
            language: new_user.language.into(),
            roles,
            groups,
            user_expires: new_user.user_expires,
            ..Default::default()
        };

        Ok(user)
    }

    #[inline]
    pub fn get_groups(&self) -> Vec<String> {
        let mut res = Vec::new();
        if self.groups.is_some() {
            self.groups
                .as_ref()
                .unwrap()
                .split(',')
                .for_each(|g| res.push(g.trim().to_owned()));
        }
        res
    }

    #[inline]
    pub fn groups_iter(&self) -> Split<'_, char> {
        self.groups.as_deref().unwrap_or_default().split(',')
    }

    #[inline]
    pub fn get_roles(&self) -> Vec<String> {
        let mut res = Vec::new();
        if !self.roles.is_empty() {
            self.roles
                .split(',')
                .for_each(|r| res.push(r.trim().to_owned()));
        }
        res
    }

    #[inline]
    pub fn roles_iter(&self) -> Split<'_, char> {
        self.roles.as_str().split(',')
    }

    #[inline(always)]
    pub fn has_webauthn_enabled(&self) -> bool {
        self.webauthn_user_id.is_some()
    }

    pub fn into_response(self, user_values: Option<UserValues>) -> UserResponse {
        let roles = self.get_roles();
        let groups = if self.groups.is_some() {
            Some(self.get_groups())
        } else {
            None
        };
        let account_type = UserAccountTypeResponse::from(self.account_type());

        UserResponse {
            id: self.id,
            email: self.email,
            given_name: if self.given_name.is_empty() {
                None
            } else {
                Some(self.given_name)
            },
            family_name: self.family_name,
            language: self.language.into(),
            roles,
            groups,
            enabled: self.enabled,
            email_verified: self.email_verified,
            password_expires: self.password_expires,
            created_at: self.created_at,
            last_login: self.last_login,
            last_failed_login: self.last_failed_login,
            failed_login_attempts: self.failed_login_attempts,
            user_expires: self.user_expires,
            account_type,
            webauthn_user_id: self.webauthn_user_id,
            user_values: user_values
                .map(UserValuesResponse::from)
                .unwrap_or_default(),
            auth_provider_id: self.auth_provider_id,
            federation_uid: self.federation_uid,
            picture_id: self.picture_id,
        }
    }

    pub fn is_argon2_uptodate(&self, params: &argon2::Params) -> Result<bool, ErrorResponse> {
        if self.password.is_none() {
            error!(
                user_id = self.id,
                "Trying to validate argon2 params with not set password"
            );
            return Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                "Cannot validate argon2 param - password is not set",
            ));
        }
        let hash = PasswordHash::new(self.password.as_ref().unwrap())
            .expect("Could not build Hash from password string");
        let curr_params =
            argon2::Params::try_from(&hash).expect("Could not extract params from hash");

        if curr_params.m_cost() == params.m_cost()
            && curr_params.t_cost() == params.t_cost()
            && curr_params.p_cost() == params.p_cost()
        {
            return Ok(true);
        }
        Ok(false)
    }

    #[inline]
    pub fn is_admin(&self) -> bool {
        self.roles_iter().any(|r| r == RAUTHY_ADMIN_ROLE)
    }

    async fn is_email_free(email: String) -> Result<(), ErrorResponse> {
        match User::find_by_email(email).await {
            Ok(_) => Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "E-Mail is already in use",
            )),
            Err(_) => Ok(()),
        }
    }

    /// Returns `true` if the passwords match and `false` if they don't.
    /// It only returns an Err(ErrorResponse) in case of a hash parsing issue or corrupted data.
    async fn match_passwords(&self, plain: String) -> Result<bool, ErrorResponse> {
        if self.password.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "The password to validate is not set yet",
            ));
        }
        let hash = self.password.as_ref().unwrap().to_owned();
        ComparePasswords::is_match(plain, hash).await
    }

    /// Checks if this user needs to accept a updated ToS
    #[inline(always)]
    pub async fn needs_tos_update(&self) -> Result<bool, ErrorResponse> {
        let needs_update = if let Some(latest) = ToS::find_latest().await.expect("TosLatest") {
            if let Some(accepted) = ToSUserAccept::find_latest(self.id.clone())
                .await
                .expect("TosUserAccept")
            {
                accepted.tos_ts != latest.ts
            } else {
                true
            }
        } else {
            false
        };

        debug!(needs_update, "User needs to accept updated ToS");
        Ok(needs_update)
    }

    pub fn picture_uri(&self) -> Option<String> {
        self.picture_id.as_ref().map(|pic_id| {
            format!(
                "{}/auth/v1/users/{}/picture/{pic_id}",
                RauthyConfig::get().pub_url_with_scheme,
                self.id,
            )
        })
    }

    pub fn push_group(&mut self, group: &str) {
        if self.groups.is_some() {
            let g = self.groups.as_ref().unwrap();
            self.groups = Some(format!("{g},{group}"));
        } else {
            self.groups = Some(group.to_owned());
        }
    }

    pub fn push_role(&mut self, role: &str) {
        if !self.roles.is_empty() {
            self.roles = format!("{},{role}", self.roles);
        } else {
            role.clone_into(&mut self.roles);
        }
    }

    pub async fn request_password_reset(
        &self,
        redirect_uri: Option<String>,
    ) -> Result<(), ErrorResponse> {
        // deny for passkey only accounts
        if self.account_type() == AccountType::Passkey {
            return Ok(());
        }

        // if any active magic links already exist - delete them and only ever have 1 active.
        MagicLink::delete_all_pwd_reset_for_user(self.id.clone()).await?;

        let usage = if self.password.is_none() && !self.has_webauthn_enabled() {
            MagicLinkUsage::NewUser(redirect_uri)
        } else {
            MagicLinkUsage::PasswordReset(redirect_uri)
        };
        let new_ml = MagicLink::create(
            self.id.clone(),
            RauthyConfig::get().vars.lifetimes.magic_link_pwd_reset as i64,
            usage,
        )
        .await?;

        let values = UserValues::find(&self.id).await?;
        let tz = values.as_ref().and_then(|uv| uv.tz.as_deref());

        send_pwd_reset(&new_ml, self, tz).await;

        Ok(())
    }

    pub async fn validate_password(&self, plain_password: String) -> Result<(), ErrorResponse> {
        if self.password.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::PasswordExpired,
                "No password set",
            ));
        }

        if let Some(exp) = self.password_expires
            && exp < Utc::now().timestamp()
        {
            if !self.enabled {
                return Err(ErrorResponse::new(
                    ErrorResponseType::PasswordExpired,
                    "The password has expired",
                ));
            }

            // if the given password does match, send out a reset link to set a new one
            return if self.match_passwords(plain_password.clone()).await? {
                let magic_link = MagicLink::create(
                    self.id.clone(),
                    RauthyConfig::get().vars.lifetimes.magic_link_pwd_reset as i64,
                    MagicLinkUsage::PasswordReset(None),
                )
                .await?;

                let values = UserValues::find(&self.id).await?;
                let tz = values.as_ref().and_then(|uv| uv.tz.as_deref());

                send_pwd_reset(&magic_link, self, tz).await;

                Err(ErrorResponse::new(
                    ErrorResponseType::PasswordRefresh,
                    "The password has expired. A reset E-Mail was sent.",
                ))
            } else {
                Err(ErrorResponse::new(
                    ErrorResponseType::Unauthorized,
                    "Invalid user credentials",
                ))
            };
        }

        if self.match_passwords(plain_password).await? {
            Ok(())
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Invalid user credentials",
            ))
        }
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: new_store_id(),
            email: String::default(),
            given_name: String::default(),
            family_name: None,
            password: None,
            roles: String::default(),
            groups: None,
            enabled: true,
            email_verified: false,
            password_expires: None,
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            last_login: None,
            last_failed_login: None,
            failed_login_attempts: None,
            language: Language::En,
            webauthn_user_id: None,
            user_expires: None,
            auth_provider_id: None,
            federation_uid: None,
            picture_id: None,
        }
    }
}

impl From<User> for UserResponseSimple {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            email: u.email,
            given_name: if u.given_name.is_empty() {
                None
            } else {
                Some(u.given_name)
            },
            family_name: u.family_name,
            created_at: u.created_at,
            last_login: u.last_login,
            picture_id: u.picture_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::sessions::{Session, SessionState};
    use pretty_assertions::assert_eq;
    use std::ops::Sub;

    #[test]
    fn test_session_impl() {
        let mut user = User {
            id: "123".to_string(),
            email: "admin@localhost".to_string(),
            given_name: "Admin".to_string(),
            family_name: Some("Rauthy".to_string()),
            password: Some("SoSafeNOTHash".to_string()),
            roles: "rauthy_admin,admin".to_string(),
            groups: Some("admin,user".to_string()),
            enabled: true,
            email_verified: true,
            password_expires: Some(OffsetDateTime::now_utc().unix_timestamp()),
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            last_login: None,
            last_failed_login: None,
            failed_login_attempts: None,
            language: Language::En,
            webauthn_user_id: None,
            user_expires: Some(
                OffsetDateTime::now_utc()
                    .sub(::time::Duration::seconds(2))
                    .unix_timestamp(),
            ),
            auth_provider_id: None,
            federation_uid: None,
            picture_id: None,
        };
        let session = Session::try_new(&user, 1, None);
        assert!(session.is_err());

        user.user_expires = None;
        let session = Session::try_new(&user, 1, None).unwrap();

        assert_eq!(session.is_valid(10, None, "/auth/v1/oidc/authorize"), true);
        // sessions are validated with second accuracy
        std::thread::sleep(core::time::Duration::from_secs(2));
        assert_eq!(session.is_valid(10, None, "/auth/v1/oidc/authorize"), false);

        // new sessions should always be in state 1 -> initializing
        assert_eq!(session.state, SessionState::Init);

        assert!(session.csrf_token.len() > 0);

        assert_eq!(session.groups_as_vec(), Ok(vec!["admin", "user"]));
        assert_eq!(
            session.roles_as_vec(),
            Ok(vec!["rauthy_admin".to_string(), "admin".to_string()])
        );
    }

    fn check_password_expired(user: &User) -> Result<(), ErrorResponse> {
        if let Some(exp) = user.password_expires {
            if exp < OffsetDateTime::now_utc().unix_timestamp() {
                return Err(ErrorResponse::new(
                    ErrorResponseType::PasswordExpired,
                    String::from("The password has expired"),
                ));
            }
        }
        Ok(())
    }

    #[test]
    fn test_user_impl() -> Result<(), ErrorResponse> {
        let mut user = User {
            id: "123".to_string(),
            email: "admin@localhost".to_string(),
            given_name: "Admin".to_string(),
            family_name: Some("Rauthy".to_string()),
            password: Some("$argon2id$v=19$m=16384,t=3,p=2$l8F0ar1wSQsce+OdPgYbhg$I2XrvC/XRW+22eI2ptBg5GQp3SHjgSQXsfstuTZne1I".to_string()),
            roles: "rauthy_admin,admin".to_string(),
            groups: Some("admin,user".to_string()),
            enabled: false,
            email_verified: true,
            password_expires: None,
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            last_login: None,
            last_failed_login: None,
            failed_login_attempts: None,
            language: Language::En,
            webauthn_user_id: None,
            user_expires: None,
            auth_provider_id: None,
            federation_uid: None,
            picture_id: None,
        };

        // enabled
        assert!(user.check_enabled().is_err());
        user.enabled = true;
        assert!(user.check_enabled().is_ok());

        // password expiry
        assert!(check_password_expired(&user).is_ok());
        user.password_expires = Some(OffsetDateTime::now_utc().unix_timestamp() + 1);
        assert!(check_password_expired(&user).is_ok());
        // password expiry is validated with second accuracy
        std::thread::sleep(core::time::Duration::from_secs(2));
        assert!(check_password_expired(&user).is_err());

        // groups
        assert_eq!(
            user.get_groups(),
            vec!["admin".to_string(), "user".to_string()]
        );
        user.push_group("test123");
        user.push_group("end");
        assert_eq!(
            user.get_groups(),
            vec![
                "admin".to_string(),
                "user".to_string(),
                "test123".to_string(),
                "end".to_string(),
            ]
        );
        user.delete_group("user");
        assert_eq!(
            user.get_groups(),
            vec![
                "admin".to_string(),
                "test123".to_string(),
                "end".to_string(),
            ]
        );
        assert_eq!(user.groups, Some("admin,test123,end".to_string()));
        user.delete_group("end");
        assert_eq!(user.groups, Some("admin,test123".to_string()));
        user.delete_group("admin");
        assert_eq!(user.groups, Some("test123".to_string()));

        // roles
        assert_eq!(
            user.get_roles(),
            vec!["rauthy_admin".to_string(), "admin".to_string()]
        );
        user.push_role("super_admin");
        user.push_role("end");
        assert_eq!(
            user.get_roles(),
            vec![
                "rauthy_admin".to_string(),
                "admin".to_string(),
                "super_admin".to_string(),
                "end".to_string(),
            ]
        );
        user.delete_role("admin");
        assert_eq!(
            user.get_roles(),
            vec![
                "rauthy_admin".to_string(),
                "super_admin".to_string(),
                "end".to_string(),
            ]
        );
        assert_eq!(user.roles, "rauthy_admin,super_admin,end");
        user.delete_role("end");
        assert_eq!(user.roles, "rauthy_admin,super_admin");
        user.delete_role("rauthy_admin");
        assert_eq!(user.roles, "super_admin");

        // argon2 params
        // defaults: argon2_m_cost = 16384, argon2_t_cost = 3, argon2_p_cost = 2
        let mut wrapped_params = argon2::Params::new(16384, 3, 2, None)?;
        let res = user.is_argon2_uptodate(&wrapped_params)?;
        assert_eq!(res, true);

        wrapped_params = argon2::Params::new(8192, 3, 2, None)?;
        let res = user.is_argon2_uptodate(&wrapped_params)?;
        assert_eq!(res, false);

        wrapped_params = argon2::Params::new(16384, 4, 2, None)?;
        let res = user.is_argon2_uptodate(&wrapped_params)?;
        assert_eq!(res, false);

        wrapped_params = argon2::Params::new(16384, 3, 5, None)?;
        let res = user.is_argon2_uptodate(&wrapped_params)?;
        assert_eq!(res, false);

        Ok(())
    }
}
