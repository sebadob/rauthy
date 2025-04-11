import { t as I, a as l, e as S } from "../chunks/BxmJRzoY.js";
import { p as H, j, a1 as J, a5 as q, f as y, a as Q, t as n, l as M, k as u, s as e, a2 as U, c, r as m, a3 as B } from "../chunks/w0HvPX0p.js";
import { h as V, s as d } from "../chunks/BzP2S3Z_.js";
import { i as W } from "../chunks/iO9_dPNE.js";
import { o as X, e as Y, q as Z } from "../chunks/B21bTIl7.js";
import { B as D } from "../chunks/C8YTstTD.js";
import { u as tt } from "../chunks/0cG6LBdy.js";
import { M as et } from "../chunks/DKM0QPz5.js";
import { C as ot } from "../chunks/QNragXLc.js";
import { L as at } from "../chunks/D0XpefKJ.js";
import { T as rt } from "../chunks/Dhcsa8BW.js";
import { u as _ } from "../chunks/DYaEteNC.js";
import { T as st } from "../chunks/B1f0afjj.js";
import { b as it } from "../chunks/UPFlzoow.js";
import { u as lt } from "../chunks/F_Qf1tHt.js";
var nt = I('<div class="container svelte-7qsjv3"><h1> </h1> <p> </p> <div class="btn svelte-7qsjv3"><!> <!></div> <!></div> <!> <!>', 1), ut = I("<!> <!>", 1);
function bt(E, F) {
  H(F, true);
  let o = tt(), h = j(false), r = j(""), $ = J({ post_logout_redirect_uri: _("post_logout_redirect_uri").get(), id_token_hint: _("id_token_hint").get(), state: _("state").get() });
  q(() => {
    u(r) && Y(u(r));
  }), q(() => {
    $.id_token_hint && x();
  });
  async function k() {
    window.location.replace("/auth/v1");
  }
  async function x() {
    M(h, true);
    let t = "/auth/v1/oidc/logout";
    lt().get() && (t = "/auth/v1/dev/logout");
    let L = await it(t, $, "form");
    Z();
    let s = L.headers.get("location");
    s ? window.location.replace(s) : await k();
  }
  var T = ut();
  V((t) => {
    n(() => U.title = (o == null ? void 0 : o.logout.logout) || "Logout");
  });
  var w = y(T);
  rt(w, { id: X, get value() {
    return u(r);
  }, set value(t) {
    M(r, t, true);
  } });
  var K = e(w, 2);
  et(K, { children: (t, L) => {
    ot(t, { children: (s, ct) => {
      var P = nt(), p = y(P), f = c(p), N = c(f, true);
      m(f);
      var v = e(f, 2), O = c(v, true);
      m(v);
      var g = e(v, 2), b = c(g);
      D(b, { onclick: x, get isLoading() {
        return u(h);
      }, children: (i, G) => {
        B();
        var a = S();
        n(() => d(a, o.logout.logout)), l(i, a);
      }, $$slots: { default: true } });
      var R = e(b, 2);
      D(R, { level: 3, onclick: k, children: (i, G) => {
        B();
        var a = S();
        n(() => d(a, o.common.cancel)), l(i, a);
      }, $$slots: { default: true } }), m(g);
      var z = e(g, 2);
      W(z, (i) => {
      }), m(p);
      var C = e(p, 2);
      st(C, { absolute: true });
      var A = e(C, 2);
      at(A, { absolute: true }), n(() => {
        d(N, o.logout.logout), d(O, o.logout.confirmMsg);
      }), l(s, P);
    } });
  } }), l(E, T), Q();
}
export {
  bt as component
};
