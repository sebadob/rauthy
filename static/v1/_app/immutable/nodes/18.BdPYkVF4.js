import { t as I, a as l, e as D, d as He } from "../chunks/BH6NCLk-.js";
import { p as we, s as a, c as s, r as n, t as x, a as Ce, k as R, aa as Ge, f as H, a7 as U, l as u, j as e, a9 as ie, a5 as Pt, O as Kt } from "../chunks/CvlvO1XB.js";
import { s as v } from "../chunks/CTI4QPiR.js";
import { i as S } from "../chunks/BUO_AUgz.js";
import { e as Et } from "../chunks/BpWRzPRQ.js";
import { s as Je } from "../chunks/BMbqVy6X.js";
import { a as ft, B as Be, t as Qe, s as Xe } from "../chunks/DMkkW5Nn.js";
import { p as h } from "../chunks/Wh68IIk2.js";
import { d as Ot, f as Vt, c as Wt, b as ct } from "../chunks/BO1A6s0c.js";
import { C as Yt } from "../chunks/BnPoFdx3.js";
import { N as Jt } from "../chunks/DZ-K8XgE.js";
import { u as Ht } from "../chunks/BYZsu9rB.js";
import { N as Qt } from "../chunks/D-L0o8jR.js";
import { B as Xt } from "../chunks/ClvgHNP_.js";
import { p as ut } from "../chunks/C6SR4G2t.js";
import { k as Zt } from "../chunks/CaD2yKt4.js";
import { h as $e } from "../chunks/i8Xqpu09.js";
import { I as T } from "../chunks/BZu_mjh1.js";
import { u as Ae } from "../chunks/D8mHI_K9.js";
import { g as vt } from "../chunks/B21bTIl7.js";
import { I as yt } from "../chunks/Nks81rMs.js";
import { I as _t } from "../chunks/Cxg1YoLR.js";
import { I as Tt } from "../chunks/B-xX0s4n.js";
import { F as At } from "../chunks/CsaAZyUr.js";
import { u as ht } from "../chunks/CUqQZdNU.js";
import { L as pt } from "../chunks/nnFaiMsH.js";
import { I as te } from "../chunks/D9bDr0Ir.js";
import { a as q, h as Lt, g as St, i as mt } from "../chunks/BRCxk8by.js";
import { I as er } from "../chunks/zg4ohzvY.js";
import { E as tr } from "../chunks/ZNvtRdQk.js";
import { T as rr } from "../chunks/_OE2Cq0B.js";
import { C as It } from "../chunks/CS_Msctd.js";
import { O as ar } from "../chunks/D1anwFsC.js";
var or = I('<div class="desc svelte-6fcb3i"><h4 class="svelte-6fcb3i">ID token claim mappings</h4> <p class="svelte-6fcb3i"> </p> <p class="svelte-6fcb3i"><!></p> <p class="svelte-6fcb3i"><!></p> <p class="svelte-6fcb3i"><!></p> <p class="svelte-6fcb3i"><!></p> <p class="svelte-6fcb3i"><!></p></div>');
function Nt(O, N) {
  we(N, true);
  let t = Ae();
  var d = or(), i = a(s(d), 2), f = s(i, true);
  n(i);
  var k = a(i, 2), p = s(k);
  $e(p, () => t.providers.config.jsonPath.p2), n(k);
  var A = a(k, 2), L = s(A);
  $e(L, () => t.providers.config.jsonPath.p3), n(A);
  var g = a(A, 2), r = s(g);
  $e(r, () => t.providers.config.jsonPath.p4), n(g);
  var K = a(g, 2), b = s(K);
  $e(b, () => t.providers.config.jsonPath.p5), n(K);
  var M = a(K, 2), m = s(M);
  $e(m, () => t.providers.config.jsonPath.p6), n(M), n(d), x(() => v(f, t.providers.config.jsonPath.p1)), l(O, d), Ce();
}
var ir = I('<img alt="No Logo Available">');
function sr(O, N) {
  we(N, true);
  var t = ir();
  Je(t, "width", 20), Je(t, "height", 20), x((d) => Je(t, "src", d), [() => `/auth/v1/providers/${N.providerId}/img?${vt()}}`]), l(O, t), Ce();
}
var nr = I("<div><!></div>"), lr = I('<div class="checkbox svelte-iec8od"><!></div>'), dr = I('<div class="err"> </div>'), cr = I('<div class="svelte-iec8od"><!></div>'), ur = I('<span class="err"> </span>'), vr = I('<!> <div></div> <div class="checkbox svelte-iec8od"><!></div> <div class="checkbox svelte-iec8od"><!></div> <!> <!> <!> <!> <!> <div class="checkbox svelte-iec8od"><!></div> <p class="desc svelte-iec8od"> </p> <!> <p class="desc svelte-iec8od"> </p> <!> <p class="desc svelte-iec8od"> </p> <!> <p class="desc svelte-iec8od"> </p> <!> <p><!></p> <div class="checkbox svelte-iec8od"><!></div> <div class="checkbox svelte-iec8od"><!></div> <!> <!> <p class="desc svelte-iec8od"> </p> <!> <!> <p class="desc svelte-iec8od"> </p> <!> <!> <div class="logo svelte-iec8od"><!> <!></div> <div class="flex gap-05"><!> <!> <!></div>', 1), _r = I('<div class="container svelte-iec8od"><!></div>');
function pr(O, N) {
  we(N, true);
  let t = ut(N, "provider", 15), d = ht(), i = Ae();
  const f = "min(calc(100dvw - .5rem), 30rem)";
  let k = false, p = R(""), A = R(false), L = R(!!t().root_pem), g = R(h(vt()));
  Ge(() => {
    t().id && (t(t().client_secret = t().client_secret || "", true), t(t().admin_claim_path = t().admin_claim_path || "", true), t(t().admin_claim_value = t().admin_claim_value || "", true), t(t().mfa_claim_path = t().mfa_claim_path || "", true), t(t().mfa_claim_value = t().mfa_claim_value || "", true), t(t().root_pem = t().root_pem || "", true));
  }), Ge(() => {
    t().scope && t(t().scope = t().scope.replaceAll("+", " "), true);
  });
  async function r(m, P) {
    if (u(p, ""), t().client_secret && !(t().client_secret_basic || t().client_secret_post)) {
      u(p, h(i.providers.config.errNoAuthMethod));
      return;
    }
    if (!t().use_pkce && !t().client_secret) {
      u(p, h(i.providers.config.errConfidential));
      return;
    }
    let C = { name: t().name, typ: t().typ, enabled: t().enabled, issuer: t().issuer, authorization_endpoint: t().authorization_endpoint, token_endpoint: t().token_endpoint, userinfo_endpoint: t().userinfo_endpoint, danger_allow_insecure: e(L) && t().root_pem ? false : t().danger_allow_insecure, use_pkce: t().use_pkce, client_secret_basic: t().client_secret_basic, client_secret_post: t().client_secret_post, client_id: t().client_id, client_secret: t().client_secret || void 0, scope: t().scope.trim(), root_pem: e(L) && t().root_pem ? t().root_pem.trim() : void 0, admin_claim_path: t().admin_claim_path || void 0, admin_claim_value: t().admin_claim_value || void 0, mfa_claim_path: t().mfa_claim_path || void 0, mfa_claim_value: t().mfa_claim_value || void 0 }, y = await Ot(m.action, C);
    y.error ? u(p, h(y.error.message)) : (u(A, true), N.onSave(), setTimeout(() => {
      u(A, false);
    }, 3e3));
  }
  var K = _r(), b = s(K);
  const M = ie(() => `/auth/v1/providers/${t().id}`);
  At(b, { get action() {
    return e(M);
  }, onSubmit: r, children: (m, P) => {
    var C = vr(), y = H(C);
    pt(y, { label: "ID", mono: true, children: (o, $) => {
      U();
      var w = D();
      x(() => v(w, t().id)), l(o, w);
    } });
    var V = a(y, 2);
    ft(V, "height", ".15rem");
    var J = a(V, 2), W = s(J);
    te(W, { get ariaLabel() {
      return i.common.enabled;
    }, get checked() {
      return t().enabled;
    }, set checked(o) {
      t(t().enabled = o, true);
    }, children: (o, $) => {
      U();
      var w = D();
      x(() => v(w, i.common.enabled)), l(o, w);
    }, $$slots: { default: true } }), n(J);
    var z = a(J, 2), re = s(z);
    te(re, { get ariaLabel() {
      return i.providers.config.custRootCa;
    }, get checked() {
      return e(L);
    }, set checked(o) {
      u(L, h(o));
    }, children: (o, $) => {
      U();
      var w = D();
      x(() => v(w, i.providers.config.custRootCa)), l(o, w);
    }, $$slots: { default: true } }), n(z);
    var ue = a(z, 2);
    {
      var se = (o) => {
        var $ = nr(), w = s($);
        _t(w, { rows: 15, name: "rootPem", get label() {
          return i.providers.config.rootPemCert;
        }, placeholder: `-----BEGIN CERTIFICATE-----
...
 -----END CERTIFICATE-----`, errMsg: "-----BEGIN CERTIFICATE----- ...", width: "min(40rem, calc(100dvw - .5rem))", fontMono: true, pattern: mt, get value() {
          return t().root_pem;
        }, set value(de) {
          t(t().root_pem = de, true);
        } }), n($), Qe(3, $, () => Xe, () => ({ duration: 150 })), l(o, $);
      }, xe = (o) => {
        var $ = lr(), w = s($);
        te(w, { get ariaLabel() {
          return i.providers.config.allowInsecureTls;
        }, get checked() {
          return t().danger_allow_insecure;
        }, set checked(de) {
          t(t().danger_allow_insecure = de, true);
        }, children: (de, gt) => {
          U();
          var ze = D();
          x(() => v(ze, i.providers.config.allowInsecureTls)), l(de, ze);
        }, $$slots: { default: true } }), n($), l(o, $);
      };
      S(ue, (o) => {
        e(L) ? o(se) : o(xe, false);
      });
    }
    var me = a(ue, 2);
    T(me, { autocomplete: "off", label: "Issuer URL", placeholder: "Issuer URL", required: true, pattern: q, width: f, get value() {
      return t().issuer;
    }, set value(o) {
      t(t().issuer = o, true);
    } });
    var Q = a(me, 2);
    T(Q, { typ: "url", autocomplete: "off", label: "Authorization Endpoint", placeholder: "Authorization Endpoint", required: true, pattern: q, width: f, get value() {
      return t().authorization_endpoint;
    }, set value(o) {
      t(t().authorization_endpoint = o, true);
    } });
    var B = a(Q, 2);
    T(B, { typ: "url", autocomplete: "off", label: "Token Endpoint", placeholder: "Token Endpoint", required: true, pattern: q, width: f, get value() {
      return t().token_endpoint;
    }, set value(o) {
      t(t().token_endpoint = o, true);
    } });
    var j = a(B, 2);
    T(j, { typ: "url", autocomplete: "off", label: "Userinfo Endpoint", placeholder: "Userinfo Endpoint", required: true, pattern: q, width: f, get value() {
      return t().userinfo_endpoint;
    }, set value(o) {
      t(t().userinfo_endpoint = o, true);
    } });
    var _ = a(j, 2), Ie = s(_);
    te(Ie, { ariaLabel: "PKCE", get checked() {
      return t().use_pkce;
    }, set checked(o) {
      t(t().use_pkce = o, true);
    }, children: (o, $) => {
      U();
      var w = D("PKCE");
      l(o, w);
    }, $$slots: { default: true } }), n(_);
    var le = a(_, 2), fe = s(le, true);
    n(le);
    var ve = a(le, 2);
    T(ve, { autocomplete: "off", label: "Scope", placeholder: "openid profile email", required: true, pattern: Lt, width: f, get value() {
      return t().scope;
    }, set value(o) {
      t(t().scope = o, true);
    } });
    var _e = a(ve, 2), he = s(_e, true);
    n(_e);
    var ge = a(_e, 2);
    T(ge, { autocomplete: "off", get label() {
      return i.providers.config.clientName;
    }, get placeholder() {
      return i.providers.config.clientName;
    }, required: true, pattern: St, width: f, get value() {
      return t().name;
    }, set value(o) {
      t(t().name = o, true);
    } });
    var Pe = a(ge, 2), Ze = s(Pe, true);
    n(Pe);
    var Le = a(Pe, 2);
    T(Le, { autocomplete: "off", label: "Client ID", placeholder: "Client ID", required: true, pattern: q, width: f, get value() {
      return t().client_id;
    }, set value(o) {
      t(t().client_id = o, true);
    } });
    var Se = a(Le, 2), et = s(Se, true);
    n(Se);
    var je = a(Se, 2);
    const E = ie(() => !t().use_pkce);
    Tt(je, { autocomplete: "off", label: "Client Secret", placeholder: "Client Secret", maxLength: 256, get errMsg() {
      return i.providers.config.errConfidential;
    }, get required() {
      return e(E);
    }, width: f, get value() {
      return t().client_secret;
    }, set value(o) {
      t(t().client_secret = o, true);
    } });
    var F = a(je, 2), X = s(F);
    $e(X, () => i.providers.config.descAuthMethod), n(F);
    var ne = a(F, 2), Ne = s(ne);
    te(Ne, { ariaLabel: "client_secret_basic", get checked() {
      return t().client_secret_basic;
    }, set checked(o) {
      t(t().client_secret_basic = o, true);
    }, children: (o, $) => {
      U();
      var w = D("client_secret_basic");
      l(o, w);
    }, $$slots: { default: true } }), n(ne);
    var ae = a(ne, 2), pe = s(ae);
    te(pe, { ariaLabel: "client_secret_post", get checked() {
      return t().client_secret_post;
    }, set checked(o) {
      t(t().client_secret_post = o, true);
    }, children: (o, $) => {
      U();
      var w = D("client_secret_post");
      l(o, w);
    }, $$slots: { default: true } }), n(ae);
    var Y = a(ae, 2);
    {
      var Z = (o) => {
        var $ = dr(), w = s($, true);
        n($), x(() => v(w, i.providers.config.errNoAuthMethod)), Qe(3, $, () => Xe, () => ({ duration: 150 })), l(o, $);
      };
      S(Y, (o) => {
        !t().use_pkce && !t().client_secret_basic && !t().client_secret_post && o(Z);
      });
    }
    var ee = a(Y, 2);
    Nt(ee, {});
    var oe = a(ee, 2), Me = s(oe, true);
    n(oe);
    var be = a(oe, 2);
    T(be, { autocomplete: "off", get label() {
      return i.providers.config.pathAdminClaim;
    }, placeholder: "$.roles.*", pattern: q, width: f, get value() {
      return t().admin_claim_path;
    }, set value(o) {
      t(t().admin_claim_path = o, true);
    } });
    var Ke = a(be, 2);
    T(Ke, { autocomplete: "off", get label() {
      return i.providers.config.valueAdminClaim;
    }, placeholder: "rauthy_admin", pattern: q, width: f, get value() {
      return t().admin_claim_value;
    }, set value(o) {
      t(t().admin_claim_value = o, true);
    } });
    var Ee = a(Ke, 2), Oe = s(Ee, true);
    n(Ee);
    var Re = a(Ee, 2);
    T(Re, { autocomplete: "off", get label() {
      return i.providers.config.pathMfaClaim;
    }, placeholder: "$.amr.*", pattern: q, width: f, get value() {
      return t().mfa_claim_path;
    }, set value(o) {
      t(t().mfa_claim_path = o, true);
    } });
    var qe = a(Re, 2);
    T(qe, { autocomplete: "off", get label() {
      return i.providers.config.valueMfaClaim;
    }, placeholder: "mfa", pattern: q, width: f, get value() {
      return t().mfa_claim_value;
    }, set value(o) {
      t(t().mfa_claim_value = o, true);
    } });
    var ke = a(qe, 2), Ve = s(ke);
    Zt(Ve, () => e(g), (o) => {
      var $ = cr(), w = s($);
      sr(w, { get providerId() {
        return t().id;
      } }), n($), l(o, $);
    });
    var tt = a(Ve, 2);
    const rt = ie(() => `/auth/v1/providers/${t().id}/img`);
    er(tt, { method: "PUT", get url() {
      return e(rt);
    }, fileName: "logo", onSuccess: () => u(g, h(vt())) }), n(ke);
    var ye = a(ke, 2), We = s(ye);
    Be(We, { type: "submit", get isLoading() {
      return k;
    }, children: (o, $) => {
      U();
      var w = D();
      x(() => v(w, d.common.save)), l(o, w);
    }, $$slots: { default: true } });
    var De = a(We, 2);
    {
      var Ue = (o) => {
        yt(o, {});
      };
      S(De, (o) => {
        e(A) && o(Ue);
      });
    }
    var at = a(De, 2);
    {
      var Ye = (o) => {
        var $ = ur(), w = s($, true);
        n($), x(() => v(w, e(p))), l(o, $);
      };
      S(at, (o) => {
        e(p) && o(Ye);
      });
    }
    n(ye), x(() => {
      v(fe, i.providers.config.descScope), v(he, i.providers.config.descClientName), v(Ze, i.providers.config.descClientId), v(et, i.providers.config.descClientSecret), v(Me, i.providers.config.mapUser), v(Oe, i.providers.config.mapMfa);
    }), l(m, C);
  }, $$slots: { default: true } }), n(K), l(O, K), Ce();
}
var mr = I('<div class="user svelte-1vu6qeb"> <span class="muted font-mono svelte-1vu6qeb"> </span></div>'), fr = I('<div class="forceDelete svelte-1vu6qeb"> </div>'), hr = I('<p class="err"><b> </b></p> <p> </p> <!> <p> </p> <!> <!>', 1), gr = I("<p> </p> <!>", 1), br = I('<div class="err"> </div>'), kr = I('<div class="container svelte-1vu6qeb"><!> <!></div>');
function $r(O, N) {
  we(N, true);
  let t = ht(), d = Ae(), i = R(true), f = R(""), k = R(false), p = R(h([]));
  Pt(async () => {
    let m = await Vt(`/auth/v1/providers/${N.provider.id}/delete_safe`);
    m.status === 406 && m.error && u(p, h(m.error)), u(i, false);
  });
  async function A() {
    u(f, ""), u(i, true);
    let m = await Wt(`/auth/v1/providers/${N.provider.id}`);
    m.error ? u(f, h(m.error.message)) : N.onSave(), u(i, false);
  }
  var L = kr(), g = s(L);
  {
    var r = (m) => {
      var P = hr(), C = H(P), y = s(C), V = s(y, true);
      n(y), n(C);
      var J = a(C, 2), W = s(J, true);
      n(J);
      var z = a(J, 2);
      tr(z, { summary: (j) => {
        U();
        var _ = D();
        x(() => v(_, d.providers.delete.linkedUsers)), l(j, _);
      }, details: (j) => {
        var _ = He(), Ie = H(_);
        Et(Ie, 17, () => e(p), (le) => le.id, (le, fe) => {
          var ve = mr(), _e = s(ve), he = a(_e), ge = s(he);
          n(he), n(ve), x(() => {
            v(_e, `${e(fe).email ?? ""} `), v(ge, `/ ${e(fe).id ?? ""}`);
          }), l(le, ve);
        }), l(j, _);
      }, $$slots: { summary: true, details: true } });
      var re = a(z, 2), ue = s(re, true);
      n(re);
      var se = a(re, 2);
      te(se, { get ariaLabel() {
        return d.providers.delete.forceDelete;
      }, get checked() {
        return e(k);
      }, set checked(Q) {
        u(k, h(Q));
      }, children: (Q, B) => {
        var j = fr(), _ = s(j, true);
        n(j), x(() => v(_, d.providers.delete.forceDelete)), l(Q, j);
      }, $$slots: { default: true } });
      var xe = a(se, 2);
      {
        var me = (Q) => {
          Be(Q, { level: -1, onclick: A, get isLoading() {
            return e(i);
          }, children: (B, j) => {
            U();
            var _ = D();
            x(() => v(_, t.common.delete)), l(B, _);
          }, $$slots: { default: true } });
        };
        S(xe, (Q) => {
          e(k) && Q(me);
        });
      }
      x(() => {
        v(V, d.providers.delete.isInUse1), v(W, d.providers.delete.isInUse2), v(ue, d.providers.delete.areYouSure);
      }), l(m, P);
    }, K = (m) => {
      var P = gr(), C = H(P), y = s(C, true);
      n(C);
      var V = a(C, 2);
      Be(V, { level: -1, onclick: A, get isLoading() {
        return e(i);
      }, children: (J, W) => {
        U();
        var z = D();
        x(() => v(z, t.common.delete)), l(J, z);
      }, $$slots: { default: true } }), x(() => v(y, d.providers.delete.areYouSure)), l(m, P);
    };
    S(g, (m) => {
      e(p).length > 0 ? m(r) : m(K, false);
    });
  }
  var b = a(g, 2);
  {
    var M = (m) => {
      var P = br(), C = s(P, true);
      n(P), x(() => v(C, e(f))), l(m, P);
    };
    S(b, (m) => {
      e(f) && m(M);
    });
  }
  n(L), l(O, L), Ce();
}
var wr = I('<div class="flex"><!></div> <!>', 1);
function Cr(O, N) {
  we(N, true);
  let t = ut(N, "provider", 15), d = ut(N, "onSave", 15), i = Ae();
  const f = [i.tabs.config, i.tabs.delete];
  let k = R(h(f[0]));
  var p = wr(), A = H(p), L = s(A);
  rr(L, { tabs: f, get selected() {
    return e(k);
  }, set selected(b) {
    u(k, h(b));
  } }), n(A);
  var g = a(A, 2);
  {
    var r = (b) => {
      pr(b, { get provider() {
        return t();
      }, set provider(M) {
        t(M);
      }, get onSave() {
        return d();
      }, set onSave(M) {
        d(M);
      } });
    }, K = (b) => {
      var M = He(), m = H(M);
      {
        var P = (C) => {
          $r(C, { get provider() {
            return t();
          }, get onSave() {
            return d();
          }, set onSave(y) {
            d(y);
          } });
        };
        S(m, (C) => {
          e(k) === i.tabs.delete && C(P);
        }, true);
      }
      l(b, M);
    };
    S(g, (b) => {
      e(k) === i.tabs.config ? b(r) : b(K, false);
    });
  }
  l(O, p), Ce();
}
var xr = I("<div><!></div>"), Ir = I('<div class="checkbox svelte-f8e9ia"><!></div>'), Pr = I('<div class="checkbox svelte-f8e9ia"><!></div> <!>', 1), Er = I('<div class="ml mb"><!></div>'), yr = I('<div class="err"> </div>'), Tr = I('<!> <!> <!> <!> <!> <div class="checkbox svelte-f8e9ia"><!></div> <p class="desc svelte-f8e9ia"> </p> <!> <p class="desc svelte-f8e9ia"> </p> <!> <p class="desc svelte-f8e9ia"> </p> <!> <p class="desc svelte-f8e9ia"> </p> <!> <p><!></p> <div class="checkbox svelte-f8e9ia"><!></div> <div class="checkbox svelte-f8e9ia"><!></div> <!> <!> <p class="desc svelte-f8e9ia"> </p> <!> <!> <p class="desc svelte-f8e9ia"> </p> <!> <!>', 1), Ar = I('<div class="err"> </div>'), Lr = I('<!> <!> <div class="flex gap-05"><!> <!> <!> <!></div>', 1), Sr = I('<div class="container svelte-f8e9ia"><!> <div></div> <!></div>');
function Nr(O, N) {
  we(N, true);
  let t = ht(), d = Ae();
  const i = "min(calc(100dvw - 1.75rem), 30rem)";
  let f = R(false), k = R(""), p = R(false), A = R(false), L = R(false), g = R(h({ issuer: "", metadata_url: "", danger_allow_insecure: false, root_pem: "" })), r = R(h({ enabled: true, typ: "oidc", issuer: "", danger_allow_insecure: false, authorization_endpoint: "", token_endpoint: "", token_auth_method_basic: false, userinfo_endpoint: "", use_pkce: true, client_secret_basic: true, client_secret_post: false, name: "", client_id: "", client_secret: "", scope: "", root_pem: "", admin_claim_path: "", admin_claim_value: "", mfa_claim_path: "", mfa_claim_value: "" })), K = ["OIDC", "Auto", "Custom", "Github", "Google"], b = R(h(K[0])), M = ie(() => e(b) === "Auto"), m = ie(() => e(b) === "Custom"), P = ie(() => e(b) === "OIDC"), C = ie(() => !e(M) && !e(m) && !e(P)), y = ie(() => !(e(C) || e(m) || e(p)));
  const V = "/auth/v1/providers/lookup";
  let J = ie(() => e(y) ? V : "/auth/v1/providers/create");
  Ge(() => {
    if (e(b)) switch (u(p, false), u(g, h({ issuer: "", metadata_url: "", danger_allow_insecure: false, root_pem: "" })), e(b)) {
      case "Github":
        u(r, h({ enabled: true, issuer: "github.com", typ: "github", danger_allow_insecure: false, authorization_endpoint: "https://github.com/login/oauth/authorize", token_endpoint: "https://github.com/login/oauth/access_token", client_secret_basic: true, client_secret_post: true, userinfo_endpoint: "https://api.github.com/user", use_pkce: false, name: "Github", client_id: "", client_secret: "", scope: "user:email", root_pem: "", admin_claim_path: "", admin_claim_value: "", mfa_claim_path: "$.two_factor_authentication", mfa_claim_value: "true" }));
        break;
      case "Google":
        u(g, h({ issuer: "accounts.google.com", metadata_url: "", danger_allow_insecure: false, root_pem: "" })), Kt(() => {
          re(V);
        });
        break;
      default:
        u(r, h({ enabled: true, issuer: "", typ: "oidc", danger_allow_insecure: false, authorization_endpoint: "", token_endpoint: "", client_secret_basic: true, client_secret_post: false, userinfo_endpoint: "", use_pkce: true, name: "", client_id: "", client_secret: "", scope: "", root_pem: "", admin_claim_path: "", admin_claim_value: "", mfa_claim_path: "", mfa_claim_value: "" }));
    }
  }), Ge(() => {
    e(A) && setTimeout(() => {
      N.onSave(), ue();
    }, 1500);
  });
  async function W(B, j) {
    e(y) ? await re(B.action) : await z(B.action);
  }
  async function z(B) {
    if (e(r).client_secret && !(e(r).client_secret_basic || e(r).client_secret_post)) {
      u(k, h(d.providers.config.errNoAuthMethod));
      return;
    }
    if (!e(r).use_pkce && !e(r).client_secret) {
      u(k, h(d.providers.config.errConfidential));
      return;
    }
    u(k, ""), u(f, true);
    let j = { name: e(r).name, typ: e(M) ? "custom" : e(b).toLowerCase(), enabled: e(r).enabled, issuer: e(r).issuer, authorization_endpoint: e(r).authorization_endpoint, token_endpoint: e(r).token_endpoint, userinfo_endpoint: e(r).userinfo_endpoint, danger_allow_insecure: e(L) && e(r).root_pem ? false : e(r).danger_allow_insecure, use_pkce: e(r).use_pkce, client_secret_basic: e(r).client_secret_basic, client_secret_post: e(r).client_secret_post, client_id: e(r).client_id, client_secret: e(r).client_secret, scope: e(r).scope.trim(), root_pem: e(L) && e(r).root_pem ? e(r).root_pem.trim() : void 0, admin_claim_path: e(r).admin_claim_path || void 0, admin_claim_value: e(r).admin_claim_value || void 0, mfa_claim_path: e(r).mfa_claim_path || void 0, mfa_claim_value: e(r).mfa_claim_value || void 0 }, _ = await ct(B, j);
    _.error ? _.error.message.includes("InvalidCertificate") ? u(k, "Insecure connection not allowed") : u(k, h(_.error.message)) : u(A, true), u(f, false);
  }
  async function re(B) {
    if (!e(g).issuer && !e(g).metadata_url) {
      u(k, "Provide at least one of Issuer / Metadata URL");
      return;
    }
    u(k, ""), u(f, true);
    let j = { issuer: e(g).issuer || void 0, metadata_url: e(g).metadata_url || void 0, danger_allow_insecure: e(g).danger_allow_insecure, root_pem: e(g).root_pem || void 0 }, _ = await ct(B, j);
    _.body ? (e(r).issuer = _.body.issuer, e(r).authorization_endpoint = _.body.authorization_endpoint, e(r).danger_allow_insecure = _.body.danger_allow_insecure, e(r).token_endpoint = _.body.token_endpoint, e(r).userinfo_endpoint = _.body.userinfo_endpoint, e(r).client_secret_basic = _.body.client_secret_basic, e(r).client_secret_post = _.body.client_secret_post, e(r).use_pkce = _.body.use_pkce, e(r).client_secret_basic = _.body.client_secret_basic, e(r).client_secret_post = !_.body.client_secret_basic && _.body.client_secret_post, e(r).scope = _.body.scope, e(r).root_pem = _.body.root_pem, u(p, true)) : _.error && (_.error.message.includes("InvalidCertificate") ? u(k, "Insecure connection not allowed") : u(k, h(_.error.message))), u(f, false);
  }
  function ue() {
    u(g, h({ issuer: "", metadata_url: "", danger_allow_insecure: false, root_pem: "" })), u(r, h({ enabled: true, issuer: "", typ: "oidc", danger_allow_insecure: false, authorization_endpoint: "", token_endpoint: "", userinfo_endpoint: "", use_pkce: true, name: "", client_id: "", client_secret_basic: true, client_secret_post: false, scope: "", root_pem: "", admin_claim_path: "", admin_claim_value: "", mfa_claim_path: "", mfa_claim_value: "" })), u(A, false), u(p, false), u(L, false);
  }
  var se = Sr(), xe = s(se);
  ar(xe, { ariaLabel: "Select Mode", options: K, get value() {
    return e(b);
  }, set value(B) {
    u(b, h(B));
  } });
  var me = a(xe, 2);
  ft(me, "height", ".5rem");
  var Q = a(me, 2);
  At(Q, { get action() {
    return e(J);
  }, onSubmit: W, children: (B, j) => {
    var _ = Lr(), Ie = H(_);
    {
      var le = (E) => {
        var F = Pr(), X = H(F), ne = s(X);
        te(ne, { get ariaLabel() {
          return d.providers.config.custRootCa;
        }, get checked() {
          return e(L);
        }, set checked(Y) {
          u(L, h(Y));
        }, children: (Y, Z) => {
          U();
          var ee = D();
          x(() => v(ee, d.providers.config.custRootCa)), l(Y, ee);
        }, $$slots: { default: true } }), n(X);
        var Ne = a(X, 2);
        {
          var ae = (Y) => {
            var Z = xr(), ee = s(Z);
            _t(ee, { rows: 15, name: "rootPem", get label() {
              return d.providers.config.rootPemCert;
            }, placeholder: `-----BEGIN CERTIFICATE-----
...
-----END CERTIFICATE-----`, errMsg: "-----BEGIN CERTIFICATE----- ...", width: "min(40rem, calc(100dvw - 1.75rem))", fontMono: true, pattern: mt, get value() {
              return e(g).root_pem;
            }, set value(oe) {
              e(g).root_pem = oe;
            } }), n(Z), Qe(3, Z, () => Xe, () => ({ duration: 150 })), l(Y, Z);
          }, pe = (Y) => {
            var Z = Ir(), ee = s(Z);
            te(ee, { get ariaLabel() {
              return d.providers.config.allowInsecureTls;
            }, get checked() {
              return e(g).danger_allow_insecure;
            }, set checked(oe) {
              e(g).danger_allow_insecure = oe;
            }, children: (oe, Me) => {
              U();
              var be = D();
              x(() => v(be, d.providers.config.allowInsecureTls)), l(oe, be);
            }, $$slots: { default: true } }), n(Z), l(Y, Z);
          };
          S(Ne, (Y) => {
            e(L) ? Y(ae) : Y(pe, false);
          });
        }
        l(E, F);
      };
      S(Ie, (E) => {
        e(p) || E(le);
      });
    }
    var fe = a(Ie, 2);
    {
      var ve = (E) => {
        T(E, { name: "issuer", label: "Issuer URL", placeholder: "Issuer URL", width: i, required: true, pattern: q, get value() {
          return e(g).issuer;
        }, set value(F) {
          e(g).issuer = F;
        } });
      }, _e = (E) => {
        var F = He(), X = H(F);
        {
          var ne = (ae) => {
            T(ae, { typ: "url", name: "metadata", label: "Metadata URL", placeholder: ".../.well-known/openid-configuration", width: i, required: true, pattern: q, get value() {
              return e(g).metadata_url;
            }, set value(pe) {
              e(g).metadata_url = pe;
            } });
          }, Ne = (ae) => {
            var pe = He(), Y = H(pe);
            {
              var Z = (ee) => {
                var oe = Tr(), Me = H(oe);
                {
                  var be = (c) => {
                    _t(c, { rows: 17, name: "rootPem", get label() {
                      return d.providers.config.rootPemCert;
                    }, placeholder: `-----BEGIN CERTIFICATE-----
...
-----END CERTIFICATE-----`, get disabled() {
                      return e(p);
                    }, errMsg: "-----BEGIN CERTIFICATE----- ...", width: "min(40rem, calc(100dvw - 1.75rem))", fontMono: true, pattern: mt, get value() {
                      return e(r).root_pem;
                    }, set value(G) {
                      e(r).root_pem = G;
                    } });
                  }, Ke = (c) => {
                    var G = Er(), ce = s(G);
                    {
                      var dt = (Te) => {
                        pt(Te, { get label() {
                          return d.providers.config.allowInsecureTls;
                        }, children: (Fe, jt) => {
                          It(Fe, {});
                        } });
                      }, Bt = (Te) => {
                        te(Te, { get ariaLabel() {
                          return d.providers.config.allowInsecureTls;
                        }, get checked() {
                          return e(r).danger_allow_insecure;
                        }, set checked(Fe) {
                          e(r).danger_allow_insecure = Fe;
                        }, children: (Fe, jt) => {
                          U();
                          var xt = D();
                          x(() => v(xt, d.providers.config.allowInsecureTls)), l(Fe, xt);
                        }, $$slots: { default: true } });
                      };
                      S(ce, (Te) => {
                        e(p) ? Te(dt) : Te(Bt, false);
                      });
                    }
                    n(G), l(c, G);
                  };
                  S(Me, (c) => {
                    e(L) ? c(be) : c(Ke, false);
                  });
                }
                var Ee = a(Me, 2);
                T(Ee, { name: "issuer", label: "Issuer URL", placeholder: "Issuer URL", width: i, get disabled() {
                  return e(p);
                }, required: true, pattern: q, get value() {
                  return e(r).issuer;
                }, set value(c) {
                  e(r).issuer = c;
                } });
                var Oe = a(Ee, 2);
                T(Oe, { typ: "url", name: "auth_endpoint", label: "Authorization Endpoint", placeholder: "Authorization Endpoint", width: i, get disabled() {
                  return e(p);
                }, required: true, pattern: q, get value() {
                  return e(r).authorization_endpoint;
                }, set value(c) {
                  e(r).authorization_endpoint = c;
                } });
                var Re = a(Oe, 2);
                T(Re, { typ: "url", name: "token_endpoint", label: "Token Endpoint", placeholder: "Token Endpoint", width: i, get disabled() {
                  return e(p);
                }, required: true, pattern: q, get value() {
                  return e(r).token_endpoint;
                }, set value(c) {
                  e(r).token_endpoint = c;
                } });
                var qe = a(Re, 2);
                T(qe, { typ: "url", name: "userinfo_endpoint", label: "Userinfo Endpoint", placeholder: "Userinfo Endpoint", width: i, get disabled() {
                  return e(p);
                }, required: true, pattern: q, get value() {
                  return e(r).userinfo_endpoint;
                }, set value(c) {
                  e(r).userinfo_endpoint = c;
                } });
                var ke = a(qe, 2), Ve = s(ke);
                {
                  var tt = (c) => {
                    pt(c, { label: "PKCE", children: (G, ce) => {
                      It(G, {});
                    } });
                  }, rt = (c) => {
                    te(c, { ariaLabel: "PKCE", get checked() {
                      return e(r).use_pkce;
                    }, set checked(G) {
                      e(r).use_pkce = G;
                    }, children: (G, ce) => {
                      U();
                      var dt = D("PKCE");
                      l(G, dt);
                    }, $$slots: { default: true } });
                  };
                  S(Ve, (c) => {
                    e(p) ? c(tt) : c(rt, false);
                  });
                }
                n(ke);
                var ye = a(ke, 2), We = s(ye, true);
                n(ye);
                var De = a(ye, 2);
                T(De, { name: "scope", label: "Scope", placeholder: "openid profile email", width: i, required: true, pattern: Lt, get value() {
                  return e(r).scope;
                }, set value(c) {
                  e(r).scope = c;
                } });
                var Ue = a(De, 2), at = s(Ue, true);
                n(Ue);
                var Ye = a(Ue, 2);
                T(Ye, { name: "client_name", label: "Client Name", placeholder: "Client Name", width: i, required: true, pattern: St, get value() {
                  return e(r).name;
                }, set value(c) {
                  e(r).name = c;
                } });
                var o = a(Ye, 2), $ = s(o, true);
                n(o);
                var w = a(o, 2);
                T(w, { name: "client_id", autocomplete: "off", label: "Client ID", placeholder: "Client ID", width: i, required: true, pattern: q, get value() {
                  return e(r).client_id;
                }, set value(c) {
                  e(r).client_id = c;
                } });
                var de = a(w, 2), gt = s(de, true);
                n(de);
                var ze = a(de, 2);
                const Mt = ie(() => !e(r).use_pkce);
                Tt(ze, { name: "client_secret", autocomplete: "off", label: "Client Secret", placeholder: "Client Secret", width: i, get errMsg() {
                  return d.providers.config.errConfidential;
                }, maxLength: 256, get required() {
                  return e(Mt);
                }, get value() {
                  return e(r).client_secret;
                }, set value(c) {
                  e(r).client_secret = c;
                } });
                var ot = a(ze, 2), Rt = s(ot);
                $e(Rt, () => d.providers.config.descAuthMethod), n(ot);
                var it = a(ot, 2), qt = s(it);
                te(qt, { ariaLabel: "client_secret_basic", get checked() {
                  return e(r).client_secret_basic;
                }, set checked(c) {
                  e(r).client_secret_basic = c;
                }, children: (c, G) => {
                  U();
                  var ce = D("client_secret_basic");
                  l(c, ce);
                }, $$slots: { default: true } }), n(it);
                var st = a(it, 2), Dt = s(st);
                te(Dt, { ariaLabel: "client_secret_post", get checked() {
                  return e(r).client_secret_post;
                }, set checked(c) {
                  e(r).client_secret_post = c;
                }, children: (c, G) => {
                  U();
                  var ce = D("client_secret_post");
                  l(c, ce);
                }, $$slots: { default: true } }), n(st);
                var bt = a(st, 2);
                {
                  var Ut = (c) => {
                    var G = yr(), ce = s(G, true);
                    n(G), x(() => v(ce, d.providers.config.errNoAuthMethod)), Qe(3, G, () => Xe, () => ({ duration: 150 })), l(c, G);
                  };
                  S(bt, (c) => {
                    !e(r).use_pkce && !e(r).client_secret_basic && !e(r).client_secret_post && c(Ut);
                  });
                }
                var kt = a(bt, 2);
                Nt(kt, {});
                var nt = a(kt, 2), zt = s(nt, true);
                n(nt);
                var $t = a(nt, 2);
                T($t, { name: "admin_claim_path", get label() {
                  return d.providers.config.pathAdminClaim;
                }, placeholder: "$.roles.*", width: i, pattern: q, get value() {
                  return e(r).admin_claim_path;
                }, set value(c) {
                  e(r).admin_claim_path = c;
                } });
                var wt = a($t, 2);
                T(wt, { name: "admin_claim_value", get label() {
                  return d.providers.config.valueAdminClaim;
                }, placeholder: "rauthy_admin", width: i, pattern: q, get value() {
                  return e(r).admin_claim_value;
                }, set value(c) {
                  e(r).admin_claim_value = c;
                } });
                var lt = a(wt, 2), Ft = s(lt, true);
                n(lt);
                var Ct = a(lt, 2);
                T(Ct, { name: "mfa_claim_path", get label() {
                  return d.providers.config.pathMfaClaim;
                }, placeholder: "$.amr.*", width: i, pattern: q, get value() {
                  return e(r).mfa_claim_path;
                }, set value(c) {
                  e(r).mfa_claim_path = c;
                } });
                var Gt = a(Ct, 2);
                T(Gt, { name: "mfa_claim_value", get label() {
                  return d.providers.config.valueMfaClaim;
                }, placeholder: "mfa", width: i, pattern: q, get value() {
                  return e(r).mfa_claim_value;
                }, set value(c) {
                  e(r).mfa_claim_value = c;
                } }), x(() => {
                  v(We, d.providers.config.descScope), v(at, d.providers.config.descClientName), v($, d.providers.config.descClientId), v(gt, d.providers.config.descClientId), v(zt, d.providers.config.mapUser), v(Ft, d.providers.config.mapMfa);
                }), l(ee, oe);
              };
              S(Y, (ee) => {
                (e(C) || e(m) || e(p)) && ee(Z);
              }, true);
            }
            l(ae, pe);
          };
          S(X, (ae) => {
            e(M) && !e(p) ? ae(ne) : ae(Ne, false);
          }, true);
        }
        l(E, F);
      };
      S(fe, (E) => {
        e(P) && !e(p) ? E(ve) : E(_e, false);
      });
    }
    var he = a(fe, 2), ge = s(he);
    Be(ge, { type: "submit", get isLoading() {
      return e(f);
    }, children: (E, F) => {
      U();
      var X = D();
      x(() => v(X, e(y) ? d.providers.config.lookup : t.common.save)), l(E, X);
    }, $$slots: { default: true } });
    var Pe = a(ge, 2);
    {
      var Ze = (E) => {
        Be(E, { level: 3, onclick: ue, get isLoading() {
          return e(f);
        }, children: (F, X) => {
          U();
          var ne = D();
          x(() => v(ne, d.common.reset)), l(F, ne);
        }, $$slots: { default: true } });
      };
      S(Pe, (E) => {
        e(y) || E(Ze);
      });
    }
    var Le = a(Pe, 2);
    {
      var Se = (E) => {
        yt(E, {});
      };
      S(Le, (E) => {
        e(A) && E(Se);
      });
    }
    var et = a(Le, 2);
    {
      var je = (E) => {
        var F = Ar(), X = s(F, true);
        n(F), x(() => v(X, e(k))), l(E, F);
      };
      S(et, (E) => {
        e(k) && E(je);
      });
    }
    n(he), l(B, _);
  }, $$slots: { default: true } }), n(se), l(O, se), Ce();
}
var Mr = I("<div></div> <!>", 1), Rr = I('<div class="err"> </div>'), qr = I('<div id="federation"><!></div> <!>', 1), Dr = I("<!> <!>", 1);
function Ur(O, N) {
  we(N, true);
  let t = Ae(), d = R(void 0), i = Ht("pid"), f = R(""), k = R(h([])), p = R(void 0);
  Pt(() => {
    A();
  }), Ge(() => {
    u(p, h(e(k).find((b) => b.id === i.get())));
  });
  async function A() {
    var _a;
    let b = await ct("/auth/v1/providers");
    b.body ? u(k, h(b.body)) : u(f, h(((_a = b.error) == null ? void 0 : _a.message) || "Error"));
  }
  function L() {
    var _a;
    i.set(void 0), (_a = e(d)) == null ? void 0 : _a(), A();
  }
  var g = Dr(), r = H(g);
  Jt(r, { width: "11rem", buttonTilesAriaControls: "federation", paddingTop: "6.65rem", buttonTiles: (M) => {
    var m = Mr(), P = H(m);
    ft(P, "height", ".5rem");
    var C = a(P, 2);
    Et(C, 17, () => e(k), (y) => y.id, (y, V, J, W) => {
      const z = ie(() => i.get() === e(V).id);
      Qt(y, { onclick: () => i.set(e(V).id), get selected() {
        return e(z);
      }, children: (re, ue) => {
        U();
        var se = D();
        x(() => v(se, e(V).name)), l(re, se);
      } });
    }), l(M, m);
  }, children: (M, m) => {
    const P = ie(() => e(k).length === 0 ? 1 : 2);
    Xt(M, { get level() {
      return e(P);
    }, get closeModal() {
      return e(d);
    }, set closeModal(C) {
      u(d, h(C));
    }, children: (C, y) => {
      Nr(C, { onSave: L });
    }, $$slots: { default: true } });
  }, $$slots: { buttonTiles: true, default: true } });
  var K = a(r, 2);
  Yt(K, { children: (b, M) => {
    var m = qr(), P = H(m), C = s(P);
    {
      var y = (W) => {
        Cr(W, { onSave: A, get provider() {
          return e(p);
        }, set provider(z) {
          u(p, h(z));
        } });
      };
      S(C, (W) => {
        e(p) && W(y);
      });
    }
    n(P);
    var V = a(P, 2);
    {
      var J = (W) => {
        var z = Rr(), re = s(z, true);
        n(z), x(() => v(re, e(f))), l(W, z);
      };
      S(V, (W) => {
        e(f) && W(J);
      });
    }
    x(() => Je(P, "aria-label", t.common.details)), l(b, m);
  } }), l(O, g), Ce();
}
function ga(O) {
  Ur(O, {});
}
export {
  ga as component
};
