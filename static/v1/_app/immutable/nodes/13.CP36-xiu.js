import { t as y, a as n, e as p } from "../chunks/BH6NCLk-.js";
import { p as ie, k, a5 as le, s as o, f as N, t as v, a as ne, j as a, c as u, l as m, r as c, a7 as f } from "../chunks/CvlvO1XB.js";
import { s } from "../chunks/CTI4QPiR.js";
import { i as h } from "../chunks/BUO_AUgz.js";
import { e as ve } from "../chunks/BpWRzPRQ.js";
import { p as I } from "../chunks/Wh68IIk2.js";
import { B as de } from "../chunks/DMkkW5Nn.js";
import { b as me, f as pe } from "../chunks/bbkAiDd0.js";
import { E as ue } from "../chunks/BlGzyD-C.js";
import { L as _ } from "../chunks/CE2_6siz.js";
import { u as ce } from "../chunks/D8mHI_K9.js";
import { I as fe } from "../chunks/Nks81rMs.js";
var _e = y('<div class="font-mono"> </div>'), $e = y('<div class="n svelte-8tua2b"> </div>'), xe = y("<!> <!> <!> <!> <!> <!> <!>", 1), he = y('<div class="err"> </div>'), be = y('<h2>Json Web Keys</h2> <p> </p> <p> </p> <p> </p> <!> <div class="btn svelte-8tua2b"><!></div> <!> <!>', 1);
function ge(K, O) {
  ie(O, true);
  let $ = ce(), D = k(I([])), L = k(false), b = k(""), C = k(false);
  le(() => {
    G();
  });
  async function G() {
    var _a;
    let r = await pe("/auth/v1/oidc/certs");
    r.body ? m(D, I(r.body.keys)) : m(b, I(((_a = r.error) == null ? void 0 : _a.message) || "Error"));
  }
  async function Q() {
    m(b, ""), m(L, true);
    let r = await me("/auth/v1/oidc/rotate_jwk");
    r.error ? m(b, I(r.error.message)) : (m(C, true), setTimeout(() => {
      m(C, false);
    }, 3e3), await G()), m(L, false);
  }
  var M = be(), E = o(N(M), 2), R = u(E, true);
  c(E);
  var B = o(E, 2), U = u(B, true);
  c(B);
  var J = o(B, 2), X = u(J, true);
  c(J);
  var S = o(J, 2);
  ve(S, 17, () => a(D), (r) => r.kid, (r, t) => {
    ue(r, { summary: (A) => {
      var x = _e(), P = u(x);
      c(x), v(() => s(P, `${a(t).alg ?? ""}
                /
                ${a(t).kid ?? ""}`)), n(A, x);
    }, details: (A) => {
      var x = xe(), P = N(x);
      _(P, { label: "Key ID", mono: true, children: (e, d) => {
        f();
        var i = p();
        v(() => s(i, a(t).kid)), n(e, i);
      } });
      var V = o(P, 2);
      _(V, { get label() {
        return $.jwks.type;
      }, mono: true, children: (e, d) => {
        f();
        var i = p();
        v(() => s(i, a(t).kty)), n(e, i);
      } });
      var q = o(V, 2);
      _(q, { get label() {
        return $.jwks.alg;
      }, mono: true, children: (e, d) => {
        f();
        var i = p();
        v(() => s(i, a(t).alg)), n(e, i);
      } });
      var z = o(q, 2);
      {
        var ee = (e) => {
          _(e, { label: "Curve", mono: true, children: (d, i) => {
            f();
            var l = p();
            v(() => s(l, a(t).crv)), n(d, l);
          } });
        };
        h(z, (e) => {
          a(t).crv && e(ee);
        });
      }
      var F = o(z, 2);
      {
        var re = (e) => {
          _(e, { label: "n", mono: true, children: (d, i) => {
            var l = $e(), se = u(l, true);
            c(l), v(() => s(se, a(t).n)), n(d, l);
          } });
        };
        h(F, (e) => {
          a(t).n && e(re);
        });
      }
      var H = o(F, 2);
      {
        var te = (e) => {
          _(e, { label: "e", mono: true, children: (d, i) => {
            f();
            var l = p();
            v(() => s(l, a(t).e)), n(d, l);
          } });
        };
        h(H, (e) => {
          a(t).e && e(te);
        });
      }
      var ae = o(H, 2);
      {
        var oe = (e) => {
          _(e, { label: "x", mono: true, children: (d, i) => {
            f();
            var l = p();
            v(() => s(l, a(t).x)), n(d, l);
          } });
        };
        h(ae, (e) => {
          a(t).x && e(oe);
        });
      }
      n(A, x);
    }, $$slots: { summary: true, details: true } });
  });
  var W = o(S, 2), Y = u(W);
  de(Y, { onclick: Q, get isLoading() {
    return a(L);
  }, children: (r, t) => {
    f();
    var g = p();
    v(() => s(g, $.jwks.rotateKeys)), n(r, g);
  }, $$slots: { default: true } }), c(W);
  var T = o(W, 2);
  {
    var Z = (r) => {
      fe(r, {});
    };
    h(T, (r) => {
      a(C) && r(Z);
    });
  }
  var w = o(T, 2);
  {
    var j = (r) => {
      var t = he(), g = u(t, true);
      c(t), v(() => s(g, a(b))), n(r, t);
    };
    h(w, (r) => {
      a(b) && r(j);
    });
  }
  v(() => {
    s(R, $.jwks.p1), s(U, $.jwks.p2), s(X, $.jwks.p3);
  }), n(K, M), ne();
}
function Ge(K) {
  ge(K, {});
}
export {
  Ge as component
};
