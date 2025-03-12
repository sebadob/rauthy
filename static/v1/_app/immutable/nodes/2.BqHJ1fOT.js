import { d as fe, a as n, t as P, n as O, e as q } from "../chunks/DKC5GJ29.js";
import { p as ue, a6 as je, l as b, k as V, f as re, j as t, a as he, c as u, r as d, t as f, a9 as ge, s as g, aa as ve, ab as D, a8 as W, ak as Te, a7 as Ee } from "../chunks/BveBAmlr.js";
import { s as L, e as Me, h as He } from "../chunks/CYCba2oX.js";
import { i as Y } from "../chunks/D-uYoVwt.js";
import { h as Ie } from "../chunks/CJj8kriY.js";
import { s as de } from "../chunks/Dv-Q3FDX.js";
import { p as T } from "../chunks/VbPNpVtZ.js";
import { b as _e } from "../chunks/DpR-4P2Y.js";
import { M as qe } from "../chunks/DaM_1q3c.js";
import { B as ce } from "../chunks/DlLEcmNg.js";
import { s as o, a as le, b as Le } from "../chunks/Dql74IOz.js";
import { T as Ce } from "../chunks/BFgWaswG.js";
import { L as Pe } from "../chunks/B5qGhNUP.js";
import { p as Be } from "../chunks/y1LQxlVJ.js";
import { f as ze, b as We } from "../chunks/BO1A6s0c.js";
import { R as Ve } from "../chunks/CE9vlfyw.js";
import { A as $e } from "../chunks/BPydPKHg.js";
import { p as v } from "../chunks/Db0ChEdV.js";
import { I as Ne } from "../chunks/B7o5HlxY.js";
import { u as we, i as Oe } from "../chunks/DtT3Jahq.js";
import { I as Re } from "../chunks/BVxkLPhB.js";
import { S as Ze, k as pe, i as Ue, B as De } from "../chunks/B21bTIl7.js";
import { T as Fe } from "../chunks/B100j8aI.js";
import { u as Ge } from "../chunks/CPBhBTQd.js";
import { e as Ke } from "../chunks/BLE4FKaJ.js";
import { E as Je } from "../chunks/ClhFTYjw.js";
import { u as Qe } from "../chunks/8R5My_LO.js";
import { O as Xe } from "../chunks/DdLeAElE.js";
var Ye = P('<div class="ver upd svelte-4tal6i"><a target="_blank" class="svelte-4tal6i"> </a></div>'), et = P('<div class="ver svelte-4tal6i"> </div>');
function tt(h, r) {
  ue(r, true);
  let a = V(void 0);
  je(() => {
    l();
  });
  async function l() {
    let w = await ze("/auth/v1/version");
    w.body ? b(a, T(w.body)) : console.error(w.error);
  }
  var i = fe(), e = re(i);
  {
    var y = (w) => {
      var z = fe(), B = re(z);
      {
        var R = (k) => {
          var c = Ye(), x = u(c), A = u(x);
          d(x), d(c), f(() => {
            o(x, "href", t(a).latest_url), L(A, `v${t(a) ?? ""} \u26A0\uFE0F`);
          }), n(k, c);
        }, S = (k) => {
          var c = et(), x = u(c);
          d(c), f(() => L(x, `v${t(a).current ?? ""}`)), n(k, c);
        };
        Y(B, (k) => {
          t(a).update_available ? k(R) : k(S, false);
        });
      }
      n(w, z);
    };
    Y(e, (w) => {
      t(a) && w(y);
    });
  }
  n(h, i), he();
}
var rt = P('<div class="compact svelte-7fczgn"><div class="iconCompact svelte-7fczgn"><!></div> <span class="svelte-7fczgn"><!></span></div>'), ot = P('<div class="wide svelte-7fczgn"><div class="iconWide svelte-7fczgn"><!></div> <div><!></div></div>');
function F(h, r) {
  const a = "/auth/v1/admin";
  let l = ge(() => r.compact ? "1.5rem" : "1.2rem"), i = ge(() => `${a}${r.route}${r.params}`);
  $e(h, { get href() {
    return t(i);
  }, hideUnderline: true, get highlightIncludes() {
    return r.highlightIncludes;
  }, children: (e, y) => {
    var w = fe(), z = re(w);
    {
      var B = (S) => {
        var k = rt(), c = u(k), x = u(c);
        de(x, () => r.icon, () => t(l)), d(c);
        var A = g(c, 2), I = u(A);
        de(I, () => r.children), d(A), d(k), n(S, k);
      }, R = (S) => {
        var k = ot(), c = u(k);
        let x;
        var A = u(c);
        de(A, () => r.icon, () => t(l)), d(c);
        var I = g(c, 2), C = u(I);
        de(C, () => r.children), d(I), d(k), f(() => x = le(c, "", x, { width: t(l) })), n(S, k);
      };
      Y(z, (S) => {
        r.compact ? S(B) : S(R, false);
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
var Lt = O(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75 11.25 15 15 9.75M21 12c0 1.268-.63 2.39-1.593 3.068a3.745 3.745 0 0 1-1.043 3.296 3.745
            3.745 0 0 1-3.296 1.043A3.745 3.745 0 0 1 12 21c-1.268 0-2.39-.63-3.068-1.593a3.746 3.746 0 0
            1-3.296-1.043 3.745 3.745 0 0 1-1.043-3.296A3.745 3.745 0 0 1 3 12c0-1.268.63-2.39 1.593-3.068a3.745
            3.745 0 0 1 1.043-3.296 3.746 3.746 0 0 1 3.296-1.043A3.746 3.746 0 0 1 12 3c1.268 0 2.39.63 3.068
            1.593a3.746 3.746 0 0 1 3.296 1.043 3.746 3.746 0 0 1 1.043 3.296A3.745 3.745 0 0 1 21 12Z"></path></svg>`);
function zt(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = Lt();
  o(e, "stroke-width", 2), f(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var St = P('<span class="inner svelte-8anrni"><!></span>'), jt = P('<span class="inner svelte-8anrni"><!></span>'), Tt = P("<div><!></div>"), Et = P('<div class="logo svelte-8anrni"><!></div> <div class="menu svelte-8anrni"><!> <!> <!> <!> <!> <!> <!> <!> <!> <!> <!> <!> <!></div>', 1), Ht = P('<div class="logout svelte-8anrni"><!></div>'), qt = P('<!> <div class="theme svelte-8anrni"><!></div> <!>', 1), Wt = P('<div class="flex gap-05"><!> <!> <!></div> <div class="version svelte-8anrni"><!></div>', 1), Vt = P('<div class="bottom svelte-8anrni"><!></div>'), $t = P('<nav id="mainNav" class="svelte-8anrni"><div class="svelte-8anrni"><div><div class="relative"><div class="absolute flex space-between navmod svelte-8anrni"><div><!></div> <!></div></div> <!></div> <!></div></nav>');
function Nt(h, r) {
  ue(r, true);
  let a = we(), l = V(false), i = V(""), e = V(false), y = V(false), w = V(void 0);
  ve(() => {
    b(i, T(Be.url.search));
  }), ve(() => {
    if (!t(w)) return;
    let M = Be.url && t(w) < 600 || t(w) < 800, E = t(w) < 1280;
    requestAnimationFrame(() => {
      b(y, T(M)), b(e, E);
    });
  });
  function z() {
    b(l, true);
  }
  function B() {
    setTimeout(() => {
      b(l, false);
    }, 1e3);
  }
  function R() {
    b(y, !t(y));
  }
  function S() {
    b(e, !t(e));
  }
  var k = $t(), c = u(k), x = u(c), A = u(x), I = u(A), C = u(I), Z = u(C);
  ce(Z, { ariaControls: "mainNav", invisible: true, onclick: R, children: (M, E) => {
    var N = St(), te = u(N);
    it(te, {}), d(N), n(M, N);
  }, $$slots: { default: true } }), d(C);
  var G = g(C, 2);
  {
    var $ = (M) => {
      var E = Tt(), N = u(E);
      ce(N, { invisible: true, onclick: S, children: (te, ne) => {
        var K = jt(), j = u(K);
        st(j, {}), d(K), n(te, K);
      }, $$slots: { default: true } }), d(E), n(M, E);
    };
    Y(G, (M) => {
      t(y) || M($);
    });
  }
  d(I), d(A);
  var Q = g(A, 2);
  {
    var _ = (M) => {
      var E = Et(), N = re(E), te = u(N);
      const ne = ge(() => t(e) ? "3rem" : "7rem");
      Ve(te, { get width() {
        return t(ne);
      } }), d(N);
      var K = g(N, 2);
      let j;
      var H = u(K);
      F(H, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/users", icon: (s, m = D) => {
        gt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.users)), n(s, p);
      } });
      var U = g(H, 2);
      F(U, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/clients", icon: (s, m = D) => {
        ht(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.clients)), n(s, p);
      } });
      var X = g(U, 2);
      F(X, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/roles", icon: (s, m = D) => {
        zt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.roles)), n(s, p);
      } });
      var oe = g(X, 2);
      F(oe, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/groups", icon: (s, m = D) => {
        vt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.groups)), n(s, p);
      } });
      var ae = g(oe, 2);
      F(ae, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/attributes", icon: (s, m = D) => {
        dt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.attributes)), n(s, p);
      } });
      var se = g(ae, 2);
      F(se, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/scopes", icon: (s, m = D) => {
        wt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.scopes)), n(s, p);
      } });
      var me = g(se, 2);
      F(me, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/sessions", icon: (s, m = D) => {
        pt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.sessions)), n(s, p);
      } });
      var ke = g(me, 2);
      F(ke, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/events", icon: (s, m = D) => {
        yt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.events)), n(s, p);
      } });
      var ye = g(ke, 2);
      F(ye, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/blacklist", icon: (s, m = D) => {
        Ne(s, { get width() {
          return m();
        }, color: "currentColor" });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.blacklist)), n(s, p);
      } });
      var xe = g(ye, 2);
      F(xe, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/api_keys", icon: (s, m = D) => {
        bt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.apiKeys)), n(s, p);
      } });
      var be = g(xe, 2);
      F(be, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/providers", icon: (s, m = D) => {
        Mt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.providers)), n(s, p);
      } });
      var Ae = g(be, 2);
      F(Ae, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/config/policy", highlightIncludes: "/config/", icon: (s, m = D) => {
        Ct(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.config)), n(s, p);
      } });
      var Se = g(Ae, 2);
      F(Se, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/docs", icon: (s, m = D) => {
        Bt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.docs)), n(s, p);
      } }), d(K), f(() => j = le(K, "", j, { "margin-top": t(e) ? ".5rem" : "1.5rem" })), n(M, E);
    };
    Y(Q, (M) => {
      t(y) || M(_);
    });
  }
  d(x);
  var ee = g(x, 2);
  {
    var ie = (M) => {
      var E = Vt();
      {
        const K = (j) => {
          var H = Ht(), U = u(H);
          ce(U, { invisible: true, onclick: () => Ze(), children: (X, oe) => {
            Fe(X, { text: "Logout", children: (ae, se) => {
              Re(ae, {});
            }, $$slots: { default: true } });
          }, $$slots: { default: true } }), d(H), n(j, H);
        };
        var N = u(E);
        {
          var te = (j) => {
            var H = qt(), U = re(H);
            K(U);
            var X = g(U, 2), oe = u(X);
            Ce(oe, {}), d(X);
            var ae = g(X, 2);
            Pe(ae, { openTop: true }), n(j, H);
          }, ne = (j) => {
            var H = Wt(), U = re(H), X = u(U);
            Ce(X, {});
            var oe = g(X, 2);
            Pe(oe, { openTop: true });
            var ae = g(oe, 2);
            K(ae), d(U);
            var se = g(U, 2), me = u(se);
            tt(me, {}), d(se), n(j, H);
          };
          Y(N, (j) => {
            t(e) ? j(te) : j(ne, false);
          });
        }
        d(E);
      }
      n(M, E);
    };
    Y(ee, (M) => {
      t(y) || M(ie);
    });
  }
  d(c), d(k), f(() => {
    o(k, "aria-hidden", t(y)), o(k, "data-compact", t(e)), o(I, "aria-hidden", !t(y) && !t(l));
  }), _e("innerWidth", (M) => b(w, T(M))), Me("mouseenter", k, z), Me("mouseleave", k, B), n(h, k), he();
}
var Ot = P('<fieldset class="svelte-1vtgp8q"><legend class="font-label svelte-1vtgp8q"> </legend> <ul><li class="svelte-1vtgp8q">Info</li> <li class="svelte-1vtgp8q">Notice</li> <li class="svelte-1vtgp8q">Warning</li> <li class="svelte-1vtgp8q">Critical</li></ul></fieldset>');
function Rt(h, r) {
  ue(r, true);
  let a = Qe();
  var l = Ot(), i = u(l), e = u(i, true);
  d(i);
  var y = g(i, 2);
  let w;
  var z = u(y);
  let B;
  var R = g(z, 2);
  let S;
  var k = g(R, 2);
  let c;
  var x = g(k, 2);
  let A;
  d(y), d(l), f((I, C, Z, G, $) => {
    L(e, a.common.legend), w = Le(y, 1, "svelte-1vtgp8q", null, w, I), B = le(z, "", B, { "border-color": C }), S = le(R, "", S, { "border-color": Z }), c = le(k, "", c, { "border-color": G }), A = le(x, "", A, { "border-color": $ });
  }, [() => ({ wide: r.wide }), () => pe("info"), () => pe("notice"), () => pe("warning"), () => pe("critical")]), n(h, l), he();
}
var Zt = P('<div role="none" id="events"><div class="upper"><div class="header svelte-zmipc9"><div class="flex gap-10"><b>Events</b> <!></div> <!></div> <div class="data svelte-zmipc9"></div></div> <!></div>');
function Ut(h, r) {
  var _a;
  ue(r, true);
  const a = 50;
  let l = we(), i = V(void 0), e = V(false), y = V(void 0), w = V(T([])), z = V(T([])), B = V(T(Ue() && ((_a = localStorage.getItem("eventLevel")) == null ? void 0 : _a.toLowerCase()) || "info")), R = "";
  Te(() => {
    var _a2;
    (_a2 = t(y)) == null ? void 0 : _a2.close();
  }), ve(() => {
    t(i) && b(e, t(i) > 1680);
  }), ve(() => {
    t(B) !== R && (console.log("running level effect"), R = t(B), k());
  }), ve(() => {
    switch (t(B)) {
      case "info":
        b(z, T(t(w)));
        break;
      case "notice":
        b(z, T(t(w).filter((_) => _.typ === "Test" || _.level === "notice" || _.level === "warning" || _.level === "critical")));
        break;
      case "warning":
        b(z, T(t(w).filter((_) => _.typ === "Test" || _.level === "warning" || _.level === "critical")));
        break;
      case "critical":
        b(z, T(t(w).filter((_) => _.typ === "Test" || _.level === "critical")));
        break;
    }
  });
  async function S() {
    await We("/auth/v1/events/test");
  }
  function k() {
    localStorage.setItem("eventLevel", t(B)), t(y) && t(y).readyState !== 2 && t(y).close(), b(y, T(new EventSource(`/auth/v1/events/stream?latest=${a}&level=${t(B).toLowerCase()}`))), t(y).onopen = () => {
      b(w, T([]));
    }, t(y).onerror = () => {
      console.error("SSE Events Stream closed");
    }, t(y).onmessage = (_) => {
      if (_.data) {
        let ee = JSON.parse(_.data);
        b(w, T([ee, ...t(w).slice(-499)]));
      }
    };
  }
  var c = Zt();
  let x;
  var A = u(c), I = u(A), C = u(I), Z = g(u(C), 2);
  Xe(Z, { get ariaLabel() {
    return l.events.eventLevel;
  }, options: De, borderless: true, get value() {
    return t(B);
  }, set value(_) {
    b(B, T(_));
  } }), d(C);
  var G = g(C, 2);
  ce(G, { level: 3, onclick: S, children: (_, ee) => {
    W();
    var ie = q("Test");
    n(_, ie);
  }, $$slots: { default: true } }), d(I);
  var $ = g(I, 2);
  Ke($, 21, () => t(z), (_) => _.id, (_, ee) => {
    Je(_, { get event() {
      return t(ee);
    } });
  }), d($), d(A);
  var Q = g(A, 2);
  Rt(Q, { get wide() {
    return t(e);
  } }), d(c), f((_) => x = Le(c, 1, "svelte-zmipc9", null, x, _), [() => ({ wide: t(e), narrow: !t(e) })]), _e("innerWidth", (_) => b(i, T(_))), n(h, c), he();
}
var Dt = P('<div class="noAdmin svelte-drznxl"><div><div class="text svelte-drznxl"><!></div> <!></div></div>'), Ft = P('<div class="noAdmin svelte-drznxl"><div><div class="text svelte-drznxl"><!></div> <!></div></div>'), Gt = P('<div class="events"><!></div>'), Kt = P('<div class="content svelte-drznxl"><!></div> <!>', 1), Jt = P("<!> <!>", 1);
function Mr(h, r) {
  ue(r, true), Oe();
  let a = we(), l = Ge("admin"), i = V(void 0), e = V(false), y = V(false), w = V(false);
  ve(() => {
    var _a;
    let c = l.get();
    c && (!!((_a = c == null ? void 0 : c.roles) == null ? void 0 : _a.includes("rauthy_admin")) ? (b(e, true), z()) : b(y, true));
  });
  async function z() {
    (await ze("/auth/v1/auth_check_admin")).status === 406 && b(w, true);
  }
  var B = fe();
  He((c) => {
    Ee.title = "Rauthy Admin";
  });
  var R = re(B);
  {
    var S = (c) => {
      var x = Dt(), A = u(x), I = u(A), C = u(I);
      Ie(C, () => a.error.noAdmin), d(I);
      var Z = g(I, 2);
      ce(Z, { onclick: () => window.location.href = "/auth/v1/account", children: (G, $) => {
        W();
        var Q = q();
        f(() => L(Q, a.common.account)), n(G, Q);
      }, $$slots: { default: true } }), d(A), d(x), n(c, x);
    }, k = (c, x) => {
      {
        var A = (C) => {
          var Z = Ft(), G = u(Z), $ = u(G), Q = u($);
          Ie(Q, () => a.error.needsAdminRole), d($);
          var _ = g($, 2);
          ce(_, { onclick: () => window.location.href = "/auth/v1/", children: (ee, ie) => {
            W();
            var M = q();
            f(() => L(M, a.common.back)), n(ee, M);
          }, $$slots: { default: true } }), d(G), d(Z), n(C, Z);
        }, I = (C, Z) => {
          {
            var G = ($) => {
              var Q = Jt(), _ = re(Q);
              Nt(_, {});
              var ee = g(_, 2);
              qe(ee, { children: (ie, M) => {
                var E = Kt(), N = re(E), te = u(N);
                de(te, () => r.children), d(N);
                var ne = g(N, 2);
                {
                  var K = (j) => {
                    var H = Gt(), U = u(H);
                    Ut(U, {}), d(H), n(j, H);
                  };
                  Y(ne, (j) => {
                    t(i) && t(i) > 1024 && j(K);
                  });
                }
                n(ie, E);
              } }), n($, Q);
            };
            Y(C, ($) => {
              t(e) && $(G);
            }, Z);
          }
        };
        Y(c, (C) => {
          t(y) ? C(A) : C(I, false);
        }, x);
      }
    };
    Y(R, (c) => {
      t(w) ? c(S) : c(k, false);
    });
  }
  _e("innerWidth", (c) => b(i, T(c))), n(h, B), he();
}
export {
  Mr as component
};
