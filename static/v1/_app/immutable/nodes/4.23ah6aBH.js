import { t as v, a as o, e as c } from "../chunks/BH6NCLk-.js";
import { p as M, f as P, a as O, a6 as G, s as r, l as N, k as W, j as k, c as q, a7 as m, t as f, r as z } from "../chunks/CvlvO1XB.js";
import { h as D, s as p } from "../chunks/CTI4QPiR.js";
import { i as F } from "../chunks/BUO_AUgz.js";
import { p as H } from "../chunks/Wh68IIk2.js";
import { B as u } from "../chunks/DMkkW5Nn.js";
import { M as J } from "../chunks/BbRujPvb.js";
import { C as K } from "../chunks/GWgockW8.js";
import { u as Q } from "../chunks/BQ1-pLIs.js";
import { T as U } from "../chunks/C8BaH7A7.js";
import { j as V } from "../chunks/B8wC3kJv.js";
import { T as X } from "../chunks/C6vpM94V.js";
import { L as Y } from "../chunks/BWe1X9CS.js";
var Z = v('<div class="btn svelte-10gbrvf"><!> <!> <!></div>'), tt = v("<!> <!> <!>", 1), et = v("<!> <!>", 1);
function _t(L, R) {
  M(R, true);
  const s = "9rem";
  let i = Q(), n = W(false);
  function C() {
    window.location.href = "/auth/v1/admin";
  }
  function S() {
    window.location.href = "/auth/v1/account";
  }
  function j() {
    window.location.href = "/auth/v1/users/register";
  }
  var h = et();
  D((a) => {
    G.title = "Rauthy";
  });
  var _ = P(h);
  U(_, { id: V, get value() {
    return k(n);
  }, set value(a) {
    N(n, H(a));
  } });
  var y = r(_, 2);
  J(y, { children: (a, ot) => {
    var $ = tt(), g = P($);
    K(g, { children: (B, rt) => {
      var l = Z(), w = q(l);
      u(w, { onclick: S, width: s, children: (t, d) => {
        m();
        var e = c();
        f(() => p(e, i.index.accountLogin)), o(t, e);
      }, $$slots: { default: true } });
      var T = r(w, 2);
      {
        var E = (t) => {
          u(t, { level: 2, onclick: j, width: s, children: (d, e) => {
            m();
            var b = c();
            f(() => p(b, i.index.register)), o(d, b);
          }, $$slots: { default: true } });
        };
        F(T, (t) => {
          k(n) && t(E);
        });
      }
      var I = r(T, 2);
      u(I, { level: 3, onclick: C, width: s, children: (t, d) => {
        m();
        var e = c();
        f(() => p(e, i.index.adminLogin)), o(t, e);
      }, $$slots: { default: true } }), z(l), o(B, l);
    } });
    var x = r(g, 2);
    X(x, { absolute: true });
    var A = r(x, 2);
    Y(A, { absolute: true }), o(a, $);
  } }), o(L, h), O();
}
export {
  _t as component
};
