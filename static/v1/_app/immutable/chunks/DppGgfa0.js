var E = {};
Object.defineProperty(E, "__esModule", { value: true });
function I(e) {
  return btoa(e).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
}
function R(e, o) {
  e || (e = 43);
  for (var r = typeof window < "u" && window.msCrypto || crypto, t = I(Array.prototype.map.call(r.getRandomValues(new Uint8Array(e)), function(s) {
    return String.fromCharCode(s);
  }).join("")).substring(0, e), n = new Uint8Array(t.length), a = 0; a < t.length; a++) n[a] = t.charCodeAt(a);
  var i = r.subtle.digest("SHA-256", n);
  typeof window < "u" && window.CryptoOperation ? (i.onerror = o, i.oncomplete = function(s) {
    C(o, t, s.target.result);
  }) : i.then(function(s) {
    C(o, t, s);
  }).catch(o);
}
var A = E.default = R;
function C(e, o, r) {
  e(null, { verifier: o, challenge: I(String.fromCharCode.apply(null, new Uint8Array(r))) });
}
const p = "pkce_verifier", S = "pkce_verifier_upstream", m = "csrf_token", w = "access_token", v = "id_token", h = "provider_token", U = "/auth/v1/oidc/authorize", y = "rauthy", L = "/auth/v1/oidc/logout", N = "/auth/v1/oidc/callback", k = "/auth/v1/admin", $ = "/auth/v1/account", P = "/auth/v1/", M = ["info", "notice", "warning", "critical"], b = ["-", "InvalidLogins", "IpBlacklisted", "IpBlacklistRemoved", "JwksRotated", "NewUserRegistered", "NewRauthyAdmin", "NewRauthyVersion", "PossibleBruteForce", "RauthyStarted", "RauthyHealthy", "RauthyUnhealthy", "SecretsMigrated", "UserEmailChange", "UserPasswordReset", "Test"], F = ["de", "en", "zh", "ko"], V = ["de", "en"], G = "tpl_auth_providers", K = "tpl_client_name", B = "tpl_client_url", H = "tpl_csrf_token", x = "tpl_email_old", j = "tpl_email_new", z = "tpl_error_text", W = "tpl_device_user_code_length", J = "tpl_is_reg_open", Y = "tpl_login_action", q = "tpl_password_reset", Q = "tpl_status_code", X = "tpl_restricted_email_domain";
var _ = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/", g = typeof Uint8Array > "u" ? [] : new Uint8Array(256);
for (var f = 0; f < _.length; f++) g[_.charCodeAt(f)] = f;
var O = function(e) {
  var o = new Uint8Array(e), r, t = o.length, n = "";
  for (r = 0; r < t; r += 3) n += _[o[r] >> 2], n += _[(o[r] & 3) << 4 | o[r + 1] >> 4], n += _[(o[r + 1] & 15) << 2 | o[r + 2] >> 6], n += _[o[r + 2] & 63];
  return t % 3 === 2 ? n = n.substring(0, n.length - 1) + "=" : t % 3 === 1 && (n = n.substring(0, n.length - 2) + "=="), n;
}, Z = function(e) {
  var o = e.length * 0.75, r = e.length, t, n = 0, a, i, s, l;
  e[e.length - 1] === "=" && (o--, e[e.length - 2] === "=" && o--);
  var d = new ArrayBuffer(o), u = new Uint8Array(d);
  for (t = 0; t < r; t += 4) a = g[e.charCodeAt(t)], i = g[e.charCodeAt(t + 1)], s = g[e.charCodeAt(t + 2)], l = g[e.charCodeAt(t + 3)], u[n++] = a << 2 | i >> 4, u[n++] = (i & 15) << 4 | s >> 2, u[n++] = (s & 3) << 6 | l & 63;
  return d;
};
function ee(e) {
  return `${window.location.origin}/auth/${e}/profile#me`;
}
function te() {
  return typeof window < "u";
}
function oe(e) {
  return e === "openid" || e === "profile" || e === "email" || e === "groups" || e === "address" || e === "phone";
}
const re = (e) => {
  A(64, (o, { challenge: r, verifier: t }) => {
    if (!o) {
      localStorage.setItem(p, t);
      const n = D(24), a = e || "admin", i = `${window.location.origin}${N}`.replaceAll(":", "%3A").replaceAll("/", "%2F");
      window.location.href = `${U}?client_id=${y}&redirect_uri=${i}&response_type=code&code_challenge=${r}&code_challenge_method=S256&scope=openid+profile+email&nonce=${n}&state=${a}`;
    }
  });
}, ne = () => {
  const e = `${L}?post_logout_redirect_uri=${window.location.origin}${P}`;
  window.location.href = e;
}, ae = (e) => {
  console.log("saving csrf token in local storage", m, e), localStorage.setItem(m, e);
}, se = (e) => {
  localStorage.setItem(v, e);
}, ie = (e) => {
  localStorage.setItem(w, e);
}, ce = (e) => {
  localStorage.setItem(h, e);
}, le = () => localStorage.getItem(h) || "", de = () => localStorage.getItem(p) || "", ue = () => localStorage.getItem(S) || "", _e = () => {
  localStorage.removeItem(p);
}, ge = () => {
  localStorage.removeItem(m), localStorage.removeItem(v), localStorage.removeItem(w), localStorage.removeItem(p), localStorage.removeItem(S), localStorage.removeItem(h);
};
function fe(e) {
  let o = O(e);
  const r = { "+": "-", "/": "_", "=": "" };
  return o.replace(/[+/=]/g, (t) => r[t]);
}
function pe(e) {
  switch (e) {
    case "info":
      return "rgba(52,255,109,0.7)";
    case "notice":
      return "rgba(51,69,255,0.7)";
    case "warning":
      return "rgba(255,211,51,0.7)";
    case "critical":
      return "rgba(255,46,46,0.7)";
  }
}
const me = (e, o) => {
  let r;
  if (o ? r = Date.parse(`${e}T${o}`) : r = Date.parse(e), !isNaN(r)) return r / 1e3;
}, he = (e, o) => {
  const r = -(/* @__PURE__ */ new Date()).getTimezoneOffset(), t = new Date((e + r * 60) * 1e3);
  let n = t.getUTCDate();
  n < 10 && (n = "0" + n);
  let a = t.getUTCMonth() + 1;
  a < 10 && (a = "0" + a);
  const i = t.getUTCFullYear();
  let s = t.getUTCHours();
  s < 10 && (s = "0" + s);
  let l = t.getUTCMinutes();
  l < 10 && (l = "0" + l);
  let d = t.getUTCSeconds();
  return d < 10 && (d = "0" + d), `${i}/${a}/${n} ${s}:${l}:${d}`;
}, Te = (e) => {
  const o = e.length_min > 20 ? e.length_min : 20, r = e.include_lower_case || 1, t = e.include_upper_case || 1, n = e.include_digits || 1, a = e.include_digits || 1;
  for (; ; ) {
    let i = 0, s = 0, l = 0, d = 0, u = [];
    for (let T = 0; T < o; T += 1) {
      let c = 60;
      for (; c > 57 && c < 65 || c > 90 && c < 97; ) c = Math.floor(Math.random() * 74) + 33;
      c >= 91 && c <= 122 && (i += 1), c >= 65 && c <= 90 && (s += 1), c >= 48 && c <= 57 && (l += 1), c >= 33 && c <= 47 && (d += 1), u.push(String.fromCharCode(c));
    }
    if (!(i < r || s < t || d < a || l < n)) return u.join("");
  }
}, D = (e) => {
  let o = [];
  e = e || 8;
  for (let r = 0; r < e; r += 1) {
    let t = 95;
    for (; t > 90 && t < 97; ) t = Math.floor(Math.random() * 57) + 65;
    o.push(String.fromCharCode(t));
  }
  return o.join("");
};
function Ce(e) {
  let o = e + "=", t = decodeURIComponent(document.cookie).split(";");
  for (let n = 0; n < t.length; n++) {
    let a = t[n];
    for (; a.charAt(0) === " "; ) a = a.substring(1);
    if (a.indexOf(o) === 0) return a.substring(o.length, a.length);
  }
  return "";
}
function Ee(e, o) {
  let r;
  return Promise.race([e, new Promise((t, n) => {
    r = setTimeout(n, o, "timeout");
  })]).finally(() => clearTimeout(r));
}
export {
  Te as A,
  me as B,
  m as C,
  M as D,
  b as E,
  Z as F,
  O as G,
  fe as H,
  G as I,
  K as J,
  B as K,
  V as L,
  Y as M,
  ce as N,
  oe as O,
  S as P,
  ee as Q,
  k as R,
  ne as S,
  Q as T,
  A as _,
  z as a,
  y as b,
  de as c,
  se as d,
  ae as e,
  _e as f,
  D as g,
  $ as h,
  te as i,
  J as j,
  pe as k,
  he as l,
  W as m,
  F as n,
  Ce as o,
  Ee as p,
  H as q,
  re as r,
  ie as s,
  ge as t,
  le as u,
  ue as v,
  X as w,
  x,
  j as y,
  q as z
};
