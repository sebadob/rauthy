import { t as w, d as B, a as l, e as k } from "./YHhP1LbZ.js";
import { p as b, s as j, c as A, f as C, l as c, k as P, j as r, t as p, a as R, a9 as y, a8 as I, r as N } from "./Ck6jKiur.js";
import { s as q } from "./tDAaDMC_.js";
import { s as z } from "./9ksWc3Vn.js";
import { b as D } from "./BTaFr7HN.js";
import { p as E } from "./ho0YXExL.js";
import { p as f } from "./DZP54pO_.js";
import { B as F } from "./Bd2Rvcxs.js";
import { M as G } from "./DnVb7Nyf.js";
import { u as H } from "./BM7IgWpA.js";
var J = w("<div><!> <!></div>");
function Y(u, t) {
  b(t, true);
  let d = f(t, "level", 3, 2), n = f(t, "closeModal", 15), v = H();
  const g = d() === 1 ? 2 : 3;
  let o = P(false);
  var a = J();
  let i;
  var m = A(a);
  const h = y(() => r(o) ? g : d());
  F(m, { get level() {
    return r(h);
  }, onclick: () => c(o, true), children: (e, x) => {
    I();
    var s = k();
    p(() => q(s, v.common.addNew)), l(e, s);
  }, $$slots: { default: true } });
  var M = j(m, 2);
  G(M, { get onClose() {
    return t.onClose;
  }, get showModal() {
    return r(o);
  }, set showModal(e) {
    c(o, E(e));
  }, get closeModal() {
    return n();
  }, set closeModal(e) {
    n(e);
  }, children: (e, x) => {
    var s = B(), _ = C(s);
    z(_, () => t.children), l(e, s);
  }, $$slots: { default: true } }), N(a), p((e) => i = D(a, 1, "svelte-1gkjphv", null, i, e), [() => ({ alignRight: t.alignRight })]), l(u, a), R();
}
export {
  Y as B
};
