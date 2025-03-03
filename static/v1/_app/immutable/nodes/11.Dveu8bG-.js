import { t as P, a as I, e as Nt } from "../chunks/BH6NCLk-.js";
import { p as ot, c as e, r as a, a7 as D, s as t, t as F, j as o, a as vt, k as H, f as _t, a9 as pt, l as _, a5 as Vt } from "../chunks/CvlvO1XB.js";
import { s } from "../chunks/CTI4QPiR.js";
import { i as it } from "../chunks/BUO_AUgz.js";
import { h as k } from "../chunks/i8Xqpu09.js";
import { p as f } from "../chunks/Wh68IIk2.js";
import { a as zt } from "../chunks/7lxiEjMQ.js";
import { B as Lt } from "../chunks/DMkkW5Nn.js";
import { e as St, i as Dt } from "../chunks/BpWRzPRQ.js";
import { u as dt } from "../chunks/D8mHI_K9.js";
import { I as st } from "../chunks/CeeD0cV_.js";
import { F as Ft } from "../chunks/Gcs0S4Fz.js";
import { b as Ut, f as jt } from "../chunks/DYtiVhoA.js";
var qt = P('<div class="row svelte-1v63e3g"><div class="svelte-1v63e3g"> </div> <div class="svelte-1v63e3g"> </div> <div class="svelte-1v63e3g"> </div> <div class="svelte-1v63e3g"> </div></div>'), Et = P('<div><div class="row header font-label svelte-1v63e3g"><div class="svelte-1v63e3g"> </div> <div class="svelte-1v63e3g">t_cost</div> <div class="svelte-1v63e3g">m_cost</div> <div class="svelte-1v63e3g">p_cost</div></div> <!> <div style="height: 1.5rem"></div></div>');
function Gt(A, $) {
  ot($, true);
  let r = dt();
  var v = Et(), c = e(v), m = e(c), b = e(m);
  a(m), D(6), a(c);
  var x = t(c, 2);
  St(x, 17, () => $.res, Dt, (d, h) => {
    var p = qt(), y = e(p), z = e(y, true);
    a(y);
    var u = t(y, 2), L = e(u, true);
    a(u);
    var C = t(u, 2), B = e(C, true);
    a(C);
    var i = t(C, 2), l = e(i, true);
    a(i), a(p), F(() => {
      s(z, o(h).time_taken), s(L, o(h).t_cost), s(B, o(h).m_cost), s(l, o(h).p_cost);
    }), I(d, p);
  }), D(2), a(v), F(() => s(b, `${r.common.time ?? ""} (ms)`)), I(A, v), vt();
}
var Mt = P("<!> <!> <!> <!>", 1), Rt = P('<div class="result svelte-d61iiz"><!></div>'), Wt = P('<div class="err"> </div>'), Yt = P('<div class="wrapper svelte-d61iiz"><div class="container svelte-d61iiz"><!></div> <!> <!></div>');
function Jt(A, $) {
  ot($, true);
  const r = "10rem";
  let v = dt(), c = H(false), m = H(""), b = H(f([])), x = H(void 0), d = f({ targetTime: "1000", mCost: $.params.m_cost.toString() || "32768", pCost: $.params.p_cost.toString() || "2" });
  async function h(i, l) {
    var _a;
    _(c, true), _(b, f([]));
    const n = { target_time: Number.parseInt(d.targetTime), m_cost: Number.parseInt(d.mCost), p_cost: Number.parseInt(d.pCost) };
    let w = await Ut(i.action, n);
    w.body ? (_(b, f(w.body.results)), _(m, ""), setTimeout(() => {
      _(x, f(window.innerHeight));
    }, 50)) : _(m, f(((_a = w.error) == null ? void 0 : _a.message) || "Error")), _(c, false);
  }
  var p = Yt(), y = e(p), z = e(y);
  Ft(z, { action: "/auth/v1/password_hash_times", onSubmit: h, children: (i, l) => {
    var n = Mt(), w = _t(n);
    const U = pt(() => `${v.docs.hashing.targetTime} (ms)`), j = pt(() => `${v.docs.hashing.targetTime} (ms)`);
    st(w, { typ: "number", autocomplete: "off", get label() {
      return o(U);
    }, get placeholder() {
      return o(j);
    }, min: "500", max: "99999", required: true, width: r, get value() {
      return d.targetTime;
    }, set value(g) {
      d.targetTime = g;
    } });
    var N = t(w, 2);
    st(N, { typ: "number", autocomplete: "off", label: "m_cost (kB)", placeholder: "m_cost (kB)", min: "32768", width: r, get value() {
      return d.mCost;
    }, set value(g) {
      d.mCost = g;
    } });
    var V = t(N, 2);
    st(V, { typ: "number", autocomplete: "off", label: "p_cost", placeholder: "p_cost", min: "2", max: "256", width: r, get value() {
      return d.pCost;
    }, set value(g) {
      d.pCost = g;
    } });
    var q = t(V, 2);
    Lt(q, { type: "submit", get isLoading() {
      return o(c);
    }, children: (g, lt) => {
      D();
      var S = Nt();
      F(() => s(S, v.docs.hashing.calculate)), I(g, S);
    }, $$slots: { default: true } }), I(i, n);
  }, $$slots: { default: true } }), a(y);
  var u = t(y, 2);
  {
    var L = (i) => {
      var l = Rt(), n = e(l);
      Gt(n, { get res() {
        return o(b);
      } }), a(l), I(i, l);
    };
    it(u, (i) => {
      o(b).length > 0 && i(L);
    });
  }
  var C = t(u, 2);
  {
    var B = (i) => {
      var l = Wt(), n = e(l, true);
      a(l), F(() => s(n, o(m))), I(i, l);
    };
    it(C, (i) => {
      o(m) && i(B);
    });
  }
  a(p), zt("y", () => o(x), (i) => _(x, f(i))), I(A, p), vt();
}
var Kt = P('<h3>Argon2ID - Password Hashing</h3> <p> </p><p></p><p><b> </b><br> </p> <p> </p> <ul><li><b>m_cost</b> <p><!></p> <p><!></p> <p><!></p></li> <li><b>p_cost</b> <p><!></p> <p><!></p></li> <li><b>t_cost</b> <p><!></p> <p><!></p></li></ul> <br> <h3> </h3> <p> </p> <p> </p> <br> <h3> </h3> <p> </p> <div><div class="flex"><div class="label svelte-klhp01">m_cost:</div> </div> <div class="flex"><div class="label svelte-klhp01">p_cost:</div> </div> <div class="flex"><div class="label svelte-klhp01">t_cost:</div> </div> <br> <div class="flex"><div class="label svelte-klhp01">Login Time:</div> </div> <p> </p> <b> </b> <br></div> <br> <h3> </h3> <p> </p> <p><!></p> <!> <div style="height: 1.5rem"></div>', 1);
function Ot(A, $) {
  ot($, true);
  let r = dt(), v = H(void 0), c = H(void 0), m = H(void 0);
  Vt(async () => {
    let T = await jt("/auth/v1/login_time");
    T.body ? (_(v, f(T.body.argon2_params)), _(c, f(T.body.login_time)), _(m, f(T.body.num_cpus))) : console.error(T.error);
  });
  var b = Kt(), x = t(_t(b), 2), d = e(x, true);
  a(x);
  var h = t(x, 2), p = e(h), y = e(p, true);
  a(p);
  var z = t(p, 2);
  a(h);
  var u = t(h, 2), L = e(u, true);
  a(u);
  var C = t(u, 2), B = e(C), i = t(e(B), 2), l = e(i);
  k(l, () => r.docs.hashing.mCost1), a(i);
  var n = t(i, 2), w = e(n);
  k(w, () => r.docs.hashing.mCost2), a(n);
  var U = t(n, 2), j = e(U);
  k(j, () => r.docs.hashing.mCost3), a(U), a(B);
  var N = t(B, 2), V = t(e(N), 2), q = e(V);
  k(q, () => r.docs.hashing.pCost1), a(V);
  var g = t(V, 2), lt = e(g);
  k(lt, () => r.docs.hashing.pCost2), a(g), a(N);
  var S = t(N, 2), E = t(e(S), 2), ct = e(E);
  k(ct, () => r.docs.hashing.tCost1), a(E);
  var nt = t(E, 2), mt = e(nt);
  k(mt, () => r.docs.hashing.tCost2), a(nt), a(S), a(C);
  var G = t(C, 4), ht = e(G, true);
  a(G);
  var M = t(G, 2), ut = e(M, true);
  a(M);
  var R = t(M, 2), gt = e(R, true);
  a(R);
  var W = t(R, 4), ft = e(W, true);
  a(W);
  var Y = t(W, 2), bt = e(Y, true);
  a(Y);
  var J = t(Y, 2), K = e(J), xt = t(e(K));
  a(K);
  var O = t(K, 2), yt = t(e(O));
  a(O);
  var Q = t(O, 2), Ct = t(e(Q));
  a(Q);
  var X = t(Q, 4), Tt = t(e(X));
  a(X);
  var Z = t(X, 2), $t = e(Z, true);
  a(Z);
  var tt = t(Z, 2), wt = e(tt, true);
  a(tt);
  var kt = t(tt);
  D(), a(J);
  var et = t(J, 4), It = e(et, true);
  a(et);
  var at = t(et, 2), At = e(at, true);
  a(at);
  var rt = t(at, 2), Bt = e(rt);
  k(Bt, () => r.docs.hashing.utility2), a(rt);
  var Ht = t(rt, 2);
  {
    var Pt = (T) => {
      Jt(T, { get params() {
        return o(v);
      } });
    };
    it(Ht, (T) => {
      o(v) && T(Pt);
    });
  }
  D(2), F(() => {
    var _a, _b, _c;
    s(d, r.docs.hashing.pUtility), s(y, r.docs.hashing.tune), s(z, ` ${r.docs.hashing.pTune ?? ""}`), s(L, r.docs.hashing.pDetials), s(ht, r.docs.hashing.loginTimeHead), s(ut, r.docs.hashing.loginTime1), s(gt, r.docs.hashing.loginTime2), s(ft, r.docs.hashing.currValuesHead), s(bt, r.docs.hashing.currValues1), s(xt, ` ${((_a = o(v)) == null ? void 0 : _a.m_cost) ?? ""} kB`), s(yt, ` ${((_b = o(v)) == null ? void 0 : _b.p_cost) ?? ""} threads`), s(Ct, ` ${((_c = o(v)) == null ? void 0 : _c.t_cost) ?? ""}`), s(Tt, ` ${o(c) ?? ""} ms`), s($t, r.docs.hashing.currValuesNote), s(wt, r.docs.hashing.currValuesThreadsAccess), s(kt, `: ${o(m) ?? ""}`), s(It, r.docs.hashing.utilityHead), s(At, r.docs.hashing.utility1);
  }), I(A, b), vt();
}
function ne(A) {
  Ot(A, {});
}
export {
  ne as component
};
