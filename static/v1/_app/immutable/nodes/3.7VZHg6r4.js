import { t as y, d as A, a as e, e as f } from "../chunks/BH6NCLk-.js";
import { p as N, c as C, f as p, r as I, t as x, j as H, a9 as J, a as S, s as h, a7 as u } from "../chunks/CvlvO1XB.js";
import { s as b } from "../chunks/CTI4QPiR.js";
import { s as j } from "../chunks/-T201g_q.js";
import { N as K } from "../chunks/G5KuhWXq.js";
import { s as L } from "../chunks/BMbqVy6X.js";
import { p as T } from "../chunks/C6SR4G2t.js";
import { p as P } from "../chunks/u-vgjdTr.js";
import { A as U } from "../chunks/8APYSIIx.js";
import { C as q } from "../chunks/BnPoFdx3.js";
import { u as z } from "../chunks/D8mHI_K9.js";
import { u as B } from "../chunks/CgEHB2u3.js";
var D = y('<div class="a svelte-o3vxt1"><!></div>');
function v(c, t) {
  N(t, true);
  let i = T(t, "highlightWithParams", 3, false), g = J(() => i() ? `${P.route.id}${P.url.search}`.startsWith(t.href) : P.route.id === t.href.split("?")[0]);
  var a = D(), n = C(a);
  U(n, { get href() {
    return t.href;
  }, hideUnderline: true, get highlightWithParams() {
    return i();
  }, children: (_, d) => {
    var l = A(), o = p(l);
    j(o, () => t.children), e(_, l);
  }, $$slots: { default: true } }), I(a), x(() => L(a, "data-current", H(g))), e(c, a), S();
}
var E = y("<!> <!> <!> <!>", 1), F = y("<!> <!>", 1);
function at(c, t) {
  N(t, true);
  let i = B(), g = z();
  var a = F(), n = p(a);
  K(n, { width: "11rem", paddingTop: "9.5rem", children: (d, l) => {
    var o = E(), m = p(o);
    v(m, { href: "/auth/v1/admin/config/policy", children: (s, $) => {
      u();
      var r = f();
      x(() => b(r, i.passwordPolicy.passwordPolicy)), e(s, r);
    }, $$slots: { default: true } });
    var w = h(m, 2);
    v(w, { href: "/auth/v1/admin/config/jwks", children: (s, $) => {
      u();
      var r = f("JWKS");
      e(s, r);
    }, $$slots: { default: true } });
    var W = h(w, 2);
    v(W, { href: "/auth/v1/admin/config/argon2", children: (s, $) => {
      u();
      var r = f("Password Hashing");
      e(s, r);
    }, $$slots: { default: true } });
    var k = h(W, 2);
    v(k, { href: "/auth/v1/admin/config/encryption", children: (s, $) => {
      u();
      var r = f();
      x(() => b(r, g.docs.encryption)), e(s, r);
    }, $$slots: { default: true } }), e(d, o);
  }, $$slots: { default: true } });
  var _ = h(n, 2);
  q(_, { children: (d, l) => {
    var o = A(), m = p(o);
    j(m, () => t.children), e(d, o);
  } }), e(c, a), S();
}
export {
  at as component
};
