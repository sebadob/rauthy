import { t as h, a as k } from "./BxmJRzoY.js";
import { p as u, c as d, s as v, t as _, a as g, r as b, a6 as w } from "./w0HvPX0p.js";
import { d as y } from "./BzP2S3Z_.js";
import { s as x } from "./DM69BKKN.js";
import { r as C, s as l, a as I } from "./BdbQ6g_y.js";
import { c as L } from "./Cxw7xmE1.js";
import { p as r } from "./C6GSeq7M.js";
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
