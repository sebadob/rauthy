import { t as R, a as o, e as d } from "./BxmJRzoY.js";
import { p as J, j, a1 as K, a0 as Q, l as _, c as h, r as x, s as C, t as l, k as I, a as S, a4 as W, a3 as X } from "./w0HvPX0p.js";
import { s as v } from "./BzP2S3Z_.js";
import { i as r } from "./iO9_dPNE.js";
import { b as Y, a as Z } from "./BdbQ6g_y.js";
import { k as $, l as F } from "./B21bTIl7.js";
import { u as tt } from "./DHOKTGcE.js";
import { A as et } from "./D7WltQR7.js";
import { B as at } from "./C8YTstTD.js";
import { T as it } from "./BnlWDKfj.js";
import { u as ot } from "./0cG6LBdy.js";
var st = R('<div class="ip font-mono svelte-1dc3uko"> </div>'), nt = R("<div><!></div>"), lt = R('<div role="contentinfo"><div class="ts svelte-1dc3uko"> </div> <!> <!> <!></div>');
function Tt(H, t) {
  J(t, true);
  let B = ot(), N = tt(), T = j(false), g = j(K(B.common.copyToClip));
  Q(() => {
    (/* @__PURE__ */ new Date()).getTime() - 1e3 * 30 < t.event.timestamp && (setTimeout(() => {
      _(T, true);
    }, 100), setTimeout(() => {
      _(T, false);
    }, 2500));
  });
  function L(e) {
    navigator.clipboard.writeText(e), _(g, N.common.copiedToClip, true), setTimeout(() => {
      _(g, B.common.copyToClip, true);
    }, 3e3);
  }
  var m = lt();
  let P, V;
  var p = h(m), M = h(p, true);
  x(p);
  var A = C(p, 2);
  {
    var O = (e) => {
      var a = d();
      l(() => v(a, t.event.typ)), o(e, a);
    };
    r(A, (e) => {
      t.event.typ !== "RauthyHealthy" && t.event.typ !== "RauthyUnhealthy" && t.event.typ !== "NewRauthyVersion" && t.event.typ !== "Test" && e(O);
    });
  }
  var D = C(A, 2);
  {
    var U = (e) => {
      var a = d();
      l(() => v(a, `: ${t.event.data}`)), o(e, a);
    }, q = (e, a) => {
      {
        var f = (s) => {
          const u = W(() => t.event.text || "");
          et(s, { get href() {
            return I(u);
          }, target: "_blank", children: (c, n) => {
            X();
            var i = d("New Version");
            o(c, i);
          }, $$slots: { default: true } });
        }, k = (s, u) => {
          {
            var c = (i) => {
              var y = d();
              l((b) => v(y, `${N.common.until ?? ""}
        ${b ?? ""}`), [() => t.event.data && F(t.event.data)]), o(i, y);
            }, n = (i, y) => {
              {
                var b = (w) => {
                  var E = d();
                  l(() => v(E, t.event.text)), o(w, E);
                };
                r(i, (w) => {
                  t.event.text && w(b);
                }, y);
              }
            };
            r(s, (i) => {
              t.event.typ === "IpBlacklisted" ? i(c) : i(n, false);
            }, u);
          }
        };
        r(e, (s) => {
          t.event.typ === "NewRauthyVersion" ? s(f) : s(k, false);
        }, a);
      }
    };
    r(D, (e) => {
      t.event.typ === "InvalidLogins" ? e(U) : e(q, false);
    });
  }
  var z = C(D, 2);
  {
    var G = (e) => {
      var a = nt(), f = h(a);
      at(f, { invisible: true, onclick: () => L(t.event.ip || ""), children: (k, s) => {
        it(k, { get text() {
          return I(g);
        }, yOffset: 20, children: (u, c) => {
          var n = st(), i = h(n, true);
          x(n), l(() => v(i, t.event.ip)), o(u, n);
        }, $$slots: { default: true } });
      }, $$slots: { default: true } }), x(a), o(e, a);
    };
    r(z, (e) => {
      t.event.ip && e(G);
    });
  }
  x(m), l((e, a, f) => {
    P = Y(m, 1, "event svelte-1dc3uko", null, P, e), V = Z(m, "", V, { "border-left": a }), v(M, f);
  }, [() => ({ highlight: I(T), inline: t.inline }), () => `2px solid ${$(t.event.level)}`, () => F(t.event.timestamp / 1e3)]), o(H, m), S();
}
export {
  Tt as E
};
