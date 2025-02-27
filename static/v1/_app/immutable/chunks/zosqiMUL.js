import { x as t, w as S, O as b, P as h, S as k } from "./CvlvO1XB.js";
function u(r, i) {
  return r === i || (r == null ? void 0 : r[k]) === i;
}
function T(r = {}, i, a, x) {
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
  T as b
};
