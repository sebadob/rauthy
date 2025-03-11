import { t as c, a as l, d as _ } from "./BH6NCLk-.js";
import { l as f, j as h, k as b, p as I, aa as M, t as L, a as S, f as y, c as B, r as T } from "./CvlvO1XB.js";
import { i as j } from "./BUO_AUgz.js";
import { a as C } from "./BMbqVy6X.js";
import { u as z } from "./CUqQZdNU.js";
import { B as A } from "./CqH8LXO-.js";
import { p as P } from "./Wh68IIk2.js";
let i = b(void 0);
function V() {
  return { isDark() {
    return h(i);
  }, setIsDark(n) {
    f(i, P(n));
  }, toggle() {
    f(i, !h(i));
  } };
}
var H = c(`<div class="icon moon svelte-m6xenu"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M21.752 15.002A9.718 9.718 0 0118 15.75c-5.385 0-9.75-4.365-9.75-9.75
                        0-1.33.266-2.597.748-3.752A9.753 9.753 0 003 11.25C3 16.635 7.365 21 12.75
                        21a9.753 9.753 0 009.002-5.998z"></path></svg></div>`), q = c(`<div class="icon sun svelte-m6xenu"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386l-1.591 1.591M21 12h-2.25m-.386 6.364l-1.591-1.591M12
                        18.75V21m-4.773-4.227l-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75
                        3.75 0 11-7.5 0 3.75 3.75 0 017.5 0z"></path></svg></div>`), E = c("<div><!></div>");
function R(n, u) {
  I(u, true);
  const a = "darkMode";
  let g = z(), e = V();
  M(() => {
    var _a, _b, _c;
    const s = ((_b = (_a = window == null ? void 0 : window.matchMedia) == null ? void 0 : _a.call(window, "(prefers-color-scheme:dark)")) == null ? void 0 : _b.matches) ?? false;
    if (e.isDark() === void 0) {
      const m = (_c = window == null ? void 0 : window.localStorage) == null ? void 0 : _c.getItem(a);
      if (m) {
        let r = m === "true";
        e.isDark() === s && localStorage.removeItem(a), e.setIsDark(r);
      } else e.setIsDark(s);
    } else e.isDark() ? (document.body.classList.remove("theme-light"), document.body.classList.add("theme-dark")) : (document.body.classList.remove("theme-dark"), document.body.classList.add("theme-light"));
    s === e.isDark() ? localStorage.removeItem(a) : localStorage.setItem(a, "true");
  });
  function k() {
    e.toggle();
  }
  var o = E();
  let v;
  var p = B(o);
  A(p, { get ariaLabel() {
    return g.common.changeTheme;
  }, invisible: true, onclick: k, children: (s, m) => {
    var r = _(), w = y(r);
    {
      var x = (t) => {
        var d = H();
        l(t, d);
      }, D = (t) => {
        var d = q();
        l(t, d);
      };
      j(w, (t) => {
        e.isDark() === true ? t(x) : t(D, false);
      });
    }
    l(s, r);
  }, $$slots: { default: true } }), T(o), L(() => v = C(o, 1, "container svelte-m6xenu", null, v, { absolute: u.absolute })), l(n, o), S();
}
export {
  R as T,
  V as u
};
