import { t as h, a as m } from "./BxmJRzoY.js";
import { p as F, j as L, k as a, r as b, l as j, t as v, a as C, a4 as E, c as N } from "./w0HvPX0p.js";
import { s as P } from "./BzP2S3Z_.js";
import { e as q, i as z } from "./S81raU5Y.js";
import { b as A, a as D, s as G } from "./BdbQ6g_y.js";
import { b as H } from "./Dgjj26O5.js";
import { p as o } from "./C6GSeq7M.js";
import { B as I } from "./C8YTstTD.js";
var J = h('<span class="font-label tab svelte-mkcuiv"> </span>'), K = h("<div></div>");
function Z(g, e) {
  F(e, true);
  let i = o(e, "selected", 15, ""), _ = o(e, "borderRadius", 3, "var(--border-radius)"), p = o(e, "center", 3, false), k = o(e, "width", 3, "inherit"), R = o(e, "focusFirst", 15);
  e.tabs.length > 0 && i() === "" && i(e.tabs[0]);
  let f = L(void 0);
  R(n);
  function n(t) {
    var _a;
    let s = (_a = a(f)) == null ? void 0 : _a.getElementsByTagName("button")[t || 0];
    s && s.focus();
  }
  function w(t) {
    n(t === 0 ? e.tabs.length - 1 : t - 1);
  }
  function x(t) {
    t === e.tabs.length - 1 ? n(0) : n(t + 1);
  }
  var r = K();
  let u, c;
  q(r, 21, () => e.tabs, z, (t, s, d) => {
    const y = E(() => a(s) === i() ? "step" : void 0);
    I(t, { get ariaCurrent() {
      return a(y);
    }, invisible: true, onclick: () => i(a(s)), onLeft: () => w(d), onRight: () => x(d), children: (B, M) => {
      var l = J(), T = N(l, true);
      b(l), v(() => {
        G(l, "data-selected", a(s) === i()), P(T, a(s));
      }), m(B, l);
    }, $$slots: { default: true } });
  }), b(r), H(r, (t) => j(f, t), () => a(f)), v((t) => {
    u = A(r, 1, "tabs svelte-mkcuiv", null, u, t), c = D(r, "", c, { "border-radius": _(), width: k() });
  }, [() => ({ center: p() })]), m(g, r), C();
}
export {
  Z as T
};
