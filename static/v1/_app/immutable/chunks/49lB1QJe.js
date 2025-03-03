import { n as H, a as y, t as A } from "./BH6NCLk-.js";
import { t as k, p as J, aa as Q, j as l, k as w, l as n, c, r as v, a as R, s as x, f as W } from "./CvlvO1XB.js";
import { e as X, s as I } from "./CTI4QPiR.js";
import { i as Y } from "./BUO_AUgz.js";
import { s as i } from "./BMbqVy6X.js";
import { a as T, L as Z } from "./DMkkW5Nn.js";
import { c as $ } from "./BhIBACXG.js";
import { p as z } from "./Wh68IIk2.js";
import { p as r } from "./C6SR4G2t.js";
import { g as ee } from "./B8wC3kJv.js";
import { u as ae } from "./D8mHI_K9.js";
import { u as te } from "./DYtiVhoA.js";
var ie = H('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5m-13.5-9L12 3m0 0l4.5 4.5M12 3v13.5"></path></svg>');
function se(j, e) {
  let U = r(e, "opacity", 3, 0.9), u = r(e, "width", 3, "1.5rem"), b = r(e, "color", 3, "currentColor");
  var d = ie();
  i(d, "stroke-width", 2), k(() => {
    i(d, "stroke", b()), i(d, "width", u()), i(d, "opacity", U());
  }), y(j, d);
}
var le = A('<div class="flex space-between loading svelte-1210bad"><span class="flex gap-05"><span> </span> <span> </span> <span>/</span> <span> </span></span> <!></div>'), re = A('<label class="svelte-1210bad"><!></label> <input type="file" class="svelte-1210bad">', 1), oe = A('<form aria-dropeffect="move" aria-label="Upload" class="flex svelte-1210bad"><!></form>');
function we(j, e) {
  J(e, true);
  let U = r(e, "method", 3, "PUT"), u = r(e, "accept", 19, () => [".image/jpg", "image/jpeg", "image/png", "image/svg+xml"]), b = r(e, "buttonOnly", 3, false), d = r(e, "buttonSize", 3, "1.5rem"), M = r(e, "multiple", 3, false), O = r(e, "width", 3, "17rem"), B = r(e, "height", 3, "2.5rem");
  const F = ee();
  let C = ae(), _ = w(false), f = w(void 0), L = w(0), g = w(0);
  Q(() => {
    !l(_) && l(f) && l(f).length > 0 && N(l(f));
  });
  function K(t) {
    var _a;
    n(f, z((_a = t.dataTransfer) == null ? void 0 : _a.files));
  }
  async function N(t) {
    var _a;
    n(L, z(t.length)), n(g, 0), n(_, true);
    for (let s of t) {
      if (u() && !u().includes(s.type)) {
        n(g, l(g) + 1);
        continue;
      }
      let a = await te(U(), e.url, s, e.fileName || s.name);
      a.error ? console.error(a.error) : (_a = e.onSuccess) == null ? void 0 : _a.call(e), n(g, l(g) + 1);
    }
    n(f, void 0), n(_, false);
  }
  var m = oe(), P = c(m);
  {
    var V = (t) => {
      var s = le(), a = c(s), h = c(a), o = c(h);
      v(h);
      var p = x(h, 2), D = c(p, true);
      v(p);
      var S = x(p, 4), E = c(S, true);
      v(S), v(a);
      var G = x(a, 2);
      Z(G, {}), v(s), k(() => {
        I(o, `${C.common.loading ?? ""}:`), I(D, l(g)), I(E, l(L));
      }), y(t, s);
    }, q = (t) => {
      var s = re(), a = W(s);
      i(a, "for", F);
      var h = c(a);
      se(h, { get width() {
        return d();
      } }), v(a);
      var o = x(a, 2);
      i(o, "id", F), k((p) => {
        i(a, "aria-disabled", e.disabled), i(o, "aria-hidden", b()), i(o, "accept", p), i(o, "aria-disabled", e.disabled), o.disabled = e.disabled, o.multiple = M();
      }, [() => u() && u().join(", ")]), $(o, () => l(f), (p) => n(f, p)), y(t, s);
    };
    Y(P, (t) => {
      l(_) ? t(V) : t(q, false);
    });
  }
  v(m), k(() => {
    i(m, "data-nopad", b()), T(m, "width", O()), T(m, "height", B());
  }), X("drop", m, K), y(j, m), R();
}
export {
  we as I
};
