import { d as ce, a as n, t as M, n as N, e as T } from "../chunks/BH6NCLk-.js";
import { p as he, a5 as Se, l as A, k as H, f as Y, j as t, a as me, c as u, r as c, t as p, a9 as ge, s as g, aa as se, a8 as R, a7 as E, aj as je, a6 as Te } from "../chunks/CvlvO1XB.js";
import { s as P, e as be, h as Ee } from "../chunks/CTI4QPiR.js";
import { i as ee } from "../chunks/BUO_AUgz.js";
import { h as Me } from "../chunks/i8Xqpu09.js";
import { s as ue } from "../chunks/-T201g_q.js";
import { p as B } from "../chunks/Wh68IIk2.js";
import { b as _e } from "../chunks/7lxiEjMQ.js";
import { M as He } from "../chunks/BbRujPvb.js";
import { a as ve, B as de } from "../chunks/CqH8LXO-.js";
import { s as o, a as Be } from "../chunks/BMbqVy6X.js";
import { T as Ie } from "../chunks/CXfCraUq.js";
import { L as Ce } from "../chunks/CyncXH1a.js";
import { p as Pe } from "../chunks/CWJbg64V.js";
import { f as ze, b as qe } from "../chunks/BO1A6s0c.js";
import { R as We } from "../chunks/CufZd29Z.js";
import { A as Ve } from "../chunks/tK0ui9-v.js";
import { p as v } from "../chunks/C6SR4G2t.js";
import { I as $e } from "../chunks/Vi3uK7uO.js";
import { u as we, i as Ne } from "../chunks/D8mHI_K9.js";
import { I as Oe } from "../chunks/CbjsfN_U.js";
import { S as Re, k as pe, i as Ze, B as Ue } from "../chunks/B21bTIl7.js";
import { T as De } from "../chunks/ChMSzdNf.js";
import { u as Ge } from "../chunks/BCsIV4gx.js";
import { e as Ke } from "../chunks/BpWRzPRQ.js";
import { E as Fe } from "../chunks/B5jqu_oq.js";
import { u as Je } from "../chunks/CUqQZdNU.js";
import { O as Qe } from "../chunks/CdqYxeyI.js";
var Xe = M('<div class="ver upd svelte-4tal6i"><a target="_blank" class="svelte-4tal6i"> </a></div>'), Ye = M('<div class="ver svelte-4tal6i"> </div>');
function et(h, r) {
  he(r, true);
  let a = H(void 0);
  Se(() => {
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
        var O = (k) => {
          var d = Xe(), x = u(d), b = u(x);
          c(x), c(d), p(() => {
            o(x, "href", t(a).latest_url), P(b, `v${t(a) ?? ""} \u26A0\uFE0F`);
          }), n(k, d);
        }, S = (k) => {
          var d = Ye(), x = u(d);
          c(d), p(() => P(x, `v${t(a).current ?? ""}`)), n(k, d);
        };
        ee(I, (k) => {
          t(a).update_available ? k(O) : k(S, false);
        });
      }
      n(_, z);
    };
    ee(e, (_) => {
      t(a) && _(y);
    });
  }
  n(h, i), me();
}
var tt = M('<div class="compact svelte-7fczgn"><div class="iconCompact svelte-7fczgn"><!></div> <span class="svelte-7fczgn"><!></span></div>'), rt = M('<div class="wide svelte-7fczgn"><div class="iconWide svelte-7fczgn"><!></div> <div><!></div></div>');
function Z(h, r) {
  const a = "/auth/v1/admin";
  let l = ge(() => r.compact ? "1.5rem" : "1.2rem"), i = ge(() => `${a}${r.route}${r.params}`);
  Ve(h, { get href() {
    return t(i);
  }, hideUnderline: true, get highlightIncludes() {
    return r.highlightIncludes;
  }, children: (e, y) => {
    var _ = ce(), z = Y(_);
    {
      var I = (S) => {
        var k = tt(), d = u(k), x = u(d);
        ue(x, () => r.icon, () => t(l)), c(d);
        var b = g(d, 2), L = u(b);
        ue(L, () => r.children), c(b), c(k), n(S, k);
      }, O = (S) => {
        var k = rt(), d = u(k), x = u(d);
        ue(x, () => r.icon, () => t(l)), c(d);
        var b = g(d, 2), L = u(b);
        ue(L, () => r.children), c(b), c(k), p(() => ve(d, "width", t(l))), n(S, k);
      };
      ee(z, (S) => {
        r.compact ? S(I) : S(O, false);
      });
    }
    n(e, _);
  }, $$slots: { default: true } });
}
var ot = N('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"></path></svg>');
function at(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = ot();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var it = N(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M19.5 12c0-1.232-.046-2.453-.138-3.662a4.006 4.006 0 0 0-3.7-3.7 48.678 48.678 0 0 0-7.324 0 4.006 4.006
            0 0 0-3.7 3.7c-.017.22-.032.441-.046.662M19.5 12l3-3m-3 3-3-3m-12 3c0 1.232.046 2.453.138 3.662a4.006 4.006
            0 0 0 3.7 3.7 48.656 48.656 0 0 0 7.324 0 4.006 4.006 0 0 0 3.7-3.7c.017-.22.032-.441.046-.662M4.5 12l3
            3m-3-3-3 3"></path></svg>`);
function nt(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = it();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var st = N(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M18 18.72a9.094 9.094 0 0 0 3.741-.479 3 3 0 0 0-4.682-2.72m.94 3.198.001.031c0
            .225-.012.447-.037.666A11.944 11.944 0 0 1 12 21c-2.17 0-4.207-.576-5.963-1.584A6.062 6.062 0 0 1 6
            18.719m12 0a5.971 5.971 0 0 0-.941-3.197m0 0A5.995 5.995 0 0 0 12 12.75a5.995 5.995 0 0 0-5.058 2.772m0
            0a3 3 0 0 0-4.681 2.72 8.986 8.986 0 0 0 3.74.477m.94-3.197a5.971 5.971 0 0 0-.94 3.197M15 6.75a3 3 0 1
            1-6 0 3 3 0 0 1 6 0Zm6 3a2.25 2.25 0 1 1-4.5 0 2.25 2.25 0 0 1 4.5 0Zm-13.5 0a2.25 2.25 0 1 1-4.5 0 2.25
            2.25 0 0 1 4.5 0Z"></path></svg>`);
function lt(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = st();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var vt = N('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z"></path></svg>');
function ct(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = vt();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var dt = N(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 21h16.5M4.5 3h15M5.25 3v18m13.5-18v18M9 6.75h1.5m-1.5 3h1.5m-1.5 3h1.5m3-6H15m-1.5 3H15m-1.5
            3H15M9 21v-3.375c0-.621.504-1.125 1.125-1.125h3.75c.621 0 1.125.504 1.125 1.125V21"></path></svg>`);
function ut(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = dt();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var ht = N('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75L11.25 15 15 9.75m-3-7.036A11.959 11.959 0 013.598 6 11.99 11.99 0 003 9.749c0 5.592 3.824 10.29 9 11.623 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.285z"></path></svg>');
function mt(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = ht();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var ft = N('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z"></path></svg>');
function pt(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = ft();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var gt = N(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M15 9h3.75M15 12h3.75M15 15h3.75M4.5 19.5h15a2.25 2.25 0 002.25-2.25V6.75A2.25 2.25 0 0019.5
            4.5h-15a2.25 2.25 0 00-2.25 2.25v10.5A2.25 2.25 0 004.5 19.5zm6-10.125a1.875 1.875 0 11-3.75 0
            1.875 1.875 0 013.75 0zm1.294 6.336a6.721 6.721 0 01-3.17.789 6.721 6.721 0 01-3.168-.789 3.376
            3.376 0 016.338 0z"></path></svg>`);
function _t(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = gt();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var wt = N(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 0 0 5.454-1.31A8.967 8.967 0 0 1 18 9.75V9A6 6 0 0 0 6 9v.75a8.967 8.967
            0 0 1-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 0 1-5.714 0m5.714 0a3 3 0 1 1-5.714
            0"></path></svg>`);
function kt(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = wt();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var yt = N('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M15.75 5.25a3 3 0 013 3m3 0a6 6 0 01-7.029 5.912c-.563-.097-1.159.026-1.563.43L10.5 17.25H8.25v2.25H6v2.25H2.25v-2.818c0-.597.237-1.17.659-1.591l6.499-6.499c.404-.404.527-1 .43-1.563A6 6 0 1121.75 8.25z"></path></svg>');
function xt(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = yt();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var At = N(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M2.25 15a4.5 4.5 0 0 0 4.5 4.5H18a3.75 3.75 0 0 0 1.332-7.257 3 3 0 0 0-3.758-3.848 5.25 5.25 0 0
            0-10.233 2.33A4.502 4.502 0 0 0 2.25 15Z"></path></svg>`);
function bt(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = At();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var Mt = N('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M11.42 15.17L17.25 21A2.652 2.652 0 0021 17.25l-5.877-5.877M11.42 15.17l2.496-3.03c.317-.384.74-.626 1.208-.766M11.42 15.17l-4.655 5.653a2.548 2.548 0 11-3.586-3.586l6.837-5.63m5.108-.233c.55-.164 1.163-.188 1.743-.14a4.5 4.5 0 004.486-6.336l-3.276 3.277a3.004 3.004 0 01-2.25-2.25l3.276-3.276a4.5 4.5 0 00-6.336 4.486c.091 1.076-.071 2.264-.904 2.95l-.102.085m-1.745 1.437L5.909 7.5H4.5L2.25 3.75l1.5-1.5L7.5 4.5v1.409l4.26 4.26m-1.745 1.437l1.745-1.437m6.615 8.206L15.75 15.75M4.867 19.125h.008v.008h-.008v-.008z"></path></svg>');
function It(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = Mt();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var Ct = N('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M12 6.042A8.967 8.967 0 006 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 016 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 016-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0018 18a8.967 8.967 0 00-6 2.292m0-14.25v14.25"></path></svg>');
function Pt(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = Ct();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var Bt = N(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75 11.25 15 15 9.75M21 12c0 1.268-.63 2.39-1.593 3.068a3.745 3.745 0 0 1-1.043 3.296 3.745
            3.745 0 0 1-3.296 1.043A3.745 3.745 0 0 1 12 21c-1.268 0-2.39-.63-3.068-1.593a3.746 3.746 0 0
            1-3.296-1.043 3.745 3.745 0 0 1-1.043-3.296A3.745 3.745 0 0 1 3 12c0-1.268.63-2.39 1.593-3.068a3.745
            3.745 0 0 1 1.043-3.296 3.746 3.746 0 0 1 3.296-1.043A3.746 3.746 0 0 1 12 3c1.268 0 2.39.63 3.068
            1.593a3.746 3.746 0 0 1 3.296 1.043 3.746 3.746 0 0 1 1.043 3.296A3.745 3.745 0 0 1 21 12Z"></path></svg>`);
function zt(h, r) {
  let a = v(r, "opacity", 3, 0.9), l = v(r, "width", 3, "1.5rem"), i = v(r, "color", 3, "currentColor");
  var e = Bt();
  o(e, "stroke-width", 2), p(() => {
    o(e, "stroke", i()), o(e, "width", l()), o(e, "opacity", a());
  }), n(h, e);
}
var Lt = M('<span class="inner svelte-8anrni"><!></span>'), St = M('<span class="inner svelte-8anrni"><!></span>'), jt = M("<div><!></div>"), Tt = M('<div class="logo svelte-8anrni"><!></div> <div class="menu svelte-8anrni"><!> <!> <!> <!> <!> <!> <!> <!> <!> <!> <!> <!> <!></div>', 1), Et = M('<div class="logout svelte-8anrni"><!></div>'), Ht = M('<!> <div class="theme svelte-8anrni"><!></div> <!>', 1), qt = M('<div class="flex gap-05"><!> <!> <!></div> <div class="version svelte-8anrni"><!></div>', 1), Wt = M('<div class="bottom svelte-8anrni"><!></div>'), Vt = M('<nav id="mainNav" class="svelte-8anrni"><div class="svelte-8anrni"><div><div class="relative"><div class="absolute flex space-between navmod svelte-8anrni"><div><!></div> <!></div></div> <!></div> <!></div></nav>');
function $t(h, r) {
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
  function O() {
    A(y, !t(y));
  }
  function S() {
    A(e, !t(e));
  }
  var k = Vt(), d = u(k), x = u(d), b = u(x), L = u(b), X = u(L), U = u(X);
  de(U, { ariaControls: "mainNav", invisible: true, onclick: O, children: (C, j) => {
    var F = Lt(), re = u(F);
    at(re, {}), c(F), n(C, F);
  }, $$slots: { default: true } }), c(X);
  var D = g(X, 2);
  {
    var te = (C) => {
      var j = jt(), F = u(j);
      de(F, { invisible: true, onclick: S, children: (re, ie) => {
        var J = St(), q = u(J);
        nt(q, {}), c(J), n(re, J);
      }, $$slots: { default: true } }), c(j), n(C, j);
    };
    ee(D, (C) => {
      t(y) || C(te);
    });
  }
  c(L), c(b);
  var G = g(b, 2);
  {
    var w = (C) => {
      var j = Tt(), F = Y(j), re = u(F);
      const ie = ge(() => t(e) ? "3rem" : "7rem");
      We(re, { get width() {
        return t(ie);
      } }), c(F);
      var J = g(F, 2), q = u(J);
      Z(q, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/users", icon: (s, m = R) => {
        pt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.users)), n(s, f);
      } });
      var $ = g(q, 2);
      Z($, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/clients", icon: (s, m = R) => {
        ut(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.clients)), n(s, f);
      } });
      var W = g($, 2);
      Z(W, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/roles", icon: (s, m = R) => {
        zt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.roles)), n(s, f);
      } });
      var V = g(W, 2);
      Z(V, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/groups", icon: (s, m = R) => {
        lt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.groups)), n(s, f);
      } });
      var oe = g(V, 2);
      Z(oe, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/attributes", icon: (s, m = R) => {
        ct(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.attributes)), n(s, f);
      } });
      var ne = g(oe, 2);
      Z(ne, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/scopes", icon: (s, m = R) => {
        _t(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.scopes)), n(s, f);
      } });
      var le = g(ne, 2);
      Z(le, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/sessions", icon: (s, m = R) => {
        mt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.sessions)), n(s, f);
      } });
      var fe = g(le, 2);
      Z(fe, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/events", icon: (s, m = R) => {
        kt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.events)), n(s, f);
      } });
      var ke = g(fe, 2);
      Z(ke, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/blacklist", icon: (s, m = R) => {
        $e(s, { get width() {
          return m();
        }, color: "currentColor" });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.blacklist)), n(s, f);
      } });
      var ye = g(ke, 2);
      Z(ye, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/api_keys", icon: (s, m = R) => {
        xt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.apiKeys)), n(s, f);
      } });
      var xe = g(ye, 2);
      Z(xe, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/providers", icon: (s, m = R) => {
        bt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.providers)), n(s, f);
      } });
      var Ae = g(xe, 2);
      Z(Ae, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/config/policy", highlightIncludes: "/config/", icon: (s, m = R) => {
        It(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.config)), n(s, f);
      } });
      var Le = g(Ae, 2);
      Z(Le, { get compact() {
        return t(e);
      }, get params() {
        return t(i);
      }, route: "/docs", icon: (s, m = R) => {
        Pt(s, { get width() {
          return m();
        } });
      }, children: (s, m) => {
        E();
        var f = T();
        p(() => P(f, a.nav.docs)), n(s, f);
      } }), c(J), p(() => ve(J, "margin-top", t(e) ? ".5rem" : "1.5rem")), n(C, j);
    };
    ee(G, (C) => {
      t(y) || C(w);
    });
  }
  c(x);
  var K = g(x, 2);
  {
    var ae = (C) => {
      var j = Wt();
      {
        const J = (q) => {
          var $ = Et(), W = u($);
          de(W, { invisible: true, onclick: () => Re(), children: (V, oe) => {
            De(V, { text: "Logout", children: (ne, le) => {
              Oe(ne, {});
            }, $$slots: { default: true } });
          }, $$slots: { default: true } }), c($), n(q, $);
        };
        var F = u(j);
        {
          var re = (q) => {
            var $ = Ht(), W = Y($);
            J(W);
            var V = g(W, 2), oe = u(V);
            Ie(oe, {}), c(V);
            var ne = g(V, 2);
            Ce(ne, { openTop: true }), n(q, $);
          }, ie = (q) => {
            var $ = qt(), W = Y($), V = u(W);
            Ie(V, {});
            var oe = g(V, 2);
            Ce(oe, { openTop: true });
            var ne = g(oe, 2);
            J(ne), c(W);
            var le = g(W, 2), fe = u(le);
            et(fe, {}), c(le), n(q, $);
          };
          ee(F, (q) => {
            t(e) ? q(re) : q(ie, false);
          });
        }
        c(j);
      }
      n(C, j);
    };
    ee(K, (C) => {
      t(y) || C(ae);
    });
  }
  c(d), c(k), p(() => {
    o(k, "aria-hidden", t(y)), o(k, "data-compact", t(e)), o(L, "aria-hidden", !t(y) && !t(l));
  }), _e("innerWidth", (C) => A(_, B(C))), be("mouseenter", k, z), be("mouseleave", k, I), n(h, k), me();
}
var Nt = M('<fieldset class="svelte-1vtgp8q"><legend class="font-label svelte-1vtgp8q"> </legend> <ul><li class="svelte-1vtgp8q">Info</li> <li class="svelte-1vtgp8q">Notice</li> <li class="svelte-1vtgp8q">Warning</li> <li class="svelte-1vtgp8q">Critical</li></ul></fieldset>');
function Ot(h, r) {
  he(r, true);
  let a = Je();
  var l = Nt(), i = u(l), e = u(i, true);
  c(i);
  var y = g(i, 2);
  let _;
  var z = u(y), I = g(z, 2), O = g(I, 2), S = g(O, 2);
  c(y), c(l), p((k, d, x, b) => {
    P(e, a.common.legend), _ = Be(y, 1, "svelte-1vtgp8q", null, _, { wide: r.wide }), ve(z, "border-color", k), ve(I, "border-color", d), ve(O, "border-color", x), ve(S, "border-color", b);
  }, [() => pe("info"), () => pe("notice"), () => pe("warning"), () => pe("critical")]), n(h, l), me();
}
var Rt = M('<div role="none" id="events"><div class="upper"><div class="header svelte-zmipc9"><div class="flex gap-10"><b>Events</b> <!></div> <!></div> <div class="data svelte-zmipc9"></div></div> <!></div>');
function Zt(h, r) {
  he(r, true);
  const a = 50;
  let l = we(), i = H(void 0), e = H(false), y = H(void 0), _ = H(B([])), z = H(B([])), I = H(B(Ze() && localStorage.getItem("eventLevel") || "info")), O = "";
  je(() => {
    var _a;
    (_a = t(y)) == null ? void 0 : _a.close();
  }), se(() => {
    t(i) && A(e, t(i) > 1680);
  }), se(() => {
    t(I) !== O && (O = t(I), k());
  }), se(() => {
    switch (t(I)) {
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
  async function S() {
    await qe("/auth/v1/events/test");
  }
  function k() {
    var _a, _b;
    localStorage.setItem("eventLevel", t(I)), ((_a = t(y)) == null ? void 0 : _a.readyState) !== 2 && ((_b = t(y)) == null ? void 0 : _b.close()), A(y, B(new EventSource(`/auth/v1/events/stream?latest=${a}&level=${t(I).toLowerCase()}`))), t(y).onopen = () => {
      A(_, B([]));
    }, t(y).onerror = () => {
      console.error("SSE Events Stream closed");
    }, t(y).onmessage = (w) => {
      if (w.data) {
        let K = JSON.parse(w.data);
        console.log("event", K.id), A(_, B([K, ...t(_).slice(-499)]));
      }
    };
  }
  var d = Rt();
  let x;
  var b = u(d), L = u(b), X = u(L), U = g(u(X), 2);
  Qe(U, { get ariaLabel() {
    return l.events.eventLevel;
  }, options: Ue, borderless: true, get value() {
    return t(I);
  }, set value(w) {
    A(I, B(w));
  } }), c(X);
  var D = g(X, 2);
  de(D, { level: 3, onclick: S, children: (w, K) => {
    E();
    var ae = T("Test");
    n(w, ae);
  }, $$slots: { default: true } }), c(L);
  var te = g(L, 2);
  Ke(te, 21, () => t(z), (w) => w.id, (w, K) => {
    Fe(w, { get event() {
      return t(K);
    } });
  }), c(te), c(b);
  var G = g(b, 2);
  Ot(G, { get wide() {
    return t(e);
  } }), c(d), p(() => x = Be(d, 1, "svelte-zmipc9", null, x, { wide: t(e), narrow: !t(e) })), _e("innerWidth", (w) => A(i, B(w))), n(h, d), me();
}
var Ut = M('<div class="noAdmin svelte-drznxl"><div><div class="text svelte-drznxl"><!></div> <!></div></div>'), Dt = M('<div class="noAdmin svelte-drznxl"><div><div class="text svelte-drznxl"><!></div> <!></div></div>'), Gt = M('<div class="events"><!></div>'), Kt = M('<div class="content svelte-drznxl"><!></div> <!>', 1), Ft = M("<!> <!>", 1);
function br(h, r) {
  he(r, true), Ne();
  let a = we(), l = Ge("admin"), i = H(void 0), e = H(false), y = H(false), _ = H(false);
  se(() => {
    var _a;
    let d = l.get();
    d && (!!((_a = d == null ? void 0 : d.roles) == null ? void 0 : _a.includes("rauthy_admin")) ? (A(e, true), z()) : A(y, true));
  });
  async function z() {
    (await ze("/auth/v1/auth_check_admin")).status === 406 && A(_, true);
  }
  var I = ce();
  Ee((d) => {
    Te.title = "Rauthy Admin";
  });
  var O = Y(I);
  {
    var S = (d) => {
      var x = Ut(), b = u(x), L = u(b), X = u(L);
      Me(X, () => a.error.noAdmin), c(L);
      var U = g(L, 2);
      de(U, { onclick: () => window.location.href = "/auth/v1/account", children: (D, te) => {
        E();
        var G = T();
        p(() => P(G, a.common.account)), n(D, G);
      }, $$slots: { default: true } }), c(b), c(x), n(d, x);
    }, k = (d) => {
      var x = ce(), b = Y(x);
      {
        var L = (U) => {
          var D = Dt(), te = u(D), G = u(te), w = u(G);
          Me(w, () => a.error.needsAdminRole), c(G);
          var K = g(G, 2);
          de(K, { onclick: () => window.location.href = "/auth/v1/", children: (ae, C) => {
            E();
            var j = T();
            p(() => P(j, a.common.back)), n(ae, j);
          }, $$slots: { default: true } }), c(te), c(D), n(U, D);
        }, X = (U) => {
          var D = ce(), te = Y(D);
          {
            var G = (w) => {
              var K = Ft(), ae = Y(K);
              $t(ae, {});
              var C = g(ae, 2);
              He(C, { children: (j, F) => {
                var re = Kt(), ie = Y(re), J = u(ie);
                ue(J, () => r.children), c(ie);
                var q = g(ie, 2);
                {
                  var $ = (W) => {
                    var V = Gt(), oe = u(V);
                    Zt(oe, {}), c(V), n(W, V);
                  };
                  ee(q, (W) => {
                    t(i) && t(i) > 1024 && W($);
                  });
                }
                n(j, re);
              } }), n(w, K);
            };
            ee(te, (w) => {
              t(e) && w(G);
            }, true);
          }
          n(U, D);
        };
        ee(b, (U) => {
          t(y) ? U(L) : U(X, false);
        }, true);
      }
      n(d, x);
    };
    ee(O, (d) => {
      t(_) ? d(S) : d(k, false);
    });
  }
  _e("innerWidth", (d) => A(i, B(d))), n(h, I), me();
}
export {
  br as component
};
