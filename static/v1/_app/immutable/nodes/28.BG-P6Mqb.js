import { t as c, a as i, e as A } from "../chunks/CWf9OOFK.js";
import { p as ie, a6 as ne, f as M, a as le, s as t, l as s, k as v, c as o, r as a, j as r, a8 as P, t as de } from "../chunks/nlANaGLT.js";
import { s as E } from "../chunks/BmMHVVX3.js";
import { i as F } from "../chunks/DOjUa9u5.js";
import { p as h } from "../chunks/5u5qd9TD.js";
import { B as K } from "../chunks/BEQMYyDu.js";
import { I as N } from "../chunks/6f3yten3.js";
import { C as z } from "../chunks/gvhP3O7i.js";
import { M as ve } from "../chunks/BDnQQV_p.js";
import { C as ce } from "../chunks/DA40Z_in.js";
import { a as G } from "../chunks/BRCxk8by.js";
import { F as pe } from "../chunks/CmL1R98Y.js";
import { g as fe } from "../chunks/B21bTIl7.js";
var ue = c("<br>-> enable idp registration:<br> <code>chrome://flags/#fedcm-idp-registration</code>", 1), ge = c("<p><!> <!> <!></p>"), me = c('<p><!></p> <!> <div class="row svelte-wuqbvo"><b>Logged In:</b> <div class="check svelte-wuqbvo"><!></div></div>', 1), _e = c('<div class="flex-col"><h1>FedCM Testing</h1> <div class="row svelte-wuqbvo"><div><b>FedCM supported:</b></div> <div class="check svelte-wuqbvo"><!></div></div> <!> <!></div>'), be = c('<div class="row svelte-wuqbvo"><b>Credential Type:</b> <span> </span></div> <div class="token svelte-wuqbvo"> </div>', 1), he = c("<!> <!>", 1);
function Te(H, J) {
  ie(J, true);
  let $ = v(false), k = v(false), w = v(""), q = v(""), p = v("fedcm"), l = v("any");
  ne(async () => {
    s(l, `${window.location.origin}/auth/v1/fed_cm/config`), s(p, `${window.location.origin}/auth/v1/fed_cm/client_config`), (await fetch("/auth/v1/fed_cm/status")).status === 200 ? (console.log("FedCM status is: logged-in"), s(k, true)) : console.log("FedCM status is: logged-out"), window.IdentityProvider && IdentityProvider.register != null ? (console.log("FedCM is supported"), s($, true)) : console.error("FedCM is not supported");
  });
  async function O() {
    console.log("using credentials get values: configUrl: " + r(l) + " / clientId: " + r(p));
    try {
      let e = await navigator.credentials.get({ identity: { mode: "button", providers: [{ configURL: r(l), clientId: r(p), nonce: fe(48) }] } });
      console.log(e), s(q, h(e.type)), s(w, h(e.token)), s(k, true);
    } catch (e) {
      console.error("FedCM credentials error: " + e);
    }
  }
  function Q() {
    IdentityProvider.register(r(l));
  }
  var L = he(), R = M(L);
  ve(R, { children: (e, C) => {
    ce(e, { children: (f, I) => {
      var u = _e(), d = t(o(u), 2), _ = t(o(d), 2), X = o(_);
      z(X, {}), a(_), a(d);
      var U = t(d, 2);
      {
        var Y = (n) => {
          var b = ue();
          P(4), i(n, b);
        };
        F(U, (n) => {
          r($) || n(Y);
        });
      }
      var Z = t(U, 2);
      {
        var ee = (n) => {
          var b = me(), y = M(b), te = o(y);
          K(te, { onclick: Q, level: 2, children: (x, oe) => {
            P();
            var g = A("Register IdP");
            i(x, g);
          }, $$slots: { default: true } }), a(y);
          var T = t(y, 2);
          pe(T, { action: "", onSubmit: O, children: (x, oe) => {
            var g = ge(), S = o(g);
            N(S, { autocomplete: "off", label: "Client ID", placeholder: "Client ID", required: true, pattern: G, get value() {
              return r(p);
            }, set value(m) {
              s(p, h(m));
            } });
            var j = t(S, 2);
            N(j, { autocomplete: "off", label: "Config URL", placeholder: "Config URL", pattern: G, get value() {
              return r(l);
            }, set value(m) {
              s(l, h(m));
            } });
            var ae = t(j, 2);
            K(ae, { type: "submit", children: (m, $e) => {
              P();
              var se = A("Login");
              i(m, se);
            }, $$slots: { default: true } }), a(g), i(x, g);
          }, $$slots: { default: true } });
          var B = t(T, 2), D = t(o(B), 2), re = o(D);
          z(re, {}), a(D), a(B), i(n, b);
        };
        F(Z, (n) => {
          r($) && n(ee);
        });
      }
      a(u), i(f, u);
    } });
  } });
  var V = t(R, 2);
  {
    var W = (e) => {
      var C = be(), f = M(C), I = t(o(f), 2), u = o(I, true);
      a(I), a(f);
      var d = t(f, 2), _ = o(d, true);
      a(d), de(() => {
        E(u, r(q)), E(_, r(w));
      }), i(e, C);
    };
    F(V, (e) => {
      r(w) && e(W);
    });
  }
  i(H, L), le();
}
export {
  Te as component
};
