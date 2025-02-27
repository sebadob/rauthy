import { t as w, d as B, a as l, e as k } from "./D8nUqfqE.js";
import { p as j, s as A, c as C, f as P, l as c, k as R, j as r, t as p, a as b, a9 as y, a7 as I, r as N } from "./D-nwkJyM.js";
import { s as q } from "./DmLjbmU6.js";
import { s as z } from "./ivlJJTAR.js";
import { a as D } from "./D_HYGYLR.js";
import { p as E } from "./BiI21XkS.js";
import { p as f } from "./2rFDT0Lm.js";
import { B as F } from "./BUAPoI3e.js";
import { M as G } from "./D4OK0IqS.js";
import { u as H } from "./DMJUG0wm.js";
var J = w("<div><!> <!></div>");
function Y(u, t) {
  j(t, true);
  let d = f(t, "level", 3, 2), n = f(t, "closeModal", 15), v = H();
  const g = d() === 1 ? 2 : 3;
  let a = R(false);
  var o = J();
  let i;
  var m = C(o);
  const h = y(() => r(a) ? g : d());
  F(m, { get level() {
    return r(h);
  }, onclick: () => c(a, true), children: (e, x) => {
    I();
    var s = k();
    p(() => q(s, v.common.addNew)), l(e, s);
  }, $$slots: { default: true } });
  var M = A(m, 2);
  G(M, { get onClose() {
    return t.onClose;
  }, get showModal() {
    return r(a);
  }, set showModal(e) {
    c(a, E(e));
  }, get closeModal() {
    return n();
  }, set closeModal(e) {
    n(e);
  }, children: (e, x) => {
    var s = B(), _ = P(s);
    z(_, () => t.children), l(e, s);
  }, $$slots: { default: true } }), N(o), p(() => i = D(o, 1, "svelte-1gkjphv", null, i, { alignRight: t.alignRight })), l(u, o), b();
}
export {
  Y as B
};
