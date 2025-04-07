use crate::database::{Cache, DB};
use chrono::Utc;
use hiqlite::{Param, params};
use rauthy_api_types::themes::ThemeRequestResponse;
use rauthy_common::compression::{compress_br, compress_gzip};
use rauthy_common::constants::BUILD_TIME;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Row};
use std::fmt::Write;
use tracing::error;

// used for possible future struct updates to be able to deserialize old themes properly
static LATEST_CSS_VERSION: i32 = 1;
static CACHE_KEY_EMAIL_CSS: &str = "css_rauthy_light_email";

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeCssFull {
    pub client_id: String,
    // used for the etag
    pub last_update: i64,
    // Should always be 1 for now, but makes it possible to have different structs
    // in the future for proper deserialization.
    pub version: i64,
    pub light: ThemeCss,
    pub dark: ThemeCss,
    pub border_radius: String,
}

impl From<hiqlite::Row<'_>> for ThemeCssFull {
    fn from(mut row: hiqlite::Row<'_>) -> Self {
        let version: i64 = row.get("version");
        let (light, dark) = if version == 1 {
            let light = ThemeCss::from(row.get::<Vec<u8>>("light").as_slice());
            let dark = ThemeCss::from(row.get::<Vec<u8>>("dark").as_slice());
            (light, dark)
        } else {
            error!(
                "Invalid CSS version {} returned from database, using defaults",
                version
            );
            (ThemeCss::default_light(), ThemeCss::default_dark())
        };

        Self {
            client_id: row.get("client_id"),
            last_update: row.get("last_update"),
            version,
            light,
            dark,
            border_radius: row.get("border_radius"),
        }
    }
}

impl FromRow<'_, sqlx::postgres::PgRow> for ThemeCssFull {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, Error> {
        let version: i32 = row.get("version");
        let (light, dark) = if version == 1 {
            let bytes: Vec<u8> = row.get("light");
            let light = ThemeCss::from(bytes.as_slice());
            let bytes: Vec<u8> = row.get("dark");
            let dark = ThemeCss::from(bytes.as_slice());

            (light, dark)
        } else {
            error!(
                "Invalid CSS version {} returned from database, using defaults",
                version
            );
            (ThemeCss::default_light(), ThemeCss::default_dark())
        };

        Ok(Self {
            client_id: row.get("client_id"),
            last_update: row.get("last_update"),
            version: version as i64,
            light,
            dark,
            border_radius: row.get("border_radius"),
        })
    }
}

impl Default for ThemeCssFull {
    fn default() -> Self {
        Self {
            client_id: "rauthy".to_string(),
            last_update: BUILD_TIME.timestamp(),
            version: 1,
            light: ThemeCss::default_light(),
            dark: ThemeCss::default_dark(),
            border_radius: "5px".to_string(),
        }
    }
}

impl From<ThemeCssFull> for ThemeRequestResponse {
    fn from(t: ThemeCssFull) -> Self {
        Self {
            client_id: t.client_id,
            light: rauthy_api_types::themes::ThemeCss::from(t.light),
            dark: rauthy_api_types::themes::ThemeCss::from(t.dark),
            border_radius: t.border_radius,
        }
    }
}

// CRUD
impl ThemeCssFull {
    pub async fn find(client_id: String) -> Result<Self, ErrorResponse> {
        let slf: Self = if is_hiqlite() {
            DB::hql()
                .query_map_optional(
                    "SELECT * FROM themes WHERE client_id = $1",
                    params!(client_id),
                )
                .await?
        } else {
            sqlx::query("SELECT * FROM themes WHERE client_id = $1")
                .bind(client_id)
                .fetch_optional(DB::conn_sqlx())
                .await?
                .map(|row| Self::from_row(&row).unwrap())
        }
        .unwrap_or_default();

        Ok(slf)
    }

    /// Returns the CSS variables for the light theme to be inserted directly into
    /// an E-Mail `body { }` CSS.
    pub async fn find_theme_variables_email() -> Result<String, ErrorResponse> {
        if let Some(bytes) = DB::hql()
            .get_bytes(Cache::Html, CACHE_KEY_EMAIL_CSS)
            .await?
        {
            let vars = String::from_utf8_lossy(&bytes);
            return Ok(vars.to_string());
        }

        let full = Self::find("rauthy".to_string()).await?;
        let mut vars = String::with_capacity(128);
        full.light.append_css(&mut vars)?;
        write!(vars, "--border-radius:{};", full.border_radius)?;

        DB::hql()
            .put_bytes(
                Cache::Html,
                CACHE_KEY_EMAIL_CSS,
                vars.as_bytes().to_vec(),
                None,
            )
            .await?;

        Ok(vars)
    }

    /// Returns `last_update` for the given `client_id`s Theme CSS. If no custom theme exists,
    /// the latest TS for the `Rauthy` theme is returned.
    #[inline(always)]
    pub async fn find_theme_ts(client_id: String) -> Result<i64, ErrorResponse> {
        Ok(Self::find(client_id).await?.last_update)
    }

    #[inline(always)]
    pub async fn find_theme_ts_rauthy() -> Result<i64, ErrorResponse> {
        Ok(Self::find("rauthy".to_string()).await?.last_update)
    }

    pub async fn delete(client_id: String) -> Result<(), ErrorResponse> {
        let id = client_id.clone();

        if is_hiqlite() {
            DB::hql()
                .execute("DELETE FROM themes WHERE client_id = $1", params!(id))
                .await?;
        } else {
            sqlx::query!("DELETE FROM themes WHERE client_id = $1", id)
                .execute(DB::conn_sqlx())
                .await?;
        }

        if client_id == "rauthy" {
            // To make sure that all admin HTML will be pre-built with the new timestamp,
            // we need to clear all caches to have a more efficient way of caching.
            //
            // No need to rebuild any other HTML, because the theme for clients is fetched
            // only during `/authorize, which is dynamically built each time and not cached.
            DB::hql().clear_cache(Cache::Html).await?;
        }
        Self::invalidate_caches(client_id).await?;

        Ok(())
    }

    pub async fn upsert(
        client_id: String,
        light: ThemeCss,
        dark: ThemeCss,
        border_radius: String,
    ) -> Result<(), ErrorResponse> {
        let id = client_id.clone();
        let now = Utc::now().timestamp();
        let light = light.as_bytes();
        let dark = dark.as_bytes();

        if is_hiqlite() {
            DB::hql()
                .execute(
                    r#"
INSERT INTO themes (client_id, last_update, version, light, dark, border_radius)
VALUES ($1, $2, $3, $4, $5, $6)
ON CONFLICT (client_id) DO UPDATE
SET last_update = $2, version = $3, light = $4, dark = $5, border_radius = $6
"#,
                    params!(id, now, LATEST_CSS_VERSION, light, dark, border_radius),
                )
                .await?;
        } else {
            sqlx::query!(
                r#"
INSERT INTO themes (client_id, last_update, version, light, dark, border_radius)
VALUES ($1, $2, $3, $4, $5, $6)
ON CONFLICT (client_id) DO UPDATE
SET last_update = $2, version = $3, light = $4, dark = $5, border_radius = $6
"#,
                id,
                now,
                LATEST_CSS_VERSION,
                light,
                dark,
                border_radius
            )
            .execute(DB::conn_sqlx())
            .await?;
        }

        if client_id == "rauthy" {
            DB::hql().delete(Cache::Html, CACHE_KEY_EMAIL_CSS).await?;
        }
        // TODO if we have the prebuild fn at some point, favor this instead of invalidation
        Self::invalidate_caches(client_id).await?;

        Ok(())
    }
}

impl ThemeCssFull {
    // TODO fn to pre-build and compress themes for all existing clients at app startup

    // pub async fn etag(client_id: &str) -> Result<String, ErrorResponse> {
    //     if let Some(etag) = DB::client()
    //         .get_bytes(Cache::Etag, Self::cache_key_etag(client_id))
    //         .await?
    //     {
    //         Ok(String::from_utf8(etag)?)
    //     } else {
    //         Self::etag_update(client_id).await
    //     }
    // }

    // #[inline]
    // fn cache_key_etag(client_id: &str) -> String {
    //     format!("{}_theme_etag", client_id)
    // }

    #[inline]
    fn cache_key_plain(client_id: &str) -> String {
        format!("{}_theme_plain", client_id)
    }

    #[inline]
    fn cache_key_br(client_id: &str) -> String {
        format!("{}_theme_br", client_id)
    }

    #[inline]
    fn cache_key_gzip(client_id: &str) -> String {
        format!("{}_theme_gzip", client_id)
    }

    // pub async fn etag_update(client_id: &str) -> Result<String, ErrorResponse> {
    //     let slf = Self::find(client_id.to_string()).await?;
    //     let etag = slf.last_update.to_string();
    //     DB::client()
    //         .put_bytes(
    //             Cache::Etag,
    //             Self::cache_key_etag(client_id),
    //             etag.as_bytes().to_vec(),
    //             None,
    //         )
    //         .await?;
    //     Ok(etag)
    // }

    async fn invalidate_caches(client_id: String) -> Result<(), ErrorResponse> {
        let client = DB::hql();

        client
            .delete(Cache::Html, Self::cache_key_plain(&client_id))
            .await?;
        client
            .delete(Cache::Html, Self::cache_key_br(&client_id))
            .await?;
        client
            .delete(Cache::Html, Self::cache_key_gzip(&client_id))
            .await?;
        client.delete(Cache::ThemeTs, client_id.to_string()).await?;

        Ok(())
    }

    pub async fn plain(client_id: String) -> Result<String, ErrorResponse> {
        // TODO do we even want to cache the plain version?
        // It is probably only used once each time to build the compressed versions anyway.

        // // Note: If only ever using ::default(), don't cache this - building from default
        // // String is always faster than retrieving from cache!
        // if let Some(bytes) = DB::client()
        //     .get_bytes(Cache::Html, Self::cache_key_plain(client_id))
        //     .await?
        // {
        //     let css = String::from_utf8(bytes)?;
        //     return Ok(css);
        // }

        let slf = Self::find(client_id).await?;
        let res = slf.as_css()?;

        // DB::client()
        //     .put_bytes(
        //         Cache::Html,
        //         Self::cache_key_plain(client_id),
        //         res.as_bytes().to_vec(),
        //         None,
        //     )
        //     .await?;

        Ok(res)
    }

    pub async fn br(client_id: &str) -> Result<Vec<u8>, ErrorResponse> {
        if let Some(bytes) = DB::hql()
            .get_bytes(Cache::Html, Self::cache_key_br(client_id))
            .await?
        {
            return Ok(bytes);
        }

        let plain = Self::plain(client_id.to_string()).await?;
        let compressed = compress_br(plain.as_bytes())?;
        DB::hql()
            .put_bytes(
                Cache::Html,
                Self::cache_key_br(client_id),
                compressed.clone(),
                None,
            )
            .await?;

        Ok(compressed)
    }

    pub async fn gzip(client_id: &str) -> Result<Vec<u8>, ErrorResponse> {
        if let Some(bytes) = DB::hql()
            .get_bytes(Cache::Html, Self::cache_key_gzip(client_id))
            .await?
        {
            return Ok(bytes);
        }

        let plain = Self::plain(client_id.to_string()).await?;
        let compressed = compress_gzip(plain.as_bytes())?;
        DB::hql()
            .put_bytes(
                Cache::Html,
                Self::cache_key_gzip(client_id),
                compressed.clone(),
                None,
            )
            .await?;

        Ok(compressed)
    }

    pub fn as_css(&self) -> Result<String, ErrorResponse> {
        let mut s = String::with_capacity(1100);

        write!(s, "body{{")?;
        self.light.append_css(&mut s)?;
        write!(s, "--border-radius:{};", self.border_radius)?;
        write!(s, "}}")?;

        write!(s, ".theme-dark{{")?;
        self.dark.append_css(&mut s)?;
        write!(s, "}}")?;

        write!(s, "@media (prefers-color-scheme: dark){{")?;
        write!(s, "body{{")?;
        self.dark.append_css(&mut s)?;
        write!(s, "}}")?;

        write!(s, ".theme-light{{")?;
        self.light.append_css(&mut s)?;
        write!(s, "}}")?;
        write!(s, "}}")?;

        Ok(s)
    }

    #[inline]
    pub fn as_css_light(&self) -> Result<String, ErrorResponse> {
        let mut s = String::with_capacity(280);

        write!(s, "body{{")?;
        self.light.append_css(&mut s)?;
        write!(s, "--border-radius:{};", self.border_radius)?;
        write!(s, "}}")?;

        Ok(s)
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ThemeCss {
    // HSL values without prefix
    pub text: [u16; 3],
    pub text_high: [u16; 3],
    pub bg: [u16; 3],
    pub bg_high: [u16; 3],
    pub action: [u16; 3],
    pub accent: [u16; 3],
    pub error: [u16; 3],

    // fully valid values
    pub btn_text: String,
    pub theme_sun: String,
    pub theme_moon: String,
}

impl From<rauthy_api_types::themes::ThemeCss> for ThemeCss {
    fn from(value: rauthy_api_types::themes::ThemeCss) -> Self {
        Self {
            text: value.text,
            text_high: value.text_high,
            bg: value.bg,
            bg_high: value.bg_high,
            action: value.action,
            accent: value.accent,
            error: value.error,
            btn_text: value.btn_text,
            theme_sun: value.theme_sun,
            theme_moon: value.theme_moon,
        }
    }
}

impl From<ThemeCss> for rauthy_api_types::themes::ThemeCss {
    fn from(t: ThemeCss) -> Self {
        Self {
            text: t.text,
            text_high: t.text_high,
            bg: t.bg,
            bg_high: t.bg_high,
            action: t.action,
            accent: t.accent,
            error: t.error,
            btn_text: t.btn_text,
            theme_sun: t.theme_sun,
            theme_moon: t.theme_moon,
        }
    }
}

impl From<&[u8]> for ThemeCss {
    fn from(value: &[u8]) -> Self {
        bincode::deserialize(value).unwrap()
    }
}

impl ThemeCss {
    pub fn append_css(&self, s: &mut String) -> Result<(), ErrorResponse> {
        write!(
            s,
            "--text:{} {} {};",
            self.text[0], self.text[1], self.text[2]
        )?;
        write!(
            s,
            "--text-high:{} {} {};",
            self.text_high[0], self.text_high[1], self.text_high[2]
        )?;
        write!(s, "--bg:{} {} {};", self.bg[0], self.bg[1], self.bg[2])?;
        write!(
            s,
            "--bg-high:{} {} {};",
            self.bg_high[0], self.bg_high[1], self.bg_high[2]
        )?;
        write!(
            s,
            "--action:{} {} {};",
            self.action[0], self.action[1], self.action[2]
        )?;
        write!(
            s,
            "--accent:{} {} {};",
            self.accent[0], self.accent[1], self.accent[2]
        )?;
        write!(
            s,
            "--error:{} {} {};",
            self.error[0], self.error[1], self.error[2]
        )?;

        write!(s, "--btn-text:{};", self.btn_text)?;
        write!(s, "--theme-sun:{};", self.theme_sun)?;
        write!(s, "--theme-moon:{};", self.theme_moon)?;

        Ok(())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    // fn default_dark() -> Self {
    //     Self {
    //         text: [228, 2, 70],
    //         text_high: [228, 8, 90],
    //         bg: [208, 90, 4],
    //         bg_high: [208, 30, 19],
    //         action: [34, 100, 59],
    //         accent: [265, 100, 53],
    //         error: [15, 100, 37],
    //         btn_text: "hsl(var(--bg))".to_string(),
    //         theme_sun: "hsla(var(--action) / .7)".to_string(),
    //         theme_moon: "hsla(var(--accent) / .85)".to_string(),
    //     }
    // }

    fn default_dark() -> Self {
        Self {
            text: [34, 5, 75],
            text_high: [34, 7, 90],
            bg: [200, 40, 6],
            bg_high: [200, 20, 17],
            action: [34, 100, 59],
            accent: [265, 100, 53],
            error: [15, 100, 37],
            btn_text: "hsl(var(--bg))".to_string(),
            theme_sun: "hsla(var(--action) / .7)".to_string(),
            theme_moon: "hsla(var(--accent) / .85)".to_string(),
        }
    }

    // fn default_light() -> Self {
    //     Self {
    //         text: [208, 10, 40],
    //         text_high: [208, 20, 20],
    //         bg: [228, 2, 98],
    //         bg_high: [228, 8, 84],
    //         action: [34, 100, 59],
    //         accent: [265, 100, 53],
    //         error: [15, 100, 37],
    //         btn_text: "white".to_string(),
    //         theme_sun: "hsla(var(--action) / .7)".to_string(),
    //         theme_moon: "hsla(var(--accent) / .85)".to_string(),
    //     }
    // }

    fn default_light() -> Self {
        Self {
            text: [200, 5, 37],
            text_high: [200, 15, 25],
            bg: [34, 25, 97],
            bg_high: [34, 20, 90],
            action: [34, 100, 59],
            accent: [265, 100, 53],
            error: [15, 100, 37],
            btn_text: "white".to_string(),
            theme_sun: "hsla(var(--action) / .7)".to_string(),
            theme_moon: "hsla(var(--accent) / .85)".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_fmt() {
        let theme = ThemeCssFull::default();
        theme.as_css().unwrap();
    }
}
