import { t as v, a as i, e as A } from "../chunks/DKC5GJ29.js";
import { p as ie, a6 as ne, f as M, a as de, s as t, l as s, k as l, c as r, r as o, j as a, a8 as P, t as le } from "../chunks/BveBAmlr.js";
import { s as E } from "../chunks/CYCba2oX.js";
import { i as F } from "../chunks/D-uYoVwt.js";
import { p as h } from "../chunks/VbPNpVtZ.js";
import { B as K } from "../chunks/DlLEcmNg.js";
import { I as N } from "../chunks/CCQyB9gY.js";
import { C as z } from "../chunks/5J717Kcr.js";
import { M as ve } from "../chunks/DaM_1q3c.js";
import { C as ce } from "../chunks/Ci6f1shh.js";
import { a as G } from "../chunks/BRCxk8by.js";
import { F as pe } from "../chunks/BguaNybM.js";
import { g as fe } from "../chunks/B21bTIl7.js";
var ue = v("<br>-> enable idp registration:<br> <code>chrome://flags/#fedcm-idp-registration</code>", 1), me = v("<p><!> <!> <!></p>"), ge = v('<p><!></p> <!> <div class="row svelte-wuqbvo"><b>Logged In:</b> <div class="check svelte-wuqbvo"><!></div></div>', 1), _e = v('<div class="flex-col"><h1>FedCM Testing</h1> <div class="row svelte-wuqbvo"><div><b>FedCM supported:</b></div> <div class="check svelte-wuqbvo"><!></div></div> <!> <!></div>'), be = v('<div class="row svelte-wuqbvo"><b>Credential Type:</b> <span> </span></div> <div class="token svelte-wuqbvo"> </div>', 1), he = v("<!> <!>", 1);
function Ue(H, J) {
  ie(J, true);
  let $ = l(false), k = l(false), w = l(""), q = l(""), g = l("fedcm"), c = l("any");
  ne(async () => {
    s(c, `${window.location.origin}/auth/v1/fed_cm/config`), s(g, `${window.location.origin}/auth/v1/fed_cm/client_config`), (await fetch("/auth/v1/fed_cm/status")).status === 200 ? (console.log("FedCM status is: logged-in"), s(k, true)) : console.log("FedCM status is: logged-out"), window.IdentityProvider && IdentityProvider.register != null ? (console.log("FedCM is supported"), s($, true)) : console.error("FedCM is not supported");
  });
  async function O() {
    try {
      let e = await navigator.credentials.get({ identity: { mode: "button", providers: [{ configURL: a(c), clientId: a(g), nonce: fe(48) }] } });
      console.log(e), s(q, h(e.type)), s(w, h(e.token)), s(k, true);
    } catch (e) {
      console.error("FedCM credentials error: " + e);
    }
  }
  function Q() {
    IdentityProvider.register(a(c));
  }
  var L = he(), R = M(L);
  ve(R, { children: (e, C) => {
    ce(e, { children: (p, y) => {
      var f = _e(), d = t(r(f), 2), _ = t(r(d), 2), X = r(_);
      z(X, {}), o(_), o(d);
      var T = t(d, 2);
      {
        var Y = (n) => {
          var b = ue();
          P(4), i(n, b);
        };
        F(T, (n) => {
          a($) || n(Y);
        });
      }
      var Z = t(T, 2);
      {
        var ee = (n) => {
          var b = ge(), I = M(b), te = r(I);
          K(te, { onclick: Q, level: 2, children: (x, oe) => {
            P();
            var u = A("Register IdP");
            i(x, u);
          }, $$slots: { default: true } }), o(I);
          var U = t(I, 2);
          pe(U, { action: "", onSubmit: O, children: (x, oe) => {
            var u = me(), S = r(u);
            N(S, { autocomplete: "off", label: "Client ID", placeholder: "Client ID", required: true, pattern: G, get value() {
              return a(g);
            }, set value(m) {
              s(g, h(m));
            } });
            var j = t(S, 2);
            N(j, { autocomplete: "off", label: "Config URL", placeholder: "Config URL", pattern: G, get value() {
              return a(c);
            }, set value(m) {
              s(c, h(m));
            } });
            var ae = t(j, 2);
            K(ae, { type: "submit", children: (m, $e) => {
              P();
              var se = A("Login");
              i(m, se);
            }, $$slots: { default: true } }), o(u), i(x, u);
          }, $$slots: { default: true } });
          var B = t(U, 2), D = t(r(B), 2), re = r(D);
          z(re, {}), o(D), o(B), i(n, b);
        };
        F(Z, (n) => {
          a($) && n(ee);
        });
      }
      o(f), i(p, f);
    } });
  } });
  var V = t(R, 2);
  {
    var W = (e) => {
      var C = be(), p = M(C), y = t(r(p), 2), f = r(y, true);
      o(y), o(p);
      var d = t(p, 2), _ = r(d, true);
      o(d), le(() => {
        E(f, a(q)), E(_, a(w));
      }), i(e, C);
    };
    F(V, (e) => {
      a(w) && e(W);
    });
  }
  i(H, L), de();
}
export {
  Ue as component
};
