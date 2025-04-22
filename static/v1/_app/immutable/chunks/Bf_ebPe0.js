import { t as I, a as x } from "./BxmJRzoY.js";
import { p as J, j as h, a5 as M, k as i, l as r, c as d, r as f, t as y, a as Q, s as _, f as R } from "./w0HvPX0p.js";
import { e as V, s as w } from "./BzP2S3Z_.js";
import { i as W } from "./iO9_dPNE.js";
import { s as m, a as X } from "./BdbQ6g_y.js";
import { a as Y } from "./Cxw7xmE1.js";
import { p } from "./C6GSeq7M.js";
import { g as Z } from "./B21bTIl7.js";
import { L as $ } from "./C4AV2CoD.js";
import { I as ee } from "./85fl2JjN.js";
import { u as ae } from "./D12OFlGX.js";
import { u as te } from "./UPFlzoow.js";
var se = I('<div class="flex space-between loading svelte-1210bad"><span class="flex gap-05"><span> </span> <span> </span> <span>/</span> <span> </span></span> <!></div>'), ie = I('<label class="svelte-1210bad"><!></label> <input type="file" class="svelte-1210bad">', 1), le = I('<form aria-dropeffect="move" aria-label="Upload" class="flex svelte-1210bad"><!></form>');
function he(k, e) {
  J(e, true);
  let z = p(e, "method", 3, "PUT"), g = p(e, "accept", 19, () => [".image/jpg", "image/jpeg", "image/png", "image/svg+xml"]), U = p(e, "buttonOnly", 3, false), L = p(e, "buttonSize", 3, "1.5rem"), O = p(e, "multiple", 3, false), A = p(e, "width", 3, "17rem"), K = p(e, "height", 3, "2.5rem");
  const j = Z();
  let N = ae(), b = h(false), o = h(void 0), F = h(0), u = h(0);
  M(() => {
    !i(b) && i(o) && i(o).length > 0 && q(i(o));
  });
  function P(t) {
    var _a;
    r(o, (_a = t.dataTransfer) == null ? void 0 : _a.files, true);
  }
  async function q(t) {
    var _a;
    r(F, t.length, true), r(u, 0), r(b, true);
    for (let s of t) {
      if (g() && !g().includes(s.type)) {
        r(u, i(u) + 1);
        continue;
      }
      let a = await te(z(), e.url, s, e.fileName || s.name);
      a.error ? console.error(a.error) : (_a = e.onSuccess) == null ? void 0 : _a.call(e), r(u, i(u) + 1);
    }
    r(o, void 0), r(b, false);
  }
  var c = le();
  let S;
  var B = d(c);
  {
    var C = (t) => {
      var s = se(), a = d(s), v = d(a), l = d(v);
      f(v);
      var n = _(v, 2), E = d(n, true);
      f(n);
      var T = _(n, 4), G = d(T, true);
      f(T), f(a);
      var H = _(a, 2);
      $(H, {}), f(s), y(() => {
        w(l, `${N.common.loading ?? ""}:`), w(E, i(u)), w(G, i(F));
      }), x(t, s);
    }, D = (t) => {
      var s = ie(), a = R(s);
      m(a, "for", j);
      var v = d(a);
      ee(v, { get width() {
        return L();
      } }), f(a);
      var l = _(a, 2);
      m(l, "id", j), y((n) => {
        m(a, "aria-disabled", e.disabled), m(l, "aria-hidden", U()), m(l, "accept", n), m(l, "aria-disabled", e.disabled), l.disabled = e.disabled, l.multiple = O();
      }, [() => g() && g().join(", ")]), Y(l, () => i(o), (n) => r(o, n)), x(t, s);
    };
    W(B, (t) => {
      i(b) ? t(C) : t(D, false);
    });
  }
  f(c), y(() => {
    m(c, "data-nopad", U()), S = X(c, "", S, { width: A(), height: K() });
  }), V("drop", c, P), x(k, c), Q();
}
export {
  he as I
};
