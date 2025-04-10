import { t as v, a as i, e as j } from "../chunks/BxmJRzoY.js";
import { p as se, j as l, a0 as ie, f as M, a as ne, s as t, l as s, c as o, k as r, r as a, a3 as P, t as de } from "../chunks/w0HvPX0p.js";
import { s as A } from "../chunks/BzP2S3Z_.js";
import { i as x } from "../chunks/iO9_dPNE.js";
import { B as E } from "../chunks/C8YTstTD.js";
import { I as K } from "../chunks/DVXwAhn3.js";
import { C as N } from "../chunks/DwsgkunL.js";
import { M as le } from "../chunks/DKM0QPz5.js";
import { C as ve } from "../chunks/QNragXLc.js";
import { a as z } from "../chunks/gfDO7tLr.js";
import { F as ce } from "../chunks/BEbxeS8S.js";
import { g as pe } from "../chunks/B21bTIl7.js";
var ue = v("<br>-> enable idp registration:<br> <code>chrome://flags/#fedcm-idp-registration</code>", 1), fe = v("<p><!> <!> <!></p>"), ge = v('<p><!></p> <!> <div class="row svelte-wuqbvo"><b>Logged In:</b> <div class="check svelte-wuqbvo"><!></div></div>', 1), me = v('<div class="flex-col"><h1>FedCM Testing</h1> <div class="row svelte-wuqbvo"><div><b>FedCM supported:</b></div> <div class="check svelte-wuqbvo"><!></div></div> <!> <!></div>'), _e = v('<div class="row svelte-wuqbvo"><b>Credential Type:</b> <span> </span></div> <div class="token svelte-wuqbvo"> </div>', 1), he = v("<!> <!>", 1);
function Re(G, H) {
  se(H, true);
  let m = l(false), $ = l(false), w = l(""), F = l(""), _ = l("fedcm"), c = l("any");
  ie(async () => {
    s(c, `${window.location.origin}/auth/v1/fed_cm/config`), s(_, `${window.location.origin}/auth/v1/fed_cm/client_config`), (await fetch("/auth/v1/fed_cm/status")).status === 200 ? (console.log("FedCM status is: logged-in"), s($, true)) : console.log("FedCM status is: logged-out"), window.IdentityProvider && IdentityProvider.register != null ? (console.log("FedCM is supported"), s(m, true)) : console.error("FedCM is not supported");
  });
  async function J() {
    try {
      let e = await navigator.credentials.get({ identity: { mode: "button", providers: [{ configURL: r(c), clientId: r(_), nonce: pe(48) }] } });
      console.log(e), s(F, e.type, true), s(w, e.token, true), s($, true);
    } catch (e) {
      console.error("FedCM credentials error: " + e);
    }
  }
  function O() {
    IdentityProvider.register(r(c));
  }
  var q = he(), L = M(q);
  le(L, { children: (e, C) => {
    ve(e, { children: (p, I) => {
      var u = me(), d = t(o(u), 2), h = t(o(d), 2), W = o(h);
      N(W, { get check() {
        return r(m);
      } }), a(h), a(d);
      var R = t(d, 2);
      {
        var X = (n) => {
          var b = ue();
          P(4), i(n, b);
        };
        x(R, (n) => {
          r(m) || n(X);
        });
      }
      var Y = t(R, 2);
      {
        var Z = (n) => {
          var b = ge(), y = M(b), ee = o(y);
          E(ee, { onclick: O, level: 2, children: (k, re) => {
            P();
            var f = j("Register IdP");
            i(k, f);
          }, $$slots: { default: true } }), a(y);
          var T = t(y, 2);
          ce(T, { action: "", onSubmit: J, children: (k, re) => {
            var f = fe(), D = o(f);
            K(D, { autocomplete: "off", label: "Client ID", placeholder: "Client ID", required: true, pattern: z, get value() {
              return r(_);
            }, set value(g) {
              s(_, g, true);
            } });
            var S = t(D, 2);
            K(S, { autocomplete: "off", label: "Config URL", placeholder: "Config URL", pattern: z, get value() {
              return r(c);
            }, set value(g) {
              s(c, g, true);
            } });
            var oe = t(S, 2);
            E(oe, { type: "submit", children: (g, be) => {
              P();
              var ae = j("Login");
              i(g, ae);
            }, $$slots: { default: true } }), a(f), i(k, f);
          }, $$slots: { default: true } });
          var U = t(T, 2), B = t(o(U), 2), te = o(B);
          N(te, { get check() {
            return r($);
          } }), a(B), a(U), i(n, b);
        };
        x(Y, (n) => {
          r(m) && n(Z);
        });
      }
      a(u), i(p, u);
    } });
  } });
  var Q = t(L, 2);
  {
    var V = (e) => {
      var C = _e(), p = M(C), I = t(o(p), 2), u = o(I, true);
      a(I), a(p);
      var d = t(p, 2), h = o(d, true);
      a(d), de(() => {
        A(u, r(F)), A(h, r(w));
      }), i(e, C);
    };
    x(Q, (e) => {
      r(w) && e(V);
    });
  }
  i(G, q), ne();
}
export {
  Re as component
};
