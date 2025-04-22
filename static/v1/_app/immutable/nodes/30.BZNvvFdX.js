import { d as k, a as p, t as g } from "../chunks/BxmJRzoY.js";
import { p as C, j as S, a0 as T, f as E, a as R, a2 as I, l as n, k as l, c as $, r as b, t as j } from "../chunks/w0HvPX0p.js";
import { h as y, s as x } from "../chunks/BzP2S3Z_.js";
import { i as A } from "../chunks/iO9_dPNE.js";
import { b as U, c as D, s as P, d as z, e as F, f as L, R as V, h as X } from "../chunks/B21bTIl7.js";
import { u as f } from "../chunks/DvTygCXn.js";
var B = g('<div class="err"> </div>');
function J(m, _) {
  C(_, true);
  let o = S(""), h = f("code"), u = f("state");
  T(async () => {
    let t = h.get();
    if (!t) {
      console.error("no `code` given");
      return;
    }
    const e = new URLSearchParams();
    e.append("grant_type", "authorization_code"), e.append("code", t), e.append("redirect_uri", `${window.location.origin}/auth/v1/oidc/callback`), e.append("client_id", U), e.append("code_verifier", D());
    let a = await (await fetch("/auth/v1/oidc/token", { method: "POST", headers: { "Content-type": "application/x-www-form-urlencoded", Accept: "application/json" }, body: e })).json();
    if (a.error) n(o, a.error.toString(), true);
    else if (a.access_token) {
      P(a.access_token), a.id_token && z(a.id_token);
      let i = await (await fetch("/auth/v1/oidc/sessioninfo/xsrf", { method: "GET", headers: { "Content-Type": "application/json", Accept: "application/json", Authorization: `Bearer ${a.access_token}` } })).json();
      if (i.csrf_token) F(i.csrf_token);
      else {
        n(o, i.toString(), true);
        return;
      }
      L();
      let s = window.location.origin + V, r = u.get();
      r && (r === "account" ? s = window.location.origin + X : r.startsWith("device") && (s = `${window.location.origin}/auth/v1/${r}`)), window.location.replace(s);
    } else n(o, "");
  });
  var c = k();
  y((t) => {
    I.title = "Callback";
  });
  var v = E(c);
  {
    var w = (t) => {
      var e = B(), d = $(e, true);
      b(e), j(() => x(d, l(o))), p(t, e);
    };
    A(v, (t) => {
      l(o) && t(w);
    });
  }
  p(m, c), R();
}
export {
  J as component
};
