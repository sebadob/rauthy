import { j as r, k as e, l as i } from "./BveBAmlr.js";
import { p as m } from "./VbPNpVtZ.js";
import { f } from "./BO1A6s0c.js";
import { i as a } from "./B21bTIl7.js";
let o = e(void 0);
function d() {
  return !r(o) && a() && f("/auth/v1/i18n_config").then((n) => {
    n.body && (n.body.common.map((t) => t === "zhhans" ? "zh" : t), i(o, m(n.body)));
  }), { admin() {
    var _a;
    return (_a = r(o)) == null ? void 0 : _a.admin;
  }, common() {
    var _a;
    return (_a = r(o)) == null ? void 0 : _a.common;
  } };
}
export {
  d as u
};
