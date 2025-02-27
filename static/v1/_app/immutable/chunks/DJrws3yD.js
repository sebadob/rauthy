import { t as b, a as w } from "./D8nUqfqE.js";
import { p as S, O as p, k as _, aa as d, t as x, a as G, l as T, j as e, a9 as s, c as y, r as I } from "./D-nwkJyM.js";
import { a as N } from "./D_HYGYLR.js";
import { p as g } from "./BiI21XkS.js";
import { p as O } from "./2rFDT0Lm.js";
import { u as C } from "./CMjKUQkH.js";
import { L as E, n as M, o as P } from "./DppGgfa0.js";
import { O as U } from "./Fz9KQU3M.js";
import { b as j } from "./emZtalxW.js";
import { p as B } from "./BsLhmaxX.js";
var D = b("<div><!></div>");
function X(v, a) {
  S(a, true);
  let i = O(a, "openTop", 3, true);
  const l = ";Path=/;SameSite=Lax;Max-Age=157680000";
  let n = C(), f = p(() => n.lang), o = _(g(f)), u = s(() => {
    var _a;
    return ((_a = B.route.id) == null ? void 0 : _a.includes("/admin")) ? E : M;
  }), L = s(() => i() ? `-${e(u).length * 2 + 2}rem` : void 0), k = s(() => i() ? ".2rem" : void 0);
  d(() => {
    let t = P("locale"), m = p(() => e(o));
    t && t !== m && (document.cookie = "locale=" + m + l);
  }), d(() => {
    e(o) !== f && h();
  });
  async function h() {
    if (document.cookie = "locale=" + e(o).toLowerCase() + l, a.updateBackend) {
      let t = await j("/auth/v1/update_language");
      if (t.error) {
        console.error(t.error);
        return;
      }
    }
    window.location.reload();
  }
  var r = D();
  let c;
  var A = y(r);
  U(A, { get ariaLabel() {
    return n.common.selectI18n;
  }, get options() {
    return e(u);
  }, borderless: true, get offsetTop() {
    return e(L);
  }, get offsetLeft() {
    return e(k);
  }, get value() {
    return e(o);
  }, set value(t) {
    T(o, g(t));
  } }), I(r), x(() => c = N(r, 1, "svelte-10y6iii", null, c, { absolute: a.absolute })), w(v, r), G();
}
export {
  X as L
};
