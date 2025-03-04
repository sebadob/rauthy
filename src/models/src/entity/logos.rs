use crate::database::{Cache, DB};
use crate::entity::auth_providers::AuthProviderTemplate;
use actix_web::web;
use chrono::Utc;
use hiqlite::{params, Param, Row};
use image::imageops::FilterType;
use image::{EncodableLayout, ImageFormat};
use jwt_simple::prelude::{Deserialize, Serialize};
use rauthy_common::constants::{
    CACHE_TTL_APP, CONTENT_TYPE_WEBP, IDX_AUTH_PROVIDER_LOGO, IDX_CLIENT_LOGO,
};
use rauthy_common::is_hiqlite;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use sqlx::{query, query_as};
use std::io::Cursor;
use svg_hush::data_url_filter;
use tracing::debug;

// The default height a client logo will be resized to
const RES_CLIENT_LOGO: u32 = 84;
// The default height an auth provider logo will be resized to
const RES_PROVIDER_LOGO: u32 = 20;
// The default height for any logo how it will be saved for possible later use
const RES_LATER_USE: u32 = 128;

#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "varchar")]
#[sqlx(rename_all = "lowercase")]
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
            "small" => Self::Small,
            "medium" => Self::Medium,
            "large" => Self::Large,
            "svg" => Self::Svg,
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
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LogoType {
    Client,
    AuthProvider,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Logo {
    pub id: String,
    pub res: LogoRes,
    pub content_type: String,
    pub data: Vec<u8>,
    pub updated: i64,
}

impl<'r> From<hiqlite::Row<'r>> for Logo {
    fn from(mut row: Row<'r>) -> Self {
        Self {
            id: row.get("id"),
            res: LogoRes::from(row.get::<String>("res")),
            content_type: row.get("content_type"),
            data: row.get("data"),
            updated: row.get("updated"),
        }
    }
}

impl Logo {
    pub async fn delete(id: &str, typ: &LogoType) -> Result<(), ErrorResponse> {
        match typ {
            LogoType::Client => {
                if is_hiqlite() {
                    DB::client()
                        .execute("DELETE FROM client_logos WHERE client_id = $1", params!(id))
                        .await?;
                } else {
                    query!("DELETE FROM client_logos WHERE client_id = $1", id)
                        .execute(DB::conn())
                        .await?;
                }
            }
            LogoType::AuthProvider => {
                if is_hiqlite() {
                    DB::client()
                        .execute(
                            "DELETE FROM auth_provider_logos WHERE auth_provider_id = $1",
                            params!(id),
                        )
                        .await?;
                } else {
                    query!(
                        "DELETE FROM auth_provider_logos WHERE auth_provider_id = $1",
                        id
                    )
                    .execute(DB::conn())
                    .await?;
                }
            }
        };

        DB::client()
            .delete(Cache::App, Self::cache_idx(typ, id))
            .await?;
        DB::client()
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
                "Invalid mime type for auth provider logo",
            )),
        }
    }

    async fn upsert_svg(
        id: String,
        mut logo: Vec<u8>,
        content_type: String,
        typ: &LogoType,
    ) -> Result<(), ErrorResponse> {
        Self::delete(&id, typ).await?;

        // SVG's don't have a resolution, save them as they are
        let slf = Self {
            id,
            res: LogoRes::Svg,
            content_type,
            data: Self::sanitize_svg(logo.as_mut_slice())?,
            updated: Utc::now().timestamp_millis(),
        };
        slf.upsert_self(typ, true).await
    }

    async fn upsert_jpg_png(id: String, logo: Vec<u8>, typ: LogoType) -> Result<(), ErrorResponse> {
        Self::delete(&id, &typ).await?;

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
                id,
                res: logo_res, // will not always be `Medium`, if the given size is smaller than that
                content_type: CONTENT_TYPE_WEBP.to_string(),
                data: buf.into_inner(),
                updated: Utc::now().timestamp_millis(),
            };
            slf_medium.upsert_self(&typ, false).await?;

            let img_small =
                image_medium.resize_to_fill(size_small, size_small, FilterType::Lanczos3);
            let mut buf = Cursor::new(Vec::with_capacity(8 * 1024));
            img_small.write_to(&mut buf, ImageFormat::WebP)?;
            Self {
                id: slf_medium.id,
                res: LogoRes::Small,
                content_type: slf_medium.content_type,
                data: buf.into_inner(),
                updated: Utc::now().timestamp_millis(),
            }
            .upsert_self(&typ, true)
            .await?;

            Ok::<(), ErrorResponse>(())
        })
        .await?
        .await?;

        Ok(())
    }

    async fn upsert_self(&self, typ: &LogoType, with_cache: bool) -> Result<(), ErrorResponse> {
        let res = self.res.as_str();

        if is_hiqlite() {
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

            DB::client()
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
            match typ {
                LogoType::Client => {
                    query!(
                        r#"
INSERT INTO client_logos (client_id, res, content_type, data, updated)
VALUES ($1, $2, $3, $4, $5)
ON CONFLICT(client_id, res) DO UPDATE
SET content_type = $3, data = $4, updated = $5"#,
                        self.id,
                        res,
                        self.content_type,
                        self.data,
                        self.updated
                    )
                }
                LogoType::AuthProvider => {
                    query!(
                        r#"
INSERT INTO auth_provider_logos (auth_provider_id, res, content_type, data, updated)
VALUES ($1, $2, $3, $4, $5)
ON CONFLICT(auth_provider_id, res) DO UPDATE
SET content_type = $3, data = $4, updated = $5"#,
                        self.id,
                        res,
                        self.content_type,
                        self.data,
                        self.updated
                    )
                }
            }
            .execute(DB::conn())
            .await?;
        }

        if with_cache {
            DB::client()
                .put(
                    Cache::App,
                    Self::cache_idx(typ, &self.id),
                    self,
                    CACHE_TTL_APP,
                )
                .await?;
            DB::client()
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
        let res_svg = LogoRes::Svg.as_str();

        let slf = if is_hiqlite() {
            let sql = match typ {
                LogoType::Client => {
                    r#"
SELECT client_id AS id, res, content_type, data, updated
FROM client_logos
WHERE client_id = $1 AND (res = $2 OR res = $3)"#
                }
                LogoType::AuthProvider => {
                    r#"
SELECT auth_provider_id AS id, res, content_type, data, updated
FROM auth_provider_logos
WHERE auth_provider_id = $1 AND (res = $2 OR res = $3)"#
                }
            };
            DB::client()
                .query_map_one(sql, params!(id, res, res_svg))
                .await?
        } else {
            match typ {
                LogoType::Client => {
                    query_as!(
                        Self,
                        r#"
SELECT client_id AS id, res, content_type, data, updated
FROM client_logos
WHERE client_id = $1 AND (res = $2 OR res = $3)"#,
                        id,
                        res,
                        res_svg,
                    )
                    .fetch_one(DB::conn())
                    .await?
                }
                LogoType::AuthProvider => {
                    query_as!(
                        Self,
                        r#"
SELECT auth_provider_id AS id, res, content_type, data, updated
FROM auth_provider_logos
WHERE auth_provider_id = $1 AND (res = $2 OR res = $3)"#,
                        id,
                        res,
                        res_svg,
                    )
                    .fetch_one(DB::conn())
                    .await?
                }
            }
        };

        Ok(slf)
    }

    // special fn because we only want to cache the small logos
    pub async fn find_cached(id: &str, typ: &LogoType) -> Result<Self, ErrorResponse> {
        let client = DB::client();
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
        let client = DB::client();
        if let Some(updated) = client
            .get(Cache::App, Self::cache_idx_updated(typ, id))
            .await?
        {
            return Ok(updated);
        }

        let updated = if is_hiqlite() {
            let sql = match typ {
                LogoType::Client => "SELECT updated FROM client_logos WHERE client_id = $1 LIMIT 1",
                LogoType::AuthProvider => {
                    "SELECT updated FROM auth_provider_logos WHERE auth_provider_id = $1 LIMIT 1"
                }
            };
            DB::client()
                .query_raw(sql, params!(id))
                .await?
                .first_mut()
                .map(|r| r.get::<i64>("updated"))
        } else {
            match typ {
                LogoType::Client => query!(
                    "SELECT updated FROM client_logos WHERE client_id = $1 LIMIT 1",
                    id,
                )
                .fetch_all(DB::conn())
                .await?
                .first()
                .map(|r| r.updated),

                LogoType::AuthProvider => query!(
                    "SELECT updated FROM auth_provider_logos WHERE auth_provider_id = $1 LIMIT 1",
                    id,
                )
                .fetch_all(DB::conn())
                .await?
                .first()
                .map(|r| r.updated),
            }
        };

        client
            .put(
                Cache::App,
                Self::cache_idx_updated(typ, id),
                &updated,
                CACHE_TTL_APP,
            )
            .await?;

        Ok(updated)
    }
}

impl Logo {
    #[inline]
    fn cache_idx(typ: &LogoType, id: &str) -> String {
        match typ {
            LogoType::Client => format!("{}_{}", IDX_CLIENT_LOGO, id),
            LogoType::AuthProvider => format!("{}_{}", IDX_AUTH_PROVIDER_LOGO, id),
        }
    }

    #[inline]
    fn cache_idx_updated(typ: &LogoType, id: &str) -> String {
        match typ {
            LogoType::Client => format!("{}_{}_updated", IDX_CLIENT_LOGO, id),
            LogoType::AuthProvider => format!("{}_{}_updated", IDX_AUTH_PROVIDER_LOGO, id),
        }
    }

    fn sanitize_svg(source: &mut [u8]) -> Result<Vec<u8>, ErrorResponse> {
        let mut filter = svg_hush::Filter::new();
        filter.set_data_url_filter(data_url_filter::allow_standard_images);

        let mut sanitized = Vec::with_capacity(source.len());
        filter.filter(&mut source.as_bytes(), &mut sanitized)?;

        Ok(sanitized)
    }
}
