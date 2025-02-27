import { t as b, a as i, e as ie, d as K } from "../chunks/D8nUqfqE.js";
import { p as Pe, k as m, a5 as we, aa as le, f as T, t as y, a as ke, s as f, j as e, l as r, a7 as Q, c as u, a9 as Te, r as _, a8 as ye } from "../chunks/D-nwkJyM.js";
import { s as B } from "../chunks/DmLjbmU6.js";
import { i as X } from "../chunks/C3XMhfdI.js";
import { e as ne } from "../chunks/6MDunRKZ.js";
import { a as Be, B as Y } from "../chunks/BUAPoI3e.js";
import { p as t } from "../chunks/BiI21XkS.js";
import { O as Ce } from "../chunks/TvawFHML.js";
import { I as De } from "../chunks/BE8gxf21.js";
import { B as Me, l as Ee } from "../chunks/DppGgfa0.js";
import { u as Fe } from "../chunks/CMjKUQkH.js";
import { u as Se } from "../chunks/DMJUG0wm.js";
import { I as Ae } from "../chunks/RN-bweNU.js";
import { f as ve, a as Oe } from "../chunks/DswDW5U8.js";
import { I as Ve } from "../chunks/Dq3vuPfn.js";
import { f as Le, b as Ne, c as Ue } from "../chunks/emZtalxW.js";
import { C as je } from "../chunks/C-Lg4I_M.js";
import { B as qe } from "../chunks/BB-qItao.js";
import { c as ze } from "../chunks/BRCxk8by.js";
import { F as Ge } from "../chunks/BgEKkuL8.js";
import { T as de } from "../chunks/g4HrlpfL.js";
import { P as Re } from "../chunks/PQEWmBlL.js";
var We = b('<!> <!> <div></div> <!> <div class="err"> </div>', 1), He = b('<div class="addNew svelte-1v7phw9"><!></div>'), Je = b("<div> </div>"), Ke = b('<div class="ip svelte-1v7phw9"><!></div>'), Qe = b('<div class="delete svelte-1v7phw9"><!></div>'), Xe = b('<div class="blacklisted svelte-1v7phw9"><!> <div class="date svelte-1v7phw9"> </div> <!></div>'), Ye = b('<div class="top svelte-1v7phw9"><!> <!></div> <div id="blacklist"><!></div> <!>', 1), Ze = b(" <!>", 1);
function et(O, me) {
  Pe(me, true);
  let V = Fe(), pe = Se();
  const Z = 30;
  let L = m(void 0), N = m(void 0), U = m(""), E = m(""), I = m(t([])), C = m(t([])), j = m(t([])), F = m(""), q = m(t(ve())), z = m(t(Oe())), G = m(""), ee = ["IP"];
  we(() => {
    R();
  }), le(() => {
    var _a;
    (_a = e(L)) == null ? void 0 : _a.focus();
  }), le(() => {
    let o = e(G).toLowerCase();
    o ? r(C, t(e(I).filter((h) => {
      var _a;
      return (_a = h.ip) == null ? void 0 : _a.includes(o);
    }))) : r(C, t(e(I)));
  });
  async function R() {
    var _a;
    let o = await Le("/auth/v1/blacklist");
    o.body ? r(I, t(o.body.ips)) : r(U, t(((_a = o.error) == null ? void 0 : _a.message) || "Error"));
  }
  async function ce(o, h) {
    var _a;
    r(E, "");
    let P = Me(e(q), e(z));
    if (!P) {
      r(E, "Invalid Date Input: User Expires");
      return;
    }
    let g = { ip: e(F), exp: P }, $ = await Ne(o.action, g);
    $.error ? r(E, t($.error.message)) : (r(F, ""), (_a = e(N)) == null ? void 0 : _a(), await R());
  }
  async function fe(o) {
    let h = await Ue(`/auth/v1/blacklist/${o}`);
    h.error ? r(U, t(h.error.message)) : await R();
  }
  function ue(o, h) {
    let P = h === "up";
    o === ee[0] && e(I).sort((g, $) => P ? g.ip.localeCompare($.ip) : $.ip.localeCompare(g.ip));
  }
  Q();
  var te = Ze(), re = T(te), _e = f(re);
  je(_e, { children: (o, h) => {
    var P = Ye(), g = T(P), $ = u(g);
    Ce($, { orderOptions: ee, onChangeOrder: ue, searchWidth: "min(15.75rem, calc(100dvw - 1rem))", get value() {
      return e(G);
    }, set value(a) {
      r(G, t(a));
    } });
    var he = f($, 2);
    qe(he, { level: 1, get closeModal() {
      return e(N);
    }, set closeModal(a) {
      r(N, t(a));
    }, children: (a, n) => {
      var x = He(), H = u(x);
      Ge(H, { action: "/auth/v1/blacklist", onSubmit: ce, children: (J, ae) => {
        var v = We(), s = T(v);
        De(s, { autocomplete: "off", label: "IP", placeholder: "IPv4", errMsg: "IPv4", required: true, maxLength: 15, pattern: ze, get ref() {
          return e(L);
        }, set ref(d) {
          r(L, t(d));
        }, get value() {
          return e(F);
        }, set value(d) {
          r(F, t(d));
        } });
        var p = f(s, 2);
        const l = Te(ve);
        Ae(p, { label: "Expiry", get min() {
          return e(l);
        }, withTime: true, get value() {
          return e(q);
        }, set value(d) {
          r(q, t(d));
        }, get timeValue() {
          return e(z);
        }, set timeValue(d) {
          r(z, t(d));
        } });
        var c = f(p, 2);
        Be(c, "height", ".5rem");
        var S = f(c, 2);
        Y(S, { type: "submit", children: (d, k) => {
          Q();
          var D = ie();
          y(() => B(D, V.common.save)), i(d, D);
        }, $$slots: { default: true } });
        var A = f(S, 2), w = u(A, true);
        _(A), y(() => B(w, e(E))), i(J, v);
      }, $$slots: { default: true } }), _(x), i(a, x);
    }, $$slots: { default: true } }), _(g);
    var W = f(g, 2), ge = u(W);
    {
      var $e = (a) => {
        var n = Je(), x = u(n, true);
        _(n), y(() => B(x, pe.common.noEntries)), i(a, n);
      }, xe = (a) => {
        var n = K();
        const x = (v, s = ye) => {
          var p = Xe(), l = u(p);
          Y(l, { invisible: true, onclick: () => navigator.clipboard.writeText(s().ip), children: (w, d) => {
            var k = Ke(), D = u(k);
            de(D, { get text() {
              return V.common.copyToClip;
            }, children: (M, oe) => {
              Q();
              var se = ie();
              y(() => B(se, s().ip)), i(M, se);
            }, $$slots: { default: true } }), _(k), i(w, k);
          }, $$slots: { default: true } });
          var c = f(l, 2), S = u(c, true);
          _(c);
          var A = f(c, 2);
          Y(A, { invisible: true, onclick: () => fe(s().ip), children: (w, d) => {
            de(w, { get text() {
              return V.common.delete;
            }, children: (k, D) => {
              var M = Qe(), oe = u(M);
              Ve(oe, { width: "1.25rem" }), _(M), i(k, M);
            }, $$slots: { default: true } });
          }, $$slots: { default: true } }), _(p), y((w) => B(S, w), [() => Ee(s().exp)]), i(v, p);
        };
        var H = T(n);
        {
          var J = (v) => {
            var s = K(), p = T(s);
            ne(p, 17, () => e(j), (l) => l.ip, (l, c) => {
              x(l, () => e(c));
            }), i(v, s);
          }, ae = (v) => {
            var s = K(), p = T(s);
            ne(p, 17, () => e(C), (l) => l.ip, (l, c) => {
              x(l, () => e(c));
            }), i(v, s);
          };
          X(H, (v) => {
            e(I).length > Z ? v(J) : v(ae, false);
          });
        }
        i(a, n);
      };
      X(ge, (a) => {
        e(I).length === 0 ? a($e) : a(xe, false);
      });
    }
    _(W);
    var be = f(W, 2);
    {
      var Ie = (a) => {
        Re(a, { pageSize: 30, get items() {
          return e(C);
        }, set items(n) {
          r(C, t(n));
        }, get itemsPaginated() {
          return e(j);
        }, set itemsPaginated(n) {
          r(j, t(n));
        } });
      };
      X(be, (a) => {
        e(I).length > Z && a(Ie);
      });
    }
    i(o, P);
  } }), y(() => B(re, `${e(U) ?? ""} `)), i(O, te), ke();
}
function Pt(O) {
  et(O, {});
}
export {
  Pt as component
};
