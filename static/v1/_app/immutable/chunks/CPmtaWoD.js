import { n as V, a as h, t as _ } from "./DKC5GJ29.js";
import { t as g, p as X, aa as Y, a9 as Z, k as S, s as x, a as $, j as t, c as l, l as p, r as n } from "./BveBAmlr.js";
import { d as tt, e as K } from "./CYCba2oX.js";
import { i as M } from "./D-uYoVwt.js";
import { s as P } from "./Dv-Q3FDX.js";
import { s as d, a as R, b as et } from "./Dql74IOz.js";
import { p as q } from "./VbPNpVtZ.js";
import { b as st } from "./DpR-4P2Y.js";
import { p as c } from "./Db0ChEdV.js";
import { g as at } from "./B21bTIl7.js";
import { B as ot } from "./DlLEcmNg.js";
var it = V('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M7.5 21 3 16.5m0 0L7.5 12M3 16.5h13.5m0-13.5L21 7.5m0 0L16.5 12M21 7.5H7.5"></path></svg>');
function rt(b, e) {
  let u = c(e, "opacity", 3, 0.9), y = c(e, "width", 3, "1.5rem"), T = c(e, "color", 3, "currentColor");
  var v = it();
  d(v, "stroke-width", 2), g(() => {
    d(v, "stroke", T()), d(v, "width", y()), d(v, "opacity", u());
  }), h(b, v);
}
function lt(b, e, u) {
  t(e) && p(u, true);
}
var nt = _('<div class="btn svelte-1c7ks5t"><!></div>'), dt = _('<div class="btnCollapsed svelte-1c7ks5t"><!></div>'), vt = _('<div class="absolute svelte-1c7ks5t"><div><!></div></div>'), ct = _("<ul><!></ul>"), ut = _('<div class="navSub svelte-1c7ks5t"><!> <nav class="svelte-1c7ks5t"><!> <div class="svelte-1c7ks5t"><!> <!></div></nav></div>');
function Bt(b, e) {
  X(e, true);
  let u = c(e, "width", 3, "min(25rem, 100dvw)"), y = c(e, "paddingTop", 3, "4.5rem"), T = c(e, "collapseButtonThreshold", 3, 800), v = c(e, "thresholdNavSub", 3, 500);
  const j = at();
  let m = S(void 0), k = Z(() => !!(t(m) && t(m) < v())), o = S(false), B = S(false);
  Y(() => {
    t(m) && p(o, q(t(k)));
  });
  function D() {
    p(o, !t(o));
  }
  function E() {
    t(m) && t(m) < T() && p(B, true);
  }
  function F() {
    p(B, false);
  }
  var C = ut();
  {
    const w = (a) => {
      ot(a, { ariaControls: j, invisible: true, onclick: D, children: (s, r) => {
        var f = nt(), N = l(f);
        rt(N, { width: "1.4rem" }), n(f), h(s, f);
      }, $$slots: { default: true } });
    };
    var A = l(C);
    {
      var G = (a) => {
        var s = dt(), r = l(s);
        w(r), n(s), h(a, s);
      };
      M(A, (a) => {
        (t(o) || t(k)) && a(G);
      });
    }
    var i = x(A, 2);
    d(i, "id", j), i.__click = [lt, k, o];
    let W;
    var H = l(i);
    {
      var J = (a) => {
        var s = vt(), r = l(s);
        let f;
        var N = l(r);
        w(N), n(r), n(s), g((U) => f = et(r, 1, "iconHover svelte-1c7ks5t", null, f, U), [() => ({ hoverLeft: t(o) })]), h(a, s);
      };
      M(H, (a) => {
        t(B) && !(t(o) || t(k)) && a(J);
      });
    }
    var L = x(H, 2);
    let z;
    var I = l(L);
    P(I, () => e.children);
    var O = x(I, 2);
    {
      var Q = (a) => {
        var s = ct(), r = l(s);
        P(r, () => e.buttonTiles), n(s), g(() => d(s, "aria-controls", e.buttonTilesAriaControls)), h(a, s);
      };
      M(O, (a) => {
        e.buttonTiles && a(Q);
      });
    }
    n(L), n(i), n(C), g(() => {
      d(i, "aria-hidden", t(o)), d(i, "data-collapsed", t(o)), W = R(i, "", W, { width: t(o) ? 0 : u(), "min-width": t(o) ? 0 : u() }), z = R(L, "", z, { "padding-top": y() });
    }), K("mouseenter", i, E), K("mouseleave", i, F);
  }
  st("innerWidth", (w) => p(m, q(w))), h(b, C), $();
}
tt(["click"]);
export {
  Bt as N
};
