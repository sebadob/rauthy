import { j as n, k as i, l as e } from "./CmQi0fbH.js";
import { p as f } from "./B_ggA-0N.js";
import { f as a } from "./BO1A6s0c.js";
import { i as s } from "./B21bTIl7.js";
let o = i(void 0);
function h() {
  return !n(o) && s() && a("/auth/v1/i18n_config").then((t) => {
    if (t.body) {
      let r = t.body;
      r.common = t.body.common.map((m) => m === "zhhans" ? "zh" : m), e(o, f(r));
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
  h as u
};
