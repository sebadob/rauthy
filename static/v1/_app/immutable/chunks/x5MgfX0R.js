import { t as d, a as o } from "./CWf9OOFK.js";
import { c as v, s as f, r as l, t as c } from "./nlANaGLT.js";
import { s as L } from "./BmMHVVX3.js";
import { i as _ } from "./DOjUa9u5.js";
import { s as b } from "./BgMuVWsk.js";
import { s as k, b as V, c as j } from "./CZv_AhHu.js";
var q = d('<div class="label font-label svelte-ud9pp2"> </div>'), w = d('<div class="button svelte-ud9pp2"><!></div>'), y = d('<div class="container svelte-ud9pp2"><!> <div class="flex gap-05"><div><!></div> <!></div></div>');
function F(u, a) {
  var r = y(), n = v(r);
  {
    var p = (t) => {
      var e = q(), s = v(e, true);
      l(e), c(() => L(s, a.label)), o(t, e);
    };
    _(n, (t) => {
      a.label && t(p);
    });
  }
  var m = f(n, 2), i = v(m), x = v(i);
  b(x, () => a.children), l(i);
  var g = f(i, 2);
  {
    var h = (t) => {
      var e = w(), s = v(e);
      b(s, () => a.button), l(e), o(t, e);
    };
    _(g, (t) => {
      a.button && t(h);
    });
  }
  l(m), l(r), c(() => {
    k(i, "title", a.title || a.label || ""), V(i, 1, j(a.mono ? "font-mono" : ""));
  }), o(u, r);
}
export {
  F as L
};
