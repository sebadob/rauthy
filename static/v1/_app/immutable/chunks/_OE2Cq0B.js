import { t as h, a as c } from "./BH6NCLk-.js";
import { p as F, j as r, k as L, r as m, l as j, t as b, a as C, c as E, a9 as N } from "./CvlvO1XB.js";
import { s as P } from "./CTI4QPiR.js";
import { e as q, i as z } from "./BpWRzPRQ.js";
import { a as A, s as D } from "./BMbqVy6X.js";
import { a as v, B as G } from "./DMkkW5Nn.js";
import { b as H } from "./zosqiMUL.js";
import { p as o } from "./C6SR4G2t.js";
var I = h('<span class="font-label tab svelte-mkcuiv"> </span>'), J = h("<div></div>");
function Y(g, t) {
  F(t, true);
  let i = o(t, "selected", 15, ""), _ = o(t, "borderRadius", 3, "var(--border-radius)"), p = o(t, "center", 3, false), k = o(t, "width", 3, "inherit"), R = o(t, "focusFirst", 15);
  t.tabs.length > 0 && i() === "" && i(t.tabs[0]);
  let f = L(void 0);
  R(n);
  function n(e) {
    var _a;
    let a = (_a = r(f)) == null ? void 0 : _a.getElementsByTagName("button")[e || 0];
    a && a.focus();
  }
  function w(e) {
    n(e === 0 ? t.tabs.length - 1 : e - 1);
  }
  function x(e) {
    e === t.tabs.length - 1 ? n(0) : n(e + 1);
  }
  var s = J();
  let d;
  q(s, 21, () => t.tabs, z, (e, a, u) => {
    const B = N(() => r(a) === i() ? "step" : void 0);
    G(e, { get ariaCurrent() {
      return r(B);
    }, invisible: true, onclick: () => i(r(a)), onLeft: () => w(u), onRight: () => x(u), children: (T, K) => {
      var l = I(), y = E(l, true);
      m(l), b(() => {
        D(l, "data-selected", r(a) === i()), P(y, r(a));
      }), c(T, l);
    }, $$slots: { default: true } });
  }), m(s), H(s, (e) => j(f, e), () => r(f)), b(() => {
    d = A(s, 1, "tabs svelte-mkcuiv", null, d, { center: p() }), v(s, "border-radius", _()), v(s, "width", k());
  }), c(g, s), C();
}
export {
  Y as T
};
