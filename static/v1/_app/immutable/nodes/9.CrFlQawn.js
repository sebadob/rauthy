import { t as b, a as i, e as ie, d as K } from "../chunks/BH6NCLk-.js";
import { p as Pe, k as m, a5 as we, aa as le, f as T, t as y, a as ke, s as f, j as e, l as r, a7 as Q, c as u, a9 as Te, r as _, a8 as ye } from "../chunks/CvlvO1XB.js";
import { s as C } from "../chunks/CTI4QPiR.js";
import { i as X } from "../chunks/BUO_AUgz.js";
import { e as ne } from "../chunks/BpWRzPRQ.js";
import { a as Ce, B as Y } from "../chunks/DMkkW5Nn.js";
import { p as t } from "../chunks/Wh68IIk2.js";
import { O as Be } from "../chunks/CONsTX1L.js";
import { I as De } from "../chunks/BZu_mjh1.js";
import { A as Me, l as Ae } from "../chunks/B21bTIl7.js";
import { u as Ee } from "../chunks/CUqQZdNU.js";
import { u as Fe } from "../chunks/D8mHI_K9.js";
import { I as Se } from "../chunks/BXc4I0Oc.js";
import { f as ve, a as Oe } from "../chunks/DswDW5U8.js";
import { I as Ve } from "../chunks/Vi3uK7uO.js";
import { f as Le, b as Ne, c as Ue } from "../chunks/BO1A6s0c.js";
import { C as je } from "../chunks/BnPoFdx3.js";
import { B as qe } from "../chunks/ClvgHNP_.js";
import { c as ze } from "../chunks/BRCxk8by.js";
import { F as Ge } from "../chunks/CsaAZyUr.js";
import { T as de } from "../chunks/BmpEzKyJ.js";
import { P as Re } from "../chunks/5W956QVQ.js";
var We = b('<!> <!> <div></div> <!> <div class="err"> </div>', 1), He = b('<div class="addNew svelte-1v7phw9"><!></div>'), Je = b("<div> </div>"), Ke = b('<div class="ip svelte-1v7phw9"><!></div>'), Qe = b('<div class="delete svelte-1v7phw9"><!></div>'), Xe = b('<div class="blacklisted svelte-1v7phw9"><!> <div class="date svelte-1v7phw9"> </div> <!></div>'), Ye = b('<div class="top svelte-1v7phw9"><!> <!></div> <div id="blacklist"><!></div> <!>', 1), Ze = b(" <!>", 1);
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
      var x = He(), H = u(x);
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
        Ce(c, "height", ".5rem");
        var F = f(c, 2);
        Y(F, { type: "submit", children: (d, k) => {
          Q();
          var D = ie();
          y(() => C(D, V.common.save)), i(d, D);
        }, $$slots: { default: true } });
        var S = f(F, 2), w = u(S, true);
        _(S), y(() => C(w, e(A))), i(J, v);
      }, $$slots: { default: true } }), _(x), i(a, x);
    }, $$slots: { default: true } }), _(g);
    var W = f(g, 2), ge = u(W);
    {
      var $e = (a) => {
        var n = Je(), x = u(n, true);
        _(n), y(() => C(x, pe.common.noEntries)), i(a, n);
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
              x(l, () => e(c));
            }), i(v, s);
          }, ae = (v) => {
            var s = K(), p = T(s);
            ne(p, 17, () => e(B), (l) => l.ip, (l, c) => {
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
          return e(B);
        }, set items(n) {
          r(B, t(n));
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
  } }), y(() => C(re, `${e(U) ?? ""} `)), i(O, te), ke();
}
function Pt(O) {
  et(O, {});
}
export {
  Pt as component
};
