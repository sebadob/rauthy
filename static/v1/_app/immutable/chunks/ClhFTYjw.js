import { t as I, a as o, e as u } from "./DKC5GJ29.js";
import { p as G, k as D, a6 as J, l as _, c as h, r as x, s as b, t as s, j as w, a as K, a8 as Q, a9 as S } from "./BveBAmlr.js";
import { s as v } from "./CYCba2oX.js";
import { i as c } from "./D-uYoVwt.js";
import { b as W, a as X } from "./Dql74IOz.js";
import { p as C } from "./VbPNpVtZ.js";
import { k as Y, l as E } from "./B21bTIl7.js";
import { u as Z } from "./DtT3Jahq.js";
import { A as $ } from "./BPydPKHg.js";
import { B as tt } from "./DlLEcmNg.js";
import { T as et } from "./B100j8aI.js";
import { u as at } from "./8R5My_LO.js";
var it = I('<div class="ip font-mono svelte-1dc3uko"> </div>'), ot = I("<div><!></div>"), nt = I('<div role="contentinfo"><div class="ts svelte-1dc3uko"> </div> <!> <!> <!></div>');
function xt(j, t) {
  G(t, true), console.log("rendering event", t.event);
  let R = at(), B = Z(), g = D(false), T = D(C(R.common.copyToClip));
  J(() => {
    (/* @__PURE__ */ new Date()).getTime() - 1e3 * 30 < t.event.timestamp && (setTimeout(() => {
      _(g, true);
    }, 100), setTimeout(() => {
      _(g, false);
    }, 2500));
  });
  function F(e) {
    navigator.clipboard.writeText(e), _(T, C(B.common.copiedToClip)), setTimeout(() => {
      _(T, C(R.common.copyToClip));
    }, 3e3);
  }
  var r = nt();
  let N, P;
  var p = h(r), H = h(p, true);
  x(p);
  var V = b(p, 2);
  {
    var L = (e) => {
      var a = u();
      s(() => v(a, t.event.typ)), o(e, a);
    };
    c(V, (e) => {
      t.event.typ !== "RauthyHealthy" && t.event.typ !== "RauthyUnhealthy" && t.event.typ !== "NewRauthyVersion" && t.event.typ !== "Test" && e(L);
    });
  }
  var A = b(V, 2);
  {
    var M = (e) => {
      var a = u();
      s(() => v(a, `: ${t.event.data}`)), o(e, a);
    }, O = (e, a) => {
      {
        var m = (n) => {
          const d = S(() => t.event.text || "");
          $(n, { get href() {
            return w(d);
          }, target: "_blank", children: (y, l) => {
            Q();
            var i = u("New Version");
            o(y, i);
          }, $$slots: { default: true } });
        }, k = (n, d) => {
          {
            var y = (i) => {
              var f = u();
              s((z) => v(f, `${B.common.until ?? ""}
        ${z ?? ""}`), [() => t.event.data && E(t.event.data)]), o(i, f);
            }, l = (i) => {
              var f = u();
              s(() => v(f, t.event.text || "")), o(i, f);
            };
            c(n, (i) => {
              t.event.typ === "IpBlacklisted" ? i(y) : i(l, false);
            }, d);
          }
        };
        c(e, (n) => {
          t.event.typ === "NewRauthyVersion" ? n(m) : n(k, false);
        }, a);
      }
    };
    c(A, (e) => {
      t.event.typ === "InvalidLogins" ? e(M) : e(O, false);
    });
  }
  var U = b(A, 2);
  {
    var q = (e) => {
      var a = ot(), m = h(a);
      tt(m, { invisible: true, onclick: () => F(t.event.ip || ""), children: (k, n) => {
        et(k, { get text() {
          return w(T);
        }, yOffset: 20, children: (d, y) => {
          var l = it(), i = h(l, true);
          x(l), s(() => v(i, t.event.ip)), o(d, l);
        }, $$slots: { default: true } });
      }, $$slots: { default: true } }), x(a), o(e, a);
    };
    c(U, (e) => {
      t.event.ip !== void 0 && e(q);
    });
  }
  x(r), s((e, a, m) => {
    N = W(r, 1, "event svelte-1dc3uko", null, N, e), P = X(r, "", P, { "border-left": a }), v(H, m);
  }, [() => ({ highlight: w(g), inline: t.inline }), () => `2px solid ${Y(t.event.level)}`, () => E(t.event.timestamp / 1e3)]), o(j, r), K();
}
export {
  xt as E
};
