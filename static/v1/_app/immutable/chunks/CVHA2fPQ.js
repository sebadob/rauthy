import { t as c, a as i, d as M } from "./BH6NCLk-.js";
import { p as _, aa as b, t as S, a as I, f as L, c as y, l as m, j as t, k as B, r as j } from "./CvlvO1XB.js";
import { i as C } from "./BUO_AUgz.js";
import { a as T } from "./BMbqVy6X.js";
import { p as z } from "./Wh68IIk2.js";
import { u as A } from "./BQ1-pLIs.js";
import { B as P } from "./DMkkW5Nn.js";
var V = c(`<div class="icon moon svelte-m6xenu"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M21.752 15.002A9.718 9.718 0 0118 15.75c-5.385 0-9.75-4.365-9.75-9.75
                        0-1.33.266-2.597.748-3.752A9.753 9.753 0 003 11.25C3 16.635 7.365 21 12.75
                        21a9.753 9.753 0 009.002-5.998z"></path></svg></div>`), H = c(`<div class="icon sun svelte-m6xenu"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386l-1.591 1.591M21 12h-2.25m-.386 6.364l-1.591-1.591M12
                        18.75V21m-4.773-4.227l-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75
                        3.75 0 11-7.5 0 3.75 3.75 0 017.5 0z"></path></svg></div>`), q = c("<div><!></div>");
function O(f, v) {
  _(v, true);
  const a = "darkMode";
  let h = A(), e = B(void 0);
  b(() => {
    var _a, _b, _c;
    const s = ((_b = (_a = window == null ? void 0 : window.matchMedia) == null ? void 0 : _a.call(window, "(prefers-color-scheme:dark)")) == null ? void 0 : _b.matches) ?? false;
    if (t(e) === void 0) {
      const n = (_c = window == null ? void 0 : window.localStorage) == null ? void 0 : _c.getItem(a);
      if (n) {
        let l = n === "true";
        t(e) === s && localStorage.removeItem(a), m(e, l);
      } else m(e, z(s));
    } else t(e) ? (document.body.classList.remove("theme-light"), document.body.classList.add("theme-dark")) : (document.body.classList.remove("theme-dark"), document.body.classList.add("theme-light"));
    s === t(e) ? localStorage.removeItem(a) : localStorage.setItem(a, t(e).toString());
  });
  function p() {
    m(e, !t(e));
  }
  var r = q();
  let u;
  var g = y(r);
  P(g, { get ariaLabel() {
    return h.common.changeTheme;
  }, invisible: true, onclick: p, children: (s, n) => {
    var l = M(), w = L(l);
    {
      var k = (o) => {
        var d = V();
        i(o, d);
      }, x = (o) => {
        var d = H();
        i(o, d);
      };
      C(w, (o) => {
        t(e) === true ? o(k) : o(x, false);
      });
    }
    i(s, l);
  }, $$slots: { default: true } }), j(r), S(() => u = T(r, 1, "container svelte-m6xenu", null, u, { absolute: v.absolute })), i(f, r), I();
}
export {
  O as T
};
