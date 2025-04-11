import { t as x, a as u, e as he, d as Re } from "../chunks/BxmJRzoY.js";
import { p as Ee, j as C, a4 as ne, k as e, c as r, s as v, r as t, t as D, a as Te, a3 as ge, l as o, f as ye, a1 as Le, a0 as Ze, a5 as Se, a2 as Qt } from "../chunks/w0HvPX0p.js";
import { s as l, d as Xt, h as Zt } from "../chunks/BzP2S3Z_.js";
import { i as h } from "../chunks/iO9_dPNE.js";
import { a as lt, b as S, c as F, s as ze, r as er } from "../chunks/BdbQ6g_y.js";
import { b as tr } from "../chunks/CJLp5kwW.js";
import { p as Ie } from "../chunks/C6GSeq7M.js";
import { l as Qe, Q as ot, J as rr, P as ar, K as sr, z as vr, S as wt, D as or, r as nr } from "../chunks/B21bTIl7.js";
import { e as Et } from "../chunks/S81raU5Y.js";
import { h as Tt } from "../chunks/C2ZdIFW_.js";
import { C as Xe } from "../chunks/DwsgkunL.js";
import { B as be, t as Me, f as Ne, s as ir } from "../chunks/C8YTstTD.js";
import { M as lr } from "../chunks/CEj6JA72.js";
import { u as De } from "../chunks/0cG6LBdy.js";
import { c as Ct, b as nt, d as ut, f as Ve } from "../chunks/UPFlzoow.js";
import { B as ur } from "../chunks/CybldwDO.js";
import { U as dr, a as cr, D as Dt } from "../chunks/pkyASokb.js";
import { I as Ce } from "../chunks/DVXwAhn3.js";
import { F as fr } from "../chunks/BEbxeS8S.js";
import { b as it, k as _r, l as bt, m as pr } from "../chunks/gfDO7tLr.js";
import { I as dt } from "../chunks/CTshzOVc.js";
import { I as mr } from "../chunks/WLAGCpPH.js";
import { u as ct } from "../chunks/CxfVL694.js";
import { w as gr } from "../chunks/BhNhSJcg.js";
import { W as Mt } from "../chunks/BjT2-OU-.js";
import { P as hr } from "../chunks/DbyBiYcN.js";
import { I as st } from "../chunks/iedauS3r.js";
import { s as wr } from "../chunks/DM69BKKN.js";
import { c as br } from "../chunks/Cxw7xmE1.js";
import { A as yr } from "../chunks/DNZT2nu7.js";
import { I as xr } from "../chunks/Dc2IDHgC.js";
import { T as kr } from "../chunks/Dhcsa8BW.js";
import { u as Pr } from "../chunks/DYaEteNC.js";
import { T as yt } from "../chunks/BdAKL3gn.js";
import { I as Ir } from "../chunks/DKnUPdjZ.js";
import { M as $r } from "../chunks/DKM0QPz5.js";
import { C as Er } from "../chunks/QNragXLc.js";
import { T as Tr } from "../chunks/B1f0afjj.js";
import { L as Cr } from "../chunks/D0XpefKJ.js";
var Dr = x('<div class="link-err value svelte-1tea8v4"> </div>'), Mr = x('<div class="fed-btn svelte-1tea8v4"><!> <!></div>'), Lr = x('<h3> </h3> <p> </p> <div class="providers svelte-1tea8v4"></div>', 1), Nr = x("<!> <!>", 1), Ar = x('<div><div> </div> <span class="value svelte-1tea8v4"> </span></div>'), Wr = x('<div><div> </div> <span class="value svelte-1tea8v4"> </span></div>'), zr = x('<div><div>WebID:</div> <span class="value svelte-1tea8v4"><a target="_blank"><!></a></span></div>'), Rr = x('<div><div><div></div> <!></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <div><div class="value svelte-1tea8v4"> </div> <!></div></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <!></div> <div><div> </div> <!></div> <div><div> </div> <!></div> <!> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <!> <!></div>');
function vt(ve, f) {
  Ee(f, true);
  let s = Ie(f, "user", 15), a = De(), k = C(false), n = C(false), E = ne(() => {
    var _a2;
    return (_a2 = s().account_type) == null ? void 0 : _a2.startsWith("federated");
  }), m = ne(() => {
    var _a2;
    return e(E) ? `${s().account_type}: ${((_a2 = f.authProvider) == null ? void 0 : _a2.name) || ""}` : s().account_type;
  }), i = ne(() => f.viewModePhone ? "rowPhone" : "row"), w = ne(() => f.viewModePhone ? "labelPhone" : "label"), P = ne(() => {
    let W = s().given_name[0];
    return s().family_name && s().family_name.length > 0 && (W += s().family_name[0]), W;
  });
  function d(W) {
    rr(64, (ee, { challenge: oe, verifier: _e }) => {
      ee || (localStorage.setItem(ar, _e), I(W, oe));
    });
  }
  async function I(W, ee) {
    let oe = { email: s().email, client_id: "rauthy", redirect_uri: window.location.href, provider_id: W, pkce_challenge: ee }, _e = await nt(`/auth/v1/providers/${W}/link`, oe);
    if (_e.text) {
      sr(_e.text);
      let me = _e.headers.get("location");
      me && (window.location.href = me);
    } else console.error(_e.error);
  }
  async function G() {
    let W = await Ct("/auth/v1/providers/link");
    W.body ? s(W.body) : (console.error(W.error), o(k, true));
  }
  var H = Rr(), J = r(H);
  lt(J, "", {}, { margin: ".5rem 0" });
  var re = r(J), z = v(re, 2);
  dr(z, { get fallbackCharacters() {
    return e(P);
  }, get userId() {
    return s().id;
  }, size: "large", get pictureId() {
    return s().picture_id;
  }, set pictureId(W) {
    s(s().picture_id = W, true);
  } }), t(J);
  var X = v(J, 2), ae = r(X), q = r(ae, true);
  t(ae);
  var B = v(ae, 2), Z = r(B, true);
  t(B), t(X);
  var te = v(X, 2), A = r(te), T = r(A, true);
  t(A);
  var _ = v(A, 2), R = r(_, true);
  t(_), t(te);
  var p = v(te, 2), Q = r(p), Y = r(Q, true);
  t(Q);
  var ue = v(Q, 2), g = r(ue, true);
  t(ue), t(p);
  var y = v(p, 2), c = r(y), M = r(c);
  t(c);
  var V = v(c, 2), j = r(V, true);
  t(V), t(y);
  var U = v(y, 2), de = r(U), pe = r(de, true);
  t(de);
  var ie = v(de, 2), le = r(ie), N = r(le, true);
  t(le);
  var b = v(le, 2);
  {
    var $ = (W) => {
      var ee = Mr(), oe = r(ee);
      be(oe, { get ariaLabel() {
        return a.account.providerUnlink;
      }, level: 3, onclick: G, children: (xe, Pe) => {
        ge();
        var $e = he();
        D(() => l($e, a.account.providerUnlink)), u(xe, $e);
      }, $$slots: { default: true } });
      var _e = v(oe, 2);
      {
        var me = (xe) => {
          var Pe = Dr(), $e = r(Pe, true);
          t(Pe), D(() => l($e, a.account.providerUnlinkDesc)), u(xe, Pe);
        };
        h(_e, (xe) => {
          e(k) && xe(me);
        });
      }
      t(ee), u(W, ee);
    }, K = (W, ee) => {
      {
        var oe = (_e) => {
          var me = Nr(), xe = ye(me);
          be(xe, { level: 2, onclick: () => o(n, true), children: ($e, Yt) => {
            ge();
            var We = he();
            D(() => l(We, a.account.providerLink)), u($e, We);
          }, $$slots: { default: true } });
          var Pe = v(xe, 2);
          lr(Pe, { get showModal() {
            return e(n);
          }, set showModal($e) {
            o(n, $e, true);
          }, children: ($e, Yt) => {
            var We = Lr(), tt = ye(We), jt = r(tt, true);
            t(tt);
            var rt = v(tt, 2), Gt = r(rt, true);
            t(rt);
            var gt = v(rt, 2);
            Et(gt, 21, () => f.providers, (at) => at.id, (at, ht) => {
              const Jt = ne(() => `${a.account.providerLink}: ${e(ht).name}`);
              ur(at, { get ariaLabel() {
                return e(Jt);
              }, get provider() {
                return e(ht);
              }, onclick: d });
            }), t(gt), D(() => {
              l(jt, a.account.providerLink), l(Gt, a.account.providerLinkDesc);
            }), u($e, We);
          }, $$slots: { default: true } }), u(_e, me);
        };
        h(W, (_e) => {
          f.providers.length > 0 && _e(oe);
        }, ee);
      }
    };
    h(b, (W) => {
      e(E) ? W($) : W(K, false);
    });
  }
  t(ie), t(U);
  var se = v(U, 2), O = r(se), L = r(O, true);
  t(O);
  var ce = v(O, 2), fe = r(ce, true);
  t(ce), t(se);
  var we = v(se, 2), ke = r(we), Ae = r(ke, true);
  t(ke);
  var et = v(ke, 2), Ue = r(et, true);
  t(et), t(we);
  var Fe = v(we, 2), qe = r(Fe), Lt = r(qe, true);
  t(qe);
  var Nt = v(qe, 2);
  const At = ne(() => !!s().webauthn_user_id);
  Xe(Nt, { get checked() {
    return e(At);
  } }), t(Fe);
  var Be = v(Fe, 2), Ke = r(Be), Wt = r(Ke, true);
  t(Ke);
  var zt = v(Ke, 2);
  Xe(zt, { get checked() {
    return s().enabled;
  } }), t(Be);
  var Oe = v(Be, 2), He = r(Oe), Rt = r(He, true);
  t(He);
  var St = v(He, 2);
  Xe(St, { get checked() {
    return s().email_verified;
  } }), t(Oe);
  var ft = v(Oe, 2);
  {
    var Vt = (W) => {
      var ee = Ar(), oe = r(ee), _e = r(oe, true);
      t(oe);
      var me = v(oe, 2), xe = r(me, true);
      t(me), t(ee), D((Pe) => {
        S(ee, 1, F(e(i)), "svelte-1tea8v4"), S(oe, 1, F(e(w)), "svelte-1tea8v4"), l(_e, a.account.lastLogin), l(xe, Pe);
      }, [() => Qe(s().last_login)]), u(W, ee);
    };
    h(ft, (W) => {
      s().last_login && W(Vt);
    });
  }
  var Ye = v(ft, 2), je = r(Ye), Ut = r(je, true);
  t(je);
  var _t = v(je, 2), Ft = r(_t, true);
  t(_t), t(Ye);
  var Ge = v(Ye, 2), Je = r(Ge), qt = r(Je, true);
  t(Je);
  var pt = v(Je, 2), Bt = r(pt, true);
  t(pt), t(Ge);
  var mt = v(Ge, 2);
  {
    var Kt = (W) => {
      var ee = Wr(), oe = r(ee), _e = r(oe, true);
      t(oe);
      var me = v(oe, 2), xe = r(me, true);
      t(me), t(ee), D((Pe) => {
        S(ee, 1, F(e(i)), "svelte-1tea8v4"), S(oe, 1, F(e(w)), "svelte-1tea8v4"), l(_e, a.account.userExpiry), l(xe, Pe);
      }, [() => Qe(s().user_expires)]), u(W, ee);
    };
    h(mt, (W) => {
      s().user_expires && W(Kt);
    });
  }
  var Ot = v(mt, 2);
  {
    var Ht = (W) => {
      var ee = zr(), oe = r(ee), _e = v(oe, 2), me = r(_e), xe = r(me);
      Tt(xe, () => ot(s().id).replace("/auth/", "/auth/<wbr/>")), t(me), t(_e), t(ee), D((Pe) => {
        S(ee, 1, F(e(i)), "svelte-1tea8v4"), S(oe, 1, F(e(w)), "svelte-1tea8v4"), ze(me, "href", Pe);
      }, [() => ot(s().id)]), u(W, ee);
    };
    h(Ot, (W) => {
      f.webIdData && W(Ht);
    });
  }
  t(H), D((W, ee) => {
    S(J, 1, F(e(i)), "svelte-1tea8v4"), S(re, 1, F(e(w)), "svelte-1tea8v4"), S(X, 1, F(e(i)), "svelte-1tea8v4"), S(ae, 1, F(e(w)), "svelte-1tea8v4"), l(q, a.common.email), l(Z, s().email), S(te, 1, F(e(i)), "svelte-1tea8v4"), S(A, 1, F(e(w)), "svelte-1tea8v4"), l(T, a.account.givenName), l(R, s().given_name), S(p, 1, F(e(i)), "svelte-1tea8v4"), S(Q, 1, F(e(w)), "svelte-1tea8v4"), l(Y, a.account.familyName), l(g, s().family_name), S(y, 1, F(e(i)), "svelte-1tea8v4"), S(c, 1, F(e(w)), "svelte-1tea8v4"), l(M, `${a.account.user ?? ""} ID`), l(j, s().id), S(U, 1, F(e(i)), "svelte-1tea8v4"), S(de, 1, F(e(w)), "svelte-1tea8v4"), l(pe, a.account.accType), l(N, e(m) || ""), S(se, 1, F(e(i)), "svelte-1tea8v4"), S(O, 1, F(e(w)), "svelte-1tea8v4"), l(L, a.account.roles), l(fe, s().roles || "None"), S(we, 1, F(e(i)), "svelte-1tea8v4"), S(ke, 1, F(e(w)), "svelte-1tea8v4"), l(Ae, a.account.groups), l(Ue, s().groups || "None"), S(Fe, 1, F(e(i)), "svelte-1tea8v4"), S(qe, 1, F(e(w)), "svelte-1tea8v4"), l(Lt, a.account.mfaActivated), S(Be, 1, F(e(i)), "svelte-1tea8v4"), S(Ke, 1, F(e(w)), "svelte-1tea8v4"), l(Wt, a.account.userEnabled), S(Oe, 1, F(e(i)), "svelte-1tea8v4"), S(He, 1, F(e(w)), "svelte-1tea8v4"), l(Rt, a.account.emailVerified), S(Ye, 1, F(e(i)), "svelte-1tea8v4"), S(je, 1, F(e(w)), "svelte-1tea8v4"), l(Ut, a.account.passwordExpiry), l(Ft, W), S(Ge, 1, F(e(i)), "svelte-1tea8v4"), S(Je, 1, F(e(w)), "svelte-1tea8v4"), l(qt, a.account.userCreated), l(Bt, ee);
  }, [() => s().password_expires && Qe(s().password_expires) || a.common.never, () => Qe(s().created_at)]), u(ve, H), Te();
}
var Sr = x('<div class="success svelte-4ogr3z"><!></div>'), Vr = x("<p> </p>"), Ur = x('<div class="err svelte-4ogr3z"> </div>'), Fr = x('<div class="formInner svelte-4ogr3z"><div><!> <!> <!> <!></div> <div><!> <!> <!> <!> <!></div></div> <div class="bottom svelte-4ogr3z"><div><!></div> <!></div> <!>', 1), qr = x('<div class="container svelte-4ogr3z"><!></div>');
function xt(ve, f) {
  var _a2;
  Ee(f, true);
  let s = Ie(f, "user", 15);
  ((_a2 = s().user_values) == null ? void 0 : _a2.birthdate) || s(s().user_values.birthdate = "", true);
  let a = De(), k = C(""), n = C(false), E = C(false);
  async function m(d, I) {
    var _a3, _b;
    const G = I.get("email"), H = I.get("family_name") || void 0, J = I.get("given_name") || void 0, re = I.get("birthdate") || void 0, z = ((_a3 = I.get("phone")) == null ? void 0 : _a3.replaceAll(" ", "")) || void 0, X = I.get("street") || void 0, ae = I.get("zip") || void 0, q = ae ? Number.parseInt(ae) : void 0, B = I.get("city") || void 0, Z = I.get("country") || void 0;
    let te = { email: G, family_name: H, given_name: J };
    (re || z || X || q || B || Z) && (te.user_values = { birthdate: re, phone: z, street: X, zip: q, city: B, country: Z });
    let A = await ut(`/auth/v1/users/${s().id}/self`, te);
    if (A.body) {
      o(n, true), ((_b = A.body.user_values) == null ? void 0 : _b.birthdate) || (A.body.user_values.birthdate = ""), s(A.body);
      let T = 3e3;
      A.status === 202 && (o(E, true), T = 3e4), setTimeout(() => {
        o(n, false);
      }, T);
    } else A.error && (console.error(A.error), o(k, A.error.message, true));
  }
  var i = qr(), w = r(i);
  const P = ne(() => `/auth/v1/users/${s().id}/self`);
  fr(w, { get action() {
    return e(P);
  }, onSubmit: m, children: (d, I) => {
    var G = Fr(), H = ye(G), J = r(H), re = r(J);
    Ce(re, { typ: "email", name: "email", get label() {
      return a.common.email;
    }, get placeholder() {
      return a.common.email;
    }, get value() {
      return s().email;
    }, required: true });
    var z = v(re, 2);
    Ce(z, { name: "given_name", autocomplete: "given-name", get label() {
      return a.account.givenName;
    }, get placeholder() {
      return a.account.givenName;
    }, get value() {
      return s().given_name;
    }, required: true, maxLength: 32, pattern: it });
    var X = v(z, 2);
    Ce(X, { name: "family_name", autocomplete: "family-name", get label() {
      return a.account.familyName;
    }, get placeholder() {
      return a.account.familyName;
    }, get value() {
      return s().family_name;
    }, maxLength: 32, pattern: it });
    var ae = v(X, 2);
    mr(ae, { name: "birthdate", get label() {
      return a.account.birthdate;
    }, withDelete: true, get value() {
      return s().user_values.birthdate;
    }, set value(c) {
      s(s().user_values.birthdate = c, true);
    } }), t(J);
    var q = v(J, 2), B = r(q);
    Ce(B, { name: "street", autocomplete: "street-address", get label() {
      return a.account.street;
    }, get placeholder() {
      return a.account.street;
    }, get value() {
      return s().user_values.street;
    }, maxLength: 48, pattern: _r });
    var Z = v(B, 2);
    Ce(Z, { typ: "number", name: "zip", autocomplete: "postal-code", get label() {
      return a.account.zip;
    }, get placeholder() {
      return a.account.zip;
    }, get value() {
      return s().user_values.zip;
    }, min: "1000", max: "9999999" });
    var te = v(Z, 2);
    Ce(te, { name: "city", autocomplete: "address-level2", get label() {
      return a.account.city;
    }, get placeholder() {
      return a.account.city;
    }, get value() {
      return s().user_values.city;
    }, maxLength: 48, pattern: bt });
    var A = v(te, 2);
    Ce(A, { name: "country", autocomplete: "country", get label() {
      return a.account.country;
    }, get placeholder() {
      return a.account.country;
    }, get value() {
      return s().user_values.country;
    }, maxLength: 48, pattern: bt });
    var T = v(A, 2);
    Ce(T, { name: "phone", autocomplete: "tel", get label() {
      return a.account.phone;
    }, get placeholder() {
      return a.account.phone;
    }, get value() {
      return s().user_values.phone;
    }, maxLength: 32, pattern: pr }), t(q), t(H);
    var _ = v(H, 2), R = r(_), p = r(R);
    be(p, { type: "submit", children: (c, M) => {
      ge();
      var V = he();
      D(() => l(V, a.common.save)), u(c, V);
    }, $$slots: { default: true } }), t(R);
    var Q = v(R, 2);
    {
      var Y = (c) => {
        var M = Sr(), V = r(M);
        dt(V, {}), t(M), Me(3, M, () => Ne), u(c, M);
      };
      h(Q, (c) => {
        e(n) && c(Y);
      });
    }
    t(_);
    var ue = v(_, 2);
    {
      var g = (c) => {
        var M = Vr(), V = r(M, true);
        t(M), D(() => l(V, a.account.emailUpdateConfirm)), u(c, M);
      }, y = (c, M) => {
        {
          var V = (j) => {
            var U = Ur(), de = r(U, true);
            t(U), D(() => l(de, e(k))), Me(3, U, () => Ne), u(j, U);
          };
          h(c, (j) => {
            e(k) && j(V);
          }, M);
        }
      };
      h(ue, (c) => {
        e(E) ? c(g) : c(y, false);
      });
    }
    u(d, G);
  }, $$slots: { default: true } }), t(i), u(ve, i), Te();
}
var Br = x('<div class="err svelte-16mlupu"><b>Your browser does not support Webauthn credentials and must be updated.</b></div>'), Kr = x('<!> <div class="regBtns svelte-16mlupu"><!> <!></div>', 1), Or = x('<div class="regNewBtn svelte-16mlupu"><!></div>'), Hr = x('<div class="keysHeader svelte-16mlupu"> </div>'), Yr = x('<div class="button svelte-16mlupu"><!></div>'), jr = x('<!> <p class="svelte-16mlupu"> <br><br> </p> <!> <!> <div class="keysContainer svelte-16mlupu"></div> <!> <div> </div>', 1), Gr = x('<div class="container svelte-16mlupu"><!></div>');
function kt(ve, f) {
  Ee(f, true);
  const s = "credentials" in navigator;
  let a = De(), k = ct("account"), n = ne(() => {
    var _a2;
    return (_a2 = k.get()) == null ? void 0 : _a2.user_id;
  }), E = C(void 0), m = C(false), i = C(""), w = C(false), P = C(f.user.account_type === "password"), d = C(void 0), I = C(""), G = C(false), H = C(Le([]));
  Ze(() => {
    re();
  }), Se(() => {
    e(H).length > 0 && f.user.account_type === "passkey" && o(P, e(H).length > 1);
  }), Se(() => {
    var _a2;
    (_a2 = e(E)) == null ? void 0 : _a2.focus();
  });
  function J() {
    o(m, false), o(i, "");
  }
  async function re() {
    var _a2;
    let T = await Ve(`/auth/v1/users/${(_a2 = k.get()) == null ? void 0 : _a2.user_id}/webauthn`);
    T.body ? o(H, T.body, true) : console.error(T.error);
  }
  async function z() {
    if (J(), e(G) || !e(n)) return;
    if (e(I).length < 1) {
      o(m, true), o(i, a.mfa.passkeyNameErr, true);
      return;
    }
    let T = await gr(e(n), e(I), a.authorize.invalidKeyUsed, a.authorize.requestExpired);
    T.error ? (o(m, true), o(i, `${a.mfa.errorReg} - ${T.error}`)) : (o(w, false), o(I, ""), await re());
  }
  async function X(T) {
    var _a2;
    let _ = await Ct(`/auth/v1/users/${f.user.id}/webauthn/delete/${T}`);
    _.status === 200 ? await re() : o(i, ((_a2 = _.error) == null ? void 0 : _a2.message) || "Error", true);
  }
  function ae(T) {
    o(d, void 0), o(m, true), o(i, T, true), setTimeout(() => {
      o(m, false), o(i, "");
    }, 5e3);
  }
  function q(T) {
    o(d, void 0), o(i, a.mfa.testSuccess, true), setTimeout(() => {
      o(i, "");
    }, 3e3);
  }
  var B = Gr(), Z = r(B);
  {
    var te = (T) => {
      var _ = Br();
      u(T, _);
    }, A = (T) => {
      var _ = jr(), R = ye(_);
      {
        var p = (N) => {
          Mt(N, { get userId() {
            return f.user.id;
          }, get purpose() {
            return e(d);
          }, onSuccess: q, onError: ae });
        };
        h(R, (N) => {
          e(d) && N(p);
        });
      }
      var Q = v(R, 2), Y = r(Q), ue = v(Y, 3);
      t(Q);
      var g = v(Q, 2);
      {
        var y = (N) => {
          var b = Kr(), $ = ye(b);
          Ce($, { autocomplete: "off", get label() {
            return a.mfa.passkeyName;
          }, get placeholder() {
            return a.mfa.passkeyName;
          }, maxLength: 32, pattern: it, onEnter: z, get ref() {
            return e(E);
          }, set ref(L) {
            o(E, L, true);
          }, get value() {
            return e(I);
          }, set value(L) {
            o(I, L, true);
          }, get isError() {
            return e(G);
          }, set isError(L) {
            o(G, L, true);
          } });
          var K = v($, 2), se = r(K);
          be(se, { onclick: z, children: (L, ce) => {
            ge();
            var fe = he();
            D(() => l(fe, a.mfa.register)), u(L, fe);
          }, $$slots: { default: true } });
          var O = v(se, 2);
          be(O, { level: 3, onclick: () => o(w, false), children: (L, ce) => {
            ge();
            var fe = he();
            D(() => l(fe, a.common.cancel)), u(L, fe);
          }, $$slots: { default: true } }), t(K), u(N, b);
        }, c = (N) => {
          var b = Or(), $ = r(b);
          const K = ne(() => e(H).length === 0 ? 1 : 2);
          be($, { get level() {
            return e(K);
          }, onclick: () => o(w, true), children: (se, O) => {
            ge();
            var L = he();
            D(() => l(L, a.mfa.registerNew)), u(se, L);
          }, $$slots: { default: true } }), t(b), u(N, b);
        };
        h(g, (N) => {
          e(w) ? N(y) : N(c, false);
        });
      }
      var M = v(g, 2);
      {
        var V = (N) => {
          var b = Hr(), $ = r(b, true);
          t(b), D(() => l($, a.mfa.registerdKeys)), u(N, b);
        };
        h(M, (N) => {
          e(H).length > 0 && N(V);
        });
      }
      var j = v(M, 2);
      Et(j, 21, () => e(H), (N) => N.name, (N, b) => {
        cr(N, { get passkey() {
          return e(b);
        }, get showDelete() {
          return e(P);
        }, onDelete: X });
      }), t(j);
      var U = v(j, 2);
      {
        var de = (N) => {
          var b = Yr(), $ = r(b);
          be($, { onclick: () => o(d, "Test"), children: (K, se) => {
            ge();
            var O = he();
            D(() => l(O, a.mfa.test)), u(K, O);
          }, $$slots: { default: true } }), t(b), u(N, b);
        };
        h(U, (N) => {
          e(H).length > 0 && N(de);
        });
      }
      var pe = v(U, 2);
      let ie;
      var le = r(pe, true);
      t(pe), D((N) => {
        l(Y, `${a.mfa.p1 ?? ""} `), l(ue, ` ${a.mfa.p2 ?? ""}`), ie = S(pe, 1, "svelte-16mlupu", null, ie, N), l(le, e(i));
      }, [() => ({ success: !e(m), err: e(m) })]), u(T, _);
    };
    h(Z, (T) => {
      s ? T(A, false) : T(te);
    });
  }
  t(B), u(ve, B), Te();
}
var Jr = x('<div class="container svelte-ue7bk2"><!> <!> <!> <!> <div class="btn svelte-ue7bk2"><!></div> <div class="err svelte-ue7bk2"> </div></div>');
function Qr(ve, f) {
  Ee(f, true);
  let s = Ie(f, "passwords", 15), a = Ie(f, "hideCurrentPassword", 3, false), k = Ie(f, "isValid", 15), n = De();
  k(I);
  let E = C(false), m = C(""), i = C(void 0), w = ne(() => {
    var _a2;
    return ((_a2 = s().new) == null ? void 0 : _a2.length) > 6 && s().new === s().newConfirm;
  }), P = C(void 0), d = C(void 0);
  Ze(async () => {
    let z = await Ve("/auth/v1/password_policy");
    z.body ? o(i, z.body, true) : console.error(z.error);
  });
  function I() {
    return o(m, ""), !a() && !s().current ? (o(m, n.account.passwordCurrReq, true), false) : s().new ? s().newConfirm ? e(E) ? s().new.length > 256 ? (o(m, "max 256"), false) : s().new !== s().newConfirm ? (o(m, n.account.passwordNoMatch, true), false) : true : (o(m, n.account.passwordPolicyFollow, true), false) : (o(m, n.account.passwordNewReq, true), false) : (o(m, n.account.passwordNewReq, true), false);
  }
  function G() {
    if (o(m, ""), e(i)) {
      let z = vr(e(i));
      s(s().new = z, true), s(s().newConfirm = z, true);
    }
    requestAnimationFrame(() => {
      var _a2, _b;
      (_a2 = e(P)) == null ? void 0 : _a2(), (_b = e(d)) == null ? void 0 : _b();
    });
  }
  var H = Re(), J = ye(H);
  {
    var re = (z) => {
      var X = Jr(), ae = r(X);
      hr(ae, { get policy() {
        return e(i);
      }, get password() {
        return s().new;
      }, get accepted() {
        return e(E);
      }, set accepted(p) {
        o(E, p, true);
      } });
      var q = v(ae, 2);
      {
        var B = (p) => {
          st(p, { autocomplete: "current-password", get label() {
            return n.account.passwordCurr;
          }, get placeholder() {
            return n.account.passwordCurr;
          }, onInput: I, get width() {
            return f.inputWidth;
          }, get value() {
            return s().current;
          }, set value(Q) {
            s(s().current = Q, true);
          } });
        };
        h(q, (p) => {
          a() || p(B);
        });
      }
      var Z = v(q, 2);
      st(Z, { autocomplete: "new-password", get label() {
        return n.account.passwordNew;
      }, get placeholder() {
        return n.account.passwordNew;
      }, onInput: I, get showCopy() {
        return e(w);
      }, get width() {
        return f.inputWidth;
      }, get value() {
        return s().new;
      }, set value(p) {
        s(s().new = p, true);
      }, get reportValidity() {
        return e(P);
      }, set reportValidity(p) {
        o(P, p, true);
      } });
      var te = v(Z, 2);
      st(te, { autocomplete: "new-password", get label() {
        return n.account.passwordConfirm;
      }, get placeholder() {
        return n.account.passwordConfirm;
      }, onInput: I, get showCopy() {
        return e(w);
      }, get width() {
        return f.inputWidth;
      }, get value() {
        return s().newConfirm;
      }, set value(p) {
        s(s().newConfirm = p, true);
      }, get reportValidity() {
        return e(d);
      }, set reportValidity(p) {
        o(d, p, true);
      } });
      var A = v(te, 2), T = r(A);
      be(T, { onclick: G, level: 2, children: (p, Q) => {
        ge();
        var Y = he();
        D(() => l(Y, n.account.generateRandom)), u(p, Y);
      }, $$slots: { default: true } }), t(A);
      var _ = v(A, 2), R = r(_, true);
      t(_), t(X), D(() => l(R, e(m))), u(z, X);
    };
    h(J, (z) => {
      e(i) && z(re);
    });
  }
  return u(ve, H), Te({ isPwdValid: I });
}
var Xr = x('<div class="m-05 svelte-1h4zvzu"><p> </p> <p><b> </b></p> <div></div> <p> </p> <!></div>'), Zr = x("<p> </p> <p> </p> <p> </p> <div><!></div>", 1), ea = x('<div class="success svelte-1h4zvzu"><!></div>'), ta = x('<div class="err"> </div>'), ra = x('<div class="cancel"><!></div>'), aa = x('<div class="convertPasskey svelte-1h4zvzu"><h3> </h3> <p> </p> <!></div>'), sa = x('<div><!> <div class="save svelte-1h4zvzu"><!> <!></div></div> <!>', 1), va = x('<div class="container svelte-1h4zvzu"><!> <!> <!> <!></div>');
function Pt(ve, f) {
  Ee(f, true);
  let s = Ie(f, "user", 15), a = De(), k = ne(() => f.viewModePhone ? "calc(100vw - 1.5rem)" : "300px"), n = C(Le(s().account_type)), E = C(Le([])), m = C(void 0), i = C(false), w = C(false), P = C(""), d = C(false), I = C(void 0), G = C(Le({ current: "", new: "", newConfirm: "" })), H = ne(() => e(E).filter((g) => g.user_verified).length > 0);
  Ze(() => {
    J();
  });
  async function J() {
    let g = await Ve(`/auth/v1/users/${s().id}/webauthn`);
    g.body ? o(E, g.body, true) : console.error("error fetching passkeys: " + g.error);
  }
  async function re() {
    let g = await nt(`/auth/v1/users/${s().id}/self/convert_passkey`);
    g.error ? console.error("error fetching passkeys: " + g.error) : window.location.reload();
  }
  async function z() {
    o(P, ""), e(E).length > 0 ? await X() : await ae();
  }
  async function X() {
    var _a2;
    if (!((_a2 = e(m)) == null ? void 0 : _a2())) {
      o(P, a.common.invalidInput, true);
      return;
    }
    o(I, "PasswordNew");
  }
  async function ae(g) {
    var _a2, _b;
    if (!((_a2 = e(m)) == null ? void 0 : _a2())) {
      o(P, a.common.invalidInput, true);
      return;
    }
    o(w, true);
    let y = { password_new: e(G).new };
    g ? y.mfa_code = g : y.password_current = e(G).current;
    let c = await ut(`/auth/v1/users/${s().id}/self`, y);
    c.body ? (o(d, true), o(G, { current: "", new: "", newConfirm: "" }, true), s(c.body), o(n, c.body.account_type, true), setTimeout(() => {
      o(d, false), o(i, false);
    }, 3e3)) : o(P, ((_b = c.error) == null ? void 0 : _b.message) || "Error", true), o(w, false);
  }
  function q(g) {
    o(I, void 0), o(P, g, true), setTimeout(() => {
      o(P, "");
    }, 5e3);
  }
  function B(g) {
    o(I, void 0), g && "code" in g && ae(g.code);
  }
  async function Z() {
    let g = { email: s().email }, y = await nt("/auth/v1/users/request_reset", g);
    y.error ? o(P, y.error.message, true) : o(d, true);
  }
  var te = va(), A = r(te);
  {
    var T = (g) => {
      Mt(g, { get userId() {
        return s().id;
      }, get purpose() {
        return e(I);
      }, onSuccess: B, onError: q });
    };
    h(A, (g) => {
      e(I) && g(T);
    });
  }
  var _ = v(A, 2);
  {
    var R = (g) => {
      var y = Xr(), c = r(y), M = r(c, true);
      t(c);
      var V = v(c, 2), j = r(V), U = r(j, true);
      t(j), t(V);
      var de = v(V, 2);
      lt(de, "", {}, { height: ".3rem" });
      var pe = v(de, 2), ie = r(pe, true);
      t(pe);
      var le = v(pe, 2);
      {
        var N = ($) => {
          Xe($, { checked: true });
        }, b = ($) => {
          be($, { level: 2, onclick: Z, children: (K, se) => {
            ge();
            var O = he();
            D(() => l(O, a.account.passwordReset)), u(K, O);
          }, $$slots: { default: true } });
        };
        h(le, ($) => {
          e(d) ? $(N) : $(b, false);
        });
      }
      t(y), D(() => {
        var _a2;
        l(M, a.account.federatedConvertPassword1), l(U, ((_a2 = f.authProvider) == null ? void 0 : _a2.name) || "UNKNOWN"), l(ie, a.account.federatedConvertPassword2);
      }), u(g, y);
    };
    h(_, (g) => {
      e(n) === "federated" && g(R);
    });
  }
  var p = v(_, 2);
  {
    var Q = (g) => {
      var y = Zr(), c = ye(y), M = r(c, true);
      t(c);
      var V = v(c, 2), j = r(V, true);
      t(V);
      var U = v(V, 2), de = r(U, true);
      t(U);
      var pe = v(U, 2), ie = r(pe);
      be(ie, { level: 2, onclick: () => o(i, true), children: (le, N) => {
        ge();
        var b = he();
        D(() => l(b, a.account.convertAccount)), u(le, b);
      }, $$slots: { default: true } }), t(pe), D(() => {
        l(M, a.account.accTypePasskeyText1), l(j, a.account.accTypePasskeyText2), l(de, a.account.accTypePasskeyText3);
      }), u(g, y);
    };
    h(p, (g) => {
      (e(n) === "passkey" || e(n) === "federated_passkey") && !e(i) && g(Q);
    });
  }
  var Y = v(p, 2);
  {
    var ue = (g) => {
      var y = sa(), c = ye(y), M = r(c);
      const V = ne(() => !(e(n) === "password" && e(E).length < 1));
      Qr(M, { get inputWidth() {
        return e(k);
      }, get hideCurrentPassword() {
        return e(V);
      }, get passwords() {
        return e(G);
      }, set passwords(b) {
        o(G, b, true);
      }, get isValid() {
        return e(m);
      }, set isValid(b) {
        o(m, b, true);
      } });
      var j = v(M, 2), U = r(j);
      be(U, { onclick: z, level: 1, get isLoading() {
        return e(w);
      }, children: (b, $) => {
        ge();
        var K = he();
        D(() => l(K, a.common.save)), u(b, K);
      }, $$slots: { default: true } });
      var de = v(U, 2);
      {
        var pe = (b) => {
          var $ = ea(), K = r($);
          dt(K, {}), t($), Me(3, $, () => Ne), u(b, $);
        }, ie = (b, $) => {
          {
            var K = (O) => {
              var L = ta(), ce = r(L, true);
              t(L), D(() => l(ce, e(P))), Me(3, L, () => Ne), u(O, L);
            }, se = (O, L) => {
              {
                var ce = (fe) => {
                  var we = ra(), ke = r(we);
                  be(ke, { level: 3, onclick: () => o(i, false), children: (Ae, et) => {
                    ge();
                    var Ue = he();
                    D(() => l(Ue, a.common.cancel)), u(Ae, Ue);
                  }, $$slots: { default: true } }), t(we), u(fe, we);
                };
                h(O, (fe) => {
                  e(i) && !e(w) && fe(ce);
                }, L);
              }
            };
            h(b, (O) => {
              e(P) ? O(K) : O(se, false);
            }, $);
          }
        };
        h(de, (b) => {
          e(d) ? b(pe) : b(ie, false);
        });
      }
      t(j), t(c);
      var le = v(c, 2);
      {
        var N = (b) => {
          var $ = aa(), K = r($), se = r(K, true);
          t(K);
          var O = v(K, 2), L = r(O, true);
          t(O);
          var ce = v(O, 2);
          be(ce, { level: 2, onclick: re, children: (fe, we) => {
            ge();
            var ke = he();
            D(() => l(ke, a.account.convertAccount)), u(fe, ke);
          }, $$slots: { default: true } }), t($), D(() => {
            l(se, a.account.convertAccount), l(L, a.account.convertAccountP1);
          }), u(b, $);
        };
        h(le, (b) => {
          !e(i) && e(H) && b(N);
        });
      }
      u(g, y);
    };
    h(Y, (g) => {
      (e(n) === "password" || e(n) === "federated_password" || e(i)) && g(ue);
    });
  }
  t(te), u(ve, te), Te();
}
function oa(ve, f) {
  ve.code === "Enter" && f();
}
var na = x('<div class="flex"><div class="label font-label noselect svelte-1xf5lr3"><!></div> <label class="switch svelte-1xf5lr3"><input type="checkbox" class="svelte-1xf5lr3"> <span class="slider slider-round svelte-1xf5lr3"></span></label></div>');
function It(ve, f) {
  Ee(f, true);
  let s = Ie(f, "checked", 15, false), a = Ie(f, "ariaLabel", 3, ""), k = Ie(f, "name", 3, "");
  function n() {
    s(!s());
  }
  var E = na(), m = r(E);
  let i;
  var w = r(m);
  wr(w, () => f.children), t(m);
  var P = v(m, 2), d = r(P);
  er(d), d.__click = n, d.__keydown = [oa, n], ge(2), t(P), t(E), D(() => {
    i = lt(m, "", i, { width: f.labelWidth }), ze(d, "name", k()), d.disabled = f.disabled, ze(d, "aria-checked", s()), ze(d, "aria-label", a());
  }), br(d, s), u(ve, E), Te();
}
Xt(["click", "keydown"]);
var ia = x("<div><p> </p> <!></div>"), la = x('<div class="success svelte-5kivuv"><!></div>'), ua = x('<div class="err svelte-5kivuv"> </div>'), da = x('<div class="container svelte-5kivuv"><p> </p> <p><!></p> <div class="switch svelte-5kivuv"><!></div> <div class="switch svelte-5kivuv"><!></div> <!> <div class="bottom svelte-5kivuv"><!> <!></div></div>');
function $t(ve, f) {
  Ee(f, true);
  let s = Ie(f, "webIdData", 15), a = De();
  const k = "14rem";
  let n = C(""), E = C(false), m = C(!!s().custom_triples), i = ot(s().user_id);
  async function w() {
    o(n, "");
    let _ = await ut(`/auth/v1/users/${s().user_id}/webid/data`, s());
    _.error ? o(n, _.error.message, true) : (o(E, true), setTimeout(() => {
      o(E, false);
    }, 3e3));
  }
  var P = da(), d = r(P), I = r(d, true);
  t(d);
  var G = v(d, 2), H = r(G);
  yr(H, { href: i, target: "_blank", children: (_, R) => {
    var p = Re(), Q = ye(p);
    Tt(Q, () => i.replace("/auth/", "/auth/<wbr/>")), u(_, p);
  }, $$slots: { default: true } }), t(G);
  var J = v(G, 2), re = r(J);
  It(re, { ariaLabel: "E-Mail", labelWidth: k, get checked() {
    return s().expose_email;
  }, set checked(_) {
    s(s().expose_email = _, true);
  }, children: (_, R) => {
    ge();
    var p = he("E-Mail");
    u(_, p);
  }, $$slots: { default: true } }), t(J);
  var z = v(J, 2), X = r(z);
  It(X, { get ariaLabel() {
    return a.account.webIdExpertMode;
  }, labelWidth: k, get checked() {
    return e(m);
  }, set checked(_) {
    o(m, _, true);
  }, children: (_, R) => {
    ge();
    var p = he();
    D(() => l(p, a.account.webIdExpertMode)), u(_, p);
  }, $$slots: { default: true } }), t(z);
  var ae = v(z, 2);
  {
    var q = (_) => {
      var R = ia(), p = r(R), Q = r(p, true);
      t(p);
      var Y = v(p, 2);
      xr(Y, { placeholder: "FOAF", rows: 15, get value() {
        return s().custom_triples;
      }, set value(ue) {
        s(s().custom_triples = ue, true);
      } }), t(R), D(() => l(Q, a.account.webIdDescData)), Me(3, R, () => ir, () => ({ duration: 150 })), u(_, R);
    };
    h(ae, (_) => {
      e(m) && _(q);
    });
  }
  var B = v(ae, 2), Z = r(B);
  be(Z, { onclick: w, children: (_, R) => {
    ge();
    var p = he();
    D(() => l(p, a.common.save)), u(_, p);
  }, $$slots: { default: true } });
  var te = v(Z, 2);
  {
    var A = (_) => {
      var R = la(), p = r(R);
      dt(p, {}), t(R), Me(3, R, () => Ne), u(_, R);
    }, T = (_, R) => {
      {
        var p = (Q) => {
          var Y = ua(), ue = r(Y, true);
          t(Y), D(() => l(ue, e(n))), Me(3, Y, () => Ne), u(Q, Y);
        };
        h(_, (Q) => {
          e(n) && Q(p);
        }, R);
      }
    };
    h(te, (_) => {
      e(E) ? _(A) : _(T, false);
    });
  }
  t(B), t(P), D(() => l(I, a.account.webIdDesc)), u(ve, P), Te();
}
function ca(ve, f) {
  Ee(f, true);
  let s = ct("account"), a = ne(() => {
    var _a2;
    return (_a2 = s.get()) == null ? void 0 : _a2.user_id;
  });
  var k = Re(), n = ye(k);
  {
    var E = (m) => {
      Dt(m, { viewMode: "account", get userId() {
        return e(a);
      } });
    };
    h(n, (m) => {
      e(a) && m(E);
    });
  }
  u(ve, k), Te();
}
var fa = x("<h3> </h3>"), _a = x('<div class="headerPhone svelte-3a6iwi"><!></div> <div class="container svelte-3a6iwi"><!> <div class="innerPhone svelte-3a6iwi"><!></div></div>', 1), pa = x('<div clasS="info svelte-3a6iwi"><!></div>'), ma = x('<div class="flex gap-05"><!> </div>'), ga = x('<div class="wide svelte-3a6iwi"><!> <div class="container svelte-3a6iwi"><!> <div class="inner svelte-3a6iwi"><!></div></div> <div class="logout svelte-3a6iwi"><!></div></div>'), ha = x('<!> <div class="wrapper svelte-3a6iwi"><!></div>', 1);
function wa(ve, f) {
  Ee(f, true);
  const s = (q) => {
    var B = fa(), Z = r(B, true);
    t(B), D(() => l(Z, `${a().given_name} ${a().family_name || ""}`)), u(q, B);
  };
  let a = Ie(f, "user", 15), k = Ie(f, "webIdData", 15), n = De(), E = C(void 0), m = C(Le([])), i = ne(() => {
    var _a2;
    if ((_a2 = a().account_type) == null ? void 0 : _a2.startsWith("federated")) return e(m).filter((q) => q.id === a().auth_provider_id)[0];
  }), w = ne(() => e(E) && e(E) < 560), P = ne(() => e(E) && e(E) < 1e3), d = C(Le(n.account.navInfo)), I = ne(() => k() ? [n.account.navInfo, n.account.navMfa, n.account.devices, n.account.navEdit, n.common.password, "WebID", n.account.navLogout] : [n.account.navInfo, n.account.navMfa, n.account.devices, n.account.navEdit, n.common.password, n.account.navLogout]), G = ne(() => k() ? [n.account.navMfa, n.account.devices, n.account.navEdit, n.common.password, "WebID"] : [n.account.navMfa, n.account.devices, n.account.navEdit, n.common.password]);
  Ze(() => {
    Pr("v").get() === "devices" && o(d, n.account.devices, true);
  }), Se(() => {
    e(w) || e(P) ? o(d, n.account.navInfo, true) : o(d, n.account.navMfa, true);
  }), Se(() => {
    e(d) === n.account.navLogout && wt();
  });
  var H = ha(), J = ye(H);
  kr(J, { id: or, get value() {
    return e(m);
  }, set value(q) {
    o(m, q, true);
  } });
  var re = v(J, 2), z = r(re);
  {
    var X = (q) => {
      var B = _a(), Z = ye(B), te = r(Z);
      s(te), t(Z);
      var A = v(Z, 2), T = r(A);
      yt(T, { get tabs() {
        return e(I);
      }, get selected() {
        return e(d);
      }, set selected(Y) {
        o(d, Y, true);
      } });
      var _ = v(T, 2), R = r(_);
      {
        var p = (Y) => {
          vt(Y, { get providers() {
            return e(m);
          }, get authProvider() {
            return e(i);
          }, viewModePhone: true, get webIdData() {
            return k();
          }, get user() {
            return a();
          }, set user(ue) {
            a(ue);
          } });
        }, Q = (Y, ue) => {
          {
            var g = (c) => {
              xt(c, { viewModePhone: true, get user() {
                return a();
              }, set user(M) {
                a(M);
              } });
            }, y = (c, M) => {
              {
                var V = (U) => {
                  Pt(U, { get user() {
                    return a();
                  }, get authProvider() {
                    return e(i);
                  }, viewModePhone: true });
                }, j = (U, de) => {
                  {
                    var pe = (le) => {
                      kt(le, { get user() {
                        return a();
                      } });
                    }, ie = (le, N) => {
                      {
                        var b = (K) => {
                          var se = Re(), O = ye(se);
                          {
                            var L = (ce) => {
                              $t(ce, { get webIdData() {
                                return k();
                              }, set webIdData(fe) {
                                k(fe);
                              } });
                            };
                            h(O, (ce) => {
                              k() && ce(L);
                            });
                          }
                          u(K, se);
                        }, $ = (K, se) => {
                          {
                            var O = (L) => {
                              Dt(L, { get userId() {
                                return a().id;
                              } });
                            };
                            h(K, (L) => {
                              e(d) === n.account.devices && L(O);
                            }, se);
                          }
                        };
                        h(le, (K) => {
                          e(d) === "WebID" ? K(b) : K($, false);
                        }, N);
                      }
                    };
                    h(U, (le) => {
                      e(d) === n.account.navMfa ? le(pe) : le(ie, false);
                    }, de);
                  }
                };
                h(c, (U) => {
                  e(d) === n.common.password ? U(V) : U(j, false);
                }, M);
              }
            };
            h(Y, (c) => {
              e(d) === n.account.navEdit ? c(g) : c(y, false);
            }, ue);
          }
        };
        h(R, (Y) => {
          e(d) === n.account.navInfo ? Y(p) : Y(Q, false);
        });
      }
      t(_), t(A), u(q, B);
    }, ae = (q) => {
      var B = ga(), Z = r(B);
      {
        var te = (y) => {
          var c = pa(), M = r(c);
          vt(M, { get webIdData() {
            return k();
          }, get providers() {
            return e(m);
          }, get authProvider() {
            return e(i);
          }, get user() {
            return a();
          }, set user(V) {
            a(V);
          } }), t(c), u(y, c);
        };
        h(Z, (y) => {
          e(P) || y(te);
        });
      }
      var A = v(Z, 2), T = r(A);
      const _ = ne(() => e(P) ? e(I) : e(G));
      yt(T, { get tabs() {
        return e(_);
      }, center: true, get selected() {
        return e(d);
      }, set selected(y) {
        o(d, y, true);
      } });
      var R = v(T, 2), p = r(R);
      {
        var Q = (y) => {
          vt(y, { get webIdData() {
            return k();
          }, get providers() {
            return e(m);
          }, get authProvider() {
            return e(i);
          }, get user() {
            return a();
          }, set user(c) {
            a(c);
          } });
        }, Y = (y, c) => {
          {
            var M = (j) => {
              xt(j, { get user() {
                return a();
              }, set user(U) {
                a(U);
              } });
            }, V = (j, U) => {
              {
                var de = (ie) => {
                  Pt(ie, { get user() {
                    return a();
                  }, get authProvider() {
                    return e(i);
                  } });
                }, pe = (ie, le) => {
                  {
                    var N = ($) => {
                      kt($, { get user() {
                        return a();
                      } });
                    }, b = ($, K) => {
                      {
                        var se = (L) => {
                          var ce = Re(), fe = ye(ce);
                          {
                            var we = (ke) => {
                              $t(ke, { get webIdData() {
                                return k();
                              }, set webIdData(Ae) {
                                k(Ae);
                              } });
                            };
                            h(fe, (ke) => {
                              k() && ke(we);
                            });
                          }
                          u(L, ce);
                        }, O = (L, ce) => {
                          {
                            var fe = (we) => {
                              ca(we, {});
                            };
                            h(L, (we) => {
                              e(d) === n.account.devices && we(fe);
                            }, ce);
                          }
                        };
                        h($, (L) => {
                          e(d) === "WebID" ? L(se) : L(O, false);
                        }, K);
                      }
                    };
                    h(ie, ($) => {
                      e(d) === n.account.navMfa ? $(N) : $(b, false);
                    }, le);
                  }
                };
                h(j, (ie) => {
                  e(d) === n.common.password ? ie(de) : ie(pe, false);
                }, U);
              }
            };
            h(y, (j) => {
              e(d) === n.account.navEdit ? j(M) : j(V, false);
            }, c);
          }
        };
        h(p, (y) => {
          e(d) === n.account.navInfo ? y(Q) : y(Y, false);
        });
      }
      t(R), t(A);
      var ue = v(A, 2), g = r(ue);
      be(g, { level: -3, onclick: wt, children: (y, c) => {
        var M = ma(), V = r(M);
        Ir(V, {});
        var j = v(V);
        t(M), D(() => {
          ze(M, "title", n.account.navLogout), l(j, ` ${n.account.navLogout ?? ""}`);
        }), u(y, M);
      }, $$slots: { default: true } }), t(ue), t(B), u(q, B);
    };
    h(z, (q) => {
      e(w) ? q(X) : q(ae, false);
    });
  }
  t(re), tr("innerWidth", (q) => o(E, q, true)), u(ve, H), Te();
}
var ba = x("<!> <!> <!>", 1);
function os(ve, f) {
  Ee(f, true);
  let s = De(), a = ct("account"), k = C(void 0), n = C(void 0), E = C(false);
  Se(() => {
    let i = a.get();
    i && m(i);
  });
  async function m(i) {
    const w = i.user_id;
    if (w) {
      let P = await Promise.all([Ve(`/auth/v1/users/${w}`), Ve(`/auth/v1/users/${w}/webid/data`)]);
      P[0].body ? o(k, P[0].body, true) : nr("account"), P[1].body ? o(n, P[1].body, true) : P[1].status === 404 && o(n, { user_id: w, expose_email: false }, true), o(E, true);
    } else console.error("no user_id in session");
  }
  Zt((i) => {
    D(() => {
      var _a2;
      return Qt.title = `${(s == null ? void 0 : s.account.account) || "Account"} ${((_a2 = e(k)) == null ? void 0 : _a2.email) ?? ""}`;
    });
  }), $r(ve, { children: (i, w) => {
    Er(i, { children: (P, d) => {
      var I = ba(), G = ye(I);
      {
        var H = (z) => {
          wa(z, { get user() {
            return e(k);
          }, set user(X) {
            o(k, X, true);
          }, get webIdData() {
            return e(n);
          }, set webIdData(X) {
            o(n, X, true);
          } });
        };
        h(G, (z) => {
          e(E) && a && e(k) && z(H);
        });
      }
      var J = v(G, 2);
      Tr(J, { absolute: true });
      var re = v(J, 2);
      Cr(re, { absolute: true }), u(P, I);
    } });
  } }), Te();
}
export {
  os as component
};
