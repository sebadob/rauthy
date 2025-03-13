export type Language = 'en' | 'de' | 'zh' | 'ko';

export interface I18nConfigResponse {
    common: Language[],
    admin: Language[],
}