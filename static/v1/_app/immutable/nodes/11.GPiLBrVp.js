import { t as H, a as k, e as Nt } from "../chunks/BxmJRzoY.js";
import { p as it, c as e, r as a, a3 as S, s as t, t as D, k as v, a as vt, j as B, a1 as nt, f as _t, a4 as pt, l as _, a0 as Vt } from "../chunks/w0HvPX0p.js";
import { s } from "../chunks/BzP2S3Z_.js";
import { i as st } from "../chunks/iO9_dPNE.js";
import { h as w } from "../chunks/C2ZdIFW_.js";
import { a as zt } from "../chunks/CJLp5kwW.js";
import { B as Lt } from "../chunks/C8YTstTD.js";
import { e as St, i as Dt } from "../chunks/S81raU5Y.js";
import { u as ot } from "../chunks/Bt_9UXew.js";
import { I as rt } from "../chunks/DVXwAhn3.js";
import { F as Ft } from "../chunks/BEbxeS8S.js";
import { b as Ut, f as jt } from "../chunks/UPFlzoow.js";
var qt = H('<div class="row svelte-1v63e3g"><div class="svelte-1v63e3g"> </div> <div class="svelte-1v63e3g"> </div> <div class="svelte-1v63e3g"> </div> <div class="svelte-1v63e3g"> </div></div>'), Et = H('<div><div class="row header font-label svelte-1v63e3g"><div class="svelte-1v63e3g"> </div> <div class="svelte-1v63e3g">t_cost</div> <div class="svelte-1v63e3g">m_cost</div> <div class="svelte-1v63e3g">p_cost</div></div> <!> <div style="height: 1.5rem"></div></div>');
function Gt(I, T) {
  it(T, true);
  let r = ot();
  var o = Et(), c = e(o), m = e(c), f = e(m);
  a(m), S(6), a(c);
  var b = t(c, 2);
  St(b, 17, () => T.res, Dt, (d, u) => {
    var p = qt(), x = e(p), V = e(x, true);
    a(x);
    var h = t(x, 2), z = e(h, true);
    a(h);
    var y = t(h, 2), A = e(y, true);
    a(y);
    var i = t(y, 2), l = e(i, true);
    a(i), a(p), D(() => {
      s(V, v(u).time_taken), s(z, v(u).t_cost), s(A, v(u).m_cost), s(l, v(u).p_cost);
    }), k(d, p);
  }), S(2), a(o), D(() => s(f, `${r.common.time ?? ""} (ms)`)), k(I, o), vt();
}
var Mt = H("<!> <!> <!> <!>", 1), Rt = H('<div class="result svelte-d61iiz"><!></div>'), Wt = H('<div class="err"> </div>'), Yt = H('<div class="wrapper svelte-d61iiz"><div class="container svelte-d61iiz"><!></div> <!> <!></div>');
function Jt(I, T) {
  it(T, true);
  const r = "10rem";
  let o = ot(), c = B(false), m = B(""), f = B(nt([])), b = B(void 0), d = nt({ targetTime: "1000", mCost: T.params.m_cost.toString() || "32768", pCost: T.params.p_cost.toString() || "2" });
  async function u(i, l) {
    var _a;
    _(c, true), _(f, [], true);
    const n = { target_time: Number.parseInt(d.targetTime), m_cost: Number.parseInt(d.mCost), p_cost: Number.parseInt(d.pCost) };
    let $ = await Ut(i.action, n);
    $.body ? (_(f, $.body.results, true), _(m, ""), setTimeout(() => {
      _(b, window.innerHeight, true);
    }, 50)) : _(m, ((_a = $.error) == null ? void 0 : _a.message) || "Error", true), _(c, false);
  }
  var p = Yt(), x = e(p), V = e(x);
  Ft(V, { action: "/auth/v1/password_hash_times", onSubmit: u, children: (i, l) => {
    var n = Mt(), $ = _t(n);
    const F = pt(() => `${o.docs.hashing.targetTime} (ms)`), U = pt(() => `${o.docs.hashing.targetTime} (ms)`);
    rt($, { typ: "number", autocomplete: "off", get label() {
      return v(F);
    }, get placeholder() {
      return v(U);
    }, min: "500", max: "99999", required: true, width: r, get value() {
      return d.targetTime;
    }, set value(g) {
      d.targetTime = g;
    } });
    var P = t($, 2);
    rt(P, { typ: "number", autocomplete: "off", label: "m_cost (kB)", placeholder: "m_cost (kB)", min: "32768", width: r, get value() {
      return d.mCost;
    }, set value(g) {
      d.mCost = g;
    } });
    var N = t(P, 2);
    rt(N, { typ: "number", autocomplete: "off", label: "p_cost", placeholder: "p_cost", min: "2", max: "256", width: r, get value() {
      return d.pCost;
    }, set value(g) {
      d.pCost = g;
    } });
    var j = t(N, 2);
    Lt(j, { type: "submit", get isLoading() {
      return v(c);
    }, children: (g, dt) => {
      S();
      var L = Nt();
      D(() => s(L, o.docs.hashing.calculate)), k(g, L);
    }, $$slots: { default: true } }), k(i, n);
  }, $$slots: { default: true } }), a(x);
  var h = t(x, 2);
  {
    var z = (i) => {
      var l = Rt(), n = e(l);
      Gt(n, { get res() {
        return v(f);
      } }), a(l), k(i, l);
    };
    st(h, (i) => {
      v(f).length > 0 && i(z);
    });
  }
  var y = t(h, 2);
  {
    var A = (i) => {
      var l = Wt(), n = e(l, true);
      a(l), D(() => s(n, v(m))), k(i, l);
    };
    st(y, (i) => {
      v(m) && i(A);
    });
  }
  a(p), zt("y", () => v(b), (i) => _(b, i, true)), k(I, p), vt();
}
var Kt = H('<h3>Argon2ID - Password Hashing</h3> <p> </p><p></p><p><b> </b><br> </p> <p> </p> <ul><li><b>m_cost</b> <p><!></p> <p><!></p> <p><!></p></li> <li><b>p_cost</b> <p><!></p> <p><!></p></li> <li><b>t_cost</b> <p><!></p> <p><!></p></li></ul> <br> <h3> </h3> <p> </p> <p> </p> <br> <h3> </h3> <p> </p> <div><div class="flex"><div class="label svelte-klhp01">m_cost:</div> </div> <div class="flex"><div class="label svelte-klhp01">p_cost:</div> </div> <div class="flex"><div class="label svelte-klhp01">t_cost:</div> </div> <br> <div class="flex"><div class="label svelte-klhp01">Login Time:</div> </div> <p> </p> <b> </b> <br></div> <br> <h3> </h3> <p> </p> <p><!></p> <!> <div style="height: 1.5rem"></div>', 1);
function Ot(I, T) {
  it(T, true);
  let r = ot(), o = B(void 0), c = B(void 0), m = B(void 0);
  Vt(async () => {
    let C = await jt("/auth/v1/login_time");
    C.body ? (_(o, C.body.argon2_params, true), _(c, C.body.login_time, true), _(m, C.body.num_cpus, true)) : console.error(C.error);
  });
  var f = Kt(), b = t(_t(f), 2), d = e(b, true);
  a(b);
  var u = t(b, 2), p = e(u), x = e(p, true);
  a(p);
  var V = t(p, 2);
  a(u);
  var h = t(u, 2), z = e(h, true);
  a(h);
  var y = t(h, 2), A = e(y), i = t(e(A), 2), l = e(i);
  w(l, () => r.docs.hashing.mCost1), a(i);
  var n = t(i, 2), $ = e(n);
  w($, () => r.docs.hashing.mCost2), a(n);
  var F = t(n, 2), U = e(F);
  w(U, () => r.docs.hashing.mCost3), a(F), a(A);
  var P = t(A, 2), N = t(e(P), 2), j = e(N);
  w(j, () => r.docs.hashing.pCost1), a(N);
  var g = t(N, 2), dt = e(g);
  w(dt, () => r.docs.hashing.pCost2), a(g), a(P);
  var L = t(P, 2), q = t(e(L), 2), ct = e(q);
  w(ct, () => r.docs.hashing.tCost1), a(q);
  var lt = t(q, 2), mt = e(lt);
  w(mt, () => r.docs.hashing.tCost2), a(lt), a(L), a(y);
  var E = t(y, 4), ut = e(E, true);
  a(E);
  var G = t(E, 2), ht = e(G, true);
  a(G);
  var M = t(G, 2), gt = e(M, true);
  a(M);
  var R = t(M, 4), ft = e(R, true);
  a(R);
  var W = t(R, 2), bt = e(W, true);
  a(W);
  var Y = t(W, 2), J = e(Y), xt = t(e(J));
  a(J);
  var K = t(J, 2), yt = t(e(K));
  a(K);
  var O = t(K, 2), Ct = t(e(O));
  a(O);
  var Q = t(O, 4), Tt = t(e(Q));
  a(Q);
  var X = t(Q, 2), $t = e(X, true);
  a(X);
  var Z = t(X, 2), wt = e(Z, true);
  a(Z);
  var kt = t(Z);
  S(), a(Y);
  var tt = t(Y, 4), It = e(tt, true);
  a(tt);
  var et = t(tt, 2), At = e(et, true);
  a(et);
  var at = t(et, 2), Bt = e(at);
  w(Bt, () => r.docs.hashing.utility2), a(at);
  var Ht = t(at, 2);
  {
    var Pt = (C) => {
      Jt(C, { get params() {
        return v(o);
      } });
    };
    st(Ht, (C) => {
      v(o) && C(Pt);
    });
  }
  S(2), D(() => {
    var _a, _b, _c;
    s(d, r.docs.hashing.pUtility), s(x, r.docs.hashing.tune), s(V, ` ${r.docs.hashing.pTune ?? ""}`), s(z, r.docs.hashing.pDetials), s(ut, r.docs.hashing.loginTimeHead), s(ht, r.docs.hashing.loginTime1), s(gt, r.docs.hashing.loginTime2), s(ft, r.docs.hashing.currValuesHead), s(bt, r.docs.hashing.currValues1), s(xt, ` ${((_a = v(o)) == null ? void 0 : _a.m_cost) ?? ""} kB`), s(yt, ` ${((_b = v(o)) == null ? void 0 : _b.p_cost) ?? ""} threads`), s(Ct, ` ${((_c = v(o)) == null ? void 0 : _c.t_cost) ?? ""}`), s(Tt, ` ${v(c) ?? ""} ms`), s($t, r.docs.hashing.currValuesNote), s(wt, r.docs.hashing.currValuesThreadsAccess), s(kt, `: ${v(m) ?? ""}`), s(It, r.docs.hashing.utilityHead), s(At, r.docs.hashing.utility1);
  }), k(I, f), vt();
}
function le(I) {
  Ot(I, {});
}
export {
  le as component
};
