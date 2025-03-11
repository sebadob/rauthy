import { t as _, a as d, d as x } from "./CWf9OOFK.js";
import { p as L, ab as g, c as s, s as S, t as j, a as y, r as i, f as z } from "./nlANaGLT.js";
import { d as A, e as E } from "./BmMHVVX3.js";
import { i as h } from "./DOjUa9u5.js";
import { s as F } from "./BgMuVWsk.js";
import { s as G } from "./CZv_AhHu.js";
import { b as H } from "./CzqnnvDH.js";
import { p as f } from "./uWmgYd3Z.js";
import { u as J } from "./DwvS5LQk.js";
import { B as K } from "./BEQMYyDu.js";
import { I as N } from "./DJ6IvX5N.js";
function O(l, o, t) {
  o() || t();
}
function Q(l) {
  l.stopPropagation();
}
var R = _('<span class="closeIcon svelte-utq0b0"><!></span>'), T = _('<div class="relative"><div class="absolute close svelte-utq0b0"><!></div></div>'), U = _('<dialog aria-modal="true" class="svelte-utq0b0"><div role="none" class="svelte-utq0b0"><!> <!></div></dialog>');
function ia(l, o) {
  L(o, true);
  let t = f(o, "showModal", 15), M = f(o, "closeModal", 15), v = f(o, "strict", 3, false), k = f(o, "prerender", 3, false), q = J(), c;
  g(() => {
    M(m);
  }), g(() => {
    t() && (c == null ? void 0 : c.showModal());
  });
  function m(a) {
    a == null ? void 0 : a.preventDefault(), c == null ? void 0 : c.close(), t(false);
  }
  var e = U();
  e.__click = [O, v, m];
  var p = s(e);
  p.__click = [Q];
  var b = s(p);
  {
    var w = (a) => {
      var r = T(), n = s(r), P = s(n);
      K(P, { get ariaLabel() {
        return q.common.close;
      }, invisible: true, onclick: m, children: (B, V) => {
        var u = R(), D = s(u);
        N(D, { color: "currentColor", width: "1.2rem" }), i(u), d(B, u);
      }, $$slots: { default: true } }), i(n), i(r), d(a, r);
    };
    h(b, (a) => {
      v() || a(w);
    });
  }
  var I = S(b, 2);
  {
    var C = (a) => {
      var r = x(), n = z(r);
      F(n, () => o.children), d(a, r);
    };
    h(I, (a) => {
      (k() || t()) && a(C);
    });
  }
  i(p), i(e), H(e, (a) => c = a, () => c), j(() => G(e, "data-strict", v())), E("close", e, () => {
    var _a;
    t(false), (_a = o.onClose) == null ? void 0 : _a.call(o);
  }), d(l, e), y();
}
A(["click"]);
export {
  ia as M
};
