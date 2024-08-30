use crate::app_state::AppState;
use crate::cache::{Cache, DB};
use actix_web::web;
use image::imageops::FilterType;
use image::ImageFormat;
use jwt_simple::prelude::{Deserialize, Serialize};
use rauthy_common::constants::{
    CACHE_TTL_APP, CONTENT_TYPE_WEBP, IDX_AUTH_PROVIDER_LOGO, IDX_CLIENT_LOGO,
};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use sqlx::{query, query_as};
use std::io::Cursor;
use tracing::debug;

// The default height a client logo will be resized to
const RES_CLIENT_LOGO: u32 = 84;
// The default height an auth provider logo will be resized to
const RES_PROVIDER_LOGO: u32 = 20;
// The default height for any logo how it will be saved for possible later use
const RES_LATER_USE: u32 = 128;

const RAUTHY_DEFAULT_SVG: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg width="100%" height="100%" viewBox="0 0 512 138" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" xml:space="preserve" xmlns:serif="http://www.serif.com/" style="fill-rule:evenodd;clip-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:1.5;">
    <g transform="matrix(1,0,0,1,0,-11)">
        <g transform="matrix(1,0,0,1,0,-176)">
            <g transform="matrix(0.920325,0,0,1.84151,45.9279,26.459)">
                <rect x="27.741" y="151.57" width="200.517" height="10.148" style="fill:rgb(4,7,11);"/>
            </g>
            <g transform="matrix(1.93472,0,0,1.82732,8.35618,28.7533)">
                <rect x="33.307" y="97.15" width="94.693" height="54.42" style="fill:rgb(4,7,11);stroke:rgb(4,7,11);stroke-width:1.06px;"/>
            </g>
            <g transform="matrix(1.82732,0,0,1.82732,-160.822,70.1806)">
                <g transform="matrix(72,0,0,72,227.174,123.417)">
                </g>
                <text x="128.982px" y="123.417px" style="font-family:'Calibri-Bold', 'Calibri', sans-serif;font-weight:700;font-size:72px;fill:white;">r<tspan x="152.994px 188.537px " y="123.417px 123.417px ">au</tspan></text>
            </g>
            <g transform="matrix(1,0,0,1.01617,-1.42109e-14,-5.24492)">
                <path d="M440.936,322.643L439.204,324.266L255.482,324.266L255.482,305.721L440.936,305.721L440.936,322.643Z" style="fill:url(#_Linear1);"/>
            </g>
            <g transform="matrix(0.920191,0,0,1.84121,46.2464,-91.3383)">
                <rect x="27.741" y="151.57" width="200.517" height="10.148" style="fill:url(#_Linear2);"/>
            </g>
            <g transform="matrix(1.97598,0,0,1.84619,190.187,26.062)">
                <rect x="33.307" y="97.15" width="94.693" height="54.42" style="fill:rgb(43,65,107);"/>
            </g>
            <path d="M439.204,187.734L440.557,189.007L440.557,206.279L256,206.279L256,187.734L439.204,187.734Z" style="fill:rgb(43,65,107);"/>
            <g transform="matrix(1.82732,0,0,1.82732,-154.661,70.1806)">
                <g transform="matrix(72,0,0,72,323.045,123.417)">
                </g>
                <text x="226.646px" y="123.417px" style="font-family:'Calibri-Bold', 'Calibri', sans-serif;font-weight:700;font-size:72px;fill:white;">th<tspan x="288.943px " y="123.417px ">y</tspan></text>
            </g>
            <g transform="matrix(2,0,0,2,0,0)">
                <path d="M219.602,93.867L256,128L219.602,162.133L219.602,93.867Z" style="fill:rgb(43,65,107);"/>
            </g>
            <g transform="matrix(2,0,0,1.95739,0,3.99997)">
                <path d="M36.398,93.867L0,93.867L35.908,128.524L0,163.619L36.398,163.619" style="fill:rgb(4,7,11);"/>
            </g>
        </g>
    </g>
    <defs>
        <linearGradient id="_Linear1" x1="0" y1="0" x2="1" y2="0" gradientUnits="userSpaceOnUse" gradientTransform="matrix(185.454,0,0,18.5443,255.482,314.994)"><stop offset="0" style="stop-color:rgb(4,7,11);stop-opacity:1"/><stop offset="1" style="stop-color:rgb(43,65,107);stop-opacity:1"/></linearGradient>
        <linearGradient id="_Linear2" x1="0" y1="0" x2="1" y2="0" gradientUnits="userSpaceOnUse" gradientTransform="matrix(200.517,0,0,10.1483,27.7414,156.645)"><stop offset="0" style="stop-color:rgb(4,7,11);stop-opacity:1"/><stop offset="1" style="stop-color:rgb(43,65,107);stop-opacity:1"/></linearGradient>
    </defs>
</svg>"#;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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

        DB::client()
            .delete(Cache::App, Self::cache_idx(typ, id))
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
                "Invalid mime type for auth provider logo",
            )),
        }
    }

    async fn upsert_svg(
        data: &web::Data<AppState>,
        id: String,
        logo: Vec<u8>,
        content_type: String,
        typ: &LogoType,
    ) -> Result<(), ErrorResponse> {
        Self::delete(data, &id, typ).await?;

        // SVG's don't have a resolution, save them as they are
        let slf = Self {
            id,
            res: LogoRes::Svg,
            content_type,
            data: logo,
        };
        slf.upsert_self(data, typ, true).await
    }

    async fn upsert_jpg_png(
        data: web::Data<AppState>,
        id: String,
        logo: Vec<u8>,
        typ: LogoType,
    ) -> Result<(), ErrorResponse> {
        Self::delete(&data, &id, &typ).await?;

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

    /// Overwrites the logo for the `rauthy` client with the default logo
    pub async fn upsert_rauthy_default(data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        // make sure to delete any possibly existing webp image before inserting the svg
        Self::delete(data, "rauthy", &LogoType::Client).await?;

        Self {
            id: "rauthy".to_string(),
            res: LogoRes::Svg,
            content_type: mime::IMAGE_SVG.to_string(),
            data: RAUTHY_DEFAULT_SVG.as_bytes().to_vec(),
        }
        .upsert_self(data, &LogoType::Client, true)
        .await
    }

    async fn upsert_self(
        &self,
        data: &web::Data<AppState>,
        typ: &LogoType,
        with_cache: bool,
    ) -> Result<(), ErrorResponse> {
        let res = self.res.as_str();

        // SVGs don't have a resolution -> just save one version
        #[cfg(not(feature = "postgres"))]
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

        #[cfg(feature = "postgres")]
        match typ {
            LogoType::Client => {
                query!(
                    r#"INSERT INTO client_logos (client_id, res, content_type, data)
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
            DB::client()
                .put(
                    Cache::App,
                    Self::cache_idx(typ, &self.id),
                    self,
                    CACHE_TTL_APP,
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
        let client = DB::client();
        if let Some(slf) = client.get(Cache::App, Self::cache_idx(typ, id)).await? {
            return Ok(slf);
        }

        let slf = Self::find(data, id, LogoRes::Small, typ).await?;

        client
            .put(Cache::App, Self::cache_idx(typ, id), &slf, CACHE_TTL_APP)
            .await?;

        Ok(slf)
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
}
