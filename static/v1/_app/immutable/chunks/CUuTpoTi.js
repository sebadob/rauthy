import { n as O, a as h, t as b } from "./BH6NCLk-.js";
import { t as k, p as Q, aa as U, a9 as V, k as N, s as S, a as X, j as t, c as l, l as p, r as v } from "./CvlvO1XB.js";
import { d as Y, e as z } from "./CTI4QPiR.js";
import { i as x } from "./BUO_AUgz.js";
import { s as K } from "./-T201g_q.js";
import { s as d, a as Z } from "./BMbqVy6X.js";
import { a as M, B as $ } from "./DMkkW5Nn.js";
import { p as P } from "./Wh68IIk2.js";
import { b as tt } from "./7lxiEjMQ.js";
import { p as c } from "./C6SR4G2t.js";
import { g as et } from "./B8wC3kJv.js";
var at = O('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M7.5 21 3 16.5m0 0L7.5 12M3 16.5h13.5m0-13.5L21 7.5m0 0L16.5 12M21 7.5H7.5"></path></svg>');
function ot(_, e) {
  let u = c(e, "opacity", 3, 0.9), j = c(e, "width", 3, "1.5rem"), T = c(e, "color", 3, "currentColor");
  var n = at();
  d(n, "stroke-width", 2), k(() => {
    d(n, "stroke", T()), d(n, "width", j()), d(n, "opacity", u());
  }), h(_, n);
}
function it(_, e, u) {
  t(e) && p(u, true);
}
var st = b('<div class="btn svelte-vbaajt"><!></div>'), rt = b('<div class="btnCollapsed svelte-vbaajt"><!></div>'), lt = b('<div class="absolute svelte-vbaajt"><div><!></div></div>'), vt = b("<ul><!></ul>"), dt = b('<div class="navSub svelte-vbaajt"><!> <nav class="svelte-vbaajt"><!> <div class="svelte-vbaajt"><!> <!></div></nav></div>');
function kt(_, e) {
  Q(e, true);
  let u = c(e, "width", 3, "min(25rem, 100dvw)"), j = c(e, "paddingTop", 3, "4.5rem"), T = c(e, "collapseButtonThreshold", 3, 800), n = c(e, "thresholdNavSub", 3, 500);
  const A = et();
  let m = N(void 0), w = V(() => !!(t(m) && t(m) < n())), i = N(false), y = N(false);
  U(() => {
    t(m) && p(i, P(t(w)));
  });
  function R() {
    p(i, !t(i));
  }
  function q() {
    t(m) && t(m) < T() && p(y, true);
  }
  function D() {
    p(y, false);
  }
  var B = dt();
  {
    const g = (o) => {
      $(o, { ariaControls: A, invisible: true, onclick: R, children: (a, r) => {
        var f = st(), L = l(f);
        ot(L, { width: "1.4rem" }), v(f), h(a, f);
      }, $$slots: { default: true } });
    };
    var H = l(B);
    {
      var E = (o) => {
        var a = rt(), r = l(a);
        g(r), v(a), h(o, a);
      };
      x(H, (o) => {
        (t(i) || t(w)) && o(E);
      });
    }
    var s = S(H, 2);
    d(s, "id", A), s.__click = [it, w, i];
    var I = l(s);
    {
      var F = (o) => {
        var a = lt(), r = l(a);
        let f;
        var L = l(r);
        g(L), v(r), v(a), k(() => f = Z(r, 1, "iconHover svelte-vbaajt", null, f, { hoverLeft: t(i) })), h(o, a);
      };
      x(I, (o) => {
        t(y) && !(t(i) || t(w)) && o(F);
      });
    }
    var C = S(I, 2), W = l(C);
    K(W, () => e.children);
    var G = S(W, 2);
    {
      var J = (o) => {
        var a = vt(), r = l(a);
        K(r, () => e.buttonTiles), v(a), k(() => d(a, "aria-controls", e.buttonTilesAriaControls)), h(o, a);
      };
      x(G, (o) => {
        e.buttonTiles && o(J);
      });
    }
    v(C), v(s), v(B), k(() => {
      d(s, "aria-hidden", t(i)), d(s, "data-collapsed", t(i)), M(s, "width", t(i) ? 0 : u()), M(s, "min-width", t(i) ? 0 : u()), M(C, "padding-top", j());
    }), z("mouseenter", s, q), z("mouseleave", s, D);
  }
  tt("innerWidth", (g) => p(m, P(g))), h(_, B), X();
}
Y(["click"]);
export {
  kt as N
};
