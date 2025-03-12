import { w as f, x as u, y as d } from "./BveBAmlr.js";
import { l as v, w as n } from "./CYCba2oX.js";
function x(l, e, c = e) {
  var w = l === "x", o = () => n(() => {
    r = true, clearTimeout(t), t = setTimeout(s, 100), c(window[w ? "scrollX" : "scrollY"]);
  });
  addEventListener("scroll", o, { passive: true });
  var r = false, t, s = () => {
    r = false;
  }, i = true;
  f(() => {
    var a = e();
    i ? i = false : !r && a != null && (r = true, clearTimeout(t), scrollTo(window.scrollX, a), t = setTimeout(s, 100));
  }), u(o), d(() => {
    removeEventListener("scroll", o);
  });
}
function T(l, e) {
  v(window, ["resize"], () => n(() => e(window[l])));
}
export {
  x as a,
  T as b
};
