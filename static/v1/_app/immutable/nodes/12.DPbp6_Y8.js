import { t as m, a as c, e as te } from "../chunks/BxmJRzoY.js";
import { p as ae, j as u, a1 as re, a0 as se, f as q, t as f, a as oe, s, c as r, l as n, r as a, k as p, a3 as ie } from "../chunks/w0HvPX0p.js";
import { s as o } from "../chunks/BzP2S3Z_.js";
import { i as w } from "../chunks/iO9_dPNE.js";
import { e as ne, i as pe } from "../chunks/S81raU5Y.js";
import { B as ve } from "../chunks/C4AV2CoD.js";
import { I as le } from "../chunks/CTshzOVc.js";
import { f as ce, b as ue } from "../chunks/UPFlzoow.js";
import { u as fe } from "../chunks/D12OFlGX.js";
import { O as me } from "../chunks/BgkEPAwa.js";
var de = m('<li class="font-mono"> </li>'), _e = m('<p> </p> <!> <div class="btn flex gap-05 svelte-lxghyr"><!> <!></div>', 1), ye = m('<p class="err"> </p>'), ge = m('<div class="err"> </div>'), xe = m('<h3> </h3> <p> </p> <p> </p> <p><span class="font-label"> </span> <span class="active font-mono svelte-lxghyr"> </span></p> <p> </p> <ul></ul> <p> </p> <!> <!>', 1);
function he(x, z) {
  ae(z, true);
  let v = fe(), d = u(""), _ = u(true), B = u(""), y = u(""), g = u(re([])), h = u(false);
  se(async () => {
    var _a;
    let e = await ce("/auth/v1/encryption/keys");
    e.body ? (n(g, e.body.keys, true), n(B, e.body.active, true), n(y, e.body.active, true)) : n(d, ((_a = e.error) == null ? void 0 : _a.message) || "Error", true), n(_, false);
  });
  async function D() {
    n(_, true);
    let e = { key_id: p(y) }, t = await ue("/auth/v1/encryption/migrate", e);
    t.error ? n(d, t.error.message, true) : (n(h, true), setTimeout(() => {
      n(h, false);
    }, 4e3)), n(_, false);
  }
  var E = xe(), K = q(E), F = r(K, true);
  a(K);
  var b = s(K, 2), H = r(b, true);
  a(b);
  var k = s(b, 2), J = r(k, true);
  a(k);
  var $ = s(k, 2), A = r($), Q = r(A);
  a(A);
  var O = s(A, 2), R = r(O, true);
  a(O), a($);
  var I = s($, 2), S = r(I);
  a(I);
  var L = s(I, 2);
  ne(L, 21, () => p(g), pe, (e, t) => {
    var i = de(), T = r(i, true);
    a(i), f(() => o(T, p(t))), c(e, i);
  }), a(L);
  var P = s(L, 2), U = r(P, true);
  a(P);
  var j = s(P, 2);
  {
    var V = (e) => {
      var t = _e(), i = q(t), T = r(i);
      a(i);
      var C = s(i, 2);
      me(C, { get ariaLabel() {
        return v.docs.encKeys.migrateToKey;
      }, get options() {
        return p(g);
      }, get value() {
        return p(y);
      }, set value(l) {
        n(y, l, true);
      } });
      var G = s(C, 2), M = r(G);
      ve(M, { onclick: D, get isLoading() {
        return p(_);
      }, children: (l, Ke) => {
        ie();
        var N = te();
        f(() => o(N, v.docs.encKeys.migrate)), c(l, N);
      }, $$slots: { default: true } });
      var Z = s(M, 2);
      {
        var ee = (l) => {
          le(l, {});
        };
        w(Z, (l) => {
          p(h) && l(ee);
        });
      }
      a(G), f(() => o(T, `${v.docs.encKeys.migrateToKey ?? ""}:`)), c(e, t);
    }, W = (e) => {
      var t = ye(), i = r(t, true);
      a(t), f(() => o(i, v.docs.encKeys.pNotPossible)), c(e, t);
    };
    w(j, (e) => {
      p(g).length > 1 ? e(V) : e(W, false);
    });
  }
  var X = s(j, 2);
  {
    var Y = (e) => {
      var t = ge(), i = r(t, true);
      a(t), f(() => o(i, p(d))), c(e, t);
    };
    w(X, (e) => {
      p(d) && e(Y);
    });
  }
  f(() => {
    o(F, v.docs.encKeys.header), o(H, v.docs.encKeys.p1), o(J, v.docs.encKeys.p2), o(Q, `${v.docs.encKeys.keyActive ?? ""}:`), o(R, p(B)), o(S, `${v.docs.encKeys.keysAvailable ?? ""}:`), o(U, v.docs.encKeys.p3);
  }), c(x, E), oe();
}
function Ee(x) {
  he(x, {});
}
export {
  Ee as component
};
