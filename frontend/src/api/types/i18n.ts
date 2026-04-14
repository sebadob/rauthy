export type Language = 'en' | 'de' | 'fr' | 'ko' | 'nb' | 'ru' | 'uk' | 'zh';

export interface I18nConfigResponse {
    common: Language[];
    admin: Language[];
}
