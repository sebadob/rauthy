import { d as C, a as p, t as S } from "../chunks/DLBGyKVC.js";
import { p as g, a6 as T, f as w, a as E, a7 as R, l as n, k as y, j as d, c as I, r as j, t as x } from "../chunks/CmQi0fbH.js";
import { h as U, s as b } from "../chunks/BjaYyaa_.js";
import { i as A } from "../chunks/C6bK2EJJ.js";
import { p as f } from "../chunks/B_ggA-0N.js";
import { R as $, b as D, c as P, s as z, d as F, e as L, f as V, h as X } from "../chunks/B21bTIl7.js";
import { u as l } from "../chunks/DJVZKROe.js";
var B = S('<div class="err"> </div>');
function Q(m, _) {
  g(_, true);
  let r = y(""), h = l("code"), u = l("state");
  T(async () => {
    let e = $, t = u.get();
    t && (t === "account" ? e = X : t.startsWith("device") && (e = `/auth/v1/${t}`));
    let s = h.get();
    if (!s) {
      console.error("no `code` given");
      return;
    }
    const o = new URLSearchParams();
    o.append("grant_type", "authorization_code"), o.append("code", s), o.append("redirect_uri", e), o.append("client_id", D), o.append("code_verifier", P());
    let a = await (await fetch("/auth/v1/oidc/token", { method: "POST", headers: { "Content-type": "application/x-www-form-urlencoded", Accept: "application/json" }, body: o })).json();
    if (a.error) n(r, f(a.error.toString()));
    else if (a.access_token) {
      z(a.access_token), a.id_token && F(a.id_token);
      let i = await (await fetch("/auth/v1/oidc/sessioninfo/xsrf", { method: "GET", headers: { "Content-Type": "application/json", Accept: "application/json", Authorization: `Bearer ${a.access_token}` } })).json();
      if (i.csrf_token) L(i.csrf_token);
      else {
        n(r, f(i.toString()));
        return;
      }
      V(), window.location.replace(e);
    } else n(r, "");
  });
  var c = C();
  U((e) => {
    R.title = "Callback";
  });
  var v = w(c);
  {
    var k = (e) => {
      var t = B(), s = I(t, true);
      j(t), x(() => b(s, d(r))), p(e, t);
    };
    A(v, (e) => {
      d(r) && e(k);
    });
  }
  p(m, c), E();
}
export {
  Q as component
};
