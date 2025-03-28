import { d as k, a as d, t as w } from "../chunks/BxmJRzoY.js";
import { p as g, j as C, a0 as S, f as T, a as E, a2 as R, l as n, k as p, c as I, r as j, t as y } from "../chunks/w0HvPX0p.js";
import { h as U, s as $ } from "../chunks/BzP2S3Z_.js";
import { i as b } from "../chunks/iO9_dPNE.js";
import { b as x, c as A, s as D, d as P, e as z, f as F, R as L, h as V } from "../chunks/B21bTIl7.js";
import { u as f } from "../chunks/Di69SM1b.js";
var X = w('<div class="err"> </div>');
function J(l, m) {
  g(m, true);
  let r = C(""), _ = f("code"), h = f("state");
  S(async () => {
    let e = window.location.origin + L, t = h.get();
    t && (t === "account" ? e = window.location.origin + V : t.startsWith("device") && (e = `${window.location.origin}/auth/v1/${t}`));
    let s = _.get();
    if (!s) {
      console.error("no `code` given");
      return;
    }
    const o = new URLSearchParams();
    o.append("grant_type", "authorization_code"), o.append("code", s), o.append("redirect_uri", e), o.append("client_id", x), o.append("code_verifier", A());
    let a = await (await fetch("/auth/v1/oidc/token", { method: "POST", headers: { "Content-type": "application/x-www-form-urlencoded", Accept: "application/json" }, body: o })).json();
    if (a.error) n(r, a.error.toString(), true);
    else if (a.access_token) {
      D(a.access_token), a.id_token && P(a.id_token);
      let i = await (await fetch("/auth/v1/oidc/sessioninfo/xsrf", { method: "GET", headers: { "Content-Type": "application/json", Accept: "application/json", Authorization: `Bearer ${a.access_token}` } })).json();
      if (i.csrf_token) z(i.csrf_token);
      else {
        n(r, i.toString(), true);
        return;
      }
      F(), window.location.replace(e);
    } else n(r, "");
  });
  var c = k();
  U((e) => {
    R.title = "Callback";
  });
  var u = T(c);
  {
    var v = (e) => {
      var t = X(), s = I(t, true);
      j(t), y(() => $(s, p(r))), d(e, t);
    };
    b(u, (e) => {
      p(r) && e(v);
    });
  }
  d(l, c), E();
}
export {
  J as component
};
