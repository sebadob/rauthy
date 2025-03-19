import { d as ft, a as n, t as P, n as O, e as q } from "../chunks/DLBGyKVC.js";
import { p as ut, a6 as jt, l as b, k as V, f as rt, j as e, a as ht, c as u, r as d, t as f, a9 as gt, s as g, aa as vt, ab as D, a8 as W, ak as Tt, a7 as Et } from "../chunks/CmQi0fbH.js";
import { s as L, e as Mt, h as Ht } from "../chunks/BjaYyaa_.js";
import { i as Y } from "../chunks/C6bK2EJJ.js";
import { h as It } from "../chunks/UlQQkuaW.js";
import { s as dt } from "../chunks/71gCR-8F.js";
import { p as T } from "../chunks/B_ggA-0N.js";
import { b as _t } from "../chunks/DzJ8OZ_u.js";
import { M as qt } from "../chunks/Cf3VYlYr.js";
import { B as ct } from "../chunks/DPLO-ozG.js";
import { s as o, a as lt, b as Lt } from "../chunks/DOlUUCkJ.js";
import { T as Ct } from "../chunks/DB8eOlLu.js";
import { L as Pt } from "../chunks/DJEyp40I.js";
import { p as Bt } from "../chunks/CWypd1E1.js";
import { f as zt, b as Wt } from "../chunks/BO1A6s0c.js";
import { R as Vt } from "../chunks/W_Ypw_dT.js";
import { A as $t } from "../chunks/D3vXntHU.js";
import { p as v } from "../chunks/DNJm3-SG.js";
import { I as Nt } from "../chunks/Dth7WIu-.js";
import { u as wt, i as Ot } from "../chunks/CZf6fJph.js";
import { I as Rt } from "../chunks/pt0gL0d6.js";
import { S as Zt, k as pt, i as Ut, B as Dt } from "../chunks/B21bTIl7.js";
import { T as Ft } from "../chunks/ibGTLHQD.js";
import { u as Gt } from "../chunks/CMm1ZRdW.js";
import { e as Kt } from "../chunks/YQCw2eEa.js";
import { E as Jt } from "../chunks/C2k6LJdu.js";
import { u as Qt } from "../chunks/DGTOa5g8.js";
import { O as Xt } from "../chunks/DqhsozSK.js";
var Yt = P('<div class="ver upd svelte-4tal6i"><a target="_blank" class="svelte-4tal6i"> </a></div>'), te = P('<div class="ver svelte-4tal6i"> </div>');
function ee(h, r) {
  ut(r, true);
  let a = V(void 0);
  jt(() => {
    l();
  });
  async function l() {
    let w = await zt("/auth/v1/version");
    w.body ? b(a, T(w.body)) : console.error(w.error);
  }
  var i = ft(), t = rt(i);
  {
    var y = (w) => {
      var z = ft(), B = rt(z);
      {
        var R = (k) => {
          var c = Yt(), x = u(c), A = u(x);
          d(x), d(c), f(() => {
            o(x, "href", e(a).latest_url), L(A, `v${e(a) ?? ""} \u26A0\uFE0F`);
          }), n(k, c);
        }, S = (k) => {
          var c = te(), x = u(c);
          d(c), f(() => L(x, `v${e(a).current ?? ""}`)), n(k, c);
        };
        Y(B, (k) => {
          e(a).update_available ? k(R) : k(S, false);
        });
      }
      n(w, z);
    };
    Y(t, (w) => {
      e(a) && w(y);
    });
  }
  n(h, i), ht();
}
var re = P('<div class="compact svelte-7fczgn"><div class="iconCompact svelte-7fczgn"><!></div> <span class="svelte-7fczgn"><!></span></div>'), oe = P('<div class="wide svelte-7fczgn"><div class="iconWide svelte-7fczgn"><!></div> <div><!></div></div>');
function F(h, r) {
  const a = "/auth/v1/admin";
  let l = gt(() => r.compact ? "1.5rem" : "1.2rem"), i = gt(() => `${a}${r.route}${r.params}`);
  $t(h, { get href() {
    return e(i);
  }, hideUnderline: true, get highlightIncludes() {
    return r.highlightIncludes;
  }, children: (t, y) => {
    var w = ft(), z = rt(w);
    {
      var B = (S) => {
        var k = re(), c = u(k), x = u(c);
        dt(x, () => r.icon, () => e(l)), d(c);
        var A = g(c, 2), I = u(A);
        dt(I, () => r.children), d(A), d(k), n(S, k);
      }, R = (S) => {
        var k = oe(), c = u(k);
        let x;
        var A = u(c);
        dt(A, () => r.icon, () => e(l)), d(c);
        var I = g(c, 2), C = u(I);
        dt(C, () => r.children), d(I), d(k), f(() => x = lt(c, "", x, { width: e(l) })), n(S, k);
      };
      Y(z, (S) => {
        r.compact ? S(B) : S(R, false);
      });
    }
    n(t, w);
  }, $$slots: { default: true } });
}
var ae = O('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"></path></svg>');
function ie(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var t = ae();
  o(t, "stroke-width", 2), f(() => {
    o(t, "stroke", i()), o(t, "width", l()), o(t, "opacity", a());
  }), n(h, t);
}
var ne = O(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M19.5 12c0-1.232-.046-2.453-.138-3.662a4.006 4.006 0 0 0-3.7-3.7 48.678 48.678 0 0 0-7.324 0 4.006 4.006
            0 0 0-3.7 3.7c-.017.22-.032.441-.046.662M19.5 12l3-3m-3 3-3-3m-12 3c0 1.232.046 2.453.138 3.662a4.006 4.006
            0 0 0 3.7 3.7 48.656 48.656 0 0 0 7.324 0 4.006 4.006 0 0 0 3.7-3.7c.017-.22.032-.441.046-.662M4.5 12l3
            3m-3-3-3 3"></path></svg>`);
function se(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var t = ne();
  o(t, "stroke-width", 2), f(() => {
    o(t, "stroke", i()), o(t, "width", l()), o(t, "opacity", a());
  }), n(h, t);
}
var le = O(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M18 18.72a9.094 9.094 0 0 0 3.741-.479 3 3 0 0 0-4.682-2.72m.94 3.198.001.031c0
            .225-.012.447-.037.666A11.944 11.944 0 0 1 12 21c-2.17 0-4.207-.576-5.963-1.584A6.062 6.062 0 0 1 6
            18.719m12 0a5.971 5.971 0 0 0-.941-3.197m0 0A5.995 5.995 0 0 0 12 12.75a5.995 5.995 0 0 0-5.058 2.772m0
            0a3 3 0 0 0-4.681 2.72 8.986 8.986 0 0 0 3.74.477m.94-3.197a5.971 5.971 0 0 0-.94 3.197M15 6.75a3 3 0 1
            1-6 0 3 3 0 0 1 6 0Zm6 3a2.25 2.25 0 1 1-4.5 0 2.25 2.25 0 0 1 4.5 0Zm-13.5 0a2.25 2.25 0 1 1-4.5 0 2.25
            2.25 0 0 1 4.5 0Z"></path></svg>`);
function ve(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var t = le();
  o(t, "stroke-width", 2), f(() => {
    o(t, "stroke", i()), o(t, "width", l()), o(t, "opacity", a());
  }), n(h, t);
}
var ce = O('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z"></path></svg>');
function de(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var t = ce();
  o(t, "stroke-width", 2), f(() => {
    o(t, "stroke", i()), o(t, "width", l()), o(t, "opacity", a());
  }), n(h, t);
}
var ue = O(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 21h16.5M4.5 3h15M5.25 3v18m13.5-18v18M9 6.75h1.5m-1.5 3h1.5m-1.5 3h1.5m3-6H15m-1.5 3H15m-1.5
            3H15M9 21v-3.375c0-.621.504-1.125 1.125-1.125h3.75c.621 0 1.125.504 1.125 1.125V21"></path></svg>`);
function he(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var t = ue();
  o(t, "stroke-width", 2), f(() => {
    o(t, "stroke", i()), o(t, "width", l()), o(t, "opacity", a());
  }), n(h, t);
}
var me = O('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75L11.25 15 15 9.75m-3-7.036A11.959 11.959 0 013.598 6 11.99 11.99 0 003 9.749c0 5.592 3.824 10.29 9 11.623 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.285z"></path></svg>');
function pe(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var t = me();
  o(t, "stroke-width", 2), f(() => {
    o(t, "stroke", i()), o(t, "width", l()), o(t, "opacity", a());
  }), n(h, t);
}
var fe = O('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z"></path></svg>');
function ge(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var t = fe();
  o(t, "stroke-width", 2), f(() => {
    o(t, "stroke", i()), o(t, "width", l()), o(t, "opacity", a());
  }), n(h, t);
}
var _e = O(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M15 9h3.75M15 12h3.75M15 15h3.75M4.5 19.5h15a2.25 2.25 0 002.25-2.25V6.75A2.25 2.25 0 0019.5
            4.5h-15a2.25 2.25 0 00-2.25 2.25v10.5A2.25 2.25 0 004.5 19.5zm6-10.125a1.875 1.875 0 11-3.75 0
            1.875 1.875 0 013.75 0zm1.294 6.336a6.721 6.721 0 01-3.17.789 6.721 6.721 0 01-3.168-.789 3.376
            3.376 0 016.338 0z"></path></svg>`);
function we(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var t = _e();
  o(t, "stroke-width", 2), f(() => {
    o(t, "stroke", i()), o(t, "width", l()), o(t, "opacity", a());
  }), n(h, t);
}
var ke = O(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 0 0 5.454-1.31A8.967 8.967 0 0 1 18 9.75V9A6 6 0 0 0 6 9v.75a8.967 8.967
            0 0 1-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 0 1-5.714 0m5.714 0a3 3 0 1 1-5.714
            0"></path></svg>`);
function ye(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var t = ke();
  o(t, "stroke-width", 2), f(() => {
    o(t, "stroke", i()), o(t, "width", l()), o(t, "opacity", a());
  }), n(h, t);
}
var xe = O('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M15.75 5.25a3 3 0 013 3m3 0a6 6 0 01-7.029 5.912c-.563-.097-1.159.026-1.563.43L10.5 17.25H8.25v2.25H6v2.25H2.25v-2.818c0-.597.237-1.17.659-1.591l6.499-6.499c.404-.404.527-1 .43-1.563A6 6 0 1121.75 8.25z"></path></svg>');
function be(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var t = xe();
  o(t, "stroke-width", 2), f(() => {
    o(t, "stroke", i()), o(t, "width", l()), o(t, "opacity", a());
  }), n(h, t);
}
var Ae = O(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M2.25 15a4.5 4.5 0 0 0 4.5 4.5H18a3.75 3.75 0 0 0 1.332-7.257 3 3 0 0 0-3.758-3.848 5.25 5.25 0 0
            0-10.233 2.33A4.502 4.502 0 0 0 2.25 15Z"></path></svg>`);
function Me(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var t = Ae();
  o(t, "stroke-width", 2), f(() => {
    o(t, "stroke", i()), o(t, "width", l()), o(t, "opacity", a());
  }), n(h, t);
}
var Ie = O('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M11.42 15.17L17.25 21A2.652 2.652 0 0021 17.25l-5.877-5.877M11.42 15.17l2.496-3.03c.317-.384.74-.626 1.208-.766M11.42 15.17l-4.655 5.653a2.548 2.548 0 11-3.586-3.586l6.837-5.63m5.108-.233c.55-.164 1.163-.188 1.743-.14a4.5 4.5 0 004.486-6.336l-3.276 3.277a3.004 3.004 0 01-2.25-2.25l3.276-3.276a4.5 4.5 0 00-6.336 4.486c.091 1.076-.071 2.264-.904 2.95l-.102.085m-1.745 1.437L5.909 7.5H4.5L2.25 3.75l1.5-1.5L7.5 4.5v1.409l4.26 4.26m-1.745 1.437l1.745-1.437m6.615 8.206L15.75 15.75M4.867 19.125h.008v.008h-.008v-.008z"></path></svg>');
function Ce(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var t = Ie();
  o(t, "stroke-width", 2), f(() => {
    o(t, "stroke", i()), o(t, "width", l()), o(t, "opacity", a());
  }), n(h, t);
}
var Pe = O('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M12 6.042A8.967 8.967 0 006 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 016 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 016-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0018 18a8.967 8.967 0 00-6 2.292m0-14.25v14.25"></path></svg>');
function Be(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var t = Pe();
  o(t, "stroke-width", 2), f(() => {
    o(t, "stroke", i()), o(t, "width", l()), o(t, "opacity", a());
  }), n(h, t);
}
var Le = O(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75 11.25 15 15 9.75M21 12c0 1.268-.63 2.39-1.593 3.068a3.745 3.745 0 0 1-1.043 3.296 3.745
            3.745 0 0 1-3.296 1.043A3.745 3.745 0 0 1 12 21c-1.268 0-2.39-.63-3.068-1.593a3.746 3.746 0 0
            1-3.296-1.043 3.745 3.745 0 0 1-1.043-3.296A3.745 3.745 0 0 1 3 12c0-1.268.63-2.39 1.593-3.068a3.745
            3.745 0 0 1 1.043-3.296 3.746 3.746 0 0 1 3.296-1.043A3.746 3.746 0 0 1 12 3c1.268 0 2.39.63 3.068
            1.593a3.746 3.746 0 0 1 3.296 1.043 3.746 3.746 0 0 1 1.043 3.296A3.745 3.745 0 0 1 21 12Z"></path></svg>`);
function ze(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var t = Le();
  o(t, "stroke-width", 2), f(() => {
    o(t, "stroke", i()), o(t, "width", l()), o(t, "opacity", a());
  }), n(h, t);
}
var Se = P('<span class="inner svelte-1vxrtnb"><!></span>'), je = P('<span class="inner svelte-1vxrtnb"><!></span>'), Te = P("<div><!></div>"), Ee = P('<div class="logo svelte-1vxrtnb"><!></div> <div class="menu svelte-1vxrtnb"><!> <!> <!> <!> <!> <!> <!> <!> <!> <!> <!> <!> <!></div>', 1), He = P('<div class="logout svelte-1vxrtnb"><!></div>'), qe = P('<!> <div class="theme svelte-1vxrtnb"><!></div> <!>', 1), We = P('<div class="flex gap-05"><!> <!> <!></div> <div class="version svelte-1vxrtnb"><!></div>', 1), Ve = P('<div class="bottom svelte-1vxrtnb"><!></div>'), $e = P('<nav id="mainNav" class="svelte-1vxrtnb"><div class="svelte-1vxrtnb"><div><div class="relative"><div class="absolute flex space-between navmod svelte-1vxrtnb"><div><!></div> <!></div></div> <!></div> <!></div></nav>');
function Ne(h, r) {
  ut(r, true);
  let a = wt(), l = V(false), i = V(""), t = V(false), y = V(false), w = V(void 0);
  vt(() => {
    b(i, T(Bt.url.search));
  }), vt(() => {
    if (!e(w)) return;
    let M = Bt.url && e(w) < 600 || e(w) < 800, E = e(w) < 1280;
    requestAnimationFrame(() => {
      b(y, T(M)), b(t, E);
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
    b(y, !e(y));
  }
  function S() {
    b(t, !e(t));
  }
  var k = $e(), c = u(k), x = u(c), A = u(x), I = u(A), C = u(I), Z = u(C);
  ct(Z, { ariaControls: "mainNav", invisible: true, onclick: R, children: (M, E) => {
    var N = Se(), et = u(N);
    ie(et, {}), d(N), n(M, N);
  }, $$slots: { default: true } }), d(C);
  var G = g(C, 2);
  {
    var $ = (M) => {
      var E = Te(), N = u(E);
      ct(N, { invisible: true, onclick: S, children: (et, nt) => {
        var K = je(), j = u(K);
        se(j, {}), d(K), n(et, K);
      }, $$slots: { default: true } }), d(E), n(M, E);
    };
    Y(G, (M) => {
      e(y) || M($);
    });
  }
  d(I), d(A);
  var Q = g(A, 2);
  {
    var _ = (M) => {
      var E = Ee(), N = rt(E), et = u(N);
      const nt = gt(() => e(t) ? "3rem" : "7rem");
      Vt(et, { get width() {
        return e(nt);
      } }), d(N);
      var K = g(N, 2);
      let j;
      var H = u(K);
      F(H, { get compact() {
        return e(t);
      }, get params() {
        return e(i);
      }, route: "/users", icon: (s, m = D) => {
        ge(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.users)), n(s, p);
      } });
      var U = g(H, 2);
      F(U, { get compact() {
        return e(t);
      }, get params() {
        return e(i);
      }, route: "/clients", icon: (s, m = D) => {
        he(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.clients)), n(s, p);
      } });
      var X = g(U, 2);
      F(X, { get compact() {
        return e(t);
      }, get params() {
        return e(i);
      }, route: "/roles", icon: (s, m = D) => {
        ze(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.roles)), n(s, p);
      } });
      var ot = g(X, 2);
      F(ot, { get compact() {
        return e(t);
      }, get params() {
        return e(i);
      }, route: "/groups", icon: (s, m = D) => {
        ve(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.groups)), n(s, p);
      } });
      var at = g(ot, 2);
      F(at, { get compact() {
        return e(t);
      }, get params() {
        return e(i);
      }, route: "/attributes", icon: (s, m = D) => {
        de(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.attributes)), n(s, p);
      } });
      var st = g(at, 2);
      F(st, { get compact() {
        return e(t);
      }, get params() {
        return e(i);
      }, route: "/scopes", icon: (s, m = D) => {
        we(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.scopes)), n(s, p);
      } });
      var mt = g(st, 2);
      F(mt, { get compact() {
        return e(t);
      }, get params() {
        return e(i);
      }, route: "/sessions", icon: (s, m = D) => {
        pe(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.sessions)), n(s, p);
      } });
      var kt = g(mt, 2);
      F(kt, { get compact() {
        return e(t);
      }, get params() {
        return e(i);
      }, route: "/events", icon: (s, m = D) => {
        ye(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.events)), n(s, p);
      } });
      var yt = g(kt, 2);
      F(yt, { get compact() {
        return e(t);
      }, get params() {
        return e(i);
      }, route: "/blacklist", icon: (s, m = D) => {
        Nt(s, { get width() {
          return m();
        }, color: "currentColor" });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.blacklist)), n(s, p);
      } });
      var xt = g(yt, 2);
      F(xt, { get compact() {
        return e(t);
      }, get params() {
        return e(i);
      }, route: "/api_keys", icon: (s, m = D) => {
        be(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.apiKeys)), n(s, p);
      } });
      var bt = g(xt, 2);
      F(bt, { get compact() {
        return e(t);
      }, get params() {
        return e(i);
      }, route: "/providers", icon: (s, m = D) => {
        Me(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.providers)), n(s, p);
      } });
      var At = g(bt, 2);
      F(At, { get compact() {
        return e(t);
      }, get params() {
        return e(i);
      }, route: "/config/policy", highlightIncludes: "/config/", icon: (s, m = D) => {
        Ce(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.config)), n(s, p);
      } });
      var St = g(At, 2);
      F(St, { get compact() {
        return e(t);
      }, get params() {
        return e(i);
      }, route: "/docs", icon: (s, m = D) => {
        Be(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        W();
        var p = q();
        f(() => L(p, a.nav.docs)), n(s, p);
      } }), d(K), f(() => j = lt(K, "", j, { "margin-top": e(t) ? ".5rem" : "1.5rem" })), n(M, E);
    };
    Y(Q, (M) => {
      e(y) || M(_);
    });
  }
  d(x);
  var tt = g(x, 2);
  {
    var it = (M) => {
      var E = Ve();
      {
        const K = (j) => {
          var H = He(), U = u(H);
          ct(U, { invisible: true, onclick: () => Zt(), children: (X, ot) => {
            Ft(X, { text: "Logout", children: (at, st) => {
              Rt(at, {});
            }, $$slots: { default: true } });
          }, $$slots: { default: true } }), d(H), n(j, H);
        };
        var N = u(E);
        {
          var et = (j) => {
            var H = qe(), U = rt(H);
            K(U);
            var X = g(U, 2), ot = u(X);
            Ct(ot, {}), d(X);
            var at = g(X, 2);
            Pt(at, { openTop: true }), n(j, H);
          }, nt = (j) => {
            var H = We(), U = rt(H), X = u(U);
            Ct(X, {});
            var ot = g(X, 2);
            Pt(ot, { openTop: true });
            var at = g(ot, 2);
            K(at), d(U);
            var st = g(U, 2), mt = u(st);
            ee(mt, {}), d(st), n(j, H);
          };
          Y(N, (j) => {
            e(t) ? j(et) : j(nt, false);
          });
        }
        d(E);
      }
      n(M, E);
    };
    Y(tt, (M) => {
      e(y) || M(it);
    });
  }
  d(c), d(k), f(() => {
    o(k, "aria-hidden", e(y)), o(k, "data-compact", e(t)), o(I, "aria-hidden", !e(y) && !e(l));
  }), _t("innerWidth", (M) => b(w, T(M))), Mt("mouseenter", k, z), Mt("mouseleave", k, B), n(h, k), ht();
}
var Oe = P('<fieldset class="svelte-1vtgp8q"><legend class="font-label svelte-1vtgp8q"> </legend> <ul><li class="svelte-1vtgp8q">Info</li> <li class="svelte-1vtgp8q">Notice</li> <li class="svelte-1vtgp8q">Warning</li> <li class="svelte-1vtgp8q">Critical</li></ul></fieldset>');
function Re(h, r) {
  ut(r, true);
  let a = Qt();
  var l = Oe(), i = u(l), t = u(i, true);
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
    L(t, a.common.legend), w = Lt(y, 1, "svelte-1vtgp8q", null, w, I), B = lt(z, "", B, { "border-color": C }), S = lt(R, "", S, { "border-color": Z }), c = lt(k, "", c, { "border-color": G }), A = lt(x, "", A, { "border-color": $ });
  }, [() => ({ wide: r.wide }), () => pt("info"), () => pt("notice"), () => pt("warning"), () => pt("critical")]), n(h, l), ht();
}
var Ze = P('<div role="none" id="events"><div class="upper"><div class="header svelte-zmipc9"><div class="flex gap-10"><b>Events</b> <!></div> <!></div> <div class="data svelte-zmipc9"></div></div> <!></div>');
function Ue(h, r) {
  var _a;
  ut(r, true);
  const a = 50;
  let l = wt(), i = V(void 0), t = V(false), y = V(void 0), w = V(T([])), z = V(T([])), B = V(T(Ut() && ((_a = localStorage.getItem("eventLevel")) == null ? void 0 : _a.toLowerCase()) || "info")), R = "";
  Tt(() => {
    var _a2;
    (_a2 = e(y)) == null ? void 0 : _a2.close();
  }), vt(() => {
    e(i) && b(t, e(i) > 1680);
  }), vt(() => {
    e(B) !== R && (R = e(B), k());
  }), vt(() => {
    switch (e(B)) {
      case "info":
        b(z, T(e(w)));
        break;
      case "notice":
        b(z, T(e(w).filter((_) => _.typ === "Test" || _.level === "notice" || _.level === "warning" || _.level === "critical")));
        break;
      case "warning":
        b(z, T(e(w).filter((_) => _.typ === "Test" || _.level === "warning" || _.level === "critical")));
        break;
      case "critical":
        b(z, T(e(w).filter((_) => _.typ === "Test" || _.level === "critical")));
        break;
    }
  });
  async function S() {
    await Wt("/auth/v1/events/test");
  }
  function k() {
    localStorage.setItem("eventLevel", e(B)), e(y) && e(y).readyState !== 2 && e(y).close(), b(y, T(new EventSource(`/auth/v1/events/stream?latest=${a}&level=${e(B).toLowerCase()}`))), e(y).onopen = () => {
      b(w, T([]));
    }, e(y).onerror = () => {
      console.error("SSE Events Stream closed");
    }, e(y).onmessage = (_) => {
      if (_.data) {
        let tt = JSON.parse(_.data);
        b(w, T([tt, ...e(w).slice(-499)]));
      }
    };
  }
  var c = Ze();
  let x;
  var A = u(c), I = u(A), C = u(I), Z = g(u(C), 2);
  Xt(Z, { get ariaLabel() {
    return l.events.eventLevel;
  }, options: Dt, borderless: true, get value() {
    return e(B);
  }, set value(_) {
    b(B, T(_));
  } }), d(C);
  var G = g(C, 2);
  ct(G, { level: 3, onclick: S, children: (_, tt) => {
    W();
    var it = q("Test");
    n(_, it);
  }, $$slots: { default: true } }), d(I);
  var $ = g(I, 2);
  Kt($, 21, () => e(z), (_) => _.id, (_, tt) => {
    Jt(_, { get event() {
      return e(tt);
    } });
  }), d($), d(A);
  var Q = g(A, 2);
  Re(Q, { get wide() {
    return e(t);
  } }), d(c), f((_) => x = Lt(c, 1, "svelte-zmipc9", null, x, _), [() => ({ wide: e(t), narrow: !e(t) })]), _t("innerWidth", (_) => b(i, T(_))), n(h, c), ht();
}
var De = P('<div class="noAdmin svelte-drznxl"><div><div class="text svelte-drznxl"><!></div> <!></div></div>'), Fe = P('<div class="noAdmin svelte-drznxl"><div><div class="text svelte-drznxl"><!></div> <!></div></div>'), Ge = P('<div class="events"><!></div>'), Ke = P('<div class="content svelte-drznxl"><!></div> <!>', 1), Je = P("<!> <!>", 1);
function Mr(h, r) {
  ut(r, true), Ot();
  let a = wt(), l = Gt("admin"), i = V(void 0), t = V(false), y = V(false), w = V(false);
  vt(() => {
    var _a;
    let c = l.get();
    c && (!!((_a = c == null ? void 0 : c.roles) == null ? void 0 : _a.includes("rauthy_admin")) ? (b(t, true), z()) : b(y, true));
  });
  async function z() {
    (await zt("/auth/v1/auth_check_admin")).status === 406 && b(w, true);
  }
  var B = ft();
  Ht((c) => {
    Et.title = "Rauthy Admin";
  });
  var R = rt(B);
  {
    var S = (c) => {
      var x = De(), A = u(x), I = u(A), C = u(I);
      It(C, () => a.error.noAdmin), d(I);
      var Z = g(I, 2);
      ct(Z, { onclick: () => window.location.href = "/auth/v1/account", children: (G, $) => {
        W();
        var Q = q();
        f(() => L(Q, a.common.account)), n(G, Q);
      }, $$slots: { default: true } }), d(A), d(x), n(c, x);
    }, k = (c, x) => {
      {
        var A = (C) => {
          var Z = Fe(), G = u(Z), $ = u(G), Q = u($);
          It(Q, () => a.error.needsAdminRole), d($);
          var _ = g($, 2);
          ct(_, { onclick: () => window.location.href = "/auth/v1/", children: (tt, it) => {
            W();
            var M = q();
            f(() => L(M, a.common.back)), n(tt, M);
          }, $$slots: { default: true } }), d(G), d(Z), n(C, Z);
        }, I = (C, Z) => {
          {
            var G = ($) => {
              var Q = Je(), _ = rt(Q);
              Ne(_, {});
              var tt = g(_, 2);
              qt(tt, { children: (it, M) => {
                var E = Ke(), N = rt(E), et = u(N);
                dt(et, () => r.children), d(N);
                var nt = g(N, 2);
                {
                  var K = (j) => {
                    var H = Ge(), U = u(H);
                    Ue(U, {}), d(H), n(j, H);
                  };
                  Y(nt, (j) => {
                    e(i) && e(i) > 1024 && j(K);
                  });
                }
                n(it, E);
              } }), n($, Q);
            };
            Y(C, ($) => {
              e(t) && $(G);
            }, Z);
          }
        };
        Y(c, (C) => {
          e(y) ? C(A) : C(I, false);
        }, x);
      }
    };
    Y(R, (c) => {
      e(w) ? c(S) : c(k, false);
    });
  }
  _t("innerWidth", (c) => b(i, T(c))), n(h, B), ht();
}
export {
  Mr as component
};
