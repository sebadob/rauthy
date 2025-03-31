import { t as x, a as s, e as ie, d as K } from "../chunks/BxmJRzoY.js";
import { p as Pe, j as d, a1 as M, a0 as we, a5 as le, f as k, t as T, a as ke, s as u, k as e, l as t, a3 as Q, c, a4 as Te, r as f, a6 as ye } from "../chunks/w0HvPX0p.js";
import { s as y } from "../chunks/BzP2S3Z_.js";
import { i as X } from "../chunks/iO9_dPNE.js";
import { e as ve } from "../chunks/S81raU5Y.js";
import { a as Ce } from "../chunks/BdbQ6g_y.js";
import { O as Be } from "../chunks/85nGGue2.js";
import { B as Y } from "../chunks/C8YTstTD.js";
import { I as De } from "../chunks/Q4PIg3iI.js";
import { A as Me, l as Ae } from "../chunks/B21bTIl7.js";
import { u as Ee } from "../chunks/0cG6LBdy.js";
import { u as Fe } from "../chunks/DHOKTGcE.js";
import { I as Se } from "../chunks/cZptBaPP.js";
import { f as ne, a as Oe } from "../chunks/DswDW5U8.js";
import { I as Ve } from "../chunks/QCVRj9pj.js";
import { f as Le, b as Ne, c as Ue } from "../chunks/BO1A6s0c.js";
import { C as je } from "../chunks/C6jTHtu1.js";
import { B as qe } from "../chunks/DoeALvoe.js";
import { c as ze } from "../chunks/gfDO7tLr.js";
import { F as Ge } from "../chunks/CDe-qvZi.js";
import { T as de } from "../chunks/BnlWDKfj.js";
import { P as Re } from "../chunks/l-KvjgJG.js";
var We = x('<!> <!> <div></div> <!> <div class="err"> </div>', 1), He = x('<div class="addNew svelte-1v7phw9"><!></div>'), Je = x("<div> </div>"), Ke = x('<div class="ip svelte-1v7phw9"><!></div>'), Qe = x('<div class="delete svelte-1v7phw9"><!></div>'), Xe = x('<div class="blacklisted svelte-1v7phw9"><!> <div class="date svelte-1v7phw9"> </div> <!></div>'), Ye = x('<div class="top svelte-1v7phw9"><!> <!></div> <div id="blacklist"><!></div> <!>', 1), Ze = x(" <!>", 1);
function et(O, me) {
  Pe(me, true);
  let V = Ee(), pe = Fe();
  const Z = 30;
  let L = d(void 0), N = d(void 0), U = d(""), A = d(""), b = d(M([])), C = d(M([])), j = d(M([])), E = d(""), q = d(M(ne())), z = d(M(Oe())), G = d(""), ee = ["IP"];
  we(() => {
    R();
  }), le(() => {
    var _a;
    (_a = e(L)) == null ? void 0 : _a.focus();
  }), le(() => {
    let a = e(G).toLowerCase();
    a ? t(C, e(b).filter((_) => {
      var _a;
      return (_a = _.ip) == null ? void 0 : _a.includes(a);
    }), true) : t(C, e(b), true);
  });
  async function R() {
    var _a;
    let a = await Le("/auth/v1/blacklist");
    a.body ? t(b, a.body.ips, true) : t(U, ((_a = a.error) == null ? void 0 : _a.message) || "Error", true);
  }
  async function ue(a, _) {
    var _a;
    t(A, "");
    let I = Me(e(q), e(z));
    if (!I) {
      t(A, "Invalid Date Input: User Expires");
      return;
    }
    let h = { ip: e(E), exp: I }, g = await Ne(a.action, h);
    g.error ? t(A, g.error.message, true) : (t(E, ""), (_a = e(N)) == null ? void 0 : _a(), await R());
  }
  async function ce(a) {
    let _ = await Ue(`/auth/v1/blacklist/${a}`);
    _.error ? t(U, _.error.message, true) : await R();
  }
  function fe(a, _) {
    let I = _ === "up";
    a === ee[0] && e(b).sort((h, g) => I ? h.ip.localeCompare(g.ip) : g.ip.localeCompare(h.ip));
  }
  Q();
  var te = Ze(), re = k(te), _e = u(re);
  je(_e, { children: (a, _) => {
    var I = Ye(), h = k(I), g = c(h);
    Be(g, { orderOptions: ee, onChangeOrder: fe, searchWidth: "min(15.75rem, calc(100dvw - 1rem))", get value() {
      return e(G);
    }, set value(r) {
      t(G, r, true);
    } });
    var he = u(g, 2);
    qe(he, { level: 1, get closeModal() {
      return e(N);
    }, set closeModal(r) {
      t(N, r, true);
    }, children: (r, l) => {
      var $ = He(), H = c($);
      Ge(H, { action: "/auth/v1/blacklist", onSubmit: ue, children: (J, ae) => {
        var v = We(), o = k(v);
        De(o, { autocomplete: "off", label: "IP", placeholder: "IPv4", errMsg: "IPv4", required: true, maxLength: 15, pattern: ze, get ref() {
          return e(L);
        }, set ref(n) {
          t(L, n, true);
        }, get value() {
          return e(E);
        }, set value(n) {
          t(E, n, true);
        } });
        var m = u(o, 2);
        const i = Te(ne);
        Se(m, { label: "Expiry", get min() {
          return e(i);
        }, withTime: true, get value() {
          return e(q);
        }, set value(n) {
          t(q, n, true);
        }, get timeValue() {
          return e(z);
        }, set timeValue(n) {
          t(z, n, true);
        } });
        var p = u(m, 2);
        Ce(p, "", {}, { height: ".5rem" });
        var F = u(p, 2);
        Y(F, { type: "submit", children: (n, w) => {
          Q();
          var B = ie();
          T(() => y(B, V.common.save)), s(n, B);
        }, $$slots: { default: true } });
        var S = u(F, 2), P = c(S, true);
        f(S), T(() => y(P, e(A))), s(J, v);
      }, $$slots: { default: true } }), f($), s(r, $);
    }, $$slots: { default: true } }), f(h);
    var W = u(h, 2), ge = c(W);
    {
      var $e = (r) => {
        var l = Je(), $ = c(l, true);
        f(l), T(() => y($, pe.common.noEntries)), s(r, l);
      }, xe = (r) => {
        var l = K();
        const $ = (v, o = ye) => {
          var m = Xe(), i = c(m);
          Y(i, { invisible: true, onclick: () => navigator.clipboard.writeText(o().ip), children: (P, n) => {
            var w = Ke(), B = c(w);
            de(B, { get text() {
              return V.common.copyToClip;
            }, children: (D, oe) => {
              Q();
              var se = ie();
              T(() => y(se, o().ip)), s(D, se);
            }, $$slots: { default: true } }), f(w), s(P, w);
          }, $$slots: { default: true } });
          var p = u(i, 2), F = c(p, true);
          f(p);
          var S = u(p, 2);
          Y(S, { invisible: true, onclick: () => ce(o().ip), children: (P, n) => {
            de(P, { get text() {
              return V.common.delete;
            }, children: (w, B) => {
              var D = Qe(), oe = c(D);
              Ve(oe, { width: "1.25rem" }), f(D), s(w, D);
            }, $$slots: { default: true } });
          }, $$slots: { default: true } }), f(m), T((P) => y(F, P), [() => Ae(o().exp)]), s(v, m);
        };
        var H = k(l);
        {
          var J = (v) => {
            var o = K(), m = k(o);
            ve(m, 17, () => e(j), (i) => i.ip, (i, p) => {
              $(i, () => e(p));
            }), s(v, o);
          }, ae = (v) => {
            var o = K(), m = k(o);
            ve(m, 17, () => e(C), (i) => i.ip, (i, p) => {
              $(i, () => e(p));
            }), s(v, o);
          };
          X(H, (v) => {
            e(b).length > Z ? v(J) : v(ae, false);
          });
        }
        s(r, l);
      };
      X(ge, (r) => {
        e(b).length === 0 ? r($e) : r(xe, false);
      });
    }
    f(W);
    var be = u(W, 2);
    {
      var Ie = (r) => {
        Re(r, { pageSize: 30, get items() {
          return e(C);
        }, set items(l) {
          t(C, l, true);
        }, get itemsPaginated() {
          return e(j);
        }, set itemsPaginated(l) {
          t(j, l, true);
        } });
      };
      X(be, (r) => {
        e(b).length > Z && r(Ie);
      });
    }
    s(a, I);
  } }), T(() => y(re, `${e(U) ?? ""} `)), s(O, te), ke();
}
function Pt(O) {
  et(O, {});
}
export {
  Pt as component
};
