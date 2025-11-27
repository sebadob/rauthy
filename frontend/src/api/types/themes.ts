export interface ThemeCss {
    /// Validation: 3 HSL values: deg, sat, lum
    text: number[];
    /// Validation: 3 HSL values: deg, sat, lum
    text_high: number[];
    /// Validation: 3 HSL values: deg, sat, lum
    bg: number[];
    /// Validation: 3 HSL values: deg, sat, lum
    bg_high: number[];
    /// Validation: 3 HSL values: deg, sat, lum
    action: number[];
    /// Validation: 3 HSL values: deg, sat, lum
    accent: number[];
    /// Validation: 3 HSL values: deg, sat, lum
    error: number[];
    /// Validation: PATTERN_CSS_VALUE_LOOSE
    btn_text: string;
    /// Validation: PATTERN_CSS_VALUE_LOOSE
    theme_sun: string;
    /// Validation: PATTERN_CSS_VALUE_LOOSE
    theme_moon: string;
}

export interface ThemeRequestResponse {
    client_id: string;
    light: ThemeCss;
    dark: ThemeCss;
    /// Validation: PATTERN_CSS_VALUE_LOOSE
    border_radius: string;
}
