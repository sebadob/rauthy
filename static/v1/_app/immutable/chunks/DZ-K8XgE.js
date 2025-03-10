import { n as O, a as h, t as _ } from "./BH6NCLk-.js";
import { t as g, p as Q, aa as U, a9 as V, k as S, s as x, a as X, j as t, c as l, l as p, r as d } from "./CvlvO1XB.js";
import { d as Y, e as z } from "./CTI4QPiR.js";
import { i as M } from "./BUO_AUgz.js";
import { s as K } from "./-T201g_q.js";
import { s as n, a as Z } from "./BMbqVy6X.js";
import { a as j, B as $ } from "./DMkkW5Nn.js";
import { p as P } from "./Wh68IIk2.js";
import { b as tt } from "./7lxiEjMQ.js";
import { p as c } from "./C6SR4G2t.js";
import { g as et } from "./B21bTIl7.js";
var st = O('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M7.5 21 3 16.5m0 0L7.5 12M3 16.5h13.5m0-13.5L21 7.5m0 0L16.5 12M21 7.5H7.5"></path></svg>');
function at(b, e) {
  let u = c(e, "opacity", 3, 0.9), T = c(e, "width", 3, "1.5rem"), y = c(e, "color", 3, "currentColor");
  var v = st();
  n(v, "stroke-width", 2), g(() => {
    n(v, "stroke", y()), n(v, "width", T()), n(v, "opacity", u());
  }), h(b, v);
}
function ot(b, e, u) {
  t(e) && p(u, true);
}
var it = _('<div class="btn svelte-1c7ks5t"><!></div>'), rt = _('<div class="btnCollapsed svelte-1c7ks5t"><!></div>'), lt = _('<div class="absolute svelte-1c7ks5t"><div><!></div></div>'), dt = _("<ul><!></ul>"), nt = _('<div class="navSub svelte-1c7ks5t"><!> <nav class="svelte-1c7ks5t"><!> <div class="svelte-1c7ks5t"><!> <!></div></nav></div>');
function gt(b, e) {
  Q(e, true);
  let u = c(e, "width", 3, "min(25rem, 100dvw)"), T = c(e, "paddingTop", 3, "4.5rem"), y = c(e, "collapseButtonThreshold", 3, 800), v = c(e, "thresholdNavSub", 3, 500);
  const A = et();
  let m = S(void 0), k = V(() => !!(t(m) && t(m) < v())), o = S(false), B = S(false);
  U(() => {
    t(m) && p(o, P(t(k)));
  });
  function R() {
    p(o, !t(o));
  }
  function q() {
    t(m) && t(m) < y() && p(B, true);
  }
  function D() {
    p(B, false);
  }
  var C = nt();
  {
    const w = (a) => {
      $(a, { ariaControls: A, invisible: true, onclick: R, children: (s, r) => {
        var f = it(), N = l(f);
        at(N, { width: "1.4rem" }), d(f), h(s, f);
      }, $$slots: { default: true } });
    };
    var H = l(C);
    {
      var E = (a) => {
        var s = rt(), r = l(s);
        w(r), d(s), h(a, s);
      };
      M(H, (a) => {
        (t(o) || t(k)) && a(E);
      });
    }
    var i = x(H, 2);
    n(i, "id", A), i.__click = [ot, k, o];
    var I = l(i);
    {
      var F = (a) => {
        var s = lt(), r = l(s);
        let f;
        var N = l(r);
        w(N), d(r), d(s), g(() => f = Z(r, 1, "iconHover svelte-1c7ks5t", null, f, { hoverLeft: t(o) })), h(a, s);
      };
      M(I, (a) => {
        t(B) && !(t(o) || t(k)) && a(F);
      });
    }
    var L = x(I, 2), W = l(L);
    K(W, () => e.children);
    var G = x(W, 2);
    {
      var J = (a) => {
        var s = dt(), r = l(s);
        K(r, () => e.buttonTiles), d(s), g(() => n(s, "aria-controls", e.buttonTilesAriaControls)), h(a, s);
      };
      M(G, (a) => {
        e.buttonTiles && a(J);
      });
    }
    d(L), d(i), d(C), g(() => {
      n(i, "aria-hidden", t(o)), n(i, "data-collapsed", t(o)), j(i, "width", t(o) ? 0 : u()), j(i, "min-width", t(o) ? 0 : u()), j(L, "padding-top", T());
    }), z("mouseenter", i, q), z("mouseleave", i, D);
  }
  tt("innerWidth", (w) => p(m, P(w))), h(b, C), X();
}
Y(["click"]);
export {
  gt as N
};
