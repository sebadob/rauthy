import { t as c, a as n, e as me } from "../chunks/DKC5GJ29.js";
import { p as le, f as N, a as ve, t as v, l as i, k as q, j as o, s as r, a7 as de, c as d, r as u, a8 as ue, a9 as ce } from "../chunks/BveBAmlr.js";
import { h as fe, s as m } from "../chunks/CYCba2oX.js";
import { i as R } from "../chunks/D-uYoVwt.js";
import { r as _e, d as pe } from "../chunks/Dql74IOz.js";
import { p as k } from "../chunks/VbPNpVtZ.js";
import { B as ge } from "../chunks/DlLEcmNg.js";
import { v as he } from "../chunks/B21bTIl7.js";
import { I as B } from "../chunks/CCQyB9gY.js";
import { L as xe } from "../chunks/DNYXBAk_.js";
import { M as $e } from "../chunks/DaM_1q3c.js";
import { C as we } from "../chunks/Ci6f1shh.js";
import { u as be } from "../chunks/8R5My_LO.js";
import { T as ye } from "../chunks/CDuWpfYL.js";
import { u as Te } from "../chunks/BdFU-gNS.js";
import { T as Ee } from "../chunks/BFgWaswG.js";
import { F as Pe } from "../chunks/BguaNybM.js";
import { u as Ie } from "../chunks/NJ3e7ymm.js";
import { b as z } from "../chunks/BRCxk8by.js";
import { b as Ne } from "../chunks/BO1A6s0c.js";
var qe = c(" <br> <code> </code>", 1), Re = c('<input type="hidden" name="redirect_uri">'), Ae = c('<div class="success svelte-f6xqw3"> <br> </div>'), Le = c('<div class="err svelte-f6xqw3"> </div>'), Me = c('<!> <!> <!> <!> <div class="submit svelte-f6xqw3"><!></div> <!>', 1), Ce = c('<div class="container svelte-f6xqw3"><div class="domainTxt svelte-f6xqw3"><h1> </h1> <!></div> <!></div> <!> <!>', 1), De = c("<!> <!>", 1);
function tr(G, H) {
  le(H, true);
  let e = be(), J = Ie(), f = q(""), A = Te("redirect_uri"), L = q(false), l = q(""), M = q(false), K = ce(() => J.get() ? "/auth/v1/dev/register" : "/auth/v1/users/register");
  async function V(_, p) {
    i(M, false), i(l, "");
    let $ = p.get("email"), C = p.get("given_name"), w = p.get("pow");
    if (!$ || !C || !w) {
      console.error("email, given_name, pow missing");
      return;
    }
    if (o(f) && !$.endsWith(o(f))) {
      i(l, k(e.register.domainErr));
      return;
    }
    let g = { email: $, given_name: C, family_name: p.get("family_name") || void 0, pow: w, redirect_uri: p.get("redirect_uri") || void 0 };
    i(L, true);
    const h = await Ne(_.action, g);
    if (h.error) {
      let x = h.error.message || "Error";
      x.includes("UNIQUE constraint") ? i(l, "E-Mail is already registered") : i(l, k(x));
    } else i(l, ""), i(M, true), A && setTimeout(() => {
      window.location.replace(g.redirect_uri || "/auth/v1/account");
    }, 3e3);
    i(L, false);
  }
  var F = De();
  fe((_) => {
    v(() => de.title = (e == null ? void 0 : e.register) || "Register");
  });
  var j = N(F);
  ye(j, { id: he, get value() {
    return o(f);
  }, set value(_) {
    i(f, k(_));
  } });
  var X = r(j, 2);
  $e(X, { children: (_, p) => {
    we(_, { children: ($, C) => {
      var w = Ce(), g = N(w), h = d(g), x = d(h), Y = d(x, true);
      u(x);
      var Z = r(x, 2);
      {
        var ee = (b) => {
          var D = qe(), y = N(D, true), T = r(y, 2), P = r(T), I = d(P);
          u(P), v(() => {
            m(y, e.register.domainRestricted), m(T, ` ${e.register.domainAllowed ?? ""} `), m(I, `@${o(f) ?? ""}`);
          }), n(b, D);
        };
        R(Z, (b) => {
          o(f) && b(ee);
        });
      }
      u(h);
      var re = r(h, 2);
      Pe(re, { get action() {
        return o(K);
      }, onSubmit: V, withPowAs: "pow", children: (b, D) => {
        var y = Me(), T = N(y);
        {
          var P = (t) => {
            var a = Re();
            _e(a), v((s) => pe(a, s), [() => A.get()]), n(t, a);
          };
          R(T, (t) => {
            A.get() && t(P);
          });
        }
        var I = r(T, 2);
        B(I, { typ: "email", name: "email", autocomplete: "email", get label() {
          return e.common.email;
        }, get placeholder() {
          return e.common.email;
        }, required: true });
        var Q = r(I, 2);
        B(Q, { name: "given_name", autocomplete: "given-name", get label() {
          return e.account.givenName;
        }, get placeholder() {
          return e.account.givenName;
        }, pattern: z, required: true });
        var W = r(Q, 2);
        B(W, { name: "family_name", autocomplete: "family-name", get label() {
          return e.account.familyName;
        }, get placeholder() {
          return e.account.familyName;
        }, pattern: z });
        var S = r(W, 2), ae = d(S);
        ge(ae, { type: "submit", get isLoading() {
          return o(L);
        }, children: (t, a) => {
          ue();
          var s = me();
          v(() => m(s, e.register.register)), n(t, s);
        }, $$slots: { default: true } }), u(S);
        var ie = r(S, 2);
        {
          var oe = (t) => {
            var a = Ae(), s = d(a, true), E = r(s, 2);
            u(a), v(() => {
              m(s, e.register.success), m(E, ` ${e.register.emailCheck ?? ""}`);
            }), n(t, a);
          }, se = (t, a) => {
            {
              var s = (E) => {
                var U = Le(), ne = d(U, true);
                u(U), v(() => m(ne, o(l))), n(E, U);
              };
              R(t, (E) => {
                o(l) && E(s);
              }, a);
            }
          };
          R(ie, (t) => {
            o(M) ? t(oe) : t(se, false);
          });
        }
        n(b, y);
      }, $$slots: { default: true } }), u(g);
      var O = r(g, 2);
      Ee(O, { absolute: true });
      var te = r(O, 2);
      xe(te, { absolute: true }), v(() => m(Y, e.register.userReg)), n($, w);
    } });
  } }), n(G, F), ve();
}
export {
  tr as component
};
