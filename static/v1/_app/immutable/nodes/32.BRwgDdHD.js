import { t as k, a as i, d as L, e as z } from "../chunks/BH6NCLk-.js";
import { p as A, a5 as C, f as w, a as D, a6 as V, s as S, l as a, k as p, j as s, c as W, a7 as G, r as F, t as J } from "../chunks/CvlvO1XB.js";
import { h as K, s as N } from "../chunks/CTI4QPiR.js";
import { i as x } from "../chunks/BUO_AUgz.js";
import { p as c } from "../chunks/Wh68IIk2.js";
import { u as O, v as Q } from "../chunks/Brp0G0eV.js";
import { W as X } from "../chunks/BKOxXyr3.js";
import { L as Y } from "../chunks/KzU4-o98.js";
import { B as Z } from "../chunks/DMkkW5Nn.js";
import { u as ee } from "../chunks/CgEHB2u3.js";
import { u as v } from "../chunks/rQ2Hm1Il.js";
import { T as te } from "../chunks/DX6irJfo.js";
import { b as re } from "../chunks/CRjU5SuJ.js";
import { u as ae } from "../chunks/BMFqJ6Jy.js";
var oe = k('<div class="btn flex-col svelte-17xvymb"><!></div>'), se = k('<div class="err"> </div>'), ie = k("<!> <!> <!>", 1);
function xe(I, R) {
  A(R, true);
  let E = ee(), T = p(false), o = p(""), g = p(void 0), u = p(void 0);
  C(async () => {
    var _a, _b;
    let t = v("error").get();
    if (t) {
      let r = v("error_description").get();
      a(o, `${t}: ${r}`);
      return;
    }
    let n = v("state").get();
    if (!n) {
      a(o, "'state' is missing in URL");
      return;
    }
    let f = v("code").get();
    if (!f) {
      a(o, "'code' is missing in URL");
      return;
    }
    let h = { state: n, code: f, pkce_verifier: Q(), xsrf_token: O() }, d = "/auth/v1/providers/callback";
    ae().get() && (d = "/auth/v1/dev/providers_callback");
    let e = await re(d, h);
    if (e.status === 202) window.location.replace(e.headers.get("location") || "/auth/v1/account");
    else if (e.status === 200) {
      a(o, "");
      let r = e.body;
      r && "user_id" in r && "code" in r ? (a(g, c(r.user_id)), a(u, c({ Login: r.code }))) : console.error("did not receive a proper WebauthnLoginResponse after HTTP200");
    } else e.status === 204 ? window.location.replace("/auth/v1/account") : e.status === 403 ? a(o, c(((_a = e.error) == null ? void 0 : _a.message) || "HTTP 403 Forbidden")) : e.status === 406 ? (a(o, c(E.authorize.clientForceMfa)), a(T, true)) : a(o, `HTTP ${e.status}: ${(_b = e.error) == null ? void 0 : _b.message}`);
  });
  function H(t) {
    a(o, c(t)), a(u, void 0);
  }
  function M(t) {
    t && "loc" in t && window.location.replace(t.loc);
  }
  var P = ie();
  K((t) => {
    V.title = "Callback";
  });
  var $ = w(P);
  {
    var U = (t) => {
      X(t, { get userId() {
        return s(g);
      }, get purpose() {
        return s(u);
      }, onSuccess: M, onError: H });
    }, B = (t) => {
      var n = L(), f = w(n);
      {
        var h = (e) => {
          var r = oe(), _ = W(r);
          Z(_, { onclick: () => window.location.href = "/auth/v1/account", children: (b, m) => {
            G();
            var l = z("Account");
            i(b, l);
          }, $$slots: { default: true } }), F(r), i(e, r);
        }, d = (e) => {
          var r = L(), _ = w(r);
          {
            var b = (m) => {
              var l = se(), q = W(l, true);
              F(l), J(() => N(q, s(o))), i(m, l);
            };
            x(_, (m) => {
              s(o) && m(b);
            }, true);
          }
          i(e, r);
        };
        x(f, (e) => {
          s(T) ? e(h) : e(d, false);
        }, true);
      }
      i(t, n);
    };
    x($, (t) => {
      s(u) && s(g) ? t(U) : t(B, false);
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
