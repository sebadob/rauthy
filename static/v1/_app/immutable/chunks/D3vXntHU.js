import { t as p, a as v } from "./DLBGyKVC.js";
import { p as P, c as W, r as b, t as x, j as U, a9 as _, a as j } from "./CmQi0fbH.js";
import { s as k } from "./71gCR-8F.js";
import { b as E, s as r } from "./DOlUUCkJ.js";
import { p as i } from "./DNJm3-SG.js";
import { p as h } from "./CWypd1E1.js";
var I = p("<a><!></a>");
function w(s, e) {
  P(e, true);
  let f = i(e, "selectedStep", 3, false), g = i(e, "hideUnderline", 3, false), n = i(e, "highlight", 3, false), d = i(e, "highlightExact", 3, false), u = i(e, "highlightWithParams", 3, false), c = _(() => {
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
  var t = I();
  let l;
  var m = W(t);
  k(m, () => e.children), b(t), x((a) => {
    l = E(t, 1, "font-label svelte-vdcj5m", null, l, a), r(t, "href", e.href), r(t, "target", e.target), r(t, "aria-current", U(c)), r(t, "data-highlight", n());
  }, [() => ({ hideUnderline: g() })]), v(s, t), j();
}
export {
  w as A
};
