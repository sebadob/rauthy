import { t as v, a as l } from "./YHhP1LbZ.js";
import { p as A, k as V, c as e, s as m, r as i, t as x, a as q, j as z, l as T } from "./Ck6jKiur.js";
import { s as D } from "./tDAaDMC_.js";
import { i as c } from "./7JDmqCCW.js";
import { s as h } from "./9ksWc3Vn.js";
import { s as E, b as F, c as G } from "./BTaFr7HN.js";
import { p } from "./ho0YXExL.js";
import { I as H } from "./DkYC4qk-.js";
import { B as J } from "./Bd2Rvcxs.js";
import { u as K } from "./mN05BXqA.js";
import { u as M } from "./BM7IgWpA.js";
import { T as N } from "./Ct8z9Mup.js";
var O = v('<div class="label font-label svelte-ud9pp2"> </div>'), Q = v('<div class="button svelte-ud9pp2"><!></div>'), R = v('<div class="button svelte-ud9pp2"><!></div>'), S = v('<div class="container svelte-ud9pp2"><!> <div class="flex gap-05"><div><!></div> <!> <!></div></div>');
function vt(y, o) {
  A(o, true);
  let u = K(), C = M(), d = V(p(u.common.copyToClip));
  function g(t) {
    navigator.clipboard.writeText(t), T(d, p(C.common.copiedToClip)), setTimeout(() => {
      T(d, p(u.common.copyToClip));
    }, 3e3);
  }
  var n = S(), f = e(n);
  {
    var I = (t) => {
      var a = O(), s = e(a, true);
      i(a), x(() => D(s, o.label)), l(t, a);
    };
    c(f, (t) => {
      o.label && t(I);
    });
  }
  var _ = m(f, 2), r = e(_), k = e(r);
  h(k, () => o.children), i(r);
  var b = m(r, 2);
  {
    var w = (t) => {
      var a = Q(), s = e(a);
      h(s, () => o.button), i(a), l(t, a);
    };
    c(b, (t) => {
      o.button && t(w);
    });
  }
  var B = m(b, 2);
  {
    var L = (t) => {
      var a = R(), s = e(a);
      J(s, { invisible: true, onclick: () => g(o.copyToClip), children: (P, U) => {
        N(P, { get text() {
          return z(d);
        }, children: (j, W) => {
          H(j, { width: "1.25rem" });
        }, $$slots: { default: true } });
      }, $$slots: { default: true } }), i(a), l(t, a);
    };
    c(B, (t) => {
      o.copyToClip && t(L);
    });
  }
  i(_), i(n), x(() => {
    E(r, "title", o.title || o.label || ""), F(r, 1, G(o.mono ? "font-mono" : ""));
  }), l(y, n), q();
}
export {
  vt as L
};
