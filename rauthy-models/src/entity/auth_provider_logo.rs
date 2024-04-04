use crate::app_state::AppState;
use crate::entity::auth_provider::AuthProvider;
use actix_web::web;
use image::imageops::FilterType;
use image::ImageFormat;
use jwt_simple::prelude::{Deserialize, Serialize};
use rauthy_common::constants::{
    CACHE_NAME_12HR, CONTENT_TYPE_WEBP, IDX_AUTH_PROVIDER, IDX_AUTH_PROVIDER_LOGO,
};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use redhac::{cache_del, cache_get, cache_get_from, cache_get_value, cache_insert, AckLevel};
use sqlx::{query, query_as};
use std::io::Cursor;
use std::str::FromStr;
use tracing::debug;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AuthProviderLogoRes {
    Small,
    Medium,
    Large,
    Custom,
    Svg,
}

impl From<String> for AuthProviderLogoRes {
    fn from(value: String) -> Self {
        match value.as_str() {
            "s" => Self::Small,
            "m" => Self::Medium,
            "l" => Self::Large,
            "svg" => Self::Svg,
            _ => Self::Custom,
        }
    }
}

impl AuthProviderLogoRes {
    pub fn as_str(&self) -> &str {
        match self {
            AuthProviderLogoRes::Small => "s",
            AuthProviderLogoRes::Medium => "m",
            AuthProviderLogoRes::Large => "l",
            AuthProviderLogoRes::Svg => "svg",
            AuthProviderLogoRes::Custom => "cust",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthProviderLogo {
    pub auth_provider_id: String,
    pub res: AuthProviderLogoRes,
    pub content_type: String,
    pub data: Vec<u8>,
}

impl AuthProviderLogo {
    pub async fn delete(
        data: &web::Data<AppState>,
        auth_provider_id: &str,
    ) -> Result<(), ErrorResponse> {
        query!(
            "DELETE FROM auth_provider_logos WHERE auth_provider_id = $1",
            auth_provider_id
        )
        .execute(&data.db)
        .await?;

        cache_del(
            CACHE_NAME_12HR.to_string(),
            Self::cache_idx(auth_provider_id),
            &data.caches.ha_cache_config,
        )
        .await?;

        Ok(())
    }

    pub async fn upsert(
        data: &web::Data<AppState>,
        id: String,
        logo: Vec<u8>,
        content_type: mime::Mime,
    ) -> Result<(), ErrorResponse> {
        // for an insert, we will do a few steps:
        // - if the content_type is not svg, try to parse it into a DynamicImage
        // - resize down to 256px and 20px height
        // - convert both versions back to bytes
        // - save both resolutions, 256px for possible later use, 20px for login page

        // To make the upsert not fail if a switch between svg and jpg/png happens, we will
        // technically not do an upsert, but actually delete + insert.

        match content_type.as_ref() {
            "image/svg+xml" => Self::upsert_svg(data, id, logo, content_type.to_string()).await,
            "image/jpeg" | "image/png" => {
                Self::upsert_jpg_png(data.clone(), id, logo, content_type.to_string()).await
            }
            _ => Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Invalid mime type for auth provider logo".to_string(),
            )),
        }
    }

    async fn upsert_svg(
        data: &web::Data<AppState>,
        auth_provider_id: String,
        logo: Vec<u8>,
        content_type: String,
    ) -> Result<(), ErrorResponse> {
        Self::delete(data, &auth_provider_id).await?;

        let slf = Self {
            auth_provider_id,
            res: AuthProviderLogoRes::Svg,
            content_type,
            data: logo,
        };
        slf.upsert_self(data, true).await
    }

    async fn upsert_jpg_png(
        data: web::Data<AppState>,
        auth_provider_id: String,
        logo: Vec<u8>,
        content_type: String,
    ) -> Result<(), ErrorResponse> {
        Self::delete(&data, &auth_provider_id).await?;

        // we will save jpg / png in 2 downscaled and optimized resolutions:
        // - 256px for possible later use
        // - 20px for the login page
        let img = image::load_from_memory(&logo)?;
        debug!(
            "current logo width: {}, height: {}",
            img.width(),
            img.height()
        );

        // image resizing can be expensive -> do not block main thread
        web::block(move || async move {
            let image_medium = img.resize_to_fill(256, 256, FilterType::Lanczos3);
            // 256x256 will be ~90 - 95 kB
            let mut buf = Cursor::new(Vec::with_capacity(96 * 1024));
            image_medium.write_to(&mut buf, ImageFormat::WebP)?;
            let slf_medium = Self {
                auth_provider_id,
                res: AuthProviderLogoRes::Medium,
                content_type: CONTENT_TYPE_WEBP.to_string(),
                data: buf.into_inner(),
            };
            slf_medium.upsert_self(&data, false).await?;

            let img_small = image_medium.resize_to_fill(20, 20, FilterType::Lanczos3);
            let mut buf = Cursor::new(Vec::with_capacity(2 * 1024));
            img_small.write_to(&mut buf, ImageFormat::WebP)?;
            Self {
                auth_provider_id: slf_medium.auth_provider_id,
                res: AuthProviderLogoRes::Small,
                content_type: slf_medium.content_type,
                data: buf.into_inner(),
            }
            .upsert_self(&data, true)
            .await?;

            Ok::<(), ErrorResponse>(())
        })
        .await?
        .await?;

        Ok(())
    }

    async fn upsert_self(
        &self,
        data: &web::Data<AppState>,
        with_cache: bool,
    ) -> Result<(), ErrorResponse> {
        let res = self.res.as_str();

        // SVGs don't have a resolution -> just save one version
        #[cfg(feature = "sqlite")]
        query!(
            r#"INSERT OR REPLACE INTO auth_provider_logos (auth_provider_id, res, content_type, data)
            VALUES ($1, $2, $3, $4)"#,
            self.auth_provider_id, res, self.content_type, self.data,
        )
            .execute(&data.db)
            .await?;
        #[cfg(not(feature = "sqlite"))]
        query!(
            r#"INSERT INTO auth_provider_logos (auth_provider_id, res, content_type, data)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT(auth_provider_id, res) DO UPDATE SET content_type = $3, data = $4"#,
            self.auth_provider_id,
            res,
            self.content_type,
            self.data,
        )
        .execute(&data.db)
        .await?;

        if with_cache {
            cache_insert(
                CACHE_NAME_12HR.to_string(),
                Self::cache_idx(&self.auth_provider_id),
                &data.caches.ha_cache_config,
                &self,
                AckLevel::Quorum,
            )
            .await?;
        }

        Ok(())
    }

    pub async fn find(
        data: &web::Data<AppState>,
        id: &str,
        res: AuthProviderLogoRes,
    ) -> Result<Self, ErrorResponse> {
        let res = res.as_str();
        let res_svg = AuthProviderLogoRes::Svg.as_str();
        let slf = query_as!(
            Self,
            r#"SELECT * FROM auth_provider_logos
            WHERE auth_provider_id = $1 AND (res = $2 OR res = $3)"#,
            id,
            res,
            res_svg,
        )
        .fetch_one(&data.db)
        .await?;

        Ok(slf)
    }

    // special fn because we would only cache the small logos
    pub async fn find_cached(data: &web::Data<AppState>, id: &str) -> Result<Self, ErrorResponse> {
        if let Some(res) = cache_get!(
            Self,
            CACHE_NAME_12HR.to_string(),
            Self::cache_idx(id),
            &data.caches.ha_cache_config,
            false
        )
        .await?
        {
            return Ok(res);
        }

        let res_small = AuthProviderLogoRes::Small.as_str();
        let res_svg = AuthProviderLogoRes::Svg.as_str();
        let slf = query_as!(
            Self,
            r#"SELECT * FROM auth_provider_logos
            WHERE auth_provider_id = $1 AND (res = $2 OR res = $3)"#,
            id,
            res_small,
            res_svg,
        )
        .fetch_one(&data.db)
        .await?;

        cache_insert(
            CACHE_NAME_12HR.to_string(),
            Self::cache_idx(id),
            &data.caches.ha_cache_config,
            &slf,
            AckLevel::Quorum,
        )
        .await?;

        Ok(slf)
    }
}

impl AuthProviderLogo {
    fn cache_idx(id: &str) -> String {
        format!("{}_{}", IDX_AUTH_PROVIDER_LOGO, id)
    }
}
