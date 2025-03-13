import { j as t, k as e, l as i } from "./Ck6jKiur.js";
import { p as f } from "./ho0YXExL.js";
import { f as a } from "./BO1A6s0c.js";
import { i as s } from "./B21bTIl7.js";
let o = e(void 0);
function h() {
  return !t(o) && s() && a("/auth/v1/i18n_config").then((n) => {
    if (n.body) {
      let r = n.body;
      r.common = n.body.common.map((m) => m === "zhhans" ? "zh" : m), i(o, f(r));
    }
  }), { admin() {
    var _a;
    return (_a = t(o)) == null ? void 0 : _a.admin;
  }, common() {
    var _a;
    return (_a = t(o)) == null ? void 0 : _a.common;
  } };
}
export {
  h as u
};
