import { t as o, a as i } from "./BH6NCLk-.js";
import { p as u, c as l, r as p, t as _, a as c } from "./CvlvO1XB.js";
import { i as w } from "./BUO_AUgz.js";
import { a as h } from "./DMkkW5Nn.js";
import { p as y } from "./C6SR4G2t.js";
import { u as n } from "./DX6irJfo.js";
var k = o('<img src="/auth/v1/assets/rauthy_light.svg" alt="Rauthy Logo" width="100%" height="100%">'), R = o('<img src="/auth/v1/assets/rauthy_dark.svg" alt="Rauthy Logo" width="100%" height="100%">'), L = o("<div><!></div>");
function z(m, r) {
  u(r, true);
  let s = y(r, "width", 3, "7rem"), v = n();
  var a = L(), d = l(a);
  {
    var f = (t) => {
      var e = k();
      i(t, e);
    }, g = (t) => {
      var e = R();
      i(t, e);
    };
    w(d, (t) => {
      r.show === "dark" || r.show === void 0 && v.isDark() ? t(f) : t(g, false);
    });
  }
  p(a), _(() => {
    h(a, "width", s()), h(a, "height", s());
  }), i(m, a), c();
}
export {
  z as R
};
