import { d as C, a as d, t as I } from "./CWf9OOFK.js";
import { p as P, P as g, k as S, ab as v, f as A, a as O, j as t, aa as n, c as j, l as B, r as M, t as q } from "./nlANaGLT.js";
import { i as z } from "./DOjUa9u5.js";
import { b as D } from "./CZv_AhHu.js";
import { p as k } from "./5u5qd9TD.js";
import { p as E } from "./uWmgYd3Z.js";
import { u as F } from "./DwvS5LQk.js";
import { n as G } from "./B21bTIl7.js";
import { O as H } from "./FmRbdHbC.js";
import { b as J } from "./BO1A6s0c.js";
import { p as K } from "./B4Xt-L9L.js";
import { u as b } from "./ByEJQidz.js";
var N = I("<div><!></div>");
function ae(h, r) {
  P(r, true);
  let l = E(r, "openTop", 3, true);
  const f = ";Path=/;SameSite=Lax;Max-Age=157680000";
  let m = F(), c = g(() => m.lang), a = S(k(c)), s = n(() => {
    var _a;
    return ((_a = K.route.id) == null ? void 0 : _a.includes("/admin")) ? b().admin() : b().common();
  }), L = n(() => {
    var _a;
    return l() ? `-${(((_a = t(s)) == null ? void 0 : _a.length) || 0) * 2 + 2}rem` : void 0;
  }), _ = n(() => l() ? ".2rem" : void 0);
  v(() => {
    let e = G("locale"), o = g(() => t(a));
    e && e !== o && (document.cookie = "locale=" + o + f);
  }), v(() => {
    t(a) !== c && w();
  });
  async function w() {
    if (document.cookie = "locale=" + t(a).toLowerCase() + f, r.updateBackend) {
      let e = await J("/auth/v1/update_language");
      if (e.error) {
        console.error(e.error);
        return;
      }
    }
    window.location.reload();
  }
  var u = C(), x = A(u);
  {
    var T = (e) => {
      var o = N();
      let p;
      var y = j(o);
      H(y, { get ariaLabel() {
        return m.common.selectI18n;
      }, get options() {
        return t(s);
      }, borderless: true, get offsetTop() {
        return t(L);
      }, get offsetLeft() {
        return t(_);
      }, get value() {
        return t(a);
      }, set value(i) {
        B(a, k(i));
      } }), M(o), q((i) => p = D(o, 1, "svelte-10y6iii", null, p, i), [() => ({ absolute: r.absolute })]), d(e, o);
    };
    z(x, (e) => {
      t(s) && e(T);
    });
  }
  d(h, u), O();
}
export {
  ae as L
};
