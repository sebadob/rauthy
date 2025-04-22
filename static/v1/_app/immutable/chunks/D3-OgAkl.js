import { t as m, a as u } from "./BxmJRzoY.js";
import { p as c, j as p, c as i, s as f, r as o, t as h, k as g, l as k, a as b } from "./w0HvPX0p.js";
import { s as _, e as y, b as B } from "./BzP2S3Z_.js";
import { s as l } from "./BdbQ6g_y.js";
import { B as q } from "./C4AV2CoD.js";
var x = m('<div class="inline svelte-1qyka17"><img alt="Provider Logo" width="20" height="20" class="svelte-1qyka17"> <span class="name svelte-1qyka17"> </span></div>');
function z(d, e) {
  c(e, true);
  let s = p(false);
  q(d, { get ariaLabel() {
    return e.ariaLabel;
  }, level: 2, onclick: () => e.onclick(e.provider.id), children: (v, L) => {
    var t = x(), a = i(t), r = f(a, 2), n = i(r, true);
    o(r), o(t), h(() => {
      l(a, "src", `/auth/v1/providers/${e.provider.id}/img?updated=${e.provider.updated}`), l(a, "aria-hidden", !g(s)), _(n, e.provider.name);
    }), y("load", a, () => k(s, true)), B(a), u(v, t);
  }, $$slots: { default: true } }), b();
}
export {
  z as B
};
