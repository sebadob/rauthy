import { t as n, a as t, d } from "../chunks/DLBGyKVC.js";
import { p as v, a6 as u, s as b, f as s, a as _, l as h, k as g, j as k } from "../chunks/CmQi0fbH.js";
import { i as y } from "../chunks/C6bK2EJJ.js";
import { i as j, s as S } from "../chunks/71gCR-8F.js";
import { p as x } from "../chunks/B_ggA-0N.js";
import { i as C } from "../chunks/DGTOa5g8.js";
const E = true, M = "never", W = Object.freeze(Object.defineProperty({ __proto__: null, prerender: E, trailingSlash: M }, Symbol.toStringTag, { value: "Module" }));
var O = n("<div><h1>Cookies disabled</h1> <p>You need to enable Cookies.<br> Without them, a safe interaction with Rauthy is not possible.</p></div>"), w = n("<noscript></noscript> <!>", 1);
function Y(p, e) {
  v(e, true), j(), C();
  let r = g(true);
  u(() => {
    h(r, x(navigator.cookieEnabled));
  });
  var i = w(), l = b(s(i), 2);
  {
    var m = (o) => {
      var a = d(), c = s(a);
      S(c, () => e.children), t(o, a);
    }, f = (o) => {
      var a = O();
      t(o, a);
    };
    y(l, (o) => {
      k(r) ? o(m) : o(f, false);
    });
  }
  t(p, i), _();
}
export {
  Y as component,
  W as universal
};
