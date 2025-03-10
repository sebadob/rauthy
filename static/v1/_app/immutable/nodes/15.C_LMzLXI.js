import { t as D, e as c, a as n } from "../chunks/BH6NCLk-.js";
import { p as G, a as N, f as R, c as s, r as p, s as t, a7 as h, t as S } from "../chunks/CvlvO1XB.js";
import { s as r } from "../chunks/CTI4QPiR.js";
import { C as U } from "../chunks/BnPoFdx3.js";
import { A as v } from "../chunks/BqWmWZH3.js";
import { u as j } from "../chunks/D8mHI_K9.js";
var q = D("<h1> </h1> <p> <!></p> <p> <!> <br> <b> </b> </p> <p> <!></p>", 1);
function z(i, P) {
  G(P, true);
  let a = j();
  U(i, { children: (y, E) => {
    var _ = q(), d = R(_), A = s(d, true);
    p(d);
    var l = t(d, 2), $ = s(l), w = t($);
    v(w, { href: "https://sebadob.github.io/rauthy/", target: "_blank", children: (e, k) => {
      h();
      var o = c("Rauthy Book");
      n(e, o);
    }, $$slots: { default: true } }), p(l);
    var m = t(l, 2), f = s(m), b = t(f);
    v(b, { href: "/docs/v1/swagger-ui/", target: "_blank", children: (e, k) => {
      h();
      var o = c("Swagger UI");
      n(e, o);
    }, $$slots: { default: true } });
    var u = t(b, 4), C = s(u);
    p(u);
    var I = t(u);
    p(m);
    var x = t(m, 2), g = s(x), B = t(g);
    v(B, { href: "https://github.com/sebadob/rauthy", target: "_blank", children: (e, k) => {
      h();
      var o = c("Github");
      n(e, o);
    }, $$slots: { default: true } }), p(x), S(() => {
      r(A, a.nav.docs), r($, `${a.docs.book ?? ""} `), r(f, `${a.docs.openapi ?? ""} `), r(C, `${a.common.note ?? ""}:`), r(I, ` ${a.docs.openapiNote ?? ""}`), r(g, `${a.docs.source ?? ""}: `);
    }), n(y, _);
  } }), N();
}
function O(i) {
  z(i, {});
}
export {
  O as component
};
