import { I as Z, d as $, J as G, q as M, h as C, u as z, b as F, k as y, K as j, A as ee, B as q, C as S, i as R, L as ae, D as J, g as K, e as re, M as b, N as P, G as ne, O as V, P as k, Q as Y, R as L, T as ie, V as B, W as fe, X as le, Y as se, Z as ue, v as te, _ as ve, m as de, $ as _e } from "./w0HvPX0p.js";
function pe(l, e) {
  return e;
}
function oe(l, e, a, u) {
  for (var v = [], _ = e.length, t = 0; t < _; t++) le(e[t].e, v, true);
  var o = _ > 0 && v.length === 0 && a !== null;
  if (o) {
    var A = a.parentNode;
    se(A), A.append(a), u.clear(), x(l, e[0].prev, e[_ - 1].next);
  }
  ue(v, () => {
    for (var h = 0; h < _; h++) {
      var d = e[h];
      o || (u.delete(d.k), x(l, d.prev, d.next)), te(d.e, !o);
    }
  });
}
function Ae(l, e, a, u, v, _ = null) {
  var t = l, o = { flags: e, items: /* @__PURE__ */ new Map(), first: null }, A = (e & G) !== 0;
  if (A) {
    var h = l;
    t = C ? M(z(h)) : h.appendChild(Z());
  }
  C && F();
  var d = null, N = false, i = j(() => {
    var s = a();
    return _e(s) ? s : s == null ? [] : P(s);
  });
  $(() => {
    var s = y(i), r = s.length;
    if (N && r === 0) return;
    N = r === 0;
    let I = false;
    if (C) {
      var E = t.data === ee;
      E !== (r === 0) && (t = q(), M(t), S(false), I = true);
    }
    if (C) {
      for (var p = null, T, c = 0; c < r; c++) {
        if (R.nodeType === 8 && R.data === ae) {
          t = R, I = true, S(false);
          break;
        }
        var n = s[c], f = u(n, c);
        T = Q(R, o, p, null, n, f, c, v, e, a), o.items.set(f, T), p = T;
      }
      r > 0 && M(q());
    }
    C || ce(s, o, t, v, e, u, a), _ !== null && (r === 0 ? d ? J(d) : d = K(() => _(t)) : d !== null && re(d, () => {
      d = null;
    })), I && S(true), y(i);
  }), C && (t = R);
}
function ce(l, e, a, u, v, _, t) {
  var _a, _b, _c, _d;
  var o = (v & fe) !== 0, A = (v & (k | L)) !== 0, h = l.length, d = e.items, N = e.first, i = N, s, r = null, I, E = [], p = [], T, c, n, f;
  if (o) for (f = 0; f < h; f += 1) T = l[f], c = _(T, f), n = d.get(c), n !== void 0 && ((_a = n.a) == null ? void 0 : _a.measure(), (I ?? (I = /* @__PURE__ */ new Set())).add(n));
  for (f = 0; f < h; f += 1) {
    if (T = l[f], c = _(T, f), n = d.get(c), n === void 0) {
      var U = i ? i.e.nodes_start : a;
      r = Q(U, e, r, r === null ? e.first : r.next, T, c, f, u, v, t), d.set(c, r), E = [], p = [], i = r.next;
      continue;
    }
    if (A && he(n, T, f, v), (n.e.f & b) !== 0 && (J(n.e), o && ((_b = n.a) == null ? void 0 : _b.unfix(), (I ?? (I = /* @__PURE__ */ new Set())).delete(n))), n !== i) {
      if (s !== void 0 && s.has(n)) {
        if (E.length < p.length) {
          var g = p[0], m;
          r = g.prev;
          var O = E[0], D = E[E.length - 1];
          for (m = 0; m < E.length; m += 1) X(E[m], g, a);
          for (m = 0; m < p.length; m += 1) s.delete(p[m]);
          x(e, O.prev, D.next), x(e, r, O), x(e, D, g), i = g, r = D, f -= 1, E = [], p = [];
        } else s.delete(n), X(n, i, a), x(e, n.prev, n.next), x(e, n, r === null ? e.first : r.next), x(e, r, n), r = n;
        continue;
      }
      for (E = [], p = []; i !== null && i.k !== c; ) (i.e.f & b) === 0 && (s ?? (s = /* @__PURE__ */ new Set())).add(i), p.push(i), i = i.next;
      if (i === null) continue;
      n = i;
    }
    E.push(n), r = n, i = n.next;
  }
  if (i !== null || s !== void 0) {
    for (var w = s === void 0 ? [] : P(s); i !== null; ) (i.e.f & b) === 0 && w.push(i), i = i.next;
    var H = w.length;
    if (H > 0) {
      var W = (v & G) !== 0 && h === 0 ? a : null;
      if (o) {
        for (f = 0; f < H; f += 1) (_c = w[f].a) == null ? void 0 : _c.measure();
        for (f = 0; f < H; f += 1) (_d = w[f].a) == null ? void 0 : _d.fix();
      }
      oe(e, w, W, d);
    }
  }
  o && ne(() => {
    var _a2;
    if (I !== void 0) for (n of I) (_a2 = n.a) == null ? void 0 : _a2.apply();
  }), V.first = e.first && e.first.e, V.last = r && r.e;
}
function he(l, e, a, u) {
  (u & k) !== 0 && Y(l.v, e), (u & L) !== 0 ? Y(l.i, a) : l.i = a;
}
function Q(l, e, a, u, v, _, t, o, A, h) {
  var d = (A & k) !== 0, N = (A & ve) === 0, i = d ? N ? ie(v) : B(v) : v, s = (A & L) === 0 ? t : B(t), r = { i: s, v: i, k: _, a: null, e: null, prev: a, next: u };
  try {
    return r.e = K(() => o(l, i, s, h), C), r.e.prev = a && a.e, r.e.next = u && u.e, a === null ? e.first = r : (a.next = r, a.e.next = r.e), u !== null && (u.prev = r, u.e.prev = r.e), r;
  } finally {
  }
}
function X(l, e, a) {
  for (var u = l.next ? l.next.e.nodes_start : a, v = e ? e.e.nodes_start : a, _ = l.e.nodes_start; _ !== u; ) {
    var t = de(_);
    v.before(_), _ = t;
  }
}
function x(l, e, a) {
  e === null ? l.first = a : (e.next = a, e.e.next = a && a.e), a !== null && (a.prev = e, a.e.prev = e && e.e);
}
export {
  Ae as e,
  pe as i
};
