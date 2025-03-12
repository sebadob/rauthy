import { t as m, a as u } from "./DKC5GJ29.js";
import { p as c, c as i, s as p, r as o, t as f, j as h, k as g, l as k, a as b } from "./BveBAmlr.js";
import { s as _, e as y, b as B } from "./CYCba2oX.js";
import { s as l } from "./Dql74IOz.js";
import { B as q } from "./DlLEcmNg.js";
var x = m('<div class="inline svelte-1qyka17"><img alt="Provider Logo" width="20" height="20" class="svelte-1qyka17"> <span class="name svelte-1qyka17"> </span></div>');
function z(d, e) {
  c(e, true);
  let s = g(false);
  q(d, { get ariaLabel() {
    return e.ariaLabel;
  }, level: 2, onclick: () => e.onclick(e.provider.id), children: (v, L) => {
    var t = x(), a = i(t), r = p(a, 2), n = i(r, true);
    o(r), o(t), f(() => {
      l(a, "src", `/auth/v1/providers/${e.provider.id}/img?updated=${e.provider.updated}`), l(a, "aria-hidden", !h(s)), _(n, e.provider.name);
    }), y("load", a, () => k(s, true)), B(a), u(v, t);
  }, $$slots: { default: true } }), b();
}
export {
  z as B
};
