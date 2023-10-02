use crate::app_state::{AppState, Argon2Params, DbTxn};
use crate::email::send_pwd_reset;
use crate::entity::colors::ColorEntity;
use crate::entity::groups::Group;
use crate::entity::magic_links::{MagicLink, MagicLinkUsage};
use crate::entity::password::PasswordPolicy;
use crate::entity::password::RecentPasswordsEntity;
use crate::entity::pow::Pow;
use crate::entity::refresh_tokens::RefreshToken;
use crate::entity::roles::Role;
use crate::entity::sessions::Session;
use crate::entity::webauthn::{PasskeyEntity, WebauthnServiceReq};
use crate::language::Language;
use crate::request::{
    NewUserRegistrationRequest, NewUserRequest, UpdateUserRequest, UpdateUserSelfRequest,
};
use crate::templates::UserEmailChangeConfirmHtml;
use actix_web::{web, HttpRequest};
use argon2::PasswordHash;
use rauthy_common::constants::{CACHE_NAME_12HR, IDX_USERS, WEBAUTHN_NO_PASSWORD_EXPIRY};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::password_hasher::{ComparePasswords, HashPassword};
use rauthy_common::utils::{get_client_ip, new_store_id};
use redhac::{cache_get, cache_get_from, cache_get_value, cache_insert, cache_remove, AckLevel};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::ops::Add;
use time::OffsetDateTime;
use tracing::{error, warn};

#[derive(Debug, Clone, PartialEq)]
pub enum AccountType {
    // New -> neither password nor a passkey has been set yet
    New,
    Password,
    Passkey,
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
}

// CRUD
impl User {
    // Inserts a user into the database
    pub async fn create(data: &web::Data<AppState>, new_user: User) -> Result<Self, ErrorResponse> {
        let lang = new_user.language.as_str();
        sqlx::query!(
            r#"insert into users
            (id, email, given_name, family_name, roles, groups, enabled, email_verified, created_at,
            language, user_expires)
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)"#,
            new_user.id,
            new_user.email,
            new_user.given_name,
            new_user.family_name,
            new_user.roles,
            new_user.groups,
            new_user.enabled,
            new_user.email_verified,
            new_user.created_at,
            lang,
            new_user.user_expires,
        )
        .execute(&data.db)
        .await?;

        let magic_link = MagicLink::create(
            data,
            new_user.id.clone(),
            data.ml_lt_pwd_first as i64,
            MagicLinkUsage::NewUser,
        )
        .await?;
        send_pwd_reset(data, &magic_link, &new_user).await;

        let mut users = User::find_all(data).await?;
        users.push(new_user.clone());
        cache_insert(
            CACHE_NAME_12HR.to_string(),
            IDX_USERS.to_string(),
            &data.caches.ha_cache_config,
            &users,
            AckLevel::Quorum,
        )
        .await?;

        Ok(new_user)
    }

    // Inserts a user into the database
    pub async fn create_from_new(
        data: &web::Data<AppState>,
        new_user_req: NewUserRequest,
    ) -> Result<User, ErrorResponse> {
        let new_user = User::from_new_user_req(data, new_user_req).await?;
        User::create(data, new_user).await
    }

    // Inserts a user from the open registration endpoint into the database
    pub async fn create_from_reg(
        data: &web::Data<AppState>,
        req_data: NewUserRegistrationRequest,
        lang: Language,
    ) -> Result<(), ErrorResponse> {
        let pow = Pow::find(data, &req_data.pow.challenge).await?;
        pow.validate(&req_data.pow.verifier).await?;
        pow.delete(data).await?;

        let mut new_user = User::from_reg_req(req_data);
        new_user.language = lang;
        User::create(data, new_user).await?;

        Ok(())
    }

    // Deletes a user
    pub async fn delete(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        sqlx::query!("delete from users where id = $1", self.id)
            .execute(&data.db)
            .await?;

        let users = User::find_all(data)
            .await?
            .into_iter()
            .filter(|u| u.id != self.id)
            .collect::<Vec<Self>>();

        cache_insert(
            CACHE_NAME_12HR.to_string(),
            IDX_USERS.to_string(),
            &data.caches.ha_cache_config,
            &users,
            AckLevel::Quorum,
        )
        .await?;

        let idx = format!("{}_{}", IDX_USERS, &self.id);
        cache_remove(
            CACHE_NAME_12HR.to_string(),
            idx,
            &data.caches.ha_cache_config,
            AckLevel::Quorum,
        )
        .await?;

        let idx = format!("{}_{}", IDX_USERS, &self.email);
        cache_remove(
            CACHE_NAME_12HR.to_string(),
            idx,
            &data.caches.ha_cache_config,
            AckLevel::Quorum,
        )
        .await?;

        // This should not be a problem with a non-existent user, but delete all sessions and
        // refresh tokens in advance.
        Session::invalidate_for_user(data, &self.id).await?;
        RefreshToken::invalidate_for_user(data, &self.id).await?;

        Ok(())
    }

    // Checks if a user exists in the database without fetching data
    pub async fn exists(data: &web::Data<AppState>, id: String) -> Result<(), ErrorResponse> {
        let idx = format!("{}_{}", IDX_USERS, id);
        let user_opt = cache_get!(
            User,
            CACHE_NAME_12HR.to_string(),
            idx.to_string(),
            &data.caches.ha_cache_config,
            false
        )
        .await?;
        if user_opt.is_some() {
            return Ok(());
        }

        sqlx::query!("select id from users where id = $1", id)
            .fetch_one(&data.db)
            .await?;

        Ok(())
    }

    // Returns a user by its id
    pub async fn find(data: &web::Data<AppState>, id: String) -> Result<Self, ErrorResponse> {
        let idx = format!("{}_{}", IDX_USERS, id);
        let user_opt = cache_get!(
            User,
            CACHE_NAME_12HR.to_string(),
            idx.to_string(),
            &data.caches.ha_cache_config,
            false
        )
        .await?;
        if user_opt.is_some() {
            return Ok(user_opt.unwrap());
        }

        // cannot be compile-time-checked because of 8byte integer on sqlite by default
        let user = sqlx::query_as::<_, Self>("select * from users where id = $1")
            .bind(&id)
            .fetch_one(&data.db)
            .await?;

        cache_insert(
            CACHE_NAME_12HR.to_string(),
            idx,
            &data.caches.ha_cache_config,
            &user,
            AckLevel::Leader,
        )
        .await?;
        Ok(user)
    }

    // Returns a user by its email
    pub async fn find_by_email(
        data: &web::Data<AppState>,
        email: String,
    ) -> Result<User, ErrorResponse> {
        let idx = format!("{}_{}", IDX_USERS, email);
        let user_opt = cache_get!(
            User,
            CACHE_NAME_12HR.to_string(),
            idx.clone(),
            &data.caches.ha_cache_config,
            false
        )
        .await?;
        if user_opt.is_some() {
            return Ok(user_opt.unwrap());
        }

        let user = sqlx::query_as::<_, Self>("select * from users where email = $1")
            .bind(&email)
            .fetch_one(&data.db)
            .await?;

        cache_insert(
            CACHE_NAME_12HR.to_string(),
            idx,
            &data.caches.ha_cache_config,
            &user,
            AckLevel::Leader,
        )
        .await?;
        Ok(user)
    }

    // Returns all existing users
    pub async fn find_all(data: &web::Data<AppState>) -> Result<Vec<Self>, ErrorResponse> {
        let users = cache_get!(
            Vec<User>,
            CACHE_NAME_12HR.to_string(),
            IDX_USERS.to_string(),
            &data.caches.ha_cache_config,
            false
        )
        .await?;
        if users.is_some() {
            return Ok(users.unwrap());
        }

        let res = sqlx::query_as::<_, Self>("select * from users")
            .fetch_all(&data.db)
            .await?;

        cache_insert(
            CACHE_NAME_12HR.to_string(),
            IDX_USERS.to_string(),
            &data.caches.ha_cache_config,
            &res,
            AckLevel::Quorum,
        )
        .await?;
        Ok(res)
    }

    pub async fn find_expired(data: &web::Data<AppState>) -> Result<Vec<Self>, ErrorResponse> {
        let now = OffsetDateTime::now_utc()
            .add(time::Duration::seconds(10))
            .unix_timestamp();
        tracing::debug!("User::find_expired - now: {}", now);
        let res = sqlx::query_as::<_, Self>("select * from users where user_expires < $1")
            .bind(now)
            .fetch_all(&data.db)
            .await?;
        tracing::debug!("User::find_expired - users: {:?}", res);
        Ok(res)
    }

    // Saves a user
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
            r#"update users set
            email = $1, given_name = $2, family_name = $3, password = $4, roles = $5, groups = $6,
            enabled = $7, email_verified = $8, password_expires = $9, last_login = $10,
            last_failed_login = $11, failed_login_attempts = $12, language = $13,
            webauthn_user_id = $14, user_expires = $15
            where id = $16"#,
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

        // TODO think about a good way to catch a possibly failing transaction -> cache invalidation
        let users = User::find_all(data)
            .await?
            .into_iter()
            .map(|mut u| {
                if u.id == self.id {
                    u = self.clone();
                }
                u
            })
            .collect::<Vec<Self>>();

        cache_insert(
            CACHE_NAME_12HR.to_string(),
            IDX_USERS.to_string(),
            &data.caches.ha_cache_config,
            &users,
            AckLevel::Quorum,
        )
        .await?;

        let idx = format!("{}_{}", IDX_USERS, &self.id);
        cache_insert(
            CACHE_NAME_12HR.to_string(),
            idx,
            &data.caches.ha_cache_config,
            &self,
            AckLevel::Quorum,
        )
        .await?;

        let idx = format!("{}_{}", IDX_USERS, &self.email);
        cache_insert(
            CACHE_NAME_12HR.to_string(),
            idx,
            &data.caches.ha_cache_config,
            &self,
            AckLevel::Quorum,
        )
        .await?;

        Ok(())
    }

    // Updates a user
    pub async fn update(
        data: &web::Data<AppState>,
        id: String,
        upd_user: UpdateUserRequest,
        user: Option<User>,
    ) -> Result<User, ErrorResponse> {
        let mut user = match user {
            None => User::find(data, id).await?,
            Some(user) => user,
        };
        let old_email = if user.email != upd_user.email {
            Some(user.email.clone())
        } else {
            None
        };

        user.email = upd_user.email;
        user.given_name = upd_user.given_name;
        user.family_name = upd_user.family_name;

        if let Some(lang) = upd_user.language {
            user.language = lang;
        }

        if let Some(password) = upd_user.password {
            user.apply_password_rules(data, &password).await?;
        }

        // sanitize roles
        user.roles = Role::sanitize(data, upd_user.roles).await?;

        // sanitize groups
        user.groups = Group::sanitize(data, upd_user.groups).await?;

        user.enabled = upd_user.enabled;
        user.email_verified = upd_user.email_verified;
        user.user_expires = upd_user.user_expires;

        user.save(data, old_email, None).await?;
        Ok(user)
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
    ) -> Result<(User, bool), ErrorResponse> {
        let user = User::find(data, id.clone()).await?;

        let mut password = None;
        if let Some(pwd_new) = upd_user.password_new {
            if let Some(pwd_curr) = upd_user.password_current {
                user.validate_password(data, pwd_curr).await?;
            } else if let Some(mfa_code) = upd_user.mfa_code {
                let svc_req = WebauthnServiceReq::find(data, mfa_code).await?;
                if svc_req.user_id != user.id {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::Forbidden,
                        "User ID does not match".to_string(),
                    ));
                }
                svc_req.delete(data).await?;
            } else {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "Cannot set a new password without the current one".to_string(),
                ));
            }
            password = Some(pwd_new);
        }

        let email_updated = if let Some(email) = upd_user.email {
            // if the email should be updated, we do not do it directly -> send out confirmation
            // email to old AND new address
            if email != user.email {
                // TODO
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
        };

        let user = User::update(data, id, req, Some(user)).await?;
        Ok((user, email_updated))
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
                "Only AccountType::Password can be converted".to_string(),
            ));
        }

        // check webauthn enabled
        if !user.has_webauthn_enabled() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Account type conversion can only happen with at least one active Passkey"
                    .to_string(),
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
        if self.password.is_some() {
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
        if rules.not_recently_used.is_some() {
            let most_recent_res = RecentPasswordsEntity::find(data, &self.id).await;
            if self.password.is_some() && most_recent_res.is_ok() {
                let mut most_recent = most_recent_res.unwrap();

                let recent_req = rules.not_recently_used.unwrap();

                // TODO cleanup after testing the migration
                let mut iteration = 1;
                // let argon2 = Argon2::default();
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
            } else {
                RecentPasswordsEntity::create(data, &self.id, &new_hash).await?;
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

    pub fn check_enabled(&self) -> Result<(), ErrorResponse> {
        if !self.enabled {
            return Err(ErrorResponse::new(
                ErrorResponseType::Disabled,
                String::from("The user is not enabled"),
            ));
        }
        Ok(())
    }

    pub fn check_expired(&self) -> Result<(), ErrorResponse> {
        if let Some(ts) = self.user_expires {
            if OffsetDateTime::now_utc().unix_timestamp() > ts {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Disabled,
                    String::from("User has expired"),
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
    ) -> Result<(String, String), ErrorResponse> {
        let mut ml = MagicLink::find(data, &confirm_id).await?;
        ml.validate(&user_id, &req, false)?;

        let usage = MagicLinkUsage::try_from(&ml.usage)?;
        let new_email = match usage {
            MagicLinkUsage::NewUser | MagicLinkUsage::PasswordReset => {
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
        let (html, nonce) =
            UserEmailChangeConfirmHtml::build(&colors, &lang, &user.email, &new_email);

        // save data
        let old_email = user.email;
        user.email = new_email;
        user.email_verified = true;
        user.save(data, Some(old_email), None).await?;
        ml.invalidate(data).await?;

        // finally, invalidate all existing sessions with the old email
        Session::invalidate_for_user(data, &user.id).await?;

        Ok((html, nonce))
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
            email: new_user.email,
            email_verified: false,
            given_name: new_user.given_name,
            family_name: new_user.family_name,
            language: new_user.language,
            roles,
            groups,
            user_expires: new_user.user_expires,
            ..Default::default()
        };

        Ok(user)
    }

    pub fn from_reg_req(new_user: NewUserRegistrationRequest) -> Self {
        Self {
            email: new_user.email,
            given_name: new_user.given_name,
            family_name: new_user.family_name,
            ..Default::default()
        }
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

    #[inline]
    pub fn has_webauthn_enabled(&self) -> bool {
        self.webauthn_user_id.is_some()
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
            self.roles = role.to_owned();
        }
    }

    pub async fn request_password_reset(
        &self,
        data: &web::Data<AppState>,
        req: HttpRequest,
    ) -> Result<(), ErrorResponse> {
        // TODO implement something with a Backup Code for passkey only accounts?
        // deny for passkey only accounts
        if self.account_type() == AccountType::Passkey {
            return Ok(());
        }

        let ml_res = MagicLink::find_by_user(data, self.id.clone()).await;
        // if an active magic link already exists - invalidate it.
        if ml_res.is_ok() {
            let mut ml = ml_res.unwrap();
            if ml.exp > OffsetDateTime::now_utc().unix_timestamp() {
                warn!(
                    "Password reset request with already existing valid magic link from: {}",
                    get_client_ip(&req)
                );
                ml.invalidate(data).await?;
            }
        }

        let usage = if self.password.is_none() && !self.has_webauthn_enabled() {
            MagicLinkUsage::NewUser
        } else {
            MagicLinkUsage::PasswordReset
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
                String::from("No password set"),
            ));
        }

        if let Some(exp) = self.password_expires {
            if exp < OffsetDateTime::now_utc().unix_timestamp() {
                // TODO introduce some "is allowed to refresh" variable
                if !self.enabled {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::PasswordExpired,
                        String::from("The password has expired"),
                    ));
                }

                // if the given password does match, send out a reset link to set a new one
                return if self.match_passwords(plain_password.clone()).await? {
                    let magic_link = MagicLink::create(
                        data,
                        self.id.clone(),
                        data.ml_lt_pwd_reset as i64,
                        MagicLinkUsage::PasswordReset,
                    )
                    .await?;
                    send_pwd_reset(data, &magic_link, self).await;

                    Err(ErrorResponse::new(
                        ErrorResponseType::PasswordRefresh,
                        String::from("The password has expired. A reset E-Mail has been sent out."),
                    ))
                } else {
                    Err(ErrorResponse::new(
                        ErrorResponseType::Unauthorized,
                        String::from("Invalid user credentials"),
                    ))
                };
            }
        }

        if self.match_passwords(plain_password).await? {
            Ok(())
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                String::from("Invalid user credentials"),
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
        assert_eq!(session.state, SessionState::Init);

        assert!(session.csrf_token.len() > 0);

        assert_eq!(session.groups_as_vec(), Ok(vec!["admin", "user"]));
        assert_eq!(
            session.roles_as_vec(),
            Ok(vec![
                "ROLE_rauthy_admin".to_string(),
                "ROLE_admin".to_string()
            ])
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
