import { j as o, k as a, l as e } from "./CmQi0fbH.js";
import { p as r } from "./B_ggA-0N.js";
import { f } from "./BO1A6s0c.js";
import { i as n, r as m } from "./B21bTIl7.js";
let t = a(void 0);
function g(i) {
  return !o(t) && n() && f("/auth/v1/oidc/sessioninfo").then((s) => {
    s.status === 401 && m(i), e(t, r(s.body));
  }), { get() {
    return o(t);
  }, set(s) {
    e(t, r(s));
  } };
}
export {
  g as u
};
