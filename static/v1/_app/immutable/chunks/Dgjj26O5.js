import { x as t, w as S, F as b, G as h, S as k } from "./w0HvPX0p.js";
function u(r, i) {
  return r === i || (r == null ? void 0 : r[k]) === i;
}
function c(r = {}, i, a, x) {
  return t(() => {
    var f, s;
    return S(() => {
      f = s, s = [], b(() => {
        r !== a(...s) && (i(r, ...s), f && u(a(...f), r) && i(null, ...f));
      });
    }), () => {
      h(() => {
        s && u(a(...s), r) && i(null, ...s);
      });
    };
  }), r;
}
export {
  c as b
};
