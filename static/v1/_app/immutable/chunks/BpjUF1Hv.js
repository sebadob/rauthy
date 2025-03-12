import { t as I, a as x } from "./DKC5GJ29.js";
import { p as M, aa as Q, j as i, k as h, l as r, c as d, r as f, t as y, a as R, s as _, f as V } from "./BveBAmlr.js";
import { e as W, s as w } from "./CYCba2oX.js";
import { i as X } from "./D-uYoVwt.js";
import { s as m, a as Y } from "./Dql74IOz.js";
import { a as Z } from "./Ds33nkO-.js";
import { p as k } from "./VbPNpVtZ.js";
import { p } from "./Db0ChEdV.js";
import { g as $ } from "./B21bTIl7.js";
import { L as ee } from "./DlLEcmNg.js";
import { I as ae } from "./DdbxeJT_.js";
import { u as te } from "./DtT3Jahq.js";
import { u as se } from "./BO1A6s0c.js";
var ie = I('<div class="flex space-between loading svelte-1210bad"><span class="flex gap-05"><span> </span> <span> </span> <span>/</span> <span> </span></span> <!></div>'), le = I('<label class="svelte-1210bad"><!></label> <input type="file" class="svelte-1210bad">', 1), re = I('<form aria-dropeffect="move" aria-label="Upload" class="flex svelte-1210bad"><!></form>');
function xe(z, e) {
  M(e, true);
  let L = p(e, "method", 3, "PUT"), g = p(e, "accept", 19, () => [".image/jpg", "image/jpeg", "image/png", "image/svg+xml"]), U = p(e, "buttonOnly", 3, false), O = p(e, "buttonSize", 3, "1.5rem"), A = p(e, "multiple", 3, false), K = p(e, "width", 3, "17rem"), N = p(e, "height", 3, "2.5rem");
  const j = $();
  let P = te(), b = h(false), o = h(void 0), F = h(0), u = h(0);
  Q(() => {
    !i(b) && i(o) && i(o).length > 0 && B(i(o));
  });
  function q(t) {
    var _a;
    r(o, k((_a = t.dataTransfer) == null ? void 0 : _a.files));
  }
  async function B(t) {
    var _a;
    r(F, k(t.length)), r(u, 0), r(b, true);
    for (let s of t) {
      if (g() && !g().includes(s.type)) {
        r(u, i(u) + 1);
        continue;
      }
      let a = await se(L(), e.url, s, e.fileName || s.name);
      a.error ? console.error(a.error) : (_a = e.onSuccess) == null ? void 0 : _a.call(e), r(u, i(u) + 1);
    }
    r(o, void 0), r(b, false);
  }
  var c = re();
  let S;
  var C = d(c);
  {
    var D = (t) => {
      var s = ie(), a = d(s), v = d(a), l = d(v);
      f(v);
      var n = _(v, 2), G = d(n, true);
      f(n);
      var T = _(n, 4), H = d(T, true);
      f(T), f(a);
      var J = _(a, 2);
      ee(J, {}), f(s), y(() => {
        w(l, `${P.common.loading ?? ""}:`), w(G, i(u)), w(H, i(F));
      }), x(t, s);
    }, E = (t) => {
      var s = le(), a = V(s);
      m(a, "for", j);
      var v = d(a);
      ae(v, { get width() {
        return O();
      } }), f(a);
      var l = _(a, 2);
      m(l, "id", j), y((n) => {
        m(a, "aria-disabled", e.disabled), m(l, "aria-hidden", U()), m(l, "accept", n), m(l, "aria-disabled", e.disabled), l.disabled = e.disabled, l.multiple = A();
      }, [() => g() && g().join(", ")]), Z(l, () => i(o), (n) => r(o, n)), x(t, s);
    };
    X(C, (t) => {
      i(b) ? t(D) : t(E, false);
    });
  }
  f(c), y(() => {
    m(c, "data-nopad", U()), S = Y(c, "", S, { width: K(), height: N() });
  }), W("drop", c, q), x(z, c), R();
}
export {
  xe as I
};
