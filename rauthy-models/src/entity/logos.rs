use crate::app_state::AppState;
use actix_web::web;
use image::imageops::FilterType;
use image::ImageFormat;
use jwt_simple::prelude::{Deserialize, Serialize};
use rauthy_common::constants::{
    CACHE_NAME_12HR, CONTENT_TYPE_WEBP, IDX_AUTH_PROVIDER_LOGO, IDX_CLIENT_LOGO,
};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use redhac::{cache_del, cache_get, cache_get_from, cache_get_value, cache_insert, AckLevel};
use sqlx::{query, query_as};
use std::io::Cursor;
use tracing::debug;

// The default height a client logo will be resized to
const RES_CLIENT_LOGO: u32 = 84;
// The default height an auth provider logo will be resized to
const RES_PROVIDER_LOGO: u32 = 20;
// The default height for any logo how it will be saved for possible later use
const RES_LATER_USE: u32 = 128;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum LogoRes {
    Small,
    Medium,
    Large,
    Custom,
    Svg,
}

impl From<String> for LogoRes {
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

impl LogoRes {
    pub fn as_str(&self) -> &str {
        match self {
            LogoRes::Small => "s",
            LogoRes::Medium => "m",
            LogoRes::Large => "l",
            LogoRes::Svg => "svg",
            LogoRes::Custom => "cust",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LogoType {
    Client,
    AuthProvider,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Logo {
    pub id: String,
    pub res: LogoRes,
    pub content_type: String,
    pub data: Vec<u8>,
}

impl Logo {
    pub async fn delete(
        data: &web::Data<AppState>,
        id: &str,
        typ: &LogoType,
    ) -> Result<(), ErrorResponse> {
        match typ {
            LogoType::Client => {
                query!("DELETE FROM client_logos WHERE client_id = $1", id)
                    .execute(&data.db)
                    .await?
            }
            LogoType::AuthProvider => {
                query!(
                    "DELETE FROM auth_provider_logos WHERE auth_provider_id = $1",
                    id
                )
                .execute(&data.db)
                .await?
            }
        };

        cache_del(
            CACHE_NAME_12HR.to_string(),
            Self::cache_idx(typ, id),
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
        typ: LogoType,
    ) -> Result<(), ErrorResponse> {
        // for an insert, we will do a few steps:
        // - if the content_type is not svg, try to parse it into a DynamicImage
        // - resize down to `RES_LATER_USE`px height and a smaller version depending on `typ`
        // - convert both versions back to bytes
        // - save both resolutions, `RES_LATER_USE`px for possible later use, smaller for login page

        // To make the upsert not fail if a switch between svg and jpg/png happens, we will
        // technically not do an upsert, but actually delete + insert.

        match content_type.as_ref() {
            "image/svg+xml" => {
                Self::upsert_svg(data, id, logo, content_type.to_string(), &typ).await
            }
            "image/jpeg" | "image/png" => Self::upsert_jpg_png(data.clone(), id, logo, typ).await,
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
        typ: &LogoType,
    ) -> Result<(), ErrorResponse> {
        Self::delete(data, &auth_provider_id, &typ).await?;

        // SVG's don't have a resolution, save them as they are
        let slf = Self {
            id: auth_provider_id,
            res: LogoRes::Svg,
            content_type,
            data: logo,
        };
        slf.upsert_self(data, typ, true).await
    }

    async fn upsert_jpg_png(
        data: web::Data<AppState>,
        auth_provider_id: String,
        logo: Vec<u8>,
        typ: LogoType,
    ) -> Result<(), ErrorResponse> {
        Self::delete(&data, &auth_provider_id, &typ).await?;

        // we will save jpg / png in 2 downscaled and optimized resolutions:
        // - `RES_LATER_USE`px for possible later use
        // - smaller for the login page
        let img = image::load_from_memory(&logo)?;
        debug!(
            "current logo width: {}, height: {}",
            img.width(),
            img.height()
        );

        // make sure the image is not too small
        let size_small = match &typ {
            LogoType::Client => RES_CLIENT_LOGO,
            LogoType::AuthProvider => RES_PROVIDER_LOGO,
        };
        if img.height() < size_small {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!("size must be at least {} px", size_small),
            ));
        }

        // image resizing can be expensive -> do not block main thread
        web::block(move || async move {
            let (image_medium, logo_res) = if img.height() < RES_LATER_USE
                && img.width() < RES_LATER_USE
            {
                // if the image is smaller than our later use size, we take it as it is
                (img, LogoRes::Custom)
            } else {
                let img = img.resize_to_fill(RES_LATER_USE, RES_LATER_USE, FilterType::Lanczos3);
                (img, LogoRes::Medium)
            };

            let mut buf = Cursor::new(Vec::with_capacity(48 * 1024));
            image_medium.write_to(&mut buf, ImageFormat::WebP)?;
            let slf_medium = Self {
                id: auth_provider_id,
                res: LogoRes::Medium,
                content_type: CONTENT_TYPE_WEBP.to_string(),
                data: buf.into_inner(),
            };
            slf_medium.upsert_self(&data, &typ, false).await?;

            let img_small =
                image_medium.resize_to_fill(size_small, size_small, FilterType::Lanczos3);
            let mut buf = Cursor::new(Vec::with_capacity(8 * 1024));
            img_small.write_to(&mut buf, ImageFormat::WebP)?;
            Self {
                id: slf_medium.id,
                res: LogoRes::Small,
                content_type: slf_medium.content_type,
                data: buf.into_inner(),
            }
            .upsert_self(&data, &typ, true)
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
        typ: &LogoType,
        with_cache: bool,
    ) -> Result<(), ErrorResponse> {
        let res = self.res.as_str();

        // SVGs don't have a resolution -> just save one version
        #[cfg(feature = "sqlite")]
        match typ {
            LogoType::Client => {
                query!(
                    r#"INSERT OR REPLACE INTO
                    client_logos (client_id, res, content_type, data)
                    VALUES ($1, $2, $3, $4)"#,
                    self.id,
                    res,
                    self.content_type,
                    self.data,
                )
            }
            LogoType::AuthProvider => {
                query!(
                    r#"INSERT OR REPLACE INTO
                    auth_provider_logos (auth_provider_id, res, content_type, data)
                    VALUES ($1, $2, $3, $4)"#,
                    self.id,
                    res,
                    self.content_type,
                    self.data,
                )
            }
        }
        .execute(&data.db)
        .await?;

        #[cfg(not(feature = "sqlite"))]
        match typ {
            LogoType::Client => {
                query!(
                    r#"INSERT INTO client_logos (id, res, content_type, data)
                    VALUES ($1, $2, $3, $4)
                    ON CONFLICT(client_id, res) DO UPDATE SET content_type = $3, data = $4"#,
                    self.id,
                    res,
                    self.content_type,
                    self.data,
                )
            }
            LogoType::AuthProvider => {
                query!(
                    r#"INSERT INTO auth_provider_logos (auth_provider_id, res, content_type, data)
                    VALUES ($1, $2, $3, $4)
                    ON CONFLICT(auth_provider_id, res) DO UPDATE SET content_type = $3, data = $4"#,
                    self.id,
                    res,
                    self.content_type,
                    self.data,
                )
            }
        }
        .execute(&data.db)
        .await?;

        if with_cache {
            cache_insert(
                CACHE_NAME_12HR.to_string(),
                Self::cache_idx(typ, &self.id),
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
        res: LogoRes,
        typ: &LogoType,
    ) -> Result<Self, ErrorResponse> {
        let res = res.as_str();
        let res_svg = LogoRes::Svg.as_str();

        let slf = match typ {
            LogoType::Client => {
                query_as!(
                    Self,
                    r#"SELECT client_id as id, res, content_type, data
                    FROM client_logos
                    WHERE client_id = $1 AND (res = $2 OR res = $3)"#,
                    id,
                    res,
                    res_svg,
                )
                .fetch_one(&data.db)
                .await?
            }
            LogoType::AuthProvider => {
                query_as!(
                    Self,
                    r#"SELECT auth_provider_id as id, res, content_type, data
                    FROM auth_provider_logos
                    WHERE auth_provider_id = $1 AND (res = $2 OR res = $3)"#,
                    id,
                    res,
                    res_svg,
                )
                .fetch_one(&data.db)
                .await?
            }
        };

        Ok(slf)
    }

    // special fn because we would only cache the small logos
    pub async fn find_cached(
        data: &web::Data<AppState>,
        id: &str,
        typ: &LogoType,
    ) -> Result<Self, ErrorResponse> {
        if let Some(res) = cache_get!(
            Self,
            CACHE_NAME_12HR.to_string(),
            Self::cache_idx(typ, id),
            &data.caches.ha_cache_config,
            false
        )
        .await?
        {
            return Ok(res);
        }

        let slf = Self::find(data, id, LogoRes::Small, typ).await?;
        // let res_small = LogoRes::Small.as_str();
        // let res_svg = LogoRes::Svg.as_str();
        // let slf = query_as!(
        //     Self,
        //     r#"SELECT * FROM auth_provider_logos
        //     WHERE auth_provider_id = $1 AND (res = $2 OR res = $3)"#,
        //     id,
        //     res_small,
        //     res_svg,
        // )
        // .fetch_one(&data.db)
        // .await?;

        cache_insert(
            CACHE_NAME_12HR.to_string(),
            Self::cache_idx(typ, id),
            &data.caches.ha_cache_config,
            &slf,
            AckLevel::Quorum,
        )
        .await?;

        Ok(slf)
    }
}

impl Logo {
    fn cache_idx(typ: &LogoType, id: &str) -> String {
        match typ {
            LogoType::Client => format!("{}_{}", IDX_CLIENT_LOGO, id),
            LogoType::AuthProvider => format!("{}_{}", IDX_AUTH_PROVIDER_LOGO, id),
        }
    }
}
