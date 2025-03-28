import { j as i, k as e, l as o } from "./w0HvPX0p.js";
import { f as a } from "./BO1A6s0c.js";
import { i as n, r as f } from "./B21bTIl7.js";
let s = i(void 0);
function d(r) {
  return !e(s) && n() && a("/auth/v1/oidc/sessioninfo").then((t) => {
    t.status === 401 && f(r), o(s, t.body, true);
  }), { get() {
    return e(s);
  }, set(t) {
    o(s, t, true);
  } };
}
export {
  d as u
};
