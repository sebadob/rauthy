var __defProp = Object.defineProperty;
var __typeError = (msg) => {
  throw TypeError(msg);
};
var __defNormalProp = (obj, key, value) => key in obj ? __defProp(obj, key, { enumerable: true, configurable: true, writable: true, value }) : obj[key] = value;
var __publicField = (obj, key, value) => __defNormalProp(obj, typeof key !== "symbol" ? key + "" : key, value);
var __accessCheck = (obj, member, msg) => member.has(obj) || __typeError("Cannot " + msg);
var __privateGet = (obj, member, getter) => (__accessCheck(obj, member, "read from private field"), getter ? getter.call(obj) : member.get(obj));
var __privateAdd = (obj, member, value) => member.has(obj) ? __typeError("Cannot add the same private member more than once") : member instanceof WeakSet ? member.add(obj) : member.set(obj, value);
var __privateSet = (obj, member, value, setter) => (__accessCheck(obj, member, "write to private field"), setter ? setter.call(obj, value) : member.set(obj, value), value);
var _e, _a2;
import { t as Pe, a as ee, e as Be, d as Jt } from "./BxmJRzoY.js";
import { p as Lt, j as Z, a4 as rt, a1 as Ve, F as Uo, a5 as kt, a as Ot, f as De, l as F, k as w, s as J, t as he, c as le, a3 as We, r as oe, a0 as xl, a6 as nv } from "./w0HvPX0p.js";
import { s as pe } from "./BzP2S3Z_.js";
import { i as be } from "./iO9_dPNE.js";
import { e as Go, i as xh } from "./S81raU5Y.js";
import { a as Zt } from "./BdbQ6g_y.js";
import { b as _h, d as _l, f as jt, c as Cl } from "./BO1A6s0c.js";
import { O as ov } from "./85nGGue2.js";
import { P as Dl } from "./l-KvjgJG.js";
import { N as lv } from "./CzQKNE2_.js";
import { B as uv } from "./DoeALvoe.js";
import { u as cv } from "./Di69SM1b.js";
import { N as dv } from "./D9abP6hj.js";
import { C as pv } from "./C6jTHtu1.js";
import { B as Et, t as Ch, s as Ph } from "./C8YTstTD.js";
import { I as mt } from "./Q4PIg3iI.js";
import { b as Wo, k as fv, l as Bl, m as hv } from "./gfDO7tLr.js";
import { F as Pl } from "./CDe-qvZi.js";
import { u as ar } from "./0cG6LBdy.js";
import { u as $t } from "./DHOKTGcE.js";
import { O as qh } from "./DTYRN0IK.js";
import { I as Fo } from "./ScYc5fRW.js";
import { I as bl } from "./cZptBaPP.js";
import { f as Qt, a as jo, b as Th } from "./DswDW5U8.js";
import { S as zo } from "./CJX07lT1.js";
import { L as Ht } from "./DOFJTuej.js";
import { u as Sh } from "./BhhTa2kE.js";
import { T as mv } from "./BdAKL3gn.js";
import { p as vv } from "./C6GSeq7M.js";
import { l as ul, z as bv } from "./B21bTIl7.js";
import { C as yv } from "./DwsgkunL.js";
import { I as Zo } from "./CTshzOVc.js";
import { U as Ah, a as gv, D as Rv } from "./BbSs9vzp.js";
import { P as xv } from "./DbyBiYcN.js";
import { I as Fl } from "./iedauS3r.js";
import { h as _v } from "./C2ZdIFW_.js";
import { P as Cv, f as Pv, a as qv } from "./CycaKChw.js";
var Tv = Pe("<div><!></div>"), Sv = Pe('<div class="err"> </div>'), Av = Pe("<!> <!> <!> <!> <!> <!> <!> <!> <div></div> <!> <!>", 1), wv = Pe('<div class="container svelte-x7a3kj"><!></div>');
function Ev(e, t) {
  Lt(t, true);
  let a = ar(), i = $t(), l = Z(void 0), d = Z(""), n = Z(""), P = Z(""), x = Z(""), T = rt(() => {
    var _a3;
    return (_a3 = Sh().common()) == null ? void 0 : _a3.map((m) => m);
  }), b = Z("en"), k = Z(Ve(Uo(() => t.roles.map((m) => ({ name: m.name, selected: false }))))), I = Z(Ve(Uo(() => t.groups.map((m) => ({ name: m.name, selected: false }))))), E = Z(false), $ = Z(Ve(Qt())), D = Z(Ve(jo()));
  kt(() => {
    requestAnimationFrame(() => {
      var _a3;
      (_a3 = w(l)) == null ? void 0 : _a3.focus();
    });
  });
  async function S(m, _) {
    var _a3, _b2;
    F(d, "");
    let R = { email: w(n), given_name: w(P), family_name: w(x) || void 0, language: w(b), groups: w(I).filter((A) => A.selected).map((A) => A.name), roles: w(k).filter((A) => A.selected).map((A) => A.name), user_expires: w(E) ? Th(w($), w(D)) : void 0 };
    ((_a3 = R.groups) == null ? void 0 : _a3.length) === 0 && (R.groups = void 0);
    let q = await _h(m.action, R);
    q.body ? t.onSave(q.body.id) : F(d, ((_b2 = q.error) == null ? void 0 : _b2.message) || "Error", true);
  }
  var h = wv(), v = le(h);
  Pl(v, { action: "/auth/v1/users", onSubmit: S, children: (m, _) => {
    var R = Av(), q = De(R);
    mt(q, { typ: "email", autocomplete: "off", label: "E-Mail", placeholder: "E-Mail", required: true, get ref() {
      return w(l);
    }, set ref(N) {
      F(l, N, true);
    }, get value() {
      return w(n);
    }, set value(N) {
      F(n, N, true);
    } });
    var A = J(q, 2);
    mt(A, { autocomplete: "off", get label() {
      return a.account.givenName;
    }, get placeholder() {
      return a.account.givenName;
    }, required: true, pattern: Wo, get value() {
      return w(P);
    }, set value(N) {
      F(P, N, true);
    } });
    var L = J(A, 2);
    mt(L, { autocomplete: "off", get label() {
      return a.account.familyName;
    }, get placeholder() {
      return a.account.familyName;
    }, pattern: Wo, get value() {
      return w(x);
    }, set value(N) {
      F(x, N, true);
    } });
    var r = J(L, 2);
    {
      var s = (N) => {
        Ht(N, { get label() {
          return i.common.language;
        }, children: (V, U) => {
          qh(V, { get ariaLabel() {
            return a.common.selectI18n;
          }, get options() {
            return w(T);
          }, borderless: true, get value() {
            return w(b);
          }, set value(K) {
            F(b, K, true);
          } });
        }, $$slots: { default: true } });
      };
      be(r, (N) => {
        w(T) && N(s);
      });
    }
    var o = J(r, 2);
    zo(o, { get items() {
      return w(k);
    }, set items(N) {
      F(k, N, true);
    }, children: (N, V) => {
      We();
      var U = Be();
      he(() => pe(U, a.account.roles)), ee(N, U);
    }, $$slots: { default: true } });
    var u = J(o, 2);
    zo(u, { get items() {
      return w(I);
    }, set items(N) {
      F(I, N, true);
    }, children: (N, V) => {
      We();
      var U = Be();
      he(() => pe(U, a.account.groups)), ee(N, U);
    }, $$slots: { default: true } });
    var p = J(u, 2);
    Fo(p, { get ariaLabel() {
      return a.account.accessExp;
    }, get checked() {
      return w(E);
    }, set checked(N) {
      F(E, N, true);
    }, children: (N, V) => {
      We();
      var U = Be();
      he(() => pe(U, a.account.accessExp)), ee(N, U);
    }, $$slots: { default: true } });
    var f = J(p, 2);
    {
      var g = (N) => {
        var V = Tv(), U = le(V);
        const K = rt(Qt);
        bl(U, { get label() {
          return a.account.accessExp;
        }, withTime: true, get min() {
          return w(K);
        }, required: true, get value() {
          return w($);
        }, set value(X) {
          F($, X, true);
        }, get timeValue() {
          return w(D);
        }, set timeValue(X) {
          F(D, X, true);
        } }), oe(V), Ch(3, V, () => Ph, () => ({ duration: 150 })), ee(N, V);
      };
      be(f, (N) => {
        w(E) && N(g);
      });
    }
    var M = J(f, 2);
    Zt(M, "", {}, { height: ".66rem" });
    var C = J(M, 2);
    Et(C, { type: "submit", children: (N, V) => {
      We();
      var U = Be();
      he(() => pe(U, a.common.save)), ee(N, U);
    }, $$slots: { default: true } });
    var y = J(C, 2);
    {
      var O = (N) => {
        var V = Sv(), U = le(V, true);
        oe(V), he(() => pe(U, w(d))), ee(N, V);
      };
      be(y, (N) => {
        w(d) && N(O);
      });
    }
    ee(m, R);
  }, $$slots: { default: true } }), oe(h), ee(e, h), Ot();
}
var kv = Pe("<div><!></div>"), Iv = Pe('<div class="svelte-126e0v0"><!></div>'), Mv = Pe('<div class="err"> </div>'), Lv = Pe('<!> <div class="values svelte-126e0v0"><div class="svelte-126e0v0"><!></div> <div class="svelte-126e0v0"><!></div></div> <div class="values svelte-126e0v0"><div class="svelte-126e0v0"><!> <!> <!> <!> <!></div> <div class="svelte-126e0v0"><!> <!> <!> <!> <!></div></div> <!> <!> <div class="values svelte-126e0v0"><div class="svelte-126e0v0"><!></div> <!></div> <div class="values svelte-126e0v0"><div class="svelte-126e0v0"><!> <!></div> <div class="svelte-126e0v0"><!> <!></div></div> <div class="btn svelte-126e0v0"><!> <!></div> <!>', 1), Ov = Pe('<div class="picture svelte-126e0v0"><!></div> <!>', 1);
function Nv(e, t) {
  Lt(t, true);
  let a = vv(t, "user", 15), i = ar(), l = $t(), d = Z(""), n = Z(false), P = Z(""), x = Z(""), T = Z(""), b = rt(() => {
    var _a3;
    return (_a3 = Sh().common()) == null ? void 0 : _a3.map((f) => f);
  }), k = Z("en"), I = Z(""), E = Z(""), $ = Z(""), D = Z(""), S = Z(""), h = Z(""), v = Z(false), m = Z(false), _ = Z(false), R = Z(Ve(Qt())), q = Z(Ve(jo())), A = Z(Ve(Uo(() => t.roles.map((f) => ({ name: f.name, selected: false }))))), L = Z(Ve(Uo(() => t.groups.map((f) => ({ name: f.name, selected: false })))));
  kt(() => {
    var _a3, _b2, _c2, _d2, _e2, _f2, _g2;
    if (a()) {
      if (F(P, a().email, true), F(x, a().given_name, true), F(T, a().family_name || "", true), F(k, a().language, true), F(I, ((_a3 = a().user_values) == null ? void 0 : _a3.birthdate) || "", true), F(E, ((_b2 = a().user_values) == null ? void 0 : _b2.phone) || "", true), F($, ((_c2 = a().user_values) == null ? void 0 : _c2.street) || "", true), F(D, ((_e2 = (_d2 = a().user_values) == null ? void 0 : _d2.zip) == null ? void 0 : _e2.toString()) || "", true), F(S, ((_f2 = a().user_values) == null ? void 0 : _f2.city) || "", true), F(h, ((_g2 = a().user_values) == null ? void 0 : _g2.country) || "", true), F(v, a().enabled, true), F(m, a().email_verified, true), a().user_expires) {
        let f = new Date(a().user_expires * 1e3);
        F(_, true), F(R, Qt(f), true), F(q, jo(f), true);
      } else F(_, false), F(R, Qt(), true), F(q, jo(), true);
      F(A, t.roles.map((f) => {
        var _a4;
        return { name: f.name, selected: ((_a4 = a()) == null ? void 0 : _a4.roles.includes(f.name)) || false };
      }), true), F(L, t.groups.map((f) => {
        var _a4, _b3;
        return { name: f.name, selected: ((_b3 = (_a4 = a()) == null ? void 0 : _a4.groups) == null ? void 0 : _b3.includes(f.name)) || false };
      }), true);
    }
  });
  function r(f) {
    let g = f.given_name[0];
    return f.family_name && (g += f.family_name[0]), g;
  }
  async function s(f, g) {
    var _a3, _b2;
    F(d, "");
    const M = { email: w(P), given_name: w(x), family_name: w(T) || void 0, language: w(k), roles: w(A).filter((y) => y.selected).map((y) => y.name), groups: w(L).filter((y) => y.selected).map((y) => y.name), enabled: w(v), email_verified: w(m), user_expires: w(_) ? Th(w(R), w(q)) : void 0 };
    (w(I) || w(E) || w($) || w(D) || w(S) || w(h)) && (M.user_values = { birthdate: w(I) || void 0, phone: ((_a3 = w(E)) == null ? void 0 : _a3.replaceAll(" ", "")) || void 0, street: w($) || void 0, zip: w(D) ? Number.parseInt(w(D)) : void 0, city: w(S) || void 0, country: w(h) || void 0 });
    let C = await _l(f.action, M);
    C.body ? (F(n, true), a(C.body), t.onSave(), setTimeout(() => {
      F(n, false);
    }, 3e3)) : F(d, ((_b2 = C.error) == null ? void 0 : _b2.message) || "Error", true);
  }
  var o = Jt(), u = De(o);
  {
    var p = (f) => {
      var g = Ov(), M = De(g), C = le(M);
      const y = rt(() => r(a()));
      Ah(C, { get fallbackCharacters() {
        return w(y);
      }, get userId() {
        return a().id;
      }, get pictureId() {
        return a().picture_id;
      }, size: "large" }), oe(M);
      var O = J(M, 2);
      const N = rt(() => `/auth/v1/users/${a().id}`);
      Pl(O, { get action() {
        return w(N);
      }, onSubmit: s, children: (V, U) => {
        var K = Lv(), X = De(K);
        Ht(X, { label: "ID", mono: true, children: (Q, Se) => {
          We();
          var fe = Be();
          he(() => pe(fe, a().id)), ee(Q, fe);
        }, $$slots: { default: true } });
        var H = J(X, 2), re = le(H), ue = le(re);
        Fo(ue, { get ariaLabel() {
          return l.common.enabled;
        }, get checked() {
          return w(v);
        }, set checked(Q) {
          F(v, Q, true);
        }, children: (Q, Se) => {
          We();
          var fe = Be();
          he(() => pe(fe, l.common.enabled)), ee(Q, fe);
        }, $$slots: { default: true } }), oe(re);
        var ye = J(re, 2), Re = le(ye);
        Fo(Re, { get ariaLabel() {
          return i.account.emailVerified;
        }, get checked() {
          return w(m);
        }, set checked(Q) {
          F(m, Q, true);
        }, children: (Q, Se) => {
          We();
          var fe = Be();
          he(() => pe(fe, i.account.emailVerified)), ee(Q, fe);
        }, $$slots: { default: true } }), oe(ye), oe(H);
        var Ae = J(H, 2), je = le(Ae), Le = le(je);
        mt(Le, { typ: "email", autocomplete: "off", label: "E-Mail", placeholder: "E-Mail", required: true, get value() {
          return w(P);
        }, set value(Q) {
          F(P, Q, true);
        } });
        var Me = J(Le, 2);
        mt(Me, { autocomplete: "off", get label() {
          return i.account.givenName;
        }, get placeholder() {
          return i.account.givenName;
        }, required: true, pattern: Wo, get value() {
          return w(x);
        }, set value(Q) {
          F(x, Q, true);
        } });
        var ze = J(Me, 2);
        mt(ze, { autocomplete: "off", get label() {
          return i.account.familyName;
        }, get placeholder() {
          return i.account.familyName;
        }, pattern: Wo, get value() {
          return w(T);
        }, set value(Q) {
          F(T, Q, true);
        } });
        var Te = J(ze, 2);
        bl(Te, { get label() {
          return i.account.birthdate;
        }, withDelete: true, get value() {
          return w(I);
        }, set value(Q) {
          F(I, Q, true);
        } });
        var ct = J(Te, 2);
        {
          var Ue = (Q) => {
            var Se = kv();
            Zt(Se, "", {}, { padding: ".25rem" });
            var fe = le(Se);
            Ht(fe, { get label() {
              return l.common.language;
            }, children: (W, Ge) => {
              qh(W, { get ariaLabel() {
                return i.common.selectI18n;
              }, get options() {
                return w(b);
              }, borderless: true, get value() {
                return w(k);
              }, set value(Ye) {
                F(k, Ye, true);
              } });
            }, $$slots: { default: true } }), oe(Se), ee(Q, Se);
          };
          be(ct, (Q) => {
            w(b) && Q(Ue);
          });
        }
        oe(je);
        var gt = J(je, 2), dt = le(gt);
        mt(dt, { autocomplete: "off", get label() {
          return i.account.street;
        }, get placeholder() {
          return i.account.street;
        }, pattern: fv, get value() {
          return w($);
        }, set value(Q) {
          F($, Q, true);
        } });
        var Rt = J(dt, 2);
        mt(Rt, { typ: "number", autocomplete: "off", get label() {
          return i.account.zip;
        }, get placeholder() {
          return i.account.zip;
        }, min: "1000", max: "9999999", get value() {
          return w(D);
        }, set value(Q) {
          F(D, Q, true);
        } });
        var Dt = J(Rt, 2);
        mt(Dt, { autocomplete: "off", get label() {
          return i.account.city;
        }, get placeholder() {
          return i.account.city;
        }, pattern: Bl, get value() {
          return w(S);
        }, set value(Q) {
          F(S, Q, true);
        } });
        var St = J(Dt, 2);
        mt(St, { autocomplete: "off", get label() {
          return i.account.country;
        }, get placeholder() {
          return i.account.country;
        }, pattern: Bl, get value() {
          return w(h);
        }, set value(Q) {
          F(h, Q, true);
        } });
        var B = J(St, 2);
        mt(B, { autocomplete: "off", get label() {
          return i.account.phone;
        }, get placeholder() {
          return i.account.phone;
        }, pattern: hv, get value() {
          return w(E);
        }, set value(Q) {
          F(E, Q, true);
        } }), oe(gt), oe(Ae);
        var G = J(Ae, 2);
        zo(G, { get items() {
          return w(A);
        }, set items(Q) {
          F(A, Q, true);
        }, children: (Q, Se) => {
          We();
          var fe = Be();
          he(() => pe(fe, i.account.roles)), ee(Q, fe);
        }, $$slots: { default: true } });
        var z = J(G, 2);
        zo(z, { get items() {
          return w(L);
        }, set items(Q) {
          F(L, Q, true);
        }, children: (Q, Se) => {
          We();
          var fe = Be();
          he(() => pe(fe, i.account.groups)), ee(Q, fe);
        }, $$slots: { default: true } });
        var Y = J(z, 2), ne = le(Y);
        Zt(ne, "", {}, { "margin-top": ".5rem" });
        var ae = le(ne);
        Fo(ae, { get ariaLabel() {
          return i.account.accessExp;
        }, get checked() {
          return w(_);
        }, set checked(Q) {
          F(_, Q, true);
        }, children: (Q, Se) => {
          We();
          var fe = Be();
          he(() => pe(fe, i.account.accessExp)), ee(Q, fe);
        }, $$slots: { default: true } }), oe(ne);
        var ve = J(ne, 2);
        {
          var qe = (Q) => {
            var Se = Iv(), fe = le(Se);
            const W = rt(Qt);
            bl(fe, { get label() {
              return i.account.accessExp;
            }, withTime: true, get min() {
              return w(W);
            }, required: true, get value() {
              return w(R);
            }, set value(Ge) {
              F(R, Ge, true);
            }, get timeValue() {
              return w(q);
            }, set timeValue(Ge) {
              F(q, Ge, true);
            } }), oe(Se), Ch(3, Se, () => Ph, () => ({ duration: 150 })), ee(Q, Se);
          };
          be(ve, (Q) => {
            w(_) && Q(qe);
          });
        }
        oe(Y);
        var de = J(Y, 2), xe = le(de), He = le(xe);
        Ht(He, { get label() {
          return i.account.userCreated;
        }, children: (Q, Se) => {
          We();
          var fe = Be();
          he((W) => pe(fe, W), [() => ul(a().created_at)]), ee(Q, fe);
        }, $$slots: { default: true } });
        var ke = J(He, 2);
        Ht(ke, { get label() {
          return l.users.lastLogin;
        }, children: (Q, Se) => {
          var fe = Jt(), W = De(fe);
          {
            var Ge = (Xe) => {
              var Je = Be();
              he((Bt) => pe(Je, Bt), [() => ul(a().last_login)]), ee(Xe, Je);
            }, Ye = (Xe) => {
              var Je = Be();
              he(() => pe(Je, i.common.never)), ee(Xe, Je);
            };
            be(W, (Xe) => {
              a().last_login ? Xe(Ge) : Xe(Ye, false);
            });
          }
          ee(Q, fe);
        }, $$slots: { default: true } }), oe(xe);
        var we = J(xe, 2), $e = le(we);
        Ht($e, { get label() {
          return i.account.passwordExpiry;
        }, children: (Q, Se) => {
          var fe = Jt(), W = De(fe);
          {
            var Ge = (Xe) => {
              var Je = Be();
              he((Bt) => pe(Je, Bt), [() => ul(a().password_expires)]), ee(Xe, Je);
            }, Ye = (Xe) => {
              var Je = Be();
              he(() => pe(Je, i.common.never)), ee(Xe, Je);
            };
            be(W, (Xe) => {
              a().password_expires ? Xe(Ge) : Xe(Ye, false);
            });
          }
          ee(Q, fe);
        }, $$slots: { default: true } });
        var Qe = J($e, 2);
        Ht(Qe, { get label() {
          return i.account.mfaActivated;
        }, children: (Q, Se) => {
          const fe = rt(() => !!a().webauthn_user_id);
          yv(Q, { get checked() {
            return w(fe);
          } });
        }, $$slots: { default: true } }), oe(we), oe(de);
        var et = J(de, 2), st = le(et);
        Et(st, { type: "submit", children: (Q, Se) => {
          We();
          var fe = Be();
          he(() => pe(fe, i.common.save)), ee(Q, fe);
        }, $$slots: { default: true } });
        var pt = J(st, 2);
        {
          var At = (Q) => {
            Zo(Q, {});
          };
          be(pt, (Q) => {
            w(n) && Q(At);
          });
        }
        oe(et);
        var Ke = J(et, 2);
        {
          var wt = (Q) => {
            var Se = Mv(), fe = le(Se, true);
            oe(Se), he(() => pe(fe, w(d))), ee(Q, Se);
          };
          be(Ke, (Q) => {
            w(d) && Q(wt);
          });
        }
        ee(V, K);
      }, $$slots: { default: true } }), ee(f, g);
    };
    be(u, (f) => {
      a() && f(p);
    });
  }
  ee(e, o), Ot();
}
function Dv(e, t) {
  return e.start <= t && t < e.end;
}
function Bv(e, t = {}) {
  const { offsetLine: a = 0, offsetColumn: i = 0 } = t;
  let l = 0;
  const d = e.split(`
`).map((x, T) => {
    const b = l + x.length + 1, k = { start: l, end: b, line: T };
    return l = b, k;
  });
  let n = 0;
  function P(x, T) {
    if (typeof x == "string" && (x = e.indexOf(x, T ?? 0)), x === -1) return;
    let b = d[n];
    const k = x >= b.end ? 1 : -1;
    for (; b; ) {
      if (Dv(b, x)) return { line: a + b.line, column: i + x - b.start, character: x };
      n += k, b = d[n];
    }
  }
  return P;
}
let Fv, jl = Bv("", { offsetLine: 1 }), jv, $l = [], $v = /* @__PURE__ */ new Map();
const Vv = /^\t+/;
function cl(e) {
  return e.replace(Vv, (t) => t.split("	").join("  "));
}
function Hv(e, t, a) {
  const i = e.split(`
`), l = Math.max(0, t - 2), d = Math.min(t + 3, i.length), n = String(d + 1).length;
  return i.slice(l, d).map((P, x) => {
    const T = l + x === t, b = String(x + l + 1).padStart(n, " ");
    if (T) {
      const k = " ".repeat(n + 2 + cl(P.slice(0, a)).length) + "^";
      return `${b}: ${cl(P)}
${k}`;
    }
    return `${b}: ${cl(P)}`;
  }).join(`
`);
}
class wh {
  constructor(t, a, i) {
    __publicField(this, "name", "CompileDiagnostic");
    this.code = t, this.message = a, i && (this.position = i, this.start = jl(i[0]), this.end = jl(i[1]), this.start && this.end && (this.frame = Hv(Fv, this.start.line - 1, this.end.column)));
  }
  toString() {
    let t = `${this.code}: ${this.message}`;
    return this.filename && (t += `
${this.filename}`, this.start && (t += `:${this.start.line}:${this.start.column}`)), this.frame && (t += `
${this.frame}`), t;
  }
  toJSON() {
    return { code: this.code, message: this.message, filename: this.filename, start: this.start, end: this.end, position: this.position, frame: this.frame };
  }
}
class Uv extends wh {
  constructor(t, a, i) {
    super(t, a, i);
    __publicField(this, "name", "CompileWarning");
  }
}
function ir(e, t, a) {
  var _a3;
  let i = $l;
  if (e && (i = $v.get(e) ?? $l), i && ((_a3 = i.at(-1)) == null ? void 0 : _a3.has(t))) return;
  const l = new Uv(t, a, e && e.start !== void 0 ? [e.start, e.end ?? e.start] : void 0);
  jv(l);
}
function Gv(e) {
  ir(e, "options_deprecated_accessors", "The `accessors` option has been deprecated. It will have no effect in runes mode\nhttps://svelte.dev/e/options_deprecated_accessors");
}
function Wv(e) {
  ir(e, "options_deprecated_immutable", "The `immutable` option has been deprecated. It will have no effect in runes mode\nhttps://svelte.dev/e/options_deprecated_immutable");
}
function zv(e) {
  ir(e, "options_removed_enable_sourcemap", "The `enableSourcemap` option has been removed. Source maps are always generated now, and tooling can choose to ignore them\nhttps://svelte.dev/e/options_removed_enable_sourcemap");
}
function Kv(e) {
  ir(e, "options_removed_hydratable", "The `hydratable` option has been removed. Svelte components are always hydratable now\nhttps://svelte.dev/e/options_removed_hydratable");
}
function Xv(e) {
  ir(e, "options_removed_loop_guard_timeout", "The `loopGuardTimeout` option has been removed\nhttps://svelte.dev/e/options_removed_loop_guard_timeout");
}
function Yv(e) {
  ir(e, "options_renamed_ssr_dom", '`generate: "dom"` and `generate: "ssr"` options have been renamed to "client" and "server" respectively\nhttps://svelte.dev/e/options_renamed_ssr_dom');
}
var Qv = [509, 0, 227, 0, 150, 4, 294, 9, 1368, 2, 2, 1, 6, 3, 41, 2, 5, 0, 166, 1, 574, 3, 9, 9, 7, 9, 32, 4, 318, 1, 80, 3, 71, 10, 50, 3, 123, 2, 54, 14, 32, 10, 3, 1, 11, 3, 46, 10, 8, 0, 46, 9, 7, 2, 37, 13, 2, 9, 6, 1, 45, 0, 13, 2, 49, 13, 9, 3, 2, 11, 83, 11, 7, 0, 3, 0, 158, 11, 6, 9, 7, 3, 56, 1, 2, 6, 3, 1, 3, 2, 10, 0, 11, 1, 3, 6, 4, 4, 68, 8, 2, 0, 3, 0, 2, 3, 2, 4, 2, 0, 15, 1, 83, 17, 10, 9, 5, 0, 82, 19, 13, 9, 214, 6, 3, 8, 28, 1, 83, 16, 16, 9, 82, 12, 9, 9, 7, 19, 58, 14, 5, 9, 243, 14, 166, 9, 71, 5, 2, 1, 3, 3, 2, 0, 2, 1, 13, 9, 120, 6, 3, 6, 4, 0, 29, 9, 41, 6, 2, 3, 9, 0, 10, 10, 47, 15, 343, 9, 54, 7, 2, 7, 17, 9, 57, 21, 2, 13, 123, 5, 4, 0, 2, 1, 2, 6, 2, 0, 9, 9, 49, 4, 2, 1, 2, 4, 9, 9, 330, 3, 10, 1, 2, 0, 49, 6, 4, 4, 14, 10, 5350, 0, 7, 14, 11465, 27, 2343, 9, 87, 9, 39, 4, 60, 6, 26, 9, 535, 9, 470, 0, 2, 54, 8, 3, 82, 0, 12, 1, 19628, 1, 4178, 9, 519, 45, 3, 22, 543, 4, 4, 5, 9, 7, 3, 6, 31, 3, 149, 2, 1418, 49, 513, 54, 5, 49, 9, 0, 15, 0, 23, 4, 2, 14, 1361, 6, 2, 16, 3, 6, 2, 1, 2, 4, 101, 0, 161, 6, 10, 9, 357, 0, 62, 13, 499, 13, 245, 1, 2, 9, 726, 6, 110, 6, 6, 9, 4759, 9, 787719, 239], Eh = [0, 11, 2, 25, 2, 18, 2, 1, 2, 14, 3, 13, 35, 122, 70, 52, 268, 28, 4, 48, 48, 31, 14, 29, 6, 37, 11, 29, 3, 35, 5, 7, 2, 4, 43, 157, 19, 35, 5, 35, 5, 39, 9, 51, 13, 10, 2, 14, 2, 6, 2, 1, 2, 10, 2, 14, 2, 6, 2, 1, 4, 51, 13, 310, 10, 21, 11, 7, 25, 5, 2, 41, 2, 8, 70, 5, 3, 0, 2, 43, 2, 1, 4, 0, 3, 22, 11, 22, 10, 30, 66, 18, 2, 1, 11, 21, 11, 25, 71, 55, 7, 1, 65, 0, 16, 3, 2, 2, 2, 28, 43, 28, 4, 28, 36, 7, 2, 27, 28, 53, 11, 21, 11, 18, 14, 17, 111, 72, 56, 50, 14, 50, 14, 35, 39, 27, 10, 22, 251, 41, 7, 1, 17, 2, 60, 28, 11, 0, 9, 21, 43, 17, 47, 20, 28, 22, 13, 52, 58, 1, 3, 0, 14, 44, 33, 24, 27, 35, 30, 0, 3, 0, 9, 34, 4, 0, 13, 47, 15, 3, 22, 0, 2, 0, 36, 17, 2, 24, 20, 1, 64, 6, 2, 0, 2, 3, 2, 14, 2, 9, 8, 46, 39, 7, 3, 1, 3, 21, 2, 6, 2, 1, 2, 4, 4, 0, 19, 0, 13, 4, 31, 9, 2, 0, 3, 0, 2, 37, 2, 0, 26, 0, 2, 0, 45, 52, 19, 3, 21, 2, 31, 47, 21, 1, 2, 0, 185, 46, 42, 3, 37, 47, 21, 0, 60, 42, 14, 0, 72, 26, 38, 6, 186, 43, 117, 63, 32, 7, 3, 0, 3, 7, 2, 1, 2, 23, 16, 0, 2, 0, 95, 7, 3, 38, 17, 0, 2, 0, 29, 0, 11, 39, 8, 0, 22, 0, 12, 45, 20, 0, 19, 72, 200, 32, 32, 8, 2, 36, 18, 0, 50, 29, 113, 6, 2, 1, 2, 37, 22, 0, 26, 5, 2, 1, 2, 31, 15, 0, 328, 18, 16, 0, 2, 12, 2, 33, 125, 0, 80, 921, 103, 110, 18, 195, 2637, 96, 16, 1071, 18, 5, 26, 3994, 6, 582, 6842, 29, 1763, 568, 8, 30, 18, 78, 18, 29, 19, 47, 17, 3, 32, 20, 6, 18, 433, 44, 212, 63, 129, 74, 6, 0, 67, 12, 65, 1, 2, 0, 29, 6135, 9, 1237, 42, 9, 8936, 3, 2, 6, 2, 1, 2, 290, 16, 0, 30, 2, 3, 0, 15, 3, 9, 395, 2309, 106, 6, 12, 4, 8, 8, 9, 5991, 84, 2, 70, 2, 1, 3, 0, 3, 1, 3, 3, 2, 11, 2, 0, 2, 6, 2, 64, 2, 3, 3, 7, 2, 6, 2, 27, 2, 3, 2, 4, 2, 0, 4, 6, 2, 339, 3, 24, 2, 24, 2, 30, 2, 24, 2, 30, 2, 24, 2, 30, 2, 24, 2, 30, 2, 24, 2, 7, 1845, 30, 7, 5, 262, 61, 147, 44, 11, 6, 17, 0, 322, 29, 19, 43, 485, 27, 229, 29, 3, 0, 496, 6, 2, 3, 2, 1, 2, 14, 2, 196, 60, 67, 8, 0, 1205, 3, 2, 26, 2, 1, 2, 0, 3, 0, 2, 9, 2, 3, 2, 0, 2, 0, 7, 0, 5, 0, 2, 0, 2, 0, 2, 2, 2, 1, 2, 0, 3, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 1, 2, 0, 3, 3, 2, 6, 2, 3, 2, 3, 2, 0, 2, 9, 2, 16, 6, 2, 2, 4, 2, 16, 4421, 42719, 33, 4153, 7, 221, 3, 5761, 15, 7472, 16, 621, 2467, 541, 1507, 4938, 6, 4191], Jv = "\u200C\u200D\xB7\u0300-\u036F\u0387\u0483-\u0487\u0591-\u05BD\u05BF\u05C1\u05C2\u05C4\u05C5\u05C7\u0610-\u061A\u064B-\u0669\u0670\u06D6-\u06DC\u06DF-\u06E4\u06E7\u06E8\u06EA-\u06ED\u06F0-\u06F9\u0711\u0730-\u074A\u07A6-\u07B0\u07C0-\u07C9\u07EB-\u07F3\u07FD\u0816-\u0819\u081B-\u0823\u0825-\u0827\u0829-\u082D\u0859-\u085B\u0897-\u089F\u08CA-\u08E1\u08E3-\u0903\u093A-\u093C\u093E-\u094F\u0951-\u0957\u0962\u0963\u0966-\u096F\u0981-\u0983\u09BC\u09BE-\u09C4\u09C7\u09C8\u09CB-\u09CD\u09D7\u09E2\u09E3\u09E6-\u09EF\u09FE\u0A01-\u0A03\u0A3C\u0A3E-\u0A42\u0A47\u0A48\u0A4B-\u0A4D\u0A51\u0A66-\u0A71\u0A75\u0A81-\u0A83\u0ABC\u0ABE-\u0AC5\u0AC7-\u0AC9\u0ACB-\u0ACD\u0AE2\u0AE3\u0AE6-\u0AEF\u0AFA-\u0AFF\u0B01-\u0B03\u0B3C\u0B3E-\u0B44\u0B47\u0B48\u0B4B-\u0B4D\u0B55-\u0B57\u0B62\u0B63\u0B66-\u0B6F\u0B82\u0BBE-\u0BC2\u0BC6-\u0BC8\u0BCA-\u0BCD\u0BD7\u0BE6-\u0BEF\u0C00-\u0C04\u0C3C\u0C3E-\u0C44\u0C46-\u0C48\u0C4A-\u0C4D\u0C55\u0C56\u0C62\u0C63\u0C66-\u0C6F\u0C81-\u0C83\u0CBC\u0CBE-\u0CC4\u0CC6-\u0CC8\u0CCA-\u0CCD\u0CD5\u0CD6\u0CE2\u0CE3\u0CE6-\u0CEF\u0CF3\u0D00-\u0D03\u0D3B\u0D3C\u0D3E-\u0D44\u0D46-\u0D48\u0D4A-\u0D4D\u0D57\u0D62\u0D63\u0D66-\u0D6F\u0D81-\u0D83\u0DCA\u0DCF-\u0DD4\u0DD6\u0DD8-\u0DDF\u0DE6-\u0DEF\u0DF2\u0DF3\u0E31\u0E34-\u0E3A\u0E47-\u0E4E\u0E50-\u0E59\u0EB1\u0EB4-\u0EBC\u0EC8-\u0ECE\u0ED0-\u0ED9\u0F18\u0F19\u0F20-\u0F29\u0F35\u0F37\u0F39\u0F3E\u0F3F\u0F71-\u0F84\u0F86\u0F87\u0F8D-\u0F97\u0F99-\u0FBC\u0FC6\u102B-\u103E\u1040-\u1049\u1056-\u1059\u105E-\u1060\u1062-\u1064\u1067-\u106D\u1071-\u1074\u1082-\u108D\u108F-\u109D\u135D-\u135F\u1369-\u1371\u1712-\u1715\u1732-\u1734\u1752\u1753\u1772\u1773\u17B4-\u17D3\u17DD\u17E0-\u17E9\u180B-\u180D\u180F-\u1819\u18A9\u1920-\u192B\u1930-\u193B\u1946-\u194F\u19D0-\u19DA\u1A17-\u1A1B\u1A55-\u1A5E\u1A60-\u1A7C\u1A7F-\u1A89\u1A90-\u1A99\u1AB0-\u1ABD\u1ABF-\u1ACE\u1B00-\u1B04\u1B34-\u1B44\u1B50-\u1B59\u1B6B-\u1B73\u1B80-\u1B82\u1BA1-\u1BAD\u1BB0-\u1BB9\u1BE6-\u1BF3\u1C24-\u1C37\u1C40-\u1C49\u1C50-\u1C59\u1CD0-\u1CD2\u1CD4-\u1CE8\u1CED\u1CF4\u1CF7-\u1CF9\u1DC0-\u1DFF\u200C\u200D\u203F\u2040\u2054\u20D0-\u20DC\u20E1\u20E5-\u20F0\u2CEF-\u2CF1\u2D7F\u2DE0-\u2DFF\u302A-\u302F\u3099\u309A\u30FB\uA620-\uA629\uA66F\uA674-\uA67D\uA69E\uA69F\uA6F0\uA6F1\uA802\uA806\uA80B\uA823-\uA827\uA82C\uA880\uA881\uA8B4-\uA8C5\uA8D0-\uA8D9\uA8E0-\uA8F1\uA8FF-\uA909\uA926-\uA92D\uA947-\uA953\uA980-\uA983\uA9B3-\uA9C0\uA9D0-\uA9D9\uA9E5\uA9F0-\uA9F9\uAA29-\uAA36\uAA43\uAA4C\uAA4D\uAA50-\uAA59\uAA7B-\uAA7D\uAAB0\uAAB2-\uAAB4\uAAB7\uAAB8\uAABE\uAABF\uAAC1\uAAEB-\uAAEF\uAAF5\uAAF6\uABE3-\uABEA\uABEC\uABED\uABF0-\uABF9\uFB1E\uFE00-\uFE0F\uFE20-\uFE2F\uFE33\uFE34\uFE4D-\uFE4F\uFF10-\uFF19\uFF3F\uFF65", kh = "\xAA\xB5\xBA\xC0-\xD6\xD8-\xF6\xF8-\u02C1\u02C6-\u02D1\u02E0-\u02E4\u02EC\u02EE\u0370-\u0374\u0376\u0377\u037A-\u037D\u037F\u0386\u0388-\u038A\u038C\u038E-\u03A1\u03A3-\u03F5\u03F7-\u0481\u048A-\u052F\u0531-\u0556\u0559\u0560-\u0588\u05D0-\u05EA\u05EF-\u05F2\u0620-\u064A\u066E\u066F\u0671-\u06D3\u06D5\u06E5\u06E6\u06EE\u06EF\u06FA-\u06FC\u06FF\u0710\u0712-\u072F\u074D-\u07A5\u07B1\u07CA-\u07EA\u07F4\u07F5\u07FA\u0800-\u0815\u081A\u0824\u0828\u0840-\u0858\u0860-\u086A\u0870-\u0887\u0889-\u088E\u08A0-\u08C9\u0904-\u0939\u093D\u0950\u0958-\u0961\u0971-\u0980\u0985-\u098C\u098F\u0990\u0993-\u09A8\u09AA-\u09B0\u09B2\u09B6-\u09B9\u09BD\u09CE\u09DC\u09DD\u09DF-\u09E1\u09F0\u09F1\u09FC\u0A05-\u0A0A\u0A0F\u0A10\u0A13-\u0A28\u0A2A-\u0A30\u0A32\u0A33\u0A35\u0A36\u0A38\u0A39\u0A59-\u0A5C\u0A5E\u0A72-\u0A74\u0A85-\u0A8D\u0A8F-\u0A91\u0A93-\u0AA8\u0AAA-\u0AB0\u0AB2\u0AB3\u0AB5-\u0AB9\u0ABD\u0AD0\u0AE0\u0AE1\u0AF9\u0B05-\u0B0C\u0B0F\u0B10\u0B13-\u0B28\u0B2A-\u0B30\u0B32\u0B33\u0B35-\u0B39\u0B3D\u0B5C\u0B5D\u0B5F-\u0B61\u0B71\u0B83\u0B85-\u0B8A\u0B8E-\u0B90\u0B92-\u0B95\u0B99\u0B9A\u0B9C\u0B9E\u0B9F\u0BA3\u0BA4\u0BA8-\u0BAA\u0BAE-\u0BB9\u0BD0\u0C05-\u0C0C\u0C0E-\u0C10\u0C12-\u0C28\u0C2A-\u0C39\u0C3D\u0C58-\u0C5A\u0C5D\u0C60\u0C61\u0C80\u0C85-\u0C8C\u0C8E-\u0C90\u0C92-\u0CA8\u0CAA-\u0CB3\u0CB5-\u0CB9\u0CBD\u0CDD\u0CDE\u0CE0\u0CE1\u0CF1\u0CF2\u0D04-\u0D0C\u0D0E-\u0D10\u0D12-\u0D3A\u0D3D\u0D4E\u0D54-\u0D56\u0D5F-\u0D61\u0D7A-\u0D7F\u0D85-\u0D96\u0D9A-\u0DB1\u0DB3-\u0DBB\u0DBD\u0DC0-\u0DC6\u0E01-\u0E30\u0E32\u0E33\u0E40-\u0E46\u0E81\u0E82\u0E84\u0E86-\u0E8A\u0E8C-\u0EA3\u0EA5\u0EA7-\u0EB0\u0EB2\u0EB3\u0EBD\u0EC0-\u0EC4\u0EC6\u0EDC-\u0EDF\u0F00\u0F40-\u0F47\u0F49-\u0F6C\u0F88-\u0F8C\u1000-\u102A\u103F\u1050-\u1055\u105A-\u105D\u1061\u1065\u1066\u106E-\u1070\u1075-\u1081\u108E\u10A0-\u10C5\u10C7\u10CD\u10D0-\u10FA\u10FC-\u1248\u124A-\u124D\u1250-\u1256\u1258\u125A-\u125D\u1260-\u1288\u128A-\u128D\u1290-\u12B0\u12B2-\u12B5\u12B8-\u12BE\u12C0\u12C2-\u12C5\u12C8-\u12D6\u12D8-\u1310\u1312-\u1315\u1318-\u135A\u1380-\u138F\u13A0-\u13F5\u13F8-\u13FD\u1401-\u166C\u166F-\u167F\u1681-\u169A\u16A0-\u16EA\u16EE-\u16F8\u1700-\u1711\u171F-\u1731\u1740-\u1751\u1760-\u176C\u176E-\u1770\u1780-\u17B3\u17D7\u17DC\u1820-\u1878\u1880-\u18A8\u18AA\u18B0-\u18F5\u1900-\u191E\u1950-\u196D\u1970-\u1974\u1980-\u19AB\u19B0-\u19C9\u1A00-\u1A16\u1A20-\u1A54\u1AA7\u1B05-\u1B33\u1B45-\u1B4C\u1B83-\u1BA0\u1BAE\u1BAF\u1BBA-\u1BE5\u1C00-\u1C23\u1C4D-\u1C4F\u1C5A-\u1C7D\u1C80-\u1C8A\u1C90-\u1CBA\u1CBD-\u1CBF\u1CE9-\u1CEC\u1CEE-\u1CF3\u1CF5\u1CF6\u1CFA\u1D00-\u1DBF\u1E00-\u1F15\u1F18-\u1F1D\u1F20-\u1F45\u1F48-\u1F4D\u1F50-\u1F57\u1F59\u1F5B\u1F5D\u1F5F-\u1F7D\u1F80-\u1FB4\u1FB6-\u1FBC\u1FBE\u1FC2-\u1FC4\u1FC6-\u1FCC\u1FD0-\u1FD3\u1FD6-\u1FDB\u1FE0-\u1FEC\u1FF2-\u1FF4\u1FF6-\u1FFC\u2071\u207F\u2090-\u209C\u2102\u2107\u210A-\u2113\u2115\u2118-\u211D\u2124\u2126\u2128\u212A-\u2139\u213C-\u213F\u2145-\u2149\u214E\u2160-\u2188\u2C00-\u2CE4\u2CEB-\u2CEE\u2CF2\u2CF3\u2D00-\u2D25\u2D27\u2D2D\u2D30-\u2D67\u2D6F\u2D80-\u2D96\u2DA0-\u2DA6\u2DA8-\u2DAE\u2DB0-\u2DB6\u2DB8-\u2DBE\u2DC0-\u2DC6\u2DC8-\u2DCE\u2DD0-\u2DD6\u2DD8-\u2DDE\u3005-\u3007\u3021-\u3029\u3031-\u3035\u3038-\u303C\u3041-\u3096\u309B-\u309F\u30A1-\u30FA\u30FC-\u30FF\u3105-\u312F\u3131-\u318E\u31A0-\u31BF\u31F0-\u31FF\u3400-\u4DBF\u4E00-\uA48C\uA4D0-\uA4FD\uA500-\uA60C\uA610-\uA61F\uA62A\uA62B\uA640-\uA66E\uA67F-\uA69D\uA6A0-\uA6EF\uA717-\uA71F\uA722-\uA788\uA78B-\uA7CD\uA7D0\uA7D1\uA7D3\uA7D5-\uA7DC\uA7F2-\uA801\uA803-\uA805\uA807-\uA80A\uA80C-\uA822\uA840-\uA873\uA882-\uA8B3\uA8F2-\uA8F7\uA8FB\uA8FD\uA8FE\uA90A-\uA925\uA930-\uA946\uA960-\uA97C\uA984-\uA9B2\uA9CF\uA9E0-\uA9E4\uA9E6-\uA9EF\uA9FA-\uA9FE\uAA00-\uAA28\uAA40-\uAA42\uAA44-\uAA4B\uAA60-\uAA76\uAA7A\uAA7E-\uAAAF\uAAB1\uAAB5\uAAB6\uAAB9-\uAABD\uAAC0\uAAC2\uAADB-\uAADD\uAAE0-\uAAEA\uAAF2-\uAAF4\uAB01-\uAB06\uAB09-\uAB0E\uAB11-\uAB16\uAB20-\uAB26\uAB28-\uAB2E\uAB30-\uAB5A\uAB5C-\uAB69\uAB70-\uABE2\uAC00-\uD7A3\uD7B0-\uD7C6\uD7CB-\uD7FB\uF900-\uFA6D\uFA70-\uFAD9\uFB00-\uFB06\uFB13-\uFB17\uFB1D\uFB1F-\uFB28\uFB2A-\uFB36\uFB38-\uFB3C\uFB3E\uFB40\uFB41\uFB43\uFB44\uFB46-\uFBB1\uFBD3-\uFD3D\uFD50-\uFD8F\uFD92-\uFDC7\uFDF0-\uFDFB\uFE70-\uFE74\uFE76-\uFEFC\uFF21-\uFF3A\uFF41-\uFF5A\uFF66-\uFFBE\uFFC2-\uFFC7\uFFCA-\uFFCF\uFFD2-\uFFD7\uFFDA-\uFFDC", dl = { 3: "abstract boolean byte char class double enum export extends final float goto implements import int interface long native package private protected public short static super synchronized throws transient volatile", 5: "class enum extends super const export import", 6: "enum", strict: "implements interface let package private protected public static yield", strictBind: "eval arguments" }, pl = "break case catch continue debugger default do else finally for function if return switch throw try var while with null true false instanceof typeof void delete new in this", Zv = { 5: pl, "5module": pl + " export import", 6: pl + " const class extends export import super" }, e1 = /^in(stanceof)?$/, t1 = new RegExp("[" + kh + "]"), r1 = new RegExp("[" + kh + Jv + "]");
function yl(e, t) {
  for (var a = 65536, i = 0; i < t.length; i += 2) {
    if (a += t[i], a > e) return false;
    if (a += t[i + 1], a >= e) return true;
  }
  return false;
}
function Pt(e, t) {
  return e < 65 ? e === 36 : e < 91 ? true : e < 97 ? e === 95 : e < 123 ? true : e <= 65535 ? e >= 170 && t1.test(String.fromCharCode(e)) : t === false ? false : yl(e, Eh);
}
function Ut(e, t) {
  return e < 48 ? e === 36 : e < 58 ? true : e < 65 ? false : e < 91 ? true : e < 97 ? e === 95 : e < 123 ? true : e <= 65535 ? e >= 170 && r1.test(String.fromCharCode(e)) : t === false ? false : yl(e, Eh) || yl(e, Qv);
}
var me = function(t, a) {
  a === void 0 && (a = {}), this.label = t, this.keyword = a.keyword, this.beforeExpr = !!a.beforeExpr, this.startsExpr = !!a.startsExpr, this.isLoop = !!a.isLoop, this.isAssign = !!a.isAssign, this.prefix = !!a.prefix, this.postfix = !!a.postfix, this.binop = a.binop || null, this.updateContext = null;
};
function ft(e, t) {
  return new me(e, { beforeExpr: true, binop: t });
}
var ht = { beforeExpr: true }, it = { startsExpr: true }, er = {};
function Ce(e, t) {
  return t === void 0 && (t = {}), t.keyword = e, er[e] = new me(e, t);
}
var c = { num: new me("num", it), regexp: new me("regexp", it), string: new me("string", it), name: new me("name", it), privateId: new me("privateId", it), eof: new me("eof"), bracketL: new me("[", { beforeExpr: true, startsExpr: true }), bracketR: new me("]"), braceL: new me("{", { beforeExpr: true, startsExpr: true }), braceR: new me("}"), parenL: new me("(", { beforeExpr: true, startsExpr: true }), parenR: new me(")"), comma: new me(",", ht), semi: new me(";", ht), colon: new me(":", ht), dot: new me("."), question: new me("?", ht), questionDot: new me("?."), arrow: new me("=>", ht), template: new me("template"), invalidTemplate: new me("invalidTemplate"), ellipsis: new me("...", ht), backQuote: new me("`", it), dollarBraceL: new me("${", { beforeExpr: true, startsExpr: true }), eq: new me("=", { beforeExpr: true, isAssign: true }), assign: new me("_=", { beforeExpr: true, isAssign: true }), incDec: new me("++/--", { prefix: true, postfix: true, startsExpr: true }), prefix: new me("!/~", { beforeExpr: true, prefix: true, startsExpr: true }), logicalOR: ft("||", 1), logicalAND: ft("&&", 2), bitwiseOR: ft("|", 3), bitwiseXOR: ft("^", 4), bitwiseAND: ft("&", 5), equality: ft("==/!=/===/!==", 6), relational: ft("</>/<=/>=", 7), bitShift: ft("<</>>/>>>", 8), plusMin: new me("+/-", { beforeExpr: true, binop: 9, prefix: true, startsExpr: true }), modulo: ft("%", 10), star: ft("*", 10), slash: ft("/", 10), starstar: new me("**", { beforeExpr: true }), coalesce: ft("??", 1), _break: Ce("break"), _case: Ce("case", ht), _catch: Ce("catch"), _continue: Ce("continue"), _debugger: Ce("debugger"), _default: Ce("default", ht), _do: Ce("do", { isLoop: true, beforeExpr: true }), _else: Ce("else", ht), _finally: Ce("finally"), _for: Ce("for", { isLoop: true }), _function: Ce("function", it), _if: Ce("if"), _return: Ce("return", ht), _switch: Ce("switch"), _throw: Ce("throw", ht), _try: Ce("try"), _var: Ce("var"), _const: Ce("const"), _while: Ce("while", { isLoop: true }), _with: Ce("with"), _new: Ce("new", { beforeExpr: true, startsExpr: true }), _this: Ce("this", it), _super: Ce("super", it), _class: Ce("class", it), _extends: Ce("extends", ht), _export: Ce("export"), _import: Ce("import", it), _null: Ce("null", it), _true: Ce("true", it), _false: Ce("false", it), _in: Ce("in", { beforeExpr: true, binop: 7 }), _instanceof: Ce("instanceof", { beforeExpr: true, binop: 7 }), _typeof: Ce("typeof", { beforeExpr: true, prefix: true, startsExpr: true }), _void: Ce("void", { beforeExpr: true, prefix: true, startsExpr: true }), _delete: Ce("delete", { beforeExpr: true, prefix: true, startsExpr: true }) }, ut = /\r\n?|\n|\u2028|\u2029/, Ih = new RegExp(ut.source, "g");
function Wt(e) {
  return e === 10 || e === 13 || e === 8232 || e === 8233;
}
function Mh(e, t, a) {
  a === void 0 && (a = e.length);
  for (var i = t; i < a; i++) {
    var l = e.charCodeAt(i);
    if (Wt(l)) return i < a - 1 && l === 13 && e.charCodeAt(i + 1) === 10 ? i + 2 : i + 1;
  }
  return -1;
}
var ql = /[\u1680\u2000-\u200a\u202f\u205f\u3000\ufeff]/, vt = /(?:\s|\/\/.*|\/\*[^]*?\*\/)*/g, Lh = Object.prototype, a1 = Lh.hasOwnProperty, i1 = Lh.toString, sr = Object.hasOwn || function(e, t) {
  return a1.call(e, t);
}, Vl = Array.isArray || function(e) {
  return i1.call(e) === "[object Array]";
}, Hl = /* @__PURE__ */ Object.create(null);
function Ft(e) {
  return Hl[e] || (Hl[e] = new RegExp("^(?:" + e.replace(/ /g, "|") + ")$"));
}
function It(e) {
  return e <= 65535 ? String.fromCharCode(e) : (e -= 65536, String.fromCharCode((e >> 10) + 55296, (e & 1023) + 56320));
}
var s1 = /(?:[\uD800-\uDBFF](?![\uDC00-\uDFFF])|(?:[^\uD800-\uDBFF]|^)[\uDC00-\uDFFF])/, tr = function(t, a) {
  this.line = t, this.column = a;
};
tr.prototype.offset = function(t) {
  return new tr(this.line, this.column + t);
};
var Ao = function(t, a, i) {
  this.start = a, this.end = i, t.sourceFile !== null && (this.source = t.sourceFile);
};
function Tl(e, t) {
  for (var a = 1, i = 0; ; ) {
    var l = Mh(e, i, t);
    if (l < 0) return new tr(a, t - i);
    ++a, i = l;
  }
}
var Ko = { ecmaVersion: null, sourceType: "script", onInsertedSemicolon: null, onTrailingComma: null, allowReserved: null, allowReturnOutsideFunction: false, allowImportExportEverywhere: false, allowAwaitOutsideFunction: null, allowSuperOutsideMethod: null, allowHashBang: false, checkPrivateFields: true, locations: false, onToken: null, onComment: null, ranges: false, program: null, sourceFile: null, directSourceFile: null, preserveParens: false }, Ul = false;
function n1(e) {
  var t = {};
  for (var a in Ko) t[a] = e && sr(e, a) ? e[a] : Ko[a];
  if (t.ecmaVersion === "latest" ? t.ecmaVersion = 1e8 : t.ecmaVersion == null ? (!Ul && typeof console == "object" && console.warn && (Ul = true, console.warn(`Since Acorn 8.0.0, options.ecmaVersion is required.
Defaulting to 2020, but this will stop working in the future.`)), t.ecmaVersion = 11) : t.ecmaVersion >= 2015 && (t.ecmaVersion -= 2009), t.allowReserved == null && (t.allowReserved = t.ecmaVersion < 5), (!e || e.allowHashBang == null) && (t.allowHashBang = t.ecmaVersion >= 14), Vl(t.onToken)) {
    var i = t.onToken;
    t.onToken = function(l) {
      return i.push(l);
    };
  }
  return Vl(t.onComment) && (t.onComment = o1(t, t.onComment)), t;
}
function o1(e, t) {
  return function(a, i, l, d, n, P) {
    var x = { type: a ? "Block" : "Line", value: i, start: l, end: d };
    e.locations && (x.loc = new Ao(this, n, P)), e.ranges && (x.range = [l, d]), t.push(x);
  };
}
var So = 1, nr = 2, Sl = 4, Oh = 8, Al = 16, Nh = 32, el = 64, Dh = 128, zt = 256, wo = 512, tl = So | nr | zt;
function wl(e, t) {
  return nr | (e ? Sl : 0) | (t ? Oh : 0);
}
var Xo = 0, El = 1, Nt = 2, Bh = 3, Fh = 4, jh = 5, Fe = function(t, a, i) {
  this.options = t = n1(t), this.sourceFile = t.sourceFile, this.keywords = Ft(Zv[t.ecmaVersion >= 6 ? 6 : t.sourceType === "module" ? "5module" : 5]);
  var l = "";
  t.allowReserved !== true && (l = dl[t.ecmaVersion >= 6 ? 6 : t.ecmaVersion === 5 ? 5 : 3], t.sourceType === "module" && (l += " await")), this.reservedWords = Ft(l);
  var d = (l ? l + " " : "") + dl.strict;
  this.reservedWordsStrict = Ft(d), this.reservedWordsStrictBind = Ft(d + " " + dl.strictBind), this.input = String(a), this.containsEsc = false, i ? (this.pos = i, this.lineStart = this.input.lastIndexOf(`
`, i - 1) + 1, this.curLine = this.input.slice(0, this.lineStart).split(ut).length) : (this.pos = this.lineStart = 0, this.curLine = 1), this.type = c.eof, this.value = null, this.start = this.end = this.pos, this.startLoc = this.endLoc = this.curPosition(), this.lastTokEndLoc = this.lastTokStartLoc = null, this.lastTokStart = this.lastTokEnd = this.pos, this.context = this.initialContext(), this.exprAllowed = true, this.inModule = t.sourceType === "module", this.strict = this.inModule || this.strictDirective(this.pos), this.potentialArrowAt = -1, this.potentialArrowInForAwait = false, this.yieldPos = this.awaitPos = this.awaitIdentPos = 0, this.labels = [], this.undefinedExports = /* @__PURE__ */ Object.create(null), this.pos === 0 && t.allowHashBang && this.input.slice(0, 2) === "#!" && this.skipLineComment(2), this.scopeStack = [], this.enterScope(So), this.regexpState = null, this.privateNameStack = [];
}, qt = { inFunction: { configurable: true }, inGenerator: { configurable: true }, inAsync: { configurable: true }, canAwait: { configurable: true }, allowSuper: { configurable: true }, allowDirectSuper: { configurable: true }, treatFunctionsAsVar: { configurable: true }, allowNewDotTarget: { configurable: true }, inClassStaticBlock: { configurable: true } };
Fe.prototype.parse = function() {
  var t = this.options.program || this.startNode();
  return this.nextToken(), this.parseTopLevel(t);
};
qt.inFunction.get = function() {
  return (this.currentVarScope().flags & nr) > 0;
};
qt.inGenerator.get = function() {
  return (this.currentVarScope().flags & Oh) > 0;
};
qt.inAsync.get = function() {
  return (this.currentVarScope().flags & Sl) > 0;
};
qt.canAwait.get = function() {
  for (var e = this.scopeStack.length - 1; e >= 0; e--) {
    var t = this.scopeStack[e], a = t.flags;
    if (a & (zt | wo)) return false;
    if (a & nr) return (a & Sl) > 0;
  }
  return this.inModule && this.options.ecmaVersion >= 13 || this.options.allowAwaitOutsideFunction;
};
qt.allowSuper.get = function() {
  var e = this.currentThisScope(), t = e.flags;
  return (t & el) > 0 || this.options.allowSuperOutsideMethod;
};
qt.allowDirectSuper.get = function() {
  return (this.currentThisScope().flags & Dh) > 0;
};
qt.treatFunctionsAsVar.get = function() {
  return this.treatFunctionsAsVarInScope(this.currentScope());
};
qt.allowNewDotTarget.get = function() {
  for (var e = this.scopeStack.length - 1; e >= 0; e--) {
    var t = this.scopeStack[e], a = t.flags;
    if (a & (zt | wo) || a & nr && !(a & Al)) return true;
  }
  return false;
};
qt.inClassStaticBlock.get = function() {
  return (this.currentVarScope().flags & zt) > 0;
};
Fe.extend = function() {
  for (var t = [], a = arguments.length; a--; ) t[a] = arguments[a];
  for (var i = this, l = 0; l < t.length; l++) i = t[l](i);
  return i;
};
Fe.parse = function(t, a) {
  return new this(a, t).parse();
};
Fe.parseExpressionAt = function(t, a, i) {
  var l = new this(i, t, a);
  return l.nextToken(), l.parseExpression();
};
Fe.tokenizer = function(t, a) {
  return new this(a, t);
};
Object.defineProperties(Fe.prototype, qt);
var at = Fe.prototype, l1 = /^(?:'((?:\\[^]|[^'\\])*?)'|"((?:\\[^]|[^"\\])*?)")/;
at.strictDirective = function(e) {
  if (this.options.ecmaVersion < 5) return false;
  for (; ; ) {
    vt.lastIndex = e, e += vt.exec(this.input)[0].length;
    var t = l1.exec(this.input.slice(e));
    if (!t) return false;
    if ((t[1] || t[2]) === "use strict") {
      vt.lastIndex = e + t[0].length;
      var a = vt.exec(this.input), i = a.index + a[0].length, l = this.input.charAt(i);
      return l === ";" || l === "}" || ut.test(a[0]) && !(/[(`.[+\-/*%<>=,?^&]/.test(l) || l === "!" && this.input.charAt(i + 1) === "=");
    }
    e += t[0].length, vt.lastIndex = e, e += vt.exec(this.input)[0].length, this.input[e] === ";" && e++;
  }
};
at.eat = function(e) {
  return this.type === e ? (this.next(), true) : false;
};
at.isContextual = function(e) {
  return this.type === c.name && this.value === e && !this.containsEsc;
};
at.eatContextual = function(e) {
  return this.isContextual(e) ? (this.next(), true) : false;
};
at.expectContextual = function(e) {
  this.eatContextual(e) || this.unexpected();
};
at.canInsertSemicolon = function() {
  return this.type === c.eof || this.type === c.braceR || ut.test(this.input.slice(this.lastTokEnd, this.start));
};
at.insertSemicolon = function() {
  if (this.canInsertSemicolon()) return this.options.onInsertedSemicolon && this.options.onInsertedSemicolon(this.lastTokEnd, this.lastTokEndLoc), true;
};
at.semicolon = function() {
  !this.eat(c.semi) && !this.insertSemicolon() && this.unexpected();
};
at.afterTrailingComma = function(e, t) {
  if (this.type === e) return this.options.onTrailingComma && this.options.onTrailingComma(this.lastTokStart, this.lastTokStartLoc), t || this.next(), true;
};
at.expect = function(e) {
  this.eat(e) || this.unexpected();
};
at.unexpected = function(e) {
  this.raise(e ?? this.start, "Unexpected token");
};
var rl = function() {
  this.shorthandAssign = this.trailingComma = this.parenthesizedAssign = this.parenthesizedBind = this.doubleProto = -1;
};
at.checkPatternErrors = function(e, t) {
  if (e) {
    e.trailingComma > -1 && this.raiseRecoverable(e.trailingComma, "Comma is not permitted after the rest element");
    var a = t ? e.parenthesizedAssign : e.parenthesizedBind;
    a > -1 && this.raiseRecoverable(a, t ? "Assigning to rvalue" : "Parenthesized pattern");
  }
};
at.checkExpressionErrors = function(e, t) {
  if (!e) return false;
  var a = e.shorthandAssign, i = e.doubleProto;
  if (!t) return a >= 0 || i >= 0;
  a >= 0 && this.raise(a, "Shorthand property assignments are valid only in destructuring patterns"), i >= 0 && this.raiseRecoverable(i, "Redefinition of __proto__ property");
};
at.checkYieldAwaitInDefaultParams = function() {
  this.yieldPos && (!this.awaitPos || this.yieldPos < this.awaitPos) && this.raise(this.yieldPos, "Yield expression cannot be a default value"), this.awaitPos && this.raise(this.awaitPos, "Await expression cannot be a default value");
};
at.isSimpleAssignTarget = function(e) {
  return e.type === "ParenthesizedExpression" ? this.isSimpleAssignTarget(e.expression) : e.type === "Identifier" || e.type === "MemberExpression";
};
var se = Fe.prototype;
se.parseTopLevel = function(e) {
  var t = /* @__PURE__ */ Object.create(null);
  for (e.body || (e.body = []); this.type !== c.eof; ) {
    var a = this.parseStatement(null, true, t);
    e.body.push(a);
  }
  if (this.inModule) for (var i = 0, l = Object.keys(this.undefinedExports); i < l.length; i += 1) {
    var d = l[i];
    this.raiseRecoverable(this.undefinedExports[d].start, "Export '" + d + "' is not defined");
  }
  return this.adaptDirectivePrologue(e.body), this.next(), e.sourceType = this.options.sourceType, this.finishNode(e, "Program");
};
var kl = { kind: "loop" }, u1 = { kind: "switch" };
se.isLet = function(e) {
  if (this.options.ecmaVersion < 6 || !this.isContextual("let")) return false;
  vt.lastIndex = this.pos;
  var t = vt.exec(this.input), a = this.pos + t[0].length, i = this.input.charCodeAt(a);
  if (i === 91 || i === 92) return true;
  if (e) return false;
  if (i === 123 || i > 55295 && i < 56320) return true;
  if (Pt(i, true)) {
    for (var l = a + 1; Ut(i = this.input.charCodeAt(l), true); ) ++l;
    if (i === 92 || i > 55295 && i < 56320) return true;
    var d = this.input.slice(a, l);
    if (!e1.test(d)) return true;
  }
  return false;
};
se.isAsyncFunction = function() {
  if (this.options.ecmaVersion < 8 || !this.isContextual("async")) return false;
  vt.lastIndex = this.pos;
  var e = vt.exec(this.input), t = this.pos + e[0].length, a;
  return !ut.test(this.input.slice(this.pos, t)) && this.input.slice(t, t + 8) === "function" && (t + 8 === this.input.length || !(Ut(a = this.input.charCodeAt(t + 8)) || a > 55295 && a < 56320));
};
se.parseStatement = function(e, t, a) {
  var i = this.type, l = this.startNode(), d;
  switch (this.isLet(e) && (i = c._var, d = "let"), i) {
    case c._break:
    case c._continue:
      return this.parseBreakContinueStatement(l, i.keyword);
    case c._debugger:
      return this.parseDebuggerStatement(l);
    case c._do:
      return this.parseDoStatement(l);
    case c._for:
      return this.parseForStatement(l);
    case c._function:
      return e && (this.strict || e !== "if" && e !== "label") && this.options.ecmaVersion >= 6 && this.unexpected(), this.parseFunctionStatement(l, false, !e);
    case c._class:
      return e && this.unexpected(), this.parseClass(l, true);
    case c._if:
      return this.parseIfStatement(l);
    case c._return:
      return this.parseReturnStatement(l);
    case c._switch:
      return this.parseSwitchStatement(l);
    case c._throw:
      return this.parseThrowStatement(l);
    case c._try:
      return this.parseTryStatement(l);
    case c._const:
    case c._var:
      return d = d || this.value, e && d !== "var" && this.unexpected(), this.parseVarStatement(l, d);
    case c._while:
      return this.parseWhileStatement(l);
    case c._with:
      return this.parseWithStatement(l);
    case c.braceL:
      return this.parseBlock(true, l);
    case c.semi:
      return this.parseEmptyStatement(l);
    case c._export:
    case c._import:
      if (this.options.ecmaVersion > 10 && i === c._import) {
        vt.lastIndex = this.pos;
        var n = vt.exec(this.input), P = this.pos + n[0].length, x = this.input.charCodeAt(P);
        if (x === 40 || x === 46) return this.parseExpressionStatement(l, this.parseExpression());
      }
      return this.options.allowImportExportEverywhere || (t || this.raise(this.start, "'import' and 'export' may only appear at the top level"), this.inModule || this.raise(this.start, "'import' and 'export' may appear only with 'sourceType: module'")), i === c._import ? this.parseImport(l) : this.parseExport(l, a);
    default:
      if (this.isAsyncFunction()) return e && this.unexpected(), this.next(), this.parseFunctionStatement(l, true, !e);
      var T = this.value, b = this.parseExpression();
      return i === c.name && b.type === "Identifier" && this.eat(c.colon) ? this.parseLabeledStatement(l, T, b, e) : this.parseExpressionStatement(l, b);
  }
};
se.parseBreakContinueStatement = function(e, t) {
  var a = t === "break";
  this.next(), this.eat(c.semi) || this.insertSemicolon() ? e.label = null : this.type !== c.name ? this.unexpected() : (e.label = this.parseIdent(), this.semicolon());
  for (var i = 0; i < this.labels.length; ++i) {
    var l = this.labels[i];
    if ((e.label == null || l.name === e.label.name) && (l.kind != null && (a || l.kind === "loop") || e.label && a)) break;
  }
  return i === this.labels.length && this.raise(e.start, "Unsyntactic " + t), this.finishNode(e, a ? "BreakStatement" : "ContinueStatement");
};
se.parseDebuggerStatement = function(e) {
  return this.next(), this.semicolon(), this.finishNode(e, "DebuggerStatement");
};
se.parseDoStatement = function(e) {
  return this.next(), this.labels.push(kl), e.body = this.parseStatement("do"), this.labels.pop(), this.expect(c._while), e.test = this.parseParenExpression(), this.options.ecmaVersion >= 6 ? this.eat(c.semi) : this.semicolon(), this.finishNode(e, "DoWhileStatement");
};
se.parseForStatement = function(e) {
  this.next();
  var t = this.options.ecmaVersion >= 9 && this.canAwait && this.eatContextual("await") ? this.lastTokStart : -1;
  if (this.labels.push(kl), this.enterScope(0), this.expect(c.parenL), this.type === c.semi) return t > -1 && this.unexpected(t), this.parseFor(e, null);
  var a = this.isLet();
  if (this.type === c._var || this.type === c._const || a) {
    var i = this.startNode(), l = a ? "let" : this.value;
    return this.next(), this.parseVar(i, true, l), this.finishNode(i, "VariableDeclaration"), (this.type === c._in || this.options.ecmaVersion >= 6 && this.isContextual("of")) && i.declarations.length === 1 ? (this.options.ecmaVersion >= 9 && (this.type === c._in ? t > -1 && this.unexpected(t) : e.await = t > -1), this.parseForIn(e, i)) : (t > -1 && this.unexpected(t), this.parseFor(e, i));
  }
  var d = this.isContextual("let"), n = false, P = this.containsEsc, x = new rl(), T = this.start, b = t > -1 ? this.parseExprSubscripts(x, "await") : this.parseExpression(true, x);
  return this.type === c._in || (n = this.options.ecmaVersion >= 6 && this.isContextual("of")) ? (t > -1 ? (this.type === c._in && this.unexpected(t), e.await = true) : n && this.options.ecmaVersion >= 8 && (b.start === T && !P && b.type === "Identifier" && b.name === "async" ? this.unexpected() : this.options.ecmaVersion >= 9 && (e.await = false)), d && n && this.raise(b.start, "The left-hand side of a for-of loop may not start with 'let'."), this.toAssignable(b, false, x), this.checkLValPattern(b), this.parseForIn(e, b)) : (this.checkExpressionErrors(x, true), t > -1 && this.unexpected(t), this.parseFor(e, b));
};
se.parseFunctionStatement = function(e, t, a) {
  return this.next(), this.parseFunction(e, qo | (a ? 0 : gl), false, t);
};
se.parseIfStatement = function(e) {
  return this.next(), e.test = this.parseParenExpression(), e.consequent = this.parseStatement("if"), e.alternate = this.eat(c._else) ? this.parseStatement("if") : null, this.finishNode(e, "IfStatement");
};
se.parseReturnStatement = function(e) {
  return !this.inFunction && !this.options.allowReturnOutsideFunction && this.raise(this.start, "'return' outside of function"), this.next(), this.eat(c.semi) || this.insertSemicolon() ? e.argument = null : (e.argument = this.parseExpression(), this.semicolon()), this.finishNode(e, "ReturnStatement");
};
se.parseSwitchStatement = function(e) {
  this.next(), e.discriminant = this.parseParenExpression(), e.cases = [], this.expect(c.braceL), this.labels.push(u1), this.enterScope(0);
  for (var t, a = false; this.type !== c.braceR; ) if (this.type === c._case || this.type === c._default) {
    var i = this.type === c._case;
    t && this.finishNode(t, "SwitchCase"), e.cases.push(t = this.startNode()), t.consequent = [], this.next(), i ? t.test = this.parseExpression() : (a && this.raiseRecoverable(this.lastTokStart, "Multiple default clauses"), a = true, t.test = null), this.expect(c.colon);
  } else t || this.unexpected(), t.consequent.push(this.parseStatement(null));
  return this.exitScope(), t && this.finishNode(t, "SwitchCase"), this.next(), this.labels.pop(), this.finishNode(e, "SwitchStatement");
};
se.parseThrowStatement = function(e) {
  return this.next(), ut.test(this.input.slice(this.lastTokEnd, this.start)) && this.raise(this.lastTokEnd, "Illegal newline after throw"), e.argument = this.parseExpression(), this.semicolon(), this.finishNode(e, "ThrowStatement");
};
var c1 = [];
se.parseCatchClauseParam = function() {
  var e = this.parseBindingAtom(), t = e.type === "Identifier";
  return this.enterScope(t ? Nh : 0), this.checkLValPattern(e, t ? Fh : Nt), this.expect(c.parenR), e;
};
se.parseTryStatement = function(e) {
  if (this.next(), e.block = this.parseBlock(), e.handler = null, this.type === c._catch) {
    var t = this.startNode();
    this.next(), this.eat(c.parenL) ? t.param = this.parseCatchClauseParam() : (this.options.ecmaVersion < 10 && this.unexpected(), t.param = null, this.enterScope(0)), t.body = this.parseBlock(false), this.exitScope(), e.handler = this.finishNode(t, "CatchClause");
  }
  return e.finalizer = this.eat(c._finally) ? this.parseBlock() : null, !e.handler && !e.finalizer && this.raise(e.start, "Missing catch or finally clause"), this.finishNode(e, "TryStatement");
};
se.parseVarStatement = function(e, t, a) {
  return this.next(), this.parseVar(e, false, t, a), this.semicolon(), this.finishNode(e, "VariableDeclaration");
};
se.parseWhileStatement = function(e) {
  return this.next(), e.test = this.parseParenExpression(), this.labels.push(kl), e.body = this.parseStatement("while"), this.labels.pop(), this.finishNode(e, "WhileStatement");
};
se.parseWithStatement = function(e) {
  return this.strict && this.raise(this.start, "'with' in strict mode"), this.next(), e.object = this.parseParenExpression(), e.body = this.parseStatement("with"), this.finishNode(e, "WithStatement");
};
se.parseEmptyStatement = function(e) {
  return this.next(), this.finishNode(e, "EmptyStatement");
};
se.parseLabeledStatement = function(e, t, a, i) {
  for (var l = 0, d = this.labels; l < d.length; l += 1) {
    var n = d[l];
    n.name === t && this.raise(a.start, "Label '" + t + "' is already declared");
  }
  for (var P = this.type.isLoop ? "loop" : this.type === c._switch ? "switch" : null, x = this.labels.length - 1; x >= 0; x--) {
    var T = this.labels[x];
    if (T.statementStart === e.start) T.statementStart = this.start, T.kind = P;
    else break;
  }
  return this.labels.push({ name: t, kind: P, statementStart: this.start }), e.body = this.parseStatement(i ? i.indexOf("label") === -1 ? i + "label" : i : "label"), this.labels.pop(), e.label = a, this.finishNode(e, "LabeledStatement");
};
se.parseExpressionStatement = function(e, t) {
  return e.expression = t, this.semicolon(), this.finishNode(e, "ExpressionStatement");
};
se.parseBlock = function(e, t, a) {
  for (e === void 0 && (e = true), t === void 0 && (t = this.startNode()), t.body = [], this.expect(c.braceL), e && this.enterScope(0); this.type !== c.braceR; ) {
    var i = this.parseStatement(null);
    t.body.push(i);
  }
  return a && (this.strict = false), this.next(), e && this.exitScope(), this.finishNode(t, "BlockStatement");
};
se.parseFor = function(e, t) {
  return e.init = t, this.expect(c.semi), e.test = this.type === c.semi ? null : this.parseExpression(), this.expect(c.semi), e.update = this.type === c.parenR ? null : this.parseExpression(), this.expect(c.parenR), e.body = this.parseStatement("for"), this.exitScope(), this.labels.pop(), this.finishNode(e, "ForStatement");
};
se.parseForIn = function(e, t) {
  var a = this.type === c._in;
  return this.next(), t.type === "VariableDeclaration" && t.declarations[0].init != null && (!a || this.options.ecmaVersion < 8 || this.strict || t.kind !== "var" || t.declarations[0].id.type !== "Identifier") && this.raise(t.start, (a ? "for-in" : "for-of") + " loop variable declaration may not have an initializer"), e.left = t, e.right = a ? this.parseExpression() : this.parseMaybeAssign(), this.expect(c.parenR), e.body = this.parseStatement("for"), this.exitScope(), this.labels.pop(), this.finishNode(e, a ? "ForInStatement" : "ForOfStatement");
};
se.parseVar = function(e, t, a, i) {
  for (e.declarations = [], e.kind = a; ; ) {
    var l = this.startNode();
    if (this.parseVarId(l, a), this.eat(c.eq) ? l.init = this.parseMaybeAssign(t) : !i && a === "const" && !(this.type === c._in || this.options.ecmaVersion >= 6 && this.isContextual("of")) ? this.unexpected() : !i && l.id.type !== "Identifier" && !(t && (this.type === c._in || this.isContextual("of"))) ? this.raise(this.lastTokEnd, "Complex binding patterns require an initialization value") : l.init = null, e.declarations.push(this.finishNode(l, "VariableDeclarator")), !this.eat(c.comma)) break;
  }
  return e;
};
se.parseVarId = function(e, t) {
  e.id = this.parseBindingAtom(), this.checkLValPattern(e.id, t === "var" ? El : Nt, false);
};
var qo = 1, gl = 2, $h = 4;
se.parseFunction = function(e, t, a, i, l) {
  this.initFunction(e), (this.options.ecmaVersion >= 9 || this.options.ecmaVersion >= 6 && !i) && (this.type === c.star && t & gl && this.unexpected(), e.generator = this.eat(c.star)), this.options.ecmaVersion >= 8 && (e.async = !!i), t & qo && (e.id = t & $h && this.type !== c.name ? null : this.parseIdent(), e.id && !(t & gl) && this.checkLValSimple(e.id, this.strict || e.generator || e.async ? this.treatFunctionsAsVar ? El : Nt : Bh));
  var d = this.yieldPos, n = this.awaitPos, P = this.awaitIdentPos;
  return this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0, this.enterScope(wl(e.async, e.generator)), t & qo || (e.id = this.type === c.name ? this.parseIdent() : null), this.parseFunctionParams(e), this.parseFunctionBody(e, a, false, l), this.yieldPos = d, this.awaitPos = n, this.awaitIdentPos = P, this.finishNode(e, t & qo ? "FunctionDeclaration" : "FunctionExpression");
};
se.parseFunctionParams = function(e) {
  this.expect(c.parenL), e.params = this.parseBindingList(c.parenR, false, this.options.ecmaVersion >= 8), this.checkYieldAwaitInDefaultParams();
};
se.parseClass = function(e, t) {
  this.next();
  var a = this.strict;
  this.strict = true, this.parseClassId(e, t), this.parseClassSuper(e);
  var i = this.enterClassBody(), l = this.startNode(), d = false;
  for (l.body = [], this.expect(c.braceL); this.type !== c.braceR; ) {
    var n = this.parseClassElement(e.superClass !== null);
    n && (l.body.push(n), n.type === "MethodDefinition" && n.kind === "constructor" ? (d && this.raiseRecoverable(n.start, "Duplicate constructor in the same class"), d = true) : n.key && n.key.type === "PrivateIdentifier" && d1(i, n) && this.raiseRecoverable(n.key.start, "Identifier '#" + n.key.name + "' has already been declared"));
  }
  return this.strict = a, this.next(), e.body = this.finishNode(l, "ClassBody"), this.exitClassBody(), this.finishNode(e, t ? "ClassDeclaration" : "ClassExpression");
};
se.parseClassElement = function(e) {
  if (this.eat(c.semi)) return null;
  var t = this.options.ecmaVersion, a = this.startNode(), i = "", l = false, d = false, n = "method", P = false;
  if (this.eatContextual("static")) {
    if (t >= 13 && this.eat(c.braceL)) return this.parseClassStaticBlock(a), a;
    this.isClassElementNameStart() || this.type === c.star ? P = true : i = "static";
  }
  if (a.static = P, !i && t >= 8 && this.eatContextual("async") && ((this.isClassElementNameStart() || this.type === c.star) && !this.canInsertSemicolon() ? d = true : i = "async"), !i && (t >= 9 || !d) && this.eat(c.star) && (l = true), !i && !d && !l) {
    var x = this.value;
    (this.eatContextual("get") || this.eatContextual("set")) && (this.isClassElementNameStart() ? n = x : i = x);
  }
  if (i ? (a.computed = false, a.key = this.startNodeAt(this.lastTokStart, this.lastTokStartLoc), a.key.name = i, this.finishNode(a.key, "Identifier")) : this.parseClassElementName(a), t < 13 || this.type === c.parenL || n !== "method" || l || d) {
    var T = !a.static && Yo(a, "constructor"), b = T && e;
    T && n !== "method" && this.raise(a.key.start, "Constructor can't have get/set modifier"), a.kind = T ? "constructor" : n, this.parseClassMethod(a, l, d, b);
  } else this.parseClassField(a);
  return a;
};
se.isClassElementNameStart = function() {
  return this.type === c.name || this.type === c.privateId || this.type === c.num || this.type === c.string || this.type === c.bracketL || this.type.keyword;
};
se.parseClassElementName = function(e) {
  this.type === c.privateId ? (this.value === "constructor" && this.raise(this.start, "Classes can't have an element named '#constructor'"), e.computed = false, e.key = this.parsePrivateIdent()) : this.parsePropertyName(e);
};
se.parseClassMethod = function(e, t, a, i) {
  var l = e.key;
  e.kind === "constructor" ? (t && this.raise(l.start, "Constructor can't be a generator"), a && this.raise(l.start, "Constructor can't be an async method")) : e.static && Yo(e, "prototype") && this.raise(l.start, "Classes may not have a static property named prototype");
  var d = e.value = this.parseMethod(t, a, i);
  return e.kind === "get" && d.params.length !== 0 && this.raiseRecoverable(d.start, "getter should have no params"), e.kind === "set" && d.params.length !== 1 && this.raiseRecoverable(d.start, "setter should have exactly one param"), e.kind === "set" && d.params[0].type === "RestElement" && this.raiseRecoverable(d.params[0].start, "Setter cannot use rest params"), this.finishNode(e, "MethodDefinition");
};
se.parseClassField = function(e) {
  return Yo(e, "constructor") ? this.raise(e.key.start, "Classes can't have a field named 'constructor'") : e.static && Yo(e, "prototype") && this.raise(e.key.start, "Classes can't have a static field named 'prototype'"), this.eat(c.eq) ? (this.enterScope(wo | el), e.value = this.parseMaybeAssign(), this.exitScope()) : e.value = null, this.semicolon(), this.finishNode(e, "PropertyDefinition");
};
se.parseClassStaticBlock = function(e) {
  e.body = [];
  var t = this.labels;
  for (this.labels = [], this.enterScope(zt | el); this.type !== c.braceR; ) {
    var a = this.parseStatement(null);
    e.body.push(a);
  }
  return this.next(), this.exitScope(), this.labels = t, this.finishNode(e, "StaticBlock");
};
se.parseClassId = function(e, t) {
  this.type === c.name ? (e.id = this.parseIdent(), t && this.checkLValSimple(e.id, Nt, false)) : (t === true && this.unexpected(), e.id = null);
};
se.parseClassSuper = function(e) {
  e.superClass = this.eat(c._extends) ? this.parseExprSubscripts(null, false) : null;
};
se.enterClassBody = function() {
  var e = { declared: /* @__PURE__ */ Object.create(null), used: [] };
  return this.privateNameStack.push(e), e.declared;
};
se.exitClassBody = function() {
  var e = this.privateNameStack.pop(), t = e.declared, a = e.used;
  if (this.options.checkPrivateFields) for (var i = this.privateNameStack.length, l = i === 0 ? null : this.privateNameStack[i - 1], d = 0; d < a.length; ++d) {
    var n = a[d];
    sr(t, n.name) || (l ? l.used.push(n) : this.raiseRecoverable(n.start, "Private field '#" + n.name + "' must be declared in an enclosing class"));
  }
};
function d1(e, t) {
  var a = t.key.name, i = e[a], l = "true";
  return t.type === "MethodDefinition" && (t.kind === "get" || t.kind === "set") && (l = (t.static ? "s" : "i") + t.kind), i === "iget" && l === "iset" || i === "iset" && l === "iget" || i === "sget" && l === "sset" || i === "sset" && l === "sget" ? (e[a] = "true", false) : i ? true : (e[a] = l, false);
}
function Yo(e, t) {
  var a = e.computed, i = e.key;
  return !a && (i.type === "Identifier" && i.name === t || i.type === "Literal" && i.value === t);
}
se.parseExportAllDeclaration = function(e, t) {
  return this.options.ecmaVersion >= 11 && (this.eatContextual("as") ? (e.exported = this.parseModuleExportName(), this.checkExport(t, e.exported, this.lastTokStart)) : e.exported = null), this.expectContextual("from"), this.type !== c.string && this.unexpected(), e.source = this.parseExprAtom(), this.options.ecmaVersion >= 16 && (e.attributes = this.parseWithClause()), this.semicolon(), this.finishNode(e, "ExportAllDeclaration");
};
se.parseExport = function(e, t) {
  if (this.next(), this.eat(c.star)) return this.parseExportAllDeclaration(e, t);
  if (this.eat(c._default)) return this.checkExport(t, "default", this.lastTokStart), e.declaration = this.parseExportDefaultDeclaration(), this.finishNode(e, "ExportDefaultDeclaration");
  if (this.shouldParseExportStatement()) e.declaration = this.parseExportDeclaration(e), e.declaration.type === "VariableDeclaration" ? this.checkVariableExport(t, e.declaration.declarations) : this.checkExport(t, e.declaration.id, e.declaration.id.start), e.specifiers = [], e.source = null, this.options.ecmaVersion >= 16 && (e.attributes = []);
  else {
    if (e.declaration = null, e.specifiers = this.parseExportSpecifiers(t), this.eatContextual("from")) this.type !== c.string && this.unexpected(), e.source = this.parseExprAtom(), this.options.ecmaVersion >= 16 && (e.attributes = this.parseWithClause());
    else {
      for (var a = 0, i = e.specifiers; a < i.length; a += 1) {
        var l = i[a];
        this.checkUnreserved(l.local), this.checkLocalExport(l.local), l.local.type === "Literal" && this.raise(l.local.start, "A string literal cannot be used as an exported binding without `from`.");
      }
      e.source = null, this.options.ecmaVersion >= 16 && (e.attributes = []);
    }
    this.semicolon();
  }
  return this.finishNode(e, "ExportNamedDeclaration");
};
se.parseExportDeclaration = function(e) {
  return this.parseStatement(null);
};
se.parseExportDefaultDeclaration = function() {
  var e;
  if (this.type === c._function || (e = this.isAsyncFunction())) {
    var t = this.startNode();
    return this.next(), e && this.next(), this.parseFunction(t, qo | $h, false, e);
  } else if (this.type === c._class) {
    var a = this.startNode();
    return this.parseClass(a, "nullableID");
  } else {
    var i = this.parseMaybeAssign();
    return this.semicolon(), i;
  }
};
se.checkExport = function(e, t, a) {
  e && (typeof t != "string" && (t = t.type === "Identifier" ? t.name : t.value), sr(e, t) && this.raiseRecoverable(a, "Duplicate export '" + t + "'"), e[t] = true);
};
se.checkPatternExport = function(e, t) {
  var a = t.type;
  if (a === "Identifier") this.checkExport(e, t, t.start);
  else if (a === "ObjectPattern") for (var i = 0, l = t.properties; i < l.length; i += 1) {
    var d = l[i];
    this.checkPatternExport(e, d);
  }
  else if (a === "ArrayPattern") for (var n = 0, P = t.elements; n < P.length; n += 1) {
    var x = P[n];
    x && this.checkPatternExport(e, x);
  }
  else a === "Property" ? this.checkPatternExport(e, t.value) : a === "AssignmentPattern" ? this.checkPatternExport(e, t.left) : a === "RestElement" && this.checkPatternExport(e, t.argument);
};
se.checkVariableExport = function(e, t) {
  if (e) for (var a = 0, i = t; a < i.length; a += 1) {
    var l = i[a];
    this.checkPatternExport(e, l.id);
  }
};
se.shouldParseExportStatement = function() {
  return this.type.keyword === "var" || this.type.keyword === "const" || this.type.keyword === "class" || this.type.keyword === "function" || this.isLet() || this.isAsyncFunction();
};
se.parseExportSpecifier = function(e) {
  var t = this.startNode();
  return t.local = this.parseModuleExportName(), t.exported = this.eatContextual("as") ? this.parseModuleExportName() : t.local, this.checkExport(e, t.exported, t.exported.start), this.finishNode(t, "ExportSpecifier");
};
se.parseExportSpecifiers = function(e) {
  var t = [], a = true;
  for (this.expect(c.braceL); !this.eat(c.braceR); ) {
    if (a) a = false;
    else if (this.expect(c.comma), this.afterTrailingComma(c.braceR)) break;
    t.push(this.parseExportSpecifier(e));
  }
  return t;
};
se.parseImport = function(e) {
  return this.next(), this.type === c.string ? (e.specifiers = c1, e.source = this.parseExprAtom()) : (e.specifiers = this.parseImportSpecifiers(), this.expectContextual("from"), e.source = this.type === c.string ? this.parseExprAtom() : this.unexpected()), this.options.ecmaVersion >= 16 && (e.attributes = this.parseWithClause()), this.semicolon(), this.finishNode(e, "ImportDeclaration");
};
se.parseImportSpecifier = function() {
  var e = this.startNode();
  return e.imported = this.parseModuleExportName(), this.eatContextual("as") ? e.local = this.parseIdent() : (this.checkUnreserved(e.imported), e.local = e.imported), this.checkLValSimple(e.local, Nt), this.finishNode(e, "ImportSpecifier");
};
se.parseImportDefaultSpecifier = function() {
  var e = this.startNode();
  return e.local = this.parseIdent(), this.checkLValSimple(e.local, Nt), this.finishNode(e, "ImportDefaultSpecifier");
};
se.parseImportNamespaceSpecifier = function() {
  var e = this.startNode();
  return this.next(), this.expectContextual("as"), e.local = this.parseIdent(), this.checkLValSimple(e.local, Nt), this.finishNode(e, "ImportNamespaceSpecifier");
};
se.parseImportSpecifiers = function() {
  var e = [], t = true;
  if (this.type === c.name && (e.push(this.parseImportDefaultSpecifier()), !this.eat(c.comma))) return e;
  if (this.type === c.star) return e.push(this.parseImportNamespaceSpecifier()), e;
  for (this.expect(c.braceL); !this.eat(c.braceR); ) {
    if (t) t = false;
    else if (this.expect(c.comma), this.afterTrailingComma(c.braceR)) break;
    e.push(this.parseImportSpecifier());
  }
  return e;
};
se.parseWithClause = function() {
  var e = [];
  if (!this.eat(c._with)) return e;
  this.expect(c.braceL);
  for (var t = {}, a = true; !this.eat(c.braceR); ) {
    if (a) a = false;
    else if (this.expect(c.comma), this.afterTrailingComma(c.braceR)) break;
    var i = this.parseImportAttribute(), l = i.key.type === "Identifier" ? i.key.name : i.key.value;
    sr(t, l) && this.raiseRecoverable(i.key.start, "Duplicate attribute key '" + l + "'"), t[l] = true, e.push(i);
  }
  return e;
};
se.parseImportAttribute = function() {
  var e = this.startNode();
  return e.key = this.type === c.string ? this.parseExprAtom() : this.parseIdent(this.options.allowReserved !== "never"), this.expect(c.colon), this.type !== c.string && this.unexpected(), e.value = this.parseExprAtom(), this.finishNode(e, "ImportAttribute");
};
se.parseModuleExportName = function() {
  if (this.options.ecmaVersion >= 13 && this.type === c.string) {
    var e = this.parseLiteral(this.value);
    return s1.test(e.value) && this.raise(e.start, "An export name cannot include a lone surrogate."), e;
  }
  return this.parseIdent(true);
};
se.adaptDirectivePrologue = function(e) {
  for (var t = 0; t < e.length && this.isDirectiveCandidate(e[t]); ++t) e[t].directive = e[t].expression.raw.slice(1, -1);
};
se.isDirectiveCandidate = function(e) {
  return this.options.ecmaVersion >= 5 && e.type === "ExpressionStatement" && e.expression.type === "Literal" && typeof e.expression.value == "string" && (this.input[e.start] === '"' || this.input[e.start] === "'");
};
var yt = Fe.prototype;
yt.toAssignable = function(e, t, a) {
  if (this.options.ecmaVersion >= 6 && e) switch (e.type) {
    case "Identifier":
      this.inAsync && e.name === "await" && this.raise(e.start, "Cannot use 'await' as identifier inside an async function");
      break;
    case "ObjectPattern":
    case "ArrayPattern":
    case "AssignmentPattern":
    case "RestElement":
      break;
    case "ObjectExpression":
      e.type = "ObjectPattern", a && this.checkPatternErrors(a, true);
      for (var i = 0, l = e.properties; i < l.length; i += 1) {
        var d = l[i];
        this.toAssignable(d, t), d.type === "RestElement" && (d.argument.type === "ArrayPattern" || d.argument.type === "ObjectPattern") && this.raise(d.argument.start, "Unexpected token");
      }
      break;
    case "Property":
      e.kind !== "init" && this.raise(e.key.start, "Object pattern can't contain getter or setter"), this.toAssignable(e.value, t);
      break;
    case "ArrayExpression":
      e.type = "ArrayPattern", a && this.checkPatternErrors(a, true), this.toAssignableList(e.elements, t);
      break;
    case "SpreadElement":
      e.type = "RestElement", this.toAssignable(e.argument, t), e.argument.type === "AssignmentPattern" && this.raise(e.argument.start, "Rest elements cannot have a default value");
      break;
    case "AssignmentExpression":
      e.operator !== "=" && this.raise(e.left.end, "Only '=' operator can be used for specifying default value."), e.type = "AssignmentPattern", delete e.operator, this.toAssignable(e.left, t);
      break;
    case "ParenthesizedExpression":
      this.toAssignable(e.expression, t, a);
      break;
    case "ChainExpression":
      this.raiseRecoverable(e.start, "Optional chaining cannot appear in left-hand side");
      break;
    case "MemberExpression":
      if (!t) break;
    default:
      this.raise(e.start, "Assigning to rvalue");
  }
  else a && this.checkPatternErrors(a, true);
  return e;
};
yt.toAssignableList = function(e, t) {
  for (var a = e.length, i = 0; i < a; i++) {
    var l = e[i];
    l && this.toAssignable(l, t);
  }
  if (a) {
    var d = e[a - 1];
    this.options.ecmaVersion === 6 && t && d && d.type === "RestElement" && d.argument.type !== "Identifier" && this.unexpected(d.argument.start);
  }
  return e;
};
yt.parseSpread = function(e) {
  var t = this.startNode();
  return this.next(), t.argument = this.parseMaybeAssign(false, e), this.finishNode(t, "SpreadElement");
};
yt.parseRestBinding = function() {
  var e = this.startNode();
  return this.next(), this.options.ecmaVersion === 6 && this.type !== c.name && this.unexpected(), e.argument = this.parseBindingAtom(), this.finishNode(e, "RestElement");
};
yt.parseBindingAtom = function() {
  if (this.options.ecmaVersion >= 6) switch (this.type) {
    case c.bracketL:
      var e = this.startNode();
      return this.next(), e.elements = this.parseBindingList(c.bracketR, true, true), this.finishNode(e, "ArrayPattern");
    case c.braceL:
      return this.parseObj(true);
  }
  return this.parseIdent();
};
yt.parseBindingList = function(e, t, a, i) {
  for (var l = [], d = true; !this.eat(e); ) if (d ? d = false : this.expect(c.comma), t && this.type === c.comma) l.push(null);
  else {
    if (a && this.afterTrailingComma(e)) break;
    if (this.type === c.ellipsis) {
      var n = this.parseRestBinding();
      this.parseBindingListItem(n), l.push(n), this.type === c.comma && this.raiseRecoverable(this.start, "Comma is not permitted after the rest element"), this.expect(e);
      break;
    } else l.push(this.parseAssignableListItem(i));
  }
  return l;
};
yt.parseAssignableListItem = function(e) {
  var t = this.parseMaybeDefault(this.start, this.startLoc);
  return this.parseBindingListItem(t), t;
};
yt.parseBindingListItem = function(e) {
  return e;
};
yt.parseMaybeDefault = function(e, t, a) {
  if (a = a || this.parseBindingAtom(), this.options.ecmaVersion < 6 || !this.eat(c.eq)) return a;
  var i = this.startNodeAt(e, t);
  return i.left = a, i.right = this.parseMaybeAssign(), this.finishNode(i, "AssignmentPattern");
};
yt.checkLValSimple = function(e, t, a) {
  t === void 0 && (t = Xo);
  var i = t !== Xo;
  switch (e.type) {
    case "Identifier":
      this.strict && this.reservedWordsStrictBind.test(e.name) && this.raiseRecoverable(e.start, (i ? "Binding " : "Assigning to ") + e.name + " in strict mode"), i && (t === Nt && e.name === "let" && this.raiseRecoverable(e.start, "let is disallowed as a lexically bound name"), a && (sr(a, e.name) && this.raiseRecoverable(e.start, "Argument name clash"), a[e.name] = true), t !== jh && this.declareName(e.name, t, e.start));
      break;
    case "ChainExpression":
      this.raiseRecoverable(e.start, "Optional chaining cannot appear in left-hand side");
      break;
    case "MemberExpression":
      i && this.raiseRecoverable(e.start, "Binding member expression");
      break;
    case "ParenthesizedExpression":
      return i && this.raiseRecoverable(e.start, "Binding parenthesized expression"), this.checkLValSimple(e.expression, t, a);
    default:
      this.raise(e.start, (i ? "Binding" : "Assigning to") + " rvalue");
  }
};
yt.checkLValPattern = function(e, t, a) {
  switch (t === void 0 && (t = Xo), e.type) {
    case "ObjectPattern":
      for (var i = 0, l = e.properties; i < l.length; i += 1) {
        var d = l[i];
        this.checkLValInnerPattern(d, t, a);
      }
      break;
    case "ArrayPattern":
      for (var n = 0, P = e.elements; n < P.length; n += 1) {
        var x = P[n];
        x && this.checkLValInnerPattern(x, t, a);
      }
      break;
    default:
      this.checkLValSimple(e, t, a);
  }
};
yt.checkLValInnerPattern = function(e, t, a) {
  switch (t === void 0 && (t = Xo), e.type) {
    case "Property":
      this.checkLValInnerPattern(e.value, t, a);
      break;
    case "AssignmentPattern":
      this.checkLValPattern(e.left, t, a);
      break;
    case "RestElement":
      this.checkLValPattern(e.argument, t, a);
      break;
    default:
      this.checkLValPattern(e, t, a);
  }
};
var tt = function(t, a, i, l, d) {
  this.token = t, this.isExpr = !!a, this.preserveSpace = !!i, this.override = l, this.generator = !!d;
}, Oe = { b_stat: new tt("{", false), b_expr: new tt("{", true), b_tmpl: new tt("${", false), p_stat: new tt("(", false), p_expr: new tt("(", true), q_tmpl: new tt("`", true, true, function(e) {
  return e.tryReadTemplateToken();
}), f_stat: new tt("function", false), f_expr: new tt("function", true), f_expr_gen: new tt("function", true, false, null, true), f_gen: new tt("function", false, false, null, true) }, or = Fe.prototype;
or.initialContext = function() {
  return [Oe.b_stat];
};
or.curContext = function() {
  return this.context[this.context.length - 1];
};
or.braceIsBlock = function(e) {
  var t = this.curContext();
  return t === Oe.f_expr || t === Oe.f_stat ? true : e === c.colon && (t === Oe.b_stat || t === Oe.b_expr) ? !t.isExpr : e === c._return || e === c.name && this.exprAllowed ? ut.test(this.input.slice(this.lastTokEnd, this.start)) : e === c._else || e === c.semi || e === c.eof || e === c.parenR || e === c.arrow ? true : e === c.braceL ? t === Oe.b_stat : e === c._var || e === c._const || e === c.name ? false : !this.exprAllowed;
};
or.inGeneratorContext = function() {
  for (var e = this.context.length - 1; e >= 1; e--) {
    var t = this.context[e];
    if (t.token === "function") return t.generator;
  }
  return false;
};
or.updateContext = function(e) {
  var t, a = this.type;
  a.keyword && e === c.dot ? this.exprAllowed = false : (t = a.updateContext) ? t.call(this, e) : this.exprAllowed = a.beforeExpr;
};
or.overrideContext = function(e) {
  this.curContext() !== e && (this.context[this.context.length - 1] = e);
};
c.parenR.updateContext = c.braceR.updateContext = function() {
  if (this.context.length === 1) {
    this.exprAllowed = true;
    return;
  }
  var e = this.context.pop();
  e === Oe.b_stat && this.curContext().token === "function" && (e = this.context.pop()), this.exprAllowed = !e.isExpr;
};
c.braceL.updateContext = function(e) {
  this.context.push(this.braceIsBlock(e) ? Oe.b_stat : Oe.b_expr), this.exprAllowed = true;
};
c.dollarBraceL.updateContext = function() {
  this.context.push(Oe.b_tmpl), this.exprAllowed = true;
};
c.parenL.updateContext = function(e) {
  var t = e === c._if || e === c._for || e === c._with || e === c._while;
  this.context.push(t ? Oe.p_stat : Oe.p_expr), this.exprAllowed = true;
};
c.incDec.updateContext = function() {
};
c._function.updateContext = c._class.updateContext = function(e) {
  e.beforeExpr && e !== c._else && !(e === c.semi && this.curContext() !== Oe.p_stat) && !(e === c._return && ut.test(this.input.slice(this.lastTokEnd, this.start))) && !((e === c.colon || e === c.braceL) && this.curContext() === Oe.b_stat) ? this.context.push(Oe.f_expr) : this.context.push(Oe.f_stat), this.exprAllowed = false;
};
c.colon.updateContext = function() {
  this.curContext().token === "function" && this.context.pop(), this.exprAllowed = true;
};
c.backQuote.updateContext = function() {
  this.curContext() === Oe.q_tmpl ? this.context.pop() : this.context.push(Oe.q_tmpl), this.exprAllowed = false;
};
c.star.updateContext = function(e) {
  if (e === c._function) {
    var t = this.context.length - 1;
    this.context[t] === Oe.f_expr ? this.context[t] = Oe.f_expr_gen : this.context[t] = Oe.f_gen;
  }
  this.exprAllowed = true;
};
c.name.updateContext = function(e) {
  var t = false;
  this.options.ecmaVersion >= 6 && e !== c.dot && (this.value === "of" && !this.exprAllowed || this.value === "yield" && this.inGeneratorContext()) && (t = true), this.exprAllowed = t;
};
var ce = Fe.prototype;
ce.checkPropClash = function(e, t, a) {
  if (!(this.options.ecmaVersion >= 9 && e.type === "SpreadElement") && !(this.options.ecmaVersion >= 6 && (e.computed || e.method || e.shorthand))) {
    var i = e.key, l;
    switch (i.type) {
      case "Identifier":
        l = i.name;
        break;
      case "Literal":
        l = String(i.value);
        break;
      default:
        return;
    }
    var d = e.kind;
    if (this.options.ecmaVersion >= 6) {
      l === "__proto__" && d === "init" && (t.proto && (a ? a.doubleProto < 0 && (a.doubleProto = i.start) : this.raiseRecoverable(i.start, "Redefinition of __proto__ property")), t.proto = true);
      return;
    }
    l = "$" + l;
    var n = t[l];
    if (n) {
      var P;
      d === "init" ? P = this.strict && n.init || n.get || n.set : P = n.init || n[d], P && this.raiseRecoverable(i.start, "Redefinition of property");
    } else n = t[l] = { init: false, get: false, set: false };
    n[d] = true;
  }
};
ce.parseExpression = function(e, t) {
  var a = this.start, i = this.startLoc, l = this.parseMaybeAssign(e, t);
  if (this.type === c.comma) {
    var d = this.startNodeAt(a, i);
    for (d.expressions = [l]; this.eat(c.comma); ) d.expressions.push(this.parseMaybeAssign(e, t));
    return this.finishNode(d, "SequenceExpression");
  }
  return l;
};
ce.parseMaybeAssign = function(e, t, a) {
  if (this.isContextual("yield")) {
    if (this.inGenerator) return this.parseYield(e);
    this.exprAllowed = false;
  }
  var i = false, l = -1, d = -1, n = -1;
  t ? (l = t.parenthesizedAssign, d = t.trailingComma, n = t.doubleProto, t.parenthesizedAssign = t.trailingComma = -1) : (t = new rl(), i = true);
  var P = this.start, x = this.startLoc;
  (this.type === c.parenL || this.type === c.name) && (this.potentialArrowAt = this.start, this.potentialArrowInForAwait = e === "await");
  var T = this.parseMaybeConditional(e, t);
  if (a && (T = a.call(this, T, P, x)), this.type.isAssign) {
    var b = this.startNodeAt(P, x);
    return b.operator = this.value, this.type === c.eq && (T = this.toAssignable(T, false, t)), i || (t.parenthesizedAssign = t.trailingComma = t.doubleProto = -1), t.shorthandAssign >= T.start && (t.shorthandAssign = -1), this.type === c.eq ? this.checkLValPattern(T) : this.checkLValSimple(T), b.left = T, this.next(), b.right = this.parseMaybeAssign(e), n > -1 && (t.doubleProto = n), this.finishNode(b, "AssignmentExpression");
  } else i && this.checkExpressionErrors(t, true);
  return l > -1 && (t.parenthesizedAssign = l), d > -1 && (t.trailingComma = d), T;
};
ce.parseMaybeConditional = function(e, t) {
  var a = this.start, i = this.startLoc, l = this.parseExprOps(e, t);
  if (this.checkExpressionErrors(t)) return l;
  if (this.eat(c.question)) {
    var d = this.startNodeAt(a, i);
    return d.test = l, d.consequent = this.parseMaybeAssign(), this.expect(c.colon), d.alternate = this.parseMaybeAssign(e), this.finishNode(d, "ConditionalExpression");
  }
  return l;
};
ce.parseExprOps = function(e, t) {
  var a = this.start, i = this.startLoc, l = this.parseMaybeUnary(t, false, false, e);
  return this.checkExpressionErrors(t) || l.start === a && l.type === "ArrowFunctionExpression" ? l : this.parseExprOp(l, a, i, -1, e);
};
ce.parseExprOp = function(e, t, a, i, l) {
  var d = this.type.binop;
  if (d != null && (!l || this.type !== c._in) && d > i) {
    var n = this.type === c.logicalOR || this.type === c.logicalAND, P = this.type === c.coalesce;
    P && (d = c.logicalAND.binop);
    var x = this.value;
    this.next();
    var T = this.start, b = this.startLoc, k = this.parseExprOp(this.parseMaybeUnary(null, false, false, l), T, b, d, l), I = this.buildBinary(t, a, e, k, x, n || P);
    return (n && this.type === c.coalesce || P && (this.type === c.logicalOR || this.type === c.logicalAND)) && this.raiseRecoverable(this.start, "Logical expressions and coalesce expressions cannot be mixed. Wrap either by parentheses"), this.parseExprOp(I, t, a, i, l);
  }
  return e;
};
ce.buildBinary = function(e, t, a, i, l, d) {
  i.type === "PrivateIdentifier" && this.raise(i.start, "Private identifier can only be left side of binary expression");
  var n = this.startNodeAt(e, t);
  return n.left = a, n.operator = l, n.right = i, this.finishNode(n, d ? "LogicalExpression" : "BinaryExpression");
};
ce.parseMaybeUnary = function(e, t, a, i) {
  var l = this.start, d = this.startLoc, n;
  if (this.isContextual("await") && this.canAwait) n = this.parseAwait(i), t = true;
  else if (this.type.prefix) {
    var P = this.startNode(), x = this.type === c.incDec;
    P.operator = this.value, P.prefix = true, this.next(), P.argument = this.parseMaybeUnary(null, true, x, i), this.checkExpressionErrors(e, true), x ? this.checkLValSimple(P.argument) : this.strict && P.operator === "delete" && Vh(P.argument) ? this.raiseRecoverable(P.start, "Deleting local variable in strict mode") : P.operator === "delete" && Rl(P.argument) ? this.raiseRecoverable(P.start, "Private fields can not be deleted") : t = true, n = this.finishNode(P, x ? "UpdateExpression" : "UnaryExpression");
  } else if (!t && this.type === c.privateId) (i || this.privateNameStack.length === 0) && this.options.checkPrivateFields && this.unexpected(), n = this.parsePrivateIdent(), this.type !== c._in && this.unexpected();
  else {
    if (n = this.parseExprSubscripts(e, i), this.checkExpressionErrors(e)) return n;
    for (; this.type.postfix && !this.canInsertSemicolon(); ) {
      var T = this.startNodeAt(l, d);
      T.operator = this.value, T.prefix = false, T.argument = n, this.checkLValSimple(n), this.next(), n = this.finishNode(T, "UpdateExpression");
    }
  }
  if (!a && this.eat(c.starstar)) if (t) this.unexpected(this.lastTokStart);
  else return this.buildBinary(l, d, n, this.parseMaybeUnary(null, false, false, i), "**", false);
  else return n;
};
function Vh(e) {
  return e.type === "Identifier" || e.type === "ParenthesizedExpression" && Vh(e.expression);
}
function Rl(e) {
  return e.type === "MemberExpression" && e.property.type === "PrivateIdentifier" || e.type === "ChainExpression" && Rl(e.expression) || e.type === "ParenthesizedExpression" && Rl(e.expression);
}
ce.parseExprSubscripts = function(e, t) {
  var a = this.start, i = this.startLoc, l = this.parseExprAtom(e, t);
  if (l.type === "ArrowFunctionExpression" && this.input.slice(this.lastTokStart, this.lastTokEnd) !== ")") return l;
  var d = this.parseSubscripts(l, a, i, false, t);
  return e && d.type === "MemberExpression" && (e.parenthesizedAssign >= d.start && (e.parenthesizedAssign = -1), e.parenthesizedBind >= d.start && (e.parenthesizedBind = -1), e.trailingComma >= d.start && (e.trailingComma = -1)), d;
};
ce.parseSubscripts = function(e, t, a, i, l) {
  for (var d = this.options.ecmaVersion >= 8 && e.type === "Identifier" && e.name === "async" && this.lastTokEnd === e.end && !this.canInsertSemicolon() && e.end - e.start === 5 && this.potentialArrowAt === e.start, n = false; ; ) {
    var P = this.parseSubscript(e, t, a, i, d, n, l);
    if (P.optional && (n = true), P === e || P.type === "ArrowFunctionExpression") {
      if (n) {
        var x = this.startNodeAt(t, a);
        x.expression = P, P = this.finishNode(x, "ChainExpression");
      }
      return P;
    }
    e = P;
  }
};
ce.shouldParseAsyncArrow = function() {
  return !this.canInsertSemicolon() && this.eat(c.arrow);
};
ce.parseSubscriptAsyncArrow = function(e, t, a, i) {
  return this.parseArrowExpression(this.startNodeAt(e, t), a, true, i);
};
ce.parseSubscript = function(e, t, a, i, l, d, n) {
  var P = this.options.ecmaVersion >= 11, x = P && this.eat(c.questionDot);
  i && x && this.raise(this.lastTokStart, "Optional chaining cannot appear in the callee of new expressions");
  var T = this.eat(c.bracketL);
  if (T || x && this.type !== c.parenL && this.type !== c.backQuote || this.eat(c.dot)) {
    var b = this.startNodeAt(t, a);
    b.object = e, T ? (b.property = this.parseExpression(), this.expect(c.bracketR)) : this.type === c.privateId && e.type !== "Super" ? b.property = this.parsePrivateIdent() : b.property = this.parseIdent(this.options.allowReserved !== "never"), b.computed = !!T, P && (b.optional = x), e = this.finishNode(b, "MemberExpression");
  } else if (!i && this.eat(c.parenL)) {
    var k = new rl(), I = this.yieldPos, E = this.awaitPos, $ = this.awaitIdentPos;
    this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0;
    var D = this.parseExprList(c.parenR, this.options.ecmaVersion >= 8, false, k);
    if (l && !x && this.shouldParseAsyncArrow()) return this.checkPatternErrors(k, false), this.checkYieldAwaitInDefaultParams(), this.awaitIdentPos > 0 && this.raise(this.awaitIdentPos, "Cannot use 'await' as identifier inside an async function"), this.yieldPos = I, this.awaitPos = E, this.awaitIdentPos = $, this.parseSubscriptAsyncArrow(t, a, D, n);
    this.checkExpressionErrors(k, true), this.yieldPos = I || this.yieldPos, this.awaitPos = E || this.awaitPos, this.awaitIdentPos = $ || this.awaitIdentPos;
    var S = this.startNodeAt(t, a);
    S.callee = e, S.arguments = D, P && (S.optional = x), e = this.finishNode(S, "CallExpression");
  } else if (this.type === c.backQuote) {
    (x || d) && this.raise(this.start, "Optional chaining cannot appear in the tag of tagged template expressions");
    var h = this.startNodeAt(t, a);
    h.tag = e, h.quasi = this.parseTemplate({ isTagged: true }), e = this.finishNode(h, "TaggedTemplateExpression");
  }
  return e;
};
ce.parseExprAtom = function(e, t, a) {
  this.type === c.slash && this.readRegexp();
  var i, l = this.potentialArrowAt === this.start;
  switch (this.type) {
    case c._super:
      return this.allowSuper || this.raise(this.start, "'super' keyword outside a method"), i = this.startNode(), this.next(), this.type === c.parenL && !this.allowDirectSuper && this.raise(i.start, "super() call outside constructor of a subclass"), this.type !== c.dot && this.type !== c.bracketL && this.type !== c.parenL && this.unexpected(), this.finishNode(i, "Super");
    case c._this:
      return i = this.startNode(), this.next(), this.finishNode(i, "ThisExpression");
    case c.name:
      var d = this.start, n = this.startLoc, P = this.containsEsc, x = this.parseIdent(false);
      if (this.options.ecmaVersion >= 8 && !P && x.name === "async" && !this.canInsertSemicolon() && this.eat(c._function)) return this.overrideContext(Oe.f_expr), this.parseFunction(this.startNodeAt(d, n), 0, false, true, t);
      if (l && !this.canInsertSemicolon()) {
        if (this.eat(c.arrow)) return this.parseArrowExpression(this.startNodeAt(d, n), [x], false, t);
        if (this.options.ecmaVersion >= 8 && x.name === "async" && this.type === c.name && !P && (!this.potentialArrowInForAwait || this.value !== "of" || this.containsEsc)) return x = this.parseIdent(false), (this.canInsertSemicolon() || !this.eat(c.arrow)) && this.unexpected(), this.parseArrowExpression(this.startNodeAt(d, n), [x], true, t);
      }
      return x;
    case c.regexp:
      var T = this.value;
      return i = this.parseLiteral(T.value), i.regex = { pattern: T.pattern, flags: T.flags }, i;
    case c.num:
    case c.string:
      return this.parseLiteral(this.value);
    case c._null:
    case c._true:
    case c._false:
      return i = this.startNode(), i.value = this.type === c._null ? null : this.type === c._true, i.raw = this.type.keyword, this.next(), this.finishNode(i, "Literal");
    case c.parenL:
      var b = this.start, k = this.parseParenAndDistinguishExpression(l, t);
      return e && (e.parenthesizedAssign < 0 && !this.isSimpleAssignTarget(k) && (e.parenthesizedAssign = b), e.parenthesizedBind < 0 && (e.parenthesizedBind = b)), k;
    case c.bracketL:
      return i = this.startNode(), this.next(), i.elements = this.parseExprList(c.bracketR, true, true, e), this.finishNode(i, "ArrayExpression");
    case c.braceL:
      return this.overrideContext(Oe.b_expr), this.parseObj(false, e);
    case c._function:
      return i = this.startNode(), this.next(), this.parseFunction(i, 0);
    case c._class:
      return this.parseClass(this.startNode(), false);
    case c._new:
      return this.parseNew();
    case c.backQuote:
      return this.parseTemplate();
    case c._import:
      return this.options.ecmaVersion >= 11 ? this.parseExprImport(a) : this.unexpected();
    default:
      return this.parseExprAtomDefault();
  }
};
ce.parseExprAtomDefault = function() {
  this.unexpected();
};
ce.parseExprImport = function(e) {
  var t = this.startNode();
  if (this.containsEsc && this.raiseRecoverable(this.start, "Escape sequence in keyword import"), this.next(), this.type === c.parenL && !e) return this.parseDynamicImport(t);
  if (this.type === c.dot) {
    var a = this.startNodeAt(t.start, t.loc && t.loc.start);
    return a.name = "import", t.meta = this.finishNode(a, "Identifier"), this.parseImportMeta(t);
  } else this.unexpected();
};
ce.parseDynamicImport = function(e) {
  if (this.next(), e.source = this.parseMaybeAssign(), this.options.ecmaVersion >= 16) this.eat(c.parenR) ? e.options = null : (this.expect(c.comma), this.afterTrailingComma(c.parenR) ? e.options = null : (e.options = this.parseMaybeAssign(), this.eat(c.parenR) || (this.expect(c.comma), this.afterTrailingComma(c.parenR) || this.unexpected())));
  else if (!this.eat(c.parenR)) {
    var t = this.start;
    this.eat(c.comma) && this.eat(c.parenR) ? this.raiseRecoverable(t, "Trailing comma is not allowed in import()") : this.unexpected(t);
  }
  return this.finishNode(e, "ImportExpression");
};
ce.parseImportMeta = function(e) {
  this.next();
  var t = this.containsEsc;
  return e.property = this.parseIdent(true), e.property.name !== "meta" && this.raiseRecoverable(e.property.start, "The only valid meta property for import is 'import.meta'"), t && this.raiseRecoverable(e.start, "'import.meta' must not contain escaped characters"), this.options.sourceType !== "module" && !this.options.allowImportExportEverywhere && this.raiseRecoverable(e.start, "Cannot use 'import.meta' outside a module"), this.finishNode(e, "MetaProperty");
};
ce.parseLiteral = function(e) {
  var t = this.startNode();
  return t.value = e, t.raw = this.input.slice(this.start, this.end), t.raw.charCodeAt(t.raw.length - 1) === 110 && (t.bigint = t.raw.slice(0, -1).replace(/_/g, "")), this.next(), this.finishNode(t, "Literal");
};
ce.parseParenExpression = function() {
  this.expect(c.parenL);
  var e = this.parseExpression();
  return this.expect(c.parenR), e;
};
ce.shouldParseArrow = function(e) {
  return !this.canInsertSemicolon();
};
ce.parseParenAndDistinguishExpression = function(e, t) {
  var a = this.start, i = this.startLoc, l, d = this.options.ecmaVersion >= 8;
  if (this.options.ecmaVersion >= 6) {
    this.next();
    var n = this.start, P = this.startLoc, x = [], T = true, b = false, k = new rl(), I = this.yieldPos, E = this.awaitPos, $;
    for (this.yieldPos = 0, this.awaitPos = 0; this.type !== c.parenR; ) if (T ? T = false : this.expect(c.comma), d && this.afterTrailingComma(c.parenR, true)) {
      b = true;
      break;
    } else if (this.type === c.ellipsis) {
      $ = this.start, x.push(this.parseParenItem(this.parseRestBinding())), this.type === c.comma && this.raiseRecoverable(this.start, "Comma is not permitted after the rest element");
      break;
    } else x.push(this.parseMaybeAssign(false, k, this.parseParenItem));
    var D = this.lastTokEnd, S = this.lastTokEndLoc;
    if (this.expect(c.parenR), e && this.shouldParseArrow(x) && this.eat(c.arrow)) return this.checkPatternErrors(k, false), this.checkYieldAwaitInDefaultParams(), this.yieldPos = I, this.awaitPos = E, this.parseParenArrowList(a, i, x, t);
    (!x.length || b) && this.unexpected(this.lastTokStart), $ && this.unexpected($), this.checkExpressionErrors(k, true), this.yieldPos = I || this.yieldPos, this.awaitPos = E || this.awaitPos, x.length > 1 ? (l = this.startNodeAt(n, P), l.expressions = x, this.finishNodeAt(l, "SequenceExpression", D, S)) : l = x[0];
  } else l = this.parseParenExpression();
  if (this.options.preserveParens) {
    var h = this.startNodeAt(a, i);
    return h.expression = l, this.finishNode(h, "ParenthesizedExpression");
  } else return l;
};
ce.parseParenItem = function(e) {
  return e;
};
ce.parseParenArrowList = function(e, t, a, i) {
  return this.parseArrowExpression(this.startNodeAt(e, t), a, false, i);
};
var p1 = [];
ce.parseNew = function() {
  this.containsEsc && this.raiseRecoverable(this.start, "Escape sequence in keyword new");
  var e = this.startNode();
  if (this.next(), this.options.ecmaVersion >= 6 && this.type === c.dot) {
    var t = this.startNodeAt(e.start, e.loc && e.loc.start);
    t.name = "new", e.meta = this.finishNode(t, "Identifier"), this.next();
    var a = this.containsEsc;
    return e.property = this.parseIdent(true), e.property.name !== "target" && this.raiseRecoverable(e.property.start, "The only valid meta property for new is 'new.target'"), a && this.raiseRecoverable(e.start, "'new.target' must not contain escaped characters"), this.allowNewDotTarget || this.raiseRecoverable(e.start, "'new.target' can only be used in functions and class static block"), this.finishNode(e, "MetaProperty");
  }
  var i = this.start, l = this.startLoc;
  return e.callee = this.parseSubscripts(this.parseExprAtom(null, false, true), i, l, true, false), this.eat(c.parenL) ? e.arguments = this.parseExprList(c.parenR, this.options.ecmaVersion >= 8, false) : e.arguments = p1, this.finishNode(e, "NewExpression");
};
ce.parseTemplateElement = function(e) {
  var t = e.isTagged, a = this.startNode();
  return this.type === c.invalidTemplate ? (t || this.raiseRecoverable(this.start, "Bad escape sequence in untagged template literal"), a.value = { raw: this.value.replace(/\r\n?/g, `
`), cooked: null }) : a.value = { raw: this.input.slice(this.start, this.end).replace(/\r\n?/g, `
`), cooked: this.value }, this.next(), a.tail = this.type === c.backQuote, this.finishNode(a, "TemplateElement");
};
ce.parseTemplate = function(e) {
  e === void 0 && (e = {});
  var t = e.isTagged;
  t === void 0 && (t = false);
  var a = this.startNode();
  this.next(), a.expressions = [];
  var i = this.parseTemplateElement({ isTagged: t });
  for (a.quasis = [i]; !i.tail; ) this.type === c.eof && this.raise(this.pos, "Unterminated template literal"), this.expect(c.dollarBraceL), a.expressions.push(this.parseExpression()), this.expect(c.braceR), a.quasis.push(i = this.parseTemplateElement({ isTagged: t }));
  return this.next(), this.finishNode(a, "TemplateLiteral");
};
ce.isAsyncProp = function(e) {
  return !e.computed && e.key.type === "Identifier" && e.key.name === "async" && (this.type === c.name || this.type === c.num || this.type === c.string || this.type === c.bracketL || this.type.keyword || this.options.ecmaVersion >= 9 && this.type === c.star) && !ut.test(this.input.slice(this.lastTokEnd, this.start));
};
ce.parseObj = function(e, t) {
  var a = this.startNode(), i = true, l = {};
  for (a.properties = [], this.next(); !this.eat(c.braceR); ) {
    if (i) i = false;
    else if (this.expect(c.comma), this.options.ecmaVersion >= 5 && this.afterTrailingComma(c.braceR)) break;
    var d = this.parseProperty(e, t);
    e || this.checkPropClash(d, l, t), a.properties.push(d);
  }
  return this.finishNode(a, e ? "ObjectPattern" : "ObjectExpression");
};
ce.parseProperty = function(e, t) {
  var a = this.startNode(), i, l, d, n;
  if (this.options.ecmaVersion >= 9 && this.eat(c.ellipsis)) return e ? (a.argument = this.parseIdent(false), this.type === c.comma && this.raiseRecoverable(this.start, "Comma is not permitted after the rest element"), this.finishNode(a, "RestElement")) : (a.argument = this.parseMaybeAssign(false, t), this.type === c.comma && t && t.trailingComma < 0 && (t.trailingComma = this.start), this.finishNode(a, "SpreadElement"));
  this.options.ecmaVersion >= 6 && (a.method = false, a.shorthand = false, (e || t) && (d = this.start, n = this.startLoc), e || (i = this.eat(c.star)));
  var P = this.containsEsc;
  return this.parsePropertyName(a), !e && !P && this.options.ecmaVersion >= 8 && !i && this.isAsyncProp(a) ? (l = true, i = this.options.ecmaVersion >= 9 && this.eat(c.star), this.parsePropertyName(a)) : l = false, this.parsePropertyValue(a, e, i, l, d, n, t, P), this.finishNode(a, "Property");
};
ce.parseGetterSetter = function(e) {
  var t = e.key.name;
  this.parsePropertyName(e), e.value = this.parseMethod(false), e.kind = t;
  var a = e.kind === "get" ? 0 : 1;
  if (e.value.params.length !== a) {
    var i = e.value.start;
    e.kind === "get" ? this.raiseRecoverable(i, "getter should have no params") : this.raiseRecoverable(i, "setter should have exactly one param");
  } else e.kind === "set" && e.value.params[0].type === "RestElement" && this.raiseRecoverable(e.value.params[0].start, "Setter cannot use rest params");
};
ce.parsePropertyValue = function(e, t, a, i, l, d, n, P) {
  (a || i) && this.type === c.colon && this.unexpected(), this.eat(c.colon) ? (e.value = t ? this.parseMaybeDefault(this.start, this.startLoc) : this.parseMaybeAssign(false, n), e.kind = "init") : this.options.ecmaVersion >= 6 && this.type === c.parenL ? (t && this.unexpected(), e.method = true, e.value = this.parseMethod(a, i), e.kind = "init") : !t && !P && this.options.ecmaVersion >= 5 && !e.computed && e.key.type === "Identifier" && (e.key.name === "get" || e.key.name === "set") && this.type !== c.comma && this.type !== c.braceR && this.type !== c.eq ? ((a || i) && this.unexpected(), this.parseGetterSetter(e)) : this.options.ecmaVersion >= 6 && !e.computed && e.key.type === "Identifier" ? ((a || i) && this.unexpected(), this.checkUnreserved(e.key), e.key.name === "await" && !this.awaitIdentPos && (this.awaitIdentPos = l), t ? e.value = this.parseMaybeDefault(l, d, this.copyNode(e.key)) : this.type === c.eq && n ? (n.shorthandAssign < 0 && (n.shorthandAssign = this.start), e.value = this.parseMaybeDefault(l, d, this.copyNode(e.key))) : e.value = this.copyNode(e.key), e.kind = "init", e.shorthand = true) : this.unexpected();
};
ce.parsePropertyName = function(e) {
  if (this.options.ecmaVersion >= 6) {
    if (this.eat(c.bracketL)) return e.computed = true, e.key = this.parseMaybeAssign(), this.expect(c.bracketR), e.key;
    e.computed = false;
  }
  return e.key = this.type === c.num || this.type === c.string ? this.parseExprAtom() : this.parseIdent(this.options.allowReserved !== "never");
};
ce.initFunction = function(e) {
  e.id = null, this.options.ecmaVersion >= 6 && (e.generator = e.expression = false), this.options.ecmaVersion >= 8 && (e.async = false);
};
ce.parseMethod = function(e, t, a) {
  var i = this.startNode(), l = this.yieldPos, d = this.awaitPos, n = this.awaitIdentPos;
  return this.initFunction(i), this.options.ecmaVersion >= 6 && (i.generator = e), this.options.ecmaVersion >= 8 && (i.async = !!t), this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0, this.enterScope(wl(t, i.generator) | el | (a ? Dh : 0)), this.expect(c.parenL), i.params = this.parseBindingList(c.parenR, false, this.options.ecmaVersion >= 8), this.checkYieldAwaitInDefaultParams(), this.parseFunctionBody(i, false, true, false), this.yieldPos = l, this.awaitPos = d, this.awaitIdentPos = n, this.finishNode(i, "FunctionExpression");
};
ce.parseArrowExpression = function(e, t, a, i) {
  var l = this.yieldPos, d = this.awaitPos, n = this.awaitIdentPos;
  return this.enterScope(wl(a, false) | Al), this.initFunction(e), this.options.ecmaVersion >= 8 && (e.async = !!a), this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0, e.params = this.toAssignableList(t, true), this.parseFunctionBody(e, true, false, i), this.yieldPos = l, this.awaitPos = d, this.awaitIdentPos = n, this.finishNode(e, "ArrowFunctionExpression");
};
ce.parseFunctionBody = function(e, t, a, i) {
  var l = t && this.type !== c.braceL, d = this.strict, n = false;
  if (l) e.body = this.parseMaybeAssign(i), e.expression = true, this.checkParams(e, false);
  else {
    var P = this.options.ecmaVersion >= 7 && !this.isSimpleParamList(e.params);
    (!d || P) && (n = this.strictDirective(this.end), n && P && this.raiseRecoverable(e.start, "Illegal 'use strict' directive in function with non-simple parameter list"));
    var x = this.labels;
    this.labels = [], n && (this.strict = true), this.checkParams(e, !d && !n && !t && !a && this.isSimpleParamList(e.params)), this.strict && e.id && this.checkLValSimple(e.id, jh), e.body = this.parseBlock(false, void 0, n && !d), e.expression = false, this.adaptDirectivePrologue(e.body.body), this.labels = x;
  }
  this.exitScope();
};
ce.isSimpleParamList = function(e) {
  for (var t = 0, a = e; t < a.length; t += 1) {
    var i = a[t];
    if (i.type !== "Identifier") return false;
  }
  return true;
};
ce.checkParams = function(e, t) {
  for (var a = /* @__PURE__ */ Object.create(null), i = 0, l = e.params; i < l.length; i += 1) {
    var d = l[i];
    this.checkLValInnerPattern(d, El, t ? null : a);
  }
};
ce.parseExprList = function(e, t, a, i) {
  for (var l = [], d = true; !this.eat(e); ) {
    if (d) d = false;
    else if (this.expect(c.comma), t && this.afterTrailingComma(e)) break;
    var n = void 0;
    a && this.type === c.comma ? n = null : this.type === c.ellipsis ? (n = this.parseSpread(i), i && this.type === c.comma && i.trailingComma < 0 && (i.trailingComma = this.start)) : n = this.parseMaybeAssign(false, i), l.push(n);
  }
  return l;
};
ce.checkUnreserved = function(e) {
  var t = e.start, a = e.end, i = e.name;
  if (this.inGenerator && i === "yield" && this.raiseRecoverable(t, "Cannot use 'yield' as identifier inside a generator"), this.inAsync && i === "await" && this.raiseRecoverable(t, "Cannot use 'await' as identifier inside an async function"), !(this.currentThisScope().flags & tl) && i === "arguments" && this.raiseRecoverable(t, "Cannot use 'arguments' in class field initializer"), this.inClassStaticBlock && (i === "arguments" || i === "await") && this.raise(t, "Cannot use " + i + " in class static initialization block"), this.keywords.test(i) && this.raise(t, "Unexpected keyword '" + i + "'"), !(this.options.ecmaVersion < 6 && this.input.slice(t, a).indexOf("\\") !== -1)) {
    var l = this.strict ? this.reservedWordsStrict : this.reservedWords;
    l.test(i) && (!this.inAsync && i === "await" && this.raiseRecoverable(t, "Cannot use keyword 'await' outside an async function"), this.raiseRecoverable(t, "The keyword '" + i + "' is reserved"));
  }
};
ce.parseIdent = function(e) {
  var t = this.parseIdentNode();
  return this.next(!!e), this.finishNode(t, "Identifier"), e || (this.checkUnreserved(t), t.name === "await" && !this.awaitIdentPos && (this.awaitIdentPos = t.start)), t;
};
ce.parseIdentNode = function() {
  var e = this.startNode();
  return this.type === c.name ? e.name = this.value : this.type.keyword ? (e.name = this.type.keyword, (e.name === "class" || e.name === "function") && (this.lastTokEnd !== this.lastTokStart + 1 || this.input.charCodeAt(this.lastTokStart) !== 46) && this.context.pop(), this.type = c.name) : this.unexpected(), e;
};
ce.parsePrivateIdent = function() {
  var e = this.startNode();
  return this.type === c.privateId ? e.name = this.value : this.unexpected(), this.next(), this.finishNode(e, "PrivateIdentifier"), this.options.checkPrivateFields && (this.privateNameStack.length === 0 ? this.raise(e.start, "Private field '#" + e.name + "' must be declared in an enclosing class") : this.privateNameStack[this.privateNameStack.length - 1].used.push(e)), e;
};
ce.parseYield = function(e) {
  this.yieldPos || (this.yieldPos = this.start);
  var t = this.startNode();
  return this.next(), this.type === c.semi || this.canInsertSemicolon() || this.type !== c.star && !this.type.startsExpr ? (t.delegate = false, t.argument = null) : (t.delegate = this.eat(c.star), t.argument = this.parseMaybeAssign(e)), this.finishNode(t, "YieldExpression");
};
ce.parseAwait = function(e) {
  this.awaitPos || (this.awaitPos = this.start);
  var t = this.startNode();
  return this.next(), t.argument = this.parseMaybeUnary(null, true, false, e), this.finishNode(t, "AwaitExpression");
};
var Qo = Fe.prototype;
Qo.raise = function(e, t) {
  var a = Tl(this.input, e);
  t += " (" + a.line + ":" + a.column + ")", this.sourceFile && (t += " in " + this.sourceFile);
  var i = new SyntaxError(t);
  throw i.pos = e, i.loc = a, i.raisedAt = this.pos, i;
};
Qo.raiseRecoverable = Qo.raise;
Qo.curPosition = function() {
  if (this.options.locations) return new tr(this.curLine, this.pos - this.lineStart);
};
var Vt = Fe.prototype, f1 = function(t) {
  this.flags = t, this.var = [], this.lexical = [], this.functions = [];
};
Vt.enterScope = function(e) {
  this.scopeStack.push(new f1(e));
};
Vt.exitScope = function() {
  this.scopeStack.pop();
};
Vt.treatFunctionsAsVarInScope = function(e) {
  return e.flags & nr || !this.inModule && e.flags & So;
};
Vt.declareName = function(e, t, a) {
  var i = false;
  if (t === Nt) {
    var l = this.currentScope();
    i = l.lexical.indexOf(e) > -1 || l.functions.indexOf(e) > -1 || l.var.indexOf(e) > -1, l.lexical.push(e), this.inModule && l.flags & So && delete this.undefinedExports[e];
  } else if (t === Fh) {
    var d = this.currentScope();
    d.lexical.push(e);
  } else if (t === Bh) {
    var n = this.currentScope();
    this.treatFunctionsAsVar ? i = n.lexical.indexOf(e) > -1 : i = n.lexical.indexOf(e) > -1 || n.var.indexOf(e) > -1, n.functions.push(e);
  } else for (var P = this.scopeStack.length - 1; P >= 0; --P) {
    var x = this.scopeStack[P];
    if (x.lexical.indexOf(e) > -1 && !(x.flags & Nh && x.lexical[0] === e) || !this.treatFunctionsAsVarInScope(x) && x.functions.indexOf(e) > -1) {
      i = true;
      break;
    }
    if (x.var.push(e), this.inModule && x.flags & So && delete this.undefinedExports[e], x.flags & tl) break;
  }
  i && this.raiseRecoverable(a, "Identifier '" + e + "' has already been declared");
};
Vt.checkLocalExport = function(e) {
  this.scopeStack[0].lexical.indexOf(e.name) === -1 && this.scopeStack[0].var.indexOf(e.name) === -1 && (this.undefinedExports[e.name] = e);
};
Vt.currentScope = function() {
  return this.scopeStack[this.scopeStack.length - 1];
};
Vt.currentVarScope = function() {
  for (var e = this.scopeStack.length - 1; ; e--) {
    var t = this.scopeStack[e];
    if (t.flags & (tl | wo | zt)) return t;
  }
};
Vt.currentThisScope = function() {
  for (var e = this.scopeStack.length - 1; ; e--) {
    var t = this.scopeStack[e];
    if (t.flags & (tl | wo | zt) && !(t.flags & Al)) return t;
  }
};
var Eo = function(t, a, i) {
  this.type = "", this.start = a, this.end = 0, t.options.locations && (this.loc = new Ao(t, i)), t.options.directSourceFile && (this.sourceFile = t.options.directSourceFile), t.options.ranges && (this.range = [a, 0]);
}, ko = Fe.prototype;
ko.startNode = function() {
  return new Eo(this, this.start, this.startLoc);
};
ko.startNodeAt = function(e, t) {
  return new Eo(this, e, t);
};
function Hh(e, t, a, i) {
  return e.type = t, e.end = a, this.options.locations && (e.loc.end = i), this.options.ranges && (e.range[1] = a), e;
}
ko.finishNode = function(e, t) {
  return Hh.call(this, e, t, this.lastTokEnd, this.lastTokEndLoc);
};
ko.finishNodeAt = function(e, t, a, i) {
  return Hh.call(this, e, t, a, i);
};
ko.copyNode = function(e) {
  var t = new Eo(this, e.start, this.startLoc);
  for (var a in e) t[a] = e[a];
  return t;
};
var h1 = "Gara Garay Gukh Gurung_Khema Hrkt Katakana_Or_Hiragana Kawi Kirat_Rai Krai Nag_Mundari Nagm Ol_Onal Onao Sunu Sunuwar Todhri Todr Tulu_Tigalari Tutg Unknown Zzzz", Uh = "ASCII ASCII_Hex_Digit AHex Alphabetic Alpha Any Assigned Bidi_Control Bidi_C Bidi_Mirrored Bidi_M Case_Ignorable CI Cased Changes_When_Casefolded CWCF Changes_When_Casemapped CWCM Changes_When_Lowercased CWL Changes_When_NFKC_Casefolded CWKCF Changes_When_Titlecased CWT Changes_When_Uppercased CWU Dash Default_Ignorable_Code_Point DI Deprecated Dep Diacritic Dia Emoji Emoji_Component Emoji_Modifier Emoji_Modifier_Base Emoji_Presentation Extender Ext Grapheme_Base Gr_Base Grapheme_Extend Gr_Ext Hex_Digit Hex IDS_Binary_Operator IDSB IDS_Trinary_Operator IDST ID_Continue IDC ID_Start IDS Ideographic Ideo Join_Control Join_C Logical_Order_Exception LOE Lowercase Lower Math Noncharacter_Code_Point NChar Pattern_Syntax Pat_Syn Pattern_White_Space Pat_WS Quotation_Mark QMark Radical Regional_Indicator RI Sentence_Terminal STerm Soft_Dotted SD Terminal_Punctuation Term Unified_Ideograph UIdeo Uppercase Upper Variation_Selector VS White_Space space XID_Continue XIDC XID_Start XIDS", Gh = Uh + " Extended_Pictographic", Wh = Gh, zh = Wh + " EBase EComp EMod EPres ExtPict", Kh = zh, m1 = Kh, v1 = { 9: Uh, 10: Gh, 11: Wh, 12: zh, 13: Kh, 14: m1 }, b1 = "Basic_Emoji Emoji_Keycap_Sequence RGI_Emoji_Modifier_Sequence RGI_Emoji_Flag_Sequence RGI_Emoji_Tag_Sequence RGI_Emoji_ZWJ_Sequence RGI_Emoji", y1 = { 9: "", 10: "", 11: "", 12: "", 13: "", 14: b1 }, Gl = "Cased_Letter LC Close_Punctuation Pe Connector_Punctuation Pc Control Cc cntrl Currency_Symbol Sc Dash_Punctuation Pd Decimal_Number Nd digit Enclosing_Mark Me Final_Punctuation Pf Format Cf Initial_Punctuation Pi Letter L Letter_Number Nl Line_Separator Zl Lowercase_Letter Ll Mark M Combining_Mark Math_Symbol Sm Modifier_Letter Lm Modifier_Symbol Sk Nonspacing_Mark Mn Number N Open_Punctuation Ps Other C Other_Letter Lo Other_Number No Other_Punctuation Po Other_Symbol So Paragraph_Separator Zp Private_Use Co Punctuation P punct Separator Z Space_Separator Zs Spacing_Mark Mc Surrogate Cs Symbol S Titlecase_Letter Lt Unassigned Cn Uppercase_Letter Lu", Xh = "Adlam Adlm Ahom Anatolian_Hieroglyphs Hluw Arabic Arab Armenian Armn Avestan Avst Balinese Bali Bamum Bamu Bassa_Vah Bass Batak Batk Bengali Beng Bhaiksuki Bhks Bopomofo Bopo Brahmi Brah Braille Brai Buginese Bugi Buhid Buhd Canadian_Aboriginal Cans Carian Cari Caucasian_Albanian Aghb Chakma Cakm Cham Cham Cherokee Cher Common Zyyy Coptic Copt Qaac Cuneiform Xsux Cypriot Cprt Cyrillic Cyrl Deseret Dsrt Devanagari Deva Duployan Dupl Egyptian_Hieroglyphs Egyp Elbasan Elba Ethiopic Ethi Georgian Geor Glagolitic Glag Gothic Goth Grantha Gran Greek Grek Gujarati Gujr Gurmukhi Guru Han Hani Hangul Hang Hanunoo Hano Hatran Hatr Hebrew Hebr Hiragana Hira Imperial_Aramaic Armi Inherited Zinh Qaai Inscriptional_Pahlavi Phli Inscriptional_Parthian Prti Javanese Java Kaithi Kthi Kannada Knda Katakana Kana Kayah_Li Kali Kharoshthi Khar Khmer Khmr Khojki Khoj Khudawadi Sind Lao Laoo Latin Latn Lepcha Lepc Limbu Limb Linear_A Lina Linear_B Linb Lisu Lisu Lycian Lyci Lydian Lydi Mahajani Mahj Malayalam Mlym Mandaic Mand Manichaean Mani Marchen Marc Masaram_Gondi Gonm Meetei_Mayek Mtei Mende_Kikakui Mend Meroitic_Cursive Merc Meroitic_Hieroglyphs Mero Miao Plrd Modi Mongolian Mong Mro Mroo Multani Mult Myanmar Mymr Nabataean Nbat New_Tai_Lue Talu Newa Newa Nko Nkoo Nushu Nshu Ogham Ogam Ol_Chiki Olck Old_Hungarian Hung Old_Italic Ital Old_North_Arabian Narb Old_Permic Perm Old_Persian Xpeo Old_South_Arabian Sarb Old_Turkic Orkh Oriya Orya Osage Osge Osmanya Osma Pahawh_Hmong Hmng Palmyrene Palm Pau_Cin_Hau Pauc Phags_Pa Phag Phoenician Phnx Psalter_Pahlavi Phlp Rejang Rjng Runic Runr Samaritan Samr Saurashtra Saur Sharada Shrd Shavian Shaw Siddham Sidd SignWriting Sgnw Sinhala Sinh Sora_Sompeng Sora Soyombo Soyo Sundanese Sund Syloti_Nagri Sylo Syriac Syrc Tagalog Tglg Tagbanwa Tagb Tai_Le Tale Tai_Tham Lana Tai_Viet Tavt Takri Takr Tamil Taml Tangut Tang Telugu Telu Thaana Thaa Thai Thai Tibetan Tibt Tifinagh Tfng Tirhuta Tirh Ugaritic Ugar Vai Vaii Warang_Citi Wara Yi Yiii Zanabazar_Square Zanb", Yh = Xh + " Dogra Dogr Gunjala_Gondi Gong Hanifi_Rohingya Rohg Makasar Maka Medefaidrin Medf Old_Sogdian Sogo Sogdian Sogd", Qh = Yh + " Elymaic Elym Nandinagari Nand Nyiakeng_Puachue_Hmong Hmnp Wancho Wcho", Jh = Qh + " Chorasmian Chrs Diak Dives_Akuru Khitan_Small_Script Kits Yezi Yezidi", Zh = Jh + " Cypro_Minoan Cpmn Old_Uyghur Ougr Tangsa Tnsa Toto Vithkuqi Vith", g1 = Zh + " " + h1, R1 = { 9: Xh, 10: Yh, 11: Qh, 12: Jh, 13: Zh, 14: g1 }, em = {};
function x1(e) {
  var t = em[e] = { binary: Ft(v1[e] + " " + Gl), binaryOfStrings: Ft(y1[e]), nonBinary: { General_Category: Ft(Gl), Script: Ft(R1[e]) } };
  t.nonBinary.Script_Extensions = t.nonBinary.Script, t.nonBinary.gc = t.nonBinary.General_Category, t.nonBinary.sc = t.nonBinary.Script, t.nonBinary.scx = t.nonBinary.Script_Extensions;
}
for (var fl = 0, Wl = [9, 10, 11, 12, 13, 14]; fl < Wl.length; fl += 1) {
  var _1 = Wl[fl];
  x1(_1);
}
var te = Fe.prototype, Jo = function(t, a) {
  this.parent = t, this.base = a || this;
};
Jo.prototype.separatedFrom = function(t) {
  for (var a = this; a; a = a.parent) for (var i = t; i; i = i.parent) if (a.base === i.base && a !== i) return true;
  return false;
};
Jo.prototype.sibling = function() {
  return new Jo(this.parent, this.base);
};
var Tt = function(t) {
  this.parser = t, this.validFlags = "gim" + (t.options.ecmaVersion >= 6 ? "uy" : "") + (t.options.ecmaVersion >= 9 ? "s" : "") + (t.options.ecmaVersion >= 13 ? "d" : "") + (t.options.ecmaVersion >= 15 ? "v" : ""), this.unicodeProperties = em[t.options.ecmaVersion >= 14 ? 14 : t.options.ecmaVersion], this.source = "", this.flags = "", this.start = 0, this.switchU = false, this.switchV = false, this.switchN = false, this.pos = 0, this.lastIntValue = 0, this.lastStringValue = "", this.lastAssertionIsQuantifiable = false, this.numCapturingParens = 0, this.maxBackReference = 0, this.groupNames = /* @__PURE__ */ Object.create(null), this.backReferenceNames = [], this.branchID = null;
};
Tt.prototype.reset = function(t, a, i) {
  var l = i.indexOf("v") !== -1, d = i.indexOf("u") !== -1;
  this.start = t | 0, this.source = a + "", this.flags = i, l && this.parser.options.ecmaVersion >= 15 ? (this.switchU = true, this.switchV = true, this.switchN = true) : (this.switchU = d && this.parser.options.ecmaVersion >= 6, this.switchV = false, this.switchN = d && this.parser.options.ecmaVersion >= 9);
};
Tt.prototype.raise = function(t) {
  this.parser.raiseRecoverable(this.start, "Invalid regular expression: /" + this.source + "/: " + t);
};
Tt.prototype.at = function(t, a) {
  a === void 0 && (a = false);
  var i = this.source, l = i.length;
  if (t >= l) return -1;
  var d = i.charCodeAt(t);
  if (!(a || this.switchU) || d <= 55295 || d >= 57344 || t + 1 >= l) return d;
  var n = i.charCodeAt(t + 1);
  return n >= 56320 && n <= 57343 ? (d << 10) + n - 56613888 : d;
};
Tt.prototype.nextIndex = function(t, a) {
  a === void 0 && (a = false);
  var i = this.source, l = i.length;
  if (t >= l) return l;
  var d = i.charCodeAt(t), n;
  return !(a || this.switchU) || d <= 55295 || d >= 57344 || t + 1 >= l || (n = i.charCodeAt(t + 1)) < 56320 || n > 57343 ? t + 1 : t + 2;
};
Tt.prototype.current = function(t) {
  return t === void 0 && (t = false), this.at(this.pos, t);
};
Tt.prototype.lookahead = function(t) {
  return t === void 0 && (t = false), this.at(this.nextIndex(this.pos, t), t);
};
Tt.prototype.advance = function(t) {
  t === void 0 && (t = false), this.pos = this.nextIndex(this.pos, t);
};
Tt.prototype.eat = function(t, a) {
  return a === void 0 && (a = false), this.current(a) === t ? (this.advance(a), true) : false;
};
Tt.prototype.eatChars = function(t, a) {
  a === void 0 && (a = false);
  for (var i = this.pos, l = 0, d = t; l < d.length; l += 1) {
    var n = d[l], P = this.at(i, a);
    if (P === -1 || P !== n) return false;
    i = this.nextIndex(i, a);
  }
  return this.pos = i, true;
};
te.validateRegExpFlags = function(e) {
  for (var t = e.validFlags, a = e.flags, i = false, l = false, d = 0; d < a.length; d++) {
    var n = a.charAt(d);
    t.indexOf(n) === -1 && this.raise(e.start, "Invalid regular expression flag"), a.indexOf(n, d + 1) > -1 && this.raise(e.start, "Duplicate regular expression flag"), n === "u" && (i = true), n === "v" && (l = true);
  }
  this.options.ecmaVersion >= 15 && i && l && this.raise(e.start, "Invalid regular expression flag");
};
function C1(e) {
  for (var t in e) return true;
  return false;
}
te.validateRegExpPattern = function(e) {
  this.regexp_pattern(e), !e.switchN && this.options.ecmaVersion >= 9 && C1(e.groupNames) && (e.switchN = true, this.regexp_pattern(e));
};
te.regexp_pattern = function(e) {
  e.pos = 0, e.lastIntValue = 0, e.lastStringValue = "", e.lastAssertionIsQuantifiable = false, e.numCapturingParens = 0, e.maxBackReference = 0, e.groupNames = /* @__PURE__ */ Object.create(null), e.backReferenceNames.length = 0, e.branchID = null, this.regexp_disjunction(e), e.pos !== e.source.length && (e.eat(41) && e.raise("Unmatched ')'"), (e.eat(93) || e.eat(125)) && e.raise("Lone quantifier brackets")), e.maxBackReference > e.numCapturingParens && e.raise("Invalid escape");
  for (var t = 0, a = e.backReferenceNames; t < a.length; t += 1) {
    var i = a[t];
    e.groupNames[i] || e.raise("Invalid named capture referenced");
  }
};
te.regexp_disjunction = function(e) {
  var t = this.options.ecmaVersion >= 16;
  for (t && (e.branchID = new Jo(e.branchID, null)), this.regexp_alternative(e); e.eat(124); ) t && (e.branchID = e.branchID.sibling()), this.regexp_alternative(e);
  t && (e.branchID = e.branchID.parent), this.regexp_eatQuantifier(e, true) && e.raise("Nothing to repeat"), e.eat(123) && e.raise("Lone quantifier brackets");
};
te.regexp_alternative = function(e) {
  for (; e.pos < e.source.length && this.regexp_eatTerm(e); ) ;
};
te.regexp_eatTerm = function(e) {
  return this.regexp_eatAssertion(e) ? (e.lastAssertionIsQuantifiable && this.regexp_eatQuantifier(e) && e.switchU && e.raise("Invalid quantifier"), true) : (e.switchU ? this.regexp_eatAtom(e) : this.regexp_eatExtendedAtom(e)) ? (this.regexp_eatQuantifier(e), true) : false;
};
te.regexp_eatAssertion = function(e) {
  var t = e.pos;
  if (e.lastAssertionIsQuantifiable = false, e.eat(94) || e.eat(36)) return true;
  if (e.eat(92)) {
    if (e.eat(66) || e.eat(98)) return true;
    e.pos = t;
  }
  if (e.eat(40) && e.eat(63)) {
    var a = false;
    if (this.options.ecmaVersion >= 9 && (a = e.eat(60)), e.eat(61) || e.eat(33)) return this.regexp_disjunction(e), e.eat(41) || e.raise("Unterminated group"), e.lastAssertionIsQuantifiable = !a, true;
  }
  return e.pos = t, false;
};
te.regexp_eatQuantifier = function(e, t) {
  return t === void 0 && (t = false), this.regexp_eatQuantifierPrefix(e, t) ? (e.eat(63), true) : false;
};
te.regexp_eatQuantifierPrefix = function(e, t) {
  return e.eat(42) || e.eat(43) || e.eat(63) || this.regexp_eatBracedQuantifier(e, t);
};
te.regexp_eatBracedQuantifier = function(e, t) {
  var a = e.pos;
  if (e.eat(123)) {
    var i = 0, l = -1;
    if (this.regexp_eatDecimalDigits(e) && (i = e.lastIntValue, e.eat(44) && this.regexp_eatDecimalDigits(e) && (l = e.lastIntValue), e.eat(125))) return l !== -1 && l < i && !t && e.raise("numbers out of order in {} quantifier"), true;
    e.switchU && !t && e.raise("Incomplete quantifier"), e.pos = a;
  }
  return false;
};
te.regexp_eatAtom = function(e) {
  return this.regexp_eatPatternCharacters(e) || e.eat(46) || this.regexp_eatReverseSolidusAtomEscape(e) || this.regexp_eatCharacterClass(e) || this.regexp_eatUncapturingGroup(e) || this.regexp_eatCapturingGroup(e);
};
te.regexp_eatReverseSolidusAtomEscape = function(e) {
  var t = e.pos;
  if (e.eat(92)) {
    if (this.regexp_eatAtomEscape(e)) return true;
    e.pos = t;
  }
  return false;
};
te.regexp_eatUncapturingGroup = function(e) {
  var t = e.pos;
  if (e.eat(40)) {
    if (e.eat(63)) {
      if (this.options.ecmaVersion >= 16) {
        var a = this.regexp_eatModifiers(e), i = e.eat(45);
        if (a || i) {
          for (var l = 0; l < a.length; l++) {
            var d = a.charAt(l);
            a.indexOf(d, l + 1) > -1 && e.raise("Duplicate regular expression modifiers");
          }
          if (i) {
            var n = this.regexp_eatModifiers(e);
            !a && !n && e.current() === 58 && e.raise("Invalid regular expression modifiers");
            for (var P = 0; P < n.length; P++) {
              var x = n.charAt(P);
              (n.indexOf(x, P + 1) > -1 || a.indexOf(x) > -1) && e.raise("Duplicate regular expression modifiers");
            }
          }
        }
      }
      if (e.eat(58)) {
        if (this.regexp_disjunction(e), e.eat(41)) return true;
        e.raise("Unterminated group");
      }
    }
    e.pos = t;
  }
  return false;
};
te.regexp_eatCapturingGroup = function(e) {
  if (e.eat(40)) {
    if (this.options.ecmaVersion >= 9 ? this.regexp_groupSpecifier(e) : e.current() === 63 && e.raise("Invalid group"), this.regexp_disjunction(e), e.eat(41)) return e.numCapturingParens += 1, true;
    e.raise("Unterminated group");
  }
  return false;
};
te.regexp_eatModifiers = function(e) {
  for (var t = "", a = 0; (a = e.current()) !== -1 && P1(a); ) t += It(a), e.advance();
  return t;
};
function P1(e) {
  return e === 105 || e === 109 || e === 115;
}
te.regexp_eatExtendedAtom = function(e) {
  return e.eat(46) || this.regexp_eatReverseSolidusAtomEscape(e) || this.regexp_eatCharacterClass(e) || this.regexp_eatUncapturingGroup(e) || this.regexp_eatCapturingGroup(e) || this.regexp_eatInvalidBracedQuantifier(e) || this.regexp_eatExtendedPatternCharacter(e);
};
te.regexp_eatInvalidBracedQuantifier = function(e) {
  return this.regexp_eatBracedQuantifier(e, true) && e.raise("Nothing to repeat"), false;
};
te.regexp_eatSyntaxCharacter = function(e) {
  var t = e.current();
  return tm(t) ? (e.lastIntValue = t, e.advance(), true) : false;
};
function tm(e) {
  return e === 36 || e >= 40 && e <= 43 || e === 46 || e === 63 || e >= 91 && e <= 94 || e >= 123 && e <= 125;
}
te.regexp_eatPatternCharacters = function(e) {
  for (var t = e.pos, a = 0; (a = e.current()) !== -1 && !tm(a); ) e.advance();
  return e.pos !== t;
};
te.regexp_eatExtendedPatternCharacter = function(e) {
  var t = e.current();
  return t !== -1 && t !== 36 && !(t >= 40 && t <= 43) && t !== 46 && t !== 63 && t !== 91 && t !== 94 && t !== 124 ? (e.advance(), true) : false;
};
te.regexp_groupSpecifier = function(e) {
  if (e.eat(63)) {
    this.regexp_eatGroupName(e) || e.raise("Invalid group");
    var t = this.options.ecmaVersion >= 16, a = e.groupNames[e.lastStringValue];
    if (a) if (t) for (var i = 0, l = a; i < l.length; i += 1) {
      var d = l[i];
      d.separatedFrom(e.branchID) || e.raise("Duplicate capture group name");
    }
    else e.raise("Duplicate capture group name");
    t ? (a || (e.groupNames[e.lastStringValue] = [])).push(e.branchID) : e.groupNames[e.lastStringValue] = true;
  }
};
te.regexp_eatGroupName = function(e) {
  if (e.lastStringValue = "", e.eat(60)) {
    if (this.regexp_eatRegExpIdentifierName(e) && e.eat(62)) return true;
    e.raise("Invalid capture group name");
  }
  return false;
};
te.regexp_eatRegExpIdentifierName = function(e) {
  if (e.lastStringValue = "", this.regexp_eatRegExpIdentifierStart(e)) {
    for (e.lastStringValue += It(e.lastIntValue); this.regexp_eatRegExpIdentifierPart(e); ) e.lastStringValue += It(e.lastIntValue);
    return true;
  }
  return false;
};
te.regexp_eatRegExpIdentifierStart = function(e) {
  var t = e.pos, a = this.options.ecmaVersion >= 11, i = e.current(a);
  return e.advance(a), i === 92 && this.regexp_eatRegExpUnicodeEscapeSequence(e, a) && (i = e.lastIntValue), q1(i) ? (e.lastIntValue = i, true) : (e.pos = t, false);
};
function q1(e) {
  return Pt(e, true) || e === 36 || e === 95;
}
te.regexp_eatRegExpIdentifierPart = function(e) {
  var t = e.pos, a = this.options.ecmaVersion >= 11, i = e.current(a);
  return e.advance(a), i === 92 && this.regexp_eatRegExpUnicodeEscapeSequence(e, a) && (i = e.lastIntValue), T1(i) ? (e.lastIntValue = i, true) : (e.pos = t, false);
};
function T1(e) {
  return Ut(e, true) || e === 36 || e === 95 || e === 8204 || e === 8205;
}
te.regexp_eatAtomEscape = function(e) {
  return this.regexp_eatBackReference(e) || this.regexp_eatCharacterClassEscape(e) || this.regexp_eatCharacterEscape(e) || e.switchN && this.regexp_eatKGroupName(e) ? true : (e.switchU && (e.current() === 99 && e.raise("Invalid unicode escape"), e.raise("Invalid escape")), false);
};
te.regexp_eatBackReference = function(e) {
  var t = e.pos;
  if (this.regexp_eatDecimalEscape(e)) {
    var a = e.lastIntValue;
    if (e.switchU) return a > e.maxBackReference && (e.maxBackReference = a), true;
    if (a <= e.numCapturingParens) return true;
    e.pos = t;
  }
  return false;
};
te.regexp_eatKGroupName = function(e) {
  if (e.eat(107)) {
    if (this.regexp_eatGroupName(e)) return e.backReferenceNames.push(e.lastStringValue), true;
    e.raise("Invalid named reference");
  }
  return false;
};
te.regexp_eatCharacterEscape = function(e) {
  return this.regexp_eatControlEscape(e) || this.regexp_eatCControlLetter(e) || this.regexp_eatZero(e) || this.regexp_eatHexEscapeSequence(e) || this.regexp_eatRegExpUnicodeEscapeSequence(e, false) || !e.switchU && this.regexp_eatLegacyOctalEscapeSequence(e) || this.regexp_eatIdentityEscape(e);
};
te.regexp_eatCControlLetter = function(e) {
  var t = e.pos;
  if (e.eat(99)) {
    if (this.regexp_eatControlLetter(e)) return true;
    e.pos = t;
  }
  return false;
};
te.regexp_eatZero = function(e) {
  return e.current() === 48 && !al(e.lookahead()) ? (e.lastIntValue = 0, e.advance(), true) : false;
};
te.regexp_eatControlEscape = function(e) {
  var t = e.current();
  return t === 116 ? (e.lastIntValue = 9, e.advance(), true) : t === 110 ? (e.lastIntValue = 10, e.advance(), true) : t === 118 ? (e.lastIntValue = 11, e.advance(), true) : t === 102 ? (e.lastIntValue = 12, e.advance(), true) : t === 114 ? (e.lastIntValue = 13, e.advance(), true) : false;
};
te.regexp_eatControlLetter = function(e) {
  var t = e.current();
  return rm(t) ? (e.lastIntValue = t % 32, e.advance(), true) : false;
};
function rm(e) {
  return e >= 65 && e <= 90 || e >= 97 && e <= 122;
}
te.regexp_eatRegExpUnicodeEscapeSequence = function(e, t) {
  t === void 0 && (t = false);
  var a = e.pos, i = t || e.switchU;
  if (e.eat(117)) {
    if (this.regexp_eatFixedHexDigits(e, 4)) {
      var l = e.lastIntValue;
      if (i && l >= 55296 && l <= 56319) {
        var d = e.pos;
        if (e.eat(92) && e.eat(117) && this.regexp_eatFixedHexDigits(e, 4)) {
          var n = e.lastIntValue;
          if (n >= 56320 && n <= 57343) return e.lastIntValue = (l - 55296) * 1024 + (n - 56320) + 65536, true;
        }
        e.pos = d, e.lastIntValue = l;
      }
      return true;
    }
    if (i && e.eat(123) && this.regexp_eatHexDigits(e) && e.eat(125) && S1(e.lastIntValue)) return true;
    i && e.raise("Invalid unicode escape"), e.pos = a;
  }
  return false;
};
function S1(e) {
  return e >= 0 && e <= 1114111;
}
te.regexp_eatIdentityEscape = function(e) {
  if (e.switchU) return this.regexp_eatSyntaxCharacter(e) ? true : e.eat(47) ? (e.lastIntValue = 47, true) : false;
  var t = e.current();
  return t !== 99 && (!e.switchN || t !== 107) ? (e.lastIntValue = t, e.advance(), true) : false;
};
te.regexp_eatDecimalEscape = function(e) {
  e.lastIntValue = 0;
  var t = e.current();
  if (t >= 49 && t <= 57) {
    do
      e.lastIntValue = 10 * e.lastIntValue + (t - 48), e.advance();
    while ((t = e.current()) >= 48 && t <= 57);
    return true;
  }
  return false;
};
var am = 0, Mt = 1, bt = 2;
te.regexp_eatCharacterClassEscape = function(e) {
  var t = e.current();
  if (A1(t)) return e.lastIntValue = -1, e.advance(), Mt;
  var a = false;
  if (e.switchU && this.options.ecmaVersion >= 9 && ((a = t === 80) || t === 112)) {
    e.lastIntValue = -1, e.advance();
    var i;
    if (e.eat(123) && (i = this.regexp_eatUnicodePropertyValueExpression(e)) && e.eat(125)) return a && i === bt && e.raise("Invalid property name"), i;
    e.raise("Invalid property name");
  }
  return am;
};
function A1(e) {
  return e === 100 || e === 68 || e === 115 || e === 83 || e === 119 || e === 87;
}
te.regexp_eatUnicodePropertyValueExpression = function(e) {
  var t = e.pos;
  if (this.regexp_eatUnicodePropertyName(e) && e.eat(61)) {
    var a = e.lastStringValue;
    if (this.regexp_eatUnicodePropertyValue(e)) {
      var i = e.lastStringValue;
      return this.regexp_validateUnicodePropertyNameAndValue(e, a, i), Mt;
    }
  }
  if (e.pos = t, this.regexp_eatLoneUnicodePropertyNameOrValue(e)) {
    var l = e.lastStringValue;
    return this.regexp_validateUnicodePropertyNameOrValue(e, l);
  }
  return am;
};
te.regexp_validateUnicodePropertyNameAndValue = function(e, t, a) {
  sr(e.unicodeProperties.nonBinary, t) || e.raise("Invalid property name"), e.unicodeProperties.nonBinary[t].test(a) || e.raise("Invalid property value");
};
te.regexp_validateUnicodePropertyNameOrValue = function(e, t) {
  if (e.unicodeProperties.binary.test(t)) return Mt;
  if (e.switchV && e.unicodeProperties.binaryOfStrings.test(t)) return bt;
  e.raise("Invalid property name");
};
te.regexp_eatUnicodePropertyName = function(e) {
  var t = 0;
  for (e.lastStringValue = ""; im(t = e.current()); ) e.lastStringValue += It(t), e.advance();
  return e.lastStringValue !== "";
};
function im(e) {
  return rm(e) || e === 95;
}
te.regexp_eatUnicodePropertyValue = function(e) {
  var t = 0;
  for (e.lastStringValue = ""; w1(t = e.current()); ) e.lastStringValue += It(t), e.advance();
  return e.lastStringValue !== "";
};
function w1(e) {
  return im(e) || al(e);
}
te.regexp_eatLoneUnicodePropertyNameOrValue = function(e) {
  return this.regexp_eatUnicodePropertyValue(e);
};
te.regexp_eatCharacterClass = function(e) {
  if (e.eat(91)) {
    var t = e.eat(94), a = this.regexp_classContents(e);
    return e.eat(93) || e.raise("Unterminated character class"), t && a === bt && e.raise("Negated character class may contain strings"), true;
  }
  return false;
};
te.regexp_classContents = function(e) {
  return e.current() === 93 ? Mt : e.switchV ? this.regexp_classSetExpression(e) : (this.regexp_nonEmptyClassRanges(e), Mt);
};
te.regexp_nonEmptyClassRanges = function(e) {
  for (; this.regexp_eatClassAtom(e); ) {
    var t = e.lastIntValue;
    if (e.eat(45) && this.regexp_eatClassAtom(e)) {
      var a = e.lastIntValue;
      e.switchU && (t === -1 || a === -1) && e.raise("Invalid character class"), t !== -1 && a !== -1 && t > a && e.raise("Range out of order in character class");
    }
  }
};
te.regexp_eatClassAtom = function(e) {
  var t = e.pos;
  if (e.eat(92)) {
    if (this.regexp_eatClassEscape(e)) return true;
    if (e.switchU) {
      var a = e.current();
      (a === 99 || om(a)) && e.raise("Invalid class escape"), e.raise("Invalid escape");
    }
    e.pos = t;
  }
  var i = e.current();
  return i !== 93 ? (e.lastIntValue = i, e.advance(), true) : false;
};
te.regexp_eatClassEscape = function(e) {
  var t = e.pos;
  if (e.eat(98)) return e.lastIntValue = 8, true;
  if (e.switchU && e.eat(45)) return e.lastIntValue = 45, true;
  if (!e.switchU && e.eat(99)) {
    if (this.regexp_eatClassControlLetter(e)) return true;
    e.pos = t;
  }
  return this.regexp_eatCharacterClassEscape(e) || this.regexp_eatCharacterEscape(e);
};
te.regexp_classSetExpression = function(e) {
  var t = Mt, a;
  if (!this.regexp_eatClassSetRange(e)) if (a = this.regexp_eatClassSetOperand(e)) {
    a === bt && (t = bt);
    for (var i = e.pos; e.eatChars([38, 38]); ) {
      if (e.current() !== 38 && (a = this.regexp_eatClassSetOperand(e))) {
        a !== bt && (t = Mt);
        continue;
      }
      e.raise("Invalid character in character class");
    }
    if (i !== e.pos) return t;
    for (; e.eatChars([45, 45]); ) this.regexp_eatClassSetOperand(e) || e.raise("Invalid character in character class");
    if (i !== e.pos) return t;
  } else e.raise("Invalid character in character class");
  for (; ; ) if (!this.regexp_eatClassSetRange(e)) {
    if (a = this.regexp_eatClassSetOperand(e), !a) return t;
    a === bt && (t = bt);
  }
};
te.regexp_eatClassSetRange = function(e) {
  var t = e.pos;
  if (this.regexp_eatClassSetCharacter(e)) {
    var a = e.lastIntValue;
    if (e.eat(45) && this.regexp_eatClassSetCharacter(e)) {
      var i = e.lastIntValue;
      return a !== -1 && i !== -1 && a > i && e.raise("Range out of order in character class"), true;
    }
    e.pos = t;
  }
  return false;
};
te.regexp_eatClassSetOperand = function(e) {
  return this.regexp_eatClassSetCharacter(e) ? Mt : this.regexp_eatClassStringDisjunction(e) || this.regexp_eatNestedClass(e);
};
te.regexp_eatNestedClass = function(e) {
  var t = e.pos;
  if (e.eat(91)) {
    var a = e.eat(94), i = this.regexp_classContents(e);
    if (e.eat(93)) return a && i === bt && e.raise("Negated character class may contain strings"), i;
    e.pos = t;
  }
  if (e.eat(92)) {
    var l = this.regexp_eatCharacterClassEscape(e);
    if (l) return l;
    e.pos = t;
  }
  return null;
};
te.regexp_eatClassStringDisjunction = function(e) {
  var t = e.pos;
  if (e.eatChars([92, 113])) {
    if (e.eat(123)) {
      var a = this.regexp_classStringDisjunctionContents(e);
      if (e.eat(125)) return a;
    } else e.raise("Invalid escape");
    e.pos = t;
  }
  return null;
};
te.regexp_classStringDisjunctionContents = function(e) {
  for (var t = this.regexp_classString(e); e.eat(124); ) this.regexp_classString(e) === bt && (t = bt);
  return t;
};
te.regexp_classString = function(e) {
  for (var t = 0; this.regexp_eatClassSetCharacter(e); ) t++;
  return t === 1 ? Mt : bt;
};
te.regexp_eatClassSetCharacter = function(e) {
  var t = e.pos;
  if (e.eat(92)) return this.regexp_eatCharacterEscape(e) || this.regexp_eatClassSetReservedPunctuator(e) ? true : e.eat(98) ? (e.lastIntValue = 8, true) : (e.pos = t, false);
  var a = e.current();
  return a < 0 || a === e.lookahead() && E1(a) || k1(a) ? false : (e.advance(), e.lastIntValue = a, true);
};
function E1(e) {
  return e === 33 || e >= 35 && e <= 38 || e >= 42 && e <= 44 || e === 46 || e >= 58 && e <= 64 || e === 94 || e === 96 || e === 126;
}
function k1(e) {
  return e === 40 || e === 41 || e === 45 || e === 47 || e >= 91 && e <= 93 || e >= 123 && e <= 125;
}
te.regexp_eatClassSetReservedPunctuator = function(e) {
  var t = e.current();
  return I1(t) ? (e.lastIntValue = t, e.advance(), true) : false;
};
function I1(e) {
  return e === 33 || e === 35 || e === 37 || e === 38 || e === 44 || e === 45 || e >= 58 && e <= 62 || e === 64 || e === 96 || e === 126;
}
te.regexp_eatClassControlLetter = function(e) {
  var t = e.current();
  return al(t) || t === 95 ? (e.lastIntValue = t % 32, e.advance(), true) : false;
};
te.regexp_eatHexEscapeSequence = function(e) {
  var t = e.pos;
  if (e.eat(120)) {
    if (this.regexp_eatFixedHexDigits(e, 2)) return true;
    e.switchU && e.raise("Invalid escape"), e.pos = t;
  }
  return false;
};
te.regexp_eatDecimalDigits = function(e) {
  var t = e.pos, a = 0;
  for (e.lastIntValue = 0; al(a = e.current()); ) e.lastIntValue = 10 * e.lastIntValue + (a - 48), e.advance();
  return e.pos !== t;
};
function al(e) {
  return e >= 48 && e <= 57;
}
te.regexp_eatHexDigits = function(e) {
  var t = e.pos, a = 0;
  for (e.lastIntValue = 0; sm(a = e.current()); ) e.lastIntValue = 16 * e.lastIntValue + nm(a), e.advance();
  return e.pos !== t;
};
function sm(e) {
  return e >= 48 && e <= 57 || e >= 65 && e <= 70 || e >= 97 && e <= 102;
}
function nm(e) {
  return e >= 65 && e <= 70 ? 10 + (e - 65) : e >= 97 && e <= 102 ? 10 + (e - 97) : e - 48;
}
te.regexp_eatLegacyOctalEscapeSequence = function(e) {
  if (this.regexp_eatOctalDigit(e)) {
    var t = e.lastIntValue;
    if (this.regexp_eatOctalDigit(e)) {
      var a = e.lastIntValue;
      t <= 3 && this.regexp_eatOctalDigit(e) ? e.lastIntValue = t * 64 + a * 8 + e.lastIntValue : e.lastIntValue = t * 8 + a;
    } else e.lastIntValue = t;
    return true;
  }
  return false;
};
te.regexp_eatOctalDigit = function(e) {
  var t = e.current();
  return om(t) ? (e.lastIntValue = t - 48, e.advance(), true) : (e.lastIntValue = 0, false);
};
function om(e) {
  return e >= 48 && e <= 55;
}
te.regexp_eatFixedHexDigits = function(e, t) {
  var a = e.pos;
  e.lastIntValue = 0;
  for (var i = 0; i < t; ++i) {
    var l = e.current();
    if (!sm(l)) return e.pos = a, false;
    e.lastIntValue = 16 * e.lastIntValue + nm(l), e.advance();
  }
  return true;
};
var il = function(t) {
  this.type = t.type, this.value = t.value, this.start = t.start, this.end = t.end, t.options.locations && (this.loc = new Ao(t, t.startLoc, t.endLoc)), t.options.ranges && (this.range = [t.start, t.end]);
}, ge = Fe.prototype;
ge.next = function(e) {
  !e && this.type.keyword && this.containsEsc && this.raiseRecoverable(this.start, "Escape sequence in keyword " + this.type.keyword), this.options.onToken && this.options.onToken(new il(this)), this.lastTokEnd = this.end, this.lastTokStart = this.start, this.lastTokEndLoc = this.endLoc, this.lastTokStartLoc = this.startLoc, this.nextToken();
};
ge.getToken = function() {
  return this.next(), new il(this);
};
typeof Symbol < "u" && (ge[Symbol.iterator] = function() {
  var e = this;
  return { next: function() {
    var t = e.getToken();
    return { done: t.type === c.eof, value: t };
  } };
});
ge.nextToken = function() {
  var e = this.curContext();
  if ((!e || !e.preserveSpace) && this.skipSpace(), this.start = this.pos, this.options.locations && (this.startLoc = this.curPosition()), this.pos >= this.input.length) return this.finishToken(c.eof);
  if (e.override) return e.override(this);
  this.readToken(this.fullCharCodeAtPos());
};
ge.readToken = function(e) {
  return Pt(e, this.options.ecmaVersion >= 6) || e === 92 ? this.readWord() : this.getTokenFromCode(e);
};
ge.fullCharCodeAtPos = function() {
  var e = this.input.charCodeAt(this.pos);
  if (e <= 55295 || e >= 56320) return e;
  var t = this.input.charCodeAt(this.pos + 1);
  return t <= 56319 || t >= 57344 ? e : (e << 10) + t - 56613888;
};
ge.skipBlockComment = function() {
  var e = this.options.onComment && this.curPosition(), t = this.pos, a = this.input.indexOf("*/", this.pos += 2);
  if (a === -1 && this.raise(this.pos - 2, "Unterminated comment"), this.pos = a + 2, this.options.locations) for (var i = void 0, l = t; (i = Mh(this.input, l, this.pos)) > -1; ) ++this.curLine, l = this.lineStart = i;
  this.options.onComment && this.options.onComment(true, this.input.slice(t + 2, a), t, this.pos, e, this.curPosition());
};
ge.skipLineComment = function(e) {
  for (var t = this.pos, a = this.options.onComment && this.curPosition(), i = this.input.charCodeAt(this.pos += e); this.pos < this.input.length && !Wt(i); ) i = this.input.charCodeAt(++this.pos);
  this.options.onComment && this.options.onComment(false, this.input.slice(t + e, this.pos), t, this.pos, a, this.curPosition());
};
ge.skipSpace = function() {
  e: for (; this.pos < this.input.length; ) {
    var e = this.input.charCodeAt(this.pos);
    switch (e) {
      case 32:
      case 160:
        ++this.pos;
        break;
      case 13:
        this.input.charCodeAt(this.pos + 1) === 10 && ++this.pos;
      case 10:
      case 8232:
      case 8233:
        ++this.pos, this.options.locations && (++this.curLine, this.lineStart = this.pos);
        break;
      case 47:
        switch (this.input.charCodeAt(this.pos + 1)) {
          case 42:
            this.skipBlockComment();
            break;
          case 47:
            this.skipLineComment(2);
            break;
          default:
            break e;
        }
        break;
      default:
        if (e > 8 && e < 14 || e >= 5760 && ql.test(String.fromCharCode(e))) ++this.pos;
        else break e;
    }
  }
};
ge.finishToken = function(e, t) {
  this.end = this.pos, this.options.locations && (this.endLoc = this.curPosition());
  var a = this.type;
  this.type = e, this.value = t, this.updateContext(a);
};
ge.readToken_dot = function() {
  var e = this.input.charCodeAt(this.pos + 1);
  if (e >= 48 && e <= 57) return this.readNumber(true);
  var t = this.input.charCodeAt(this.pos + 2);
  return this.options.ecmaVersion >= 6 && e === 46 && t === 46 ? (this.pos += 3, this.finishToken(c.ellipsis)) : (++this.pos, this.finishToken(c.dot));
};
ge.readToken_slash = function() {
  var e = this.input.charCodeAt(this.pos + 1);
  return this.exprAllowed ? (++this.pos, this.readRegexp()) : e === 61 ? this.finishOp(c.assign, 2) : this.finishOp(c.slash, 1);
};
ge.readToken_mult_modulo_exp = function(e) {
  var t = this.input.charCodeAt(this.pos + 1), a = 1, i = e === 42 ? c.star : c.modulo;
  return this.options.ecmaVersion >= 7 && e === 42 && t === 42 && (++a, i = c.starstar, t = this.input.charCodeAt(this.pos + 2)), t === 61 ? this.finishOp(c.assign, a + 1) : this.finishOp(i, a);
};
ge.readToken_pipe_amp = function(e) {
  var t = this.input.charCodeAt(this.pos + 1);
  if (t === e) {
    if (this.options.ecmaVersion >= 12) {
      var a = this.input.charCodeAt(this.pos + 2);
      if (a === 61) return this.finishOp(c.assign, 3);
    }
    return this.finishOp(e === 124 ? c.logicalOR : c.logicalAND, 2);
  }
  return t === 61 ? this.finishOp(c.assign, 2) : this.finishOp(e === 124 ? c.bitwiseOR : c.bitwiseAND, 1);
};
ge.readToken_caret = function() {
  var e = this.input.charCodeAt(this.pos + 1);
  return e === 61 ? this.finishOp(c.assign, 2) : this.finishOp(c.bitwiseXOR, 1);
};
ge.readToken_plus_min = function(e) {
  var t = this.input.charCodeAt(this.pos + 1);
  return t === e ? t === 45 && !this.inModule && this.input.charCodeAt(this.pos + 2) === 62 && (this.lastTokEnd === 0 || ut.test(this.input.slice(this.lastTokEnd, this.pos))) ? (this.skipLineComment(3), this.skipSpace(), this.nextToken()) : this.finishOp(c.incDec, 2) : t === 61 ? this.finishOp(c.assign, 2) : this.finishOp(c.plusMin, 1);
};
ge.readToken_lt_gt = function(e) {
  var t = this.input.charCodeAt(this.pos + 1), a = 1;
  return t === e ? (a = e === 62 && this.input.charCodeAt(this.pos + 2) === 62 ? 3 : 2, this.input.charCodeAt(this.pos + a) === 61 ? this.finishOp(c.assign, a + 1) : this.finishOp(c.bitShift, a)) : t === 33 && e === 60 && !this.inModule && this.input.charCodeAt(this.pos + 2) === 45 && this.input.charCodeAt(this.pos + 3) === 45 ? (this.skipLineComment(4), this.skipSpace(), this.nextToken()) : (t === 61 && (a = 2), this.finishOp(c.relational, a));
};
ge.readToken_eq_excl = function(e) {
  var t = this.input.charCodeAt(this.pos + 1);
  return t === 61 ? this.finishOp(c.equality, this.input.charCodeAt(this.pos + 2) === 61 ? 3 : 2) : e === 61 && t === 62 && this.options.ecmaVersion >= 6 ? (this.pos += 2, this.finishToken(c.arrow)) : this.finishOp(e === 61 ? c.eq : c.prefix, 1);
};
ge.readToken_question = function() {
  var e = this.options.ecmaVersion;
  if (e >= 11) {
    var t = this.input.charCodeAt(this.pos + 1);
    if (t === 46) {
      var a = this.input.charCodeAt(this.pos + 2);
      if (a < 48 || a > 57) return this.finishOp(c.questionDot, 2);
    }
    if (t === 63) {
      if (e >= 12) {
        var i = this.input.charCodeAt(this.pos + 2);
        if (i === 61) return this.finishOp(c.assign, 3);
      }
      return this.finishOp(c.coalesce, 2);
    }
  }
  return this.finishOp(c.question, 1);
};
ge.readToken_numberSign = function() {
  var e = this.options.ecmaVersion, t = 35;
  if (e >= 13 && (++this.pos, t = this.fullCharCodeAtPos(), Pt(t, true) || t === 92)) return this.finishToken(c.privateId, this.readWord1());
  this.raise(this.pos, "Unexpected character '" + It(t) + "'");
};
ge.getTokenFromCode = function(e) {
  switch (e) {
    case 46:
      return this.readToken_dot();
    case 40:
      return ++this.pos, this.finishToken(c.parenL);
    case 41:
      return ++this.pos, this.finishToken(c.parenR);
    case 59:
      return ++this.pos, this.finishToken(c.semi);
    case 44:
      return ++this.pos, this.finishToken(c.comma);
    case 91:
      return ++this.pos, this.finishToken(c.bracketL);
    case 93:
      return ++this.pos, this.finishToken(c.bracketR);
    case 123:
      return ++this.pos, this.finishToken(c.braceL);
    case 125:
      return ++this.pos, this.finishToken(c.braceR);
    case 58:
      return ++this.pos, this.finishToken(c.colon);
    case 96:
      if (this.options.ecmaVersion < 6) break;
      return ++this.pos, this.finishToken(c.backQuote);
    case 48:
      var t = this.input.charCodeAt(this.pos + 1);
      if (t === 120 || t === 88) return this.readRadixNumber(16);
      if (this.options.ecmaVersion >= 6) {
        if (t === 111 || t === 79) return this.readRadixNumber(8);
        if (t === 98 || t === 66) return this.readRadixNumber(2);
      }
    case 49:
    case 50:
    case 51:
    case 52:
    case 53:
    case 54:
    case 55:
    case 56:
    case 57:
      return this.readNumber(false);
    case 34:
    case 39:
      return this.readString(e);
    case 47:
      return this.readToken_slash();
    case 37:
    case 42:
      return this.readToken_mult_modulo_exp(e);
    case 124:
    case 38:
      return this.readToken_pipe_amp(e);
    case 94:
      return this.readToken_caret();
    case 43:
    case 45:
      return this.readToken_plus_min(e);
    case 60:
    case 62:
      return this.readToken_lt_gt(e);
    case 61:
    case 33:
      return this.readToken_eq_excl(e);
    case 63:
      return this.readToken_question();
    case 126:
      return this.finishOp(c.prefix, 1);
    case 35:
      return this.readToken_numberSign();
  }
  this.raise(this.pos, "Unexpected character '" + It(e) + "'");
};
ge.finishOp = function(e, t) {
  var a = this.input.slice(this.pos, this.pos + t);
  return this.pos += t, this.finishToken(e, a);
};
ge.readRegexp = function() {
  for (var e, t, a = this.pos; ; ) {
    this.pos >= this.input.length && this.raise(a, "Unterminated regular expression");
    var i = this.input.charAt(this.pos);
    if (ut.test(i) && this.raise(a, "Unterminated regular expression"), e) e = false;
    else {
      if (i === "[") t = true;
      else if (i === "]" && t) t = false;
      else if (i === "/" && !t) break;
      e = i === "\\";
    }
    ++this.pos;
  }
  var l = this.input.slice(a, this.pos);
  ++this.pos;
  var d = this.pos, n = this.readWord1();
  this.containsEsc && this.unexpected(d);
  var P = this.regexpState || (this.regexpState = new Tt(this));
  P.reset(a, l, n), this.validateRegExpFlags(P), this.validateRegExpPattern(P);
  var x = null;
  try {
    x = new RegExp(l, n);
  } catch {
  }
  return this.finishToken(c.regexp, { pattern: l, flags: n, value: x });
};
ge.readInt = function(e, t, a) {
  for (var i = this.options.ecmaVersion >= 12 && t === void 0, l = a && this.input.charCodeAt(this.pos) === 48, d = this.pos, n = 0, P = 0, x = 0, T = t ?? 1 / 0; x < T; ++x, ++this.pos) {
    var b = this.input.charCodeAt(this.pos), k = void 0;
    if (i && b === 95) {
      l && this.raiseRecoverable(this.pos, "Numeric separator is not allowed in legacy octal numeric literals"), P === 95 && this.raiseRecoverable(this.pos, "Numeric separator must be exactly one underscore"), x === 0 && this.raiseRecoverable(this.pos, "Numeric separator is not allowed at the first of digits"), P = b;
      continue;
    }
    if (b >= 97 ? k = b - 97 + 10 : b >= 65 ? k = b - 65 + 10 : b >= 48 && b <= 57 ? k = b - 48 : k = 1 / 0, k >= e) break;
    P = b, n = n * e + k;
  }
  return i && P === 95 && this.raiseRecoverable(this.pos - 1, "Numeric separator is not allowed at the last of digits"), this.pos === d || t != null && this.pos - d !== t ? null : n;
};
function M1(e, t) {
  return t ? parseInt(e, 8) : parseFloat(e.replace(/_/g, ""));
}
function lm(e) {
  return typeof BigInt != "function" ? null : BigInt(e.replace(/_/g, ""));
}
ge.readRadixNumber = function(e) {
  var t = this.pos;
  this.pos += 2;
  var a = this.readInt(e);
  return a == null && this.raise(this.start + 2, "Expected number in radix " + e), this.options.ecmaVersion >= 11 && this.input.charCodeAt(this.pos) === 110 ? (a = lm(this.input.slice(t, this.pos)), ++this.pos) : Pt(this.fullCharCodeAtPos()) && this.raise(this.pos, "Identifier directly after number"), this.finishToken(c.num, a);
};
ge.readNumber = function(e) {
  var t = this.pos;
  !e && this.readInt(10, void 0, true) === null && this.raise(t, "Invalid number");
  var a = this.pos - t >= 2 && this.input.charCodeAt(t) === 48;
  a && this.strict && this.raise(t, "Invalid number");
  var i = this.input.charCodeAt(this.pos);
  if (!a && !e && this.options.ecmaVersion >= 11 && i === 110) {
    var l = lm(this.input.slice(t, this.pos));
    return ++this.pos, Pt(this.fullCharCodeAtPos()) && this.raise(this.pos, "Identifier directly after number"), this.finishToken(c.num, l);
  }
  a && /[89]/.test(this.input.slice(t, this.pos)) && (a = false), i === 46 && !a && (++this.pos, this.readInt(10), i = this.input.charCodeAt(this.pos)), (i === 69 || i === 101) && !a && (i = this.input.charCodeAt(++this.pos), (i === 43 || i === 45) && ++this.pos, this.readInt(10) === null && this.raise(t, "Invalid number")), Pt(this.fullCharCodeAtPos()) && this.raise(this.pos, "Identifier directly after number");
  var d = M1(this.input.slice(t, this.pos), a);
  return this.finishToken(c.num, d);
};
ge.readCodePoint = function() {
  var e = this.input.charCodeAt(this.pos), t;
  if (e === 123) {
    this.options.ecmaVersion < 6 && this.unexpected();
    var a = ++this.pos;
    t = this.readHexChar(this.input.indexOf("}", this.pos) - this.pos), ++this.pos, t > 1114111 && this.invalidStringToken(a, "Code point out of bounds");
  } else t = this.readHexChar(4);
  return t;
};
ge.readString = function(e) {
  for (var t = "", a = ++this.pos; ; ) {
    this.pos >= this.input.length && this.raise(this.start, "Unterminated string constant");
    var i = this.input.charCodeAt(this.pos);
    if (i === e) break;
    i === 92 ? (t += this.input.slice(a, this.pos), t += this.readEscapedChar(false), a = this.pos) : i === 8232 || i === 8233 ? (this.options.ecmaVersion < 10 && this.raise(this.start, "Unterminated string constant"), ++this.pos, this.options.locations && (this.curLine++, this.lineStart = this.pos)) : (Wt(i) && this.raise(this.start, "Unterminated string constant"), ++this.pos);
  }
  return t += this.input.slice(a, this.pos++), this.finishToken(c.string, t);
};
var um = {};
ge.tryReadTemplateToken = function() {
  this.inTemplateElement = true;
  try {
    this.readTmplToken();
  } catch (e) {
    if (e === um) this.readInvalidTemplateToken();
    else throw e;
  }
  this.inTemplateElement = false;
};
ge.invalidStringToken = function(e, t) {
  if (this.inTemplateElement && this.options.ecmaVersion >= 9) throw um;
  this.raise(e, t);
};
ge.readTmplToken = function() {
  for (var e = "", t = this.pos; ; ) {
    this.pos >= this.input.length && this.raise(this.start, "Unterminated template");
    var a = this.input.charCodeAt(this.pos);
    if (a === 96 || a === 36 && this.input.charCodeAt(this.pos + 1) === 123) return this.pos === this.start && (this.type === c.template || this.type === c.invalidTemplate) ? a === 36 ? (this.pos += 2, this.finishToken(c.dollarBraceL)) : (++this.pos, this.finishToken(c.backQuote)) : (e += this.input.slice(t, this.pos), this.finishToken(c.template, e));
    if (a === 92) e += this.input.slice(t, this.pos), e += this.readEscapedChar(true), t = this.pos;
    else if (Wt(a)) {
      switch (e += this.input.slice(t, this.pos), ++this.pos, a) {
        case 13:
          this.input.charCodeAt(this.pos) === 10 && ++this.pos;
        case 10:
          e += `
`;
          break;
        default:
          e += String.fromCharCode(a);
          break;
      }
      this.options.locations && (++this.curLine, this.lineStart = this.pos), t = this.pos;
    } else ++this.pos;
  }
};
ge.readInvalidTemplateToken = function() {
  for (; this.pos < this.input.length; this.pos++) switch (this.input[this.pos]) {
    case "\\":
      ++this.pos;
      break;
    case "$":
      if (this.input[this.pos + 1] !== "{") break;
    case "`":
      return this.finishToken(c.invalidTemplate, this.input.slice(this.start, this.pos));
    case "\r":
      this.input[this.pos + 1] === `
` && ++this.pos;
    case `
`:
    case "\u2028":
    case "\u2029":
      ++this.curLine, this.lineStart = this.pos + 1;
      break;
  }
  this.raise(this.start, "Unterminated template");
};
ge.readEscapedChar = function(e) {
  var t = this.input.charCodeAt(++this.pos);
  switch (++this.pos, t) {
    case 110:
      return `
`;
    case 114:
      return "\r";
    case 120:
      return String.fromCharCode(this.readHexChar(2));
    case 117:
      return It(this.readCodePoint());
    case 116:
      return "	";
    case 98:
      return "\b";
    case 118:
      return "\v";
    case 102:
      return "\f";
    case 13:
      this.input.charCodeAt(this.pos) === 10 && ++this.pos;
    case 10:
      return this.options.locations && (this.lineStart = this.pos, ++this.curLine), "";
    case 56:
    case 57:
      if (this.strict && this.invalidStringToken(this.pos - 1, "Invalid escape sequence"), e) {
        var a = this.pos - 1;
        this.invalidStringToken(a, "Invalid escape sequence in template string");
      }
    default:
      if (t >= 48 && t <= 55) {
        var i = this.input.substr(this.pos - 1, 3).match(/^[0-7]+/)[0], l = parseInt(i, 8);
        return l > 255 && (i = i.slice(0, -1), l = parseInt(i, 8)), this.pos += i.length - 1, t = this.input.charCodeAt(this.pos), (i !== "0" || t === 56 || t === 57) && (this.strict || e) && this.invalidStringToken(this.pos - 1 - i.length, e ? "Octal literal in template string" : "Octal literal in strict mode"), String.fromCharCode(l);
      }
      return Wt(t) ? (this.options.locations && (this.lineStart = this.pos, ++this.curLine), "") : String.fromCharCode(t);
  }
};
ge.readHexChar = function(e) {
  var t = this.pos, a = this.readInt(16, e);
  return a === null && this.invalidStringToken(t, "Bad character escape sequence"), a;
};
ge.readWord1 = function() {
  this.containsEsc = false;
  for (var e = "", t = true, a = this.pos, i = this.options.ecmaVersion >= 6; this.pos < this.input.length; ) {
    var l = this.fullCharCodeAtPos();
    if (Ut(l, i)) this.pos += l <= 65535 ? 1 : 2;
    else if (l === 92) {
      this.containsEsc = true, e += this.input.slice(a, this.pos);
      var d = this.pos;
      this.input.charCodeAt(++this.pos) !== 117 && this.invalidStringToken(this.pos, "Expecting Unicode escape sequence \\uXXXX"), ++this.pos;
      var n = this.readCodePoint();
      (t ? Pt : Ut)(n, i) || this.invalidStringToken(d, "Invalid Unicode escape"), e += It(n), a = this.pos;
    } else break;
    t = false;
  }
  return e + this.input.slice(a, this.pos);
};
ge.readWord = function() {
  var e = this.readWord1(), t = c.name;
  return this.keywords.test(e) && (t = er[e]), this.finishToken(t, e);
};
var cm = "8.14.1";
Fe.acorn = { Parser: Fe, version: cm, defaultOptions: Ko, Position: tr, SourceLocation: Ao, getLineInfo: Tl, Node: Eo, TokenType: me, tokTypes: c, keywordTypes: er, TokContext: tt, tokContexts: Oe, isIdentifierChar: Ut, isIdentifierStart: Pt, Token: il, isNewLine: Wt, lineBreak: ut, lineBreakG: Ih, nonASCIIwhitespace: ql };
function L1(e, t) {
  return Fe.parse(e, t);
}
function O1(e, t, a) {
  return Fe.parseExpressionAt(e, t, a);
}
function N1(e, t) {
  return Fe.tokenizer(e, t);
}
const D1 = Object.freeze(Object.defineProperty({ __proto__: null, Node: Eo, Parser: Fe, Position: tr, SourceLocation: Ao, TokContext: tt, Token: il, TokenType: me, defaultOptions: Ko, getLineInfo: Tl, isIdentifierChar: Ut, isIdentifierStart: Pt, isNewLine: Wt, keywordTypes: er, lineBreak: ut, lineBreakG: Ih, nonASCIIwhitespace: ql, parse: L1, parseExpressionAt: O1, tokContexts: Oe, tokTypes: c, tokenizer: N1, version: cm }, Symbol.toStringTag, { value: "Module" }));
var nt = true;
function ot(e, t = {}) {
  return new me("name", t);
}
var B1 = /* @__PURE__ */ new WeakMap();
function F1(e) {
  const t = e.Parser.acorn || e;
  let a = B1.get(t);
  if (!a) {
    let i = function(E) {
      return [c.name, c.string, c.num, ...Object.values(er), ...Object.values(T)].includes(E);
    }, l = function(E) {
      return [c.name, ...Object.values(er), ...Object.values(T)].includes(E);
    }, d = function(E) {
      return [...Object.values(T), c.name].includes(E);
    }, n = function(E) {
      return [T.abstract, T.declare, T.enum, T.module, T.namespace, T.interface, T.type].includes(E);
    }, P = function(E) {
      return [T.keyof, T.readonly, T.unique].includes(E);
    }, x = function(E) {
      return E === c.invalidTemplate;
    };
    const T = V1(), b = $1(), k = j1(), I = new RegExp(`^(?:${Object.keys(T).join("|")})$`);
    b.jsxTagStart.updateContext = function() {
      this.context.push(k.tc_expr), this.context.push(k.tc_oTag), this.exprAllowed = false;
    }, b.jsxTagEnd.updateContext = function(E) {
      let $ = this.context.pop();
      $ === k.tc_oTag && E === c.slash || $ === k.tc_cTag ? (this.context.pop(), this.exprAllowed = this.curContext() === k.tc_expr) : this.exprAllowed = true;
    }, a = { tokTypes: { ...T, ...b }, tokContexts: { ...k }, keywordsRegExp: I, tokenIsLiteralPropertyName: i, tokenIsKeywordOrIdentifier: l, tokenIsIdentifier: d, tokenIsTSDeclarationStart: n, tokenIsTSTypeOperator: P, tokenIsTemplate: x };
  }
  return a;
}
function j1() {
  return { tc_oTag: new tt("<tag", false, false), tc_cTag: new tt("</tag", false, false), tc_expr: new tt("<tag>...</tag>", true, true) };
}
function $1() {
  return { at: new me("@"), jsxName: new me("jsxName"), jsxText: new me("jsxText", { beforeExpr: true }), jsxTagStart: new me("jsxTagStart", { startsExpr: true }), jsxTagEnd: new me("jsxTagEnd") };
}
function V1() {
  return { assert: ot("assert", { startsExpr: nt }), asserts: ot("asserts", { startsExpr: nt }), global: ot("global", { startsExpr: nt }), keyof: ot("keyof", { startsExpr: nt }), readonly: ot("readonly", { startsExpr: nt }), unique: ot("unique", { startsExpr: nt }), abstract: ot("abstract", { startsExpr: nt }), declare: ot("declare", { startsExpr: nt }), enum: ot("enum", { startsExpr: nt }), module: ot("module", { startsExpr: nt }), namespace: ot("namespace", { startsExpr: nt }), interface: ot("interface", { startsExpr: nt }), type: ot("type", { startsExpr: nt }) };
}
var H1 = 512, ur = 1024, U1 = /(?:[^\S\n\r\u2028\u2029]|\/\/.*|\/\*.*?\*\/)*/y, zl = new RegExp("(?=(" + U1.source + "))\\1" + /(?=[\n\r\u2028\u2029]|\/\*(?!.*?\*\/)|$)/.source, "y"), cr = class {
  constructor() {
    this.shorthandAssign = this.trailingComma = this.parenthesizedAssign = this.parenthesizedBind = this.doubleProto = -1;
  }
};
function G1(e, t) {
  const a = t.key.name, i = e[a];
  let l = "true";
  return t.type === "MethodDefinition" && (t.kind === "get" || t.kind === "set") && (l = (t.static ? "s" : "i") + t.kind), i === "iget" && l === "iset" || i === "iset" && l === "iget" || i === "sget" && l === "sset" || i === "sset" && l === "sget" ? (e[a] = "true", false) : i ? true : (e[a] = l, false);
}
function Kl(e, t) {
  const { computed: a, key: i } = e;
  return !a && (i.type === "Identifier" && i.name === t || i.type === "Literal" && i.value === t);
}
var ie = { AbstractMethodHasImplementation: ({ methodName: e }) => `Method '${e}' cannot have an implementation because it is marked abstract.`, AbstractPropertyHasInitializer: ({ propertyName: e }) => `Property '${e}' cannot have an initializer because it is marked abstract.`, AccesorCannotDeclareThisParameter: "'get' and 'set' accessors cannot declare 'this' parameters.", AccesorCannotHaveTypeParameters: "An accessor cannot have type parameters.", CannotFindName: ({ name: e }) => `Cannot find name '${e}'.`, ClassMethodHasDeclare: "Class methods cannot have the 'declare' modifier.", ClassMethodHasReadonly: "Class methods cannot have the 'readonly' modifier.", ConstInitiailizerMustBeStringOrNumericLiteralOrLiteralEnumReference: "A 'const' initializer in an ambient context must be a string or numeric literal or literal enum reference.", ConstructorHasTypeParameters: "Type parameters cannot appear on a constructor declaration.", DeclareAccessor: ({ kind: e }) => `'declare' is not allowed in ${e}ters.`, DeclareClassFieldHasInitializer: "Initializers are not allowed in ambient contexts.", DeclareFunctionHasImplementation: "An implementation cannot be declared in ambient contexts.", DuplicateAccessibilityModifier: () => "Accessibility modifier already seen.", DuplicateModifier: ({ modifier: e }) => `Duplicate modifier: '${e}'.`, EmptyHeritageClauseType: ({ token: e }) => `'${e}' list cannot be empty.`, EmptyTypeArguments: "Type argument list cannot be empty.", EmptyTypeParameters: "Type parameter list cannot be empty.", ExpectedAmbientAfterExportDeclare: "'export declare' must be followed by an ambient declaration.", ImportAliasHasImportType: "An import alias can not use 'import type'.", IncompatibleModifiers: ({ modifiers: e }) => `'${e[0]}' modifier cannot be used with '${e[1]}' modifier.`, IndexSignatureHasAbstract: "Index signatures cannot have the 'abstract' modifier.", IndexSignatureHasAccessibility: ({ modifier: e }) => `Index signatures cannot have an accessibility modifier ('${e}').`, IndexSignatureHasDeclare: "Index signatures cannot have the 'declare' modifier.", IndexSignatureHasOverride: "'override' modifier cannot appear on an index signature.", IndexSignatureHasStatic: "Index signatures cannot have the 'static' modifier.", InitializerNotAllowedInAmbientContext: "Initializers are not allowed in ambient contexts.", InvalidModifierOnTypeMember: ({ modifier: e }) => `'${e}' modifier cannot appear on a type member.`, InvalidModifierOnTypeParameter: ({ modifier: e }) => `'${e}' modifier cannot appear on a type parameter.`, InvalidModifierOnTypeParameterPositions: ({ modifier: e }) => `'${e}' modifier can only appear on a type parameter of a class, interface or type alias.`, InvalidModifiersOrder: ({ orderedModifiers: e }) => `'${e[0]}' modifier must precede '${e[1]}' modifier.`, InvalidPropertyAccessAfterInstantiationExpression: "Invalid property access after an instantiation expression. You can either wrap the instantiation expression in parentheses, or delete the type arguments.", InvalidTupleMemberLabel: "Tuple members must be labeled with a simple identifier.", MissingInterfaceName: "'interface' declarations must be followed by an identifier.", MixedLabeledAndUnlabeledElements: "Tuple members must all have names or all not have names.", NonAbstractClassHasAbstractMethod: "Abstract methods can only appear within an abstract class.", NonClassMethodPropertyHasAbstractModifer: "'abstract' modifier can only appear on a class, method, or property declaration.", OptionalTypeBeforeRequired: "A required element cannot follow an optional element.", OverrideNotInSubClass: "This member cannot have an 'override' modifier because its containing class does not extend another class.", PatternIsOptional: "A binding pattern parameter cannot be optional in an implementation signature.", PrivateElementHasAbstract: "Private elements cannot have the 'abstract' modifier.", PrivateElementHasAccessibility: ({ modifier: e }) => `Private elements cannot have an accessibility modifier ('${e}').`, PrivateMethodsHasAccessibility: ({ modifier: e }) => `Private methods cannot have an accessibility modifier ('${e}').`, ReadonlyForMethodSignature: "'readonly' modifier can only appear on a property declaration or index signature.", ReservedArrowTypeParam: "This syntax is reserved in files with the .mts or .cts extension. Add a trailing comma, as in `<T,>() => ...`.", ReservedTypeAssertion: "This syntax is reserved in files with the .mts or .cts extension. Use an `as` expression instead.", SetAccesorCannotHaveOptionalParameter: "A 'set' accessor cannot have an optional parameter.", SetAccesorCannotHaveRestParameter: "A 'set' accessor cannot have rest parameter.", SetAccesorCannotHaveReturnType: "A 'set' accessor cannot have a return type annotation.", SingleTypeParameterWithoutTrailingComma: ({ typeParameterName: e }) => `Single type parameter ${e} should have a trailing comma. Example usage: <${e},>.`, StaticBlockCannotHaveModifier: "Static class blocks cannot have any modifier.", TypeAnnotationAfterAssign: "Type annotations must come before default assignments, e.g. instead of `age = 25: number` use `age: number = 25`.", TypeImportCannotSpecifyDefaultAndNamed: "A type-only import can specify a default import or named bindings, but not both.", TypeModifierIsUsedInTypeExports: "The 'type' modifier cannot be used on a named export when 'export type' is used on its export statement.", TypeModifierIsUsedInTypeImports: "The 'type' modifier cannot be used on a named import when 'import type' is used on its import statement.", UnexpectedParameterModifier: "A parameter property is only allowed in a constructor implementation.", UnexpectedReadonly: "'readonly' type modifier is only permitted on array and tuple literal types.", GenericsEndWithComma: "Trailing comma is not allowed at the end of generics.", UnexpectedTypeAnnotation: "Did not expect a type annotation here.", UnexpectedTypeCastInParameter: "Unexpected type cast in parameter position.", UnsupportedImportTypeArgument: "Argument in a type import must be a string literal.", UnsupportedParameterPropertyKind: "A parameter property may not be declared using a binding pattern.", UnsupportedSignatureParameterKind: ({ type: e }) => `Name in a signature must be an Identifier, ObjectPattern or ArrayPattern, instead got ${e}.`, LetInLexicalBinding: "'let' is not allowed to be used as a name in 'let' or 'const' declarations." }, $o = { UnexpectedLeadingDecorator: "Leading decorators must be attached to a class declaration.", DecoratorConstructor: "Decorators can't be used with a constructor. Did you mean '@dec class { ... }'?", TrailingDecorator: "Decorators must be attached to a class element.", SpreadElementDecorator: "Decorators can't be used with SpreadElement" };
function W1(e, t, a) {
  const { tokTypes: i } = a, { tokTypes: l } = t;
  return class extends e {
    takeDecorators(n) {
      const P = this.decoratorStack[this.decoratorStack.length - 1];
      P.length && (n.decorators = P, this.resetStartLocationFromNode(n, P[0]), this.decoratorStack[this.decoratorStack.length - 1] = []);
    }
    parseDecorators(n) {
      const P = this.decoratorStack[this.decoratorStack.length - 1];
      for (; this.match(l.at); ) {
        const x = this.parseDecorator();
        P.push(x);
      }
      this.match(i._export) ? n || this.unexpected() : this.canHaveLeadingDecorator() || this.raise(this.start, $o.UnexpectedLeadingDecorator);
    }
    parseDecorator() {
      const n = this.startNode();
      this.next(), this.decoratorStack.push([]);
      const P = this.start, x = this.startLoc;
      let T;
      if (this.match(i.parenL)) {
        const b = this.start, k = this.startLoc;
        if (this.next(), T = this.parseExpression(), this.expect(i.parenR), this.options.preserveParens) {
          let I = this.startNodeAt(b, k);
          I.expression = T, T = this.finishNode(I, "ParenthesizedExpression");
        }
      } else for (T = this.parseIdent(false); this.eat(i.dot); ) {
        const b = this.startNodeAt(P, x);
        b.object = T, b.property = this.parseIdent(true), b.computed = false, T = this.finishNode(b, "MemberExpression");
      }
      return n.expression = this.parseMaybeDecoratorArguments(T), this.decoratorStack.pop(), this.finishNode(n, "Decorator");
    }
    parseMaybeDecoratorArguments(n) {
      if (this.eat(i.parenL)) {
        const P = this.startNodeAtNode(n);
        return P.callee = n, P.arguments = this.parseExprList(i.parenR, false), this.finishNode(P, "CallExpression");
      }
      return n;
    }
  };
}
function z1(e, t, a) {
  const { tokTypes: i } = t, { tokTypes: l } = a;
  return class extends e {
    parseMaybeImportAttributes(n) {
      if (this.type === l._with || this.type === i.assert) {
        this.next();
        const P = this.parseImportAttributes();
        P && (n.attributes = P);
      }
    }
    parseImportAttributes() {
      this.expect(l.braceL);
      const n = this.parseWithEntries();
      return this.expect(l.braceR), n;
    }
    parseWithEntries() {
      const n = [], P = /* @__PURE__ */ new Set();
      do {
        if (this.type === l.braceR) break;
        const x = this.startNode();
        let T;
        this.type === l.string ? T = this.parseLiteral(this.value) : T = this.parseIdent(true), this.next(), x.key = T, P.has(x.key.name) && this.raise(this.pos, "Duplicated key in attributes"), P.add(x.key.name), this.type !== l.string && this.raise(this.pos, "Only string is supported as an attribute value"), x.value = this.parseLiteral(this.value), n.push(this.finishNode(x, "ImportAttribute"));
      } while (this.eat(l.comma));
      return n;
    }
  };
}
var hl = /(?:\s|\/\/.*|\/\*[^]*?\*\/)*/g;
function Xl(e) {
  if (!e) throw new Error("Assert fail");
}
function K1(e) {
  return e === "accessor";
}
function X1(e) {
  return e === "in" || e === "out";
}
var No = 1, Yl = 2, Y1 = 4, Ee = { SCOPE_TOP: 1, SCOPE_FUNCTION: 2, SCOPE_ASYNC: 4, SCOPE_GENERATOR: 8, SCOPE_ARROW: 16, SCOPE_SIMPLE_CATCH: 32, SCOPE_SUPER: 64, SCOPE_DIRECT_SUPER: 128, BIND_NONE: 0, BIND_VAR: 1, BIND_LEXICAL: 2, BIND_FUNCTION: 3, BIND_SIMPLE_CATCH: 4, BIND_TS_TYPE: 6, BIND_TS_INTERFACE: 7, BIND_TS_NAMESPACE: 8, BIND_FLAGS_TS_EXPORT_ONLY: 1024, BIND_FLAGS_TS_IMPORT: 4096, BIND_FLAGS_TS_ENUM: 256, BIND_FLAGS_TS_CONST_ENUM: 512, BIND_FLAGS_CLASS: 128 };
function ml(e, t) {
  return Ee.SCOPE_FUNCTION | (e ? Ee.SCOPE_ASYNC : 0) | (t ? Ee.SCOPE_GENERATOR : 0);
}
function Q1(e) {
  if (e.type !== "MemberExpression") return false;
  const { computed: t, property: a } = e;
  return t && (a.type !== "TemplateLiteral" || a.expressions.length > 0) ? false : dm(e.object);
}
function dm(e) {
  return e.type === "Identifier" ? true : e.type !== "MemberExpression" || e.computed ? false : dm(e.object);
}
function Ql(e) {
  return e === "private" || e === "public" || e === "protected";
}
function J1(e) {
  return !!e.startsExpr;
}
function Z1(e) {
  if (e == null) throw new Error(`Unexpected ${e} value.`);
  return e;
}
function e0(e) {
  switch (e) {
    case "any":
      return "TSAnyKeyword";
    case "boolean":
      return "TSBooleanKeyword";
    case "bigint":
      return "TSBigIntKeyword";
    case "never":
      return "TSNeverKeyword";
    case "number":
      return "TSNumberKeyword";
    case "object":
      return "TSObjectKeyword";
    case "string":
      return "TSStringKeyword";
    case "symbol":
      return "TSSymbolKeyword";
    case "undefined":
      return "TSUndefinedKeyword";
    case "unknown":
      return "TSUnknownKeyword";
    default:
      return;
  }
}
function t0(e) {
  const { dts: t = false } = {}, a = false;
  return function(i) {
    const l = i.acorn || D1, d = F1(l), n = l.tokTypes, P = l.keywordTypes, x = l.isIdentifierStart, T = l.lineBreak, b = l.isNewLine, k = l.tokContexts, I = l.isIdentifierChar, { tokTypes: E, tokContexts: $, keywordsRegExp: D, tokenIsLiteralPropertyName: S, tokenIsTemplate: h, tokenIsTSDeclarationStart: v, tokenIsIdentifier: m, tokenIsKeywordOrIdentifier: _, tokenIsTSTypeOperator: R } = d;
    function q(L, r, s = L.length) {
      for (let o = r; o < s; o++) {
        let u = L.charCodeAt(o);
        if (b(u)) return o < s - 1 && u === 13 && L.charCodeAt(o + 1) === 10 ? o + 2 : o + 1;
      }
      return -1;
    }
    i = W1(i, d, l), i = z1(i, d, l);
    class A extends i {
      constructor(r, s, o) {
        super(r, s, o), this.preValue = null, this.preToken = null, this.isLookahead = false, this.isAmbientContext = false, this.inAbstractClass = false, this.inType = false, this.inDisallowConditionalTypesContext = false, this.maybeInArrowParameters = false, this.shouldParseArrowReturnType = void 0, this.shouldParseAsyncArrowReturnType = void 0, this.decoratorStack = [[]], this.importsStack = [[]], this.importOrExportOuterKind = void 0, this.tsParseConstModifier = (u) => {
          this.tsParseModifiers({ modified: u, allowedModifiers: ["const"], disallowedModifiers: ["in", "out"], errorTemplate: ie.InvalidModifierOnTypeParameterPositions });
        }, this.ecmaVersion = this.options.ecmaVersion;
      }
      static get acornTypeScript() {
        return d;
      }
      get acornTypeScript() {
        return d;
      }
      getTokenFromCodeInType(r) {
        return r === 62 ? this.finishOp(n.relational, 1) : r === 60 ? this.finishOp(n.relational, 1) : super.getTokenFromCode(r);
      }
      readToken(r) {
        if (!this.inType) {
          let s = this.curContext();
          if (s === $.tc_expr) return this.jsx_readToken();
          if (s === $.tc_oTag || s === $.tc_cTag) {
            if (x(r)) return this.jsx_readWord();
            if (r == 62) return ++this.pos, this.finishToken(E.jsxTagEnd);
            if ((r === 34 || r === 39) && s == $.tc_oTag) return this.jsx_readString(r);
          }
          if (r === 60 && this.exprAllowed && this.input.charCodeAt(this.pos + 1) !== 33) return ++this.pos, this.finishToken(n.relational, "<");
        }
        return super.readToken(r);
      }
      getTokenFromCode(r) {
        return this.inType ? this.getTokenFromCodeInType(r) : r === 64 ? (++this.pos, this.finishToken(E.at)) : super.getTokenFromCode(r);
      }
      isAbstractClass() {
        return this.ts_isContextual(E.abstract) && this.lookahead().type === n._class;
      }
      finishNode(r, s) {
        return r.type !== "" && r.end !== 0 ? r : super.finishNode(r, s);
      }
      tryParse(r, s = this.cloneCurLookaheadState()) {
        const o = { node: null };
        try {
          return { node: r((p = null) => {
            throw o.node = p, o;
          }), error: null, thrown: false, aborted: false, failState: null };
        } catch (u) {
          const p = this.getCurLookaheadState();
          if (this.setLookaheadState(s), u instanceof SyntaxError) return { node: null, error: u, thrown: true, aborted: false, failState: p };
          if (u === o) return { node: o.node, error: null, thrown: false, aborted: true, failState: p };
          throw u;
        }
      }
      setOptionalParametersError(r, s) {
        r.optionalParametersLoc = (s == null ? void 0 : s.loc) ?? this.startLoc;
      }
      reScan_lt_gt() {
        this.type === n.relational && (this.pos -= 1, this.readToken_lt_gt(this.fullCharCodeAtPos()));
      }
      reScan_lt() {
        const { type: r } = this;
        return r === n.bitShift ? (this.pos -= 2, this.finishOp(n.relational, 1), n.relational) : r;
      }
      resetEndLocation(r, s = this.lastTokEnd, o = this.lastTokEndLoc) {
        r.end = s, r.loc.end = o, this.options.ranges && (r.range[1] = s);
      }
      startNodeAtNode(r) {
        return super.startNodeAt(r.start, r.loc.start);
      }
      nextTokenStart() {
        return this.nextTokenStartSince(this.pos);
      }
      tsHasSomeModifiers(r, s) {
        return s.some((o) => Ql(o) ? r.accessibility === o : !!r[o]);
      }
      tsIsStartOfStaticBlocks() {
        return this.isContextual("static") && this.lookaheadCharCode() === 123;
      }
      tsCheckForInvalidTypeCasts(r) {
        r.forEach((s) => {
          (s == null ? void 0 : s.type) === "TSTypeCastExpression" && this.raise(s.typeAnnotation.start, ie.UnexpectedTypeAnnotation);
        });
      }
      atPossibleAsyncArrow(r) {
        return r.type === "Identifier" && r.name === "async" && this.lastTokEndLoc.column === r.end && !this.canInsertSemicolon() && r.end - r.start === 5 && r.start === this.potentialArrowAt;
      }
      tsIsIdentifier() {
        return m(this.type);
      }
      tsTryParseTypeOrTypePredicateAnnotation() {
        return this.match(n.colon) ? this.tsParseTypeOrTypePredicateAnnotation(n.colon) : void 0;
      }
      tsTryParseGenericAsyncArrowFunction(r, s, o) {
        if (!this.tsMatchLeftRelational()) return;
        const u = this.maybeInArrowParameters;
        this.maybeInArrowParameters = true;
        const p = this.tsTryParseAndCatch(() => {
          const f = this.startNodeAt(r, s);
          return f.typeParameters = this.tsParseTypeParameters(this.tsParseConstModifier), super.parseFunctionParams(f), f.returnType = this.tsTryParseTypeOrTypePredicateAnnotation(), this.expect(n.arrow), f;
        });
        if (this.maybeInArrowParameters = u, !!p) return super.parseArrowExpression(p, null, true, o);
      }
      tsParseTypeArgumentsInExpression() {
        if (this.reScan_lt() === n.relational) return this.tsParseTypeArguments();
      }
      tsInNoContext(r) {
        const s = this.context;
        this.context = [s[0]];
        try {
          return r();
        } finally {
          this.context = s;
        }
      }
      tsTryParseTypeAnnotation() {
        return this.match(n.colon) ? this.tsParseTypeAnnotation() : void 0;
      }
      isUnparsedContextual(r, s) {
        const o = r + s.length;
        if (this.input.slice(r, o) === s) {
          const u = this.input.charCodeAt(o);
          return !(I(u) || (u & 64512) === 55296);
        }
        return false;
      }
      isAbstractConstructorSignature() {
        return this.ts_isContextual(E.abstract) && this.lookahead().type === n._new;
      }
      nextTokenStartSince(r) {
        return hl.lastIndex = r, hl.test(this.input) ? hl.lastIndex : r;
      }
      lookaheadCharCode() {
        return this.input.charCodeAt(this.nextTokenStart());
      }
      compareLookaheadState(r, s) {
        for (const o of Object.keys(r)) if (r[o] !== s[o]) return false;
        return true;
      }
      createLookaheadState() {
        this.value = null, this.context = [this.curContext()];
      }
      getCurLookaheadState() {
        return { endLoc: this.endLoc, lastTokEnd: this.lastTokEnd, lastTokStart: this.lastTokStart, lastTokStartLoc: this.lastTokStartLoc, pos: this.pos, value: this.value, type: this.type, start: this.start, end: this.end, context: this.context, startLoc: this.startLoc, lastTokEndLoc: this.lastTokEndLoc, curLine: this.curLine, lineStart: this.lineStart, curPosition: this.curPosition, containsEsc: this.containsEsc };
      }
      cloneCurLookaheadState() {
        return { pos: this.pos, value: this.value, type: this.type, start: this.start, end: this.end, context: this.context && this.context.slice(), startLoc: this.startLoc, lastTokEndLoc: this.lastTokEndLoc, endLoc: this.endLoc, lastTokEnd: this.lastTokEnd, lastTokStart: this.lastTokStart, lastTokStartLoc: this.lastTokStartLoc, curLine: this.curLine, lineStart: this.lineStart, curPosition: this.curPosition, containsEsc: this.containsEsc };
      }
      setLookaheadState(r) {
        this.pos = r.pos, this.value = r.value, this.endLoc = r.endLoc, this.lastTokEnd = r.lastTokEnd, this.lastTokStart = r.lastTokStart, this.lastTokStartLoc = r.lastTokStartLoc, this.type = r.type, this.start = r.start, this.end = r.end, this.context = r.context, this.startLoc = r.startLoc, this.lastTokEndLoc = r.lastTokEndLoc, this.curLine = r.curLine, this.lineStart = r.lineStart, this.curPosition = r.curPosition, this.containsEsc = r.containsEsc;
      }
      tsLookAhead(r) {
        const s = this.getCurLookaheadState(), o = r();
        return this.setLookaheadState(s), o;
      }
      lookahead(r) {
        const s = this.getCurLookaheadState();
        if (this.createLookaheadState(), this.isLookahead = true, r !== void 0) for (let u = 0; u < r; u++) this.nextToken();
        else this.nextToken();
        this.isLookahead = false;
        const o = this.getCurLookaheadState();
        return this.setLookaheadState(s), o;
      }
      readWord() {
        let r = this.readWord1(), s = n.name;
        return this.keywords.test(r) ? s = P[r] : new RegExp(D).test(r) && (s = E[r]), this.finishToken(s, r);
      }
      skipBlockComment() {
        let r;
        this.isLookahead || (r = this.options.onComment && this.curPosition());
        let s = this.pos, o = this.input.indexOf("*/", this.pos += 2);
        if (o === -1 && this.raise(this.pos - 2, "Unterminated comment"), this.pos = o + 2, this.options.locations) for (let u, p = s; (u = q(this.input, p, this.pos)) > -1; ) ++this.curLine, p = this.lineStart = u;
        this.isLookahead || this.options.onComment && this.options.onComment(true, this.input.slice(s + 2, o), s, this.pos, r, this.curPosition());
      }
      skipLineComment(r) {
        let s = this.pos, o;
        this.isLookahead || (o = this.options.onComment && this.curPosition());
        let u = this.input.charCodeAt(this.pos += r);
        for (; this.pos < this.input.length && !b(u); ) u = this.input.charCodeAt(++this.pos);
        this.isLookahead || this.options.onComment && this.options.onComment(false, this.input.slice(s + r, this.pos), s, this.pos, o, this.curPosition());
      }
      finishToken(r, s) {
        this.preValue = this.value, this.preToken = this.type, this.end = this.pos, this.options.locations && (this.endLoc = this.curPosition());
        let o = this.type;
        this.type = r, this.value = s, this.isLookahead || this.updateContext(o);
      }
      resetStartLocation(r, s, o) {
        r.start = s, r.loc.start = o, this.options.ranges && (r.range[0] = s);
      }
      isLineTerminator() {
        return this.eat(n.semi) || super.canInsertSemicolon();
      }
      hasFollowingLineBreak() {
        return zl.lastIndex = this.end, zl.test(this.input);
      }
      addExtra(r, s, o, u = true) {
        if (!r) return;
        const p = r.extra = r.extra || {};
        u ? p[s] = o : Object.defineProperty(p, s, { enumerable: u, value: o });
      }
      isLiteralPropertyName() {
        return S(this.type);
      }
      hasPrecedingLineBreak() {
        return T.test(this.input.slice(this.lastTokEnd, this.start));
      }
      createIdentifier(r, s) {
        return r.name = s, this.finishNode(r, "Identifier");
      }
      resetStartLocationFromNode(r, s) {
        this.resetStartLocation(r, s.start, s.loc.start);
      }
      isThisParam(r) {
        return r.type === "Identifier" && r.name === "this";
      }
      isLookaheadContextual(r) {
        const s = this.nextTokenStart();
        return this.isUnparsedContextual(s, r);
      }
      ts_type_isContextual(r, s) {
        return r === s && !this.containsEsc;
      }
      ts_isContextual(r) {
        return this.type === r && !this.containsEsc;
      }
      ts_isContextualWithState(r, s) {
        return r.type === s && !r.containsEsc;
      }
      isContextualWithState(r, s) {
        return s.type === n.name && s.value === r && !s.containsEsc;
      }
      tsIsStartOfMappedType() {
        return this.next(), this.eat(n.plusMin) ? this.ts_isContextual(E.readonly) : (this.ts_isContextual(E.readonly) && this.next(), !this.match(n.bracketL) || (this.next(), !this.tsIsIdentifier()) ? false : (this.next(), this.match(n._in)));
      }
      tsInDisallowConditionalTypesContext(r) {
        const s = this.inDisallowConditionalTypesContext;
        this.inDisallowConditionalTypesContext = true;
        try {
          return r();
        } finally {
          this.inDisallowConditionalTypesContext = s;
        }
      }
      tsTryParseType() {
        return this.tsEatThenParseType(n.colon);
      }
      match(r) {
        return this.type === r;
      }
      matchJsx(r) {
        return this.type === d.tokTypes[r];
      }
      ts_eatWithState(r, s, o) {
        const u = o.type;
        if (r === u) {
          for (let p = 0; p < s; p++) this.next();
          return true;
        } else return false;
      }
      ts_eatContextualWithState(r, s, o) {
        if (D.test(r)) {
          if (this.ts_isContextualWithState(o, E[r])) {
            for (let u = 0; u < s; u++) this.next();
            return true;
          }
          return false;
        } else {
          if (!this.isContextualWithState(r, o)) return false;
          for (let u = 0; u < s; u++) this.next();
          return true;
        }
      }
      canHaveLeadingDecorator() {
        return this.match(n._class);
      }
      eatContextual(r) {
        return D.test(r) ? this.ts_isContextual(E[r]) ? (this.next(), true) : false : super.eatContextual(r);
      }
      tsIsExternalModuleReference() {
        return this.isContextual("require") && this.lookaheadCharCode() === 40;
      }
      tsParseExternalModuleReference() {
        const r = this.startNode();
        return this.expectContextual("require"), this.expect(n.parenL), this.match(n.string) || this.unexpected(), r.expression = this.parseExprAtom(), this.expect(n.parenR), this.finishNode(r, "TSExternalModuleReference");
      }
      tsParseEntityName(r = true) {
        let s = this.parseIdent(r);
        for (; this.eat(n.dot); ) {
          const o = this.startNodeAtNode(s);
          o.left = s, o.right = this.parseIdent(r), s = this.finishNode(o, "TSQualifiedName");
        }
        return s;
      }
      tsParseEnumMember() {
        const r = this.startNode();
        return r.id = this.match(n.string) ? this.parseLiteral(this.value) : this.parseIdent(true), this.eat(n.eq) && (r.initializer = this.parseMaybeAssign()), this.finishNode(r, "TSEnumMember");
      }
      tsParseEnumDeclaration(r, s = {}) {
        return s.const && (r.const = true), s.declare && (r.declare = true), this.expectContextual("enum"), r.id = this.parseIdent(), this.checkLValSimple(r.id), this.expect(n.braceL), r.members = this.tsParseDelimitedList("EnumMembers", this.tsParseEnumMember.bind(this)), this.expect(n.braceR), this.finishNode(r, "TSEnumDeclaration");
      }
      tsParseModuleBlock() {
        const r = this.startNode();
        for (this.enterScope(H1), this.expect(n.braceL), r.body = []; this.type !== n.braceR; ) {
          let s = this.parseStatement(null, true);
          r.body.push(s);
        }
        return this.next(), super.exitScope(), this.finishNode(r, "TSModuleBlock");
      }
      tsParseAmbientExternalModuleDeclaration(r) {
        return this.ts_isContextual(E.global) ? (r.global = true, r.id = this.parseIdent()) : this.match(n.string) ? r.id = this.parseLiteral(this.value) : this.unexpected(), this.match(n.braceL) ? (this.enterScope(ur), r.body = this.tsParseModuleBlock(), super.exitScope()) : super.semicolon(), this.finishNode(r, "TSModuleDeclaration");
      }
      tsTryParseDeclare(r) {
        if (this.isLineTerminator()) return;
        let s = this.type, o;
        return this.isContextual("let") && (s = n._var, o = "let"), this.tsInAmbientContext(() => {
          if (s === n._function) return r.declare = true, this.parseFunctionStatement(r, false, true);
          if (s === n._class) return r.declare = true, this.parseClass(r, true);
          if (s === E.enum) return this.tsParseEnumDeclaration(r, { declare: true });
          if (s === E.global) return this.tsParseAmbientExternalModuleDeclaration(r);
          if (s === n._const || s === n._var) return !this.match(n._const) || !this.isLookaheadContextual("enum") ? (r.declare = true, this.parseVarStatement(r, o || this.value, true)) : (this.expect(n._const), this.tsParseEnumDeclaration(r, { const: true, declare: true }));
          if (s === E.interface) {
            const u = this.tsParseInterfaceDeclaration(r, { declare: true });
            if (u) return u;
          }
          if (m(s)) return this.tsParseDeclaration(r, this.value, true);
        });
      }
      tsIsListTerminator(r) {
        switch (r) {
          case "EnumMembers":
          case "TypeMembers":
            return this.match(n.braceR);
          case "HeritageClauseElement":
            return this.match(n.braceL);
          case "TupleElementTypes":
            return this.match(n.bracketR);
          case "TypeParametersOrArguments":
            return this.tsMatchRightRelational();
        }
      }
      tsParseDelimitedListWorker(r, s, o, u) {
        const p = [];
        let f = -1;
        for (; !this.tsIsListTerminator(r); ) {
          f = -1;
          const g = s();
          if (g == null) return;
          if (p.push(g), this.eat(n.comma)) {
            f = this.lastTokStart;
            continue;
          }
          if (this.tsIsListTerminator(r)) break;
          o && this.expect(n.comma);
          return;
        }
        return u && (u.value = f), p;
      }
      tsParseDelimitedList(r, s, o) {
        return Z1(this.tsParseDelimitedListWorker(r, s, true, o));
      }
      tsParseBracketedList(r, s, o, u, p) {
        u || (o ? this.expect(n.bracketL) : this.expect(n.relational));
        const f = this.tsParseDelimitedList(r, s, p);
        return o ? this.expect(n.bracketR) : this.expect(n.relational), f;
      }
      tsParseTypeParameterName() {
        return this.parseIdent().name;
      }
      tsEatThenParseType(r) {
        return this.match(r) ? this.tsNextThenParseType() : void 0;
      }
      tsExpectThenParseType(r) {
        return this.tsDoThenParseType(() => this.expect(r));
      }
      tsNextThenParseType() {
        return this.tsDoThenParseType(() => this.next());
      }
      tsDoThenParseType(r) {
        return this.tsInType(() => (r(), this.tsParseType()));
      }
      tsSkipParameterStart() {
        if (m(this.type) || this.match(n._this)) return this.next(), true;
        if (this.match(n.braceL)) try {
          return this.parseObj(true), true;
        } catch {
          return false;
        }
        if (this.match(n.bracketL)) {
          this.next();
          try {
            return this.parseBindingList(n.bracketR, true, true), true;
          } catch {
            return false;
          }
        }
        return false;
      }
      tsIsUnambiguouslyStartOfFunctionType() {
        return this.next(), !!(this.match(n.parenR) || this.match(n.ellipsis) || this.tsSkipParameterStart() && (this.match(n.colon) || this.match(n.comma) || this.match(n.question) || this.match(n.eq) || this.match(n.parenR) && (this.next(), this.match(n.arrow))));
      }
      tsIsStartOfFunctionType() {
        return this.tsMatchLeftRelational() ? true : this.match(n.parenL) && this.tsLookAhead(this.tsIsUnambiguouslyStartOfFunctionType.bind(this));
      }
      tsInAllowConditionalTypesContext(r) {
        const s = this.inDisallowConditionalTypesContext;
        this.inDisallowConditionalTypesContext = false;
        try {
          return r();
        } finally {
          this.inDisallowConditionalTypesContext = s;
        }
      }
      tsParseBindingListForSignature() {
        return super.parseBindingList(n.parenR, true, true).map((r) => (r.type !== "Identifier" && r.type !== "RestElement" && r.type !== "ObjectPattern" && r.type !== "ArrayPattern" && this.raise(r.start, ie.UnsupportedSignatureParameterKind(r.type)), r));
      }
      tsParseTypePredicateAsserts() {
        if (this.type !== E.asserts) return false;
        const r = this.containsEsc;
        return this.next(), !m(this.type) && !this.match(n._this) ? false : (r && this.raise(this.lastTokStart, "Escape sequence in keyword asserts"), true);
      }
      tsParseThisTypeNode() {
        const r = this.startNode();
        return this.next(), this.finishNode(r, "TSThisType");
      }
      tsParseTypeAnnotation(r = true, s = this.startNode()) {
        return this.tsInType(() => {
          r && this.expect(n.colon), s.typeAnnotation = this.tsParseType();
        }), this.finishNode(s, "TSTypeAnnotation");
      }
      tsParseThisTypePredicate(r) {
        this.next();
        const s = this.startNodeAtNode(r);
        return s.parameterName = r, s.typeAnnotation = this.tsParseTypeAnnotation(false), s.asserts = false, this.finishNode(s, "TSTypePredicate");
      }
      tsParseThisTypeOrThisTypePredicate() {
        const r = this.tsParseThisTypeNode();
        return this.isContextual("is") && !this.hasPrecedingLineBreak() ? this.tsParseThisTypePredicate(r) : r;
      }
      tsParseTypePredicatePrefix() {
        const r = this.parseIdent();
        if (this.isContextual("is") && !this.hasPrecedingLineBreak()) return this.next(), r;
      }
      tsParseTypeOrTypePredicateAnnotation(r) {
        return this.tsInType(() => {
          const s = this.startNode();
          this.expect(r);
          const o = this.startNode(), u = !!this.tsTryParse(this.tsParseTypePredicateAsserts.bind(this));
          if (u && this.match(n._this)) {
            let g = this.tsParseThisTypeOrThisTypePredicate();
            return g.type === "TSThisType" ? (o.parameterName = g, o.asserts = true, o.typeAnnotation = null, g = this.finishNode(o, "TSTypePredicate")) : (this.resetStartLocationFromNode(g, o), g.asserts = true), s.typeAnnotation = g, this.finishNode(s, "TSTypeAnnotation");
          }
          const p = this.tsIsIdentifier() && this.tsTryParse(this.tsParseTypePredicatePrefix.bind(this));
          if (!p) return u ? (o.parameterName = this.parseIdent(), o.asserts = u, o.typeAnnotation = null, s.typeAnnotation = this.finishNode(o, "TSTypePredicate"), this.finishNode(s, "TSTypeAnnotation")) : this.tsParseTypeAnnotation(false, s);
          const f = this.tsParseTypeAnnotation(false);
          return o.parameterName = p, o.typeAnnotation = f, o.asserts = u, s.typeAnnotation = this.finishNode(o, "TSTypePredicate"), this.finishNode(s, "TSTypeAnnotation");
        });
      }
      tsFillSignature(r, s) {
        const o = r === n.arrow, u = "parameters", p = "typeAnnotation";
        s.typeParameters = this.tsTryParseTypeParameters(), this.expect(n.parenL), s[u] = this.tsParseBindingListForSignature(), o ? s[p] = this.tsParseTypeOrTypePredicateAnnotation(r) : this.match(r) && (s[p] = this.tsParseTypeOrTypePredicateAnnotation(r));
      }
      tsTryNextParseConstantContext() {
        if (this.lookahead().type !== n._const) return null;
        this.next();
        const r = this.tsParseTypeReference();
        return (r.typeParameters || r.typeArguments) && this.raise(r.typeName.start, ie.CannotFindName({ name: "const" })), r;
      }
      tsParseFunctionOrConstructorType(r, s) {
        const o = this.startNode();
        return r === "TSConstructorType" && (o.abstract = !!s, s && this.next(), this.next()), this.tsInAllowConditionalTypesContext(() => this.tsFillSignature(n.arrow, o)), this.finishNode(o, r);
      }
      tsParseUnionOrIntersectionType(r, s, o) {
        const u = this.startNode(), p = this.eat(o), f = [];
        do
          f.push(s());
        while (this.eat(o));
        return f.length === 1 && !p ? f[0] : (u.types = f, this.finishNode(u, r));
      }
      tsCheckTypeAnnotationForReadOnly(r) {
        switch (r.typeAnnotation.type) {
          case "TSTupleType":
          case "TSArrayType":
            return;
          default:
            this.raise(r.start, ie.UnexpectedReadonly);
        }
      }
      tsParseTypeOperator() {
        const r = this.startNode(), s = this.value;
        return this.next(), r.operator = s, r.typeAnnotation = this.tsParseTypeOperatorOrHigher(), s === "readonly" && this.tsCheckTypeAnnotationForReadOnly(r), this.finishNode(r, "TSTypeOperator");
      }
      tsParseConstraintForInferType() {
        if (this.eat(n._extends)) {
          const r = this.tsInDisallowConditionalTypesContext(() => this.tsParseType());
          if (this.inDisallowConditionalTypesContext || !this.match(n.question)) return r;
        }
      }
      tsParseInferType() {
        const r = this.startNode();
        this.expectContextual("infer");
        const s = this.startNode();
        return s.name = this.tsParseTypeParameterName(), s.constraint = this.tsTryParse(() => this.tsParseConstraintForInferType()), r.typeParameter = this.finishNode(s, "TSTypeParameter"), this.finishNode(r, "TSInferType");
      }
      tsParseLiteralTypeNode() {
        const r = this.startNode();
        return r.literal = (() => {
          switch (this.type) {
            case n.num:
            case n.string:
            case n._true:
            case n._false:
              return this.parseExprAtom();
            default:
              this.unexpected();
          }
        })(), this.finishNode(r, "TSLiteralType");
      }
      tsParseImportType() {
        const r = this.startNode();
        return this.expect(n._import), this.expect(n.parenL), this.match(n.string) || this.raise(this.start, ie.UnsupportedImportTypeArgument), r.argument = this.parseExprAtom(), this.expect(n.parenR), this.eat(n.dot) && (r.qualifier = this.tsParseEntityName()), this.tsMatchLeftRelational() && (r.typeArguments = this.tsParseTypeArguments()), this.finishNode(r, "TSImportType");
      }
      tsParseTypeQuery() {
        const r = this.startNode();
        return this.expect(n._typeof), this.match(n._import) ? r.exprName = this.tsParseImportType() : r.exprName = this.tsParseEntityName(), !this.hasPrecedingLineBreak() && this.tsMatchLeftRelational() && (r.typeArguments = this.tsParseTypeArguments()), this.finishNode(r, "TSTypeQuery");
      }
      tsParseMappedTypeParameter() {
        const r = this.startNode();
        return r.name = this.tsParseTypeParameterName(), r.constraint = this.tsExpectThenParseType(n._in), this.finishNode(r, "TSTypeParameter");
      }
      tsParseMappedType() {
        const r = this.startNode();
        return this.expect(n.braceL), this.match(n.plusMin) ? (r.readonly = this.value, this.next(), this.expectContextual("readonly")) : this.eatContextual("readonly") && (r.readonly = true), this.expect(n.bracketL), r.typeParameter = this.tsParseMappedTypeParameter(), r.nameType = this.eatContextual("as") ? this.tsParseType() : null, this.expect(n.bracketR), this.match(n.plusMin) ? (r.optional = this.value, this.next(), this.expect(n.question)) : this.eat(n.question) && (r.optional = true), r.typeAnnotation = this.tsTryParseType(), this.semicolon(), this.expect(n.braceR), this.finishNode(r, "TSMappedType");
      }
      tsParseTypeLiteral() {
        const r = this.startNode();
        return r.members = this.tsParseObjectTypeMembers(), this.finishNode(r, "TSTypeLiteral");
      }
      tsParseTupleElementType() {
        const r = this.startLoc, s = this.start, o = this.eat(n.ellipsis);
        let u = this.tsParseType();
        const p = this.eat(n.question);
        if (this.eat(n.colon)) {
          const g = this.startNodeAtNode(u);
          g.optional = p, u.type === "TSTypeReference" && !u.typeArguments && u.typeName.type === "Identifier" ? g.label = u.typeName : (this.raise(u.start, ie.InvalidTupleMemberLabel), g.label = u), g.elementType = this.tsParseType(), u = this.finishNode(g, "TSNamedTupleMember");
        } else if (p) {
          const g = this.startNodeAtNode(u);
          g.typeAnnotation = u, u = this.finishNode(g, "TSOptionalType");
        }
        if (o) {
          const g = this.startNodeAt(s, r);
          g.typeAnnotation = u, u = this.finishNode(g, "TSRestType");
        }
        return u;
      }
      tsParseTupleType() {
        const r = this.startNode();
        r.elementTypes = this.tsParseBracketedList("TupleElementTypes", this.tsParseTupleElementType.bind(this), true, false);
        let s = false, o = null;
        return r.elementTypes.forEach((u) => {
          const { type: p } = u;
          s && p !== "TSRestType" && p !== "TSOptionalType" && !(p === "TSNamedTupleMember" && u.optional) && this.raise(u.start, ie.OptionalTypeBeforeRequired), s || (s = p === "TSNamedTupleMember" && u.optional || p === "TSOptionalType");
          let f = p;
          p === "TSRestType" && (u = u.typeAnnotation, f = u.type);
          const g = f === "TSNamedTupleMember";
          o ?? (o = g), o !== g && this.raise(u.start, ie.MixedLabeledAndUnlabeledElements);
        }), this.finishNode(r, "TSTupleType");
      }
      tsParseTemplateLiteralType() {
        const r = this.startNode();
        return r.literal = this.parseTemplate({ isTagged: false }), this.finishNode(r, "TSLiteralType");
      }
      tsParseTypeReference() {
        const r = this.startNode();
        return r.typeName = this.tsParseEntityName(), !this.hasPrecedingLineBreak() && this.tsMatchLeftRelational() && (r.typeArguments = this.tsParseTypeArguments()), this.finishNode(r, "TSTypeReference");
      }
      tsMatchLeftRelational() {
        return this.match(n.relational) && this.value === "<";
      }
      tsMatchRightRelational() {
        return this.match(n.relational) && this.value === ">";
      }
      tsParseParenthesizedType() {
        const r = this.startNode();
        return this.expect(n.parenL), r.typeAnnotation = this.tsParseType(), this.expect(n.parenR), this.finishNode(r, "TSParenthesizedType");
      }
      tsParseNonArrayType() {
        switch (this.type) {
          case n.string:
          case n.num:
          case n._true:
          case n._false:
            return this.tsParseLiteralTypeNode();
          case n.plusMin:
            if (this.value === "-") {
              const r = this.startNode();
              return this.lookahead().type !== n.num && this.unexpected(), r.literal = this.parseMaybeUnary(), this.finishNode(r, "TSLiteralType");
            }
            break;
          case n._this:
            return this.tsParseThisTypeOrThisTypePredicate();
          case n._typeof:
            return this.tsParseTypeQuery();
          case n._import:
            return this.tsParseImportType();
          case n.braceL:
            return this.tsLookAhead(this.tsIsStartOfMappedType.bind(this)) ? this.tsParseMappedType() : this.tsParseTypeLiteral();
          case n.bracketL:
            return this.tsParseTupleType();
          case n.parenL:
            return this.tsParseParenthesizedType();
          case n.backQuote:
          case n.dollarBraceL:
            return this.tsParseTemplateLiteralType();
          default: {
            const { type: r } = this;
            if (m(r) || r === n._void || r === n._null) {
              const s = r === n._void ? "TSVoidKeyword" : r === n._null ? "TSNullKeyword" : e0(this.value);
              if (s !== void 0 && this.lookaheadCharCode() !== 46) {
                const o = this.startNode();
                return this.next(), this.finishNode(o, s);
              }
              return this.tsParseTypeReference();
            }
          }
        }
        this.unexpected();
      }
      tsParseArrayTypeOrHigher() {
        let r = this.tsParseNonArrayType();
        for (; !this.hasPrecedingLineBreak() && this.eat(n.bracketL); ) if (this.match(n.bracketR)) {
          const s = this.startNodeAtNode(r);
          s.elementType = r, this.expect(n.bracketR), r = this.finishNode(s, "TSArrayType");
        } else {
          const s = this.startNodeAtNode(r);
          s.objectType = r, s.indexType = this.tsParseType(), this.expect(n.bracketR), r = this.finishNode(s, "TSIndexedAccessType");
        }
        return r;
      }
      tsParseTypeOperatorOrHigher() {
        return R(this.type) && !this.containsEsc ? this.tsParseTypeOperator() : this.isContextual("infer") ? this.tsParseInferType() : this.tsInAllowConditionalTypesContext(() => this.tsParseArrayTypeOrHigher());
      }
      tsParseIntersectionTypeOrHigher() {
        return this.tsParseUnionOrIntersectionType("TSIntersectionType", this.tsParseTypeOperatorOrHigher.bind(this), n.bitwiseAND);
      }
      tsParseUnionTypeOrHigher() {
        return this.tsParseUnionOrIntersectionType("TSUnionType", this.tsParseIntersectionTypeOrHigher.bind(this), n.bitwiseOR);
      }
      tsParseNonConditionalType() {
        return this.tsIsStartOfFunctionType() ? this.tsParseFunctionOrConstructorType("TSFunctionType") : this.match(n._new) ? this.tsParseFunctionOrConstructorType("TSConstructorType") : this.isAbstractConstructorSignature() ? this.tsParseFunctionOrConstructorType("TSConstructorType", true) : this.tsParseUnionTypeOrHigher();
      }
      tsParseType() {
        Xl(this.inType);
        const r = this.tsParseNonConditionalType();
        if (this.inDisallowConditionalTypesContext || this.hasPrecedingLineBreak() || !this.eat(n._extends)) return r;
        const s = this.startNodeAtNode(r);
        return s.checkType = r, s.extendsType = this.tsInDisallowConditionalTypesContext(() => this.tsParseNonConditionalType()), this.expect(n.question), s.trueType = this.tsInAllowConditionalTypesContext(() => this.tsParseType()), this.expect(n.colon), s.falseType = this.tsInAllowConditionalTypesContext(() => this.tsParseType()), this.finishNode(s, "TSConditionalType");
      }
      tsIsUnambiguouslyIndexSignature() {
        return this.next(), m(this.type) ? (this.next(), this.match(n.colon)) : false;
      }
      tsInType(r) {
        const s = this.inType;
        this.inType = true;
        try {
          return r();
        } finally {
          this.inType = s;
        }
      }
      tsTryParseIndexSignature(r) {
        if (!(this.match(n.bracketL) && this.tsLookAhead(this.tsIsUnambiguouslyIndexSignature.bind(this)))) return;
        this.expect(n.bracketL);
        const s = this.parseIdent();
        s.typeAnnotation = this.tsParseTypeAnnotation(), this.resetEndLocation(s), this.expect(n.bracketR), r.parameters = [s];
        const o = this.tsTryParseTypeAnnotation();
        return o && (r.typeAnnotation = o), this.tsParseTypeMemberSemicolon(), this.finishNode(r, "TSIndexSignature");
      }
      tsParseNoneModifiers(r) {
        this.tsParseModifiers({ modified: r, allowedModifiers: [], disallowedModifiers: ["in", "out"], errorTemplate: ie.InvalidModifierOnTypeParameterPositions });
      }
      tsParseTypeParameter(r = this.tsParseNoneModifiers.bind(this)) {
        const s = this.startNode();
        return r(s), s.name = this.tsParseTypeParameterName(), s.constraint = this.tsEatThenParseType(n._extends), s.default = this.tsEatThenParseType(n.eq), this.finishNode(s, "TSTypeParameter");
      }
      tsParseTypeParameters(r) {
        const s = this.startNode();
        this.tsMatchLeftRelational() || this.matchJsx("jsxTagStart") ? this.next() : this.unexpected();
        const o = { value: -1 };
        return s.params = this.tsParseBracketedList("TypeParametersOrArguments", this.tsParseTypeParameter.bind(this, r), false, true, o), s.params.length === 0 && this.raise(this.start, ie.EmptyTypeParameters), o.value !== -1 && this.addExtra(s, "trailingComma", o.value), this.finishNode(s, "TSTypeParameterDeclaration");
      }
      tsTryParseTypeParameters(r) {
        if (this.tsMatchLeftRelational()) return this.tsParseTypeParameters(r);
      }
      tsTryParse(r) {
        const s = this.getCurLookaheadState(), o = r();
        if (o !== void 0 && o !== false) return o;
        this.setLookaheadState(s);
      }
      tsTokenCanFollowModifier() {
        return (this.match(n.bracketL) || this.match(n.braceL) || this.match(n.star) || this.match(n.ellipsis) || this.match(n.privateId) || this.isLiteralPropertyName()) && !this.hasPrecedingLineBreak();
      }
      tsNextTokenCanFollowModifier() {
        return this.next(true), this.tsTokenCanFollowModifier();
      }
      tsParseModifier(r, s) {
        const o = this.value;
        if (r.indexOf(o) !== -1 && !this.containsEsc) {
          if (s && this.tsIsStartOfStaticBlocks()) return;
          if (this.tsTryParse(this.tsNextTokenCanFollowModifier.bind(this))) return o;
        }
      }
      tsParseModifiersByMap({ modified: r, map: s }) {
        for (const o of Object.keys(s)) r[o] = s[o];
      }
      tsParseModifiers({ modified: r, allowedModifiers: s, disallowedModifiers: o, stopOnStartOfClassStaticBlock: u, errorTemplate: p = ie.InvalidModifierOnTypeMember }) {
        const f = {}, g = (C, y, O, N) => {
          y === O && r[N] && this.raise(C.column, ie.InvalidModifiersOrder({ orderedModifiers: [O, N] }));
        }, M = (C, y, O, N) => {
          (r[O] && y === N || r[N] && y === O) && this.raise(C.column, ie.IncompatibleModifiers({ modifiers: [O, N] }));
        };
        for (; ; ) {
          const C = this.startLoc, y = this.tsParseModifier(s.concat(o ?? []), u);
          if (!y) break;
          Ql(y) ? r.accessibility ? this.raise(this.start, ie.DuplicateAccessibilityModifier()) : (g(C, y, y, "override"), g(C, y, y, "static"), g(C, y, y, "readonly"), g(C, y, y, "accessor"), f.accessibility = y, r.accessibility = y) : X1(y) ? r[y] ? this.raise(this.start, ie.DuplicateModifier({ modifier: y })) : (g(C, y, "in", "out"), f[y] = y, r[y] = true) : K1(y) ? r[y] ? this.raise(this.start, ie.DuplicateModifier({ modifier: y })) : (M(C, y, "accessor", "readonly"), M(C, y, "accessor", "static"), M(C, y, "accessor", "override"), f[y] = y, r[y] = true) : y === "const" ? r[y] ? this.raise(this.start, ie.DuplicateModifier({ modifier: y })) : (f[y] = y, r[y] = true) : Object.hasOwnProperty.call(r, y) ? this.raise(this.start, ie.DuplicateModifier({ modifier: y })) : (g(C, y, "static", "readonly"), g(C, y, "static", "override"), g(C, y, "override", "readonly"), g(C, y, "abstract", "override"), M(C, y, "declare", "override"), M(C, y, "static", "abstract"), f[y] = y, r[y] = true), (o == null ? void 0 : o.includes(y)) && this.raise(this.start, p);
        }
        return f;
      }
      tsParseInOutModifiers(r) {
        this.tsParseModifiers({ modified: r, allowedModifiers: ["in", "out"], disallowedModifiers: ["public", "private", "protected", "readonly", "declare", "abstract", "override"], errorTemplate: ie.InvalidModifierOnTypeParameter });
      }
      parseMaybeUnary(r, s, o, u) {
        return this.tsMatchLeftRelational() ? this.tsParseTypeAssertion() : super.parseMaybeUnary(r, s, o, u);
      }
      tsParseTypeAssertion() {
        const r = this.tryParse(() => {
          const s = this.startNode(), o = this.tsTryNextParseConstantContext();
          return s.typeAnnotation = o || this.tsNextThenParseType(), this.expect(n.relational), s.expression = this.parseMaybeUnary(), this.finishNode(s, "TSTypeAssertion");
        });
        return r.error ? this.tsParseTypeParameters(this.tsParseConstModifier) : r.node;
      }
      tsParseTypeArguments() {
        const r = this.startNode();
        return r.params = this.tsInType(() => this.tsInNoContext(() => (this.expect(n.relational), this.tsParseDelimitedList("TypeParametersOrArguments", this.tsParseType.bind(this))))), r.params.length === 0 && this.raise(this.start, ie.EmptyTypeArguments), this.exprAllowed = false, this.expect(n.relational), this.finishNode(r, "TSTypeParameterInstantiation");
      }
      tsParseHeritageClause(r) {
        const s = this.start, o = this.tsParseDelimitedList("HeritageClauseElement", () => {
          const u = this.startNode();
          return u.expression = this.tsParseEntityName(), this.tsMatchLeftRelational() && (u.typeParameters = this.tsParseTypeArguments()), this.finishNode(u, "TSExpressionWithTypeArguments");
        });
        return o.length || this.raise(s, ie.EmptyHeritageClauseType({ token: r })), o;
      }
      tsParseTypeMemberSemicolon() {
        !this.eat(n.comma) && !this.isLineTerminator() && this.expect(n.semi);
      }
      tsTryParseAndCatch(r) {
        const s = this.tryParse((o) => r() || o());
        if (!(s.aborted || !s.node)) return s.error && this.setLookaheadState(s.failState), s.node;
      }
      tsParseSignatureMember(r, s) {
        return this.tsFillSignature(n.colon, s), this.tsParseTypeMemberSemicolon(), this.finishNode(s, r);
      }
      tsParsePropertyOrMethodSignature(r, s) {
        this.eat(n.question) && (r.optional = true);
        const o = r;
        if (this.match(n.parenL) || this.tsMatchLeftRelational()) {
          s && this.raise(r.start, ie.ReadonlyForMethodSignature);
          const u = o;
          u.kind && this.tsMatchLeftRelational() && this.raise(this.start, ie.AccesorCannotHaveTypeParameters), this.tsFillSignature(n.colon, u), this.tsParseTypeMemberSemicolon();
          const p = "parameters", f = "typeAnnotation";
          if (u.kind === "get") u[p].length > 0 && (this.raise(this.start, "A 'get' accesor must not have any formal parameters."), this.isThisParam(u[p][0]) && this.raise(this.start, ie.AccesorCannotDeclareThisParameter));
          else if (u.kind === "set") {
            if (u[p].length !== 1) this.raise(this.start, "A 'get' accesor must not have any formal parameters.");
            else {
              const g = u[p][0];
              this.isThisParam(g) && this.raise(this.start, ie.AccesorCannotDeclareThisParameter), g.type === "Identifier" && g.optional && this.raise(this.start, ie.SetAccesorCannotHaveOptionalParameter), g.type === "RestElement" && this.raise(this.start, ie.SetAccesorCannotHaveRestParameter);
            }
            u[f] && this.raise(u[f].start, ie.SetAccesorCannotHaveReturnType);
          } else u.kind = "method";
          return this.finishNode(u, "TSMethodSignature");
        } else {
          const u = o;
          s && (u.readonly = true);
          const p = this.tsTryParseTypeAnnotation();
          return p && (u.typeAnnotation = p), this.tsParseTypeMemberSemicolon(), this.finishNode(u, "TSPropertySignature");
        }
      }
      tsParseTypeMember() {
        const r = this.startNode();
        if (this.match(n.parenL) || this.tsMatchLeftRelational()) return this.tsParseSignatureMember("TSCallSignatureDeclaration", r);
        if (this.match(n._new)) {
          const o = this.startNode();
          return this.next(), this.match(n.parenL) || this.tsMatchLeftRelational() ? this.tsParseSignatureMember("TSConstructSignatureDeclaration", r) : (r.key = this.createIdentifier(o, "new"), this.tsParsePropertyOrMethodSignature(r, false));
        }
        this.tsParseModifiers({ modified: r, allowedModifiers: ["readonly"], disallowedModifiers: ["declare", "abstract", "private", "protected", "public", "static", "override"] });
        const s = this.tsTryParseIndexSignature(r);
        return s || (this.parsePropertyName(r), !r.computed && r.key.type === "Identifier" && (r.key.name === "get" || r.key.name === "set") && this.tsTokenCanFollowModifier() && (r.kind = r.key.name, this.parsePropertyName(r)), this.tsParsePropertyOrMethodSignature(r, !!r.readonly));
      }
      tsParseList(r, s) {
        const o = [];
        for (; !this.tsIsListTerminator(r); ) o.push(s());
        return o;
      }
      tsParseObjectTypeMembers() {
        this.expect(n.braceL);
        const r = this.tsParseList("TypeMembers", this.tsParseTypeMember.bind(this));
        return this.expect(n.braceR), r;
      }
      tsParseInterfaceDeclaration(r, s = {}) {
        if (this.hasFollowingLineBreak()) return null;
        this.expectContextual("interface"), s.declare && (r.declare = true), m(this.type) ? (r.id = this.parseIdent(), this.checkLValSimple(r.id, Ee.BIND_TS_INTERFACE)) : (r.id = null, this.raise(this.start, ie.MissingInterfaceName)), r.typeParameters = this.tsTryParseTypeParameters(this.tsParseInOutModifiers.bind(this)), this.eat(n._extends) && (r.extends = this.tsParseHeritageClause("extends"));
        const o = this.startNode();
        return o.body = this.tsInType(this.tsParseObjectTypeMembers.bind(this)), r.body = this.finishNode(o, "TSInterfaceBody"), this.finishNode(r, "TSInterfaceDeclaration");
      }
      tsParseAbstractDeclaration(r) {
        if (this.match(n._class)) return r.abstract = true, this.parseClass(r, true);
        if (this.ts_isContextual(E.interface)) {
          if (!this.hasFollowingLineBreak()) return r.abstract = true, this.tsParseInterfaceDeclaration(r);
        } else this.unexpected(r.start);
      }
      tsIsDeclarationStart() {
        return v(this.type);
      }
      tsParseExpressionStatement(r, s) {
        switch (s.name) {
          case "declare": {
            const o = this.tsTryParseDeclare(r);
            if (o) return o.declare = true, o;
            break;
          }
          case "global":
            if (this.match(n.braceL)) {
              this.enterScope(ur);
              const o = r;
              return o.global = true, o.id = s, o.body = this.tsParseModuleBlock(), super.exitScope(), this.finishNode(o, "TSModuleDeclaration");
            }
            break;
          default:
            return this.tsParseDeclaration(r, s.name, false);
        }
      }
      tsParseModuleReference() {
        return this.tsIsExternalModuleReference() ? this.tsParseExternalModuleReference() : this.tsParseEntityName(false);
      }
      tsIsExportDefaultSpecifier() {
        const { type: r } = this, s = this.isAsyncFunction(), o = this.isLet();
        if (m(r)) {
          if (s && !this.containsEsc || o) return false;
          if ((r === E.type || r === E.interface) && !this.containsEsc) {
            const f = this.lookahead();
            if (m(f.type) && !this.isContextualWithState("from", f) || f.type === n.braceL) return false;
          }
        } else if (!this.match(n._default)) return false;
        const u = this.nextTokenStart(), p = this.isUnparsedContextual(u, "from");
        if (this.input.charCodeAt(u) === 44 || m(this.type) && p) return true;
        if (this.match(n._default) && p) {
          const f = this.input.charCodeAt(this.nextTokenStartSince(u + 4));
          return f === 34 || f === 39;
        }
        return false;
      }
      tsInAmbientContext(r) {
        const s = this.isAmbientContext;
        this.isAmbientContext = true;
        try {
          return r();
        } finally {
          this.isAmbientContext = s;
        }
      }
      tsCheckLineTerminator(r) {
        return r ? this.hasFollowingLineBreak() ? false : (this.next(), true) : !this.isLineTerminator();
      }
      tsParseModuleOrNamespaceDeclaration(r, s = false) {
        if (r.id = this.parseIdent(), s || this.checkLValSimple(r.id, Ee.BIND_TS_NAMESPACE), this.eat(n.dot)) {
          const o = this.startNode();
          this.tsParseModuleOrNamespaceDeclaration(o, true), r.body = o;
        } else this.enterScope(ur), r.body = this.tsParseModuleBlock(), super.exitScope();
        return this.finishNode(r, "TSModuleDeclaration");
      }
      checkLValSimple(r, s = Ee.BIND_NONE, o) {
        return (r.type === "TSNonNullExpression" || r.type === "TSAsExpression") && (r = r.expression), super.checkLValSimple(r, s, o);
      }
      tsParseTypeAliasDeclaration(r) {
        return r.id = this.parseIdent(), this.checkLValSimple(r.id, Ee.BIND_TS_TYPE), r.typeAnnotation = this.tsInType(() => {
          if (r.typeParameters = this.tsTryParseTypeParameters(this.tsParseInOutModifiers.bind(this)), this.expect(n.eq), this.ts_isContextual(E.interface) && this.lookahead().type !== n.dot) {
            const s = this.startNode();
            return this.next(), this.finishNode(s, "TSIntrinsicKeyword");
          }
          return this.tsParseType();
        }), this.semicolon(), this.finishNode(r, "TSTypeAliasDeclaration");
      }
      tsParseDeclaration(r, s, o) {
        switch (s) {
          case "abstract":
            if (this.tsCheckLineTerminator(o) && (this.match(n._class) || m(this.type))) return this.tsParseAbstractDeclaration(r);
            break;
          case "module":
            if (this.tsCheckLineTerminator(o)) {
              if (this.match(n.string)) return this.tsParseAmbientExternalModuleDeclaration(r);
              if (m(this.type)) return this.tsParseModuleOrNamespaceDeclaration(r);
            }
            break;
          case "namespace":
            if (this.tsCheckLineTerminator(o) && m(this.type)) return this.tsParseModuleOrNamespaceDeclaration(r);
            break;
          case "type":
            if (this.tsCheckLineTerminator(o) && m(this.type)) return this.tsParseTypeAliasDeclaration(r);
            break;
        }
      }
      tsTryParseExportDeclaration() {
        return this.tsParseDeclaration(this.startNode(), this.value, true);
      }
      tsParseImportEqualsDeclaration(r, s) {
        r.isExport = s || false, r.id = this.parseIdent(), this.checkLValSimple(r.id, Ee.BIND_LEXICAL), super.expect(n.eq);
        const o = this.tsParseModuleReference();
        return r.importKind === "type" && o.type !== "TSExternalModuleReference" && this.raise(o.start, ie.ImportAliasHasImportType), r.moduleReference = o, super.semicolon(), this.finishNode(r, "TSImportEqualsDeclaration");
      }
      isExportDefaultSpecifier() {
        if (this.tsIsDeclarationStart()) return false;
        const { type: r } = this;
        if (m(r)) {
          if (this.isContextual("async") || this.isContextual("let")) return false;
          if ((r === E.type || r === E.interface) && !this.containsEsc) {
            const u = this.lookahead();
            if (m(u.type) && !this.isContextualWithState("from", u) || u.type === n.braceL) return false;
          }
        } else if (!this.match(n._default)) return false;
        const s = this.nextTokenStart(), o = this.isUnparsedContextual(s, "from");
        if (this.input.charCodeAt(s) === 44 || m(this.type) && o) return true;
        if (this.match(n._default) && o) {
          const u = this.input.charCodeAt(this.nextTokenStartSince(s + 4));
          return u === 34 || u === 39;
        }
        return false;
      }
      parseTemplate({ isTagged: r = false } = {}) {
        let s = this.startNode();
        this.next(), s.expressions = [];
        let o = this.parseTemplateElement({ isTagged: r });
        for (s.quasis = [o]; !o.tail; ) this.type === n.eof && this.raise(this.pos, "Unterminated template literal"), this.expect(n.dollarBraceL), s.expressions.push(this.inType ? this.tsParseType() : this.parseExpression()), this.expect(n.braceR), s.quasis.push(o = this.parseTemplateElement({ isTagged: r }));
        return this.next(), this.finishNode(s, "TemplateLiteral");
      }
      parseFunction(r, s, o, u, p) {
        this.initFunction(r), (this.ecmaVersion >= 9 || this.ecmaVersion >= 6 && !u) && (this.type === n.star && s & Yl && this.unexpected(), r.generator = this.eat(n.star)), this.ecmaVersion >= 8 && (r.async = !!u), s & No && (r.id = s & Y1 && this.type !== n.name ? null : this.parseIdent());
        let f = this.yieldPos, g = this.awaitPos, M = this.awaitIdentPos;
        const C = this.maybeInArrowParameters;
        this.maybeInArrowParameters = false, this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0, this.enterScope(ml(r.async, r.generator)), s & No || (r.id = this.type === n.name ? this.parseIdent() : null), this.parseFunctionParams(r);
        const y = s & No;
        return this.parseFunctionBody(r, o, false, p, { isFunctionDeclaration: y }), this.yieldPos = f, this.awaitPos = g, this.awaitIdentPos = M, s & No && r.id && !(s & Yl) && (r.body ? this.checkLValSimple(r.id, this.strict || r.generator || r.async ? this.treatFunctionsAsVar ? Ee.BIND_VAR : Ee.BIND_LEXICAL : Ee.BIND_FUNCTION) : this.checkLValSimple(r.id, Ee.BIND_NONE)), this.maybeInArrowParameters = C, this.finishNode(r, y ? "FunctionDeclaration" : "FunctionExpression");
      }
      parseFunctionBody(r, s = false, o = false, u = false, p) {
        this.match(n.colon) && (r.returnType = this.tsParseTypeOrTypePredicateAnnotation(n.colon));
        const f = (p == null ? void 0 : p.isFunctionDeclaration) ? "TSDeclareFunction" : (p == null ? void 0 : p.isClassMethod) ? "TSDeclareMethod" : void 0;
        return f && !this.match(n.braceL) && this.isLineTerminator() ? this.finishNode(r, f) : f === "TSDeclareFunction" && this.isAmbientContext && (this.raise(r.start, ie.DeclareFunctionHasImplementation), r.declare) ? (super.parseFunctionBody(r, s, o, false), this.finishNode(r, f)) : (super.parseFunctionBody(r, s, o, u), r);
      }
      parseNew() {
        var _a3;
        this.containsEsc && this.raiseRecoverable(this.start, "Escape sequence in keyword new");
        let r = this.startNode(), s = this.parseIdent(true);
        if (this.ecmaVersion >= 6 && this.eat(n.dot)) {
          r.meta = s;
          let g = this.containsEsc;
          return r.property = this.parseIdent(true), r.property.name !== "target" && this.raiseRecoverable(r.property.start, "The only valid meta property for new is 'new.target'"), g && this.raiseRecoverable(r.start, "'new.target' must not contain escaped characters"), this.allowNewDotTarget || this.raiseRecoverable(r.start, "'new.target' can only be used in functions and class static block"), this.finishNode(r, "MetaProperty");
        }
        let o = this.start, u = this.startLoc, p = this.type === n._import;
        r.callee = this.parseSubscripts(this.parseExprAtom(), o, u, true, false), p && r.callee.type === "ImportExpression" && this.raise(o, "Cannot use new with import()");
        const { callee: f } = r;
        return f.type === "TSInstantiationExpression" && !((_a3 = f.extra) == null ? void 0 : _a3.parenthesized) && (r.typeArguments = f.typeArguments, r.callee = f.expression), this.eat(n.parenL) ? r.arguments = this.parseExprList(n.parenR, this.ecmaVersion >= 8, false) : r.arguments = [], this.finishNode(r, "NewExpression");
      }
      parseExprOp(r, s, o, u, p) {
        if (n._in.binop > u && !this.hasPrecedingLineBreak()) {
          let f;
          if (this.isContextual("as") && (f = "TSAsExpression"), this.isContextual("satisfies") && (f = "TSSatisfiesExpression"), f) {
            const g = this.startNodeAt(s, o);
            g.expression = r;
            const M = this.tsTryNextParseConstantContext();
            return M ? g.typeAnnotation = M : g.typeAnnotation = this.tsNextThenParseType(), this.finishNode(g, f), this.reScan_lt_gt(), this.parseExprOp(g, s, o, u, p);
          }
        }
        return super.parseExprOp(r, s, o, u, p);
      }
      parseImportSpecifiers() {
        let r = [], s = true;
        if (d.tokenIsIdentifier(this.type) && (r.push(this.parseImportDefaultSpecifier()), !this.eat(n.comma))) return r;
        if (this.type === n.star) return r.push(this.parseImportNamespaceSpecifier()), r;
        for (this.expect(n.braceL); !this.eat(n.braceR); ) {
          if (s) s = false;
          else if (this.expect(n.comma), this.afterTrailingComma(n.braceR)) break;
          r.push(this.parseImportSpecifier());
        }
        return r;
      }
      parseImport(r) {
        let s = this.lookahead();
        if (r.importKind = "value", this.importOrExportOuterKind = "value", m(s.type) || this.match(n.star) || this.match(n.braceL)) {
          let o = this.lookahead(2);
          if (o.type !== n.comma && !this.isContextualWithState("from", o) && o.type !== n.eq && this.ts_eatContextualWithState("type", 1, s) && (this.importOrExportOuterKind = "type", r.importKind = "type", s = this.lookahead(), o = this.lookahead(2)), m(s.type) && o.type === n.eq) {
            this.next();
            const u = this.tsParseImportEqualsDeclaration(r);
            return this.importOrExportOuterKind = "value", u;
          }
        }
        return this.next(), this.type === n.string ? (r.specifiers = [], r.source = this.parseExprAtom()) : (r.specifiers = this.parseImportSpecifiers(), this.expectContextual("from"), r.source = this.type === n.string ? this.parseExprAtom() : this.unexpected()), this.parseMaybeImportAttributes(r), this.semicolon(), this.finishNode(r, "ImportDeclaration"), this.importOrExportOuterKind = "value", r.importKind === "type" && r.specifiers.length > 1 && r.specifiers[0].type === "ImportDefaultSpecifier" && this.raise(r.start, ie.TypeImportCannotSpecifyDefaultAndNamed), r;
      }
      parseExportDefaultDeclaration() {
        if (this.isAbstractClass()) {
          const r = this.startNode();
          return this.next(), r.abstract = true, this.parseClass(r, true);
        }
        if (this.match(E.interface)) {
          const r = this.tsParseInterfaceDeclaration(this.startNode());
          if (r) return r;
        }
        return super.parseExportDefaultDeclaration();
      }
      parseExportAllDeclaration(r, s) {
        return this.ecmaVersion >= 11 && (this.eatContextual("as") ? (r.exported = this.parseModuleExportName(), this.checkExport(s, r.exported, this.lastTokStart)) : r.exported = null), this.expectContextual("from"), this.type !== n.string && this.unexpected(), r.source = this.parseExprAtom(), this.parseMaybeImportAttributes(r), this.semicolon(), this.finishNode(r, "ExportAllDeclaration");
      }
      parseDynamicImport(r) {
        if (this.next(), r.source = this.parseMaybeAssign(), this.eat(n.comma)) {
          const s = this.parseExpression();
          r.arguments = [s];
        }
        if (!this.eat(n.parenR)) {
          const s = this.start;
          this.eat(n.comma) && this.eat(n.parenR) ? this.raiseRecoverable(s, "Trailing comma is not allowed in import()") : this.unexpected(s);
        }
        return this.finishNode(r, "ImportExpression");
      }
      parseExport(r, s) {
        let o = this.lookahead();
        if (this.ts_eatWithState(n._import, 2, o)) {
          this.ts_isContextual(E.type) && this.lookaheadCharCode() !== 61 ? (r.importKind = "type", this.importOrExportOuterKind = "type", this.next()) : (r.importKind = "value", this.importOrExportOuterKind = "value");
          const u = this.tsParseImportEqualsDeclaration(r, true);
          return this.importOrExportOuterKind = void 0, u;
        } else if (this.ts_eatWithState(n.eq, 2, o)) {
          const u = r;
          return u.expression = this.parseExpression(), this.semicolon(), this.importOrExportOuterKind = void 0, this.finishNode(u, "TSExportAssignment");
        } else if (this.ts_eatContextualWithState("as", 2, o)) {
          const u = r;
          return this.expectContextual("namespace"), u.id = this.parseIdent(), this.semicolon(), this.importOrExportOuterKind = void 0, this.finishNode(u, "TSNamespaceExportDeclaration");
        } else {
          if (this.ts_isContextualWithState(o, E.type) && this.lookahead(2).type === n.braceL ? (this.next(), this.importOrExportOuterKind = "type", r.exportKind = "type") : (this.importOrExportOuterKind = "value", r.exportKind = "value"), this.next(), this.eat(n.star)) return this.parseExportAllDeclaration(r, s);
          if (this.eat(n._default)) return this.checkExport(s, "default", this.lastTokStart), r.declaration = this.parseExportDefaultDeclaration(), this.finishNode(r, "ExportDefaultDeclaration");
          if (this.shouldParseExportStatement()) r.declaration = this.parseExportDeclaration(r), r.declaration.type === "VariableDeclaration" ? this.checkVariableExport(s, r.declaration.declarations) : this.checkExport(s, r.declaration.id, r.declaration.id.start), r.specifiers = [], r.source = null;
          else {
            if (r.declaration = null, r.specifiers = this.parseExportSpecifiers(s), this.eatContextual("from")) this.type !== n.string && this.unexpected(), r.source = this.parseExprAtom(), this.parseMaybeImportAttributes(r);
            else {
              for (let u of r.specifiers) this.checkUnreserved(u.local), this.checkLocalExport(u.local), u.local.type === "Literal" && this.raise(u.local.start, "A string literal cannot be used as an exported binding without `from`.");
              r.source = null;
            }
            this.semicolon();
          }
          return this.finishNode(r, "ExportNamedDeclaration");
        }
      }
      checkExport(r, s, o) {
        r && (typeof s != "string" && (s = s.type === "Identifier" ? s.name : s.value), r[s] = true);
      }
      parseMaybeDefault(r, s, o) {
        const u = super.parseMaybeDefault(r, s, o);
        return u.type === "AssignmentPattern" && u.typeAnnotation && u.right.start < u.typeAnnotation.start && this.raise(u.typeAnnotation.start, ie.TypeAnnotationAfterAssign), u;
      }
      typeCastToParameter(r) {
        var _a3;
        return r.expression.typeAnnotation = r.typeAnnotation, this.resetEndLocation(r.expression, r.typeAnnotation.end, (_a3 = r.typeAnnotation.loc) == null ? void 0 : _a3.end), r.expression;
      }
      toAssignableList(r, s) {
        for (let o = 0; o < r.length; o++) {
          const u = r[o];
          (u == null ? void 0 : u.type) === "TSTypeCastExpression" && (r[o] = this.typeCastToParameter(u));
        }
        return super.toAssignableList(r, s);
      }
      reportReservedArrowTypeParam(r) {
        var _a3;
        r.params.length === 1 && ((_a3 = r.extra) == null ? void 0 : _a3.trailingComma);
      }
      parseExprAtom(r, s, o) {
        if (this.type === E.jsxText) return this.jsx_parseText();
        if (this.type === E.jsxTagStart) return this.jsx_parseElement();
        if (this.type === E.at) return this.parseDecorators(), this.parseExprAtom();
        if (m(this.type)) {
          let u = this.potentialArrowAt === this.start, p = this.start, f = this.startLoc, g = this.containsEsc, M = this.parseIdent(false);
          if (this.ecmaVersion >= 8 && !g && M.name === "async" && !this.canInsertSemicolon() && this.eat(n._function)) return this.overrideContext(k.f_expr), this.parseFunction(this.startNodeAt(p, f), 0, false, true, s);
          if (u && !this.canInsertSemicolon()) {
            if (this.eat(n.arrow)) return this.parseArrowExpression(this.startNodeAt(p, f), [M], false, s);
            if (this.ecmaVersion >= 8 && M.name === "async" && this.type === n.name && !g && (!this.potentialArrowInForAwait || this.value !== "of" || this.containsEsc)) return M = this.parseIdent(false), (this.canInsertSemicolon() || !this.eat(n.arrow)) && this.unexpected(), this.parseArrowExpression(this.startNodeAt(p, f), [M], true, s);
          }
          return M;
        } else return super.parseExprAtom(r, s, o);
      }
      parseExprAtomDefault() {
        if (m(this.type)) {
          const r = this.potentialArrowAt === this.start, s = this.containsEsc, o = this.parseIdent();
          if (!s && o.name === "async" && !this.canInsertSemicolon()) {
            const { type: u } = this;
            if (u === n._function) return this.next(), this.parseFunction(this.startNodeAtNode(o), void 0, true, true);
            if (m(u)) if (this.lookaheadCharCode() === 61) {
              const p = this.parseIdent(false);
              return (this.canInsertSemicolon() || !this.eat(n.arrow)) && this.unexpected(), this.parseArrowExpression(this.startNodeAtNode(o), [p], true);
            } else return o;
          }
          return r && this.match(n.arrow) && !this.canInsertSemicolon() ? (this.next(), this.parseArrowExpression(this.startNodeAtNode(o), [o], false)) : o;
        } else this.unexpected();
      }
      parseIdentNode() {
        let r = this.startNode();
        if (_(this.type) && !((this.type.keyword === "class" || this.type.keyword === "function") && (this.lastTokEnd !== this.lastTokStart + 1 || this.input.charCodeAt(this.lastTokStart) !== 46))) r.name = this.value;
        else return super.parseIdentNode();
        return r;
      }
      parseVarStatement(r, s, o = false) {
        const { isAmbientContext: u } = this;
        this.next(), super.parseVar(r, false, s, o || u), this.semicolon();
        const p = this.finishNode(r, "VariableDeclaration");
        if (!u) return p;
        for (const { id: f, init: g } of p.declarations) g && (s !== "const" || f.typeAnnotation ? this.raise(g.start, ie.InitializerNotAllowedInAmbientContext) : g.type !== "StringLiteral" && g.type !== "BooleanLiteral" && g.type !== "NumericLiteral" && g.type !== "BigIntLiteral" && (g.type !== "TemplateLiteral" || g.expressions.length > 0) && !Q1(g) && this.raise(g.start, ie.ConstInitiailizerMustBeStringOrNumericLiteralOrLiteralEnumReference));
        return p;
      }
      parseStatement(r, s, o) {
        if (this.match(E.at) && this.parseDecorators(true), this.match(n._const) && this.isLookaheadContextual("enum")) {
          const u = this.startNode();
          return this.expect(n._const), this.tsParseEnumDeclaration(u, { const: true });
        }
        if (this.ts_isContextual(E.enum)) return this.tsParseEnumDeclaration(this.startNode());
        if (this.ts_isContextual(E.interface)) {
          const u = this.tsParseInterfaceDeclaration(this.startNode());
          if (u) return u;
        }
        return super.parseStatement(r, s, o);
      }
      parseAccessModifier() {
        return this.tsParseModifier(["public", "protected", "private"]);
      }
      parsePostMemberNameModifiers(r) {
        this.eat(n.question) && (r.optional = true), r.readonly && this.match(n.parenL) && this.raise(r.start, ie.ClassMethodHasReadonly), r.declare && this.match(n.parenL) && this.raise(r.start, ie.ClassMethodHasDeclare);
      }
      parseExpressionStatement(r, s) {
        return (s.type === "Identifier" ? this.tsParseExpressionStatement(r, s) : void 0) || super.parseExpressionStatement(r, s);
      }
      shouldParseExportStatement() {
        return this.tsIsDeclarationStart() || this.match(E.at) ? true : super.shouldParseExportStatement();
      }
      parseConditional(r, s, o, u, p) {
        if (this.eat(n.question)) {
          let f = this.startNodeAt(s, o);
          return f.test = r, f.consequent = this.parseMaybeAssign(), this.expect(n.colon), f.alternate = this.parseMaybeAssign(u), this.finishNode(f, "ConditionalExpression");
        }
        return r;
      }
      parseMaybeConditional(r, s) {
        let o = this.start, u = this.startLoc, p = this.parseExprOps(r, s);
        if (this.checkExpressionErrors(s)) return p;
        if (!this.maybeInArrowParameters || !this.match(n.question)) return this.parseConditional(p, o, u, r, s);
        const f = this.tryParse(() => this.parseConditional(p, o, u, r, s));
        return f.node ? (f.error && this.setLookaheadState(f.failState), f.node) : (f.error && this.setOptionalParametersError(s, f.error), p);
      }
      parseParenItem(r) {
        const s = this.start, o = this.startLoc;
        if (r = super.parseParenItem(r), this.eat(n.question) && (r.optional = true, this.resetEndLocation(r)), this.match(n.colon)) {
          const u = this.startNodeAt(s, o);
          return u.expression = r, u.typeAnnotation = this.tsParseTypeAnnotation(), this.finishNode(u, "TSTypeCastExpression");
        }
        return r;
      }
      parseExportDeclaration(r) {
        if (!this.isAmbientContext && this.ts_isContextual(E.declare)) return this.tsInAmbientContext(() => this.parseExportDeclaration(r));
        const s = this.start, o = this.startLoc, u = this.eatContextual("declare");
        u && (this.ts_isContextual(E.declare) || !this.shouldParseExportStatement()) && this.raise(this.start, ie.ExpectedAmbientAfterExportDeclare);
        const f = m(this.type) && this.tsTryParseExportDeclaration() || this.parseStatement(null);
        return f ? ((f.type === "TSInterfaceDeclaration" || f.type === "TSTypeAliasDeclaration" || u) && (r.exportKind = "type"), u && (this.resetStartLocation(f, s, o), f.declare = true), f) : null;
      }
      parseClassId(r, s) {
        if (!s && this.isContextual("implements")) return;
        super.parseClassId(r, s);
        const o = this.tsTryParseTypeParameters(this.tsParseInOutModifiers.bind(this));
        o && (r.typeParameters = o);
      }
      parseClassPropertyAnnotation(r) {
        r.optional || (this.value === "!" && this.eat(n.prefix) ? r.definite = true : this.eat(n.question) && (r.optional = true));
        const s = this.tsTryParseTypeAnnotation();
        s && (r.typeAnnotation = s);
      }
      parseClassField(r) {
        if (r.key.type === "PrivateIdentifier") r.abstract && this.raise(r.start, ie.PrivateElementHasAbstract), r.accessibility && this.raise(r.start, ie.PrivateElementHasAccessibility({ modifier: r.accessibility })), this.parseClassPropertyAnnotation(r);
        else if (this.parseClassPropertyAnnotation(r), this.isAmbientContext && !(r.readonly && !r.typeAnnotation) && this.match(n.eq) && this.raise(this.start, ie.DeclareClassFieldHasInitializer), r.abstract && this.match(n.eq)) {
          const { key: o } = r;
          this.raise(this.start, ie.AbstractPropertyHasInitializer({ propertyName: o.type === "Identifier" && !r.computed ? o.name : `[${this.input.slice(o.start, o.end)}]` }));
        }
        return super.parseClassField(r);
      }
      parseClassMethod(r, s, o, u) {
        const p = r.kind === "constructor", f = r.key.type === "PrivateIdentifier", g = this.tsTryParseTypeParameters(this.tsParseConstModifier);
        f ? (g && (r.typeParameters = g), r.accessibility && this.raise(r.start, ie.PrivateMethodsHasAccessibility({ modifier: r.accessibility }))) : g && p && this.raise(g.start, ie.ConstructorHasTypeParameters);
        const { declare: M = false, kind: C } = r;
        M && (C === "get" || C === "set") && this.raise(r.start, ie.DeclareAccessor({ kind: C })), g && (r.typeParameters = g);
        const y = r.key;
        r.kind === "constructor" ? (s && this.raise(y.start, "Constructor can't be a generator"), o && this.raise(y.start, "Constructor can't be an async method")) : r.static && Kl(r, "prototype") && this.raise(y.start, "Classes may not have a static property named prototype");
        const O = r.value = this.parseMethod(s, o, u, true, r);
        return r.kind === "get" && O.params.length !== 0 && this.raiseRecoverable(O.start, "getter should have no params"), r.kind === "set" && O.params.length !== 1 && this.raiseRecoverable(O.start, "setter should have exactly one param"), r.kind === "set" && O.params[0].type === "RestElement" && this.raiseRecoverable(O.params[0].start, "Setter cannot use rest params"), this.finishNode(r, "MethodDefinition");
      }
      isClassMethod() {
        return this.match(n.relational);
      }
      parseClassElement(r) {
        if (this.eat(n.semi)) return null;
        let s = this.startNode(), o = "", u = false, p = false, f = "method", g = false;
        const M = ["declare", "private", "public", "protected", "accessor", "override", "abstract", "readonly", "static"];
        g = !!this.tsParseModifiers({ modified: s, allowedModifiers: M, disallowedModifiers: ["in", "out"], stopOnStartOfClassStaticBlock: true, errorTemplate: ie.InvalidModifierOnTypeParameterPositions }).static;
        const y = () => {
          if (this.tsIsStartOfStaticBlocks()) {
            if (this.next(), this.next(), this.tsHasSomeModifiers(s, M) && this.raise(this.start, ie.StaticBlockCannotHaveModifier), this.ecmaVersion >= 13) return super.parseClassStaticBlock(s), s;
          } else {
            const O = this.tsTryParseIndexSignature(s);
            if (O) return s.abstract && this.raise(s.start, ie.IndexSignatureHasAbstract), s.accessibility && this.raise(s.start, ie.IndexSignatureHasAccessibility({ modifier: s.accessibility })), s.declare && this.raise(s.start, ie.IndexSignatureHasDeclare), s.override && this.raise(s.start, ie.IndexSignatureHasOverride), O;
            if (!this.inAbstractClass && s.abstract && this.raise(s.start, ie.NonAbstractClassHasAbstractMethod), s.override && (r || this.raise(s.start, ie.OverrideNotInSubClass)), s.static = g, g && (this.isClassElementNameStart() || this.type === n.star || (o = "static")), !o && this.ecmaVersion >= 8 && this.eatContextual("async") && ((this.isClassElementNameStart() || this.type === n.star) && !this.canInsertSemicolon() ? p = true : o = "async"), !o && (this.ecmaVersion >= 9 || !p) && this.eat(n.star) && (u = true), !o && !p && !u) {
              const N = this.value;
              (this.eatContextual("get") || this.eatContextual("set")) && (this.isClassElementNameStart() ? f = N : o = N);
            }
            if (o ? (s.computed = false, s.key = this.startNodeAt(this.lastTokStart, this.lastTokStartLoc), s.key.name = o, this.finishNode(s.key, "Identifier")) : this.parseClassElementName(s), this.parsePostMemberNameModifiers(s), this.isClassMethod() || this.ecmaVersion < 13 || this.type === n.parenL || f !== "method" || u || p) {
              const N = !s.static && Kl(s, "constructor"), V = N && r;
              N && f !== "method" && this.raise(s.key.start, "Constructor can't have get/set modifier"), s.kind = N ? "constructor" : f, this.parseClassMethod(s, u, p, V);
            } else this.parseClassField(s);
            return s;
          }
        };
        return s.declare ? this.tsInAmbientContext(y) : y(), s;
      }
      isClassElementNameStart() {
        return this.tsIsIdentifier() ? true : super.isClassElementNameStart();
      }
      parseClassSuper(r) {
        super.parseClassSuper(r), r.superClass && (this.tsMatchLeftRelational() || this.match(n.bitShift)) && (r.superTypeParameters = this.tsParseTypeArgumentsInExpression()), this.eatContextual("implements") && (r.implements = this.tsParseHeritageClause("implements"));
      }
      parseFunctionParams(r) {
        const s = this.tsTryParseTypeParameters(this.tsParseConstModifier);
        s && (r.typeParameters = s), super.parseFunctionParams(r);
      }
      parseVarId(r, s) {
        super.parseVarId(r, s), r.id.type === "Identifier" && !this.hasPrecedingLineBreak() && this.value === "!" && this.eat(n.prefix) && (r.definite = true);
        const o = this.tsTryParseTypeAnnotation();
        o && (r.id.typeAnnotation = o, this.resetEndLocation(r.id));
      }
      parseArrowExpression(r, s, o, u) {
        this.match(n.colon) && (r.returnType = this.tsParseTypeAnnotation());
        let p = this.yieldPos, f = this.awaitPos, g = this.awaitIdentPos;
        this.enterScope(ml(o, false) | Ee.SCOPE_ARROW), this.initFunction(r);
        const M = this.maybeInArrowParameters;
        return this.ecmaVersion >= 8 && (r.async = !!o), this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0, this.maybeInArrowParameters = true, r.params = this.toAssignableList(s, true), this.maybeInArrowParameters = false, this.parseFunctionBody(r, true, false, u), this.yieldPos = p, this.awaitPos = f, this.awaitIdentPos = g, this.maybeInArrowParameters = M, this.finishNode(r, "ArrowFunctionExpression");
      }
      parseMaybeAssignOrigin(r, s, o) {
        if (this.isContextual("yield")) {
          if (this.inGenerator) return this.parseYield(r);
          this.exprAllowed = false;
        }
        let u = false, p = -1, f = -1, g = -1;
        s ? (p = s.parenthesizedAssign, f = s.trailingComma, g = s.doubleProto, s.parenthesizedAssign = s.trailingComma = -1) : (s = new cr(), u = true);
        let M = this.start, C = this.startLoc;
        (this.type === n.parenL || m(this.type)) && (this.potentialArrowAt = this.start, this.potentialArrowInForAwait = r === "await");
        let y = this.parseMaybeConditional(r, s);
        if (o && (y = o.call(this, y, M, C)), this.type.isAssign) {
          let O = this.startNodeAt(M, C);
          return O.operator = this.value, this.type === n.eq && (y = this.toAssignable(y, true, s)), u || (s.parenthesizedAssign = s.trailingComma = s.doubleProto = -1), s.shorthandAssign >= y.start && (s.shorthandAssign = -1), this.maybeInArrowParameters || (this.type === n.eq ? this.checkLValPattern(y) : this.checkLValSimple(y)), O.left = y, this.next(), O.right = this.parseMaybeAssign(r), g > -1 && (s.doubleProto = g), this.finishNode(O, "AssignmentExpression");
        } else u && this.checkExpressionErrors(s, true);
        return p > -1 && (s.parenthesizedAssign = p), f > -1 && (s.trailingComma = f), y;
      }
      parseMaybeAssign(r, s, o) {
        let u, p;
        if (!this.tsMatchLeftRelational()) return this.parseMaybeAssignOrigin(r, s, o);
        (!u || this.compareLookaheadState(u, this.getCurLookaheadState())) && (u = this.cloneCurLookaheadState());
        let f;
        const g = this.tryParse((M) => {
          var _a3;
          f = this.tsParseTypeParameters(this.tsParseConstModifier);
          const C = this.parseMaybeAssignOrigin(r, s, o);
          return (C.type !== "ArrowFunctionExpression" || ((_a3 = C.extra) == null ? void 0 : _a3.parenthesized)) && M(), (f == null ? void 0 : f.params.length) !== 0 && this.resetStartLocationFromNode(C, f), C.typeParameters = f, C;
        }, u);
        if (!g.error && !g.aborted) return f && this.reportReservedArrowTypeParam(f), g.node;
        if (Xl(true), p = this.tryParse(() => this.parseMaybeAssignOrigin(r, s, o), u), !p.error) return p.node;
        if (g.node) return this.setLookaheadState(g.failState), f && this.reportReservedArrowTypeParam(f), g.node;
        if (p == null ? void 0 : p.node) return this.setLookaheadState(p.failState), p.node;
        throw g.thrown ? g.error : (p == null ? void 0 : p.thrown) ? p.error : g.error || (p == null ? void 0 : p.error);
      }
      parseAssignableListItem(r) {
        const s = [];
        for (; this.match(E.at); ) s.push(this.parseDecorator());
        const o = this.start, u = this.startLoc;
        let p, f = false, g = false;
        if (r !== void 0) {
          const y = {};
          this.tsParseModifiers({ modified: y, allowedModifiers: ["public", "private", "protected", "override", "readonly"] }), p = y.accessibility, g = y.override, f = y.readonly, r === false && (p || f || g) && this.raise(u.start, ie.UnexpectedParameterModifier);
        }
        const M = this.parseMaybeDefault(o, u);
        this.parseBindingListItem(M);
        const C = this.parseMaybeDefault(M.start, M.loc, M);
        if (s.length && (C.decorators = s), p || f || g) {
          const y = this.startNodeAt(o, u);
          return p && (y.accessibility = p), f && (y.readonly = f), g && (y.override = g), C.type !== "Identifier" && C.type !== "AssignmentPattern" && this.raise(y.start, ie.UnsupportedParameterPropertyKind), y.parameter = C, this.finishNode(y, "TSParameterProperty");
        }
        return C;
      }
      checkLValInnerPattern(r, s = Ee.BIND_NONE, o) {
        switch (r.type) {
          case "TSParameterProperty":
            this.checkLValInnerPattern(r.parameter, s, o);
            break;
          default: {
            super.checkLValInnerPattern(r, s, o);
            break;
          }
        }
      }
      parseBindingListItem(r) {
        this.eat(n.question) && (r.type !== "Identifier" && !this.isAmbientContext && !this.inType && this.raise(r.start, ie.PatternIsOptional), r.optional = true);
        const s = this.tsTryParseTypeAnnotation();
        return s && (r.typeAnnotation = s), this.resetEndLocation(r), r;
      }
      isAssignable(r, s) {
        switch (r.type) {
          case "TSTypeCastExpression":
            return this.isAssignable(r.expression, s);
          case "TSParameterProperty":
            return true;
          case "Identifier":
          case "ObjectPattern":
          case "ArrayPattern":
          case "AssignmentPattern":
          case "RestElement":
            return true;
          case "ObjectExpression": {
            const o = r.properties.length - 1;
            return r.properties.every((u, p) => u.type !== "ObjectMethod" && (p === o || u.type !== "SpreadElement") && this.isAssignable(u));
          }
          case "Property":
          case "ObjectProperty":
            return this.isAssignable(r.value);
          case "SpreadElement":
            return this.isAssignable(r.argument);
          case "ArrayExpression":
            return r.elements.every((o) => o === null || this.isAssignable(o));
          case "AssignmentExpression":
            return r.operator === "=";
          case "ParenthesizedExpression":
            return this.isAssignable(r.expression);
          case "MemberExpression":
          case "OptionalMemberExpression":
            return !s;
          default:
            return false;
        }
      }
      toAssignable(r, s = false, o = new cr()) {
        switch (r.type) {
          case "ParenthesizedExpression":
            return this.toAssignableParenthesizedExpression(r, s, o);
          case "TSAsExpression":
          case "TSSatisfiesExpression":
          case "TSNonNullExpression":
          case "TSTypeAssertion":
            return s || this.raise(r.start, ie.UnexpectedTypeCastInParameter), this.toAssignable(r.expression, s, o);
          case "MemberExpression":
            break;
          case "AssignmentExpression":
            return !s && r.left.type === "TSTypeCastExpression" && (r.left = this.typeCastToParameter(r.left)), super.toAssignable(r, s, o);
          case "TSTypeCastExpression":
            return this.typeCastToParameter(r);
          default:
            return super.toAssignable(r, s, o);
        }
        return r;
      }
      toAssignableParenthesizedExpression(r, s, o) {
        switch (r.expression.type) {
          case "TSAsExpression":
          case "TSSatisfiesExpression":
          case "TSNonNullExpression":
          case "TSTypeAssertion":
          case "ParenthesizedExpression":
            return this.toAssignable(r.expression, s, o);
          default:
            return super.toAssignable(r, s, o);
        }
      }
      parseBindingAtom() {
        switch (this.type) {
          case n._this:
            return this.parseIdent(true);
          default:
            return super.parseBindingAtom();
        }
      }
      shouldParseArrow(r) {
        let s;
        if (this.match(n.colon) ? s = r.every((o) => this.isAssignable(o, true)) : s = !this.canInsertSemicolon(), s) {
          if (this.match(n.colon)) {
            const o = this.tryParse((u) => {
              const p = this.tsParseTypeOrTypePredicateAnnotation(n.colon);
              return (this.canInsertSemicolon() || !this.match(n.arrow)) && u(), p;
            });
            if (o.aborted) return this.shouldParseArrowReturnType = void 0, false;
            o.thrown || (o.error && this.setLookaheadState(o.failState), this.shouldParseArrowReturnType = o.node);
          }
          return this.match(n.arrow) ? true : (this.shouldParseArrowReturnType = void 0, false);
        }
        return this.shouldParseArrowReturnType = void 0, s;
      }
      parseParenArrowList(r, s, o, u) {
        const p = this.startNodeAt(r, s);
        return p.returnType = this.shouldParseArrowReturnType, this.shouldParseArrowReturnType = void 0, this.parseArrowExpression(p, o, false, u);
      }
      parseParenAndDistinguishExpression(r, s) {
        let o = this.start, u = this.startLoc, p, f = this.ecmaVersion >= 8;
        if (this.ecmaVersion >= 6) {
          const g = this.maybeInArrowParameters;
          this.maybeInArrowParameters = true, this.next();
          let M = this.start, C = this.startLoc, y = [], O = true, N = false, V = new cr(), U = this.yieldPos, K = this.awaitPos, X;
          for (this.yieldPos = 0, this.awaitPos = 0; this.type !== n.parenR; ) if (O ? O = false : this.expect(n.comma), f && this.afterTrailingComma(n.parenR, true)) {
            N = true;
            break;
          } else if (this.type === n.ellipsis) {
            X = this.start, y.push(this.parseParenItem(this.parseRestBinding())), this.type === n.comma && this.raise(this.start, "Comma is not permitted after the rest element");
            break;
          } else y.push(this.parseMaybeAssign(s, V, this.parseParenItem));
          let H = this.lastTokEnd, re = this.lastTokEndLoc;
          if (this.expect(n.parenR), this.maybeInArrowParameters = g, r && this.shouldParseArrow(y) && this.eat(n.arrow)) return this.checkPatternErrors(V, false), this.checkYieldAwaitInDefaultParams(), this.yieldPos = U, this.awaitPos = K, this.parseParenArrowList(o, u, y, s);
          (!y.length || N) && this.unexpected(this.lastTokStart), X && this.unexpected(X), this.checkExpressionErrors(V, true), this.yieldPos = U || this.yieldPos, this.awaitPos = K || this.awaitPos, y.length > 1 ? (p = this.startNodeAt(M, C), p.expressions = y, this.finishNodeAt(p, "SequenceExpression", H, re)) : p = y[0];
        } else p = this.parseParenExpression();
        if (this.options.preserveParens) {
          let g = this.startNodeAt(o, u);
          return g.expression = p, this.finishNode(g, "ParenthesizedExpression");
        } else return p;
      }
      parseTaggedTemplateExpression(r, s, o, u) {
        const p = this.startNodeAt(s, o);
        return p.tag = r, p.quasi = this.parseTemplate({ isTagged: true }), u && this.raise(s, "Tagged Template Literals are not allowed in optionalChain."), this.finishNode(p, "TaggedTemplateExpression");
      }
      shouldParseAsyncArrow() {
        if (this.match(n.colon)) {
          const r = this.tryParse((s) => {
            const o = this.tsParseTypeOrTypePredicateAnnotation(n.colon);
            return (this.canInsertSemicolon() || !this.match(n.arrow)) && s(), o;
          });
          if (r.aborted) return this.shouldParseAsyncArrowReturnType = void 0, false;
          if (!r.thrown) return r.error && this.setLookaheadState(r.failState), this.shouldParseAsyncArrowReturnType = r.node, !this.canInsertSemicolon() && this.eat(n.arrow);
        } else return !this.canInsertSemicolon() && this.eat(n.arrow);
      }
      parseSubscriptAsyncArrow(r, s, o, u) {
        const p = this.startNodeAt(r, s);
        return p.returnType = this.shouldParseAsyncArrowReturnType, this.shouldParseAsyncArrowReturnType = void 0, this.parseArrowExpression(p, o, true, u);
      }
      parseExprList(r, s, o, u) {
        let p = [], f = true;
        for (; !this.eat(r); ) {
          if (f) f = false;
          else if (this.expect(n.comma), s && this.afterTrailingComma(r)) break;
          let g;
          o && this.type === n.comma ? g = null : this.type === n.ellipsis ? (g = this.parseSpread(u), this.maybeInArrowParameters && this.match(n.colon) && (g.typeAnnotation = this.tsParseTypeAnnotation()), u && this.type === n.comma && u.trailingComma < 0 && (u.trailingComma = this.start)) : g = this.parseMaybeAssign(false, u, this.parseParenItem), p.push(g);
        }
        return p;
      }
      parseSubscript(r, s, o, u, p, f, g) {
        let M = f;
        if (!this.hasPrecedingLineBreak() && this.value === "!" && this.match(n.prefix)) {
          this.exprAllowed = false, this.next();
          const V = this.startNodeAt(s, o);
          return V.expression = r, r = this.finishNode(V, "TSNonNullExpression"), r;
        }
        let C = false;
        if (this.match(n.questionDot) && this.lookaheadCharCode() === 60) {
          if (u) return r;
          r.optional = true, M = C = true, this.next();
        }
        if (this.tsMatchLeftRelational() || this.match(n.bitShift)) {
          let V;
          const U = this.tsTryParseAndCatch(() => {
            if (!u && this.atPossibleAsyncArrow(r)) {
              const re = this.tsTryParseGenericAsyncArrowFunction(s, o, g);
              if (re) return r = re, r;
            }
            const K = this.tsParseTypeArgumentsInExpression();
            if (!K) return r;
            if (C && !this.match(n.parenL)) return V = this.curPosition(), r;
            if (h(this.type) || this.type === n.backQuote) {
              const re = this.parseTaggedTemplateExpression(r, s, o, M);
              return re.typeArguments = K, re;
            }
            if (!u && this.eat(n.parenL)) {
              let re = new cr();
              const ue = this.startNodeAt(s, o);
              return ue.callee = r, ue.arguments = this.parseExprList(n.parenR, this.ecmaVersion >= 8, false, re), this.tsCheckForInvalidTypeCasts(ue.arguments), ue.typeArguments = K, M && (ue.optional = C), this.checkExpressionErrors(re, true), r = this.finishNode(ue, "CallExpression"), r;
            }
            const X = this.type;
            if (this.tsMatchRightRelational() || X === n.bitShift || X !== n.parenL && J1(X) && !this.hasPrecedingLineBreak()) return;
            const H = this.startNodeAt(s, o);
            return H.expression = r, H.typeArguments = K, this.finishNode(H, "TSInstantiationExpression");
          });
          if (V && this.unexpected(V), U) return U.type === "TSInstantiationExpression" && (this.match(n.dot) || this.match(n.questionDot) && this.lookaheadCharCode() !== 40) && this.raise(this.start, ie.InvalidPropertyAccessAfterInstantiationExpression), r = U, r;
        }
        let y = this.ecmaVersion >= 11, O = y && this.eat(n.questionDot);
        u && O && this.raise(this.lastTokStart, "Optional chaining cannot appear in the callee of new expressions");
        let N = this.eat(n.bracketL);
        if (N || O && this.type !== n.parenL && this.type !== n.backQuote || this.eat(n.dot)) {
          let V = this.startNodeAt(s, o);
          V.object = r, N ? (V.property = this.parseExpression(), this.expect(n.bracketR)) : this.type === n.privateId && r.type !== "Super" ? V.property = this.parsePrivateIdent() : V.property = this.parseIdent(this.options.allowReserved !== "never"), V.computed = !!N, y && (V.optional = O), r = this.finishNode(V, "MemberExpression");
        } else if (!u && this.eat(n.parenL)) {
          const V = this.maybeInArrowParameters;
          this.maybeInArrowParameters = true;
          let U = new cr(), K = this.yieldPos, X = this.awaitPos, H = this.awaitIdentPos;
          this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0;
          let re = this.parseExprList(n.parenR, this.ecmaVersion >= 8, false, U);
          if (p && !O && this.shouldParseAsyncArrow()) this.checkPatternErrors(U, false), this.checkYieldAwaitInDefaultParams(), this.awaitIdentPos > 0 && this.raise(this.awaitIdentPos, "Cannot use 'await' as identifier inside an async function"), this.yieldPos = K, this.awaitPos = X, this.awaitIdentPos = H, r = this.parseSubscriptAsyncArrow(s, o, re, g);
          else {
            this.checkExpressionErrors(U, true), this.yieldPos = K || this.yieldPos, this.awaitPos = X || this.awaitPos, this.awaitIdentPos = H || this.awaitIdentPos;
            let ue = this.startNodeAt(s, o);
            ue.callee = r, ue.arguments = re, y && (ue.optional = O), r = this.finishNode(ue, "CallExpression");
          }
          this.maybeInArrowParameters = V;
        } else if (this.type === n.backQuote) {
          (O || M) && this.raise(this.start, "Optional chaining cannot appear in the tag of tagged template expressions");
          let V = this.startNodeAt(s, o);
          V.tag = r, V.quasi = this.parseTemplate({ isTagged: true }), r = this.finishNode(V, "TaggedTemplateExpression");
        }
        return r;
      }
      parseGetterSetter(r) {
        r.kind = r.key.name, this.parsePropertyName(r), r.value = this.parseMethod(false);
        let s = r.kind === "get" ? 0 : 1;
        const o = r.value.params[0];
        if (s = o && this.isThisParam(o) ? s + 1 : s, r.value.params.length !== s) {
          let p = r.value.start;
          r.kind === "get" ? this.raiseRecoverable(p, "getter should have no params") : this.raiseRecoverable(p, "setter should have exactly one param");
        } else r.kind === "set" && r.value.params[0].type === "RestElement" && this.raiseRecoverable(r.value.params[0].start, "Setter cannot use rest params");
      }
      parseProperty(r, s) {
        if (!r) {
          let o = [];
          if (this.match(E.at)) for (; this.match(E.at); ) o.push(this.parseDecorator());
          const u = super.parseProperty(r, s);
          return u.type === "SpreadElement" && o.length && this.raise(u.start, $o.SpreadElementDecorator), o.length && (u.decorators = o, o = []), u;
        }
        return super.parseProperty(r, s);
      }
      parseCatchClauseParam() {
        const r = this.parseBindingAtom();
        let s = r.type === "Identifier";
        this.enterScope(s ? Ee.SCOPE_SIMPLE_CATCH : 0), this.checkLValPattern(r, s ? Ee.BIND_SIMPLE_CATCH : Ee.BIND_LEXICAL);
        const o = this.tsTryParseTypeAnnotation();
        return o && (r.typeAnnotation = o, this.resetEndLocation(r)), this.expect(n.parenR), r;
      }
      parseClass(r, s) {
        const o = this.inAbstractClass;
        this.inAbstractClass = !!r.abstract;
        try {
          this.next(), this.takeDecorators(r);
          const u = this.strict;
          this.strict = true, this.parseClassId(r, s), this.parseClassSuper(r);
          const p = this.enterClassBody(), f = this.startNode();
          let g = false;
          f.body = [];
          let M = [];
          for (this.expect(n.braceL); this.type !== n.braceR; ) {
            if (this.match(E.at)) {
              M.push(this.parseDecorator());
              continue;
            }
            const C = this.parseClassElement(r.superClass !== null);
            M.length && (C.decorators = M, this.resetStartLocationFromNode(C, M[0]), M = []), C && (f.body.push(C), C.type === "MethodDefinition" && C.kind === "constructor" && C.value.type === "FunctionExpression" ? (g && this.raiseRecoverable(C.start, "Duplicate constructor in the same class"), g = true, C.decorators && C.decorators.length > 0 && this.raise(C.start, $o.DecoratorConstructor)) : C.key && C.key.type === "PrivateIdentifier" && G1(p, C) && this.raiseRecoverable(C.key.start, `Identifier '#${C.key.name}' has already been declared`));
          }
          return this.strict = u, this.next(), M.length && this.raise(this.start, $o.TrailingDecorator), r.body = this.finishNode(f, "ClassBody"), this.exitClassBody(), this.finishNode(r, s ? "ClassDeclaration" : "ClassExpression");
        } finally {
          this.inAbstractClass = o;
        }
      }
      parseClassFunctionParams() {
        const r = this.tsTryParseTypeParameters();
        let s = this.parseBindingList(n.parenR, false, this.ecmaVersion >= 8, true);
        return r && (s.typeParameters = r), s;
      }
      parseMethod(r, s, o, u, p) {
        let f = this.startNode(), g = this.yieldPos, M = this.awaitPos, C = this.awaitIdentPos;
        if (this.initFunction(f), this.ecmaVersion >= 6 && (f.generator = r), this.ecmaVersion >= 8 && (f.async = !!s), this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0, this.enterScope(ml(s, f.generator) | Ee.SCOPE_SUPER | (o ? Ee.SCOPE_DIRECT_SUPER : 0)), this.expect(n.parenL), f.params = this.parseClassFunctionParams(), this.checkYieldAwaitInDefaultParams(), this.parseFunctionBody(f, false, true, false, { isClassMethod: u }), this.yieldPos = g, this.awaitPos = M, this.awaitIdentPos = C, p && p.abstract && !!f.body) {
          const { key: O } = p;
          this.raise(p.start, ie.AbstractMethodHasImplementation({ methodName: O.type === "Identifier" && !p.computed ? O.name : `[${this.input.slice(O.start, O.end)}]` }));
        }
        return this.finishNode(f, "FunctionExpression");
      }
      static parse(r, s) {
        if (s.locations === false) throw new Error("You have to enable options.locations while using acorn-typescript");
        s.locations = true;
        const o = new this(s, r);
        return t && (o.isAmbientContext = true), o.parse();
      }
      static parseExpressionAt(r, s, o) {
        if (o.locations === false) throw new Error("You have to enable options.locations while using acorn-typescript");
        o.locations = true;
        const u = new this(o, r, s);
        return t && (u.isAmbientContext = true), u.nextToken(), u.parseExpression();
      }
      parseImportSpecifier() {
        if (this.ts_isContextual(E.type)) {
          let s = this.startNode();
          return s.imported = this.parseModuleExportName(), this.parseTypeOnlyImportExportSpecifier(s, true, this.importOrExportOuterKind === "type"), this.finishNode(s, "ImportSpecifier");
        } else {
          const s = super.parseImportSpecifier();
          return s.importKind = "value", s;
        }
      }
      parseExportSpecifier(r) {
        const s = this.ts_isContextual(E.type);
        if (!this.match(n.string) && s) {
          let u = this.startNode();
          return u.local = this.parseModuleExportName(), this.parseTypeOnlyImportExportSpecifier(u, false, this.importOrExportOuterKind === "type"), this.finishNode(u, "ExportSpecifier"), this.checkExport(r, u.exported, u.exported.start), u;
        } else {
          const u = super.parseExportSpecifier(r);
          return u.exportKind = "value", u;
        }
      }
      parseTypeOnlyImportExportSpecifier(r, s, o) {
        const u = s ? "imported" : "local", p = s ? "local" : "exported";
        let f = r[u], g, M = false, C = true;
        const y = f.start;
        if (this.isContextual("as")) {
          const N = this.parseIdent();
          if (this.isContextual("as")) {
            const V = this.parseIdent();
            _(this.type) ? (M = true, f = N, g = s ? this.parseIdent() : this.parseModuleExportName(), C = false) : (g = V, C = false);
          } else _(this.type) ? (C = false, g = s ? this.parseIdent() : this.parseModuleExportName()) : (M = true, f = N);
        } else _(this.type) && (M = true, s ? (f = super.parseIdent(true), this.isContextual("as") || this.checkUnreserved(f)) : f = this.parseModuleExportName());
        M && o && this.raise(y, s ? ie.TypeModifierIsUsedInTypeImports : ie.TypeModifierIsUsedInTypeExports), r[u] = f, r[p] = g;
        const O = s ? "importKind" : "exportKind";
        r[O] = M ? "type" : "value", C && this.eatContextual("as") && (r[p] = s ? this.parseIdent() : this.parseModuleExportName()), r[p] || (r[p] = this.copyNode(r[u])), s && this.checkLValSimple(r[p], Ee.BIND_LEXICAL);
      }
      raiseCommonCheck(r, s, o) {
        switch (s) {
          case "Comma is not permitted after the rest element":
            if (this.isAmbientContext && this.match(n.comma) && this.lookaheadCharCode() === 41) {
              this.next();
              return;
            } else return super.raise(r, s);
        }
        return o ? super.raiseRecoverable(r, s) : super.raise(r, s);
      }
      raiseRecoverable(r, s) {
        return this.raiseCommonCheck(r, s, true);
      }
      raise(r, s) {
        return this.raiseCommonCheck(r, s, true);
      }
      updateContext(r) {
        const { type: s } = this;
        if (s == n.braceL) {
          var o = this.curContext();
          o == $.tc_oTag ? this.context.push(k.b_expr) : o == $.tc_expr ? this.context.push(k.b_tmpl) : super.updateContext(r), this.exprAllowed = true;
        } else if (s === n.slash && r === E.jsxTagStart) this.context.length -= 2, this.context.push($.tc_cTag), this.exprAllowed = false;
        else return super.updateContext(r);
      }
      jsx_parseOpeningElementAt(r, s) {
        let o = this.startNodeAt(r, s), u = this.jsx_parseElementName();
        if (u && (o.name = u), this.match(n.relational) || this.match(n.bitShift)) {
          const p = this.tsTryParseAndCatch(() => this.tsParseTypeArgumentsInExpression());
          p && (o.typeArguments = p);
        }
        for (o.attributes = []; this.type !== n.slash && this.type !== E.jsxTagEnd; ) o.attributes.push(this.jsx_parseAttribute());
        return o.selfClosing = this.eat(n.slash), this.expect(E.jsxTagEnd), this.finishNode(o, u ? "JSXOpeningElement" : "JSXOpeningFragment");
      }
      enterScope(r) {
        r === ur && this.importsStack.push([]), super.enterScope(r);
        const s = super.currentScope();
        s.types = [], s.enums = [], s.constEnums = [], s.classes = [], s.exportOnlyBindings = [];
      }
      exitScope() {
        super.currentScope().flags === ur && this.importsStack.pop(), super.exitScope();
      }
      hasImport(r, s) {
        const o = this.importsStack.length;
        if (this.importsStack[o - 1].indexOf(r) > -1) return true;
        if (!s && o > 1) {
          for (let u = 0; u < o - 1; u++) if (this.importsStack[u].indexOf(r) > -1) return true;
        }
        return false;
      }
      maybeExportDefined(r, s) {
        this.inModule && r.flags & Ee.SCOPE_TOP && this.undefinedExports.delete(s);
      }
      declareName(r, s, o) {
        if (s & Ee.BIND_FLAGS_TS_IMPORT) {
          this.hasImport(r, true) && this.raise(o, `Identifier '${r}' has already been declared.`), this.importsStack[this.importsStack.length - 1].push(r);
          return;
        }
        const u = this.currentScope();
        if (s & Ee.BIND_FLAGS_TS_EXPORT_ONLY) {
          this.maybeExportDefined(u, r), u.exportOnlyBindings.push(r);
          return;
        }
        s === Ee.BIND_TS_TYPE || s === Ee.BIND_TS_INTERFACE ? (s === Ee.BIND_TS_TYPE && u.types.includes(r) && this.raise(o, `type '${r}' has already been declared.`), u.types.push(r)) : super.declareName(r, s, o), s & Ee.BIND_FLAGS_TS_ENUM && u.enums.push(r), s & Ee.BIND_FLAGS_TS_CONST_ENUM && u.constEnums.push(r), s & Ee.BIND_FLAGS_CLASS && u.classes.push(r);
      }
      checkLocalExport(r) {
        const { name: s } = r;
        if (this.hasImport(s)) return;
        const o = this.scopeStack.length;
        for (let u = o - 1; u >= 0; u--) {
          const p = this.scopeStack[u];
          if (p.types.indexOf(s) > -1 || p.exportOnlyBindings.indexOf(s) > -1) return;
        }
        super.checkLocalExport(r);
      }
    }
    return A;
  };
}
Fe.extend(t0());
class r0 extends Error {
  constructor(t, a, i) {
    super(a);
    __publicField(this, "message", "");
    __privateAdd(this, _e);
    this.stack = "", __privateSet(this, _e, new wh(t, a, i)), Object.assign(this, __privateGet(this, _e)), this.name = "CompileError";
  }
  toString() {
    return __privateGet(this, _e).toString();
  }
  toJSON() {
    return __privateGet(this, _e).toJSON();
  }
}
_e = new WeakMap();
function Il(e, t, a) {
  throw new r0(t, a, void 0);
}
function a0(e, t) {
  Il(e, "options_invalid_value", `Invalid compiler option: ${t}
https://svelte.dev/e/options_invalid_value`);
}
function i0(e, t) {
  Il(e, "options_removed", `Invalid compiler option: ${t}
https://svelte.dev/e/options_removed`);
}
function s0(e, t) {
  Il(e, "options_unrecognised", `Unrecognised compiler option ${t}
https://svelte.dev/e/options_unrecognised`);
}
const n0 = { "CounterClockwiseContourIntegral;": 8755, "ClockwiseContourIntegral;": 8754, "DoubleLongLeftRightArrow;": 10234, "NotNestedGreaterGreater;": 10914, "DiacriticalDoubleAcute;": 733, "NotSquareSupersetEqual;": 8931, "CloseCurlyDoubleQuote;": 8221, "DoubleContourIntegral;": 8751, "FilledVerySmallSquare;": 9642, "NegativeVeryThinSpace;": 8203, "NotPrecedesSlantEqual;": 8928, "NotRightTriangleEqual;": 8941, "NotSucceedsSlantEqual;": 8929, "CapitalDifferentialD;": 8517, "DoubleLeftRightArrow;": 8660, "DoubleLongRightArrow;": 10233, "EmptyVerySmallSquare;": 9643, "NestedGreaterGreater;": 8811, "NotDoubleVerticalBar;": 8742, "NotGreaterSlantEqual;": 10878, "NotLeftTriangleEqual;": 8940, "NotSquareSubsetEqual;": 8930, "OpenCurlyDoubleQuote;": 8220, "ReverseUpEquilibrium;": 10607, "DoubleLongLeftArrow;": 10232, "DownLeftRightVector;": 10576, "LeftArrowRightArrow;": 8646, "NegativeMediumSpace;": 8203, "NotGreaterFullEqual;": 8807, "NotRightTriangleBar;": 10704, "RightArrowLeftArrow;": 8644, "SquareSupersetEqual;": 8850, "leftrightsquigarrow;": 8621, "DownRightTeeVector;": 10591, "DownRightVectorBar;": 10583, "LongLeftRightArrow;": 10231, "Longleftrightarrow;": 10234, "NegativeThickSpace;": 8203, "NotLeftTriangleBar;": 10703, "PrecedesSlantEqual;": 8828, "ReverseEquilibrium;": 8651, "RightDoubleBracket;": 10215, "RightDownTeeVector;": 10589, "RightDownVectorBar;": 10581, "RightTriangleEqual;": 8885, "SquareIntersection;": 8851, "SucceedsSlantEqual;": 8829, "blacktriangleright;": 9656, "longleftrightarrow;": 10231, "DoubleUpDownArrow;": 8661, "DoubleVerticalBar;": 8741, "DownLeftTeeVector;": 10590, "DownLeftVectorBar;": 10582, "FilledSmallSquare;": 9724, "GreaterSlantEqual;": 10878, "LeftDoubleBracket;": 10214, "LeftDownTeeVector;": 10593, "LeftDownVectorBar;": 10585, "LeftTriangleEqual;": 8884, "NegativeThinSpace;": 8203, "NotGreaterGreater;": 8811, "NotLessSlantEqual;": 10877, "NotNestedLessLess;": 10913, "NotReverseElement;": 8716, "NotSquareSuperset;": 8848, "NotTildeFullEqual;": 8775, "RightAngleBracket;": 10217, "RightUpDownVector;": 10575, "SquareSubsetEqual;": 8849, "VerticalSeparator;": 10072, "blacktriangledown;": 9662, "blacktriangleleft;": 9666, "leftrightharpoons;": 8651, "rightleftharpoons;": 8652, "twoheadrightarrow;": 8608, "DiacriticalAcute;": 180, "DiacriticalGrave;": 96, "DiacriticalTilde;": 732, "DoubleRightArrow;": 8658, "DownArrowUpArrow;": 8693, "EmptySmallSquare;": 9723, "GreaterEqualLess;": 8923, "GreaterFullEqual;": 8807, "LeftAngleBracket;": 10216, "LeftUpDownVector;": 10577, "LessEqualGreater;": 8922, "NonBreakingSpace;": 160, "NotPrecedesEqual;": 10927, "NotRightTriangle;": 8939, "NotSucceedsEqual;": 10928, "NotSucceedsTilde;": 8831, "NotSupersetEqual;": 8841, "RightTriangleBar;": 10704, "RightUpTeeVector;": 10588, "RightUpVectorBar;": 10580, "UnderParenthesis;": 9181, "UpArrowDownArrow;": 8645, "circlearrowright;": 8635, "downharpoonright;": 8642, "ntrianglerighteq;": 8941, "rightharpoondown;": 8641, "rightrightarrows;": 8649, "twoheadleftarrow;": 8606, "vartriangleright;": 8883, "CloseCurlyQuote;": 8217, "ContourIntegral;": 8750, "DoubleDownArrow;": 8659, "DoubleLeftArrow;": 8656, "DownRightVector;": 8641, "LeftRightVector;": 10574, "LeftTriangleBar;": 10703, "LeftUpTeeVector;": 10592, "LeftUpVectorBar;": 10584, "LowerRightArrow;": 8600, "NotGreaterEqual;": 8817, "NotGreaterTilde;": 8821, "NotHumpDownHump;": 8782, "NotLeftTriangle;": 8938, "NotSquareSubset;": 8847, "OverParenthesis;": 9180, "RightDownVector;": 8642, "ShortRightArrow;": 8594, "UpperRightArrow;": 8599, "bigtriangledown;": 9661, "circlearrowleft;": 8634, "curvearrowright;": 8631, "downharpoonleft;": 8643, "leftharpoondown;": 8637, "leftrightarrows;": 8646, "nLeftrightarrow;": 8654, "nleftrightarrow;": 8622, "ntrianglelefteq;": 8940, "rightleftarrows;": 8644, "rightsquigarrow;": 8605, "rightthreetimes;": 8908, "straightepsilon;": 1013, "trianglerighteq;": 8885, "vartriangleleft;": 8882, "DiacriticalDot;": 729, "DoubleRightTee;": 8872, "DownLeftVector;": 8637, "GreaterGreater;": 10914, "HorizontalLine;": 9472, "InvisibleComma;": 8291, "InvisibleTimes;": 8290, "LeftDownVector;": 8643, "LeftRightArrow;": 8596, "Leftrightarrow;": 8660, "LessSlantEqual;": 10877, "LongRightArrow;": 10230, "Longrightarrow;": 10233, "LowerLeftArrow;": 8601, "NestedLessLess;": 8810, "NotGreaterLess;": 8825, "NotLessGreater;": 8824, "NotSubsetEqual;": 8840, "NotVerticalBar;": 8740, "OpenCurlyQuote;": 8216, "ReverseElement;": 8715, "RightTeeVector;": 10587, "RightVectorBar;": 10579, "ShortDownArrow;": 8595, "ShortLeftArrow;": 8592, "SquareSuperset;": 8848, "TildeFullEqual;": 8773, "UpperLeftArrow;": 8598, "ZeroWidthSpace;": 8203, "curvearrowleft;": 8630, "doublebarwedge;": 8966, "downdownarrows;": 8650, "hookrightarrow;": 8618, "leftleftarrows;": 8647, "leftrightarrow;": 8596, "leftthreetimes;": 8907, "longrightarrow;": 10230, "looparrowright;": 8620, "nshortparallel;": 8742, "ntriangleright;": 8939, "rightarrowtail;": 8611, "rightharpoonup;": 8640, "trianglelefteq;": 8884, "upharpoonright;": 8638, "ApplyFunction;": 8289, "DifferentialD;": 8518, "DoubleLeftTee;": 10980, "DoubleUpArrow;": 8657, "LeftTeeVector;": 10586, "LeftVectorBar;": 10578, "LessFullEqual;": 8806, "LongLeftArrow;": 10229, "Longleftarrow;": 10232, "NotEqualTilde;": 8770, "NotTildeEqual;": 8772, "NotTildeTilde;": 8777, "Poincareplane;": 8460, "PrecedesEqual;": 10927, "PrecedesTilde;": 8830, "RightArrowBar;": 8677, "RightTeeArrow;": 8614, "RightTriangle;": 8883, "RightUpVector;": 8638, "SucceedsEqual;": 10928, "SucceedsTilde;": 8831, "SupersetEqual;": 8839, "UpEquilibrium;": 10606, "VerticalTilde;": 8768, "VeryThinSpace;": 8202, "bigtriangleup;": 9651, "blacktriangle;": 9652, "divideontimes;": 8903, "fallingdotseq;": 8786, "hookleftarrow;": 8617, "leftarrowtail;": 8610, "leftharpoonup;": 8636, "longleftarrow;": 10229, "looparrowleft;": 8619, "measuredangle;": 8737, "ntriangleleft;": 8938, "shortparallel;": 8741, "smallsetminus;": 8726, "triangleright;": 9657, "upharpoonleft;": 8639, "varsubsetneqq;": 10955, "varsupsetneqq;": 10956, "DownArrowBar;": 10515, "DownTeeArrow;": 8615, "ExponentialE;": 8519, "GreaterEqual;": 8805, "GreaterTilde;": 8819, "HilbertSpace;": 8459, "HumpDownHump;": 8782, "Intersection;": 8898, "LeftArrowBar;": 8676, "LeftTeeArrow;": 8612, "LeftTriangle;": 8882, "LeftUpVector;": 8639, "NotCongruent;": 8802, "NotHumpEqual;": 8783, "NotLessEqual;": 8816, "NotLessTilde;": 8820, "Proportional;": 8733, "RightCeiling;": 8969, "RoundImplies;": 10608, "ShortUpArrow;": 8593, "SquareSubset;": 8847, "UnderBracket;": 9141, "VerticalLine;": 124, "blacklozenge;": 10731, "exponentiale;": 8519, "risingdotseq;": 8787, "triangledown;": 9663, "triangleleft;": 9667, "varsubsetneq;": 8842, "varsupsetneq;": 8843, "CircleMinus;": 8854, "CircleTimes;": 8855, "Equilibrium;": 8652, "GreaterLess;": 8823, "LeftCeiling;": 8968, "LessGreater;": 8822, "MediumSpace;": 8287, "NotLessLess;": 8810, "NotPrecedes;": 8832, "NotSucceeds;": 8833, "NotSuperset;": 8835, "OverBracket;": 9140, "RightVector;": 8640, "Rrightarrow;": 8667, "RuleDelayed;": 10740, "SmallCircle;": 8728, "SquareUnion;": 8852, "SubsetEqual;": 8838, "UpDownArrow;": 8597, "Updownarrow;": 8661, "VerticalBar;": 8739, "backepsilon;": 1014, "blacksquare;": 9642, "circledcirc;": 8858, "circleddash;": 8861, "curlyeqprec;": 8926, "curlyeqsucc;": 8927, "diamondsuit;": 9830, "eqslantless;": 10901, "expectation;": 8496, "nRightarrow;": 8655, "nrightarrow;": 8603, "preccurlyeq;": 8828, "precnapprox;": 10937, "quaternions;": 8461, "straightphi;": 981, "succcurlyeq;": 8829, "succnapprox;": 10938, "thickapprox;": 8776, "updownarrow;": 8597, "Bernoullis;": 8492, "CirclePlus;": 8853, "EqualTilde;": 8770, "Fouriertrf;": 8497, "ImaginaryI;": 8520, "Laplacetrf;": 8466, "LeftVector;": 8636, "Lleftarrow;": 8666, "NotElement;": 8713, "NotGreater;": 8815, "Proportion;": 8759, "RightArrow;": 8594, "RightFloor;": 8971, "Rightarrow;": 8658, "ThickSpace;": 8287, "TildeEqual;": 8771, "TildeTilde;": 8776, "UnderBrace;": 9183, "UpArrowBar;": 10514, "UpTeeArrow;": 8613, "circledast;": 8859, "complement;": 8705, "curlywedge;": 8911, "eqslantgtr;": 10902, "gtreqqless;": 10892, "lessapprox;": 10885, "lesseqqgtr;": 10891, "lmoustache;": 9136, "longmapsto;": 10236, "mapstodown;": 8615, "mapstoleft;": 8612, "nLeftarrow;": 8653, "nleftarrow;": 8602, "nsubseteqq;": 10949, "nsupseteqq;": 10950, "precapprox;": 10935, "rightarrow;": 8594, "rmoustache;": 9137, "sqsubseteq;": 8849, "sqsupseteq;": 8850, "subsetneqq;": 10955, "succapprox;": 10936, "supsetneqq;": 10956, "upuparrows;": 8648, "varepsilon;": 1013, "varnothing;": 8709, "Backslash;": 8726, "CenterDot;": 183, "CircleDot;": 8857, "Congruent;": 8801, "Coproduct;": 8720, "DoubleDot;": 168, "DownArrow;": 8595, "DownBreve;": 785, "Downarrow;": 8659, "HumpEqual;": 8783, "LeftArrow;": 8592, "LeftFloor;": 8970, "Leftarrow;": 8656, "LessTilde;": 8818, "Mellintrf;": 8499, "MinusPlus;": 8723, "NotCupCap;": 8813, "NotExists;": 8708, "NotSubset;": 8834, "OverBrace;": 9182, "PlusMinus;": 177, "Therefore;": 8756, "ThinSpace;": 8201, "TripleDot;": 8411, "UnionPlus;": 8846, "backprime;": 8245, "backsimeq;": 8909, "bigotimes;": 10754, "centerdot;": 183, "checkmark;": 10003, "complexes;": 8450, "dotsquare;": 8865, "downarrow;": 8595, "gtrapprox;": 10886, "gtreqless;": 8923, "gvertneqq;": 8809, "heartsuit;": 9829, "leftarrow;": 8592, "lesseqgtr;": 8922, "lvertneqq;": 8808, "ngeqslant;": 10878, "nleqslant;": 10877, "nparallel;": 8742, "nshortmid;": 8740, "nsubseteq;": 8840, "nsupseteq;": 8841, "pitchfork;": 8916, "rationals;": 8474, "spadesuit;": 9824, "subseteqq;": 10949, "subsetneq;": 8842, "supseteqq;": 10950, "supsetneq;": 8843, "therefore;": 8756, "triangleq;": 8796, "varpropto;": 8733, "DDotrahd;": 10513, "DotEqual;": 8784, "Integral;": 8747, "LessLess;": 10913, "NotEqual;": 8800, "NotTilde;": 8769, "PartialD;": 8706, "Precedes;": 8826, "RightTee;": 8866, "Succeeds;": 8827, "SuchThat;": 8715, "Superset;": 8835, "Uarrocir;": 10569, "UnderBar;": 95, "andslope;": 10840, "angmsdaa;": 10664, "angmsdab;": 10665, "angmsdac;": 10666, "angmsdad;": 10667, "angmsdae;": 10668, "angmsdaf;": 10669, "angmsdag;": 10670, "angmsdah;": 10671, "angrtvbd;": 10653, "approxeq;": 8778, "awconint;": 8755, "backcong;": 8780, "barwedge;": 8965, "bbrktbrk;": 9142, "bigoplus;": 10753, "bigsqcup;": 10758, "biguplus;": 10756, "bigwedge;": 8896, "boxminus;": 8863, "boxtimes;": 8864, "bsolhsub;": 10184, "capbrcup;": 10825, "circledR;": 174, "circledS;": 9416, "cirfnint;": 10768, "clubsuit;": 9827, "cupbrcap;": 10824, "curlyvee;": 8910, "cwconint;": 8754, "doteqdot;": 8785, "dotminus;": 8760, "drbkarow;": 10512, "dzigrarr;": 10239, "elinters;": 9191, "emptyset;": 8709, "eqvparsl;": 10725, "fpartint;": 10765, "geqslant;": 10878, "gesdotol;": 10884, "gnapprox;": 10890, "hksearow;": 10533, "hkswarow;": 10534, "imagline;": 8464, "imagpart;": 8465, "infintie;": 10717, "integers;": 8484, "intercal;": 8890, "intlarhk;": 10775, "laemptyv;": 10676, "ldrushar;": 10571, "leqslant;": 10877, "lesdotor;": 10883, "llcorner;": 8990, "lnapprox;": 10889, "lrcorner;": 8991, "lurdshar;": 10570, "mapstoup;": 8613, "multimap;": 8888, "naturals;": 8469, "ncongdot;": 10861, "notindot;": 8949, "otimesas;": 10806, "parallel;": 8741, "plusacir;": 10787, "pointint;": 10773, "precneqq;": 10933, "precnsim;": 8936, "profalar;": 9006, "profline;": 8978, "profsurf;": 8979, "raemptyv;": 10675, "realpart;": 8476, "rppolint;": 10770, "rtriltri;": 10702, "scpolint;": 10771, "setminus;": 8726, "shortmid;": 8739, "smeparsl;": 10724, "sqsubset;": 8847, "sqsupset;": 8848, "subseteq;": 8838, "succneqq;": 10934, "succnsim;": 8937, "supseteq;": 8839, "thetasym;": 977, "thicksim;": 8764, "timesbar;": 10801, "triangle;": 9653, "triminus;": 10810, "trpezium;": 9186, "ulcorner;": 8988, "urcorner;": 8989, "varkappa;": 1008, "varsigma;": 962, "vartheta;": 977, "Because;": 8757, "Cayleys;": 8493, "Cconint;": 8752, "Cedilla;": 184, "Diamond;": 8900, "DownTee;": 8868, "Element;": 8712, "Epsilon;": 917, "Implies;": 8658, "LeftTee;": 8867, "NewLine;": 10, "NoBreak;": 8288, "NotLess;": 8814, "Omicron;": 927, "OverBar;": 8254, "Product;": 8719, "UpArrow;": 8593, "Uparrow;": 8657, "Upsilon;": 933, "alefsym;": 8501, "angrtvb;": 8894, "angzarr;": 9084, "asympeq;": 8781, "backsim;": 8765, "because;": 8757, "bemptyv;": 10672, "between;": 8812, "bigcirc;": 9711, "bigodot;": 10752, "bigstar;": 9733, "bnequiv;": 8801, "boxplus;": 8862, "ccupssm;": 10832, "cemptyv;": 10674, "cirscir;": 10690, "coloneq;": 8788, "congdot;": 10861, "cudarrl;": 10552, "cudarrr;": 10549, "cularrp;": 10557, "curarrm;": 10556, "dbkarow;": 10511, "ddagger;": 8225, "ddotseq;": 10871, "demptyv;": 10673, "diamond;": 8900, "digamma;": 989, "dotplus;": 8724, "dwangle;": 10662, "epsilon;": 949, "eqcolon;": 8789, "equivDD;": 10872, "gesdoto;": 10882, "gtquest;": 10876, "gtrless;": 8823, "harrcir;": 10568, "intprod;": 10812, "isindot;": 8949, "larrbfs;": 10527, "larrsim;": 10611, "lbrksld;": 10639, "lbrkslu;": 10637, "ldrdhar;": 10599, "lesdoto;": 10881, "lessdot;": 8918, "lessgtr;": 8822, "lesssim;": 8818, "lotimes;": 10804, "lozenge;": 9674, "ltquest;": 10875, "luruhar;": 10598, "maltese;": 10016, "minusdu;": 10794, "napprox;": 8777, "natural;": 9838, "nearrow;": 8599, "nexists;": 8708, "notinva;": 8713, "notinvb;": 8951, "notinvc;": 8950, "notniva;": 8716, "notnivb;": 8958, "notnivc;": 8957, "npolint;": 10772, "npreceq;": 10927, "nsqsube;": 8930, "nsqsupe;": 8931, "nsubset;": 8834, "nsucceq;": 10928, "nsupset;": 8835, "nvinfin;": 10718, "nvltrie;": 8884, "nvrtrie;": 8885, "nwarrow;": 8598, "olcross;": 10683, "omicron;": 959, "orderof;": 8500, "orslope;": 10839, "pertenk;": 8241, "planckh;": 8462, "pluscir;": 10786, "plussim;": 10790, "plustwo;": 10791, "precsim;": 8830, "quatint;": 10774, "questeq;": 8799, "rarrbfs;": 10528, "rarrsim;": 10612, "rbrksld;": 10638, "rbrkslu;": 10640, "rdldhar;": 10601, "realine;": 8475, "rotimes;": 10805, "ruluhar;": 10600, "searrow;": 8600, "simplus;": 10788, "simrarr;": 10610, "subedot;": 10947, "submult;": 10945, "subplus;": 10943, "subrarr;": 10617, "succsim;": 8831, "supdsub;": 10968, "supedot;": 10948, "suphsol;": 10185, "suphsub;": 10967, "suplarr;": 10619, "supmult;": 10946, "supplus;": 10944, "swarrow;": 8601, "topfork;": 10970, "triplus;": 10809, "tritime;": 10811, "uparrow;": 8593, "upsilon;": 965, "uwangle;": 10663, "vzigzag;": 10650, "zigrarr;": 8669, "Aacute;": 193, "Abreve;": 258, "Agrave;": 192, "Assign;": 8788, "Atilde;": 195, "Barwed;": 8966, "Bumpeq;": 8782, "Cacute;": 262, "Ccaron;": 268, "Ccedil;": 199, "Colone;": 10868, "Conint;": 8751, "CupCap;": 8781, "Dagger;": 8225, "Dcaron;": 270, "DotDot;": 8412, "Dstrok;": 272, "Eacute;": 201, "Ecaron;": 282, "Egrave;": 200, "Exists;": 8707, "ForAll;": 8704, "Gammad;": 988, "Gbreve;": 286, "Gcedil;": 290, "HARDcy;": 1066, "Hstrok;": 294, "Iacute;": 205, "Igrave;": 204, "Itilde;": 296, "Jsercy;": 1032, "Kcedil;": 310, "Lacute;": 313, "Lambda;": 923, "Lcaron;": 317, "Lcedil;": 315, "Lmidot;": 319, "Lstrok;": 321, "Nacute;": 323, "Ncaron;": 327, "Ncedil;": 325, "Ntilde;": 209, "Oacute;": 211, "Odblac;": 336, "Ograve;": 210, "Oslash;": 216, "Otilde;": 213, "Otimes;": 10807, "Racute;": 340, "Rarrtl;": 10518, "Rcaron;": 344, "Rcedil;": 342, "SHCHcy;": 1065, "SOFTcy;": 1068, "Sacute;": 346, "Scaron;": 352, "Scedil;": 350, "Square;": 9633, "Subset;": 8912, "Supset;": 8913, "Tcaron;": 356, "Tcedil;": 354, "Tstrok;": 358, "Uacute;": 218, "Ubreve;": 364, "Udblac;": 368, "Ugrave;": 217, "Utilde;": 360, "Vdashl;": 10982, "Verbar;": 8214, "Vvdash;": 8874, "Yacute;": 221, "Zacute;": 377, "Zcaron;": 381, "aacute;": 225, "abreve;": 259, "agrave;": 224, "andand;": 10837, "angmsd;": 8737, "angsph;": 8738, "apacir;": 10863, "approx;": 8776, "atilde;": 227, "barvee;": 8893, "barwed;": 8965, "becaus;": 8757, "bernou;": 8492, "bigcap;": 8898, "bigcup;": 8899, "bigvee;": 8897, "bkarow;": 10509, "bottom;": 8869, "bowtie;": 8904, "boxbox;": 10697, "bprime;": 8245, "brvbar;": 166, "bullet;": 8226, "bumpeq;": 8783, "cacute;": 263, "capand;": 10820, "capcap;": 10827, "capcup;": 10823, "capdot;": 10816, "ccaron;": 269, "ccedil;": 231, "circeq;": 8791, "cirmid;": 10991, "colone;": 8788, "commat;": 64, "compfn;": 8728, "conint;": 8750, "coprod;": 8720, "copysr;": 8471, "cularr;": 8630, "cupcap;": 10822, "cupcup;": 10826, "cupdot;": 8845, "curarr;": 8631, "curren;": 164, "cylcty;": 9005, "dagger;": 8224, "daleth;": 8504, "dcaron;": 271, "dfisht;": 10623, "divide;": 247, "divonx;": 8903, "dlcorn;": 8990, "dlcrop;": 8973, "dollar;": 36, "drcorn;": 8991, "drcrop;": 8972, "dstrok;": 273, "eacute;": 233, "easter;": 10862, "ecaron;": 283, "ecolon;": 8789, "egrave;": 232, "egsdot;": 10904, "elsdot;": 10903, "emptyv;": 8709, "emsp13;": 8196, "emsp14;": 8197, "eparsl;": 10723, "eqcirc;": 8790, "equals;": 61, "equest;": 8799, "female;": 9792, "ffilig;": 64259, "ffllig;": 64260, "forall;": 8704, "frac12;": 189, "frac13;": 8531, "frac14;": 188, "frac15;": 8533, "frac16;": 8537, "frac18;": 8539, "frac23;": 8532, "frac25;": 8534, "frac34;": 190, "frac35;": 8535, "frac38;": 8540, "frac45;": 8536, "frac56;": 8538, "frac58;": 8541, "frac78;": 8542, "gacute;": 501, "gammad;": 989, "gbreve;": 287, "gesdot;": 10880, "gesles;": 10900, "gtlPar;": 10645, "gtrarr;": 10616, "gtrdot;": 8919, "gtrsim;": 8819, "hairsp;": 8202, "hamilt;": 8459, "hardcy;": 1098, "hearts;": 9829, "hellip;": 8230, "hercon;": 8889, "homtht;": 8763, "horbar;": 8213, "hslash;": 8463, "hstrok;": 295, "hybull;": 8259, "hyphen;": 8208, "iacute;": 237, "igrave;": 236, "iiiint;": 10764, "iinfin;": 10716, "incare;": 8453, "inodot;": 305, "intcal;": 8890, "iquest;": 191, "isinsv;": 8947, "itilde;": 297, "jsercy;": 1112, "kappav;": 1008, "kcedil;": 311, "kgreen;": 312, "lAtail;": 10523, "lacute;": 314, "lagran;": 8466, "lambda;": 955, "langle;": 10216, "larrfs;": 10525, "larrhk;": 8617, "larrlp;": 8619, "larrpl;": 10553, "larrtl;": 8610, "latail;": 10521, "lbrace;": 123, "lbrack;": 91, "lcaron;": 318, "lcedil;": 316, "ldquor;": 8222, "lesdot;": 10879, "lesges;": 10899, "lfisht;": 10620, "lfloor;": 8970, "lharul;": 10602, "llhard;": 10603, "lmidot;": 320, "lmoust;": 9136, "loplus;": 10797, "lowast;": 8727, "lowbar;": 95, "lparlt;": 10643, "lrhard;": 10605, "lsaquo;": 8249, "lsquor;": 8218, "lstrok;": 322, "lthree;": 8907, "ltimes;": 8905, "ltlarr;": 10614, "ltrPar;": 10646, "mapsto;": 8614, "marker;": 9646, "mcomma;": 10793, "midast;": 42, "midcir;": 10992, "middot;": 183, "minusb;": 8863, "minusd;": 8760, "mnplus;": 8723, "models;": 8871, "mstpos;": 8766, "nVDash;": 8879, "nVdash;": 8878, "nacute;": 324, "nbumpe;": 8783, "ncaron;": 328, "ncedil;": 326, "nearhk;": 10532, "nequiv;": 8802, "nesear;": 10536, "nexist;": 8708, "nltrie;": 8940, "notinE;": 8953, "nparsl;": 11005, "nprcue;": 8928, "nrarrc;": 10547, "nrarrw;": 8605, "nrtrie;": 8941, "nsccue;": 8929, "nsimeq;": 8772, "ntilde;": 241, "numero;": 8470, "nvDash;": 8877, "nvHarr;": 10500, "nvdash;": 8876, "nvlArr;": 10498, "nvrArr;": 10499, "nwarhk;": 10531, "nwnear;": 10535, "oacute;": 243, "odblac;": 337, "odsold;": 10684, "ograve;": 242, "ominus;": 8854, "origof;": 8886, "oslash;": 248, "otilde;": 245, "otimes;": 8855, "parsim;": 10995, "percnt;": 37, "period;": 46, "permil;": 8240, "phmmat;": 8499, "planck;": 8463, "plankv;": 8463, "plusdo;": 8724, "plusdu;": 10789, "plusmn;": 177, "preceq;": 10927, "primes;": 8473, "prnsim;": 8936, "propto;": 8733, "prurel;": 8880, "puncsp;": 8200, "qprime;": 8279, "rAtail;": 10524, "racute;": 341, "rangle;": 10217, "rarrap;": 10613, "rarrfs;": 10526, "rarrhk;": 8618, "rarrlp;": 8620, "rarrpl;": 10565, "rarrtl;": 8611, "ratail;": 10522, "rbrace;": 125, "rbrack;": 93, "rcaron;": 345, "rcedil;": 343, "rdquor;": 8221, "rfisht;": 10621, "rfloor;": 8971, "rharul;": 10604, "rmoust;": 9137, "roplus;": 10798, "rpargt;": 10644, "rsaquo;": 8250, "rsquor;": 8217, "rthree;": 8908, "rtimes;": 8906, "sacute;": 347, "scaron;": 353, "scedil;": 351, "scnsim;": 8937, "searhk;": 10533, "seswar;": 10537, "sfrown;": 8994, "shchcy;": 1097, "sigmaf;": 962, "sigmav;": 962, "simdot;": 10858, "smashp;": 10803, "softcy;": 1100, "solbar;": 9023, "spades;": 9824, "sqcaps;": 8851, "sqcups;": 8852, "sqsube;": 8849, "sqsupe;": 8850, "square;": 9633, "squarf;": 9642, "ssetmn;": 8726, "ssmile;": 8995, "sstarf;": 8902, "subdot;": 10941, "subset;": 8834, "subsim;": 10951, "subsub;": 10965, "subsup;": 10963, "succeq;": 10928, "supdot;": 10942, "supset;": 8835, "supsim;": 10952, "supsub;": 10964, "supsup;": 10966, "swarhk;": 10534, "swnwar;": 10538, "target;": 8982, "tcaron;": 357, "tcedil;": 355, "telrec;": 8981, "there4;": 8756, "thetav;": 977, "thinsp;": 8201, "thksim;": 8764, "timesb;": 8864, "timesd;": 10800, "topbot;": 9014, "topcir;": 10993, "tprime;": 8244, "tridot;": 9708, "tstrok;": 359, "uacute;": 250, "ubreve;": 365, "udblac;": 369, "ufisht;": 10622, "ugrave;": 249, "ulcorn;": 8988, "ulcrop;": 8975, "urcorn;": 8989, "urcrop;": 8974, "utilde;": 361, "vangrt;": 10652, "varphi;": 981, "varrho;": 1009, "veebar;": 8891, "vellip;": 8942, "verbar;": 124, "vsubnE;": 10955, "vsubne;": 8842, "vsupnE;": 10956, "vsupne;": 8843, "wedbar;": 10847, "wedgeq;": 8793, "weierp;": 8472, "wreath;": 8768, "xoplus;": 10753, "xotime;": 10754, "xsqcup;": 10758, "xuplus;": 10756, "xwedge;": 8896, "yacute;": 253, "zacute;": 378, "zcaron;": 382, "zeetrf;": 8488, "AElig;": 198, Aacute: 193, "Acirc;": 194, Agrave: 192, "Alpha;": 913, "Amacr;": 256, "Aogon;": 260, "Aring;": 197, Atilde: 195, "Breve;": 728, Ccedil: 199, "Ccirc;": 264, "Colon;": 8759, "Cross;": 10799, "Dashv;": 10980, "Delta;": 916, Eacute: 201, "Ecirc;": 202, Egrave: 200, "Emacr;": 274, "Eogon;": 280, "Equal;": 10869, "Gamma;": 915, "Gcirc;": 284, "Hacek;": 711, "Hcirc;": 292, "IJlig;": 306, Iacute: 205, "Icirc;": 206, Igrave: 204, "Imacr;": 298, "Iogon;": 302, "Iukcy;": 1030, "Jcirc;": 308, "Jukcy;": 1028, "Kappa;": 922, Ntilde: 209, "OElig;": 338, Oacute: 211, "Ocirc;": 212, Ograve: 210, "Omacr;": 332, "Omega;": 937, Oslash: 216, Otilde: 213, "Prime;": 8243, "RBarr;": 10512, "Scirc;": 348, "Sigma;": 931, "THORN;": 222, "TRADE;": 8482, "TSHcy;": 1035, "Theta;": 920, "Tilde;": 8764, Uacute: 218, "Ubrcy;": 1038, "Ucirc;": 219, Ugrave: 217, "Umacr;": 362, "Union;": 8899, "Uogon;": 370, "UpTee;": 8869, "Uring;": 366, "VDash;": 8875, "Vdash;": 8873, "Wcirc;": 372, "Wedge;": 8896, Yacute: 221, "Ycirc;": 374, aacute: 225, "acirc;": 226, "acute;": 180, "aelig;": 230, agrave: 224, "aleph;": 8501, "alpha;": 945, "amacr;": 257, "amalg;": 10815, "angle;": 8736, "angrt;": 8735, "angst;": 197, "aogon;": 261, "aring;": 229, "asymp;": 8776, atilde: 227, "awint;": 10769, "bcong;": 8780, "bdquo;": 8222, "bepsi;": 1014, "blank;": 9251, "blk12;": 9618, "blk14;": 9617, "blk34;": 9619, "block;": 9608, "boxDL;": 9559, "boxDR;": 9556, "boxDl;": 9558, "boxDr;": 9555, "boxHD;": 9574, "boxHU;": 9577, "boxHd;": 9572, "boxHu;": 9575, "boxUL;": 9565, "boxUR;": 9562, "boxUl;": 9564, "boxUr;": 9561, "boxVH;": 9580, "boxVL;": 9571, "boxVR;": 9568, "boxVh;": 9579, "boxVl;": 9570, "boxVr;": 9567, "boxdL;": 9557, "boxdR;": 9554, "boxdl;": 9488, "boxdr;": 9484, "boxhD;": 9573, "boxhU;": 9576, "boxhd;": 9516, "boxhu;": 9524, "boxuL;": 9563, "boxuR;": 9560, "boxul;": 9496, "boxur;": 9492, "boxvH;": 9578, "boxvL;": 9569, "boxvR;": 9566, "boxvh;": 9532, "boxvl;": 9508, "boxvr;": 9500, "breve;": 728, brvbar: 166, "bsemi;": 8271, "bsime;": 8909, "bsolb;": 10693, "bumpE;": 10926, "bumpe;": 8783, "caret;": 8257, "caron;": 711, "ccaps;": 10829, ccedil: 231, "ccirc;": 265, "ccups;": 10828, "cedil;": 184, "check;": 10003, "clubs;": 9827, "colon;": 58, "comma;": 44, "crarr;": 8629, "cross;": 10007, "csube;": 10961, "csupe;": 10962, "ctdot;": 8943, "cuepr;": 8926, "cuesc;": 8927, "cupor;": 10821, curren: 164, "cuvee;": 8910, "cuwed;": 8911, "cwint;": 8753, "dashv;": 8867, "dblac;": 733, "ddarr;": 8650, "delta;": 948, "dharl;": 8643, "dharr;": 8642, "diams;": 9830, "disin;": 8946, divide: 247, "doteq;": 8784, "dtdot;": 8945, "dtrif;": 9662, "duarr;": 8693, "duhar;": 10607, "eDDot;": 10871, eacute: 233, "ecirc;": 234, "efDot;": 8786, egrave: 232, "emacr;": 275, "empty;": 8709, "eogon;": 281, "eplus;": 10865, "epsiv;": 1013, "eqsim;": 8770, "equiv;": 8801, "erDot;": 8787, "erarr;": 10609, "esdot;": 8784, "exist;": 8707, "fflig;": 64256, "filig;": 64257, "fjlig;": 102, "fllig;": 64258, "fltns;": 9649, "forkv;": 10969, frac12: 189, frac14: 188, frac34: 190, "frasl;": 8260, "frown;": 8994, "gamma;": 947, "gcirc;": 285, "gescc;": 10921, "gimel;": 8503, "gneqq;": 8809, "gnsim;": 8935, "grave;": 96, "gsime;": 10894, "gsiml;": 10896, "gtcir;": 10874, "gtdot;": 8919, "harrw;": 8621, "hcirc;": 293, "hoarr;": 8703, iacute: 237, "icirc;": 238, "iexcl;": 161, igrave: 236, "iiint;": 8749, "iiota;": 8489, "ijlig;": 307, "imacr;": 299, "image;": 8465, "imath;": 305, "imped;": 437, "infin;": 8734, "iogon;": 303, "iprod;": 10812, iquest: 191, "isinE;": 8953, "isins;": 8948, "isinv;": 8712, "iukcy;": 1110, "jcirc;": 309, "jmath;": 567, "jukcy;": 1108, "kappa;": 954, "lAarr;": 8666, "lBarr;": 10510, "langd;": 10641, "laquo;": 171, "larrb;": 8676, "lates;": 10925, "lbarr;": 10508, "lbbrk;": 10098, "lbrke;": 10635, "lceil;": 8968, "ldquo;": 8220, "lescc;": 10920, "lhard;": 8637, "lharu;": 8636, "lhblk;": 9604, "llarr;": 8647, "lltri;": 9722, "lneqq;": 8808, "lnsim;": 8934, "loang;": 10220, "loarr;": 8701, "lobrk;": 10214, "lopar;": 10629, "lrarr;": 8646, "lrhar;": 8651, "lrtri;": 8895, "lsime;": 10893, "lsimg;": 10895, "lsquo;": 8216, "ltcir;": 10873, "ltdot;": 8918, "ltrie;": 8884, "ltrif;": 9666, "mDDot;": 8762, "mdash;": 8212, "micro;": 181, middot: 183, "minus;": 8722, "mumap;": 8888, "nabla;": 8711, "napid;": 8779, "napos;": 329, "natur;": 9838, "nbump;": 8782, "ncong;": 8775, "ndash;": 8211, "neArr;": 8663, "nearr;": 8599, "nedot;": 8784, "nesim;": 8770, "ngeqq;": 8807, "ngsim;": 8821, "nhArr;": 8654, "nharr;": 8622, "nhpar;": 10994, "nlArr;": 8653, "nlarr;": 8602, "nleqq;": 8806, "nless;": 8814, "nlsim;": 8820, "nltri;": 8938, "notin;": 8713, "notni;": 8716, "npart;": 8706, "nprec;": 8832, "nrArr;": 8655, "nrarr;": 8603, "nrtri;": 8939, "nsime;": 8772, "nsmid;": 8740, "nspar;": 8742, "nsubE;": 10949, "nsube;": 8840, "nsucc;": 8833, "nsupE;": 10950, "nsupe;": 8841, ntilde: 241, "numsp;": 8199, "nvsim;": 8764, "nwArr;": 8662, "nwarr;": 8598, oacute: 243, "ocirc;": 244, "odash;": 8861, "oelig;": 339, "ofcir;": 10687, ograve: 242, "ohbar;": 10677, "olarr;": 8634, "olcir;": 10686, "oline;": 8254, "omacr;": 333, "omega;": 969, "operp;": 10681, "oplus;": 8853, "orarr;": 8635, "order;": 8500, oslash: 248, otilde: 245, "ovbar;": 9021, "parsl;": 11005, "phone;": 9742, "plusb;": 8862, "pluse;": 10866, plusmn: 177, "pound;": 163, "prcue;": 8828, "prime;": 8242, "prnap;": 10937, "prsim;": 8830, "quest;": 63, "rAarr;": 8667, "rBarr;": 10511, "radic;": 8730, "rangd;": 10642, "range;": 10661, "raquo;": 187, "rarrb;": 8677, "rarrc;": 10547, "rarrw;": 8605, "ratio;": 8758, "rbarr;": 10509, "rbbrk;": 10099, "rbrke;": 10636, "rceil;": 8969, "rdquo;": 8221, "reals;": 8477, "rhard;": 8641, "rharu;": 8640, "rlarr;": 8644, "rlhar;": 8652, "rnmid;": 10990, "roang;": 10221, "roarr;": 8702, "robrk;": 10215, "ropar;": 10630, "rrarr;": 8649, "rsquo;": 8217, "rtrie;": 8885, "rtrif;": 9656, "sbquo;": 8218, "sccue;": 8829, "scirc;": 349, "scnap;": 10938, "scsim;": 8831, "sdotb;": 8865, "sdote;": 10854, "seArr;": 8664, "searr;": 8600, "setmn;": 8726, "sharp;": 9839, "sigma;": 963, "simeq;": 8771, "simgE;": 10912, "simlE;": 10911, "simne;": 8774, "slarr;": 8592, "smile;": 8995, "smtes;": 10924, "sqcap;": 8851, "sqcup;": 8852, "sqsub;": 8847, "sqsup;": 8848, "srarr;": 8594, "starf;": 9733, "strns;": 175, "subnE;": 10955, "subne;": 8842, "supnE;": 10956, "supne;": 8843, "swArr;": 8665, "swarr;": 8601, "szlig;": 223, "theta;": 952, "thkap;": 8776, "thorn;": 254, "tilde;": 732, "times;": 215, "trade;": 8482, "trisb;": 10701, "tshcy;": 1115, "twixt;": 8812, uacute: 250, "ubrcy;": 1118, "ucirc;": 251, "udarr;": 8645, "udhar;": 10606, ugrave: 249, "uharl;": 8639, "uharr;": 8638, "uhblk;": 9600, "ultri;": 9720, "umacr;": 363, "uogon;": 371, "uplus;": 8846, "upsih;": 978, "uring;": 367, "urtri;": 9721, "utdot;": 8944, "utrif;": 9652, "uuarr;": 8648, "vBarv;": 10985, "vDash;": 8872, "varpi;": 982, "vdash;": 8866, "veeeq;": 8794, "vltri;": 8882, "vnsub;": 8834, "vnsup;": 8835, "vprop;": 8733, "vrtri;": 8883, "wcirc;": 373, "wedge;": 8743, "xcirc;": 9711, "xdtri;": 9661, "xhArr;": 10234, "xharr;": 10231, "xlArr;": 10232, "xlarr;": 10229, "xodot;": 10752, "xrArr;": 10233, "xrarr;": 10230, "xutri;": 9651, yacute: 253, "ycirc;": 375, AElig: 198, Acirc: 194, "Aopf;": 120120, Aring: 197, "Ascr;": 119964, "Auml;": 196, "Barv;": 10983, "Beta;": 914, "Bopf;": 120121, "Bscr;": 8492, "CHcy;": 1063, "COPY;": 169, "Cdot;": 266, "Copf;": 8450, "Cscr;": 119966, "DJcy;": 1026, "DScy;": 1029, "DZcy;": 1039, "Darr;": 8609, "Dopf;": 120123, "Dscr;": 119967, Ecirc: 202, "Edot;": 278, "Eopf;": 120124, "Escr;": 8496, "Esim;": 10867, "Euml;": 203, "Fopf;": 120125, "Fscr;": 8497, "GJcy;": 1027, "Gdot;": 288, "Gopf;": 120126, "Gscr;": 119970, "Hopf;": 8461, "Hscr;": 8459, "IEcy;": 1045, "IOcy;": 1025, Icirc: 206, "Idot;": 304, "Iopf;": 120128, "Iota;": 921, "Iscr;": 8464, "Iuml;": 207, "Jopf;": 120129, "Jscr;": 119973, "KHcy;": 1061, "KJcy;": 1036, "Kopf;": 120130, "Kscr;": 119974, "LJcy;": 1033, "Lang;": 10218, "Larr;": 8606, "Lopf;": 120131, "Lscr;": 8466, "Mopf;": 120132, "Mscr;": 8499, "NJcy;": 1034, "Nopf;": 8469, "Nscr;": 119977, Ocirc: 212, "Oopf;": 120134, "Oscr;": 119978, "Ouml;": 214, "Popf;": 8473, "Pscr;": 119979, "QUOT;": 34, "Qopf;": 8474, "Qscr;": 119980, "Rang;": 10219, "Rarr;": 8608, "Ropf;": 8477, "Rscr;": 8475, "SHcy;": 1064, "Sopf;": 120138, "Sqrt;": 8730, "Sscr;": 119982, "Star;": 8902, THORN: 222, "TScy;": 1062, "Topf;": 120139, "Tscr;": 119983, "Uarr;": 8607, Ucirc: 219, "Uopf;": 120140, "Upsi;": 978, "Uscr;": 119984, "Uuml;": 220, "Vbar;": 10987, "Vert;": 8214, "Vopf;": 120141, "Vscr;": 119985, "Wopf;": 120142, "Wscr;": 119986, "Xopf;": 120143, "Xscr;": 119987, "YAcy;": 1071, "YIcy;": 1031, "YUcy;": 1070, "Yopf;": 120144, "Yscr;": 119988, "Yuml;": 376, "ZHcy;": 1046, "Zdot;": 379, "Zeta;": 918, "Zopf;": 8484, "Zscr;": 119989, acirc: 226, acute: 180, aelig: 230, "andd;": 10844, "andv;": 10842, "ange;": 10660, "aopf;": 120146, "apid;": 8779, "apos;": 39, aring: 229, "ascr;": 119990, "auml;": 228, "bNot;": 10989, "bbrk;": 9141, "beta;": 946, "beth;": 8502, "bnot;": 8976, "bopf;": 120147, "boxH;": 9552, "boxV;": 9553, "boxh;": 9472, "boxv;": 9474, "bscr;": 119991, "bsim;": 8765, "bsol;": 92, "bull;": 8226, "bump;": 8782, "caps;": 8745, "cdot;": 267, cedil: 184, "cent;": 162, "chcy;": 1095, "cirE;": 10691, "circ;": 710, "cire;": 8791, "comp;": 8705, "cong;": 8773, "copf;": 120148, "copy;": 169, "cscr;": 119992, "csub;": 10959, "csup;": 10960, "cups;": 8746, "dArr;": 8659, "dHar;": 10597, "darr;": 8595, "dash;": 8208, "diam;": 8900, "djcy;": 1106, "dopf;": 120149, "dscr;": 119993, "dscy;": 1109, "dsol;": 10742, "dtri;": 9663, "dzcy;": 1119, "eDot;": 8785, "ecir;": 8790, ecirc: 234, "edot;": 279, "emsp;": 8195, "ensp;": 8194, "eopf;": 120150, "epar;": 8917, "epsi;": 949, "escr;": 8495, "esim;": 8770, "euml;": 235, "euro;": 8364, "excl;": 33, "flat;": 9837, "fnof;": 402, "fopf;": 120151, "fork;": 8916, "fscr;": 119995, "gdot;": 289, "geqq;": 8807, "gesl;": 8923, "gjcy;": 1107, "gnap;": 10890, "gneq;": 10888, "gopf;": 120152, "gscr;": 8458, "gsim;": 8819, "gtcc;": 10919, "gvnE;": 8809, "hArr;": 8660, "half;": 189, "harr;": 8596, "hbar;": 8463, "hopf;": 120153, "hscr;": 119997, icirc: 238, "iecy;": 1077, iexcl: 161, "imof;": 8887, "iocy;": 1105, "iopf;": 120154, "iota;": 953, "iscr;": 119998, "isin;": 8712, "iuml;": 239, "jopf;": 120155, "jscr;": 119999, "khcy;": 1093, "kjcy;": 1116, "kopf;": 120156, "kscr;": 12e4, "lArr;": 8656, "lHar;": 10594, "lang;": 10216, laquo: 171, "larr;": 8592, "late;": 10925, "lcub;": 123, "ldca;": 10550, "ldsh;": 8626, "leqq;": 8806, "lesg;": 8922, "ljcy;": 1113, "lnap;": 10889, "lneq;": 10887, "lopf;": 120157, "lozf;": 10731, "lpar;": 40, "lscr;": 120001, "lsim;": 8818, "lsqb;": 91, "ltcc;": 10918, "ltri;": 9667, "lvnE;": 8808, "macr;": 175, "male;": 9794, "malt;": 10016, micro: 181, "mlcp;": 10971, "mldr;": 8230, "mopf;": 120158, "mscr;": 120002, "nGtv;": 8811, "nLtv;": 8810, "nang;": 8736, "napE;": 10864, "nbsp;": 160, "ncap;": 10819, "ncup;": 10818, "ngeq;": 8817, "nges;": 10878, "ngtr;": 8815, "nisd;": 8954, "njcy;": 1114, "nldr;": 8229, "nleq;": 8816, "nles;": 10877, "nmid;": 8740, "nopf;": 120159, "npar;": 8742, "npre;": 10927, "nsce;": 10928, "nscr;": 120003, "nsim;": 8769, "nsub;": 8836, "nsup;": 8837, "ntgl;": 8825, "ntlg;": 8824, "nvap;": 8781, "nvge;": 8805, "nvgt;": 62, "nvle;": 8804, "nvlt;": 60, "oast;": 8859, "ocir;": 8858, ocirc: 244, "odiv;": 10808, "odot;": 8857, "ogon;": 731, "oint;": 8750, "omid;": 10678, "oopf;": 120160, "opar;": 10679, "ordf;": 170, "ordm;": 186, "oror;": 10838, "oscr;": 8500, "osol;": 8856, "ouml;": 246, "para;": 182, "part;": 8706, "perp;": 8869, "phiv;": 981, "plus;": 43, "popf;": 120161, pound: 163, "prap;": 10935, "prec;": 8826, "prnE;": 10933, "prod;": 8719, "prop;": 8733, "pscr;": 120005, "qint;": 10764, "qopf;": 120162, "qscr;": 120006, "quot;": 34, "rArr;": 8658, "rHar;": 10596, "race;": 8765, "rang;": 10217, raquo: 187, "rarr;": 8594, "rcub;": 125, "rdca;": 10551, "rdsh;": 8627, "real;": 8476, "rect;": 9645, "rhov;": 1009, "ring;": 730, "ropf;": 120163, "rpar;": 41, "rscr;": 120007, "rsqb;": 93, "rtri;": 9657, "scap;": 10936, "scnE;": 10934, "sdot;": 8901, "sect;": 167, "semi;": 59, "sext;": 10038, "shcy;": 1096, "sime;": 8771, "simg;": 10910, "siml;": 10909, "smid;": 8739, "smte;": 10924, "solb;": 10692, "sopf;": 120164, "spar;": 8741, "squf;": 9642, "sscr;": 120008, "star;": 9734, "subE;": 10949, "sube;": 8838, "succ;": 8827, "sung;": 9834, "sup1;": 185, "sup2;": 178, "sup3;": 179, "supE;": 10950, "supe;": 8839, szlig: 223, "tbrk;": 9140, "tdot;": 8411, thorn: 254, times: 215, "tint;": 8749, "toea;": 10536, "topf;": 120165, "tosa;": 10537, "trie;": 8796, "tscr;": 120009, "tscy;": 1094, "uArr;": 8657, "uHar;": 10595, "uarr;": 8593, ucirc: 251, "uopf;": 120166, "upsi;": 965, "uscr;": 120010, "utri;": 9653, "uuml;": 252, "vArr;": 8661, "vBar;": 10984, "varr;": 8597, "vert;": 124, "vopf;": 120167, "vscr;": 120011, "wopf;": 120168, "wscr;": 120012, "xcap;": 8898, "xcup;": 8899, "xmap;": 10236, "xnis;": 8955, "xopf;": 120169, "xscr;": 120013, "xvee;": 8897, "yacy;": 1103, "yicy;": 1111, "yopf;": 120170, "yscr;": 120014, "yucy;": 1102, "yuml;": 255, "zdot;": 380, "zeta;": 950, "zhcy;": 1078, "zopf;": 120171, "zscr;": 120015, "zwnj;": 8204, "AMP;": 38, "Acy;": 1040, "Afr;": 120068, "And;": 10835, Auml: 196, "Bcy;": 1041, "Bfr;": 120069, COPY: 169, "Cap;": 8914, "Cfr;": 8493, "Chi;": 935, "Cup;": 8915, "Dcy;": 1044, "Del;": 8711, "Dfr;": 120071, "Dot;": 168, "ENG;": 330, "ETH;": 208, "Ecy;": 1069, "Efr;": 120072, "Eta;": 919, Euml: 203, "Fcy;": 1060, "Ffr;": 120073, "Gcy;": 1043, "Gfr;": 120074, "Hat;": 94, "Hfr;": 8460, "Icy;": 1048, "Ifr;": 8465, "Int;": 8748, Iuml: 207, "Jcy;": 1049, "Jfr;": 120077, "Kcy;": 1050, "Kfr;": 120078, "Lcy;": 1051, "Lfr;": 120079, "Lsh;": 8624, "Map;": 10501, "Mcy;": 1052, "Mfr;": 120080, "Ncy;": 1053, "Nfr;": 120081, "Not;": 10988, "Ocy;": 1054, "Ofr;": 120082, Ouml: 214, "Pcy;": 1055, "Pfr;": 120083, "Phi;": 934, "Psi;": 936, QUOT: 34, "Qfr;": 120084, "REG;": 174, "Rcy;": 1056, "Rfr;": 8476, "Rho;": 929, "Rsh;": 8625, "Scy;": 1057, "Sfr;": 120086, "Sub;": 8912, "Sum;": 8721, "Sup;": 8913, "Tab;": 9, "Tau;": 932, "Tcy;": 1058, "Tfr;": 120087, "Ucy;": 1059, "Ufr;": 120088, Uuml: 220, "Vcy;": 1042, "Vee;": 8897, "Vfr;": 120089, "Wfr;": 120090, "Xfr;": 120091, "Ycy;": 1067, "Yfr;": 120092, "Zcy;": 1047, "Zfr;": 8488, "acE;": 8766, "acd;": 8767, "acy;": 1072, "afr;": 120094, "amp;": 38, "and;": 8743, "ang;": 8736, "apE;": 10864, "ape;": 8778, "ast;": 42, auml: 228, "bcy;": 1073, "bfr;": 120095, "bne;": 61, "bot;": 8869, "cap;": 8745, cent: 162, "cfr;": 120096, "chi;": 967, "cir;": 9675, copy: 169, "cup;": 8746, "dcy;": 1076, "deg;": 176, "dfr;": 120097, "die;": 168, "div;": 247, "dot;": 729, "ecy;": 1101, "efr;": 120098, "egs;": 10902, "ell;": 8467, "els;": 10901, "eng;": 331, "eta;": 951, "eth;": 240, euml: 235, "fcy;": 1092, "ffr;": 120099, "gEl;": 10892, "gap;": 10886, "gcy;": 1075, "gel;": 8923, "geq;": 8805, "ges;": 10878, "gfr;": 120100, "ggg;": 8921, "glE;": 10898, "gla;": 10917, "glj;": 10916, "gnE;": 8809, "gne;": 10888, "hfr;": 120101, "icy;": 1080, "iff;": 8660, "ifr;": 120102, "int;": 8747, iuml: 239, "jcy;": 1081, "jfr;": 120103, "kcy;": 1082, "kfr;": 120104, "lEg;": 10891, "lap;": 10885, "lat;": 10923, "lcy;": 1083, "leg;": 8922, "leq;": 8804, "les;": 10877, "lfr;": 120105, "lgE;": 10897, "lnE;": 8808, "lne;": 10887, "loz;": 9674, "lrm;": 8206, "lsh;": 8624, macr: 175, "map;": 8614, "mcy;": 1084, "mfr;": 120106, "mho;": 8487, "mid;": 8739, "nGg;": 8921, "nGt;": 8811, "nLl;": 8920, "nLt;": 8810, "nap;": 8777, nbsp: 160, "ncy;": 1085, "nfr;": 120107, "ngE;": 8807, "nge;": 8817, "ngt;": 8815, "nis;": 8956, "niv;": 8715, "nlE;": 8806, "nle;": 8816, "nlt;": 8814, "not;": 172, "npr;": 8832, "nsc;": 8833, "num;": 35, "ocy;": 1086, "ofr;": 120108, "ogt;": 10689, "ohm;": 937, "olt;": 10688, "ord;": 10845, ordf: 170, ordm: 186, "orv;": 10843, ouml: 246, "par;": 8741, para: 182, "pcy;": 1087, "pfr;": 120109, "phi;": 966, "piv;": 982, "prE;": 10931, "pre;": 10927, "psi;": 968, "qfr;": 120110, quot: 34, "rcy;": 1088, "reg;": 174, "rfr;": 120111, "rho;": 961, "rlm;": 8207, "rsh;": 8625, "scE;": 10932, "sce;": 10928, "scy;": 1089, sect: 167, "sfr;": 120112, "shy;": 173, "sim;": 8764, "smt;": 10922, "sol;": 47, "squ;": 9633, "sub;": 8834, "sum;": 8721, sup1: 185, sup2: 178, sup3: 179, "sup;": 8835, "tau;": 964, "tcy;": 1090, "tfr;": 120113, "top;": 8868, "ucy;": 1091, "ufr;": 120114, "uml;": 168, uuml: 252, "vcy;": 1074, "vee;": 8744, "vfr;": 120115, "wfr;": 120116, "xfr;": 120117, "ycy;": 1099, "yen;": 165, "yfr;": 120118, yuml: 255, "zcy;": 1079, "zfr;": 120119, "zwj;": 8205, AMP: 38, "DD;": 8517, ETH: 208, "GT;": 62, "Gg;": 8921, "Gt;": 8811, "Im;": 8465, "LT;": 60, "Ll;": 8920, "Lt;": 8810, "Mu;": 924, "Nu;": 925, "Or;": 10836, "Pi;": 928, "Pr;": 10939, REG: 174, "Re;": 8476, "Sc;": 10940, "Xi;": 926, "ac;": 8766, "af;": 8289, amp: 38, "ap;": 8776, "dd;": 8518, deg: 176, "ee;": 8519, "eg;": 10906, "el;": 10905, eth: 240, "gE;": 8807, "ge;": 8805, "gg;": 8811, "gl;": 8823, "gt;": 62, "ic;": 8291, "ii;": 8520, "in;": 8712, "it;": 8290, "lE;": 8806, "le;": 8804, "lg;": 8822, "ll;": 8810, "lt;": 60, "mp;": 8723, "mu;": 956, "ne;": 8800, "ni;": 8715, not: 172, "nu;": 957, "oS;": 9416, "or;": 8744, "pi;": 960, "pm;": 177, "pr;": 8826, reg: 174, "rx;": 8478, "sc;": 8827, shy: 173, uml: 168, "wp;": 8472, "wr;": 8768, "xi;": 958, yen: 165, GT: 62, LT: 60, gt: 62, lt: 60 };
function o0(e, t) {
  return t && !e.endsWith(";") ? `${e}\\b(?!=)` : e;
}
function pm(e) {
  const t = "#(?:x[a-fA-F\\d]+|\\d+)(?:;)?", a = Object.keys(n0).map((l) => o0(l, e));
  return new RegExp(`&(${t}|${a.join("|")})`, "g");
}
pm(false);
pm(true);
const l0 = /* @__PURE__ */ new Map([["svelte:head", "SvelteHead"], ["svelte:options", "SvelteOptions"], ["svelte:window", "SvelteWindow"], ["svelte:document", "SvelteDocument"], ["svelte:body", "SvelteBody"]]);
new Map([...l0, ["svelte:element", "SvelteElement"], ["svelte:component", "SvelteComponent"], ["svelte:self", "SvelteSelf"], ["svelte:fragment", "SvelteFragment"], ["svelte:boundary", "SvelteBoundary"]]);
var lt = {}, dr = {}, Do = {}, pr = {}, Jl;
function u0() {
  if (Jl) return pr;
  Jl = 1, Object.defineProperty(pr, "__esModule", { value: true }), pr.default = void 0;
  function e() {
    var t = this, a = 0, i = { "@@iterator": function() {
      return i;
    }, next: function() {
      if (a < t.length) {
        var d = t[a];
        return a = a + 1, { done: false, value: d };
      } else return { done: true };
    } };
    return i;
  }
  return pr.default = e, pr;
}
var Zl;
function Io() {
  if (Zl) return Do;
  Zl = 1, Object.defineProperty(Do, "__esModule", { value: true }), Do.default = i;
  var e = t(u0());
  function t(l) {
    return l && l.__esModule ? l : { default: l };
  }
  function a(l) {
    "@babel/helpers - typeof";
    return a = typeof Symbol == "function" && typeof Symbol.iterator == "symbol" ? function(d) {
      return typeof d;
    } : function(d) {
      return d && typeof Symbol == "function" && d.constructor === Symbol && d !== Symbol.prototype ? "symbol" : typeof d;
    }, a(l);
  }
  function i(l, d) {
    return typeof Symbol == "function" && a(Symbol.iterator) === "symbol" && Object.defineProperty(l, Symbol.iterator, { value: e.default.bind(d) }), l;
  }
  return Do;
}
var eu;
function c0() {
  if (eu) return dr;
  eu = 1, Object.defineProperty(dr, "__esModule", { value: true }), dr.default = void 0;
  var e = t(Io());
  function t(b) {
    return b && b.__esModule ? b : { default: b };
  }
  function a(b, k) {
    return P(b) || n(b, k) || l(b, k) || i();
  }
  function i() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function l(b, k) {
    if (b) {
      if (typeof b == "string") return d(b, k);
      var I = {}.toString.call(b).slice(8, -1);
      return I === "Object" && b.constructor && (I = b.constructor.name), I === "Map" || I === "Set" ? Array.from(b) : I === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(I) ? d(b, k) : void 0;
    }
  }
  function d(b, k) {
    (k == null || k > b.length) && (k = b.length);
    for (var I = 0, E = Array(k); I < k; I++) E[I] = b[I];
    return E;
  }
  function n(b, k) {
    var I = b == null ? null : typeof Symbol < "u" && b[Symbol.iterator] || b["@@iterator"];
    if (I != null) {
      var E, $, D, S, h = [], v = true, m = false;
      try {
        if (D = (I = I.call(b)).next, k === 0) {
          if (Object(I) !== I) return;
          v = false;
        } else for (; !(v = (E = D.call(I)).done) && (h.push(E.value), h.length !== k); v = true) ;
      } catch (_) {
        m = true, $ = _;
      } finally {
        try {
          if (!v && I.return != null && (S = I.return(), Object(S) !== S)) return;
        } finally {
          if (m) throw $;
        }
      }
      return h;
    }
  }
  function P(b) {
    if (Array.isArray(b)) return b;
  }
  var x = [["aria-activedescendant", { type: "id" }], ["aria-atomic", { type: "boolean" }], ["aria-autocomplete", { type: "token", values: ["inline", "list", "both", "none"] }], ["aria-braillelabel", { type: "string" }], ["aria-brailleroledescription", { type: "string" }], ["aria-busy", { type: "boolean" }], ["aria-checked", { type: "tristate" }], ["aria-colcount", { type: "integer" }], ["aria-colindex", { type: "integer" }], ["aria-colspan", { type: "integer" }], ["aria-controls", { type: "idlist" }], ["aria-current", { type: "token", values: ["page", "step", "location", "date", "time", true, false] }], ["aria-describedby", { type: "idlist" }], ["aria-description", { type: "string" }], ["aria-details", { type: "id" }], ["aria-disabled", { type: "boolean" }], ["aria-dropeffect", { type: "tokenlist", values: ["copy", "execute", "link", "move", "none", "popup"] }], ["aria-errormessage", { type: "id" }], ["aria-expanded", { type: "boolean", allowundefined: true }], ["aria-flowto", { type: "idlist" }], ["aria-grabbed", { type: "boolean", allowundefined: true }], ["aria-haspopup", { type: "token", values: [false, true, "menu", "listbox", "tree", "grid", "dialog"] }], ["aria-hidden", { type: "boolean", allowundefined: true }], ["aria-invalid", { type: "token", values: ["grammar", false, "spelling", true] }], ["aria-keyshortcuts", { type: "string" }], ["aria-label", { type: "string" }], ["aria-labelledby", { type: "idlist" }], ["aria-level", { type: "integer" }], ["aria-live", { type: "token", values: ["assertive", "off", "polite"] }], ["aria-modal", { type: "boolean" }], ["aria-multiline", { type: "boolean" }], ["aria-multiselectable", { type: "boolean" }], ["aria-orientation", { type: "token", values: ["vertical", "undefined", "horizontal"] }], ["aria-owns", { type: "idlist" }], ["aria-placeholder", { type: "string" }], ["aria-posinset", { type: "integer" }], ["aria-pressed", { type: "tristate" }], ["aria-readonly", { type: "boolean" }], ["aria-relevant", { type: "tokenlist", values: ["additions", "all", "removals", "text"] }], ["aria-required", { type: "boolean" }], ["aria-roledescription", { type: "string" }], ["aria-rowcount", { type: "integer" }], ["aria-rowindex", { type: "integer" }], ["aria-rowspan", { type: "integer" }], ["aria-selected", { type: "boolean", allowundefined: true }], ["aria-setsize", { type: "integer" }], ["aria-sort", { type: "token", values: ["ascending", "descending", "none", "other"] }], ["aria-valuemax", { type: "number" }], ["aria-valuemin", { type: "number" }], ["aria-valuenow", { type: "number" }], ["aria-valuetext", { type: "string" }]], T = { entries: function() {
    return x;
  }, forEach: function(k) {
    for (var I = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, E = 0, $ = x; E < $.length; E++) {
      var D = a($[E], 2), S = D[0], h = D[1];
      k.call(I, h, S, x);
    }
  }, get: function(k) {
    var I = x.filter(function(E) {
      return E[0] === k;
    })[0];
    return I && I[1];
  }, has: function(k) {
    return !!T.get(k);
  }, keys: function() {
    return x.map(function(k) {
      var I = a(k, 1), E = I[0];
      return E;
    });
  }, values: function() {
    return x.map(function(k) {
      var I = a(k, 2), E = I[1];
      return E;
    });
  } };
  return dr.default = (0, e.default)(T, T.entries()), dr;
}
var fr = {}, tu;
function d0() {
  if (tu) return fr;
  tu = 1, Object.defineProperty(fr, "__esModule", { value: true }), fr.default = void 0;
  var e = t(Io());
  function t(b) {
    return b && b.__esModule ? b : { default: b };
  }
  function a(b, k) {
    return P(b) || n(b, k) || l(b, k) || i();
  }
  function i() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function l(b, k) {
    if (b) {
      if (typeof b == "string") return d(b, k);
      var I = {}.toString.call(b).slice(8, -1);
      return I === "Object" && b.constructor && (I = b.constructor.name), I === "Map" || I === "Set" ? Array.from(b) : I === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(I) ? d(b, k) : void 0;
    }
  }
  function d(b, k) {
    (k == null || k > b.length) && (k = b.length);
    for (var I = 0, E = Array(k); I < k; I++) E[I] = b[I];
    return E;
  }
  function n(b, k) {
    var I = b == null ? null : typeof Symbol < "u" && b[Symbol.iterator] || b["@@iterator"];
    if (I != null) {
      var E, $, D, S, h = [], v = true, m = false;
      try {
        if (D = (I = I.call(b)).next, k === 0) {
          if (Object(I) !== I) return;
          v = false;
        } else for (; !(v = (E = D.call(I)).done) && (h.push(E.value), h.length !== k); v = true) ;
      } catch (_) {
        m = true, $ = _;
      } finally {
        try {
          if (!v && I.return != null && (S = I.return(), Object(S) !== S)) return;
        } finally {
          if (m) throw $;
        }
      }
      return h;
    }
  }
  function P(b) {
    if (Array.isArray(b)) return b;
  }
  var x = [["a", { reserved: false }], ["abbr", { reserved: false }], ["acronym", { reserved: false }], ["address", { reserved: false }], ["applet", { reserved: false }], ["area", { reserved: false }], ["article", { reserved: false }], ["aside", { reserved: false }], ["audio", { reserved: false }], ["b", { reserved: false }], ["base", { reserved: true }], ["bdi", { reserved: false }], ["bdo", { reserved: false }], ["big", { reserved: false }], ["blink", { reserved: false }], ["blockquote", { reserved: false }], ["body", { reserved: false }], ["br", { reserved: false }], ["button", { reserved: false }], ["canvas", { reserved: false }], ["caption", { reserved: false }], ["center", { reserved: false }], ["cite", { reserved: false }], ["code", { reserved: false }], ["col", { reserved: true }], ["colgroup", { reserved: true }], ["content", { reserved: false }], ["data", { reserved: false }], ["datalist", { reserved: false }], ["dd", { reserved: false }], ["del", { reserved: false }], ["details", { reserved: false }], ["dfn", { reserved: false }], ["dialog", { reserved: false }], ["dir", { reserved: false }], ["div", { reserved: false }], ["dl", { reserved: false }], ["dt", { reserved: false }], ["em", { reserved: false }], ["embed", { reserved: false }], ["fieldset", { reserved: false }], ["figcaption", { reserved: false }], ["figure", { reserved: false }], ["font", { reserved: false }], ["footer", { reserved: false }], ["form", { reserved: false }], ["frame", { reserved: false }], ["frameset", { reserved: false }], ["h1", { reserved: false }], ["h2", { reserved: false }], ["h3", { reserved: false }], ["h4", { reserved: false }], ["h5", { reserved: false }], ["h6", { reserved: false }], ["head", { reserved: true }], ["header", { reserved: false }], ["hgroup", { reserved: false }], ["hr", { reserved: false }], ["html", { reserved: true }], ["i", { reserved: false }], ["iframe", { reserved: false }], ["img", { reserved: false }], ["input", { reserved: false }], ["ins", { reserved: false }], ["kbd", { reserved: false }], ["keygen", { reserved: false }], ["label", { reserved: false }], ["legend", { reserved: false }], ["li", { reserved: false }], ["link", { reserved: true }], ["main", { reserved: false }], ["map", { reserved: false }], ["mark", { reserved: false }], ["marquee", { reserved: false }], ["menu", { reserved: false }], ["menuitem", { reserved: false }], ["meta", { reserved: true }], ["meter", { reserved: false }], ["nav", { reserved: false }], ["noembed", { reserved: true }], ["noscript", { reserved: true }], ["object", { reserved: false }], ["ol", { reserved: false }], ["optgroup", { reserved: false }], ["option", { reserved: false }], ["output", { reserved: false }], ["p", { reserved: false }], ["param", { reserved: true }], ["picture", { reserved: true }], ["pre", { reserved: false }], ["progress", { reserved: false }], ["q", { reserved: false }], ["rp", { reserved: false }], ["rt", { reserved: false }], ["rtc", { reserved: false }], ["ruby", { reserved: false }], ["s", { reserved: false }], ["samp", { reserved: false }], ["script", { reserved: true }], ["section", { reserved: false }], ["select", { reserved: false }], ["small", { reserved: false }], ["source", { reserved: true }], ["spacer", { reserved: false }], ["span", { reserved: false }], ["strike", { reserved: false }], ["strong", { reserved: false }], ["style", { reserved: true }], ["sub", { reserved: false }], ["summary", { reserved: false }], ["sup", { reserved: false }], ["table", { reserved: false }], ["tbody", { reserved: false }], ["td", { reserved: false }], ["textarea", { reserved: false }], ["tfoot", { reserved: false }], ["th", { reserved: false }], ["thead", { reserved: false }], ["time", { reserved: false }], ["title", { reserved: true }], ["tr", { reserved: false }], ["track", { reserved: true }], ["tt", { reserved: false }], ["u", { reserved: false }], ["ul", { reserved: false }], ["var", { reserved: false }], ["video", { reserved: false }], ["wbr", { reserved: false }], ["xmp", { reserved: false }]], T = { entries: function() {
    return x;
  }, forEach: function(k) {
    for (var I = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, E = 0, $ = x; E < $.length; E++) {
      var D = a($[E], 2), S = D[0], h = D[1];
      k.call(I, h, S, x);
    }
  }, get: function(k) {
    var I = x.filter(function(E) {
      return E[0] === k;
    })[0];
    return I && I[1];
  }, has: function(k) {
    return !!T.get(k);
  }, keys: function() {
    return x.map(function(k) {
      var I = a(k, 1), E = I[0];
      return E;
    });
  }, values: function() {
    return x.map(function(k) {
      var I = a(k, 2), E = I[1];
      return E;
    });
  } };
  return fr.default = (0, e.default)(T, T.entries()), fr;
}
var hr = {}, mr = {}, vr = {}, ru;
function p0() {
  if (ru) return vr;
  ru = 1, Object.defineProperty(vr, "__esModule", { value: true }), vr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget"]] };
  return vr.default = e, vr;
}
var br = {}, au;
function f0() {
  if (au) return br;
  au = 1, Object.defineProperty(br, "__esModule", { value: true }), br.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-activedescendant": null, "aria-disabled": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget"]] };
  return br.default = e, br;
}
var yr = {}, iu;
function h0() {
  if (iu) return yr;
  iu = 1, Object.defineProperty(yr, "__esModule", { value: true }), yr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null }, relatedConcepts: [{ concept: { name: "input" }, module: "XForms" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget"]] };
  return yr.default = e, yr;
}
var gr = {}, su;
function m0() {
  if (su) return gr;
  su = 1, Object.defineProperty(gr, "__esModule", { value: true }), gr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return gr.default = e, gr;
}
var Rr = {}, nu;
function v0() {
  if (nu) return Rr;
  nu = 1, Object.defineProperty(Rr, "__esModule", { value: true }), Rr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-valuemax": null, "aria-valuemin": null, "aria-valuenow": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return Rr.default = e, Rr;
}
var xr = {}, ou;
function b0() {
  if (ou) return xr;
  ou = 1, Object.defineProperty(xr, "__esModule", { value: true }), xr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: [], prohibitedProps: [], props: { "aria-atomic": null, "aria-busy": null, "aria-controls": null, "aria-current": null, "aria-describedby": null, "aria-details": null, "aria-dropeffect": null, "aria-flowto": null, "aria-grabbed": null, "aria-hidden": null, "aria-keyshortcuts": null, "aria-label": null, "aria-labelledby": null, "aria-live": null, "aria-owns": null, "aria-relevant": null, "aria-roledescription": null }, relatedConcepts: [{ concept: { name: "role" }, module: "XHTML" }, { concept: { name: "type" }, module: "Dublin Core" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [] };
  return xr.default = e, xr;
}
var _r = {}, lu;
function y0() {
  if (lu) return _r;
  lu = 1, Object.defineProperty(_r, "__esModule", { value: true }), _r.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: [], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "frontmatter" }, module: "DTB" }, { concept: { name: "level" }, module: "DTB" }, { concept: { name: "level" }, module: "SMIL" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return _r.default = e, _r;
}
var Cr = {}, uu;
function g0() {
  if (uu) return Cr;
  uu = 1, Object.defineProperty(Cr, "__esModule", { value: true }), Cr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return Cr.default = e, Cr;
}
var Pr = {}, cu;
function R0() {
  if (cu) return Pr;
  cu = 1, Object.defineProperty(Pr, "__esModule", { value: true }), Pr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-orientation": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "composite"], ["roletype", "structure", "section", "group"]] };
  return Pr.default = e, Pr;
}
var qr = {}, du;
function x0() {
  if (du) return qr;
  du = 1, Object.defineProperty(qr, "__esModule", { value: true }), qr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: [], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype"]] };
  return qr.default = e, qr;
}
var Tr = {}, pu;
function _0() {
  if (pu) return Tr;
  pu = 1, Object.defineProperty(Tr, "__esModule", { value: true }), Tr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: [], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype"]] };
  return Tr.default = e, Tr;
}
var Sr = {}, fu;
function C0() {
  if (fu) return Sr;
  fu = 1, Object.defineProperty(Sr, "__esModule", { value: true }), Sr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-modal": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype"]] };
  return Sr.default = e, Sr;
}
var hu;
function P0() {
  if (hu) return mr;
  hu = 1, Object.defineProperty(mr, "__esModule", { value: true }), mr.default = void 0;
  var e = I(p0()), t = I(f0()), a = I(h0()), i = I(m0()), l = I(v0()), d = I(b0()), n = I(y0()), P = I(g0()), x = I(R0()), T = I(x0()), b = I(_0()), k = I(C0());
  function I($) {
    return $ && $.__esModule ? $ : { default: $ };
  }
  var E = [["command", e.default], ["composite", t.default], ["input", a.default], ["landmark", i.default], ["range", l.default], ["roletype", d.default], ["section", n.default], ["sectionhead", P.default], ["select", x.default], ["structure", T.default], ["widget", b.default], ["window", k.default]];
  return mr.default = E, mr;
}
var Ar = {}, wr = {}, mu;
function q0() {
  if (mu) return wr;
  mu = 1, Object.defineProperty(wr, "__esModule", { value: true }), wr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-atomic": "true", "aria-live": "assertive" }, relatedConcepts: [{ concept: { name: "alert" }, module: "XForms" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return wr.default = e, wr;
}
var Er = {}, vu;
function T0() {
  if (vu) return Er;
  vu = 1, Object.defineProperty(Er, "__esModule", { value: true }), Er.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "alert" }, module: "XForms" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "alert"], ["roletype", "window", "dialog"]] };
  return Er.default = e, Er;
}
var kr = {}, bu;
function S0() {
  if (bu) return kr;
  bu = 1, Object.defineProperty(kr, "__esModule", { value: true }), kr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-activedescendant": null, "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "Device Independence Delivery Unit" } }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return kr.default = e, kr;
}
var Ir = {}, yu;
function A0() {
  if (yu) return Ir;
  yu = 1, Object.defineProperty(Ir, "__esModule", { value: true }), Ir.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-posinset": null, "aria-setsize": null }, relatedConcepts: [{ concept: { name: "article" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "document"]] };
  return Ir.default = e, Ir;
}
var Mr = {}, gu;
function w0() {
  if (gu) return Mr;
  gu = 1, Object.defineProperty(Mr, "__esModule", { value: true }), Mr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { constraints: ["scoped to the body element"], name: "header" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Mr.default = e, Mr;
}
var Lr = {}, Ru;
function E0() {
  if (Ru) return Lr;
  Ru = 1, Object.defineProperty(Lr, "__esModule", { value: true }), Lr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "blockquote" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Lr.default = e, Lr;
}
var Or = {}, xu;
function k0() {
  if (xu) return Or;
  xu = 1, Object.defineProperty(Or, "__esModule", { value: true }), Or.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-disabled": null, "aria-expanded": null, "aria-haspopup": null, "aria-pressed": null }, relatedConcepts: [{ concept: { attributes: [{ name: "type", value: "button" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ name: "type", value: "image" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ name: "type", value: "reset" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ name: "type", value: "submit" }], name: "input" }, module: "HTML" }, { concept: { name: "button" }, module: "HTML" }, { concept: { name: "trigger" }, module: "XForms" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command"]] };
  return Or.default = e, Or;
}
var Nr = {}, _u;
function I0() {
  if (_u) return Nr;
  _u = 1, Object.defineProperty(Nr, "__esModule", { value: true }), Nr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "caption" }, module: "HTML" }], requireContextRole: ["figure", "grid", "table"], requiredContextRole: ["figure", "grid", "table"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Nr.default = e, Nr;
}
var Dr = {}, Cu;
function M0() {
  if (Cu) return Dr;
  Cu = 1, Object.defineProperty(Dr, "__esModule", { value: true }), Dr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-colindex": null, "aria-colspan": null, "aria-rowindex": null, "aria-rowspan": null }, relatedConcepts: [{ concept: { constraints: ["ancestor table element has table role"], name: "td" }, module: "HTML" }], requireContextRole: ["row"], requiredContextRole: ["row"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Dr.default = e, Dr;
}
var Br = {}, Pu;
function L0() {
  if (Pu) return Br;
  Pu = 1, Object.defineProperty(Br, "__esModule", { value: true }), Br.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-checked": null, "aria-errormessage": null, "aria-expanded": null, "aria-invalid": null, "aria-readonly": null, "aria-required": null }, relatedConcepts: [{ concept: { attributes: [{ name: "type", value: "checkbox" }], name: "input" }, module: "HTML" }, { concept: { name: "option" }, module: "ARIA" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-checked": null }, superClass: [["roletype", "widget", "input"]] };
  return Br.default = e, Br;
}
var Fr = {}, qu;
function O0() {
  if (qu) return Fr;
  qu = 1, Object.defineProperty(Fr, "__esModule", { value: true }), Fr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "code" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Fr.default = e, Fr;
}
var jr = {}, Tu;
function N0() {
  if (Tu) return jr;
  Tu = 1, Object.defineProperty(jr, "__esModule", { value: true }), jr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-sort": null }, relatedConcepts: [{ concept: { name: "th" }, module: "HTML" }, { concept: { attributes: [{ name: "scope", value: "col" }], name: "th" }, module: "HTML" }, { concept: { attributes: [{ name: "scope", value: "colgroup" }], name: "th" }, module: "HTML" }], requireContextRole: ["row"], requiredContextRole: ["row"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "cell"], ["roletype", "structure", "section", "cell", "gridcell"], ["roletype", "widget", "gridcell"], ["roletype", "structure", "sectionhead"]] };
  return jr.default = e, jr;
}
var $r = {}, Su;
function D0() {
  if (Su) return $r;
  Su = 1, Object.defineProperty($r, "__esModule", { value: true }), $r.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-activedescendant": null, "aria-autocomplete": null, "aria-errormessage": null, "aria-invalid": null, "aria-readonly": null, "aria-required": null, "aria-expanded": "false", "aria-haspopup": "listbox" }, relatedConcepts: [{ concept: { attributes: [{ constraints: ["set"], name: "list" }, { name: "type", value: "email" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "list" }, { name: "type", value: "search" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "list" }, { name: "type", value: "tel" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "list" }, { name: "type", value: "text" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "list" }, { name: "type", value: "url" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "list" }, { name: "type", value: "url" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["undefined"], name: "multiple" }, { constraints: ["undefined"], name: "size" }], constraints: ["the multiple attribute is not set and the size attribute does not have a value greater than 1"], name: "select" }, module: "HTML" }, { concept: { name: "select" }, module: "XForms" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-controls": null, "aria-expanded": "false" }, superClass: [["roletype", "widget", "input"]] };
  return $r.default = e, $r;
}
var Vr = {}, Au;
function B0() {
  if (Au) return Vr;
  Au = 1, Object.defineProperty(Vr, "__esModule", { value: true }), Vr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { constraints: ["scoped to the body element", "scoped to the main element"], name: "aside" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "aria-label" }], constraints: ["scoped to a sectioning content element", "scoped to a sectioning root element other than body"], name: "aside" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "aria-labelledby" }], constraints: ["scoped to a sectioning content element", "scoped to a sectioning root element other than body"], name: "aside" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Vr.default = e, Vr;
}
var Hr = {}, wu;
function F0() {
  if (wu) return Hr;
  wu = 1, Object.defineProperty(Hr, "__esModule", { value: true }), Hr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { constraints: ["scoped to the body element"], name: "footer" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Hr.default = e, Hr;
}
var Ur = {}, Eu;
function j0() {
  if (Eu) return Ur;
  Eu = 1, Object.defineProperty(Ur, "__esModule", { value: true }), Ur.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "dd" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Ur.default = e, Ur;
}
var Gr = {}, ku;
function $0() {
  if (ku) return Gr;
  ku = 1, Object.defineProperty(Gr, "__esModule", { value: true }), Gr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "del" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Gr.default = e, Gr;
}
var Wr = {}, Iu;
function V0() {
  if (Iu) return Wr;
  Iu = 1, Object.defineProperty(Wr, "__esModule", { value: true }), Wr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "dialog" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "window"]] };
  return Wr.default = e, Wr;
}
var zr = {}, Mu;
function H0() {
  if (Mu) return zr;
  Mu = 1, Object.defineProperty(zr, "__esModule", { value: true }), zr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ module: "DAISY Guide" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "list"]] };
  return zr.default = e, zr;
}
var Kr = {}, Lu;
function U0() {
  if (Lu) return Kr;
  Lu = 1, Object.defineProperty(Kr, "__esModule", { value: true }), Kr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "Device Independence Delivery Unit" } }, { concept: { name: "html" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return Kr.default = e, Kr;
}
var Xr = {}, Ou;
function G0() {
  if (Ou) return Xr;
  Ou = 1, Object.defineProperty(Xr, "__esModule", { value: true }), Xr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "em" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Xr.default = e, Xr;
}
var Yr = {}, Nu;
function W0() {
  if (Nu) return Yr;
  Nu = 1, Object.defineProperty(Yr, "__esModule", { value: true }), Yr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["article"]], requiredProps: {}, superClass: [["roletype", "structure", "section", "list"]] };
  return Yr.default = e, Yr;
}
var Qr = {}, Du;
function z0() {
  if (Du) return Qr;
  Du = 1, Object.defineProperty(Qr, "__esModule", { value: true }), Qr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "figure" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Qr.default = e, Qr;
}
var Jr = {}, Bu;
function K0() {
  if (Bu) return Jr;
  Bu = 1, Object.defineProperty(Jr, "__esModule", { value: true }), Jr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { attributes: [{ constraints: ["set"], name: "aria-label" }], name: "form" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "aria-labelledby" }], name: "form" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "name" }], name: "form" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Jr.default = e, Jr;
}
var Zr = {}, Fu;
function X0() {
  if (Fu) return Zr;
  Fu = 1, Object.defineProperty(Zr, "__esModule", { value: true }), Zr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "a" }, module: "HTML" }, { concept: { name: "area" }, module: "HTML" }, { concept: { name: "aside" }, module: "HTML" }, { concept: { name: "b" }, module: "HTML" }, { concept: { name: "bdo" }, module: "HTML" }, { concept: { name: "body" }, module: "HTML" }, { concept: { name: "data" }, module: "HTML" }, { concept: { name: "div" }, module: "HTML" }, { concept: { constraints: ["scoped to the main element", "scoped to a sectioning content element", "scoped to a sectioning root element other than body"], name: "footer" }, module: "HTML" }, { concept: { constraints: ["scoped to the main element", "scoped to a sectioning content element", "scoped to a sectioning root element other than body"], name: "header" }, module: "HTML" }, { concept: { name: "hgroup" }, module: "HTML" }, { concept: { name: "i" }, module: "HTML" }, { concept: { name: "pre" }, module: "HTML" }, { concept: { name: "q" }, module: "HTML" }, { concept: { name: "samp" }, module: "HTML" }, { concept: { name: "section" }, module: "HTML" }, { concept: { name: "small" }, module: "HTML" }, { concept: { name: "span" }, module: "HTML" }, { concept: { name: "u" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return Zr.default = e, Zr;
}
var ea = {}, ju;
function Y0() {
  if (ju) return ea;
  ju = 1, Object.defineProperty(ea, "__esModule", { value: true }), ea.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-multiselectable": null, "aria-readonly": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["row"], ["row", "rowgroup"]], requiredProps: {}, superClass: [["roletype", "widget", "composite"], ["roletype", "structure", "section", "table"]] };
  return ea.default = e, ea;
}
var ta = {}, $u;
function Q0() {
  if ($u) return ta;
  $u = 1, Object.defineProperty(ta, "__esModule", { value: true }), ta.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null, "aria-readonly": null, "aria-required": null, "aria-selected": null }, relatedConcepts: [{ concept: { constraints: ["ancestor table element has grid role", "ancestor table element has treegrid role"], name: "td" }, module: "HTML" }], requireContextRole: ["row"], requiredContextRole: ["row"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "cell"], ["roletype", "widget"]] };
  return ta.default = e, ta;
}
var ra = {}, Vu;
function J0() {
  if (Vu) return ra;
  Vu = 1, Object.defineProperty(ra, "__esModule", { value: true }), ra.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-activedescendant": null, "aria-disabled": null }, relatedConcepts: [{ concept: { name: "details" }, module: "HTML" }, { concept: { name: "fieldset" }, module: "HTML" }, { concept: { name: "optgroup" }, module: "HTML" }, { concept: { name: "address" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ra.default = e, ra;
}
var aa = {}, Hu;
function Z0() {
  if (Hu) return aa;
  Hu = 1, Object.defineProperty(aa, "__esModule", { value: true }), aa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-level": "2" }, relatedConcepts: [{ concept: { name: "h1" }, module: "HTML" }, { concept: { name: "h2" }, module: "HTML" }, { concept: { name: "h3" }, module: "HTML" }, { concept: { name: "h4" }, module: "HTML" }, { concept: { name: "h5" }, module: "HTML" }, { concept: { name: "h6" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-level": "2" }, superClass: [["roletype", "structure", "sectionhead"]] };
  return aa.default = e, aa;
}
var ia = {}, Uu;
function eb() {
  if (Uu) return ia;
  Uu = 1, Object.defineProperty(ia, "__esModule", { value: true }), ia.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { attributes: [{ constraints: ["set"], name: "alt" }], name: "img" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["undefined"], name: "alt" }], name: "img" }, module: "HTML" }, { concept: { name: "imggroup" }, module: "DTB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ia.default = e, ia;
}
var sa = {}, Gu;
function tb() {
  if (Gu) return sa;
  Gu = 1, Object.defineProperty(sa, "__esModule", { value: true }), sa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "ins" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return sa.default = e, sa;
}
var na = {}, Wu;
function rb() {
  if (Wu) return na;
  Wu = 1, Object.defineProperty(na, "__esModule", { value: true }), na.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-disabled": null, "aria-expanded": null, "aria-haspopup": null }, relatedConcepts: [{ concept: { attributes: [{ constraints: ["set"], name: "href" }], name: "a" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "href" }], name: "area" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command"]] };
  return na.default = e, na;
}
var oa = {}, zu;
function ab() {
  if (zu) return oa;
  zu = 1, Object.defineProperty(oa, "__esModule", { value: true }), oa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "menu" }, module: "HTML" }, { concept: { name: "ol" }, module: "HTML" }, { concept: { name: "ul" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["listitem"]], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return oa.default = e, oa;
}
var la = {}, Ku;
function ib() {
  if (Ku) return la;
  Ku = 1, Object.defineProperty(la, "__esModule", { value: true }), la.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-expanded": null, "aria-invalid": null, "aria-multiselectable": null, "aria-readonly": null, "aria-required": null, "aria-orientation": "vertical" }, relatedConcepts: [{ concept: { attributes: [{ constraints: [">1"], name: "size" }], constraints: ["the size attribute value is greater than 1"], name: "select" }, module: "HTML" }, { concept: { attributes: [{ name: "multiple" }], name: "select" }, module: "HTML" }, { concept: { name: "datalist" }, module: "HTML" }, { concept: { name: "list" }, module: "ARIA" }, { concept: { name: "select" }, module: "XForms" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["option", "group"], ["option"]], requiredProps: {}, superClass: [["roletype", "widget", "composite", "select"], ["roletype", "structure", "section", "group", "select"]] };
  return la.default = e, la;
}
var ua = {}, Xu;
function sb() {
  if (Xu) return ua;
  Xu = 1, Object.defineProperty(ua, "__esModule", { value: true }), ua.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-level": null, "aria-posinset": null, "aria-setsize": null }, relatedConcepts: [{ concept: { constraints: ["direct descendant of ol", "direct descendant of ul", "direct descendant of menu"], name: "li" }, module: "HTML" }, { concept: { name: "item" }, module: "XForms" }], requireContextRole: ["directory", "list"], requiredContextRole: ["directory", "list"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ua.default = e, ua;
}
var ca = {}, Yu;
function nb() {
  if (Yu) return ca;
  Yu = 1, Object.defineProperty(ca, "__esModule", { value: true }), ca.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-live": "polite" }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ca.default = e, ca;
}
var da = {}, Qu;
function ob() {
  if (Qu) return da;
  Qu = 1, Object.defineProperty(da, "__esModule", { value: true }), da.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "main" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return da.default = e, da;
}
var pa = {}, Ju;
function lb() {
  if (Ju) return pa;
  Ju = 1, Object.defineProperty(pa, "__esModule", { value: true }), pa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: [], props: { "aria-braillelabel": null, "aria-brailleroledescription": null, "aria-description": null }, relatedConcepts: [{ concept: { name: "mark" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return pa.default = e, pa;
}
var fa = {}, Zu;
function ub() {
  if (Zu) return fa;
  Zu = 1, Object.defineProperty(fa, "__esModule", { value: true }), fa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return fa.default = e, fa;
}
var ha = {}, ec;
function cb() {
  if (ec) return ha;
  ec = 1, Object.defineProperty(ha, "__esModule", { value: true }), ha.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "math" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ha.default = e, ha;
}
var ma = {}, tc;
function db() {
  if (tc) return ma;
  tc = 1, Object.defineProperty(ma, "__esModule", { value: true }), ma.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-orientation": "vertical" }, relatedConcepts: [{ concept: { name: "MENU" }, module: "JAPI" }, { concept: { name: "list" }, module: "ARIA" }, { concept: { name: "select" }, module: "XForms" }, { concept: { name: "sidebar" }, module: "DTB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["menuitem", "group"], ["menuitemradio", "group"], ["menuitemcheckbox", "group"], ["menuitem"], ["menuitemcheckbox"], ["menuitemradio"]], requiredProps: {}, superClass: [["roletype", "widget", "composite", "select"], ["roletype", "structure", "section", "group", "select"]] };
  return ma.default = e, ma;
}
var va = {}, rc;
function pb() {
  if (rc) return va;
  rc = 1, Object.defineProperty(va, "__esModule", { value: true }), va.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-orientation": "horizontal" }, relatedConcepts: [{ concept: { name: "toolbar" }, module: "ARIA" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["menuitem", "group"], ["menuitemradio", "group"], ["menuitemcheckbox", "group"], ["menuitem"], ["menuitemcheckbox"], ["menuitemradio"]], requiredProps: {}, superClass: [["roletype", "widget", "composite", "select", "menu"], ["roletype", "structure", "section", "group", "select", "menu"]] };
  return va.default = e, va;
}
var ba = {}, ac;
function fb() {
  if (ac) return ba;
  ac = 1, Object.defineProperty(ba, "__esModule", { value: true }), ba.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-disabled": null, "aria-expanded": null, "aria-haspopup": null, "aria-posinset": null, "aria-setsize": null }, relatedConcepts: [{ concept: { name: "MENU_ITEM" }, module: "JAPI" }, { concept: { name: "listitem" }, module: "ARIA" }, { concept: { name: "option" }, module: "ARIA" }], requireContextRole: ["group", "menu", "menubar"], requiredContextRole: ["group", "menu", "menubar"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command"]] };
  return ba.default = e, ba;
}
var ya = {}, ic;
function hb() {
  if (ic) return ya;
  ic = 1, Object.defineProperty(ya, "__esModule", { value: true }), ya.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "menuitem" }, module: "ARIA" }], requireContextRole: ["group", "menu", "menubar"], requiredContextRole: ["group", "menu", "menubar"], requiredOwnedElements: [], requiredProps: { "aria-checked": null }, superClass: [["roletype", "widget", "input", "checkbox"], ["roletype", "widget", "command", "menuitem"]] };
  return ya.default = e, ya;
}
var ga = {}, sc;
function mb() {
  if (sc) return ga;
  sc = 1, Object.defineProperty(ga, "__esModule", { value: true }), ga.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "menuitem" }, module: "ARIA" }], requireContextRole: ["group", "menu", "menubar"], requiredContextRole: ["group", "menu", "menubar"], requiredOwnedElements: [], requiredProps: { "aria-checked": null }, superClass: [["roletype", "widget", "input", "checkbox", "menuitemcheckbox"], ["roletype", "widget", "command", "menuitem", "menuitemcheckbox"], ["roletype", "widget", "input", "radio"]] };
  return ga.default = e, ga;
}
var Ra = {}, nc;
function vb() {
  if (nc) return Ra;
  nc = 1, Object.defineProperty(Ra, "__esModule", { value: true }), Ra.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-valuetext": null, "aria-valuemax": "100", "aria-valuemin": "0" }, relatedConcepts: [{ concept: { name: "meter" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-valuenow": null }, superClass: [["roletype", "structure", "range"]] };
  return Ra.default = e, Ra;
}
var xa = {}, oc;
function bb() {
  if (oc) return xa;
  oc = 1, Object.defineProperty(xa, "__esModule", { value: true }), xa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "nav" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return xa.default = e, xa;
}
var _a = {}, lc;
function yb() {
  if (lc) return _a;
  lc = 1, Object.defineProperty(_a, "__esModule", { value: true }), _a.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: [], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [] };
  return _a.default = e, _a;
}
var Ca = {}, uc;
function gb() {
  if (uc) return Ca;
  uc = 1, Object.defineProperty(Ca, "__esModule", { value: true }), Ca.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Ca.default = e, Ca;
}
var Pa = {}, cc;
function Rb() {
  if (cc) return Pa;
  cc = 1, Object.defineProperty(Pa, "__esModule", { value: true }), Pa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-checked": null, "aria-posinset": null, "aria-setsize": null, "aria-selected": "false" }, relatedConcepts: [{ concept: { name: "item" }, module: "XForms" }, { concept: { name: "listitem" }, module: "ARIA" }, { concept: { name: "option" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-selected": "false" }, superClass: [["roletype", "widget", "input"]] };
  return Pa.default = e, Pa;
}
var qa = {}, dc;
function xb() {
  if (dc) return qa;
  dc = 1, Object.defineProperty(qa, "__esModule", { value: true }), qa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "p" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return qa.default = e, qa;
}
var Ta = {}, pc;
function _b() {
  if (pc) return Ta;
  pc = 1, Object.defineProperty(Ta, "__esModule", { value: true }), Ta.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { attributes: [{ name: "alt", value: "" }], name: "img" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return Ta.default = e, Ta;
}
var Sa = {}, fc;
function Cb() {
  if (fc) return Sa;
  fc = 1, Object.defineProperty(Sa, "__esModule", { value: true }), Sa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-valuetext": null }, relatedConcepts: [{ concept: { name: "progress" }, module: "HTML" }, { concept: { name: "status" }, module: "ARIA" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "range"], ["roletype", "widget"]] };
  return Sa.default = e, Sa;
}
var Aa = {}, hc;
function Pb() {
  if (hc) return Aa;
  hc = 1, Object.defineProperty(Aa, "__esModule", { value: true }), Aa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-checked": null, "aria-posinset": null, "aria-setsize": null }, relatedConcepts: [{ concept: { attributes: [{ name: "type", value: "radio" }], name: "input" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-checked": null }, superClass: [["roletype", "widget", "input"]] };
  return Aa.default = e, Aa;
}
var wa = {}, mc;
function qb() {
  if (mc) return wa;
  mc = 1, Object.defineProperty(wa, "__esModule", { value: true }), wa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null, "aria-readonly": null, "aria-required": null }, relatedConcepts: [{ concept: { name: "list" }, module: "ARIA" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["radio"]], requiredProps: {}, superClass: [["roletype", "widget", "composite", "select"], ["roletype", "structure", "section", "group", "select"]] };
  return wa.default = e, wa;
}
var Ea = {}, vc;
function Tb() {
  if (vc) return Ea;
  vc = 1, Object.defineProperty(Ea, "__esModule", { value: true }), Ea.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { attributes: [{ constraints: ["set"], name: "aria-label" }], name: "section" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "aria-labelledby" }], name: "section" }, module: "HTML" }, { concept: { name: "Device Independence Glossart perceivable unit" } }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Ea.default = e, Ea;
}
var ka = {}, bc;
function Sb() {
  if (bc) return ka;
  bc = 1, Object.defineProperty(ka, "__esModule", { value: true }), ka.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-colindex": null, "aria-expanded": null, "aria-level": null, "aria-posinset": null, "aria-rowindex": null, "aria-selected": null, "aria-setsize": null }, relatedConcepts: [{ concept: { name: "tr" }, module: "HTML" }], requireContextRole: ["grid", "rowgroup", "table", "treegrid"], requiredContextRole: ["grid", "rowgroup", "table", "treegrid"], requiredOwnedElements: [["cell"], ["columnheader"], ["gridcell"], ["rowheader"]], requiredProps: {}, superClass: [["roletype", "structure", "section", "group"], ["roletype", "widget"]] };
  return ka.default = e, ka;
}
var Ia = {}, yc;
function Ab() {
  if (yc) return Ia;
  yc = 1, Object.defineProperty(Ia, "__esModule", { value: true }), Ia.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "tbody" }, module: "HTML" }, { concept: { name: "tfoot" }, module: "HTML" }, { concept: { name: "thead" }, module: "HTML" }], requireContextRole: ["grid", "table", "treegrid"], requiredContextRole: ["grid", "table", "treegrid"], requiredOwnedElements: [["row"]], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return Ia.default = e, Ia;
}
var Ma = {}, gc;
function wb() {
  if (gc) return Ma;
  gc = 1, Object.defineProperty(Ma, "__esModule", { value: true }), Ma.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-sort": null }, relatedConcepts: [{ concept: { attributes: [{ name: "scope", value: "row" }], name: "th" }, module: "HTML" }, { concept: { attributes: [{ name: "scope", value: "rowgroup" }], name: "th" }, module: "HTML" }], requireContextRole: ["row", "rowgroup"], requiredContextRole: ["row", "rowgroup"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "cell"], ["roletype", "structure", "section", "cell", "gridcell"], ["roletype", "widget", "gridcell"], ["roletype", "structure", "sectionhead"]] };
  return Ma.default = e, Ma;
}
var La = {}, Rc;
function Eb() {
  if (Rc) return La;
  Rc = 1, Object.defineProperty(La, "__esModule", { value: true }), La.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-valuetext": null, "aria-orientation": "vertical", "aria-valuemax": "100", "aria-valuemin": "0" }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-controls": null, "aria-valuenow": null }, superClass: [["roletype", "structure", "range"], ["roletype", "widget"]] };
  return La.default = e, La;
}
var Oa = {}, xc;
function kb() {
  if (xc) return Oa;
  xc = 1, Object.defineProperty(Oa, "__esModule", { value: true }), Oa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Oa.default = e, Oa;
}
var Na = {}, _c;
function Ib() {
  if (_c) return Na;
  _c = 1, Object.defineProperty(Na, "__esModule", { value: true }), Na.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { attributes: [{ constraints: ["undefined"], name: "list" }, { name: "type", value: "search" }], constraints: ["the list attribute is not set"], name: "input" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "input", "textbox"]] };
  return Na.default = e, Na;
}
var Da = {}, Cc;
function Mb() {
  if (Cc) return Da;
  Cc = 1, Object.defineProperty(Da, "__esModule", { value: true }), Da.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-orientation": "horizontal", "aria-valuemax": "100", "aria-valuemin": "0", "aria-valuenow": null, "aria-valuetext": null }, relatedConcepts: [{ concept: { name: "hr" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return Da.default = e, Da;
}
var Ba = {}, Pc;
function Lb() {
  if (Pc) return Ba;
  Pc = 1, Object.defineProperty(Ba, "__esModule", { value: true }), Ba.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-haspopup": null, "aria-invalid": null, "aria-readonly": null, "aria-valuetext": null, "aria-orientation": "horizontal", "aria-valuemax": "100", "aria-valuemin": "0" }, relatedConcepts: [{ concept: { attributes: [{ name: "type", value: "range" }], name: "input" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-valuenow": null }, superClass: [["roletype", "widget", "input"], ["roletype", "structure", "range"]] };
  return Ba.default = e, Ba;
}
var Fa = {}, qc;
function Ob() {
  if (qc) return Fa;
  qc = 1, Object.defineProperty(Fa, "__esModule", { value: true }), Fa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null, "aria-readonly": null, "aria-required": null, "aria-valuetext": null, "aria-valuenow": "0" }, relatedConcepts: [{ concept: { attributes: [{ name: "type", value: "number" }], name: "input" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "composite"], ["roletype", "widget", "input"], ["roletype", "structure", "range"]] };
  return Fa.default = e, Fa;
}
var ja = {}, Tc;
function Nb() {
  if (Tc) return ja;
  Tc = 1, Object.defineProperty(ja, "__esModule", { value: true }), ja.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-atomic": "true", "aria-live": "polite" }, relatedConcepts: [{ concept: { name: "output" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ja.default = e, ja;
}
var $a = {}, Sc;
function Db() {
  if (Sc) return $a;
  Sc = 1, Object.defineProperty($a, "__esModule", { value: true }), $a.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "strong" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return $a.default = e, $a;
}
var Va = {}, Ac;
function Bb() {
  if (Ac) return Va;
  Ac = 1, Object.defineProperty(Va, "__esModule", { value: true }), Va.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "sub" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Va.default = e, Va;
}
var Ha = {}, wc;
function Fb() {
  if (wc) return Ha;
  wc = 1, Object.defineProperty(Ha, "__esModule", { value: true }), Ha.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "sup" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Ha.default = e, Ha;
}
var Ua = {}, Ec;
function jb() {
  if (Ec) return Ua;
  Ec = 1, Object.defineProperty(Ua, "__esModule", { value: true }), Ua.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "button" }, module: "ARIA" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-checked": null }, superClass: [["roletype", "widget", "input", "checkbox"]] };
  return Ua.default = e, Ua;
}
var Ga = {}, kc;
function $b() {
  if (kc) return Ga;
  kc = 1, Object.defineProperty(Ga, "__esModule", { value: true }), Ga.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-disabled": null, "aria-expanded": null, "aria-haspopup": null, "aria-posinset": null, "aria-setsize": null, "aria-selected": "false" }, relatedConcepts: [], requireContextRole: ["tablist"], requiredContextRole: ["tablist"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "sectionhead"], ["roletype", "widget"]] };
  return Ga.default = e, Ga;
}
var Wa = {}, Ic;
function Vb() {
  if (Ic) return Wa;
  Ic = 1, Object.defineProperty(Wa, "__esModule", { value: true }), Wa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-colcount": null, "aria-rowcount": null }, relatedConcepts: [{ concept: { name: "table" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["row"], ["row", "rowgroup"]], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Wa.default = e, Wa;
}
var za = {}, Mc;
function Hb() {
  if (Mc) return za;
  Mc = 1, Object.defineProperty(za, "__esModule", { value: true }), za.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-level": null, "aria-multiselectable": null, "aria-orientation": "horizontal" }, relatedConcepts: [{ module: "DAISY", concept: { name: "guide" } }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["tab"]], requiredProps: {}, superClass: [["roletype", "widget", "composite"]] };
  return za.default = e, za;
}
var Ka = {}, Lc;
function Ub() {
  if (Lc) return Ka;
  Lc = 1, Object.defineProperty(Ka, "__esModule", { value: true }), Ka.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Ka.default = e, Ka;
}
var Xa = {}, Oc;
function Gb() {
  if (Oc) return Xa;
  Oc = 1, Object.defineProperty(Xa, "__esModule", { value: true }), Xa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "dfn" }, module: "HTML" }, { concept: { name: "dt" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Xa.default = e, Xa;
}
var Ya = {}, Nc;
function Wb() {
  if (Nc) return Ya;
  Nc = 1, Object.defineProperty(Ya, "__esModule", { value: true }), Ya.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-activedescendant": null, "aria-autocomplete": null, "aria-errormessage": null, "aria-haspopup": null, "aria-invalid": null, "aria-multiline": null, "aria-placeholder": null, "aria-readonly": null, "aria-required": null }, relatedConcepts: [{ concept: { attributes: [{ constraints: ["undefined"], name: "type" }, { constraints: ["undefined"], name: "list" }], constraints: ["the list attribute is not set"], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["undefined"], name: "list" }, { name: "type", value: "email" }], constraints: ["the list attribute is not set"], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["undefined"], name: "list" }, { name: "type", value: "tel" }], constraints: ["the list attribute is not set"], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["undefined"], name: "list" }, { name: "type", value: "text" }], constraints: ["the list attribute is not set"], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["undefined"], name: "list" }, { name: "type", value: "url" }], constraints: ["the list attribute is not set"], name: "input" }, module: "HTML" }, { concept: { name: "input" }, module: "XForms" }, { concept: { name: "textarea" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "input"]] };
  return Ya.default = e, Ya;
}
var Qa = {}, Dc;
function zb() {
  if (Dc) return Qa;
  Dc = 1, Object.defineProperty(Qa, "__esModule", { value: true }), Qa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "time" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Qa.default = e, Qa;
}
var Ja = {}, Bc;
function Kb() {
  if (Bc) return Ja;
  Bc = 1, Object.defineProperty(Ja, "__esModule", { value: true }), Ja.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "status"]] };
  return Ja.default = e, Ja;
}
var Za = {}, Fc;
function Xb() {
  if (Fc) return Za;
  Fc = 1, Object.defineProperty(Za, "__esModule", { value: true }), Za.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-orientation": "horizontal" }, relatedConcepts: [{ concept: { name: "menubar" }, module: "ARIA" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "group"]] };
  return Za.default = e, Za;
}
var ei = {}, jc;
function Yb() {
  if (jc) return ei;
  jc = 1, Object.defineProperty(ei, "__esModule", { value: true }), ei.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ei.default = e, ei;
}
var ti = {}, $c;
function Qb() {
  if ($c) return ti;
  $c = 1, Object.defineProperty(ti, "__esModule", { value: true }), ti.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null, "aria-multiselectable": null, "aria-required": null, "aria-orientation": "vertical" }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["treeitem", "group"], ["treeitem"]], requiredProps: {}, superClass: [["roletype", "widget", "composite", "select"], ["roletype", "structure", "section", "group", "select"]] };
  return ti.default = e, ti;
}
var ri = {}, Vc;
function Jb() {
  if (Vc) return ri;
  Vc = 1, Object.defineProperty(ri, "__esModule", { value: true }), ri.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["row"], ["row", "rowgroup"]], requiredProps: {}, superClass: [["roletype", "widget", "composite", "grid"], ["roletype", "structure", "section", "table", "grid"], ["roletype", "widget", "composite", "select", "tree"], ["roletype", "structure", "section", "group", "select", "tree"]] };
  return ri.default = e, ri;
}
var ai = {}, Hc;
function Zb() {
  if (Hc) return ai;
  Hc = 1, Object.defineProperty(ai, "__esModule", { value: true }), ai.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-expanded": null, "aria-haspopup": null }, relatedConcepts: [], requireContextRole: ["group", "tree"], requiredContextRole: ["group", "tree"], requiredOwnedElements: [], requiredProps: { "aria-selected": null }, superClass: [["roletype", "structure", "section", "listitem"], ["roletype", "widget", "input", "option"]] };
  return ai.default = e, ai;
}
var Uc;
function ey() {
  if (Uc) return Ar;
  Uc = 1, Object.defineProperty(Ar, "__esModule", { value: true }), Ar.default = void 0;
  var e = W(q0()), t = W(T0()), a = W(S0()), i = W(A0()), l = W(w0()), d = W(E0()), n = W(k0()), P = W(I0()), x = W(M0()), T = W(L0()), b = W(O0()), k = W(N0()), I = W(D0()), E = W(B0()), $ = W(F0()), D = W(j0()), S = W($0()), h = W(V0()), v = W(H0()), m = W(U0()), _ = W(G0()), R = W(W0()), q = W(z0()), A = W(K0()), L = W(X0()), r = W(Y0()), s = W(Q0()), o = W(J0()), u = W(Z0()), p = W(eb()), f = W(tb()), g = W(rb()), M = W(ab()), C = W(ib()), y = W(sb()), O = W(nb()), N = W(ob()), V = W(lb()), U = W(ub()), K = W(cb()), X = W(db()), H = W(pb()), re = W(fb()), ue = W(hb()), ye = W(mb()), Re = W(vb()), Ae = W(bb()), je = W(yb()), Le = W(gb()), Me = W(Rb()), ze = W(xb()), Te = W(_b()), ct = W(Cb()), Ue = W(Pb()), gt = W(qb()), dt = W(Tb()), Rt = W(Sb()), Dt = W(Ab()), St = W(wb()), B = W(Eb()), G = W(kb()), z = W(Ib()), Y = W(Mb()), ne = W(Lb()), ae = W(Ob()), ve = W(Nb()), qe = W(Db()), de = W(Bb()), xe = W(Fb()), He = W(jb()), ke = W($b()), we = W(Vb()), $e = W(Hb()), Qe = W(Ub()), et = W(Gb()), st = W(Wb()), pt = W(zb()), At = W(Kb()), Ke = W(Xb()), wt = W(Yb()), Q = W(Qb()), Se = W(Jb()), fe = W(Zb());
  function W(Ye) {
    return Ye && Ye.__esModule ? Ye : { default: Ye };
  }
  var Ge = [["alert", e.default], ["alertdialog", t.default], ["application", a.default], ["article", i.default], ["banner", l.default], ["blockquote", d.default], ["button", n.default], ["caption", P.default], ["cell", x.default], ["checkbox", T.default], ["code", b.default], ["columnheader", k.default], ["combobox", I.default], ["complementary", E.default], ["contentinfo", $.default], ["definition", D.default], ["deletion", S.default], ["dialog", h.default], ["directory", v.default], ["document", m.default], ["emphasis", _.default], ["feed", R.default], ["figure", q.default], ["form", A.default], ["generic", L.default], ["grid", r.default], ["gridcell", s.default], ["group", o.default], ["heading", u.default], ["img", p.default], ["insertion", f.default], ["link", g.default], ["list", M.default], ["listbox", C.default], ["listitem", y.default], ["log", O.default], ["main", N.default], ["mark", V.default], ["marquee", U.default], ["math", K.default], ["menu", X.default], ["menubar", H.default], ["menuitem", re.default], ["menuitemcheckbox", ue.default], ["menuitemradio", ye.default], ["meter", Re.default], ["navigation", Ae.default], ["none", je.default], ["note", Le.default], ["option", Me.default], ["paragraph", ze.default], ["presentation", Te.default], ["progressbar", ct.default], ["radio", Ue.default], ["radiogroup", gt.default], ["region", dt.default], ["row", Rt.default], ["rowgroup", Dt.default], ["rowheader", St.default], ["scrollbar", B.default], ["search", G.default], ["searchbox", z.default], ["separator", Y.default], ["slider", ne.default], ["spinbutton", ae.default], ["status", ve.default], ["strong", qe.default], ["subscript", de.default], ["superscript", xe.default], ["switch", He.default], ["tab", ke.default], ["table", we.default], ["tablist", $e.default], ["tabpanel", Qe.default], ["term", et.default], ["textbox", st.default], ["time", pt.default], ["timer", At.default], ["toolbar", Ke.default], ["tooltip", wt.default], ["tree", Q.default], ["treegrid", Se.default], ["treeitem", fe.default]];
  return Ar.default = Ge, Ar;
}
var ii = {}, si = {}, Gc;
function ty() {
  if (Gc) return si;
  Gc = 1, Object.defineProperty(si, "__esModule", { value: true }), si.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "abstract [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return si.default = e, si;
}
var ni = {}, Wc;
function ry() {
  if (Wc) return ni;
  Wc = 1, Object.defineProperty(ni, "__esModule", { value: true }), ni.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "acknowledgments [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return ni.default = e, ni;
}
var oi = {}, zc;
function ay() {
  if (zc) return oi;
  zc = 1, Object.defineProperty(oi, "__esModule", { value: true }), oi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "afterword [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return oi.default = e, oi;
}
var li = {}, Kc;
function iy() {
  if (Kc) return li;
  Kc = 1, Object.defineProperty(li, "__esModule", { value: true }), li.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "appendix [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return li.default = e, li;
}
var ui = {}, Xc;
function sy() {
  if (Xc) return ui;
  Xc = 1, Object.defineProperty(ui, "__esModule", { value: true }), ui.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "referrer [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command", "link"]] };
  return ui.default = e, ui;
}
var ci = {}, Yc;
function ny() {
  if (Yc) return ci;
  Yc = 1, Object.defineProperty(ci, "__esModule", { value: true }), ci.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "EPUB biblioentry [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: ["doc-bibliography"], requiredContextRole: ["doc-bibliography"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "listitem"]] };
  return ci.default = e, ci;
}
var di = {}, Qc;
function oy() {
  if (Qc) return di;
  Qc = 1, Object.defineProperty(di, "__esModule", { value: true }), di.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "bibliography [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["doc-biblioentry"]], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return di.default = e, di;
}
var pi = {}, Jc;
function ly() {
  if (Jc) return pi;
  Jc = 1, Object.defineProperty(pi, "__esModule", { value: true }), pi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "biblioref [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command", "link"]] };
  return pi.default = e, pi;
}
var fi = {}, Zc;
function uy() {
  if (Zc) return fi;
  Zc = 1, Object.defineProperty(fi, "__esModule", { value: true }), fi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "chapter [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return fi.default = e, fi;
}
var hi = {}, ed;
function cy() {
  if (ed) return hi;
  ed = 1, Object.defineProperty(hi, "__esModule", { value: true }), hi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "colophon [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return hi.default = e, hi;
}
var mi = {}, td;
function dy() {
  if (td) return mi;
  td = 1, Object.defineProperty(mi, "__esModule", { value: true }), mi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "conclusion [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return mi.default = e, mi;
}
var vi = {}, rd;
function py() {
  if (rd) return vi;
  rd = 1, Object.defineProperty(vi, "__esModule", { value: true }), vi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "cover [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "img"]] };
  return vi.default = e, vi;
}
var bi = {}, ad;
function fy() {
  if (ad) return bi;
  ad = 1, Object.defineProperty(bi, "__esModule", { value: true }), bi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "credit [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return bi.default = e, bi;
}
var yi = {}, id;
function hy() {
  if (id) return yi;
  id = 1, Object.defineProperty(yi, "__esModule", { value: true }), yi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "credits [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return yi.default = e, yi;
}
var gi = {}, sd;
function my() {
  if (sd) return gi;
  sd = 1, Object.defineProperty(gi, "__esModule", { value: true }), gi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "dedication [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return gi.default = e, gi;
}
var Ri = {}, nd;
function vy() {
  if (nd) return Ri;
  nd = 1, Object.defineProperty(Ri, "__esModule", { value: true }), Ri.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "rearnote [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: ["doc-endnotes"], requiredContextRole: ["doc-endnotes"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "listitem"]] };
  return Ri.default = e, Ri;
}
var xi = {}, od;
function by() {
  if (od) return xi;
  od = 1, Object.defineProperty(xi, "__esModule", { value: true }), xi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "rearnotes [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["doc-endnote"]], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return xi.default = e, xi;
}
var _i = {}, ld;
function yy() {
  if (ld) return _i;
  ld = 1, Object.defineProperty(_i, "__esModule", { value: true }), _i.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "epigraph [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return _i.default = e, _i;
}
var Ci = {}, ud;
function gy() {
  if (ud) return Ci;
  ud = 1, Object.defineProperty(Ci, "__esModule", { value: true }), Ci.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "epilogue [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Ci.default = e, Ci;
}
var Pi = {}, cd;
function Ry() {
  if (cd) return Pi;
  cd = 1, Object.defineProperty(Pi, "__esModule", { value: true }), Pi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "errata [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Pi.default = e, Pi;
}
var qi = {}, dd;
function xy() {
  if (dd) return qi;
  dd = 1, Object.defineProperty(qi, "__esModule", { value: true }), qi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return qi.default = e, qi;
}
var Ti = {}, pd;
function _y() {
  if (pd) return Ti;
  pd = 1, Object.defineProperty(Ti, "__esModule", { value: true }), Ti.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "footnote [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Ti.default = e, Ti;
}
var Si = {}, fd;
function Cy() {
  if (fd) return Si;
  fd = 1, Object.defineProperty(Si, "__esModule", { value: true }), Si.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "foreword [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Si.default = e, Si;
}
var Ai = {}, hd;
function Py() {
  if (hd) return Ai;
  hd = 1, Object.defineProperty(Ai, "__esModule", { value: true }), Ai.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "glossary [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["definition"], ["term"]], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Ai.default = e, Ai;
}
var wi = {}, md;
function qy() {
  if (md) return wi;
  md = 1, Object.defineProperty(wi, "__esModule", { value: true }), wi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "glossref [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command", "link"]] };
  return wi.default = e, wi;
}
var Ei = {}, vd;
function Ty() {
  if (vd) return Ei;
  vd = 1, Object.defineProperty(Ei, "__esModule", { value: true }), Ei.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "index [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark", "navigation"]] };
  return Ei.default = e, Ei;
}
var ki = {}, bd;
function Sy() {
  if (bd) return ki;
  bd = 1, Object.defineProperty(ki, "__esModule", { value: true }), ki.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "introduction [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return ki.default = e, ki;
}
var Ii = {}, yd;
function Ay() {
  if (yd) return Ii;
  yd = 1, Object.defineProperty(Ii, "__esModule", { value: true }), Ii.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "noteref [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command", "link"]] };
  return Ii.default = e, Ii;
}
var Mi = {}, gd;
function wy() {
  if (gd) return Mi;
  gd = 1, Object.defineProperty(Mi, "__esModule", { value: true }), Mi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "notice [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "note"]] };
  return Mi.default = e, Mi;
}
var Li = {}, Rd;
function Ey() {
  if (Rd) return Li;
  Rd = 1, Object.defineProperty(Li, "__esModule", { value: true }), Li.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "pagebreak [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "separator"]] };
  return Li.default = e, Li;
}
var Oi = {}, xd;
function ky() {
  if (xd) return Oi;
  xd = 1, Object.defineProperty(Oi, "__esModule", { value: true }), Oi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: [], props: { "aria-braillelabel": null, "aria-brailleroledescription": null, "aria-description": null, "aria-disabled": null, "aria-errormessage": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Oi.default = e, Oi;
}
var Ni = {}, _d;
function Iy() {
  if (_d) return Ni;
  _d = 1, Object.defineProperty(Ni, "__esModule", { value: true }), Ni.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: [], props: { "aria-braillelabel": null, "aria-brailleroledescription": null, "aria-description": null, "aria-disabled": null, "aria-errormessage": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Ni.default = e, Ni;
}
var Di = {}, Cd;
function My() {
  if (Cd) return Di;
  Cd = 1, Object.defineProperty(Di, "__esModule", { value: true }), Di.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "page-list [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark", "navigation"]] };
  return Di.default = e, Di;
}
var Bi = {}, Pd;
function Ly() {
  if (Pd) return Bi;
  Pd = 1, Object.defineProperty(Bi, "__esModule", { value: true }), Bi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "part [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Bi.default = e, Bi;
}
var Fi = {}, qd;
function Oy() {
  if (qd) return Fi;
  qd = 1, Object.defineProperty(Fi, "__esModule", { value: true }), Fi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "preface [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Fi.default = e, Fi;
}
var ji = {}, Td;
function Ny() {
  if (Td) return ji;
  Td = 1, Object.defineProperty(ji, "__esModule", { value: true }), ji.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "prologue [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return ji.default = e, ji;
}
var $i = {}, Sd;
function Dy() {
  if (Sd) return $i;
  Sd = 1, Object.defineProperty($i, "__esModule", { value: true }), $i.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "pullquote [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["none"]] };
  return $i.default = e, $i;
}
var Vi = {}, Ad;
function By() {
  if (Ad) return Vi;
  Ad = 1, Object.defineProperty(Vi, "__esModule", { value: true }), Vi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "qna [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Vi.default = e, Vi;
}
var Hi = {}, wd;
function Fy() {
  if (wd) return Hi;
  wd = 1, Object.defineProperty(Hi, "__esModule", { value: true }), Hi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "subtitle [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "sectionhead"]] };
  return Hi.default = e, Hi;
}
var Ui = {}, Ed;
function jy() {
  if (Ed) return Ui;
  Ed = 1, Object.defineProperty(Ui, "__esModule", { value: true }), Ui.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "help [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "note"]] };
  return Ui.default = e, Ui;
}
var Gi = {}, kd;
function $y() {
  if (kd) return Gi;
  kd = 1, Object.defineProperty(Gi, "__esModule", { value: true }), Gi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "toc [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark", "navigation"]] };
  return Gi.default = e, Gi;
}
var Id;
function Vy() {
  if (Id) return ii;
  Id = 1, Object.defineProperty(ii, "__esModule", { value: true }), ii.default = void 0;
  var e = H(ty()), t = H(ry()), a = H(ay()), i = H(iy()), l = H(sy()), d = H(ny()), n = H(oy()), P = H(ly()), x = H(uy()), T = H(cy()), b = H(dy()), k = H(py()), I = H(fy()), E = H(hy()), $ = H(my()), D = H(vy()), S = H(by()), h = H(yy()), v = H(gy()), m = H(Ry()), _ = H(xy()), R = H(_y()), q = H(Cy()), A = H(Py()), L = H(qy()), r = H(Ty()), s = H(Sy()), o = H(Ay()), u = H(wy()), p = H(Ey()), f = H(ky()), g = H(Iy()), M = H(My()), C = H(Ly()), y = H(Oy()), O = H(Ny()), N = H(Dy()), V = H(By()), U = H(Fy()), K = H(jy()), X = H($y());
  function H(ue) {
    return ue && ue.__esModule ? ue : { default: ue };
  }
  var re = [["doc-abstract", e.default], ["doc-acknowledgments", t.default], ["doc-afterword", a.default], ["doc-appendix", i.default], ["doc-backlink", l.default], ["doc-biblioentry", d.default], ["doc-bibliography", n.default], ["doc-biblioref", P.default], ["doc-chapter", x.default], ["doc-colophon", T.default], ["doc-conclusion", b.default], ["doc-cover", k.default], ["doc-credit", I.default], ["doc-credits", E.default], ["doc-dedication", $.default], ["doc-endnote", D.default], ["doc-endnotes", S.default], ["doc-epigraph", h.default], ["doc-epilogue", v.default], ["doc-errata", m.default], ["doc-example", _.default], ["doc-footnote", R.default], ["doc-foreword", q.default], ["doc-glossary", A.default], ["doc-glossref", L.default], ["doc-index", r.default], ["doc-introduction", s.default], ["doc-noteref", o.default], ["doc-notice", u.default], ["doc-pagebreak", p.default], ["doc-pagefooter", f.default], ["doc-pageheader", g.default], ["doc-pagelist", M.default], ["doc-part", C.default], ["doc-preface", y.default], ["doc-prologue", O.default], ["doc-pullquote", N.default], ["doc-qna", V.default], ["doc-subtitle", U.default], ["doc-tip", K.default], ["doc-toc", X.default]];
  return ii.default = re, ii;
}
var Wi = {}, zi = {}, Md;
function Hy() {
  if (Md) return zi;
  Md = 1, Object.defineProperty(zi, "__esModule", { value: true }), zi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ module: "GRAPHICS", concept: { name: "graphics-object" } }, { module: "ARIA", concept: { name: "img" } }, { module: "ARIA", concept: { name: "article" } }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "document"]] };
  return zi.default = e, zi;
}
var Ki = {}, Ld;
function Uy() {
  if (Ld) return Ki;
  Ld = 1, Object.defineProperty(Ki, "__esModule", { value: true }), Ki.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ module: "GRAPHICS", concept: { name: "graphics-document" } }, { module: "ARIA", concept: { name: "group" } }, { module: "ARIA", concept: { name: "img" } }, { module: "GRAPHICS", concept: { name: "graphics-symbol" } }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "group"]] };
  return Ki.default = e, Ki;
}
var Xi = {}, Od;
function Gy() {
  if (Od) return Xi;
  Od = 1, Object.defineProperty(Xi, "__esModule", { value: true }), Xi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "img"]] };
  return Xi.default = e, Xi;
}
var Nd;
function Wy() {
  if (Nd) return Wi;
  Nd = 1, Object.defineProperty(Wi, "__esModule", { value: true }), Wi.default = void 0;
  var e = i(Hy()), t = i(Uy()), a = i(Gy());
  function i(d) {
    return d && d.__esModule ? d : { default: d };
  }
  var l = [["graphics-document", e.default], ["graphics-object", t.default], ["graphics-symbol", a.default]];
  return Wi.default = l, Wi;
}
var Dd;
function Ml() {
  if (Dd) return hr;
  Dd = 1, Object.defineProperty(hr, "__esModule", { value: true }), hr.default = void 0;
  var e = d(P0()), t = d(ey()), a = d(Vy()), i = d(Wy()), l = d(Io());
  function d(D) {
    return D && D.__esModule ? D : { default: D };
  }
  function n(D, S) {
    var h = typeof Symbol < "u" && D[Symbol.iterator] || D["@@iterator"];
    if (!h) {
      if (Array.isArray(D) || (h = T(D)) || S) {
        h && (D = h);
        var v = 0, m = function() {
        };
        return { s: m, n: function() {
          return v >= D.length ? { done: true } : { done: false, value: D[v++] };
        }, e: function(L) {
          throw L;
        }, f: m };
      }
      throw new TypeError(`Invalid attempt to iterate non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
    }
    var _, R = true, q = false;
    return { s: function() {
      h = h.call(D);
    }, n: function() {
      var L = h.next();
      return R = L.done, L;
    }, e: function(L) {
      q = true, _ = L;
    }, f: function() {
      try {
        R || h.return == null || h.return();
      } finally {
        if (q) throw _;
      }
    } };
  }
  function P(D, S) {
    return I(D) || k(D, S) || T(D, S) || x();
  }
  function x() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function T(D, S) {
    if (D) {
      if (typeof D == "string") return b(D, S);
      var h = {}.toString.call(D).slice(8, -1);
      return h === "Object" && D.constructor && (h = D.constructor.name), h === "Map" || h === "Set" ? Array.from(D) : h === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(h) ? b(D, S) : void 0;
    }
  }
  function b(D, S) {
    (S == null || S > D.length) && (S = D.length);
    for (var h = 0, v = Array(S); h < S; h++) v[h] = D[h];
    return v;
  }
  function k(D, S) {
    var h = D == null ? null : typeof Symbol < "u" && D[Symbol.iterator] || D["@@iterator"];
    if (h != null) {
      var v, m, _, R, q = [], A = true, L = false;
      try {
        if (_ = (h = h.call(D)).next, S === 0) {
          if (Object(h) !== h) return;
          A = false;
        } else for (; !(A = (v = _.call(h)).done) && (q.push(v.value), q.length !== S); A = true) ;
      } catch (r) {
        L = true, m = r;
      } finally {
        try {
          if (!A && h.return != null && (R = h.return(), Object(R) !== R)) return;
        } finally {
          if (L) throw m;
        }
      }
      return q;
    }
  }
  function I(D) {
    if (Array.isArray(D)) return D;
  }
  var E = [].concat(e.default, t.default, a.default, i.default);
  E.forEach(function(D) {
    var S = P(D, 2), h = S[1], v = n(h.superClass), m;
    try {
      for (v.s(); !(m = v.n()).done; ) {
        var _ = m.value, R = n(_), q;
        try {
          var A = function() {
            var r = q.value, s = E.filter(function(g) {
              var M = P(g, 1), C = M[0];
              return C === r;
            })[0];
            if (s) for (var o = s[1], u = 0, p = Object.keys(o.props); u < p.length; u++) {
              var f = p[u];
              Object.prototype.hasOwnProperty.call(h.props, f) || (h.props[f] = o.props[f]);
            }
          };
          for (R.s(); !(q = R.n()).done; ) A();
        } catch (L) {
          R.e(L);
        } finally {
          R.f();
        }
      }
    } catch (L) {
      v.e(L);
    } finally {
      v.f();
    }
  });
  var $ = { entries: function() {
    return E;
  }, forEach: function(S) {
    var h = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, v = n(E), m;
    try {
      for (v.s(); !(m = v.n()).done; ) {
        var _ = P(m.value, 2), R = _[0], q = _[1];
        S.call(h, q, R, E);
      }
    } catch (A) {
      v.e(A);
    } finally {
      v.f();
    }
  }, get: function(S) {
    var h = E.filter(function(v) {
      return v[0] === S;
    })[0];
    return h && h[1];
  }, has: function(S) {
    return !!$.get(S);
  }, keys: function() {
    return E.map(function(S) {
      var h = P(S, 1), v = h[0];
      return v;
    });
  }, values: function() {
    return E.map(function(S) {
      var h = P(S, 2), v = h[1];
      return v;
    });
  } };
  return hr.default = (0, l.default)($, $.entries()), hr;
}
var Yi = {}, Bd;
function zy() {
  if (Bd) return Yi;
  Bd = 1, Object.defineProperty(Yi, "__esModule", { value: true }), Yi.default = void 0;
  var e = a(Io()), t = a(Ml());
  function a(R) {
    return R && R.__esModule ? R : { default: R };
  }
  function i(R, q) {
    return x(R) || P(R, q) || d(R, q) || l();
  }
  function l() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function d(R, q) {
    if (R) {
      if (typeof R == "string") return n(R, q);
      var A = {}.toString.call(R).slice(8, -1);
      return A === "Object" && R.constructor && (A = R.constructor.name), A === "Map" || A === "Set" ? Array.from(R) : A === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(A) ? n(R, q) : void 0;
    }
  }
  function n(R, q) {
    (q == null || q > R.length) && (q = R.length);
    for (var A = 0, L = Array(q); A < q; A++) L[A] = R[A];
    return L;
  }
  function P(R, q) {
    var A = R == null ? null : typeof Symbol < "u" && R[Symbol.iterator] || R["@@iterator"];
    if (A != null) {
      var L, r, s, o, u = [], p = true, f = false;
      try {
        if (s = (A = A.call(R)).next, q === 0) {
          if (Object(A) !== A) return;
          p = false;
        } else for (; !(p = (L = s.call(A)).done) && (u.push(L.value), u.length !== q); p = true) ;
      } catch (g) {
        f = true, r = g;
      } finally {
        try {
          if (!p && A.return != null && (o = A.return(), Object(o) !== o)) return;
        } finally {
          if (f) throw r;
        }
      }
      return u;
    }
  }
  function x(R) {
    if (Array.isArray(R)) return R;
  }
  for (var T = [], b = t.default.keys(), k = 0; k < b.length; k++) {
    var I = b[k], E = t.default.get(I);
    if (E) for (var $ = [].concat(E.baseConcepts, E.relatedConcepts), D = function() {
      var q = $[S];
      if (q.module === "HTML") {
        var A = q.concept;
        if (A) {
          var L = T.filter(function(u) {
            return v(u[0], A);
          })[0], r;
          L ? r = L[1] : r = [];
          for (var s = true, o = 0; o < r.length; o++) if (r[o] === I) {
            s = false;
            break;
          }
          s && r.push(I), L || T.push([A, r]);
        }
      }
    }, S = 0; S < $.length; S++) D();
  }
  var h = { entries: function() {
    return T;
  }, forEach: function(q) {
    for (var A = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, L = 0, r = T; L < r.length; L++) {
      var s = i(r[L], 2), o = s[0], u = s[1];
      q.call(A, u, o, T);
    }
  }, get: function(q) {
    var A = T.filter(function(L) {
      return q.name === L[0].name && _(q.attributes, L[0].attributes);
    })[0];
    return A && A[1];
  }, has: function(q) {
    return !!h.get(q);
  }, keys: function() {
    return T.map(function(q) {
      var A = i(q, 1), L = A[0];
      return L;
    });
  }, values: function() {
    return T.map(function(q) {
      var A = i(q, 2), L = A[1];
      return L;
    });
  } };
  function v(R, q) {
    return R.name === q.name && m(R.constraints, q.constraints) && _(R.attributes, q.attributes);
  }
  function m(R, q) {
    if (R === void 0 && q !== void 0 || R !== void 0 && q === void 0) return false;
    if (R !== void 0 && q !== void 0) {
      if (R.length !== q.length) return false;
      for (var A = 0; A < R.length; A++) if (R[A] !== q[A]) return false;
    }
    return true;
  }
  function _(R, q) {
    if (R === void 0 && q !== void 0 || R !== void 0 && q === void 0) return false;
    if (R !== void 0 && q !== void 0) {
      if (R.length !== q.length) return false;
      for (var A = 0; A < R.length; A++) {
        if (R[A].name !== q[A].name || R[A].value !== q[A].value || R[A].constraints === void 0 && q[A].constraints !== void 0 || R[A].constraints !== void 0 && q[A].constraints === void 0) return false;
        if (R[A].constraints !== void 0 && q[A].constraints !== void 0) {
          if (R[A].constraints.length !== q[A].constraints.length) return false;
          for (var L = 0; L < R[A].constraints.length; L++) if (R[A].constraints[L] !== q[A].constraints[L]) return false;
        }
      }
    }
    return true;
  }
  return Yi.default = (0, e.default)(h, h.entries()), Yi;
}
var Qi = {}, Fd;
function Ky() {
  if (Fd) return Qi;
  Fd = 1, Object.defineProperty(Qi, "__esModule", { value: true }), Qi.default = void 0;
  var e = a(Io()), t = a(Ml());
  function a(_) {
    return _ && _.__esModule ? _ : { default: _ };
  }
  function i(_, R) {
    return x(_) || P(_, R) || d(_, R) || l();
  }
  function l() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function d(_, R) {
    if (_) {
      if (typeof _ == "string") return n(_, R);
      var q = {}.toString.call(_).slice(8, -1);
      return q === "Object" && _.constructor && (q = _.constructor.name), q === "Map" || q === "Set" ? Array.from(_) : q === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(q) ? n(_, R) : void 0;
    }
  }
  function n(_, R) {
    (R == null || R > _.length) && (R = _.length);
    for (var q = 0, A = Array(R); q < R; q++) A[q] = _[q];
    return A;
  }
  function P(_, R) {
    var q = _ == null ? null : typeof Symbol < "u" && _[Symbol.iterator] || _["@@iterator"];
    if (q != null) {
      var A, L, r, s, o = [], u = true, p = false;
      try {
        if (r = (q = q.call(_)).next, R === 0) {
          if (Object(q) !== q) return;
          u = false;
        } else for (; !(u = (A = r.call(q)).done) && (o.push(A.value), o.length !== R); u = true) ;
      } catch (f) {
        p = true, L = f;
      } finally {
        try {
          if (!u && q.return != null && (s = q.return(), Object(s) !== s)) return;
        } finally {
          if (p) throw L;
        }
      }
      return o;
    }
  }
  function x(_) {
    if (Array.isArray(_)) return _;
  }
  for (var T = [], b = t.default.keys(), k = 0; k < b.length; k++) {
    var I = b[k], E = t.default.get(I), $ = [];
    if (E) {
      for (var D = [].concat(E.baseConcepts, E.relatedConcepts), S = 0; S < D.length; S++) {
        var h = D[S];
        if (h.module === "HTML") {
          var v = h.concept;
          v != null && $.push(v);
        }
      }
      $.length > 0 && T.push([I, $]);
    }
  }
  var m = { entries: function() {
    return T;
  }, forEach: function(R) {
    for (var q = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, A = 0, L = T; A < L.length; A++) {
      var r = i(L[A], 2), s = r[0], o = r[1];
      R.call(q, o, s, T);
    }
  }, get: function(R) {
    var q = T.filter(function(A) {
      return A[0] === R;
    })[0];
    return q && q[1];
  }, has: function(R) {
    return !!m.get(R);
  }, keys: function() {
    return T.map(function(R) {
      var q = i(R, 1), A = q[0];
      return A;
    });
  }, values: function() {
    return T.map(function(R) {
      var q = i(R, 2), A = q[1];
      return A;
    });
  } };
  return Qi.default = (0, e.default)(m, m.entries()), Qi;
}
var jd;
function Xy() {
  if (jd) return lt;
  jd = 1, Object.defineProperty(lt, "__esModule", { value: true }), lt.roles = lt.roleElements = lt.elementRoles = lt.dom = lt.aria = void 0;
  var e = d(c0()), t = d(d0()), a = d(Ml()), i = d(zy()), l = d(Ky());
  function d(n) {
    return n && n.__esModule ? n : { default: n };
  }
  return lt.aria = e.default, lt.dom = t.default, lt.roles = a.default, lt.elementRoles = i.default, lt.roleElements = l.default, lt;
}
var Mo = Xy(), xt = {}, Ji = {}, Bo = {}, Zi = {}, $d;
function Yy() {
  if ($d) return Zi;
  $d = 1, Object.defineProperty(Zi, "__esModule", { value: true }), Zi.default = void 0;
  function e() {
    var a = this, i = 0, l = { "@@iterator": function() {
      return l;
    }, next: function() {
      if (i < a.length) {
        var n = a[i];
        return i = i + 1, { done: false, value: n };
      } else return { done: true };
    } };
    return l;
  }
  var t = e;
  return Zi.default = t, Zi;
}
var Vd;
function sl() {
  if (Vd) return Bo;
  Vd = 1, Object.defineProperty(Bo, "__esModule", { value: true }), Bo.default = i;
  var e = t(Yy());
  function t(l) {
    return l && l.__esModule ? l : { default: l };
  }
  function a(l) {
    "@babel/helpers - typeof";
    return a = typeof Symbol == "function" && typeof Symbol.iterator == "symbol" ? function(d) {
      return typeof d;
    } : function(d) {
      return d && typeof Symbol == "function" && d.constructor === Symbol && d !== Symbol.prototype ? "symbol" : typeof d;
    }, a(l);
  }
  function i(l, d) {
    return typeof Symbol == "function" && a(Symbol.iterator) === "symbol" && Object.defineProperty(l, Symbol.iterator, { value: e.default.bind(d) }), l;
  }
  return Bo;
}
var es = {}, ts = {}, Hd;
function Qy() {
  if (Hd) return ts;
  Hd = 1, Object.defineProperty(ts, "__esModule", { value: true }), ts.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "abbr" } }], type: "structure" }, t = e;
  return ts.default = t, ts;
}
var rs = {}, Ud;
function Jy() {
  if (Ud) return rs;
  Ud = 1, Object.defineProperty(rs, "__esModule", { value: true }), rs.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "alertdialog" } }], type: "window" }, t = e;
  return rs.default = t, rs;
}
var as = {}, Gd;
function Zy() {
  if (Gd) return as;
  Gd = 1, Object.defineProperty(as, "__esModule", { value: true }), as.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "alert" } }], type: "structure" }, t = e;
  return as.default = t, as;
}
var is = {}, Wd;
function eg() {
  if (Wd) return is;
  Wd = 1, Object.defineProperty(is, "__esModule", { value: true }), is.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return is.default = t, is;
}
var ss = {}, zd;
function tg() {
  if (zd) return ss;
  zd = 1, Object.defineProperty(ss, "__esModule", { value: true }), ss.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "application" } }], type: "window" }, t = e;
  return ss.default = t, ss;
}
var ns = {}, Kd;
function rg() {
  if (Kd) return ns;
  Kd = 1, Object.defineProperty(ns, "__esModule", { value: true }), ns.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "article" } }, { module: "HTML", concept: { name: "article" } }], type: "structure" }, t = e;
  return ns.default = t, ns;
}
var os = {}, Xd;
function ag() {
  if (Xd) return os;
  Xd = 1, Object.defineProperty(os, "__esModule", { value: true }), os.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "audio" } }], type: "widget" }, t = e;
  return os.default = t, os;
}
var ls = {}, Yd;
function ig() {
  if (Yd) return ls;
  Yd = 1, Object.defineProperty(ls, "__esModule", { value: true }), ls.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "banner" } }], type: "structure" }, t = e;
  return ls.default = t, ls;
}
var us = {}, Qd;
function sg() {
  if (Qd) return us;
  Qd = 1, Object.defineProperty(us, "__esModule", { value: true }), us.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "blockquote" } }], type: "structure" }, t = e;
  return us.default = t, us;
}
var cs = {}, Jd;
function ng() {
  if (Jd) return cs;
  Jd = 1, Object.defineProperty(cs, "__esModule", { value: true }), cs.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { attributes: [{ name: "aria-busy", value: "true" }] } }], type: "widget" }, t = e;
  return cs.default = t, cs;
}
var ds = {}, Zd;
function og() {
  if (Zd) return ds;
  Zd = 1, Object.defineProperty(ds, "__esModule", { value: true }), ds.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "button" } }, { module: "HTML", concept: { name: "button" } }], type: "widget" }, t = e;
  return ds.default = t, ds;
}
var ps = {}, ep;
function lg() {
  if (ep) return ps;
  ep = 1, Object.defineProperty(ps, "__esModule", { value: true }), ps.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "canvas" } }], type: "widget" }, t = e;
  return ps.default = t, ps;
}
var fs = {}, tp;
function ug() {
  if (tp) return fs;
  tp = 1, Object.defineProperty(fs, "__esModule", { value: true }), fs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "caption" } }], type: "structure" }, t = e;
  return fs.default = t, fs;
}
var hs = {}, rp;
function cg() {
  if (rp) return hs;
  rp = 1, Object.defineProperty(hs, "__esModule", { value: true }), hs.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "cell" } }, { module: "ARIA", concept: { name: "gridcell" } }, { module: "HTML", concept: { name: "td" } }], type: "widget" }, t = e;
  return hs.default = t, hs;
}
var ms = {}, ap;
function dg() {
  if (ap) return ms;
  ap = 1, Object.defineProperty(ms, "__esModule", { value: true }), ms.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "checkbox" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "checkbox" }] } }], type: "widget" }, t = e;
  return ms.default = t, ms;
}
var vs = {}, ip;
function pg() {
  if (ip) return vs;
  ip = 1, Object.defineProperty(vs, "__esModule", { value: true }), vs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "color" }] } }], type: "widget" }, t = e;
  return vs.default = t, vs;
}
var bs = {}, sp;
function fg() {
  if (sp) return bs;
  sp = 1, Object.defineProperty(bs, "__esModule", { value: true }), bs.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "columnheader" } }, { module: "HTML", concept: { name: "th" } }], type: "widget" }, t = e;
  return bs.default = t, bs;
}
var ys = {}, np;
function hg() {
  if (np) return ys;
  np = 1, Object.defineProperty(ys, "__esModule", { value: true }), ys.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return ys.default = t, ys;
}
var gs = {}, op;
function mg() {
  if (op) return gs;
  op = 1, Object.defineProperty(gs, "__esModule", { value: true }), gs.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "combobox" } }, { module: "HTML", concept: { name: "select" } }], type: "widget" }, t = e;
  return gs.default = t, gs;
}
var Rs = {}, lp;
function vg() {
  if (lp) return Rs;
  lp = 1, Object.defineProperty(Rs, "__esModule", { value: true }), Rs.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "complementary" } }], type: "structure" }, t = e;
  return Rs.default = t, Rs;
}
var xs = {}, up;
function bg() {
  if (up) return xs;
  up = 1, Object.defineProperty(xs, "__esModule", { value: true }), xs.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "structureinfo" } }], type: "structure" }, t = e;
  return xs.default = t, xs;
}
var _s = {}, cp;
function yg() {
  if (cp) return _s;
  cp = 1, Object.defineProperty(_s, "__esModule", { value: true }), _s.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "date" }] } }], type: "widget" }, t = e;
  return _s.default = t, _s;
}
var Cs = {}, dp;
function gg() {
  if (dp) return Cs;
  dp = 1, Object.defineProperty(Cs, "__esModule", { value: true }), Cs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "datetime" }] } }], type: "widget" }, t = e;
  return Cs.default = t, Cs;
}
var Ps = {}, pp;
function Rg() {
  if (pp) return Ps;
  pp = 1, Object.defineProperty(Ps, "__esModule", { value: true }), Ps.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "dfn" } }], type: "structure" }, t = e;
  return Ps.default = t, Ps;
}
var qs = {}, fp;
function xg() {
  if (fp) return qs;
  fp = 1, Object.defineProperty(qs, "__esModule", { value: true }), qs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "dd" } }], type: "structure" }, t = e;
  return qs.default = t, qs;
}
var Ts = {}, hp;
function _g() {
  if (hp) return Ts;
  hp = 1, Object.defineProperty(Ts, "__esModule", { value: true }), Ts.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "dl" } }], type: "structure" }, t = e;
  return Ts.default = t, Ts;
}
var Ss = {}, mp;
function Cg() {
  if (mp) return Ss;
  mp = 1, Object.defineProperty(Ss, "__esModule", { value: true }), Ss.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "dt" } }], type: "structure" }, t = e;
  return Ss.default = t, Ss;
}
var As = {}, vp;
function Pg() {
  if (vp) return As;
  vp = 1, Object.defineProperty(As, "__esModule", { value: true }), As.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "details" } }], type: "structure" }, t = e;
  return As.default = t, As;
}
var ws = {}, bp;
function qg() {
  if (bp) return ws;
  bp = 1, Object.defineProperty(ws, "__esModule", { value: true }), ws.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "dialog" } }, { module: "HTML", concept: { name: "dialog" } }], type: "window" }, t = e;
  return ws.default = t, ws;
}
var Es = {}, yp;
function Tg() {
  if (yp) return Es;
  yp = 1, Object.defineProperty(Es, "__esModule", { value: true }), Es.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "directory" } }, { module: "HTML", concept: { name: "dir" } }], type: "structure" }, t = e;
  return Es.default = t, Es;
}
var ks = {}, gp;
function Sg() {
  if (gp) return ks;
  gp = 1, Object.defineProperty(ks, "__esModule", { value: true }), ks.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { constraints: ["scoped to a details element"], name: "summary" } }], type: "widget" }, t = e;
  return ks.default = t, ks;
}
var Is = {}, Rp;
function Ag() {
  if (Rp) return Is;
  Rp = 1, Object.defineProperty(Is, "__esModule", { value: true }), Is.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "div" } }], type: "generic" }, t = e;
  return Is.default = t, Is;
}
var Ms = {}, xp;
function wg() {
  if (xp) return Ms;
  xp = 1, Object.defineProperty(Ms, "__esModule", { value: true }), Ms.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "document" } }], type: "structure" }, t = e;
  return Ms.default = t, Ms;
}
var Ls = {}, _p;
function Eg() {
  if (_p) return Ls;
  _p = 1, Object.defineProperty(Ls, "__esModule", { value: true }), Ls.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "embed" } }], type: "widget" }, t = e;
  return Ls.default = t, Ls;
}
var Os = {}, Cp;
function kg() {
  if (Cp) return Os;
  Cp = 1, Object.defineProperty(Os, "__esModule", { value: true }), Os.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "feed" } }], type: "structure" }, t = e;
  return Os.default = t, Os;
}
var Ns = {}, Pp;
function Ig() {
  if (Pp) return Ns;
  Pp = 1, Object.defineProperty(Ns, "__esModule", { value: true }), Ns.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "figcaption" } }], type: "structure" }, t = e;
  return Ns.default = t, Ns;
}
var Ds = {}, qp;
function Mg() {
  if (qp) return Ds;
  qp = 1, Object.defineProperty(Ds, "__esModule", { value: true }), Ds.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "figure" } }, { module: "HTML", concept: { name: "figure" } }], type: "structure" }, t = e;
  return Ds.default = t, Ds;
}
var Bs = {}, Tp;
function Lg() {
  if (Tp) return Bs;
  Tp = 1, Object.defineProperty(Bs, "__esModule", { value: true }), Bs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "footer" } }], type: "structure" }, t = e;
  return Bs.default = t, Bs;
}
var Fs = {}, Sp;
function Og() {
  if (Sp) return Fs;
  Sp = 1, Object.defineProperty(Fs, "__esModule", { value: true }), Fs.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "form" } }, { module: "HTML", concept: { name: "form" } }], type: "structure" }, t = e;
  return Fs.default = t, Fs;
}
var js = {}, Ap;
function Ng() {
  if (Ap) return js;
  Ap = 1, Object.defineProperty(js, "__esModule", { value: true }), js.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "grid" } }], type: "widget" }, t = e;
  return js.default = t, js;
}
var $s = {}, wp;
function Dg() {
  if (wp) return $s;
  wp = 1, Object.defineProperty($s, "__esModule", { value: true }), $s.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "group" } }], type: "structure" }, t = e;
  return $s.default = t, $s;
}
var Vs = {}, Ep;
function Bg() {
  if (Ep) return Vs;
  Ep = 1, Object.defineProperty(Vs, "__esModule", { value: true }), Vs.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "heading" } }, { module: "HTML", concept: { name: "h1" } }, { module: "HTML", concept: { name: "h2" } }, { module: "HTML", concept: { name: "h3" } }, { module: "HTML", concept: { name: "h4" } }, { module: "HTML", concept: { name: "h5" } }, { module: "HTML", concept: { name: "h6" } }], type: "structure" }, t = e;
  return Vs.default = t, Vs;
}
var Hs = {}, kp;
function Fg() {
  if (kp) return Hs;
  kp = 1, Object.defineProperty(Hs, "__esModule", { value: true }), Hs.default = void 0;
  var e = { relatedConcepts: [], type: "window" }, t = e;
  return Hs.default = t, Hs;
}
var Us = {}, Ip;
function jg() {
  if (Ip) return Us;
  Ip = 1, Object.defineProperty(Us, "__esModule", { value: true }), Us.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "iframe" } }], type: "window" }, t = e;
  return Us.default = t, Us;
}
var Gs = {}, Mp;
function $g() {
  if (Mp) return Gs;
  Mp = 1, Object.defineProperty(Gs, "__esModule", { value: true }), Gs.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Gs.default = t, Gs;
}
var Ws = {}, Lp;
function Vg() {
  if (Lp) return Ws;
  Lp = 1, Object.defineProperty(Ws, "__esModule", { value: true }), Ws.default = void 0;
  var e = { relatedConcepts: [], type: "widget" }, t = e;
  return Ws.default = t, Ws;
}
var zs = {}, Op;
function Hg() {
  if (Op) return zs;
  Op = 1, Object.defineProperty(zs, "__esModule", { value: true }), zs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "img", attributes: [{ name: "usemap" }] } }], type: "structure" }, t = e;
  return zs.default = t, zs;
}
var Ks = {}, Np;
function Ug() {
  if (Np) return Ks;
  Np = 1, Object.defineProperty(Ks, "__esModule", { value: true }), Ks.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "img" } }, { module: "HTML", concept: { name: "img" } }], type: "structure" }, t = e;
  return Ks.default = t, Ks;
}
var Xs = {}, Dp;
function Gg() {
  if (Dp) return Xs;
  Dp = 1, Object.defineProperty(Xs, "__esModule", { value: true }), Xs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "input" } }], type: "widget" }, t = e;
  return Xs.default = t, Xs;
}
var Ys = {}, Bp;
function Wg() {
  if (Bp) return Ys;
  Bp = 1, Object.defineProperty(Ys, "__esModule", { value: true }), Ys.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "time" }] } }], type: "widget" }, t = e;
  return Ys.default = t, Ys;
}
var Qs = {}, Fp;
function zg() {
  if (Fp) return Qs;
  Fp = 1, Object.defineProperty(Qs, "__esModule", { value: true }), Qs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "label" } }], type: "structure" }, t = e;
  return Qs.default = t, Qs;
}
var Js = {}, jp;
function Kg() {
  if (jp) return Js;
  jp = 1, Object.defineProperty(Js, "__esModule", { value: true }), Js.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "legend" } }], type: "structure" }, t = e;
  return Js.default = t, Js;
}
var Zs = {}, $p;
function Xg() {
  if ($p) return Zs;
  $p = 1, Object.defineProperty(Zs, "__esModule", { value: true }), Zs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "br" } }], type: "structure" }, t = e;
  return Zs.default = t, Zs;
}
var en = {}, Vp;
function Yg() {
  if (Vp) return en;
  Vp = 1, Object.defineProperty(en, "__esModule", { value: true }), en.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "link" } }, { module: "HTML", concept: { name: "a", attributes: [{ name: "href" }] } }], type: "widget" }, t = e;
  return en.default = t, en;
}
var tn = {}, Hp;
function Qg() {
  if (Hp) return tn;
  Hp = 1, Object.defineProperty(tn, "__esModule", { value: true }), tn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "option" } }, { module: "HTML", concept: { name: "option" } }], type: "widget" }, t = e;
  return tn.default = t, tn;
}
var rn = {}, Up;
function Jg() {
  if (Up) return rn;
  Up = 1, Object.defineProperty(rn, "__esModule", { value: true }), rn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "listbox" } }, { module: "HTML", concept: { name: "datalist" } }, { module: "HTML", concept: { name: "select" } }], type: "widget" }, t = e;
  return rn.default = t, rn;
}
var an = {}, Gp;
function Zg() {
  if (Gp) return an;
  Gp = 1, Object.defineProperty(an, "__esModule", { value: true }), an.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "listitem" } }, { module: "HTML", concept: { name: "li" } }], type: "structure" }, t = e;
  return an.default = t, an;
}
var sn = {}, Wp;
function eR() {
  if (Wp) return sn;
  Wp = 1, Object.defineProperty(sn, "__esModule", { value: true }), sn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return sn.default = t, sn;
}
var nn = {}, zp;
function tR() {
  if (zp) return nn;
  zp = 1, Object.defineProperty(nn, "__esModule", { value: true }), nn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "list" } }, { module: "HTML", concept: { name: "ul" } }, { module: "HTML", concept: { name: "ol" } }], type: "structure" }, t = e;
  return nn.default = t, nn;
}
var on = {}, Kp;
function rR() {
  if (Kp) return on;
  Kp = 1, Object.defineProperty(on, "__esModule", { value: true }), on.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "log" } }], type: "structure" }, t = e;
  return on.default = t, on;
}
var ln = {}, Xp;
function aR() {
  if (Xp) return ln;
  Xp = 1, Object.defineProperty(ln, "__esModule", { value: true }), ln.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "main" } }, { module: "HTML", concept: { name: "main" } }], type: "structure" }, t = e;
  return ln.default = t, ln;
}
var un = {}, Yp;
function iR() {
  if (Yp) return un;
  Yp = 1, Object.defineProperty(un, "__esModule", { value: true }), un.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "mark" } }], type: "structure" }, t = e;
  return un.default = t, un;
}
var cn = {}, Qp;
function sR() {
  if (Qp) return cn;
  Qp = 1, Object.defineProperty(cn, "__esModule", { value: true }), cn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "marquee" } }, { module: "HTML", concept: { name: "marquee" } }], type: "structure" }, t = e;
  return cn.default = t, cn;
}
var dn = {}, Jp;
function nR() {
  if (Jp) return dn;
  Jp = 1, Object.defineProperty(dn, "__esModule", { value: true }), dn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "math" } }], type: "structure" }, t = e;
  return dn.default = t, dn;
}
var pn = {}, Zp;
function oR() {
  if (Zp) return pn;
  Zp = 1, Object.defineProperty(pn, "__esModule", { value: true }), pn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "menubar" } }], type: "structure" }, t = e;
  return pn.default = t, pn;
}
var fn = {}, ef;
function lR() {
  if (ef) return fn;
  ef = 1, Object.defineProperty(fn, "__esModule", { value: true }), fn.default = void 0;
  var e = { relatedConcepts: [], type: "widget" }, t = e;
  return fn.default = t, fn;
}
var hn = {}, tf;
function uR() {
  if (tf) return hn;
  tf = 1, Object.defineProperty(hn, "__esModule", { value: true }), hn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "menuitem" } }, { module: "HTML", concept: { name: "menuitem" } }], type: "widget" }, t = e;
  return hn.default = t, hn;
}
var mn = {}, rf;
function cR() {
  if (rf) return mn;
  rf = 1, Object.defineProperty(mn, "__esModule", { value: true }), mn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "menuitemcheckbox" } }], type: "widget" }, t = e;
  return mn.default = t, mn;
}
var vn = {}, af;
function dR() {
  if (af) return vn;
  af = 1, Object.defineProperty(vn, "__esModule", { value: true }), vn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "menuitemradio" } }], type: "widget" }, t = e;
  return vn.default = t, vn;
}
var bn = {}, sf;
function pR() {
  if (sf) return bn;
  sf = 1, Object.defineProperty(bn, "__esModule", { value: true }), bn.default = void 0;
  var e = { relatedConcepts: [], type: "widget" }, t = e;
  return bn.default = t, bn;
}
var yn = {}, nf;
function fR() {
  if (nf) return yn;
  nf = 1, Object.defineProperty(yn, "__esModule", { value: true }), yn.default = void 0;
  var e = { relatedConcepts: [], type: "widget" }, t = e;
  return yn.default = t, yn;
}
var gn = {}, of;
function hR() {
  if (of) return gn;
  of = 1, Object.defineProperty(gn, "__esModule", { value: true }), gn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "menu" } }, { module: "HTML", concept: { name: "menu" } }], type: "structure" }, t = e;
  return gn.default = t, gn;
}
var Rn = {}, lf;
function mR() {
  if (lf) return Rn;
  lf = 1, Object.defineProperty(Rn, "__esModule", { value: true }), Rn.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "meter" } }], type: "structure" }, t = e;
  return Rn.default = t, Rn;
}
var xn = {}, uf;
function vR() {
  if (uf) return xn;
  uf = 1, Object.defineProperty(xn, "__esModule", { value: true }), xn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "navigation" } }, { module: "HTML", concept: { name: "nav" } }], type: "structure" }, t = e;
  return xn.default = t, xn;
}
var _n = {}, cf;
function bR() {
  if (cf) return _n;
  cf = 1, Object.defineProperty(_n, "__esModule", { value: true }), _n.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "none" } }], type: "structure" }, t = e;
  return _n.default = t, _n;
}
var Cn = {}, df;
function yR() {
  if (df) return Cn;
  df = 1, Object.defineProperty(Cn, "__esModule", { value: true }), Cn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "note" } }], type: "structure" }, t = e;
  return Cn.default = t, Cn;
}
var Pn = {}, pf;
function gR() {
  if (pf) return Pn;
  pf = 1, Object.defineProperty(Pn, "__esModule", { value: true }), Pn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Pn.default = t, Pn;
}
var qn = {}, ff;
function RR() {
  if (ff) return qn;
  ff = 1, Object.defineProperty(qn, "__esModule", { value: true }), qn.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "p" } }], type: "structure" }, t = e;
  return qn.default = t, qn;
}
var Tn = {}, hf;
function xR() {
  if (hf) return Tn;
  hf = 1, Object.defineProperty(Tn, "__esModule", { value: true }), Tn.default = void 0;
  var e = { relatedConcepts: [], type: "widget" }, t = e;
  return Tn.default = t, Tn;
}
var Sn = {}, mf;
function _R() {
  if (mf) return Sn;
  mf = 1, Object.defineProperty(Sn, "__esModule", { value: true }), Sn.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "pre" } }], type: "structure" }, t = e;
  return Sn.default = t, Sn;
}
var An = {}, vf;
function CR() {
  if (vf) return An;
  vf = 1, Object.defineProperty(An, "__esModule", { value: true }), An.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "presentation" } }], type: "structure" }, t = e;
  return An.default = t, An;
}
var wn = {}, bf;
function PR() {
  if (bf) return wn;
  bf = 1, Object.defineProperty(wn, "__esModule", { value: true }), wn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "progressbar" } }, { module: "HTML", concept: { name: "progress" } }], type: "structure" }, t = e;
  return wn.default = t, wn;
}
var En = {}, yf;
function qR() {
  if (yf) return En;
  yf = 1, Object.defineProperty(En, "__esModule", { value: true }), En.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "radio" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "radio" }] } }], type: "widget" }, t = e;
  return En.default = t, En;
}
var kn = {}, gf;
function TR() {
  if (gf) return kn;
  gf = 1, Object.defineProperty(kn, "__esModule", { value: true }), kn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "radiogroup" } }], type: "structure" }, t = e;
  return kn.default = t, kn;
}
var In = {}, Rf;
function SR() {
  if (Rf) return In;
  Rf = 1, Object.defineProperty(In, "__esModule", { value: true }), In.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "region" } }], type: "structure" }, t = e;
  return In.default = t, In;
}
var Mn = {}, xf;
function AR() {
  if (xf) return Mn;
  xf = 1, Object.defineProperty(Mn, "__esModule", { value: true }), Mn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Mn.default = t, Mn;
}
var Ln = {}, _f;
function wR() {
  if (_f) return Ln;
  _f = 1, Object.defineProperty(Ln, "__esModule", { value: true }), Ln.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "rowheader" } }, { module: "HTML", concept: { name: "th", attributes: [{ name: "scope", value: "row" }] } }], type: "widget" }, t = e;
  return Ln.default = t, Ln;
}
var On = {}, Cf;
function ER() {
  if (Cf) return On;
  Cf = 1, Object.defineProperty(On, "__esModule", { value: true }), On.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "row" } }, { module: "HTML", concept: { name: "tr" } }], type: "structure" }, t = e;
  return On.default = t, On;
}
var Nn = {}, Pf;
function kR() {
  if (Pf) return Nn;
  Pf = 1, Object.defineProperty(Nn, "__esModule", { value: true }), Nn.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "ruby" } }], type: "structure" }, t = e;
  return Nn.default = t, Nn;
}
var Dn = {}, qf;
function IR() {
  if (qf) return Dn;
  qf = 1, Object.defineProperty(Dn, "__esModule", { value: true }), Dn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Dn.default = t, Dn;
}
var Bn = {}, Tf;
function MR() {
  if (Tf) return Bn;
  Tf = 1, Object.defineProperty(Bn, "__esModule", { value: true }), Bn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Bn.default = t, Bn;
}
var Fn = {}, Sf;
function LR() {
  if (Sf) return Fn;
  Sf = 1, Object.defineProperty(Fn, "__esModule", { value: true }), Fn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "scrollbar" } }], type: "widget" }, t = e;
  return Fn.default = t, Fn;
}
var jn = {}, Af;
function OR() {
  if (Af) return jn;
  Af = 1, Object.defineProperty(jn, "__esModule", { value: true }), jn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return jn.default = t, jn;
}
var $n = {}, wf;
function NR() {
  if (wf) return $n;
  wf = 1, Object.defineProperty($n, "__esModule", { value: true }), $n.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "search" } }], type: "structure" }, t = e;
  return $n.default = t, $n;
}
var Vn = {}, Ef;
function DR() {
  if (Ef) return Vn;
  Ef = 1, Object.defineProperty(Vn, "__esModule", { value: true }), Vn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "searchbox" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "search" }] } }], type: "widget" }, t = e;
  return Vn.default = t, Vn;
}
var Hn = {}, kf;
function BR() {
  if (kf) return Hn;
  kf = 1, Object.defineProperty(Hn, "__esModule", { value: true }), Hn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "slider" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "range" }] } }], type: "widget" }, t = e;
  return Hn.default = t, Hn;
}
var Un = {}, If;
function FR() {
  if (If) return Un;
  If = 1, Object.defineProperty(Un, "__esModule", { value: true }), Un.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Un.default = t, Un;
}
var Gn = {}, Mf;
function jR() {
  if (Mf) return Gn;
  Mf = 1, Object.defineProperty(Gn, "__esModule", { value: true }), Gn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "spinbutton" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "number" }] } }], type: "widget" }, t = e;
  return Gn.default = t, Gn;
}
var Wn = {}, Lf;
function $R() {
  if (Lf) return Wn;
  Lf = 1, Object.defineProperty(Wn, "__esModule", { value: true }), Wn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Wn.default = t, Wn;
}
var zn = {}, Of;
function VR() {
  if (Of) return zn;
  Of = 1, Object.defineProperty(zn, "__esModule", { value: true }), zn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "separator" } }], type: "widget" }, t = e;
  return zn.default = t, zn;
}
var Kn = {}, Nf;
function HR() {
  if (Nf) return Kn;
  Nf = 1, Object.defineProperty(Kn, "__esModule", { value: true }), Kn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Kn.default = t, Kn;
}
var Xn = {}, Df;
function UR() {
  if (Df) return Xn;
  Df = 1, Object.defineProperty(Xn, "__esModule", { value: true }), Xn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "status" } }], type: "structure" }, t = e;
  return Xn.default = t, Xn;
}
var Yn = {}, Bf;
function GR() {
  if (Bf) return Yn;
  Bf = 1, Object.defineProperty(Yn, "__esModule", { value: true }), Yn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Yn.default = t, Yn;
}
var Qn = {}, Ff;
function WR() {
  if (Ff) return Qn;
  Ff = 1, Object.defineProperty(Qn, "__esModule", { value: true }), Qn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "switch" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "checkbox" }] } }], type: "widget" }, t = e;
  return Qn.default = t, Qn;
}
var Jn = {}, jf;
function zR() {
  if (jf) return Jn;
  jf = 1, Object.defineProperty(Jn, "__esModule", { value: true }), Jn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "tablist" } }], type: "structure" }, t = e;
  return Jn.default = t, Jn;
}
var Zn = {}, $f;
function KR() {
  if ($f) return Zn;
  $f = 1, Object.defineProperty(Zn, "__esModule", { value: true }), Zn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "tab" } }], type: "widget" }, t = e;
  return Zn.default = t, Zn;
}
var eo = {}, Vf;
function XR() {
  if (Vf) return eo;
  Vf = 1, Object.defineProperty(eo, "__esModule", { value: true }), eo.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return eo.default = t, eo;
}
var to = {}, Hf;
function YR() {
  if (Hf) return to;
  Hf = 1, Object.defineProperty(to, "__esModule", { value: true }), to.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "table" } }, { module: "HTML", concept: { name: "table" } }], type: "structure" }, t = e;
  return to.default = t, to;
}
var ro = {}, Uf;
function QR() {
  if (Uf) return ro;
  Uf = 1, Object.defineProperty(ro, "__esModule", { value: true }), ro.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "tablist" } }], type: "structure" }, t = e;
  return ro.default = t, ro;
}
var ao = {}, Gf;
function JR() {
  if (Gf) return ao;
  Gf = 1, Object.defineProperty(ao, "__esModule", { value: true }), ao.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "tabpanel" } }], type: "structure" }, t = e;
  return ao.default = t, ao;
}
var io = {}, Wf;
function ZR() {
  if (Wf) return io;
  Wf = 1, Object.defineProperty(io, "__esModule", { value: true }), io.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "term" } }], type: "structure" }, t = e;
  return io.default = t, io;
}
var so = {}, zf;
function ex() {
  if (zf) return so;
  zf = 1, Object.defineProperty(so, "__esModule", { value: true }), so.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { attributes: [{ name: "aria-multiline", value: "true" }], name: "textbox" } }, { module: "HTML", concept: { name: "textarea" } }], type: "widget" }, t = e;
  return so.default = t, so;
}
var no = {}, Kf;
function tx() {
  if (Kf) return no;
  Kf = 1, Object.defineProperty(no, "__esModule", { value: true }), no.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "textbox" } }, { module: "HTML", concept: { name: "input" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "text" }] } }], type: "widget" }, t = e;
  return no.default = t, no;
}
var oo = {}, Xf;
function rx() {
  if (Xf) return oo;
  Xf = 1, Object.defineProperty(oo, "__esModule", { value: true }), oo.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "time" } }], type: "structure" }, t = e;
  return oo.default = t, oo;
}
var lo = {}, Yf;
function ax() {
  if (Yf) return lo;
  Yf = 1, Object.defineProperty(lo, "__esModule", { value: true }), lo.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "timer" } }], type: "structure" }, t = e;
  return lo.default = t, lo;
}
var uo = {}, Qf;
function ix() {
  if (Qf) return uo;
  Qf = 1, Object.defineProperty(uo, "__esModule", { value: true }), uo.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { attributes: [{ name: "aria-pressed" }] } }], type: "widget" }, t = e;
  return uo.default = t, uo;
}
var co = {}, Jf;
function sx() {
  if (Jf) return co;
  Jf = 1, Object.defineProperty(co, "__esModule", { value: true }), co.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "toolbar" } }], type: "structure" }, t = e;
  return co.default = t, co;
}
var po = {}, Zf;
function nx() {
  if (Zf) return po;
  Zf = 1, Object.defineProperty(po, "__esModule", { value: true }), po.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "tree" } }], type: "widget" }, t = e;
  return po.default = t, po;
}
var fo = {}, eh;
function ox() {
  if (eh) return fo;
  eh = 1, Object.defineProperty(fo, "__esModule", { value: true }), fo.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "treegrid" } }], type: "widget" }, t = e;
  return fo.default = t, fo;
}
var ho = {}, th;
function lx() {
  if (th) return ho;
  th = 1, Object.defineProperty(ho, "__esModule", { value: true }), ho.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "treeitem" } }], type: "widget" }, t = e;
  return ho.default = t, ho;
}
var mo = {}, rh;
function ux() {
  if (rh) return mo;
  rh = 1, Object.defineProperty(mo, "__esModule", { value: true }), mo.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "tooltip" } }], type: "structure" }, t = e;
  return mo.default = t, mo;
}
var vo = {}, ah;
function cx() {
  if (ah) return vo;
  ah = 1, Object.defineProperty(vo, "__esModule", { value: true }), vo.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "video" } }], type: "widget" }, t = e;
  return vo.default = t, vo;
}
var bo = {}, ih;
function dx() {
  if (ih) return bo;
  ih = 1, Object.defineProperty(bo, "__esModule", { value: true }), bo.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return bo.default = t, bo;
}
var yo = {}, sh;
function px() {
  if (sh) return yo;
  sh = 1, Object.defineProperty(yo, "__esModule", { value: true }), yo.default = void 0;
  var e = { relatedConcepts: [], type: "window" }, t = e;
  return yo.default = t, yo;
}
var nh;
function nl() {
  if (nh) return es;
  nh = 1, Object.defineProperty(es, "__esModule", { value: true }), es.default = void 0;
  var e = j(sl()), t = j(Qy()), a = j(Jy()), i = j(Zy()), l = j(eg()), d = j(tg()), n = j(rg()), P = j(ag()), x = j(ig()), T = j(sg()), b = j(ng()), k = j(og()), I = j(lg()), E = j(ug()), $ = j(cg()), D = j(dg()), S = j(pg()), h = j(fg()), v = j(hg()), m = j(mg()), _ = j(vg()), R = j(bg()), q = j(yg()), A = j(gg()), L = j(Rg()), r = j(xg()), s = j(_g()), o = j(Cg()), u = j(Pg()), p = j(qg()), f = j(Tg()), g = j(Sg()), M = j(Ag()), C = j(wg()), y = j(Eg()), O = j(kg()), N = j(Ig()), V = j(Mg()), U = j(Lg()), K = j(Og()), X = j(Ng()), H = j(Dg()), re = j(Bg()), ue = j(Fg()), ye = j(jg()), Re = j($g()), Ae = j(Vg()), je = j(Hg()), Le = j(Ug()), Me = j(Gg()), ze = j(Wg()), Te = j(zg()), ct = j(Kg()), Ue = j(Xg()), gt = j(Yg()), dt = j(Qg()), Rt = j(Jg()), Dt = j(Zg()), St = j(eR()), B = j(tR()), G = j(rR()), z = j(aR()), Y = j(iR()), ne = j(sR()), ae = j(nR()), ve = j(oR()), qe = j(lR()), de = j(uR()), xe = j(cR()), He = j(dR()), ke = j(pR()), we = j(fR()), $e = j(hR()), Qe = j(mR()), et = j(vR()), st = j(bR()), pt = j(yR()), At = j(gR()), Ke = j(RR()), wt = j(xR()), Q = j(_R()), Se = j(CR()), fe = j(PR()), W = j(qR()), Ge = j(TR()), Ye = j(SR()), Xe = j(AR()), Je = j(wR()), Bt = j(ER()), ym = j(kR()), gm = j(IR()), Rm = j(MR()), xm = j(LR()), _m = j(OR()), Cm = j(NR()), Pm = j(DR()), qm = j(BR()), Tm = j(FR()), Sm = j(jR()), Am = j($R()), wm = j(VR()), Em = j(HR()), km = j(UR()), Im = j(GR()), Mm = j(WR()), Lm = j(zR()), Om = j(KR()), Nm = j(XR()), Dm = j(YR()), Bm = j(QR()), Fm = j(JR()), jm = j(ZR()), $m = j(ex()), Vm = j(tx()), Hm = j(rx()), Um = j(ax()), Gm = j(ix()), Wm = j(sx()), zm = j(nx()), Km = j(ox()), Xm = j(lx()), Ym = j(ux()), Qm = j(cx()), Jm = j(dx()), Zm = j(px());
  function j(_e2) {
    return _e2 && _e2.__esModule ? _e2 : { default: _e2 };
  }
  function ol(_e2, Ne) {
    return av(_e2) || rv(_e2, Ne) || tv(_e2, Ne) || ev();
  }
  function ev() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function tv(_e2, Ne) {
    if (_e2) {
      if (typeof _e2 == "string") return Nl(_e2, Ne);
      var Ie = Object.prototype.toString.call(_e2).slice(8, -1);
      if (Ie === "Object" && _e2.constructor && (Ie = _e2.constructor.name), Ie === "Map" || Ie === "Set") return Array.from(_e2);
      if (Ie === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(Ie)) return Nl(_e2, Ne);
    }
  }
  function Nl(_e2, Ne) {
    (Ne == null || Ne > _e2.length) && (Ne = _e2.length);
    for (var Ie = 0, Ze = new Array(Ne); Ie < Ne; Ie++) Ze[Ie] = _e2[Ie];
    return Ze;
  }
  function rv(_e2, Ne) {
    var Ie = _e2 == null ? null : typeof Symbol < "u" && _e2[Symbol.iterator] || _e2["@@iterator"];
    if (Ie != null) {
      var Ze = [], Xt = true, lr = false, Lo, Oo;
      try {
        for (Ie = Ie.call(_e2); !(Xt = (Lo = Ie.next()).done) && (Ze.push(Lo.value), !(Ne && Ze.length === Ne)); Xt = true) ;
      } catch (sv) {
        lr = true, Oo = sv;
      } finally {
        try {
          !Xt && Ie.return != null && Ie.return();
        } finally {
          if (lr) throw Oo;
        }
      }
      return Ze;
    }
  }
  function av(_e2) {
    if (Array.isArray(_e2)) return _e2;
  }
  var Kt = [["AbbrRole", t.default], ["AlertDialogRole", a.default], ["AlertRole", i.default], ["AnnotationRole", l.default], ["ApplicationRole", d.default], ["ArticleRole", n.default], ["AudioRole", P.default], ["BannerRole", x.default], ["BlockquoteRole", T.default], ["BusyIndicatorRole", b.default], ["ButtonRole", k.default], ["CanvasRole", I.default], ["CaptionRole", E.default], ["CellRole", $.default], ["CheckBoxRole", D.default], ["ColorWellRole", S.default], ["ColumnHeaderRole", h.default], ["ColumnRole", v.default], ["ComboBoxRole", m.default], ["ComplementaryRole", _.default], ["ContentInfoRole", R.default], ["DateRole", q.default], ["DateTimeRole", A.default], ["DefinitionRole", L.default], ["DescriptionListDetailRole", r.default], ["DescriptionListRole", s.default], ["DescriptionListTermRole", o.default], ["DetailsRole", u.default], ["DialogRole", p.default], ["DirectoryRole", f.default], ["DisclosureTriangleRole", g.default], ["DivRole", M.default], ["DocumentRole", C.default], ["EmbeddedObjectRole", y.default], ["FeedRole", O.default], ["FigcaptionRole", N.default], ["FigureRole", V.default], ["FooterRole", U.default], ["FormRole", K.default], ["GridRole", X.default], ["GroupRole", H.default], ["HeadingRole", re.default], ["IframePresentationalRole", ue.default], ["IframeRole", ye.default], ["IgnoredRole", Re.default], ["ImageMapLinkRole", Ae.default], ["ImageMapRole", je.default], ["ImageRole", Le.default], ["InlineTextBoxRole", Me.default], ["InputTimeRole", ze.default], ["LabelRole", Te.default], ["LegendRole", ct.default], ["LineBreakRole", Ue.default], ["LinkRole", gt.default], ["ListBoxOptionRole", dt.default], ["ListBoxRole", Rt.default], ["ListItemRole", Dt.default], ["ListMarkerRole", St.default], ["ListRole", B.default], ["LogRole", G.default], ["MainRole", z.default], ["MarkRole", Y.default], ["MarqueeRole", ne.default], ["MathRole", ae.default], ["MenuBarRole", ve.default], ["MenuButtonRole", qe.default], ["MenuItemRole", de.default], ["MenuItemCheckBoxRole", xe.default], ["MenuItemRadioRole", He.default], ["MenuListOptionRole", ke.default], ["MenuListPopupRole", we.default], ["MenuRole", $e.default], ["MeterRole", Qe.default], ["NavigationRole", et.default], ["NoneRole", st.default], ["NoteRole", pt.default], ["OutlineRole", At.default], ["ParagraphRole", Ke.default], ["PopUpButtonRole", wt.default], ["PreRole", Q.default], ["PresentationalRole", Se.default], ["ProgressIndicatorRole", fe.default], ["RadioButtonRole", W.default], ["RadioGroupRole", Ge.default], ["RegionRole", Ye.default], ["RootWebAreaRole", Xe.default], ["RowHeaderRole", Je.default], ["RowRole", Bt.default], ["RubyRole", ym.default], ["RulerRole", gm.default], ["ScrollAreaRole", Rm.default], ["ScrollBarRole", xm.default], ["SeamlessWebAreaRole", _m.default], ["SearchRole", Cm.default], ["SearchBoxRole", Pm.default], ["SliderRole", qm.default], ["SliderThumbRole", Tm.default], ["SpinButtonRole", Sm.default], ["SpinButtonPartRole", Am.default], ["SplitterRole", wm.default], ["StaticTextRole", Em.default], ["StatusRole", km.default], ["SVGRootRole", Im.default], ["SwitchRole", Mm.default], ["TabGroupRole", Lm.default], ["TabRole", Om.default], ["TableHeaderContainerRole", Nm.default], ["TableRole", Dm.default], ["TabListRole", Bm.default], ["TabPanelRole", Fm.default], ["TermRole", jm.default], ["TextAreaRole", $m.default], ["TextFieldRole", Vm.default], ["TimeRole", Hm.default], ["TimerRole", Um.default], ["ToggleButtonRole", Gm.default], ["ToolbarRole", Wm.default], ["TreeRole", zm.default], ["TreeGridRole", Km.default], ["TreeItemRole", Xm.default], ["UserInterfaceTooltipRole", Ym.default], ["VideoRole", Qm.default], ["WebAreaRole", Jm.default], ["WindowRole", Zm.default]], ll = { entries: function() {
    return Kt;
  }, forEach: function(Ne) {
    for (var Ie = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, Ze = 0, Xt = Kt; Ze < Xt.length; Ze++) {
      var lr = ol(Xt[Ze], 2), Lo = lr[0], Oo = lr[1];
      Ne.call(Ie, Oo, Lo, Kt);
    }
  }, get: function(Ne) {
    var Ie = Kt.find(function(Ze) {
      return Ze[0] === Ne;
    });
    return Ie && Ie[1];
  }, has: function(Ne) {
    return !!ll.get(Ne);
  }, keys: function() {
    return Kt.map(function(Ne) {
      var Ie = ol(Ne, 1), Ze = Ie[0];
      return Ze;
    });
  }, values: function() {
    return Kt.map(function(Ne) {
      var Ie = ol(Ne, 2), Ze = Ie[1];
      return Ze;
    });
  } }, iv = (0, e.default)(ll, ll.entries());
  return es.default = iv, es;
}
var oh;
function fx() {
  if (oh) return Ji;
  oh = 1, Object.defineProperty(Ji, "__esModule", { value: true }), Ji.default = void 0;
  var e = a(sl()), t = a(nl());
  function a(S) {
    return S && S.__esModule ? S : { default: S };
  }
  function i(S, h) {
    return n(S) || d(S, h) || x(S, h) || l();
  }
  function l() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function d(S, h) {
    var v = S == null ? null : typeof Symbol < "u" && S[Symbol.iterator] || S["@@iterator"];
    if (v != null) {
      var m = [], _ = true, R = false, q, A;
      try {
        for (v = v.call(S); !(_ = (q = v.next()).done) && (m.push(q.value), !(h && m.length === h)); _ = true) ;
      } catch (L) {
        R = true, A = L;
      } finally {
        try {
          !_ && v.return != null && v.return();
        } finally {
          if (R) throw A;
        }
      }
      return m;
    }
  }
  function n(S) {
    if (Array.isArray(S)) return S;
  }
  function P(S, h) {
    var v = typeof Symbol < "u" && S[Symbol.iterator] || S["@@iterator"];
    if (!v) {
      if (Array.isArray(S) || (v = x(S)) || h) {
        v && (S = v);
        var m = 0, _ = function() {
        };
        return { s: _, n: function() {
          return m >= S.length ? { done: true } : { done: false, value: S[m++] };
        }, e: function(r) {
          throw r;
        }, f: _ };
      }
      throw new TypeError(`Invalid attempt to iterate non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
    }
    var R = true, q = false, A;
    return { s: function() {
      v = v.call(S);
    }, n: function() {
      var r = v.next();
      return R = r.done, r;
    }, e: function(r) {
      q = true, A = r;
    }, f: function() {
      try {
        !R && v.return != null && v.return();
      } finally {
        if (q) throw A;
      }
    } };
  }
  function x(S, h) {
    if (S) {
      if (typeof S == "string") return T(S, h);
      var v = Object.prototype.toString.call(S).slice(8, -1);
      if (v === "Object" && S.constructor && (v = S.constructor.name), v === "Map" || v === "Set") return Array.from(S);
      if (v === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(v)) return T(S, h);
    }
  }
  function T(S, h) {
    (h == null || h > S.length) && (h = S.length);
    for (var v = 0, m = new Array(h); v < h; v++) m[v] = S[v];
    return m;
  }
  var b = [], k = P(t.default.entries()), I;
  try {
    var E = function() {
      var h = i(I.value, 2), v = h[0], m = h[1], _ = m.relatedConcepts;
      Array.isArray(_) && _.forEach(function(R) {
        if (R.module === "HTML") {
          var q = R.concept;
          if (q) {
            var A = b.findIndex(function(L) {
              var r = i(L, 1), s = r[0];
              return s === v;
            });
            A === -1 && (b.push([v, []]), A = b.length - 1), b[A][1].push(q);
          }
        }
      });
    };
    for (k.s(); !(I = k.n()).done; ) E();
  } catch (S) {
    k.e(S);
  } finally {
    k.f();
  }
  var $ = { entries: function() {
    return b;
  }, forEach: function(h) {
    for (var v = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, m = 0, _ = b; m < _.length; m++) {
      var R = i(_[m], 2), q = R[0], A = R[1];
      h.call(v, A, q, b);
    }
  }, get: function(h) {
    var v = b.find(function(m) {
      return m[0] === h;
    });
    return v && v[1];
  }, has: function(h) {
    return !!$.get(h);
  }, keys: function() {
    return b.map(function(h) {
      var v = i(h, 1), m = v[0];
      return m;
    });
  }, values: function() {
    return b.map(function(h) {
      var v = i(h, 2), m = v[1];
      return m;
    });
  } }, D = (0, e.default)($, $.entries());
  return Ji.default = D, Ji;
}
var go = {}, lh;
function hx() {
  if (lh) return go;
  lh = 1, Object.defineProperty(go, "__esModule", { value: true }), go.default = void 0;
  var e = a(sl()), t = a(nl());
  function a(S) {
    return S && S.__esModule ? S : { default: S };
  }
  function i(S, h) {
    return n(S) || d(S, h) || x(S, h) || l();
  }
  function l() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function d(S, h) {
    var v = S == null ? null : typeof Symbol < "u" && S[Symbol.iterator] || S["@@iterator"];
    if (v != null) {
      var m = [], _ = true, R = false, q, A;
      try {
        for (v = v.call(S); !(_ = (q = v.next()).done) && (m.push(q.value), !(h && m.length === h)); _ = true) ;
      } catch (L) {
        R = true, A = L;
      } finally {
        try {
          !_ && v.return != null && v.return();
        } finally {
          if (R) throw A;
        }
      }
      return m;
    }
  }
  function n(S) {
    if (Array.isArray(S)) return S;
  }
  function P(S, h) {
    var v = typeof Symbol < "u" && S[Symbol.iterator] || S["@@iterator"];
    if (!v) {
      if (Array.isArray(S) || (v = x(S)) || h) {
        v && (S = v);
        var m = 0, _ = function() {
        };
        return { s: _, n: function() {
          return m >= S.length ? { done: true } : { done: false, value: S[m++] };
        }, e: function(r) {
          throw r;
        }, f: _ };
      }
      throw new TypeError(`Invalid attempt to iterate non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
    }
    var R = true, q = false, A;
    return { s: function() {
      v = v.call(S);
    }, n: function() {
      var r = v.next();
      return R = r.done, r;
    }, e: function(r) {
      q = true, A = r;
    }, f: function() {
      try {
        !R && v.return != null && v.return();
      } finally {
        if (q) throw A;
      }
    } };
  }
  function x(S, h) {
    if (S) {
      if (typeof S == "string") return T(S, h);
      var v = Object.prototype.toString.call(S).slice(8, -1);
      if (v === "Object" && S.constructor && (v = S.constructor.name), v === "Map" || v === "Set") return Array.from(S);
      if (v === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(v)) return T(S, h);
    }
  }
  function T(S, h) {
    (h == null || h > S.length) && (h = S.length);
    for (var v = 0, m = new Array(h); v < h; v++) m[v] = S[v];
    return m;
  }
  var b = [], k = P(t.default.entries()), I;
  try {
    var E = function() {
      var h = i(I.value, 2), v = h[0], m = h[1], _ = m.relatedConcepts;
      Array.isArray(_) && _.forEach(function(R) {
        if (R.module === "ARIA") {
          var q = R.concept;
          if (q) {
            var A = b.findIndex(function(L) {
              var r = i(L, 1), s = r[0];
              return s === v;
            });
            A === -1 && (b.push([v, []]), A = b.length - 1), b[A][1].push(q);
          }
        }
      });
    };
    for (k.s(); !(I = k.n()).done; ) E();
  } catch (S) {
    k.e(S);
  } finally {
    k.f();
  }
  var $ = { entries: function() {
    return b;
  }, forEach: function(h) {
    for (var v = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, m = 0, _ = b; m < _.length; m++) {
      var R = i(_[m], 2), q = R[0], A = R[1];
      h.call(v, A, q, b);
    }
  }, get: function(h) {
    var v = b.find(function(m) {
      return m[0] === h;
    });
    return v && v[1];
  }, has: function(h) {
    return !!$.get(h);
  }, keys: function() {
    return b.map(function(h) {
      var v = i(h, 1), m = v[0];
      return m;
    });
  }, values: function() {
    return b.map(function(h) {
      var v = i(h, 2), m = v[1];
      return m;
    });
  } }, D = (0, e.default)($, $.entries());
  return go.default = D, go;
}
var Ro = {}, uh;
function mx() {
  if (uh) return Ro;
  uh = 1, Object.defineProperty(Ro, "__esModule", { value: true }), Ro.default = void 0;
  var e = a(nl()), t = a(sl());
  function a(h) {
    return h && h.__esModule ? h : { default: h };
  }
  function i(h, v) {
    return n(h) || d(h, v) || x(h, v) || l();
  }
  function l() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function d(h, v) {
    var m = h == null ? null : typeof Symbol < "u" && h[Symbol.iterator] || h["@@iterator"];
    if (m != null) {
      var _ = [], R = true, q = false, A, L;
      try {
        for (m = m.call(h); !(R = (A = m.next()).done) && (_.push(A.value), !(v && _.length === v)); R = true) ;
      } catch (r) {
        q = true, L = r;
      } finally {
        try {
          !R && m.return != null && m.return();
        } finally {
          if (q) throw L;
        }
      }
      return _;
    }
  }
  function n(h) {
    if (Array.isArray(h)) return h;
  }
  function P(h, v) {
    var m = typeof Symbol < "u" && h[Symbol.iterator] || h["@@iterator"];
    if (!m) {
      if (Array.isArray(h) || (m = x(h)) || v) {
        m && (h = m);
        var _ = 0, R = function() {
        };
        return { s: R, n: function() {
          return _ >= h.length ? { done: true } : { done: false, value: h[_++] };
        }, e: function(s) {
          throw s;
        }, f: R };
      }
      throw new TypeError(`Invalid attempt to iterate non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
    }
    var q = true, A = false, L;
    return { s: function() {
      m = m.call(h);
    }, n: function() {
      var s = m.next();
      return q = s.done, s;
    }, e: function(s) {
      A = true, L = s;
    }, f: function() {
      try {
        !q && m.return != null && m.return();
      } finally {
        if (A) throw L;
      }
    } };
  }
  function x(h, v) {
    if (h) {
      if (typeof h == "string") return T(h, v);
      var m = Object.prototype.toString.call(h).slice(8, -1);
      if (m === "Object" && h.constructor && (m = h.constructor.name), m === "Map" || m === "Set") return Array.from(h);
      if (m === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(m)) return T(h, v);
    }
  }
  function T(h, v) {
    (v == null || v > h.length) && (v = h.length);
    for (var m = 0, _ = new Array(v); m < v; m++) _[m] = h[m];
    return _;
  }
  var b = [], k = P(e.default.entries()), I;
  try {
    var E = function() {
      var v = i(I.value, 2), m = v[0], _ = v[1], R = _.relatedConcepts;
      Array.isArray(R) && R.forEach(function(q) {
        if (q.module === "HTML") {
          var A = q.concept;
          if (A != null) {
            for (var L = JSON.stringify(A), r, s = 0; s < b.length; s++) {
              var o = b[s][0];
              if (JSON.stringify(o) === L) {
                r = b[s][1];
                break;
              }
            }
            Array.isArray(r) || (r = []);
            var u = r.findIndex(function(p) {
              return p === m;
            });
            u === -1 && r.push(m), s < b.length ? b.splice(s, 1, [A, r]) : b.push([A, r]);
          }
        }
      });
    };
    for (k.s(); !(I = k.n()).done; ) E();
  } catch (h) {
    k.e(h);
  } finally {
    k.f();
  }
  function $(h, v) {
    if (h === void 0 && v !== void 0 || h !== void 0 && v === void 0) return false;
    if (h !== void 0 && v !== void 0) {
      if (h.length != v.length) return false;
      for (var m = 0; m < h.length; m++) if (v[m].name !== h[m].name || v[m].value !== h[m].value) return false;
    }
    return true;
  }
  var D = { entries: function() {
    return b;
  }, forEach: function(v) {
    for (var m = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, _ = 0, R = b; _ < R.length; _++) {
      var q = i(R[_], 2), A = q[0], L = q[1];
      v.call(m, L, A, b);
    }
  }, get: function(v) {
    var m = b.find(function(_) {
      return v.name === _[0].name && $(v.attributes, _[0].attributes);
    });
    return m && m[1];
  }, has: function(v) {
    return !!D.get(v);
  }, keys: function() {
    return b.map(function(v) {
      var m = i(v, 1), _ = m[0];
      return _;
    });
  }, values: function() {
    return b.map(function(v) {
      var m = i(v, 2), _ = m[1];
      return _;
    });
  } }, S = (0, t.default)(D, D.entries());
  return Ro.default = S, Ro;
}
var ch;
function vx() {
  if (ch) return xt;
  ch = 1, Object.defineProperty(xt, "__esModule", { value: true }), xt.elementAXObjects = xt.AXObjects = xt.AXObjectRoles = xt.AXObjectElements = void 0;
  var e = l(fx()), t = l(hx()), a = l(nl()), i = l(mx());
  function l(T) {
    return T && T.__esModule ? T : { default: T };
  }
  var d = e.default;
  xt.AXObjectElements = d;
  var n = t.default;
  xt.AXObjectRoles = n;
  var P = a.default;
  xt.AXObjects = P;
  var x = i.default;
  return xt.elementAXObjects = x, xt;
}
var rr = vx();
const fm = Mo.roles.keys(), bx = fm.filter((e) => {
  var _a3;
  return (_a3 = Mo.roles.get(e)) == null ? void 0 : _a3.abstract;
}), hm = fm.filter((e) => !bx.includes(e)), mm = hm.filter((e) => {
  const t = Mo.roles.get(e);
  return !["toolbar", "tabpanel", "generic", "cell"].includes(e) && !(t == null ? void 0 : t.superClass.some((a) => a.includes("widget") || a.includes("window")));
}).concat("progressbar"), yx = hm.filter((e) => !mm.includes(e) && e !== "generic");
Mo.elementRoles.entries().forEach(([e, t]) => {
  [...t].every((a) => a !== "generic" && mm.includes(a));
});
Mo.elementRoles.entries().forEach(([e, t]) => {
  [...t].every((a) => yx.includes(a));
});
const gx = [...rr.AXObjects.keys()].filter((e) => rr.AXObjects.get(e).type === "widget"), Rx = [...rr.AXObjects.keys()].filter((e) => ["windows", "structure"].includes(rr.AXObjects.get(e).type));
rr.elementAXObjects.entries().forEach(([e, t]) => {
  [...t].every((a) => gx.includes(a));
});
rr.elementAXObjects.entries().forEach(([e, t]) => {
  [...t].every((a) => Rx.includes(a));
});
var xo = { exports: {} }, xx = xo.exports, dh;
function Ll() {
  return dh || (dh = 1, function(e, t) {
    (function(a, i) {
      i(t);
    })(xx, function(a) {
      const d = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/", n = new Uint8Array(64), P = new Uint8Array(128);
      for (let o = 0; o < d.length; o++) {
        const u = d.charCodeAt(o);
        n[o] = u, P[u] = o;
      }
      function x(o, u) {
        let p = 0, f = 0, g = 0;
        do {
          const C = o.next();
          g = P[C], p |= (g & 31) << f, f += 5;
        } while (g & 32);
        const M = p & 1;
        return p >>>= 1, M && (p = -2147483648 | -p), u + p;
      }
      function T(o, u, p) {
        let f = u - p;
        f = f < 0 ? -f << 1 | 1 : f << 1;
        do {
          let g = f & 31;
          f >>>= 5, f > 0 && (g |= 32), o.write(n[g]);
        } while (f > 0);
        return u;
      }
      function b(o, u) {
        return o.pos >= u ? false : o.peek() !== 44;
      }
      const k = 1024 * 16, I = typeof TextDecoder < "u" ? new TextDecoder() : typeof Buffer < "u" ? { decode(o) {
        return Buffer.from(o.buffer, o.byteOffset, o.byteLength).toString();
      } } : { decode(o) {
        let u = "";
        for (let p = 0; p < o.length; p++) u += String.fromCharCode(o[p]);
        return u;
      } };
      class E {
        constructor() {
          this.pos = 0, this.out = "", this.buffer = new Uint8Array(k);
        }
        write(u) {
          const { buffer: p } = this;
          p[this.pos++] = u, this.pos === k && (this.out += I.decode(p), this.pos = 0);
        }
        flush() {
          const { buffer: u, out: p, pos: f } = this;
          return f > 0 ? p + I.decode(u.subarray(0, f)) : p;
        }
      }
      class $ {
        constructor(u) {
          this.pos = 0, this.buffer = u;
        }
        next() {
          return this.buffer.charCodeAt(this.pos++);
        }
        peek() {
          return this.buffer.charCodeAt(this.pos);
        }
        indexOf(u) {
          const { buffer: p, pos: f } = this, g = p.indexOf(u, f);
          return g === -1 ? p.length : g;
        }
      }
      const D = [];
      function S(o) {
        const { length: u } = o, p = new $(o), f = [], g = [];
        let M = 0;
        for (; p.pos < u; p.pos++) {
          M = x(p, M);
          const C = x(p, 0);
          if (!b(p, u)) {
            const K = g.pop();
            K[2] = M, K[3] = C;
            continue;
          }
          const y = x(p, 0), V = x(p, 0) & 1 ? [M, C, 0, 0, y, x(p, 0)] : [M, C, 0, 0, y];
          let U = D;
          if (b(p, u)) {
            U = [];
            do {
              const K = x(p, 0);
              U.push(K);
            } while (b(p, u));
          }
          V.vars = U, f.push(V), g.push(V);
        }
        return f;
      }
      function h(o) {
        const u = new E();
        for (let p = 0; p < o.length; ) p = v(o, p, u, [0]);
        return u.flush();
      }
      function v(o, u, p, f) {
        const g = o[u], { 0: M, 1: C, 2: y, 3: O, 4: N, vars: V } = g;
        u > 0 && p.write(44), f[0] = T(p, M, f[0]), T(p, C, 0), T(p, N, 0);
        const U = g.length === 6 ? 1 : 0;
        T(p, U, 0), g.length === 6 && T(p, g[5], 0);
        for (const K of V) T(p, K, 0);
        for (u++; u < o.length; ) {
          const K = o[u], { 0: X, 1: H } = K;
          if (X > y || X === y && H >= O) break;
          u = v(o, u, p, f);
        }
        return p.write(44), f[0] = T(p, y, f[0]), T(p, O, 0), u;
      }
      function m(o) {
        const { length: u } = o, p = new $(o), f = [], g = [];
        let M = 0, C = 0, y = 0, O = 0, N = 0, V = 0, U = 0, K = 0;
        do {
          const X = p.indexOf(";");
          let H = 0;
          for (; p.pos < X; p.pos++) {
            if (H = x(p, H), !b(p, X)) {
              const Me = g.pop();
              Me[2] = M, Me[3] = H;
              continue;
            }
            const re = x(p, 0), ue = re & 1, ye = re & 2, Re = re & 4;
            let Ae = null, je = D, Le;
            if (ue) {
              const Me = x(p, C);
              y = x(p, C === Me ? y : 0), C = Me, Le = [M, H, 0, 0, Me, y];
            } else Le = [M, H, 0, 0];
            if (Le.isScope = !!Re, ye) {
              const Me = O, ze = N;
              O = x(p, O);
              const Te = Me === O;
              N = x(p, Te ? N : 0), V = x(p, Te && ze === N ? V : 0), Ae = [O, N, V];
            }
            if (Le.callsite = Ae, b(p, X)) {
              je = [];
              do {
                U = M, K = H;
                const Me = x(p, 0);
                let ze;
                if (Me < -1) {
                  ze = [[x(p, 0)]];
                  for (let Te = -1; Te > Me; Te--) {
                    const ct = U;
                    U = x(p, U), K = x(p, U === ct ? K : 0);
                    const Ue = x(p, 0);
                    ze.push([Ue, U, K]);
                  }
                } else ze = [[Me]];
                je.push(ze);
              } while (b(p, X));
            }
            Le.bindings = je, f.push(Le), g.push(Le);
          }
          M++, p.pos = X + 1;
        } while (p.pos < u);
        return f;
      }
      function _(o) {
        if (o.length === 0) return "";
        const u = new E();
        for (let p = 0; p < o.length; ) p = R(o, p, u, [0, 0, 0, 0, 0, 0, 0]);
        return u.flush();
      }
      function R(o, u, p, f) {
        const g = o[u], { 0: M, 1: C, 2: y, 3: O, isScope: N, callsite: V, bindings: U } = g;
        f[0] < M ? (q(p, f[0], M), f[0] = M, f[1] = 0) : u > 0 && p.write(44), f[1] = T(p, g[1], f[1]);
        const K = (g.length === 6 ? 1 : 0) | (V ? 2 : 0) | (N ? 4 : 0);
        if (T(p, K, 0), g.length === 6) {
          const { 4: X, 5: H } = g;
          X !== f[2] && (f[3] = 0), f[2] = T(p, X, f[2]), f[3] = T(p, H, f[3]);
        }
        if (V) {
          const { 0: X, 1: H, 2: re } = g.callsite;
          X !== f[4] ? (f[5] = 0, f[6] = 0) : H !== f[5] && (f[6] = 0), f[4] = T(p, X, f[4]), f[5] = T(p, H, f[5]), f[6] = T(p, re, f[6]);
        }
        if (U) for (const X of U) {
          X.length > 1 && T(p, -X.length, 0);
          const H = X[0][0];
          T(p, H, 0);
          let re = M, ue = C;
          for (let ye = 1; ye < X.length; ye++) {
            const Re = X[ye];
            re = T(p, Re[1], re), ue = T(p, Re[2], ue), T(p, Re[0], 0);
          }
        }
        for (u++; u < o.length; ) {
          const X = o[u], { 0: H, 1: re } = X;
          if (H > y || H === y && re >= O) break;
          u = R(o, u, p, f);
        }
        return f[0] < y ? (q(p, f[0], y), f[0] = y, f[1] = 0) : p.write(44), f[1] = T(p, O, f[1]), u;
      }
      function q(o, u, p) {
        do
          o.write(59);
        while (++u < p);
      }
      function A(o) {
        const { length: u } = o, p = new $(o), f = [];
        let g = 0, M = 0, C = 0, y = 0, O = 0;
        do {
          const N = p.indexOf(";"), V = [];
          let U = true, K = 0;
          for (g = 0; p.pos < N; ) {
            let X;
            g = x(p, g), g < K && (U = false), K = g, b(p, N) ? (M = x(p, M), C = x(p, C), y = x(p, y), b(p, N) ? (O = x(p, O), X = [g, M, C, y, O]) : X = [g, M, C, y]) : X = [g], V.push(X), p.pos++;
          }
          U || L(V), f.push(V), p.pos = N + 1;
        } while (p.pos <= u);
        return f;
      }
      function L(o) {
        o.sort(r);
      }
      function r(o, u) {
        return o[0] - u[0];
      }
      function s(o) {
        const u = new E();
        let p = 0, f = 0, g = 0, M = 0;
        for (let C = 0; C < o.length; C++) {
          const y = o[C];
          if (C > 0 && u.write(59), y.length === 0) continue;
          let O = 0;
          for (let N = 0; N < y.length; N++) {
            const V = y[N];
            N > 0 && u.write(44), O = T(u, V[0], O), V.length !== 1 && (p = T(u, V[1], p), f = T(u, V[2], f), g = T(u, V[3], g), V.length !== 4 && (M = T(u, V[4], M)));
          }
        }
        return u.flush();
      }
      a.decode = A, a.decodeGeneratedRanges = m, a.decodeOriginalScopes = S, a.encode = s, a.encodeGeneratedRanges = _, a.encodeOriginalScopes = h, Object.defineProperty(a, "__esModule", { value: true });
    });
  }(xo, xo.exports)), xo.exports;
}
Ll();
var Vo = { exports: {} }, _o = { exports: {} }, Ho = { exports: {} }, _x = Ho.exports, ph;
function Cx() {
  return ph || (ph = 1, function(e, t) {
    (function(a, i) {
      e.exports = i();
    })(_x, function() {
      const a = /^[\w+.-]+:\/\//, i = /^([\w+.-]+:)\/\/([^@/#?]*@)?([^:/#?]*)(:\d+)?(\/[^#?]*)?(\?[^#]*)?(#.*)?/, l = /^file:(?:\/\/((?![a-z]:)[^/#?]*)?)?(\/?[^#?]*)(\?[^#]*)?(#.*)?/i;
      function d(v) {
        return a.test(v);
      }
      function n(v) {
        return v.startsWith("//");
      }
      function P(v) {
        return v.startsWith("/");
      }
      function x(v) {
        return v.startsWith("file:");
      }
      function T(v) {
        return /^[.?#]/.test(v);
      }
      function b(v) {
        const m = i.exec(v);
        return I(m[1], m[2] || "", m[3], m[4] || "", m[5] || "/", m[6] || "", m[7] || "");
      }
      function k(v) {
        const m = l.exec(v), _ = m[2];
        return I("file:", "", m[1] || "", "", P(_) ? _ : "/" + _, m[3] || "", m[4] || "");
      }
      function I(v, m, _, R, q, A, L) {
        return { scheme: v, user: m, host: _, port: R, path: q, query: A, hash: L, type: 7 };
      }
      function E(v) {
        if (n(v)) {
          const _ = b("http:" + v);
          return _.scheme = "", _.type = 6, _;
        }
        if (P(v)) {
          const _ = b("http://foo.com" + v);
          return _.scheme = "", _.host = "", _.type = 5, _;
        }
        if (x(v)) return k(v);
        if (d(v)) return b(v);
        const m = b("http://foo.com/" + v);
        return m.scheme = "", m.host = "", m.type = v ? v.startsWith("?") ? 3 : v.startsWith("#") ? 2 : 4 : 1, m;
      }
      function $(v) {
        if (v.endsWith("/..")) return v;
        const m = v.lastIndexOf("/");
        return v.slice(0, m + 1);
      }
      function D(v, m) {
        S(m, m.type), v.path === "/" ? v.path = m.path : v.path = $(m.path) + v.path;
      }
      function S(v, m) {
        const _ = m <= 4, R = v.path.split("/");
        let q = 1, A = 0, L = false;
        for (let s = 1; s < R.length; s++) {
          const o = R[s];
          if (!o) {
            L = true;
            continue;
          }
          if (L = false, o !== ".") {
            if (o === "..") {
              A ? (L = true, A--, q--) : _ && (R[q++] = o);
              continue;
            }
            R[q++] = o, A++;
          }
        }
        let r = "";
        for (let s = 1; s < q; s++) r += "/" + R[s];
        (!r || L && !r.endsWith("/..")) && (r += "/"), v.path = r;
      }
      function h(v, m) {
        if (!v && !m) return "";
        const _ = E(v);
        let R = _.type;
        if (m && R !== 7) {
          const A = E(m), L = A.type;
          switch (R) {
            case 1:
              _.hash = A.hash;
            case 2:
              _.query = A.query;
            case 3:
            case 4:
              D(_, A);
            case 5:
              _.user = A.user, _.host = A.host, _.port = A.port;
            case 6:
              _.scheme = A.scheme;
          }
          L > R && (R = L);
        }
        S(_, R);
        const q = _.query + _.hash;
        switch (R) {
          case 2:
          case 3:
            return q;
          case 4: {
            const A = _.path.slice(1);
            return A ? T(m || v) && !T(A) ? "./" + A + q : A + q : q || ".";
          }
          case 5:
            return _.path + q;
          default:
            return _.scheme + "//" + _.user + _.host + _.port + _.path + q;
        }
      }
      return h;
    });
  }(Ho)), Ho.exports;
}
var Px = _o.exports, fh;
function vm() {
  return fh || (fh = 1, function(e, t) {
    (function(a, i) {
      i(t, Ll(), Cx());
    })(Px, function(a, i, l) {
      function d(B, G) {
        return G && !G.endsWith("/") && (G += "/"), l(B, G);
      }
      function n(B) {
        if (!B) return "";
        const G = B.lastIndexOf("/");
        return B.slice(0, G + 1);
      }
      const P = 0, x = 1, T = 2, b = 3, k = 4, I = 1, E = 2;
      function $(B, G) {
        const z = D(B, 0);
        if (z === B.length) return B;
        G || (B = B.slice());
        for (let Y = z; Y < B.length; Y = D(B, Y + 1)) B[Y] = h(B[Y], G);
        return B;
      }
      function D(B, G) {
        for (let z = G; z < B.length; z++) if (!S(B[z])) return z;
        return B.length;
      }
      function S(B) {
        for (let G = 1; G < B.length; G++) if (B[G][P] < B[G - 1][P]) return false;
        return true;
      }
      function h(B, G) {
        return G || (B = B.slice()), B.sort(v);
      }
      function v(B, G) {
        return B[P] - G[P];
      }
      let m = false;
      function _(B, G, z, Y) {
        for (; z <= Y; ) {
          const ne = z + (Y - z >> 1), ae = B[ne][P] - G;
          if (ae === 0) return m = true, ne;
          ae < 0 ? z = ne + 1 : Y = ne - 1;
        }
        return m = false, z - 1;
      }
      function R(B, G, z) {
        for (let Y = z + 1; Y < B.length && B[Y][P] === G; z = Y++) ;
        return z;
      }
      function q(B, G, z) {
        for (let Y = z - 1; Y >= 0 && B[Y][P] === G; z = Y--) ;
        return z;
      }
      function A() {
        return { lastKey: -1, lastNeedle: -1, lastIndex: -1 };
      }
      function L(B, G, z, Y) {
        const { lastKey: ne, lastNeedle: ae, lastIndex: ve } = z;
        let qe = 0, de = B.length - 1;
        if (Y === ne) {
          if (G === ae) return m = ve !== -1 && B[ve][P] === G, ve;
          G >= ae ? qe = ve === -1 ? 0 : ve : de = ve;
        }
        return z.lastKey = Y, z.lastNeedle = G, z.lastIndex = _(B, G, qe, de);
      }
      function r(B, G) {
        const z = G.map(o);
        for (let Y = 0; Y < B.length; Y++) {
          const ne = B[Y];
          for (let ae = 0; ae < ne.length; ae++) {
            const ve = ne[ae];
            if (ve.length === 1) continue;
            const qe = ve[x], de = ve[T], xe = ve[b], He = z[qe], ke = He[de] || (He[de] = []), we = G[qe];
            let $e = R(ke, xe, L(ke, xe, we, de));
            we.lastIndex = ++$e, s(ke, $e, [xe, Y, ve[P]]);
          }
        }
        return z;
      }
      function s(B, G, z) {
        for (let Y = B.length; Y > G; Y--) B[Y] = B[Y - 1];
        B[G] = z;
      }
      function o() {
        return { __proto__: null };
      }
      const u = function(B, G) {
        const z = p(B);
        if (!("sections" in z)) return new U(z, G);
        const Y = [], ne = [], ae = [], ve = [], qe = [];
        f(z, G, Y, ne, ae, ve, qe, 0, 0, 1 / 0, 1 / 0);
        const de = { version: 3, file: z.file, names: ve, sources: ne, sourcesContent: ae, mappings: Y, ignoreList: qe };
        return ze(de);
      };
      function p(B) {
        return typeof B == "string" ? JSON.parse(B) : B;
      }
      function f(B, G, z, Y, ne, ae, ve, qe, de, xe, He) {
        const { sections: ke } = B;
        for (let we = 0; we < ke.length; we++) {
          const { map: $e, offset: Qe } = ke[we];
          let et = xe, st = He;
          if (we + 1 < ke.length) {
            const pt = ke[we + 1].offset;
            et = Math.min(xe, qe + pt.line), et === xe ? st = Math.min(He, de + pt.column) : et < xe && (st = de + pt.column);
          }
          g($e, G, z, Y, ne, ae, ve, qe + Qe.line, de + Qe.column, et, st);
        }
      }
      function g(B, G, z, Y, ne, ae, ve, qe, de, xe, He) {
        const ke = p(B);
        if ("sections" in ke) return f(...arguments);
        const we = new U(ke, G), $e = Y.length, Qe = ae.length, et = H(we), { resolvedSources: st, sourcesContent: pt, ignoreList: At } = we;
        if (M(Y, st), M(ae, we.names), pt) M(ne, pt);
        else for (let Ke = 0; Ke < st.length; Ke++) ne.push(null);
        if (At) for (let Ke = 0; Ke < At.length; Ke++) ve.push(At[Ke] + $e);
        for (let Ke = 0; Ke < et.length; Ke++) {
          const wt = qe + Ke;
          if (wt > xe) return;
          const Q = C(z, wt), Se = Ke === 0 ? de : 0, fe = et[Ke];
          for (let W = 0; W < fe.length; W++) {
            const Ge = fe[W], Ye = Se + Ge[P];
            if (wt === xe && Ye >= He) return;
            if (Ge.length === 1) {
              Q.push([Ye]);
              continue;
            }
            const Xe = $e + Ge[x], Je = Ge[T], Bt = Ge[b];
            Q.push(Ge.length === 4 ? [Ye, Xe, Je, Bt] : [Ye, Xe, Je, Bt, Qe + Ge[k]]);
          }
        }
      }
      function M(B, G) {
        for (let z = 0; z < G.length; z++) B.push(G[z]);
      }
      function C(B, G) {
        for (let z = B.length; z <= G; z++) B[z] = [];
        return B[G];
      }
      const y = "`line` must be greater than 0 (lines start at line 1)", O = "`column` must be greater than or equal to 0 (columns start at column 0)", N = -1, V = 1;
      class U {
        constructor(G, z) {
          const Y = typeof G == "string";
          if (!Y && G._decodedMemo) return G;
          const ne = Y ? JSON.parse(G) : G, { version: ae, file: ve, names: qe, sourceRoot: de, sources: xe, sourcesContent: He } = ne;
          this.version = ae, this.file = ve, this.names = qe || [], this.sourceRoot = de, this.sources = xe, this.sourcesContent = He, this.ignoreList = ne.ignoreList || ne.x_google_ignoreList || void 0;
          const ke = d(de || "", n(z));
          this.resolvedSources = xe.map(($e) => d($e || "", ke));
          const { mappings: we } = ne;
          typeof we == "string" ? (this._encoded = we, this._decoded = void 0) : (this._encoded = void 0, this._decoded = $(we, Y)), this._decodedMemo = A(), this._bySources = void 0, this._bySourceMemos = void 0;
        }
      }
      function K(B) {
        return B;
      }
      function X(B) {
        var G, z;
        return (G = (z = B)._encoded) !== null && G !== void 0 ? G : z._encoded = i.encode(B._decoded);
      }
      function H(B) {
        var G;
        return (G = B)._decoded || (G._decoded = i.decode(B._encoded));
      }
      function re(B, G, z) {
        const Y = H(B);
        if (G >= Y.length) return null;
        const ne = Y[G], ae = Rt(ne, B._decodedMemo, G, z, V);
        return ae === -1 ? null : ne[ae];
      }
      function ue(B, G) {
        let { line: z, column: Y, bias: ne } = G;
        if (z--, z < 0) throw new Error(y);
        if (Y < 0) throw new Error(O);
        const ae = H(B);
        if (z >= ae.length) return gt(null, null, null, null);
        const ve = ae[z], qe = Rt(ve, B._decodedMemo, z, Y, ne || V);
        if (qe === -1) return gt(null, null, null, null);
        const de = ve[qe];
        if (de.length === 1) return gt(null, null, null, null);
        const { names: xe, resolvedSources: He } = B;
        return gt(He[de[x]], de[T] + 1, de[b], de.length === 5 ? xe[de[k]] : null);
      }
      function ye(B, G) {
        const { source: z, line: Y, column: ne, bias: ae } = G;
        return St(B, z, Y, ne, ae || V, false);
      }
      function Re(B, G) {
        const { source: z, line: Y, column: ne, bias: ae } = G;
        return St(B, z, Y, ne, ae || N, true);
      }
      function Ae(B, G) {
        const z = H(B), { names: Y, resolvedSources: ne } = B;
        for (let ae = 0; ae < z.length; ae++) {
          const ve = z[ae];
          for (let qe = 0; qe < ve.length; qe++) {
            const de = ve[qe], xe = ae + 1, He = de[0];
            let ke = null, we = null, $e = null, Qe = null;
            de.length !== 1 && (ke = ne[de[1]], we = de[2] + 1, $e = de[3]), de.length === 5 && (Qe = Y[de[4]]), G({ generatedLine: xe, generatedColumn: He, source: ke, originalLine: we, originalColumn: $e, name: Qe });
          }
        }
      }
      function je(B, G) {
        const { sources: z, resolvedSources: Y } = B;
        let ne = z.indexOf(G);
        return ne === -1 && (ne = Y.indexOf(G)), ne;
      }
      function Le(B, G) {
        const { sourcesContent: z } = B;
        if (z == null) return null;
        const Y = je(B, G);
        return Y === -1 ? null : z[Y];
      }
      function Me(B, G) {
        const { ignoreList: z } = B;
        if (z == null) return false;
        const Y = je(B, G);
        return Y === -1 ? false : z.includes(Y);
      }
      function ze(B, G) {
        const z = new U(Ue(B, []), G);
        return z._decoded = B.mappings, z;
      }
      function Te(B) {
        return Ue(B, H(B));
      }
      function ct(B) {
        return Ue(B, X(B));
      }
      function Ue(B, G) {
        return { version: B.version, file: B.file, names: B.names, sourceRoot: B.sourceRoot, sources: B.sources, sourcesContent: B.sourcesContent, mappings: G, ignoreList: B.ignoreList || B.x_google_ignoreList };
      }
      function gt(B, G, z, Y) {
        return { source: B, line: G, column: z, name: Y };
      }
      function dt(B, G) {
        return { line: B, column: G };
      }
      function Rt(B, G, z, Y, ne) {
        let ae = L(B, Y, G, z);
        return m ? ae = (ne === N ? R : q)(B, Y, ae) : ne === N && ae++, ae === -1 || ae === B.length ? -1 : ae;
      }
      function Dt(B, G, z, Y, ne) {
        let ae = Rt(B, G, z, Y, V);
        if (!m && ne === N && ae++, ae === -1 || ae === B.length) return [];
        const ve = m ? Y : B[ae][P];
        m || (ae = q(B, ve, ae));
        const qe = R(B, ve, ae), de = [];
        for (; ae <= qe; ae++) {
          const xe = B[ae];
          de.push(dt(xe[I] + 1, xe[E]));
        }
        return de;
      }
      function St(B, G, z, Y, ne, ae) {
        var ve;
        if (z--, z < 0) throw new Error(y);
        if (Y < 0) throw new Error(O);
        const { sources: qe, resolvedSources: de } = B;
        let xe = qe.indexOf(G);
        if (xe === -1 && (xe = de.indexOf(G)), xe === -1) return ae ? [] : dt(null, null);
        const ke = ((ve = B)._bySources || (ve._bySources = r(H(B), B._bySourceMemos = qe.map(A))))[xe][z];
        if (ke == null) return ae ? [] : dt(null, null);
        const we = B._bySourceMemos[xe];
        if (ae) return Dt(ke, we, z, Y, ne);
        const $e = Rt(ke, we, z, Y, ne);
        if ($e === -1) return dt(null, null);
        const Qe = ke[$e];
        return dt(Qe[I] + 1, Qe[E]);
      }
      a.AnyMap = u, a.GREATEST_LOWER_BOUND = V, a.LEAST_UPPER_BOUND = N, a.TraceMap = U, a.allGeneratedPositionsFor = Re, a.decodedMap = Te, a.decodedMappings = H, a.eachMapping = Ae, a.encodedMap = ct, a.encodedMappings = X, a.generatedPositionFor = ye, a.isIgnored = Me, a.originalPositionFor = ue, a.presortedDecodedMap = ze, a.sourceContentFor = Le, a.traceSegment = re;
    });
  }(_o, _o.exports)), _o.exports;
}
var Co = { exports: {} }, Po = { exports: {} }, qx = Po.exports, hh;
function Tx() {
  return hh || (hh = 1, function(e, t) {
    (function(a, i) {
      i(t);
    })(qx, function(a) {
      class i {
        constructor() {
          this._indexes = { __proto__: null }, this.array = [];
        }
      }
      function l(T) {
        return T;
      }
      function d(T, b) {
        return T._indexes[b];
      }
      function n(T, b) {
        const k = d(T, b);
        if (k !== void 0) return k;
        const { array: I, _indexes: E } = T, $ = I.push(b);
        return E[b] = $ - 1;
      }
      function P(T) {
        const { array: b, _indexes: k } = T;
        if (b.length === 0) return;
        const I = b.pop();
        k[I] = void 0;
      }
      function x(T, b) {
        const k = d(T, b);
        if (k === void 0) return;
        const { array: I, _indexes: E } = T;
        for (let $ = k + 1; $ < I.length; $++) {
          const D = I[$];
          I[$ - 1] = D, E[D]--;
        }
        E[b] = void 0, I.pop();
      }
      a.SetArray = i, a.get = d, a.pop = P, a.put = n, a.remove = x, Object.defineProperty(a, "__esModule", { value: true });
    });
  }(Po, Po.exports)), Po.exports;
}
var Sx = Co.exports, mh;
function Ax() {
  return mh || (mh = 1, function(e, t) {
    (function(a, i) {
      i(t, Tx(), Ll(), vm());
    })(Sx, function(a, i, l, d) {
      class I {
        constructor({ file: y, sourceRoot: O } = {}) {
          this._names = new i.SetArray(), this._sources = new i.SetArray(), this._sourcesContent = [], this._mappings = [], this.file = y, this.sourceRoot = O, this._ignoreList = new i.SetArray();
        }
      }
      function E(C) {
        return C;
      }
      function $(C, y, O, N, V, U, K, X) {
        return L(false, C, y, O, N, V, U, K, X);
      }
      function D(C, y) {
        return M(false, C, y);
      }
      const S = (C, y, O, N, V, U, K, X) => L(true, C, y, O, N, V, U, K, X), h = (C, y) => M(true, C, y);
      function v(C, y, O) {
        const { _sources: N, _sourcesContent: V } = C, U = i.put(N, y);
        V[U] = O;
      }
      function m(C, y, O = true) {
        const { _sources: N, _sourcesContent: V, _ignoreList: U } = C, K = i.put(N, y);
        K === V.length && (V[K] = null), O ? i.put(U, K) : i.remove(U, K);
      }
      function _(C) {
        const { _mappings: y, _sources: O, _sourcesContent: N, _names: V, _ignoreList: U } = C;
        return u(y), { version: 3, file: C.file || void 0, names: V.array, sourceRoot: C.sourceRoot || void 0, sources: O.array, sourcesContent: N, mappings: y, ignoreList: U.array };
      }
      function R(C) {
        const y = _(C);
        return Object.assign(Object.assign({}, y), { mappings: l.encode(y.mappings) });
      }
      function q(C) {
        const y = new d.TraceMap(C), O = new I({ file: y.file, sourceRoot: y.sourceRoot });
        return p(O._names, y.names), p(O._sources, y.sources), O._sourcesContent = y.sourcesContent || y.sources.map(() => null), O._mappings = d.decodedMappings(y), y.ignoreList && p(O._ignoreList, y.ignoreList), O;
      }
      function A(C) {
        const y = [], { _mappings: O, _sources: N, _names: V } = C;
        for (let U = 0; U < O.length; U++) {
          const K = O[U];
          for (let X = 0; X < K.length; X++) {
            const H = K[X], re = { line: U + 1, column: H[0] };
            let ue, ye, Re;
            H.length !== 1 && (ue = N.array[H[1]], ye = { line: H[2] + 1, column: H[3] }, H.length === 5 && (Re = V.array[H[4]])), y.push({ generated: re, source: ue, original: ye, name: Re });
          }
        }
        return y;
      }
      function L(C, y, O, N, V, U, K, X, H) {
        const { _mappings: re, _sources: ue, _sourcesContent: ye, _names: Re } = y, Ae = r(re, O), je = s(Ae, N);
        if (!V) return C && f(Ae, je) ? void 0 : o(Ae, je, [N]);
        const Le = i.put(ue, V), Me = X ? i.put(Re, X) : -1;
        if (Le === ye.length && (ye[Le] = H ?? null), !(C && g(Ae, je, Le, U, K, Me))) return o(Ae, je, X ? [N, Le, U, K, Me] : [N, Le, U, K]);
      }
      function r(C, y) {
        for (let O = C.length; O <= y; O++) C[O] = [];
        return C[y];
      }
      function s(C, y) {
        let O = C.length;
        for (let N = O - 1; N >= 0; O = N--) {
          const V = C[N];
          if (y >= V[0]) break;
        }
        return O;
      }
      function o(C, y, O) {
        for (let N = C.length; N > y; N--) C[N] = C[N - 1];
        C[y] = O;
      }
      function u(C) {
        const { length: y } = C;
        let O = y;
        for (let N = O - 1; N >= 0 && !(C[N].length > 0); O = N, N--) ;
        O < y && (C.length = O);
      }
      function p(C, y) {
        for (let O = 0; O < y.length; O++) i.put(C, y[O]);
      }
      function f(C, y) {
        return y === 0 ? true : C[y - 1].length === 1;
      }
      function g(C, y, O, N, V, U) {
        if (y === 0) return false;
        const K = C[y - 1];
        return K.length === 1 ? false : O === K[1] && N === K[2] && V === K[3] && U === (K.length === 5 ? K[4] : -1);
      }
      function M(C, y, O) {
        const { generated: N, source: V, original: U, name: K, content: X } = O;
        return V ? L(C, y, N.line - 1, N.column, V, U.line - 1, U.column, K, X) : L(C, y, N.line - 1, N.column, null, null, null, null, null);
      }
      a.GenMapping = I, a.addMapping = D, a.addSegment = $, a.allMappings = A, a.fromMap = q, a.maybeAddMapping = h, a.maybeAddSegment = S, a.setIgnore = m, a.setSourceContent = v, a.toDecodedMap = _, a.toEncodedMap = R, Object.defineProperty(a, "__esModule", { value: true });
    });
  }(Co, Co.exports)), Co.exports;
}
var wx = Vo.exports, vh;
function Ex() {
  return vh || (vh = 1, function(e, t) {
    (function(a, i) {
      e.exports = i(vm(), Ax());
    })(wx, function(a, i) {
      const l = n("", -1, -1, "", null, false), d = [];
      function n(h, v, m, _, R, q) {
        return { source: h, line: v, column: m, name: _, content: R, ignore: q };
      }
      function P(h, v, m, _, R) {
        return { map: h, sources: v, source: m, content: _, ignore: R };
      }
      function x(h, v) {
        return P(h, v, "", null, false);
      }
      function T(h, v, m) {
        return P(null, d, h, v, m);
      }
      function b(h) {
        const v = new i.GenMapping({ file: h.map.file }), { sources: m, map: _ } = h, R = _.names, q = a.decodedMappings(_);
        for (let A = 0; A < q.length; A++) {
          const L = q[A];
          for (let r = 0; r < L.length; r++) {
            const s = L[r], o = s[0];
            let u = l;
            if (s.length !== 1) {
              const O = m[s[1]];
              if (u = k(O, s[2], s[3], s.length === 5 ? R[s[4]] : ""), u == null) continue;
            }
            const { column: p, line: f, name: g, content: M, source: C, ignore: y } = u;
            i.maybeAddSegment(v, A, o, C, f, p, g), C && M != null && i.setSourceContent(v, C, M), y && i.setIgnore(v, C, true);
          }
        }
        return v;
      }
      function k(h, v, m, _) {
        if (!h.map) return n(h.source, v, m, _, h.content, h.ignore);
        const R = a.traceSegment(h.map, v, m);
        return R == null ? null : R.length === 1 ? l : k(h.sources[R[1]], R[2], R[3], R.length === 5 ? h.map.names[R[4]] : _);
      }
      function I(h) {
        return Array.isArray(h) ? h : [h];
      }
      function E(h, v) {
        const m = I(h).map((q) => new a.TraceMap(q, "")), _ = m.pop();
        for (let q = 0; q < m.length; q++) if (m[q].sources.length > 1) throw new Error(`Transformation map ${q} must have exactly one source file.
Did you specify these with the most recent transformation maps first?`);
        let R = $(_, v, "", 0);
        for (let q = m.length - 1; q >= 0; q--) R = x(m[q], [R]);
        return R;
      }
      function $(h, v, m, _) {
        const { resolvedSources: R, sourcesContent: q, ignoreList: A } = h, L = _ + 1, r = R.map((s, o) => {
          const u = { importer: m, depth: L, source: s || "", content: void 0, ignore: void 0 }, p = v(u.source, u), { source: f, content: g, ignore: M } = u;
          if (p) return $(new a.TraceMap(p, f), v, f, L);
          const C = g !== void 0 ? g : q ? q[o] : null, y = M !== void 0 ? M : A ? A.includes(o) : false;
          return T(f, C, y);
        });
        return x(h, r);
      }
      class D {
        constructor(v, m) {
          const _ = m.decodedMappings ? i.toDecodedMap(v) : i.toEncodedMap(v);
          this.version = _.version, this.file = _.file, this.mappings = _.mappings, this.names = _.names, this.ignoreList = _.ignoreList, this.sourceRoot = _.sourceRoot, this.sources = _.sources, m.excludeContent || (this.sourcesContent = _.sourcesContent);
        }
        toString() {
          return JSON.stringify(this);
        }
      }
      function S(h, v, m) {
        const _ = typeof m == "object" ? m : { excludeContent: !!m, decodedMappings: false }, R = E(h, v);
        return new D(b(R), _);
      }
      return S;
    });
  }(Vo)), Vo.exports;
}
Ex();
const kx = { filename: To("(unknown)"), rootDir: To(typeof process < "u" ? (_a2 = process.cwd) == null ? void 0 : _a2.call(process) : typeof Deno < "u" ? Deno.cwd() : void 0), dev: _t(false), generate: Gt("client", (e, t) => e === "dom" || e === "ssr" ? (Ol(Yv), e === "dom" ? "client" : "server") : (e !== "client" && e !== "server" && e !== false && Ct(`${t} must be "client", "server" or false`), e)), warningFilter: bm(() => true) };
gh({ ...kx, accessors: yh(Gv, _t(false)), css: Gt("external", (e) => ((e === true || e === false) && Ct('The boolean options have been removed from the css option. Use "external" instead of false and "injected" instead of true'), e === "none" && Ct('css: "none" is no longer a valid option. If this was crucial for you, please open an issue on GitHub with your use case.'), e !== "external" && e !== "injected" && Ct('css should be either "external" (default, recommended) or "injected"'), e)), cssHash: bm(({ css: e, hash: t }) => `svelte-${t(e)}`), cssOutputFilename: To(void 0), customElement: _t(false), discloseVersion: _t(true), immutable: yh(Wv, _t(false)), legacy: Yt("The legacy option has been removed. If you are using this because of legacy.componentApi, use compatibility.componentApi instead"), compatibility: gh({ componentApi: Rh([4, 5], 5) }), loopGuardTimeout: vl(Xv), name: To(void 0), namespace: Rh(["html", "mathml", "svg"]), modernAst: _t(false), outputFilename: To(void 0), preserveComments: _t(false), preserveWhitespace: _t(false), runes: _t(void 0), hmr: _t(false), sourcemap: Gt(void 0, (e) => e), enableSourcemap: vl(zv), hydratable: vl(Kv), format: Yt('The format option has been removed in Svelte 4, the compiler only outputs ESM now. Remove "format" from your compiler options. If you did not set this yourself, bump the version of your bundler plugin (vite-plugin-svelte/rollup-plugin-svelte/svelte-loader)'), tag: Yt('The tag option has been removed in Svelte 5. Use `<svelte:options customElement="tag-name" />` inside the component instead. If that does not solve your use case, please open an issue on GitHub with details.'), sveltePath: Yt("The sveltePath option has been removed in Svelte 5. If this option was crucial for you, please open an issue on GitHub with your use case."), errorMode: Yt("The errorMode option has been removed. If you are using this through svelte-preprocess with TypeScript, use the https://www.typescriptlang.org/tsconfig#verbatimModuleSyntax setting instead"), varsReport: Yt("The vars option has been removed. If you are using this through svelte-preprocess with TypeScript, use the https://www.typescriptlang.org/tsconfig#verbatimModuleSyntax setting instead") });
function Yt(e) {
  return (t) => {
    t !== void 0 && i0(null, e);
  };
}
const bh = /* @__PURE__ */ new Set();
function Ol(e) {
  bh.has(e) || (bh.add(e), e(null));
}
function vl(e) {
  return (t) => {
    t !== void 0 && Ol(e);
  };
}
function yh(e, t) {
  return (a, i) => (a !== void 0 && Ol(e), t(a, i));
}
function gh(e, t = false) {
  return (a, i) => {
    const l = {};
    (a && typeof a != "object" || Array.isArray(a)) && Ct(`${i} should be an object`);
    for (const d in a) d in e || (t ? l[d] = a[d] : s0(null, `${i ? `${i}.${d}` : d}`));
    for (const d in e) {
      const n = e[d];
      l[d] = n(a && a[d], i ? `${i}.${d}` : d);
    }
    return l;
  };
}
function Gt(e, t) {
  return (a, i) => a === void 0 ? e : t(a, i);
}
function To(e, t = true) {
  return Gt(e, (a, i) => (typeof a != "string" && Ct(`${i} should be a string, if specified`), !t && a === "" && Ct(`${i} cannot be empty`), a));
}
function _t(e) {
  return Gt(e, (t, a) => (typeof t != "boolean" && Ct(`${a} should be true or false, if specified`), t));
}
function Rh(e, t = e[0]) {
  return Gt(t, (a, i) => {
    if (!e.includes(a)) {
      const l = e.length > 2 ? `${i} should be one of ${e.slice(0, -1).map((d) => `"${d}"`).join(", ")} or "${e[e.length - 1]}"` : `${i} should be either "${e[0]}" or "${e[1]}"`;
      Ct(l);
    }
    return a;
  });
}
function bm(e) {
  return Gt(e, (t, a) => (typeof t != "function" && Ct(`${a} should be a function, if specified`), t));
}
function Ct(e) {
  a0(null, e);
}
var Ix = Pe("<p> </p>"), Mx = Pe('<p> </p> <!> <div class="flex gap-05"><!> <!></div>', 1), Lx = Pe('<div class="err"> </div>'), Ox = Pe('<div class="container svelte-b4h84n"><!></div> <!>', 1);
function Nx(e, t) {
  Lt(t, true);
  let a = ar(), i = $t(), l = Z(""), d = Z(false), n = Z(Ve([])), P = Z(Ve([])), x = Z(Ve([]));
  xl(() => {
    T();
  }), kt(() => {
    b();
  }), kt(() => {
    F(x, w(n).map((m) => {
      var _a3;
      return { name: m.name, desc: m.desc, value: ((_a3 = w(P).find((R) => R.key === m.name)) == null ? void 0 : _a3.value) || "" };
    }), true);
  });
  async function T() {
    var _a3;
    let m = await jt("/auth/v1/users/attr");
    m.body ? F(n, m.body.values.toSorted((_, R) => _.name.localeCompare(R.name)), true) : F(l, ((_a3 = m.error) == null ? void 0 : _a3.message) || "Error fetching attrs", true);
  }
  async function b() {
    var _a3;
    let m = await jt(`/auth/v1/users/${t.user.id}/attr`);
    m.body ? F(P, m.body.values, true) : F(l, ((_a3 = m.error) == null ? void 0 : _a3.message) || "Error fetching users attrs", true);
  }
  async function k() {
    F(l, "");
    let m = w(P).map((q) => q.key), _ = { values: w(x).filter((q) => q.value.trim() || m.includes(q.name)).map((q) => ({ key: q.name, value: q.value.trim() })) }, R = await _l(`/auth/v1/users/${t.user.id}/attr`, _);
    R.error ? F(l, R.error.message, true) : (F(d, true), setTimeout(() => {
      F(d, false);
    }, 3e3)), t.onSave();
  }
  var I = Ox(), E = De(I), $ = le(E);
  {
    var D = (m) => {
      var _ = Ix(), R = le(_, true);
      oe(_), he(() => pe(R, i.common.noEntries)), ee(m, _);
    }, S = (m) => {
      var _ = Mx(), R = De(_), q = le(R, true);
      oe(R);
      var A = J(R, 2);
      Go(A, 17, () => w(x), xh, (u, p, f) => {
        const g = rt(() => w(p).desc || "JSON Value");
        mt(u, { autocomplete: "off", get label() {
          return w(p).name;
        }, get placeholder() {
          return w(g);
        }, get value() {
          return w(p).value;
        }, set value(M) {
          w(p).value = M;
        } });
      });
      var L = J(A, 2), r = le(L);
      Et(r, { onclick: k, children: (u, p) => {
        We();
        var f = Be();
        he(() => pe(f, a.common.save)), ee(u, f);
      }, $$slots: { default: true } });
      var s = J(r, 2);
      {
        var o = (u) => {
          Zo(u, {});
        };
        be(s, (u) => {
          w(d) && u(o);
        });
      }
      oe(L), he(() => pe(q, i.users.descAttr)), ee(m, _);
    };
    be($, (m) => {
      w(x).length === 0 ? m(D) : m(S, false);
    });
  }
  oe(E);
  var h = J(E, 2);
  {
    var v = (m) => {
      var _ = Lx(), R = le(_, true);
      oe(_), he(() => pe(R, w(l))), ee(m, _);
    };
    be(h, (m) => {
      w(l) && m(v);
    });
  }
  ee(e, I), Ot();
}
var Dx = Pe("<p><b> </b></p> <p> </p> <!>", 1), Bx = Pe('<div class="desc"><p><b> </b></p> <p> </p> <p> </p></div>'), Fx = Pe('<!> <!> <!> <div></div> <p> </p> <div class="flex gap-05"><!> <!> <!></div>', 1), jx = Pe("<div></div> <!> <!>", 1), $x = Pe('<div class="err"> </div>'), Vx = Pe("<!> <!>", 1);
function Hx(e, t) {
  Lt(t, true);
  const a = "min(20rem, calc(100dvw - .5rem))";
  let i = ar(), l = $t(), d = Z(false), n = Z(""), P = Z(false), x = Z(void 0), T = Z(false), b = Z(""), k = Z(""), I = rt(() => {
    var _a3;
    return w(b).length > (((_a3 = w(x)) == null ? void 0 : _a3.length_min) || 8) && w(b) === w(k);
  }), E = Z(void 0), $ = Z(void 0);
  xl(async () => {
    var _a3;
    let L = await jt("/auth/v1/password_policy");
    L.body ? F(x, L.body, true) : F(n, ((_a3 = L.error) == null ? void 0 : _a3.message) || "Error", true);
  }), kt(() => {
    t.user.id && (F(b, ""), F(k, ""));
  });
  async function D() {
    F(n, ""), F(d, true);
    let L = { email: t.user.email }, r = await _h("/auth/v1/users/request_reset", L);
    r.error ? F(n, r.error.message, true) : (F(P, true), setTimeout(() => {
      F(P, false);
    }, 3e3)), F(d, false);
  }
  async function S(L, r) {
    if (!w(T)) {
      F(n, i.account.passwordPolicyFollow, true);
      return;
    }
    if (w(b) !== w(k)) {
      F(n, i.account.passwordNoMatch, true);
      return;
    }
    let s = { email: t.user.email, given_name: t.user.given_name, family_name: t.user.family_name, language: t.user.language, password: w(b), roles: t.user.roles, groups: t.user.groups, enabled: t.user.enabled, email_verified: t.user.email_verified, user_expires: t.user.user_expires };
    F(n, ""), F(d, true);
    let o = await _l(L.action, s);
    o.error ? F(n, o.error.message, true) : (F(P, true), setTimeout(() => {
      F(P, false), t.onSave();
    }, 3e3)), F(d, false);
  }
  function h() {
    if (F(n, ""), !w(x)) return;
    let L = bv(w(x));
    F(b, L, true), F(k, L, true), requestAnimationFrame(() => {
      var _a3, _b2;
      (_a3 = w(E)) == null ? void 0 : _a3(), (_b2 = w($)) == null ? void 0 : _b2();
    });
  }
  var v = Vx(), m = De(v);
  {
    var _ = (L) => {
      var r = Dx(), s = De(r), o = le(s), u = le(o, true);
      oe(o), oe(s);
      var p = J(s, 2), f = le(p, true);
      oe(p);
      var g = J(p, 2);
      Et(g, { onclick: D, get isLoading() {
        return w(d);
      }, children: (M, C) => {
        We();
        var y = Be();
        he(() => pe(y, l.users.pwdSendEmailBtn)), ee(M, y);
      }, $$slots: { default: true } }), he(() => {
        pe(u, l.users.pwdNoInit), pe(f, l.users.pwdSendEmailDesc);
      }), ee(L, r);
    }, R = (L, r) => {
      {
        var s = (u) => {
          var p = Bx(), f = le(p), g = le(f), M = le(g, true);
          oe(g), oe(f);
          var C = J(f, 2), y = le(C, true);
          oe(C);
          var O = J(C, 2), N = le(O, true);
          oe(O), oe(p), he(() => {
            pe(M, l.users.pkOnly1), pe(y, l.users.pkOnly2), pe(N, l.users.pkOnly3);
          }), ee(u, p);
        }, o = (u) => {
          var p = jx(), f = De(p);
          Zt(f, "", {}, { "margin-top": ".5rem" });
          var g = J(f, 2);
          {
            var M = (O) => {
              xv(O, { get password() {
                return w(b);
              }, get policy() {
                return w(x);
              }, get accepted() {
                return w(T);
              }, set accepted(N) {
                F(T, N, true);
              } });
            };
            be(g, (O) => {
              w(x) && O(M);
            });
          }
          var C = J(g, 2);
          const y = rt(() => `/auth/v1/users/${t.user.id}`);
          Pl(C, { get action() {
            return w(y);
          }, onSubmit: S, children: (O, N) => {
            var V = Fx(), U = De(V);
            const K = rt(() => {
              var _a3;
              return ((_a3 = w(x)) == null ? void 0 : _a3.length_max) || 256;
            });
            Fl(U, { autocomplete: "off", get label() {
              return i.account.passwordNew;
            }, get placeholder() {
              return i.account.passwordNew;
            }, get showCopy() {
              return w(I);
            }, required: true, get maxLength() {
              return w(K);
            }, width: a, get value() {
              return w(b);
            }, set value(Te) {
              F(b, Te, true);
            }, get reportValidity() {
              return w(E);
            }, set reportValidity(Te) {
              F(E, Te, true);
            } });
            var X = J(U, 2);
            const H = rt(() => {
              var _a3;
              return ((_a3 = w(x)) == null ? void 0 : _a3.length_max) || 256;
            });
            Fl(X, { autocomplete: "off", get label() {
              return i.account.passwordConfirm;
            }, get placeholder() {
              return i.account.passwordConfirm;
            }, required: true, get maxLength() {
              return w(H);
            }, width: a, get value() {
              return w(k);
            }, set value(Te) {
              F(k, Te, true);
            }, get reportValidity() {
              return w($);
            }, set reportValidity(Te) {
              F($, Te, true);
            } });
            var re = J(X, 2);
            Et(re, { level: 2, onclick: h, children: (Te, ct) => {
              We();
              var Ue = Be();
              he(() => pe(Ue, i.account.generateRandom)), ee(Te, Ue);
            }, $$slots: { default: true } });
            var ue = J(re, 2);
            Zt(ue, "", {}, { "margin-top": "1rem" });
            var ye = J(ue, 2), Re = le(ye, true);
            oe(ye);
            var Ae = J(ye, 2), je = le(Ae);
            Et(je, { onclick: D, get isLoading() {
              return w(d);
            }, children: (Te, ct) => {
              We();
              var Ue = Be();
              he(() => pe(Ue, l.users.sendResetEmail)), ee(Te, Ue);
            }, $$slots: { default: true } });
            var Le = J(je, 2);
            Et(Le, { type: "submit", level: 2, get isLoading() {
              return w(d);
            }, children: (Te, ct) => {
              We();
              var Ue = Be();
              he(() => pe(Ue, l.users.savePassword)), ee(Te, Ue);
            }, $$slots: { default: true } });
            var Me = J(Le, 2);
            {
              var ze = (Te) => {
                Zo(Te, {});
              };
              be(Me, (Te) => {
                w(P) && Te(ze);
              });
            }
            oe(Ae), he(() => pe(Re, l.users.selfServiceDesc)), ee(O, V);
          }, $$slots: { default: true } }), ee(u, p);
        };
        be(L, (u) => {
          t.user.account_type === "passkey" || t.user.account_type === "federated_passkey" ? u(s) : u(o, false);
        }, r);
      }
    };
    be(m, (L) => {
      t.user.account_type === "new" ? L(_) : L(R, false);
    });
  }
  var q = J(m, 2);
  {
    var A = (L) => {
      var r = $x(), s = le(r, true);
      oe(r), he(() => pe(s, w(n))), ee(L, r);
    };
    be(q, (L) => {
      w(n) && L(A);
    });
  }
  ee(e, v), Ot();
}
var Ux = Pe("<p> </p>"), Gx = Pe('<p> </p> <p><!></p> <div class="keysContainer svelte-1teaezc"></div>', 1), Wx = Pe('<div class="err"> </div>'), zx = Pe("<!> <!>", 1);
function Kx(e, t) {
  Lt(t, true);
  let a = $t(), i = Z(""), l = Z(Ve([]));
  kt(() => {
    d();
  });
  async function d() {
    var _a3;
    let E = await jt(`/auth/v1/users/${t.user.id}/webauthn`);
    E.body ? F(l, E.body, true) : F(i, ((_a3 = E.error) == null ? void 0 : _a3.message) || "Error", true);
  }
  async function n(E) {
    var _a3;
    let $ = w(l).length === 1, D = await Cl(`/auth/v1/users/${t.user.id}/webauthn/delete/${E}`);
    D.status === 200 ? (await d(), $ && t.onSave()) : F(i, ((_a3 = D.error) == null ? void 0 : _a3.message) || "Error", true);
  }
  var P = zx(), x = De(P);
  {
    var T = (E) => {
      var $ = Ux(), D = le($, true);
      oe($), he(() => pe(D, a.users.noMfaKeys)), ee(E, $);
    }, b = (E) => {
      var $ = Gx(), D = De($), S = le(D, true);
      oe(D);
      var h = J(D, 2), v = le(h);
      _v(v, () => a.users.mfaDelete2), oe(h);
      var m = J(h, 2);
      Go(m, 21, () => w(l), (_) => _.name, (_, R) => {
        gv(_, { get passkey() {
          return w(R);
        }, showDelete: true, onDelete: n });
      }), oe(m), he(() => pe(S, a.users.mfaDelete1)), ee(E, $);
    };
    be(x, (E) => {
      w(l).length === 0 ? E(T) : E(b, false);
    });
  }
  var k = J(x, 2);
  {
    var I = (E) => {
      var $ = Wx(), D = le($, true);
      oe($), he(() => pe(D, w(i))), ee(E, $);
    };
    be(k, (E) => {
      w(i) && E(I);
    });
  }
  ee(e, P), Ot();
}
var Xx = Pe('<div class="err"> </div>'), Yx = Pe('<p> </p> <div class="flex gap-05"><!> <!></div> <!>', 1);
function Qx(e, t) {
  Lt(t, true);
  let a = $t(), i = Z(""), l = Z(false);
  async function d() {
    F(i, "");
    let D = await Cl(`/auth/v1/sessions/${t.userId}`);
    D.error ? F(i, D.error.message, true) : (F(l, true), setTimeout(() => {
      F(l, false);
    }, 3e3));
  }
  var n = Yx(), P = De(n), x = le(P, true);
  oe(P);
  var T = J(P, 2), b = le(T);
  Et(b, { level: -1, onclick: d, children: (D, S) => {
    We();
    var h = Be("Logout");
    ee(D, h);
  }, $$slots: { default: true } });
  var k = J(b, 2);
  {
    var I = (D) => {
      Zo(D, {});
    };
    be(k, (D) => {
      w(l) && D(I);
    });
  }
  oe(T);
  var E = J(T, 2);
  {
    var $ = (D) => {
      var S = Xx(), h = le(S, true);
      oe(S), he(() => pe(h, w(i))), ee(D, S);
    };
    be(E, (D) => {
      w(i) && D($);
    });
  }
  he(() => pe(x, a.users.forceLogout)), ee(e, n), Ot();
}
var Jx = Pe('<div class="err"> </div>'), Zx = Pe("<p> </p> <!> <!>", 1);
function e_(e, t) {
  Lt(t, true);
  let a = ar(), i = $t(), l = Z("");
  async function d() {
    F(l, "");
    let I = await Cl(`/auth/v1/users/${t.userId}`);
    I.error ? F(l, I.error.message, true) : t.onSave();
  }
  var n = Zx(), P = De(n), x = le(P, true);
  oe(P);
  var T = J(P, 2);
  Et(T, { level: -1, onclick: d, children: (I, E) => {
    We();
    var $ = Be();
    he(() => pe($, a.common.delete)), ee(I, $);
  }, $$slots: { default: true } });
  var b = J(T, 2);
  {
    var k = (I) => {
      var E = Jx(), $ = le(E, true);
      oe(E), he(() => pe($, w(l))), ee(I, E);
    };
    be(b, (I) => {
      w(l) && I(k);
    });
  }
  he(() => pe(x, i.users.deleteUser)), ee(e, n), Ot();
}
var t_ = Pe('<div class="err"> </div>'), r_ = Pe('<!> <div class="flex"><!></div> <!>', 1);
function a_(e, t) {
  Lt(t, true);
  let a = ar(), i = $t();
  const l = [a.account.navInfo, i.users.attributes, a.common.password, a.account.navMfa, a.account.devices, a.account.navLogout, a.common.delete];
  let d = Z(Ve(l[0])), n = Z(void 0), P = Z(""), x = Z(void 0);
  kt(() => {
    T();
  });
  async function T() {
    var _a3;
    let h = await jt(`/auth/v1/users/${t.userId}`);
    h.body ? (F(x, h.body, true), requestAnimationFrame(() => {
      var _a4;
      (_a4 = w(n)) == null ? void 0 : _a4();
    })) : F(P, ((_a3 = h.error) == null ? void 0 : _a3.message) || "Error fetching user", true);
  }
  var b = r_(), k = De(b);
  {
    var I = (h) => {
      var v = t_(), m = le(v, true);
      oe(v), he(() => pe(m, w(P))), ee(h, v);
    };
    be(k, (h) => {
      w(P) && h(I);
    });
  }
  var E = J(k, 2), $ = le(E);
  mv($, { tabs: l, get selected() {
    return w(d);
  }, set selected(h) {
    F(d, h, true);
  }, get focusFirst() {
    return w(n);
  }, set focusFirst(h) {
    F(n, h, true);
  } }), oe(E);
  var D = J(E, 2);
  {
    var S = (h) => {
      var v = Jt(), m = De(v);
      {
        var _ = (q) => {
          Nv(q, { get roles() {
            return t.roles;
          }, get groups() {
            return t.groups;
          }, get onSave() {
            return t.onSave;
          }, get user() {
            return w(x);
          }, set user(A) {
            F(x, A, true);
          } });
        }, R = (q, A) => {
          {
            var L = (s) => {
              Nx(s, { get user() {
                return w(x);
              }, get onSave() {
                return t.onSave;
              } });
            }, r = (s, o) => {
              {
                var u = (f) => {
                  Hx(f, { get user() {
                    return w(x);
                  }, get onSave() {
                    return t.onSave;
                  } });
                }, p = (f, g) => {
                  {
                    var M = (y) => {
                      Kx(y, { get user() {
                        return w(x);
                      }, get onSave() {
                        return t.onSave;
                      } });
                    }, C = (y, O) => {
                      {
                        var N = (U) => {
                          Rv(U, { get userId() {
                            return t.userId;
                          } });
                        }, V = (U, K) => {
                          {
                            var X = (re) => {
                              Qx(re, { get userId() {
                                return t.userId;
                              } });
                            }, H = (re, ue) => {
                              {
                                var ye = (Re) => {
                                  e_(Re, { get userId() {
                                    return t.userId;
                                  }, get onSave() {
                                    return t.onSave;
                                  } });
                                };
                                be(re, (Re) => {
                                  w(d) === l[6] && Re(ye);
                                }, ue);
                              }
                            };
                            be(U, (re) => {
                              w(d) === l[5] ? re(X) : re(H, false);
                            }, K);
                          }
                        };
                        be(y, (U) => {
                          w(d) === l[4] ? U(N) : U(V, false);
                        }, O);
                      }
                    };
                    be(f, (y) => {
                      w(d) === l[3] ? y(M) : y(C, false);
                    }, g);
                  }
                };
                be(s, (f) => {
                  w(d) === l[2] ? f(u) : f(p, false);
                }, o);
              }
            };
            be(q, (s) => {
              w(d) === l[1] ? s(L) : s(r, false);
            }, A);
          }
        };
        be(m, (q) => {
          w(d) === l[0] ? q(_) : q(R, false);
        });
      }
      ee(h, v);
    };
    be(D, (h) => {
      w(x) && h(S);
    });
  }
  ee(e, b), Ot();
}
var i_ = Pe('<div class="navBtn svelte-16zs9sp"><div class="picture"><!></div> <div class="tile svelte-16zs9sp"><div> </div> <div class="muted svelte-16zs9sp"> </div></div></div>'), s_ = Pe("<div></div> <!> <!>", 1), n_ = Pe("<!> <!>", 1), o_ = Pe('<div class="err"> </div>'), l_ = Pe('<!> <div id="users"><!></div>', 1), u_ = Pe("<!> <!>", 1);
function X_(e, t) {
  Lt(t, true);
  const a = (g, M = nv) => {
    const C = rt(() => x.get() === M().id);
    dv(g, { onclick: () => x.set(M().id), get selected() {
      return w(C);
    }, pictureLeft: true, children: (y, O) => {
      var N = i_(), V = le(N), U = le(V);
      const K = rt(() => L(M()));
      Ah(U, { get fallbackCharacters() {
        return w(K);
      }, get userId() {
        return M().id;
      }, get pictureId() {
        return M().picture_id;
      }, size: "small", disableUpload: true }), oe(V);
      var X = J(V, 2), H = le(X), re = le(H, true);
      oe(H);
      var ue = J(H, 2), ye = le(ue);
      oe(ue), oe(X), oe(N), he(() => {
        pe(re, M().email), pe(ye, `${M().given_name ?? ""}
                    ${M().family_name ?? ""}`);
      }), ee(y, N);
    } });
  };
  let i = Z(void 0), l = Z(""), d = Z(Ve([])), n = Z(Ve([])), P = Z(Ve([])), x = cv("uid"), T = Z(void 0), b = Z(Ve([])), k = Z(Ve([])), I = Z(false), E = Z(void 0), $ = Z(Ve(Cv)), D = Z(false), S = Ve(["E-Mail", "ID"]), h = Z(Ve(S[0])), v = Z(""), m = Ve(["E-Mail", "ID", "Created", "Last Login"]);
  xl(() => {
    R("page_size=" + w($)), q(), A();
  }), kt(() => {
    var _a3;
    F(T, (_a3 = w(d).find((g) => g.id === x.get())) == null ? void 0 : _a3.id, true);
  }), kt(() => {
    let g = w(v).toLowerCase();
    w(I) ? g.length < 3 ? w(D) && (R("page_size=" + w($)), F(D, false)) : _(g) : g ? w(h) === S[0] ? F(n, w(d).filter((M) => {
      var _a3;
      return (_a3 = M.email) == null ? void 0 : _a3.toLowerCase().includes(g);
    }), true) : w(h) === S[1] && F(n, w(d).filter((M) => M.id.toLowerCase().includes(g)), true) : F(n, w(d), true);
  });
  async function _(g) {
    F(E, void 0), F(D, true);
    let M;
    w(h) === S[0] ? M = "email" : M = "id";
    let C = await Pv({ ty: "user", idx: M, q: g });
    C.body ? F(d, C.body, true) : console.error(C.error);
  }
  async function R(g) {
    let M = "/auth/v1/users";
    g && (M += `?${g}`);
    let C = await jt(M);
    return C.error ? F(l, "Error fetching users: " + C.error.message) : C.body && (C.status === 206 ? (F(I, true), F(E, C.headers, true)) : (F(I, false), F(E, void 0)), F(d, C.body, true)), [C.status, C.headers];
  }
  async function q() {
    var _a3;
    let g = await jt("/auth/v1/roles");
    g.body ? F(k, g.body.toSorted((M, C) => M.name.localeCompare(C.name)), true) : F(l, ((_a3 = g.error) == null ? void 0 : _a3.message) || "Error", true);
  }
  async function A() {
    var _a3;
    let g = await jt("/auth/v1/groups");
    g.body ? F(b, g.body.toSorted((M, C) => M.name.localeCompare(C.name)), true) : F(l, ((_a3 = g.error) == null ? void 0 : _a3.message) || "Error", true);
  }
  function L(g) {
    let M = g.given_name[0];
    return g.family_name && (M += g.family_name[0]), M;
  }
  function r(g, M) {
    let C = M === "up";
    g === m[0] ? w(d).sort((y, O) => C ? y.email.localeCompare(O.email) : O.email.localeCompare(y.email)) : g === m[1] ? w(d).sort((y, O) => C ? y.id.localeCompare(O.id) : O.id.localeCompare(y.id)) : g === m[2] ? w(d).sort((y, O) => C ? y.created_at - O.created_at : O.created_at - y.created_at) : g === m[3] && w(d).sort((y, O) => {
      let N = y.last_login || 9999999999, V = y.last_login || 9999999999;
      return C ? N - V : V - N;
    });
  }
  async function s(g) {
    var _a3;
    (_a3 = w(i)) == null ? void 0 : _a3(), await R(), x.set(g);
  }
  function o() {
    R(), q(), A(), F(v, "");
  }
  var u = u_(), p = De(u);
  lv(p, { paddingTop: "2.1rem", buttonTilesAriaControls: "users", width: "min(23rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (M) => {
    var C = s_(), y = De(C);
    Zt(y, "", {}, { height: ".5rem" });
    var O = J(y, 2);
    {
      var N = (H) => {
        var re = Jt(), ue = De(re);
        Go(ue, 17, () => w(d), xh, (ye, Re) => {
          a(ye, () => w(Re));
        }), ee(H, re);
      }, V = (H) => {
        var re = Jt(), ue = De(re);
        Go(ue, 17, () => w(P), (ye) => ye.id, (ye, Re) => {
          a(ye, () => w(Re));
        }), ee(H, re);
      };
      be(O, (H) => {
        w(E) ? H(N) : H(V, false);
      });
    }
    var U = J(O, 2);
    {
      var K = (H) => {
        qv(H, { sspFetch: R, idxTotalCount: "x-user-count", get itemsLength() {
          return w(d).length;
        }, get firstFetchHeaders() {
          return w(E);
        }, get pageSize() {
          return w($);
        }, set pageSize(re) {
          F($, re, true);
        } });
      }, X = (H, re) => {
        {
          var ue = (Re) => {
            Dl(Re, { get items() {
              return w(d);
            }, set items(Ae) {
              F(d, Ae, true);
            }, get itemsPaginated() {
              return w(P);
            }, set itemsPaginated(Ae) {
              F(P, Ae, true);
            } });
          }, ye = (Re) => {
            Dl(Re, { get items() {
              return w(n);
            }, set items(Ae) {
              F(n, Ae, true);
            }, get itemsPaginated() {
              return w(P);
            }, set itemsPaginated(Ae) {
              F(P, Ae, true);
            } });
          };
          be(H, (Re) => {
            w(I) ? Re(ue) : Re(ye, false);
          }, re);
        }
      };
      be(U, (H) => {
        w(E) ? H(K) : H(X, false);
      });
    }
    ee(M, C);
  }, children: (M, C) => {
    var y = n_(), O = De(y);
    const N = rt(() => w(k).length === 0 ? 1 : 2);
    uv(O, { get level() {
      return w(N);
    }, alignRight: true, get closeModal() {
      return w(i);
    }, set closeModal(U) {
      F(i, U, true);
    }, children: (U, K) => {
      Ev(U, { onSave: s, get roles() {
        return w(k);
      }, get groups() {
        return w(b);
      } });
    }, $$slots: { default: true } });
    var V = J(O, 2);
    ov(V, { get searchOptions() {
      return S;
    }, get orderOptions() {
      return m;
    }, onChangeOrder: r, searchWidth: "min(23rem, calc(100dvw - .5rem))", get value() {
      return w(v);
    }, set value(U) {
      F(v, U, true);
    }, get searchOption() {
      return w(h);
    }, set searchOption(U) {
      F(h, U, true);
    } }), ee(M, y);
  }, $$slots: { buttonTiles: true, default: true } });
  var f = J(p, 2);
  pv(f, { children: (g, M) => {
    var C = l_(), y = De(C);
    {
      var O = (K) => {
        var X = o_(), H = le(X, true);
        oe(X), he(() => pe(H, w(l))), ee(K, X);
      };
      be(y, (K) => {
        w(l) && K(O);
      });
    }
    var N = J(y, 2), V = le(N);
    {
      var U = (K) => {
        a_(K, { get userId() {
          return w(T);
        }, get roles() {
          return w(k);
        }, get groups() {
          return w(b);
        }, onSave: o });
      };
      be(V, (K) => {
        w(T) && K(U);
      });
    }
    oe(N), ee(g, C);
  } }), ee(e, u), Ot();
}
export {
  X_ as U
};
