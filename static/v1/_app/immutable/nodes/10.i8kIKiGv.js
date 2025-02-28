import { t as j, a as u, d as Pe, e as J } from "../chunks/BH6NCLk-.js";
import { p as re, c, s as a, k as L, t as D, l as s, j as t, a as ae, r as d, a9 as _e, f as te, aa as he, a7 as H, a5 as Yt } from "../chunks/CvlvO1XB.js";
import { d as ze, e as Ct, s as I } from "../chunks/CTI4QPiR.js";
import { i as ee } from "../chunks/BUO_AUgz.js";
import { e as Fe, i as Pt } from "../chunks/BpWRzPRQ.js";
import { a as K, B as se, t as Zt, s as $t } from "../chunks/DMkkW5Nn.js";
import { p as i } from "../chunks/Wh68IIk2.js";
import { B as er } from "../chunks/dWvWuq1E.js";
import { C as tr } from "../chunks/BnPoFdx3.js";
import { N as rr } from "../chunks/D-L0o8jR.js";
import { N as ar } from "../chunks/DpcgKHU5.js";
import { O as lr } from "../chunks/CT2gdhvj.js";
import { b as He, d as Ke, c as Rt, f as Lt } from "../chunks/CBGoQiUs.js";
import { u as ir } from "../chunks/DxHSykar.js";
import { h as we } from "../chunks/i8Xqpu09.js";
import { I as fe } from "../chunks/BtKnbFDH.js";
import { n as nr, g as pt, a as Be, o as sr, p as or, q as Et } from "../chunks/BRCxk8by.js";
import { F as Ve } from "../chunks/BS0DIDHc.js";
import { u as xe } from "../chunks/BQ1-pLIs.js";
import { u as me } from "../chunks/D8mHI_K9.js";
import { I as ve } from "../chunks/DTR8xafZ.js";
import { r as We, s as F } from "../chunks/BMbqVy6X.js";
import { b as Ge } from "../chunks/BhIBACXG.js";
import { b as Mt } from "../chunks/zosqiMUL.js";
import { p as Z } from "../chunks/C6SR4G2t.js";
import { g as ke } from "../chunks/BaE7H8ny.js";
import { I as dr } from "../chunks/Vi3uK7uO.js";
import { T as Ut } from "../chunks/_OE2Cq0B.js";
import { I as Nt } from "../chunks/Nks81rMs.js";
import { L as Oe } from "../chunks/CE2_6siz.js";
import { S as St } from "../chunks/Bbu1Qkv4.js";
import { O as At } from "../chunks/BHnvQvwL.js";
import { I as cr } from "../chunks/D6E4lNPv.js";
import { k as vr } from "../chunks/CaD2yKt4.js";
import { T as ur } from "../chunks/CVHA2fPQ.js";
import { I as fr } from "../chunks/Cog9no35.js";
function _r(U, e, r, l) {
  var _a;
  e(), (_a = r.onInput) == null ? void 0 : _a.call(r, t(l));
}
function hr(U, e, r) {
  var _a;
  switch (U.code) {
    case "Enter":
      e(), (_a = r.onEnter) == null ? void 0 : _a.call(r);
      break;
  }
}
function mr(U, e) {
  var _a, _b;
  (_a = e()) == null ? void 0 : _a.focus(), (_b = e()) == null ? void 0 : _b.scrollIntoView({ behavior: "smooth", block: "center" });
}
var gr = j('<li class="value svelte-r5y8pk"><div class="label"> </div> <div class="btnClose svelte-r5y8pk"><!></div></li>'), br = j("<option></option>"), kr = j('<datalist class="absolute svelte-r5y8pk"></datalist>'), xr = j('<span class="err"> </span>'), wr = j('<div role="none" class="container svelte-r5y8pk"><ul class="svelte-r5y8pk"><!> <li class="svelte-r5y8pk"><input autocomplete="off" class="svelte-r5y8pk"> <!></li></ul> <label class="font-label noselect svelte-r5y8pk"> <!></label></div>');
function be(U, e) {
  re(e, true);
  let r = Z(e, "ref", 15), l = Z(e, "typ", 3, "text"), v = Z(e, "id", 19, ke), f = Z(e, "values", 31, () => i([])), m = Z(e, "label", 3, ""), h = Z(e, "placeholder", 3, ""), b = Z(e, "disabled", 3, false), g = Z(e, "required", 3, false), _ = Z(e, "isError", 15, false), k = Z(e, "width", 3, "inherit"), R = Z(e, "maxHeightList", 3, "inherit"), x = xe();
  const w = ke();
  let y = L(""), S = _e(() => e.datalist && e.datalist.length > 0 ? w : void 0);
  function O(B) {
    f(f().filter((X) => X !== B));
  }
  function N(B) {
    var _a;
    C(), (_a = e.onBlur) == null ? void 0 : _a.call(e), E();
  }
  function z(B) {
    B.preventDefault(), _(true);
  }
  function E() {
    t(y) && C() && (f().push(t(y)), s(y, ""));
  }
  function C() {
    var _a;
    let B = (_a = r()) == null ? void 0 : _a.validity;
    return B ? (_(!B.valid), B.valid) : (_(false), true);
  }
  var P = wr();
  P.__click = [mr, r];
  var o = c(P), p = c(o);
  Fe(p, 17, f, Pt, (B, X, ue, oe) => {
    var de = gr(), ie = c(de), q = c(ie, true);
    d(ie);
    var ne = a(ie, 2), ce = c(ne);
    se(ce, { invisible: true, onclick: () => O(t(X)), children: (ye, Le) => {
      dr(ye, { width: "1.2rem" });
    }, $$slots: { default: true } }), d(ne), d(de), D(() => I(q, t(X))), u(B, de);
  });
  var Q = a(p, 2), A = c(Q);
  We(A), A.__keydown = [hr, E, e], A.__input = [_r, C, e, y], Mt(A, (B) => r(B), () => r());
  var M = a(A, 2);
  {
    var V = (B) => {
      var X = kr();
      F(X, "id", w), Fe(X, 21, () => e.datalist, Pt, (ue, oe, de, ie) => {
        var q = br(), ne = {};
        D(() => {
          ne !== (ne = t(oe)) && (q.value = (q.__value = t(oe)) == null ? "" : t(oe));
        }), u(ue, q);
      }), d(X), u(B, X);
    };
    ee(M, (B) => {
      e.datalist && e.datalist.length > 1 && B(V);
    });
  }
  d(Q), d(o);
  var W = a(o, 2), G = c(W), le = a(G);
  {
    var $ = (B) => {
      var X = Pe(), ue = te(X);
      {
        var oe = (de) => {
          var ie = xr(), q = c(ie, true);
          d(ie), D(() => I(q, e.errMsg || x.common.invalidInput)), u(de, ie);
        };
        ee(ue, (de) => {
          _() && de(oe);
        });
      }
      u(B, X);
    };
    ee(le, (B) => {
      _() && B($);
    });
  }
  d(W), d(P), D(() => {
    K(P, "width", k()), K(o, "max-height", R()), F(A, "type", l()), F(A, "id", v()), F(A, "name", e.name), F(A, "list", t(S)), F(A, "title", e.errMsg), F(A, "aria-label", m() || h()), F(A, "placeholder", h()), F(A, "aria-placeholder", h()), A.disabled = b(), F(A, "aria-disabled", b()), A.required = g() && f().length < 1, F(A, "aria-required", g() && f().length < 1), F(A, "maxlength", e.maxLength || void 0), F(A, "min", e.min || void 0), F(A, "max", e.max || void 0), F(A, "pattern", e.pattern), F(W, "for", v()), F(W, "data-required", g()), I(G, `${m() ?? ""} `);
  }), Ct("invalid", A, z), Ct("blur", A, N), Ge(A, () => t(y), (B) => s(y, B)), u(U, P), ae();
}
ze(["click", "keydown", "input"]);
var yr = j('<div class="err"> </div>'), Cr = j('<!> <!> <!> <p class="svelte-15tp6i6"><!></p> <!> <!> <!> <!>', 1), Pr = j('<div class="container svelte-15tp6i6"><!></div>');
function Lr(U, e) {
  re(e, true);
  let r = xe(), l = me(), v = L(void 0), f = L(""), m = L(""), h = L(""), b = L(true), g = L(i([])), _ = L(i([]));
  he(() => {
    requestAnimationFrame(() => {
      var _a;
      (_a = t(v)) == null ? void 0 : _a.focus();
    });
  });
  async function k(w, y) {
    var _a;
    if (e.clients.find((N) => N.id === t(m))) {
      s(f, i(l.common.nameExistsAlready));
      return;
    }
    s(f, "");
    let S = { id: t(m), name: t(h) || void 0, confidential: t(b), redirect_uris: t(g), post_logout_redirect_uris: t(_).length > 0 ? t(_) : void 0 }, O = await He(w.action, S);
    O.body ? e.onSave(O.body.id) : s(f, i(((_a = O.error) == null ? void 0 : _a.message) || "Error"));
  }
  var R = Pr(), x = c(R);
  Ve(x, { action: "/auth/v1/clients", onSubmit: k, children: (w, y) => {
    var S = Cr(), O = te(S);
    fe(O, { autocomplete: "off", label: "Client ID", placeholder: "Client ID", required: true, pattern: nr, get ref() {
      return t(v);
    }, set ref(M) {
      s(v, i(M));
    }, get value() {
      return t(m);
    }, set value(M) {
      s(m, i(M));
    } });
    var N = a(O, 2);
    fe(N, { autocomplete: "off", get label() {
      return l.clients.name;
    }, get placeholder() {
      return l.clients.name;
    }, pattern: pt, get value() {
      return t(h);
    }, set value(M) {
      s(h, i(M));
    } });
    var z = a(N, 2);
    ve(z, { get ariaLabel() {
      return l.clients.confidential;
    }, get checked() {
      return t(b);
    }, set checked(M) {
      s(b, i(M));
    }, children: (M, V) => {
      H();
      var W = J();
      D(() => I(W, l.clients.confidential)), u(M, W);
    }, $$slots: { default: true } });
    var E = a(z, 2), C = c(E);
    we(C, () => l.clients.descUri), d(E);
    var P = a(E, 2);
    be(P, { typ: "url", label: "Redirect URIs", get errMsg() {
      return l.validation.uri;
    }, get values() {
      return t(g);
    }, set values(M) {
      s(g, i(M));
    } });
    var o = a(P, 2);
    be(o, { typ: "url", label: "Post Logout Redirect URIs", get errMsg() {
      return l.validation.uri;
    }, get values() {
      return t(_);
    }, set values(M) {
      s(_, i(M));
    } });
    var p = a(o, 2);
    se(p, { type: "submit", children: (M, V) => {
      H();
      var W = J();
      D(() => I(W, r.common.save)), u(M, W);
    }, $$slots: { default: true } });
    var Q = a(p, 2);
    {
      var A = (M) => {
        var V = yr(), W = c(V, true);
        d(V), D(() => I(W, t(f))), u(M, V);
      };
      ee(Q, (M) => {
        t(f) && M(A);
      });
    }
    u(w, S);
  }, $$slots: { default: true } }), d(R), u(U, R), ae();
}
const De = "urn:ietf:params:oauth:grant-type:device_code";
var Sr = j('<div class="err"> </div>'), Ar = j('<div class="err"> </div>'), Tr = j('<h5> </h5> <!> <p class="desc svelte-5tvvvt"> </p> <!> <p class="desc svelte-5tvvvt"> </p> <!> <!> <div></div> <h5> </h5> <!> <!> <!> <p class="mb-0 svelte-5tvvvt">Authentication Flows</p> <!> <!> <!> <!> <!> <div></div> <p class="desc svelte-5tvvvt"> </p> <p class="desc svelte-5tvvvt"><strong> </strong></p> <!> <!> <!> <div></div> <p class="desc svelte-5tvvvt"> </p> <!> <p class="desc svelte-5tvvvt"><!></p> <!> <!> <div></div> <p class="desc svelte-5tvvvt"><!></p> <!> <!> <div></div> <p> </p> <!> <div></div> <p> </p> <p> </p> <!> <!> <div></div> <p> </p> <!> <div class="flex gap-05"><!> <!></div> <!> <div></div>', 1), Ir = j('<div class="container svelte-5tvvvt"><!></div>');
function Rr(U, e) {
  var _a, _b;
  re(e, true);
  let r = xe(), l = me();
  const v = "min(20rem, calc(100dvw - .5rem))";
  let f = L(""), m = L(false), h = L(i(e.client.id)), b = L(i(e.client.name || "")), g = L(i(e.client.enabled)), _ = L(i(e.client.confidential)), k = L(i(e.client.client_uri || "")), R = L(i(e.client.contacts ? Array.from(e.client.contacts) : [])), x = L(i(e.client.allowed_origins ? Array.from(e.client.allowed_origins) : [])), w = L(i(Array.from(e.client.redirect_uris))), y = L(i(e.client.post_logout_redirect_uris ? Array.from(e.client.post_logout_redirect_uris) : [])), S = i({ authorizationCode: e.client.flows_enabled.includes("authorization_code"), clientCredentials: e.client.flows_enabled.includes("client_credentials"), password: e.client.flows_enabled.includes("password"), refreshToken: e.client.flows_enabled.includes("refresh_token"), deviceCode: e.client.flows_enabled.includes(De) });
  const O = ["RS256", "RS384", "RS512", "EdDSA"];
  let N = L(i(e.client.access_token_alg)), z = L(i(e.client.id_token_alg)), E = L(i(e.client.access_token_lifetime.toString())), C = L(i(e.client.auth_code_lifetime.toString())), P = L(i(e.scopesAll.map((G) => ({ name: G, selected: e.client.scopes.includes(G) || false })))), o = L(i(e.scopesAll.map((G) => ({ name: G, selected: e.client.default_scopes.includes(G) || false })))), p = i({ plain: ((_a = e.client.challenges) == null ? void 0 : _a.includes("plain")) || false, s256: ((_b = e.client.challenges) == null ? void 0 : _b.includes("S256")) || false }), Q = L(i(e.client.force_mfa));
  he(() => {
    var _a2, _b2;
    e.client.id && (s(h, i(e.client.id)), s(b, i(e.client.name || "")), s(g, i(e.client.enabled)), s(_, i(e.client.confidential)), s(k, i(e.client.client_uri || "")), s(R, i(e.client.contacts ? Array.from(e.client.contacts) : [])), s(x, i(e.client.allowed_origins ? Array.from(e.client.allowed_origins) : [])), s(w, i(Array.from(e.client.redirect_uris))), s(y, i(e.client.post_logout_redirect_uris ? Array.from(e.client.post_logout_redirect_uris) : [])), S.authorizationCode = e.client.flows_enabled.includes("authorization_code"), S.clientCredentials = e.client.flows_enabled.includes("client_credentials"), S.password = e.client.flows_enabled.includes("password"), S.refreshToken = e.client.flows_enabled.includes("refresh_token"), S.deviceCode = e.client.flows_enabled.includes(De), s(N, i(e.client.access_token_alg)), s(z, i(e.client.id_token_alg)), s(E, i(e.client.access_token_lifetime.toString())), s(C, i(e.client.auth_code_lifetime.toString())), s(P, i(e.scopesAll.map((G) => ({ name: G, selected: e.client.scopes.includes(G) || false })))), s(o, i(e.scopesAll.map((G) => ({ name: G, selected: e.client.default_scopes.includes(G) || false })))), p.plain = ((_a2 = e.client.challenges) == null ? void 0 : _a2.includes("plain")) || false, p.s256 = ((_b2 = e.client.challenges) == null ? void 0 : _b2.includes("S256")) || false);
  });
  async function A(G, le) {
    s(f, "");
    let $ = { id: t(h), name: t(b) || void 0, enabled: t(g), confidential: t(_), allowed_origins: t(x).length > 0 ? t(x) : void 0, redirect_uris: t(w), post_logout_redirect_uris: t(y).length > 0 ? t(y) : void 0, flows_enabled: [], access_token_alg: t(N), id_token_alg: t(z), access_token_lifetime: Number.parseInt(t(E)), auth_code_lifetime: Number.parseInt(t(C)), scopes: t(P).filter((X) => X.selected).map((X) => X.name), default_scopes: t(o).filter((X) => X.selected).map((X) => X.name), challenges: void 0, force_mfa: t(Q), client_uri: t(k) || void 0, contacts: t(R).length > 0 ? t(R) : void 0 };
    S.authorizationCode && $.flows_enabled.push("authorization_code"), S.clientCredentials && $.flows_enabled.push("client_credentials"), S.password && $.flows_enabled.push("password"), S.refreshToken && $.flows_enabled.push("refresh_token"), S.deviceCode && $.flows_enabled.push(De), p.plain && ($.challenges = ["plain"]), p.s256 && ($.challenges ? $.challenges.push("S256") : $.challenges = ["S256"]);
    let B = await Ke(G.action, $);
    B.error ? s(f, i(B.error.message)) : (s(m, true), e.onSave(), setTimeout(() => {
      s(m, false);
    }, 2e3));
  }
  var M = Ir(), V = c(M);
  const W = _e(() => `/auth/v1/clients/${e.client.id}`);
  Ve(V, { get action() {
    return t(W);
  }, onSubmit: A, children: (G, le) => {
    var $ = Tr(), B = te($), X = c(B, true);
    d(B);
    var ue = a(B, 2);
    Oe(ue, { label: "ID", mono: true, children: (n, Y) => {
      H();
      var T = J();
      D(() => I(T, e.client.id)), u(n, T);
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
    var q = a(ie, 2), ne = c(q, true);
    d(q);
    var ce = a(q, 2);
    fe(ce, { typ: "url", autocomplete: "off", label: "URI", placeholder: "URI", width: v, pattern: Be, get value() {
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
    K(Le, "height", ".5rem");
    var Se = a(Le, 2), Bt = c(Se, true);
    d(Se);
    var Je = a(Se, 2);
    ve(Je, { get ariaLabel() {
      return l.common.enabled;
    }, get checked() {
      return t(g);
    }, set checked(n) {
      s(g, i(n));
    }, children: (n, Y) => {
      H();
      var T = J();
      D(() => I(T, l.common.enabled)), u(n, T);
    }, $$slots: { default: true } });
    var Qe = a(Je, 2);
    ve(Qe, { get ariaLabel() {
      return l.clients.confidential;
    }, get checked() {
      return t(_);
    }, set checked(n) {
      s(_, i(n));
    }, children: (n, Y) => {
      H();
      var T = J();
      D(() => I(T, l.clients.confidential)), u(n, T);
    }, $$slots: { default: true } });
    var Xe = a(Qe, 2);
    ve(Xe, { get ariaLabel() {
      return l.clients.forceMfa;
    }, get checked() {
      return t(Q);
    }, set checked(n) {
      s(Q, i(n));
    }, children: (n, Y) => {
      H();
      var T = J();
      D(() => I(T, l.clients.forceMfa)), u(n, T);
    }, $$slots: { default: true } });
    var Ye = a(Xe, 4);
    ve(Ye, { ariaLabel: "authorization_code", get checked() {
      return S.authorizationCode;
    }, set checked(n) {
      S.authorizationCode = n;
    }, children: (n, Y) => {
      H();
      var T = J("authorization_code");
      u(n, T);
    }, $$slots: { default: true } });
    var Ze = a(Ye, 2);
    ve(Ze, { ariaLabel: "urn:ietf:params:oauth:grant-type:device_code", get checked() {
      return S.deviceCode;
    }, set checked(n) {
      S.deviceCode = n;
    }, children: (n, Y) => {
      H();
      var T = J("device_code");
      u(n, T);
    }, $$slots: { default: true } });
    var $e = a(Ze, 2);
    ve($e, { ariaLabel: "client_credentials", get checked() {
      return S.clientCredentials;
    }, set checked(n) {
      S.clientCredentials = n;
    }, children: (n, Y) => {
      H();
      var T = J("client_credentials");
      u(n, T);
    }, $$slots: { default: true } });
    var et = a($e, 2);
    ve(et, { ariaLabel: "password", get checked() {
      return S.password;
    }, set checked(n) {
      S.password = n;
    }, children: (n, Y) => {
      H();
      var T = J("password");
      u(n, T);
    }, $$slots: { default: true } });
    var tt = a(et, 2);
    ve(tt, { ariaLabel: "refresh_token", get checked() {
      return S.refreshToken;
    }, set checked(n) {
      S.refreshToken = n;
    }, children: (n, Y) => {
      H();
      var T = J("refresh_token");
      u(n, T);
    }, $$slots: { default: true } });
    var rt = a(tt, 2);
    K(rt, "height", ".5rem");
    var Ae = a(rt, 2), Ot = c(Ae, true);
    d(Ae);
    var Te = a(Ae, 2), at = c(Te), Dt = c(at, true);
    d(at), d(Te);
    var lt = a(Te, 2);
    ve(lt, { ariaLabel: "PKCE plain", get checked() {
      return p.plain;
    }, set checked(n) {
      p.plain = n;
    }, children: (n, Y) => {
      H();
      var T = J("plain");
      u(n, T);
    }, $$slots: { default: true } });
    var it = a(lt, 2);
    ve(it, { ariaLabel: "PKCE S256", get checked() {
      return p.s256;
    }, set checked(n) {
      p.s256 = n;
    }, children: (n, Y) => {
      H();
      var T = J("S256");
      u(n, T);
    }, $$slots: { default: true } });
    var nt = a(it, 2);
    {
      var qt = (n) => {
        var Y = Sr(), T = c(Y, true);
        d(Y), D(() => I(T, l.clients.errConfidentialPKCE)), Zt(3, Y, () => $t, () => ({ duration: 150 })), u(n, Y);
      };
      ee(nt, (n) => {
        !t(_) && !p.plain && !p.s256 && n(qt);
      });
    }
    var st = a(nt, 2);
    K(st, "height", ".5rem");
    var Ie = a(st, 2), jt = c(Ie, true);
    d(Ie);
    var ot = a(Ie, 2);
    be(ot, { typ: "url", label: "Allowed Origins", get errMsg() {
      return l.validation.origin;
    }, pattern: or, get values() {
      return t(x);
    }, set values(n) {
      s(x, i(n));
    } });
    var Re = a(ot, 2), Ft = c(Re);
    we(Ft, () => l.clients.descUri), d(Re);
    var dt = a(Re, 2);
    be(dt, { typ: "url", label: "Redirect URIs", get errMsg() {
      return l.validation.uri;
    }, get required() {
      return S.authorizationCode;
    }, pattern: Be, get values() {
      return t(w);
    }, set values(n) {
      s(w, i(n));
    } });
    var ct = a(dt, 2);
    be(ct, { typ: "url", label: "Post Logout Redirect URIs", get errMsg() {
      return l.validation.uri;
    }, pattern: Be, get values() {
      return t(y);
    }, set values(n) {
      s(y, i(n));
    } });
    var vt = a(ct, 2);
    K(vt, "height", ".5rem");
    var pe = a(vt, 2), zt = c(pe);
    we(zt, () => l.clients.scopes.desc), d(pe);
    var ut = a(pe, 2);
    St(ut, { get items() {
      return t(P);
    }, set items(n) {
      s(P, i(n));
    }, children: (n, Y) => {
      H();
      var T = J();
      D(() => I(T, l.clients.scopes.allowed)), u(n, T);
    }, $$slots: { default: true } });
    var ft = a(ut, 2);
    St(ft, { get items() {
      return t(o);
    }, set items(n) {
      s(o, i(n));
    }, children: (n, Y) => {
      H();
      var T = J();
      D(() => I(T, l.clients.scopes.default)), u(n, T);
    }, $$slots: { default: true } });
    var _t = a(ft, 2);
    K(_t, "height", ".75rem");
    var Ee = a(_t, 2), Ht = c(Ee, true);
    d(Ee);
    var ht = a(Ee, 2);
    fe(ht, { typ: "number", autocomplete: "off", label: "Token Lifetime", placeholder: "Token Lifetime", width: v, min: "10", max: "86400", errMsg: "10 <= Token Lifetime <= 86400", get value() {
      return t(E);
    }, set value(n) {
      s(E, i(n));
    } });
    var mt = a(ht, 2);
    K(mt, "height", ".5rem");
    var Me = a(mt, 2), Kt = c(Me, true);
    d(Me);
    var Ue = a(Me, 2), Vt = c(Ue, true);
    d(Ue);
    var gt = a(Ue, 2);
    Oe(gt, { label: "Access Token Algorithm", children: (n, Y) => {
      At(n, { ariaLabel: "Access Token Algorithm", options: O, borderless: true, get value() {
        return t(N);
      }, set value(T) {
        s(N, i(T));
      } });
    } });
    var bt = a(gt, 2);
    Oe(bt, { label: "ID Token Algorithm", children: (n, Y) => {
      At(n, { ariaLabel: "ID Token Algorithm", options: O, borderless: true, get value() {
        return t(z);
      }, set value(T) {
        s(z, i(T));
      } });
    } });
    var kt = a(bt, 2);
    K(kt, "height", ".5rem");
    var Ne = a(kt, 2), Wt = c(Ne, true);
    d(Ne);
    var xt = a(Ne, 2);
    fe(xt, { typ: "number", autocomplete: "off", label: "Auth Code Lifetime", placeholder: "Auth Code Lifetime", width: v, min: "10", max: "300", errMsg: "10 <= Auth Code Lifetime <= 300", get value() {
      return t(C);
    }, set value(n) {
      s(C, i(n));
    } });
    var Ce = a(xt, 2);
    K(Ce, "margin-top", "1rem");
    var wt = c(Ce);
    se(wt, { type: "submit", children: (n, Y) => {
      H();
      var T = J();
      D(() => I(T, r.common.save)), u(n, T);
    }, $$slots: { default: true } });
    var Gt = a(wt, 2);
    {
      var Jt = (n) => {
        Nt(n, {});
      };
      ee(Gt, (n) => {
        t(m) && n(Jt);
      });
    }
    d(Ce);
    var yt = a(Ce, 2);
    {
      var Qt = (n) => {
        var Y = Ar(), T = c(Y, true);
        d(Y), D(() => I(T, t(f))), u(n, Y);
      };
      ee(yt, (n) => {
        t(f) && n(Qt);
      });
    }
    var Xt = a(yt, 2);
    K(Xt, "height", "1rem"), D(() => {
      I(X, l.common.information), I(de, l.clients.descName), I(ne, l.clients.descClientUri), I(Bt, l.clients.config), I(Ot, l.clients.descPKCE), I(Dt, l.clients.descPKCEEnforce), I(jt, l.clients.descOrigin), I(Ht, l.clients.tokenLifetime.p1), I(Kt, l.clients.tokenLifetime.p2), I(Vt, l.clients.tokenLifetime.p3), I(Wt, l.clients.descAuthCode);
    }), u(G, $);
  }, $$slots: { default: true } }), d(M), u(U, M), ae();
}
var pr = j("<!> <!>", 1), Er = j('<div class="container svelte-19xcrje"><div class="err"> </div> <!></div>');
function Mr(U, e) {
  re(e, true);
  let r = me(), l = L(""), v = L(void 0);
  he(() => {
    s(l, ""), s(v, ""), e.client.confidential ? f() : s(l, i(r.clients.confidentialNoSecret));
  });
  async function f() {
    let x = await He(`/auth/v1/clients/${e.client.id}/secret`);
    h(x);
  }
  async function m() {
    let x = await Ke(`/auth/v1/clients/${e.client.id}/secret`);
    h(x);
  }
  function h(x) {
    var _a;
    x.body ? x.body.secret && s(v, i(x.body.secret)) : s(l, i(((_a = x.error) == null ? void 0 : _a.message) || "Error"));
  }
  var b = Er(), g = c(b), _ = c(g, true);
  d(g);
  var k = a(g, 2);
  {
    var R = (x) => {
      var w = pr(), y = te(w);
      cr(y, { autocomplete: "off", label: "Client Secret", placeholder: "Client Secret", disabled: true, showCopy: true, get value() {
        return t(v);
      }, set value(O) {
        s(v, i(O));
      } });
      var S = a(y, 2);
      se(S, { onclick: m, children: (O, N) => {
        H();
        var z = J();
        D(() => I(z, r.clients.generateSecret)), u(O, z);
      }, $$slots: { default: true } }), u(x, w);
    };
    ee(k, (x) => {
      t(v) && x(R);
    });
  }
  d(b), D(() => I(_, t(l))), u(U, b), ae();
}
var Ur = j('<div class="err"> </div>'), Nr = j("<p> </p> <!> <!>", 1);
function Br(U, e) {
  re(e, true);
  let r = xe(), l = me(), v = L("");
  async function f() {
    s(v, "");
    let R = await Rt(`/auth/v1/clients/${e.client.id}`);
    R.error ? s(v, i(R.error.message)) : e.onSave();
  }
  var m = Nr(), h = te(m), b = c(h, true);
  d(h);
  var g = a(h, 2);
  se(g, { level: -1, onclick: f, children: (R, x) => {
    H();
    var w = J();
    D(() => I(w, r.common.delete)), u(R, w);
  }, $$slots: { default: true } });
  var _ = a(g, 2);
  {
    var k = (R) => {
      var x = Ur(), w = c(x, true);
      d(x), D(() => I(w, t(v))), u(R, x);
    };
    ee(_, (R) => {
      t(v) && R(k);
    });
  }
  D(() => I(b, l.clients.delete1)), u(U, m), ae();
}
var Or = j('<div><div class="flex"><input type="range" class="svelte-12j8dit"> <div class="value font-mono svelte-12j8dit"> </div></div> <div class="label svelte-12j8dit"><label class="font-label noselect svelte-12j8dit"> </label></div></div>');
function qe(U, e) {
  re(e, true);
  let r = Z(e, "value", 15), l = Z(e, "label", 3, ""), v = Z(e, "disabled", 3, false), f = Z(e, "step", 3, 1), m = Z(e, "widthRange", 3, "15rem");
  const h = ke();
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
  var g = Or(), _ = c(g), k = c(_);
  We(k), F(k, "id", h);
  var R = a(k, 2), x = c(R, true);
  d(R), d(_);
  var w = a(_, 2), y = c(w);
  F(y, "for", h);
  var S = c(y, true);
  d(y), d(w), d(g), D(() => {
    F(k, "name", e.name), F(k, "title", l()), F(k, "aria-label", l()), k.disabled = v(), F(k, "aria-disabled", v()), F(k, "min", e.min), F(k, "max", e.max), F(k, "step", f()), K(k, "width", m()), K(k, "--bg-slider", t(b)), I(x, r()), I(S, l());
  }), Ge(k, r), u(U, g), ae();
}
var Dr = j('<div class="outer svelte-1evkwuy"><div class="container svelte-1evkwuy"><div><label class="font-label svelte-1evkwuy"> </label> <div class="values svelte-1evkwuy"><!> <!> <!></div></div> <div class="color svelte-1evkwuy"></div></div></div>');
function ge(U, e) {
  re(e, true);
  let r = Z(e, "h", 15), l = Z(e, "s", 15), v = Z(e, "l", 15);
  const f = ke(), m = "15rem";
  let h = _e(() => `hsl(${r()} ${l()} ${v()})`);
  var b = Dr(), g = c(b), _ = c(g), k = c(_);
  F(k, "for", f);
  var R = c(k, true);
  d(k);
  var x = a(k, 2);
  F(x, "id", f);
  var w = c(x);
  qe(w, { label: "Hue", min: 0, max: 359, widthRange: m, bgMode: "hue", get hue() {
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
  qe(y, { label: "Sat", min: 0, max: 100, widthRange: m, bgMode: "sat", get hue() {
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
  qe(S, { label: "Lum", min: 0, max: 100, widthRange: m, bgMode: "lum", get hue() {
    return r();
  }, get sat() {
    return l();
  }, get lum() {
    return v();
  }, get value() {
    return v();
  }, set value(N) {
    v(N);
  } }), d(x), d(_);
  var O = a(_, 2);
  d(g), d(b), D(() => {
    K(g, "border-color", t(h)), I(R, e.label), K(O, "background", t(h));
  }), u(U, b), ae();
}
var qr = (U, e) => {
  var _a;
  return (_a = t(e)) == null ? void 0 : _a.click();
}, jr = j('<div class="container svelte-2nx18n"><!> <div role="none" class="mask svelte-2nx18n"><div class="colorWrapper svelte-2nx18n"><div class="color svelte-2nx18n"></div></div> <input type="color" class="svelte-2nx18n"></div></div>');
function je(U, e) {
  re(e, true);
  let r = Z(e, "value", 15), l = me(), v = L(void 0);
  var f = jr(), m = c(f);
  fe(m, { get label() {
    return e.label;
  }, get placeholder() {
    return e.label;
  }, get errMsg() {
    return l.validation.css;
  }, width: "17.5rem", required: true, pattern: Et, get value() {
    return r();
  }, set value(g) {
    r(g);
  } });
  var h = a(m, 2);
  h.__click = [qr, v];
  var b = a(c(h), 2);
  We(b), Mt(b, (g) => s(v, g), () => t(v)), d(h), d(f), D(() => K(f, "--color", r())), Ge(b, r), u(U, f), ae();
}
ze(["click"]);
var Fr = j('<div><p> </p> <div class="hsl svelte-10udpph"><div><!> <!> <!> <!></div> <div><!> <!> <!></div></div> <div><p><!></p> <!> <!> <!></div></div>');
function Tt(U, e) {
  re(e, true);
  let r = Z(e, "values", 15), l = me();
  var v = Fr(), f = c(v), m = c(f, true);
  d(f);
  var h = a(f, 2), b = c(h), g = c(b);
  ge(g, { label: "--text", get h() {
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
  var _ = a(g, 2);
  ge(_, { label: "--text-high", get h() {
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
  var k = a(_, 2);
  ge(k, { label: "--bg", get h() {
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
  ge(R, { label: "--bg-high", get h() {
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
  ge(w, { label: "--action", get h() {
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
  ge(y, { label: "--accent", get h() {
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
  ge(S, { label: "--error", get h() {
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
  } }), d(x), d(h);
  var O = a(h, 2), N = c(O), z = c(N);
  we(z, () => l.clients.branding.descFullCss), d(N);
  var E = a(N, 2);
  je(E, { label: "--btn-text", get value() {
    return r().btn_text;
  }, set value(o) {
    r(r().btn_text = o, true);
  } });
  var C = a(E, 2);
  je(C, { label: "--theme-sun", get value() {
    return r().theme_sun;
  }, set value(o) {
    r(r().theme_sun = o, true);
  } });
  var P = a(C, 2);
  je(P, { label: "--theme-moon", get value() {
    return r().theme_moon;
  }, set value(o) {
    r(r().theme_moon = o, true);
  } }), d(O), d(v), D(() => I(m, l.clients.branding.descHsl)), u(U, v), ae();
}
var zr = (U) => U.preventDefault(), Hr = j('<div aria-label="Preview: All components inside are only for theme and colors preview and have no effect or interaction"><div class="inner svelte-fkmn75"><div class="container svelte-fkmn75"><div class="logo svelte-fkmn75"><img alt="Client Logo"></div> <h3 class="svelte-fkmn75">Header</h3> <p>Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam</p> <p><span class="svelte-fkmn75">--accent-color</span></p> <!> <div class="btn svelte-fkmn75"><div class="svelte-fkmn75"><!> <br> <!> <br> <!> <br></div> <div class="svelte-fkmn75"><!> <br> <!> <br> <!></div></div> <a>Link</a> <br> <!></div></div></div>');
function It(U, e) {
  re(e, true);
  let r = L(false);
  he(() => {
    t(r) && setTimeout(() => {
      s(r, false);
    }, 2e3);
  });
  var l = Hr(), v = c(l), f = c(v), m = c(f), h = c(m);
  d(m);
  var b = a(m, 8);
  fe(b, { label: "Preview", placeholder: "Preview", width: "12.5rem" });
  var g = a(b, 2), _ = c(g), k = c(_);
  se(k, { level: 1, get isLoading() {
    return t(r);
  }, onclick: () => s(r, true), children: (E, C) => {
    H();
    var P = J("Button");
    u(E, P);
  }, $$slots: { default: true } });
  var R = a(k, 4);
  se(R, { level: 2, get isLoading() {
    return t(r);
  }, onclick: () => s(r, true), children: (E, C) => {
    H();
    var P = J("Button");
    u(E, P);
  }, $$slots: { default: true } });
  var x = a(R, 4);
  se(x, { level: 3, get isLoading() {
    return t(r);
  }, onclick: () => s(r, true), children: (E, C) => {
    H();
    var P = J("Button");
    u(E, P);
  }, $$slots: { default: true } }), H(2), d(_);
  var w = a(_, 2), y = c(w);
  se(y, { level: -1, get isLoading() {
    return t(r);
  }, onclick: () => s(r, true), children: (E, C) => {
    H();
    var P = J("Button");
    u(E, P);
  }, $$slots: { default: true } });
  var S = a(y, 4);
  se(S, { level: -2, get isLoading() {
    return t(r);
  }, onclick: () => s(r, true), children: (E, C) => {
    H();
    var P = J("Button");
    u(E, P);
  }, $$slots: { default: true } });
  var O = a(S, 4);
  se(O, { level: -3, get isLoading() {
    return t(r);
  }, onclick: () => s(r, true), children: (E, C) => {
    H();
    var P = J("Button");
    u(E, P);
  }, $$slots: { default: true } }), d(w), d(g);
  var N = a(g, 2);
  F(N, "href", window.location.href), N.__click = [zr];
  var z = a(N, 4);
  ur(z, {}), d(f), d(v), d(l), D((E, C, P, o, p, Q, A) => {
    K(l, "--border-radius", e.borderRadius), K(l, "--text", E), K(l, "--text-high", C), K(l, "--bg", P), K(l, "--bg-high", o), K(l, "--action", p), K(l, "--accent", Q), K(l, "--error", A), K(l, "--btn-text", e.theme.btn_text), K(l, "--theme-sun", e.theme.theme_sun), K(l, "--theme-moon", e.theme.theme_moon), F(h, "src", e.logoUrl);
  }, [() => e.theme.text.join(" "), () => e.theme.text_high.join(" "), () => e.theme.bg.join(" "), () => e.theme.bg_high.join(" "), () => e.theme.action.join(" "), () => e.theme.accent.join(" "), () => e.theme.error.join(" ")]), u(U, l), ae();
}
ze(["click"]);
var Kr = j('<div><h2>Preview</h2> <!> <div class="preview svelte-1nlq99j"><!></div></div>');
function Vr(U, e) {
  re(e, true);
  let r = ["Light Theme", "Dark Theme"], l = L(i(r[0]));
  var v = Kr(), f = a(c(v), 2);
  Ut(f, { tabs: r, center: true, get selected() {
    return t(l);
  }, set selected(_) {
    s(l, i(_));
  } });
  var m = a(f, 2), h = c(m);
  {
    var b = (_) => {
      It(_, { get logoUrl() {
        return e.logoUrl;
      }, get borderRadius() {
        return e.theme.border_radius;
      }, get theme() {
        return e.theme.light;
      } });
    }, g = (_) => {
      It(_, { get logoUrl() {
        return e.logoUrl;
      }, get borderRadius() {
        return e.theme.border_radius;
      }, get theme() {
        return e.theme.dark;
      } });
    };
    ee(h, (_) => {
      t(l) === r[0] ? _(b) : _(g, false);
    });
  }
  d(m), d(v), u(U, v), ae();
}
var Wr = j('<div class="err"> </div>'), Gr = j('<div class="values svelte-1pkqupw"><p><!></p> <!> <h1>Light Theme</h1> <!> <br> <h1>Dark Theme</h1> <!></div> <div class="preview"><!> <hr> <p>Logo Upload</p> <!> <div class="buttons svelte-1pkqupw"><!> <!> <!></div> <!></div>', 1), Jr = j('<div class="container svelte-1pkqupw"><!></div>');
function Qr(U, e) {
  re(e, true);
  const r = "13rem";
  let l = xe(), v = me(), f = L(false), m = L(""), h = L(void 0), b = L(i(ke())), g = _e(() => `/auth/v1/clients/${e.client.id}/logo?${t(b)}`), _ = _e(() => `/auth/v1/theme/${e.client.id}`);
  he(() => {
    k();
  });
  async function k() {
    var _a;
    let y = await He(t(_));
    y.body ? (y.body.client_id === "rauthy" && console.log("using Rauthy default scheme"), s(h, i(y.body))) : s(m, i(((_a = y.error) == null ? void 0 : _a.message) || "Error"));
  }
  async function R(y, S) {
    if (!t(h)) {
      console.error("theme is undefined");
      return;
    }
    let O = t(h);
    O.client_id = e.client.id;
    let N = await Ke(t(_), O);
    N.error ? s(m, i(N.error.message)) : (s(f, true), setTimeout(() => {
      s(f, false);
    }, 2e3));
  }
  async function x() {
    let y = await Rt(t(_));
    y.error ? s(m, i(y.error.message)) : (await k(), s(f, true), setTimeout(() => {
      s(f, false);
    }, 2e3));
  }
  async function w() {
    s(b, i(ke()));
  }
  Ve(U, { get action() {
    return t(_);
  }, onSubmit: R, children: (y, S) => {
    var O = Jr(), N = c(O);
    {
      var z = (E) => {
        var C = Gr(), P = te(C), o = c(P), p = c(o);
        we(p, () => v.clients.branding.descVariables), d(o);
        var Q = a(o, 2);
        fe(Q, { label: "--border-radius", placeholder: "--border-radius", get errMsg() {
          return v.validation.css;
        }, width: r, required: true, pattern: Et, get value() {
          return t(h).border_radius;
        }, set value(q) {
          t(h).border_radius = q;
        } });
        var A = a(Q, 4);
        Tt(A, { get values() {
          return t(h).light;
        }, set values(q) {
          t(h).light = q;
        } });
        var M = a(A, 6);
        Tt(M, { get values() {
          return t(h).dark;
        }, set values(q) {
          t(h).dark = q;
        } }), d(P);
        var V = a(P, 2), W = c(V);
        vr(W, () => t(b), (q) => {
          Vr(q, { get logoUrl() {
            return t(g);
          }, get theme() {
            return t(h);
          } });
        });
        var G = a(W, 6);
        const le = _e(() => `/auth/v1/clients/${e.client.id}/logo`);
        fr(G, { method: "PUT", get url() {
          return t(le);
        }, fileName: "logo", onSuccess: w });
        var $ = a(G, 2), B = c($);
        se(B, { type: "submit", children: (q, ne) => {
          H();
          var ce = J();
          D(() => I(ce, l.common.save)), u(q, ce);
        }, $$slots: { default: true } });
        var X = a(B, 2);
        se(X, { level: -2, onclick: x, children: (q, ne) => {
          H();
          var ce = J();
          D(() => I(ce, v.common.reset)), u(q, ce);
        }, $$slots: { default: true } });
        var ue = a(X, 2);
        {
          var oe = (q) => {
            Nt(q, {});
          };
          ee(ue, (q) => {
            t(f) && q(oe);
          });
        }
        d($);
        var de = a($, 2);
        {
          var ie = (q) => {
            var ne = Wr(), ce = c(ne, true);
            d(ne), D(() => I(ce, t(m))), u(q, ne);
          };
          ee(de, (q) => {
            t(m) && q(ie);
          });
        }
        d(V), u(E, C);
      };
      ee(N, (E) => {
        t(h) && E(z);
      });
    }
    d(O), u(y, O);
  }, $$slots: { default: true } }), ae();
}
var Xr = j('<div class="flex"><!></div> <div class="details"><!></div>', 1);
function Yr(U, e) {
  re(e, true);
  let r = xe(), l = me();
  const v = [l.nav.config, "Secret", "Branding", r.common.delete];
  let f = L(i(v[0])), m = L(void 0);
  he(() => {
    e.client.id && requestAnimationFrame(() => {
      var _a;
      (_a = t(m)) == null ? void 0 : _a();
    });
  });
  var h = Xr(), b = te(h), g = c(b);
  Ut(g, { tabs: v, get selected() {
    return t(f);
  }, set selected(w) {
    s(f, i(w));
  }, get focusFirst() {
    return t(m);
  }, set focusFirst(w) {
    s(m, i(w));
  } }), d(b);
  var _ = a(b, 2), k = c(_);
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
        var O = (z) => {
          Mr(z, { get client() {
            return e.client;
          } });
        }, N = (z) => {
          var E = Pe(), C = te(E);
          {
            var P = (p) => {
              Qr(p, { get client() {
                return e.client;
              } });
            }, o = (p) => {
              var Q = Pe(), A = te(Q);
              {
                var M = (V) => {
                  Br(V, { get client() {
                    return e.client;
                  }, get onSave() {
                    return e.onSave;
                  } });
                };
                ee(A, (V) => {
                  t(f) === r.common.delete && V(M);
                }, true);
              }
              u(p, Q);
            };
            ee(C, (p) => {
              t(f) === "Branding" ? p(P) : p(o, false);
            }, true);
          }
          u(z, E);
        };
        ee(S, (z) => {
          t(f) === "Secret" ? z(O) : z(N, false);
        }, true);
      }
      u(w, y);
    };
    ee(k, (w) => {
      t(f) === l.nav.config ? w(R) : w(x, false);
    });
  }
  d(_), u(U, h), ae();
}
var Zr = j("<div></div> <!>", 1), $r = j("<!> <!>", 1), ea = j('<div class="err"> </div>'), ta = j('<!> <div id="groups"><!></div>', 1), ra = j("<!> <!>", 1);
function aa(U, e) {
  re(e, true);
  let r = L(void 0), l = L(""), v = L(i([])), f = L(i([])), m = L(void 0), h = ir("cid"), b = L(i([]));
  const g = ["ID"];
  let _ = L(i(g[0])), k = L("");
  const R = ["ID"];
  Yt(() => {
    x(), w();
  }), he(() => {
    s(m, i(t(v).find((C) => C.id === h.get())));
  }), he(() => {
    let C = t(k).toLowerCase();
    C ? t(_) === g[0] && s(f, i(t(v).filter((P) => P.id.toLowerCase().includes(C)))) : s(f, i(t(v)));
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
  async function O(C) {
    var _a;
    (_a = t(r)) == null ? void 0 : _a(), await x(), h.set(C);
  }
  var N = ra(), z = te(N);
  ar(z, { paddingTop: "2.1rem", buttonTilesAriaControls: "groups", width: "min(20rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (P) => {
    var o = Zr(), p = te(o);
    K(p, "height", ".5rem");
    var Q = a(p, 2);
    Fe(Q, 17, () => t(f), (A) => A.id, (A, M, V, W) => {
      const G = _e(() => h.get() === t(M).id);
      rr(A, { onclick: () => h.set(t(M).id), get selected() {
        return t(G);
      }, children: (le, $) => {
        H();
        var B = J();
        D(() => I(B, t(M).id)), u(le, B);
      } });
    }), u(P, o);
  }, children: (P, o) => {
    var p = $r(), Q = te(p);
    const A = _e(() => t(v).length === 0 ? 1 : 2);
    er(Q, { get level() {
      return t(A);
    }, alignRight: true, get closeModal() {
      return t(r);
    }, set closeModal(V) {
      s(r, i(V));
    }, children: (V, W) => {
      Lr(V, { onSave: O, get clients() {
        return t(v);
      } });
    }, $$slots: { default: true } });
    var M = a(Q, 2);
    lr(M, { searchOptions: g, orderOptions: R, onChangeOrder: y, searchWidth: "min(19.5rem, 100dvw - .5rem)", get searchOption() {
      return t(_);
    }, set searchOption(V) {
      s(_, i(V));
    }, get value() {
      return t(k);
    }, set value(V) {
      s(k, i(V));
    } }), u(P, p);
  }, $$slots: { buttonTiles: true, default: true } });
  var E = a(z, 2);
  tr(E, { children: (C, P) => {
    var o = ta(), p = te(o);
    {
      var Q = (W) => {
        var G = ea(), le = c(G, true);
        d(G), D(() => I(le, t(l))), u(W, G);
      };
      ee(p, (W) => {
        t(l) && W(Q);
      });
    }
    var A = a(p, 2), M = c(A);
    {
      var V = (W) => {
        Yr(W, { get client() {
          return t(m);
        }, get clients() {
          return t(v);
        }, get scopesAll() {
          return t(b);
        }, onSave: S });
      };
      ee(M, (W) => {
        t(m) && W(V);
      });
    }
    d(A), u(C, o);
  } }), u(U, N), ae();
}
function ja(U) {
  aa(U, {});
}
export {
  ja as component
};
