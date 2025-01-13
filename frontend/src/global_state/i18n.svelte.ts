// This hack makes typescript happy and is fine as long as
// we `initI18n()` as the very first thing in +layout
import {type I18n} from "$api/response/user/i18n.ts";

// The `undefined as any as I18n` is just a workaround to make TS happy.
// We just want to avoid `undefined` checking everywhere for improved DX.
// This will never be undefined, because we have a `preload` in each HTML
// `head` and we set the i18n immediately as the very first thing in the
// base layout.
let _i18n: I18n = $state(undefined as any as I18n);

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

// export function initI18n(route: string | null) {
//     if (route) {
//         if (route === '/') {
//             _i18n = I18nDe;
//         } else if (route.length < 3) {
//             console.error("route is not '/' and length < 3 -> this should never happen: " + route);
//         } else {
//             let start = route.slice(0, 3);
//             switch (start) {
//                 case '/en':
//                     _i18n = I18nEn;
//                     break;
//                 case '/de':
//                     _i18n = I18nDe;
//                     break;
//                 default:
//                     console.error(`Invalid route start '${start}' - cannot initialize i18n`);
//             }
//         }
//     } else {
//         console.error('route in initI18n() is null!');
//     }
// }
