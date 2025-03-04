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
  function n(t, o) {
    t || (t = 43);
    for (var a = typeof window < "u" && window.msCrypto || crypto, s = e(Array.prototype.map.call(a.getRandomValues(new Uint8Array(t)), function(u) {
      return String.fromCharCode(u);
    }).join("")).substring(0, t), c = new Uint8Array(s.length), i = 0; i < s.length; i++) c[i] = s.charCodeAt(i);
    var l = a.subtle.digest("SHA-256", c);
    typeof window < "u" && window.CryptoOperation ? (l.onerror = o, l.oncomplete = function(u) {
      r(o, s, u.target.result);
    }) : l.then(function(u) {
      r(o, s, u);
    }).catch(o);
  }
  g.default = n;
  function r(t, o, a) {
    t(null, { verifier: o, challenge: e(String.fromCharCode.apply(null, new Uint8Array(a))) });
  }
  return g;
}
var A = R();
const U = v(A), m = "pkce_verifier", C = "pkce_verifier_upstream", I = "csrf_token", S = "access_token", w = "id_token", h = "provider_token", L = "/auth/v1/oidc/authorize", y = "rauthy", P = "/auth/v1/oidc/logout", N = "/auth/v1/oidc/callback", $ = "/auth/v1/admin/users", M = "/auth/v1/account", O = "/auth/v1/", b = ["info", "notice", "warning", "critical"], F = ["-", "InvalidLogins", "IpBlacklisted", "IpBlacklistRemoved", "JwksRotated", "NewUserRegistered", "NewRauthyAdmin", "NewRauthyVersion", "PossibleBruteForce", "RauthyStarted", "RauthyHealthy", "RauthyUnhealthy", "SecretsMigrated", "UserEmailChange", "UserPasswordReset", "Test"], V = ["de", "en", "zh", "ko"], G = ["de", "en"], K = "tpl_auth_providers", B = "tpl_client_name", j = "tpl_client_url", H = "tpl_client_logo_updated", x = "tpl_csrf_token", z = "tpl_email_old", q = "tpl_email_new", W = "tpl_error_text", J = "tpl_device_user_code_length", Y = "tpl_is_reg_open", Q = "tpl_login_action", X = "tpl_password_reset", Z = "tpl_status_code", ee = "tpl_restricted_email_domain";
var _ = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/", f = typeof Uint8Array > "u" ? [] : new Uint8Array(256);
for (var p = 0; p < _.length; p++) f[_.charCodeAt(p)] = p;
var D = function(e) {
  var n = new Uint8Array(e), r, t = n.length, o = "";
  for (r = 0; r < t; r += 3) o += _[n[r] >> 2], o += _[(n[r] & 3) << 4 | n[r + 1] >> 4], o += _[(n[r + 1] & 15) << 2 | n[r + 2] >> 6], o += _[n[r + 2] & 63];
  return t % 3 === 2 ? o = o.substring(0, o.length - 1) + "=" : t % 3 === 1 && (o = o.substring(0, o.length - 2) + "=="), o;
}, te = function(e) {
  var n = e.length * 0.75, r = e.length, t, o = 0, a, s, c, i;
  e[e.length - 1] === "=" && (n--, e[e.length - 2] === "=" && n--);
  var l = new ArrayBuffer(n), u = new Uint8Array(l);
  for (t = 0; t < r; t += 4) a = f[e.charCodeAt(t)], s = f[e.charCodeAt(t + 1)], c = f[e.charCodeAt(t + 2)], i = f[e.charCodeAt(t + 3)], u[o++] = a << 2 | s >> 4, u[o++] = (s & 15) << 4 | c >> 2, u[o++] = (c & 3) << 6 | i & 63;
  return l;
};
function oe(e) {
  return `${window.location.origin}/auth/${e}/profile#me`;
}
function re() {
  return typeof window < "u";
}
function ne(e) {
  return e === "openid" || e === "profile" || e === "email" || e === "groups" || e === "address" || e === "phone";
}
const ae = (e) => {
  U(64, (n, { challenge: r, verifier: t }) => {
    if (!n) {
      localStorage.setItem(m, t);
      const o = k(24), a = e || "admin", s = `${window.location.origin}${N}`.replaceAll(":", "%3A").replaceAll("/", "%2F");
      window.location.href = `${L}?client_id=${y}&redirect_uri=${s}&response_type=code&code_challenge=${r}&code_challenge_method=S256&scope=openid+profile+email&nonce=${o}&state=${a}`;
    }
  });
}, se = () => {
  const e = `${P}?post_logout_redirect_uri=${window.location.origin}${O}`;
  window.location.href = e;
}, ie = (e) => {
  localStorage.setItem(I, e);
}, ce = (e) => {
  localStorage.setItem(w, e);
}, le = (e) => {
  localStorage.setItem(S, e);
}, de = (e) => {
  localStorage.setItem(h, e);
}, ue = () => localStorage.getItem(h) || "", _e = () => localStorage.getItem(m) || "", fe = () => localStorage.getItem(C) || "", ge = () => {
  localStorage.removeItem(m);
}, pe = () => {
  localStorage.removeItem(I), localStorage.removeItem(w), localStorage.removeItem(S), localStorage.removeItem(m), localStorage.removeItem(C), localStorage.removeItem(h);
};
function me(e) {
  let n = D(e);
  const r = { "+": "-", "/": "_", "=": "" };
  return n.replace(/[+/=]/g, (t) => r[t]);
}
function he(e) {
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
const Te = (e, n) => {
  let r;
  if (n ? r = Date.parse(`${e}T${n}`) : r = Date.parse(e), !isNaN(r)) return r / 1e3;
}, Ee = (e, n) => {
  const r = -(/* @__PURE__ */ new Date()).getTimezoneOffset(), t = new Date((e + r * 60) * 1e3);
  let o = t.getUTCDate();
  o < 10 && (o = "0" + o);
  let a = t.getUTCMonth() + 1;
  a < 10 && (a = "0" + a);
  const s = t.getUTCFullYear();
  let c = t.getUTCHours();
  c < 10 && (c = "0" + c);
  let i = t.getUTCMinutes();
  i < 10 && (i = "0" + i);
  let l = t.getUTCSeconds();
  return l < 10 && (l = "0" + l), `${s}/${a}/${o} ${c}:${i}:${l}`;
}, Ce = (e) => {
  const n = e.length_min > 20 ? e.length_min : 20, r = e.include_lower_case || 1, t = e.include_upper_case || 1, o = e.include_digits || 1, a = e.include_digits || 1;
  for (; ; ) {
    let s = 0, c = 0, i = 0, l = 0, u = [];
    for (let T = 0; T < n; T += 1) {
      let d = 60;
      for (; d > 57 && d < 65 || d > 90 && d < 97; ) d = Math.floor(Math.random() * 74) + 33;
      d >= 91 && d <= 122 && (s += 1), d >= 65 && d <= 90 && (c += 1), d >= 48 && d <= 57 && (i += 1), d >= 33 && d <= 47 && (l += 1), u.push(String.fromCharCode(d));
    }
    if (!(s < r || c < t || l < a || i < o)) return u.join("");
  }
}, k = (e) => {
  let n = [];
  e = e || 8;
  for (let r = 0; r < e; r += 1) {
    let t = 95;
    for (; t > 90 && t < 97; ) t = Math.floor(Math.random() * 57) + 65;
    n.push(String.fromCharCode(t));
  }
  return n.join("");
};
function Ie(e) {
  let n = e + "=", t = decodeURIComponent(document.cookie).split(";");
  for (let o = 0; o < t.length; o++) {
    let a = t[o];
    for (; a.charAt(0) === " "; ) a = a.substring(1);
    if (a.indexOf(n) === 0) return a.substring(n.length, a.length);
  }
  return "";
}
function Se(e, n) {
  let r;
  return Promise.race([e, new Promise((t, o) => {
    r = setTimeout(o, n, "timeout");
  })]).finally(() => clearTimeout(r));
}
export {
  Ce as A,
  Te as B,
  I as C,
  b as D,
  F as E,
  K as F,
  B as G,
  j as H,
  H as I,
  Q as J,
  U as K,
  G as L,
  de as M,
  te as N,
  D as O,
  C as P,
  me as Q,
  $ as R,
  ne as S,
  Z as T,
  oe as U,
  se as V,
  W as a,
  y as b,
  _e as c,
  ce as d,
  ie as e,
  ge as f,
  k as g,
  M as h,
  re as i,
  Y as j,
  he as k,
  Ee as l,
  J as m,
  V as n,
  Ie as o,
  Se as p,
  x as q,
  ae as r,
  le as s,
  pe as t,
  ue as u,
  fe as v,
  ee as w,
  z as x,
  q as y,
  X as z
};
