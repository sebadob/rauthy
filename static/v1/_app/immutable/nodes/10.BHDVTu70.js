import { t as D, a as c, d as Xt, e as J } from "../chunks/BxmJRzoY.js";
import { p as ne, a1 as K, j as y, a4 as fe, c as d, s as a, t as B, l, k as t, a as se, r as o, f as ce, a5 as _e, a3 as W, a0 as Yt } from "../chunks/w0HvPX0p.js";
import { d as He, e as je, s as I, b as Zt } from "../chunks/BzP2S3Z_.js";
import { i as $ } from "../chunks/iO9_dPNE.js";
import { e as Fe, i as Lt } from "../chunks/S81raU5Y.js";
import { r as Ke, a as re, s as z } from "../chunks/BdbQ6g_y.js";
import "../chunks/DM69BKKN.js";
import { B as $t } from "../chunks/DoeALvoe.js";
import { C as er } from "../chunks/C6jTHtu1.js";
import { N as tr } from "../chunks/D9abP6hj.js";
import { N as rr } from "../chunks/CzQKNE2_.js";
import { O as ar } from "../chunks/85nGGue2.js";
import { b as Ve, d as We, c as ze, f as St } from "../chunks/BO1A6s0c.js";
import { u as lr } from "../chunks/Cy3hLAXJ.js";
import { h as Ce } from "../chunks/C2ZdIFW_.js";
import { B as oe, t as ir, s as nr } from "../chunks/C8YTstTD.js";
import { I as ve } from "../chunks/Q4PIg3iI.js";
import { n as sr, g as pt, a as Ne, o as or, p as dr, q as Et } from "../chunks/gfDO7tLr.js";
import { F as Ge } from "../chunks/CDe-qvZi.js";
import { u as we } from "../chunks/0cG6LBdy.js";
import { u as ge } from "../chunks/DHOKTGcE.js";
import { I as ue } from "../chunks/ScYc5fRW.js";
import { b as Je } from "../chunks/Cxw7xmE1.js";
import { b as Mt } from "../chunks/Dgjj26O5.js";
import { p as Y } from "../chunks/C6GSeq7M.js";
import { g as ke } from "../chunks/B21bTIl7.js";
import { I as ur } from "../chunks/QCVRj9pj.js";
import { T as Ut } from "../chunks/BdAKL3gn.js";
import { I as Nt } from "../chunks/CTshzOVc.js";
import { L as Be } from "../chunks/DOFJTuej.js";
import { S as At } from "../chunks/CJX07lT1.js";
import { O as Tt } from "../chunks/DTYRN0IK.js";
import { I as cr } from "../chunks/iedauS3r.js";
import { k as vr } from "../chunks/DDNkWuIk.js";
import { T as fr } from "../chunks/B1f0afjj.js";
import { R as _r } from "../chunks/B-AEc_6C.js";
import { I as hr } from "../chunks/QDa6yuoZ.js";
import "../chunks/F_Qf1tHt.js";
function mr(M, e, r, n) {
  var _a;
  e(), (_a = r.onInput) == null ? void 0 : _a.call(r, t(n));
}
function gr(M, e, r) {
  var _a;
  switch (M.code) {
    case "Enter":
      M.preventDefault(), e(), (_a = r.onEnter) == null ? void 0 : _a.call(r);
      break;
  }
}
function br(M, e) {
  var _a, _b;
  (_a = e()) == null ? void 0 : _a.focus(), (_b = e()) == null ? void 0 : _b.scrollIntoView({ behavior: "smooth", block: "center" });
}
var kr = D('<li class="value svelte-r5y8pk"><div class="label"> </div> <div class="btnClose svelte-r5y8pk"><!></div></li>'), xr = D("<option></option>"), wr = D('<datalist class="absolute svelte-r5y8pk"></datalist>'), yr = D('<span class="err"> </span>'), Cr = D('<div role="none" class="container svelte-r5y8pk"><ul class="svelte-r5y8pk"><!> <li class="svelte-r5y8pk"><input autocomplete="off" class="svelte-r5y8pk"> <!></li></ul> <label class="font-label noselect svelte-r5y8pk"> <!></label></div>');
function xe(M, e) {
  ne(e, true);
  let r = Y(e, "ref", 15), n = Y(e, "typ", 3, "text"), u = Y(e, "id", 19, ke), v = Y(e, "values", 31, () => K([])), C = Y(e, "label", 3, ""), f = Y(e, "placeholder", 3, ""), g = Y(e, "disabled", 3, false), P = Y(e, "required", 3, false), m = Y(e, "isError", 15, false), h = Y(e, "width", 3, "inherit"), S = Y(e, "maxHeightList", 3, "inherit"), x = we();
  const w = ke();
  let L = y(""), b = fe(() => e.datalist && e.datalist.length > 0 ? w : void 0);
  function N(F) {
    v(v().filter((Z) => Z !== F));
  }
  function U(F) {
    var _a;
    R(), (_a = e.onBlur) == null ? void 0 : _a.call(e), j();
  }
  function V(F) {
    F.preventDefault(), m(true);
  }
  function j() {
    t(L) && R() && (v().push(t(L)), l(L, ""));
  }
  function R() {
    var _a;
    let F = (_a = r()) == null ? void 0 : _a.validity;
    return F ? (m(!F.valid), F.valid) : (m(false), true);
  }
  var T = Cr();
  T.__click = [br, r];
  let s;
  var _ = d(T);
  let E;
  var O = d(_);
  Fe(O, 17, v, Lt, (F, Z, he, de) => {
    var q = kr(), le = d(q), ee = d(le, true);
    o(le);
    var me = a(le, 2), Pe = d(me);
    oe(Pe, { invisible: true, onclick: () => N(t(Z)), children: (ye, Qe) => {
      ur(ye, { width: "1.2rem" });
    }, $$slots: { default: true } }), o(me), o(q), B(() => I(ee, t(Z))), c(F, q);
  });
  var p = a(O, 2), k = d(p);
  Ke(k), k.__keydown = [gr, j, e], k.__input = [mr, R, e, L], Mt(k, (F) => r(F), () => r());
  var G = a(k, 2);
  {
    var H = (F) => {
      var Z = wr();
      z(Z, "id", w), Fe(Z, 21, () => e.datalist, Lt, (he, de, q, le) => {
        var ee = xr(), me = {};
        B(() => {
          me !== (me = t(de)) && (ee.value = (ee.__value = t(de)) == null ? "" : t(de));
        }), c(he, ee);
      }), o(Z), c(F, Z);
    };
    $(G, (F) => {
      e.datalist && e.datalist.length > 1 && F(H);
    });
  }
  o(p), o(_);
  var ae = a(_, 2), X = d(ae), ie = a(X);
  {
    var te = (F) => {
      var Z = Xt(), he = ce(Z);
      {
        var de = (q) => {
          var le = yr(), ee = d(le, true);
          o(le), B(() => I(ee, e.errMsg || x.common.invalidInput)), c(q, le);
        };
        $(he, (q) => {
          m() && q(de);
        });
      }
      c(F, Z);
    };
    $(ie, (F) => {
      m() && F(te);
    });
  }
  o(ae), o(T), B(() => {
    s = re(T, "", s, { width: h() }), E = re(_, "", E, { "max-height": S() }), z(k, "type", n()), z(k, "id", u()), z(k, "name", e.name), z(k, "list", t(b)), z(k, "title", e.errMsg), z(k, "aria-label", C() || f()), z(k, "placeholder", f()), z(k, "aria-placeholder", f()), k.disabled = g(), z(k, "aria-disabled", g()), k.required = P() && v().length < 1, z(k, "aria-required", P() && v().length < 1), z(k, "maxlength", e.maxLength || void 0), z(k, "min", e.min || void 0), z(k, "max", e.max || void 0), z(k, "pattern", e.pattern), z(ae, "for", u()), z(ae, "data-required", P()), I(X, `${C() ?? ""} `);
  }), je("invalid", k, V), je("blur", k, U), Je(k, () => t(L), (F) => l(L, F)), c(M, T), se();
}
He(["click", "keydown", "input"]);
var Pr = D('<div class="err"> </div>'), Lr = D('<!> <!> <!> <p class="svelte-15tp6i6"><!></p> <!> <!> <!> <!>', 1), Sr = D('<div class="container svelte-15tp6i6"><!></div>');
function Ar(M, e) {
  ne(e, true);
  let r = we(), n = ge(), u = y(void 0), v = y(""), C = y(""), f = y(""), g = y(true), P = y(K([])), m = y(K([]));
  _e(() => {
    requestAnimationFrame(() => {
      var _a;
      (_a = t(u)) == null ? void 0 : _a.focus();
    });
  });
  async function h(w, L) {
    var _a;
    if (e.clients.find((U) => U.id === t(C))) {
      l(v, n.common.nameExistsAlready, true);
      return;
    }
    l(v, "");
    let b = { id: t(C), name: t(f) || void 0, confidential: t(g), redirect_uris: t(P), post_logout_redirect_uris: t(m).length > 0 ? t(m) : void 0 }, N = await Ve(w.action, b);
    N.body ? e.onSave(N.body.id) : l(v, ((_a = N.error) == null ? void 0 : _a.message) || "Error", true);
  }
  var S = Sr(), x = d(S);
  Ge(x, { action: "/auth/v1/clients", onSubmit: h, children: (w, L) => {
    var b = Lr(), N = ce(b);
    ve(N, { autocomplete: "off", label: "Client ID", placeholder: "Client ID", required: true, pattern: sr, get ref() {
      return t(u);
    }, set ref(p) {
      l(u, p, true);
    }, get value() {
      return t(C);
    }, set value(p) {
      l(C, p, true);
    } });
    var U = a(N, 2);
    ve(U, { autocomplete: "off", get label() {
      return n.clients.name;
    }, get placeholder() {
      return n.clients.name;
    }, pattern: pt, get value() {
      return t(f);
    }, set value(p) {
      l(f, p, true);
    } });
    var V = a(U, 2);
    ue(V, { get ariaLabel() {
      return n.clients.confidential;
    }, get checked() {
      return t(g);
    }, set checked(p) {
      l(g, p, true);
    }, children: (p, k) => {
      W();
      var G = J();
      B(() => I(G, n.clients.confidential)), c(p, G);
    }, $$slots: { default: true } });
    var j = a(V, 2), R = d(j);
    Ce(R, () => n.clients.descUri), o(j);
    var T = a(j, 2);
    xe(T, { typ: "url", label: "Redirect URIs", get errMsg() {
      return n.validation.uri;
    }, get values() {
      return t(P);
    }, set values(p) {
      l(P, p, true);
    } });
    var s = a(T, 2);
    xe(s, { typ: "url", label: "Post Logout Redirect URIs", get errMsg() {
      return n.validation.uri;
    }, get values() {
      return t(m);
    }, set values(p) {
      l(m, p, true);
    } });
    var _ = a(s, 2);
    oe(_, { type: "submit", children: (p, k) => {
      W();
      var G = J();
      B(() => I(G, r.common.save)), c(p, G);
    }, $$slots: { default: true } });
    var E = a(_, 2);
    {
      var O = (p) => {
        var k = Pr(), G = d(k, true);
        o(k), B(() => I(G, t(v))), c(p, k);
      };
      $(E, (p) => {
        t(v) && p(O);
      });
    }
    c(w, b);
  }, $$slots: { default: true } }), o(S), c(M, S), se();
}
const De = "urn:ietf:params:oauth:grant-type:device_code";
var Tr = D('<div class="err"> </div>'), Ir = D('<div class="err"> </div>'), Rr = D('<h5> </h5> <!> <p class="desc svelte-5tvvvt"> </p> <!> <p class="desc svelte-5tvvvt"> </p> <!> <!> <div></div> <h5> </h5> <!> <!> <!> <p class="mb-0 svelte-5tvvvt">Authentication Flows</p> <!> <!> <!> <!> <!> <div></div> <p class="desc svelte-5tvvvt"> </p> <p class="desc svelte-5tvvvt"><strong> </strong></p> <!> <!> <!> <div></div> <p class="desc svelte-5tvvvt"> </p> <!> <p class="desc svelte-5tvvvt"><!></p> <!> <!> <div></div> <p class="desc svelte-5tvvvt"><!></p> <!> <!> <div></div> <p> </p> <!> <div></div> <p> </p> <p> </p> <!> <!> <div></div> <p> </p> <!> <div class="flex gap-05"><!> <!></div> <!> <div></div>', 1), pr = D('<div class="container svelte-5tvvvt"><!></div>');
function Er(M, e) {
  var _a, _b;
  ne(e, true);
  let r = we(), n = ge();
  const u = "min(20rem, calc(100dvw - .5rem))";
  let v = y(""), C = y(false), f = y(K(e.client.id)), g = y(K(e.client.name || "")), P = y(K(e.client.enabled)), m = y(K(e.client.confidential)), h = y(K(e.client.client_uri || "")), S = y(K(e.client.contacts ? Array.from(e.client.contacts) : [])), x = y(K(e.client.allowed_origins ? Array.from(e.client.allowed_origins) : [])), w = y(K(Array.from(e.client.redirect_uris))), L = y(K(e.client.post_logout_redirect_uris ? Array.from(e.client.post_logout_redirect_uris) : [])), b = K({ authorizationCode: e.client.flows_enabled.includes("authorization_code"), clientCredentials: e.client.flows_enabled.includes("client_credentials"), password: e.client.flows_enabled.includes("password"), refreshToken: e.client.flows_enabled.includes("refresh_token"), deviceCode: e.client.flows_enabled.includes(De) });
  const N = ["RS256", "RS384", "RS512", "EdDSA"];
  let U = y(K(e.client.access_token_alg)), V = y(K(e.client.id_token_alg)), j = y(K(e.client.access_token_lifetime.toString())), R = y(K(e.client.auth_code_lifetime.toString())), T = y(K(e.scopesAll.map((H) => ({ name: H, selected: e.client.scopes.includes(H) || false })))), s = y(K(e.scopesAll.map((H) => ({ name: H, selected: e.client.default_scopes.includes(H) || false })))), _ = K({ plain: ((_a = e.client.challenges) == null ? void 0 : _a.includes("plain")) || false, s256: ((_b = e.client.challenges) == null ? void 0 : _b.includes("S256")) || false }), E = y(K(e.client.force_mfa));
  _e(() => {
    var _a2, _b2;
    e.client.id && (l(f, e.client.id, true), l(g, e.client.name || "", true), l(P, e.client.enabled, true), l(m, e.client.confidential, true), l(h, e.client.client_uri || "", true), l(S, e.client.contacts ? Array.from(e.client.contacts) : [], true), l(x, e.client.allowed_origins ? Array.from(e.client.allowed_origins) : [], true), l(w, Array.from(e.client.redirect_uris), true), l(L, e.client.post_logout_redirect_uris ? Array.from(e.client.post_logout_redirect_uris) : [], true), b.authorizationCode = e.client.flows_enabled.includes("authorization_code"), b.clientCredentials = e.client.flows_enabled.includes("client_credentials"), b.password = e.client.flows_enabled.includes("password"), b.refreshToken = e.client.flows_enabled.includes("refresh_token"), b.deviceCode = e.client.flows_enabled.includes(De), l(U, e.client.access_token_alg, true), l(V, e.client.id_token_alg, true), l(j, e.client.access_token_lifetime.toString(), true), l(R, e.client.auth_code_lifetime.toString(), true), l(T, e.scopesAll.map((H) => ({ name: H, selected: e.client.scopes.includes(H) || false })), true), l(s, e.scopesAll.map((H) => ({ name: H, selected: e.client.default_scopes.includes(H) || false })), true), _.plain = ((_a2 = e.client.challenges) == null ? void 0 : _a2.includes("plain")) || false, _.s256 = ((_b2 = e.client.challenges) == null ? void 0 : _b2.includes("S256")) || false);
  });
  async function O(H, ae) {
    l(v, "");
    let X = { id: t(f), name: t(g) || void 0, enabled: t(P), confidential: t(m), allowed_origins: t(x).length > 0 ? t(x) : void 0, redirect_uris: t(w), post_logout_redirect_uris: t(L).length > 0 ? t(L) : void 0, flows_enabled: [], access_token_alg: t(U), id_token_alg: t(V), access_token_lifetime: Number.parseInt(t(j)), auth_code_lifetime: Number.parseInt(t(R)), scopes: t(T).filter((te) => te.selected).map((te) => te.name), default_scopes: t(s).filter((te) => te.selected).map((te) => te.name), challenges: void 0, force_mfa: t(E), client_uri: t(h) || void 0, contacts: t(S).length > 0 ? t(S) : void 0 };
    b.authorizationCode && X.flows_enabled.push("authorization_code"), b.clientCredentials && X.flows_enabled.push("client_credentials"), b.password && X.flows_enabled.push("password"), b.refreshToken && X.flows_enabled.push("refresh_token"), b.deviceCode && X.flows_enabled.push(De), _.plain && (X.challenges = ["plain"]), _.s256 && (X.challenges ? X.challenges.push("S256") : X.challenges = ["S256"]);
    let ie = await We(H.action, X);
    ie.error ? l(v, ie.error.message, true) : (l(C, true), e.onSave(), setTimeout(() => {
      l(C, false);
    }, 2e3));
  }
  var p = pr(), k = d(p);
  const G = fe(() => `/auth/v1/clients/${e.client.id}`);
  Ge(k, { get action() {
    return t(G);
  }, onSubmit: O, children: (H, ae) => {
    var X = Rr(), ie = ce(X), te = d(ie, true);
    o(ie);
    var F = a(ie, 2);
    Be(F, { label: "ID", mono: true, get copyToClip() {
      return e.client.id;
    }, children: (i, Q) => {
      W();
      var A = J();
      B(() => I(A, e.client.id)), c(i, A);
    }, $$slots: { default: true } });
    var Z = a(F, 2), he = d(Z, true);
    o(Z);
    var de = a(Z, 2);
    ve(de, { autocomplete: "off", get label() {
      return n.clients.name;
    }, get placeholder() {
      return n.clients.name;
    }, width: u, pattern: pt, get value() {
      return t(g);
    }, set value(i) {
      l(g, i, true);
    } });
    var q = a(de, 2), le = d(q, true);
    o(q);
    var ee = a(q, 2);
    ve(ee, { typ: "url", autocomplete: "off", label: "URI", placeholder: "URI", width: u, pattern: Ne, get value() {
      return t(h);
    }, set value(i) {
      l(h, i, true);
    } });
    var me = a(ee, 2);
    xe(me, { get label() {
      return n.common.contact;
    }, pattern: or, get values() {
      return t(S);
    }, set values(i) {
      l(S, i, true);
    } });
    var Pe = a(me, 2);
    re(Pe, "", {}, { height: ".5rem" });
    var ye = a(Pe, 2), Qe = d(ye, true);
    o(ye);
    var Xe = a(ye, 2);
    ue(Xe, { get ariaLabel() {
      return n.common.enabled;
    }, get checked() {
      return t(P);
    }, set checked(i) {
      l(P, i, true);
    }, children: (i, Q) => {
      W();
      var A = J();
      B(() => I(A, n.common.enabled)), c(i, A);
    }, $$slots: { default: true } });
    var Ye = a(Xe, 2);
    ue(Ye, { get ariaLabel() {
      return n.clients.confidential;
    }, get checked() {
      return t(m);
    }, set checked(i) {
      l(m, i, true);
    }, children: (i, Q) => {
      W();
      var A = J();
      B(() => I(A, n.clients.confidential)), c(i, A);
    }, $$slots: { default: true } });
    var Ze = a(Ye, 2);
    ue(Ze, { get ariaLabel() {
      return n.clients.forceMfa;
    }, get checked() {
      return t(E);
    }, set checked(i) {
      l(E, i, true);
    }, children: (i, Q) => {
      W();
      var A = J();
      B(() => I(A, n.clients.forceMfa)), c(i, A);
    }, $$slots: { default: true } });
    var $e = a(Ze, 4);
    ue($e, { ariaLabel: "authorization_code", get checked() {
      return b.authorizationCode;
    }, set checked(i) {
      b.authorizationCode = i;
    }, children: (i, Q) => {
      W();
      var A = J("authorization_code");
      c(i, A);
    }, $$slots: { default: true } });
    var et = a($e, 2);
    ue(et, { ariaLabel: "urn:ietf:params:oauth:grant-type:device_code", get checked() {
      return b.deviceCode;
    }, set checked(i) {
      b.deviceCode = i;
    }, children: (i, Q) => {
      W();
      var A = J("device_code");
      c(i, A);
    }, $$slots: { default: true } });
    var tt = a(et, 2);
    ue(tt, { ariaLabel: "client_credentials", get checked() {
      return b.clientCredentials;
    }, set checked(i) {
      b.clientCredentials = i;
    }, children: (i, Q) => {
      W();
      var A = J("client_credentials");
      c(i, A);
    }, $$slots: { default: true } });
    var rt = a(tt, 2);
    ue(rt, { ariaLabel: "password", get checked() {
      return b.password;
    }, set checked(i) {
      b.password = i;
    }, children: (i, Q) => {
      W();
      var A = J("password");
      c(i, A);
    }, $$slots: { default: true } });
    var at = a(rt, 2);
    ue(at, { ariaLabel: "refresh_token", get checked() {
      return b.refreshToken;
    }, set checked(i) {
      b.refreshToken = i;
    }, children: (i, Q) => {
      W();
      var A = J("refresh_token");
      c(i, A);
    }, $$slots: { default: true } });
    var lt = a(at, 2);
    re(lt, "", {}, { height: ".5rem" });
    var Se = a(lt, 2), Bt = d(Se, true);
    o(Se);
    var Ae = a(Se, 2), it = d(Ae), Dt = d(it, true);
    o(it), o(Ae);
    var nt = a(Ae, 2);
    ue(nt, { ariaLabel: "PKCE plain", get checked() {
      return _.plain;
    }, set checked(i) {
      _.plain = i;
    }, children: (i, Q) => {
      W();
      var A = J("plain");
      c(i, A);
    }, $$slots: { default: true } });
    var st = a(nt, 2);
    ue(st, { ariaLabel: "PKCE S256", get checked() {
      return _.s256;
    }, set checked(i) {
      _.s256 = i;
    }, children: (i, Q) => {
      W();
      var A = J("S256");
      c(i, A);
    }, $$slots: { default: true } });
    var ot = a(st, 2);
    {
      var Ot = (i) => {
        var Q = Tr(), A = d(Q, true);
        o(Q), B(() => I(A, n.clients.errConfidentialPKCE)), ir(3, Q, () => nr, () => ({ duration: 150 })), c(i, Q);
      };
      $(ot, (i) => {
        !t(m) && !_.plain && !_.s256 && i(Ot);
      });
    }
    var dt = a(ot, 2);
    re(dt, "", {}, { height: ".5rem" });
    var Te = a(dt, 2), qt = d(Te, true);
    o(Te);
    var ut = a(Te, 2);
    xe(ut, { typ: "url", label: "Allowed Origins", get errMsg() {
      return n.validation.origin;
    }, pattern: dr, get values() {
      return t(x);
    }, set values(i) {
      l(x, i, true);
    } });
    var Ie = a(ut, 2), jt = d(Ie);
    Ce(jt, () => n.clients.descUri), o(Ie);
    var ct = a(Ie, 2);
    xe(ct, { typ: "url", label: "Redirect URIs", get errMsg() {
      return n.validation.uri;
    }, get required() {
      return b.authorizationCode;
    }, pattern: Ne, get values() {
      return t(w);
    }, set values(i) {
      l(w, i, true);
    } });
    var vt = a(ct, 2);
    xe(vt, { typ: "url", label: "Post Logout Redirect URIs", get errMsg() {
      return n.validation.uri;
    }, pattern: Ne, get values() {
      return t(L);
    }, set values(i) {
      l(L, i, true);
    } });
    var ft = a(vt, 2);
    re(ft, "", {}, { height: ".5rem" });
    var Re = a(ft, 2), Ft = d(Re);
    Ce(Ft, () => n.clients.scopes.desc), o(Re);
    var _t = a(Re, 2);
    At(_t, { get items() {
      return t(T);
    }, set items(i) {
      l(T, i, true);
    }, children: (i, Q) => {
      W();
      var A = J();
      B(() => I(A, n.clients.scopes.allowed)), c(i, A);
    }, $$slots: { default: true } });
    var ht = a(_t, 2);
    At(ht, { get items() {
      return t(s);
    }, set items(i) {
      l(s, i, true);
    }, children: (i, Q) => {
      W();
      var A = J();
      B(() => I(A, n.clients.scopes.default)), c(i, A);
    }, $$slots: { default: true } });
    var mt = a(ht, 2);
    re(mt, "", {}, { height: ".75rem" });
    var pe = a(mt, 2), zt = d(pe, true);
    o(pe);
    var gt = a(pe, 2);
    ve(gt, { typ: "number", autocomplete: "off", label: "Token Lifetime", placeholder: "Token Lifetime", width: u, min: "10", max: "86400", errMsg: "10 <= Token Lifetime <= 86400", get value() {
      return t(j);
    }, set value(i) {
      l(j, i, true);
    } });
    var bt = a(gt, 2);
    re(bt, "", {}, { height: ".5rem" });
    var Ee = a(bt, 2), Ht = d(Ee, true);
    o(Ee);
    var Me = a(Ee, 2), Kt = d(Me, true);
    o(Me);
    var kt = a(Me, 2);
    Be(kt, { label: "Access Token Algorithm", children: (i, Q) => {
      Tt(i, { ariaLabel: "Access Token Algorithm", options: N, borderless: true, get value() {
        return t(U);
      }, set value(A) {
        l(U, A, true);
      } });
    }, $$slots: { default: true } });
    var xt = a(kt, 2);
    Be(xt, { label: "ID Token Algorithm", children: (i, Q) => {
      Tt(i, { ariaLabel: "ID Token Algorithm", options: N, borderless: true, get value() {
        return t(V);
      }, set value(A) {
        l(V, A, true);
      } });
    }, $$slots: { default: true } });
    var wt = a(xt, 2);
    re(wt, "", {}, { height: ".5rem" });
    var Ue = a(wt, 2), Vt = d(Ue, true);
    o(Ue);
    var yt = a(Ue, 2);
    ve(yt, { typ: "number", autocomplete: "off", label: "Auth Code Lifetime", placeholder: "Auth Code Lifetime", width: u, min: "10", max: "300", errMsg: "10 <= Auth Code Lifetime <= 300", get value() {
      return t(R);
    }, set value(i) {
      l(R, i, true);
    } });
    var Le = a(yt, 2);
    re(Le, "", {}, { "margin-top": "1rem" });
    var Ct = d(Le);
    oe(Ct, { type: "submit", children: (i, Q) => {
      W();
      var A = J();
      B(() => I(A, r.common.save)), c(i, A);
    }, $$slots: { default: true } });
    var Wt = a(Ct, 2);
    {
      var Gt = (i) => {
        Nt(i, {});
      };
      $(Wt, (i) => {
        t(C) && i(Gt);
      });
    }
    o(Le);
    var Pt = a(Le, 2);
    {
      var Jt = (i) => {
        var Q = Ir(), A = d(Q, true);
        o(Q), B(() => I(A, t(v))), c(i, Q);
      };
      $(Pt, (i) => {
        t(v) && i(Jt);
      });
    }
    var Qt = a(Pt, 2);
    re(Qt, "", {}, { height: "1rem" }), B(() => {
      I(te, n.common.information), I(he, n.clients.descName), I(le, n.clients.descClientUri), I(Qe, n.clients.config), I(Bt, n.clients.descPKCE), I(Dt, n.clients.descPKCEEnforce), I(qt, n.clients.descOrigin), I(zt, n.clients.tokenLifetime.p1), I(Ht, n.clients.tokenLifetime.p2), I(Kt, n.clients.tokenLifetime.p3), I(Vt, n.clients.descAuthCode);
    }), c(H, X);
  }, $$slots: { default: true } }), o(p), c(M, p), se();
}
var Mr = D("<!> <!>", 1), Ur = D('<div class="container svelte-19xcrje"><div class="err"> </div> <!></div>');
function Nr(M, e) {
  ne(e, true);
  let r = ge(), n = y(""), u = y(void 0);
  _e(() => {
    l(n, ""), l(u, ""), e.client.confidential ? v() : l(n, r.clients.confidentialNoSecret, true);
  });
  async function v() {
    let x = await Ve(`/auth/v1/clients/${e.client.id}/secret`);
    f(x);
  }
  async function C() {
    let x = await We(`/auth/v1/clients/${e.client.id}/secret`);
    f(x);
  }
  function f(x) {
    var _a;
    x.body ? x.body.secret && l(u, x.body.secret, true) : l(n, ((_a = x.error) == null ? void 0 : _a.message) || "Error", true);
  }
  var g = Ur(), P = d(g), m = d(P, true);
  o(P);
  var h = a(P, 2);
  {
    var S = (x) => {
      var w = Mr(), L = ce(w);
      cr(L, { autocomplete: "off", label: "Client Secret", placeholder: "Client Secret", disabled: true, showCopy: true, get value() {
        return t(u);
      }, set value(N) {
        l(u, N, true);
      } });
      var b = a(L, 2);
      oe(b, { onclick: C, children: (N, U) => {
        W();
        var V = J();
        B(() => I(V, r.clients.generateSecret)), c(N, V);
      }, $$slots: { default: true } }), c(x, w);
    };
    $(h, (x) => {
      t(u) && x(S);
    });
  }
  o(g), B(() => I(m, t(n))), c(M, g), se();
}
var Br = D('<div class="err"> </div>'), Dr = D("<p> </p> <!> <!>", 1);
function Or(M, e) {
  ne(e, true);
  let r = we(), n = ge(), u = y("");
  async function v() {
    l(u, "");
    let S = await ze(`/auth/v1/clients/${e.client.id}`);
    S.error ? l(u, S.error.message, true) : e.onSave();
  }
  var C = Dr(), f = ce(C), g = d(f, true);
  o(f);
  var P = a(f, 2);
  oe(P, { level: -1, onclick: v, children: (S, x) => {
    W();
    var w = J();
    B(() => I(w, r.common.delete)), c(S, w);
  }, $$slots: { default: true } });
  var m = a(P, 2);
  {
    var h = (S) => {
      var x = Br(), w = d(x, true);
      o(x), B(() => I(w, t(u))), c(S, x);
    };
    $(m, (S) => {
      t(u) && S(h);
    });
  }
  B(() => I(g, n.clients.delete1)), c(M, C), se();
}
var qr = D('<div><div class="flex"><input type="range" class="svelte-12j8dit"> <div class="value font-mono svelte-12j8dit"> </div></div> <div class="label svelte-12j8dit"><label class="font-label noselect svelte-12j8dit"> </label></div></div>');
function Oe(M, e) {
  ne(e, true);
  let r = Y(e, "value", 15), n = Y(e, "label", 3, ""), u = Y(e, "disabled", 3, false), v = Y(e, "step", 3, 1), C = Y(e, "widthRange", 3, "15rem");
  const f = ke();
  let g = fe(() => {
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
  var P = qr(), m = d(P), h = d(m);
  Ke(h), z(h, "id", f);
  let S;
  var x = a(h, 2), w = d(x, true);
  o(x), o(m);
  var L = a(m, 2), b = d(L);
  z(b, "for", f);
  var N = d(b, true);
  o(b), o(L), o(P), B(() => {
    z(h, "name", e.name), z(h, "title", n()), z(h, "aria-label", n()), h.disabled = u(), z(h, "aria-disabled", u()), z(h, "min", e.min), z(h, "max", e.max), z(h, "step", v()), S = re(h, "", S, { width: C(), "--bg-slider": t(g) }), I(w, r()), I(N, n());
  }), Je(h, r), c(M, P), se();
}
var jr = D('<div class="outer svelte-1evkwuy"><div class="container svelte-1evkwuy"><div><label class="font-label svelte-1evkwuy"> </label> <div class="values svelte-1evkwuy"><!> <!> <!></div></div> <div class="color svelte-1evkwuy"></div></div></div>');
function be(M, e) {
  ne(e, true);
  let r = Y(e, "h", 15), n = Y(e, "s", 15), u = Y(e, "l", 15);
  const v = ke(), C = "15rem";
  let f = fe(() => `hsl(${r()} ${n()} ${u()})`);
  var g = jr(), P = d(g);
  let m;
  var h = d(P), S = d(h);
  z(S, "for", v);
  var x = d(S, true);
  o(S);
  var w = a(S, 2);
  z(w, "id", v);
  var L = d(w);
  Oe(L, { label: "Hue", min: 0, max: 359, widthRange: C, bgMode: "hue", get hue() {
    return r();
  }, get sat() {
    return n();
  }, get lum() {
    return u();
  }, get value() {
    return r();
  }, set value(j) {
    r(j);
  } });
  var b = a(L, 2);
  Oe(b, { label: "Sat", min: 0, max: 100, widthRange: C, bgMode: "sat", get hue() {
    return r();
  }, get sat() {
    return n();
  }, get lum() {
    return u();
  }, get value() {
    return n();
  }, set value(j) {
    n(j);
  } });
  var N = a(b, 2);
  Oe(N, { label: "Lum", min: 0, max: 100, widthRange: C, bgMode: "lum", get hue() {
    return r();
  }, get sat() {
    return n();
  }, get lum() {
    return u();
  }, get value() {
    return u();
  }, set value(j) {
    u(j);
  } }), o(w), o(h);
  var U = a(h, 2);
  let V;
  o(P), o(g), B(() => {
    m = re(P, "", m, { "border-color": t(f) }), I(x, e.label), V = re(U, "", V, { background: t(f) });
  }), c(M, g), se();
}
var Fr = (M, e) => {
  var _a;
  return (_a = t(e)) == null ? void 0 : _a.click();
}, zr = D('<div class="container svelte-2nx18n"><!> <div role="none" class="mask svelte-2nx18n"><div class="colorWrapper svelte-2nx18n"><div class="color svelte-2nx18n"></div></div> <input type="color" class="svelte-2nx18n"></div></div>');
function qe(M, e) {
  ne(e, true);
  let r = Y(e, "value", 15), n = ge(), u = y(void 0);
  var v = zr();
  let C;
  var f = d(v);
  ve(f, { get label() {
    return e.label;
  }, get placeholder() {
    return e.label;
  }, get errMsg() {
    return n.validation.css;
  }, width: "17.5rem", required: true, pattern: Et, get value() {
    return r();
  }, set value(m) {
    r(m);
  } });
  var g = a(f, 2);
  g.__click = [Fr, u];
  var P = a(d(g), 2);
  Ke(P), Mt(P, (m) => l(u, m), () => t(u)), o(g), o(v), B(() => C = re(v, "", C, { "--color": r() })), Je(P, r), c(M, v), se();
}
He(["click"]);
var Hr = D('<div><p> </p> <div class="hsl svelte-10udpph"><div><!> <!> <!> <!></div> <div><!> <!> <!></div></div> <div><p><!></p> <!> <!> <!></div></div>');
function It(M, e) {
  ne(e, true);
  let r = Y(e, "values", 15), n = ge();
  var u = Hr(), v = d(u), C = d(v, true);
  o(v);
  var f = a(v, 2), g = d(f), P = d(g);
  be(P, { label: "--text", get h() {
    return r().text[0];
  }, set h(s) {
    r(r().text[0] = s, true);
  }, get s() {
    return r().text[1];
  }, set s(s) {
    r(r().text[1] = s, true);
  }, get l() {
    return r().text[2];
  }, set l(s) {
    r(r().text[2] = s, true);
  } });
  var m = a(P, 2);
  be(m, { label: "--text-high", get h() {
    return r().text_high[0];
  }, set h(s) {
    r(r().text_high[0] = s, true);
  }, get s() {
    return r().text_high[1];
  }, set s(s) {
    r(r().text_high[1] = s, true);
  }, get l() {
    return r().text_high[2];
  }, set l(s) {
    r(r().text_high[2] = s, true);
  } });
  var h = a(m, 2);
  be(h, { label: "--bg", get h() {
    return r().bg[0];
  }, set h(s) {
    r(r().bg[0] = s, true);
  }, get s() {
    return r().bg[1];
  }, set s(s) {
    r(r().bg[1] = s, true);
  }, get l() {
    return r().bg[2];
  }, set l(s) {
    r(r().bg[2] = s, true);
  } });
  var S = a(h, 2);
  be(S, { label: "--bg-high", get h() {
    return r().bg_high[0];
  }, set h(s) {
    r(r().bg_high[0] = s, true);
  }, get s() {
    return r().bg_high[1];
  }, set s(s) {
    r(r().bg_high[1] = s, true);
  }, get l() {
    return r().bg_high[2];
  }, set l(s) {
    r(r().bg_high[2] = s, true);
  } }), o(g);
  var x = a(g, 2), w = d(x);
  be(w, { label: "--action", get h() {
    return r().action[0];
  }, set h(s) {
    r(r().action[0] = s, true);
  }, get s() {
    return r().action[1];
  }, set s(s) {
    r(r().action[1] = s, true);
  }, get l() {
    return r().action[2];
  }, set l(s) {
    r(r().action[2] = s, true);
  } });
  var L = a(w, 2);
  be(L, { label: "--accent", get h() {
    return r().accent[0];
  }, set h(s) {
    r(r().accent[0] = s, true);
  }, get s() {
    return r().accent[1];
  }, set s(s) {
    r(r().accent[1] = s, true);
  }, get l() {
    return r().accent[2];
  }, set l(s) {
    r(r().accent[2] = s, true);
  } });
  var b = a(L, 2);
  be(b, { label: "--error", get h() {
    return r().error[0];
  }, set h(s) {
    r(r().error[0] = s, true);
  }, get s() {
    return r().error[1];
  }, set s(s) {
    r(r().error[1] = s, true);
  }, get l() {
    return r().error[2];
  }, set l(s) {
    r(r().error[2] = s, true);
  } }), o(x), o(f);
  var N = a(f, 2), U = d(N), V = d(U);
  Ce(V, () => n.clients.branding.descFullCss), o(U);
  var j = a(U, 2);
  qe(j, { label: "--btn-text", get value() {
    return r().btn_text;
  }, set value(s) {
    r(r().btn_text = s, true);
  } });
  var R = a(j, 2);
  qe(R, { label: "--theme-sun", get value() {
    return r().theme_sun;
  }, set value(s) {
    r(r().theme_sun = s, true);
  } });
  var T = a(R, 2);
  qe(T, { label: "--theme-moon", get value() {
    return r().theme_moon;
  }, set value(s) {
    r(r().theme_moon = s, true);
  } }), o(N), o(u), B(() => I(C, n.clients.branding.descHsl)), c(M, u), se();
}
var Kr = D('<img alt="Client Logo" width="100%" height="100%">'), Vr = (M) => M.preventDefault(), Wr = D('<div aria-label="Preview: All components inside are only for theme and colors preview and have no effect or interaction"><div class="inner svelte-fkmn75"><div class="container svelte-fkmn75"><div class="logo svelte-fkmn75"><!></div> <h3 class="svelte-fkmn75">Header</h3> <p>Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam</p> <p><span class="svelte-fkmn75">--accent-color</span></p> <!> <div class="btn svelte-fkmn75"><div class="svelte-fkmn75"><!> <br> <!> <br> <!> <br></div> <div class="svelte-fkmn75"><!> <br> <!> <br> <!></div></div> <a>Link</a> <br> <!></div></div></div>');
function Rt(M, e) {
  ne(e, true);
  let r = y(false), n = y(false);
  _e(() => {
    e.logoUrl && l(n, false);
  }), _e(() => {
    t(r) && setTimeout(() => {
      l(r, false);
    }, 2e3);
  });
  var u = Wr();
  let v;
  var C = d(u), f = d(C), g = d(f), P = d(g);
  {
    var m = (_) => {
      _r(_, { width: "100%", get show() {
        return e.typ;
      } });
    }, h = (_) => {
      var E = Kr();
      B(() => z(E, "src", e.logoUrl)), je("error", E, () => l(n, true)), Zt(E), c(_, E);
    };
    $(P, (_) => {
      t(n) ? _(m) : _(h, false);
    });
  }
  o(g);
  var S = a(g, 8);
  ve(S, { label: "Preview", placeholder: "Preview", width: "12.5rem" });
  var x = a(S, 2), w = d(x), L = d(w);
  oe(L, { level: 1, get isLoading() {
    return t(r);
  }, onclick: () => l(r, true), children: (_, E) => {
    W();
    var O = J("Button");
    c(_, O);
  }, $$slots: { default: true } });
  var b = a(L, 4);
  oe(b, { level: 2, get isLoading() {
    return t(r);
  }, onclick: () => l(r, true), children: (_, E) => {
    W();
    var O = J("Button");
    c(_, O);
  }, $$slots: { default: true } });
  var N = a(b, 4);
  oe(N, { level: 3, get isLoading() {
    return t(r);
  }, onclick: () => l(r, true), children: (_, E) => {
    W();
    var O = J("Button");
    c(_, O);
  }, $$slots: { default: true } }), W(2), o(w);
  var U = a(w, 2), V = d(U);
  oe(V, { level: -1, get isLoading() {
    return t(r);
  }, onclick: () => l(r, true), children: (_, E) => {
    W();
    var O = J("Button");
    c(_, O);
  }, $$slots: { default: true } });
  var j = a(V, 4);
  oe(j, { level: -2, get isLoading() {
    return t(r);
  }, onclick: () => l(r, true), children: (_, E) => {
    W();
    var O = J("Button");
    c(_, O);
  }, $$slots: { default: true } });
  var R = a(j, 4);
  oe(R, { level: -3, get isLoading() {
    return t(r);
  }, onclick: () => l(r, true), children: (_, E) => {
    W();
    var O = J("Button");
    c(_, O);
  }, $$slots: { default: true } }), o(U), o(x);
  var T = a(x, 2);
  z(T, "href", window.location.href), T.__click = [Vr];
  var s = a(T, 4);
  fr(s, {}), o(f), o(C), o(u), B((_, E, O, p, k, G, H) => v = re(u, "", v, { "--border-radius": e.borderRadius, "--text": _, "--text-high": E, "--bg": O, "--bg-high": p, "--action": k, "--accent": G, "--error": H, "--btn-text": e.theme.btn_text, "--theme-sun": e.theme.theme_sun, "--theme-moon": e.theme.theme_moon }), [() => e.theme.text.join(" "), () => e.theme.text_high.join(" "), () => e.theme.bg.join(" "), () => e.theme.bg_high.join(" "), () => e.theme.action.join(" "), () => e.theme.accent.join(" "), () => e.theme.error.join(" ")]), c(M, u), se();
}
He(["click"]);
var Gr = D('<div><h2>Preview</h2> <div class="tabs svelte-1klemk8"><!></div> <div class="preview svelte-1klemk8"><!></div></div>');
function Jr(M, e) {
  ne(e, true);
  let r = ["Light Theme", "Dark Theme"], n = y(K(r[0]));
  var u = Gr(), v = a(d(u), 2), C = d(v);
  Ut(C, { tabs: r, center: true, get selected() {
    return t(n);
  }, set selected(h) {
    l(n, h, true);
  } }), o(v);
  var f = a(v, 2), g = d(f);
  {
    var P = (h) => {
      Rt(h, { get logoUrl() {
        return e.logoUrl;
      }, get borderRadius() {
        return e.theme.border_radius;
      }, get theme() {
        return e.theme.light;
      }, typ: "light" });
    }, m = (h) => {
      Rt(h, { get logoUrl() {
        return e.logoUrl;
      }, get borderRadius() {
        return e.theme.border_radius;
      }, get theme() {
        return e.theme.dark;
      }, typ: "dark" });
    };
    $(g, (h) => {
      t(n) === r[0] ? h(P) : h(m, false);
    });
  }
  o(f), o(u), c(M, u), se();
}
var Qr = D('<div class="err"> </div>'), Xr = D('<div class="values svelte-1pkqupw"><p><!></p> <!> <h1>Light Theme</h1> <!> <br> <h1>Dark Theme</h1> <!></div> <div class="preview"><!> <hr> <p>Logo Upload</p> <!> <div class="buttons svelte-1pkqupw"><!> <!> <!></div> <!></div>', 1), Yr = D('<div class="container svelte-1pkqupw"><!></div>');
function Zr(M, e) {
  ne(e, true);
  const r = "13rem";
  let n = we(), u = ge(), v = y(false), C = y(""), f = y(void 0), g = y(K(ke())), P = fe(() => `/auth/v1/clients/${e.client.id}/logo?${t(g)}`), m = fe(() => `/auth/v1/theme/${e.client.id}`);
  _e(() => {
    h();
  });
  async function h() {
    var _a;
    let L = await Ve(t(m));
    L.body ? l(f, L.body, true) : l(C, ((_a = L.error) == null ? void 0 : _a.message) || "Error", true);
  }
  async function S(L, b) {
    if (!t(f)) {
      console.error("theme is undefined");
      return;
    }
    let N = t(f);
    N.client_id = e.client.id;
    let U = await We(t(m), N);
    U.error ? l(C, U.error.message, true) : (l(v, true), setTimeout(() => {
      l(v, false);
    }, 2e3));
  }
  async function x() {
    let L = await ze(t(m));
    L.error ? l(C, L.error.message, true) : (await ze(`/auth/v1/clients/${e.client.id}/logo`), await h(), l(g, ke(), true), l(v, true), setTimeout(() => {
      l(v, false);
    }, 2e3));
  }
  async function w() {
    l(g, ke(), true);
  }
  Ge(M, { get action() {
    return t(m);
  }, onSubmit: S, children: (L, b) => {
    var N = Yr(), U = d(N);
    {
      var V = (j) => {
        var R = Xr(), T = ce(R), s = d(T), _ = d(s);
        Ce(_, () => u.clients.branding.descVariables), o(s);
        var E = a(s, 2);
        ve(E, { label: "--border-radius", placeholder: "--border-radius", get errMsg() {
          return u.validation.css;
        }, width: r, required: true, pattern: Et, get value() {
          return t(f).border_radius;
        }, set value(q) {
          t(f).border_radius = q;
        } });
        var O = a(E, 4);
        It(O, { get values() {
          return t(f).light;
        }, set values(q) {
          t(f).light = q;
        } });
        var p = a(O, 6);
        It(p, { get values() {
          return t(f).dark;
        }, set values(q) {
          t(f).dark = q;
        } }), o(T);
        var k = a(T, 2), G = d(k);
        vr(G, () => t(g), (q) => {
          Jr(q, { get logoUrl() {
            return t(P);
          }, get theme() {
            return t(f);
          } });
        });
        var H = a(G, 6);
        const ae = fe(() => `/auth/v1/clients/${e.client.id}/logo`);
        hr(H, { method: "PUT", get url() {
          return t(ae);
        }, fileName: "logo", onSuccess: w });
        var X = a(H, 2), ie = d(X);
        oe(ie, { type: "submit", children: (q, le) => {
          W();
          var ee = J();
          B(() => I(ee, n.common.save)), c(q, ee);
        }, $$slots: { default: true } });
        var te = a(ie, 2);
        oe(te, { level: -2, onclick: x, children: (q, le) => {
          W();
          var ee = J();
          B(() => I(ee, u.common.reset)), c(q, ee);
        }, $$slots: { default: true } });
        var F = a(te, 2);
        {
          var Z = (q) => {
            Nt(q, {});
          };
          $(F, (q) => {
            t(v) && q(Z);
          });
        }
        o(X);
        var he = a(X, 2);
        {
          var de = (q) => {
            var le = Qr(), ee = d(le, true);
            o(le), B(() => I(ee, t(C))), c(q, le);
          };
          $(he, (q) => {
            t(C) && q(de);
          });
        }
        o(k), c(j, R);
      };
      $(U, (j) => {
        t(f) && j(V);
      });
    }
    o(N), c(L, N);
  }, $$slots: { default: true } }), se();
}
var $r = D('<div class="flex"><!></div> <div class="details"><!></div>', 1);
function ea(M, e) {
  ne(e, true);
  let r = we(), n = ge();
  const u = [n.nav.config, "Secret", "Branding", r.common.delete];
  let v = y(K(u[0])), C = y(void 0);
  _e(() => {
    e.client.id && requestAnimationFrame(() => {
      var _a;
      (_a = t(C)) == null ? void 0 : _a();
    });
  });
  var f = $r(), g = ce(f), P = d(g);
  Ut(P, { tabs: u, get selected() {
    return t(v);
  }, set selected(w) {
    l(v, w, true);
  }, get focusFirst() {
    return t(C);
  }, set focusFirst(w) {
    l(C, w, true);
  } }), o(g);
  var m = a(g, 2), h = d(m);
  {
    var S = (w) => {
      Er(w, { get client() {
        return e.client;
      }, get clients() {
        return e.clients;
      }, get scopesAll() {
        return e.scopesAll;
      }, get onSave() {
        return e.onSave;
      } });
    }, x = (w, L) => {
      {
        var b = (U) => {
          Nr(U, { get client() {
            return e.client;
          } });
        }, N = (U, V) => {
          {
            var j = (T) => {
              Zr(T, { get client() {
                return e.client;
              } });
            }, R = (T, s) => {
              {
                var _ = (E) => {
                  Or(E, { get client() {
                    return e.client;
                  }, get onSave() {
                    return e.onSave;
                  } });
                };
                $(T, (E) => {
                  t(v) === r.common.delete && E(_);
                }, s);
              }
            };
            $(U, (T) => {
              t(v) === "Branding" ? T(j) : T(R, false);
            }, V);
          }
        };
        $(w, (U) => {
          t(v) === "Secret" ? U(b) : U(N, false);
        }, L);
      }
    };
    $(h, (w) => {
      t(v) === n.nav.config ? w(S) : w(x, false);
    });
  }
  o(m), c(M, f), se();
}
var ta = D('<div class="tile svelte-140q9n9"> <div class="muted svelte-140q9n9"> </div></div>'), ra = D("<div></div> <!>", 1), aa = D("<!> <!>", 1), la = D('<div class="err"> </div>'), ia = D('<!> <div id="groups"><!></div>', 1), na = D("<!> <!>", 1);
function sa(M, e) {
  ne(e, true);
  let r = y(void 0), n = y(""), u = y(K([])), v = y(K([])), C = y(void 0), f = lr("cid"), g = y(K([]));
  const P = ["ID"];
  let m = y(K(P[0])), h = y("");
  const S = ["ID"];
  Yt(() => {
    x(), w();
  }), _e(() => {
    l(C, t(u).find((R) => R.id === f.get()), true);
  }), _e(() => {
    let R = t(h).toLowerCase();
    R ? t(m) === P[0] && l(v, t(u).filter((T) => T.id.toLowerCase().includes(R)), true) : l(v, t(u), true);
  });
  async function x() {
    var _a;
    let R = await St("/auth/v1/clients");
    R.body ? l(u, R.body, true) : l(n, ((_a = R.error) == null ? void 0 : _a.message) || "Error", true);
  }
  async function w() {
    var _a;
    let R = await St("/auth/v1/scopes");
    R.body ? l(g, R.body.map((T) => T.name), true) : l(n, ((_a = R.error) == null ? void 0 : _a.message) || "Error", true);
  }
  function L(R, T) {
    let s = T === "up";
    R === S[0] && t(u).sort((_, E) => s ? _.id.localeCompare(E.id) : E.id.localeCompare(_.id));
  }
  function b() {
    x();
  }
  async function N(R) {
    var _a;
    (_a = t(r)) == null ? void 0 : _a(), await x(), f.set(R);
  }
  var U = na(), V = ce(U);
  rr(V, { paddingTop: "2.1rem", buttonTilesAriaControls: "groups", width: "min(20rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (T) => {
    var s = ra(), _ = ce(s);
    re(_, "", {}, { height: ".5rem" });
    var E = a(_, 2);
    Fe(E, 17, () => t(v), (O) => O.id, (O, p, k, G) => {
      const H = fe(() => f.get() === t(p).id);
      tr(O, { onclick: () => f.set(t(p).id), get selected() {
        return t(H);
      }, children: (ae, X) => {
        var ie = ta(), te = d(ie), F = a(te), Z = d(F, true);
        o(F), o(ie), B(() => {
          I(te, `${t(p).id ?? ""} `), I(Z, t(p).name);
        }), c(ae, ie);
      } });
    }), c(T, s);
  }, children: (T, s) => {
    var _ = aa(), E = ce(_);
    const O = fe(() => t(u).length === 0 ? 1 : 2);
    $t(E, { get level() {
      return t(O);
    }, alignRight: true, get closeModal() {
      return t(r);
    }, set closeModal(k) {
      l(r, k, true);
    }, children: (k, G) => {
      Ar(k, { onSave: N, get clients() {
        return t(u);
      } });
    }, $$slots: { default: true } });
    var p = a(E, 2);
    ar(p, { searchOptions: P, orderOptions: S, onChangeOrder: L, searchWidth: "min(19.5rem, 100dvw - .5rem)", get searchOption() {
      return t(m);
    }, set searchOption(k) {
      l(m, k, true);
    }, get value() {
      return t(h);
    }, set value(k) {
      l(h, k, true);
    } }), c(T, _);
  }, $$slots: { buttonTiles: true, default: true } });
  var j = a(V, 2);
  er(j, { children: (R, T) => {
    var s = ia(), _ = ce(s);
    {
      var E = (G) => {
        var H = la(), ae = d(H, true);
        o(H), B(() => I(ae, t(n))), c(G, H);
      };
      $(_, (G) => {
        t(n) && G(E);
      });
    }
    var O = a(_, 2), p = d(O);
    {
      var k = (G) => {
        ea(G, { get client() {
          return t(C);
        }, get clients() {
          return t(u);
        }, get scopesAll() {
          return t(g);
        }, onSave: b });
      };
      $(p, (G) => {
        t(C) && G(k);
      });
    }
    o(O), c(R, s);
  } }), c(M, U), se();
}
function Ga(M) {
  sa(M, {});
}
export {
  Ga as component
};
