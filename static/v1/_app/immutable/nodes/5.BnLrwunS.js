import { t as k, a as d, e as we, d as Re } from "../chunks/DKC5GJ29.js";
import { p as Ce, c as r, s as v, j as e, a9 as ie, r as t, t as D, a as De, a8 as he, k as M, l as o, f as xe, a6 as Ze, aa as Se, a7 as Qt } from "../chunks/BveBAmlr.js";
import { s as l, d as Xt, h as Zt } from "../chunks/CYCba2oX.js";
import { i as w } from "../chunks/D-uYoVwt.js";
import { p as m } from "../chunks/VbPNpVtZ.js";
import { a as lt, b as S, c as q, s as ze, r as er } from "../chunks/Dql74IOz.js";
import { b as tr } from "../chunks/DpR-4P2Y.js";
import { p as $e } from "../chunks/Db0ChEdV.js";
import { l as Qe, Q as ot, J as rr, P as ar, K as sr, z as vr, S as wt, D as or, r as nr } from "../chunks/B21bTIl7.js";
import { e as Et } from "../chunks/BLE4FKaJ.js";
import { h as Tt } from "../chunks/CJj8kriY.js";
import { C as Xe } from "../chunks/5J717Kcr.js";
import { B as ye, t as Le, f as Ne, s as ir } from "../chunks/DlLEcmNg.js";
import { M as lr } from "../chunks/Bu9w5Iki.js";
import { u as Me } from "../chunks/8R5My_LO.js";
import { c as Ct, b as nt, d as dt, f as Ve } from "../chunks/BO1A6s0c.js";
import { B as dr } from "../chunks/DjyBHFkv.js";
import { U as ur, a as cr, D as Dt } from "../chunks/q-HSZoV6.js";
import { I as Te } from "../chunks/CCQyB9gY.js";
import { F as fr } from "../chunks/BguaNybM.js";
import { b as it, k as _r, l as bt, m as pr } from "../chunks/BRCxk8by.js";
import { I as ut } from "../chunks/Cxysd5TC.js";
import { I as mr } from "../chunks/CkL7I1FG.js";
import { u as ct } from "../chunks/CPBhBTQd.js";
import { w as gr } from "../chunks/BH-05F4S.js";
import { W as Mt } from "../chunks/C3pTUZii.js";
import { P as hr } from "../chunks/CpbwoRiQ.js";
import { I as st } from "../chunks/CUO8Plwh.js";
import { s as wr } from "../chunks/Dv-Q3FDX.js";
import { c as br } from "../chunks/Ds33nkO-.js";
import { A as yr } from "../chunks/BoQH-Qa9.js";
import { I as xr } from "../chunks/Cv33Ea-7.js";
import { T as kr } from "../chunks/CDuWpfYL.js";
import { u as Pr } from "../chunks/BdFU-gNS.js";
import { T as yt } from "../chunks/BGiCnx1L.js";
import { I as Ir } from "../chunks/BVxkLPhB.js";
import { M as $r } from "../chunks/DaM_1q3c.js";
import { C as Er } from "../chunks/Ci6f1shh.js";
import { T as Tr } from "../chunks/BFgWaswG.js";
import { L as Cr } from "../chunks/DNYXBAk_.js";
var Dr = k('<div class="link-err value svelte-1tea8v4"> </div>'), Mr = k('<div class="fed-btn svelte-1tea8v4"><!> <!></div>'), Lr = k('<h3> </h3> <p> </p> <div class="providers svelte-1tea8v4"></div>', 1), Nr = k("<!> <!>", 1), Ar = k('<div><div> </div> <span class="value svelte-1tea8v4"> </span></div>'), Wr = k('<div><div> </div> <span class="value svelte-1tea8v4"> </span></div>'), zr = k('<div><div>WebID:</div> <span class="value svelte-1tea8v4"><a target="_blank"><!></a></span></div>'), Rr = k('<div><div><div></div> <!></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <div><div class="value svelte-1tea8v4"> </div> <!></div></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <!></div> <div><div> </div> <!></div> <div><div> </div> <!></div> <!> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <!> <!></div>');
function vt(oe, _) {
  Ce(_, true);
  let s = $e(_, "user", 15), a = Me(), P = M(false), n = M(false), T = ie(() => {
    var _a2;
    return (_a2 = s().account_type) == null ? void 0 : _a2.startsWith("federated");
  }), g = ie(() => {
    var _a2;
    return e(T) ? `${s().account_type}: ${((_a2 = _.authProvider) == null ? void 0 : _a2.name) || ""}` : s().account_type;
  }), i = ie(() => _.viewModePhone ? "rowPhone" : "row"), b = ie(() => _.viewModePhone ? "labelPhone" : "label"), I = ie(() => {
    let W = s().given_name[0];
    return s().family_name && s().family_name.length > 0 && (W += s().family_name[0]), W;
  });
  function u(W) {
    rr(64, (te, { challenge: ne, verifier: pe }) => {
      te || (localStorage.setItem(ar, pe), $(W, ne));
    });
  }
  async function $(W, te) {
    let ne = { email: s().email, client_id: "rauthy", redirect_uri: window.location.href, provider_id: W, pkce_challenge: te }, pe = await nt(`/auth/v1/providers/${W}/link`, ne);
    if (pe.text) {
      sr(pe.text);
      let ge = pe.headers.get("location");
      ge && (window.location.href = ge);
    } else console.error(pe.error);
  }
  async function J() {
    let W = await Ct("/auth/v1/providers/link");
    W.body ? s(W.body) : (console.error(W.error), o(P, true));
  }
  var Y = Rr(), Q = r(Y);
  lt(Q, "", {}, { margin: ".5rem 0" });
  var ae = r(Q), z = v(ae, 2);
  ur(z, { get fallbackCharacters() {
    return e(I);
  }, get userId() {
    return s().id;
  }, size: "large", get pictureId() {
    return s().picture_id;
  }, set pictureId(W) {
    s(s().picture_id = W, true);
  } }), t(Q);
  var Z = v(Q, 2), se = r(Z), B = r(se, true);
  t(se);
  var K = v(se, 2), ee = r(K, true);
  t(K), t(Z);
  var re = v(Z, 2), V = r(re), C = r(V, true);
  t(V);
  var f = v(V, 2), R = r(f, true);
  t(f), t(re);
  var p = v(re, 2), X = r(p), j = r(X, true);
  t(X);
  var ue = v(X, 2), h = r(ue, true);
  t(ue), t(p);
  var x = v(p, 2), c = r(x), L = r(c);
  t(c);
  var U = v(c, 2), G = r(U, true);
  t(U), t(x);
  var F = v(x, 2), ce = r(F), me = r(ce, true);
  t(ce);
  var le = v(ce, 2), de = r(le), A = r(de, true);
  t(de);
  var y = v(de, 2);
  {
    var E = (W) => {
      var te = Mr(), ne = r(te);
      ye(ne, { get ariaLabel() {
        return a.account.providerUnlink;
      }, level: 3, onclick: J, children: (ke, Ie) => {
        he();
        var Ee = we();
        D(() => l(Ee, a.account.providerUnlink)), d(ke, Ee);
      }, $$slots: { default: true } });
      var pe = v(ne, 2);
      {
        var ge = (ke) => {
          var Ie = Dr(), Ee = r(Ie, true);
          t(Ie), D(() => l(Ee, a.account.providerUnlinkDesc)), d(ke, Ie);
        };
        w(pe, (ke) => {
          e(P) && ke(ge);
        });
      }
      t(te), d(W, te);
    }, O = (W, te) => {
      {
        var ne = (pe) => {
          var ge = Nr(), ke = xe(ge);
          ye(ke, { level: 2, onclick: () => o(n, true), children: (Ee, Yt) => {
            he();
            var We = we();
            D(() => l(We, a.account.providerLink)), d(Ee, We);
          }, $$slots: { default: true } });
          var Ie = v(ke, 2);
          lr(Ie, { get showModal() {
            return e(n);
          }, set showModal(Ee) {
            o(n, m(Ee));
          }, children: (Ee, Yt) => {
            var We = Lr(), tt = xe(We), jt = r(tt, true);
            t(tt);
            var rt = v(tt, 2), Gt = r(rt, true);
            t(rt);
            var gt = v(rt, 2);
            Et(gt, 21, () => _.providers, (at) => at.id, (at, ht) => {
              const Jt = ie(() => `${a.account.providerLink}: ${e(ht).name}`);
              dr(at, { get ariaLabel() {
                return e(Jt);
              }, get provider() {
                return e(ht);
              }, onclick: u });
            }), t(gt), D(() => {
              l(jt, a.account.providerLink), l(Gt, a.account.providerLinkDesc);
            }), d(Ee, We);
          }, $$slots: { default: true } }), d(pe, ge);
        };
        w(W, (pe) => {
          _.providers.length > 0 && pe(ne);
        }, te);
      }
    };
    w(y, (W) => {
      e(T) ? W(E) : W(O, false);
    });
  }
  t(le), t(F);
  var ve = v(F, 2), H = r(ve), N = r(H, true);
  t(H);
  var fe = v(H, 2), _e = r(fe, true);
  t(fe), t(ve);
  var be = v(ve, 2), Pe = r(be), Ae = r(Pe, true);
  t(Pe);
  var et = v(Pe, 2), Ue = r(et, true);
  t(et), t(be);
  var Fe = v(be, 2), qe = r(Fe), Lt = r(qe, true);
  t(qe);
  var Nt = v(qe, 2);
  const At = ie(() => !!s().webauthn_user_id);
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
      var te = Ar(), ne = r(te), pe = r(ne, true);
      t(ne);
      var ge = v(ne, 2), ke = r(ge, true);
      t(ge), t(te), D((Ie) => {
        S(te, 1, q(e(i)), "svelte-1tea8v4"), S(ne, 1, q(e(b)), "svelte-1tea8v4"), l(pe, a.account.lastLogin), l(ke, Ie);
      }, [() => Qe(s().last_login)]), d(W, te);
    };
    w(ft, (W) => {
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
      var te = Wr(), ne = r(te), pe = r(ne, true);
      t(ne);
      var ge = v(ne, 2), ke = r(ge, true);
      t(ge), t(te), D((Ie) => {
        S(te, 1, q(e(i)), "svelte-1tea8v4"), S(ne, 1, q(e(b)), "svelte-1tea8v4"), l(pe, a.account.userExpiry), l(ke, Ie);
      }, [() => Qe(s().user_expires)]), d(W, te);
    };
    w(mt, (W) => {
      s().user_expires && W(Kt);
    });
  }
  var Ot = v(mt, 2);
  {
    var Ht = (W) => {
      var te = zr(), ne = r(te), pe = v(ne, 2), ge = r(pe), ke = r(ge);
      Tt(ke, () => ot(s().id).replace("/auth/", "/auth/<wbr/>")), t(ge), t(pe), t(te), D((Ie) => {
        S(te, 1, q(e(i)), "svelte-1tea8v4"), S(ne, 1, q(e(b)), "svelte-1tea8v4"), ze(ge, "href", Ie);
      }, [() => ot(s().id)]), d(W, te);
    };
    w(Ot, (W) => {
      _.webIdData && W(Ht);
    });
  }
  t(Y), D((W, te) => {
    S(Q, 1, q(e(i)), "svelte-1tea8v4"), S(ae, 1, q(e(b)), "svelte-1tea8v4"), S(Z, 1, q(e(i)), "svelte-1tea8v4"), S(se, 1, q(e(b)), "svelte-1tea8v4"), l(B, a.common.email), l(ee, s().email), S(re, 1, q(e(i)), "svelte-1tea8v4"), S(V, 1, q(e(b)), "svelte-1tea8v4"), l(C, a.account.givenName), l(R, s().given_name), S(p, 1, q(e(i)), "svelte-1tea8v4"), S(X, 1, q(e(b)), "svelte-1tea8v4"), l(j, a.account.familyName), l(h, s().family_name), S(x, 1, q(e(i)), "svelte-1tea8v4"), S(c, 1, q(e(b)), "svelte-1tea8v4"), l(L, `${a.account.user ?? ""} ID`), l(G, s().id), S(F, 1, q(e(i)), "svelte-1tea8v4"), S(ce, 1, q(e(b)), "svelte-1tea8v4"), l(me, a.account.accType), l(A, e(g) || ""), S(ve, 1, q(e(i)), "svelte-1tea8v4"), S(H, 1, q(e(b)), "svelte-1tea8v4"), l(N, a.account.roles), l(_e, s().roles || "None"), S(be, 1, q(e(i)), "svelte-1tea8v4"), S(Pe, 1, q(e(b)), "svelte-1tea8v4"), l(Ae, a.account.groups), l(Ue, s().groups || "None"), S(Fe, 1, q(e(i)), "svelte-1tea8v4"), S(qe, 1, q(e(b)), "svelte-1tea8v4"), l(Lt, a.account.mfaActivated), S(Be, 1, q(e(i)), "svelte-1tea8v4"), S(Ke, 1, q(e(b)), "svelte-1tea8v4"), l(Wt, a.account.userEnabled), S(Oe, 1, q(e(i)), "svelte-1tea8v4"), S(He, 1, q(e(b)), "svelte-1tea8v4"), l(Rt, a.account.emailVerified), S(Ye, 1, q(e(i)), "svelte-1tea8v4"), S(je, 1, q(e(b)), "svelte-1tea8v4"), l(Ut, a.account.passwordExpiry), l(Ft, W), S(Ge, 1, q(e(i)), "svelte-1tea8v4"), S(Je, 1, q(e(b)), "svelte-1tea8v4"), l(qt, a.account.userCreated), l(Bt, te);
  }, [() => s().password_expires && Qe(s().password_expires) || a.common.never, () => Qe(s().created_at)]), d(oe, Y), De();
}
var Sr = k('<div class="success svelte-4ogr3z"><!></div>'), Vr = k("<p> </p>"), Ur = k('<div class="err svelte-4ogr3z"> </div>'), Fr = k('<div class="formInner svelte-4ogr3z"><div><!> <!> <!> <!></div> <div><!> <!> <!> <!> <!></div></div> <div class="bottom svelte-4ogr3z"><div><!></div> <!></div> <!>', 1), qr = k('<div class="container svelte-4ogr3z"><!></div>');
function xt(oe, _) {
  var _a2;
  Ce(_, true);
  let s = $e(_, "user", 15);
  ((_a2 = s().user_values) == null ? void 0 : _a2.birthdate) || s(s().user_values.birthdate = "", true);
  let a = Me(), P = M(""), n = M(false), T = M(false);
  async function g(u, $) {
    var _a3;
    const J = $.get("email"), Y = $.get("family_name") || void 0, Q = $.get("given_name") || void 0, ae = $.get("birthdate") || void 0, z = ((_a3 = $.get("phone")) == null ? void 0 : _a3.replaceAll(" ", "")) || void 0, Z = $.get("street") || void 0, se = $.get("zip") || void 0, B = se ? Number.parseInt(se) : void 0, K = $.get("city") || void 0, ee = $.get("country") || void 0;
    let re = { email: J, family_name: Y, given_name: Q };
    (ae || z || Z || B || K || ee) && (re.user_values = { birthdate: ae, phone: z, street: Z, zip: B, city: K, country: ee });
    let V = await dt(`/auth/v1/users/${s().id}/self`, re);
    if (V.body) {
      o(n, true), s(V.body);
      let C = 3e3;
      V.status === 202 && (o(T, true), C = 3e4), setTimeout(() => {
        o(n, false);
      }, C);
    } else V.error && (console.error(V.error), o(P, m(V.error.message)));
  }
  var i = qr(), b = r(i);
  const I = ie(() => `/auth/v1/users/${s().id}/self`);
  fr(b, { get action() {
    return e(I);
  }, onSubmit: g, children: (u, $) => {
    var J = Fr(), Y = xe(J), Q = r(Y), ae = r(Q);
    Te(ae, { typ: "email", name: "email", get label() {
      return a.common.email;
    }, get placeholder() {
      return a.common.email;
    }, get value() {
      return s().email;
    }, required: true });
    var z = v(ae, 2);
    Te(z, { name: "given_name", autocomplete: "given-name", get label() {
      return a.account.givenName;
    }, get placeholder() {
      return a.account.givenName;
    }, get value() {
      return s().given_name;
    }, required: true, maxLength: 32, pattern: it });
    var Z = v(z, 2);
    Te(Z, { name: "family_name", autocomplete: "family-name", get label() {
      return a.account.familyName;
    }, get placeholder() {
      return a.account.familyName;
    }, get value() {
      return s().family_name;
    }, maxLength: 32, pattern: it });
    var se = v(Z, 2);
    mr(se, { name: "birthdate", get label() {
      return a.account.birthdate;
    }, withDelete: true, get value() {
      return s().user_values.birthdate;
    }, set value(c) {
      s(s().user_values.birthdate = c, true);
    } }), t(Q);
    var B = v(Q, 2), K = r(B);
    Te(K, { name: "street", autocomplete: "street-address", get label() {
      return a.account.street;
    }, get placeholder() {
      return a.account.street;
    }, get value() {
      return s().user_values.street;
    }, maxLength: 48, pattern: _r });
    var ee = v(K, 2);
    Te(ee, { typ: "number", name: "zip", autocomplete: "postal-code", get label() {
      return a.account.zip;
    }, get placeholder() {
      return a.account.zip;
    }, get value() {
      return s().user_values.zip;
    }, min: "1000", max: "9999999" });
    var re = v(ee, 2);
    Te(re, { name: "city", autocomplete: "address-level2", get label() {
      return a.account.city;
    }, get placeholder() {
      return a.account.city;
    }, get value() {
      return s().user_values.city;
    }, maxLength: 48, pattern: bt });
    var V = v(re, 2);
    Te(V, { name: "country", autocomplete: "country", get label() {
      return a.account.country;
    }, get placeholder() {
      return a.account.country;
    }, get value() {
      return s().user_values.country;
    }, maxLength: 48, pattern: bt });
    var C = v(V, 2);
    Te(C, { name: "phone", autocomplete: "tel", get label() {
      return a.account.phone;
    }, get placeholder() {
      return a.account.phone;
    }, get value() {
      return s().user_values.phone;
    }, maxLength: 32, pattern: pr }), t(B), t(Y);
    var f = v(Y, 2), R = r(f), p = r(R);
    ye(p, { type: "submit", children: (c, L) => {
      he();
      var U = we();
      D(() => l(U, a.common.save)), d(c, U);
    }, $$slots: { default: true } }), t(R);
    var X = v(R, 2);
    {
      var j = (c) => {
        var L = Sr(), U = r(L);
        ut(U, {}), t(L), Le(3, L, () => Ne), d(c, L);
      };
      w(X, (c) => {
        e(n) && c(j);
      });
    }
    t(f);
    var ue = v(f, 2);
    {
      var h = (c) => {
        var L = Vr(), U = r(L, true);
        t(L), D(() => l(U, a.account.emailUpdateConfirm)), d(c, L);
      }, x = (c, L) => {
        {
          var U = (G) => {
            var F = Ur(), ce = r(F, true);
            t(F), D(() => l(ce, e(P))), Le(3, F, () => Ne), d(G, F);
          };
          w(c, (G) => {
            e(P) && G(U);
          }, L);
        }
      };
      w(ue, (c) => {
        e(T) ? c(h) : c(x, false);
      });
    }
    d(u, J);
  }, $$slots: { default: true } }), t(i), d(oe, i), De();
}
var Br = k('<div class="err svelte-16mlupu"><b>Your browser does not support Webauthn credentials and must be updated.</b></div>'), Kr = k('<!> <div class="regBtns svelte-16mlupu"><!> <!></div>', 1), Or = k('<div class="regNewBtn svelte-16mlupu"><!></div>'), Hr = k('<div class="keysHeader svelte-16mlupu"> </div>'), Yr = k('<div class="button svelte-16mlupu"><!></div>'), jr = k('<!> <p class="svelte-16mlupu"> <br><br> </p> <!> <!> <div class="keysContainer svelte-16mlupu"></div> <!> <div> </div>', 1), Gr = k('<div class="container svelte-16mlupu"><!></div>');
function kt(oe, _) {
  Ce(_, true);
  const s = "credentials" in navigator;
  let a = Me(), P = ct("account"), n = ie(() => {
    var _a2;
    return (_a2 = P.get()) == null ? void 0 : _a2.user_id;
  }), T = M(void 0), g = M(false), i = M(""), b = M(false), I = M(_.user.account_type === "password"), u = M(void 0), $ = M(""), J = M(false), Y = M(m([]));
  Ze(() => {
    ae();
  }), Se(() => {
    e(Y).length > 0 && _.user.account_type === "passkey" && o(I, e(Y).length > 1);
  }), Se(() => {
    var _a2;
    (_a2 = e(T)) == null ? void 0 : _a2.focus();
  });
  function Q() {
    o(g, false), o(i, "");
  }
  async function ae() {
    var _a2;
    let C = await Ve(`/auth/v1/users/${(_a2 = P.get()) == null ? void 0 : _a2.user_id}/webauthn`);
    C.body ? o(Y, m(C.body)) : console.error(C.error);
  }
  async function z() {
    if (Q(), e(J) || !e(n)) return;
    if (e($).length < 1) {
      o(g, true), o(i, m(a.mfa.passkeyNameErr));
      return;
    }
    let C = await gr(e(n), e($), a.authorize.invalidKeyUsed, a.authorize.requestExpired);
    C.error ? (o(g, true), o(i, `${a.mfa.errorReg} - ${C.error}`)) : (o(b, false), o($, ""), await ae());
  }
  async function Z(C) {
    var _a2;
    let f = await Ct(`/auth/v1/users/${_.user.id}/webauthn/delete/${C}`);
    f.status === 200 ? await ae() : o(i, m(((_a2 = f.error) == null ? void 0 : _a2.message) || "Error"));
  }
  function se(C) {
    o(u, void 0), o(g, true), o(i, m(C)), setTimeout(() => {
      o(g, false), o(i, "");
    }, 5e3);
  }
  function B(C) {
    o(u, void 0), o(i, m(a.mfa.testSuccess)), setTimeout(() => {
      o(i, "");
    }, 3e3);
  }
  var K = Gr(), ee = r(K);
  {
    var re = (C) => {
      var f = Br();
      d(C, f);
    }, V = (C) => {
      var f = jr(), R = xe(f);
      {
        var p = (A) => {
          Mt(A, { get userId() {
            return _.user.id;
          }, get purpose() {
            return e(u);
          }, onSuccess: B, onError: se });
        };
        w(R, (A) => {
          e(u) && A(p);
        });
      }
      var X = v(R, 2), j = r(X), ue = v(j, 3);
      t(X);
      var h = v(X, 2);
      {
        var x = (A) => {
          var y = Kr(), E = xe(y);
          Te(E, { autocomplete: "off", get label() {
            return a.mfa.passkeyName;
          }, get placeholder() {
            return a.mfa.passkeyName;
          }, maxLength: 32, pattern: it, onEnter: z, get ref() {
            return e(T);
          }, set ref(N) {
            o(T, m(N));
          }, get value() {
            return e($);
          }, set value(N) {
            o($, m(N));
          }, get isError() {
            return e(J);
          }, set isError(N) {
            o(J, m(N));
          } });
          var O = v(E, 2), ve = r(O);
          ye(ve, { onclick: z, children: (N, fe) => {
            he();
            var _e = we();
            D(() => l(_e, a.mfa.register)), d(N, _e);
          }, $$slots: { default: true } });
          var H = v(ve, 2);
          ye(H, { level: 3, onclick: () => o(b, false), children: (N, fe) => {
            he();
            var _e = we();
            D(() => l(_e, a.common.cancel)), d(N, _e);
          }, $$slots: { default: true } }), t(O), d(A, y);
        }, c = (A) => {
          var y = Or(), E = r(y);
          const O = ie(() => e(Y).length === 0 ? 1 : 2);
          ye(E, { get level() {
            return e(O);
          }, onclick: () => o(b, true), children: (ve, H) => {
            he();
            var N = we();
            D(() => l(N, a.mfa.registerNew)), d(ve, N);
          }, $$slots: { default: true } }), t(y), d(A, y);
        };
        w(h, (A) => {
          e(b) ? A(x) : A(c, false);
        });
      }
      var L = v(h, 2);
      {
        var U = (A) => {
          var y = Hr(), E = r(y, true);
          t(y), D(() => l(E, a.mfa.registerdKeys)), d(A, y);
        };
        w(L, (A) => {
          e(Y).length > 0 && A(U);
        });
      }
      var G = v(L, 2);
      Et(G, 21, () => e(Y), (A) => A.name, (A, y) => {
        cr(A, { get passkey() {
          return e(y);
        }, get showDelete() {
          return e(I);
        }, onDelete: Z });
      }), t(G);
      var F = v(G, 2);
      {
        var ce = (A) => {
          var y = Yr(), E = r(y);
          ye(E, { onclick: () => o(u, "Test"), children: (O, ve) => {
            he();
            var H = we();
            D(() => l(H, a.mfa.test)), d(O, H);
          }, $$slots: { default: true } }), t(y), d(A, y);
        };
        w(F, (A) => {
          e(Y).length > 0 && A(ce);
        });
      }
      var me = v(F, 2);
      let le;
      var de = r(me, true);
      t(me), D((A) => {
        l(j, `${a.mfa.p1 ?? ""} `), l(ue, ` ${a.mfa.p2 ?? ""}`), le = S(me, 1, "svelte-16mlupu", null, le, A), l(de, e(i));
      }, [() => ({ success: !e(g), err: e(g) })]), d(C, f);
    };
    w(ee, (C) => {
      s ? C(V, false) : C(re);
    });
  }
  t(K), d(oe, K), De();
}
var Jr = k('<div class="container svelte-ue7bk2"><!> <!> <!> <!> <div class="btn svelte-ue7bk2"><!></div> <div class="err svelte-ue7bk2"> </div></div>');
function Qr(oe, _) {
  Ce(_, true);
  let s = $e(_, "passwords", 15), a = $e(_, "hideCurrentPassword", 3, false), P = $e(_, "isValid", 15), n = Me();
  P($);
  let T = M(false), g = M(""), i = M(void 0), b = ie(() => {
    var _a2;
    return ((_a2 = s().new) == null ? void 0 : _a2.length) > 6 && s().new === s().newConfirm;
  }), I = M(void 0), u = M(void 0);
  Ze(async () => {
    let z = await Ve("/auth/v1/password_policy");
    z.body ? o(i, m(z.body)) : console.error(z.error);
  });
  function $() {
    return o(g, ""), !a() && !s().current ? (o(g, m(n.account.passwordCurrReq)), false) : s().new ? s().newConfirm ? e(T) ? s().new.length > 256 ? (o(g, "max 256"), false) : s().new !== s().newConfirm ? (o(g, m(n.account.passwordNoMatch)), false) : true : (o(g, m(n.account.passwordPolicyFollow)), false) : (o(g, m(n.account.passwordNewReq)), false) : (o(g, m(n.account.passwordNewReq)), false);
  }
  function J() {
    if (o(g, ""), e(i)) {
      let z = vr(e(i));
      s(s().new = z, true), s(s().newConfirm = z, true);
    }
    requestAnimationFrame(() => {
      var _a2, _b;
      (_a2 = e(I)) == null ? void 0 : _a2(), (_b = e(u)) == null ? void 0 : _b();
    });
  }
  var Y = Re(), Q = xe(Y);
  {
    var ae = (z) => {
      var Z = Jr(), se = r(Z);
      hr(se, { get policy() {
        return e(i);
      }, get password() {
        return s().new;
      }, get accepted() {
        return e(T);
      }, set accepted(p) {
        o(T, m(p));
      } });
      var B = v(se, 2);
      {
        var K = (p) => {
          st(p, { autocomplete: "current-password", get label() {
            return n.account.passwordCurr;
          }, get placeholder() {
            return n.account.passwordCurr;
          }, onInput: $, get width() {
            return _.inputWidth;
          }, get value() {
            return s().current;
          }, set value(X) {
            s(s().current = X, true);
          } });
        };
        w(B, (p) => {
          a() || p(K);
        });
      }
      var ee = v(B, 2);
      st(ee, { autocomplete: "new-password", get label() {
        return n.account.passwordNew;
      }, get placeholder() {
        return n.account.passwordNew;
      }, onInput: $, get showCopy() {
        return e(b);
      }, get width() {
        return _.inputWidth;
      }, get value() {
        return s().new;
      }, set value(p) {
        s(s().new = p, true);
      }, get reportValidity() {
        return e(I);
      }, set reportValidity(p) {
        o(I, m(p));
      } });
      var re = v(ee, 2);
      st(re, { autocomplete: "new-password", get label() {
        return n.account.passwordConfirm;
      }, get placeholder() {
        return n.account.passwordConfirm;
      }, onInput: $, get showCopy() {
        return e(b);
      }, get width() {
        return _.inputWidth;
      }, get value() {
        return s().newConfirm;
      }, set value(p) {
        s(s().newConfirm = p, true);
      }, get reportValidity() {
        return e(u);
      }, set reportValidity(p) {
        o(u, m(p));
      } });
      var V = v(re, 2), C = r(V);
      ye(C, { onclick: J, level: 2, children: (p, X) => {
        he();
        var j = we();
        D(() => l(j, n.account.generateRandom)), d(p, j);
      }, $$slots: { default: true } }), t(V);
      var f = v(V, 2), R = r(f, true);
      t(f), t(Z), D(() => l(R, e(g))), d(z, Z);
    };
    w(Q, (z) => {
      e(i) && z(ae);
    });
  }
  return d(oe, Y), De({ isPwdValid: $ });
}
var Xr = k('<div class="m-05 svelte-1h4zvzu"><p> </p> <p><b> </b></p> <div></div> <p> </p> <!></div>'), Zr = k("<p> </p> <p> </p> <p> </p> <div><!></div>", 1), ea = k('<div class="success svelte-1h4zvzu"><!></div>'), ta = k('<div class="err"> </div>'), ra = k('<div class="cancel"><!></div>'), aa = k('<div class="convertPasskey svelte-1h4zvzu"><h3> </h3> <p> </p> <!></div>'), sa = k('<div><!> <div class="save svelte-1h4zvzu"><!> <!></div></div> <!>', 1), va = k('<div class="container svelte-1h4zvzu"><!> <!> <!> <!></div>');
function Pt(oe, _) {
  Ce(_, true);
  let s = $e(_, "user", 15), a = Me(), P = ie(() => _.viewModePhone ? "calc(100vw - 1.5rem)" : "300px"), n = M(m(s().account_type)), T = M(m([])), g = M(void 0), i = M(false), b = M(false), I = M(""), u = M(false), $ = M(void 0), J = M(m({ current: "", new: "", newConfirm: "" })), Y = ie(() => e(T).filter((h) => h.user_verified).length > 0);
  Ze(() => {
    Q();
  });
  async function Q() {
    let h = await Ve(`/auth/v1/users/${s().id}/webauthn`);
    h.body ? o(T, m(h.body)) : console.error("error fetching passkeys: " + h.error);
  }
  async function ae() {
    let h = await nt(`/auth/v1/users/${s().id}/self/convert_passkey`);
    h.error ? console.error("error fetching passkeys: " + h.error) : window.location.reload();
  }
  async function z() {
    o(I, ""), e(T).length > 0 ? await Z() : await se();
  }
  async function Z() {
    var _a2;
    if (!((_a2 = e(g)) == null ? void 0 : _a2())) {
      o(I, m(a.common.invalidInput));
      return;
    }
    o($, "PasswordNew");
  }
  async function se(h) {
    var _a2, _b;
    if (!((_a2 = e(g)) == null ? void 0 : _a2())) {
      o(I, m(a.common.invalidInput));
      return;
    }
    o(b, true);
    let x = { password_new: e(J).new };
    h ? x.mfa_code = h : x.password_current = e(J).current;
    let c = await dt(`/auth/v1/users/${s().id}/self`, x);
    c.body ? (o(u, true), o(J, m({ current: "", new: "", newConfirm: "" })), s(c.body), o(n, m(c.body.account_type)), setTimeout(() => {
      o(u, false), o(i, false);
    }, 3e3)) : o(I, m(((_b = c.error) == null ? void 0 : _b.message) || "Error")), o(b, false);
  }
  function B(h) {
    o($, void 0), o(I, m(h)), setTimeout(() => {
      o(I, "");
    }, 5e3);
  }
  function K(h) {
    o($, void 0), h && "code" in h && se(h.code);
  }
  async function ee() {
    let h = { email: s().email }, x = await nt("/auth/v1/users/request_reset", h);
    x.error ? o(I, m(x.error.message)) : o(u, true);
  }
  var re = va(), V = r(re);
  {
    var C = (h) => {
      Mt(h, { get userId() {
        return s().id;
      }, get purpose() {
        return e($);
      }, onSuccess: K, onError: B });
    };
    w(V, (h) => {
      e($) && h(C);
    });
  }
  var f = v(V, 2);
  {
    var R = (h) => {
      var x = Xr(), c = r(x), L = r(c, true);
      t(c);
      var U = v(c, 2), G = r(U), F = r(G, true);
      t(G), t(U);
      var ce = v(U, 2);
      lt(ce, "", {}, { height: ".3rem" });
      var me = v(ce, 2), le = r(me, true);
      t(me);
      var de = v(me, 2);
      {
        var A = (E) => {
          Xe(E, { checked: true });
        }, y = (E) => {
          ye(E, { level: 2, onclick: ee, children: (O, ve) => {
            he();
            var H = we();
            D(() => l(H, a.account.passwordReset)), d(O, H);
          }, $$slots: { default: true } });
        };
        w(de, (E) => {
          e(u) ? E(A) : E(y, false);
        });
      }
      t(x), D(() => {
        var _a2;
        l(L, a.account.federatedConvertPassword1), l(F, ((_a2 = _.authProvider) == null ? void 0 : _a2.name) || "UNKNOWN"), l(le, a.account.federatedConvertPassword2);
      }), d(h, x);
    };
    w(f, (h) => {
      e(n) === "federated" && h(R);
    });
  }
  var p = v(f, 2);
  {
    var X = (h) => {
      var x = Zr(), c = xe(x), L = r(c, true);
      t(c);
      var U = v(c, 2), G = r(U, true);
      t(U);
      var F = v(U, 2), ce = r(F, true);
      t(F);
      var me = v(F, 2), le = r(me);
      ye(le, { level: 2, onclick: () => o(i, true), children: (de, A) => {
        he();
        var y = we();
        D(() => l(y, a.account.convertAccount)), d(de, y);
      }, $$slots: { default: true } }), t(me), D(() => {
        l(L, a.account.accTypePasskeyText1), l(G, a.account.accTypePasskeyText2), l(ce, a.account.accTypePasskeyText3);
      }), d(h, x);
    };
    w(p, (h) => {
      (e(n) === "passkey" || e(n) === "federated_passkey") && !e(i) && h(X);
    });
  }
  var j = v(p, 2);
  {
    var ue = (h) => {
      var x = sa(), c = xe(x), L = r(c);
      const U = ie(() => !(e(n) === "password" && e(T).length < 1));
      Qr(L, { get inputWidth() {
        return e(P);
      }, get hideCurrentPassword() {
        return e(U);
      }, get passwords() {
        return e(J);
      }, set passwords(y) {
        o(J, m(y));
      }, get isValid() {
        return e(g);
      }, set isValid(y) {
        o(g, m(y));
      } });
      var G = v(L, 2), F = r(G);
      ye(F, { onclick: z, level: 1, get isLoading() {
        return e(b);
      }, children: (y, E) => {
        he();
        var O = we();
        D(() => l(O, a.common.save)), d(y, O);
      }, $$slots: { default: true } });
      var ce = v(F, 2);
      {
        var me = (y) => {
          var E = ea(), O = r(E);
          ut(O, {}), t(E), Le(3, E, () => Ne), d(y, E);
        }, le = (y, E) => {
          {
            var O = (H) => {
              var N = ta(), fe = r(N, true);
              t(N), D(() => l(fe, e(I))), Le(3, N, () => Ne), d(H, N);
            }, ve = (H, N) => {
              {
                var fe = (_e) => {
                  var be = ra(), Pe = r(be);
                  ye(Pe, { level: 3, onclick: () => o(i, false), children: (Ae, et) => {
                    he();
                    var Ue = we();
                    D(() => l(Ue, a.common.cancel)), d(Ae, Ue);
                  }, $$slots: { default: true } }), t(be), d(_e, be);
                };
                w(H, (_e) => {
                  e(i) && !e(b) && _e(fe);
                }, N);
              }
            };
            w(y, (H) => {
              e(I) ? H(O) : H(ve, false);
            }, E);
          }
        };
        w(ce, (y) => {
          e(u) ? y(me) : y(le, false);
        });
      }
      t(G), t(c);
      var de = v(c, 2);
      {
        var A = (y) => {
          var E = aa(), O = r(E), ve = r(O, true);
          t(O);
          var H = v(O, 2), N = r(H, true);
          t(H);
          var fe = v(H, 2);
          ye(fe, { level: 2, onclick: ae, children: (_e, be) => {
            he();
            var Pe = we();
            D(() => l(Pe, a.account.convertAccount)), d(_e, Pe);
          }, $$slots: { default: true } }), t(E), D(() => {
            l(ve, a.account.convertAccount), l(N, a.account.convertAccountP1);
          }), d(y, E);
        };
        w(de, (y) => {
          !e(i) && e(Y) && y(A);
        });
      }
      d(h, x);
    };
    w(j, (h) => {
      (e(n) === "password" || e(n) === "federated_password" || e(i)) && h(ue);
    });
  }
  t(re), d(oe, re), De();
}
function oa(oe, _) {
  oe.code === "Enter" && _();
}
var na = k('<div class="flex"><div class="label font-label noselect svelte-1xf5lr3"><!></div> <label class="switch svelte-1xf5lr3"><input type="checkbox" class="svelte-1xf5lr3"> <span class="slider slider-round svelte-1xf5lr3"></span></label></div>');
function It(oe, _) {
  let s = $e(_, "checked", 15, false), a = $e(_, "ariaLabel", 3, ""), P = $e(_, "name", 3, "");
  function n() {
    s(!s());
  }
  var T = na(), g = r(T);
  let i;
  var b = r(g);
  wr(b, () => _.children), t(g);
  var I = v(g, 2), u = r(I);
  er(u), u.__click = n, u.__keydown = [oa, n], he(2), t(I), t(T), D(() => {
    i = lt(g, "", i, { width: _.labelWidth }), ze(u, "name", P()), u.disabled = _.disabled, ze(u, "aria-checked", s()), ze(u, "aria-label", a());
  }), br(u, s), d(oe, T);
}
Xt(["click", "keydown"]);
var ia = k("<div><p> </p> <!></div>"), la = k('<div class="success svelte-5kivuv"><!></div>'), da = k('<div class="err svelte-5kivuv"> </div>'), ua = k('<div class="container svelte-5kivuv"><p> </p> <p><!></p> <div class="switch svelte-5kivuv"><!></div> <div class="switch svelte-5kivuv"><!></div> <!> <div class="bottom svelte-5kivuv"><!> <!></div></div>');
function $t(oe, _) {
  Ce(_, true);
  let s = $e(_, "webIdData", 15), a = Me();
  const P = "14rem";
  let n = M(""), T = M(false), g = M(!!s().custom_triples), i = ot(s().user_id);
  async function b() {
    o(n, "");
    let f = await dt(`/auth/v1/users/${s().user_id}/webid/data`, s());
    f.error ? o(n, m(f.error.message)) : (o(T, true), setTimeout(() => {
      o(T, false);
    }, 3e3));
  }
  var I = ua(), u = r(I), $ = r(u, true);
  t(u);
  var J = v(u, 2), Y = r(J);
  yr(Y, { href: i, target: "_blank", children: (f, R) => {
    var p = Re(), X = xe(p);
    Tt(X, () => i.replace("/auth/", "/auth/<wbr/>")), d(f, p);
  }, $$slots: { default: true } }), t(J);
  var Q = v(J, 2), ae = r(Q);
  It(ae, { ariaLabel: "E-Mail", labelWidth: P, get checked() {
    return s().expose_email;
  }, set checked(f) {
    s(s().expose_email = f, true);
  }, children: (f, R) => {
    he();
    var p = we("E-Mail");
    d(f, p);
  }, $$slots: { default: true } }), t(Q);
  var z = v(Q, 2), Z = r(z);
  It(Z, { get ariaLabel() {
    return a.account.webIdExpertMode;
  }, labelWidth: P, get checked() {
    return e(g);
  }, set checked(f) {
    o(g, m(f));
  }, children: (f, R) => {
    he();
    var p = we();
    D(() => l(p, a.account.webIdExpertMode)), d(f, p);
  }, $$slots: { default: true } }), t(z);
  var se = v(z, 2);
  {
    var B = (f) => {
      var R = ia(), p = r(R), X = r(p, true);
      t(p);
      var j = v(p, 2);
      xr(j, { placeholder: "FOAF", rows: 15, get value() {
        return s().custom_triples;
      }, set value(ue) {
        s(s().custom_triples = ue, true);
      } }), t(R), D(() => l(X, a.account.webIdDescData)), Le(3, R, () => ir), d(f, R);
    };
    w(se, (f) => {
      e(g) && f(B);
    });
  }
  var K = v(se, 2), ee = r(K);
  ye(ee, { onclick: b, children: (f, R) => {
    he();
    var p = we();
    D(() => l(p, a.common.save)), d(f, p);
  }, $$slots: { default: true } });
  var re = v(ee, 2);
  {
    var V = (f) => {
      var R = la(), p = r(R);
      ut(p, {}), t(R), Le(3, R, () => Ne), d(f, R);
    }, C = (f, R) => {
      {
        var p = (X) => {
          var j = da(), ue = r(j, true);
          t(j), D(() => l(ue, e(n))), Le(3, j, () => Ne), d(X, j);
        };
        w(f, (X) => {
          e(n) && X(p);
        }, R);
      }
    };
    w(re, (f) => {
      e(T) ? f(V) : f(C, false);
    });
  }
  t(K), t(I), D(() => l($, a.account.webIdDesc)), d(oe, I), De();
}
function ca(oe, _) {
  Ce(_, true);
  let s = ct("account"), a = ie(() => {
    var _a2;
    return (_a2 = s.get()) == null ? void 0 : _a2.user_id;
  });
  var P = Re(), n = xe(P);
  {
    var T = (g) => {
      Dt(g, { viewMode: "account", get userId() {
        return e(a);
      } });
    };
    w(n, (g) => {
      e(a) && g(T);
    });
  }
  d(oe, P), De();
}
var fa = k("<h3> </h3>"), _a = k('<div class="headerPhone svelte-3a6iwi"><!></div> <div class="container svelte-3a6iwi"><!> <div class="innerPhone svelte-3a6iwi"><!></div></div>', 1), pa = k('<div clasS="info svelte-3a6iwi"><!></div>'), ma = k('<div class="flex gap-05"><!> </div>'), ga = k('<div class="wide svelte-3a6iwi"><!> <div class="container svelte-3a6iwi"><!> <div class="inner svelte-3a6iwi"><!></div></div> <div class="logout svelte-3a6iwi"><!></div></div>'), ha = k('<!> <div class="wrapper svelte-3a6iwi"><!></div>', 1);
function wa(oe, _) {
  Ce(_, true);
  const s = (B) => {
    var K = fa(), ee = r(K, true);
    t(K), D(() => l(ee, `${a().given_name} ${a().family_name || ""}`)), d(B, K);
  };
  let a = $e(_, "user", 15), P = $e(_, "webIdData", 15), n = Me(), T = M(void 0), g = M(m([])), i = ie(() => {
    var _a2;
    if ((_a2 = a().account_type) == null ? void 0 : _a2.startsWith("federated")) return e(g).filter((B) => B.id === a().auth_provider_id)[0];
  }), b = ie(() => e(T) && e(T) < 560), I = ie(() => e(T) && e(T) < 1e3), u = M(m(n.account.navInfo)), $ = ie(() => P() ? [n.account.navInfo, n.account.navMfa, n.account.devices, n.account.navEdit, n.common.password, "WebID", n.account.navLogout] : [n.account.navInfo, n.account.navMfa, n.account.devices, n.account.navEdit, n.common.password, n.account.navLogout]), J = ie(() => P() ? [n.account.navMfa, n.account.devices, n.account.navEdit, n.common.password, "WebID"] : [n.account.navMfa, n.account.devices, n.account.navEdit, n.common.password]);
  Ze(() => {
    Pr("v").get() === "devices" && o(u, m(n.account.devices));
  }), Se(() => {
    e(b) || e(I) ? o(u, m(n.account.navInfo)) : o(u, m(n.account.navMfa));
  }), Se(() => {
    e(u) === n.account.navLogout && wt();
  });
  var Y = ha(), Q = xe(Y);
  kr(Q, { id: or, get value() {
    return e(g);
  }, set value(B) {
    o(g, m(B));
  } });
  var ae = v(Q, 2), z = r(ae);
  {
    var Z = (B) => {
      var K = _a(), ee = xe(K), re = r(ee);
      s(re), t(ee);
      var V = v(ee, 2), C = r(V);
      yt(C, { get tabs() {
        return e($);
      }, get selected() {
        return e(u);
      }, set selected(j) {
        o(u, m(j));
      } });
      var f = v(C, 2), R = r(f);
      {
        var p = (j) => {
          vt(j, { get providers() {
            return e(g);
          }, get authProvider() {
            return e(i);
          }, viewModePhone: true, get webIdData() {
            return P();
          }, get user() {
            return a();
          }, set user(ue) {
            a(ue);
          } });
        }, X = (j, ue) => {
          {
            var h = (c) => {
              xt(c, { viewModePhone: true, get user() {
                return a();
              }, set user(L) {
                a(L);
              } });
            }, x = (c, L) => {
              {
                var U = (F) => {
                  Pt(F, { get user() {
                    return a();
                  }, get authProvider() {
                    return e(i);
                  }, viewModePhone: true });
                }, G = (F, ce) => {
                  {
                    var me = (de) => {
                      kt(de, { get user() {
                        return a();
                      } });
                    }, le = (de, A) => {
                      {
                        var y = (O) => {
                          var ve = Re(), H = xe(ve);
                          {
                            var N = (fe) => {
                              $t(fe, { get webIdData() {
                                return P();
                              }, set webIdData(_e) {
                                P(_e);
                              } });
                            };
                            w(H, (fe) => {
                              P() && fe(N);
                            });
                          }
                          d(O, ve);
                        }, E = (O, ve) => {
                          {
                            var H = (N) => {
                              Dt(N, { get userId() {
                                return a().id;
                              } });
                            };
                            w(O, (N) => {
                              e(u) === n.account.devices && N(H);
                            }, ve);
                          }
                        };
                        w(de, (O) => {
                          e(u) === "WebID" ? O(y) : O(E, false);
                        }, A);
                      }
                    };
                    w(F, (de) => {
                      e(u) === n.account.navMfa ? de(me) : de(le, false);
                    }, ce);
                  }
                };
                w(c, (F) => {
                  e(u) === n.common.password ? F(U) : F(G, false);
                }, L);
              }
            };
            w(j, (c) => {
              e(u) === n.account.navEdit ? c(h) : c(x, false);
            }, ue);
          }
        };
        w(R, (j) => {
          e(u) === n.account.navInfo ? j(p) : j(X, false);
        });
      }
      t(f), t(V), d(B, K);
    }, se = (B) => {
      var K = ga(), ee = r(K);
      {
        var re = (x) => {
          var c = pa(), L = r(c);
          vt(L, { get webIdData() {
            return P();
          }, get providers() {
            return e(g);
          }, get authProvider() {
            return e(i);
          }, get user() {
            return a();
          }, set user(U) {
            a(U);
          } }), t(c), d(x, c);
        };
        w(ee, (x) => {
          e(I) || x(re);
        });
      }
      var V = v(ee, 2), C = r(V);
      const f = ie(() => e(I) ? e($) : e(J));
      yt(C, { get tabs() {
        return e(f);
      }, center: true, get selected() {
        return e(u);
      }, set selected(x) {
        o(u, m(x));
      } });
      var R = v(C, 2), p = r(R);
      {
        var X = (x) => {
          vt(x, { get webIdData() {
            return P();
          }, get providers() {
            return e(g);
          }, get authProvider() {
            return e(i);
          }, get user() {
            return a();
          }, set user(c) {
            a(c);
          } });
        }, j = (x, c) => {
          {
            var L = (G) => {
              xt(G, { get user() {
                return a();
              }, set user(F) {
                a(F);
              } });
            }, U = (G, F) => {
              {
                var ce = (le) => {
                  Pt(le, { get user() {
                    return a();
                  }, get authProvider() {
                    return e(i);
                  } });
                }, me = (le, de) => {
                  {
                    var A = (E) => {
                      kt(E, { get user() {
                        return a();
                      } });
                    }, y = (E, O) => {
                      {
                        var ve = (N) => {
                          var fe = Re(), _e = xe(fe);
                          {
                            var be = (Pe) => {
                              $t(Pe, { get webIdData() {
                                return P();
                              }, set webIdData(Ae) {
                                P(Ae);
                              } });
                            };
                            w(_e, (Pe) => {
                              P() && Pe(be);
                            });
                          }
                          d(N, fe);
                        }, H = (N, fe) => {
                          {
                            var _e = (be) => {
                              ca(be, {});
                            };
                            w(N, (be) => {
                              e(u) === n.account.devices && be(_e);
                            }, fe);
                          }
                        };
                        w(E, (N) => {
                          e(u) === "WebID" ? N(ve) : N(H, false);
                        }, O);
                      }
                    };
                    w(le, (E) => {
                      e(u) === n.account.navMfa ? E(A) : E(y, false);
                    }, de);
                  }
                };
                w(G, (le) => {
                  e(u) === n.common.password ? le(ce) : le(me, false);
                }, F);
              }
            };
            w(x, (G) => {
              e(u) === n.account.navEdit ? G(L) : G(U, false);
            }, c);
          }
        };
        w(p, (x) => {
          e(u) === n.account.navInfo ? x(X) : x(j, false);
        });
      }
      t(R), t(V);
      var ue = v(V, 2), h = r(ue);
      ye(h, { level: -3, onclick: wt, children: (x, c) => {
        var L = ma(), U = r(L);
        Ir(U, {});
        var G = v(U);
        t(L), D(() => {
          ze(L, "title", n.account.navLogout), l(G, ` ${n.account.navLogout ?? ""}`);
        }), d(x, L);
      }, $$slots: { default: true } }), t(ue), t(K), d(B, K);
    };
    w(z, (B) => {
      e(b) ? B(Z) : B(se, false);
    });
  }
  t(ae), tr("innerWidth", (B) => o(T, m(B))), d(oe, Y), De();
}
var ba = k("<!> <!> <!>", 1);
function ns(oe, _) {
  Ce(_, true);
  let s = Me(), a = ct("account"), P = M(void 0), n = M(void 0), T = M(false);
  Se(() => {
    let i = a.get();
    i && g(i);
  });
  async function g(i) {
    const b = i.user_id;
    if (b) {
      let I = await Promise.all([Ve(`/auth/v1/users/${b}`), Ve(`/auth/v1/users/${b}/webid/data`)]);
      I[0].body ? o(P, m(I[0].body)) : nr("account"), I[1].body ? o(n, m(I[1].body)) : I[1].status === 404 && o(n, m({ user_id: b, expose_email: false })), o(T, true);
    } else console.error("no user_id in session");
  }
  Zt((i) => {
    D(() => {
      var _a2;
      return Qt.title = `${(s == null ? void 0 : s.account.account) || "Account"} ${((_a2 = e(P)) == null ? void 0 : _a2.email) ?? ""}`;
    });
  }), $r(oe, { children: (i, b) => {
    Er(i, { children: (I, u) => {
      var $ = ba(), J = xe($);
      {
        var Y = (z) => {
          wa(z, { get user() {
            return e(P);
          }, set user(Z) {
            o(P, m(Z));
          }, get webIdData() {
            return e(n);
          }, set webIdData(Z) {
            o(n, m(Z));
          } });
        };
        w(J, (z) => {
          e(T) && a && e(P) && z(Y);
        });
      }
      var Q = v(J, 2);
      Tr(Q, { absolute: true });
      var ae = v(Q, 2);
      Cr(ae, { absolute: true }), d(I, $);
    } });
  } }), De();
}
export {
  ns as component
};
