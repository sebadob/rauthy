import { K as w, ac as E, ad as T, j as _, a9 as y, ae as B, l as D, a0 as q, P as x, af as K, S as M, ag as Y, ah as N, ai as U } from "./CmQi0fbH.js";
import { p as $ } from "./B_ggA-0N.js";
let o = false;
function j(e) {
  var r = o;
  try {
    return o = false, [e(), o];
  } finally {
    o = r;
  }
}
const z = { get(e, r) {
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
function Z(e, r, i) {
  return new Proxy({ props: e, exclude: r }, z);
}
function I(e) {
  var _a;
  return ((_a = e.ctx) == null ? void 0 : _a.d) ?? false;
}
function F(e, r, i, d) {
  var _a;
  var h = (i & N) !== 0, A = true, c = (i & K) !== 0, O = (i & U) !== 0, b = false, t;
  c ? [t, b] = j(() => e[r]) : t = e[r];
  var L = M in e || Y in e, s = c && (((_a = w(e, r)) == null ? void 0 : _a.set) ?? (L && r in e && ((n) => e[r] = n))) || void 0, f = d, v = true, p = false, g = () => (p = true, v && (v = false, O ? f = x(d) : f = d), f);
  t === void 0 && d !== void 0 && (s && A && E(), t = g(), s && s(t));
  var l;
  if (l = () => {
    var n = e[r];
    return n === void 0 ? g() : (v = true, p = false, n);
  }, (i & T) === 0) return l;
  if (s) {
    var R = e.$$legacy;
    return function(n, u) {
      return arguments.length > 0 ? ((!u || R || b) && s(u ? l() : n), n) : l();
    };
  }
  var P = false, m = q(t), a = y(() => {
    var n = l(), u = _(m);
    return P ? (P = false, u) : m.v = n;
  });
  return c && _(a), h || (a.equals = B), function(n, u) {
    if (arguments.length > 0) {
      const S = u ? _(a) : c ? $(n) : n;
      if (!a.equals(S)) {
        if (P = true, D(m, S), p && f !== void 0 && (f = S), I(a)) return n;
        x(() => _(a));
      }
      return n;
    }
    return I(a) ? a.v : _(a);
  };
}
export {
  F as p,
  Z as r
};
