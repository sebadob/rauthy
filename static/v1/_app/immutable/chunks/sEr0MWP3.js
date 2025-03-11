import { d as A, a as _, t as S } from "./BH6NCLk-.js";
import { p as J, a5 as M, l as P, k as H, aa as G, j as l, f as B, a as Q, c as n, r as c, s as z, t as T } from "./CvlvO1XB.js";
import { s as D } from "./CTI4QPiR.js";
import { i as k } from "./BUO_AUgz.js";
import { p as V } from "./Wh68IIk2.js";
import { L as X } from "./CqH8LXO-.js";
import { L as Y, M as Z, p as $, N as x } from "./B21bTIl7.js";
import { b as U } from "./BO1A6s0c.js";
import { u as ee } from "./CUqQZdNU.js";
function me(o) {
  let t = Z(o);
  const a = { "+": "-", "/": "_", "=": "" };
  return t.replace(/[+/=]/g, (e) => a[e]);
}
function C(o) {
  if (typeof o == "string") {
    const t = { "-": "+", _: "/", ".": "=" }, a = o.replace(/[-_.]/g, (e) => t[e]);
    return Y(a);
  } else return o;
}
async function re(o, t, a, e) {
  var _a;
  let g = { purpose: t }, d = await U(`/auth/v1/users/${o}/webauthn/auth/start`, g);
  if (d.error) return console.error(d.error), { error: d.error.message || "Error starting the Authentication" };
  if (!d.body) {
    let r = "Did not receive a valid webauthn body";
    return console.error(r), { error: r };
  }
  let h = d.body, i = h.rcr;
  if (!i.publicKey) {
    let r = "no publicKey in challenge from the backend";
    return console.error(r), { error: r };
  }
  if (i.publicKey.challenge = C(i.publicKey.challenge), i.publicKey.allowCredentials) for (let r of i.publicKey.allowCredentials) r.id = C(r.id);
  const m = (h.exp - 1) * 1e3, b = (/* @__PURE__ */ new Date()).getTime() + m;
  let s;
  try {
    const r = await $(navigator.credentials.get(i), m);
    if (r) s = r;
    else return { error: a };
  } catch {
    return { error: (/* @__PURE__ */ new Date()).getTime() >= b ? e : a };
  }
  let y = { code: h.code, data: { id: s.id, rawId: x(s.rawId), response: { authenticatorData: x(s.response.authenticatorData), clientDataJSON: x(s.response.clientDataJSON), signature: x(s.response.signature) }, extensions: s.getClientExtensionResults(), type: s.type } }, v = await U(`/auth/v1/users/${o}/webauthn/auth/finish`, y);
  return v.status === 202 ? { data: v.body } : (console.error(v), { error: ((_a = v.error) == null ? void 0 : _a.message) || "Authentication Error" });
}
var te = S('<div class="err svelte-jfkcym"> </div>'), ae = S('<div class="good svelte-jfkcym"> </div>'), se = S('<div class="wrapperOuter svelte-jfkcym"><div class="wrapperInner svelte-jfkcym"><div class="content svelte-jfkcym"><div class="contentRow svelte-jfkcym"><div class="contentHeader svelte-jfkcym"> </div></div> <div class="contentRow svelte-jfkcym"><div><!></div></div> <div class="contentRow svelte-jfkcym"><!></div></div></div></div>');
function pe(o, t) {
  J(t, true);
  let a = ee(), e = H(void 0);
  M(async () => {
    P(e, V(await re(t.userId, t.purpose, a.authorize.invalidKeyUsed, a.authorize.requestExpired)));
  }), G(() => {
    l(e) && (l(e).error ? setTimeout(() => {
      var _a;
      t.onError(((_a = l(e)) == null ? void 0 : _a.error) || "Webauthn Error");
    }, 3e3) : t.onSuccess(l(e).data));
  });
  var g = A(), d = B(g);
  {
    var h = (i) => {
      var m = se(), b = n(m), s = n(b), y = n(s), v = n(y), r = n(v, true);
      c(v), c(y);
      var w = z(y, 2), E = n(w), L = n(E);
      {
        var N = (u) => {
          X(u, {});
        };
        k(L, (u) => {
          l(e) || u(N);
        });
      }
      c(E), c(w);
      var K = z(w, 2), O = n(K);
      {
        var W = (u) => {
          var R = A(), q = B(R);
          {
            var F = (p) => {
              var f = te(), j = n(f, true);
              c(f), T(() => D(j, l(e).error)), _(p, f);
            }, I = (p) => {
              var f = ae(), j = n(f, true);
              c(f), T(() => D(j, a.authorize.mfaAck)), _(p, f);
            };
            k(q, (p) => {
              l(e).error ? p(F) : p(I, false);
            });
          }
          _(u, R);
        };
        k(O, (u) => {
          l(e) && u(W);
        });
      }
      c(K), c(s), c(b), c(m), T(() => D(r, a.authorize.expectingPasskey)), _(i, m);
    };
    k(d, (i) => {
      t.purpose && i(h);
    });
  }
  _(o, g), Q();
}
export {
  pe as W,
  me as a,
  C as b
};
