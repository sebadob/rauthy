import { t as q, a as u, d as Pe, e as Q } from "../chunks/BH6NCLk-.js";
import { p as ae, c as v, s as a, k as x, t as O, l as n, j as t, a as le, r as c, a9 as _e, f as re, aa as he, a7 as J, a5 as Yt } from "../chunks/CvlvO1XB.js";
import { d as Ke, e as Fe, s as I, b as Zt } from "../chunks/CTI4QPiR.js";
import { i as ee } from "../chunks/BUO_AUgz.js";
import { e as ze, i as Lt } from "../chunks/BpWRzPRQ.js";
import { a as W, B as oe, t as $t, s as er } from "../chunks/CqH8LXO-.js";
import { p as i } from "../chunks/Wh68IIk2.js";
import { B as tr } from "../chunks/CKd5IV8T.js";
import { C as rr } from "../chunks/BnPoFdx3.js";
import { N as ar } from "../chunks/xdJ7HxlL.js";
import { N as lr } from "../chunks/TVaajnXc.js";
import { O as ir } from "../chunks/V6L-Tw6V.js";
import { b as Ve, d as We, c as He, f as St } from "../chunks/BO1A6s0c.js";
import { u as nr } from "../chunks/EvbDpXwj.js";
import { h as we } from "../chunks/i8Xqpu09.js";
import { I as fe } from "../chunks/KD8vSltG.js";
import { n as sr, g as pt, a as Be, o as or, p as dr, q as Et } from "../chunks/BRCxk8by.js";
import { F as Ge } from "../chunks/Df37IfRm.js";
import { u as xe } from "../chunks/CUqQZdNU.js";
import { u as me } from "../chunks/D8mHI_K9.js";
import { I as ue } from "../chunks/BXwFLokg.js";
import { r as Je, s as F } from "../chunks/BMbqVy6X.js";
import { b as Qe } from "../chunks/dU6E9WaN.js";
import { b as Mt } from "../chunks/zosqiMUL.js";
import { p as Z } from "../chunks/C6SR4G2t.js";
import { g as be } from "../chunks/B21bTIl7.js";
import { I as cr } from "../chunks/Vi3uK7uO.js";
import { T as Ut } from "../chunks/BunZbqN2.js";
import { I as Nt } from "../chunks/Nks81rMs.js";
import { L as De } from "../chunks/nnFaiMsH.js";
import { S as At } from "../chunks/Caxiwo-d.js";
import { O as Tt } from "../chunks/CdqYxeyI.js";
import { I as vr } from "../chunks/BMwvmSAM.js";
import { k as ur } from "../chunks/CaD2yKt4.js";
import { T as fr } from "../chunks/CXfCraUq.js";
import { R as _r } from "../chunks/CufZd29Z.js";
import { I as hr } from "../chunks/DaHSOo5I.js";
function mr(M, e, r, o) {
  var _a;
  e(), (_a = r.onInput) == null ? void 0 : _a.call(r, t(o));
}
function gr(M, e, r) {
  var _a;
  switch (M.code) {
    case "Enter":
      e(), (_a = r.onEnter) == null ? void 0 : _a.call(r);
      break;
  }
}
function br(M, e) {
  var _a, _b;
  (_a = e()) == null ? void 0 : _a.focus(), (_b = e()) == null ? void 0 : _b.scrollIntoView({ behavior: "smooth", block: "center" });
}
var kr = q('<li class="value svelte-r5y8pk"><div class="label"> </div> <div class="btnClose svelte-r5y8pk"><!></div></li>'), xr = q("<option></option>"), wr = q('<datalist class="absolute svelte-r5y8pk"></datalist>'), yr = q('<span class="err"> </span>'), Cr = q('<div role="none" class="container svelte-r5y8pk"><ul class="svelte-r5y8pk"><!> <li class="svelte-r5y8pk"><input autocomplete="off" class="svelte-r5y8pk"> <!></li></ul> <label class="font-label noselect svelte-r5y8pk"> <!></label></div>');
function ke(M, e) {
  ae(e, true);
  let r = Z(e, "ref", 15), o = Z(e, "typ", 3, "text"), d = Z(e, "id", 19, be), f = Z(e, "values", 31, () => i([])), w = Z(e, "label", 3, ""), _ = Z(e, "placeholder", 3, ""), b = Z(e, "disabled", 3, false), y = Z(e, "required", 3, false), k = Z(e, "isError", 15, false), h = Z(e, "width", 3, "inherit"), R = Z(e, "maxHeightList", 3, "inherit"), g = xe();
  const C = be();
  let P = x(""), L = _e(() => e.datalist && e.datalist.length > 0 ? C : void 0);
  function B(U) {
    f(f().filter((V) => V !== U));
  }
  function N(U) {
    var _a;
    A(), (_a = e.onBlur) == null ? void 0 : _a.call(e), X();
  }
  function z(U) {
    U.preventDefault(), k(true);
  }
  function X() {
    t(P) && A() && (f().push(t(P)), n(P, ""));
  }
  function A() {
    var _a;
    let U = (_a = r()) == null ? void 0 : _a.validity;
    return U ? (k(!U.valid), U.valid) : (k(false), true);
  }
  var D = Cr();
  D.__click = [br, r];
  var l = v(D), m = v(l);
  ze(m, 17, f, Lt, (U, V, de, te) => {
    var ce = kr(), ne = v(ce), j = v(ne, true);
    c(ne);
    var se = a(ne, 2), ve = v(se);
    oe(ve, { invisible: true, onclick: () => B(t(V)), children: (ye, Le) => {
      cr(ye, { width: "1.2rem" });
    }, $$slots: { default: true } }), c(se), c(ce), O(() => I(j, t(V))), u(U, ce);
  });
  var E = a(m, 2), S = v(E);
  Je(S), S.__keydown = [gr, X, e], S.__input = [mr, A, e, P], Mt(S, (U) => r(U), () => r());
  var p = a(S, 2);
  {
    var H = (U) => {
      var V = wr();
      F(V, "id", C), ze(V, 21, () => e.datalist, Lt, (de, te, ce, ne) => {
        var j = xr(), se = {};
        O(() => {
          se !== (se = t(te)) && (j.value = (j.__value = t(te)) == null ? "" : t(te));
        }), u(de, j);
      }), c(V), u(U, V);
    };
    ee(p, (U) => {
      e.datalist && e.datalist.length > 1 && U(H);
    });
  }
  c(E), c(l);
  var K = a(l, 2), G = v(K), ie = a(G);
  {
    var $ = (U) => {
      var V = Pe(), de = re(V);
      {
        var te = (ce) => {
          var ne = yr(), j = v(ne, true);
          c(ne), O(() => I(j, e.errMsg || g.common.invalidInput)), u(ce, ne);
        };
        ee(de, (ce) => {
          k() && ce(te);
        });
      }
      u(U, V);
    };
    ee(ie, (U) => {
      k() && U($);
    });
  }
  c(K), c(D), O(() => {
    W(D, "width", h()), W(l, "max-height", R()), F(S, "type", o()), F(S, "id", d()), F(S, "name", e.name), F(S, "list", t(L)), F(S, "title", e.errMsg), F(S, "aria-label", w() || _()), F(S, "placeholder", _()), F(S, "aria-placeholder", _()), S.disabled = b(), F(S, "aria-disabled", b()), S.required = y() && f().length < 1, F(S, "aria-required", y() && f().length < 1), F(S, "maxlength", e.maxLength || void 0), F(S, "min", e.min || void 0), F(S, "max", e.max || void 0), F(S, "pattern", e.pattern), F(K, "for", d()), F(K, "data-required", y()), I(G, `${w() ?? ""} `);
  }), Fe("invalid", S, z), Fe("blur", S, N), Qe(S, () => t(P), (U) => n(P, U)), u(M, D), le();
}
Ke(["click", "keydown", "input"]);
var Pr = q('<div class="err"> </div>'), Lr = q('<!> <!> <!> <p class="svelte-15tp6i6"><!></p> <!> <!> <!> <!>', 1), Sr = q('<div class="container svelte-15tp6i6"><!></div>');
function Ar(M, e) {
  ae(e, true);
  let r = xe(), o = me(), d = x(void 0), f = x(""), w = x(""), _ = x(""), b = x(true), y = x(i([])), k = x(i([]));
  he(() => {
    requestAnimationFrame(() => {
      var _a;
      (_a = t(d)) == null ? void 0 : _a.focus();
    });
  });
  async function h(C, P) {
    var _a;
    if (e.clients.find((N) => N.id === t(w))) {
      n(f, i(o.common.nameExistsAlready));
      return;
    }
    n(f, "");
    let L = { id: t(w), name: t(_) || void 0, confidential: t(b), redirect_uris: t(y), post_logout_redirect_uris: t(k).length > 0 ? t(k) : void 0 }, B = await Ve(C.action, L);
    B.body ? e.onSave(B.body.id) : n(f, i(((_a = B.error) == null ? void 0 : _a.message) || "Error"));
  }
  var R = Sr(), g = v(R);
  Ge(g, { action: "/auth/v1/clients", onSubmit: h, children: (C, P) => {
    var L = Lr(), B = re(L);
    fe(B, { autocomplete: "off", label: "Client ID", placeholder: "Client ID", required: true, pattern: sr, get ref() {
      return t(d);
    }, set ref(p) {
      n(d, i(p));
    }, get value() {
      return t(w);
    }, set value(p) {
      n(w, i(p));
    } });
    var N = a(B, 2);
    fe(N, { autocomplete: "off", get label() {
      return o.clients.name;
    }, get placeholder() {
      return o.clients.name;
    }, pattern: pt, get value() {
      return t(_);
    }, set value(p) {
      n(_, i(p));
    } });
    var z = a(N, 2);
    ue(z, { get ariaLabel() {
      return o.clients.confidential;
    }, get checked() {
      return t(b);
    }, set checked(p) {
      n(b, i(p));
    }, children: (p, H) => {
      J();
      var K = Q();
      O(() => I(K, o.clients.confidential)), u(p, K);
    }, $$slots: { default: true } });
    var X = a(z, 2), A = v(X);
    we(A, () => o.clients.descUri), c(X);
    var D = a(X, 2);
    ke(D, { typ: "url", label: "Redirect URIs", get errMsg() {
      return o.validation.uri;
    }, get values() {
      return t(y);
    }, set values(p) {
      n(y, i(p));
    } });
    var l = a(D, 2);
    ke(l, { typ: "url", label: "Post Logout Redirect URIs", get errMsg() {
      return o.validation.uri;
    }, get values() {
      return t(k);
    }, set values(p) {
      n(k, i(p));
    } });
    var m = a(l, 2);
    oe(m, { type: "submit", children: (p, H) => {
      J();
      var K = Q();
      O(() => I(K, r.common.save)), u(p, K);
    }, $$slots: { default: true } });
    var E = a(m, 2);
    {
      var S = (p) => {
        var H = Pr(), K = v(H, true);
        c(H), O(() => I(K, t(f))), u(p, H);
      };
      ee(E, (p) => {
        t(f) && p(S);
      });
    }
    u(C, L);
  }, $$slots: { default: true } }), c(R), u(M, R), le();
}
const Oe = "urn:ietf:params:oauth:grant-type:device_code";
var Tr = q('<div class="err"> </div>'), Ir = q('<div class="err"> </div>'), Rr = q('<h5> </h5> <!> <p class="desc svelte-5tvvvt"> </p> <!> <p class="desc svelte-5tvvvt"> </p> <!> <!> <div></div> <h5> </h5> <!> <!> <!> <p class="mb-0 svelte-5tvvvt">Authentication Flows</p> <!> <!> <!> <!> <!> <div></div> <p class="desc svelte-5tvvvt"> </p> <p class="desc svelte-5tvvvt"><strong> </strong></p> <!> <!> <!> <div></div> <p class="desc svelte-5tvvvt"> </p> <!> <p class="desc svelte-5tvvvt"><!></p> <!> <!> <div></div> <p class="desc svelte-5tvvvt"><!></p> <!> <!> <div></div> <p> </p> <!> <div></div> <p> </p> <p> </p> <!> <!> <div></div> <p> </p> <!> <div class="flex gap-05"><!> <!></div> <!> <div></div>', 1), pr = q('<div class="container svelte-5tvvvt"><!></div>');
function Er(M, e) {
  var _a, _b;
  ae(e, true);
  let r = xe(), o = me();
  const d = "min(20rem, calc(100dvw - .5rem))";
  let f = x(""), w = x(false), _ = x(i(e.client.id)), b = x(i(e.client.name || "")), y = x(i(e.client.enabled)), k = x(i(e.client.confidential)), h = x(i(e.client.client_uri || "")), R = x(i(e.client.contacts ? Array.from(e.client.contacts) : [])), g = x(i(e.client.allowed_origins ? Array.from(e.client.allowed_origins) : [])), C = x(i(Array.from(e.client.redirect_uris))), P = x(i(e.client.post_logout_redirect_uris ? Array.from(e.client.post_logout_redirect_uris) : [])), L = i({ authorizationCode: e.client.flows_enabled.includes("authorization_code"), clientCredentials: e.client.flows_enabled.includes("client_credentials"), password: e.client.flows_enabled.includes("password"), refreshToken: e.client.flows_enabled.includes("refresh_token"), deviceCode: e.client.flows_enabled.includes(Oe) });
  const B = ["RS256", "RS384", "RS512", "EdDSA"];
  let N = x(i(e.client.access_token_alg)), z = x(i(e.client.id_token_alg)), X = x(i(e.client.access_token_lifetime.toString())), A = x(i(e.client.auth_code_lifetime.toString())), D = x(i(e.scopesAll.map((G) => ({ name: G, selected: e.client.scopes.includes(G) || false })))), l = x(i(e.scopesAll.map((G) => ({ name: G, selected: e.client.default_scopes.includes(G) || false })))), m = i({ plain: ((_a = e.client.challenges) == null ? void 0 : _a.includes("plain")) || false, s256: ((_b = e.client.challenges) == null ? void 0 : _b.includes("S256")) || false }), E = x(i(e.client.force_mfa));
  he(() => {
    var _a2, _b2;
    e.client.id && (n(_, i(e.client.id)), n(b, i(e.client.name || "")), n(y, i(e.client.enabled)), n(k, i(e.client.confidential)), n(h, i(e.client.client_uri || "")), n(R, i(e.client.contacts ? Array.from(e.client.contacts) : [])), n(g, i(e.client.allowed_origins ? Array.from(e.client.allowed_origins) : [])), n(C, i(Array.from(e.client.redirect_uris))), n(P, i(e.client.post_logout_redirect_uris ? Array.from(e.client.post_logout_redirect_uris) : [])), L.authorizationCode = e.client.flows_enabled.includes("authorization_code"), L.clientCredentials = e.client.flows_enabled.includes("client_credentials"), L.password = e.client.flows_enabled.includes("password"), L.refreshToken = e.client.flows_enabled.includes("refresh_token"), L.deviceCode = e.client.flows_enabled.includes(Oe), n(N, i(e.client.access_token_alg)), n(z, i(e.client.id_token_alg)), n(X, i(e.client.access_token_lifetime.toString())), n(A, i(e.client.auth_code_lifetime.toString())), n(D, i(e.scopesAll.map((G) => ({ name: G, selected: e.client.scopes.includes(G) || false })))), n(l, i(e.scopesAll.map((G) => ({ name: G, selected: e.client.default_scopes.includes(G) || false })))), m.plain = ((_a2 = e.client.challenges) == null ? void 0 : _a2.includes("plain")) || false, m.s256 = ((_b2 = e.client.challenges) == null ? void 0 : _b2.includes("S256")) || false);
  });
  async function S(G, ie) {
    n(f, "");
    let $ = { id: t(_), name: t(b) || void 0, enabled: t(y), confidential: t(k), allowed_origins: t(g).length > 0 ? t(g) : void 0, redirect_uris: t(C), post_logout_redirect_uris: t(P).length > 0 ? t(P) : void 0, flows_enabled: [], access_token_alg: t(N), id_token_alg: t(z), access_token_lifetime: Number.parseInt(t(X)), auth_code_lifetime: Number.parseInt(t(A)), scopes: t(D).filter((V) => V.selected).map((V) => V.name), default_scopes: t(l).filter((V) => V.selected).map((V) => V.name), challenges: void 0, force_mfa: t(E), client_uri: t(h) || void 0, contacts: t(R).length > 0 ? t(R) : void 0 };
    L.authorizationCode && $.flows_enabled.push("authorization_code"), L.clientCredentials && $.flows_enabled.push("client_credentials"), L.password && $.flows_enabled.push("password"), L.refreshToken && $.flows_enabled.push("refresh_token"), L.deviceCode && $.flows_enabled.push(Oe), m.plain && ($.challenges = ["plain"]), m.s256 && ($.challenges ? $.challenges.push("S256") : $.challenges = ["S256"]);
    let U = await We(G.action, $);
    U.error ? n(f, i(U.error.message)) : (n(w, true), e.onSave(), setTimeout(() => {
      n(w, false);
    }, 2e3));
  }
  var p = pr(), H = v(p);
  const K = _e(() => `/auth/v1/clients/${e.client.id}`);
  Ge(H, { get action() {
    return t(K);
  }, onSubmit: S, children: (G, ie) => {
    var $ = Rr(), U = re($), V = v(U, true);
    c(U);
    var de = a(U, 2);
    De(de, { label: "ID", mono: true, children: (s, Y) => {
      J();
      var T = Q();
      O(() => I(T, e.client.id)), u(s, T);
    } });
    var te = a(de, 2), ce = v(te, true);
    c(te);
    var ne = a(te, 2);
    fe(ne, { autocomplete: "off", get label() {
      return o.clients.name;
    }, get placeholder() {
      return o.clients.name;
    }, width: d, pattern: pt, get value() {
      return t(b);
    }, set value(s) {
      n(b, i(s));
    } });
    var j = a(ne, 2), se = v(j, true);
    c(j);
    var ve = a(j, 2);
    fe(ve, { typ: "url", autocomplete: "off", label: "URI", placeholder: "URI", width: d, pattern: Be, get value() {
      return t(h);
    }, set value(s) {
      n(h, i(s));
    } });
    var ye = a(ve, 2);
    ke(ye, { get label() {
      return o.common.contact;
    }, pattern: or, get values() {
      return t(R);
    }, set values(s) {
      n(R, i(s));
    } });
    var Le = a(ye, 2);
    W(Le, "height", ".5rem");
    var Se = a(Le, 2), Bt = v(Se, true);
    c(Se);
    var Xe = a(Se, 2);
    ue(Xe, { get ariaLabel() {
      return o.common.enabled;
    }, get checked() {
      return t(y);
    }, set checked(s) {
      n(y, i(s));
    }, children: (s, Y) => {
      J();
      var T = Q();
      O(() => I(T, o.common.enabled)), u(s, T);
    }, $$slots: { default: true } });
    var Ye = a(Xe, 2);
    ue(Ye, { get ariaLabel() {
      return o.clients.confidential;
    }, get checked() {
      return t(k);
    }, set checked(s) {
      n(k, i(s));
    }, children: (s, Y) => {
      J();
      var T = Q();
      O(() => I(T, o.clients.confidential)), u(s, T);
    }, $$slots: { default: true } });
    var Ze = a(Ye, 2);
    ue(Ze, { get ariaLabel() {
      return o.clients.forceMfa;
    }, get checked() {
      return t(E);
    }, set checked(s) {
      n(E, i(s));
    }, children: (s, Y) => {
      J();
      var T = Q();
      O(() => I(T, o.clients.forceMfa)), u(s, T);
    }, $$slots: { default: true } });
    var $e = a(Ze, 4);
    ue($e, { ariaLabel: "authorization_code", get checked() {
      return L.authorizationCode;
    }, set checked(s) {
      L.authorizationCode = s;
    }, children: (s, Y) => {
      J();
      var T = Q("authorization_code");
      u(s, T);
    }, $$slots: { default: true } });
    var et = a($e, 2);
    ue(et, { ariaLabel: "urn:ietf:params:oauth:grant-type:device_code", get checked() {
      return L.deviceCode;
    }, set checked(s) {
      L.deviceCode = s;
    }, children: (s, Y) => {
      J();
      var T = Q("device_code");
      u(s, T);
    }, $$slots: { default: true } });
    var tt = a(et, 2);
    ue(tt, { ariaLabel: "client_credentials", get checked() {
      return L.clientCredentials;
    }, set checked(s) {
      L.clientCredentials = s;
    }, children: (s, Y) => {
      J();
      var T = Q("client_credentials");
      u(s, T);
    }, $$slots: { default: true } });
    var rt = a(tt, 2);
    ue(rt, { ariaLabel: "password", get checked() {
      return L.password;
    }, set checked(s) {
      L.password = s;
    }, children: (s, Y) => {
      J();
      var T = Q("password");
      u(s, T);
    }, $$slots: { default: true } });
    var at = a(rt, 2);
    ue(at, { ariaLabel: "refresh_token", get checked() {
      return L.refreshToken;
    }, set checked(s) {
      L.refreshToken = s;
    }, children: (s, Y) => {
      J();
      var T = Q("refresh_token");
      u(s, T);
    }, $$slots: { default: true } });
    var lt = a(at, 2);
    W(lt, "height", ".5rem");
    var Ae = a(lt, 2), Dt = v(Ae, true);
    c(Ae);
    var Te = a(Ae, 2), it = v(Te), Ot = v(it, true);
    c(it), c(Te);
    var nt = a(Te, 2);
    ue(nt, { ariaLabel: "PKCE plain", get checked() {
      return m.plain;
    }, set checked(s) {
      m.plain = s;
    }, children: (s, Y) => {
      J();
      var T = Q("plain");
      u(s, T);
    }, $$slots: { default: true } });
    var st = a(nt, 2);
    ue(st, { ariaLabel: "PKCE S256", get checked() {
      return m.s256;
    }, set checked(s) {
      m.s256 = s;
    }, children: (s, Y) => {
      J();
      var T = Q("S256");
      u(s, T);
    }, $$slots: { default: true } });
    var ot = a(st, 2);
    {
      var qt = (s) => {
        var Y = Tr(), T = v(Y, true);
        c(Y), O(() => I(T, o.clients.errConfidentialPKCE)), $t(3, Y, () => er, () => ({ duration: 150 })), u(s, Y);
      };
      ee(ot, (s) => {
        !t(k) && !m.plain && !m.s256 && s(qt);
      });
    }
    var dt = a(ot, 2);
    W(dt, "height", ".5rem");
    var Ie = a(dt, 2), jt = v(Ie, true);
    c(Ie);
    var ct = a(Ie, 2);
    ke(ct, { typ: "url", label: "Allowed Origins", get errMsg() {
      return o.validation.origin;
    }, pattern: dr, get values() {
      return t(g);
    }, set values(s) {
      n(g, i(s));
    } });
    var Re = a(ct, 2), Ft = v(Re);
    we(Ft, () => o.clients.descUri), c(Re);
    var vt = a(Re, 2);
    ke(vt, { typ: "url", label: "Redirect URIs", get errMsg() {
      return o.validation.uri;
    }, get required() {
      return L.authorizationCode;
    }, pattern: Be, get values() {
      return t(C);
    }, set values(s) {
      n(C, i(s));
    } });
    var ut = a(vt, 2);
    ke(ut, { typ: "url", label: "Post Logout Redirect URIs", get errMsg() {
      return o.validation.uri;
    }, pattern: Be, get values() {
      return t(P);
    }, set values(s) {
      n(P, i(s));
    } });
    var ft = a(ut, 2);
    W(ft, "height", ".5rem");
    var pe = a(ft, 2), zt = v(pe);
    we(zt, () => o.clients.scopes.desc), c(pe);
    var _t = a(pe, 2);
    At(_t, { get items() {
      return t(D);
    }, set items(s) {
      n(D, i(s));
    }, children: (s, Y) => {
      J();
      var T = Q();
      O(() => I(T, o.clients.scopes.allowed)), u(s, T);
    }, $$slots: { default: true } });
    var ht = a(_t, 2);
    At(ht, { get items() {
      return t(l);
    }, set items(s) {
      n(l, i(s));
    }, children: (s, Y) => {
      J();
      var T = Q();
      O(() => I(T, o.clients.scopes.default)), u(s, T);
    }, $$slots: { default: true } });
    var mt = a(ht, 2);
    W(mt, "height", ".75rem");
    var Ee = a(mt, 2), Ht = v(Ee, true);
    c(Ee);
    var gt = a(Ee, 2);
    fe(gt, { typ: "number", autocomplete: "off", label: "Token Lifetime", placeholder: "Token Lifetime", width: d, min: "10", max: "86400", errMsg: "10 <= Token Lifetime <= 86400", get value() {
      return t(X);
    }, set value(s) {
      n(X, i(s));
    } });
    var bt = a(gt, 2);
    W(bt, "height", ".5rem");
    var Me = a(bt, 2), Kt = v(Me, true);
    c(Me);
    var Ue = a(Me, 2), Vt = v(Ue, true);
    c(Ue);
    var kt = a(Ue, 2);
    De(kt, { label: "Access Token Algorithm", children: (s, Y) => {
      Tt(s, { ariaLabel: "Access Token Algorithm", options: B, borderless: true, get value() {
        return t(N);
      }, set value(T) {
        n(N, i(T));
      } });
    } });
    var xt = a(kt, 2);
    De(xt, { label: "ID Token Algorithm", children: (s, Y) => {
      Tt(s, { ariaLabel: "ID Token Algorithm", options: B, borderless: true, get value() {
        return t(z);
      }, set value(T) {
        n(z, i(T));
      } });
    } });
    var wt = a(xt, 2);
    W(wt, "height", ".5rem");
    var Ne = a(wt, 2), Wt = v(Ne, true);
    c(Ne);
    var yt = a(Ne, 2);
    fe(yt, { typ: "number", autocomplete: "off", label: "Auth Code Lifetime", placeholder: "Auth Code Lifetime", width: d, min: "10", max: "300", errMsg: "10 <= Auth Code Lifetime <= 300", get value() {
      return t(A);
    }, set value(s) {
      n(A, i(s));
    } });
    var Ce = a(yt, 2);
    W(Ce, "margin-top", "1rem");
    var Ct = v(Ce);
    oe(Ct, { type: "submit", children: (s, Y) => {
      J();
      var T = Q();
      O(() => I(T, r.common.save)), u(s, T);
    }, $$slots: { default: true } });
    var Gt = a(Ct, 2);
    {
      var Jt = (s) => {
        Nt(s, {});
      };
      ee(Gt, (s) => {
        t(w) && s(Jt);
      });
    }
    c(Ce);
    var Pt = a(Ce, 2);
    {
      var Qt = (s) => {
        var Y = Ir(), T = v(Y, true);
        c(Y), O(() => I(T, t(f))), u(s, Y);
      };
      ee(Pt, (s) => {
        t(f) && s(Qt);
      });
    }
    var Xt = a(Pt, 2);
    W(Xt, "height", "1rem"), O(() => {
      I(V, o.common.information), I(ce, o.clients.descName), I(se, o.clients.descClientUri), I(Bt, o.clients.config), I(Dt, o.clients.descPKCE), I(Ot, o.clients.descPKCEEnforce), I(jt, o.clients.descOrigin), I(Ht, o.clients.tokenLifetime.p1), I(Kt, o.clients.tokenLifetime.p2), I(Vt, o.clients.tokenLifetime.p3), I(Wt, o.clients.descAuthCode);
    }), u(G, $);
  }, $$slots: { default: true } }), c(p), u(M, p), le();
}
var Mr = q("<!> <!>", 1), Ur = q('<div class="container svelte-19xcrje"><div class="err"> </div> <!></div>');
function Nr(M, e) {
  ae(e, true);
  let r = me(), o = x(""), d = x(void 0);
  he(() => {
    n(o, ""), n(d, ""), e.client.confidential ? f() : n(o, i(r.clients.confidentialNoSecret));
  });
  async function f() {
    let g = await Ve(`/auth/v1/clients/${e.client.id}/secret`);
    _(g);
  }
  async function w() {
    let g = await We(`/auth/v1/clients/${e.client.id}/secret`);
    _(g);
  }
  function _(g) {
    var _a;
    g.body ? g.body.secret && n(d, i(g.body.secret)) : n(o, i(((_a = g.error) == null ? void 0 : _a.message) || "Error"));
  }
  var b = Ur(), y = v(b), k = v(y, true);
  c(y);
  var h = a(y, 2);
  {
    var R = (g) => {
      var C = Mr(), P = re(C);
      vr(P, { autocomplete: "off", label: "Client Secret", placeholder: "Client Secret", disabled: true, showCopy: true, get value() {
        return t(d);
      }, set value(B) {
        n(d, i(B));
      } });
      var L = a(P, 2);
      oe(L, { onclick: w, children: (B, N) => {
        J();
        var z = Q();
        O(() => I(z, r.clients.generateSecret)), u(B, z);
      }, $$slots: { default: true } }), u(g, C);
    };
    ee(h, (g) => {
      t(d) && g(R);
    });
  }
  c(b), O(() => I(k, t(o))), u(M, b), le();
}
var Br = q('<div class="err"> </div>'), Dr = q("<p> </p> <!> <!>", 1);
function Or(M, e) {
  ae(e, true);
  let r = xe(), o = me(), d = x("");
  async function f() {
    n(d, "");
    let R = await He(`/auth/v1/clients/${e.client.id}`);
    R.error ? n(d, i(R.error.message)) : e.onSave();
  }
  var w = Dr(), _ = re(w), b = v(_, true);
  c(_);
  var y = a(_, 2);
  oe(y, { level: -1, onclick: f, children: (R, g) => {
    J();
    var C = Q();
    O(() => I(C, r.common.delete)), u(R, C);
  }, $$slots: { default: true } });
  var k = a(y, 2);
  {
    var h = (R) => {
      var g = Br(), C = v(g, true);
      c(g), O(() => I(C, t(d))), u(R, g);
    };
    ee(k, (R) => {
      t(d) && R(h);
    });
  }
  O(() => I(b, o.clients.delete1)), u(M, w), le();
}
var qr = q('<div><div class="flex"><input type="range" class="svelte-12j8dit"> <div class="value font-mono svelte-12j8dit"> </div></div> <div class="label svelte-12j8dit"><label class="font-label noselect svelte-12j8dit"> </label></div></div>');
function qe(M, e) {
  ae(e, true);
  let r = Z(e, "value", 15), o = Z(e, "label", 3, ""), d = Z(e, "disabled", 3, false), f = Z(e, "step", 3, 1), w = Z(e, "widthRange", 3, "15rem");
  const _ = be();
  let b = _e(() => {
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
  var y = qr(), k = v(y), h = v(k);
  Je(h), F(h, "id", _);
  var R = a(h, 2), g = v(R, true);
  c(R), c(k);
  var C = a(k, 2), P = v(C);
  F(P, "for", _);
  var L = v(P, true);
  c(P), c(C), c(y), O(() => {
    F(h, "name", e.name), F(h, "title", o()), F(h, "aria-label", o()), h.disabled = d(), F(h, "aria-disabled", d()), F(h, "min", e.min), F(h, "max", e.max), F(h, "step", f()), W(h, "width", w()), W(h, "--bg-slider", t(b)), I(g, r()), I(L, o());
  }), Qe(h, r), u(M, y), le();
}
var jr = q('<div class="outer svelte-1evkwuy"><div class="container svelte-1evkwuy"><div><label class="font-label svelte-1evkwuy"> </label> <div class="values svelte-1evkwuy"><!> <!> <!></div></div> <div class="color svelte-1evkwuy"></div></div></div>');
function ge(M, e) {
  ae(e, true);
  let r = Z(e, "h", 15), o = Z(e, "s", 15), d = Z(e, "l", 15);
  const f = be(), w = "15rem";
  let _ = _e(() => `hsl(${r()} ${o()} ${d()})`);
  var b = jr(), y = v(b), k = v(y), h = v(k);
  F(h, "for", f);
  var R = v(h, true);
  c(h);
  var g = a(h, 2);
  F(g, "id", f);
  var C = v(g);
  qe(C, { label: "Hue", min: 0, max: 359, widthRange: w, bgMode: "hue", get hue() {
    return r();
  }, get sat() {
    return o();
  }, get lum() {
    return d();
  }, get value() {
    return r();
  }, set value(N) {
    r(N);
  } });
  var P = a(C, 2);
  qe(P, { label: "Sat", min: 0, max: 100, widthRange: w, bgMode: "sat", get hue() {
    return r();
  }, get sat() {
    return o();
  }, get lum() {
    return d();
  }, get value() {
    return o();
  }, set value(N) {
    o(N);
  } });
  var L = a(P, 2);
  qe(L, { label: "Lum", min: 0, max: 100, widthRange: w, bgMode: "lum", get hue() {
    return r();
  }, get sat() {
    return o();
  }, get lum() {
    return d();
  }, get value() {
    return d();
  }, set value(N) {
    d(N);
  } }), c(g), c(k);
  var B = a(k, 2);
  c(y), c(b), O(() => {
    W(y, "border-color", t(_)), I(R, e.label), W(B, "background", t(_));
  }), u(M, b), le();
}
var Fr = (M, e) => {
  var _a;
  return (_a = t(e)) == null ? void 0 : _a.click();
}, zr = q('<div class="container svelte-2nx18n"><!> <div role="none" class="mask svelte-2nx18n"><div class="colorWrapper svelte-2nx18n"><div class="color svelte-2nx18n"></div></div> <input type="color" class="svelte-2nx18n"></div></div>');
function je(M, e) {
  ae(e, true);
  let r = Z(e, "value", 15), o = me(), d = x(void 0);
  var f = zr(), w = v(f);
  fe(w, { get label() {
    return e.label;
  }, get placeholder() {
    return e.label;
  }, get errMsg() {
    return o.validation.css;
  }, width: "17.5rem", required: true, pattern: Et, get value() {
    return r();
  }, set value(y) {
    r(y);
  } });
  var _ = a(w, 2);
  _.__click = [Fr, d];
  var b = a(v(_), 2);
  Je(b), Mt(b, (y) => n(d, y), () => t(d)), c(_), c(f), O(() => W(f, "--color", r())), Qe(b, r), u(M, f), le();
}
Ke(["click"]);
var Hr = q('<div><p> </p> <div class="hsl svelte-10udpph"><div><!> <!> <!> <!></div> <div><!> <!> <!></div></div> <div><p><!></p> <!> <!> <!></div></div>');
function It(M, e) {
  ae(e, true);
  let r = Z(e, "values", 15), o = me();
  var d = Hr(), f = v(d), w = v(f, true);
  c(f);
  var _ = a(f, 2), b = v(_), y = v(b);
  ge(y, { label: "--text", get h() {
    return r().text[0];
  }, set h(l) {
    r(r().text[0] = l, true);
  }, get s() {
    return r().text[1];
  }, set s(l) {
    r(r().text[1] = l, true);
  }, get l() {
    return r().text[2];
  }, set l(l) {
    r(r().text[2] = l, true);
  } });
  var k = a(y, 2);
  ge(k, { label: "--text-high", get h() {
    return r().text_high[0];
  }, set h(l) {
    r(r().text_high[0] = l, true);
  }, get s() {
    return r().text_high[1];
  }, set s(l) {
    r(r().text_high[1] = l, true);
  }, get l() {
    return r().text_high[2];
  }, set l(l) {
    r(r().text_high[2] = l, true);
  } });
  var h = a(k, 2);
  ge(h, { label: "--bg", get h() {
    return r().bg[0];
  }, set h(l) {
    r(r().bg[0] = l, true);
  }, get s() {
    return r().bg[1];
  }, set s(l) {
    r(r().bg[1] = l, true);
  }, get l() {
    return r().bg[2];
  }, set l(l) {
    r(r().bg[2] = l, true);
  } });
  var R = a(h, 2);
  ge(R, { label: "--bg-high", get h() {
    return r().bg_high[0];
  }, set h(l) {
    r(r().bg_high[0] = l, true);
  }, get s() {
    return r().bg_high[1];
  }, set s(l) {
    r(r().bg_high[1] = l, true);
  }, get l() {
    return r().bg_high[2];
  }, set l(l) {
    r(r().bg_high[2] = l, true);
  } }), c(b);
  var g = a(b, 2), C = v(g);
  ge(C, { label: "--action", get h() {
    return r().action[0];
  }, set h(l) {
    r(r().action[0] = l, true);
  }, get s() {
    return r().action[1];
  }, set s(l) {
    r(r().action[1] = l, true);
  }, get l() {
    return r().action[2];
  }, set l(l) {
    r(r().action[2] = l, true);
  } });
  var P = a(C, 2);
  ge(P, { label: "--accent", get h() {
    return r().accent[0];
  }, set h(l) {
    r(r().accent[0] = l, true);
  }, get s() {
    return r().accent[1];
  }, set s(l) {
    r(r().accent[1] = l, true);
  }, get l() {
    return r().accent[2];
  }, set l(l) {
    r(r().accent[2] = l, true);
  } });
  var L = a(P, 2);
  ge(L, { label: "--error", get h() {
    return r().error[0];
  }, set h(l) {
    r(r().error[0] = l, true);
  }, get s() {
    return r().error[1];
  }, set s(l) {
    r(r().error[1] = l, true);
  }, get l() {
    return r().error[2];
  }, set l(l) {
    r(r().error[2] = l, true);
  } }), c(g), c(_);
  var B = a(_, 2), N = v(B), z = v(N);
  we(z, () => o.clients.branding.descFullCss), c(N);
  var X = a(N, 2);
  je(X, { label: "--btn-text", get value() {
    return r().btn_text;
  }, set value(l) {
    r(r().btn_text = l, true);
  } });
  var A = a(X, 2);
  je(A, { label: "--theme-sun", get value() {
    return r().theme_sun;
  }, set value(l) {
    r(r().theme_sun = l, true);
  } });
  var D = a(A, 2);
  je(D, { label: "--theme-moon", get value() {
    return r().theme_moon;
  }, set value(l) {
    r(r().theme_moon = l, true);
  } }), c(B), c(d), O(() => I(w, o.clients.branding.descHsl)), u(M, d), le();
}
var Kr = q('<img alt="Client Logo" width="100%" height="100%">'), Vr = (M) => M.preventDefault(), Wr = q('<div aria-label="Preview: All components inside are only for theme and colors preview and have no effect or interaction"><div class="inner svelte-fkmn75"><div class="container svelte-fkmn75"><div class="logo svelte-fkmn75"><!></div> <h3 class="svelte-fkmn75">Header</h3> <p>Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam</p> <p><span class="svelte-fkmn75">--accent-color</span></p> <!> <div class="btn svelte-fkmn75"><div class="svelte-fkmn75"><!> <br> <!> <br> <!> <br></div> <div class="svelte-fkmn75"><!> <br> <!> <br> <!></div></div> <a>Link</a> <br> <!></div></div></div>');
function Rt(M, e) {
  ae(e, true);
  let r = x(false), o = x(false);
  he(() => {
    e.logoUrl && n(o, false);
  }), he(() => {
    t(r) && setTimeout(() => {
      n(r, false);
    }, 2e3);
  });
  var d = Wr(), f = v(d), w = v(f), _ = v(w), b = v(_);
  {
    var y = (l) => {
      _r(l, { width: "100%", get show() {
        return e.typ;
      } });
    }, k = (l) => {
      var m = Kr();
      O(() => F(m, "src", e.logoUrl)), Fe("error", m, () => n(o, true)), Zt(m), u(l, m);
    };
    ee(b, (l) => {
      t(o) ? l(y) : l(k, false);
    });
  }
  c(_);
  var h = a(_, 8);
  fe(h, { label: "Preview", placeholder: "Preview", width: "12.5rem" });
  var R = a(h, 2), g = v(R), C = v(g);
  oe(C, { level: 1, get isLoading() {
    return t(r);
  }, onclick: () => n(r, true), children: (l, m) => {
    J();
    var E = Q("Button");
    u(l, E);
  }, $$slots: { default: true } });
  var P = a(C, 4);
  oe(P, { level: 2, get isLoading() {
    return t(r);
  }, onclick: () => n(r, true), children: (l, m) => {
    J();
    var E = Q("Button");
    u(l, E);
  }, $$slots: { default: true } });
  var L = a(P, 4);
  oe(L, { level: 3, get isLoading() {
    return t(r);
  }, onclick: () => n(r, true), children: (l, m) => {
    J();
    var E = Q("Button");
    u(l, E);
  }, $$slots: { default: true } }), J(2), c(g);
  var B = a(g, 2), N = v(B);
  oe(N, { level: -1, get isLoading() {
    return t(r);
  }, onclick: () => n(r, true), children: (l, m) => {
    J();
    var E = Q("Button");
    u(l, E);
  }, $$slots: { default: true } });
  var z = a(N, 4);
  oe(z, { level: -2, get isLoading() {
    return t(r);
  }, onclick: () => n(r, true), children: (l, m) => {
    J();
    var E = Q("Button");
    u(l, E);
  }, $$slots: { default: true } });
  var X = a(z, 4);
  oe(X, { level: -3, get isLoading() {
    return t(r);
  }, onclick: () => n(r, true), children: (l, m) => {
    J();
    var E = Q("Button");
    u(l, E);
  }, $$slots: { default: true } }), c(B), c(R);
  var A = a(R, 2);
  F(A, "href", window.location.href), A.__click = [Vr];
  var D = a(A, 4);
  fr(D, {}), c(w), c(f), c(d), O((l, m, E, S, p, H, K) => {
    W(d, "--border-radius", e.borderRadius), W(d, "--text", l), W(d, "--text-high", m), W(d, "--bg", E), W(d, "--bg-high", S), W(d, "--action", p), W(d, "--accent", H), W(d, "--error", K), W(d, "--btn-text", e.theme.btn_text), W(d, "--theme-sun", e.theme.theme_sun), W(d, "--theme-moon", e.theme.theme_moon);
  }, [() => e.theme.text.join(" "), () => e.theme.text_high.join(" "), () => e.theme.bg.join(" "), () => e.theme.bg_high.join(" "), () => e.theme.action.join(" "), () => e.theme.accent.join(" "), () => e.theme.error.join(" ")]), u(M, d), le();
}
Ke(["click"]);
var Gr = q('<div><h2>Preview</h2> <div class="tabs svelte-1klemk8"><!></div> <div class="preview svelte-1klemk8"><!></div></div>');
function Jr(M, e) {
  ae(e, true);
  let r = ["Light Theme", "Dark Theme"], o = x(i(r[0]));
  var d = Gr(), f = a(v(d), 2), w = v(f);
  Ut(w, { tabs: r, center: true, get selected() {
    return t(o);
  }, set selected(h) {
    n(o, i(h));
  } }), c(f);
  var _ = a(f, 2), b = v(_);
  {
    var y = (h) => {
      Rt(h, { get logoUrl() {
        return e.logoUrl;
      }, get borderRadius() {
        return e.theme.border_radius;
      }, get theme() {
        return e.theme.light;
      }, typ: "light" });
    }, k = (h) => {
      Rt(h, { get logoUrl() {
        return e.logoUrl;
      }, get borderRadius() {
        return e.theme.border_radius;
      }, get theme() {
        return e.theme.dark;
      }, typ: "dark" });
    };
    ee(b, (h) => {
      t(o) === r[0] ? h(y) : h(k, false);
    });
  }
  c(_), c(d), u(M, d), le();
}
var Qr = q('<div class="err"> </div>'), Xr = q('<div class="values svelte-1pkqupw"><p><!></p> <!> <h1>Light Theme</h1> <!> <br> <h1>Dark Theme</h1> <!></div> <div class="preview"><!> <hr> <p>Logo Upload</p> <!> <div class="buttons svelte-1pkqupw"><!> <!> <!></div> <!></div>', 1), Yr = q('<div class="container svelte-1pkqupw"><!></div>');
function Zr(M, e) {
  ae(e, true);
  const r = "13rem";
  let o = xe(), d = me(), f = x(false), w = x(""), _ = x(void 0), b = x(i(be())), y = _e(() => `/auth/v1/clients/${e.client.id}/logo?${t(b)}`), k = _e(() => `/auth/v1/theme/${e.client.id}`);
  he(() => {
    h();
  });
  async function h() {
    var _a;
    let P = await Ve(t(k));
    P.body ? (P.body.client_id === "rauthy" && console.log("using Rauthy default scheme"), n(_, i(P.body))) : n(w, i(((_a = P.error) == null ? void 0 : _a.message) || "Error"));
  }
  async function R(P, L) {
    if (!t(_)) {
      console.error("theme is undefined");
      return;
    }
    let B = t(_);
    B.client_id = e.client.id;
    let N = await We(t(k), B);
    N.error ? n(w, i(N.error.message)) : (n(f, true), setTimeout(() => {
      n(f, false);
    }, 2e3));
  }
  async function g() {
    let P = await He(t(k));
    P.error ? n(w, i(P.error.message)) : (await He(`/auth/v1/clients/${e.client.id}/logo`), await h(), n(b, i(be())), n(f, true), setTimeout(() => {
      n(f, false);
    }, 2e3));
  }
  async function C() {
    n(b, i(be()));
  }
  Ge(M, { get action() {
    return t(k);
  }, onSubmit: R, children: (P, L) => {
    var B = Yr(), N = v(B);
    {
      var z = (X) => {
        var A = Xr(), D = re(A), l = v(D), m = v(l);
        we(m, () => d.clients.branding.descVariables), c(l);
        var E = a(l, 2);
        fe(E, { label: "--border-radius", placeholder: "--border-radius", get errMsg() {
          return d.validation.css;
        }, width: r, required: true, pattern: Et, get value() {
          return t(_).border_radius;
        }, set value(j) {
          t(_).border_radius = j;
        } });
        var S = a(E, 4);
        It(S, { get values() {
          return t(_).light;
        }, set values(j) {
          t(_).light = j;
        } });
        var p = a(S, 6);
        It(p, { get values() {
          return t(_).dark;
        }, set values(j) {
          t(_).dark = j;
        } }), c(D);
        var H = a(D, 2), K = v(H);
        ur(K, () => t(b), (j) => {
          Jr(j, { get logoUrl() {
            return t(y);
          }, get theme() {
            return t(_);
          } });
        });
        var G = a(K, 6);
        const ie = _e(() => `/auth/v1/clients/${e.client.id}/logo`);
        hr(G, { method: "PUT", get url() {
          return t(ie);
        }, fileName: "logo", onSuccess: C });
        var $ = a(G, 2), U = v($);
        oe(U, { type: "submit", children: (j, se) => {
          J();
          var ve = Q();
          O(() => I(ve, o.common.save)), u(j, ve);
        }, $$slots: { default: true } });
        var V = a(U, 2);
        oe(V, { level: -2, onclick: g, children: (j, se) => {
          J();
          var ve = Q();
          O(() => I(ve, d.common.reset)), u(j, ve);
        }, $$slots: { default: true } });
        var de = a(V, 2);
        {
          var te = (j) => {
            Nt(j, {});
          };
          ee(de, (j) => {
            t(f) && j(te);
          });
        }
        c($);
        var ce = a($, 2);
        {
          var ne = (j) => {
            var se = Qr(), ve = v(se, true);
            c(se), O(() => I(ve, t(w))), u(j, se);
          };
          ee(ce, (j) => {
            t(w) && j(ne);
          });
        }
        c(H), u(X, A);
      };
      ee(N, (X) => {
        t(_) && X(z);
      });
    }
    c(B), u(P, B);
  }, $$slots: { default: true } }), le();
}
var $r = q('<div class="flex"><!></div> <div class="details"><!></div>', 1);
function ea(M, e) {
  ae(e, true);
  let r = xe(), o = me();
  const d = [o.nav.config, "Secret", "Branding", r.common.delete];
  let f = x(i(d[0])), w = x(void 0);
  he(() => {
    e.client.id && requestAnimationFrame(() => {
      var _a;
      (_a = t(w)) == null ? void 0 : _a();
    });
  });
  var _ = $r(), b = re(_), y = v(b);
  Ut(y, { tabs: d, get selected() {
    return t(f);
  }, set selected(C) {
    n(f, i(C));
  }, get focusFirst() {
    return t(w);
  }, set focusFirst(C) {
    n(w, i(C));
  } }), c(b);
  var k = a(b, 2), h = v(k);
  {
    var R = (C) => {
      Er(C, { get client() {
        return e.client;
      }, get clients() {
        return e.clients;
      }, get scopesAll() {
        return e.scopesAll;
      }, get onSave() {
        return e.onSave;
      } });
    }, g = (C) => {
      var P = Pe(), L = re(P);
      {
        var B = (z) => {
          Nr(z, { get client() {
            return e.client;
          } });
        }, N = (z) => {
          var X = Pe(), A = re(X);
          {
            var D = (m) => {
              Zr(m, { get client() {
                return e.client;
              } });
            }, l = (m) => {
              var E = Pe(), S = re(E);
              {
                var p = (H) => {
                  Or(H, { get client() {
                    return e.client;
                  }, get onSave() {
                    return e.onSave;
                  } });
                };
                ee(S, (H) => {
                  t(f) === r.common.delete && H(p);
                }, true);
              }
              u(m, E);
            };
            ee(A, (m) => {
              t(f) === "Branding" ? m(D) : m(l, false);
            }, true);
          }
          u(z, X);
        };
        ee(L, (z) => {
          t(f) === "Secret" ? z(B) : z(N, false);
        }, true);
      }
      u(C, P);
    };
    ee(h, (C) => {
      t(f) === o.nav.config ? C(R) : C(g, false);
    });
  }
  c(k), u(M, _), le();
}
var ta = q('<div class="tile svelte-140q9n9"> <div class="muted svelte-140q9n9"> </div></div>'), ra = q("<div></div> <!>", 1), aa = q("<!> <!>", 1), la = q('<div class="err"> </div>'), ia = q('<!> <div id="groups"><!></div>', 1), na = q("<!> <!>", 1);
function sa(M, e) {
  ae(e, true);
  let r = x(void 0), o = x(""), d = x(i([])), f = x(i([])), w = x(void 0), _ = nr("cid"), b = x(i([]));
  const y = ["ID"];
  let k = x(i(y[0])), h = x("");
  const R = ["ID"];
  Yt(() => {
    g(), C();
  }), he(() => {
    n(w, i(t(d).find((A) => A.id === _.get())));
  }), he(() => {
    let A = t(h).toLowerCase();
    A ? t(k) === y[0] && n(f, i(t(d).filter((D) => D.id.toLowerCase().includes(A)))) : n(f, i(t(d)));
  });
  async function g() {
    var _a;
    let A = await St("/auth/v1/clients");
    A.body ? n(d, i(A.body)) : n(o, i(((_a = A.error) == null ? void 0 : _a.message) || "Error"));
  }
  async function C() {
    var _a;
    let A = await St("/auth/v1/scopes");
    A.body ? n(b, i(A.body.map((D) => D.name))) : n(o, i(((_a = A.error) == null ? void 0 : _a.message) || "Error"));
  }
  function P(A, D) {
    let l = D === "up";
    A === R[0] && t(d).sort((m, E) => l ? m.id.localeCompare(E.id) : E.id.localeCompare(m.id));
  }
  function L() {
    g();
  }
  async function B(A) {
    var _a;
    (_a = t(r)) == null ? void 0 : _a(), await g(), _.set(A);
  }
  var N = na(), z = re(N);
  lr(z, { paddingTop: "2.1rem", buttonTilesAriaControls: "groups", width: "min(20rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (D) => {
    var l = ra(), m = re(l);
    W(m, "height", ".5rem");
    var E = a(m, 2);
    ze(E, 17, () => t(f), (S) => S.id, (S, p, H, K) => {
      const G = _e(() => _.get() === t(p).id);
      ar(S, { onclick: () => _.set(t(p).id), get selected() {
        return t(G);
      }, children: (ie, $) => {
        var U = ta(), V = v(U), de = a(V), te = v(de, true);
        c(de), c(U), O(() => {
          I(V, `${t(p).id ?? ""} `), I(te, t(p).name);
        }), u(ie, U);
      } });
    }), u(D, l);
  }, children: (D, l) => {
    var m = aa(), E = re(m);
    const S = _e(() => t(d).length === 0 ? 1 : 2);
    tr(E, { get level() {
      return t(S);
    }, alignRight: true, get closeModal() {
      return t(r);
    }, set closeModal(H) {
      n(r, i(H));
    }, children: (H, K) => {
      Ar(H, { onSave: B, get clients() {
        return t(d);
      } });
    }, $$slots: { default: true } });
    var p = a(E, 2);
    ir(p, { searchOptions: y, orderOptions: R, onChangeOrder: P, searchWidth: "min(19.5rem, 100dvw - .5rem)", get searchOption() {
      return t(k);
    }, set searchOption(H) {
      n(k, i(H));
    }, get value() {
      return t(h);
    }, set value(H) {
      n(h, i(H));
    } }), u(D, m);
  }, $$slots: { buttonTiles: true, default: true } });
  var X = a(z, 2);
  rr(X, { children: (A, D) => {
    var l = ia(), m = re(l);
    {
      var E = (K) => {
        var G = la(), ie = v(G, true);
        c(G), O(() => I(ie, t(o))), u(K, G);
      };
      ee(m, (K) => {
        t(o) && K(E);
      });
    }
    var S = a(m, 2), p = v(S);
    {
      var H = (K) => {
        ea(K, { get client() {
          return t(w);
        }, get clients() {
          return t(d);
        }, get scopesAll() {
          return t(b);
        }, onSave: L });
      };
      ee(p, (K) => {
        t(w) && K(H);
      });
    }
    c(S), u(A, l);
  } }), u(M, N), le();
}
function Wa(M) {
  sa(M, {});
}
export {
  Wa as component
};
