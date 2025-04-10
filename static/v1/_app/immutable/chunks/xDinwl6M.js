import { d as y, a as d, t as C } from "./BxmJRzoY.js";
import { p as I, F as g, j as S, a1 as A, a4 as n, a5 as v, f as O, a as P, k as t, c as j, l as B, r as F, t as M } from "./w0HvPX0p.js";
import { i as q } from "./iO9_dPNE.js";
import { b as z } from "./BdbQ6g_y.js";
import { p as D } from "./C6GSeq7M.js";
import { u as E } from "./0cG6LBdy.js";
import { n as G } from "./B21bTIl7.js";
import { O as H } from "./BBiSUZQU.js";
import { b as J } from "./UPFlzoow.js";
import { p as K } from "./DLjh9JEQ.js";
import { u as k } from "./C1JKVfs5.js";
var N = C("<div><!></div>");
function oe(_, r) {
  I(r, true);
  let l = D(r, "openTop", 3, true);
  const f = ";Path=/;SameSite=Lax;Max-Age=157680000";
  let m = E(), u = g(() => m.lang), a = S(A(u)), s = n(() => {
    var _a;
    return ((_a = K.route.id) == null ? void 0 : _a.includes("/admin")) ? k().admin() : k().common();
  }), h = n(() => {
    var _a;
    return l() ? `-${(((_a = t(s)) == null ? void 0 : _a.length) || 0) * 2 + 2}rem` : void 0;
  }), L = n(() => l() ? ".2rem" : void 0);
  v(() => {
    let e = G("locale"), o = g(() => t(a));
    e && e !== o && (document.cookie = "locale=" + o + f);
  }), v(() => {
    t(a) !== u && b();
  });
  async function b() {
    if (document.cookie = "locale=" + t(a).toLowerCase() + f, r.updateBackend) {
      let e = await J("/auth/v1/update_language");
      if (e.error) {
        console.error(e.error);
        return;
      }
    }
    window.location.reload();
  }
  var c = y(), w = O(c);
  {
    var x = (e) => {
      var o = N();
      let p;
      var T = j(o);
      H(T, { get ariaLabel() {
        return m.common.selectI18n;
      }, get options() {
        return t(s);
      }, borderless: true, get offsetTop() {
        return t(h);
      }, get offsetLeft() {
        return t(L);
      }, get value() {
        return t(a);
      }, set value(i) {
        B(a, i, true);
      } }), F(o), M((i) => p = z(o, 1, "svelte-10y6iii", null, p, i), [() => ({ absolute: r.absolute })]), d(e, o);
    };
    q(w, (e) => {
      t(s) && e(x);
    });
  }
  d(_, c), P();
}
export {
  oe as L
};
