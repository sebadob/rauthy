import { t as A, a as m, e as O, d as He } from "../chunks/BxmJRzoY.js";
import { p as ne, s as o, c as u, r as c, t as T, a as se, f as Q, a3 as Y, a4 as ae, k as e, j as N, a1 as fe, a5 as ke, l as n, a0 as qe, F as Qe } from "../chunks/w0HvPX0p.js";
import { s as E } from "../chunks/BzP2S3Z_.js";
import { i as U } from "../chunks/iO9_dPNE.js";
import { e as De } from "../chunks/S81raU5Y.js";
import { s as we, a as ze } from "../chunks/BdbQ6g_y.js";
import { d as Xe, f as Ze, c as et, b as Ae } from "../chunks/UPFlzoow.js";
import { C as tt } from "../chunks/C6jTHtu1.js";
import { N as rt } from "../chunks/CzQKNE2_.js";
import { u as at } from "../chunks/Bz_VUaol.js";
import { N as it } from "../chunks/D9abP6hj.js";
import { B as ot } from "../chunks/CyiD59-2.js";
import { p as D } from "../chunks/C6GSeq7M.js";
import { k as nt } from "../chunks/DDNkWuIk.js";
import { t as st, s as lt, B as Ce } from "../chunks/C8YTstTD.js";
import { I as H } from "../chunks/DVXwAhn3.js";
import { u as pe } from "../chunks/Bt_9UXew.js";
import { g as Le } from "../chunks/B21bTIl7.js";
import { I as Ue } from "../chunks/CTshzOVc.js";
import { I as dt } from "../chunks/Dc2IDHgC.js";
import { I as ut } from "../chunks/iedauS3r.js";
import { F as $e } from "../chunks/BEbxeS8S.js";
import { u as Re } from "../chunks/0cG6LBdy.js";
import { L as Ve } from "../chunks/DQwTjYxX.js";
import { I as me } from "../chunks/ScYc5fRW.js";
import { I as ct } from "../chunks/C39TjCb0.js";
import { i as _t, a as ie, j as vt, g as mt } from "../chunks/gfDO7tLr.js";
import { h as he } from "../chunks/C2ZdIFW_.js";
import { E as pt } from "../chunks/755dAfGH.js";
import { T as ft } from "../chunks/BdAKL3gn.js";
import { C as gt } from "../chunks/DwsgkunL.js";
import { O as ht } from "../chunks/BBiSUZQU.js";
var bt = A('<div class="desc svelte-6fcb3i"><h4 class="svelte-6fcb3i">ID token claim mappings</h4> <p class="svelte-6fcb3i"> </p> <p class="svelte-6fcb3i"><!></p> <p class="svelte-6fcb3i"><!></p> <p class="svelte-6fcb3i"><!></p> <p class="svelte-6fcb3i"><!></p> <p class="svelte-6fcb3i"><!></p></div>');
function Pt(z, a) {
  ne(a, true);
  let t = pe();
  var f = bt(), h = o(u(f), 2), b = u(h, true);
  c(h);
  var p = o(h, 2), s = u(p);
  he(s, () => t.providers.config.jsonPath.p2), c(p);
  var I = o(p, 2), x = u(I);
  he(x, () => t.providers.config.jsonPath.p3), c(I);
  var g = o(I, 2), r = u(g);
  he(r, () => t.providers.config.jsonPath.p4), c(g);
  var l = o(g, 2), _ = u(l);
  he(_, () => t.providers.config.jsonPath.p5), c(l);
  var y = o(l, 2), d = u(y);
  he(d, () => t.providers.config.jsonPath.p6), c(y), c(f), T(() => E(b, t.providers.config.jsonPath.p1)), m(z, f), se();
}
var kt = A('<img alt="No Logo Available">');
function Ct(z, a) {
  ne(a, true);
  var t = kt();
  we(t, "width", 20), we(t, "height", 20), T((f) => we(t, "src", f), [() => `/auth/v1/providers/${a.providerId}/img?${Le()}}`]), m(z, t), se();
}
var wt = A('<div class="checkbox"><!></div>'), It = A('<div class="checkbox"><!></div> <!>', 1);
function Te(z, a) {
  ne(a, true);
  let t = D(a, "dangerAllowInsecure", 15), f = D(a, "showRootPem", 15), h = D(a, "rootPemCert", 15), b = pe();
  var p = It(), s = Q(p), I = u(s);
  me(I, { get ariaLabel() {
    return b.providers.config.custRootCa;
  }, get checked() {
    return f();
  }, set checked(l) {
    f(l);
  }, children: (l, _) => {
    Y();
    var y = O();
    T(() => E(y, b.providers.config.custRootCa)), m(l, y);
  }, $$slots: { default: true } }), c(s);
  var x = o(s, 2);
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
      var _ = wt(), y = u(_);
      me(y, { get ariaLabel() {
        return b.providers.config.allowInsecureTls;
      }, get checked() {
        return t();
      }, set checked(d) {
        t(d);
      }, children: (d, w) => {
        Y();
        var P = O();
        T(() => E(P, b.providers.config.allowInsecureTls)), m(d, P);
      }, $$slots: { default: true } }), c(_), m(l, _);
    };
    U(x, (l) => {
      f() ? l(g) : l(r, false);
    });
  }
  m(z, p), se();
}
var xt = A("<!> <!> <!> <!>", 1);
function We(z, a) {
  ne(a, true);
  let t = D(a, "issuer", 15), f = D(a, "authorizationEndpoint", 15), h = D(a, "tokenEndpoint", 15), b = D(a, "userinfoEndpoint", 15);
  var p = xt(), s = Q(p);
  H(s, { autocomplete: "off", label: "Issuer URL", placeholder: "Issuer URL", required: true, pattern: ie, get width() {
    return a.inputWidth;
  }, get disabled() {
    return a.disabled;
  }, get value() {
    return t();
  }, set value(r) {
    t(r);
  } });
  var I = o(s, 2);
  H(I, { typ: "url", autocomplete: "off", label: "Authorization Endpoint", placeholder: "Authorization Endpoint", required: true, pattern: ie, get width() {
    return a.inputWidth;
  }, get disabled() {
    return a.disabled;
  }, get value() {
    return f();
  }, set value(r) {
    f(r);
  } });
  var x = o(I, 2);
  H(x, { typ: "url", autocomplete: "off", label: "Token Endpoint", placeholder: "Token Endpoint", required: true, pattern: ie, get width() {
    return a.inputWidth;
  }, get disabled() {
    return a.disabled;
  }, get value() {
    return h();
  }, set value(r) {
    h(r);
  } });
  var g = o(x, 2);
  H(g, { typ: "url", autocomplete: "off", label: "Userinfo Endpoint", placeholder: "Userinfo Endpoint", required: true, pattern: ie, get width() {
    return a.inputWidth;
  }, get disabled() {
    return a.disabled;
  }, get value() {
    return b();
  }, set value(r) {
    b(r);
  } }), m(z, p), se();
}
var yt = A('<div class="err"> </div>'), Et = A('<p class="desc svelte-1u8yru7"> </p> <!> <p class="desc svelte-1u8yru7"> </p> <!> <p class="desc svelte-1u8yru7"> </p> <!> <p class="desc svelte-1u8yru7"> </p> <!> <p><!></p> <div class="checkbox"><!></div> <div class="checkbox"><!></div> <!> <!> <p class="desc svelte-1u8yru7"> </p> <!> <!> <p class="desc svelte-1u8yru7"> </p> <!> <!>', 1);
function je(z, a) {
  ne(a, true);
  let t = D(a, "scope", 15), f = D(a, "name", 15), h = D(a, "clientId", 15), b = D(a, "clientSecret", 15), p = D(a, "clientSecretBasic", 15), s = D(a, "clientSecretPost", 15), I = D(a, "adminClaimPath", 15), x = D(a, "adminClaimValue", 15), g = D(a, "mfaClaimPath", 15), r = D(a, "mfaClaimValue", 15), l = pe();
  var _ = Et(), y = Q(_), d = u(y, true);
  c(y);
  var w = o(y, 2);
  H(w, { autocomplete: "off", label: "Scope", placeholder: "openid profile email", required: true, pattern: vt, get width() {
    return a.inputWidth;
  }, get value() {
    return t();
  }, set value(S) {
    t(S);
  } });
  var P = o(w, 2), L = u(P, true);
  c(P);
  var V = o(P, 2);
  H(V, { autocomplete: "off", get label() {
    return l.providers.config.clientName;
  }, get placeholder() {
    return l.providers.config.clientName;
  }, required: true, pattern: mt, get width() {
    return a.inputWidth;
  }, get value() {
    return f();
  }, set value(S) {
    f(S);
  } });
  var W = o(V, 2), K = u(W, true);
  c(W);
  var R = o(W, 2);
  H(R, { autocomplete: "off", label: "Client ID", placeholder: "Client ID", required: true, pattern: ie, get width() {
    return a.inputWidth;
  }, get value() {
    return h();
  }, set value(S) {
    h(S);
  } });
  var G = o(R, 2), ee = u(G, true);
  c(G);
  var J = o(G, 2);
  const le = ae(() => !a.usePKCE);
  ut(J, { autocomplete: "off", label: "Client Secret", placeholder: "Client Secret", maxLength: 256, get errMsg() {
    return l.providers.config.errConfidential;
  }, get required() {
    return e(le);
  }, get width() {
    return a.inputWidth;
  }, get value() {
    return b();
  }, set value(S) {
    b(S);
  } });
  var X = o(J, 2), F = u(X);
  he(F, () => l.providers.config.descAuthMethod), c(X);
  var M = o(X, 2), j = u(M);
  me(j, { ariaLabel: "client_secret_basic", get checked() {
    return p();
  }, set checked(S) {
    p(S);
  }, children: (S, _e) => {
    Y();
    var C = O("client_secret_basic");
    m(S, C);
  }, $$slots: { default: true } }), c(M);
  var v = o(M, 2), de = u(v);
  me(de, { ariaLabel: "client_secret_post", get checked() {
    return s();
  }, set checked(S) {
    s(S);
  }, children: (S, _e) => {
    Y();
    var C = O("client_secret_post");
    m(S, C);
  }, $$slots: { default: true } }), c(v);
  var te = o(v, 2);
  {
    var ce = (S) => {
      var _e = yt(), C = u(_e, true);
      c(_e), T(() => E(C, l.providers.config.errNoAuthMethod)), st(3, _e, () => lt, () => ({ duration: 150 })), m(S, _e);
    };
    U(te, (S) => {
      !a.usePKCE && !p() && !s() && S(ce);
    });
  }
  var re = o(te, 2);
  Pt(re, {});
  var oe = o(re, 2), i = u(oe, true);
  c(oe);
  var $ = o(oe, 2);
  H($, { autocomplete: "off", get label() {
    return l.providers.config.pathAdminClaim;
  }, placeholder: "$.roles.*", pattern: ie, get width() {
    return a.inputWidth;
  }, get value() {
    return I();
  }, set value(S) {
    I(S);
  } });
  var q = o($, 2);
  H(q, { autocomplete: "off", get label() {
    return l.providers.config.valueAdminClaim;
  }, placeholder: "rauthy_admin", pattern: ie, get width() {
    return a.inputWidth;
  }, get value() {
    return x();
  }, set value(S) {
    x(S);
  } });
  var ge = o(q, 2), Ie = u(ge, true);
  c(ge);
  var be = o(ge, 2);
  H(be, { autocomplete: "off", get label() {
    return l.providers.config.pathMfaClaim;
  }, placeholder: "$.amr.*", pattern: ie, get width() {
    return a.inputWidth;
  }, get value() {
    return g();
  }, set value(S) {
    g(S);
  } });
  var xe = o(be, 2);
  H(xe, { autocomplete: "off", get label() {
    return l.providers.config.valueMfaClaim;
  }, placeholder: "mfa", pattern: ie, get width() {
    return a.inputWidth;
  }, get value() {
    return r();
  }, set value(S) {
    r(S);
  } }), T(() => {
    E(d, l.providers.config.descScope), E(L, l.providers.config.descClientName), E(K, l.providers.config.descClientId), E(ee, l.providers.config.descClientSecret), E(i, l.providers.config.mapUser), E(Ie, l.providers.config.mapMfa);
  }), m(z, _), se();
}
var St = A('<div class="svelte-jts2zb"><!></div>'), At = A('<span class="err"> </span>'), Lt = A('<!> <div></div> <div class="checkbox svelte-jts2zb"><!></div> <!> <!> <div class="checkbox svelte-jts2zb"><!></div> <!> <div class="logo svelte-jts2zb"><!> <!></div> <div class="flex gap-05"><!> <!> <!></div>', 1), Tt = A('<div class="container svelte-jts2zb"><!></div>');
function zt(z, a) {
  ne(a, true);
  let t = D(a, "provider", 15), f = Re(), h = pe();
  const b = "min(calc(100dvw - .5rem), 30rem)";
  let p = false, s = N(""), I = N(false), x = N(!!t().root_pem), g = N(fe(Le()));
  ke(() => {
    t().id && (t(t().client_secret = t().client_secret || "", true), t(t().admin_claim_path = t().admin_claim_path || "", true), t(t().admin_claim_value = t().admin_claim_value || "", true), t(t().mfa_claim_path = t().mfa_claim_path || "", true), t(t().mfa_claim_value = t().mfa_claim_value || "", true), t(t().root_pem = t().root_pem || "", true));
  }), ke(() => {
    t().scope && t(t().scope = t().scope.replaceAll("+", " "), true);
  });
  async function r(d, w) {
    if (n(s, ""), t().client_secret && !(t().client_secret_basic || t().client_secret_post)) {
      n(s, h.providers.config.errNoAuthMethod, true);
      return;
    }
    if (!t().use_pkce && !t().client_secret) {
      n(s, h.providers.config.errConfidential, true);
      return;
    }
    let P = { name: t().name, typ: t().typ, enabled: t().enabled, issuer: t().issuer, authorization_endpoint: t().authorization_endpoint, token_endpoint: t().token_endpoint, userinfo_endpoint: t().userinfo_endpoint, danger_allow_insecure: e(x) && t().root_pem ? false : t().danger_allow_insecure, use_pkce: t().use_pkce, client_secret_basic: t().client_secret_basic, client_secret_post: t().client_secret_post, client_id: t().client_id, client_secret: t().client_secret || void 0, scope: t().scope.trim(), root_pem: e(x) && t().root_pem ? t().root_pem.trim() : void 0, admin_claim_path: t().admin_claim_path || void 0, admin_claim_value: t().admin_claim_value || void 0, mfa_claim_path: t().mfa_claim_path || void 0, mfa_claim_value: t().mfa_claim_value || void 0 }, L = await Xe(d.action, P);
    L.error ? n(s, L.error.message, true) : (n(I, true), a.onSave(), setTimeout(() => {
      n(I, false);
    }, 3e3));
  }
  var l = Tt(), _ = u(l);
  const y = ae(() => `/auth/v1/providers/${t().id}`);
  $e(_, { get action() {
    return e(y);
  }, onSubmit: r, children: (d, w) => {
    var P = Lt(), L = Q(P);
    Ve(L, { label: "ID", mono: true, children: (i, $) => {
      Y();
      var q = O();
      T(() => E(q, t().id)), m(i, q);
    }, $$slots: { default: true } });
    var V = o(L, 2);
    ze(V, "", {}, { height: ".15rem" });
    var W = o(V, 2), K = u(W);
    me(K, { get ariaLabel() {
      return h.common.enabled;
    }, get checked() {
      return t().enabled;
    }, set checked(i) {
      t(t().enabled = i, true);
    }, children: (i, $) => {
      Y();
      var q = O();
      T(() => E(q, h.common.enabled)), m(i, q);
    }, $$slots: { default: true } }), c(W);
    var R = o(W, 2);
    Te(R, { get dangerAllowInsecure() {
      return t().danger_allow_insecure;
    }, set dangerAllowInsecure(i) {
      t(t().danger_allow_insecure = i, true);
    }, get showRootPem() {
      return e(x);
    }, set showRootPem(i) {
      n(x, i, true);
    }, get rootPemCert() {
      return t().root_pem;
    }, set rootPemCert(i) {
      t(t().root_pem = i, true);
    } });
    var G = o(R, 2);
    We(G, { inputWidth: b, get issuer() {
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
    var ee = o(G, 2), J = u(ee);
    me(J, { ariaLabel: "PKCE", get checked() {
      return t().use_pkce;
    }, set checked(i) {
      t(t().use_pkce = i, true);
    }, children: (i, $) => {
      Y();
      var q = O("PKCE");
      m(i, q);
    }, $$slots: { default: true } }), c(ee);
    var le = o(ee, 2);
    je(le, { get usePKCE() {
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
    var X = o(le, 2), F = u(X);
    nt(F, () => e(g), (i) => {
      var $ = St(), q = u($);
      Ct(q, { get providerId() {
        return t().id;
      } }), c($), m(i, $);
    });
    var M = o(F, 2);
    const j = ae(() => `/auth/v1/providers/${t().id}/img`);
    ct(M, { method: "PUT", get url() {
      return e(j);
    }, fileName: "logo", onSuccess: () => n(g, Le(), true) }), c(X);
    var v = o(X, 2), de = u(v);
    Ce(de, { type: "submit", get isLoading() {
      return p;
    }, children: (i, $) => {
      Y();
      var q = O();
      T(() => E(q, f.common.save)), m(i, q);
    }, $$slots: { default: true } });
    var te = o(de, 2);
    {
      var ce = (i) => {
        Ue(i, {});
      };
      U(te, (i) => {
        e(I) && i(ce);
      });
    }
    var re = o(te, 2);
    {
      var oe = (i) => {
        var $ = At(), q = u($, true);
        c($), T(() => E(q, e(s))), m(i, $);
      };
      U(re, (i) => {
        e(s) && i(oe);
      });
    }
    c(v), m(d, P);
  }, $$slots: { default: true } }), c(l), m(z, l), se();
}
var Rt = A('<div class="user svelte-1vu6qeb"> <span class="muted font-mono svelte-1vu6qeb"> </span></div>'), Nt = A('<div class="forceDelete svelte-1vu6qeb"> </div>'), Mt = A('<p class="err"><b> </b></p> <p> </p> <!> <p> </p> <!> <!>', 1), qt = A("<p> </p> <!>", 1), Dt = A('<div class="err"> </div>'), Ut = A('<div class="container svelte-1vu6qeb"><!> <!></div>');
function $t(z, a) {
  ne(a, true);
  let t = Re(), f = pe(), h = N(true), b = N(""), p = N(false), s = N(fe([]));
  qe(async () => {
    let d = await Ze(`/auth/v1/providers/${a.provider.id}/delete_safe`);
    d.status === 406 && d.error && n(s, d.error, true), n(h, false);
  });
  async function I() {
    n(b, ""), n(h, true);
    let d = await et(`/auth/v1/providers/${a.provider.id}`);
    d.error ? n(b, d.error.message, true) : a.onSave(), n(h, false);
  }
  var x = Ut(), g = u(x);
  {
    var r = (d) => {
      var w = Mt(), P = Q(w), L = u(P), V = u(L, true);
      c(L), c(P);
      var W = o(P, 2), K = u(W, true);
      c(W);
      var R = o(W, 2);
      pt(R, { summary: (j) => {
        Y();
        var v = O();
        T(() => E(v, f.providers.delete.linkedUsers)), m(j, v);
      }, details: (j) => {
        var v = He(), de = Q(v);
        De(de, 17, () => e(s), (te) => te.id, (te, ce) => {
          var re = Rt(), oe = u(re), i = o(oe), $ = u(i);
          c(i), c(re), T(() => {
            E(oe, `${e(ce).email ?? ""} `), E($, `/ ${e(ce).id ?? ""}`);
          }), m(te, re);
        }), m(j, v);
      }, $$slots: { summary: true, details: true } });
      var G = o(R, 2), ee = u(G, true);
      c(G);
      var J = o(G, 2);
      me(J, { get ariaLabel() {
        return f.providers.delete.forceDelete;
      }, get checked() {
        return e(p);
      }, set checked(F) {
        n(p, F, true);
      }, children: (F, M) => {
        var j = Nt(), v = u(j, true);
        c(j), T(() => E(v, f.providers.delete.forceDelete)), m(F, j);
      }, $$slots: { default: true } });
      var le = o(J, 2);
      {
        var X = (F) => {
          Ce(F, { level: -1, onclick: I, get isLoading() {
            return e(h);
          }, children: (M, j) => {
            Y();
            var v = O();
            T(() => E(v, t.common.delete)), m(M, v);
          }, $$slots: { default: true } });
        };
        U(le, (F) => {
          e(p) && F(X);
        });
      }
      T(() => {
        E(V, f.providers.delete.isInUse1), E(K, f.providers.delete.isInUse2), E(ee, f.providers.delete.areYouSure);
      }), m(d, w);
    }, l = (d) => {
      var w = qt(), P = Q(w), L = u(P, true);
      c(P);
      var V = o(P, 2);
      Ce(V, { level: -1, onclick: I, get isLoading() {
        return e(h);
      }, children: (W, K) => {
        Y();
        var R = O();
        T(() => E(R, t.common.delete)), m(W, R);
      }, $$slots: { default: true } }), T(() => E(L, f.providers.delete.areYouSure)), m(d, w);
    };
    U(g, (d) => {
      e(s).length > 0 ? d(r) : d(l, false);
    });
  }
  var _ = o(g, 2);
  {
    var y = (d) => {
      var w = Dt(), P = u(w, true);
      c(w), T(() => E(P, e(b))), m(d, w);
    };
    U(_, (d) => {
      e(b) && d(y);
    });
  }
  c(x), m(z, x), se();
}
var Vt = A('<div class="flex"><!></div> <!>', 1);
function Wt(z, a) {
  ne(a, true);
  let t = D(a, "provider", 15), f = D(a, "onSave", 15), h = pe();
  const b = [h.tabs.config, h.tabs.delete];
  let p = N(fe(b[0]));
  var s = Vt(), I = Q(s), x = u(I);
  ft(x, { tabs: b, get selected() {
    return e(p);
  }, set selected(_) {
    n(p, _, true);
  } }), c(I);
  var g = o(I, 2);
  {
    var r = (_) => {
      zt(_, { get provider() {
        return t();
      }, set provider(y) {
        t(y);
      }, get onSave() {
        return f();
      }, set onSave(y) {
        f(y);
      } });
    }, l = (_, y) => {
      {
        var d = (w) => {
          $t(w, { get provider() {
            return t();
          }, get onSave() {
            return f();
          }, set onSave(P) {
            f(P);
          } });
        };
        U(_, (w) => {
          e(p) === h.tabs.delete && w(d);
        }, y);
      }
    };
    U(g, (_) => {
      e(p) === h.tabs.config ? _(r) : _(l, false);
    });
  }
  m(z, s), se();
}
var jt = A('<!> <div class="checkbox svelte-qeky4o"><!></div> <!>', 1), Bt = A('<div class="err"> </div>'), Kt = A('<!> <!> <div class="flex gap-05"><!> <!> <!> <!></div>', 1), Gt = A('<div class="container svelte-qeky4o"><!> <div></div> <!></div>');
function Ft(z, a) {
  ne(a, true);
  let t = Re(), f = pe();
  const h = "min(calc(100dvw - 1.75rem), 30rem)";
  let b = N(false), p = N(""), s = N(false), I = N(false), x = N(false), g = N(fe({ issuer: "", metadata_url: "", danger_allow_insecure: false, root_pem: "" })), r = N(fe({ enabled: true, typ: "oidc", issuer: "", danger_allow_insecure: false, authorization_endpoint: "", token_endpoint: "", token_auth_method_basic: false, userinfo_endpoint: "", use_pkce: true, client_secret_basic: true, client_secret_post: false, name: "", client_id: "", client_secret: "", scope: "", root_pem: "", admin_claim_path: "", admin_claim_value: "", mfa_claim_path: "", mfa_claim_value: "" })), l = ["OIDC", "Auto", "Custom", "Github", "Google"], _ = N(fe(l[0])), y = ae(() => e(_) === "Auto"), d = ae(() => e(_) === "Custom"), w = ae(() => e(_) === "OIDC"), P = ae(() => !e(y) && !e(d) && !e(w)), L = ae(() => !(e(P) || e(d) || e(s)));
  const V = "/auth/v1/providers/lookup";
  let W = ae(() => e(L) ? V : "/auth/v1/providers/create");
  ke(() => {
    if (e(_)) switch (n(s, false), n(g, { issuer: "", metadata_url: "", danger_allow_insecure: false, root_pem: "" }, true), e(_)) {
      case "Github":
        n(r, { enabled: true, issuer: "github.com", typ: "github", danger_allow_insecure: false, authorization_endpoint: "https://github.com/login/oauth/authorize", token_endpoint: "https://github.com/login/oauth/access_token", client_secret_basic: true, client_secret_post: true, userinfo_endpoint: "https://api.github.com/user", use_pkce: false, name: "Github", client_id: "", client_secret: "", scope: "user:email", root_pem: "", admin_claim_path: "", admin_claim_value: "", mfa_claim_path: "$.two_factor_authentication", mfa_claim_value: "true" }, true), n(s, true);
        break;
      case "Google":
        n(g, { issuer: "accounts.google.com", metadata_url: "", danger_allow_insecure: false, root_pem: "" }, true), Qe(() => {
          G(V);
        });
        break;
      default:
        n(r, { enabled: true, issuer: "", typ: "oidc", danger_allow_insecure: false, authorization_endpoint: "", token_endpoint: "", client_secret_basic: true, client_secret_post: false, userinfo_endpoint: "", use_pkce: true, name: "", client_id: "", client_secret: "", scope: "", root_pem: "", admin_claim_path: "", admin_claim_value: "", mfa_claim_path: "", mfa_claim_value: "" }, true);
    }
  }), ke(() => {
    e(I) && setTimeout(() => {
      a.onSave(), ee();
    }, 1500);
  });
  async function K(M, j) {
    e(L) ? await G(M.action) : await R(M.action);
  }
  async function R(M) {
    if (e(r).client_secret && !(e(r).client_secret_basic || e(r).client_secret_post)) {
      n(p, f.providers.config.errNoAuthMethod, true);
      return;
    }
    if (!e(r).use_pkce && !e(r).client_secret) {
      n(p, f.providers.config.errConfidential, true);
      return;
    }
    n(p, ""), n(b, true);
    let j = { name: e(r).name, typ: e(y) ? "custom" : e(_).toLowerCase(), enabled: e(r).enabled, issuer: e(r).issuer, authorization_endpoint: e(r).authorization_endpoint, token_endpoint: e(r).token_endpoint, userinfo_endpoint: e(r).userinfo_endpoint, danger_allow_insecure: e(x) && e(r).root_pem ? false : e(r).danger_allow_insecure, use_pkce: e(r).use_pkce, client_secret_basic: e(r).client_secret_basic, client_secret_post: e(r).client_secret_post, client_id: e(r).client_id, client_secret: e(r).client_secret, scope: e(r).scope.trim(), root_pem: e(x) && e(r).root_pem ? e(r).root_pem.trim() : void 0, admin_claim_path: e(r).admin_claim_path || void 0, admin_claim_value: e(r).admin_claim_value || void 0, mfa_claim_path: e(r).mfa_claim_path || void 0, mfa_claim_value: e(r).mfa_claim_value || void 0 }, v = await Ae(M, j);
    v.error ? v.error.message.includes("InvalidCertificate") ? n(p, "Insecure connection not allowed") : n(p, v.error.message, true) : n(I, true), n(b, false);
  }
  async function G(M) {
    if (!e(g).issuer && !e(g).metadata_url) {
      n(p, "Provide at least one of Issuer / Metadata URL");
      return;
    }
    n(p, ""), n(b, true);
    let j = { issuer: e(g).issuer || void 0, metadata_url: e(g).metadata_url || void 0, danger_allow_insecure: e(g).danger_allow_insecure, root_pem: e(g).root_pem || void 0 }, v = await Ae(M, j);
    v.body ? (e(r).issuer = v.body.issuer, e(r).authorization_endpoint = v.body.authorization_endpoint, e(r).danger_allow_insecure = v.body.danger_allow_insecure, e(r).token_endpoint = v.body.token_endpoint, e(r).userinfo_endpoint = v.body.userinfo_endpoint, e(r).client_secret_basic = v.body.client_secret_basic, e(r).client_secret_post = v.body.client_secret_post, e(r).use_pkce = v.body.use_pkce, e(r).client_secret_basic = v.body.client_secret_basic, e(r).client_secret_post = !v.body.client_secret_basic && v.body.client_secret_post, e(r).scope = v.body.scope, e(r).root_pem = v.body.root_pem, n(s, true)) : v.error && (v.error.message.includes("InvalidCertificate") ? n(p, "Insecure connection not allowed") : n(p, v.error.message, true)), n(b, false);
  }
  function ee() {
    n(g, { issuer: "", metadata_url: "", danger_allow_insecure: false, root_pem: "" }, true), n(r, { enabled: true, issuer: "", typ: "oidc", danger_allow_insecure: false, authorization_endpoint: "", token_endpoint: "", userinfo_endpoint: "", use_pkce: true, name: "", client_id: "", client_secret_basic: true, client_secret_post: false, scope: "", root_pem: "", admin_claim_path: "", admin_claim_value: "", mfa_claim_path: "", mfa_claim_value: "" }, true), n(I, false), n(s, false), n(x, false);
  }
  var J = Gt(), le = u(J);
  ht(le, { ariaLabel: "Select Mode", options: l, get value() {
    return e(_);
  }, set value(M) {
    n(_, M, true);
  } });
  var X = o(le, 2);
  ze(X, "", {}, { height: ".5rem" });
  var F = o(X, 2);
  $e(F, { get action() {
    return e(W);
  }, onSubmit: K, children: (M, j) => {
    var v = Kt(), de = Q(v);
    {
      var te = (C) => {
        Te(C, { get dangerAllowInsecure() {
          return e(r).danger_allow_insecure;
        }, set dangerAllowInsecure(B) {
          e(r).danger_allow_insecure = B;
        }, get showRootPem() {
          return e(x);
        }, set showRootPem(B) {
          n(x, B, true);
        }, get rootPemCert() {
          return e(r).root_pem;
        }, set rootPemCert(B) {
          e(r).root_pem = B;
        } });
      }, ce = (C, B) => {
        {
          var ue = (ve) => {
            Te(ve, { get dangerAllowInsecure() {
              return e(g).danger_allow_insecure;
            }, set dangerAllowInsecure(Z) {
              e(g).danger_allow_insecure = Z;
            }, get showRootPem() {
              return e(x);
            }, set showRootPem(Z) {
              n(x, Z, true);
            }, get rootPemCert() {
              return e(g).root_pem;
            }, set rootPemCert(Z) {
              e(g).root_pem = Z;
            } });
          };
          U(C, (ve) => {
            e(s) || ve(ue);
          }, B);
        }
      };
      U(de, (C) => {
        e(d) ? C(te) : C(ce, false);
      });
    }
    var re = o(de, 2);
    {
      var oe = (C) => {
        H(C, { name: "issuer", label: "Issuer URL", placeholder: "Issuer URL", width: h, required: true, pattern: ie, get value() {
          return e(g).issuer;
        }, set value(B) {
          e(g).issuer = B;
        } });
      }, i = (C, B) => {
        {
          var ue = (Z) => {
            H(Z, { typ: "url", name: "metadata", label: "Metadata URL", placeholder: ".../.well-known/openid-configuration", width: h, required: true, pattern: ie, get value() {
              return e(g).metadata_url;
            }, set value(ye) {
              e(g).metadata_url = ye;
            } });
          }, ve = (Z, ye) => {
            {
              var Be = (Ee) => {
                var Ne = jt(), Me = Q(Ne);
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
                var Se = o(Me, 2), Ke = u(Se);
                {
                  var Ge = (k) => {
                    Ve(k, { label: "PKCE", children: (Pe, Ye) => {
                      gt(Pe, { get checked() {
                        return e(r).use_pkce;
                      } });
                    }, $$slots: { default: true } });
                  }, Fe = (k) => {
                    me(k, { ariaLabel: "PKCE", get checked() {
                      return e(r).use_pkce;
                    }, set checked(Pe) {
                      e(r).use_pkce = Pe;
                    }, children: (Pe, Ye) => {
                      Y();
                      var Je = O("PKCE");
                      m(Pe, Je);
                    }, $$slots: { default: true } });
                  };
                  U(Ke, (k) => {
                    e(s) ? k(Ge) : k(Fe, false);
                  });
                }
                c(Se);
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
              U(Z, (Ee) => {
                (e(P) || e(d) || e(s)) && Ee(Be);
              }, ye);
            }
          };
          U(C, (Z) => {
            e(y) && !e(s) ? Z(ue) : Z(ve, false);
          }, B);
        }
      };
      U(re, (C) => {
        e(w) && !e(s) ? C(oe) : C(i, false);
      });
    }
    var $ = o(re, 2), q = u($);
    Ce(q, { type: "submit", get isLoading() {
      return e(b);
    }, children: (C, B) => {
      Y();
      var ue = O();
      T(() => E(ue, e(L) ? f.providers.config.lookup : t.common.save)), m(C, ue);
    }, $$slots: { default: true } });
    var ge = o(q, 2);
    {
      var Ie = (C) => {
        Ce(C, { level: 3, onclick: ee, get isLoading() {
          return e(b);
        }, children: (B, ue) => {
          Y();
          var ve = O();
          T(() => E(ve, f.common.reset)), m(B, ve);
        }, $$slots: { default: true } });
      };
      U(ge, (C) => {
        e(L) || C(Ie);
      });
    }
    var be = o(ge, 2);
    {
      var xe = (C) => {
        Ue(C, {});
      };
      U(be, (C) => {
        e(I) && C(xe);
      });
    }
    var S = o(be, 2);
    {
      var _e = (C) => {
        var B = Bt(), ue = u(B, true);
        c(B), T(() => E(ue, e(p))), m(C, B);
      };
      U(S, (C) => {
        e(p) && C(_e);
      });
    }
    c($), m(M, v);
  }, $$slots: { default: true } }), c(J), m(z, J), se();
}
var Ot = A("<div></div> <!>", 1), Yt = A('<div class="err"> </div>'), Jt = A('<div id="federation"><!></div> <!>', 1), Ht = A("<!> <!>", 1);
function Qt(z, a) {
  ne(a, true);
  let t = pe(), f = N(void 0), h = at("pid"), b = N(""), p = N(fe([])), s = N(void 0);
  qe(() => {
    I();
  }), ke(() => {
    n(s, e(p).find((_) => _.id === h.get()), true);
  });
  async function I() {
    var _a;
    let _ = await Ae("/auth/v1/providers");
    _.body ? n(p, _.body, true) : n(b, ((_a = _.error) == null ? void 0 : _a.message) || "Error", true);
  }
  function x() {
    var _a;
    h.set(void 0), (_a = e(f)) == null ? void 0 : _a(), I();
  }
  var g = Ht(), r = Q(g);
  rt(r, { width: "11rem", buttonTilesAriaControls: "federation", paddingTop: "6.65rem", buttonTiles: (y) => {
    var d = Ot(), w = Q(d);
    ze(w, "", {}, { height: ".5rem" });
    var P = o(w, 2);
    De(P, 17, () => e(p), (L) => L.id, (L, V, W, K) => {
      const R = ae(() => h.get() === e(V).id);
      it(L, { onclick: () => h.set(e(V).id), get selected() {
        return e(R);
      }, children: (G, ee) => {
        Y();
        var J = O();
        T(() => E(J, e(V).name)), m(G, J);
      } });
    }), m(y, d);
  }, children: (y, d) => {
    const w = ae(() => e(p).length === 0 ? 1 : 2);
    ot(y, { get level() {
      return e(w);
    }, get closeModal() {
      return e(f);
    }, set closeModal(P) {
      n(f, P, true);
    }, children: (P, L) => {
      Ft(P, { onSave: x });
    }, $$slots: { default: true } });
  }, $$slots: { buttonTiles: true, default: true } });
  var l = o(r, 2);
  tt(l, { children: (_, y) => {
    var d = Jt(), w = Q(d), P = u(w);
    {
      var L = (K) => {
        Wt(K, { onSave: I, get provider() {
          return e(s);
        }, set provider(R) {
          n(s, R, true);
        } });
      };
      U(P, (K) => {
        e(s) && K(L);
      });
    }
    c(w);
    var V = o(w, 2);
    {
      var W = (K) => {
        var R = Yt(), G = u(R, true);
        c(R), T(() => E(G, e(b))), m(K, R);
      };
      U(V, (K) => {
        e(b) && K(W);
      });
    }
    T(() => we(w, "aria-label", t.common.details)), m(_, d);
  } }), m(z, g), se();
}
function Lr(z) {
  Qt(z, {});
}
export {
  Lr as component
};
