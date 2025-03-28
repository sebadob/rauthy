import { t as p, a as v } from "./BxmJRzoY.js";
import { p as P, a4 as W, c as _, r as b, t as k, k as x, a as U } from "./w0HvPX0p.js";
import { s as E } from "./DM69BKKN.js";
import { b as I, s as r } from "./BdbQ6g_y.js";
import { p as i } from "./C6GSeq7M.js";
import { p as h } from "./DgT4Z4Y9.js";
var S = p("<a><!></a>");
function w(s, e) {
  P(e, true);
  let f = i(e, "selectedStep", 3, false), g = i(e, "hideUnderline", 3, false), n = i(e, "highlight", 3, false), d = i(e, "highlightExact", 3, false), u = i(e, "highlightWithParams", 3, false), c = W(() => {
    if (f()) return "step";
    let a = h.route.id;
    if (a) {
      if (u()) {
        if (`${a}${h.url.search}`.startsWith(e.href)) return "page";
      } else if (d()) {
        if (a === e.href.split("?")[0]) return "page";
      } else if (e.highlightIncludes) {
        if (a.includes(e.highlightIncludes)) return "page";
      } else if (a && e.href.split("?")[0].endsWith(a)) return "page";
    }
  });
  var t = S();
  let l;
  var m = _(t);
  E(m, () => e.children), b(t), k((a) => {
    l = I(t, 1, "font-label svelte-vdcj5m", null, l, a), r(t, "href", e.href), r(t, "target", e.target), r(t, "aria-current", x(c)), r(t, "data-highlight", n());
  }, [() => ({ hideUnderline: g() })]), v(s, t), U();
}
export {
  w as A
};
