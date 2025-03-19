import { n as fe, a as s, t as w, e as L, d as X } from "./DLBGyKVC.js";
import { t as _, p as _e, s as b, c as f, l as P, k as D, f as F, j as e, a as he, r as o, a8 as j, a9 as G } from "./CmQi0fbH.js";
import { s as M } from "./BjaYyaa_.js";
import { i as I } from "./C6bK2EJJ.js";
import { e as J } from "./YQCw2eEa.js";
import { s as Y } from "./71gCR-8F.js";
import { s as $, a as ue } from "./DOlUUCkJ.js";
import { p as K } from "./B_ggA-0N.js";
import { b as pe } from "./DzJ8OZ_u.js";
import { p as A } from "./DNJm3-SG.js";
import { B as ee } from "./DPLO-ozG.js";
import "./ibGTLHQD.js";
import { u as be } from "./CZf6fJph.js";
import { M as we } from "./CNTKxNWA.js";
import { I as N } from "./CFiOSVzB.js";
import { u as xe } from "./DGTOa5g8.js";
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
      o(x), _(() => M(d, e(ae))), s(t, x);
    };
    I(Q, (t) => {
      e(re) && t(se);
    });
  }
  var V = b(Q, 2), oe = f(V);
  ee(oe, { invisible: true, onclick: () => P(H, true), children: (t, x) => {
    var d = $e(), g = f(d);
    ke(g, { width: "1.2rem" }), o(d), _(() => $(d, "title", C.common.edit)), s(t, d);
  }, $$slots: { default: true } }), o(V);
  var le = b(V, 2);
  we(le, { get showModal() {
    return e(H);
  }, set showModal(t) {
    P(H, K(t));
  }, get closeModal() {
    return e(z);
  }, set closeModal(t) {
    P(z, K(t));
  }, children: (t, x) => {
    var d = Pe(), g = F(d), de = f(g);
    Y(de, () => r.children), o(g);
    var T = b(g, 2);
    {
      var ve = (v) => {
        var n = ye();
        J(n, 21, () => r.items, (l) => l.name, (l, h, c) => {
          N(l, { get ariaLabel() {
            return e(h).name;
          }, get checked() {
            return e(h).selected;
          }, set checked(a) {
            e(h).selected = a;
          }, children: (a, U) => {
            j();
            var m = L();
            _(() => M(m, e(h).name)), s(a, m);
          }, $$slots: { default: true } });
        }), o(n), s(v, n);
      }, ne = (v) => {
        var n = Le(), l = f(n);
        J(l, 21, () => r.items, (c) => c.name, (c, a, U) => {
          var m = X(), Z = F(m);
          {
            var q = (u) => {
              N(u, { get ariaLabel() {
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
            I(Z, (u) => {
              e(a).selected || u(q);
            });
          }
          s(c, m);
        }), o(l);
        var h = b(l, 2);
        J(h, 21, () => r.items, (c) => c.name, (c, a, U) => {
          var m = X(), Z = F(m);
          {
            var q = (u) => {
              N(u, { get ariaLabel() {
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
            I(Z, (u) => {
              e(a).selected && u(q);
            });
          }
          s(c, m);
        }), o(h), o(n), s(v, n);
      };
      I(T, (v) => {
        e(te) ? v(ve) : v(ne, false);
      });
    }
    var ce = b(T, 2);
    ee(ce, { onclick: () => {
      var _a;
      return (_a = e(z)) == null ? void 0 : _a();
    }, children: (v, n) => {
      j();
      var l = L();
      _(() => M(l, S.common.close)), s(v, l);
    }, $$slots: { default: true } }), s(t, d);
  }, $$slots: { default: true } }), o(E);
  var R = b(E, 2), ie = f(R);
  Y(ie, () => r.children), o(R), o(y), _(() => O = ue(y, "", O, { "max-width": B() })), pe("innerWidth", (t) => P(i, K(t))), s(W, y), he();
}
export {
  Ke as S
};
