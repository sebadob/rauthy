import { t as k, a as o, e as we, d as fe } from "../chunks/BH6NCLk-.js";
import { p as De, c as a, r as t, s as v, j as e, a9 as he, t as C, a as Ce, a7 as pe, k as T, f as j, l as n, a5 as Qe, aa as Je, a6 as Yt } from "../chunks/CvlvO1XB.js";
import { s as l, d as jt, h as Gt } from "../chunks/CTI4QPiR.js";
import { i as x } from "../chunks/BUO_AUgz.js";
import { p as h } from "../chunks/Wh68IIk2.js";
import { b as Jt } from "../chunks/7lxiEjMQ.js";
import { p as Te } from "../chunks/C6SR4G2t.js";
import { l as Ye, U as st, K as Qt, P as Xt, M as Zt, A as er, V as tr, F as rr, r as ar } from "../chunks/Brp0G0eV.js";
import { e as It } from "../chunks/BpWRzPRQ.js";
import { h as Tt } from "../chunks/i8Xqpu09.js";
import { a as F, c as O, s as je, r as sr } from "../chunks/BMbqVy6X.js";
import { C as Ge } from "../chunks/CS_Msctd.js";
import { B as xe, t as Ae, f as We, a as Et, s as vr } from "../chunks/DMkkW5Nn.js";
import { M as nr } from "../chunks/DkpnkP5j.js";
import { u as Ne } from "../chunks/CgEHB2u3.js";
import { c as $t, b as vt, d as ot, f as Ve } from "../chunks/CRjU5SuJ.js";
import { B as or } from "../chunks/DQDimnsq.js";
import { I as $e } from "../chunks/D-haFZtM.js";
import { F as ir } from "../chunks/BJ_mu-WB.js";
import { b as nt, k as lr, l as gt, m as dr } from "../chunks/BRCxk8by.js";
import { I as it } from "../chunks/Nks81rMs.js";
import { I as ur } from "../chunks/CTgOhmZ_.js";
import { u as lt } from "../chunks/CO1TlAUy.js";
import { w as cr } from "../chunks/5iTOvKgJ.js";
import { W as Dt } from "../chunks/BKOxXyr3.js";
import { U as fr, D as Ct } from "../chunks/YxbVyMQV.js";
import { P as _r } from "../chunks/DUV2YlaD.js";
import { I as at } from "../chunks/BUc4wf1T.js";
import { s as mr } from "../chunks/-T201g_q.js";
import { a as pr } from "../chunks/BhIBACXG.js";
import { A as gr } from "../chunks/8APYSIIx.js";
import { I as hr } from "../chunks/Cv55insy.js";
import { T as wr } from "../chunks/Bb2Y1h4J.js";
import { u as br } from "../chunks/rQ2Hm1Il.js";
import { T as ht } from "../chunks/_OE2Cq0B.js";
import { M as yr } from "../chunks/BbRujPvb.js";
import { C as xr } from "../chunks/GWgockW8.js";
import { T as kr } from "../chunks/DX6irJfo.js";
import { L as Pr } from "../chunks/KzU4-o98.js";
var Ir = k('<div class="link-err value svelte-1tea8v4"> </div>'), Tr = k('<div class="fed-btn svelte-1tea8v4"><!> <!></div>'), Er = k('<h3> </h3> <p> </p> <div class="providers svelte-1tea8v4"></div>', 1), $r = k("<!> <!>", 1), Dr = k('<div><div> </div> <span class="value svelte-1tea8v4"> </span></div>'), Cr = k('<div><div> </div> <span class="value svelte-1tea8v4"> </span></div>'), Mr = k('<div><div>WebID:</div> <span class="value svelte-1tea8v4"><a target="_blank"><!></a></span></div>'), Nr = k('<div><div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <div><div class="value svelte-1tea8v4"> </div> <!></div></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <!></div> <div><div> </div> <!></div> <div><div> </div> <!></div> <!> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <!> <!></div>');
function wt(se, f) {
  De(f, true);
  let s = Te(f, "user", 15), r = Ne(), M = T(false), i = T(false), N = he(() => {
    var _a;
    return (_a = s().account_type) == null ? void 0 : _a.startsWith("federated");
  }), w = he(() => {
    var _a;
    return e(N) ? `${s().account_type}: ${((_a = f.authProvider) == null ? void 0 : _a.name) || ""}` : s().account_type;
  }), d = he(() => f.viewModePhone ? "rowPhone" : "row"), y = he(() => f.viewModePhone ? "labelPhone" : "label");
  function c(Y) {
    Qt(64, (J, { challenge: oe, verifier: me }) => {
      J || (localStorage.setItem(Xt, me), G(Y, oe));
    });
  }
  async function G(Y, J) {
    let oe = { email: s().email, client_id: "rauthy", redirect_uri: window.location.href, provider_id: Y, pkce_challenge: J }, me = await vt(`/auth/v1/providers/${Y}/link`, oe);
    if (me.text) {
      Zt(me.text);
      let ce = me.headers.get("location");
      ce && (window.location.href = ce);
    } else console.error(me.error);
  }
  async function P() {
    let Y = await $t("/auth/v1/providers/link");
    Y.body ? s(Y.body) : (console.error(Y.error), n(M, true));
  }
  var q = Nr(), B = a(q), Z = a(B), ve = a(Z, true);
  t(Z);
  var L = v(Z, 2), E = a(L, true);
  t(L), t(B);
  var K = v(B, 2), H = a(K), ae = a(H, true);
  t(H);
  var ee = v(H, 2), te = a(ee, true);
  t(ee), t(K);
  var S = v(K, 2), I = a(S), _ = a(I, true);
  t(I);
  var A = v(I, 2), u = a(A, true);
  t(A), t(S);
  var V = v(S, 2), re = a(V), de = a(re);
  t(re);
  var m = v(re, 2), $ = a(m, true);
  t(m), t(V);
  var g = v(V, 2), U = a(g), Q = a(U, true);
  t(U);
  var ie = v(U, 2), W = a(ie), ne = a(W, true);
  t(W);
  var le = v(W, 2);
  {
    var be = (Y) => {
      var J = Tr(), oe = a(J);
      xe(oe, { get ariaLabel() {
        return r.account.providerUnlink;
      }, level: 3, onclick: P, children: (ye, Pe) => {
        pe();
        var ze = we();
        C(() => l(ze, r.account.providerUnlink)), o(ye, ze);
      }, $$slots: { default: true } });
      var me = v(oe, 2);
      {
        var ce = (ye) => {
          var Pe = Ir(), ze = a(Pe, true);
          t(Pe), C(() => l(ze, r.account.providerUnlinkDesc)), o(ye, Pe);
        };
        x(me, (ye) => {
          e(M) && ye(ce);
        });
      }
      t(J), o(Y, J);
    }, ke = (Y) => {
      var J = fe(), oe = j(J);
      {
        var me = (ce) => {
          var ye = $r(), Pe = j(ye);
          xe(Pe, { level: 2, onclick: () => n(i, true), children: (Re, Bt) => {
            pe();
            var Se = we();
            C(() => l(Se, r.account.providerLink)), o(Re, Se);
          }, $$slots: { default: true } });
          var ze = v(Pe, 2);
          nr(ze, { get showModal() {
            return e(i);
          }, set showModal(Re) {
            n(i, h(Re));
          }, children: (Re, Bt) => {
            var Se = Er(), et = j(Se), Kt = a(et, true);
            t(et);
            var tt = v(et, 2), Ot = a(tt, true);
            t(tt);
            var mt = v(tt, 2);
            It(mt, 21, () => f.providers, (rt) => rt.id, (rt, pt) => {
              const Ht = he(() => `${r.account.providerLink}: ${e(pt).name}`);
              or(rt, { get ariaLabel() {
                return e(Ht);
              }, get provider() {
                return e(pt);
              }, onclick: c });
            }), t(mt), C(() => {
              l(Kt, r.account.providerLink), l(Ot, r.account.providerLinkDesc);
            }), o(Re, Se);
          }, $$slots: { default: true } }), o(ce, ye);
        };
        x(oe, (ce) => {
          f.providers.length > 0 && ce(me);
        }, true);
      }
      o(Y, J);
    };
    x(le, (Y) => {
      e(N) ? Y(be) : Y(ke, false);
    });
  }
  t(ie), t(g);
  var b = v(g, 2), p = a(b), z = a(p, true);
  t(p);
  var X = v(p, 2), ge = a(X, true);
  t(X), t(b);
  var R = v(b, 2), D = a(R), _e = a(D, true);
  t(D);
  var ue = v(D, 2), Ie = a(ue, true);
  t(ue), t(R);
  var Ee = v(R, 2), Me = a(Ee), Xe = a(Me, true);
  t(Me);
  var Ze = v(Me, 2);
  const dt = he(() => !!s().webauthn_user_id);
  Ge(Ze, { get checked() {
    return e(dt);
  } }), t(Ee);
  var Le = v(Ee, 2), Ue = a(Le), Mt = a(Ue, true);
  t(Ue);
  var Nt = v(Ue, 2);
  Ge(Nt, { get checked() {
    return s().enabled;
  } }), t(Le);
  var Fe = v(Le, 2), qe = a(Fe), Lt = a(qe, true);
  t(qe);
  var At = v(qe, 2);
  Ge(At, { get checked() {
    return s().email_verified;
  } }), t(Fe);
  var ut = v(Fe, 2);
  {
    var zt = (Y) => {
      var J = Dr(), oe = a(J), me = a(oe, true);
      t(oe);
      var ce = v(oe, 2), ye = a(ce, true);
      t(ce), t(J), C((Pe) => {
        F(J, 1, O(e(d)), "svelte-1tea8v4"), F(oe, 1, O(e(y)), "svelte-1tea8v4"), l(me, r.account.lastLogin), l(ye, Pe);
      }, [() => Ye(s().last_login)]), o(Y, J);
    };
    x(ut, (Y) => {
      s().last_login && Y(zt);
    });
  }
  var Be = v(ut, 2), Ke = a(Be), Wt = a(Ke, true);
  t(Ke);
  var ct = v(Ke, 2), Rt = a(ct, true);
  t(ct), t(Be);
  var Oe = v(Be, 2), He = a(Oe), St = a(He, true);
  t(He);
  var ft = v(He, 2), Vt = a(ft, true);
  t(ft), t(Oe);
  var _t = v(Oe, 2);
  {
    var Ut = (Y) => {
      var J = Cr(), oe = a(J), me = a(oe, true);
      t(oe);
      var ce = v(oe, 2), ye = a(ce, true);
      t(ce), t(J), C((Pe) => {
        F(J, 1, O(e(d)), "svelte-1tea8v4"), F(oe, 1, O(e(y)), "svelte-1tea8v4"), l(me, r.account.userExpiry), l(ye, Pe);
      }, [() => Ye(s().user_expires)]), o(Y, J);
    };
    x(_t, (Y) => {
      s().user_expires && Y(Ut);
    });
  }
  var Ft = v(_t, 2);
  {
    var qt = (Y) => {
      var J = Mr(), oe = a(J), me = v(oe, 2), ce = a(me), ye = a(ce);
      Tt(ye, () => st(s().id).replace("/auth/", "/auth/<wbr/>")), t(ce), t(me), t(J), C((Pe) => {
        F(J, 1, O(e(d)), "svelte-1tea8v4"), F(oe, 1, O(e(y)), "svelte-1tea8v4"), je(ce, "href", Pe);
      }, [() => st(s().id)]), o(Y, J);
    };
    x(Ft, (Y) => {
      f.webIdData && Y(qt);
    });
  }
  t(q), C((Y, J) => {
    F(B, 1, O(e(d)), "svelte-1tea8v4"), F(Z, 1, O(e(y)), "svelte-1tea8v4"), l(ve, r.common.email), l(E, s().email), F(K, 1, O(e(d)), "svelte-1tea8v4"), F(H, 1, O(e(y)), "svelte-1tea8v4"), l(ae, r.account.givenName), l(te, s().given_name), F(S, 1, O(e(d)), "svelte-1tea8v4"), F(I, 1, O(e(y)), "svelte-1tea8v4"), l(_, r.account.familyName), l(u, s().family_name), F(V, 1, O(e(d)), "svelte-1tea8v4"), F(re, 1, O(e(y)), "svelte-1tea8v4"), l(de, `${r.account.user ?? ""} ID`), l($, s().id), F(g, 1, O(e(d)), "svelte-1tea8v4"), F(U, 1, O(e(y)), "svelte-1tea8v4"), l(Q, r.account.accType), l(ne, e(w) || ""), F(b, 1, O(e(d)), "svelte-1tea8v4"), F(p, 1, O(e(y)), "svelte-1tea8v4"), l(z, r.account.roles), l(ge, s().roles || "None"), F(R, 1, O(e(d)), "svelte-1tea8v4"), F(D, 1, O(e(y)), "svelte-1tea8v4"), l(_e, r.account.groups), l(Ie, s().groups || "None"), F(Ee, 1, O(e(d)), "svelte-1tea8v4"), F(Me, 1, O(e(y)), "svelte-1tea8v4"), l(Xe, r.account.mfaActivated), F(Le, 1, O(e(d)), "svelte-1tea8v4"), F(Ue, 1, O(e(y)), "svelte-1tea8v4"), l(Mt, r.account.userEnabled), F(Fe, 1, O(e(d)), "svelte-1tea8v4"), F(qe, 1, O(e(y)), "svelte-1tea8v4"), l(Lt, r.account.emailVerified), F(Be, 1, O(e(d)), "svelte-1tea8v4"), F(Ke, 1, O(e(y)), "svelte-1tea8v4"), l(Wt, r.account.passwordExpiry), l(Rt, Y), F(Oe, 1, O(e(d)), "svelte-1tea8v4"), F(He, 1, O(e(y)), "svelte-1tea8v4"), l(St, r.account.userCreated), l(Vt, J);
  }, [() => s().password_expires && Ye(s().password_expires) || r.common.never, () => Ye(s().created_at)]), o(se, q), Ce();
}
var Lr = k('<div class="success svelte-4ogr3z"><!></div>'), Ar = k("<p> </p>"), zr = k('<div class="err svelte-4ogr3z"> </div>'), Wr = k('<div class="formInner svelte-4ogr3z"><div><!> <!> <!> <!></div> <div><!> <!> <!> <!> <!></div></div> <div class="bottom svelte-4ogr3z"><div><!></div> <!></div> <!>', 1), Rr = k('<div class="container svelte-4ogr3z"><!></div>');
function bt(se, f) {
  var _a;
  De(f, true);
  let s = Te(f, "user", 15);
  ((_a = s().user_values) == null ? void 0 : _a.birthdate) || s(s().user_values.birthdate = "", true);
  let r = Ne(), M = T(""), i = T(false), N = T(false);
  async function w(G, P) {
    var _a2;
    const q = P.get("email"), B = P.get("family_name") || void 0, Z = P.get("given_name") || void 0, ve = P.get("birthdate") || void 0, L = ((_a2 = P.get("phone")) == null ? void 0 : _a2.replaceAll(" ", "")) || void 0, E = P.get("street") || void 0, K = P.get("zip") || void 0, H = K ? Number.parseInt(K) : void 0, ae = P.get("city") || void 0, ee = P.get("country") || void 0;
    let te = { email: q, family_name: B, given_name: Z };
    (ve || L || E || H || ae || ee) && (te.user_values = { birthdate: ve, phone: L, street: E, zip: H, city: ae, country: ee });
    let S = await ot(`/auth/v1/users/${s().id}/self`, te);
    if (S.body) {
      n(i, true), s(S.body);
      let I = 3e3;
      S.status === 202 && (n(N, true), I = 3e4), setTimeout(() => {
        n(i, false);
      }, I);
    } else S.error && (console.error(S.error), n(M, h(S.error.message)));
  }
  var d = Rr(), y = a(d);
  const c = he(() => `/auth/v1/users/${s().id}/self`);
  ir(y, { get action() {
    return e(c);
  }, onSubmit: w, children: (G, P) => {
    var q = Wr(), B = j(q), Z = a(B), ve = a(Z);
    $e(ve, { typ: "email", name: "email", get label() {
      return r.common.email;
    }, get placeholder() {
      return r.common.email;
    }, get value() {
      return s().email;
    }, required: true });
    var L = v(ve, 2);
    $e(L, { name: "given_name", autocomplete: "given-name", get label() {
      return r.account.givenName;
    }, get placeholder() {
      return r.account.givenName;
    }, get value() {
      return s().given_name;
    }, required: true, maxLength: 32, pattern: nt });
    var E = v(L, 2);
    $e(E, { name: "family_name", autocomplete: "family-name", get label() {
      return r.account.familyName;
    }, get placeholder() {
      return r.account.familyName;
    }, get value() {
      return s().family_name;
    }, maxLength: 32, pattern: nt });
    var K = v(E, 2);
    ur(K, { name: "birthdate", get label() {
      return r.account.birthdate;
    }, withDelete: true, get value() {
      return s().user_values.birthdate;
    }, set value(g) {
      s(s().user_values.birthdate = g, true);
    } }), t(Z);
    var H = v(Z, 2), ae = a(H);
    $e(ae, { name: "street", autocomplete: "street-address", get label() {
      return r.account.street;
    }, get placeholder() {
      return r.account.street;
    }, get value() {
      return s().user_values.street;
    }, maxLength: 48, pattern: lr });
    var ee = v(ae, 2);
    $e(ee, { typ: "number", name: "zip", autocomplete: "postal-code", get label() {
      return r.account.zip;
    }, get placeholder() {
      return r.account.zip;
    }, get value() {
      return s().user_values.zip;
    }, min: "1000", max: "9999999" });
    var te = v(ee, 2);
    $e(te, { name: "city", autocomplete: "address-level2", get label() {
      return r.account.city;
    }, get placeholder() {
      return r.account.city;
    }, get value() {
      return s().user_values.city;
    }, maxLength: 48, pattern: gt });
    var S = v(te, 2);
    $e(S, { name: "country", autocomplete: "country", get label() {
      return r.account.country;
    }, get placeholder() {
      return r.account.country;
    }, get value() {
      return s().user_values.country;
    }, maxLength: 48, pattern: gt });
    var I = v(S, 2);
    $e(I, { name: "phone", autocomplete: "tel", get label() {
      return r.account.phone;
    }, get placeholder() {
      return r.account.phone;
    }, get value() {
      return s().user_values.phone;
    }, maxLength: 32, pattern: dr }), t(H), t(B);
    var _ = v(B, 2), A = a(_), u = a(A);
    xe(u, { type: "submit", children: (g, U) => {
      pe();
      var Q = we();
      C(() => l(Q, r.common.save)), o(g, Q);
    }, $$slots: { default: true } }), t(A);
    var V = v(A, 2);
    {
      var re = (g) => {
        var U = Lr(), Q = a(U);
        it(Q, {}), t(U), Ae(3, U, () => We), o(g, U);
      };
      x(V, (g) => {
        e(i) && g(re);
      });
    }
    t(_);
    var de = v(_, 2);
    {
      var m = (g) => {
        var U = Ar(), Q = a(U, true);
        t(U), C(() => l(Q, r.account.emailUpdateConfirm)), o(g, U);
      }, $ = (g) => {
        var U = fe(), Q = j(U);
        {
          var ie = (W) => {
            var ne = zr(), le = a(ne, true);
            t(ne), C(() => l(le, e(M))), Ae(3, ne, () => We), o(W, ne);
          };
          x(Q, (W) => {
            e(M) && W(ie);
          }, true);
        }
        o(g, U);
      };
      x(de, (g) => {
        e(N) ? g(m) : g($, false);
      });
    }
    o(G, q);
  }, $$slots: { default: true } }), t(d), o(se, d), Ce();
}
var Sr = k('<div class="err svelte-16mlupu"><b>Your browser does not support Webauthn credentials and must be updated.</b></div>'), Vr = k('<!> <div class="regBtns svelte-16mlupu"><!> <!></div>', 1), Ur = k('<div class="regNewBtn svelte-16mlupu"><!></div>'), Fr = k('<div class="keysHeader svelte-16mlupu"> </div>'), qr = k('<div class="button svelte-16mlupu"><!></div>'), Br = k('<!> <p class="svelte-16mlupu"> <br><br> </p> <!> <!> <div class="keysContainer svelte-16mlupu"></div> <!> <div> </div>', 1), Kr = k('<div class="container svelte-16mlupu"><!></div>');
function yt(se, f) {
  De(f, true);
  const s = "credentials" in navigator;
  let r = Ne(), M = lt("account"), i = he(() => {
    var _a;
    return (_a = M.get()) == null ? void 0 : _a.user_id;
  }), N = T(void 0), w = T(false), d = T(""), y = T(false), c = T(f.user.account_type === "password"), G = T(void 0), P = T(""), q = T(false), B = T(h([]));
  Qe(() => {
    ve();
  }), Je(() => {
    e(B).length > 0 && f.user.account_type === "passkey" && n(c, e(B).length > 1);
  }), Je(() => {
    var _a;
    (_a = e(N)) == null ? void 0 : _a.focus();
  });
  function Z() {
    n(w, false), n(d, "");
  }
  async function ve() {
    var _a;
    let I = await Ve(`/auth/v1/users/${(_a = M.get()) == null ? void 0 : _a.user_id}/webauthn`);
    I.body ? n(B, h(I.body)) : console.error(I.error);
  }
  async function L() {
    if (Z(), e(q) || !e(i)) return;
    if (e(P).length < 1) {
      n(w, true), n(d, h(r.mfa.passkeyNameErr));
      return;
    }
    let I = await cr(e(i), e(P), r.authorize.invalidKeyUsed, r.authorize.requestExpired);
    I.error ? (n(w, true), n(d, `${r.mfa.errorReg} - ${I.error}`)) : (n(y, false), n(P, ""), await ve());
  }
  async function E(I) {
    var _a;
    let _ = await $t(`/auth/v1/users/${f.user.id}/webauthn/delete/${I}`);
    _.status === 200 ? await ve() : n(d, h(((_a = _.error) == null ? void 0 : _a.message) || "Error"));
  }
  function K(I) {
    n(G, void 0), n(w, true), n(d, h(I)), setTimeout(() => {
      n(w, false), n(d, "");
    }, 5e3);
  }
  function H(I) {
    n(G, void 0), n(d, h(r.mfa.testSuccess)), setTimeout(() => {
      n(d, "");
    }, 3e3);
  }
  var ae = Kr(), ee = a(ae);
  {
    var te = (I) => {
      var _ = Sr();
      o(I, _);
    }, S = (I) => {
      var _ = Br(), A = j(_);
      {
        var u = (b) => {
          Dt(b, { get userId() {
            return f.user.id;
          }, get purpose() {
            return e(G);
          }, onSuccess: H, onError: K });
        };
        x(A, (b) => {
          e(G) && b(u);
        });
      }
      var V = v(A, 2), re = a(V), de = v(re, 3);
      t(V);
      var m = v(V, 2);
      {
        var $ = (b) => {
          var p = Vr(), z = j(p);
          $e(z, { autocomplete: "off", get label() {
            return r.mfa.passkeyName;
          }, get placeholder() {
            return r.mfa.passkeyName;
          }, maxLength: 32, pattern: nt, onEnter: L, get ref() {
            return e(N);
          }, set ref(D) {
            n(N, h(D));
          }, get value() {
            return e(P);
          }, set value(D) {
            n(P, h(D));
          }, get isError() {
            return e(q);
          }, set isError(D) {
            n(q, h(D));
          } });
          var X = v(z, 2), ge = a(X);
          xe(ge, { onclick: L, children: (D, _e) => {
            pe();
            var ue = we();
            C(() => l(ue, r.mfa.register)), o(D, ue);
          }, $$slots: { default: true } });
          var R = v(ge, 2);
          xe(R, { level: 3, onclick: () => n(y, false), children: (D, _e) => {
            pe();
            var ue = we();
            C(() => l(ue, r.common.cancel)), o(D, ue);
          }, $$slots: { default: true } }), t(X), o(b, p);
        }, g = (b) => {
          var p = Ur(), z = a(p);
          const X = he(() => e(B).length === 0 ? 1 : 2);
          xe(z, { get level() {
            return e(X);
          }, onclick: () => n(y, true), children: (ge, R) => {
            pe();
            var D = we();
            C(() => l(D, r.mfa.registerNew)), o(ge, D);
          }, $$slots: { default: true } }), t(p), o(b, p);
        };
        x(m, (b) => {
          e(y) ? b($) : b(g, false);
        });
      }
      var U = v(m, 2);
      {
        var Q = (b) => {
          var p = Fr(), z = a(p, true);
          t(p), C(() => l(z, r.mfa.registerdKeys)), o(b, p);
        };
        x(U, (b) => {
          e(B).length > 0 && b(Q);
        });
      }
      var ie = v(U, 2);
      It(ie, 21, () => e(B), (b) => b.name, (b, p) => {
        fr(b, { get passkey() {
          return e(p);
        }, get showDelete() {
          return e(c);
        }, onDelete: E });
      }), t(ie);
      var W = v(ie, 2);
      {
        var ne = (b) => {
          var p = qr(), z = a(p);
          xe(z, { onclick: () => n(G, "Test"), children: (X, ge) => {
            pe();
            var R = we();
            C(() => l(R, r.mfa.test)), o(X, R);
          }, $$slots: { default: true } }), t(p), o(b, p);
        };
        x(W, (b) => {
          e(B).length > 0 && b(ne);
        });
      }
      var le = v(W, 2);
      let be;
      var ke = a(le, true);
      t(le), C(() => {
        l(re, `${r.mfa.p1 ?? ""} `), l(de, ` ${r.mfa.p2 ?? ""}`), be = F(le, 1, "svelte-16mlupu", null, be, { success: !e(w), err: e(w) }), l(ke, e(d));
      }), o(I, _);
    };
    x(ee, (I) => {
      s ? I(S, false) : I(te);
    });
  }
  t(ae), o(se, ae), Ce();
}
var Or = k('<div class="container svelte-ue7bk2"><!> <!> <!> <!> <div class="btn svelte-ue7bk2"><!></div> <div class="err svelte-ue7bk2"> </div></div>');
function Hr(se, f) {
  De(f, true);
  let s = Te(f, "passwords", 15), r = Te(f, "hideCurrentPassword", 3, false), M = Te(f, "isValid", 15), i = Ne();
  M(P);
  let N = T(false), w = T(""), d = T(void 0), y = he(() => {
    var _a;
    return ((_a = s().new) == null ? void 0 : _a.length) > 6 && s().new === s().newConfirm;
  }), c = T(void 0), G = T(void 0);
  Qe(async () => {
    let L = await Ve("/auth/v1/password_policy");
    L.body ? n(d, h(L.body)) : console.error(L.error);
  });
  function P() {
    return n(w, ""), !r() && !s().current ? (n(w, h(i.account.passwordCurrReq)), false) : s().new ? s().newConfirm ? e(N) ? s().new.length > 256 ? (n(w, "max 256"), false) : s().new !== s().newConfirm ? (n(w, h(i.account.passwordNoMatch)), false) : true : (n(w, h(i.account.passwordPolicyFollow)), false) : (n(w, h(i.account.passwordNewReq)), false) : (n(w, h(i.account.passwordNewReq)), false);
  }
  function q() {
    if (n(w, ""), e(d)) {
      let L = er(e(d));
      s(s().new = L, true), s(s().newConfirm = L, true);
    }
    requestAnimationFrame(() => {
      var _a, _b;
      (_a = e(c)) == null ? void 0 : _a(), (_b = e(G)) == null ? void 0 : _b();
    });
  }
  var B = fe(), Z = j(B);
  {
    var ve = (L) => {
      var E = Or(), K = a(E);
      _r(K, { get policy() {
        return e(d);
      }, get password() {
        return s().new;
      }, get accepted() {
        return e(N);
      }, set accepted(u) {
        n(N, h(u));
      } });
      var H = v(K, 2);
      {
        var ae = (u) => {
          at(u, { autocomplete: "current-password", get label() {
            return i.account.passwordCurr;
          }, get placeholder() {
            return i.account.passwordCurr;
          }, onInput: P, get width() {
            return f.inputWidth;
          }, get value() {
            return s().current;
          }, set value(V) {
            s(s().current = V, true);
          } });
        };
        x(H, (u) => {
          r() || u(ae);
        });
      }
      var ee = v(H, 2);
      at(ee, { autocomplete: "new-password", get label() {
        return i.account.passwordNew;
      }, get placeholder() {
        return i.account.passwordNew;
      }, onInput: P, get showCopy() {
        return e(y);
      }, get width() {
        return f.inputWidth;
      }, get value() {
        return s().new;
      }, set value(u) {
        s(s().new = u, true);
      }, get reportValidity() {
        return e(c);
      }, set reportValidity(u) {
        n(c, h(u));
      } });
      var te = v(ee, 2);
      at(te, { autocomplete: "new-password", get label() {
        return i.account.passwordConfirm;
      }, get placeholder() {
        return i.account.passwordConfirm;
      }, onInput: P, get showCopy() {
        return e(y);
      }, get width() {
        return f.inputWidth;
      }, get value() {
        return s().newConfirm;
      }, set value(u) {
        s(s().newConfirm = u, true);
      }, get reportValidity() {
        return e(G);
      }, set reportValidity(u) {
        n(G, h(u));
      } });
      var S = v(te, 2), I = a(S);
      xe(I, { onclick: q, level: 2, children: (u, V) => {
        pe();
        var re = we();
        C(() => l(re, i.account.generateRandom)), o(u, re);
      }, $$slots: { default: true } }), t(S);
      var _ = v(S, 2), A = a(_, true);
      t(_), t(E), C(() => l(A, e(w))), o(L, E);
    };
    x(Z, (L) => {
      e(d) && L(ve);
    });
  }
  return o(se, B), Ce({ isPwdValid: P });
}
var Yr = k('<div class="m-05 svelte-1h4zvzu"><p> </p> <p><b> </b></p> <div></div> <p> </p> <!></div>'), jr = k("<p> </p> <p> </p> <p> </p> <div><!></div>", 1), Gr = k('<div class="success svelte-1h4zvzu"><!></div>'), Jr = k('<div class="err"> </div>'), Qr = k('<div class="cancel"><!></div>'), Xr = k('<div class="convertPasskey svelte-1h4zvzu"><h3> </h3> <p> </p> <!></div>'), Zr = k('<div><!> <div class="save svelte-1h4zvzu"><!> <!></div></div> <!>', 1), ea = k('<div class="container svelte-1h4zvzu"><!> <!> <!> <!></div>');
function xt(se, f) {
  De(f, true);
  let s = Te(f, "user", 15), r = Ne(), M = he(() => f.viewModePhone ? "calc(100vw - 1.5rem)" : "300px"), i = T(h(s().account_type)), N = T(h([])), w = T(void 0), d = T(false), y = T(false), c = T(""), G = T(false), P = T(void 0), q = T(h({ current: "", new: "", newConfirm: "" })), B = he(() => e(N).filter((m) => m.user_verified).length > 0);
  Qe(() => {
    Z();
  });
  async function Z() {
    let m = await Ve(`/auth/v1/users/${s().id}/webauthn`);
    m.body ? n(N, h(m.body)) : console.error("error fetching passkeys: " + m.error);
  }
  async function ve() {
    let m = await vt(`/auth/v1/users/${s().id}/self/convert_passkey`);
    m.error ? console.error("error fetching passkeys: " + m.error) : window.location.reload();
  }
  async function L() {
    n(c, ""), e(N).length > 0 ? await E() : await K();
  }
  async function E() {
    var _a;
    if (!((_a = e(w)) == null ? void 0 : _a())) {
      n(c, h(r.common.invalidInput));
      return;
    }
    n(P, "PasswordNew");
  }
  async function K(m) {
    var _a, _b;
    if (!((_a = e(w)) == null ? void 0 : _a())) {
      n(c, h(r.common.invalidInput));
      return;
    }
    n(y, true);
    let $ = { password_new: e(q).new };
    m ? $.mfa_code = m : $.password_current = e(q).current;
    let g = await ot(`/auth/v1/users/${s().id}/self`, $);
    g.body ? (n(G, true), n(q, h({ current: "", new: "", newConfirm: "" })), s(g.body), n(i, h(g.body.account_type)), setTimeout(() => {
      n(G, false), n(d, false);
    }, 3e3)) : n(c, h(((_b = g.error) == null ? void 0 : _b.message) || "Error")), n(y, false);
  }
  function H(m) {
    n(P, void 0), n(c, h(m)), setTimeout(() => {
      n(c, "");
    }, 5e3);
  }
  function ae(m) {
    n(P, void 0), m && "code" in m && K(m.code);
  }
  async function ee() {
    let m = { email: s().email }, $ = await vt("/auth/v1/users/request_reset", m);
    $.error ? n(c, h($.error.message)) : n(G, true);
  }
  var te = ea(), S = a(te);
  {
    var I = (m) => {
      Dt(m, { get userId() {
        return s().id;
      }, get purpose() {
        return e(P);
      }, onSuccess: ae, onError: H });
    };
    x(S, (m) => {
      e(P) && m(I);
    });
  }
  var _ = v(S, 2);
  {
    var A = (m) => {
      var $ = Yr(), g = a($), U = a(g, true);
      t(g);
      var Q = v(g, 2), ie = a(Q), W = a(ie, true);
      t(ie), t(Q);
      var ne = v(Q, 2);
      Et(ne, "height", ".3rem");
      var le = v(ne, 2), be = a(le, true);
      t(le);
      var ke = v(le, 2);
      {
        var b = (z) => {
          Ge(z, {});
        }, p = (z) => {
          xe(z, { level: 2, onclick: ee, children: (X, ge) => {
            pe();
            var R = we();
            C(() => l(R, r.account.passwordReset)), o(X, R);
          }, $$slots: { default: true } });
        };
        x(ke, (z) => {
          e(G) ? z(b) : z(p, false);
        });
      }
      t($), C(() => {
        var _a;
        l(U, r.account.federatedConvertPassword1), l(W, ((_a = f.authProvider) == null ? void 0 : _a.name) || "UNKNOWN"), l(be, r.account.federatedConvertPassword2);
      }), o(m, $);
    };
    x(_, (m) => {
      e(i) === "federated" && m(A);
    });
  }
  var u = v(_, 2);
  {
    var V = (m) => {
      var $ = jr(), g = j($), U = a(g, true);
      t(g);
      var Q = v(g, 2), ie = a(Q, true);
      t(Q);
      var W = v(Q, 2), ne = a(W, true);
      t(W);
      var le = v(W, 2), be = a(le);
      xe(be, { level: 2, onclick: () => n(d, true), children: (ke, b) => {
        pe();
        var p = we();
        C(() => l(p, r.account.convertAccount)), o(ke, p);
      }, $$slots: { default: true } }), t(le), C(() => {
        l(U, r.account.accTypePasskeyText1), l(ie, r.account.accTypePasskeyText2), l(ne, r.account.accTypePasskeyText3);
      }), o(m, $);
    };
    x(u, (m) => {
      (e(i) === "passkey" || e(i) === "federated_passkey") && !e(d) && m(V);
    });
  }
  var re = v(u, 2);
  {
    var de = (m) => {
      var $ = Zr(), g = j($), U = a(g);
      const Q = he(() => !(e(i) === "password" && e(N).length < 1));
      Hr(U, { get inputWidth() {
        return e(M);
      }, get hideCurrentPassword() {
        return e(Q);
      }, get passwords() {
        return e(q);
      }, set passwords(p) {
        n(q, h(p));
      }, get isValid() {
        return e(w);
      }, set isValid(p) {
        n(w, h(p));
      } });
      var ie = v(U, 2), W = a(ie);
      xe(W, { onclick: L, level: 1, get isLoading() {
        return e(y);
      }, children: (p, z) => {
        pe();
        var X = we();
        C(() => l(X, r.common.save)), o(p, X);
      }, $$slots: { default: true } });
      var ne = v(W, 2);
      {
        var le = (p) => {
          var z = Gr(), X = a(z);
          it(X, {}), t(z), Ae(3, z, () => We), o(p, z);
        }, be = (p) => {
          var z = fe(), X = j(z);
          {
            var ge = (D) => {
              var _e = Jr(), ue = a(_e, true);
              t(_e), C(() => l(ue, e(c))), Ae(3, _e, () => We), o(D, _e);
            }, R = (D) => {
              var _e = fe(), ue = j(_e);
              {
                var Ie = (Ee) => {
                  var Me = Qr(), Xe = a(Me);
                  xe(Xe, { level: 3, onclick: () => n(d, false), children: (Ze, dt) => {
                    pe();
                    var Le = we();
                    C(() => l(Le, r.common.cancel)), o(Ze, Le);
                  }, $$slots: { default: true } }), t(Me), o(Ee, Me);
                };
                x(ue, (Ee) => {
                  e(d) && !e(y) && Ee(Ie);
                }, true);
              }
              o(D, _e);
            };
            x(X, (D) => {
              e(c) ? D(ge) : D(R, false);
            }, true);
          }
          o(p, z);
        };
        x(ne, (p) => {
          e(G) ? p(le) : p(be, false);
        });
      }
      t(ie), t(g);
      var ke = v(g, 2);
      {
        var b = (p) => {
          var z = Xr(), X = a(z), ge = a(X, true);
          t(X);
          var R = v(X, 2), D = a(R, true);
          t(R);
          var _e = v(R, 2);
          xe(_e, { level: 2, onclick: ve, children: (ue, Ie) => {
            pe();
            var Ee = we();
            C(() => l(Ee, r.account.convertAccount)), o(ue, Ee);
          }, $$slots: { default: true } }), t(z), C(() => {
            l(ge, r.account.convertAccount), l(D, r.account.convertAccountP1);
          }), o(p, z);
        };
        x(ke, (p) => {
          !e(d) && e(B) && p(b);
        });
      }
      o(m, $);
    };
    x(re, (m) => {
      (e(i) === "password" || e(i) === "federated_password" || e(d)) && m(de);
    });
  }
  t(te), o(se, te), Ce();
}
function ta(se, f) {
  se.code === "Enter" && f();
}
var ra = k('<div class="flex"><div class="label font-label noselect svelte-1xf5lr3"><!></div> <label class="switch svelte-1xf5lr3"><input type="checkbox" class="svelte-1xf5lr3"> <span class="slider slider-round svelte-1xf5lr3"></span></label></div>');
function kt(se, f) {
  let s = Te(f, "checked", 15, false), r = Te(f, "ariaLabel", 3, ""), M = Te(f, "name", 3, "");
  function i() {
    s(!s());
  }
  var N = ra(), w = a(N), d = a(w);
  mr(d, () => f.children), t(w);
  var y = v(w, 2), c = a(y);
  sr(c), c.__click = i, c.__keydown = [ta, i], pe(2), t(y), t(N), C(() => {
    Et(w, "width", f.labelWidth), je(c, "name", M()), c.disabled = f.disabled, je(c, "aria-checked", s()), je(c, "aria-label", r());
  }), pr(c, s), o(se, N);
}
jt(["click", "keydown"]);
var aa = k("<div><p> </p> <!></div>"), sa = k('<div class="success svelte-5kivuv"><!></div>'), va = k('<div class="err svelte-5kivuv"> </div>'), na = k('<div class="container svelte-5kivuv"><p> </p> <p><!></p> <div class="switch svelte-5kivuv"><!></div> <div class="switch svelte-5kivuv"><!></div> <!> <div class="bottom svelte-5kivuv"><!> <!></div></div>');
function Pt(se, f) {
  De(f, true);
  let s = Te(f, "webIdData", 15), r = Ne();
  const M = "14rem";
  let i = T(""), N = T(false), w = T(!!s().custom_triples), d = st(s().user_id);
  async function y() {
    n(i, "");
    let _ = await ot(`/auth/v1/users/${s().user_id}/webid/data`, s());
    _.error ? n(i, h(_.error.message)) : (n(N, true), setTimeout(() => {
      n(N, false);
    }, 3e3));
  }
  var c = na(), G = a(c), P = a(G, true);
  t(G);
  var q = v(G, 2), B = a(q);
  gr(B, { href: d, target: "_blank", children: (_, A) => {
    var u = fe(), V = j(u);
    Tt(V, () => d.replace("/auth/", "/auth/<wbr/>")), o(_, u);
  }, $$slots: { default: true } }), t(q);
  var Z = v(q, 2), ve = a(Z);
  kt(ve, { ariaLabel: "E-Mail", labelWidth: M, get checked() {
    return s().expose_email;
  }, set checked(_) {
    s(s().expose_email = _, true);
  }, children: (_, A) => {
    pe();
    var u = we("E-Mail");
    o(_, u);
  }, $$slots: { default: true } }), t(Z);
  var L = v(Z, 2), E = a(L);
  kt(E, { get ariaLabel() {
    return r.account.webIdExpertMode;
  }, labelWidth: M, get checked() {
    return e(w);
  }, set checked(_) {
    n(w, h(_));
  }, children: (_, A) => {
    pe();
    var u = we();
    C(() => l(u, r.account.webIdExpertMode)), o(_, u);
  }, $$slots: { default: true } }), t(L);
  var K = v(L, 2);
  {
    var H = (_) => {
      var A = aa(), u = a(A), V = a(u, true);
      t(u);
      var re = v(u, 2);
      hr(re, { placeholder: "FOAF", rows: 15, get value() {
        return s().custom_triples;
      }, set value(de) {
        s(s().custom_triples = de, true);
      } }), t(A), C(() => l(V, r.account.webIdDescData)), Ae(3, A, () => vr), o(_, A);
    };
    x(K, (_) => {
      e(w) && _(H);
    });
  }
  var ae = v(K, 2), ee = a(ae);
  xe(ee, { onclick: y, children: (_, A) => {
    pe();
    var u = we();
    C(() => l(u, r.common.save)), o(_, u);
  }, $$slots: { default: true } });
  var te = v(ee, 2);
  {
    var S = (_) => {
      var A = sa(), u = a(A);
      it(u, {}), t(A), Ae(3, A, () => We), o(_, A);
    }, I = (_) => {
      var A = fe(), u = j(A);
      {
        var V = (re) => {
          var de = va(), m = a(de, true);
          t(de), C(() => l(m, e(i))), Ae(3, de, () => We), o(re, de);
        };
        x(u, (re) => {
          e(i) && re(V);
        }, true);
      }
      o(_, A);
    };
    x(te, (_) => {
      e(N) ? _(S) : _(I, false);
    });
  }
  t(ae), t(c), C(() => l(P, r.account.webIdDesc)), o(se, c), Ce();
}
function oa(se, f) {
  De(f, true);
  let s = lt("account"), r = he(() => {
    var _a;
    return (_a = s.get()) == null ? void 0 : _a.user_id;
  });
  var M = fe(), i = j(M);
  {
    var N = (w) => {
      Ct(w, { viewMode: "account", get userId() {
        return e(r);
      } });
    };
    x(i, (w) => {
      e(r) && w(N);
    });
  }
  o(se, M), Ce();
}
var ia = k("<h3> </h3>"), la = k('<div class="headerPhone svelte-k6hxbm"><!></div> <div class="container svelte-k6hxbm"><!> <div class="innerPhone svelte-k6hxbm"><!></div></div>', 1), da = k('<div class="header svelte-k6hxbm"><!></div> <div class="container svelte-k6hxbm"><!> <div class="inner svelte-k6hxbm"><!></div></div>', 1), ua = k('<!> <div class="wrapper svelte-k6hxbm"><!></div>', 1);
function ca(se, f) {
  De(f, true);
  const s = (E) => {
    var K = ia(), H = a(K, true);
    t(K), C(() => l(H, `${r().given_name} ${r().family_name || ""}`)), o(E, K);
  };
  let r = Te(f, "user", 15), M = Te(f, "webIdData", 15), i = Ne(), N = T(void 0), w = T(h([])), d = he(() => {
    var _a;
    if ((_a = r().account_type) == null ? void 0 : _a.startsWith("federated")) return e(w).filter((E) => E.id === r().auth_provider_id)[0];
  }), y = he(() => e(N) && e(N) < 500), c = T(h(i.account.navInfo)), G = h(M() ? [i.account.navInfo, i.account.navEdit, i.common.password, i.account.navMfa, "WebID", i.account.devices, i.account.navLogout] : [i.account.navInfo, i.account.navEdit, i.common.password, i.account.navMfa, i.account.devices, i.account.navLogout]);
  Qe(() => {
    br("v").get() === "devices" && n(c, h(i.account.devices));
  }), Je(() => {
    e(c) === i.account.navLogout && tr();
  });
  var P = ua(), q = j(P);
  wr(q, { id: rr, get value() {
    return e(w);
  }, set value(E) {
    n(w, h(E));
  } });
  var B = v(q, 2), Z = a(B);
  {
    var ve = (E) => {
      var K = la(), H = j(K), ae = a(H);
      s(ae), t(H);
      var ee = v(H, 2), te = a(ee);
      ht(te, { get tabs() {
        return G;
      }, get selected() {
        return e(c);
      }, set selected(u) {
        n(c, h(u));
      } });
      var S = v(te, 2), I = a(S);
      {
        var _ = (u) => {
          wt(u, { get providers() {
            return e(w);
          }, get authProvider() {
            return e(d);
          }, viewModePhone: true, get webIdData() {
            return M();
          }, get user() {
            return r();
          }, set user(V) {
            r(V);
          } });
        }, A = (u) => {
          var V = fe(), re = j(V);
          {
            var de = ($) => {
              bt($, { viewModePhone: true, get user() {
                return r();
              }, set user(g) {
                r(g);
              } });
            }, m = ($) => {
              var g = fe(), U = j(g);
              {
                var Q = (W) => {
                  xt(W, { get user() {
                    return r();
                  }, get authProvider() {
                    return e(d);
                  }, viewModePhone: true });
                }, ie = (W) => {
                  var ne = fe(), le = j(ne);
                  {
                    var be = (b) => {
                      yt(b, { get user() {
                        return r();
                      } });
                    }, ke = (b) => {
                      var p = fe(), z = j(p);
                      {
                        var X = (R) => {
                          Pt(R, { get webIdData() {
                            return M();
                          }, set webIdData(D) {
                            M(D);
                          } });
                        }, ge = (R) => {
                          var D = fe(), _e = j(D);
                          {
                            var ue = (Ie) => {
                              Ct(Ie, { get userId() {
                                return r().id;
                              } });
                            };
                            x(_e, (Ie) => {
                              e(c) === i.account.devices && Ie(ue);
                            }, true);
                          }
                          o(R, D);
                        };
                        x(z, (R) => {
                          e(c) === "WebID" ? R(X) : R(ge, false);
                        }, true);
                      }
                      o(b, p);
                    };
                    x(le, (b) => {
                      e(c) === i.account.navMfa ? b(be) : b(ke, false);
                    }, true);
                  }
                  o(W, ne);
                };
                x(U, (W) => {
                  e(c) === i.common.password ? W(Q) : W(ie, false);
                }, true);
              }
              o($, g);
            };
            x(re, ($) => {
              e(c) === i.account.navEdit ? $(de) : $(m, false);
            }, true);
          }
          o(u, V);
        };
        x(I, (u) => {
          e(c) === i.account.navInfo ? u(_) : u(A, false);
        });
      }
      t(S), t(ee), o(E, K);
    }, L = (E) => {
      var K = da(), H = j(K), ae = a(H);
      s(ae), t(H);
      var ee = v(H, 2), te = a(ee);
      ht(te, { get tabs() {
        return G;
      }, center: true, get selected() {
        return e(c);
      }, set selected(u) {
        n(c, h(u));
      } });
      var S = v(te, 2), I = a(S);
      {
        var _ = (u) => {
          wt(u, { get webIdData() {
            return M();
          }, get providers() {
            return e(w);
          }, get authProvider() {
            return e(d);
          }, get user() {
            return r();
          }, set user(V) {
            r(V);
          } });
        }, A = (u) => {
          var V = fe(), re = j(V);
          {
            var de = ($) => {
              bt($, { get user() {
                return r();
              }, set user(g) {
                r(g);
              } });
            }, m = ($) => {
              var g = fe(), U = j(g);
              {
                var Q = (W) => {
                  xt(W, { get user() {
                    return r();
                  }, get authProvider() {
                    return e(d);
                  } });
                }, ie = (W) => {
                  var ne = fe(), le = j(ne);
                  {
                    var be = (b) => {
                      yt(b, { get user() {
                        return r();
                      } });
                    }, ke = (b) => {
                      var p = fe(), z = j(p);
                      {
                        var X = (R) => {
                          Pt(R, { get webIdData() {
                            return M();
                          }, set webIdData(D) {
                            M(D);
                          } });
                        }, ge = (R) => {
                          var D = fe(), _e = j(D);
                          {
                            var ue = (Ie) => {
                              oa(Ie, {});
                            };
                            x(_e, (Ie) => {
                              e(c) === i.account.devices && Ie(ue);
                            }, true);
                          }
                          o(R, D);
                        };
                        x(z, (R) => {
                          e(c) === "WebID" ? R(X) : R(ge, false);
                        }, true);
                      }
                      o(b, p);
                    };
                    x(le, (b) => {
                      e(c) === i.account.navMfa ? b(be) : b(ke, false);
                    }, true);
                  }
                  o(W, ne);
                };
                x(U, (W) => {
                  e(c) === i.common.password ? W(Q) : W(ie, false);
                }, true);
              }
              o($, g);
            };
            x(re, ($) => {
              e(c) === i.account.navEdit ? $(de) : $(m, false);
            }, true);
          }
          o(u, V);
        };
        x(I, (u) => {
          e(c) === i.account.navInfo ? u(_) : u(A, false);
        });
      }
      t(S), t(ee), o(E, K);
    };
    x(Z, (E) => {
      e(y) ? E(ve) : E(L, false);
    });
  }
  t(B), Jt("innerWidth", (E) => n(N, h(E))), o(se, P), Ce();
}
var fa = k("<!> <!> <!>", 1);
function Za(se, f) {
  De(f, true);
  let s = Ne(), r = lt("account"), M = T(void 0), i = T(void 0), N = T(false);
  Je(() => {
    let d = r.get();
    d && w(d);
  });
  async function w(d) {
    const y = d.user_id;
    if (y) {
      let c = await Promise.all([Ve(`/auth/v1/users/${y}`), Ve(`/auth/v1/users/${y}/webid/data`)]);
      c[0].body ? n(M, h(c[0].body)) : ar("account"), c[1].body ? n(i, h(c[1].body)) : c[1].status === 404 && n(i, h({ user_id: y, expose_email: false })), n(N, true);
    } else console.error("no user_id in session");
  }
  Gt((d) => {
    C(() => {
      var _a;
      return Yt.title = `${(s == null ? void 0 : s.account.account) || "Account"} ${((_a = e(M)) == null ? void 0 : _a.email) ?? ""}`;
    });
  }), yr(se, { children: (d, y) => {
    xr(d, { children: (c, G) => {
      var P = fa(), q = j(P);
      {
        var B = (L) => {
          ca(L, { get user() {
            return e(M);
          }, set user(E) {
            n(M, h(E));
          }, get webIdData() {
            return e(i);
          }, set webIdData(E) {
            n(i, h(E));
          } });
        };
        x(q, (L) => {
          e(N) && r && e(M) && L(B);
        });
      }
      var Z = v(q, 2);
      kr(Z, { absolute: true });
      var ve = v(Z, 2);
      Pr(ve, { absolute: true }), o(c, P);
    } });
  } }), Ce();
}
export {
  Za as component
};
