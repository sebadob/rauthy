import { t as O, a as c, d as er, e as J } from "../chunks/BxmJRzoY.js";
import { p as ie, a1 as H, j as w, a4 as he, c as d, s as a, t as D, l, k as t, a as ne, r as s, f as ue, a5 as fe, a3 as V, a0 as tr } from "../chunks/w0HvPX0p.js";
import { d as Ve, e as ze, s as p, b as rr } from "../chunks/BzP2S3Z_.js";
import { i as Z } from "../chunks/iO9_dPNE.js";
import { e as He, i as At } from "../chunks/S81raU5Y.js";
import { r as We, a as re, s as z } from "../chunks/BdbQ6g_y.js";
import "../chunks/DM69BKKN.js";
import { B as ar } from "../chunks/CyiD59-2.js";
import { C as lr } from "../chunks/C6jTHtu1.js";
import { N as ir } from "../chunks/D9abP6hj.js";
import { N as nr } from "../chunks/CzQKNE2_.js";
import { O as sr } from "../chunks/HVQD_1Ej.js";
import { b as Ge, d as Je, c as Ke, f as Tt } from "../chunks/UPFlzoow.js";
import { u as or } from "../chunks/Bz_VUaol.js";
import { h as we } from "../chunks/C2ZdIFW_.js";
import { B as oe, t as dr, s as ur } from "../chunks/C8YTstTD.js";
import { I as _e } from "../chunks/DVXwAhn3.js";
import { n as cr, g as Ut, a as Ae, o as vr, p as _r, q as Mt } from "../chunks/gfDO7tLr.js";
import { F as Qe } from "../chunks/BEbxeS8S.js";
import { u as ye } from "../chunks/0cG6LBdy.js";
import { u as me } from "../chunks/Bt_9UXew.js";
import { I as de } from "../chunks/ScYc5fRW.js";
import { b as Xe } from "../chunks/Cxw7xmE1.js";
import { b as Bt } from "../chunks/Dgjj26O5.js";
import { p as Y } from "../chunks/C6GSeq7M.js";
import { g as ke } from "../chunks/B21bTIl7.js";
import { I as hr } from "../chunks/QCVRj9pj.js";
import { T as Nt } from "../chunks/BdAKL3gn.js";
import { I as Dt } from "../chunks/CTshzOVc.js";
import { L as Oe } from "../chunks/DQwTjYxX.js";
import { S as It } from "../chunks/ueDoZEuj.js";
import { O as pt } from "../chunks/BBiSUZQU.js";
import { I as fr } from "../chunks/iedauS3r.js";
import { k as gr } from "../chunks/DDNkWuIk.js";
import { T as mr } from "../chunks/B1f0afjj.js";
import { R as br } from "../chunks/B-AEc_6C.js";
import { I as kr } from "../chunks/C39TjCb0.js";
import "../chunks/F_Qf1tHt.js";
function xr(E, e, r, n) {
  var _a;
  e(), (_a = r.onInput) == null ? void 0 : _a.call(r, t(n));
}
function wr(E, e, r) {
  var _a;
  switch (E.code) {
    case "Enter":
      E.preventDefault(), e(), (_a = r.onEnter) == null ? void 0 : _a.call(r);
      break;
  }
}
function yr(E, e) {
  var _a, _b;
  (_a = e()) == null ? void 0 : _a.focus(), (_b = e()) == null ? void 0 : _b.scrollIntoView({ behavior: "smooth", block: "center" });
}
var Cr = O('<li class="value svelte-r5y8pk"><div class="label"> </div> <div class="btnClose svelte-r5y8pk"><!></div></li>'), Lr = O("<option></option>"), Pr = O('<datalist class="absolute svelte-r5y8pk"></datalist>'), Sr = O('<span class="err"> </span>'), Ar = O('<div role="none" class="container svelte-r5y8pk"><ul class="svelte-r5y8pk"><!> <li class="svelte-r5y8pk"><input autocomplete="off" class="svelte-r5y8pk"> <!></li></ul> <label class="font-label noselect svelte-r5y8pk"> <!></label></div>');
function xe(E, e) {
  ie(e, true);
  let r = Y(e, "ref", 15), n = Y(e, "typ", 3, "text"), u = Y(e, "id", 19, ke), v = Y(e, "values", 31, () => H([])), C = Y(e, "label", 3, ""), _ = Y(e, "placeholder", 3, ""), b = Y(e, "disabled", 3, false), L = Y(e, "required", 3, false), g = Y(e, "isError", 15, false), f = Y(e, "width", 3, "inherit"), T = Y(e, "maxHeightList", 3, "inherit"), y = ye();
  const x = ke();
  let S = w(""), q = he(() => e.datalist && e.datalist.length > 0 ? x : void 0);
  function h(M) {
    v(v().filter(($) => $ !== M));
  }
  function B(M) {
    var _a;
    R(), (_a = e.onBlur) == null ? void 0 : _a.call(e), j();
  }
  function K(M) {
    M.preventDefault(), g(true);
  }
  function j() {
    t(S) && R() && (v().push(t(S)), l(S, ""));
  }
  function R() {
    var _a;
    let M = (_a = r()) == null ? void 0 : _a.validity;
    return M ? (g(!M.valid), M.valid) : (g(false), true);
  }
  var I = Ar();
  I.__click = [yr, r];
  let o;
  var k = d(I);
  let P;
  var N = d(k);
  He(N, 17, v, At, (M, $, ce, ve) => {
    var F = Cr(), ee = d(F), te = d(ee, true);
    s(ee);
    var ge = a(ee, 2), Ce = d(ge);
    oe(Ce, { invisible: true, onclick: () => h(t($)), children: (Le, Pe) => {
      hr(Le, { width: "1.2rem" });
    }, $$slots: { default: true } }), s(ge), s(F), D(() => p(te, t($))), c(M, F);
  });
  var U = a(N, 2), m = d(U);
  We(m), m.__keydown = [wr, j, e], m.__input = [xr, R, e, S], Bt(m, (M) => r(M), () => r());
  var W = a(m, 2);
  {
    var ae = (M) => {
      var $ = Pr();
      z($, "id", x), He($, 21, () => e.datalist, At, (ce, ve, F, ee) => {
        var te = Lr(), ge = {};
        D(() => {
          ge !== (ge = t(ve)) && (te.value = (te.__value = t(ve)) == null ? "" : t(ve));
        }), c(ce, te);
      }), s($), c(M, $);
    };
    Z(W, (M) => {
      e.datalist && e.datalist.length > 1 && M(ae);
    });
  }
  s(U), s(k);
  var G = a(k, 2), se = d(G), X = a(se);
  {
    var le = (M) => {
      var $ = er(), ce = ue($);
      {
        var ve = (F) => {
          var ee = Sr(), te = d(ee, true);
          s(ee), D(() => p(te, e.errMsg || y.common.invalidInput)), c(F, ee);
        };
        Z(ce, (F) => {
          g() && F(ve);
        });
      }
      c(M, $);
    };
    Z(X, (M) => {
      g() && M(le);
    });
  }
  s(G), s(I), D(() => {
    o = re(I, "", o, { width: f() }), P = re(k, "", P, { "max-height": T() }), z(m, "type", n()), z(m, "id", u()), z(m, "name", e.name), z(m, "list", t(q)), z(m, "title", e.errMsg), z(m, "aria-label", C() || _()), z(m, "placeholder", _()), z(m, "aria-placeholder", _()), m.disabled = b(), z(m, "aria-disabled", b()), m.required = L() && v().length < 1, z(m, "aria-required", L() && v().length < 1), z(m, "maxlength", e.maxLength || void 0), z(m, "min", e.min || void 0), z(m, "max", e.max || void 0), z(m, "pattern", e.pattern), z(G, "for", u()), z(G, "data-required", L()), p(se, `${C() ?? ""} `);
  }), ze("invalid", m, K), ze("blur", m, B), Xe(m, () => t(S), (M) => l(S, M)), c(E, I), ne();
}
Ve(["click", "keydown", "input"]);
var Tr = O('<div class="err"> </div>'), Ir = O('<!> <!> <!> <p class="svelte-15tp6i6"><!></p> <!> <!> <!> <!>', 1), pr = O('<div class="container svelte-15tp6i6"><!></div>');
function Rr(E, e) {
  ie(e, true);
  let r = ye(), n = me(), u = w(void 0), v = w(""), C = w(""), _ = w(""), b = w(true), L = w(H([])), g = w(H([]));
  fe(() => {
    requestAnimationFrame(() => {
      var _a;
      (_a = t(u)) == null ? void 0 : _a.focus();
    });
  });
  async function f(x, S) {
    var _a;
    if (e.clients.find((B) => B.id === t(C))) {
      l(v, n.common.nameExistsAlready, true);
      return;
    }
    l(v, "");
    let q = { id: t(C), name: t(_) || void 0, confidential: t(b), redirect_uris: t(L), post_logout_redirect_uris: t(g).length > 0 ? t(g) : void 0 }, h = await Ge(x.action, q);
    h.body ? e.onSave(h.body.id) : l(v, ((_a = h.error) == null ? void 0 : _a.message) || "Error", true);
  }
  var T = pr(), y = d(T);
  Qe(y, { action: "/auth/v1/clients", onSubmit: f, children: (x, S) => {
    var q = Ir(), h = ue(q);
    _e(h, { autocomplete: "off", label: "Client ID", placeholder: "Client ID", required: true, pattern: cr, get ref() {
      return t(u);
    }, set ref(U) {
      l(u, U, true);
    }, get value() {
      return t(C);
    }, set value(U) {
      l(C, U, true);
    } });
    var B = a(h, 2);
    _e(B, { autocomplete: "off", get label() {
      return n.clients.name;
    }, get placeholder() {
      return n.clients.name;
    }, pattern: Ut, get value() {
      return t(_);
    }, set value(U) {
      l(_, U, true);
    } });
    var K = a(B, 2);
    de(K, { get ariaLabel() {
      return n.clients.confidential;
    }, get checked() {
      return t(b);
    }, set checked(U) {
      l(b, U, true);
    }, children: (U, m) => {
      V();
      var W = J();
      D(() => p(W, n.clients.confidential)), c(U, W);
    }, $$slots: { default: true } });
    var j = a(K, 2), R = d(j);
    we(R, () => n.clients.descUri), s(j);
    var I = a(j, 2);
    xe(I, { typ: "url", label: "Redirect URIs", get errMsg() {
      return n.validation.uri;
    }, get values() {
      return t(L);
    }, set values(U) {
      l(L, U, true);
    } });
    var o = a(I, 2);
    xe(o, { typ: "url", label: "Post Logout Redirect URIs", get errMsg() {
      return n.validation.uri;
    }, get values() {
      return t(g);
    }, set values(U) {
      l(g, U, true);
    } });
    var k = a(o, 2);
    oe(k, { type: "submit", children: (U, m) => {
      V();
      var W = J();
      D(() => p(W, r.common.save)), c(U, W);
    }, $$slots: { default: true } });
    var P = a(k, 2);
    {
      var N = (U) => {
        var m = Tr(), W = d(m, true);
        s(m), D(() => p(W, t(v))), c(U, m);
      };
      Z(P, (U) => {
        t(v) && U(N);
      });
    }
    c(x, q);
  }, $$slots: { default: true } }), s(T), c(E, T), ne();
}
const qe = "urn:ietf:params:oauth:grant-type:device_code";
var Er = O('<div class="err"> </div>'), Ur = O('<div class="err"> </div>'), Mr = O('<h5> </h5> <!> <p class="desc svelte-5tvvvt"> </p> <!> <p class="desc svelte-5tvvvt"> </p> <!> <!> <div></div> <h5> </h5> <!> <!> <!> <p class="mb-0 svelte-5tvvvt">Authentication Flows</p> <!> <!> <!> <!> <!> <div></div> <p class="desc svelte-5tvvvt"> </p> <p class="desc svelte-5tvvvt"><strong> </strong></p> <!> <!> <!> <div></div> <p class="desc svelte-5tvvvt"><!></p> <!> <p class="desc svelte-5tvvvt"> </p> <!> <p class="desc svelte-5tvvvt"><!></p> <!> <!> <div></div> <p class="desc svelte-5tvvvt"><!></p> <!> <!> <div></div> <p> </p> <!> <div></div> <p> </p> <p> </p> <!> <!> <div></div> <p> </p> <!> <div class="flex gap-05"><!> <!></div> <!> <div></div>', 1), Br = O('<div class="container svelte-5tvvvt"><!></div>');
function Nr(E, e) {
  var _a, _b;
  ie(e, true);
  let r = ye(), n = me();
  const u = "min(20rem, calc(100dvw - .5rem))";
  let v = w(""), C = w(false), _ = w(H(e.client.id)), b = w(H(e.client.name || "")), L = w(H(e.client.enabled)), g = w(H(e.client.confidential)), f = w(H(e.client.client_uri || "")), T = w(H(e.client.backchannel_logout_uri || "")), y = w(H(e.client.contacts ? Array.from(e.client.contacts) : [])), x = w(H(e.client.allowed_origins ? Array.from(e.client.allowed_origins) : [])), S = w(H(Array.from(e.client.redirect_uris))), q = w(H(e.client.post_logout_redirect_uris ? Array.from(e.client.post_logout_redirect_uris) : [])), h = H({ authorizationCode: e.client.flows_enabled.includes("authorization_code"), clientCredentials: e.client.flows_enabled.includes("client_credentials"), password: e.client.flows_enabled.includes("password"), refreshToken: e.client.flows_enabled.includes("refresh_token"), deviceCode: e.client.flows_enabled.includes(qe) });
  const B = ["RS256", "RS384", "RS512", "EdDSA"];
  let K = w(H(e.client.access_token_alg)), j = w(H(e.client.id_token_alg)), R = w(H(e.client.access_token_lifetime.toString())), I = w(H(e.client.auth_code_lifetime.toString())), o = w(H(e.scopesAll.map((G) => ({ name: G, selected: e.client.scopes.includes(G) || false })))), k = w(H(e.scopesAll.map((G) => ({ name: G, selected: e.client.default_scopes.includes(G) || false })))), P = H({ plain: ((_a = e.client.challenges) == null ? void 0 : _a.includes("plain")) || false, s256: ((_b = e.client.challenges) == null ? void 0 : _b.includes("S256")) || false }), N = w(H(e.client.force_mfa));
  fe(() => {
    var _a2, _b2;
    e.client.id && (l(_, e.client.id, true), l(b, e.client.name || "", true), l(L, e.client.enabled, true), l(g, e.client.confidential, true), l(f, e.client.client_uri || "", true), l(T, e.client.backchannel_logout_uri || "", true), l(y, e.client.contacts ? Array.from(e.client.contacts) : [], true), l(x, e.client.allowed_origins ? Array.from(e.client.allowed_origins) : [], true), l(S, Array.from(e.client.redirect_uris), true), l(q, e.client.post_logout_redirect_uris ? Array.from(e.client.post_logout_redirect_uris) : [], true), h.authorizationCode = e.client.flows_enabled.includes("authorization_code"), h.clientCredentials = e.client.flows_enabled.includes("client_credentials"), h.password = e.client.flows_enabled.includes("password"), h.refreshToken = e.client.flows_enabled.includes("refresh_token"), h.deviceCode = e.client.flows_enabled.includes(qe), l(K, e.client.access_token_alg, true), l(j, e.client.id_token_alg, true), l(R, e.client.access_token_lifetime.toString(), true), l(I, e.client.auth_code_lifetime.toString(), true), l(o, e.scopesAll.map((G) => ({ name: G, selected: e.client.scopes.includes(G) || false })), true), l(k, e.scopesAll.map((G) => ({ name: G, selected: e.client.default_scopes.includes(G) || false })), true), P.plain = ((_a2 = e.client.challenges) == null ? void 0 : _a2.includes("plain")) || false, P.s256 = ((_b2 = e.client.challenges) == null ? void 0 : _b2.includes("S256")) || false);
  });
  async function U(G, se) {
    l(v, "");
    let X = { id: t(_), name: t(b) || void 0, enabled: t(L), confidential: t(g), allowed_origins: t(x).length > 0 ? t(x) : void 0, redirect_uris: t(S), post_logout_redirect_uris: t(q).length > 0 ? t(q) : void 0, flows_enabled: [], access_token_alg: t(K), id_token_alg: t(j), access_token_lifetime: Number.parseInt(t(R)), auth_code_lifetime: Number.parseInt(t(I)), scopes: t(o).filter((M) => M.selected).map((M) => M.name), default_scopes: t(k).filter((M) => M.selected).map((M) => M.name), challenges: void 0, force_mfa: t(N), client_uri: t(f) || void 0, contacts: t(y).length > 0 ? t(y) : void 0, backchannel_logout_uri: t(T) || void 0 };
    h.authorizationCode && X.flows_enabled.push("authorization_code"), h.clientCredentials && X.flows_enabled.push("client_credentials"), h.password && X.flows_enabled.push("password"), h.refreshToken && X.flows_enabled.push("refresh_token"), h.deviceCode && X.flows_enabled.push(qe), P.plain && (X.challenges = ["plain"]), P.s256 && (X.challenges ? X.challenges.push("S256") : X.challenges = ["S256"]);
    let le = await Je(G.action, X);
    le.error ? l(v, le.error.message, true) : (l(C, true), e.onSave(), setTimeout(() => {
      l(C, false);
    }, 2e3));
  }
  var m = Br(), W = d(m);
  const ae = he(() => `/auth/v1/clients/${e.client.id}`);
  Qe(W, { get action() {
    return t(ae);
  }, onSubmit: U, children: (G, se) => {
    var X = Mr(), le = ue(X), M = d(le, true);
    s(le);
    var $ = a(le, 2);
    Oe($, { label: "ID", mono: true, get copyToClip() {
      return e.client.id;
    }, children: (i, Q) => {
      V();
      var A = J();
      D(() => p(A, e.client.id)), c(i, A);
    }, $$slots: { default: true } });
    var ce = a($, 2), ve = d(ce, true);
    s(ce);
    var F = a(ce, 2);
    _e(F, { autocomplete: "off", get label() {
      return n.clients.name;
    }, get placeholder() {
      return n.clients.name;
    }, width: u, pattern: Ut, get value() {
      return t(b);
    }, set value(i) {
      l(b, i, true);
    } });
    var ee = a(F, 2), te = d(ee, true);
    s(ee);
    var ge = a(ee, 2);
    _e(ge, { typ: "url", autocomplete: "off", label: "URI", placeholder: "URI", width: u, pattern: Ae, get value() {
      return t(f);
    }, set value(i) {
      l(f, i, true);
    } });
    var Ce = a(ge, 2);
    xe(Ce, { get label() {
      return n.common.contact;
    }, pattern: vr, get values() {
      return t(y);
    }, set values(i) {
      l(y, i, true);
    } });
    var Le = a(Ce, 2);
    re(Le, "", {}, { height: ".5rem" });
    var Pe = a(Le, 2), Ot = d(Pe, true);
    s(Pe);
    var Ye = a(Pe, 2);
    de(Ye, { get ariaLabel() {
      return n.common.enabled;
    }, get checked() {
      return t(L);
    }, set checked(i) {
      l(L, i, true);
    }, children: (i, Q) => {
      V();
      var A = J();
      D(() => p(A, n.common.enabled)), c(i, A);
    }, $$slots: { default: true } });
    var Ze = a(Ye, 2);
    de(Ze, { get ariaLabel() {
      return n.clients.confidential;
    }, get checked() {
      return t(g);
    }, set checked(i) {
      l(g, i, true);
    }, children: (i, Q) => {
      V();
      var A = J();
      D(() => p(A, n.clients.confidential)), c(i, A);
    }, $$slots: { default: true } });
    var $e = a(Ze, 2);
    de($e, { get ariaLabel() {
      return n.clients.forceMfa;
    }, get checked() {
      return t(N);
    }, set checked(i) {
      l(N, i, true);
    }, children: (i, Q) => {
      V();
      var A = J();
      D(() => p(A, n.clients.forceMfa)), c(i, A);
    }, $$slots: { default: true } });
    var et = a($e, 4);
    de(et, { ariaLabel: "authorization_code", get checked() {
      return h.authorizationCode;
    }, set checked(i) {
      h.authorizationCode = i;
    }, children: (i, Q) => {
      V();
      var A = J("authorization_code");
      c(i, A);
    }, $$slots: { default: true } });
    var tt = a(et, 2);
    de(tt, { ariaLabel: "urn:ietf:params:oauth:grant-type:device_code", get checked() {
      return h.deviceCode;
    }, set checked(i) {
      h.deviceCode = i;
    }, children: (i, Q) => {
      V();
      var A = J("device_code");
      c(i, A);
    }, $$slots: { default: true } });
    var rt = a(tt, 2);
    de(rt, { ariaLabel: "client_credentials", get checked() {
      return h.clientCredentials;
    }, set checked(i) {
      h.clientCredentials = i;
    }, children: (i, Q) => {
      V();
      var A = J("client_credentials");
      c(i, A);
    }, $$slots: { default: true } });
    var at = a(rt, 2);
    de(at, { ariaLabel: "password", get checked() {
      return h.password;
    }, set checked(i) {
      h.password = i;
    }, children: (i, Q) => {
      V();
      var A = J("password");
      c(i, A);
    }, $$slots: { default: true } });
    var lt = a(at, 2);
    de(lt, { ariaLabel: "refresh_token", get checked() {
      return h.refreshToken;
    }, set checked(i) {
      h.refreshToken = i;
    }, children: (i, Q) => {
      V();
      var A = J("refresh_token");
      c(i, A);
    }, $$slots: { default: true } });
    var it = a(lt, 2);
    re(it, "", {}, { height: ".5rem" });
    var Te = a(it, 2), qt = d(Te, true);
    s(Te);
    var Ie = a(Te, 2), nt = d(Ie), jt = d(nt, true);
    s(nt), s(Ie);
    var st = a(Ie, 2);
    de(st, { ariaLabel: "PKCE plain", get checked() {
      return P.plain;
    }, set checked(i) {
      P.plain = i;
    }, children: (i, Q) => {
      V();
      var A = J("plain");
      c(i, A);
    }, $$slots: { default: true } });
    var ot = a(st, 2);
    de(ot, { ariaLabel: "PKCE S256", get checked() {
      return P.s256;
    }, set checked(i) {
      P.s256 = i;
    }, children: (i, Q) => {
      V();
      var A = J("S256");
      c(i, A);
    }, $$slots: { default: true } });
    var dt = a(ot, 2);
    {
      var Ft = (i) => {
        var Q = Er(), A = d(Q, true);
        s(Q), D(() => p(A, n.clients.errConfidentialPKCE)), dr(3, Q, () => ur, () => ({ duration: 150 })), c(i, Q);
      };
      Z(dt, (i) => {
        !t(g) && !P.plain && !P.s256 && i(Ft);
      });
    }
    var ut = a(dt, 2);
    re(ut, "", {}, { height: ".5rem" });
    var pe = a(ut, 2), zt = d(pe);
    we(zt, () => n.clients.backchannelLogout.replace("{{ OIDC_BCL }}", '<a href="https://openid.net/specs/openid-connect-backchannel-1_0.html" target="_blank">OpenID Connect Back-Channel Logout</a>')), s(pe);
    var ct = a(pe, 2);
    _e(ct, { typ: "url", autocomplete: "off", label: "Backchannel Logout URI", placeholder: "Backchannel Logout URI", width: u, pattern: Ae, get value() {
      return t(T);
    }, set value(i) {
      l(T, i, true);
    } });
    var Re = a(ct, 2), Ht = d(Re, true);
    s(Re);
    var vt = a(Re, 2);
    xe(vt, { typ: "url", label: "Allowed Origins", get errMsg() {
      return n.validation.origin;
    }, pattern: _r, get values() {
      return t(x);
    }, set values(i) {
      l(x, i, true);
    } });
    var Ee = a(vt, 2), Kt = d(Ee);
    we(Kt, () => n.clients.descUri), s(Ee);
    var _t = a(Ee, 2);
    xe(_t, { typ: "url", label: "Redirect URIs", get errMsg() {
      return n.validation.uri;
    }, get required() {
      return h.authorizationCode;
    }, pattern: Ae, get values() {
      return t(S);
    }, set values(i) {
      l(S, i, true);
    } });
    var ht = a(_t, 2);
    xe(ht, { typ: "url", label: "Post Logout Redirect URIs", get errMsg() {
      return n.validation.uri;
    }, pattern: Ae, get values() {
      return t(q);
    }, set values(i) {
      l(q, i, true);
    } });
    var ft = a(ht, 2);
    re(ft, "", {}, { height: ".5rem" });
    var Ue = a(ft, 2), Vt = d(Ue);
    we(Vt, () => n.clients.scopes.desc), s(Ue);
    var gt = a(Ue, 2);
    It(gt, { get items() {
      return t(o);
    }, set items(i) {
      l(o, i, true);
    }, children: (i, Q) => {
      V();
      var A = J();
      D(() => p(A, n.clients.scopes.allowed)), c(i, A);
    }, $$slots: { default: true } });
    var mt = a(gt, 2);
    It(mt, { get items() {
      return t(k);
    }, set items(i) {
      l(k, i, true);
    }, children: (i, Q) => {
      V();
      var A = J();
      D(() => p(A, n.clients.scopes.default)), c(i, A);
    }, $$slots: { default: true } });
    var bt = a(mt, 2);
    re(bt, "", {}, { height: ".75rem" });
    var Me = a(bt, 2), Wt = d(Me, true);
    s(Me);
    var kt = a(Me, 2);
    _e(kt, { typ: "number", autocomplete: "off", label: "Token Lifetime", placeholder: "Token Lifetime", width: u, min: "10", max: "86400", errMsg: "10 <= Token Lifetime <= 86400", get value() {
      return t(R);
    }, set value(i) {
      l(R, i, true);
    } });
    var xt = a(kt, 2);
    re(xt, "", {}, { height: ".5rem" });
    var Be = a(xt, 2), Gt = d(Be, true);
    s(Be);
    var Ne = a(Be, 2), Jt = d(Ne, true);
    s(Ne);
    var wt = a(Ne, 2);
    Oe(wt, { label: "Access Token Algorithm", children: (i, Q) => {
      pt(i, { ariaLabel: "Access Token Algorithm", options: B, borderless: true, get value() {
        return t(K);
      }, set value(A) {
        l(K, A, true);
      } });
    }, $$slots: { default: true } });
    var yt = a(wt, 2);
    Oe(yt, { label: "ID Token Algorithm", children: (i, Q) => {
      pt(i, { ariaLabel: "ID Token Algorithm", options: B, borderless: true, get value() {
        return t(j);
      }, set value(A) {
        l(j, A, true);
      } });
    }, $$slots: { default: true } });
    var Ct = a(yt, 2);
    re(Ct, "", {}, { height: ".5rem" });
    var De = a(Ct, 2), Qt = d(De, true);
    s(De);
    var Lt = a(De, 2);
    _e(Lt, { typ: "number", autocomplete: "off", label: "Auth Code Lifetime", placeholder: "Auth Code Lifetime", width: u, min: "10", max: "300", errMsg: "10 <= Auth Code Lifetime <= 300", get value() {
      return t(I);
    }, set value(i) {
      l(I, i, true);
    } });
    var Se = a(Lt, 2);
    re(Se, "", {}, { "margin-top": "1rem" });
    var Pt = d(Se);
    oe(Pt, { type: "submit", children: (i, Q) => {
      V();
      var A = J();
      D(() => p(A, r.common.save)), c(i, A);
    }, $$slots: { default: true } });
    var Xt = a(Pt, 2);
    {
      var Yt = (i) => {
        Dt(i, {});
      };
      Z(Xt, (i) => {
        t(C) && i(Yt);
      });
    }
    s(Se);
    var St = a(Se, 2);
    {
      var Zt = (i) => {
        var Q = Ur(), A = d(Q, true);
        s(Q), D(() => p(A, t(v))), c(i, Q);
      };
      Z(St, (i) => {
        t(v) && i(Zt);
      });
    }
    var $t = a(St, 2);
    re($t, "", {}, { height: "1rem" }), D(() => {
      p(M, n.common.information), p(ve, n.clients.descName), p(te, n.clients.descClientUri), p(Ot, n.clients.config), p(qt, n.clients.descPKCE), p(jt, n.clients.descPKCEEnforce), p(Ht, n.clients.descOrigin), p(Wt, n.clients.tokenLifetime.p1), p(Gt, n.clients.tokenLifetime.p2), p(Jt, n.clients.tokenLifetime.p3), p(Qt, n.clients.descAuthCode);
    }), c(G, X);
  }, $$slots: { default: true } }), s(m), c(E, m), ne();
}
var Dr = O("<!> <!>", 1), Or = O('<div class="container svelte-19xcrje"><div class="err"> </div> <!></div>');
function qr(E, e) {
  ie(e, true);
  let r = me(), n = w(""), u = w(void 0);
  fe(() => {
    l(n, ""), l(u, ""), e.client.confidential ? v() : l(n, r.clients.confidentialNoSecret, true);
  });
  async function v() {
    let y = await Ge(`/auth/v1/clients/${e.client.id}/secret`);
    _(y);
  }
  async function C() {
    let y = await Je(`/auth/v1/clients/${e.client.id}/secret`);
    _(y);
  }
  function _(y) {
    var _a;
    y.body ? y.body.secret && l(u, y.body.secret, true) : l(n, ((_a = y.error) == null ? void 0 : _a.message) || "Error", true);
  }
  var b = Or(), L = d(b), g = d(L, true);
  s(L);
  var f = a(L, 2);
  {
    var T = (y) => {
      var x = Dr(), S = ue(x);
      fr(S, { autocomplete: "off", label: "Client Secret", placeholder: "Client Secret", disabled: true, showCopy: true, get value() {
        return t(u);
      }, set value(h) {
        l(u, h, true);
      } });
      var q = a(S, 2);
      oe(q, { onclick: C, children: (h, B) => {
        V();
        var K = J();
        D(() => p(K, r.clients.generateSecret)), c(h, K);
      }, $$slots: { default: true } }), c(y, x);
    };
    Z(f, (y) => {
      t(u) && y(T);
    });
  }
  s(b), D(() => p(g, t(n))), c(E, b), ne();
}
var jr = O('<div class="err"> </div>'), Fr = O("<p> </p> <!> <!>", 1);
function zr(E, e) {
  ie(e, true);
  let r = ye(), n = me(), u = w("");
  async function v() {
    l(u, "");
    let T = await Ke(`/auth/v1/clients/${e.client.id}`);
    T.error ? l(u, T.error.message, true) : e.onSave();
  }
  var C = Fr(), _ = ue(C), b = d(_, true);
  s(_);
  var L = a(_, 2);
  oe(L, { level: -1, onclick: v, children: (T, y) => {
    V();
    var x = J();
    D(() => p(x, r.common.delete)), c(T, x);
  }, $$slots: { default: true } });
  var g = a(L, 2);
  {
    var f = (T) => {
      var y = jr(), x = d(y, true);
      s(y), D(() => p(x, t(u))), c(T, y);
    };
    Z(g, (T) => {
      t(u) && T(f);
    });
  }
  D(() => p(b, n.clients.delete1)), c(E, C), ne();
}
var Hr = O('<div><div class="flex"><input type="range" class="svelte-12j8dit"> <div class="value font-mono svelte-12j8dit"> </div></div> <div class="label svelte-12j8dit"><label class="font-label noselect svelte-12j8dit"> </label></div></div>');
function je(E, e) {
  ie(e, true);
  let r = Y(e, "value", 15), n = Y(e, "label", 3, ""), u = Y(e, "disabled", 3, false), v = Y(e, "step", 3, 1), C = Y(e, "widthRange", 3, "15rem");
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
  var L = Hr(), g = d(L), f = d(g);
  We(f), z(f, "id", _);
  let T;
  var y = a(f, 2), x = d(y, true);
  s(y), s(g);
  var S = a(g, 2), q = d(S);
  z(q, "for", _);
  var h = d(q, true);
  s(q), s(S), s(L), D(() => {
    z(f, "name", e.name), z(f, "title", n()), z(f, "aria-label", n()), f.disabled = u(), z(f, "aria-disabled", u()), z(f, "min", e.min), z(f, "max", e.max), z(f, "step", v()), T = re(f, "", T, { width: C(), "--bg-slider": t(b) }), p(x, r()), p(h, n());
  }), Xe(f, r), c(E, L), ne();
}
var Kr = O('<div class="outer svelte-1evkwuy"><div class="container svelte-1evkwuy"><div><label class="font-label svelte-1evkwuy"> </label> <div class="values svelte-1evkwuy"><!> <!> <!></div></div> <div class="color svelte-1evkwuy"></div></div></div>');
function be(E, e) {
  ie(e, true);
  let r = Y(e, "h", 15), n = Y(e, "s", 15), u = Y(e, "l", 15);
  const v = ke(), C = "15rem";
  let _ = he(() => `hsl(${r()} ${n()} ${u()})`);
  var b = Kr(), L = d(b);
  let g;
  var f = d(L), T = d(f);
  z(T, "for", v);
  var y = d(T, true);
  s(T);
  var x = a(T, 2);
  z(x, "id", v);
  var S = d(x);
  je(S, { label: "Hue", min: 0, max: 359, widthRange: C, bgMode: "hue", get hue() {
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
  var q = a(S, 2);
  je(q, { label: "Sat", min: 0, max: 100, widthRange: C, bgMode: "sat", get hue() {
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
  var h = a(q, 2);
  je(h, { label: "Lum", min: 0, max: 100, widthRange: C, bgMode: "lum", get hue() {
    return r();
  }, get sat() {
    return n();
  }, get lum() {
    return u();
  }, get value() {
    return u();
  }, set value(j) {
    u(j);
  } }), s(x), s(f);
  var B = a(f, 2);
  let K;
  s(L), s(b), D(() => {
    g = re(L, "", g, { "border-color": t(_) }), p(y, e.label), K = re(B, "", K, { background: t(_) });
  }), c(E, b), ne();
}
var Vr = (E, e) => {
  var _a;
  return (_a = t(e)) == null ? void 0 : _a.click();
}, Wr = O('<div class="container svelte-2nx18n"><!> <div role="none" class="mask svelte-2nx18n"><div class="colorWrapper svelte-2nx18n"><div class="color svelte-2nx18n"></div></div> <input type="color" class="svelte-2nx18n"></div></div>');
function Fe(E, e) {
  ie(e, true);
  let r = Y(e, "value", 15), n = me(), u = w(void 0);
  var v = Wr();
  let C;
  var _ = d(v);
  _e(_, { get label() {
    return e.label;
  }, get placeholder() {
    return e.label;
  }, get errMsg() {
    return n.validation.css;
  }, width: "17.5rem", required: true, pattern: Mt, get value() {
    return r();
  }, set value(g) {
    r(g);
  } });
  var b = a(_, 2);
  b.__click = [Vr, u];
  var L = a(d(b), 2);
  We(L), Bt(L, (g) => l(u, g), () => t(u)), s(b), s(v), D(() => C = re(v, "", C, { "--color": r() })), Xe(L, r), c(E, v), ne();
}
Ve(["click"]);
var Gr = O('<div><p> </p> <div class="hsl svelte-10udpph"><div><!> <!> <!> <!></div> <div><!> <!> <!></div></div> <div><p><!></p> <!> <!> <!></div></div>');
function Rt(E, e) {
  ie(e, true);
  let r = Y(e, "values", 15), n = me();
  var u = Gr(), v = d(u), C = d(v, true);
  s(v);
  var _ = a(v, 2), b = d(_), L = d(b);
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
  var f = a(g, 2);
  be(f, { label: "--bg", get h() {
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
  var T = a(f, 2);
  be(T, { label: "--bg-high", get h() {
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
  } }), s(b);
  var y = a(b, 2), x = d(y);
  be(x, { label: "--action", get h() {
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
  var S = a(x, 2);
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
  var q = a(S, 2);
  be(q, { label: "--error", get h() {
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
  } }), s(y), s(_);
  var h = a(_, 2), B = d(h), K = d(B);
  we(K, () => n.clients.branding.descFullCss), s(B);
  var j = a(B, 2);
  Fe(j, { label: "--btn-text", get value() {
    return r().btn_text;
  }, set value(o) {
    r(r().btn_text = o, true);
  } });
  var R = a(j, 2);
  Fe(R, { label: "--theme-sun", get value() {
    return r().theme_sun;
  }, set value(o) {
    r(r().theme_sun = o, true);
  } });
  var I = a(R, 2);
  Fe(I, { label: "--theme-moon", get value() {
    return r().theme_moon;
  }, set value(o) {
    r(r().theme_moon = o, true);
  } }), s(h), s(u), D(() => p(C, n.clients.branding.descHsl)), c(E, u), ne();
}
var Jr = O('<img alt="Client Logo" width="100%" height="100%">'), Qr = (E) => E.preventDefault(), Xr = O('<div aria-label="Preview: All components inside are only for theme and colors preview and have no effect or interaction"><div class="inner svelte-fkmn75"><div class="container svelte-fkmn75"><div class="logo svelte-fkmn75"><!></div> <h3 class="svelte-fkmn75">Header</h3> <p>Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam</p> <p><span class="svelte-fkmn75">--accent-color</span></p> <!> <div class="btn svelte-fkmn75"><div class="svelte-fkmn75"><!> <br> <!> <br> <!> <br></div> <div class="svelte-fkmn75"><!> <br> <!> <br> <!></div></div> <a>Link</a> <br> <!></div></div></div>');
function Et(E, e) {
  ie(e, true);
  let r = w(false), n = w(false);
  fe(() => {
    e.logoUrl && l(n, false);
  }), fe(() => {
    t(r) && setTimeout(() => {
      l(r, false);
    }, 2e3);
  });
  var u = Xr();
  let v;
  var C = d(u), _ = d(C), b = d(_), L = d(b);
  {
    var g = (k) => {
      br(k, { width: "100%", get show() {
        return e.typ;
      } });
    }, f = (k) => {
      var P = Jr();
      D(() => z(P, "src", e.logoUrl)), ze("error", P, () => l(n, true)), rr(P), c(k, P);
    };
    Z(L, (k) => {
      t(n) ? k(g) : k(f, false);
    });
  }
  s(b);
  var T = a(b, 8);
  _e(T, { label: "Preview", placeholder: "Preview", width: "12.5rem" });
  var y = a(T, 2), x = d(y), S = d(x);
  oe(S, { level: 1, get isLoading() {
    return t(r);
  }, onclick: () => l(r, true), children: (k, P) => {
    V();
    var N = J("Button");
    c(k, N);
  }, $$slots: { default: true } });
  var q = a(S, 4);
  oe(q, { level: 2, get isLoading() {
    return t(r);
  }, onclick: () => l(r, true), children: (k, P) => {
    V();
    var N = J("Button");
    c(k, N);
  }, $$slots: { default: true } });
  var h = a(q, 4);
  oe(h, { level: 3, get isLoading() {
    return t(r);
  }, onclick: () => l(r, true), children: (k, P) => {
    V();
    var N = J("Button");
    c(k, N);
  }, $$slots: { default: true } }), V(2), s(x);
  var B = a(x, 2), K = d(B);
  oe(K, { level: -1, get isLoading() {
    return t(r);
  }, onclick: () => l(r, true), children: (k, P) => {
    V();
    var N = J("Button");
    c(k, N);
  }, $$slots: { default: true } });
  var j = a(K, 4);
  oe(j, { level: -2, get isLoading() {
    return t(r);
  }, onclick: () => l(r, true), children: (k, P) => {
    V();
    var N = J("Button");
    c(k, N);
  }, $$slots: { default: true } });
  var R = a(j, 4);
  oe(R, { level: -3, get isLoading() {
    return t(r);
  }, onclick: () => l(r, true), children: (k, P) => {
    V();
    var N = J("Button");
    c(k, N);
  }, $$slots: { default: true } }), s(B), s(y);
  var I = a(y, 2);
  z(I, "href", window.location.href), I.__click = [Qr];
  var o = a(I, 4);
  mr(o, {}), s(_), s(C), s(u), D((k, P, N, U, m, W, ae) => v = re(u, "", v, { "--border-radius": e.borderRadius, "--text": k, "--text-high": P, "--bg": N, "--bg-high": U, "--action": m, "--accent": W, "--error": ae, "--btn-text": e.theme.btn_text, "--theme-sun": e.theme.theme_sun, "--theme-moon": e.theme.theme_moon }), [() => e.theme.text.join(" "), () => e.theme.text_high.join(" "), () => e.theme.bg.join(" "), () => e.theme.bg_high.join(" "), () => e.theme.action.join(" "), () => e.theme.accent.join(" "), () => e.theme.error.join(" ")]), c(E, u), ne();
}
Ve(["click"]);
var Yr = O('<div><h2>Preview</h2> <div class="tabs svelte-1klemk8"><!></div> <div class="preview svelte-1klemk8"><!></div></div>');
function Zr(E, e) {
  ie(e, true);
  let r = ["Light Theme", "Dark Theme"], n = w(H(r[0]));
  var u = Yr(), v = a(d(u), 2), C = d(v);
  Nt(C, { tabs: r, center: true, get selected() {
    return t(n);
  }, set selected(f) {
    l(n, f, true);
  } }), s(v);
  var _ = a(v, 2), b = d(_);
  {
    var L = (f) => {
      Et(f, { get logoUrl() {
        return e.logoUrl;
      }, get borderRadius() {
        return e.theme.border_radius;
      }, get theme() {
        return e.theme.light;
      }, typ: "light" });
    }, g = (f) => {
      Et(f, { get logoUrl() {
        return e.logoUrl;
      }, get borderRadius() {
        return e.theme.border_radius;
      }, get theme() {
        return e.theme.dark;
      }, typ: "dark" });
    };
    Z(b, (f) => {
      t(n) === r[0] ? f(L) : f(g, false);
    });
  }
  s(_), s(u), c(E, u), ne();
}
var $r = O('<div class="err"> </div>'), ea = O('<div class="values svelte-1pkqupw"><p><!></p> <!> <h1>Light Theme</h1> <!> <br> <h1>Dark Theme</h1> <!></div> <div class="preview"><!> <hr> <p>Logo Upload</p> <!> <div class="buttons svelte-1pkqupw"><!> <!> <!></div> <!></div>', 1), ta = O('<div class="container svelte-1pkqupw"><!></div>');
function ra(E, e) {
  ie(e, true);
  const r = "13rem";
  let n = ye(), u = me(), v = w(false), C = w(""), _ = w(void 0), b = w(H(ke())), L = he(() => `/auth/v1/clients/${e.client.id}/logo?${t(b)}`), g = he(() => `/auth/v1/theme/${e.client.id}`);
  fe(() => {
    f();
  });
  async function f() {
    var _a;
    let S = await Ge(t(g));
    S.body ? l(_, S.body, true) : l(C, ((_a = S.error) == null ? void 0 : _a.message) || "Error", true);
  }
  async function T(S, q) {
    if (!t(_)) {
      console.error("theme is undefined");
      return;
    }
    let h = t(_);
    h.client_id = e.client.id;
    let B = await Je(t(g), h);
    B.error ? l(C, B.error.message, true) : (l(v, true), setTimeout(() => {
      l(v, false);
    }, 2e3));
  }
  async function y() {
    let S = await Ke(t(g));
    S.error ? l(C, S.error.message, true) : (await Ke(`/auth/v1/clients/${e.client.id}/logo`), await f(), l(b, ke(), true), l(v, true), setTimeout(() => {
      l(v, false);
    }, 2e3));
  }
  async function x() {
    l(b, ke(), true);
  }
  Qe(E, { get action() {
    return t(g);
  }, onSubmit: T, children: (S, q) => {
    var h = ta(), B = d(h);
    {
      var K = (j) => {
        var R = ea(), I = ue(R), o = d(I), k = d(o);
        we(k, () => u.clients.branding.descVariables), s(o);
        var P = a(o, 2);
        _e(P, { label: "--border-radius", placeholder: "--border-radius", get errMsg() {
          return u.validation.css;
        }, width: r, required: true, pattern: Mt, get value() {
          return t(_).border_radius;
        }, set value(F) {
          t(_).border_radius = F;
        } });
        var N = a(P, 4);
        Rt(N, { get values() {
          return t(_).light;
        }, set values(F) {
          t(_).light = F;
        } });
        var U = a(N, 6);
        Rt(U, { get values() {
          return t(_).dark;
        }, set values(F) {
          t(_).dark = F;
        } }), s(I);
        var m = a(I, 2), W = d(m);
        gr(W, () => t(b), (F) => {
          Zr(F, { get logoUrl() {
            return t(L);
          }, get theme() {
            return t(_);
          } });
        });
        var ae = a(W, 6);
        const G = he(() => `/auth/v1/clients/${e.client.id}/logo`);
        kr(ae, { method: "PUT", get url() {
          return t(G);
        }, fileName: "logo", onSuccess: x });
        var se = a(ae, 2), X = d(se);
        oe(X, { type: "submit", children: (F, ee) => {
          V();
          var te = J();
          D(() => p(te, n.common.save)), c(F, te);
        }, $$slots: { default: true } });
        var le = a(X, 2);
        oe(le, { level: -2, onclick: y, children: (F, ee) => {
          V();
          var te = J();
          D(() => p(te, u.common.reset)), c(F, te);
        }, $$slots: { default: true } });
        var M = a(le, 2);
        {
          var $ = (F) => {
            Dt(F, {});
          };
          Z(M, (F) => {
            t(v) && F($);
          });
        }
        s(se);
        var ce = a(se, 2);
        {
          var ve = (F) => {
            var ee = $r(), te = d(ee, true);
            s(ee), D(() => p(te, t(C))), c(F, ee);
          };
          Z(ce, (F) => {
            t(C) && F(ve);
          });
        }
        s(m), c(j, R);
      };
      Z(B, (j) => {
        t(_) && j(K);
      });
    }
    s(h), c(S, h);
  }, $$slots: { default: true } }), ne();
}
var aa = O('<div class="flex"><!></div> <div class="details"><!></div>', 1);
function la(E, e) {
  ie(e, true);
  let r = ye(), n = me();
  const u = [n.nav.config, "Secret", "Branding", r.common.delete];
  let v = w(H(u[0])), C = w(void 0);
  fe(() => {
    e.client.id && requestAnimationFrame(() => {
      var _a;
      (_a = t(C)) == null ? void 0 : _a();
    });
  });
  var _ = aa(), b = ue(_), L = d(b);
  Nt(L, { tabs: u, get selected() {
    return t(v);
  }, set selected(x) {
    l(v, x, true);
  }, get focusFirst() {
    return t(C);
  }, set focusFirst(x) {
    l(C, x, true);
  } }), s(b);
  var g = a(b, 2), f = d(g);
  {
    var T = (x) => {
      Nr(x, { get client() {
        return e.client;
      }, get clients() {
        return e.clients;
      }, get scopesAll() {
        return e.scopesAll;
      }, get onSave() {
        return e.onSave;
      } });
    }, y = (x, S) => {
      {
        var q = (B) => {
          qr(B, { get client() {
            return e.client;
          } });
        }, h = (B, K) => {
          {
            var j = (I) => {
              ra(I, { get client() {
                return e.client;
              } });
            }, R = (I, o) => {
              {
                var k = (P) => {
                  zr(P, { get client() {
                    return e.client;
                  }, get onSave() {
                    return e.onSave;
                  } });
                };
                Z(I, (P) => {
                  t(v) === r.common.delete && P(k);
                }, o);
              }
            };
            Z(B, (I) => {
              t(v) === "Branding" ? I(j) : I(R, false);
            }, K);
          }
        };
        Z(x, (B) => {
          t(v) === "Secret" ? B(q) : B(h, false);
        }, S);
      }
    };
    Z(f, (x) => {
      t(v) === n.nav.config ? x(T) : x(y, false);
    });
  }
  s(g), c(E, _), ne();
}
var ia = O('<div class="tile svelte-140q9n9"> <div class="muted svelte-140q9n9"> </div></div>'), na = O("<div></div> <!>", 1), sa = O("<!> <!>", 1), oa = O('<div class="err"> </div>'), da = O('<!> <div id="groups"><!></div>', 1), ua = O("<!> <!>", 1);
function ca(E, e) {
  ie(e, true);
  let r = w(void 0), n = w(""), u = w(H([])), v = w(H([])), C = w(void 0), _ = or("cid"), b = w(H([]));
  const L = ["ID"];
  let g = w(H(L[0])), f = w("");
  const T = ["ID"];
  tr(() => {
    y(), x();
  }), fe(() => {
    l(C, t(u).find((R) => R.id === _.get()), true);
  }), fe(() => {
    let R = t(f).toLowerCase();
    R ? t(g) === L[0] && l(v, t(u).filter((I) => I.id.toLowerCase().includes(R)), true) : l(v, t(u), true);
  });
  async function y() {
    var _a;
    let R = await Tt("/auth/v1/clients");
    R.body ? l(u, R.body, true) : l(n, ((_a = R.error) == null ? void 0 : _a.message) || "Error", true);
  }
  async function x() {
    var _a;
    let R = await Tt("/auth/v1/scopes");
    R.body ? l(b, R.body.map((I) => I.name), true) : l(n, ((_a = R.error) == null ? void 0 : _a.message) || "Error", true);
  }
  function S(R, I) {
    let o = I === "up";
    R === T[0] && t(u).sort((k, P) => o ? k.id.localeCompare(P.id) : P.id.localeCompare(k.id));
  }
  function q() {
    y();
  }
  async function h(R) {
    var _a;
    (_a = t(r)) == null ? void 0 : _a(), await y(), _.set(R);
  }
  var B = ua(), K = ue(B);
  nr(K, { paddingTop: "2.1rem", buttonTilesAriaControls: "groups", width: "min(20rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (I) => {
    var o = na(), k = ue(o);
    re(k, "", {}, { height: ".5rem" });
    var P = a(k, 2);
    He(P, 17, () => t(v), (N) => N.id, (N, U, m, W) => {
      const ae = he(() => _.get() === t(U).id);
      ir(N, { onclick: () => _.set(t(U).id), get selected() {
        return t(ae);
      }, children: (G, se) => {
        var X = ia(), le = d(X), M = a(le), $ = d(M, true);
        s(M), s(X), D(() => {
          p(le, `${t(U).id ?? ""} `), p($, t(U).name);
        }), c(G, X);
      } });
    }), c(I, o);
  }, children: (I, o) => {
    var k = sa(), P = ue(k);
    const N = he(() => t(u).length === 0 ? 1 : 2);
    ar(P, { get level() {
      return t(N);
    }, alignRight: true, get closeModal() {
      return t(r);
    }, set closeModal(m) {
      l(r, m, true);
    }, children: (m, W) => {
      Rr(m, { onSave: h, get clients() {
        return t(u);
      } });
    }, $$slots: { default: true } });
    var U = a(P, 2);
    sr(U, { searchOptions: L, orderOptions: T, onChangeOrder: S, searchWidth: "min(19.5rem, 100dvw - .5rem)", get searchOption() {
      return t(g);
    }, set searchOption(m) {
      l(g, m, true);
    }, get value() {
      return t(f);
    }, set value(m) {
      l(f, m, true);
    } }), c(I, k);
  }, $$slots: { buttonTiles: true, default: true } });
  var j = a(K, 2);
  lr(j, { children: (R, I) => {
    var o = da(), k = ue(o);
    {
      var P = (W) => {
        var ae = oa(), G = d(ae, true);
        s(ae), D(() => p(G, t(n))), c(W, ae);
      };
      Z(k, (W) => {
        t(n) && W(P);
      });
    }
    var N = a(k, 2), U = d(N);
    {
      var m = (W) => {
        la(W, { get client() {
          return t(C);
        }, get clients() {
          return t(u);
        }, get scopesAll() {
          return t(b);
        }, onSave: q });
      };
      Z(U, (W) => {
        t(C) && W(m);
      });
    }
    s(N), c(R, o);
  } }), c(E, B), ne();
}
function Ya(E) {
  ca(E, {});
}
export {
  Ya as component
};
