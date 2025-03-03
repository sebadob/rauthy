import { d as A, a as _, t as S } from "./BH6NCLk-.js";
import { p as L, a5 as M, l as P, k as H, aa as G, j as l, f as B, a as Q, c as n, r as c, s as z, t as T } from "./CvlvO1XB.js";
import { s as D } from "./CTI4QPiR.js";
import { i as k } from "./BUO_AUgz.js";
import { p as V } from "./Wh68IIk2.js";
import { L as X } from "./DMkkW5Nn.js";
import { M as Y, N as Z, p as $, O as x } from "./B8wC3kJv.js";
import { b as O } from "./DYtiVhoA.js";
import { u as ee } from "./BQ1-pLIs.js";
function me(o) {
  let t = Z(o);
  const a = { "+": "-", "/": "_", "=": "" };
  return t.replace(/[+/=]/g, (e) => a[e]);
}
function U(o) {
  if (typeof o == "string") {
    const t = { "-": "+", _: "/", ".": "=" }, a = o.replace(/[-_.]/g, (e) => t[e]);
    return Y(a);
  } else return o;
}
async function re(o, t, a, e) {
  var _a;
  let g = { purpose: t }, d = await O(`/auth/v1/users/${o}/webauthn/auth/start`, g);
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
  if (i.publicKey.challenge = U(i.publicKey.challenge), i.publicKey.allowCredentials) for (let r of i.publicKey.allowCredentials) r.id = U(r.id);
  const m = (h.exp - 1) * 1e3, b = (/* @__PURE__ */ new Date()).getTime() + m;
  let s;
  try {
    const r = await $(navigator.credentials.get(i), m);
    if (r) s = r;
    else return { error: a };
  } catch {
    return { error: (/* @__PURE__ */ new Date()).getTime() >= b ? e : a };
  }
  let y = { code: h.code, data: { id: s.id, rawId: x(s.rawId), response: { authenticatorData: x(s.response.authenticatorData), clientDataJSON: x(s.response.clientDataJSON), signature: x(s.response.signature) }, extensions: s.getClientExtensionResults(), type: s.type } }, v = await O(`/auth/v1/users/${o}/webauthn/auth/finish`, y);
  return v.status === 202 ? { data: v.body } : (console.error(v), { error: ((_a = v.error) == null ? void 0 : _a.message) || "Authentication Error" });
}
var te = S('<div class="err svelte-jfkcym"> </div>'), ae = S('<div class="good svelte-jfkcym"> </div>'), se = S('<div class="wrapperOuter svelte-jfkcym"><div class="wrapperInner svelte-jfkcym"><div class="content svelte-jfkcym"><div class="contentRow svelte-jfkcym"><div class="contentHeader svelte-jfkcym"> </div></div> <div class="contentRow svelte-jfkcym"><div><!></div></div> <div class="contentRow svelte-jfkcym"><!></div></div></div></div>');
function pe(o, t) {
  L(t, true);
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
      var w = z(y, 2), E = n(w), C = n(E);
      {
        var N = (u) => {
          X(u, {});
        };
        k(C, (u) => {
          l(e) || u(N);
        });
      }
      c(E), c(w);
      var K = z(w, 2), W = n(K);
      {
        var q = (u) => {
          var R = A(), F = B(R);
          {
            var I = (p) => {
              var f = te(), j = n(f, true);
              c(f), T(() => D(j, l(e).error)), _(p, f);
            }, J = (p) => {
              var f = ae(), j = n(f, true);
              c(f), T(() => D(j, a.authorize.mfaAck)), _(p, f);
            };
            k(F, (p) => {
              l(e).error ? p(I) : p(J, false);
            });
          }
          _(u, R);
        };
        k(W, (u) => {
          l(e) && u(q);
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
  U as b
};
