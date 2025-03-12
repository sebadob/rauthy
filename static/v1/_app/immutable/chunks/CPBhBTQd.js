import { j as e, k as n, l as o } from "./BveBAmlr.js";
import { p as r } from "./VbPNpVtZ.js";
import { f as a } from "./BO1A6s0c.js";
import { i as f, r as m } from "./B21bTIl7.js";
let t = n(void 0);
function g(i) {
  return !e(t) && f() && a("/auth/v1/oidc/sessioninfo").then((s) => {
    s.status === 401 && m(i), o(t, r(s.body));
  }), { get() {
    return e(t);
  }, set(s) {
    o(t, r(s));
  } };
}
export {
  g as u
};
