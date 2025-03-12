import { t as x, a as d, e as D, d as Gt } from "../chunks/DKC5GJ29.js";
import { p as ke, s as a, c as s, r as n, t as C, a as we, k as R, aa as Ge, f as ie, a8 as U, l as u, j as e, a9 as re, a6 as xt, P as Bt } from "../chunks/BveBAmlr.js";
import { s as v } from "../chunks/CYCba2oX.js";
import { i as L } from "../chunks/D-uYoVwt.js";
import { e as It } from "../chunks/BLE4FKaJ.js";
import { s as He, a as pt } from "../chunks/Dql74IOz.js";
import { p as h } from "../chunks/VbPNpVtZ.js";
import { d as jt, f as Kt, c as Ot, b as lt } from "../chunks/BO1A6s0c.js";
import { C as Vt } from "../chunks/CqiL-eHy.js";
import { N as Wt } from "../chunks/DLHOTAgm.js";
import { u as Yt } from "../chunks/BdFU-gNS.js";
import { N as Jt } from "../chunks/pmj7Ha_a.js";
import { B as Ht } from "../chunks/B1rQzxl_.js";
import { p as dt } from "../chunks/Db0ChEdV.js";
import { k as Qt } from "../chunks/BPEbum0P.js";
import { h as $e } from "../chunks/CJj8kriY.js";
import { B as Be, t as Qe, s as Xe } from "../chunks/DlLEcmNg.js";
import { I as y } from "../chunks/CCQyB9gY.js";
import { u as Le } from "../chunks/DtT3Jahq.js";
import { g as ct } from "../chunks/B21bTIl7.js";
import { I as Pt } from "../chunks/Cxysd5TC.js";
import { I as ut } from "../chunks/Cv33Ea-7.js";
import { I as Et } from "../chunks/CUO8Plwh.js";
import { F as yt } from "../chunks/BguaNybM.js";
import { u as mt } from "../chunks/8R5My_LO.js";
import { L as vt } from "../chunks/FI26onRO.js";
import { I as Z } from "../chunks/4YTb0aaH.js";
import { a as q, h as Tt, g as At, i as _t } from "../chunks/BRCxk8by.js";
import { I as Xt } from "../chunks/BpjUF1Hv.js";
import { E as Zt } from "../chunks/VvDlSTic.js";
import { T as er } from "../chunks/BGiCnx1L.js";
import { C as Ct } from "../chunks/5J717Kcr.js";
import { O as tr } from "../chunks/DdLeAElE.js";
var rr = x('<div class="desc svelte-6fcb3i"><h4 class="svelte-6fcb3i">ID token claim mappings</h4> <p class="svelte-6fcb3i"> </p> <p class="svelte-6fcb3i"><!></p> <p class="svelte-6fcb3i"><!></p> <p class="svelte-6fcb3i"><!></p> <p class="svelte-6fcb3i"><!></p> <p class="svelte-6fcb3i"><!></p></div>');
function Lt(V, S) {
  ke(S, true);
  let t = Le();
  var l = rr(), i = a(s(l), 2), f = s(i, true);
  n(i);
  var k = a(i, 2), p = s(k);
  $e(p, () => t.providers.config.jsonPath.p2), n(k);
  var T = a(k, 2), A = s(T);
  $e(A, () => t.providers.config.jsonPath.p3), n(T);
  var g = a(T, 2), r = s(g);
  $e(r, () => t.providers.config.jsonPath.p4), n(g);
  var O = a(g, 2), b = s(O);
  $e(b, () => t.providers.config.jsonPath.p5), n(O);
  var M = a(O, 2), m = s(M);
  $e(m, () => t.providers.config.jsonPath.p6), n(M), n(l), C(() => v(f, t.providers.config.jsonPath.p1)), d(V, l), we();
}
var ar = x('<img alt="No Logo Available">');
function or(V, S) {
  ke(S, true);
  var t = ar();
  He(t, "width", 20), He(t, "height", 20), C((l) => He(t, "src", l), [() => `/auth/v1/providers/${S.providerId}/img?${ct()}}`]), d(V, t), we();
}
var ir = x("<div><!></div>"), sr = x('<div class="checkbox svelte-iec8od"><!></div>'), nr = x('<div class="err"> </div>'), lr = x('<div class="svelte-iec8od"><!></div>'), dr = x('<span class="err"> </span>'), cr = x('<!> <div></div> <div class="checkbox svelte-iec8od"><!></div> <div class="checkbox svelte-iec8od"><!></div> <!> <!> <!> <!> <!> <div class="checkbox svelte-iec8od"><!></div> <p class="desc svelte-iec8od"> </p> <!> <p class="desc svelte-iec8od"> </p> <!> <p class="desc svelte-iec8od"> </p> <!> <p class="desc svelte-iec8od"> </p> <!> <p><!></p> <div class="checkbox svelte-iec8od"><!></div> <div class="checkbox svelte-iec8od"><!></div> <!> <!> <p class="desc svelte-iec8od"> </p> <!> <!> <p class="desc svelte-iec8od"> </p> <!> <!> <div class="logo svelte-iec8od"><!> <!></div> <div class="flex gap-05"><!> <!> <!></div>', 1), ur = x('<div class="container svelte-iec8od"><!></div>');
function vr(V, S) {
  ke(S, true);
  let t = dt(S, "provider", 15), l = mt(), i = Le();
  const f = "min(calc(100dvw - .5rem), 30rem)";
  let k = false, p = R(""), T = R(false), A = R(!!t().root_pem), g = R(h(ct()));
  Ge(() => {
    t().id && (t(t().client_secret = t().client_secret || "", true), t(t().admin_claim_path = t().admin_claim_path || "", true), t(t().admin_claim_value = t().admin_claim_value || "", true), t(t().mfa_claim_path = t().mfa_claim_path || "", true), t(t().mfa_claim_value = t().mfa_claim_value || "", true), t(t().root_pem = t().root_pem || "", true));
  }), Ge(() => {
    t().scope && t(t().scope = t().scope.replaceAll("+", " "), true);
  });
  async function r(m, I) {
    if (u(p, ""), t().client_secret && !(t().client_secret_basic || t().client_secret_post)) {
      u(p, h(i.providers.config.errNoAuthMethod));
      return;
    }
    if (!t().use_pkce && !t().client_secret) {
      u(p, h(i.providers.config.errConfidential));
      return;
    }
    let P = { name: t().name, typ: t().typ, enabled: t().enabled, issuer: t().issuer, authorization_endpoint: t().authorization_endpoint, token_endpoint: t().token_endpoint, userinfo_endpoint: t().userinfo_endpoint, danger_allow_insecure: e(A) && t().root_pem ? false : t().danger_allow_insecure, use_pkce: t().use_pkce, client_secret_basic: t().client_secret_basic, client_secret_post: t().client_secret_post, client_id: t().client_id, client_secret: t().client_secret || void 0, scope: t().scope.trim(), root_pem: e(A) && t().root_pem ? t().root_pem.trim() : void 0, admin_claim_path: t().admin_claim_path || void 0, admin_claim_value: t().admin_claim_value || void 0, mfa_claim_path: t().mfa_claim_path || void 0, mfa_claim_value: t().mfa_claim_value || void 0 }, N = await jt(m.action, P);
    N.error ? u(p, h(N.error.message)) : (u(T, true), S.onSave(), setTimeout(() => {
      u(T, false);
    }, 3e3));
  }
  var O = ur(), b = s(O);
  const M = re(() => `/auth/v1/providers/${t().id}`);
  yt(b, { get action() {
    return e(M);
  }, onSubmit: r, children: (m, I) => {
    var P = cr(), N = ie(P);
    vt(N, { label: "ID", mono: true, children: (o, $) => {
      U();
      var w = D();
      C(() => v(w, t().id)), d(o, w);
    }, $$slots: { default: true } });
    var W = a(N, 2);
    pt(W, "", {}, { height: ".15rem" });
    var J = a(W, 2), Y = s(J);
    Z(Y, { get ariaLabel() {
      return i.common.enabled;
    }, get checked() {
      return t().enabled;
    }, set checked(o) {
      t(t().enabled = o, true);
    }, children: (o, $) => {
      U();
      var w = D();
      C(() => v(w, i.common.enabled)), d(o, w);
    }, $$slots: { default: true } }), n(J);
    var z = a(J, 2), ee = s(z);
    Z(ee, { get ariaLabel() {
      return i.providers.config.custRootCa;
    }, get checked() {
      return e(A);
    }, set checked(o) {
      u(A, h(o));
    }, children: (o, $) => {
      U();
      var w = D();
      C(() => v(w, i.providers.config.custRootCa)), d(o, w);
    }, $$slots: { default: true } }), n(z);
    var ue = a(z, 2);
    {
      var ae = (o) => {
        var $ = ir(), w = s($);
        ut(w, { rows: 15, name: "rootPem", get label() {
          return i.providers.config.rootPemCert;
        }, placeholder: `-----BEGIN CERTIFICATE-----
...
 -----END CERTIFICATE-----`, errMsg: "-----BEGIN CERTIFICATE----- ...", width: "min(40rem, calc(100dvw - .5rem))", fontMono: true, pattern: _t, get value() {
          return t().root_pem;
        }, set value(me) {
          t(t().root_pem = me, true);
        } }), n($), Qe(3, $, () => Xe, () => ({ duration: 150 })), d(o, $);
      }, Ce = (o) => {
        var $ = sr(), w = s($);
        Z(w, { get ariaLabel() {
          return i.providers.config.allowInsecureTls;
        }, get checked() {
          return t().danger_allow_insecure;
        }, set checked(me) {
          t(t().danger_allow_insecure = me, true);
        }, children: (me, ft) => {
          U();
          var Te = D();
          C(() => v(Te, i.providers.config.allowInsecureTls)), d(me, Te);
        }, $$slots: { default: true } }), n($), d(o, $);
      };
      L(ue, (o) => {
        e(A) ? o(ae) : o(Ce, false);
      });
    }
    var fe = a(ue, 2);
    y(fe, { autocomplete: "off", label: "Issuer URL", placeholder: "Issuer URL", required: true, pattern: q, width: f, get value() {
      return t().issuer;
    }, set value(o) {
      t(t().issuer = o, true);
    } });
    var Q = a(fe, 2);
    y(Q, { typ: "url", autocomplete: "off", label: "Authorization Endpoint", placeholder: "Authorization Endpoint", required: true, pattern: q, width: f, get value() {
      return t().authorization_endpoint;
    }, set value(o) {
      t(t().authorization_endpoint = o, true);
    } });
    var G = a(Q, 2);
    y(G, { typ: "url", autocomplete: "off", label: "Token Endpoint", placeholder: "Token Endpoint", required: true, pattern: q, width: f, get value() {
      return t().token_endpoint;
    }, set value(o) {
      t(t().token_endpoint = o, true);
    } });
    var B = a(G, 2);
    y(B, { typ: "url", autocomplete: "off", label: "Userinfo Endpoint", placeholder: "Userinfo Endpoint", required: true, pattern: q, width: f, get value() {
      return t().userinfo_endpoint;
    }, set value(o) {
      t(t().userinfo_endpoint = o, true);
    } });
    var _ = a(B, 2), xe = s(_);
    Z(xe, { ariaLabel: "PKCE", get checked() {
      return t().use_pkce;
    }, set checked(o) {
      t(t().use_pkce = o, true);
    }, children: (o, $) => {
      U();
      var w = D("PKCE");
      d(o, w);
    }, $$slots: { default: true } }), n(_);
    var le = a(_, 2), he = s(le, true);
    n(le);
    var ve = a(le, 2);
    y(ve, { autocomplete: "off", label: "Scope", placeholder: "openid profile email", required: true, pattern: Tt, width: f, get value() {
      return t().scope;
    }, set value(o) {
      t(t().scope = o, true);
    } });
    var _e = a(ve, 2), ge = s(_e, true);
    n(_e);
    var be = a(_e, 2);
    y(be, { autocomplete: "off", get label() {
      return i.providers.config.clientName;
    }, get placeholder() {
      return i.providers.config.clientName;
    }, required: true, pattern: At, width: f, get value() {
      return t().name;
    }, set value(o) {
      t(t().name = o, true);
    } });
    var Ie = a(be, 2), Ze = s(Ie, true);
    n(Ie);
    var Se = a(Ie, 2);
    y(Se, { autocomplete: "off", label: "Client ID", placeholder: "Client ID", required: true, pattern: q, width: f, get value() {
      return t().client_id;
    }, set value(o) {
      t(t().client_id = o, true);
    } });
    var Ne = a(Se, 2), et = s(Ne, true);
    n(Ne);
    var je = a(Ne, 2);
    const E = re(() => !t().use_pkce);
    Et(je, { autocomplete: "off", label: "Client Secret", placeholder: "Client Secret", maxLength: 256, get errMsg() {
      return i.providers.config.errConfidential;
    }, get required() {
      return e(E);
    }, width: f, get value() {
      return t().client_secret;
    }, set value(o) {
      t(t().client_secret = o, true);
    } });
    var j = a(je, 2), X = s(j);
    $e(X, () => i.providers.config.descAuthMethod), n(j);
    var se = a(j, 2), ne = s(se);
    Z(ne, { ariaLabel: "client_secret_basic", get checked() {
      return t().client_secret_basic;
    }, set checked(o) {
      t(t().client_secret_basic = o, true);
    }, children: (o, $) => {
      U();
      var w = D("client_secret_basic");
      d(o, w);
    }, $$slots: { default: true } }), n(se);
    var de = a(se, 2), Me = s(de);
    Z(Me, { ariaLabel: "client_secret_post", get checked() {
      return t().client_secret_post;
    }, set checked(o) {
      t(t().client_secret_post = o, true);
    }, children: (o, $) => {
      U();
      var w = D("client_secret_post");
      d(o, w);
    }, $$slots: { default: true } }), n(de);
    var K = a(de, 2);
    {
      var H = (o) => {
        var $ = nr(), w = s($, true);
        n($), C(() => v(w, i.providers.config.errNoAuthMethod)), Qe(3, $, () => Xe, () => ({ duration: 150 })), d(o, $);
      };
      L(K, (o) => {
        !t().use_pkce && !t().client_secret_basic && !t().client_secret_post && o(H);
      });
    }
    var te = a(K, 2);
    Lt(te, {});
    var oe = a(te, 2), Ke = s(oe, true);
    n(oe);
    var pe = a(oe, 2);
    y(pe, { autocomplete: "off", get label() {
      return i.providers.config.pathAdminClaim;
    }, placeholder: "$.roles.*", pattern: q, width: f, get value() {
      return t().admin_claim_path;
    }, set value(o) {
      t(t().admin_claim_path = o, true);
    } });
    var Re = a(pe, 2);
    y(Re, { autocomplete: "off", get label() {
      return i.providers.config.valueAdminClaim;
    }, placeholder: "rauthy_admin", pattern: q, width: f, get value() {
      return t().admin_claim_value;
    }, set value(o) {
      t(t().admin_claim_value = o, true);
    } });
    var Pe = a(Re, 2), Oe = s(Pe, true);
    n(Pe);
    var Ee = a(Pe, 2);
    y(Ee, { autocomplete: "off", get label() {
      return i.providers.config.pathMfaClaim;
    }, placeholder: "$.amr.*", pattern: q, width: f, get value() {
      return t().mfa_claim_path;
    }, set value(o) {
      t(t().mfa_claim_path = o, true);
    } });
    var Ve = a(Ee, 2);
    y(Ve, { autocomplete: "off", get label() {
      return i.providers.config.valueMfaClaim;
    }, placeholder: "mfa", pattern: q, width: f, get value() {
      return t().mfa_claim_value;
    }, set value(o) {
      t(t().mfa_claim_value = o, true);
    } });
    var qe = a(Ve, 2), We = s(qe);
    Qt(We, () => e(g), (o) => {
      var $ = lr(), w = s($);
      or(w, { get providerId() {
        return t().id;
      } }), n($), d(o, $);
    });
    var De = a(We, 2);
    const tt = re(() => `/auth/v1/providers/${t().id}/img`);
    Xt(De, { method: "PUT", get url() {
      return e(tt);
    }, fileName: "logo", onSuccess: () => u(g, h(ct())) }), n(qe);
    var Ue = a(qe, 2), ye = s(Ue);
    Be(ye, { type: "submit", get isLoading() {
      return k;
    }, children: (o, $) => {
      U();
      var w = D();
      C(() => v(w, l.common.save)), d(o, w);
    }, $$slots: { default: true } });
    var Ye = a(ye, 2);
    {
      var Je = (o) => {
        Pt(o, {});
      };
      L(Ye, (o) => {
        e(T) && o(Je);
      });
    }
    var ze = a(Ye, 2);
    {
      var rt = (o) => {
        var $ = dr(), w = s($, true);
        n($), C(() => v(w, e(p))), d(o, $);
      };
      L(ze, (o) => {
        e(p) && o(rt);
      });
    }
    n(Ue), C(() => {
      v(he, i.providers.config.descScope), v(ge, i.providers.config.descClientName), v(Ze, i.providers.config.descClientId), v(et, i.providers.config.descClientSecret), v(Ke, i.providers.config.mapUser), v(Oe, i.providers.config.mapMfa);
    }), d(m, P);
  }, $$slots: { default: true } }), n(O), d(V, O), we();
}
var _r = x('<div class="user svelte-1vu6qeb"> <span class="muted font-mono svelte-1vu6qeb"> </span></div>'), pr = x('<div class="forceDelete svelte-1vu6qeb"> </div>'), mr = x('<p class="err"><b> </b></p> <p> </p> <!> <p> </p> <!> <!>', 1), fr = x("<p> </p> <!>", 1), hr = x('<div class="err"> </div>'), gr = x('<div class="container svelte-1vu6qeb"><!> <!></div>');
function br(V, S) {
  ke(S, true);
  let t = mt(), l = Le(), i = R(true), f = R(""), k = R(false), p = R(h([]));
  xt(async () => {
    let m = await Kt(`/auth/v1/providers/${S.provider.id}/delete_safe`);
    m.status === 406 && m.error && u(p, h(m.error)), u(i, false);
  });
  async function T() {
    u(f, ""), u(i, true);
    let m = await Ot(`/auth/v1/providers/${S.provider.id}`);
    m.error ? u(f, h(m.error.message)) : S.onSave(), u(i, false);
  }
  var A = gr(), g = s(A);
  {
    var r = (m) => {
      var I = mr(), P = ie(I), N = s(P), W = s(N, true);
      n(N), n(P);
      var J = a(P, 2), Y = s(J, true);
      n(J);
      var z = a(J, 2);
      Zt(z, { summary: (B) => {
        U();
        var _ = D();
        C(() => v(_, l.providers.delete.linkedUsers)), d(B, _);
      }, details: (B) => {
        var _ = Gt(), xe = ie(_);
        It(xe, 17, () => e(p), (le) => le.id, (le, he) => {
          var ve = _r(), _e = s(ve), ge = a(_e), be = s(ge);
          n(ge), n(ve), C(() => {
            v(_e, `${e(he).email ?? ""} `), v(be, `/ ${e(he).id ?? ""}`);
          }), d(le, ve);
        }), d(B, _);
      }, $$slots: { summary: true, details: true } });
      var ee = a(z, 2), ue = s(ee, true);
      n(ee);
      var ae = a(ee, 2);
      Z(ae, { get ariaLabel() {
        return l.providers.delete.forceDelete;
      }, get checked() {
        return e(k);
      }, set checked(Q) {
        u(k, h(Q));
      }, children: (Q, G) => {
        var B = pr(), _ = s(B, true);
        n(B), C(() => v(_, l.providers.delete.forceDelete)), d(Q, B);
      }, $$slots: { default: true } });
      var Ce = a(ae, 2);
      {
        var fe = (Q) => {
          Be(Q, { level: -1, onclick: T, get isLoading() {
            return e(i);
          }, children: (G, B) => {
            U();
            var _ = D();
            C(() => v(_, t.common.delete)), d(G, _);
          }, $$slots: { default: true } });
        };
        L(Ce, (Q) => {
          e(k) && Q(fe);
        });
      }
      C(() => {
        v(W, l.providers.delete.isInUse1), v(Y, l.providers.delete.isInUse2), v(ue, l.providers.delete.areYouSure);
      }), d(m, I);
    }, O = (m) => {
      var I = fr(), P = ie(I), N = s(P, true);
      n(P);
      var W = a(P, 2);
      Be(W, { level: -1, onclick: T, get isLoading() {
        return e(i);
      }, children: (J, Y) => {
        U();
        var z = D();
        C(() => v(z, t.common.delete)), d(J, z);
      }, $$slots: { default: true } }), C(() => v(N, l.providers.delete.areYouSure)), d(m, I);
    };
    L(g, (m) => {
      e(p).length > 0 ? m(r) : m(O, false);
    });
  }
  var b = a(g, 2);
  {
    var M = (m) => {
      var I = hr(), P = s(I, true);
      n(I), C(() => v(P, e(f))), d(m, I);
    };
    L(b, (m) => {
      e(f) && m(M);
    });
  }
  n(A), d(V, A), we();
}
var $r = x('<div class="flex"><!></div> <!>', 1);
function kr(V, S) {
  ke(S, true);
  let t = dt(S, "provider", 15), l = dt(S, "onSave", 15), i = Le();
  const f = [i.tabs.config, i.tabs.delete];
  let k = R(h(f[0]));
  var p = $r(), T = ie(p), A = s(T);
  er(A, { tabs: f, get selected() {
    return e(k);
  }, set selected(b) {
    u(k, h(b));
  } }), n(T);
  var g = a(T, 2);
  {
    var r = (b) => {
      vr(b, { get provider() {
        return t();
      }, set provider(M) {
        t(M);
      }, get onSave() {
        return l();
      }, set onSave(M) {
        l(M);
      } });
    }, O = (b, M) => {
      {
        var m = (I) => {
          br(I, { get provider() {
            return t();
          }, get onSave() {
            return l();
          }, set onSave(P) {
            l(P);
          } });
        };
        L(b, (I) => {
          e(k) === i.tabs.delete && I(m);
        }, M);
      }
    };
    L(g, (b) => {
      e(k) === i.tabs.config ? b(r) : b(O, false);
    });
  }
  d(V, p), we();
}
var wr = x("<div><!></div>"), Cr = x('<div class="checkbox svelte-f8e9ia"><!></div>'), xr = x('<div class="checkbox svelte-f8e9ia"><!></div> <!>', 1), Ir = x('<div class="ml mb"><!></div>'), Pr = x('<div class="err"> </div>'), Er = x('<!> <!> <!> <!> <!> <div class="checkbox svelte-f8e9ia"><!></div> <p class="desc svelte-f8e9ia"> </p> <!> <p class="desc svelte-f8e9ia"> </p> <!> <p class="desc svelte-f8e9ia"> </p> <!> <p class="desc svelte-f8e9ia"> </p> <!> <p><!></p> <div class="checkbox svelte-f8e9ia"><!></div> <div class="checkbox svelte-f8e9ia"><!></div> <!> <!> <p class="desc svelte-f8e9ia"> </p> <!> <!> <p class="desc svelte-f8e9ia"> </p> <!> <!>', 1), yr = x('<div class="err"> </div>'), Tr = x('<!> <!> <div class="flex gap-05"><!> <!> <!> <!></div>', 1), Ar = x('<div class="container svelte-f8e9ia"><!> <div></div> <!></div>');
function Lr(V, S) {
  ke(S, true);
  let t = mt(), l = Le();
  const i = "min(calc(100dvw - 1.75rem), 30rem)";
  let f = R(false), k = R(""), p = R(false), T = R(false), A = R(false), g = R(h({ issuer: "", metadata_url: "", danger_allow_insecure: false, root_pem: "" })), r = R(h({ enabled: true, typ: "oidc", issuer: "", danger_allow_insecure: false, authorization_endpoint: "", token_endpoint: "", token_auth_method_basic: false, userinfo_endpoint: "", use_pkce: true, client_secret_basic: true, client_secret_post: false, name: "", client_id: "", client_secret: "", scope: "", root_pem: "", admin_claim_path: "", admin_claim_value: "", mfa_claim_path: "", mfa_claim_value: "" })), O = ["OIDC", "Auto", "Custom", "Github", "Google"], b = R(h(O[0])), M = re(() => e(b) === "Auto"), m = re(() => e(b) === "Custom"), I = re(() => e(b) === "OIDC"), P = re(() => !e(M) && !e(m) && !e(I)), N = re(() => !(e(P) || e(m) || e(p)));
  const W = "/auth/v1/providers/lookup";
  let J = re(() => e(N) ? W : "/auth/v1/providers/create");
  Ge(() => {
    if (e(b)) switch (u(p, false), u(g, h({ issuer: "", metadata_url: "", danger_allow_insecure: false, root_pem: "" })), e(b)) {
      case "Github":
        u(r, h({ enabled: true, issuer: "github.com", typ: "github", danger_allow_insecure: false, authorization_endpoint: "https://github.com/login/oauth/authorize", token_endpoint: "https://github.com/login/oauth/access_token", client_secret_basic: true, client_secret_post: true, userinfo_endpoint: "https://api.github.com/user", use_pkce: false, name: "Github", client_id: "", client_secret: "", scope: "user:email", root_pem: "", admin_claim_path: "", admin_claim_value: "", mfa_claim_path: "$.two_factor_authentication", mfa_claim_value: "true" }));
        break;
      case "Google":
        u(g, h({ issuer: "accounts.google.com", metadata_url: "", danger_allow_insecure: false, root_pem: "" })), Bt(() => {
          ee(W);
        });
        break;
      default:
        u(r, h({ enabled: true, issuer: "", typ: "oidc", danger_allow_insecure: false, authorization_endpoint: "", token_endpoint: "", client_secret_basic: true, client_secret_post: false, userinfo_endpoint: "", use_pkce: true, name: "", client_id: "", client_secret: "", scope: "", root_pem: "", admin_claim_path: "", admin_claim_value: "", mfa_claim_path: "", mfa_claim_value: "" }));
    }
  }), Ge(() => {
    e(T) && setTimeout(() => {
      S.onSave(), ue();
    }, 1500);
  });
  async function Y(G, B) {
    e(N) ? await ee(G.action) : await z(G.action);
  }
  async function z(G) {
    if (e(r).client_secret && !(e(r).client_secret_basic || e(r).client_secret_post)) {
      u(k, h(l.providers.config.errNoAuthMethod));
      return;
    }
    if (!e(r).use_pkce && !e(r).client_secret) {
      u(k, h(l.providers.config.errConfidential));
      return;
    }
    u(k, ""), u(f, true);
    let B = { name: e(r).name, typ: e(M) ? "custom" : e(b).toLowerCase(), enabled: e(r).enabled, issuer: e(r).issuer, authorization_endpoint: e(r).authorization_endpoint, token_endpoint: e(r).token_endpoint, userinfo_endpoint: e(r).userinfo_endpoint, danger_allow_insecure: e(A) && e(r).root_pem ? false : e(r).danger_allow_insecure, use_pkce: e(r).use_pkce, client_secret_basic: e(r).client_secret_basic, client_secret_post: e(r).client_secret_post, client_id: e(r).client_id, client_secret: e(r).client_secret, scope: e(r).scope.trim(), root_pem: e(A) && e(r).root_pem ? e(r).root_pem.trim() : void 0, admin_claim_path: e(r).admin_claim_path || void 0, admin_claim_value: e(r).admin_claim_value || void 0, mfa_claim_path: e(r).mfa_claim_path || void 0, mfa_claim_value: e(r).mfa_claim_value || void 0 }, _ = await lt(G, B);
    _.error ? _.error.message.includes("InvalidCertificate") ? u(k, "Insecure connection not allowed") : u(k, h(_.error.message)) : u(T, true), u(f, false);
  }
  async function ee(G) {
    if (!e(g).issuer && !e(g).metadata_url) {
      u(k, "Provide at least one of Issuer / Metadata URL");
      return;
    }
    u(k, ""), u(f, true);
    let B = { issuer: e(g).issuer || void 0, metadata_url: e(g).metadata_url || void 0, danger_allow_insecure: e(g).danger_allow_insecure, root_pem: e(g).root_pem || void 0 }, _ = await lt(G, B);
    _.body ? (e(r).issuer = _.body.issuer, e(r).authorization_endpoint = _.body.authorization_endpoint, e(r).danger_allow_insecure = _.body.danger_allow_insecure, e(r).token_endpoint = _.body.token_endpoint, e(r).userinfo_endpoint = _.body.userinfo_endpoint, e(r).client_secret_basic = _.body.client_secret_basic, e(r).client_secret_post = _.body.client_secret_post, e(r).use_pkce = _.body.use_pkce, e(r).client_secret_basic = _.body.client_secret_basic, e(r).client_secret_post = !_.body.client_secret_basic && _.body.client_secret_post, e(r).scope = _.body.scope, e(r).root_pem = _.body.root_pem, u(p, true)) : _.error && (_.error.message.includes("InvalidCertificate") ? u(k, "Insecure connection not allowed") : u(k, h(_.error.message))), u(f, false);
  }
  function ue() {
    u(g, h({ issuer: "", metadata_url: "", danger_allow_insecure: false, root_pem: "" })), u(r, h({ enabled: true, issuer: "", typ: "oidc", danger_allow_insecure: false, authorization_endpoint: "", token_endpoint: "", userinfo_endpoint: "", use_pkce: true, name: "", client_id: "", client_secret_basic: true, client_secret_post: false, scope: "", root_pem: "", admin_claim_path: "", admin_claim_value: "", mfa_claim_path: "", mfa_claim_value: "" })), u(T, false), u(p, false), u(A, false);
  }
  var ae = Ar(), Ce = s(ae);
  tr(Ce, { ariaLabel: "Select Mode", options: O, get value() {
    return e(b);
  }, set value(G) {
    u(b, h(G));
  } });
  var fe = a(Ce, 2);
  pt(fe, "", {}, { height: ".5rem" });
  var Q = a(fe, 2);
  yt(Q, { get action() {
    return e(J);
  }, onSubmit: Y, children: (G, B) => {
    var _ = Tr(), xe = ie(_);
    {
      var le = (E) => {
        var j = xr(), X = ie(j), se = s(X);
        Z(se, { get ariaLabel() {
          return l.providers.config.custRootCa;
        }, get checked() {
          return e(A);
        }, set checked(K) {
          u(A, h(K));
        }, children: (K, H) => {
          U();
          var te = D();
          C(() => v(te, l.providers.config.custRootCa)), d(K, te);
        }, $$slots: { default: true } }), n(X);
        var ne = a(X, 2);
        {
          var de = (K) => {
            var H = wr(), te = s(H);
            ut(te, { rows: 15, name: "rootPem", get label() {
              return l.providers.config.rootPemCert;
            }, placeholder: `-----BEGIN CERTIFICATE-----
...
-----END CERTIFICATE-----`, errMsg: "-----BEGIN CERTIFICATE----- ...", width: "min(40rem, calc(100dvw - 1.75rem))", fontMono: true, pattern: _t, get value() {
              return e(g).root_pem;
            }, set value(oe) {
              e(g).root_pem = oe;
            } }), n(H), Qe(3, H, () => Xe, () => ({ duration: 150 })), d(K, H);
          }, Me = (K) => {
            var H = Cr(), te = s(H);
            Z(te, { get ariaLabel() {
              return l.providers.config.allowInsecureTls;
            }, get checked() {
              return e(g).danger_allow_insecure;
            }, set checked(oe) {
              e(g).danger_allow_insecure = oe;
            }, children: (oe, Ke) => {
              U();
              var pe = D();
              C(() => v(pe, l.providers.config.allowInsecureTls)), d(oe, pe);
            }, $$slots: { default: true } }), n(H), d(K, H);
          };
          L(ne, (K) => {
            e(A) ? K(de) : K(Me, false);
          });
        }
        d(E, j);
      };
      L(xe, (E) => {
        e(p) || E(le);
      });
    }
    var he = a(xe, 2);
    {
      var ve = (E) => {
        y(E, { name: "issuer", label: "Issuer URL", placeholder: "Issuer URL", width: i, required: true, pattern: q, get value() {
          return e(g).issuer;
        }, set value(j) {
          e(g).issuer = j;
        } });
      }, _e = (E, j) => {
        {
          var X = (ne) => {
            y(ne, { typ: "url", name: "metadata", label: "Metadata URL", placeholder: ".../.well-known/openid-configuration", width: i, required: true, pattern: q, get value() {
              return e(g).metadata_url;
            }, set value(de) {
              e(g).metadata_url = de;
            } });
          }, se = (ne, de) => {
            {
              var Me = (K) => {
                var H = Er(), te = ie(H);
                {
                  var oe = (c) => {
                    ut(c, { rows: 17, name: "rootPem", get label() {
                      return l.providers.config.rootPemCert;
                    }, placeholder: `-----BEGIN CERTIFICATE-----
...
-----END CERTIFICATE-----`, get disabled() {
                      return e(p);
                    }, errMsg: "-----BEGIN CERTIFICATE----- ...", width: "min(40rem, calc(100dvw - 1.75rem))", fontMono: true, pattern: _t, get value() {
                      return e(r).root_pem;
                    }, set value(F) {
                      e(r).root_pem = F;
                    } });
                  }, Ke = (c) => {
                    var F = Ir(), ce = s(F);
                    {
                      var nt = (Ae) => {
                        vt(Ae, { get label() {
                          return l.providers.config.allowInsecureTls;
                        }, children: (Fe, Ft) => {
                          Ct(Fe, {});
                        }, $$slots: { default: true } });
                      }, zt = (Ae) => {
                        Z(Ae, { get ariaLabel() {
                          return l.providers.config.allowInsecureTls;
                        }, get checked() {
                          return e(r).danger_allow_insecure;
                        }, set checked(Fe) {
                          e(r).danger_allow_insecure = Fe;
                        }, children: (Fe, Ft) => {
                          U();
                          var wt = D();
                          C(() => v(wt, l.providers.config.allowInsecureTls)), d(Fe, wt);
                        }, $$slots: { default: true } });
                      };
                      L(ce, (Ae) => {
                        e(p) ? Ae(nt) : Ae(zt, false);
                      });
                    }
                    n(F), d(c, F);
                  };
                  L(te, (c) => {
                    e(A) ? c(oe) : c(Ke, false);
                  });
                }
                var pe = a(te, 2);
                y(pe, { name: "issuer", label: "Issuer URL", placeholder: "Issuer URL", width: i, get disabled() {
                  return e(p);
                }, required: true, pattern: q, get value() {
                  return e(r).issuer;
                }, set value(c) {
                  e(r).issuer = c;
                } });
                var Re = a(pe, 2);
                y(Re, { typ: "url", name: "auth_endpoint", label: "Authorization Endpoint", placeholder: "Authorization Endpoint", width: i, get disabled() {
                  return e(p);
                }, required: true, pattern: q, get value() {
                  return e(r).authorization_endpoint;
                }, set value(c) {
                  e(r).authorization_endpoint = c;
                } });
                var Pe = a(Re, 2);
                y(Pe, { typ: "url", name: "token_endpoint", label: "Token Endpoint", placeholder: "Token Endpoint", width: i, get disabled() {
                  return e(p);
                }, required: true, pattern: q, get value() {
                  return e(r).token_endpoint;
                }, set value(c) {
                  e(r).token_endpoint = c;
                } });
                var Oe = a(Pe, 2);
                y(Oe, { typ: "url", name: "userinfo_endpoint", label: "Userinfo Endpoint", placeholder: "Userinfo Endpoint", width: i, get disabled() {
                  return e(p);
                }, required: true, pattern: q, get value() {
                  return e(r).userinfo_endpoint;
                }, set value(c) {
                  e(r).userinfo_endpoint = c;
                } });
                var Ee = a(Oe, 2), Ve = s(Ee);
                {
                  var qe = (c) => {
                    vt(c, { label: "PKCE", children: (F, ce) => {
                      Ct(F, {});
                    }, $$slots: { default: true } });
                  }, We = (c) => {
                    Z(c, { ariaLabel: "PKCE", get checked() {
                      return e(r).use_pkce;
                    }, set checked(F) {
                      e(r).use_pkce = F;
                    }, children: (F, ce) => {
                      U();
                      var nt = D("PKCE");
                      d(F, nt);
                    }, $$slots: { default: true } });
                  };
                  L(Ve, (c) => {
                    e(p) ? c(qe) : c(We, false);
                  });
                }
                n(Ee);
                var De = a(Ee, 2), tt = s(De, true);
                n(De);
                var Ue = a(De, 2);
                y(Ue, { name: "scope", label: "Scope", placeholder: "openid profile email", width: i, required: true, pattern: Tt, get value() {
                  return e(r).scope;
                }, set value(c) {
                  e(r).scope = c;
                } });
                var ye = a(Ue, 2), Ye = s(ye, true);
                n(ye);
                var Je = a(ye, 2);
                y(Je, { name: "client_name", label: "Client Name", placeholder: "Client Name", width: i, required: true, pattern: At, get value() {
                  return e(r).name;
                }, set value(c) {
                  e(r).name = c;
                } });
                var ze = a(Je, 2), rt = s(ze, true);
                n(ze);
                var o = a(ze, 2);
                y(o, { name: "client_id", autocomplete: "off", label: "Client ID", placeholder: "Client ID", width: i, required: true, pattern: q, get value() {
                  return e(r).client_id;
                }, set value(c) {
                  e(r).client_id = c;
                } });
                var $ = a(o, 2), w = s($, true);
                n($);
                var me = a($, 2);
                const ft = re(() => !e(r).use_pkce);
                Et(me, { name: "client_secret", autocomplete: "off", label: "Client Secret", placeholder: "Client Secret", width: i, get errMsg() {
                  return l.providers.config.errConfidential;
                }, maxLength: 256, get required() {
                  return e(ft);
                }, get value() {
                  return e(r).client_secret;
                }, set value(c) {
                  e(r).client_secret = c;
                } });
                var Te = a(me, 2), St = s(Te);
                $e(St, () => l.providers.config.descAuthMethod), n(Te);
                var at = a(Te, 2), Nt = s(at);
                Z(Nt, { ariaLabel: "client_secret_basic", get checked() {
                  return e(r).client_secret_basic;
                }, set checked(c) {
                  e(r).client_secret_basic = c;
                }, children: (c, F) => {
                  U();
                  var ce = D("client_secret_basic");
                  d(c, ce);
                }, $$slots: { default: true } }), n(at);
                var ot = a(at, 2), Mt = s(ot);
                Z(Mt, { ariaLabel: "client_secret_post", get checked() {
                  return e(r).client_secret_post;
                }, set checked(c) {
                  e(r).client_secret_post = c;
                }, children: (c, F) => {
                  U();
                  var ce = D("client_secret_post");
                  d(c, ce);
                }, $$slots: { default: true } }), n(ot);
                var ht = a(ot, 2);
                {
                  var Rt = (c) => {
                    var F = Pr(), ce = s(F, true);
                    n(F), C(() => v(ce, l.providers.config.errNoAuthMethod)), Qe(3, F, () => Xe, () => ({ duration: 150 })), d(c, F);
                  };
                  L(ht, (c) => {
                    !e(r).use_pkce && !e(r).client_secret_basic && !e(r).client_secret_post && c(Rt);
                  });
                }
                var gt = a(ht, 2);
                Lt(gt, {});
                var it = a(gt, 2), qt = s(it, true);
                n(it);
                var bt = a(it, 2);
                y(bt, { name: "admin_claim_path", get label() {
                  return l.providers.config.pathAdminClaim;
                }, placeholder: "$.roles.*", width: i, pattern: q, get value() {
                  return e(r).admin_claim_path;
                }, set value(c) {
                  e(r).admin_claim_path = c;
                } });
                var $t = a(bt, 2);
                y($t, { name: "admin_claim_value", get label() {
                  return l.providers.config.valueAdminClaim;
                }, placeholder: "rauthy_admin", width: i, pattern: q, get value() {
                  return e(r).admin_claim_value;
                }, set value(c) {
                  e(r).admin_claim_value = c;
                } });
                var st = a($t, 2), Dt = s(st, true);
                n(st);
                var kt = a(st, 2);
                y(kt, { name: "mfa_claim_path", get label() {
                  return l.providers.config.pathMfaClaim;
                }, placeholder: "$.amr.*", width: i, pattern: q, get value() {
                  return e(r).mfa_claim_path;
                }, set value(c) {
                  e(r).mfa_claim_path = c;
                } });
                var Ut = a(kt, 2);
                y(Ut, { name: "mfa_claim_value", get label() {
                  return l.providers.config.valueMfaClaim;
                }, placeholder: "mfa", width: i, pattern: q, get value() {
                  return e(r).mfa_claim_value;
                }, set value(c) {
                  e(r).mfa_claim_value = c;
                } }), C(() => {
                  v(tt, l.providers.config.descScope), v(Ye, l.providers.config.descClientName), v(rt, l.providers.config.descClientId), v(w, l.providers.config.descClientId), v(qt, l.providers.config.mapUser), v(Dt, l.providers.config.mapMfa);
                }), d(K, H);
              };
              L(ne, (K) => {
                (e(P) || e(m) || e(p)) && K(Me);
              }, de);
            }
          };
          L(E, (ne) => {
            e(M) && !e(p) ? ne(X) : ne(se, false);
          }, j);
        }
      };
      L(he, (E) => {
        e(I) && !e(p) ? E(ve) : E(_e, false);
      });
    }
    var ge = a(he, 2), be = s(ge);
    Be(be, { type: "submit", get isLoading() {
      return e(f);
    }, children: (E, j) => {
      U();
      var X = D();
      C(() => v(X, e(N) ? l.providers.config.lookup : t.common.save)), d(E, X);
    }, $$slots: { default: true } });
    var Ie = a(be, 2);
    {
      var Ze = (E) => {
        Be(E, { level: 3, onclick: ue, get isLoading() {
          return e(f);
        }, children: (j, X) => {
          U();
          var se = D();
          C(() => v(se, l.common.reset)), d(j, se);
        }, $$slots: { default: true } });
      };
      L(Ie, (E) => {
        e(N) || E(Ze);
      });
    }
    var Se = a(Ie, 2);
    {
      var Ne = (E) => {
        Pt(E, {});
      };
      L(Se, (E) => {
        e(T) && E(Ne);
      });
    }
    var et = a(Se, 2);
    {
      var je = (E) => {
        var j = yr(), X = s(j, true);
        n(j), C(() => v(X, e(k))), d(E, j);
      };
      L(et, (E) => {
        e(k) && E(je);
      });
    }
    n(ge), d(G, _);
  }, $$slots: { default: true } }), n(ae), d(V, ae), we();
}
var Sr = x("<div></div> <!>", 1), Nr = x('<div class="err"> </div>'), Mr = x('<div id="federation"><!></div> <!>', 1), Rr = x("<!> <!>", 1);
function qr(V, S) {
  ke(S, true);
  let t = Le(), l = R(void 0), i = Yt("pid"), f = R(""), k = R(h([])), p = R(void 0);
  xt(() => {
    T();
  }), Ge(() => {
    u(p, h(e(k).find((b) => b.id === i.get())));
  });
  async function T() {
    var _a;
    let b = await lt("/auth/v1/providers");
    b.body ? u(k, h(b.body)) : u(f, h(((_a = b.error) == null ? void 0 : _a.message) || "Error"));
  }
  function A() {
    var _a;
    i.set(void 0), (_a = e(l)) == null ? void 0 : _a(), T();
  }
  var g = Rr(), r = ie(g);
  Wt(r, { width: "11rem", buttonTilesAriaControls: "federation", paddingTop: "6.65rem", buttonTiles: (M) => {
    var m = Sr(), I = ie(m);
    pt(I, "", {}, { height: ".5rem" });
    var P = a(I, 2);
    It(P, 17, () => e(k), (N) => N.id, (N, W, J, Y) => {
      const z = re(() => i.get() === e(W).id);
      Jt(N, { onclick: () => i.set(e(W).id), get selected() {
        return e(z);
      }, children: (ee, ue) => {
        U();
        var ae = D();
        C(() => v(ae, e(W).name)), d(ee, ae);
      } });
    }), d(M, m);
  }, children: (M, m) => {
    const I = re(() => e(k).length === 0 ? 1 : 2);
    Ht(M, { get level() {
      return e(I);
    }, get closeModal() {
      return e(l);
    }, set closeModal(P) {
      u(l, h(P));
    }, children: (P, N) => {
      Lr(P, { onSave: A });
    }, $$slots: { default: true } });
  }, $$slots: { buttonTiles: true, default: true } });
  var O = a(r, 2);
  Vt(O, { children: (b, M) => {
    var m = Mr(), I = ie(m), P = s(I);
    {
      var N = (Y) => {
        kr(Y, { onSave: T, get provider() {
          return e(p);
        }, set provider(z) {
          u(p, h(z));
        } });
      };
      L(P, (Y) => {
        e(p) && Y(N);
      });
    }
    n(I);
    var W = a(I, 2);
    {
      var J = (Y) => {
        var z = Nr(), ee = s(z, true);
        n(z), C(() => v(ee, e(f))), d(Y, z);
      };
      L(W, (Y) => {
        e(f) && Y(J);
      });
    }
    C(() => He(I, "aria-label", t.common.details)), d(b, m);
  } }), d(V, g), we();
}
function fa(V) {
  qr(V, {});
}
export {
  fa as component
};
