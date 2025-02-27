import { t as E, a as l, e as S } from "../chunks/D8nUqfqE.js";
import { p as J, aa as j, f as q, a as Q, t as n, l as y, k as M, j as u, s as o, a6 as U, c, r as m, a7 as B } from "../chunks/D-nwkJyM.js";
import { h as V, s as p } from "../chunks/DmLjbmU6.js";
import { i as W } from "../chunks/C3XMhfdI.js";
import { p as D } from "../chunks/BiI21XkS.js";
import { q as X, e as Y, t as Z } from "../chunks/DppGgfa0.js";
import { B as I } from "../chunks/BUAPoI3e.js";
import { u as tt } from "../chunks/CMjKUQkH.js";
import { M as ot } from "../chunks/C-ghU9Ac.js";
import { C as et } from "../chunks/BQuV3eOZ.js";
import { L as at } from "../chunks/DJrws3yD.js";
import { T as rt } from "../chunks/Bzn2dU6j.js";
import { u as _ } from "../chunks/C6Ehfmfa.js";
import { T as st } from "../chunks/DmGHSNVM.js";
import { b as it } from "../chunks/emZtalxW.js";
import { u as lt } from "../chunks/BO-Hjil0.js";
var nt = E('<div class="container svelte-7qsjv3"><h1> </h1> <p> </p> <div class="btn svelte-7qsjv3"><!> <!></div> <!></div> <!> <!>', 1), ut = E("<!> <!>", 1);
function Ct(F, K) {
  J(K, true);
  let e = tt(), h = M(false), s = M(""), $ = D({ post_logout_redirect_uri: _("post_logout_redirect_uri").get(), id_token_hint: _("id_token_hint").get(), state: _("state").get() });
  j(() => {
    u(s) && Y(u(s));
  }), j(() => {
    $.id_token_hint && x();
  });
  async function k() {
    window.location.replace("/auth/v1");
  }
  async function x() {
    y(h, true);
    let t = "/auth/v1/oidc/logout";
    lt().get() && (t = "/auth/v1/dev/logout");
    let L = await it(t, $, "form");
    Z();
    let a = L.headers.get("location");
    console.log("loc", a), a ? window.location.replace(a) : await k();
  }
  var T = ut();
  V((t) => {
    n(() => U.title = (e == null ? void 0 : e.logout.logout) || "Logout");
  });
  var w = q(T);
  rt(w, { id: X, get value() {
    return u(s);
  }, set value(t) {
    y(s, D(t));
  } });
  var N = o(w, 2);
  ot(N, { children: (t, L) => {
    et(t, { children: (a, ct) => {
      var P = nt(), d = q(P), f = c(d), O = c(f, true);
      m(f);
      var v = o(f, 2), R = c(v, true);
      m(v);
      var g = o(v, 2), b = c(g);
      I(b, { onclick: x, get isLoading() {
        return u(h);
      }, children: (i, H) => {
        B();
        var r = S();
        n(() => p(r, e.logout.logout)), l(i, r);
      }, $$slots: { default: true } });
      var z = o(b, 2);
      I(z, { level: 3, onclick: k, children: (i, H) => {
        B();
        var r = S();
        n(() => p(r, e.common.cancel)), l(i, r);
      }, $$slots: { default: true } }), m(g);
      var A = o(g, 2);
      W(A, (i) => {
      }), m(d);
      var C = o(d, 2);
      st(C, { absolute: true });
      var G = o(C, 2);
      at(G, { absolute: true }), n(() => {
        p(O, e.logout.logout), p(R, e.logout.confirmMsg);
      }), l(a, P);
    } });
  } }), l(F, T), Q();
}
export {
  Ct as component
};
