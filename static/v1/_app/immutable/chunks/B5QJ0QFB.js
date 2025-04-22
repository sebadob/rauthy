import { t as w, d as B, a as l, e as k } from "./BxmJRzoY.js";
import { p as b, j, a4 as A, s as C, c as P, f as R, l as c, k as r, t as u, a as I, a3 as N, r as q } from "./w0HvPX0p.js";
import { s as y } from "./BzP2S3Z_.js";
import { s as z } from "./DM69BKKN.js";
import { b as D } from "./BdbQ6g_y.js";
import { p as f } from "./C6GSeq7M.js";
import { B as E } from "./C4AV2CoD.js";
import { M as F } from "./DRNQecP5.js";
import { u as G } from "./D12OFlGX.js";
var H = w("<div><!> <!></div>");
function W(p, t) {
  b(t, true);
  let d = f(t, "level", 3, 2), n = f(t, "closeModal", 15), v = G();
  const g = d() === 1 ? 2 : 3;
  let a = j(false);
  var o = H();
  let i;
  var m = P(o);
  const h = A(() => r(a) ? g : d());
  E(m, { get level() {
    return r(h);
  }, onclick: () => c(a, true), children: (e, _) => {
    N();
    var s = k();
    u(() => y(s, v.common.addNew)), l(e, s);
  }, $$slots: { default: true } });
  var M = C(m, 2);
  F(M, { get onClose() {
    return t.onClose;
  }, get showModal() {
    return r(a);
  }, set showModal(e) {
    c(a, e, true);
  }, get closeModal() {
    return n();
  }, set closeModal(e) {
    n(e);
  }, children: (e, _) => {
    var s = B(), x = R(s);
    z(x, () => t.children), l(e, s);
  }, $$slots: { default: true } }), q(o), u((e) => i = D(o, 1, "svelte-1gkjphv", null, i, e), [() => ({ alignRight: t.alignRight })]), l(p, o), I();
}
export {
  W as B
};
