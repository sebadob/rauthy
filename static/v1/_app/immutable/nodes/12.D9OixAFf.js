import { t as u, a as f, e as ae } from "../chunks/YHhP1LbZ.js";
import { p as re, k as m, a6 as se, f as z, t as d, a as oe, s, c as r, l as n, r as a, j as p, a8 as ie } from "../chunks/Ck6jKiur.js";
import { s as o } from "../chunks/tDAaDMC_.js";
import { i as B } from "../chunks/7JDmqCCW.js";
import { e as ne, i as pe } from "../chunks/Cj5zIR7o.js";
import { p as c } from "../chunks/ho0YXExL.js";
import { B as ve } from "../chunks/Bd2Rvcxs.js";
import { I as le } from "../chunks/CWz_piBP.js";
import { f as ce, b as fe } from "../chunks/BO1A6s0c.js";
import { u as me } from "../chunks/BM7IgWpA.js";
import { O as de } from "../chunks/By82gADu.js";
var ue = u('<li class="font-mono"> </li>'), _e = u('<p> </p> <!> <div class="btn flex gap-05 svelte-lxghyr"><!> <!></div>', 1), ye = u('<p class="err"> </p>'), ge = u('<div class="err"> </div>'), xe = u('<h3> </h3> <p> </p> <p> </p> <p><span class="font-label"> </span> <span class="active font-mono svelte-lxghyr"> </span></p> <p> </p> <ul></ul> <p> </p> <!> <!>', 1);
function he(h, D) {
  re(D, true);
  let v = me(), _ = m(""), y = m(true), E = m(""), g = m(""), x = m(c([])), K = m(false);
  se(async () => {
    var _a;
    let e = await ce("/auth/v1/encryption/keys");
    e.body ? (n(x, c(e.body.keys)), n(E, c(e.body.active)), n(g, c(e.body.active))) : n(_, c(((_a = e.error) == null ? void 0 : _a.message) || "Error")), n(y, false);
  });
  async function F() {
    n(y, true);
    let e = { key_id: p(g) }, t = await fe("/auth/v1/encryption/migrate", e);
    t.error ? n(_, c(t.error.message)) : (n(K, true), setTimeout(() => {
      n(K, false);
    }, 4e3)), n(y, false);
  }
  var O = xe(), b = z(O), H = r(b, true);
  a(b);
  var k = s(b, 2), J = r(k, true);
  a(k);
  var $ = s(k, 2), Q = r($, true);
  a($);
  var A = s($, 2), I = r(A), R = r(I);
  a(I);
  var j = s(I, 2), S = r(j, true);
  a(j), a(A);
  var L = s(A, 2), U = r(L);
  a(L);
  var P = s(L, 2);
  ne(P, 21, () => p(x), pe, (e, t) => {
    var i = ue(), w = r(i, true);
    a(i), d(() => o(w, p(t))), f(e, i);
  }), a(P);
  var T = s(P, 2), V = r(T, true);
  a(T);
  var C = s(T, 2);
  {
    var W = (e) => {
      var t = _e(), i = z(t), w = r(i);
      a(i);
      var G = s(i, 2);
      de(G, { get ariaLabel() {
        return v.docs.encKeys.migrateToKey;
      }, get options() {
        return p(x);
      }, get value() {
        return p(g);
      }, set value(l) {
        n(g, c(l));
      } });
      var M = s(G, 2), N = r(M);
      ve(N, { onclick: F, get isLoading() {
        return p(y);
      }, children: (l, Ke) => {
        ie();
        var q = ae();
        d(() => o(q, v.docs.encKeys.migrate)), f(l, q);
      }, $$slots: { default: true } });
      var ee = s(N, 2);
      {
        var te = (l) => {
          le(l, {});
        };
        B(ee, (l) => {
          p(K) && l(te);
        });
      }
      a(M), d(() => o(w, `${v.docs.encKeys.migrateToKey ?? ""}:`)), f(e, t);
    }, X = (e) => {
      var t = ye(), i = r(t, true);
      a(t), d(() => o(i, v.docs.encKeys.pNotPossible)), f(e, t);
    };
    B(C, (e) => {
      p(x).length > 1 ? e(W) : e(X, false);
    });
  }
  var Y = s(C, 2);
  {
    var Z = (e) => {
      var t = ge(), i = r(t, true);
      a(t), d(() => o(i, p(_))), f(e, t);
    };
    B(Y, (e) => {
      p(_) && e(Z);
    });
  }
  d(() => {
    o(H, v.docs.encKeys.header), o(J, v.docs.encKeys.p1), o(Q, v.docs.encKeys.p2), o(R, `${v.docs.encKeys.keyActive ?? ""}:`), o(S, p(E)), o(U, `${v.docs.encKeys.keysAvailable ?? ""}:`), o(V, v.docs.encKeys.p3);
  }), f(h, O), oe();
}
function Oe(h) {
  he(h, {});
}
export {
  Oe as component
};
