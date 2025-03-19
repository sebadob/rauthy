import { t as n, a as c } from "./DLBGyKVC.js";
import { c as a, r as i, l as p, k as u, j as b, t as k } from "./CmQi0fbH.js";
import { s as _ } from "./71gCR-8F.js";
import { s as g, b as j } from "./DOlUUCkJ.js";
import { p as h } from "./B_ggA-0N.js";
import { B as w } from "./DPLO-ozG.js";
var B = n('<div class="svelte-1jb6swk"><!></div>'), x = n('<li><div class="svelte-1jb6swk"><!></div></li>');
function A(f, s) {
  let l = u(void 0);
  var e = x();
  let o;
  var v = a(e), d = a(v);
  w(d, { invisible: true, get onclick() {
    return s.onclick;
  }, get ref() {
    return b(l);
  }, set ref(t) {
    p(l, h(t));
  }, children: (t, L) => {
    var r = B(), m = a(r);
    _(m, () => s.children), i(r), c(t, r);
  }, $$slots: { default: true } }), i(v), i(e), k((t) => {
    g(e, "aria-current", s.selected ? "page" : "false"), o = j(e, 1, "svelte-1jb6swk", null, o, t);
  }, [() => ({ pictureLeft: s.pictureLeft })]), c(f, e);
}
export {
  A as N
};
