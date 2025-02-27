var kn = Array.isArray, Ut = Array.prototype.indexOf, Dn = Array.from, Sn = Object.defineProperty, st = Object.getOwnPropertyDescriptor, Cn = Object.getOwnPropertyDescriptors, bn = Object.prototype, Fn = Array.prototype, Pn = Object.getPrototypeOf;
function qn(t) {
  return typeof t == "function";
}
const Ln = () => {
};
function _t(t) {
  for (var n = 0; n < t.length; n++) t[n]();
}
function Mn() {
  var t, n, e = new Promise((r, l) => {
    t = r, n = l;
  });
  return { promise: e, resolve: t, reject: n };
}
const T = 2, ct = 4, X = 8, rt = 16, x = 32, M = 64, U = 128, w = 256, V = 512, _ = 1024, R = 2048, b = 4096, S = 8192, Q = 16384, Vt = 32768, vt = 65536, Gt = 1 << 19, pt = 1 << 20, Yn = Symbol("$state"), Hn = Symbol("legacy props"), Bn = Symbol("");
function ht(t) {
  return t === this.v;
}
function $t(t, n) {
  return t != t ? n == n : t !== n || t !== null && typeof t == "object" || typeof t == "function";
}
function jn(t, n) {
  return t !== n;
}
function dt(t) {
  return !$t(t, this.v);
}
function Kt(t) {
  throw new Error("https://svelte.dev/e/effect_in_teardown");
}
function Zt() {
  throw new Error("https://svelte.dev/e/effect_in_unowned_derived");
}
function zt(t) {
  throw new Error("https://svelte.dev/e/effect_orphan");
}
function Jt() {
  throw new Error("https://svelte.dev/e/effect_update_depth_exceeded");
}
function Un() {
  throw new Error("https://svelte.dev/e/hydration_failed");
}
function Vn(t) {
  throw new Error("https://svelte.dev/e/props_invalid_value");
}
function Gn() {
  throw new Error("https://svelte.dev/e/state_descriptors_fixed");
}
function $n() {
  throw new Error("https://svelte.dev/e/state_prototype_fixed");
}
function Wt() {
  throw new Error("https://svelte.dev/e/state_unsafe_local_read");
}
function Xt() {
  throw new Error("https://svelte.dev/e/state_unsafe_mutation");
}
const Kn = 1, Zn = 2, zn = 4, Jn = 8, Wn = 16, Xn = 1, Qn = 4, te = 8, ne = 16, ee = 1, re = 2, le = 4, ae = 1, se = 2, Qt = "[", tn = "[!", nn = "]", Et = {}, ue = Symbol();
function wt(t) {
  console.warn("https://svelte.dev/e/hydration_mismatch");
}
function yt(t) {
  throw new Error("https://svelte.dev/e/lifecycle_outside_component");
}
let c = null;
function ut(t) {
  c = t;
}
function oe(t, n = false, e) {
  c = { p: c, c: null, e: null, m: false, s: t, x: null, l: null };
}
function ie(t) {
  const n = c;
  if (n !== null) {
    t !== void 0 && (n.x = t);
    const s = n.e;
    if (s !== null) {
      var e = o, r = u;
      n.e = null;
      try {
        for (var l = 0; l < s.length; l++) {
          var a = s[l];
          z(a.effect), Z(a.reaction), kt(a.fn);
        }
      } finally {
        z(e), Z(r);
      }
    }
    c = n.p, n.m = true;
  }
  return t || {};
}
function Tt() {
  return true;
}
function gt(t, n) {
  var e = { f: 0, v: t, reactions: null, equals: ht, rv: 0, wv: 0 };
  return e;
}
function fe(t) {
  return en(gt(t));
}
function _e(t, n = false) {
  const e = gt(t);
  return n || (e.equals = dt), e;
}
function en(t) {
  return u !== null && !y && u.f & T && (m === null ? En([t]) : m.push(t)), t;
}
function ce(t, n) {
  return u !== null && !y && Tt() && u.f & (T | rt) && (m === null || !m.includes(t)) && Xt(), rn(t, n);
}
function rn(t, n) {
  return t.equals(n) || (t.v, t.v = n, t.wv = Mt(), mt(t, R), o !== null && o.f & _ && !(o.f & (x | M)) && (I === null ? wn([t]) : I.push(t))), n;
}
function mt(t, n) {
  var e = t.reactions;
  if (e !== null) for (var r = e.length, l = 0; l < r; l++) {
    var a = e[l], s = a.f;
    s & R || (A(a, n), s & (_ | w) && (s & T ? mt(a, b) : nt(a)));
  }
}
let N = false;
function ve(t) {
  N = t;
}
let d;
function C(t) {
  if (t === null) throw wt(), Et;
  return d = t;
}
function pe() {
  return C(D(d));
}
function he(t) {
  if (N) {
    if (D(d) !== null) throw wt(), Et;
    d = t;
  }
}
function de(t = 1) {
  if (N) {
    for (var n = t, e = d; n--; ) e = D(e);
    d = e;
  }
}
function Ee() {
  for (var t = 0, n = d; ; ) {
    if (n.nodeType === 8) {
      var e = n.data;
      if (e === nn) {
        if (t === 0) return n;
        t -= 1;
      } else (e === Qt || e === tn) && (t += 1);
    }
    var r = D(n);
    n.remove(), n = r;
  }
}
var ot, ln, an, At, It;
function we() {
  if (ot === void 0) {
    ot = window, ln = document, an = /Firefox/.test(navigator.userAgent);
    var t = Element.prototype, n = Node.prototype;
    At = st(n, "firstChild").get, It = st(n, "nextSibling").get, t.__click = void 0, t.__className = void 0, t.__attributes = null, t.__styles = null, t.__e = void 0, Text.prototype.__t = void 0;
  }
}
function G(t = "") {
  return document.createTextNode(t);
}
function et(t) {
  return At.call(t);
}
function D(t) {
  return It.call(t);
}
function ye(t, n) {
  if (!N) return et(t);
  var e = et(d);
  if (e === null) e = d.appendChild(G());
  else if (n && e.nodeType !== 3) {
    var r = G();
    return e == null ? void 0 : e.before(r), C(r), r;
  }
  return C(e), e;
}
function Te(t, n) {
  if (!N) {
    var e = et(t);
    return e instanceof Comment && e.data === "" ? D(e) : e;
  }
  if (n && (d == null ? void 0 : d.nodeType) !== 3) {
    var r = G();
    return d == null ? void 0 : d.before(r), C(r), r;
  }
  return d;
}
function ge(t, n = 1, e = false) {
  let r = N ? d : t;
  for (var l; n--; ) l = r, r = D(r);
  if (!N) return r;
  var a = r == null ? void 0 : r.nodeType;
  if (e && a !== 3) {
    var s = G();
    return r === null ? l == null ? void 0 : l.after(s) : r.before(s), C(s), s;
  }
  return C(r), r;
}
function me(t) {
  t.textContent = "";
}
function xt(t) {
  var n = T | R, e = u !== null && u.f & T ? u : null;
  return o === null || e !== null && e.f & w ? n |= w : o.f |= pt, { ctx: c, deps: null, effects: null, equals: ht, f: n, fn: t, reactions: null, rv: 0, v: null, wv: 0, parent: e ?? o };
}
function Ae(t) {
  const n = xt(t);
  return n.equals = dt, n;
}
function Ot(t) {
  var n = t.effects;
  if (n !== null) {
    t.effects = null;
    for (var e = 0; e < n.length; e += 1) k(n[e]);
  }
}
function sn(t) {
  for (var n = t.parent; n !== null; ) {
    if (!(n.f & T)) return n;
    n = n.parent;
  }
  return null;
}
function un(t) {
  var n, e = o;
  z(sn(t));
  try {
    Ot(t), n = Ht(t);
  } finally {
    z(e);
  }
  return n;
}
function Rt(t) {
  var n = un(t), e = (O || t.f & w) && t.deps !== null ? b : _;
  A(t, e), t.equals(n) || (t.v = n, t.wv = Mt());
}
function Nt(t) {
  o === null && u === null && zt(), u !== null && u.f & w && o === null && Zt(), lt && Kt();
}
function on(t, n) {
  var e = n.last;
  e === null ? n.last = n.first = t : (e.next = t, t.prev = e, n.last = t);
}
function F(t, n, e, r = true) {
  var l = (t & M) !== 0, a = o, s = { ctx: c, deps: null, nodes_start: null, nodes_end: null, f: t | R, first: null, fn: n, last: null, next: null, parent: l ? null : a, prev: null, teardown: null, transitions: null, wv: 0 };
  if (e) try {
    at(s), s.f |= Vt;
  } catch (E) {
    throw k(s), E;
  }
  else n !== null && nt(s);
  var v = e && s.deps === null && s.first === null && s.nodes_start === null && s.teardown === null && (s.f & (pt | U)) === 0;
  if (!v && !l && r && (a !== null && on(s, a), u !== null && u.f & T)) {
    var i = u;
    (i.effects ?? (i.effects = [])).push(s);
  }
  return s;
}
function Ie(t) {
  const n = F(X, null, false);
  return A(n, _), n.teardown = t, n;
}
function fn(t) {
  Nt();
  var n = o !== null && (o.f & x) !== 0 && c !== null && !c.m;
  if (n) {
    var e = c;
    (e.e ?? (e.e = [])).push({ fn: t, effect: o, reaction: u });
  } else {
    var r = kt(t);
    return r;
  }
}
function xe(t) {
  return Nt(), _n(t);
}
function Oe(t) {
  const n = F(M, t, true);
  return (e = {}) => new Promise((r) => {
    e.outro ? pn(n, () => {
      k(n), r(void 0);
    }) : (k(n), r(void 0));
  });
}
function kt(t) {
  return F(ct, t, false);
}
function _n(t) {
  return F(X, t, true);
}
function Re(t, n = [], e = xt) {
  const r = n.map(e);
  return cn(() => t(...r.map(On)));
}
function cn(t, n = 0) {
  return F(X | rt | n, t, true);
}
function Ne(t, n = true) {
  return F(X | x, t, true, n);
}
function Dt(t) {
  var n = t.teardown;
  if (n !== null) {
    const e = lt, r = u;
    ft(true), Z(null);
    try {
      n.call(null);
    } finally {
      ft(e), Z(r);
    }
  }
}
function St(t, n = false) {
  var e = t.first;
  for (t.first = t.last = null; e !== null; ) {
    var r = e.next;
    k(e, n), e = r;
  }
}
function vn(t) {
  for (var n = t.first; n !== null; ) {
    var e = n.next;
    n.f & x || k(n), n = e;
  }
}
function k(t, n = true) {
  var e = false;
  if ((n || t.f & Gt) && t.nodes_start !== null) {
    for (var r = t.nodes_start, l = t.nodes_end; r !== null; ) {
      var a = r === l ? null : D(r);
      r.remove(), r = a;
    }
    e = true;
  }
  St(t, n && !e), W(t, 0), A(t, Q);
  var s = t.transitions;
  if (s !== null) for (const i of s) i.stop();
  Dt(t);
  var v = t.parent;
  v !== null && v.first !== null && Ct(t), t.next = t.prev = t.teardown = t.ctx = t.deps = t.fn = t.nodes_start = t.nodes_end = null;
}
function Ct(t) {
  var n = t.parent, e = t.prev, r = t.next;
  e !== null && (e.next = r), r !== null && (r.prev = e), n !== null && (n.first === t && (n.first = r), n.last === t && (n.last = e));
}
function pn(t, n) {
  var e = [];
  bt(t, e, true), hn(e, () => {
    k(t), n && n();
  });
}
function hn(t, n) {
  var e = t.length;
  if (e > 0) {
    var r = () => --e || n();
    for (var l of t) l.out(r);
  } else n();
}
function bt(t, n, e) {
  if (!(t.f & S)) {
    if (t.f ^= S, t.transitions !== null) for (const s of t.transitions) (s.is_global || e) && n.push(s);
    for (var r = t.first; r !== null; ) {
      var l = r.next, a = (r.f & vt) !== 0 || (r.f & x) !== 0;
      bt(r, n, a ? e : false), r = l;
    }
  }
}
function ke(t) {
  Ft(t, true);
}
function Ft(t, n) {
  if (t.f & S) {
    t.f ^= S, t.f & _ || (t.f ^= _), Y(t) && (A(t, R), nt(t));
    for (var e = t.first; e !== null; ) {
      var r = e.next, l = (e.f & vt) !== 0 || (e.f & x) !== 0;
      Ft(e, l ? n : false), e = r;
    }
    if (t.transitions !== null) for (const a of t.transitions) (a.is_global || n) && a.in();
  }
}
const dn = typeof requestIdleCallback > "u" ? (t) => setTimeout(t, 1) : requestIdleCallback;
let q = [], L = [];
function Pt() {
  var t = q;
  q = [], _t(t);
}
function qt() {
  var t = L;
  L = [], _t(t);
}
function De(t) {
  q.length === 0 && queueMicrotask(Pt), q.push(t);
}
function Se(t) {
  L.length === 0 && dn(qt), L.push(t);
}
function it() {
  q.length > 0 && Pt(), L.length > 0 && qt();
}
let B = false, $ = false, K = null, j = false, lt = false;
function ft(t) {
  lt = t;
}
let P = [];
let u = null, y = false;
function Z(t) {
  u = t;
}
let o = null;
function z(t) {
  o = t;
}
let m = null;
function En(t) {
  m = t;
}
let f = null, h = 0, I = null;
function wn(t) {
  I = t;
}
let Lt = 1, J = 0, O = false;
function Mt() {
  return ++Lt;
}
function Y(t) {
  var _a;
  var n = t.f;
  if (n & R) return true;
  if (n & b) {
    var e = t.deps, r = (n & w) !== 0;
    if (e !== null) {
      var l, a, s = (n & V) !== 0, v = r && o !== null && !O, i = e.length;
      if (s || v) {
        var E = t, H = E.parent;
        for (l = 0; l < i; l++) a = e[l], (s || !((_a = a == null ? void 0 : a.reactions) == null ? void 0 : _a.includes(E))) && (a.reactions ?? (a.reactions = [])).push(E);
        s && (E.f ^= V), v && H !== null && !(H.f & w) && (E.f ^= w);
      }
      for (l = 0; l < i; l++) if (a = e[l], Y(a) && Rt(a), a.wv > t.wv) return true;
    }
    (!r || o !== null && !O) && A(t, _);
  }
  return false;
}
function yn(t, n) {
  for (var e = n; e !== null; ) {
    if (e.f & U) try {
      e.fn(t);
      return;
    } catch {
      e.f ^= U;
    }
    e = e.parent;
  }
  throw B = false, t;
}
function Tn(t) {
  return (t.f & Q) === 0 && (t.parent === null || (t.parent.f & U) === 0);
}
function tt(t, n, e, r) {
  if (B) {
    if (e === null && (B = false), Tn(n)) throw t;
    return;
  }
  e !== null && (B = true);
  {
    yn(t, n);
    return;
  }
}
function Yt(t, n, e = true) {
  var r = t.reactions;
  if (r !== null) for (var l = 0; l < r.length; l++) {
    var a = r[l];
    a.f & T ? Yt(a, n, false) : n === a && (e ? A(a, R) : a.f & _ && A(a, b), nt(a));
  }
}
function Ht(t) {
  var _a;
  var n = f, e = h, r = I, l = u, a = O, s = m, v = c, i = y, E = t.f;
  f = null, h = 0, I = null, O = (E & w) !== 0 && (y || !j || u === null), u = E & (x | M) ? null : t, m = null, ut(t.ctx), y = false, J++;
  try {
    var H = (0, t.fn)(), g = t.deps;
    if (f !== null) {
      var p;
      if (W(t, h), g !== null && h > 0) for (g.length = h + f.length, p = 0; p < f.length; p++) g[h + p] = f[p];
      else t.deps = g = f;
      if (!O) for (p = h; p < g.length; p++) ((_a = g[p]).reactions ?? (_a.reactions = [])).push(t);
    } else g !== null && h < g.length && (W(t, h), g.length = h);
    if (Tt() && I !== null && !y && g !== null && !(t.f & (T | b | R))) for (p = 0; p < I.length; p++) Yt(I[p], t);
    return l !== null && J++, H;
  } finally {
    f = n, h = e, I = r, u = l, O = a, m = s, ut(v), y = i;
  }
}
function gn(t, n) {
  let e = n.reactions;
  if (e !== null) {
    var r = Ut.call(e, t);
    if (r !== -1) {
      var l = e.length - 1;
      l === 0 ? e = n.reactions = null : (e[r] = e[l], e.pop());
    }
  }
  e === null && n.f & T && (f === null || !f.includes(n)) && (A(n, b), n.f & (w | V) || (n.f ^= V), Ot(n), W(n, 0));
}
function W(t, n) {
  var e = t.deps;
  if (e !== null) for (var r = n; r < e.length; r++) gn(t, e[r]);
}
function at(t) {
  var n = t.f;
  if (!(n & Q)) {
    A(t, _);
    var e = o, r = c, l = j;
    o = t, j = true;
    try {
      n & rt ? vn(t) : St(t), Dt(t);
      var a = Ht(t);
      t.teardown = typeof a == "function" ? a : null, t.wv = Lt;
      var s = t.deps, v;
    } catch (i) {
      tt(i, t, e, r || t.ctx);
    } finally {
      j = l, o = e;
    }
  }
}
function mn() {
  try {
    Jt();
  } catch (t) {
    if (K !== null) tt(t, K, null);
    else throw t;
  }
}
function Bt() {
  try {
    for (var t = 0; P.length > 0; ) {
      t++ > 1e3 && mn();
      var n = P, e = n.length;
      P = [];
      for (var r = 0; r < e; r++) {
        var l = n[r];
        l.f & _ || (l.f ^= _);
        var a = In(l);
        An(a);
      }
    }
  } finally {
    $ = false, K = null;
  }
}
function An(t) {
  var n = t.length;
  if (n !== 0) for (var e = 0; e < n; e++) {
    var r = t[e];
    if (!(r.f & (Q | S))) try {
      Y(r) && (at(r), r.deps === null && r.first === null && r.nodes_start === null && (r.teardown === null ? Ct(r) : r.fn = null));
    } catch (l) {
      tt(l, r, null, r.ctx);
    }
  }
}
function nt(t) {
  $ || ($ = true, queueMicrotask(Bt));
  for (var n = K = t; n.parent !== null; ) {
    n = n.parent;
    var e = n.f;
    if (e & (M | x)) {
      if (!(e & _)) return;
      n.f ^= _;
    }
  }
  P.push(n);
}
function In(t) {
  for (var n = [], e = t.first; e !== null; ) {
    var r = e.f, l = (r & x) !== 0, a = l && (r & _) !== 0;
    if (!a && !(r & S)) {
      if (r & ct) n.push(e);
      else if (l) e.f ^= _;
      else {
        var s = u;
        try {
          u = e, Y(e) && at(e);
        } catch (E) {
          tt(E, e, null, e.ctx);
        } finally {
          u = s;
        }
      }
      var v = e.first;
      if (v !== null) {
        e = v;
        continue;
      }
    }
    var i = e.parent;
    for (e = e.next; e === null && i !== null; ) e = i.next, i = i.parent;
  }
  return n;
}
function xn(t) {
  var n;
  for (it(); P.length > 0; ) $ = true, Bt(), it();
  return n;
}
async function Ce() {
  await Promise.resolve(), xn();
}
function On(t) {
  var n = t.f, e = (n & T) !== 0;
  if (u !== null && !y) {
    m !== null && m.includes(t) && Wt();
    var r = u.deps;
    t.rv < J && (t.rv = J, f === null && r !== null && r[h] === t ? h++ : f === null ? f = [t] : (!O || !f.includes(t)) && f.push(t));
  } else if (e && t.deps === null && t.effects === null) {
    var l = t, a = l.parent;
    a !== null && !(a.f & w) && (l.f ^= w);
  }
  return e && (l = t, Y(l) && Rt(l)), t.v;
}
function jt(t) {
  var n = y;
  try {
    return y = true, t();
  } finally {
    y = n;
  }
}
const Rn = -7169;
function A(t, n) {
  t.f = t.f & Rn | n;
}
function Nn(t) {
  c === null && yt(), fn(() => {
    const n = jt(t);
    if (typeof n == "function") return n;
  });
}
function be(t) {
  c === null && yt(), Nn(() => () => jt(t));
}
export {
  _e as $,
  Ee as A,
  ve as B,
  ke as C,
  bn as D,
  vt as E,
  Fn as F,
  gt as G,
  Et as H,
  $n as I,
  st as J,
  o as K,
  Gn as L,
  Pn as M,
  kn as N,
  jt as O,
  De as P,
  G as Q,
  zn as R,
  Yn as S,
  Ae as T,
  ue as U,
  nn as V,
  S as W,
  Dn as X,
  Kn as Y,
  rn as Z,
  Zn as _,
  ie as a,
  Jn as a0,
  bt as a1,
  me as a2,
  hn as a3,
  Wn as a4,
  Nn as a5,
  ln as a6,
  de as a7,
  Ln as a8,
  xt as a9,
  Un as aA,
  Oe as aB,
  c as aC,
  rt as aD,
  Vt as aE,
  le as aF,
  qn as aG,
  ee as aH,
  re as aI,
  $t as aJ,
  fn as aa,
  Vn as ab,
  Qn as ac,
  dt as ad,
  te as ae,
  Hn as af,
  Xn as ag,
  ne as ah,
  Mn as ai,
  be as aj,
  an as ak,
  ae as al,
  se as am,
  Bn as an,
  Se as ao,
  Cn as ap,
  xn as aq,
  Sn as ar,
  xe as as,
  Ce as at,
  Z as au,
  z as av,
  u as aw,
  Gt as ax,
  Qt as ay,
  we as az,
  pe as b,
  ye as c,
  cn as d,
  pn as e,
  Te as f,
  Ne as g,
  N as h,
  d as i,
  On as j,
  fe as k,
  ce as l,
  D as m,
  jn as n,
  wt as o,
  oe as p,
  C as q,
  he as r,
  ge as s,
  Re as t,
  et as u,
  k as v,
  _n as w,
  kt as x,
  Ie as y,
  tn as z
};
