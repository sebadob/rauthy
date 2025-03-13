import { t as y, a as n, e as f } from "../chunks/YHhP1LbZ.js";
import { p as lt, k, a6 as it, s, f as N, t as v, a as nt, c as m, l as u, r as p, j as a, a8 as c } from "../chunks/Ck6jKiur.js";
import { s as o } from "../chunks/tDAaDMC_.js";
import { i as h } from "../chunks/7JDmqCCW.js";
import { e as vt } from "../chunks/Cj5zIR7o.js";
import { p as I } from "../chunks/ho0YXExL.js";
import { B as dt } from "../chunks/Bd2Rvcxs.js";
import { f as ut, b as ft } from "../chunks/BO1A6s0c.js";
import { E as mt } from "../chunks/CKFrqqP0.js";
import { L as _ } from "../chunks/Sykf3ifF.js";
import { u as pt } from "../chunks/BM7IgWpA.js";
import { I as ct } from "../chunks/CWz_piBP.js";
var _t = y('<div class="font-mono"> </div>'), $t = y('<div class="n svelte-8tua2b"> </div>'), xt = y("<!> <!> <!> <!> <!> <!> <!>", 1), ht = y('<div class="err"> </div>'), bt = y('<h2>Json Web Keys</h2> <p> </p> <p> </p> <p> </p> <!> <div class="btn flex gap-05 svelte-8tua2b"><!> <!></div> <!>', 1);
function gt(K, O) {
  lt(O, true);
  let $ = pt(), D = k(I([])), L = k(false), b = k(""), C = k(false);
  it(() => {
    G();
  });
  async function G() {
    var _a;
    let e = await ut("/auth/v1/oidc/certs");
    e.body ? u(D, I(e.body.keys)) : u(b, I(((_a = e.error) == null ? void 0 : _a.message) || "Error"));
  }
  async function Q() {
    u(b, ""), u(L, true);
    let e = await ft("/auth/v1/oidc/rotate_jwk");
    e.error ? u(b, I(e.error.message)) : (u(C, true), setTimeout(() => {
      u(C, false);
    }, 3e3), await G()), u(L, false);
  }
  var M = bt(), E = s(N(M), 2), R = m(E, true);
  p(E);
  var B = s(E, 2), U = m(B, true);
  p(B);
  var J = s(B, 2), X = m(J, true);
  p(J);
  var S = s(J, 2);
  vt(S, 17, () => a(D), (e) => e.kid, (e, r) => {
    mt(e, { summary: (A) => {
      var x = _t(), P = m(x);
      p(x), v(() => o(P, `${a(r).alg ?? ""}
                /
                ${a(r).kid ?? ""}`)), n(A, x);
    }, details: (A) => {
      var x = xt(), P = N(x);
      _(P, { label: "Key ID", mono: true, children: (t, d) => {
        c();
        var l = f();
        v(() => o(l, a(r).kid)), n(t, l);
      }, $$slots: { default: true } });
      var V = s(P, 2);
      _(V, { get label() {
        return $.jwks.type;
      }, mono: true, children: (t, d) => {
        c();
        var l = f();
        v(() => o(l, a(r).kty)), n(t, l);
      }, $$slots: { default: true } });
      var q = s(V, 2);
      _(q, { get label() {
        return $.jwks.alg;
      }, mono: true, children: (t, d) => {
        c();
        var l = f();
        v(() => o(l, a(r).alg)), n(t, l);
      }, $$slots: { default: true } });
      var z = s(q, 2);
      {
        var tt = (t) => {
          _(t, { label: "Curve", mono: true, children: (d, l) => {
            c();
            var i = f();
            v(() => o(i, a(r).crv)), n(d, i);
          }, $$slots: { default: true } });
        };
        h(z, (t) => {
          a(r).crv && t(tt);
        });
      }
      var F = s(z, 2);
      {
        var et = (t) => {
          _(t, { label: "n", mono: true, children: (d, l) => {
            var i = $t(), ot = m(i, true);
            p(i), v(() => o(ot, a(r).n)), n(d, i);
          }, $$slots: { default: true } });
        };
        h(F, (t) => {
          a(r).n && t(et);
        });
      }
      var H = s(F, 2);
      {
        var rt = (t) => {
          _(t, { label: "e", mono: true, children: (d, l) => {
            c();
            var i = f();
            v(() => o(i, a(r).e)), n(d, i);
          }, $$slots: { default: true } });
        };
        h(H, (t) => {
          a(r).e && t(rt);
        });
      }
      var at = s(H, 2);
      {
        var st = (t) => {
          _(t, { label: "x", mono: true, children: (d, l) => {
            c();
            var i = f();
            v(() => o(i, a(r).x)), n(d, i);
          }, $$slots: { default: true } });
        };
        h(at, (t) => {
          a(r).x && t(st);
        });
      }
      n(A, x);
    }, $$slots: { summary: true, details: true } });
  });
  var W = s(S, 2), T = m(W);
  dt(T, { onclick: Q, get isLoading() {
    return a(L);
  }, children: (e, r) => {
    c();
    var g = f();
    v(() => o(g, $.jwks.rotateKeys)), n(e, g);
  }, $$slots: { default: true } });
  var Y = s(T, 2);
  {
    var Z = (e) => {
      ct(e, {});
    };
    h(Y, (e) => {
      a(C) && e(Z);
    });
  }
  p(W);
  var w = s(W, 2);
  {
    var j = (e) => {
      var r = ht(), g = m(r, true);
      p(r), v(() => o(g, a(b))), n(e, r);
    };
    h(w, (e) => {
      a(b) && e(j);
    });
  }
  v(() => {
    o(R, $.jwks.p1), o(U, $.jwks.p2), o(X, $.jwks.p3);
  }), n(K, M), nt();
}
function Gt(K) {
  gt(K, {});
}
export {
  Gt as component
};
