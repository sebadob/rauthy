use crate::database::{Cache, DB};
use hiqlite_macros::params;
use rauthy_api_types::users::{UserValuesRequest, UserValuesResponse};
use rauthy_common::constants::{CACHE_TTL_USER, IDX_USERS_VALUES};
use rauthy_common::is_hiqlite;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UserValues {
    pub id: String,
    pub birthdate: Option<String>,
    pub phone: Option<String>,
    pub street: Option<String>,
    pub zip: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub preferred_username: Option<String>,
    pub tz: Option<String>,
}

impl From<hiqlite::Row<'_>> for UserValues {
    fn from(mut row: hiqlite::Row<'_>) -> Self {
        Self {
            id: row.get("id"),
            birthdate: row.get("birthdate"),
            phone: row.get("phone"),
            street: row.get("street"),
            zip: row.get("zip"),
            city: row.get("city"),
            country: row.get("country"),
            preferred_username: row.get("preferred_username"),
            tz: row.get("tz"),
        }
    }
}

impl From<tokio_postgres::Row> for UserValues {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            birthdate: row.get("birthdate"),
            phone: row.get("phone"),
            street: row.get("street"),
            zip: row.get("zip"),
            city: row.get("city"),
            country: row.get("country"),
            preferred_username: row.get("preferred_username"),
            tz: row.get("tz"),
        }
    }
}

impl UserValues {
    #[inline(always)]
    fn cache_idx(user_id: &str) -> String {
        format!("{IDX_USERS_VALUES}_{user_id}")
    }

    /// CAUTION: Does also set the `preferred_username`. This should only be used with open registration.
    pub async fn insert(
        user_id: String,
        values: UserValuesRequest,
        preferred_username: Option<String>,
    ) -> Result<(), ErrorResponse> {
        let sql = r#"
INSERT INTO
users_values (id, birthdate, phone, street, zip, city, country, preferred_username, tz)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        user_id,
                        values.birthdate,
                        values.phone,
                        values.street,
                        values.zip,
                        values.city,
                        values.country,
                        preferred_username,
                        values.tz
                    ),
                )
                .await
                .map_err(|err| {
                    let err = ErrorResponse::from(err);
                    if err.message.contains("UNIQUE") {
                        ErrorResponse::new(
                            ErrorResponseType::NotAccepted,
                            "UNIQUE constraint on: 'preferred_username'",
                        )
                    } else {
                        err
                    }
                })?
        } else {
            DB::pg_execute(
                sql,
                &[
                    &user_id,
                    &values.birthdate,
                    &values.phone,
                    &values.street,
                    &values.zip,
                    &values.city,
                    &values.country,
                    &preferred_username,
                    &values.tz,
                ],
            )
            .await
            .map_err(|err| {
                if err.message.contains("UNIQUE") {
                    ErrorResponse::new(
                        ErrorResponseType::NotAccepted,
                        "UNIQUE constraint on: 'preferred_username'",
                    )
                } else {
                    err
                }
            })?
        };

        Ok(())
    }

    pub async fn find(user_id: &str) -> Result<Option<Self>, ErrorResponse> {
        let idx = Self::cache_idx(user_id);
        let client = DB::hql();

        let opt: Option<Option<Self>> = client.get(Cache::User, &idx).await?;
        if let Some(slf) = opt {
            return Ok(slf);
        }

        let sql = "SELECT * FROM users_values WHERE id = $1";
        let slf = if is_hiqlite() {
            client.query_map_optional(sql, params!(user_id)).await?
        } else {
            DB::pg_query_opt(sql, &[&user_id]).await?
        };

        client.put(Cache::User, idx, &slf, CACHE_TTL_USER).await?;

        Ok(slf)
    }

    pub async fn find_preferred_username(user_id: &str) -> Result<Option<String>, ErrorResponse> {
        let idx = Self::cache_idx(user_id);
        let client = DB::hql();

        let opt: Option<Option<Self>> = client.get(Cache::User, &idx).await?;
        if let Some(slf) = opt {
            return Ok(slf.and_then(|s| s.preferred_username));
        }

        let sql = "SELECT preferred_username FROM users_values WHERE id = $1";
        let slf = if is_hiqlite() {
            let mut rows = client.query_raw(sql, params!(user_id)).await?;
            if rows.is_empty() {
                None
            } else {
                rows[0].try_get("preferred_username").ok()
            }
        } else {
            let rows = DB::pg_query_rows(sql, &[&user_id], 0).await?;
            if rows.is_empty() {
                None
            } else {
                rows[0].try_get("preferred_username").ok()
            }
        };

        Ok(slf)
    }

    /// CAUTION: Does NOT update the `preferred_username`, which has dedicated functions for
    /// additional validation.
    pub async fn upsert(
        user_id: String,
        values: UserValuesRequest,
    ) -> Result<Option<Self>, ErrorResponse> {
        let idx = Self::cache_idx(&user_id);
        let sql = r#"
INSERT INTO
users_values (id, birthdate, phone, street, zip, city, country, tz)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
ON CONFLICT(id) DO UPDATE
SET birthdate = $2, phone = $3, street = $4, zip = $5, city = $6, country = $7, tz = $8
RETURNING *"#;

        let values: Self = if is_hiqlite() {
            DB::hql()
                .execute_returning_map_one(
                    sql,
                    params!(
                        user_id,
                        values.birthdate,
                        values.phone,
                        values.street,
                        values.zip,
                        values.city,
                        values.country,
                        values.tz
                    ),
                )
                .await?
        } else {
            DB::pg_query_one(
                sql,
                &[
                    &user_id,
                    &values.birthdate,
                    &values.phone,
                    &values.street,
                    &values.zip,
                    &values.city,
                    &values.country,
                    &values.tz,
                ],
            )
            .await?
        };

        let slf = Some(values);
        DB::hql()
            .put(Cache::User, idx, &slf, CACHE_TTL_USER)
            .await?;

        Ok(slf)
    }

    pub async fn upsert_preferred_username(
        user_id: String,
        preferred_username: String,
    ) -> Result<Option<Self>, ErrorResponse> {
        let idx = Self::cache_idx(&user_id);
        let sql = r#"
INSERT INTO
users_values (id, preferred_username)
VALUES ($1, $2)
ON CONFLICT(id) DO UPDATE
SET preferred_username = $2
RETURNING *"#;

        let values: Self = if is_hiqlite() {
            DB::hql()
                .execute_returning_map_one(sql, params!(user_id, preferred_username))
                .await
                .map_err(|err| {
                    let err = ErrorResponse::from(err);
                    if err.message.contains("UNIQUE") {
                        ErrorResponse::new(
                            ErrorResponseType::NotAccepted,
                            "'preferred_username' is not available",
                        )
                    } else {
                        err
                    }
                })?
        } else {
            DB::pg_query_one(sql, &[&user_id, &preferred_username])
                .await
                .map_err(|err| {
                    if err.message.contains("UNIQUE") {
                        ErrorResponse::new(
                            ErrorResponseType::NotAccepted,
                            "'preferred_username' is not available",
                        )
                    } else {
                        err
                    }
                })?
        };

        let slf = Some(values);
        DB::hql()
            .put(Cache::User, idx, &slf, CACHE_TTL_USER)
            .await?;

        Ok(slf)
    }

    pub async fn delete(user_id: String) -> Result<(), ErrorResponse> {
        let cache_idx = Self::cache_idx(&user_id);
        let sql = "DELETE FROM users_values WHERE id = $1";

        if is_hiqlite() {
            DB::hql().execute(sql, params!(user_id)).await?;
        } else {
            DB::pg_execute(sql, &[&user_id]).await?;
        }

        DB::hql().delete(Cache::User, cache_idx).await?;

        Ok(())
    }

    pub async fn delete_preferred_username(user_id: String) -> Result<(), ErrorResponse> {
        let idx = Self::cache_idx(&user_id);
        let sql = r#"
UPDATE users_values
SET preferred_username = null
WHERE id = $1
RETURNING *"#;

        let values: Self = if is_hiqlite() {
            DB::hql()
                .execute_returning_map_one(sql, params!(user_id))
                .await?
        } else {
            DB::pg_query_one(sql, &[&user_id]).await?
        };

        DB::hql()
            .put(Cache::User, idx, &Some(values), CACHE_TTL_USER)
            .await?;

        Ok(())
    }

    pub async fn validate_preferred_username_free(
        preferred_username: String,
    ) -> Result<(), ErrorResponse> {
        let sql = "SELECT 1 FROM users_values WHERE preferred_username = $1";

        let is_free = if is_hiqlite() {
            DB::hql()
                .query_raw_one(sql, params!(preferred_username))
                .await
                .is_err()
        } else {
            DB::pg_query_one_row(sql, &[&preferred_username])
                .await
                .is_err()
        };

        if is_free {
            Ok(())
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::NotAccepted,
                "UNIQUE constraint on: 'preferred_username'",
            ))
        }
    }
}

impl From<UserValues> for UserValuesResponse {
    fn from(value: UserValues) -> Self {
        Self {
            birthdate: value.birthdate,
            phone: value.phone,
            street: value.street,
            zip: value.zip,
            city: value.city,
            country: value.country,
            preferred_username: value.preferred_username,
            tz: value.tz,
        }
    }
}
