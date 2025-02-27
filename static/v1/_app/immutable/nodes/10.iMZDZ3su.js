import { t as D, a as u, d as Pe, e as J } from "../chunks/D8nUqfqE.js";
import { p as re, c, s as a, k as L, t as j, l as s, j as t, a as ae, r as d, a9 as he, f as te, aa as _e, a7 as z, a5 as Yt } from "../chunks/D-nwkJyM.js";
import { d as Fe, e as Ct, s as I } from "../chunks/DmLjbmU6.js";
import { i as ee } from "../chunks/C3XMhfdI.js";
import { e as We, i as Pt } from "../chunks/6MDunRKZ.js";
import { a as H, B as se, t as Zt, s as $t } from "../chunks/BUAPoI3e.js";
import { p as i } from "../chunks/BiI21XkS.js";
import { B as er } from "../chunks/BB-qItao.js";
import { C as tr } from "../chunks/C-Lg4I_M.js";
import { N as rr } from "../chunks/MNfc1IKL.js";
import { N as ar } from "../chunks/B82wSkDx.js";
import { O as lr } from "../chunks/TvawFHML.js";
import { b as ze, d as He, c as Rt, f as Lt } from "../chunks/emZtalxW.js";
import { u as ir } from "../chunks/C6Ehfmfa.js";
import { h as we } from "../chunks/1rF6JvjJ.js";
import { I as fe } from "../chunks/BE8gxf21.js";
import { n as nr, g as pt, a as qe, o as sr, p as or, q as Et } from "../chunks/BRCxk8by.js";
import { F as Ke } from "../chunks/BgEKkuL8.js";
import { u as xe } from "../chunks/CMjKUQkH.js";
import { u as ge } from "../chunks/DMJUG0wm.js";
import { I as ve } from "../chunks/CFT15Ou1.js";
import { r as Ve, s as W } from "../chunks/D_HYGYLR.js";
import { b as Ge } from "../chunks/DAX5MQt8.js";
import { b as Mt } from "../chunks/CwQ5lQAV.js";
import { p as $ } from "../chunks/2rFDT0Lm.js";
import { g as ke } from "../chunks/DppGgfa0.js";
import { I as dr } from "../chunks/Dq3vuPfn.js";
import { T as Ut } from "../chunks/C1ZebG9t.js";
import { I as Nt } from "../chunks/C0_UZAx3.js";
import { L as Be } from "../chunks/CvnJ-bH5.js";
import { S as St } from "../chunks/BK_OTjGh.js";
import { O as At } from "../chunks/Fz9KQU3M.js";
import { I as cr } from "../chunks/Bw4crv1X.js";
import { k as vr } from "../chunks/D0NByJFk.js";
import { T as ur } from "../chunks/DmGHSNVM.js";
import { I as fr } from "../chunks/9p_FdTsB.js";
function hr(U, e, r, l) {
  var _a;
  e(), (_a = r.onInput) == null ? void 0 : _a.call(r, t(l));
}
function _r(U, e, r) {
  var _a;
  switch (U.code) {
    case "Enter":
      e(), (_a = r.onEnter) == null ? void 0 : _a.call(r);
      break;
  }
}
function gr(U, e) {
  var _a, _b;
  (_a = e()) == null ? void 0 : _a.focus(), (_b = e()) == null ? void 0 : _b.scrollIntoView({ behavior: "smooth", block: "center" });
}
var mr = D('<li class="value svelte-r5y8pk"><div class="label"> </div> <div class="btnClose svelte-r5y8pk"><!></div></li>'), br = D("<option></option>"), kr = D('<datalist class="absolute svelte-r5y8pk"></datalist>'), xr = D('<span class="err"> </span>'), wr = D('<div role="none" class="container svelte-r5y8pk"><ul class="svelte-r5y8pk"><!> <li class="svelte-r5y8pk"><input autocomplete="off" class="svelte-r5y8pk"> <!></li></ul> <label class="font-label noselect svelte-r5y8pk"> <!></label></div>');
function be(U, e) {
  re(e, true);
  let r = $(e, "ref", 15), l = $(e, "typ", 3, "text"), v = $(e, "id", 19, ke), f = $(e, "values", 31, () => i([])), g = $(e, "label", 3, ""), _ = $(e, "placeholder", 3, ""), b = $(e, "disabled", 3, false), m = $(e, "required", 3, false), h = $(e, "isError", 15, false), k = $(e, "width", 3, "inherit"), R = $(e, "maxHeightList", 3, "inherit"), x = xe();
  const w = ke();
  let y = L(""), S = he(() => e.datalist && e.datalist.length > 0 ? w : void 0);
  function B(q) {
    f(f().filter((X) => X !== q));
  }
  function N(q) {
    var _a;
    C(), (_a = e.onBlur) == null ? void 0 : _a.call(e), E();
  }
  function F(q) {
    q.preventDefault(), h(true);
  }
  function E() {
    t(y) && C() && (f().push(t(y)), s(y, ""));
  }
  function C() {
    var _a;
    let q = (_a = r()) == null ? void 0 : _a.validity;
    return q ? (h(!q.valid), q.valid) : (h(false), true);
  }
  var P = wr();
  P.__click = [gr, r];
  var o = c(P), p = c(o);
  We(p, 17, f, Pt, (q, X, ue, oe) => {
    var de = mr(), ie = c(de), O = c(ie, true);
    d(ie);
    var ne = a(ie, 2), ce = c(ne);
    se(ce, { invisible: true, onclick: () => B(t(X)), children: (ye, Le) => {
      dr(ye, { width: "1.2rem" });
    }, $$slots: { default: true } }), d(ne), d(de), j(() => I(O, t(X))), u(q, de);
  });
  var Q = a(p, 2), A = c(Q);
  Ve(A), A.__keydown = [_r, E, e], A.__input = [hr, C, e, y], Mt(A, (q) => r(q), () => r());
  var M = a(A, 2);
  {
    var K = (q) => {
      var X = kr();
      W(X, "id", w), We(X, 21, () => e.datalist, Pt, (ue, oe, de, ie) => {
        var O = br(), ne = {};
        j(() => {
          ne !== (ne = t(oe)) && (O.value = (O.__value = t(oe)) == null ? "" : t(oe));
        }), u(ue, O);
      }), d(X), u(q, X);
    };
    ee(M, (q) => {
      e.datalist && e.datalist.length > 1 && q(K);
    });
  }
  d(Q), d(o);
  var V = a(o, 2), G = c(V), le = a(G);
  {
    var Z = (q) => {
      var X = Pe(), ue = te(X);
      {
        var oe = (de) => {
          var ie = xr(), O = c(ie, true);
          d(ie), j(() => I(O, e.errMsg || x.common.invalidInput)), u(de, ie);
        };
        ee(ue, (de) => {
          h() && de(oe);
        });
      }
      u(q, X);
    };
    ee(le, (q) => {
      h() && q(Z);
    });
  }
  d(V), d(P), j(() => {
    H(P, "width", k()), H(o, "max-height", R()), W(A, "type", l()), W(A, "id", v()), W(A, "name", e.name), W(A, "list", t(S)), W(A, "title", e.errMsg), W(A, "aria-label", g() || _()), W(A, "placeholder", _()), W(A, "aria-placeholder", _()), A.disabled = b(), W(A, "aria-disabled", b()), A.required = m() && f().length < 1, W(A, "aria-required", m() && f().length < 1), W(A, "maxlength", e.maxLength || void 0), W(A, "min", e.min || void 0), W(A, "max", e.max || void 0), W(A, "pattern", e.pattern), W(V, "for", v()), W(V, "data-required", m()), I(G, `${g() ?? ""} `);
  }), Ct("invalid", A, F), Ct("blur", A, N), Ge(A, () => t(y), (q) => s(y, q)), u(U, P), ae();
}
Fe(["click", "keydown", "input"]);
var yr = D('<div class="err"> </div>'), Cr = D('<!> <!> <!> <p class="svelte-15tp6i6"><!></p> <!> <!> <!> <!>', 1), Pr = D('<div class="container svelte-15tp6i6"><!></div>');
function Lr(U, e) {
  re(e, true);
  let r = xe(), l = ge(), v = L(void 0), f = L(""), g = L(""), _ = L(""), b = L(true), m = L(i([])), h = L(i([]));
  _e(() => {
    requestAnimationFrame(() => {
      var _a;
      (_a = t(v)) == null ? void 0 : _a.focus();
    });
  });
  async function k(w, y) {
    var _a;
    if (e.clients.find((N) => N.id === t(g))) {
      s(f, i(l.common.nameExistsAlready));
      return;
    }
    s(f, "");
    let S = { id: t(g), name: t(_) || void 0, confidential: t(b), redirect_uris: t(m), post_logout_redirect_uris: t(h).length > 0 ? t(h) : void 0 }, B = await ze(w.action, S);
    B.body ? e.onSave(B.body.id) : s(f, i(((_a = B.error) == null ? void 0 : _a.message) || "Error"));
  }
  var R = Pr(), x = c(R);
  Ke(x, { action: "/auth/v1/clients", onSubmit: k, children: (w, y) => {
    var S = Cr(), B = te(S);
    fe(B, { autocomplete: "off", label: "Client ID", placeholder: "Client ID", required: true, pattern: nr, get ref() {
      return t(v);
    }, set ref(M) {
      s(v, i(M));
    }, get value() {
      return t(g);
    }, set value(M) {
      s(g, i(M));
    } });
    var N = a(B, 2);
    fe(N, { autocomplete: "off", get label() {
      return l.clients.name;
    }, get placeholder() {
      return l.clients.name;
    }, pattern: pt, get value() {
      return t(_);
    }, set value(M) {
      s(_, i(M));
    } });
    var F = a(N, 2);
    ve(F, { get ariaLabel() {
      return l.clients.confidential;
    }, get checked() {
      return t(b);
    }, set checked(M) {
      s(b, i(M));
    }, children: (M, K) => {
      z();
      var V = J();
      j(() => I(V, l.clients.confidential)), u(M, V);
    }, $$slots: { default: true } });
    var E = a(F, 2), C = c(E);
    we(C, () => l.clients.descUri), d(E);
    var P = a(E, 2);
    be(P, { typ: "url", label: "Redirect URIs", get errMsg() {
      return l.validation.uri;
    }, get values() {
      return t(m);
    }, set values(M) {
      s(m, i(M));
    } });
    var o = a(P, 2);
    be(o, { typ: "url", label: "Post Logout Redirect URIs", get errMsg() {
      return l.validation.uri;
    }, get values() {
      return t(h);
    }, set values(M) {
      s(h, i(M));
    } });
    var p = a(o, 2);
    se(p, { type: "submit", children: (M, K) => {
      z();
      var V = J();
      j(() => I(V, r.common.save)), u(M, V);
    }, $$slots: { default: true } });
    var Q = a(p, 2);
    {
      var A = (M) => {
        var K = yr(), V = c(K, true);
        d(K), j(() => I(V, t(f))), u(M, K);
      };
      ee(Q, (M) => {
        t(f) && M(A);
      });
    }
    u(w, S);
  }, $$slots: { default: true } }), d(R), u(U, R), ae();
}
const je = "urn:ietf:params:oauth:grant-type:device_code";
var Sr = D('<div class="err"> </div>'), Ar = D('<div class="err"> </div>'), Tr = D('<h5> </h5> <!> <p class="desc svelte-5tvvvt"> </p> <!> <p class="desc svelte-5tvvvt"> </p> <!> <!> <div></div> <h5> </h5> <!> <!> <!> <p class="mb-0 svelte-5tvvvt">Authentication Flows</p> <!> <!> <!> <!> <!> <div></div> <p class="desc svelte-5tvvvt"> </p> <p class="desc svelte-5tvvvt"><strong> </strong></p> <!> <!> <!> <div></div> <p class="desc svelte-5tvvvt"> </p> <!> <p class="desc svelte-5tvvvt"><!></p> <!> <!> <div></div> <p class="desc svelte-5tvvvt"><!></p> <!> <!> <div></div> <p> </p> <!> <div></div> <p> </p> <p> </p> <!> <!> <div></div> <p> </p> <!> <div class="flex gap-05"><!> <!></div> <!> <div></div>', 1), Ir = D('<div class="container svelte-5tvvvt"><!></div>');
function Rr(U, e) {
  var _a, _b;
  re(e, true);
  let r = xe(), l = ge();
  const v = "min(20rem, calc(100dvw - .5rem))";
  let f = L(""), g = L(false), _ = L(i(e.client.id)), b = L(i(e.client.name || "")), m = L(i(e.client.enabled)), h = L(i(e.client.confidential)), k = L(i(e.client.client_uri || "")), R = L(i(e.client.contacts ? Array.from(e.client.contacts) : [])), x = L(i(e.client.allowed_origins ? Array.from(e.client.allowed_origins) : [])), w = L(i(Array.from(e.client.redirect_uris))), y = L(i(e.client.post_logout_redirect_uris ? Array.from(e.client.post_logout_redirect_uris) : [])), S = i({ authorizationCode: e.client.flows_enabled.includes("authorization_code"), clientCredentials: e.client.flows_enabled.includes("client_credentials"), password: e.client.flows_enabled.includes("password"), refreshToken: e.client.flows_enabled.includes("refresh_token"), deviceCode: e.client.flows_enabled.includes(je) });
  const B = ["RS256", "RS384", "RS512", "EdDSA"];
  let N = L(i(e.client.access_token_alg)), F = L(i(e.client.id_token_alg)), E = L(i(e.client.access_token_lifetime.toString())), C = L(i(e.client.auth_code_lifetime.toString())), P = L(i(e.scopesAll.map((G) => ({ name: G, selected: e.client.scopes.includes(G) || false })))), o = L(i(e.scopesAll.map((G) => ({ name: G, selected: e.client.default_scopes.includes(G) || false })))), p = i({ plain: ((_a = e.client.challenges) == null ? void 0 : _a.includes("plain")) || false, s256: ((_b = e.client.challenges) == null ? void 0 : _b.includes("S256")) || false }), Q = L(i(e.client.force_mfa));
  _e(() => {
    var _a2, _b2;
    e.client.id && (s(_, i(e.client.id)), s(b, i(e.client.name || "")), s(m, i(e.client.enabled)), s(h, i(e.client.confidential)), s(k, i(e.client.client_uri || "")), s(R, i(e.client.contacts ? Array.from(e.client.contacts) : [])), s(x, i(e.client.allowed_origins ? Array.from(e.client.allowed_origins) : [])), s(w, i(Array.from(e.client.redirect_uris))), s(y, i(e.client.post_logout_redirect_uris ? Array.from(e.client.post_logout_redirect_uris) : [])), S.authorizationCode = e.client.flows_enabled.includes("authorization_code"), S.clientCredentials = e.client.flows_enabled.includes("client_credentials"), S.password = e.client.flows_enabled.includes("password"), S.refreshToken = e.client.flows_enabled.includes("refresh_token"), S.deviceCode = e.client.flows_enabled.includes(je), s(N, i(e.client.access_token_alg)), s(F, i(e.client.id_token_alg)), s(E, i(e.client.access_token_lifetime.toString())), s(C, i(e.client.auth_code_lifetime.toString())), s(P, i(e.scopesAll.map((G) => ({ name: G, selected: e.client.scopes.includes(G) || false })))), s(o, i(e.scopesAll.map((G) => ({ name: G, selected: e.client.default_scopes.includes(G) || false })))), p.plain = ((_a2 = e.client.challenges) == null ? void 0 : _a2.includes("plain")) || false, p.s256 = ((_b2 = e.client.challenges) == null ? void 0 : _b2.includes("S256")) || false);
  });
  async function A(G, le) {
    s(f, "");
    let Z = { id: t(_), name: t(b) || void 0, enabled: t(m), confidential: t(h), allowed_origins: t(x).length > 0 ? t(x) : void 0, redirect_uris: t(w), post_logout_redirect_uris: t(y).length > 0 ? t(y) : void 0, flows_enabled: [], access_token_alg: t(N), id_token_alg: t(F), access_token_lifetime: Number.parseInt(t(E)), auth_code_lifetime: Number.parseInt(t(C)), scopes: t(P).filter((X) => X.selected).map((X) => X.name), default_scopes: t(o).filter((X) => X.selected).map((X) => X.name), challenges: void 0, force_mfa: t(Q), client_uri: t(k) || void 0, contacts: t(R).length > 0 ? t(R) : void 0 };
    S.authorizationCode && Z.flows_enabled.push("authorization_code"), S.clientCredentials && Z.flows_enabled.push("client_credentials"), S.password && Z.flows_enabled.push("password"), S.refreshToken && Z.flows_enabled.push("refresh_token"), S.deviceCode && Z.flows_enabled.push(je), p.plain && (Z.challenges = ["plain"]), p.s256 && (Z.challenges ? Z.challenges.push("S256") : Z.challenges = ["S256"]), console.log("payload", Z);
    let q = await He(G.action, Z);
    q.error ? s(f, i(q.error.message)) : (s(g, true), e.onSave(), setTimeout(() => {
      s(g, false);
    }, 2e3));
  }
  var M = Ir(), K = c(M);
  const V = he(() => `/auth/v1/clients/${e.client.id}`);
  Ke(K, { get action() {
    return t(V);
  }, onSubmit: A, children: (G, le) => {
    var Z = Tr(), q = te(Z), X = c(q, true);
    d(q);
    var ue = a(q, 2);
    Be(ue, { label: "ID", mono: true, children: (n, Y) => {
      z();
      var T = J();
      j(() => I(T, e.client.id)), u(n, T);
    } });
    var oe = a(ue, 2), de = c(oe, true);
    d(oe);
    var ie = a(oe, 2);
    fe(ie, { autocomplete: "off", get label() {
      return l.clients.name;
    }, get placeholder() {
      return l.clients.name;
    }, width: v, pattern: pt, get value() {
      return t(b);
    }, set value(n) {
      s(b, i(n));
    } });
    var O = a(ie, 2), ne = c(O, true);
    d(O);
    var ce = a(O, 2);
    fe(ce, { typ: "url", autocomplete: "off", label: "URI", placeholder: "URI", width: v, pattern: qe, get value() {
      return t(k);
    }, set value(n) {
      s(k, i(n));
    } });
    var ye = a(ce, 2);
    be(ye, { get label() {
      return l.common.contact;
    }, pattern: sr, get values() {
      return t(R);
    }, set values(n) {
      s(R, i(n));
    } });
    var Le = a(ye, 2);
    H(Le, "height", ".5rem");
    var Se = a(Le, 2), qt = c(Se, true);
    d(Se);
    var Je = a(Se, 2);
    ve(Je, { get ariaLabel() {
      return l.common.enabled;
    }, get checked() {
      return t(m);
    }, set checked(n) {
      s(m, i(n));
    }, children: (n, Y) => {
      z();
      var T = J();
      j(() => I(T, l.common.enabled)), u(n, T);
    }, $$slots: { default: true } });
    var Qe = a(Je, 2);
    ve(Qe, { get ariaLabel() {
      return l.clients.confidential;
    }, get checked() {
      return t(h);
    }, set checked(n) {
      s(h, i(n));
    }, children: (n, Y) => {
      z();
      var T = J();
      j(() => I(T, l.clients.confidential)), u(n, T);
    }, $$slots: { default: true } });
    var Xe = a(Qe, 2);
    ve(Xe, { get ariaLabel() {
      return l.clients.forceMfa;
    }, get checked() {
      return t(Q);
    }, set checked(n) {
      s(Q, i(n));
    }, children: (n, Y) => {
      z();
      var T = J();
      j(() => I(T, l.clients.forceMfa)), u(n, T);
    }, $$slots: { default: true } });
    var Ye = a(Xe, 4);
    ve(Ye, { ariaLabel: "authorization_code", get checked() {
      return S.authorizationCode;
    }, set checked(n) {
      S.authorizationCode = n;
    }, children: (n, Y) => {
      z();
      var T = J("authorization_code");
      u(n, T);
    }, $$slots: { default: true } });
    var Ze = a(Ye, 2);
    ve(Ze, { ariaLabel: "urn:ietf:params:oauth:grant-type:device_code", get checked() {
      return S.deviceCode;
    }, set checked(n) {
      S.deviceCode = n;
    }, children: (n, Y) => {
      z();
      var T = J("device_code");
      u(n, T);
    }, $$slots: { default: true } });
    var $e = a(Ze, 2);
    ve($e, { ariaLabel: "client_credentials", get checked() {
      return S.clientCredentials;
    }, set checked(n) {
      S.clientCredentials = n;
    }, children: (n, Y) => {
      z();
      var T = J("client_credentials");
      u(n, T);
    }, $$slots: { default: true } });
    var et = a($e, 2);
    ve(et, { ariaLabel: "password", get checked() {
      return S.password;
    }, set checked(n) {
      S.password = n;
    }, children: (n, Y) => {
      z();
      var T = J("password");
      u(n, T);
    }, $$slots: { default: true } });
    var tt = a(et, 2);
    ve(tt, { ariaLabel: "refresh_token", get checked() {
      return S.refreshToken;
    }, set checked(n) {
      S.refreshToken = n;
    }, children: (n, Y) => {
      z();
      var T = J("refresh_token");
      u(n, T);
    }, $$slots: { default: true } });
    var rt = a(tt, 2);
    H(rt, "height", ".5rem");
    var Ae = a(rt, 2), Bt = c(Ae, true);
    d(Ae);
    var Te = a(Ae, 2), at = c(Te), jt = c(at, true);
    d(at), d(Te);
    var lt = a(Te, 2);
    ve(lt, { ariaLabel: "PKCE plain", get checked() {
      return p.plain;
    }, set checked(n) {
      p.plain = n;
    }, children: (n, Y) => {
      z();
      var T = J("plain");
      u(n, T);
    }, $$slots: { default: true } });
    var it = a(lt, 2);
    ve(it, { ariaLabel: "PKCE S256", get checked() {
      return p.s256;
    }, set checked(n) {
      p.s256 = n;
    }, children: (n, Y) => {
      z();
      var T = J("S256");
      u(n, T);
    }, $$slots: { default: true } });
    var nt = a(it, 2);
    {
      var Ot = (n) => {
        var Y = Sr(), T = c(Y, true);
        d(Y), j(() => I(T, l.clients.errConfidentialPKCE)), Zt(3, Y, () => $t, () => ({ duration: 150 })), u(n, Y);
      };
      ee(nt, (n) => {
        !t(h) && !p.plain && !p.s256 && n(Ot);
      });
    }
    var st = a(nt, 2);
    H(st, "height", ".5rem");
    var Ie = a(st, 2), Dt = c(Ie, true);
    d(Ie);
    var ot = a(Ie, 2);
    be(ot, { typ: "url", label: "Allowed Origins", get errMsg() {
      return l.validation.origin;
    }, pattern: or, get values() {
      return t(x);
    }, set values(n) {
      s(x, i(n));
    } });
    var Re = a(ot, 2), Wt = c(Re);
    we(Wt, () => l.clients.descUri), d(Re);
    var dt = a(Re, 2);
    be(dt, { typ: "url", label: "Redirect URIs", get errMsg() {
      return l.validation.uri;
    }, get required() {
      return S.authorizationCode;
    }, pattern: qe, get values() {
      return t(w);
    }, set values(n) {
      s(w, i(n));
    } });
    var ct = a(dt, 2);
    be(ct, { typ: "url", label: "Post Logout Redirect URIs", get errMsg() {
      return l.validation.uri;
    }, pattern: qe, get values() {
      return t(y);
    }, set values(n) {
      s(y, i(n));
    } });
    var vt = a(ct, 2);
    H(vt, "height", ".5rem");
    var pe = a(vt, 2), Ft = c(pe);
    we(Ft, () => l.clients.scopes.desc), d(pe);
    var ut = a(pe, 2);
    St(ut, { get items() {
      return t(P);
    }, set items(n) {
      s(P, i(n));
    }, children: (n, Y) => {
      z();
      var T = J();
      j(() => I(T, l.clients.scopes.allowed)), u(n, T);
    }, $$slots: { default: true } });
    var ft = a(ut, 2);
    St(ft, { get items() {
      return t(o);
    }, set items(n) {
      s(o, i(n));
    }, children: (n, Y) => {
      z();
      var T = J();
      j(() => I(T, l.clients.scopes.default)), u(n, T);
    }, $$slots: { default: true } });
    var ht = a(ft, 2);
    H(ht, "height", ".75rem");
    var Ee = a(ht, 2), zt = c(Ee, true);
    d(Ee);
    var _t = a(Ee, 2);
    fe(_t, { typ: "number", autocomplete: "off", label: "Token Lifetime", placeholder: "Token Lifetime", width: v, min: "10", max: "86400", errMsg: "10 <= Token Lifetime <= 86400", get value() {
      return t(E);
    }, set value(n) {
      s(E, i(n));
    } });
    var gt = a(_t, 2);
    H(gt, "height", ".5rem");
    var Me = a(gt, 2), Ht = c(Me, true);
    d(Me);
    var Ue = a(Me, 2), Kt = c(Ue, true);
    d(Ue);
    var mt = a(Ue, 2);
    Be(mt, { label: "Access Token Algorithm", children: (n, Y) => {
      At(n, { ariaLabel: "Access Token Algorithm", options: B, borderless: true, get value() {
        return t(N);
      }, set value(T) {
        s(N, i(T));
      } });
    } });
    var bt = a(mt, 2);
    Be(bt, { label: "ID Token Algorithm", children: (n, Y) => {
      At(n, { ariaLabel: "ID Token Algorithm", options: B, borderless: true, get value() {
        return t(F);
      }, set value(T) {
        s(F, i(T));
      } });
    } });
    var kt = a(bt, 2);
    H(kt, "height", ".5rem");
    var Ne = a(kt, 2), Vt = c(Ne, true);
    d(Ne);
    var xt = a(Ne, 2);
    fe(xt, { typ: "number", autocomplete: "off", label: "Auth Code Lifetime", placeholder: "Auth Code Lifetime", width: v, min: "10", max: "300", errMsg: "10 <= Auth Code Lifetime <= 300", get value() {
      return t(C);
    }, set value(n) {
      s(C, i(n));
    } });
    var Ce = a(xt, 2);
    H(Ce, "margin-top", "1rem");
    var wt = c(Ce);
    se(wt, { type: "submit", children: (n, Y) => {
      z();
      var T = J();
      j(() => I(T, r.common.save)), u(n, T);
    }, $$slots: { default: true } });
    var Gt = a(wt, 2);
    {
      var Jt = (n) => {
        Nt(n, {});
      };
      ee(Gt, (n) => {
        t(g) && n(Jt);
      });
    }
    d(Ce);
    var yt = a(Ce, 2);
    {
      var Qt = (n) => {
        var Y = Ar(), T = c(Y, true);
        d(Y), j(() => I(T, t(f))), u(n, Y);
      };
      ee(yt, (n) => {
        t(f) && n(Qt);
      });
    }
    var Xt = a(yt, 2);
    H(Xt, "height", "1rem"), j(() => {
      I(X, l.common.information), I(de, l.clients.descName), I(ne, l.clients.descClientUri), I(qt, l.clients.config), I(Bt, l.clients.descPKCE), I(jt, l.clients.descPKCEEnforce), I(Dt, l.clients.descOrigin), I(zt, l.clients.tokenLifetime.p1), I(Ht, l.clients.tokenLifetime.p2), I(Kt, l.clients.tokenLifetime.p3), I(Vt, l.clients.descAuthCode);
    }), u(G, Z);
  }, $$slots: { default: true } }), d(M), u(U, M), ae();
}
var pr = D("<!> <!>", 1), Er = D('<div class="container svelte-19xcrje"><div class="err"> </div> <!></div>');
function Mr(U, e) {
  re(e, true);
  let r = ge(), l = L(""), v = L(void 0);
  _e(() => {
    s(l, ""), s(v, ""), e.client.confidential ? f() : s(l, i(r.clients.confidentialNoSecret));
  });
  async function f() {
    let x = await ze(`/auth/v1/clients/${e.client.id}/secret`);
    _(x);
  }
  async function g() {
    let x = await He(`/auth/v1/clients/${e.client.id}/secret`);
    _(x);
  }
  function _(x) {
    var _a;
    x.body ? x.body.secret && s(v, i(x.body.secret)) : s(l, i(((_a = x.error) == null ? void 0 : _a.message) || "Error"));
  }
  var b = Er(), m = c(b), h = c(m, true);
  d(m);
  var k = a(m, 2);
  {
    var R = (x) => {
      var w = pr(), y = te(w);
      cr(y, { autocomplete: "off", label: "Client Secret", placeholder: "Client Secret", disabled: true, showCopy: true, get value() {
        return t(v);
      }, set value(B) {
        s(v, i(B));
      } });
      var S = a(y, 2);
      se(S, { onclick: g, children: (B, N) => {
        z();
        var F = J();
        j(() => I(F, r.clients.generateSecret)), u(B, F);
      }, $$slots: { default: true } }), u(x, w);
    };
    ee(k, (x) => {
      t(v) && x(R);
    });
  }
  d(b), j(() => I(h, t(l))), u(U, b), ae();
}
var Ur = D('<div class="err"> </div>'), Nr = D("<p> </p> <!> <!>", 1);
function qr(U, e) {
  re(e, true);
  let r = xe(), l = ge(), v = L("");
  async function f() {
    s(v, "");
    let R = await Rt(`/auth/v1/clients/${e.client.id}`);
    R.error ? s(v, i(R.error.message)) : e.onSave();
  }
  var g = Nr(), _ = te(g), b = c(_, true);
  d(_);
  var m = a(_, 2);
  se(m, { level: -1, onclick: f, children: (R, x) => {
    z();
    var w = J();
    j(() => I(w, r.common.delete)), u(R, w);
  }, $$slots: { default: true } });
  var h = a(m, 2);
  {
    var k = (R) => {
      var x = Ur(), w = c(x, true);
      d(x), j(() => I(w, t(v))), u(R, x);
    };
    ee(h, (R) => {
      t(v) && R(k);
    });
  }
  j(() => I(b, l.clients.delete1)), u(U, g), ae();
}
var Br = D('<div><div class="flex"><input type="range" class="svelte-12j8dit"> <div class="value font-mono svelte-12j8dit"> </div></div> <div class="label svelte-12j8dit"><label class="font-label noselect svelte-12j8dit"> </label></div></div>');
function Oe(U, e) {
  re(e, true);
  let r = $(e, "value", 15), l = $(e, "label", 3, ""), v = $(e, "disabled", 3, false), f = $(e, "step", 3, 1), g = $(e, "widthRange", 3, "15rem");
  const _ = ke();
  let b = he(() => {
    if (e.hue !== void 0 && e.sat !== void 0 && e.lum !== void 0) {
      if (e.bgMode === "hue") return `linear-gradient(
                    to right,
                    hsl(0 ${e.sat} ${e.lum}) 0%,
                    hsl(60 ${e.sat} ${e.lum}) 17%,
                    hsl(120 ${e.sat} ${e.lum}) 33%,
                    hsl(180 ${e.sat} ${e.lum}) 50%,
                    hsl(240 ${e.sat} ${e.lum}) 67%,
                    hsl(300 ${e.sat} ${e.lum}) 83%,
                    hsl(3600 ${e.sat} ${e.lum}) 100%
                )`;
      if (e.bgMode === "sat") return `linear-gradient(to right, hsl(${e.hue} 0 ${e.lum}), hsl(${e.hue} 100 ${e.lum}))`;
      if (e.bgMode === "lum") return `linear-gradient(to right, hsl(${e.hue} ${e.sat} 0), hsl(${e.hue} ${e.sat} 100))`;
    }
    return "hsl(var(--text) / .5)";
  });
  var m = Br(), h = c(m), k = c(h);
  Ve(k), W(k, "id", _);
  var R = a(k, 2), x = c(R, true);
  d(R), d(h);
  var w = a(h, 2), y = c(w);
  W(y, "for", _);
  var S = c(y, true);
  d(y), d(w), d(m), j(() => {
    W(k, "name", e.name), W(k, "title", l()), W(k, "aria-label", l()), k.disabled = v(), W(k, "aria-disabled", v()), W(k, "min", e.min), W(k, "max", e.max), W(k, "step", f()), H(k, "width", g()), H(k, "--bg-slider", t(b)), I(x, r()), I(S, l());
  }), Ge(k, r), u(U, m), ae();
}
var jr = D('<div class="outer svelte-1evkwuy"><div class="container svelte-1evkwuy"><div><label class="font-label svelte-1evkwuy"> </label> <div class="values svelte-1evkwuy"><!> <!> <!></div></div> <div class="color svelte-1evkwuy"></div></div></div>');
function me(U, e) {
  re(e, true);
  let r = $(e, "h", 15), l = $(e, "s", 15), v = $(e, "l", 15);
  const f = ke(), g = "15rem";
  let _ = he(() => `hsl(${r()} ${l()} ${v()})`);
  var b = jr(), m = c(b), h = c(m), k = c(h);
  W(k, "for", f);
  var R = c(k, true);
  d(k);
  var x = a(k, 2);
  W(x, "id", f);
  var w = c(x);
  Oe(w, { label: "Hue", min: 0, max: 359, widthRange: g, bgMode: "hue", get hue() {
    return r();
  }, get sat() {
    return l();
  }, get lum() {
    return v();
  }, get value() {
    return r();
  }, set value(N) {
    r(N);
  } });
  var y = a(w, 2);
  Oe(y, { label: "Sat", min: 0, max: 100, widthRange: g, bgMode: "sat", get hue() {
    return r();
  }, get sat() {
    return l();
  }, get lum() {
    return v();
  }, get value() {
    return l();
  }, set value(N) {
    l(N);
  } });
  var S = a(y, 2);
  Oe(S, { label: "Lum", min: 0, max: 100, widthRange: g, bgMode: "lum", get hue() {
    return r();
  }, get sat() {
    return l();
  }, get lum() {
    return v();
  }, get value() {
    return v();
  }, set value(N) {
    v(N);
  } }), d(x), d(h);
  var B = a(h, 2);
  d(m), d(b), j(() => {
    H(m, "border-color", t(_)), I(R, e.label), H(B, "background", t(_));
  }), u(U, b), ae();
}
var Or = (U, e) => {
  var _a;
  return (_a = t(e)) == null ? void 0 : _a.click();
}, Dr = D('<div class="container svelte-lcwq3j"><!> <div role="none" class="mask svelte-lcwq3j"><div class="colorWrapper svelte-lcwq3j"><div class="color svelte-lcwq3j"></div></div> <input type="color" class="svelte-lcwq3j"></div></div>');
function De(U, e) {
  re(e, true);
  let r = $(e, "value", 15), l = ge(), v = L(void 0);
  var f = Dr(), g = c(f);
  fe(g, { get label() {
    return e.label;
  }, get placeholder() {
    return e.label;
  }, get errMsg() {
    return l.validation.css;
  }, get width() {
    return e.inputWidth;
  }, required: true, pattern: Et, get value() {
    return r();
  }, set value(m) {
    r(m);
  } });
  var _ = a(g, 2);
  _.__click = [Or, v];
  var b = a(c(_), 2);
  Ve(b), Mt(b, (m) => s(v, m), () => t(v)), d(_), d(f), j(() => H(f, "--color", r())), Ge(b, r), u(U, f), ae();
}
Fe(["click"]);
var Wr = D('<div><p> </p> <div class="hsl svelte-10udpph"><div><!> <!> <!> <!></div> <div><!> <!> <!></div></div> <div><p><!></p> <!> <!> <!></div></div>');
function Tt(U, e) {
  re(e, true);
  let r = $(e, "values", 15), l = ge();
  var v = Wr(), f = c(v), g = c(f, true);
  d(f);
  var _ = a(f, 2), b = c(_), m = c(b);
  me(m, { label: "--text", get h() {
    return r().text[0];
  }, set h(o) {
    r(r().text[0] = o, true);
  }, get s() {
    return r().text[1];
  }, set s(o) {
    r(r().text[1] = o, true);
  }, get l() {
    return r().text[2];
  }, set l(o) {
    r(r().text[2] = o, true);
  } });
  var h = a(m, 2);
  me(h, { label: "--text-high", get h() {
    return r().text_high[0];
  }, set h(o) {
    r(r().text_high[0] = o, true);
  }, get s() {
    return r().text_high[1];
  }, set s(o) {
    r(r().text_high[1] = o, true);
  }, get l() {
    return r().text_high[2];
  }, set l(o) {
    r(r().text_high[2] = o, true);
  } });
  var k = a(h, 2);
  me(k, { label: "--bg", get h() {
    return r().bg[0];
  }, set h(o) {
    r(r().bg[0] = o, true);
  }, get s() {
    return r().bg[1];
  }, set s(o) {
    r(r().bg[1] = o, true);
  }, get l() {
    return r().bg[2];
  }, set l(o) {
    r(r().bg[2] = o, true);
  } });
  var R = a(k, 2);
  me(R, { label: "--bg-high", get h() {
    return r().bg_high[0];
  }, set h(o) {
    r(r().bg_high[0] = o, true);
  }, get s() {
    return r().bg_high[1];
  }, set s(o) {
    r(r().bg_high[1] = o, true);
  }, get l() {
    return r().bg_high[2];
  }, set l(o) {
    r(r().bg_high[2] = o, true);
  } }), d(b);
  var x = a(b, 2), w = c(x);
  me(w, { label: "--action", get h() {
    return r().action[0];
  }, set h(o) {
    r(r().action[0] = o, true);
  }, get s() {
    return r().action[1];
  }, set s(o) {
    r(r().action[1] = o, true);
  }, get l() {
    return r().action[2];
  }, set l(o) {
    r(r().action[2] = o, true);
  } });
  var y = a(w, 2);
  me(y, { label: "--accent", get h() {
    return r().accent[0];
  }, set h(o) {
    r(r().accent[0] = o, true);
  }, get s() {
    return r().accent[1];
  }, set s(o) {
    r(r().accent[1] = o, true);
  }, get l() {
    return r().accent[2];
  }, set l(o) {
    r(r().accent[2] = o, true);
  } });
  var S = a(y, 2);
  me(S, { label: "--error", get h() {
    return r().error[0];
  }, set h(o) {
    r(r().error[0] = o, true);
  }, get s() {
    return r().error[1];
  }, set s(o) {
    r(r().error[1] = o, true);
  }, get l() {
    return r().error[2];
  }, set l(o) {
    r(r().error[2] = o, true);
  } }), d(x), d(_);
  var B = a(_, 2), N = c(B), F = c(N);
  we(F, () => l.clients.branding.descFullCss), d(N);
  var E = a(N, 2);
  De(E, { label: "--btn-text", get inputWidth() {
    return e.inputWidth;
  }, get value() {
    return r().btn_text;
  }, set value(o) {
    r(r().btn_text = o, true);
  } });
  var C = a(E, 2);
  De(C, { label: "--theme-sun", get inputWidth() {
    return e.inputWidth;
  }, get value() {
    return r().theme_sun;
  }, set value(o) {
    r(r().theme_sun = o, true);
  } });
  var P = a(C, 2);
  De(P, { label: "--theme-moon", get inputWidth() {
    return e.inputWidth;
  }, get value() {
    return r().theme_moon;
  }, set value(o) {
    r(r().theme_moon = o, true);
  } }), d(B), d(v), j(() => I(g, l.clients.branding.descHsl)), u(U, v), ae();
}
var Fr = (U) => U.preventDefault(), zr = D('<div aria-label="Preview: All components inside are only for theme and colors preview and have no effect or interaction"><div class="inner svelte-fkmn75"><div class="container svelte-fkmn75"><div class="logo svelte-fkmn75"><img alt="Client Logo"></div> <h3 class="svelte-fkmn75">Header</h3> <p>Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam</p> <p><span class="svelte-fkmn75">--accent-color</span></p> <!> <div class="btn svelte-fkmn75"><div class="svelte-fkmn75"><!> <br> <!> <br> <!> <br></div> <div class="svelte-fkmn75"><!> <br> <!> <br> <!></div></div> <a>Link</a> <br> <!></div></div></div>');
function It(U, e) {
  re(e, true);
  let r = L(false);
  _e(() => {
    t(r) && setTimeout(() => {
      s(r, false);
    }, 2e3);
  });
  var l = zr(), v = c(l), f = c(v), g = c(f), _ = c(g);
  d(g);
  var b = a(g, 8);
  fe(b, { label: "Preview", placeholder: "Preview", width: "12.5rem" });
  var m = a(b, 2), h = c(m), k = c(h);
  se(k, { level: 1, get isLoading() {
    return t(r);
  }, onclick: () => s(r, true), children: (E, C) => {
    z();
    var P = J("Button");
    u(E, P);
  }, $$slots: { default: true } });
  var R = a(k, 4);
  se(R, { level: 2, get isLoading() {
    return t(r);
  }, onclick: () => s(r, true), children: (E, C) => {
    z();
    var P = J("Button");
    u(E, P);
  }, $$slots: { default: true } });
  var x = a(R, 4);
  se(x, { level: 3, get isLoading() {
    return t(r);
  }, onclick: () => s(r, true), children: (E, C) => {
    z();
    var P = J("Button");
    u(E, P);
  }, $$slots: { default: true } }), z(2), d(h);
  var w = a(h, 2), y = c(w);
  se(y, { level: -1, get isLoading() {
    return t(r);
  }, onclick: () => s(r, true), children: (E, C) => {
    z();
    var P = J("Button");
    u(E, P);
  }, $$slots: { default: true } });
  var S = a(y, 4);
  se(S, { level: -2, get isLoading() {
    return t(r);
  }, onclick: () => s(r, true), children: (E, C) => {
    z();
    var P = J("Button");
    u(E, P);
  }, $$slots: { default: true } });
  var B = a(S, 4);
  se(B, { level: -3, get isLoading() {
    return t(r);
  }, onclick: () => s(r, true), children: (E, C) => {
    z();
    var P = J("Button");
    u(E, P);
  }, $$slots: { default: true } }), d(w), d(m);
  var N = a(m, 2);
  W(N, "href", window.location.href), N.__click = [Fr];
  var F = a(N, 4);
  ur(F, {}), d(f), d(v), d(l), j((E, C, P, o, p, Q, A) => {
    H(l, "--border-radius", e.borderRadius), H(l, "--text", E), H(l, "--text-high", C), H(l, "--bg", P), H(l, "--bg-high", o), H(l, "--action", p), H(l, "--accent", Q), H(l, "--error", A), H(l, "--btn-text", e.theme.btn_text), H(l, "--theme-sun", e.theme.theme_sun), H(l, "--theme-moon", e.theme.theme_moon), W(_, "src", e.logoUrl);
  }, [() => e.theme.text.join(" "), () => e.theme.text_high.join(" "), () => e.theme.bg.join(" "), () => e.theme.bg_high.join(" "), () => e.theme.action.join(" "), () => e.theme.accent.join(" "), () => e.theme.error.join(" ")]), u(U, l), ae();
}
Fe(["click"]);
var Hr = D('<div><h2>Preview</h2> <!> <div class="preview svelte-1nlq99j"><!></div></div>');
function Kr(U, e) {
  re(e, true);
  let r = ["Light Theme", "Dark Theme"], l = L(i(r[0]));
  var v = Hr(), f = a(c(v), 2);
  Ut(f, { tabs: r, center: true, get selected() {
    return t(l);
  }, set selected(h) {
    s(l, i(h));
  } });
  var g = a(f, 2), _ = c(g);
  {
    var b = (h) => {
      It(h, { get logoUrl() {
        return e.logoUrl;
      }, get borderRadius() {
        return e.theme.border_radius;
      }, get theme() {
        return e.theme.light;
      } });
    }, m = (h) => {
      It(h, { get logoUrl() {
        return e.logoUrl;
      }, get borderRadius() {
        return e.theme.border_radius;
      }, get theme() {
        return e.theme.dark;
      } });
    };
    ee(_, (h) => {
      t(l) === r[0] ? h(b) : h(m, false);
    });
  }
  d(g), d(v), u(U, v), ae();
}
var Vr = D('<div class="err"> </div>'), Gr = D('<div class="values svelte-1pkqupw"><p><!></p> <!> <h1>Light Theme</h1> <!> <br> <h1>Dark Theme</h1> <!></div> <div class="preview"><!> <hr> <p>Logo Upload</p> <!> <div class="buttons svelte-1pkqupw"><!> <!> <!></div> <!></div>', 1), Jr = D('<div class="container svelte-1pkqupw"><!></div>');
function Qr(U, e) {
  re(e, true);
  const r = "13rem";
  let l = xe(), v = ge(), f = L(false), g = L(""), _ = L(void 0), b = L(i(ke())), m = he(() => `/auth/v1/clients/${e.client.id}/logo?${t(b)}`), h = he(() => `/auth/v1/theme/${e.client.id}`);
  _e(() => {
    k();
  });
  async function k() {
    var _a;
    let y = await ze(t(h));
    y.body ? (y.body.client_id === "rauthy" && console.log("using Rauthy default scheme"), s(_, i(y.body))) : s(g, i(((_a = y.error) == null ? void 0 : _a.message) || "Error"));
  }
  async function R(y, S) {
    if (!t(_)) {
      console.error("theme is undefined");
      return;
    }
    let B = t(_);
    B.client_id = e.client.id;
    let N = await He(t(h), B);
    N.error ? s(g, i(N.error.message)) : (s(f, true), setTimeout(() => {
      s(f, false);
    }, 2e3));
  }
  async function x() {
    let y = await Rt(t(h));
    y.error ? s(g, i(y.error.message)) : (await k(), s(f, true), setTimeout(() => {
      s(f, false);
    }, 2e3));
  }
  async function w() {
    s(b, i(ke()));
  }
  Ke(U, { get action() {
    return t(h);
  }, onSubmit: R, children: (y, S) => {
    var B = Jr(), N = c(B);
    {
      var F = (E) => {
        var C = Gr(), P = te(C), o = c(P), p = c(o);
        we(p, () => v.clients.branding.descVariables), d(o);
        var Q = a(o, 2);
        fe(Q, { label: "--border-radius", placeholder: "--border-radius", get errMsg() {
          return v.validation.css;
        }, width: r, required: true, pattern: Et, get value() {
          return t(_).border_radius;
        }, set value(O) {
          t(_).border_radius = O;
        } });
        var A = a(Q, 4);
        Tt(A, { inputWidth: r, get values() {
          return t(_).light;
        }, set values(O) {
          t(_).light = O;
        } });
        var M = a(A, 6);
        Tt(M, { inputWidth: r, get values() {
          return t(_).dark;
        }, set values(O) {
          t(_).dark = O;
        } }), d(P);
        var K = a(P, 2), V = c(K);
        vr(V, () => t(b), (O) => {
          Kr(O, { get logoUrl() {
            return t(m);
          }, get theme() {
            return t(_);
          } });
        });
        var G = a(V, 6);
        const le = he(() => `/auth/v1/clients/${e.client.id}/logo`);
        fr(G, { method: "PUT", get url() {
          return t(le);
        }, fileName: "logo", onSuccess: w });
        var Z = a(G, 2), q = c(Z);
        se(q, { type: "submit", children: (O, ne) => {
          z();
          var ce = J();
          j(() => I(ce, l.common.save)), u(O, ce);
        }, $$slots: { default: true } });
        var X = a(q, 2);
        se(X, { level: -2, onclick: x, children: (O, ne) => {
          z();
          var ce = J();
          j(() => I(ce, v.common.reset)), u(O, ce);
        }, $$slots: { default: true } });
        var ue = a(X, 2);
        {
          var oe = (O) => {
            Nt(O, {});
          };
          ee(ue, (O) => {
            t(f) && O(oe);
          });
        }
        d(Z);
        var de = a(Z, 2);
        {
          var ie = (O) => {
            var ne = Vr(), ce = c(ne, true);
            d(ne), j(() => I(ce, t(g))), u(O, ne);
          };
          ee(de, (O) => {
            t(g) && O(ie);
          });
        }
        d(K), u(E, C);
      };
      ee(N, (E) => {
        t(_) && E(F);
      });
    }
    d(B), u(y, B);
  }, $$slots: { default: true } }), ae();
}
var Xr = D('<div class="flex"><!></div> <div class="details"><!></div>', 1);
function Yr(U, e) {
  re(e, true);
  let r = xe(), l = ge();
  const v = [l.nav.config, "Secret", "Branding", r.common.delete];
  let f = L(i(v[0])), g = L(void 0);
  _e(() => {
    e.client.id && requestAnimationFrame(() => {
      var _a;
      (_a = t(g)) == null ? void 0 : _a();
    });
  });
  var _ = Xr(), b = te(_), m = c(b);
  Ut(m, { tabs: v, get selected() {
    return t(f);
  }, set selected(w) {
    s(f, i(w));
  }, get focusFirst() {
    return t(g);
  }, set focusFirst(w) {
    s(g, i(w));
  } }), d(b);
  var h = a(b, 2), k = c(h);
  {
    var R = (w) => {
      Rr(w, { get client() {
        return e.client;
      }, get clients() {
        return e.clients;
      }, get scopesAll() {
        return e.scopesAll;
      }, get onSave() {
        return e.onSave;
      } });
    }, x = (w) => {
      var y = Pe(), S = te(y);
      {
        var B = (F) => {
          Mr(F, { get client() {
            return e.client;
          } });
        }, N = (F) => {
          var E = Pe(), C = te(E);
          {
            var P = (p) => {
              Qr(p, { get client() {
                return e.client;
              } });
            }, o = (p) => {
              var Q = Pe(), A = te(Q);
              {
                var M = (K) => {
                  qr(K, { get client() {
                    return e.client;
                  }, get onSave() {
                    return e.onSave;
                  } });
                };
                ee(A, (K) => {
                  t(f) === r.common.delete && K(M);
                }, true);
              }
              u(p, Q);
            };
            ee(C, (p) => {
              t(f) === "Branding" ? p(P) : p(o, false);
            }, true);
          }
          u(F, E);
        };
        ee(S, (F) => {
          t(f) === "Secret" ? F(B) : F(N, false);
        }, true);
      }
      u(w, y);
    };
    ee(k, (w) => {
      t(f) === l.nav.config ? w(R) : w(x, false);
    });
  }
  d(h), u(U, _), ae();
}
var Zr = D("<div></div> <!>", 1), $r = D("<!> <!>", 1), ea = D('<div class="err"> </div>'), ta = D('<!> <div id="groups"><!></div>', 1), ra = D("<!> <!>", 1);
function aa(U, e) {
  re(e, true);
  let r = L(void 0), l = L(""), v = L(i([])), f = L(i([])), g = L(void 0), _ = ir("cid"), b = L(i([]));
  const m = ["ID"];
  let h = L(i(m[0])), k = L("");
  const R = ["ID"];
  Yt(() => {
    x(), w();
  }), _e(() => {
    s(g, i(t(v).find((C) => C.id === _.get())));
  }), _e(() => {
    let C = t(k).toLowerCase();
    C ? t(h) === m[0] && s(f, i(t(v).filter((P) => P.id.toLowerCase().includes(C)))) : s(f, i(t(v)));
  });
  async function x() {
    var _a;
    let C = await Lt("/auth/v1/clients");
    C.body ? s(v, i(C.body)) : s(l, i(((_a = C.error) == null ? void 0 : _a.message) || "Error"));
  }
  async function w() {
    var _a;
    let C = await Lt("/auth/v1/scopes");
    C.body ? s(b, i(C.body.map((P) => P.name))) : s(l, i(((_a = C.error) == null ? void 0 : _a.message) || "Error"));
  }
  function y(C, P) {
    let o = P === "up";
    C === R[0] && t(v).sort((p, Q) => o ? p.id.localeCompare(Q.id) : Q.id.localeCompare(p.id));
  }
  function S() {
    x();
  }
  async function B(C) {
    var _a;
    (_a = t(r)) == null ? void 0 : _a(), await x(), _.set(C);
  }
  var N = ra(), F = te(N);
  ar(F, { paddingTop: "2.1rem", buttonTilesAriaControls: "groups", width: "min(20rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (P) => {
    var o = Zr(), p = te(o);
    H(p, "height", ".5rem");
    var Q = a(p, 2);
    We(Q, 17, () => t(f), (A) => A.id, (A, M, K, V) => {
      const G = he(() => _.get() === t(M).id);
      rr(A, { onclick: () => _.set(t(M).id), get selected() {
        return t(G);
      }, children: (le, Z) => {
        z();
        var q = J();
        j(() => I(q, t(M).id)), u(le, q);
      } });
    }), u(P, o);
  }, children: (P, o) => {
    var p = $r(), Q = te(p);
    const A = he(() => t(v).length === 0 ? 1 : 2);
    er(Q, { get level() {
      return t(A);
    }, alignRight: true, get closeModal() {
      return t(r);
    }, set closeModal(K) {
      s(r, i(K));
    }, children: (K, V) => {
      Lr(K, { onSave: B, get clients() {
        return t(v);
      } });
    }, $$slots: { default: true } });
    var M = a(Q, 2);
    lr(M, { searchOptions: m, orderOptions: R, onChangeOrder: y, searchWidth: "min(19.5rem, 100dvw - .5rem)", get searchOption() {
      return t(h);
    }, set searchOption(K) {
      s(h, i(K));
    }, get value() {
      return t(k);
    }, set value(K) {
      s(k, i(K));
    } }), u(P, p);
  }, $$slots: { buttonTiles: true, default: true } });
  var E = a(F, 2);
  tr(E, { children: (C, P) => {
    var o = ta(), p = te(o);
    {
      var Q = (V) => {
        var G = ea(), le = c(G, true);
        d(G), j(() => I(le, t(l))), u(V, G);
      };
      ee(p, (V) => {
        t(l) && V(Q);
      });
    }
    var A = a(p, 2), M = c(A);
    {
      var K = (V) => {
        Yr(V, { get client() {
          return t(g);
        }, get clients() {
          return t(v);
        }, get scopesAll() {
          return t(b);
        }, onSave: S });
      };
      ee(M, (V) => {
        t(g) && V(K);
      });
    }
    d(A), u(C, o);
  } }), u(U, N), ae();
}
function Da(U) {
  aa(U, {});
}
export {
  Da as component
};
