import { n as me, a as s, t as w, e as y, d as U } from "./BxmJRzoY.js";
import { t as _, p as ue, j as D, a4 as F, k as e, s as b, c as u, l as P, f as G, a as _e, r as l, a3 as j } from "./w0HvPX0p.js";
import { s as M } from "./BzP2S3Z_.js";
import { i as I } from "./iO9_dPNE.js";
import { e as J } from "./S81raU5Y.js";
import { s as X } from "./DM69BKKN.js";
import { s as $, a as fe } from "./BdbQ6g_y.js";
import { b as he } from "./CJLp5kwW.js";
import { p as A } from "./C6GSeq7M.js";
import { B as Y } from "./C8YTstTD.js";
import "./BnlWDKfj.js";
import { u as pe } from "./DHOKTGcE.js";
import { M as be } from "./CEj6JA72.js";
import { I as K } from "./ScYc5fRW.js";
import { u as we } from "./0cG6LBdy.js";
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
function Ge(W, r) {
  ue(r, true);
  let B = A(r, "maxWidth", 3, "467pt"), S = we(), C = pe(), i = D(void 0), H = D(false), z = D(void 0), ee = F(() => e(i) && e(i) < 800), te = F(() => r.items.filter((t) => t.selected).map((t) => t.name).join(", ")), ae = F(() => r.items.find((t) => t.selected));
  var L = Pe();
  let N;
  var E = u(L), O = u(E);
  {
    var re = (t) => {
      var x = ke(), d = u(x, true);
      l(x), _(() => M(d, e(te))), s(t, x);
    };
    I(O, (t) => {
      e(ae) && t(re);
    });
  }
  var V = b(O, 2), se = u(V);
  Y(se, { invisible: true, onclick: () => P(H, true), children: (t, x) => {
    var d = Me(), g = u(d);
    ge(g, { width: "1.2rem" }), l(d), _(() => $(d, "title", C.common.edit)), s(t, d);
  }, $$slots: { default: true } }), l(V);
  var le = b(V, 2);
  be(le, { get showModal() {
    return e(H);
  }, set showModal(t) {
    P(H, t, true);
  }, get closeModal() {
    return e(z);
  }, set closeModal(t) {
    P(z, t, true);
  }, children: (t, x) => {
    var d = ye(), g = G(d), ie = u(g);
    X(ie, () => r.children), l(g);
    var R = b(g, 2);
    {
      var de = (v) => {
        var n = $e();
        J(n, 21, () => r.items, (o) => o.name, (o, f, c) => {
          K(o, { get ariaLabel() {
            return e(f).name;
          }, get checked() {
            return e(f).selected;
          }, set checked(a) {
            e(f).selected = a;
          }, children: (a, T) => {
            j();
            var m = y();
            _(() => M(m, e(f).name)), s(a, m);
          }, $$slots: { default: true } });
        }), l(n), s(v, n);
      }, ve = (v) => {
        var n = Le(), o = u(n);
        J(o, 21, () => r.items, (c) => c.name, (c, a, T) => {
          var m = U(), Z = G(m);
          {
            var q = (h) => {
              K(h, { get ariaLabel() {
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
        }), l(o);
        var f = b(o, 2);
        J(f, 21, () => r.items, (c) => c.name, (c, a, T) => {
          var m = U(), Z = G(m);
          {
            var q = (h) => {
              K(h, { get ariaLabel() {
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
        }), l(f), l(n), s(v, n);
      };
      I(R, (v) => {
        e(ee) ? v(de) : v(ve, false);
      });
    }
    var ne = b(R, 2);
    Y(ne, { onclick: () => {
      var _a;
      return (_a = e(z)) == null ? void 0 : _a();
    }, children: (v, n) => {
      j();
      var o = y();
      _(() => M(o, S.common.close)), s(v, o);
    }, $$slots: { default: true } }), s(t, d);
  }, $$slots: { default: true } }), l(E);
  var Q = b(E, 2), oe = u(Q);
  X(oe, () => r.children), l(Q), l(L), _(() => N = fe(L, "", N, { "max-width": B() })), he("innerWidth", (t) => P(i, t, true)), s(W, L), _e();
}
export {
  Ge as S
};
