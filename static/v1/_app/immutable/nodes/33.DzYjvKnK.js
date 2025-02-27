import { t as c, a as n, e as me, d as ve } from "../chunks/D8nUqfqE.js";
import { p as ue, f as E, a as de, t as v, l as o, k as N, j as i, s as r, a6 as ce, c as u, r as d, a7 as fe, a9 as _e } from "../chunks/D-nwkJyM.js";
import { h as pe, s as l } from "../chunks/DmLjbmU6.js";
import { i as R } from "../chunks/C3XMhfdI.js";
import { r as ge, b as he } from "../chunks/D_HYGYLR.js";
import { p as B } from "../chunks/BiI21XkS.js";
import { B as $e } from "../chunks/BUAPoI3e.js";
import { w as be } from "../chunks/DppGgfa0.js";
import { I as F } from "../chunks/BE8gxf21.js";
import { L as xe } from "../chunks/DJrws3yD.js";
import { M as ye } from "../chunks/C-ghU9Ac.js";
import { C as we } from "../chunks/BQuV3eOZ.js";
import { u as Te } from "../chunks/CMjKUQkH.js";
import { T as Ee } from "../chunks/Bzn2dU6j.js";
import { u as Pe } from "../chunks/C6Ehfmfa.js";
import { T as Ie } from "../chunks/DmGHSNVM.js";
import { F as Ne } from "../chunks/BgEKkuL8.js";
import { u as Re } from "../chunks/BO-Hjil0.js";
import { b as G } from "../chunks/BRCxk8by.js";
import { b as Ae } from "../chunks/emZtalxW.js";
var Le = c(" <br> <code> </code>", 1), Me = c('<input type="hidden" name="redirect_uri">'), Ce = c('<div class="success svelte-cul8ah"> <br> </div>'), De = c('<div class="err svelte-cul8ah"> </div>'), Se = c('<!> <!> <!> <!> <div class="submit svelte-cul8ah"><!></div> <!>', 1), Ue = c('<div class="container svelte-cul8ah"><div class="domainTxt svelte-cul8ah"><h1> </h1> <!></div> <!></div> <!> <!>', 1), ke = c("<!> <!>", 1);
function or(H, J) {
  ue(J, true);
  let e = Te(), K = Re(), f = N(""), A = Pe("redirect_uri"), L = N(false), m = N(""), M = N(false), V = _e(() => K.get() ? "/auth/v1/dev/register" : "/auth/v1/users/register");
  async function X(_, p) {
    o(M, false), o(m, "");
    let b = p.get("email"), C = p.get("given_name"), x = p.get("pow");
    if (!b || !C || !x) {
      console.error("email, given_name, pow missing");
      return;
    }
    if (i(f) && !b.endsWith(i(f))) {
      o(m, B(e.register.domainErr));
      return;
    }
    let g = { email: b, given_name: C, family_name: p.get("family_name") || void 0, pow: x, redirect_uri: p.get("redirect_uri") || void 0 };
    o(L, true);
    const h = await Ae(_.action, g);
    if (h.error) {
      let $ = h.error.message || "Error";
      $.includes("UNIQUE constraint") ? o(m, "E-Mail is already registered") : o(m, B($));
    } else o(m, ""), o(M, true), A && setTimeout(() => {
      window.location.replace(g.redirect_uri || "/auth/v1/account");
    }, 3e3);
    o(L, false);
  }
  var j = ke();
  pe((_) => {
    v(() => ce.title = (e == null ? void 0 : e.register) || "Register");
  });
  var O = E(j);
  Ee(O, { id: be, get value() {
    return i(f);
  }, set value(_) {
    o(f, B(_));
  } });
  var Y = r(O, 2);
  ye(Y, { children: (_, p) => {
    we(_, { children: (b, C) => {
      var x = Ue(), g = E(x), h = u(g), $ = u(h), Z = u($, true);
      d($);
      var ee = r($, 2);
      {
        var re = (y) => {
          var D = Le(), w = E(D, true), T = r(w, 2), P = r(T), I = u(P);
          d(P), v(() => {
            l(w, e.register.domainRestricted), l(T, ` ${e.register.domainAllowed ?? ""} `), l(I, `@${i(f) ?? ""}`);
          }), n(y, D);
        };
        R(ee, (y) => {
          i(f) && y(re);
        });
      }
      d(h);
      var te = r(h, 2);
      Ne(te, { get action() {
        return i(V);
      }, onSubmit: X, withPowAs: "pow", children: (y, D) => {
        var w = Se(), T = E(w);
        {
          var P = (t) => {
            var a = Me();
            ge(a), v((s) => he(a, s), [() => A.get()]), n(t, a);
          };
          R(T, (t) => {
            A.get() && t(P);
          });
        }
        var I = r(T, 2);
        F(I, { typ: "email", name: "email", autocomplete: "email", get label() {
          return e.common.email;
        }, get placeholder() {
          return e.common.email;
        }, required: true });
        var W = r(I, 2);
        F(W, { name: "given_name", autocomplete: "given-name", get label() {
          return e.account.givenName;
        }, get placeholder() {
          return e.account.givenName;
        }, pattern: G, required: true });
        var z = r(W, 2);
        F(z, { name: "family_name", autocomplete: "family-name", get label() {
          return e.account.familyName;
        }, get placeholder() {
          return e.account.familyName;
        }, pattern: G });
        var S = r(z, 2), oe = u(S);
        $e(oe, { type: "submit", get isLoading() {
          return i(L);
        }, children: (t, a) => {
          fe();
          var s = me();
          v(() => l(s, e.register.register)), n(t, s);
        }, $$slots: { default: true } }), d(S);
        var ie = r(S, 2);
        {
          var se = (t) => {
            var a = Ce(), s = u(a, true), U = r(s, 2);
            d(a), v(() => {
              l(s, e.register.success), l(U, ` ${e.register.emailCheck ?? ""}`);
            }), n(t, a);
          }, ne = (t) => {
            var a = ve(), s = E(a);
            {
              var U = (k) => {
                var q = De(), le = u(q, true);
                d(q), v(() => l(le, i(m))), n(k, q);
              };
              R(s, (k) => {
                i(m) && k(U);
              }, true);
            }
            n(t, a);
          };
          R(ie, (t) => {
            i(M) ? t(se) : t(ne, false);
          });
        }
        n(y, w);
      }, $$slots: { default: true } }), d(g);
      var Q = r(g, 2);
      Ie(Q, { absolute: true });
      var ae = r(Q, 2);
      xe(ae, { absolute: true }), v(() => l(Z, e.register.userReg)), n(b, x);
    } });
  } }), n(H, j), de();
}
export {
  or as component
};
