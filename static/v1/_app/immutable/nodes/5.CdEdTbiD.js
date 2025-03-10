import { t as I, a as v, d as fe, e as be } from "../chunks/BH6NCLk-.js";
import { p as $e, aa as ze, j as e, k as D, l as o, c as a, a9 as _e, r as t, s as n, t as N, a as De, f as Q, a7 as we, a5 as tt, a6 as Qt } from "../chunks/CvlvO1XB.js";
import { e as nt, s as c, b as Zt, d as er, h as tr } from "../chunks/CTI4QPiR.js";
import { i as k } from "../chunks/BUO_AUgz.js";
import { p as b } from "../chunks/Wh68IIk2.js";
import { b as rr } from "../chunks/7lxiEjMQ.js";
import { p as Ie } from "../chunks/C6SR4G2t.js";
import { g as ar, C as sr, l as Ze, U as it, K as or, P as nr, M as vr, A as ir, V as lr, F as dr, r as ur } from "../chunks/Brp0G0eV.js";
import { e as $t } from "../chunks/BpWRzPRQ.js";
import { h as Dt } from "../chunks/i8Xqpu09.js";
import { s as Te, a as q, c as X, r as cr } from "../chunks/BMbqVy6X.js";
import { a as Me, B as ke, t as Se, f as We, s as fr } from "../chunks/DMkkW5Nn.js";
import { C as et } from "../chunks/CS_Msctd.js";
import { M as _r } from "../chunks/C712LKcc.js";
import { u as Ne } from "../chunks/CUqQZdNU.js";
import { c as ut, b as lt, d as ct, f as Be } from "../chunks/CRjU5SuJ.js";
import { B as mr } from "../chunks/DQDimnsq.js";
import { a as pr, c as gr } from "../chunks/dU6E9WaN.js";
import { I as hr } from "../chunks/BWnKfvFA.js";
import { I as wr } from "../chunks/Vi3uK7uO.js";
import { I as Le } from "../chunks/CJP-ccgO.js";
import { F as br } from "../chunks/CW8abPUe.js";
import { b as dt, k as yr, l as yt, m as xr } from "../chunks/BRCxk8by.js";
import { I as ft } from "../chunks/Nks81rMs.js";
import { I as kr } from "../chunks/oGfQq50S.js";
import { u as _t } from "../chunks/CO1TlAUy.js";
import { w as Pr } from "../chunks/D7n-kBd8.js";
import { W as Lt } from "../chunks/DEx36zkG.js";
import { U as Ir, D as Nt } from "../chunks/BQ25PIO-.js";
import { P as Er } from "../chunks/DIB4eys6.js";
import { I as vt } from "../chunks/B-xX0s4n.js";
import { s as Tr } from "../chunks/-T201g_q.js";
import { A as Cr } from "../chunks/kx3e4WWQ.js";
import { I as $r } from "../chunks/Cxg1YoLR.js";
import { T as Dr } from "../chunks/Bb2Y1h4J.js";
import { u as Lr } from "../chunks/BNumlRaG.js";
import { T as xt } from "../chunks/_OE2Cq0B.js";
import { M as Nr } from "../chunks/BbRujPvb.js";
import { C as Ar } from "../chunks/GWgockW8.js";
import { T as Mr } from "../chunks/Cbuxv9a6.js";
import { L as zr } from "../chunks/BCG8E-Sv.js";
var Sr = I("<span> </span>"), Rr = I('<img loading="lazy" class="absolute svelte-1390lfo" aria-label="Avatar" alt="" width="100%">'), Wr = I('<div class="errLimit svelte-1390lfo"> </div>'), Ur = I("<div><!></div>"), Fr = I('<div class="relative"><div class="delete svelte-1390lfo"><!></div></div>'), Vr = I('<div class="container svelte-1390lfo"><form class="avatar svelte-1390lfo" aria-dropeffect="move" aria-label="Upload"><label class="svelte-1390lfo"><!></label> <input type="file" aria-hidden="true" class="svelte-1390lfo"> <!> <!></form> <!></div>');
function qr(ve, m) {
  $e(m, true);
  const s = (_) => {
    var d = fe(), T = Q(d);
    {
      var R = (y) => {
        var $ = Sr(), Z = a($, true);
        t($), N((ue) => {
          q($, 1, `absolute font-mono font-${M()} noselect`, "svelte-1390lfo"), c(Z, ue);
        }, [() => m.fallbackCharacters.toUpperCase()]), v(y, $);
      }, te = (y) => {
        var $ = Rr();
        N(() => Te($, "src", `/auth/v1/users/${m.userId}/picture/${r()}`)), nt("error", $, U), Zt($), v(y, $);
      };
      k(T, (y) => {
        e(l) || !r() ? y(R) : y(te, false);
      });
    }
    v(_, d);
  };
  let r = Ie(m, "pictureId", 15), M = Ie(m, "size", 3, "medium");
  const i = ar(), z = "image/png, image/jpeg, image/webp";
  let h = Ne(), l = D(""), p = D(""), f, O = D(false), P = D(false), W = D(void 0), Y = _e(ne), G = _e(() => {
    switch (M()) {
      case "small":
        return "1.5rem";
      case "medium":
        return "3rem";
      default:
        return "192px";
    }
  });
  ze(() => {
    e(l) && console.error(e(l));
  }), ze(() => {
    e(p) && (f && clearTimeout(f), f = setTimeout(() => {
      o(p, "");
    }, 5e3));
  }), ze(() => {
    e(W) && e(W).length > 0 && se(e(W));
  });
  function ne() {
    return `hsl(${ee(m.fallbackCharacters)}, 50%, 50%)`;
  }
  function U(_) {
    _.type === "error" && o(l, "Input Error");
  }
  function L() {
    o(O, true);
  }
  function j() {
    o(O, false);
  }
  function ee(_) {
    let d = [1, 255, 3, 13, 19];
    for (let T = 0; T < 5; T++) {
      let R = _.charCodeAt(T);
      if (R) d[T] = R;
      else break;
    }
    return (d[0] * (64 + d[1]) + 4 * d[0] + (d[2] * d[3] - d[4])) % 360;
  }
  async function ae() {
    if (o(l, ""), o(p, ""), !r()) return;
    let _ = await ut(`/auth/v1/users/${m.userId}/picture/${r()}`);
    _.error ? o(l, b(_.error.message)) : r(void 0);
  }
  async function se(_) {
    o(l, ""), o(p, ""), o(P, true);
    let d = `/auth/v1/users/${m.userId}/picture`, T = await fetch(d, { method: "HEAD" });
    if (!T.ok) {
      o(l, "Cannot do HEAD request to picture upload");
      return;
    }
    let R = T.headers.get("X-Content-Length-Limit");
    if (!R) {
      o(l, "No X-Content-Length-Limit in HEAD request headers");
      return;
    }
    let te = Number.parseInt(R);
    console.log("content length limit: ", te);
    for (let y of _) {
      if (!z.includes(y.type)) {
        o(l, "Invalid File Format, allowed: " + z);
        break;
      }
      if (y.size > te) {
        o(p, `${h.common.maxFileSize}: ${te / 1024 / 1024} MB`);
        break;
      }
      let $ = new FormData();
      $.append(y.name, y);
      let Z = await fetch(d, { method: "PUT", headers: { "csrf-token": localStorage.getItem(sr) || "" }, body: $ });
      if (Z.ok) r(await Z.text());
      else {
        let ue = await Z.json();
        Z.status === 406 ? alert("max size" + ue.message) : o(l, b(ue.message || "Upload Error"));
      }
    }
    o(W, void 0), o(P, false);
  }
  var J = Vr(), S = a(J), E = a(S);
  Te(E, "for", i), Te(E, "aria-controls", i);
  var g = a(E);
  hr(g, { get width() {
    return e(G);
  } }), t(E);
  var A = n(E, 2);
  Te(A, "id", i), Te(A, "accept", z);
  var u = n(A, 2);
  {
    var B = (_) => {
      var d = Wr(), T = a(d, true);
      t(d), N(() => {
        Me(d, "width", e(G)), c(T, e(p));
      }), v(_, d);
    };
    k(u, (_) => {
      e(p) && _(B);
    });
  }
  var ie = n(u, 2);
  s(ie), t(S);
  var le = n(S, 2);
  {
    var w = (_) => {
      var d = Fr(), T = a(d), R = a(T);
      ke(R, { invisible: true, onclick: ae, children: (te, y) => {
        var $ = Ur(), Z = a($);
        wr(Z, {}), t($), N(() => Te($, "title", h.common.delete)), v(te, $);
      }, $$slots: { default: true } }), t(T), t(d), v(_, d);
    };
    k(le, (_) => {
      M() === "large" && r() && _(w);
    });
  }
  t(J), N(() => {
    Me(S, "background-color", e(Y)), Me(S, "width", e(G)), Te(E, "aria-disabled", e(P)), Te(E, "data-show", !e(P) && e(O)), Me(E, "width", e(G)), A.disabled = e(P), Te(A, "aria-disabled", e(P));
  }), nt("mouseenter", S, L), nt("mouseleave", S, j), pr(A, () => e(W), (_) => o(W, _)), v(ve, J), De();
}
var Br = I('<div class="link-err value svelte-1tea8v4"> </div>'), Hr = I('<div class="fed-btn svelte-1tea8v4"><!> <!></div>'), Kr = I('<h3> </h3> <p> </p> <div class="providers svelte-1tea8v4"></div>', 1), Or = I("<!> <!>", 1), jr = I('<div><div> </div> <span class="value svelte-1tea8v4"> </span></div>'), Xr = I('<div><div> </div> <span class="value svelte-1tea8v4"> </span></div>'), Yr = I('<div><div>WebID:</div> <span class="value svelte-1tea8v4"><a target="_blank"><!></a></span></div>'), Gr = I('<div><div><div></div> <!></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <div><div class="value svelte-1tea8v4"> </div> <!></div></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <!></div> <div><div> </div> <!></div> <div><div> </div> <!></div> <!> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <!> <!></div>');
function kt(ve, m) {
  $e(m, true);
  let s = Ie(m, "user", 15), r = Ne(), M = D(false), i = D(false), z = _e(() => {
    var _a2;
    return (_a2 = s().account_type) == null ? void 0 : _a2.startsWith("federated");
  }), h = _e(() => {
    var _a2;
    return e(z) ? `${s().account_type}: ${((_a2 = m.authProvider) == null ? void 0 : _a2.name) || ""}` : s().account_type;
  }), l = _e(() => m.viewModePhone ? "rowPhone" : "row"), p = _e(() => m.viewModePhone ? "labelPhone" : "label"), f = _e(() => {
    let V = s().given_name[0];
    return s().family_name && s().family_name.length > 0 && (V += s().family_name[0]), V;
  });
  function O(V) {
    or(64, (re, { challenge: de, verifier: he }) => {
      re || (localStorage.setItem(nr, he), P(V, de));
    });
  }
  async function P(V, re) {
    let de = { email: s().email, client_id: "rauthy", redirect_uri: window.location.href, provider_id: V, pkce_challenge: re }, he = await lt(`/auth/v1/providers/${V}/link`, de);
    if (he.text) {
      vr(he.text);
      let pe = he.headers.get("location");
      pe && (window.location.href = pe);
    } else console.error(he.error);
  }
  async function W() {
    let V = await ut("/auth/v1/providers/link");
    V.body ? s(V.body) : (console.error(V.error), o(M, true));
  }
  var Y = Gr(), G = a(Y), ne = a(G), U = n(ne, 2);
  qr(U, { get fallbackCharacters() {
    return e(f);
  }, get userId() {
    return s().id;
  }, size: "large", get pictureId() {
    return s().picture_id;
  }, set pictureId(V) {
    s(s().picture_id = V, true);
  } }), t(G);
  var L = n(G, 2), j = a(L), ee = a(j, true);
  t(j);
  var ae = n(j, 2), se = a(ae, true);
  t(ae), t(L);
  var J = n(L, 2), S = a(J), E = a(S, true);
  t(S);
  var g = n(S, 2), A = a(g, true);
  t(g), t(J);
  var u = n(J, 2), B = a(u), ie = a(B, true);
  t(B);
  var le = n(B, 2), w = a(le, true);
  t(le), t(u);
  var _ = n(u, 2), d = a(_), T = a(d);
  t(d);
  var R = n(d, 2), te = a(R, true);
  t(R), t(_);
  var y = n(_, 2), $ = a(y), Z = a($, true);
  t($);
  var ue = n($, 2), ye = a(ue), C = a(ye, true);
  t(ye);
  var x = n(ye, 2);
  {
    var F = (V) => {
      var re = Hr(), de = a(re);
      ke(de, { get ariaLabel() {
        return r.account.providerUnlink;
      }, level: 3, onclick: W, children: (Pe, Ee) => {
        we();
        var Re = be();
        N(() => c(Re, r.account.providerUnlink)), v(Pe, Re);
      }, $$slots: { default: true } });
      var he = n(de, 2);
      {
        var pe = (Pe) => {
          var Ee = Br(), Re = a(Ee, true);
          t(Ee), N(() => c(Re, r.account.providerUnlinkDesc)), v(Pe, Ee);
        };
        k(he, (Pe) => {
          e(M) && Pe(pe);
        });
      }
      t(re), v(V, re);
    }, oe = (V) => {
      var re = fe(), de = Q(re);
      {
        var he = (pe) => {
          var Pe = Or(), Ee = Q(Pe);
          ke(Ee, { level: 2, onclick: () => o(i, true), children: (Ve, Xt) => {
            we();
            var qe = be();
            N(() => c(qe, r.account.providerLink)), v(Ve, qe);
          }, $$slots: { default: true } });
          var Re = n(Ee, 2);
          _r(Re, { get showModal() {
            return e(i);
          }, set showModal(Ve) {
            o(i, b(Ve));
          }, children: (Ve, Xt) => {
            var qe = Kr(), at = Q(qe), Yt = a(at, true);
            t(at);
            var st = n(at, 2), Gt = a(st, true);
            t(st);
            var wt = n(st, 2);
            $t(wt, 21, () => m.providers, (ot) => ot.id, (ot, bt) => {
              const Jt = _e(() => `${r.account.providerLink}: ${e(bt).name}`);
              mr(ot, { get ariaLabel() {
                return e(Jt);
              }, get provider() {
                return e(bt);
              }, onclick: O });
            }), t(wt), N(() => {
              c(Yt, r.account.providerLink), c(Gt, r.account.providerLinkDesc);
            }), v(Ve, qe);
          }, $$slots: { default: true } }), v(pe, Pe);
        };
        k(de, (pe) => {
          m.providers.length > 0 && pe(he);
        }, true);
      }
      v(V, re);
    };
    k(x, (V) => {
      e(z) ? V(F) : V(oe, false);
    });
  }
  t(ue), t(y);
  var ce = n(y, 2), H = a(ce), K = a(H, true);
  t(H);
  var me = n(H, 2), ge = a(me, true);
  t(me), t(ce);
  var xe = n(ce, 2), Ce = a(xe), Ue = a(Ce, true);
  t(Ce);
  var He = n(Ce, 2), rt = a(He, true);
  t(He), t(xe);
  var Fe = n(xe, 2), Ae = a(Fe), At = a(Ae, true);
  t(Ae);
  var Mt = n(Ae, 2);
  const zt = _e(() => !!s().webauthn_user_id);
  et(Mt, { get checked() {
    return e(zt);
  } }), t(Fe);
  var Ke = n(Fe, 2), Oe = a(Ke), St = a(Oe, true);
  t(Oe);
  var Rt = n(Oe, 2);
  et(Rt, { get checked() {
    return s().enabled;
  } }), t(Ke);
  var je = n(Ke, 2), Xe = a(je), Wt = a(Xe, true);
  t(Xe);
  var Ut = n(Xe, 2);
  et(Ut, { get checked() {
    return s().email_verified;
  } }), t(je);
  var mt = n(je, 2);
  {
    var Ft = (V) => {
      var re = jr(), de = a(re), he = a(de, true);
      t(de);
      var pe = n(de, 2), Pe = a(pe, true);
      t(pe), t(re), N((Ee) => {
        q(re, 1, X(e(l)), "svelte-1tea8v4"), q(de, 1, X(e(p)), "svelte-1tea8v4"), c(he, r.account.lastLogin), c(Pe, Ee);
      }, [() => Ze(s().last_login)]), v(V, re);
    };
    k(mt, (V) => {
      s().last_login && V(Ft);
    });
  }
  var Ye = n(mt, 2), Ge = a(Ye), Vt = a(Ge, true);
  t(Ge);
  var pt = n(Ge, 2), qt = a(pt, true);
  t(pt), t(Ye);
  var Je = n(Ye, 2), Qe = a(Je), Bt = a(Qe, true);
  t(Qe);
  var gt = n(Qe, 2), Ht = a(gt, true);
  t(gt), t(Je);
  var ht = n(Je, 2);
  {
    var Kt = (V) => {
      var re = Xr(), de = a(re), he = a(de, true);
      t(de);
      var pe = n(de, 2), Pe = a(pe, true);
      t(pe), t(re), N((Ee) => {
        q(re, 1, X(e(l)), "svelte-1tea8v4"), q(de, 1, X(e(p)), "svelte-1tea8v4"), c(he, r.account.userExpiry), c(Pe, Ee);
      }, [() => Ze(s().user_expires)]), v(V, re);
    };
    k(ht, (V) => {
      s().user_expires && V(Kt);
    });
  }
  var Ot = n(ht, 2);
  {
    var jt = (V) => {
      var re = Yr(), de = a(re), he = n(de, 2), pe = a(he), Pe = a(pe);
      Dt(Pe, () => it(s().id).replace("/auth/", "/auth/<wbr/>")), t(pe), t(he), t(re), N((Ee) => {
        q(re, 1, X(e(l)), "svelte-1tea8v4"), q(de, 1, X(e(p)), "svelte-1tea8v4"), Te(pe, "href", Ee);
      }, [() => it(s().id)]), v(V, re);
    };
    k(Ot, (V) => {
      m.webIdData && V(jt);
    });
  }
  t(Y), N((V, re) => {
    q(G, 1, X(e(l)), "svelte-1tea8v4"), Me(G, "margin", ".5rem 0"), q(ne, 1, X(e(p)), "svelte-1tea8v4"), q(L, 1, X(e(l)), "svelte-1tea8v4"), q(j, 1, X(e(p)), "svelte-1tea8v4"), c(ee, r.common.email), c(se, s().email), q(J, 1, X(e(l)), "svelte-1tea8v4"), q(S, 1, X(e(p)), "svelte-1tea8v4"), c(E, r.account.givenName), c(A, s().given_name), q(u, 1, X(e(l)), "svelte-1tea8v4"), q(B, 1, X(e(p)), "svelte-1tea8v4"), c(ie, r.account.familyName), c(w, s().family_name), q(_, 1, X(e(l)), "svelte-1tea8v4"), q(d, 1, X(e(p)), "svelte-1tea8v4"), c(T, `${r.account.user ?? ""} ID`), c(te, s().id), q(y, 1, X(e(l)), "svelte-1tea8v4"), q($, 1, X(e(p)), "svelte-1tea8v4"), c(Z, r.account.accType), c(C, e(h) || ""), q(ce, 1, X(e(l)), "svelte-1tea8v4"), q(H, 1, X(e(p)), "svelte-1tea8v4"), c(K, r.account.roles), c(ge, s().roles || "None"), q(xe, 1, X(e(l)), "svelte-1tea8v4"), q(Ce, 1, X(e(p)), "svelte-1tea8v4"), c(Ue, r.account.groups), c(rt, s().groups || "None"), q(Fe, 1, X(e(l)), "svelte-1tea8v4"), q(Ae, 1, X(e(p)), "svelte-1tea8v4"), c(At, r.account.mfaActivated), q(Ke, 1, X(e(l)), "svelte-1tea8v4"), q(Oe, 1, X(e(p)), "svelte-1tea8v4"), c(St, r.account.userEnabled), q(je, 1, X(e(l)), "svelte-1tea8v4"), q(Xe, 1, X(e(p)), "svelte-1tea8v4"), c(Wt, r.account.emailVerified), q(Ye, 1, X(e(l)), "svelte-1tea8v4"), q(Ge, 1, X(e(p)), "svelte-1tea8v4"), c(Vt, r.account.passwordExpiry), c(qt, V), q(Je, 1, X(e(l)), "svelte-1tea8v4"), q(Qe, 1, X(e(p)), "svelte-1tea8v4"), c(Bt, r.account.userCreated), c(Ht, re);
  }, [() => s().password_expires && Ze(s().password_expires) || r.common.never, () => Ze(s().created_at)]), v(ve, Y), De();
}
var Jr = I('<div class="success svelte-4ogr3z"><!></div>'), Qr = I("<p> </p>"), Zr = I('<div class="err svelte-4ogr3z"> </div>'), ea = I('<div class="formInner svelte-4ogr3z"><div><!> <!> <!> <!></div> <div><!> <!> <!> <!> <!></div></div> <div class="bottom svelte-4ogr3z"><div><!></div> <!></div> <!>', 1), ta = I('<div class="container svelte-4ogr3z"><!></div>');
function Pt(ve, m) {
  var _a2;
  $e(m, true);
  let s = Ie(m, "user", 15);
  ((_a2 = s().user_values) == null ? void 0 : _a2.birthdate) || s(s().user_values.birthdate = "", true);
  let r = Ne(), M = D(""), i = D(false), z = D(false);
  async function h(O, P) {
    var _a3;
    const W = P.get("email"), Y = P.get("family_name") || void 0, G = P.get("given_name") || void 0, ne = P.get("birthdate") || void 0, U = ((_a3 = P.get("phone")) == null ? void 0 : _a3.replaceAll(" ", "")) || void 0, L = P.get("street") || void 0, j = P.get("zip") || void 0, ee = j ? Number.parseInt(j) : void 0, ae = P.get("city") || void 0, se = P.get("country") || void 0;
    let J = { email: W, family_name: Y, given_name: G };
    (ne || U || L || ee || ae || se) && (J.user_values = { birthdate: ne, phone: U, street: L, zip: ee, city: ae, country: se });
    let S = await ct(`/auth/v1/users/${s().id}/self`, J);
    if (S.body) {
      o(i, true), s(S.body);
      let E = 3e3;
      S.status === 202 && (o(z, true), E = 3e4), setTimeout(() => {
        o(i, false);
      }, E);
    } else S.error && (console.error(S.error), o(M, b(S.error.message)));
  }
  var l = ta(), p = a(l);
  const f = _e(() => `/auth/v1/users/${s().id}/self`);
  br(p, { get action() {
    return e(f);
  }, onSubmit: h, children: (O, P) => {
    var W = ea(), Y = Q(W), G = a(Y), ne = a(G);
    Le(ne, { typ: "email", name: "email", get label() {
      return r.common.email;
    }, get placeholder() {
      return r.common.email;
    }, get value() {
      return s().email;
    }, required: true });
    var U = n(ne, 2);
    Le(U, { name: "given_name", autocomplete: "given-name", get label() {
      return r.account.givenName;
    }, get placeholder() {
      return r.account.givenName;
    }, get value() {
      return s().given_name;
    }, required: true, maxLength: 32, pattern: dt });
    var L = n(U, 2);
    Le(L, { name: "family_name", autocomplete: "family-name", get label() {
      return r.account.familyName;
    }, get placeholder() {
      return r.account.familyName;
    }, get value() {
      return s().family_name;
    }, maxLength: 32, pattern: dt });
    var j = n(L, 2);
    kr(j, { name: "birthdate", get label() {
      return r.account.birthdate;
    }, withDelete: true, get value() {
      return s().user_values.birthdate;
    }, set value(d) {
      s(s().user_values.birthdate = d, true);
    } }), t(G);
    var ee = n(G, 2), ae = a(ee);
    Le(ae, { name: "street", autocomplete: "street-address", get label() {
      return r.account.street;
    }, get placeholder() {
      return r.account.street;
    }, get value() {
      return s().user_values.street;
    }, maxLength: 48, pattern: yr });
    var se = n(ae, 2);
    Le(se, { typ: "number", name: "zip", autocomplete: "postal-code", get label() {
      return r.account.zip;
    }, get placeholder() {
      return r.account.zip;
    }, get value() {
      return s().user_values.zip;
    }, min: "1000", max: "9999999" });
    var J = n(se, 2);
    Le(J, { name: "city", autocomplete: "address-level2", get label() {
      return r.account.city;
    }, get placeholder() {
      return r.account.city;
    }, get value() {
      return s().user_values.city;
    }, maxLength: 48, pattern: yt });
    var S = n(J, 2);
    Le(S, { name: "country", autocomplete: "country", get label() {
      return r.account.country;
    }, get placeholder() {
      return r.account.country;
    }, get value() {
      return s().user_values.country;
    }, maxLength: 48, pattern: yt });
    var E = n(S, 2);
    Le(E, { name: "phone", autocomplete: "tel", get label() {
      return r.account.phone;
    }, get placeholder() {
      return r.account.phone;
    }, get value() {
      return s().user_values.phone;
    }, maxLength: 32, pattern: xr }), t(ee), t(Y);
    var g = n(Y, 2), A = a(g), u = a(A);
    ke(u, { type: "submit", children: (d, T) => {
      we();
      var R = be();
      N(() => c(R, r.common.save)), v(d, R);
    }, $$slots: { default: true } }), t(A);
    var B = n(A, 2);
    {
      var ie = (d) => {
        var T = Jr(), R = a(T);
        ft(R, {}), t(T), Se(3, T, () => We), v(d, T);
      };
      k(B, (d) => {
        e(i) && d(ie);
      });
    }
    t(g);
    var le = n(g, 2);
    {
      var w = (d) => {
        var T = Qr(), R = a(T, true);
        t(T), N(() => c(R, r.account.emailUpdateConfirm)), v(d, T);
      }, _ = (d) => {
        var T = fe(), R = Q(T);
        {
          var te = (y) => {
            var $ = Zr(), Z = a($, true);
            t($), N(() => c(Z, e(M))), Se(3, $, () => We), v(y, $);
          };
          k(R, (y) => {
            e(M) && y(te);
          }, true);
        }
        v(d, T);
      };
      k(le, (d) => {
        e(z) ? d(w) : d(_, false);
      });
    }
    v(O, W);
  }, $$slots: { default: true } }), t(l), v(ve, l), De();
}
var ra = I('<div class="err svelte-16mlupu"><b>Your browser does not support Webauthn credentials and must be updated.</b></div>'), aa = I('<!> <div class="regBtns svelte-16mlupu"><!> <!></div>', 1), sa = I('<div class="regNewBtn svelte-16mlupu"><!></div>'), oa = I('<div class="keysHeader svelte-16mlupu"> </div>'), na = I('<div class="button svelte-16mlupu"><!></div>'), va = I('<!> <p class="svelte-16mlupu"> <br><br> </p> <!> <!> <div class="keysContainer svelte-16mlupu"></div> <!> <div> </div>', 1), ia = I('<div class="container svelte-16mlupu"><!></div>');
function It(ve, m) {
  $e(m, true);
  const s = "credentials" in navigator;
  let r = Ne(), M = _t("account"), i = _e(() => {
    var _a2;
    return (_a2 = M.get()) == null ? void 0 : _a2.user_id;
  }), z = D(void 0), h = D(false), l = D(""), p = D(false), f = D(m.user.account_type === "password"), O = D(void 0), P = D(""), W = D(false), Y = D(b([]));
  tt(() => {
    ne();
  }), ze(() => {
    e(Y).length > 0 && m.user.account_type === "passkey" && o(f, e(Y).length > 1);
  }), ze(() => {
    var _a2;
    (_a2 = e(z)) == null ? void 0 : _a2.focus();
  });
  function G() {
    o(h, false), o(l, "");
  }
  async function ne() {
    var _a2;
    let E = await Be(`/auth/v1/users/${(_a2 = M.get()) == null ? void 0 : _a2.user_id}/webauthn`);
    E.body ? o(Y, b(E.body)) : console.error(E.error);
  }
  async function U() {
    if (G(), e(W) || !e(i)) return;
    if (e(P).length < 1) {
      o(h, true), o(l, b(r.mfa.passkeyNameErr));
      return;
    }
    let E = await Pr(e(i), e(P), r.authorize.invalidKeyUsed, r.authorize.requestExpired);
    E.error ? (o(h, true), o(l, `${r.mfa.errorReg} - ${E.error}`)) : (o(p, false), o(P, ""), await ne());
  }
  async function L(E) {
    var _a2;
    let g = await ut(`/auth/v1/users/${m.user.id}/webauthn/delete/${E}`);
    g.status === 200 ? await ne() : o(l, b(((_a2 = g.error) == null ? void 0 : _a2.message) || "Error"));
  }
  function j(E) {
    o(O, void 0), o(h, true), o(l, b(E)), setTimeout(() => {
      o(h, false), o(l, "");
    }, 5e3);
  }
  function ee(E) {
    o(O, void 0), o(l, b(r.mfa.testSuccess)), setTimeout(() => {
      o(l, "");
    }, 3e3);
  }
  var ae = ia(), se = a(ae);
  {
    var J = (E) => {
      var g = ra();
      v(E, g);
    }, S = (E) => {
      var g = va(), A = Q(g);
      {
        var u = (C) => {
          Lt(C, { get userId() {
            return m.user.id;
          }, get purpose() {
            return e(O);
          }, onSuccess: ee, onError: j });
        };
        k(A, (C) => {
          e(O) && C(u);
        });
      }
      var B = n(A, 2), ie = a(B), le = n(ie, 3);
      t(B);
      var w = n(B, 2);
      {
        var _ = (C) => {
          var x = aa(), F = Q(x);
          Le(F, { autocomplete: "off", get label() {
            return r.mfa.passkeyName;
          }, get placeholder() {
            return r.mfa.passkeyName;
          }, maxLength: 32, pattern: dt, onEnter: U, get ref() {
            return e(z);
          }, set ref(K) {
            o(z, b(K));
          }, get value() {
            return e(P);
          }, set value(K) {
            o(P, b(K));
          }, get isError() {
            return e(W);
          }, set isError(K) {
            o(W, b(K));
          } });
          var oe = n(F, 2), ce = a(oe);
          ke(ce, { onclick: U, children: (K, me) => {
            we();
            var ge = be();
            N(() => c(ge, r.mfa.register)), v(K, ge);
          }, $$slots: { default: true } });
          var H = n(ce, 2);
          ke(H, { level: 3, onclick: () => o(p, false), children: (K, me) => {
            we();
            var ge = be();
            N(() => c(ge, r.common.cancel)), v(K, ge);
          }, $$slots: { default: true } }), t(oe), v(C, x);
        }, d = (C) => {
          var x = sa(), F = a(x);
          const oe = _e(() => e(Y).length === 0 ? 1 : 2);
          ke(F, { get level() {
            return e(oe);
          }, onclick: () => o(p, true), children: (ce, H) => {
            we();
            var K = be();
            N(() => c(K, r.mfa.registerNew)), v(ce, K);
          }, $$slots: { default: true } }), t(x), v(C, x);
        };
        k(w, (C) => {
          e(p) ? C(_) : C(d, false);
        });
      }
      var T = n(w, 2);
      {
        var R = (C) => {
          var x = oa(), F = a(x, true);
          t(x), N(() => c(F, r.mfa.registerdKeys)), v(C, x);
        };
        k(T, (C) => {
          e(Y).length > 0 && C(R);
        });
      }
      var te = n(T, 2);
      $t(te, 21, () => e(Y), (C) => C.name, (C, x) => {
        Ir(C, { get passkey() {
          return e(x);
        }, get showDelete() {
          return e(f);
        }, onDelete: L });
      }), t(te);
      var y = n(te, 2);
      {
        var $ = (C) => {
          var x = na(), F = a(x);
          ke(F, { onclick: () => o(O, "Test"), children: (oe, ce) => {
            we();
            var H = be();
            N(() => c(H, r.mfa.test)), v(oe, H);
          }, $$slots: { default: true } }), t(x), v(C, x);
        };
        k(y, (C) => {
          e(Y).length > 0 && C($);
        });
      }
      var Z = n(y, 2);
      let ue;
      var ye = a(Z, true);
      t(Z), N(() => {
        c(ie, `${r.mfa.p1 ?? ""} `), c(le, ` ${r.mfa.p2 ?? ""}`), ue = q(Z, 1, "svelte-16mlupu", null, ue, { success: !e(h), err: e(h) }), c(ye, e(l));
      }), v(E, g);
    };
    k(se, (E) => {
      s ? E(S, false) : E(J);
    });
  }
  t(ae), v(ve, ae), De();
}
var la = I('<div class="container svelte-ue7bk2"><!> <!> <!> <!> <div class="btn svelte-ue7bk2"><!></div> <div class="err svelte-ue7bk2"> </div></div>');
function da(ve, m) {
  $e(m, true);
  let s = Ie(m, "passwords", 15), r = Ie(m, "hideCurrentPassword", 3, false), M = Ie(m, "isValid", 15), i = Ne();
  M(P);
  let z = D(false), h = D(""), l = D(void 0), p = _e(() => {
    var _a2;
    return ((_a2 = s().new) == null ? void 0 : _a2.length) > 6 && s().new === s().newConfirm;
  }), f = D(void 0), O = D(void 0);
  tt(async () => {
    let U = await Be("/auth/v1/password_policy");
    U.body ? o(l, b(U.body)) : console.error(U.error);
  });
  function P() {
    return o(h, ""), !r() && !s().current ? (o(h, b(i.account.passwordCurrReq)), false) : s().new ? s().newConfirm ? e(z) ? s().new.length > 256 ? (o(h, "max 256"), false) : s().new !== s().newConfirm ? (o(h, b(i.account.passwordNoMatch)), false) : true : (o(h, b(i.account.passwordPolicyFollow)), false) : (o(h, b(i.account.passwordNewReq)), false) : (o(h, b(i.account.passwordNewReq)), false);
  }
  function W() {
    if (o(h, ""), e(l)) {
      let U = ir(e(l));
      s(s().new = U, true), s(s().newConfirm = U, true);
    }
    requestAnimationFrame(() => {
      var _a2, _b;
      (_a2 = e(f)) == null ? void 0 : _a2(), (_b = e(O)) == null ? void 0 : _b();
    });
  }
  var Y = fe(), G = Q(Y);
  {
    var ne = (U) => {
      var L = la(), j = a(L);
      Er(j, { get policy() {
        return e(l);
      }, get password() {
        return s().new;
      }, get accepted() {
        return e(z);
      }, set accepted(u) {
        o(z, b(u));
      } });
      var ee = n(j, 2);
      {
        var ae = (u) => {
          vt(u, { autocomplete: "current-password", get label() {
            return i.account.passwordCurr;
          }, get placeholder() {
            return i.account.passwordCurr;
          }, onInput: P, get width() {
            return m.inputWidth;
          }, get value() {
            return s().current;
          }, set value(B) {
            s(s().current = B, true);
          } });
        };
        k(ee, (u) => {
          r() || u(ae);
        });
      }
      var se = n(ee, 2);
      vt(se, { autocomplete: "new-password", get label() {
        return i.account.passwordNew;
      }, get placeholder() {
        return i.account.passwordNew;
      }, onInput: P, get showCopy() {
        return e(p);
      }, get width() {
        return m.inputWidth;
      }, get value() {
        return s().new;
      }, set value(u) {
        s(s().new = u, true);
      }, get reportValidity() {
        return e(f);
      }, set reportValidity(u) {
        o(f, b(u));
      } });
      var J = n(se, 2);
      vt(J, { autocomplete: "new-password", get label() {
        return i.account.passwordConfirm;
      }, get placeholder() {
        return i.account.passwordConfirm;
      }, onInput: P, get showCopy() {
        return e(p);
      }, get width() {
        return m.inputWidth;
      }, get value() {
        return s().newConfirm;
      }, set value(u) {
        s(s().newConfirm = u, true);
      }, get reportValidity() {
        return e(O);
      }, set reportValidity(u) {
        o(O, b(u));
      } });
      var S = n(J, 2), E = a(S);
      ke(E, { onclick: W, level: 2, children: (u, B) => {
        we();
        var ie = be();
        N(() => c(ie, i.account.generateRandom)), v(u, ie);
      }, $$slots: { default: true } }), t(S);
      var g = n(S, 2), A = a(g, true);
      t(g), t(L), N(() => c(A, e(h))), v(U, L);
    };
    k(G, (U) => {
      e(l) && U(ne);
    });
  }
  return v(ve, Y), De({ isPwdValid: P });
}
var ua = I('<div class="m-05 svelte-1h4zvzu"><p> </p> <p><b> </b></p> <div></div> <p> </p> <!></div>'), ca = I("<p> </p> <p> </p> <p> </p> <div><!></div>", 1), fa = I('<div class="success svelte-1h4zvzu"><!></div>'), _a = I('<div class="err"> </div>'), ma = I('<div class="cancel"><!></div>'), pa = I('<div class="convertPasskey svelte-1h4zvzu"><h3> </h3> <p> </p> <!></div>'), ga = I('<div><!> <div class="save svelte-1h4zvzu"><!> <!></div></div> <!>', 1), ha = I('<div class="container svelte-1h4zvzu"><!> <!> <!> <!></div>');
function Et(ve, m) {
  $e(m, true);
  let s = Ie(m, "user", 15), r = Ne(), M = _e(() => m.viewModePhone ? "calc(100vw - 1.5rem)" : "300px"), i = D(b(s().account_type)), z = D(b([])), h = D(void 0), l = D(false), p = D(false), f = D(""), O = D(false), P = D(void 0), W = D(b({ current: "", new: "", newConfirm: "" })), Y = _e(() => e(z).filter((w) => w.user_verified).length > 0);
  tt(() => {
    G();
  });
  async function G() {
    let w = await Be(`/auth/v1/users/${s().id}/webauthn`);
    w.body ? o(z, b(w.body)) : console.error("error fetching passkeys: " + w.error);
  }
  async function ne() {
    let w = await lt(`/auth/v1/users/${s().id}/self/convert_passkey`);
    w.error ? console.error("error fetching passkeys: " + w.error) : window.location.reload();
  }
  async function U() {
    o(f, ""), e(z).length > 0 ? await L() : await j();
  }
  async function L() {
    var _a2;
    if (!((_a2 = e(h)) == null ? void 0 : _a2())) {
      o(f, b(r.common.invalidInput));
      return;
    }
    o(P, "PasswordNew");
  }
  async function j(w) {
    var _a2, _b;
    if (!((_a2 = e(h)) == null ? void 0 : _a2())) {
      o(f, b(r.common.invalidInput));
      return;
    }
    o(p, true);
    let _ = { password_new: e(W).new };
    w ? _.mfa_code = w : _.password_current = e(W).current;
    let d = await ct(`/auth/v1/users/${s().id}/self`, _);
    d.body ? (o(O, true), o(W, b({ current: "", new: "", newConfirm: "" })), s(d.body), o(i, b(d.body.account_type)), setTimeout(() => {
      o(O, false), o(l, false);
    }, 3e3)) : o(f, b(((_b = d.error) == null ? void 0 : _b.message) || "Error")), o(p, false);
  }
  function ee(w) {
    o(P, void 0), o(f, b(w)), setTimeout(() => {
      o(f, "");
    }, 5e3);
  }
  function ae(w) {
    o(P, void 0), w && "code" in w && j(w.code);
  }
  async function se() {
    let w = { email: s().email }, _ = await lt("/auth/v1/users/request_reset", w);
    _.error ? o(f, b(_.error.message)) : o(O, true);
  }
  var J = ha(), S = a(J);
  {
    var E = (w) => {
      Lt(w, { get userId() {
        return s().id;
      }, get purpose() {
        return e(P);
      }, onSuccess: ae, onError: ee });
    };
    k(S, (w) => {
      e(P) && w(E);
    });
  }
  var g = n(S, 2);
  {
    var A = (w) => {
      var _ = ua(), d = a(_), T = a(d, true);
      t(d);
      var R = n(d, 2), te = a(R), y = a(te, true);
      t(te), t(R);
      var $ = n(R, 2);
      Me($, "height", ".3rem");
      var Z = n($, 2), ue = a(Z, true);
      t(Z);
      var ye = n(Z, 2);
      {
        var C = (F) => {
          et(F, {});
        }, x = (F) => {
          ke(F, { level: 2, onclick: se, children: (oe, ce) => {
            we();
            var H = be();
            N(() => c(H, r.account.passwordReset)), v(oe, H);
          }, $$slots: { default: true } });
        };
        k(ye, (F) => {
          e(O) ? F(C) : F(x, false);
        });
      }
      t(_), N(() => {
        var _a2;
        c(T, r.account.federatedConvertPassword1), c(y, ((_a2 = m.authProvider) == null ? void 0 : _a2.name) || "UNKNOWN"), c(ue, r.account.federatedConvertPassword2);
      }), v(w, _);
    };
    k(g, (w) => {
      e(i) === "federated" && w(A);
    });
  }
  var u = n(g, 2);
  {
    var B = (w) => {
      var _ = ca(), d = Q(_), T = a(d, true);
      t(d);
      var R = n(d, 2), te = a(R, true);
      t(R);
      var y = n(R, 2), $ = a(y, true);
      t(y);
      var Z = n(y, 2), ue = a(Z);
      ke(ue, { level: 2, onclick: () => o(l, true), children: (ye, C) => {
        we();
        var x = be();
        N(() => c(x, r.account.convertAccount)), v(ye, x);
      }, $$slots: { default: true } }), t(Z), N(() => {
        c(T, r.account.accTypePasskeyText1), c(te, r.account.accTypePasskeyText2), c($, r.account.accTypePasskeyText3);
      }), v(w, _);
    };
    k(u, (w) => {
      (e(i) === "passkey" || e(i) === "federated_passkey") && !e(l) && w(B);
    });
  }
  var ie = n(u, 2);
  {
    var le = (w) => {
      var _ = ga(), d = Q(_), T = a(d);
      const R = _e(() => !(e(i) === "password" && e(z).length < 1));
      da(T, { get inputWidth() {
        return e(M);
      }, get hideCurrentPassword() {
        return e(R);
      }, get passwords() {
        return e(W);
      }, set passwords(x) {
        o(W, b(x));
      }, get isValid() {
        return e(h);
      }, set isValid(x) {
        o(h, b(x));
      } });
      var te = n(T, 2), y = a(te);
      ke(y, { onclick: U, level: 1, get isLoading() {
        return e(p);
      }, children: (x, F) => {
        we();
        var oe = be();
        N(() => c(oe, r.common.save)), v(x, oe);
      }, $$slots: { default: true } });
      var $ = n(y, 2);
      {
        var Z = (x) => {
          var F = fa(), oe = a(F);
          ft(oe, {}), t(F), Se(3, F, () => We), v(x, F);
        }, ue = (x) => {
          var F = fe(), oe = Q(F);
          {
            var ce = (K) => {
              var me = _a(), ge = a(me, true);
              t(me), N(() => c(ge, e(f))), Se(3, me, () => We), v(K, me);
            }, H = (K) => {
              var me = fe(), ge = Q(me);
              {
                var xe = (Ce) => {
                  var Ue = ma(), He = a(Ue);
                  ke(He, { level: 3, onclick: () => o(l, false), children: (rt, Fe) => {
                    we();
                    var Ae = be();
                    N(() => c(Ae, r.common.cancel)), v(rt, Ae);
                  }, $$slots: { default: true } }), t(Ue), v(Ce, Ue);
                };
                k(ge, (Ce) => {
                  e(l) && !e(p) && Ce(xe);
                }, true);
              }
              v(K, me);
            };
            k(oe, (K) => {
              e(f) ? K(ce) : K(H, false);
            }, true);
          }
          v(x, F);
        };
        k($, (x) => {
          e(O) ? x(Z) : x(ue, false);
        });
      }
      t(te), t(d);
      var ye = n(d, 2);
      {
        var C = (x) => {
          var F = pa(), oe = a(F), ce = a(oe, true);
          t(oe);
          var H = n(oe, 2), K = a(H, true);
          t(H);
          var me = n(H, 2);
          ke(me, { level: 2, onclick: ne, children: (ge, xe) => {
            we();
            var Ce = be();
            N(() => c(Ce, r.account.convertAccount)), v(ge, Ce);
          }, $$slots: { default: true } }), t(F), N(() => {
            c(ce, r.account.convertAccount), c(K, r.account.convertAccountP1);
          }), v(x, F);
        };
        k(ye, (x) => {
          !e(l) && e(Y) && x(C);
        });
      }
      v(w, _);
    };
    k(ie, (w) => {
      (e(i) === "password" || e(i) === "federated_password" || e(l)) && w(le);
    });
  }
  t(J), v(ve, J), De();
}
function wa(ve, m) {
  ve.code === "Enter" && m();
}
var ba = I('<div class="flex"><div class="label font-label noselect svelte-1xf5lr3"><!></div> <label class="switch svelte-1xf5lr3"><input type="checkbox" class="svelte-1xf5lr3"> <span class="slider slider-round svelte-1xf5lr3"></span></label></div>');
function Tt(ve, m) {
  let s = Ie(m, "checked", 15, false), r = Ie(m, "ariaLabel", 3, ""), M = Ie(m, "name", 3, "");
  function i() {
    s(!s());
  }
  var z = ba(), h = a(z), l = a(h);
  Tr(l, () => m.children), t(h);
  var p = n(h, 2), f = a(p);
  cr(f), f.__click = i, f.__keydown = [wa, i], we(2), t(p), t(z), N(() => {
    Me(h, "width", m.labelWidth), Te(f, "name", M()), f.disabled = m.disabled, Te(f, "aria-checked", s()), Te(f, "aria-label", r());
  }), gr(f, s), v(ve, z);
}
er(["click", "keydown"]);
var ya = I("<div><p> </p> <!></div>"), xa = I('<div class="success svelte-5kivuv"><!></div>'), ka = I('<div class="err svelte-5kivuv"> </div>'), Pa = I('<div class="container svelte-5kivuv"><p> </p> <p><!></p> <div class="switch svelte-5kivuv"><!></div> <div class="switch svelte-5kivuv"><!></div> <!> <div class="bottom svelte-5kivuv"><!> <!></div></div>');
function Ct(ve, m) {
  $e(m, true);
  let s = Ie(m, "webIdData", 15), r = Ne();
  const M = "14rem";
  let i = D(""), z = D(false), h = D(!!s().custom_triples), l = it(s().user_id);
  async function p() {
    o(i, "");
    let g = await ct(`/auth/v1/users/${s().user_id}/webid/data`, s());
    g.error ? o(i, b(g.error.message)) : (o(z, true), setTimeout(() => {
      o(z, false);
    }, 3e3));
  }
  var f = Pa(), O = a(f), P = a(O, true);
  t(O);
  var W = n(O, 2), Y = a(W);
  Cr(Y, { href: l, target: "_blank", children: (g, A) => {
    var u = fe(), B = Q(u);
    Dt(B, () => l.replace("/auth/", "/auth/<wbr/>")), v(g, u);
  }, $$slots: { default: true } }), t(W);
  var G = n(W, 2), ne = a(G);
  Tt(ne, { ariaLabel: "E-Mail", labelWidth: M, get checked() {
    return s().expose_email;
  }, set checked(g) {
    s(s().expose_email = g, true);
  }, children: (g, A) => {
    we();
    var u = be("E-Mail");
    v(g, u);
  }, $$slots: { default: true } }), t(G);
  var U = n(G, 2), L = a(U);
  Tt(L, { get ariaLabel() {
    return r.account.webIdExpertMode;
  }, labelWidth: M, get checked() {
    return e(h);
  }, set checked(g) {
    o(h, b(g));
  }, children: (g, A) => {
    we();
    var u = be();
    N(() => c(u, r.account.webIdExpertMode)), v(g, u);
  }, $$slots: { default: true } }), t(U);
  var j = n(U, 2);
  {
    var ee = (g) => {
      var A = ya(), u = a(A), B = a(u, true);
      t(u);
      var ie = n(u, 2);
      $r(ie, { placeholder: "FOAF", rows: 15, get value() {
        return s().custom_triples;
      }, set value(le) {
        s(s().custom_triples = le, true);
      } }), t(A), N(() => c(B, r.account.webIdDescData)), Se(3, A, () => fr), v(g, A);
    };
    k(j, (g) => {
      e(h) && g(ee);
    });
  }
  var ae = n(j, 2), se = a(ae);
  ke(se, { onclick: p, children: (g, A) => {
    we();
    var u = be();
    N(() => c(u, r.common.save)), v(g, u);
  }, $$slots: { default: true } });
  var J = n(se, 2);
  {
    var S = (g) => {
      var A = xa(), u = a(A);
      ft(u, {}), t(A), Se(3, A, () => We), v(g, A);
    }, E = (g) => {
      var A = fe(), u = Q(A);
      {
        var B = (ie) => {
          var le = ka(), w = a(le, true);
          t(le), N(() => c(w, e(i))), Se(3, le, () => We), v(ie, le);
        };
        k(u, (ie) => {
          e(i) && ie(B);
        }, true);
      }
      v(g, A);
    };
    k(J, (g) => {
      e(z) ? g(S) : g(E, false);
    });
  }
  t(ae), t(f), N(() => c(P, r.account.webIdDesc)), v(ve, f), De();
}
function Ia(ve, m) {
  $e(m, true);
  let s = _t("account"), r = _e(() => {
    var _a2;
    return (_a2 = s.get()) == null ? void 0 : _a2.user_id;
  });
  var M = fe(), i = Q(M);
  {
    var z = (h) => {
      Nt(h, { viewMode: "account", get userId() {
        return e(r);
      } });
    };
    k(i, (h) => {
      e(r) && h(z);
    });
  }
  v(ve, M), De();
}
var Ea = I("<h3> </h3>"), Ta = I('<div class="headerPhone svelte-k6hxbm"><!></div> <div class="container svelte-k6hxbm"><!> <div class="innerPhone svelte-k6hxbm"><!></div></div>', 1), Ca = I('<div class="header svelte-k6hxbm"><!></div> <div class="container svelte-k6hxbm"><!> <div class="inner svelte-k6hxbm"><!></div></div>', 1), $a = I('<!> <div class="wrapper svelte-k6hxbm"><!></div>', 1);
function Da(ve, m) {
  $e(m, true);
  const s = (L) => {
    var j = Ea(), ee = a(j, true);
    t(j), N(() => c(ee, `${r().given_name} ${r().family_name || ""}`)), v(L, j);
  };
  let r = Ie(m, "user", 15), M = Ie(m, "webIdData", 15), i = Ne(), z = D(void 0), h = D(b([])), l = _e(() => {
    var _a2;
    if ((_a2 = r().account_type) == null ? void 0 : _a2.startsWith("federated")) return e(h).filter((L) => L.id === r().auth_provider_id)[0];
  }), p = _e(() => e(z) && e(z) < 500), f = D(b(i.account.navInfo)), O = b(M() ? [i.account.navInfo, i.account.navEdit, i.common.password, i.account.navMfa, "WebID", i.account.devices, i.account.navLogout] : [i.account.navInfo, i.account.navEdit, i.common.password, i.account.navMfa, i.account.devices, i.account.navLogout]);
  tt(() => {
    Lr("v").get() === "devices" && o(f, b(i.account.devices));
  }), ze(() => {
    e(f) === i.account.navLogout && lr();
  });
  var P = $a(), W = Q(P);
  Dr(W, { id: dr, get value() {
    return e(h);
  }, set value(L) {
    o(h, b(L));
  } });
  var Y = n(W, 2), G = a(Y);
  {
    var ne = (L) => {
      var j = Ta(), ee = Q(j), ae = a(ee);
      s(ae), t(ee);
      var se = n(ee, 2), J = a(se);
      xt(J, { get tabs() {
        return O;
      }, get selected() {
        return e(f);
      }, set selected(u) {
        o(f, b(u));
      } });
      var S = n(J, 2), E = a(S);
      {
        var g = (u) => {
          kt(u, { get providers() {
            return e(h);
          }, get authProvider() {
            return e(l);
          }, viewModePhone: true, get webIdData() {
            return M();
          }, get user() {
            return r();
          }, set user(B) {
            r(B);
          } });
        }, A = (u) => {
          var B = fe(), ie = Q(B);
          {
            var le = (_) => {
              Pt(_, { viewModePhone: true, get user() {
                return r();
              }, set user(d) {
                r(d);
              } });
            }, w = (_) => {
              var d = fe(), T = Q(d);
              {
                var R = (y) => {
                  Et(y, { get user() {
                    return r();
                  }, get authProvider() {
                    return e(l);
                  }, viewModePhone: true });
                }, te = (y) => {
                  var $ = fe(), Z = Q($);
                  {
                    var ue = (C) => {
                      It(C, { get user() {
                        return r();
                      } });
                    }, ye = (C) => {
                      var x = fe(), F = Q(x);
                      {
                        var oe = (H) => {
                          Ct(H, { get webIdData() {
                            return M();
                          }, set webIdData(K) {
                            M(K);
                          } });
                        }, ce = (H) => {
                          var K = fe(), me = Q(K);
                          {
                            var ge = (xe) => {
                              Nt(xe, { get userId() {
                                return r().id;
                              } });
                            };
                            k(me, (xe) => {
                              e(f) === i.account.devices && xe(ge);
                            }, true);
                          }
                          v(H, K);
                        };
                        k(F, (H) => {
                          e(f) === "WebID" ? H(oe) : H(ce, false);
                        }, true);
                      }
                      v(C, x);
                    };
                    k(Z, (C) => {
                      e(f) === i.account.navMfa ? C(ue) : C(ye, false);
                    }, true);
                  }
                  v(y, $);
                };
                k(T, (y) => {
                  e(f) === i.common.password ? y(R) : y(te, false);
                }, true);
              }
              v(_, d);
            };
            k(ie, (_) => {
              e(f) === i.account.navEdit ? _(le) : _(w, false);
            }, true);
          }
          v(u, B);
        };
        k(E, (u) => {
          e(f) === i.account.navInfo ? u(g) : u(A, false);
        });
      }
      t(S), t(se), v(L, j);
    }, U = (L) => {
      var j = Ca(), ee = Q(j), ae = a(ee);
      s(ae), t(ee);
      var se = n(ee, 2), J = a(se);
      xt(J, { get tabs() {
        return O;
      }, center: true, get selected() {
        return e(f);
      }, set selected(u) {
        o(f, b(u));
      } });
      var S = n(J, 2), E = a(S);
      {
        var g = (u) => {
          kt(u, { get webIdData() {
            return M();
          }, get providers() {
            return e(h);
          }, get authProvider() {
            return e(l);
          }, get user() {
            return r();
          }, set user(B) {
            r(B);
          } });
        }, A = (u) => {
          var B = fe(), ie = Q(B);
          {
            var le = (_) => {
              Pt(_, { get user() {
                return r();
              }, set user(d) {
                r(d);
              } });
            }, w = (_) => {
              var d = fe(), T = Q(d);
              {
                var R = (y) => {
                  Et(y, { get user() {
                    return r();
                  }, get authProvider() {
                    return e(l);
                  } });
                }, te = (y) => {
                  var $ = fe(), Z = Q($);
                  {
                    var ue = (C) => {
                      It(C, { get user() {
                        return r();
                      } });
                    }, ye = (C) => {
                      var x = fe(), F = Q(x);
                      {
                        var oe = (H) => {
                          Ct(H, { get webIdData() {
                            return M();
                          }, set webIdData(K) {
                            M(K);
                          } });
                        }, ce = (H) => {
                          var K = fe(), me = Q(K);
                          {
                            var ge = (xe) => {
                              Ia(xe, {});
                            };
                            k(me, (xe) => {
                              e(f) === i.account.devices && xe(ge);
                            }, true);
                          }
                          v(H, K);
                        };
                        k(F, (H) => {
                          e(f) === "WebID" ? H(oe) : H(ce, false);
                        }, true);
                      }
                      v(C, x);
                    };
                    k(Z, (C) => {
                      e(f) === i.account.navMfa ? C(ue) : C(ye, false);
                    }, true);
                  }
                  v(y, $);
                };
                k(T, (y) => {
                  e(f) === i.common.password ? y(R) : y(te, false);
                }, true);
              }
              v(_, d);
            };
            k(ie, (_) => {
              e(f) === i.account.navEdit ? _(le) : _(w, false);
            }, true);
          }
          v(u, B);
        };
        k(E, (u) => {
          e(f) === i.account.navInfo ? u(g) : u(A, false);
        });
      }
      t(S), t(se), v(L, j);
    };
    k(G, (L) => {
      e(p) ? L(ne) : L(U, false);
    });
  }
  t(Y), rr("innerWidth", (L) => o(z, b(L))), v(ve, P), De();
}
var La = I("<!> <!> <!>", 1);
function ws(ve, m) {
  $e(m, true);
  let s = Ne(), r = _t("account"), M = D(void 0), i = D(void 0), z = D(false);
  ze(() => {
    let l = r.get();
    l && h(l);
  });
  async function h(l) {
    const p = l.user_id;
    if (p) {
      let f = await Promise.all([Be(`/auth/v1/users/${p}`), Be(`/auth/v1/users/${p}/webid/data`)]);
      f[0].body ? o(M, b(f[0].body)) : ur("account"), f[1].body ? o(i, b(f[1].body)) : f[1].status === 404 && o(i, b({ user_id: p, expose_email: false })), o(z, true);
    } else console.error("no user_id in session");
  }
  tr((l) => {
    N(() => {
      var _a2;
      return Qt.title = `${(s == null ? void 0 : s.account.account) || "Account"} ${((_a2 = e(M)) == null ? void 0 : _a2.email) ?? ""}`;
    });
  }), Nr(ve, { children: (l, p) => {
    Ar(l, { children: (f, O) => {
      var P = La(), W = Q(P);
      {
        var Y = (U) => {
          Da(U, { get user() {
            return e(M);
          }, set user(L) {
            o(M, b(L));
          }, get webIdData() {
            return e(i);
          }, set webIdData(L) {
            o(i, b(L));
          } });
        };
        k(W, (U) => {
          e(z) && r && e(M) && U(Y);
        });
      }
      var G = n(W, 2);
      Mr(G, { absolute: true });
      var ne = n(G, 2);
      zr(ne, { absolute: true }), v(f, P);
    } });
  } }), De();
}
export {
  ws as component
};
