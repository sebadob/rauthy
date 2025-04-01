import { t as v, a as r, e as c } from "../chunks/BxmJRzoY.js";
import { p as O, j as y, f as P, a as G, a2 as N, s as o, l as W, k, c as q, a3 as m, t as f, r as z } from "../chunks/w0HvPX0p.js";
import { h as D, s as u } from "../chunks/BzP2S3Z_.js";
import { i as F } from "../chunks/iO9_dPNE.js";
import { B as p } from "../chunks/C8YTstTD.js";
import { M as H } from "../chunks/DKM0QPz5.js";
import { C as J } from "../chunks/QNragXLc.js";
import { u as K } from "../chunks/0cG6LBdy.js";
import { T as Q } from "../chunks/BdDmaO9S.js";
import { j as U } from "../chunks/B21bTIl7.js";
import { T as V } from "../chunks/B1f0afjj.js";
import { L as X } from "../chunks/D9_8lD-s.js";
var Y = v('<div class="btn svelte-10gbrvf"><!> <!> <!></div>'), Z = v("<!> <!> <!>", 1), tt = v("<!> <!>", 1);
function vt(L, R) {
  O(R, true);
  const s = "9rem";
  let i = K(), n = y(false);
  function C() {
    window.location.href = "/auth/v1/admin";
  }
  function S() {
    window.location.href = "/auth/v1/account";
  }
  function j() {
    window.location.href = "/auth/v1/users/register";
  }
  var h = tt();
  D((a) => {
    N.title = "Rauthy";
  });
  var _ = P(h);
  Q(_, { id: U, get value() {
    return k(n);
  }, set value(a) {
    W(n, a, true);
  } });
  var A = o(_, 2);
  H(A, { children: (a, et) => {
    var $ = Z(), g = P($);
    J(g, { children: (E, rt) => {
      var l = Y(), x = q(l);
      p(x, { onclick: S, width: s, children: (t, d) => {
        m();
        var e = c();
        f(() => u(e, i.index.accountLogin)), r(t, e);
      }, $$slots: { default: true } });
      var T = o(x, 2);
      {
        var I = (t) => {
          p(t, { level: 2, onclick: j, width: s, children: (d, e) => {
            m();
            var b = c();
            f(() => u(b, i.index.register)), r(d, b);
          }, $$slots: { default: true } });
        };
        F(T, (t) => {
          k(n) && t(I);
        });
      }
      var M = o(T, 2);
      p(M, { level: 3, onclick: C, width: s, children: (t, d) => {
        m();
        var e = c();
        f(() => u(e, i.index.adminLogin)), r(t, e);
      }, $$slots: { default: true } }), z(l), r(E, l);
    } });
    var w = o(g, 2);
    V(w, { absolute: true });
    var B = o(w, 2);
    X(B, { absolute: true }), r(a, $);
  } }), r(L, h), G();
}
export {
  vt as component
};
