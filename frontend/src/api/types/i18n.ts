export type Language =
    | 'en'
    | 'cs'
    | 'de'
    | 'es'
    | 'fr'
    | 'it'
    | 'ja'
    | 'ko'
    | 'nb'
    | 'nl'
    | 'pl'
    | 'pt'
    | 'ru'
    | 'uk'
    | 'zh';

export interface I18nConfigResponse {
    common: Language[];
    admin: Language[];
}
