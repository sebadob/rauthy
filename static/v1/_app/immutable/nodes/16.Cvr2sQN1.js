import { t as j, a as x, d as X } from "../chunks/BH6NCLk-.js";
import { p as Z, k as n, aa as S, a as $, j as e, l, c as p, s as f, r as d, f as ee } from "../chunks/CvlvO1XB.js";
import { i as te } from "../chunks/BUO_AUgz.js";
import { e as re } from "../chunks/BpWRzPRQ.js";
import { p as r } from "../chunks/Wh68IIk2.js";
import { E as P, B as ae } from "../chunks/B21bTIl7.js";
import { E as ie } from "../chunks/B5jqu_oq.js";
import { b as se } from "../chunks/BO1A6s0c.js";
import { O as k } from "../chunks/CdqYxeyI.js";
import { u as le } from "../chunks/D8mHI_K9.js";
import { I as B } from "../chunks/Dre48gu9.js";
import { f as oe, a as ne, b as F } from "../chunks/DswDW5U8.js";
import { O as ve } from "../chunks/V6L-Tw6V.js";
import { C as me } from "../chunks/BnPoFdx3.js";
var ue = j('<div class="row svelte-itq81i">No events found</div>'), pe = j('<div id="archive"><!> <div class="row svelte-itq81i"><!> <!></div> <div class="filters svelte-itq81i"><div class="level svelte-itq81i"><!></div> <div class="typ svelte-itq81i"><!></div></div> <!></div>');
function fe(b, R) {
  Z(R, true);
  let c = le(), m = n(r([])), u = n(r([])), L = n(r(oe())), O = n(r(ne())), h = n(""), _ = n("--:--"), w = n("info"), g = n(r(P[0])), y = ["IP", "Content"], T = n(r(y[0])), D = n(""), C = ["Timestamp", "Level", "Type"];
  S(() => {
    Y();
  }), S(() => {
    let i = e(D).toLowerCase();
    i ? e(T) === y[0] ? l(u, r(e(m).filter((o) => {
      var _a;
      return (_a = o.ip) == null ? void 0 : _a.includes(i);
    }))) : e(T) === y[1] && l(u, r(e(m).filter((o) => {
      var _a, _b;
      return ((_a = o.typ) == null ? void 0 : _a.toLowerCase().includes(i)) || ((_b = o.text) == null ? void 0 : _b.toLowerCase().includes(i));
    }))) : l(u, r(e(m)));
  });
  async function Y() {
    let i = F(e(L), e(O));
    if (!i) {
      console.error("from ts invalid", i);
      return;
    }
    let o;
    e(h) && e(_) && (o = F(e(h), e(_)));
    let v = { from: i, until: o, level: e(w), typ: e(g) !== P[0] ? e(g) : void 0 }, a = await se("/auth/v1/events", v);
    a.body ? l(m, r(a.body)) : console.error(a.error);
  }
  function z(i, o) {
    let v = o === "up";
    i === C[0] ? e(m).sort((a, s) => v ? a.timestamp - s.timestamp : s.timestamp - a.timestamp) : i === C[1] ? e(m).sort((a, s) => v ? a.level.localeCompare(s.level) : s.level.localeCompare(a.level)) : i === C[2] && e(m).sort((a, s) => v ? a.typ.localeCompare(s.typ) : s.typ.localeCompare(a.typ));
  }
  me(b, { children: (i, o) => {
    var v = pe(), a = p(v);
    ve(a, { searchOptions: y, orderOptions: C, onChangeOrder: z, firstDirReverse: true, get searchOption() {
      return e(T);
    }, set searchOption(t) {
      l(T, r(t));
    }, get value() {
      return e(D);
    }, set value(t) {
      l(D, r(t));
    } });
    var s = f(a, 2), A = p(s);
    B(A, { get label() {
      return c.common.from;
    }, withTime: true, get value() {
      return e(L);
    }, set value(t) {
      l(L, r(t));
    }, get timeValue() {
      return e(O);
    }, set timeValue(t) {
      l(O, r(t));
    } });
    var G = f(A, 2);
    B(G, { get label() {
      return c.common.until;
    }, withTime: true, withDelete: true, get value() {
      return e(h);
    }, set value(t) {
      l(h, r(t));
    }, get timeValue() {
      return e(_);
    }, set timeValue(t) {
      l(_, r(t));
    } }), d(s);
    var V = f(s, 2), I = p(V), H = p(I);
    k(H, { get ariaLabel() {
      return c.events.eventLevel;
    }, options: ae, borderless: true, get value() {
      return e(w);
    }, set value(t) {
      l(w, r(t));
    } }), d(I);
    var N = f(I, 2), J = p(N);
    k(J, { get ariaLabel() {
      return c.events.eventType;
    }, options: P, borderless: true, get value() {
      return e(g);
    }, set value(t) {
      l(g, r(t));
    } }), d(N), d(V);
    var K = f(V, 2);
    {
      var M = (t) => {
        var E = ue();
        x(t, E);
      }, Q = (t) => {
        var E = X(), U = ee(E);
        re(U, 17, () => e(u), (q) => q.id, (q, W) => {
          ie(q, { get event() {
            return e(W);
          }, inline: true });
        }), x(t, E);
      };
      te(K, (t) => {
        e(u).length === 0 ? t(M) : t(Q, false);
      });
    }
    d(v), x(i, v);
  } }), $();
}
function Ve(b) {
  fe(b, {});
}
export {
  Ve as component
};
