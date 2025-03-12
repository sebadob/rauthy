import { t as B, a as o, e as u } from "./DKC5GJ29.js";
import { p as K, k as F, a6 as Q, l as _, c as h, r as x, s as C, t as l, j as I, a as S, a8 as W, a9 as X } from "./BveBAmlr.js";
import { s as v } from "./CYCba2oX.js";
import { i as r } from "./D-uYoVwt.js";
import { b as Y, a as Z } from "./Dql74IOz.js";
import { p as R } from "./VbPNpVtZ.js";
import { k as $, l as H } from "./B21bTIl7.js";
import { u as tt } from "./DtT3Jahq.js";
import { A as et } from "./BoQH-Qa9.js";
import { B as at } from "./DlLEcmNg.js";
import { T as it } from "./B100j8aI.js";
import { u as ot } from "./8R5My_LO.js";
var nt = B('<div class="ip font-mono svelte-1dc3uko"> </div>'), st = B("<div><!></div>"), lt = B('<div role="contentinfo"><div class="ts svelte-1dc3uko"> </div> <!> <!> <!></div>');
function gt(L, t) {
  K(t, true);
  let N = ot(), P = tt(), T = F(false), g = F(R(N.common.copyToClip));
  Q(() => {
    (/* @__PURE__ */ new Date()).getTime() - 1e3 * 30 < t.event.timestamp && (setTimeout(() => {
      _(T, true);
    }, 100), setTimeout(() => {
      _(T, false);
    }, 2500));
  });
  function M(e) {
    navigator.clipboard.writeText(e), _(g, R(P.common.copiedToClip)), setTimeout(() => {
      _(g, R(N.common.copyToClip));
    }, 3e3);
  }
  var m = lt();
  let V, A;
  var p = h(m), O = h(p, true);
  x(p);
  var D = C(p, 2);
  {
    var U = (e) => {
      var a = u();
      l(() => v(a, t.event.typ)), o(e, a);
    };
    r(D, (e) => {
      t.event.typ !== "RauthyHealthy" && t.event.typ !== "RauthyUnhealthy" && t.event.typ !== "NewRauthyVersion" && t.event.typ !== "Test" && e(U);
    });
  }
  var E = C(D, 2);
  {
    var q = (e) => {
      var a = u();
      l(() => v(a, `: ${t.event.data}`)), o(e, a);
    }, z = (e, a) => {
      {
        var f = (n) => {
          const d = X(() => t.event.text || "");
          et(n, { get href() {
            return I(d);
          }, target: "_blank", children: (c, s) => {
            W();
            var i = u("New Version");
            o(c, i);
          }, $$slots: { default: true } });
        }, k = (n, d) => {
          {
            var c = (i) => {
              var y = u();
              l((b) => v(y, `${P.common.until ?? ""}
        ${b ?? ""}`), [() => t.event.data && H(t.event.data)]), o(i, y);
            }, s = (i, y) => {
              {
                var b = (w) => {
                  var j = u();
                  l(() => v(j, t.event.text)), o(w, j);
                };
                r(i, (w) => {
                  t.event.text && w(b);
                }, y);
              }
            };
            r(n, (i) => {
              t.event.typ === "IpBlacklisted" ? i(c) : i(s, false);
            }, d);
          }
        };
        r(e, (n) => {
          t.event.typ === "NewRauthyVersion" ? n(f) : n(k, false);
        }, a);
      }
    };
    r(E, (e) => {
      t.event.typ === "InvalidLogins" ? e(q) : e(z, false);
    });
  }
  var G = C(E, 2);
  {
    var J = (e) => {
      var a = st(), f = h(a);
      at(f, { invisible: true, onclick: () => M(t.event.ip || ""), children: (k, n) => {
        it(k, { get text() {
          return I(g);
        }, yOffset: 20, children: (d, c) => {
          var s = nt(), i = h(s, true);
          x(s), l(() => v(i, t.event.ip)), o(d, s);
        }, $$slots: { default: true } });
      }, $$slots: { default: true } }), x(a), o(e, a);
    };
    r(G, (e) => {
      t.event.ip && e(J);
    });
  }
  x(m), l((e, a, f) => {
    V = Y(m, 1, "event svelte-1dc3uko", null, V, e), A = Z(m, "", A, { "border-left": a }), v(O, f);
  }, [() => ({ highlight: I(T), inline: t.inline }), () => `2px solid ${$(t.event.level)}`, () => H(t.event.timestamp / 1e3)]), o(L, m), S();
}
export {
  gt as E
};
