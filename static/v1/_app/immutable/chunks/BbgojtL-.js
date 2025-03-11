import { n as fe, a as s, t as w, e as L, d as X } from "./CWf9OOFK.js";
import { t as _, p as _e, s as b, c as f, l as P, k as D, f as F, j as e, a as ue, r as l, a8 as j, aa as G } from "./nlANaGLT.js";
import { s as M } from "./BmMHVVX3.js";
import { i as I } from "./DOjUa9u5.js";
import { e as J } from "./B6azywu7.js";
import { s as Y } from "./BgMuVWsk.js";
import { s as $, a as he } from "./CZv_AhHu.js";
import { p as K } from "./5u5qd9TD.js";
import { b as pe } from "./D5tppINK.js";
import { p as A } from "./uWmgYd3Z.js";
import { B as ee } from "./BEQMYyDu.js";
import "./C8iV2OFO.js";
import { u as be } from "./Bb8ybDgy.js";
import { M as we } from "./AUelbH2M.js";
import { I as N } from "./ChgGgenc.js";
import { u as xe } from "./DwvS5LQk.js";
var ge = fe(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6
            18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75
            21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10"></path></svg>`);
function ke(W, r) {
  let B = A(r, "opacity", 3, 0.9), S = A(r, "width", 3, "1.5rem"), C = A(r, "color", 3, "currentColor");
  var i = ge();
  $(i, "stroke-width", 2), _(() => {
    $(i, "stroke", C()), $(i, "width", S()), $(i, "opacity", B());
  }), s(W, i);
}
var Me = w('<div class="joined svelte-12rw6bl"> </div>'), $e = w("<div><!></div>"), ye = w('<div class="compact svelte-12rw6bl"></div>'), Le = w('<div class="boxes svelte-12rw6bl"><div class="svelte-12rw6bl"></div> <div class="svelte-12rw6bl"></div></div>'), Pe = w("<h3><!></h3> <!> <!>", 1), je = w('<div class="container svelte-12rw6bl"><div class="items svelte-12rw6bl"><!> <div class="edit svelte-12rw6bl"><!></div> <!></div> <div class="label font-label svelte-12rw6bl"><!></div></div>');
function Ke(W, r) {
  _e(r, true);
  let B = A(r, "maxWidth", 3, "467pt"), S = xe(), C = be(), i = D(void 0), H = D(false), z = D(void 0), te = G(() => e(i) && e(i) < 800), ae = G(() => r.items.filter((t) => t.selected).map((t) => t.name).join(", ")), re = G(() => r.items.find((t) => t.selected));
  var y = je();
  let O;
  var E = f(y), Q = f(E);
  {
    var se = (t) => {
      var x = Me(), d = f(x, true);
      l(x), _(() => M(d, e(ae))), s(t, x);
    };
    I(Q, (t) => {
      e(re) && t(se);
    });
  }
  var V = b(Q, 2), le = f(V);
  ee(le, { invisible: true, onclick: () => P(H, true), children: (t, x) => {
    var d = $e(), g = f(d);
    ke(g, { width: "1.2rem" }), l(d), _(() => $(d, "title", C.common.edit)), s(t, d);
  }, $$slots: { default: true } }), l(V);
  var oe = b(V, 2);
  we(oe, { get showModal() {
    return e(H);
  }, set showModal(t) {
    P(H, K(t));
  }, get closeModal() {
    return e(z);
  }, set closeModal(t) {
    P(z, K(t));
  }, children: (t, x) => {
    var d = Pe(), g = F(d), de = f(g);
    Y(de, () => r.children), l(g);
    var T = b(g, 2);
    {
      var ne = (n) => {
        var v = ye();
        J(v, 21, () => r.items, (o) => o.name, (o, u, c) => {
          N(o, { get ariaLabel() {
            return e(u).name;
          }, get checked() {
            return e(u).selected;
          }, set checked(a) {
            e(u).selected = a;
          }, children: (a, U) => {
            j();
            var m = L();
            _(() => M(m, e(u).name)), s(a, m);
          }, $$slots: { default: true } });
        }), l(v), s(n, v);
      }, ve = (n) => {
        var v = Le(), o = f(v);
        J(o, 21, () => r.items, (c) => c.name, (c, a, U) => {
          var m = X(), Z = F(m);
          {
            var q = (h) => {
              N(h, { get ariaLabel() {
                return e(a).name;
              }, get checked() {
                return e(a).selected;
              }, set checked(p) {
                e(a).selected = p;
              }, children: (p, me) => {
                j();
                var k = L();
                _(() => M(k, e(a).name)), s(p, k);
              }, $$slots: { default: true } });
            };
            I(Z, (h) => {
              e(a).selected || h(q);
            });
          }
          s(c, m);
        }), l(o);
        var u = b(o, 2);
        J(u, 21, () => r.items, (c) => c.name, (c, a, U) => {
          var m = X(), Z = F(m);
          {
            var q = (h) => {
              N(h, { get ariaLabel() {
                return e(a).name;
              }, get checked() {
                return e(a).selected;
              }, set checked(p) {
                e(a).selected = p;
              }, children: (p, me) => {
                j();
                var k = L();
                _(() => M(k, e(a).name)), s(p, k);
              }, $$slots: { default: true } });
            };
            I(Z, (h) => {
              e(a).selected && h(q);
            });
          }
          s(c, m);
        }), l(u), l(v), s(n, v);
      };
      I(T, (n) => {
        e(te) ? n(ne) : n(ve, false);
      });
    }
    var ce = b(T, 2);
    ee(ce, { onclick: () => {
      var _a;
      return (_a = e(z)) == null ? void 0 : _a();
    }, children: (n, v) => {
      j();
      var o = L();
      _(() => M(o, S.common.close)), s(n, o);
    }, $$slots: { default: true } }), s(t, d);
  }, $$slots: { default: true } }), l(E);
  var R = b(E, 2), ie = f(R);
  Y(ie, () => r.children), l(R), l(y), _(() => O = he(y, "", O, { "max-width": B() })), pe("innerWidth", (t) => P(i, K(t))), s(W, y), ue();
}
export {
  Ke as S
};
