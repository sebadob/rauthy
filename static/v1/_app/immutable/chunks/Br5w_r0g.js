import { t as i, a as s } from "./YHhP1LbZ.js";
import { p as g, c as u, r as p, t as _, a as c } from "./Ck6jKiur.js";
import { i as w } from "./7JDmqCCW.js";
import { a as y } from "./BTaFr7HN.js";
import { p as n } from "./DZP54pO_.js";
import { u as k } from "./B31o65yn.js";
var R = i('<img src="/auth/v1/assets/rauthy_light.svg" alt="Rauthy Logo" width="100%" height="100%">'), L = i('<img src="/auth/v1/assets/rauthy_dark.svg" alt="Rauthy Logo" width="100%" height="100%">'), b = i("<div><!></div>");
function A(m, a) {
  g(a, true);
  let o = n(a, "width", 3, "7rem"), v = k();
  var e = b();
  let h;
  var d = u(e);
  {
    var l = (t) => {
      var r = R();
      s(t, r);
    }, f = (t) => {
      var r = L();
      s(t, r);
    };
    w(d, (t) => {
      a.show === "dark" || a.show === void 0 && v.isDark() ? t(l) : t(f, false);
    });
  }
  p(e), _(() => h = y(e, "", h, { width: o(), height: o() })), s(m, e), c();
}
export {
  A as R
};
