import { t as P, a as i, e as xe, d as me } from "../chunks/BH6NCLk-.js";
import { p as Ne, c as r, s as v, j as e, a9 as _e, r as t, t as L, a as Ae, a7 as be, k as N, f as B, l as n, a5 as tt, aa as Be, a6 as Xt } from "../chunks/CvlvO1XB.js";
import { s as l, d as Zt, h as er } from "../chunks/CTI4QPiR.js";
import { i as w } from "../chunks/BUO_AUgz.js";
import { p as m } from "../chunks/Wh68IIk2.js";
import { a as S, c as U, s as qe, r as tr } from "../chunks/BMbqVy6X.js";
import { b as rr } from "../chunks/7lxiEjMQ.js";
import { p as Ce } from "../chunks/C6SR4G2t.js";
import { l as Ze, Q as ot, J as ar, P as sr, K as vr, z as nr, S as bt, D as or, r as ir } from "../chunks/B21bTIl7.js";
import { e as Tt } from "../chunks/BpWRzPRQ.js";
import { h as Ct } from "../chunks/i8Xqpu09.js";
import { a as dt, B as Pe, t as Re, f as Ve, s as lr } from "../chunks/CqH8LXO-.js";
import { C as et } from "../chunks/CS_Msctd.js";
import { M as dr } from "../chunks/S7-4yce3.js";
import { u as We } from "../chunks/CUqQZdNU.js";
import { c as Dt, b as it, d as ut, f as Ke } from "../chunks/BO1A6s0c.js";
import { B as ur } from "../chunks/DyuRZLKi.js";
import { U as cr, a as fr, D as Mt } from "../chunks/COMF9fQR.js";
import { I as Le } from "../chunks/KD8vSltG.js";
import { F as _r } from "../chunks/Df37IfRm.js";
import { b as lt, k as mr, l as yt, m as pr } from "../chunks/BRCxk8by.js";
import { I as ct } from "../chunks/Nks81rMs.js";
import { I as gr } from "../chunks/Dre48gu9.js";
import { u as ft } from "../chunks/BCsIV4gx.js";
import { w as hr } from "../chunks/CGaoN_LG.js";
import { W as Lt } from "../chunks/sEr0MWP3.js";
import { P as wr } from "../chunks/DIB4eys6.js";
import { I as vt } from "../chunks/BMwvmSAM.js";
import { s as br } from "../chunks/-T201g_q.js";
import { c as yr } from "../chunks/dU6E9WaN.js";
import { A as xr } from "../chunks/tK0ui9-v.js";
import { I as kr } from "../chunks/BSqmaQGc.js";
import { T as Pr } from "../chunks/aeeWmV22.js";
import { u as Ir } from "../chunks/EvbDpXwj.js";
import { T as xt } from "../chunks/BunZbqN2.js";
import { I as $r } from "../chunks/CbjsfN_U.js";
import { M as Er } from "../chunks/BbRujPvb.js";
import { C as Tr } from "../chunks/GWgockW8.js";
import { T as Cr } from "../chunks/CXfCraUq.js";
import { L as Dr } from "../chunks/CyncXH1a.js";
var Mr = P('<div class="link-err value svelte-1tea8v4"> </div>'), Lr = P('<div class="fed-btn svelte-1tea8v4"><!> <!></div>'), Nr = P('<h3> </h3> <p> </p> <div class="providers svelte-1tea8v4"></div>', 1), Ar = P("<!> <!>", 1), Wr = P('<div><div> </div> <span class="value svelte-1tea8v4"> </span></div>'), zr = P('<div><div> </div> <span class="value svelte-1tea8v4"> </span></div>'), Rr = P('<div><div>WebID:</div> <span class="value svelte-1tea8v4"><a target="_blank"><!></a></span></div>'), Sr = P('<div><div><div></div> <!></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <div><div class="value svelte-1tea8v4"> </div> <!></div></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <!></div> <div><div> </div> <!></div> <div><div> </div> <!></div> <!> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <!> <!></div>');
function nt(de, c) {
  Ne(c, true);
  let s = Ce(c, "user", 15), a = We(), $ = N(false), o = N(false), D = _e(() => {
    var _a2;
    return (_a2 = s().account_type) == null ? void 0 : _a2.startsWith("federated");
  }), p = _e(() => {
    var _a2;
    return e(D) ? `${s().account_type}: ${((_a2 = c.authProvider) == null ? void 0 : _a2.name) || ""}` : s().account_type;
  }), d = _e(() => c.viewModePhone ? "rowPhone" : "row"), b = _e(() => c.viewModePhone ? "labelPhone" : "label"), y = _e(() => {
    let z = s().given_name[0];
    return s().family_name && s().family_name.length > 0 && (z += s().family_name[0]), z;
  });
  function h(z) {
    ar(64, (Q, { challenge: fe, verifier: we }) => {
      Q || (localStorage.setItem(sr, we), T(z, fe));
    });
  }
  async function T(z, Q) {
    let fe = { email: s().email, client_id: "rauthy", redirect_uri: window.location.href, provider_id: z, pkce_challenge: Q }, we = await it(`/auth/v1/providers/${z}/link`, fe);
    if (we.text) {
      vr(we.text);
      let he = we.headers.get("location");
      he && (window.location.href = he);
    } else console.error(we.error);
  }
  async function G() {
    let z = await Dt("/auth/v1/providers/link");
    z.body ? s(z.body) : (console.error(z.error), n($, true));
  }
  var K = Sr(), J = r(K), ve = r(J), R = v(ve, 2);
  cr(R, { get fallbackCharacters() {
    return e(y);
  }, get userId() {
    return s().id;
  }, size: "large", get pictureId() {
    return s().picture_id;
  }, set pictureId(z) {
    s(s().picture_id = z, true);
  } }), t(J);
  var X = v(J, 2), ie = r(X), F = r(ie, true);
  t(ie);
  var q = v(ie, 2), Z = r(q, true);
  t(q), t(X);
  var te = v(X, 2), V = r(te), M = r(V, true);
  t(V);
  var u = v(V, 2), W = r(u, true);
  t(u), t(te);
  var f = v(te, 2), re = r(f), H = r(re, true);
  t(re);
  var ne = v(re, 2), g = r(ne, true);
  t(ne), t(f);
  var k = v(f, 2), _ = r(k), I = r(_);
  t(_);
  var A = v(_, 2), ue = r(A, true);
  t(A), t(k);
  var O = v(k, 2), ae = r(O), se = r(ae, true);
  t(ae);
  var ye = v(ae, 2), ke = r(ye), C = r(ke, true);
  t(ke);
  var x = v(ke, 2);
  {
    var E = (z) => {
      var Q = Lr(), fe = r(Q);
      Pe(fe, { get ariaLabel() {
        return a.account.providerUnlink;
      }, level: 3, onclick: G, children: (Ie, Te) => {
        be();
        var Se = xe();
        L(() => l(Se, a.account.providerUnlink)), i(Ie, Se);
      }, $$slots: { default: true } });
      var we = v(fe, 2);
      {
        var he = (Ie) => {
          var Te = Mr(), Se = r(Te, true);
          t(Te), L(() => l(Se, a.account.providerUnlinkDesc)), i(Ie, Te);
        };
        w(we, (Ie) => {
          e($) && Ie(he);
        });
      }
      t(Q), i(z, Q);
    }, ee = (z) => {
      var Q = me(), fe = B(Q);
      {
        var we = (he) => {
          var Ie = Ar(), Te = B(Ie);
          Pe(Te, { level: 2, onclick: () => n(o, true), children: (Ue, jt) => {
            be();
            var Fe = xe();
            L(() => l(Fe, a.account.providerLink)), i(Ue, Fe);
          }, $$slots: { default: true } });
          var Se = v(Te, 2);
          dr(Se, { get showModal() {
            return e(o);
          }, set showModal(Ue) {
            n(o, m(Ue));
          }, children: (Ue, jt) => {
            var Fe = Nr(), rt = B(Fe), Gt = r(rt, true);
            t(rt);
            var at = v(rt, 2), Jt = r(at, true);
            t(at);
            var ht = v(at, 2);
            Tt(ht, 21, () => c.providers, (st) => st.id, (st, wt) => {
              const Qt = _e(() => `${a.account.providerLink}: ${e(wt).name}`);
              ur(st, { get ariaLabel() {
                return e(Qt);
              }, get provider() {
                return e(wt);
              }, onclick: h });
            }), t(ht), L(() => {
              l(Gt, a.account.providerLink), l(Jt, a.account.providerLinkDesc);
            }), i(Ue, Fe);
          }, $$slots: { default: true } }), i(he, Ie);
        };
        w(fe, (he) => {
          c.providers.length > 0 && he(we);
        }, true);
      }
      i(z, Q);
    };
    w(x, (z) => {
      e(D) ? z(E) : z(ee, false);
    });
  }
  t(ye), t(O);
  var pe = v(O, 2), Y = r(pe), j = r(Y, true);
  t(Y);
  var oe = v(Y, 2), ce = r(oe, true);
  t(oe), t(pe);
  var $e = v(pe, 2), le = r($e), ge = r(le, true);
  t(le);
  var De = v(le, 2), ze = r(De, true);
  t(De), t($e);
  var Ee = v($e, 2), Me = r(Ee), Nt = r(Me, true);
  t(Me);
  var At = v(Me, 2);
  const Wt = _e(() => !!s().webauthn_user_id);
  et(At, { get checked() {
    return e(Wt);
  } }), t(Ee);
  var Oe = v(Ee, 2), He = r(Oe), zt = r(He, true);
  t(He);
  var Rt = v(He, 2);
  et(Rt, { get checked() {
    return s().enabled;
  } }), t(Oe);
  var Ye = v(Oe, 2), je = r(Ye), St = r(je, true);
  t(je);
  var Vt = v(je, 2);
  et(Vt, { get checked() {
    return s().email_verified;
  } }), t(Ye);
  var _t = v(Ye, 2);
  {
    var Ut = (z) => {
      var Q = Wr(), fe = r(Q), we = r(fe, true);
      t(fe);
      var he = v(fe, 2), Ie = r(he, true);
      t(he), t(Q), L((Te) => {
        S(Q, 1, U(e(d)), "svelte-1tea8v4"), S(fe, 1, U(e(b)), "svelte-1tea8v4"), l(we, a.account.lastLogin), l(Ie, Te);
      }, [() => Ze(s().last_login)]), i(z, Q);
    };
    w(_t, (z) => {
      s().last_login && z(Ut);
    });
  }
  var Ge = v(_t, 2), Je = r(Ge), Ft = r(Je, true);
  t(Je);
  var mt = v(Je, 2), qt = r(mt, true);
  t(mt), t(Ge);
  var Qe = v(Ge, 2), Xe = r(Qe), Bt = r(Xe, true);
  t(Xe);
  var pt = v(Xe, 2), Kt = r(pt, true);
  t(pt), t(Qe);
  var gt = v(Qe, 2);
  {
    var Ot = (z) => {
      var Q = zr(), fe = r(Q), we = r(fe, true);
      t(fe);
      var he = v(fe, 2), Ie = r(he, true);
      t(he), t(Q), L((Te) => {
        S(Q, 1, U(e(d)), "svelte-1tea8v4"), S(fe, 1, U(e(b)), "svelte-1tea8v4"), l(we, a.account.userExpiry), l(Ie, Te);
      }, [() => Ze(s().user_expires)]), i(z, Q);
    };
    w(gt, (z) => {
      s().user_expires && z(Ot);
    });
  }
  var Ht = v(gt, 2);
  {
    var Yt = (z) => {
      var Q = Rr(), fe = r(Q), we = v(fe, 2), he = r(we), Ie = r(he);
      Ct(Ie, () => ot(s().id).replace("/auth/", "/auth/<wbr/>")), t(he), t(we), t(Q), L((Te) => {
        S(Q, 1, U(e(d)), "svelte-1tea8v4"), S(fe, 1, U(e(b)), "svelte-1tea8v4"), qe(he, "href", Te);
      }, [() => ot(s().id)]), i(z, Q);
    };
    w(Ht, (z) => {
      c.webIdData && z(Yt);
    });
  }
  t(K), L((z, Q) => {
    S(J, 1, U(e(d)), "svelte-1tea8v4"), dt(J, "margin", ".5rem 0"), S(ve, 1, U(e(b)), "svelte-1tea8v4"), S(X, 1, U(e(d)), "svelte-1tea8v4"), S(ie, 1, U(e(b)), "svelte-1tea8v4"), l(F, a.common.email), l(Z, s().email), S(te, 1, U(e(d)), "svelte-1tea8v4"), S(V, 1, U(e(b)), "svelte-1tea8v4"), l(M, a.account.givenName), l(W, s().given_name), S(f, 1, U(e(d)), "svelte-1tea8v4"), S(re, 1, U(e(b)), "svelte-1tea8v4"), l(H, a.account.familyName), l(g, s().family_name), S(k, 1, U(e(d)), "svelte-1tea8v4"), S(_, 1, U(e(b)), "svelte-1tea8v4"), l(I, `${a.account.user ?? ""} ID`), l(ue, s().id), S(O, 1, U(e(d)), "svelte-1tea8v4"), S(ae, 1, U(e(b)), "svelte-1tea8v4"), l(se, a.account.accType), l(C, e(p) || ""), S(pe, 1, U(e(d)), "svelte-1tea8v4"), S(Y, 1, U(e(b)), "svelte-1tea8v4"), l(j, a.account.roles), l(ce, s().roles || "None"), S($e, 1, U(e(d)), "svelte-1tea8v4"), S(le, 1, U(e(b)), "svelte-1tea8v4"), l(ge, a.account.groups), l(ze, s().groups || "None"), S(Ee, 1, U(e(d)), "svelte-1tea8v4"), S(Me, 1, U(e(b)), "svelte-1tea8v4"), l(Nt, a.account.mfaActivated), S(Oe, 1, U(e(d)), "svelte-1tea8v4"), S(He, 1, U(e(b)), "svelte-1tea8v4"), l(zt, a.account.userEnabled), S(Ye, 1, U(e(d)), "svelte-1tea8v4"), S(je, 1, U(e(b)), "svelte-1tea8v4"), l(St, a.account.emailVerified), S(Ge, 1, U(e(d)), "svelte-1tea8v4"), S(Je, 1, U(e(b)), "svelte-1tea8v4"), l(Ft, a.account.passwordExpiry), l(qt, z), S(Qe, 1, U(e(d)), "svelte-1tea8v4"), S(Xe, 1, U(e(b)), "svelte-1tea8v4"), l(Bt, a.account.userCreated), l(Kt, Q);
  }, [() => s().password_expires && Ze(s().password_expires) || a.common.never, () => Ze(s().created_at)]), i(de, K), Ae();
}
var Vr = P('<div class="success svelte-4ogr3z"><!></div>'), Ur = P("<p> </p>"), Fr = P('<div class="err svelte-4ogr3z"> </div>'), qr = P('<div class="formInner svelte-4ogr3z"><div><!> <!> <!> <!></div> <div><!> <!> <!> <!> <!></div></div> <div class="bottom svelte-4ogr3z"><div><!></div> <!></div> <!>', 1), Br = P('<div class="container svelte-4ogr3z"><!></div>');
function kt(de, c) {
  var _a2;
  Ne(c, true);
  let s = Ce(c, "user", 15);
  ((_a2 = s().user_values) == null ? void 0 : _a2.birthdate) || s(s().user_values.birthdate = "", true);
  let a = We(), $ = N(""), o = N(false), D = N(false);
  async function p(h, T) {
    var _a3;
    const G = T.get("email"), K = T.get("family_name") || void 0, J = T.get("given_name") || void 0, ve = T.get("birthdate") || void 0, R = ((_a3 = T.get("phone")) == null ? void 0 : _a3.replaceAll(" ", "")) || void 0, X = T.get("street") || void 0, ie = T.get("zip") || void 0, F = ie ? Number.parseInt(ie) : void 0, q = T.get("city") || void 0, Z = T.get("country") || void 0;
    let te = { email: G, family_name: K, given_name: J };
    (ve || R || X || F || q || Z) && (te.user_values = { birthdate: ve, phone: R, street: X, zip: F, city: q, country: Z });
    let V = await ut(`/auth/v1/users/${s().id}/self`, te);
    if (V.body) {
      n(o, true), s(V.body);
      let M = 3e3;
      V.status === 202 && (n(D, true), M = 3e4), setTimeout(() => {
        n(o, false);
      }, M);
    } else V.error && (console.error(V.error), n($, m(V.error.message)));
  }
  var d = Br(), b = r(d);
  const y = _e(() => `/auth/v1/users/${s().id}/self`);
  _r(b, { get action() {
    return e(y);
  }, onSubmit: p, children: (h, T) => {
    var G = qr(), K = B(G), J = r(K), ve = r(J);
    Le(ve, { typ: "email", name: "email", get label() {
      return a.common.email;
    }, get placeholder() {
      return a.common.email;
    }, get value() {
      return s().email;
    }, required: true });
    var R = v(ve, 2);
    Le(R, { name: "given_name", autocomplete: "given-name", get label() {
      return a.account.givenName;
    }, get placeholder() {
      return a.account.givenName;
    }, get value() {
      return s().given_name;
    }, required: true, maxLength: 32, pattern: lt });
    var X = v(R, 2);
    Le(X, { name: "family_name", autocomplete: "family-name", get label() {
      return a.account.familyName;
    }, get placeholder() {
      return a.account.familyName;
    }, get value() {
      return s().family_name;
    }, maxLength: 32, pattern: lt });
    var ie = v(X, 2);
    gr(ie, { name: "birthdate", get label() {
      return a.account.birthdate;
    }, withDelete: true, get value() {
      return s().user_values.birthdate;
    }, set value(_) {
      s(s().user_values.birthdate = _, true);
    } }), t(J);
    var F = v(J, 2), q = r(F);
    Le(q, { name: "street", autocomplete: "street-address", get label() {
      return a.account.street;
    }, get placeholder() {
      return a.account.street;
    }, get value() {
      return s().user_values.street;
    }, maxLength: 48, pattern: mr });
    var Z = v(q, 2);
    Le(Z, { typ: "number", name: "zip", autocomplete: "postal-code", get label() {
      return a.account.zip;
    }, get placeholder() {
      return a.account.zip;
    }, get value() {
      return s().user_values.zip;
    }, min: "1000", max: "9999999" });
    var te = v(Z, 2);
    Le(te, { name: "city", autocomplete: "address-level2", get label() {
      return a.account.city;
    }, get placeholder() {
      return a.account.city;
    }, get value() {
      return s().user_values.city;
    }, maxLength: 48, pattern: yt });
    var V = v(te, 2);
    Le(V, { name: "country", autocomplete: "country", get label() {
      return a.account.country;
    }, get placeholder() {
      return a.account.country;
    }, get value() {
      return s().user_values.country;
    }, maxLength: 48, pattern: yt });
    var M = v(V, 2);
    Le(M, { name: "phone", autocomplete: "tel", get label() {
      return a.account.phone;
    }, get placeholder() {
      return a.account.phone;
    }, get value() {
      return s().user_values.phone;
    }, maxLength: 32, pattern: pr }), t(F), t(K);
    var u = v(K, 2), W = r(u), f = r(W);
    Pe(f, { type: "submit", children: (_, I) => {
      be();
      var A = xe();
      L(() => l(A, a.common.save)), i(_, A);
    }, $$slots: { default: true } }), t(W);
    var re = v(W, 2);
    {
      var H = (_) => {
        var I = Vr(), A = r(I);
        ct(A, {}), t(I), Re(3, I, () => Ve), i(_, I);
      };
      w(re, (_) => {
        e(o) && _(H);
      });
    }
    t(u);
    var ne = v(u, 2);
    {
      var g = (_) => {
        var I = Ur(), A = r(I, true);
        t(I), L(() => l(A, a.account.emailUpdateConfirm)), i(_, I);
      }, k = (_) => {
        var I = me(), A = B(I);
        {
          var ue = (O) => {
            var ae = Fr(), se = r(ae, true);
            t(ae), L(() => l(se, e($))), Re(3, ae, () => Ve), i(O, ae);
          };
          w(A, (O) => {
            e($) && O(ue);
          }, true);
        }
        i(_, I);
      };
      w(ne, (_) => {
        e(D) ? _(g) : _(k, false);
      });
    }
    i(h, G);
  }, $$slots: { default: true } }), t(d), i(de, d), Ae();
}
var Kr = P('<div class="err svelte-16mlupu"><b>Your browser does not support Webauthn credentials and must be updated.</b></div>'), Or = P('<!> <div class="regBtns svelte-16mlupu"><!> <!></div>', 1), Hr = P('<div class="regNewBtn svelte-16mlupu"><!></div>'), Yr = P('<div class="keysHeader svelte-16mlupu"> </div>'), jr = P('<div class="button svelte-16mlupu"><!></div>'), Gr = P('<!> <p class="svelte-16mlupu"> <br><br> </p> <!> <!> <div class="keysContainer svelte-16mlupu"></div> <!> <div> </div>', 1), Jr = P('<div class="container svelte-16mlupu"><!></div>');
function Pt(de, c) {
  Ne(c, true);
  const s = "credentials" in navigator;
  let a = We(), $ = ft("account"), o = _e(() => {
    var _a2;
    return (_a2 = $.get()) == null ? void 0 : _a2.user_id;
  }), D = N(void 0), p = N(false), d = N(""), b = N(false), y = N(c.user.account_type === "password"), h = N(void 0), T = N(""), G = N(false), K = N(m([]));
  tt(() => {
    ve();
  }), Be(() => {
    e(K).length > 0 && c.user.account_type === "passkey" && n(y, e(K).length > 1);
  }), Be(() => {
    var _a2;
    (_a2 = e(D)) == null ? void 0 : _a2.focus();
  });
  function J() {
    n(p, false), n(d, "");
  }
  async function ve() {
    var _a2;
    let M = await Ke(`/auth/v1/users/${(_a2 = $.get()) == null ? void 0 : _a2.user_id}/webauthn`);
    M.body ? n(K, m(M.body)) : console.error(M.error);
  }
  async function R() {
    if (J(), e(G) || !e(o)) return;
    if (e(T).length < 1) {
      n(p, true), n(d, m(a.mfa.passkeyNameErr));
      return;
    }
    let M = await hr(e(o), e(T), a.authorize.invalidKeyUsed, a.authorize.requestExpired);
    M.error ? (n(p, true), n(d, `${a.mfa.errorReg} - ${M.error}`)) : (n(b, false), n(T, ""), await ve());
  }
  async function X(M) {
    var _a2;
    let u = await Dt(`/auth/v1/users/${c.user.id}/webauthn/delete/${M}`);
    u.status === 200 ? await ve() : n(d, m(((_a2 = u.error) == null ? void 0 : _a2.message) || "Error"));
  }
  function ie(M) {
    n(h, void 0), n(p, true), n(d, m(M)), setTimeout(() => {
      n(p, false), n(d, "");
    }, 5e3);
  }
  function F(M) {
    n(h, void 0), n(d, m(a.mfa.testSuccess)), setTimeout(() => {
      n(d, "");
    }, 3e3);
  }
  var q = Jr(), Z = r(q);
  {
    var te = (M) => {
      var u = Kr();
      i(M, u);
    }, V = (M) => {
      var u = Gr(), W = B(u);
      {
        var f = (C) => {
          Lt(C, { get userId() {
            return c.user.id;
          }, get purpose() {
            return e(h);
          }, onSuccess: F, onError: ie });
        };
        w(W, (C) => {
          e(h) && C(f);
        });
      }
      var re = v(W, 2), H = r(re), ne = v(H, 3);
      t(re);
      var g = v(re, 2);
      {
        var k = (C) => {
          var x = Or(), E = B(x);
          Le(E, { autocomplete: "off", get label() {
            return a.mfa.passkeyName;
          }, get placeholder() {
            return a.mfa.passkeyName;
          }, maxLength: 32, pattern: lt, onEnter: R, get ref() {
            return e(D);
          }, set ref(j) {
            n(D, m(j));
          }, get value() {
            return e(T);
          }, set value(j) {
            n(T, m(j));
          }, get isError() {
            return e(G);
          }, set isError(j) {
            n(G, m(j));
          } });
          var ee = v(E, 2), pe = r(ee);
          Pe(pe, { onclick: R, children: (j, oe) => {
            be();
            var ce = xe();
            L(() => l(ce, a.mfa.register)), i(j, ce);
          }, $$slots: { default: true } });
          var Y = v(pe, 2);
          Pe(Y, { level: 3, onclick: () => n(b, false), children: (j, oe) => {
            be();
            var ce = xe();
            L(() => l(ce, a.common.cancel)), i(j, ce);
          }, $$slots: { default: true } }), t(ee), i(C, x);
        }, _ = (C) => {
          var x = Hr(), E = r(x);
          const ee = _e(() => e(K).length === 0 ? 1 : 2);
          Pe(E, { get level() {
            return e(ee);
          }, onclick: () => n(b, true), children: (pe, Y) => {
            be();
            var j = xe();
            L(() => l(j, a.mfa.registerNew)), i(pe, j);
          }, $$slots: { default: true } }), t(x), i(C, x);
        };
        w(g, (C) => {
          e(b) ? C(k) : C(_, false);
        });
      }
      var I = v(g, 2);
      {
        var A = (C) => {
          var x = Yr(), E = r(x, true);
          t(x), L(() => l(E, a.mfa.registerdKeys)), i(C, x);
        };
        w(I, (C) => {
          e(K).length > 0 && C(A);
        });
      }
      var ue = v(I, 2);
      Tt(ue, 21, () => e(K), (C) => C.name, (C, x) => {
        fr(C, { get passkey() {
          return e(x);
        }, get showDelete() {
          return e(y);
        }, onDelete: X });
      }), t(ue);
      var O = v(ue, 2);
      {
        var ae = (C) => {
          var x = jr(), E = r(x);
          Pe(E, { onclick: () => n(h, "Test"), children: (ee, pe) => {
            be();
            var Y = xe();
            L(() => l(Y, a.mfa.test)), i(ee, Y);
          }, $$slots: { default: true } }), t(x), i(C, x);
        };
        w(O, (C) => {
          e(K).length > 0 && C(ae);
        });
      }
      var se = v(O, 2);
      let ye;
      var ke = r(se, true);
      t(se), L(() => {
        l(H, `${a.mfa.p1 ?? ""} `), l(ne, ` ${a.mfa.p2 ?? ""}`), ye = S(se, 1, "svelte-16mlupu", null, ye, { success: !e(p), err: e(p) }), l(ke, e(d));
      }), i(M, u);
    };
    w(Z, (M) => {
      s ? M(V, false) : M(te);
    });
  }
  t(q), i(de, q), Ae();
}
var Qr = P('<div class="container svelte-ue7bk2"><!> <!> <!> <!> <div class="btn svelte-ue7bk2"><!></div> <div class="err svelte-ue7bk2"> </div></div>');
function Xr(de, c) {
  Ne(c, true);
  let s = Ce(c, "passwords", 15), a = Ce(c, "hideCurrentPassword", 3, false), $ = Ce(c, "isValid", 15), o = We();
  $(T);
  let D = N(false), p = N(""), d = N(void 0), b = _e(() => {
    var _a2;
    return ((_a2 = s().new) == null ? void 0 : _a2.length) > 6 && s().new === s().newConfirm;
  }), y = N(void 0), h = N(void 0);
  tt(async () => {
    let R = await Ke("/auth/v1/password_policy");
    R.body ? n(d, m(R.body)) : console.error(R.error);
  });
  function T() {
    return n(p, ""), !a() && !s().current ? (n(p, m(o.account.passwordCurrReq)), false) : s().new ? s().newConfirm ? e(D) ? s().new.length > 256 ? (n(p, "max 256"), false) : s().new !== s().newConfirm ? (n(p, m(o.account.passwordNoMatch)), false) : true : (n(p, m(o.account.passwordPolicyFollow)), false) : (n(p, m(o.account.passwordNewReq)), false) : (n(p, m(o.account.passwordNewReq)), false);
  }
  function G() {
    if (n(p, ""), e(d)) {
      let R = nr(e(d));
      s(s().new = R, true), s(s().newConfirm = R, true);
    }
    requestAnimationFrame(() => {
      var _a2, _b;
      (_a2 = e(y)) == null ? void 0 : _a2(), (_b = e(h)) == null ? void 0 : _b();
    });
  }
  var K = me(), J = B(K);
  {
    var ve = (R) => {
      var X = Qr(), ie = r(X);
      wr(ie, { get policy() {
        return e(d);
      }, get password() {
        return s().new;
      }, get accepted() {
        return e(D);
      }, set accepted(f) {
        n(D, m(f));
      } });
      var F = v(ie, 2);
      {
        var q = (f) => {
          vt(f, { autocomplete: "current-password", get label() {
            return o.account.passwordCurr;
          }, get placeholder() {
            return o.account.passwordCurr;
          }, onInput: T, get width() {
            return c.inputWidth;
          }, get value() {
            return s().current;
          }, set value(re) {
            s(s().current = re, true);
          } });
        };
        w(F, (f) => {
          a() || f(q);
        });
      }
      var Z = v(F, 2);
      vt(Z, { autocomplete: "new-password", get label() {
        return o.account.passwordNew;
      }, get placeholder() {
        return o.account.passwordNew;
      }, onInput: T, get showCopy() {
        return e(b);
      }, get width() {
        return c.inputWidth;
      }, get value() {
        return s().new;
      }, set value(f) {
        s(s().new = f, true);
      }, get reportValidity() {
        return e(y);
      }, set reportValidity(f) {
        n(y, m(f));
      } });
      var te = v(Z, 2);
      vt(te, { autocomplete: "new-password", get label() {
        return o.account.passwordConfirm;
      }, get placeholder() {
        return o.account.passwordConfirm;
      }, onInput: T, get showCopy() {
        return e(b);
      }, get width() {
        return c.inputWidth;
      }, get value() {
        return s().newConfirm;
      }, set value(f) {
        s(s().newConfirm = f, true);
      }, get reportValidity() {
        return e(h);
      }, set reportValidity(f) {
        n(h, m(f));
      } });
      var V = v(te, 2), M = r(V);
      Pe(M, { onclick: G, level: 2, children: (f, re) => {
        be();
        var H = xe();
        L(() => l(H, o.account.generateRandom)), i(f, H);
      }, $$slots: { default: true } }), t(V);
      var u = v(V, 2), W = r(u, true);
      t(u), t(X), L(() => l(W, e(p))), i(R, X);
    };
    w(J, (R) => {
      e(d) && R(ve);
    });
  }
  return i(de, K), Ae({ isPwdValid: T });
}
var Zr = P('<div class="m-05 svelte-1h4zvzu"><p> </p> <p><b> </b></p> <div></div> <p> </p> <!></div>'), ea = P("<p> </p> <p> </p> <p> </p> <div><!></div>", 1), ta = P('<div class="success svelte-1h4zvzu"><!></div>'), ra = P('<div class="err"> </div>'), aa = P('<div class="cancel"><!></div>'), sa = P('<div class="convertPasskey svelte-1h4zvzu"><h3> </h3> <p> </p> <!></div>'), va = P('<div><!> <div class="save svelte-1h4zvzu"><!> <!></div></div> <!>', 1), na = P('<div class="container svelte-1h4zvzu"><!> <!> <!> <!></div>');
function It(de, c) {
  Ne(c, true);
  let s = Ce(c, "user", 15), a = We(), $ = _e(() => c.viewModePhone ? "calc(100vw - 1.5rem)" : "300px"), o = N(m(s().account_type)), D = N(m([])), p = N(void 0), d = N(false), b = N(false), y = N(""), h = N(false), T = N(void 0), G = N(m({ current: "", new: "", newConfirm: "" })), K = _e(() => e(D).filter((g) => g.user_verified).length > 0);
  tt(() => {
    J();
  });
  async function J() {
    let g = await Ke(`/auth/v1/users/${s().id}/webauthn`);
    g.body ? n(D, m(g.body)) : console.error("error fetching passkeys: " + g.error);
  }
  async function ve() {
    let g = await it(`/auth/v1/users/${s().id}/self/convert_passkey`);
    g.error ? console.error("error fetching passkeys: " + g.error) : window.location.reload();
  }
  async function R() {
    n(y, ""), e(D).length > 0 ? await X() : await ie();
  }
  async function X() {
    var _a2;
    if (!((_a2 = e(p)) == null ? void 0 : _a2())) {
      n(y, m(a.common.invalidInput));
      return;
    }
    n(T, "PasswordNew");
  }
  async function ie(g) {
    var _a2, _b;
    if (!((_a2 = e(p)) == null ? void 0 : _a2())) {
      n(y, m(a.common.invalidInput));
      return;
    }
    n(b, true);
    let k = { password_new: e(G).new };
    g ? k.mfa_code = g : k.password_current = e(G).current;
    let _ = await ut(`/auth/v1/users/${s().id}/self`, k);
    _.body ? (n(h, true), n(G, m({ current: "", new: "", newConfirm: "" })), s(_.body), n(o, m(_.body.account_type)), setTimeout(() => {
      n(h, false), n(d, false);
    }, 3e3)) : n(y, m(((_b = _.error) == null ? void 0 : _b.message) || "Error")), n(b, false);
  }
  function F(g) {
    n(T, void 0), n(y, m(g)), setTimeout(() => {
      n(y, "");
    }, 5e3);
  }
  function q(g) {
    n(T, void 0), g && "code" in g && ie(g.code);
  }
  async function Z() {
    let g = { email: s().email }, k = await it("/auth/v1/users/request_reset", g);
    k.error ? n(y, m(k.error.message)) : n(h, true);
  }
  var te = na(), V = r(te);
  {
    var M = (g) => {
      Lt(g, { get userId() {
        return s().id;
      }, get purpose() {
        return e(T);
      }, onSuccess: q, onError: F });
    };
    w(V, (g) => {
      e(T) && g(M);
    });
  }
  var u = v(V, 2);
  {
    var W = (g) => {
      var k = Zr(), _ = r(k), I = r(_, true);
      t(_);
      var A = v(_, 2), ue = r(A), O = r(ue, true);
      t(ue), t(A);
      var ae = v(A, 2);
      dt(ae, "height", ".3rem");
      var se = v(ae, 2), ye = r(se, true);
      t(se);
      var ke = v(se, 2);
      {
        var C = (E) => {
          et(E, {});
        }, x = (E) => {
          Pe(E, { level: 2, onclick: Z, children: (ee, pe) => {
            be();
            var Y = xe();
            L(() => l(Y, a.account.passwordReset)), i(ee, Y);
          }, $$slots: { default: true } });
        };
        w(ke, (E) => {
          e(h) ? E(C) : E(x, false);
        });
      }
      t(k), L(() => {
        var _a2;
        l(I, a.account.federatedConvertPassword1), l(O, ((_a2 = c.authProvider) == null ? void 0 : _a2.name) || "UNKNOWN"), l(ye, a.account.federatedConvertPassword2);
      }), i(g, k);
    };
    w(u, (g) => {
      e(o) === "federated" && g(W);
    });
  }
  var f = v(u, 2);
  {
    var re = (g) => {
      var k = ea(), _ = B(k), I = r(_, true);
      t(_);
      var A = v(_, 2), ue = r(A, true);
      t(A);
      var O = v(A, 2), ae = r(O, true);
      t(O);
      var se = v(O, 2), ye = r(se);
      Pe(ye, { level: 2, onclick: () => n(d, true), children: (ke, C) => {
        be();
        var x = xe();
        L(() => l(x, a.account.convertAccount)), i(ke, x);
      }, $$slots: { default: true } }), t(se), L(() => {
        l(I, a.account.accTypePasskeyText1), l(ue, a.account.accTypePasskeyText2), l(ae, a.account.accTypePasskeyText3);
      }), i(g, k);
    };
    w(f, (g) => {
      (e(o) === "passkey" || e(o) === "federated_passkey") && !e(d) && g(re);
    });
  }
  var H = v(f, 2);
  {
    var ne = (g) => {
      var k = va(), _ = B(k), I = r(_);
      const A = _e(() => !(e(o) === "password" && e(D).length < 1));
      Xr(I, { get inputWidth() {
        return e($);
      }, get hideCurrentPassword() {
        return e(A);
      }, get passwords() {
        return e(G);
      }, set passwords(x) {
        n(G, m(x));
      }, get isValid() {
        return e(p);
      }, set isValid(x) {
        n(p, m(x));
      } });
      var ue = v(I, 2), O = r(ue);
      Pe(O, { onclick: R, level: 1, get isLoading() {
        return e(b);
      }, children: (x, E) => {
        be();
        var ee = xe();
        L(() => l(ee, a.common.save)), i(x, ee);
      }, $$slots: { default: true } });
      var ae = v(O, 2);
      {
        var se = (x) => {
          var E = ta(), ee = r(E);
          ct(ee, {}), t(E), Re(3, E, () => Ve), i(x, E);
        }, ye = (x) => {
          var E = me(), ee = B(E);
          {
            var pe = (j) => {
              var oe = ra(), ce = r(oe, true);
              t(oe), L(() => l(ce, e(y))), Re(3, oe, () => Ve), i(j, oe);
            }, Y = (j) => {
              var oe = me(), ce = B(oe);
              {
                var $e = (le) => {
                  var ge = aa(), De = r(ge);
                  Pe(De, { level: 3, onclick: () => n(d, false), children: (ze, Ee) => {
                    be();
                    var Me = xe();
                    L(() => l(Me, a.common.cancel)), i(ze, Me);
                  }, $$slots: { default: true } }), t(ge), i(le, ge);
                };
                w(ce, (le) => {
                  e(d) && !e(b) && le($e);
                }, true);
              }
              i(j, oe);
            };
            w(ee, (j) => {
              e(y) ? j(pe) : j(Y, false);
            }, true);
          }
          i(x, E);
        };
        w(ae, (x) => {
          e(h) ? x(se) : x(ye, false);
        });
      }
      t(ue), t(_);
      var ke = v(_, 2);
      {
        var C = (x) => {
          var E = sa(), ee = r(E), pe = r(ee, true);
          t(ee);
          var Y = v(ee, 2), j = r(Y, true);
          t(Y);
          var oe = v(Y, 2);
          Pe(oe, { level: 2, onclick: ve, children: (ce, $e) => {
            be();
            var le = xe();
            L(() => l(le, a.account.convertAccount)), i(ce, le);
          }, $$slots: { default: true } }), t(E), L(() => {
            l(pe, a.account.convertAccount), l(j, a.account.convertAccountP1);
          }), i(x, E);
        };
        w(ke, (x) => {
          !e(d) && e(K) && x(C);
        });
      }
      i(g, k);
    };
    w(H, (g) => {
      (e(o) === "password" || e(o) === "federated_password" || e(d)) && g(ne);
    });
  }
  t(te), i(de, te), Ae();
}
function oa(de, c) {
  de.code === "Enter" && c();
}
var ia = P('<div class="flex"><div class="label font-label noselect svelte-1xf5lr3"><!></div> <label class="switch svelte-1xf5lr3"><input type="checkbox" class="svelte-1xf5lr3"> <span class="slider slider-round svelte-1xf5lr3"></span></label></div>');
function $t(de, c) {
  let s = Ce(c, "checked", 15, false), a = Ce(c, "ariaLabel", 3, ""), $ = Ce(c, "name", 3, "");
  function o() {
    s(!s());
  }
  var D = ia(), p = r(D), d = r(p);
  br(d, () => c.children), t(p);
  var b = v(p, 2), y = r(b);
  tr(y), y.__click = o, y.__keydown = [oa, o], be(2), t(b), t(D), L(() => {
    dt(p, "width", c.labelWidth), qe(y, "name", $()), y.disabled = c.disabled, qe(y, "aria-checked", s()), qe(y, "aria-label", a());
  }), yr(y, s), i(de, D);
}
Zt(["click", "keydown"]);
var la = P("<div><p> </p> <!></div>"), da = P('<div class="success svelte-5kivuv"><!></div>'), ua = P('<div class="err svelte-5kivuv"> </div>'), ca = P('<div class="container svelte-5kivuv"><p> </p> <p><!></p> <div class="switch svelte-5kivuv"><!></div> <div class="switch svelte-5kivuv"><!></div> <!> <div class="bottom svelte-5kivuv"><!> <!></div></div>');
function Et(de, c) {
  Ne(c, true);
  let s = Ce(c, "webIdData", 15), a = We();
  const $ = "14rem";
  let o = N(""), D = N(false), p = N(!!s().custom_triples), d = ot(s().user_id);
  async function b() {
    n(o, "");
    let u = await ut(`/auth/v1/users/${s().user_id}/webid/data`, s());
    u.error ? n(o, m(u.error.message)) : (n(D, true), setTimeout(() => {
      n(D, false);
    }, 3e3));
  }
  var y = ca(), h = r(y), T = r(h, true);
  t(h);
  var G = v(h, 2), K = r(G);
  xr(K, { href: d, target: "_blank", children: (u, W) => {
    var f = me(), re = B(f);
    Ct(re, () => d.replace("/auth/", "/auth/<wbr/>")), i(u, f);
  }, $$slots: { default: true } }), t(G);
  var J = v(G, 2), ve = r(J);
  $t(ve, { ariaLabel: "E-Mail", labelWidth: $, get checked() {
    return s().expose_email;
  }, set checked(u) {
    s(s().expose_email = u, true);
  }, children: (u, W) => {
    be();
    var f = xe("E-Mail");
    i(u, f);
  }, $$slots: { default: true } }), t(J);
  var R = v(J, 2), X = r(R);
  $t(X, { get ariaLabel() {
    return a.account.webIdExpertMode;
  }, labelWidth: $, get checked() {
    return e(p);
  }, set checked(u) {
    n(p, m(u));
  }, children: (u, W) => {
    be();
    var f = xe();
    L(() => l(f, a.account.webIdExpertMode)), i(u, f);
  }, $$slots: { default: true } }), t(R);
  var ie = v(R, 2);
  {
    var F = (u) => {
      var W = la(), f = r(W), re = r(f, true);
      t(f);
      var H = v(f, 2);
      kr(H, { placeholder: "FOAF", rows: 15, get value() {
        return s().custom_triples;
      }, set value(ne) {
        s(s().custom_triples = ne, true);
      } }), t(W), L(() => l(re, a.account.webIdDescData)), Re(3, W, () => lr), i(u, W);
    };
    w(ie, (u) => {
      e(p) && u(F);
    });
  }
  var q = v(ie, 2), Z = r(q);
  Pe(Z, { onclick: b, children: (u, W) => {
    be();
    var f = xe();
    L(() => l(f, a.common.save)), i(u, f);
  }, $$slots: { default: true } });
  var te = v(Z, 2);
  {
    var V = (u) => {
      var W = da(), f = r(W);
      ct(f, {}), t(W), Re(3, W, () => Ve), i(u, W);
    }, M = (u) => {
      var W = me(), f = B(W);
      {
        var re = (H) => {
          var ne = ua(), g = r(ne, true);
          t(ne), L(() => l(g, e(o))), Re(3, ne, () => Ve), i(H, ne);
        };
        w(f, (H) => {
          e(o) && H(re);
        }, true);
      }
      i(u, W);
    };
    w(te, (u) => {
      e(D) ? u(V) : u(M, false);
    });
  }
  t(q), t(y), L(() => l(T, a.account.webIdDesc)), i(de, y), Ae();
}
function fa(de, c) {
  Ne(c, true);
  let s = ft("account"), a = _e(() => {
    var _a2;
    return (_a2 = s.get()) == null ? void 0 : _a2.user_id;
  });
  var $ = me(), o = B($);
  {
    var D = (p) => {
      Mt(p, { viewMode: "account", get userId() {
        return e(a);
      } });
    };
    w(o, (p) => {
      e(a) && p(D);
    });
  }
  i(de, $), Ae();
}
var _a = P("<h3> </h3>"), ma = P('<div class="headerPhone svelte-3a6iwi"><!></div> <div class="container svelte-3a6iwi"><!> <div class="innerPhone svelte-3a6iwi"><!></div></div>', 1), pa = P('<div clasS="info svelte-3a6iwi"><!></div>'), ga = P('<div class="flex gap-05"><!> </div>'), ha = P('<div class="wide svelte-3a6iwi"><!> <div class="container svelte-3a6iwi"><!> <div class="inner svelte-3a6iwi"><!></div></div> <div class="logout svelte-3a6iwi"><!></div></div>'), wa = P('<!> <div class="wrapper svelte-3a6iwi"><!></div>', 1);
function ba(de, c) {
  Ne(c, true);
  const s = (F) => {
    var q = _a(), Z = r(q, true);
    t(q), L(() => l(Z, `${a().given_name} ${a().family_name || ""}`)), i(F, q);
  };
  let a = Ce(c, "user", 15), $ = Ce(c, "webIdData", 15), o = We(), D = N(void 0), p = N(m([])), d = _e(() => {
    var _a2;
    if ((_a2 = a().account_type) == null ? void 0 : _a2.startsWith("federated")) return e(p).filter((F) => F.id === a().auth_provider_id)[0];
  }), b = _e(() => e(D) && e(D) < 560), y = _e(() => e(D) && e(D) < 1e3), h = N(m(o.account.navInfo)), T = _e(() => $() ? [o.account.navInfo, o.account.navMfa, o.account.devices, o.account.navEdit, o.common.password, "WebID", o.account.navLogout] : [o.account.navInfo, o.account.navMfa, o.account.devices, o.account.navEdit, o.common.password, o.account.navLogout]), G = _e(() => $() ? [o.account.navMfa, o.account.devices, o.account.navEdit, o.common.password, "WebID"] : [o.account.navMfa, o.account.devices, o.account.navEdit, o.common.password]);
  tt(() => {
    Ir("v").get() === "devices" && n(h, m(o.account.devices));
  }), Be(() => {
    e(b) || e(y) ? n(h, m(o.account.navInfo)) : n(h, m(o.account.navMfa));
  }), Be(() => {
    e(h) === o.account.navLogout && bt();
  });
  var K = wa(), J = B(K);
  Pr(J, { id: or, get value() {
    return e(p);
  }, set value(F) {
    n(p, m(F));
  } });
  var ve = v(J, 2), R = r(ve);
  {
    var X = (F) => {
      var q = ma(), Z = B(q), te = r(Z);
      s(te), t(Z);
      var V = v(Z, 2), M = r(V);
      xt(M, { get tabs() {
        return e(T);
      }, get selected() {
        return e(h);
      }, set selected(H) {
        n(h, m(H));
      } });
      var u = v(M, 2), W = r(u);
      {
        var f = (H) => {
          nt(H, { get providers() {
            return e(p);
          }, get authProvider() {
            return e(d);
          }, viewModePhone: true, get webIdData() {
            return $();
          }, get user() {
            return a();
          }, set user(ne) {
            a(ne);
          } });
        }, re = (H) => {
          var ne = me(), g = B(ne);
          {
            var k = (I) => {
              kt(I, { viewModePhone: true, get user() {
                return a();
              }, set user(A) {
                a(A);
              } });
            }, _ = (I) => {
              var A = me(), ue = B(A);
              {
                var O = (se) => {
                  It(se, { get user() {
                    return a();
                  }, get authProvider() {
                    return e(d);
                  }, viewModePhone: true });
                }, ae = (se) => {
                  var ye = me(), ke = B(ye);
                  {
                    var C = (E) => {
                      Pt(E, { get user() {
                        return a();
                      } });
                    }, x = (E) => {
                      var ee = me(), pe = B(ee);
                      {
                        var Y = (oe) => {
                          var ce = me(), $e = B(ce);
                          {
                            var le = (ge) => {
                              Et(ge, { get webIdData() {
                                return $();
                              }, set webIdData(De) {
                                $(De);
                              } });
                            };
                            w($e, (ge) => {
                              $() && ge(le);
                            });
                          }
                          i(oe, ce);
                        }, j = (oe) => {
                          var ce = me(), $e = B(ce);
                          {
                            var le = (ge) => {
                              Mt(ge, { get userId() {
                                return a().id;
                              } });
                            };
                            w($e, (ge) => {
                              e(h) === o.account.devices && ge(le);
                            }, true);
                          }
                          i(oe, ce);
                        };
                        w(pe, (oe) => {
                          e(h) === "WebID" ? oe(Y) : oe(j, false);
                        }, true);
                      }
                      i(E, ee);
                    };
                    w(ke, (E) => {
                      e(h) === o.account.navMfa ? E(C) : E(x, false);
                    }, true);
                  }
                  i(se, ye);
                };
                w(ue, (se) => {
                  e(h) === o.common.password ? se(O) : se(ae, false);
                }, true);
              }
              i(I, A);
            };
            w(g, (I) => {
              e(h) === o.account.navEdit ? I(k) : I(_, false);
            }, true);
          }
          i(H, ne);
        };
        w(W, (H) => {
          e(h) === o.account.navInfo ? H(f) : H(re, false);
        });
      }
      t(u), t(V), i(F, q);
    }, ie = (F) => {
      var q = ha(), Z = r(q);
      {
        var te = (k) => {
          var _ = pa(), I = r(_);
          nt(I, { get webIdData() {
            return $();
          }, get providers() {
            return e(p);
          }, get authProvider() {
            return e(d);
          }, get user() {
            return a();
          }, set user(A) {
            a(A);
          } }), t(_), i(k, _);
        };
        w(Z, (k) => {
          e(y) || k(te);
        });
      }
      var V = v(Z, 2), M = r(V);
      const u = _e(() => e(y) ? e(T) : e(G));
      xt(M, { get tabs() {
        return e(u);
      }, center: true, get selected() {
        return e(h);
      }, set selected(k) {
        n(h, m(k));
      } });
      var W = v(M, 2), f = r(W);
      {
        var re = (k) => {
          nt(k, { get webIdData() {
            return $();
          }, get providers() {
            return e(p);
          }, get authProvider() {
            return e(d);
          }, get user() {
            return a();
          }, set user(_) {
            a(_);
          } });
        }, H = (k) => {
          var _ = me(), I = B(_);
          {
            var A = (O) => {
              kt(O, { get user() {
                return a();
              }, set user(ae) {
                a(ae);
              } });
            }, ue = (O) => {
              var ae = me(), se = B(ae);
              {
                var ye = (C) => {
                  It(C, { get user() {
                    return a();
                  }, get authProvider() {
                    return e(d);
                  } });
                }, ke = (C) => {
                  var x = me(), E = B(x);
                  {
                    var ee = (Y) => {
                      Pt(Y, { get user() {
                        return a();
                      } });
                    }, pe = (Y) => {
                      var j = me(), oe = B(j);
                      {
                        var ce = (le) => {
                          var ge = me(), De = B(ge);
                          {
                            var ze = (Ee) => {
                              Et(Ee, { get webIdData() {
                                return $();
                              }, set webIdData(Me) {
                                $(Me);
                              } });
                            };
                            w(De, (Ee) => {
                              $() && Ee(ze);
                            });
                          }
                          i(le, ge);
                        }, $e = (le) => {
                          var ge = me(), De = B(ge);
                          {
                            var ze = (Ee) => {
                              fa(Ee, {});
                            };
                            w(De, (Ee) => {
                              e(h) === o.account.devices && Ee(ze);
                            }, true);
                          }
                          i(le, ge);
                        };
                        w(oe, (le) => {
                          e(h) === "WebID" ? le(ce) : le($e, false);
                        }, true);
                      }
                      i(Y, j);
                    };
                    w(E, (Y) => {
                      e(h) === o.account.navMfa ? Y(ee) : Y(pe, false);
                    }, true);
                  }
                  i(C, x);
                };
                w(se, (C) => {
                  e(h) === o.common.password ? C(ye) : C(ke, false);
                }, true);
              }
              i(O, ae);
            };
            w(I, (O) => {
              e(h) === o.account.navEdit ? O(A) : O(ue, false);
            }, true);
          }
          i(k, _);
        };
        w(f, (k) => {
          e(h) === o.account.navInfo ? k(re) : k(H, false);
        });
      }
      t(W), t(V);
      var ne = v(V, 2), g = r(ne);
      Pe(g, { level: -3, onclick: bt, children: (k, _) => {
        var I = ga(), A = r(I);
        $r(A, {});
        var ue = v(A);
        t(I), L(() => {
          qe(I, "title", o.account.navLogout), l(ue, ` ${o.account.navLogout ?? ""}`);
        }), i(k, I);
      }, $$slots: { default: true } }), t(ne), t(q), i(F, q);
    };
    w(R, (F) => {
      e(b) ? F(X) : F(ie, false);
    });
  }
  t(ve), rr("innerWidth", (F) => n(D, m(F))), i(de, K), Ae();
}
var ya = P("<!> <!> <!>", 1);
function is(de, c) {
  Ne(c, true);
  let s = We(), a = ft("account"), $ = N(void 0), o = N(void 0), D = N(false);
  Be(() => {
    let d = a.get();
    d && p(d);
  });
  async function p(d) {
    const b = d.user_id;
    if (b) {
      let y = await Promise.all([Ke(`/auth/v1/users/${b}`), Ke(`/auth/v1/users/${b}/webid/data`)]);
      y[0].body ? n($, m(y[0].body)) : ir("account"), y[1].body ? n(o, m(y[1].body)) : y[1].status === 404 && n(o, m({ user_id: b, expose_email: false })), n(D, true);
    } else console.error("no user_id in session");
  }
  er((d) => {
    L(() => {
      var _a2;
      return Xt.title = `${(s == null ? void 0 : s.account.account) || "Account"} ${((_a2 = e($)) == null ? void 0 : _a2.email) ?? ""}`;
    });
  }), Er(de, { children: (d, b) => {
    Tr(d, { children: (y, h) => {
      var T = ya(), G = B(T);
      {
        var K = (R) => {
          ba(R, { get user() {
            return e($);
          }, set user(X) {
            n($, m(X));
          }, get webIdData() {
            return e(o);
          }, set webIdData(X) {
            n(o, m(X));
          } });
        };
        w(G, (R) => {
          e(D) && a && e($) && R(K);
        });
      }
      var J = v(G, 2);
      Cr(J, { absolute: true });
      var ve = v(J, 2);
      Dr(ve, { absolute: true }), i(y, T);
    } });
  } }), Ae();
}
export {
  is as component
};
