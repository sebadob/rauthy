import { t as c, a as n, e as le } from "../chunks/DKC5GJ29.js";
import { p as me, f as N, a as ve, t as v, l as i, k as R, j as o, s as r, a7 as ue, c as u, r as d, a8 as de, a9 as ce } from "../chunks/BveBAmlr.js";
import { h as fe, s as l } from "../chunks/CYCba2oX.js";
import { i as A } from "../chunks/D-uYoVwt.js";
import { r as _e, d as pe } from "../chunks/Dql74IOz.js";
import { p as q } from "../chunks/VbPNpVtZ.js";
import { B as ge } from "../chunks/DlLEcmNg.js";
import { v as he } from "../chunks/B21bTIl7.js";
import { I as B } from "../chunks/CCQyB9gY.js";
import { L as $e } from "../chunks/B5qGhNUP.js";
import { M as be } from "../chunks/DaM_1q3c.js";
import { C as xe } from "../chunks/Ci6f1shh.js";
import { u as ye } from "../chunks/8R5My_LO.js";
import { T as we } from "../chunks/CDuWpfYL.js";
import { u as Te } from "../chunks/Bvh621KL.js";
import { T as Ee } from "../chunks/BFgWaswG.js";
import { F as Pe } from "../chunks/BguaNybM.js";
import { u as Ie } from "../chunks/NJ3e7ymm.js";
import { b as z } from "../chunks/BRCxk8by.js";
import { b as Ne } from "../chunks/BO1A6s0c.js";
var Re = c(" <br> <code> </code>", 1), Ae = c('<input type="hidden" name="redirect_uri">'), Le = c('<div class="success svelte-cul8ah"> <br> </div>'), Me = c('<div class="err svelte-cul8ah"> </div>'), Ce = c('<!> <!> <!> <!> <div class="submit svelte-cul8ah"><!></div> <!>', 1), De = c('<div class="container svelte-cul8ah"><div class="domainTxt svelte-cul8ah"><h1> </h1> <!></div> <!></div> <!> <!>', 1), Se = c("<!> <!>", 1);
function tr(G, H) {
  me(H, true);
  let e = ye(), J = Ie(), f = R(""), L = Te("redirect_uri"), M = R(false), m = R(""), C = R(false), K = ce(() => J.get() ? "/auth/v1/dev/register" : "/auth/v1/users/register");
  async function V(_, p) {
    i(C, false), i(m, "");
    let b = p.get("email"), D = p.get("given_name"), x = p.get("pow");
    if (!b || !D || !x) {
      console.error("email, given_name, pow missing");
      return;
    }
    if (o(f) && !b.endsWith(o(f))) {
      i(m, q(e.register.domainErr));
      return;
    }
    let g = { email: b, given_name: D, family_name: p.get("family_name") || void 0, pow: x, redirect_uri: p.get("redirect_uri") || void 0 };
    i(M, true);
    const h = await Ne(_.action, g);
    if (h.error) {
      let $ = h.error.message || "Error";
      $.includes("UNIQUE constraint") ? i(m, "E-Mail is already registered") : i(m, q($));
    } else i(m, ""), i(C, true), L && setTimeout(() => {
      window.location.replace(g.redirect_uri || "/auth/v1/account");
    }, 3e3);
    i(M, false);
  }
  var F = Se();
  fe((_) => {
    v(() => ue.title = (e == null ? void 0 : e.register) || "Register");
  });
  var j = N(F);
  we(j, { id: he, get value() {
    return o(f);
  }, set value(_) {
    i(f, q(_));
  } });
  var X = r(j, 2);
  be(X, { children: (_, p) => {
    xe(_, { children: (b, D) => {
      var x = De(), g = N(x), h = u(g), $ = u(h), Y = u($, true);
      d($);
      var Z = r($, 2);
      {
        var ee = (y) => {
          var S = Re(), w = N(S, true), T = r(w, 2), P = r(T), I = u(P);
          d(P), v(() => {
            l(w, e.register.domainRestricted), l(T, ` ${e.register.domainAllowed ?? ""} `), l(I, `@${o(f) ?? ""}`);
          }), n(y, S);
        };
        A(Z, (y) => {
          o(f) && y(ee);
        });
      }
      d(h);
      var re = r(h, 2);
      Pe(re, { get action() {
        return o(K);
      }, onSubmit: V, withPowAs: "pow", children: (y, S) => {
        var w = Ce(), T = N(w);
        {
          var P = (t) => {
            var a = Ae();
            _e(a), v((s) => pe(a, s), [() => L.get()]), n(t, a);
          };
          A(T, (t) => {
            L.get() && t(P);
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
        var U = r(W, 2), ae = u(U);
        ge(ae, { type: "submit", get isLoading() {
          return o(M);
        }, children: (t, a) => {
          de();
          var s = le();
          v(() => l(s, e.register.register)), n(t, s);
        }, $$slots: { default: true } }), d(U);
        var ie = r(U, 2);
        {
          var oe = (t) => {
            var a = Le(), s = u(a, true), E = r(s, 2);
            d(a), v(() => {
              l(s, e.register.success), l(E, ` ${e.register.emailCheck ?? ""}`);
            }), n(t, a);
          }, se = (t, a) => {
            {
              var s = (E) => {
                var k = Me(), ne = u(k, true);
                d(k), v(() => l(ne, o(m))), n(E, k);
              };
              A(t, (E) => {
                o(m) && E(s);
              }, a);
            }
          };
          A(ie, (t) => {
            o(C) ? t(oe) : t(se, false);
          });
        }
        n(y, w);
      }, $$slots: { default: true } }), d(g);
      var O = r(g, 2);
      Ee(O, { absolute: true });
      var te = r(O, 2);
      $e(te, { absolute: true }), v(() => l(Y, e.register.userReg)), n(b, x);
    } });
  } }), n(G, F), ve();
}
export {
  tr as component
};
