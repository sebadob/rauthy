import { n as V, a as p, t as _ } from "./YHhP1LbZ.js";
import { t as k, p as X, aa as Y, a9 as Z, k as N, s as S, a as $, j as t, c as l, l as h, r as v } from "./Ck6jKiur.js";
import { d as tt, e as z } from "./tDAaDMC_.js";
import { i as x } from "./7JDmqCCW.js";
import { s as K } from "./9ksWc3Vn.js";
import { s as n, a as P, b as et } from "./BTaFr7HN.js";
import { p as R } from "./ho0YXExL.js";
import { b as at } from "./DOEB3Ovi.js";
import { p as c } from "./DZP54pO_.js";
import { g as ot } from "./B21bTIl7.js";
import { B as st } from "./Bd2Rvcxs.js";
var it = V('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M7.5 21 3 16.5m0 0L7.5 12M3 16.5h13.5m0-13.5L21 7.5m0 0L16.5 12M21 7.5H7.5"></path></svg>');
function rt(b, e) {
  let f = c(e, "opacity", 3, 0.9), y = c(e, "width", 3, "1.5rem"), q = c(e, "color", 3, "currentColor");
  var d = it();
  n(d, "stroke-width", 2), k(() => {
    n(d, "stroke", q()), n(d, "width", y()), n(d, "opacity", f());
  }), p(b, d);
}
function lt(b, e, f) {
  t(e) && h(f, true);
}
var vt = _('<div class="btn svelte-fpq55v"><!></div>'), nt = _('<div class="btnCollapsed svelte-fpq55v"><!></div>'), dt = _('<div class="absolute svelte-fpq55v"><div><!></div></div>'), ct = _("<ul><!></ul>"), ft = _('<div class="navSub svelte-fpq55v"><!> <nav class="svelte-fpq55v"><!> <div class="svelte-fpq55v"><!> <!></div></nav></div>');
function Tt(b, e) {
  X(e, true);
  let f = c(e, "width", 3, "min(25rem, 100dvw)"), y = c(e, "paddingTop", 3, "4.5rem"), q = c(e, "collapseButtonThreshold", 3, 800), d = c(e, "thresholdNavSub", 3, 500);
  const M = ot();
  let u = N(void 0), w = Z(() => !!(t(u) && t(u) < d())), s = N(false), T = N(false);
  Y(() => {
    t(u) && h(s, R(t(w)));
  });
  function D() {
    h(s, !t(s));
  }
  function E() {
    t(u) && t(u) < q() && h(T, true);
  }
  function F() {
    h(T, false);
  }
  var B = ft();
  {
    const g = (o) => {
      st(o, { ariaControls: M, invisible: true, onclick: D, children: (a, r) => {
        var m = vt(), L = l(m);
        rt(L, { width: "1.4rem" }), v(m), p(a, m);
      }, $$slots: { default: true } });
    };
    var j = l(B);
    {
      var G = (o) => {
        var a = nt(), r = l(a);
        g(r), v(a), p(o, a);
      };
      x(j, (o) => {
        (t(s) || t(w)) && o(G);
      });
    }
    var i = S(j, 2);
    n(i, "id", M), i.__click = [lt, w, s];
    let I;
    var A = l(i);
    {
      var J = (o) => {
        var a = dt(), r = l(a);
        let m;
        var L = l(r);
        g(L), v(r), v(a), k((U) => m = et(r, 1, "iconHover svelte-fpq55v", null, m, U), [() => ({ hoverLeft: t(s) })]), p(o, a);
      };
      x(A, (o) => {
        t(T) && !(t(s) || t(w)) && o(J);
      });
    }
    var C = S(A, 2);
    let W;
    var H = l(C);
    K(H, () => e.children);
    var O = S(H, 2);
    {
      var Q = (o) => {
        var a = ct(), r = l(a);
        K(r, () => e.buttonTiles), v(a), k(() => n(a, "aria-controls", e.buttonTilesAriaControls)), p(o, a);
      };
      x(O, (o) => {
        e.buttonTiles && o(Q);
      });
    }
    v(C), v(i), v(B), k(() => {
      n(i, "aria-hidden", t(s)), n(i, "data-collapsed", t(s)), I = P(i, "", I, { width: t(s) ? 0 : f(), "min-width": t(s) ? 0 : f() }), W = P(C, "", W, { "padding-top": y() });
    }), z("mouseenter", i, E), z("mouseleave", i, F);
  }
  at("innerWidth", (g) => h(u, R(g))), p(b, B), $();
}
tt(["click"]);
export {
  Tt as N
};
