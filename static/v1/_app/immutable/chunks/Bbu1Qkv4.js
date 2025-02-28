import { n as me, a as s, t as w, e as y, d as U } from "./BH6NCLk-.js";
import { t as _, p as fe, s as b, c as f, l as P, k as D, f as F, j as e, a as _e, r as o, a7 as j, a9 as G } from "./CvlvO1XB.js";
import { s as M } from "./CTI4QPiR.js";
import { i as I } from "./BUO_AUgz.js";
import { e as J } from "./BpWRzPRQ.js";
import { s as X } from "./-T201g_q.js";
import { s as $ } from "./BMbqVy6X.js";
import { B as Y, a as ue } from "./DMkkW5Nn.js";
import { p as K } from "./Wh68IIk2.js";
import { b as he } from "./7lxiEjMQ.js";
import { p as A } from "./C6SR4G2t.js";
import "./BmpEzKyJ.js";
import { u as pe } from "./D8mHI_K9.js";
import { M as be } from "./BHXYRhv2.js";
import { I as N } from "./DTR8xafZ.js";
import { u as we } from "./BQ1-pLIs.js";
var xe = me(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6
            18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75
            21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10"></path></svg>`);
function ge(W, r) {
  let B = A(r, "opacity", 3, 0.9), S = A(r, "width", 3, "1.5rem"), C = A(r, "color", 3, "currentColor");
  var i = xe();
  $(i, "stroke-width", 2), _(() => {
    $(i, "stroke", C()), $(i, "width", S()), $(i, "opacity", B());
  }), s(W, i);
}
var ke = w('<div class="joined svelte-12rw6bl"> </div>'), Me = w("<div><!></div>"), $e = w('<div class="compact svelte-12rw6bl"></div>'), Le = w('<div class="boxes svelte-12rw6bl"><div class="svelte-12rw6bl"></div> <div class="svelte-12rw6bl"></div></div>'), ye = w("<h3><!></h3> <!> <!>", 1), Pe = w('<div class="container svelte-12rw6bl"><div class="items svelte-12rw6bl"><!> <div class="edit svelte-12rw6bl"><!></div> <!></div> <div class="label font-label svelte-12rw6bl"><!></div></div>');
function Je(W, r) {
  fe(r, true);
  let B = A(r, "maxWidth", 3, "467pt"), S = we(), C = pe(), i = D(void 0), H = D(false), z = D(void 0), ee = G(() => e(i) && e(i) < 800), te = G(() => r.items.filter((t) => t.selected).map((t) => t.name).join(", ")), ae = G(() => r.items.find((t) => t.selected));
  var L = Pe(), E = f(L), O = f(E);
  {
    var re = (t) => {
      var x = ke(), d = f(x, true);
      o(x), _(() => M(d, e(te))), s(t, x);
    };
    I(O, (t) => {
      e(ae) && t(re);
    });
  }
  var V = b(O, 2), se = f(V);
  Y(se, { invisible: true, onclick: () => P(H, true), children: (t, x) => {
    var d = Me(), g = f(d);
    ge(g, { width: "1.2rem" }), o(d), _(() => $(d, "title", C.common.edit)), s(t, d);
  }, $$slots: { default: true } }), o(V);
  var oe = b(V, 2);
  be(oe, { get showModal() {
    return e(H);
  }, set showModal(t) {
    P(H, K(t));
  }, get closeModal() {
    return e(z);
  }, set closeModal(t) {
    P(z, K(t));
  }, children: (t, x) => {
    var d = ye(), g = F(d), ie = f(g);
    X(ie, () => r.children), o(g);
    var R = b(g, 2);
    {
      var de = (n) => {
        var v = $e();
        J(v, 21, () => r.items, (l) => l.name, (l, u, c) => {
          N(l, { get ariaLabel() {
            return e(u).name;
          }, get checked() {
            return e(u).selected;
          }, set checked(a) {
            e(u).selected = a;
          }, children: (a, T) => {
            j();
            var m = y();
            _(() => M(m, e(u).name)), s(a, m);
          }, $$slots: { default: true } });
        }), o(v), s(n, v);
      }, ne = (n) => {
        var v = Le(), l = f(v);
        J(l, 21, () => r.items, (c) => c.name, (c, a, T) => {
          var m = U(), Z = F(m);
          {
            var q = (h) => {
              N(h, { get ariaLabel() {
                return e(a).name;
              }, get checked() {
                return e(a).selected;
              }, set checked(p) {
                e(a).selected = p;
              }, children: (p, ce) => {
                j();
                var k = y();
                _(() => M(k, e(a).name)), s(p, k);
              }, $$slots: { default: true } });
            };
            I(Z, (h) => {
              e(a).selected || h(q);
            });
          }
          s(c, m);
        }), o(l);
        var u = b(l, 2);
        J(u, 21, () => r.items, (c) => c.name, (c, a, T) => {
          var m = U(), Z = F(m);
          {
            var q = (h) => {
              N(h, { get ariaLabel() {
                return e(a).name;
              }, get checked() {
                return e(a).selected;
              }, set checked(p) {
                e(a).selected = p;
              }, children: (p, ce) => {
                j();
                var k = y();
                _(() => M(k, e(a).name)), s(p, k);
              }, $$slots: { default: true } });
            };
            I(Z, (h) => {
              e(a).selected && h(q);
            });
          }
          s(c, m);
        }), o(u), o(v), s(n, v);
      };
      I(R, (n) => {
        e(ee) ? n(de) : n(ne, false);
      });
    }
    var ve = b(R, 2);
    Y(ve, { onclick: () => {
      var _a;
      return (_a = e(z)) == null ? void 0 : _a();
    }, children: (n, v) => {
      j();
      var l = y();
      _(() => M(l, S.common.close)), s(n, l);
    }, $$slots: { default: true } }), s(t, d);
  }, $$slots: { default: true } }), o(E);
  var Q = b(E, 2), le = f(Q);
  X(le, () => r.children), o(Q), o(L), _(() => ue(L, "max-width", B())), he("innerWidth", (t) => P(i, K(t))), s(W, L), _e();
}
export {
  Je as S
};
