import { n as O, a as h, t as y } from "./BH6NCLk-.js";
import { t as g, p as Q, aa as U, a9 as V, k as S, s as x, a as X, j as t, c as l, l as p, r as d } from "./CvlvO1XB.js";
import { d as Y, e as z } from "./CTI4QPiR.js";
import { i as M } from "./BUO_AUgz.js";
import { s as K } from "./-T201g_q.js";
import { s as n, a as Z } from "./BMbqVy6X.js";
import { a as j, B as $ } from "./DMkkW5Nn.js";
import { p as P } from "./Wh68IIk2.js";
import { b as tt } from "./7lxiEjMQ.js";
import { p as c } from "./C6SR4G2t.js";
import { g as et } from "./CGXT-546.js";
var it = O('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M7.5 21 3 16.5m0 0L7.5 12M3 16.5h13.5m0-13.5L21 7.5m0 0L16.5 12M21 7.5H7.5"></path></svg>');
function at(_, e) {
  let u = c(e, "opacity", 3, 0.9), k = c(e, "width", 3, "1.5rem"), T = c(e, "color", 3, "currentColor");
  var v = it();
  n(v, "stroke-width", 2), g(() => {
    n(v, "stroke", T()), n(v, "width", k()), n(v, "opacity", u());
  }), h(_, v);
}
function ot(_, e, u) {
  t(e) && p(u, true);
}
var st = y('<div class="btn svelte-1c0yi3y"><!></div>'), rt = y('<div class="btnCollapsed svelte-1c0yi3y"><!></div>'), lt = y('<div class="absolute svelte-1c0yi3y"><div><!></div></div>'), dt = y("<ul><!></ul>"), nt = y('<div class="navSub svelte-1c0yi3y"><!> <nav class="svelte-1c0yi3y"><!> <div class="svelte-1c0yi3y"><!> <!></div></nav></div>');
function gt(_, e) {
  Q(e, true);
  let u = c(e, "width", 3, "min(25rem, 100dvw)"), k = c(e, "paddingTop", 3, "4.5rem"), T = c(e, "collapseButtonThreshold", 3, 800), v = c(e, "thresholdNavSub", 3, 500);
  const A = et();
  let m = S(void 0), b = V(() => !!(t(m) && t(m) < v())), o = S(false), B = S(false);
  U(() => {
    t(m) && p(o, P(t(b)));
  });
  function R() {
    p(o, !t(o));
  }
  function q() {
    t(m) && t(m) < T() && p(B, true);
  }
  function D() {
    p(B, false);
  }
  var C = nt();
  {
    const w = (a) => {
      $(a, { ariaControls: A, invisible: true, onclick: R, children: (i, r) => {
        var f = st(), N = l(f);
        at(N, { width: "1.4rem" }), d(f), h(i, f);
      }, $$slots: { default: true } });
    };
    var H = l(C);
    {
      var E = (a) => {
        var i = rt(), r = l(i);
        w(r), d(i), h(a, i);
      };
      M(H, (a) => {
        (t(o) || t(b)) && a(E);
      });
    }
    var s = x(H, 2);
    n(s, "id", A), s.__click = [ot, b, o];
    var I = l(s);
    {
      var F = (a) => {
        var i = lt(), r = l(i);
        let f;
        var N = l(r);
        w(N), d(r), d(i), g(() => f = Z(r, 1, "iconHover svelte-1c0yi3y", null, f, { hoverLeft: t(o) })), h(a, i);
      };
      M(I, (a) => {
        t(B) && !(t(o) || t(b)) && a(F);
      });
    }
    var L = x(I, 2), W = l(L);
    K(W, () => e.children);
    var G = x(W, 2);
    {
      var J = (a) => {
        var i = dt(), r = l(i);
        K(r, () => e.buttonTiles), d(i), g(() => n(i, "aria-controls", e.buttonTilesAriaControls)), h(a, i);
      };
      M(G, (a) => {
        e.buttonTiles && a(J);
      });
    }
    d(L), d(s), d(C), g(() => {
      n(s, "aria-hidden", t(o)), n(s, "data-collapsed", t(o)), j(s, "width", t(o) ? 0 : u()), j(s, "min-width", t(o) ? 0 : u()), j(L, "padding-top", k());
    }), z("mouseenter", s, q), z("mouseleave", s, D);
  }
  tt("innerWidth", (w) => p(m, P(w))), h(_, C), X();
}
Y(["click"]);
export {
  gt as N
};
