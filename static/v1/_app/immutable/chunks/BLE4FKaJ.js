import { R as G, d as J, T as Q, q as S, h as C, u as K, b as P, j as y, V as j, A as ee, B as q, C as b, i as R, W as ae, D as U, g as W, e as re, X as M, Y as Z, Q as ne, L as V, Z as k, _ as Y, $ as L, a0 as ie, I as B, a1 as fe, a2 as le, a3 as se, a4 as ue, v as te, a5 as ve, m as de, O as _e } from "./BveBAmlr.js";
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
  var t = l, o = { flags: e, items: /* @__PURE__ */ new Map(), first: null }, A = (e & Q) !== 0;
  if (A) {
    var h = l;
    t = C ? S(K(h)) : h.appendChild(G());
  }
  C && P();
  var d = null, w = false, i = j(() => {
    var s = a();
    return _e(s) ? s : s == null ? [] : Z(s);
  });
  J(() => {
    var s = y(i), r = s.length;
    if (w && r === 0) return;
    w = r === 0;
    let I = false;
    if (C) {
      var E = t.data === ee;
      E !== (r === 0) && (t = q(), S(t), b(false), I = true);
    }
    if (C) {
      for (var p = null, T, c = 0; c < r; c++) {
        if (R.nodeType === 8 && R.data === ae) {
          t = R, I = true, b(false);
          break;
        }
        var n = s[c], f = u(n, c);
        T = $(R, o, p, null, n, f, c, v, e, a), o.items.set(f, T), p = T;
      }
      r > 0 && S(q());
    }
    C || ce(s, o, t, v, e, u, a), _ !== null && (r === 0 ? d ? U(d) : d = W(() => _(t)) : d !== null && re(d, () => {
      d = null;
    })), I && b(true), y(i);
  }), C && (t = R);
}
function ce(l, e, a, u, v, _, t) {
  var _a, _b, _c, _d;
  var o = (v & fe) !== 0, A = (v & (k | L)) !== 0, h = l.length, d = e.items, w = e.first, i = w, s, r = null, I, E = [], p = [], T, c, n, f;
  if (o) for (f = 0; f < h; f += 1) T = l[f], c = _(T, f), n = d.get(c), n !== void 0 && ((_a = n.a) == null ? void 0 : _a.measure(), (I ?? (I = /* @__PURE__ */ new Set())).add(n));
  for (f = 0; f < h; f += 1) {
    if (T = l[f], c = _(T, f), n = d.get(c), n === void 0) {
      var z = i ? i.e.nodes_start : a;
      r = $(z, e, r, r === null ? e.first : r.next, T, c, f, u, v, t), d.set(c, r), E = [], p = [], i = r.next;
      continue;
    }
    if (A && he(n, T, f, v), (n.e.f & M) !== 0 && (U(n.e), o && ((_b = n.a) == null ? void 0 : _b.unfix(), (I ?? (I = /* @__PURE__ */ new Set())).delete(n))), n !== i) {
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
      for (E = [], p = []; i !== null && i.k !== c; ) (i.e.f & M) === 0 && (s ?? (s = /* @__PURE__ */ new Set())).add(i), p.push(i), i = i.next;
      if (i === null) continue;
      n = i;
    }
    E.push(n), r = n, i = n.next;
  }
  if (i !== null || s !== void 0) {
    for (var N = s === void 0 ? [] : Z(s); i !== null; ) (i.e.f & M) === 0 && N.push(i), i = i.next;
    var H = N.length;
    if (H > 0) {
      var F = (v & Q) !== 0 && h === 0 ? a : null;
      if (o) {
        for (f = 0; f < H; f += 1) (_c = N[f].a) == null ? void 0 : _c.measure();
        for (f = 0; f < H; f += 1) (_d = N[f].a) == null ? void 0 : _d.fix();
      }
      oe(e, N, F, d);
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
function $(l, e, a, u, v, _, t, o, A, h) {
  var d = (A & k) !== 0, w = (A & ve) === 0, i = d ? w ? ie(v) : B(v) : v, s = (A & L) === 0 ? t : B(t), r = { i: s, v: i, k: _, a: null, e: null, prev: a, next: u };
  try {
    return r.e = W(() => o(l, i, s, h), C), r.e.prev = a && a.e, r.e.next = u && u.e, a === null ? e.first = r : (a.next = r, a.e.next = r.e), u !== null && (u.prev = r, u.e.prev = r.e), r;
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
