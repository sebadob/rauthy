export type Language = 'en' | 'de' | 'ko' | 'nb' | 'ru' | 'uk' | 'zh';

export interface I18nConfigResponse {
    common: Language[];
    admin: Language[];
}
