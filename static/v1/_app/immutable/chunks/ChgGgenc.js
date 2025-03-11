import { t as k, a as h } from "./CWf9OOFK.js";
import { c as d, s as v, t as _, r as b, a9 as u } from "./nlANaGLT.js";
import { d as g } from "./BmMHVVX3.js";
import { s as w } from "./BgMuVWsk.js";
import { r as y, s as l, a as x } from "./CZv_AhHu.js";
import { c as C } from "./DHRD3bu9.js";
import { p as r } from "./uWmgYd3Z.js";
function I(s, a) {
  s.code === "Enter" && a();
}
var L = k('<label class="font-label noselect svelte-t6wfg9"><input type="checkbox" class="svelte-t6wfg9"> <span class="svelte-t6wfg9"><!></span></label>');
function F(s, a) {
  let t = r(a, "checked", 15, false), f = r(a, "ariaLabel", 3, ""), m = r(a, "borderColor", 3, "hsl(var(--bg-high))");
  function i() {
    t(!t());
  }
  var o = L(), e = d(o);
  y(e), e.__click = i, e.__keydown = [I, i];
  let c;
  var n = v(e, 2), p = d(n);
  w(p, () => a.children ?? u), b(n), b(o), _(() => {
    l(e, "name", a.name), e.disabled = a.disabled, l(e, "aria-disabled", a.disabled), l(e, "aria-checked", t()), l(e, "aria-label", f()), c = x(e, "", c, { "border-color": m() });
  }), C(e, t), h(s, o);
}
g(["click", "keydown"]);
export {
  F as I
};
