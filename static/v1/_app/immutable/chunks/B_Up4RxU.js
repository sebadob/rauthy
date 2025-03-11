import { t as u, a as r, e as F, d as Q } from "./CWf9OOFK.js";
import { p as S, k as H, a6 as W, l as T, c as k, r as b, s as h, t as f, j as N, a as X, f as w, a9 as Y, a8 as Z, aa as $ } from "./nlANaGLT.js";
import { s as d } from "./BmMHVVX3.js";
import { i as x } from "./DOjUa9u5.js";
import { b as tt, a as et } from "./CZv_AhHu.js";
import { p as P } from "./5u5qd9TD.js";
import { k as at, l as L } from "./B21bTIl7.js";
import { u as ot } from "./Bb8ybDgy.js";
import { A as it } from "./CDyK5fuy.js";
import { B as rt } from "./BEQMYyDu.js";
import { T as nt } from "./C8iV2OFO.js";
import { u as st } from "./DwvS5LQk.js";
var vt = u('<div class="ip font-mono svelte-1dc3uko"> </div>'), lt = u("<div><!></div>"), mt = u(" <!>", 1), ft = u(" <!>", 1), dt = u(" <!>", 1), ut = u('<div role="contentinfo"><div class="ts svelte-1dc3uko"> </div> <!> <!></div>');
function Rt(M, t) {
  S(t, true);
  const C = (e, a = Y) => {
    var i = Q(), _ = w(i);
    {
      var n = (s) => {
        var v = lt(), g = k(v);
        rt(g, { invisible: true, onclick: () => O(a()), children: (o, m) => {
          nt(o, { get text() {
            return N(I);
          }, yOffset: 20, children: (l, p) => {
            var y = vt(), K = k(y, true);
            b(y), f(() => d(K, a())), r(l, y);
          }, $$slots: { default: true } });
        }, $$slots: { default: true } }), b(v), r(s, v);
      };
      x(_, (s) => {
        a() && s(n);
      });
    }
    r(e, i);
  };
  let V = st(), A = ot(), R = H(false), I = H(P(V.common.copyToClip));
  W(() => {
    (/* @__PURE__ */ new Date()).getTime() - 1e3 * 30 < t.event.timestamp && (setTimeout(() => {
      T(R, true);
    }, 100), setTimeout(() => {
      T(R, false);
    }, 2500));
  });
  function O(e) {
    navigator.clipboard.writeText(e), T(I, P(A.common.copiedToClip)), setTimeout(() => {
      T(I, P(V.common.copyToClip));
    }, 3e3);
  }
  var c = ut();
  let D, E;
  var B = k(c), U = k(B, true);
  b(B);
  var j = h(B, 2);
  {
    var q = (e) => {
      var a = F();
      f(() => d(a, t.event.typ)), r(e, a);
    };
    x(j, (e) => {
      t.event.typ !== "RauthyHealthy" && t.event.typ !== "RauthyUnhealthy" && t.event.typ !== "NewRauthyVersion" && t.event.typ !== "Test" && e(q);
    });
  }
  var z = h(j, 2);
  {
    var G = (e) => {
      var a = mt(), i = w(a), _ = h(i);
      C(_, () => t.event.ip), f(() => d(i, `${`: ${t.event.data}` ?? ""} `)), r(e, a);
    }, J = (e, a) => {
      {
        var i = (n) => {
          const s = $(() => t.event.text || "");
          it(n, { get href() {
            return N(s);
          }, target: "_blank", children: (v, g) => {
            Z();
            var o = F("NewRauthyVersion");
            r(v, o);
          }, $$slots: { default: true } });
        }, _ = (n, s) => {
          {
            var v = (o) => {
              var m = ft(), l = w(m), p = h(l);
              C(p, () => t.event.ip), f((y) => d(l, `${A.common.until ?? ""}
        ${y ?? ""} `), [() => t.event.data && L(t.event.data)]), r(o, m);
            }, g = (o) => {
              var m = dt(), l = w(m), p = h(l);
              C(p, () => t.event.ip), f(() => d(l, `${t.event.text ?? ""} `)), r(o, m);
            };
            x(n, (o) => {
              t.event.typ === "IpBlacklisted" ? o(v) : o(g, false);
            }, s);
          }
        };
        x(e, (n) => {
          t.event.typ === "NewRauthyVersion" ? n(i) : n(_, false);
        }, a);
      }
    };
    x(z, (e) => {
      t.event.typ === "InvalidLogins" ? e(G) : e(J, false);
    });
  }
  b(c), f((e, a, i) => {
    D = tt(c, 1, "event svelte-1dc3uko", null, D, e), E = et(c, "", E, { "border-left": a }), d(U, i);
  }, [() => ({ highlight: N(R), inline: t.inline }), () => `2px solid ${at(t.event.level)}`, () => L(t.event.timestamp / 1e3)]), r(M, c), X();
}
export {
  Rt as E
};
