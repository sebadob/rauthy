import { t as y, d as A, a as e, e as f } from "../chunks/D8nUqfqE.js";
import { p as N, c as C, f as c, r as I, t as x, j as H, a9 as J, a as S, s as h, a7 as u } from "../chunks/D-nwkJyM.js";
import { s as b } from "../chunks/DmLjbmU6.js";
import { s as j } from "../chunks/ivlJJTAR.js";
import { N as K } from "../chunks/B82wSkDx.js";
import { s as L } from "../chunks/D_HYGYLR.js";
import { p as U } from "../chunks/2rFDT0Lm.js";
import { p as P } from "../chunks/BsLhmaxX.js";
import { A as q } from "../chunks/Ch0F_Zoj.js";
import { C as z } from "../chunks/C-Lg4I_M.js";
import { u as B } from "../chunks/DMJUG0wm.js";
import { u as D } from "../chunks/CMjKUQkH.js";
var E = y('<div class="a svelte-o3vxt1"><!></div>');
function v(p, t) {
  N(t, true);
  let i = U(t, "highlightWithParams", 3, false), g = J(() => i() ? `${P.route.id}${P.url.search}`.startsWith(t.href) : P.route.id === t.href.split("?")[0]);
  var a = E(), n = C(a);
  q(n, { get href() {
    return t.href;
  }, hideUnderline: true, get highlightWithParams() {
    return i();
  }, children: (_, l) => {
    var d = A(), o = c(d);
    j(o, () => t.children), e(_, d);
  }, $$slots: { default: true } }), I(a), x(() => L(a, "data-current", H(g))), e(p, a), S();
}
var F = y("<!> <!> <!> <!>", 1), G = y("<!> <!>", 1);
function at(p, t) {
  N(t, true);
  let i = D(), g = B();
  var a = G(), n = c(a);
  K(n, { width: "11rem", children: (l, d) => {
    var o = F(), m = c(o);
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
    }, $$slots: { default: true } }), e(l, o);
  }, $$slots: { default: true } });
  var _ = h(n, 2);
  z(_, { children: (l, d) => {
    var o = A(), m = c(o);
    j(m, () => t.children), e(l, o);
  } }), e(p, a), S();
}
export {
  at as component
};
