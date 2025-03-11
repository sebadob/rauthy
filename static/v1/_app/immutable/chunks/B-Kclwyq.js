import { t as w, d as B, a as l, e as k } from "./CWf9OOFK.js";
import { p as b, s as j, c as A, f as C, l as c, k as P, j as r, t as p, a as R, aa as y, a8 as I, r as N } from "./nlANaGLT.js";
import { s as q } from "./BmMHVVX3.js";
import { s as z } from "./BgMuVWsk.js";
import { b as D } from "./CZv_AhHu.js";
import { p as E } from "./5u5qd9TD.js";
import { p as f } from "./uWmgYd3Z.js";
import { B as F } from "./BEQMYyDu.js";
import { M as G } from "./AUelbH2M.js";
import { u as H } from "./Bb8ybDgy.js";
var J = w("<div><!> <!></div>");
function Y(u, t) {
  b(t, true);
  let d = f(t, "level", 3, 2), n = f(t, "closeModal", 15), v = H();
  const g = d() === 1 ? 2 : 3;
  let a = P(false);
  var o = J();
  let i;
  var m = A(o);
  const h = y(() => r(a) ? g : d());
  F(m, { get level() {
    return r(h);
  }, onclick: () => c(a, true), children: (e, x) => {
    I();
    var s = k();
    p(() => q(s, v.common.addNew)), l(e, s);
  }, $$slots: { default: true } });
  var M = j(m, 2);
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
    var s = B(), _ = C(s);
    z(_, () => t.children), l(e, s);
  }, $$slots: { default: true } }), N(o), p((e) => i = D(o, 1, "svelte-1gkjphv", null, i, e), [() => ({ alignRight: t.alignRight })]), l(u, o), R();
}
export {
  Y as B
};
