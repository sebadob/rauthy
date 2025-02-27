import { t as o, a } from "./BH6NCLk-.js";
import { c as s, r as i, t as v } from "./CvlvO1XB.js";
import { s as f } from "./-T201g_q.js";
import { s as d } from "./BMbqVy6X.js";
import { B as p } from "./DMkkW5Nn.js";
var u = o('<div class="svelte-1emf3rj"><!></div>'), _ = o('<li class="svelte-1emf3rj"><!></li>');
function N(l, e) {
  var t = _(), n = s(t);
  p(n, { invisible: true, get onclick() {
    return e.onclick;
  }, children: (c, h) => {
    var r = u(), m = s(r);
    f(m, () => e.children), i(r), a(c, r);
  }, $$slots: { default: true } }), i(t), v(() => d(t, "aria-current", e.selected ? "page" : "false")), a(l, t);
}
export {
  N
};
