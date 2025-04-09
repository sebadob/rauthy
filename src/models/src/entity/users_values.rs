use crate::database::{Cache, DB};
use hiqlite::{Param, params};
use jwt_simple::prelude::{Deserialize, Serialize};
use rauthy_api_types::users::{UserValuesRequest, UserValuesResponse};
use rauthy_common::constants::{CACHE_TTL_USER, IDX_USERS_VALUES};
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserValues {
    pub id: String,
    pub birthdate: Option<String>,
    pub phone: Option<String>,
    pub street: Option<String>,
    pub zip: Option<i32>,
    pub city: Option<String>,
    pub country: Option<String>,
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
        }
    }
}

impl UserValues {
    #[inline(always)]
    fn cache_idx(user_id: &str) -> String {
        format!("{}_{}", IDX_USERS_VALUES, user_id)
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
            client.query_as_optional(sql, params!(user_id)).await?
        } else {
            DB::pg_query_opt(sql, &[&user_id]).await?
        };

        client.put(Cache::User, idx, &slf, CACHE_TTL_USER).await?;

        Ok(slf)
    }

    pub async fn upsert(
        user_id: String,
        values: UserValuesRequest,
    ) -> Result<Option<Self>, ErrorResponse> {
        let sql = r#"
INSERT INTO
users_values (id, birthdate, phone, street, zip, city, country)
VALUES ($1, $2, $3, $4, $5, $6, $7)
ON CONFLICT(id) DO UPDATE
SET birthdate = $2, phone = $3, street = $4, zip = $5, city = $6, country = $7"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        &user_id,
                        &values.birthdate,
                        &values.phone,
                        &values.street,
                        values.zip,
                        &values.city,
                        &values.country
                    ),
                )
                .await?;
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
                ],
            )
            .await?;
        }

        let idx = Self::cache_idx(&user_id);
        let slf = Some(Self {
            id: user_id,
            birthdate: values.birthdate,
            phone: values.phone,
            street: values.street,
            zip: values.zip,
            city: values.city,
            country: values.country,
        });
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
        }
    }
}
