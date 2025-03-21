import { t as L, a as m, e as Y, d as He } from "../chunks/DLBGyKVC.js";
import { p as se, s as o, c, r as u, t as z, a as le, f as X, a8 as J, j as e, a9 as ie, k as M, aa as ke, l as n, a6 as qe, P as Qe } from "../chunks/CmQi0fbH.js";
import { s as S } from "../chunks/BjaYyaa_.js";
import { i as $ } from "../chunks/C6bK2EJJ.js";
import { e as De } from "../chunks/YQCw2eEa.js";
import { s as we, a as ze } from "../chunks/DOlUUCkJ.js";
import { p as C } from "../chunks/B_ggA-0N.js";
import { d as Xe, f as Ze, c as et, b as Ae } from "../chunks/BO1A6s0c.js";
import { C as tt } from "../chunks/Bfi4YUpT.js";
import { N as rt } from "../chunks/ClbMy9q0.js";
import { u as at } from "../chunks/DJVZKROe.js";
import { N as it } from "../chunks/DpqPmJAJ.js";
import { B as ot } from "../chunks/BLvNyGE2.js";
import { p as U } from "../chunks/DNJm3-SG.js";
import { k as nt } from "../chunks/DRMO-JgS.js";
import { t as st, s as lt, B as Ce } from "../chunks/DPLO-ozG.js";
import { I as Q } from "../chunks/DwPr_s7h.js";
import { u as fe } from "../chunks/CZf6fJph.js";
import { g as Le } from "../chunks/B21bTIl7.js";
import { I as Ue } from "../chunks/CAzQQhB1.js";
import { I as dt } from "../chunks/CgbF3u05.js";
import { I as ct } from "../chunks/Clnxc71_.js";
import { F as $e } from "../chunks/CNMQp9ma.js";
import { u as Re } from "../chunks/DGTOa5g8.js";
import { L as Ve } from "../chunks/gwKtuDud.js";
import { I as pe } from "../chunks/CFiOSVzB.js";
import { I as ut } from "../chunks/CZJtqSUy.js";
import { i as _t, a as oe, j as vt, g as mt } from "../chunks/gfDO7tLr.js";
import { h as he } from "../chunks/UlQQkuaW.js";
import { E as pt } from "../chunks/BWK4fHb9.js";
import { T as ft } from "../chunks/MKf726CY.js";
import { C as gt } from "../chunks/B7t3YI0Q.js";
import { O as ht } from "../chunks/DqhsozSK.js";
var bt = L('<div class="desc svelte-6fcb3i"><h4 class="svelte-6fcb3i">ID token claim mappings</h4> <p class="svelte-6fcb3i"> </p> <p class="svelte-6fcb3i"><!></p> <p class="svelte-6fcb3i"><!></p> <p class="svelte-6fcb3i"><!></p> <p class="svelte-6fcb3i"><!></p> <p class="svelte-6fcb3i"><!></p></div>');
function Pt(R, a) {
  se(a, true);
  let t = fe();
  var f = bt(), h = o(c(f), 2), b = c(h, true);
  u(h);
  var p = o(h, 2), s = c(p);
  he(s, () => t.providers.config.jsonPath.p2), u(p);
  var x = o(p, 2), y = c(x);
  he(y, () => t.providers.config.jsonPath.p3), u(x);
  var g = o(x, 2), r = c(g);
  he(r, () => t.providers.config.jsonPath.p4), u(g);
  var l = o(g, 2), _ = c(l);
  he(_, () => t.providers.config.jsonPath.p5), u(l);
  var E = o(l, 2), d = c(E);
  he(d, () => t.providers.config.jsonPath.p6), u(E), u(f), z(() => S(b, t.providers.config.jsonPath.p1)), m(R, f), le();
}
var kt = L('<img alt="No Logo Available">');
function Ct(R, a) {
  se(a, true);
  var t = kt();
  we(t, "width", 20), we(t, "height", 20), z((f) => we(t, "src", f), [() => `/auth/v1/providers/${a.providerId}/img?${Le()}}`]), m(R, t), le();
}
var wt = L('<div class="checkbox"><!></div>'), It = L('<div class="checkbox"><!></div> <!>', 1);
function Te(R, a) {
  se(a, true);
  let t = U(a, "dangerAllowInsecure", 15), f = U(a, "showRootPem", 15), h = U(a, "rootPemCert", 15), b = fe();
  var p = It(), s = X(p), x = c(s);
  pe(x, { get ariaLabel() {
    return b.providers.config.custRootCa;
  }, get checked() {
    return f();
  }, set checked(l) {
    f(l);
  }, children: (l, _) => {
    J();
    var E = Y();
    z(() => S(E, b.providers.config.custRootCa)), m(l, E);
  }, $$slots: { default: true } }), u(s);
  var y = o(s, 2);
  {
    var g = (l) => {
      dt(l, { rows: 17, name: "rootPem", get label() {
        return b.providers.config.rootPemCert;
      }, placeholder: `-----BEGIN CERTIFICATE-----
...
-----END CERTIFICATE-----`, get disabled() {
        return a.disabled;
      }, errMsg: "-----BEGIN CERTIFICATE----- ...", width: "min(40rem, calc(100dvw - 1.75rem))", fontMono: true, pattern: _t, get value() {
        return h();
      }, set value(_) {
        h(_);
      } });
    }, r = (l) => {
      var _ = wt(), E = c(_);
      pe(E, { get ariaLabel() {
        return b.providers.config.allowInsecureTls;
      }, get checked() {
        return t();
      }, set checked(d) {
        t(d);
      }, children: (d, I) => {
        J();
        var P = Y();
        z(() => S(P, b.providers.config.allowInsecureTls)), m(d, P);
      }, $$slots: { default: true } }), u(_), m(l, _);
    };
    $(y, (l) => {
      f() ? l(g) : l(r, false);
    });
  }
  m(R, p), le();
}
var xt = L("<!> <!> <!> <!>", 1);
function We(R, a) {
  se(a, true);
  let t = U(a, "issuer", 15), f = U(a, "authorizationEndpoint", 15), h = U(a, "tokenEndpoint", 15), b = U(a, "userinfoEndpoint", 15);
  var p = xt(), s = X(p);
  Q(s, { autocomplete: "off", label: "Issuer URL", placeholder: "Issuer URL", required: true, pattern: oe, get width() {
    return a.inputWidth;
  }, get disabled() {
    return a.disabled;
  }, get value() {
    return t();
  }, set value(r) {
    t(r);
  } });
  var x = o(s, 2);
  Q(x, { typ: "url", autocomplete: "off", label: "Authorization Endpoint", placeholder: "Authorization Endpoint", required: true, pattern: oe, get width() {
    return a.inputWidth;
  }, get disabled() {
    return a.disabled;
  }, get value() {
    return f();
  }, set value(r) {
    f(r);
  } });
  var y = o(x, 2);
  Q(y, { typ: "url", autocomplete: "off", label: "Token Endpoint", placeholder: "Token Endpoint", required: true, pattern: oe, get width() {
    return a.inputWidth;
  }, get disabled() {
    return a.disabled;
  }, get value() {
    return h();
  }, set value(r) {
    h(r);
  } });
  var g = o(y, 2);
  Q(g, { typ: "url", autocomplete: "off", label: "Userinfo Endpoint", placeholder: "Userinfo Endpoint", required: true, pattern: oe, get width() {
    return a.inputWidth;
  }, get disabled() {
    return a.disabled;
  }, get value() {
    return b();
  }, set value(r) {
    b(r);
  } }), m(R, p), le();
}
var yt = L('<div class="err"> </div>'), Et = L('<p class="desc svelte-1u8yru7"> </p> <!> <p class="desc svelte-1u8yru7"> </p> <!> <p class="desc svelte-1u8yru7"> </p> <!> <p class="desc svelte-1u8yru7"> </p> <!> <p><!></p> <div class="checkbox"><!></div> <div class="checkbox"><!></div> <!> <!> <p class="desc svelte-1u8yru7"> </p> <!> <!> <p class="desc svelte-1u8yru7"> </p> <!> <!>', 1);
function je(R, a) {
  se(a, true);
  let t = U(a, "scope", 15), f = U(a, "name", 15), h = U(a, "clientId", 15), b = U(a, "clientSecret", 15), p = U(a, "clientSecretBasic", 15), s = U(a, "clientSecretPost", 15), x = U(a, "adminClaimPath", 15), y = U(a, "adminClaimValue", 15), g = U(a, "mfaClaimPath", 15), r = U(a, "mfaClaimValue", 15), l = fe();
  var _ = Et(), E = X(_), d = c(E, true);
  u(E);
  var I = o(E, 2);
  Q(I, { autocomplete: "off", label: "Scope", placeholder: "openid profile email", required: true, pattern: vt, get width() {
    return a.inputWidth;
  }, get value() {
    return t();
  }, set value(A) {
    t(A);
  } });
  var P = o(I, 2), T = c(P, true);
  u(P);
  var W = o(P, 2);
  Q(W, { autocomplete: "off", get label() {
    return l.providers.config.clientName;
  }, get placeholder() {
    return l.providers.config.clientName;
  }, required: true, pattern: mt, get width() {
    return a.inputWidth;
  }, get value() {
    return f();
  }, set value(A) {
    f(A);
  } });
  var j = o(W, 2), G = c(j, true);
  u(j);
  var N = o(j, 2);
  Q(N, { autocomplete: "off", label: "Client ID", placeholder: "Client ID", required: true, pattern: oe, get width() {
    return a.inputWidth;
  }, get value() {
    return h();
  }, set value(A) {
    h(A);
  } });
  var F = o(N, 2), te = c(F, true);
  u(F);
  var H = o(F, 2);
  const de = ie(() => !a.usePKCE);
  ct(H, { autocomplete: "off", label: "Client Secret", placeholder: "Client Secret", maxLength: 256, get errMsg() {
    return l.providers.config.errConfidential;
  }, get required() {
    return e(de);
  }, get width() {
    return a.inputWidth;
  }, get value() {
    return b();
  }, set value(A) {
    b(A);
  } });
  var Z = o(H, 2), O = c(Z);
  he(O, () => l.providers.config.descAuthMethod), u(Z);
  var q = o(Z, 2), B = c(q);
  pe(B, { ariaLabel: "client_secret_basic", get checked() {
    return p();
  }, set checked(A) {
    p(A);
  }, children: (A, ve) => {
    J();
    var w = Y("client_secret_basic");
    m(A, w);
  }, $$slots: { default: true } }), u(q);
  var v = o(q, 2), ce = c(v);
  pe(ce, { ariaLabel: "client_secret_post", get checked() {
    return s();
  }, set checked(A) {
    s(A);
  }, children: (A, ve) => {
    J();
    var w = Y("client_secret_post");
    m(A, w);
  }, $$slots: { default: true } }), u(v);
  var re = o(v, 2);
  {
    var _e = (A) => {
      var ve = yt(), w = c(ve, true);
      u(ve), z(() => S(w, l.providers.config.errNoAuthMethod)), st(3, ve, () => lt, () => ({ duration: 150 })), m(A, ve);
    };
    $(re, (A) => {
      !a.usePKCE && !p() && !s() && A(_e);
    });
  }
  var ae = o(re, 2);
  Pt(ae, {});
  var ne = o(ae, 2), i = c(ne, true);
  u(ne);
  var V = o(ne, 2);
  Q(V, { autocomplete: "off", get label() {
    return l.providers.config.pathAdminClaim;
  }, placeholder: "$.roles.*", pattern: oe, get width() {
    return a.inputWidth;
  }, get value() {
    return x();
  }, set value(A) {
    x(A);
  } });
  var D = o(V, 2);
  Q(D, { autocomplete: "off", get label() {
    return l.providers.config.valueAdminClaim;
  }, placeholder: "rauthy_admin", pattern: oe, get width() {
    return a.inputWidth;
  }, get value() {
    return y();
  }, set value(A) {
    y(A);
  } });
  var ge = o(D, 2), Ie = c(ge, true);
  u(ge);
  var be = o(ge, 2);
  Q(be, { autocomplete: "off", get label() {
    return l.providers.config.pathMfaClaim;
  }, placeholder: "$.amr.*", pattern: oe, get width() {
    return a.inputWidth;
  }, get value() {
    return g();
  }, set value(A) {
    g(A);
  } });
  var xe = o(be, 2);
  Q(xe, { autocomplete: "off", get label() {
    return l.providers.config.valueMfaClaim;
  }, placeholder: "mfa", pattern: oe, get width() {
    return a.inputWidth;
  }, get value() {
    return r();
  }, set value(A) {
    r(A);
  } }), z(() => {
    S(d, l.providers.config.descScope), S(T, l.providers.config.descClientName), S(G, l.providers.config.descClientId), S(te, l.providers.config.descClientSecret), S(i, l.providers.config.mapUser), S(Ie, l.providers.config.mapMfa);
  }), m(R, _), le();
}
var St = L('<div class="svelte-jts2zb"><!></div>'), At = L('<span class="err"> </span>'), Lt = L('<!> <div></div> <div class="checkbox svelte-jts2zb"><!></div> <!> <!> <div class="checkbox svelte-jts2zb"><!></div> <!> <div class="logo svelte-jts2zb"><!> <!></div> <div class="flex gap-05"><!> <!> <!></div>', 1), Tt = L('<div class="container svelte-jts2zb"><!></div>');
function zt(R, a) {
  se(a, true);
  let t = U(a, "provider", 15), f = Re(), h = fe();
  const b = "min(calc(100dvw - .5rem), 30rem)";
  let p = false, s = M(""), x = M(false), y = M(!!t().root_pem), g = M(C(Le()));
  ke(() => {
    t().id && (t(t().client_secret = t().client_secret || "", true), t(t().admin_claim_path = t().admin_claim_path || "", true), t(t().admin_claim_value = t().admin_claim_value || "", true), t(t().mfa_claim_path = t().mfa_claim_path || "", true), t(t().mfa_claim_value = t().mfa_claim_value || "", true), t(t().root_pem = t().root_pem || "", true));
  }), ke(() => {
    t().scope && t(t().scope = t().scope.replaceAll("+", " "), true);
  });
  async function r(d, I) {
    if (n(s, ""), t().client_secret && !(t().client_secret_basic || t().client_secret_post)) {
      n(s, C(h.providers.config.errNoAuthMethod));
      return;
    }
    if (!t().use_pkce && !t().client_secret) {
      n(s, C(h.providers.config.errConfidential));
      return;
    }
    let P = { name: t().name, typ: t().typ, enabled: t().enabled, issuer: t().issuer, authorization_endpoint: t().authorization_endpoint, token_endpoint: t().token_endpoint, userinfo_endpoint: t().userinfo_endpoint, danger_allow_insecure: e(y) && t().root_pem ? false : t().danger_allow_insecure, use_pkce: t().use_pkce, client_secret_basic: t().client_secret_basic, client_secret_post: t().client_secret_post, client_id: t().client_id, client_secret: t().client_secret || void 0, scope: t().scope.trim(), root_pem: e(y) && t().root_pem ? t().root_pem.trim() : void 0, admin_claim_path: t().admin_claim_path || void 0, admin_claim_value: t().admin_claim_value || void 0, mfa_claim_path: t().mfa_claim_path || void 0, mfa_claim_value: t().mfa_claim_value || void 0 }, T = await Xe(d.action, P);
    T.error ? n(s, C(T.error.message)) : (n(x, true), a.onSave(), setTimeout(() => {
      n(x, false);
    }, 3e3));
  }
  var l = Tt(), _ = c(l);
  const E = ie(() => `/auth/v1/providers/${t().id}`);
  $e(_, { get action() {
    return e(E);
  }, onSubmit: r, children: (d, I) => {
    var P = Lt(), T = X(P);
    Ve(T, { label: "ID", mono: true, children: (i, V) => {
      J();
      var D = Y();
      z(() => S(D, t().id)), m(i, D);
    }, $$slots: { default: true } });
    var W = o(T, 2);
    ze(W, "", {}, { height: ".15rem" });
    var j = o(W, 2), G = c(j);
    pe(G, { get ariaLabel() {
      return h.common.enabled;
    }, get checked() {
      return t().enabled;
    }, set checked(i) {
      t(t().enabled = i, true);
    }, children: (i, V) => {
      J();
      var D = Y();
      z(() => S(D, h.common.enabled)), m(i, D);
    }, $$slots: { default: true } }), u(j);
    var N = o(j, 2);
    Te(N, { get dangerAllowInsecure() {
      return t().danger_allow_insecure;
    }, set dangerAllowInsecure(i) {
      t(t().danger_allow_insecure = i, true);
    }, get showRootPem() {
      return e(y);
    }, set showRootPem(i) {
      n(y, C(i));
    }, get rootPemCert() {
      return t().root_pem;
    }, set rootPemCert(i) {
      t(t().root_pem = i, true);
    } });
    var F = o(N, 2);
    We(F, { inputWidth: b, get issuer() {
      return t().issuer;
    }, set issuer(i) {
      t(t().issuer = i, true);
    }, get authorizationEndpoint() {
      return t().authorization_endpoint;
    }, set authorizationEndpoint(i) {
      t(t().authorization_endpoint = i, true);
    }, get tokenEndpoint() {
      return t().token_endpoint;
    }, set tokenEndpoint(i) {
      t(t().token_endpoint = i, true);
    }, get userinfoEndpoint() {
      return t().userinfo_endpoint;
    }, set userinfoEndpoint(i) {
      t(t().userinfo_endpoint = i, true);
    } });
    var te = o(F, 2), H = c(te);
    pe(H, { ariaLabel: "PKCE", get checked() {
      return t().use_pkce;
    }, set checked(i) {
      t(t().use_pkce = i, true);
    }, children: (i, V) => {
      J();
      var D = Y("PKCE");
      m(i, D);
    }, $$slots: { default: true } }), u(te);
    var de = o(te, 2);
    je(de, { get usePKCE() {
      return t().use_pkce;
    }, inputWidth: b, get scope() {
      return t().scope;
    }, set scope(i) {
      t(t().scope = i, true);
    }, get name() {
      return t().name;
    }, set name(i) {
      t(t().name = i, true);
    }, get clientId() {
      return t().client_id;
    }, set clientId(i) {
      t(t().client_id = i, true);
    }, get clientSecret() {
      return t().client_secret;
    }, set clientSecret(i) {
      t(t().client_secret = i, true);
    }, get clientSecretBasic() {
      return t().client_secret_basic;
    }, set clientSecretBasic(i) {
      t(t().client_secret_basic = i, true);
    }, get clientSecretPost() {
      return t().client_secret_post;
    }, set clientSecretPost(i) {
      t(t().client_secret_post = i, true);
    }, get adminClaimPath() {
      return t().admin_claim_path;
    }, set adminClaimPath(i) {
      t(t().admin_claim_path = i, true);
    }, get adminClaimValue() {
      return t().admin_claim_value;
    }, set adminClaimValue(i) {
      t(t().admin_claim_value = i, true);
    }, get mfaClaimPath() {
      return t().mfa_claim_path;
    }, set mfaClaimPath(i) {
      t(t().mfa_claim_path = i, true);
    }, get mfaClaimValue() {
      return t().mfa_claim_value;
    }, set mfaClaimValue(i) {
      t(t().mfa_claim_value = i, true);
    } });
    var Z = o(de, 2), O = c(Z);
    nt(O, () => e(g), (i) => {
      var V = St(), D = c(V);
      Ct(D, { get providerId() {
        return t().id;
      } }), u(V), m(i, V);
    });
    var q = o(O, 2);
    const B = ie(() => `/auth/v1/providers/${t().id}/img`);
    ut(q, { method: "PUT", get url() {
      return e(B);
    }, fileName: "logo", onSuccess: () => n(g, C(Le())) }), u(Z);
    var v = o(Z, 2), ce = c(v);
    Ce(ce, { type: "submit", get isLoading() {
      return p;
    }, children: (i, V) => {
      J();
      var D = Y();
      z(() => S(D, f.common.save)), m(i, D);
    }, $$slots: { default: true } });
    var re = o(ce, 2);
    {
      var _e = (i) => {
        Ue(i, {});
      };
      $(re, (i) => {
        e(x) && i(_e);
      });
    }
    var ae = o(re, 2);
    {
      var ne = (i) => {
        var V = At(), D = c(V, true);
        u(V), z(() => S(D, e(s))), m(i, V);
      };
      $(ae, (i) => {
        e(s) && i(ne);
      });
    }
    u(v), m(d, P);
  }, $$slots: { default: true } }), u(l), m(R, l), le();
}
var Rt = L('<div class="user svelte-1vu6qeb"> <span class="muted font-mono svelte-1vu6qeb"> </span></div>'), Nt = L('<div class="forceDelete svelte-1vu6qeb"> </div>'), Mt = L('<p class="err"><b> </b></p> <p> </p> <!> <p> </p> <!> <!>', 1), qt = L("<p> </p> <!>", 1), Dt = L('<div class="err"> </div>'), Ut = L('<div class="container svelte-1vu6qeb"><!> <!></div>');
function $t(R, a) {
  se(a, true);
  let t = Re(), f = fe(), h = M(true), b = M(""), p = M(false), s = M(C([]));
  qe(async () => {
    let d = await Ze(`/auth/v1/providers/${a.provider.id}/delete_safe`);
    d.status === 406 && d.error && n(s, C(d.error)), n(h, false);
  });
  async function x() {
    n(b, ""), n(h, true);
    let d = await et(`/auth/v1/providers/${a.provider.id}`);
    d.error ? n(b, C(d.error.message)) : a.onSave(), n(h, false);
  }
  var y = Ut(), g = c(y);
  {
    var r = (d) => {
      var I = Mt(), P = X(I), T = c(P), W = c(T, true);
      u(T), u(P);
      var j = o(P, 2), G = c(j, true);
      u(j);
      var N = o(j, 2);
      pt(N, { summary: (B) => {
        J();
        var v = Y();
        z(() => S(v, f.providers.delete.linkedUsers)), m(B, v);
      }, details: (B) => {
        var v = He(), ce = X(v);
        De(ce, 17, () => e(s), (re) => re.id, (re, _e) => {
          var ae = Rt(), ne = c(ae), i = o(ne), V = c(i);
          u(i), u(ae), z(() => {
            S(ne, `${e(_e).email ?? ""} `), S(V, `/ ${e(_e).id ?? ""}`);
          }), m(re, ae);
        }), m(B, v);
      }, $$slots: { summary: true, details: true } });
      var F = o(N, 2), te = c(F, true);
      u(F);
      var H = o(F, 2);
      pe(H, { get ariaLabel() {
        return f.providers.delete.forceDelete;
      }, get checked() {
        return e(p);
      }, set checked(O) {
        n(p, C(O));
      }, children: (O, q) => {
        var B = Nt(), v = c(B, true);
        u(B), z(() => S(v, f.providers.delete.forceDelete)), m(O, B);
      }, $$slots: { default: true } });
      var de = o(H, 2);
      {
        var Z = (O) => {
          Ce(O, { level: -1, onclick: x, get isLoading() {
            return e(h);
          }, children: (q, B) => {
            J();
            var v = Y();
            z(() => S(v, t.common.delete)), m(q, v);
          }, $$slots: { default: true } });
        };
        $(de, (O) => {
          e(p) && O(Z);
        });
      }
      z(() => {
        S(W, f.providers.delete.isInUse1), S(G, f.providers.delete.isInUse2), S(te, f.providers.delete.areYouSure);
      }), m(d, I);
    }, l = (d) => {
      var I = qt(), P = X(I), T = c(P, true);
      u(P);
      var W = o(P, 2);
      Ce(W, { level: -1, onclick: x, get isLoading() {
        return e(h);
      }, children: (j, G) => {
        J();
        var N = Y();
        z(() => S(N, t.common.delete)), m(j, N);
      }, $$slots: { default: true } }), z(() => S(T, f.providers.delete.areYouSure)), m(d, I);
    };
    $(g, (d) => {
      e(s).length > 0 ? d(r) : d(l, false);
    });
  }
  var _ = o(g, 2);
  {
    var E = (d) => {
      var I = Dt(), P = c(I, true);
      u(I), z(() => S(P, e(b))), m(d, I);
    };
    $(_, (d) => {
      e(b) && d(E);
    });
  }
  u(y), m(R, y), le();
}
var Vt = L('<div class="flex"><!></div> <!>', 1);
function Wt(R, a) {
  se(a, true);
  let t = U(a, "provider", 15), f = U(a, "onSave", 15), h = fe();
  const b = [h.tabs.config, h.tabs.delete];
  let p = M(C(b[0]));
  var s = Vt(), x = X(s), y = c(x);
  ft(y, { tabs: b, get selected() {
    return e(p);
  }, set selected(_) {
    n(p, C(_));
  } }), u(x);
  var g = o(x, 2);
  {
    var r = (_) => {
      zt(_, { get provider() {
        return t();
      }, set provider(E) {
        t(E);
      }, get onSave() {
        return f();
      }, set onSave(E) {
        f(E);
      } });
    }, l = (_, E) => {
      {
        var d = (I) => {
          $t(I, { get provider() {
            return t();
          }, get onSave() {
            return f();
          }, set onSave(P) {
            f(P);
          } });
        };
        $(_, (I) => {
          e(p) === h.tabs.delete && I(d);
        }, E);
      }
    };
    $(g, (_) => {
      e(p) === h.tabs.config ? _(r) : _(l, false);
    });
  }
  m(R, s), le();
}
var jt = L('<!> <div class="checkbox svelte-qeky4o"><!></div> <!>', 1), Bt = L('<div class="err"> </div>'), Kt = L('<!> <!> <div class="flex gap-05"><!> <!> <!> <!></div>', 1), Gt = L('<div class="container svelte-qeky4o"><!> <div></div> <!></div>');
function Ft(R, a) {
  se(a, true);
  let t = Re(), f = fe();
  const h = "min(calc(100dvw - 1.75rem), 30rem)";
  let b = M(false), p = M(""), s = M(false), x = M(false), y = M(false), g = M(C({ issuer: "", metadata_url: "", danger_allow_insecure: false, root_pem: "" })), r = M(C({ enabled: true, typ: "oidc", issuer: "", danger_allow_insecure: false, authorization_endpoint: "", token_endpoint: "", token_auth_method_basic: false, userinfo_endpoint: "", use_pkce: true, client_secret_basic: true, client_secret_post: false, name: "", client_id: "", client_secret: "", scope: "", root_pem: "", admin_claim_path: "", admin_claim_value: "", mfa_claim_path: "", mfa_claim_value: "" })), l = ["OIDC", "Auto", "Custom", "Github", "Google"], _ = M(C(l[0])), E = ie(() => e(_) === "Auto"), d = ie(() => e(_) === "Custom"), I = ie(() => e(_) === "OIDC"), P = ie(() => !e(E) && !e(d) && !e(I)), T = ie(() => !(e(P) || e(d) || e(s)));
  const W = "/auth/v1/providers/lookup";
  let j = ie(() => e(T) ? W : "/auth/v1/providers/create");
  ke(() => {
    if (e(_)) switch (n(s, false), n(g, C({ issuer: "", metadata_url: "", danger_allow_insecure: false, root_pem: "" })), e(_)) {
      case "Github":
        n(r, C({ enabled: true, issuer: "github.com", typ: "github", danger_allow_insecure: false, authorization_endpoint: "https://github.com/login/oauth/authorize", token_endpoint: "https://github.com/login/oauth/access_token", client_secret_basic: true, client_secret_post: true, userinfo_endpoint: "https://api.github.com/user", use_pkce: false, name: "Github", client_id: "", client_secret: "", scope: "user:email", root_pem: "", admin_claim_path: "", admin_claim_value: "", mfa_claim_path: "$.two_factor_authentication", mfa_claim_value: "true" })), n(s, true);
        break;
      case "Google":
        n(g, C({ issuer: "accounts.google.com", metadata_url: "", danger_allow_insecure: false, root_pem: "" })), Qe(() => {
          F(W);
        });
        break;
      default:
        n(r, C({ enabled: true, issuer: "", typ: "oidc", danger_allow_insecure: false, authorization_endpoint: "", token_endpoint: "", client_secret_basic: true, client_secret_post: false, userinfo_endpoint: "", use_pkce: true, name: "", client_id: "", client_secret: "", scope: "", root_pem: "", admin_claim_path: "", admin_claim_value: "", mfa_claim_path: "", mfa_claim_value: "" }));
    }
  }), ke(() => {
    e(x) && setTimeout(() => {
      a.onSave(), te();
    }, 1500);
  });
  async function G(q, B) {
    e(T) ? await F(q.action) : await N(q.action);
  }
  async function N(q) {
    if (e(r).client_secret && !(e(r).client_secret_basic || e(r).client_secret_post)) {
      n(p, C(f.providers.config.errNoAuthMethod));
      return;
    }
    if (!e(r).use_pkce && !e(r).client_secret) {
      n(p, C(f.providers.config.errConfidential));
      return;
    }
    n(p, ""), n(b, true);
    let B = { name: e(r).name, typ: e(E) ? "custom" : e(_).toLowerCase(), enabled: e(r).enabled, issuer: e(r).issuer, authorization_endpoint: e(r).authorization_endpoint, token_endpoint: e(r).token_endpoint, userinfo_endpoint: e(r).userinfo_endpoint, danger_allow_insecure: e(y) && e(r).root_pem ? false : e(r).danger_allow_insecure, use_pkce: e(r).use_pkce, client_secret_basic: e(r).client_secret_basic, client_secret_post: e(r).client_secret_post, client_id: e(r).client_id, client_secret: e(r).client_secret, scope: e(r).scope.trim(), root_pem: e(y) && e(r).root_pem ? e(r).root_pem.trim() : void 0, admin_claim_path: e(r).admin_claim_path || void 0, admin_claim_value: e(r).admin_claim_value || void 0, mfa_claim_path: e(r).mfa_claim_path || void 0, mfa_claim_value: e(r).mfa_claim_value || void 0 }, v = await Ae(q, B);
    v.error ? v.error.message.includes("InvalidCertificate") ? n(p, "Insecure connection not allowed") : n(p, C(v.error.message)) : n(x, true), n(b, false);
  }
  async function F(q) {
    if (!e(g).issuer && !e(g).metadata_url) {
      n(p, "Provide at least one of Issuer / Metadata URL");
      return;
    }
    n(p, ""), n(b, true);
    let B = { issuer: e(g).issuer || void 0, metadata_url: e(g).metadata_url || void 0, danger_allow_insecure: e(g).danger_allow_insecure, root_pem: e(g).root_pem || void 0 }, v = await Ae(q, B);
    v.body ? (e(r).issuer = v.body.issuer, e(r).authorization_endpoint = v.body.authorization_endpoint, e(r).danger_allow_insecure = v.body.danger_allow_insecure, e(r).token_endpoint = v.body.token_endpoint, e(r).userinfo_endpoint = v.body.userinfo_endpoint, e(r).client_secret_basic = v.body.client_secret_basic, e(r).client_secret_post = v.body.client_secret_post, e(r).use_pkce = v.body.use_pkce, e(r).client_secret_basic = v.body.client_secret_basic, e(r).client_secret_post = !v.body.client_secret_basic && v.body.client_secret_post, e(r).scope = v.body.scope, e(r).root_pem = v.body.root_pem, n(s, true)) : v.error && (v.error.message.includes("InvalidCertificate") ? n(p, "Insecure connection not allowed") : n(p, C(v.error.message))), n(b, false);
  }
  function te() {
    n(g, C({ issuer: "", metadata_url: "", danger_allow_insecure: false, root_pem: "" })), n(r, C({ enabled: true, issuer: "", typ: "oidc", danger_allow_insecure: false, authorization_endpoint: "", token_endpoint: "", userinfo_endpoint: "", use_pkce: true, name: "", client_id: "", client_secret_basic: true, client_secret_post: false, scope: "", root_pem: "", admin_claim_path: "", admin_claim_value: "", mfa_claim_path: "", mfa_claim_value: "" })), n(x, false), n(s, false), n(y, false);
  }
  var H = Gt(), de = c(H);
  ht(de, { ariaLabel: "Select Mode", options: l, get value() {
    return e(_);
  }, set value(q) {
    n(_, C(q));
  } });
  var Z = o(de, 2);
  ze(Z, "", {}, { height: ".5rem" });
  var O = o(Z, 2);
  $e(O, { get action() {
    return e(j);
  }, onSubmit: G, children: (q, B) => {
    var v = Kt(), ce = X(v);
    {
      var re = (w) => {
        Te(w, { get dangerAllowInsecure() {
          return e(r).danger_allow_insecure;
        }, set dangerAllowInsecure(K) {
          e(r).danger_allow_insecure = K;
        }, get showRootPem() {
          return e(y);
        }, set showRootPem(K) {
          n(y, C(K));
        }, get rootPemCert() {
          return e(r).root_pem;
        }, set rootPemCert(K) {
          e(r).root_pem = K;
        } });
      }, _e = (w, K) => {
        {
          var ue = (me) => {
            Te(me, { get dangerAllowInsecure() {
              return e(g).danger_allow_insecure;
            }, set dangerAllowInsecure(ee) {
              e(g).danger_allow_insecure = ee;
            }, get showRootPem() {
              return e(y);
            }, set showRootPem(ee) {
              n(y, C(ee));
            }, get rootPemCert() {
              return e(g).root_pem;
            }, set rootPemCert(ee) {
              e(g).root_pem = ee;
            } });
          };
          $(w, (me) => {
            e(s) || me(ue);
          }, K);
        }
      };
      $(ce, (w) => {
        e(d) ? w(re) : w(_e, false);
      });
    }
    var ae = o(ce, 2);
    {
      var ne = (w) => {
        Q(w, { name: "issuer", label: "Issuer URL", placeholder: "Issuer URL", width: h, required: true, pattern: oe, get value() {
          return e(g).issuer;
        }, set value(K) {
          e(g).issuer = K;
        } });
      }, i = (w, K) => {
        {
          var ue = (ee) => {
            Q(ee, { typ: "url", name: "metadata", label: "Metadata URL", placeholder: ".../.well-known/openid-configuration", width: h, required: true, pattern: oe, get value() {
              return e(g).metadata_url;
            }, set value(ye) {
              e(g).metadata_url = ye;
            } });
          }, me = (ee, ye) => {
            {
              var Be = (Ee) => {
                var Ne = jt(), Me = X(Ne);
                We(Me, { inputWidth: h, get disabled() {
                  return e(s);
                }, get issuer() {
                  return e(r).issuer;
                }, set issuer(k) {
                  e(r).issuer = k;
                }, get authorizationEndpoint() {
                  return e(r).authorization_endpoint;
                }, set authorizationEndpoint(k) {
                  e(r).authorization_endpoint = k;
                }, get tokenEndpoint() {
                  return e(r).token_endpoint;
                }, set tokenEndpoint(k) {
                  e(r).token_endpoint = k;
                }, get userinfoEndpoint() {
                  return e(r).userinfo_endpoint;
                }, set userinfoEndpoint(k) {
                  e(r).userinfo_endpoint = k;
                } });
                var Se = o(Me, 2), Ke = c(Se);
                {
                  var Ge = (k) => {
                    Ve(k, { label: "PKCE", children: (Pe, Ye) => {
                      gt(Pe, { get checked() {
                        return e(r).use_pkce;
                      } });
                    }, $$slots: { default: true } });
                  }, Fe = (k) => {
                    pe(k, { ariaLabel: "PKCE", get checked() {
                      return e(r).use_pkce;
                    }, set checked(Pe) {
                      e(r).use_pkce = Pe;
                    }, children: (Pe, Ye) => {
                      J();
                      var Je = Y("PKCE");
                      m(Pe, Je);
                    }, $$slots: { default: true } });
                  };
                  $(Ke, (k) => {
                    e(s) ? k(Ge) : k(Fe, false);
                  });
                }
                u(Se);
                var Oe = o(Se, 2);
                je(Oe, { get usePKCE() {
                  return e(r).use_pkce;
                }, inputWidth: h, get scope() {
                  return e(r).scope;
                }, set scope(k) {
                  e(r).scope = k;
                }, get name() {
                  return e(r).name;
                }, set name(k) {
                  e(r).name = k;
                }, get clientId() {
                  return e(r).client_id;
                }, set clientId(k) {
                  e(r).client_id = k;
                }, get clientSecret() {
                  return e(r).client_secret;
                }, set clientSecret(k) {
                  e(r).client_secret = k;
                }, get clientSecretBasic() {
                  return e(r).client_secret_basic;
                }, set clientSecretBasic(k) {
                  e(r).client_secret_basic = k;
                }, get clientSecretPost() {
                  return e(r).client_secret_post;
                }, set clientSecretPost(k) {
                  e(r).client_secret_post = k;
                }, get adminClaimPath() {
                  return e(r).admin_claim_path;
                }, set adminClaimPath(k) {
                  e(r).admin_claim_path = k;
                }, get adminClaimValue() {
                  return e(r).admin_claim_value;
                }, set adminClaimValue(k) {
                  e(r).admin_claim_value = k;
                }, get mfaClaimPath() {
                  return e(r).mfa_claim_path;
                }, set mfaClaimPath(k) {
                  e(r).mfa_claim_path = k;
                }, get mfaClaimValue() {
                  return e(r).mfa_claim_value;
                }, set mfaClaimValue(k) {
                  e(r).mfa_claim_value = k;
                } }), m(Ee, Ne);
              };
              $(ee, (Ee) => {
                (e(P) || e(d) || e(s)) && Ee(Be);
              }, ye);
            }
          };
          $(w, (ee) => {
            e(E) && !e(s) ? ee(ue) : ee(me, false);
          }, K);
        }
      };
      $(ae, (w) => {
        e(I) && !e(s) ? w(ne) : w(i, false);
      });
    }
    var V = o(ae, 2), D = c(V);
    Ce(D, { type: "submit", get isLoading() {
      return e(b);
    }, children: (w, K) => {
      J();
      var ue = Y();
      z(() => S(ue, e(T) ? f.providers.config.lookup : t.common.save)), m(w, ue);
    }, $$slots: { default: true } });
    var ge = o(D, 2);
    {
      var Ie = (w) => {
        Ce(w, { level: 3, onclick: te, get isLoading() {
          return e(b);
        }, children: (K, ue) => {
          J();
          var me = Y();
          z(() => S(me, f.common.reset)), m(K, me);
        }, $$slots: { default: true } });
      };
      $(ge, (w) => {
        e(T) || w(Ie);
      });
    }
    var be = o(ge, 2);
    {
      var xe = (w) => {
        Ue(w, {});
      };
      $(be, (w) => {
        e(x) && w(xe);
      });
    }
    var A = o(be, 2);
    {
      var ve = (w) => {
        var K = Bt(), ue = c(K, true);
        u(K), z(() => S(ue, e(p))), m(w, K);
      };
      $(A, (w) => {
        e(p) && w(ve);
      });
    }
    u(V), m(q, v);
  }, $$slots: { default: true } }), u(H), m(R, H), le();
}
var Ot = L("<div></div> <!>", 1), Yt = L('<div class="err"> </div>'), Jt = L('<div id="federation"><!></div> <!>', 1), Ht = L("<!> <!>", 1);
function Qt(R, a) {
  se(a, true);
  let t = fe(), f = M(void 0), h = at("pid"), b = M(""), p = M(C([])), s = M(void 0);
  qe(() => {
    x();
  }), ke(() => {
    n(s, C(e(p).find((_) => _.id === h.get())));
  });
  async function x() {
    var _a;
    let _ = await Ae("/auth/v1/providers");
    _.body ? n(p, C(_.body)) : n(b, C(((_a = _.error) == null ? void 0 : _a.message) || "Error"));
  }
  function y() {
    var _a;
    h.set(void 0), (_a = e(f)) == null ? void 0 : _a(), x();
  }
  var g = Ht(), r = X(g);
  rt(r, { width: "11rem", buttonTilesAriaControls: "federation", paddingTop: "6.65rem", buttonTiles: (E) => {
    var d = Ot(), I = X(d);
    ze(I, "", {}, { height: ".5rem" });
    var P = o(I, 2);
    De(P, 17, () => e(p), (T) => T.id, (T, W, j, G) => {
      const N = ie(() => h.get() === e(W).id);
      it(T, { onclick: () => h.set(e(W).id), get selected() {
        return e(N);
      }, children: (F, te) => {
        J();
        var H = Y();
        z(() => S(H, e(W).name)), m(F, H);
      } });
    }), m(E, d);
  }, children: (E, d) => {
    const I = ie(() => e(p).length === 0 ? 1 : 2);
    ot(E, { get level() {
      return e(I);
    }, get closeModal() {
      return e(f);
    }, set closeModal(P) {
      n(f, C(P));
    }, children: (P, T) => {
      Ft(P, { onSave: y });
    }, $$slots: { default: true } });
  }, $$slots: { buttonTiles: true, default: true } });
  var l = o(r, 2);
  tt(l, { children: (_, E) => {
    var d = Jt(), I = X(d), P = c(I);
    {
      var T = (G) => {
        Wt(G, { onSave: x, get provider() {
          return e(s);
        }, set provider(N) {
          n(s, C(N));
        } });
      };
      $(P, (G) => {
        e(s) && G(T);
      });
    }
    u(I);
    var W = o(I, 2);
    {
      var j = (G) => {
        var N = Yt(), F = c(N, true);
        u(N), z(() => S(F, e(b))), m(G, N);
      };
      $(W, (G) => {
        e(b) && G(j);
      });
    }
    z(() => we(I, "aria-label", t.common.details)), m(_, d);
  } }), m(R, g), le();
}
function Tr(R) {
  Qt(R, {});
}
export {
  Tr as component
};
