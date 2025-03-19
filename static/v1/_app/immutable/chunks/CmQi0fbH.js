var Sn = Array.isArray, Vt = Array.prototype.indexOf, Cn = Array.from, bn = Object.defineProperty, ut = Object.getOwnPropertyDescriptor, Fn = Object.getOwnPropertyDescriptors, Pn = Object.prototype, Mn = Array.prototype, qn = Object.getPrototypeOf;
function Ln(t) {
  return typeof t == "function";
}
const Hn = () => {
};
function ct(t) {
  for (var n = 0; n < t.length; n++) t[n]();
}
function Yn() {
  var t, n, e = new Promise((r, l) => {
    t = r, n = l;
  });
  return { promise: e, resolve: t, reject: n };
}
const T = 2, vt = 4, tt = 8, at = 16, I = 32, F = 64, G = 128, E = 256, K = 512, h = 1024, O = 2048, P = 4096, C = 8192, nt = 16384, Gt = 32768, pt = 65536, Kt = 1 << 19, dt = 1 << 20, Bn = Symbol("$state"), jn = Symbol("legacy props"), Un = Symbol("");
function ht(t) {
  return t === this.v;
}
function $t(t, n) {
  return t != t ? n == n : t !== n || t !== null && typeof t == "object" || typeof t == "function";
}
function Vn(t, n) {
  return t !== n;
}
function wt(t) {
  return !$t(t, this.v);
}
function Zt(t) {
  throw new Error("https://svelte.dev/e/effect_in_teardown");
}
function zt() {
  throw new Error("https://svelte.dev/e/effect_in_unowned_derived");
}
function Jt(t) {
  throw new Error("https://svelte.dev/e/effect_orphan");
}
function Wt() {
  throw new Error("https://svelte.dev/e/effect_update_depth_exceeded");
}
function Gn() {
  throw new Error("https://svelte.dev/e/hydration_failed");
}
function Kn(t) {
  throw new Error("https://svelte.dev/e/props_invalid_value");
}
function $n() {
  throw new Error("https://svelte.dev/e/state_descriptors_fixed");
}
function Zn() {
  throw new Error("https://svelte.dev/e/state_prototype_fixed");
}
function Xt() {
  throw new Error("https://svelte.dev/e/state_unsafe_local_read");
}
function Qt() {
  throw new Error("https://svelte.dev/e/state_unsafe_mutation");
}
const zn = 1, Jn = 2, Wn = 4, Xn = 8, Qn = 16, te = 1, ne = 4, ee = 8, re = 16, le = 1, ae = 2, se = 4, ue = 1, oe = 2, tn = "[", nn = "[!", en = "]", Et = {}, ie = Symbol(), fe = "http://www.w3.org/1999/xhtml";
function yt(t) {
  console.warn("https://svelte.dev/e/hydration_mismatch");
}
function Tt(t) {
  throw new Error("https://svelte.dev/e/lifecycle_outside_component");
}
let _ = null;
function ot(t) {
  _ = t;
}
function _e(t, n = false, e) {
  var r = _ = { p: _, c: null, d: false, e: null, m: false, s: t, x: null, l: null };
  _n(() => {
    r.d = true;
  });
}
function ce(t) {
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
          W(a.effect), J(a.reaction), Dt(a.fn);
        }
      } finally {
        W(e), J(r);
      }
    }
    _ = n.p, n.m = true;
  }
  return t || {};
}
function gt() {
  return true;
}
const L = /* @__PURE__ */ new Map();
function mt(t, n) {
  var e = { f: 0, v: t, reactions: null, equals: ht, rv: 0, wv: 0 };
  return e;
}
function ve(t) {
  return rn(mt(t));
}
function pe(t, n = false) {
  const e = mt(t);
  return n || (e.equals = wt), e;
}
function rn(t) {
  return u !== null && !y && (u.f & T) !== 0 && (A === null ? yn([t]) : A.push(t)), t;
}
function de(t, n) {
  return u !== null && !y && gt() && (u.f & (T | at)) !== 0 && (A === null || !A.includes(t)) && Qt(), ln(t, n);
}
function ln(t, n) {
  if (!t.equals(n)) {
    var e = t.v;
    B ? L.set(t, n) : L.set(t, e), t.v = n, t.wv = Ht(), At(t, O), o !== null && (o.f & h) !== 0 && (o.f & (I | F)) === 0 && (w === null ? Tn([t]) : w.push(t));
  }
  return n;
}
function At(t, n) {
  var e = t.reactions;
  if (e !== null) for (var r = e.length, l = 0; l < r; l++) {
    var a = e[l], s = a.f;
    (s & O) === 0 && (x(a, n), (s & (h | E)) !== 0 && ((s & T) !== 0 ? At(a, P) : rt(a)));
  }
}
let k = false;
function he(t) {
  k = t;
}
let d;
function b(t) {
  if (t === null) throw yt(), Et;
  return d = t;
}
function we() {
  return b(S(d));
}
function Ee(t) {
  if (k) {
    if (S(d) !== null) throw yt(), Et;
    d = t;
  }
}
function ye(t = 1) {
  if (k) {
    for (var n = t, e = d; n--; ) e = S(e);
    d = e;
  }
}
function Te() {
  for (var t = 0, n = d; ; ) {
    if (n.nodeType === 8) {
      var e = n.data;
      if (e === en) {
        if (t === 0) return n;
        t -= 1;
      } else (e === tn || e === nn) && (t += 1);
    }
    var r = S(n);
    n.remove(), n = r;
  }
}
var it, an, sn, xt, It;
function ge() {
  if (it === void 0) {
    it = window, an = document, sn = /Firefox/.test(navigator.userAgent);
    var t = Element.prototype, n = Node.prototype;
    xt = ut(n, "firstChild").get, It = ut(n, "nextSibling").get, t.__click = void 0, t.__className = void 0, t.__attributes = null, t.__style = void 0, t.__e = void 0, Text.prototype.__t = void 0;
  }
}
function $(t = "") {
  return document.createTextNode(t);
}
function lt(t) {
  return xt.call(t);
}
function S(t) {
  return It.call(t);
}
function me(t, n) {
  if (!k) return lt(t);
  var e = lt(d);
  if (e === null) e = d.appendChild($());
  else if (n && e.nodeType !== 3) {
    var r = $();
    return e == null ? void 0 : e.before(r), b(r), r;
  }
  return b(e), e;
}
function Ae(t, n) {
  if (!k) {
    var e = lt(t);
    return e instanceof Comment && e.data === "" ? S(e) : e;
  }
  if (n && (d == null ? void 0 : d.nodeType) !== 3) {
    var r = $();
    return d == null ? void 0 : d.before(r), b(r), r;
  }
  return d;
}
function xe(t, n = 1, e = false) {
  let r = k ? d : t;
  for (var l; n--; ) l = r, r = S(r);
  if (!k) return r;
  var a = r == null ? void 0 : r.nodeType;
  if (e && a !== 3) {
    var s = $();
    return r === null ? l == null ? void 0 : l.after(s) : r.before(s), b(s), s;
  }
  return b(r), r;
}
function Ie(t) {
  t.textContent = "";
}
function Nt(t) {
  var n = T | O, e = u !== null && (u.f & T) !== 0 ? u : null;
  return o === null || e !== null && (e.f & E) !== 0 ? n |= E : o.f |= dt, { ctx: _, deps: null, effects: null, equals: ht, f: n, fn: t, reactions: null, rv: 0, v: null, wv: 0, parent: e ?? o };
}
function Ne(t) {
  const n = Nt(t);
  return n.equals = wt, n;
}
function Ot(t) {
  var n = t.effects;
  if (n !== null) {
    t.effects = null;
    for (var e = 0; e < n.length; e += 1) D(n[e]);
  }
}
function un(t) {
  for (var n = t.parent; n !== null; ) {
    if ((n.f & T) === 0) return n;
    n = n.parent;
  }
  return null;
}
function on(t) {
  var n, e = o;
  W(un(t));
  try {
    Ot(t), n = Bt(t);
  } finally {
    W(e);
  }
  return n;
}
function Rt(t) {
  var n = on(t), e = (N || (t.f & E) !== 0) && t.deps !== null ? P : h;
  x(t, e), t.equals(n) || (t.v = n, t.wv = Ht());
}
function kt(t) {
  o === null && u === null && Jt(), u !== null && (u.f & E) !== 0 && o === null && zt(), B && Zt();
}
function fn(t, n) {
  var e = n.last;
  e === null ? n.last = n.first = t : (e.next = t, t.prev = e, n.last = t);
}
function M(t, n, e, r = true) {
  var l = o, a = { ctx: _, deps: null, nodes_start: null, nodes_end: null, f: t | O, first: null, fn: n, last: null, next: null, parent: l, prev: null, teardown: null, transitions: null, wv: 0 };
  if (e) try {
    st(a), a.f |= Gt;
  } catch (i) {
    throw D(a), i;
  }
  else n !== null && rt(a);
  var s = e && a.deps === null && a.first === null && a.nodes_start === null && a.teardown === null && (a.f & (dt | G)) === 0;
  if (!s && r && (l !== null && fn(a, l), u !== null && (u.f & T) !== 0)) {
    var c = u;
    (c.effects ?? (c.effects = [])).push(a);
  }
  return a;
}
function _n(t) {
  const n = M(tt, null, false);
  return x(n, h), n.teardown = t, n;
}
function cn(t) {
  kt();
  var n = o !== null && (o.f & I) !== 0 && _ !== null && !_.m;
  if (n) {
    var e = _;
    (e.e ?? (e.e = [])).push({ fn: t, effect: o, reaction: u });
  } else {
    var r = Dt(t);
    return r;
  }
}
function Oe(t) {
  return kt(), vn(t);
}
function Re(t) {
  const n = M(F, t, true);
  return (e = {}) => new Promise((r) => {
    e.outro ? hn(n, () => {
      D(n), r(void 0);
    }) : (D(n), r(void 0));
  });
}
function Dt(t) {
  return M(vt, t, false);
}
function vn(t) {
  return M(tt, t, true);
}
function ke(t, n = [], e = Nt) {
  const r = n.map(e);
  return pn(() => t(...r.map(Rn)));
}
function pn(t, n = 0) {
  return M(tt | at | n, t, true);
}
function De(t, n = true) {
  return M(tt | I, t, true, n);
}
function St(t) {
  var n = t.teardown;
  if (n !== null) {
    const e = B, r = u;
    _t(true), J(null);
    try {
      n.call(null);
    } finally {
      _t(e), J(r);
    }
  }
}
function Ct(t, n = false) {
  var e = t.first;
  for (t.first = t.last = null; e !== null; ) {
    var r = e.next;
    (e.f & F) !== 0 ? e.parent = null : D(e, n), e = r;
  }
}
function dn(t) {
  for (var n = t.first; n !== null; ) {
    var e = n.next;
    (n.f & I) === 0 && D(n), n = e;
  }
}
function D(t, n = true) {
  var e = false;
  if ((n || (t.f & Kt) !== 0) && t.nodes_start !== null) {
    for (var r = t.nodes_start, l = t.nodes_end; r !== null; ) {
      var a = r === l ? null : S(r);
      r.remove(), r = a;
    }
    e = true;
  }
  Ct(t, n && !e), Q(t, 0), x(t, nt);
  var s = t.transitions;
  if (s !== null) for (const i of s) i.stop();
  St(t);
  var c = t.parent;
  c !== null && c.first !== null && bt(t), t.next = t.prev = t.teardown = t.ctx = t.deps = t.fn = t.nodes_start = t.nodes_end = null;
}
function bt(t) {
  var n = t.parent, e = t.prev, r = t.next;
  e !== null && (e.next = r), r !== null && (r.prev = e), n !== null && (n.first === t && (n.first = r), n.last === t && (n.last = e));
}
function hn(t, n) {
  var e = [];
  Ft(t, e, true), wn(e, () => {
    D(t), n && n();
  });
}
function wn(t, n) {
  var e = t.length;
  if (e > 0) {
    var r = () => --e || n();
    for (var l of t) l.out(r);
  } else n();
}
function Ft(t, n, e) {
  if ((t.f & C) === 0) {
    if (t.f ^= C, t.transitions !== null) for (const s of t.transitions) (s.is_global || e) && n.push(s);
    for (var r = t.first; r !== null; ) {
      var l = r.next, a = (r.f & pt) !== 0 || (r.f & I) !== 0;
      Ft(r, n, a ? e : false), r = l;
    }
  }
}
function Se(t) {
  Pt(t, true);
}
function Pt(t, n) {
  if ((t.f & C) !== 0) {
    t.f ^= C, (t.f & h) === 0 && (t.f ^= h), j(t) && (x(t, O), rt(t));
    for (var e = t.first; e !== null; ) {
      var r = e.next, l = (e.f & pt) !== 0 || (e.f & I) !== 0;
      Pt(e, l ? n : false), e = r;
    }
    if (t.transitions !== null) for (const a of t.transitions) (a.is_global || n) && a.in();
  }
}
const En = typeof requestIdleCallback > "u" ? (t) => setTimeout(t, 1) : requestIdleCallback;
let H = [], Y = [];
function Mt() {
  var t = H;
  H = [], ct(t);
}
function qt() {
  var t = Y;
  Y = [], ct(t);
}
function Ce(t) {
  H.length === 0 && queueMicrotask(Mt), H.push(t);
}
function be(t) {
  Y.length === 0 && En(qt), Y.push(t);
}
function ft() {
  H.length > 0 && Mt(), Y.length > 0 && qt();
}
let V = false, Z = false, z = null, R = false, B = false;
function _t(t) {
  B = t;
}
let q = [];
let u = null, y = false;
function J(t) {
  u = t;
}
let o = null;
function W(t) {
  o = t;
}
let A = null;
function yn(t) {
  A = t;
}
let f = null, p = 0, w = null;
function Tn(t) {
  w = t;
}
let Lt = 1, X = 0, N = false;
function Ht() {
  return ++Lt;
}
function j(t) {
  var _a;
  var n = t.f;
  if ((n & O) !== 0) return true;
  if ((n & P) !== 0) {
    var e = t.deps, r = (n & E) !== 0;
    if (e !== null) {
      var l, a, s = (n & K) !== 0, c = r && o !== null && !N, i = e.length;
      if (s || c) {
        var g = t, U = g.parent;
        for (l = 0; l < i; l++) a = e[l], (s || !((_a = a == null ? void 0 : a.reactions) == null ? void 0 : _a.includes(g))) && (a.reactions ?? (a.reactions = [])).push(g);
        s && (g.f ^= K), c && U !== null && (U.f & E) === 0 && (g.f ^= E);
      }
      for (l = 0; l < i; l++) if (a = e[l], j(a) && Rt(a), a.wv > t.wv) return true;
    }
    (!r || o !== null && !N) && x(t, h);
  }
  return false;
}
function gn(t, n) {
  for (var e = n; e !== null; ) {
    if ((e.f & G) !== 0) try {
      e.fn(t);
      return;
    } catch {
      e.f ^= G;
    }
    e = e.parent;
  }
  throw V = false, t;
}
function mn(t) {
  return (t.f & nt) === 0 && (t.parent === null || (t.parent.f & G) === 0);
}
function et(t, n, e, r) {
  if (V) {
    if (e === null && (V = false), mn(n)) throw t;
    return;
  }
  e !== null && (V = true);
  {
    gn(t, n);
    return;
  }
}
function Yt(t, n, e = true) {
  var r = t.reactions;
  if (r !== null) for (var l = 0; l < r.length; l++) {
    var a = r[l];
    (a.f & T) !== 0 ? Yt(a, n, false) : n === a && (e ? x(a, O) : (a.f & h) !== 0 && x(a, P), rt(a));
  }
}
function Bt(t) {
  var _a;
  var n = f, e = p, r = w, l = u, a = N, s = A, c = _, i = y, g = t.f;
  f = null, p = 0, w = null, N = (g & E) !== 0 && (y || !R || u === null), u = (g & (I | F)) === 0 ? t : null, A = null, ot(t.ctx), y = false, X++;
  try {
    var U = (0, t.fn)(), m = t.deps;
    if (f !== null) {
      var v;
      if (Q(t, p), m !== null && p > 0) for (m.length = p + f.length, v = 0; v < f.length; v++) m[p + v] = f[v];
      else t.deps = m = f;
      if (!N) for (v = p; v < m.length; v++) ((_a = m[v]).reactions ?? (_a.reactions = [])).push(t);
    } else m !== null && p < m.length && (Q(t, p), m.length = p);
    if (gt() && w !== null && !y && m !== null && (t.f & (T | P | O)) === 0) for (v = 0; v < w.length; v++) Yt(w[v], t);
    return l !== null && (X++, w !== null && (r === null ? r = w : r.push(...w))), U;
  } finally {
    f = n, p = e, w = r, u = l, N = a, A = s, ot(c), y = i;
  }
}
function An(t, n) {
  let e = n.reactions;
  if (e !== null) {
    var r = Vt.call(e, t);
    if (r !== -1) {
      var l = e.length - 1;
      l === 0 ? e = n.reactions = null : (e[r] = e[l], e.pop());
    }
  }
  e === null && (n.f & T) !== 0 && (f === null || !f.includes(n)) && (x(n, P), (n.f & (E | K)) === 0 && (n.f ^= K), Ot(n), Q(n, 0));
}
function Q(t, n) {
  var e = t.deps;
  if (e !== null) for (var r = n; r < e.length; r++) An(t, e[r]);
}
function st(t) {
  var n = t.f;
  if ((n & nt) === 0) {
    x(t, h);
    var e = o, r = _, l = R;
    o = t, R = true;
    try {
      (n & at) !== 0 ? dn(t) : Ct(t), St(t);
      var a = Bt(t);
      t.teardown = typeof a == "function" ? a : null, t.wv = Lt;
      var s = t.deps, c;
    } catch (i) {
      et(i, t, e, r || t.ctx);
    } finally {
      R = l, o = e;
    }
  }
}
function xn() {
  try {
    Wt();
  } catch (t) {
    if (z !== null) et(t, z, null);
    else throw t;
  }
}
function jt() {
  var t = R;
  try {
    var n = 0;
    for (R = true; q.length > 0; ) {
      n++ > 1e3 && xn();
      var e = q, r = e.length;
      q = [];
      for (var l = 0; l < r; l++) {
        var a = Nn(e[l]);
        In(a);
      }
    }
  } finally {
    Z = false, R = t, z = null, L.clear();
  }
}
function In(t) {
  var n = t.length;
  if (n !== 0) for (var e = 0; e < n; e++) {
    var r = t[e];
    if ((r.f & (nt | C)) === 0) try {
      j(r) && (st(r), r.deps === null && r.first === null && r.nodes_start === null && (r.teardown === null ? bt(r) : r.fn = null));
    } catch (l) {
      et(l, r, null, r.ctx);
    }
  }
}
function rt(t) {
  Z || (Z = true, queueMicrotask(jt));
  for (var n = z = t; n.parent !== null; ) {
    n = n.parent;
    var e = n.f;
    if ((e & (F | I)) !== 0) {
      if ((e & h) === 0) return;
      n.f ^= h;
    }
  }
  q.push(n);
}
function Nn(t) {
  for (var n = [], e = t; e !== null; ) {
    var r = e.f, l = (r & (I | F)) !== 0, a = l && (r & h) !== 0;
    if (!a && (r & C) === 0) {
      if ((r & vt) !== 0) n.push(e);
      else if (l) e.f ^= h;
      else {
        var s = u;
        try {
          u = e, j(e) && st(e);
        } catch (g) {
          et(g, e, null, e.ctx);
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
function On(t) {
  var n;
  for (ft(); q.length > 0; ) Z = true, jt(), ft();
  return n;
}
async function Fe() {
  await Promise.resolve(), On();
}
function Rn(t) {
  var n = t.f, e = (n & T) !== 0;
  if (u !== null && !y) {
    A !== null && A.includes(t) && Xt();
    var r = u.deps;
    t.rv < X && (t.rv = X, f === null && r !== null && r[p] === t ? p++ : f === null ? f = [t] : (!N || !f.includes(t)) && f.push(t));
  } else if (e && t.deps === null && t.effects === null) {
    var l = t, a = l.parent;
    a !== null && (a.f & E) === 0 && (l.f ^= E);
  }
  return e && (l = t, j(l) && Rt(l)), B && L.has(t) ? L.get(t) : t.v;
}
function Ut(t) {
  var n = y;
  try {
    return y = true, t();
  } finally {
    y = n;
  }
}
const kn = -7169;
function x(t, n) {
  t.f = t.f & kn | n;
}
function Dn(t) {
  _ === null && Tt(), cn(() => {
    const n = Ut(t);
    if (typeof n == "function") return n;
  });
}
function Pe(t) {
  _ === null && Tt(), Dn(() => () => Ut(t));
}
export {
  Jn as $,
  nn as A,
  Te as B,
  he as C,
  Se as D,
  pt as E,
  Pn as F,
  Mn as G,
  Et as H,
  mt as I,
  Zn as J,
  ut as K,
  o as L,
  $n as M,
  qn as N,
  Sn as O,
  Ut as P,
  Ce as Q,
  $ as R,
  Bn as S,
  Wn as T,
  ie as U,
  Ne as V,
  en as W,
  C as X,
  Cn as Y,
  zn as Z,
  ln as _,
  ce as a,
  pe as a0,
  Xn as a1,
  Ft as a2,
  Ie as a3,
  wn as a4,
  Qn as a5,
  Dn as a6,
  an as a7,
  ye as a8,
  Nt as a9,
  ge as aA,
  Gn as aB,
  Re as aC,
  _ as aD,
  at as aE,
  Gt as aF,
  se as aG,
  Ln as aH,
  le as aI,
  ae as aJ,
  $t as aK,
  cn as aa,
  Hn as ab,
  Kn as ac,
  ne as ad,
  wt as ae,
  ee as af,
  jn as ag,
  te as ah,
  re as ai,
  Yn as aj,
  Pe as ak,
  sn as al,
  ue as am,
  oe as an,
  fe as ao,
  be as ap,
  Un as aq,
  Fn as ar,
  On as as,
  bn as at,
  Oe as au,
  Fe as av,
  J as aw,
  W as ax,
  u as ay,
  Kt as az,
  we as b,
  me as c,
  pn as d,
  hn as e,
  Ae as f,
  De as g,
  k as h,
  d as i,
  Rn as j,
  ve as k,
  de as l,
  S as m,
  Vn as n,
  yt as o,
  _e as p,
  b as q,
  Ee as r,
  xe as s,
  ke as t,
  lt as u,
  D as v,
  vn as w,
  Dt as x,
  _n as y,
  tn as z
};
