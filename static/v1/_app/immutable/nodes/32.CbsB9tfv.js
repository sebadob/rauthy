import { t as k, a as i, d as L, e as z } from "../chunks/BH6NCLk-.js";
import { p as A, a5 as C, f as w, a as D, a6 as V, s as S, l as o, k as p, j as s, c as W, a7 as G, r as F, t as J } from "../chunks/CvlvO1XB.js";
import { h as K, s as N } from "../chunks/CTI4QPiR.js";
import { i as x } from "../chunks/BUO_AUgz.js";
import { p as c } from "../chunks/Wh68IIk2.js";
import { u as O, v as Q } from "../chunks/CGXT-546.js";
import { W as X } from "../chunks/CY7Th9O-.js";
import { L as Y } from "../chunks/KFFqpuZ0.js";
import { B as Z } from "../chunks/DMkkW5Nn.js";
import { u as ee } from "../chunks/DOl8_ubJ.js";
import { u as v } from "../chunks/D45liQ4S.js";
import { T as te } from "../chunks/DNdifVes.js";
import { b as re } from "../chunks/bbkAiDd0.js";
import { u as oe } from "../chunks/BMFqJ6Jy.js";
var ae = k('<div class="btn flex-col svelte-17xvymb"><!></div>'), se = k('<div class="err"> </div>'), ie = k("<!> <!> <!>", 1);
function xe(I, R) {
  A(R, true);
  let E = ee(), T = p(false), a = p(""), g = p(void 0), u = p(void 0);
  C(async () => {
    var _a, _b;
    let e = v("error").get();
    if (e) {
      let r = v("error_description").get();
      o(a, `${e}: ${r}`);
      return;
    }
    let n = v("state").get();
    if (!n) {
      o(a, "'state' is missing in URL");
      return;
    }
    let f = v("code").get();
    if (!f) {
      o(a, "'code' is missing in URL");
      return;
    }
    let h = { state: n, code: f, pkce_verifier: Q(), xsrf_token: O() }, d = "/auth/v1/providers/callback";
    oe().get() && (d = "/auth/v1/dev/providers_callback");
    let t = await re(d, h);
    if (t.status === 202) window.location.replace(t.headers.get("location") || "/auth/v1/account");
    else if (t.status === 200) {
      o(a, "");
      let r = t.body;
      r && "user_id" in r && "code" in r ? (o(g, c(r.user_id)), o(u, c({ Login: r.code }))) : console.error("did not receive a proper WebauthnLoginResponse after HTTP200");
    } else t.status === 204 ? window.location.replace("/auth/v1/account") : t.status === 403 ? o(a, c(((_a = t.error) == null ? void 0 : _a.message) || "HTTP 403 Forbidden")) : t.status === 406 ? (o(a, c(E.authorize.clientForceMfa)), o(T, true)) : o(a, `HTTP ${t.status}: ${(_b = t.error) == null ? void 0 : _b.message}`);
  });
  function H(e) {
    o(a, c(e)), o(u, void 0);
  }
  function M(e) {
    console.log("onWebauthnSuccess", e), e && "loc" in e && window.location.replace(e.loc);
  }
  var P = ie();
  K((e) => {
    V.title = "Callback";
  });
  var $ = w(P);
  {
    var U = (e) => {
      X(e, { get userId() {
        return s(g);
      }, get purpose() {
        return s(u);
      }, onSuccess: M, onError: H });
    }, B = (e) => {
      var n = L(), f = w(n);
      {
        var h = (t) => {
          var r = ae(), _ = W(r);
          Z(_, { onclick: () => window.location.href = "/auth/v1/account", children: (b, m) => {
            G();
            var l = z("Account");
            i(b, l);
          }, $$slots: { default: true } }), F(r), i(t, r);
        }, d = (t) => {
          var r = L(), _ = w(r);
          {
            var b = (m) => {
              var l = se(), q = W(l, true);
              F(l), J(() => N(q, s(a))), i(m, l);
            };
            x(_, (m) => {
              s(a) && m(b);
            }, true);
          }
          i(t, r);
        };
        x(f, (t) => {
          s(T) ? t(h) : t(d, false);
        }, true);
      }
      i(e, n);
    };
    x($, (e) => {
      s(u) && s(g) ? e(U) : e(B, false);
    });
  }
  var y = S($, 2);
  te(y, { absolute: true });
  var j = S(y, 2);
  Y(j, { absolute: true }), i(I, P), D();
}
export {
  xe as component
};
