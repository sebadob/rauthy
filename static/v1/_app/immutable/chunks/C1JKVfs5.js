import { j as m, k as n, l as i } from "./w0HvPX0p.js";
import { f } from "./UPFlzoow.js";
import { i as a } from "./B21bTIl7.js";
let o = m(void 0);
function d() {
  return !n(o) && a() && f("/auth/v1/i18n_config").then((t) => {
    if (t.body) {
      let r = t.body;
      r.common = t.body.common.map((e) => e === "zhhans" ? "zh" : e), i(o, r, true);
    }
  }), { admin() {
    var _a;
    return (_a = n(o)) == null ? void 0 : _a.admin;
  }, common() {
    var _a;
    return (_a = n(o)) == null ? void 0 : _a.common;
  } };
}
export {
  d as u
};
