import { t as R, a as P, d as X } from "../chunks/BH6NCLk-.js";
import { p as Z, k as n, aa as k, a as $, j as e, l as o, c as p, s as f, r as d, f as ee } from "../chunks/CvlvO1XB.js";
import { i as te } from "../chunks/BUO_AUgz.js";
import { e as re } from "../chunks/BpWRzPRQ.js";
import { p as r } from "../chunks/Wh68IIk2.js";
import { E as A, B as ae } from "../chunks/B21bTIl7.js";
import { E as se } from "../chunks/DV_qELhu.js";
import { b as le } from "../chunks/BO1A6s0c.js";
import { O as B } from "../chunks/D1anwFsC.js";
import { u as oe } from "../chunks/D8mHI_K9.js";
import { I as F } from "../chunks/BXc4I0Oc.js";
import { f as ie, a as ne, b as j } from "../chunks/DswDW5U8.js";
import { O as ve } from "../chunks/CONsTX1L.js";
import { C as me } from "../chunks/BnPoFdx3.js";
var ue = R('<div class="row svelte-1sw547f">No events found</div>'), pe = R('<div id="archive" class="svelte-1sw547f"><!> <div class="row svelte-1sw547f"><!> <!></div> <div class="filters svelte-1sw547f"><div class="level svelte-1sw547f"><!></div> <div class="typ svelte-1sw547f"><!></div></div> <!></div>');
function fe(E, Y) {
  Z(Y, true);
  let c = oe(), m = n(r([])), u = n(r([])), b = n(r(ie())), L = n(r(ne())), h = n(""), _ = n("--:--"), O = n("info"), g = n(r(A[0])), w = ["IP", "Content"], y = n(r(w[0])), D = n(""), T = ["Timestamp", "Level", "Type"];
  k(() => {
    q();
  }), k(() => {
    let s = e(D).toLowerCase();
    s ? e(y) === w[0] ? o(u, r(e(m).filter((i) => {
      var _a;
      return (_a = i.ip) == null ? void 0 : _a.includes(s);
    }))) : e(y) === w[1] && o(u, r(e(m).filter((i) => {
      var _a, _b;
      return ((_a = i.typ) == null ? void 0 : _a.toLowerCase().includes(s)) || ((_b = i.text) == null ? void 0 : _b.toLowerCase().includes(s));
    }))) : o(u, r(e(m)));
  });
  async function q() {
    let s = j(e(b), e(L));
    if (!s) {
      console.error("from ts invalid", s);
      return;
    }
    let i;
    e(h) && e(_) && (i = j(e(h), e(_)));
    let v = { from: s, until: i, level: e(O), typ: e(g) !== A[0] ? e(g) : void 0 }, a = await le("/auth/v1/events", v);
    a.body ? o(m, r(a.body)) : console.error(a.error);
  }
  function z(s, i) {
    let v = i === "up";
    s === T[0] ? e(m).sort((a, l) => v ? a.timestamp - l.timestamp : l.timestamp - a.timestamp) : s === T[1] ? e(m).sort((a, l) => v ? a.level.localeCompare(l.level) : l.level.localeCompare(a.level)) : s === T[2] && e(m).sort((a, l) => v ? a.typ.localeCompare(l.typ) : l.typ.localeCompare(a.typ));
  }
  me(E, { children: (s, i) => {
    var v = pe(), a = p(v);
    ve(a, { searchOptions: w, orderOptions: T, onChangeOrder: z, firstDirReverse: true, get searchOption() {
      return e(y);
    }, set searchOption(t) {
      o(y, r(t));
    }, get value() {
      return e(D);
    }, set value(t) {
      o(D, r(t));
    } });
    var l = f(a, 2), N = p(l);
    F(N, { get label() {
      return c.common.from;
    }, withTime: true, get value() {
      return e(b);
    }, set value(t) {
      o(b, r(t));
    }, get timeValue() {
      return e(L);
    }, set timeValue(t) {
      o(L, r(t));
    } });
    var G = f(N, 2);
    F(G, { get label() {
      return c.common.until;
    }, withTime: true, withDelete: true, get value() {
      return e(h);
    }, set value(t) {
      o(h, r(t));
    }, get timeValue() {
      return e(_);
    }, set timeValue(t) {
      o(_, r(t));
    } }), d(l);
    var V = f(l, 2), I = p(V), H = p(I);
    B(H, { get ariaLabel() {
      return c.events.eventLevel;
    }, options: ae, borderless: true, get value() {
      return e(O);
    }, set value(t) {
      o(O, r(t));
    } }), d(I);
    var S = f(I, 2), J = p(S);
    B(J, { get ariaLabel() {
      return c.events.eventType;
    }, options: A, borderless: true, get value() {
      return e(g);
    }, set value(t) {
      o(g, r(t));
    } }), d(S), d(V);
    var K = f(V, 2);
    {
      var M = (t) => {
        var C = ue();
        P(t, C);
      }, Q = (t) => {
        var C = X(), U = ee(C);
        re(U, 17, () => e(u), (x) => x.id, (x, W) => {
          se(x, { get event() {
            return e(W);
          }, inline: true });
        }), P(t, C);
      };
      te(K, (t) => {
        e(u).length === 0 ? t(M) : t(Q, false);
      });
    }
    d(v), P(s, v);
  } }), $();
}
function Ve(E) {
  fe(E, {});
}
export {
  Ve as component
};
