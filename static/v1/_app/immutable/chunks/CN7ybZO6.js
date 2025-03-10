import { j as r, k as e, l as i } from "./CvlvO1XB.js";
import { p as m } from "./Wh68IIk2.js";
import { f } from "./tF66aiNY.js";
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
