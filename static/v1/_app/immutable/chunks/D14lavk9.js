import { t as g, a as p } from "./BH6NCLk-.js";
import { p as ae, O as N, k as q, aa as w, l as E, j as s, c as l, r as o, t as h, a9 as F, s as k, a as ie } from "./CvlvO1XB.js";
import { s as B } from "./CTI4QPiR.js";
import { i as se } from "./BUO_AUgz.js";
import { e as oe, i as re } from "./BpWRzPRQ.js";
import { s as f } from "./BMbqVy6X.js";
import { p as b } from "./Wh68IIk2.js";
import { p as P } from "./C6SR4G2t.js";
import { I as G } from "./BZaoh_H_.js";
import { u as le } from "./CgEHB2u3.js";
import { B as I } from "./DMkkW5Nn.js";
import { O as ne } from "./ahjiAH8y.js";
var ve = g('<div class="link noselect svelte-u1kdp4"> </div>'), de = g('<li class="svelte-u1kdp4"><!></li>'), ce = g('<nav class="svelte-u1kdp4"><ul class="svelte-u1kdp4"></ul></nav>'), ue = g('<div class="flex gap-10 svelte-u1kdp4"><div class="flex gap-05 chunkSize noselect svelte-u1kdp4"><div class="svelte-u1kdp4"> </div> <div class="svelte-u1kdp4"><!></div></div> <div class="font-label total svelte-u1kdp4"> </div></div>'), pe = g('<div class="iconLeft svelte-u1kdp4"><!></div>'), fe = g('<div class="iconRight svelte-u1kdp4"><!></div>'), ge = g('<div class="container svelte-u1kdp4"><div class="flex"><!> <!> <!></div> <!></div>');
function Be(J, d) {
  ae(d, true);
  const K = (t) => {
    var a = ce(), e = l(a);
    oe(e, 21, () => s(T), re, (v, m) => {
      var c = de(), R = l(c);
      I(R, { invisible: true, onclick: () => z(s(m)), onLeft: $, onRight: j, children: (x, M) => {
        var _ = ve(), te = l(_, true);
        o(_), h(() => B(te, s(m))), p(x, _);
      }, $$slots: { default: true } }), o(c), h(() => {
        f(c, "aria-label", `${u.pagination.gotoPage} ${s(m)}`), f(c, "aria-current", i() === s(m) ? "step" : void 0);
      }), p(v, c);
    }), o(e), o(a), h(() => f(a, "aria-label", u.pagination.pagination)), p(t, a);
  }, Q = (t) => {
    var a = ue(), e = l(a), v = l(e), m = l(v, true);
    o(v);
    var c = k(v, 2), R = l(c);
    ne(R, { get ariaLabel() {
      return u.pagination.showCount;
    }, options: O, offsetTop: "-14rem", borderless: true, get value() {
      return r();
    }, set value(_) {
      r(_);
    } }), o(c), o(e);
    var x = k(e, 2), M = l(x);
    o(x), o(a), h(() => {
      B(m, u.pagination.entries), B(M, `${u.pagination.total ?? ""}: ${d.items.length ?? ""}`);
    }), p(t, a);
  }, O = [5, 10, 20, 30, 50, 100];
  let U = P(d, "itemsPaginated", 15), i = P(d, "page", 15, 1), r = P(d, "pageSize", 31, () => b(O[2])), V = P(d, "compact", 3, false), u = le();
  const C = "1rem";
  let D = N(() => r()), n = q(b([])), T = q(b([]));
  w(() => {
    r() !== D && (D = N(() => r()), i(1));
  }), w(() => {
    let t = [];
    for (let a = 0; a < d.items.length; a += r()) {
      const e = d.items.slice(a, a + r());
      t.push(e);
    }
    E(n, b(t)), U(t[i() - 1]);
  }), w(() => {
    y();
  });
  function $() {
    i() > 1 && z(i() - 1);
  }
  function j() {
    i() < s(n).length && z(i() + 1);
  }
  function z(t) {
    i(t), y();
  }
  function y() {
    let t = [], a = Math.floor(r() / 2);
    if (s(n).length <= r()) for (let e = 1; e <= s(n).length; e++) t.push(e);
    else if (i() <= a) for (let e = 1; e <= r(); e++) t.push(e);
    else if (i() > s(n).length - a - 1) for (let e = s(n).length - r(); e <= s(n).length - 1; e++) t.push(e + 1);
    else for (let e = i() - a; e < i() - a + r(); e++) t.push(e);
    E(T, b(t));
  }
  var S = ge(), L = l(S), A = l(L);
  const W = F(() => i() === 1);
  I(A, { onclick: $, invisible: true, get isDisabled() {
    return s(W);
  }, children: (t, a) => {
    var e = pe(), v = l(e);
    G(v, { width: C }), o(e), h(() => {
      f(e, "aria-label", u.pagination.gotoPagePrev), f(e, "data-disabled", i() === 1);
    }), p(t, e);
  }, $$slots: { default: true } });
  var H = k(A, 2);
  K(H);
  var X = k(H, 2);
  const Y = F(() => i() === s(n).length);
  I(X, { onclick: j, invisible: true, get isDisabled() {
    return s(Y);
  }, children: (t, a) => {
    var e = fe(), v = l(e);
    G(v, { width: C }), o(e), h(() => {
      f(e, "aria-label", u.pagination.gotoPageNext), f(e, "data-disabled", i() === s(n).length);
    }), p(t, e);
  }, $$slots: { default: true } }), o(L);
  var Z = k(L, 2);
  {
    var ee = (t) => {
      Q(t);
    };
    se(Z, (t) => {
      V() || t(ee);
    });
  }
  o(S), p(J, S), ie();
}
export {
  Be as P
};
