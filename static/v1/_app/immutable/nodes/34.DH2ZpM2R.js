import { t as k, a as c, e as F } from "../chunks/BxmJRzoY.js";
import { p as G, j as M, f as P, a as H, t as _, l as E, k as l, s as e, a2 as J, c as t, r as a, a3 as K } from "../chunks/w0HvPX0p.js";
import { h as Q, s as r } from "../chunks/BzP2S3Z_.js";
import { L as R } from "../chunks/D9_8lD-s.js";
import { B as U } from "../chunks/C8YTstTD.js";
import { M as V } from "../chunks/DKM0QPz5.js";
import { C as X } from "../chunks/QNragXLc.js";
import { u as Y } from "../chunks/0cG6LBdy.js";
import { T as I } from "../chunks/BdDmaO9S.js";
import { w as Z, x as ee } from "../chunks/B21bTIl7.js";
import { T as te } from "../chunks/B1f0afjj.js";
var ae = k('<div class="container svelte-1g19pwu"><h1> </h1> <p class="svelte-1g19pwu"> <br> <b> </b> <b> </b></p> <p class="svelte-1g19pwu"> </p> <div class="btn svelte-1g19pwu"><!></div></div> <!> <!>', 1), re = k("<!> <!> <!>", 1);
function fe(A, B) {
  G(B, true);
  let o = Y(), i = M("old@mail.org"), n = M("new@mail.org");
  var g = re();
  Q((s) => {
    _(() => J.title = o.emailChange.title || "E-Mail Change Confirm");
  });
  var h = P(g);
  I(h, { id: Z, get value() {
    return l(i);
  }, set value(s) {
    E(i, s, true);
  } });
  var f = e(h, 2);
  I(f, { id: ee, get value() {
    return l(n);
  }, set value(s) {
    E(n, s, true);
  } });
  var N = e(f, 2);
  V(N, { children: (s, oe) => {
    X(s, { children: (O, se) => {
      var x = ae(), m = P(x), v = t(m), S = t(v, true);
      a(v);
      var p = e(v, 2), $ = t(p), u = e($, 3), j = t(u, true);
      a(u);
      var b = e(u), C = e(b), z = t(C, true);
      a(C), a(p);
      var d = e(p, 2), D = t(d, true);
      a(d);
      var w = e(d, 2), W = t(w);
      U(W, { onclick: () => window.location.replace("/auth/v1/account"), children: (y, le) => {
        K();
        var T = F();
        _(() => r(T, o.authorize.login)), c(y, T);
      }, $$slots: { default: true } }), a(w), a(m);
      var L = e(m, 2);
      te(L, { absolute: true });
      var q = e(L, 2);
      R(q, { absolute: true }), _(() => {
        r(S, o.emailChange.title), r($, `${o.emailChange.textChanged ?? ""}:`), r(j, l(i)), r(b, ` ${o.emailChange.to ?? ""} `), r(z, l(n)), r(D, o.emailChange.textLogin);
      }), c(O, x);
    } });
  } }), c(A, g), H();
}
export {
  fe as component
};
