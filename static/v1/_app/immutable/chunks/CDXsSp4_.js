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
var _e2, _a2;
import { t as Me, a as ae, e as He, d as nt } from "./BH6NCLk-.js";
import { p as Mt, k as ie, O as $o, aa as Et, a as Lt, f as Le, l as $, j as I, s as se, t as _e, c as ve, a9 as bt, a7 as Ke, r as me, a5 as gl, a8 as Nl } from "./CvlvO1XB.js";
import { s as Re } from "./CTI4QPiR.js";
import { i as Pe } from "./BUO_AUgz.js";
import { e as Ho, i as ov } from "./BpWRzPRQ.js";
import { a as Xt, B as At, t as Ch, s as Ph } from "./DMkkW5Nn.js";
import { p as z } from "./Wh68IIk2.js";
import { b as qh, d as Rl, f as Dt, c as xl } from "./BO1A6s0c.js";
import { O as lv } from "./CONsTX1L.js";
import { P as Dl } from "./5W956QVQ.js";
import { N as uv } from "./DZ-K8XgE.js";
import { B as cv } from "./ClvgHNP_.js";
import { u as dv } from "./BYZsu9rB.js";
import { N as pv } from "./D-L0o8jR.js";
import { C as fv } from "./BnPoFdx3.js";
import { I as ht } from "./BZu_mjh1.js";
import { b as Uo, k as hv, l as jl, m as mv } from "./BRCxk8by.js";
import { F as _l } from "./CsaAZyUr.js";
import { u as Yt } from "./CUqQZdNU.js";
import { u as jt } from "./D8mHI_K9.js";
import { O as Th } from "./D1anwFsC.js";
import { I as jo } from "./D9bDr0Ir.js";
import { I as hl } from "./BXc4I0Oc.js";
import { f as Wt, a as Bo, b as Sh } from "./DswDW5U8.js";
import { S as Go } from "./w8oBH_pH.js";
import { L as Ft } from "./nnFaiMsH.js";
import { u as wh } from "./D-rrgBMm.js";
import { T as vv } from "./_OE2Cq0B.js";
import { p as bv } from "./C6SR4G2t.js";
import { l as nl, z as yv } from "./B21bTIl7.js";
import { C as gv } from "./CS_Msctd.js";
import { I as Yo } from "./Nks81rMs.js";
import { P as Rv } from "./DIB4eys6.js";
import { I as Bl } from "./B-xX0s4n.js";
import { h as xv } from "./i8Xqpu09.js";
import { U as _v, D as Cv } from "./3us5LYPD.js";
import { P as Pv, f as qv, a as Tv } from "./DDTc5aFx.js";
var Sv = Me("<div><!></div>"), wv = Me('<div class="err"> </div>'), Av = Me("<!> <!> <!> <!> <!> <!> <!> <!> <div></div> <!> <!>", 1), Ev = Me('<div class="container svelte-x7a3kj"><!></div>');
function kv(e, t) {
  Mt(t, true);
  let a = Yt(), s = jt(), u = ie(void 0), p = ie(""), m = ie(""), w = ie(""), x = ie(""), o = bt(() => {
    var _a3;
    return (_a3 = wh().common()) == null ? void 0 : _a3.map((g) => g);
  }), y = ie("en"), O = ie(z($o(() => t.roles.map((g) => ({ name: g.name, selected: false }))))), L = ie(z($o(() => t.groups.map((g) => ({ name: g.name, selected: false }))))), B = ie(false), G = ie(z(Wt())), j = ie(z(Bo()));
  Et(() => {
    requestAnimationFrame(() => {
      var _a3;
      (_a3 = I(u)) == null ? void 0 : _a3.focus();
    });
  });
  async function R(g, S) {
    var _a3, _b2;
    $(p, "");
    let C = { email: I(m), given_name: I(w), family_name: I(x) || void 0, language: I(y), groups: I(L).filter((k) => k.selected).map((k) => k.name), roles: I(O).filter((k) => k.selected).map((k) => k.name), user_expires: Sh(I(G), I(j)) };
    ((_a3 = C.groups) == null ? void 0 : _a3.length) === 0 && (C.groups = void 0);
    let _ = await qh(g.action, C);
    _.body ? t.onSave(_.body.id) : $(p, z(((_b2 = _.error) == null ? void 0 : _b2.message) || "Error"));
  }
  var h = Ev(), v = ve(h);
  _l(v, { action: "/auth/v1/users", onSubmit: R, children: (g, S) => {
    var C = Av(), _ = Le(C);
    ht(_, { typ: "email", autocomplete: "off", label: "E-Mail", placeholder: "E-Mail", required: true, get ref() {
      return I(u);
    }, set ref(l) {
      $(u, z(l));
    }, get value() {
      return I(m);
    }, set value(l) {
      $(m, z(l));
    } });
    var k = se(_, 2);
    ht(k, { autocomplete: "off", get label() {
      return a.account.givenName;
    }, get placeholder() {
      return a.account.givenName;
    }, required: true, pattern: Uo, get value() {
      return I(w);
    }, set value(l) {
      $(w, z(l));
    } });
    var N = se(k, 2);
    ht(N, { autocomplete: "off", get label() {
      return a.account.familyName;
    }, get placeholder() {
      return a.account.familyName;
    }, pattern: Uo, get value() {
      return I(x);
    }, set value(l) {
      $(x, z(l));
    } });
    var W = se(N, 2);
    {
      var ee = (l) => {
        Ft(l, { get label() {
          return s.common.language;
        }, children: (f, b) => {
          Th(f, { get ariaLabel() {
            return a.common.selectI18n;
          }, get options() {
            return I(o);
          }, borderless: true, get value() {
            return I(y);
          }, set value(P) {
            $(y, z(P));
          } });
        } });
      };
      Pe(W, (l) => {
        I(o) && l(ee);
      });
    }
    var A = se(W, 2);
    Go(A, { get items() {
      return I(O);
    }, set items(l) {
      $(O, z(l));
    }, children: (l, f) => {
      Ke();
      var b = He();
      _e(() => Re(b, a.account.roles)), ae(l, b);
    }, $$slots: { default: true } });
    var F = se(A, 2);
    Go(F, { get items() {
      return I(L);
    }, set items(l) {
      $(L, z(l));
    }, children: (l, f) => {
      Ke();
      var b = He();
      _e(() => Re(b, a.account.groups)), ae(l, b);
    }, $$slots: { default: true } });
    var E = se(F, 2);
    jo(E, { get ariaLabel() {
      return a.account.accessExp;
    }, get checked() {
      return I(B);
    }, set checked(l) {
      $(B, z(l));
    }, children: (l, f) => {
      Ke();
      var b = He();
      _e(() => Re(b, a.account.accessExp)), ae(l, b);
    }, $$slots: { default: true } });
    var M = se(E, 2);
    {
      var V = (l) => {
        var f = Sv(), b = ve(f);
        const P = bt(Wt);
        hl(b, { get label() {
          return a.account.accessExp;
        }, withTime: true, get min() {
          return I(P);
        }, required: true, get value() {
          return I(G);
        }, set value(q) {
          $(G, z(q));
        }, get timeValue() {
          return I(j);
        }, set timeValue(q) {
          $(j, z(q));
        } }), me(f), Ch(3, f, () => Ph, () => ({ duration: 150 })), ae(l, f);
      };
      Pe(M, (l) => {
        I(B) && l(V);
      });
    }
    var c = se(M, 2);
    Xt(c, "height", ".66rem");
    var r = se(c, 2);
    At(r, { type: "submit", children: (l, f) => {
      Ke();
      var b = He();
      _e(() => Re(b, a.common.save)), ae(l, b);
    }, $$slots: { default: true } });
    var i = se(r, 2);
    {
      var n = (l) => {
        var f = wv(), b = ve(f, true);
        me(f), _e(() => Re(b, I(p))), ae(l, f);
      };
      Pe(i, (l) => {
        I(p) && l(n);
      });
    }
    ae(g, C);
  }, $$slots: { default: true } }), me(h), ae(e, h), Lt();
}
var Iv = Me("<div><!></div>"), Mv = Me('<div class="svelte-k6dexs"><!></div>'), Lv = Me('<div class="err"> </div>'), Ov = Me('<!> <div class="values svelte-k6dexs"><div class="svelte-k6dexs"><!></div> <div class="svelte-k6dexs"><!></div></div> <div class="values svelte-k6dexs"><div class="svelte-k6dexs"><!> <!> <!> <!> <!></div> <div class="svelte-k6dexs"><!> <!> <!> <!> <!></div></div> <!> <!> <div class="values svelte-k6dexs"><div class="svelte-k6dexs"><!></div> <!></div> <div class="values svelte-k6dexs"><div class="svelte-k6dexs"><!> <!></div> <div class="svelte-k6dexs"><!> <!></div></div> <div class="flex gap-05"><!> <!></div> <!>', 1);
function Nv(e, t) {
  Mt(t, true);
  let a = bv(t, "user", 15), s = Yt(), u = jt(), p = ie(""), m = ie(false), w = ie(""), x = ie(""), o = ie(""), y = bt(() => {
    var _a3;
    return (_a3 = wh().common()) == null ? void 0 : _a3.map((E) => E);
  }), O = ie("en"), L = ie(""), B = ie(""), G = ie(""), j = ie(""), R = ie(""), h = ie(""), v = ie(false), g = ie(false), S = ie(false), C = ie(z(Wt())), _ = ie(z(Bo())), k = ie(z($o(() => t.roles.map((E) => ({ name: E.name, selected: false }))))), N = ie(z($o(() => t.groups.map((E) => ({ name: E.name, selected: false })))));
  Et(() => {
    var _a3, _b2, _c2, _d2, _e3, _f2, _g2;
    if (a()) {
      if ($(w, z(a().email)), $(x, z(a().given_name)), $(o, z(a().family_name || "")), $(O, z(a().language)), $(L, z(((_a3 = a().user_values) == null ? void 0 : _a3.birthdate) || "")), $(B, z(((_b2 = a().user_values) == null ? void 0 : _b2.phone) || "")), $(G, z(((_c2 = a().user_values) == null ? void 0 : _c2.street) || "")), $(j, z(((_e3 = (_d2 = a().user_values) == null ? void 0 : _d2.zip) == null ? void 0 : _e3.toString()) || "")), $(R, z(((_f2 = a().user_values) == null ? void 0 : _f2.city) || "")), $(h, z(((_g2 = a().user_values) == null ? void 0 : _g2.country) || "")), $(v, z(a().enabled)), $(g, z(a().email_verified)), a().user_expires) {
        let E = new Date(a().user_expires * 1e3);
        $(S, true), $(C, z(Wt(E))), $(_, z(Bo(E)));
      } else $(S, false), $(C, z(Wt())), $(_, z(Bo()));
      $(k, z(t.roles.map((E) => {
        var _a4;
        return { name: E.name, selected: ((_a4 = a()) == null ? void 0 : _a4.roles.includes(E.name)) || false };
      }))), $(N, z(t.roles.map((E) => {
        var _a4, _b3;
        return { name: E.name, selected: ((_b3 = (_a4 = a()) == null ? void 0 : _a4.groups) == null ? void 0 : _b3.includes(E.name)) || false };
      })));
    }
  });
  async function W(E, M) {
    var _a3, _b2;
    $(p, "");
    const V = { email: I(w), given_name: I(x), family_name: I(o) || void 0, language: I(O), roles: I(k).filter((r) => r.selected).map((r) => r.name), groups: I(N).filter((r) => r.selected).map((r) => r.name), enabled: I(v), email_verified: I(g), user_expires: I(S) ? Sh(I(C), I(_)) : void 0 };
    (I(L) || I(B) || I(G) || I(j) || I(R) || I(h)) && (V.user_values = { birthdate: I(L) || void 0, phone: ((_a3 = I(B)) == null ? void 0 : _a3.replaceAll(" ", "")) || void 0, street: I(G) || void 0, zip: I(j) ? Number.parseInt(I(j)) : void 0, city: I(R) || void 0, country: I(h) || void 0 });
    let c = await Rl(E.action, V);
    c.body ? ($(m, true), a(c.body), setTimeout(() => {
      $(m, false);
    }, 3e3)) : $(p, z(((_b2 = c.error) == null ? void 0 : _b2.message) || "Error"));
  }
  var ee = nt(), A = Le(ee);
  {
    var F = (E) => {
      const M = bt(() => `/auth/v1/users/${a().id}`);
      _l(E, { get action() {
        return I(M);
      }, onSubmit: W, children: (V, c) => {
        var r = Ov(), i = Le(r);
        Ft(i, { label: "ID", mono: true, children: (te, Se) => {
          Ke();
          var be = He();
          _e(() => Re(be, a().id)), ae(te, be);
        } });
        var n = se(i, 2), l = ve(n), f = ve(l);
        jo(f, { get ariaLabel() {
          return u.common.enabled;
        }, get checked() {
          return I(v);
        }, set checked(te) {
          $(v, z(te));
        }, children: (te, Se) => {
          Ke();
          var be = He();
          _e(() => Re(be, u.common.enabled)), ae(te, be);
        }, $$slots: { default: true } }), me(l);
        var b = se(l, 2), P = ve(b);
        jo(P, { get ariaLabel() {
          return s.account.emailVerified;
        }, get checked() {
          return I(g);
        }, set checked(te) {
          $(g, z(te));
        }, children: (te, Se) => {
          Ke();
          var be = He();
          _e(() => Re(be, s.account.emailVerified)), ae(te, be);
        }, $$slots: { default: true } }), me(b), me(n);
        var q = se(n, 2), T = ve(q), H = ve(T);
        ht(H, { typ: "email", autocomplete: "off", label: "E-Mail", placeholder: "E-Mail", required: true, get value() {
          return I(w);
        }, set value(te) {
          $(w, z(te));
        } });
        var J = se(H, 2);
        ht(J, { autocomplete: "off", get label() {
          return s.account.givenName;
        }, get placeholder() {
          return s.account.givenName;
        }, required: true, pattern: Uo, get value() {
          return I(x);
        }, set value(te) {
          $(x, z(te));
        } });
        var re = se(J, 2);
        ht(re, { autocomplete: "off", get label() {
          return s.account.familyName;
        }, get placeholder() {
          return s.account.familyName;
        }, pattern: Uo, get value() {
          return I(o);
        }, set value(te) {
          $(o, z(te));
        } });
        var Q = se(re, 2);
        hl(Q, { get label() {
          return s.account.birthdate;
        }, withDelete: true, get value() {
          return I(L);
        }, set value(te) {
          $(L, z(te));
        } });
        var de = se(Q, 2);
        {
          var pe = (te) => {
            var Se = Iv();
            Xt(Se, "padding", ".25rem");
            var be = ve(Se);
            Ft(be, { get label() {
              return u.common.language;
            }, children: (Qe, Fe) => {
              Th(Qe, { get ariaLabel() {
                return s.common.selectI18n;
              }, get options() {
                return I(y);
              }, borderless: true, get value() {
                return I(O);
              }, set value(ot) {
                $(O, z(ot));
              } });
            } }), me(Se), ae(te, Se);
          };
          Pe(de, (te) => {
            I(y) && te(pe);
          });
        }
        me(T);
        var fe = se(T, 2), he = ve(fe);
        ht(he, { autocomplete: "off", get label() {
          return s.account.street;
        }, get placeholder() {
          return s.account.street;
        }, pattern: hv, get value() {
          return I(G);
        }, set value(te) {
          $(G, z(te));
        } });
        var Ee = se(he, 2);
        ht(Ee, { typ: "number", autocomplete: "off", get label() {
          return s.account.zip;
        }, get placeholder() {
          return s.account.zip;
        }, min: "1000", max: "9999999", get value() {
          return I(j);
        }, set value(te) {
          $(j, z(te));
        } });
        var Oe = se(Ee, 2);
        ht(Oe, { autocomplete: "off", get label() {
          return s.account.city;
        }, get placeholder() {
          return s.account.city;
        }, pattern: jl, get value() {
          return I(R);
        }, set value(te) {
          $(R, z(te));
        } });
        var ke = se(Oe, 2);
        ht(ke, { autocomplete: "off", get label() {
          return s.account.country;
        }, get placeholder() {
          return s.account.country;
        }, pattern: jl, get value() {
          return I(h);
        }, set value(te) {
          $(h, z(te));
        } });
        var We = se(ke, 2);
        ht(We, { autocomplete: "off", get label() {
          return s.account.phone;
        }, get placeholder() {
          return s.account.phone;
        }, pattern: mv, get value() {
          return I(B);
        }, set value(te) {
          $(B, z(te));
        } }), me(fe), me(q);
        var Ne = se(q, 2);
        Go(Ne, { get items() {
          return I(k);
        }, set items(te) {
          $(k, z(te));
        }, children: (te, Se) => {
          Ke();
          var be = He();
          _e(() => Re(be, s.account.roles)), ae(te, be);
        }, $$slots: { default: true } });
        var tt = se(Ne, 2);
        Go(tt, { get items() {
          return I(N);
        }, set items(te) {
          $(N, z(te));
        }, children: (te, Se) => {
          Ke();
          var be = He();
          _e(() => Re(be, s.account.groups)), ae(te, be);
        }, $$slots: { default: true } });
        var Je = se(tt, 2), it = ve(Je);
        Xt(it, "margin-top", ".5rem");
        var gt = ve(it);
        jo(gt, { get ariaLabel() {
          return s.account.accessExp;
        }, get checked() {
          return I(S);
        }, set checked(te) {
          $(S, z(te));
        }, children: (te, Se) => {
          Ke();
          var be = He();
          _e(() => Re(be, s.account.accessExp)), ae(te, be);
        }, $$slots: { default: true } }), me(it);
        var D = se(it, 2);
        {
          var X = (te) => {
            var Se = Mv(), be = ve(Se);
            const Qe = bt(Wt);
            hl(be, { get label() {
              return s.account.accessExp;
            }, withTime: true, get min() {
              return I(Qe);
            }, required: true, get value() {
              return I(C);
            }, set value(Fe) {
              $(C, z(Fe));
            }, get timeValue() {
              return I(_);
            }, set timeValue(Fe) {
              $(_, z(Fe));
            } }), me(Se), Ch(3, Se, () => Ph, () => ({ duration: 150 })), ae(te, Se);
          };
          Pe(D, (te) => {
            I(S) && te(X);
          });
        }
        me(Je);
        var K = se(Je, 2), Y = ve(K), ce = ve(Y);
        Ft(ce, { get label() {
          return s.account.userCreated;
        }, children: (te, Se) => {
          Ke();
          var be = He();
          _e((Qe) => Re(be, Qe), [() => nl(a().created_at)]), ae(te, be);
        } });
        var ne = se(ce, 2);
        Ft(ne, { get label() {
          return u.users.lastLogin;
        }, children: (te, Se) => {
          var be = nt(), Qe = Le(be);
          {
            var Fe = (Xe) => {
              var Ye = He();
              _e((Ct) => Re(Ye, Ct), [() => nl(a().last_login)]), ae(Xe, Ye);
            }, ot = (Xe) => {
              var Ye = He();
              _e(() => Re(Ye, s.common.never)), ae(Xe, Ye);
            };
            Pe(Qe, (Xe) => {
              a().last_login ? Xe(Fe) : Xe(ot, false);
            });
          }
          ae(te, be);
        } }), me(Y);
        var ge = se(Y, 2), Ie = ve(ge);
        Ft(Ie, { get label() {
          return s.account.passwordExpiry;
        }, children: (te, Se) => {
          var be = nt(), Qe = Le(be);
          {
            var Fe = (Xe) => {
              var Ye = He();
              _e((Ct) => Re(Ye, Ct), [() => nl(a().password_expires)]), ae(Xe, Ye);
            }, ot = (Xe) => {
              var Ye = He();
              _e(() => Re(Ye, s.common.never)), ae(Xe, Ye);
            };
            Pe(Qe, (Xe) => {
              a().password_expires ? Xe(Fe) : Xe(ot, false);
            });
          }
          ae(te, be);
        } });
        var xe = se(Ie, 2);
        Ft(xe, { get label() {
          return s.account.mfaActivated;
        }, children: (te, Se) => {
          gv(te, {});
        } }), me(ge), me(K);
        var Te = se(K, 2), Ge = ve(Te);
        At(Ge, { type: "submit", children: (te, Se) => {
          Ke();
          var be = He();
          _e(() => Re(be, s.common.save)), ae(te, be);
        }, $$slots: { default: true } });
        var je = se(Ge, 2);
        {
          var De = (te) => {
            Yo(te, {});
          };
          Pe(je, (te) => {
            I(m) && te(De);
          });
        }
        me(Te);
        var ze = se(Te, 2);
        {
          var Ze = (te) => {
            var Se = Lv(), be = ve(Se, true);
            me(Se), _e(() => Re(be, I(p))), ae(te, Se);
          };
          Pe(ze, (te) => {
            I(p) && te(Ze);
          });
        }
        ae(V, r);
      }, $$slots: { default: true } });
    };
    Pe(A, (E) => {
      a() && E(F);
    });
  }
  ae(e, ee), Lt();
}
function Dv(e, t) {
  return e.start <= t && t < e.end;
}
function jv(e, t = {}) {
  const { offsetLine: a = 0, offsetColumn: s = 0 } = t;
  let u = 0;
  const p = e.split(`
`).map((x, o) => {
    const y = u + x.length + 1, O = { start: u, end: y, line: o };
    return u = y, O;
  });
  let m = 0;
  function w(x, o) {
    if (typeof x == "string" && (x = e.indexOf(x, o ?? 0)), x === -1) return;
    let y = p[m];
    const O = x >= y.end ? 1 : -1;
    for (; y; ) {
      if (Dv(y, x)) return { line: a + y.line, column: s + x - y.start, character: x };
      m += O, y = p[m];
    }
  }
  return w;
}
let Bv, Fl = jv("", { offsetLine: 1 }), Fv, Vl = [], Vv = /* @__PURE__ */ new Map();
const $v = /^\t+/;
function ol(e) {
  return e.replace($v, (t) => t.split("	").join("  "));
}
function Hv(e, t, a) {
  const s = e.split(`
`), u = Math.max(0, t - 2), p = Math.min(t + 3, s.length), m = String(p + 1).length;
  return s.slice(u, p).map((w, x) => {
    const o = u + x === t, y = String(x + u + 1).padStart(m, " ");
    if (o) {
      const O = " ".repeat(m + 2 + ol(w.slice(0, a)).length) + "^";
      return `${y}: ${ol(w)}
${O}`;
    }
    return `${y}: ${ol(w)}`;
  }).join(`
`);
}
class Ah {
  constructor(t, a, s) {
    __publicField(this, "name", "CompileDiagnostic");
    this.code = t, this.message = a, s && (this.position = s, this.start = Fl(s[0]), this.end = Fl(s[1]), this.start && this.end && (this.frame = Hv(Bv, this.start.line - 1, this.end.column)));
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
class Uv extends Ah {
  constructor(t, a, s) {
    super(t, a, s);
    __publicField(this, "name", "CompileWarning");
  }
}
function Zt(e, t, a) {
  var _a3;
  let s = Vl;
  if (e && (s = Vv.get(e) ?? Vl), s && ((_a3 = s.at(-1)) == null ? void 0 : _a3.has(t))) return;
  const u = new Uv(t, a, e && e.start !== void 0 ? [e.start, e.end ?? e.start] : void 0);
  Fv(u);
}
function Gv(e) {
  Zt(e, "options_deprecated_accessors", "The `accessors` option has been deprecated. It will have no effect in runes mode\nhttps://svelte.dev/e/options_deprecated_accessors");
}
function zv(e) {
  Zt(e, "options_deprecated_immutable", "The `immutable` option has been deprecated. It will have no effect in runes mode\nhttps://svelte.dev/e/options_deprecated_immutable");
}
function Wv(e) {
  Zt(e, "options_removed_enable_sourcemap", "The `enableSourcemap` option has been removed. Source maps are always generated now, and tooling can choose to ignore them\nhttps://svelte.dev/e/options_removed_enable_sourcemap");
}
function Xv(e) {
  Zt(e, "options_removed_hydratable", "The `hydratable` option has been removed. Svelte components are always hydratable now\nhttps://svelte.dev/e/options_removed_hydratable");
}
function Kv(e) {
  Zt(e, "options_removed_loop_guard_timeout", "The `loopGuardTimeout` option has been removed\nhttps://svelte.dev/e/options_removed_loop_guard_timeout");
}
function Jv(e) {
  Zt(e, "options_renamed_ssr_dom", '`generate: "dom"` and `generate: "ssr"` options have been renamed to "client" and "server" respectively\nhttps://svelte.dev/e/options_renamed_ssr_dom');
}
var Qv = [509, 0, 227, 0, 150, 4, 294, 9, 1368, 2, 2, 1, 6, 3, 41, 2, 5, 0, 166, 1, 574, 3, 9, 9, 7, 9, 32, 4, 318, 1, 80, 3, 71, 10, 50, 3, 123, 2, 54, 14, 32, 10, 3, 1, 11, 3, 46, 10, 8, 0, 46, 9, 7, 2, 37, 13, 2, 9, 6, 1, 45, 0, 13, 2, 49, 13, 9, 3, 2, 11, 83, 11, 7, 0, 3, 0, 158, 11, 6, 9, 7, 3, 56, 1, 2, 6, 3, 1, 3, 2, 10, 0, 11, 1, 3, 6, 4, 4, 68, 8, 2, 0, 3, 0, 2, 3, 2, 4, 2, 0, 15, 1, 83, 17, 10, 9, 5, 0, 82, 19, 13, 9, 214, 6, 3, 8, 28, 1, 83, 16, 16, 9, 82, 12, 9, 9, 7, 19, 58, 14, 5, 9, 243, 14, 166, 9, 71, 5, 2, 1, 3, 3, 2, 0, 2, 1, 13, 9, 120, 6, 3, 6, 4, 0, 29, 9, 41, 6, 2, 3, 9, 0, 10, 10, 47, 15, 343, 9, 54, 7, 2, 7, 17, 9, 57, 21, 2, 13, 123, 5, 4, 0, 2, 1, 2, 6, 2, 0, 9, 9, 49, 4, 2, 1, 2, 4, 9, 9, 330, 3, 10, 1, 2, 0, 49, 6, 4, 4, 14, 10, 5350, 0, 7, 14, 11465, 27, 2343, 9, 87, 9, 39, 4, 60, 6, 26, 9, 535, 9, 470, 0, 2, 54, 8, 3, 82, 0, 12, 1, 19628, 1, 4178, 9, 519, 45, 3, 22, 543, 4, 4, 5, 9, 7, 3, 6, 31, 3, 149, 2, 1418, 49, 513, 54, 5, 49, 9, 0, 15, 0, 23, 4, 2, 14, 1361, 6, 2, 16, 3, 6, 2, 1, 2, 4, 101, 0, 161, 6, 10, 9, 357, 0, 62, 13, 499, 13, 245, 1, 2, 9, 726, 6, 110, 6, 6, 9, 4759, 9, 787719, 239], Eh = [0, 11, 2, 25, 2, 18, 2, 1, 2, 14, 3, 13, 35, 122, 70, 52, 268, 28, 4, 48, 48, 31, 14, 29, 6, 37, 11, 29, 3, 35, 5, 7, 2, 4, 43, 157, 19, 35, 5, 35, 5, 39, 9, 51, 13, 10, 2, 14, 2, 6, 2, 1, 2, 10, 2, 14, 2, 6, 2, 1, 4, 51, 13, 310, 10, 21, 11, 7, 25, 5, 2, 41, 2, 8, 70, 5, 3, 0, 2, 43, 2, 1, 4, 0, 3, 22, 11, 22, 10, 30, 66, 18, 2, 1, 11, 21, 11, 25, 71, 55, 7, 1, 65, 0, 16, 3, 2, 2, 2, 28, 43, 28, 4, 28, 36, 7, 2, 27, 28, 53, 11, 21, 11, 18, 14, 17, 111, 72, 56, 50, 14, 50, 14, 35, 39, 27, 10, 22, 251, 41, 7, 1, 17, 2, 60, 28, 11, 0, 9, 21, 43, 17, 47, 20, 28, 22, 13, 52, 58, 1, 3, 0, 14, 44, 33, 24, 27, 35, 30, 0, 3, 0, 9, 34, 4, 0, 13, 47, 15, 3, 22, 0, 2, 0, 36, 17, 2, 24, 20, 1, 64, 6, 2, 0, 2, 3, 2, 14, 2, 9, 8, 46, 39, 7, 3, 1, 3, 21, 2, 6, 2, 1, 2, 4, 4, 0, 19, 0, 13, 4, 31, 9, 2, 0, 3, 0, 2, 37, 2, 0, 26, 0, 2, 0, 45, 52, 19, 3, 21, 2, 31, 47, 21, 1, 2, 0, 185, 46, 42, 3, 37, 47, 21, 0, 60, 42, 14, 0, 72, 26, 38, 6, 186, 43, 117, 63, 32, 7, 3, 0, 3, 7, 2, 1, 2, 23, 16, 0, 2, 0, 95, 7, 3, 38, 17, 0, 2, 0, 29, 0, 11, 39, 8, 0, 22, 0, 12, 45, 20, 0, 19, 72, 200, 32, 32, 8, 2, 36, 18, 0, 50, 29, 113, 6, 2, 1, 2, 37, 22, 0, 26, 5, 2, 1, 2, 31, 15, 0, 328, 18, 16, 0, 2, 12, 2, 33, 125, 0, 80, 921, 103, 110, 18, 195, 2637, 96, 16, 1071, 18, 5, 26, 3994, 6, 582, 6842, 29, 1763, 568, 8, 30, 18, 78, 18, 29, 19, 47, 17, 3, 32, 20, 6, 18, 433, 44, 212, 63, 129, 74, 6, 0, 67, 12, 65, 1, 2, 0, 29, 6135, 9, 1237, 42, 9, 8936, 3, 2, 6, 2, 1, 2, 290, 16, 0, 30, 2, 3, 0, 15, 3, 9, 395, 2309, 106, 6, 12, 4, 8, 8, 9, 5991, 84, 2, 70, 2, 1, 3, 0, 3, 1, 3, 3, 2, 11, 2, 0, 2, 6, 2, 64, 2, 3, 3, 7, 2, 6, 2, 27, 2, 3, 2, 4, 2, 0, 4, 6, 2, 339, 3, 24, 2, 24, 2, 30, 2, 24, 2, 30, 2, 24, 2, 30, 2, 24, 2, 30, 2, 24, 2, 7, 1845, 30, 7, 5, 262, 61, 147, 44, 11, 6, 17, 0, 322, 29, 19, 43, 485, 27, 229, 29, 3, 0, 496, 6, 2, 3, 2, 1, 2, 14, 2, 196, 60, 67, 8, 0, 1205, 3, 2, 26, 2, 1, 2, 0, 3, 0, 2, 9, 2, 3, 2, 0, 2, 0, 7, 0, 5, 0, 2, 0, 2, 0, 2, 2, 2, 1, 2, 0, 3, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 1, 2, 0, 3, 3, 2, 6, 2, 3, 2, 3, 2, 0, 2, 9, 2, 16, 6, 2, 2, 4, 2, 16, 4421, 42719, 33, 4153, 7, 221, 3, 5761, 15, 7472, 16, 621, 2467, 541, 1507, 4938, 6, 4191], Yv = "\u200C\u200D\xB7\u0300-\u036F\u0387\u0483-\u0487\u0591-\u05BD\u05BF\u05C1\u05C2\u05C4\u05C5\u05C7\u0610-\u061A\u064B-\u0669\u0670\u06D6-\u06DC\u06DF-\u06E4\u06E7\u06E8\u06EA-\u06ED\u06F0-\u06F9\u0711\u0730-\u074A\u07A6-\u07B0\u07C0-\u07C9\u07EB-\u07F3\u07FD\u0816-\u0819\u081B-\u0823\u0825-\u0827\u0829-\u082D\u0859-\u085B\u0897-\u089F\u08CA-\u08E1\u08E3-\u0903\u093A-\u093C\u093E-\u094F\u0951-\u0957\u0962\u0963\u0966-\u096F\u0981-\u0983\u09BC\u09BE-\u09C4\u09C7\u09C8\u09CB-\u09CD\u09D7\u09E2\u09E3\u09E6-\u09EF\u09FE\u0A01-\u0A03\u0A3C\u0A3E-\u0A42\u0A47\u0A48\u0A4B-\u0A4D\u0A51\u0A66-\u0A71\u0A75\u0A81-\u0A83\u0ABC\u0ABE-\u0AC5\u0AC7-\u0AC9\u0ACB-\u0ACD\u0AE2\u0AE3\u0AE6-\u0AEF\u0AFA-\u0AFF\u0B01-\u0B03\u0B3C\u0B3E-\u0B44\u0B47\u0B48\u0B4B-\u0B4D\u0B55-\u0B57\u0B62\u0B63\u0B66-\u0B6F\u0B82\u0BBE-\u0BC2\u0BC6-\u0BC8\u0BCA-\u0BCD\u0BD7\u0BE6-\u0BEF\u0C00-\u0C04\u0C3C\u0C3E-\u0C44\u0C46-\u0C48\u0C4A-\u0C4D\u0C55\u0C56\u0C62\u0C63\u0C66-\u0C6F\u0C81-\u0C83\u0CBC\u0CBE-\u0CC4\u0CC6-\u0CC8\u0CCA-\u0CCD\u0CD5\u0CD6\u0CE2\u0CE3\u0CE6-\u0CEF\u0CF3\u0D00-\u0D03\u0D3B\u0D3C\u0D3E-\u0D44\u0D46-\u0D48\u0D4A-\u0D4D\u0D57\u0D62\u0D63\u0D66-\u0D6F\u0D81-\u0D83\u0DCA\u0DCF-\u0DD4\u0DD6\u0DD8-\u0DDF\u0DE6-\u0DEF\u0DF2\u0DF3\u0E31\u0E34-\u0E3A\u0E47-\u0E4E\u0E50-\u0E59\u0EB1\u0EB4-\u0EBC\u0EC8-\u0ECE\u0ED0-\u0ED9\u0F18\u0F19\u0F20-\u0F29\u0F35\u0F37\u0F39\u0F3E\u0F3F\u0F71-\u0F84\u0F86\u0F87\u0F8D-\u0F97\u0F99-\u0FBC\u0FC6\u102B-\u103E\u1040-\u1049\u1056-\u1059\u105E-\u1060\u1062-\u1064\u1067-\u106D\u1071-\u1074\u1082-\u108D\u108F-\u109D\u135D-\u135F\u1369-\u1371\u1712-\u1715\u1732-\u1734\u1752\u1753\u1772\u1773\u17B4-\u17D3\u17DD\u17E0-\u17E9\u180B-\u180D\u180F-\u1819\u18A9\u1920-\u192B\u1930-\u193B\u1946-\u194F\u19D0-\u19DA\u1A17-\u1A1B\u1A55-\u1A5E\u1A60-\u1A7C\u1A7F-\u1A89\u1A90-\u1A99\u1AB0-\u1ABD\u1ABF-\u1ACE\u1B00-\u1B04\u1B34-\u1B44\u1B50-\u1B59\u1B6B-\u1B73\u1B80-\u1B82\u1BA1-\u1BAD\u1BB0-\u1BB9\u1BE6-\u1BF3\u1C24-\u1C37\u1C40-\u1C49\u1C50-\u1C59\u1CD0-\u1CD2\u1CD4-\u1CE8\u1CED\u1CF4\u1CF7-\u1CF9\u1DC0-\u1DFF\u200C\u200D\u203F\u2040\u2054\u20D0-\u20DC\u20E1\u20E5-\u20F0\u2CEF-\u2CF1\u2D7F\u2DE0-\u2DFF\u302A-\u302F\u3099\u309A\u30FB\uA620-\uA629\uA66F\uA674-\uA67D\uA69E\uA69F\uA6F0\uA6F1\uA802\uA806\uA80B\uA823-\uA827\uA82C\uA880\uA881\uA8B4-\uA8C5\uA8D0-\uA8D9\uA8E0-\uA8F1\uA8FF-\uA909\uA926-\uA92D\uA947-\uA953\uA980-\uA983\uA9B3-\uA9C0\uA9D0-\uA9D9\uA9E5\uA9F0-\uA9F9\uAA29-\uAA36\uAA43\uAA4C\uAA4D\uAA50-\uAA59\uAA7B-\uAA7D\uAAB0\uAAB2-\uAAB4\uAAB7\uAAB8\uAABE\uAABF\uAAC1\uAAEB-\uAAEF\uAAF5\uAAF6\uABE3-\uABEA\uABEC\uABED\uABF0-\uABF9\uFB1E\uFE00-\uFE0F\uFE20-\uFE2F\uFE33\uFE34\uFE4D-\uFE4F\uFF10-\uFF19\uFF3F\uFF65", kh = "\xAA\xB5\xBA\xC0-\xD6\xD8-\xF6\xF8-\u02C1\u02C6-\u02D1\u02E0-\u02E4\u02EC\u02EE\u0370-\u0374\u0376\u0377\u037A-\u037D\u037F\u0386\u0388-\u038A\u038C\u038E-\u03A1\u03A3-\u03F5\u03F7-\u0481\u048A-\u052F\u0531-\u0556\u0559\u0560-\u0588\u05D0-\u05EA\u05EF-\u05F2\u0620-\u064A\u066E\u066F\u0671-\u06D3\u06D5\u06E5\u06E6\u06EE\u06EF\u06FA-\u06FC\u06FF\u0710\u0712-\u072F\u074D-\u07A5\u07B1\u07CA-\u07EA\u07F4\u07F5\u07FA\u0800-\u0815\u081A\u0824\u0828\u0840-\u0858\u0860-\u086A\u0870-\u0887\u0889-\u088E\u08A0-\u08C9\u0904-\u0939\u093D\u0950\u0958-\u0961\u0971-\u0980\u0985-\u098C\u098F\u0990\u0993-\u09A8\u09AA-\u09B0\u09B2\u09B6-\u09B9\u09BD\u09CE\u09DC\u09DD\u09DF-\u09E1\u09F0\u09F1\u09FC\u0A05-\u0A0A\u0A0F\u0A10\u0A13-\u0A28\u0A2A-\u0A30\u0A32\u0A33\u0A35\u0A36\u0A38\u0A39\u0A59-\u0A5C\u0A5E\u0A72-\u0A74\u0A85-\u0A8D\u0A8F-\u0A91\u0A93-\u0AA8\u0AAA-\u0AB0\u0AB2\u0AB3\u0AB5-\u0AB9\u0ABD\u0AD0\u0AE0\u0AE1\u0AF9\u0B05-\u0B0C\u0B0F\u0B10\u0B13-\u0B28\u0B2A-\u0B30\u0B32\u0B33\u0B35-\u0B39\u0B3D\u0B5C\u0B5D\u0B5F-\u0B61\u0B71\u0B83\u0B85-\u0B8A\u0B8E-\u0B90\u0B92-\u0B95\u0B99\u0B9A\u0B9C\u0B9E\u0B9F\u0BA3\u0BA4\u0BA8-\u0BAA\u0BAE-\u0BB9\u0BD0\u0C05-\u0C0C\u0C0E-\u0C10\u0C12-\u0C28\u0C2A-\u0C39\u0C3D\u0C58-\u0C5A\u0C5D\u0C60\u0C61\u0C80\u0C85-\u0C8C\u0C8E-\u0C90\u0C92-\u0CA8\u0CAA-\u0CB3\u0CB5-\u0CB9\u0CBD\u0CDD\u0CDE\u0CE0\u0CE1\u0CF1\u0CF2\u0D04-\u0D0C\u0D0E-\u0D10\u0D12-\u0D3A\u0D3D\u0D4E\u0D54-\u0D56\u0D5F-\u0D61\u0D7A-\u0D7F\u0D85-\u0D96\u0D9A-\u0DB1\u0DB3-\u0DBB\u0DBD\u0DC0-\u0DC6\u0E01-\u0E30\u0E32\u0E33\u0E40-\u0E46\u0E81\u0E82\u0E84\u0E86-\u0E8A\u0E8C-\u0EA3\u0EA5\u0EA7-\u0EB0\u0EB2\u0EB3\u0EBD\u0EC0-\u0EC4\u0EC6\u0EDC-\u0EDF\u0F00\u0F40-\u0F47\u0F49-\u0F6C\u0F88-\u0F8C\u1000-\u102A\u103F\u1050-\u1055\u105A-\u105D\u1061\u1065\u1066\u106E-\u1070\u1075-\u1081\u108E\u10A0-\u10C5\u10C7\u10CD\u10D0-\u10FA\u10FC-\u1248\u124A-\u124D\u1250-\u1256\u1258\u125A-\u125D\u1260-\u1288\u128A-\u128D\u1290-\u12B0\u12B2-\u12B5\u12B8-\u12BE\u12C0\u12C2-\u12C5\u12C8-\u12D6\u12D8-\u1310\u1312-\u1315\u1318-\u135A\u1380-\u138F\u13A0-\u13F5\u13F8-\u13FD\u1401-\u166C\u166F-\u167F\u1681-\u169A\u16A0-\u16EA\u16EE-\u16F8\u1700-\u1711\u171F-\u1731\u1740-\u1751\u1760-\u176C\u176E-\u1770\u1780-\u17B3\u17D7\u17DC\u1820-\u1878\u1880-\u18A8\u18AA\u18B0-\u18F5\u1900-\u191E\u1950-\u196D\u1970-\u1974\u1980-\u19AB\u19B0-\u19C9\u1A00-\u1A16\u1A20-\u1A54\u1AA7\u1B05-\u1B33\u1B45-\u1B4C\u1B83-\u1BA0\u1BAE\u1BAF\u1BBA-\u1BE5\u1C00-\u1C23\u1C4D-\u1C4F\u1C5A-\u1C7D\u1C80-\u1C8A\u1C90-\u1CBA\u1CBD-\u1CBF\u1CE9-\u1CEC\u1CEE-\u1CF3\u1CF5\u1CF6\u1CFA\u1D00-\u1DBF\u1E00-\u1F15\u1F18-\u1F1D\u1F20-\u1F45\u1F48-\u1F4D\u1F50-\u1F57\u1F59\u1F5B\u1F5D\u1F5F-\u1F7D\u1F80-\u1FB4\u1FB6-\u1FBC\u1FBE\u1FC2-\u1FC4\u1FC6-\u1FCC\u1FD0-\u1FD3\u1FD6-\u1FDB\u1FE0-\u1FEC\u1FF2-\u1FF4\u1FF6-\u1FFC\u2071\u207F\u2090-\u209C\u2102\u2107\u210A-\u2113\u2115\u2118-\u211D\u2124\u2126\u2128\u212A-\u2139\u213C-\u213F\u2145-\u2149\u214E\u2160-\u2188\u2C00-\u2CE4\u2CEB-\u2CEE\u2CF2\u2CF3\u2D00-\u2D25\u2D27\u2D2D\u2D30-\u2D67\u2D6F\u2D80-\u2D96\u2DA0-\u2DA6\u2DA8-\u2DAE\u2DB0-\u2DB6\u2DB8-\u2DBE\u2DC0-\u2DC6\u2DC8-\u2DCE\u2DD0-\u2DD6\u2DD8-\u2DDE\u3005-\u3007\u3021-\u3029\u3031-\u3035\u3038-\u303C\u3041-\u3096\u309B-\u309F\u30A1-\u30FA\u30FC-\u30FF\u3105-\u312F\u3131-\u318E\u31A0-\u31BF\u31F0-\u31FF\u3400-\u4DBF\u4E00-\uA48C\uA4D0-\uA4FD\uA500-\uA60C\uA610-\uA61F\uA62A\uA62B\uA640-\uA66E\uA67F-\uA69D\uA6A0-\uA6EF\uA717-\uA71F\uA722-\uA788\uA78B-\uA7CD\uA7D0\uA7D1\uA7D3\uA7D5-\uA7DC\uA7F2-\uA801\uA803-\uA805\uA807-\uA80A\uA80C-\uA822\uA840-\uA873\uA882-\uA8B3\uA8F2-\uA8F7\uA8FB\uA8FD\uA8FE\uA90A-\uA925\uA930-\uA946\uA960-\uA97C\uA984-\uA9B2\uA9CF\uA9E0-\uA9E4\uA9E6-\uA9EF\uA9FA-\uA9FE\uAA00-\uAA28\uAA40-\uAA42\uAA44-\uAA4B\uAA60-\uAA76\uAA7A\uAA7E-\uAAAF\uAAB1\uAAB5\uAAB6\uAAB9-\uAABD\uAAC0\uAAC2\uAADB-\uAADD\uAAE0-\uAAEA\uAAF2-\uAAF4\uAB01-\uAB06\uAB09-\uAB0E\uAB11-\uAB16\uAB20-\uAB26\uAB28-\uAB2E\uAB30-\uAB5A\uAB5C-\uAB69\uAB70-\uABE2\uAC00-\uD7A3\uD7B0-\uD7C6\uD7CB-\uD7FB\uF900-\uFA6D\uFA70-\uFAD9\uFB00-\uFB06\uFB13-\uFB17\uFB1D\uFB1F-\uFB28\uFB2A-\uFB36\uFB38-\uFB3C\uFB3E\uFB40\uFB41\uFB43\uFB44\uFB46-\uFBB1\uFBD3-\uFD3D\uFD50-\uFD8F\uFD92-\uFDC7\uFDF0-\uFDFB\uFE70-\uFE74\uFE76-\uFEFC\uFF21-\uFF3A\uFF41-\uFF5A\uFF66-\uFFBE\uFFC2-\uFFC7\uFFCA-\uFFCF\uFFD2-\uFFD7\uFFDA-\uFFDC", ll = { 3: "abstract boolean byte char class double enum export extends final float goto implements import int interface long native package private protected public short static super synchronized throws transient volatile", 5: "class enum extends super const export import", 6: "enum", strict: "implements interface let package private protected public static yield", strictBind: "eval arguments" }, ul = "break case catch continue debugger default do else finally for function if return switch throw try var while with null true false instanceof typeof void delete new in this", Zv = { 5: ul, "5module": ul + " export import", 6: ul + " const class extends export import super" }, e1 = /^in(stanceof)?$/, t1 = new RegExp("[" + kh + "]"), r1 = new RegExp("[" + kh + Yv + "]");
function ml(e, t) {
  for (var a = 65536, s = 0; s < t.length; s += 2) {
    if (a += t[s], a > e) return false;
    if (a += t[s + 1], a >= e) return true;
  }
  return false;
}
function Tt(e, t) {
  return e < 65 ? e === 36 : e < 91 ? true : e < 97 ? e === 95 : e < 123 ? true : e <= 65535 ? e >= 170 && t1.test(String.fromCharCode(e)) : t === false ? false : ml(e, Eh);
}
function Vt(e, t) {
  return e < 48 ? e === 36 : e < 58 ? true : e < 65 ? false : e < 91 ? true : e < 97 ? e === 95 : e < 123 ? true : e <= 65535 ? e >= 170 && r1.test(String.fromCharCode(e)) : t === false ? false : ml(e, Eh) || ml(e, Qv);
}
var Ce = function(t, a) {
  a === void 0 && (a = {}), this.label = t, this.keyword = a.keyword, this.beforeExpr = !!a.beforeExpr, this.startsExpr = !!a.startsExpr, this.isLoop = !!a.isLoop, this.isAssign = !!a.isAssign, this.prefix = !!a.prefix, this.postfix = !!a.postfix, this.binop = a.binop || null, this.updateContext = null;
};
function pt(e, t) {
  return new Ce(e, { beforeExpr: true, binop: t });
}
var ft = { beforeExpr: true }, st = { startsExpr: true }, Kt = {};
function Ae(e, t) {
  return t === void 0 && (t = {}), t.keyword = e, Kt[e] = new Ce(e, t);
}
var d = { num: new Ce("num", st), regexp: new Ce("regexp", st), string: new Ce("string", st), name: new Ce("name", st), privateId: new Ce("privateId", st), eof: new Ce("eof"), bracketL: new Ce("[", { beforeExpr: true, startsExpr: true }), bracketR: new Ce("]"), braceL: new Ce("{", { beforeExpr: true, startsExpr: true }), braceR: new Ce("}"), parenL: new Ce("(", { beforeExpr: true, startsExpr: true }), parenR: new Ce(")"), comma: new Ce(",", ft), semi: new Ce(";", ft), colon: new Ce(":", ft), dot: new Ce("."), question: new Ce("?", ft), questionDot: new Ce("?."), arrow: new Ce("=>", ft), template: new Ce("template"), invalidTemplate: new Ce("invalidTemplate"), ellipsis: new Ce("...", ft), backQuote: new Ce("`", st), dollarBraceL: new Ce("${", { beforeExpr: true, startsExpr: true }), eq: new Ce("=", { beforeExpr: true, isAssign: true }), assign: new Ce("_=", { beforeExpr: true, isAssign: true }), incDec: new Ce("++/--", { prefix: true, postfix: true, startsExpr: true }), prefix: new Ce("!/~", { beforeExpr: true, prefix: true, startsExpr: true }), logicalOR: pt("||", 1), logicalAND: pt("&&", 2), bitwiseOR: pt("|", 3), bitwiseXOR: pt("^", 4), bitwiseAND: pt("&", 5), equality: pt("==/!=/===/!==", 6), relational: pt("</>/<=/>=", 7), bitShift: pt("<</>>/>>>", 8), plusMin: new Ce("+/-", { beforeExpr: true, binop: 9, prefix: true, startsExpr: true }), modulo: pt("%", 10), star: pt("*", 10), slash: pt("/", 10), starstar: new Ce("**", { beforeExpr: true }), coalesce: pt("??", 1), _break: Ae("break"), _case: Ae("case", ft), _catch: Ae("catch"), _continue: Ae("continue"), _debugger: Ae("debugger"), _default: Ae("default", ft), _do: Ae("do", { isLoop: true, beforeExpr: true }), _else: Ae("else", ft), _finally: Ae("finally"), _for: Ae("for", { isLoop: true }), _function: Ae("function", st), _if: Ae("if"), _return: Ae("return", ft), _switch: Ae("switch"), _throw: Ae("throw", ft), _try: Ae("try"), _var: Ae("var"), _const: Ae("const"), _while: Ae("while", { isLoop: true }), _with: Ae("with"), _new: Ae("new", { beforeExpr: true, startsExpr: true }), _this: Ae("this", st), _super: Ae("super", st), _class: Ae("class", st), _extends: Ae("extends", ft), _export: Ae("export"), _import: Ae("import", st), _null: Ae("null", st), _true: Ae("true", st), _false: Ae("false", st), _in: Ae("in", { beforeExpr: true, binop: 7 }), _instanceof: Ae("instanceof", { beforeExpr: true, binop: 7 }), _typeof: Ae("typeof", { beforeExpr: true, prefix: true, startsExpr: true }), _void: Ae("void", { beforeExpr: true, prefix: true, startsExpr: true }), _delete: Ae("delete", { beforeExpr: true, prefix: true, startsExpr: true }) }, dt = /\r\n?|\n|\u2028|\u2029/, Ih = new RegExp(dt.source, "g");
function Ht(e) {
  return e === 10 || e === 13 || e === 8232 || e === 8233;
}
function Mh(e, t, a) {
  a === void 0 && (a = e.length);
  for (var s = t; s < a; s++) {
    var u = e.charCodeAt(s);
    if (Ht(u)) return s < a - 1 && u === 13 && e.charCodeAt(s + 1) === 10 ? s + 2 : s + 1;
  }
  return -1;
}
var Cl = /[\u1680\u2000-\u200a\u202f\u205f\u3000\ufeff]/, mt = /(?:\s|\/\/.*|\/\*[^]*?\*\/)*/g, Lh = Object.prototype, a1 = Lh.hasOwnProperty, i1 = Lh.toString, er = Object.hasOwn || function(e, t) {
  return a1.call(e, t);
}, $l = Array.isArray || function(e) {
  return i1.call(e) === "[object Array]";
}, Hl = /* @__PURE__ */ Object.create(null);
function Nt(e) {
  return Hl[e] || (Hl[e] = new RegExp("^(?:" + e.replace(/ /g, "|") + ")$"));
}
function kt(e) {
  return e <= 65535 ? String.fromCharCode(e) : (e -= 65536, String.fromCharCode((e >> 10) + 55296, (e & 1023) + 56320));
}
var s1 = /(?:[\uD800-\uDBFF](?![\uDC00-\uDFFF])|(?:[^\uD800-\uDBFF]|^)[\uDC00-\uDFFF])/, Jt = function(t, a) {
  this.line = t, this.column = a;
};
Jt.prototype.offset = function(t) {
  return new Jt(this.line, this.column + t);
};
var Po = function(t, a, s) {
  this.start = a, this.end = s, t.sourceFile !== null && (this.source = t.sourceFile);
};
function Pl(e, t) {
  for (var a = 1, s = 0; ; ) {
    var u = Mh(e, s, t);
    if (u < 0) return new Jt(a, t - s);
    ++a, s = u;
  }
}
var zo = { ecmaVersion: null, sourceType: "script", onInsertedSemicolon: null, onTrailingComma: null, allowReserved: null, allowReturnOutsideFunction: false, allowImportExportEverywhere: false, allowAwaitOutsideFunction: null, allowSuperOutsideMethod: null, allowHashBang: false, checkPrivateFields: true, locations: false, onToken: null, onComment: null, ranges: false, program: null, sourceFile: null, directSourceFile: null, preserveParens: false }, Ul = false;
function n1(e) {
  var t = {};
  for (var a in zo) t[a] = e && er(e, a) ? e[a] : zo[a];
  if (t.ecmaVersion === "latest" ? t.ecmaVersion = 1e8 : t.ecmaVersion == null ? (!Ul && typeof console == "object" && console.warn && (Ul = true, console.warn(`Since Acorn 8.0.0, options.ecmaVersion is required.
Defaulting to 2020, but this will stop working in the future.`)), t.ecmaVersion = 11) : t.ecmaVersion >= 2015 && (t.ecmaVersion -= 2009), t.allowReserved == null && (t.allowReserved = t.ecmaVersion < 5), (!e || e.allowHashBang == null) && (t.allowHashBang = t.ecmaVersion >= 14), $l(t.onToken)) {
    var s = t.onToken;
    t.onToken = function(u) {
      return s.push(u);
    };
  }
  return $l(t.onComment) && (t.onComment = o1(t, t.onComment)), t;
}
function o1(e, t) {
  return function(a, s, u, p, m, w) {
    var x = { type: a ? "Block" : "Line", value: s, start: u, end: p };
    e.locations && (x.loc = new Po(this, m, w)), e.ranges && (x.range = [u, p]), t.push(x);
  };
}
var Co = 1, tr = 2, ql = 4, Oh = 8, Nh = 16, Dh = 32, Tl = 64, jh = 128, qo = 256, Sl = Co | tr | qo;
function wl(e, t) {
  return tr | (e ? ql : 0) | (t ? Oh : 0);
}
var Wo = 0, Al = 1, Ot = 2, Bh = 3, Fh = 4, Vh = 5, Ue = function(t, a, s) {
  this.options = t = n1(t), this.sourceFile = t.sourceFile, this.keywords = Nt(Zv[t.ecmaVersion >= 6 ? 6 : t.sourceType === "module" ? "5module" : 5]);
  var u = "";
  t.allowReserved !== true && (u = ll[t.ecmaVersion >= 6 ? 6 : t.ecmaVersion === 5 ? 5 : 3], t.sourceType === "module" && (u += " await")), this.reservedWords = Nt(u);
  var p = (u ? u + " " : "") + ll.strict;
  this.reservedWordsStrict = Nt(p), this.reservedWordsStrictBind = Nt(p + " " + ll.strictBind), this.input = String(a), this.containsEsc = false, s ? (this.pos = s, this.lineStart = this.input.lastIndexOf(`
`, s - 1) + 1, this.curLine = this.input.slice(0, this.lineStart).split(dt).length) : (this.pos = this.lineStart = 0, this.curLine = 1), this.type = d.eof, this.value = null, this.start = this.end = this.pos, this.startLoc = this.endLoc = this.curPosition(), this.lastTokEndLoc = this.lastTokStartLoc = null, this.lastTokStart = this.lastTokEnd = this.pos, this.context = this.initialContext(), this.exprAllowed = true, this.inModule = t.sourceType === "module", this.strict = this.inModule || this.strictDirective(this.pos), this.potentialArrowAt = -1, this.potentialArrowInForAwait = false, this.yieldPos = this.awaitPos = this.awaitIdentPos = 0, this.labels = [], this.undefinedExports = /* @__PURE__ */ Object.create(null), this.pos === 0 && t.allowHashBang && this.input.slice(0, 2) === "#!" && this.skipLineComment(2), this.scopeStack = [], this.enterScope(Co), this.regexpState = null, this.privateNameStack = [];
}, St = { inFunction: { configurable: true }, inGenerator: { configurable: true }, inAsync: { configurable: true }, canAwait: { configurable: true }, allowSuper: { configurable: true }, allowDirectSuper: { configurable: true }, treatFunctionsAsVar: { configurable: true }, allowNewDotTarget: { configurable: true }, inClassStaticBlock: { configurable: true } };
Ue.prototype.parse = function() {
  var t = this.options.program || this.startNode();
  return this.nextToken(), this.parseTopLevel(t);
};
St.inFunction.get = function() {
  return (this.currentVarScope().flags & tr) > 0;
};
St.inGenerator.get = function() {
  return (this.currentVarScope().flags & Oh) > 0 && !this.currentVarScope().inClassFieldInit;
};
St.inAsync.get = function() {
  return (this.currentVarScope().flags & ql) > 0 && !this.currentVarScope().inClassFieldInit;
};
St.canAwait.get = function() {
  for (var e = this.scopeStack.length - 1; e >= 0; e--) {
    var t = this.scopeStack[e];
    if (t.inClassFieldInit || t.flags & qo) return false;
    if (t.flags & tr) return (t.flags & ql) > 0;
  }
  return this.inModule && this.options.ecmaVersion >= 13 || this.options.allowAwaitOutsideFunction;
};
St.allowSuper.get = function() {
  var e = this.currentThisScope(), t = e.flags, a = e.inClassFieldInit;
  return (t & Tl) > 0 || a || this.options.allowSuperOutsideMethod;
};
St.allowDirectSuper.get = function() {
  return (this.currentThisScope().flags & jh) > 0;
};
St.treatFunctionsAsVar.get = function() {
  return this.treatFunctionsAsVarInScope(this.currentScope());
};
St.allowNewDotTarget.get = function() {
  var e = this.currentThisScope(), t = e.flags, a = e.inClassFieldInit;
  return (t & (tr | qo)) > 0 || a;
};
St.inClassStaticBlock.get = function() {
  return (this.currentVarScope().flags & qo) > 0;
};
Ue.extend = function() {
  for (var t = [], a = arguments.length; a--; ) t[a] = arguments[a];
  for (var s = this, u = 0; u < t.length; u++) s = t[u](s);
  return s;
};
Ue.parse = function(t, a) {
  return new this(a, t).parse();
};
Ue.parseExpressionAt = function(t, a, s) {
  var u = new this(s, t, a);
  return u.nextToken(), u.parseExpression();
};
Ue.tokenizer = function(t, a) {
  return new this(a, t);
};
Object.defineProperties(Ue.prototype, St);
var at = Ue.prototype, l1 = /^(?:'((?:\\[^]|[^'\\])*?)'|"((?:\\[^]|[^"\\])*?)")/;
at.strictDirective = function(e) {
  if (this.options.ecmaVersion < 5) return false;
  for (; ; ) {
    mt.lastIndex = e, e += mt.exec(this.input)[0].length;
    var t = l1.exec(this.input.slice(e));
    if (!t) return false;
    if ((t[1] || t[2]) === "use strict") {
      mt.lastIndex = e + t[0].length;
      var a = mt.exec(this.input), s = a.index + a[0].length, u = this.input.charAt(s);
      return u === ";" || u === "}" || dt.test(a[0]) && !(/[(`.[+\-/*%<>=,?^&]/.test(u) || u === "!" && this.input.charAt(s + 1) === "=");
    }
    e += t[0].length, mt.lastIndex = e, e += mt.exec(this.input)[0].length, this.input[e] === ";" && e++;
  }
};
at.eat = function(e) {
  return this.type === e ? (this.next(), true) : false;
};
at.isContextual = function(e) {
  return this.type === d.name && this.value === e && !this.containsEsc;
};
at.eatContextual = function(e) {
  return this.isContextual(e) ? (this.next(), true) : false;
};
at.expectContextual = function(e) {
  this.eatContextual(e) || this.unexpected();
};
at.canInsertSemicolon = function() {
  return this.type === d.eof || this.type === d.braceR || dt.test(this.input.slice(this.lastTokEnd, this.start));
};
at.insertSemicolon = function() {
  if (this.canInsertSemicolon()) return this.options.onInsertedSemicolon && this.options.onInsertedSemicolon(this.lastTokEnd, this.lastTokEndLoc), true;
};
at.semicolon = function() {
  !this.eat(d.semi) && !this.insertSemicolon() && this.unexpected();
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
var Zo = function() {
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
  var a = e.shorthandAssign, s = e.doubleProto;
  if (!t) return a >= 0 || s >= 0;
  a >= 0 && this.raise(a, "Shorthand property assignments are valid only in destructuring patterns"), s >= 0 && this.raiseRecoverable(s, "Redefinition of __proto__ property");
};
at.checkYieldAwaitInDefaultParams = function() {
  this.yieldPos && (!this.awaitPos || this.yieldPos < this.awaitPos) && this.raise(this.yieldPos, "Yield expression cannot be a default value"), this.awaitPos && this.raise(this.awaitPos, "Await expression cannot be a default value");
};
at.isSimpleAssignTarget = function(e) {
  return e.type === "ParenthesizedExpression" ? this.isSimpleAssignTarget(e.expression) : e.type === "Identifier" || e.type === "MemberExpression";
};
var ue = Ue.prototype;
ue.parseTopLevel = function(e) {
  var t = /* @__PURE__ */ Object.create(null);
  for (e.body || (e.body = []); this.type !== d.eof; ) {
    var a = this.parseStatement(null, true, t);
    e.body.push(a);
  }
  if (this.inModule) for (var s = 0, u = Object.keys(this.undefinedExports); s < u.length; s += 1) {
    var p = u[s];
    this.raiseRecoverable(this.undefinedExports[p].start, "Export '" + p + "' is not defined");
  }
  return this.adaptDirectivePrologue(e.body), this.next(), e.sourceType = this.options.sourceType, this.finishNode(e, "Program");
};
var El = { kind: "loop" }, u1 = { kind: "switch" };
ue.isLet = function(e) {
  if (this.options.ecmaVersion < 6 || !this.isContextual("let")) return false;
  mt.lastIndex = this.pos;
  var t = mt.exec(this.input), a = this.pos + t[0].length, s = this.input.charCodeAt(a);
  if (s === 91 || s === 92) return true;
  if (e) return false;
  if (s === 123 || s > 55295 && s < 56320) return true;
  if (Tt(s, true)) {
    for (var u = a + 1; Vt(s = this.input.charCodeAt(u), true); ) ++u;
    if (s === 92 || s > 55295 && s < 56320) return true;
    var p = this.input.slice(a, u);
    if (!e1.test(p)) return true;
  }
  return false;
};
ue.isAsyncFunction = function() {
  if (this.options.ecmaVersion < 8 || !this.isContextual("async")) return false;
  mt.lastIndex = this.pos;
  var e = mt.exec(this.input), t = this.pos + e[0].length, a;
  return !dt.test(this.input.slice(this.pos, t)) && this.input.slice(t, t + 8) === "function" && (t + 8 === this.input.length || !(Vt(a = this.input.charCodeAt(t + 8)) || a > 55295 && a < 56320));
};
ue.parseStatement = function(e, t, a) {
  var s = this.type, u = this.startNode(), p;
  switch (this.isLet(e) && (s = d._var, p = "let"), s) {
    case d._break:
    case d._continue:
      return this.parseBreakContinueStatement(u, s.keyword);
    case d._debugger:
      return this.parseDebuggerStatement(u);
    case d._do:
      return this.parseDoStatement(u);
    case d._for:
      return this.parseForStatement(u);
    case d._function:
      return e && (this.strict || e !== "if" && e !== "label") && this.options.ecmaVersion >= 6 && this.unexpected(), this.parseFunctionStatement(u, false, !e);
    case d._class:
      return e && this.unexpected(), this.parseClass(u, true);
    case d._if:
      return this.parseIfStatement(u);
    case d._return:
      return this.parseReturnStatement(u);
    case d._switch:
      return this.parseSwitchStatement(u);
    case d._throw:
      return this.parseThrowStatement(u);
    case d._try:
      return this.parseTryStatement(u);
    case d._const:
    case d._var:
      return p = p || this.value, e && p !== "var" && this.unexpected(), this.parseVarStatement(u, p);
    case d._while:
      return this.parseWhileStatement(u);
    case d._with:
      return this.parseWithStatement(u);
    case d.braceL:
      return this.parseBlock(true, u);
    case d.semi:
      return this.parseEmptyStatement(u);
    case d._export:
    case d._import:
      if (this.options.ecmaVersion > 10 && s === d._import) {
        mt.lastIndex = this.pos;
        var m = mt.exec(this.input), w = this.pos + m[0].length, x = this.input.charCodeAt(w);
        if (x === 40 || x === 46) return this.parseExpressionStatement(u, this.parseExpression());
      }
      return this.options.allowImportExportEverywhere || (t || this.raise(this.start, "'import' and 'export' may only appear at the top level"), this.inModule || this.raise(this.start, "'import' and 'export' may appear only with 'sourceType: module'")), s === d._import ? this.parseImport(u) : this.parseExport(u, a);
    default:
      if (this.isAsyncFunction()) return e && this.unexpected(), this.next(), this.parseFunctionStatement(u, true, !e);
      var o = this.value, y = this.parseExpression();
      return s === d.name && y.type === "Identifier" && this.eat(d.colon) ? this.parseLabeledStatement(u, o, y, e) : this.parseExpressionStatement(u, y);
  }
};
ue.parseBreakContinueStatement = function(e, t) {
  var a = t === "break";
  this.next(), this.eat(d.semi) || this.insertSemicolon() ? e.label = null : this.type !== d.name ? this.unexpected() : (e.label = this.parseIdent(), this.semicolon());
  for (var s = 0; s < this.labels.length; ++s) {
    var u = this.labels[s];
    if ((e.label == null || u.name === e.label.name) && (u.kind != null && (a || u.kind === "loop") || e.label && a)) break;
  }
  return s === this.labels.length && this.raise(e.start, "Unsyntactic " + t), this.finishNode(e, a ? "BreakStatement" : "ContinueStatement");
};
ue.parseDebuggerStatement = function(e) {
  return this.next(), this.semicolon(), this.finishNode(e, "DebuggerStatement");
};
ue.parseDoStatement = function(e) {
  return this.next(), this.labels.push(El), e.body = this.parseStatement("do"), this.labels.pop(), this.expect(d._while), e.test = this.parseParenExpression(), this.options.ecmaVersion >= 6 ? this.eat(d.semi) : this.semicolon(), this.finishNode(e, "DoWhileStatement");
};
ue.parseForStatement = function(e) {
  this.next();
  var t = this.options.ecmaVersion >= 9 && this.canAwait && this.eatContextual("await") ? this.lastTokStart : -1;
  if (this.labels.push(El), this.enterScope(0), this.expect(d.parenL), this.type === d.semi) return t > -1 && this.unexpected(t), this.parseFor(e, null);
  var a = this.isLet();
  if (this.type === d._var || this.type === d._const || a) {
    var s = this.startNode(), u = a ? "let" : this.value;
    return this.next(), this.parseVar(s, true, u), this.finishNode(s, "VariableDeclaration"), (this.type === d._in || this.options.ecmaVersion >= 6 && this.isContextual("of")) && s.declarations.length === 1 ? (this.options.ecmaVersion >= 9 && (this.type === d._in ? t > -1 && this.unexpected(t) : e.await = t > -1), this.parseForIn(e, s)) : (t > -1 && this.unexpected(t), this.parseFor(e, s));
  }
  var p = this.isContextual("let"), m = false, w = this.containsEsc, x = new Zo(), o = this.start, y = t > -1 ? this.parseExprSubscripts(x, "await") : this.parseExpression(true, x);
  return this.type === d._in || (m = this.options.ecmaVersion >= 6 && this.isContextual("of")) ? (t > -1 ? (this.type === d._in && this.unexpected(t), e.await = true) : m && this.options.ecmaVersion >= 8 && (y.start === o && !w && y.type === "Identifier" && y.name === "async" ? this.unexpected() : this.options.ecmaVersion >= 9 && (e.await = false)), p && m && this.raise(y.start, "The left-hand side of a for-of loop may not start with 'let'."), this.toAssignable(y, false, x), this.checkLValPattern(y), this.parseForIn(e, y)) : (this.checkExpressionErrors(x, true), t > -1 && this.unexpected(t), this.parseFor(e, y));
};
ue.parseFunctionStatement = function(e, t, a) {
  return this.next(), this.parseFunction(e, Ro | (a ? 0 : vl), false, t);
};
ue.parseIfStatement = function(e) {
  return this.next(), e.test = this.parseParenExpression(), e.consequent = this.parseStatement("if"), e.alternate = this.eat(d._else) ? this.parseStatement("if") : null, this.finishNode(e, "IfStatement");
};
ue.parseReturnStatement = function(e) {
  return !this.inFunction && !this.options.allowReturnOutsideFunction && this.raise(this.start, "'return' outside of function"), this.next(), this.eat(d.semi) || this.insertSemicolon() ? e.argument = null : (e.argument = this.parseExpression(), this.semicolon()), this.finishNode(e, "ReturnStatement");
};
ue.parseSwitchStatement = function(e) {
  this.next(), e.discriminant = this.parseParenExpression(), e.cases = [], this.expect(d.braceL), this.labels.push(u1), this.enterScope(0);
  for (var t, a = false; this.type !== d.braceR; ) if (this.type === d._case || this.type === d._default) {
    var s = this.type === d._case;
    t && this.finishNode(t, "SwitchCase"), e.cases.push(t = this.startNode()), t.consequent = [], this.next(), s ? t.test = this.parseExpression() : (a && this.raiseRecoverable(this.lastTokStart, "Multiple default clauses"), a = true, t.test = null), this.expect(d.colon);
  } else t || this.unexpected(), t.consequent.push(this.parseStatement(null));
  return this.exitScope(), t && this.finishNode(t, "SwitchCase"), this.next(), this.labels.pop(), this.finishNode(e, "SwitchStatement");
};
ue.parseThrowStatement = function(e) {
  return this.next(), dt.test(this.input.slice(this.lastTokEnd, this.start)) && this.raise(this.lastTokEnd, "Illegal newline after throw"), e.argument = this.parseExpression(), this.semicolon(), this.finishNode(e, "ThrowStatement");
};
var c1 = [];
ue.parseCatchClauseParam = function() {
  var e = this.parseBindingAtom(), t = e.type === "Identifier";
  return this.enterScope(t ? Dh : 0), this.checkLValPattern(e, t ? Fh : Ot), this.expect(d.parenR), e;
};
ue.parseTryStatement = function(e) {
  if (this.next(), e.block = this.parseBlock(), e.handler = null, this.type === d._catch) {
    var t = this.startNode();
    this.next(), this.eat(d.parenL) ? t.param = this.parseCatchClauseParam() : (this.options.ecmaVersion < 10 && this.unexpected(), t.param = null, this.enterScope(0)), t.body = this.parseBlock(false), this.exitScope(), e.handler = this.finishNode(t, "CatchClause");
  }
  return e.finalizer = this.eat(d._finally) ? this.parseBlock() : null, !e.handler && !e.finalizer && this.raise(e.start, "Missing catch or finally clause"), this.finishNode(e, "TryStatement");
};
ue.parseVarStatement = function(e, t, a) {
  return this.next(), this.parseVar(e, false, t, a), this.semicolon(), this.finishNode(e, "VariableDeclaration");
};
ue.parseWhileStatement = function(e) {
  return this.next(), e.test = this.parseParenExpression(), this.labels.push(El), e.body = this.parseStatement("while"), this.labels.pop(), this.finishNode(e, "WhileStatement");
};
ue.parseWithStatement = function(e) {
  return this.strict && this.raise(this.start, "'with' in strict mode"), this.next(), e.object = this.parseParenExpression(), e.body = this.parseStatement("with"), this.finishNode(e, "WithStatement");
};
ue.parseEmptyStatement = function(e) {
  return this.next(), this.finishNode(e, "EmptyStatement");
};
ue.parseLabeledStatement = function(e, t, a, s) {
  for (var u = 0, p = this.labels; u < p.length; u += 1) {
    var m = p[u];
    m.name === t && this.raise(a.start, "Label '" + t + "' is already declared");
  }
  for (var w = this.type.isLoop ? "loop" : this.type === d._switch ? "switch" : null, x = this.labels.length - 1; x >= 0; x--) {
    var o = this.labels[x];
    if (o.statementStart === e.start) o.statementStart = this.start, o.kind = w;
    else break;
  }
  return this.labels.push({ name: t, kind: w, statementStart: this.start }), e.body = this.parseStatement(s ? s.indexOf("label") === -1 ? s + "label" : s : "label"), this.labels.pop(), e.label = a, this.finishNode(e, "LabeledStatement");
};
ue.parseExpressionStatement = function(e, t) {
  return e.expression = t, this.semicolon(), this.finishNode(e, "ExpressionStatement");
};
ue.parseBlock = function(e, t, a) {
  for (e === void 0 && (e = true), t === void 0 && (t = this.startNode()), t.body = [], this.expect(d.braceL), e && this.enterScope(0); this.type !== d.braceR; ) {
    var s = this.parseStatement(null);
    t.body.push(s);
  }
  return a && (this.strict = false), this.next(), e && this.exitScope(), this.finishNode(t, "BlockStatement");
};
ue.parseFor = function(e, t) {
  return e.init = t, this.expect(d.semi), e.test = this.type === d.semi ? null : this.parseExpression(), this.expect(d.semi), e.update = this.type === d.parenR ? null : this.parseExpression(), this.expect(d.parenR), e.body = this.parseStatement("for"), this.exitScope(), this.labels.pop(), this.finishNode(e, "ForStatement");
};
ue.parseForIn = function(e, t) {
  var a = this.type === d._in;
  return this.next(), t.type === "VariableDeclaration" && t.declarations[0].init != null && (!a || this.options.ecmaVersion < 8 || this.strict || t.kind !== "var" || t.declarations[0].id.type !== "Identifier") && this.raise(t.start, (a ? "for-in" : "for-of") + " loop variable declaration may not have an initializer"), e.left = t, e.right = a ? this.parseExpression() : this.parseMaybeAssign(), this.expect(d.parenR), e.body = this.parseStatement("for"), this.exitScope(), this.labels.pop(), this.finishNode(e, a ? "ForInStatement" : "ForOfStatement");
};
ue.parseVar = function(e, t, a, s) {
  for (e.declarations = [], e.kind = a; ; ) {
    var u = this.startNode();
    if (this.parseVarId(u, a), this.eat(d.eq) ? u.init = this.parseMaybeAssign(t) : !s && a === "const" && !(this.type === d._in || this.options.ecmaVersion >= 6 && this.isContextual("of")) ? this.unexpected() : !s && u.id.type !== "Identifier" && !(t && (this.type === d._in || this.isContextual("of"))) ? this.raise(this.lastTokEnd, "Complex binding patterns require an initialization value") : u.init = null, e.declarations.push(this.finishNode(u, "VariableDeclarator")), !this.eat(d.comma)) break;
  }
  return e;
};
ue.parseVarId = function(e, t) {
  e.id = this.parseBindingAtom(), this.checkLValPattern(e.id, t === "var" ? Al : Ot, false);
};
var Ro = 1, vl = 2, $h = 4;
ue.parseFunction = function(e, t, a, s, u) {
  this.initFunction(e), (this.options.ecmaVersion >= 9 || this.options.ecmaVersion >= 6 && !s) && (this.type === d.star && t & vl && this.unexpected(), e.generator = this.eat(d.star)), this.options.ecmaVersion >= 8 && (e.async = !!s), t & Ro && (e.id = t & $h && this.type !== d.name ? null : this.parseIdent(), e.id && !(t & vl) && this.checkLValSimple(e.id, this.strict || e.generator || e.async ? this.treatFunctionsAsVar ? Al : Ot : Bh));
  var p = this.yieldPos, m = this.awaitPos, w = this.awaitIdentPos;
  return this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0, this.enterScope(wl(e.async, e.generator)), t & Ro || (e.id = this.type === d.name ? this.parseIdent() : null), this.parseFunctionParams(e), this.parseFunctionBody(e, a, false, u), this.yieldPos = p, this.awaitPos = m, this.awaitIdentPos = w, this.finishNode(e, t & Ro ? "FunctionDeclaration" : "FunctionExpression");
};
ue.parseFunctionParams = function(e) {
  this.expect(d.parenL), e.params = this.parseBindingList(d.parenR, false, this.options.ecmaVersion >= 8), this.checkYieldAwaitInDefaultParams();
};
ue.parseClass = function(e, t) {
  this.next();
  var a = this.strict;
  this.strict = true, this.parseClassId(e, t), this.parseClassSuper(e);
  var s = this.enterClassBody(), u = this.startNode(), p = false;
  for (u.body = [], this.expect(d.braceL); this.type !== d.braceR; ) {
    var m = this.parseClassElement(e.superClass !== null);
    m && (u.body.push(m), m.type === "MethodDefinition" && m.kind === "constructor" ? (p && this.raiseRecoverable(m.start, "Duplicate constructor in the same class"), p = true) : m.key && m.key.type === "PrivateIdentifier" && d1(s, m) && this.raiseRecoverable(m.key.start, "Identifier '#" + m.key.name + "' has already been declared"));
  }
  return this.strict = a, this.next(), e.body = this.finishNode(u, "ClassBody"), this.exitClassBody(), this.finishNode(e, t ? "ClassDeclaration" : "ClassExpression");
};
ue.parseClassElement = function(e) {
  if (this.eat(d.semi)) return null;
  var t = this.options.ecmaVersion, a = this.startNode(), s = "", u = false, p = false, m = "method", w = false;
  if (this.eatContextual("static")) {
    if (t >= 13 && this.eat(d.braceL)) return this.parseClassStaticBlock(a), a;
    this.isClassElementNameStart() || this.type === d.star ? w = true : s = "static";
  }
  if (a.static = w, !s && t >= 8 && this.eatContextual("async") && ((this.isClassElementNameStart() || this.type === d.star) && !this.canInsertSemicolon() ? p = true : s = "async"), !s && (t >= 9 || !p) && this.eat(d.star) && (u = true), !s && !p && !u) {
    var x = this.value;
    (this.eatContextual("get") || this.eatContextual("set")) && (this.isClassElementNameStart() ? m = x : s = x);
  }
  if (s ? (a.computed = false, a.key = this.startNodeAt(this.lastTokStart, this.lastTokStartLoc), a.key.name = s, this.finishNode(a.key, "Identifier")) : this.parseClassElementName(a), t < 13 || this.type === d.parenL || m !== "method" || u || p) {
    var o = !a.static && Xo(a, "constructor"), y = o && e;
    o && m !== "method" && this.raise(a.key.start, "Constructor can't have get/set modifier"), a.kind = o ? "constructor" : m, this.parseClassMethod(a, u, p, y);
  } else this.parseClassField(a);
  return a;
};
ue.isClassElementNameStart = function() {
  return this.type === d.name || this.type === d.privateId || this.type === d.num || this.type === d.string || this.type === d.bracketL || this.type.keyword;
};
ue.parseClassElementName = function(e) {
  this.type === d.privateId ? (this.value === "constructor" && this.raise(this.start, "Classes can't have an element named '#constructor'"), e.computed = false, e.key = this.parsePrivateIdent()) : this.parsePropertyName(e);
};
ue.parseClassMethod = function(e, t, a, s) {
  var u = e.key;
  e.kind === "constructor" ? (t && this.raise(u.start, "Constructor can't be a generator"), a && this.raise(u.start, "Constructor can't be an async method")) : e.static && Xo(e, "prototype") && this.raise(u.start, "Classes may not have a static property named prototype");
  var p = e.value = this.parseMethod(t, a, s);
  return e.kind === "get" && p.params.length !== 0 && this.raiseRecoverable(p.start, "getter should have no params"), e.kind === "set" && p.params.length !== 1 && this.raiseRecoverable(p.start, "setter should have exactly one param"), e.kind === "set" && p.params[0].type === "RestElement" && this.raiseRecoverable(p.params[0].start, "Setter cannot use rest params"), this.finishNode(e, "MethodDefinition");
};
ue.parseClassField = function(e) {
  if (Xo(e, "constructor") ? this.raise(e.key.start, "Classes can't have a field named 'constructor'") : e.static && Xo(e, "prototype") && this.raise(e.key.start, "Classes can't have a static field named 'prototype'"), this.eat(d.eq)) {
    var t = this.currentThisScope(), a = t.inClassFieldInit;
    t.inClassFieldInit = true, e.value = this.parseMaybeAssign(), t.inClassFieldInit = a;
  } else e.value = null;
  return this.semicolon(), this.finishNode(e, "PropertyDefinition");
};
ue.parseClassStaticBlock = function(e) {
  e.body = [];
  var t = this.labels;
  for (this.labels = [], this.enterScope(qo | Tl); this.type !== d.braceR; ) {
    var a = this.parseStatement(null);
    e.body.push(a);
  }
  return this.next(), this.exitScope(), this.labels = t, this.finishNode(e, "StaticBlock");
};
ue.parseClassId = function(e, t) {
  this.type === d.name ? (e.id = this.parseIdent(), t && this.checkLValSimple(e.id, Ot, false)) : (t === true && this.unexpected(), e.id = null);
};
ue.parseClassSuper = function(e) {
  e.superClass = this.eat(d._extends) ? this.parseExprSubscripts(null, false) : null;
};
ue.enterClassBody = function() {
  var e = { declared: /* @__PURE__ */ Object.create(null), used: [] };
  return this.privateNameStack.push(e), e.declared;
};
ue.exitClassBody = function() {
  var e = this.privateNameStack.pop(), t = e.declared, a = e.used;
  if (this.options.checkPrivateFields) for (var s = this.privateNameStack.length, u = s === 0 ? null : this.privateNameStack[s - 1], p = 0; p < a.length; ++p) {
    var m = a[p];
    er(t, m.name) || (u ? u.used.push(m) : this.raiseRecoverable(m.start, "Private field '#" + m.name + "' must be declared in an enclosing class"));
  }
};
function d1(e, t) {
  var a = t.key.name, s = e[a], u = "true";
  return t.type === "MethodDefinition" && (t.kind === "get" || t.kind === "set") && (u = (t.static ? "s" : "i") + t.kind), s === "iget" && u === "iset" || s === "iset" && u === "iget" || s === "sget" && u === "sset" || s === "sset" && u === "sget" ? (e[a] = "true", false) : s ? true : (e[a] = u, false);
}
function Xo(e, t) {
  var a = e.computed, s = e.key;
  return !a && (s.type === "Identifier" && s.name === t || s.type === "Literal" && s.value === t);
}
ue.parseExportAllDeclaration = function(e, t) {
  return this.options.ecmaVersion >= 11 && (this.eatContextual("as") ? (e.exported = this.parseModuleExportName(), this.checkExport(t, e.exported, this.lastTokStart)) : e.exported = null), this.expectContextual("from"), this.type !== d.string && this.unexpected(), e.source = this.parseExprAtom(), this.options.ecmaVersion >= 16 && (e.attributes = this.parseWithClause()), this.semicolon(), this.finishNode(e, "ExportAllDeclaration");
};
ue.parseExport = function(e, t) {
  if (this.next(), this.eat(d.star)) return this.parseExportAllDeclaration(e, t);
  if (this.eat(d._default)) return this.checkExport(t, "default", this.lastTokStart), e.declaration = this.parseExportDefaultDeclaration(), this.finishNode(e, "ExportDefaultDeclaration");
  if (this.shouldParseExportStatement()) e.declaration = this.parseExportDeclaration(e), e.declaration.type === "VariableDeclaration" ? this.checkVariableExport(t, e.declaration.declarations) : this.checkExport(t, e.declaration.id, e.declaration.id.start), e.specifiers = [], e.source = null;
  else {
    if (e.declaration = null, e.specifiers = this.parseExportSpecifiers(t), this.eatContextual("from")) this.type !== d.string && this.unexpected(), e.source = this.parseExprAtom(), this.options.ecmaVersion >= 16 && (e.attributes = this.parseWithClause());
    else {
      for (var a = 0, s = e.specifiers; a < s.length; a += 1) {
        var u = s[a];
        this.checkUnreserved(u.local), this.checkLocalExport(u.local), u.local.type === "Literal" && this.raise(u.local.start, "A string literal cannot be used as an exported binding without `from`.");
      }
      e.source = null;
    }
    this.semicolon();
  }
  return this.finishNode(e, "ExportNamedDeclaration");
};
ue.parseExportDeclaration = function(e) {
  return this.parseStatement(null);
};
ue.parseExportDefaultDeclaration = function() {
  var e;
  if (this.type === d._function || (e = this.isAsyncFunction())) {
    var t = this.startNode();
    return this.next(), e && this.next(), this.parseFunction(t, Ro | $h, false, e);
  } else if (this.type === d._class) {
    var a = this.startNode();
    return this.parseClass(a, "nullableID");
  } else {
    var s = this.parseMaybeAssign();
    return this.semicolon(), s;
  }
};
ue.checkExport = function(e, t, a) {
  e && (typeof t != "string" && (t = t.type === "Identifier" ? t.name : t.value), er(e, t) && this.raiseRecoverable(a, "Duplicate export '" + t + "'"), e[t] = true);
};
ue.checkPatternExport = function(e, t) {
  var a = t.type;
  if (a === "Identifier") this.checkExport(e, t, t.start);
  else if (a === "ObjectPattern") for (var s = 0, u = t.properties; s < u.length; s += 1) {
    var p = u[s];
    this.checkPatternExport(e, p);
  }
  else if (a === "ArrayPattern") for (var m = 0, w = t.elements; m < w.length; m += 1) {
    var x = w[m];
    x && this.checkPatternExport(e, x);
  }
  else a === "Property" ? this.checkPatternExport(e, t.value) : a === "AssignmentPattern" ? this.checkPatternExport(e, t.left) : a === "RestElement" && this.checkPatternExport(e, t.argument);
};
ue.checkVariableExport = function(e, t) {
  if (e) for (var a = 0, s = t; a < s.length; a += 1) {
    var u = s[a];
    this.checkPatternExport(e, u.id);
  }
};
ue.shouldParseExportStatement = function() {
  return this.type.keyword === "var" || this.type.keyword === "const" || this.type.keyword === "class" || this.type.keyword === "function" || this.isLet() || this.isAsyncFunction();
};
ue.parseExportSpecifier = function(e) {
  var t = this.startNode();
  return t.local = this.parseModuleExportName(), t.exported = this.eatContextual("as") ? this.parseModuleExportName() : t.local, this.checkExport(e, t.exported, t.exported.start), this.finishNode(t, "ExportSpecifier");
};
ue.parseExportSpecifiers = function(e) {
  var t = [], a = true;
  for (this.expect(d.braceL); !this.eat(d.braceR); ) {
    if (a) a = false;
    else if (this.expect(d.comma), this.afterTrailingComma(d.braceR)) break;
    t.push(this.parseExportSpecifier(e));
  }
  return t;
};
ue.parseImport = function(e) {
  return this.next(), this.type === d.string ? (e.specifiers = c1, e.source = this.parseExprAtom()) : (e.specifiers = this.parseImportSpecifiers(), this.expectContextual("from"), e.source = this.type === d.string ? this.parseExprAtom() : this.unexpected()), this.options.ecmaVersion >= 16 && (e.attributes = this.parseWithClause()), this.semicolon(), this.finishNode(e, "ImportDeclaration");
};
ue.parseImportSpecifier = function() {
  var e = this.startNode();
  return e.imported = this.parseModuleExportName(), this.eatContextual("as") ? e.local = this.parseIdent() : (this.checkUnreserved(e.imported), e.local = e.imported), this.checkLValSimple(e.local, Ot), this.finishNode(e, "ImportSpecifier");
};
ue.parseImportDefaultSpecifier = function() {
  var e = this.startNode();
  return e.local = this.parseIdent(), this.checkLValSimple(e.local, Ot), this.finishNode(e, "ImportDefaultSpecifier");
};
ue.parseImportNamespaceSpecifier = function() {
  var e = this.startNode();
  return this.next(), this.expectContextual("as"), e.local = this.parseIdent(), this.checkLValSimple(e.local, Ot), this.finishNode(e, "ImportNamespaceSpecifier");
};
ue.parseImportSpecifiers = function() {
  var e = [], t = true;
  if (this.type === d.name && (e.push(this.parseImportDefaultSpecifier()), !this.eat(d.comma))) return e;
  if (this.type === d.star) return e.push(this.parseImportNamespaceSpecifier()), e;
  for (this.expect(d.braceL); !this.eat(d.braceR); ) {
    if (t) t = false;
    else if (this.expect(d.comma), this.afterTrailingComma(d.braceR)) break;
    e.push(this.parseImportSpecifier());
  }
  return e;
};
ue.parseWithClause = function() {
  var e = [];
  if (!this.eat(d._with)) return e;
  this.expect(d.braceL);
  for (var t = {}, a = true; !this.eat(d.braceR); ) {
    if (a) a = false;
    else if (this.expect(d.comma), this.afterTrailingComma(d.braceR)) break;
    var s = this.parseImportAttribute(), u = s.key.type === "Identifier" ? s.key.name : s.key.value;
    er(t, u) && this.raiseRecoverable(s.key.start, "Duplicate attribute key '" + u + "'"), t[u] = true, e.push(s);
  }
  return e;
};
ue.parseImportAttribute = function() {
  var e = this.startNode();
  return e.key = this.type === d.string ? this.parseExprAtom() : this.parseIdent(this.options.allowReserved !== "never"), this.expect(d.colon), this.type !== d.string && this.unexpected(), e.value = this.parseExprAtom(), this.finishNode(e, "ImportAttribute");
};
ue.parseModuleExportName = function() {
  if (this.options.ecmaVersion >= 13 && this.type === d.string) {
    var e = this.parseLiteral(this.value);
    return s1.test(e.value) && this.raise(e.start, "An export name cannot include a lone surrogate."), e;
  }
  return this.parseIdent(true);
};
ue.adaptDirectivePrologue = function(e) {
  for (var t = 0; t < e.length && this.isDirectiveCandidate(e[t]); ++t) e[t].directive = e[t].expression.raw.slice(1, -1);
};
ue.isDirectiveCandidate = function(e) {
  return this.options.ecmaVersion >= 5 && e.type === "ExpressionStatement" && e.expression.type === "Literal" && typeof e.expression.value == "string" && (this.input[e.start] === '"' || this.input[e.start] === "'");
};
var yt = Ue.prototype;
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
      for (var s = 0, u = e.properties; s < u.length; s += 1) {
        var p = u[s];
        this.toAssignable(p, t), p.type === "RestElement" && (p.argument.type === "ArrayPattern" || p.argument.type === "ObjectPattern") && this.raise(p.argument.start, "Unexpected token");
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
  for (var a = e.length, s = 0; s < a; s++) {
    var u = e[s];
    u && this.toAssignable(u, t);
  }
  if (a) {
    var p = e[a - 1];
    this.options.ecmaVersion === 6 && t && p && p.type === "RestElement" && p.argument.type !== "Identifier" && this.unexpected(p.argument.start);
  }
  return e;
};
yt.parseSpread = function(e) {
  var t = this.startNode();
  return this.next(), t.argument = this.parseMaybeAssign(false, e), this.finishNode(t, "SpreadElement");
};
yt.parseRestBinding = function() {
  var e = this.startNode();
  return this.next(), this.options.ecmaVersion === 6 && this.type !== d.name && this.unexpected(), e.argument = this.parseBindingAtom(), this.finishNode(e, "RestElement");
};
yt.parseBindingAtom = function() {
  if (this.options.ecmaVersion >= 6) switch (this.type) {
    case d.bracketL:
      var e = this.startNode();
      return this.next(), e.elements = this.parseBindingList(d.bracketR, true, true), this.finishNode(e, "ArrayPattern");
    case d.braceL:
      return this.parseObj(true);
  }
  return this.parseIdent();
};
yt.parseBindingList = function(e, t, a, s) {
  for (var u = [], p = true; !this.eat(e); ) if (p ? p = false : this.expect(d.comma), t && this.type === d.comma) u.push(null);
  else {
    if (a && this.afterTrailingComma(e)) break;
    if (this.type === d.ellipsis) {
      var m = this.parseRestBinding();
      this.parseBindingListItem(m), u.push(m), this.type === d.comma && this.raiseRecoverable(this.start, "Comma is not permitted after the rest element"), this.expect(e);
      break;
    } else u.push(this.parseAssignableListItem(s));
  }
  return u;
};
yt.parseAssignableListItem = function(e) {
  var t = this.parseMaybeDefault(this.start, this.startLoc);
  return this.parseBindingListItem(t), t;
};
yt.parseBindingListItem = function(e) {
  return e;
};
yt.parseMaybeDefault = function(e, t, a) {
  if (a = a || this.parseBindingAtom(), this.options.ecmaVersion < 6 || !this.eat(d.eq)) return a;
  var s = this.startNodeAt(e, t);
  return s.left = a, s.right = this.parseMaybeAssign(), this.finishNode(s, "AssignmentPattern");
};
yt.checkLValSimple = function(e, t, a) {
  t === void 0 && (t = Wo);
  var s = t !== Wo;
  switch (e.type) {
    case "Identifier":
      this.strict && this.reservedWordsStrictBind.test(e.name) && this.raiseRecoverable(e.start, (s ? "Binding " : "Assigning to ") + e.name + " in strict mode"), s && (t === Ot && e.name === "let" && this.raiseRecoverable(e.start, "let is disallowed as a lexically bound name"), a && (er(a, e.name) && this.raiseRecoverable(e.start, "Argument name clash"), a[e.name] = true), t !== Vh && this.declareName(e.name, t, e.start));
      break;
    case "ChainExpression":
      this.raiseRecoverable(e.start, "Optional chaining cannot appear in left-hand side");
      break;
    case "MemberExpression":
      s && this.raiseRecoverable(e.start, "Binding member expression");
      break;
    case "ParenthesizedExpression":
      return s && this.raiseRecoverable(e.start, "Binding parenthesized expression"), this.checkLValSimple(e.expression, t, a);
    default:
      this.raise(e.start, (s ? "Binding" : "Assigning to") + " rvalue");
  }
};
yt.checkLValPattern = function(e, t, a) {
  switch (t === void 0 && (t = Wo), e.type) {
    case "ObjectPattern":
      for (var s = 0, u = e.properties; s < u.length; s += 1) {
        var p = u[s];
        this.checkLValInnerPattern(p, t, a);
      }
      break;
    case "ArrayPattern":
      for (var m = 0, w = e.elements; m < w.length; m += 1) {
        var x = w[m];
        x && this.checkLValInnerPattern(x, t, a);
      }
      break;
    default:
      this.checkLValSimple(e, t, a);
  }
};
yt.checkLValInnerPattern = function(e, t, a) {
  switch (t === void 0 && (t = Wo), e.type) {
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
var rt = function(t, a, s, u, p) {
  this.token = t, this.isExpr = !!a, this.preserveSpace = !!s, this.override = u, this.generator = !!p;
}, Ve = { b_stat: new rt("{", false), b_expr: new rt("{", true), b_tmpl: new rt("${", false), p_stat: new rt("(", false), p_expr: new rt("(", true), q_tmpl: new rt("`", true, true, function(e) {
  return e.tryReadTemplateToken();
}), f_stat: new rt("function", false), f_expr: new rt("function", true), f_expr_gen: new rt("function", true, false, null, true), f_gen: new rt("function", false, false, null, true) }, rr = Ue.prototype;
rr.initialContext = function() {
  return [Ve.b_stat];
};
rr.curContext = function() {
  return this.context[this.context.length - 1];
};
rr.braceIsBlock = function(e) {
  var t = this.curContext();
  return t === Ve.f_expr || t === Ve.f_stat ? true : e === d.colon && (t === Ve.b_stat || t === Ve.b_expr) ? !t.isExpr : e === d._return || e === d.name && this.exprAllowed ? dt.test(this.input.slice(this.lastTokEnd, this.start)) : e === d._else || e === d.semi || e === d.eof || e === d.parenR || e === d.arrow ? true : e === d.braceL ? t === Ve.b_stat : e === d._var || e === d._const || e === d.name ? false : !this.exprAllowed;
};
rr.inGeneratorContext = function() {
  for (var e = this.context.length - 1; e >= 1; e--) {
    var t = this.context[e];
    if (t.token === "function") return t.generator;
  }
  return false;
};
rr.updateContext = function(e) {
  var t, a = this.type;
  a.keyword && e === d.dot ? this.exprAllowed = false : (t = a.updateContext) ? t.call(this, e) : this.exprAllowed = a.beforeExpr;
};
rr.overrideContext = function(e) {
  this.curContext() !== e && (this.context[this.context.length - 1] = e);
};
d.parenR.updateContext = d.braceR.updateContext = function() {
  if (this.context.length === 1) {
    this.exprAllowed = true;
    return;
  }
  var e = this.context.pop();
  e === Ve.b_stat && this.curContext().token === "function" && (e = this.context.pop()), this.exprAllowed = !e.isExpr;
};
d.braceL.updateContext = function(e) {
  this.context.push(this.braceIsBlock(e) ? Ve.b_stat : Ve.b_expr), this.exprAllowed = true;
};
d.dollarBraceL.updateContext = function() {
  this.context.push(Ve.b_tmpl), this.exprAllowed = true;
};
d.parenL.updateContext = function(e) {
  var t = e === d._if || e === d._for || e === d._with || e === d._while;
  this.context.push(t ? Ve.p_stat : Ve.p_expr), this.exprAllowed = true;
};
d.incDec.updateContext = function() {
};
d._function.updateContext = d._class.updateContext = function(e) {
  e.beforeExpr && e !== d._else && !(e === d.semi && this.curContext() !== Ve.p_stat) && !(e === d._return && dt.test(this.input.slice(this.lastTokEnd, this.start))) && !((e === d.colon || e === d.braceL) && this.curContext() === Ve.b_stat) ? this.context.push(Ve.f_expr) : this.context.push(Ve.f_stat), this.exprAllowed = false;
};
d.colon.updateContext = function() {
  this.curContext().token === "function" && this.context.pop(), this.exprAllowed = true;
};
d.backQuote.updateContext = function() {
  this.curContext() === Ve.q_tmpl ? this.context.pop() : this.context.push(Ve.q_tmpl), this.exprAllowed = false;
};
d.star.updateContext = function(e) {
  if (e === d._function) {
    var t = this.context.length - 1;
    this.context[t] === Ve.f_expr ? this.context[t] = Ve.f_expr_gen : this.context[t] = Ve.f_gen;
  }
  this.exprAllowed = true;
};
d.name.updateContext = function(e) {
  var t = false;
  this.options.ecmaVersion >= 6 && e !== d.dot && (this.value === "of" && !this.exprAllowed || this.value === "yield" && this.inGeneratorContext()) && (t = true), this.exprAllowed = t;
};
var ye = Ue.prototype;
ye.checkPropClash = function(e, t, a) {
  if (!(this.options.ecmaVersion >= 9 && e.type === "SpreadElement") && !(this.options.ecmaVersion >= 6 && (e.computed || e.method || e.shorthand))) {
    var s = e.key, u;
    switch (s.type) {
      case "Identifier":
        u = s.name;
        break;
      case "Literal":
        u = String(s.value);
        break;
      default:
        return;
    }
    var p = e.kind;
    if (this.options.ecmaVersion >= 6) {
      u === "__proto__" && p === "init" && (t.proto && (a ? a.doubleProto < 0 && (a.doubleProto = s.start) : this.raiseRecoverable(s.start, "Redefinition of __proto__ property")), t.proto = true);
      return;
    }
    u = "$" + u;
    var m = t[u];
    if (m) {
      var w;
      p === "init" ? w = this.strict && m.init || m.get || m.set : w = m.init || m[p], w && this.raiseRecoverable(s.start, "Redefinition of property");
    } else m = t[u] = { init: false, get: false, set: false };
    m[p] = true;
  }
};
ye.parseExpression = function(e, t) {
  var a = this.start, s = this.startLoc, u = this.parseMaybeAssign(e, t);
  if (this.type === d.comma) {
    var p = this.startNodeAt(a, s);
    for (p.expressions = [u]; this.eat(d.comma); ) p.expressions.push(this.parseMaybeAssign(e, t));
    return this.finishNode(p, "SequenceExpression");
  }
  return u;
};
ye.parseMaybeAssign = function(e, t, a) {
  if (this.isContextual("yield")) {
    if (this.inGenerator) return this.parseYield(e);
    this.exprAllowed = false;
  }
  var s = false, u = -1, p = -1, m = -1;
  t ? (u = t.parenthesizedAssign, p = t.trailingComma, m = t.doubleProto, t.parenthesizedAssign = t.trailingComma = -1) : (t = new Zo(), s = true);
  var w = this.start, x = this.startLoc;
  (this.type === d.parenL || this.type === d.name) && (this.potentialArrowAt = this.start, this.potentialArrowInForAwait = e === "await");
  var o = this.parseMaybeConditional(e, t);
  if (a && (o = a.call(this, o, w, x)), this.type.isAssign) {
    var y = this.startNodeAt(w, x);
    return y.operator = this.value, this.type === d.eq && (o = this.toAssignable(o, false, t)), s || (t.parenthesizedAssign = t.trailingComma = t.doubleProto = -1), t.shorthandAssign >= o.start && (t.shorthandAssign = -1), this.type === d.eq ? this.checkLValPattern(o) : this.checkLValSimple(o), y.left = o, this.next(), y.right = this.parseMaybeAssign(e), m > -1 && (t.doubleProto = m), this.finishNode(y, "AssignmentExpression");
  } else s && this.checkExpressionErrors(t, true);
  return u > -1 && (t.parenthesizedAssign = u), p > -1 && (t.trailingComma = p), o;
};
ye.parseMaybeConditional = function(e, t) {
  var a = this.start, s = this.startLoc, u = this.parseExprOps(e, t);
  if (this.checkExpressionErrors(t)) return u;
  if (this.eat(d.question)) {
    var p = this.startNodeAt(a, s);
    return p.test = u, p.consequent = this.parseMaybeAssign(), this.expect(d.colon), p.alternate = this.parseMaybeAssign(e), this.finishNode(p, "ConditionalExpression");
  }
  return u;
};
ye.parseExprOps = function(e, t) {
  var a = this.start, s = this.startLoc, u = this.parseMaybeUnary(t, false, false, e);
  return this.checkExpressionErrors(t) || u.start === a && u.type === "ArrowFunctionExpression" ? u : this.parseExprOp(u, a, s, -1, e);
};
ye.parseExprOp = function(e, t, a, s, u) {
  var p = this.type.binop;
  if (p != null && (!u || this.type !== d._in) && p > s) {
    var m = this.type === d.logicalOR || this.type === d.logicalAND, w = this.type === d.coalesce;
    w && (p = d.logicalAND.binop);
    var x = this.value;
    this.next();
    var o = this.start, y = this.startLoc, O = this.parseExprOp(this.parseMaybeUnary(null, false, false, u), o, y, p, u), L = this.buildBinary(t, a, e, O, x, m || w);
    return (m && this.type === d.coalesce || w && (this.type === d.logicalOR || this.type === d.logicalAND)) && this.raiseRecoverable(this.start, "Logical expressions and coalesce expressions cannot be mixed. Wrap either by parentheses"), this.parseExprOp(L, t, a, s, u);
  }
  return e;
};
ye.buildBinary = function(e, t, a, s, u, p) {
  s.type === "PrivateIdentifier" && this.raise(s.start, "Private identifier can only be left side of binary expression");
  var m = this.startNodeAt(e, t);
  return m.left = a, m.operator = u, m.right = s, this.finishNode(m, p ? "LogicalExpression" : "BinaryExpression");
};
ye.parseMaybeUnary = function(e, t, a, s) {
  var u = this.start, p = this.startLoc, m;
  if (this.isContextual("await") && this.canAwait) m = this.parseAwait(s), t = true;
  else if (this.type.prefix) {
    var w = this.startNode(), x = this.type === d.incDec;
    w.operator = this.value, w.prefix = true, this.next(), w.argument = this.parseMaybeUnary(null, true, x, s), this.checkExpressionErrors(e, true), x ? this.checkLValSimple(w.argument) : this.strict && w.operator === "delete" && Hh(w.argument) ? this.raiseRecoverable(w.start, "Deleting local variable in strict mode") : w.operator === "delete" && bl(w.argument) ? this.raiseRecoverable(w.start, "Private fields can not be deleted") : t = true, m = this.finishNode(w, x ? "UpdateExpression" : "UnaryExpression");
  } else if (!t && this.type === d.privateId) (s || this.privateNameStack.length === 0) && this.options.checkPrivateFields && this.unexpected(), m = this.parsePrivateIdent(), this.type !== d._in && this.unexpected();
  else {
    if (m = this.parseExprSubscripts(e, s), this.checkExpressionErrors(e)) return m;
    for (; this.type.postfix && !this.canInsertSemicolon(); ) {
      var o = this.startNodeAt(u, p);
      o.operator = this.value, o.prefix = false, o.argument = m, this.checkLValSimple(m), this.next(), m = this.finishNode(o, "UpdateExpression");
    }
  }
  if (!a && this.eat(d.starstar)) if (t) this.unexpected(this.lastTokStart);
  else return this.buildBinary(u, p, m, this.parseMaybeUnary(null, false, false, s), "**", false);
  else return m;
};
function Hh(e) {
  return e.type === "Identifier" || e.type === "ParenthesizedExpression" && Hh(e.expression);
}
function bl(e) {
  return e.type === "MemberExpression" && e.property.type === "PrivateIdentifier" || e.type === "ChainExpression" && bl(e.expression) || e.type === "ParenthesizedExpression" && bl(e.expression);
}
ye.parseExprSubscripts = function(e, t) {
  var a = this.start, s = this.startLoc, u = this.parseExprAtom(e, t);
  if (u.type === "ArrowFunctionExpression" && this.input.slice(this.lastTokStart, this.lastTokEnd) !== ")") return u;
  var p = this.parseSubscripts(u, a, s, false, t);
  return e && p.type === "MemberExpression" && (e.parenthesizedAssign >= p.start && (e.parenthesizedAssign = -1), e.parenthesizedBind >= p.start && (e.parenthesizedBind = -1), e.trailingComma >= p.start && (e.trailingComma = -1)), p;
};
ye.parseSubscripts = function(e, t, a, s, u) {
  for (var p = this.options.ecmaVersion >= 8 && e.type === "Identifier" && e.name === "async" && this.lastTokEnd === e.end && !this.canInsertSemicolon() && e.end - e.start === 5 && this.potentialArrowAt === e.start, m = false; ; ) {
    var w = this.parseSubscript(e, t, a, s, p, m, u);
    if (w.optional && (m = true), w === e || w.type === "ArrowFunctionExpression") {
      if (m) {
        var x = this.startNodeAt(t, a);
        x.expression = w, w = this.finishNode(x, "ChainExpression");
      }
      return w;
    }
    e = w;
  }
};
ye.shouldParseAsyncArrow = function() {
  return !this.canInsertSemicolon() && this.eat(d.arrow);
};
ye.parseSubscriptAsyncArrow = function(e, t, a, s) {
  return this.parseArrowExpression(this.startNodeAt(e, t), a, true, s);
};
ye.parseSubscript = function(e, t, a, s, u, p, m) {
  var w = this.options.ecmaVersion >= 11, x = w && this.eat(d.questionDot);
  s && x && this.raise(this.lastTokStart, "Optional chaining cannot appear in the callee of new expressions");
  var o = this.eat(d.bracketL);
  if (o || x && this.type !== d.parenL && this.type !== d.backQuote || this.eat(d.dot)) {
    var y = this.startNodeAt(t, a);
    y.object = e, o ? (y.property = this.parseExpression(), this.expect(d.bracketR)) : this.type === d.privateId && e.type !== "Super" ? y.property = this.parsePrivateIdent() : y.property = this.parseIdent(this.options.allowReserved !== "never"), y.computed = !!o, w && (y.optional = x), e = this.finishNode(y, "MemberExpression");
  } else if (!s && this.eat(d.parenL)) {
    var O = new Zo(), L = this.yieldPos, B = this.awaitPos, G = this.awaitIdentPos;
    this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0;
    var j = this.parseExprList(d.parenR, this.options.ecmaVersion >= 8, false, O);
    if (u && !x && this.shouldParseAsyncArrow()) return this.checkPatternErrors(O, false), this.checkYieldAwaitInDefaultParams(), this.awaitIdentPos > 0 && this.raise(this.awaitIdentPos, "Cannot use 'await' as identifier inside an async function"), this.yieldPos = L, this.awaitPos = B, this.awaitIdentPos = G, this.parseSubscriptAsyncArrow(t, a, j, m);
    this.checkExpressionErrors(O, true), this.yieldPos = L || this.yieldPos, this.awaitPos = B || this.awaitPos, this.awaitIdentPos = G || this.awaitIdentPos;
    var R = this.startNodeAt(t, a);
    R.callee = e, R.arguments = j, w && (R.optional = x), e = this.finishNode(R, "CallExpression");
  } else if (this.type === d.backQuote) {
    (x || p) && this.raise(this.start, "Optional chaining cannot appear in the tag of tagged template expressions");
    var h = this.startNodeAt(t, a);
    h.tag = e, h.quasi = this.parseTemplate({ isTagged: true }), e = this.finishNode(h, "TaggedTemplateExpression");
  }
  return e;
};
ye.parseExprAtom = function(e, t, a) {
  this.type === d.slash && this.readRegexp();
  var s, u = this.potentialArrowAt === this.start;
  switch (this.type) {
    case d._super:
      return this.allowSuper || this.raise(this.start, "'super' keyword outside a method"), s = this.startNode(), this.next(), this.type === d.parenL && !this.allowDirectSuper && this.raise(s.start, "super() call outside constructor of a subclass"), this.type !== d.dot && this.type !== d.bracketL && this.type !== d.parenL && this.unexpected(), this.finishNode(s, "Super");
    case d._this:
      return s = this.startNode(), this.next(), this.finishNode(s, "ThisExpression");
    case d.name:
      var p = this.start, m = this.startLoc, w = this.containsEsc, x = this.parseIdent(false);
      if (this.options.ecmaVersion >= 8 && !w && x.name === "async" && !this.canInsertSemicolon() && this.eat(d._function)) return this.overrideContext(Ve.f_expr), this.parseFunction(this.startNodeAt(p, m), 0, false, true, t);
      if (u && !this.canInsertSemicolon()) {
        if (this.eat(d.arrow)) return this.parseArrowExpression(this.startNodeAt(p, m), [x], false, t);
        if (this.options.ecmaVersion >= 8 && x.name === "async" && this.type === d.name && !w && (!this.potentialArrowInForAwait || this.value !== "of" || this.containsEsc)) return x = this.parseIdent(false), (this.canInsertSemicolon() || !this.eat(d.arrow)) && this.unexpected(), this.parseArrowExpression(this.startNodeAt(p, m), [x], true, t);
      }
      return x;
    case d.regexp:
      var o = this.value;
      return s = this.parseLiteral(o.value), s.regex = { pattern: o.pattern, flags: o.flags }, s;
    case d.num:
    case d.string:
      return this.parseLiteral(this.value);
    case d._null:
    case d._true:
    case d._false:
      return s = this.startNode(), s.value = this.type === d._null ? null : this.type === d._true, s.raw = this.type.keyword, this.next(), this.finishNode(s, "Literal");
    case d.parenL:
      var y = this.start, O = this.parseParenAndDistinguishExpression(u, t);
      return e && (e.parenthesizedAssign < 0 && !this.isSimpleAssignTarget(O) && (e.parenthesizedAssign = y), e.parenthesizedBind < 0 && (e.parenthesizedBind = y)), O;
    case d.bracketL:
      return s = this.startNode(), this.next(), s.elements = this.parseExprList(d.bracketR, true, true, e), this.finishNode(s, "ArrayExpression");
    case d.braceL:
      return this.overrideContext(Ve.b_expr), this.parseObj(false, e);
    case d._function:
      return s = this.startNode(), this.next(), this.parseFunction(s, 0);
    case d._class:
      return this.parseClass(this.startNode(), false);
    case d._new:
      return this.parseNew();
    case d.backQuote:
      return this.parseTemplate();
    case d._import:
      return this.options.ecmaVersion >= 11 ? this.parseExprImport(a) : this.unexpected();
    default:
      return this.parseExprAtomDefault();
  }
};
ye.parseExprAtomDefault = function() {
  this.unexpected();
};
ye.parseExprImport = function(e) {
  var t = this.startNode();
  if (this.containsEsc && this.raiseRecoverable(this.start, "Escape sequence in keyword import"), this.next(), this.type === d.parenL && !e) return this.parseDynamicImport(t);
  if (this.type === d.dot) {
    var a = this.startNodeAt(t.start, t.loc && t.loc.start);
    return a.name = "import", t.meta = this.finishNode(a, "Identifier"), this.parseImportMeta(t);
  } else this.unexpected();
};
ye.parseDynamicImport = function(e) {
  if (this.next(), e.source = this.parseMaybeAssign(), this.options.ecmaVersion >= 16) this.eat(d.parenR) ? e.options = null : (this.expect(d.comma), this.afterTrailingComma(d.parenR) ? e.options = null : (e.options = this.parseMaybeAssign(), this.eat(d.parenR) || (this.expect(d.comma), this.afterTrailingComma(d.parenR) || this.unexpected())));
  else if (!this.eat(d.parenR)) {
    var t = this.start;
    this.eat(d.comma) && this.eat(d.parenR) ? this.raiseRecoverable(t, "Trailing comma is not allowed in import()") : this.unexpected(t);
  }
  return this.finishNode(e, "ImportExpression");
};
ye.parseImportMeta = function(e) {
  this.next();
  var t = this.containsEsc;
  return e.property = this.parseIdent(true), e.property.name !== "meta" && this.raiseRecoverable(e.property.start, "The only valid meta property for import is 'import.meta'"), t && this.raiseRecoverable(e.start, "'import.meta' must not contain escaped characters"), this.options.sourceType !== "module" && !this.options.allowImportExportEverywhere && this.raiseRecoverable(e.start, "Cannot use 'import.meta' outside a module"), this.finishNode(e, "MetaProperty");
};
ye.parseLiteral = function(e) {
  var t = this.startNode();
  return t.value = e, t.raw = this.input.slice(this.start, this.end), t.raw.charCodeAt(t.raw.length - 1) === 110 && (t.bigint = t.raw.slice(0, -1).replace(/_/g, "")), this.next(), this.finishNode(t, "Literal");
};
ye.parseParenExpression = function() {
  this.expect(d.parenL);
  var e = this.parseExpression();
  return this.expect(d.parenR), e;
};
ye.shouldParseArrow = function(e) {
  return !this.canInsertSemicolon();
};
ye.parseParenAndDistinguishExpression = function(e, t) {
  var a = this.start, s = this.startLoc, u, p = this.options.ecmaVersion >= 8;
  if (this.options.ecmaVersion >= 6) {
    this.next();
    var m = this.start, w = this.startLoc, x = [], o = true, y = false, O = new Zo(), L = this.yieldPos, B = this.awaitPos, G;
    for (this.yieldPos = 0, this.awaitPos = 0; this.type !== d.parenR; ) if (o ? o = false : this.expect(d.comma), p && this.afterTrailingComma(d.parenR, true)) {
      y = true;
      break;
    } else if (this.type === d.ellipsis) {
      G = this.start, x.push(this.parseParenItem(this.parseRestBinding())), this.type === d.comma && this.raiseRecoverable(this.start, "Comma is not permitted after the rest element");
      break;
    } else x.push(this.parseMaybeAssign(false, O, this.parseParenItem));
    var j = this.lastTokEnd, R = this.lastTokEndLoc;
    if (this.expect(d.parenR), e && this.shouldParseArrow(x) && this.eat(d.arrow)) return this.checkPatternErrors(O, false), this.checkYieldAwaitInDefaultParams(), this.yieldPos = L, this.awaitPos = B, this.parseParenArrowList(a, s, x, t);
    (!x.length || y) && this.unexpected(this.lastTokStart), G && this.unexpected(G), this.checkExpressionErrors(O, true), this.yieldPos = L || this.yieldPos, this.awaitPos = B || this.awaitPos, x.length > 1 ? (u = this.startNodeAt(m, w), u.expressions = x, this.finishNodeAt(u, "SequenceExpression", j, R)) : u = x[0];
  } else u = this.parseParenExpression();
  if (this.options.preserveParens) {
    var h = this.startNodeAt(a, s);
    return h.expression = u, this.finishNode(h, "ParenthesizedExpression");
  } else return u;
};
ye.parseParenItem = function(e) {
  return e;
};
ye.parseParenArrowList = function(e, t, a, s) {
  return this.parseArrowExpression(this.startNodeAt(e, t), a, false, s);
};
var p1 = [];
ye.parseNew = function() {
  this.containsEsc && this.raiseRecoverable(this.start, "Escape sequence in keyword new");
  var e = this.startNode();
  if (this.next(), this.options.ecmaVersion >= 6 && this.type === d.dot) {
    var t = this.startNodeAt(e.start, e.loc && e.loc.start);
    t.name = "new", e.meta = this.finishNode(t, "Identifier"), this.next();
    var a = this.containsEsc;
    return e.property = this.parseIdent(true), e.property.name !== "target" && this.raiseRecoverable(e.property.start, "The only valid meta property for new is 'new.target'"), a && this.raiseRecoverable(e.start, "'new.target' must not contain escaped characters"), this.allowNewDotTarget || this.raiseRecoverable(e.start, "'new.target' can only be used in functions and class static block"), this.finishNode(e, "MetaProperty");
  }
  var s = this.start, u = this.startLoc;
  return e.callee = this.parseSubscripts(this.parseExprAtom(null, false, true), s, u, true, false), this.eat(d.parenL) ? e.arguments = this.parseExprList(d.parenR, this.options.ecmaVersion >= 8, false) : e.arguments = p1, this.finishNode(e, "NewExpression");
};
ye.parseTemplateElement = function(e) {
  var t = e.isTagged, a = this.startNode();
  return this.type === d.invalidTemplate ? (t || this.raiseRecoverable(this.start, "Bad escape sequence in untagged template literal"), a.value = { raw: this.value.replace(/\r\n?/g, `
`), cooked: null }) : a.value = { raw: this.input.slice(this.start, this.end).replace(/\r\n?/g, `
`), cooked: this.value }, this.next(), a.tail = this.type === d.backQuote, this.finishNode(a, "TemplateElement");
};
ye.parseTemplate = function(e) {
  e === void 0 && (e = {});
  var t = e.isTagged;
  t === void 0 && (t = false);
  var a = this.startNode();
  this.next(), a.expressions = [];
  var s = this.parseTemplateElement({ isTagged: t });
  for (a.quasis = [s]; !s.tail; ) this.type === d.eof && this.raise(this.pos, "Unterminated template literal"), this.expect(d.dollarBraceL), a.expressions.push(this.parseExpression()), this.expect(d.braceR), a.quasis.push(s = this.parseTemplateElement({ isTagged: t }));
  return this.next(), this.finishNode(a, "TemplateLiteral");
};
ye.isAsyncProp = function(e) {
  return !e.computed && e.key.type === "Identifier" && e.key.name === "async" && (this.type === d.name || this.type === d.num || this.type === d.string || this.type === d.bracketL || this.type.keyword || this.options.ecmaVersion >= 9 && this.type === d.star) && !dt.test(this.input.slice(this.lastTokEnd, this.start));
};
ye.parseObj = function(e, t) {
  var a = this.startNode(), s = true, u = {};
  for (a.properties = [], this.next(); !this.eat(d.braceR); ) {
    if (s) s = false;
    else if (this.expect(d.comma), this.options.ecmaVersion >= 5 && this.afterTrailingComma(d.braceR)) break;
    var p = this.parseProperty(e, t);
    e || this.checkPropClash(p, u, t), a.properties.push(p);
  }
  return this.finishNode(a, e ? "ObjectPattern" : "ObjectExpression");
};
ye.parseProperty = function(e, t) {
  var a = this.startNode(), s, u, p, m;
  if (this.options.ecmaVersion >= 9 && this.eat(d.ellipsis)) return e ? (a.argument = this.parseIdent(false), this.type === d.comma && this.raiseRecoverable(this.start, "Comma is not permitted after the rest element"), this.finishNode(a, "RestElement")) : (a.argument = this.parseMaybeAssign(false, t), this.type === d.comma && t && t.trailingComma < 0 && (t.trailingComma = this.start), this.finishNode(a, "SpreadElement"));
  this.options.ecmaVersion >= 6 && (a.method = false, a.shorthand = false, (e || t) && (p = this.start, m = this.startLoc), e || (s = this.eat(d.star)));
  var w = this.containsEsc;
  return this.parsePropertyName(a), !e && !w && this.options.ecmaVersion >= 8 && !s && this.isAsyncProp(a) ? (u = true, s = this.options.ecmaVersion >= 9 && this.eat(d.star), this.parsePropertyName(a)) : u = false, this.parsePropertyValue(a, e, s, u, p, m, t, w), this.finishNode(a, "Property");
};
ye.parseGetterSetter = function(e) {
  e.kind = e.key.name, this.parsePropertyName(e), e.value = this.parseMethod(false);
  var t = e.kind === "get" ? 0 : 1;
  if (e.value.params.length !== t) {
    var a = e.value.start;
    e.kind === "get" ? this.raiseRecoverable(a, "getter should have no params") : this.raiseRecoverable(a, "setter should have exactly one param");
  } else e.kind === "set" && e.value.params[0].type === "RestElement" && this.raiseRecoverable(e.value.params[0].start, "Setter cannot use rest params");
};
ye.parsePropertyValue = function(e, t, a, s, u, p, m, w) {
  (a || s) && this.type === d.colon && this.unexpected(), this.eat(d.colon) ? (e.value = t ? this.parseMaybeDefault(this.start, this.startLoc) : this.parseMaybeAssign(false, m), e.kind = "init") : this.options.ecmaVersion >= 6 && this.type === d.parenL ? (t && this.unexpected(), e.kind = "init", e.method = true, e.value = this.parseMethod(a, s)) : !t && !w && this.options.ecmaVersion >= 5 && !e.computed && e.key.type === "Identifier" && (e.key.name === "get" || e.key.name === "set") && this.type !== d.comma && this.type !== d.braceR && this.type !== d.eq ? ((a || s) && this.unexpected(), this.parseGetterSetter(e)) : this.options.ecmaVersion >= 6 && !e.computed && e.key.type === "Identifier" ? ((a || s) && this.unexpected(), this.checkUnreserved(e.key), e.key.name === "await" && !this.awaitIdentPos && (this.awaitIdentPos = u), e.kind = "init", t ? e.value = this.parseMaybeDefault(u, p, this.copyNode(e.key)) : this.type === d.eq && m ? (m.shorthandAssign < 0 && (m.shorthandAssign = this.start), e.value = this.parseMaybeDefault(u, p, this.copyNode(e.key))) : e.value = this.copyNode(e.key), e.shorthand = true) : this.unexpected();
};
ye.parsePropertyName = function(e) {
  if (this.options.ecmaVersion >= 6) {
    if (this.eat(d.bracketL)) return e.computed = true, e.key = this.parseMaybeAssign(), this.expect(d.bracketR), e.key;
    e.computed = false;
  }
  return e.key = this.type === d.num || this.type === d.string ? this.parseExprAtom() : this.parseIdent(this.options.allowReserved !== "never");
};
ye.initFunction = function(e) {
  e.id = null, this.options.ecmaVersion >= 6 && (e.generator = e.expression = false), this.options.ecmaVersion >= 8 && (e.async = false);
};
ye.parseMethod = function(e, t, a) {
  var s = this.startNode(), u = this.yieldPos, p = this.awaitPos, m = this.awaitIdentPos;
  return this.initFunction(s), this.options.ecmaVersion >= 6 && (s.generator = e), this.options.ecmaVersion >= 8 && (s.async = !!t), this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0, this.enterScope(wl(t, s.generator) | Tl | (a ? jh : 0)), this.expect(d.parenL), s.params = this.parseBindingList(d.parenR, false, this.options.ecmaVersion >= 8), this.checkYieldAwaitInDefaultParams(), this.parseFunctionBody(s, false, true, false), this.yieldPos = u, this.awaitPos = p, this.awaitIdentPos = m, this.finishNode(s, "FunctionExpression");
};
ye.parseArrowExpression = function(e, t, a, s) {
  var u = this.yieldPos, p = this.awaitPos, m = this.awaitIdentPos;
  return this.enterScope(wl(a, false) | Nh), this.initFunction(e), this.options.ecmaVersion >= 8 && (e.async = !!a), this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0, e.params = this.toAssignableList(t, true), this.parseFunctionBody(e, true, false, s), this.yieldPos = u, this.awaitPos = p, this.awaitIdentPos = m, this.finishNode(e, "ArrowFunctionExpression");
};
ye.parseFunctionBody = function(e, t, a, s) {
  var u = t && this.type !== d.braceL, p = this.strict, m = false;
  if (u) e.body = this.parseMaybeAssign(s), e.expression = true, this.checkParams(e, false);
  else {
    var w = this.options.ecmaVersion >= 7 && !this.isSimpleParamList(e.params);
    (!p || w) && (m = this.strictDirective(this.end), m && w && this.raiseRecoverable(e.start, "Illegal 'use strict' directive in function with non-simple parameter list"));
    var x = this.labels;
    this.labels = [], m && (this.strict = true), this.checkParams(e, !p && !m && !t && !a && this.isSimpleParamList(e.params)), this.strict && e.id && this.checkLValSimple(e.id, Vh), e.body = this.parseBlock(false, void 0, m && !p), e.expression = false, this.adaptDirectivePrologue(e.body.body), this.labels = x;
  }
  this.exitScope();
};
ye.isSimpleParamList = function(e) {
  for (var t = 0, a = e; t < a.length; t += 1) {
    var s = a[t];
    if (s.type !== "Identifier") return false;
  }
  return true;
};
ye.checkParams = function(e, t) {
  for (var a = /* @__PURE__ */ Object.create(null), s = 0, u = e.params; s < u.length; s += 1) {
    var p = u[s];
    this.checkLValInnerPattern(p, Al, t ? null : a);
  }
};
ye.parseExprList = function(e, t, a, s) {
  for (var u = [], p = true; !this.eat(e); ) {
    if (p) p = false;
    else if (this.expect(d.comma), t && this.afterTrailingComma(e)) break;
    var m = void 0;
    a && this.type === d.comma ? m = null : this.type === d.ellipsis ? (m = this.parseSpread(s), s && this.type === d.comma && s.trailingComma < 0 && (s.trailingComma = this.start)) : m = this.parseMaybeAssign(false, s), u.push(m);
  }
  return u;
};
ye.checkUnreserved = function(e) {
  var t = e.start, a = e.end, s = e.name;
  if (this.inGenerator && s === "yield" && this.raiseRecoverable(t, "Cannot use 'yield' as identifier inside a generator"), this.inAsync && s === "await" && this.raiseRecoverable(t, "Cannot use 'await' as identifier inside an async function"), this.currentThisScope().inClassFieldInit && s === "arguments" && this.raiseRecoverable(t, "Cannot use 'arguments' in class field initializer"), this.inClassStaticBlock && (s === "arguments" || s === "await") && this.raise(t, "Cannot use " + s + " in class static initialization block"), this.keywords.test(s) && this.raise(t, "Unexpected keyword '" + s + "'"), !(this.options.ecmaVersion < 6 && this.input.slice(t, a).indexOf("\\") !== -1)) {
    var u = this.strict ? this.reservedWordsStrict : this.reservedWords;
    u.test(s) && (!this.inAsync && s === "await" && this.raiseRecoverable(t, "Cannot use keyword 'await' outside an async function"), this.raiseRecoverable(t, "The keyword '" + s + "' is reserved"));
  }
};
ye.parseIdent = function(e) {
  var t = this.parseIdentNode();
  return this.next(!!e), this.finishNode(t, "Identifier"), e || (this.checkUnreserved(t), t.name === "await" && !this.awaitIdentPos && (this.awaitIdentPos = t.start)), t;
};
ye.parseIdentNode = function() {
  var e = this.startNode();
  return this.type === d.name ? e.name = this.value : this.type.keyword ? (e.name = this.type.keyword, (e.name === "class" || e.name === "function") && (this.lastTokEnd !== this.lastTokStart + 1 || this.input.charCodeAt(this.lastTokStart) !== 46) && this.context.pop(), this.type = d.name) : this.unexpected(), e;
};
ye.parsePrivateIdent = function() {
  var e = this.startNode();
  return this.type === d.privateId ? e.name = this.value : this.unexpected(), this.next(), this.finishNode(e, "PrivateIdentifier"), this.options.checkPrivateFields && (this.privateNameStack.length === 0 ? this.raise(e.start, "Private field '#" + e.name + "' must be declared in an enclosing class") : this.privateNameStack[this.privateNameStack.length - 1].used.push(e)), e;
};
ye.parseYield = function(e) {
  this.yieldPos || (this.yieldPos = this.start);
  var t = this.startNode();
  return this.next(), this.type === d.semi || this.canInsertSemicolon() || this.type !== d.star && !this.type.startsExpr ? (t.delegate = false, t.argument = null) : (t.delegate = this.eat(d.star), t.argument = this.parseMaybeAssign(e)), this.finishNode(t, "YieldExpression");
};
ye.parseAwait = function(e) {
  this.awaitPos || (this.awaitPos = this.start);
  var t = this.startNode();
  return this.next(), t.argument = this.parseMaybeUnary(null, true, false, e), this.finishNode(t, "AwaitExpression");
};
var Ko = Ue.prototype;
Ko.raise = function(e, t) {
  var a = Pl(this.input, e);
  t += " (" + a.line + ":" + a.column + ")";
  var s = new SyntaxError(t);
  throw s.pos = e, s.loc = a, s.raisedAt = this.pos, s;
};
Ko.raiseRecoverable = Ko.raise;
Ko.curPosition = function() {
  if (this.options.locations) return new Jt(this.curLine, this.pos - this.lineStart);
};
var Bt = Ue.prototype, f1 = function(t) {
  this.flags = t, this.var = [], this.lexical = [], this.functions = [], this.inClassFieldInit = false;
};
Bt.enterScope = function(e) {
  this.scopeStack.push(new f1(e));
};
Bt.exitScope = function() {
  this.scopeStack.pop();
};
Bt.treatFunctionsAsVarInScope = function(e) {
  return e.flags & tr || !this.inModule && e.flags & Co;
};
Bt.declareName = function(e, t, a) {
  var s = false;
  if (t === Ot) {
    var u = this.currentScope();
    s = u.lexical.indexOf(e) > -1 || u.functions.indexOf(e) > -1 || u.var.indexOf(e) > -1, u.lexical.push(e), this.inModule && u.flags & Co && delete this.undefinedExports[e];
  } else if (t === Fh) {
    var p = this.currentScope();
    p.lexical.push(e);
  } else if (t === Bh) {
    var m = this.currentScope();
    this.treatFunctionsAsVar ? s = m.lexical.indexOf(e) > -1 : s = m.lexical.indexOf(e) > -1 || m.var.indexOf(e) > -1, m.functions.push(e);
  } else for (var w = this.scopeStack.length - 1; w >= 0; --w) {
    var x = this.scopeStack[w];
    if (x.lexical.indexOf(e) > -1 && !(x.flags & Dh && x.lexical[0] === e) || !this.treatFunctionsAsVarInScope(x) && x.functions.indexOf(e) > -1) {
      s = true;
      break;
    }
    if (x.var.push(e), this.inModule && x.flags & Co && delete this.undefinedExports[e], x.flags & Sl) break;
  }
  s && this.raiseRecoverable(a, "Identifier '" + e + "' has already been declared");
};
Bt.checkLocalExport = function(e) {
  this.scopeStack[0].lexical.indexOf(e.name) === -1 && this.scopeStack[0].var.indexOf(e.name) === -1 && (this.undefinedExports[e.name] = e);
};
Bt.currentScope = function() {
  return this.scopeStack[this.scopeStack.length - 1];
};
Bt.currentVarScope = function() {
  for (var e = this.scopeStack.length - 1; ; e--) {
    var t = this.scopeStack[e];
    if (t.flags & Sl) return t;
  }
};
Bt.currentThisScope = function() {
  for (var e = this.scopeStack.length - 1; ; e--) {
    var t = this.scopeStack[e];
    if (t.flags & Sl && !(t.flags & Nh)) return t;
  }
};
var To = function(t, a, s) {
  this.type = "", this.start = a, this.end = 0, t.options.locations && (this.loc = new Po(t, s)), t.options.directSourceFile && (this.sourceFile = t.options.directSourceFile), t.options.ranges && (this.range = [a, 0]);
}, So = Ue.prototype;
So.startNode = function() {
  return new To(this, this.start, this.startLoc);
};
So.startNodeAt = function(e, t) {
  return new To(this, e, t);
};
function Uh(e, t, a, s) {
  return e.type = t, e.end = a, this.options.locations && (e.loc.end = s), this.options.ranges && (e.range[1] = a), e;
}
So.finishNode = function(e, t) {
  return Uh.call(this, e, t, this.lastTokEnd, this.lastTokEndLoc);
};
So.finishNodeAt = function(e, t, a, s) {
  return Uh.call(this, e, t, a, s);
};
So.copyNode = function(e) {
  var t = new To(this, e.start, this.startLoc);
  for (var a in e) t[a] = e[a];
  return t;
};
var h1 = "Gara Garay Gukh Gurung_Khema Hrkt Katakana_Or_Hiragana Kawi Kirat_Rai Krai Nag_Mundari Nagm Ol_Onal Onao Sunu Sunuwar Todhri Todr Tulu_Tigalari Tutg Unknown Zzzz", Gh = "ASCII ASCII_Hex_Digit AHex Alphabetic Alpha Any Assigned Bidi_Control Bidi_C Bidi_Mirrored Bidi_M Case_Ignorable CI Cased Changes_When_Casefolded CWCF Changes_When_Casemapped CWCM Changes_When_Lowercased CWL Changes_When_NFKC_Casefolded CWKCF Changes_When_Titlecased CWT Changes_When_Uppercased CWU Dash Default_Ignorable_Code_Point DI Deprecated Dep Diacritic Dia Emoji Emoji_Component Emoji_Modifier Emoji_Modifier_Base Emoji_Presentation Extender Ext Grapheme_Base Gr_Base Grapheme_Extend Gr_Ext Hex_Digit Hex IDS_Binary_Operator IDSB IDS_Trinary_Operator IDST ID_Continue IDC ID_Start IDS Ideographic Ideo Join_Control Join_C Logical_Order_Exception LOE Lowercase Lower Math Noncharacter_Code_Point NChar Pattern_Syntax Pat_Syn Pattern_White_Space Pat_WS Quotation_Mark QMark Radical Regional_Indicator RI Sentence_Terminal STerm Soft_Dotted SD Terminal_Punctuation Term Unified_Ideograph UIdeo Uppercase Upper Variation_Selector VS White_Space space XID_Continue XIDC XID_Start XIDS", zh = Gh + " Extended_Pictographic", Wh = zh, Xh = Wh + " EBase EComp EMod EPres ExtPict", Kh = Xh, m1 = Kh, v1 = { 9: Gh, 10: zh, 11: Wh, 12: Xh, 13: Kh, 14: m1 }, b1 = "Basic_Emoji Emoji_Keycap_Sequence RGI_Emoji_Modifier_Sequence RGI_Emoji_Flag_Sequence RGI_Emoji_Tag_Sequence RGI_Emoji_ZWJ_Sequence RGI_Emoji", y1 = { 9: "", 10: "", 11: "", 12: "", 13: "", 14: b1 }, Gl = "Cased_Letter LC Close_Punctuation Pe Connector_Punctuation Pc Control Cc cntrl Currency_Symbol Sc Dash_Punctuation Pd Decimal_Number Nd digit Enclosing_Mark Me Final_Punctuation Pf Format Cf Initial_Punctuation Pi Letter L Letter_Number Nl Line_Separator Zl Lowercase_Letter Ll Mark M Combining_Mark Math_Symbol Sm Modifier_Letter Lm Modifier_Symbol Sk Nonspacing_Mark Mn Number N Open_Punctuation Ps Other C Other_Letter Lo Other_Number No Other_Punctuation Po Other_Symbol So Paragraph_Separator Zp Private_Use Co Punctuation P punct Separator Z Space_Separator Zs Spacing_Mark Mc Surrogate Cs Symbol S Titlecase_Letter Lt Unassigned Cn Uppercase_Letter Lu", Jh = "Adlam Adlm Ahom Anatolian_Hieroglyphs Hluw Arabic Arab Armenian Armn Avestan Avst Balinese Bali Bamum Bamu Bassa_Vah Bass Batak Batk Bengali Beng Bhaiksuki Bhks Bopomofo Bopo Brahmi Brah Braille Brai Buginese Bugi Buhid Buhd Canadian_Aboriginal Cans Carian Cari Caucasian_Albanian Aghb Chakma Cakm Cham Cham Cherokee Cher Common Zyyy Coptic Copt Qaac Cuneiform Xsux Cypriot Cprt Cyrillic Cyrl Deseret Dsrt Devanagari Deva Duployan Dupl Egyptian_Hieroglyphs Egyp Elbasan Elba Ethiopic Ethi Georgian Geor Glagolitic Glag Gothic Goth Grantha Gran Greek Grek Gujarati Gujr Gurmukhi Guru Han Hani Hangul Hang Hanunoo Hano Hatran Hatr Hebrew Hebr Hiragana Hira Imperial_Aramaic Armi Inherited Zinh Qaai Inscriptional_Pahlavi Phli Inscriptional_Parthian Prti Javanese Java Kaithi Kthi Kannada Knda Katakana Kana Kayah_Li Kali Kharoshthi Khar Khmer Khmr Khojki Khoj Khudawadi Sind Lao Laoo Latin Latn Lepcha Lepc Limbu Limb Linear_A Lina Linear_B Linb Lisu Lisu Lycian Lyci Lydian Lydi Mahajani Mahj Malayalam Mlym Mandaic Mand Manichaean Mani Marchen Marc Masaram_Gondi Gonm Meetei_Mayek Mtei Mende_Kikakui Mend Meroitic_Cursive Merc Meroitic_Hieroglyphs Mero Miao Plrd Modi Mongolian Mong Mro Mroo Multani Mult Myanmar Mymr Nabataean Nbat New_Tai_Lue Talu Newa Newa Nko Nkoo Nushu Nshu Ogham Ogam Ol_Chiki Olck Old_Hungarian Hung Old_Italic Ital Old_North_Arabian Narb Old_Permic Perm Old_Persian Xpeo Old_South_Arabian Sarb Old_Turkic Orkh Oriya Orya Osage Osge Osmanya Osma Pahawh_Hmong Hmng Palmyrene Palm Pau_Cin_Hau Pauc Phags_Pa Phag Phoenician Phnx Psalter_Pahlavi Phlp Rejang Rjng Runic Runr Samaritan Samr Saurashtra Saur Sharada Shrd Shavian Shaw Siddham Sidd SignWriting Sgnw Sinhala Sinh Sora_Sompeng Sora Soyombo Soyo Sundanese Sund Syloti_Nagri Sylo Syriac Syrc Tagalog Tglg Tagbanwa Tagb Tai_Le Tale Tai_Tham Lana Tai_Viet Tavt Takri Takr Tamil Taml Tangut Tang Telugu Telu Thaana Thaa Thai Thai Tibetan Tibt Tifinagh Tfng Tirhuta Tirh Ugaritic Ugar Vai Vaii Warang_Citi Wara Yi Yiii Zanabazar_Square Zanb", Qh = Jh + " Dogra Dogr Gunjala_Gondi Gong Hanifi_Rohingya Rohg Makasar Maka Medefaidrin Medf Old_Sogdian Sogo Sogdian Sogd", Yh = Qh + " Elymaic Elym Nandinagari Nand Nyiakeng_Puachue_Hmong Hmnp Wancho Wcho", Zh = Yh + " Chorasmian Chrs Diak Dives_Akuru Khitan_Small_Script Kits Yezi Yezidi", em = Zh + " Cypro_Minoan Cpmn Old_Uyghur Ougr Tangsa Tnsa Toto Vithkuqi Vith", g1 = em + " " + h1, R1 = { 9: Jh, 10: Qh, 11: Yh, 12: Zh, 13: em, 14: g1 }, tm = {};
function x1(e) {
  var t = tm[e] = { binary: Nt(v1[e] + " " + Gl), binaryOfStrings: Nt(y1[e]), nonBinary: { General_Category: Nt(Gl), Script: Nt(R1[e]) } };
  t.nonBinary.Script_Extensions = t.nonBinary.Script, t.nonBinary.gc = t.nonBinary.General_Category, t.nonBinary.sc = t.nonBinary.Script, t.nonBinary.scx = t.nonBinary.Script_Extensions;
}
for (var cl = 0, zl = [9, 10, 11, 12, 13, 14]; cl < zl.length; cl += 1) {
  var _1 = zl[cl];
  x1(_1);
}
var oe = Ue.prototype, Jo = function(t, a) {
  this.parent = t, this.base = a || this;
};
Jo.prototype.separatedFrom = function(t) {
  for (var a = this; a; a = a.parent) for (var s = t; s; s = s.parent) if (a.base === s.base && a !== s) return true;
  return false;
};
Jo.prototype.sibling = function() {
  return new Jo(this.parent, this.base);
};
var wt = function(t) {
  this.parser = t, this.validFlags = "gim" + (t.options.ecmaVersion >= 6 ? "uy" : "") + (t.options.ecmaVersion >= 9 ? "s" : "") + (t.options.ecmaVersion >= 13 ? "d" : "") + (t.options.ecmaVersion >= 15 ? "v" : ""), this.unicodeProperties = tm[t.options.ecmaVersion >= 14 ? 14 : t.options.ecmaVersion], this.source = "", this.flags = "", this.start = 0, this.switchU = false, this.switchV = false, this.switchN = false, this.pos = 0, this.lastIntValue = 0, this.lastStringValue = "", this.lastAssertionIsQuantifiable = false, this.numCapturingParens = 0, this.maxBackReference = 0, this.groupNames = /* @__PURE__ */ Object.create(null), this.backReferenceNames = [], this.branchID = null;
};
wt.prototype.reset = function(t, a, s) {
  var u = s.indexOf("v") !== -1, p = s.indexOf("u") !== -1;
  this.start = t | 0, this.source = a + "", this.flags = s, u && this.parser.options.ecmaVersion >= 15 ? (this.switchU = true, this.switchV = true, this.switchN = true) : (this.switchU = p && this.parser.options.ecmaVersion >= 6, this.switchV = false, this.switchN = p && this.parser.options.ecmaVersion >= 9);
};
wt.prototype.raise = function(t) {
  this.parser.raiseRecoverable(this.start, "Invalid regular expression: /" + this.source + "/: " + t);
};
wt.prototype.at = function(t, a) {
  a === void 0 && (a = false);
  var s = this.source, u = s.length;
  if (t >= u) return -1;
  var p = s.charCodeAt(t);
  if (!(a || this.switchU) || p <= 55295 || p >= 57344 || t + 1 >= u) return p;
  var m = s.charCodeAt(t + 1);
  return m >= 56320 && m <= 57343 ? (p << 10) + m - 56613888 : p;
};
wt.prototype.nextIndex = function(t, a) {
  a === void 0 && (a = false);
  var s = this.source, u = s.length;
  if (t >= u) return u;
  var p = s.charCodeAt(t), m;
  return !(a || this.switchU) || p <= 55295 || p >= 57344 || t + 1 >= u || (m = s.charCodeAt(t + 1)) < 56320 || m > 57343 ? t + 1 : t + 2;
};
wt.prototype.current = function(t) {
  return t === void 0 && (t = false), this.at(this.pos, t);
};
wt.prototype.lookahead = function(t) {
  return t === void 0 && (t = false), this.at(this.nextIndex(this.pos, t), t);
};
wt.prototype.advance = function(t) {
  t === void 0 && (t = false), this.pos = this.nextIndex(this.pos, t);
};
wt.prototype.eat = function(t, a) {
  return a === void 0 && (a = false), this.current(a) === t ? (this.advance(a), true) : false;
};
wt.prototype.eatChars = function(t, a) {
  a === void 0 && (a = false);
  for (var s = this.pos, u = 0, p = t; u < p.length; u += 1) {
    var m = p[u], w = this.at(s, a);
    if (w === -1 || w !== m) return false;
    s = this.nextIndex(s, a);
  }
  return this.pos = s, true;
};
oe.validateRegExpFlags = function(e) {
  for (var t = e.validFlags, a = e.flags, s = false, u = false, p = 0; p < a.length; p++) {
    var m = a.charAt(p);
    t.indexOf(m) === -1 && this.raise(e.start, "Invalid regular expression flag"), a.indexOf(m, p + 1) > -1 && this.raise(e.start, "Duplicate regular expression flag"), m === "u" && (s = true), m === "v" && (u = true);
  }
  this.options.ecmaVersion >= 15 && s && u && this.raise(e.start, "Invalid regular expression flag");
};
function C1(e) {
  for (var t in e) return true;
  return false;
}
oe.validateRegExpPattern = function(e) {
  this.regexp_pattern(e), !e.switchN && this.options.ecmaVersion >= 9 && C1(e.groupNames) && (e.switchN = true, this.regexp_pattern(e));
};
oe.regexp_pattern = function(e) {
  e.pos = 0, e.lastIntValue = 0, e.lastStringValue = "", e.lastAssertionIsQuantifiable = false, e.numCapturingParens = 0, e.maxBackReference = 0, e.groupNames = /* @__PURE__ */ Object.create(null), e.backReferenceNames.length = 0, e.branchID = null, this.regexp_disjunction(e), e.pos !== e.source.length && (e.eat(41) && e.raise("Unmatched ')'"), (e.eat(93) || e.eat(125)) && e.raise("Lone quantifier brackets")), e.maxBackReference > e.numCapturingParens && e.raise("Invalid escape");
  for (var t = 0, a = e.backReferenceNames; t < a.length; t += 1) {
    var s = a[t];
    e.groupNames[s] || e.raise("Invalid named capture referenced");
  }
};
oe.regexp_disjunction = function(e) {
  var t = this.options.ecmaVersion >= 16;
  for (t && (e.branchID = new Jo(e.branchID, null)), this.regexp_alternative(e); e.eat(124); ) t && (e.branchID = e.branchID.sibling()), this.regexp_alternative(e);
  t && (e.branchID = e.branchID.parent), this.regexp_eatQuantifier(e, true) && e.raise("Nothing to repeat"), e.eat(123) && e.raise("Lone quantifier brackets");
};
oe.regexp_alternative = function(e) {
  for (; e.pos < e.source.length && this.regexp_eatTerm(e); ) ;
};
oe.regexp_eatTerm = function(e) {
  return this.regexp_eatAssertion(e) ? (e.lastAssertionIsQuantifiable && this.regexp_eatQuantifier(e) && e.switchU && e.raise("Invalid quantifier"), true) : (e.switchU ? this.regexp_eatAtom(e) : this.regexp_eatExtendedAtom(e)) ? (this.regexp_eatQuantifier(e), true) : false;
};
oe.regexp_eatAssertion = function(e) {
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
oe.regexp_eatQuantifier = function(e, t) {
  return t === void 0 && (t = false), this.regexp_eatQuantifierPrefix(e, t) ? (e.eat(63), true) : false;
};
oe.regexp_eatQuantifierPrefix = function(e, t) {
  return e.eat(42) || e.eat(43) || e.eat(63) || this.regexp_eatBracedQuantifier(e, t);
};
oe.regexp_eatBracedQuantifier = function(e, t) {
  var a = e.pos;
  if (e.eat(123)) {
    var s = 0, u = -1;
    if (this.regexp_eatDecimalDigits(e) && (s = e.lastIntValue, e.eat(44) && this.regexp_eatDecimalDigits(e) && (u = e.lastIntValue), e.eat(125))) return u !== -1 && u < s && !t && e.raise("numbers out of order in {} quantifier"), true;
    e.switchU && !t && e.raise("Incomplete quantifier"), e.pos = a;
  }
  return false;
};
oe.regexp_eatAtom = function(e) {
  return this.regexp_eatPatternCharacters(e) || e.eat(46) || this.regexp_eatReverseSolidusAtomEscape(e) || this.regexp_eatCharacterClass(e) || this.regexp_eatUncapturingGroup(e) || this.regexp_eatCapturingGroup(e);
};
oe.regexp_eatReverseSolidusAtomEscape = function(e) {
  var t = e.pos;
  if (e.eat(92)) {
    if (this.regexp_eatAtomEscape(e)) return true;
    e.pos = t;
  }
  return false;
};
oe.regexp_eatUncapturingGroup = function(e) {
  var t = e.pos;
  if (e.eat(40)) {
    if (e.eat(63)) {
      if (this.options.ecmaVersion >= 16) {
        var a = this.regexp_eatModifiers(e), s = e.eat(45);
        if (a || s) {
          for (var u = 0; u < a.length; u++) {
            var p = a.charAt(u);
            a.indexOf(p, u + 1) > -1 && e.raise("Duplicate regular expression modifiers");
          }
          if (s) {
            var m = this.regexp_eatModifiers(e);
            !a && !m && e.current() === 58 && e.raise("Invalid regular expression modifiers");
            for (var w = 0; w < m.length; w++) {
              var x = m.charAt(w);
              (m.indexOf(x, w + 1) > -1 || a.indexOf(x) > -1) && e.raise("Duplicate regular expression modifiers");
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
oe.regexp_eatCapturingGroup = function(e) {
  if (e.eat(40)) {
    if (this.options.ecmaVersion >= 9 ? this.regexp_groupSpecifier(e) : e.current() === 63 && e.raise("Invalid group"), this.regexp_disjunction(e), e.eat(41)) return e.numCapturingParens += 1, true;
    e.raise("Unterminated group");
  }
  return false;
};
oe.regexp_eatModifiers = function(e) {
  for (var t = "", a = 0; (a = e.current()) !== -1 && P1(a); ) t += kt(a), e.advance();
  return t;
};
function P1(e) {
  return e === 105 || e === 109 || e === 115;
}
oe.regexp_eatExtendedAtom = function(e) {
  return e.eat(46) || this.regexp_eatReverseSolidusAtomEscape(e) || this.regexp_eatCharacterClass(e) || this.regexp_eatUncapturingGroup(e) || this.regexp_eatCapturingGroup(e) || this.regexp_eatInvalidBracedQuantifier(e) || this.regexp_eatExtendedPatternCharacter(e);
};
oe.regexp_eatInvalidBracedQuantifier = function(e) {
  return this.regexp_eatBracedQuantifier(e, true) && e.raise("Nothing to repeat"), false;
};
oe.regexp_eatSyntaxCharacter = function(e) {
  var t = e.current();
  return rm(t) ? (e.lastIntValue = t, e.advance(), true) : false;
};
function rm(e) {
  return e === 36 || e >= 40 && e <= 43 || e === 46 || e === 63 || e >= 91 && e <= 94 || e >= 123 && e <= 125;
}
oe.regexp_eatPatternCharacters = function(e) {
  for (var t = e.pos, a = 0; (a = e.current()) !== -1 && !rm(a); ) e.advance();
  return e.pos !== t;
};
oe.regexp_eatExtendedPatternCharacter = function(e) {
  var t = e.current();
  return t !== -1 && t !== 36 && !(t >= 40 && t <= 43) && t !== 46 && t !== 63 && t !== 91 && t !== 94 && t !== 124 ? (e.advance(), true) : false;
};
oe.regexp_groupSpecifier = function(e) {
  if (e.eat(63)) {
    this.regexp_eatGroupName(e) || e.raise("Invalid group");
    var t = this.options.ecmaVersion >= 16, a = e.groupNames[e.lastStringValue];
    if (a) if (t) for (var s = 0, u = a; s < u.length; s += 1) {
      var p = u[s];
      p.separatedFrom(e.branchID) || e.raise("Duplicate capture group name");
    }
    else e.raise("Duplicate capture group name");
    t ? (a || (e.groupNames[e.lastStringValue] = [])).push(e.branchID) : e.groupNames[e.lastStringValue] = true;
  }
};
oe.regexp_eatGroupName = function(e) {
  if (e.lastStringValue = "", e.eat(60)) {
    if (this.regexp_eatRegExpIdentifierName(e) && e.eat(62)) return true;
    e.raise("Invalid capture group name");
  }
  return false;
};
oe.regexp_eatRegExpIdentifierName = function(e) {
  if (e.lastStringValue = "", this.regexp_eatRegExpIdentifierStart(e)) {
    for (e.lastStringValue += kt(e.lastIntValue); this.regexp_eatRegExpIdentifierPart(e); ) e.lastStringValue += kt(e.lastIntValue);
    return true;
  }
  return false;
};
oe.regexp_eatRegExpIdentifierStart = function(e) {
  var t = e.pos, a = this.options.ecmaVersion >= 11, s = e.current(a);
  return e.advance(a), s === 92 && this.regexp_eatRegExpUnicodeEscapeSequence(e, a) && (s = e.lastIntValue), q1(s) ? (e.lastIntValue = s, true) : (e.pos = t, false);
};
function q1(e) {
  return Tt(e, true) || e === 36 || e === 95;
}
oe.regexp_eatRegExpIdentifierPart = function(e) {
  var t = e.pos, a = this.options.ecmaVersion >= 11, s = e.current(a);
  return e.advance(a), s === 92 && this.regexp_eatRegExpUnicodeEscapeSequence(e, a) && (s = e.lastIntValue), T1(s) ? (e.lastIntValue = s, true) : (e.pos = t, false);
};
function T1(e) {
  return Vt(e, true) || e === 36 || e === 95 || e === 8204 || e === 8205;
}
oe.regexp_eatAtomEscape = function(e) {
  return this.regexp_eatBackReference(e) || this.regexp_eatCharacterClassEscape(e) || this.regexp_eatCharacterEscape(e) || e.switchN && this.regexp_eatKGroupName(e) ? true : (e.switchU && (e.current() === 99 && e.raise("Invalid unicode escape"), e.raise("Invalid escape")), false);
};
oe.regexp_eatBackReference = function(e) {
  var t = e.pos;
  if (this.regexp_eatDecimalEscape(e)) {
    var a = e.lastIntValue;
    if (e.switchU) return a > e.maxBackReference && (e.maxBackReference = a), true;
    if (a <= e.numCapturingParens) return true;
    e.pos = t;
  }
  return false;
};
oe.regexp_eatKGroupName = function(e) {
  if (e.eat(107)) {
    if (this.regexp_eatGroupName(e)) return e.backReferenceNames.push(e.lastStringValue), true;
    e.raise("Invalid named reference");
  }
  return false;
};
oe.regexp_eatCharacterEscape = function(e) {
  return this.regexp_eatControlEscape(e) || this.regexp_eatCControlLetter(e) || this.regexp_eatZero(e) || this.regexp_eatHexEscapeSequence(e) || this.regexp_eatRegExpUnicodeEscapeSequence(e, false) || !e.switchU && this.regexp_eatLegacyOctalEscapeSequence(e) || this.regexp_eatIdentityEscape(e);
};
oe.regexp_eatCControlLetter = function(e) {
  var t = e.pos;
  if (e.eat(99)) {
    if (this.regexp_eatControlLetter(e)) return true;
    e.pos = t;
  }
  return false;
};
oe.regexp_eatZero = function(e) {
  return e.current() === 48 && !el(e.lookahead()) ? (e.lastIntValue = 0, e.advance(), true) : false;
};
oe.regexp_eatControlEscape = function(e) {
  var t = e.current();
  return t === 116 ? (e.lastIntValue = 9, e.advance(), true) : t === 110 ? (e.lastIntValue = 10, e.advance(), true) : t === 118 ? (e.lastIntValue = 11, e.advance(), true) : t === 102 ? (e.lastIntValue = 12, e.advance(), true) : t === 114 ? (e.lastIntValue = 13, e.advance(), true) : false;
};
oe.regexp_eatControlLetter = function(e) {
  var t = e.current();
  return am(t) ? (e.lastIntValue = t % 32, e.advance(), true) : false;
};
function am(e) {
  return e >= 65 && e <= 90 || e >= 97 && e <= 122;
}
oe.regexp_eatRegExpUnicodeEscapeSequence = function(e, t) {
  t === void 0 && (t = false);
  var a = e.pos, s = t || e.switchU;
  if (e.eat(117)) {
    if (this.regexp_eatFixedHexDigits(e, 4)) {
      var u = e.lastIntValue;
      if (s && u >= 55296 && u <= 56319) {
        var p = e.pos;
        if (e.eat(92) && e.eat(117) && this.regexp_eatFixedHexDigits(e, 4)) {
          var m = e.lastIntValue;
          if (m >= 56320 && m <= 57343) return e.lastIntValue = (u - 55296) * 1024 + (m - 56320) + 65536, true;
        }
        e.pos = p, e.lastIntValue = u;
      }
      return true;
    }
    if (s && e.eat(123) && this.regexp_eatHexDigits(e) && e.eat(125) && S1(e.lastIntValue)) return true;
    s && e.raise("Invalid unicode escape"), e.pos = a;
  }
  return false;
};
function S1(e) {
  return e >= 0 && e <= 1114111;
}
oe.regexp_eatIdentityEscape = function(e) {
  if (e.switchU) return this.regexp_eatSyntaxCharacter(e) ? true : e.eat(47) ? (e.lastIntValue = 47, true) : false;
  var t = e.current();
  return t !== 99 && (!e.switchN || t !== 107) ? (e.lastIntValue = t, e.advance(), true) : false;
};
oe.regexp_eatDecimalEscape = function(e) {
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
var im = 0, It = 1, vt = 2;
oe.regexp_eatCharacterClassEscape = function(e) {
  var t = e.current();
  if (w1(t)) return e.lastIntValue = -1, e.advance(), It;
  var a = false;
  if (e.switchU && this.options.ecmaVersion >= 9 && ((a = t === 80) || t === 112)) {
    e.lastIntValue = -1, e.advance();
    var s;
    if (e.eat(123) && (s = this.regexp_eatUnicodePropertyValueExpression(e)) && e.eat(125)) return a && s === vt && e.raise("Invalid property name"), s;
    e.raise("Invalid property name");
  }
  return im;
};
function w1(e) {
  return e === 100 || e === 68 || e === 115 || e === 83 || e === 119 || e === 87;
}
oe.regexp_eatUnicodePropertyValueExpression = function(e) {
  var t = e.pos;
  if (this.regexp_eatUnicodePropertyName(e) && e.eat(61)) {
    var a = e.lastStringValue;
    if (this.regexp_eatUnicodePropertyValue(e)) {
      var s = e.lastStringValue;
      return this.regexp_validateUnicodePropertyNameAndValue(e, a, s), It;
    }
  }
  if (e.pos = t, this.regexp_eatLoneUnicodePropertyNameOrValue(e)) {
    var u = e.lastStringValue;
    return this.regexp_validateUnicodePropertyNameOrValue(e, u);
  }
  return im;
};
oe.regexp_validateUnicodePropertyNameAndValue = function(e, t, a) {
  er(e.unicodeProperties.nonBinary, t) || e.raise("Invalid property name"), e.unicodeProperties.nonBinary[t].test(a) || e.raise("Invalid property value");
};
oe.regexp_validateUnicodePropertyNameOrValue = function(e, t) {
  if (e.unicodeProperties.binary.test(t)) return It;
  if (e.switchV && e.unicodeProperties.binaryOfStrings.test(t)) return vt;
  e.raise("Invalid property name");
};
oe.regexp_eatUnicodePropertyName = function(e) {
  var t = 0;
  for (e.lastStringValue = ""; sm(t = e.current()); ) e.lastStringValue += kt(t), e.advance();
  return e.lastStringValue !== "";
};
function sm(e) {
  return am(e) || e === 95;
}
oe.regexp_eatUnicodePropertyValue = function(e) {
  var t = 0;
  for (e.lastStringValue = ""; A1(t = e.current()); ) e.lastStringValue += kt(t), e.advance();
  return e.lastStringValue !== "";
};
function A1(e) {
  return sm(e) || el(e);
}
oe.regexp_eatLoneUnicodePropertyNameOrValue = function(e) {
  return this.regexp_eatUnicodePropertyValue(e);
};
oe.regexp_eatCharacterClass = function(e) {
  if (e.eat(91)) {
    var t = e.eat(94), a = this.regexp_classContents(e);
    return e.eat(93) || e.raise("Unterminated character class"), t && a === vt && e.raise("Negated character class may contain strings"), true;
  }
  return false;
};
oe.regexp_classContents = function(e) {
  return e.current() === 93 ? It : e.switchV ? this.regexp_classSetExpression(e) : (this.regexp_nonEmptyClassRanges(e), It);
};
oe.regexp_nonEmptyClassRanges = function(e) {
  for (; this.regexp_eatClassAtom(e); ) {
    var t = e.lastIntValue;
    if (e.eat(45) && this.regexp_eatClassAtom(e)) {
      var a = e.lastIntValue;
      e.switchU && (t === -1 || a === -1) && e.raise("Invalid character class"), t !== -1 && a !== -1 && t > a && e.raise("Range out of order in character class");
    }
  }
};
oe.regexp_eatClassAtom = function(e) {
  var t = e.pos;
  if (e.eat(92)) {
    if (this.regexp_eatClassEscape(e)) return true;
    if (e.switchU) {
      var a = e.current();
      (a === 99 || lm(a)) && e.raise("Invalid class escape"), e.raise("Invalid escape");
    }
    e.pos = t;
  }
  var s = e.current();
  return s !== 93 ? (e.lastIntValue = s, e.advance(), true) : false;
};
oe.regexp_eatClassEscape = function(e) {
  var t = e.pos;
  if (e.eat(98)) return e.lastIntValue = 8, true;
  if (e.switchU && e.eat(45)) return e.lastIntValue = 45, true;
  if (!e.switchU && e.eat(99)) {
    if (this.regexp_eatClassControlLetter(e)) return true;
    e.pos = t;
  }
  return this.regexp_eatCharacterClassEscape(e) || this.regexp_eatCharacterEscape(e);
};
oe.regexp_classSetExpression = function(e) {
  var t = It, a;
  if (!this.regexp_eatClassSetRange(e)) if (a = this.regexp_eatClassSetOperand(e)) {
    a === vt && (t = vt);
    for (var s = e.pos; e.eatChars([38, 38]); ) {
      if (e.current() !== 38 && (a = this.regexp_eatClassSetOperand(e))) {
        a !== vt && (t = It);
        continue;
      }
      e.raise("Invalid character in character class");
    }
    if (s !== e.pos) return t;
    for (; e.eatChars([45, 45]); ) this.regexp_eatClassSetOperand(e) || e.raise("Invalid character in character class");
    if (s !== e.pos) return t;
  } else e.raise("Invalid character in character class");
  for (; ; ) if (!this.regexp_eatClassSetRange(e)) {
    if (a = this.regexp_eatClassSetOperand(e), !a) return t;
    a === vt && (t = vt);
  }
};
oe.regexp_eatClassSetRange = function(e) {
  var t = e.pos;
  if (this.regexp_eatClassSetCharacter(e)) {
    var a = e.lastIntValue;
    if (e.eat(45) && this.regexp_eatClassSetCharacter(e)) {
      var s = e.lastIntValue;
      return a !== -1 && s !== -1 && a > s && e.raise("Range out of order in character class"), true;
    }
    e.pos = t;
  }
  return false;
};
oe.regexp_eatClassSetOperand = function(e) {
  return this.regexp_eatClassSetCharacter(e) ? It : this.regexp_eatClassStringDisjunction(e) || this.regexp_eatNestedClass(e);
};
oe.regexp_eatNestedClass = function(e) {
  var t = e.pos;
  if (e.eat(91)) {
    var a = e.eat(94), s = this.regexp_classContents(e);
    if (e.eat(93)) return a && s === vt && e.raise("Negated character class may contain strings"), s;
    e.pos = t;
  }
  if (e.eat(92)) {
    var u = this.regexp_eatCharacterClassEscape(e);
    if (u) return u;
    e.pos = t;
  }
  return null;
};
oe.regexp_eatClassStringDisjunction = function(e) {
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
oe.regexp_classStringDisjunctionContents = function(e) {
  for (var t = this.regexp_classString(e); e.eat(124); ) this.regexp_classString(e) === vt && (t = vt);
  return t;
};
oe.regexp_classString = function(e) {
  for (var t = 0; this.regexp_eatClassSetCharacter(e); ) t++;
  return t === 1 ? It : vt;
};
oe.regexp_eatClassSetCharacter = function(e) {
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
oe.regexp_eatClassSetReservedPunctuator = function(e) {
  var t = e.current();
  return I1(t) ? (e.lastIntValue = t, e.advance(), true) : false;
};
function I1(e) {
  return e === 33 || e === 35 || e === 37 || e === 38 || e === 44 || e === 45 || e >= 58 && e <= 62 || e === 64 || e === 96 || e === 126;
}
oe.regexp_eatClassControlLetter = function(e) {
  var t = e.current();
  return el(t) || t === 95 ? (e.lastIntValue = t % 32, e.advance(), true) : false;
};
oe.regexp_eatHexEscapeSequence = function(e) {
  var t = e.pos;
  if (e.eat(120)) {
    if (this.regexp_eatFixedHexDigits(e, 2)) return true;
    e.switchU && e.raise("Invalid escape"), e.pos = t;
  }
  return false;
};
oe.regexp_eatDecimalDigits = function(e) {
  var t = e.pos, a = 0;
  for (e.lastIntValue = 0; el(a = e.current()); ) e.lastIntValue = 10 * e.lastIntValue + (a - 48), e.advance();
  return e.pos !== t;
};
function el(e) {
  return e >= 48 && e <= 57;
}
oe.regexp_eatHexDigits = function(e) {
  var t = e.pos, a = 0;
  for (e.lastIntValue = 0; nm(a = e.current()); ) e.lastIntValue = 16 * e.lastIntValue + om(a), e.advance();
  return e.pos !== t;
};
function nm(e) {
  return e >= 48 && e <= 57 || e >= 65 && e <= 70 || e >= 97 && e <= 102;
}
function om(e) {
  return e >= 65 && e <= 70 ? 10 + (e - 65) : e >= 97 && e <= 102 ? 10 + (e - 97) : e - 48;
}
oe.regexp_eatLegacyOctalEscapeSequence = function(e) {
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
oe.regexp_eatOctalDigit = function(e) {
  var t = e.current();
  return lm(t) ? (e.lastIntValue = t - 48, e.advance(), true) : (e.lastIntValue = 0, false);
};
function lm(e) {
  return e >= 48 && e <= 55;
}
oe.regexp_eatFixedHexDigits = function(e, t) {
  var a = e.pos;
  e.lastIntValue = 0;
  for (var s = 0; s < t; ++s) {
    var u = e.current();
    if (!nm(u)) return e.pos = a, false;
    e.lastIntValue = 16 * e.lastIntValue + om(u), e.advance();
  }
  return true;
};
var tl = function(t) {
  this.type = t.type, this.value = t.value, this.start = t.start, this.end = t.end, t.options.locations && (this.loc = new Po(t, t.startLoc, t.endLoc)), t.options.ranges && (this.range = [t.start, t.end]);
}, qe = Ue.prototype;
qe.next = function(e) {
  !e && this.type.keyword && this.containsEsc && this.raiseRecoverable(this.start, "Escape sequence in keyword " + this.type.keyword), this.options.onToken && this.options.onToken(new tl(this)), this.lastTokEnd = this.end, this.lastTokStart = this.start, this.lastTokEndLoc = this.endLoc, this.lastTokStartLoc = this.startLoc, this.nextToken();
};
qe.getToken = function() {
  return this.next(), new tl(this);
};
typeof Symbol < "u" && (qe[Symbol.iterator] = function() {
  var e = this;
  return { next: function() {
    var t = e.getToken();
    return { done: t.type === d.eof, value: t };
  } };
});
qe.nextToken = function() {
  var e = this.curContext();
  if ((!e || !e.preserveSpace) && this.skipSpace(), this.start = this.pos, this.options.locations && (this.startLoc = this.curPosition()), this.pos >= this.input.length) return this.finishToken(d.eof);
  if (e.override) return e.override(this);
  this.readToken(this.fullCharCodeAtPos());
};
qe.readToken = function(e) {
  return Tt(e, this.options.ecmaVersion >= 6) || e === 92 ? this.readWord() : this.getTokenFromCode(e);
};
qe.fullCharCodeAtPos = function() {
  var e = this.input.charCodeAt(this.pos);
  if (e <= 55295 || e >= 56320) return e;
  var t = this.input.charCodeAt(this.pos + 1);
  return t <= 56319 || t >= 57344 ? e : (e << 10) + t - 56613888;
};
qe.skipBlockComment = function() {
  var e = this.options.onComment && this.curPosition(), t = this.pos, a = this.input.indexOf("*/", this.pos += 2);
  if (a === -1 && this.raise(this.pos - 2, "Unterminated comment"), this.pos = a + 2, this.options.locations) for (var s = void 0, u = t; (s = Mh(this.input, u, this.pos)) > -1; ) ++this.curLine, u = this.lineStart = s;
  this.options.onComment && this.options.onComment(true, this.input.slice(t + 2, a), t, this.pos, e, this.curPosition());
};
qe.skipLineComment = function(e) {
  for (var t = this.pos, a = this.options.onComment && this.curPosition(), s = this.input.charCodeAt(this.pos += e); this.pos < this.input.length && !Ht(s); ) s = this.input.charCodeAt(++this.pos);
  this.options.onComment && this.options.onComment(false, this.input.slice(t + e, this.pos), t, this.pos, a, this.curPosition());
};
qe.skipSpace = function() {
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
        if (e > 8 && e < 14 || e >= 5760 && Cl.test(String.fromCharCode(e))) ++this.pos;
        else break e;
    }
  }
};
qe.finishToken = function(e, t) {
  this.end = this.pos, this.options.locations && (this.endLoc = this.curPosition());
  var a = this.type;
  this.type = e, this.value = t, this.updateContext(a);
};
qe.readToken_dot = function() {
  var e = this.input.charCodeAt(this.pos + 1);
  if (e >= 48 && e <= 57) return this.readNumber(true);
  var t = this.input.charCodeAt(this.pos + 2);
  return this.options.ecmaVersion >= 6 && e === 46 && t === 46 ? (this.pos += 3, this.finishToken(d.ellipsis)) : (++this.pos, this.finishToken(d.dot));
};
qe.readToken_slash = function() {
  var e = this.input.charCodeAt(this.pos + 1);
  return this.exprAllowed ? (++this.pos, this.readRegexp()) : e === 61 ? this.finishOp(d.assign, 2) : this.finishOp(d.slash, 1);
};
qe.readToken_mult_modulo_exp = function(e) {
  var t = this.input.charCodeAt(this.pos + 1), a = 1, s = e === 42 ? d.star : d.modulo;
  return this.options.ecmaVersion >= 7 && e === 42 && t === 42 && (++a, s = d.starstar, t = this.input.charCodeAt(this.pos + 2)), t === 61 ? this.finishOp(d.assign, a + 1) : this.finishOp(s, a);
};
qe.readToken_pipe_amp = function(e) {
  var t = this.input.charCodeAt(this.pos + 1);
  if (t === e) {
    if (this.options.ecmaVersion >= 12) {
      var a = this.input.charCodeAt(this.pos + 2);
      if (a === 61) return this.finishOp(d.assign, 3);
    }
    return this.finishOp(e === 124 ? d.logicalOR : d.logicalAND, 2);
  }
  return t === 61 ? this.finishOp(d.assign, 2) : this.finishOp(e === 124 ? d.bitwiseOR : d.bitwiseAND, 1);
};
qe.readToken_caret = function() {
  var e = this.input.charCodeAt(this.pos + 1);
  return e === 61 ? this.finishOp(d.assign, 2) : this.finishOp(d.bitwiseXOR, 1);
};
qe.readToken_plus_min = function(e) {
  var t = this.input.charCodeAt(this.pos + 1);
  return t === e ? t === 45 && !this.inModule && this.input.charCodeAt(this.pos + 2) === 62 && (this.lastTokEnd === 0 || dt.test(this.input.slice(this.lastTokEnd, this.pos))) ? (this.skipLineComment(3), this.skipSpace(), this.nextToken()) : this.finishOp(d.incDec, 2) : t === 61 ? this.finishOp(d.assign, 2) : this.finishOp(d.plusMin, 1);
};
qe.readToken_lt_gt = function(e) {
  var t = this.input.charCodeAt(this.pos + 1), a = 1;
  return t === e ? (a = e === 62 && this.input.charCodeAt(this.pos + 2) === 62 ? 3 : 2, this.input.charCodeAt(this.pos + a) === 61 ? this.finishOp(d.assign, a + 1) : this.finishOp(d.bitShift, a)) : t === 33 && e === 60 && !this.inModule && this.input.charCodeAt(this.pos + 2) === 45 && this.input.charCodeAt(this.pos + 3) === 45 ? (this.skipLineComment(4), this.skipSpace(), this.nextToken()) : (t === 61 && (a = 2), this.finishOp(d.relational, a));
};
qe.readToken_eq_excl = function(e) {
  var t = this.input.charCodeAt(this.pos + 1);
  return t === 61 ? this.finishOp(d.equality, this.input.charCodeAt(this.pos + 2) === 61 ? 3 : 2) : e === 61 && t === 62 && this.options.ecmaVersion >= 6 ? (this.pos += 2, this.finishToken(d.arrow)) : this.finishOp(e === 61 ? d.eq : d.prefix, 1);
};
qe.readToken_question = function() {
  var e = this.options.ecmaVersion;
  if (e >= 11) {
    var t = this.input.charCodeAt(this.pos + 1);
    if (t === 46) {
      var a = this.input.charCodeAt(this.pos + 2);
      if (a < 48 || a > 57) return this.finishOp(d.questionDot, 2);
    }
    if (t === 63) {
      if (e >= 12) {
        var s = this.input.charCodeAt(this.pos + 2);
        if (s === 61) return this.finishOp(d.assign, 3);
      }
      return this.finishOp(d.coalesce, 2);
    }
  }
  return this.finishOp(d.question, 1);
};
qe.readToken_numberSign = function() {
  var e = this.options.ecmaVersion, t = 35;
  if (e >= 13 && (++this.pos, t = this.fullCharCodeAtPos(), Tt(t, true) || t === 92)) return this.finishToken(d.privateId, this.readWord1());
  this.raise(this.pos, "Unexpected character '" + kt(t) + "'");
};
qe.getTokenFromCode = function(e) {
  switch (e) {
    case 46:
      return this.readToken_dot();
    case 40:
      return ++this.pos, this.finishToken(d.parenL);
    case 41:
      return ++this.pos, this.finishToken(d.parenR);
    case 59:
      return ++this.pos, this.finishToken(d.semi);
    case 44:
      return ++this.pos, this.finishToken(d.comma);
    case 91:
      return ++this.pos, this.finishToken(d.bracketL);
    case 93:
      return ++this.pos, this.finishToken(d.bracketR);
    case 123:
      return ++this.pos, this.finishToken(d.braceL);
    case 125:
      return ++this.pos, this.finishToken(d.braceR);
    case 58:
      return ++this.pos, this.finishToken(d.colon);
    case 96:
      if (this.options.ecmaVersion < 6) break;
      return ++this.pos, this.finishToken(d.backQuote);
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
      return this.finishOp(d.prefix, 1);
    case 35:
      return this.readToken_numberSign();
  }
  this.raise(this.pos, "Unexpected character '" + kt(e) + "'");
};
qe.finishOp = function(e, t) {
  var a = this.input.slice(this.pos, this.pos + t);
  return this.pos += t, this.finishToken(e, a);
};
qe.readRegexp = function() {
  for (var e, t, a = this.pos; ; ) {
    this.pos >= this.input.length && this.raise(a, "Unterminated regular expression");
    var s = this.input.charAt(this.pos);
    if (dt.test(s) && this.raise(a, "Unterminated regular expression"), e) e = false;
    else {
      if (s === "[") t = true;
      else if (s === "]" && t) t = false;
      else if (s === "/" && !t) break;
      e = s === "\\";
    }
    ++this.pos;
  }
  var u = this.input.slice(a, this.pos);
  ++this.pos;
  var p = this.pos, m = this.readWord1();
  this.containsEsc && this.unexpected(p);
  var w = this.regexpState || (this.regexpState = new wt(this));
  w.reset(a, u, m), this.validateRegExpFlags(w), this.validateRegExpPattern(w);
  var x = null;
  try {
    x = new RegExp(u, m);
  } catch {
  }
  return this.finishToken(d.regexp, { pattern: u, flags: m, value: x });
};
qe.readInt = function(e, t, a) {
  for (var s = this.options.ecmaVersion >= 12 && t === void 0, u = a && this.input.charCodeAt(this.pos) === 48, p = this.pos, m = 0, w = 0, x = 0, o = t ?? 1 / 0; x < o; ++x, ++this.pos) {
    var y = this.input.charCodeAt(this.pos), O = void 0;
    if (s && y === 95) {
      u && this.raiseRecoverable(this.pos, "Numeric separator is not allowed in legacy octal numeric literals"), w === 95 && this.raiseRecoverable(this.pos, "Numeric separator must be exactly one underscore"), x === 0 && this.raiseRecoverable(this.pos, "Numeric separator is not allowed at the first of digits"), w = y;
      continue;
    }
    if (y >= 97 ? O = y - 97 + 10 : y >= 65 ? O = y - 65 + 10 : y >= 48 && y <= 57 ? O = y - 48 : O = 1 / 0, O >= e) break;
    w = y, m = m * e + O;
  }
  return s && w === 95 && this.raiseRecoverable(this.pos - 1, "Numeric separator is not allowed at the last of digits"), this.pos === p || t != null && this.pos - p !== t ? null : m;
};
function M1(e, t) {
  return t ? parseInt(e, 8) : parseFloat(e.replace(/_/g, ""));
}
function um(e) {
  return typeof BigInt != "function" ? null : BigInt(e.replace(/_/g, ""));
}
qe.readRadixNumber = function(e) {
  var t = this.pos;
  this.pos += 2;
  var a = this.readInt(e);
  return a == null && this.raise(this.start + 2, "Expected number in radix " + e), this.options.ecmaVersion >= 11 && this.input.charCodeAt(this.pos) === 110 ? (a = um(this.input.slice(t, this.pos)), ++this.pos) : Tt(this.fullCharCodeAtPos()) && this.raise(this.pos, "Identifier directly after number"), this.finishToken(d.num, a);
};
qe.readNumber = function(e) {
  var t = this.pos;
  !e && this.readInt(10, void 0, true) === null && this.raise(t, "Invalid number");
  var a = this.pos - t >= 2 && this.input.charCodeAt(t) === 48;
  a && this.strict && this.raise(t, "Invalid number");
  var s = this.input.charCodeAt(this.pos);
  if (!a && !e && this.options.ecmaVersion >= 11 && s === 110) {
    var u = um(this.input.slice(t, this.pos));
    return ++this.pos, Tt(this.fullCharCodeAtPos()) && this.raise(this.pos, "Identifier directly after number"), this.finishToken(d.num, u);
  }
  a && /[89]/.test(this.input.slice(t, this.pos)) && (a = false), s === 46 && !a && (++this.pos, this.readInt(10), s = this.input.charCodeAt(this.pos)), (s === 69 || s === 101) && !a && (s = this.input.charCodeAt(++this.pos), (s === 43 || s === 45) && ++this.pos, this.readInt(10) === null && this.raise(t, "Invalid number")), Tt(this.fullCharCodeAtPos()) && this.raise(this.pos, "Identifier directly after number");
  var p = M1(this.input.slice(t, this.pos), a);
  return this.finishToken(d.num, p);
};
qe.readCodePoint = function() {
  var e = this.input.charCodeAt(this.pos), t;
  if (e === 123) {
    this.options.ecmaVersion < 6 && this.unexpected();
    var a = ++this.pos;
    t = this.readHexChar(this.input.indexOf("}", this.pos) - this.pos), ++this.pos, t > 1114111 && this.invalidStringToken(a, "Code point out of bounds");
  } else t = this.readHexChar(4);
  return t;
};
qe.readString = function(e) {
  for (var t = "", a = ++this.pos; ; ) {
    this.pos >= this.input.length && this.raise(this.start, "Unterminated string constant");
    var s = this.input.charCodeAt(this.pos);
    if (s === e) break;
    s === 92 ? (t += this.input.slice(a, this.pos), t += this.readEscapedChar(false), a = this.pos) : s === 8232 || s === 8233 ? (this.options.ecmaVersion < 10 && this.raise(this.start, "Unterminated string constant"), ++this.pos, this.options.locations && (this.curLine++, this.lineStart = this.pos)) : (Ht(s) && this.raise(this.start, "Unterminated string constant"), ++this.pos);
  }
  return t += this.input.slice(a, this.pos++), this.finishToken(d.string, t);
};
var cm = {};
qe.tryReadTemplateToken = function() {
  this.inTemplateElement = true;
  try {
    this.readTmplToken();
  } catch (e) {
    if (e === cm) this.readInvalidTemplateToken();
    else throw e;
  }
  this.inTemplateElement = false;
};
qe.invalidStringToken = function(e, t) {
  if (this.inTemplateElement && this.options.ecmaVersion >= 9) throw cm;
  this.raise(e, t);
};
qe.readTmplToken = function() {
  for (var e = "", t = this.pos; ; ) {
    this.pos >= this.input.length && this.raise(this.start, "Unterminated template");
    var a = this.input.charCodeAt(this.pos);
    if (a === 96 || a === 36 && this.input.charCodeAt(this.pos + 1) === 123) return this.pos === this.start && (this.type === d.template || this.type === d.invalidTemplate) ? a === 36 ? (this.pos += 2, this.finishToken(d.dollarBraceL)) : (++this.pos, this.finishToken(d.backQuote)) : (e += this.input.slice(t, this.pos), this.finishToken(d.template, e));
    if (a === 92) e += this.input.slice(t, this.pos), e += this.readEscapedChar(true), t = this.pos;
    else if (Ht(a)) {
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
qe.readInvalidTemplateToken = function() {
  for (; this.pos < this.input.length; this.pos++) switch (this.input[this.pos]) {
    case "\\":
      ++this.pos;
      break;
    case "$":
      if (this.input[this.pos + 1] !== "{") break;
    case "`":
      return this.finishToken(d.invalidTemplate, this.input.slice(this.start, this.pos));
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
qe.readEscapedChar = function(e) {
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
      return kt(this.readCodePoint());
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
        var s = this.input.substr(this.pos - 1, 3).match(/^[0-7]+/)[0], u = parseInt(s, 8);
        return u > 255 && (s = s.slice(0, -1), u = parseInt(s, 8)), this.pos += s.length - 1, t = this.input.charCodeAt(this.pos), (s !== "0" || t === 56 || t === 57) && (this.strict || e) && this.invalidStringToken(this.pos - 1 - s.length, e ? "Octal literal in template string" : "Octal literal in strict mode"), String.fromCharCode(u);
      }
      return Ht(t) ? (this.options.locations && (this.lineStart = this.pos, ++this.curLine), "") : String.fromCharCode(t);
  }
};
qe.readHexChar = function(e) {
  var t = this.pos, a = this.readInt(16, e);
  return a === null && this.invalidStringToken(t, "Bad character escape sequence"), a;
};
qe.readWord1 = function() {
  this.containsEsc = false;
  for (var e = "", t = true, a = this.pos, s = this.options.ecmaVersion >= 6; this.pos < this.input.length; ) {
    var u = this.fullCharCodeAtPos();
    if (Vt(u, s)) this.pos += u <= 65535 ? 1 : 2;
    else if (u === 92) {
      this.containsEsc = true, e += this.input.slice(a, this.pos);
      var p = this.pos;
      this.input.charCodeAt(++this.pos) !== 117 && this.invalidStringToken(this.pos, "Expecting Unicode escape sequence \\uXXXX"), ++this.pos;
      var m = this.readCodePoint();
      (t ? Tt : Vt)(m, s) || this.invalidStringToken(p, "Invalid Unicode escape"), e += kt(m), a = this.pos;
    } else break;
    t = false;
  }
  return e + this.input.slice(a, this.pos);
};
qe.readWord = function() {
  var e = this.readWord1(), t = d.name;
  return this.keywords.test(e) && (t = Kt[e]), this.finishToken(t, e);
};
var dm = "8.14.0";
Ue.acorn = { Parser: Ue, version: dm, defaultOptions: zo, Position: Jt, SourceLocation: Po, getLineInfo: Pl, Node: To, TokenType: Ce, tokTypes: d, keywordTypes: Kt, TokContext: rt, tokContexts: Ve, isIdentifierChar: Vt, isIdentifierStart: Tt, Token: tl, isNewLine: Ht, lineBreak: dt, lineBreakG: Ih, nonASCIIwhitespace: Cl };
function L1(e, t) {
  return Ue.parse(e, t);
}
function O1(e, t, a) {
  return Ue.parseExpressionAt(e, t, a);
}
function N1(e, t) {
  return Ue.tokenizer(e, t);
}
const D1 = Object.freeze(Object.defineProperty({ __proto__: null, Node: To, Parser: Ue, Position: Jt, SourceLocation: Po, TokContext: rt, Token: tl, TokenType: Ce, defaultOptions: zo, getLineInfo: Pl, isIdentifierChar: Vt, isIdentifierStart: Tt, isNewLine: Ht, keywordTypes: Kt, lineBreak: dt, lineBreakG: Ih, nonASCIIwhitespace: Cl, parse: L1, parseExpressionAt: O1, tokContexts: Ve, tokTypes: d, tokenizer: N1, version: dm }, Symbol.toStringTag, { value: "Module" }));
function Wl(e, t) {
  for (var a = 0; a < t.length; a++) {
    var s = t[a];
    s.enumerable = s.enumerable || false, s.configurable = true, "value" in s && (s.writable = true), Object.defineProperty(e, typeof (u = function(p, m) {
      if (typeof p != "object" || p === null) return p;
      var w = p[Symbol.toPrimitive];
      if (w !== void 0) {
        var x = w.call(p, "string");
        if (typeof x != "object") return x;
        throw new TypeError("@@toPrimitive must return a primitive value.");
      }
      return String(p);
    }(s.key)) == "symbol" ? u : String(u), s);
  }
  var u;
}
function Qo() {
  return Qo = Object.assign ? Object.assign.bind() : function(e) {
    for (var t = 1; t < arguments.length; t++) {
      var a = arguments[t];
      for (var s in a) Object.prototype.hasOwnProperty.call(a, s) && (e[s] = a[s]);
    }
    return e;
  }, Qo.apply(this, arguments);
}
function Oo(e, t) {
  e.prototype = Object.create(t.prototype), e.prototype.constructor = e, yl(e, t);
}
function yl(e, t) {
  return yl = Object.setPrototypeOf ? Object.setPrototypeOf.bind() : function(a, s) {
    return a.__proto__ = s, a;
  }, yl(e, t);
}
function Xl(e, t) {
  (t == null || t > e.length) && (t = e.length);
  for (var a = 0, s = new Array(t); a < t; a++) s[a] = e[a];
  return s;
}
function Kl(e, t) {
  var a = typeof Symbol < "u" && e[Symbol.iterator] || e["@@iterator"];
  if (a) return (a = a.call(e)).next.bind(a);
  if (Array.isArray(e) || (a = function(u, p) {
    if (u) {
      if (typeof u == "string") return Xl(u, p);
      var m = Object.prototype.toString.call(u).slice(8, -1);
      return m === "Object" && u.constructor && (m = u.constructor.name), m === "Map" || m === "Set" ? Array.from(u) : m === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(m) ? Xl(u, p) : void 0;
    }
  }(e)) || t) {
    a && (e = a);
    var s = 0;
    return function() {
      return s >= e.length ? { done: true } : { done: false, value: e[s++] };
    };
  }
  throw new TypeError(`Invalid attempt to iterate non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
}
var lt = true;
function ut(e, t) {
  return t === void 0 && (t = {}), new Ce("name", t);
}
var j1 = /* @__PURE__ */ new WeakMap();
function B1(e) {
  var t = j1.get(e.Parser.acorn || e);
  if (!t) {
    var a = { assert: ut(0, { startsExpr: lt }), asserts: ut(0, { startsExpr: lt }), global: ut(0, { startsExpr: lt }), keyof: ut(0, { startsExpr: lt }), readonly: ut(0, { startsExpr: lt }), unique: ut(0, { startsExpr: lt }), abstract: ut(0, { startsExpr: lt }), declare: ut(0, { startsExpr: lt }), enum: ut(0, { startsExpr: lt }), module: ut(0, { startsExpr: lt }), namespace: ut(0, { startsExpr: lt }), interface: ut(0, { startsExpr: lt }), type: ut(0, { startsExpr: lt }) }, s = { at: new Ce("@"), jsxName: new Ce("jsxName"), jsxText: new Ce("jsxText", { beforeExpr: true }), jsxTagStart: new Ce("jsxTagStart", { startsExpr: true }), jsxTagEnd: new Ce("jsxTagEnd") }, u = { tc_oTag: new rt("<tag", false, false), tc_cTag: new rt("</tag", false, false), tc_expr: new rt("<tag>...</tag>", true, true) }, p = new RegExp("^(?:" + Object.keys(a).join("|") + ")$");
    s.jsxTagStart.updateContext = function() {
      this.context.push(u.tc_expr), this.context.push(u.tc_oTag), this.exprAllowed = false;
    }, s.jsxTagEnd.updateContext = function(m) {
      var w = this.context.pop();
      w === u.tc_oTag && m === d.slash || w === u.tc_cTag ? (this.context.pop(), this.exprAllowed = this.curContext() === u.tc_expr) : this.exprAllowed = true;
    }, t = { tokTypes: Qo({}, a, s), tokContexts: Qo({}, u), keywordsRegExp: p, tokenIsLiteralPropertyName: function(m) {
      return [d.name, d.string, d.num].concat(Object.values(Kt), Object.values(a)).includes(m);
    }, tokenIsKeywordOrIdentifier: function(m) {
      return [d.name].concat(Object.values(Kt), Object.values(a)).includes(m);
    }, tokenIsIdentifier: function(m) {
      return [].concat(Object.values(a), [d.name]).includes(m);
    }, tokenIsTSDeclarationStart: function(m) {
      return [a.abstract, a.declare, a.enum, a.module, a.namespace, a.interface, a.type].includes(m);
    }, tokenIsTSTypeOperator: function(m) {
      return [a.keyof, a.readonly, a.unique].includes(m);
    }, tokenIsTemplate: function(m) {
      return m === d.invalidTemplate;
    } };
  }
  return t;
}
var ir = 1024, F1 = new RegExp("(?:[^\\S\\n\\r\\u2028\\u2029]|\\/\\/.*|\\/\\*.*?\\*\\/)*", "y"), Jl = new RegExp("(?=(" + F1.source + "))\\1" + /(?=[\n\r\u2028\u2029]|\/\*(?!.*?\*\/)|$)/.source, "y"), sr = function() {
  this.shorthandAssign = void 0, this.trailingComma = void 0, this.parenthesizedAssign = void 0, this.parenthesizedBind = void 0, this.doubleProto = void 0, this.shorthandAssign = this.trailingComma = this.parenthesizedAssign = this.parenthesizedBind = this.doubleProto = -1;
};
function V1(e, t) {
  var a = t.key.name, s = e[a], u = "true";
  return t.type !== "MethodDefinition" || t.kind !== "get" && t.kind !== "set" || (u = (t.static ? "s" : "i") + t.kind), s === "iget" && u === "iset" || s === "iset" && u === "iget" || s === "sget" && u === "sset" || s === "sset" && u === "sget" ? (e[a] = "true", false) : !!s || (e[a] = u, false);
}
function Ql(e, t) {
  var a = e.key;
  return !e.computed && (a.type === "Identifier" && a.name === t || a.type === "Literal" && a.value === t);
}
var le = { AbstractMethodHasImplementation: function(e) {
  return "Method '" + e.methodName + "' cannot have an implementation because it is marked abstract.";
}, AbstractPropertyHasInitializer: function(e) {
  return "Property '" + e.propertyName + "' cannot have an initializer because it is marked abstract.";
}, AccesorCannotDeclareThisParameter: "'get' and 'set' accessors cannot declare 'this' parameters.", AccesorCannotHaveTypeParameters: "An accessor cannot have type parameters.", CannotFindName: function(e) {
  return "Cannot find name '" + e.name + "'.";
}, ClassMethodHasDeclare: "Class methods cannot have the 'declare' modifier.", ClassMethodHasReadonly: "Class methods cannot have the 'readonly' modifier.", ConstInitiailizerMustBeStringOrNumericLiteralOrLiteralEnumReference: "A 'const' initializer in an ambient context must be a string or numeric literal or literal enum reference.", ConstructorHasTypeParameters: "Type parameters cannot appear on a constructor declaration.", DeclareAccessor: function(e) {
  return "'declare' is not allowed in " + e.kind + "ters.";
}, DeclareClassFieldHasInitializer: "Initializers are not allowed in ambient contexts.", DeclareFunctionHasImplementation: "An implementation cannot be declared in ambient contexts.", DuplicateAccessibilityModifier: function() {
  return "Accessibility modifier already seen.";
}, DuplicateModifier: function(e) {
  return "Duplicate modifier: '" + e.modifier + "'.";
}, EmptyHeritageClauseType: function(e) {
  return "'" + e.token + "' list cannot be empty.";
}, EmptyTypeArguments: "Type argument list cannot be empty.", EmptyTypeParameters: "Type parameter list cannot be empty.", ExpectedAmbientAfterExportDeclare: "'export declare' must be followed by an ambient declaration.", ImportAliasHasImportType: "An import alias can not use 'import type'.", IncompatibleModifiers: function(e) {
  var t = e.modifiers;
  return "'" + t[0] + "' modifier cannot be used with '" + t[1] + "' modifier.";
}, IndexSignatureHasAbstract: "Index signatures cannot have the 'abstract' modifier.", IndexSignatureHasAccessibility: function(e) {
  return "Index signatures cannot have an accessibility modifier ('" + e.modifier + "').";
}, IndexSignatureHasDeclare: "Index signatures cannot have the 'declare' modifier.", IndexSignatureHasOverride: "'override' modifier cannot appear on an index signature.", IndexSignatureHasStatic: "Index signatures cannot have the 'static' modifier.", InitializerNotAllowedInAmbientContext: "Initializers are not allowed in ambient contexts.", InvalidModifierOnTypeMember: function(e) {
  return "'" + e.modifier + "' modifier cannot appear on a type member.";
}, InvalidModifierOnTypeParameter: function(e) {
  return "'" + e.modifier + "' modifier cannot appear on a type parameter.";
}, InvalidModifierOnTypeParameterPositions: function(e) {
  return "'" + e.modifier + "' modifier can only appear on a type parameter of a class, interface or type alias.";
}, InvalidModifiersOrder: function(e) {
  var t = e.orderedModifiers;
  return "'" + t[0] + "' modifier must precede '" + t[1] + "' modifier.";
}, InvalidPropertyAccessAfterInstantiationExpression: "Invalid property access after an instantiation expression. You can either wrap the instantiation expression in parentheses, or delete the type arguments.", InvalidTupleMemberLabel: "Tuple members must be labeled with a simple identifier.", MissingInterfaceName: "'interface' declarations must be followed by an identifier.", MixedLabeledAndUnlabeledElements: "Tuple members must all have names or all not have names.", NonAbstractClassHasAbstractMethod: "Abstract methods can only appear within an abstract class.", NonClassMethodPropertyHasAbstractModifer: "'abstract' modifier can only appear on a class, method, or property declaration.", OptionalTypeBeforeRequired: "A required element cannot follow an optional element.", OverrideNotInSubClass: "This member cannot have an 'override' modifier because its containing class does not extend another class.", PatternIsOptional: "A binding pattern parameter cannot be optional in an implementation signature.", PrivateElementHasAbstract: "Private elements cannot have the 'abstract' modifier.", PrivateElementHasAccessibility: function(e) {
  return "Private elements cannot have an accessibility modifier ('" + e.modifier + "').";
}, PrivateMethodsHasAccessibility: function(e) {
  return "Private methods cannot have an accessibility modifier ('" + e.modifier + "').";
}, ReadonlyForMethodSignature: "'readonly' modifier can only appear on a property declaration or index signature.", ReservedArrowTypeParam: "This syntax is reserved in files with the .mts or .cts extension. Add a trailing comma, as in `<T,>() => ...`.", ReservedTypeAssertion: "This syntax is reserved in files with the .mts or .cts extension. Use an `as` expression instead.", SetAccesorCannotHaveOptionalParameter: "A 'set' accessor cannot have an optional parameter.", SetAccesorCannotHaveRestParameter: "A 'set' accessor cannot have rest parameter.", SetAccesorCannotHaveReturnType: "A 'set' accessor cannot have a return type annotation.", SingleTypeParameterWithoutTrailingComma: function(e) {
  var t = e.typeParameterName;
  return "Single type parameter " + t + " should have a trailing comma. Example usage: <" + t + ",>.";
}, StaticBlockCannotHaveModifier: "Static class blocks cannot have any modifier.", TypeAnnotationAfterAssign: "Type annotations must come before default assignments, e.g. instead of `age = 25: number` use `age: number = 25`.", TypeImportCannotSpecifyDefaultAndNamed: "A type-only import can specify a default import or named bindings, but not both.", TypeModifierIsUsedInTypeExports: "The 'type' modifier cannot be used on a named export when 'export type' is used on its export statement.", TypeModifierIsUsedInTypeImports: "The 'type' modifier cannot be used on a named import when 'import type' is used on its import statement.", UnexpectedParameterModifier: "A parameter property is only allowed in a constructor implementation.", UnexpectedReadonly: "'readonly' type modifier is only permitted on array and tuple literal types.", GenericsEndWithComma: "Trailing comma is not allowed at the end of generics.", UnexpectedTypeAnnotation: "Did not expect a type annotation here.", UnexpectedTypeCastInParameter: "Unexpected type cast in parameter position.", UnsupportedImportTypeArgument: "Argument in a type import must be a string literal.", UnsupportedParameterPropertyKind: "A parameter property may not be declared using a binding pattern.", UnsupportedSignatureParameterKind: function(e) {
  return "Name in a signature must be an Identifier, ObjectPattern or ArrayPattern, instead got " + e.type + ".";
}, LetInLexicalBinding: "'let' is not allowed to be used as a name in 'let' or 'const' declarations." }, $1 = { quot: '"', amp: "&", apos: "'", lt: "<", gt: ">", nbsp: "\xA0", iexcl: "\xA1", cent: "\xA2", pound: "\xA3", curren: "\xA4", yen: "\xA5", brvbar: "\xA6", sect: "\xA7", uml: "\xA8", copy: "\xA9", ordf: "\xAA", laquo: "\xAB", not: "\xAC", shy: "\xAD", reg: "\xAE", macr: "\xAF", deg: "\xB0", plusmn: "\xB1", sup2: "\xB2", sup3: "\xB3", acute: "\xB4", micro: "\xB5", para: "\xB6", middot: "\xB7", cedil: "\xB8", sup1: "\xB9", ordm: "\xBA", raquo: "\xBB", frac14: "\xBC", frac12: "\xBD", frac34: "\xBE", iquest: "\xBF", Agrave: "\xC0", Aacute: "\xC1", Acirc: "\xC2", Atilde: "\xC3", Auml: "\xC4", Aring: "\xC5", AElig: "\xC6", Ccedil: "\xC7", Egrave: "\xC8", Eacute: "\xC9", Ecirc: "\xCA", Euml: "\xCB", Igrave: "\xCC", Iacute: "\xCD", Icirc: "\xCE", Iuml: "\xCF", ETH: "\xD0", Ntilde: "\xD1", Ograve: "\xD2", Oacute: "\xD3", Ocirc: "\xD4", Otilde: "\xD5", Ouml: "\xD6", times: "\xD7", Oslash: "\xD8", Ugrave: "\xD9", Uacute: "\xDA", Ucirc: "\xDB", Uuml: "\xDC", Yacute: "\xDD", THORN: "\xDE", szlig: "\xDF", agrave: "\xE0", aacute: "\xE1", acirc: "\xE2", atilde: "\xE3", auml: "\xE4", aring: "\xE5", aelig: "\xE6", ccedil: "\xE7", egrave: "\xE8", eacute: "\xE9", ecirc: "\xEA", euml: "\xEB", igrave: "\xEC", iacute: "\xED", icirc: "\xEE", iuml: "\xEF", eth: "\xF0", ntilde: "\xF1", ograve: "\xF2", oacute: "\xF3", ocirc: "\xF4", otilde: "\xF5", ouml: "\xF6", divide: "\xF7", oslash: "\xF8", ugrave: "\xF9", uacute: "\xFA", ucirc: "\xFB", uuml: "\xFC", yacute: "\xFD", thorn: "\xFE", yuml: "\xFF", OElig: "\u0152", oelig: "\u0153", Scaron: "\u0160", scaron: "\u0161", Yuml: "\u0178", fnof: "\u0192", circ: "\u02C6", tilde: "\u02DC", Alpha: "\u0391", Beta: "\u0392", Gamma: "\u0393", Delta: "\u0394", Epsilon: "\u0395", Zeta: "\u0396", Eta: "\u0397", Theta: "\u0398", Iota: "\u0399", Kappa: "\u039A", Lambda: "\u039B", Mu: "\u039C", Nu: "\u039D", Xi: "\u039E", Omicron: "\u039F", Pi: "\u03A0", Rho: "\u03A1", Sigma: "\u03A3", Tau: "\u03A4", Upsilon: "\u03A5", Phi: "\u03A6", Chi: "\u03A7", Psi: "\u03A8", Omega: "\u03A9", alpha: "\u03B1", beta: "\u03B2", gamma: "\u03B3", delta: "\u03B4", epsilon: "\u03B5", zeta: "\u03B6", eta: "\u03B7", theta: "\u03B8", iota: "\u03B9", kappa: "\u03BA", lambda: "\u03BB", mu: "\u03BC", nu: "\u03BD", xi: "\u03BE", omicron: "\u03BF", pi: "\u03C0", rho: "\u03C1", sigmaf: "\u03C2", sigma: "\u03C3", tau: "\u03C4", upsilon: "\u03C5", phi: "\u03C6", chi: "\u03C7", psi: "\u03C8", omega: "\u03C9", thetasym: "\u03D1", upsih: "\u03D2", piv: "\u03D6", ensp: "\u2002", emsp: "\u2003", thinsp: "\u2009", zwnj: "\u200C", zwj: "\u200D", lrm: "\u200E", rlm: "\u200F", ndash: "\u2013", mdash: "\u2014", lsquo: "\u2018", rsquo: "\u2019", sbquo: "\u201A", ldquo: "\u201C", rdquo: "\u201D", bdquo: "\u201E", dagger: "\u2020", Dagger: "\u2021", bull: "\u2022", hellip: "\u2026", permil: "\u2030", prime: "\u2032", Prime: "\u2033", lsaquo: "\u2039", rsaquo: "\u203A", oline: "\u203E", frasl: "\u2044", euro: "\u20AC", image: "\u2111", weierp: "\u2118", real: "\u211C", trade: "\u2122", alefsym: "\u2135", larr: "\u2190", uarr: "\u2191", rarr: "\u2192", darr: "\u2193", harr: "\u2194", crarr: "\u21B5", lArr: "\u21D0", uArr: "\u21D1", rArr: "\u21D2", dArr: "\u21D3", hArr: "\u21D4", forall: "\u2200", part: "\u2202", exist: "\u2203", empty: "\u2205", nabla: "\u2207", isin: "\u2208", notin: "\u2209", ni: "\u220B", prod: "\u220F", sum: "\u2211", minus: "\u2212", lowast: "\u2217", radic: "\u221A", prop: "\u221D", infin: "\u221E", ang: "\u2220", and: "\u2227", or: "\u2228", cap: "\u2229", cup: "\u222A", int: "\u222B", there4: "\u2234", sim: "\u223C", cong: "\u2245", asymp: "\u2248", ne: "\u2260", equiv: "\u2261", le: "\u2264", ge: "\u2265", sub: "\u2282", sup: "\u2283", nsub: "\u2284", sube: "\u2286", supe: "\u2287", oplus: "\u2295", otimes: "\u2297", perp: "\u22A5", sdot: "\u22C5", lceil: "\u2308", rceil: "\u2309", lfloor: "\u230A", rfloor: "\u230B", lang: "\u2329", rang: "\u232A", loz: "\u25CA", spades: "\u2660", clubs: "\u2663", hearts: "\u2665", diams: "\u2666" }, H1 = /^[\da-fA-F]+$/, U1 = /^\d+$/;
function xo(e) {
  return e && (e.type === "JSXIdentifier" ? e.name : e.type === "JSXNamespacedName" ? e.namespace.name + ":" + e.name.name : e.type === "JSXMemberExpression" ? xo(e.object) + "." + xo(e.property) : void 0);
}
var dl = /(?:\s|\/\/.*|\/\*[^]*?\*\/)*/g;
function Yl(e) {
  if (!e) throw new Error("Assert fail");
}
function G1(e) {
  return e === "accessor";
}
function z1(e) {
  return e === "in" || e === "out";
}
function pl(e, t) {
  return 2 | (e ? 4 : 0) | (t ? 8 : 0);
}
function W1(e) {
  if (e.type !== "MemberExpression") return false;
  var t = e.property;
  return (!e.computed || !(t.type !== "TemplateLiteral" || t.expressions.length > 0)) && pm(e.object);
}
function pm(e) {
  return e.type === "Identifier" || e.type === "MemberExpression" && !e.computed && pm(e.object);
}
function Zl(e) {
  return e === "private" || e === "public" || e === "protected";
}
function X1(e) {
  var t = e || {}, a = t.dts, s = a !== void 0 && a, u = t.allowSatisfies, p = u !== void 0 && u;
  return function(m) {
    var w = m.acorn || D1, x = B1(w), o = w.tokTypes, y = w.keywordTypes, O = w.isIdentifierStart, L = w.lineBreak, B = w.isNewLine, G = w.tokContexts, j = w.isIdentifierChar, R = x.tokTypes, h = x.tokContexts, v = x.keywordsRegExp, g = x.tokenIsLiteralPropertyName, S = x.tokenIsTemplate, C = x.tokenIsTSDeclarationStart, _ = x.tokenIsIdentifier, k = x.tokenIsKeywordOrIdentifier, N = x.tokenIsTSTypeOperator;
    function W(A, F, E) {
      E === void 0 && (E = A.length);
      for (var M = F; M < E; M++) {
        var V = A.charCodeAt(M);
        if (B(V)) return M < E - 1 && V === 13 && A.charCodeAt(M + 1) === 10 ? M + 2 : M + 1;
      }
      return -1;
    }
    m = function(A, F, E) {
      var M = E.tokTypes, V = F.tokTypes;
      return function(c) {
        function r() {
          return c.apply(this, arguments) || this;
        }
        Oo(r, c);
        var i = r.prototype;
        return i.takeDecorators = function(n) {
          var l = this.decoratorStack[this.decoratorStack.length - 1];
          l.length && (n.decorators = l, this.resetStartLocationFromNode(n, l[0]), this.decoratorStack[this.decoratorStack.length - 1] = []);
        }, i.parseDecorators = function(n) {
          for (var l = this.decoratorStack[this.decoratorStack.length - 1]; this.match(V.at); ) {
            var f = this.parseDecorator();
            l.push(f);
          }
          this.match(M._export) ? n || this.unexpected() : this.canHaveLeadingDecorator() || this.raise(this.start, "Leading decorators must be attached to a class declaration.");
        }, i.parseDecorator = function() {
          var n = this.startNode();
          this.next(), this.decoratorStack.push([]);
          var l, f = this.start, b = this.startLoc;
          if (this.match(M.parenL)) {
            var P = this.start, q = this.startLoc;
            if (this.next(), l = this.parseExpression(), this.expect(M.parenR), this.options.preserveParens) {
              var T = this.startNodeAt(P, q);
              T.expression = l, l = this.finishNode(T, "ParenthesizedExpression");
            }
          } else for (l = this.parseIdent(false); this.eat(M.dot); ) {
            var H = this.startNodeAt(f, b);
            H.object = l, H.property = this.parseIdent(true), H.computed = false, l = this.finishNode(H, "MemberExpression");
          }
          return n.expression = this.parseMaybeDecoratorArguments(l), this.decoratorStack.pop(), this.finishNode(n, "Decorator");
        }, i.parseMaybeDecoratorArguments = function(n) {
          if (this.eat(M.parenL)) {
            var l = this.startNodeAtNode(n);
            return l.callee = n, l.arguments = this.parseExprList(M.parenR, false), this.finishNode(l, "CallExpression");
          }
          return n;
        }, r;
      }(A);
    }(m, x, w), m = function(A, F, E, M) {
      var V = A.tokTypes, c = F.tokTypes, r = A.isNewLine, i = A.isIdentifierChar, n = Object.assign({ allowNamespaces: true, allowNamespacedObjects: true }, M || {});
      return function(l) {
        function f() {
          return l.apply(this, arguments) || this;
        }
        Oo(f, l);
        var b = f.prototype;
        return b.jsx_readToken = function() {
          for (var P = "", q = this.pos; ; ) {
            this.pos >= this.input.length && this.raise(this.start, "Unterminated JSX contents");
            var T = this.input.charCodeAt(this.pos);
            switch (T) {
              case 60:
              case 123:
                return this.pos === this.start ? T === 60 && this.exprAllowed ? (++this.pos, this.finishToken(c.jsxTagStart)) : this.getTokenFromCode(T) : (P += this.input.slice(q, this.pos), this.finishToken(c.jsxText, P));
              case 38:
                P += this.input.slice(q, this.pos), P += this.jsx_readEntity(), q = this.pos;
                break;
              case 62:
              case 125:
                this.raise(this.pos, "Unexpected token `" + this.input[this.pos] + "`. Did you mean `" + (T === 62 ? "&gt;" : "&rbrace;") + '` or `{"' + this.input[this.pos] + '"}`?');
              default:
                r(T) ? (P += this.input.slice(q, this.pos), P += this.jsx_readNewLine(true), q = this.pos) : ++this.pos;
            }
          }
        }, b.jsx_readNewLine = function(P) {
          var q, T = this.input.charCodeAt(this.pos);
          return ++this.pos, T === 13 && this.input.charCodeAt(this.pos) === 10 ? (++this.pos, q = P ? `
` : `\r
`) : q = String.fromCharCode(T), this.options.locations && (++this.curLine, this.lineStart = this.pos), q;
        }, b.jsx_readString = function(P) {
          for (var q = "", T = ++this.pos; ; ) {
            this.pos >= this.input.length && this.raise(this.start, "Unterminated string constant");
            var H = this.input.charCodeAt(this.pos);
            if (H === P) break;
            H === 38 ? (q += this.input.slice(T, this.pos), q += this.jsx_readEntity(), T = this.pos) : r(H) ? (q += this.input.slice(T, this.pos), q += this.jsx_readNewLine(false), T = this.pos) : ++this.pos;
          }
          return q += this.input.slice(T, this.pos++), this.finishToken(V.string, q);
        }, b.jsx_readEntity = function() {
          var P, q = "", T = 0, H = this.input[this.pos];
          H !== "&" && this.raise(this.pos, "Entity must start with an ampersand");
          for (var J = ++this.pos; this.pos < this.input.length && T++ < 10; ) {
            if ((H = this.input[this.pos++]) === ";") {
              q[0] === "#" ? q[1] === "x" ? (q = q.substr(2), H1.test(q) && (P = String.fromCharCode(parseInt(q, 16)))) : (q = q.substr(1), U1.test(q) && (P = String.fromCharCode(parseInt(q, 10)))) : P = $1[q];
              break;
            }
            q += H;
          }
          return P || (this.pos = J, "&");
        }, b.jsx_readWord = function() {
          var P, q = this.pos;
          do
            P = this.input.charCodeAt(++this.pos);
          while (i(P) || P === 45);
          return this.finishToken(c.jsxName, this.input.slice(q, this.pos));
        }, b.jsx_parseIdentifier = function() {
          var P = this.startNode();
          return this.type === c.jsxName ? P.name = this.value : this.type.keyword ? P.name = this.type.keyword : this.unexpected(), this.next(), this.finishNode(P, "JSXIdentifier");
        }, b.jsx_parseNamespacedName = function() {
          var P = this.start, q = this.startLoc, T = this.jsx_parseIdentifier();
          if (!n.allowNamespaces || !this.eat(V.colon)) return T;
          var H = this.startNodeAt(P, q);
          return H.namespace = T, H.name = this.jsx_parseIdentifier(), this.finishNode(H, "JSXNamespacedName");
        }, b.jsx_parseElementName = function() {
          if (this.type === c.jsxTagEnd) return "";
          var P = this.start, q = this.startLoc, T = this.jsx_parseNamespacedName();
          for (this.type !== V.dot || T.type !== "JSXNamespacedName" || n.allowNamespacedObjects || this.unexpected(); this.eat(V.dot); ) {
            var H = this.startNodeAt(P, q);
            H.object = T, H.property = this.jsx_parseIdentifier(), T = this.finishNode(H, "JSXMemberExpression");
          }
          return T;
        }, b.jsx_parseAttributeValue = function() {
          switch (this.type) {
            case V.braceL:
              var P = this.jsx_parseExpressionContainer();
              return P.expression.type === "JSXEmptyExpression" && this.raise(P.start, "JSX attributes must only be assigned a non-empty expression"), P;
            case c.jsxTagStart:
            case V.string:
              return this.parseExprAtom();
            default:
              this.raise(this.start, "JSX value should be either an expression or a quoted JSX text");
          }
        }, b.jsx_parseEmptyExpression = function() {
          var P = this.startNodeAt(this.lastTokEnd, this.lastTokEndLoc);
          return this.finishNodeAt(P, "JSXEmptyExpression", this.start, this.startLoc);
        }, b.jsx_parseExpressionContainer = function() {
          var P = this.startNode();
          return this.next(), P.expression = this.type === V.braceR ? this.jsx_parseEmptyExpression() : this.parseExpression(), this.expect(V.braceR), this.finishNode(P, "JSXExpressionContainer");
        }, b.jsx_parseAttribute = function() {
          var P = this.startNode();
          return this.eat(V.braceL) ? (this.expect(V.ellipsis), P.argument = this.parseMaybeAssign(), this.expect(V.braceR), this.finishNode(P, "JSXSpreadAttribute")) : (P.name = this.jsx_parseNamespacedName(), P.value = this.eat(V.eq) ? this.jsx_parseAttributeValue() : null, this.finishNode(P, "JSXAttribute"));
        }, b.jsx_parseOpeningElementAt = function(P, q) {
          var T = this.startNodeAt(P, q);
          T.attributes = [];
          var H = this.jsx_parseElementName();
          for (H && (T.name = H); this.type !== V.slash && this.type !== c.jsxTagEnd; ) T.attributes.push(this.jsx_parseAttribute());
          return T.selfClosing = this.eat(V.slash), this.expect(c.jsxTagEnd), this.finishNode(T, H ? "JSXOpeningElement" : "JSXOpeningFragment");
        }, b.jsx_parseClosingElementAt = function(P, q) {
          var T = this.startNodeAt(P, q), H = this.jsx_parseElementName();
          return H && (T.name = H), this.expect(c.jsxTagEnd), this.finishNode(T, H ? "JSXClosingElement" : "JSXClosingFragment");
        }, b.jsx_parseElementAt = function(P, q) {
          var T = this.startNodeAt(P, q), H = [], J = this.jsx_parseOpeningElementAt(P, q), re = null;
          if (!J.selfClosing) {
            e: for (; ; ) switch (this.type) {
              case c.jsxTagStart:
                if (P = this.start, q = this.startLoc, this.next(), this.eat(V.slash)) {
                  re = this.jsx_parseClosingElementAt(P, q);
                  break e;
                }
                H.push(this.jsx_parseElementAt(P, q));
                break;
              case c.jsxText:
                H.push(this.parseExprAtom());
                break;
              case V.braceL:
                H.push(this.jsx_parseExpressionContainer());
                break;
              default:
                this.unexpected();
            }
            xo(re.name) !== xo(J.name) && this.raise(re.start, "Expected corresponding JSX closing tag for <" + xo(J.name) + ">");
          }
          var Q = J.name ? "Element" : "Fragment";
          return T["opening" + Q] = J, T["closing" + Q] = re, T.children = H, this.type === V.relational && this.value === "<" && this.raise(this.start, "Adjacent JSX elements must be wrapped in an enclosing tag"), this.finishNode(T, "JSX" + Q);
        }, b.jsx_parseText = function() {
          var P = this.parseLiteral(this.value);
          return P.type = "JSXText", P;
        }, b.jsx_parseElement = function() {
          var P = this.start, q = this.startLoc;
          return this.next(), this.jsx_parseElementAt(P, q);
        }, f;
      }(E);
    }(w, x, m, e == null ? void 0 : e.jsx), m = function(A, F, E) {
      var M = F.tokTypes, V = E.tokTypes;
      return function(c) {
        function r() {
          return c.apply(this, arguments) || this;
        }
        Oo(r, c);
        var i = r.prototype;
        return i.parseMaybeImportAttributes = function(n) {
          if (this.type === V._with || this.type === M.assert) {
            this.next();
            var l = this.parseImportAttributes();
            l && (n.attributes = l);
          }
        }, i.parseImportAttributes = function() {
          this.expect(V.braceL);
          var n = this.parseWithEntries();
          return this.expect(V.braceR), n;
        }, i.parseWithEntries = function() {
          var n = [], l = /* @__PURE__ */ new Set();
          do {
            if (this.type === V.braceR) break;
            var f, b = this.startNode();
            f = this.type === V.string ? this.parseLiteral(this.value) : this.parseIdent(true), this.next(), b.key = f, l.has(b.key.name) && this.raise(this.pos, "Duplicated key in attributes"), l.add(b.key.name), this.type !== V.string && this.raise(this.pos, "Only string is supported as an attribute value"), b.value = this.parseLiteral(this.value), n.push(this.finishNode(b, "ImportAttribute"));
          } while (this.eat(V.comma));
          return n;
        }, r;
      }(A);
    }(m, x, w);
    var ee = function(A) {
      function F(r, i, n) {
        var l;
        return (l = A.call(this, r, i, n) || this).preValue = null, l.preToken = null, l.isLookahead = false, l.isAmbientContext = false, l.inAbstractClass = false, l.inType = false, l.inDisallowConditionalTypesContext = false, l.maybeInArrowParameters = false, l.shouldParseArrowReturnType = void 0, l.shouldParseAsyncArrowReturnType = void 0, l.decoratorStack = [[]], l.importsStack = [[]], l.importOrExportOuterKind = void 0, l.tsParseConstModifier = l.tsParseModifiers.bind(function(f) {
          if (f === void 0) throw new ReferenceError("this hasn't been initialised - super() hasn't been called");
          return f;
        }(l), { allowedModifiers: ["const"], disallowedModifiers: ["in", "out"], errorTemplate: le.InvalidModifierOnTypeParameterPositions }), l;
      }
      Oo(F, A);
      var E, M, V, c = F.prototype;
      return c.getTokenFromCodeInType = function(r) {
        return r === 62 || r === 60 ? this.finishOp(o.relational, 1) : A.prototype.getTokenFromCode.call(this, r);
      }, c.readToken = function(r) {
        if (!this.inType) {
          var i = this.curContext();
          if (i === h.tc_expr) return this.jsx_readToken();
          if (i === h.tc_oTag || i === h.tc_cTag) {
            if (O(r)) return this.jsx_readWord();
            if (r == 62) return ++this.pos, this.finishToken(R.jsxTagEnd);
            if ((r === 34 || r === 39) && i == h.tc_oTag) return this.jsx_readString(r);
          }
          if (r === 60 && this.exprAllowed && this.input.charCodeAt(this.pos + 1) !== 33) return ++this.pos, this.finishToken(R.jsxTagStart);
        }
        return A.prototype.readToken.call(this, r);
      }, c.getTokenFromCode = function(r) {
        return this.inType ? this.getTokenFromCodeInType(r) : r === 64 ? (++this.pos, this.finishToken(R.at)) : A.prototype.getTokenFromCode.call(this, r);
      }, c.isAbstractClass = function() {
        return this.ts_isContextual(R.abstract) && this.lookahead().type === o._class;
      }, c.finishNode = function(r, i) {
        return r.type !== "" && r.end !== 0 ? r : A.prototype.finishNode.call(this, r, i);
      }, c.tryParse = function(r, i) {
        i === void 0 && (i = this.cloneCurLookaheadState());
        var n = { node: null };
        try {
          return { node: r(function(f) {
            throw f === void 0 && (f = null), n.node = f, n;
          }), error: null, thrown: false, aborted: false, failState: null };
        } catch (f) {
          var l = this.getCurLookaheadState();
          if (this.setLookaheadState(i), f instanceof SyntaxError) return { node: null, error: f, thrown: true, aborted: false, failState: l };
          if (f === n) return { node: n.node, error: null, thrown: false, aborted: true, failState: l };
          throw f;
        }
      }, c.setOptionalParametersError = function(r, i) {
        var n;
        r.optionalParametersLoc = (n = i == null ? void 0 : i.loc) != null ? n : this.startLoc;
      }, c.reScan_lt_gt = function() {
        this.type === o.relational && (this.pos -= 1, this.readToken_lt_gt(this.fullCharCodeAtPos()));
      }, c.reScan_lt = function() {
        var r = this.type;
        return r === o.bitShift ? (this.pos -= 2, this.finishOp(o.relational, 1), o.relational) : r;
      }, c.resetEndLocation = function(r, i) {
        i === void 0 && (i = this.lastTokEndLoc), r.end = i.column, r.loc.end = i, this.options.ranges && (r.range[1] = i.column);
      }, c.startNodeAtNode = function(r) {
        return A.prototype.startNodeAt.call(this, r.start, r.loc.start);
      }, c.nextTokenStart = function() {
        return this.nextTokenStartSince(this.pos);
      }, c.tsHasSomeModifiers = function(r, i) {
        return i.some(function(n) {
          return Zl(n) ? r.accessibility === n : !!r[n];
        });
      }, c.tsIsStartOfStaticBlocks = function() {
        return this.isContextual("static") && this.lookaheadCharCode() === 123;
      }, c.tsCheckForInvalidTypeCasts = function(r) {
        var i = this;
        r.forEach(function(n) {
          (n == null ? void 0 : n.type) === "TSTypeCastExpression" && i.raise(n.typeAnnotation.start, le.UnexpectedTypeAnnotation);
        });
      }, c.atPossibleAsyncArrow = function(r) {
        return r.type === "Identifier" && r.name === "async" && this.lastTokEndLoc.column === r.end && !this.canInsertSemicolon() && r.end - r.start == 5 && r.start === this.potentialArrowAt;
      }, c.tsIsIdentifier = function() {
        return _(this.type);
      }, c.tsTryParseTypeOrTypePredicateAnnotation = function() {
        return this.match(o.colon) ? this.tsParseTypeOrTypePredicateAnnotation(o.colon) : void 0;
      }, c.tsTryParseGenericAsyncArrowFunction = function(r, i, n) {
        var l = this;
        if (this.tsMatchLeftRelational()) {
          var f = this.maybeInArrowParameters;
          this.maybeInArrowParameters = true;
          var b = this.tsTryParseAndCatch(function() {
            var P = l.startNodeAt(r, i);
            return P.typeParameters = l.tsParseTypeParameters(), A.prototype.parseFunctionParams.call(l, P), P.returnType = l.tsTryParseTypeOrTypePredicateAnnotation(), l.expect(o.arrow), P;
          });
          if (this.maybeInArrowParameters = f, b) return A.prototype.parseArrowExpression.call(this, b, null, true, n);
        }
      }, c.tsParseTypeArgumentsInExpression = function() {
        if (this.reScan_lt() === o.relational) return this.tsParseTypeArguments();
      }, c.tsInNoContext = function(r) {
        var i = this.context;
        this.context = [i[0]];
        try {
          return r();
        } finally {
          this.context = i;
        }
      }, c.tsTryParseTypeAnnotation = function() {
        return this.match(o.colon) ? this.tsParseTypeAnnotation() : void 0;
      }, c.isUnparsedContextual = function(r, i) {
        var n = r + i.length;
        if (this.input.slice(r, n) === i) {
          var l = this.input.charCodeAt(n);
          return !(j(l) || (64512 & l) == 55296);
        }
        return false;
      }, c.isAbstractConstructorSignature = function() {
        return this.ts_isContextual(R.abstract) && this.lookahead().type === o._new;
      }, c.nextTokenStartSince = function(r) {
        return dl.lastIndex = r, dl.test(this.input) ? dl.lastIndex : r;
      }, c.lookaheadCharCode = function() {
        return this.input.charCodeAt(this.nextTokenStart());
      }, c.compareLookaheadState = function(r, i) {
        for (var n = 0, l = Object.keys(r); n < l.length; n++) {
          var f = l[n];
          if (r[f] !== i[f]) return false;
        }
        return true;
      }, c.createLookaheadState = function() {
        this.value = null, this.context = [this.curContext()];
      }, c.getCurLookaheadState = function() {
        return { endLoc: this.endLoc, lastTokEnd: this.lastTokEnd, lastTokStart: this.lastTokStart, lastTokStartLoc: this.lastTokStartLoc, pos: this.pos, value: this.value, type: this.type, start: this.start, end: this.end, context: this.context, startLoc: this.startLoc, lastTokEndLoc: this.lastTokEndLoc, curLine: this.curLine, lineStart: this.lineStart, curPosition: this.curPosition, containsEsc: this.containsEsc };
      }, c.cloneCurLookaheadState = function() {
        return { pos: this.pos, value: this.value, type: this.type, start: this.start, end: this.end, context: this.context && this.context.slice(), startLoc: this.startLoc, lastTokEndLoc: this.lastTokEndLoc, endLoc: this.endLoc, lastTokEnd: this.lastTokEnd, lastTokStart: this.lastTokStart, lastTokStartLoc: this.lastTokStartLoc, curLine: this.curLine, lineStart: this.lineStart, curPosition: this.curPosition, containsEsc: this.containsEsc };
      }, c.setLookaheadState = function(r) {
        this.pos = r.pos, this.value = r.value, this.endLoc = r.endLoc, this.lastTokEnd = r.lastTokEnd, this.lastTokStart = r.lastTokStart, this.lastTokStartLoc = r.lastTokStartLoc, this.type = r.type, this.start = r.start, this.end = r.end, this.context = r.context, this.startLoc = r.startLoc, this.lastTokEndLoc = r.lastTokEndLoc, this.curLine = r.curLine, this.lineStart = r.lineStart, this.curPosition = r.curPosition, this.containsEsc = r.containsEsc;
      }, c.tsLookAhead = function(r) {
        var i = this.getCurLookaheadState(), n = r();
        return this.setLookaheadState(i), n;
      }, c.lookahead = function(r) {
        var i = this.getCurLookaheadState();
        if (this.createLookaheadState(), this.isLookahead = true, r !== void 0) for (var n = 0; n < r; n++) this.nextToken();
        else this.nextToken();
        this.isLookahead = false;
        var l = this.getCurLookaheadState();
        return this.setLookaheadState(i), l;
      }, c.readWord = function() {
        var r = this.readWord1(), i = o.name;
        return this.keywords.test(r) ? i = y[r] : new RegExp(v).test(r) && (i = R[r]), this.finishToken(i, r);
      }, c.skipBlockComment = function() {
        var r;
        this.isLookahead || (r = this.options.onComment && this.curPosition());
        var i = this.pos, n = this.input.indexOf("*/", this.pos += 2);
        if (n === -1 && this.raise(this.pos - 2, "Unterminated comment"), this.pos = n + 2, this.options.locations) for (var l, f = i; (l = W(this.input, f, this.pos)) > -1; ) ++this.curLine, f = this.lineStart = l;
        this.isLookahead || this.options.onComment && this.options.onComment(true, this.input.slice(i + 2, n), i, this.pos, r, this.curPosition());
      }, c.skipLineComment = function(r) {
        var i, n = this.pos;
        this.isLookahead || (i = this.options.onComment && this.curPosition());
        for (var l = this.input.charCodeAt(this.pos += r); this.pos < this.input.length && !B(l); ) l = this.input.charCodeAt(++this.pos);
        this.isLookahead || this.options.onComment && this.options.onComment(false, this.input.slice(n + r, this.pos), n, this.pos, i, this.curPosition());
      }, c.finishToken = function(r, i) {
        this.preValue = this.value, this.preToken = this.type, this.end = this.pos, this.options.locations && (this.endLoc = this.curPosition());
        var n = this.type;
        this.type = r, this.value = i, this.isLookahead || this.updateContext(n);
      }, c.resetStartLocation = function(r, i, n) {
        r.start = i, r.loc.start = n, this.options.ranges && (r.range[0] = i);
      }, c.isLineTerminator = function() {
        return this.eat(o.semi) || A.prototype.canInsertSemicolon.call(this);
      }, c.hasFollowingLineBreak = function() {
        return Jl.lastIndex = this.end, Jl.test(this.input);
      }, c.addExtra = function(r, i, n, l) {
        if (l === void 0 && (l = true), r) {
          var f = r.extra = r.extra || {};
          l ? f[i] = n : Object.defineProperty(f, i, { enumerable: l, value: n });
        }
      }, c.isLiteralPropertyName = function() {
        return g(this.type);
      }, c.hasPrecedingLineBreak = function() {
        return L.test(this.input.slice(this.lastTokEndLoc.index, this.start));
      }, c.createIdentifier = function(r, i) {
        return r.name = i, this.finishNode(r, "Identifier");
      }, c.resetStartLocationFromNode = function(r, i) {
        this.resetStartLocation(r, i.start, i.loc.start);
      }, c.isThisParam = function(r) {
        return r.type === "Identifier" && r.name === "this";
      }, c.isLookaheadContextual = function(r) {
        var i = this.nextTokenStart();
        return this.isUnparsedContextual(i, r);
      }, c.ts_type_isContextual = function(r, i) {
        return r === i && !this.containsEsc;
      }, c.ts_isContextual = function(r) {
        return this.type === r && !this.containsEsc;
      }, c.ts_isContextualWithState = function(r, i) {
        return r.type === i && !r.containsEsc;
      }, c.isContextualWithState = function(r, i) {
        return i.type === o.name && i.value === r && !i.containsEsc;
      }, c.tsIsStartOfMappedType = function() {
        return this.next(), this.eat(o.plusMin) ? this.ts_isContextual(R.readonly) : (this.ts_isContextual(R.readonly) && this.next(), !!this.match(o.bracketL) && (this.next(), !!this.tsIsIdentifier() && (this.next(), this.match(o._in))));
      }, c.tsInDisallowConditionalTypesContext = function(r) {
        var i = this.inDisallowConditionalTypesContext;
        this.inDisallowConditionalTypesContext = true;
        try {
          return r();
        } finally {
          this.inDisallowConditionalTypesContext = i;
        }
      }, c.tsTryParseType = function() {
        return this.tsEatThenParseType(o.colon);
      }, c.match = function(r) {
        return this.type === r;
      }, c.matchJsx = function(r) {
        return this.type === x.tokTypes[r];
      }, c.ts_eatWithState = function(r, i, n) {
        if (r === n.type) {
          for (var l = 0; l < i; l++) this.next();
          return true;
        }
        return false;
      }, c.ts_eatContextualWithState = function(r, i, n) {
        if (v.test(r)) {
          if (this.ts_isContextualWithState(n, R[r])) {
            for (var l = 0; l < i; l++) this.next();
            return true;
          }
          return false;
        }
        if (!this.isContextualWithState(r, n)) return false;
        for (var f = 0; f < i; f++) this.next();
        return true;
      }, c.canHaveLeadingDecorator = function() {
        return this.match(o._class);
      }, c.eatContextual = function(r) {
        return v.test(r) ? !!this.ts_isContextual(R[r]) && (this.next(), true) : A.prototype.eatContextual.call(this, r);
      }, c.tsIsExternalModuleReference = function() {
        return this.isContextual("require") && this.lookaheadCharCode() === 40;
      }, c.tsParseExternalModuleReference = function() {
        var r = this.startNode();
        return this.expectContextual("require"), this.expect(o.parenL), this.match(o.string) || this.unexpected(), r.expression = this.parseExprAtom(), this.expect(o.parenR), this.finishNode(r, "TSExternalModuleReference");
      }, c.tsParseEntityName = function(r) {
        r === void 0 && (r = true);
        for (var i = this.parseIdent(r); this.eat(o.dot); ) {
          var n = this.startNodeAtNode(i);
          n.left = i, n.right = this.parseIdent(r), i = this.finishNode(n, "TSQualifiedName");
        }
        return i;
      }, c.tsParseEnumMember = function() {
        var r = this.startNode();
        return r.id = this.match(o.string) ? this.parseLiteral(this.value) : this.parseIdent(true), this.eat(o.eq) && (r.initializer = this.parseMaybeAssign()), this.finishNode(r, "TSEnumMember");
      }, c.tsParseEnumDeclaration = function(r, i) {
        return i === void 0 && (i = {}), i.const && (r.const = true), i.declare && (r.declare = true), this.expectContextual("enum"), r.id = this.parseIdent(), this.checkLValSimple(r.id), this.expect(o.braceL), r.members = this.tsParseDelimitedList("EnumMembers", this.tsParseEnumMember.bind(this)), this.expect(o.braceR), this.finishNode(r, "TSEnumDeclaration");
      }, c.tsParseModuleBlock = function() {
        var r = this.startNode();
        for (A.prototype.enterScope.call(this, 512), this.expect(o.braceL), r.body = []; this.type !== o.braceR; ) {
          var i = this.parseStatement(null, true);
          r.body.push(i);
        }
        return this.next(), A.prototype.exitScope.call(this), this.finishNode(r, "TSModuleBlock");
      }, c.tsParseAmbientExternalModuleDeclaration = function(r) {
        return this.ts_isContextual(R.global) ? (r.global = true, r.id = this.parseIdent()) : this.match(o.string) ? r.id = this.parseLiteral(this.value) : this.unexpected(), this.match(o.braceL) ? (A.prototype.enterScope.call(this, ir), r.body = this.tsParseModuleBlock(), A.prototype.exitScope.call(this)) : A.prototype.semicolon.call(this), this.finishNode(r, "TSModuleDeclaration");
      }, c.tsTryParseDeclare = function(r) {
        var i = this;
        if (!this.isLineTerminator()) {
          var n, l = this.type;
          return this.isContextual("let") && (l = o._var, n = "let"), this.tsInAmbientContext(function() {
            if (l === o._function) return r.declare = true, i.parseFunctionStatement(r, false, true);
            if (l === o._class) return r.declare = true, i.parseClass(r, true);
            if (l === R.enum) return i.tsParseEnumDeclaration(r, { declare: true });
            if (l === R.global) return i.tsParseAmbientExternalModuleDeclaration(r);
            if (l === o._const || l === o._var) return i.match(o._const) && i.isLookaheadContextual("enum") ? (i.expect(o._const), i.tsParseEnumDeclaration(r, { const: true, declare: true })) : (r.declare = true, i.parseVarStatement(r, n || i.value, true));
            if (l === R.interface) {
              var f = i.tsParseInterfaceDeclaration(r, { declare: true });
              if (f) return f;
            }
            return _(l) ? i.tsParseDeclaration(r, i.value, true) : void 0;
          });
        }
      }, c.tsIsListTerminator = function(r) {
        switch (r) {
          case "EnumMembers":
          case "TypeMembers":
            return this.match(o.braceR);
          case "HeritageClauseElement":
            return this.match(o.braceL);
          case "TupleElementTypes":
            return this.match(o.bracketR);
          case "TypeParametersOrArguments":
            return this.tsMatchRightRelational();
        }
      }, c.tsParseDelimitedListWorker = function(r, i, n, l) {
        for (var f = [], b = -1; !this.tsIsListTerminator(r); ) {
          b = -1;
          var P = i();
          if (P == null) return;
          if (f.push(P), !this.eat(o.comma)) {
            if (this.tsIsListTerminator(r)) break;
            return void (n && this.expect(o.comma));
          }
          b = this.lastTokStart;
        }
        return l && (l.value = b), f;
      }, c.tsParseDelimitedList = function(r, i, n) {
        return function(l) {
          if (l == null) throw new Error("Unexpected " + l + " value.");
          return l;
        }(this.tsParseDelimitedListWorker(r, i, true, n));
      }, c.tsParseBracketedList = function(r, i, n, l, f) {
        l || this.expect(n ? o.bracketL : o.relational);
        var b = this.tsParseDelimitedList(r, i, f);
        return this.expect(n ? o.bracketR : o.relational), b;
      }, c.tsParseTypeParameterName = function() {
        return this.parseIdent().name;
      }, c.tsEatThenParseType = function(r) {
        return this.match(r) ? this.tsNextThenParseType() : void 0;
      }, c.tsExpectThenParseType = function(r) {
        var i = this;
        return this.tsDoThenParseType(function() {
          return i.expect(r);
        });
      }, c.tsNextThenParseType = function() {
        var r = this;
        return this.tsDoThenParseType(function() {
          return r.next();
        });
      }, c.tsDoThenParseType = function(r) {
        var i = this;
        return this.tsInType(function() {
          return r(), i.tsParseType();
        });
      }, c.tsSkipParameterStart = function() {
        if (_(this.type) || this.match(o._this)) return this.next(), true;
        if (this.match(o.braceL)) try {
          return this.parseObj(true), true;
        } catch {
          return false;
        }
        if (this.match(o.bracketL)) {
          this.next();
          try {
            return this.parseBindingList(o.bracketR, true, true), true;
          } catch {
            return false;
          }
        }
        return false;
      }, c.tsIsUnambiguouslyStartOfFunctionType = function() {
        return this.next(), !!(this.match(o.parenR) || this.match(o.ellipsis) || this.tsSkipParameterStart() && (this.match(o.colon) || this.match(o.comma) || this.match(o.question) || this.match(o.eq) || this.match(o.parenR) && (this.next(), this.match(o.arrow))));
      }, c.tsIsStartOfFunctionType = function() {
        return !!this.tsMatchLeftRelational() || this.match(o.parenL) && this.tsLookAhead(this.tsIsUnambiguouslyStartOfFunctionType.bind(this));
      }, c.tsInAllowConditionalTypesContext = function(r) {
        var i = this.inDisallowConditionalTypesContext;
        this.inDisallowConditionalTypesContext = false;
        try {
          return r();
        } finally {
          this.inDisallowConditionalTypesContext = i;
        }
      }, c.tsParseBindingListForSignature = function() {
        var r = this;
        return A.prototype.parseBindingList.call(this, o.parenR, true, true).map(function(i) {
          return i.type !== "Identifier" && i.type !== "RestElement" && i.type !== "ObjectPattern" && i.type !== "ArrayPattern" && r.raise(i.start, le.UnsupportedSignatureParameterKind(i.type)), i;
        });
      }, c.tsParseTypePredicateAsserts = function() {
        if (this.type !== R.asserts) return false;
        var r = this.containsEsc;
        return this.next(), !(!_(this.type) && !this.match(o._this) || (r && this.raise(this.lastTokStart, "Escape sequence in keyword asserts"), 0));
      }, c.tsParseThisTypeNode = function() {
        var r = this.startNode();
        return this.next(), this.finishNode(r, "TSThisType");
      }, c.tsParseTypeAnnotation = function(r, i) {
        var n = this;
        return r === void 0 && (r = true), i === void 0 && (i = this.startNode()), this.tsInType(function() {
          r && n.expect(o.colon), i.typeAnnotation = n.tsParseType();
        }), this.finishNode(i, "TSTypeAnnotation");
      }, c.tsParseThisTypePredicate = function(r) {
        this.next();
        var i = this.startNodeAtNode(r);
        return i.parameterName = r, i.typeAnnotation = this.tsParseTypeAnnotation(false), i.asserts = false, this.finishNode(i, "TSTypePredicate");
      }, c.tsParseThisTypeOrThisTypePredicate = function() {
        var r = this.tsParseThisTypeNode();
        return this.isContextual("is") && !this.hasPrecedingLineBreak() ? this.tsParseThisTypePredicate(r) : r;
      }, c.tsParseTypePredicatePrefix = function() {
        var r = this.parseIdent();
        if (this.isContextual("is") && !this.hasPrecedingLineBreak()) return this.next(), r;
      }, c.tsParseTypeOrTypePredicateAnnotation = function(r) {
        var i = this;
        return this.tsInType(function() {
          var n = i.startNode();
          i.expect(r);
          var l = i.startNode(), f = !!i.tsTryParse(i.tsParseTypePredicateAsserts.bind(i));
          if (f && i.match(o._this)) {
            var b = i.tsParseThisTypeOrThisTypePredicate();
            return b.type === "TSThisType" ? (l.parameterName = b, l.asserts = true, l.typeAnnotation = null, b = i.finishNode(l, "TSTypePredicate")) : (i.resetStartLocationFromNode(b, l), b.asserts = true), n.typeAnnotation = b, i.finishNode(n, "TSTypeAnnotation");
          }
          var P = i.tsIsIdentifier() && i.tsTryParse(i.tsParseTypePredicatePrefix.bind(i));
          if (!P) return f ? (l.parameterName = i.parseIdent(), l.asserts = f, l.typeAnnotation = null, n.typeAnnotation = i.finishNode(l, "TSTypePredicate"), i.finishNode(n, "TSTypeAnnotation")) : i.tsParseTypeAnnotation(false, n);
          var q = i.tsParseTypeAnnotation(false);
          return l.parameterName = P, l.typeAnnotation = q, l.asserts = f, n.typeAnnotation = i.finishNode(l, "TSTypePredicate"), i.finishNode(n, "TSTypeAnnotation");
        });
      }, c.tsFillSignature = function(r, i) {
        var n = r === o.arrow;
        i.typeParameters = this.tsTryParseTypeParameters(), this.expect(o.parenL), i.parameters = this.tsParseBindingListForSignature(), (n || this.match(r)) && (i.typeAnnotation = this.tsParseTypeOrTypePredicateAnnotation(r));
      }, c.tsTryNextParseConstantContext = function() {
        if (this.lookahead().type !== o._const) return null;
        this.next();
        var r = this.tsParseTypeReference();
        return r.typeParameters && this.raise(r.typeName.start, le.CannotFindName({ name: "const" })), r;
      }, c.tsParseFunctionOrConstructorType = function(r, i) {
        var n = this, l = this.startNode();
        return r === "TSConstructorType" && (l.abstract = !!i, i && this.next(), this.next()), this.tsInAllowConditionalTypesContext(function() {
          return n.tsFillSignature(o.arrow, l);
        }), this.finishNode(l, r);
      }, c.tsParseUnionOrIntersectionType = function(r, i, n) {
        var l = this.startNode(), f = this.eat(n), b = [];
        do
          b.push(i());
        while (this.eat(n));
        return b.length !== 1 || f ? (l.types = b, this.finishNode(l, r)) : b[0];
      }, c.tsCheckTypeAnnotationForReadOnly = function(r) {
        switch (r.typeAnnotation.type) {
          case "TSTupleType":
          case "TSArrayType":
            return;
          default:
            this.raise(r.start, le.UnexpectedReadonly);
        }
      }, c.tsParseTypeOperator = function() {
        var r = this.startNode(), i = this.value;
        return this.next(), r.operator = i, r.typeAnnotation = this.tsParseTypeOperatorOrHigher(), i === "readonly" && this.tsCheckTypeAnnotationForReadOnly(r), this.finishNode(r, "TSTypeOperator");
      }, c.tsParseConstraintForInferType = function() {
        var r = this;
        if (this.eat(o._extends)) {
          var i = this.tsInDisallowConditionalTypesContext(function() {
            return r.tsParseType();
          });
          if (this.inDisallowConditionalTypesContext || !this.match(o.question)) return i;
        }
      }, c.tsParseInferType = function() {
        var r = this, i = this.startNode();
        this.expectContextual("infer");
        var n = this.startNode();
        return n.name = this.tsParseTypeParameterName(), n.constraint = this.tsTryParse(function() {
          return r.tsParseConstraintForInferType();
        }), i.typeParameter = this.finishNode(n, "TSTypeParameter"), this.finishNode(i, "TSInferType");
      }, c.tsParseLiteralTypeNode = function() {
        var r = this, i = this.startNode();
        return i.literal = function() {
          switch (r.type) {
            case o.num:
            case o.string:
            case o._true:
            case o._false:
              return r.parseExprAtom();
            default:
              r.unexpected();
          }
        }(), this.finishNode(i, "TSLiteralType");
      }, c.tsParseImportType = function() {
        var r = this.startNode();
        return this.expect(o._import), this.expect(o.parenL), this.match(o.string) || this.raise(this.start, le.UnsupportedImportTypeArgument), r.argument = this.parseExprAtom(), this.expect(o.parenR), this.eat(o.dot) && (r.qualifier = this.tsParseEntityName()), this.tsMatchLeftRelational() && (r.typeParameters = this.tsParseTypeArguments()), this.finishNode(r, "TSImportType");
      }, c.tsParseTypeQuery = function() {
        var r = this.startNode();
        return this.expect(o._typeof), r.exprName = this.match(o._import) ? this.tsParseImportType() : this.tsParseEntityName(), !this.hasPrecedingLineBreak() && this.tsMatchLeftRelational() && (r.typeParameters = this.tsParseTypeArguments()), this.finishNode(r, "TSTypeQuery");
      }, c.tsParseMappedTypeParameter = function() {
        var r = this.startNode();
        return r.name = this.tsParseTypeParameterName(), r.constraint = this.tsExpectThenParseType(o._in), this.finishNode(r, "TSTypeParameter");
      }, c.tsParseMappedType = function() {
        var r = this.startNode();
        return this.expect(o.braceL), this.match(o.plusMin) ? (r.readonly = this.value, this.next(), this.expectContextual("readonly")) : this.eatContextual("readonly") && (r.readonly = true), this.expect(o.bracketL), r.typeParameter = this.tsParseMappedTypeParameter(), r.nameType = this.eatContextual("as") ? this.tsParseType() : null, this.expect(o.bracketR), this.match(o.plusMin) ? (r.optional = this.value, this.next(), this.expect(o.question)) : this.eat(o.question) && (r.optional = true), r.typeAnnotation = this.tsTryParseType(), this.semicolon(), this.expect(o.braceR), this.finishNode(r, "TSMappedType");
      }, c.tsParseTypeLiteral = function() {
        var r = this.startNode();
        return r.members = this.tsParseObjectTypeMembers(), this.finishNode(r, "TSTypeLiteral");
      }, c.tsParseTupleElementType = function() {
        var r = this.startLoc, i = this.start, n = this.eat(o.ellipsis), l = this.tsParseType(), f = this.eat(o.question);
        if (this.eat(o.colon)) {
          var b = this.startNodeAtNode(l);
          b.optional = f, l.type !== "TSTypeReference" || l.typeParameters || l.typeName.type !== "Identifier" ? (this.raise(l.start, le.InvalidTupleMemberLabel), b.label = l) : b.label = l.typeName, b.elementType = this.tsParseType(), l = this.finishNode(b, "TSNamedTupleMember");
        } else if (f) {
          var P = this.startNodeAtNode(l);
          P.typeAnnotation = l, l = this.finishNode(P, "TSOptionalType");
        }
        if (n) {
          var q = this.startNodeAt(i, r);
          q.typeAnnotation = l, l = this.finishNode(q, "TSRestType");
        }
        return l;
      }, c.tsParseTupleType = function() {
        var r = this, i = this.startNode();
        i.elementTypes = this.tsParseBracketedList("TupleElementTypes", this.tsParseTupleElementType.bind(this), true, false);
        var n = false, l = null;
        return i.elementTypes.forEach(function(f) {
          var b = f.type;
          !n || b === "TSRestType" || b === "TSOptionalType" || b === "TSNamedTupleMember" && f.optional || r.raise(f.start, le.OptionalTypeBeforeRequired), n || (n = b === "TSNamedTupleMember" && f.optional || b === "TSOptionalType");
          var P = b;
          b === "TSRestType" && (P = (f = f.typeAnnotation).type);
          var q = P === "TSNamedTupleMember";
          l != null || (l = q), l !== q && r.raise(f.start, le.MixedLabeledAndUnlabeledElements);
        }), this.finishNode(i, "TSTupleType");
      }, c.tsParseTemplateLiteralType = function() {
        var r = this.startNode();
        return r.literal = this.parseTemplate({ isTagged: false }), this.finishNode(r, "TSLiteralType");
      }, c.tsParseTypeReference = function() {
        var r = this.startNode();
        return r.typeName = this.tsParseEntityName(), !this.hasPrecedingLineBreak() && this.tsMatchLeftRelational() && (r.typeParameters = this.tsParseTypeArguments()), this.finishNode(r, "TSTypeReference");
      }, c.tsMatchLeftRelational = function() {
        return this.match(o.relational) && this.value === "<";
      }, c.tsMatchRightRelational = function() {
        return this.match(o.relational) && this.value === ">";
      }, c.tsParseParenthesizedType = function() {
        var r = this.startNode();
        return this.expect(o.parenL), r.typeAnnotation = this.tsParseType(), this.expect(o.parenR), this.finishNode(r, "TSParenthesizedType");
      }, c.tsParseNonArrayType = function() {
        switch (this.type) {
          case o.string:
          case o.num:
          case o._true:
          case o._false:
            return this.tsParseLiteralTypeNode();
          case o.plusMin:
            if (this.value === "-") {
              var r = this.startNode();
              return this.lookahead().type !== o.num && this.unexpected(), r.literal = this.parseMaybeUnary(), this.finishNode(r, "TSLiteralType");
            }
            break;
          case o._this:
            return this.tsParseThisTypeOrThisTypePredicate();
          case o._typeof:
            return this.tsParseTypeQuery();
          case o._import:
            return this.tsParseImportType();
          case o.braceL:
            return this.tsLookAhead(this.tsIsStartOfMappedType.bind(this)) ? this.tsParseMappedType() : this.tsParseTypeLiteral();
          case o.bracketL:
            return this.tsParseTupleType();
          case o.parenL:
            return this.tsParseParenthesizedType();
          case o.backQuote:
          case o.dollarBraceL:
            return this.tsParseTemplateLiteralType();
          default:
            var i = this.type;
            if (_(i) || i === o._void || i === o._null) {
              var n = i === o._void ? "TSVoidKeyword" : i === o._null ? "TSNullKeyword" : function(f) {
                switch (f) {
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
              }(this.value);
              if (n !== void 0 && this.lookaheadCharCode() !== 46) {
                var l = this.startNode();
                return this.next(), this.finishNode(l, n);
              }
              return this.tsParseTypeReference();
            }
        }
        this.unexpected();
      }, c.tsParseArrayTypeOrHigher = function() {
        for (var r = this.tsParseNonArrayType(); !this.hasPrecedingLineBreak() && this.eat(o.bracketL); ) if (this.match(o.bracketR)) {
          var i = this.startNodeAtNode(r);
          i.elementType = r, this.expect(o.bracketR), r = this.finishNode(i, "TSArrayType");
        } else {
          var n = this.startNodeAtNode(r);
          n.objectType = r, n.indexType = this.tsParseType(), this.expect(o.bracketR), r = this.finishNode(n, "TSIndexedAccessType");
        }
        return r;
      }, c.tsParseTypeOperatorOrHigher = function() {
        var r = this;
        return N(this.type) && !this.containsEsc ? this.tsParseTypeOperator() : this.isContextual("infer") ? this.tsParseInferType() : this.tsInAllowConditionalTypesContext(function() {
          return r.tsParseArrayTypeOrHigher();
        });
      }, c.tsParseIntersectionTypeOrHigher = function() {
        return this.tsParseUnionOrIntersectionType("TSIntersectionType", this.tsParseTypeOperatorOrHigher.bind(this), o.bitwiseAND);
      }, c.tsParseUnionTypeOrHigher = function() {
        return this.tsParseUnionOrIntersectionType("TSUnionType", this.tsParseIntersectionTypeOrHigher.bind(this), o.bitwiseOR);
      }, c.tsParseNonConditionalType = function() {
        return this.tsIsStartOfFunctionType() ? this.tsParseFunctionOrConstructorType("TSFunctionType") : this.match(o._new) ? this.tsParseFunctionOrConstructorType("TSConstructorType") : this.isAbstractConstructorSignature() ? this.tsParseFunctionOrConstructorType("TSConstructorType", true) : this.tsParseUnionTypeOrHigher();
      }, c.tsParseType = function() {
        var r = this;
        Yl(this.inType);
        var i = this.tsParseNonConditionalType();
        if (this.inDisallowConditionalTypesContext || this.hasPrecedingLineBreak() || !this.eat(o._extends)) return i;
        var n = this.startNodeAtNode(i);
        return n.checkType = i, n.extendsType = this.tsInDisallowConditionalTypesContext(function() {
          return r.tsParseNonConditionalType();
        }), this.expect(o.question), n.trueType = this.tsInAllowConditionalTypesContext(function() {
          return r.tsParseType();
        }), this.expect(o.colon), n.falseType = this.tsInAllowConditionalTypesContext(function() {
          return r.tsParseType();
        }), this.finishNode(n, "TSConditionalType");
      }, c.tsIsUnambiguouslyIndexSignature = function() {
        return this.next(), !!_(this.type) && (this.next(), this.match(o.colon));
      }, c.tsInType = function(r) {
        var i = this.inType;
        this.inType = true;
        try {
          return r();
        } finally {
          this.inType = i;
        }
      }, c.tsTryParseIndexSignature = function(r) {
        if (this.match(o.bracketL) && this.tsLookAhead(this.tsIsUnambiguouslyIndexSignature.bind(this))) {
          this.expect(o.bracketL);
          var i = this.parseIdent();
          i.typeAnnotation = this.tsParseTypeAnnotation(), this.resetEndLocation(i), this.expect(o.bracketR), r.parameters = [i];
          var n = this.tsTryParseTypeAnnotation();
          return n && (r.typeAnnotation = n), this.tsParseTypeMemberSemicolon(), this.finishNode(r, "TSIndexSignature");
        }
      }, c.tsParseNoneModifiers = function(r) {
        this.tsParseModifiers({ modified: r, allowedModifiers: [], disallowedModifiers: ["in", "out"], errorTemplate: le.InvalidModifierOnTypeParameterPositions });
      }, c.tsParseTypeParameter = function(r) {
        r === void 0 && (r = this.tsParseNoneModifiers.bind(this));
        var i = this.startNode();
        return r(i), i.name = this.tsParseTypeParameterName(), i.constraint = this.tsEatThenParseType(o._extends), i.default = this.tsEatThenParseType(o.eq), this.finishNode(i, "TSTypeParameter");
      }, c.tsParseTypeParameters = function(r) {
        var i = this.startNode();
        this.tsMatchLeftRelational() || this.matchJsx("jsxTagStart") ? this.next() : this.unexpected();
        var n = { value: -1 };
        return i.params = this.tsParseBracketedList("TypeParametersOrArguments", this.tsParseTypeParameter.bind(this, r), false, true, n), i.params.length === 0 && this.raise(this.start, le.EmptyTypeParameters), n.value !== -1 && this.addExtra(i, "trailingComma", n.value), this.finishNode(i, "TSTypeParameterDeclaration");
      }, c.tsTryParseTypeParameters = function(r) {
        if (this.tsMatchLeftRelational()) return this.tsParseTypeParameters(r);
      }, c.tsTryParse = function(r) {
        var i = this.getCurLookaheadState(), n = r();
        return n !== void 0 && n !== false ? n : void this.setLookaheadState(i);
      }, c.tsTokenCanFollowModifier = function() {
        return (this.match(o.bracketL) || this.match(o.braceL) || this.match(o.star) || this.match(o.ellipsis) || this.match(o.privateId) || this.isLiteralPropertyName()) && !this.hasPrecedingLineBreak();
      }, c.tsNextTokenCanFollowModifier = function() {
        return this.next(true), this.tsTokenCanFollowModifier();
      }, c.tsParseModifier = function(r, i) {
        if (_(this.type) || this.type === o._in) {
          var n = this.value;
          if (r.indexOf(n) !== -1 && !this.containsEsc) {
            if (i && this.tsIsStartOfStaticBlocks()) return;
            if (this.tsTryParse(this.tsNextTokenCanFollowModifier.bind(this))) return n;
          }
        }
      }, c.tsParseModifiersByMap = function(r) {
        for (var i = r.modified, n = r.map, l = 0, f = Object.keys(n); l < f.length; l++) {
          var b = f[l];
          i[b] = n[b];
        }
      }, c.tsParseModifiers = function(r) {
        for (var i = this, n = r.modified, l = r.allowedModifiers, f = r.disallowedModifiers, b = r.stopOnStartOfClassStaticBlock, P = r.errorTemplate, q = P === void 0 ? le.InvalidModifierOnTypeMember : P, T = {}, H = function(de, pe, fe, he) {
          pe === fe && n[he] && i.raise(de.column, le.InvalidModifiersOrder({ orderedModifiers: [fe, he] }));
        }, J = function(de, pe, fe, he) {
          (n[fe] && pe === he || n[he] && pe === fe) && i.raise(de.column, le.IncompatibleModifiers({ modifiers: [fe, he] }));
        }; ; ) {
          var re = this.startLoc, Q = this.tsParseModifier(l.concat(f ?? []), b);
          if (!Q) break;
          Zl(Q) ? n.accessibility ? this.raise(this.start, le.DuplicateAccessibilityModifier()) : (H(re, Q, Q, "override"), H(re, Q, Q, "static"), H(re, Q, Q, "readonly"), H(re, Q, Q, "accessor"), T.accessibility = Q, n.accessibility = Q) : z1(Q) ? n[Q] ? this.raise(this.start, le.DuplicateModifier({ modifier: Q })) : (H(re, Q, "in", "out"), T[Q] = Q, n[Q] = true) : G1(Q) ? n[Q] ? this.raise(this.start, le.DuplicateModifier({ modifier: Q })) : (J(re, Q, "accessor", "readonly"), J(re, Q, "accessor", "static"), J(re, Q, "accessor", "override"), T[Q] = Q, n[Q] = true) : Object.hasOwnProperty.call(n, Q) ? this.raise(this.start, le.DuplicateModifier({ modifier: Q })) : (H(re, Q, "static", "readonly"), H(re, Q, "static", "override"), H(re, Q, "override", "readonly"), H(re, Q, "abstract", "override"), J(re, Q, "declare", "override"), J(re, Q, "static", "abstract"), T[Q] = Q, n[Q] = true), f != null && f.includes(Q) && this.raise(this.start, q);
        }
        return T;
      }, c.tsParseInOutModifiers = function(r) {
        this.tsParseModifiers({ modified: r, allowedModifiers: ["in", "out"], disallowedModifiers: ["public", "private", "protected", "readonly", "declare", "abstract", "override"], errorTemplate: le.InvalidModifierOnTypeParameter });
      }, c.tsParseTypeArguments = function() {
        var r = this, i = this.startNode();
        return i.params = this.tsInType(function() {
          return r.tsInNoContext(function() {
            return r.expect(o.relational), r.tsParseDelimitedList("TypeParametersOrArguments", r.tsParseType.bind(r));
          });
        }), i.params.length === 0 && this.raise(this.start, le.EmptyTypeArguments), this.exprAllowed = false, this.expect(o.relational), this.finishNode(i, "TSTypeParameterInstantiation");
      }, c.tsParseHeritageClause = function(r) {
        var i = this, n = this.start, l = this.tsParseDelimitedList("HeritageClauseElement", function() {
          var f = i.startNode();
          return f.expression = i.tsParseEntityName(), i.tsMatchLeftRelational() && (f.typeParameters = i.tsParseTypeArguments()), i.finishNode(f, "TSExpressionWithTypeArguments");
        });
        return l.length || this.raise(n, le.EmptyHeritageClauseType({ token: r })), l;
      }, c.tsParseTypeMemberSemicolon = function() {
        this.eat(o.comma) || this.isLineTerminator() || this.expect(o.semi);
      }, c.tsTryParseAndCatch = function(r) {
        var i = this.tryParse(function(n) {
          return r() || n();
        });
        if (!i.aborted && i.node) return i.error && this.setLookaheadState(i.failState), i.node;
      }, c.tsParseSignatureMember = function(r, i) {
        return this.tsFillSignature(o.colon, i), this.tsParseTypeMemberSemicolon(), this.finishNode(i, r);
      }, c.tsParsePropertyOrMethodSignature = function(r, i) {
        this.eat(o.question) && (r.optional = true);
        var n = r;
        if (this.match(o.parenL) || this.tsMatchLeftRelational()) {
          i && this.raise(r.start, le.ReadonlyForMethodSignature);
          var l = n;
          l.kind && this.tsMatchLeftRelational() && this.raise(this.start, le.AccesorCannotHaveTypeParameters), this.tsFillSignature(o.colon, l), this.tsParseTypeMemberSemicolon();
          var f = "parameters", b = "typeAnnotation";
          if (l.kind === "get") l[f].length > 0 && (this.raise(this.start, "A 'get' accesor must not have any formal parameters."), this.isThisParam(l[f][0]) && this.raise(this.start, le.AccesorCannotDeclareThisParameter));
          else if (l.kind === "set") {
            if (l[f].length !== 1) this.raise(this.start, "A 'get' accesor must not have any formal parameters.");
            else {
              var P = l[f][0];
              this.isThisParam(P) && this.raise(this.start, le.AccesorCannotDeclareThisParameter), P.type === "Identifier" && P.optional && this.raise(this.start, le.SetAccesorCannotHaveOptionalParameter), P.type === "RestElement" && this.raise(this.start, le.SetAccesorCannotHaveRestParameter);
            }
            l[b] && this.raise(l[b].start, le.SetAccesorCannotHaveReturnType);
          } else l.kind = "method";
          return this.finishNode(l, "TSMethodSignature");
        }
        var q = n;
        i && (q.readonly = true);
        var T = this.tsTryParseTypeAnnotation();
        return T && (q.typeAnnotation = T), this.tsParseTypeMemberSemicolon(), this.finishNode(q, "TSPropertySignature");
      }, c.tsParseTypeMember = function() {
        var r = this.startNode();
        if (this.match(o.parenL) || this.tsMatchLeftRelational()) return this.tsParseSignatureMember("TSCallSignatureDeclaration", r);
        if (this.match(o._new)) {
          var i = this.startNode();
          return this.next(), this.match(o.parenL) || this.tsMatchLeftRelational() ? this.tsParseSignatureMember("TSConstructSignatureDeclaration", r) : (r.key = this.createIdentifier(i, "new"), this.tsParsePropertyOrMethodSignature(r, false));
        }
        return this.tsParseModifiers({ modified: r, allowedModifiers: ["readonly"], disallowedModifiers: ["declare", "abstract", "private", "protected", "public", "static", "override"] }), this.tsTryParseIndexSignature(r) || (this.parsePropertyName(r), r.computed || r.key.type !== "Identifier" || r.key.name !== "get" && r.key.name !== "set" || !this.tsTokenCanFollowModifier() || (r.kind = r.key.name, this.parsePropertyName(r)), this.tsParsePropertyOrMethodSignature(r, !!r.readonly));
      }, c.tsParseList = function(r, i) {
        for (var n = []; !this.tsIsListTerminator(r); ) n.push(i());
        return n;
      }, c.tsParseObjectTypeMembers = function() {
        this.expect(o.braceL);
        var r = this.tsParseList("TypeMembers", this.tsParseTypeMember.bind(this));
        return this.expect(o.braceR), r;
      }, c.tsParseInterfaceDeclaration = function(r, i) {
        if (i === void 0 && (i = {}), this.hasFollowingLineBreak()) return null;
        this.expectContextual("interface"), i.declare && (r.declare = true), _(this.type) ? (r.id = this.parseIdent(), this.checkLValSimple(r.id, 7)) : (r.id = null, this.raise(this.start, le.MissingInterfaceName)), r.typeParameters = this.tsTryParseTypeParameters(this.tsParseInOutModifiers.bind(this)), this.eat(o._extends) && (r.extends = this.tsParseHeritageClause("extends"));
        var n = this.startNode();
        return n.body = this.tsInType(this.tsParseObjectTypeMembers.bind(this)), r.body = this.finishNode(n, "TSInterfaceBody"), this.finishNode(r, "TSInterfaceDeclaration");
      }, c.tsParseAbstractDeclaration = function(r) {
        if (this.match(o._class)) return r.abstract = true, this.parseClass(r, true);
        if (this.ts_isContextual(R.interface)) {
          if (!this.hasFollowingLineBreak()) return r.abstract = true, this.tsParseInterfaceDeclaration(r);
        } else this.unexpected(r.start);
      }, c.tsIsDeclarationStart = function() {
        return C(this.type);
      }, c.tsParseExpressionStatement = function(r, i) {
        switch (i.name) {
          case "declare":
            var n = this.tsTryParseDeclare(r);
            if (n) return n.declare = true, n;
            break;
          case "global":
            if (this.match(o.braceL)) {
              A.prototype.enterScope.call(this, ir);
              var l = r;
              return l.global = true, l.id = i, l.body = this.tsParseModuleBlock(), A.prototype.exitScope.call(this), this.finishNode(l, "TSModuleDeclaration");
            }
            break;
          default:
            return this.tsParseDeclaration(r, i.name, false);
        }
      }, c.tsParseModuleReference = function() {
        return this.tsIsExternalModuleReference() ? this.tsParseExternalModuleReference() : this.tsParseEntityName(false);
      }, c.tsIsExportDefaultSpecifier = function() {
        var r = this.type, i = this.isAsyncFunction(), n = this.isLet();
        if (_(r)) {
          if (i && !this.containsEsc || n) return false;
          if ((r === R.type || r === R.interface) && !this.containsEsc) {
            var l = this.lookahead();
            if (_(l.type) && !this.isContextualWithState("from", l) || l.type === o.braceL) return false;
          }
        } else if (!this.match(o._default)) return false;
        var f = this.nextTokenStart(), b = this.isUnparsedContextual(f, "from");
        if (this.input.charCodeAt(f) === 44 || _(this.type) && b) return true;
        if (this.match(o._default) && b) {
          var P = this.input.charCodeAt(this.nextTokenStartSince(f + 4));
          return P === 34 || P === 39;
        }
        return false;
      }, c.tsInAmbientContext = function(r) {
        var i = this.isAmbientContext;
        this.isAmbientContext = true;
        try {
          return r();
        } finally {
          this.isAmbientContext = i;
        }
      }, c.tsCheckLineTerminator = function(r) {
        return r ? !this.hasFollowingLineBreak() && (this.next(), true) : !this.isLineTerminator();
      }, c.tsParseModuleOrNamespaceDeclaration = function(r, i) {
        if (i === void 0 && (i = false), r.id = this.parseIdent(), i || this.checkLValSimple(r.id, 8), this.eat(o.dot)) {
          var n = this.startNode();
          this.tsParseModuleOrNamespaceDeclaration(n, true), r.body = n;
        } else A.prototype.enterScope.call(this, ir), r.body = this.tsParseModuleBlock(), A.prototype.exitScope.call(this);
        return this.finishNode(r, "TSModuleDeclaration");
      }, c.checkLValSimple = function(r, i, n) {
        return i === void 0 && (i = 0), A.prototype.checkLValSimple.call(this, r, i, n);
      }, c.tsParseTypeAliasDeclaration = function(r) {
        var i = this;
        return r.id = this.parseIdent(), this.checkLValSimple(r.id, 6), r.typeAnnotation = this.tsInType(function() {
          if (r.typeParameters = i.tsTryParseTypeParameters(i.tsParseInOutModifiers.bind(i)), i.expect(o.eq), i.ts_isContextual(R.interface) && i.lookahead().type !== o.dot) {
            var n = i.startNode();
            return i.next(), i.finishNode(n, "TSIntrinsicKeyword");
          }
          return i.tsParseType();
        }), this.semicolon(), this.finishNode(r, "TSTypeAliasDeclaration");
      }, c.tsParseDeclaration = function(r, i, n) {
        switch (i) {
          case "abstract":
            if (this.tsCheckLineTerminator(n) && (this.match(o._class) || _(this.type))) return this.tsParseAbstractDeclaration(r);
            break;
          case "module":
            if (this.tsCheckLineTerminator(n)) {
              if (this.match(o.string)) return this.tsParseAmbientExternalModuleDeclaration(r);
              if (_(this.type)) return this.tsParseModuleOrNamespaceDeclaration(r);
            }
            break;
          case "namespace":
            if (this.tsCheckLineTerminator(n) && _(this.type)) return this.tsParseModuleOrNamespaceDeclaration(r);
            break;
          case "type":
            if (this.tsCheckLineTerminator(n) && _(this.type)) return this.tsParseTypeAliasDeclaration(r);
        }
      }, c.tsTryParseExportDeclaration = function() {
        return this.tsParseDeclaration(this.startNode(), this.value, true);
      }, c.tsParseImportEqualsDeclaration = function(r, i) {
        r.isExport = i || false, r.id = this.parseIdent(), this.checkLValSimple(r.id, 2), A.prototype.expect.call(this, o.eq);
        var n = this.tsParseModuleReference();
        return r.importKind === "type" && n.type !== "TSExternalModuleReference" && this.raise(n.start, le.ImportAliasHasImportType), r.moduleReference = n, A.prototype.semicolon.call(this), this.finishNode(r, "TSImportEqualsDeclaration");
      }, c.isExportDefaultSpecifier = function() {
        if (this.tsIsDeclarationStart()) return false;
        var r = this.type;
        if (_(r)) {
          if (this.isContextual("async") || this.isContextual("let")) return false;
          if ((r === R.type || r === R.interface) && !this.containsEsc) {
            var i = this.lookahead();
            if (_(i.type) && !this.isContextualWithState("from", i) || i.type === o.braceL) return false;
          }
        } else if (!this.match(o._default)) return false;
        var n = this.nextTokenStart(), l = this.isUnparsedContextual(n, "from");
        if (this.input.charCodeAt(n) === 44 || _(this.type) && l) return true;
        if (this.match(o._default) && l) {
          var f = this.input.charCodeAt(this.nextTokenStartSince(n + 4));
          return f === 34 || f === 39;
        }
        return false;
      }, c.parseTemplate = function(r) {
        var i = (r === void 0 ? {} : r).isTagged, n = i !== void 0 && i, l = this.startNode();
        this.next(), l.expressions = [];
        var f = this.parseTemplateElement({ isTagged: n });
        for (l.quasis = [f]; !f.tail; ) this.type === o.eof && this.raise(this.pos, "Unterminated template literal"), this.expect(o.dollarBraceL), l.expressions.push(this.inType ? this.tsParseType() : this.parseExpression()), this.expect(o.braceR), l.quasis.push(f = this.parseTemplateElement({ isTagged: n }));
        return this.next(), this.finishNode(l, "TemplateLiteral");
      }, c.parseFunction = function(r, i, n, l, f) {
        this.initFunction(r), (this.options.ecmaVersion >= 9 || this.options.ecmaVersion >= 6 && !l) && (this.type === o.star && 2 & i && this.unexpected(), r.generator = this.eat(o.star)), this.options.ecmaVersion >= 8 && (r.async = !!l), 1 & i && (r.id = 4 & i && this.type !== o.name ? null : this.parseIdent());
        var b = this.yieldPos, P = this.awaitPos, q = this.awaitIdentPos, T = this.maybeInArrowParameters;
        this.maybeInArrowParameters = false, this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0, this.enterScope(pl(r.async, r.generator)), 1 & i || (r.id = this.type === o.name ? this.parseIdent() : null), this.parseFunctionParams(r);
        var H = 1 & i;
        return this.parseFunctionBody(r, n, false, f, { isFunctionDeclaration: H }), this.yieldPos = b, this.awaitPos = P, this.awaitIdentPos = q, 1 & i && r.id && !(2 & i) && this.checkLValSimple(r.id, r.body ? this.strict || r.generator || r.async ? this.treatFunctionsAsVar ? 1 : 2 : 3 : 0), this.maybeInArrowParameters = T, this.finishNode(r, H ? "FunctionDeclaration" : "FunctionExpression");
      }, c.parseFunctionBody = function(r, i, n, l, f) {
        i === void 0 && (i = false), n === void 0 && (n = false), l === void 0 && (l = false), this.match(o.colon) && (r.returnType = this.tsParseTypeOrTypePredicateAnnotation(o.colon));
        var b = f != null && f.isFunctionDeclaration ? "TSDeclareFunction" : f != null && f.isClassMethod ? "TSDeclareMethod" : void 0;
        return b && !this.match(o.braceL) && this.isLineTerminator() ? this.finishNode(r, b) : b === "TSDeclareFunction" && this.isAmbientContext && (this.raise(r.start, le.DeclareFunctionHasImplementation), r.declare) ? (A.prototype.parseFunctionBody.call(this, r, i, n, false), this.finishNode(r, b)) : (A.prototype.parseFunctionBody.call(this, r, i, n, l), r);
      }, c.parseNew = function() {
        var r;
        this.containsEsc && this.raiseRecoverable(this.start, "Escape sequence in keyword new");
        var i = this.startNode(), n = this.parseIdent(true);
        if (this.options.ecmaVersion >= 6 && this.eat(o.dot)) {
          i.meta = n;
          var l = this.containsEsc;
          return i.property = this.parseIdent(true), i.property.name !== "target" && this.raiseRecoverable(i.property.start, "The only valid meta property for new is 'new.target'"), l && this.raiseRecoverable(i.start, "'new.target' must not contain escaped characters"), this.allowNewDotTarget || this.raiseRecoverable(i.start, "'new.target' can only be used in functions and class static block"), this.finishNode(i, "MetaProperty");
        }
        var f = this.start, b = this.startLoc, P = this.type === o._import;
        i.callee = this.parseSubscripts(this.parseExprAtom(), f, b, true, false), P && i.callee.type === "ImportExpression" && this.raise(f, "Cannot use new with import()");
        var q = i.callee;
        return q.type !== "TSInstantiationExpression" || (r = q.extra) != null && r.parenthesized || (i.typeParameters = q.typeParameters, i.callee = q.expression), i.arguments = this.eat(o.parenL) ? this.parseExprList(o.parenR, this.options.ecmaVersion >= 8, false) : [], this.finishNode(i, "NewExpression");
      }, c.parseExprOp = function(r, i, n, l, f) {
        var b;
        if (o._in.binop > l && !this.hasPrecedingLineBreak() && (this.isContextual("as") && (b = "TSAsExpression"), p && this.isContextual("satisfies") && (b = "TSSatisfiesExpression"), b)) {
          var P = this.startNodeAt(i, n);
          P.expression = r;
          var q = this.tsTryNextParseConstantContext();
          return P.typeAnnotation = q || this.tsNextThenParseType(), this.finishNode(P, b), this.reScan_lt_gt(), this.parseExprOp(P, i, n, l, f);
        }
        return A.prototype.parseExprOp.call(this, r, i, n, l, f);
      }, c.parseImportSpecifiers = function() {
        var r = [], i = true;
        if (x.tokenIsIdentifier(this.type) && (r.push(this.parseImportDefaultSpecifier()), !this.eat(o.comma))) return r;
        if (this.type === o.star) return r.push(this.parseImportNamespaceSpecifier()), r;
        for (this.expect(o.braceL); !this.eat(o.braceR); ) {
          if (i) i = false;
          else if (this.expect(o.comma), this.afterTrailingComma(o.braceR)) break;
          r.push(this.parseImportSpecifier());
        }
        return r;
      }, c.parseImport = function(r) {
        var i = this.lookahead();
        if (r.importKind = "value", this.importOrExportOuterKind = "value", _(i.type) || this.match(o.star) || this.match(o.braceL)) {
          var n = this.lookahead(2);
          if (n.type !== o.comma && !this.isContextualWithState("from", n) && n.type !== o.eq && this.ts_eatContextualWithState("type", 1, i) && (this.importOrExportOuterKind = "type", r.importKind = "type", i = this.lookahead(), n = this.lookahead(2)), _(i.type) && n.type === o.eq) {
            this.next();
            var l = this.tsParseImportEqualsDeclaration(r);
            return this.importOrExportOuterKind = "value", l;
          }
        }
        return this.next(), this.type === o.string ? (r.specifiers = [], r.source = this.parseExprAtom()) : (r.specifiers = this.parseImportSpecifiers(), this.expectContextual("from"), r.source = this.type === o.string ? this.parseExprAtom() : this.unexpected()), this.parseMaybeImportAttributes(r), this.semicolon(), this.finishNode(r, "ImportDeclaration"), this.importOrExportOuterKind = "value", r.importKind === "type" && r.specifiers.length > 1 && r.specifiers[0].type === "ImportDefaultSpecifier" && this.raise(r.start, le.TypeImportCannotSpecifyDefaultAndNamed), r;
      }, c.parseExportDefaultDeclaration = function() {
        if (this.isAbstractClass()) {
          var r = this.startNode();
          return this.next(), r.abstract = true, this.parseClass(r, true);
        }
        if (this.match(R.interface)) {
          var i = this.tsParseInterfaceDeclaration(this.startNode());
          if (i) return i;
        }
        return A.prototype.parseExportDefaultDeclaration.call(this);
      }, c.parseExportAllDeclaration = function(r, i) {
        return this.options.ecmaVersion >= 11 && (this.eatContextual("as") ? (r.exported = this.parseModuleExportName(), this.checkExport(i, r.exported, this.lastTokStart)) : r.exported = null), this.expectContextual("from"), this.type !== o.string && this.unexpected(), r.source = this.parseExprAtom(), this.parseMaybeImportAttributes(r), this.semicolon(), this.finishNode(r, "ExportAllDeclaration");
      }, c.parseDynamicImport = function(r) {
        if (this.next(), r.source = this.parseMaybeAssign(), this.eat(o.comma)) {
          var i = this.parseExpression();
          r.arguments = [i];
        }
        if (!this.eat(o.parenR)) {
          var n = this.start;
          this.eat(o.comma) && this.eat(o.parenR) ? this.raiseRecoverable(n, "Trailing comma is not allowed in import()") : this.unexpected(n);
        }
        return this.finishNode(r, "ImportExpression");
      }, c.parseExport = function(r, i) {
        var n = this.lookahead();
        if (this.ts_eatWithState(o._import, 2, n)) {
          this.ts_isContextual(R.type) && this.lookaheadCharCode() !== 61 ? (r.importKind = "type", this.importOrExportOuterKind = "type", this.next()) : (r.importKind = "value", this.importOrExportOuterKind = "value");
          var l = this.tsParseImportEqualsDeclaration(r, true);
          return this.importOrExportOuterKind = void 0, l;
        }
        if (this.ts_eatWithState(o.eq, 2, n)) {
          var f = r;
          return f.expression = this.parseExpression(), this.semicolon(), this.importOrExportOuterKind = void 0, this.finishNode(f, "TSExportAssignment");
        }
        if (this.ts_eatContextualWithState("as", 2, n)) {
          var b = r;
          return this.expectContextual("namespace"), b.id = this.parseIdent(), this.semicolon(), this.importOrExportOuterKind = void 0, this.finishNode(b, "TSNamespaceExportDeclaration");
        }
        if (this.ts_isContextualWithState(n, R.type) && this.lookahead(2).type === o.braceL ? (this.next(), this.importOrExportOuterKind = "type", r.exportKind = "type") : (this.importOrExportOuterKind = "value", r.exportKind = "value"), this.next(), this.eat(o.star)) return this.parseExportAllDeclaration(r, i);
        if (this.eat(o._default)) return this.checkExport(i, "default", this.lastTokStart), r.declaration = this.parseExportDefaultDeclaration(), this.finishNode(r, "ExportDefaultDeclaration");
        if (this.shouldParseExportStatement()) r.declaration = this.parseExportDeclaration(r), r.declaration.type === "VariableDeclaration" ? this.checkVariableExport(i, r.declaration.declarations) : this.checkExport(i, r.declaration.id, r.declaration.id.start), r.specifiers = [], r.source = null;
        else {
          if (r.declaration = null, r.specifiers = this.parseExportSpecifiers(i), this.eatContextual("from")) this.type !== o.string && this.unexpected(), r.source = this.parseExprAtom(), this.parseMaybeImportAttributes(r);
          else {
            for (var P, q = Kl(r.specifiers); !(P = q()).done; ) {
              var T = P.value;
              this.checkUnreserved(T.local), this.checkLocalExport(T.local), T.local.type === "Literal" && this.raise(T.local.start, "A string literal cannot be used as an exported binding without `from`.");
            }
            r.source = null;
          }
          this.semicolon();
        }
        return this.finishNode(r, "ExportNamedDeclaration");
      }, c.checkExport = function(r, i, n) {
        r && (typeof i != "string" && (i = i.type === "Identifier" ? i.name : i.value), r[i] = true);
      }, c.parseMaybeDefault = function(r, i, n) {
        var l = A.prototype.parseMaybeDefault.call(this, r, i, n);
        return l.type === "AssignmentPattern" && l.typeAnnotation && l.right.start < l.typeAnnotation.start && this.raise(l.typeAnnotation.start, le.TypeAnnotationAfterAssign), l;
      }, c.typeCastToParameter = function(r) {
        return r.expression.typeAnnotation = r.typeAnnotation, this.resetEndLocation(r.expression, r.typeAnnotation.end), r.expression;
      }, c.toAssignableList = function(r, i) {
        for (var n = 0; n < r.length; n++) {
          var l = r[n];
          (l == null ? void 0 : l.type) === "TSTypeCastExpression" && (r[n] = this.typeCastToParameter(l));
        }
        return A.prototype.toAssignableList.call(this, r, i);
      }, c.reportReservedArrowTypeParam = function(r) {
      }, c.parseExprAtom = function(r, i, n) {
        if (this.type === R.jsxText) return this.jsx_parseText();
        if (this.type === R.jsxTagStart) return this.jsx_parseElement();
        if (this.type === R.at) return this.parseDecorators(), this.parseExprAtom();
        if (_(this.type)) {
          var l = this.potentialArrowAt === this.start, f = this.start, b = this.startLoc, P = this.containsEsc, q = this.parseIdent(false);
          if (this.options.ecmaVersion >= 8 && !P && q.name === "async" && !this.canInsertSemicolon() && this.eat(o._function)) return this.overrideContext(G.f_expr), this.parseFunction(this.startNodeAt(f, b), 0, false, true, i);
          if (l && !this.canInsertSemicolon()) {
            if (this.eat(o.arrow)) return this.parseArrowExpression(this.startNodeAt(f, b), [q], false, i);
            if (this.options.ecmaVersion >= 8 && q.name === "async" && this.type === o.name && !P && (!this.potentialArrowInForAwait || this.value !== "of" || this.containsEsc)) return q = this.parseIdent(false), !this.canInsertSemicolon() && this.eat(o.arrow) || this.unexpected(), this.parseArrowExpression(this.startNodeAt(f, b), [q], true, i);
          }
          return q;
        }
        return A.prototype.parseExprAtom.call(this, r, i, n);
      }, c.parseExprAtomDefault = function() {
        if (_(this.type)) {
          var r = this.potentialArrowAt === this.start, i = this.containsEsc, n = this.parseIdent();
          if (!i && n.name === "async" && !this.canInsertSemicolon()) {
            var l = this.type;
            if (l === o._function) return this.next(), this.parseFunction(this.startNodeAtNode(n), void 0, true, true);
            if (_(l)) {
              if (this.lookaheadCharCode() === 61) {
                var f = this.parseIdent(false);
                return !this.canInsertSemicolon() && this.eat(o.arrow) || this.unexpected(), this.parseArrowExpression(this.startNodeAtNode(n), [f], true);
              }
              return n;
            }
          }
          return r && this.match(o.arrow) && !this.canInsertSemicolon() ? (this.next(), this.parseArrowExpression(this.startNodeAtNode(n), [n], false)) : n;
        }
        this.unexpected();
      }, c.parseIdentNode = function() {
        var r = this.startNode();
        return k(this.type) ? (r.name = this.value, r) : A.prototype.parseIdentNode.call(this);
      }, c.parseVarStatement = function(r, i, n) {
        n === void 0 && (n = false);
        var l = this.isAmbientContext;
        this.next(), A.prototype.parseVar.call(this, r, false, i, n || l), this.semicolon();
        var f = this.finishNode(r, "VariableDeclaration");
        if (!l) return f;
        for (var b, P = Kl(f.declarations); !(b = P()).done; ) {
          var q = b.value, T = q.init;
          T && (i !== "const" || q.id.typeAnnotation ? this.raise(T.start, le.InitializerNotAllowedInAmbientContext) : T.type !== "StringLiteral" && T.type !== "BooleanLiteral" && T.type !== "NumericLiteral" && T.type !== "BigIntLiteral" && (T.type !== "TemplateLiteral" || T.expressions.length > 0) && !W1(T) && this.raise(T.start, le.ConstInitiailizerMustBeStringOrNumericLiteralOrLiteralEnumReference));
        }
        return f;
      }, c.parseStatement = function(r, i, n) {
        if (this.match(R.at) && this.parseDecorators(true), this.match(o._const) && this.isLookaheadContextual("enum")) {
          var l = this.startNode();
          return this.expect(o._const), this.tsParseEnumDeclaration(l, { const: true });
        }
        if (this.ts_isContextual(R.enum)) return this.tsParseEnumDeclaration(this.startNode());
        if (this.ts_isContextual(R.interface)) {
          var f = this.tsParseInterfaceDeclaration(this.startNode());
          if (f) return f;
        }
        return A.prototype.parseStatement.call(this, r, i, n);
      }, c.parseAccessModifier = function() {
        return this.tsParseModifier(["public", "protected", "private"]);
      }, c.parsePostMemberNameModifiers = function(r) {
        this.eat(o.question) && (r.optional = true), r.readonly && this.match(o.parenL) && this.raise(r.start, le.ClassMethodHasReadonly), r.declare && this.match(o.parenL) && this.raise(r.start, le.ClassMethodHasDeclare);
      }, c.parseExpressionStatement = function(r, i) {
        return (i.type === "Identifier" ? this.tsParseExpressionStatement(r, i) : void 0) || A.prototype.parseExpressionStatement.call(this, r, i);
      }, c.shouldParseExportStatement = function() {
        return !!this.tsIsDeclarationStart() || !!this.match(R.at) || A.prototype.shouldParseExportStatement.call(this);
      }, c.parseConditional = function(r, i, n, l, f) {
        if (this.eat(o.question)) {
          var b = this.startNodeAt(i, n);
          return b.test = r, b.consequent = this.parseMaybeAssign(), this.expect(o.colon), b.alternate = this.parseMaybeAssign(l), this.finishNode(b, "ConditionalExpression");
        }
        return r;
      }, c.parseMaybeConditional = function(r, i) {
        var n = this, l = this.start, f = this.startLoc, b = this.parseExprOps(r, i);
        if (this.checkExpressionErrors(i)) return b;
        if (!this.maybeInArrowParameters || !this.match(o.question)) return this.parseConditional(b, l, f, r, i);
        var P = this.tryParse(function() {
          return n.parseConditional(b, l, f, r, i);
        });
        return P.node ? (P.error && this.setLookaheadState(P.failState), P.node) : (P.error && this.setOptionalParametersError(i, P.error), b);
      }, c.parseParenItem = function(r) {
        var i = this.start, n = this.startLoc;
        if (r = A.prototype.parseParenItem.call(this, r), this.eat(o.question) && (r.optional = true, this.resetEndLocation(r)), this.match(o.colon)) {
          var l = this.startNodeAt(i, n);
          return l.expression = r, l.typeAnnotation = this.tsParseTypeAnnotation(), this.finishNode(l, "TSTypeCastExpression");
        }
        return r;
      }, c.parseExportDeclaration = function(r) {
        var i = this;
        if (!this.isAmbientContext && this.ts_isContextual(R.declare)) return this.tsInAmbientContext(function() {
          return i.parseExportDeclaration(r);
        });
        var n = this.start, l = this.startLoc, f = this.eatContextual("declare");
        !f || !this.ts_isContextual(R.declare) && this.shouldParseExportStatement() || this.raise(this.start, le.ExpectedAmbientAfterExportDeclare);
        var b = _(this.type) && this.tsTryParseExportDeclaration() || this.parseStatement(null);
        return b ? ((b.type === "TSInterfaceDeclaration" || b.type === "TSTypeAliasDeclaration" || f) && (r.exportKind = "type"), f && (this.resetStartLocation(b, n, l), b.declare = true), b) : null;
      }, c.parseClassId = function(r, i) {
        if (i || !this.isContextual("implements")) {
          A.prototype.parseClassId.call(this, r, i);
          var n = this.tsTryParseTypeParameters(this.tsParseInOutModifiers.bind(this));
          n && (r.typeParameters = n);
        }
      }, c.parseClassPropertyAnnotation = function(r) {
        r.optional || (this.value === "!" && this.eat(o.prefix) ? r.definite = true : this.eat(o.question) && (r.optional = true));
        var i = this.tsTryParseTypeAnnotation();
        i && (r.typeAnnotation = i);
      }, c.parseClassField = function(r) {
        if (r.key.type === "PrivateIdentifier") r.abstract && this.raise(r.start, le.PrivateElementHasAbstract), r.accessibility && this.raise(r.start, le.PrivateElementHasAccessibility({ modifier: r.accessibility })), this.parseClassPropertyAnnotation(r);
        else if (this.parseClassPropertyAnnotation(r), this.isAmbientContext && (!r.readonly || r.typeAnnotation) && this.match(o.eq) && this.raise(this.start, le.DeclareClassFieldHasInitializer), r.abstract && this.match(o.eq)) {
          var i = r.key;
          this.raise(this.start, le.AbstractPropertyHasInitializer({ propertyName: i.type !== "Identifier" || r.computed ? "[" + this.input.slice(i.start, i.end) + "]" : i.name }));
        }
        return A.prototype.parseClassField.call(this, r);
      }, c.parseClassMethod = function(r, i, n, l) {
        var f = r.kind === "constructor", b = r.key.type === "PrivateIdentifier", P = this.tsTryParseTypeParameters();
        b ? (P && (r.typeParameters = P), r.accessibility && this.raise(r.start, le.PrivateMethodsHasAccessibility({ modifier: r.accessibility }))) : P && f && this.raise(P.start, le.ConstructorHasTypeParameters);
        var q = r.declare, T = r.kind;
        !(q !== void 0 && q) || T !== "get" && T !== "set" || this.raise(r.start, le.DeclareAccessor({ kind: T })), P && (r.typeParameters = P);
        var H = r.key;
        r.kind === "constructor" ? (i && this.raise(H.start, "Constructor can't be a generator"), n && this.raise(H.start, "Constructor can't be an async method")) : r.static && Ql(r, "prototype") && this.raise(H.start, "Classes may not have a static property named prototype");
        var J = r.value = this.parseMethod(i, n, l, true, r);
        return r.kind === "get" && J.params.length !== 0 && this.raiseRecoverable(J.start, "getter should have no params"), r.kind === "set" && J.params.length !== 1 && this.raiseRecoverable(J.start, "setter should have exactly one param"), r.kind === "set" && J.params[0].type === "RestElement" && this.raiseRecoverable(J.params[0].start, "Setter cannot use rest params"), this.finishNode(r, "MethodDefinition");
      }, c.isClassMethod = function() {
        return this.match(o.relational);
      }, c.parseClassElement = function(r) {
        var i = this;
        if (this.eat(o.semi)) return null;
        var n, l = this.options.ecmaVersion, f = this.startNode(), b = "", P = false, q = false, T = "method", H = ["declare", "private", "public", "protected", "accessor", "override", "abstract", "readonly", "static"], J = this.tsParseModifiers({ modified: f, allowedModifiers: H, disallowedModifiers: ["in", "out"], stopOnStartOfClassStaticBlock: true, errorTemplate: le.InvalidModifierOnTypeParameterPositions });
        n = !!J.static;
        var re = function() {
          if (!i.tsIsStartOfStaticBlocks()) {
            var Q = i.tsTryParseIndexSignature(f);
            if (Q) return f.abstract && i.raise(f.start, le.IndexSignatureHasAbstract), f.accessibility && i.raise(f.start, le.IndexSignatureHasAccessibility({ modifier: f.accessibility })), f.declare && i.raise(f.start, le.IndexSignatureHasDeclare), f.override && i.raise(f.start, le.IndexSignatureHasOverride), Q;
            if (!i.inAbstractClass && f.abstract && i.raise(f.start, le.NonAbstractClassHasAbstractMethod), f.override && r && i.raise(f.start, le.OverrideNotInSubClass), f.static = n, n && (i.isClassElementNameStart() || i.type === o.star || (b = "static")), !b && l >= 8 && i.eatContextual("async") && (!i.isClassElementNameStart() && i.type !== o.star || i.canInsertSemicolon() ? b = "async" : q = true), !b && (l >= 9 || !q) && i.eat(o.star) && (P = true), !b && !q && !P) {
              var de = i.value;
              (i.eatContextual("get") || i.eatContextual("set")) && (i.isClassElementNameStart() ? T = de : b = de);
            }
            if (b ? (f.computed = false, f.key = i.startNodeAt(i.lastTokStart, i.lastTokStartLoc), f.key.name = b, i.finishNode(f.key, "Identifier")) : i.parseClassElementName(f), i.parsePostMemberNameModifiers(f), i.isClassMethod() || l < 13 || i.type === o.parenL || T !== "method" || P || q) {
              var pe = !f.static && Ql(f, "constructor"), fe = pe && r;
              pe && T !== "method" && i.raise(f.key.start, "Constructor can't have get/set modifier"), f.kind = pe ? "constructor" : T, i.parseClassMethod(f, P, q, fe);
            } else i.parseClassField(f);
            return f;
          }
          if (i.next(), i.next(), i.tsHasSomeModifiers(f, H) && i.raise(i.start, le.StaticBlockCannotHaveModifier), l >= 13) return A.prototype.parseClassStaticBlock.call(i, f), f;
        };
        return f.declare ? this.tsInAmbientContext(re) : re(), f;
      }, c.isClassElementNameStart = function() {
        return !!this.tsIsIdentifier() || A.prototype.isClassElementNameStart.call(this);
      }, c.parseClassSuper = function(r) {
        A.prototype.parseClassSuper.call(this, r), r.superClass && (this.tsMatchLeftRelational() || this.match(o.bitShift)) && (r.superTypeParameters = this.tsParseTypeArgumentsInExpression()), this.eatContextual("implements") && (r.implements = this.tsParseHeritageClause("implements"));
      }, c.parseFunctionParams = function(r) {
        var i = this.tsTryParseTypeParameters();
        i && (r.typeParameters = i), A.prototype.parseFunctionParams.call(this, r);
      }, c.parseVarId = function(r, i) {
        A.prototype.parseVarId.call(this, r, i), r.id.type === "Identifier" && !this.hasPrecedingLineBreak() && this.value === "!" && this.eat(o.prefix) && (r.definite = true);
        var n = this.tsTryParseTypeAnnotation();
        n && (r.id.typeAnnotation = n, this.resetEndLocation(r.id));
      }, c.parseArrowExpression = function(r, i, n, l) {
        this.match(o.colon) && (r.returnType = this.tsParseTypeAnnotation());
        var f = this.yieldPos, b = this.awaitPos, P = this.awaitIdentPos;
        this.enterScope(16 | pl(n, false)), this.initFunction(r);
        var q = this.maybeInArrowParameters;
        return this.options.ecmaVersion >= 8 && (r.async = !!n), this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0, this.maybeInArrowParameters = true, r.params = this.toAssignableList(i, true), this.maybeInArrowParameters = false, this.parseFunctionBody(r, true, false, l), this.yieldPos = f, this.awaitPos = b, this.awaitIdentPos = P, this.maybeInArrowParameters = q, this.finishNode(r, "ArrowFunctionExpression");
      }, c.parseMaybeAssignOrigin = function(r, i, n) {
        if (this.isContextual("yield")) {
          if (this.inGenerator) return this.parseYield(r);
          this.exprAllowed = false;
        }
        var l = false, f = -1, b = -1, P = -1;
        i ? (f = i.parenthesizedAssign, b = i.trailingComma, P = i.doubleProto, i.parenthesizedAssign = i.trailingComma = -1) : (i = new sr(), l = true);
        var q = this.start, T = this.startLoc;
        (this.type === o.parenL || _(this.type)) && (this.potentialArrowAt = this.start, this.potentialArrowInForAwait = r === "await");
        var H = this.parseMaybeConditional(r, i);
        if (n && (H = n.call(this, H, q, T)), this.type.isAssign) {
          var J = this.startNodeAt(q, T);
          return J.operator = this.value, this.type === o.eq && (H = this.toAssignable(H, true, i)), l || (i.parenthesizedAssign = i.trailingComma = i.doubleProto = -1), i.shorthandAssign >= H.start && (i.shorthandAssign = -1), this.type === o.eq ? this.checkLValPattern(H) : this.checkLValSimple(H), J.left = H, this.next(), J.right = this.parseMaybeAssign(r), P > -1 && (i.doubleProto = P), this.finishNode(J, "AssignmentExpression");
        }
        return l && this.checkExpressionErrors(i, true), f > -1 && (i.parenthesizedAssign = f), b > -1 && (i.trailingComma = b), H;
      }, c.parseMaybeAssign = function(r, i, n) {
        var l, f, b, P, q, T, H, J, re, Q, de, pe = this;
        if (this.matchJsx("jsxTagStart") || this.tsMatchLeftRelational()) {
          if (J = this.cloneCurLookaheadState(), !(re = this.tryParse(function() {
            return pe.parseMaybeAssignOrigin(r, i, n);
          }, J)).error) return re.node;
          var fe = this.context, he = fe[fe.length - 1];
          he === x.tokContexts.tc_oTag && fe[fe.length - 2] === x.tokContexts.tc_expr ? (fe.pop(), fe.pop()) : he !== x.tokContexts.tc_oTag && he !== x.tokContexts.tc_expr || fe.pop();
        }
        if (!((l = re) != null && l.error || this.tsMatchLeftRelational())) return this.parseMaybeAssignOrigin(r, i, n);
        J && !this.compareLookaheadState(J, this.getCurLookaheadState()) || (J = this.cloneCurLookaheadState());
        var Ee = this.tryParse(function(Oe) {
          var ke, We;
          de = pe.tsParseTypeParameters();
          var Ne = pe.parseMaybeAssignOrigin(r, i, n);
          return (Ne.type !== "ArrowFunctionExpression" || (ke = Ne.extra) != null && ke.parenthesized) && Oe(), ((We = de) == null ? void 0 : We.params.length) !== 0 && pe.resetStartLocationFromNode(Ne, de), Ne.typeParameters = de, Ne;
        }, J);
        if (!Ee.error && !Ee.aborted) return de && this.reportReservedArrowTypeParam(de), Ee.node;
        if (!re && (Yl(true), !(Q = this.tryParse(function() {
          return pe.parseMaybeAssignOrigin(r, i, n);
        }, J)).error)) return Q.node;
        if ((f = re) != null && f.node) return this.setLookaheadState(re.failState), re.node;
        if (Ee.node) return this.setLookaheadState(Ee.failState), de && this.reportReservedArrowTypeParam(de), Ee.node;
        if ((b = Q) != null && b.node) return this.setLookaheadState(Q.failState), Q.node;
        throw (P = re) != null && P.thrown ? re.error : Ee.thrown ? Ee.error : (q = Q) != null && q.thrown ? Q.error : ((T = re) == null ? void 0 : T.error) || Ee.error || ((H = Q) == null ? void 0 : H.error);
      }, c.parseAssignableListItem = function(r) {
        for (var i = []; this.match(R.at); ) i.push(this.parseDecorator());
        var n, l = this.start, f = this.startLoc, b = false, P = false;
        if (r !== void 0) {
          var q = {};
          this.tsParseModifiers({ modified: q, allowedModifiers: ["public", "private", "protected", "override", "readonly"] }), n = q.accessibility, P = q.override, b = q.readonly, r === false && (n || b || P) && this.raise(f.start, le.UnexpectedParameterModifier);
        }
        var T = this.parseMaybeDefault(l, f);
        this.parseBindingListItem(T);
        var H = this.parseMaybeDefault(T.start, T.loc, T);
        if (i.length && (H.decorators = i), n || b || P) {
          var J = this.startNodeAt(l, f);
          return n && (J.accessibility = n), b && (J.readonly = b), P && (J.override = P), H.type !== "Identifier" && H.type !== "AssignmentPattern" && this.raise(J.start, le.UnsupportedParameterPropertyKind), J.parameter = H, this.finishNode(J, "TSParameterProperty");
        }
        return H;
      }, c.checkLValInnerPattern = function(r, i, n) {
        i === void 0 && (i = 0), r.type === "TSParameterProperty" ? this.checkLValInnerPattern(r.parameter, i, n) : A.prototype.checkLValInnerPattern.call(this, r, i, n);
      }, c.parseBindingListItem = function(r) {
        this.eat(o.question) && (r.type === "Identifier" || this.isAmbientContext || this.inType || this.raise(r.start, le.PatternIsOptional), r.optional = true);
        var i = this.tsTryParseTypeAnnotation();
        return i && (r.typeAnnotation = i), this.resetEndLocation(r), r;
      }, c.isAssignable = function(r, i) {
        var n = this;
        switch (r.type) {
          case "TSTypeCastExpression":
            return this.isAssignable(r.expression, i);
          case "TSParameterProperty":
          case "Identifier":
          case "ObjectPattern":
          case "ArrayPattern":
          case "AssignmentPattern":
          case "RestElement":
            return true;
          case "ObjectExpression":
            var l = r.properties.length - 1;
            return r.properties.every(function(f, b) {
              return f.type !== "ObjectMethod" && (b === l || f.type !== "SpreadElement") && n.isAssignable(f);
            });
          case "Property":
          case "ObjectProperty":
            return this.isAssignable(r.value);
          case "SpreadElement":
            return this.isAssignable(r.argument);
          case "ArrayExpression":
            return r.elements.every(function(f) {
              return f === null || n.isAssignable(f);
            });
          case "AssignmentExpression":
            return r.operator === "=";
          case "ParenthesizedExpression":
            return this.isAssignable(r.expression);
          case "MemberExpression":
          case "OptionalMemberExpression":
            return !i;
          default:
            return false;
        }
      }, c.toAssignable = function(r, i, n) {
        switch (i === void 0 && (i = false), n === void 0 && (n = new sr()), r.type) {
          case "ParenthesizedExpression":
            return this.toAssignableParenthesizedExpression(r, i, n);
          case "TSAsExpression":
          case "TSSatisfiesExpression":
          case "TSNonNullExpression":
          case "TSTypeAssertion":
            return i || this.raise(r.start, le.UnexpectedTypeCastInParameter), this.toAssignable(r.expression, i, n);
          case "MemberExpression":
            break;
          case "AssignmentExpression":
            return i || r.left.type !== "TSTypeCastExpression" || (r.left = this.typeCastToParameter(r.left)), A.prototype.toAssignable.call(this, r, i, n);
          case "TSTypeCastExpression":
            return this.typeCastToParameter(r);
          default:
            return A.prototype.toAssignable.call(this, r, i, n);
        }
        return r;
      }, c.toAssignableParenthesizedExpression = function(r, i, n) {
        switch (r.expression.type) {
          case "TSAsExpression":
          case "TSSatisfiesExpression":
          case "TSNonNullExpression":
          case "TSTypeAssertion":
          case "ParenthesizedExpression":
            return this.toAssignable(r.expression, i, n);
          default:
            return A.prototype.toAssignable.call(this, r, i, n);
        }
      }, c.curPosition = function() {
        if (this.options.locations) {
          var r = A.prototype.curPosition.call(this);
          return Object.defineProperty(r, "offset", { get: function() {
            return function(i) {
              var n = new w.Position(this.line, this.column + i);
              return n.index = this.index + i, n;
            };
          } }), r.index = this.pos, r;
        }
      }, c.parseBindingAtom = function() {
        return this.type === o._this ? this.parseIdent(true) : A.prototype.parseBindingAtom.call(this);
      }, c.shouldParseArrow = function(r) {
        var i, n = this;
        if (i = this.match(o.colon) ? r.every(function(f) {
          return n.isAssignable(f, true);
        }) : !this.canInsertSemicolon()) {
          if (this.match(o.colon)) {
            var l = this.tryParse(function(f) {
              var b = n.tsParseTypeOrTypePredicateAnnotation(o.colon);
              return !n.canInsertSemicolon() && n.match(o.arrow) || f(), b;
            });
            if (l.aborted) return this.shouldParseArrowReturnType = void 0, false;
            l.thrown || (l.error && this.setLookaheadState(l.failState), this.shouldParseArrowReturnType = l.node);
          }
          return !!this.match(o.arrow) || (this.shouldParseArrowReturnType = void 0, false);
        }
        return this.shouldParseArrowReturnType = void 0, i;
      }, c.parseParenArrowList = function(r, i, n, l) {
        var f = this.startNodeAt(r, i);
        return f.returnType = this.shouldParseArrowReturnType, this.shouldParseArrowReturnType = void 0, this.parseArrowExpression(f, n, false, l);
      }, c.parseParenAndDistinguishExpression = function(r, i) {
        var n, l = this.start, f = this.startLoc, b = this.options.ecmaVersion >= 8;
        if (this.options.ecmaVersion >= 6) {
          var P = this.maybeInArrowParameters;
          this.maybeInArrowParameters = true, this.next();
          var q, T = this.start, H = this.startLoc, J = [], re = true, Q = false, de = new sr(), pe = this.yieldPos, fe = this.awaitPos;
          for (this.yieldPos = 0, this.awaitPos = 0; this.type !== o.parenR; ) {
            if (re ? re = false : this.expect(o.comma), b && this.afterTrailingComma(o.parenR, true)) {
              Q = true;
              break;
            }
            if (this.type === o.ellipsis) {
              q = this.start, J.push(this.parseParenItem(this.parseRestBinding())), this.type === o.comma && this.raise(this.start, "Comma is not permitted after the rest element");
              break;
            }
            J.push(this.parseMaybeAssign(i, de, this.parseParenItem));
          }
          var he = this.lastTokEnd, Ee = this.lastTokEndLoc;
          if (this.expect(o.parenR), this.maybeInArrowParameters = P, r && this.shouldParseArrow(J) && this.eat(o.arrow)) return this.checkPatternErrors(de, false), this.checkYieldAwaitInDefaultParams(), this.yieldPos = pe, this.awaitPos = fe, this.parseParenArrowList(l, f, J, i);
          J.length && !Q || this.unexpected(this.lastTokStart), q && this.unexpected(q), this.checkExpressionErrors(de, true), this.yieldPos = pe || this.yieldPos, this.awaitPos = fe || this.awaitPos, J.length > 1 ? ((n = this.startNodeAt(T, H)).expressions = J, this.finishNodeAt(n, "SequenceExpression", he, Ee)) : n = J[0];
        } else n = this.parseParenExpression();
        if (this.options.preserveParens) {
          var Oe = this.startNodeAt(l, f);
          return Oe.expression = n, this.finishNode(Oe, "ParenthesizedExpression");
        }
        return n;
      }, c.parseTaggedTemplateExpression = function(r, i, n, l) {
        var f = this.startNodeAt(i, n);
        return f.tag = r, f.quasi = this.parseTemplate({ isTagged: true }), l && this.raise(i, "Tagged Template Literals are not allowed in optionalChain."), this.finishNode(f, "TaggedTemplateExpression");
      }, c.shouldParseAsyncArrow = function() {
        var r = this;
        if (!this.match(o.colon)) return !this.canInsertSemicolon() && this.eat(o.arrow);
        var i = this.tryParse(function(n) {
          var l = r.tsParseTypeOrTypePredicateAnnotation(o.colon);
          return !r.canInsertSemicolon() && r.match(o.arrow) || n(), l;
        });
        return i.aborted ? (this.shouldParseAsyncArrowReturnType = void 0, false) : i.thrown ? void 0 : (i.error && this.setLookaheadState(i.failState), this.shouldParseAsyncArrowReturnType = i.node, !this.canInsertSemicolon() && this.eat(o.arrow));
      }, c.parseSubscriptAsyncArrow = function(r, i, n, l) {
        var f = this.startNodeAt(r, i);
        return f.returnType = this.shouldParseAsyncArrowReturnType, this.shouldParseAsyncArrowReturnType = void 0, this.parseArrowExpression(f, n, true, l);
      }, c.parseExprList = function(r, i, n, l) {
        for (var f = [], b = true; !this.eat(r); ) {
          if (b) b = false;
          else if (this.expect(o.comma), i && this.afterTrailingComma(r)) break;
          var P = void 0;
          n && this.type === o.comma ? P = null : this.type === o.ellipsis ? (P = this.parseSpread(l), l && this.type === o.comma && l.trailingComma < 0 && (l.trailingComma = this.start)) : P = this.parseMaybeAssign(false, l, this.parseParenItem), f.push(P);
        }
        return f;
      }, c.parseSubscript = function(r, i, n, l, f, b, P) {
        var q = this, T = b;
        if (!this.hasPrecedingLineBreak() && this.value === "!" && this.match(o.prefix)) {
          this.exprAllowed = false, this.next();
          var H = this.startNodeAt(i, n);
          return H.expression = r, r = this.finishNode(H, "TSNonNullExpression");
        }
        var J = false;
        if (this.match(o.questionDot) && this.lookaheadCharCode() === 60) {
          if (l) return r;
          r.optional = true, T = J = true, this.next();
        }
        if (this.tsMatchLeftRelational() || this.match(o.bitShift)) {
          var re, Q = this.tsTryParseAndCatch(function() {
            if (!l && q.atPossibleAsyncArrow(r)) {
              var gt = q.tsTryParseGenericAsyncArrowFunction(i, n, P);
              if (gt) return r = gt;
            }
            var D = q.tsParseTypeArgumentsInExpression();
            if (!D) return r;
            if (J && !q.match(o.parenL)) return re = q.curPosition(), r;
            if (S(q.type) || q.type === o.backQuote) {
              var X = q.parseTaggedTemplateExpression(r, i, n, T);
              return X.typeParameters = D, X;
            }
            if (!l && q.eat(o.parenL)) {
              var K = new sr(), Y = q.startNodeAt(i, n);
              return Y.callee = r, Y.arguments = q.parseExprList(o.parenR, q.options.ecmaVersion >= 8, false, K), q.tsCheckForInvalidTypeCasts(Y.arguments), Y.typeParameters = D, T && (Y.optional = J), q.checkExpressionErrors(K, true), r = q.finishNode(Y, "CallExpression");
            }
            var ce = q.type;
            if (!(q.tsMatchRightRelational() || ce === o.bitShift || ce !== o.parenL && (ne = ce, !!ne.startsExpr) && !q.hasPrecedingLineBreak())) {
              var ne, ge = q.startNodeAt(i, n);
              return ge.expression = r, ge.typeParameters = D, q.finishNode(ge, "TSInstantiationExpression");
            }
          });
          if (re && this.unexpected(re), Q) return Q.type === "TSInstantiationExpression" && (this.match(o.dot) || this.match(o.questionDot) && this.lookaheadCharCode() !== 40) && this.raise(this.start, le.InvalidPropertyAccessAfterInstantiationExpression), r = Q;
        }
        var de = this.options.ecmaVersion >= 11, pe = de && this.eat(o.questionDot);
        l && pe && this.raise(this.lastTokStart, "Optional chaining cannot appear in the callee of new expressions");
        var fe = this.eat(o.bracketL);
        if (fe || pe && this.type !== o.parenL && this.type !== o.backQuote || this.eat(o.dot)) {
          var he = this.startNodeAt(i, n);
          he.object = r, fe ? (he.property = this.parseExpression(), this.expect(o.bracketR)) : he.property = this.type === o.privateId && r.type !== "Super" ? this.parsePrivateIdent() : this.parseIdent(this.options.allowReserved !== "never"), he.computed = !!fe, de && (he.optional = pe), r = this.finishNode(he, "MemberExpression");
        } else if (!l && this.eat(o.parenL)) {
          var Ee = this.maybeInArrowParameters;
          this.maybeInArrowParameters = true;
          var Oe = new sr(), ke = this.yieldPos, We = this.awaitPos, Ne = this.awaitIdentPos;
          this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0;
          var tt = this.parseExprList(o.parenR, this.options.ecmaVersion >= 8, false, Oe);
          if (f && !pe && this.shouldParseAsyncArrow()) this.checkPatternErrors(Oe, false), this.checkYieldAwaitInDefaultParams(), this.awaitIdentPos > 0 && this.raise(this.awaitIdentPos, "Cannot use 'await' as identifier inside an async function"), this.yieldPos = ke, this.awaitPos = We, this.awaitIdentPos = Ne, r = this.parseSubscriptAsyncArrow(i, n, tt, P);
          else {
            this.checkExpressionErrors(Oe, true), this.yieldPos = ke || this.yieldPos, this.awaitPos = We || this.awaitPos, this.awaitIdentPos = Ne || this.awaitIdentPos;
            var Je = this.startNodeAt(i, n);
            Je.callee = r, Je.arguments = tt, de && (Je.optional = pe), r = this.finishNode(Je, "CallExpression");
          }
          this.maybeInArrowParameters = Ee;
        } else if (this.type === o.backQuote) {
          (pe || T) && this.raise(this.start, "Optional chaining cannot appear in the tag of tagged template expressions");
          var it = this.startNodeAt(i, n);
          it.tag = r, it.quasi = this.parseTemplate({ isTagged: true }), r = this.finishNode(it, "TaggedTemplateExpression");
        }
        return r;
      }, c.parseGetterSetter = function(r) {
        r.kind = r.key.name, this.parsePropertyName(r), r.value = this.parseMethod(false);
        var i = r.kind === "get" ? 0 : 1, n = r.value.params[0], l = n && this.isThisParam(n);
        r.value.params.length !== (i = l ? i + 1 : i) ? this.raiseRecoverable(r.value.start, r.kind === "get" ? "getter should have no params" : "setter should have exactly one param") : r.kind === "set" && r.value.params[0].type === "RestElement" && this.raiseRecoverable(r.value.params[0].start, "Setter cannot use rest params");
      }, c.parseProperty = function(r, i) {
        if (!r) {
          var n = [];
          if (this.match(R.at)) for (; this.match(R.at); ) n.push(this.parseDecorator());
          var l = A.prototype.parseProperty.call(this, r, i);
          return l.type === "SpreadElement" && n.length && this.raise(l.start, "Decorators can't be used with SpreadElement"), n.length && (l.decorators = n, n = []), l;
        }
        return A.prototype.parseProperty.call(this, r, i);
      }, c.parseCatchClauseParam = function() {
        var r = this.parseBindingAtom(), i = r.type === "Identifier";
        this.enterScope(i ? 32 : 0), this.checkLValPattern(r, i ? 4 : 2);
        var n = this.tsTryParseTypeAnnotation();
        return n && (r.typeAnnotation = n, this.resetEndLocation(r)), this.expect(o.parenR), r;
      }, c.parseClass = function(r, i) {
        var n = this.inAbstractClass;
        this.inAbstractClass = !!r.abstract;
        try {
          this.next(), this.takeDecorators(r);
          var l = this.strict;
          this.strict = true, this.parseClassId(r, i), this.parseClassSuper(r);
          var f = this.enterClassBody(), b = this.startNode(), P = false;
          b.body = [];
          var q = [];
          for (this.expect(o.braceL); this.type !== o.braceR; ) if (this.match(R.at)) q.push(this.parseDecorator());
          else {
            var T = this.parseClassElement(r.superClass !== null);
            q.length && (T.decorators = q, this.resetStartLocationFromNode(T, q[0]), q = []), T && (b.body.push(T), T.type === "MethodDefinition" && T.kind === "constructor" && T.value.type === "FunctionExpression" ? (P && this.raiseRecoverable(T.start, "Duplicate constructor in the same class"), P = true, T.decorators && T.decorators.length > 0 && this.raise(T.start, "Decorators can't be used with a constructor. Did you mean '@dec class { ... }'?")) : T.key && T.key.type === "PrivateIdentifier" && V1(f, T) && this.raiseRecoverable(T.key.start, "Identifier '#" + T.key.name + "' has already been declared"));
          }
          return this.strict = l, this.next(), q.length && this.raise(this.start, "Decorators must be attached to a class element."), r.body = this.finishNode(b, "ClassBody"), this.exitClassBody(), this.finishNode(r, i ? "ClassDeclaration" : "ClassExpression");
        } finally {
          this.inAbstractClass = n;
        }
      }, c.parseClassFunctionParams = function() {
        var r = this.tsTryParseTypeParameters(this.tsParseConstModifier), i = this.parseBindingList(o.parenR, false, this.options.ecmaVersion >= 8, true);
        return r && (i.typeParameters = r), i;
      }, c.parseMethod = function(r, i, n, l, f) {
        var b = this.startNode(), P = this.yieldPos, q = this.awaitPos, T = this.awaitIdentPos;
        if (this.initFunction(b), this.options.ecmaVersion >= 6 && (b.generator = r), this.options.ecmaVersion >= 8 && (b.async = !!i), this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0, this.enterScope(64 | pl(i, b.generator) | (n ? 128 : 0)), this.expect(o.parenL), b.params = this.parseClassFunctionParams(), this.checkYieldAwaitInDefaultParams(), this.parseFunctionBody(b, false, true, false, { isClassMethod: l }), this.yieldPos = P, this.awaitPos = q, this.awaitIdentPos = T, f && f.abstract && b.body) {
          var H = f.key;
          this.raise(f.start, le.AbstractMethodHasImplementation({ methodName: H.type !== "Identifier" || f.computed ? "[" + this.input.slice(H.start, H.end) + "]" : H.name }));
        }
        return this.finishNode(b, "FunctionExpression");
      }, F.parse = function(r, i) {
        if (i.locations === false) throw new Error("You have to enable options.locations while using acorn-typescript");
        i.locations = true;
        var n = new this(i, r);
        return s && (n.isAmbientContext = true), n.parse();
      }, F.parseExpressionAt = function(r, i, n) {
        if (n.locations === false) throw new Error("You have to enable options.locations while using acorn-typescript");
        n.locations = true;
        var l = new this(n, r, i);
        return s && (l.isAmbientContext = true), l.nextToken(), l.parseExpression();
      }, c.parseImportSpecifier = function() {
        if (this.ts_isContextual(R.type)) {
          var r = this.startNode();
          return r.imported = this.parseModuleExportName(), this.parseTypeOnlyImportExportSpecifier(r, true, this.importOrExportOuterKind === "type"), this.finishNode(r, "ImportSpecifier");
        }
        var i = A.prototype.parseImportSpecifier.call(this);
        return i.importKind = "value", i;
      }, c.parseExportSpecifier = function(r) {
        var i = this.ts_isContextual(R.type);
        if (!this.match(o.string) && i) {
          var n = this.startNode();
          return n.local = this.parseModuleExportName(), this.parseTypeOnlyImportExportSpecifier(n, false, this.importOrExportOuterKind === "type"), this.finishNode(n, "ExportSpecifier"), this.checkExport(r, n.exported, n.exported.start), n;
        }
        var l = A.prototype.parseExportSpecifier.call(this, r);
        return l.exportKind = "value", l;
      }, c.parseTypeOnlyImportExportSpecifier = function(r, i, n) {
        var l, f = i ? "imported" : "local", b = i ? "local" : "exported", P = r[f], q = false, T = true, H = P.start;
        if (this.isContextual("as")) {
          var J = this.parseIdent();
          if (this.isContextual("as")) {
            var re = this.parseIdent();
            k(this.type) ? (q = true, P = J, l = i ? this.parseIdent() : this.parseModuleExportName(), T = false) : (l = re, T = false);
          } else k(this.type) ? (T = false, l = i ? this.parseIdent() : this.parseModuleExportName()) : (q = true, P = J);
        } else k(this.type) && (q = true, i ? (P = A.prototype.parseIdent.call(this, true), this.isContextual("as") || this.checkUnreserved(P)) : P = this.parseModuleExportName());
        q && n && this.raise(H, i ? le.TypeModifierIsUsedInTypeImports : le.TypeModifierIsUsedInTypeExports), r[f] = P, r[b] = l, r[i ? "importKind" : "exportKind"] = q ? "type" : "value", T && this.eatContextual("as") && (r[b] = i ? this.parseIdent() : this.parseModuleExportName()), r[b] || (r[b] = this.copyNode(r[f])), i && this.checkLValSimple(r[b], 2);
      }, c.raiseCommonCheck = function(r, i, n) {
        return i === "Comma is not permitted after the rest element" ? this.isAmbientContext && this.match(o.comma) && this.lookaheadCharCode() === 41 ? void this.next() : A.prototype.raise.call(this, r, i) : n ? A.prototype.raiseRecoverable.call(this, r, i) : A.prototype.raise.call(this, r, i);
      }, c.raiseRecoverable = function(r, i) {
        return this.raiseCommonCheck(r, i, true);
      }, c.raise = function(r, i) {
        return this.raiseCommonCheck(r, i, true);
      }, c.updateContext = function(r) {
        var i = this.type;
        if (i == o.braceL) {
          var n = this.curContext();
          n == h.tc_oTag ? this.context.push(G.b_expr) : n == h.tc_expr ? this.context.push(G.b_tmpl) : A.prototype.updateContext.call(this, r), this.exprAllowed = true;
        } else {
          if (i !== o.slash || r !== R.jsxTagStart) return A.prototype.updateContext.call(this, r);
          this.context.length -= 2, this.context.push(h.tc_cTag), this.exprAllowed = false;
        }
      }, c.jsx_parseOpeningElementAt = function(r, i) {
        var n = this, l = this.startNodeAt(r, i), f = this.jsx_parseElementName();
        if (f && (l.name = f), this.match(o.relational) || this.match(o.bitShift)) {
          var b = this.tsTryParseAndCatch(function() {
            return n.tsParseTypeArgumentsInExpression();
          });
          b && (l.typeParameters = b);
        }
        for (l.attributes = []; this.type !== o.slash && this.type !== R.jsxTagEnd; ) l.attributes.push(this.jsx_parseAttribute());
        return l.selfClosing = this.eat(o.slash), this.expect(R.jsxTagEnd), this.finishNode(l, f ? "JSXOpeningElement" : "JSXOpeningFragment");
      }, c.enterScope = function(r) {
        r === ir && this.importsStack.push([]), A.prototype.enterScope.call(this, r);
        var i = A.prototype.currentScope.call(this);
        i.types = [], i.enums = [], i.constEnums = [], i.classes = [], i.exportOnlyBindings = [];
      }, c.exitScope = function() {
        A.prototype.currentScope.call(this).flags === ir && this.importsStack.pop(), A.prototype.exitScope.call(this);
      }, c.hasImport = function(r, i) {
        var n = this.importsStack.length;
        if (this.importsStack[n - 1].indexOf(r) > -1) return true;
        if (!i && n > 1) {
          for (var l = 0; l < n - 1; l++) if (this.importsStack[l].indexOf(r) > -1) return true;
        }
        return false;
      }, c.maybeExportDefined = function(r, i) {
        this.inModule && 1 & r.flags && this.undefinedExports.delete(i);
      }, c.isRedeclaredInScope = function(r, i, n) {
        return !!(0 & n) && (2 & n ? r.lexical.indexOf(i) > -1 || r.functions.indexOf(i) > -1 || r.var.indexOf(i) > -1 : 3 & n ? r.lexical.indexOf(i) > -1 || !A.prototype.treatFunctionsAsVarInScope.call(this, r) && r.var.indexOf(i) > -1 : r.lexical.indexOf(i) > -1 && !(32 & r.flags && r.lexical[0] === i) || !this.treatFunctionsAsVarInScope(r) && r.functions.indexOf(i) > -1);
      }, c.checkRedeclarationInScope = function(r, i, n, l) {
        this.isRedeclaredInScope(r, i, n) && this.raise(l, "Identifier '" + i + "' has already been declared.");
      }, c.declareName = function(r, i, n) {
        if (4096 & i) return this.hasImport(r, true) && this.raise(n, "Identifier '" + r + "' has already been declared."), void this.importsStack[this.importsStack.length - 1].push(r);
        var l = this.currentScope();
        if (1024 & i) return this.maybeExportDefined(l, r), void l.exportOnlyBindings.push(r);
        A.prototype.declareName.call(this, r, i, n), 0 & i && (0 & i || (this.checkRedeclarationInScope(l, r, i, n), this.maybeExportDefined(l, r)), l.types.push(r)), 256 & i && l.enums.push(r), 512 & i && l.constEnums.push(r), 128 & i && l.classes.push(r);
      }, c.checkLocalExport = function(r) {
        var i = r.name;
        if (!this.hasImport(i)) {
          for (var n = this.scopeStack.length - 1; n >= 0; n--) {
            var l = this.scopeStack[n];
            if (l.types.indexOf(i) > -1 || l.exportOnlyBindings.indexOf(i) > -1) return;
          }
          A.prototype.checkLocalExport.call(this, r);
        }
      }, E = F, V = [{ key: "acornTypeScript", get: function() {
        return x;
      } }], (M = [{ key: "acornTypeScript", get: function() {
        return x;
      } }]) && Wl(E.prototype, M), V && Wl(E, V), Object.defineProperty(E, "prototype", { writable: false }), F;
    }(m);
    return ee;
  };
}
Ue.extend(X1({ allowSatisfies: true }));
class K1 extends Error {
  constructor(t, a, s) {
    super(a);
    __publicField(this, "message", "");
    __privateAdd(this, _e2);
    this.stack = "", __privateSet(this, _e2, new Ah(t, a, s)), Object.assign(this, __privateGet(this, _e2)), this.name = "CompileError";
  }
  toString() {
    return __privateGet(this, _e2).toString();
  }
  toJSON() {
    return __privateGet(this, _e2).toJSON();
  }
}
_e2 = new WeakMap();
function kl(e, t, a) {
  throw new K1(t, a, void 0);
}
function J1(e, t) {
  kl(e, "options_invalid_value", `Invalid compiler option: ${t}
https://svelte.dev/e/options_invalid_value`);
}
function Q1(e, t) {
  kl(e, "options_removed", `Invalid compiler option: ${t}
https://svelte.dev/e/options_removed`);
}
function Y1(e, t) {
  kl(e, "options_unrecognised", `Unrecognised compiler option ${t}
https://svelte.dev/e/options_unrecognised`);
}
const Z1 = { "CounterClockwiseContourIntegral;": 8755, "ClockwiseContourIntegral;": 8754, "DoubleLongLeftRightArrow;": 10234, "NotNestedGreaterGreater;": 10914, "DiacriticalDoubleAcute;": 733, "NotSquareSupersetEqual;": 8931, "CloseCurlyDoubleQuote;": 8221, "DoubleContourIntegral;": 8751, "FilledVerySmallSquare;": 9642, "NegativeVeryThinSpace;": 8203, "NotPrecedesSlantEqual;": 8928, "NotRightTriangleEqual;": 8941, "NotSucceedsSlantEqual;": 8929, "CapitalDifferentialD;": 8517, "DoubleLeftRightArrow;": 8660, "DoubleLongRightArrow;": 10233, "EmptyVerySmallSquare;": 9643, "NestedGreaterGreater;": 8811, "NotDoubleVerticalBar;": 8742, "NotGreaterSlantEqual;": 10878, "NotLeftTriangleEqual;": 8940, "NotSquareSubsetEqual;": 8930, "OpenCurlyDoubleQuote;": 8220, "ReverseUpEquilibrium;": 10607, "DoubleLongLeftArrow;": 10232, "DownLeftRightVector;": 10576, "LeftArrowRightArrow;": 8646, "NegativeMediumSpace;": 8203, "NotGreaterFullEqual;": 8807, "NotRightTriangleBar;": 10704, "RightArrowLeftArrow;": 8644, "SquareSupersetEqual;": 8850, "leftrightsquigarrow;": 8621, "DownRightTeeVector;": 10591, "DownRightVectorBar;": 10583, "LongLeftRightArrow;": 10231, "Longleftrightarrow;": 10234, "NegativeThickSpace;": 8203, "NotLeftTriangleBar;": 10703, "PrecedesSlantEqual;": 8828, "ReverseEquilibrium;": 8651, "RightDoubleBracket;": 10215, "RightDownTeeVector;": 10589, "RightDownVectorBar;": 10581, "RightTriangleEqual;": 8885, "SquareIntersection;": 8851, "SucceedsSlantEqual;": 8829, "blacktriangleright;": 9656, "longleftrightarrow;": 10231, "DoubleUpDownArrow;": 8661, "DoubleVerticalBar;": 8741, "DownLeftTeeVector;": 10590, "DownLeftVectorBar;": 10582, "FilledSmallSquare;": 9724, "GreaterSlantEqual;": 10878, "LeftDoubleBracket;": 10214, "LeftDownTeeVector;": 10593, "LeftDownVectorBar;": 10585, "LeftTriangleEqual;": 8884, "NegativeThinSpace;": 8203, "NotGreaterGreater;": 8811, "NotLessSlantEqual;": 10877, "NotNestedLessLess;": 10913, "NotReverseElement;": 8716, "NotSquareSuperset;": 8848, "NotTildeFullEqual;": 8775, "RightAngleBracket;": 10217, "RightUpDownVector;": 10575, "SquareSubsetEqual;": 8849, "VerticalSeparator;": 10072, "blacktriangledown;": 9662, "blacktriangleleft;": 9666, "leftrightharpoons;": 8651, "rightleftharpoons;": 8652, "twoheadrightarrow;": 8608, "DiacriticalAcute;": 180, "DiacriticalGrave;": 96, "DiacriticalTilde;": 732, "DoubleRightArrow;": 8658, "DownArrowUpArrow;": 8693, "EmptySmallSquare;": 9723, "GreaterEqualLess;": 8923, "GreaterFullEqual;": 8807, "LeftAngleBracket;": 10216, "LeftUpDownVector;": 10577, "LessEqualGreater;": 8922, "NonBreakingSpace;": 160, "NotPrecedesEqual;": 10927, "NotRightTriangle;": 8939, "NotSucceedsEqual;": 10928, "NotSucceedsTilde;": 8831, "NotSupersetEqual;": 8841, "RightTriangleBar;": 10704, "RightUpTeeVector;": 10588, "RightUpVectorBar;": 10580, "UnderParenthesis;": 9181, "UpArrowDownArrow;": 8645, "circlearrowright;": 8635, "downharpoonright;": 8642, "ntrianglerighteq;": 8941, "rightharpoondown;": 8641, "rightrightarrows;": 8649, "twoheadleftarrow;": 8606, "vartriangleright;": 8883, "CloseCurlyQuote;": 8217, "ContourIntegral;": 8750, "DoubleDownArrow;": 8659, "DoubleLeftArrow;": 8656, "DownRightVector;": 8641, "LeftRightVector;": 10574, "LeftTriangleBar;": 10703, "LeftUpTeeVector;": 10592, "LeftUpVectorBar;": 10584, "LowerRightArrow;": 8600, "NotGreaterEqual;": 8817, "NotGreaterTilde;": 8821, "NotHumpDownHump;": 8782, "NotLeftTriangle;": 8938, "NotSquareSubset;": 8847, "OverParenthesis;": 9180, "RightDownVector;": 8642, "ShortRightArrow;": 8594, "UpperRightArrow;": 8599, "bigtriangledown;": 9661, "circlearrowleft;": 8634, "curvearrowright;": 8631, "downharpoonleft;": 8643, "leftharpoondown;": 8637, "leftrightarrows;": 8646, "nLeftrightarrow;": 8654, "nleftrightarrow;": 8622, "ntrianglelefteq;": 8940, "rightleftarrows;": 8644, "rightsquigarrow;": 8605, "rightthreetimes;": 8908, "straightepsilon;": 1013, "trianglerighteq;": 8885, "vartriangleleft;": 8882, "DiacriticalDot;": 729, "DoubleRightTee;": 8872, "DownLeftVector;": 8637, "GreaterGreater;": 10914, "HorizontalLine;": 9472, "InvisibleComma;": 8291, "InvisibleTimes;": 8290, "LeftDownVector;": 8643, "LeftRightArrow;": 8596, "Leftrightarrow;": 8660, "LessSlantEqual;": 10877, "LongRightArrow;": 10230, "Longrightarrow;": 10233, "LowerLeftArrow;": 8601, "NestedLessLess;": 8810, "NotGreaterLess;": 8825, "NotLessGreater;": 8824, "NotSubsetEqual;": 8840, "NotVerticalBar;": 8740, "OpenCurlyQuote;": 8216, "ReverseElement;": 8715, "RightTeeVector;": 10587, "RightVectorBar;": 10579, "ShortDownArrow;": 8595, "ShortLeftArrow;": 8592, "SquareSuperset;": 8848, "TildeFullEqual;": 8773, "UpperLeftArrow;": 8598, "ZeroWidthSpace;": 8203, "curvearrowleft;": 8630, "doublebarwedge;": 8966, "downdownarrows;": 8650, "hookrightarrow;": 8618, "leftleftarrows;": 8647, "leftrightarrow;": 8596, "leftthreetimes;": 8907, "longrightarrow;": 10230, "looparrowright;": 8620, "nshortparallel;": 8742, "ntriangleright;": 8939, "rightarrowtail;": 8611, "rightharpoonup;": 8640, "trianglelefteq;": 8884, "upharpoonright;": 8638, "ApplyFunction;": 8289, "DifferentialD;": 8518, "DoubleLeftTee;": 10980, "DoubleUpArrow;": 8657, "LeftTeeVector;": 10586, "LeftVectorBar;": 10578, "LessFullEqual;": 8806, "LongLeftArrow;": 10229, "Longleftarrow;": 10232, "NotEqualTilde;": 8770, "NotTildeEqual;": 8772, "NotTildeTilde;": 8777, "Poincareplane;": 8460, "PrecedesEqual;": 10927, "PrecedesTilde;": 8830, "RightArrowBar;": 8677, "RightTeeArrow;": 8614, "RightTriangle;": 8883, "RightUpVector;": 8638, "SucceedsEqual;": 10928, "SucceedsTilde;": 8831, "SupersetEqual;": 8839, "UpEquilibrium;": 10606, "VerticalTilde;": 8768, "VeryThinSpace;": 8202, "bigtriangleup;": 9651, "blacktriangle;": 9652, "divideontimes;": 8903, "fallingdotseq;": 8786, "hookleftarrow;": 8617, "leftarrowtail;": 8610, "leftharpoonup;": 8636, "longleftarrow;": 10229, "looparrowleft;": 8619, "measuredangle;": 8737, "ntriangleleft;": 8938, "shortparallel;": 8741, "smallsetminus;": 8726, "triangleright;": 9657, "upharpoonleft;": 8639, "varsubsetneqq;": 10955, "varsupsetneqq;": 10956, "DownArrowBar;": 10515, "DownTeeArrow;": 8615, "ExponentialE;": 8519, "GreaterEqual;": 8805, "GreaterTilde;": 8819, "HilbertSpace;": 8459, "HumpDownHump;": 8782, "Intersection;": 8898, "LeftArrowBar;": 8676, "LeftTeeArrow;": 8612, "LeftTriangle;": 8882, "LeftUpVector;": 8639, "NotCongruent;": 8802, "NotHumpEqual;": 8783, "NotLessEqual;": 8816, "NotLessTilde;": 8820, "Proportional;": 8733, "RightCeiling;": 8969, "RoundImplies;": 10608, "ShortUpArrow;": 8593, "SquareSubset;": 8847, "UnderBracket;": 9141, "VerticalLine;": 124, "blacklozenge;": 10731, "exponentiale;": 8519, "risingdotseq;": 8787, "triangledown;": 9663, "triangleleft;": 9667, "varsubsetneq;": 8842, "varsupsetneq;": 8843, "CircleMinus;": 8854, "CircleTimes;": 8855, "Equilibrium;": 8652, "GreaterLess;": 8823, "LeftCeiling;": 8968, "LessGreater;": 8822, "MediumSpace;": 8287, "NotLessLess;": 8810, "NotPrecedes;": 8832, "NotSucceeds;": 8833, "NotSuperset;": 8835, "OverBracket;": 9140, "RightVector;": 8640, "Rrightarrow;": 8667, "RuleDelayed;": 10740, "SmallCircle;": 8728, "SquareUnion;": 8852, "SubsetEqual;": 8838, "UpDownArrow;": 8597, "Updownarrow;": 8661, "VerticalBar;": 8739, "backepsilon;": 1014, "blacksquare;": 9642, "circledcirc;": 8858, "circleddash;": 8861, "curlyeqprec;": 8926, "curlyeqsucc;": 8927, "diamondsuit;": 9830, "eqslantless;": 10901, "expectation;": 8496, "nRightarrow;": 8655, "nrightarrow;": 8603, "preccurlyeq;": 8828, "precnapprox;": 10937, "quaternions;": 8461, "straightphi;": 981, "succcurlyeq;": 8829, "succnapprox;": 10938, "thickapprox;": 8776, "updownarrow;": 8597, "Bernoullis;": 8492, "CirclePlus;": 8853, "EqualTilde;": 8770, "Fouriertrf;": 8497, "ImaginaryI;": 8520, "Laplacetrf;": 8466, "LeftVector;": 8636, "Lleftarrow;": 8666, "NotElement;": 8713, "NotGreater;": 8815, "Proportion;": 8759, "RightArrow;": 8594, "RightFloor;": 8971, "Rightarrow;": 8658, "ThickSpace;": 8287, "TildeEqual;": 8771, "TildeTilde;": 8776, "UnderBrace;": 9183, "UpArrowBar;": 10514, "UpTeeArrow;": 8613, "circledast;": 8859, "complement;": 8705, "curlywedge;": 8911, "eqslantgtr;": 10902, "gtreqqless;": 10892, "lessapprox;": 10885, "lesseqqgtr;": 10891, "lmoustache;": 9136, "longmapsto;": 10236, "mapstodown;": 8615, "mapstoleft;": 8612, "nLeftarrow;": 8653, "nleftarrow;": 8602, "nsubseteqq;": 10949, "nsupseteqq;": 10950, "precapprox;": 10935, "rightarrow;": 8594, "rmoustache;": 9137, "sqsubseteq;": 8849, "sqsupseteq;": 8850, "subsetneqq;": 10955, "succapprox;": 10936, "supsetneqq;": 10956, "upuparrows;": 8648, "varepsilon;": 1013, "varnothing;": 8709, "Backslash;": 8726, "CenterDot;": 183, "CircleDot;": 8857, "Congruent;": 8801, "Coproduct;": 8720, "DoubleDot;": 168, "DownArrow;": 8595, "DownBreve;": 785, "Downarrow;": 8659, "HumpEqual;": 8783, "LeftArrow;": 8592, "LeftFloor;": 8970, "Leftarrow;": 8656, "LessTilde;": 8818, "Mellintrf;": 8499, "MinusPlus;": 8723, "NotCupCap;": 8813, "NotExists;": 8708, "NotSubset;": 8834, "OverBrace;": 9182, "PlusMinus;": 177, "Therefore;": 8756, "ThinSpace;": 8201, "TripleDot;": 8411, "UnionPlus;": 8846, "backprime;": 8245, "backsimeq;": 8909, "bigotimes;": 10754, "centerdot;": 183, "checkmark;": 10003, "complexes;": 8450, "dotsquare;": 8865, "downarrow;": 8595, "gtrapprox;": 10886, "gtreqless;": 8923, "gvertneqq;": 8809, "heartsuit;": 9829, "leftarrow;": 8592, "lesseqgtr;": 8922, "lvertneqq;": 8808, "ngeqslant;": 10878, "nleqslant;": 10877, "nparallel;": 8742, "nshortmid;": 8740, "nsubseteq;": 8840, "nsupseteq;": 8841, "pitchfork;": 8916, "rationals;": 8474, "spadesuit;": 9824, "subseteqq;": 10949, "subsetneq;": 8842, "supseteqq;": 10950, "supsetneq;": 8843, "therefore;": 8756, "triangleq;": 8796, "varpropto;": 8733, "DDotrahd;": 10513, "DotEqual;": 8784, "Integral;": 8747, "LessLess;": 10913, "NotEqual;": 8800, "NotTilde;": 8769, "PartialD;": 8706, "Precedes;": 8826, "RightTee;": 8866, "Succeeds;": 8827, "SuchThat;": 8715, "Superset;": 8835, "Uarrocir;": 10569, "UnderBar;": 95, "andslope;": 10840, "angmsdaa;": 10664, "angmsdab;": 10665, "angmsdac;": 10666, "angmsdad;": 10667, "angmsdae;": 10668, "angmsdaf;": 10669, "angmsdag;": 10670, "angmsdah;": 10671, "angrtvbd;": 10653, "approxeq;": 8778, "awconint;": 8755, "backcong;": 8780, "barwedge;": 8965, "bbrktbrk;": 9142, "bigoplus;": 10753, "bigsqcup;": 10758, "biguplus;": 10756, "bigwedge;": 8896, "boxminus;": 8863, "boxtimes;": 8864, "bsolhsub;": 10184, "capbrcup;": 10825, "circledR;": 174, "circledS;": 9416, "cirfnint;": 10768, "clubsuit;": 9827, "cupbrcap;": 10824, "curlyvee;": 8910, "cwconint;": 8754, "doteqdot;": 8785, "dotminus;": 8760, "drbkarow;": 10512, "dzigrarr;": 10239, "elinters;": 9191, "emptyset;": 8709, "eqvparsl;": 10725, "fpartint;": 10765, "geqslant;": 10878, "gesdotol;": 10884, "gnapprox;": 10890, "hksearow;": 10533, "hkswarow;": 10534, "imagline;": 8464, "imagpart;": 8465, "infintie;": 10717, "integers;": 8484, "intercal;": 8890, "intlarhk;": 10775, "laemptyv;": 10676, "ldrushar;": 10571, "leqslant;": 10877, "lesdotor;": 10883, "llcorner;": 8990, "lnapprox;": 10889, "lrcorner;": 8991, "lurdshar;": 10570, "mapstoup;": 8613, "multimap;": 8888, "naturals;": 8469, "ncongdot;": 10861, "notindot;": 8949, "otimesas;": 10806, "parallel;": 8741, "plusacir;": 10787, "pointint;": 10773, "precneqq;": 10933, "precnsim;": 8936, "profalar;": 9006, "profline;": 8978, "profsurf;": 8979, "raemptyv;": 10675, "realpart;": 8476, "rppolint;": 10770, "rtriltri;": 10702, "scpolint;": 10771, "setminus;": 8726, "shortmid;": 8739, "smeparsl;": 10724, "sqsubset;": 8847, "sqsupset;": 8848, "subseteq;": 8838, "succneqq;": 10934, "succnsim;": 8937, "supseteq;": 8839, "thetasym;": 977, "thicksim;": 8764, "timesbar;": 10801, "triangle;": 9653, "triminus;": 10810, "trpezium;": 9186, "ulcorner;": 8988, "urcorner;": 8989, "varkappa;": 1008, "varsigma;": 962, "vartheta;": 977, "Because;": 8757, "Cayleys;": 8493, "Cconint;": 8752, "Cedilla;": 184, "Diamond;": 8900, "DownTee;": 8868, "Element;": 8712, "Epsilon;": 917, "Implies;": 8658, "LeftTee;": 8867, "NewLine;": 10, "NoBreak;": 8288, "NotLess;": 8814, "Omicron;": 927, "OverBar;": 8254, "Product;": 8719, "UpArrow;": 8593, "Uparrow;": 8657, "Upsilon;": 933, "alefsym;": 8501, "angrtvb;": 8894, "angzarr;": 9084, "asympeq;": 8781, "backsim;": 8765, "because;": 8757, "bemptyv;": 10672, "between;": 8812, "bigcirc;": 9711, "bigodot;": 10752, "bigstar;": 9733, "bnequiv;": 8801, "boxplus;": 8862, "ccupssm;": 10832, "cemptyv;": 10674, "cirscir;": 10690, "coloneq;": 8788, "congdot;": 10861, "cudarrl;": 10552, "cudarrr;": 10549, "cularrp;": 10557, "curarrm;": 10556, "dbkarow;": 10511, "ddagger;": 8225, "ddotseq;": 10871, "demptyv;": 10673, "diamond;": 8900, "digamma;": 989, "dotplus;": 8724, "dwangle;": 10662, "epsilon;": 949, "eqcolon;": 8789, "equivDD;": 10872, "gesdoto;": 10882, "gtquest;": 10876, "gtrless;": 8823, "harrcir;": 10568, "intprod;": 10812, "isindot;": 8949, "larrbfs;": 10527, "larrsim;": 10611, "lbrksld;": 10639, "lbrkslu;": 10637, "ldrdhar;": 10599, "lesdoto;": 10881, "lessdot;": 8918, "lessgtr;": 8822, "lesssim;": 8818, "lotimes;": 10804, "lozenge;": 9674, "ltquest;": 10875, "luruhar;": 10598, "maltese;": 10016, "minusdu;": 10794, "napprox;": 8777, "natural;": 9838, "nearrow;": 8599, "nexists;": 8708, "notinva;": 8713, "notinvb;": 8951, "notinvc;": 8950, "notniva;": 8716, "notnivb;": 8958, "notnivc;": 8957, "npolint;": 10772, "npreceq;": 10927, "nsqsube;": 8930, "nsqsupe;": 8931, "nsubset;": 8834, "nsucceq;": 10928, "nsupset;": 8835, "nvinfin;": 10718, "nvltrie;": 8884, "nvrtrie;": 8885, "nwarrow;": 8598, "olcross;": 10683, "omicron;": 959, "orderof;": 8500, "orslope;": 10839, "pertenk;": 8241, "planckh;": 8462, "pluscir;": 10786, "plussim;": 10790, "plustwo;": 10791, "precsim;": 8830, "quatint;": 10774, "questeq;": 8799, "rarrbfs;": 10528, "rarrsim;": 10612, "rbrksld;": 10638, "rbrkslu;": 10640, "rdldhar;": 10601, "realine;": 8475, "rotimes;": 10805, "ruluhar;": 10600, "searrow;": 8600, "simplus;": 10788, "simrarr;": 10610, "subedot;": 10947, "submult;": 10945, "subplus;": 10943, "subrarr;": 10617, "succsim;": 8831, "supdsub;": 10968, "supedot;": 10948, "suphsol;": 10185, "suphsub;": 10967, "suplarr;": 10619, "supmult;": 10946, "supplus;": 10944, "swarrow;": 8601, "topfork;": 10970, "triplus;": 10809, "tritime;": 10811, "uparrow;": 8593, "upsilon;": 965, "uwangle;": 10663, "vzigzag;": 10650, "zigrarr;": 8669, "Aacute;": 193, "Abreve;": 258, "Agrave;": 192, "Assign;": 8788, "Atilde;": 195, "Barwed;": 8966, "Bumpeq;": 8782, "Cacute;": 262, "Ccaron;": 268, "Ccedil;": 199, "Colone;": 10868, "Conint;": 8751, "CupCap;": 8781, "Dagger;": 8225, "Dcaron;": 270, "DotDot;": 8412, "Dstrok;": 272, "Eacute;": 201, "Ecaron;": 282, "Egrave;": 200, "Exists;": 8707, "ForAll;": 8704, "Gammad;": 988, "Gbreve;": 286, "Gcedil;": 290, "HARDcy;": 1066, "Hstrok;": 294, "Iacute;": 205, "Igrave;": 204, "Itilde;": 296, "Jsercy;": 1032, "Kcedil;": 310, "Lacute;": 313, "Lambda;": 923, "Lcaron;": 317, "Lcedil;": 315, "Lmidot;": 319, "Lstrok;": 321, "Nacute;": 323, "Ncaron;": 327, "Ncedil;": 325, "Ntilde;": 209, "Oacute;": 211, "Odblac;": 336, "Ograve;": 210, "Oslash;": 216, "Otilde;": 213, "Otimes;": 10807, "Racute;": 340, "Rarrtl;": 10518, "Rcaron;": 344, "Rcedil;": 342, "SHCHcy;": 1065, "SOFTcy;": 1068, "Sacute;": 346, "Scaron;": 352, "Scedil;": 350, "Square;": 9633, "Subset;": 8912, "Supset;": 8913, "Tcaron;": 356, "Tcedil;": 354, "Tstrok;": 358, "Uacute;": 218, "Ubreve;": 364, "Udblac;": 368, "Ugrave;": 217, "Utilde;": 360, "Vdashl;": 10982, "Verbar;": 8214, "Vvdash;": 8874, "Yacute;": 221, "Zacute;": 377, "Zcaron;": 381, "aacute;": 225, "abreve;": 259, "agrave;": 224, "andand;": 10837, "angmsd;": 8737, "angsph;": 8738, "apacir;": 10863, "approx;": 8776, "atilde;": 227, "barvee;": 8893, "barwed;": 8965, "becaus;": 8757, "bernou;": 8492, "bigcap;": 8898, "bigcup;": 8899, "bigvee;": 8897, "bkarow;": 10509, "bottom;": 8869, "bowtie;": 8904, "boxbox;": 10697, "bprime;": 8245, "brvbar;": 166, "bullet;": 8226, "bumpeq;": 8783, "cacute;": 263, "capand;": 10820, "capcap;": 10827, "capcup;": 10823, "capdot;": 10816, "ccaron;": 269, "ccedil;": 231, "circeq;": 8791, "cirmid;": 10991, "colone;": 8788, "commat;": 64, "compfn;": 8728, "conint;": 8750, "coprod;": 8720, "copysr;": 8471, "cularr;": 8630, "cupcap;": 10822, "cupcup;": 10826, "cupdot;": 8845, "curarr;": 8631, "curren;": 164, "cylcty;": 9005, "dagger;": 8224, "daleth;": 8504, "dcaron;": 271, "dfisht;": 10623, "divide;": 247, "divonx;": 8903, "dlcorn;": 8990, "dlcrop;": 8973, "dollar;": 36, "drcorn;": 8991, "drcrop;": 8972, "dstrok;": 273, "eacute;": 233, "easter;": 10862, "ecaron;": 283, "ecolon;": 8789, "egrave;": 232, "egsdot;": 10904, "elsdot;": 10903, "emptyv;": 8709, "emsp13;": 8196, "emsp14;": 8197, "eparsl;": 10723, "eqcirc;": 8790, "equals;": 61, "equest;": 8799, "female;": 9792, "ffilig;": 64259, "ffllig;": 64260, "forall;": 8704, "frac12;": 189, "frac13;": 8531, "frac14;": 188, "frac15;": 8533, "frac16;": 8537, "frac18;": 8539, "frac23;": 8532, "frac25;": 8534, "frac34;": 190, "frac35;": 8535, "frac38;": 8540, "frac45;": 8536, "frac56;": 8538, "frac58;": 8541, "frac78;": 8542, "gacute;": 501, "gammad;": 989, "gbreve;": 287, "gesdot;": 10880, "gesles;": 10900, "gtlPar;": 10645, "gtrarr;": 10616, "gtrdot;": 8919, "gtrsim;": 8819, "hairsp;": 8202, "hamilt;": 8459, "hardcy;": 1098, "hearts;": 9829, "hellip;": 8230, "hercon;": 8889, "homtht;": 8763, "horbar;": 8213, "hslash;": 8463, "hstrok;": 295, "hybull;": 8259, "hyphen;": 8208, "iacute;": 237, "igrave;": 236, "iiiint;": 10764, "iinfin;": 10716, "incare;": 8453, "inodot;": 305, "intcal;": 8890, "iquest;": 191, "isinsv;": 8947, "itilde;": 297, "jsercy;": 1112, "kappav;": 1008, "kcedil;": 311, "kgreen;": 312, "lAtail;": 10523, "lacute;": 314, "lagran;": 8466, "lambda;": 955, "langle;": 10216, "larrfs;": 10525, "larrhk;": 8617, "larrlp;": 8619, "larrpl;": 10553, "larrtl;": 8610, "latail;": 10521, "lbrace;": 123, "lbrack;": 91, "lcaron;": 318, "lcedil;": 316, "ldquor;": 8222, "lesdot;": 10879, "lesges;": 10899, "lfisht;": 10620, "lfloor;": 8970, "lharul;": 10602, "llhard;": 10603, "lmidot;": 320, "lmoust;": 9136, "loplus;": 10797, "lowast;": 8727, "lowbar;": 95, "lparlt;": 10643, "lrhard;": 10605, "lsaquo;": 8249, "lsquor;": 8218, "lstrok;": 322, "lthree;": 8907, "ltimes;": 8905, "ltlarr;": 10614, "ltrPar;": 10646, "mapsto;": 8614, "marker;": 9646, "mcomma;": 10793, "midast;": 42, "midcir;": 10992, "middot;": 183, "minusb;": 8863, "minusd;": 8760, "mnplus;": 8723, "models;": 8871, "mstpos;": 8766, "nVDash;": 8879, "nVdash;": 8878, "nacute;": 324, "nbumpe;": 8783, "ncaron;": 328, "ncedil;": 326, "nearhk;": 10532, "nequiv;": 8802, "nesear;": 10536, "nexist;": 8708, "nltrie;": 8940, "notinE;": 8953, "nparsl;": 11005, "nprcue;": 8928, "nrarrc;": 10547, "nrarrw;": 8605, "nrtrie;": 8941, "nsccue;": 8929, "nsimeq;": 8772, "ntilde;": 241, "numero;": 8470, "nvDash;": 8877, "nvHarr;": 10500, "nvdash;": 8876, "nvlArr;": 10498, "nvrArr;": 10499, "nwarhk;": 10531, "nwnear;": 10535, "oacute;": 243, "odblac;": 337, "odsold;": 10684, "ograve;": 242, "ominus;": 8854, "origof;": 8886, "oslash;": 248, "otilde;": 245, "otimes;": 8855, "parsim;": 10995, "percnt;": 37, "period;": 46, "permil;": 8240, "phmmat;": 8499, "planck;": 8463, "plankv;": 8463, "plusdo;": 8724, "plusdu;": 10789, "plusmn;": 177, "preceq;": 10927, "primes;": 8473, "prnsim;": 8936, "propto;": 8733, "prurel;": 8880, "puncsp;": 8200, "qprime;": 8279, "rAtail;": 10524, "racute;": 341, "rangle;": 10217, "rarrap;": 10613, "rarrfs;": 10526, "rarrhk;": 8618, "rarrlp;": 8620, "rarrpl;": 10565, "rarrtl;": 8611, "ratail;": 10522, "rbrace;": 125, "rbrack;": 93, "rcaron;": 345, "rcedil;": 343, "rdquor;": 8221, "rfisht;": 10621, "rfloor;": 8971, "rharul;": 10604, "rmoust;": 9137, "roplus;": 10798, "rpargt;": 10644, "rsaquo;": 8250, "rsquor;": 8217, "rthree;": 8908, "rtimes;": 8906, "sacute;": 347, "scaron;": 353, "scedil;": 351, "scnsim;": 8937, "searhk;": 10533, "seswar;": 10537, "sfrown;": 8994, "shchcy;": 1097, "sigmaf;": 962, "sigmav;": 962, "simdot;": 10858, "smashp;": 10803, "softcy;": 1100, "solbar;": 9023, "spades;": 9824, "sqcaps;": 8851, "sqcups;": 8852, "sqsube;": 8849, "sqsupe;": 8850, "square;": 9633, "squarf;": 9642, "ssetmn;": 8726, "ssmile;": 8995, "sstarf;": 8902, "subdot;": 10941, "subset;": 8834, "subsim;": 10951, "subsub;": 10965, "subsup;": 10963, "succeq;": 10928, "supdot;": 10942, "supset;": 8835, "supsim;": 10952, "supsub;": 10964, "supsup;": 10966, "swarhk;": 10534, "swnwar;": 10538, "target;": 8982, "tcaron;": 357, "tcedil;": 355, "telrec;": 8981, "there4;": 8756, "thetav;": 977, "thinsp;": 8201, "thksim;": 8764, "timesb;": 8864, "timesd;": 10800, "topbot;": 9014, "topcir;": 10993, "tprime;": 8244, "tridot;": 9708, "tstrok;": 359, "uacute;": 250, "ubreve;": 365, "udblac;": 369, "ufisht;": 10622, "ugrave;": 249, "ulcorn;": 8988, "ulcrop;": 8975, "urcorn;": 8989, "urcrop;": 8974, "utilde;": 361, "vangrt;": 10652, "varphi;": 981, "varrho;": 1009, "veebar;": 8891, "vellip;": 8942, "verbar;": 124, "vsubnE;": 10955, "vsubne;": 8842, "vsupnE;": 10956, "vsupne;": 8843, "wedbar;": 10847, "wedgeq;": 8793, "weierp;": 8472, "wreath;": 8768, "xoplus;": 10753, "xotime;": 10754, "xsqcup;": 10758, "xuplus;": 10756, "xwedge;": 8896, "yacute;": 253, "zacute;": 378, "zcaron;": 382, "zeetrf;": 8488, "AElig;": 198, Aacute: 193, "Acirc;": 194, Agrave: 192, "Alpha;": 913, "Amacr;": 256, "Aogon;": 260, "Aring;": 197, Atilde: 195, "Breve;": 728, Ccedil: 199, "Ccirc;": 264, "Colon;": 8759, "Cross;": 10799, "Dashv;": 10980, "Delta;": 916, Eacute: 201, "Ecirc;": 202, Egrave: 200, "Emacr;": 274, "Eogon;": 280, "Equal;": 10869, "Gamma;": 915, "Gcirc;": 284, "Hacek;": 711, "Hcirc;": 292, "IJlig;": 306, Iacute: 205, "Icirc;": 206, Igrave: 204, "Imacr;": 298, "Iogon;": 302, "Iukcy;": 1030, "Jcirc;": 308, "Jukcy;": 1028, "Kappa;": 922, Ntilde: 209, "OElig;": 338, Oacute: 211, "Ocirc;": 212, Ograve: 210, "Omacr;": 332, "Omega;": 937, Oslash: 216, Otilde: 213, "Prime;": 8243, "RBarr;": 10512, "Scirc;": 348, "Sigma;": 931, "THORN;": 222, "TRADE;": 8482, "TSHcy;": 1035, "Theta;": 920, "Tilde;": 8764, Uacute: 218, "Ubrcy;": 1038, "Ucirc;": 219, Ugrave: 217, "Umacr;": 362, "Union;": 8899, "Uogon;": 370, "UpTee;": 8869, "Uring;": 366, "VDash;": 8875, "Vdash;": 8873, "Wcirc;": 372, "Wedge;": 8896, Yacute: 221, "Ycirc;": 374, aacute: 225, "acirc;": 226, "acute;": 180, "aelig;": 230, agrave: 224, "aleph;": 8501, "alpha;": 945, "amacr;": 257, "amalg;": 10815, "angle;": 8736, "angrt;": 8735, "angst;": 197, "aogon;": 261, "aring;": 229, "asymp;": 8776, atilde: 227, "awint;": 10769, "bcong;": 8780, "bdquo;": 8222, "bepsi;": 1014, "blank;": 9251, "blk12;": 9618, "blk14;": 9617, "blk34;": 9619, "block;": 9608, "boxDL;": 9559, "boxDR;": 9556, "boxDl;": 9558, "boxDr;": 9555, "boxHD;": 9574, "boxHU;": 9577, "boxHd;": 9572, "boxHu;": 9575, "boxUL;": 9565, "boxUR;": 9562, "boxUl;": 9564, "boxUr;": 9561, "boxVH;": 9580, "boxVL;": 9571, "boxVR;": 9568, "boxVh;": 9579, "boxVl;": 9570, "boxVr;": 9567, "boxdL;": 9557, "boxdR;": 9554, "boxdl;": 9488, "boxdr;": 9484, "boxhD;": 9573, "boxhU;": 9576, "boxhd;": 9516, "boxhu;": 9524, "boxuL;": 9563, "boxuR;": 9560, "boxul;": 9496, "boxur;": 9492, "boxvH;": 9578, "boxvL;": 9569, "boxvR;": 9566, "boxvh;": 9532, "boxvl;": 9508, "boxvr;": 9500, "breve;": 728, brvbar: 166, "bsemi;": 8271, "bsime;": 8909, "bsolb;": 10693, "bumpE;": 10926, "bumpe;": 8783, "caret;": 8257, "caron;": 711, "ccaps;": 10829, ccedil: 231, "ccirc;": 265, "ccups;": 10828, "cedil;": 184, "check;": 10003, "clubs;": 9827, "colon;": 58, "comma;": 44, "crarr;": 8629, "cross;": 10007, "csube;": 10961, "csupe;": 10962, "ctdot;": 8943, "cuepr;": 8926, "cuesc;": 8927, "cupor;": 10821, curren: 164, "cuvee;": 8910, "cuwed;": 8911, "cwint;": 8753, "dashv;": 8867, "dblac;": 733, "ddarr;": 8650, "delta;": 948, "dharl;": 8643, "dharr;": 8642, "diams;": 9830, "disin;": 8946, divide: 247, "doteq;": 8784, "dtdot;": 8945, "dtrif;": 9662, "duarr;": 8693, "duhar;": 10607, "eDDot;": 10871, eacute: 233, "ecirc;": 234, "efDot;": 8786, egrave: 232, "emacr;": 275, "empty;": 8709, "eogon;": 281, "eplus;": 10865, "epsiv;": 1013, "eqsim;": 8770, "equiv;": 8801, "erDot;": 8787, "erarr;": 10609, "esdot;": 8784, "exist;": 8707, "fflig;": 64256, "filig;": 64257, "fjlig;": 102, "fllig;": 64258, "fltns;": 9649, "forkv;": 10969, frac12: 189, frac14: 188, frac34: 190, "frasl;": 8260, "frown;": 8994, "gamma;": 947, "gcirc;": 285, "gescc;": 10921, "gimel;": 8503, "gneqq;": 8809, "gnsim;": 8935, "grave;": 96, "gsime;": 10894, "gsiml;": 10896, "gtcir;": 10874, "gtdot;": 8919, "harrw;": 8621, "hcirc;": 293, "hoarr;": 8703, iacute: 237, "icirc;": 238, "iexcl;": 161, igrave: 236, "iiint;": 8749, "iiota;": 8489, "ijlig;": 307, "imacr;": 299, "image;": 8465, "imath;": 305, "imped;": 437, "infin;": 8734, "iogon;": 303, "iprod;": 10812, iquest: 191, "isinE;": 8953, "isins;": 8948, "isinv;": 8712, "iukcy;": 1110, "jcirc;": 309, "jmath;": 567, "jukcy;": 1108, "kappa;": 954, "lAarr;": 8666, "lBarr;": 10510, "langd;": 10641, "laquo;": 171, "larrb;": 8676, "lates;": 10925, "lbarr;": 10508, "lbbrk;": 10098, "lbrke;": 10635, "lceil;": 8968, "ldquo;": 8220, "lescc;": 10920, "lhard;": 8637, "lharu;": 8636, "lhblk;": 9604, "llarr;": 8647, "lltri;": 9722, "lneqq;": 8808, "lnsim;": 8934, "loang;": 10220, "loarr;": 8701, "lobrk;": 10214, "lopar;": 10629, "lrarr;": 8646, "lrhar;": 8651, "lrtri;": 8895, "lsime;": 10893, "lsimg;": 10895, "lsquo;": 8216, "ltcir;": 10873, "ltdot;": 8918, "ltrie;": 8884, "ltrif;": 9666, "mDDot;": 8762, "mdash;": 8212, "micro;": 181, middot: 183, "minus;": 8722, "mumap;": 8888, "nabla;": 8711, "napid;": 8779, "napos;": 329, "natur;": 9838, "nbump;": 8782, "ncong;": 8775, "ndash;": 8211, "neArr;": 8663, "nearr;": 8599, "nedot;": 8784, "nesim;": 8770, "ngeqq;": 8807, "ngsim;": 8821, "nhArr;": 8654, "nharr;": 8622, "nhpar;": 10994, "nlArr;": 8653, "nlarr;": 8602, "nleqq;": 8806, "nless;": 8814, "nlsim;": 8820, "nltri;": 8938, "notin;": 8713, "notni;": 8716, "npart;": 8706, "nprec;": 8832, "nrArr;": 8655, "nrarr;": 8603, "nrtri;": 8939, "nsime;": 8772, "nsmid;": 8740, "nspar;": 8742, "nsubE;": 10949, "nsube;": 8840, "nsucc;": 8833, "nsupE;": 10950, "nsupe;": 8841, ntilde: 241, "numsp;": 8199, "nvsim;": 8764, "nwArr;": 8662, "nwarr;": 8598, oacute: 243, "ocirc;": 244, "odash;": 8861, "oelig;": 339, "ofcir;": 10687, ograve: 242, "ohbar;": 10677, "olarr;": 8634, "olcir;": 10686, "oline;": 8254, "omacr;": 333, "omega;": 969, "operp;": 10681, "oplus;": 8853, "orarr;": 8635, "order;": 8500, oslash: 248, otilde: 245, "ovbar;": 9021, "parsl;": 11005, "phone;": 9742, "plusb;": 8862, "pluse;": 10866, plusmn: 177, "pound;": 163, "prcue;": 8828, "prime;": 8242, "prnap;": 10937, "prsim;": 8830, "quest;": 63, "rAarr;": 8667, "rBarr;": 10511, "radic;": 8730, "rangd;": 10642, "range;": 10661, "raquo;": 187, "rarrb;": 8677, "rarrc;": 10547, "rarrw;": 8605, "ratio;": 8758, "rbarr;": 10509, "rbbrk;": 10099, "rbrke;": 10636, "rceil;": 8969, "rdquo;": 8221, "reals;": 8477, "rhard;": 8641, "rharu;": 8640, "rlarr;": 8644, "rlhar;": 8652, "rnmid;": 10990, "roang;": 10221, "roarr;": 8702, "robrk;": 10215, "ropar;": 10630, "rrarr;": 8649, "rsquo;": 8217, "rtrie;": 8885, "rtrif;": 9656, "sbquo;": 8218, "sccue;": 8829, "scirc;": 349, "scnap;": 10938, "scsim;": 8831, "sdotb;": 8865, "sdote;": 10854, "seArr;": 8664, "searr;": 8600, "setmn;": 8726, "sharp;": 9839, "sigma;": 963, "simeq;": 8771, "simgE;": 10912, "simlE;": 10911, "simne;": 8774, "slarr;": 8592, "smile;": 8995, "smtes;": 10924, "sqcap;": 8851, "sqcup;": 8852, "sqsub;": 8847, "sqsup;": 8848, "srarr;": 8594, "starf;": 9733, "strns;": 175, "subnE;": 10955, "subne;": 8842, "supnE;": 10956, "supne;": 8843, "swArr;": 8665, "swarr;": 8601, "szlig;": 223, "theta;": 952, "thkap;": 8776, "thorn;": 254, "tilde;": 732, "times;": 215, "trade;": 8482, "trisb;": 10701, "tshcy;": 1115, "twixt;": 8812, uacute: 250, "ubrcy;": 1118, "ucirc;": 251, "udarr;": 8645, "udhar;": 10606, ugrave: 249, "uharl;": 8639, "uharr;": 8638, "uhblk;": 9600, "ultri;": 9720, "umacr;": 363, "uogon;": 371, "uplus;": 8846, "upsih;": 978, "uring;": 367, "urtri;": 9721, "utdot;": 8944, "utrif;": 9652, "uuarr;": 8648, "vBarv;": 10985, "vDash;": 8872, "varpi;": 982, "vdash;": 8866, "veeeq;": 8794, "vltri;": 8882, "vnsub;": 8834, "vnsup;": 8835, "vprop;": 8733, "vrtri;": 8883, "wcirc;": 373, "wedge;": 8743, "xcirc;": 9711, "xdtri;": 9661, "xhArr;": 10234, "xharr;": 10231, "xlArr;": 10232, "xlarr;": 10229, "xodot;": 10752, "xrArr;": 10233, "xrarr;": 10230, "xutri;": 9651, yacute: 253, "ycirc;": 375, AElig: 198, Acirc: 194, "Aopf;": 120120, Aring: 197, "Ascr;": 119964, "Auml;": 196, "Barv;": 10983, "Beta;": 914, "Bopf;": 120121, "Bscr;": 8492, "CHcy;": 1063, "COPY;": 169, "Cdot;": 266, "Copf;": 8450, "Cscr;": 119966, "DJcy;": 1026, "DScy;": 1029, "DZcy;": 1039, "Darr;": 8609, "Dopf;": 120123, "Dscr;": 119967, Ecirc: 202, "Edot;": 278, "Eopf;": 120124, "Escr;": 8496, "Esim;": 10867, "Euml;": 203, "Fopf;": 120125, "Fscr;": 8497, "GJcy;": 1027, "Gdot;": 288, "Gopf;": 120126, "Gscr;": 119970, "Hopf;": 8461, "Hscr;": 8459, "IEcy;": 1045, "IOcy;": 1025, Icirc: 206, "Idot;": 304, "Iopf;": 120128, "Iota;": 921, "Iscr;": 8464, "Iuml;": 207, "Jopf;": 120129, "Jscr;": 119973, "KHcy;": 1061, "KJcy;": 1036, "Kopf;": 120130, "Kscr;": 119974, "LJcy;": 1033, "Lang;": 10218, "Larr;": 8606, "Lopf;": 120131, "Lscr;": 8466, "Mopf;": 120132, "Mscr;": 8499, "NJcy;": 1034, "Nopf;": 8469, "Nscr;": 119977, Ocirc: 212, "Oopf;": 120134, "Oscr;": 119978, "Ouml;": 214, "Popf;": 8473, "Pscr;": 119979, "QUOT;": 34, "Qopf;": 8474, "Qscr;": 119980, "Rang;": 10219, "Rarr;": 8608, "Ropf;": 8477, "Rscr;": 8475, "SHcy;": 1064, "Sopf;": 120138, "Sqrt;": 8730, "Sscr;": 119982, "Star;": 8902, THORN: 222, "TScy;": 1062, "Topf;": 120139, "Tscr;": 119983, "Uarr;": 8607, Ucirc: 219, "Uopf;": 120140, "Upsi;": 978, "Uscr;": 119984, "Uuml;": 220, "Vbar;": 10987, "Vert;": 8214, "Vopf;": 120141, "Vscr;": 119985, "Wopf;": 120142, "Wscr;": 119986, "Xopf;": 120143, "Xscr;": 119987, "YAcy;": 1071, "YIcy;": 1031, "YUcy;": 1070, "Yopf;": 120144, "Yscr;": 119988, "Yuml;": 376, "ZHcy;": 1046, "Zdot;": 379, "Zeta;": 918, "Zopf;": 8484, "Zscr;": 119989, acirc: 226, acute: 180, aelig: 230, "andd;": 10844, "andv;": 10842, "ange;": 10660, "aopf;": 120146, "apid;": 8779, "apos;": 39, aring: 229, "ascr;": 119990, "auml;": 228, "bNot;": 10989, "bbrk;": 9141, "beta;": 946, "beth;": 8502, "bnot;": 8976, "bopf;": 120147, "boxH;": 9552, "boxV;": 9553, "boxh;": 9472, "boxv;": 9474, "bscr;": 119991, "bsim;": 8765, "bsol;": 92, "bull;": 8226, "bump;": 8782, "caps;": 8745, "cdot;": 267, cedil: 184, "cent;": 162, "chcy;": 1095, "cirE;": 10691, "circ;": 710, "cire;": 8791, "comp;": 8705, "cong;": 8773, "copf;": 120148, "copy;": 169, "cscr;": 119992, "csub;": 10959, "csup;": 10960, "cups;": 8746, "dArr;": 8659, "dHar;": 10597, "darr;": 8595, "dash;": 8208, "diam;": 8900, "djcy;": 1106, "dopf;": 120149, "dscr;": 119993, "dscy;": 1109, "dsol;": 10742, "dtri;": 9663, "dzcy;": 1119, "eDot;": 8785, "ecir;": 8790, ecirc: 234, "edot;": 279, "emsp;": 8195, "ensp;": 8194, "eopf;": 120150, "epar;": 8917, "epsi;": 949, "escr;": 8495, "esim;": 8770, "euml;": 235, "euro;": 8364, "excl;": 33, "flat;": 9837, "fnof;": 402, "fopf;": 120151, "fork;": 8916, "fscr;": 119995, "gdot;": 289, "geqq;": 8807, "gesl;": 8923, "gjcy;": 1107, "gnap;": 10890, "gneq;": 10888, "gopf;": 120152, "gscr;": 8458, "gsim;": 8819, "gtcc;": 10919, "gvnE;": 8809, "hArr;": 8660, "half;": 189, "harr;": 8596, "hbar;": 8463, "hopf;": 120153, "hscr;": 119997, icirc: 238, "iecy;": 1077, iexcl: 161, "imof;": 8887, "iocy;": 1105, "iopf;": 120154, "iota;": 953, "iscr;": 119998, "isin;": 8712, "iuml;": 239, "jopf;": 120155, "jscr;": 119999, "khcy;": 1093, "kjcy;": 1116, "kopf;": 120156, "kscr;": 12e4, "lArr;": 8656, "lHar;": 10594, "lang;": 10216, laquo: 171, "larr;": 8592, "late;": 10925, "lcub;": 123, "ldca;": 10550, "ldsh;": 8626, "leqq;": 8806, "lesg;": 8922, "ljcy;": 1113, "lnap;": 10889, "lneq;": 10887, "lopf;": 120157, "lozf;": 10731, "lpar;": 40, "lscr;": 120001, "lsim;": 8818, "lsqb;": 91, "ltcc;": 10918, "ltri;": 9667, "lvnE;": 8808, "macr;": 175, "male;": 9794, "malt;": 10016, micro: 181, "mlcp;": 10971, "mldr;": 8230, "mopf;": 120158, "mscr;": 120002, "nGtv;": 8811, "nLtv;": 8810, "nang;": 8736, "napE;": 10864, "nbsp;": 160, "ncap;": 10819, "ncup;": 10818, "ngeq;": 8817, "nges;": 10878, "ngtr;": 8815, "nisd;": 8954, "njcy;": 1114, "nldr;": 8229, "nleq;": 8816, "nles;": 10877, "nmid;": 8740, "nopf;": 120159, "npar;": 8742, "npre;": 10927, "nsce;": 10928, "nscr;": 120003, "nsim;": 8769, "nsub;": 8836, "nsup;": 8837, "ntgl;": 8825, "ntlg;": 8824, "nvap;": 8781, "nvge;": 8805, "nvgt;": 62, "nvle;": 8804, "nvlt;": 60, "oast;": 8859, "ocir;": 8858, ocirc: 244, "odiv;": 10808, "odot;": 8857, "ogon;": 731, "oint;": 8750, "omid;": 10678, "oopf;": 120160, "opar;": 10679, "ordf;": 170, "ordm;": 186, "oror;": 10838, "oscr;": 8500, "osol;": 8856, "ouml;": 246, "para;": 182, "part;": 8706, "perp;": 8869, "phiv;": 981, "plus;": 43, "popf;": 120161, pound: 163, "prap;": 10935, "prec;": 8826, "prnE;": 10933, "prod;": 8719, "prop;": 8733, "pscr;": 120005, "qint;": 10764, "qopf;": 120162, "qscr;": 120006, "quot;": 34, "rArr;": 8658, "rHar;": 10596, "race;": 8765, "rang;": 10217, raquo: 187, "rarr;": 8594, "rcub;": 125, "rdca;": 10551, "rdsh;": 8627, "real;": 8476, "rect;": 9645, "rhov;": 1009, "ring;": 730, "ropf;": 120163, "rpar;": 41, "rscr;": 120007, "rsqb;": 93, "rtri;": 9657, "scap;": 10936, "scnE;": 10934, "sdot;": 8901, "sect;": 167, "semi;": 59, "sext;": 10038, "shcy;": 1096, "sime;": 8771, "simg;": 10910, "siml;": 10909, "smid;": 8739, "smte;": 10924, "solb;": 10692, "sopf;": 120164, "spar;": 8741, "squf;": 9642, "sscr;": 120008, "star;": 9734, "subE;": 10949, "sube;": 8838, "succ;": 8827, "sung;": 9834, "sup1;": 185, "sup2;": 178, "sup3;": 179, "supE;": 10950, "supe;": 8839, szlig: 223, "tbrk;": 9140, "tdot;": 8411, thorn: 254, times: 215, "tint;": 8749, "toea;": 10536, "topf;": 120165, "tosa;": 10537, "trie;": 8796, "tscr;": 120009, "tscy;": 1094, "uArr;": 8657, "uHar;": 10595, "uarr;": 8593, ucirc: 251, "uopf;": 120166, "upsi;": 965, "uscr;": 120010, "utri;": 9653, "uuml;": 252, "vArr;": 8661, "vBar;": 10984, "varr;": 8597, "vert;": 124, "vopf;": 120167, "vscr;": 120011, "wopf;": 120168, "wscr;": 120012, "xcap;": 8898, "xcup;": 8899, "xmap;": 10236, "xnis;": 8955, "xopf;": 120169, "xscr;": 120013, "xvee;": 8897, "yacy;": 1103, "yicy;": 1111, "yopf;": 120170, "yscr;": 120014, "yucy;": 1102, "yuml;": 255, "zdot;": 380, "zeta;": 950, "zhcy;": 1078, "zopf;": 120171, "zscr;": 120015, "zwnj;": 8204, "AMP;": 38, "Acy;": 1040, "Afr;": 120068, "And;": 10835, Auml: 196, "Bcy;": 1041, "Bfr;": 120069, COPY: 169, "Cap;": 8914, "Cfr;": 8493, "Chi;": 935, "Cup;": 8915, "Dcy;": 1044, "Del;": 8711, "Dfr;": 120071, "Dot;": 168, "ENG;": 330, "ETH;": 208, "Ecy;": 1069, "Efr;": 120072, "Eta;": 919, Euml: 203, "Fcy;": 1060, "Ffr;": 120073, "Gcy;": 1043, "Gfr;": 120074, "Hat;": 94, "Hfr;": 8460, "Icy;": 1048, "Ifr;": 8465, "Int;": 8748, Iuml: 207, "Jcy;": 1049, "Jfr;": 120077, "Kcy;": 1050, "Kfr;": 120078, "Lcy;": 1051, "Lfr;": 120079, "Lsh;": 8624, "Map;": 10501, "Mcy;": 1052, "Mfr;": 120080, "Ncy;": 1053, "Nfr;": 120081, "Not;": 10988, "Ocy;": 1054, "Ofr;": 120082, Ouml: 214, "Pcy;": 1055, "Pfr;": 120083, "Phi;": 934, "Psi;": 936, QUOT: 34, "Qfr;": 120084, "REG;": 174, "Rcy;": 1056, "Rfr;": 8476, "Rho;": 929, "Rsh;": 8625, "Scy;": 1057, "Sfr;": 120086, "Sub;": 8912, "Sum;": 8721, "Sup;": 8913, "Tab;": 9, "Tau;": 932, "Tcy;": 1058, "Tfr;": 120087, "Ucy;": 1059, "Ufr;": 120088, Uuml: 220, "Vcy;": 1042, "Vee;": 8897, "Vfr;": 120089, "Wfr;": 120090, "Xfr;": 120091, "Ycy;": 1067, "Yfr;": 120092, "Zcy;": 1047, "Zfr;": 8488, "acE;": 8766, "acd;": 8767, "acy;": 1072, "afr;": 120094, "amp;": 38, "and;": 8743, "ang;": 8736, "apE;": 10864, "ape;": 8778, "ast;": 42, auml: 228, "bcy;": 1073, "bfr;": 120095, "bne;": 61, "bot;": 8869, "cap;": 8745, cent: 162, "cfr;": 120096, "chi;": 967, "cir;": 9675, copy: 169, "cup;": 8746, "dcy;": 1076, "deg;": 176, "dfr;": 120097, "die;": 168, "div;": 247, "dot;": 729, "ecy;": 1101, "efr;": 120098, "egs;": 10902, "ell;": 8467, "els;": 10901, "eng;": 331, "eta;": 951, "eth;": 240, euml: 235, "fcy;": 1092, "ffr;": 120099, "gEl;": 10892, "gap;": 10886, "gcy;": 1075, "gel;": 8923, "geq;": 8805, "ges;": 10878, "gfr;": 120100, "ggg;": 8921, "glE;": 10898, "gla;": 10917, "glj;": 10916, "gnE;": 8809, "gne;": 10888, "hfr;": 120101, "icy;": 1080, "iff;": 8660, "ifr;": 120102, "int;": 8747, iuml: 239, "jcy;": 1081, "jfr;": 120103, "kcy;": 1082, "kfr;": 120104, "lEg;": 10891, "lap;": 10885, "lat;": 10923, "lcy;": 1083, "leg;": 8922, "leq;": 8804, "les;": 10877, "lfr;": 120105, "lgE;": 10897, "lnE;": 8808, "lne;": 10887, "loz;": 9674, "lrm;": 8206, "lsh;": 8624, macr: 175, "map;": 8614, "mcy;": 1084, "mfr;": 120106, "mho;": 8487, "mid;": 8739, "nGg;": 8921, "nGt;": 8811, "nLl;": 8920, "nLt;": 8810, "nap;": 8777, nbsp: 160, "ncy;": 1085, "nfr;": 120107, "ngE;": 8807, "nge;": 8817, "ngt;": 8815, "nis;": 8956, "niv;": 8715, "nlE;": 8806, "nle;": 8816, "nlt;": 8814, "not;": 172, "npr;": 8832, "nsc;": 8833, "num;": 35, "ocy;": 1086, "ofr;": 120108, "ogt;": 10689, "ohm;": 937, "olt;": 10688, "ord;": 10845, ordf: 170, ordm: 186, "orv;": 10843, ouml: 246, "par;": 8741, para: 182, "pcy;": 1087, "pfr;": 120109, "phi;": 966, "piv;": 982, "prE;": 10931, "pre;": 10927, "psi;": 968, "qfr;": 120110, quot: 34, "rcy;": 1088, "reg;": 174, "rfr;": 120111, "rho;": 961, "rlm;": 8207, "rsh;": 8625, "scE;": 10932, "sce;": 10928, "scy;": 1089, sect: 167, "sfr;": 120112, "shy;": 173, "sim;": 8764, "smt;": 10922, "sol;": 47, "squ;": 9633, "sub;": 8834, "sum;": 8721, sup1: 185, sup2: 178, sup3: 179, "sup;": 8835, "tau;": 964, "tcy;": 1090, "tfr;": 120113, "top;": 8868, "ucy;": 1091, "ufr;": 120114, "uml;": 168, uuml: 252, "vcy;": 1074, "vee;": 8744, "vfr;": 120115, "wfr;": 120116, "xfr;": 120117, "ycy;": 1099, "yen;": 165, "yfr;": 120118, yuml: 255, "zcy;": 1079, "zfr;": 120119, "zwj;": 8205, AMP: 38, "DD;": 8517, ETH: 208, "GT;": 62, "Gg;": 8921, "Gt;": 8811, "Im;": 8465, "LT;": 60, "Ll;": 8920, "Lt;": 8810, "Mu;": 924, "Nu;": 925, "Or;": 10836, "Pi;": 928, "Pr;": 10939, REG: 174, "Re;": 8476, "Sc;": 10940, "Xi;": 926, "ac;": 8766, "af;": 8289, amp: 38, "ap;": 8776, "dd;": 8518, deg: 176, "ee;": 8519, "eg;": 10906, "el;": 10905, eth: 240, "gE;": 8807, "ge;": 8805, "gg;": 8811, "gl;": 8823, "gt;": 62, "ic;": 8291, "ii;": 8520, "in;": 8712, "it;": 8290, "lE;": 8806, "le;": 8804, "lg;": 8822, "ll;": 8810, "lt;": 60, "mp;": 8723, "mu;": 956, "ne;": 8800, "ni;": 8715, not: 172, "nu;": 957, "oS;": 9416, "or;": 8744, "pi;": 960, "pm;": 177, "pr;": 8826, reg: 174, "rx;": 8478, "sc;": 8827, shy: 173, uml: 168, "wp;": 8472, "wr;": 8768, "xi;": 958, yen: 165, GT: 62, LT: 60, gt: 62, lt: 60 };
function e0(e, t) {
  return t && !e.endsWith(";") ? `${e}\\b(?!=)` : e;
}
function fm(e) {
  const t = "#(?:x[a-fA-F\\d]+|\\d+)(?:;)?", a = Object.keys(Z1).map((u) => e0(u, e));
  return new RegExp(`&(${t}|${a.join("|")})`, "g");
}
fm(false);
fm(true);
const t0 = /* @__PURE__ */ new Map([["svelte:head", "SvelteHead"], ["svelte:options", "SvelteOptions"], ["svelte:window", "SvelteWindow"], ["svelte:document", "SvelteDocument"], ["svelte:body", "SvelteBody"]]);
new Map([...t0, ["svelte:element", "SvelteElement"], ["svelte:component", "SvelteComponent"], ["svelte:self", "SvelteSelf"], ["svelte:fragment", "SvelteFragment"], ["svelte:boundary", "SvelteBoundary"]]);
var ct = {}, nr = {}, No = {}, or = {}, eu;
function r0() {
  if (eu) return or;
  eu = 1, Object.defineProperty(or, "__esModule", { value: true }), or.default = void 0;
  function e() {
    var t = this, a = 0, s = { "@@iterator": function() {
      return s;
    }, next: function() {
      if (a < t.length) {
        var p = t[a];
        return a = a + 1, { done: false, value: p };
      } else return { done: true };
    } };
    return s;
  }
  return or.default = e, or;
}
var tu;
function wo() {
  if (tu) return No;
  tu = 1, Object.defineProperty(No, "__esModule", { value: true }), No.default = s;
  var e = t(r0());
  function t(u) {
    return u && u.__esModule ? u : { default: u };
  }
  function a(u) {
    "@babel/helpers - typeof";
    return a = typeof Symbol == "function" && typeof Symbol.iterator == "symbol" ? function(p) {
      return typeof p;
    } : function(p) {
      return p && typeof Symbol == "function" && p.constructor === Symbol && p !== Symbol.prototype ? "symbol" : typeof p;
    }, a(u);
  }
  function s(u, p) {
    return typeof Symbol == "function" && a(Symbol.iterator) === "symbol" && Object.defineProperty(u, Symbol.iterator, { value: e.default.bind(p) }), u;
  }
  return No;
}
var ru;
function a0() {
  if (ru) return nr;
  ru = 1, Object.defineProperty(nr, "__esModule", { value: true }), nr.default = void 0;
  var e = t(wo());
  function t(y) {
    return y && y.__esModule ? y : { default: y };
  }
  function a(y, O) {
    return w(y) || m(y, O) || u(y, O) || s();
  }
  function s() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function u(y, O) {
    if (y) {
      if (typeof y == "string") return p(y, O);
      var L = {}.toString.call(y).slice(8, -1);
      return L === "Object" && y.constructor && (L = y.constructor.name), L === "Map" || L === "Set" ? Array.from(y) : L === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(L) ? p(y, O) : void 0;
    }
  }
  function p(y, O) {
    (O == null || O > y.length) && (O = y.length);
    for (var L = 0, B = Array(O); L < O; L++) B[L] = y[L];
    return B;
  }
  function m(y, O) {
    var L = y == null ? null : typeof Symbol < "u" && y[Symbol.iterator] || y["@@iterator"];
    if (L != null) {
      var B, G, j, R, h = [], v = true, g = false;
      try {
        if (j = (L = L.call(y)).next, O === 0) {
          if (Object(L) !== L) return;
          v = false;
        } else for (; !(v = (B = j.call(L)).done) && (h.push(B.value), h.length !== O); v = true) ;
      } catch (S) {
        g = true, G = S;
      } finally {
        try {
          if (!v && L.return != null && (R = L.return(), Object(R) !== R)) return;
        } finally {
          if (g) throw G;
        }
      }
      return h;
    }
  }
  function w(y) {
    if (Array.isArray(y)) return y;
  }
  var x = [["aria-activedescendant", { type: "id" }], ["aria-atomic", { type: "boolean" }], ["aria-autocomplete", { type: "token", values: ["inline", "list", "both", "none"] }], ["aria-braillelabel", { type: "string" }], ["aria-brailleroledescription", { type: "string" }], ["aria-busy", { type: "boolean" }], ["aria-checked", { type: "tristate" }], ["aria-colcount", { type: "integer" }], ["aria-colindex", { type: "integer" }], ["aria-colspan", { type: "integer" }], ["aria-controls", { type: "idlist" }], ["aria-current", { type: "token", values: ["page", "step", "location", "date", "time", true, false] }], ["aria-describedby", { type: "idlist" }], ["aria-description", { type: "string" }], ["aria-details", { type: "id" }], ["aria-disabled", { type: "boolean" }], ["aria-dropeffect", { type: "tokenlist", values: ["copy", "execute", "link", "move", "none", "popup"] }], ["aria-errormessage", { type: "id" }], ["aria-expanded", { type: "boolean", allowundefined: true }], ["aria-flowto", { type: "idlist" }], ["aria-grabbed", { type: "boolean", allowundefined: true }], ["aria-haspopup", { type: "token", values: [false, true, "menu", "listbox", "tree", "grid", "dialog"] }], ["aria-hidden", { type: "boolean", allowundefined: true }], ["aria-invalid", { type: "token", values: ["grammar", false, "spelling", true] }], ["aria-keyshortcuts", { type: "string" }], ["aria-label", { type: "string" }], ["aria-labelledby", { type: "idlist" }], ["aria-level", { type: "integer" }], ["aria-live", { type: "token", values: ["assertive", "off", "polite"] }], ["aria-modal", { type: "boolean" }], ["aria-multiline", { type: "boolean" }], ["aria-multiselectable", { type: "boolean" }], ["aria-orientation", { type: "token", values: ["vertical", "undefined", "horizontal"] }], ["aria-owns", { type: "idlist" }], ["aria-placeholder", { type: "string" }], ["aria-posinset", { type: "integer" }], ["aria-pressed", { type: "tristate" }], ["aria-readonly", { type: "boolean" }], ["aria-relevant", { type: "tokenlist", values: ["additions", "all", "removals", "text"] }], ["aria-required", { type: "boolean" }], ["aria-roledescription", { type: "string" }], ["aria-rowcount", { type: "integer" }], ["aria-rowindex", { type: "integer" }], ["aria-rowspan", { type: "integer" }], ["aria-selected", { type: "boolean", allowundefined: true }], ["aria-setsize", { type: "integer" }], ["aria-sort", { type: "token", values: ["ascending", "descending", "none", "other"] }], ["aria-valuemax", { type: "number" }], ["aria-valuemin", { type: "number" }], ["aria-valuenow", { type: "number" }], ["aria-valuetext", { type: "string" }]], o = { entries: function() {
    return x;
  }, forEach: function(O) {
    for (var L = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, B = 0, G = x; B < G.length; B++) {
      var j = a(G[B], 2), R = j[0], h = j[1];
      O.call(L, h, R, x);
    }
  }, get: function(O) {
    var L = x.filter(function(B) {
      return B[0] === O;
    })[0];
    return L && L[1];
  }, has: function(O) {
    return !!o.get(O);
  }, keys: function() {
    return x.map(function(O) {
      var L = a(O, 1), B = L[0];
      return B;
    });
  }, values: function() {
    return x.map(function(O) {
      var L = a(O, 2), B = L[1];
      return B;
    });
  } };
  return nr.default = (0, e.default)(o, o.entries()), nr;
}
var lr = {}, au;
function i0() {
  if (au) return lr;
  au = 1, Object.defineProperty(lr, "__esModule", { value: true }), lr.default = void 0;
  var e = t(wo());
  function t(y) {
    return y && y.__esModule ? y : { default: y };
  }
  function a(y, O) {
    return w(y) || m(y, O) || u(y, O) || s();
  }
  function s() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function u(y, O) {
    if (y) {
      if (typeof y == "string") return p(y, O);
      var L = {}.toString.call(y).slice(8, -1);
      return L === "Object" && y.constructor && (L = y.constructor.name), L === "Map" || L === "Set" ? Array.from(y) : L === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(L) ? p(y, O) : void 0;
    }
  }
  function p(y, O) {
    (O == null || O > y.length) && (O = y.length);
    for (var L = 0, B = Array(O); L < O; L++) B[L] = y[L];
    return B;
  }
  function m(y, O) {
    var L = y == null ? null : typeof Symbol < "u" && y[Symbol.iterator] || y["@@iterator"];
    if (L != null) {
      var B, G, j, R, h = [], v = true, g = false;
      try {
        if (j = (L = L.call(y)).next, O === 0) {
          if (Object(L) !== L) return;
          v = false;
        } else for (; !(v = (B = j.call(L)).done) && (h.push(B.value), h.length !== O); v = true) ;
      } catch (S) {
        g = true, G = S;
      } finally {
        try {
          if (!v && L.return != null && (R = L.return(), Object(R) !== R)) return;
        } finally {
          if (g) throw G;
        }
      }
      return h;
    }
  }
  function w(y) {
    if (Array.isArray(y)) return y;
  }
  var x = [["a", { reserved: false }], ["abbr", { reserved: false }], ["acronym", { reserved: false }], ["address", { reserved: false }], ["applet", { reserved: false }], ["area", { reserved: false }], ["article", { reserved: false }], ["aside", { reserved: false }], ["audio", { reserved: false }], ["b", { reserved: false }], ["base", { reserved: true }], ["bdi", { reserved: false }], ["bdo", { reserved: false }], ["big", { reserved: false }], ["blink", { reserved: false }], ["blockquote", { reserved: false }], ["body", { reserved: false }], ["br", { reserved: false }], ["button", { reserved: false }], ["canvas", { reserved: false }], ["caption", { reserved: false }], ["center", { reserved: false }], ["cite", { reserved: false }], ["code", { reserved: false }], ["col", { reserved: true }], ["colgroup", { reserved: true }], ["content", { reserved: false }], ["data", { reserved: false }], ["datalist", { reserved: false }], ["dd", { reserved: false }], ["del", { reserved: false }], ["details", { reserved: false }], ["dfn", { reserved: false }], ["dialog", { reserved: false }], ["dir", { reserved: false }], ["div", { reserved: false }], ["dl", { reserved: false }], ["dt", { reserved: false }], ["em", { reserved: false }], ["embed", { reserved: false }], ["fieldset", { reserved: false }], ["figcaption", { reserved: false }], ["figure", { reserved: false }], ["font", { reserved: false }], ["footer", { reserved: false }], ["form", { reserved: false }], ["frame", { reserved: false }], ["frameset", { reserved: false }], ["h1", { reserved: false }], ["h2", { reserved: false }], ["h3", { reserved: false }], ["h4", { reserved: false }], ["h5", { reserved: false }], ["h6", { reserved: false }], ["head", { reserved: true }], ["header", { reserved: false }], ["hgroup", { reserved: false }], ["hr", { reserved: false }], ["html", { reserved: true }], ["i", { reserved: false }], ["iframe", { reserved: false }], ["img", { reserved: false }], ["input", { reserved: false }], ["ins", { reserved: false }], ["kbd", { reserved: false }], ["keygen", { reserved: false }], ["label", { reserved: false }], ["legend", { reserved: false }], ["li", { reserved: false }], ["link", { reserved: true }], ["main", { reserved: false }], ["map", { reserved: false }], ["mark", { reserved: false }], ["marquee", { reserved: false }], ["menu", { reserved: false }], ["menuitem", { reserved: false }], ["meta", { reserved: true }], ["meter", { reserved: false }], ["nav", { reserved: false }], ["noembed", { reserved: true }], ["noscript", { reserved: true }], ["object", { reserved: false }], ["ol", { reserved: false }], ["optgroup", { reserved: false }], ["option", { reserved: false }], ["output", { reserved: false }], ["p", { reserved: false }], ["param", { reserved: true }], ["picture", { reserved: true }], ["pre", { reserved: false }], ["progress", { reserved: false }], ["q", { reserved: false }], ["rp", { reserved: false }], ["rt", { reserved: false }], ["rtc", { reserved: false }], ["ruby", { reserved: false }], ["s", { reserved: false }], ["samp", { reserved: false }], ["script", { reserved: true }], ["section", { reserved: false }], ["select", { reserved: false }], ["small", { reserved: false }], ["source", { reserved: true }], ["spacer", { reserved: false }], ["span", { reserved: false }], ["strike", { reserved: false }], ["strong", { reserved: false }], ["style", { reserved: true }], ["sub", { reserved: false }], ["summary", { reserved: false }], ["sup", { reserved: false }], ["table", { reserved: false }], ["tbody", { reserved: false }], ["td", { reserved: false }], ["textarea", { reserved: false }], ["tfoot", { reserved: false }], ["th", { reserved: false }], ["thead", { reserved: false }], ["time", { reserved: false }], ["title", { reserved: true }], ["tr", { reserved: false }], ["track", { reserved: true }], ["tt", { reserved: false }], ["u", { reserved: false }], ["ul", { reserved: false }], ["var", { reserved: false }], ["video", { reserved: false }], ["wbr", { reserved: false }], ["xmp", { reserved: false }]], o = { entries: function() {
    return x;
  }, forEach: function(O) {
    for (var L = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, B = 0, G = x; B < G.length; B++) {
      var j = a(G[B], 2), R = j[0], h = j[1];
      O.call(L, h, R, x);
    }
  }, get: function(O) {
    var L = x.filter(function(B) {
      return B[0] === O;
    })[0];
    return L && L[1];
  }, has: function(O) {
    return !!o.get(O);
  }, keys: function() {
    return x.map(function(O) {
      var L = a(O, 1), B = L[0];
      return B;
    });
  }, values: function() {
    return x.map(function(O) {
      var L = a(O, 2), B = L[1];
      return B;
    });
  } };
  return lr.default = (0, e.default)(o, o.entries()), lr;
}
var ur = {}, cr = {}, dr = {}, iu;
function s0() {
  if (iu) return dr;
  iu = 1, Object.defineProperty(dr, "__esModule", { value: true }), dr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget"]] };
  return dr.default = e, dr;
}
var pr = {}, su;
function n0() {
  if (su) return pr;
  su = 1, Object.defineProperty(pr, "__esModule", { value: true }), pr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-activedescendant": null, "aria-disabled": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget"]] };
  return pr.default = e, pr;
}
var fr = {}, nu;
function o0() {
  if (nu) return fr;
  nu = 1, Object.defineProperty(fr, "__esModule", { value: true }), fr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null }, relatedConcepts: [{ concept: { name: "input" }, module: "XForms" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget"]] };
  return fr.default = e, fr;
}
var hr = {}, ou;
function l0() {
  if (ou) return hr;
  ou = 1, Object.defineProperty(hr, "__esModule", { value: true }), hr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return hr.default = e, hr;
}
var mr = {}, lu;
function u0() {
  if (lu) return mr;
  lu = 1, Object.defineProperty(mr, "__esModule", { value: true }), mr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-valuemax": null, "aria-valuemin": null, "aria-valuenow": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return mr.default = e, mr;
}
var vr = {}, uu;
function c0() {
  if (uu) return vr;
  uu = 1, Object.defineProperty(vr, "__esModule", { value: true }), vr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: [], prohibitedProps: [], props: { "aria-atomic": null, "aria-busy": null, "aria-controls": null, "aria-current": null, "aria-describedby": null, "aria-details": null, "aria-dropeffect": null, "aria-flowto": null, "aria-grabbed": null, "aria-hidden": null, "aria-keyshortcuts": null, "aria-label": null, "aria-labelledby": null, "aria-live": null, "aria-owns": null, "aria-relevant": null, "aria-roledescription": null }, relatedConcepts: [{ concept: { name: "role" }, module: "XHTML" }, { concept: { name: "type" }, module: "Dublin Core" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [] };
  return vr.default = e, vr;
}
var br = {}, cu;
function d0() {
  if (cu) return br;
  cu = 1, Object.defineProperty(br, "__esModule", { value: true }), br.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: [], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "frontmatter" }, module: "DTB" }, { concept: { name: "level" }, module: "DTB" }, { concept: { name: "level" }, module: "SMIL" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return br.default = e, br;
}
var yr = {}, du;
function p0() {
  if (du) return yr;
  du = 1, Object.defineProperty(yr, "__esModule", { value: true }), yr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return yr.default = e, yr;
}
var gr = {}, pu;
function f0() {
  if (pu) return gr;
  pu = 1, Object.defineProperty(gr, "__esModule", { value: true }), gr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-orientation": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "composite"], ["roletype", "structure", "section", "group"]] };
  return gr.default = e, gr;
}
var Rr = {}, fu;
function h0() {
  if (fu) return Rr;
  fu = 1, Object.defineProperty(Rr, "__esModule", { value: true }), Rr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: [], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype"]] };
  return Rr.default = e, Rr;
}
var xr = {}, hu;
function m0() {
  if (hu) return xr;
  hu = 1, Object.defineProperty(xr, "__esModule", { value: true }), xr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: [], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype"]] };
  return xr.default = e, xr;
}
var _r = {}, mu;
function v0() {
  if (mu) return _r;
  mu = 1, Object.defineProperty(_r, "__esModule", { value: true }), _r.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-modal": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype"]] };
  return _r.default = e, _r;
}
var vu;
function b0() {
  if (vu) return cr;
  vu = 1, Object.defineProperty(cr, "__esModule", { value: true }), cr.default = void 0;
  var e = L(s0()), t = L(n0()), a = L(o0()), s = L(l0()), u = L(u0()), p = L(c0()), m = L(d0()), w = L(p0()), x = L(f0()), o = L(h0()), y = L(m0()), O = L(v0());
  function L(G) {
    return G && G.__esModule ? G : { default: G };
  }
  var B = [["command", e.default], ["composite", t.default], ["input", a.default], ["landmark", s.default], ["range", u.default], ["roletype", p.default], ["section", m.default], ["sectionhead", w.default], ["select", x.default], ["structure", o.default], ["widget", y.default], ["window", O.default]];
  return cr.default = B, cr;
}
var Cr = {}, Pr = {}, bu;
function y0() {
  if (bu) return Pr;
  bu = 1, Object.defineProperty(Pr, "__esModule", { value: true }), Pr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-atomic": "true", "aria-live": "assertive" }, relatedConcepts: [{ concept: { name: "alert" }, module: "XForms" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Pr.default = e, Pr;
}
var qr = {}, yu;
function g0() {
  if (yu) return qr;
  yu = 1, Object.defineProperty(qr, "__esModule", { value: true }), qr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "alert" }, module: "XForms" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "alert"], ["roletype", "window", "dialog"]] };
  return qr.default = e, qr;
}
var Tr = {}, gu;
function R0() {
  if (gu) return Tr;
  gu = 1, Object.defineProperty(Tr, "__esModule", { value: true }), Tr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-activedescendant": null, "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "Device Independence Delivery Unit" } }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return Tr.default = e, Tr;
}
var Sr = {}, Ru;
function x0() {
  if (Ru) return Sr;
  Ru = 1, Object.defineProperty(Sr, "__esModule", { value: true }), Sr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-posinset": null, "aria-setsize": null }, relatedConcepts: [{ concept: { name: "article" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "document"]] };
  return Sr.default = e, Sr;
}
var wr = {}, xu;
function _0() {
  if (xu) return wr;
  xu = 1, Object.defineProperty(wr, "__esModule", { value: true }), wr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { constraints: ["scoped to the body element"], name: "header" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return wr.default = e, wr;
}
var Ar = {}, _u;
function C0() {
  if (_u) return Ar;
  _u = 1, Object.defineProperty(Ar, "__esModule", { value: true }), Ar.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "blockquote" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Ar.default = e, Ar;
}
var Er = {}, Cu;
function P0() {
  if (Cu) return Er;
  Cu = 1, Object.defineProperty(Er, "__esModule", { value: true }), Er.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-disabled": null, "aria-expanded": null, "aria-haspopup": null, "aria-pressed": null }, relatedConcepts: [{ concept: { attributes: [{ name: "type", value: "button" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ name: "type", value: "image" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ name: "type", value: "reset" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ name: "type", value: "submit" }], name: "input" }, module: "HTML" }, { concept: { name: "button" }, module: "HTML" }, { concept: { name: "trigger" }, module: "XForms" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command"]] };
  return Er.default = e, Er;
}
var kr = {}, Pu;
function q0() {
  if (Pu) return kr;
  Pu = 1, Object.defineProperty(kr, "__esModule", { value: true }), kr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "caption" }, module: "HTML" }], requireContextRole: ["figure", "grid", "table"], requiredContextRole: ["figure", "grid", "table"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return kr.default = e, kr;
}
var Ir = {}, qu;
function T0() {
  if (qu) return Ir;
  qu = 1, Object.defineProperty(Ir, "__esModule", { value: true }), Ir.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-colindex": null, "aria-colspan": null, "aria-rowindex": null, "aria-rowspan": null }, relatedConcepts: [{ concept: { constraints: ["ancestor table element has table role"], name: "td" }, module: "HTML" }], requireContextRole: ["row"], requiredContextRole: ["row"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Ir.default = e, Ir;
}
var Mr = {}, Tu;
function S0() {
  if (Tu) return Mr;
  Tu = 1, Object.defineProperty(Mr, "__esModule", { value: true }), Mr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-checked": null, "aria-errormessage": null, "aria-expanded": null, "aria-invalid": null, "aria-readonly": null, "aria-required": null }, relatedConcepts: [{ concept: { attributes: [{ name: "type", value: "checkbox" }], name: "input" }, module: "HTML" }, { concept: { name: "option" }, module: "ARIA" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-checked": null }, superClass: [["roletype", "widget", "input"]] };
  return Mr.default = e, Mr;
}
var Lr = {}, Su;
function w0() {
  if (Su) return Lr;
  Su = 1, Object.defineProperty(Lr, "__esModule", { value: true }), Lr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "code" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Lr.default = e, Lr;
}
var Or = {}, wu;
function A0() {
  if (wu) return Or;
  wu = 1, Object.defineProperty(Or, "__esModule", { value: true }), Or.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-sort": null }, relatedConcepts: [{ concept: { name: "th" }, module: "HTML" }, { concept: { attributes: [{ name: "scope", value: "col" }], name: "th" }, module: "HTML" }, { concept: { attributes: [{ name: "scope", value: "colgroup" }], name: "th" }, module: "HTML" }], requireContextRole: ["row"], requiredContextRole: ["row"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "cell"], ["roletype", "structure", "section", "cell", "gridcell"], ["roletype", "widget", "gridcell"], ["roletype", "structure", "sectionhead"]] };
  return Or.default = e, Or;
}
var Nr = {}, Au;
function E0() {
  if (Au) return Nr;
  Au = 1, Object.defineProperty(Nr, "__esModule", { value: true }), Nr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-activedescendant": null, "aria-autocomplete": null, "aria-errormessage": null, "aria-invalid": null, "aria-readonly": null, "aria-required": null, "aria-expanded": "false", "aria-haspopup": "listbox" }, relatedConcepts: [{ concept: { attributes: [{ constraints: ["set"], name: "list" }, { name: "type", value: "email" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "list" }, { name: "type", value: "search" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "list" }, { name: "type", value: "tel" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "list" }, { name: "type", value: "text" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "list" }, { name: "type", value: "url" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "list" }, { name: "type", value: "url" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["undefined"], name: "multiple" }, { constraints: ["undefined"], name: "size" }], constraints: ["the multiple attribute is not set and the size attribute does not have a value greater than 1"], name: "select" }, module: "HTML" }, { concept: { name: "select" }, module: "XForms" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-controls": null, "aria-expanded": "false" }, superClass: [["roletype", "widget", "input"]] };
  return Nr.default = e, Nr;
}
var Dr = {}, Eu;
function k0() {
  if (Eu) return Dr;
  Eu = 1, Object.defineProperty(Dr, "__esModule", { value: true }), Dr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { constraints: ["scoped to the body element", "scoped to the main element"], name: "aside" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "aria-label" }], constraints: ["scoped to a sectioning content element", "scoped to a sectioning root element other than body"], name: "aside" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "aria-labelledby" }], constraints: ["scoped to a sectioning content element", "scoped to a sectioning root element other than body"], name: "aside" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Dr.default = e, Dr;
}
var jr = {}, ku;
function I0() {
  if (ku) return jr;
  ku = 1, Object.defineProperty(jr, "__esModule", { value: true }), jr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { constraints: ["scoped to the body element"], name: "footer" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return jr.default = e, jr;
}
var Br = {}, Iu;
function M0() {
  if (Iu) return Br;
  Iu = 1, Object.defineProperty(Br, "__esModule", { value: true }), Br.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "dd" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Br.default = e, Br;
}
var Fr = {}, Mu;
function L0() {
  if (Mu) return Fr;
  Mu = 1, Object.defineProperty(Fr, "__esModule", { value: true }), Fr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "del" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Fr.default = e, Fr;
}
var Vr = {}, Lu;
function O0() {
  if (Lu) return Vr;
  Lu = 1, Object.defineProperty(Vr, "__esModule", { value: true }), Vr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "dialog" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "window"]] };
  return Vr.default = e, Vr;
}
var $r = {}, Ou;
function N0() {
  if (Ou) return $r;
  Ou = 1, Object.defineProperty($r, "__esModule", { value: true }), $r.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ module: "DAISY Guide" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "list"]] };
  return $r.default = e, $r;
}
var Hr = {}, Nu;
function D0() {
  if (Nu) return Hr;
  Nu = 1, Object.defineProperty(Hr, "__esModule", { value: true }), Hr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "Device Independence Delivery Unit" } }, { concept: { name: "html" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return Hr.default = e, Hr;
}
var Ur = {}, Du;
function j0() {
  if (Du) return Ur;
  Du = 1, Object.defineProperty(Ur, "__esModule", { value: true }), Ur.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "em" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Ur.default = e, Ur;
}
var Gr = {}, ju;
function B0() {
  if (ju) return Gr;
  ju = 1, Object.defineProperty(Gr, "__esModule", { value: true }), Gr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["article"]], requiredProps: {}, superClass: [["roletype", "structure", "section", "list"]] };
  return Gr.default = e, Gr;
}
var zr = {}, Bu;
function F0() {
  if (Bu) return zr;
  Bu = 1, Object.defineProperty(zr, "__esModule", { value: true }), zr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "figure" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return zr.default = e, zr;
}
var Wr = {}, Fu;
function V0() {
  if (Fu) return Wr;
  Fu = 1, Object.defineProperty(Wr, "__esModule", { value: true }), Wr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { attributes: [{ constraints: ["set"], name: "aria-label" }], name: "form" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "aria-labelledby" }], name: "form" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "name" }], name: "form" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Wr.default = e, Wr;
}
var Xr = {}, Vu;
function $0() {
  if (Vu) return Xr;
  Vu = 1, Object.defineProperty(Xr, "__esModule", { value: true }), Xr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "a" }, module: "HTML" }, { concept: { name: "area" }, module: "HTML" }, { concept: { name: "aside" }, module: "HTML" }, { concept: { name: "b" }, module: "HTML" }, { concept: { name: "bdo" }, module: "HTML" }, { concept: { name: "body" }, module: "HTML" }, { concept: { name: "data" }, module: "HTML" }, { concept: { name: "div" }, module: "HTML" }, { concept: { constraints: ["scoped to the main element", "scoped to a sectioning content element", "scoped to a sectioning root element other than body"], name: "footer" }, module: "HTML" }, { concept: { constraints: ["scoped to the main element", "scoped to a sectioning content element", "scoped to a sectioning root element other than body"], name: "header" }, module: "HTML" }, { concept: { name: "hgroup" }, module: "HTML" }, { concept: { name: "i" }, module: "HTML" }, { concept: { name: "pre" }, module: "HTML" }, { concept: { name: "q" }, module: "HTML" }, { concept: { name: "samp" }, module: "HTML" }, { concept: { name: "section" }, module: "HTML" }, { concept: { name: "small" }, module: "HTML" }, { concept: { name: "span" }, module: "HTML" }, { concept: { name: "u" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return Xr.default = e, Xr;
}
var Kr = {}, $u;
function H0() {
  if ($u) return Kr;
  $u = 1, Object.defineProperty(Kr, "__esModule", { value: true }), Kr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-multiselectable": null, "aria-readonly": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["row"], ["row", "rowgroup"]], requiredProps: {}, superClass: [["roletype", "widget", "composite"], ["roletype", "structure", "section", "table"]] };
  return Kr.default = e, Kr;
}
var Jr = {}, Hu;
function U0() {
  if (Hu) return Jr;
  Hu = 1, Object.defineProperty(Jr, "__esModule", { value: true }), Jr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null, "aria-readonly": null, "aria-required": null, "aria-selected": null }, relatedConcepts: [{ concept: { constraints: ["ancestor table element has grid role", "ancestor table element has treegrid role"], name: "td" }, module: "HTML" }], requireContextRole: ["row"], requiredContextRole: ["row"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "cell"], ["roletype", "widget"]] };
  return Jr.default = e, Jr;
}
var Qr = {}, Uu;
function G0() {
  if (Uu) return Qr;
  Uu = 1, Object.defineProperty(Qr, "__esModule", { value: true }), Qr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-activedescendant": null, "aria-disabled": null }, relatedConcepts: [{ concept: { name: "details" }, module: "HTML" }, { concept: { name: "fieldset" }, module: "HTML" }, { concept: { name: "optgroup" }, module: "HTML" }, { concept: { name: "address" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Qr.default = e, Qr;
}
var Yr = {}, Gu;
function z0() {
  if (Gu) return Yr;
  Gu = 1, Object.defineProperty(Yr, "__esModule", { value: true }), Yr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-level": "2" }, relatedConcepts: [{ concept: { name: "h1" }, module: "HTML" }, { concept: { name: "h2" }, module: "HTML" }, { concept: { name: "h3" }, module: "HTML" }, { concept: { name: "h4" }, module: "HTML" }, { concept: { name: "h5" }, module: "HTML" }, { concept: { name: "h6" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-level": "2" }, superClass: [["roletype", "structure", "sectionhead"]] };
  return Yr.default = e, Yr;
}
var Zr = {}, zu;
function W0() {
  if (zu) return Zr;
  zu = 1, Object.defineProperty(Zr, "__esModule", { value: true }), Zr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { attributes: [{ constraints: ["set"], name: "alt" }], name: "img" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["undefined"], name: "alt" }], name: "img" }, module: "HTML" }, { concept: { name: "imggroup" }, module: "DTB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Zr.default = e, Zr;
}
var ea = {}, Wu;
function X0() {
  if (Wu) return ea;
  Wu = 1, Object.defineProperty(ea, "__esModule", { value: true }), ea.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "ins" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ea.default = e, ea;
}
var ta = {}, Xu;
function K0() {
  if (Xu) return ta;
  Xu = 1, Object.defineProperty(ta, "__esModule", { value: true }), ta.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-disabled": null, "aria-expanded": null, "aria-haspopup": null }, relatedConcepts: [{ concept: { attributes: [{ constraints: ["set"], name: "href" }], name: "a" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "href" }], name: "area" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command"]] };
  return ta.default = e, ta;
}
var ra = {}, Ku;
function J0() {
  if (Ku) return ra;
  Ku = 1, Object.defineProperty(ra, "__esModule", { value: true }), ra.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "menu" }, module: "HTML" }, { concept: { name: "ol" }, module: "HTML" }, { concept: { name: "ul" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["listitem"]], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ra.default = e, ra;
}
var aa = {}, Ju;
function Q0() {
  if (Ju) return aa;
  Ju = 1, Object.defineProperty(aa, "__esModule", { value: true }), aa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-expanded": null, "aria-invalid": null, "aria-multiselectable": null, "aria-readonly": null, "aria-required": null, "aria-orientation": "vertical" }, relatedConcepts: [{ concept: { attributes: [{ constraints: [">1"], name: "size" }], constraints: ["the size attribute value is greater than 1"], name: "select" }, module: "HTML" }, { concept: { attributes: [{ name: "multiple" }], name: "select" }, module: "HTML" }, { concept: { name: "datalist" }, module: "HTML" }, { concept: { name: "list" }, module: "ARIA" }, { concept: { name: "select" }, module: "XForms" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["option", "group"], ["option"]], requiredProps: {}, superClass: [["roletype", "widget", "composite", "select"], ["roletype", "structure", "section", "group", "select"]] };
  return aa.default = e, aa;
}
var ia = {}, Qu;
function Y0() {
  if (Qu) return ia;
  Qu = 1, Object.defineProperty(ia, "__esModule", { value: true }), ia.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-level": null, "aria-posinset": null, "aria-setsize": null }, relatedConcepts: [{ concept: { constraints: ["direct descendant of ol", "direct descendant of ul", "direct descendant of menu"], name: "li" }, module: "HTML" }, { concept: { name: "item" }, module: "XForms" }], requireContextRole: ["directory", "list"], requiredContextRole: ["directory", "list"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ia.default = e, ia;
}
var sa = {}, Yu;
function Z0() {
  if (Yu) return sa;
  Yu = 1, Object.defineProperty(sa, "__esModule", { value: true }), sa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-live": "polite" }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return sa.default = e, sa;
}
var na = {}, Zu;
function eb() {
  if (Zu) return na;
  Zu = 1, Object.defineProperty(na, "__esModule", { value: true }), na.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "main" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return na.default = e, na;
}
var oa = {}, ec;
function tb() {
  if (ec) return oa;
  ec = 1, Object.defineProperty(oa, "__esModule", { value: true }), oa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: [], props: { "aria-braillelabel": null, "aria-brailleroledescription": null, "aria-description": null }, relatedConcepts: [{ concept: { name: "mark" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return oa.default = e, oa;
}
var la = {}, tc;
function rb() {
  if (tc) return la;
  tc = 1, Object.defineProperty(la, "__esModule", { value: true }), la.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return la.default = e, la;
}
var ua = {}, rc;
function ab() {
  if (rc) return ua;
  rc = 1, Object.defineProperty(ua, "__esModule", { value: true }), ua.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "math" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ua.default = e, ua;
}
var ca = {}, ac;
function ib() {
  if (ac) return ca;
  ac = 1, Object.defineProperty(ca, "__esModule", { value: true }), ca.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-orientation": "vertical" }, relatedConcepts: [{ concept: { name: "MENU" }, module: "JAPI" }, { concept: { name: "list" }, module: "ARIA" }, { concept: { name: "select" }, module: "XForms" }, { concept: { name: "sidebar" }, module: "DTB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["menuitem", "group"], ["menuitemradio", "group"], ["menuitemcheckbox", "group"], ["menuitem"], ["menuitemcheckbox"], ["menuitemradio"]], requiredProps: {}, superClass: [["roletype", "widget", "composite", "select"], ["roletype", "structure", "section", "group", "select"]] };
  return ca.default = e, ca;
}
var da = {}, ic;
function sb() {
  if (ic) return da;
  ic = 1, Object.defineProperty(da, "__esModule", { value: true }), da.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-orientation": "horizontal" }, relatedConcepts: [{ concept: { name: "toolbar" }, module: "ARIA" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["menuitem", "group"], ["menuitemradio", "group"], ["menuitemcheckbox", "group"], ["menuitem"], ["menuitemcheckbox"], ["menuitemradio"]], requiredProps: {}, superClass: [["roletype", "widget", "composite", "select", "menu"], ["roletype", "structure", "section", "group", "select", "menu"]] };
  return da.default = e, da;
}
var pa = {}, sc;
function nb() {
  if (sc) return pa;
  sc = 1, Object.defineProperty(pa, "__esModule", { value: true }), pa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-disabled": null, "aria-expanded": null, "aria-haspopup": null, "aria-posinset": null, "aria-setsize": null }, relatedConcepts: [{ concept: { name: "MENU_ITEM" }, module: "JAPI" }, { concept: { name: "listitem" }, module: "ARIA" }, { concept: { name: "option" }, module: "ARIA" }], requireContextRole: ["group", "menu", "menubar"], requiredContextRole: ["group", "menu", "menubar"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command"]] };
  return pa.default = e, pa;
}
var fa = {}, nc;
function ob() {
  if (nc) return fa;
  nc = 1, Object.defineProperty(fa, "__esModule", { value: true }), fa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "menuitem" }, module: "ARIA" }], requireContextRole: ["group", "menu", "menubar"], requiredContextRole: ["group", "menu", "menubar"], requiredOwnedElements: [], requiredProps: { "aria-checked": null }, superClass: [["roletype", "widget", "input", "checkbox"], ["roletype", "widget", "command", "menuitem"]] };
  return fa.default = e, fa;
}
var ha = {}, oc;
function lb() {
  if (oc) return ha;
  oc = 1, Object.defineProperty(ha, "__esModule", { value: true }), ha.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "menuitem" }, module: "ARIA" }], requireContextRole: ["group", "menu", "menubar"], requiredContextRole: ["group", "menu", "menubar"], requiredOwnedElements: [], requiredProps: { "aria-checked": null }, superClass: [["roletype", "widget", "input", "checkbox", "menuitemcheckbox"], ["roletype", "widget", "command", "menuitem", "menuitemcheckbox"], ["roletype", "widget", "input", "radio"]] };
  return ha.default = e, ha;
}
var ma = {}, lc;
function ub() {
  if (lc) return ma;
  lc = 1, Object.defineProperty(ma, "__esModule", { value: true }), ma.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-valuetext": null, "aria-valuemax": "100", "aria-valuemin": "0" }, relatedConcepts: [{ concept: { name: "meter" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-valuenow": null }, superClass: [["roletype", "structure", "range"]] };
  return ma.default = e, ma;
}
var va = {}, uc;
function cb() {
  if (uc) return va;
  uc = 1, Object.defineProperty(va, "__esModule", { value: true }), va.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "nav" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return va.default = e, va;
}
var ba = {}, cc;
function db() {
  if (cc) return ba;
  cc = 1, Object.defineProperty(ba, "__esModule", { value: true }), ba.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: [], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [] };
  return ba.default = e, ba;
}
var ya = {}, dc;
function pb() {
  if (dc) return ya;
  dc = 1, Object.defineProperty(ya, "__esModule", { value: true }), ya.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ya.default = e, ya;
}
var ga = {}, pc;
function fb() {
  if (pc) return ga;
  pc = 1, Object.defineProperty(ga, "__esModule", { value: true }), ga.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-checked": null, "aria-posinset": null, "aria-setsize": null, "aria-selected": "false" }, relatedConcepts: [{ concept: { name: "item" }, module: "XForms" }, { concept: { name: "listitem" }, module: "ARIA" }, { concept: { name: "option" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-selected": "false" }, superClass: [["roletype", "widget", "input"]] };
  return ga.default = e, ga;
}
var Ra = {}, fc;
function hb() {
  if (fc) return Ra;
  fc = 1, Object.defineProperty(Ra, "__esModule", { value: true }), Ra.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "p" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Ra.default = e, Ra;
}
var xa = {}, hc;
function mb() {
  if (hc) return xa;
  hc = 1, Object.defineProperty(xa, "__esModule", { value: true }), xa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { attributes: [{ name: "alt", value: "" }], name: "img" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return xa.default = e, xa;
}
var _a = {}, mc;
function vb() {
  if (mc) return _a;
  mc = 1, Object.defineProperty(_a, "__esModule", { value: true }), _a.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-valuetext": null }, relatedConcepts: [{ concept: { name: "progress" }, module: "HTML" }, { concept: { name: "status" }, module: "ARIA" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "range"], ["roletype", "widget"]] };
  return _a.default = e, _a;
}
var Ca = {}, vc;
function bb() {
  if (vc) return Ca;
  vc = 1, Object.defineProperty(Ca, "__esModule", { value: true }), Ca.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-checked": null, "aria-posinset": null, "aria-setsize": null }, relatedConcepts: [{ concept: { attributes: [{ name: "type", value: "radio" }], name: "input" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-checked": null }, superClass: [["roletype", "widget", "input"]] };
  return Ca.default = e, Ca;
}
var Pa = {}, bc;
function yb() {
  if (bc) return Pa;
  bc = 1, Object.defineProperty(Pa, "__esModule", { value: true }), Pa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null, "aria-readonly": null, "aria-required": null }, relatedConcepts: [{ concept: { name: "list" }, module: "ARIA" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["radio"]], requiredProps: {}, superClass: [["roletype", "widget", "composite", "select"], ["roletype", "structure", "section", "group", "select"]] };
  return Pa.default = e, Pa;
}
var qa = {}, yc;
function gb() {
  if (yc) return qa;
  yc = 1, Object.defineProperty(qa, "__esModule", { value: true }), qa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { attributes: [{ constraints: ["set"], name: "aria-label" }], name: "section" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "aria-labelledby" }], name: "section" }, module: "HTML" }, { concept: { name: "Device Independence Glossart perceivable unit" } }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return qa.default = e, qa;
}
var Ta = {}, gc;
function Rb() {
  if (gc) return Ta;
  gc = 1, Object.defineProperty(Ta, "__esModule", { value: true }), Ta.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-colindex": null, "aria-expanded": null, "aria-level": null, "aria-posinset": null, "aria-rowindex": null, "aria-selected": null, "aria-setsize": null }, relatedConcepts: [{ concept: { name: "tr" }, module: "HTML" }], requireContextRole: ["grid", "rowgroup", "table", "treegrid"], requiredContextRole: ["grid", "rowgroup", "table", "treegrid"], requiredOwnedElements: [["cell"], ["columnheader"], ["gridcell"], ["rowheader"]], requiredProps: {}, superClass: [["roletype", "structure", "section", "group"], ["roletype", "widget"]] };
  return Ta.default = e, Ta;
}
var Sa = {}, Rc;
function xb() {
  if (Rc) return Sa;
  Rc = 1, Object.defineProperty(Sa, "__esModule", { value: true }), Sa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "tbody" }, module: "HTML" }, { concept: { name: "tfoot" }, module: "HTML" }, { concept: { name: "thead" }, module: "HTML" }], requireContextRole: ["grid", "table", "treegrid"], requiredContextRole: ["grid", "table", "treegrid"], requiredOwnedElements: [["row"]], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return Sa.default = e, Sa;
}
var wa = {}, xc;
function _b() {
  if (xc) return wa;
  xc = 1, Object.defineProperty(wa, "__esModule", { value: true }), wa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-sort": null }, relatedConcepts: [{ concept: { attributes: [{ name: "scope", value: "row" }], name: "th" }, module: "HTML" }, { concept: { attributes: [{ name: "scope", value: "rowgroup" }], name: "th" }, module: "HTML" }], requireContextRole: ["row", "rowgroup"], requiredContextRole: ["row", "rowgroup"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "cell"], ["roletype", "structure", "section", "cell", "gridcell"], ["roletype", "widget", "gridcell"], ["roletype", "structure", "sectionhead"]] };
  return wa.default = e, wa;
}
var Aa = {}, _c;
function Cb() {
  if (_c) return Aa;
  _c = 1, Object.defineProperty(Aa, "__esModule", { value: true }), Aa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-valuetext": null, "aria-orientation": "vertical", "aria-valuemax": "100", "aria-valuemin": "0" }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-controls": null, "aria-valuenow": null }, superClass: [["roletype", "structure", "range"], ["roletype", "widget"]] };
  return Aa.default = e, Aa;
}
var Ea = {}, Cc;
function Pb() {
  if (Cc) return Ea;
  Cc = 1, Object.defineProperty(Ea, "__esModule", { value: true }), Ea.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Ea.default = e, Ea;
}
var ka = {}, Pc;
function qb() {
  if (Pc) return ka;
  Pc = 1, Object.defineProperty(ka, "__esModule", { value: true }), ka.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { attributes: [{ constraints: ["undefined"], name: "list" }, { name: "type", value: "search" }], constraints: ["the list attribute is not set"], name: "input" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "input", "textbox"]] };
  return ka.default = e, ka;
}
var Ia = {}, qc;
function Tb() {
  if (qc) return Ia;
  qc = 1, Object.defineProperty(Ia, "__esModule", { value: true }), Ia.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-orientation": "horizontal", "aria-valuemax": "100", "aria-valuemin": "0", "aria-valuenow": null, "aria-valuetext": null }, relatedConcepts: [{ concept: { name: "hr" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return Ia.default = e, Ia;
}
var Ma = {}, Tc;
function Sb() {
  if (Tc) return Ma;
  Tc = 1, Object.defineProperty(Ma, "__esModule", { value: true }), Ma.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-haspopup": null, "aria-invalid": null, "aria-readonly": null, "aria-valuetext": null, "aria-orientation": "horizontal", "aria-valuemax": "100", "aria-valuemin": "0" }, relatedConcepts: [{ concept: { attributes: [{ name: "type", value: "range" }], name: "input" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-valuenow": null }, superClass: [["roletype", "widget", "input"], ["roletype", "structure", "range"]] };
  return Ma.default = e, Ma;
}
var La = {}, Sc;
function wb() {
  if (Sc) return La;
  Sc = 1, Object.defineProperty(La, "__esModule", { value: true }), La.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null, "aria-readonly": null, "aria-required": null, "aria-valuetext": null, "aria-valuenow": "0" }, relatedConcepts: [{ concept: { attributes: [{ name: "type", value: "number" }], name: "input" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "composite"], ["roletype", "widget", "input"], ["roletype", "structure", "range"]] };
  return La.default = e, La;
}
var Oa = {}, wc;
function Ab() {
  if (wc) return Oa;
  wc = 1, Object.defineProperty(Oa, "__esModule", { value: true }), Oa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-atomic": "true", "aria-live": "polite" }, relatedConcepts: [{ concept: { name: "output" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Oa.default = e, Oa;
}
var Na = {}, Ac;
function Eb() {
  if (Ac) return Na;
  Ac = 1, Object.defineProperty(Na, "__esModule", { value: true }), Na.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "strong" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Na.default = e, Na;
}
var Da = {}, Ec;
function kb() {
  if (Ec) return Da;
  Ec = 1, Object.defineProperty(Da, "__esModule", { value: true }), Da.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "sub" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Da.default = e, Da;
}
var ja = {}, kc;
function Ib() {
  if (kc) return ja;
  kc = 1, Object.defineProperty(ja, "__esModule", { value: true }), ja.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "sup" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ja.default = e, ja;
}
var Ba = {}, Ic;
function Mb() {
  if (Ic) return Ba;
  Ic = 1, Object.defineProperty(Ba, "__esModule", { value: true }), Ba.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "button" }, module: "ARIA" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-checked": null }, superClass: [["roletype", "widget", "input", "checkbox"]] };
  return Ba.default = e, Ba;
}
var Fa = {}, Mc;
function Lb() {
  if (Mc) return Fa;
  Mc = 1, Object.defineProperty(Fa, "__esModule", { value: true }), Fa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-disabled": null, "aria-expanded": null, "aria-haspopup": null, "aria-posinset": null, "aria-setsize": null, "aria-selected": "false" }, relatedConcepts: [], requireContextRole: ["tablist"], requiredContextRole: ["tablist"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "sectionhead"], ["roletype", "widget"]] };
  return Fa.default = e, Fa;
}
var Va = {}, Lc;
function Ob() {
  if (Lc) return Va;
  Lc = 1, Object.defineProperty(Va, "__esModule", { value: true }), Va.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-colcount": null, "aria-rowcount": null }, relatedConcepts: [{ concept: { name: "table" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["row"], ["row", "rowgroup"]], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Va.default = e, Va;
}
var $a = {}, Oc;
function Nb() {
  if (Oc) return $a;
  Oc = 1, Object.defineProperty($a, "__esModule", { value: true }), $a.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-level": null, "aria-multiselectable": null, "aria-orientation": "horizontal" }, relatedConcepts: [{ module: "DAISY", concept: { name: "guide" } }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["tab"]], requiredProps: {}, superClass: [["roletype", "widget", "composite"]] };
  return $a.default = e, $a;
}
var Ha = {}, Nc;
function Db() {
  if (Nc) return Ha;
  Nc = 1, Object.defineProperty(Ha, "__esModule", { value: true }), Ha.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Ha.default = e, Ha;
}
var Ua = {}, Dc;
function jb() {
  if (Dc) return Ua;
  Dc = 1, Object.defineProperty(Ua, "__esModule", { value: true }), Ua.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "dfn" }, module: "HTML" }, { concept: { name: "dt" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Ua.default = e, Ua;
}
var Ga = {}, jc;
function Bb() {
  if (jc) return Ga;
  jc = 1, Object.defineProperty(Ga, "__esModule", { value: true }), Ga.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-activedescendant": null, "aria-autocomplete": null, "aria-errormessage": null, "aria-haspopup": null, "aria-invalid": null, "aria-multiline": null, "aria-placeholder": null, "aria-readonly": null, "aria-required": null }, relatedConcepts: [{ concept: { attributes: [{ constraints: ["undefined"], name: "type" }, { constraints: ["undefined"], name: "list" }], constraints: ["the list attribute is not set"], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["undefined"], name: "list" }, { name: "type", value: "email" }], constraints: ["the list attribute is not set"], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["undefined"], name: "list" }, { name: "type", value: "tel" }], constraints: ["the list attribute is not set"], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["undefined"], name: "list" }, { name: "type", value: "text" }], constraints: ["the list attribute is not set"], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["undefined"], name: "list" }, { name: "type", value: "url" }], constraints: ["the list attribute is not set"], name: "input" }, module: "HTML" }, { concept: { name: "input" }, module: "XForms" }, { concept: { name: "textarea" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "input"]] };
  return Ga.default = e, Ga;
}
var za = {}, Bc;
function Fb() {
  if (Bc) return za;
  Bc = 1, Object.defineProperty(za, "__esModule", { value: true }), za.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "time" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return za.default = e, za;
}
var Wa = {}, Fc;
function Vb() {
  if (Fc) return Wa;
  Fc = 1, Object.defineProperty(Wa, "__esModule", { value: true }), Wa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "status"]] };
  return Wa.default = e, Wa;
}
var Xa = {}, Vc;
function $b() {
  if (Vc) return Xa;
  Vc = 1, Object.defineProperty(Xa, "__esModule", { value: true }), Xa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-orientation": "horizontal" }, relatedConcepts: [{ concept: { name: "menubar" }, module: "ARIA" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "group"]] };
  return Xa.default = e, Xa;
}
var Ka = {}, $c;
function Hb() {
  if ($c) return Ka;
  $c = 1, Object.defineProperty(Ka, "__esModule", { value: true }), Ka.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Ka.default = e, Ka;
}
var Ja = {}, Hc;
function Ub() {
  if (Hc) return Ja;
  Hc = 1, Object.defineProperty(Ja, "__esModule", { value: true }), Ja.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null, "aria-multiselectable": null, "aria-required": null, "aria-orientation": "vertical" }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["treeitem", "group"], ["treeitem"]], requiredProps: {}, superClass: [["roletype", "widget", "composite", "select"], ["roletype", "structure", "section", "group", "select"]] };
  return Ja.default = e, Ja;
}
var Qa = {}, Uc;
function Gb() {
  if (Uc) return Qa;
  Uc = 1, Object.defineProperty(Qa, "__esModule", { value: true }), Qa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["row"], ["row", "rowgroup"]], requiredProps: {}, superClass: [["roletype", "widget", "composite", "grid"], ["roletype", "structure", "section", "table", "grid"], ["roletype", "widget", "composite", "select", "tree"], ["roletype", "structure", "section", "group", "select", "tree"]] };
  return Qa.default = e, Qa;
}
var Ya = {}, Gc;
function zb() {
  if (Gc) return Ya;
  Gc = 1, Object.defineProperty(Ya, "__esModule", { value: true }), Ya.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-expanded": null, "aria-haspopup": null }, relatedConcepts: [], requireContextRole: ["group", "tree"], requiredContextRole: ["group", "tree"], requiredOwnedElements: [], requiredProps: { "aria-selected": null }, superClass: [["roletype", "structure", "section", "listitem"], ["roletype", "widget", "input", "option"]] };
  return Ya.default = e, Ya;
}
var zc;
function Wb() {
  if (zc) return Cr;
  zc = 1, Object.defineProperty(Cr, "__esModule", { value: true }), Cr.default = void 0;
  var e = Z(y0()), t = Z(g0()), a = Z(R0()), s = Z(x0()), u = Z(_0()), p = Z(C0()), m = Z(P0()), w = Z(q0()), x = Z(T0()), o = Z(S0()), y = Z(w0()), O = Z(A0()), L = Z(E0()), B = Z(k0()), G = Z(I0()), j = Z(M0()), R = Z(L0()), h = Z(O0()), v = Z(N0()), g = Z(D0()), S = Z(j0()), C = Z(B0()), _ = Z(F0()), k = Z(V0()), N = Z($0()), W = Z(H0()), ee = Z(U0()), A = Z(G0()), F = Z(z0()), E = Z(W0()), M = Z(X0()), V = Z(K0()), c = Z(J0()), r = Z(Q0()), i = Z(Y0()), n = Z(Z0()), l = Z(eb()), f = Z(tb()), b = Z(rb()), P = Z(ab()), q = Z(ib()), T = Z(sb()), H = Z(nb()), J = Z(ob()), re = Z(lb()), Q = Z(ub()), de = Z(cb()), pe = Z(db()), fe = Z(pb()), he = Z(fb()), Ee = Z(hb()), Oe = Z(mb()), ke = Z(vb()), We = Z(bb()), Ne = Z(yb()), tt = Z(gb()), Je = Z(Rb()), it = Z(xb()), gt = Z(_b()), D = Z(Cb()), X = Z(Pb()), K = Z(qb()), Y = Z(Tb()), ce = Z(Sb()), ne = Z(wb()), ge = Z(Ab()), Ie = Z(Eb()), xe = Z(kb()), Te = Z(Ib()), Ge = Z(Mb()), je = Z(Lb()), De = Z(Ob()), ze = Z(Nb()), Ze = Z(Db()), te = Z(jb()), Se = Z(Bb()), be = Z(Fb()), Qe = Z(Vb()), Fe = Z($b()), ot = Z(Hb()), Xe = Z(Ub()), Ye = Z(Gb()), Ct = Z(zb());
  function Z(xt) {
    return xt && xt.__esModule ? xt : { default: xt };
  }
  var Rt = [["alert", e.default], ["alertdialog", t.default], ["application", a.default], ["article", s.default], ["banner", u.default], ["blockquote", p.default], ["button", m.default], ["caption", w.default], ["cell", x.default], ["checkbox", o.default], ["code", y.default], ["columnheader", O.default], ["combobox", L.default], ["complementary", B.default], ["contentinfo", G.default], ["definition", j.default], ["deletion", R.default], ["dialog", h.default], ["directory", v.default], ["document", g.default], ["emphasis", S.default], ["feed", C.default], ["figure", _.default], ["form", k.default], ["generic", N.default], ["grid", W.default], ["gridcell", ee.default], ["group", A.default], ["heading", F.default], ["img", E.default], ["insertion", M.default], ["link", V.default], ["list", c.default], ["listbox", r.default], ["listitem", i.default], ["log", n.default], ["main", l.default], ["mark", f.default], ["marquee", b.default], ["math", P.default], ["menu", q.default], ["menubar", T.default], ["menuitem", H.default], ["menuitemcheckbox", J.default], ["menuitemradio", re.default], ["meter", Q.default], ["navigation", de.default], ["none", pe.default], ["note", fe.default], ["option", he.default], ["paragraph", Ee.default], ["presentation", Oe.default], ["progressbar", ke.default], ["radio", We.default], ["radiogroup", Ne.default], ["region", tt.default], ["row", Je.default], ["rowgroup", it.default], ["rowheader", gt.default], ["scrollbar", D.default], ["search", X.default], ["searchbox", K.default], ["separator", Y.default], ["slider", ce.default], ["spinbutton", ne.default], ["status", ge.default], ["strong", Ie.default], ["subscript", xe.default], ["superscript", Te.default], ["switch", Ge.default], ["tab", je.default], ["table", De.default], ["tablist", ze.default], ["tabpanel", Ze.default], ["term", te.default], ["textbox", Se.default], ["time", be.default], ["timer", Qe.default], ["toolbar", Fe.default], ["tooltip", ot.default], ["tree", Xe.default], ["treegrid", Ye.default], ["treeitem", Ct.default]];
  return Cr.default = Rt, Cr;
}
var Za = {}, ei = {}, Wc;
function Xb() {
  if (Wc) return ei;
  Wc = 1, Object.defineProperty(ei, "__esModule", { value: true }), ei.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "abstract [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ei.default = e, ei;
}
var ti = {}, Xc;
function Kb() {
  if (Xc) return ti;
  Xc = 1, Object.defineProperty(ti, "__esModule", { value: true }), ti.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "acknowledgments [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return ti.default = e, ti;
}
var ri = {}, Kc;
function Jb() {
  if (Kc) return ri;
  Kc = 1, Object.defineProperty(ri, "__esModule", { value: true }), ri.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "afterword [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return ri.default = e, ri;
}
var ai = {}, Jc;
function Qb() {
  if (Jc) return ai;
  Jc = 1, Object.defineProperty(ai, "__esModule", { value: true }), ai.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "appendix [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return ai.default = e, ai;
}
var ii = {}, Qc;
function Yb() {
  if (Qc) return ii;
  Qc = 1, Object.defineProperty(ii, "__esModule", { value: true }), ii.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "referrer [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command", "link"]] };
  return ii.default = e, ii;
}
var si = {}, Yc;
function Zb() {
  if (Yc) return si;
  Yc = 1, Object.defineProperty(si, "__esModule", { value: true }), si.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "EPUB biblioentry [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: ["doc-bibliography"], requiredContextRole: ["doc-bibliography"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "listitem"]] };
  return si.default = e, si;
}
var ni = {}, Zc;
function ey() {
  if (Zc) return ni;
  Zc = 1, Object.defineProperty(ni, "__esModule", { value: true }), ni.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "bibliography [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["doc-biblioentry"]], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return ni.default = e, ni;
}
var oi = {}, ed;
function ty() {
  if (ed) return oi;
  ed = 1, Object.defineProperty(oi, "__esModule", { value: true }), oi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "biblioref [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command", "link"]] };
  return oi.default = e, oi;
}
var li = {}, td;
function ry() {
  if (td) return li;
  td = 1, Object.defineProperty(li, "__esModule", { value: true }), li.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "chapter [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return li.default = e, li;
}
var ui = {}, rd;
function ay() {
  if (rd) return ui;
  rd = 1, Object.defineProperty(ui, "__esModule", { value: true }), ui.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "colophon [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ui.default = e, ui;
}
var ci = {}, ad;
function iy() {
  if (ad) return ci;
  ad = 1, Object.defineProperty(ci, "__esModule", { value: true }), ci.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "conclusion [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return ci.default = e, ci;
}
var di = {}, id;
function sy() {
  if (id) return di;
  id = 1, Object.defineProperty(di, "__esModule", { value: true }), di.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "cover [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "img"]] };
  return di.default = e, di;
}
var pi = {}, sd;
function ny() {
  if (sd) return pi;
  sd = 1, Object.defineProperty(pi, "__esModule", { value: true }), pi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "credit [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return pi.default = e, pi;
}
var fi = {}, nd;
function oy() {
  if (nd) return fi;
  nd = 1, Object.defineProperty(fi, "__esModule", { value: true }), fi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "credits [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return fi.default = e, fi;
}
var hi = {}, od;
function ly() {
  if (od) return hi;
  od = 1, Object.defineProperty(hi, "__esModule", { value: true }), hi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "dedication [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return hi.default = e, hi;
}
var mi = {}, ld;
function uy() {
  if (ld) return mi;
  ld = 1, Object.defineProperty(mi, "__esModule", { value: true }), mi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "rearnote [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: ["doc-endnotes"], requiredContextRole: ["doc-endnotes"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "listitem"]] };
  return mi.default = e, mi;
}
var vi = {}, ud;
function cy() {
  if (ud) return vi;
  ud = 1, Object.defineProperty(vi, "__esModule", { value: true }), vi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "rearnotes [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["doc-endnote"]], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return vi.default = e, vi;
}
var bi = {}, cd;
function dy() {
  if (cd) return bi;
  cd = 1, Object.defineProperty(bi, "__esModule", { value: true }), bi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "epigraph [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return bi.default = e, bi;
}
var yi = {}, dd;
function py() {
  if (dd) return yi;
  dd = 1, Object.defineProperty(yi, "__esModule", { value: true }), yi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "epilogue [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return yi.default = e, yi;
}
var gi = {}, pd;
function fy() {
  if (pd) return gi;
  pd = 1, Object.defineProperty(gi, "__esModule", { value: true }), gi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "errata [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return gi.default = e, gi;
}
var Ri = {}, fd;
function hy() {
  if (fd) return Ri;
  fd = 1, Object.defineProperty(Ri, "__esModule", { value: true }), Ri.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Ri.default = e, Ri;
}
var xi = {}, hd;
function my() {
  if (hd) return xi;
  hd = 1, Object.defineProperty(xi, "__esModule", { value: true }), xi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "footnote [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return xi.default = e, xi;
}
var _i = {}, md;
function vy() {
  if (md) return _i;
  md = 1, Object.defineProperty(_i, "__esModule", { value: true }), _i.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "foreword [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return _i.default = e, _i;
}
var Ci = {}, vd;
function by() {
  if (vd) return Ci;
  vd = 1, Object.defineProperty(Ci, "__esModule", { value: true }), Ci.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "glossary [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["definition"], ["term"]], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Ci.default = e, Ci;
}
var Pi = {}, bd;
function yy() {
  if (bd) return Pi;
  bd = 1, Object.defineProperty(Pi, "__esModule", { value: true }), Pi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "glossref [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command", "link"]] };
  return Pi.default = e, Pi;
}
var qi = {}, yd;
function gy() {
  if (yd) return qi;
  yd = 1, Object.defineProperty(qi, "__esModule", { value: true }), qi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "index [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark", "navigation"]] };
  return qi.default = e, qi;
}
var Ti = {}, gd;
function Ry() {
  if (gd) return Ti;
  gd = 1, Object.defineProperty(Ti, "__esModule", { value: true }), Ti.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "introduction [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Ti.default = e, Ti;
}
var Si = {}, Rd;
function xy() {
  if (Rd) return Si;
  Rd = 1, Object.defineProperty(Si, "__esModule", { value: true }), Si.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "noteref [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command", "link"]] };
  return Si.default = e, Si;
}
var wi = {}, xd;
function _y() {
  if (xd) return wi;
  xd = 1, Object.defineProperty(wi, "__esModule", { value: true }), wi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "notice [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "note"]] };
  return wi.default = e, wi;
}
var Ai = {}, _d;
function Cy() {
  if (_d) return Ai;
  _d = 1, Object.defineProperty(Ai, "__esModule", { value: true }), Ai.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "pagebreak [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "separator"]] };
  return Ai.default = e, Ai;
}
var Ei = {}, Cd;
function Py() {
  if (Cd) return Ei;
  Cd = 1, Object.defineProperty(Ei, "__esModule", { value: true }), Ei.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: [], props: { "aria-braillelabel": null, "aria-brailleroledescription": null, "aria-description": null, "aria-disabled": null, "aria-errormessage": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Ei.default = e, Ei;
}
var ki = {}, Pd;
function qy() {
  if (Pd) return ki;
  Pd = 1, Object.defineProperty(ki, "__esModule", { value: true }), ki.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: [], props: { "aria-braillelabel": null, "aria-brailleroledescription": null, "aria-description": null, "aria-disabled": null, "aria-errormessage": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ki.default = e, ki;
}
var Ii = {}, qd;
function Ty() {
  if (qd) return Ii;
  qd = 1, Object.defineProperty(Ii, "__esModule", { value: true }), Ii.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "page-list [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark", "navigation"]] };
  return Ii.default = e, Ii;
}
var Mi = {}, Td;
function Sy() {
  if (Td) return Mi;
  Td = 1, Object.defineProperty(Mi, "__esModule", { value: true }), Mi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "part [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Mi.default = e, Mi;
}
var Li = {}, Sd;
function wy() {
  if (Sd) return Li;
  Sd = 1, Object.defineProperty(Li, "__esModule", { value: true }), Li.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "preface [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Li.default = e, Li;
}
var Oi = {}, wd;
function Ay() {
  if (wd) return Oi;
  wd = 1, Object.defineProperty(Oi, "__esModule", { value: true }), Oi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "prologue [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Oi.default = e, Oi;
}
var Ni = {}, Ad;
function Ey() {
  if (Ad) return Ni;
  Ad = 1, Object.defineProperty(Ni, "__esModule", { value: true }), Ni.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "pullquote [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["none"]] };
  return Ni.default = e, Ni;
}
var Di = {}, Ed;
function ky() {
  if (Ed) return Di;
  Ed = 1, Object.defineProperty(Di, "__esModule", { value: true }), Di.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "qna [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Di.default = e, Di;
}
var ji = {}, kd;
function Iy() {
  if (kd) return ji;
  kd = 1, Object.defineProperty(ji, "__esModule", { value: true }), ji.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "subtitle [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "sectionhead"]] };
  return ji.default = e, ji;
}
var Bi = {}, Id;
function My() {
  if (Id) return Bi;
  Id = 1, Object.defineProperty(Bi, "__esModule", { value: true }), Bi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "help [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "note"]] };
  return Bi.default = e, Bi;
}
var Fi = {}, Md;
function Ly() {
  if (Md) return Fi;
  Md = 1, Object.defineProperty(Fi, "__esModule", { value: true }), Fi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "toc [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark", "navigation"]] };
  return Fi.default = e, Fi;
}
var Ld;
function Oy() {
  if (Ld) return Za;
  Ld = 1, Object.defineProperty(Za, "__esModule", { value: true }), Za.default = void 0;
  var e = T(Xb()), t = T(Kb()), a = T(Jb()), s = T(Qb()), u = T(Yb()), p = T(Zb()), m = T(ey()), w = T(ty()), x = T(ry()), o = T(ay()), y = T(iy()), O = T(sy()), L = T(ny()), B = T(oy()), G = T(ly()), j = T(uy()), R = T(cy()), h = T(dy()), v = T(py()), g = T(fy()), S = T(hy()), C = T(my()), _ = T(vy()), k = T(by()), N = T(yy()), W = T(gy()), ee = T(Ry()), A = T(xy()), F = T(_y()), E = T(Cy()), M = T(Py()), V = T(qy()), c = T(Ty()), r = T(Sy()), i = T(wy()), n = T(Ay()), l = T(Ey()), f = T(ky()), b = T(Iy()), P = T(My()), q = T(Ly());
  function T(J) {
    return J && J.__esModule ? J : { default: J };
  }
  var H = [["doc-abstract", e.default], ["doc-acknowledgments", t.default], ["doc-afterword", a.default], ["doc-appendix", s.default], ["doc-backlink", u.default], ["doc-biblioentry", p.default], ["doc-bibliography", m.default], ["doc-biblioref", w.default], ["doc-chapter", x.default], ["doc-colophon", o.default], ["doc-conclusion", y.default], ["doc-cover", O.default], ["doc-credit", L.default], ["doc-credits", B.default], ["doc-dedication", G.default], ["doc-endnote", j.default], ["doc-endnotes", R.default], ["doc-epigraph", h.default], ["doc-epilogue", v.default], ["doc-errata", g.default], ["doc-example", S.default], ["doc-footnote", C.default], ["doc-foreword", _.default], ["doc-glossary", k.default], ["doc-glossref", N.default], ["doc-index", W.default], ["doc-introduction", ee.default], ["doc-noteref", A.default], ["doc-notice", F.default], ["doc-pagebreak", E.default], ["doc-pagefooter", M.default], ["doc-pageheader", V.default], ["doc-pagelist", c.default], ["doc-part", r.default], ["doc-preface", i.default], ["doc-prologue", n.default], ["doc-pullquote", l.default], ["doc-qna", f.default], ["doc-subtitle", b.default], ["doc-tip", P.default], ["doc-toc", q.default]];
  return Za.default = H, Za;
}
var Vi = {}, $i = {}, Od;
function Ny() {
  if (Od) return $i;
  Od = 1, Object.defineProperty($i, "__esModule", { value: true }), $i.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ module: "GRAPHICS", concept: { name: "graphics-object" } }, { module: "ARIA", concept: { name: "img" } }, { module: "ARIA", concept: { name: "article" } }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "document"]] };
  return $i.default = e, $i;
}
var Hi = {}, Nd;
function Dy() {
  if (Nd) return Hi;
  Nd = 1, Object.defineProperty(Hi, "__esModule", { value: true }), Hi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ module: "GRAPHICS", concept: { name: "graphics-document" } }, { module: "ARIA", concept: { name: "group" } }, { module: "ARIA", concept: { name: "img" } }, { module: "GRAPHICS", concept: { name: "graphics-symbol" } }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "group"]] };
  return Hi.default = e, Hi;
}
var Ui = {}, Dd;
function jy() {
  if (Dd) return Ui;
  Dd = 1, Object.defineProperty(Ui, "__esModule", { value: true }), Ui.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "img"]] };
  return Ui.default = e, Ui;
}
var jd;
function By() {
  if (jd) return Vi;
  jd = 1, Object.defineProperty(Vi, "__esModule", { value: true }), Vi.default = void 0;
  var e = s(Ny()), t = s(Dy()), a = s(jy());
  function s(p) {
    return p && p.__esModule ? p : { default: p };
  }
  var u = [["graphics-document", e.default], ["graphics-object", t.default], ["graphics-symbol", a.default]];
  return Vi.default = u, Vi;
}
var Bd;
function Il() {
  if (Bd) return ur;
  Bd = 1, Object.defineProperty(ur, "__esModule", { value: true }), ur.default = void 0;
  var e = p(b0()), t = p(Wb()), a = p(Oy()), s = p(By()), u = p(wo());
  function p(j) {
    return j && j.__esModule ? j : { default: j };
  }
  function m(j, R) {
    var h = typeof Symbol < "u" && j[Symbol.iterator] || j["@@iterator"];
    if (!h) {
      if (Array.isArray(j) || (h = o(j)) || R) {
        h && (j = h);
        var v = 0, g = function() {
        };
        return { s: g, n: function() {
          return v >= j.length ? { done: true } : { done: false, value: j[v++] };
        }, e: function(N) {
          throw N;
        }, f: g };
      }
      throw new TypeError(`Invalid attempt to iterate non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
    }
    var S, C = true, _ = false;
    return { s: function() {
      h = h.call(j);
    }, n: function() {
      var N = h.next();
      return C = N.done, N;
    }, e: function(N) {
      _ = true, S = N;
    }, f: function() {
      try {
        C || h.return == null || h.return();
      } finally {
        if (_) throw S;
      }
    } };
  }
  function w(j, R) {
    return L(j) || O(j, R) || o(j, R) || x();
  }
  function x() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function o(j, R) {
    if (j) {
      if (typeof j == "string") return y(j, R);
      var h = {}.toString.call(j).slice(8, -1);
      return h === "Object" && j.constructor && (h = j.constructor.name), h === "Map" || h === "Set" ? Array.from(j) : h === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(h) ? y(j, R) : void 0;
    }
  }
  function y(j, R) {
    (R == null || R > j.length) && (R = j.length);
    for (var h = 0, v = Array(R); h < R; h++) v[h] = j[h];
    return v;
  }
  function O(j, R) {
    var h = j == null ? null : typeof Symbol < "u" && j[Symbol.iterator] || j["@@iterator"];
    if (h != null) {
      var v, g, S, C, _ = [], k = true, N = false;
      try {
        if (S = (h = h.call(j)).next, R === 0) {
          if (Object(h) !== h) return;
          k = false;
        } else for (; !(k = (v = S.call(h)).done) && (_.push(v.value), _.length !== R); k = true) ;
      } catch (W) {
        N = true, g = W;
      } finally {
        try {
          if (!k && h.return != null && (C = h.return(), Object(C) !== C)) return;
        } finally {
          if (N) throw g;
        }
      }
      return _;
    }
  }
  function L(j) {
    if (Array.isArray(j)) return j;
  }
  var B = [].concat(e.default, t.default, a.default, s.default);
  B.forEach(function(j) {
    var R = w(j, 2), h = R[1], v = m(h.superClass), g;
    try {
      for (v.s(); !(g = v.n()).done; ) {
        var S = g.value, C = m(S), _;
        try {
          var k = function() {
            var W = _.value, ee = B.filter(function(V) {
              var c = w(V, 1), r = c[0];
              return r === W;
            })[0];
            if (ee) for (var A = ee[1], F = 0, E = Object.keys(A.props); F < E.length; F++) {
              var M = E[F];
              Object.prototype.hasOwnProperty.call(h.props, M) || (h.props[M] = A.props[M]);
            }
          };
          for (C.s(); !(_ = C.n()).done; ) k();
        } catch (N) {
          C.e(N);
        } finally {
          C.f();
        }
      }
    } catch (N) {
      v.e(N);
    } finally {
      v.f();
    }
  });
  var G = { entries: function() {
    return B;
  }, forEach: function(R) {
    var h = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, v = m(B), g;
    try {
      for (v.s(); !(g = v.n()).done; ) {
        var S = w(g.value, 2), C = S[0], _ = S[1];
        R.call(h, _, C, B);
      }
    } catch (k) {
      v.e(k);
    } finally {
      v.f();
    }
  }, get: function(R) {
    var h = B.filter(function(v) {
      return v[0] === R;
    })[0];
    return h && h[1];
  }, has: function(R) {
    return !!G.get(R);
  }, keys: function() {
    return B.map(function(R) {
      var h = w(R, 1), v = h[0];
      return v;
    });
  }, values: function() {
    return B.map(function(R) {
      var h = w(R, 2), v = h[1];
      return v;
    });
  } };
  return ur.default = (0, u.default)(G, G.entries()), ur;
}
var Gi = {}, Fd;
function Fy() {
  if (Fd) return Gi;
  Fd = 1, Object.defineProperty(Gi, "__esModule", { value: true }), Gi.default = void 0;
  var e = a(wo()), t = a(Il());
  function a(C) {
    return C && C.__esModule ? C : { default: C };
  }
  function s(C, _) {
    return x(C) || w(C, _) || p(C, _) || u();
  }
  function u() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function p(C, _) {
    if (C) {
      if (typeof C == "string") return m(C, _);
      var k = {}.toString.call(C).slice(8, -1);
      return k === "Object" && C.constructor && (k = C.constructor.name), k === "Map" || k === "Set" ? Array.from(C) : k === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(k) ? m(C, _) : void 0;
    }
  }
  function m(C, _) {
    (_ == null || _ > C.length) && (_ = C.length);
    for (var k = 0, N = Array(_); k < _; k++) N[k] = C[k];
    return N;
  }
  function w(C, _) {
    var k = C == null ? null : typeof Symbol < "u" && C[Symbol.iterator] || C["@@iterator"];
    if (k != null) {
      var N, W, ee, A, F = [], E = true, M = false;
      try {
        if (ee = (k = k.call(C)).next, _ === 0) {
          if (Object(k) !== k) return;
          E = false;
        } else for (; !(E = (N = ee.call(k)).done) && (F.push(N.value), F.length !== _); E = true) ;
      } catch (V) {
        M = true, W = V;
      } finally {
        try {
          if (!E && k.return != null && (A = k.return(), Object(A) !== A)) return;
        } finally {
          if (M) throw W;
        }
      }
      return F;
    }
  }
  function x(C) {
    if (Array.isArray(C)) return C;
  }
  for (var o = [], y = t.default.keys(), O = 0; O < y.length; O++) {
    var L = y[O], B = t.default.get(L);
    if (B) for (var G = [].concat(B.baseConcepts, B.relatedConcepts), j = function() {
      var _ = G[R];
      if (_.module === "HTML") {
        var k = _.concept;
        if (k) {
          var N = o.filter(function(F) {
            return v(F[0], k);
          })[0], W;
          N ? W = N[1] : W = [];
          for (var ee = true, A = 0; A < W.length; A++) if (W[A] === L) {
            ee = false;
            break;
          }
          ee && W.push(L), N || o.push([k, W]);
        }
      }
    }, R = 0; R < G.length; R++) j();
  }
  var h = { entries: function() {
    return o;
  }, forEach: function(_) {
    for (var k = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, N = 0, W = o; N < W.length; N++) {
      var ee = s(W[N], 2), A = ee[0], F = ee[1];
      _.call(k, F, A, o);
    }
  }, get: function(_) {
    var k = o.filter(function(N) {
      return _.name === N[0].name && S(_.attributes, N[0].attributes);
    })[0];
    return k && k[1];
  }, has: function(_) {
    return !!h.get(_);
  }, keys: function() {
    return o.map(function(_) {
      var k = s(_, 1), N = k[0];
      return N;
    });
  }, values: function() {
    return o.map(function(_) {
      var k = s(_, 2), N = k[1];
      return N;
    });
  } };
  function v(C, _) {
    return C.name === _.name && g(C.constraints, _.constraints) && S(C.attributes, _.attributes);
  }
  function g(C, _) {
    if (C === void 0 && _ !== void 0 || C !== void 0 && _ === void 0) return false;
    if (C !== void 0 && _ !== void 0) {
      if (C.length !== _.length) return false;
      for (var k = 0; k < C.length; k++) if (C[k] !== _[k]) return false;
    }
    return true;
  }
  function S(C, _) {
    if (C === void 0 && _ !== void 0 || C !== void 0 && _ === void 0) return false;
    if (C !== void 0 && _ !== void 0) {
      if (C.length !== _.length) return false;
      for (var k = 0; k < C.length; k++) {
        if (C[k].name !== _[k].name || C[k].value !== _[k].value || C[k].constraints === void 0 && _[k].constraints !== void 0 || C[k].constraints !== void 0 && _[k].constraints === void 0) return false;
        if (C[k].constraints !== void 0 && _[k].constraints !== void 0) {
          if (C[k].constraints.length !== _[k].constraints.length) return false;
          for (var N = 0; N < C[k].constraints.length; N++) if (C[k].constraints[N] !== _[k].constraints[N]) return false;
        }
      }
    }
    return true;
  }
  return Gi.default = (0, e.default)(h, h.entries()), Gi;
}
var zi = {}, Vd;
function Vy() {
  if (Vd) return zi;
  Vd = 1, Object.defineProperty(zi, "__esModule", { value: true }), zi.default = void 0;
  var e = a(wo()), t = a(Il());
  function a(S) {
    return S && S.__esModule ? S : { default: S };
  }
  function s(S, C) {
    return x(S) || w(S, C) || p(S, C) || u();
  }
  function u() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function p(S, C) {
    if (S) {
      if (typeof S == "string") return m(S, C);
      var _ = {}.toString.call(S).slice(8, -1);
      return _ === "Object" && S.constructor && (_ = S.constructor.name), _ === "Map" || _ === "Set" ? Array.from(S) : _ === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(_) ? m(S, C) : void 0;
    }
  }
  function m(S, C) {
    (C == null || C > S.length) && (C = S.length);
    for (var _ = 0, k = Array(C); _ < C; _++) k[_] = S[_];
    return k;
  }
  function w(S, C) {
    var _ = S == null ? null : typeof Symbol < "u" && S[Symbol.iterator] || S["@@iterator"];
    if (_ != null) {
      var k, N, W, ee, A = [], F = true, E = false;
      try {
        if (W = (_ = _.call(S)).next, C === 0) {
          if (Object(_) !== _) return;
          F = false;
        } else for (; !(F = (k = W.call(_)).done) && (A.push(k.value), A.length !== C); F = true) ;
      } catch (M) {
        E = true, N = M;
      } finally {
        try {
          if (!F && _.return != null && (ee = _.return(), Object(ee) !== ee)) return;
        } finally {
          if (E) throw N;
        }
      }
      return A;
    }
  }
  function x(S) {
    if (Array.isArray(S)) return S;
  }
  for (var o = [], y = t.default.keys(), O = 0; O < y.length; O++) {
    var L = y[O], B = t.default.get(L), G = [];
    if (B) {
      for (var j = [].concat(B.baseConcepts, B.relatedConcepts), R = 0; R < j.length; R++) {
        var h = j[R];
        if (h.module === "HTML") {
          var v = h.concept;
          v != null && G.push(v);
        }
      }
      G.length > 0 && o.push([L, G]);
    }
  }
  var g = { entries: function() {
    return o;
  }, forEach: function(C) {
    for (var _ = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, k = 0, N = o; k < N.length; k++) {
      var W = s(N[k], 2), ee = W[0], A = W[1];
      C.call(_, A, ee, o);
    }
  }, get: function(C) {
    var _ = o.filter(function(k) {
      return k[0] === C;
    })[0];
    return _ && _[1];
  }, has: function(C) {
    return !!g.get(C);
  }, keys: function() {
    return o.map(function(C) {
      var _ = s(C, 1), k = _[0];
      return k;
    });
  }, values: function() {
    return o.map(function(C) {
      var _ = s(C, 2), k = _[1];
      return k;
    });
  } };
  return zi.default = (0, e.default)(g, g.entries()), zi;
}
var $d;
function $y() {
  if ($d) return ct;
  $d = 1, Object.defineProperty(ct, "__esModule", { value: true }), ct.roles = ct.roleElements = ct.elementRoles = ct.dom = ct.aria = void 0;
  var e = p(a0()), t = p(i0()), a = p(Il()), s = p(Fy()), u = p(Vy());
  function p(m) {
    return m && m.__esModule ? m : { default: m };
  }
  return ct.aria = e.default, ct.dom = t.default, ct.roles = a.default, ct.elementRoles = s.default, ct.roleElements = u.default, ct;
}
var Ao = $y(), _t = {}, Wi = {}, Do = {}, Xi = {}, Hd;
function Hy() {
  if (Hd) return Xi;
  Hd = 1, Object.defineProperty(Xi, "__esModule", { value: true }), Xi.default = void 0;
  function e() {
    var a = this, s = 0, u = { "@@iterator": function() {
      return u;
    }, next: function() {
      if (s < a.length) {
        var m = a[s];
        return s = s + 1, { done: false, value: m };
      } else return { done: true };
    } };
    return u;
  }
  var t = e;
  return Xi.default = t, Xi;
}
var Ud;
function rl() {
  if (Ud) return Do;
  Ud = 1, Object.defineProperty(Do, "__esModule", { value: true }), Do.default = s;
  var e = t(Hy());
  function t(u) {
    return u && u.__esModule ? u : { default: u };
  }
  function a(u) {
    "@babel/helpers - typeof";
    return a = typeof Symbol == "function" && typeof Symbol.iterator == "symbol" ? function(p) {
      return typeof p;
    } : function(p) {
      return p && typeof Symbol == "function" && p.constructor === Symbol && p !== Symbol.prototype ? "symbol" : typeof p;
    }, a(u);
  }
  function s(u, p) {
    return typeof Symbol == "function" && a(Symbol.iterator) === "symbol" && Object.defineProperty(u, Symbol.iterator, { value: e.default.bind(p) }), u;
  }
  return Do;
}
var Ki = {}, Ji = {}, Gd;
function Uy() {
  if (Gd) return Ji;
  Gd = 1, Object.defineProperty(Ji, "__esModule", { value: true }), Ji.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "abbr" } }], type: "structure" }, t = e;
  return Ji.default = t, Ji;
}
var Qi = {}, zd;
function Gy() {
  if (zd) return Qi;
  zd = 1, Object.defineProperty(Qi, "__esModule", { value: true }), Qi.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "alertdialog" } }], type: "window" }, t = e;
  return Qi.default = t, Qi;
}
var Yi = {}, Wd;
function zy() {
  if (Wd) return Yi;
  Wd = 1, Object.defineProperty(Yi, "__esModule", { value: true }), Yi.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "alert" } }], type: "structure" }, t = e;
  return Yi.default = t, Yi;
}
var Zi = {}, Xd;
function Wy() {
  if (Xd) return Zi;
  Xd = 1, Object.defineProperty(Zi, "__esModule", { value: true }), Zi.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Zi.default = t, Zi;
}
var es = {}, Kd;
function Xy() {
  if (Kd) return es;
  Kd = 1, Object.defineProperty(es, "__esModule", { value: true }), es.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "application" } }], type: "window" }, t = e;
  return es.default = t, es;
}
var ts = {}, Jd;
function Ky() {
  if (Jd) return ts;
  Jd = 1, Object.defineProperty(ts, "__esModule", { value: true }), ts.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "article" } }, { module: "HTML", concept: { name: "article" } }], type: "structure" }, t = e;
  return ts.default = t, ts;
}
var rs = {}, Qd;
function Jy() {
  if (Qd) return rs;
  Qd = 1, Object.defineProperty(rs, "__esModule", { value: true }), rs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "audio" } }], type: "widget" }, t = e;
  return rs.default = t, rs;
}
var as = {}, Yd;
function Qy() {
  if (Yd) return as;
  Yd = 1, Object.defineProperty(as, "__esModule", { value: true }), as.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "banner" } }], type: "structure" }, t = e;
  return as.default = t, as;
}
var is = {}, Zd;
function Yy() {
  if (Zd) return is;
  Zd = 1, Object.defineProperty(is, "__esModule", { value: true }), is.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "blockquote" } }], type: "structure" }, t = e;
  return is.default = t, is;
}
var ss = {}, ep;
function Zy() {
  if (ep) return ss;
  ep = 1, Object.defineProperty(ss, "__esModule", { value: true }), ss.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { attributes: [{ name: "aria-busy", value: "true" }] } }], type: "widget" }, t = e;
  return ss.default = t, ss;
}
var ns = {}, tp;
function eg() {
  if (tp) return ns;
  tp = 1, Object.defineProperty(ns, "__esModule", { value: true }), ns.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "button" } }, { module: "HTML", concept: { name: "button" } }], type: "widget" }, t = e;
  return ns.default = t, ns;
}
var os = {}, rp;
function tg() {
  if (rp) return os;
  rp = 1, Object.defineProperty(os, "__esModule", { value: true }), os.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "canvas" } }], type: "widget" }, t = e;
  return os.default = t, os;
}
var ls = {}, ap;
function rg() {
  if (ap) return ls;
  ap = 1, Object.defineProperty(ls, "__esModule", { value: true }), ls.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "caption" } }], type: "structure" }, t = e;
  return ls.default = t, ls;
}
var us = {}, ip;
function ag() {
  if (ip) return us;
  ip = 1, Object.defineProperty(us, "__esModule", { value: true }), us.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "cell" } }, { module: "ARIA", concept: { name: "gridcell" } }, { module: "HTML", concept: { name: "td" } }], type: "widget" }, t = e;
  return us.default = t, us;
}
var cs = {}, sp;
function ig() {
  if (sp) return cs;
  sp = 1, Object.defineProperty(cs, "__esModule", { value: true }), cs.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "checkbox" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "checkbox" }] } }], type: "widget" }, t = e;
  return cs.default = t, cs;
}
var ds = {}, np;
function sg() {
  if (np) return ds;
  np = 1, Object.defineProperty(ds, "__esModule", { value: true }), ds.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "color" }] } }], type: "widget" }, t = e;
  return ds.default = t, ds;
}
var ps = {}, op;
function ng() {
  if (op) return ps;
  op = 1, Object.defineProperty(ps, "__esModule", { value: true }), ps.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "columnheader" } }, { module: "HTML", concept: { name: "th" } }], type: "widget" }, t = e;
  return ps.default = t, ps;
}
var fs = {}, lp;
function og() {
  if (lp) return fs;
  lp = 1, Object.defineProperty(fs, "__esModule", { value: true }), fs.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return fs.default = t, fs;
}
var hs = {}, up;
function lg() {
  if (up) return hs;
  up = 1, Object.defineProperty(hs, "__esModule", { value: true }), hs.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "combobox" } }, { module: "HTML", concept: { name: "select" } }], type: "widget" }, t = e;
  return hs.default = t, hs;
}
var ms = {}, cp;
function ug() {
  if (cp) return ms;
  cp = 1, Object.defineProperty(ms, "__esModule", { value: true }), ms.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "complementary" } }], type: "structure" }, t = e;
  return ms.default = t, ms;
}
var vs = {}, dp;
function cg() {
  if (dp) return vs;
  dp = 1, Object.defineProperty(vs, "__esModule", { value: true }), vs.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "structureinfo" } }], type: "structure" }, t = e;
  return vs.default = t, vs;
}
var bs = {}, pp;
function dg() {
  if (pp) return bs;
  pp = 1, Object.defineProperty(bs, "__esModule", { value: true }), bs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "date" }] } }], type: "widget" }, t = e;
  return bs.default = t, bs;
}
var ys = {}, fp;
function pg() {
  if (fp) return ys;
  fp = 1, Object.defineProperty(ys, "__esModule", { value: true }), ys.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "datetime" }] } }], type: "widget" }, t = e;
  return ys.default = t, ys;
}
var gs = {}, hp;
function fg() {
  if (hp) return gs;
  hp = 1, Object.defineProperty(gs, "__esModule", { value: true }), gs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "dfn" } }], type: "structure" }, t = e;
  return gs.default = t, gs;
}
var Rs = {}, mp;
function hg() {
  if (mp) return Rs;
  mp = 1, Object.defineProperty(Rs, "__esModule", { value: true }), Rs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "dd" } }], type: "structure" }, t = e;
  return Rs.default = t, Rs;
}
var xs = {}, vp;
function mg() {
  if (vp) return xs;
  vp = 1, Object.defineProperty(xs, "__esModule", { value: true }), xs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "dl" } }], type: "structure" }, t = e;
  return xs.default = t, xs;
}
var _s = {}, bp;
function vg() {
  if (bp) return _s;
  bp = 1, Object.defineProperty(_s, "__esModule", { value: true }), _s.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "dt" } }], type: "structure" }, t = e;
  return _s.default = t, _s;
}
var Cs = {}, yp;
function bg() {
  if (yp) return Cs;
  yp = 1, Object.defineProperty(Cs, "__esModule", { value: true }), Cs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "details" } }], type: "structure" }, t = e;
  return Cs.default = t, Cs;
}
var Ps = {}, gp;
function yg() {
  if (gp) return Ps;
  gp = 1, Object.defineProperty(Ps, "__esModule", { value: true }), Ps.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "dialog" } }, { module: "HTML", concept: { name: "dialog" } }], type: "window" }, t = e;
  return Ps.default = t, Ps;
}
var qs = {}, Rp;
function gg() {
  if (Rp) return qs;
  Rp = 1, Object.defineProperty(qs, "__esModule", { value: true }), qs.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "directory" } }, { module: "HTML", concept: { name: "dir" } }], type: "structure" }, t = e;
  return qs.default = t, qs;
}
var Ts = {}, xp;
function Rg() {
  if (xp) return Ts;
  xp = 1, Object.defineProperty(Ts, "__esModule", { value: true }), Ts.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { constraints: ["scoped to a details element"], name: "summary" } }], type: "widget" }, t = e;
  return Ts.default = t, Ts;
}
var Ss = {}, _p;
function xg() {
  if (_p) return Ss;
  _p = 1, Object.defineProperty(Ss, "__esModule", { value: true }), Ss.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "div" } }], type: "generic" }, t = e;
  return Ss.default = t, Ss;
}
var ws = {}, Cp;
function _g() {
  if (Cp) return ws;
  Cp = 1, Object.defineProperty(ws, "__esModule", { value: true }), ws.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "document" } }], type: "structure" }, t = e;
  return ws.default = t, ws;
}
var As = {}, Pp;
function Cg() {
  if (Pp) return As;
  Pp = 1, Object.defineProperty(As, "__esModule", { value: true }), As.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "embed" } }], type: "widget" }, t = e;
  return As.default = t, As;
}
var Es = {}, qp;
function Pg() {
  if (qp) return Es;
  qp = 1, Object.defineProperty(Es, "__esModule", { value: true }), Es.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "feed" } }], type: "structure" }, t = e;
  return Es.default = t, Es;
}
var ks = {}, Tp;
function qg() {
  if (Tp) return ks;
  Tp = 1, Object.defineProperty(ks, "__esModule", { value: true }), ks.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "figcaption" } }], type: "structure" }, t = e;
  return ks.default = t, ks;
}
var Is = {}, Sp;
function Tg() {
  if (Sp) return Is;
  Sp = 1, Object.defineProperty(Is, "__esModule", { value: true }), Is.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "figure" } }, { module: "HTML", concept: { name: "figure" } }], type: "structure" }, t = e;
  return Is.default = t, Is;
}
var Ms = {}, wp;
function Sg() {
  if (wp) return Ms;
  wp = 1, Object.defineProperty(Ms, "__esModule", { value: true }), Ms.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "footer" } }], type: "structure" }, t = e;
  return Ms.default = t, Ms;
}
var Ls = {}, Ap;
function wg() {
  if (Ap) return Ls;
  Ap = 1, Object.defineProperty(Ls, "__esModule", { value: true }), Ls.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "form" } }, { module: "HTML", concept: { name: "form" } }], type: "structure" }, t = e;
  return Ls.default = t, Ls;
}
var Os = {}, Ep;
function Ag() {
  if (Ep) return Os;
  Ep = 1, Object.defineProperty(Os, "__esModule", { value: true }), Os.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "grid" } }], type: "widget" }, t = e;
  return Os.default = t, Os;
}
var Ns = {}, kp;
function Eg() {
  if (kp) return Ns;
  kp = 1, Object.defineProperty(Ns, "__esModule", { value: true }), Ns.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "group" } }], type: "structure" }, t = e;
  return Ns.default = t, Ns;
}
var Ds = {}, Ip;
function kg() {
  if (Ip) return Ds;
  Ip = 1, Object.defineProperty(Ds, "__esModule", { value: true }), Ds.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "heading" } }, { module: "HTML", concept: { name: "h1" } }, { module: "HTML", concept: { name: "h2" } }, { module: "HTML", concept: { name: "h3" } }, { module: "HTML", concept: { name: "h4" } }, { module: "HTML", concept: { name: "h5" } }, { module: "HTML", concept: { name: "h6" } }], type: "structure" }, t = e;
  return Ds.default = t, Ds;
}
var js = {}, Mp;
function Ig() {
  if (Mp) return js;
  Mp = 1, Object.defineProperty(js, "__esModule", { value: true }), js.default = void 0;
  var e = { relatedConcepts: [], type: "window" }, t = e;
  return js.default = t, js;
}
var Bs = {}, Lp;
function Mg() {
  if (Lp) return Bs;
  Lp = 1, Object.defineProperty(Bs, "__esModule", { value: true }), Bs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "iframe" } }], type: "window" }, t = e;
  return Bs.default = t, Bs;
}
var Fs = {}, Op;
function Lg() {
  if (Op) return Fs;
  Op = 1, Object.defineProperty(Fs, "__esModule", { value: true }), Fs.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Fs.default = t, Fs;
}
var Vs = {}, Np;
function Og() {
  if (Np) return Vs;
  Np = 1, Object.defineProperty(Vs, "__esModule", { value: true }), Vs.default = void 0;
  var e = { relatedConcepts: [], type: "widget" }, t = e;
  return Vs.default = t, Vs;
}
var $s = {}, Dp;
function Ng() {
  if (Dp) return $s;
  Dp = 1, Object.defineProperty($s, "__esModule", { value: true }), $s.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "img", attributes: [{ name: "usemap" }] } }], type: "structure" }, t = e;
  return $s.default = t, $s;
}
var Hs = {}, jp;
function Dg() {
  if (jp) return Hs;
  jp = 1, Object.defineProperty(Hs, "__esModule", { value: true }), Hs.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "img" } }, { module: "HTML", concept: { name: "img" } }], type: "structure" }, t = e;
  return Hs.default = t, Hs;
}
var Us = {}, Bp;
function jg() {
  if (Bp) return Us;
  Bp = 1, Object.defineProperty(Us, "__esModule", { value: true }), Us.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "input" } }], type: "widget" }, t = e;
  return Us.default = t, Us;
}
var Gs = {}, Fp;
function Bg() {
  if (Fp) return Gs;
  Fp = 1, Object.defineProperty(Gs, "__esModule", { value: true }), Gs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "time" }] } }], type: "widget" }, t = e;
  return Gs.default = t, Gs;
}
var zs = {}, Vp;
function Fg() {
  if (Vp) return zs;
  Vp = 1, Object.defineProperty(zs, "__esModule", { value: true }), zs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "label" } }], type: "structure" }, t = e;
  return zs.default = t, zs;
}
var Ws = {}, $p;
function Vg() {
  if ($p) return Ws;
  $p = 1, Object.defineProperty(Ws, "__esModule", { value: true }), Ws.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "legend" } }], type: "structure" }, t = e;
  return Ws.default = t, Ws;
}
var Xs = {}, Hp;
function $g() {
  if (Hp) return Xs;
  Hp = 1, Object.defineProperty(Xs, "__esModule", { value: true }), Xs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "br" } }], type: "structure" }, t = e;
  return Xs.default = t, Xs;
}
var Ks = {}, Up;
function Hg() {
  if (Up) return Ks;
  Up = 1, Object.defineProperty(Ks, "__esModule", { value: true }), Ks.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "link" } }, { module: "HTML", concept: { name: "a", attributes: [{ name: "href" }] } }], type: "widget" }, t = e;
  return Ks.default = t, Ks;
}
var Js = {}, Gp;
function Ug() {
  if (Gp) return Js;
  Gp = 1, Object.defineProperty(Js, "__esModule", { value: true }), Js.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "option" } }, { module: "HTML", concept: { name: "option" } }], type: "widget" }, t = e;
  return Js.default = t, Js;
}
var Qs = {}, zp;
function Gg() {
  if (zp) return Qs;
  zp = 1, Object.defineProperty(Qs, "__esModule", { value: true }), Qs.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "listbox" } }, { module: "HTML", concept: { name: "datalist" } }, { module: "HTML", concept: { name: "select" } }], type: "widget" }, t = e;
  return Qs.default = t, Qs;
}
var Ys = {}, Wp;
function zg() {
  if (Wp) return Ys;
  Wp = 1, Object.defineProperty(Ys, "__esModule", { value: true }), Ys.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "listitem" } }, { module: "HTML", concept: { name: "li" } }], type: "structure" }, t = e;
  return Ys.default = t, Ys;
}
var Zs = {}, Xp;
function Wg() {
  if (Xp) return Zs;
  Xp = 1, Object.defineProperty(Zs, "__esModule", { value: true }), Zs.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Zs.default = t, Zs;
}
var en = {}, Kp;
function Xg() {
  if (Kp) return en;
  Kp = 1, Object.defineProperty(en, "__esModule", { value: true }), en.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "list" } }, { module: "HTML", concept: { name: "ul" } }, { module: "HTML", concept: { name: "ol" } }], type: "structure" }, t = e;
  return en.default = t, en;
}
var tn = {}, Jp;
function Kg() {
  if (Jp) return tn;
  Jp = 1, Object.defineProperty(tn, "__esModule", { value: true }), tn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "log" } }], type: "structure" }, t = e;
  return tn.default = t, tn;
}
var rn = {}, Qp;
function Jg() {
  if (Qp) return rn;
  Qp = 1, Object.defineProperty(rn, "__esModule", { value: true }), rn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "main" } }, { module: "HTML", concept: { name: "main" } }], type: "structure" }, t = e;
  return rn.default = t, rn;
}
var an = {}, Yp;
function Qg() {
  if (Yp) return an;
  Yp = 1, Object.defineProperty(an, "__esModule", { value: true }), an.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "mark" } }], type: "structure" }, t = e;
  return an.default = t, an;
}
var sn = {}, Zp;
function Yg() {
  if (Zp) return sn;
  Zp = 1, Object.defineProperty(sn, "__esModule", { value: true }), sn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "marquee" } }, { module: "HTML", concept: { name: "marquee" } }], type: "structure" }, t = e;
  return sn.default = t, sn;
}
var nn = {}, ef;
function Zg() {
  if (ef) return nn;
  ef = 1, Object.defineProperty(nn, "__esModule", { value: true }), nn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "math" } }], type: "structure" }, t = e;
  return nn.default = t, nn;
}
var on = {}, tf;
function eR() {
  if (tf) return on;
  tf = 1, Object.defineProperty(on, "__esModule", { value: true }), on.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "menubar" } }], type: "structure" }, t = e;
  return on.default = t, on;
}
var ln = {}, rf;
function tR() {
  if (rf) return ln;
  rf = 1, Object.defineProperty(ln, "__esModule", { value: true }), ln.default = void 0;
  var e = { relatedConcepts: [], type: "widget" }, t = e;
  return ln.default = t, ln;
}
var un = {}, af;
function rR() {
  if (af) return un;
  af = 1, Object.defineProperty(un, "__esModule", { value: true }), un.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "menuitem" } }, { module: "HTML", concept: { name: "menuitem" } }], type: "widget" }, t = e;
  return un.default = t, un;
}
var cn = {}, sf;
function aR() {
  if (sf) return cn;
  sf = 1, Object.defineProperty(cn, "__esModule", { value: true }), cn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "menuitemcheckbox" } }], type: "widget" }, t = e;
  return cn.default = t, cn;
}
var dn = {}, nf;
function iR() {
  if (nf) return dn;
  nf = 1, Object.defineProperty(dn, "__esModule", { value: true }), dn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "menuitemradio" } }], type: "widget" }, t = e;
  return dn.default = t, dn;
}
var pn = {}, of;
function sR() {
  if (of) return pn;
  of = 1, Object.defineProperty(pn, "__esModule", { value: true }), pn.default = void 0;
  var e = { relatedConcepts: [], type: "widget" }, t = e;
  return pn.default = t, pn;
}
var fn = {}, lf;
function nR() {
  if (lf) return fn;
  lf = 1, Object.defineProperty(fn, "__esModule", { value: true }), fn.default = void 0;
  var e = { relatedConcepts: [], type: "widget" }, t = e;
  return fn.default = t, fn;
}
var hn = {}, uf;
function oR() {
  if (uf) return hn;
  uf = 1, Object.defineProperty(hn, "__esModule", { value: true }), hn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "menu" } }, { module: "HTML", concept: { name: "menu" } }], type: "structure" }, t = e;
  return hn.default = t, hn;
}
var mn = {}, cf;
function lR() {
  if (cf) return mn;
  cf = 1, Object.defineProperty(mn, "__esModule", { value: true }), mn.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "meter" } }], type: "structure" }, t = e;
  return mn.default = t, mn;
}
var vn = {}, df;
function uR() {
  if (df) return vn;
  df = 1, Object.defineProperty(vn, "__esModule", { value: true }), vn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "navigation" } }, { module: "HTML", concept: { name: "nav" } }], type: "structure" }, t = e;
  return vn.default = t, vn;
}
var bn = {}, pf;
function cR() {
  if (pf) return bn;
  pf = 1, Object.defineProperty(bn, "__esModule", { value: true }), bn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "none" } }], type: "structure" }, t = e;
  return bn.default = t, bn;
}
var yn = {}, ff;
function dR() {
  if (ff) return yn;
  ff = 1, Object.defineProperty(yn, "__esModule", { value: true }), yn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "note" } }], type: "structure" }, t = e;
  return yn.default = t, yn;
}
var gn = {}, hf;
function pR() {
  if (hf) return gn;
  hf = 1, Object.defineProperty(gn, "__esModule", { value: true }), gn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return gn.default = t, gn;
}
var Rn = {}, mf;
function fR() {
  if (mf) return Rn;
  mf = 1, Object.defineProperty(Rn, "__esModule", { value: true }), Rn.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "p" } }], type: "structure" }, t = e;
  return Rn.default = t, Rn;
}
var xn = {}, vf;
function hR() {
  if (vf) return xn;
  vf = 1, Object.defineProperty(xn, "__esModule", { value: true }), xn.default = void 0;
  var e = { relatedConcepts: [], type: "widget" }, t = e;
  return xn.default = t, xn;
}
var _n = {}, bf;
function mR() {
  if (bf) return _n;
  bf = 1, Object.defineProperty(_n, "__esModule", { value: true }), _n.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "pre" } }], type: "structure" }, t = e;
  return _n.default = t, _n;
}
var Cn = {}, yf;
function vR() {
  if (yf) return Cn;
  yf = 1, Object.defineProperty(Cn, "__esModule", { value: true }), Cn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "presentation" } }], type: "structure" }, t = e;
  return Cn.default = t, Cn;
}
var Pn = {}, gf;
function bR() {
  if (gf) return Pn;
  gf = 1, Object.defineProperty(Pn, "__esModule", { value: true }), Pn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "progressbar" } }, { module: "HTML", concept: { name: "progress" } }], type: "structure" }, t = e;
  return Pn.default = t, Pn;
}
var qn = {}, Rf;
function yR() {
  if (Rf) return qn;
  Rf = 1, Object.defineProperty(qn, "__esModule", { value: true }), qn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "radio" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "radio" }] } }], type: "widget" }, t = e;
  return qn.default = t, qn;
}
var Tn = {}, xf;
function gR() {
  if (xf) return Tn;
  xf = 1, Object.defineProperty(Tn, "__esModule", { value: true }), Tn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "radiogroup" } }], type: "structure" }, t = e;
  return Tn.default = t, Tn;
}
var Sn = {}, _f;
function RR() {
  if (_f) return Sn;
  _f = 1, Object.defineProperty(Sn, "__esModule", { value: true }), Sn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "region" } }], type: "structure" }, t = e;
  return Sn.default = t, Sn;
}
var wn = {}, Cf;
function xR() {
  if (Cf) return wn;
  Cf = 1, Object.defineProperty(wn, "__esModule", { value: true }), wn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return wn.default = t, wn;
}
var An = {}, Pf;
function _R() {
  if (Pf) return An;
  Pf = 1, Object.defineProperty(An, "__esModule", { value: true }), An.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "rowheader" } }, { module: "HTML", concept: { name: "th", attributes: [{ name: "scope", value: "row" }] } }], type: "widget" }, t = e;
  return An.default = t, An;
}
var En = {}, qf;
function CR() {
  if (qf) return En;
  qf = 1, Object.defineProperty(En, "__esModule", { value: true }), En.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "row" } }, { module: "HTML", concept: { name: "tr" } }], type: "structure" }, t = e;
  return En.default = t, En;
}
var kn = {}, Tf;
function PR() {
  if (Tf) return kn;
  Tf = 1, Object.defineProperty(kn, "__esModule", { value: true }), kn.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "ruby" } }], type: "structure" }, t = e;
  return kn.default = t, kn;
}
var In = {}, Sf;
function qR() {
  if (Sf) return In;
  Sf = 1, Object.defineProperty(In, "__esModule", { value: true }), In.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return In.default = t, In;
}
var Mn = {}, wf;
function TR() {
  if (wf) return Mn;
  wf = 1, Object.defineProperty(Mn, "__esModule", { value: true }), Mn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Mn.default = t, Mn;
}
var Ln = {}, Af;
function SR() {
  if (Af) return Ln;
  Af = 1, Object.defineProperty(Ln, "__esModule", { value: true }), Ln.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "scrollbar" } }], type: "widget" }, t = e;
  return Ln.default = t, Ln;
}
var On = {}, Ef;
function wR() {
  if (Ef) return On;
  Ef = 1, Object.defineProperty(On, "__esModule", { value: true }), On.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return On.default = t, On;
}
var Nn = {}, kf;
function AR() {
  if (kf) return Nn;
  kf = 1, Object.defineProperty(Nn, "__esModule", { value: true }), Nn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "search" } }], type: "structure" }, t = e;
  return Nn.default = t, Nn;
}
var Dn = {}, If;
function ER() {
  if (If) return Dn;
  If = 1, Object.defineProperty(Dn, "__esModule", { value: true }), Dn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "searchbox" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "search" }] } }], type: "widget" }, t = e;
  return Dn.default = t, Dn;
}
var jn = {}, Mf;
function kR() {
  if (Mf) return jn;
  Mf = 1, Object.defineProperty(jn, "__esModule", { value: true }), jn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "slider" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "range" }] } }], type: "widget" }, t = e;
  return jn.default = t, jn;
}
var Bn = {}, Lf;
function IR() {
  if (Lf) return Bn;
  Lf = 1, Object.defineProperty(Bn, "__esModule", { value: true }), Bn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Bn.default = t, Bn;
}
var Fn = {}, Of;
function MR() {
  if (Of) return Fn;
  Of = 1, Object.defineProperty(Fn, "__esModule", { value: true }), Fn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "spinbutton" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "number" }] } }], type: "widget" }, t = e;
  return Fn.default = t, Fn;
}
var Vn = {}, Nf;
function LR() {
  if (Nf) return Vn;
  Nf = 1, Object.defineProperty(Vn, "__esModule", { value: true }), Vn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Vn.default = t, Vn;
}
var $n = {}, Df;
function OR() {
  if (Df) return $n;
  Df = 1, Object.defineProperty($n, "__esModule", { value: true }), $n.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "separator" } }], type: "widget" }, t = e;
  return $n.default = t, $n;
}
var Hn = {}, jf;
function NR() {
  if (jf) return Hn;
  jf = 1, Object.defineProperty(Hn, "__esModule", { value: true }), Hn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Hn.default = t, Hn;
}
var Un = {}, Bf;
function DR() {
  if (Bf) return Un;
  Bf = 1, Object.defineProperty(Un, "__esModule", { value: true }), Un.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "status" } }], type: "structure" }, t = e;
  return Un.default = t, Un;
}
var Gn = {}, Ff;
function jR() {
  if (Ff) return Gn;
  Ff = 1, Object.defineProperty(Gn, "__esModule", { value: true }), Gn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Gn.default = t, Gn;
}
var zn = {}, Vf;
function BR() {
  if (Vf) return zn;
  Vf = 1, Object.defineProperty(zn, "__esModule", { value: true }), zn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "switch" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "checkbox" }] } }], type: "widget" }, t = e;
  return zn.default = t, zn;
}
var Wn = {}, $f;
function FR() {
  if ($f) return Wn;
  $f = 1, Object.defineProperty(Wn, "__esModule", { value: true }), Wn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "tablist" } }], type: "structure" }, t = e;
  return Wn.default = t, Wn;
}
var Xn = {}, Hf;
function VR() {
  if (Hf) return Xn;
  Hf = 1, Object.defineProperty(Xn, "__esModule", { value: true }), Xn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "tab" } }], type: "widget" }, t = e;
  return Xn.default = t, Xn;
}
var Kn = {}, Uf;
function $R() {
  if (Uf) return Kn;
  Uf = 1, Object.defineProperty(Kn, "__esModule", { value: true }), Kn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Kn.default = t, Kn;
}
var Jn = {}, Gf;
function HR() {
  if (Gf) return Jn;
  Gf = 1, Object.defineProperty(Jn, "__esModule", { value: true }), Jn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "table" } }, { module: "HTML", concept: { name: "table" } }], type: "structure" }, t = e;
  return Jn.default = t, Jn;
}
var Qn = {}, zf;
function UR() {
  if (zf) return Qn;
  zf = 1, Object.defineProperty(Qn, "__esModule", { value: true }), Qn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "tablist" } }], type: "structure" }, t = e;
  return Qn.default = t, Qn;
}
var Yn = {}, Wf;
function GR() {
  if (Wf) return Yn;
  Wf = 1, Object.defineProperty(Yn, "__esModule", { value: true }), Yn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "tabpanel" } }], type: "structure" }, t = e;
  return Yn.default = t, Yn;
}
var Zn = {}, Xf;
function zR() {
  if (Xf) return Zn;
  Xf = 1, Object.defineProperty(Zn, "__esModule", { value: true }), Zn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "term" } }], type: "structure" }, t = e;
  return Zn.default = t, Zn;
}
var eo = {}, Kf;
function WR() {
  if (Kf) return eo;
  Kf = 1, Object.defineProperty(eo, "__esModule", { value: true }), eo.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { attributes: [{ name: "aria-multiline", value: "true" }], name: "textbox" } }, { module: "HTML", concept: { name: "textarea" } }], type: "widget" }, t = e;
  return eo.default = t, eo;
}
var to = {}, Jf;
function XR() {
  if (Jf) return to;
  Jf = 1, Object.defineProperty(to, "__esModule", { value: true }), to.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "textbox" } }, { module: "HTML", concept: { name: "input" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "text" }] } }], type: "widget" }, t = e;
  return to.default = t, to;
}
var ro = {}, Qf;
function KR() {
  if (Qf) return ro;
  Qf = 1, Object.defineProperty(ro, "__esModule", { value: true }), ro.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "time" } }], type: "structure" }, t = e;
  return ro.default = t, ro;
}
var ao = {}, Yf;
function JR() {
  if (Yf) return ao;
  Yf = 1, Object.defineProperty(ao, "__esModule", { value: true }), ao.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "timer" } }], type: "structure" }, t = e;
  return ao.default = t, ao;
}
var io = {}, Zf;
function QR() {
  if (Zf) return io;
  Zf = 1, Object.defineProperty(io, "__esModule", { value: true }), io.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { attributes: [{ name: "aria-pressed" }] } }], type: "widget" }, t = e;
  return io.default = t, io;
}
var so = {}, eh;
function YR() {
  if (eh) return so;
  eh = 1, Object.defineProperty(so, "__esModule", { value: true }), so.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "toolbar" } }], type: "structure" }, t = e;
  return so.default = t, so;
}
var no = {}, th;
function ZR() {
  if (th) return no;
  th = 1, Object.defineProperty(no, "__esModule", { value: true }), no.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "tree" } }], type: "widget" }, t = e;
  return no.default = t, no;
}
var oo = {}, rh;
function ex() {
  if (rh) return oo;
  rh = 1, Object.defineProperty(oo, "__esModule", { value: true }), oo.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "treegrid" } }], type: "widget" }, t = e;
  return oo.default = t, oo;
}
var lo = {}, ah;
function tx() {
  if (ah) return lo;
  ah = 1, Object.defineProperty(lo, "__esModule", { value: true }), lo.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "treeitem" } }], type: "widget" }, t = e;
  return lo.default = t, lo;
}
var uo = {}, ih;
function rx() {
  if (ih) return uo;
  ih = 1, Object.defineProperty(uo, "__esModule", { value: true }), uo.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "tooltip" } }], type: "structure" }, t = e;
  return uo.default = t, uo;
}
var co = {}, sh;
function ax() {
  if (sh) return co;
  sh = 1, Object.defineProperty(co, "__esModule", { value: true }), co.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "video" } }], type: "widget" }, t = e;
  return co.default = t, co;
}
var po = {}, nh;
function ix() {
  if (nh) return po;
  nh = 1, Object.defineProperty(po, "__esModule", { value: true }), po.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return po.default = t, po;
}
var fo = {}, oh;
function sx() {
  if (oh) return fo;
  oh = 1, Object.defineProperty(fo, "__esModule", { value: true }), fo.default = void 0;
  var e = { relatedConcepts: [], type: "window" }, t = e;
  return fo.default = t, fo;
}
var lh;
function al() {
  if (lh) return Ki;
  lh = 1, Object.defineProperty(Ki, "__esModule", { value: true }), Ki.default = void 0;
  var e = U(rl()), t = U(Uy()), a = U(Gy()), s = U(zy()), u = U(Wy()), p = U(Xy()), m = U(Ky()), w = U(Jy()), x = U(Qy()), o = U(Yy()), y = U(Zy()), O = U(eg()), L = U(tg()), B = U(rg()), G = U(ag()), j = U(ig()), R = U(sg()), h = U(ng()), v = U(og()), g = U(lg()), S = U(ug()), C = U(cg()), _ = U(dg()), k = U(pg()), N = U(fg()), W = U(hg()), ee = U(mg()), A = U(vg()), F = U(bg()), E = U(yg()), M = U(gg()), V = U(Rg()), c = U(xg()), r = U(_g()), i = U(Cg()), n = U(Pg()), l = U(qg()), f = U(Tg()), b = U(Sg()), P = U(wg()), q = U(Ag()), T = U(Eg()), H = U(kg()), J = U(Ig()), re = U(Mg()), Q = U(Lg()), de = U(Og()), pe = U(Ng()), fe = U(Dg()), he = U(jg()), Ee = U(Bg()), Oe = U(Fg()), ke = U(Vg()), We = U($g()), Ne = U(Hg()), tt = U(Ug()), Je = U(Gg()), it = U(zg()), gt = U(Wg()), D = U(Xg()), X = U(Kg()), K = U(Jg()), Y = U(Qg()), ce = U(Yg()), ne = U(Zg()), ge = U(eR()), Ie = U(tR()), xe = U(rR()), Te = U(aR()), Ge = U(iR()), je = U(sR()), De = U(nR()), ze = U(oR()), Ze = U(lR()), te = U(uR()), Se = U(cR()), be = U(dR()), Qe = U(pR()), Fe = U(fR()), ot = U(hR()), Xe = U(mR()), Ye = U(vR()), Ct = U(bR()), Z = U(yR()), Rt = U(gR()), xt = U(RR()), Eo = U(xR()), ko = U(_R()), Io = U(CR()), gm = U(PR()), Rm = U(qR()), xm = U(TR()), _m = U(SR()), Cm = U(wR()), Pm = U(AR()), qm = U(ER()), Tm = U(kR()), Sm = U(IR()), wm = U(MR()), Am = U(LR()), Em = U(OR()), km = U(NR()), Im = U(DR()), Mm = U(jR()), Lm = U(BR()), Om = U(FR()), Nm = U(VR()), Dm = U($R()), jm = U(HR()), Bm = U(UR()), Fm = U(GR()), Vm = U(zR()), $m = U(WR()), Hm = U(XR()), Um = U(KR()), Gm = U(JR()), zm = U(QR()), Wm = U(YR()), Xm = U(ZR()), Km = U(ex()), Jm = U(tx()), Qm = U(rx()), Ym = U(ax()), Zm = U(ix()), ev = U(sx());
  function U(we) {
    return we && we.__esModule ? we : { default: we };
  }
  function il(we, $e) {
    return iv(we) || av(we, $e) || rv(we, $e) || tv();
  }
  function tv() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function rv(we, $e) {
    if (we) {
      if (typeof we == "string") return Ol(we, $e);
      var Be = Object.prototype.toString.call(we).slice(8, -1);
      if (Be === "Object" && we.constructor && (Be = we.constructor.name), Be === "Map" || Be === "Set") return Array.from(we);
      if (Be === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(Be)) return Ol(we, $e);
    }
  }
  function Ol(we, $e) {
    ($e == null || $e > we.length) && ($e = we.length);
    for (var Be = 0, et = new Array($e); Be < $e; Be++) et[Be] = we[Be];
    return et;
  }
  function av(we, $e) {
    var Be = we == null ? null : typeof Symbol < "u" && we[Symbol.iterator] || we["@@iterator"];
    if (Be != null) {
      var et = [], Gt = true, ar = false, Mo, Lo;
      try {
        for (Be = Be.call(we); !(Gt = (Mo = Be.next()).done) && (et.push(Mo.value), !($e && et.length === $e)); Gt = true) ;
      } catch (nv) {
        ar = true, Lo = nv;
      } finally {
        try {
          !Gt && Be.return != null && Be.return();
        } finally {
          if (ar) throw Lo;
        }
      }
      return et;
    }
  }
  function iv(we) {
    if (Array.isArray(we)) return we;
  }
  var Ut = [["AbbrRole", t.default], ["AlertDialogRole", a.default], ["AlertRole", s.default], ["AnnotationRole", u.default], ["ApplicationRole", p.default], ["ArticleRole", m.default], ["AudioRole", w.default], ["BannerRole", x.default], ["BlockquoteRole", o.default], ["BusyIndicatorRole", y.default], ["ButtonRole", O.default], ["CanvasRole", L.default], ["CaptionRole", B.default], ["CellRole", G.default], ["CheckBoxRole", j.default], ["ColorWellRole", R.default], ["ColumnHeaderRole", h.default], ["ColumnRole", v.default], ["ComboBoxRole", g.default], ["ComplementaryRole", S.default], ["ContentInfoRole", C.default], ["DateRole", _.default], ["DateTimeRole", k.default], ["DefinitionRole", N.default], ["DescriptionListDetailRole", W.default], ["DescriptionListRole", ee.default], ["DescriptionListTermRole", A.default], ["DetailsRole", F.default], ["DialogRole", E.default], ["DirectoryRole", M.default], ["DisclosureTriangleRole", V.default], ["DivRole", c.default], ["DocumentRole", r.default], ["EmbeddedObjectRole", i.default], ["FeedRole", n.default], ["FigcaptionRole", l.default], ["FigureRole", f.default], ["FooterRole", b.default], ["FormRole", P.default], ["GridRole", q.default], ["GroupRole", T.default], ["HeadingRole", H.default], ["IframePresentationalRole", J.default], ["IframeRole", re.default], ["IgnoredRole", Q.default], ["ImageMapLinkRole", de.default], ["ImageMapRole", pe.default], ["ImageRole", fe.default], ["InlineTextBoxRole", he.default], ["InputTimeRole", Ee.default], ["LabelRole", Oe.default], ["LegendRole", ke.default], ["LineBreakRole", We.default], ["LinkRole", Ne.default], ["ListBoxOptionRole", tt.default], ["ListBoxRole", Je.default], ["ListItemRole", it.default], ["ListMarkerRole", gt.default], ["ListRole", D.default], ["LogRole", X.default], ["MainRole", K.default], ["MarkRole", Y.default], ["MarqueeRole", ce.default], ["MathRole", ne.default], ["MenuBarRole", ge.default], ["MenuButtonRole", Ie.default], ["MenuItemRole", xe.default], ["MenuItemCheckBoxRole", Te.default], ["MenuItemRadioRole", Ge.default], ["MenuListOptionRole", je.default], ["MenuListPopupRole", De.default], ["MenuRole", ze.default], ["MeterRole", Ze.default], ["NavigationRole", te.default], ["NoneRole", Se.default], ["NoteRole", be.default], ["OutlineRole", Qe.default], ["ParagraphRole", Fe.default], ["PopUpButtonRole", ot.default], ["PreRole", Xe.default], ["PresentationalRole", Ye.default], ["ProgressIndicatorRole", Ct.default], ["RadioButtonRole", Z.default], ["RadioGroupRole", Rt.default], ["RegionRole", xt.default], ["RootWebAreaRole", Eo.default], ["RowHeaderRole", ko.default], ["RowRole", Io.default], ["RubyRole", gm.default], ["RulerRole", Rm.default], ["ScrollAreaRole", xm.default], ["ScrollBarRole", _m.default], ["SeamlessWebAreaRole", Cm.default], ["SearchRole", Pm.default], ["SearchBoxRole", qm.default], ["SliderRole", Tm.default], ["SliderThumbRole", Sm.default], ["SpinButtonRole", wm.default], ["SpinButtonPartRole", Am.default], ["SplitterRole", Em.default], ["StaticTextRole", km.default], ["StatusRole", Im.default], ["SVGRootRole", Mm.default], ["SwitchRole", Lm.default], ["TabGroupRole", Om.default], ["TabRole", Nm.default], ["TableHeaderContainerRole", Dm.default], ["TableRole", jm.default], ["TabListRole", Bm.default], ["TabPanelRole", Fm.default], ["TermRole", Vm.default], ["TextAreaRole", $m.default], ["TextFieldRole", Hm.default], ["TimeRole", Um.default], ["TimerRole", Gm.default], ["ToggleButtonRole", zm.default], ["ToolbarRole", Wm.default], ["TreeRole", Xm.default], ["TreeGridRole", Km.default], ["TreeItemRole", Jm.default], ["UserInterfaceTooltipRole", Qm.default], ["VideoRole", Ym.default], ["WebAreaRole", Zm.default], ["WindowRole", ev.default]], sl = { entries: function() {
    return Ut;
  }, forEach: function($e) {
    for (var Be = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, et = 0, Gt = Ut; et < Gt.length; et++) {
      var ar = il(Gt[et], 2), Mo = ar[0], Lo = ar[1];
      $e.call(Be, Lo, Mo, Ut);
    }
  }, get: function($e) {
    var Be = Ut.find(function(et) {
      return et[0] === $e;
    });
    return Be && Be[1];
  }, has: function($e) {
    return !!sl.get($e);
  }, keys: function() {
    return Ut.map(function($e) {
      var Be = il($e, 1), et = Be[0];
      return et;
    });
  }, values: function() {
    return Ut.map(function($e) {
      var Be = il($e, 2), et = Be[1];
      return et;
    });
  } }, sv = (0, e.default)(sl, sl.entries());
  return Ki.default = sv, Ki;
}
var uh;
function nx() {
  if (uh) return Wi;
  uh = 1, Object.defineProperty(Wi, "__esModule", { value: true }), Wi.default = void 0;
  var e = a(rl()), t = a(al());
  function a(R) {
    return R && R.__esModule ? R : { default: R };
  }
  function s(R, h) {
    return m(R) || p(R, h) || x(R, h) || u();
  }
  function u() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function p(R, h) {
    var v = R == null ? null : typeof Symbol < "u" && R[Symbol.iterator] || R["@@iterator"];
    if (v != null) {
      var g = [], S = true, C = false, _, k;
      try {
        for (v = v.call(R); !(S = (_ = v.next()).done) && (g.push(_.value), !(h && g.length === h)); S = true) ;
      } catch (N) {
        C = true, k = N;
      } finally {
        try {
          !S && v.return != null && v.return();
        } finally {
          if (C) throw k;
        }
      }
      return g;
    }
  }
  function m(R) {
    if (Array.isArray(R)) return R;
  }
  function w(R, h) {
    var v = typeof Symbol < "u" && R[Symbol.iterator] || R["@@iterator"];
    if (!v) {
      if (Array.isArray(R) || (v = x(R)) || h) {
        v && (R = v);
        var g = 0, S = function() {
        };
        return { s: S, n: function() {
          return g >= R.length ? { done: true } : { done: false, value: R[g++] };
        }, e: function(W) {
          throw W;
        }, f: S };
      }
      throw new TypeError(`Invalid attempt to iterate non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
    }
    var C = true, _ = false, k;
    return { s: function() {
      v = v.call(R);
    }, n: function() {
      var W = v.next();
      return C = W.done, W;
    }, e: function(W) {
      _ = true, k = W;
    }, f: function() {
      try {
        !C && v.return != null && v.return();
      } finally {
        if (_) throw k;
      }
    } };
  }
  function x(R, h) {
    if (R) {
      if (typeof R == "string") return o(R, h);
      var v = Object.prototype.toString.call(R).slice(8, -1);
      if (v === "Object" && R.constructor && (v = R.constructor.name), v === "Map" || v === "Set") return Array.from(R);
      if (v === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(v)) return o(R, h);
    }
  }
  function o(R, h) {
    (h == null || h > R.length) && (h = R.length);
    for (var v = 0, g = new Array(h); v < h; v++) g[v] = R[v];
    return g;
  }
  var y = [], O = w(t.default.entries()), L;
  try {
    var B = function() {
      var h = s(L.value, 2), v = h[0], g = h[1], S = g.relatedConcepts;
      Array.isArray(S) && S.forEach(function(C) {
        if (C.module === "HTML") {
          var _ = C.concept;
          if (_) {
            var k = y.findIndex(function(N) {
              var W = s(N, 1), ee = W[0];
              return ee === v;
            });
            k === -1 && (y.push([v, []]), k = y.length - 1), y[k][1].push(_);
          }
        }
      });
    };
    for (O.s(); !(L = O.n()).done; ) B();
  } catch (R) {
    O.e(R);
  } finally {
    O.f();
  }
  var G = { entries: function() {
    return y;
  }, forEach: function(h) {
    for (var v = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, g = 0, S = y; g < S.length; g++) {
      var C = s(S[g], 2), _ = C[0], k = C[1];
      h.call(v, k, _, y);
    }
  }, get: function(h) {
    var v = y.find(function(g) {
      return g[0] === h;
    });
    return v && v[1];
  }, has: function(h) {
    return !!G.get(h);
  }, keys: function() {
    return y.map(function(h) {
      var v = s(h, 1), g = v[0];
      return g;
    });
  }, values: function() {
    return y.map(function(h) {
      var v = s(h, 2), g = v[1];
      return g;
    });
  } }, j = (0, e.default)(G, G.entries());
  return Wi.default = j, Wi;
}
var ho = {}, ch;
function ox() {
  if (ch) return ho;
  ch = 1, Object.defineProperty(ho, "__esModule", { value: true }), ho.default = void 0;
  var e = a(rl()), t = a(al());
  function a(R) {
    return R && R.__esModule ? R : { default: R };
  }
  function s(R, h) {
    return m(R) || p(R, h) || x(R, h) || u();
  }
  function u() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function p(R, h) {
    var v = R == null ? null : typeof Symbol < "u" && R[Symbol.iterator] || R["@@iterator"];
    if (v != null) {
      var g = [], S = true, C = false, _, k;
      try {
        for (v = v.call(R); !(S = (_ = v.next()).done) && (g.push(_.value), !(h && g.length === h)); S = true) ;
      } catch (N) {
        C = true, k = N;
      } finally {
        try {
          !S && v.return != null && v.return();
        } finally {
          if (C) throw k;
        }
      }
      return g;
    }
  }
  function m(R) {
    if (Array.isArray(R)) return R;
  }
  function w(R, h) {
    var v = typeof Symbol < "u" && R[Symbol.iterator] || R["@@iterator"];
    if (!v) {
      if (Array.isArray(R) || (v = x(R)) || h) {
        v && (R = v);
        var g = 0, S = function() {
        };
        return { s: S, n: function() {
          return g >= R.length ? { done: true } : { done: false, value: R[g++] };
        }, e: function(W) {
          throw W;
        }, f: S };
      }
      throw new TypeError(`Invalid attempt to iterate non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
    }
    var C = true, _ = false, k;
    return { s: function() {
      v = v.call(R);
    }, n: function() {
      var W = v.next();
      return C = W.done, W;
    }, e: function(W) {
      _ = true, k = W;
    }, f: function() {
      try {
        !C && v.return != null && v.return();
      } finally {
        if (_) throw k;
      }
    } };
  }
  function x(R, h) {
    if (R) {
      if (typeof R == "string") return o(R, h);
      var v = Object.prototype.toString.call(R).slice(8, -1);
      if (v === "Object" && R.constructor && (v = R.constructor.name), v === "Map" || v === "Set") return Array.from(R);
      if (v === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(v)) return o(R, h);
    }
  }
  function o(R, h) {
    (h == null || h > R.length) && (h = R.length);
    for (var v = 0, g = new Array(h); v < h; v++) g[v] = R[v];
    return g;
  }
  var y = [], O = w(t.default.entries()), L;
  try {
    var B = function() {
      var h = s(L.value, 2), v = h[0], g = h[1], S = g.relatedConcepts;
      Array.isArray(S) && S.forEach(function(C) {
        if (C.module === "ARIA") {
          var _ = C.concept;
          if (_) {
            var k = y.findIndex(function(N) {
              var W = s(N, 1), ee = W[0];
              return ee === v;
            });
            k === -1 && (y.push([v, []]), k = y.length - 1), y[k][1].push(_);
          }
        }
      });
    };
    for (O.s(); !(L = O.n()).done; ) B();
  } catch (R) {
    O.e(R);
  } finally {
    O.f();
  }
  var G = { entries: function() {
    return y;
  }, forEach: function(h) {
    for (var v = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, g = 0, S = y; g < S.length; g++) {
      var C = s(S[g], 2), _ = C[0], k = C[1];
      h.call(v, k, _, y);
    }
  }, get: function(h) {
    var v = y.find(function(g) {
      return g[0] === h;
    });
    return v && v[1];
  }, has: function(h) {
    return !!G.get(h);
  }, keys: function() {
    return y.map(function(h) {
      var v = s(h, 1), g = v[0];
      return g;
    });
  }, values: function() {
    return y.map(function(h) {
      var v = s(h, 2), g = v[1];
      return g;
    });
  } }, j = (0, e.default)(G, G.entries());
  return ho.default = j, ho;
}
var mo = {}, dh;
function lx() {
  if (dh) return mo;
  dh = 1, Object.defineProperty(mo, "__esModule", { value: true }), mo.default = void 0;
  var e = a(al()), t = a(rl());
  function a(h) {
    return h && h.__esModule ? h : { default: h };
  }
  function s(h, v) {
    return m(h) || p(h, v) || x(h, v) || u();
  }
  function u() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function p(h, v) {
    var g = h == null ? null : typeof Symbol < "u" && h[Symbol.iterator] || h["@@iterator"];
    if (g != null) {
      var S = [], C = true, _ = false, k, N;
      try {
        for (g = g.call(h); !(C = (k = g.next()).done) && (S.push(k.value), !(v && S.length === v)); C = true) ;
      } catch (W) {
        _ = true, N = W;
      } finally {
        try {
          !C && g.return != null && g.return();
        } finally {
          if (_) throw N;
        }
      }
      return S;
    }
  }
  function m(h) {
    if (Array.isArray(h)) return h;
  }
  function w(h, v) {
    var g = typeof Symbol < "u" && h[Symbol.iterator] || h["@@iterator"];
    if (!g) {
      if (Array.isArray(h) || (g = x(h)) || v) {
        g && (h = g);
        var S = 0, C = function() {
        };
        return { s: C, n: function() {
          return S >= h.length ? { done: true } : { done: false, value: h[S++] };
        }, e: function(ee) {
          throw ee;
        }, f: C };
      }
      throw new TypeError(`Invalid attempt to iterate non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
    }
    var _ = true, k = false, N;
    return { s: function() {
      g = g.call(h);
    }, n: function() {
      var ee = g.next();
      return _ = ee.done, ee;
    }, e: function(ee) {
      k = true, N = ee;
    }, f: function() {
      try {
        !_ && g.return != null && g.return();
      } finally {
        if (k) throw N;
      }
    } };
  }
  function x(h, v) {
    if (h) {
      if (typeof h == "string") return o(h, v);
      var g = Object.prototype.toString.call(h).slice(8, -1);
      if (g === "Object" && h.constructor && (g = h.constructor.name), g === "Map" || g === "Set") return Array.from(h);
      if (g === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(g)) return o(h, v);
    }
  }
  function o(h, v) {
    (v == null || v > h.length) && (v = h.length);
    for (var g = 0, S = new Array(v); g < v; g++) S[g] = h[g];
    return S;
  }
  var y = [], O = w(e.default.entries()), L;
  try {
    var B = function() {
      var v = s(L.value, 2), g = v[0], S = v[1], C = S.relatedConcepts;
      Array.isArray(C) && C.forEach(function(_) {
        if (_.module === "HTML") {
          var k = _.concept;
          if (k != null) {
            for (var N = JSON.stringify(k), W, ee = 0; ee < y.length; ee++) {
              var A = y[ee][0];
              if (JSON.stringify(A) === N) {
                W = y[ee][1];
                break;
              }
            }
            Array.isArray(W) || (W = []);
            var F = W.findIndex(function(E) {
              return E === g;
            });
            F === -1 && W.push(g), ee < y.length ? y.splice(ee, 1, [k, W]) : y.push([k, W]);
          }
        }
      });
    };
    for (O.s(); !(L = O.n()).done; ) B();
  } catch (h) {
    O.e(h);
  } finally {
    O.f();
  }
  function G(h, v) {
    if (h === void 0 && v !== void 0 || h !== void 0 && v === void 0) return false;
    if (h !== void 0 && v !== void 0) {
      if (h.length != v.length) return false;
      for (var g = 0; g < h.length; g++) if (v[g].name !== h[g].name || v[g].value !== h[g].value) return false;
    }
    return true;
  }
  var j = { entries: function() {
    return y;
  }, forEach: function(v) {
    for (var g = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, S = 0, C = y; S < C.length; S++) {
      var _ = s(C[S], 2), k = _[0], N = _[1];
      v.call(g, N, k, y);
    }
  }, get: function(v) {
    var g = y.find(function(S) {
      return v.name === S[0].name && G(v.attributes, S[0].attributes);
    });
    return g && g[1];
  }, has: function(v) {
    return !!j.get(v);
  }, keys: function() {
    return y.map(function(v) {
      var g = s(v, 1), S = g[0];
      return S;
    });
  }, values: function() {
    return y.map(function(v) {
      var g = s(v, 2), S = g[1];
      return S;
    });
  } }, R = (0, t.default)(j, j.entries());
  return mo.default = R, mo;
}
var ph;
function ux() {
  if (ph) return _t;
  ph = 1, Object.defineProperty(_t, "__esModule", { value: true }), _t.elementAXObjects = _t.AXObjects = _t.AXObjectRoles = _t.AXObjectElements = void 0;
  var e = u(nx()), t = u(ox()), a = u(al()), s = u(lx());
  function u(o) {
    return o && o.__esModule ? o : { default: o };
  }
  var p = e.default;
  _t.AXObjectElements = p;
  var m = t.default;
  _t.AXObjectRoles = m;
  var w = a.default;
  _t.AXObjects = w;
  var x = s.default;
  return _t.elementAXObjects = x, _t;
}
var Qt = ux();
const hm = Ao.roles.keys(), cx = hm.filter((e) => {
  var _a3;
  return (_a3 = Ao.roles.get(e)) == null ? void 0 : _a3.abstract;
}), mm = hm.filter((e) => !cx.includes(e)), vm = mm.filter((e) => {
  const t = Ao.roles.get(e);
  return !["toolbar", "tabpanel", "generic", "cell"].includes(e) && !(t == null ? void 0 : t.superClass.some((a) => a.includes("widget")));
}).concat("progressbar"), dx = mm.filter((e) => !vm.includes(e) && e !== "generic");
Ao.elementRoles.entries().forEach(([e, t]) => {
  [...t].every((a) => a !== "generic" && vm.includes(a));
});
Ao.elementRoles.entries().forEach(([e, t]) => {
  [...t].every((a) => dx.includes(a));
});
const px = [...Qt.AXObjects.keys()].filter((e) => Qt.AXObjects.get(e).type === "widget"), fx = [...Qt.AXObjects.keys()].filter((e) => ["windows", "structure"].includes(Qt.AXObjects.get(e).type));
Qt.elementAXObjects.entries().forEach(([e, t]) => {
  [...t].every((a) => px.includes(a));
});
Qt.elementAXObjects.entries().forEach(([e, t]) => {
  [...t].every((a) => fx.includes(a));
});
var vo = { exports: {} }, hx = vo.exports, fh;
function Ml() {
  return fh || (fh = 1, function(e, t) {
    (function(a, s) {
      s(t);
    })(hx, function(a) {
      const p = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/", m = new Uint8Array(64), w = new Uint8Array(128);
      for (let A = 0; A < p.length; A++) {
        const F = p.charCodeAt(A);
        m[A] = F, w[F] = A;
      }
      function x(A, F) {
        let E = 0, M = 0, V = 0;
        do {
          const r = A.next();
          V = w[r], E |= (V & 31) << M, M += 5;
        } while (V & 32);
        const c = E & 1;
        return E >>>= 1, c && (E = -2147483648 | -E), F + E;
      }
      function o(A, F, E) {
        let M = F - E;
        M = M < 0 ? -M << 1 | 1 : M << 1;
        do {
          let V = M & 31;
          M >>>= 5, M > 0 && (V |= 32), A.write(m[V]);
        } while (M > 0);
        return F;
      }
      function y(A, F) {
        return A.pos >= F ? false : A.peek() !== 44;
      }
      const O = 1024 * 16, L = typeof TextDecoder < "u" ? new TextDecoder() : typeof Buffer < "u" ? { decode(A) {
        return Buffer.from(A.buffer, A.byteOffset, A.byteLength).toString();
      } } : { decode(A) {
        let F = "";
        for (let E = 0; E < A.length; E++) F += String.fromCharCode(A[E]);
        return F;
      } };
      class B {
        constructor() {
          this.pos = 0, this.out = "", this.buffer = new Uint8Array(O);
        }
        write(F) {
          const { buffer: E } = this;
          E[this.pos++] = F, this.pos === O && (this.out += L.decode(E), this.pos = 0);
        }
        flush() {
          const { buffer: F, out: E, pos: M } = this;
          return M > 0 ? E + L.decode(F.subarray(0, M)) : E;
        }
      }
      class G {
        constructor(F) {
          this.pos = 0, this.buffer = F;
        }
        next() {
          return this.buffer.charCodeAt(this.pos++);
        }
        peek() {
          return this.buffer.charCodeAt(this.pos);
        }
        indexOf(F) {
          const { buffer: E, pos: M } = this, V = E.indexOf(F, M);
          return V === -1 ? E.length : V;
        }
      }
      const j = [];
      function R(A) {
        const { length: F } = A, E = new G(A), M = [], V = [];
        let c = 0;
        for (; E.pos < F; E.pos++) {
          c = x(E, c);
          const r = x(E, 0);
          if (!y(E, F)) {
            const P = V.pop();
            P[2] = c, P[3] = r;
            continue;
          }
          const i = x(E, 0), f = x(E, 0) & 1 ? [c, r, 0, 0, i, x(E, 0)] : [c, r, 0, 0, i];
          let b = j;
          if (y(E, F)) {
            b = [];
            do {
              const P = x(E, 0);
              b.push(P);
            } while (y(E, F));
          }
          f.vars = b, M.push(f), V.push(f);
        }
        return M;
      }
      function h(A) {
        const F = new B();
        for (let E = 0; E < A.length; ) E = v(A, E, F, [0]);
        return F.flush();
      }
      function v(A, F, E, M) {
        const V = A[F], { 0: c, 1: r, 2: i, 3: n, 4: l, vars: f } = V;
        F > 0 && E.write(44), M[0] = o(E, c, M[0]), o(E, r, 0), o(E, l, 0);
        const b = V.length === 6 ? 1 : 0;
        o(E, b, 0), V.length === 6 && o(E, V[5], 0);
        for (const P of f) o(E, P, 0);
        for (F++; F < A.length; ) {
          const P = A[F], { 0: q, 1: T } = P;
          if (q > i || q === i && T >= n) break;
          F = v(A, F, E, M);
        }
        return E.write(44), M[0] = o(E, i, M[0]), o(E, n, 0), F;
      }
      function g(A) {
        const { length: F } = A, E = new G(A), M = [], V = [];
        let c = 0, r = 0, i = 0, n = 0, l = 0, f = 0, b = 0, P = 0;
        do {
          const q = E.indexOf(";");
          let T = 0;
          for (; E.pos < q; E.pos++) {
            if (T = x(E, T), !y(E, q)) {
              const he = V.pop();
              he[2] = c, he[3] = T;
              continue;
            }
            const H = x(E, 0), J = H & 1, re = H & 2, Q = H & 4;
            let de = null, pe = j, fe;
            if (J) {
              const he = x(E, r);
              i = x(E, r === he ? i : 0), r = he, fe = [c, T, 0, 0, he, i];
            } else fe = [c, T, 0, 0];
            if (fe.isScope = !!Q, re) {
              const he = n, Ee = l;
              n = x(E, n);
              const Oe = he === n;
              l = x(E, Oe ? l : 0), f = x(E, Oe && Ee === l ? f : 0), de = [n, l, f];
            }
            if (fe.callsite = de, y(E, q)) {
              pe = [];
              do {
                b = c, P = T;
                const he = x(E, 0);
                let Ee;
                if (he < -1) {
                  Ee = [[x(E, 0)]];
                  for (let Oe = -1; Oe > he; Oe--) {
                    const ke = b;
                    b = x(E, b), P = x(E, b === ke ? P : 0);
                    const We = x(E, 0);
                    Ee.push([We, b, P]);
                  }
                } else Ee = [[he]];
                pe.push(Ee);
              } while (y(E, q));
            }
            fe.bindings = pe, M.push(fe), V.push(fe);
          }
          c++, E.pos = q + 1;
        } while (E.pos < F);
        return M;
      }
      function S(A) {
        if (A.length === 0) return "";
        const F = new B();
        for (let E = 0; E < A.length; ) E = C(A, E, F, [0, 0, 0, 0, 0, 0, 0]);
        return F.flush();
      }
      function C(A, F, E, M) {
        const V = A[F], { 0: c, 1: r, 2: i, 3: n, isScope: l, callsite: f, bindings: b } = V;
        M[0] < c ? (_(E, M[0], c), M[0] = c, M[1] = 0) : F > 0 && E.write(44), M[1] = o(E, V[1], M[1]);
        const P = (V.length === 6 ? 1 : 0) | (f ? 2 : 0) | (l ? 4 : 0);
        if (o(E, P, 0), V.length === 6) {
          const { 4: q, 5: T } = V;
          q !== M[2] && (M[3] = 0), M[2] = o(E, q, M[2]), M[3] = o(E, T, M[3]);
        }
        if (f) {
          const { 0: q, 1: T, 2: H } = V.callsite;
          q !== M[4] ? (M[5] = 0, M[6] = 0) : T !== M[5] && (M[6] = 0), M[4] = o(E, q, M[4]), M[5] = o(E, T, M[5]), M[6] = o(E, H, M[6]);
        }
        if (b) for (const q of b) {
          q.length > 1 && o(E, -q.length, 0);
          const T = q[0][0];
          o(E, T, 0);
          let H = c, J = r;
          for (let re = 1; re < q.length; re++) {
            const Q = q[re];
            H = o(E, Q[1], H), J = o(E, Q[2], J), o(E, Q[0], 0);
          }
        }
        for (F++; F < A.length; ) {
          const q = A[F], { 0: T, 1: H } = q;
          if (T > i || T === i && H >= n) break;
          F = C(A, F, E, M);
        }
        return M[0] < i ? (_(E, M[0], i), M[0] = i, M[1] = 0) : E.write(44), M[1] = o(E, n, M[1]), F;
      }
      function _(A, F, E) {
        do
          A.write(59);
        while (++F < E);
      }
      function k(A) {
        const { length: F } = A, E = new G(A), M = [];
        let V = 0, c = 0, r = 0, i = 0, n = 0;
        do {
          const l = E.indexOf(";"), f = [];
          let b = true, P = 0;
          for (V = 0; E.pos < l; ) {
            let q;
            V = x(E, V), V < P && (b = false), P = V, y(E, l) ? (c = x(E, c), r = x(E, r), i = x(E, i), y(E, l) ? (n = x(E, n), q = [V, c, r, i, n]) : q = [V, c, r, i]) : q = [V], f.push(q), E.pos++;
          }
          b || N(f), M.push(f), E.pos = l + 1;
        } while (E.pos <= F);
        return M;
      }
      function N(A) {
        A.sort(W);
      }
      function W(A, F) {
        return A[0] - F[0];
      }
      function ee(A) {
        const F = new B();
        let E = 0, M = 0, V = 0, c = 0;
        for (let r = 0; r < A.length; r++) {
          const i = A[r];
          if (r > 0 && F.write(59), i.length === 0) continue;
          let n = 0;
          for (let l = 0; l < i.length; l++) {
            const f = i[l];
            l > 0 && F.write(44), n = o(F, f[0], n), f.length !== 1 && (E = o(F, f[1], E), M = o(F, f[2], M), V = o(F, f[3], V), f.length !== 4 && (c = o(F, f[4], c)));
          }
        }
        return F.flush();
      }
      a.decode = k, a.decodeGeneratedRanges = g, a.decodeOriginalScopes = R, a.encode = ee, a.encodeGeneratedRanges = S, a.encodeOriginalScopes = h, Object.defineProperty(a, "__esModule", { value: true });
    });
  }(vo, vo.exports)), vo.exports;
}
Ml();
var Fo = { exports: {} }, bo = { exports: {} }, Vo = { exports: {} }, mx = Vo.exports, hh;
function vx() {
  return hh || (hh = 1, function(e, t) {
    (function(a, s) {
      e.exports = s();
    })(mx, function() {
      const a = /^[\w+.-]+:\/\//, s = /^([\w+.-]+:)\/\/([^@/#?]*@)?([^:/#?]*)(:\d+)?(\/[^#?]*)?(\?[^#]*)?(#.*)?/, u = /^file:(?:\/\/((?![a-z]:)[^/#?]*)?)?(\/?[^#?]*)(\?[^#]*)?(#.*)?/i;
      function p(v) {
        return a.test(v);
      }
      function m(v) {
        return v.startsWith("//");
      }
      function w(v) {
        return v.startsWith("/");
      }
      function x(v) {
        return v.startsWith("file:");
      }
      function o(v) {
        return /^[.?#]/.test(v);
      }
      function y(v) {
        const g = s.exec(v);
        return L(g[1], g[2] || "", g[3], g[4] || "", g[5] || "/", g[6] || "", g[7] || "");
      }
      function O(v) {
        const g = u.exec(v), S = g[2];
        return L("file:", "", g[1] || "", "", w(S) ? S : "/" + S, g[3] || "", g[4] || "");
      }
      function L(v, g, S, C, _, k, N) {
        return { scheme: v, user: g, host: S, port: C, path: _, query: k, hash: N, type: 7 };
      }
      function B(v) {
        if (m(v)) {
          const S = y("http:" + v);
          return S.scheme = "", S.type = 6, S;
        }
        if (w(v)) {
          const S = y("http://foo.com" + v);
          return S.scheme = "", S.host = "", S.type = 5, S;
        }
        if (x(v)) return O(v);
        if (p(v)) return y(v);
        const g = y("http://foo.com/" + v);
        return g.scheme = "", g.host = "", g.type = v ? v.startsWith("?") ? 3 : v.startsWith("#") ? 2 : 4 : 1, g;
      }
      function G(v) {
        if (v.endsWith("/..")) return v;
        const g = v.lastIndexOf("/");
        return v.slice(0, g + 1);
      }
      function j(v, g) {
        R(g, g.type), v.path === "/" ? v.path = g.path : v.path = G(g.path) + v.path;
      }
      function R(v, g) {
        const S = g <= 4, C = v.path.split("/");
        let _ = 1, k = 0, N = false;
        for (let ee = 1; ee < C.length; ee++) {
          const A = C[ee];
          if (!A) {
            N = true;
            continue;
          }
          if (N = false, A !== ".") {
            if (A === "..") {
              k ? (N = true, k--, _--) : S && (C[_++] = A);
              continue;
            }
            C[_++] = A, k++;
          }
        }
        let W = "";
        for (let ee = 1; ee < _; ee++) W += "/" + C[ee];
        (!W || N && !W.endsWith("/..")) && (W += "/"), v.path = W;
      }
      function h(v, g) {
        if (!v && !g) return "";
        const S = B(v);
        let C = S.type;
        if (g && C !== 7) {
          const k = B(g), N = k.type;
          switch (C) {
            case 1:
              S.hash = k.hash;
            case 2:
              S.query = k.query;
            case 3:
            case 4:
              j(S, k);
            case 5:
              S.user = k.user, S.host = k.host, S.port = k.port;
            case 6:
              S.scheme = k.scheme;
          }
          N > C && (C = N);
        }
        R(S, C);
        const _ = S.query + S.hash;
        switch (C) {
          case 2:
          case 3:
            return _;
          case 4: {
            const k = S.path.slice(1);
            return k ? o(g || v) && !o(k) ? "./" + k + _ : k + _ : _ || ".";
          }
          case 5:
            return S.path + _;
          default:
            return S.scheme + "//" + S.user + S.host + S.port + S.path + _;
        }
      }
      return h;
    });
  }(Vo)), Vo.exports;
}
var bx = bo.exports, mh;
function bm() {
  return mh || (mh = 1, function(e, t) {
    (function(a, s) {
      s(t, Ml(), vx());
    })(bx, function(a, s, u) {
      function p(D, X) {
        return X && !X.endsWith("/") && (X += "/"), u(D, X);
      }
      function m(D) {
        if (!D) return "";
        const X = D.lastIndexOf("/");
        return D.slice(0, X + 1);
      }
      const w = 0, x = 1, o = 2, y = 3, O = 4, L = 1, B = 2;
      function G(D, X) {
        const K = j(D, 0);
        if (K === D.length) return D;
        X || (D = D.slice());
        for (let Y = K; Y < D.length; Y = j(D, Y + 1)) D[Y] = h(D[Y], X);
        return D;
      }
      function j(D, X) {
        for (let K = X; K < D.length; K++) if (!R(D[K])) return K;
        return D.length;
      }
      function R(D) {
        for (let X = 1; X < D.length; X++) if (D[X][w] < D[X - 1][w]) return false;
        return true;
      }
      function h(D, X) {
        return X || (D = D.slice()), D.sort(v);
      }
      function v(D, X) {
        return D[w] - X[w];
      }
      let g = false;
      function S(D, X, K, Y) {
        for (; K <= Y; ) {
          const ce = K + (Y - K >> 1), ne = D[ce][w] - X;
          if (ne === 0) return g = true, ce;
          ne < 0 ? K = ce + 1 : Y = ce - 1;
        }
        return g = false, K - 1;
      }
      function C(D, X, K) {
        for (let Y = K + 1; Y < D.length && D[Y][w] === X; K = Y++) ;
        return K;
      }
      function _(D, X, K) {
        for (let Y = K - 1; Y >= 0 && D[Y][w] === X; K = Y--) ;
        return K;
      }
      function k() {
        return { lastKey: -1, lastNeedle: -1, lastIndex: -1 };
      }
      function N(D, X, K, Y) {
        const { lastKey: ce, lastNeedle: ne, lastIndex: ge } = K;
        let Ie = 0, xe = D.length - 1;
        if (Y === ce) {
          if (X === ne) return g = ge !== -1 && D[ge][w] === X, ge;
          X >= ne ? Ie = ge === -1 ? 0 : ge : xe = ge;
        }
        return K.lastKey = Y, K.lastNeedle = X, K.lastIndex = S(D, X, Ie, xe);
      }
      function W(D, X) {
        const K = X.map(A);
        for (let Y = 0; Y < D.length; Y++) {
          const ce = D[Y];
          for (let ne = 0; ne < ce.length; ne++) {
            const ge = ce[ne];
            if (ge.length === 1) continue;
            const Ie = ge[x], xe = ge[o], Te = ge[y], Ge = K[Ie], je = Ge[xe] || (Ge[xe] = []), De = X[Ie];
            let ze = C(je, Te, N(je, Te, De, xe));
            De.lastIndex = ++ze, ee(je, ze, [Te, Y, ge[w]]);
          }
        }
        return K;
      }
      function ee(D, X, K) {
        for (let Y = D.length; Y > X; Y--) D[Y] = D[Y - 1];
        D[X] = K;
      }
      function A() {
        return { __proto__: null };
      }
      const F = function(D, X) {
        const K = E(D);
        if (!("sections" in K)) return new b(K, X);
        const Y = [], ce = [], ne = [], ge = [], Ie = [];
        M(K, X, Y, ce, ne, ge, Ie, 0, 0, 1 / 0, 1 / 0);
        const xe = { version: 3, file: K.file, names: ge, sources: ce, sourcesContent: ne, mappings: Y, ignoreList: Ie };
        return Ee(xe);
      };
      function E(D) {
        return typeof D == "string" ? JSON.parse(D) : D;
      }
      function M(D, X, K, Y, ce, ne, ge, Ie, xe, Te, Ge) {
        const { sections: je } = D;
        for (let De = 0; De < je.length; De++) {
          const { map: ze, offset: Ze } = je[De];
          let te = Te, Se = Ge;
          if (De + 1 < je.length) {
            const be = je[De + 1].offset;
            te = Math.min(Te, Ie + be.line), te === Te ? Se = Math.min(Ge, xe + be.column) : te < Te && (Se = xe + be.column);
          }
          V(ze, X, K, Y, ce, ne, ge, Ie + Ze.line, xe + Ze.column, te, Se);
        }
      }
      function V(D, X, K, Y, ce, ne, ge, Ie, xe, Te, Ge) {
        const je = E(D);
        if ("sections" in je) return M(...arguments);
        const De = new b(je, X), ze = Y.length, Ze = ne.length, te = T(De), { resolvedSources: Se, sourcesContent: be, ignoreList: Qe } = De;
        if (c(Y, Se), c(ne, De.names), be) c(ce, be);
        else for (let Fe = 0; Fe < Se.length; Fe++) ce.push(null);
        if (Qe) for (let Fe = 0; Fe < Qe.length; Fe++) ge.push(Qe[Fe] + ze);
        for (let Fe = 0; Fe < te.length; Fe++) {
          const ot = Ie + Fe;
          if (ot > Te) return;
          const Xe = r(K, ot), Ye = Fe === 0 ? xe : 0, Ct = te[Fe];
          for (let Z = 0; Z < Ct.length; Z++) {
            const Rt = Ct[Z], xt = Ye + Rt[w];
            if (ot === Te && xt >= Ge) return;
            if (Rt.length === 1) {
              Xe.push([xt]);
              continue;
            }
            const Eo = ze + Rt[x], ko = Rt[o], Io = Rt[y];
            Xe.push(Rt.length === 4 ? [xt, Eo, ko, Io] : [xt, Eo, ko, Io, Ze + Rt[O]]);
          }
        }
      }
      function c(D, X) {
        for (let K = 0; K < X.length; K++) D.push(X[K]);
      }
      function r(D, X) {
        for (let K = D.length; K <= X; K++) D[K] = [];
        return D[X];
      }
      const i = "`line` must be greater than 0 (lines start at line 1)", n = "`column` must be greater than or equal to 0 (columns start at column 0)", l = -1, f = 1;
      class b {
        constructor(X, K) {
          const Y = typeof X == "string";
          if (!Y && X._decodedMemo) return X;
          const ce = Y ? JSON.parse(X) : X, { version: ne, file: ge, names: Ie, sourceRoot: xe, sources: Te, sourcesContent: Ge } = ce;
          this.version = ne, this.file = ge, this.names = Ie || [], this.sourceRoot = xe, this.sources = Te, this.sourcesContent = Ge, this.ignoreList = ce.ignoreList || ce.x_google_ignoreList || void 0;
          const je = p(xe || "", m(K));
          this.resolvedSources = Te.map((ze) => p(ze || "", je));
          const { mappings: De } = ce;
          typeof De == "string" ? (this._encoded = De, this._decoded = void 0) : (this._encoded = void 0, this._decoded = G(De, Y)), this._decodedMemo = k(), this._bySources = void 0, this._bySourceMemos = void 0;
        }
      }
      function P(D) {
        return D;
      }
      function q(D) {
        var X, K;
        return (X = (K = D)._encoded) !== null && X !== void 0 ? X : K._encoded = s.encode(D._decoded);
      }
      function T(D) {
        var X;
        return (X = D)._decoded || (X._decoded = s.decode(D._encoded));
      }
      function H(D, X, K) {
        const Y = T(D);
        if (X >= Y.length) return null;
        const ce = Y[X], ne = Je(ce, D._decodedMemo, X, K, f);
        return ne === -1 ? null : ce[ne];
      }
      function J(D, X) {
        let { line: K, column: Y, bias: ce } = X;
        if (K--, K < 0) throw new Error(i);
        if (Y < 0) throw new Error(n);
        const ne = T(D);
        if (K >= ne.length) return Ne(null, null, null, null);
        const ge = ne[K], Ie = Je(ge, D._decodedMemo, K, Y, ce || f);
        if (Ie === -1) return Ne(null, null, null, null);
        const xe = ge[Ie];
        if (xe.length === 1) return Ne(null, null, null, null);
        const { names: Te, resolvedSources: Ge } = D;
        return Ne(Ge[xe[x]], xe[o] + 1, xe[y], xe.length === 5 ? Te[xe[O]] : null);
      }
      function re(D, X) {
        const { source: K, line: Y, column: ce, bias: ne } = X;
        return gt(D, K, Y, ce, ne || f, false);
      }
      function Q(D, X) {
        const { source: K, line: Y, column: ce, bias: ne } = X;
        return gt(D, K, Y, ce, ne || l, true);
      }
      function de(D, X) {
        const K = T(D), { names: Y, resolvedSources: ce } = D;
        for (let ne = 0; ne < K.length; ne++) {
          const ge = K[ne];
          for (let Ie = 0; Ie < ge.length; Ie++) {
            const xe = ge[Ie], Te = ne + 1, Ge = xe[0];
            let je = null, De = null, ze = null, Ze = null;
            xe.length !== 1 && (je = ce[xe[1]], De = xe[2] + 1, ze = xe[3]), xe.length === 5 && (Ze = Y[xe[4]]), X({ generatedLine: Te, generatedColumn: Ge, source: je, originalLine: De, originalColumn: ze, name: Ze });
          }
        }
      }
      function pe(D, X) {
        const { sources: K, resolvedSources: Y } = D;
        let ce = K.indexOf(X);
        return ce === -1 && (ce = Y.indexOf(X)), ce;
      }
      function fe(D, X) {
        const { sourcesContent: K } = D;
        if (K == null) return null;
        const Y = pe(D, X);
        return Y === -1 ? null : K[Y];
      }
      function he(D, X) {
        const { ignoreList: K } = D;
        if (K == null) return false;
        const Y = pe(D, X);
        return Y === -1 ? false : K.includes(Y);
      }
      function Ee(D, X) {
        const K = new b(We(D, []), X);
        return K._decoded = D.mappings, K;
      }
      function Oe(D) {
        return We(D, T(D));
      }
      function ke(D) {
        return We(D, q(D));
      }
      function We(D, X) {
        return { version: D.version, file: D.file, names: D.names, sourceRoot: D.sourceRoot, sources: D.sources, sourcesContent: D.sourcesContent, mappings: X, ignoreList: D.ignoreList || D.x_google_ignoreList };
      }
      function Ne(D, X, K, Y) {
        return { source: D, line: X, column: K, name: Y };
      }
      function tt(D, X) {
        return { line: D, column: X };
      }
      function Je(D, X, K, Y, ce) {
        let ne = N(D, Y, X, K);
        return g ? ne = (ce === l ? C : _)(D, Y, ne) : ce === l && ne++, ne === -1 || ne === D.length ? -1 : ne;
      }
      function it(D, X, K, Y, ce) {
        let ne = Je(D, X, K, Y, f);
        if (!g && ce === l && ne++, ne === -1 || ne === D.length) return [];
        const ge = g ? Y : D[ne][w];
        g || (ne = _(D, ge, ne));
        const Ie = C(D, ge, ne), xe = [];
        for (; ne <= Ie; ne++) {
          const Te = D[ne];
          xe.push(tt(Te[L] + 1, Te[B]));
        }
        return xe;
      }
      function gt(D, X, K, Y, ce, ne) {
        var ge;
        if (K--, K < 0) throw new Error(i);
        if (Y < 0) throw new Error(n);
        const { sources: Ie, resolvedSources: xe } = D;
        let Te = Ie.indexOf(X);
        if (Te === -1 && (Te = xe.indexOf(X)), Te === -1) return ne ? [] : tt(null, null);
        const je = ((ge = D)._bySources || (ge._bySources = W(T(D), D._bySourceMemos = Ie.map(k))))[Te][K];
        if (je == null) return ne ? [] : tt(null, null);
        const De = D._bySourceMemos[Te];
        if (ne) return it(je, De, K, Y, ce);
        const ze = Je(je, De, K, Y, ce);
        if (ze === -1) return tt(null, null);
        const Ze = je[ze];
        return tt(Ze[L] + 1, Ze[B]);
      }
      a.AnyMap = F, a.GREATEST_LOWER_BOUND = f, a.LEAST_UPPER_BOUND = l, a.TraceMap = b, a.allGeneratedPositionsFor = Q, a.decodedMap = Oe, a.decodedMappings = T, a.eachMapping = de, a.encodedMap = ke, a.encodedMappings = q, a.generatedPositionFor = re, a.isIgnored = he, a.originalPositionFor = J, a.presortedDecodedMap = Ee, a.sourceContentFor = fe, a.traceSegment = H;
    });
  }(bo, bo.exports)), bo.exports;
}
var yo = { exports: {} }, go = { exports: {} }, yx = go.exports, vh;
function gx() {
  return vh || (vh = 1, function(e, t) {
    (function(a, s) {
      s(t);
    })(yx, function(a) {
      class s {
        constructor() {
          this._indexes = { __proto__: null }, this.array = [];
        }
      }
      function u(o) {
        return o;
      }
      function p(o, y) {
        return o._indexes[y];
      }
      function m(o, y) {
        const O = p(o, y);
        if (O !== void 0) return O;
        const { array: L, _indexes: B } = o, G = L.push(y);
        return B[y] = G - 1;
      }
      function w(o) {
        const { array: y, _indexes: O } = o;
        if (y.length === 0) return;
        const L = y.pop();
        O[L] = void 0;
      }
      function x(o, y) {
        const O = p(o, y);
        if (O === void 0) return;
        const { array: L, _indexes: B } = o;
        for (let G = O + 1; G < L.length; G++) {
          const j = L[G];
          L[G - 1] = j, B[j]--;
        }
        B[y] = void 0, L.pop();
      }
      a.SetArray = s, a.get = p, a.pop = w, a.put = m, a.remove = x, Object.defineProperty(a, "__esModule", { value: true });
    });
  }(go, go.exports)), go.exports;
}
var Rx = yo.exports, bh;
function xx() {
  return bh || (bh = 1, function(e, t) {
    (function(a, s) {
      s(t, gx(), Ml(), bm());
    })(Rx, function(a, s, u, p) {
      class L {
        constructor({ file: i, sourceRoot: n } = {}) {
          this._names = new s.SetArray(), this._sources = new s.SetArray(), this._sourcesContent = [], this._mappings = [], this.file = i, this.sourceRoot = n, this._ignoreList = new s.SetArray();
        }
      }
      function B(r) {
        return r;
      }
      function G(r, i, n, l, f, b, P, q) {
        return N(false, r, i, n, l, f, b, P, q);
      }
      function j(r, i) {
        return c(false, r, i);
      }
      const R = (r, i, n, l, f, b, P, q) => N(true, r, i, n, l, f, b, P, q), h = (r, i) => c(true, r, i);
      function v(r, i, n) {
        const { _sources: l, _sourcesContent: f } = r, b = s.put(l, i);
        f[b] = n;
      }
      function g(r, i, n = true) {
        const { _sources: l, _sourcesContent: f, _ignoreList: b } = r, P = s.put(l, i);
        P === f.length && (f[P] = null), n ? s.put(b, P) : s.remove(b, P);
      }
      function S(r) {
        const { _mappings: i, _sources: n, _sourcesContent: l, _names: f, _ignoreList: b } = r;
        return F(i), { version: 3, file: r.file || void 0, names: f.array, sourceRoot: r.sourceRoot || void 0, sources: n.array, sourcesContent: l, mappings: i, ignoreList: b.array };
      }
      function C(r) {
        const i = S(r);
        return Object.assign(Object.assign({}, i), { mappings: u.encode(i.mappings) });
      }
      function _(r) {
        const i = new p.TraceMap(r), n = new L({ file: i.file, sourceRoot: i.sourceRoot });
        return E(n._names, i.names), E(n._sources, i.sources), n._sourcesContent = i.sourcesContent || i.sources.map(() => null), n._mappings = p.decodedMappings(i), i.ignoreList && E(n._ignoreList, i.ignoreList), n;
      }
      function k(r) {
        const i = [], { _mappings: n, _sources: l, _names: f } = r;
        for (let b = 0; b < n.length; b++) {
          const P = n[b];
          for (let q = 0; q < P.length; q++) {
            const T = P[q], H = { line: b + 1, column: T[0] };
            let J, re, Q;
            T.length !== 1 && (J = l.array[T[1]], re = { line: T[2] + 1, column: T[3] }, T.length === 5 && (Q = f.array[T[4]])), i.push({ generated: H, source: J, original: re, name: Q });
          }
        }
        return i;
      }
      function N(r, i, n, l, f, b, P, q, T) {
        const { _mappings: H, _sources: J, _sourcesContent: re, _names: Q } = i, de = W(H, n), pe = ee(de, l);
        if (!f) return r && M(de, pe) ? void 0 : A(de, pe, [l]);
        const fe = s.put(J, f), he = q ? s.put(Q, q) : -1;
        if (fe === re.length && (re[fe] = T ?? null), !(r && V(de, pe, fe, b, P, he))) return A(de, pe, q ? [l, fe, b, P, he] : [l, fe, b, P]);
      }
      function W(r, i) {
        for (let n = r.length; n <= i; n++) r[n] = [];
        return r[i];
      }
      function ee(r, i) {
        let n = r.length;
        for (let l = n - 1; l >= 0; n = l--) {
          const f = r[l];
          if (i >= f[0]) break;
        }
        return n;
      }
      function A(r, i, n) {
        for (let l = r.length; l > i; l--) r[l] = r[l - 1];
        r[i] = n;
      }
      function F(r) {
        const { length: i } = r;
        let n = i;
        for (let l = n - 1; l >= 0 && !(r[l].length > 0); n = l, l--) ;
        n < i && (r.length = n);
      }
      function E(r, i) {
        for (let n = 0; n < i.length; n++) s.put(r, i[n]);
      }
      function M(r, i) {
        return i === 0 ? true : r[i - 1].length === 1;
      }
      function V(r, i, n, l, f, b) {
        if (i === 0) return false;
        const P = r[i - 1];
        return P.length === 1 ? false : n === P[1] && l === P[2] && f === P[3] && b === (P.length === 5 ? P[4] : -1);
      }
      function c(r, i, n) {
        const { generated: l, source: f, original: b, name: P, content: q } = n;
        return f ? N(r, i, l.line - 1, l.column, f, b.line - 1, b.column, P, q) : N(r, i, l.line - 1, l.column, null, null, null, null, null);
      }
      a.GenMapping = L, a.addMapping = j, a.addSegment = G, a.allMappings = k, a.fromMap = _, a.maybeAddMapping = h, a.maybeAddSegment = R, a.setIgnore = g, a.setSourceContent = v, a.toDecodedMap = S, a.toEncodedMap = C, Object.defineProperty(a, "__esModule", { value: true });
    });
  }(yo, yo.exports)), yo.exports;
}
var _x = Fo.exports, yh;
function Cx() {
  return yh || (yh = 1, function(e, t) {
    (function(a, s) {
      e.exports = s(bm(), xx());
    })(_x, function(a, s) {
      const u = m("", -1, -1, "", null, false), p = [];
      function m(h, v, g, S, C, _) {
        return { source: h, line: v, column: g, name: S, content: C, ignore: _ };
      }
      function w(h, v, g, S, C) {
        return { map: h, sources: v, source: g, content: S, ignore: C };
      }
      function x(h, v) {
        return w(h, v, "", null, false);
      }
      function o(h, v, g) {
        return w(null, p, h, v, g);
      }
      function y(h) {
        const v = new s.GenMapping({ file: h.map.file }), { sources: g, map: S } = h, C = S.names, _ = a.decodedMappings(S);
        for (let k = 0; k < _.length; k++) {
          const N = _[k];
          for (let W = 0; W < N.length; W++) {
            const ee = N[W], A = ee[0];
            let F = u;
            if (ee.length !== 1) {
              const n = g[ee[1]];
              if (F = O(n, ee[2], ee[3], ee.length === 5 ? C[ee[4]] : ""), F == null) continue;
            }
            const { column: E, line: M, name: V, content: c, source: r, ignore: i } = F;
            s.maybeAddSegment(v, k, A, r, M, E, V), r && c != null && s.setSourceContent(v, r, c), i && s.setIgnore(v, r, true);
          }
        }
        return v;
      }
      function O(h, v, g, S) {
        if (!h.map) return m(h.source, v, g, S, h.content, h.ignore);
        const C = a.traceSegment(h.map, v, g);
        return C == null ? null : C.length === 1 ? u : O(h.sources[C[1]], C[2], C[3], C.length === 5 ? h.map.names[C[4]] : S);
      }
      function L(h) {
        return Array.isArray(h) ? h : [h];
      }
      function B(h, v) {
        const g = L(h).map((_) => new a.TraceMap(_, "")), S = g.pop();
        for (let _ = 0; _ < g.length; _++) if (g[_].sources.length > 1) throw new Error(`Transformation map ${_} must have exactly one source file.
Did you specify these with the most recent transformation maps first?`);
        let C = G(S, v, "", 0);
        for (let _ = g.length - 1; _ >= 0; _--) C = x(g[_], [C]);
        return C;
      }
      function G(h, v, g, S) {
        const { resolvedSources: C, sourcesContent: _, ignoreList: k } = h, N = S + 1, W = C.map((ee, A) => {
          const F = { importer: g, depth: N, source: ee || "", content: void 0, ignore: void 0 }, E = v(F.source, F), { source: M, content: V, ignore: c } = F;
          if (E) return G(new a.TraceMap(E, M), v, M, N);
          const r = V !== void 0 ? V : _ ? _[A] : null, i = c !== void 0 ? c : k ? k.includes(A) : false;
          return o(M, r, i);
        });
        return x(h, W);
      }
      class j {
        constructor(v, g) {
          const S = g.decodedMappings ? s.toDecodedMap(v) : s.toEncodedMap(v);
          this.version = S.version, this.file = S.file, this.mappings = S.mappings, this.names = S.names, this.ignoreList = S.ignoreList, this.sourceRoot = S.sourceRoot, this.sources = S.sources, g.excludeContent || (this.sourcesContent = S.sourcesContent);
        }
        toString() {
          return JSON.stringify(this);
        }
      }
      function R(h, v, g) {
        const S = typeof g == "object" ? g : { excludeContent: !!g, decodedMappings: false }, C = B(h, v);
        return new j(y(C), S);
      }
      return R;
    });
  }(Fo)), Fo.exports;
}
Cx();
const Px = { filename: _o("(unknown)"), rootDir: _o(typeof process < "u" ? (_a2 = process.cwd) == null ? void 0 : _a2.call(process) : typeof Deno < "u" ? Deno.cwd() : void 0), dev: Pt(false), generate: $t("client", (e, t) => e === "dom" || e === "ssr" ? (Ll(Jv), e === "dom" ? "client" : "server") : (e !== "client" && e !== "server" && e !== false && qt(`${t} must be "client", "server" or false`), e)), warningFilter: ym(() => true) };
xh({ ...Px, accessors: Rh(Gv, Pt(false)), css: $t("external", (e) => ((e === true || e === false) && qt('The boolean options have been removed from the css option. Use "external" instead of false and "injected" instead of true'), e === "none" && qt('css: "none" is no longer a valid option. If this was crucial for you, please open an issue on GitHub with your use case.'), e !== "external" && e !== "injected" && qt('css should be either "external" (default, recommended) or "injected"'), e)), cssHash: ym(({ css: e, hash: t }) => `svelte-${t(e)}`), cssOutputFilename: _o(void 0), customElement: Pt(false), discloseVersion: Pt(true), immutable: Rh(zv, Pt(false)), legacy: zt("The legacy option has been removed. If you are using this because of legacy.componentApi, use compatibility.componentApi instead"), compatibility: xh({ componentApi: _h([4, 5], 5) }), loopGuardTimeout: fl(Kv), name: _o(void 0), namespace: _h(["html", "mathml", "svg"]), modernAst: Pt(false), outputFilename: _o(void 0), preserveComments: Pt(false), preserveWhitespace: Pt(false), runes: Pt(void 0), hmr: Pt(false), sourcemap: $t(void 0, (e) => e), enableSourcemap: fl(Wv), hydratable: fl(Xv), format: zt('The format option has been removed in Svelte 4, the compiler only outputs ESM now. Remove "format" from your compiler options. If you did not set this yourself, bump the version of your bundler plugin (vite-plugin-svelte/rollup-plugin-svelte/svelte-loader)'), tag: zt('The tag option has been removed in Svelte 5. Use `<svelte:options customElement="tag-name" />` inside the component instead. If that does not solve your use case, please open an issue on GitHub with details.'), sveltePath: zt("The sveltePath option has been removed in Svelte 5. If this option was crucial for you, please open an issue on GitHub with your use case."), errorMode: zt("The errorMode option has been removed. If you are using this through svelte-preprocess with TypeScript, use the https://www.typescriptlang.org/tsconfig#verbatimModuleSyntax setting instead"), varsReport: zt("The vars option has been removed. If you are using this through svelte-preprocess with TypeScript, use the https://www.typescriptlang.org/tsconfig#verbatimModuleSyntax setting instead") });
function zt(e) {
  return (t) => {
    t !== void 0 && Q1(null, e);
  };
}
const gh = /* @__PURE__ */ new Set();
function Ll(e) {
  gh.has(e) || (gh.add(e), e(null));
}
function fl(e) {
  return (t) => {
    t !== void 0 && Ll(e);
  };
}
function Rh(e, t) {
  return (a, s) => (a !== void 0 && Ll(e), t(a, s));
}
function xh(e, t = false) {
  return (a, s) => {
    const u = {};
    (a && typeof a != "object" || Array.isArray(a)) && qt(`${s} should be an object`);
    for (const p in a) p in e || (t ? u[p] = a[p] : Y1(null, `${s ? `${s}.${p}` : p}`));
    for (const p in e) {
      const m = e[p];
      u[p] = m(a && a[p], s ? `${s}.${p}` : p);
    }
    return u;
  };
}
function $t(e, t) {
  return (a, s) => a === void 0 ? e : t(a, s);
}
function _o(e, t = true) {
  return $t(e, (a, s) => (typeof a != "string" && qt(`${s} should be a string, if specified`), !t && a === "" && qt(`${s} cannot be empty`), a));
}
function Pt(e) {
  return $t(e, (t, a) => (typeof t != "boolean" && qt(`${a} should be true or false, if specified`), t));
}
function _h(e, t = e[0]) {
  return $t(t, (a, s) => {
    if (!e.includes(a)) {
      const u = e.length > 2 ? `${s} should be one of ${e.slice(0, -1).map((p) => `"${p}"`).join(", ")} or "${e[e.length - 1]}"` : `${s} should be either "${e[0]}" or "${e[1]}"`;
      qt(u);
    }
    return a;
  });
}
function ym(e) {
  return $t(e, (t, a) => (typeof t != "function" && qt(`${a} should be a function, if specified`), t));
}
function qt(e) {
  J1(null, e);
}
var qx = Me("<p> </p>"), Tx = Me('<p> </p> <!> <div class="flex gap-05"><!> <!></div>', 1), Sx = Me('<div class="err"> </div>'), wx = Me('<div class="container svelte-b4h84n"><!></div> <!>', 1);
function Ax(e, t) {
  Mt(t, true);
  let a = Yt(), s = jt(), u = ie(""), p = ie(false), m = ie(z([])), w = ie(z([])), x = ie(z([]));
  gl(() => {
    o();
  }), Et(() => {
    y();
  }), Et(() => {
    $(x, z(I(m).map((g) => {
      var _a3;
      return { name: g.name, desc: g.desc, value: ((_a3 = I(w).find((C) => C.key === g.name)) == null ? void 0 : _a3.value) || "" };
    })));
  });
  async function o() {
    var _a3;
    let g = await Dt("/auth/v1/users/attr");
    g.body ? $(m, z(g.body.values.toSorted((S, C) => S.name.localeCompare(C.name)))) : $(u, z(((_a3 = g.error) == null ? void 0 : _a3.message) || "Error fetching attrs"));
  }
  async function y() {
    var _a3;
    let g = await Dt(`/auth/v1/users/${t.user.id}/attr`);
    g.body ? $(w, z(g.body.values)) : $(u, z(((_a3 = g.error) == null ? void 0 : _a3.message) || "Error fetching users attrs"));
  }
  async function O() {
    $(u, "");
    let g = I(w).map((_) => _.key), S = { values: I(x).filter((_) => _.value.trim() || g.includes(_.name)).map((_) => ({ key: _.name, value: _.value.trim() })) }, C = await Rl(`/auth/v1/users/${t.user.id}/attr`, S);
    C.error ? $(u, z(C.error.message)) : ($(p, true), setTimeout(() => {
      $(p, false);
    }, 3e3)), t.onSave();
  }
  var L = wx(), B = Le(L), G = ve(B);
  {
    var j = (g) => {
      var S = qx(), C = ve(S, true);
      me(S), _e(() => Re(C, s.common.noEntries)), ae(g, S);
    }, R = (g) => {
      var S = Tx(), C = Le(S), _ = ve(C, true);
      me(C);
      var k = se(C, 2);
      Ho(k, 17, () => I(x), ov, (F, E, M) => {
        const V = bt(() => I(E).desc || "JSON Value");
        ht(F, { autocomplete: "off", get label() {
          return I(E).name;
        }, get placeholder() {
          return I(V);
        }, get value() {
          return I(E).value;
        }, set value(c) {
          I(E).value = c;
        } });
      });
      var N = se(k, 2), W = ve(N);
      At(W, { onclick: O, children: (F, E) => {
        Ke();
        var M = He();
        _e(() => Re(M, a.common.save)), ae(F, M);
      }, $$slots: { default: true } });
      var ee = se(W, 2);
      {
        var A = (F) => {
          Yo(F, {});
        };
        Pe(ee, (F) => {
          I(p) && F(A);
        });
      }
      me(N), _e(() => Re(_, s.users.descAttr)), ae(g, S);
    };
    Pe(G, (g) => {
      I(x).length === 0 ? g(j) : g(R, false);
    });
  }
  me(B);
  var h = se(B, 2);
  {
    var v = (g) => {
      var S = Sx(), C = ve(S, true);
      me(S), _e(() => Re(C, I(u))), ae(g, S);
    };
    Pe(h, (g) => {
      I(u) && g(v);
    });
  }
  ae(e, L), Lt();
}
var Ex = Me("<p><b> </b></p> <p> </p> <!>", 1), kx = Me('<div class="desc"><p><b> </b></p> <p> </p> <p> </p></div>'), Ix = Me('<!> <!> <!> <div></div> <p> </p> <div class="flex gap-05"><!> <!> <!></div>', 1), Mx = Me("<div></div> <!> <!>", 1), Lx = Me('<div class="err"> </div>'), Ox = Me("<!> <!>", 1);
function Nx(e, t) {
  Mt(t, true);
  const a = "min(20rem, calc(100dvw - .5rem))";
  let s = Yt(), u = jt(), p = ie(false), m = ie(""), w = ie(false), x = ie(void 0), o = ie(false), y = ie(""), O = ie(""), L = bt(() => {
    var _a3;
    return I(y).length > (((_a3 = I(x)) == null ? void 0 : _a3.length_min) || 8) && I(y) === I(O);
  }), B = ie(void 0), G = ie(void 0);
  gl(async () => {
    var _a3;
    let N = await Dt("/auth/v1/password_policy");
    N.body ? $(x, z(N.body)) : $(m, z(((_a3 = N.error) == null ? void 0 : _a3.message) || "Error"));
  }), Et(() => {
    t.user.id && ($(y, ""), $(O, ""));
  });
  async function j() {
    $(m, ""), $(p, true);
    let N = { email: t.user.email }, W = await qh("/auth/v1/users/request_reset", N);
    W.error ? $(m, z(W.error.message)) : ($(w, true), setTimeout(() => {
      $(w, false);
    }, 3e3)), $(p, false);
  }
  async function R(N, W) {
    if (!I(o)) {
      $(m, z(s.account.passwordPolicyFollow));
      return;
    }
    if (I(y) !== I(O)) {
      $(m, z(s.account.passwordNoMatch));
      return;
    }
    let ee = { email: t.user.email, given_name: t.user.given_name, family_name: t.user.family_name, language: t.user.language, password: I(y), roles: t.user.roles, groups: t.user.groups, enabled: t.user.enabled, email_verified: t.user.email_verified, user_expires: t.user.user_expires };
    $(m, ""), $(p, true);
    let A = await Rl(N.action, ee);
    A.error ? $(m, z(A.error.message)) : ($(w, true), setTimeout(() => {
      $(w, false), t.onSave();
    }, 3e3)), $(p, false);
  }
  function h() {
    if ($(m, ""), !I(x)) return;
    let N = yv(I(x));
    $(y, z(N)), $(O, z(N)), requestAnimationFrame(() => {
      var _a3, _b2;
      (_a3 = I(B)) == null ? void 0 : _a3(), (_b2 = I(G)) == null ? void 0 : _b2();
    });
  }
  var v = Ox(), g = Le(v);
  {
    var S = (N) => {
      var W = Ex(), ee = Le(W), A = ve(ee), F = ve(A, true);
      me(A), me(ee);
      var E = se(ee, 2), M = ve(E, true);
      me(E);
      var V = se(E, 2);
      At(V, { onclick: j, get isLoading() {
        return I(p);
      }, children: (c, r) => {
        Ke();
        var i = He();
        _e(() => Re(i, u.users.pwdSendEmailBtn)), ae(c, i);
      }, $$slots: { default: true } }), _e(() => {
        Re(F, u.users.pwdNoInit), Re(M, u.users.pwdSendEmailDesc);
      }), ae(N, W);
    }, C = (N) => {
      var W = nt(), ee = Le(W);
      {
        var A = (E) => {
          var M = kx(), V = ve(M), c = ve(V), r = ve(c, true);
          me(c), me(V);
          var i = se(V, 2), n = ve(i, true);
          me(i);
          var l = se(i, 2), f = ve(l, true);
          me(l), me(M), _e(() => {
            Re(r, u.users.pkOnly1), Re(n, u.users.pkOnly2), Re(f, u.users.pkOnly3);
          }), ae(E, M);
        }, F = (E) => {
          var M = Mx(), V = Le(M);
          Xt(V, "margin-top", ".5rem");
          var c = se(V, 2);
          {
            var r = (l) => {
              Rv(l, { get password() {
                return I(y);
              }, get policy() {
                return I(x);
              }, get accepted() {
                return I(o);
              }, set accepted(f) {
                $(o, z(f));
              } });
            };
            Pe(c, (l) => {
              I(x) && l(r);
            });
          }
          var i = se(c, 2);
          const n = bt(() => `/auth/v1/users/${t.user.id}`);
          _l(i, { get action() {
            return I(n);
          }, onSubmit: R, children: (l, f) => {
            var b = Ix(), P = Le(b);
            const q = bt(() => {
              var _a3;
              return ((_a3 = I(x)) == null ? void 0 : _a3.length_max) || 256;
            });
            Bl(P, { autocomplete: "off", get label() {
              return s.account.passwordNew;
            }, get placeholder() {
              return s.account.passwordNew;
            }, get showCopy() {
              return I(L);
            }, required: true, get maxLength() {
              return I(q);
            }, width: a, get value() {
              return I(y);
            }, set value(ke) {
              $(y, z(ke));
            }, get reportValidity() {
              return I(B);
            }, set reportValidity(ke) {
              $(B, z(ke));
            } });
            var T = se(P, 2);
            const H = bt(() => {
              var _a3;
              return ((_a3 = I(x)) == null ? void 0 : _a3.length_max) || 256;
            });
            Bl(T, { autocomplete: "off", get label() {
              return s.account.passwordConfirm;
            }, get placeholder() {
              return s.account.passwordConfirm;
            }, required: true, get maxLength() {
              return I(H);
            }, width: a, get value() {
              return I(O);
            }, set value(ke) {
              $(O, z(ke));
            }, get reportValidity() {
              return I(G);
            }, set reportValidity(ke) {
              $(G, z(ke));
            } });
            var J = se(T, 2);
            At(J, { level: 2, onclick: h, children: (ke, We) => {
              Ke();
              var Ne = He();
              _e(() => Re(Ne, s.account.generateRandom)), ae(ke, Ne);
            }, $$slots: { default: true } });
            var re = se(J, 2);
            Xt(re, "margin-top", "1rem");
            var Q = se(re, 2), de = ve(Q, true);
            me(Q);
            var pe = se(Q, 2), fe = ve(pe);
            At(fe, { onclick: j, get isLoading() {
              return I(p);
            }, children: (ke, We) => {
              Ke();
              var Ne = He();
              _e(() => Re(Ne, u.users.sendResetEmail)), ae(ke, Ne);
            }, $$slots: { default: true } });
            var he = se(fe, 2);
            At(he, { type: "submit", level: 2, get isLoading() {
              return I(p);
            }, children: (ke, We) => {
              Ke();
              var Ne = He();
              _e(() => Re(Ne, u.users.savePassword)), ae(ke, Ne);
            }, $$slots: { default: true } });
            var Ee = se(he, 2);
            {
              var Oe = (ke) => {
                Yo(ke, {});
              };
              Pe(Ee, (ke) => {
                I(w) && ke(Oe);
              });
            }
            me(pe), _e(() => Re(de, u.users.selfServiceDesc)), ae(l, b);
          }, $$slots: { default: true } }), ae(E, M);
        };
        Pe(ee, (E) => {
          t.user.account_type === "passkey" || t.user.account_type === "federated_passkey" ? E(A) : E(F, false);
        }, true);
      }
      ae(N, W);
    };
    Pe(g, (N) => {
      t.user.account_type === "new" ? N(S) : N(C, false);
    });
  }
  var _ = se(g, 2);
  {
    var k = (N) => {
      var W = Lx(), ee = ve(W, true);
      me(W), _e(() => Re(ee, I(m))), ae(N, W);
    };
    Pe(_, (N) => {
      I(m) && N(k);
    });
  }
  ae(e, v), Lt();
}
var Dx = Me("<p> </p>"), jx = Me('<p> </p> <p><!></p> <div class="keysContainer svelte-1teaezc"></div>', 1), Bx = Me('<div class="err"> </div>'), Fx = Me("<!> <!>", 1);
function Vx(e, t) {
  Mt(t, true);
  let a = jt(), s = ie(""), u = ie(z([]));
  Et(() => {
    p();
  });
  async function p() {
    var _a3;
    let B = await Dt(`/auth/v1/users/${t.user.id}/webauthn`);
    B.body ? $(u, z(B.body)) : $(s, z(((_a3 = B.error) == null ? void 0 : _a3.message) || "Error"));
  }
  async function m(B) {
    var _a3;
    let G = I(u).length === 1, j = await xl(`/auth/v1/users/${t.user.id}/webauthn/delete/${B}`);
    j.status === 200 ? (await p(), G && t.onSave()) : $(s, z(((_a3 = j.error) == null ? void 0 : _a3.message) || "Error"));
  }
  var w = Fx(), x = Le(w);
  {
    var o = (B) => {
      var G = Dx(), j = ve(G, true);
      me(G), _e(() => Re(j, a.users.noMfaKeys)), ae(B, G);
    }, y = (B) => {
      var G = jx(), j = Le(G), R = ve(j, true);
      me(j);
      var h = se(j, 2), v = ve(h);
      xv(v, () => a.users.mfaDelete2), me(h);
      var g = se(h, 2);
      Ho(g, 21, () => I(u), (S) => S.name, (S, C) => {
        _v(S, { get passkey() {
          return I(C);
        }, showDelete: true, onDelete: m });
      }), me(g), _e(() => Re(R, a.users.mfaDelete1)), ae(B, G);
    };
    Pe(x, (B) => {
      I(u).length === 0 ? B(o) : B(y, false);
    });
  }
  var O = se(x, 2);
  {
    var L = (B) => {
      var G = Bx(), j = ve(G, true);
      me(G), _e(() => Re(j, I(s))), ae(B, G);
    };
    Pe(O, (B) => {
      I(s) && B(L);
    });
  }
  ae(e, w), Lt();
}
var $x = Me('<div class="err"> </div>'), Hx = Me('<p> </p> <div class="flex gap-05"><!> <!></div> <!>', 1);
function Ux(e, t) {
  Mt(t, true);
  let a = jt(), s = ie(""), u = ie(false);
  async function p() {
    $(s, "");
    let j = await xl(`/auth/v1/sessions/${t.userId}`);
    j.error ? $(s, z(j.error.message)) : ($(u, true), setTimeout(() => {
      $(u, false);
    }, 3e3));
  }
  var m = Hx(), w = Le(m), x = ve(w, true);
  me(w);
  var o = se(w, 2), y = ve(o);
  At(y, { level: -1, onclick: p, children: (j, R) => {
    Ke();
    var h = He("Logout");
    ae(j, h);
  }, $$slots: { default: true } });
  var O = se(y, 2);
  {
    var L = (j) => {
      Yo(j, {});
    };
    Pe(O, (j) => {
      I(u) && j(L);
    });
  }
  me(o);
  var B = se(o, 2);
  {
    var G = (j) => {
      var R = $x(), h = ve(R, true);
      me(R), _e(() => Re(h, I(s))), ae(j, R);
    };
    Pe(B, (j) => {
      I(s) && j(G);
    });
  }
  _e(() => Re(x, a.users.forceLogout)), ae(e, m), Lt();
}
var Gx = Me('<div class="err"> </div>'), zx = Me("<p> </p> <!> <!>", 1);
function Wx(e, t) {
  Mt(t, true);
  let a = Yt(), s = jt(), u = ie("");
  async function p() {
    $(u, "");
    let L = await xl(`/auth/v1/users/${t.userId}`);
    L.error ? $(u, z(L.error.message)) : t.onSave();
  }
  var m = zx(), w = Le(m), x = ve(w, true);
  me(w);
  var o = se(w, 2);
  At(o, { level: -1, onclick: p, children: (L, B) => {
    Ke();
    var G = He();
    _e(() => Re(G, a.common.delete)), ae(L, G);
  }, $$slots: { default: true } });
  var y = se(o, 2);
  {
    var O = (L) => {
      var B = Gx(), G = ve(B, true);
      me(B), _e(() => Re(G, I(u))), ae(L, B);
    };
    Pe(y, (L) => {
      I(u) && L(O);
    });
  }
  _e(() => Re(x, s.users.deleteUser)), ae(e, m), Lt();
}
var Xx = Me('<div class="err"> </div>'), Kx = Me('<!> <div class="flex"><!></div> <!>', 1);
function Jx(e, t) {
  Mt(t, true);
  let a = Yt(), s = jt();
  const u = [a.account.navInfo, s.users.attributes, a.common.password, a.account.navMfa, a.account.devices, a.account.navLogout, a.common.delete];
  let p = ie(z(u[0])), m = ie(void 0), w = ie(""), x = ie(void 0);
  Et(() => {
    o();
  });
  async function o() {
    var _a3;
    let h = await Dt(`/auth/v1/users/${t.userId}`);
    h.body ? ($(x, z(h.body)), requestAnimationFrame(() => {
      var _a4;
      (_a4 = I(m)) == null ? void 0 : _a4();
    })) : $(w, z(((_a3 = h.error) == null ? void 0 : _a3.message) || "Error fetching user"));
  }
  var y = Kx(), O = Le(y);
  {
    var L = (h) => {
      var v = Xx(), g = ve(v, true);
      me(v), _e(() => Re(g, I(w))), ae(h, v);
    };
    Pe(O, (h) => {
      I(w) && h(L);
    });
  }
  var B = se(O, 2), G = ve(B);
  vv(G, { tabs: u, get selected() {
    return I(p);
  }, set selected(h) {
    $(p, z(h));
  }, get focusFirst() {
    return I(m);
  }, set focusFirst(h) {
    $(m, z(h));
  } }), me(B);
  var j = se(B, 2);
  {
    var R = (h) => {
      var v = nt(), g = Le(v);
      {
        var S = (_) => {
          Nv(_, { get roles() {
            return t.roles;
          }, get groups() {
            return t.groups;
          }, get onSave() {
            return t.onSave;
          }, get user() {
            return I(x);
          }, set user(k) {
            $(x, z(k));
          } });
        }, C = (_) => {
          var k = nt(), N = Le(k);
          {
            var W = (A) => {
              Ax(A, { get user() {
                return I(x);
              }, get onSave() {
                return t.onSave;
              } });
            }, ee = (A) => {
              var F = nt(), E = Le(F);
              {
                var M = (c) => {
                  Nx(c, { get user() {
                    return I(x);
                  }, get onSave() {
                    return t.onSave;
                  } });
                }, V = (c) => {
                  var r = nt(), i = Le(r);
                  {
                    var n = (f) => {
                      Vx(f, { get user() {
                        return I(x);
                      }, get onSave() {
                        return t.onSave;
                      } });
                    }, l = (f) => {
                      var b = nt(), P = Le(b);
                      {
                        var q = (H) => {
                          Cv(H, { get userId() {
                            return t.userId;
                          } });
                        }, T = (H) => {
                          var J = nt(), re = Le(J);
                          {
                            var Q = (pe) => {
                              Ux(pe, { get userId() {
                                return t.userId;
                              } });
                            }, de = (pe) => {
                              var fe = nt(), he = Le(fe);
                              {
                                var Ee = (Oe) => {
                                  Wx(Oe, { get userId() {
                                    return t.userId;
                                  }, get onSave() {
                                    return t.onSave;
                                  } });
                                };
                                Pe(he, (Oe) => {
                                  I(p) === u[6] && Oe(Ee);
                                }, true);
                              }
                              ae(pe, fe);
                            };
                            Pe(re, (pe) => {
                              I(p) === u[5] ? pe(Q) : pe(de, false);
                            }, true);
                          }
                          ae(H, J);
                        };
                        Pe(P, (H) => {
                          I(p) === u[4] ? H(q) : H(T, false);
                        }, true);
                      }
                      ae(f, b);
                    };
                    Pe(i, (f) => {
                      I(p) === u[3] ? f(n) : f(l, false);
                    }, true);
                  }
                  ae(c, r);
                };
                Pe(E, (c) => {
                  I(p) === u[2] ? c(M) : c(V, false);
                }, true);
              }
              ae(A, F);
            };
            Pe(N, (A) => {
              I(p) === u[1] ? A(W) : A(ee, false);
            }, true);
          }
          ae(_, k);
        };
        Pe(g, (_) => {
          I(p) === u[0] ? _(S) : _(C, false);
        });
      }
      ae(h, v);
    };
    Pe(j, (h) => {
      I(x) && h(R);
    });
  }
  ae(e, y), Lt();
}
var Qx = Me("<div></div> <!> <!>", 1), Yx = Me("<!> <!>", 1), Zx = Me('<div class="err"> </div>'), e_ = Me('<!> <div id="users"><!></div>', 1), t_ = Me("<!> <!>", 1);
function B_(e, t) {
  Mt(t, true);
  const a = (M, V = Nl, c = Nl) => {
    const r = bt(() => x.get() === V());
    pv(M, { onclick: () => x.set(V()), get selected() {
      return I(r);
    }, children: (i, n) => {
      Ke();
      var l = He();
      _e(() => Re(l, c())), ae(i, l);
    } });
  };
  let s = ie(void 0), u = ie(""), p = ie(z([])), m = ie(z([])), w = ie(z([])), x = dv("uid"), o = ie(void 0), y = ie(z([])), O = ie(z([])), L = ie(false), B = ie(void 0), G = ie(z(Pv)), j = ie(false), R = z(["E-Mail", "ID"]), h = ie(z(R[0])), v = ie(""), g = z(["E-Mail", "ID", "Created", "Last Login"]);
  gl(() => {
    C("page_size=" + I(G)), _(), k();
  }), Et(() => {
    var _a3;
    $(o, z((_a3 = I(p).find((M) => M.id === x.get())) == null ? void 0 : _a3.id));
  }), Et(() => {
    let M = I(v).toLowerCase();
    I(L) ? M.length < 3 ? I(j) && (C("page_size=" + I(G)), $(j, false)) : S(M) : M ? I(h) === R[0] ? $(m, z(I(p).filter((V) => {
      var _a3;
      return (_a3 = V.email) == null ? void 0 : _a3.toLowerCase().includes(M);
    }))) : I(h) === R[1] && $(m, z(I(p).filter((V) => V.id.toLowerCase().includes(M)))) : $(m, z(I(p)));
  });
  async function S(M) {
    $(B, void 0), $(j, true);
    let V;
    I(h) === R[0] ? V = "email" : V = "id";
    let c = await qv({ ty: "user", idx: V, q: M });
    c.body ? $(p, z(c.body)) : console.error(c.error);
  }
  async function C(M) {
    let V = "/auth/v1/users";
    M && (V += `?${M}`);
    let c = await Dt(V);
    return c.error ? $(u, "Error fetching users: " + c.error.message) : c.body && (c.status === 206 ? ($(L, true), $(B, z(c.headers))) : ($(L, false), $(B, void 0)), $(p, z(c.body))), [c.status, c.headers];
  }
  async function _() {
    var _a3;
    let M = await Dt("/auth/v1/roles");
    M.body ? $(O, z(M.body.toSorted((V, c) => V.name.localeCompare(c.name)))) : $(u, z(((_a3 = M.error) == null ? void 0 : _a3.message) || "Error"));
  }
  async function k() {
    var _a3;
    let M = await Dt("/auth/v1/groups");
    M.body ? $(y, z(M.body.toSorted((V, c) => V.name.localeCompare(c.name)))) : $(u, z(((_a3 = M.error) == null ? void 0 : _a3.message) || "Error"));
  }
  function N(M, V) {
    let c = V === "up";
    M === g[0] ? I(p).sort((r, i) => c ? r.email.localeCompare(i.email) : i.email.localeCompare(r.email)) : M === g[1] ? I(p).sort((r, i) => c ? r.id.localeCompare(i.id) : i.id.localeCompare(r.id)) : M === g[2] ? I(p).sort((r, i) => c ? r.created_at - i.created_at : i.created_at - r.created_at) : M === g[3] && I(p).sort((r, i) => {
      let n = r.last_login || 9999999999, l = r.last_login || 9999999999;
      return c ? n - l : l - n;
    });
  }
  async function W(M) {
    var _a3;
    (_a3 = I(s)) == null ? void 0 : _a3(), await C(), x.set(M);
  }
  function ee() {
    C(), _(), k(), $(v, "");
  }
  var A = t_(), F = Le(A);
  uv(F, { paddingTop: "2.1rem", buttonTilesAriaControls: "users", width: "min(23rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (V) => {
    var c = Qx(), r = Le(c);
    Xt(r, "height", ".5rem");
    var i = se(r, 2);
    {
      var n = (q) => {
        var T = nt(), H = Le(T);
        Ho(H, 17, () => I(p), (J) => J.id, (J, re) => {
          a(J, () => I(re).id, () => I(re).email);
        }), ae(q, T);
      }, l = (q) => {
        var T = nt(), H = Le(T);
        Ho(H, 17, () => I(w), (J) => J.id, (J, re) => {
          a(J, () => I(re).id, () => I(re).email);
        }), ae(q, T);
      };
      Pe(i, (q) => {
        I(B) ? q(n) : q(l, false);
      });
    }
    var f = se(i, 2);
    {
      var b = (q) => {
        Tv(q, { sspFetch: C, idxTotalCount: "x-user-count", get itemsLength() {
          return I(p).length;
        }, get firstFetchHeaders() {
          return I(B);
        }, get pageSize() {
          return I(G);
        }, set pageSize(T) {
          $(G, z(T));
        } });
      }, P = (q) => {
        var T = nt(), H = Le(T);
        {
          var J = (Q) => {
            Dl(Q, { get items() {
              return I(p);
            }, set items(de) {
              $(p, z(de));
            }, get itemsPaginated() {
              return I(w);
            }, set itemsPaginated(de) {
              $(w, z(de));
            } });
          }, re = (Q) => {
            Dl(Q, { get items() {
              return I(m);
            }, set items(de) {
              $(m, z(de));
            }, get itemsPaginated() {
              return I(w);
            }, set itemsPaginated(de) {
              $(w, z(de));
            } });
          };
          Pe(H, (Q) => {
            I(L) ? Q(J) : Q(re, false);
          }, true);
        }
        ae(q, T);
      };
      Pe(f, (q) => {
        I(B) ? q(b) : q(P, false);
      });
    }
    ae(V, c);
  }, children: (V, c) => {
    var r = Yx(), i = Le(r);
    const n = bt(() => I(O).length === 0 ? 1 : 2);
    cv(i, { get level() {
      return I(n);
    }, alignRight: true, get closeModal() {
      return I(s);
    }, set closeModal(f) {
      $(s, z(f));
    }, children: (f, b) => {
      kv(f, { onSave: W, get roles() {
        return I(O);
      }, get groups() {
        return I(y);
      } });
    }, $$slots: { default: true } });
    var l = se(i, 2);
    lv(l, { get searchOptions() {
      return R;
    }, get orderOptions() {
      return g;
    }, onChangeOrder: N, searchWidth: "min(23rem, calc(100dvw - .5rem))", get value() {
      return I(v);
    }, set value(f) {
      $(v, z(f));
    }, get searchOption() {
      return I(h);
    }, set searchOption(f) {
      $(h, z(f));
    } }), ae(V, r);
  }, $$slots: { buttonTiles: true, default: true } });
  var E = se(F, 2);
  fv(E, { children: (M, V) => {
    var c = e_(), r = Le(c);
    {
      var i = (b) => {
        var P = Zx(), q = ve(P, true);
        me(P), _e(() => Re(q, I(u))), ae(b, P);
      };
      Pe(r, (b) => {
        I(u) && b(i);
      });
    }
    var n = se(r, 2), l = ve(n);
    {
      var f = (b) => {
        Jx(b, { get userId() {
          return I(o);
        }, get roles() {
          return I(O);
        }, get groups() {
          return I(y);
        }, onSave: ee });
      };
      Pe(l, (b) => {
        I(o) && b(f);
      });
    }
    me(n), ae(M, c);
  } }), ae(e, A), Lt();
}
export {
  B_ as U
};
