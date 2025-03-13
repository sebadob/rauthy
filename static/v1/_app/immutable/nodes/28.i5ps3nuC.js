import { t as v, a as i, e as A } from "../chunks/YHhP1LbZ.js";
import { p as ie, a6 as ne, f as M, a as de, s as t, l as s, k as l, c as o, j as r, r as a, a8 as P, t as le } from "../chunks/Ck6jKiur.js";
import { s as E } from "../chunks/tDAaDMC_.js";
import { i as F } from "../chunks/7JDmqCCW.js";
import { p as $ } from "../chunks/ho0YXExL.js";
import { B as K } from "../chunks/Bd2Rvcxs.js";
import { I as N } from "../chunks/D75ao6x5.js";
import { C as z } from "../chunks/CY4mVlEI.js";
import { M as ve } from "../chunks/rkCvXuuX.js";
import { C as ce } from "../chunks/DIUoHniG.js";
import { a as G } from "../chunks/BRCxk8by.js";
import { F as pe } from "../chunks/iKqYlEgo.js";
import { g as fe } from "../chunks/B21bTIl7.js";
var ue = v("<br>-> enable idp registration:<br> <code>chrome://flags/#fedcm-idp-registration</code>", 1), ge = v("<p><!> <!> <!></p>"), me = v('<p><!></p> <!> <div class="row svelte-wuqbvo"><b>Logged In:</b> <div class="check svelte-wuqbvo"><!></div></div>', 1), _e = v('<div class="flex-col"><h1>FedCM Testing</h1> <div class="row svelte-wuqbvo"><div><b>FedCM supported:</b></div> <div class="check svelte-wuqbvo"><!></div></div> <!> <!></div>'), he = v('<div class="row svelte-wuqbvo"><b>Credential Type:</b> <span> </span></div> <div class="token svelte-wuqbvo"> </div>', 1), be = v("<!> <!>", 1);
function Ue(H, J) {
  ie(J, true);
  let m = l(false), w = l(false), C = l(""), q = l(""), _ = l("fedcm"), c = l("any");
  ne(async () => {
    s(c, `${window.location.origin}/auth/v1/fed_cm/config`), s(_, `${window.location.origin}/auth/v1/fed_cm/client_config`), (await fetch("/auth/v1/fed_cm/status")).status === 200 ? (console.log("FedCM status is: logged-in"), s(w, true)) : console.log("FedCM status is: logged-out"), window.IdentityProvider && IdentityProvider.register != null ? (console.log("FedCM is supported"), s(m, true)) : console.error("FedCM is not supported");
  });
  async function O() {
    try {
      let e = await navigator.credentials.get({ identity: { mode: "button", providers: [{ configURL: r(c), clientId: r(_), nonce: fe(48) }] } });
      console.log(e), s(q, $(e.type)), s(C, $(e.token)), s(w, true);
    } catch (e) {
      console.error("FedCM credentials error: " + e);
    }
  }
  function Q() {
    IdentityProvider.register(r(c));
  }
  var L = be(), R = M(L);
  ve(R, { children: (e, y) => {
    ce(e, { children: (p, I) => {
      var f = _e(), d = t(o(f), 2), h = t(o(d), 2), X = o(h);
      z(X, { get check() {
        return r(m);
      } }), a(h), a(d);
      var T = t(d, 2);
      {
        var Y = (n) => {
          var b = ue();
          P(4), i(n, b);
        };
        F(T, (n) => {
          r(m) || n(Y);
        });
      }
      var Z = t(T, 2);
      {
        var ee = (n) => {
          var b = me(), k = M(b), te = o(k);
          K(te, { onclick: Q, level: 2, children: (x, oe) => {
            P();
            var u = A("Register IdP");
            i(x, u);
          }, $$slots: { default: true } }), a(k);
          var U = t(k, 2);
          pe(U, { action: "", onSubmit: O, children: (x, oe) => {
            var u = ge(), S = o(u);
            N(S, { autocomplete: "off", label: "Client ID", placeholder: "Client ID", required: true, pattern: G, get value() {
              return r(_);
            }, set value(g) {
              s(_, $(g));
            } });
            var j = t(S, 2);
            N(j, { autocomplete: "off", label: "Config URL", placeholder: "Config URL", pattern: G, get value() {
              return r(c);
            }, set value(g) {
              s(c, $(g));
            } });
            var ae = t(j, 2);
            K(ae, { type: "submit", children: (g, $e) => {
              P();
              var se = A("Login");
              i(g, se);
            }, $$slots: { default: true } }), a(u), i(x, u);
          }, $$slots: { default: true } });
          var B = t(U, 2), D = t(o(B), 2), re = o(D);
          z(re, { get check() {
            return r(w);
          } }), a(D), a(B), i(n, b);
        };
        F(Z, (n) => {
          r(m) && n(ee);
        });
      }
      a(f), i(p, f);
    } });
  } });
  var V = t(R, 2);
  {
    var W = (e) => {
      var y = he(), p = M(y), I = t(o(p), 2), f = o(I, true);
      a(I), a(p);
      var d = t(p, 2), h = o(d, true);
      a(d), le(() => {
        E(f, r(q)), E(h, r(C));
      }), i(e, y);
    };
    F(V, (e) => {
      r(C) && e(W);
    });
  }
  i(H, L), de();
}
export {
  Ue as component
};
