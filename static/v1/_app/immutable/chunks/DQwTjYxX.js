import { t as v, a as l } from "./BxmJRzoY.js";
import { p as j, j as A, a1 as V, c as e, s as m, r as i, t as b, a as q, k as z, l as x } from "./w0HvPX0p.js";
import { s as D } from "./BzP2S3Z_.js";
import { i as c } from "./iO9_dPNE.js";
import { s as T } from "./DM69BKKN.js";
import { s as E, b as F, c as G } from "./BdbQ6g_y.js";
import { I as H } from "./BR_xb8zP.js";
import { B as J } from "./C8YTstTD.js";
import { u as K } from "./0cG6LBdy.js";
import { u as M } from "./Bt_9UXew.js";
import { T as N } from "./BnlWDKfj.js";
var O = v('<div class="label font-label svelte-ud9pp2"> </div>'), Q = v('<div class="button svelte-ud9pp2"><!></div>'), R = v('<div class="button svelte-ud9pp2"><!></div>'), S = v('<div class="container svelte-ud9pp2"><!> <div class="flex gap-05"><div><!></div> <!> <!></div></div>');
function lt(h, o) {
  j(o, true);
  let u = K(), y = M(), d = A(V(u.common.copyToClip));
  function C(t) {
    navigator.clipboard.writeText(t), x(d, y.common.copiedToClip, true), setTimeout(() => {
      x(d, u.common.copyToClip, true);
    }, 3e3);
  }
  var n = S(), p = e(n);
  {
    var g = (t) => {
      var a = O(), s = e(a, true);
      i(a), b(() => D(s, o.label)), l(t, a);
    };
    c(p, (t) => {
      o.label && t(g);
    });
  }
  var f = m(p, 2), r = e(f), I = e(r);
  T(I, () => o.children), i(r);
  var _ = m(r, 2);
  {
    var k = (t) => {
      var a = Q(), s = e(a);
      T(s, () => o.button), i(a), l(t, a);
    };
    c(_, (t) => {
      o.button && t(k);
    });
  }
  var w = m(_, 2);
  {
    var B = (t) => {
      var a = R(), s = e(a);
      J(s, { invisible: true, onclick: () => C(o.copyToClip), children: (L, U) => {
        N(L, { get text() {
          return z(d);
        }, children: (P, W) => {
          H(P, { width: "1.25rem" });
        }, $$slots: { default: true } });
      }, $$slots: { default: true } }), i(a), l(t, a);
    };
    c(w, (t) => {
      o.copyToClip && t(B);
    });
  }
  i(f), i(n), b(() => {
    E(r, "title", o.title || o.label || ""), F(r, 1, G(o.mono ? "font-mono" : ""));
  }), l(h, n), q();
}
export {
  lt as L
};
