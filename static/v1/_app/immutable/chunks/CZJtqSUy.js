import { t as I, a as x } from "./DLBGyKVC.js";
import { p as M, aa as Q, j as i, k as h, l as r, c as d, r as f, t as y, a as R, s as _, f as V } from "./CmQi0fbH.js";
import { e as W, s as w } from "./BjaYyaa_.js";
import { i as X } from "./C6bK2EJJ.js";
import { s as m, a as Y } from "./DOlUUCkJ.js";
import { a as Z } from "./B3JAWFL5.js";
import { p as k } from "./B_ggA-0N.js";
import { p } from "./DNJm3-SG.js";
import { g as $ } from "./B21bTIl7.js";
import { L as aa } from "./DPLO-ozG.js";
import { I as ea } from "./CHCNH3ik.js";
import { u as ta } from "./CZf6fJph.js";
import { u as sa } from "./BO1A6s0c.js";
var ia = I('<div class="flex space-between loading svelte-1210bad"><span class="flex gap-05"><span> </span> <span> </span> <span>/</span> <span> </span></span> <!></div>'), la = I('<label class="svelte-1210bad"><!></label> <input type="file" class="svelte-1210bad">', 1), ra = I('<form aria-dropeffect="move" aria-label="Upload" class="flex svelte-1210bad"><!></form>');
function xa(z, a) {
  M(a, true);
  let L = p(a, "method", 3, "PUT"), g = p(a, "accept", 19, () => [".image/jpg", "image/jpeg", "image/png", "image/svg+xml"]), U = p(a, "buttonOnly", 3, false), O = p(a, "buttonSize", 3, "1.5rem"), A = p(a, "multiple", 3, false), K = p(a, "width", 3, "17rem"), N = p(a, "height", 3, "2.5rem");
  const j = $();
  let P = ta(), b = h(false), o = h(void 0), F = h(0), c = h(0);
  Q(() => {
    !i(b) && i(o) && i(o).length > 0 && B(i(o));
  });
  function q(t) {
    var _a;
    r(o, k((_a = t.dataTransfer) == null ? void 0 : _a.files));
  }
  async function B(t) {
    var _a;
    r(F, k(t.length)), r(c, 0), r(b, true);
    for (let s of t) {
      if (g() && !g().includes(s.type)) {
        r(c, i(c) + 1);
        continue;
      }
      let e = await sa(L(), a.url, s, a.fileName || s.name);
      e.error ? console.error(e.error) : (_a = a.onSuccess) == null ? void 0 : _a.call(a), r(c, i(c) + 1);
    }
    r(o, void 0), r(b, false);
  }
  var u = ra();
  let S;
  var C = d(u);
  {
    var D = (t) => {
      var s = ia(), e = d(s), v = d(e), l = d(v);
      f(v);
      var n = _(v, 2), G = d(n, true);
      f(n);
      var T = _(n, 4), H = d(T, true);
      f(T), f(e);
      var J = _(e, 2);
      aa(J, {}), f(s), y(() => {
        w(l, `${P.common.loading ?? ""}:`), w(G, i(c)), w(H, i(F));
      }), x(t, s);
    }, E = (t) => {
      var s = la(), e = V(s);
      m(e, "for", j);
      var v = d(e);
      ea(v, { get width() {
        return O();
      } }), f(e);
      var l = _(e, 2);
      m(l, "id", j), y((n) => {
        m(e, "aria-disabled", a.disabled), m(l, "aria-hidden", U()), m(l, "accept", n), m(l, "aria-disabled", a.disabled), l.disabled = a.disabled, l.multiple = A();
      }, [() => g() && g().join(", ")]), Z(l, () => i(o), (n) => r(o, n)), x(t, s);
    };
    X(C, (t) => {
      i(b) ? t(D) : t(E, false);
    });
  }
  f(u), y(() => {
    m(u, "data-nopad", U()), S = Y(u, "", S, { width: K(), height: N() });
  }), W("drop", u, q), x(z, u), R();
}
export {
  xa as I
};
