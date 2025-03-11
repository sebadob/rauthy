use crate::database::{Cache, DB};
use hiqlite::{Param, params};
use jwt_simple::prelude::{Deserialize, Serialize};
use rauthy_api_types::users::{UserValuesRequest, UserValuesResponse};
use rauthy_common::constants::{CACHE_TTL_USER, IDX_USERS_VALUES};
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserValues {
    pub id: String,
    pub birthdate: Option<String>,
    pub phone: Option<String>,
    pub street: Option<String>,
    pub zip: Option<i32>,
    pub city: Option<String>,
    pub country: Option<String>,
}

impl UserValues {
    #[inline(always)]
    fn cache_idx(user_id: &str) -> String {
        format!("{}_{}", IDX_USERS_VALUES, user_id)
    }

    pub async fn find(user_id: &str) -> Result<Option<Self>, ErrorResponse> {
        let idx = Self::cache_idx(user_id);
        let client = DB::client();

        let opt: Option<Option<Self>> = client.get(Cache::User, &idx).await?;
        if let Some(slf) = opt {
            return Ok(slf);
        }

        let slf = if is_hiqlite() {
            client
                .query_as_optional("SELECT * FROM users_values WHERE id = $1", params!(user_id))
                .await?
        } else {
            sqlx::query_as::<_, Self>("SELECT * FROM users_values WHERE id = $1")
                .bind(user_id)
                .fetch_optional(DB::conn())
                .await?
        };

        client.put(Cache::User, idx, &slf, CACHE_TTL_USER).await?;

        Ok(slf)
    }

    pub async fn upsert(
        user_id: String,
        values: UserValuesRequest,
    ) -> Result<Option<Self>, ErrorResponse> {
        if is_hiqlite() {
            DB::client()
                .execute(
                    r#"
INSERT INTO
users_values (id, birthdate, phone, street, zip, city, country)
VALUES ($1, $2, $3, $4, $5, $6, $7)
ON CONFLICT(id) DO UPDATE
SET birthdate = $2, phone = $3, street = $4, zip = $5, city = $6, country = $7"#,
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
            sqlx::query!(
                r#"
INSERT INTO
users_values (id, birthdate, phone, street, zip, city, country)
VALUES ($1, $2, $3, $4, $5, $6, $7)
ON CONFLICT(id) DO UPDATE
SET birthdate = $2, phone = $3, street = $4, zip = $5, city = $6, country = $7"#,
                user_id,
                values.birthdate,
                values.phone,
                values.street,
                values.zip,
                values.city,
                values.country,
            )
            .execute(DB::conn())
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
        DB::client()
            .put(Cache::User, idx, &slf, CACHE_TTL_USER)
            .await?;

        Ok(slf)
    }

    pub async fn delete(user_id: String) -> Result<(), ErrorResponse> {
        let cache_idx = Self::cache_idx(&user_id);

        if is_hiqlite() {
            DB::client()
                .execute("DELETE FROM users_values WHERE id = $1", params!(user_id))
                .await?;
        } else {
            sqlx::query!("DELETE FROM users_values WHERE id = $1", user_id)
                .execute(DB::conn())
                .await?;
        }

        DB::client().delete(Cache::User, cache_idx).await?;

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
