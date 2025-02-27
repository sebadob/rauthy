import { j as e, k as n, l as o } from "./D-nwkJyM.js";
import { p as r } from "./BiI21XkS.js";
import { f as a } from "./emZtalxW.js";
import { i as f, r as m } from "./DppGgfa0.js";
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
