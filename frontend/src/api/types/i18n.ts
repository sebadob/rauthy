export type Language = 'en' | 'de' | 'zhhans' | 'ko';

export interface I18nConfigResponse {
    common: Language[],
    admin: Language[],
}