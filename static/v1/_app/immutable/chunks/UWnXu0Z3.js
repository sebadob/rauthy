import { t as n, a as c } from "./BxmJRzoY.js";
import { j as m, c as a, r as i, l as p, k as b, t as k } from "./w0HvPX0p.js";
import { s as _ } from "./DM69BKKN.js";
import { s as g, b as j } from "./BdbQ6g_y.js";
import { B as h } from "./C4AV2CoD.js";
var w = n('<div class="svelte-1jb6swk"><!></div>'), B = n('<li><div class="svelte-1jb6swk"><!></div></li>');
function y(d, s) {
  let l = m(void 0);
  var e = B();
  let o;
  var v = a(e), f = a(v);
  h(f, { invisible: true, get onclick() {
    return s.onclick;
  }, get ref() {
    return b(l);
  }, set ref(t) {
    p(l, t, true);
  }, children: (t, L) => {
    var r = w(), u = a(r);
    _(u, () => s.children), i(r), c(t, r);
  }, $$slots: { default: true } }), i(v), i(e), k((t) => {
    g(e, "aria-current", s.selected ? "page" : "false"), o = j(e, 1, "svelte-1jb6swk", null, o, t);
  }, [() => ({ pictureLeft: s.pictureLeft })]), c(d, e);
}
export {
  y as N
};
