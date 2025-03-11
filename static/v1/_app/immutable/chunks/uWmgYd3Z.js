import { K as R, ac as w, ad as E, ae as T, aa as B, j as d, l as D, a0 as q, P as I, af as y, S as K, ag as M, ah as Y, ai as N } from "./nlANaGLT.js";
import { p as U } from "./5u5qd9TD.js";
let v = false;
function $(e) {
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
function Z(e, r, t, _) {
  var _a;
  var h = (t & Y) !== 0, x = true, c = (t & y) !== 0, A = (t & N) !== 0, m = false, a;
  c ? [a, m] = $(() => e[r]) : a = e[r];
  var O = K in e || M in e, i = c && (((_a = R(e, r)) == null ? void 0 : _a.set) ?? (O && r in e && ((n) => e[r] = n))) || void 0, s = _, o = true, p = false, g = () => (p = true, o && (o = false, A ? s = I(_) : s = _), s);
  a === void 0 && _ !== void 0 && (i && x && w(), a = g(), i && i(a));
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
  var P = false, S = q(a), f = B(() => {
    var n = l(), u = d(S);
    return P ? (P = false, u) : S.v = n;
  });
  return h || (f.equals = T), function(n, u) {
    if (arguments.length > 0) {
      const b = u ? d(f) : c ? U(n) : n;
      return f.equals(b) || (P = true, D(S, b), p && s !== void 0 && (s = b), I(() => d(f))), n;
    }
    return d(f);
  };
}
export {
  Z as p,
  G as r
};
