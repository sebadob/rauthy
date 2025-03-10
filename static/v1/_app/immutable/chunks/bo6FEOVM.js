import { t as u, a as o, e as H, d as P } from "./BH6NCLk-.js";
import { p as S, k as L, a5 as W, l as k, c as w, r as b, s as g, t as m, j as V, a as X, f, a8 as Y, a7 as Z, a9 as $ } from "./CvlvO1XB.js";
import { s as d } from "./CTI4QPiR.js";
import { i as x } from "./BUO_AUgz.js";
import { a as tt } from "./BMbqVy6X.js";
import { a as et, B as at } from "./DMkkW5Nn.js";
import { p as A } from "./Wh68IIk2.js";
import { k as ot, l as M } from "./B21bTIl7.js";
import { u as rt } from "./D8mHI_K9.js";
import { A as it } from "./BqWmWZH3.js";
import { T as nt } from "./BmpEzKyJ.js";
import { u as vt } from "./CUqQZdNU.js";
var st = u('<div class="ip font-mono svelte-1dc3uko"> </div>'), lt = u("<div><!></div>"), mt = u(" <!>", 1), ft = u(" <!>", 1), dt = u(" <!>", 1), ut = u('<div role="contentinfo"><div class="ts svelte-1dc3uko"> </div> <!> <!></div>');
function Rt(O, t) {
  S(t, true);
  const C = (e, a = Y) => {
    var s = P(), c = f(s);
    {
      var N = (r) => {
        var n = lt(), h = w(n);
        at(h, { invisible: true, onclick: () => U(a()), children: (p, T) => {
          nt(p, { get text() {
            return V(I);
          }, yOffset: 20, children: (v, l) => {
            var i = st(), y = w(i, true);
            b(i), m(() => d(y, a())), o(v, i);
          }, $$slots: { default: true } });
        }, $$slots: { default: true } }), b(n), o(r, n);
      };
      x(c, (r) => {
        a() && r(N);
      });
    }
    o(e, s);
  };
  let D = vt(), E = rt(), R = L(false), I = L(A(D.common.copyToClip));
  W(() => {
    (/* @__PURE__ */ new Date()).getTime() - 1e3 * 30 < t.event.timestamp && (setTimeout(() => {
      k(R, true);
    }, 100), setTimeout(() => {
      k(R, false);
    }, 2500));
  });
  function U(e) {
    navigator.clipboard.writeText(e), k(I, A(E.common.copiedToClip)), setTimeout(() => {
      k(I, A(D.common.copyToClip));
    }, 3e3);
  }
  var _ = ut();
  let j;
  var B = w(_), q = w(B, true);
  b(B);
  var F = g(B, 2);
  {
    var z = (e) => {
      var a = H();
      m(() => d(a, t.event.typ)), o(e, a);
    };
    x(F, (e) => {
      t.event.typ !== "RauthyHealthy" && t.event.typ !== "RauthyUnhealthy" && t.event.typ !== "NewRauthyVersion" && t.event.typ !== "Test" && e(z);
    });
  }
  var G = g(F, 2);
  {
    var J = (e) => {
      var a = mt(), s = f(a), c = g(s);
      C(c, () => t.event.ip), m(() => d(s, `${`: ${t.event.data}` ?? ""} `)), o(e, a);
    }, K = (e) => {
      var a = P(), s = f(a);
      {
        var c = (r) => {
          const n = $(() => t.event.text || "");
          it(r, { get href() {
            return V(n);
          }, target: "_blank", children: (h, p) => {
            Z();
            var T = H("NewRauthyVersion");
            o(h, T);
          }, $$slots: { default: true } });
        }, N = (r) => {
          var n = P(), h = f(n);
          {
            var p = (v) => {
              var l = ft(), i = f(l), y = g(i);
              C(y, () => t.event.ip), m((Q) => d(i, `${E.common.until ?? ""}
        ${Q ?? ""} `), [() => t.event.data && M(t.event.data)]), o(v, l);
            }, T = (v) => {
              var l = dt(), i = f(l), y = g(i);
              C(y, () => t.event.ip), m(() => d(i, `${t.event.text ?? ""} `)), o(v, l);
            };
            x(h, (v) => {
              t.event.typ === "IpBlacklisted" ? v(p) : v(T, false);
            }, true);
          }
          o(r, n);
        };
        x(s, (r) => {
          t.event.typ === "NewRauthyVersion" ? r(c) : r(N, false);
        }, true);
      }
      o(e, a);
    };
    x(G, (e) => {
      t.event.typ === "InvalidLogins" ? e(J) : e(K, false);
    });
  }
  b(_), m((e, a) => {
    j = tt(_, 1, "event svelte-1dc3uko", null, j, { highlight: V(R), inline: t.inline }), et(_, "border-left", e), d(q, a);
  }, [() => `2px solid ${ot(t.event.level)}`, () => M(t.event.timestamp / 1e3)]), o(O, _), X();
}
export {
  Rt as E
};
