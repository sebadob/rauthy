import { d as g, h as u, b as p, E as h, z as D, A as S, B as k, q, C as I, D as b, g as A, e as v, U as C, i as F } from "./BveBAmlr.js";
function L(E, m, [t, s] = [0, 0]) {
  u && t === 0 && p();
  var a = E, f = null, e = null, i = C, N = t > 0 ? h : 0, c = false;
  const R = (n, l = true) => {
    c = true, o(l, n);
  }, o = (n, l) => {
    if (i === (i = n)) return;
    let T = false;
    if (u && s !== -1) {
      if (t === 0) {
        const r = a.data;
        r === D ? s = 0 : r === S ? s = 1 / 0 : (s = parseInt(r.substring(1)), s !== s && (s = i ? 1 / 0 : -1));
      }
      const _ = s > t;
      !!i === _ && (a = k(), q(a), I(false), T = true, s = -1);
    }
    i ? (f ? b(f) : l && (f = A(() => l(a))), e && v(e, () => {
      e = null;
    })) : (e ? b(e) : l && (e = A(() => l(a, [t + 1, s]))), f && v(f, () => {
      f = null;
    })), T && I(true);
  };
  g(() => {
    c = false, m(R), c || o(null, null);
  }, N), u && (a = F);
}
export {
  L as i
};
