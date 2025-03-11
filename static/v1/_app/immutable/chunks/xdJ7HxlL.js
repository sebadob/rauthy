import { t as v, a as c } from "./BH6NCLk-.js";
import { c as a, r as i, l as u, k as p, j as k, t as _ } from "./CvlvO1XB.js";
import { s as b } from "./-T201g_q.js";
import { s as g, a as j } from "./BMbqVy6X.js";
import { p as h } from "./Wh68IIk2.js";
import { B as w } from "./CqH8LXO-.js";
var B = v('<div class="svelte-1jb6swk"><!></div>'), x = v('<li><div class="svelte-1jb6swk"><!></div></li>');
function A(f, t) {
  let l = p(void 0);
  var e = x();
  let o;
  var n = a(e), d = a(n);
  w(d, { invisible: true, get onclick() {
    return t.onclick;
  }, get ref() {
    return k(l);
  }, set ref(s) {
    u(l, h(s));
  }, children: (s, L) => {
    var r = B(), m = a(r);
    b(m, () => t.children), i(r), c(s, r);
  }, $$slots: { default: true } }), i(n), i(e), _(() => {
    g(e, "aria-current", t.selected ? "page" : "false"), o = j(e, 1, "svelte-1jb6swk", null, o, { pictureLeft: t.pictureLeft });
  }), c(f, e);
}
export {
  A as N
};
