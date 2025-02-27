import { t as c, a as f } from "./BH6NCLk-.js";
import { p as E, c as r, r as t, s as h, t as K, a as L } from "./CvlvO1XB.js";
import { i as P } from "./BUO_AUgz.js";
import { s as g } from "./-T201g_q.js";
import { s as i } from "./BMbqVy6X.js";
import { B as R, t as S, s as j } from "./DMkkW5Nn.js";
import { p as q } from "./C6SR4G2t.js";
import { I as w } from "./BZaoh_H_.js";
import { g as x } from "./CGXT-546.js";
import { u as z } from "./DOl8_ubJ.js";
var A = c('<div class="chevron svelte-li3f92"><!></div>'), F = c("<div><!></div>"), G = c('<div class="container svelte-li3f92"><div class="header svelte-li3f92"><!> <div class="summary svelte-li3f92"><!></div></div> <div class="details svelte-li3f92"><!></div></div>');
function Y(y, l) {
  E(l, true);
  let m = z();
  const u = x(), _ = x();
  let s = q(l, "expand", 15, false);
  function C(a) {
    a.preventDefault(), s(!s());
  }
  var n = G(), p = r(n), b = r(p);
  R(b, { get ariaLabel() {
    return m.common.expandContent;
  }, ariaControls: _, invisible: true, onclick: (a) => C(a), children: (a, o) => {
    var d = A(), D = r(d);
    w(D, {}), t(d), f(a, d);
  }, $$slots: { default: true } });
  var v = h(b, 2);
  i(v, "id", u);
  var I = r(v);
  g(I, () => l.summary), t(v), t(p);
  var e = h(p, 2);
  i(e, "id", _), i(e, "aria-labelledby", u);
  var k = r(e);
  {
    var B = (a) => {
      var o = F(), d = r(o);
      g(d, () => l.details), t(o), S(3, o, () => j, () => ({ duration: 150 })), f(a, o);
    };
    P(k, (a) => {
      s() && a(B);
    });
  }
  t(e), t(n), K(() => {
    i(n, "aria-expanded", s()), i(v, "aria-label", m.common.summary), i(e, "aria-label", m.common.details), i(e, "aria-hidden", !s());
  }), f(y, n), L();
}
export {
  Y as E
};
