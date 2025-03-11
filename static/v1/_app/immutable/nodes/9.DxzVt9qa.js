import { t as x, a as i, e as ie, d as K } from "../chunks/CWf9OOFK.js";
import { p as Pe, k as m, a6 as we, ab as le, f as T, t as y, a as ke, s as f, j as e, l as r, a8 as Q, c as u, aa as Te, r as _, a9 as ye } from "../chunks/nlANaGLT.js";
import { s as C } from "../chunks/BmMHVVX3.js";
import { i as X } from "../chunks/DOjUa9u5.js";
import { e as ne } from "../chunks/B6azywu7.js";
import { a as Ce } from "../chunks/CZv_AhHu.js";
import { p as t } from "../chunks/5u5qd9TD.js";
import { O as Be } from "../chunks/Np7i_reI.js";
import { B as Y } from "../chunks/BEQMYyDu.js";
import { I as De } from "../chunks/6f3yten3.js";
import { A as Me, l as Ae } from "../chunks/B21bTIl7.js";
import { u as Ee } from "../chunks/DwvS5LQk.js";
import { u as Fe } from "../chunks/Bb8ybDgy.js";
import { I as Se } from "../chunks/Bnlt5G4d.js";
import { f as ve, a as Oe } from "../chunks/DswDW5U8.js";
import { I as Ve } from "../chunks/DJ6IvX5N.js";
import { f as Le, b as Ne, c as Ue } from "../chunks/BO1A6s0c.js";
import { C as je } from "../chunks/BSE9YYho.js";
import { B as qe } from "../chunks/B-Kclwyq.js";
import { c as ze } from "../chunks/BRCxk8by.js";
import { F as Ge } from "../chunks/CmL1R98Y.js";
import { T as de } from "../chunks/C8iV2OFO.js";
import { P as Re } from "../chunks/mxwYGKSe.js";
var We = x('<!> <!> <div></div> <!> <div class="err"> </div>', 1), He = x('<div class="addNew svelte-1v7phw9"><!></div>'), Je = x("<div> </div>"), Ke = x('<div class="ip svelte-1v7phw9"><!></div>'), Qe = x('<div class="delete svelte-1v7phw9"><!></div>'), Xe = x('<div class="blacklisted svelte-1v7phw9"><!> <div class="date svelte-1v7phw9"> </div> <!></div>'), Ye = x('<div class="top svelte-1v7phw9"><!> <!></div> <div id="blacklist"><!></div> <!>', 1), Ze = x(" <!>", 1);
function et(O, me) {
  Pe(me, true);
  let V = Ee(), pe = Fe();
  const Z = 30;
  let L = m(void 0), N = m(void 0), U = m(""), A = m(""), I = m(t([])), B = m(t([])), j = m(t([])), E = m(""), q = m(t(ve())), z = m(t(Oe())), G = m(""), ee = ["IP"];
  we(() => {
    R();
  }), le(() => {
    var _a;
    (_a = e(L)) == null ? void 0 : _a.focus();
  }), le(() => {
    let o = e(G).toLowerCase();
    o ? r(B, t(e(I).filter((h) => {
      var _a;
      return (_a = h.ip) == null ? void 0 : _a.includes(o);
    }))) : r(B, t(e(I)));
  });
  async function R() {
    var _a;
    let o = await Le("/auth/v1/blacklist");
    o.body ? r(I, t(o.body.ips)) : r(U, t(((_a = o.error) == null ? void 0 : _a.message) || "Error"));
  }
  async function ce(o, h) {
    var _a;
    r(A, "");
    let P = Me(e(q), e(z));
    if (!P) {
      r(A, "Invalid Date Input: User Expires");
      return;
    }
    let g = { ip: e(E), exp: P }, $ = await Ne(o.action, g);
    $.error ? r(A, t($.error.message)) : (r(E, ""), (_a = e(N)) == null ? void 0 : _a(), await R());
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
    Be($, { orderOptions: ee, onChangeOrder: ue, searchWidth: "min(15.75rem, calc(100dvw - 1rem))", get value() {
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
      var b = He(), H = u(b);
      Ge(H, { action: "/auth/v1/blacklist", onSubmit: ce, children: (J, ae) => {
        var v = We(), s = T(v);
        De(s, { autocomplete: "off", label: "IP", placeholder: "IPv4", errMsg: "IPv4", required: true, maxLength: 15, pattern: ze, get ref() {
          return e(L);
        }, set ref(d) {
          r(L, t(d));
        }, get value() {
          return e(E);
        }, set value(d) {
          r(E, t(d));
        } });
        var p = f(s, 2);
        const l = Te(ve);
        Se(p, { label: "Expiry", get min() {
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
        Ce(c, "", {}, { height: ".5rem" });
        var F = f(c, 2);
        Y(F, { type: "submit", children: (d, k) => {
          Q();
          var D = ie();
          y(() => C(D, V.common.save)), i(d, D);
        }, $$slots: { default: true } });
        var S = f(F, 2), w = u(S, true);
        _(S), y(() => C(w, e(A))), i(J, v);
      }, $$slots: { default: true } }), _(b), i(a, b);
    }, $$slots: { default: true } }), _(g);
    var W = f(g, 2), ge = u(W);
    {
      var $e = (a) => {
        var n = Je(), b = u(n, true);
        _(n), y(() => C(b, pe.common.noEntries)), i(a, n);
      }, be = (a) => {
        var n = K();
        const b = (v, s = ye) => {
          var p = Xe(), l = u(p);
          Y(l, { invisible: true, onclick: () => navigator.clipboard.writeText(s().ip), children: (w, d) => {
            var k = Ke(), D = u(k);
            de(D, { get text() {
              return V.common.copyToClip;
            }, children: (M, oe) => {
              Q();
              var se = ie();
              y(() => C(se, s().ip)), i(M, se);
            }, $$slots: { default: true } }), _(k), i(w, k);
          }, $$slots: { default: true } });
          var c = f(l, 2), F = u(c, true);
          _(c);
          var S = f(c, 2);
          Y(S, { invisible: true, onclick: () => fe(s().ip), children: (w, d) => {
            de(w, { get text() {
              return V.common.delete;
            }, children: (k, D) => {
              var M = Qe(), oe = u(M);
              Ve(oe, { width: "1.25rem" }), _(M), i(k, M);
            }, $$slots: { default: true } });
          }, $$slots: { default: true } }), _(p), y((w) => C(F, w), [() => Ae(s().exp)]), i(v, p);
        };
        var H = T(n);
        {
          var J = (v) => {
            var s = K(), p = T(s);
            ne(p, 17, () => e(j), (l) => l.ip, (l, c) => {
              b(l, () => e(c));
            }), i(v, s);
          }, ae = (v) => {
            var s = K(), p = T(s);
            ne(p, 17, () => e(B), (l) => l.ip, (l, c) => {
              b(l, () => e(c));
            }), i(v, s);
          };
          X(H, (v) => {
            e(I).length > Z ? v(J) : v(ae, false);
          });
        }
        i(a, n);
      };
      X(ge, (a) => {
        e(I).length === 0 ? a($e) : a(be, false);
      });
    }
    _(W);
    var xe = f(W, 2);
    {
      var Ie = (a) => {
        Re(a, { pageSize: 30, get items() {
          return e(B);
        }, set items(n) {
          r(B, t(n));
        }, get itemsPaginated() {
          return e(j);
        }, set itemsPaginated(n) {
          r(j, t(n));
        } });
      };
      X(xe, (a) => {
        e(I).length > Z && a(Ie);
      });
    }
    i(o, P);
  } }), y(() => C(re, `${e(U) ?? ""} `)), i(O, te), ke();
}
function wt(O) {
  et(O, {});
}
export {
  wt as component
};
