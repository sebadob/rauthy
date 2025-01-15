import {useLang} from "$state/language.svelte.ts";
import {type I18n} from "../i18n/interface.ts";
import {I18nEn} from "../i18n/en.ts";
import {I18nDe} from "../i18n/de.ts";
import {I18nKo} from "../i18n/ko.ts";
import {I18nZh} from "../i18n/zh.ts";

// This hack makes typescript happy and is fine as long as
// we `initI18n()` as the very first thing in +layout
let _i18n: I18n = undefined as any as I18n;

// IMPORTANT:
//
// It is not possible to set a new value to the returned rune without a page reload!
// Such an API is usually a very bad idea because you can easily mess it up.
// However, in this case we want it like that on purpose.
//
// We only ever set the i18n in the initial routing or inside the <LangSelector>.
// A manual page reload is only necessary in the <LangSelector> though which is a
// small price when we never need a `get()` in so many other places where we just read
// the value.
//
// For the same reason, we don't even need a rune here.
export function useI18n(): I18n {
    return _i18n;
}

export function initI18n() {
    let lang = useLang();
    console.log(lang);
    switch (lang) {
        case 'de':
            _i18n = I18nDe;
            break;
        case 'ko':
            _i18n = I18nKo;
            break;
        case 'zh':
            _i18n = I18nZh;
            break;
        default:
            _i18n = I18nEn;
            break;
    }
}
