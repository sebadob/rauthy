import { n as H, a as x, t as A } from "./D8nUqfqE.js";
import { t as k, p as J, aa as Q, j as s, k as w, l as n, c, r as v, a as R, s as y, f as W } from "./D-nwkJyM.js";
import { e as X, s as I } from "./DmLjbmU6.js";
import { i as Y } from "./C3XMhfdI.js";
import { s as l } from "./D_HYGYLR.js";
import { a as T, L as Z } from "./BUAPoI3e.js";
import { c as $ } from "./DAX5MQt8.js";
import { p as z } from "./BiI21XkS.js";
import { p as o } from "./2rFDT0Lm.js";
import { g as ee } from "./DppGgfa0.js";
import { u as ae } from "./DMJUG0wm.js";
import { u as te } from "./emZtalxW.js";
var ie = H('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5m-13.5-9L12 3m0 0l4.5 4.5M12 3v13.5"></path></svg>');
function le(j, e) {
  let U = o(e, "opacity", 3, 0.9), u = o(e, "width", 3, "1.5rem"), b = o(e, "color", 3, "currentColor");
  var d = ie();
  l(d, "stroke-width", 2), k(() => {
    l(d, "stroke", b()), l(d, "width", u()), l(d, "opacity", U());
  }), x(j, d);
}
var se = A('<div class="flex space-between loading svelte-1210bad"><span class="flex gap-05"><span> </span> <span> </span> <span>/</span> <span> </span></span> <!></div>'), oe = A('<label class="svelte-1210bad"><!></label> <input type="file" class="svelte-1210bad">', 1), re = A('<form aria-dropeffect="move" aria-label="Upload" class="flex svelte-1210bad"><!></form>');
function we(j, e) {
  J(e, true);
  let U = o(e, "method", 3, "PUT"), u = o(e, "accept", 19, () => [".image/jpg", "image/jpeg", "image/png", "image/svg"]), b = o(e, "buttonOnly", 3, false), d = o(e, "buttonSize", 3, "1.5rem"), M = o(e, "multiple", 3, false), O = o(e, "width", 3, "17rem"), B = o(e, "height", 3, "2.5rem");
  const F = ee();
  let C = ae(), _ = w(false), f = w(void 0), L = w(0), g = w(0);
  Q(() => {
    !s(_) && s(f) && s(f).length > 0 && N(s(f));
  });
  function K(t) {
    var _a;
    n(f, z((_a = t.dataTransfer) == null ? void 0 : _a.files));
  }
  async function N(t) {
    var _a;
    n(L, z(t.length)), n(g, 0), n(_, true);
    for (let i of t) {
      if (console.log("uploading file", i), u() && !u().includes(i.type)) {
        n(g, s(g) + 1);
        continue;
      }
      let a = await te(U(), e.url, i, e.fileName || i.name);
      a.error ? console.error(a.error) : (_a = e.onSuccess) == null ? void 0 : _a.call(e), n(g, s(g) + 1);
    }
    n(f, void 0), n(_, false);
  }
  var m = re(), P = c(m);
  {
    var V = (t) => {
      var i = se(), a = c(i), h = c(a), r = c(h);
      v(h);
      var p = y(h, 2), D = c(p, true);
      v(p);
      var S = y(p, 4), E = c(S, true);
      v(S), v(a);
      var G = y(a, 2);
      Z(G, {}), v(i), k(() => {
        I(r, `${C.common.loading ?? ""}:`), I(D, s(g)), I(E, s(L));
      }), x(t, i);
    }, q = (t) => {
      var i = oe(), a = W(i);
      l(a, "for", F);
      var h = c(a);
      le(h, { get width() {
        return d();
      } }), v(a);
      var r = y(a, 2);
      l(r, "id", F), k((p) => {
        l(a, "aria-disabled", e.disabled), l(r, "aria-hidden", b()), l(r, "accept", p), l(r, "aria-disabled", e.disabled), r.disabled = e.disabled, r.multiple = M();
      }, [() => u() && u().join(", ")]), $(r, () => s(f), (p) => n(f, p)), x(t, i);
    };
    Y(P, (t) => {
      s(_) ? t(V) : t(q, false);
    });
  }
  v(m), k(() => {
    l(m, "data-nopad", b()), T(m, "width", O()), T(m, "height", B());
  }), X("drop", m, K), x(j, m), R();
}
export {
  we as I
};
