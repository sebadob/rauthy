import { t as h, a as k } from "./DLBGyKVC.js";
import { p as u, c as d, s as v, t as _, a as g, r as b, ab as w } from "./CmQi0fbH.js";
import { d as y } from "./BjaYyaa_.js";
import { s as x } from "./71gCR-8F.js";
import { r as C, s as l, a as I } from "./DOlUUCkJ.js";
import { c as L } from "./B3JAWFL5.js";
import { p as r } from "./DNJm3-SG.js";
function E(s, a) {
  s.code === "Enter" && a();
}
var j = h('<label class="font-label noselect svelte-t6wfg9"><input type="checkbox" class="svelte-t6wfg9"> <span class="svelte-t6wfg9"><!></span></label>');
function H(s, a) {
  u(a, true);
  let t = r(a, "checked", 15, false), f = r(a, "ariaLabel", 3, ""), m = r(a, "borderColor", 3, "hsl(var(--bg-high))");
  function i() {
    t(!t());
  }
  var o = j(), e = d(o);
  C(e), e.__click = i, e.__keydown = [E, i];
  let c;
  var n = v(e, 2), p = d(n);
  x(p, () => a.children ?? w), b(n), b(o), _(() => {
    l(e, "name", a.name), e.disabled = a.disabled, l(e, "aria-disabled", a.disabled), l(e, "aria-checked", t()), l(e, "aria-label", f()), c = I(e, "", c, { "border-color": m() });
  }), L(e, t), k(s, o), g();
}
y(["click", "keydown"]);
export {
  H as I
};
