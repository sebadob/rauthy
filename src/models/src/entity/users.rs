use crate::app_state::{AppState, Argon2Params, DbTxn};
use crate::cache::{Cache, DB};
use crate::email::{send_email_change_info_new, send_email_confirm_change, send_pwd_reset};
use crate::entity::colors::ColorEntity;
use crate::entity::continuation_token::ContinuationToken;
use crate::entity::groups::Group;
use crate::entity::magic_links::{MagicLink, MagicLinkUsage};
use crate::entity::password::PasswordPolicy;
use crate::entity::password::RecentPasswordsEntity;
use crate::entity::refresh_tokens::RefreshToken;
use crate::entity::roles::Role;
use crate::entity::sessions::Session;
use crate::entity::users_values::UserValues;
use crate::entity::webauthn::{PasskeyEntity, WebauthnServiceReq};
use crate::events::event::Event;
use crate::language::Language;
use crate::templates::UserEmailChangeConfirmHtml;
use actix_web::{web, HttpRequest};
use argon2::PasswordHash;
use chrono::Utc;
use rauthy_api_types::generic::SearchParamsIdx;
use rauthy_api_types::users::{
    NewUserRegistrationRequest, NewUserRequest, UpdateUserRequest, UpdateUserSelfRequest,
    UserAccountTypeResponse, UserResponse, UserResponseSimple, UserValuesResponse,
};
use rauthy_common::constants::{
    CACHE_TTL_APP, CACHE_TTL_USER, IDX_USERS, IDX_USER_COUNT, RAUTHY_ADMIN_ROLE,
    WEBAUTHN_NO_PASSWORD_EXPIRY,
};
use rauthy_common::password_hasher::{ComparePasswords, HashPassword};
use rauthy_common::utils::{new_store_id, real_ip_from_req};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, FromRow};
use std::ops::Add;
use time::OffsetDateTime;
use tracing::{debug, error, trace, warn};

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

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
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
}

// CRUD
impl User {
    pub async fn count(data: &web::Data<AppState>) -> Result<i64, ErrorResponse> {
        let client = DB::client();

        if let Some(count) = client.get(Cache::App, IDX_USER_COUNT).await? {
            return Ok(count);
        }

        let res = sqlx::query!("SELECT COUNT (*) count FROM users")
            .fetch_one(&data.db)
            .await?;

        // sqlite returns an i32 for count while postgres returns an Option<i64>
        #[cfg(feature = "postgres")]
        let count = res.count.unwrap_or_default();
        #[cfg(not(feature = "postgres"))]
        let count = res.count as i64;

        client
            .put(Cache::App, IDX_USER_COUNT, &count, CACHE_TTL_APP)
            .await?;

        Ok(count)
    }

    async fn count_inc(data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        let mut count = Self::count(data).await?;
        // theoretically, we could have overlaps here, but we don't really care
        // -> used for dynamic pagination only and SQLite has limited query features
        count += 1;
        DB::client()
            .put(Cache::App, IDX_USER_COUNT, &count, CACHE_TTL_APP)
            .await?;
        Ok(())
    }

    async fn count_dec(data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        let mut count = Self::count(data).await?;
        // theoretically, we could have overlaps here, but we don't really care
        // -> used for dynamic pagination only and SQLite has limited query features
        count -= 1;
        DB::client()
            .put(Cache::App, IDX_USER_COUNT, &count, CACHE_TTL_APP)
            .await?;
        Ok(())
    }

    // Inserts a user into the database
    pub async fn create(
        data: &web::Data<AppState>,
        new_user: User,
        post_reset_redirect_uri: Option<String>,
    ) -> Result<Self, ErrorResponse> {
        let slf = Self::insert(data, new_user).await?;

        let magic_link = MagicLink::create(
            data,
            slf.id.clone(),
            data.ml_lt_pwd_first as i64,
            MagicLinkUsage::NewUser(post_reset_redirect_uri),
        )
        .await?;
        send_pwd_reset(data, &magic_link, &slf).await;

        Ok(slf)
    }

    pub async fn create_federated(
        data: &web::Data<AppState>,
        new_user: User,
    ) -> Result<Self, ErrorResponse> {
        Self::insert(data, new_user).await
    }

    // Inserts a user into the database
    pub async fn create_from_new(
        data: &web::Data<AppState>,
        new_user_req: NewUserRequest,
    ) -> Result<User, ErrorResponse> {
        let new_user = User::from_new_user_req(data, new_user_req).await?;
        User::create(data, new_user, None).await
    }

    // Inserts a user from the open registration endpoint into the database
    pub async fn create_from_reg(
        data: &web::Data<AppState>,
        req_data: NewUserRegistrationRequest,
        lang: Language,
    ) -> Result<User, ErrorResponse> {
        let mut new_user = Self {
            email: req_data.email.to_lowercase(),
            given_name: req_data.given_name,
            family_name: req_data.family_name,
            ..Default::default()
        };
        new_user.language = lang;
        let new_user = User::create(data, new_user, req_data.redirect_uri).await?;

        Ok(new_user)
    }

    // Deletes a user
    pub async fn delete(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        // Clean up all possibly existing sessions from the cache
        Session::delete_by_user(data, &self.id).await?;

        // Delete the user itself
        sqlx::query!("DELETE FROM users WHERE id = $1", self.id)
            .execute(&data.db)
            .await?;

        let client = DB::client();

        let idx = format!("{}_{}", IDX_USERS, &self.id);
        client.delete(Cache::User, idx).await?;

        let idx = format!("{}_{}", IDX_USERS, &self.email);
        client.delete(Cache::User, idx).await?;

        Self::count_dec(data).await?;

        Ok(())
    }

    // Checks if a user exists in the database without fetching data
    pub async fn exists(data: &web::Data<AppState>, id: String) -> Result<(), ErrorResponse> {
        let idx = format!("{}_{}", IDX_USERS, id);

        let opt: Option<Self> = DB::client().get(Cache::User, idx).await?;
        if opt.is_some() {
            return Ok(());
        }

        sqlx::query!("SELECT id FROM users WHERE id = $1", id)
            .fetch_one(&data.db)
            .await?;

        Ok(())
    }

    pub async fn find(data: &web::Data<AppState>, id: String) -> Result<Self, ErrorResponse> {
        let idx = format!("{}_{}", IDX_USERS, id);
        let client = DB::client();

        if let Some(slf) = client.get(Cache::User, &idx).await? {
            return Ok(slf);
        }

        let user = sqlx::query_as!(Self, "SELECT * FROM users WHERE id = $1", id)
            .fetch_one(&data.db)
            .await?;

        client.put(Cache::User, idx, &user, CACHE_TTL_USER).await?;
        Ok(user)
    }

    pub async fn find_by_email(
        data: &web::Data<AppState>,
        email: String,
    ) -> Result<User, ErrorResponse> {
        let email = email.to_lowercase();

        let idx = format!("{}_{}", IDX_USERS, email);
        let client = DB::client();

        if let Some(slf) = client.get(Cache::User, &idx).await? {
            return Ok(slf);
        }

        let user = sqlx::query_as!(Self, "SELECT * FROM users WHERE email = $1", email)
            .fetch_one(&data.db)
            .await?;

        client.put(Cache::User, idx, &user, CACHE_TTL_USER).await?;
        Ok(user)
    }

    pub async fn find_by_federation(
        data: &web::Data<AppState>,
        auth_provider_id: &str,
        federation_uid: &str,
    ) -> Result<Self, ErrorResponse> {
        let user = sqlx::query_as!(
            Self,
            "SELECT * FROM users WHERE auth_provider_id = $1 AND federation_uid = $2",
            auth_provider_id,
            federation_uid
        )
        .fetch_one(&data.db)
        .await?;
        Ok(user)
    }

    pub async fn find_all(data: &web::Data<AppState>) -> Result<Vec<Self>, ErrorResponse> {
        let res = sqlx::query_as!(Self, "SELECT * FROM users ORDER BY created_at ASC")
            .fetch_all(&data.db)
            .await?;
        Ok(res)
    }

    pub async fn find_all_simple(
        data: &web::Data<AppState>,
    ) -> Result<Vec<UserResponseSimple>, ErrorResponse> {
        let res = sqlx::query_as!(
            UserResponseSimple,
            "SELECT id, email, created_at, last_login FROM users ORDER BY created_at ASC"
        )
        .fetch_all(&data.db)
        .await?;
        Ok(res)
    }

    pub async fn find_expired(data: &web::Data<AppState>) -> Result<Vec<Self>, ErrorResponse> {
        let now = OffsetDateTime::now_utc()
            .add(time::Duration::seconds(10))
            .unix_timestamp();
        let res = sqlx::query_as!(Self, "SELECT * FROM users WHERE user_expires < $1", now)
            .fetch_all(&data.db)
            .await?;
        Ok(res)
    }

    pub async fn find_for_fed_cm_validated(
        data: &web::Data<AppState>,
        user_id: String,
    ) -> Result<Self, ErrorResponse> {
        // We will stick to the WWW-Authenticate header for now and use duplicated code from
        // some OAuth2 api for now until the spec has settled on an error behavior.
        debug!("Looking up FedCM user_id {}", user_id);
        let slf = Self::find(data, user_id).await.map_err(|_| {
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

    pub async fn find_paginated(
        data: &web::Data<AppState>,
        continuation_token: Option<ContinuationToken>,
        page_size: i64,
        mut offset: i64,
        backwards: bool,
    ) -> Result<(Vec<UserResponseSimple>, Option<ContinuationToken>), ErrorResponse> {
        let mut res = Vec::with_capacity(page_size as usize);
        let mut latest_ts = 0;

        if let Some(token) = continuation_token {
            if backwards {
                offset += page_size;
                let mut rows = sqlx::query!(
                    r#"SELECT id, email, created_at, last_login
                    FROM users
                    WHERE created_at <= $1 AND id != $2
                    ORDER BY created_at DESC
                    LIMIT $3
                    OFFSET $4"#,
                    token.ts,
                    token.id,
                    page_size,
                    offset,
                )
                .fetch_all(&data.db)
                .await?;

                rows.reverse();

                for row in rows {
                    res.push(UserResponseSimple {
                        id: row.id,
                        email: row.email,
                        created_at: row.created_at,
                        last_login: row.last_login,
                    });
                    latest_ts = row.created_at;
                }
            } else {
                let rows = sqlx::query!(
                    r#"SELECT id, email, created_at, last_login
                    FROM users
                    WHERE created_at >= $1 AND id != $2
                    ORDER BY created_at ASC
                    LIMIT $3
                    OFFSET $4"#,
                    token.ts,
                    token.id,
                    page_size,
                    offset,
                )
                .fetch_all(&data.db)
                .await?;

                for row in rows {
                    res.push(UserResponseSimple {
                        id: row.id,
                        email: row.email,
                        created_at: row.created_at,
                        last_login: row.last_login,
                    });
                    latest_ts = row.created_at;
                }
            };
        } else if backwards {
            // backwards without any continuation token will simply
            // serve the last elements without any other conditions
            let mut rows = sqlx::query!(
                r#"SELECT id, email, created_at, last_login
                   FROM users
                   ORDER BY created_at DESC
                   LIMIT $1
                   OFFSET $2"#,
                page_size,
                offset,
            )
            .fetch_all(&data.db)
            .await?;

            rows.reverse();

            for row in rows {
                res.push(UserResponseSimple {
                    id: row.id,
                    email: row.email,
                    created_at: row.created_at,
                    last_login: row.last_login,
                });
                latest_ts = row.created_at;
            }
        } else {
            let rows = sqlx::query!(
                r#"SELECT id, email, created_at, last_login
                   FROM users
                   ORDER BY created_at ASC
                   LIMIT $1
                   OFFSET $2"#,
                page_size,
                offset,
            )
            .fetch_all(&data.db)
            .await?;

            for row in rows {
                res.push(UserResponseSimple {
                    id: row.id,
                    email: row.email,
                    created_at: row.created_at,
                    last_login: row.last_login,
                });
                latest_ts = row.created_at;
            }
        };

        let token = res
            .last()
            .map(|entry| ContinuationToken::new(entry.id.clone(), latest_ts));

        Ok((res, token))
    }

    pub async fn insert(data: &web::Data<AppState>, new_user: User) -> Result<Self, ErrorResponse> {
        let lang = new_user.language.as_str();
        sqlx::query!(
            r#"INSERT INTO USERS
            (id, email, given_name, family_name, roles, groups, enabled, email_verified, created_at,
            last_login, language, user_expires, auth_provider_id, federation_uid)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)"#,
            new_user.id,
            new_user.email,
            new_user.given_name,
            new_user.family_name,
            new_user.roles,
            new_user.groups,
            new_user.enabled,
            new_user.email_verified,
            new_user.created_at,
            new_user.last_login,
            lang,
            new_user.user_expires,
            new_user.auth_provider_id,
            new_user.federation_uid,
        )
        .execute(&data.db)
        .await?;

        Self::count_inc(data).await?;

        Ok(new_user)
    }

    pub async fn provider_unlink(
        data: &web::Data<AppState>,
        user_id: String,
    ) -> Result<Self, ErrorResponse> {
        // we need to find the user first and validate that it has been set up properly
        // to work without a provider
        let mut slf = Self::find(data, user_id).await?;
        if slf.password.is_none() && !slf.has_webauthn_enabled() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "You must have at least a password or passkey set up before you can remove a provider link",
            ));
        }

        slf.auth_provider_id = None;
        slf.federation_uid = None;
        slf.save(data, None, None).await?;

        Ok(slf)
    }

    pub async fn save(
        &self,
        data: &web::Data<AppState>,
        old_email: Option<String>,
        txn: Option<&mut DbTxn<'_>>,
    ) -> Result<(), ErrorResponse> {
        if old_email.is_some() {
            User::is_email_free(data, self.email.clone()).await?;
        }

        let lang = self.language.as_str();
        let q = sqlx::query(
            r#"UPDATE USERS SET
            email = $1, given_name = $2, family_name = $3, password = $4, roles = $5, groups = $6,
            enabled = $7, email_verified = $8, password_expires = $9, last_login = $10,
            last_failed_login = $11, failed_login_attempts = $12, language = $13,
            webauthn_user_id = $14, user_expires = $15, auth_provider_id = $16, federation_uid = $17
            WHERE id = $18"#,
        )
        .bind(&self.email)
        .bind(&self.given_name)
        .bind(&self.family_name)
        .bind(&self.password)
        .bind(&self.roles)
        .bind(&self.groups)
        .bind(self.enabled)
        .bind(self.email_verified)
        .bind(self.password_expires)
        .bind(self.last_login)
        .bind(self.last_failed_login)
        .bind(self.failed_login_attempts)
        .bind(lang)
        .bind(self.webauthn_user_id.clone())
        .bind(self.user_expires)
        .bind(&self.auth_provider_id)
        .bind(&self.federation_uid)
        .bind(&self.id);

        if let Some(txn) = txn {
            q.execute(&mut **txn).await?;
        } else {
            q.execute(&data.db).await?;
        }

        // invalidate all possibly existing sessions and refresh tokens, if the user has been disabled
        if !self.enabled {
            Session::invalidate_for_user(data, &self.id).await?;
            RefreshToken::invalidate_for_user(data, &self.id).await?;
        }

        let client = DB::client();

        if let Some(email) = old_email {
            let idx = format!("{}_{}", IDX_USERS, email);
            client.delete(Cache::User, idx).await?;
        }

        let idx = format!("{}_{}", IDX_USERS, &self.id);
        client.put(Cache::User, idx, self, CACHE_TTL_USER).await?;

        let idx = format!("{}_{}", IDX_USERS, &self.email);
        client.put(Cache::User, idx, self, CACHE_TTL_USER).await?;

        Ok(())
    }

    /// Caution: Uses regex / LIKE on the database -> very costly query
    pub async fn search(
        data: &web::Data<AppState>,
        idx: &SearchParamsIdx,
        q: &str,
        limit: i64,
    ) -> Result<Vec<UserResponseSimple>, ErrorResponse> {
        let q = format!("%{}%", q);

        let res = match idx {
            SearchParamsIdx::Id | SearchParamsIdx::UserId => {
                query_as!(
                    UserResponseSimple,
                    r#"SELECT id, email, created_at, last_login
                    FROM users
                    WHERE id LIKE $1
                    ORDER BY created_at ASC
                    LIMIT $2"#,
                    q,
                    limit
                )
                .fetch_all(&data.db)
                .await?
            }
            SearchParamsIdx::Email => {
                query_as!(
                    UserResponseSimple,
                    r#"SELECT id, email, created_at, last_login
                    FROM users
                    WHERE email LIKE $1
                    ORDER BY created_at ASC
                    LIMIT $2"#,
                    q,
                    limit
                )
                .fetch_all(&data.db)
                .await?
            }
            _ => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "supported search idx for users: id / user_id, email",
                ))
            }
        };

        Ok(res)
    }

    pub async fn update(
        data: &web::Data<AppState>,
        id: String,
        mut upd_user: UpdateUserRequest,
        user: Option<User>,
    ) -> Result<(User, Option<UserValues>, bool), ErrorResponse> {
        let mut user = match user {
            None => User::find(data, id).await?,
            Some(user) => user,
        };
        upd_user.email = upd_user.email.to_lowercase();
        let old_email = if user.email != upd_user.email {
            Some(user.email.clone())
        } else {
            None
        };

        user.email = upd_user.email;
        user.given_name = upd_user.given_name;
        user.family_name = upd_user.family_name;

        if let Some(lang) = upd_user.language {
            user.language = lang.into();
        }

        if let Some(password) = &upd_user.password {
            user.apply_password_rules(data, password).await?;
        }

        let is_admin_before_update = user.is_admin();
        user.roles = Role::sanitize(data, upd_user.roles).await?;
        user.groups = Group::sanitize(data, upd_user.groups).await?;

        user.enabled = upd_user.enabled;
        user.email_verified = upd_user.email_verified;
        user.user_expires = upd_user.user_expires;

        user.save(data, old_email.clone(), None).await?;

        if upd_user.password.is_some() {
            data.tx_events
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
            Session::invalidate_for_user(data, &user.id).await?;

            // send out confirmation E-Mails to both addresses
            send_email_confirm_change(data, &user, &user.email, &user.email, true).await;
            send_email_confirm_change(data, &user, old_email, &user.email, true).await;

            let event_text = format!("Change by admin: {} -> {}", old_email, user.email);
            data.tx_events
                .send_async(Event::user_email_change(event_text, None))
                .await
                .unwrap();
        }

        let is_new_admin = !is_admin_before_update && user.is_admin();

        // finally, update the custom users values
        let user_values = if let Some(values) = upd_user.user_values {
            UserValues::upsert(data, user.id.clone(), values).await?
        } else {
            None
        };

        Ok((user, user_values, is_new_admin))
    }

    pub async fn update_language(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        let lang = self.language.as_str();
        sqlx::query(r#"update users set language = $1 where id = $2"#)
            .bind(lang)
            .bind(&self.id)
            .execute(&data.db)
            .await?;

        Ok(())
    }

    // Updates a user from himself. This is needed for the account page to make each user able to
    // update its own data.
    pub async fn update_self_req(
        data: &web::Data<AppState>,
        id: String,
        upd_user: UpdateUserSelfRequest,
    ) -> Result<(User, Option<UserValues>, bool), ErrorResponse> {
        let user = User::find(data, id.clone()).await?;

        let mut password = None;
        if let Some(pwd_new) = upd_user.password_new {
            if let Some(pwd_curr) = upd_user.password_current {
                user.validate_password(data, pwd_curr).await?;
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
                MagicLink::invalidate_all_email_change(data, &user.id).await?;

                let ml = MagicLink::create(
                    data,
                    user.id.clone(),
                    60,
                    MagicLinkUsage::EmailChange(email.clone()),
                )
                .await?;
                send_email_change_info_new(data, &ml, &user, email).await;
                true
            } else {
                false
            }
        } else {
            false
        };

        let given_name = if let Some(given_name) = upd_user.given_name {
            given_name
        } else {
            user.given_name.clone()
        };
        let family_name = if let Some(family_name) = upd_user.family_name {
            family_name
        } else {
            user.family_name.clone()
        };
        let groups = if user.groups.is_some() {
            Some(user.get_groups())
        } else {
            None
        };
        let req = UpdateUserRequest {
            // never update the email directly here, only via email confirmation action from the user
            email: user.email.clone(),
            given_name,
            family_name,
            language: upd_user.language,
            password,
            roles: user.get_roles(),
            groups,
            enabled: user.enabled,
            email_verified: user.email_verified,
            user_expires: user.user_expires,
            user_values: upd_user.user_values,
        };

        // a user cannot become a new admin from a self-req
        let (user, user_values, _is_new_admin) = User::update(data, id, req, Some(user)).await?;
        Ok((user, user_values, email_updated))
    }

    /// Converts a user account from as password account type to passkey only with all necessary
    /// checks included.
    pub async fn convert_to_passkey(
        data: &web::Data<AppState>,
        id: String,
    ) -> Result<(), ErrorResponse> {
        let mut user = User::find(data, id.clone()).await?;

        // only allow conversion for password type accounts
        if user.account_type() != AccountType::Password {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Only AccountType::Password can be converted",
            ));
        }

        // check webauthn enabled
        if !user.has_webauthn_enabled() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Account type conversion can only happen with at least one active Passkey",
            ));
        }

        // only allow passkeys with active UV
        let pks = PasskeyEntity::find_for_user_with_uv(data, &user.id).await?;
        if pks.is_empty() {
            return Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "Could not find any passkeys with active User Verification".to_string(),
            ));
        }

        // all good -> delete password
        user.password = None;
        user.password_expires = None;

        user.save(data, None, None).await?;
        Ok(())
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

    pub async fn apply_password_rules(
        &mut self,
        data: &web::Data<AppState>,
        plain_pwd: &str,
    ) -> Result<(), ErrorResponse> {
        let rules = PasswordPolicy::find(data).await?;

        // check length
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

        // check min character counts
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
                    "New password does not include the minimum lower character count: {}",
                    lower_req
                ),
            ));
        }

        let upper_req = rules.include_upper_case.unwrap_or(0);
        if upper_req > count_upper {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!(
                    "New password does not include the minimum upper character count: {}",
                    upper_req
                ),
            ));
        }

        let digit_req = rules.include_digits.unwrap_or(0);
        if digit_req > count_digit {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!(
                    "New password does not include the minimum digit count: {}",
                    digit_req
                ),
            ));
        }

        let special_req = rules.include_special.unwrap_or(0);
        if special_req > count_special {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!(
                    "New password does not include the minimum special character count: {}",
                    special_req
                ),
            ));
        }

        let new_hash = HashPassword::hash_password(plain_pwd.to_string()).await?;
        let mut new_recent = Vec::new();

        // check recently used passwords
        if let Some(recent_req) = rules.not_recently_used {
            match RecentPasswordsEntity::find(data, &self.id).await {
                Ok(mut most_recent) => {
                    let mut iteration = 1;
                    for old_hash in most_recent.passwords.split('\n') {
                        if ComparePasswords::is_match(plain_pwd.to_string(), old_hash.to_string())
                            .await?
                        {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                format!(
                                    "The new password must not be one of the last {} used passwords",
                                    recent_req,
                                ),
                            ));
                        }

                        // build up the new most recent passwords value
                        new_recent.push(old_hash);

                        // check if we have more recent passwords than needed
                        iteration += 1;
                        if iteration == recent_req {
                            break;
                        }
                    }

                    // all good - update most recent passwords
                    most_recent.passwords = format!("{}\n{}", new_hash, new_recent.join("\n"));
                    most_recent.save(data).await?;
                }

                Err(_) => {
                    RecentPasswordsEntity::create(data, &self.id, &new_hash).await?;
                }
            }
        }

        if *WEBAUTHN_NO_PASSWORD_EXPIRY && self.has_webauthn_enabled() {
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
        if let Some(ts) = self.user_expires {
            if Utc::now().timestamp() > ts {
                trace!("User has expired");
                return Err(ErrorResponse::new(
                    ErrorResponseType::Disabled,
                    "User has expired",
                ));
            }
        }
        Ok(())
    }

    pub async fn confirm_email_address(
        data: &web::Data<AppState>,
        req: HttpRequest,
        user_id: String,
        confirm_id: String,
    ) -> Result<String, ErrorResponse> {
        let mut ml = MagicLink::find(data, &confirm_id).await?;
        ml.validate(&user_id, &req, false)?;

        let usage = MagicLinkUsage::try_from(&ml.usage)?;
        let new_email = match usage {
            MagicLinkUsage::NewUser(_) | MagicLinkUsage::PasswordReset(_) => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "The Magic Link is not meant to be used to confirm an E-Mail address"
                        .to_string(),
                ));
            }
            MagicLinkUsage::EmailChange(email) => email,
        };

        let mut user = Self::find(data, user_id).await?;

        // build response HTML
        let colors = ColorEntity::find_rauthy(data).await?;
        let lang = Language::try_from(&req).unwrap_or_default();
        let html = UserEmailChangeConfirmHtml::build(&colors, &lang, &user.email, &new_email);

        // save data
        let old_email = user.email;
        user.email = new_email;
        user.email_verified = true;
        user.save(data, Some(old_email.clone()), None).await?;
        ml.invalidate(data).await?;

        // finally, invalidate all existing sessions with the old email
        Session::invalidate_for_user(data, &user.id).await?;

        // send out confirmation E-Mails to both addresses
        send_email_confirm_change(data, &user, &user.email, &user.email, false).await;
        send_email_confirm_change(data, &user, &old_email, &user.email, false).await;

        let event_text = format!("{} -> {}", old_email, user.email);
        let ip = real_ip_from_req(&req).ok();
        data.tx_events
            .send_async(Event::user_email_change(event_text, ip))
            .await
            .unwrap();

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
        let len = group.bytes().len();
        if i == 0 {
            // the role is the first entry
            if old_groups.len() > len {
                let g = format!("{},", group);
                self.groups = Some(old_groups.replace(&g, ""));
            } else {
                self.groups = None;
            }
        } else {
            // the role is at the end or in the middle
            let g = format!(",{}", group);
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
        let len = role.bytes().len();
        if i == 0 {
            // the role is the first entry
            if self.roles.len() > len {
                let r = format!("{},", role);
                self.roles = self.roles.replace(&r, "");
            } else {
                self.roles = String::from("");
            }
        } else {
            // the role is at the end or in the middle
            let r = format!(",{}", role);
            self.roles = self.roles.replace(&r, "");
        }
    }

    pub async fn from_new_user_req(
        data: &web::Data<AppState>,
        new_user: NewUserRequest,
    ) -> Result<Self, ErrorResponse> {
        let roles = Role::sanitize(data, new_user.roles).await?;
        let groups = Group::sanitize(data, new_user.groups).await?;

        let user = Self {
            email: new_user.email.to_lowercase(),
            email_verified: false,
            given_name: new_user.given_name,
            family_name: new_user.family_name,
            language: new_user.language.into(),
            roles,
            groups,
            user_expires: new_user.user_expires,
            ..Default::default()
        };

        Ok(user)
    }

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

    pub fn get_roles(&self) -> Vec<String> {
        let mut res = Vec::new();
        if self.roles.ne("") {
            self.roles
                .split(',')
                .for_each(|r| res.push(r.trim().to_owned()));
        }
        res
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
            given_name: self.given_name,
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
        }
    }

    pub fn is_argon2_uptodate(&self, params: &Argon2Params) -> Result<bool, ErrorResponse> {
        if self.password.is_none() {
            error!(
                "Trying to validate argon2 params with not set password for user '{:?}'",
                self.id
            );
            return Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                String::from("Cannot validate argon2 param - password is not set"),
            ));
        }
        let hash = PasswordHash::new(self.password.as_ref().unwrap())
            .expect("Could not build Hash from password string");
        let curr_params =
            argon2::Params::try_from(&hash).expect("Could not extract params from hash");

        if curr_params.m_cost() == params.params.m_cost()
            && curr_params.t_cost() == params.params.t_cost()
            && curr_params.p_cost() == params.params.p_cost()
        {
            return Ok(true);
        }
        Ok(false)
    }

    pub fn is_admin(&self) -> bool {
        self.get_roles().contains(&RAUTHY_ADMIN_ROLE)
    }

    async fn is_email_free(data: &web::Data<AppState>, email: String) -> Result<(), ErrorResponse> {
        match User::find_by_email(data, email).await {
            Ok(_) => Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "E-Mail is already in use".to_string(),
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
                String::from("The password to validate is not set yet"),
            ));
        }
        let hash = self.password.as_ref().unwrap().to_owned();
        ComparePasswords::is_match(plain, hash).await
    }

    pub fn push_group(&mut self, group: &str) {
        if self.groups.is_some() {
            let g = self.groups.as_ref().unwrap();
            self.groups = Some(format!("{},{}", g, group));
        } else {
            self.groups = Some(group.to_owned());
        }
    }

    pub fn push_role(&mut self, role: &str) {
        if self.roles.ne("") {
            self.roles = format!("{},{}", self.roles, role);
        } else {
            role.clone_into(&mut self.roles);
        }
    }

    pub async fn request_password_reset(
        &self,
        data: &web::Data<AppState>,
        req: HttpRequest,
        redirect_uri: Option<String>,
    ) -> Result<(), ErrorResponse> {
        // TODO implement something with a Backup Code for passkey only accounts?
        // deny for passkey only accounts
        if self.account_type() == AccountType::Passkey {
            return Ok(());
        }

        let ml_res = MagicLink::find_by_user(data, self.id.clone()).await;
        // if an active magic link already exists - invalidate it.
        if let Ok(mut ml) = ml_res {
            if ml.exp > OffsetDateTime::now_utc().unix_timestamp() {
                warn!(
                    "Password reset request with already existing valid magic link from: {}",
                    real_ip_from_req(&req)?
                );
                ml.invalidate(data).await?;
            }
        }

        let usage = if self.password.is_none() && !self.has_webauthn_enabled() {
            MagicLinkUsage::NewUser(redirect_uri)
        } else {
            MagicLinkUsage::PasswordReset(redirect_uri)
        };
        let new_ml =
            MagicLink::create(data, self.id.clone(), data.ml_lt_pwd_reset as i64, usage).await?;
        send_pwd_reset(data, &new_ml, self).await;

        Ok(())
    }

    pub async fn validate_password(
        &self,
        data: &web::Data<AppState>,
        plain_password: String,
    ) -> Result<(), ErrorResponse> {
        if self.password.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::PasswordExpired,
                "No password set",
            ));
        }

        if let Some(exp) = self.password_expires {
            if exp < OffsetDateTime::now_utc().unix_timestamp() {
                // TODO introduce some "is allowed to refresh" variable
                if !self.enabled {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::PasswordExpired,
                        "The password has expired",
                    ));
                }

                // if the given password does match, send out a reset link to set a new one
                return if self.match_passwords(plain_password.clone()).await? {
                    let magic_link = MagicLink::create(
                        data,
                        self.id.clone(),
                        data.ml_lt_pwd_reset as i64,
                        MagicLinkUsage::PasswordReset(None),
                    )
                    .await?;
                    send_pwd_reset(data, &magic_link, self).await;

                    Err(ErrorResponse::new(
                        ErrorResponseType::PasswordRefresh,
                        "The password has expired. A reset E-Mail has been sent out.",
                    ))
                } else {
                    Err(ErrorResponse::new(
                        ErrorResponseType::Unauthorized,
                        "Invalid user credentials",
                    ))
                };
            }
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
            family_name: String::default(),
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
        }
    }
}

impl From<User> for UserResponseSimple {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            email: value.email,
            created_at: value.created_at,
            last_login: value.last_login,
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
            email: "admin@localhost.de".to_string(),
            given_name: "Admin".to_string(),
            family_name: "Rauthy".to_string(),
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
        };
        let session = Session::try_new(&user, 1, None);
        assert!(session.is_err());

        user.user_expires = None;
        let session = Session::try_new(&user, 1, None).unwrap();

        assert_eq!(session.is_valid(10, None), true);
        // sessions are validated with second accuracy
        std::thread::sleep(core::time::Duration::from_secs(2));
        assert_eq!(session.is_valid(10, None), false);

        // new sessions should always be in state 1 -> initializing
        assert_eq!(session.state, SessionState::Init.as_str());

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
        let mut user =  User {
            id: "123".to_string(),
            email: "admin@localhost.de".to_string(),
            given_name: "Admin".to_string(),
            family_name: "Rauthy".to_string(),
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
        let mut wrapped_params = Argon2Params {
            params: argon2::Params::new(16384, 3, 2, None).unwrap(),
        };
        let res = user.is_argon2_uptodate(&wrapped_params)?;
        assert_eq!(res, true);

        wrapped_params.params = argon2::Params::new(8192, 3, 2, None).unwrap();
        let res = user.is_argon2_uptodate(&wrapped_params)?;
        assert_eq!(res, false);

        wrapped_params.params = argon2::Params::new(16384, 4, 2, None).unwrap();
        let res = user.is_argon2_uptodate(&wrapped_params)?;
        assert_eq!(res, false);

        wrapped_params.params = argon2::Params::new(16384, 3, 5, None).unwrap();
        let res = user.is_argon2_uptodate(&wrapped_params)?;
        assert_eq!(res, false);

        Ok(())
    }
}
