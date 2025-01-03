use rauthy_common::constants::{RE_CLIENT_ID_EPHEMERAL, RE_CSS_VALUE_LOOSE};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
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

impl ThemeCss {
    #[inline]
    pub fn validate(&self) -> Result<(), ErrorResponse> {
        Self::validate_hsl(self.text)?;
        Self::validate_hsl(self.text_high)?;
        Self::validate_hsl(self.bg)?;
        Self::validate_hsl(self.bg_high)?;
        Self::validate_hsl(self.accent)?;
        Self::validate_hsl(self.accent)?;
        Self::validate_hsl(self.error)?;

        ThemeRequestResponse::validate_css_loose(&self.btn_text)?;
        ThemeRequestResponse::validate_css_loose(&self.theme_sun)?;
        ThemeRequestResponse::validate_css_loose(&self.theme_moon)?;

        Ok(())
    }

    #[inline]
    fn validate_hsl(values: [u16; 3]) -> Result<(), ErrorResponse> {
        if values[0] > 360 {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "hue max value is 360",
            ));
        }

        if values[1] > 100 || values[2] > 100 {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "sat|lum max value is 100",
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ThemeRequestResponse {
    pub client_id: String,
    pub light: ThemeCss,
    pub dark: ThemeCss,
    pub border_radius: String,
}

impl ThemeRequestResponse {
    pub fn validate(&self) -> Result<(), ErrorResponse> {
        if !RE_CLIENT_ID_EPHEMERAL.is_match(&self.client_id) {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "client_id must match: ^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,256}$",
            ));
        }

        self.light.validate()?;
        self.dark.validate()?;

        Self::validate_css_loose(&self.border_radius)?;

        Ok(())
    }

    #[inline]
    fn validate_css_loose(value: &str) -> Result<(), ErrorResponse> {
        if !RE_CSS_VALUE_LOOSE.is_match(value) {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "css value must match: ^[a-z0-9-,.()%]+$",
            ));
        }

        Ok(())
    }
}
