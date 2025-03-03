import { t as R, a as x, d as Z } from "../chunks/BH6NCLk-.js";
import { p as $, k as n, aa as S, a as ee, j as e, l, f as k, s as p, c as C, r as E } from "../chunks/CvlvO1XB.js";
import { i as te } from "../chunks/BUO_AUgz.js";
import { e as re } from "../chunks/BpWRzPRQ.js";
import { p as r } from "../chunks/Wh68IIk2.js";
import { E as P, D as ae } from "../chunks/B8wC3kJv.js";
import { E as se } from "../chunks/DM0MFlCa.js";
import { b as ie } from "../chunks/DYtiVhoA.js";
import { O as F } from "../chunks/BPVdf4Rb.js";
import { u as le } from "../chunks/D8mHI_K9.js";
import { I as j } from "../chunks/CxCmFdor.js";
import { f as oe, a as ne, b as B } from "../chunks/DswDW5U8.js";
import { O as me } from "../chunks/wXCWlxMM.js";
import { C as ve } from "../chunks/BnPoFdx3.js";
var ue = R('<div class="row svelte-itq81i">No events found</div>'), pe = R('<!> <div class="row svelte-itq81i"><!> <!></div> <div class="filters svelte-itq81i"><div class="level svelte-itq81i"><!></div> <div class="typ svelte-itq81i"><!></div></div> <!>', 1);
function fe(b, Y) {
  $(Y, true);
  let f = le(), v = n(r([])), u = n(r([])), L = n(r(oe())), O = n(r(ne())), d = n(""), c = n("--:--"), w = n("info"), h = n(r(P[0])), g = ["IP", "Content"], _ = n(r(g[0])), D = n(""), y = ["Timestamp", "Level", "Type"];
  S(() => {
    z();
  }), S(() => {
    let s = e(D).toLowerCase();
    s ? e(_) === g[0] ? l(u, r(e(v).filter((o) => {
      var _a;
      return (_a = o.ip) == null ? void 0 : _a.includes(s);
    }))) : e(_) === g[1] && l(u, r(e(v).filter((o) => {
      var _a, _b;
      return ((_a = o.typ) == null ? void 0 : _a.toLowerCase().includes(s)) || ((_b = o.text) == null ? void 0 : _b.toLowerCase().includes(s));
    }))) : l(u, r(e(v)));
  });
  async function z() {
    let s = B(e(L), e(O));
    if (!s) {
      console.error("from ts invalid", s);
      return;
    }
    let o;
    e(d) && e(c) && (o = B(e(d), e(c)));
    let m = { from: s, until: o, level: e(w), typ: e(h) !== P[0] ? e(h) : void 0 }, a = await ie("/auth/v1/events", m);
    a.body ? l(v, r(a.body)) : console.error(a.error);
  }
  function G(s, o) {
    let m = o === "up";
    s === y[0] ? e(v).sort((a, i) => m ? a.timestamp - i.timestamp : i.timestamp - a.timestamp) : s === y[1] ? e(v).sort((a, i) => m ? a.level.localeCompare(i.level) : i.level.localeCompare(a.level)) : s === y[2] && e(v).sort((a, i) => m ? a.typ.localeCompare(i.typ) : i.typ.localeCompare(a.typ));
  }
  ve(b, { children: (s, o) => {
    var m = pe(), a = k(m);
    me(a, { searchOptions: g, orderOptions: y, onChangeOrder: G, firstDirReverse: true, get searchOption() {
      return e(_);
    }, set searchOption(t) {
      l(_, r(t));
    }, get value() {
      return e(D);
    }, set value(t) {
      l(D, r(t));
    } });
    var i = p(a, 2), A = C(i);
    j(A, { get label() {
      return f.common.from;
    }, withTime: true, get value() {
      return e(L);
    }, set value(t) {
      l(L, r(t));
    }, get timeValue() {
      return e(O);
    }, set timeValue(t) {
      l(O, r(t));
    } });
    var H = p(A, 2);
    j(H, { get label() {
      return f.common.until;
    }, withTime: true, withDelete: true, get value() {
      return e(d);
    }, set value(t) {
      l(d, r(t));
    }, get timeValue() {
      return e(c);
    }, set timeValue(t) {
      l(c, r(t));
    } }), E(i);
    var V = p(i, 2), I = C(V), J = C(I);
    F(J, { get ariaLabel() {
      return f.events.eventLevel;
    }, options: ae, borderless: true, get value() {
      return e(w);
    }, set value(t) {
      l(w, r(t));
    } }), E(I);
    var N = p(I, 2), K = C(N);
    F(K, { get ariaLabel() {
      return f.events.eventType;
    }, options: P, borderless: true, get value() {
      return e(h);
    }, set value(t) {
      l(h, r(t));
    } }), E(N), E(V);
    var M = p(V, 2);
    {
      var Q = (t) => {
        var T = ue();
        x(t, T);
      }, U = (t) => {
        var T = Z(), W = k(T);
        re(W, 17, () => e(u), (q) => q.id, (q, X) => {
          se(q, { get event() {
            return e(X);
          }, inline: true });
        }), x(t, T);
      };
      te(M, (t) => {
        e(u).length === 0 ? t(Q) : t(U, false);
      });
    }
    x(s, m);
  } }), ee();
}
function Ve(b) {
  fe(b, {});
}
export {
  Ve as component
};
