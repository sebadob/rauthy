import { n as V, a as p, t as _ } from "./DLBGyKVC.js";
import { t as k, p as X, aa as Y, a9 as Z, k as N, s as S, a as $, j as t, c as l, l as h, r as v } from "./CmQi0fbH.js";
import { d as tt, e as z } from "./BjaYyaa_.js";
import { i as x } from "./C6bK2EJJ.js";
import { s as K } from "./71gCR-8F.js";
import { s as d, a as P, b as et } from "./DOlUUCkJ.js";
import { p as R } from "./B_ggA-0N.js";
import { b as ot } from "./DzJ8OZ_u.js";
import { p as c } from "./DNJm3-SG.js";
import { g as at } from "./B21bTIl7.js";
import { B as st } from "./DPLO-ozG.js";
var it = V('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M7.5 21 3 16.5m0 0L7.5 12M3 16.5h13.5m0-13.5L21 7.5m0 0L16.5 12M21 7.5H7.5"></path></svg>');
function rt(b, e) {
  let f = c(e, "opacity", 3, 0.9), y = c(e, "width", 3, "1.5rem"), q = c(e, "color", 3, "currentColor");
  var n = it();
  d(n, "stroke-width", 2), k(() => {
    d(n, "stroke", q()), d(n, "width", y()), d(n, "opacity", f());
  }), p(b, n);
}
function lt(b, e, f) {
  t(e) && h(f, true);
}
var vt = _('<div class="btn svelte-fpq55v"><!></div>'), dt = _('<div class="btnCollapsed svelte-fpq55v"><!></div>'), nt = _('<div class="absolute svelte-fpq55v"><div><!></div></div>'), ct = _("<ul><!></ul>"), ft = _('<div class="navSub svelte-fpq55v"><!> <nav class="svelte-fpq55v"><!> <div class="svelte-fpq55v"><!> <!></div></nav></div>');
function Tt(b, e) {
  X(e, true);
  let f = c(e, "width", 3, "min(25rem, 100dvw)"), y = c(e, "paddingTop", 3, "4.5rem"), q = c(e, "collapseButtonThreshold", 3, 800), n = c(e, "thresholdNavSub", 3, 500);
  const M = at();
  let m = N(void 0), w = Z(() => !!(t(m) && t(m) < n())), s = N(false), T = N(false);
  Y(() => {
    t(m) && h(s, R(t(w)));
  });
  function D() {
    h(s, !t(s));
  }
  function E() {
    t(m) && t(m) < q() && h(T, true);
  }
  function F() {
    h(T, false);
  }
  var B = ft();
  {
    const g = (a) => {
      st(a, { ariaControls: M, invisible: true, onclick: D, children: (o, r) => {
        var u = vt(), L = l(u);
        rt(L, { width: "1.4rem" }), v(u), p(o, u);
      }, $$slots: { default: true } });
    };
    var j = l(B);
    {
      var G = (a) => {
        var o = dt(), r = l(o);
        g(r), v(o), p(a, o);
      };
      x(j, (a) => {
        (t(s) || t(w)) && a(G);
      });
    }
    var i = S(j, 2);
    d(i, "id", M), i.__click = [lt, w, s];
    let I;
    var A = l(i);
    {
      var J = (a) => {
        var o = nt(), r = l(o);
        let u;
        var L = l(r);
        g(L), v(r), v(o), k((U) => u = et(r, 1, "iconHover svelte-fpq55v", null, u, U), [() => ({ hoverLeft: t(s) })]), p(a, o);
      };
      x(A, (a) => {
        t(T) && !(t(s) || t(w)) && a(J);
      });
    }
    var C = S(A, 2);
    let W;
    var H = l(C);
    K(H, () => e.children);
    var O = S(H, 2);
    {
      var Q = (a) => {
        var o = ct(), r = l(o);
        K(r, () => e.buttonTiles), v(o), k(() => d(o, "aria-controls", e.buttonTilesAriaControls)), p(a, o);
      };
      x(O, (a) => {
        e.buttonTiles && a(Q);
      });
    }
    v(C), v(i), v(B), k(() => {
      d(i, "aria-hidden", t(s)), d(i, "data-collapsed", t(s)), I = P(i, "", I, { width: t(s) ? 0 : f(), "min-width": t(s) ? 0 : f() }), W = P(C, "", W, { "padding-top": y() });
    }), z("mouseenter", i, E), z("mouseleave", i, F);
  }
  ot("innerWidth", (g) => h(m, R(g))), p(b, B), $();
}
tt(["click"]);
export {
  Tt as N
};
