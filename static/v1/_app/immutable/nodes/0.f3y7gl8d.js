import { t as n, a as e, d as f } from "../chunks/BxmJRzoY.js";
import { p as u, j as v, a0 as b, s as _, f as s, a as h, l as g, k } from "../chunks/w0HvPX0p.js";
import { i as y } from "../chunks/iO9_dPNE.js";
import { i as j, s as S } from "../chunks/DM69BKKN.js";
import { i as C } from "../chunks/N6FgGI8m.js";
const E = true, M = "never", R = Object.freeze(Object.defineProperty({ __proto__: null, prerender: E, trailingSlash: M }, Symbol.toStringTag, { value: "Module" }));
var O = n("<div><h1>Cookies disabled</h1> <p>You need to enable Cookies.<br> Without them, a safe interaction with Rauthy is not possible.</p></div>"), w = n("<noscript></noscript> <!>", 1);
function T(l, o) {
  u(o, true), j(), C();
  let i = v(true);
  b(() => {
    g(i, navigator.cookieEnabled, true);
  });
  var r = w(), p = _(s(r), 2);
  {
    var m = (t) => {
      var a = f(), d = s(a);
      S(d, () => o.children), e(t, a);
    }, c = (t) => {
      var a = O();
      e(t, a);
    };
    y(p, (t) => {
      k(i) ? t(m) : t(c, false);
    });
  }
  e(l, r), h();
}
export {
  T as component,
  R as universal
};
