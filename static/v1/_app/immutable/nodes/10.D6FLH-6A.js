import { t as O, a as u, d as Dr, e as J } from "../chunks/BxmJRzoY.js";
import { p as ie, a1 as H, j as b, a4 as ve, c as o, s as a, t as N, l as n, k as t, a as ne, r as s, f as fe, a5 as me, a3 as G, a0 as Nr } from "../chunks/w0HvPX0p.js";
import { d as at, e as $e, s as T, b as qr } from "../chunks/BzP2S3Z_.js";
import { i as Y } from "../chunks/iO9_dPNE.js";
import { e as et, i as Jt } from "../chunks/S81raU5Y.js";
import { r as lt, a as Z, s as V } from "../chunks/BdbQ6g_y.js";
import "../chunks/DM69BKKN.js";
import { B as Or } from "../chunks/B5QJ0QFB.js";
import { C as jr } from "../chunks/C6jTHtu1.js";
import { N as Fr } from "../chunks/UWnXu0Z3.js";
import { N as zr } from "../chunks/Bmyn6g0M.js";
import { O as Kr } from "../chunks/Ssae7OHr.js";
import { b as it, f as tt, d as nt, c as rt } from "../chunks/UPFlzoow.js";
import { u as Hr } from "../chunks/DvTygCXn.js";
import { h as ce } from "../chunks/C2ZdIFW_.js";
import { B as de, t as Ge, s as Je } from "../chunks/C4AV2CoD.js";
import { I as ue } from "../chunks/Bk5EVqw2.js";
import { n as Vr, g as $t, a as pe, o as Wr, p as Gr, f as Jr, q as er } from "../chunks/gfDO7tLr.js";
import { F as st } from "../chunks/CfHEvq46.js";
import { u as Le } from "../chunks/N6FgGI8m.js";
import { u as ke } from "../chunks/D12OFlGX.js";
import { I as oe } from "../chunks/ScYc5fRW.js";
import { b as ot } from "../chunks/Cxw7xmE1.js";
import { b as tr } from "../chunks/Dgjj26O5.js";
import { p as $ } from "../chunks/C6GSeq7M.js";
import { g as ye } from "../chunks/B21bTIl7.js";
import { I as Qr } from "../chunks/QCVRj9pj.js";
import { T as rr } from "../chunks/Dgt5DyOi.js";
import { I as ar } from "../chunks/CTshzOVc.js";
import { L as Qe } from "../chunks/D8MSWRQv.js";
import { S as Qt } from "../chunks/cmE-Qqfe.js";
import { O as Xt } from "../chunks/BgkEPAwa.js";
import { I as lr } from "../chunks/CfKSJy9E.js";
import { k as Xr } from "../chunks/DDNkWuIk.js";
import { T as Yr } from "../chunks/B808p9S3.js";
import { R as Zr } from "../chunks/BsfuR8cO.js";
import { I as $r } from "../chunks/Bf_ebPe0.js";
import "../chunks/F_Qf1tHt.js";
function ea(D, e, r, l) {
  var _a2;
  e(), (_a2 = r.onInput) == null ? void 0 : _a2.call(r, t(l));
}
function ta(D, e, r) {
  var _a2;
  switch (D.code) {
    case "Enter":
      D.preventDefault(), e(), (_a2 = r.onEnter) == null ? void 0 : _a2.call(r);
      break;
  }
}
function ra(D, e) {
  var _a2, _b;
  (_a2 = e()) == null ? void 0 : _a2.focus(), (_b = e()) == null ? void 0 : _b.scrollIntoView({ behavior: "smooth", block: "center" });
}
var aa = O('<li class="value svelte-r5y8pk"><div class="label"> </div> <div class="btnClose svelte-r5y8pk"><!></div></li>'), la = O("<option></option>"), ia = O('<datalist class="absolute svelte-r5y8pk"></datalist>'), na = O('<span class="err"> </span>'), sa = O('<div role="none" class="container svelte-r5y8pk"><ul class="svelte-r5y8pk"><!> <li class="svelte-r5y8pk"><input autocomplete="off" class="svelte-r5y8pk"> <!></li></ul> <label class="font-label noselect svelte-r5y8pk"> <!></label></div>');
function Ce(D, e) {
  ie(e, true);
  let r = $(e, "ref", 15), l = $(e, "typ", 3, "text"), c = $(e, "id", 19, ye), v = $(e, "values", 31, () => H([])), C = $(e, "label", 3, ""), _ = $(e, "placeholder", 3, ""), k = $(e, "disabled", 3, false), L = $(e, "required", 3, false), g = $(e, "isError", 15, false), f = $(e, "width", 3, "inherit"), A = $(e, "maxHeightList", 3, "inherit"), w = Le();
  const p = ye();
  let I = b(""), j = ve(() => e.datalist && e.datalist.length > 0 ? p : void 0);
  function E(U) {
    v(v().filter((F) => F !== U));
  }
  function x(U) {
    var _a2;
    M(), (_a2 = e.onBlur) == null ? void 0 : _a2.call(e), W();
  }
  function S(U) {
    U.preventDefault(), g(true);
  }
  function W() {
    t(I) && M() && (v().push(t(I)), n(I, ""));
  }
  function M() {
    var _a2;
    let U = (_a2 = r()) == null ? void 0 : _a2.validity;
    return U ? (g(!U.valid), U.valid) : (g(false), true);
  }
  var R = sa();
  R.__click = [ra, r];
  let d;
  var y = o(R);
  let B;
  var q = o(y);
  et(q, 17, v, Jt, (U, F, se, ee) => {
    var z = aa(), te = o(z), re = o(te, true);
    s(te);
    var be = a(te, 2), Pe = o(be);
    de(Pe, { invisible: true, onclick: () => E(t(F)), children: (Re, Ee) => {
      Qr(Re, { width: "1.2rem" });
    }, $$slots: { default: true } }), s(be), s(z), N(() => T(re, t(F))), u(U, z);
  });
  var P = a(q, 2), h = o(P);
  lt(h), h.__keydown = [ta, W, e], h.__input = [ea, M, e, I], tr(h, (U) => r(U), () => r());
  var X = a(h, 2);
  {
    var ae = (U) => {
      var F = ia();
      V(F, "id", p), et(F, 21, () => e.datalist, Jt, (se, ee, z, te) => {
        var re = la(), be = {};
        N(() => {
          be !== (be = t(ee)) && (re.value = (re.__value = t(ee)) == null ? "" : t(ee));
        }), u(se, re);
      }), s(F), u(U, F);
    };
    Y(X, (U) => {
      e.datalist && e.datalist.length > 1 && U(ae);
    });
  }
  s(P), s(y);
  var le = a(y, 2), he = o(le), ge = a(he);
  {
    var Q = (U) => {
      var F = Dr(), se = fe(F);
      {
        var ee = (z) => {
          var te = na(), re = o(te, true);
          s(te), N(() => T(re, e.errMsg || w.common.invalidInput)), u(z, te);
        };
        Y(se, (z) => {
          g() && z(ee);
        });
      }
      u(U, F);
    };
    Y(ge, (U) => {
      g() && U(Q);
    });
  }
  s(le), s(R), N(() => {
    d = Z(R, "", d, { width: f() }), B = Z(y, "", B, { "max-height": A() }), V(h, "type", l()), V(h, "id", c()), V(h, "name", e.name), V(h, "list", t(j)), V(h, "title", e.errMsg), V(h, "aria-label", C() || _()), V(h, "placeholder", _()), V(h, "aria-placeholder", _()), h.disabled = k(), V(h, "aria-disabled", k()), h.required = L() && v().length < 1, V(h, "aria-required", L() && v().length < 1), V(h, "maxlength", e.maxLength || void 0), V(h, "min", e.min || void 0), V(h, "max", e.max || void 0), V(h, "pattern", e.pattern), V(le, "for", c()), V(le, "data-required", L()), T(he, `${C() ?? ""} `);
  }), $e("invalid", h, S), $e("blur", h, x), ot(h, () => t(I), (U) => n(I, U)), u(D, R), ne();
}
at(["click", "keydown", "input"]);
var oa = O('<div class="err"> </div>'), da = O('<!> <!> <!> <p class="svelte-15tp6i6"><!></p> <!> <!> <!> <!>', 1), ca = O('<div class="container svelte-15tp6i6"><!></div>');
function ua(D, e) {
  ie(e, true);
  let r = Le(), l = ke(), c = b(void 0), v = b(""), C = b(""), _ = b(""), k = b(true), L = b(H([])), g = b(H([]));
  me(() => {
    requestAnimationFrame(() => {
      var _a2;
      (_a2 = t(c)) == null ? void 0 : _a2.focus();
    });
  });
  async function f(p, I) {
    var _a2;
    if (e.clients.find((x) => x.id === t(C))) {
      n(v, l.common.nameExistsAlready, true);
      return;
    }
    n(v, "");
    let j = { id: t(C), name: t(_) || void 0, confidential: t(k), redirect_uris: t(L), post_logout_redirect_uris: t(g).length > 0 ? t(g) : void 0 }, E = await it(p.action, j);
    E.body ? e.onSave(E.body.id) : n(v, ((_a2 = E.error) == null ? void 0 : _a2.message) || "Error", true);
  }
  var A = ca(), w = o(A);
  st(w, { action: "/auth/v1/clients", onSubmit: f, children: (p, I) => {
    var j = da(), E = fe(j);
    ue(E, { autocomplete: "off", label: "Client ID", placeholder: "Client ID", required: true, pattern: Vr, get ref() {
      return t(c);
    }, set ref(P) {
      n(c, P, true);
    }, get value() {
      return t(C);
    }, set value(P) {
      n(C, P, true);
    } });
    var x = a(E, 2);
    ue(x, { autocomplete: "off", get label() {
      return l.clients.name;
    }, get placeholder() {
      return l.clients.name;
    }, pattern: $t, get value() {
      return t(_);
    }, set value(P) {
      n(_, P, true);
    } });
    var S = a(x, 2);
    oe(S, { get ariaLabel() {
      return l.clients.confidential;
    }, get checked() {
      return t(k);
    }, set checked(P) {
      n(k, P, true);
    }, children: (P, h) => {
      G();
      var X = J();
      N(() => T(X, l.clients.confidential)), u(P, X);
    }, $$slots: { default: true } });
    var W = a(S, 2), M = o(W);
    ce(M, () => l.clients.descUri), s(W);
    var R = a(W, 2);
    Ce(R, { typ: "url", label: "Redirect URIs", get errMsg() {
      return l.validation.uri;
    }, get values() {
      return t(L);
    }, set values(P) {
      n(L, P, true);
    } });
    var d = a(R, 2);
    Ce(d, { typ: "url", label: "Post Logout Redirect URIs", get errMsg() {
      return l.validation.uri;
    }, get values() {
      return t(g);
    }, set values(P) {
      n(g, P, true);
    } });
    var y = a(d, 2);
    de(y, { type: "submit", children: (P, h) => {
      G();
      var X = J();
      N(() => T(X, r.common.save)), u(P, X);
    }, $$slots: { default: true } });
    var B = a(y, 2);
    {
      var q = (P) => {
        var h = oa(), X = o(h, true);
        s(h), N(() => T(X, t(v))), u(P, h);
      };
      Y(B, (P) => {
        t(v) && P(q);
      });
    }
    u(p, j);
  }, $$slots: { default: true } }), s(A), u(D, A), ne();
}
const Xe = "urn:ietf:params:oauth:grant-type:device_code";
var va = O('<div class="err"> </div>'), _a = O('<div><p class="mb-0 svelte-5tvvvt"><!></p> <!></div>'), fa = O('<div><p class="desc svelte-5tvvvt"> </p> <ul><li><!></li> <li><!></li> <li><!></li></ul> <p class="mb-0 svelte-5tvvvt"><!></p> <!> <!> <div></div> <!> <!></div>'), ga = O('<div class="err"> </div>'), ha = O('<h5> </h5> <!> <p class="desc svelte-5tvvvt"> </p> <!> <p class="desc svelte-5tvvvt"> </p> <!> <!> <div></div> <h5> </h5> <!> <!> <!> <p class="mb-0 svelte-5tvvvt"><b>Authentication Flows</b></p> <!> <!> <!> <!> <!> <div></div> <p class="mb-0 svelte-5tvvvt"><b>PKCE</b></p> <p class="desc svelte-5tvvvt"> </p> <p class="desc svelte-5tvvvt"><strong> </strong></p> <!> <!> <!> <div></div> <p class="mb-0 svelte-5tvvvt"><b>Origin</b></p> <p class="desc svelte-5tvvvt"> </p> <!> <p class="desc svelte-5tvvvt"><!></p> <!> <!> <div></div> <p class="mb-0 svelte-5tvvvt"><b>Scopes</b></p> <p class="desc svelte-5tvvvt"><!></p> <!> <!> <div></div> <p class="mb-0 svelte-5tvvvt"><b>Tokens</b></p> <p> </p> <!> <div></div> <p> </p> <p> </p> <!> <!> <div></div> <p> </p> <!> <p class="mb-0 svelte-5tvvvt"><b>Backchannel Logout</b></p> <p class="desc svelte-5tvvvt"><!></p> <!> <p class="mb-0 svelte-5tvvvt"><b>SCIM</b></p> <p class="desc svelte-5tvvvt"><!></p> <!> <!> <div class="flex gap-05"><!> <!></div> <!> <div></div>', 1), ma = O('<div class="container svelte-5tvvvt"><!></div>');
function ba(D, e) {
  var _a2, _b, _c, _d, _e, _f;
  ie(e, true);
  let r = Le(), l = ke();
  const c = "min(20rem, calc(100dvw - .5rem))";
  let v = b(""), C = b(false), _ = b(H(e.client.id)), k = b(H(e.client.name || "")), L = b(H(e.client.enabled)), g = b(H(e.client.confidential)), f = b(H(e.client.client_uri || "")), A = b(H(e.client.contacts ? Array.from(e.client.contacts) : [])), w = b(H(e.client.allowed_origins ? Array.from(e.client.allowed_origins) : [])), p = b(H(Array.from(e.client.redirect_uris))), I = b(H(e.client.post_logout_redirect_uris ? Array.from(e.client.post_logout_redirect_uris) : [])), j = b(H(e.client.backchannel_logout_uri || "")), E = b(e.client.scim !== void 0), x = b(H({ base_uri: ((_a2 = e.client.scim) == null ? void 0 : _a2.base_uri) || "", bearer_token: ((_b = e.client.scim) == null ? void 0 : _b.bearer_token) || "", sync_groups: ((_c = e.client.scim) == null ? void 0 : _c.sync_groups) === true || true, group_sync_prefix: ((_d = e.client.scim) == null ? void 0 : _d.group_sync_prefix) || "" })), S = H({ authorizationCode: e.client.flows_enabled.includes("authorization_code"), clientCredentials: e.client.flows_enabled.includes("client_credentials"), password: e.client.flows_enabled.includes("password"), refreshToken: e.client.flows_enabled.includes("refresh_token"), deviceCode: e.client.flows_enabled.includes(Xe) });
  const W = ["RS256", "RS384", "RS512", "EdDSA"];
  let M = b(H(e.client.access_token_alg)), R = b(H(e.client.id_token_alg)), d = b(H(e.client.access_token_lifetime.toString())), y = b(H(e.client.auth_code_lifetime.toString())), B = b(H(e.scopesAll.map((Q) => ({ name: Q, selected: e.client.scopes.includes(Q) || false })))), q = b(H(e.scopesAll.map((Q) => ({ name: Q, selected: e.client.default_scopes.includes(Q) || false })))), P = H({ plain: ((_e = e.client.challenges) == null ? void 0 : _e.includes("plain")) || false, s256: ((_f = e.client.challenges) == null ? void 0 : _f.includes("S256")) || false }), h = b(H(e.client.force_mfa));
  me(() => {
    var _a3, _b2, _c2, _d2, _e2, _f2;
    e.client.id && (n(_, e.client.id, true), n(k, e.client.name || "", true), n(L, e.client.enabled, true), n(g, e.client.confidential, true), n(f, e.client.client_uri || "", true), n(j, e.client.backchannel_logout_uri || "", true), n(A, e.client.contacts ? Array.from(e.client.contacts) : [], true), n(w, e.client.allowed_origins ? Array.from(e.client.allowed_origins) : [], true), n(p, Array.from(e.client.redirect_uris), true), n(I, e.client.post_logout_redirect_uris ? Array.from(e.client.post_logout_redirect_uris) : [], true), S.authorizationCode = e.client.flows_enabled.includes("authorization_code"), S.clientCredentials = e.client.flows_enabled.includes("client_credentials"), S.password = e.client.flows_enabled.includes("password"), S.refreshToken = e.client.flows_enabled.includes("refresh_token"), S.deviceCode = e.client.flows_enabled.includes(Xe), n(M, e.client.access_token_alg, true), n(R, e.client.id_token_alg, true), n(d, e.client.access_token_lifetime.toString(), true), n(y, e.client.auth_code_lifetime.toString(), true), n(B, e.scopesAll.map((Q) => ({ name: Q, selected: e.client.scopes.includes(Q) || false })), true), n(q, e.scopesAll.map((Q) => ({ name: Q, selected: e.client.default_scopes.includes(Q) || false })), true), P.plain = ((_a3 = e.client.challenges) == null ? void 0 : _a3.includes("plain")) || false, P.s256 = ((_b2 = e.client.challenges) == null ? void 0 : _b2.includes("S256")) || false, n(x, { base_uri: ((_c2 = e.client.scim) == null ? void 0 : _c2.base_uri) || "", bearer_token: ((_d2 = e.client.scim) == null ? void 0 : _d2.bearer_token) || "", sync_groups: ((_e2 = e.client.scim) == null ? void 0 : _e2.sync_groups) === true || true, group_sync_prefix: ((_f2 = e.client.scim) == null ? void 0 : _f2.group_sync_prefix) || "" }, true), X());
  });
  async function X() {
    let Q = await tt(`/auth/v1/clients/${e.client.id}`);
    if (Q.body) {
      let U = Q.body;
      n(E, !!U.scim), U.scim && n(x, { base_uri: U.scim.base_uri, bearer_token: U.scim.bearer_token, sync_groups: U.scim.sync_groups, group_sync_prefix: U.scim.group_sync_prefix || "" }, true);
    } else console.error(Q.error);
  }
  async function ae(Q, U) {
    n(v, "");
    let F = { id: t(_), name: t(k) || void 0, enabled: t(L), confidential: t(g), allowed_origins: t(w).length > 0 ? t(w) : void 0, redirect_uris: t(p), post_logout_redirect_uris: t(I).length > 0 ? t(I) : void 0, flows_enabled: [], access_token_alg: t(M), id_token_alg: t(R), access_token_lifetime: Number.parseInt(t(d)), auth_code_lifetime: Number.parseInt(t(y)), scopes: t(B).filter((ee) => ee.selected).map((ee) => ee.name), default_scopes: t(q).filter((ee) => ee.selected).map((ee) => ee.name), challenges: void 0, force_mfa: t(h), client_uri: t(f) || void 0, contacts: t(A).length > 0 ? t(A) : void 0, backchannel_logout_uri: t(j) || void 0 };
    S.authorizationCode && F.flows_enabled.push("authorization_code"), S.clientCredentials && F.flows_enabled.push("client_credentials"), S.password && F.flows_enabled.push("password"), S.refreshToken && F.flows_enabled.push("refresh_token"), S.deviceCode && F.flows_enabled.push(Xe), P.plain && (F.challenges = ["plain"]), P.s256 && (F.challenges ? F.challenges.push("S256") : F.challenges = ["S256"]), t(E) && (F.scim = { base_uri: t(x).base_uri, bearer_token: t(x).bearer_token, sync_groups: t(x).sync_groups, group_sync_prefix: t(x).sync_groups && t(x).group_sync_prefix.length ? t(x).group_sync_prefix : void 0 });
    let se = await nt(Q.action, F);
    se.error ? n(v, se.error.message, true) : (n(C, true), e.onSave(), setTimeout(() => {
      n(C, false);
    }, 2e3));
  }
  var le = ma(), he = o(le);
  const ge = ve(() => `/auth/v1/clients/${e.client.id}`);
  st(he, { get action() {
    return t(ge);
  }, onSubmit: ae, children: (Q, U) => {
    var F = ha(), se = fe(F), ee = o(se, true);
    s(se);
    var z = a(se, 2);
    Qe(z, { label: "ID", mono: true, get copyToClip() {
      return e.client.id;
    }, children: (i, K) => {
      G();
      var m = J();
      N(() => T(m, e.client.id)), u(i, m);
    }, $$slots: { default: true } });
    var te = a(z, 2), re = o(te, true);
    s(te);
    var be = a(te, 2);
    ue(be, { autocomplete: "off", get label() {
      return l.clients.name;
    }, get placeholder() {
      return l.clients.name;
    }, width: c, pattern: $t, get value() {
      return t(k);
    }, set value(i) {
      n(k, i, true);
    } });
    var Pe = a(be, 2), Re = o(Pe, true);
    s(Pe);
    var Ee = a(Pe, 2);
    ue(Ee, { typ: "url", autocomplete: "off", label: "URI", placeholder: "URI", width: c, pattern: pe, get value() {
      return t(f);
    }, set value(i) {
      n(f, i, true);
    } });
    var dt = a(Ee, 2);
    Ce(dt, { get label() {
      return l.common.contact;
    }, pattern: Wr, get values() {
      return t(A);
    }, set values(i) {
      n(A, i, true);
    } });
    var ct = a(dt, 2);
    Z(ct, "", {}, { height: ".5rem" });
    var Me = a(ct, 2), ir = o(Me, true);
    s(Me);
    var ut = a(Me, 2);
    oe(ut, { get ariaLabel() {
      return l.common.enabled;
    }, get checked() {
      return t(L);
    }, set checked(i) {
      n(L, i, true);
    }, children: (i, K) => {
      G();
      var m = J();
      N(() => T(m, l.common.enabled)), u(i, m);
    }, $$slots: { default: true } });
    var vt = a(ut, 2);
    oe(vt, { get ariaLabel() {
      return l.clients.confidential;
    }, get checked() {
      return t(g);
    }, set checked(i) {
      n(g, i, true);
    }, children: (i, K) => {
      G();
      var m = J();
      N(() => T(m, l.clients.confidential)), u(i, m);
    }, $$slots: { default: true } });
    var _t = a(vt, 2);
    oe(_t, { get ariaLabel() {
      return l.clients.forceMfa;
    }, get checked() {
      return t(h);
    }, set checked(i) {
      n(h, i, true);
    }, children: (i, K) => {
      G();
      var m = J();
      N(() => T(m, l.clients.forceMfa)), u(i, m);
    }, $$slots: { default: true } });
    var ft = a(_t, 4);
    oe(ft, { ariaLabel: "authorization_code", get checked() {
      return S.authorizationCode;
    }, set checked(i) {
      S.authorizationCode = i;
    }, children: (i, K) => {
      G();
      var m = J("authorization_code");
      u(i, m);
    }, $$slots: { default: true } });
    var gt = a(ft, 2);
    oe(gt, { ariaLabel: "urn:ietf:params:oauth:grant-type:device_code", get checked() {
      return S.deviceCode;
    }, set checked(i) {
      S.deviceCode = i;
    }, children: (i, K) => {
      G();
      var m = J("device_code");
      u(i, m);
    }, $$slots: { default: true } });
    var ht = a(gt, 2);
    oe(ht, { ariaLabel: "client_credentials", get checked() {
      return S.clientCredentials;
    }, set checked(i) {
      S.clientCredentials = i;
    }, children: (i, K) => {
      G();
      var m = J("client_credentials");
      u(i, m);
    }, $$slots: { default: true } });
    var mt = a(ht, 2);
    oe(mt, { ariaLabel: "password", get checked() {
      return S.password;
    }, set checked(i) {
      S.password = i;
    }, children: (i, K) => {
      G();
      var m = J("password");
      u(i, m);
    }, $$slots: { default: true } });
    var bt = a(mt, 2);
    oe(bt, { ariaLabel: "refresh_token", get checked() {
      return S.refreshToken;
    }, set checked(i) {
      S.refreshToken = i;
    }, children: (i, K) => {
      G();
      var m = J("refresh_token");
      u(i, m);
    }, $$slots: { default: true } });
    var kt = a(bt, 2);
    Z(kt, "", {}, { height: ".5rem" });
    var Ue = a(kt, 4), nr = o(Ue, true);
    s(Ue);
    var Be = a(Ue, 2), xt = o(Be), sr = o(xt, true);
    s(xt), s(Be);
    var yt = a(Be, 2);
    oe(yt, { ariaLabel: "PKCE plain", get checked() {
      return P.plain;
    }, set checked(i) {
      P.plain = i;
    }, children: (i, K) => {
      G();
      var m = J("plain");
      u(i, m);
    }, $$slots: { default: true } });
    var wt = a(yt, 2);
    oe(wt, { ariaLabel: "PKCE S256", get checked() {
      return P.s256;
    }, set checked(i) {
      P.s256 = i;
    }, children: (i, K) => {
      G();
      var m = J("S256");
      u(i, m);
    }, $$slots: { default: true } });
    var pt = a(wt, 2);
    {
      var or = (i) => {
        var K = va(), m = o(K, true);
        s(K), N(() => T(m, l.clients.errConfidentialPKCE)), Ge(3, K, () => Je, () => ({ duration: 150 })), u(i, K);
      };
      Y(pt, (i) => {
        !t(g) && !P.plain && !P.s256 && i(or);
      });
    }
    var Ct = a(pt, 2);
    Z(Ct, "", {}, { height: ".5rem" });
    var De = a(Ct, 4), dr = o(De, true);
    s(De);
    var Lt = a(De, 2);
    Ce(Lt, { typ: "url", label: "Allowed Origins", get errMsg() {
      return l.validation.origin;
    }, pattern: Gr, get values() {
      return t(w);
    }, set values(i) {
      n(w, i, true);
    } });
    var Ne = a(Lt, 2), cr = o(Ne);
    ce(cr, () => l.clients.descUri), s(Ne);
    var Pt = a(Ne, 2);
    Ce(Pt, { typ: "url", label: "Redirect URIs", get errMsg() {
      return l.validation.uri;
    }, get required() {
      return S.authorizationCode;
    }, pattern: pe, get values() {
      return t(p);
    }, set values(i) {
      n(p, i, true);
    } });
    var St = a(Pt, 2);
    Ce(St, { typ: "url", label: "Post Logout Redirect URIs", get errMsg() {
      return l.validation.uri;
    }, pattern: pe, get values() {
      return t(I);
    }, set values(i) {
      n(I, i, true);
    } });
    var Tt = a(St, 2);
    Z(Tt, "", {}, { height: ".5rem" });
    var qe = a(Tt, 4), ur = o(qe);
    ce(ur, () => l.clients.scopes.desc), s(qe);
    var It = a(qe, 2);
    Qt(It, { get items() {
      return t(B);
    }, set items(i) {
      n(B, i, true);
    }, children: (i, K) => {
      G();
      var m = J();
      N(() => T(m, l.clients.scopes.allowed)), u(i, m);
    }, $$slots: { default: true } });
    var At = a(It, 2);
    Qt(At, { get items() {
      return t(q);
    }, set items(i) {
      n(q, i, true);
    }, children: (i, K) => {
      G();
      var m = J();
      N(() => T(m, l.clients.scopes.default)), u(i, m);
    }, $$slots: { default: true } });
    var Rt = a(At, 2);
    Z(Rt, "", {}, { height: ".75rem" });
    var Oe = a(Rt, 4), vr = o(Oe, true);
    s(Oe);
    var Et = a(Oe, 2);
    ue(Et, { typ: "number", autocomplete: "off", label: "Token Lifetime", placeholder: "Token Lifetime", width: c, min: "10", max: "86400", errMsg: "10 <= Token Lifetime <= 86400", get value() {
      return t(d);
    }, set value(i) {
      n(d, i, true);
    } });
    var Mt = a(Et, 2);
    Z(Mt, "", {}, { height: ".5rem" });
    var je = a(Mt, 2), _r = o(je, true);
    s(je);
    var Fe = a(je, 2), fr = o(Fe, true);
    s(Fe);
    var Ut = a(Fe, 2);
    Qe(Ut, { label: "Access Token Algorithm", children: (i, K) => {
      Xt(i, { ariaLabel: "Access Token Algorithm", options: W, borderless: true, get value() {
        return t(M);
      }, set value(m) {
        n(M, m, true);
      } });
    }, $$slots: { default: true } });
    var Bt = a(Ut, 2);
    Qe(Bt, { label: "ID Token Algorithm", children: (i, K) => {
      Xt(i, { ariaLabel: "ID Token Algorithm", options: W, borderless: true, get value() {
        return t(R);
      }, set value(m) {
        n(R, m, true);
      } });
    }, $$slots: { default: true } });
    var Dt = a(Bt, 2);
    Z(Dt, "", {}, { height: ".5rem" });
    var ze = a(Dt, 2), gr = o(ze, true);
    s(ze);
    var Nt = a(ze, 2);
    ue(Nt, { typ: "number", autocomplete: "off", label: "Auth Code Lifetime", placeholder: "Auth Code Lifetime", width: c, min: "10", max: "300", errMsg: "10 <= Auth Code Lifetime <= 300", get value() {
      return t(y);
    }, set value(i) {
      n(y, i, true);
    } });
    var Ke = a(Nt, 4), hr = o(Ke);
    ce(hr, () => l.clients.backchannelLogout.replace("{{ OIDC_BCL }}", '<a href="https://openid.net/specs/openid-connect-backchannel-1_0.html" target="_blank">OpenID Connect Back-Channel Logout</a>')), s(Ke);
    var qt = a(Ke, 2);
    const mr = ve(() => e.client.id === "rauthy");
    ue(qt, { typ: "url", autocomplete: "off", label: "Backchannel Logout URI", placeholder: "Backchannel Logout URI", width: c, pattern: pe, get disabled() {
      return t(mr);
    }, get value() {
      return t(j);
    }, set value(i) {
      n(j, i, true);
    } });
    var Te = a(qt, 4);
    Z(Te, "", {}, { "margin-bottom": ".5rem" });
    var br = o(Te);
    ce(br, () => l.clients.scim.desc.replace("{{ SCIM_LINK }}", '<a href="https://www.rfc-editor.org/rfc/rfc7644" target="_blank">SCIM v2</a>')), s(Te);
    var Ot = a(Te, 2);
    const kr = ve(() => e.client.id === "rauthy");
    oe(Ot, { get ariaLabel() {
      return l.clients.scim.enable;
    }, get disabled() {
      return t(kr);
    }, get checked() {
      return t(E);
    }, set checked(i) {
      n(E, i, true);
    }, children: (i, K) => {
      G();
      var m = J();
      N(() => T(m, l.clients.scim.enable)), u(i, m);
    }, $$slots: { default: true } });
    var jt = a(Ot, 2);
    {
      var xr = (i) => {
        var K = fa(), m = o(K);
        Z(m, "", {}, { "margin-bottom": ".5rem" });
        var Lr = o(m, true);
        s(m);
        var He = a(m, 2), Ve = o(He), Pr = o(Ve);
        ce(Pr, () => l.clients.scim.reqLi1), s(Ve);
        var We = a(Ve, 2), Sr = o(We);
        ce(Sr, () => l.clients.scim.reqLi2), s(We);
        var Kt = a(We, 2), Tr = o(Kt);
        ce(Tr, () => l.clients.scim.reqLi3), s(Kt), s(He);
        var Ae = a(He, 2);
        Z(Ae, "", {}, { "margin-top": "1rem" });
        var Ir = o(Ae);
        ce(Ir, () => l.clients.scim.baseUri), s(Ae);
        var Ht = a(Ae, 2);
        ue(Ht, { typ: "url", autocomplete: "off", label: "SCIM Base URI", placeholder: "SCIM Base URI", width: c, pattern: pe, get required() {
          return t(E);
        }, get value() {
          return t(x).base_uri;
        }, set value(_e2) {
          t(x).base_uri = _e2;
        } });
        var Vt = a(Ht, 2);
        lr(Vt, { autocomplete: "off", label: "Bearer Token", placeholder: "Bearer Token", width: c, pattern: pe, get required() {
          return t(E);
        }, get value() {
          return t(x).bearer_token;
        }, set value(_e2) {
          t(x).bearer_token = _e2;
        } });
        var Wt = a(Vt, 2);
        Z(Wt, "", {}, { height: ".5rem" });
        var Gt = a(Wt, 2);
        const Ar = ve(() => e.client.id === "rauthy");
        oe(Gt, { get ariaLabel() {
          return l.clients.scim.groupSync;
        }, get disabled() {
          return t(Ar);
        }, get checked() {
          return t(x).sync_groups;
        }, set checked(_e2) {
          t(x).sync_groups = _e2;
        }, children: (_e2, Se) => {
          G();
          var we = J();
          N(() => T(we, l.clients.scim.groupSync)), u(_e2, we);
        }, $$slots: { default: true } });
        var Rr = a(Gt, 2);
        {
          var Er = (_e2) => {
            var Se = _a(), we = o(Se), Mr = o(we);
            ce(Mr, () => l.clients.scim.groupSyncPrefixDesc), s(we);
            var Ur = a(we, 2);
            ue(Ur, { autocomplete: "off", get label() {
              return l.clients.scim.groupSyncPrefix;
            }, get placeholder() {
              return l.clients.scim.groupSyncPrefix;
            }, width: c, pattern: Jr, get value() {
              return t(x).group_sync_prefix;
            }, set value(Br) {
              t(x).group_sync_prefix = Br;
            } }), s(Se), Ge(3, Se, () => Je, () => ({ duration: 150 })), u(_e2, Se);
          };
          Y(Rr, (_e2) => {
            t(x).sync_groups && _e2(Er);
          });
        }
        s(K), N(() => T(Lr, l.clients.scim.reqDesc)), Ge(3, K, () => Je, () => ({ duration: 150 })), u(i, K);
      };
      Y(jt, (i) => {
        t(E) && i(xr);
      });
    }
    var Ie = a(jt, 2);
    Z(Ie, "", {}, { "margin-top": "1rem" });
    var Ft = o(Ie);
    de(Ft, { type: "submit", children: (i, K) => {
      G();
      var m = J();
      N(() => T(m, r.common.save)), u(i, m);
    }, $$slots: { default: true } });
    var yr = a(Ft, 2);
    {
      var wr = (i) => {
        ar(i, {});
      };
      Y(yr, (i) => {
        t(C) && i(wr);
      });
    }
    s(Ie);
    var zt = a(Ie, 2);
    {
      var pr = (i) => {
        var K = ga(), m = o(K, true);
        s(K), N(() => T(m, t(v))), u(i, K);
      };
      Y(zt, (i) => {
        t(v) && i(pr);
      });
    }
    var Cr = a(zt, 2);
    Z(Cr, "", {}, { height: "1rem" }), N(() => {
      T(ee, l.common.information), T(re, l.clients.descName), T(Re, l.clients.descClientUri), T(ir, l.clients.config), T(nr, l.clients.descPKCE), T(sr, l.clients.descPKCEEnforce), T(dr, l.clients.descOrigin), T(vr, l.clients.tokenLifetime.p1), T(_r, l.clients.tokenLifetime.p2), T(fr, l.clients.tokenLifetime.p3), T(gr, l.clients.descAuthCode);
    }), u(Q, F);
  }, $$slots: { default: true } }), s(le), u(D, le), ne();
}
var ka = O("<!> <!>", 1), xa = O('<div class="container svelte-19xcrje"><div class="err"> </div> <!></div>');
function ya(D, e) {
  ie(e, true);
  let r = ke(), l = b(""), c = b(void 0);
  me(() => {
    n(l, ""), n(c, ""), e.client.confidential ? v() : n(l, r.clients.confidentialNoSecret, true);
  });
  async function v() {
    let w = await it(`/auth/v1/clients/${e.client.id}/secret`);
    _(w);
  }
  async function C() {
    let w = await nt(`/auth/v1/clients/${e.client.id}/secret`);
    _(w);
  }
  function _(w) {
    var _a2;
    w.body ? w.body.secret && n(c, w.body.secret, true) : n(l, ((_a2 = w.error) == null ? void 0 : _a2.message) || "Error", true);
  }
  var k = xa(), L = o(k), g = o(L, true);
  s(L);
  var f = a(L, 2);
  {
    var A = (w) => {
      var p = ka(), I = fe(p);
      lr(I, { autocomplete: "off", label: "Client Secret", placeholder: "Client Secret", disabled: true, showCopy: true, get value() {
        return t(c);
      }, set value(E) {
        n(c, E, true);
      } });
      var j = a(I, 2);
      de(j, { onclick: C, children: (E, x) => {
        G();
        var S = J();
        N(() => T(S, r.clients.generateSecret)), u(E, S);
      }, $$slots: { default: true } }), u(w, p);
    };
    Y(f, (w) => {
      t(c) && w(A);
    });
  }
  s(k), N(() => T(g, t(l))), u(D, k), ne();
}
var wa = O('<div class="err"> </div>'), pa = O("<p> </p> <!> <!>", 1);
function Ca(D, e) {
  ie(e, true);
  let r = Le(), l = ke(), c = b("");
  async function v() {
    n(c, "");
    let A = await rt(`/auth/v1/clients/${e.client.id}`);
    A.error ? n(c, A.error.message, true) : e.onSave();
  }
  var C = pa(), _ = fe(C), k = o(_, true);
  s(_);
  var L = a(_, 2);
  de(L, { level: -1, onclick: v, children: (A, w) => {
    G();
    var p = J();
    N(() => T(p, r.common.delete)), u(A, p);
  }, $$slots: { default: true } });
  var g = a(L, 2);
  {
    var f = (A) => {
      var w = wa(), p = o(w, true);
      s(w), N(() => T(p, t(c))), u(A, w);
    };
    Y(g, (A) => {
      t(c) && A(f);
    });
  }
  N(() => T(k, l.clients.delete1)), u(D, C), ne();
}
var La = O('<div><div class="flex"><input type="range" class="svelte-12j8dit"> <div class="value font-mono svelte-12j8dit"> </div></div> <div class="label svelte-12j8dit"><label class="font-label noselect svelte-12j8dit"> </label></div></div>');
function Ye(D, e) {
  ie(e, true);
  let r = $(e, "value", 15), l = $(e, "label", 3, ""), c = $(e, "disabled", 3, false), v = $(e, "step", 3, 1), C = $(e, "widthRange", 3, "15rem");
  const _ = ye();
  let k = ve(() => {
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
  var L = La(), g = o(L), f = o(g);
  lt(f), V(f, "id", _);
  let A;
  var w = a(f, 2), p = o(w, true);
  s(w), s(g);
  var I = a(g, 2), j = o(I);
  V(j, "for", _);
  var E = o(j, true);
  s(j), s(I), s(L), N(() => {
    V(f, "name", e.name), V(f, "title", l()), V(f, "aria-label", l()), f.disabled = c(), V(f, "aria-disabled", c()), V(f, "min", e.min), V(f, "max", e.max), V(f, "step", v()), A = Z(f, "", A, { width: C(), "--bg-slider": t(k) }), T(p, r()), T(E, l());
  }), ot(f, r), u(D, L), ne();
}
var Pa = O('<div class="outer svelte-1evkwuy"><div class="container svelte-1evkwuy"><div><label class="font-label svelte-1evkwuy"> </label> <div class="values svelte-1evkwuy"><!> <!> <!></div></div> <div class="color svelte-1evkwuy"></div></div></div>');
function xe(D, e) {
  ie(e, true);
  let r = $(e, "h", 15), l = $(e, "s", 15), c = $(e, "l", 15);
  const v = ye(), C = "15rem";
  let _ = ve(() => `hsl(${r()} ${l()} ${c()})`);
  var k = Pa(), L = o(k);
  let g;
  var f = o(L), A = o(f);
  V(A, "for", v);
  var w = o(A, true);
  s(A);
  var p = a(A, 2);
  V(p, "id", v);
  var I = o(p);
  Ye(I, { label: "Hue", min: 0, max: 359, widthRange: C, bgMode: "hue", get hue() {
    return r();
  }, get sat() {
    return l();
  }, get lum() {
    return c();
  }, get value() {
    return r();
  }, set value(W) {
    r(W);
  } });
  var j = a(I, 2);
  Ye(j, { label: "Sat", min: 0, max: 100, widthRange: C, bgMode: "sat", get hue() {
    return r();
  }, get sat() {
    return l();
  }, get lum() {
    return c();
  }, get value() {
    return l();
  }, set value(W) {
    l(W);
  } });
  var E = a(j, 2);
  Ye(E, { label: "Lum", min: 0, max: 100, widthRange: C, bgMode: "lum", get hue() {
    return r();
  }, get sat() {
    return l();
  }, get lum() {
    return c();
  }, get value() {
    return c();
  }, set value(W) {
    c(W);
  } }), s(p), s(f);
  var x = a(f, 2);
  let S;
  s(L), s(k), N(() => {
    g = Z(L, "", g, { "border-color": t(_) }), T(w, e.label), S = Z(x, "", S, { background: t(_) });
  }), u(D, k), ne();
}
var Sa = (D, e) => {
  var _a2;
  return (_a2 = t(e)) == null ? void 0 : _a2.click();
}, Ta = O('<div class="container svelte-2nx18n"><!> <div role="none" class="mask svelte-2nx18n"><div class="colorWrapper svelte-2nx18n"><div class="color svelte-2nx18n"></div></div> <input type="color" class="svelte-2nx18n"></div></div>');
function Ze(D, e) {
  ie(e, true);
  let r = $(e, "value", 15), l = ke(), c = b(void 0);
  var v = Ta();
  let C;
  var _ = o(v);
  ue(_, { get label() {
    return e.label;
  }, get placeholder() {
    return e.label;
  }, get errMsg() {
    return l.validation.css;
  }, width: "17.5rem", required: true, pattern: er, get value() {
    return r();
  }, set value(g) {
    r(g);
  } });
  var k = a(_, 2);
  k.__click = [Sa, c];
  var L = a(o(k), 2);
  lt(L), tr(L, (g) => n(c, g), () => t(c)), s(k), s(v), N(() => C = Z(v, "", C, { "--color": r() })), ot(L, r), u(D, v), ne();
}
at(["click"]);
var Ia = O('<div><p> </p> <div class="hsl svelte-10udpph"><div><!> <!> <!> <!></div> <div><!> <!> <!></div></div> <div><p><!></p> <!> <!> <!></div></div>');
function Yt(D, e) {
  ie(e, true);
  let r = $(e, "values", 15), l = ke();
  var c = Ia(), v = o(c), C = o(v, true);
  s(v);
  var _ = a(v, 2), k = o(_), L = o(k);
  xe(L, { label: "--text", get h() {
    return r().text[0];
  }, set h(d) {
    r(r().text[0] = d, true);
  }, get s() {
    return r().text[1];
  }, set s(d) {
    r(r().text[1] = d, true);
  }, get l() {
    return r().text[2];
  }, set l(d) {
    r(r().text[2] = d, true);
  } });
  var g = a(L, 2);
  xe(g, { label: "--text-high", get h() {
    return r().text_high[0];
  }, set h(d) {
    r(r().text_high[0] = d, true);
  }, get s() {
    return r().text_high[1];
  }, set s(d) {
    r(r().text_high[1] = d, true);
  }, get l() {
    return r().text_high[2];
  }, set l(d) {
    r(r().text_high[2] = d, true);
  } });
  var f = a(g, 2);
  xe(f, { label: "--bg", get h() {
    return r().bg[0];
  }, set h(d) {
    r(r().bg[0] = d, true);
  }, get s() {
    return r().bg[1];
  }, set s(d) {
    r(r().bg[1] = d, true);
  }, get l() {
    return r().bg[2];
  }, set l(d) {
    r(r().bg[2] = d, true);
  } });
  var A = a(f, 2);
  xe(A, { label: "--bg-high", get h() {
    return r().bg_high[0];
  }, set h(d) {
    r(r().bg_high[0] = d, true);
  }, get s() {
    return r().bg_high[1];
  }, set s(d) {
    r(r().bg_high[1] = d, true);
  }, get l() {
    return r().bg_high[2];
  }, set l(d) {
    r(r().bg_high[2] = d, true);
  } }), s(k);
  var w = a(k, 2), p = o(w);
  xe(p, { label: "--action", get h() {
    return r().action[0];
  }, set h(d) {
    r(r().action[0] = d, true);
  }, get s() {
    return r().action[1];
  }, set s(d) {
    r(r().action[1] = d, true);
  }, get l() {
    return r().action[2];
  }, set l(d) {
    r(r().action[2] = d, true);
  } });
  var I = a(p, 2);
  xe(I, { label: "--accent", get h() {
    return r().accent[0];
  }, set h(d) {
    r(r().accent[0] = d, true);
  }, get s() {
    return r().accent[1];
  }, set s(d) {
    r(r().accent[1] = d, true);
  }, get l() {
    return r().accent[2];
  }, set l(d) {
    r(r().accent[2] = d, true);
  } });
  var j = a(I, 2);
  xe(j, { label: "--error", get h() {
    return r().error[0];
  }, set h(d) {
    r(r().error[0] = d, true);
  }, get s() {
    return r().error[1];
  }, set s(d) {
    r(r().error[1] = d, true);
  }, get l() {
    return r().error[2];
  }, set l(d) {
    r(r().error[2] = d, true);
  } }), s(w), s(_);
  var E = a(_, 2), x = o(E), S = o(x);
  ce(S, () => l.clients.branding.descFullCss), s(x);
  var W = a(x, 2);
  Ze(W, { label: "--btn-text", get value() {
    return r().btn_text;
  }, set value(d) {
    r(r().btn_text = d, true);
  } });
  var M = a(W, 2);
  Ze(M, { label: "--theme-sun", get value() {
    return r().theme_sun;
  }, set value(d) {
    r(r().theme_sun = d, true);
  } });
  var R = a(M, 2);
  Ze(R, { label: "--theme-moon", get value() {
    return r().theme_moon;
  }, set value(d) {
    r(r().theme_moon = d, true);
  } }), s(E), s(c), N(() => T(C, l.clients.branding.descHsl)), u(D, c), ne();
}
var Aa = O('<img alt="Client Logo" width="100%" height="100%">'), Ra = (D) => D.preventDefault(), Ea = O('<div aria-label="Preview: All components inside are only for theme and colors preview and have no effect or interaction"><div class="inner svelte-fkmn75"><div class="container svelte-fkmn75"><div class="logo svelte-fkmn75"><!></div> <h3 class="svelte-fkmn75">Header</h3> <p>Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam</p> <p><span class="svelte-fkmn75">--accent-color</span></p> <!> <div class="btn svelte-fkmn75"><div class="svelte-fkmn75"><!> <br> <!> <br> <!> <br></div> <div class="svelte-fkmn75"><!> <br> <!> <br> <!></div></div> <a>Link</a> <br> <!></div></div></div>');
function Zt(D, e) {
  ie(e, true);
  let r = b(false), l = b(false);
  me(() => {
    e.logoUrl && n(l, false);
  }), me(() => {
    t(r) && setTimeout(() => {
      n(r, false);
    }, 2e3);
  });
  var c = Ea();
  let v;
  var C = o(c), _ = o(C), k = o(_), L = o(k);
  {
    var g = (y) => {
      Zr(y, { width: "100%", get show() {
        return e.typ;
      } });
    }, f = (y) => {
      var B = Aa();
      N(() => V(B, "src", e.logoUrl)), $e("error", B, () => n(l, true)), qr(B), u(y, B);
    };
    Y(L, (y) => {
      t(l) ? y(g) : y(f, false);
    });
  }
  s(k);
  var A = a(k, 8);
  ue(A, { label: "Preview", placeholder: "Preview", width: "12.5rem" });
  var w = a(A, 2), p = o(w), I = o(p);
  de(I, { level: 1, get isLoading() {
    return t(r);
  }, onclick: () => n(r, true), children: (y, B) => {
    G();
    var q = J("Button");
    u(y, q);
  }, $$slots: { default: true } });
  var j = a(I, 4);
  de(j, { level: 2, get isLoading() {
    return t(r);
  }, onclick: () => n(r, true), children: (y, B) => {
    G();
    var q = J("Button");
    u(y, q);
  }, $$slots: { default: true } });
  var E = a(j, 4);
  de(E, { level: 3, get isLoading() {
    return t(r);
  }, onclick: () => n(r, true), children: (y, B) => {
    G();
    var q = J("Button");
    u(y, q);
  }, $$slots: { default: true } }), G(2), s(p);
  var x = a(p, 2), S = o(x);
  de(S, { level: -1, get isLoading() {
    return t(r);
  }, onclick: () => n(r, true), children: (y, B) => {
    G();
    var q = J("Button");
    u(y, q);
  }, $$slots: { default: true } });
  var W = a(S, 4);
  de(W, { level: -2, get isLoading() {
    return t(r);
  }, onclick: () => n(r, true), children: (y, B) => {
    G();
    var q = J("Button");
    u(y, q);
  }, $$slots: { default: true } });
  var M = a(W, 4);
  de(M, { level: -3, get isLoading() {
    return t(r);
  }, onclick: () => n(r, true), children: (y, B) => {
    G();
    var q = J("Button");
    u(y, q);
  }, $$slots: { default: true } }), s(x), s(w);
  var R = a(w, 2);
  V(R, "href", window.location.href), R.__click = [Ra];
  var d = a(R, 4);
  Yr(d, {}), s(_), s(C), s(c), N((y, B, q, P, h, X, ae) => v = Z(c, "", v, { "--border-radius": e.borderRadius, "--text": y, "--text-high": B, "--bg": q, "--bg-high": P, "--action": h, "--accent": X, "--error": ae, "--btn-text": e.theme.btn_text, "--theme-sun": e.theme.theme_sun, "--theme-moon": e.theme.theme_moon }), [() => e.theme.text.join(" "), () => e.theme.text_high.join(" "), () => e.theme.bg.join(" "), () => e.theme.bg_high.join(" "), () => e.theme.action.join(" "), () => e.theme.accent.join(" "), () => e.theme.error.join(" ")]), u(D, c), ne();
}
at(["click"]);
var Ma = O('<div><h2>Preview</h2> <div class="tabs svelte-1klemk8"><!></div> <div class="preview svelte-1klemk8"><!></div></div>');
function Ua(D, e) {
  ie(e, true);
  let r = ["Light Theme", "Dark Theme"], l = b(H(r[0]));
  var c = Ma(), v = a(o(c), 2), C = o(v);
  rr(C, { tabs: r, center: true, get selected() {
    return t(l);
  }, set selected(f) {
    n(l, f, true);
  } }), s(v);
  var _ = a(v, 2), k = o(_);
  {
    var L = (f) => {
      Zt(f, { get logoUrl() {
        return e.logoUrl;
      }, get borderRadius() {
        return e.theme.border_radius;
      }, get theme() {
        return e.theme.light;
      }, typ: "light" });
    }, g = (f) => {
      Zt(f, { get logoUrl() {
        return e.logoUrl;
      }, get borderRadius() {
        return e.theme.border_radius;
      }, get theme() {
        return e.theme.dark;
      }, typ: "dark" });
    };
    Y(k, (f) => {
      t(l) === r[0] ? f(L) : f(g, false);
    });
  }
  s(_), s(c), u(D, c), ne();
}
var Ba = O('<div class="err"> </div>'), Da = O('<div class="values svelte-1pkqupw"><p><!></p> <!> <h1>Light Theme</h1> <!> <br> <h1>Dark Theme</h1> <!></div> <div class="preview"><!> <hr> <p>Logo Upload</p> <!> <div class="buttons svelte-1pkqupw"><!> <!> <!></div> <!></div>', 1), Na = O('<div class="container svelte-1pkqupw"><!></div>');
function qa(D, e) {
  ie(e, true);
  const r = "13rem";
  let l = Le(), c = ke(), v = b(false), C = b(""), _ = b(void 0), k = b(H(ye())), L = ve(() => `/auth/v1/clients/${e.client.id}/logo?${t(k)}`), g = ve(() => `/auth/v1/theme/${e.client.id}`);
  me(() => {
    f();
  });
  async function f() {
    var _a2;
    let I = await it(t(g));
    I.body ? n(_, I.body, true) : n(C, ((_a2 = I.error) == null ? void 0 : _a2.message) || "Error", true);
  }
  async function A(I, j) {
    if (!t(_)) {
      console.error("theme is undefined");
      return;
    }
    let E = t(_);
    E.client_id = e.client.id;
    let x = await nt(t(g), E);
    x.error ? n(C, x.error.message, true) : (n(v, true), setTimeout(() => {
      n(v, false);
    }, 2e3));
  }
  async function w() {
    let I = await rt(t(g));
    I.error ? n(C, I.error.message, true) : (await rt(`/auth/v1/clients/${e.client.id}/logo`), await f(), n(k, ye(), true), n(v, true), setTimeout(() => {
      n(v, false);
    }, 2e3));
  }
  async function p() {
    n(k, ye(), true);
  }
  st(D, { get action() {
    return t(g);
  }, onSubmit: A, children: (I, j) => {
    var E = Na(), x = o(E);
    {
      var S = (W) => {
        var M = Da(), R = fe(M), d = o(R), y = o(d);
        ce(y, () => c.clients.branding.descVariables), s(d);
        var B = a(d, 2);
        ue(B, { label: "--border-radius", placeholder: "--border-radius", get errMsg() {
          return c.validation.css;
        }, width: r, required: true, pattern: er, get value() {
          return t(_).border_radius;
        }, set value(z) {
          t(_).border_radius = z;
        } });
        var q = a(B, 4);
        Yt(q, { get values() {
          return t(_).light;
        }, set values(z) {
          t(_).light = z;
        } });
        var P = a(q, 6);
        Yt(P, { get values() {
          return t(_).dark;
        }, set values(z) {
          t(_).dark = z;
        } }), s(R);
        var h = a(R, 2), X = o(h);
        Xr(X, () => t(k), (z) => {
          Ua(z, { get logoUrl() {
            return t(L);
          }, get theme() {
            return t(_);
          } });
        });
        var ae = a(X, 6);
        const le = ve(() => `/auth/v1/clients/${e.client.id}/logo`);
        $r(ae, { method: "PUT", get url() {
          return t(le);
        }, fileName: "logo", onSuccess: p });
        var he = a(ae, 2), ge = o(he);
        de(ge, { type: "submit", children: (z, te) => {
          G();
          var re = J();
          N(() => T(re, l.common.save)), u(z, re);
        }, $$slots: { default: true } });
        var Q = a(ge, 2);
        de(Q, { level: -2, onclick: w, children: (z, te) => {
          G();
          var re = J();
          N(() => T(re, c.common.reset)), u(z, re);
        }, $$slots: { default: true } });
        var U = a(Q, 2);
        {
          var F = (z) => {
            ar(z, {});
          };
          Y(U, (z) => {
            t(v) && z(F);
          });
        }
        s(he);
        var se = a(he, 2);
        {
          var ee = (z) => {
            var te = Ba(), re = o(te, true);
            s(te), N(() => T(re, t(C))), u(z, te);
          };
          Y(se, (z) => {
            t(C) && z(ee);
          });
        }
        s(h), u(W, M);
      };
      Y(x, (W) => {
        t(_) && W(S);
      });
    }
    s(E), u(I, E);
  }, $$slots: { default: true } }), ne();
}
var Oa = O('<div class="flex"><!></div> <div class="details"><!></div>', 1);
function ja(D, e) {
  ie(e, true);
  let r = Le(), l = ke();
  const c = [l.nav.config, "Secret", "Branding", r.common.delete];
  let v = b(H(c[0])), C = b(void 0);
  me(() => {
    e.client.id && requestAnimationFrame(() => {
      var _a2;
      (_a2 = t(C)) == null ? void 0 : _a2();
    });
  });
  var _ = Oa(), k = fe(_), L = o(k);
  rr(L, { tabs: c, get selected() {
    return t(v);
  }, set selected(p) {
    n(v, p, true);
  }, get focusFirst() {
    return t(C);
  }, set focusFirst(p) {
    n(C, p, true);
  } }), s(k);
  var g = a(k, 2), f = o(g);
  {
    var A = (p) => {
      ba(p, { get client() {
        return e.client;
      }, get clients() {
        return e.clients;
      }, get scopesAll() {
        return e.scopesAll;
      }, get onSave() {
        return e.onSave;
      } });
    }, w = (p, I) => {
      {
        var j = (x) => {
          ya(x, { get client() {
            return e.client;
          } });
        }, E = (x, S) => {
          {
            var W = (R) => {
              qa(R, { get client() {
                return e.client;
              } });
            }, M = (R, d) => {
              {
                var y = (B) => {
                  Ca(B, { get client() {
                    return e.client;
                  }, get onSave() {
                    return e.onSave;
                  } });
                };
                Y(R, (B) => {
                  t(v) === r.common.delete && B(y);
                }, d);
              }
            };
            Y(x, (R) => {
              t(v) === "Branding" ? R(W) : R(M, false);
            }, S);
          }
        };
        Y(p, (x) => {
          t(v) === "Secret" ? x(j) : x(E, false);
        }, I);
      }
    };
    Y(f, (p) => {
      t(v) === l.nav.config ? p(A) : p(w, false);
    });
  }
  s(g), u(D, _), ne();
}
var Fa = O('<div class="tile svelte-140q9n9"> <div class="muted svelte-140q9n9"> </div></div>'), za = O("<div></div> <!>", 1), Ka = O("<!> <!>", 1), Ha = O('<div class="err"> </div>'), Va = O('<!> <div id="groups"><!></div>', 1), Wa = O("<!> <!>", 1);
function Ga(D, e) {
  ie(e, true);
  let r = b(void 0), l = b(""), c = b(H([])), v = b(H([])), C = b(void 0), _ = Hr("cid"), k = b(H([]));
  const L = ["ID"];
  let g = b(H(L[0])), f = b("");
  const A = ["ID"];
  Nr(() => {
    w(), p();
  }), me(() => {
    n(C, t(c).find((M) => M.id === _.get()), true);
  }), me(() => {
    let M = t(f).toLowerCase();
    M ? t(g) === L[0] && n(v, t(c).filter((R) => R.id.toLowerCase().includes(M)), true) : n(v, t(c), true);
  });
  async function w() {
    var _a2;
    let M = await tt("/auth/v1/clients");
    M.body ? n(c, M.body, true) : n(l, ((_a2 = M.error) == null ? void 0 : _a2.message) || "Error", true);
  }
  async function p() {
    var _a2;
    let M = await tt("/auth/v1/scopes");
    M.body ? n(k, M.body.map((R) => R.name), true) : n(l, ((_a2 = M.error) == null ? void 0 : _a2.message) || "Error", true);
  }
  function I(M, R) {
    let d = R === "up";
    M === A[0] && t(c).sort((y, B) => d ? y.id.localeCompare(B.id) : B.id.localeCompare(y.id));
  }
  function j() {
    w();
  }
  async function E(M) {
    var _a2;
    (_a2 = t(r)) == null ? void 0 : _a2(), await w(), _.set(M);
  }
  var x = Wa(), S = fe(x);
  zr(S, { paddingTop: "2.1rem", buttonTilesAriaControls: "groups", width: "min(20rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (R) => {
    var d = za(), y = fe(d);
    Z(y, "", {}, { height: ".5rem" });
    var B = a(y, 2);
    et(B, 17, () => t(v), (q) => q.id, (q, P, h, X) => {
      const ae = ve(() => _.get() === t(P).id);
      Fr(q, { onclick: () => _.set(t(P).id), get selected() {
        return t(ae);
      }, children: (le, he) => {
        var ge = Fa(), Q = o(ge), U = a(Q), F = o(U, true);
        s(U), s(ge), N(() => {
          T(Q, `${t(P).id ?? ""} `), T(F, t(P).name);
        }), u(le, ge);
      } });
    }), u(R, d);
  }, children: (R, d) => {
    var y = Ka(), B = fe(y);
    const q = ve(() => t(c).length === 0 ? 1 : 2);
    Or(B, { get level() {
      return t(q);
    }, alignRight: true, get closeModal() {
      return t(r);
    }, set closeModal(h) {
      n(r, h, true);
    }, children: (h, X) => {
      ua(h, { onSave: E, get clients() {
        return t(c);
      } });
    }, $$slots: { default: true } });
    var P = a(B, 2);
    Kr(P, { searchOptions: L, orderOptions: A, onChangeOrder: I, searchWidth: "min(19.5rem, 100dvw - .5rem)", get searchOption() {
      return t(g);
    }, set searchOption(h) {
      n(g, h, true);
    }, get value() {
      return t(f);
    }, set value(h) {
      n(f, h, true);
    } }), u(R, y);
  }, $$slots: { buttonTiles: true, default: true } });
  var W = a(S, 2);
  jr(W, { children: (M, R) => {
    var d = Va(), y = fe(d);
    {
      var B = (X) => {
        var ae = Ha(), le = o(ae, true);
        s(ae), N(() => T(le, t(l))), u(X, ae);
      };
      Y(y, (X) => {
        t(l) && X(B);
      });
    }
    var q = a(y, 2), P = o(q);
    {
      var h = (X) => {
        ja(X, { get client() {
          return t(C);
        }, get clients() {
          return t(c);
        }, get scopesAll() {
          return t(k);
        }, onSave: j });
      };
      Y(P, (X) => {
        t(C) && X(h);
      });
    }
    s(q), u(M, d);
  } }), u(D, x), ne();
}
function Ml(D) {
  Ga(D, {});
}
export {
  Ml as component
};
