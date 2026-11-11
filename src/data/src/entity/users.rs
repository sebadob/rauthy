use crate::database::{Cache, DB, PgClient};
use crate::email::email_change_confirm::send_email_confirm_change;
use crate::email::email_change_info::send_email_change_info_new;
use crate::email::password_reset::send_pwd_reset;
use crate::entity::continuation_token::ContinuationToken;
use crate::entity::groups::Group;
use crate::entity::magic_links::{MagicLink, MagicLinkUsage};
use crate::entity::one_time_password::OneTimePassword;
use crate::entity::pam::users::PamUser;
use crate::entity::password::PasswordPolicy;
use crate::entity::password::RecentPasswordsEntity;
use crate::entity::pictures::UserPicture;
use crate::entity::pwd_exp_mails::PasswordExpMail;
use crate::entity::refresh_tokens::{PgDeletedJtis, RefreshToken};
use crate::entity::roles::Role;
use crate::entity::sessions::Session;
use crate::entity::theme::ThemeCssFull;
use crate::entity::tos::ToS;
use crate::entity::tos_user_accept::ToSUserAccept;
use crate::entity::users_values::UserValues;
use crate::entity::webauthn::auth_req::WebauthnServiceReq;
use crate::entity::webauthn::passkey::PasskeyEntity;
use crate::events::event::Event;
use crate::html::templates::{HtmlTemplate, UserEmailChangeConfirmHtml};
use crate::language::Language;
use crate::rauthy_config::RauthyConfig;
use actix_web::HttpRequest;
use argon2::PasswordHash;
use chrono::Utc;
use core::str::Split;
use hiqlite::Params;
use hiqlite::macros::params;
use rauthy_api_types::PatchOp;
use rauthy_api_types::generic::SearchParamsIdx;
use rauthy_api_types::users::{
    ActiveOtp, NewUserRegistrationRequest, NewUserRequest, OtpKind, UpdateUserRequest,
    UpdateUserSelfRequest, UserAccountTypeResponse, UserResponse, UserResponseSimple,
    UserValuesRequest, UserValuesResponse,
};
use rauthy_common::constants::{
    CACHE_TTL_APP, CACHE_TTL_USER, IDX_USER_COUNT, IDX_USERS, RAUTHY_ADMIN_ROLE,
};
use rauthy_common::is_hiqlite;
use rauthy_common::password_hasher::{ComparePasswords, HashPassword};
use rauthy_common::utils::{csv_remove, csv_rename, new_store_id, real_ip_from_req};
use rauthy_derive::FromPgRow;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::cmp::max;
use std::default::Default;
use std::fmt::{Debug, Formatter};
use std::mem;
use std::ops::Add;
use std::str::FromStr;
use std::time::Duration;
use time::OffsetDateTime;
use tracing::{debug, error, trace};

/// A statement of the Postgres transaction of [`User::delete`], each binding the user id as `$1`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PgDeleteStep {
    /// A table that references the user. The refresh token deletes return the
    /// `access_token_jti` of their rows.
    Rows(&'static str),
    /// The user row, returning its `picture_id`
    User,
    /// Revokes the access tokens of the deleted refresh tokens
    RevokeAccessTokens,
    /// Revokes every access token issued to the user
    RevokeUserAccessTokens,
}

/// The statements of the Postgres transaction of [`User::delete`], in order. The user row
/// cascades into the consents and refresh tokens of the user, in the order of the foreign key
/// triggers. A consent revoke (`UserConsent::revoke`) and `Client::delete` take these tables in
/// another order: consents, device refresh tokens, legacy refresh tokens, the other refresh
/// tokens, then `issued_tokens`. Run at the same time, each could wait for a row the other one
/// holds, and Postgres would abort one of them as a deadlock. So they are deleted in that order
/// first, then the user row, and `issued_tokens` last.
///
/// The access tokens of the deleted refresh tokens are revoked in the same transaction. A
/// discard of one of these refresh tokens that runs at the same time and finds its row deleted
/// leaves the access token to this transaction.
///
/// A refresh rotation can still commit its replacement refresh token, device refresh token
/// included, after the token deletes and before the user row is deleted: its foreign key check
/// only takes a `KEY SHARE` lock on the user row. The cascade of the user row then removes it
/// without returning its access token. Its access token was committed before it, so the last
/// statement revokes every access token of the user by `user_id`. A refresh that comes later
/// fails its foreign key check, so its access token is never handed out. The user row is not
/// locked first instead: a device refresh token insert holds a `KEY SHARE` lock on its device
/// while it waits for the user row, and the cascade of the user row into that device would then
/// deadlock with it.
pub(crate) const PG_DELETE_STEPS: [PgDeleteStep; 7] = [
    PgDeleteStep::Rows("DELETE FROM user_consents WHERE user_id = $1"),
    PgDeleteStep::Rows(
        "DELETE FROM refresh_tokens_devices WHERE user_id = $1 RETURNING access_token_jti",
    ),
    PgDeleteStep::Rows(
        "DELETE FROM refresh_tokens WHERE user_id = $1 AND client_id IS NULL \
        RETURNING access_token_jti",
    ),
    PgDeleteStep::Rows("DELETE FROM refresh_tokens WHERE user_id = $1 RETURNING access_token_jti"),
    PgDeleteStep::User,
    PgDeleteStep::RevokeAccessTokens,
    PgDeleteStep::RevokeUserAccessTokens,
];

/// Whether the hiqlite delete of a user removed the user, when its transaction failed (`res` is
/// `Err`) or deleted no user row (`Ok(0)`). Either can come from a transaction that committed
/// while its result was lost, or from a re-run of a committed one. `found` tells whether the read
/// before the delete saw the user, and `gone` is the answer of a read on the leader after it: a
/// user that was there before and is gone after was deleted by this call. A user that was gone
/// before was not, so its original error, or `Ok(false)` for no user row, stands.
fn hql_delete_recovered(
    res: Result<usize, ErrorResponse>,
    found: bool,
    gone: bool,
) -> Result<bool, ErrorResponse> {
    if found && gone {
        return Ok(true);
    }
    res.map(|rows| rows > 0)
}

/// The state of the Postgres transaction of [`User::delete`]
#[derive(Debug, Default)]
pub(crate) struct PgDelete {
    jtis: PgDeletedJtis,
    /// `Some` with the `picture_id` of the user row once [`PgDeleteStep::User`] deleted it
    pub(crate) deleted: Option<Option<String>>,
}

impl PgDelete {
    pub(crate) async fn step(
        &mut self,
        txn: &deadpool_postgres::Transaction<'_>,
        step: PgDeleteStep,
        user_id: &str,
    ) -> Result<(), ErrorResponse> {
        match step {
            PgDeleteStep::Rows(sql) => self.jtis.delete(txn, sql, &[&user_id]).await?,
            PgDeleteStep::User => {
                self.deleted = txn
                    .query_opt(
                        "DELETE FROM users WHERE id = $1 RETURNING picture_id",
                        &[&user_id],
                    )
                    .await?
                    .map(|row| row.get::<_, Option<String>>(0));
            }
            PgDeleteStep::RevokeAccessTokens => self.jtis.revoke(txn).await?,
            PgDeleteStep::RevokeUserAccessTokens => {
                txn.execute(
                    "UPDATE issued_tokens SET revoked = true WHERE user_id = $1",
                    &[&user_id],
                )
                .await?;
            }
        }
        Ok(())
    }
}

static SQL_SAVE: &str = r#"
UPDATE USERS SET
email = $1, given_name = $2, family_name = $3, password = $4, roles = $5, groups = $6, enabled = $7,
email_verified = $8, password_expires = $9, last_login = $10, last_failed_login = $11,
failed_login_attempts = $12, language = $13, webauthn_user_id = $14, user_expires = $15,
auth_provider_id = $16, federation_uid = $17
WHERE id = $18"#;

/// `picture_id` is updated only here, by the picture upload and removal; a row insert sets it
/// too. A full-row save never writes it: it works on a copy that may predate a concurrent
/// picture change.
static SQL_SET_PICTURE_ID: &str = "UPDATE users SET picture_id = $1 WHERE id = $2 RETURNING email";

/// The new password hash `$3` is written only while the stored one is still the validated `$2`.
/// SQLite numbers `$n` in the order of first appearance, so they appear in order.
static SQL_SAVE_LOGIN_SUCCESS: &str = r#"
UPDATE users SET last_login = $1, last_failed_login = NULL, failed_login_attempts = NULL,
password = CASE WHEN password = $2 THEN COALESCE($3, password) ELSE password END
WHERE id = $4
RETURNING email"#;

/// The reset proves the email it was sent to, `$3`: `email_verified` is set only while the
/// stored email is still that one.
static SQL_SAVE_PASSWORD_RESET: &str = r#"
UPDATE users SET password = $1, password_expires = $2,
email_verified = CASE WHEN email = $3 THEN true ELSE email_verified END
WHERE id = $4
RETURNING email, email_verified, enabled"#;

static SQL_SAVE_LOGIN_FAILURE: &str = r#"
UPDATE users SET last_failed_login = $1,
failed_login_attempts = COALESCE(failed_login_attempts, 0) + 1
WHERE id = $2
RETURNING email, failed_login_attempts"#;

/// Runs the hiqlite write `sql` of one user row and decodes the row its `RETURNING` gives, if
/// the user exists.
async fn hql_returning<T>(
    sql: &'static str,
    params: Params,
    decode: impl FnOnce(hiqlite::Row<'_>) -> T,
) -> Result<Option<T>, ErrorResponse> {
    Ok(DB::hql()
        .execute_returning(sql, params)
        .await?
        .into_iter()
        .next()
        .transpose()?
        .map(decode))
}

/// The Postgres [`hql_returning`] on the client `cl`.
async fn pg_returning<T>(
    cl: &PgClient,
    sql: &str,
    params: &[&(dyn postgres_types::ToSql + Sync)],
    decode: impl FnOnce(&tokio_postgres::Row) -> T,
) -> Result<Option<T>, ErrorResponse> {
    Ok(DB::pg_query_rows_with(cl, sql, params, 1)
        .await?
        .first()
        .map(decode))
}

/// The stored columns a password reset returns.
#[derive(Debug, PartialEq)]
struct StoredReset {
    email: String,
    email_verified: bool,
    enabled: bool,
}

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

#[derive(Clone, Serialize, Deserialize, FromPgRow)]
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
    #[column(from_string)]
    pub language: Language,
    pub webauthn_user_id: Option<String>,
    pub user_expires: Option<i64>,
    pub auth_provider_id: Option<String>,
    pub federation_uid: Option<String>,
    /// Updated only by [`Self::set_picture_id`]: [`Self::save`] does not write it.
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

// CRUD
// Keyset cursors over a composite key: `created_at` alone (seconds) is not unique, so a
// plain comparison on it skips users at batch boundaries (same reasoning as `find_paginated`).
const SQL_FIND_BATCH: &str = r#"
SELECT * FROM users
WHERE (created_at, email) > ($1, $2)
ORDER BY created_at ASC, email ASC
LIMIT $3"#;

const SQL_FIND_FOR_SCIM_SYNC: &str = r#"
SELECT u.id AS user_id, email, email_verified, given_name, family_name, roles, groups, enabled,
    created_at, language, picture_id, birthdate, phone, street, zip, city, country,
    preferred_username, tz
FROM users u
LEFT JOIN users_values uv ON u.id = uv.id
WHERE (u.created_at, u.id) > ($1, $2)
ORDER BY u.created_at ASC, u.id ASC
LIMIT $3"#;

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

        // TODO migrate to the new distributed counter from hiqlite
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

    pub async fn count_dec() -> Result<(), ErrorResponse> {
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
            RauthyConfig::get().vars.lifetimes.magic_link_pwd_first,
            MagicLinkUsage::NewUser(post_reset_redirect_uri),
        )
        .await?;
        send_pwd_reset(&magic_link, &slf, user_tz).await;

        Ok(slf)
    }

    pub async fn create_federated(new_user: User) -> Result<Self, ErrorResponse> {
        Self::insert(new_user).await
    }

    pub async fn create_from_new(mut new_user_req: NewUserRequest) -> Result<User, ErrorResponse> {
        // The UserValuesValidator is intentionally not used on this path (see post_users), but a
        // broken tz would otherwise be stored as-is, so validate the format here.
        if let Some(tz) = &new_user_req.tz
            && chrono_tz::Tz::from_str(tz).is_err()
        {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "'tz' cannot be parsed",
            ));
        }

        // pre-uniqueness check for better UX and error handling; the DB unique index on
        // `users_values.preferred_username` is the authoritative guard (see UserValues::insert).
        if let Some(preferred_username) = &new_user_req.preferred_username {
            UserValues::validate_preferred_username_free(preferred_username.clone()).await?;
        }

        // neither value is read by `from_new_user_req()`, so both can be moved out directly
        let tz = mem::take(&mut new_user_req.tz);
        let preferred_username = mem::take(&mut new_user_req.preferred_username);
        let new_user = User::from_new_user_req(new_user_req).await?;
        let user = User::create(new_user, None, tz.as_deref()).await?;

        // UTC is the implicit default and is not persisted into the user values.
        let store_tz = if tz.as_deref() != Some("UTC") && tz.as_deref() != Some("Etc/UTC") {
            tz
        } else {
            None
        };
        if store_tz.is_some() || preferred_username.is_some() {
            UserValues::insert(
                user.id.clone(),
                UserValuesRequest {
                    tz: store_tz,
                    ..Default::default()
                },
                preferred_username,
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
        // We are cleaning up PAM data manually instead of using FKs because
        // of a chicken-and-egg problem in some cases.
        if let Ok(user) = PamUser::find_by_user_id(self.id.clone()).await {
            user.delete().await?;
        }

        Session::delete_by_user(&self.id).await?;

        // The stored `picture_id`, not the one of `self`, which may be an older copy. Postgres
        // takes it from the deleted row (RETURNING), hiqlite from a read just before the delete.
        let deleted: Option<Option<String>> = if is_hiqlite() {
            // A picture changed after this read is left behind as an orphan.
            // No `Row` may live past this statement: it is not `Send`.
            let (found, picture_id): (bool, Option<String>) = match DB::hql()
                .query_raw(
                    "SELECT picture_id FROM users WHERE id = $1",
                    params!(&self.id),
                )
                .await?
                .pop()
            {
                Some(mut row) => (true, row.get("picture_id")),
                None => (false, None),
            };
            // Revokes the access tokens of the user and of its refresh tokens before the cascade
            // of the user row deletes those.
            let res = DB::hql()
                .txn([
                    (
                        r#"
UPDATE issued_tokens SET revoked = true
WHERE user_id = $1
   OR jti IN (
       SELECT access_token_jti FROM refresh_tokens WHERE user_id = $1
       UNION
       SELECT access_token_jti FROM refresh_tokens_devices WHERE user_id = $1
   )"#,
                        params!(&self.id),
                    ),
                    ("DELETE FROM users WHERE id = $1", params!(&self.id)),
                ])
                .await
                .map_err(ErrorResponse::from)
                .and_then(|res| {
                    res.into_iter()
                        .try_fold(0, |_, rows| rows.map_err(ErrorResponse::from))
                });
            let user_deleted = match res {
                Ok(rows) if rows > 0 => true,
                // A user gone before this call was not deleted by it: another delete got there
                // first. Only for a user that was there, a read on the leader tells whether the
                // failed or empty delete removed it.
                res => {
                    let gone = found
                        && match DB::hql()
                            .query_consistent(
                                "SELECT 1 FROM users WHERE id = $1",
                                params!(&self.id),
                            )
                            .await
                        {
                            Ok(rows) => rows.is_empty(),
                            Err(err) => {
                                error!(
                                    user_id = self.id,
                                    ?err,
                                    "Error checking whether a failed delete removed the user"
                                );
                                false
                            }
                        };
                    hql_delete_recovered(res, found, gone)?
                }
            };
            user_deleted.then_some(picture_id)
        } else {
            let mut cl = DB::pg().await?;
            let txn = cl
                .build_transaction()
                .isolation_level(tokio_postgres::IsolationLevel::ReadCommitted)
                .start()
                .await?;
            let mut delete = PgDelete::default();
            for step in PG_DELETE_STEPS {
                delete.step(&txn, step, &self.id).await?;
            }
            if delete.deleted.is_some() {
                txn.commit().await?;
            }
            delete.deleted
        };
        // Another delete of the same user got there first and did the rest. A `User::save` of
        // an older copy may have put the user back into the cache after that delete cleared it,
        // so clear it again.
        let Some(picture_id) = deleted else {
            if let Err(err) = Self::invalidate_cache(&self.id, &self.email).await {
                error!(
                    user_id = self.id,
                    ?err,
                    "Error clearing the cache of a user deleted by another delete"
                );
            }
            return Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "user does not exist",
            ));
        };

        // The user is gone, so the delete succeeded. What follows only cleans up after it, and
        // a failure is logged, so that the caller still runs its own follow-up work.
        if let Err(err) = Self::invalidate_cache(&self.id, &self.email).await {
            error!(
                user_id = self.id,
                ?err,
                "Error clearing the cache of a deleted user"
            );
        }
        if let Err(err) = Self::count_dec().await {
            error!(user_id = self.id, ?err, "Error counting a deleted user");
        }

        // Only once the user is gone: a failed delete keeps the user with its picture. The
        // picture is not referenced anymore, so a failure here leaves an orphan, not an error.
        if let Some(picture_id) = picture_id {
            match UserPicture::delete_with_data(picture_id.clone()).await {
                Ok(()) => {}
                Err(err) if err.error == ErrorResponseType::NotFound => {
                    debug!(
                        picture_id,
                        user_id = self.id,
                        "The picture of a deleted user was already removed"
                    );
                }
                Err(err) => {
                    error!(
                        picture_id,
                        user_id = self.id,
                        ?err,
                        "Error deleting the picture of a deleted user"
                    );
                }
            }
        }

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
        let batch_size = batch_size as i64;
        let res = if is_hiqlite() {
            DB::hql()
                .query_as(
                    SQL_FIND_BATCH,
                    params!(from_created_at, after_email, batch_size),
                )
                .await?
        } else {
            DB::pg_query(
                SQL_FIND_BATCH,
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

        let mut res: Vec<Self> = if is_hiqlite() {
            DB::hql().query_as(sql, params!(like)).await?
        } else {
            let count = Self::count().await? as usize;
            DB::pg_query(sql, &[&like], count).await?
        };
        // LIKE matches substrings - keep only real members
        res.retain(|u| u.groups_iter().any(|g| g == group_name));

        Ok(res)
    }

    /// This is a very expensive query using `LIKE`, use only when necessary.
    pub async fn find_with_role(role_name: &str) -> Result<Vec<Self>, ErrorResponse> {
        let like = format!("%{role_name}%");
        let sql = "SELECT * FROM users WHERE roles LIKE $1";

        let mut res: Vec<Self> = if is_hiqlite() {
            DB::hql().query_as(sql, params!(like)).await?
        } else {
            let count = Self::count().await? as usize;
            DB::pg_query(sql, &[&like], count).await?
        };
        // LIKE matches substrings - keep only real members
        res.retain(|u| u.roles_iter().any(|r| r == role_name));

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
            // Keyset pagination over the composite key (created_at, id): `created_at` alone is
            // not unique, so without the `id` tie-breaker, rows with an equal `created_at`
            // would be duplicated or skipped across pages.
            if backwards {
                let sql = r#"
SELECT id, email, given_name, family_name, created_at, last_login, picture_id
FROM users
WHERE (created_at, id) < ($1, $2)
ORDER BY created_at DESC, id DESC
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
WHERE (created_at, id) > ($1, $2)
ORDER BY created_at ASC, id ASC
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
ORDER BY created_at DESC, id DESC
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
ORDER BY created_at ASC, id ASC
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
        last_user_id: &str,
        limit: u16,
    ) -> Result<Vec<(Self, UserValues)>, ErrorResponse> {
        let mut res = Vec::with_capacity(limit as usize);

        if is_hiqlite() {
            let rows = DB::hql()
                .query_raw(
                    SQL_FIND_FOR_SCIM_SYNC,
                    params!(last_created_ts, last_user_id, limit),
                )
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
            let rows = DB::pg_query_rows(
                SQL_FIND_FOR_SCIM_SYNC,
                &[&last_created_ts, &last_user_id, &(limit as i32)],
                limit as usize,
            )
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
                &self.id,
            ],
        )
        .await?;

        Ok(())
    }

    /// Writes every column of this copy but `picture_id`, which only [`Self::set_picture_id`]
    /// updates, then drops the cached user, also under `old_email`.
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
                    &self.id,
                ],
            )
            .await?;
        }

        if !self.enabled {
            Session::invalidate_for_user(&self.id).await?;
            RefreshToken::invalidate_for_user(&self.id).await?;
            // TODO trigger backchannel logout and SCIM updates as well?
            //  -> compare with old value beforehand somehow, probably in calling fn before updates
        }

        // `self` may be a copy read before a concurrent change of a column this save does not
        // write, like `picture_id`: drop the cached user instead of caching this copy
        self.invalidate_cache_also(old_email.as_deref()).await
    }

    /// Sets the `picture_id` of this user alone, in the database and in this copy, then drops
    /// the cached user, under the email of this copy and under the stored one, if a concurrent
    /// update changed it.
    pub async fn set_picture_id(&mut self, picture_id: Option<&str>) -> Result<(), ErrorResponse> {
        let stored_email = if is_hiqlite() {
            hql_returning(
                SQL_SET_PICTURE_ID,
                params!(picture_id.map(String::from), &self.id),
                |mut row| row.get::<String>("email"),
            )
            .await?
        } else {
            let cl = DB::pg().await?;
            self.pg_set_picture_id(&cl, picture_id).await?
        };
        self.picture_id = picture_id.map(String::from);
        self.invalidate_cache_also(stored_email.as_deref()).await
    }

    /// The Postgres statement of [`Self::set_picture_id`], the stored email if the row exists.
    async fn pg_set_picture_id(
        &self,
        cl: &PgClient,
        picture_id: Option<&str>,
    ) -> Result<Option<String>, ErrorResponse> {
        pg_returning(cl, SQL_SET_PICTURE_ID, &[&picture_id, &self.id], |row| {
            row.get("email")
        })
        .await
    }

    /// Saves a successful login: sets `last_login` to now and clears the failed login columns,
    /// in this copy and in the database, and replaces a rehashed password with `new_password`
    /// in the database only: this copy keeps the hash the login validated.
    /// Unlike [`Self::save`], it does not write back the other columns of this copy, which a
    /// login may have read long before, e.g. before it spent its time on the password hash, so
    /// a concurrent change of the user (a picture upload, an admin update) is kept.
    /// `new_password` replaces only the hash this copy holds, the one the login validated: a
    /// password changed meanwhile is kept. The cached user is dropped under the email of this
    /// copy, and under the stored one, if a concurrent update changed it.
    pub async fn save_login_success(
        &mut self,
        new_password: Option<&str>,
    ) -> Result<(), ErrorResponse> {
        self.last_login = Some(Utc::now().timestamp());
        self.last_failed_login = None;
        self.failed_login_attempts = None;

        let stored_email = if is_hiqlite() {
            hql_returning(
                SQL_SAVE_LOGIN_SUCCESS,
                params!(
                    self.last_login,
                    &self.password,
                    new_password.map(String::from),
                    &self.id
                ),
                |mut row| row.get::<String>("email"),
            )
            .await?
        } else {
            let cl = DB::pg().await?;
            self.pg_save_login_success(&cl, new_password).await?
        };

        self.invalidate_cache_also(stored_email.as_deref()).await
    }

    /// The Postgres statement of [`Self::save_login_success`], the stored email if the row exists.
    async fn pg_save_login_success(
        &self,
        cl: &PgClient,
        new_password: Option<&str>,
    ) -> Result<Option<String>, ErrorResponse> {
        pg_returning(
            cl,
            SQL_SAVE_LOGIN_SUCCESS,
            &[&self.last_login, &self.password, &new_password, &self.id],
            |row| row.get("email"),
        )
        .await
    }

    /// Saves a failed login: sets `last_failed_login` to now and increments
    /// `failed_login_attempts` in the database, then sets both in this copy, the counter to the
    /// stored one. Like [`Self::save_login_success`], it writes no other column, and the counter
    /// counts every failed login, also those that run at the same time. For a user deleted
    /// meanwhile, the counter of this copy is incremented instead.
    pub async fn save_login_failure(&mut self) -> Result<(), ErrorResponse> {
        self.last_failed_login = Some(Utc::now().timestamp());

        let stored = if is_hiqlite() {
            hql_returning(
                SQL_SAVE_LOGIN_FAILURE,
                params!(self.last_failed_login, &self.id),
                |mut row| {
                    (
                        row.get::<String>("email"),
                        row.get::<i64>("failed_login_attempts"),
                    )
                },
            )
            .await?
        } else {
            let cl = DB::pg().await?;
            self.pg_save_login_failure(&cl).await?
        };

        let (stored_email, attempts) = match stored {
            Some((email, attempts)) => (Some(email), attempts),
            None => (None, self.failed_login_attempts.unwrap_or_default() + 1),
        };
        self.failed_login_attempts = Some(attempts);

        self.invalidate_cache_also(stored_email.as_deref()).await
    }

    /// The Postgres statement of [`Self::save_login_failure`], the stored email and failed
    /// login count if the row exists.
    async fn pg_save_login_failure(
        &self,
        cl: &PgClient,
    ) -> Result<Option<(String, i64)>, ErrorResponse> {
        pg_returning(
            cl,
            SQL_SAVE_LOGIN_FAILURE,
            &[&self.last_failed_login, &self.id],
            |row| (row.get("email"), row.get("failed_login_attempts")),
        )
        .await
    }

    /// Saves a password reset: writes the `password` and `password_expires` that
    /// [`Self::apply_password_rules`] set in this copy, and sets `email_verified` while the
    /// stored email is still the one of this copy, the one the reset was sent to; this copy
    /// takes the stored `email_verified`. Unlike [`Self::save`], it does not write back the
    /// other columns of this copy, which the reset read before it spent its time on the password
    /// hash, so a concurrent change of the user (an admin update, a failed login) is kept. The
    /// cached user is dropped under the email of this copy, and under the stored one, if a
    /// concurrent update changed it. Like [`Self::save`], it drops the sessions and refresh
    /// tokens of a user the stored row has disabled.
    pub async fn save_password_reset(&mut self) -> Result<(), ErrorResponse> {
        let stored = if is_hiqlite() {
            hql_returning(
                SQL_SAVE_PASSWORD_RESET,
                params!(&self.password, self.password_expires, &self.email, &self.id),
                |mut row| StoredReset {
                    email: row.get("email"),
                    email_verified: row.get("email_verified"),
                    enabled: row.get("enabled"),
                },
            )
            .await?
        } else {
            let cl = DB::pg().await?;
            self.pg_save_password_reset(&cl).await?
        };

        let stored_email = match stored {
            Some(StoredReset {
                email,
                email_verified,
                enabled,
            }) => {
                self.email_verified = email_verified;
                if !enabled {
                    // like `save` does for a disabled user, judged by the stored row, which a
                    // concurrent update may have disabled
                    Session::invalidate_for_user(&self.id).await?;
                    RefreshToken::invalidate_for_user(&self.id).await?;
                }
                Some(email)
            }
            None => None,
        };
        self.invalidate_cache_also(stored_email.as_deref()).await
    }

    /// The Postgres statement of [`Self::save_password_reset`], the stored email,
    /// `email_verified` and `enabled` if the row exists.
    async fn pg_save_password_reset(
        &self,
        cl: &PgClient,
    ) -> Result<Option<StoredReset>, ErrorResponse> {
        pg_returning(
            cl,
            SQL_SAVE_PASSWORD_RESET,
            &[
                &self.password,
                &self.password_expires,
                &self.email,
                &self.id,
            ],
            |row| StoredReset {
                email: row.get("email"),
                email_verified: row.get("email_verified"),
                enabled: row.get("enabled"),
            },
        )
        .await
    }

    /// Drops the cached user under its id and the email of this copy, and under `other_email`
    /// too, if it differs: the email before a change, or the stored one a write returned, which
    /// a concurrent update may have changed.
    async fn invalidate_cache_also(&self, other_email: Option<&str>) -> Result<(), ErrorResponse> {
        Self::invalidate_cache(&self.id, &self.email).await?;
        if let Some(email) = other_email.filter(|email| *email != self.email) {
            let idx = format!("{IDX_USERS}_{email}");
            DB::hql().delete(Cache::User, idx).await?;
        }
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
        is_self_update: bool,
        password_reset_by: Option<&str>,
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

        if upd_user.password.is_some() && !is_self_update {
            let actor = password_reset_by.unwrap_or("admin");
            RauthyConfig::get()
                .tx_events
                .send_async(Event::user_password_reset(
                    format!("Reset done by {actor} for user {}", user.email),
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

        // make sure to clean up exp email reminders after a password update
        if upd_user.password.is_some() {
            PasswordExpMail::delete(user.id.clone()).await?;
        }

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
                    Duration::from_secs(3600),
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
            User::update(id, req, Some(user), preferred_username, true, None).await?;

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

        // emails are stored lowercased (see from_new_user_req / create_from_reg), so the
        // pre-check must compare against the normalized form as well
        let email = email.to_lowercase();

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
        if let Some(groups) = &self.groups {
            self.groups = Some(csv_remove(groups, group)).filter(|g| !g.is_empty());
        }
    }

    pub fn delete_role(&mut self, role: &str) {
        self.roles = csv_remove(&self.roles, role);
    }

    pub fn rename_group(&mut self, old: &str, new: &str) {
        self.groups = self.groups.as_deref().map(|g| csv_rename(g, old, new));
    }

    pub fn rename_role(&mut self, old: &str, new: &str) {
        self.roles = csv_rename(&self.roles, old, new);
    }

    pub fn email_recipient_name(&self) -> String {
        if let Some(n) = &self.family_name {
            format!("{} {n}", self.given_name)
        } else {
            self.given_name.to_string()
        }
    }

    /// The `name` claim for the `id_token` and `/userinfo`. With
    /// `user_values.name_order_reverse`, the family name comes first.
    pub fn full_name(&self) -> String {
        let mut parts = [
            self.given_name.as_str(),
            self.family_name.as_deref().unwrap_or_default(),
        ];
        if RauthyConfig::get().vars.user_values.name_order_reverse {
            parts.reverse();
        }
        parts
            .into_iter()
            .filter(|p| !p.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
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
        if let Some(groups) = &self.groups {
            groups.split(',').map(String::from).collect::<Vec<_>>()
        } else {
            Vec::default()
        }
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

    pub async fn get_otp_kind(&self) -> Result<Vec<ActiveOtp>, ErrorResponse> {
        Ok(OneTimePassword::find_for_user(&self.id)
            .await?
            .into_iter()
            .map(|f| ActiveOtp {
                otp_id: f.id,
                otp_kind: f.kind,
            })
            .collect())
    }

    pub async fn has_otp_of_kind_enabled(&self, kind: &OtpKind) -> bool {
        OneTimePassword::find_active_kind_for_user(kind, &self.id)
            .await
            .is_ok()
    }

    #[inline(always)]
    pub async fn has_otp_enabled(&self) -> bool {
        OneTimePassword::find_active_for_user(&self.id)
            .await
            .is_ok_and(|f| !f.is_empty())
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
        if let Some(groups) = &self.groups {
            self.groups = Some(format!("{groups},{group}"));
        } else {
            self.groups = Some(group.to_string());
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
            RauthyConfig::get().vars.lifetimes.magic_link_pwd_reset,
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

            // If the given password does match, send out a reset link to set a new one.
            return if self.match_passwords(plain_password.clone()).await? {
                // Sending a new magic link without rate-limiting may seem bad at first, but the
                // user has to be in a specific state AND the currently set password must be known.
                // This means there is no real way for an attacker that knows a users' email that
                // also has an expired password to trigger email spam.
                let magic_link = MagicLink::create(
                    self.id.clone(),
                    RauthyConfig::get().vars.lifetimes.magic_link_pwd_reset,
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
    use crate::database::HQL_TEST_LOCK;
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

        assert!(!session.csrf_token.is_empty());

        assert_eq!(session.groups_as_vec(), Ok(vec!["admin", "user"]));
        assert_eq!(
            session.roles_as_vec(),
            Ok(vec!["rauthy_admin".to_string(), "admin".to_string()])
        );
    }

    fn check_password_expired(user: &User) -> Result<(), ErrorResponse> {
        if let Some(exp) = user.password_expires
            && exp < Utc::now().timestamp()
        {
            return Err(ErrorResponse::new(
                ErrorResponseType::PasswordExpired,
                "The password has expired",
            ));
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

    /// A user inserted into the test hiqlite node, with the email `{id}@example.com`
    async fn hql_insert_user(id: &str, picture_id: Option<&str>) -> User {
        User::insert(User {
            id: id.to_string(),
            email: format!("{id}@example.com"),
            given_name: id.to_string(),
            roles: "user".to_string(),
            picture_id: picture_id.map(String::from),
            ..Default::default()
        })
        .await
        .unwrap()
    }

    async fn hql_cached(idx: String) -> Option<User> {
        DB::hql().get(Cache::User, idx).await.unwrap()
    }

    async fn hql_picture_ids() -> Vec<String> {
        DB::hql()
            .query_raw("SELECT id FROM pictures ORDER BY id", params!())
            .await
            .unwrap()
            .into_iter()
            .map(|mut row| row.get("id"))
            .collect()
    }

    /// A hiqlite delete whose transaction committed but whose result was lost, or whose re-run
    /// deleted no row, counts as done once the leader no longer has the user, if the user was
    /// there before it. While the user is still there, or was gone before, the original error or
    /// NotFound stands.
    #[test]
    fn test_hql_delete_recovered() {
        let lost = || ErrorResponse::new(ErrorResponseType::Connection, "lost");

        assert!(hql_delete_recovered(Err(lost()), true, true).unwrap());
        assert!(hql_delete_recovered(Ok(0), true, true).unwrap());

        let err = hql_delete_recovered(Err(lost()), true, false).unwrap_err();
        assert_eq!(err.error, ErrorResponseType::Connection);
        assert!(!hql_delete_recovered(Ok(0), true, false).unwrap());

        // A user that was gone before the delete was not deleted by it, whatever the leader says.
        let err = hql_delete_recovered(Err(lost()), false, true).unwrap_err();
        assert_eq!(err.error, ErrorResponseType::Connection);
        assert!(!hql_delete_recovered(Ok(0), false, true).unwrap());
    }

    /// A delete through a stale copy of a user another delete already removed returns NotFound
    /// and leaves the user count alone, but still clears the cache entries a concurrent
    /// `User::save` may have put back.
    #[tokio::test]
    async fn test_delete_twice_hiqlite() {
        let _lock = HQL_TEST_LOCK.lock().await;
        crate::database::init_test_hiqlite();
        let user = hql_insert_user("del_twice", None).await;
        let stale = user.clone();
        user.delete().await.unwrap();
        let count = User::count().await.unwrap();

        let idx_id = format!("{IDX_USERS}_{}", user.id);
        let idx_email = format!("{IDX_USERS}_{}", user.email);
        for idx in [&idx_id, &idx_email] {
            DB::hql()
                .put(Cache::User, idx.clone(), &stale, CACHE_TTL_USER)
                .await
                .unwrap();
        }

        let err = stale.delete().await.unwrap_err();
        assert_eq!(err.error, ErrorResponseType::NotFound);
        assert_eq!(User::count().await.unwrap(), count);
        assert!(hql_cached(idx_id).await.is_none());
        assert!(hql_cached(idx_email).await.is_none());
    }

    async fn hql_insert_picture(id: &str) {
        DB::hql()
            .execute(
                "INSERT INTO pictures (id, content_type, storage, data) \
                VALUES ($1, 'image/png', 'db', NULL)",
                params!(id),
            )
            .await
            .unwrap();
    }

    /// A save through a copy of the user read before a picture upload, like a login that read
    /// the user before it hashed the password, keeps the uploaded picture. The save drops the
    /// cached user instead of caching its copy, which still has no picture.
    #[tokio::test]
    async fn test_save_keeps_concurrent_picture_hiqlite() {
        let _lock = HQL_TEST_LOCK.lock().await;
        crate::database::init_test_hiqlite();
        hql_insert_picture("save_pic_1").await;
        let mut user = hql_insert_user("save_pic", None).await;
        let idx_id = format!("{IDX_USERS}_{}", user.id);
        let idx_email = format!("{IDX_USERS}_{}", user.email);

        let mut stale = User::find(user.id.clone()).await.unwrap();
        user.set_picture_id(Some("save_pic_1")).await.unwrap();
        assert!(hql_cached(idx_id.clone()).await.is_none());
        assert!(hql_cached(idx_email.clone()).await.is_none());
        let found = User::find(user.id.clone()).await.unwrap();
        assert_eq!(found.picture_id.as_deref(), Some("save_pic_1"));
        assert!(hql_cached(idx_id.clone()).await.is_some());

        assert_eq!(stale.picture_id, None);
        stale.given_name = "changed".to_string();
        stale.save(None).await.unwrap();
        assert!(hql_cached(idx_id).await.is_none());
        assert!(hql_cached(idx_email).await.is_none());
        let saved = User::find(user.id.clone()).await.unwrap();
        assert_eq!(saved.picture_id.as_deref(), Some("save_pic_1"));
        assert_eq!(saved.given_name, "changed");
        let saved = User::find_by_email(user.email.clone()).await.unwrap();
        assert_eq!(saved.picture_id.as_deref(), Some("save_pic_1"));
    }

    /// `set_picture_id` writes only the `picture_id` of its user, and sets it in its copy: a
    /// removal through a stale copy keeps the columns changed meanwhile, and other users are left
    /// alone.
    #[tokio::test]
    async fn test_set_picture_id_hiqlite() {
        let _lock = HQL_TEST_LOCK.lock().await;
        crate::database::init_test_hiqlite();
        hql_insert_picture("set_pic_1").await;
        hql_insert_picture("set_pic_other").await;
        let user = hql_insert_user("set_pic", Some("set_pic_1")).await;
        let other = hql_insert_user("set_pic_other", Some("set_pic_other")).await;
        let mut stale = User::find(user.id.clone()).await.unwrap();

        let mut admin = User::find(user.id.clone()).await.unwrap();
        admin.enabled = false;
        admin.roles = "admin".to_string();
        admin.save(None).await.unwrap();

        stale.set_picture_id(None).await.unwrap();
        assert_eq!(stale.picture_id, None);
        let saved = User::find(user.id.clone()).await.unwrap();
        assert_eq!(saved.picture_id, None);
        assert!(!saved.enabled);
        assert_eq!(saved.roles, "admin");
        let other = User::find(other.id).await.unwrap();
        assert_eq!(other.picture_id.as_deref(), Some("set_pic_other"));
    }

    /// `set_picture_id` through a copy read before a concurrent email change drops the cached
    /// user under the stored email too, not only under the email of the copy.
    #[tokio::test]
    async fn test_set_picture_id_concurrent_email_change_hiqlite() {
        let _lock = HQL_TEST_LOCK.lock().await;
        crate::database::init_test_hiqlite();
        hql_insert_picture("set_pic_email_1").await;
        let user = hql_insert_user("set_pic_email", None).await;
        let mut stale = User::find(user.id.clone()).await.unwrap();

        let mut admin = User::find(user.id.clone()).await.unwrap();
        admin.email = "set_pic_email_new@example.com".to_string();
        admin.save(Some(user.email.clone())).await.unwrap();
        User::find_by_email(admin.email.clone()).await.unwrap();
        let idx_new = format!("{IDX_USERS}_{}", admin.email);
        assert!(hql_cached(idx_new.clone()).await.is_some());

        stale.set_picture_id(Some("set_pic_email_1")).await.unwrap();
        assert_eq!(stale.picture_id.as_deref(), Some("set_pic_email_1"));
        assert!(hql_cached(idx_new).await.is_none());
        let saved = User::find_by_email(admin.email.clone()).await.unwrap();
        assert_eq!(saved.picture_id.as_deref(), Some("set_pic_email_1"));
    }

    /// A password reset saved through a copy read before concurrent changes, like an admin
    /// update or a failed login during the password hashing, keeps those changes: it writes
    /// only the new password and its expiry, and drops the cached user, also under the stored
    /// email a concurrent update changed. The reset proves only the email of its copy, so a
    /// stored email changed meanwhile stays unverified, in the database and in the copy.
    #[tokio::test]
    async fn test_save_password_reset_keeps_concurrent_changes_hiqlite() {
        let _lock = HQL_TEST_LOCK.lock().await;
        crate::database::init_test_hiqlite();
        let user = hql_insert_login_user("pwd_reset").await;
        let mut reset = User::find(user.id.clone()).await.unwrap();
        assert!(reset.enabled);
        assert!(!reset.email_verified);

        let mut admin = User::find(user.id.clone()).await.unwrap();
        admin.enabled = false;
        admin.roles = "admin".to_string();
        admin.email = "pwd_reset_new@example.com".to_string();
        admin.email_verified = false;
        admin.save(Some(user.email.clone())).await.unwrap();
        let mut failed = User::find(user.id.clone()).await.unwrap();
        failed.save_login_failure().await.unwrap();
        User::find_by_email(admin.email.clone()).await.unwrap();
        let idx_id = format!("{IDX_USERS}_{}", user.id);
        let idx_new = format!("{IDX_USERS}_{}", admin.email);
        assert!(hql_cached(idx_new.clone()).await.is_some());

        reset.password = Some("reset".to_string());
        reset.password_expires = Some(42);
        reset.save_password_reset().await.unwrap();
        assert!(!reset.email_verified);
        assert!(hql_cached(idx_id).await.is_none());
        assert!(hql_cached(idx_new).await.is_none());

        let saved = User::find(user.id.clone()).await.unwrap();
        assert_eq!(saved.password.as_deref(), Some("reset"));
        assert_eq!(saved.password_expires, Some(42));
        assert!(!saved.email_verified);
        assert!(!saved.enabled);
        assert_eq!(saved.roles, "admin");
        assert_eq!(saved.email, "pwd_reset_new@example.com");
        assert_eq!(saved.failed_login_attempts, Some(1));
        assert_eq!(saved.last_failed_login, failed.last_failed_login);
    }

    /// A password reset of a user the stored row has disabled, also through a copy read before
    /// the user was disabled, drops the sessions and refresh tokens of the user, like a save of
    /// a disabled user does. It verifies the email it was sent to, which is still the stored one.
    #[tokio::test]
    async fn test_save_password_reset_disabled_user_hiqlite() {
        let _lock = HQL_TEST_LOCK.lock().await;
        crate::database::init_test_hiqlite();
        let user = hql_insert_login_user("pwd_reset_off").await;
        hql_insert_user("pwd_reset_off_other", None).await;
        let mut reset = User::find(user.id.clone()).await.unwrap();
        assert!(reset.enabled);
        assert!(!reset.email_verified);

        let mut admin = User::find(user.id.clone()).await.unwrap();
        admin.enabled = false;
        admin.save(None).await.unwrap();
        // issued after the user was disabled, e.g. by a login past its checks
        hql_insert_tokens(&user.id).await;
        hql_insert_tokens("pwd_reset_off_other").await;
        for (sid, uid) in [
            ("pwd_reset_off_sid", user.id.as_str()),
            ("pwd_reset_off_other_sid", "pwd_reset_off_other"),
        ] {
            DB::hql()
                .execute(
                    "INSERT INTO sessions (id, csrf_token, user_id, is_mfa, state, exp, last_seen) \
                    VALUES ($1, 'csrf', $2, 0, 'Auth', 2, 1)",
                    params!(sid, uid),
                )
                .await
                .unwrap();
        }
        let count = async |table: &str, uid: &str| -> i64 {
            DB::hql()
                .query_raw(
                    format!("SELECT COUNT(*) AS n FROM {table} WHERE user_id = $1"),
                    params!(uid.to_string()),
                )
                .await
                .unwrap()
                .pop()
                .unwrap()
                .get("n")
        };
        assert_eq!(count("refresh_tokens", &user.id).await, 1);
        assert_eq!(count("sessions", &user.id).await, 1);

        reset.password = Some("reset".to_string());
        reset.save_password_reset().await.unwrap();
        assert!(reset.email_verified);

        let saved = User::find(user.id.clone()).await.unwrap();
        assert_eq!(saved.password.as_deref(), Some("reset"));
        assert!(saved.email_verified);
        assert!(!saved.enabled);
        assert_eq!(count("refresh_tokens", &user.id).await, 0);
        assert_eq!(count("sessions", &user.id).await, 0);
        assert_eq!(count("refresh_tokens", "pwd_reset_off_other").await, 1);
        assert_eq!(count("sessions", "pwd_reset_off_other").await, 1);
    }

    /// A user with the validated password hash `validated` for the login state tests.
    async fn hql_insert_login_user(id: &str) -> User {
        let mut user = hql_insert_user(id, None).await;
        user.password = Some("validated".to_string());
        user.save(None).await.unwrap();
        user
    }

    /// A login saved through a copy of the user read before a concurrent change, like a picture
    /// upload or an admin update during the password hashing, keeps that change and drops the
    /// cached user. The failed login columns are cleared.
    #[tokio::test]
    async fn test_save_login_success_keeps_concurrent_changes_hiqlite() {
        let _lock = HQL_TEST_LOCK.lock().await;
        crate::database::init_test_hiqlite();
        hql_insert_picture("login_ok_pic").await;
        let mut user = hql_insert_login_user("login_ok").await;
        user.last_failed_login = Some(7);
        user.failed_login_attempts = Some(3);
        user.save(None).await.unwrap();
        let mut login = User::find(user.id.clone()).await.unwrap();
        assert_eq!(login.failed_login_attempts, Some(3));

        user.set_picture_id(Some("login_ok_pic")).await.unwrap();
        let mut admin = User::find(user.id.clone()).await.unwrap();
        admin.roles = "admin".to_string();
        admin.enabled = false;
        admin.save(None).await.unwrap();
        // cache the updated user, which the login must not leave behind with its old state
        User::find(user.id.clone()).await.unwrap();

        login.save_login_success(Some("rehashed")).await.unwrap();
        assert!(login.last_login.is_some());
        assert_eq!(login.last_failed_login, None);
        assert_eq!(login.failed_login_attempts, None);

        let idx_id = format!("{IDX_USERS}_{}", user.id);
        let idx_email = format!("{IDX_USERS}_{}", user.email);
        assert!(hql_cached(idx_id).await.is_none());
        assert!(hql_cached(idx_email).await.is_none());
        let saved = User::find(user.id.clone()).await.unwrap();
        assert_eq!(saved.picture_id.as_deref(), Some("login_ok_pic"));
        assert_eq!(saved.roles, "admin");
        assert!(!saved.enabled);
        assert_eq!(saved.last_login, login.last_login);
        assert_eq!(saved.last_failed_login, None);
        assert_eq!(saved.failed_login_attempts, None);
        assert_eq!(saved.password.as_deref(), Some("rehashed"));

        // without a rehash, the password is left alone
        login.password = Some("rehashed".to_string());
        login.save_login_success(None).await.unwrap();
        let saved = User::find(user.id.clone()).await.unwrap();
        assert_eq!(saved.password.as_deref(), Some("rehashed"));
        assert_eq!(saved.picture_id.as_deref(), Some("login_ok_pic"));
    }

    /// A rehash does not overwrite a password changed during the login, e.g. by a reset, while
    /// the login columns are still saved.
    #[tokio::test]
    async fn test_save_login_success_keeps_concurrent_password_hiqlite() {
        let _lock = HQL_TEST_LOCK.lock().await;
        crate::database::init_test_hiqlite();
        let user = hql_insert_login_user("login_ok_pwd").await;
        let mut login = User::find(user.id.clone()).await.unwrap();

        let mut reset = User::find(user.id.clone()).await.unwrap();
        reset.password = Some("changed".to_string());
        reset.save(None).await.unwrap();

        login.save_login_success(Some("rehashed")).await.unwrap();
        let saved = User::find(user.id.clone()).await.unwrap();
        assert_eq!(saved.password.as_deref(), Some("changed"));
        assert_eq!(saved.last_login, login.last_login);
    }

    /// Failed logins that run at the same time through copies read before any of them each
    /// count, and each copy gets the stored counter. They write no other column.
    #[tokio::test]
    async fn test_save_login_failure_counts_concurrent_failures_hiqlite() {
        let _lock = HQL_TEST_LOCK.lock().await;
        crate::database::init_test_hiqlite();
        hql_insert_picture("login_fail_pic").await;
        let mut user = hql_insert_login_user("login_fail").await;
        let mut logins = Vec::new();
        for _ in 0..5 {
            logins.push(User::find(user.id.clone()).await.unwrap());
        }

        user.set_picture_id(Some("login_fail_pic")).await.unwrap();
        let mut admin = User::find(user.id.clone()).await.unwrap();
        admin.enabled = false;
        admin.password = Some("changed".to_string());
        admin.save(None).await.unwrap();

        let mut counts =
            futures_util::future::join_all(logins.into_iter().map(async |mut login| {
                login.save_login_failure().await.unwrap();
                assert!(login.last_failed_login.is_some());
                login.failed_login_attempts.unwrap()
            }))
            .await;
        counts.sort_unstable();
        assert_eq!(counts, [1, 2, 3, 4, 5]);

        let saved = User::find(user.id.clone()).await.unwrap();
        assert_eq!(saved.failed_login_attempts, Some(5));
        assert!(saved.last_failed_login.is_some());
        assert_eq!(saved.picture_id.as_deref(), Some("login_fail_pic"));
        assert!(!saved.enabled);
        assert_eq!(saved.password.as_deref(), Some("changed"));

        // a successful login resets the counter, the next failure starts over
        let mut login = User::find(user.id.clone()).await.unwrap();
        login.save_login_success(None).await.unwrap();
        login.save_login_failure().await.unwrap();
        assert_eq!(login.failed_login_attempts, Some(1));
        let saved = User::find(user.id.clone()).await.unwrap();
        assert_eq!(saved.failed_login_attempts, Some(1));
    }

    /// A login state saved for a user deleted meanwhile is a no-op: no row is created and no
    /// user is cached. A failure still counts in the copy, for the login delay.
    #[tokio::test]
    async fn test_save_login_state_missing_user_hiqlite() {
        let _lock = HQL_TEST_LOCK.lock().await;
        crate::database::init_test_hiqlite();
        let mut login = User {
            id: "login_missing".to_string(),
            email: "login_missing@example.com".to_string(),
            password: Some("validated".to_string()),
            failed_login_attempts: Some(2),
            ..Default::default()
        };
        login.save_login_failure().await.unwrap();
        assert_eq!(login.failed_login_attempts, Some(3));
        login.save_login_success(Some("rehashed")).await.unwrap();

        let err = User::find(login.id.clone()).await.unwrap_err();
        assert_eq!(err.error, ErrorResponseType::NotFound);
        assert!(
            hql_cached(format!("{IDX_USERS}_{}", login.id))
                .await
                .is_none()
        );
        assert!(
            hql_cached(format!("{IDX_USERS}_{}", login.email))
                .await
                .is_none()
        );
    }

    /// An email changed during the login does not keep the user cached under the new email with
    /// the login state from before.
    #[tokio::test]
    async fn test_save_login_state_after_concurrent_email_change_hiqlite() {
        let _lock = HQL_TEST_LOCK.lock().await;
        crate::database::init_test_hiqlite();
        let user = hql_insert_login_user("login_mail").await;
        let mut login = User::find(user.id.clone()).await.unwrap();

        let mut admin = User::find(user.id.clone()).await.unwrap();
        let new_email = "login_mail_new@example.com".to_string();
        admin.email = new_email.clone();
        admin.save(Some(user.email.clone())).await.unwrap();
        // cache the user under the new email
        User::find_by_email(new_email.clone()).await.unwrap();
        let idx_new = format!("{IDX_USERS}_{new_email}");
        assert!(hql_cached(idx_new.clone()).await.is_some());

        login.save_login_failure().await.unwrap();
        assert!(hql_cached(idx_new.clone()).await.is_none());
        let saved = User::find_by_email(new_email.clone()).await.unwrap();
        assert_eq!(saved.failed_login_attempts, Some(1));

        login.save_login_success(None).await.unwrap();
        assert!(hql_cached(idx_new).await.is_none());
        let saved = User::find_by_email(new_email).await.unwrap();
        assert_eq!(saved.failed_login_attempts, None);
        assert_eq!(saved.last_login, login.last_login);
    }

    /// The Postgres statements of `User::save_login_success` and `User::save_login_failure`
    /// against all Postgres migrations: only the login columns are written, the failed login
    /// counter is incremented in place, a password changed meanwhile is kept, and the stored
    /// email is returned.
    /// Run with `RAUTHY_TEST_PG_URL=postgres://.. cargo test -p rauthy-data -- --ignored`.
    #[tokio::test]
    #[ignore = "needs Postgres via RAUTHY_TEST_PG_URL"]
    async fn test_save_login_state_postgres() {
        let (cl, schema) = crate::database::pg_test_schema("login_state").await;
        cl.batch_execute(
            r#"
INSERT INTO pictures (id, content_type, storage, data) VALUES ('p1', 'image/png', 'db', NULL);
INSERT INTO users (id, email, given_name, password, roles, enabled, email_verified, created_at,
                   last_failed_login, failed_login_attempts, language)
VALUES ('u1', 'u1@example.com', 'u1', 'validated', 'user', true, true, 1, 7, 3, 'en'),
       ('u2', 'u2@example.com', 'u2', 'validated', 'user', true, true, 1, NULL, NULL, 'en');
UPDATE users SET enabled = false, roles = 'admin', email = 'u1_new@example.com',
    picture_id = 'p1' WHERE id = 'u1';
UPDATE users SET password = 'changed' WHERE id = 'u2';"#,
        )
        .await
        .unwrap();
        let row = async |id: &str| {
            cl.query_one(
                "SELECT password, enabled, roles, picture_id, last_login, last_failed_login, \
                failed_login_attempts FROM users WHERE id = $1",
                &[&id],
            )
            .await
            .unwrap()
        };
        let login = |id: &str| User {
            id: id.to_string(),
            email: format!("{id}@example.com"),
            password: Some("validated".to_string()),
            last_login: Some(42),
            last_failed_login: Some(43),
            ..Default::default()
        };
        let assert_kept = |r: &tokio_postgres::Row| {
            assert!(!r.get::<_, bool>("enabled"));
            assert_eq!(r.get::<_, String>("roles"), "admin");
            assert_eq!(
                r.get::<_, Option<String>>("picture_id").as_deref(),
                Some("p1")
            );
        };

        // a failure increments the stored counter, not the one of the copy
        let res = login("u1").pg_save_login_failure(&cl).await.unwrap();
        assert_eq!(res, Some(("u1_new@example.com".to_string(), 4)));
        let r = row("u1").await;
        assert_eq!(r.get::<_, Option<i64>>("last_failed_login"), Some(43));
        assert_eq!(r.get::<_, Option<i64>>("failed_login_attempts"), Some(4));
        assert_eq!(
            r.get::<_, Option<String>>("password").as_deref(),
            Some("validated")
        );
        assert_kept(&r);

        // a success with a rehash: the login columns and the new hash
        let email = login("u1")
            .pg_save_login_success(&cl, Some("rehashed"))
            .await
            .unwrap();
        assert_eq!(email.as_deref(), Some("u1_new@example.com"));
        let r = row("u1").await;
        assert_eq!(
            r.get::<_, Option<String>>("password").as_deref(),
            Some("rehashed")
        );
        assert_eq!(r.get::<_, Option<i64>>("last_login"), Some(42));
        assert_eq!(r.get::<_, Option<i64>>("last_failed_login"), None);
        assert_eq!(r.get::<_, Option<i64>>("failed_login_attempts"), None);
        assert_kept(&r);

        // without a rehash, the password is left alone, also when it is the validated one
        let mut ok = login("u1");
        ok.password = Some("rehashed".to_string());
        ok.last_login = Some(44);
        ok.pg_save_login_success(&cl, None).await.unwrap();
        let r = row("u1").await;
        assert_eq!(
            r.get::<_, Option<String>>("password").as_deref(),
            Some("rehashed")
        );
        assert_eq!(r.get::<_, Option<i64>>("last_login"), Some(44));

        // a failure on a NULL counter starts at 1
        let res = login("u2").pg_save_login_failure(&cl).await.unwrap();
        assert_eq!(res, Some(("u2@example.com".to_string(), 1)));

        // a password changed meanwhile is not overwritten by the rehash
        let email = login("u2")
            .pg_save_login_success(&cl, Some("rehashed"))
            .await
            .unwrap();
        assert_eq!(email.as_deref(), Some("u2@example.com"));
        let r = row("u2").await;
        assert_eq!(
            r.get::<_, Option<String>>("password").as_deref(),
            Some("changed")
        );
        assert_eq!(r.get::<_, Option<i64>>("last_login"), Some(42));
        assert_eq!(r.get::<_, Option<i64>>("failed_login_attempts"), None);

        // a missing user
        assert!(
            login("u3")
                .pg_save_login_success(&cl, Some("rehashed"))
                .await
                .unwrap()
                .is_none()
        );
        assert!(
            login("u3")
                .pg_save_login_failure(&cl)
                .await
                .unwrap()
                .is_none()
        );

        // the full save binds no `picture_id`, which only `set_picture_id` writes, returning
        // the stored email
        assert_eq!(cl.prepare(SQL_SAVE).await.unwrap().params().len(), 18);
        let email = login("u1").pg_set_picture_id(&cl, None).await.unwrap();
        assert_eq!(email.as_deref(), Some("u1_new@example.com"));
        assert_eq!(row("u1").await.get::<_, Option<String>>("picture_id"), None);
        assert!(
            login("u3")
                .pg_set_picture_id(&cl, None)
                .await
                .unwrap()
                .is_none()
        );

        cl.batch_execute(&format!("DROP SCHEMA {schema} CASCADE"))
            .await
            .unwrap();
    }

    /// The Postgres statement of `User::save_password_reset` against all Postgres migrations:
    /// only the password and its expiry are written, and `email_verified` only while the stored
    /// email is still the one of the copy, and the stored email is returned.
    /// Run with `RAUTHY_TEST_PG_URL=postgres://.. cargo test -p rauthy-data -- --ignored`.
    #[tokio::test]
    #[ignore = "needs Postgres via RAUTHY_TEST_PG_URL"]
    async fn test_save_password_reset_postgres() {
        let (cl, schema) = crate::database::pg_test_schema("pwd_reset").await;
        cl.batch_execute(
            r#"
INSERT INTO users (id, email, given_name, password, roles, enabled, email_verified, created_at,
                   language)
VALUES ('u1', 'u1@example.com', 'u1', 'old', 'user', true, false, 1, 'en'),
       ('u3', 'u3@example.com', 'u3', 'old', 'user', true, false, 1, 'en');
UPDATE users SET enabled = false, roles = 'admin', email = 'u1_new@example.com',
    last_failed_login = 7, failed_login_attempts = 3 WHERE id = 'u1';"#,
        )
        .await
        .unwrap();
        let reset = |id: &str| User {
            id: id.to_string(),
            email: format!("{id}@example.com"),
            password: Some("reset".to_string()),
            password_expires: Some(42),
            ..Default::default()
        };

        let stored = reset("u1").pg_save_password_reset(&cl).await.unwrap();
        assert_eq!(
            stored,
            Some(StoredReset {
                email: "u1_new@example.com".to_string(),
                email_verified: false,
                enabled: false,
            })
        );
        let r = cl
            .query_one(
                "SELECT password, password_expires, email_verified, enabled, roles, \
                last_failed_login, failed_login_attempts FROM users WHERE id = 'u1'",
                &[],
            )
            .await
            .unwrap();
        assert_eq!(
            r.get::<_, Option<String>>("password").as_deref(),
            Some("reset")
        );
        assert_eq!(r.get::<_, Option<i64>>("password_expires"), Some(42));
        assert!(!r.get::<_, bool>("email_verified"));
        assert!(!r.get::<_, bool>("enabled"));
        assert_eq!(r.get::<_, String>("roles"), "admin");
        assert_eq!(r.get::<_, Option<i64>>("last_failed_login"), Some(7));
        assert_eq!(r.get::<_, Option<i64>>("failed_login_attempts"), Some(3));

        // the email of the copy is still the stored one
        let stored = reset("u3").pg_save_password_reset(&cl).await.unwrap();
        assert_eq!(
            stored,
            Some(StoredReset {
                email: "u3@example.com".to_string(),
                email_verified: true,
                enabled: true,
            })
        );
        let r = cl
            .query_one(
                "SELECT password, email_verified FROM users WHERE id = 'u3'",
                &[],
            )
            .await
            .unwrap();
        assert_eq!(
            r.get::<_, Option<String>>("password").as_deref(),
            Some("reset")
        );
        assert!(r.get::<_, bool>("email_verified"));

        // a missing user
        assert!(
            reset("u2")
                .pg_save_password_reset(&cl)
                .await
                .unwrap()
                .is_none()
        );

        cl.batch_execute(&format!("DROP SCHEMA {schema} CASCADE"))
            .await
            .unwrap();
    }

    /// The picture removed with a user is the one of the deleted row, not the one of a stale
    /// copy, and it is gone once the delete returned.
    #[tokio::test]
    async fn test_delete_removes_stored_picture_hiqlite() {
        let _lock = HQL_TEST_LOCK.lock().await;
        crate::database::init_test_hiqlite();
        for id in ["del_pic_old", "del_pic_new"] {
            DB::hql()
                .execute(
                    "INSERT INTO pictures (id, content_type, storage, data) \
                    VALUES ($1, 'image/png', 'db', NULL)",
                    params!(id),
                )
                .await
                .unwrap();
        }
        let user = hql_insert_user("del_pic", Some("del_pic_new")).await;
        let mut stale = user.clone();
        stale.picture_id = Some("del_pic_old".to_string());

        stale.delete().await.unwrap();

        let pictures = hql_picture_ids().await;
        assert!(
            pictures.contains(&"del_pic_old".to_string()),
            "{pictures:?}"
        );
        assert!(
            !pictures.contains(&"del_pic_new".to_string()),
            "{pictures:?}"
        );
    }

    /// Inserts for the user `{user_id}` an access token `{user_id}_at`, a refresh token
    /// `{user_id}_rt` and a device refresh token `{user_id}_rtd`, whose access tokens are
    /// `{user_id}_rt_at` and `{user_id}_rtd_at`.
    async fn hql_insert_tokens(user_id: &str) {
        let (at, rt_at, rtd_at) = (
            format!("{user_id}_at"),
            format!("{user_id}_rt_at"),
            format!("{user_id}_rtd_at"),
        );
        DB::hql()
            .txn([
                (
                    "INSERT INTO issued_tokens (jti, user_id, exp) \
                    VALUES ($1, $2, 2), ($3, NULL, 2), ($4, NULL, 2)",
                    params!(at, user_id, rt_at.clone(), rtd_at.clone()),
                ),
                (
                    "INSERT INTO refresh_tokens (id, user_id, nbf, exp, access_token_jti) \
                    VALUES ($1, $2, 1, 2, $3)",
                    params!(format!("{user_id}_rt"), user_id, rt_at),
                ),
                (
                    "INSERT INTO devices (id, client_id, user_id, created, access_exp, peer_ip, name) \
                    VALUES ($1, 'rauthy', $2, 1, 2, '127.0.0.1', 'dev')",
                    params!(format!("{user_id}_dev"), user_id),
                ),
                (
                    "INSERT INTO refresh_tokens_devices \
                    (id, device_id, user_id, nbf, exp, access_token_jti) \
                    VALUES ($1, $2, $3, 1, 2, $4)",
                    params!(
                        format!("{user_id}_rtd"),
                        format!("{user_id}_dev"),
                        user_id,
                        rtd_at
                    ),
                ),
            ])
            .await
            .unwrap()
            .into_iter()
            .for_each(|res| {
                res.unwrap();
            });
    }

    /// The ids of the refresh tokens and device refresh tokens of the user
    async fn hql_refresh_token_ids(user_id: &str) -> Vec<String> {
        DB::hql()
            .query_raw(
                "SELECT id FROM refresh_tokens WHERE user_id = $1 \
                UNION ALL SELECT id FROM refresh_tokens_devices WHERE user_id = $1",
                params!(user_id),
            )
            .await
            .unwrap()
            .into_iter()
            .map(|mut row| row.get("id"))
            .collect()
    }

    async fn hql_revoked(jti: &str) -> bool {
        let mut rows = DB::hql()
            .query_raw(
                "SELECT revoked FROM issued_tokens WHERE jti = $1",
                params!(jti),
            )
            .await
            .unwrap();
        let revoked: Option<i64> = rows.pop().unwrap().get("revoked");
        revoked == Some(1)
    }

    /// The delete revokes every access token of the user, and the access tokens of its refresh
    /// tokens, which the cascade of the user row removes, in the same transaction.
    #[tokio::test]
    async fn test_delete_revokes_access_tokens_hiqlite() {
        let _lock = HQL_TEST_LOCK.lock().await;
        crate::database::init_test_hiqlite();
        let user = hql_insert_user("del_revoke", None).await;
        hql_insert_user("del_revoke_other", None).await;
        DB::hql()
            .txn([
                (
                    "INSERT INTO issued_tokens (jti, user_id, exp) \
                    VALUES ('del_revoke_at', 'del_revoke', 2), \
                    ('del_revoke_rt_at', NULL, 2), \
                    ('del_revoke_rt_client_at', NULL, 2), \
                    ('del_revoke_rtd_at', NULL, 2)",
                    params!(),
                ),
                (
                    "INSERT INTO refresh_tokens \
                    (id, user_id, nbf, exp, access_token_jti, client_id) \
                    VALUES ('del_revoke_rt', 'del_revoke', 1, 2, 'del_revoke_rt_at', NULL), \
                    ('del_revoke_rt_client', 'del_revoke', 1, 2, \
                    'del_revoke_rt_client_at', 'rauthy')",
                    params!(),
                ),
                (
                    "INSERT INTO devices (id, client_id, user_id, created, access_exp, peer_ip, name) \
                    VALUES ('del_revoke_dev', 'rauthy', 'del_revoke', 1, 2, '127.0.0.1', 'dev')",
                    params!(),
                ),
                (
                    "INSERT INTO refresh_tokens_devices \
                    (id, device_id, user_id, nbf, exp, access_token_jti) \
                    VALUES ('del_revoke_rtd', 'del_revoke_dev', 'del_revoke', 1, 2, \
                    'del_revoke_rtd_at')",
                    params!(),
                ),
            ])
            .await
            .unwrap()
            .into_iter()
            .for_each(|res| {
                res.unwrap();
            });

        // tokens of another user, whose access tokens have no `user_id`, as for refresh tokens
        hql_insert_tokens("del_revoke_other").await;

        user.delete().await.unwrap();

        assert!(hql_revoked("del_revoke_at").await);
        assert!(hql_revoked("del_revoke_rt_at").await);
        assert!(hql_revoked("del_revoke_rt_client_at").await);
        assert!(hql_revoked("del_revoke_rtd_at").await);
        assert!(!hql_revoked("del_revoke_other_at").await);
        assert!(!hql_revoked("del_revoke_other_rt_at").await);
        assert!(!hql_revoked("del_revoke_other_rtd_at").await);
        let mut other_rts = hql_refresh_token_ids("del_revoke_other").await;
        other_rts.sort();
        assert_eq!(
            other_rts,
            vec!["del_revoke_other_rt", "del_revoke_other_rtd"]
        );
        let rts = DB::hql()
            .query_raw(
                "SELECT id FROM refresh_tokens WHERE user_id = 'del_revoke' \
                UNION ALL SELECT id FROM refresh_tokens_devices WHERE user_id = 'del_revoke'",
                params!(),
            )
            .await
            .unwrap();
        assert!(rts.is_empty());
    }

    /// Pages through all users with the real batch SQL on an in-memory SQLite, the way
    /// `EmailJob::task_execute` and `ClientScim::sync_users_exec` advance their cursors.
    #[test]
    fn test_batch_cursors_reach_every_user() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.execute_batch(
            r#"
CREATE TABLE users (id TEXT PRIMARY KEY, email TEXT NOT NULL UNIQUE, email_verified INT,
    given_name TEXT, family_name TEXT, roles TEXT, groups TEXT, enabled INT,
    created_at INT NOT NULL, language TEXT, picture_id TEXT);
CREATE TABLE users_values (id TEXT PRIMARY KEY, birthdate TEXT, phone TEXT, street TEXT,
    zip TEXT, city TEXT, country TEXT, preferred_username TEXT, tz TEXT);
INSERT INTO users (id, email, created_at) VALUES
    ('a', 'anna@x', 100), ('b', 'zoe@x', 200), ('c', 'bob@x', 300),
    ('d', 'carl@x', 400), ('e', 'zack@x', 500);
"#,
        )
        .unwrap();

        // email job: cursor is (created_at, email) of the last user in the batch
        let mut cursor = (0i64, String::new());
        let mut reached = Vec::new();
        loop {
            let rows: Vec<(i64, String)> = conn
                .prepare(SQL_FIND_BATCH)
                .unwrap()
                .query_map(rusqlite::params![cursor.0, cursor.1, 2], |r| {
                    Ok((r.get("created_at")?, r.get("email")?))
                })
                .unwrap()
                .map(Result::unwrap)
                .collect();
            if rows.is_empty() {
                break;
            }
            cursor = rows.last().unwrap().clone();
            reached.extend(rows.into_iter().map(|(_, email)| email));
        }
        assert_eq!(reached, ["anna@x", "zoe@x", "bob@x", "carl@x", "zack@x"]);

        // SCIM sync: several users created in the same second, cursor is the last synced user
        conn.execute_batch(
            "DELETE FROM users; INSERT INTO users (id, email, created_at) VALUES
    ('a', 'a@x', 100), ('b', 'b@x', 100), ('c', 'c@x', 100), ('d', 'd@x', 200);",
        )
        .unwrap();
        let mut cursor = (0i64, String::new());
        let mut reached = Vec::new();
        loop {
            let rows: Vec<(i64, String)> = conn
                .prepare(SQL_FIND_FOR_SCIM_SYNC)
                .unwrap()
                .query_map(rusqlite::params![cursor.0, cursor.1, 2], |r| {
                    Ok((r.get("created_at")?, r.get("user_id")?))
                })
                .unwrap()
                .map(Result::unwrap)
                .collect();
            if rows.is_empty() {
                break;
            }
            cursor = rows.last().unwrap().clone();
            reached.extend(rows.into_iter().map(|(_, id)| id));
        }
        assert_eq!(reached, ["a", "b", "c", "d"]);
    }
}
