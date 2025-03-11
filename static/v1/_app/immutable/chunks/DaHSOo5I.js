import { t as I, a as x } from "./BH6NCLk-.js";
import { p as M, aa as Q, j as s, k as h, l as r, c as f, r as m, t as y, a as R, s as _, f as V } from "./CvlvO1XB.js";
import { e as W, s as w } from "./CTI4QPiR.js";
import { i as X } from "./BUO_AUgz.js";
import { s as p } from "./BMbqVy6X.js";
import { a as T, L as Y } from "./CqH8LXO-.js";
import { a as Z } from "./dU6E9WaN.js";
import { p as k } from "./Wh68IIk2.js";
import { p as u } from "./C6SR4G2t.js";
import { g as $ } from "./B21bTIl7.js";
import { I as aa } from "./BWnKfvFA.js";
import { u as ea } from "./D8mHI_K9.js";
import { u as ta } from "./BO1A6s0c.js";
var ia = I('<div class="flex space-between loading svelte-1210bad"><span class="flex gap-05"><span> </span> <span> </span> <span>/</span> <span> </span></span> <!></div>'), sa = I('<label class="svelte-1210bad"><!></label> <input type="file" class="svelte-1210bad">', 1), la = I('<form aria-dropeffect="move" aria-label="Upload" class="flex svelte-1210bad"><!></form>');
function _a(z, a) {
  M(a, true);
  let L = u(a, "method", 3, "PUT"), g = u(a, "accept", 19, () => [".image/jpg", "image/jpeg", "image/png", "image/svg+xml"]), U = u(a, "buttonOnly", 3, false), O = u(a, "buttonSize", 3, "1.5rem"), A = u(a, "multiple", 3, false), K = u(a, "width", 3, "17rem"), N = u(a, "height", 3, "2.5rem");
  const j = $();
  let P = ea(), b = h(false), o = h(void 0), F = h(0), c = h(0);
  Q(() => {
    !s(b) && s(o) && s(o).length > 0 && B(s(o));
  });
  function q(t) {
    var _a2;
    r(o, k((_a2 = t.dataTransfer) == null ? void 0 : _a2.files));
  }
  async function B(t) {
    var _a2;
    r(F, k(t.length)), r(c, 0), r(b, true);
    for (let i of t) {
      if (g() && !g().includes(i.type)) {
        r(c, s(c) + 1);
        continue;
      }
      let e = await ta(L(), a.url, i, a.fileName || i.name);
      e.error ? console.error(e.error) : (_a2 = a.onSuccess) == null ? void 0 : _a2.call(a), r(c, s(c) + 1);
    }
    r(o, void 0), r(b, false);
  }
  var n = la(), C = f(n);
  {
    var D = (t) => {
      var i = ia(), e = f(i), v = f(e), l = f(v);
      m(v);
      var d = _(v, 2), G = f(d, true);
      m(d);
      var S = _(d, 4), H = f(S, true);
      m(S), m(e);
      var J = _(e, 2);
      Y(J, {}), m(i), y(() => {
        w(l, `${P.common.loading ?? ""}:`), w(G, s(c)), w(H, s(F));
      }), x(t, i);
    }, E = (t) => {
      var i = sa(), e = V(i);
      p(e, "for", j);
      var v = f(e);
      aa(v, { get width() {
        return O();
      } }), m(e);
      var l = _(e, 2);
      p(l, "id", j), y((d) => {
        p(e, "aria-disabled", a.disabled), p(l, "aria-hidden", U()), p(l, "accept", d), p(l, "aria-disabled", a.disabled), l.disabled = a.disabled, l.multiple = A();
      }, [() => g() && g().join(", ")]), Z(l, () => s(o), (d) => r(o, d)), x(t, i);
    };
    X(C, (t) => {
      s(b) ? t(D) : t(E, false);
    });
  }
  m(n), y(() => {
    p(n, "data-nopad", U()), T(n, "width", K()), T(n, "height", N());
  }), W("drop", n, q), x(z, n), R();
}
export {
  _a as I
};
