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
    // TODO make sure the incompatibility warning is gone after the i18n
    // migration has been finished. Currently, there are some inner types
    // wrapped from the old setup, which are now taken out into root for
    // convenience.
    switch (lang) {
        case 'de':
            _i18n = I18nDe;
            // _i18n = await import('../i18n/de.json');
            break;
        case 'ko':
            _i18n = I18nKo;
            // _i18n = await import('../i18n/ko.json');
            break;
        case 'zh':
            _i18n = I18nZh;
            // _i18n = await import('../i18n/zhhans.json');
            break;
        default:
            _i18n = I18nEn;
            // _i18n = await import('../i18n/en.json');
            break;
    }
    //
    // if (isBrowser()) {
    //     await initI18nBrowser();
    // } else {
    //     await initI18nSsr();
    // }
}

// // It is possible to get into a chicken-and-egg problem here.
// // This may be possible for a first-time-setup after a fresh clone
// // and if for whatever reason, the files in `frontend/src/i18n` are
// // deleted.
// // They are built automatically be the backend with each start to be
// // 100% sure  the types match exactly. If you can't `just build-ui`
// // because these files are missing for you at some point, do a
// // `just run` before. It is likely to fail, because the backend expects
// // the UI static templates being built, but it will execute the
// // build step in any case. So after the failed build command for the
// // backend, you should have these files back.
// //
// // The dynamic import makes it possible to not need to fetch the json
// // in the UI later on in production, but will insert the correct values
// // during local dev and at compilation.
// async function initI18nSsr() {
//     let lang = useLang();
//     // TODO make sure the incompatibility warning is gone after the i18n
//     // migration has been finished. Currently, there are some inner types
//     // wrapped from the old setup, which are now taken out into root for
//     // convenience.
//     switch (lang) {
//         case 'de':
//             _i18n = i18nDe;
//             // _i18n = await import('../i18n/de.json');
//             break;
//         case 'ko':
//             _i18n = i18nKo;
//             // _i18n = await import('../i18n/ko.json');
//             break;
//         case 'zh':
//             _i18n = i18nZh;
//             // _i18n = await import('../i18n/zhhans.json');
//             break;
//         default:
//             _i18n = i18nEn;
//             // _i18n = await import('../i18n/en.json');
//             break;
//     }
// }
//
// async function initI18nBrowser() {
//     let lang = useLang();
//
//     // This will resolve immediately because of a `preload` link inside the html head.
//     let res = await fetchGet<I18n>(`${window.origin}/auth/v1/i18n/${lang}`);
//     if (res.body) {
//         _i18n = res.body;
//     } else {
//         console.error('initI18n error: ', res.error);
//     }
// }
