export type Language = 'en' | 'de' | 'fr' | 'ja' | 'ko' | 'nb' | 'nl' | 'ru' | 'uk' | 'zh';

export interface I18nConfigResponse {
    common: Language[];
    admin: Language[];
}
