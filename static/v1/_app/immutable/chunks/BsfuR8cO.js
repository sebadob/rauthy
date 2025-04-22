import { t as i, a as s } from "./BxmJRzoY.js";
import { p as g, c as u, r as p, t as _, a as c } from "./w0HvPX0p.js";
import { i as w } from "./iO9_dPNE.js";
import { a as y } from "./BdbQ6g_y.js";
import { p as n } from "./C6GSeq7M.js";
import { u as k } from "./B808p9S3.js";
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
