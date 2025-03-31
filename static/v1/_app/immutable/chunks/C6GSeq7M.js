import { a7 as T, a8 as w, a9 as E, k as _, aa as y, ab as B, a1 as D, l as q, T as M, F as I, ac as Y, S as K, ad as N, ae as U, af as $ } from "./w0HvPX0p.js";
let v = false;
function z(e) {
  var r = v;
  try {
    return v = false, [e(), v];
  } finally {
    v = r;
  }
}
const C = { get(e, r) {
  if (!e.exclude.includes(r)) return e.props[r];
}, set(e, r) {
  return false;
}, getOwnPropertyDescriptor(e, r) {
  if (!e.exclude.includes(r) && r in e.props) return { enumerable: true, configurable: true, value: e.props[r] };
}, has(e, r) {
  return e.exclude.includes(r) ? false : r in e.props;
}, ownKeys(e) {
  return Reflect.ownKeys(e.props).filter((r) => !e.exclude.includes(r));
} };
function G(e, r, i) {
  return new Proxy({ props: e, exclude: r }, C);
}
function g(e) {
  var _a;
  return ((_a = e.ctx) == null ? void 0 : _a.d) ?? false;
}
function Z(e, r, i, d) {
  var _a;
  var h = (i & U) !== 0, A = true, c = (i & Y) !== 0, O = (i & $) !== 0, m = false, t;
  c ? [t, m] = z(() => e[r]) : t = e[r];
  var L = K in e || N in e, s = c && (((_a = T(e, r)) == null ? void 0 : _a.set) ?? (L && r in e && ((n) => e[r] = n))) || void 0, f = d, o = true, p = false, x = () => (p = true, o && (o = false, O ? f = I(d) : f = d), f);
  t === void 0 && d !== void 0 && (s && A && w(), t = x(), s && s(t));
  var l;
  if (l = () => {
    var n = e[r];
    return n === void 0 ? x() : (o = true, p = false, n);
  }, (i & E) === 0) return l;
  if (s) {
    var R = e.$$legacy;
    return function(n, u) {
      return arguments.length > 0 ? ((!u || R || m) && s(u ? l() : n), n) : l();
    };
  }
  var P = false, b = M(t), a = y(() => {
    var n = l(), u = _(b);
    return P ? (P = false, u) : b.v = n;
  });
  return c && _(a), h || (a.equals = B), function(n, u) {
    if (arguments.length > 0) {
      const S = u ? _(a) : c ? D(n) : n;
      if (!a.equals(S)) {
        if (P = true, q(b, S), p && f !== void 0 && (f = S), g(a)) return n;
        I(() => _(a));
      }
      return n;
    }
    return g(a) ? a.v : _(a);
  };
}
export {
  Z as p,
  G as r
};
