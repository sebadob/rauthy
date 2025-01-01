use crate::database::{Cache, DB};
use chrono::Utc;
use rauthy_common::compression::{compress_br, compress_gzip};
use rauthy_error::ErrorResponse;
use std::fmt::Write;

#[derive(Debug)]
pub struct ThemeCssFull {
    pub ts: i64,
    pub light: ThemeCss,
    pub dark: ThemeCss,
    pub border_radius: String,
}

impl Default for ThemeCssFull {
    fn default() -> Self {
        Self {
            ts: Utc::now().timestamp(), // TODO change to proper TS when using DB
            light: ThemeCss {
                text: vec![208, 10, 40],
                text_high: vec![208, 20, 20],
                bg: vec![228, 2, 98],
                bg_high: vec![228, 8, 84],
                action: vec![34, 100, 59],
                accent: vec![246, 60, 53],
                error: vec![15, 100, 37],
                btn_text: "white".to_string(),
                theme_sun: "hsla(var(--action), .7)".to_string(),
                theme_moon: "hsla(var(--accent), .85)".to_string(),
            },
            dark: ThemeCss {
                text: vec![228, 2, 70],
                text_high: vec![228, 8, 90],
                bg: vec![208, 90, 4],
                bg_high: vec![208, 30, 19],
                action: vec![34, 100, 59],
                accent: vec![246, 60, 53],
                error: vec![15, 100, 37],
                btn_text: "hsl(var(--bg))".to_string(),
                theme_sun: "hsla(var(--action), .7)".to_string(),
                theme_moon: "hsla(var(--accent), .85)".to_string(),
            },
            border_radius: "5px".to_string(),
        }
    }
}

impl ThemeCssFull {
    // TODO fn to pre-build and compress themes for all existing clients at app startup

    pub async fn etag(client_id: &str) -> Result<String, ErrorResponse> {
        if let Some(etag) = DB::client()
            .get_bytes(Cache::Etag, Self::cache_key_etag(client_id))
            .await?
        {
            Ok(String::from_utf8(etag)?)
        } else {
            Self::etag_update(client_id).await
        }
    }

    #[inline]
    fn cache_key_etag(client_id: &str) -> String {
        format!("{}_theme_etag", client_id)
    }

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

    pub async fn etag_update(client_id: &str) -> Result<String, ErrorResponse> {
        // TODO get "real" etag from DB in that case to avoid re-fetches between restarts.
        // timestamp is just a placeholder for now
        let etag = Utc::now().timestamp().to_string();
        DB::client()
            .put_bytes(
                Cache::Etag,
                Self::cache_key_etag(client_id),
                etag.as_bytes().to_vec(),
                None,
            )
            .await?;
        Ok(etag)
    }

    async fn invalidate_caches(client_id: &str) -> Result<(), ErrorResponse> {
        let client = DB::client();

        client
            .delete(Cache::Etag, Self::cache_key_etag(client_id))
            .await?;
        client
            .delete(Cache::Html, Self::cache_key_plain(client_id))
            .await?;
        client
            .delete(Cache::Html, Self::cache_key_br(client_id))
            .await?;
        client
            .delete(Cache::Html, Self::cache_key_gzip(client_id))
            .await?;

        Ok(())
    }

    pub async fn plain(client_id: &str) -> Result<String, ErrorResponse> {
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

        // TODO update to proper DB query once the theme is dynamic
        let res = Self::default().as_css()?;

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
        if let Some(bytes) = DB::client()
            .get_bytes(Cache::Html, Self::cache_key_br(client_id))
            .await?
        {
            return Ok(bytes);
        }

        let plain = Self::plain(client_id).await?;
        let compressed = compress_br(plain.as_bytes())?;
        DB::client()
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
        if let Some(bytes) = DB::client()
            .get_bytes(Cache::Html, Self::cache_key_gzip(client_id))
            .await?
        {
            return Ok(bytes);
        }

        let plain = Self::plain(client_id).await?;
        let compressed = compress_gzip(plain.as_bytes())?;
        DB::client()
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

#[derive(Debug)]
pub struct ThemeCss {
    // HSL values without prefix
    pub text: Vec<u16>,
    pub text_high: Vec<u16>,
    pub bg: Vec<u16>,
    pub bg_high: Vec<u16>,
    pub action: Vec<u16>,
    pub accent: Vec<u16>,
    pub error: Vec<u16>,

    // fully valid values
    pub btn_text: String,
    pub theme_sun: String,
    pub theme_moon: String,
}

impl ThemeCss {
    pub fn append_css(&self, s: &mut String) -> Result<(), ErrorResponse> {
        write!(
            s,
            "--text:{},{}%,{}%;",
            self.text[0], self.text[1], self.text[2]
        )?;
        write!(
            s,
            "--text-high:{},{}%,{}%;",
            self.text_high[0], self.text_high[1], self.text_high[2]
        )?;
        write!(s, "--bg:{},{}%,{}%;", self.bg[0], self.bg[1], self.bg[2])?;
        write!(
            s,
            "--bg-high:{},{}%,{}%;",
            self.bg_high[0], self.bg_high[1], self.bg_high[2]
        )?;
        write!(
            s,
            "--action:{},{}%,{}%;",
            self.action[0], self.action[1], self.action[2]
        )?;
        write!(
            s,
            "--accent:{},{}%,{}%;",
            self.accent[0], self.accent[1], self.accent[2]
        )?;
        write!(
            s,
            "--error:{},{}%,{}%;",
            self.error[0], self.error[1], self.error[2]
        )?;

        write!(s, "--btn-text:{};", self.btn_text)?;
        write!(s, "--theme-sun:{};", self.theme_sun)?;
        write!(s, "--theme-moon:{};", self.theme_moon)?;

        Ok(())
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
