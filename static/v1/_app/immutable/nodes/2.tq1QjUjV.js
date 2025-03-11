import { d as fe, a as n, t as C, n as O, e as H } from "../chunks/CWf9OOFK.js";
import { p as ue, a6 as je, l as b, k as W, f as re, j as t, a as he, c as u, r as d, t as f, aa as ge, s as g, ab as ie, a9 as D, a8 as q, ak as Te, a7 as Ee } from "../chunks/nlANaGLT.js";
import { s as z, e as Me, h as He } from "../chunks/BmMHVVX3.js";
import { i as ee } from "../chunks/DOjUa9u5.js";
import { h as Ie } from "../chunks/CSmZOR6t.js";
import { s as de } from "../chunks/BgMuVWsk.js";
import { p as L } from "../chunks/5u5qd9TD.js";
import { b as _e } from "../chunks/D5tppINK.js";
import { M as qe } from "../chunks/BDnQQV_p.js";
import { B as ce } from "../chunks/BEQMYyDu.js";
import { s as o, a as ve, b as ze } from "../chunks/CZv_AhHu.js";
import { T as Ce } from "../chunks/Afb0E4F7.js";
import { L as Pe } from "../chunks/DbogliCN.js";
import { p as Be } from "../chunks/B4Xt-L9L.js";
import { f as Le, b as We } from "../chunks/BO1A6s0c.js";
import { R as Ve } from "../chunks/CdxD-BT8.js";
import { A as $e } from "../chunks/DhLqkb2H.js";
import { p as v } from "../chunks/uWmgYd3Z.js";
import { I as Ne } from "../chunks/DJ6IvX5N.js";
import { u as we, i as Oe } from "../chunks/Bb8ybDgy.js";
import { I as Re } from "../chunks/Dnd9tLWr.js";
import { S as Ze, k as pe, i as Ue, B as De } from "../chunks/B21bTIl7.js";
import { T as Ge } from "../chunks/C8iV2OFO.js";
import { u as Ke } from "../chunks/Be0yYLp7.js";
import { e as Fe } from "../chunks/B6azywu7.js";
import { E as Je } from "../chunks/DYOzC1O-.js";
import { u as Qe } from "../chunks/DwvS5LQk.js";
import { O as Xe } from "../chunks/FmRbdHbC.js";
var Ye = C('<div class="ver upd svelte-4tal6i"><a target="_blank" class="svelte-4tal6i"> </a></div>'), et = C('<div class="ver svelte-4tal6i"> </div>');
function tt(h, r) {
  ue(r, true);
  let a = W(void 0);
  je(() => {
    l();
  });
  async function l() {
    let w = await Le("/auth/v1/version");
    w.body ? b(a, L(w.body)) : console.error(w.error);
  }
  var i = fe(), e = re(i);
  {
    var y = (w) => {
      var S = fe(), P = re(S);
      {
        var R = (k) => {
          var c = Ye(), x = u(c), A = u(x);
          d(x), d(c), f(() => {
            o(x, "href", t(a).latest_url), z(A, `v${t(a) ?? ""} \u26A0\uFE0F`);
          }), n(k, c);
        }, j = (k) => {
          var c = et(), x = u(c);
          d(c), f(() => z(x, `v${t(a).current ?? ""}`)), n(k, c);
        };
        ee(P, (k) => {
          t(a).update_available ? k(R) : k(j, false);
        });
      }
      n(w, S);
    };
    ee(e, (w) => {
      t(a) && w(y);
    });
  }
  n(h, i), he();
}
var rt = C('<div class="compact svelte-7fczgn"><div class="iconCompact svelte-7fczgn"><!></div> <span class="svelte-7fczgn"><!></span></div>'), ot = C('<div class="wide svelte-7fczgn"><div class="iconWide svelte-7fczgn"><!></div> <div><!></div></div>');
function G(h, r) {
  const a = "/auth/v1/admin";
  let l = ge(() => r.compact ? "1.5rem" : "1.2rem"), i = ge(() => `${a}${r.route}${r.params}`);
  $e(h, { get href() {
    return t(i);
  }, hideUnderline: true, get highlightIncludes() {
    return r.highlightIncludes;
  }, children: (e, y) => {
    var w = fe(), S = re(w);
    {
      var P = (j) => {
        var k = rt(), c = u(k), x = u(c);
        de(x, () => r.icon, () => t(l)), d(c);
        var A = g(c, 2), M = u(A);
        de(M, () => r.children), d(A), d(k), n(j, k);
      }, R = (j) => {
        var k = ot(), c = u(k);
        let x;
        var A = u(c);
        de(A, () => r.icon, () => t(l)), d(c);
        var M = g(c, 2), I = u(M);
        de(I, () => r.children), d(M), d(k), f(() => x = ve(c, "", x, { width: t(l) })), n(j, k);
      };
      ee(S, (j) => {
        r.compact ? j(P) : j(R, false);
      });
    }
    n(e, w);
  }, $$slots: { default: true } });
}
var at = O('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"></path></svg>');
function it(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = at();
  o(e, "stroke-width", 2), f(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var nt = O(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M19.5 12c0-1.232-.046-2.453-.138-3.662a4.006 4.006 0 0 0-3.7-3.7 48.678 48.678 0 0 0-7.324 0 4.006 4.006
            0 0 0-3.7 3.7c-.017.22-.032.441-.046.662M19.5 12l3-3m-3 3-3-3m-12 3c0 1.232.046 2.453.138 3.662a4.006 4.006
            0 0 0 3.7 3.7 48.656 48.656 0 0 0 7.324 0 4.006 4.006 0 0 0 3.7-3.7c.017-.22.032-.441.046-.662M4.5 12l3
            3m-3-3-3 3"></path></svg>`);
function st(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = nt();
  o(e, "stroke-width", 2), f(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var lt = O(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M18 18.72a9.094 9.094 0 0 0 3.741-.479 3 3 0 0 0-4.682-2.72m.94 3.198.001.031c0
            .225-.012.447-.037.666A11.944 11.944 0 0 1 12 21c-2.17 0-4.207-.576-5.963-1.584A6.062 6.062 0 0 1 6
            18.719m12 0a5.971 5.971 0 0 0-.941-3.197m0 0A5.995 5.995 0 0 0 12 12.75a5.995 5.995 0 0 0-5.058 2.772m0
            0a3 3 0 0 0-4.681 2.72 8.986 8.986 0 0 0 3.74.477m.94-3.197a5.971 5.971 0 0 0-.94 3.197M15 6.75a3 3 0 1
            1-6 0 3 3 0 0 1 6 0Zm6 3a2.25 2.25 0 1 1-4.5 0 2.25 2.25 0 0 1 4.5 0Zm-13.5 0a2.25 2.25 0 1 1-4.5 0 2.25
            2.25 0 0 1 4.5 0Z"></path></svg>`);
function vt(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = lt();
  o(e, "stroke-width", 2), f(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var ct = O('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z"></path></svg>');
function dt(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = ct();
  o(e, "stroke-width", 2), f(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var ut = O(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 21h16.5M4.5 3h15M5.25 3v18m13.5-18v18M9 6.75h1.5m-1.5 3h1.5m-1.5 3h1.5m3-6H15m-1.5 3H15m-1.5
            3H15M9 21v-3.375c0-.621.504-1.125 1.125-1.125h3.75c.621 0 1.125.504 1.125 1.125V21"></path></svg>`);
function ht(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = ut();
  o(e, "stroke-width", 2), f(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var mt = O('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75L11.25 15 15 9.75m-3-7.036A11.959 11.959 0 013.598 6 11.99 11.99 0 003 9.749c0 5.592 3.824 10.29 9 11.623 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.285z"></path></svg>');
function pt(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = mt();
  o(e, "stroke-width", 2), f(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var ft = O('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z"></path></svg>');
function gt(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = ft();
  o(e, "stroke-width", 2), f(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var _t = O(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M15 9h3.75M15 12h3.75M15 15h3.75M4.5 19.5h15a2.25 2.25 0 002.25-2.25V6.75A2.25 2.25 0 0019.5
            4.5h-15a2.25 2.25 0 00-2.25 2.25v10.5A2.25 2.25 0 004.5 19.5zm6-10.125a1.875 1.875 0 11-3.75 0
            1.875 1.875 0 013.75 0zm1.294 6.336a6.721 6.721 0 01-3.17.789 6.721 6.721 0 01-3.168-.789 3.376
            3.376 0 016.338 0z"></path></svg>`);
function wt(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = _t();
  o(e, "stroke-width", 2), f(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var kt = O(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 0 0 5.454-1.31A8.967 8.967 0 0 1 18 9.75V9A6 6 0 0 0 6 9v.75a8.967 8.967
            0 0 1-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 0 1-5.714 0m5.714 0a3 3 0 1 1-5.714
            0"></path></svg>`);
function yt(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = kt();
  o(e, "stroke-width", 2), f(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var xt = O('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M15.75 5.25a3 3 0 013 3m3 0a6 6 0 01-7.029 5.912c-.563-.097-1.159.026-1.563.43L10.5 17.25H8.25v2.25H6v2.25H2.25v-2.818c0-.597.237-1.17.659-1.591l6.499-6.499c.404-.404.527-1 .43-1.563A6 6 0 1121.75 8.25z"></path></svg>');
function bt(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = xt();
  o(e, "stroke-width", 2), f(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var At = O(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M2.25 15a4.5 4.5 0 0 0 4.5 4.5H18a3.75 3.75 0 0 0 1.332-7.257 3 3 0 0 0-3.758-3.848 5.25 5.25 0 0
            0-10.233 2.33A4.502 4.502 0 0 0 2.25 15Z"></path></svg>`);
function Mt(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = At();
  o(e, "stroke-width", 2), f(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var It = O('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M11.42 15.17L17.25 21A2.652 2.652 0 0021 17.25l-5.877-5.877M11.42 15.17l2.496-3.03c.317-.384.74-.626 1.208-.766M11.42 15.17l-4.655 5.653a2.548 2.548 0 11-3.586-3.586l6.837-5.63m5.108-.233c.55-.164 1.163-.188 1.743-.14a4.5 4.5 0 004.486-6.336l-3.276 3.277a3.004 3.004 0 01-2.25-2.25l3.276-3.276a4.5 4.5 0 00-6.336 4.486c.091 1.076-.071 2.264-.904 2.95l-.102.085m-1.745 1.437L5.909 7.5H4.5L2.25 3.75l1.5-1.5L7.5 4.5v1.409l4.26 4.26m-1.745 1.437l1.745-1.437m6.615 8.206L15.75 15.75M4.867 19.125h.008v.008h-.008v-.008z"></path></svg>');
function Ct(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = It();
  o(e, "stroke-width", 2), f(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var Pt = O('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M12 6.042A8.967 8.967 0 006 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 016 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 016-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0018 18a8.967 8.967 0 00-6 2.292m0-14.25v14.25"></path></svg>');
function Bt(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = Pt();
  o(e, "stroke-width", 2), f(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var zt = O(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75 11.25 15 15 9.75M21 12c0 1.268-.63 2.39-1.593 3.068a3.745 3.745 0 0 1-1.043 3.296 3.745
            3.745 0 0 1-3.296 1.043A3.745 3.745 0 0 1 12 21c-1.268 0-2.39-.63-3.068-1.593a3.746 3.746 0 0
            1-3.296-1.043 3.745 3.745 0 0 1-1.043-3.296A3.745 3.745 0 0 1 3 12c0-1.268.63-2.39 1.593-3.068a3.745
            3.745 0 0 1 1.043-3.296 3.746 3.746 0 0 1 3.296-1.043A3.746 3.746 0 0 1 12 3c1.268 0 2.39.63 3.068
            1.593a3.746 3.746 0 0 1 3.296 1.043 3.746 3.746 0 0 1 1.043 3.296A3.745 3.745 0 0 1 21 12Z"></path></svg>`);
function Lt(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = zt();
  o(e, "stroke-width", 2), f(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var St = C('<span class="inner svelte-8anrni"><!></span>'), jt = C('<span class="inner svelte-8anrni"><!></span>'), Tt = C("<div><!></div>"), Et = C('<div class="logo svelte-8anrni"><!></div> <div class="menu svelte-8anrni"><!> <!> <!> <!> <!> <!> <!> <!> <!> <!> <!> <!> <!></div>', 1), Ht = C('<div class="logout svelte-8anrni"><!></div>'), qt = C('<!> <div class="theme svelte-8anrni"><!></div> <!>', 1), Wt = C('<div class="flex gap-05"><!> <!> <!></div> <div class="version svelte-8anrni"><!></div>', 1), Vt = C('<div class="bottom svelte-8anrni"><!></div>'), $t = C('<nav id="mainNav" class="svelte-8anrni"><div class="svelte-8anrni"><div><div class="relative"><div class="absolute flex space-between navmod svelte-8anrni"><div><!></div> <!></div></div> <!></div> <!></div></nav>');
function Nt(h, r) {
  ue(r, true);
  let a = we(), l = W(false), i = W(""), e = W(false), y = W(true), w = W(void 0);
  ie(() => {
    b(i, L(Be.url.search));
  }), ie(() => {
    t(w) && t(w) < 600 && Be.url && b(y, true);
  }), ie(() => {
    b(y, L(t(w) && t(w) < 800 || false));
  }), ie(() => {
    b(e, L(t(w) && t(w) < 1280 || false));
  });
  function S() {
    b(l, true);
  }
  function P() {
    setTimeout(() => {
      b(l, false);
    }, 1e3);
  }
  function R() {
    b(y, !t(y));
  }
  function j() {
    b(e, !t(e));
  }
  var k = $t(), c = u(k), x = u(c), A = u(x), M = u(A), I = u(M), Z = u(I);
  ce(Z, { ariaControls: "mainNav", invisible: true, onclick: R, children: (B, N) => {
    var $ = St(), te = u($);
    it(te, {}), d($), n(B, $);
  }, $$slots: { default: true } }), d(I);
  var K = g(I, 2);
  {
    var V = (B) => {
      var N = Tt(), $ = u(N);
      ce($, { invisible: true, onclick: j, children: (te, se) => {
        var F = jt(), T = u(F);
        st(T, {}), d(F), n(te, F);
      }, $$slots: { default: true } }), d(N), n(B, N);
    };
    ee(K, (B) => {
      t(y) || B(V);
    });
  }
  d(M), d(A);
  var Q = g(A, 2);
  {
    var _ = (B) => {
      var N = Et(), $ = re(N), te = u($);
      const se = ge(() => t(e) ? "3rem" : "7rem");
      Ve(te, { get width() {
        return t(se);
      } }), d($);
      var F = g($, 2);
      let T;
      var E = u(F);
      G(E, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/users", icon: (s, m = D) => {
        gt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        q();
        var p = H();
        f(() => z(p, a.nav.users)), n(s, p);
      } });
      var U = g(E, 2);
      G(U, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/clients", icon: (s, m = D) => {
        ht(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        q();
        var p = H();
        f(() => z(p, a.nav.clients)), n(s, p);
      } });
      var Y = g(U, 2);
      G(Y, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/roles", icon: (s, m = D) => {
        Lt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        q();
        var p = H();
        f(() => z(p, a.nav.roles)), n(s, p);
      } });
      var oe = g(Y, 2);
      G(oe, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/groups", icon: (s, m = D) => {
        vt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        q();
        var p = H();
        f(() => z(p, a.nav.groups)), n(s, p);
      } });
      var ae = g(oe, 2);
      G(ae, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/attributes", icon: (s, m = D) => {
        dt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        q();
        var p = H();
        f(() => z(p, a.nav.attributes)), n(s, p);
      } });
      var le = g(ae, 2);
      G(le, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/scopes", icon: (s, m = D) => {
        wt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        q();
        var p = H();
        f(() => z(p, a.nav.scopes)), n(s, p);
      } });
      var me = g(le, 2);
      G(me, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/sessions", icon: (s, m = D) => {
        pt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        q();
        var p = H();
        f(() => z(p, a.nav.sessions)), n(s, p);
      } });
      var ke = g(me, 2);
      G(ke, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/events", icon: (s, m = D) => {
        yt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        q();
        var p = H();
        f(() => z(p, a.nav.events)), n(s, p);
      } });
      var ye = g(ke, 2);
      G(ye, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/blacklist", icon: (s, m = D) => {
        Ne(s, { get width() {
          return m();
        }, color: "currentColor" });
      }, children: (s, m) => {
        q();
        var p = H();
        f(() => z(p, a.nav.blacklist)), n(s, p);
      } });
      var xe = g(ye, 2);
      G(xe, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/api_keys", icon: (s, m = D) => {
        bt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        q();
        var p = H();
        f(() => z(p, a.nav.apiKeys)), n(s, p);
      } });
      var be = g(xe, 2);
      G(be, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/providers", icon: (s, m = D) => {
        Mt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        q();
        var p = H();
        f(() => z(p, a.nav.providers)), n(s, p);
      } });
      var Ae = g(be, 2);
      G(Ae, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/config/policy", highlightIncludes: "/config/", icon: (s, m = D) => {
        Ct(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        q();
        var p = H();
        f(() => z(p, a.nav.config)), n(s, p);
      } });
      var Se = g(Ae, 2);
      G(Se, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/docs", icon: (s, m = D) => {
        Bt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        q();
        var p = H();
        f(() => z(p, a.nav.docs)), n(s, p);
      } }), d(F), f(() => T = ve(F, "", T, { "margin-top": t(e) ? ".5rem" : "1.5rem" })), n(B, N);
    };
    ee(Q, (B) => {
      t(y) || B(_);
    });
  }
  d(x);
  var X = g(x, 2);
  {
    var ne = (B) => {
      var N = Vt();
      {
        const F = (T) => {
          var E = Ht(), U = u(E);
          ce(U, { invisible: true, onclick: () => Ze(), children: (Y, oe) => {
            Ge(Y, { text: "Logout", children: (ae, le) => {
              Re(ae, {});
            }, $$slots: { default: true } });
          }, $$slots: { default: true } }), d(E), n(T, E);
        };
        var $ = u(N);
        {
          var te = (T) => {
            var E = qt(), U = re(E);
            F(U);
            var Y = g(U, 2), oe = u(Y);
            Ce(oe, {}), d(Y);
            var ae = g(Y, 2);
            Pe(ae, { openTop: true }), n(T, E);
          }, se = (T) => {
            var E = Wt(), U = re(E), Y = u(U);
            Ce(Y, {});
            var oe = g(Y, 2);
            Pe(oe, { openTop: true });
            var ae = g(oe, 2);
            F(ae), d(U);
            var le = g(U, 2), me = u(le);
            tt(me, {}), d(le), n(T, E);
          };
          ee($, (T) => {
            t(e) ? T(te) : T(se, false);
          });
        }
        d(N);
      }
      n(B, N);
    };
    ee(X, (B) => {
      t(y) || B(ne);
    });
  }
  d(c), d(k), f(() => {
    o(k, "aria-hidden", t(y)), o(k, "data-compact", t(e)), o(M, "aria-hidden", !t(y) && !t(l));
  }), _e("innerWidth", (B) => b(w, L(B))), Me("mouseenter", k, S), Me("mouseleave", k, P), n(h, k), he();
}
var Ot = C('<fieldset class="svelte-1vtgp8q"><legend class="font-label svelte-1vtgp8q"> </legend> <ul><li class="svelte-1vtgp8q">Info</li> <li class="svelte-1vtgp8q">Notice</li> <li class="svelte-1vtgp8q">Warning</li> <li class="svelte-1vtgp8q">Critical</li></ul></fieldset>');
function Rt(h, r) {
  ue(r, true);
  let a = Qe();
  var l = Ot(), i = u(l), e = u(i, true);
  d(i);
  var y = g(i, 2);
  let w;
  var S = u(y);
  let P;
  var R = g(S, 2);
  let j;
  var k = g(R, 2);
  let c;
  var x = g(k, 2);
  let A;
  d(y), d(l), f((M, I, Z, K, V) => {
    z(e, a.common.legend), w = ze(y, 1, "svelte-1vtgp8q", null, w, M), P = ve(S, "", P, { "border-color": I }), j = ve(R, "", j, { "border-color": Z }), c = ve(k, "", c, { "border-color": K }), A = ve(x, "", A, { "border-color": V });
  }, [() => ({ wide: r.wide }), () => pe("info"), () => pe("notice"), () => pe("warning"), () => pe("critical")]), n(h, l), he();
}
var Zt = C('<div role="none" id="events"><div class="upper"><div class="header svelte-zmipc9"><div class="flex gap-10"><b>Events</b> <!></div> <!></div> <div class="data svelte-zmipc9"></div></div> <!></div>');
function Ut(h, r) {
  ue(r, true);
  const a = 50;
  let l = we(), i = W(void 0), e = W(false), y = W(void 0), w = W(L([])), S = W(L([])), P = W(L(Ue() && localStorage.getItem("eventLevel") || "info")), R = "";
  Te(() => {
    var _a;
    (_a = t(y)) == null ? void 0 : _a.close();
  }), ie(() => {
    t(i) && b(e, t(i) > 1680);
  }), ie(() => {
    t(P) !== R && (R = t(P), k());
  }), ie(() => {
    switch (t(P)) {
      case "info":
        b(S, L(t(w)));
        break;
      case "notice":
        b(S, L(t(w).filter((_) => _.typ === "Test" || _.level === "notice" || _.level === "warning" || _.level === "critical")));
        break;
      case "warning":
        b(S, L(t(w).filter((_) => _.typ === "Test" || _.level === "warning" || _.level === "critical")));
        break;
      case "critical":
        b(S, L(t(w).filter((_) => _.typ === "Test" || _.level === "critical")));
        break;
    }
  });
  async function j() {
    await We("/auth/v1/events/test");
  }
  function k() {
    var _a, _b;
    localStorage.setItem("eventLevel", t(P)), ((_a = t(y)) == null ? void 0 : _a.readyState) !== 2 && ((_b = t(y)) == null ? void 0 : _b.close()), b(y, L(new EventSource(`/auth/v1/events/stream?latest=${a}&level=${t(P).toLowerCase()}`))), t(y).onopen = () => {
      b(w, L([]));
    }, t(y).onerror = () => {
      console.error("SSE Events Stream closed");
    }, t(y).onmessage = (_) => {
      if (_.data) {
        let X = JSON.parse(_.data);
        console.log("event", X.id), b(w, L([X, ...t(w).slice(-499)]));
      }
    };
  }
  var c = Zt();
  let x;
  var A = u(c), M = u(A), I = u(M), Z = g(u(I), 2);
  Xe(Z, { get ariaLabel() {
    return l.events.eventLevel;
  }, options: De, borderless: true, get value() {
    return t(P);
  }, set value(_) {
    b(P, L(_));
  } }), d(I);
  var K = g(I, 2);
  ce(K, { level: 3, onclick: j, children: (_, X) => {
    q();
    var ne = H("Test");
    n(_, ne);
  }, $$slots: { default: true } }), d(M);
  var V = g(M, 2);
  Fe(V, 21, () => t(S), (_) => _.id, (_, X) => {
    Je(_, { get event() {
      return t(X);
    } });
  }), d(V), d(A);
  var Q = g(A, 2);
  Rt(Q, { get wide() {
    return t(e);
  } }), d(c), f((_) => x = ze(c, 1, "svelte-zmipc9", null, x, _), [() => ({ wide: t(e), narrow: !t(e) })]), _e("innerWidth", (_) => b(i, L(_))), n(h, c), he();
}
var Dt = C('<div class="noAdmin svelte-drznxl"><div><div class="text svelte-drznxl"><!></div> <!></div></div>'), Gt = C('<div class="noAdmin svelte-drznxl"><div><div class="text svelte-drznxl"><!></div> <!></div></div>'), Kt = C('<div class="events"><!></div>'), Ft = C('<div class="content svelte-drznxl"><!></div> <!>', 1), Jt = C("<!> <!>", 1);
function Mr(h, r) {
  ue(r, true), Oe();
  let a = we(), l = Ke("admin"), i = W(void 0), e = W(false), y = W(false), w = W(false);
  ie(() => {
    var _a;
    let c = l.get();
    c && (!!((_a = c == null ? void 0 : c.roles) == null ? void 0 : _a.includes("rauthy_admin")) ? (b(e, true), S()) : b(y, true));
  });
  async function S() {
    (await Le("/auth/v1/auth_check_admin")).status === 406 && b(w, true);
  }
  var P = fe();
  He((c) => {
    Ee.title = "Rauthy Admin";
  });
  var R = re(P);
  {
    var j = (c) => {
      var x = Dt(), A = u(x), M = u(A), I = u(M);
      Ie(I, () => a.error.noAdmin), d(M);
      var Z = g(M, 2);
      ce(Z, { onclick: () => window.location.href = "/auth/v1/account", children: (K, V) => {
        q();
        var Q = H();
        f(() => z(Q, a.common.account)), n(K, Q);
      }, $$slots: { default: true } }), d(A), d(x), n(c, x);
    }, k = (c, x) => {
      {
        var A = (I) => {
          var Z = Gt(), K = u(Z), V = u(K), Q = u(V);
          Ie(Q, () => a.error.needsAdminRole), d(V);
          var _ = g(V, 2);
          ce(_, { onclick: () => window.location.href = "/auth/v1/", children: (X, ne) => {
            q();
            var B = H();
            f(() => z(B, a.common.back)), n(X, B);
          }, $$slots: { default: true } }), d(K), d(Z), n(I, Z);
        }, M = (I, Z) => {
          {
            var K = (V) => {
              var Q = Jt(), _ = re(Q);
              Nt(_, {});
              var X = g(_, 2);
              qe(X, { children: (ne, B) => {
                var N = Ft(), $ = re(N), te = u($);
                de(te, () => r.children), d($);
                var se = g($, 2);
                {
                  var F = (T) => {
                    var E = Kt(), U = u(E);
                    Ut(U, {}), d(E), n(T, E);
                  };
                  ee(se, (T) => {
                    t(i) && t(i) > 1024 && T(F);
                  });
                }
                n(ne, N);
              } }), n(V, Q);
            };
            ee(I, (V) => {
              t(e) && V(K);
            }, Z);
          }
        };
        ee(c, (I) => {
          t(y) ? I(A) : I(M, false);
        }, x);
      }
    };
    ee(R, (c) => {
      t(w) ? c(j) : c(k, false);
    });
  }
  _e("innerWidth", (c) => b(i, L(c))), n(h, P), he();
}
export {
  Mr as component
};
