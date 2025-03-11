import { t as w, a as p, e as U } from "../chunks/CWf9OOFK.js";
import { p as B, a6 as j, f as q, a as z, a7 as A, s as $, l as r, k as m, j as i, c as y, a8 as C, r as L, t as D } from "../chunks/nlANaGLT.js";
import { h as V, s as G } from "../chunks/BmMHVVX3.js";
import { i as b } from "../chunks/DOjUa9u5.js";
import { p as l } from "../chunks/5u5qd9TD.js";
import { t as J, u as K } from "../chunks/B21bTIl7.js";
import { W as N } from "../chunks/C5gerY3H.js";
import { L as O } from "../chunks/DbogliCN.js";
import { B as Q } from "../chunks/BEQMYyDu.js";
import { u as X } from "../chunks/DwvS5LQk.js";
import { u as v } from "../chunks/BTUJVHfP.js";
import { T as Y } from "../chunks/Afb0E4F7.js";
import { b as Z } from "../chunks/BO1A6s0c.js";
import { u as ee } from "../chunks/Js0GGUMw.js";
var te = w('<div class="btn flex-col svelte-17xvymb"><!></div>'), re = w('<div class="err"> </div>'), oe = w("<!> <!> <!>", 1);
function _e(S, W) {
  B(W, true);
  let F = X(), x = m(false), a = m(""), g = m(void 0), c = m(void 0);
  j(async () => {
    var _a, _b;
    let e = v("error").get();
    if (e) {
      let o = v("error_description").get();
      r(a, `${e}: ${o}`);
      return;
    }
    let u = v("state").get();
    if (!u) {
      r(a, "'state' is missing in URL");
      return;
    }
    let f = v("code").get();
    if (!f) {
      r(a, "'code' is missing in URL");
      return;
    }
    let h = { state: u, code: f, pkce_verifier: K(), xsrf_token: J() }, s = "/auth/v1/providers/callback";
    ee().get() && (s = "/auth/v1/dev/providers_callback");
    let t = await Z(s, h);
    if (t.status === 202) window.location.replace(t.headers.get("location") || "/auth/v1/account");
    else if (t.status === 200) {
      r(a, "");
      let o = t.body;
      o && "user_id" in o && "code" in o ? (r(g, l(o.user_id)), r(c, l({ Login: o.code }))) : console.error("did not receive a proper WebauthnLoginResponse after HTTP200");
    } else t.status === 204 ? window.location.replace("/auth/v1/account") : t.status === 403 ? r(a, l(((_a = t.error) == null ? void 0 : _a.message) || "HTTP 403 Forbidden")) : t.status === 406 ? (r(a, l(F.authorize.clientForceMfa)), r(x, true)) : r(a, `HTTP ${t.status}: ${(_b = t.error) == null ? void 0 : _b.message}`);
  });
  function I(e) {
    r(a, l(e)), r(c, void 0);
  }
  function R(e) {
    e && "loc" in e && window.location.replace(e.loc);
  }
  var k = oe();
  V((e) => {
    A.title = "Callback";
  });
  var T = q(k);
  {
    var E = (e) => {
      N(e, { get userId() {
        return i(g);
      }, get purpose() {
        return i(c);
      }, onSuccess: R, onError: I });
    }, H = (e, u) => {
      {
        var f = (s) => {
          var t = te(), o = y(t);
          Q(o, { onclick: () => window.location.href = "/auth/v1/account", children: (n, d) => {
            C();
            var _ = U("Account");
            p(n, _);
          }, $$slots: { default: true } }), L(t), p(s, t);
        }, h = (s, t) => {
          {
            var o = (n) => {
              var d = re(), _ = y(d, true);
              L(d), D(() => G(_, i(a))), p(n, d);
            };
            b(s, (n) => {
              i(a) && n(o);
            }, t);
          }
        };
        b(e, (s) => {
          i(x) ? s(f) : s(h, false);
        }, u);
      }
    };
    b(T, (e) => {
      i(c) && i(g) ? e(E) : e(H, false);
    });
  }
  var P = $(T, 2);
  Y(P, { absolute: true });
  var M = $(P, 2);
  O(M, { absolute: true }), p(S, k), z();
}
export {
  _e as component
};
