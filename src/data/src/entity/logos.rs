use crate::database::{Cache, DB};
use crate::entity::auth_providers::AuthProviderTemplate;
use actix_web::web;
use chrono::Utc;
use hiqlite::macros::{FromRow, params};
use image::imageops::FilterType;
use image::{EncodableLayout, ImageFormat};
use rauthy_common::constants::{
    CACHE_TTL_APP, CONTENT_TYPE_WEBP, IDX_AUTH_PROVIDER_LOGO, IDX_CLIENT_LOGO,
};
use rauthy_common::is_hiqlite;
use rauthy_derive::FromPgRow;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use svg_hush::data_url_filter;
use tracing::debug;

// The default height a client logo will be resized to
const RES_CLIENT_LOGO: u32 = 84;
// The favicon size served on authorize pages
const RES_CLIENT_FAVICON: u32 = 32;
// The default height an auth provider logo will be resized to
const RES_PROVIDER_LOGO: u32 = 20;
// The default height for any logo how it will be saved for possible later use
const RES_LATER_USE: u32 = 128;

#[derive(Debug, PartialEq, Serialize, Deserialize, postgres_types::FromSql)]
#[serde(rename_all = "lowercase")]
#[postgres(rename_all = "lowercase")]
pub enum LogoRes {
    Small,
    Medium,
    Large,
    Custom,
    Svg,
    Favicon,
}

impl From<String> for LogoRes {
    fn from(value: String) -> Self {
        match value.as_str() {
            "small" => Self::Small,
            "medium" => Self::Medium,
            "large" => Self::Large,
            "svg" => Self::Svg,
            "favicon" => Self::Favicon,
            _ => Self::Custom,
        }
    }
}

impl LogoRes {
    pub fn as_str(&self) -> &str {
        match self {
            LogoRes::Small => "small",
            LogoRes::Medium => "medium",
            LogoRes::Large => "large",
            LogoRes::Svg => "svg",
            LogoRes::Custom => "custom",
            LogoRes::Favicon => "favicon",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LogoType {
    Client,
    AuthProvider,
}

#[derive(Debug, Serialize, Deserialize, FromRow, FromPgRow)]
pub struct Logo {
    pub id: String,
    #[column(from_string)]
    pub res: LogoRes,
    pub content_type: String,
    pub data: Vec<u8>,
    pub updated: i64,
}

impl Logo {
    pub async fn delete(id: &str, typ: &LogoType) -> Result<(), ErrorResponse> {
        match typ {
            LogoType::Client => {
                let sql = "DELETE FROM client_logos WHERE client_id = $1";
                if is_hiqlite() {
                    DB::hql().execute(sql, params!(id)).await?;
                } else {
                    DB::pg_execute(sql, &[&id]).await?;
                }
            }
            LogoType::AuthProvider => {
                let sql = "DELETE FROM auth_provider_logos WHERE auth_provider_id = $1";
                if is_hiqlite() {
                    DB::hql().execute(sql, params!(id)).await?;
                } else {
                    DB::pg_execute(sql, &[&id]).await?;
                }
            }
        };

        DB::hql()
            .delete(Cache::App, Self::cache_idx(typ, id))
            .await?;
        DB::hql()
            .delete(Cache::App, Self::cache_idx_updated(typ, id))
            .await?;

        Ok(())
    }

    pub async fn delete_client_logo(id: &str) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM client_logos WHERE client_id = $1 AND res != 'favicon'";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(id)).await?;
        } else {
            DB::pg_execute(sql, &[&id]).await?;
        }

        DB::hql()
            .delete(Cache::App, Self::cache_idx(&LogoType::Client, id))
            .await?;
        DB::hql()
            .delete(Cache::App, Self::cache_idx_updated(&LogoType::Client, id))
            .await?;

        Ok(())
    }

    pub async fn delete_res(id: &str, typ: &LogoType, res: LogoRes) -> Result<(), ErrorResponse> {
        let res = res.as_str();
        match typ {
            LogoType::Client => {
                let sql = "DELETE FROM client_logos WHERE client_id = $1 AND res = $2";
                if is_hiqlite() {
                    DB::hql().execute(sql, params!(id, res)).await?;
                } else {
                    DB::pg_execute(sql, &[&id, &res]).await?;
                }
            }
            LogoType::AuthProvider => {
                let sql =
                    "DELETE FROM auth_provider_logos WHERE auth_provider_id = $1 AND res = $2";
                if is_hiqlite() {
                    DB::hql().execute(sql, params!(id, res)).await?;
                } else {
                    DB::pg_execute(sql, &[&id, &res]).await?;
                }
            }
        };

        DB::hql()
            .delete(Cache::App, Self::cache_idx(typ, id))
            .await?;
        DB::hql()
            .delete(Cache::App, Self::cache_idx_updated(typ, id))
            .await?;

        Ok(())
    }

    pub async fn upsert(
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
            "image/svg+xml" => Self::upsert_svg(id, logo, content_type.to_string(), &typ).await,
            "image/jpeg" | "image/png" => Self::upsert_jpg_png(id, logo, typ).await,
            _ => Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Invalid mime type for image asset",
            )),
        }
    }

    pub async fn upsert_with_res(
        id: String,
        logo: Vec<u8>,
        content_type: mime::Mime,
        typ: LogoType,
        res: LogoRes,
    ) -> Result<(), ErrorResponse> {
        if res == LogoRes::Favicon && typ == LogoType::Client {
            match content_type.as_ref() {
                "image/svg+xml" => {
                    Self::upsert_svg_with_res(
                        id,
                        logo,
                        content_type.to_string(),
                        &typ,
                        LogoRes::Favicon,
                    )
                    .await
                }
                "image/jpeg" | "image/png" => Self::upsert_favicon_jpg_png(id, logo, typ).await,
                _ => Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "Invalid mime type for client favicon",
                )),
            }
        } else {
            Self::upsert(id, logo, content_type, typ).await
        }
    }

    async fn upsert_svg(
        id: String,
        logo: Vec<u8>,
        content_type: String,
        typ: &LogoType,
    ) -> Result<(), ErrorResponse> {
        Self::upsert_svg_with_res(id, logo, content_type, typ, LogoRes::Svg).await
    }

    async fn upsert_svg_with_res(
        id: String,
        mut logo: Vec<u8>,
        content_type: String,
        typ: &LogoType,
        res: LogoRes,
    ) -> Result<(), ErrorResponse> {
        // SVG's don't have a resolution, sanitize and validate them before replacing an asset.
        let slf = Self {
            id,
            res,
            content_type,
            data: Self::sanitize_svg(logo.as_mut_slice())?,
            updated: Utc::now().timestamp_millis(),
        };

        match typ {
            LogoType::Client => {
                if slf.res == LogoRes::Favicon {
                    Self::delete_res(&slf.id, typ, LogoRes::Favicon).await?;
                } else {
                    Self::delete_client_logo(&slf.id).await?;
                }
            }
            LogoType::AuthProvider => Self::delete(&slf.id, typ).await?,
        }

        slf.upsert_self(typ, slf.res != LogoRes::Favicon).await
    }

    async fn upsert_jpg_png(id: String, logo: Vec<u8>, typ: LogoType) -> Result<(), ErrorResponse> {
        // we will save jpg / png in 2 downscaled and optimized resolutions:
        // - `RES_LATER_USE`px for possible later use
        // - smaller for the login page
        let size_small = match &typ {
            LogoType::Client => RES_CLIENT_LOGO,
            LogoType::AuthProvider => RES_PROVIDER_LOGO,
        };

        // Decode and resize before deleting the existing asset. Image processing can be expensive,
        // so keep all of it off the main thread.
        let (slf_medium, slf_small) = web::block(move || {
            let img = image::load_from_memory(&logo)?;
            debug!(
                "current logo width: {}, height: {}",
                img.width(),
                img.height()
            );

            if img.height() < size_small {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    format!("size must be at least {size_small} px"),
                ));
            }

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
                id: id.clone(),
                res: logo_res, // will not always be `Medium`, if the given size is smaller than that
                content_type: CONTENT_TYPE_WEBP.to_string(),
                data: buf.into_inner(),
                updated: Utc::now().timestamp_millis(),
            };

            let img_small =
                image_medium.resize_to_fill(size_small, size_small, FilterType::Lanczos3);
            let mut buf = Cursor::new(Vec::with_capacity(8 * 1024));
            img_small.write_to(&mut buf, ImageFormat::WebP)?;
            let slf_small = Self {
                id,
                res: LogoRes::Small,
                content_type: CONTENT_TYPE_WEBP.to_string(),
                data: buf.into_inner(),
                updated: Utc::now().timestamp_millis(),
            };

            Ok::<(Self, Self), ErrorResponse>((slf_medium, slf_small))
        })
        .await??;

        match typ {
            LogoType::Client => Self::delete_client_logo(&slf_medium.id).await?,
            LogoType::AuthProvider => Self::delete(&slf_medium.id, &typ).await?,
        }

        slf_medium.upsert_self(&typ, false).await?;
        slf_small.upsert_self(&typ, true).await
    }

    async fn upsert_favicon_jpg_png(
        id: String,
        logo: Vec<u8>,
        typ: LogoType,
    ) -> Result<(), ErrorResponse> {
        let slf = web::block(move || {
            let img = image::load_from_memory(&logo)?;
            let size_small = RES_CLIENT_FAVICON;

            debug!(
                "current logo width: {}, height: {}",
                img.width(),
                img.height()
            );

            if img.height() < size_small {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    format!("size must be at least {size_small} px"),
                ));
            }

            let img_small = img.resize_to_fill(size_small, size_small, FilterType::Lanczos3);
            let mut buf = Cursor::new(Vec::with_capacity(8 * 1024));
            img_small.write_to(&mut buf, ImageFormat::WebP)?;

            Ok::<Self, ErrorResponse>(Self {
                id,
                res: LogoRes::Favicon,
                content_type: CONTENT_TYPE_WEBP.to_string(),
                data: buf.into_inner(),
                updated: Utc::now().timestamp_millis(),
            })
        })
        .await??;

        Self::delete_res(&slf.id, &typ, LogoRes::Favicon).await?;
        slf.upsert_self(&typ, false).await
    }

    async fn upsert_self(&self, typ: &LogoType, with_cache: bool) -> Result<(), ErrorResponse> {
        let res = self.res.as_str();

        let sql = match typ {
            LogoType::Client => {
                r#"
INSERT INTO client_logos (client_id, res, content_type, data, updated)
VALUES ($1, $2, $3, $4, $5)
ON CONFLICT(client_id, res) DO UPDATE
SET content_type = $3, data = $4, updated = $5"#
            }
            LogoType::AuthProvider => {
                r#"
INSERT INTO auth_provider_logos (auth_provider_id, res, content_type, data, updated)
VALUES ($1, $2, $3, $4, $5)
ON CONFLICT(auth_provider_id, res) DO UPDATE
SET content_type = $3, data = $4, updated = $5"#
            }
        };

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        self.id.clone(),
                        res,
                        self.content_type.clone(),
                        self.data.clone(),
                        self.updated
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &self.id,
                    &res,
                    &self.content_type,
                    &self.data,
                    &self.updated,
                ],
            )
            .await?;
        }

        if with_cache {
            DB::hql()
                .put(
                    Cache::App,
                    Self::cache_idx(typ, &self.id),
                    self,
                    CACHE_TTL_APP,
                )
                .await?;
            DB::hql()
                .put(
                    Cache::App,
                    Self::cache_idx_updated(typ, &self.id),
                    &Some(self.updated),
                    CACHE_TTL_APP,
                )
                .await?;

            if typ == &LogoType::AuthProvider {
                AuthProviderTemplate::update_cache().await?;
            }
        }

        Ok(())
    }

    pub async fn find(id: &str, res: LogoRes, typ: &LogoType) -> Result<Self, ErrorResponse> {
        let res = res.as_str();
        let use_svg_fallback = res != LogoRes::Favicon.as_str();

        let sql = match typ {
            LogoType::Client => {
                if use_svg_fallback {
                    r#"
SELECT client_id AS id, res, content_type, data, updated
FROM client_logos
WHERE client_id = $1 AND (res = $2 OR res = $3)"#
                } else {
                    r#"
SELECT client_id AS id, res, content_type, data, updated
FROM client_logos
WHERE client_id = $1 AND res = $2"#
                }
            }
            LogoType::AuthProvider => {
                if use_svg_fallback {
                    r#"
SELECT auth_provider_id AS id, res, content_type, data, updated
FROM auth_provider_logos
WHERE auth_provider_id = $1 AND (res = $2 OR res = $3)"#
                } else {
                    r#"
SELECT auth_provider_id AS id, res, content_type, data, updated
FROM auth_provider_logos
WHERE auth_provider_id = $1 AND res = $2"#
                }
            }
        };

        let slf = if is_hiqlite() {
            if use_svg_fallback {
                DB::hql()
                    .query_map_one(sql, params!(id, res, LogoRes::Svg.as_str()))
                    .await?
            } else {
                DB::hql().query_map_one(sql, params!(id, res)).await?
            }
        } else if use_svg_fallback {
            DB::pg_query_one(sql, &[&id, &res, &LogoRes::Svg.as_str()]).await?
        } else {
            DB::pg_query_one(sql, &[&id, &res]).await?
        };

        Ok(slf)
    }

    /// special fn because we only want to cache the small logos
    pub async fn find_cached(id: &str, typ: &LogoType) -> Result<Self, ErrorResponse> {
        let client = DB::hql();
        if let Some(slf) = client.get(Cache::App, Self::cache_idx(typ, id)).await? {
            return Ok(slf);
        }

        let slf = Self::find(id, LogoRes::Small, typ).await?;

        client
            .put(Cache::App, Self::cache_idx(typ, id), &slf, CACHE_TTL_APP)
            .await?;

        Ok(slf)
    }

    pub async fn find_updated(id: &str, typ: &LogoType) -> Result<Option<i64>, ErrorResponse> {
        Self::find_updated_with_res(id, LogoRes::Small, typ).await
    }

    pub async fn find_updated_with_res(
        id: &str,
        res: LogoRes,
        typ: &LogoType,
    ) -> Result<Option<i64>, ErrorResponse> {
        let client = DB::hql();
        let cacheable = matches!(typ, LogoType::Client) && matches!(res, LogoRes::Small);
        if cacheable
            && let Some(updated) = client
                .get(Cache::App, Self::cache_idx_updated(typ, id))
                .await?
        {
            return Ok(updated);
        }

        let use_svg_fallback = res != LogoRes::Favicon;
        let sql = match typ {
            LogoType::Client => {
                if use_svg_fallback {
                    "SELECT updated FROM client_logos WHERE client_id = $1 AND (res = $2 OR res = $3)"
                } else {
                    "SELECT updated FROM client_logos WHERE client_id = $1 AND res = $2"
                }
            }
            LogoType::AuthProvider => {
                if use_svg_fallback {
                    "SELECT updated FROM auth_provider_logos WHERE auth_provider_id = $1 AND (res = $2 OR res = $3)"
                } else {
                    "SELECT updated FROM auth_provider_logos WHERE auth_provider_id = $1 AND res = $2"
                }
            }
        };

        let res = res.as_str();
        let updated = if is_hiqlite() {
            if use_svg_fallback {
                DB::hql()
                    .query_raw(sql, params!(id, res, LogoRes::Svg.as_str()))
                    .await?
                    .first_mut()
                    .map(|r| r.get::<i64>("updated"))
            } else {
                DB::hql()
                    .query_raw(sql, params!(id, res))
                    .await?
                    .first_mut()
                    .map(|r| r.get::<i64>("updated"))
            }
        } else {
            if use_svg_fallback {
                DB::pg_query_rows(sql, &[&id, &res, &LogoRes::Svg.as_str()], 1)
                    .await?
                    .first()
                    .map(|r| r.get::<_, i64>("updated"))
            } else {
                DB::pg_query_rows(sql, &[&id, &res], 1)
                    .await?
                    .first()
                    .map(|r| r.get::<_, i64>("updated"))
            }
        };

        if cacheable {
            client
                .put(
                    Cache::App,
                    Self::cache_idx_updated(typ, id),
                    &updated,
                    CACHE_TTL_APP,
                )
                .await?;
        }

        Ok(updated)
    }
}

impl Logo {
    #[inline]
    fn cache_idx(typ: &LogoType, id: &str) -> String {
        match typ {
            LogoType::Client => format!("{IDX_CLIENT_LOGO}_{id}"),
            LogoType::AuthProvider => format!("{IDX_AUTH_PROVIDER_LOGO}_{id}"),
        }
    }

    #[inline]
    fn cache_idx_updated(typ: &LogoType, id: &str) -> String {
        match typ {
            LogoType::Client => format!("{IDX_CLIENT_LOGO}_{id}_updated"),
            LogoType::AuthProvider => format!("{IDX_AUTH_PROVIDER_LOGO}_{id}_updated"),
        }
    }

    pub fn sanitize_svg(source: &mut [u8]) -> Result<Vec<u8>, ErrorResponse> {
        let mut filter = svg_hush::Filter::new();
        filter.set_data_url_filter(data_url_filter::allow_standard_images);

        let mut sanitized = Vec::with_capacity(source.len());
        filter.filter(&mut source.as_bytes(), &mut sanitized)?;

        Ok(sanitized)
    }
}
