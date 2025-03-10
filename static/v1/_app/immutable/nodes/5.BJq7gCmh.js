import { t as P, a as i, d as _e, e as ye } from "../chunks/BH6NCLk-.js";
import { j as e, k as C, l as o, p as De, aa as Ae, a9 as ce, c as r, r as t, a as ze, f as X, s as n, t as $, a7 as be, a5 as tt, a6 as Zt } from "../chunks/CvlvO1XB.js";
import { e as nt, s as c, b as er, d as tr, h as rr } from "../chunks/CTI4QPiR.js";
import { i as k } from "../chunks/BUO_AUgz.js";
import { p as b } from "../chunks/Wh68IIk2.js";
import { b as ar } from "../chunks/7lxiEjMQ.js";
import { p as Ie } from "../chunks/C6SR4G2t.js";
import { i as sr, g as or, C as nr, l as Ze, Q as lt, J as vr, P as ir, K as lr, z as dr, S as ur, D as cr, r as fr } from "../chunks/B21bTIl7.js";
import { e as Dt } from "../chunks/BpWRzPRQ.js";
import { h as zt } from "../chunks/i8Xqpu09.js";
import { s as Ce, a as K, c as J, r as _r } from "../chunks/BMbqVy6X.js";
import { a as $e, B as ke, t as Se, f as We, s as mr } from "../chunks/DMkkW5Nn.js";
import { C as et } from "../chunks/CS_Msctd.js";
import { M as pr } from "../chunks/C712LKcc.js";
import { u as Ne } from "../chunks/CUqQZdNU.js";
import { f as Ue, c as ct, b as dt, d as ft } from "../chunks/tF66aiNY.js";
import { B as gr } from "../chunks/DQDimnsq.js";
import { a as hr, c as wr } from "../chunks/dU6E9WaN.js";
import { I as br } from "../chunks/BWnKfvFA.js";
import { I as yr } from "../chunks/Vi3uK7uO.js";
import { I as Me } from "../chunks/CsPRTKI3.js";
import { F as xr } from "../chunks/C0egthaI.js";
import { b as ut, k as kr, l as xt, m as Pr } from "../chunks/BRCxk8by.js";
import { I as _t } from "../chunks/Nks81rMs.js";
import { I as Ir } from "../chunks/BXc4I0Oc.js";
import { u as mt } from "../chunks/cwCl71UF.js";
import { w as Tr } from "../chunks/D8WJ_d7a.js";
import { W as Mt } from "../chunks/s_Ni8j5e.js";
import { U as Cr, D as Nt } from "../chunks/Dnrb_T0P.js";
import { P as Er } from "../chunks/DIB4eys6.js";
import { I as vt } from "../chunks/B-xX0s4n.js";
import { s as $r } from "../chunks/-T201g_q.js";
import { A as Dr } from "../chunks/BqWmWZH3.js";
import { I as zr } from "../chunks/Cxg1YoLR.js";
import { T as Mr } from "../chunks/BHb9vWaQ.js";
import { u as Nr } from "../chunks/B_RTeDUK.js";
import { T as kt } from "../chunks/_OE2Cq0B.js";
import { M as Lr } from "../chunks/BbRujPvb.js";
import { C as Ar } from "../chunks/GWgockW8.js";
import { T as Sr } from "../chunks/Cbuxv9a6.js";
import { L as Rr } from "../chunks/Pcv4BnHP.js";
let it = C(void 0);
function Wr() {
  return !e(it) && sr() && Ue("/auth/v1/users/picture_config").then((ne) => {
    o(it, b(ne.body));
  }), { get() {
    return e(it);
  } };
}
var Ur = P("<span> </span>"), Fr = P('<img loading="lazy" class="absolute svelte-1390lfo" aria-label="Avatar" alt="" width="100%">'), Vr = P('<div class="errLimit svelte-1390lfo"> </div>'), qr = P("<div><!></div>"), Br = P('<div class="relative"><div class="delete svelte-1390lfo"><!></div></div>'), Kr = P('<form class="avatar svelte-1390lfo" aria-dropeffect="move" aria-label="Upload"><label class="svelte-1390lfo"><!></label> <input type="file" aria-hidden="true" class="svelte-1390lfo"> <!> <!></form> <!>', 1), Or = P('<span class="avatar svelte-1390lfo"><!></span>'), Hr = P('<div class="container svelte-1390lfo"><!></div>');
function jr(ne, _) {
  De(_, true);
  const s = (v) => {
    var h = _e(), T = X(h);
    {
      var S = (p) => {
        var f = Ur(), G = r(f, true);
        t(f), $((Q) => {
          K(f, 1, `absolute font-mono font-${z()} noselect`, "svelte-1390lfo"), c(G, Q);
        }, [() => _.fallbackCharacters.toUpperCase()]), i(p, f);
      }, m = (p) => {
        var f = Fr();
        $(() => Ce(f, "src", `/auth/v1/users/${_.userId}/picture/${a()}`)), nt("error", f, D), er(f), i(p, f);
      };
      k(T, (p) => {
        e(g) || !a() ? p(S) : p(m, false);
      });
    }
    i(v, h);
  };
  let a = Ie(_, "pictureId", 15), z = Ie(_, "size", 3, "medium");
  const l = or(), M = "image/png, image/jpeg, image/webp";
  let y = Ne(), u = ce(() => Wr().get()), g = C(""), d = C(""), j, E = C(false), W = C(false), A = C(void 0), Z = ce(U), ee = ce(() => {
    switch (z()) {
      case "small":
        return "1.5rem";
      case "medium":
        return "3rem";
      default:
        return "192px";
    }
  });
  Ae(() => {
    e(g) && console.error(e(g));
  }), Ae(() => {
    e(d) && (j && clearTimeout(j), j = setTimeout(() => {
      o(d, "");
    }, 5e3));
  }), Ae(() => {
    e(A) && e(A).length > 0 && te(e(A));
  });
  function U() {
    return `hsl(${ve(_.fallbackCharacters)}, 50%, 50%)`;
  }
  function D(v) {
    v.type === "error" && o(g, "Input Error");
  }
  function Y() {
    o(E, true);
  }
  function ae() {
    o(E, false);
  }
  function ve(v) {
    let h = [1, 255, 3, 13, 19];
    for (let T = 0; T < 5; T++) {
      let S = v.charCodeAt(T);
      if (S) h[T] = S;
      else break;
    }
    return (h[0] * (64 + h[1]) + 4 * h[0] + (h[2] * h[3] - h[4])) % 360;
  }
  async function ie() {
    if (o(g, ""), o(d, ""), !a()) return;
    let v = await ct(`/auth/v1/users/${_.userId}/picture/${a()}`);
    v.error ? o(g, b(v.error.message)) : a(void 0);
  }
  async function te(v) {
    if (o(g, ""), o(d, ""), !e(u)) return;
    o(W, true);
    let h = `/auth/v1/users/${_.userId}/picture`;
    for (let T of v) {
      if (!M.includes(T.type)) {
        o(g, "Invalid File Format, allowed: " + M);
        break;
      }
      if (T.size > e(u).content_len_limit) {
        o(d, `${y.common.maxFileSize}: ${e(u).content_len_limit / 1024 / 1024} MB`);
        break;
      }
      let S = new FormData();
      S.append(T.name, T);
      let m = await fetch(h, { method: "PUT", headers: { "csrf-token": localStorage.getItem(nr) || "" }, body: S });
      if (m.ok) a(await m.text());
      else {
        let p = await m.json();
        m.status === 406 ? alert("max size" + p.message) : o(g, b(p.message || "Upload Error"));
      }
    }
    o(A, void 0), o(W, false);
  }
  var F = Hr(), L = r(F);
  {
    var w = (v) => {
      var h = Kr(), T = X(h), S = r(T);
      Ce(S, "for", l), Ce(S, "aria-controls", l);
      var m = r(S);
      br(m, { get width() {
        return e(ee);
      } }), t(S);
      var p = n(S, 2);
      Ce(p, "id", l), Ce(p, "accept", M);
      var f = n(p, 2);
      {
        var G = (q) => {
          var re = Vr(), ue = r(re, true);
          t(re), $(() => {
            $e(re, "width", e(ee)), c(ue, e(d));
          }), i(q, re);
        };
        k(f, (q) => {
          e(d) && q(G);
        });
      }
      var Q = n(f, 2);
      s(Q), t(T);
      var le = n(T, 2);
      {
        var R = (q) => {
          var re = Br(), ue = r(re), ge = r(ue);
          ke(ge, { invisible: true, onclick: ie, children: (I, x) => {
            var N = qr(), se = r(N);
            yr(se, {}), t(N), $(() => Ce(N, "title", y.common.delete)), i(I, N);
          }, $$slots: { default: true } }), t(ue), t(re), i(q, re);
        };
        k(le, (q) => {
          z() === "large" && a() && q(R);
        });
      }
      $(() => {
        $e(T, "background-color", e(Z)), $e(T, "width", e(ee)), Ce(S, "aria-disabled", e(W)), Ce(S, "data-show", !e(W) && e(E)), $e(S, "width", e(ee)), p.disabled = e(W), Ce(p, "aria-disabled", e(W));
      }), nt("mouseenter", T, Y), nt("mouseleave", T, ae), hr(p, () => e(A), (q) => o(A, q)), i(v, h);
    }, V = (v) => {
      var h = Or(), T = r(h);
      s(T), t(h), $(() => {
        $e(h, "background-color", e(Z)), $e(h, "width", e(ee)), $e(h, "height", e(ee));
      }), i(v, h);
    };
    k(L, (v) => {
      var _a2;
      ((_a2 = e(u)) == null ? void 0 : _a2.upload_allowed) ? v(w) : v(V, false);
    });
  }
  t(F), i(ne, F), ze();
}
var Yr = P('<div class="link-err value svelte-1tea8v4"> </div>'), Gr = P('<div class="fed-btn svelte-1tea8v4"><!> <!></div>'), Jr = P('<h3> </h3> <p> </p> <div class="providers svelte-1tea8v4"></div>', 1), Qr = P("<!> <!>", 1), Xr = P('<div><div> </div> <span class="value svelte-1tea8v4"> </span></div>'), Zr = P('<div><div> </div> <span class="value svelte-1tea8v4"> </span></div>'), ea = P('<div><div>WebID:</div> <span class="value svelte-1tea8v4"><a target="_blank"><!></a></span></div>'), ta = P('<div><div><div></div> <!></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <div><div class="value svelte-1tea8v4"> </div> <!></div></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <!></div> <div><div> </div> <!></div> <div><div> </div> <!></div> <!> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <div><div> </div> <span class="value svelte-1tea8v4"> </span></div> <!> <!></div>');
function Pt(ne, _) {
  De(_, true);
  let s = Ie(_, "user", 15), a = Ne(), z = C(false), l = C(false), M = ce(() => {
    var _a2;
    return (_a2 = s().account_type) == null ? void 0 : _a2.startsWith("federated");
  }), y = ce(() => {
    var _a2;
    return e(M) ? `${s().account_type}: ${((_a2 = _.authProvider) == null ? void 0 : _a2.name) || ""}` : s().account_type;
  }), u = ce(() => _.viewModePhone ? "rowPhone" : "row"), g = ce(() => _.viewModePhone ? "labelPhone" : "label"), d = ce(() => {
    let B = s().given_name[0];
    return s().family_name && s().family_name.length > 0 && (B += s().family_name[0]), B;
  });
  function j(B) {
    vr(64, (oe, { challenge: de, verifier: we }) => {
      oe || (localStorage.setItem(ir, we), E(B, de));
    });
  }
  async function E(B, oe) {
    let de = { email: s().email, client_id: "rauthy", redirect_uri: window.location.href, provider_id: B, pkce_challenge: oe }, we = await dt(`/auth/v1/providers/${B}/link`, de);
    if (we.text) {
      lr(we.text);
      let pe = we.headers.get("location");
      pe && (window.location.href = pe);
    } else console.error(we.error);
  }
  async function W() {
    let B = await ct("/auth/v1/providers/link");
    B.body ? s(B.body) : (console.error(B.error), o(z, true));
  }
  var A = ta(), Z = r(A), ee = r(Z), U = n(ee, 2);
  jr(U, { get fallbackCharacters() {
    return e(d);
  }, get userId() {
    return s().id;
  }, size: "large", get pictureId() {
    return s().picture_id;
  }, set pictureId(B) {
    s(s().picture_id = B, true);
  } }), t(Z);
  var D = n(Z, 2), Y = r(D), ae = r(Y, true);
  t(Y);
  var ve = n(Y, 2), ie = r(ve, true);
  t(ve), t(D);
  var te = n(D, 2), F = r(te), L = r(F, true);
  t(F);
  var w = n(F, 2), V = r(w, true);
  t(w), t(te);
  var v = n(te, 2), h = r(v), T = r(h, true);
  t(h);
  var S = n(h, 2), m = r(S, true);
  t(S), t(v);
  var p = n(v, 2), f = r(p), G = r(f);
  t(f);
  var Q = n(f, 2), le = r(Q, true);
  t(Q), t(p);
  var R = n(p, 2), q = r(R), re = r(q, true);
  t(q);
  var ue = n(q, 2), ge = r(ue), I = r(ge, true);
  t(ge);
  var x = n(ge, 2);
  {
    var N = (B) => {
      var oe = Gr(), de = r(oe);
      ke(de, { get ariaLabel() {
        return a.account.providerUnlink;
      }, level: 3, onclick: W, children: (Pe, Te) => {
        be();
        var Re = ye();
        $(() => c(Re, a.account.providerUnlink)), i(Pe, Re);
      }, $$slots: { default: true } });
      var we = n(de, 2);
      {
        var pe = (Pe) => {
          var Te = Yr(), Re = r(Te, true);
          t(Te), $(() => c(Re, a.account.providerUnlinkDesc)), i(Pe, Te);
        };
        k(we, (Pe) => {
          e(z) && Pe(pe);
        });
      }
      t(oe), i(B, oe);
    }, se = (B) => {
      var oe = _e(), de = X(oe);
      {
        var we = (pe) => {
          var Pe = Qr(), Te = X(Pe);
          ke(Te, { level: 2, onclick: () => o(l, true), children: (qe, Gt) => {
            be();
            var Be = ye();
            $(() => c(Be, a.account.providerLink)), i(qe, Be);
          }, $$slots: { default: true } });
          var Re = n(Te, 2);
          pr(Re, { get showModal() {
            return e(l);
          }, set showModal(qe) {
            o(l, b(qe));
          }, children: (qe, Gt) => {
            var Be = Jr(), at = X(Be), Jt = r(at, true);
            t(at);
            var st = n(at, 2), Qt = r(st, true);
            t(st);
            var bt = n(st, 2);
            Dt(bt, 21, () => _.providers, (ot) => ot.id, (ot, yt) => {
              const Xt = ce(() => `${a.account.providerLink}: ${e(yt).name}`);
              gr(ot, { get ariaLabel() {
                return e(Xt);
              }, get provider() {
                return e(yt);
              }, onclick: j });
            }), t(bt), $(() => {
              c(Jt, a.account.providerLink), c(Qt, a.account.providerLinkDesc);
            }), i(qe, Be);
          }, $$slots: { default: true } }), i(pe, Pe);
        };
        k(de, (pe) => {
          _.providers.length > 0 && pe(we);
        }, true);
      }
      i(B, oe);
    };
    k(x, (B) => {
      e(M) ? B(N) : B(se, false);
    });
  }
  t(ue), t(R);
  var fe = n(R, 2), O = r(fe), H = r(O, true);
  t(O);
  var me = n(O, 2), he = r(me, true);
  t(me), t(fe);
  var xe = n(fe, 2), Ee = r(xe), Fe = r(Ee, true);
  t(Ee);
  var Ke = n(Ee, 2), rt = r(Ke, true);
  t(Ke), t(xe);
  var Ve = n(xe, 2), Le = r(Ve), Lt = r(Le, true);
  t(Le);
  var At = n(Le, 2);
  const St = ce(() => !!s().webauthn_user_id);
  et(At, { get checked() {
    return e(St);
  } }), t(Ve);
  var Oe = n(Ve, 2), He = r(Oe), Rt = r(He, true);
  t(He);
  var Wt = n(He, 2);
  et(Wt, { get checked() {
    return s().enabled;
  } }), t(Oe);
  var je = n(Oe, 2), Ye = r(je), Ut = r(Ye, true);
  t(Ye);
  var Ft = n(Ye, 2);
  et(Ft, { get checked() {
    return s().email_verified;
  } }), t(je);
  var pt = n(je, 2);
  {
    var Vt = (B) => {
      var oe = Xr(), de = r(oe), we = r(de, true);
      t(de);
      var pe = n(de, 2), Pe = r(pe, true);
      t(pe), t(oe), $((Te) => {
        K(oe, 1, J(e(u)), "svelte-1tea8v4"), K(de, 1, J(e(g)), "svelte-1tea8v4"), c(we, a.account.lastLogin), c(Pe, Te);
      }, [() => Ze(s().last_login)]), i(B, oe);
    };
    k(pt, (B) => {
      s().last_login && B(Vt);
    });
  }
  var Ge = n(pt, 2), Je = r(Ge), qt = r(Je, true);
  t(Je);
  var gt = n(Je, 2), Bt = r(gt, true);
  t(gt), t(Ge);
  var Qe = n(Ge, 2), Xe = r(Qe), Kt = r(Xe, true);
  t(Xe);
  var ht = n(Xe, 2), Ot = r(ht, true);
  t(ht), t(Qe);
  var wt = n(Qe, 2);
  {
    var Ht = (B) => {
      var oe = Zr(), de = r(oe), we = r(de, true);
      t(de);
      var pe = n(de, 2), Pe = r(pe, true);
      t(pe), t(oe), $((Te) => {
        K(oe, 1, J(e(u)), "svelte-1tea8v4"), K(de, 1, J(e(g)), "svelte-1tea8v4"), c(we, a.account.userExpiry), c(Pe, Te);
      }, [() => Ze(s().user_expires)]), i(B, oe);
    };
    k(wt, (B) => {
      s().user_expires && B(Ht);
    });
  }
  var jt = n(wt, 2);
  {
    var Yt = (B) => {
      var oe = ea(), de = r(oe), we = n(de, 2), pe = r(we), Pe = r(pe);
      zt(Pe, () => lt(s().id).replace("/auth/", "/auth/<wbr/>")), t(pe), t(we), t(oe), $((Te) => {
        K(oe, 1, J(e(u)), "svelte-1tea8v4"), K(de, 1, J(e(g)), "svelte-1tea8v4"), Ce(pe, "href", Te);
      }, [() => lt(s().id)]), i(B, oe);
    };
    k(jt, (B) => {
      _.webIdData && B(Yt);
    });
  }
  t(A), $((B, oe) => {
    K(Z, 1, J(e(u)), "svelte-1tea8v4"), $e(Z, "margin", ".5rem 0"), K(ee, 1, J(e(g)), "svelte-1tea8v4"), K(D, 1, J(e(u)), "svelte-1tea8v4"), K(Y, 1, J(e(g)), "svelte-1tea8v4"), c(ae, a.common.email), c(ie, s().email), K(te, 1, J(e(u)), "svelte-1tea8v4"), K(F, 1, J(e(g)), "svelte-1tea8v4"), c(L, a.account.givenName), c(V, s().given_name), K(v, 1, J(e(u)), "svelte-1tea8v4"), K(h, 1, J(e(g)), "svelte-1tea8v4"), c(T, a.account.familyName), c(m, s().family_name), K(p, 1, J(e(u)), "svelte-1tea8v4"), K(f, 1, J(e(g)), "svelte-1tea8v4"), c(G, `${a.account.user ?? ""} ID`), c(le, s().id), K(R, 1, J(e(u)), "svelte-1tea8v4"), K(q, 1, J(e(g)), "svelte-1tea8v4"), c(re, a.account.accType), c(I, e(y) || ""), K(fe, 1, J(e(u)), "svelte-1tea8v4"), K(O, 1, J(e(g)), "svelte-1tea8v4"), c(H, a.account.roles), c(he, s().roles || "None"), K(xe, 1, J(e(u)), "svelte-1tea8v4"), K(Ee, 1, J(e(g)), "svelte-1tea8v4"), c(Fe, a.account.groups), c(rt, s().groups || "None"), K(Ve, 1, J(e(u)), "svelte-1tea8v4"), K(Le, 1, J(e(g)), "svelte-1tea8v4"), c(Lt, a.account.mfaActivated), K(Oe, 1, J(e(u)), "svelte-1tea8v4"), K(He, 1, J(e(g)), "svelte-1tea8v4"), c(Rt, a.account.userEnabled), K(je, 1, J(e(u)), "svelte-1tea8v4"), K(Ye, 1, J(e(g)), "svelte-1tea8v4"), c(Ut, a.account.emailVerified), K(Ge, 1, J(e(u)), "svelte-1tea8v4"), K(Je, 1, J(e(g)), "svelte-1tea8v4"), c(qt, a.account.passwordExpiry), c(Bt, B), K(Qe, 1, J(e(u)), "svelte-1tea8v4"), K(Xe, 1, J(e(g)), "svelte-1tea8v4"), c(Kt, a.account.userCreated), c(Ot, oe);
  }, [() => s().password_expires && Ze(s().password_expires) || a.common.never, () => Ze(s().created_at)]), i(ne, A), ze();
}
var ra = P('<div class="success svelte-4ogr3z"><!></div>'), aa = P("<p> </p>"), sa = P('<div class="err svelte-4ogr3z"> </div>'), oa = P('<div class="formInner svelte-4ogr3z"><div><!> <!> <!> <!></div> <div><!> <!> <!> <!> <!></div></div> <div class="bottom svelte-4ogr3z"><div><!></div> <!></div> <!>', 1), na = P('<div class="container svelte-4ogr3z"><!></div>');
function It(ne, _) {
  var _a2;
  De(_, true);
  let s = Ie(_, "user", 15);
  ((_a2 = s().user_values) == null ? void 0 : _a2.birthdate) || s(s().user_values.birthdate = "", true);
  let a = Ne(), z = C(""), l = C(false), M = C(false);
  async function y(j, E) {
    var _a3;
    const W = E.get("email"), A = E.get("family_name") || void 0, Z = E.get("given_name") || void 0, ee = E.get("birthdate") || void 0, U = ((_a3 = E.get("phone")) == null ? void 0 : _a3.replaceAll(" ", "")) || void 0, D = E.get("street") || void 0, Y = E.get("zip") || void 0, ae = Y ? Number.parseInt(Y) : void 0, ve = E.get("city") || void 0, ie = E.get("country") || void 0;
    let te = { email: W, family_name: A, given_name: Z };
    (ee || U || D || ae || ve || ie) && (te.user_values = { birthdate: ee, phone: U, street: D, zip: ae, city: ve, country: ie });
    let F = await ft(`/auth/v1/users/${s().id}/self`, te);
    if (F.body) {
      o(l, true), s(F.body);
      let L = 3e3;
      F.status === 202 && (o(M, true), L = 3e4), setTimeout(() => {
        o(l, false);
      }, L);
    } else F.error && (console.error(F.error), o(z, b(F.error.message)));
  }
  var u = na(), g = r(u);
  const d = ce(() => `/auth/v1/users/${s().id}/self`);
  xr(g, { get action() {
    return e(d);
  }, onSubmit: y, children: (j, E) => {
    var W = oa(), A = X(W), Z = r(A), ee = r(Z);
    Me(ee, { typ: "email", name: "email", get label() {
      return a.common.email;
    }, get placeholder() {
      return a.common.email;
    }, get value() {
      return s().email;
    }, required: true });
    var U = n(ee, 2);
    Me(U, { name: "given_name", autocomplete: "given-name", get label() {
      return a.account.givenName;
    }, get placeholder() {
      return a.account.givenName;
    }, get value() {
      return s().given_name;
    }, required: true, maxLength: 32, pattern: ut });
    var D = n(U, 2);
    Me(D, { name: "family_name", autocomplete: "family-name", get label() {
      return a.account.familyName;
    }, get placeholder() {
      return a.account.familyName;
    }, get value() {
      return s().family_name;
    }, maxLength: 32, pattern: ut });
    var Y = n(D, 2);
    Ir(Y, { name: "birthdate", get label() {
      return a.account.birthdate;
    }, withDelete: true, get value() {
      return s().user_values.birthdate;
    }, set value(f) {
      s(s().user_values.birthdate = f, true);
    } }), t(Z);
    var ae = n(Z, 2), ve = r(ae);
    Me(ve, { name: "street", autocomplete: "street-address", get label() {
      return a.account.street;
    }, get placeholder() {
      return a.account.street;
    }, get value() {
      return s().user_values.street;
    }, maxLength: 48, pattern: kr });
    var ie = n(ve, 2);
    Me(ie, { typ: "number", name: "zip", autocomplete: "postal-code", get label() {
      return a.account.zip;
    }, get placeholder() {
      return a.account.zip;
    }, get value() {
      return s().user_values.zip;
    }, min: "1000", max: "9999999" });
    var te = n(ie, 2);
    Me(te, { name: "city", autocomplete: "address-level2", get label() {
      return a.account.city;
    }, get placeholder() {
      return a.account.city;
    }, get value() {
      return s().user_values.city;
    }, maxLength: 48, pattern: xt });
    var F = n(te, 2);
    Me(F, { name: "country", autocomplete: "country", get label() {
      return a.account.country;
    }, get placeholder() {
      return a.account.country;
    }, get value() {
      return s().user_values.country;
    }, maxLength: 48, pattern: xt });
    var L = n(F, 2);
    Me(L, { name: "phone", autocomplete: "tel", get label() {
      return a.account.phone;
    }, get placeholder() {
      return a.account.phone;
    }, get value() {
      return s().user_values.phone;
    }, maxLength: 32, pattern: Pr }), t(ae), t(A);
    var w = n(A, 2), V = r(w), v = r(V);
    ke(v, { type: "submit", children: (f, G) => {
      be();
      var Q = ye();
      $(() => c(Q, a.common.save)), i(f, Q);
    }, $$slots: { default: true } }), t(V);
    var h = n(V, 2);
    {
      var T = (f) => {
        var G = ra(), Q = r(G);
        _t(Q, {}), t(G), Se(3, G, () => We), i(f, G);
      };
      k(h, (f) => {
        e(l) && f(T);
      });
    }
    t(w);
    var S = n(w, 2);
    {
      var m = (f) => {
        var G = aa(), Q = r(G, true);
        t(G), $(() => c(Q, a.account.emailUpdateConfirm)), i(f, G);
      }, p = (f) => {
        var G = _e(), Q = X(G);
        {
          var le = (R) => {
            var q = sa(), re = r(q, true);
            t(q), $(() => c(re, e(z))), Se(3, q, () => We), i(R, q);
          };
          k(Q, (R) => {
            e(z) && R(le);
          }, true);
        }
        i(f, G);
      };
      k(S, (f) => {
        e(M) ? f(m) : f(p, false);
      });
    }
    i(j, W);
  }, $$slots: { default: true } }), t(u), i(ne, u), ze();
}
var va = P('<div class="err svelte-16mlupu"><b>Your browser does not support Webauthn credentials and must be updated.</b></div>'), ia = P('<!> <div class="regBtns svelte-16mlupu"><!> <!></div>', 1), la = P('<div class="regNewBtn svelte-16mlupu"><!></div>'), da = P('<div class="keysHeader svelte-16mlupu"> </div>'), ua = P('<div class="button svelte-16mlupu"><!></div>'), ca = P('<!> <p class="svelte-16mlupu"> <br><br> </p> <!> <!> <div class="keysContainer svelte-16mlupu"></div> <!> <div> </div>', 1), fa = P('<div class="container svelte-16mlupu"><!></div>');
function Tt(ne, _) {
  De(_, true);
  const s = "credentials" in navigator;
  let a = Ne(), z = mt("account"), l = ce(() => {
    var _a2;
    return (_a2 = z.get()) == null ? void 0 : _a2.user_id;
  }), M = C(void 0), y = C(false), u = C(""), g = C(false), d = C(_.user.account_type === "password"), j = C(void 0), E = C(""), W = C(false), A = C(b([]));
  tt(() => {
    ee();
  }), Ae(() => {
    e(A).length > 0 && _.user.account_type === "passkey" && o(d, e(A).length > 1);
  }), Ae(() => {
    var _a2;
    (_a2 = e(M)) == null ? void 0 : _a2.focus();
  });
  function Z() {
    o(y, false), o(u, "");
  }
  async function ee() {
    var _a2;
    let L = await Ue(`/auth/v1/users/${(_a2 = z.get()) == null ? void 0 : _a2.user_id}/webauthn`);
    L.body ? o(A, b(L.body)) : console.error(L.error);
  }
  async function U() {
    if (Z(), e(W) || !e(l)) return;
    if (e(E).length < 1) {
      o(y, true), o(u, b(a.mfa.passkeyNameErr));
      return;
    }
    let L = await Tr(e(l), e(E), a.authorize.invalidKeyUsed, a.authorize.requestExpired);
    L.error ? (o(y, true), o(u, `${a.mfa.errorReg} - ${L.error}`)) : (o(g, false), o(E, ""), await ee());
  }
  async function D(L) {
    var _a2;
    let w = await ct(`/auth/v1/users/${_.user.id}/webauthn/delete/${L}`);
    w.status === 200 ? await ee() : o(u, b(((_a2 = w.error) == null ? void 0 : _a2.message) || "Error"));
  }
  function Y(L) {
    o(j, void 0), o(y, true), o(u, b(L)), setTimeout(() => {
      o(y, false), o(u, "");
    }, 5e3);
  }
  function ae(L) {
    o(j, void 0), o(u, b(a.mfa.testSuccess)), setTimeout(() => {
      o(u, "");
    }, 3e3);
  }
  var ve = fa(), ie = r(ve);
  {
    var te = (L) => {
      var w = va();
      i(L, w);
    }, F = (L) => {
      var w = ca(), V = X(w);
      {
        var v = (I) => {
          Mt(I, { get userId() {
            return _.user.id;
          }, get purpose() {
            return e(j);
          }, onSuccess: ae, onError: Y });
        };
        k(V, (I) => {
          e(j) && I(v);
        });
      }
      var h = n(V, 2), T = r(h), S = n(T, 3);
      t(h);
      var m = n(h, 2);
      {
        var p = (I) => {
          var x = ia(), N = X(x);
          Me(N, { autocomplete: "off", get label() {
            return a.mfa.passkeyName;
          }, get placeholder() {
            return a.mfa.passkeyName;
          }, maxLength: 32, pattern: ut, onEnter: U, get ref() {
            return e(M);
          }, set ref(H) {
            o(M, b(H));
          }, get value() {
            return e(E);
          }, set value(H) {
            o(E, b(H));
          }, get isError() {
            return e(W);
          }, set isError(H) {
            o(W, b(H));
          } });
          var se = n(N, 2), fe = r(se);
          ke(fe, { onclick: U, children: (H, me) => {
            be();
            var he = ye();
            $(() => c(he, a.mfa.register)), i(H, he);
          }, $$slots: { default: true } });
          var O = n(fe, 2);
          ke(O, { level: 3, onclick: () => o(g, false), children: (H, me) => {
            be();
            var he = ye();
            $(() => c(he, a.common.cancel)), i(H, he);
          }, $$slots: { default: true } }), t(se), i(I, x);
        }, f = (I) => {
          var x = la(), N = r(x);
          const se = ce(() => e(A).length === 0 ? 1 : 2);
          ke(N, { get level() {
            return e(se);
          }, onclick: () => o(g, true), children: (fe, O) => {
            be();
            var H = ye();
            $(() => c(H, a.mfa.registerNew)), i(fe, H);
          }, $$slots: { default: true } }), t(x), i(I, x);
        };
        k(m, (I) => {
          e(g) ? I(p) : I(f, false);
        });
      }
      var G = n(m, 2);
      {
        var Q = (I) => {
          var x = da(), N = r(x, true);
          t(x), $(() => c(N, a.mfa.registerdKeys)), i(I, x);
        };
        k(G, (I) => {
          e(A).length > 0 && I(Q);
        });
      }
      var le = n(G, 2);
      Dt(le, 21, () => e(A), (I) => I.name, (I, x) => {
        Cr(I, { get passkey() {
          return e(x);
        }, get showDelete() {
          return e(d);
        }, onDelete: D });
      }), t(le);
      var R = n(le, 2);
      {
        var q = (I) => {
          var x = ua(), N = r(x);
          ke(N, { onclick: () => o(j, "Test"), children: (se, fe) => {
            be();
            var O = ye();
            $(() => c(O, a.mfa.test)), i(se, O);
          }, $$slots: { default: true } }), t(x), i(I, x);
        };
        k(R, (I) => {
          e(A).length > 0 && I(q);
        });
      }
      var re = n(R, 2);
      let ue;
      var ge = r(re, true);
      t(re), $(() => {
        c(T, `${a.mfa.p1 ?? ""} `), c(S, ` ${a.mfa.p2 ?? ""}`), ue = K(re, 1, "svelte-16mlupu", null, ue, { success: !e(y), err: e(y) }), c(ge, e(u));
      }), i(L, w);
    };
    k(ie, (L) => {
      s ? L(F, false) : L(te);
    });
  }
  t(ve), i(ne, ve), ze();
}
var _a = P('<div class="container svelte-ue7bk2"><!> <!> <!> <!> <div class="btn svelte-ue7bk2"><!></div> <div class="err svelte-ue7bk2"> </div></div>');
function ma(ne, _) {
  De(_, true);
  let s = Ie(_, "passwords", 15), a = Ie(_, "hideCurrentPassword", 3, false), z = Ie(_, "isValid", 15), l = Ne();
  z(E);
  let M = C(false), y = C(""), u = C(void 0), g = ce(() => {
    var _a2;
    return ((_a2 = s().new) == null ? void 0 : _a2.length) > 6 && s().new === s().newConfirm;
  }), d = C(void 0), j = C(void 0);
  tt(async () => {
    let U = await Ue("/auth/v1/password_policy");
    U.body ? o(u, b(U.body)) : console.error(U.error);
  });
  function E() {
    return o(y, ""), !a() && !s().current ? (o(y, b(l.account.passwordCurrReq)), false) : s().new ? s().newConfirm ? e(M) ? s().new.length > 256 ? (o(y, "max 256"), false) : s().new !== s().newConfirm ? (o(y, b(l.account.passwordNoMatch)), false) : true : (o(y, b(l.account.passwordPolicyFollow)), false) : (o(y, b(l.account.passwordNewReq)), false) : (o(y, b(l.account.passwordNewReq)), false);
  }
  function W() {
    if (o(y, ""), e(u)) {
      let U = dr(e(u));
      s(s().new = U, true), s(s().newConfirm = U, true);
    }
    requestAnimationFrame(() => {
      var _a2, _b;
      (_a2 = e(d)) == null ? void 0 : _a2(), (_b = e(j)) == null ? void 0 : _b();
    });
  }
  var A = _e(), Z = X(A);
  {
    var ee = (U) => {
      var D = _a(), Y = r(D);
      Er(Y, { get policy() {
        return e(u);
      }, get password() {
        return s().new;
      }, get accepted() {
        return e(M);
      }, set accepted(v) {
        o(M, b(v));
      } });
      var ae = n(Y, 2);
      {
        var ve = (v) => {
          vt(v, { autocomplete: "current-password", get label() {
            return l.account.passwordCurr;
          }, get placeholder() {
            return l.account.passwordCurr;
          }, onInput: E, get width() {
            return _.inputWidth;
          }, get value() {
            return s().current;
          }, set value(h) {
            s(s().current = h, true);
          } });
        };
        k(ae, (v) => {
          a() || v(ve);
        });
      }
      var ie = n(ae, 2);
      vt(ie, { autocomplete: "new-password", get label() {
        return l.account.passwordNew;
      }, get placeholder() {
        return l.account.passwordNew;
      }, onInput: E, get showCopy() {
        return e(g);
      }, get width() {
        return _.inputWidth;
      }, get value() {
        return s().new;
      }, set value(v) {
        s(s().new = v, true);
      }, get reportValidity() {
        return e(d);
      }, set reportValidity(v) {
        o(d, b(v));
      } });
      var te = n(ie, 2);
      vt(te, { autocomplete: "new-password", get label() {
        return l.account.passwordConfirm;
      }, get placeholder() {
        return l.account.passwordConfirm;
      }, onInput: E, get showCopy() {
        return e(g);
      }, get width() {
        return _.inputWidth;
      }, get value() {
        return s().newConfirm;
      }, set value(v) {
        s(s().newConfirm = v, true);
      }, get reportValidity() {
        return e(j);
      }, set reportValidity(v) {
        o(j, b(v));
      } });
      var F = n(te, 2), L = r(F);
      ke(L, { onclick: W, level: 2, children: (v, h) => {
        be();
        var T = ye();
        $(() => c(T, l.account.generateRandom)), i(v, T);
      }, $$slots: { default: true } }), t(F);
      var w = n(F, 2), V = r(w, true);
      t(w), t(D), $(() => c(V, e(y))), i(U, D);
    };
    k(Z, (U) => {
      e(u) && U(ee);
    });
  }
  return i(ne, A), ze({ isPwdValid: E });
}
var pa = P('<div class="m-05 svelte-1h4zvzu"><p> </p> <p><b> </b></p> <div></div> <p> </p> <!></div>'), ga = P("<p> </p> <p> </p> <p> </p> <div><!></div>", 1), ha = P('<div class="success svelte-1h4zvzu"><!></div>'), wa = P('<div class="err"> </div>'), ba = P('<div class="cancel"><!></div>'), ya = P('<div class="convertPasskey svelte-1h4zvzu"><h3> </h3> <p> </p> <!></div>'), xa = P('<div><!> <div class="save svelte-1h4zvzu"><!> <!></div></div> <!>', 1), ka = P('<div class="container svelte-1h4zvzu"><!> <!> <!> <!></div>');
function Ct(ne, _) {
  De(_, true);
  let s = Ie(_, "user", 15), a = Ne(), z = ce(() => _.viewModePhone ? "calc(100vw - 1.5rem)" : "300px"), l = C(b(s().account_type)), M = C(b([])), y = C(void 0), u = C(false), g = C(false), d = C(""), j = C(false), E = C(void 0), W = C(b({ current: "", new: "", newConfirm: "" })), A = ce(() => e(M).filter((m) => m.user_verified).length > 0);
  tt(() => {
    Z();
  });
  async function Z() {
    let m = await Ue(`/auth/v1/users/${s().id}/webauthn`);
    m.body ? o(M, b(m.body)) : console.error("error fetching passkeys: " + m.error);
  }
  async function ee() {
    let m = await dt(`/auth/v1/users/${s().id}/self/convert_passkey`);
    m.error ? console.error("error fetching passkeys: " + m.error) : window.location.reload();
  }
  async function U() {
    o(d, ""), e(M).length > 0 ? await D() : await Y();
  }
  async function D() {
    var _a2;
    if (!((_a2 = e(y)) == null ? void 0 : _a2())) {
      o(d, b(a.common.invalidInput));
      return;
    }
    o(E, "PasswordNew");
  }
  async function Y(m) {
    var _a2, _b;
    if (!((_a2 = e(y)) == null ? void 0 : _a2())) {
      o(d, b(a.common.invalidInput));
      return;
    }
    o(g, true);
    let p = { password_new: e(W).new };
    m ? p.mfa_code = m : p.password_current = e(W).current;
    let f = await ft(`/auth/v1/users/${s().id}/self`, p);
    f.body ? (o(j, true), o(W, b({ current: "", new: "", newConfirm: "" })), s(f.body), o(l, b(f.body.account_type)), setTimeout(() => {
      o(j, false), o(u, false);
    }, 3e3)) : o(d, b(((_b = f.error) == null ? void 0 : _b.message) || "Error")), o(g, false);
  }
  function ae(m) {
    o(E, void 0), o(d, b(m)), setTimeout(() => {
      o(d, "");
    }, 5e3);
  }
  function ve(m) {
    o(E, void 0), m && "code" in m && Y(m.code);
  }
  async function ie() {
    let m = { email: s().email }, p = await dt("/auth/v1/users/request_reset", m);
    p.error ? o(d, b(p.error.message)) : o(j, true);
  }
  var te = ka(), F = r(te);
  {
    var L = (m) => {
      Mt(m, { get userId() {
        return s().id;
      }, get purpose() {
        return e(E);
      }, onSuccess: ve, onError: ae });
    };
    k(F, (m) => {
      e(E) && m(L);
    });
  }
  var w = n(F, 2);
  {
    var V = (m) => {
      var p = pa(), f = r(p), G = r(f, true);
      t(f);
      var Q = n(f, 2), le = r(Q), R = r(le, true);
      t(le), t(Q);
      var q = n(Q, 2);
      $e(q, "height", ".3rem");
      var re = n(q, 2), ue = r(re, true);
      t(re);
      var ge = n(re, 2);
      {
        var I = (N) => {
          et(N, {});
        }, x = (N) => {
          ke(N, { level: 2, onclick: ie, children: (se, fe) => {
            be();
            var O = ye();
            $(() => c(O, a.account.passwordReset)), i(se, O);
          }, $$slots: { default: true } });
        };
        k(ge, (N) => {
          e(j) ? N(I) : N(x, false);
        });
      }
      t(p), $(() => {
        var _a2;
        c(G, a.account.federatedConvertPassword1), c(R, ((_a2 = _.authProvider) == null ? void 0 : _a2.name) || "UNKNOWN"), c(ue, a.account.federatedConvertPassword2);
      }), i(m, p);
    };
    k(w, (m) => {
      e(l) === "federated" && m(V);
    });
  }
  var v = n(w, 2);
  {
    var h = (m) => {
      var p = ga(), f = X(p), G = r(f, true);
      t(f);
      var Q = n(f, 2), le = r(Q, true);
      t(Q);
      var R = n(Q, 2), q = r(R, true);
      t(R);
      var re = n(R, 2), ue = r(re);
      ke(ue, { level: 2, onclick: () => o(u, true), children: (ge, I) => {
        be();
        var x = ye();
        $(() => c(x, a.account.convertAccount)), i(ge, x);
      }, $$slots: { default: true } }), t(re), $(() => {
        c(G, a.account.accTypePasskeyText1), c(le, a.account.accTypePasskeyText2), c(q, a.account.accTypePasskeyText3);
      }), i(m, p);
    };
    k(v, (m) => {
      (e(l) === "passkey" || e(l) === "federated_passkey") && !e(u) && m(h);
    });
  }
  var T = n(v, 2);
  {
    var S = (m) => {
      var p = xa(), f = X(p), G = r(f);
      const Q = ce(() => !(e(l) === "password" && e(M).length < 1));
      ma(G, { get inputWidth() {
        return e(z);
      }, get hideCurrentPassword() {
        return e(Q);
      }, get passwords() {
        return e(W);
      }, set passwords(x) {
        o(W, b(x));
      }, get isValid() {
        return e(y);
      }, set isValid(x) {
        o(y, b(x));
      } });
      var le = n(G, 2), R = r(le);
      ke(R, { onclick: U, level: 1, get isLoading() {
        return e(g);
      }, children: (x, N) => {
        be();
        var se = ye();
        $(() => c(se, a.common.save)), i(x, se);
      }, $$slots: { default: true } });
      var q = n(R, 2);
      {
        var re = (x) => {
          var N = ha(), se = r(N);
          _t(se, {}), t(N), Se(3, N, () => We), i(x, N);
        }, ue = (x) => {
          var N = _e(), se = X(N);
          {
            var fe = (H) => {
              var me = wa(), he = r(me, true);
              t(me), $(() => c(he, e(d))), Se(3, me, () => We), i(H, me);
            }, O = (H) => {
              var me = _e(), he = X(me);
              {
                var xe = (Ee) => {
                  var Fe = ba(), Ke = r(Fe);
                  ke(Ke, { level: 3, onclick: () => o(u, false), children: (rt, Ve) => {
                    be();
                    var Le = ye();
                    $(() => c(Le, a.common.cancel)), i(rt, Le);
                  }, $$slots: { default: true } }), t(Fe), i(Ee, Fe);
                };
                k(he, (Ee) => {
                  e(u) && !e(g) && Ee(xe);
                }, true);
              }
              i(H, me);
            };
            k(se, (H) => {
              e(d) ? H(fe) : H(O, false);
            }, true);
          }
          i(x, N);
        };
        k(q, (x) => {
          e(j) ? x(re) : x(ue, false);
        });
      }
      t(le), t(f);
      var ge = n(f, 2);
      {
        var I = (x) => {
          var N = ya(), se = r(N), fe = r(se, true);
          t(se);
          var O = n(se, 2), H = r(O, true);
          t(O);
          var me = n(O, 2);
          ke(me, { level: 2, onclick: ee, children: (he, xe) => {
            be();
            var Ee = ye();
            $(() => c(Ee, a.account.convertAccount)), i(he, Ee);
          }, $$slots: { default: true } }), t(N), $(() => {
            c(fe, a.account.convertAccount), c(H, a.account.convertAccountP1);
          }), i(x, N);
        };
        k(ge, (x) => {
          !e(u) && e(A) && x(I);
        });
      }
      i(m, p);
    };
    k(T, (m) => {
      (e(l) === "password" || e(l) === "federated_password" || e(u)) && m(S);
    });
  }
  t(te), i(ne, te), ze();
}
function Pa(ne, _) {
  ne.code === "Enter" && _();
}
var Ia = P('<div class="flex"><div class="label font-label noselect svelte-1xf5lr3"><!></div> <label class="switch svelte-1xf5lr3"><input type="checkbox" class="svelte-1xf5lr3"> <span class="slider slider-round svelte-1xf5lr3"></span></label></div>');
function Et(ne, _) {
  let s = Ie(_, "checked", 15, false), a = Ie(_, "ariaLabel", 3, ""), z = Ie(_, "name", 3, "");
  function l() {
    s(!s());
  }
  var M = Ia(), y = r(M), u = r(y);
  $r(u, () => _.children), t(y);
  var g = n(y, 2), d = r(g);
  _r(d), d.__click = l, d.__keydown = [Pa, l], be(2), t(g), t(M), $(() => {
    $e(y, "width", _.labelWidth), Ce(d, "name", z()), d.disabled = _.disabled, Ce(d, "aria-checked", s()), Ce(d, "aria-label", a());
  }), wr(d, s), i(ne, M);
}
tr(["click", "keydown"]);
var Ta = P("<div><p> </p> <!></div>"), Ca = P('<div class="success svelte-5kivuv"><!></div>'), Ea = P('<div class="err svelte-5kivuv"> </div>'), $a = P('<div class="container svelte-5kivuv"><p> </p> <p><!></p> <div class="switch svelte-5kivuv"><!></div> <div class="switch svelte-5kivuv"><!></div> <!> <div class="bottom svelte-5kivuv"><!> <!></div></div>');
function $t(ne, _) {
  De(_, true);
  let s = Ie(_, "webIdData", 15), a = Ne();
  const z = "14rem";
  let l = C(""), M = C(false), y = C(!!s().custom_triples), u = lt(s().user_id);
  async function g() {
    o(l, "");
    let w = await ft(`/auth/v1/users/${s().user_id}/webid/data`, s());
    w.error ? o(l, b(w.error.message)) : (o(M, true), setTimeout(() => {
      o(M, false);
    }, 3e3));
  }
  var d = $a(), j = r(d), E = r(j, true);
  t(j);
  var W = n(j, 2), A = r(W);
  Dr(A, { href: u, target: "_blank", children: (w, V) => {
    var v = _e(), h = X(v);
    zt(h, () => u.replace("/auth/", "/auth/<wbr/>")), i(w, v);
  }, $$slots: { default: true } }), t(W);
  var Z = n(W, 2), ee = r(Z);
  Et(ee, { ariaLabel: "E-Mail", labelWidth: z, get checked() {
    return s().expose_email;
  }, set checked(w) {
    s(s().expose_email = w, true);
  }, children: (w, V) => {
    be();
    var v = ye("E-Mail");
    i(w, v);
  }, $$slots: { default: true } }), t(Z);
  var U = n(Z, 2), D = r(U);
  Et(D, { get ariaLabel() {
    return a.account.webIdExpertMode;
  }, labelWidth: z, get checked() {
    return e(y);
  }, set checked(w) {
    o(y, b(w));
  }, children: (w, V) => {
    be();
    var v = ye();
    $(() => c(v, a.account.webIdExpertMode)), i(w, v);
  }, $$slots: { default: true } }), t(U);
  var Y = n(U, 2);
  {
    var ae = (w) => {
      var V = Ta(), v = r(V), h = r(v, true);
      t(v);
      var T = n(v, 2);
      zr(T, { placeholder: "FOAF", rows: 15, get value() {
        return s().custom_triples;
      }, set value(S) {
        s(s().custom_triples = S, true);
      } }), t(V), $(() => c(h, a.account.webIdDescData)), Se(3, V, () => mr), i(w, V);
    };
    k(Y, (w) => {
      e(y) && w(ae);
    });
  }
  var ve = n(Y, 2), ie = r(ve);
  ke(ie, { onclick: g, children: (w, V) => {
    be();
    var v = ye();
    $(() => c(v, a.common.save)), i(w, v);
  }, $$slots: { default: true } });
  var te = n(ie, 2);
  {
    var F = (w) => {
      var V = Ca(), v = r(V);
      _t(v, {}), t(V), Se(3, V, () => We), i(w, V);
    }, L = (w) => {
      var V = _e(), v = X(V);
      {
        var h = (T) => {
          var S = Ea(), m = r(S, true);
          t(S), $(() => c(m, e(l))), Se(3, S, () => We), i(T, S);
        };
        k(v, (T) => {
          e(l) && T(h);
        }, true);
      }
      i(w, V);
    };
    k(te, (w) => {
      e(M) ? w(F) : w(L, false);
    });
  }
  t(ve), t(d), $(() => c(E, a.account.webIdDesc)), i(ne, d), ze();
}
function Da(ne, _) {
  De(_, true);
  let s = mt("account"), a = ce(() => {
    var _a2;
    return (_a2 = s.get()) == null ? void 0 : _a2.user_id;
  });
  var z = _e(), l = X(z);
  {
    var M = (y) => {
      Nt(y, { viewMode: "account", get userId() {
        return e(a);
      } });
    };
    k(l, (y) => {
      e(a) && y(M);
    });
  }
  i(ne, z), ze();
}
var za = P("<h3> </h3>"), Ma = P('<div class="headerPhone svelte-k6hxbm"><!></div> <div class="container svelte-k6hxbm"><!> <div class="innerPhone svelte-k6hxbm"><!></div></div>', 1), Na = P('<div class="header svelte-k6hxbm"><!></div> <div class="container svelte-k6hxbm"><!> <div class="inner svelte-k6hxbm"><!></div></div>', 1), La = P('<!> <div class="wrapper svelte-k6hxbm"><!></div>', 1);
function Aa(ne, _) {
  De(_, true);
  const s = (D) => {
    var Y = za(), ae = r(Y, true);
    t(Y), $(() => c(ae, `${a().given_name} ${a().family_name || ""}`)), i(D, Y);
  };
  let a = Ie(_, "user", 15), z = Ie(_, "webIdData", 15), l = Ne(), M = C(void 0), y = C(b([])), u = ce(() => {
    var _a2;
    if ((_a2 = a().account_type) == null ? void 0 : _a2.startsWith("federated")) return e(y).filter((D) => D.id === a().auth_provider_id)[0];
  }), g = ce(() => e(M) && e(M) < 500), d = C(b(l.account.navInfo)), j = b(z() ? [l.account.navInfo, l.account.navEdit, l.common.password, l.account.navMfa, "WebID", l.account.devices, l.account.navLogout] : [l.account.navInfo, l.account.navEdit, l.common.password, l.account.navMfa, l.account.devices, l.account.navLogout]);
  tt(() => {
    Nr("v").get() === "devices" && o(d, b(l.account.devices));
  }), Ae(() => {
    e(d) === l.account.navLogout && ur();
  });
  var E = La(), W = X(E);
  Mr(W, { id: cr, get value() {
    return e(y);
  }, set value(D) {
    o(y, b(D));
  } });
  var A = n(W, 2), Z = r(A);
  {
    var ee = (D) => {
      var Y = Ma(), ae = X(Y), ve = r(ae);
      s(ve), t(ae);
      var ie = n(ae, 2), te = r(ie);
      kt(te, { get tabs() {
        return j;
      }, get selected() {
        return e(d);
      }, set selected(v) {
        o(d, b(v));
      } });
      var F = n(te, 2), L = r(F);
      {
        var w = (v) => {
          Pt(v, { get providers() {
            return e(y);
          }, get authProvider() {
            return e(u);
          }, viewModePhone: true, get webIdData() {
            return z();
          }, get user() {
            return a();
          }, set user(h) {
            a(h);
          } });
        }, V = (v) => {
          var h = _e(), T = X(h);
          {
            var S = (p) => {
              It(p, { viewModePhone: true, get user() {
                return a();
              }, set user(f) {
                a(f);
              } });
            }, m = (p) => {
              var f = _e(), G = X(f);
              {
                var Q = (R) => {
                  Ct(R, { get user() {
                    return a();
                  }, get authProvider() {
                    return e(u);
                  }, viewModePhone: true });
                }, le = (R) => {
                  var q = _e(), re = X(q);
                  {
                    var ue = (I) => {
                      Tt(I, { get user() {
                        return a();
                      } });
                    }, ge = (I) => {
                      var x = _e(), N = X(x);
                      {
                        var se = (O) => {
                          $t(O, { get webIdData() {
                            return z();
                          }, set webIdData(H) {
                            z(H);
                          } });
                        }, fe = (O) => {
                          var H = _e(), me = X(H);
                          {
                            var he = (xe) => {
                              Nt(xe, { get userId() {
                                return a().id;
                              } });
                            };
                            k(me, (xe) => {
                              e(d) === l.account.devices && xe(he);
                            }, true);
                          }
                          i(O, H);
                        };
                        k(N, (O) => {
                          e(d) === "WebID" ? O(se) : O(fe, false);
                        }, true);
                      }
                      i(I, x);
                    };
                    k(re, (I) => {
                      e(d) === l.account.navMfa ? I(ue) : I(ge, false);
                    }, true);
                  }
                  i(R, q);
                };
                k(G, (R) => {
                  e(d) === l.common.password ? R(Q) : R(le, false);
                }, true);
              }
              i(p, f);
            };
            k(T, (p) => {
              e(d) === l.account.navEdit ? p(S) : p(m, false);
            }, true);
          }
          i(v, h);
        };
        k(L, (v) => {
          e(d) === l.account.navInfo ? v(w) : v(V, false);
        });
      }
      t(F), t(ie), i(D, Y);
    }, U = (D) => {
      var Y = Na(), ae = X(Y), ve = r(ae);
      s(ve), t(ae);
      var ie = n(ae, 2), te = r(ie);
      kt(te, { get tabs() {
        return j;
      }, center: true, get selected() {
        return e(d);
      }, set selected(v) {
        o(d, b(v));
      } });
      var F = n(te, 2), L = r(F);
      {
        var w = (v) => {
          Pt(v, { get webIdData() {
            return z();
          }, get providers() {
            return e(y);
          }, get authProvider() {
            return e(u);
          }, get user() {
            return a();
          }, set user(h) {
            a(h);
          } });
        }, V = (v) => {
          var h = _e(), T = X(h);
          {
            var S = (p) => {
              It(p, { get user() {
                return a();
              }, set user(f) {
                a(f);
              } });
            }, m = (p) => {
              var f = _e(), G = X(f);
              {
                var Q = (R) => {
                  Ct(R, { get user() {
                    return a();
                  }, get authProvider() {
                    return e(u);
                  } });
                }, le = (R) => {
                  var q = _e(), re = X(q);
                  {
                    var ue = (I) => {
                      Tt(I, { get user() {
                        return a();
                      } });
                    }, ge = (I) => {
                      var x = _e(), N = X(x);
                      {
                        var se = (O) => {
                          $t(O, { get webIdData() {
                            return z();
                          }, set webIdData(H) {
                            z(H);
                          } });
                        }, fe = (O) => {
                          var H = _e(), me = X(H);
                          {
                            var he = (xe) => {
                              Da(xe, {});
                            };
                            k(me, (xe) => {
                              e(d) === l.account.devices && xe(he);
                            }, true);
                          }
                          i(O, H);
                        };
                        k(N, (O) => {
                          e(d) === "WebID" ? O(se) : O(fe, false);
                        }, true);
                      }
                      i(I, x);
                    };
                    k(re, (I) => {
                      e(d) === l.account.navMfa ? I(ue) : I(ge, false);
                    }, true);
                  }
                  i(R, q);
                };
                k(G, (R) => {
                  e(d) === l.common.password ? R(Q) : R(le, false);
                }, true);
              }
              i(p, f);
            };
            k(T, (p) => {
              e(d) === l.account.navEdit ? p(S) : p(m, false);
            }, true);
          }
          i(v, h);
        };
        k(L, (v) => {
          e(d) === l.account.navInfo ? v(w) : v(V, false);
        });
      }
      t(F), t(ie), i(D, Y);
    };
    k(Z, (D) => {
      e(g) ? D(ee) : D(U, false);
    });
  }
  t(A), ar("innerWidth", (D) => o(M, b(D))), i(ne, E), ze();
}
var Sa = P("<!> <!> <!>", 1);
function Ps(ne, _) {
  De(_, true);
  let s = Ne(), a = mt("account"), z = C(void 0), l = C(void 0), M = C(false);
  Ae(() => {
    let u = a.get();
    u && y(u);
  });
  async function y(u) {
    const g = u.user_id;
    if (g) {
      let d = await Promise.all([Ue(`/auth/v1/users/${g}`), Ue(`/auth/v1/users/${g}/webid/data`)]);
      d[0].body ? o(z, b(d[0].body)) : fr("account"), d[1].body ? o(l, b(d[1].body)) : d[1].status === 404 && o(l, b({ user_id: g, expose_email: false })), o(M, true);
    } else console.error("no user_id in session");
  }
  rr((u) => {
    $(() => {
      var _a2;
      return Zt.title = `${(s == null ? void 0 : s.account.account) || "Account"} ${((_a2 = e(z)) == null ? void 0 : _a2.email) ?? ""}`;
    });
  }), Lr(ne, { children: (u, g) => {
    Ar(u, { children: (d, j) => {
      var E = Sa(), W = X(E);
      {
        var A = (U) => {
          Aa(U, { get user() {
            return e(z);
          }, set user(D) {
            o(z, b(D));
          }, get webIdData() {
            return e(l);
          }, set webIdData(D) {
            o(l, b(D));
          } });
        };
        k(W, (U) => {
          e(M) && a && e(z) && U(A);
        });
      }
      var Z = n(W, 2);
      Sr(Z, { absolute: true });
      var ee = n(Z, 2);
      Rr(ee, { absolute: true }), i(d, E);
    } });
  } }), ze();
}
export {
  Ps as component
};
