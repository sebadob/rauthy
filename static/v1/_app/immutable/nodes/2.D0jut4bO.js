import { d as ce, a as n, t as b, n as N, e as T } from "../chunks/BH6NCLk-.js";
import { p as he, a5 as je, l as A, k as H, f as Y, j as t, a as me, c as h, r as d, t as p, a9 as ge, s as g, aa as se, a8 as Z, a7 as E, aj as Se, a6 as Te } from "../chunks/CvlvO1XB.js";
import { s as P, e as Me, h as Ee } from "../chunks/CTI4QPiR.js";
import { i as ee } from "../chunks/BUO_AUgz.js";
import { h as be } from "../chunks/i8Xqpu09.js";
import { s as ue } from "../chunks/-T201g_q.js";
import { p as B } from "../chunks/Wh68IIk2.js";
import { b as _e } from "../chunks/7lxiEjMQ.js";
import { M as He } from "../chunks/BbRujPvb.js";
import { a as ve, B as de } from "../chunks/DMkkW5Nn.js";
import { s as o, a as Be } from "../chunks/BMbqVy6X.js";
import { T as Ie } from "../chunks/C6vpM94V.js";
import { L as Ce } from "../chunks/BWe1X9CS.js";
import { p as Pe } from "../chunks/CRwGPA6d.js";
import { f as ze, b as Ve } from "../chunks/DYtiVhoA.js";
import { R as qe } from "../chunks/2DYY54Zs.js";
import { A as We } from "../chunks/Ckhed6J8.js";
import { p as v } from "../chunks/C6SR4G2t.js";
import { I as Ne } from "../chunks/Vi3uK7uO.js";
import { u as we, i as Oe } from "../chunks/D8mHI_K9.js";
import { U as Re, k as pe, i as Ze, D as $e } from "../chunks/B8wC3kJv.js";
import { T as Ue } from "../chunks/BmpEzKyJ.js";
import { u as De } from "../chunks/K02i8WAj.js";
import { e as Ge } from "../chunks/BpWRzPRQ.js";
import { E as Ke } from "../chunks/DM0MFlCa.js";
import { u as Fe } from "../chunks/BQ1-pLIs.js";
import { O as Je } from "../chunks/BPVdf4Rb.js";
var Qe = b('<div class="ver upd svelte-4tal6i"><a target="_blank" class="svelte-4tal6i"> </a></div>'), Xe = b('<div class="ver svelte-4tal6i"> </div>');
function Ye(c, r) {
  he(r, true);
  let a = H(void 0);
  je(() => {
    l();
  });
  async function l() {
    let _ = await ze("/auth/v1/version");
    _.body ? A(a, B(_.body)) : console.error(_.error);
  }
  var i = ce(), e = Y(i);
  {
    var y = (_) => {
      var z = ce(), I = Y(z);
      {
        var R = (k) => {
          var u = Qe(), x = h(u), M = h(x);
          d(x), d(u), p(() => {
            o(x, "href", t(a).latest_url), P(M, `v${t(a) ?? ""} \u26A0\uFE0F`);
          }), n(k, u);
        }, j = (k) => {
          var u = Xe(), x = h(u);
          d(u), p(() => P(x, `v${t(a).current ?? ""}`)), n(k, u);
        };
        ee(I, (k) => {
          t(a).update_available ? k(R) : k(j, false);
        });
      }
      n(_, z);
    };
    ee(e, (_) => {
      t(a) && _(y);
    });
  }
  n(c, i), me();
}
var et = b('<div class="compact svelte-7fczgn"><div class="iconCompact svelte-7fczgn"><!></div> <span class="svelte-7fczgn"><!></span></div>'), tt = b('<div class="wide svelte-7fczgn"><div class="iconWide svelte-7fczgn"><!></div> <div><!></div></div>');
function $(c, r) {
  const a = "/auth/v1/admin";
  let l = ge(() => r.compact ? "1.5rem" : "1.2rem"), i = ge(() => `${a}${r.route}${r.params}`);
  We(c, { get href() {
    return t(i);
  }, hideUnderline: true, get highlightIncludes() {
    return r.highlightIncludes;
  }, children: (e, y) => {
    var _ = ce(), z = Y(_);
    {
      var I = (j) => {
        var k = et(), u = h(k), x = h(u);
        ue(x, () => r.icon, () => t(l)), d(u);
        var M = g(u, 2), L = h(M);
        ue(L, () => r.children), d(M), d(k), n(j, k);
      }, R = (j) => {
        var k = tt(), u = h(k), x = h(u);
        ue(x, () => r.icon, () => t(l)), d(u);
        var M = g(u, 2), L = h(M);
        ue(L, () => r.children), d(M), d(k), p(() => ve(u, "width", t(l))), n(j, k);
      };
      ee(z, (j) => {
        r.compact ? j(I) : j(R, false);
      });
    }
    n(e, _);
  }, $$slots: { default: true } });
}
var rt = N('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"></path></svg>');
function ot(c, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = rt();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(c, e);
}
var at = N(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M19.5 12c0-1.232-.046-2.453-.138-3.662a4.006 4.006 0 0 0-3.7-3.7 48.678 48.678 0 0 0-7.324 0 4.006 4.006
            0 0 0-3.7 3.7c-.017.22-.032.441-.046.662M19.5 12l3-3m-3 3-3-3m-12 3c0 1.232.046 2.453.138 3.662a4.006 4.006
            0 0 0 3.7 3.7 48.656 48.656 0 0 0 7.324 0 4.006 4.006 0 0 0 3.7-3.7c.017-.22.032-.441.046-.662M4.5 12l3
            3m-3-3-3 3"></path></svg>`);
function it(c, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = at();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(c, e);
}
var nt = N(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M18 18.72a9.094 9.094 0 0 0 3.741-.479 3 3 0 0 0-4.682-2.72m.94 3.198.001.031c0
            .225-.012.447-.037.666A11.944 11.944 0 0 1 12 21c-2.17 0-4.207-.576-5.963-1.584A6.062 6.062 0 0 1 6
            18.719m12 0a5.971 5.971 0 0 0-.941-3.197m0 0A5.995 5.995 0 0 0 12 12.75a5.995 5.995 0 0 0-5.058 2.772m0
            0a3 3 0 0 0-4.681 2.72 8.986 8.986 0 0 0 3.74.477m.94-3.197a5.971 5.971 0 0 0-.94 3.197M15 6.75a3 3 0 1
            1-6 0 3 3 0 0 1 6 0Zm6 3a2.25 2.25 0 1 1-4.5 0 2.25 2.25 0 0 1 4.5 0Zm-13.5 0a2.25 2.25 0 1 1-4.5 0 2.25
            2.25 0 0 1 4.5 0Z"></path></svg>`);
function st(c, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = nt();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(c, e);
}
var lt = N('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z"></path></svg>');
function vt(c, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = lt();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(c, e);
}
var ct = N(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 21h16.5M4.5 3h15M5.25 3v18m13.5-18v18M9 6.75h1.5m-1.5 3h1.5m-1.5 3h1.5m3-6H15m-1.5 3H15m-1.5
            3H15M9 21v-3.375c0-.621.504-1.125 1.125-1.125h3.75c.621 0 1.125.504 1.125 1.125V21"></path></svg>`);
function dt(c, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = ct();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(c, e);
}
var ut = N('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75L11.25 15 15 9.75m-3-7.036A11.959 11.959 0 013.598 6 11.99 11.99 0 003 9.749c0 5.592 3.824 10.29 9 11.623 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.285z"></path></svg>');
function ht(c, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = ut();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(c, e);
}
var mt = N('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z"></path></svg>');
function ft(c, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = mt();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(c, e);
}
var pt = N(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M15 9h3.75M15 12h3.75M15 15h3.75M4.5 19.5h15a2.25 2.25 0 002.25-2.25V6.75A2.25 2.25 0 0019.5
            4.5h-15a2.25 2.25 0 00-2.25 2.25v10.5A2.25 2.25 0 004.5 19.5zm6-10.125a1.875 1.875 0 11-3.75 0
            1.875 1.875 0 013.75 0zm1.294 6.336a6.721 6.721 0 01-3.17.789 6.721 6.721 0 01-3.168-.789 3.376
            3.376 0 016.338 0z"></path></svg>`);
function gt(c, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = pt();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(c, e);
}
var _t = N(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 0 0 5.454-1.31A8.967 8.967 0 0 1 18 9.75V9A6 6 0 0 0 6 9v.75a8.967 8.967
            0 0 1-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 0 1-5.714 0m5.714 0a3 3 0 1 1-5.714
            0"></path></svg>`);
function wt(c, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = _t();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(c, e);
}
var kt = N('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M15.75 5.25a3 3 0 013 3m3 0a6 6 0 01-7.029 5.912c-.563-.097-1.159.026-1.563.43L10.5 17.25H8.25v2.25H6v2.25H2.25v-2.818c0-.597.237-1.17.659-1.591l6.499-6.499c.404-.404.527-1 .43-1.563A6 6 0 1121.75 8.25z"></path></svg>');
function yt(c, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = kt();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(c, e);
}
var xt = N(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M2.25 15a4.5 4.5 0 0 0 4.5 4.5H18a3.75 3.75 0 0 0 1.332-7.257 3 3 0 0 0-3.758-3.848 5.25 5.25 0 0
            0-10.233 2.33A4.502 4.502 0 0 0 2.25 15Z"></path></svg>`);
function At(c, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = xt();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(c, e);
}
var Mt = N('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M11.42 15.17L17.25 21A2.652 2.652 0 0021 17.25l-5.877-5.877M11.42 15.17l2.496-3.03c.317-.384.74-.626 1.208-.766M11.42 15.17l-4.655 5.653a2.548 2.548 0 11-3.586-3.586l6.837-5.63m5.108-.233c.55-.164 1.163-.188 1.743-.14a4.5 4.5 0 004.486-6.336l-3.276 3.277a3.004 3.004 0 01-2.25-2.25l3.276-3.276a4.5 4.5 0 00-6.336 4.486c.091 1.076-.071 2.264-.904 2.95l-.102.085m-1.745 1.437L5.909 7.5H4.5L2.25 3.75l1.5-1.5L7.5 4.5v1.409l4.26 4.26m-1.745 1.437l1.745-1.437m6.615 8.206L15.75 15.75M4.867 19.125h.008v.008h-.008v-.008z"></path></svg>');
function bt(c, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = Mt();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(c, e);
}
var It = N('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M12 6.042A8.967 8.967 0 006 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 016 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 016-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0018 18a8.967 8.967 0 00-6 2.292m0-14.25v14.25"></path></svg>');
function Ct(c, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = It();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(c, e);
}
var Pt = N(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75 11.25 15 15 9.75M21 12c0 1.268-.63 2.39-1.593 3.068a3.745 3.745 0 0 1-1.043 3.296 3.745
            3.745 0 0 1-3.296 1.043A3.745 3.745 0 0 1 12 21c-1.268 0-2.39-.63-3.068-1.593a3.746 3.746 0 0
            1-3.296-1.043 3.745 3.745 0 0 1-1.043-3.296A3.745 3.745 0 0 1 3 12c0-1.268.63-2.39 1.593-3.068a3.745
            3.745 0 0 1 1.043-3.296 3.746 3.746 0 0 1 3.296-1.043A3.746 3.746 0 0 1 12 3c1.268 0 2.39.63 3.068
            1.593a3.746 3.746 0 0 1 3.296 1.043 3.746 3.746 0 0 1 1.043 3.296A3.745 3.745 0 0 1 21 12Z"></path></svg>`);
function Bt(c, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = Pt();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(c, e);
}
var zt = N('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15M12 9l-3 3m0 0l3 3m-3-3h12.75"></path></svg>');
function Lt(c, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = zt();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(c, e);
}
var jt = b('<span class="inner svelte-8anrni"><!></span>'), St = b('<span class="inner svelte-8anrni"><!></span>'), Tt = b("<div><!></div>"), Et = b('<div class="logo svelte-8anrni"><!></div> <div class="menu svelte-8anrni"><!> <!> <!> <!> <!> <!> <!> <!> <!> <!> <!> <!> <!></div>', 1), Ht = b('<div class="logout svelte-8anrni"><!></div>'), Vt = b('<!> <div class="theme svelte-8anrni"><!></div> <!>', 1), qt = b('<div class="flex gap-05"><!> <!> <!></div> <div class="version svelte-8anrni"><!></div>', 1), Wt = b('<div class="bottom svelte-8anrni"><!></div>'), Nt = b('<nav id="mainNav" class="svelte-8anrni"><div class="svelte-8anrni"><div><div class="relative"><div class="absolute flex space-between navmod svelte-8anrni"><div><!></div> <!></div></div> <!></div> <!></div></nav>');
function Ot(c, r) {
  he(r, true);
  let a = we(), l = H(false), i = H(""), e = H(false), y = H(true), _ = H(void 0);
  se(() => {
    A(i, B(Pe.url.search));
  }), se(() => {
    t(_) && t(_) < 600 && Pe.url && A(y, true);
  }), se(() => {
    A(y, B(t(_) && t(_) < 800 || false));
  }), se(() => {
    A(e, B(t(_) && t(_) < 1280 || false));
  });
  function z() {
    A(l, true);
  }
  function I() {
    setTimeout(() => {
      A(l, false);
    }, 1e3);
  }
  function R() {
    A(y, !t(y));
  }
  function j() {
    A(e, !t(e));
  }
  var k = Nt(), u = h(k), x = h(u), M = h(x), L = h(M), Q = h(L), U = h(Q);
  de(U, { ariaControls: "mainNav", invisible: true, onclick: R, children: (C, S) => {
    var K = jt(), re = h(K);
    ot(re, {}), d(K), n(C, K);
  }, $$slots: { default: true } }), d(Q);
  var D = g(Q, 2);
  {
    var te = (C) => {
      var S = Tt(), K = h(S);
      de(K, { invisible: true, onclick: j, children: (re, ie) => {
        var F = St(), V = h(F);
        it(V, {}), d(F), n(re, F);
      }, $$slots: { default: true } }), d(S), n(C, S);
    };
    ee(D, (C) => {
      t(y) || C(te);
    });
  }
  d(L), d(M);
  var G = g(M, 2);
  {
    var w = (C) => {
      var S = Et(), K = Y(S), re = h(K);
      const ie = ge(() => t(e) ? "3rem" : "7rem");
      qe(re, { get width() {
        return t(ie);
      } }), d(K);
      var F = g(K, 2), V = h(F);
      $(V, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/users", icon: (s, m = Z) => {
        ft(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.users)), n(s, f);
      } });
      var O = g(V, 2);
      $(O, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/attributes", icon: (s, m = Z) => {
        vt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.attributes)), n(s, f);
      } });
      var q = g(O, 2);
      $(q, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/clients", icon: (s, m = Z) => {
        dt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.clients)), n(s, f);
      } });
      var W = g(q, 2);
      $(W, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/roles", icon: (s, m = Z) => {
        Bt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.roles)), n(s, f);
      } });
      var oe = g(W, 2);
      $(oe, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/groups", icon: (s, m = Z) => {
        st(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.groups)), n(s, f);
      } });
      var ne = g(oe, 2);
      $(ne, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/scopes", icon: (s, m = Z) => {
        gt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.scopes)), n(s, f);
      } });
      var le = g(ne, 2);
      $(le, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/sessions", icon: (s, m = Z) => {
        ht(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.sessions)), n(s, f);
      } });
      var fe = g(le, 2);
      $(fe, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/events", icon: (s, m = Z) => {
        wt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.events)), n(s, f);
      } });
      var ke = g(fe, 2);
      $(ke, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/blacklist", icon: (s, m = Z) => {
        Ne(s, { get width() {
          return m();
        }, color: "currentColor" });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.blacklist)), n(s, f);
      } });
      var ye = g(ke, 2);
      $(ye, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/api_keys", icon: (s, m = Z) => {
        yt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.apiKeys)), n(s, f);
      } });
      var xe = g(ye, 2);
      $(xe, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/providers", icon: (s, m = Z) => {
        At(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.providers)), n(s, f);
      } });
      var Ae = g(xe, 2);
      $(Ae, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/config/policy", highlightIncludes: "/config/", icon: (s, m = Z) => {
        bt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.config)), n(s, f);
      } });
      var Le = g(Ae, 2);
      $(Le, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/docs", icon: (s, m = Z) => {
        Ct(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.docs)), n(s, f);
      } }), d(F), p(() => ve(F, "margin-top", t(e) ? ".5rem" : "1.5rem")), n(C, S);
    };
    ee(G, (C) => {
      t(y) || C(w);
    });
  }
  d(x);
  var X = g(x, 2);
  {
    var ae = (C) => {
      var S = Wt();
      {
        const F = (V) => {
          var O = Ht(), q = h(O);
          de(q, { invisible: true, onclick: () => Re(), children: (W, oe) => {
            Ue(W, { text: "Logout", children: (ne, le) => {
              Lt(ne, {});
            }, $$slots: { default: true } });
          }, $$slots: { default: true } }), d(O), n(V, O);
        };
        var K = h(S);
        {
          var re = (V) => {
            var O = Vt(), q = Y(O);
            F(q);
            var W = g(q, 2), oe = h(W);
            Ie(oe, {}), d(W);
            var ne = g(W, 2);
            Ce(ne, { openTop: true }), n(V, O);
          }, ie = (V) => {
            var O = qt(), q = Y(O), W = h(q);
            Ie(W, {});
            var oe = g(W, 2);
            Ce(oe, { openTop: true });
            var ne = g(oe, 2);
            F(ne), d(q);
            var le = g(q, 2), fe = h(le);
            Ye(fe, {}), d(le), n(V, O);
          };
          ee(K, (V) => {
            t(e) ? V(re) : V(ie, false);
          });
        }
        d(S);
      }
      n(C, S);
    };
    ee(X, (C) => {
      t(y) || C(ae);
    });
  }
  d(u), d(k), p(() => {
    o(k, "aria-hidden", t(y)), o(k, "data-compact", t(e)), o(L, "aria-hidden", !t(y) && !t(l));
  }), _e("innerWidth", (C) => A(_, B(C))), Me("mouseenter", k, z), Me("mouseleave", k, I), n(c, k), me();
}
var Rt = b('<fieldset class="svelte-1vtgp8q"><legend class="font-label svelte-1vtgp8q"> </legend> <ul><li class="svelte-1vtgp8q">Info</li> <li class="svelte-1vtgp8q">Notice</li> <li class="svelte-1vtgp8q">Warning</li> <li class="svelte-1vtgp8q">Critical</li></ul></fieldset>');
function Zt(c, r) {
  he(r, true);
  let a = Fe();
  var l = Rt(), i = h(l), e = h(i, true);
  d(i);
  var y = g(i, 2);
  let _;
  var z = h(y), I = g(z, 2), R = g(I, 2), j = g(R, 2);
  d(y), d(l), p((k, u, x, M) => {
    P(e, a.common.legend), _ = Be(y, 1, "svelte-1vtgp8q", null, _, { wide: r.wide }), ve(z, "border-color", k), ve(I, "border-color", u), ve(R, "border-color", x), ve(j, "border-color", M);
  }, [() => pe("info"), () => pe("notice"), () => pe("warning"), () => pe("critical")]), n(c, l), me();
}
var $t = b('<div role="none" id="events"><div class="upper"><div class="header svelte-zmipc9"><div class="flex gap-10"><b>Events</b> <!></div> <!></div> <div class="data svelte-zmipc9"></div></div> <!></div>');
function Ut(c, r) {
  he(r, true);
  const a = 50;
  let l = we(), i = H(void 0), e = H(false), y = H(void 0), _ = H(B([])), z = H(B([])), I = H(B(Ze() && localStorage.getItem("eventLevel") || "info")), R = "";
  Se(() => {
    var _a;
    (_a = t(y)) == null ? void 0 : _a.close();
  }), se(() => {
    t(i) && A(e, t(i) > 1680);
  }), se(() => {
    t(I) !== R && (R = t(I), k());
  }), se(() => {
    if (t(_)) switch (t(I)) {
      case "info":
        A(z, B(t(_)));
        break;
      case "notice":
        A(z, B(t(_).filter((w) => w.typ === "Test" || w.level === "notice" || w.level === "warning" || w.level === "critical")));
        break;
      case "warning":
        A(z, B(t(_).filter((w) => w.typ === "Test" || w.level === "warning" || w.level === "critical")));
        break;
      case "critical":
        A(z, B(t(_).filter((w) => w.typ === "Test" || w.level === "critical")));
        break;
    }
  });
  async function j() {
    await Ve("/auth/v1/events/test");
  }
  function k() {
    var _a, _b;
    localStorage.setItem("eventLevel", t(I)), ((_a = t(y)) == null ? void 0 : _a.readyState) !== 2 && ((_b = t(y)) == null ? void 0 : _b.close()), A(y, B(new EventSource(`/auth/v1/events/stream?latest=${a}&level=${t(I).toLowerCase()}`))), t(y).onopen = () => {
      A(_, B([]));
    }, t(y).onerror = () => {
      console.error("SSE Events Stream closed");
    }, t(y).onmessage = (w) => {
      if (w.data) {
        let X = JSON.parse(w.data);
        A(_, B([X, ...t(_).slice(-499)]));
      }
    };
  }
  var u = $t();
  let x;
  var M = h(u), L = h(M), Q = h(L), U = g(h(Q), 2);
  Je(U, { get ariaLabel() {
    return l.events.eventLevel;
  }, options: $e, borderless: true, get value() {
    return t(I);
  }, set value(w) {
    A(I, B(w));
  } }), d(Q);
  var D = g(Q, 2);
  de(D, { level: 3, onclick: j, children: (w, X) => {
    E();
    var ae = T("Test");
    n(w, ae);
  }, $$slots: { default: true } }), d(L);
  var te = g(L, 2);
  Ge(te, 21, () => t(z), (w) => w.id, (w, X) => {
    Ke(w, { get event() {
      return t(X);
    } });
  }), d(te), d(M);
  var G = g(M, 2);
  Zt(G, { get wide() {
    return t(e);
  } }), d(u), p(() => x = Be(u, 1, "svelte-zmipc9", null, x, { wide: t(e), narrow: !t(e) })), _e("innerWidth", (w) => A(i, B(w))), n(c, u), me();
}
var Dt = b('<div class="noAdmin svelte-drznxl"><div><div class="text svelte-drznxl"><!></div> <!></div></div>'), Gt = b('<div class="noAdmin svelte-drznxl"><div><div class="text svelte-drznxl"><!></div> <!></div></div>'), Kt = b('<div class="events"><!></div>'), Ft = b('<div class="content svelte-drznxl"><!></div> <!>', 1), Jt = b("<!> <!>", 1);
function Mr(c, r) {
  he(r, true), Oe();
  let a = we(), l = De("admin"), i = H(void 0), e = H(false), y = H(false), _ = H(false);
  se(() => {
    var _a;
    let u = l.get();
    u && (!!((_a = u == null ? void 0 : u.roles) == null ? void 0 : _a.includes("rauthy_admin")) ? (A(e, true), z()) : A(y, true));
  });
  async function z() {
    (await ze("/auth/v1/auth_check_admin")).status === 406 && A(_, true);
  }
  var I = ce();
  Ee((u) => {
    Te.title = "Rauthy Admin";
  });
  var R = Y(I);
  {
    var j = (u) => {
      var x = Dt(), M = h(x), L = h(M), Q = h(L);
      be(Q, () => a.error.noAdmin), d(L);
      var U = g(L, 2);
      de(U, { onclick: () => window.location.href = "/auth/v1/account", children: (D, te) => {
        E();
        var G = T();
        p(() => P(G, a.common.account)), n(D, G);
      }, $$slots: { default: true } }), d(M), d(x), n(u, x);
    }, k = (u) => {
      var x = ce(), M = Y(x);
      {
        var L = (U) => {
          var D = Gt(), te = h(D), G = h(te), w = h(G);
          be(w, () => a.error.needsAdminRole), d(G);
          var X = g(G, 2);
          de(X, { onclick: () => window.location.href = "/auth/v1/", children: (ae, C) => {
            E();
            var S = T();
            p(() => P(S, a.common.back)), n(ae, S);
          }, $$slots: { default: true } }), d(te), d(D), n(U, D);
        }, Q = (U) => {
          var D = ce(), te = Y(D);
          {
            var G = (w) => {
              var X = Jt(), ae = Y(X);
              Ot(ae, {});
              var C = g(ae, 2);
              He(C, { children: (S, K) => {
                var re = Ft(), ie = Y(re), F = h(ie);
                ue(F, () => r.children), d(ie);
                var V = g(ie, 2);
                {
                  var O = (q) => {
                    var W = Kt(), oe = h(W);
                    Ut(oe, {}), d(W), n(q, W);
                  };
                  ee(V, (q) => {
                    t(i) && t(i) > 1024 && q(O);
                  });
                }
                n(S, re);
              } }), n(w, X);
            };
            ee(te, (w) => {
              t(e) && w(G);
            }, true);
          }
          n(U, D);
        };
        ee(M, (U) => {
          t(y) ? U(L) : U(Q, false);
        }, true);
      }
      n(u, x);
    };
    ee(R, (u) => {
      t(_) ? u(j) : u(k, false);
    });
  }
  _e("innerWidth", (u) => A(i, B(u))), n(c, I), me();
}
export {
  Mr as component
};
