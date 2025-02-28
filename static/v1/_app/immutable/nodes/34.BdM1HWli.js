import { t as y, a as c, e as G } from "../chunks/BH6NCLk-.js";
import { p as H, f as M, a as J, t as _, l as P, k as E, j as i, s as e, a6 as K, c as t, r as a, a7 as Q } from "../chunks/CvlvO1XB.js";
import { h as R, s as r } from "../chunks/CTI4QPiR.js";
import { p as I } from "../chunks/Wh68IIk2.js";
import { L as U } from "../chunks/BAqymMo_.js";
import { B as V } from "../chunks/DMkkW5Nn.js";
import { M as X } from "../chunks/Ds6bXi0i.js";
import { C as Y } from "../chunks/GWgockW8.js";
import { u as Z } from "../chunks/BQ1-pLIs.js";
import { T as k } from "../chunks/DcJo-CQ6.js";
import { x as ee, y as te } from "../chunks/BaE7H8ny.js";
import { T as ae } from "../chunks/CVHA2fPQ.js";
var re = y('<div class="container svelte-1g19pwu"><h1> </h1> <p class="svelte-1g19pwu"> <br> <b> </b> <b> </b></p> <p class="svelte-1g19pwu"> </p> <div class="btn svelte-1g19pwu"><!></div></div> <!> <!>', 1), oe = y("<!> <!> <!>", 1);
function $e(A, B) {
  H(B, true);
  let o = Z(), l = E("old@mail.org"), n = E("new@mail.org");
  var g = oe();
  R((s) => {
    _(() => K.title = o.emailChange.title || "E-Mail Change Confirm");
  });
  var h = M(g);
  k(h, { id: ee, get value() {
    return i(l);
  }, set value(s) {
    P(l, I(s));
  } });
  var f = e(h, 2);
  k(f, { id: te, get value() {
    return i(n);
  }, set value(s) {
    P(n, I(s));
  } });
  var N = e(f, 2);
  X(N, { children: (s, se) => {
    Y(s, { children: (O, ie) => {
      var x = re(), m = M(x), p = t(m), S = t(p, true);
      a(p);
      var v = e(p, 2), $ = t(v), u = e($, 3), j = t(u, true);
      a(u);
      var b = e(u), C = e(b), z = t(C, true);
      a(C), a(v);
      var d = e(v, 2), D = t(d, true);
      a(d);
      var w = e(d, 2), W = t(w);
      V(W, { onclick: () => window.location.replace("/auth/v1/account"), children: (F, le) => {
        Q();
        var T = G();
        _(() => r(T, o.authorize.login)), c(F, T);
      }, $$slots: { default: true } }), a(w), a(m);
      var L = e(m, 2);
      ae(L, { absolute: true });
      var q = e(L, 2);
      U(q, { absolute: true }), _(() => {
        r(S, o.emailChange.title), r($, `${o.emailChange.textChanged ?? ""}:`), r(j, i(l)), r(b, ` ${o.emailChange.to ?? ""} `), r(z, i(n)), r(D, o.emailChange.textLogin);
      }), c(O, x);
    } });
  } }), c(A, g), J();
}
export {
  $e as component
};
