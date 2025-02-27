import { J as R, ab as w, ac as E, ad as T, a9 as B, j as d, l as D, $ as q, O as I, ae as y, S as M, af as Y, ag as $, ah as K } from "./CvlvO1XB.js";
import { p as N } from "./Wh68IIk2.js";
let v = false;
function U(e) {
  var r = v;
  try {
    return v = false, [e(), v];
  } finally {
    v = r;
  }
}
const j = { get(e, r) {
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
function G(e, r, t) {
  return new Proxy({ props: e, exclude: r }, j);
}
function J(e, r, t, _) {
  var _a;
  var O = (t & $) !== 0, h = true, c = (t & y) !== 0, x = (t & K) !== 0, m = false, a;
  c ? [a, m] = U(() => e[r]) : a = e[r];
  var A = M in e || Y in e, i = c && (((_a = R(e, r)) == null ? void 0 : _a.set) ?? (A && r in e && ((n) => e[r] = n))) || void 0, s = _, o = true, p = false, g = () => (p = true, o && (o = false, x ? s = I(_) : s = _), s);
  a === void 0 && _ !== void 0 && (i && h && w(), a = g(), i && i(a));
  var l;
  if (l = () => {
    var n = e[r];
    return n === void 0 ? g() : (o = true, p = false, n);
  }, (t & E) === 0) return l;
  if (i) {
    var L = e.$$legacy;
    return function(n, u) {
      return arguments.length > 0 ? ((!u || L || m) && i(u ? l() : n), n) : l();
    };
  }
  var P = false, b = q(a), f = B(() => {
    var n = l(), u = d(b);
    return P ? (P = false, u) : b.v = n;
  });
  return O || (f.equals = T), function(n, u) {
    if (arguments.length > 0) {
      const S = u ? d(f) : c ? N(n) : n;
      return f.equals(S) || (P = true, D(b, S), p && s !== void 0 && (s = S), I(() => d(f))), n;
    }
    return d(f);
  };
}
export {
  J as p,
  G as r
};
