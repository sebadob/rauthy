import { t as c, a as n, e as ne } from "../chunks/BxmJRzoY.js";
import { p as le, j as N, a4 as me, f as q, a as ve, t as v, l as i, k as o, s as r, a2 as ue, c as u, r as d, a3 as de } from "../chunks/w0HvPX0p.js";
import { h as ce, s as l } from "../chunks/BzP2S3Z_.js";
import { i as R } from "../chunks/iO9_dPNE.js";
import { r as fe, d as _e } from "../chunks/BdbQ6g_y.js";
import { B as pe } from "../chunks/C8YTstTD.js";
import { v as ge } from "../chunks/B21bTIl7.js";
import { I as k } from "../chunks/Q4PIg3iI.js";
import { L as he } from "../chunks/D9_8lD-s.js";
import { M as xe } from "../chunks/DKM0QPz5.js";
import { C as $e } from "../chunks/QNragXLc.js";
import { u as we } from "../chunks/0cG6LBdy.js";
import { T as be } from "../chunks/BdDmaO9S.js";
import { u as ye } from "../chunks/Cy3hLAXJ.js";
import { T as Te } from "../chunks/B1f0afjj.js";
import { F as Ee } from "../chunks/CDe-qvZi.js";
import { u as Pe } from "../chunks/F_Qf1tHt.js";
import { b as W } from "../chunks/gfDO7tLr.js";
import { b as Ie } from "../chunks/BO1A6s0c.js";
var Ne = c(" <br> <code> </code>", 1), qe = c('<input type="hidden" name="redirect_uri">'), Re = c('<div class="success svelte-f6xqw3"> <br> </div>'), Ae = c('<div class="err svelte-f6xqw3"> </div>'), Le = c('<!> <!> <!> <!> <div class="submit svelte-f6xqw3"><!></div> <!>', 1), Me = c('<div class="container svelte-f6xqw3"><div class="domainTxt svelte-f6xqw3"><h1> </h1> <!></div> <!></div> <!> <!>', 1), Ce = c("<!> <!>", 1);
function er(z, G) {
  le(G, true);
  let e = we(), H = Pe(), f = N(""), A = ye("redirect_uri"), L = N(false), m = N(""), M = N(false), J = me(() => H.get() ? "/auth/v1/dev/register" : "/auth/v1/users/register");
  async function K(_, p) {
    i(M, false), i(m, "");
    let $ = p.get("email"), C = p.get("given_name"), w = p.get("pow");
    if (!$ || !C || !w) {
      console.error("email, given_name, pow missing");
      return;
    }
    if (o(f) && !$.endsWith(o(f))) {
      i(m, e.register.domainErr, true);
      return;
    }
    let g = { email: $, given_name: C, family_name: p.get("family_name") || void 0, pow: w, redirect_uri: p.get("redirect_uri") || void 0 };
    i(L, true);
    const h = await Ie(_.action, g);
    if (h.error) {
      let x = h.error.message || "Error";
      x.includes("UNIQUE constraint") ? i(m, "E-Mail is already registered") : i(m, x, true);
    } else i(m, ""), i(M, true), A && setTimeout(() => {
      window.location.replace(g.redirect_uri || "/auth/v1/account");
    }, 3e3);
    i(L, false);
  }
  var B = Ce();
  ce((_) => {
    v(() => ue.title = (e == null ? void 0 : e.register) || "Register");
  });
  var F = q(B);
  be(F, { id: ge, get value() {
    return o(f);
  }, set value(_) {
    i(f, _, true);
  } });
  var V = r(F, 2);
  xe(V, { children: (_, p) => {
    $e(_, { children: ($, C) => {
      var w = Me(), g = q(w), h = u(g), x = u(h), X = u(x, true);
      d(x);
      var Y = r(x, 2);
      {
        var Z = (b) => {
          var D = Ne(), y = q(D, true), T = r(y, 2), P = r(T), I = u(P);
          d(P), v(() => {
            l(y, e.register.domainRestricted), l(T, ` ${e.register.domainAllowed ?? ""} `), l(I, `@${o(f) ?? ""}`);
          }), n(b, D);
        };
        R(Y, (b) => {
          o(f) && b(Z);
        });
      }
      d(h);
      var ee = r(h, 2);
      Ee(ee, { get action() {
        return o(J);
      }, onSubmit: K, withPowAs: "pow", children: (b, D) => {
        var y = Le(), T = q(y);
        {
          var P = (t) => {
            var a = qe();
            fe(a), v((s) => _e(a, s), [() => A.get()]), n(t, a);
          };
          R(T, (t) => {
            A.get() && t(P);
          });
        }
        var I = r(T, 2);
        k(I, { typ: "email", name: "email", autocomplete: "email", get label() {
          return e.common.email;
        }, get placeholder() {
          return e.common.email;
        }, required: true });
        var O = r(I, 2);
        k(O, { name: "given_name", autocomplete: "given-name", get label() {
          return e.account.givenName;
        }, get placeholder() {
          return e.account.givenName;
        }, pattern: W, required: true });
        var Q = r(O, 2);
        k(Q, { name: "family_name", autocomplete: "family-name", get label() {
          return e.account.familyName;
        }, get placeholder() {
          return e.account.familyName;
        }, pattern: W });
        var S = r(Q, 2), te = u(S);
        pe(te, { type: "submit", get isLoading() {
          return o(L);
        }, children: (t, a) => {
          de();
          var s = ne();
          v(() => l(s, e.register.register)), n(t, s);
        }, $$slots: { default: true } }), d(S);
        var ae = r(S, 2);
        {
          var ie = (t) => {
            var a = Re(), s = u(a, true), E = r(s, 2);
            d(a), v(() => {
              l(s, e.register.success), l(E, ` ${e.register.emailCheck ?? ""}`);
            }), n(t, a);
          }, oe = (t, a) => {
            {
              var s = (E) => {
                var U = Ae(), se = u(U, true);
                d(U), v(() => l(se, o(m))), n(E, U);
              };
              R(t, (E) => {
                o(m) && E(s);
              }, a);
            }
          };
          R(ae, (t) => {
            o(M) ? t(ie) : t(oe, false);
          });
        }
        n(b, y);
      }, $$slots: { default: true } }), d(g);
      var j = r(g, 2);
      Te(j, { absolute: true });
      var re = r(j, 2);
      he(re, { absolute: true }), v(() => l(X, e.register.userReg)), n($, w);
    } });
  } }), n(z, B), ve();
}
export {
  er as component
};
