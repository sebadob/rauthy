import { d as C, a as p, t as I } from "./BH6NCLk-.js";
import { p as O, O as d, k as S, aa as g, f as A, a as P, j as t, a9 as i, c as j, l as B, r as M, t as q } from "./CvlvO1XB.js";
import { i as z } from "./BUO_AUgz.js";
import { a as D } from "./BMbqVy6X.js";
import { p as v } from "./Wh68IIk2.js";
import { p as E } from "./C6SR4G2t.js";
import { u as F } from "./CUqQZdNU.js";
import { n as G } from "./B21bTIl7.js";
import { O as H } from "./D1anwFsC.js";
import { b as J } from "./tF66aiNY.js";
import { p as K } from "./DlDtjbc8.js";
import { u as k } from "./CN7ybZO6.js";
var N = I("<div><!></div>");
function ae(h, r) {
  O(r, true);
  let n = E(r, "openTop", 3, true);
  const l = ";Path=/;SameSite=Lax;Max-Age=157680000";
  let f = F(), m = d(() => f.lang), a = S(v(m)), s = i(() => {
    var _a;
    return ((_a = K.route.id) == null ? void 0 : _a.includes("/admin")) ? k().admin() : k().common();
  }), L = i(() => {
    var _a;
    return n() ? `-${(((_a = t(s)) == null ? void 0 : _a.length) || 0) * 2 + 2}rem` : void 0;
  }), _ = i(() => n() ? ".2rem" : void 0);
  g(() => {
    let e = G("locale"), o = d(() => t(a));
    e && e !== o && (document.cookie = "locale=" + o + l);
  }), g(() => {
    t(a) !== m && b();
  });
  async function b() {
    if (document.cookie = "locale=" + t(a).toLowerCase() + l, r.updateBackend) {
      let e = await J("/auth/v1/update_language");
      if (e.error) {
        console.error(e.error);
        return;
      }
    }
    window.location.reload();
  }
  var c = C(), w = A(c);
  {
    var x = (e) => {
      var o = N();
      let u;
      var T = j(o);
      H(T, { get ariaLabel() {
        return f.common.selectI18n;
      }, get options() {
        return t(s);
      }, borderless: true, get offsetTop() {
        return t(L);
      }, get offsetLeft() {
        return t(_);
      }, get value() {
        return t(a);
      }, set value(y) {
        B(a, v(y));
      } }), M(o), q(() => u = D(o, 1, "svelte-10y6iii", null, u, { absolute: r.absolute })), p(e, o);
    };
    z(w, (e) => {
      t(s) && e(x);
    });
  }
  p(h, c), P();
}
export {
  ae as L
};
