import { t as b, a as d, e as M } from "../chunks/BxmJRzoY.js";
import { p as U, j as m, a0 as B, f as j, a as q, a2 as z, s as P, l as r, k as i, c as $, a3 as A, r as L, t as C } from "../chunks/w0HvPX0p.js";
import { h as D, s as V } from "../chunks/BzP2S3Z_.js";
import { i as _ } from "../chunks/iO9_dPNE.js";
import { t as G, u as J } from "../chunks/B21bTIl7.js";
import { W as K } from "../chunks/BjT2-OU-.js";
import { L as N } from "../chunks/D0XpefKJ.js";
import { B as O } from "../chunks/C8YTstTD.js";
import { u as Q } from "../chunks/0cG6LBdy.js";
import { u as p } from "../chunks/DYaEteNC.js";
import { T as X } from "../chunks/B1f0afjj.js";
import { b as Y } from "../chunks/UPFlzoow.js";
import { u as Z } from "../chunks/F_Qf1tHt.js";
var ee = b('<div class="btn flex-col svelte-17xvymb"><!></div>'), te = b('<div class="err"> </div>'), re = b("<!> <!> <!>", 1);
function ge(y, S) {
  U(S, true);
  let W = Q(), w = m(false), a = m(""), v = m(void 0), l = m(void 0);
  B(async () => {
    var _a, _b;
    let e = p("error").get();
    if (e) {
      let o = p("error_description").get();
      r(a, `${e}: ${o}`);
      return;
    }
    let u = p("state").get();
    if (!u) {
      r(a, "'state' is missing in URL");
      return;
    }
    let c = p("code").get();
    if (!c) {
      r(a, "'code' is missing in URL");
      return;
    }
    let g = { state: u, code: c, pkce_verifier: J(), xsrf_token: G() }, s = "/auth/v1/providers/callback";
    Z().get() && (s = "/auth/v1/dev/providers_callback");
    let t = await Y(s, g);
    if (t.status === 202) window.location.replace(t.headers.get("location") || "/auth/v1/account");
    else if (t.status === 200) {
      r(a, "");
      let o = t.body;
      o && "user_id" in o && "code" in o ? (r(v, o.user_id, true), r(l, { Login: o.code }, true)) : console.error("did not receive a proper WebauthnLoginResponse after HTTP200");
    } else t.status === 204 ? window.location.replace("/auth/v1/account") : t.status === 403 ? r(a, ((_a = t.error) == null ? void 0 : _a.message) || "HTTP 403 Forbidden", true) : t.status === 406 ? (r(a, W.authorize.clientForceMfa, true), r(w, true)) : r(a, `HTTP ${t.status}: ${(_b = t.error) == null ? void 0 : _b.message}`);
  });
  function F(e) {
    r(a, e, true), r(l, void 0);
  }
  function I(e) {
    e && "loc" in e && window.location.replace(e.loc);
  }
  var k = re();
  D((e) => {
    z.title = "Callback";
  });
  var x = j(k);
  {
    var R = (e) => {
      K(e, { get userId() {
        return i(v);
      }, get purpose() {
        return i(l);
      }, onSuccess: I, onError: F });
    }, E = (e, u) => {
      {
        var c = (s) => {
          var t = ee(), o = $(t);
          O(o, { onclick: () => window.location.href = "/auth/v1/account", children: (n, f) => {
            A();
            var h = M("Account");
            d(n, h);
          }, $$slots: { default: true } }), L(t), d(s, t);
        }, g = (s, t) => {
          {
            var o = (n) => {
              var f = te(), h = $(f, true);
              L(f), C(() => V(h, i(a))), d(n, f);
            };
            _(s, (n) => {
              i(a) && n(o);
            }, t);
          }
        };
        _(e, (s) => {
          i(w) ? s(c) : s(g, false);
        }, u);
      }
    };
    _(x, (e) => {
      i(l) && i(v) ? e(R) : e(E, false);
    });
  }
  var T = P(x, 2);
  X(T, { absolute: true });
  var H = P(T, 2);
  N(H, { absolute: true }), d(y, k), q();
}
export {
  ge as component
};
