import { t as j, a as x, d as X } from "../chunks/BxmJRzoY.js";
import { p as Z, j as o, a1 as u, a5 as S, a as $, k as e, l as s, c as p, s as f, r as d, f as ee } from "../chunks/w0HvPX0p.js";
import { i as te } from "../chunks/iO9_dPNE.js";
import { e as re } from "../chunks/S81raU5Y.js";
import { E as P, B as ae } from "../chunks/B21bTIl7.js";
import { E as ie } from "../chunks/QgMpqyCL.js";
import { b as se } from "../chunks/UPFlzoow.js";
import { O as k } from "../chunks/BgkEPAwa.js";
import { u as le } from "../chunks/D12OFlGX.js";
import { I as B } from "../chunks/DtzF8ri4.js";
import { f as oe, a as ne, b as F } from "../chunks/DswDW5U8.js";
import { O as ve } from "../chunks/Ssae7OHr.js";
import { C as ue } from "../chunks/C6jTHtu1.js";
var me = j('<div class="row svelte-itq81i">No events found</div>'), pe = j('<div id="archive"><!> <div class="row svelte-itq81i"><!> <!></div> <div class="filters svelte-itq81i"><div class="level svelte-itq81i"><!></div> <div class="typ svelte-itq81i"><!></div></div> <!></div>');
function fe(b, R) {
  Z(R, true);
  let c = le(), v = o(u([])), m = o(u([])), L = o(u(oe())), O = o(u(ne())), h = o(""), _ = o("--:--"), w = o("info"), g = o(u(P[0])), y = ["IP", "Content"], T = o(u(y[0])), D = o(""), C = ["Timestamp", "Level", "Type"];
  S(() => {
    Y();
  }), S(() => {
    let a = e(D).toLowerCase();
    a ? e(T) === y[0] ? s(m, e(v).filter((l) => {
      var _a;
      return (_a = l.ip) == null ? void 0 : _a.includes(a);
    }), true) : e(T) === y[1] && s(m, e(v).filter((l) => {
      var _a, _b;
      return ((_a = l.typ) == null ? void 0 : _a.toLowerCase().includes(a)) || ((_b = l.text) == null ? void 0 : _b.toLowerCase().includes(a));
    }), true) : s(m, e(v), true);
  });
  async function Y() {
    let a = F(e(L), e(O));
    if (!a) {
      console.error("from ts invalid", a);
      return;
    }
    let l;
    e(h) && e(_) && (l = F(e(h), e(_)));
    let n = { from: a, until: l, level: e(w), typ: e(g) !== P[0] ? e(g) : void 0 }, r = await se("/auth/v1/events", n);
    r.body ? s(v, r.body, true) : console.error(r.error);
  }
  function z(a, l) {
    let n = l === "up";
    a === C[0] ? e(v).sort((r, i) => n ? r.timestamp - i.timestamp : i.timestamp - r.timestamp) : a === C[1] ? e(v).sort((r, i) => n ? r.level.localeCompare(i.level) : i.level.localeCompare(r.level)) : a === C[2] && e(v).sort((r, i) => n ? r.typ.localeCompare(i.typ) : i.typ.localeCompare(r.typ));
  }
  ue(b, { children: (a, l) => {
    var n = pe(), r = p(n);
    ve(r, { searchOptions: y, orderOptions: C, onChangeOrder: z, firstDirReverse: true, get searchOption() {
      return e(T);
    }, set searchOption(t) {
      s(T, t, true);
    }, get value() {
      return e(D);
    }, set value(t) {
      s(D, t, true);
    } });
    var i = f(r, 2), A = p(i);
    B(A, { get label() {
      return c.common.from;
    }, withTime: true, get value() {
      return e(L);
    }, set value(t) {
      s(L, t, true);
    }, get timeValue() {
      return e(O);
    }, set timeValue(t) {
      s(O, t, true);
    } });
    var G = f(A, 2);
    B(G, { get label() {
      return c.common.until;
    }, withTime: true, withDelete: true, get value() {
      return e(h);
    }, set value(t) {
      s(h, t, true);
    }, get timeValue() {
      return e(_);
    }, set timeValue(t) {
      s(_, t, true);
    } }), d(i);
    var V = f(i, 2), I = p(V), H = p(I);
    k(H, { get ariaLabel() {
      return c.events.eventLevel;
    }, options: ae, borderless: true, get value() {
      return e(w);
    }, set value(t) {
      s(w, t, true);
    } }), d(I);
    var N = f(I, 2), J = p(N);
    k(J, { get ariaLabel() {
      return c.events.eventType;
    }, options: P, borderless: true, get value() {
      return e(g);
    }, set value(t) {
      s(g, t, true);
    } }), d(N), d(V);
    var K = f(V, 2);
    {
      var M = (t) => {
        var E = me();
        x(t, E);
      }, Q = (t) => {
        var E = X(), U = ee(E);
        re(U, 17, () => e(m), (q) => q.id, (q, W) => {
          ie(q, { get event() {
            return e(W);
          }, inline: true });
        }), x(t, E);
      };
      te(K, (t) => {
        e(m).length === 0 ? t(M) : t(Q, false);
      });
    }
    d(n), x(a, n);
  } }), $();
}
function De(b) {
  fe(b, {});
}
export {
  De as component
};
