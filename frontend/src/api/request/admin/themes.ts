export interface ThemeCss {
    text: number[],
    textHigh: number[],
    bg: number[],
    bgHigh: number[],
    action: number[],
    accent: number[],
    error: number[],
    btnText: string,
    themeSun: string,
    themeMoon: string,
}

export interface ThemeRequestResponse {
    clientId: string,
    light: ThemeCss,
    dark: ThemeCss,
    borderRadius: string,
}
