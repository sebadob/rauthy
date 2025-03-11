import { t as w, d as B, a as l, e as k } from "./BH6NCLk-.js";
import { p as j, s as A, c as C, f as P, l as c, k as R, j as r, t as p, a as b, a9 as y, a7 as I, r as N } from "./CvlvO1XB.js";
import { s as q } from "./CTI4QPiR.js";
import { s as z } from "./-T201g_q.js";
import { a as D } from "./BMbqVy6X.js";
import { p as E } from "./Wh68IIk2.js";
import { p as f } from "./C6SR4G2t.js";
import { B as F } from "./CqH8LXO-.js";
import { M as G } from "./S7-4yce3.js";
import { u as H } from "./D8mHI_K9.js";
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
