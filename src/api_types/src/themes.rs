use rauthy_common::regex::{RE_CLIENT_ID, RE_CSS_VALUE_LOOSE};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ThemeCss {
    /// Validation: 3 HSL values: deg, sat, lum
    pub text: [u16; 3],
    /// Validation: 3 HSL values: deg, sat, lum
    pub text_high: [u16; 3],
    /// Validation: 3 HSL values: deg, sat, lum
    pub bg: [u16; 3],
    /// Validation: 3 HSL values: deg, sat, lum
    pub bg_high: [u16; 3],
    /// Validation: 3 HSL values: deg, sat, lum
    pub action: [u16; 3],
    /// Validation: 3 HSL values: deg, sat, lum
    pub accent: [u16; 3],
    /// Validation: 3 HSL values: deg, sat, lum
    pub error: [u16; 3],

    /// Validation: PATTERN_CSS_VALUE_LOOSE - valid CSS value
    pub btn_text: String,
    /// Validation: PATTERN_CSS_VALUE_LOOSE - valid CSS value
    pub theme_sun: String,
    /// Validation: PATTERN_CSS_VALUE_LOOSE - valid CSS value
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

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ThemeRequestResponse {
    pub client_id: String,
    pub light: ThemeCss,
    pub dark: ThemeCss,
    /// Validation: PATTERN_CSS_VALUE_LOOSE - valid CSS value
    pub border_radius: String,
}

impl ThemeRequestResponse {
    pub fn validate(&self) -> Result<(), ErrorResponse> {
        if !RE_CLIENT_ID.is_match(&self.client_id) {
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
                "css value must match: ^[a-z0-9-,.#()%/\\s]+$",
            ));
        }

        Ok(())
    }
}
