export type Language = 'en' | 'de' | 'ko' | 'nb' | 'zh';

export interface I18nConfigResponse {
    common: Language[],
    admin: Language[],
}