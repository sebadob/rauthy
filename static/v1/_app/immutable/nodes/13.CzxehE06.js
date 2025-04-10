import { t as y, a as n, e as f } from "../chunks/BxmJRzoY.js";
import { p as ot, j as k, a1 as lt, a0 as it, s, f as H, t as v, a as nt, c as m, l as d, r as p, k as a, a3 as c } from "../chunks/w0HvPX0p.js";
import { s as o } from "../chunks/BzP2S3Z_.js";
import { i as h } from "../chunks/iO9_dPNE.js";
import { e as vt } from "../chunks/S81raU5Y.js";
import { B as ut } from "../chunks/C8YTstTD.js";
import { f as dt, b as ft } from "../chunks/UPFlzoow.js";
import { E as mt } from "../chunks/755dAfGH.js";
import { L as _ } from "../chunks/DQwTjYxX.js";
import { u as pt } from "../chunks/Bt_9UXew.js";
import { I as ct } from "../chunks/CTshzOVc.js";
var _t = y('<div class="font-mono"> </div>'), $t = y('<div class="n svelte-8tua2b"> </div>'), xt = y("<!> <!> <!> <!> <!> <!> <!>", 1), ht = y('<div class="err"> </div>'), bt = y('<h2>Json Web Keys</h2> <p> </p> <p> </p> <p> </p> <!> <div class="btn flex gap-05 svelte-8tua2b"><!> <!></div> <!>', 1);
function gt(I, N) {
  ot(N, true);
  let $ = pt(), A = k(lt([])), K = k(false), b = k(""), L = k(false);
  it(() => {
    D();
  });
  async function D() {
    var _a;
    let e = await dt("/auth/v1/oidc/certs");
    e.body ? d(A, e.body.keys, true) : d(b, ((_a = e.error) == null ? void 0 : _a.message) || "Error", true);
  }
  async function O() {
    d(b, ""), d(K, true);
    let e = await ft("/auth/v1/oidc/rotate_jwk");
    e.error ? d(b, e.error.message, true) : (d(L, true), setTimeout(() => {
      d(L, false);
    }, 3e3), await D()), d(K, false);
  }
  var G = bt(), C = s(H(G), 2), Q = m(C, true);
  p(C);
  var E = s(C, 2), R = m(E, true);
  p(E);
  var B = s(E, 2), U = m(B, true);
  p(B);
  var M = s(B, 2);
  vt(M, 17, () => a(A), (e) => e.kid, (e, r) => {
    mt(e, { summary: (W) => {
      var x = _t(), P = m(x);
      p(x), v(() => o(P, `${a(r).alg ?? ""}
                /
                ${a(r).kid ?? ""}`)), n(W, x);
    }, details: (W) => {
      var x = xt(), P = H(x);
      _(P, { label: "Key ID", mono: true, children: (t, u) => {
        c();
        var l = f();
        v(() => o(l, a(r).kid)), n(t, l);
      }, $$slots: { default: true } });
      var T = s(P, 2);
      _(T, { get label() {
        return $.jwks.type;
      }, mono: true, children: (t, u) => {
        c();
        var l = f();
        v(() => o(l, a(r).kty)), n(t, l);
      }, $$slots: { default: true } });
      var V = s(T, 2);
      _(V, { get label() {
        return $.jwks.alg;
      }, mono: true, children: (t, u) => {
        c();
        var l = f();
        v(() => o(l, a(r).alg)), n(t, l);
      }, $$slots: { default: true } });
      var q = s(V, 2);
      {
        var j = (t) => {
          _(t, { label: "Curve", mono: true, children: (u, l) => {
            c();
            var i = f();
            v(() => o(i, a(r).crv)), n(u, i);
          }, $$slots: { default: true } });
        };
        h(q, (t) => {
          a(r).crv && t(j);
        });
      }
      var z = s(q, 2);
      {
        var tt = (t) => {
          _(t, { label: "n", mono: true, children: (u, l) => {
            var i = $t(), st = m(i, true);
            p(i), v(() => o(st, a(r).n)), n(u, i);
          }, $$slots: { default: true } });
        };
        h(z, (t) => {
          a(r).n && t(tt);
        });
      }
      var F = s(z, 2);
      {
        var et = (t) => {
          _(t, { label: "e", mono: true, children: (u, l) => {
            c();
            var i = f();
            v(() => o(i, a(r).e)), n(u, i);
          }, $$slots: { default: true } });
        };
        h(F, (t) => {
          a(r).e && t(et);
        });
      }
      var rt = s(F, 2);
      {
        var at = (t) => {
          _(t, { label: "x", mono: true, children: (u, l) => {
            c();
            var i = f();
            v(() => o(i, a(r).x)), n(u, i);
          }, $$slots: { default: true } });
        };
        h(rt, (t) => {
          a(r).x && t(at);
        });
      }
      n(W, x);
    }, $$slots: { summary: true, details: true } });
  });
  var J = s(M, 2), S = m(J);
  ut(S, { onclick: O, get isLoading() {
    return a(K);
  }, children: (e, r) => {
    c();
    var g = f();
    v(() => o(g, $.jwks.rotateKeys)), n(e, g);
  }, $$slots: { default: true } });
  var X = s(S, 2);
  {
    var Y = (e) => {
      ct(e, {});
    };
    h(X, (e) => {
      a(L) && e(Y);
    });
  }
  p(J);
  var Z = s(J, 2);
  {
    var w = (e) => {
      var r = ht(), g = m(r, true);
      p(r), v(() => o(g, a(b))), n(e, r);
    };
    h(Z, (e) => {
      a(b) && e(w);
    });
  }
  v(() => {
    o(Q, $.jwks.p1), o(R, $.jwks.p2), o(U, $.jwks.p3);
  }), n(I, G), nt();
}
function Dt(I) {
  gt(I, {});
}
export {
  Dt as component
};
