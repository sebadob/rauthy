import { t as q, a as u, d as Pe, e as J } from "../chunks/BH6NCLk-.js";
import { p as re, c as v, s as a, k as x, t as O, l as n, j as t, a as ae, r as c, a9 as he, f as te, aa as _e, a7 as V, a5 as Yt } from "../chunks/CvlvO1XB.js";
import { d as Ke, e as Fe, s as R, b as Zt } from "../chunks/CTI4QPiR.js";
import { i as ee } from "../chunks/BUO_AUgz.js";
import { e as ze, i as Lt } from "../chunks/BpWRzPRQ.js";
import { a as W, B as se, t as $t, s as er } from "../chunks/DMkkW5Nn.js";
import { p as i } from "../chunks/Wh68IIk2.js";
import { B as tr } from "../chunks/DC7AzrtQ.js";
import { C as rr } from "../chunks/BnPoFdx3.js";
import { N as ar } from "../chunks/D-L0o8jR.js";
import { N as lr } from "../chunks/G5KuhWXq.js";
import { O as ir } from "../chunks/B6KLrTr9.js";
import { b as Ve, d as We, c as He, f as St } from "../chunks/CRjU5SuJ.js";
import { u as nr } from "../chunks/rQ2Hm1Il.js";
import { h as we } from "../chunks/i8Xqpu09.js";
import { I as fe } from "../chunks/D-haFZtM.js";
import { n as sr, g as pt, a as Be, o as or, p as dr, q as Et } from "../chunks/BRCxk8by.js";
import { F as Ge } from "../chunks/BJ_mu-WB.js";
import { u as xe } from "../chunks/CgEHB2u3.js";
import { u as ge } from "../chunks/D8mHI_K9.js";
import { I as ve } from "../chunks/DTR8xafZ.js";
import { r as Je, s as F } from "../chunks/BMbqVy6X.js";
import { b as Qe } from "../chunks/BhIBACXG.js";
import { b as Mt } from "../chunks/zosqiMUL.js";
import { p as Z } from "../chunks/C6SR4G2t.js";
import { g as be } from "../chunks/Brp0G0eV.js";
import { I as cr } from "../chunks/Vi3uK7uO.js";
import { T as Ut } from "../chunks/_OE2Cq0B.js";
import { I as Nt } from "../chunks/Nks81rMs.js";
import { L as De } from "../chunks/CE2_6siz.js";
import { S as At } from "../chunks/DwPt1Ew5.js";
import { O as Tt } from "../chunks/ahjiAH8y.js";
import { I as vr } from "../chunks/BUc4wf1T.js";
import { k as ur } from "../chunks/CaD2yKt4.js";
import { T as fr } from "../chunks/DX6irJfo.js";
import { R as hr } from "../chunks/BjNKHfO0.js";
import { I as _r } from "../chunks/CT7fLpQy.js";
function gr(M, e, r, o) {
  var _a;
  e(), (_a = r.onInput) == null ? void 0 : _a.call(r, t(o));
}
function mr(M, e, r) {
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
  re(e, true);
  let r = Z(e, "ref", 15), o = Z(e, "typ", 3, "text"), d = Z(e, "id", 19, be), f = Z(e, "values", 31, () => i([])), w = Z(e, "label", 3, ""), h = Z(e, "placeholder", 3, ""), b = Z(e, "disabled", 3, false), y = Z(e, "required", 3, false), k = Z(e, "isError", 15, false), _ = Z(e, "width", 3, "inherit"), I = Z(e, "maxHeightList", 3, "inherit"), m = xe();
  const C = be();
  let P = x(""), L = he(() => e.datalist && e.datalist.length > 0 ? C : void 0);
  function B(N) {
    f(f().filter((X) => X !== N));
  }
  function U(N) {
    var _a;
    A(), (_a = e.onBlur) == null ? void 0 : _a.call(e), Q();
  }
  function z(N) {
    N.preventDefault(), k(true);
  }
  function Q() {
    t(P) && A() && (f().push(t(P)), n(P, ""));
  }
  function A() {
    var _a;
    let N = (_a = r()) == null ? void 0 : _a.validity;
    return N ? (k(!N.valid), N.valid) : (k(false), true);
  }
  var D = Cr();
  D.__click = [br, r];
  var l = v(D), g = v(l);
  ze(g, 17, f, Lt, (N, X, ue, oe) => {
    var de = kr(), ie = v(de), j = v(ie, true);
    c(ie);
    var ne = a(ie, 2), ce = v(ne);
    se(ce, { invisible: true, onclick: () => B(t(X)), children: (ye, Le) => {
      cr(ye, { width: "1.2rem" });
    }, $$slots: { default: true } }), c(ne), c(de), O(() => R(j, t(X))), u(N, de);
  });
  var E = a(g, 2), S = v(E);
  Je(S), S.__keydown = [mr, Q, e], S.__input = [gr, A, e, P], Mt(S, (N) => r(N), () => r());
  var p = a(S, 2);
  {
    var H = (N) => {
      var X = wr();
      F(X, "id", C), ze(X, 21, () => e.datalist, Lt, (ue, oe, de, ie) => {
        var j = xr(), ne = {};
        O(() => {
          ne !== (ne = t(oe)) && (j.value = (j.__value = t(oe)) == null ? "" : t(oe));
        }), u(ue, j);
      }), c(X), u(N, X);
    };
    ee(p, (N) => {
      e.datalist && e.datalist.length > 1 && N(H);
    });
  }
  c(E), c(l);
  var K = a(l, 2), G = v(K), le = a(G);
  {
    var $ = (N) => {
      var X = Pe(), ue = te(X);
      {
        var oe = (de) => {
          var ie = yr(), j = v(ie, true);
          c(ie), O(() => R(j, e.errMsg || m.common.invalidInput)), u(de, ie);
        };
        ee(ue, (de) => {
          k() && de(oe);
        });
      }
      u(N, X);
    };
    ee(le, (N) => {
      k() && N($);
    });
  }
  c(K), c(D), O(() => {
    W(D, "width", _()), W(l, "max-height", I()), F(S, "type", o()), F(S, "id", d()), F(S, "name", e.name), F(S, "list", t(L)), F(S, "title", e.errMsg), F(S, "aria-label", w() || h()), F(S, "placeholder", h()), F(S, "aria-placeholder", h()), S.disabled = b(), F(S, "aria-disabled", b()), S.required = y() && f().length < 1, F(S, "aria-required", y() && f().length < 1), F(S, "maxlength", e.maxLength || void 0), F(S, "min", e.min || void 0), F(S, "max", e.max || void 0), F(S, "pattern", e.pattern), F(K, "for", d()), F(K, "data-required", y()), R(G, `${w() ?? ""} `);
  }), Fe("invalid", S, z), Fe("blur", S, U), Qe(S, () => t(P), (N) => n(P, N)), u(M, D), ae();
}
Ke(["click", "keydown", "input"]);
var Pr = q('<div class="err"> </div>'), Lr = q('<!> <!> <!> <p class="svelte-15tp6i6"><!></p> <!> <!> <!> <!>', 1), Sr = q('<div class="container svelte-15tp6i6"><!></div>');
function Ar(M, e) {
  re(e, true);
  let r = xe(), o = ge(), d = x(void 0), f = x(""), w = x(""), h = x(""), b = x(true), y = x(i([])), k = x(i([]));
  _e(() => {
    requestAnimationFrame(() => {
      var _a;
      (_a = t(d)) == null ? void 0 : _a.focus();
    });
  });
  async function _(C, P) {
    var _a;
    if (e.clients.find((U) => U.id === t(w))) {
      n(f, i(o.common.nameExistsAlready));
      return;
    }
    n(f, "");
    let L = { id: t(w), name: t(h) || void 0, confidential: t(b), redirect_uris: t(y), post_logout_redirect_uris: t(k).length > 0 ? t(k) : void 0 }, B = await Ve(C.action, L);
    B.body ? e.onSave(B.body.id) : n(f, i(((_a = B.error) == null ? void 0 : _a.message) || "Error"));
  }
  var I = Sr(), m = v(I);
  Ge(m, { action: "/auth/v1/clients", onSubmit: _, children: (C, P) => {
    var L = Lr(), B = te(L);
    fe(B, { autocomplete: "off", label: "Client ID", placeholder: "Client ID", required: true, pattern: sr, get ref() {
      return t(d);
    }, set ref(p) {
      n(d, i(p));
    }, get value() {
      return t(w);
    }, set value(p) {
      n(w, i(p));
    } });
    var U = a(B, 2);
    fe(U, { autocomplete: "off", get label() {
      return o.clients.name;
    }, get placeholder() {
      return o.clients.name;
    }, pattern: pt, get value() {
      return t(h);
    }, set value(p) {
      n(h, i(p));
    } });
    var z = a(U, 2);
    ve(z, { get ariaLabel() {
      return o.clients.confidential;
    }, get checked() {
      return t(b);
    }, set checked(p) {
      n(b, i(p));
    }, children: (p, H) => {
      V();
      var K = J();
      O(() => R(K, o.clients.confidential)), u(p, K);
    }, $$slots: { default: true } });
    var Q = a(z, 2), A = v(Q);
    we(A, () => o.clients.descUri), c(Q);
    var D = a(Q, 2);
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
    var g = a(l, 2);
    se(g, { type: "submit", children: (p, H) => {
      V();
      var K = J();
      O(() => R(K, r.common.save)), u(p, K);
    }, $$slots: { default: true } });
    var E = a(g, 2);
    {
      var S = (p) => {
        var H = Pr(), K = v(H, true);
        c(H), O(() => R(K, t(f))), u(p, H);
      };
      ee(E, (p) => {
        t(f) && p(S);
      });
    }
    u(C, L);
  }, $$slots: { default: true } }), c(I), u(M, I), ae();
}
const Oe = "urn:ietf:params:oauth:grant-type:device_code";
var Tr = q('<div class="err"> </div>'), Ir = q('<div class="err"> </div>'), Rr = q('<h5> </h5> <!> <p class="desc svelte-5tvvvt"> </p> <!> <p class="desc svelte-5tvvvt"> </p> <!> <!> <div></div> <h5> </h5> <!> <!> <!> <p class="mb-0 svelte-5tvvvt">Authentication Flows</p> <!> <!> <!> <!> <!> <div></div> <p class="desc svelte-5tvvvt"> </p> <p class="desc svelte-5tvvvt"><strong> </strong></p> <!> <!> <!> <div></div> <p class="desc svelte-5tvvvt"> </p> <!> <p class="desc svelte-5tvvvt"><!></p> <!> <!> <div></div> <p class="desc svelte-5tvvvt"><!></p> <!> <!> <div></div> <p> </p> <!> <div></div> <p> </p> <p> </p> <!> <!> <div></div> <p> </p> <!> <div class="flex gap-05"><!> <!></div> <!> <div></div>', 1), pr = q('<div class="container svelte-5tvvvt"><!></div>');
function Er(M, e) {
  var _a, _b;
  re(e, true);
  let r = xe(), o = ge();
  const d = "min(20rem, calc(100dvw - .5rem))";
  let f = x(""), w = x(false), h = x(i(e.client.id)), b = x(i(e.client.name || "")), y = x(i(e.client.enabled)), k = x(i(e.client.confidential)), _ = x(i(e.client.client_uri || "")), I = x(i(e.client.contacts ? Array.from(e.client.contacts) : [])), m = x(i(e.client.allowed_origins ? Array.from(e.client.allowed_origins) : [])), C = x(i(Array.from(e.client.redirect_uris))), P = x(i(e.client.post_logout_redirect_uris ? Array.from(e.client.post_logout_redirect_uris) : [])), L = i({ authorizationCode: e.client.flows_enabled.includes("authorization_code"), clientCredentials: e.client.flows_enabled.includes("client_credentials"), password: e.client.flows_enabled.includes("password"), refreshToken: e.client.flows_enabled.includes("refresh_token"), deviceCode: e.client.flows_enabled.includes(Oe) });
  const B = ["RS256", "RS384", "RS512", "EdDSA"];
  let U = x(i(e.client.access_token_alg)), z = x(i(e.client.id_token_alg)), Q = x(i(e.client.access_token_lifetime.toString())), A = x(i(e.client.auth_code_lifetime.toString())), D = x(i(e.scopesAll.map((G) => ({ name: G, selected: e.client.scopes.includes(G) || false })))), l = x(i(e.scopesAll.map((G) => ({ name: G, selected: e.client.default_scopes.includes(G) || false })))), g = i({ plain: ((_a = e.client.challenges) == null ? void 0 : _a.includes("plain")) || false, s256: ((_b = e.client.challenges) == null ? void 0 : _b.includes("S256")) || false }), E = x(i(e.client.force_mfa));
  _e(() => {
    var _a2, _b2;
    e.client.id && (n(h, i(e.client.id)), n(b, i(e.client.name || "")), n(y, i(e.client.enabled)), n(k, i(e.client.confidential)), n(_, i(e.client.client_uri || "")), n(I, i(e.client.contacts ? Array.from(e.client.contacts) : [])), n(m, i(e.client.allowed_origins ? Array.from(e.client.allowed_origins) : [])), n(C, i(Array.from(e.client.redirect_uris))), n(P, i(e.client.post_logout_redirect_uris ? Array.from(e.client.post_logout_redirect_uris) : [])), L.authorizationCode = e.client.flows_enabled.includes("authorization_code"), L.clientCredentials = e.client.flows_enabled.includes("client_credentials"), L.password = e.client.flows_enabled.includes("password"), L.refreshToken = e.client.flows_enabled.includes("refresh_token"), L.deviceCode = e.client.flows_enabled.includes(Oe), n(U, i(e.client.access_token_alg)), n(z, i(e.client.id_token_alg)), n(Q, i(e.client.access_token_lifetime.toString())), n(A, i(e.client.auth_code_lifetime.toString())), n(D, i(e.scopesAll.map((G) => ({ name: G, selected: e.client.scopes.includes(G) || false })))), n(l, i(e.scopesAll.map((G) => ({ name: G, selected: e.client.default_scopes.includes(G) || false })))), g.plain = ((_a2 = e.client.challenges) == null ? void 0 : _a2.includes("plain")) || false, g.s256 = ((_b2 = e.client.challenges) == null ? void 0 : _b2.includes("S256")) || false);
  });
  async function S(G, le) {
    n(f, "");
    let $ = { id: t(h), name: t(b) || void 0, enabled: t(y), confidential: t(k), allowed_origins: t(m).length > 0 ? t(m) : void 0, redirect_uris: t(C), post_logout_redirect_uris: t(P).length > 0 ? t(P) : void 0, flows_enabled: [], access_token_alg: t(U), id_token_alg: t(z), access_token_lifetime: Number.parseInt(t(Q)), auth_code_lifetime: Number.parseInt(t(A)), scopes: t(D).filter((X) => X.selected).map((X) => X.name), default_scopes: t(l).filter((X) => X.selected).map((X) => X.name), challenges: void 0, force_mfa: t(E), client_uri: t(_) || void 0, contacts: t(I).length > 0 ? t(I) : void 0 };
    L.authorizationCode && $.flows_enabled.push("authorization_code"), L.clientCredentials && $.flows_enabled.push("client_credentials"), L.password && $.flows_enabled.push("password"), L.refreshToken && $.flows_enabled.push("refresh_token"), L.deviceCode && $.flows_enabled.push(Oe), g.plain && ($.challenges = ["plain"]), g.s256 && ($.challenges ? $.challenges.push("S256") : $.challenges = ["S256"]);
    let N = await We(G.action, $);
    N.error ? n(f, i(N.error.message)) : (n(w, true), e.onSave(), setTimeout(() => {
      n(w, false);
    }, 2e3));
  }
  var p = pr(), H = v(p);
  const K = he(() => `/auth/v1/clients/${e.client.id}`);
  Ge(H, { get action() {
    return t(K);
  }, onSubmit: S, children: (G, le) => {
    var $ = Rr(), N = te($), X = v(N, true);
    c(N);
    var ue = a(N, 2);
    De(ue, { label: "ID", mono: true, children: (s, Y) => {
      V();
      var T = J();
      O(() => R(T, e.client.id)), u(s, T);
    } });
    var oe = a(ue, 2), de = v(oe, true);
    c(oe);
    var ie = a(oe, 2);
    fe(ie, { autocomplete: "off", get label() {
      return o.clients.name;
    }, get placeholder() {
      return o.clients.name;
    }, width: d, pattern: pt, get value() {
      return t(b);
    }, set value(s) {
      n(b, i(s));
    } });
    var j = a(ie, 2), ne = v(j, true);
    c(j);
    var ce = a(j, 2);
    fe(ce, { typ: "url", autocomplete: "off", label: "URI", placeholder: "URI", width: d, pattern: Be, get value() {
      return t(_);
    }, set value(s) {
      n(_, i(s));
    } });
    var ye = a(ce, 2);
    ke(ye, { get label() {
      return o.common.contact;
    }, pattern: or, get values() {
      return t(I);
    }, set values(s) {
      n(I, i(s));
    } });
    var Le = a(ye, 2);
    W(Le, "height", ".5rem");
    var Se = a(Le, 2), Bt = v(Se, true);
    c(Se);
    var Xe = a(Se, 2);
    ve(Xe, { get ariaLabel() {
      return o.common.enabled;
    }, get checked() {
      return t(y);
    }, set checked(s) {
      n(y, i(s));
    }, children: (s, Y) => {
      V();
      var T = J();
      O(() => R(T, o.common.enabled)), u(s, T);
    }, $$slots: { default: true } });
    var Ye = a(Xe, 2);
    ve(Ye, { get ariaLabel() {
      return o.clients.confidential;
    }, get checked() {
      return t(k);
    }, set checked(s) {
      n(k, i(s));
    }, children: (s, Y) => {
      V();
      var T = J();
      O(() => R(T, o.clients.confidential)), u(s, T);
    }, $$slots: { default: true } });
    var Ze = a(Ye, 2);
    ve(Ze, { get ariaLabel() {
      return o.clients.forceMfa;
    }, get checked() {
      return t(E);
    }, set checked(s) {
      n(E, i(s));
    }, children: (s, Y) => {
      V();
      var T = J();
      O(() => R(T, o.clients.forceMfa)), u(s, T);
    }, $$slots: { default: true } });
    var $e = a(Ze, 4);
    ve($e, { ariaLabel: "authorization_code", get checked() {
      return L.authorizationCode;
    }, set checked(s) {
      L.authorizationCode = s;
    }, children: (s, Y) => {
      V();
      var T = J("authorization_code");
      u(s, T);
    }, $$slots: { default: true } });
    var et = a($e, 2);
    ve(et, { ariaLabel: "urn:ietf:params:oauth:grant-type:device_code", get checked() {
      return L.deviceCode;
    }, set checked(s) {
      L.deviceCode = s;
    }, children: (s, Y) => {
      V();
      var T = J("device_code");
      u(s, T);
    }, $$slots: { default: true } });
    var tt = a(et, 2);
    ve(tt, { ariaLabel: "client_credentials", get checked() {
      return L.clientCredentials;
    }, set checked(s) {
      L.clientCredentials = s;
    }, children: (s, Y) => {
      V();
      var T = J("client_credentials");
      u(s, T);
    }, $$slots: { default: true } });
    var rt = a(tt, 2);
    ve(rt, { ariaLabel: "password", get checked() {
      return L.password;
    }, set checked(s) {
      L.password = s;
    }, children: (s, Y) => {
      V();
      var T = J("password");
      u(s, T);
    }, $$slots: { default: true } });
    var at = a(rt, 2);
    ve(at, { ariaLabel: "refresh_token", get checked() {
      return L.refreshToken;
    }, set checked(s) {
      L.refreshToken = s;
    }, children: (s, Y) => {
      V();
      var T = J("refresh_token");
      u(s, T);
    }, $$slots: { default: true } });
    var lt = a(at, 2);
    W(lt, "height", ".5rem");
    var Ae = a(lt, 2), Dt = v(Ae, true);
    c(Ae);
    var Te = a(Ae, 2), it = v(Te), Ot = v(it, true);
    c(it), c(Te);
    var nt = a(Te, 2);
    ve(nt, { ariaLabel: "PKCE plain", get checked() {
      return g.plain;
    }, set checked(s) {
      g.plain = s;
    }, children: (s, Y) => {
      V();
      var T = J("plain");
      u(s, T);
    }, $$slots: { default: true } });
    var st = a(nt, 2);
    ve(st, { ariaLabel: "PKCE S256", get checked() {
      return g.s256;
    }, set checked(s) {
      g.s256 = s;
    }, children: (s, Y) => {
      V();
      var T = J("S256");
      u(s, T);
    }, $$slots: { default: true } });
    var ot = a(st, 2);
    {
      var qt = (s) => {
        var Y = Tr(), T = v(Y, true);
        c(Y), O(() => R(T, o.clients.errConfidentialPKCE)), $t(3, Y, () => er, () => ({ duration: 150 })), u(s, Y);
      };
      ee(ot, (s) => {
        !t(k) && !g.plain && !g.s256 && s(qt);
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
      return t(m);
    }, set values(s) {
      n(m, i(s));
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
    var ht = a(pe, 2);
    At(ht, { get items() {
      return t(D);
    }, set items(s) {
      n(D, i(s));
    }, children: (s, Y) => {
      V();
      var T = J();
      O(() => R(T, o.clients.scopes.allowed)), u(s, T);
    }, $$slots: { default: true } });
    var _t = a(ht, 2);
    At(_t, { get items() {
      return t(l);
    }, set items(s) {
      n(l, i(s));
    }, children: (s, Y) => {
      V();
      var T = J();
      O(() => R(T, o.clients.scopes.default)), u(s, T);
    }, $$slots: { default: true } });
    var gt = a(_t, 2);
    W(gt, "height", ".75rem");
    var Ee = a(gt, 2), Ht = v(Ee, true);
    c(Ee);
    var mt = a(Ee, 2);
    fe(mt, { typ: "number", autocomplete: "off", label: "Token Lifetime", placeholder: "Token Lifetime", width: d, min: "10", max: "86400", errMsg: "10 <= Token Lifetime <= 86400", get value() {
      return t(Q);
    }, set value(s) {
      n(Q, i(s));
    } });
    var bt = a(mt, 2);
    W(bt, "height", ".5rem");
    var Me = a(bt, 2), Kt = v(Me, true);
    c(Me);
    var Ue = a(Me, 2), Vt = v(Ue, true);
    c(Ue);
    var kt = a(Ue, 2);
    De(kt, { label: "Access Token Algorithm", children: (s, Y) => {
      Tt(s, { ariaLabel: "Access Token Algorithm", options: B, borderless: true, get value() {
        return t(U);
      }, set value(T) {
        n(U, i(T));
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
    se(Ct, { type: "submit", children: (s, Y) => {
      V();
      var T = J();
      O(() => R(T, r.common.save)), u(s, T);
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
        c(Y), O(() => R(T, t(f))), u(s, Y);
      };
      ee(Pt, (s) => {
        t(f) && s(Qt);
      });
    }
    var Xt = a(Pt, 2);
    W(Xt, "height", "1rem"), O(() => {
      R(X, o.common.information), R(de, o.clients.descName), R(ne, o.clients.descClientUri), R(Bt, o.clients.config), R(Dt, o.clients.descPKCE), R(Ot, o.clients.descPKCEEnforce), R(jt, o.clients.descOrigin), R(Ht, o.clients.tokenLifetime.p1), R(Kt, o.clients.tokenLifetime.p2), R(Vt, o.clients.tokenLifetime.p3), R(Wt, o.clients.descAuthCode);
    }), u(G, $);
  }, $$slots: { default: true } }), c(p), u(M, p), ae();
}
var Mr = q("<!> <!>", 1), Ur = q('<div class="container svelte-19xcrje"><div class="err"> </div> <!></div>');
function Nr(M, e) {
  re(e, true);
  let r = ge(), o = x(""), d = x(void 0);
  _e(() => {
    n(o, ""), n(d, ""), e.client.confidential ? f() : n(o, i(r.clients.confidentialNoSecret));
  });
  async function f() {
    let m = await Ve(`/auth/v1/clients/${e.client.id}/secret`);
    h(m);
  }
  async function w() {
    let m = await We(`/auth/v1/clients/${e.client.id}/secret`);
    h(m);
  }
  function h(m) {
    var _a;
    m.body ? m.body.secret && n(d, i(m.body.secret)) : n(o, i(((_a = m.error) == null ? void 0 : _a.message) || "Error"));
  }
  var b = Ur(), y = v(b), k = v(y, true);
  c(y);
  var _ = a(y, 2);
  {
    var I = (m) => {
      var C = Mr(), P = te(C);
      vr(P, { autocomplete: "off", label: "Client Secret", placeholder: "Client Secret", disabled: true, showCopy: true, get value() {
        return t(d);
      }, set value(B) {
        n(d, i(B));
      } });
      var L = a(P, 2);
      se(L, { onclick: w, children: (B, U) => {
        V();
        var z = J();
        O(() => R(z, r.clients.generateSecret)), u(B, z);
      }, $$slots: { default: true } }), u(m, C);
    };
    ee(_, (m) => {
      t(d) && m(I);
    });
  }
  c(b), O(() => R(k, t(o))), u(M, b), ae();
}
var Br = q('<div class="err"> </div>'), Dr = q("<p> </p> <!> <!>", 1);
function Or(M, e) {
  re(e, true);
  let r = xe(), o = ge(), d = x("");
  async function f() {
    n(d, "");
    let I = await He(`/auth/v1/clients/${e.client.id}`);
    I.error ? n(d, i(I.error.message)) : e.onSave();
  }
  var w = Dr(), h = te(w), b = v(h, true);
  c(h);
  var y = a(h, 2);
  se(y, { level: -1, onclick: f, children: (I, m) => {
    V();
    var C = J();
    O(() => R(C, r.common.delete)), u(I, C);
  }, $$slots: { default: true } });
  var k = a(y, 2);
  {
    var _ = (I) => {
      var m = Br(), C = v(m, true);
      c(m), O(() => R(C, t(d))), u(I, m);
    };
    ee(k, (I) => {
      t(d) && I(_);
    });
  }
  O(() => R(b, o.clients.delete1)), u(M, w), ae();
}
var qr = q('<div><div class="flex"><input type="range" class="svelte-12j8dit"> <div class="value font-mono svelte-12j8dit"> </div></div> <div class="label svelte-12j8dit"><label class="font-label noselect svelte-12j8dit"> </label></div></div>');
function qe(M, e) {
  re(e, true);
  let r = Z(e, "value", 15), o = Z(e, "label", 3, ""), d = Z(e, "disabled", 3, false), f = Z(e, "step", 3, 1), w = Z(e, "widthRange", 3, "15rem");
  const h = be();
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
  var y = qr(), k = v(y), _ = v(k);
  Je(_), F(_, "id", h);
  var I = a(_, 2), m = v(I, true);
  c(I), c(k);
  var C = a(k, 2), P = v(C);
  F(P, "for", h);
  var L = v(P, true);
  c(P), c(C), c(y), O(() => {
    F(_, "name", e.name), F(_, "title", o()), F(_, "aria-label", o()), _.disabled = d(), F(_, "aria-disabled", d()), F(_, "min", e.min), F(_, "max", e.max), F(_, "step", f()), W(_, "width", w()), W(_, "--bg-slider", t(b)), R(m, r()), R(L, o());
  }), Qe(_, r), u(M, y), ae();
}
var jr = q('<div class="outer svelte-1evkwuy"><div class="container svelte-1evkwuy"><div><label class="font-label svelte-1evkwuy"> </label> <div class="values svelte-1evkwuy"><!> <!> <!></div></div> <div class="color svelte-1evkwuy"></div></div></div>');
function me(M, e) {
  re(e, true);
  let r = Z(e, "h", 15), o = Z(e, "s", 15), d = Z(e, "l", 15);
  const f = be(), w = "15rem";
  let h = he(() => `hsl(${r()} ${o()} ${d()})`);
  var b = jr(), y = v(b), k = v(y), _ = v(k);
  F(_, "for", f);
  var I = v(_, true);
  c(_);
  var m = a(_, 2);
  F(m, "id", f);
  var C = v(m);
  qe(C, { label: "Hue", min: 0, max: 359, widthRange: w, bgMode: "hue", get hue() {
    return r();
  }, get sat() {
    return o();
  }, get lum() {
    return d();
  }, get value() {
    return r();
  }, set value(U) {
    r(U);
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
  }, set value(U) {
    o(U);
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
  }, set value(U) {
    d(U);
  } }), c(m), c(k);
  var B = a(k, 2);
  c(y), c(b), O(() => {
    W(y, "border-color", t(h)), R(I, e.label), W(B, "background", t(h));
  }), u(M, b), ae();
}
var Fr = (M, e) => {
  var _a;
  return (_a = t(e)) == null ? void 0 : _a.click();
}, zr = q('<div class="container svelte-2nx18n"><!> <div role="none" class="mask svelte-2nx18n"><div class="colorWrapper svelte-2nx18n"><div class="color svelte-2nx18n"></div></div> <input type="color" class="svelte-2nx18n"></div></div>');
function je(M, e) {
  re(e, true);
  let r = Z(e, "value", 15), o = ge(), d = x(void 0);
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
  var h = a(w, 2);
  h.__click = [Fr, d];
  var b = a(v(h), 2);
  Je(b), Mt(b, (y) => n(d, y), () => t(d)), c(h), c(f), O(() => W(f, "--color", r())), Qe(b, r), u(M, f), ae();
}
Ke(["click"]);
var Hr = q('<div><p> </p> <div class="hsl svelte-10udpph"><div><!> <!> <!> <!></div> <div><!> <!> <!></div></div> <div><p><!></p> <!> <!> <!></div></div>');
function It(M, e) {
  re(e, true);
  let r = Z(e, "values", 15), o = ge();
  var d = Hr(), f = v(d), w = v(f, true);
  c(f);
  var h = a(f, 2), b = v(h), y = v(b);
  me(y, { label: "--text", get h() {
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
  me(k, { label: "--text-high", get h() {
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
  var _ = a(k, 2);
  me(_, { label: "--bg", get h() {
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
  var I = a(_, 2);
  me(I, { label: "--bg-high", get h() {
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
  var m = a(b, 2), C = v(m);
  me(C, { label: "--action", get h() {
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
  me(P, { label: "--accent", get h() {
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
  me(L, { label: "--error", get h() {
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
  } }), c(m), c(h);
  var B = a(h, 2), U = v(B), z = v(U);
  we(z, () => o.clients.branding.descFullCss), c(U);
  var Q = a(U, 2);
  je(Q, { label: "--btn-text", get value() {
    return r().btn_text;
  }, set value(l) {
    r(r().btn_text = l, true);
  } });
  var A = a(Q, 2);
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
  } }), c(B), c(d), O(() => R(w, o.clients.branding.descHsl)), u(M, d), ae();
}
var Kr = q('<img alt="Client Logo" width="100%" height="100%">'), Vr = (M) => M.preventDefault(), Wr = q('<div aria-label="Preview: All components inside are only for theme and colors preview and have no effect or interaction"><div class="inner svelte-fkmn75"><div class="container svelte-fkmn75"><div class="logo svelte-fkmn75"><!></div> <h3 class="svelte-fkmn75">Header</h3> <p>Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam</p> <p><span class="svelte-fkmn75">--accent-color</span></p> <!> <div class="btn svelte-fkmn75"><div class="svelte-fkmn75"><!> <br> <!> <br> <!> <br></div> <div class="svelte-fkmn75"><!> <br> <!> <br> <!></div></div> <a>Link</a> <br> <!></div></div></div>');
function Rt(M, e) {
  re(e, true);
  let r = x(false), o = x(false);
  _e(() => {
    e.logoUrl && n(o, false);
  }), _e(() => {
    t(r) && setTimeout(() => {
      n(r, false);
    }, 2e3);
  });
  var d = Wr(), f = v(d), w = v(f), h = v(w), b = v(h);
  {
    var y = (l) => {
      hr(l, { width: "100%", get show() {
        return e.typ;
      } });
    }, k = (l) => {
      var g = Kr();
      O(() => F(g, "src", e.logoUrl)), Fe("error", g, () => n(o, true)), Zt(g), u(l, g);
    };
    ee(b, (l) => {
      t(o) ? l(y) : l(k, false);
    });
  }
  c(h);
  var _ = a(h, 8);
  fe(_, { label: "Preview", placeholder: "Preview", width: "12.5rem" });
  var I = a(_, 2), m = v(I), C = v(m);
  se(C, { level: 1, get isLoading() {
    return t(r);
  }, onclick: () => n(r, true), children: (l, g) => {
    V();
    var E = J("Button");
    u(l, E);
  }, $$slots: { default: true } });
  var P = a(C, 4);
  se(P, { level: 2, get isLoading() {
    return t(r);
  }, onclick: () => n(r, true), children: (l, g) => {
    V();
    var E = J("Button");
    u(l, E);
  }, $$slots: { default: true } });
  var L = a(P, 4);
  se(L, { level: 3, get isLoading() {
    return t(r);
  }, onclick: () => n(r, true), children: (l, g) => {
    V();
    var E = J("Button");
    u(l, E);
  }, $$slots: { default: true } }), V(2), c(m);
  var B = a(m, 2), U = v(B);
  se(U, { level: -1, get isLoading() {
    return t(r);
  }, onclick: () => n(r, true), children: (l, g) => {
    V();
    var E = J("Button");
    u(l, E);
  }, $$slots: { default: true } });
  var z = a(U, 4);
  se(z, { level: -2, get isLoading() {
    return t(r);
  }, onclick: () => n(r, true), children: (l, g) => {
    V();
    var E = J("Button");
    u(l, E);
  }, $$slots: { default: true } });
  var Q = a(z, 4);
  se(Q, { level: -3, get isLoading() {
    return t(r);
  }, onclick: () => n(r, true), children: (l, g) => {
    V();
    var E = J("Button");
    u(l, E);
  }, $$slots: { default: true } }), c(B), c(I);
  var A = a(I, 2);
  F(A, "href", window.location.href), A.__click = [Vr];
  var D = a(A, 4);
  fr(D, {}), c(w), c(f), c(d), O((l, g, E, S, p, H, K) => {
    W(d, "--border-radius", e.borderRadius), W(d, "--text", l), W(d, "--text-high", g), W(d, "--bg", E), W(d, "--bg-high", S), W(d, "--action", p), W(d, "--accent", H), W(d, "--error", K), W(d, "--btn-text", e.theme.btn_text), W(d, "--theme-sun", e.theme.theme_sun), W(d, "--theme-moon", e.theme.theme_moon);
  }, [() => e.theme.text.join(" "), () => e.theme.text_high.join(" "), () => e.theme.bg.join(" "), () => e.theme.bg_high.join(" "), () => e.theme.action.join(" "), () => e.theme.accent.join(" "), () => e.theme.error.join(" ")]), u(M, d), ae();
}
Ke(["click"]);
var Gr = q('<div><h2>Preview</h2> <div class="tabs svelte-1klemk8"><!></div> <div class="preview svelte-1klemk8"><!></div></div>');
function Jr(M, e) {
  re(e, true);
  let r = ["Light Theme", "Dark Theme"], o = x(i(r[0]));
  var d = Gr(), f = a(v(d), 2), w = v(f);
  Ut(w, { tabs: r, center: true, get selected() {
    return t(o);
  }, set selected(_) {
    n(o, i(_));
  } }), c(f);
  var h = a(f, 2), b = v(h);
  {
    var y = (_) => {
      Rt(_, { get logoUrl() {
        return e.logoUrl;
      }, get borderRadius() {
        return e.theme.border_radius;
      }, get theme() {
        return e.theme.light;
      }, typ: "light" });
    }, k = (_) => {
      Rt(_, { get logoUrl() {
        return e.logoUrl;
      }, get borderRadius() {
        return e.theme.border_radius;
      }, get theme() {
        return e.theme.dark;
      }, typ: "dark" });
    };
    ee(b, (_) => {
      t(o) === r[0] ? _(y) : _(k, false);
    });
  }
  c(h), c(d), u(M, d), ae();
}
var Qr = q('<div class="err"> </div>'), Xr = q('<div class="values svelte-1pkqupw"><p><!></p> <!> <h1>Light Theme</h1> <!> <br> <h1>Dark Theme</h1> <!></div> <div class="preview"><!> <hr> <p>Logo Upload</p> <!> <div class="buttons svelte-1pkqupw"><!> <!> <!></div> <!></div>', 1), Yr = q('<div class="container svelte-1pkqupw"><!></div>');
function Zr(M, e) {
  re(e, true);
  const r = "13rem";
  let o = xe(), d = ge(), f = x(false), w = x(""), h = x(void 0), b = x(i(be())), y = he(() => `/auth/v1/clients/${e.client.id}/logo?${t(b)}`), k = he(() => `/auth/v1/theme/${e.client.id}`);
  _e(() => {
    _();
  });
  async function _() {
    var _a;
    let P = await Ve(t(k));
    P.body ? (P.body.client_id === "rauthy" && console.log("using Rauthy default scheme"), n(h, i(P.body))) : n(w, i(((_a = P.error) == null ? void 0 : _a.message) || "Error"));
  }
  async function I(P, L) {
    if (!t(h)) {
      console.error("theme is undefined");
      return;
    }
    let B = t(h);
    B.client_id = e.client.id;
    let U = await We(t(k), B);
    U.error ? n(w, i(U.error.message)) : (n(f, true), setTimeout(() => {
      n(f, false);
    }, 2e3));
  }
  async function m() {
    let P = await He(t(k));
    P.error ? n(w, i(P.error.message)) : (await He(`/auth/v1/clients/${e.client.id}/logo`), await _(), n(b, i(be())), n(f, true), setTimeout(() => {
      n(f, false);
    }, 2e3));
  }
  async function C() {
    n(b, i(be()));
  }
  Ge(M, { get action() {
    return t(k);
  }, onSubmit: I, children: (P, L) => {
    var B = Yr(), U = v(B);
    {
      var z = (Q) => {
        var A = Xr(), D = te(A), l = v(D), g = v(l);
        we(g, () => d.clients.branding.descVariables), c(l);
        var E = a(l, 2);
        fe(E, { label: "--border-radius", placeholder: "--border-radius", get errMsg() {
          return d.validation.css;
        }, width: r, required: true, pattern: Et, get value() {
          return t(h).border_radius;
        }, set value(j) {
          t(h).border_radius = j;
        } });
        var S = a(E, 4);
        It(S, { get values() {
          return t(h).light;
        }, set values(j) {
          t(h).light = j;
        } });
        var p = a(S, 6);
        It(p, { get values() {
          return t(h).dark;
        }, set values(j) {
          t(h).dark = j;
        } }), c(D);
        var H = a(D, 2), K = v(H);
        ur(K, () => t(b), (j) => {
          Jr(j, { get logoUrl() {
            return t(y);
          }, get theme() {
            return t(h);
          } });
        });
        var G = a(K, 6);
        const le = he(() => `/auth/v1/clients/${e.client.id}/logo`);
        _r(G, { method: "PUT", get url() {
          return t(le);
        }, fileName: "logo", onSuccess: C });
        var $ = a(G, 2), N = v($);
        se(N, { type: "submit", children: (j, ne) => {
          V();
          var ce = J();
          O(() => R(ce, o.common.save)), u(j, ce);
        }, $$slots: { default: true } });
        var X = a(N, 2);
        se(X, { level: -2, onclick: m, children: (j, ne) => {
          V();
          var ce = J();
          O(() => R(ce, d.common.reset)), u(j, ce);
        }, $$slots: { default: true } });
        var ue = a(X, 2);
        {
          var oe = (j) => {
            Nt(j, {});
          };
          ee(ue, (j) => {
            t(f) && j(oe);
          });
        }
        c($);
        var de = a($, 2);
        {
          var ie = (j) => {
            var ne = Qr(), ce = v(ne, true);
            c(ne), O(() => R(ce, t(w))), u(j, ne);
          };
          ee(de, (j) => {
            t(w) && j(ie);
          });
        }
        c(H), u(Q, A);
      };
      ee(U, (Q) => {
        t(h) && Q(z);
      });
    }
    c(B), u(P, B);
  }, $$slots: { default: true } }), ae();
}
var $r = q('<div class="flex"><!></div> <div class="details"><!></div>', 1);
function ea(M, e) {
  re(e, true);
  let r = xe(), o = ge();
  const d = [o.nav.config, "Secret", "Branding", r.common.delete];
  let f = x(i(d[0])), w = x(void 0);
  _e(() => {
    e.client.id && requestAnimationFrame(() => {
      var _a;
      (_a = t(w)) == null ? void 0 : _a();
    });
  });
  var h = $r(), b = te(h), y = v(b);
  Ut(y, { tabs: d, get selected() {
    return t(f);
  }, set selected(C) {
    n(f, i(C));
  }, get focusFirst() {
    return t(w);
  }, set focusFirst(C) {
    n(w, i(C));
  } }), c(b);
  var k = a(b, 2), _ = v(k);
  {
    var I = (C) => {
      Er(C, { get client() {
        return e.client;
      }, get clients() {
        return e.clients;
      }, get scopesAll() {
        return e.scopesAll;
      }, get onSave() {
        return e.onSave;
      } });
    }, m = (C) => {
      var P = Pe(), L = te(P);
      {
        var B = (z) => {
          Nr(z, { get client() {
            return e.client;
          } });
        }, U = (z) => {
          var Q = Pe(), A = te(Q);
          {
            var D = (g) => {
              Zr(g, { get client() {
                return e.client;
              } });
            }, l = (g) => {
              var E = Pe(), S = te(E);
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
              u(g, E);
            };
            ee(A, (g) => {
              t(f) === "Branding" ? g(D) : g(l, false);
            }, true);
          }
          u(z, Q);
        };
        ee(L, (z) => {
          t(f) === "Secret" ? z(B) : z(U, false);
        }, true);
      }
      u(C, P);
    };
    ee(_, (C) => {
      t(f) === o.nav.config ? C(I) : C(m, false);
    });
  }
  c(k), u(M, h), ae();
}
var ta = q("<div></div> <!>", 1), ra = q("<!> <!>", 1), aa = q('<div class="err"> </div>'), la = q('<!> <div id="groups"><!></div>', 1), ia = q("<!> <!>", 1);
function na(M, e) {
  re(e, true);
  let r = x(void 0), o = x(""), d = x(i([])), f = x(i([])), w = x(void 0), h = nr("cid"), b = x(i([]));
  const y = ["ID"];
  let k = x(i(y[0])), _ = x("");
  const I = ["ID"];
  Yt(() => {
    m(), C();
  }), _e(() => {
    n(w, i(t(d).find((A) => A.id === h.get())));
  }), _e(() => {
    let A = t(_).toLowerCase();
    A ? t(k) === y[0] && n(f, i(t(d).filter((D) => D.id.toLowerCase().includes(A)))) : n(f, i(t(d)));
  });
  async function m() {
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
    A === I[0] && t(d).sort((g, E) => l ? g.id.localeCompare(E.id) : E.id.localeCompare(g.id));
  }
  function L() {
    m();
  }
  async function B(A) {
    var _a;
    (_a = t(r)) == null ? void 0 : _a(), await m(), h.set(A);
  }
  var U = ia(), z = te(U);
  lr(z, { paddingTop: "2.1rem", buttonTilesAriaControls: "groups", width: "min(20rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (D) => {
    var l = ta(), g = te(l);
    W(g, "height", ".5rem");
    var E = a(g, 2);
    ze(E, 17, () => t(f), (S) => S.id, (S, p, H, K) => {
      const G = he(() => h.get() === t(p).id);
      ar(S, { onclick: () => h.set(t(p).id), get selected() {
        return t(G);
      }, children: (le, $) => {
        V();
        var N = J();
        O(() => R(N, t(p).id)), u(le, N);
      } });
    }), u(D, l);
  }, children: (D, l) => {
    var g = ra(), E = te(g);
    const S = he(() => t(d).length === 0 ? 1 : 2);
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
    ir(p, { searchOptions: y, orderOptions: I, onChangeOrder: P, searchWidth: "min(19.5rem, 100dvw - .5rem)", get searchOption() {
      return t(k);
    }, set searchOption(H) {
      n(k, i(H));
    }, get value() {
      return t(_);
    }, set value(H) {
      n(_, i(H));
    } }), u(D, g);
  }, $$slots: { buttonTiles: true, default: true } });
  var Q = a(z, 2);
  rr(Q, { children: (A, D) => {
    var l = la(), g = te(l);
    {
      var E = (K) => {
        var G = aa(), le = v(G, true);
        c(G), O(() => R(le, t(o))), u(K, G);
      };
      ee(g, (K) => {
        t(o) && K(E);
      });
    }
    var S = a(g, 2), p = v(S);
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
  } }), u(M, U), ae();
}
function Va(M) {
  na(M, {});
}
export {
  Va as component
};
