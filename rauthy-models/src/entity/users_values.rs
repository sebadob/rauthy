use crate::app_state::AppState;
use crate::request::UserValuesRequest;
use actix_web::web;
use jwt_simple::prelude::{Deserialize, Serialize};
use rauthy_common::constants::{CACHE_NAME_USERS, IDX_USERS_VALUES};
use rauthy_common::error_response::ErrorResponse;
use redhac::{cache_get, cache_get_from, cache_get_value, cache_insert, AckLevel};
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
    pub async fn find(
        data: &web::Data<AppState>,
        user_id: &str,
    ) -> Result<Option<Self>, ErrorResponse> {
        let idx = format!("{}_{}", IDX_USERS_VALUES, user_id);
        if let Ok(Some(values)) = cache_get!(
            Option<Self>,
            CACHE_NAME_USERS.to_string(),
            idx.to_string(),
            &data.caches.ha_cache_config,
            false
        )
        .await
        {
            return Ok(values);
        }

        let slf = sqlx::query_as::<_, Self>("SELECT * FROM users_values WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&data.db)
            .await?;

        cache_insert(
            CACHE_NAME_USERS.to_string(),
            idx,
            &data.caches.ha_cache_config,
            &slf,
            AckLevel::Leader,
        )
        .await?;

        Ok(slf)
    }

    pub async fn upsert(
        data: &web::Data<AppState>,
        user_id: String,
        values: UserValuesRequest,
    ) -> Result<Option<Self>, ErrorResponse> {
        #[cfg(feature = "sqlite")]
        let q = sqlx::query!(
            r#"INSERT OR REPLACE INTO
                users_values (id, birthdate, phone, street, zip, city, country)
                VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
            user_id,
            values.birthdate,
            values.phone,
            values.street,
            values.zip,
            values.city,
            values.country,
        );
        #[cfg(not(feature = "sqlite"))]
        let q = sqlx::query!(
            r#"INSERT INTO
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
        );

        q.execute(&data.db).await?;

        let idx = format!("{}_{}", IDX_USERS_VALUES, user_id);
        let slf = Some(Self {
            id: user_id,
            birthdate: values.birthdate,
            phone: values.phone,
            street: values.street,
            zip: values.zip,
            city: values.city,
            country: values.country,
        });
        cache_insert(
            CACHE_NAME_USERS.to_string(),
            idx,
            &data.caches.ha_cache_config,
            &slf,
            AckLevel::Leader,
        )
        .await?;

        Ok(slf)
    }
}
