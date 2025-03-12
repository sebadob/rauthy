import { t as O, a as u, d as Xt, e as J } from "../chunks/DKC5GJ29.js";
import { p as ne, c, s as a, k as C, t as D, l as i, j as t, a as se, r as d, a9 as fe, f as ve, aa as he, a8 as W, a6 as Yt } from "../chunks/BveBAmlr.js";
import { d as He, e as je, s as R, b as Zt } from "../chunks/CYCba2oX.js";
import { i as $ } from "../chunks/D-uYoVwt.js";
import { e as Fe, i as Lt } from "../chunks/BLE4FKaJ.js";
import { r as Ke, a as re, s as H } from "../chunks/Dql74IOz.js";
import { p as l } from "../chunks/VbPNpVtZ.js";
import { B as $t } from "../chunks/B1rQzxl_.js";
import { C as er } from "../chunks/CqiL-eHy.js";
import { N as tr } from "../chunks/pmj7Ha_a.js";
import { N as rr } from "../chunks/CPmtaWoD.js";
import { O as ar } from "../chunks/CPgfHyhH.js";
import { b as Ve, d as We, c as ze, f as St } from "../chunks/BO1A6s0c.js";
import { u as lr } from "../chunks/Bvh621KL.js";
import { h as Ce } from "../chunks/CJj8kriY.js";
import { B as oe, t as ir, s as nr } from "../chunks/DlLEcmNg.js";
import { I as ue } from "../chunks/CCQyB9gY.js";
import { n as sr, g as pt, a as Ne, o as or, p as dr, q as Et } from "../chunks/BRCxk8by.js";
import { F as Ge } from "../chunks/BguaNybM.js";
import { u as we } from "../chunks/8R5My_LO.js";
import { u as ge } from "../chunks/DtT3Jahq.js";
import { I as ce } from "../chunks/4YTb0aaH.js";
import { b as Je } from "../chunks/Ds33nkO-.js";
import { b as Mt } from "../chunks/DLmJ2nn8.js";
import { p as Y } from "../chunks/Db0ChEdV.js";
import { g as ke } from "../chunks/B21bTIl7.js";
import { I as cr } from "../chunks/B7o5HlxY.js";
import { T as Ut } from "../chunks/BGiCnx1L.js";
import { I as Nt } from "../chunks/Cxysd5TC.js";
import { L as Be } from "../chunks/FI26onRO.js";
import { S as At } from "../chunks/CKIfa-j4.js";
import { O as Tt } from "../chunks/DdLeAElE.js";
import { I as vr } from "../chunks/CUO8Plwh.js";
import { k as ur } from "../chunks/BPEbum0P.js";
import { T as fr } from "../chunks/BFgWaswG.js";
import { R as hr } from "../chunks/CE9vlfyw.js";
import { I as _r } from "../chunks/BpjUF1Hv.js";
function mr(U, e, r, s) {
  var _a;
  e(), (_a = r.onInput) == null ? void 0 : _a.call(r, t(s));
}
function gr(U, e, r) {
  var _a;
  switch (U.code) {
    case "Enter":
      U.preventDefault(), e(), (_a = r.onEnter) == null ? void 0 : _a.call(r);
      break;
  }
}
function br(U, e) {
  var _a, _b;
  (_a = e()) == null ? void 0 : _a.focus(), (_b = e()) == null ? void 0 : _b.scrollIntoView({ behavior: "smooth", block: "center" });
}
var kr = O('<li class="value svelte-r5y8pk"><div class="label"> </div> <div class="btnClose svelte-r5y8pk"><!></div></li>'), xr = O("<option></option>"), wr = O('<datalist class="absolute svelte-r5y8pk"></datalist>'), yr = O('<span class="err"> </span>'), Cr = O('<div role="none" class="container svelte-r5y8pk"><ul class="svelte-r5y8pk"><!> <li class="svelte-r5y8pk"><input autocomplete="off" class="svelte-r5y8pk"> <!></li></ul> <label class="font-label noselect svelte-r5y8pk"> <!></label></div>');
function xe(U, e) {
  ne(e, true);
  let r = Y(e, "ref", 15), s = Y(e, "typ", 3, "text"), v = Y(e, "id", 19, ke), f = Y(e, "values", 31, () => l([])), P = Y(e, "label", 3, ""), h = Y(e, "placeholder", 3, ""), b = Y(e, "disabled", 3, false), L = Y(e, "required", 3, false), g = Y(e, "isError", 15, false), m = Y(e, "width", 3, "inherit"), A = Y(e, "maxHeightList", 3, "inherit"), w = we();
  const y = ke();
  let S = C(""), k = fe(() => e.datalist && e.datalist.length > 0 ? y : void 0);
  function B(z) {
    f(f().filter((Z) => Z !== z));
  }
  function N(z) {
    var _a;
    p(), (_a = e.onBlur) == null ? void 0 : _a.call(e), F();
  }
  function V(z) {
    z.preventDefault(), g(true);
  }
  function F() {
    t(S) && p() && (f().push(t(S)), i(S, ""));
  }
  function p() {
    var _a;
    let z = (_a = r()) == null ? void 0 : _a.validity;
    return z ? (g(!z.valid), z.valid) : (g(false), true);
  }
  var I = Cr();
  I.__click = [br, r];
  let o;
  var _ = c(I);
  let M;
  var q = c(_);
  Fe(q, 17, f, Lt, (z, Z, _e, de) => {
    var j = kr(), le = c(j), ee = c(le, true);
    d(le);
    var me = a(le, 2), Pe = c(me);
    oe(Pe, { invisible: true, onclick: () => B(t(Z)), children: (ye, Qe) => {
      cr(ye, { width: "1.2rem" });
    }, $$slots: { default: true } }), d(me), d(j), D(() => R(ee, t(Z))), u(z, j);
  });
  var E = a(q, 2), x = c(E);
  Ke(x), x.__keydown = [gr, F, e], x.__input = [mr, p, e, S], Mt(x, (z) => r(z), () => r());
  var G = a(x, 2);
  {
    var K = (z) => {
      var Z = wr();
      H(Z, "id", y), Fe(Z, 21, () => e.datalist, Lt, (_e, de, j, le) => {
        var ee = xr(), me = {};
        D(() => {
          me !== (me = t(de)) && (ee.value = (ee.__value = t(de)) == null ? "" : t(de));
        }), u(_e, ee);
      }), d(Z), u(z, Z);
    };
    $(G, (z) => {
      e.datalist && e.datalist.length > 1 && z(K);
    });
  }
  d(E), d(_);
  var ae = a(_, 2), X = c(ae), ie = a(X);
  {
    var te = (z) => {
      var Z = Xt(), _e = ve(Z);
      {
        var de = (j) => {
          var le = yr(), ee = c(le, true);
          d(le), D(() => R(ee, e.errMsg || w.common.invalidInput)), u(j, le);
        };
        $(_e, (j) => {
          g() && j(de);
        });
      }
      u(z, Z);
    };
    $(ie, (z) => {
      g() && z(te);
    });
  }
  d(ae), d(I), D(() => {
    o = re(I, "", o, { width: m() }), M = re(_, "", M, { "max-height": A() }), H(x, "type", s()), H(x, "id", v()), H(x, "name", e.name), H(x, "list", t(k)), H(x, "title", e.errMsg), H(x, "aria-label", P() || h()), H(x, "placeholder", h()), H(x, "aria-placeholder", h()), x.disabled = b(), H(x, "aria-disabled", b()), x.required = L() && f().length < 1, H(x, "aria-required", L() && f().length < 1), H(x, "maxlength", e.maxLength || void 0), H(x, "min", e.min || void 0), H(x, "max", e.max || void 0), H(x, "pattern", e.pattern), H(ae, "for", v()), H(ae, "data-required", L()), R(X, `${P() ?? ""} `);
  }), je("invalid", x, V), je("blur", x, N), Je(x, () => t(S), (z) => i(S, z)), u(U, I), se();
}
He(["click", "keydown", "input"]);
var Pr = O('<div class="err"> </div>'), Lr = O('<!> <!> <!> <p class="svelte-15tp6i6"><!></p> <!> <!> <!> <!>', 1), Sr = O('<div class="container svelte-15tp6i6"><!></div>');
function Ar(U, e) {
  ne(e, true);
  let r = we(), s = ge(), v = C(void 0), f = C(""), P = C(""), h = C(""), b = C(true), L = C(l([])), g = C(l([]));
  he(() => {
    requestAnimationFrame(() => {
      var _a;
      (_a = t(v)) == null ? void 0 : _a.focus();
    });
  });
  async function m(y, S) {
    var _a;
    if (e.clients.find((N) => N.id === t(P))) {
      i(f, l(s.common.nameExistsAlready));
      return;
    }
    i(f, "");
    let k = { id: t(P), name: t(h) || void 0, confidential: t(b), redirect_uris: t(L), post_logout_redirect_uris: t(g).length > 0 ? t(g) : void 0 }, B = await Ve(y.action, k);
    B.body ? e.onSave(B.body.id) : i(f, l(((_a = B.error) == null ? void 0 : _a.message) || "Error"));
  }
  var A = Sr(), w = c(A);
  Ge(w, { action: "/auth/v1/clients", onSubmit: m, children: (y, S) => {
    var k = Lr(), B = ve(k);
    ue(B, { autocomplete: "off", label: "Client ID", placeholder: "Client ID", required: true, pattern: sr, get ref() {
      return t(v);
    }, set ref(E) {
      i(v, l(E));
    }, get value() {
      return t(P);
    }, set value(E) {
      i(P, l(E));
    } });
    var N = a(B, 2);
    ue(N, { autocomplete: "off", get label() {
      return s.clients.name;
    }, get placeholder() {
      return s.clients.name;
    }, pattern: pt, get value() {
      return t(h);
    }, set value(E) {
      i(h, l(E));
    } });
    var V = a(N, 2);
    ce(V, { get ariaLabel() {
      return s.clients.confidential;
    }, get checked() {
      return t(b);
    }, set checked(E) {
      i(b, l(E));
    }, children: (E, x) => {
      W();
      var G = J();
      D(() => R(G, s.clients.confidential)), u(E, G);
    }, $$slots: { default: true } });
    var F = a(V, 2), p = c(F);
    Ce(p, () => s.clients.descUri), d(F);
    var I = a(F, 2);
    xe(I, { typ: "url", label: "Redirect URIs", get errMsg() {
      return s.validation.uri;
    }, get values() {
      return t(L);
    }, set values(E) {
      i(L, l(E));
    } });
    var o = a(I, 2);
    xe(o, { typ: "url", label: "Post Logout Redirect URIs", get errMsg() {
      return s.validation.uri;
    }, get values() {
      return t(g);
    }, set values(E) {
      i(g, l(E));
    } });
    var _ = a(o, 2);
    oe(_, { type: "submit", children: (E, x) => {
      W();
      var G = J();
      D(() => R(G, r.common.save)), u(E, G);
    }, $$slots: { default: true } });
    var M = a(_, 2);
    {
      var q = (E) => {
        var x = Pr(), G = c(x, true);
        d(x), D(() => R(G, t(f))), u(E, x);
      };
      $(M, (E) => {
        t(f) && E(q);
      });
    }
    u(y, k);
  }, $$slots: { default: true } }), d(A), u(U, A), se();
}
const De = "urn:ietf:params:oauth:grant-type:device_code";
var Tr = O('<div class="err"> </div>'), Ir = O('<div class="err"> </div>'), Rr = O('<h5> </h5> <!> <p class="desc svelte-5tvvvt"> </p> <!> <p class="desc svelte-5tvvvt"> </p> <!> <!> <div></div> <h5> </h5> <!> <!> <!> <p class="mb-0 svelte-5tvvvt">Authentication Flows</p> <!> <!> <!> <!> <!> <div></div> <p class="desc svelte-5tvvvt"> </p> <p class="desc svelte-5tvvvt"><strong> </strong></p> <!> <!> <!> <div></div> <p class="desc svelte-5tvvvt"> </p> <!> <p class="desc svelte-5tvvvt"><!></p> <!> <!> <div></div> <p class="desc svelte-5tvvvt"><!></p> <!> <!> <div></div> <p> </p> <!> <div></div> <p> </p> <p> </p> <!> <!> <div></div> <p> </p> <!> <div class="flex gap-05"><!> <!></div> <!> <div></div>', 1), pr = O('<div class="container svelte-5tvvvt"><!></div>');
function Er(U, e) {
  var _a, _b;
  ne(e, true);
  let r = we(), s = ge();
  const v = "min(20rem, calc(100dvw - .5rem))";
  let f = C(""), P = C(false), h = C(l(e.client.id)), b = C(l(e.client.name || "")), L = C(l(e.client.enabled)), g = C(l(e.client.confidential)), m = C(l(e.client.client_uri || "")), A = C(l(e.client.contacts ? Array.from(e.client.contacts) : [])), w = C(l(e.client.allowed_origins ? Array.from(e.client.allowed_origins) : [])), y = C(l(Array.from(e.client.redirect_uris))), S = C(l(e.client.post_logout_redirect_uris ? Array.from(e.client.post_logout_redirect_uris) : [])), k = l({ authorizationCode: e.client.flows_enabled.includes("authorization_code"), clientCredentials: e.client.flows_enabled.includes("client_credentials"), password: e.client.flows_enabled.includes("password"), refreshToken: e.client.flows_enabled.includes("refresh_token"), deviceCode: e.client.flows_enabled.includes(De) });
  const B = ["RS256", "RS384", "RS512", "EdDSA"];
  let N = C(l(e.client.access_token_alg)), V = C(l(e.client.id_token_alg)), F = C(l(e.client.access_token_lifetime.toString())), p = C(l(e.client.auth_code_lifetime.toString())), I = C(l(e.scopesAll.map((K) => ({ name: K, selected: e.client.scopes.includes(K) || false })))), o = C(l(e.scopesAll.map((K) => ({ name: K, selected: e.client.default_scopes.includes(K) || false })))), _ = l({ plain: ((_a = e.client.challenges) == null ? void 0 : _a.includes("plain")) || false, s256: ((_b = e.client.challenges) == null ? void 0 : _b.includes("S256")) || false }), M = C(l(e.client.force_mfa));
  he(() => {
    var _a2, _b2;
    e.client.id && (i(h, l(e.client.id)), i(b, l(e.client.name || "")), i(L, l(e.client.enabled)), i(g, l(e.client.confidential)), i(m, l(e.client.client_uri || "")), i(A, l(e.client.contacts ? Array.from(e.client.contacts) : [])), i(w, l(e.client.allowed_origins ? Array.from(e.client.allowed_origins) : [])), i(y, l(Array.from(e.client.redirect_uris))), i(S, l(e.client.post_logout_redirect_uris ? Array.from(e.client.post_logout_redirect_uris) : [])), k.authorizationCode = e.client.flows_enabled.includes("authorization_code"), k.clientCredentials = e.client.flows_enabled.includes("client_credentials"), k.password = e.client.flows_enabled.includes("password"), k.refreshToken = e.client.flows_enabled.includes("refresh_token"), k.deviceCode = e.client.flows_enabled.includes(De), i(N, l(e.client.access_token_alg)), i(V, l(e.client.id_token_alg)), i(F, l(e.client.access_token_lifetime.toString())), i(p, l(e.client.auth_code_lifetime.toString())), i(I, l(e.scopesAll.map((K) => ({ name: K, selected: e.client.scopes.includes(K) || false })))), i(o, l(e.scopesAll.map((K) => ({ name: K, selected: e.client.default_scopes.includes(K) || false })))), _.plain = ((_a2 = e.client.challenges) == null ? void 0 : _a2.includes("plain")) || false, _.s256 = ((_b2 = e.client.challenges) == null ? void 0 : _b2.includes("S256")) || false);
  });
  async function q(K, ae) {
    i(f, "");
    let X = { id: t(h), name: t(b) || void 0, enabled: t(L), confidential: t(g), allowed_origins: t(w).length > 0 ? t(w) : void 0, redirect_uris: t(y), post_logout_redirect_uris: t(S).length > 0 ? t(S) : void 0, flows_enabled: [], access_token_alg: t(N), id_token_alg: t(V), access_token_lifetime: Number.parseInt(t(F)), auth_code_lifetime: Number.parseInt(t(p)), scopes: t(I).filter((te) => te.selected).map((te) => te.name), default_scopes: t(o).filter((te) => te.selected).map((te) => te.name), challenges: void 0, force_mfa: t(M), client_uri: t(m) || void 0, contacts: t(A).length > 0 ? t(A) : void 0 };
    k.authorizationCode && X.flows_enabled.push("authorization_code"), k.clientCredentials && X.flows_enabled.push("client_credentials"), k.password && X.flows_enabled.push("password"), k.refreshToken && X.flows_enabled.push("refresh_token"), k.deviceCode && X.flows_enabled.push(De), _.plain && (X.challenges = ["plain"]), _.s256 && (X.challenges ? X.challenges.push("S256") : X.challenges = ["S256"]);
    let ie = await We(K.action, X);
    ie.error ? i(f, l(ie.error.message)) : (i(P, true), e.onSave(), setTimeout(() => {
      i(P, false);
    }, 2e3));
  }
  var E = pr(), x = c(E);
  const G = fe(() => `/auth/v1/clients/${e.client.id}`);
  Ge(x, { get action() {
    return t(G);
  }, onSubmit: q, children: (K, ae) => {
    var X = Rr(), ie = ve(X), te = c(ie, true);
    d(ie);
    var z = a(ie, 2);
    Be(z, { label: "ID", mono: true, get copyToClip() {
      return e.client.id;
    }, children: (n, Q) => {
      W();
      var T = J();
      D(() => R(T, e.client.id)), u(n, T);
    }, $$slots: { default: true } });
    var Z = a(z, 2), _e = c(Z, true);
    d(Z);
    var de = a(Z, 2);
    ue(de, { autocomplete: "off", get label() {
      return s.clients.name;
    }, get placeholder() {
      return s.clients.name;
    }, width: v, pattern: pt, get value() {
      return t(b);
    }, set value(n) {
      i(b, l(n));
    } });
    var j = a(de, 2), le = c(j, true);
    d(j);
    var ee = a(j, 2);
    ue(ee, { typ: "url", autocomplete: "off", label: "URI", placeholder: "URI", width: v, pattern: Ne, get value() {
      return t(m);
    }, set value(n) {
      i(m, l(n));
    } });
    var me = a(ee, 2);
    xe(me, { get label() {
      return s.common.contact;
    }, pattern: or, get values() {
      return t(A);
    }, set values(n) {
      i(A, l(n));
    } });
    var Pe = a(me, 2);
    re(Pe, "", {}, { height: ".5rem" });
    var ye = a(Pe, 2), Qe = c(ye, true);
    d(ye);
    var Xe = a(ye, 2);
    ce(Xe, { get ariaLabel() {
      return s.common.enabled;
    }, get checked() {
      return t(L);
    }, set checked(n) {
      i(L, l(n));
    }, children: (n, Q) => {
      W();
      var T = J();
      D(() => R(T, s.common.enabled)), u(n, T);
    }, $$slots: { default: true } });
    var Ye = a(Xe, 2);
    ce(Ye, { get ariaLabel() {
      return s.clients.confidential;
    }, get checked() {
      return t(g);
    }, set checked(n) {
      i(g, l(n));
    }, children: (n, Q) => {
      W();
      var T = J();
      D(() => R(T, s.clients.confidential)), u(n, T);
    }, $$slots: { default: true } });
    var Ze = a(Ye, 2);
    ce(Ze, { get ariaLabel() {
      return s.clients.forceMfa;
    }, get checked() {
      return t(M);
    }, set checked(n) {
      i(M, l(n));
    }, children: (n, Q) => {
      W();
      var T = J();
      D(() => R(T, s.clients.forceMfa)), u(n, T);
    }, $$slots: { default: true } });
    var $e = a(Ze, 4);
    ce($e, { ariaLabel: "authorization_code", get checked() {
      return k.authorizationCode;
    }, set checked(n) {
      k.authorizationCode = n;
    }, children: (n, Q) => {
      W();
      var T = J("authorization_code");
      u(n, T);
    }, $$slots: { default: true } });
    var et = a($e, 2);
    ce(et, { ariaLabel: "urn:ietf:params:oauth:grant-type:device_code", get checked() {
      return k.deviceCode;
    }, set checked(n) {
      k.deviceCode = n;
    }, children: (n, Q) => {
      W();
      var T = J("device_code");
      u(n, T);
    }, $$slots: { default: true } });
    var tt = a(et, 2);
    ce(tt, { ariaLabel: "client_credentials", get checked() {
      return k.clientCredentials;
    }, set checked(n) {
      k.clientCredentials = n;
    }, children: (n, Q) => {
      W();
      var T = J("client_credentials");
      u(n, T);
    }, $$slots: { default: true } });
    var rt = a(tt, 2);
    ce(rt, { ariaLabel: "password", get checked() {
      return k.password;
    }, set checked(n) {
      k.password = n;
    }, children: (n, Q) => {
      W();
      var T = J("password");
      u(n, T);
    }, $$slots: { default: true } });
    var at = a(rt, 2);
    ce(at, { ariaLabel: "refresh_token", get checked() {
      return k.refreshToken;
    }, set checked(n) {
      k.refreshToken = n;
    }, children: (n, Q) => {
      W();
      var T = J("refresh_token");
      u(n, T);
    }, $$slots: { default: true } });
    var lt = a(at, 2);
    re(lt, "", {}, { height: ".5rem" });
    var Se = a(lt, 2), Bt = c(Se, true);
    d(Se);
    var Ae = a(Se, 2), it = c(Ae), Dt = c(it, true);
    d(it), d(Ae);
    var nt = a(Ae, 2);
    ce(nt, { ariaLabel: "PKCE plain", get checked() {
      return _.plain;
    }, set checked(n) {
      _.plain = n;
    }, children: (n, Q) => {
      W();
      var T = J("plain");
      u(n, T);
    }, $$slots: { default: true } });
    var st = a(nt, 2);
    ce(st, { ariaLabel: "PKCE S256", get checked() {
      return _.s256;
    }, set checked(n) {
      _.s256 = n;
    }, children: (n, Q) => {
      W();
      var T = J("S256");
      u(n, T);
    }, $$slots: { default: true } });
    var ot = a(st, 2);
    {
      var Ot = (n) => {
        var Q = Tr(), T = c(Q, true);
        d(Q), D(() => R(T, s.clients.errConfidentialPKCE)), ir(3, Q, () => nr, () => ({ duration: 150 })), u(n, Q);
      };
      $(ot, (n) => {
        !t(g) && !_.plain && !_.s256 && n(Ot);
      });
    }
    var dt = a(ot, 2);
    re(dt, "", {}, { height: ".5rem" });
    var Te = a(dt, 2), qt = c(Te, true);
    d(Te);
    var ct = a(Te, 2);
    xe(ct, { typ: "url", label: "Allowed Origins", get errMsg() {
      return s.validation.origin;
    }, pattern: dr, get values() {
      return t(w);
    }, set values(n) {
      i(w, l(n));
    } });
    var Ie = a(ct, 2), jt = c(Ie);
    Ce(jt, () => s.clients.descUri), d(Ie);
    var vt = a(Ie, 2);
    xe(vt, { typ: "url", label: "Redirect URIs", get errMsg() {
      return s.validation.uri;
    }, get required() {
      return k.authorizationCode;
    }, pattern: Ne, get values() {
      return t(y);
    }, set values(n) {
      i(y, l(n));
    } });
    var ut = a(vt, 2);
    xe(ut, { typ: "url", label: "Post Logout Redirect URIs", get errMsg() {
      return s.validation.uri;
    }, pattern: Ne, get values() {
      return t(S);
    }, set values(n) {
      i(S, l(n));
    } });
    var ft = a(ut, 2);
    re(ft, "", {}, { height: ".5rem" });
    var Re = a(ft, 2), Ft = c(Re);
    Ce(Ft, () => s.clients.scopes.desc), d(Re);
    var ht = a(Re, 2);
    At(ht, { get items() {
      return t(I);
    }, set items(n) {
      i(I, l(n));
    }, children: (n, Q) => {
      W();
      var T = J();
      D(() => R(T, s.clients.scopes.allowed)), u(n, T);
    }, $$slots: { default: true } });
    var _t = a(ht, 2);
    At(_t, { get items() {
      return t(o);
    }, set items(n) {
      i(o, l(n));
    }, children: (n, Q) => {
      W();
      var T = J();
      D(() => R(T, s.clients.scopes.default)), u(n, T);
    }, $$slots: { default: true } });
    var mt = a(_t, 2);
    re(mt, "", {}, { height: ".75rem" });
    var pe = a(mt, 2), zt = c(pe, true);
    d(pe);
    var gt = a(pe, 2);
    ue(gt, { typ: "number", autocomplete: "off", label: "Token Lifetime", placeholder: "Token Lifetime", width: v, min: "10", max: "86400", errMsg: "10 <= Token Lifetime <= 86400", get value() {
      return t(F);
    }, set value(n) {
      i(F, l(n));
    } });
    var bt = a(gt, 2);
    re(bt, "", {}, { height: ".5rem" });
    var Ee = a(bt, 2), Ht = c(Ee, true);
    d(Ee);
    var Me = a(Ee, 2), Kt = c(Me, true);
    d(Me);
    var kt = a(Me, 2);
    Be(kt, { label: "Access Token Algorithm", children: (n, Q) => {
      Tt(n, { ariaLabel: "Access Token Algorithm", options: B, borderless: true, get value() {
        return t(N);
      }, set value(T) {
        i(N, l(T));
      } });
    }, $$slots: { default: true } });
    var xt = a(kt, 2);
    Be(xt, { label: "ID Token Algorithm", children: (n, Q) => {
      Tt(n, { ariaLabel: "ID Token Algorithm", options: B, borderless: true, get value() {
        return t(V);
      }, set value(T) {
        i(V, l(T));
      } });
    }, $$slots: { default: true } });
    var wt = a(xt, 2);
    re(wt, "", {}, { height: ".5rem" });
    var Ue = a(wt, 2), Vt = c(Ue, true);
    d(Ue);
    var yt = a(Ue, 2);
    ue(yt, { typ: "number", autocomplete: "off", label: "Auth Code Lifetime", placeholder: "Auth Code Lifetime", width: v, min: "10", max: "300", errMsg: "10 <= Auth Code Lifetime <= 300", get value() {
      return t(p);
    }, set value(n) {
      i(p, l(n));
    } });
    var Le = a(yt, 2);
    re(Le, "", {}, { "margin-top": "1rem" });
    var Ct = c(Le);
    oe(Ct, { type: "submit", children: (n, Q) => {
      W();
      var T = J();
      D(() => R(T, r.common.save)), u(n, T);
    }, $$slots: { default: true } });
    var Wt = a(Ct, 2);
    {
      var Gt = (n) => {
        Nt(n, {});
      };
      $(Wt, (n) => {
        t(P) && n(Gt);
      });
    }
    d(Le);
    var Pt = a(Le, 2);
    {
      var Jt = (n) => {
        var Q = Ir(), T = c(Q, true);
        d(Q), D(() => R(T, t(f))), u(n, Q);
      };
      $(Pt, (n) => {
        t(f) && n(Jt);
      });
    }
    var Qt = a(Pt, 2);
    re(Qt, "", {}, { height: "1rem" }), D(() => {
      R(te, s.common.information), R(_e, s.clients.descName), R(le, s.clients.descClientUri), R(Qe, s.clients.config), R(Bt, s.clients.descPKCE), R(Dt, s.clients.descPKCEEnforce), R(qt, s.clients.descOrigin), R(zt, s.clients.tokenLifetime.p1), R(Ht, s.clients.tokenLifetime.p2), R(Kt, s.clients.tokenLifetime.p3), R(Vt, s.clients.descAuthCode);
    }), u(K, X);
  }, $$slots: { default: true } }), d(E), u(U, E), se();
}
var Mr = O("<!> <!>", 1), Ur = O('<div class="container svelte-19xcrje"><div class="err"> </div> <!></div>');
function Nr(U, e) {
  ne(e, true);
  let r = ge(), s = C(""), v = C(void 0);
  he(() => {
    i(s, ""), i(v, ""), e.client.confidential ? f() : i(s, l(r.clients.confidentialNoSecret));
  });
  async function f() {
    let w = await Ve(`/auth/v1/clients/${e.client.id}/secret`);
    h(w);
  }
  async function P() {
    let w = await We(`/auth/v1/clients/${e.client.id}/secret`);
    h(w);
  }
  function h(w) {
    var _a;
    w.body ? w.body.secret && i(v, l(w.body.secret)) : i(s, l(((_a = w.error) == null ? void 0 : _a.message) || "Error"));
  }
  var b = Ur(), L = c(b), g = c(L, true);
  d(L);
  var m = a(L, 2);
  {
    var A = (w) => {
      var y = Mr(), S = ve(y);
      vr(S, { autocomplete: "off", label: "Client Secret", placeholder: "Client Secret", disabled: true, showCopy: true, get value() {
        return t(v);
      }, set value(B) {
        i(v, l(B));
      } });
      var k = a(S, 2);
      oe(k, { onclick: P, children: (B, N) => {
        W();
        var V = J();
        D(() => R(V, r.clients.generateSecret)), u(B, V);
      }, $$slots: { default: true } }), u(w, y);
    };
    $(m, (w) => {
      t(v) && w(A);
    });
  }
  d(b), D(() => R(g, t(s))), u(U, b), se();
}
var Br = O('<div class="err"> </div>'), Dr = O("<p> </p> <!> <!>", 1);
function Or(U, e) {
  ne(e, true);
  let r = we(), s = ge(), v = C("");
  async function f() {
    i(v, "");
    let A = await ze(`/auth/v1/clients/${e.client.id}`);
    A.error ? i(v, l(A.error.message)) : e.onSave();
  }
  var P = Dr(), h = ve(P), b = c(h, true);
  d(h);
  var L = a(h, 2);
  oe(L, { level: -1, onclick: f, children: (A, w) => {
    W();
    var y = J();
    D(() => R(y, r.common.delete)), u(A, y);
  }, $$slots: { default: true } });
  var g = a(L, 2);
  {
    var m = (A) => {
      var w = Br(), y = c(w, true);
      d(w), D(() => R(y, t(v))), u(A, w);
    };
    $(g, (A) => {
      t(v) && A(m);
    });
  }
  D(() => R(b, s.clients.delete1)), u(U, P), se();
}
var qr = O('<div><div class="flex"><input type="range" class="svelte-12j8dit"> <div class="value font-mono svelte-12j8dit"> </div></div> <div class="label svelte-12j8dit"><label class="font-label noselect svelte-12j8dit"> </label></div></div>');
function Oe(U, e) {
  ne(e, true);
  let r = Y(e, "value", 15), s = Y(e, "label", 3, ""), v = Y(e, "disabled", 3, false), f = Y(e, "step", 3, 1), P = Y(e, "widthRange", 3, "15rem");
  const h = ke();
  let b = fe(() => {
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
  var L = qr(), g = c(L), m = c(g);
  Ke(m), H(m, "id", h);
  let A;
  var w = a(m, 2), y = c(w, true);
  d(w), d(g);
  var S = a(g, 2), k = c(S);
  H(k, "for", h);
  var B = c(k, true);
  d(k), d(S), d(L), D(() => {
    H(m, "name", e.name), H(m, "title", s()), H(m, "aria-label", s()), m.disabled = v(), H(m, "aria-disabled", v()), H(m, "min", e.min), H(m, "max", e.max), H(m, "step", f()), A = re(m, "", A, { width: P(), "--bg-slider": t(b) }), R(y, r()), R(B, s());
  }), Je(m, r), u(U, L), se();
}
var jr = O('<div class="outer svelte-1evkwuy"><div class="container svelte-1evkwuy"><div><label class="font-label svelte-1evkwuy"> </label> <div class="values svelte-1evkwuy"><!> <!> <!></div></div> <div class="color svelte-1evkwuy"></div></div></div>');
function be(U, e) {
  ne(e, true);
  let r = Y(e, "h", 15), s = Y(e, "s", 15), v = Y(e, "l", 15);
  const f = ke(), P = "15rem";
  let h = fe(() => `hsl(${r()} ${s()} ${v()})`);
  var b = jr(), L = c(b);
  let g;
  var m = c(L), A = c(m);
  H(A, "for", f);
  var w = c(A, true);
  d(A);
  var y = a(A, 2);
  H(y, "id", f);
  var S = c(y);
  Oe(S, { label: "Hue", min: 0, max: 359, widthRange: P, bgMode: "hue", get hue() {
    return r();
  }, get sat() {
    return s();
  }, get lum() {
    return v();
  }, get value() {
    return r();
  }, set value(F) {
    r(F);
  } });
  var k = a(S, 2);
  Oe(k, { label: "Sat", min: 0, max: 100, widthRange: P, bgMode: "sat", get hue() {
    return r();
  }, get sat() {
    return s();
  }, get lum() {
    return v();
  }, get value() {
    return s();
  }, set value(F) {
    s(F);
  } });
  var B = a(k, 2);
  Oe(B, { label: "Lum", min: 0, max: 100, widthRange: P, bgMode: "lum", get hue() {
    return r();
  }, get sat() {
    return s();
  }, get lum() {
    return v();
  }, get value() {
    return v();
  }, set value(F) {
    v(F);
  } }), d(y), d(m);
  var N = a(m, 2);
  let V;
  d(L), d(b), D(() => {
    g = re(L, "", g, { "border-color": t(h) }), R(w, e.label), V = re(N, "", V, { background: t(h) });
  }), u(U, b), se();
}
var Fr = (U, e) => {
  var _a;
  return (_a = t(e)) == null ? void 0 : _a.click();
}, zr = O('<div class="container svelte-2nx18n"><!> <div role="none" class="mask svelte-2nx18n"><div class="colorWrapper svelte-2nx18n"><div class="color svelte-2nx18n"></div></div> <input type="color" class="svelte-2nx18n"></div></div>');
function qe(U, e) {
  ne(e, true);
  let r = Y(e, "value", 15), s = ge(), v = C(void 0);
  var f = zr();
  let P;
  var h = c(f);
  ue(h, { get label() {
    return e.label;
  }, get placeholder() {
    return e.label;
  }, get errMsg() {
    return s.validation.css;
  }, width: "17.5rem", required: true, pattern: Et, get value() {
    return r();
  }, set value(g) {
    r(g);
  } });
  var b = a(h, 2);
  b.__click = [Fr, v];
  var L = a(c(b), 2);
  Ke(L), Mt(L, (g) => i(v, g), () => t(v)), d(b), d(f), D(() => P = re(f, "", P, { "--color": r() })), Je(L, r), u(U, f), se();
}
He(["click"]);
var Hr = O('<div><p> </p> <div class="hsl svelte-10udpph"><div><!> <!> <!> <!></div> <div><!> <!> <!></div></div> <div><p><!></p> <!> <!> <!></div></div>');
function It(U, e) {
  ne(e, true);
  let r = Y(e, "values", 15), s = ge();
  var v = Hr(), f = c(v), P = c(f, true);
  d(f);
  var h = a(f, 2), b = c(h), L = c(b);
  be(L, { label: "--text", get h() {
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
  var g = a(L, 2);
  be(g, { label: "--text-high", get h() {
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
  var m = a(g, 2);
  be(m, { label: "--bg", get h() {
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
  var A = a(m, 2);
  be(A, { label: "--bg-high", get h() {
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
  var w = a(b, 2), y = c(w);
  be(y, { label: "--action", get h() {
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
  var S = a(y, 2);
  be(S, { label: "--accent", get h() {
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
  var k = a(S, 2);
  be(k, { label: "--error", get h() {
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
  } }), d(w), d(h);
  var B = a(h, 2), N = c(B), V = c(N);
  Ce(V, () => s.clients.branding.descFullCss), d(N);
  var F = a(N, 2);
  qe(F, { label: "--btn-text", get value() {
    return r().btn_text;
  }, set value(o) {
    r(r().btn_text = o, true);
  } });
  var p = a(F, 2);
  qe(p, { label: "--theme-sun", get value() {
    return r().theme_sun;
  }, set value(o) {
    r(r().theme_sun = o, true);
  } });
  var I = a(p, 2);
  qe(I, { label: "--theme-moon", get value() {
    return r().theme_moon;
  }, set value(o) {
    r(r().theme_moon = o, true);
  } }), d(B), d(v), D(() => R(P, s.clients.branding.descHsl)), u(U, v), se();
}
var Kr = O('<img alt="Client Logo" width="100%" height="100%">'), Vr = (U) => U.preventDefault(), Wr = O('<div aria-label="Preview: All components inside are only for theme and colors preview and have no effect or interaction"><div class="inner svelte-fkmn75"><div class="container svelte-fkmn75"><div class="logo svelte-fkmn75"><!></div> <h3 class="svelte-fkmn75">Header</h3> <p>Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam</p> <p><span class="svelte-fkmn75">--accent-color</span></p> <!> <div class="btn svelte-fkmn75"><div class="svelte-fkmn75"><!> <br> <!> <br> <!> <br></div> <div class="svelte-fkmn75"><!> <br> <!> <br> <!></div></div> <a>Link</a> <br> <!></div></div></div>');
function Rt(U, e) {
  ne(e, true);
  let r = C(false), s = C(false);
  he(() => {
    e.logoUrl && i(s, false);
  }), he(() => {
    t(r) && setTimeout(() => {
      i(r, false);
    }, 2e3);
  });
  var v = Wr();
  let f;
  var P = c(v), h = c(P), b = c(h), L = c(b);
  {
    var g = (_) => {
      hr(_, { width: "100%", get show() {
        return e.typ;
      } });
    }, m = (_) => {
      var M = Kr();
      D(() => H(M, "src", e.logoUrl)), je("error", M, () => i(s, true)), Zt(M), u(_, M);
    };
    $(L, (_) => {
      t(s) ? _(g) : _(m, false);
    });
  }
  d(b);
  var A = a(b, 8);
  ue(A, { label: "Preview", placeholder: "Preview", width: "12.5rem" });
  var w = a(A, 2), y = c(w), S = c(y);
  oe(S, { level: 1, get isLoading() {
    return t(r);
  }, onclick: () => i(r, true), children: (_, M) => {
    W();
    var q = J("Button");
    u(_, q);
  }, $$slots: { default: true } });
  var k = a(S, 4);
  oe(k, { level: 2, get isLoading() {
    return t(r);
  }, onclick: () => i(r, true), children: (_, M) => {
    W();
    var q = J("Button");
    u(_, q);
  }, $$slots: { default: true } });
  var B = a(k, 4);
  oe(B, { level: 3, get isLoading() {
    return t(r);
  }, onclick: () => i(r, true), children: (_, M) => {
    W();
    var q = J("Button");
    u(_, q);
  }, $$slots: { default: true } }), W(2), d(y);
  var N = a(y, 2), V = c(N);
  oe(V, { level: -1, get isLoading() {
    return t(r);
  }, onclick: () => i(r, true), children: (_, M) => {
    W();
    var q = J("Button");
    u(_, q);
  }, $$slots: { default: true } });
  var F = a(V, 4);
  oe(F, { level: -2, get isLoading() {
    return t(r);
  }, onclick: () => i(r, true), children: (_, M) => {
    W();
    var q = J("Button");
    u(_, q);
  }, $$slots: { default: true } });
  var p = a(F, 4);
  oe(p, { level: -3, get isLoading() {
    return t(r);
  }, onclick: () => i(r, true), children: (_, M) => {
    W();
    var q = J("Button");
    u(_, q);
  }, $$slots: { default: true } }), d(N), d(w);
  var I = a(w, 2);
  H(I, "href", window.location.href), I.__click = [Vr];
  var o = a(I, 4);
  fr(o, {}), d(h), d(P), d(v), D((_, M, q, E, x, G, K) => f = re(v, "", f, { "--border-radius": e.borderRadius, "--text": _, "--text-high": M, "--bg": q, "--bg-high": E, "--action": x, "--accent": G, "--error": K, "--btn-text": e.theme.btn_text, "--theme-sun": e.theme.theme_sun, "--theme-moon": e.theme.theme_moon }), [() => e.theme.text.join(" "), () => e.theme.text_high.join(" "), () => e.theme.bg.join(" "), () => e.theme.bg_high.join(" "), () => e.theme.action.join(" "), () => e.theme.accent.join(" "), () => e.theme.error.join(" ")]), u(U, v), se();
}
He(["click"]);
var Gr = O('<div><h2>Preview</h2> <div class="tabs svelte-1klemk8"><!></div> <div class="preview svelte-1klemk8"><!></div></div>');
function Jr(U, e) {
  ne(e, true);
  let r = ["Light Theme", "Dark Theme"], s = C(l(r[0]));
  var v = Gr(), f = a(c(v), 2), P = c(f);
  Ut(P, { tabs: r, center: true, get selected() {
    return t(s);
  }, set selected(m) {
    i(s, l(m));
  } }), d(f);
  var h = a(f, 2), b = c(h);
  {
    var L = (m) => {
      Rt(m, { get logoUrl() {
        return e.logoUrl;
      }, get borderRadius() {
        return e.theme.border_radius;
      }, get theme() {
        return e.theme.light;
      }, typ: "light" });
    }, g = (m) => {
      Rt(m, { get logoUrl() {
        return e.logoUrl;
      }, get borderRadius() {
        return e.theme.border_radius;
      }, get theme() {
        return e.theme.dark;
      }, typ: "dark" });
    };
    $(b, (m) => {
      t(s) === r[0] ? m(L) : m(g, false);
    });
  }
  d(h), d(v), u(U, v), se();
}
var Qr = O('<div class="err"> </div>'), Xr = O('<div class="values svelte-1pkqupw"><p><!></p> <!> <h1>Light Theme</h1> <!> <br> <h1>Dark Theme</h1> <!></div> <div class="preview"><!> <hr> <p>Logo Upload</p> <!> <div class="buttons svelte-1pkqupw"><!> <!> <!></div> <!></div>', 1), Yr = O('<div class="container svelte-1pkqupw"><!></div>');
function Zr(U, e) {
  ne(e, true);
  const r = "13rem";
  let s = we(), v = ge(), f = C(false), P = C(""), h = C(void 0), b = C(l(ke())), L = fe(() => `/auth/v1/clients/${e.client.id}/logo?${t(b)}`), g = fe(() => `/auth/v1/theme/${e.client.id}`);
  he(() => {
    m();
  });
  async function m() {
    var _a;
    let S = await Ve(t(g));
    S.body ? i(h, l(S.body)) : i(P, l(((_a = S.error) == null ? void 0 : _a.message) || "Error"));
  }
  async function A(S, k) {
    if (!t(h)) {
      console.error("theme is undefined");
      return;
    }
    let B = t(h);
    B.client_id = e.client.id;
    let N = await We(t(g), B);
    N.error ? i(P, l(N.error.message)) : (i(f, true), setTimeout(() => {
      i(f, false);
    }, 2e3));
  }
  async function w() {
    let S = await ze(t(g));
    S.error ? i(P, l(S.error.message)) : (await ze(`/auth/v1/clients/${e.client.id}/logo`), await m(), i(b, l(ke())), i(f, true), setTimeout(() => {
      i(f, false);
    }, 2e3));
  }
  async function y() {
    i(b, l(ke()));
  }
  Ge(U, { get action() {
    return t(g);
  }, onSubmit: A, children: (S, k) => {
    var B = Yr(), N = c(B);
    {
      var V = (F) => {
        var p = Xr(), I = ve(p), o = c(I), _ = c(o);
        Ce(_, () => v.clients.branding.descVariables), d(o);
        var M = a(o, 2);
        ue(M, { label: "--border-radius", placeholder: "--border-radius", get errMsg() {
          return v.validation.css;
        }, width: r, required: true, pattern: Et, get value() {
          return t(h).border_radius;
        }, set value(j) {
          t(h).border_radius = j;
        } });
        var q = a(M, 4);
        It(q, { get values() {
          return t(h).light;
        }, set values(j) {
          t(h).light = j;
        } });
        var E = a(q, 6);
        It(E, { get values() {
          return t(h).dark;
        }, set values(j) {
          t(h).dark = j;
        } }), d(I);
        var x = a(I, 2), G = c(x);
        ur(G, () => t(b), (j) => {
          Jr(j, { get logoUrl() {
            return t(L);
          }, get theme() {
            return t(h);
          } });
        });
        var K = a(G, 6);
        const ae = fe(() => `/auth/v1/clients/${e.client.id}/logo`);
        _r(K, { method: "PUT", get url() {
          return t(ae);
        }, fileName: "logo", onSuccess: y });
        var X = a(K, 2), ie = c(X);
        oe(ie, { type: "submit", children: (j, le) => {
          W();
          var ee = J();
          D(() => R(ee, s.common.save)), u(j, ee);
        }, $$slots: { default: true } });
        var te = a(ie, 2);
        oe(te, { level: -2, onclick: w, children: (j, le) => {
          W();
          var ee = J();
          D(() => R(ee, v.common.reset)), u(j, ee);
        }, $$slots: { default: true } });
        var z = a(te, 2);
        {
          var Z = (j) => {
            Nt(j, {});
          };
          $(z, (j) => {
            t(f) && j(Z);
          });
        }
        d(X);
        var _e = a(X, 2);
        {
          var de = (j) => {
            var le = Qr(), ee = c(le, true);
            d(le), D(() => R(ee, t(P))), u(j, le);
          };
          $(_e, (j) => {
            t(P) && j(de);
          });
        }
        d(x), u(F, p);
      };
      $(N, (F) => {
        t(h) && F(V);
      });
    }
    d(B), u(S, B);
  }, $$slots: { default: true } }), se();
}
var $r = O('<div class="flex"><!></div> <div class="details"><!></div>', 1);
function ea(U, e) {
  ne(e, true);
  let r = we(), s = ge();
  const v = [s.nav.config, "Secret", "Branding", r.common.delete];
  let f = C(l(v[0])), P = C(void 0);
  he(() => {
    e.client.id && requestAnimationFrame(() => {
      var _a;
      (_a = t(P)) == null ? void 0 : _a();
    });
  });
  var h = $r(), b = ve(h), L = c(b);
  Ut(L, { tabs: v, get selected() {
    return t(f);
  }, set selected(y) {
    i(f, l(y));
  }, get focusFirst() {
    return t(P);
  }, set focusFirst(y) {
    i(P, l(y));
  } }), d(b);
  var g = a(b, 2), m = c(g);
  {
    var A = (y) => {
      Er(y, { get client() {
        return e.client;
      }, get clients() {
        return e.clients;
      }, get scopesAll() {
        return e.scopesAll;
      }, get onSave() {
        return e.onSave;
      } });
    }, w = (y, S) => {
      {
        var k = (N) => {
          Nr(N, { get client() {
            return e.client;
          } });
        }, B = (N, V) => {
          {
            var F = (I) => {
              Zr(I, { get client() {
                return e.client;
              } });
            }, p = (I, o) => {
              {
                var _ = (M) => {
                  Or(M, { get client() {
                    return e.client;
                  }, get onSave() {
                    return e.onSave;
                  } });
                };
                $(I, (M) => {
                  t(f) === r.common.delete && M(_);
                }, o);
              }
            };
            $(N, (I) => {
              t(f) === "Branding" ? I(F) : I(p, false);
            }, V);
          }
        };
        $(y, (N) => {
          t(f) === "Secret" ? N(k) : N(B, false);
        }, S);
      }
    };
    $(m, (y) => {
      t(f) === s.nav.config ? y(A) : y(w, false);
    });
  }
  d(g), u(U, h), se();
}
var ta = O('<div class="tile svelte-140q9n9"> <div class="muted svelte-140q9n9"> </div></div>'), ra = O("<div></div> <!>", 1), aa = O("<!> <!>", 1), la = O('<div class="err"> </div>'), ia = O('<!> <div id="groups"><!></div>', 1), na = O("<!> <!>", 1);
function sa(U, e) {
  ne(e, true);
  let r = C(void 0), s = C(""), v = C(l([])), f = C(l([])), P = C(void 0), h = lr("cid"), b = C(l([]));
  const L = ["ID"];
  let g = C(l(L[0])), m = C("");
  const A = ["ID"];
  Yt(() => {
    w(), y();
  }), he(() => {
    i(P, l(t(v).find((p) => p.id === h.get())));
  }), he(() => {
    let p = t(m).toLowerCase();
    p ? t(g) === L[0] && i(f, l(t(v).filter((I) => I.id.toLowerCase().includes(p)))) : i(f, l(t(v)));
  });
  async function w() {
    var _a;
    let p = await St("/auth/v1/clients");
    p.body ? i(v, l(p.body)) : i(s, l(((_a = p.error) == null ? void 0 : _a.message) || "Error"));
  }
  async function y() {
    var _a;
    let p = await St("/auth/v1/scopes");
    p.body ? i(b, l(p.body.map((I) => I.name))) : i(s, l(((_a = p.error) == null ? void 0 : _a.message) || "Error"));
  }
  function S(p, I) {
    let o = I === "up";
    p === A[0] && t(v).sort((_, M) => o ? _.id.localeCompare(M.id) : M.id.localeCompare(_.id));
  }
  function k() {
    w();
  }
  async function B(p) {
    var _a;
    (_a = t(r)) == null ? void 0 : _a(), await w(), h.set(p);
  }
  var N = na(), V = ve(N);
  rr(V, { paddingTop: "2.1rem", buttonTilesAriaControls: "groups", width: "min(20rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (I) => {
    var o = ra(), _ = ve(o);
    re(_, "", {}, { height: ".5rem" });
    var M = a(_, 2);
    Fe(M, 17, () => t(f), (q) => q.id, (q, E, x, G) => {
      const K = fe(() => h.get() === t(E).id);
      tr(q, { onclick: () => h.set(t(E).id), get selected() {
        return t(K);
      }, children: (ae, X) => {
        var ie = ta(), te = c(ie), z = a(te), Z = c(z, true);
        d(z), d(ie), D(() => {
          R(te, `${t(E).id ?? ""} `), R(Z, t(E).name);
        }), u(ae, ie);
      } });
    }), u(I, o);
  }, children: (I, o) => {
    var _ = aa(), M = ve(_);
    const q = fe(() => t(v).length === 0 ? 1 : 2);
    $t(M, { get level() {
      return t(q);
    }, alignRight: true, get closeModal() {
      return t(r);
    }, set closeModal(x) {
      i(r, l(x));
    }, children: (x, G) => {
      Ar(x, { onSave: B, get clients() {
        return t(v);
      } });
    }, $$slots: { default: true } });
    var E = a(M, 2);
    ar(E, { searchOptions: L, orderOptions: A, onChangeOrder: S, searchWidth: "min(19.5rem, 100dvw - .5rem)", get searchOption() {
      return t(g);
    }, set searchOption(x) {
      i(g, l(x));
    }, get value() {
      return t(m);
    }, set value(x) {
      i(m, l(x));
    } }), u(I, _);
  }, $$slots: { buttonTiles: true, default: true } });
  var F = a(V, 2);
  er(F, { children: (p, I) => {
    var o = ia(), _ = ve(o);
    {
      var M = (G) => {
        var K = la(), ae = c(K, true);
        d(K), D(() => R(ae, t(s))), u(G, K);
      };
      $(_, (G) => {
        t(s) && G(M);
      });
    }
    var q = a(_, 2), E = c(q);
    {
      var x = (G) => {
        ea(G, { get client() {
          return t(P);
        }, get clients() {
          return t(v);
        }, get scopesAll() {
          return t(b);
        }, onSave: k });
      };
      $(E, (G) => {
        t(P) && G(x);
      });
    }
    d(q), u(p, o);
  } }), u(U, N), se();
}
function Wa(U) {
  sa(U, {});
}
export {
  Wa as component
};
