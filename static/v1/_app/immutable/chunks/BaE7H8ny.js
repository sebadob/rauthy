function v(e) {
  return e && e.__esModule && Object.prototype.hasOwnProperty.call(e, "default") ? e.default : e;
}
var g = {}, E;
function R() {
  if (E) return g;
  E = 1, Object.defineProperty(g, "__esModule", { value: true });
  function e(t) {
    return btoa(t).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
  }
  function n(t, r) {
    t || (t = 43);
    for (var a = typeof window < "u" && window.msCrypto || crypto, s = e(Array.prototype.map.call(a.getRandomValues(new Uint8Array(t)), function(u) {
      return String.fromCharCode(u);
    }).join("")).substring(0, t), c = new Uint8Array(s.length), i = 0; i < s.length; i++) c[i] = s.charCodeAt(i);
    var l = a.subtle.digest("SHA-256", c);
    typeof window < "u" && window.CryptoOperation ? (l.onerror = r, l.oncomplete = function(u) {
      o(r, s, u.target.result);
    }) : l.then(function(u) {
      o(r, s, u);
    }).catch(r);
  }
  g.default = n;
  function o(t, r, a) {
    t(null, { verifier: r, challenge: e(String.fromCharCode.apply(null, new Uint8Array(a))) });
  }
  return g;
}
var A = R();
const U = v(A), m = "pkce_verifier", C = "pkce_verifier_upstream", I = "csrf_token", S = "access_token", w = "id_token", h = "provider_token", y = "/auth/v1/oidc/authorize", L = "rauthy", P = "/auth/v1/oidc/logout", N = "/auth/v1/oidc/callback", $ = "/auth/v1/admin/users", M = "/auth/v1/account", O = "/auth/v1/", b = ["info", "notice", "warning", "critical"], F = ["-", "InvalidLogins", "IpBlacklisted", "IpBlacklistRemoved", "JwksRotated", "NewUserRegistered", "NewRauthyAdmin", "NewRauthyVersion", "PossibleBruteForce", "RauthyStarted", "RauthyHealthy", "RauthyUnhealthy", "SecretsMigrated", "UserEmailChange", "UserPasswordReset", "Test"], V = ["de", "en", "zh", "ko"], G = ["de", "en"], K = "tpl_auth_providers", B = "tpl_client_name", j = "tpl_client_url", H = "tpl_csrf_token", x = "tpl_email_old", z = "tpl_email_new", q = "tpl_error_text", W = "tpl_device_user_code_length", J = "tpl_is_reg_open", Y = "tpl_login_action", Q = "tpl_password_reset", X = "tpl_status_code", Z = "tpl_restricted_email_domain";
var _ = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/", f = typeof Uint8Array > "u" ? [] : new Uint8Array(256);
for (var p = 0; p < _.length; p++) f[_.charCodeAt(p)] = p;
var D = function(e) {
  var n = new Uint8Array(e), o, t = n.length, r = "";
  for (o = 0; o < t; o += 3) r += _[n[o] >> 2], r += _[(n[o] & 3) << 4 | n[o + 1] >> 4], r += _[(n[o + 1] & 15) << 2 | n[o + 2] >> 6], r += _[n[o + 2] & 63];
  return t % 3 === 2 ? r = r.substring(0, r.length - 1) + "=" : t % 3 === 1 && (r = r.substring(0, r.length - 2) + "=="), r;
}, ee = function(e) {
  var n = e.length * 0.75, o = e.length, t, r = 0, a, s, c, i;
  e[e.length - 1] === "=" && (n--, e[e.length - 2] === "=" && n--);
  var l = new ArrayBuffer(n), u = new Uint8Array(l);
  for (t = 0; t < o; t += 4) a = f[e.charCodeAt(t)], s = f[e.charCodeAt(t + 1)], c = f[e.charCodeAt(t + 2)], i = f[e.charCodeAt(t + 3)], u[r++] = a << 2 | s >> 4, u[r++] = (s & 15) << 4 | c >> 2, u[r++] = (c & 3) << 6 | i & 63;
  return l;
};
function te(e) {
  return `${window.location.origin}/auth/${e}/profile#me`;
}
function re() {
  return typeof window < "u";
}
function oe(e) {
  return e === "openid" || e === "profile" || e === "email" || e === "groups" || e === "address" || e === "phone";
}
const ne = (e) => {
  U(64, (n, { challenge: o, verifier: t }) => {
    if (!n) {
      localStorage.setItem(m, t);
      const r = k(24), a = e || "admin", s = `${window.location.origin}${N}`.replaceAll(":", "%3A").replaceAll("/", "%2F");
      window.location.href = `${y}?client_id=${L}&redirect_uri=${s}&response_type=code&code_challenge=${o}&code_challenge_method=S256&scope=openid+profile+email&nonce=${r}&state=${a}`;
    }
  });
}, ae = () => {
  const e = `${P}?post_logout_redirect_uri=${window.location.origin}${O}`;
  window.location.href = e;
}, se = (e) => {
  localStorage.setItem(I, e);
}, ie = (e) => {
  localStorage.setItem(w, e);
}, ce = (e) => {
  localStorage.setItem(S, e);
}, le = (e) => {
  localStorage.setItem(h, e);
}, de = () => localStorage.getItem(h) || "", ue = () => localStorage.getItem(m) || "", _e = () => localStorage.getItem(C) || "", fe = () => {
  localStorage.removeItem(m);
}, ge = () => {
  localStorage.removeItem(I), localStorage.removeItem(w), localStorage.removeItem(S), localStorage.removeItem(m), localStorage.removeItem(C), localStorage.removeItem(h);
};
function pe(e) {
  let n = D(e);
  const o = { "+": "-", "/": "_", "=": "" };
  return n.replace(/[+/=]/g, (t) => o[t]);
}
function me(e) {
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
const he = (e, n) => {
  let o;
  if (n ? o = Date.parse(`${e}T${n}`) : o = Date.parse(e), !isNaN(o)) return o / 1e3;
}, Te = (e, n) => {
  const o = -(/* @__PURE__ */ new Date()).getTimezoneOffset(), t = new Date((e + o * 60) * 1e3);
  let r = t.getUTCDate();
  r < 10 && (r = "0" + r);
  let a = t.getUTCMonth() + 1;
  a < 10 && (a = "0" + a);
  const s = t.getUTCFullYear();
  let c = t.getUTCHours();
  c < 10 && (c = "0" + c);
  let i = t.getUTCMinutes();
  i < 10 && (i = "0" + i);
  let l = t.getUTCSeconds();
  return l < 10 && (l = "0" + l), `${s}/${a}/${r} ${c}:${i}:${l}`;
}, Ee = (e) => {
  const n = e.length_min > 20 ? e.length_min : 20, o = e.include_lower_case || 1, t = e.include_upper_case || 1, r = e.include_digits || 1, a = e.include_digits || 1;
  for (; ; ) {
    let s = 0, c = 0, i = 0, l = 0, u = [];
    for (let T = 0; T < n; T += 1) {
      let d = 60;
      for (; d > 57 && d < 65 || d > 90 && d < 97; ) d = Math.floor(Math.random() * 74) + 33;
      d >= 91 && d <= 122 && (s += 1), d >= 65 && d <= 90 && (c += 1), d >= 48 && d <= 57 && (i += 1), d >= 33 && d <= 47 && (l += 1), u.push(String.fromCharCode(d));
    }
    if (!(s < o || c < t || l < a || i < r)) return u.join("");
  }
}, k = (e) => {
  let n = [];
  e = e || 8;
  for (let o = 0; o < e; o += 1) {
    let t = 95;
    for (; t > 90 && t < 97; ) t = Math.floor(Math.random() * 57) + 65;
    n.push(String.fromCharCode(t));
  }
  return n.join("");
};
function Ce(e) {
  let n = e + "=", t = decodeURIComponent(document.cookie).split(";");
  for (let r = 0; r < t.length; r++) {
    let a = t[r];
    for (; a.charAt(0) === " "; ) a = a.substring(1);
    if (a.indexOf(n) === 0) return a.substring(n.length, a.length);
  }
  return "";
}
function Ie(e, n) {
  let o;
  return Promise.race([e, new Promise((t, r) => {
    o = setTimeout(r, n, "timeout");
  })]).finally(() => clearTimeout(o));
}
export {
  Ee as A,
  he as B,
  I as C,
  b as D,
  F as E,
  ee as F,
  D as G,
  pe as H,
  K as I,
  B as J,
  j as K,
  G as L,
  Y as M,
  U as N,
  le as O,
  C as P,
  oe as Q,
  $ as R,
  te as S,
  X as T,
  ae as U,
  q as a,
  L as b,
  ue as c,
  ie as d,
  se as e,
  fe as f,
  k as g,
  M as h,
  re as i,
  J as j,
  me as k,
  Te as l,
  W as m,
  V as n,
  Ce as o,
  Ie as p,
  H as q,
  ne as r,
  ce as s,
  ge as t,
  de as u,
  _e as v,
  Z as w,
  x,
  z as y,
  Q as z
};
