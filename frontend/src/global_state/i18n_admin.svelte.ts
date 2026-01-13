import { useLang } from '$state/language.svelte';
import type { I18nAdmin } from '../i18n/admin/interface.ts';
import { I18nAdminDe } from '../i18n/admin/de';
import { I18nAdminEn } from '../i18n/admin/en';
import { I18nAdminKo } from '../i18n/admin/ko';
import { I18nAdminNb } from '../i18n/admin/nb';
import { I18nAdminUk } from '../i18n/admin/uk';
import { I18nAdminZh } from '../i18n/admin/zh';

// This hack makes typescript happy and is fine as long as
// we `initI18n()` as the very first thing in +layout
let _i18n: I18nAdmin = undefined as any as I18nAdmin;

// IMPORTANT:
//
// It is not possible to set a new value to the returned rune without a page reload!
// Such an API is usually a very bad idea because you can easily mess it up.
// However, in this case we want it like that on purpose.
//
// We only ever set the i18n_email in the initial routing or inside the <LangSelector>.
// A manual page reload is only necessary in the <LangSelector> though which is a
// small price when we never need a `get()` in so many other places where we just read
// the value.
//
// For the same reason, we don't even need a rune here.
export function useI18nAdmin(): I18nAdmin {
    return _i18n;
}

const i18nMap: Record<string, I18nAdmin> = {
    de: I18nAdminDe,
    en: I18nAdminEn,
    ko: I18nAdminKo,
    nb: I18nAdminNb,
    uk: I18nAdminUk,
    zh: I18nAdminZh,
};

export function initI18nAdmin() {
    let lang = useLang();
    _i18n = i18nMap[lang] ?? I18nAdminEn;
}
