import { t as g, a as p } from "./BxmJRzoY.js";
import { p as ae, a1 as R, F as M, j as N, a5 as w, l as q, k as s, c as l, a4 as E, r as o, t as h, s as k, a as ie } from "./w0HvPX0p.js";
import { s as B } from "./BzP2S3Z_.js";
import { i as se } from "./iO9_dPNE.js";
import { e as oe, i as re } from "./S81raU5Y.js";
import { s as f } from "./BdbQ6g_y.js";
import { p as x } from "./C6GSeq7M.js";
import { I as G } from "./Ck-A6HWf.js";
import { u as le } from "./N6FgGI8m.js";
import { B as I } from "./C4AV2CoD.js";
import { O as ne } from "./BgkEPAwa.js";
var ve = g('<div class="link noselect svelte-u1kdp4"> </div>'), de = g('<li class="svelte-u1kdp4"><!></li>'), ue = g('<nav class="svelte-u1kdp4"><ul class="svelte-u1kdp4"></ul></nav>'), ce = g('<div class="flex gap-10 svelte-u1kdp4"><div class="flex gap-05 chunkSize noselect svelte-u1kdp4"><div class="svelte-u1kdp4"> </div> <div class="svelte-u1kdp4"><!></div></div> <div class="font-label total svelte-u1kdp4"> </div></div>'), pe = g('<div class="iconLeft svelte-u1kdp4"><!></div>'), fe = g('<div class="iconRight svelte-u1kdp4"><!></div>'), ge = g('<div class="container svelte-u1kdp4"><div class="flex"><!> <!> <!></div> <!></div>');
function we(J, d) {
  ae(d, true);
  const K = (t) => {
    var a = ue(), e = l(a);
    oe(e, 21, () => s(T), re, (v, m) => {
      var u = de(), L = l(u);
      I(L, { invisible: true, onclick: () => P(s(m)), onLeft: $, onRight: j, children: (b, H) => {
        var _ = ve(), te = l(_, true);
        o(_), h(() => B(te, s(m))), p(b, _);
      }, $$slots: { default: true } }), o(u), h(() => {
        f(u, "aria-label", `${c.pagination.gotoPage} ${s(m)}`), f(u, "aria-current", i() === s(m) ? "step" : void 0);
      }), p(v, u);
    }), o(e), o(a), h(() => f(a, "aria-label", c.pagination.pagination)), p(t, a);
  }, Q = (t) => {
    var a = ce(), e = l(a), v = l(e), m = l(v, true);
    o(v);
    var u = k(v, 2), L = l(u);
    ne(L, { get ariaLabel() {
      return c.pagination.showCount;
    }, options: C, offsetTop: "-14rem", borderless: true, get value() {
      return r();
    }, set value(_) {
      r(_);
    } }), o(u), o(e);
    var b = k(e, 2), H = l(b);
    o(b), o(a), h(() => {
      B(m, c.pagination.entries), B(H, `${c.pagination.total ?? ""}: ${d.items.length ?? ""}`);
    }), p(t, a);
  }, C = [5, 10, 20, 30, 50, 100];
  let U = x(d, "itemsPaginated", 15), i = x(d, "page", 15, 1), r = x(d, "pageSize", 31, () => R(C[2])), V = x(d, "compact", 3, false), c = le();
  const D = "1rem";
  let O = M(() => r()), n = N(R([])), T = N(R([]));
  w(() => {
    r() !== O && (O = M(() => r()), i(1));
  }), w(() => {
    let t = [];
    for (let a = 0; a < d.items.length; a += r()) {
      const e = d.items.slice(a, a + r());
      t.push(e);
    }
    q(n, t, true), U(t[i() - 1]);
  }), w(() => {
    y();
  });
  function $() {
    i() > 1 && P(i() - 1);
  }
  function j() {
    i() < s(n).length && P(i() + 1);
  }
  function P(t) {
    i(t), y();
  }
  function y() {
    let t = [], a = Math.floor(r() / 2);
    if (s(n).length <= r()) for (let e = 1; e <= s(n).length; e++) t.push(e);
    else if (i() <= a) for (let e = 1; e <= r(); e++) t.push(e);
    else if (i() > s(n).length - a - 1) for (let e = s(n).length - r(); e <= s(n).length - 1; e++) t.push(e + 1);
    else for (let e = i() - a; e < i() - a + r(); e++) t.push(e);
    q(T, t, true);
  }
  var z = ge(), S = l(z), A = l(S);
  const W = E(() => i() === 1);
  I(A, { onclick: $, invisible: true, get isDisabled() {
    return s(W);
  }, children: (t, a) => {
    var e = pe(), v = l(e);
    G(v, { width: D }), o(e), h(() => {
      f(e, "aria-label", c.pagination.gotoPagePrev), f(e, "data-disabled", i() === 1);
    }), p(t, e);
  }, $$slots: { default: true } });
  var F = k(A, 2);
  K(F);
  var X = k(F, 2);
  const Y = E(() => i() === s(n).length);
  I(X, { onclick: j, invisible: true, get isDisabled() {
    return s(Y);
  }, children: (t, a) => {
    var e = fe(), v = l(e);
    G(v, { width: D }), o(e), h(() => {
      f(e, "aria-label", c.pagination.gotoPageNext), f(e, "data-disabled", i() === s(n).length);
    }), p(t, e);
  }, $$slots: { default: true } }), o(S);
  var Z = k(S, 2);
  {
    var ee = (t) => {
      Q(t);
    };
    se(Z, (t) => {
      V() || t(ee);
    });
  }
  o(z), p(J, z), ie();
}
export {
  we as P
};
