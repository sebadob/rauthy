import { t as p, a as k } from "./D8nUqfqE.js";
import { c as n, s as h, t as v, r as d, a8 as _ } from "./D-nwkJyM.js";
import { d as u } from "./DmLjbmU6.js";
import { s as g } from "./ivlJJTAR.js";
import { r as w, s as l } from "./D_HYGYLR.js";
import { a as y } from "./BUAPoI3e.js";
import { a as x } from "./DAX5MQt8.js";
import { p as s } from "./2rFDT0Lm.js";
function C(o, a) {
  o.code === "Enter" && a();
}
var I = p('<label class="font-label noselect svelte-t6wfg9"><input type="checkbox" class="svelte-t6wfg9"> <span class="svelte-t6wfg9"><!></span></label>');
function F(o, a) {
  let t = s(a, "checked", 15, false), m = s(a, "ariaLabel", 3, ""), b = s(a, "borderColor", 3, "hsl(var(--bg-high))");
  function i() {
    t(!t());
  }
  var r = I(), e = n(r);
  w(e), e.__click = i, e.__keydown = [C, i];
  var c = h(e, 2), f = n(c);
  g(f, () => a.children ?? _), d(c), d(r), v(() => {
    l(e, "name", a.name), e.disabled = a.disabled, l(e, "aria-disabled", a.disabled), l(e, "aria-checked", t()), l(e, "aria-label", m()), y(e, "border-color", b());
  }), x(e, t), k(o, r);
}
u(["click", "keydown"]);
export {
  F as I
};
