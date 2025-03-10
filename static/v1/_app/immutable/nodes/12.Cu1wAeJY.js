import { t as B, a as u, e as Y } from "../chunks/BH6NCLk-.js";
import { p as Z, k as v, a5 as ee, f as te, t as y, a as ae, s as a, c as r, l as s, r as t, j as i, a7 as re } from "../chunks/CvlvO1XB.js";
import { s as o } from "../chunks/CTI4QPiR.js";
import { i as G } from "../chunks/BUO_AUgz.js";
import { e as se, i as oe } from "../chunks/BpWRzPRQ.js";
import { p as l } from "../chunks/Wh68IIk2.js";
import { B as ie } from "../chunks/DMkkW5Nn.js";
import { I as ne } from "../chunks/Nks81rMs.js";
import { f as pe, b as ce } from "../chunks/BO1A6s0c.js";
import { u as le } from "../chunks/D8mHI_K9.js";
import { O as ve } from "../chunks/D1anwFsC.js";
var me = B('<li class="font-mono"> </li>'), fe = B('<div class="err"> </div>'), de = B('<h3> </h3> <p> </p> <p> </p> <p><span class="font-label"> </span> <span class="active font-mono svelte-lxghyr"> </span></p> <p> </p> <ul></ul> <p> </p> <p> </p> <!> <div class="btn flex gap-05 svelte-lxghyr"><!> <!></div> <!>', 1);
function ue(_, M) {
  Z(M, true);
  let n = le(), m = v(""), f = v(true), E = v(""), d = v(""), g = v(l([])), x = v(false);
  ee(async () => {
    var _a;
    let e = await pe("/auth/v1/encryption/keys");
    e.body ? (s(g, l(e.body.keys)), s(E, l(e.body.active)), s(d, l(e.body.active))) : s(m, l(((_a = e.error) == null ? void 0 : _a.message) || "Error")), s(f, false);
  });
  async function q() {
    s(f, true);
    let e = { key_id: i(d) }, p = await ce("/auth/v1/encryption/migrate", e);
    p.error ? s(m, l(p.error.message)) : (s(x, true), setTimeout(() => {
      s(x, false);
    }, 4e3)), s(f, false);
  }
  var O = de(), h = te(O), z = r(h, true);
  t(h);
  var K = a(h, 2), D = r(K, true);
  t(K);
  var b = a(K, 2), F = r(b, true);
  t(b);
  var k = a(b, 2), $ = r(k), H = r($);
  t($);
  var P = a($, 2), J = r(P, true);
  t(P), t(k);
  var A = a(k, 2), N = r(A);
  t(A);
  var I = a(A, 2);
  se(I, 21, () => i(g), oe, (e, p) => {
    var c = me(), X = r(c, true);
    t(c), y(() => o(X, i(p))), u(e, c);
  }), t(I);
  var L = a(I, 2), Q = r(L, true);
  t(L);
  var T = a(L, 2), R = r(T);
  t(T);
  var j = a(T, 2);
  ve(j, { get ariaLabel() {
    return n.docs.encKeys.migrateToKey;
  }, get options() {
    return i(g);
  }, get value() {
    return i(d);
  }, set value(e) {
    s(d, l(e));
  } });
  var w = a(j, 2), C = r(w);
  ie(C, { onclick: q, get isLoading() {
    return i(f);
  }, children: (e, p) => {
    re();
    var c = Y();
    y(() => o(c, n.docs.encKeys.migrate)), u(e, c);
  }, $$slots: { default: true } });
  var S = a(C, 2);
  {
    var U = (e) => {
      ne(e, {});
    };
    G(S, (e) => {
      i(x) && e(U);
    });
  }
  t(w);
  var V = a(w, 2);
  {
    var W = (e) => {
      var p = fe(), c = r(p, true);
      t(p), y(() => o(c, i(m))), u(e, p);
    };
    G(V, (e) => {
      i(m) && e(W);
    });
  }
  y(() => {
    o(z, n.docs.encKeys.header), o(D, n.docs.encKeys.p1), o(F, n.docs.encKeys.p2), o(H, `${n.docs.encKeys.keyActive ?? ""}:`), o(J, i(E)), o(N, `${n.docs.encKeys.keysAvailable ?? ""}:`), o(Q, n.docs.encKeys.p3), o(R, `${n.docs.encKeys.migrateToKey ?? ""}:`);
  }), u(_, O), ae();
}
function Le(_) {
  ue(_, {});
}
export {
  Le as component
};
