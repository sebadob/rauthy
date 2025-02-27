import { d as A, h as i, b as y, E as I, z as N, A as g, q as p, B as _, C as o, g as u, e as h, U as R, i as S } from "./D-nwkJyM.js";
function q(d, b, v = false) {
  i && y();
  var e = d, a = null, s = null, f = R, E = v ? I : 0, r = false;
  const T = (l, t = true) => {
    r = true, n(t, l);
  }, n = (l, t) => {
    if (f === (f = l)) return;
    let c = false;
    if (i) {
      const m = e.data === N;
      !!f === m && (e = g(), p(e), _(false), c = true);
    }
    f ? (a ? o(a) : t && (a = u(() => t(e))), s && h(s, () => {
      s = null;
    })) : (s ? o(s) : t && (s = u(() => t(e))), a && h(a, () => {
      a = null;
    })), c && _(true);
  };
  A(() => {
    r = false, b(T), r || n(null, null);
  }, E), i && (e = S);
}
export {
  q as i
};
