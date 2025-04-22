import { t as y, d as k, a as e, e as f } from "../chunks/BxmJRzoY.js";
import { p as A, a4 as I, c as j, f as p, r as H, t as x, k as J, a as N, s as h, a3 as u } from "../chunks/w0HvPX0p.js";
import { s as b } from "../chunks/BzP2S3Z_.js";
import { s as S } from "../chunks/DM69BKKN.js";
import { N as K } from "../chunks/Bmyn6g0M.js";
import { s as L } from "../chunks/BdbQ6g_y.js";
import { p as T } from "../chunks/C6GSeq7M.js";
import { p as P } from "../chunks/DlEiqYXV.js";
import { A as U } from "../chunks/IF8gMcJ7.js";
import { C as q } from "../chunks/C6jTHtu1.js";
import { u as z } from "../chunks/D12OFlGX.js";
import { u as B } from "../chunks/N6FgGI8m.js";
var D = y('<div class="a svelte-o3vxt1"><!></div>');
function v(c, t) {
  A(t, true);
  let i = T(t, "highlightWithParams", 3, false), g = I(() => i() ? `${P.route.id}${P.url.search}`.startsWith(t.href) : P.route.id === t.href.split("?")[0]);
  var a = D(), n = j(a);
  U(n, { get href() {
    return t.href;
  }, hideUnderline: true, get highlightWithParams() {
    return i();
  }, children: (_, d) => {
    var l = k(), o = p(l);
    S(o, () => t.children), e(_, l);
  }, $$slots: { default: true } }), H(a), x(() => L(a, "data-current", J(g))), e(c, a), N();
}
var E = y("<!> <!> <!> <!>", 1), F = y("<!> <!>", 1);
function at(c, t) {
  A(t, true);
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
    var C = h(W, 2);
    v(C, { href: "/auth/v1/admin/config/encryption", children: (s, $) => {
      u();
      var r = f();
      x(() => b(r, g.docs.encryption)), e(s, r);
    }, $$slots: { default: true } }), e(d, o);
  }, $$slots: { default: true } });
  var _ = h(n, 2);
  q(_, { children: (d, l) => {
    var o = k(), m = p(o);
    S(m, () => t.children), e(d, o);
  } }), e(c, a), N();
}
export {
  at as component
};
