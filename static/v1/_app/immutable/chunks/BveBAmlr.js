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
const y = 2, ct = 4, X = 8, rt = 16, I = 32, F = 64, U = 128, w = 256, V = 512, d = 1024, O = 2048, P = 4096, C = 8192, Q = 16384, Vt = 32768, vt = 65536, Gt = 1 << 19, pt = 1 << 20, Hn = Symbol("$state"), Yn = Symbol("legacy props"), Bn = Symbol("");
function ht(t) {
  return t === this.v;
}
function Kt(t, n) {
  return t != t ? n == n : t !== n || t !== null && typeof t == "object" || typeof t == "function";
}
function jn(t, n) {
  return t !== n;
}
function dt(t) {
  return !Kt(t, this.v);
}
function $t(t) {
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
function Kn() {
  throw new Error("https://svelte.dev/e/state_prototype_fixed");
}
function Wt() {
  throw new Error("https://svelte.dev/e/state_unsafe_local_read");
}
function Xt() {
  throw new Error("https://svelte.dev/e/state_unsafe_mutation");
}
const $n = 1, Zn = 2, zn = 4, Jn = 8, Wn = 16, Xn = 1, Qn = 4, te = 8, ne = 16, ee = 1, re = 2, le = 4, ae = 1, se = 2, Qt = "[", tn = "[!", nn = "]", wt = {}, ue = Symbol(), oe = "http://www.w3.org/1999/xhtml";
function Et(t) {
  console.warn("https://svelte.dev/e/hydration_mismatch");
}
function yt(t) {
  throw new Error("https://svelte.dev/e/lifecycle_outside_component");
}
let _ = null;
function ut(t) {
  _ = t;
}
function ie(t, n = false, e) {
  _ = { p: _, c: null, e: null, m: false, s: t, x: null, l: null };
}
function fe(t) {
  const n = _;
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
    _ = n.p, n.m = true;
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
function _e(t) {
  return en(gt(t));
}
function ce(t, n = false) {
  const e = gt(t);
  return n || (e.equals = dt), e;
}
function en(t) {
  return u !== null && !E && (u.f & y) !== 0 && (m === null ? wn([t]) : m.push(t)), t;
}
function ve(t, n) {
  return u !== null && !E && Tt() && (u.f & (y | rt)) !== 0 && (m === null || !m.includes(t)) && Xt(), rn(t, n);
}
function rn(t, n) {
  return t.equals(n) || (t.v, t.v = n, t.wv = Mt(), mt(t, O), o !== null && (o.f & d) !== 0 && (o.f & (I | F)) === 0 && (x === null ? En([t]) : x.push(t))), n;
}
function mt(t, n) {
  var e = t.reactions;
  if (e !== null) for (var r = e.length, l = 0; l < r; l++) {
    var a = e[l], s = a.f;
    (s & O) === 0 && (A(a, n), (s & (d | w)) !== 0 && ((s & y) !== 0 ? mt(a, P) : nt(a)));
  }
}
let k = false;
function pe(t) {
  k = t;
}
let h;
function b(t) {
  if (t === null) throw Et(), wt;
  return h = t;
}
function he() {
  return b(S(h));
}
function de(t) {
  if (k) {
    if (S(h) !== null) throw Et(), wt;
    h = t;
  }
}
function we(t = 1) {
  if (k) {
    for (var n = t, e = h; n--; ) e = S(e);
    h = e;
  }
}
function Ee() {
  for (var t = 0, n = h; ; ) {
    if (n.nodeType === 8) {
      var e = n.data;
      if (e === nn) {
        if (t === 0) return n;
        t -= 1;
      } else (e === Qt || e === tn) && (t += 1);
    }
    var r = S(n);
    n.remove(), n = r;
  }
}
var ot, ln, an, At, xt;
function ye() {
  if (ot === void 0) {
    ot = window, ln = document, an = /Firefox/.test(navigator.userAgent);
    var t = Element.prototype, n = Node.prototype;
    At = st(n, "firstChild").get, xt = st(n, "nextSibling").get, t.__click = void 0, t.__className = void 0, t.__attributes = null, t.__style = void 0, t.__e = void 0, Text.prototype.__t = void 0;
  }
}
function G(t = "") {
  return document.createTextNode(t);
}
function et(t) {
  return At.call(t);
}
function S(t) {
  return xt.call(t);
}
function Te(t, n) {
  if (!k) return et(t);
  var e = et(h);
  if (e === null) e = h.appendChild(G());
  else if (n && e.nodeType !== 3) {
    var r = G();
    return e == null ? void 0 : e.before(r), b(r), r;
  }
  return b(e), e;
}
function ge(t, n) {
  if (!k) {
    var e = et(t);
    return e instanceof Comment && e.data === "" ? S(e) : e;
  }
  if (n && (h == null ? void 0 : h.nodeType) !== 3) {
    var r = G();
    return h == null ? void 0 : h.before(r), b(r), r;
  }
  return h;
}
function me(t, n = 1, e = false) {
  let r = k ? h : t;
  for (var l; n--; ) l = r, r = S(r);
  if (!k) return r;
  var a = r == null ? void 0 : r.nodeType;
  if (e && a !== 3) {
    var s = G();
    return r === null ? l == null ? void 0 : l.after(s) : r.before(s), b(s), s;
  }
  return b(r), r;
}
function Ae(t) {
  t.textContent = "";
}
function It(t) {
  var n = y | O, e = u !== null && (u.f & y) !== 0 ? u : null;
  return o === null || e !== null && (e.f & w) !== 0 ? n |= w : o.f |= pt, { ctx: _, deps: null, effects: null, equals: ht, f: n, fn: t, reactions: null, rv: 0, v: null, wv: 0, parent: e ?? o };
}
function xe(t) {
  const n = It(t);
  return n.equals = dt, n;
}
function Nt(t) {
  var n = t.effects;
  if (n !== null) {
    t.effects = null;
    for (var e = 0; e < n.length; e += 1) D(n[e]);
  }
}
function sn(t) {
  for (var n = t.parent; n !== null; ) {
    if ((n.f & y) === 0) return n;
    n = n.parent;
  }
  return null;
}
function un(t) {
  var n, e = o;
  z(sn(t));
  try {
    Nt(t), n = Yt(t);
  } finally {
    z(e);
  }
  return n;
}
function Ot(t) {
  var n = un(t), e = (N || (t.f & w) !== 0) && t.deps !== null ? P : d;
  A(t, e), t.equals(n) || (t.v = n, t.wv = Mt());
}
function Rt(t) {
  o === null && u === null && zt(), u !== null && (u.f & w) !== 0 && o === null && Zt(), lt && $t();
}
function on(t, n) {
  var e = n.last;
  e === null ? n.last = n.first = t : (e.next = t, t.prev = e, n.last = t);
}
function q(t, n, e, r = true) {
  var l = o, a = { ctx: _, deps: null, nodes_start: null, nodes_end: null, f: t | O, first: null, fn: n, last: null, next: null, parent: l, prev: null, teardown: null, transitions: null, wv: 0 };
  if (e) try {
    at(a), a.f |= Vt;
  } catch (i) {
    throw D(a), i;
  }
  else n !== null && nt(a);
  var s = e && a.deps === null && a.first === null && a.nodes_start === null && a.teardown === null && (a.f & (pt | U)) === 0;
  if (!s && r && (l !== null && on(a, l), u !== null && (u.f & y) !== 0)) {
    var c = u;
    (c.effects ?? (c.effects = [])).push(a);
  }
  return a;
}
function Ie(t) {
  const n = q(X, null, false);
  return A(n, d), n.teardown = t, n;
}
function fn(t) {
  Rt();
  var n = o !== null && (o.f & I) !== 0 && _ !== null && !_.m;
  if (n) {
    var e = _;
    (e.e ?? (e.e = [])).push({ fn: t, effect: o, reaction: u });
  } else {
    var r = kt(t);
    return r;
  }
}
function Ne(t) {
  return Rt(), _n(t);
}
function Oe(t) {
  const n = q(F, t, true);
  return (e = {}) => new Promise((r) => {
    e.outro ? pn(n, () => {
      D(n), r(void 0);
    }) : (D(n), r(void 0));
  });
}
function kt(t) {
  return q(ct, t, false);
}
function _n(t) {
  return q(X, t, true);
}
function Re(t, n = [], e = It) {
  const r = n.map(e);
  return cn(() => t(...r.map(Nn)));
}
function cn(t, n = 0) {
  return q(X | rt | n, t, true);
}
function ke(t, n = true) {
  return q(X | I, t, true, n);
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
    (e.f & F) !== 0 ? e.parent = null : D(e, n), e = r;
  }
}
function vn(t) {
  for (var n = t.first; n !== null; ) {
    var e = n.next;
    (n.f & I) === 0 && D(n), n = e;
  }
}
function D(t, n = true) {
  var e = false;
  if ((n || (t.f & Gt) !== 0) && t.nodes_start !== null) {
    for (var r = t.nodes_start, l = t.nodes_end; r !== null; ) {
      var a = r === l ? null : S(r);
      r.remove(), r = a;
    }
    e = true;
  }
  St(t, n && !e), W(t, 0), A(t, Q);
  var s = t.transitions;
  if (s !== null) for (const i of s) i.stop();
  Dt(t);
  var c = t.parent;
  c !== null && c.first !== null && Ct(t), t.next = t.prev = t.teardown = t.ctx = t.deps = t.fn = t.nodes_start = t.nodes_end = null;
}
function Ct(t) {
  var n = t.parent, e = t.prev, r = t.next;
  e !== null && (e.next = r), r !== null && (r.prev = e), n !== null && (n.first === t && (n.first = r), n.last === t && (n.last = e));
}
function pn(t, n) {
  var e = [];
  bt(t, e, true), hn(e, () => {
    D(t), n && n();
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
  if ((t.f & C) === 0) {
    if (t.f ^= C, t.transitions !== null) for (const s of t.transitions) (s.is_global || e) && n.push(s);
    for (var r = t.first; r !== null; ) {
      var l = r.next, a = (r.f & vt) !== 0 || (r.f & I) !== 0;
      bt(r, n, a ? e : false), r = l;
    }
  }
}
function De(t) {
  Ft(t, true);
}
function Ft(t, n) {
  if ((t.f & C) !== 0) {
    t.f ^= C, (t.f & d) === 0 && (t.f ^= d), Y(t) && (A(t, O), nt(t));
    for (var e = t.first; e !== null; ) {
      var r = e.next, l = (e.f & vt) !== 0 || (e.f & I) !== 0;
      Ft(e, l ? n : false), e = r;
    }
    if (t.transitions !== null) for (const a of t.transitions) (a.is_global || n) && a.in();
  }
}
const dn = typeof requestIdleCallback > "u" ? (t) => setTimeout(t, 1) : requestIdleCallback;
let M = [], H = [];
function Pt() {
  var t = M;
  M = [], _t(t);
}
function qt() {
  var t = H;
  H = [], _t(t);
}
function Se(t) {
  M.length === 0 && queueMicrotask(Pt), M.push(t);
}
function Ce(t) {
  H.length === 0 && dn(qt), H.push(t);
}
function it() {
  M.length > 0 && Pt(), H.length > 0 && qt();
}
let j = false, K = false, $ = null, R = false, lt = false;
function ft(t) {
  lt = t;
}
let L = [];
let u = null, E = false;
function Z(t) {
  u = t;
}
let o = null;
function z(t) {
  o = t;
}
let m = null;
function wn(t) {
  m = t;
}
let f = null, p = 0, x = null;
function En(t) {
  x = t;
}
let Lt = 1, J = 0, N = false;
function Mt() {
  return ++Lt;
}
function Y(t) {
  var _a;
  var n = t.f;
  if ((n & O) !== 0) return true;
  if ((n & P) !== 0) {
    var e = t.deps, r = (n & w) !== 0;
    if (e !== null) {
      var l, a, s = (n & V) !== 0, c = r && o !== null && !N, i = e.length;
      if (s || c) {
        var T = t, B = T.parent;
        for (l = 0; l < i; l++) a = e[l], (s || !((_a = a == null ? void 0 : a.reactions) == null ? void 0 : _a.includes(T))) && (a.reactions ?? (a.reactions = [])).push(T);
        s && (T.f ^= V), c && B !== null && (B.f & w) === 0 && (T.f ^= w);
      }
      for (l = 0; l < i; l++) if (a = e[l], Y(a) && Ot(a), a.wv > t.wv) return true;
    }
    (!r || o !== null && !N) && A(t, d);
  }
  return false;
}
function yn(t, n) {
  for (var e = n; e !== null; ) {
    if ((e.f & U) !== 0) try {
      e.fn(t);
      return;
    } catch {
      e.f ^= U;
    }
    e = e.parent;
  }
  throw j = false, t;
}
function Tn(t) {
  return (t.f & Q) === 0 && (t.parent === null || (t.parent.f & U) === 0);
}
function tt(t, n, e, r) {
  if (j) {
    if (e === null && (j = false), Tn(n)) throw t;
    return;
  }
  e !== null && (j = true);
  {
    yn(t, n);
    return;
  }
}
function Ht(t, n, e = true) {
  var r = t.reactions;
  if (r !== null) for (var l = 0; l < r.length; l++) {
    var a = r[l];
    (a.f & y) !== 0 ? Ht(a, n, false) : n === a && (e ? A(a, O) : (a.f & d) !== 0 && A(a, P), nt(a));
  }
}
function Yt(t) {
  var _a;
  var n = f, e = p, r = x, l = u, a = N, s = m, c = _, i = E, T = t.f;
  f = null, p = 0, x = null, N = (T & w) !== 0 && (E || !R || u === null), u = (T & (I | F)) === 0 ? t : null, m = null, ut(t.ctx), E = false, J++;
  try {
    var B = (0, t.fn)(), g = t.deps;
    if (f !== null) {
      var v;
      if (W(t, p), g !== null && p > 0) for (g.length = p + f.length, v = 0; v < f.length; v++) g[p + v] = f[v];
      else t.deps = g = f;
      if (!N) for (v = p; v < g.length; v++) ((_a = g[v]).reactions ?? (_a.reactions = [])).push(t);
    } else g !== null && p < g.length && (W(t, p), g.length = p);
    if (Tt() && x !== null && !E && g !== null && (t.f & (y | P | O)) === 0) for (v = 0; v < x.length; v++) Ht(x[v], t);
    return l !== null && J++, B;
  } finally {
    f = n, p = e, x = r, u = l, N = a, m = s, ut(c), E = i;
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
  e === null && (n.f & y) !== 0 && (f === null || !f.includes(n)) && (A(n, P), (n.f & (w | V)) === 0 && (n.f ^= V), Nt(n), W(n, 0));
}
function W(t, n) {
  var e = t.deps;
  if (e !== null) for (var r = n; r < e.length; r++) gn(t, e[r]);
}
function at(t) {
  var n = t.f;
  if ((n & Q) === 0) {
    A(t, d);
    var e = o, r = _, l = R;
    o = t, R = true;
    try {
      (n & rt) !== 0 ? vn(t) : St(t), Dt(t);
      var a = Yt(t);
      t.teardown = typeof a == "function" ? a : null, t.wv = Lt;
      var s = t.deps, c;
    } catch (i) {
      tt(i, t, e, r || t.ctx);
    } finally {
      R = l, o = e;
    }
  }
}
function mn() {
  try {
    Jt();
  } catch (t) {
    if ($ !== null) tt(t, $, null);
    else throw t;
  }
}
function Bt() {
  var t = R;
  try {
    var n = 0;
    for (R = true; L.length > 0; ) {
      n++ > 1e3 && mn();
      var e = L, r = e.length;
      L = [];
      for (var l = 0; l < r; l++) {
        var a = xn(e[l]);
        An(a);
      }
    }
  } finally {
    K = false, R = t, $ = null;
  }
}
function An(t) {
  var n = t.length;
  if (n !== 0) for (var e = 0; e < n; e++) {
    var r = t[e];
    if ((r.f & (Q | C)) === 0) try {
      Y(r) && (at(r), r.deps === null && r.first === null && r.nodes_start === null && (r.teardown === null ? Ct(r) : r.fn = null));
    } catch (l) {
      tt(l, r, null, r.ctx);
    }
  }
}
function nt(t) {
  K || (K = true, queueMicrotask(Bt));
  for (var n = $ = t; n.parent !== null; ) {
    n = n.parent;
    var e = n.f;
    if ((e & (F | I)) !== 0) {
      if ((e & d) === 0) return;
      n.f ^= d;
    }
  }
  L.push(n);
}
function xn(t) {
  for (var n = [], e = t; e !== null; ) {
    var r = e.f, l = (r & (I | F)) !== 0, a = l && (r & d) !== 0;
    if (!a && (r & C) === 0) {
      if ((r & ct) !== 0) n.push(e);
      else if (l) e.f ^= d;
      else {
        var s = u;
        try {
          u = e, Y(e) && at(e);
        } catch (T) {
          tt(T, e, null, e.ctx);
        } finally {
          u = s;
        }
      }
      var c = e.first;
      if (c !== null) {
        e = c;
        continue;
      }
    }
    var i = e.parent;
    for (e = e.next; e === null && i !== null; ) e = i.next, i = i.parent;
  }
  return n;
}
function In(t) {
  var n;
  for (it(); L.length > 0; ) K = true, Bt(), it();
  return n;
}
async function be() {
  await Promise.resolve(), In();
}
function Nn(t) {
  var n = t.f, e = (n & y) !== 0;
  if (u !== null && !E) {
    m !== null && m.includes(t) && Wt();
    var r = u.deps;
    t.rv < J && (t.rv = J, f === null && r !== null && r[p] === t ? p++ : f === null ? f = [t] : (!N || !f.includes(t)) && f.push(t));
  } else if (e && t.deps === null && t.effects === null) {
    var l = t, a = l.parent;
    a !== null && (a.f & w) === 0 && (l.f ^= w);
  }
  return e && (l = t, Y(l) && Ot(l)), t.v;
}
function jt(t) {
  var n = E;
  try {
    return E = true, t();
  } finally {
    E = n;
  }
}
const On = -7169;
function A(t, n) {
  t.f = t.f & On | n;
}
function Rn(t) {
  _ === null && yt(), fn(() => {
    const n = jt(t);
    if (typeof n == "function") return n;
  });
}
function Fe(t) {
  _ === null && yt(), Rn(() => () => jt(t));
}
export {
  Zn as $,
  tn as A,
  Ee as B,
  pe as C,
  De as D,
  vt as E,
  bn as F,
  Fn as G,
  wt as H,
  gt as I,
  Kn as J,
  st as K,
  o as L,
  Gn as M,
  Pn as N,
  kn as O,
  jt as P,
  Se as Q,
  G as R,
  Hn as S,
  zn as T,
  ue as U,
  xe as V,
  nn as W,
  C as X,
  Dn as Y,
  $n as Z,
  rn as _,
  fe as a,
  ce as a0,
  Jn as a1,
  bt as a2,
  Ae as a3,
  hn as a4,
  Wn as a5,
  Rn as a6,
  ln as a7,
  we as a8,
  It as a9,
  ye as aA,
  Un as aB,
  Oe as aC,
  _ as aD,
  rt as aE,
  Vt as aF,
  le as aG,
  qn as aH,
  ee as aI,
  re as aJ,
  Kt as aK,
  fn as aa,
  Ln as ab,
  Vn as ac,
  Qn as ad,
  dt as ae,
  te as af,
  Yn as ag,
  Xn as ah,
  ne as ai,
  Mn as aj,
  Fe as ak,
  an as al,
  ae as am,
  se as an,
  oe as ao,
  Ce as ap,
  Bn as aq,
  Cn as ar,
  In as as,
  Sn as at,
  Ne as au,
  be as av,
  Z as aw,
  z as ax,
  u as ay,
  Gt as az,
  he as b,
  Te as c,
  cn as d,
  pn as e,
  ge as f,
  ke as g,
  k as h,
  h as i,
  Nn as j,
  _e as k,
  ve as l,
  S as m,
  jn as n,
  Et as o,
  ie as p,
  b as q,
  de as r,
  me as s,
  Re as t,
  et as u,
  D as v,
  _n as w,
  kt as x,
  Ie as y,
  Qt as z
};
