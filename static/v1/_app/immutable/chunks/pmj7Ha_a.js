import { t as v, a as c } from "./DKC5GJ29.js";
import { c as a, r as i, l as u, k as p, j as b, t as k } from "./BveBAmlr.js";
import { s as _ } from "./Dv-Q3FDX.js";
import { s as g, b as j } from "./Dql74IOz.js";
import { p as h } from "./VbPNpVtZ.js";
import { B as w } from "./DlLEcmNg.js";
var B = v('<div class="svelte-1jb6swk"><!></div>'), x = v('<li><div class="svelte-1jb6swk"><!></div></li>');
function A(f, s) {
  let l = p(void 0);
  var e = x();
  let o;
  var n = a(e), d = a(n);
  w(d, { invisible: true, get onclick() {
    return s.onclick;
  }, get ref() {
    return b(l);
  }, set ref(t) {
    u(l, h(t));
  }, children: (t, L) => {
    var r = B(), m = a(r);
    _(m, () => s.children), i(r), c(t, r);
  }, $$slots: { default: true } }), i(n), i(e), k((t) => {
    g(e, "aria-current", s.selected ? "page" : "false"), o = j(e, 1, "svelte-1jb6swk", null, o, t);
  }, [() => ({ pictureLeft: s.pictureLeft })]), c(f, e);
}
export {
  A as N
};
