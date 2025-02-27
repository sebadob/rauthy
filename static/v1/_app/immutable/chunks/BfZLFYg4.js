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
import { t as Ae, a as Y, e as $e, d as et } from "./D8nUqfqE.js";
import { p as At, k as ee, O as Dr, aa as wt, a as Et, f as Ee, l as E, j as x, s as te, t as _e, c as he, a7 as Ve, a9 as yt, r as pe, a5 as Xl, a8 as mu } from "./D-nwkJyM.js";
import { s as ye } from "./DmLjbmU6.js";
import { i as Se } from "./C3XMhfdI.js";
import { e as jr, i as md } from "./6MDunRKZ.js";
import { a as Yt, B as Rt, t as nc, s as oc } from "./BUAPoI3e.js";
import { p as M } from "./BiI21XkS.js";
import { b as lc, d as Jl, f as Lt, c as Ql } from "./emZtalxW.js";
import { O as vd } from "./TvawFHML.js";
import { P as vu } from "./PQEWmBlL.js";
import { N as bd } from "./B82wSkDx.js";
import { B as yd } from "./BB-qItao.js";
import { u as gd } from "./C6Ehfmfa.js";
import { N as xd } from "./MNfc1IKL.js";
import { C as _d } from "./C-Lg4I_M.js";
import { I as ot } from "./BE8gxf21.js";
import { b as $r, k as Cd, l as bu, m as Rd } from "./BRCxk8by.js";
import { F as Yl } from "./BgEKkuL8.js";
import { u as tr } from "./CMjKUQkH.js";
import { u as Ot } from "./DMJUG0wm.js";
import { O as uc } from "./Fz9KQU3M.js";
import { n as cc, l as ul, A as Pd } from "./DppGgfa0.js";
import { I as Lr } from "./CFT15Ou1.js";
import { I as Il } from "./RN-bweNU.js";
import { f as Qt, a as Or, b as dc } from "./DswDW5U8.js";
import { S as Fr } from "./BK_OTjGh.js";
import { L as Ft } from "./CvnJ-bH5.js";
import { T as wd } from "./C1ZebG9t.js";
import { p as Td } from "./2rFDT0Lm.js";
import { C as Sd } from "./BSpE3f0b.js";
import { I as Wr } from "./C0_UZAx3.js";
import { P as Ad } from "./Drrpav8g.js";
import { I as yu } from "./Bw4crv1X.js";
import { h as Ed } from "./1rF6JvjJ.js";
import { U as kd, D as Id } from "./B1UlKktF.js";
import { P as qd, f as Md, a as Ld } from "./ChQZXov_.js";
var rr = typeof globalThis < "u" ? globalThis : typeof window < "u" ? window : typeof global < "u" ? global : typeof self < "u" ? self : {}, Od = Ae("<div><!></div>"), Nd = Ae('<div class="err"> </div>'), Dd = Ae("<!> <!> <!> <!> <!> <!> <!> <!> <div></div> <!> <!>", 1), jd = Ae('<div class="container svelte-x7a3kj"><!></div>');
function $d(e, t) {
  At(t, true);
  let r = tr(), s = Ot(), o = ee(void 0), p = ee(""), h = ee(""), v = ee(""), m = ee(""), l = ee("en"), w = ee(M(Dr(() => t.roles.map((L) => ({ name: L.name, selected: false }))))), U = ee(M(Dr(() => t.groups.map((L) => ({ name: L.name, selected: false }))))), J = ee(false), z = ee(M(Qt())), K = ee(M(Or()));
  wt(() => {
    requestAnimationFrame(() => {
      var _a3;
      (_a3 = x(o)) == null ? void 0 : _a3.focus();
    });
  });
  async function Z(L, k) {
    var _a3, _b2;
    E(p, "");
    let O = { email: x(h), given_name: x(v), family_name: x(m) || void 0, language: x(l), groups: x(U).filter((D) => D.selected).map((D) => D.name), roles: x(w).filter((D) => D.selected).map((D) => D.name), user_expires: dc(x(z), x(K)) };
    ((_a3 = O.groups) == null ? void 0 : _a3.length) === 0 && (O.groups = void 0);
    let H = await lc(L.action, O);
    H.body ? t.onSave(H.body.id) : E(p, M(((_b2 = H.error) == null ? void 0 : _b2.message) || "Error"));
  }
  var B = jd(), $ = he(B);
  Yl($, { action: "/auth/v1/users", onSubmit: Z, children: (L, k) => {
    var O = Dd(), H = Ee(O);
    ot(H, { typ: "email", autocomplete: "off", label: "E-Mail", placeholder: "E-Mail", required: true, get ref() {
      return x(o);
    }, set ref(i) {
      E(o, M(i));
    }, get value() {
      return x(h);
    }, set value(i) {
      E(h, M(i));
    } });
    var D = te(H, 2);
    ot(D, { autocomplete: "off", get label() {
      return r.account.givenName;
    }, get placeholder() {
      return r.account.givenName;
    }, required: true, pattern: $r, get value() {
      return x(v);
    }, set value(i) {
      E(v, M(i));
    } });
    var ie = te(D, 2);
    ot(ie, { autocomplete: "off", get label() {
      return r.account.familyName;
    }, get placeholder() {
      return r.account.familyName;
    }, pattern: $r, get value() {
      return x(m);
    }, set value(i) {
      E(m, M(i));
    } });
    var se = te(ie, 2);
    Ft(se, { get label() {
      return s.common.language;
    }, children: (i, n) => {
      uc(i, { get ariaLabel() {
        return r.common.selectI18n;
      }, options: cc, borderless: true, get value() {
        return x(l);
      }, set value(u) {
        E(l, M(u));
      } });
    } });
    var fe = te(se, 2);
    Fr(fe, { get items() {
      return x(w);
    }, set items(i) {
      E(w, M(i));
    }, children: (i, n) => {
      Ve();
      var u = $e();
      _e(() => ye(u, r.account.roles)), Y(i, u);
    }, $$slots: { default: true } });
    var me = te(fe, 2);
    Fr(me, { get items() {
      return x(U);
    }, set items(i) {
      E(U, M(i));
    }, children: (i, n) => {
      Ve();
      var u = $e();
      _e(() => ye(u, r.account.groups)), Y(i, u);
    }, $$slots: { default: true } });
    var _ = te(me, 2);
    Lr(_, { get ariaLabel() {
      return r.account.accessExp;
    }, get checked() {
      return x(J);
    }, set checked(i) {
      E(J, M(i));
    }, children: (i, n) => {
      Ve();
      var u = $e();
      _e(() => ye(u, r.account.accessExp)), Y(i, u);
    }, $$slots: { default: true } });
    var S = te(_, 2);
    {
      var R = (i) => {
        var n = Od(), u = he(n);
        const f = yt(Qt);
        Il(u, { get label() {
          return r.account.accessExp;
        }, withTime: true, get min() {
          return x(f);
        }, required: true, get value() {
          return x(z);
        }, set value(b) {
          E(z, M(b));
        }, get timeValue() {
          return x(K);
        }, set timeValue(b) {
          E(K, M(b));
        } }), pe(n), nc(3, n, () => oc, () => ({ duration: 150 })), Y(i, n);
      };
      Se(S, (i) => {
        x(J) && i(R);
      });
    }
    var P = te(S, 2);
    Yt(P, "height", ".66rem");
    var A = te(P, 2);
    Rt(A, { type: "submit", children: (i, n) => {
      Ve();
      var u = $e();
      _e(() => ye(u, r.common.save)), Y(i, u);
    }, $$slots: { default: true } });
    var c = te(A, 2);
    {
      var a = (i) => {
        var n = Nd(), u = he(n, true);
        pe(n), _e(() => ye(u, x(p))), Y(i, n);
      };
      Se(c, (i) => {
        x(p) && i(a);
      });
    }
    Y(L, O);
  }, $$slots: { default: true } }), pe(B), Y(e, B), Et();
}
var Fd = Ae('<div class="svelte-k6dexs"><!></div>'), Bd = Ae('<div class="err"> </div>'), Vd = Ae('<!> <div class="values svelte-k6dexs"><div class="svelte-k6dexs"><!></div> <div class="svelte-k6dexs"><!></div></div> <div class="values svelte-k6dexs"><div class="svelte-k6dexs"><!> <!> <!> <!> <div><!></div></div> <div class="svelte-k6dexs"><!> <!> <!> <!> <!></div></div> <!> <!> <div class="values svelte-k6dexs"><div class="svelte-k6dexs"><!></div> <!></div> <div class="values svelte-k6dexs"><div class="svelte-k6dexs"><!> <!></div> <div class="svelte-k6dexs"><!> <!></div></div> <div class="flex gap-05"><!> <!></div> <!>', 1);
function Hd(e, t) {
  At(t, true);
  let r = Td(t, "user", 15), s = tr(), o = Ot(), p = ee(""), h = ee(false), v = ee(""), m = ee(""), l = ee(""), w = ee("en"), U = ee(""), J = ee(""), z = ee(""), K = ee(""), Z = ee(""), B = ee(""), $ = ee(false), L = ee(false), k = ee(false), O = ee(M(Qt())), H = ee(M(Or())), D = ee(M(Dr(() => t.roles.map((S) => ({ name: S.name, selected: false }))))), ie = ee(M(Dr(() => t.groups.map((S) => ({ name: S.name, selected: false })))));
  wt(() => {
    var _a3, _b2, _c2, _d2, _e3, _f2, _g2;
    if (r()) {
      if (E(v, M(r().email)), E(m, M(r().given_name)), E(l, M(r().family_name || "")), E(w, M(r().language)), E(U, M(((_a3 = r().user_values) == null ? void 0 : _a3.birthdate) || "")), E(J, M(((_b2 = r().user_values) == null ? void 0 : _b2.phone) || "")), E(z, M(((_c2 = r().user_values) == null ? void 0 : _c2.street) || "")), E(K, M(((_e3 = (_d2 = r().user_values) == null ? void 0 : _d2.zip) == null ? void 0 : _e3.toString()) || "")), E(Z, M(((_f2 = r().user_values) == null ? void 0 : _f2.city) || "")), E(B, M(((_g2 = r().user_values) == null ? void 0 : _g2.country) || "")), E($, M(r().enabled)), E(L, M(r().email_verified)), r().user_expires) {
        let S = new Date(r().user_expires * 1e3);
        E(k, true), E(O, M(Qt(S))), E(H, M(Or(S)));
      } else E(k, false), E(O, M(Qt())), E(H, M(Or()));
      E(D, M(t.roles.map((S) => {
        var _a4;
        return { name: S.name, selected: ((_a4 = r()) == null ? void 0 : _a4.roles.includes(S.name)) || false };
      }))), E(ie, M(t.roles.map((S) => {
        var _a4, _b3;
        return { name: S.name, selected: ((_b3 = (_a4 = r()) == null ? void 0 : _a4.groups) == null ? void 0 : _b3.includes(S.name)) || false };
      })));
    }
  });
  async function se(S, R) {
    var _a3, _b2;
    E(p, "");
    const P = { email: x(v), given_name: x(m), family_name: x(l) || void 0, language: x(w), roles: x(D).filter((c) => c.selected).map((c) => c.name), groups: x(ie).filter((c) => c.selected).map((c) => c.name), enabled: x($), email_verified: x(L), user_expires: x(k) ? dc(x(O), x(H)) : void 0 };
    (x(U) || x(J) || x(z) || x(K) || x(Z) || x(B)) && (console.log(x(U)), P.user_values = { birthdate: x(U) || void 0, phone: ((_a3 = x(J)) == null ? void 0 : _a3.replaceAll(" ", "")) || void 0, street: x(z) || void 0, zip: x(K) ? Number.parseInt(x(K)) : void 0, city: x(Z) || void 0, country: x(B) || void 0 });
    let A = await Jl(S.action, P);
    A.body ? (E(h, true), r(A.body), setTimeout(() => {
      E(h, false);
    }, 3e3)) : E(p, M(((_b2 = A.error) == null ? void 0 : _b2.message) || "Error"));
  }
  var fe = et(), me = Ee(fe);
  {
    var _ = (S) => {
      const R = yt(() => `/auth/v1/users/${r().id}`);
      Yl(S, { get action() {
        return x(R);
      }, onSubmit: se, children: (P, A) => {
        var c = Vd(), a = Ee(c);
        Ft(a, { label: "ID", mono: true, children: (X, Le) => {
          Ve();
          var ge = $e();
          _e(() => ye(ge, r().id)), Y(X, ge);
        } });
        var i = te(a, 2), n = he(i), u = he(n);
        Lr(u, { get ariaLabel() {
          return o.common.enabled;
        }, get checked() {
          return x($);
        }, set checked(X) {
          E($, M(X));
        }, children: (X, Le) => {
          Ve();
          var ge = $e();
          _e(() => ye(ge, o.common.enabled)), Y(X, ge);
        }, $$slots: { default: true } }), pe(n);
        var f = te(n, 2), b = he(f);
        Lr(b, { get ariaLabel() {
          return s.account.emailVerified;
        }, get checked() {
          return x(L);
        }, set checked(X) {
          E(L, M(X));
        }, children: (X, Le) => {
          Ve();
          var ge = $e();
          _e(() => ye(ge, s.account.emailVerified)), Y(X, ge);
        }, $$slots: { default: true } }), pe(f), pe(i);
        var y = te(i, 2), g = he(y), C = he(g);
        ot(C, { typ: "email", autocomplete: "off", label: "E-Mail", placeholder: "E-Mail", required: true, get value() {
          return x(v);
        }, set value(X) {
          E(v, M(X));
        } });
        var q = te(C, 2);
        ot(q, { autocomplete: "off", get label() {
          return s.account.givenName;
        }, get placeholder() {
          return s.account.givenName;
        }, required: true, pattern: $r, get value() {
          return x(m);
        }, set value(X) {
          E(m, M(X));
        } });
        var V = te(q, 2);
        ot(V, { autocomplete: "off", get label() {
          return s.account.familyName;
        }, get placeholder() {
          return s.account.familyName;
        }, pattern: $r, get value() {
          return x(l);
        }, set value(X) {
          E(l, M(X));
        } });
        var Q = te(V, 2);
        Il(Q, { get label() {
          return s.account.birthdate;
        }, withDelete: true, get value() {
          return x(U);
        }, set value(X) {
          E(U, M(X));
        } });
        var F = te(Q, 2);
        Yt(F, "padding", ".25rem");
        var ce = he(F);
        Ft(ce, { get label() {
          return o.common.language;
        }, children: (X, Le) => {
          uc(X, { get ariaLabel() {
            return s.common.selectI18n;
          }, options: cc, borderless: true, get value() {
            return x(w);
          }, set value(ge) {
            E(w, M(ge));
          } });
        } }), pe(F), pe(g);
        var ue = te(g, 2), de = he(ue);
        ot(de, { autocomplete: "off", get label() {
          return s.account.street;
        }, get placeholder() {
          return s.account.street;
        }, pattern: Cd, get value() {
          return x(z);
        }, set value(X) {
          E(z, M(X));
        } });
        var be = te(de, 2);
        ot(be, { typ: "number", autocomplete: "off", get label() {
          return s.account.zip;
        }, get placeholder() {
          return s.account.zip;
        }, min: "1000", max: "9999999", get value() {
          return x(K);
        }, set value(X) {
          E(K, M(X));
        } });
        var ke = te(be, 2);
        ot(ke, { autocomplete: "off", get label() {
          return s.account.city;
        }, get placeholder() {
          return s.account.city;
        }, pattern: bu, get value() {
          return x(Z);
        }, set value(X) {
          E(Z, M(X));
        } });
        var Oe = te(ke, 2);
        ot(Oe, { autocomplete: "off", get label() {
          return s.account.country;
        }, get placeholder() {
          return s.account.country;
        }, pattern: bu, get value() {
          return x(B);
        }, set value(X) {
          E(B, M(X));
        } });
        var qe = te(Oe, 2);
        ot(qe, { autocomplete: "off", get label() {
          return s.account.phone;
        }, get placeholder() {
          return s.account.phone;
        }, pattern: Rd, get value() {
          return x(J);
        }, set value(X) {
          E(J, M(X));
        } }), pe(ue), pe(y);
        var He = te(y, 2);
        Fr(He, { get items() {
          return x(D);
        }, set items(X) {
          E(D, M(X));
        }, children: (X, Le) => {
          Ve();
          var ge = $e();
          _e(() => ye(ge, s.account.roles)), Y(X, ge);
        }, $$slots: { default: true } });
        var Ne = te(He, 2);
        Fr(Ne, { get items() {
          return x(ie);
        }, set items(X) {
          E(ie, M(X));
        }, children: (X, Le) => {
          Ve();
          var ge = $e();
          _e(() => ye(ge, s.account.groups)), Y(X, ge);
        }, $$slots: { default: true } });
        var it = te(Ne, 2), Je = he(it);
        Yt(Je, "margin-top", ".5rem");
        var Dt = he(Je);
        Lr(Dt, { get ariaLabel() {
          return s.account.accessExp;
        }, get checked() {
          return x(k);
        }, set checked(X) {
          E(k, M(X));
        }, children: (X, Le) => {
          Ve();
          var ge = $e();
          _e(() => ye(ge, s.account.accessExp)), Y(X, ge);
        }, $$slots: { default: true } }), pe(Je);
        var jt = te(Je, 2);
        {
          var T = (X) => {
            var Le = Fd(), ge = he(Le);
            const Qe = yt(Qt);
            Il(ge, { get label() {
              return s.account.accessExp;
            }, withTime: true, get min() {
              return x(Qe);
            }, required: true, get value() {
              return x(O);
            }, set value(ht) {
              E(O, M(ht));
            }, get timeValue() {
              return x(H);
            }, set timeValue(ht) {
              E(H, M(ht));
            } }), pe(Le), nc(3, Le, () => oc, () => ({ duration: 150 })), Y(X, Le);
          };
          Se(jt, (X) => {
            x(k) && X(T);
          });
        }
        pe(it);
        var N = te(it, 2), j = he(N), G = he(j);
        Ft(G, { get label() {
          return s.account.userCreated;
        }, children: (X, Le) => {
          Ve();
          var ge = $e();
          _e((Qe) => ye(ge, Qe), [() => ul(r().created_at)]), Y(X, ge);
        } });
        var le = te(G, 2);
        Ft(le, { get label() {
          return o.users.lastLogin;
        }, children: (X, Le) => {
          var ge = et(), Qe = Ee(ge);
          {
            var ht = (Ge) => {
              var Ye = $e();
              _e((ur) => ye(Ye, ur), [() => ul(r().last_login)]), Y(Ge, Ye);
            }, We = (Ge) => {
              var Ye = $e();
              _e(() => ye(Ye, s.common.never)), Y(Ge, Ye);
            };
            Se(Qe, (Ge) => {
              r().last_login ? Ge(ht) : Ge(We, false);
            });
          }
          Y(X, ge);
        } }), pe(j);
        var ae = te(j, 2), Re = he(ae);
        Ft(Re, { get label() {
          return s.account.passwordExpiry;
        }, children: (X, Le) => {
          var ge = et(), Qe = Ee(ge);
          {
            var ht = (Ge) => {
              var Ye = $e();
              _e((ur) => ye(Ye, ur), [() => ul(r().password_expires)]), Y(Ge, Ye);
            }, We = (Ge) => {
              var Ye = $e();
              _e(() => ye(Ye, s.common.never)), Y(Ge, Ye);
            };
            Se(Qe, (Ge) => {
              r().password_expires ? Ge(ht) : Ge(We, false);
            });
          }
          Y(X, ge);
        } });
        var Me = te(Re, 2);
        Ft(Me, { get label() {
          return s.account.mfaActivated;
        }, children: (X, Le) => {
          Sd(X, {});
        } }), pe(ae), pe(N);
        var xe = te(N, 2), Ie = he(xe);
        Rt(Ie, { type: "submit", children: (X, Le) => {
          Ve();
          var ge = $e();
          _e(() => ye(ge, s.common.save)), Y(X, ge);
        }, $$slots: { default: true } });
        var ze = te(Ie, 2);
        {
          var Fe = (X) => {
            Wr(X, {});
          };
          Se(ze, (X) => {
            x(h) && X(Fe);
          });
        }
        pe(xe);
        var je = te(xe, 2);
        {
          var Ue = (X) => {
            var Le = Bd(), ge = he(Le, true);
            pe(Le), _e(() => ye(ge, x(p))), Y(X, Le);
          };
          Se(je, (X) => {
            x(p) && X(Ue);
          });
        }
        Y(P, c);
      }, $$slots: { default: true } });
    };
    Se(me, (S) => {
      r() && S(_);
    });
  }
  Y(e, fe), Et();
}
function Ud(e, t) {
  return e.start <= t && t < e.end;
}
function Gd(e, t = {}) {
  const { offsetLine: r = 0, offsetColumn: s = 0 } = t;
  let o = 0;
  const p = e.split(`
`).map((m, l) => {
    const w = o + m.length + 1, U = { start: o, end: w, line: l };
    return o = w, U;
  });
  let h = 0;
  function v(m, l) {
    if (typeof m == "string" && (m = e.indexOf(m, l ?? 0)), m === -1) return;
    let w = p[h];
    const U = m >= w.end ? 1 : -1;
    for (; w; ) {
      if (Ud(w, m)) return { line: r + w.line, column: s + m - w.start, character: m };
      h += U, w = p[h];
    }
  }
  return v;
}
let zd, gu = Gd("", { offsetLine: 1 }), Wd, xu = [], Kd = /* @__PURE__ */ new Map();
const Xd = /^\t+/;
function cl(e) {
  return e.replace(Xd, (t) => t.split("	").join("  "));
}
function Jd(e, t, r) {
  const s = e.split(`
`), o = Math.max(0, t - 2), p = Math.min(t + 3, s.length), h = String(p + 1).length;
  return s.slice(o, p).map((v, m) => {
    const l = o + m === t, w = String(m + o + 1).padStart(h, " ");
    if (l) {
      const U = " ".repeat(h + 2 + cl(v.slice(0, r)).length) + "^";
      return `${w}: ${cl(v)}
${U}`;
    }
    return `${w}: ${cl(v)}`;
  }).join(`
`);
}
class pc {
  constructor(t, r, s) {
    __publicField(this, "name", "CompileDiagnostic");
    this.code = t, this.message = r, s && (this.position = s, this.start = gu(s[0]), this.end = gu(s[1]), this.start && this.end && (this.frame = Jd(zd, this.start.line - 1, this.end.column)));
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
class Qd extends pc {
  constructor(t, r, s) {
    super(t, r, s);
    __publicField(this, "name", "CompileWarning");
  }
}
function ar(e, t, r) {
  var _a3;
  let s = xu;
  if (e && (s = Kd.get(e) ?? xu), s && ((_a3 = s.at(-1)) == null ? void 0 : _a3.has(t))) return;
  const o = new Qd(t, r, e && e.start !== void 0 ? [e.start, e.end ?? e.start] : void 0);
  Wd(o);
}
function Yd(e) {
  ar(e, "options_deprecated_accessors", "The `accessors` option has been deprecated. It will have no effect in runes mode\nhttps://svelte.dev/e/options_deprecated_accessors");
}
function Zd(e) {
  ar(e, "options_deprecated_immutable", "The `immutable` option has been deprecated. It will have no effect in runes mode\nhttps://svelte.dev/e/options_deprecated_immutable");
}
function ep(e) {
  ar(e, "options_removed_enable_sourcemap", "The `enableSourcemap` option has been removed. Source maps are always generated now, and tooling can choose to ignore them\nhttps://svelte.dev/e/options_removed_enable_sourcemap");
}
function tp(e) {
  ar(e, "options_removed_hydratable", "The `hydratable` option has been removed. Svelte components are always hydratable now\nhttps://svelte.dev/e/options_removed_hydratable");
}
function rp(e) {
  ar(e, "options_removed_loop_guard_timeout", "The `loopGuardTimeout` option has been removed\nhttps://svelte.dev/e/options_removed_loop_guard_timeout");
}
function ap(e) {
  ar(e, "options_renamed_ssr_dom", '`generate: "dom"` and `generate: "ssr"` options have been renamed to "client" and "server" respectively\nhttps://svelte.dev/e/options_renamed_ssr_dom');
}
var ip = [509, 0, 227, 0, 150, 4, 294, 9, 1368, 2, 2, 1, 6, 3, 41, 2, 5, 0, 166, 1, 574, 3, 9, 9, 7, 9, 32, 4, 318, 1, 80, 3, 71, 10, 50, 3, 123, 2, 54, 14, 32, 10, 3, 1, 11, 3, 46, 10, 8, 0, 46, 9, 7, 2, 37, 13, 2, 9, 6, 1, 45, 0, 13, 2, 49, 13, 9, 3, 2, 11, 83, 11, 7, 0, 3, 0, 158, 11, 6, 9, 7, 3, 56, 1, 2, 6, 3, 1, 3, 2, 10, 0, 11, 1, 3, 6, 4, 4, 68, 8, 2, 0, 3, 0, 2, 3, 2, 4, 2, 0, 15, 1, 83, 17, 10, 9, 5, 0, 82, 19, 13, 9, 214, 6, 3, 8, 28, 1, 83, 16, 16, 9, 82, 12, 9, 9, 7, 19, 58, 14, 5, 9, 243, 14, 166, 9, 71, 5, 2, 1, 3, 3, 2, 0, 2, 1, 13, 9, 120, 6, 3, 6, 4, 0, 29, 9, 41, 6, 2, 3, 9, 0, 10, 10, 47, 15, 343, 9, 54, 7, 2, 7, 17, 9, 57, 21, 2, 13, 123, 5, 4, 0, 2, 1, 2, 6, 2, 0, 9, 9, 49, 4, 2, 1, 2, 4, 9, 9, 330, 3, 10, 1, 2, 0, 49, 6, 4, 4, 14, 10, 5350, 0, 7, 14, 11465, 27, 2343, 9, 87, 9, 39, 4, 60, 6, 26, 9, 535, 9, 470, 0, 2, 54, 8, 3, 82, 0, 12, 1, 19628, 1, 4178, 9, 519, 45, 3, 22, 543, 4, 4, 5, 9, 7, 3, 6, 31, 3, 149, 2, 1418, 49, 513, 54, 5, 49, 9, 0, 15, 0, 23, 4, 2, 14, 1361, 6, 2, 16, 3, 6, 2, 1, 2, 4, 101, 0, 161, 6, 10, 9, 357, 0, 62, 13, 499, 13, 245, 1, 2, 9, 726, 6, 110, 6, 6, 9, 4759, 9, 787719, 239], hc = [0, 11, 2, 25, 2, 18, 2, 1, 2, 14, 3, 13, 35, 122, 70, 52, 268, 28, 4, 48, 48, 31, 14, 29, 6, 37, 11, 29, 3, 35, 5, 7, 2, 4, 43, 157, 19, 35, 5, 35, 5, 39, 9, 51, 13, 10, 2, 14, 2, 6, 2, 1, 2, 10, 2, 14, 2, 6, 2, 1, 4, 51, 13, 310, 10, 21, 11, 7, 25, 5, 2, 41, 2, 8, 70, 5, 3, 0, 2, 43, 2, 1, 4, 0, 3, 22, 11, 22, 10, 30, 66, 18, 2, 1, 11, 21, 11, 25, 71, 55, 7, 1, 65, 0, 16, 3, 2, 2, 2, 28, 43, 28, 4, 28, 36, 7, 2, 27, 28, 53, 11, 21, 11, 18, 14, 17, 111, 72, 56, 50, 14, 50, 14, 35, 39, 27, 10, 22, 251, 41, 7, 1, 17, 2, 60, 28, 11, 0, 9, 21, 43, 17, 47, 20, 28, 22, 13, 52, 58, 1, 3, 0, 14, 44, 33, 24, 27, 35, 30, 0, 3, 0, 9, 34, 4, 0, 13, 47, 15, 3, 22, 0, 2, 0, 36, 17, 2, 24, 20, 1, 64, 6, 2, 0, 2, 3, 2, 14, 2, 9, 8, 46, 39, 7, 3, 1, 3, 21, 2, 6, 2, 1, 2, 4, 4, 0, 19, 0, 13, 4, 31, 9, 2, 0, 3, 0, 2, 37, 2, 0, 26, 0, 2, 0, 45, 52, 19, 3, 21, 2, 31, 47, 21, 1, 2, 0, 185, 46, 42, 3, 37, 47, 21, 0, 60, 42, 14, 0, 72, 26, 38, 6, 186, 43, 117, 63, 32, 7, 3, 0, 3, 7, 2, 1, 2, 23, 16, 0, 2, 0, 95, 7, 3, 38, 17, 0, 2, 0, 29, 0, 11, 39, 8, 0, 22, 0, 12, 45, 20, 0, 19, 72, 200, 32, 32, 8, 2, 36, 18, 0, 50, 29, 113, 6, 2, 1, 2, 37, 22, 0, 26, 5, 2, 1, 2, 31, 15, 0, 328, 18, 16, 0, 2, 12, 2, 33, 125, 0, 80, 921, 103, 110, 18, 195, 2637, 96, 16, 1071, 18, 5, 26, 3994, 6, 582, 6842, 29, 1763, 568, 8, 30, 18, 78, 18, 29, 19, 47, 17, 3, 32, 20, 6, 18, 433, 44, 212, 63, 129, 74, 6, 0, 67, 12, 65, 1, 2, 0, 29, 6135, 9, 1237, 42, 9, 8936, 3, 2, 6, 2, 1, 2, 290, 16, 0, 30, 2, 3, 0, 15, 3, 9, 395, 2309, 106, 6, 12, 4, 8, 8, 9, 5991, 84, 2, 70, 2, 1, 3, 0, 3, 1, 3, 3, 2, 11, 2, 0, 2, 6, 2, 64, 2, 3, 3, 7, 2, 6, 2, 27, 2, 3, 2, 4, 2, 0, 4, 6, 2, 339, 3, 24, 2, 24, 2, 30, 2, 24, 2, 30, 2, 24, 2, 30, 2, 24, 2, 30, 2, 24, 2, 7, 1845, 30, 7, 5, 262, 61, 147, 44, 11, 6, 17, 0, 322, 29, 19, 43, 485, 27, 229, 29, 3, 0, 496, 6, 2, 3, 2, 1, 2, 14, 2, 196, 60, 67, 8, 0, 1205, 3, 2, 26, 2, 1, 2, 0, 3, 0, 2, 9, 2, 3, 2, 0, 2, 0, 7, 0, 5, 0, 2, 0, 2, 0, 2, 2, 2, 1, 2, 0, 3, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 1, 2, 0, 3, 3, 2, 6, 2, 3, 2, 3, 2, 0, 2, 9, 2, 16, 6, 2, 2, 4, 2, 16, 4421, 42719, 33, 4153, 7, 221, 3, 5761, 15, 7472, 16, 621, 2467, 541, 1507, 4938, 6, 4191], sp = "\u200C\u200D\xB7\u0300-\u036F\u0387\u0483-\u0487\u0591-\u05BD\u05BF\u05C1\u05C2\u05C4\u05C5\u05C7\u0610-\u061A\u064B-\u0669\u0670\u06D6-\u06DC\u06DF-\u06E4\u06E7\u06E8\u06EA-\u06ED\u06F0-\u06F9\u0711\u0730-\u074A\u07A6-\u07B0\u07C0-\u07C9\u07EB-\u07F3\u07FD\u0816-\u0819\u081B-\u0823\u0825-\u0827\u0829-\u082D\u0859-\u085B\u0897-\u089F\u08CA-\u08E1\u08E3-\u0903\u093A-\u093C\u093E-\u094F\u0951-\u0957\u0962\u0963\u0966-\u096F\u0981-\u0983\u09BC\u09BE-\u09C4\u09C7\u09C8\u09CB-\u09CD\u09D7\u09E2\u09E3\u09E6-\u09EF\u09FE\u0A01-\u0A03\u0A3C\u0A3E-\u0A42\u0A47\u0A48\u0A4B-\u0A4D\u0A51\u0A66-\u0A71\u0A75\u0A81-\u0A83\u0ABC\u0ABE-\u0AC5\u0AC7-\u0AC9\u0ACB-\u0ACD\u0AE2\u0AE3\u0AE6-\u0AEF\u0AFA-\u0AFF\u0B01-\u0B03\u0B3C\u0B3E-\u0B44\u0B47\u0B48\u0B4B-\u0B4D\u0B55-\u0B57\u0B62\u0B63\u0B66-\u0B6F\u0B82\u0BBE-\u0BC2\u0BC6-\u0BC8\u0BCA-\u0BCD\u0BD7\u0BE6-\u0BEF\u0C00-\u0C04\u0C3C\u0C3E-\u0C44\u0C46-\u0C48\u0C4A-\u0C4D\u0C55\u0C56\u0C62\u0C63\u0C66-\u0C6F\u0C81-\u0C83\u0CBC\u0CBE-\u0CC4\u0CC6-\u0CC8\u0CCA-\u0CCD\u0CD5\u0CD6\u0CE2\u0CE3\u0CE6-\u0CEF\u0CF3\u0D00-\u0D03\u0D3B\u0D3C\u0D3E-\u0D44\u0D46-\u0D48\u0D4A-\u0D4D\u0D57\u0D62\u0D63\u0D66-\u0D6F\u0D81-\u0D83\u0DCA\u0DCF-\u0DD4\u0DD6\u0DD8-\u0DDF\u0DE6-\u0DEF\u0DF2\u0DF3\u0E31\u0E34-\u0E3A\u0E47-\u0E4E\u0E50-\u0E59\u0EB1\u0EB4-\u0EBC\u0EC8-\u0ECE\u0ED0-\u0ED9\u0F18\u0F19\u0F20-\u0F29\u0F35\u0F37\u0F39\u0F3E\u0F3F\u0F71-\u0F84\u0F86\u0F87\u0F8D-\u0F97\u0F99-\u0FBC\u0FC6\u102B-\u103E\u1040-\u1049\u1056-\u1059\u105E-\u1060\u1062-\u1064\u1067-\u106D\u1071-\u1074\u1082-\u108D\u108F-\u109D\u135D-\u135F\u1369-\u1371\u1712-\u1715\u1732-\u1734\u1752\u1753\u1772\u1773\u17B4-\u17D3\u17DD\u17E0-\u17E9\u180B-\u180D\u180F-\u1819\u18A9\u1920-\u192B\u1930-\u193B\u1946-\u194F\u19D0-\u19DA\u1A17-\u1A1B\u1A55-\u1A5E\u1A60-\u1A7C\u1A7F-\u1A89\u1A90-\u1A99\u1AB0-\u1ABD\u1ABF-\u1ACE\u1B00-\u1B04\u1B34-\u1B44\u1B50-\u1B59\u1B6B-\u1B73\u1B80-\u1B82\u1BA1-\u1BAD\u1BB0-\u1BB9\u1BE6-\u1BF3\u1C24-\u1C37\u1C40-\u1C49\u1C50-\u1C59\u1CD0-\u1CD2\u1CD4-\u1CE8\u1CED\u1CF4\u1CF7-\u1CF9\u1DC0-\u1DFF\u200C\u200D\u203F\u2040\u2054\u20D0-\u20DC\u20E1\u20E5-\u20F0\u2CEF-\u2CF1\u2D7F\u2DE0-\u2DFF\u302A-\u302F\u3099\u309A\u30FB\uA620-\uA629\uA66F\uA674-\uA67D\uA69E\uA69F\uA6F0\uA6F1\uA802\uA806\uA80B\uA823-\uA827\uA82C\uA880\uA881\uA8B4-\uA8C5\uA8D0-\uA8D9\uA8E0-\uA8F1\uA8FF-\uA909\uA926-\uA92D\uA947-\uA953\uA980-\uA983\uA9B3-\uA9C0\uA9D0-\uA9D9\uA9E5\uA9F0-\uA9F9\uAA29-\uAA36\uAA43\uAA4C\uAA4D\uAA50-\uAA59\uAA7B-\uAA7D\uAAB0\uAAB2-\uAAB4\uAAB7\uAAB8\uAABE\uAABF\uAAC1\uAAEB-\uAAEF\uAAF5\uAAF6\uABE3-\uABEA\uABEC\uABED\uABF0-\uABF9\uFB1E\uFE00-\uFE0F\uFE20-\uFE2F\uFE33\uFE34\uFE4D-\uFE4F\uFF10-\uFF19\uFF3F\uFF65", fc = "\xAA\xB5\xBA\xC0-\xD6\xD8-\xF6\xF8-\u02C1\u02C6-\u02D1\u02E0-\u02E4\u02EC\u02EE\u0370-\u0374\u0376\u0377\u037A-\u037D\u037F\u0386\u0388-\u038A\u038C\u038E-\u03A1\u03A3-\u03F5\u03F7-\u0481\u048A-\u052F\u0531-\u0556\u0559\u0560-\u0588\u05D0-\u05EA\u05EF-\u05F2\u0620-\u064A\u066E\u066F\u0671-\u06D3\u06D5\u06E5\u06E6\u06EE\u06EF\u06FA-\u06FC\u06FF\u0710\u0712-\u072F\u074D-\u07A5\u07B1\u07CA-\u07EA\u07F4\u07F5\u07FA\u0800-\u0815\u081A\u0824\u0828\u0840-\u0858\u0860-\u086A\u0870-\u0887\u0889-\u088E\u08A0-\u08C9\u0904-\u0939\u093D\u0950\u0958-\u0961\u0971-\u0980\u0985-\u098C\u098F\u0990\u0993-\u09A8\u09AA-\u09B0\u09B2\u09B6-\u09B9\u09BD\u09CE\u09DC\u09DD\u09DF-\u09E1\u09F0\u09F1\u09FC\u0A05-\u0A0A\u0A0F\u0A10\u0A13-\u0A28\u0A2A-\u0A30\u0A32\u0A33\u0A35\u0A36\u0A38\u0A39\u0A59-\u0A5C\u0A5E\u0A72-\u0A74\u0A85-\u0A8D\u0A8F-\u0A91\u0A93-\u0AA8\u0AAA-\u0AB0\u0AB2\u0AB3\u0AB5-\u0AB9\u0ABD\u0AD0\u0AE0\u0AE1\u0AF9\u0B05-\u0B0C\u0B0F\u0B10\u0B13-\u0B28\u0B2A-\u0B30\u0B32\u0B33\u0B35-\u0B39\u0B3D\u0B5C\u0B5D\u0B5F-\u0B61\u0B71\u0B83\u0B85-\u0B8A\u0B8E-\u0B90\u0B92-\u0B95\u0B99\u0B9A\u0B9C\u0B9E\u0B9F\u0BA3\u0BA4\u0BA8-\u0BAA\u0BAE-\u0BB9\u0BD0\u0C05-\u0C0C\u0C0E-\u0C10\u0C12-\u0C28\u0C2A-\u0C39\u0C3D\u0C58-\u0C5A\u0C5D\u0C60\u0C61\u0C80\u0C85-\u0C8C\u0C8E-\u0C90\u0C92-\u0CA8\u0CAA-\u0CB3\u0CB5-\u0CB9\u0CBD\u0CDD\u0CDE\u0CE0\u0CE1\u0CF1\u0CF2\u0D04-\u0D0C\u0D0E-\u0D10\u0D12-\u0D3A\u0D3D\u0D4E\u0D54-\u0D56\u0D5F-\u0D61\u0D7A-\u0D7F\u0D85-\u0D96\u0D9A-\u0DB1\u0DB3-\u0DBB\u0DBD\u0DC0-\u0DC6\u0E01-\u0E30\u0E32\u0E33\u0E40-\u0E46\u0E81\u0E82\u0E84\u0E86-\u0E8A\u0E8C-\u0EA3\u0EA5\u0EA7-\u0EB0\u0EB2\u0EB3\u0EBD\u0EC0-\u0EC4\u0EC6\u0EDC-\u0EDF\u0F00\u0F40-\u0F47\u0F49-\u0F6C\u0F88-\u0F8C\u1000-\u102A\u103F\u1050-\u1055\u105A-\u105D\u1061\u1065\u1066\u106E-\u1070\u1075-\u1081\u108E\u10A0-\u10C5\u10C7\u10CD\u10D0-\u10FA\u10FC-\u1248\u124A-\u124D\u1250-\u1256\u1258\u125A-\u125D\u1260-\u1288\u128A-\u128D\u1290-\u12B0\u12B2-\u12B5\u12B8-\u12BE\u12C0\u12C2-\u12C5\u12C8-\u12D6\u12D8-\u1310\u1312-\u1315\u1318-\u135A\u1380-\u138F\u13A0-\u13F5\u13F8-\u13FD\u1401-\u166C\u166F-\u167F\u1681-\u169A\u16A0-\u16EA\u16EE-\u16F8\u1700-\u1711\u171F-\u1731\u1740-\u1751\u1760-\u176C\u176E-\u1770\u1780-\u17B3\u17D7\u17DC\u1820-\u1878\u1880-\u18A8\u18AA\u18B0-\u18F5\u1900-\u191E\u1950-\u196D\u1970-\u1974\u1980-\u19AB\u19B0-\u19C9\u1A00-\u1A16\u1A20-\u1A54\u1AA7\u1B05-\u1B33\u1B45-\u1B4C\u1B83-\u1BA0\u1BAE\u1BAF\u1BBA-\u1BE5\u1C00-\u1C23\u1C4D-\u1C4F\u1C5A-\u1C7D\u1C80-\u1C8A\u1C90-\u1CBA\u1CBD-\u1CBF\u1CE9-\u1CEC\u1CEE-\u1CF3\u1CF5\u1CF6\u1CFA\u1D00-\u1DBF\u1E00-\u1F15\u1F18-\u1F1D\u1F20-\u1F45\u1F48-\u1F4D\u1F50-\u1F57\u1F59\u1F5B\u1F5D\u1F5F-\u1F7D\u1F80-\u1FB4\u1FB6-\u1FBC\u1FBE\u1FC2-\u1FC4\u1FC6-\u1FCC\u1FD0-\u1FD3\u1FD6-\u1FDB\u1FE0-\u1FEC\u1FF2-\u1FF4\u1FF6-\u1FFC\u2071\u207F\u2090-\u209C\u2102\u2107\u210A-\u2113\u2115\u2118-\u211D\u2124\u2126\u2128\u212A-\u2139\u213C-\u213F\u2145-\u2149\u214E\u2160-\u2188\u2C00-\u2CE4\u2CEB-\u2CEE\u2CF2\u2CF3\u2D00-\u2D25\u2D27\u2D2D\u2D30-\u2D67\u2D6F\u2D80-\u2D96\u2DA0-\u2DA6\u2DA8-\u2DAE\u2DB0-\u2DB6\u2DB8-\u2DBE\u2DC0-\u2DC6\u2DC8-\u2DCE\u2DD0-\u2DD6\u2DD8-\u2DDE\u3005-\u3007\u3021-\u3029\u3031-\u3035\u3038-\u303C\u3041-\u3096\u309B-\u309F\u30A1-\u30FA\u30FC-\u30FF\u3105-\u312F\u3131-\u318E\u31A0-\u31BF\u31F0-\u31FF\u3400-\u4DBF\u4E00-\uA48C\uA4D0-\uA4FD\uA500-\uA60C\uA610-\uA61F\uA62A\uA62B\uA640-\uA66E\uA67F-\uA69D\uA6A0-\uA6EF\uA717-\uA71F\uA722-\uA788\uA78B-\uA7CD\uA7D0\uA7D1\uA7D3\uA7D5-\uA7DC\uA7F2-\uA801\uA803-\uA805\uA807-\uA80A\uA80C-\uA822\uA840-\uA873\uA882-\uA8B3\uA8F2-\uA8F7\uA8FB\uA8FD\uA8FE\uA90A-\uA925\uA930-\uA946\uA960-\uA97C\uA984-\uA9B2\uA9CF\uA9E0-\uA9E4\uA9E6-\uA9EF\uA9FA-\uA9FE\uAA00-\uAA28\uAA40-\uAA42\uAA44-\uAA4B\uAA60-\uAA76\uAA7A\uAA7E-\uAAAF\uAAB1\uAAB5\uAAB6\uAAB9-\uAABD\uAAC0\uAAC2\uAADB-\uAADD\uAAE0-\uAAEA\uAAF2-\uAAF4\uAB01-\uAB06\uAB09-\uAB0E\uAB11-\uAB16\uAB20-\uAB26\uAB28-\uAB2E\uAB30-\uAB5A\uAB5C-\uAB69\uAB70-\uABE2\uAC00-\uD7A3\uD7B0-\uD7C6\uD7CB-\uD7FB\uF900-\uFA6D\uFA70-\uFAD9\uFB00-\uFB06\uFB13-\uFB17\uFB1D\uFB1F-\uFB28\uFB2A-\uFB36\uFB38-\uFB3C\uFB3E\uFB40\uFB41\uFB43\uFB44\uFB46-\uFBB1\uFBD3-\uFD3D\uFD50-\uFD8F\uFD92-\uFDC7\uFDF0-\uFDFB\uFE70-\uFE74\uFE76-\uFEFC\uFF21-\uFF3A\uFF41-\uFF5A\uFF66-\uFFBE\uFFC2-\uFFC7\uFFCA-\uFFCF\uFFD2-\uFFD7\uFFDA-\uFFDC", dl = { 3: "abstract boolean byte char class double enum export extends final float goto implements import int interface long native package private protected public short static super synchronized throws transient volatile", 5: "class enum extends super const export import", 6: "enum", strict: "implements interface let package private protected public static yield", strictBind: "eval arguments" }, pl = "break case catch continue debugger default do else finally for function if return switch throw try var while with null true false instanceof typeof void delete new in this", np = { 5: pl, "5module": pl + " export import", 6: pl + " const class extends export import super" }, op = /^in(stanceof)?$/, lp = new RegExp("[" + fc + "]"), up = new RegExp("[" + fc + sp + "]");
function ql(e, t) {
  for (var r = 65536, s = 0; s < t.length; s += 2) {
    if (r += t[s], r > e) return false;
    if (r += t[s + 1], r >= e) return true;
  }
  return false;
}
function xt(e, t) {
  return e < 65 ? e === 36 : e < 91 ? true : e < 97 ? e === 95 : e < 123 ? true : e <= 65535 ? e >= 170 && lp.test(String.fromCharCode(e)) : t === false ? false : ql(e, hc);
}
function Vt(e, t) {
  return e < 48 ? e === 36 : e < 58 ? true : e < 65 ? false : e < 91 ? true : e < 97 ? e === 95 : e < 123 ? true : e <= 65535 ? e >= 170 && up.test(String.fromCharCode(e)) : t === false ? false : ql(e, hc) || ql(e, ip);
}
var Pe = function(t, r) {
  r === void 0 && (r = {}), this.label = t, this.keyword = r.keyword, this.beforeExpr = !!r.beforeExpr, this.startsExpr = !!r.startsExpr, this.isLoop = !!r.isLoop, this.isAssign = !!r.isAssign, this.prefix = !!r.prefix, this.postfix = !!r.postfix, this.binop = r.binop || null, this.updateContext = null;
};
function st(e, t) {
  return new Pe(e, { beforeExpr: true, binop: t });
}
var nt = { beforeExpr: true }, Ze = { startsExpr: true }, Zt = {};
function Te(e, t) {
  return t === void 0 && (t = {}), t.keyword = e, Zt[e] = new Pe(e, t);
}
var d = { num: new Pe("num", Ze), regexp: new Pe("regexp", Ze), string: new Pe("string", Ze), name: new Pe("name", Ze), privateId: new Pe("privateId", Ze), eof: new Pe("eof"), bracketL: new Pe("[", { beforeExpr: true, startsExpr: true }), bracketR: new Pe("]"), braceL: new Pe("{", { beforeExpr: true, startsExpr: true }), braceR: new Pe("}"), parenL: new Pe("(", { beforeExpr: true, startsExpr: true }), parenR: new Pe(")"), comma: new Pe(",", nt), semi: new Pe(";", nt), colon: new Pe(":", nt), dot: new Pe("."), question: new Pe("?", nt), questionDot: new Pe("?."), arrow: new Pe("=>", nt), template: new Pe("template"), invalidTemplate: new Pe("invalidTemplate"), ellipsis: new Pe("...", nt), backQuote: new Pe("`", Ze), dollarBraceL: new Pe("${", { beforeExpr: true, startsExpr: true }), eq: new Pe("=", { beforeExpr: true, isAssign: true }), assign: new Pe("_=", { beforeExpr: true, isAssign: true }), incDec: new Pe("++/--", { prefix: true, postfix: true, startsExpr: true }), prefix: new Pe("!/~", { beforeExpr: true, prefix: true, startsExpr: true }), logicalOR: st("||", 1), logicalAND: st("&&", 2), bitwiseOR: st("|", 3), bitwiseXOR: st("^", 4), bitwiseAND: st("&", 5), equality: st("==/!=/===/!==", 6), relational: st("</>/<=/>=", 7), bitShift: st("<</>>/>>>", 8), plusMin: new Pe("+/-", { beforeExpr: true, binop: 9, prefix: true, startsExpr: true }), modulo: st("%", 10), star: st("*", 10), slash: st("/", 10), starstar: new Pe("**", { beforeExpr: true }), coalesce: st("??", 1), _break: Te("break"), _case: Te("case", nt), _catch: Te("catch"), _continue: Te("continue"), _debugger: Te("debugger"), _default: Te("default", nt), _do: Te("do", { isLoop: true, beforeExpr: true }), _else: Te("else", nt), _finally: Te("finally"), _for: Te("for", { isLoop: true }), _function: Te("function", Ze), _if: Te("if"), _return: Te("return", nt), _switch: Te("switch"), _throw: Te("throw", nt), _try: Te("try"), _var: Te("var"), _const: Te("const"), _while: Te("while", { isLoop: true }), _with: Te("with"), _new: Te("new", { beforeExpr: true, startsExpr: true }), _this: Te("this", Ze), _super: Te("super", Ze), _class: Te("class", Ze), _extends: Te("extends", nt), _export: Te("export"), _import: Te("import", Ze), _null: Te("null", Ze), _true: Te("true", Ze), _false: Te("false", Ze), _in: Te("in", { beforeExpr: true, binop: 7 }), _instanceof: Te("instanceof", { beforeExpr: true, binop: 7 }), _typeof: Te("typeof", { beforeExpr: true, prefix: true, startsExpr: true }), _void: Te("void", { beforeExpr: true, prefix: true, startsExpr: true }), _delete: Te("delete", { beforeExpr: true, prefix: true, startsExpr: true }) }, at = /\r\n?|\n|\u2028|\u2029/, mc = new RegExp(at.source, "g");
function Ut(e) {
  return e === 10 || e === 13 || e === 8232 || e === 8233;
}
function vc(e, t, r) {
  r === void 0 && (r = e.length);
  for (var s = t; s < r; s++) {
    var o = e.charCodeAt(s);
    if (Ut(o)) return s < r - 1 && o === 13 && e.charCodeAt(s + 1) === 10 ? s + 2 : s + 1;
  }
  return -1;
}
var Zl = /[\u1680\u2000-\u200a\u202f\u205f\u3000\ufeff]/, lt = /(?:\s|\/\/.*|\/\*[^]*?\*\/)*/g, bc = Object.prototype, cp = bc.hasOwnProperty, dp = bc.toString, ir = Object.hasOwn || function(e, t) {
  return cp.call(e, t);
}, _u = Array.isArray || function(e) {
  return dp.call(e) === "[object Array]";
}, Cu = /* @__PURE__ */ Object.create(null);
function Mt(e) {
  return Cu[e] || (Cu[e] = new RegExp("^(?:" + e.replace(/ /g, "|") + ")$"));
}
function Tt(e) {
  return e <= 65535 ? String.fromCharCode(e) : (e -= 65536, String.fromCharCode((e >> 10) + 55296, (e & 1023) + 56320));
}
var pp = /(?:[\uD800-\uDBFF](?![\uDC00-\uDFFF])|(?:[^\uD800-\uDBFF]|^)[\uDC00-\uDFFF])/, er = function(t, r) {
  this.line = t, this.column = r;
};
er.prototype.offset = function(t) {
  return new er(this.line, this.column + t);
};
var xr = function(t, r, s) {
  this.start = r, this.end = s, t.sourceFile !== null && (this.source = t.sourceFile);
};
function eu(e, t) {
  for (var r = 1, s = 0; ; ) {
    var o = vc(e, s, t);
    if (o < 0) return new er(r, t - s);
    ++r, s = o;
  }
}
var Br = { ecmaVersion: null, sourceType: "script", onInsertedSemicolon: null, onTrailingComma: null, allowReserved: null, allowReturnOutsideFunction: false, allowImportExportEverywhere: false, allowAwaitOutsideFunction: null, allowSuperOutsideMethod: null, allowHashBang: false, checkPrivateFields: true, locations: false, onToken: null, onComment: null, ranges: false, program: null, sourceFile: null, directSourceFile: null, preserveParens: false }, Ru = false;
function hp(e) {
  var t = {};
  for (var r in Br) t[r] = e && ir(e, r) ? e[r] : Br[r];
  if (t.ecmaVersion === "latest" ? t.ecmaVersion = 1e8 : t.ecmaVersion == null ? (!Ru && typeof console == "object" && console.warn && (Ru = true, console.warn(`Since Acorn 8.0.0, options.ecmaVersion is required.
Defaulting to 2020, but this will stop working in the future.`)), t.ecmaVersion = 11) : t.ecmaVersion >= 2015 && (t.ecmaVersion -= 2009), t.allowReserved == null && (t.allowReserved = t.ecmaVersion < 5), (!e || e.allowHashBang == null) && (t.allowHashBang = t.ecmaVersion >= 14), _u(t.onToken)) {
    var s = t.onToken;
    t.onToken = function(o) {
      return s.push(o);
    };
  }
  return _u(t.onComment) && (t.onComment = fp(t, t.onComment)), t;
}
function fp(e, t) {
  return function(r, s, o, p, h, v) {
    var m = { type: r ? "Block" : "Line", value: s, start: o, end: p };
    e.locations && (m.loc = new xr(this, h, v)), e.ranges && (m.range = [o, p]), t.push(m);
  };
}
var yr = 1, sr = 2, tu = 4, yc = 8, gc = 16, xc = 32, ru = 64, _c = 128, _r = 256, au = yr | sr | _r;
function iu(e, t) {
  return sr | (e ? tu : 0) | (t ? yc : 0);
}
var Vr = 0, su = 1, kt = 2, Cc = 3, Rc = 4, Pc = 5, Be = function(t, r, s) {
  this.options = t = hp(t), this.sourceFile = t.sourceFile, this.keywords = Mt(np[t.ecmaVersion >= 6 ? 6 : t.sourceType === "module" ? "5module" : 5]);
  var o = "";
  t.allowReserved !== true && (o = dl[t.ecmaVersion >= 6 ? 6 : t.ecmaVersion === 5 ? 5 : 3], t.sourceType === "module" && (o += " await")), this.reservedWords = Mt(o);
  var p = (o ? o + " " : "") + dl.strict;
  this.reservedWordsStrict = Mt(p), this.reservedWordsStrictBind = Mt(p + " " + dl.strictBind), this.input = String(r), this.containsEsc = false, s ? (this.pos = s, this.lineStart = this.input.lastIndexOf(`
`, s - 1) + 1, this.curLine = this.input.slice(0, this.lineStart).split(at).length) : (this.pos = this.lineStart = 0, this.curLine = 1), this.type = d.eof, this.value = null, this.start = this.end = this.pos, this.startLoc = this.endLoc = this.curPosition(), this.lastTokEndLoc = this.lastTokStartLoc = null, this.lastTokStart = this.lastTokEnd = this.pos, this.context = this.initialContext(), this.exprAllowed = true, this.inModule = t.sourceType === "module", this.strict = this.inModule || this.strictDirective(this.pos), this.potentialArrowAt = -1, this.potentialArrowInForAwait = false, this.yieldPos = this.awaitPos = this.awaitIdentPos = 0, this.labels = [], this.undefinedExports = /* @__PURE__ */ Object.create(null), this.pos === 0 && t.allowHashBang && this.input.slice(0, 2) === "#!" && this.skipLineComment(2), this.scopeStack = [], this.enterScope(yr), this.regexpState = null, this.privateNameStack = [];
}, _t = { inFunction: { configurable: true }, inGenerator: { configurable: true }, inAsync: { configurable: true }, canAwait: { configurable: true }, allowSuper: { configurable: true }, allowDirectSuper: { configurable: true }, treatFunctionsAsVar: { configurable: true }, allowNewDotTarget: { configurable: true }, inClassStaticBlock: { configurable: true } };
Be.prototype.parse = function() {
  var t = this.options.program || this.startNode();
  return this.nextToken(), this.parseTopLevel(t);
};
_t.inFunction.get = function() {
  return (this.currentVarScope().flags & sr) > 0;
};
_t.inGenerator.get = function() {
  return (this.currentVarScope().flags & yc) > 0 && !this.currentVarScope().inClassFieldInit;
};
_t.inAsync.get = function() {
  return (this.currentVarScope().flags & tu) > 0 && !this.currentVarScope().inClassFieldInit;
};
_t.canAwait.get = function() {
  for (var e = this.scopeStack.length - 1; e >= 0; e--) {
    var t = this.scopeStack[e];
    if (t.inClassFieldInit || t.flags & _r) return false;
    if (t.flags & sr) return (t.flags & tu) > 0;
  }
  return this.inModule && this.options.ecmaVersion >= 13 || this.options.allowAwaitOutsideFunction;
};
_t.allowSuper.get = function() {
  var e = this.currentThisScope(), t = e.flags, r = e.inClassFieldInit;
  return (t & ru) > 0 || r || this.options.allowSuperOutsideMethod;
};
_t.allowDirectSuper.get = function() {
  return (this.currentThisScope().flags & _c) > 0;
};
_t.treatFunctionsAsVar.get = function() {
  return this.treatFunctionsAsVarInScope(this.currentScope());
};
_t.allowNewDotTarget.get = function() {
  var e = this.currentThisScope(), t = e.flags, r = e.inClassFieldInit;
  return (t & (sr | _r)) > 0 || r;
};
_t.inClassStaticBlock.get = function() {
  return (this.currentVarScope().flags & _r) > 0;
};
Be.extend = function() {
  for (var t = [], r = arguments.length; r--; ) t[r] = arguments[r];
  for (var s = this, o = 0; o < t.length; o++) s = t[o](s);
  return s;
};
Be.parse = function(t, r) {
  return new this(r, t).parse();
};
Be.parseExpressionAt = function(t, r, s) {
  var o = new this(s, t, r);
  return o.nextToken(), o.parseExpression();
};
Be.tokenizer = function(t, r) {
  return new this(r, t);
};
Object.defineProperties(Be.prototype, _t);
var Xe = Be.prototype, mp = /^(?:'((?:\\[^]|[^'\\])*?)'|"((?:\\[^]|[^"\\])*?)")/;
Xe.strictDirective = function(e) {
  if (this.options.ecmaVersion < 5) return false;
  for (; ; ) {
    lt.lastIndex = e, e += lt.exec(this.input)[0].length;
    var t = mp.exec(this.input.slice(e));
    if (!t) return false;
    if ((t[1] || t[2]) === "use strict") {
      lt.lastIndex = e + t[0].length;
      var r = lt.exec(this.input), s = r.index + r[0].length, o = this.input.charAt(s);
      return o === ";" || o === "}" || at.test(r[0]) && !(/[(`.[+\-/*%<>=,?^&]/.test(o) || o === "!" && this.input.charAt(s + 1) === "=");
    }
    e += t[0].length, lt.lastIndex = e, e += lt.exec(this.input)[0].length, this.input[e] === ";" && e++;
  }
};
Xe.eat = function(e) {
  return this.type === e ? (this.next(), true) : false;
};
Xe.isContextual = function(e) {
  return this.type === d.name && this.value === e && !this.containsEsc;
};
Xe.eatContextual = function(e) {
  return this.isContextual(e) ? (this.next(), true) : false;
};
Xe.expectContextual = function(e) {
  this.eatContextual(e) || this.unexpected();
};
Xe.canInsertSemicolon = function() {
  return this.type === d.eof || this.type === d.braceR || at.test(this.input.slice(this.lastTokEnd, this.start));
};
Xe.insertSemicolon = function() {
  if (this.canInsertSemicolon()) return this.options.onInsertedSemicolon && this.options.onInsertedSemicolon(this.lastTokEnd, this.lastTokEndLoc), true;
};
Xe.semicolon = function() {
  !this.eat(d.semi) && !this.insertSemicolon() && this.unexpected();
};
Xe.afterTrailingComma = function(e, t) {
  if (this.type === e) return this.options.onTrailingComma && this.options.onTrailingComma(this.lastTokStart, this.lastTokStartLoc), t || this.next(), true;
};
Xe.expect = function(e) {
  this.eat(e) || this.unexpected();
};
Xe.unexpected = function(e) {
  this.raise(e ?? this.start, "Unexpected token");
};
var Kr = function() {
  this.shorthandAssign = this.trailingComma = this.parenthesizedAssign = this.parenthesizedBind = this.doubleProto = -1;
};
Xe.checkPatternErrors = function(e, t) {
  if (e) {
    e.trailingComma > -1 && this.raiseRecoverable(e.trailingComma, "Comma is not permitted after the rest element");
    var r = t ? e.parenthesizedAssign : e.parenthesizedBind;
    r > -1 && this.raiseRecoverable(r, t ? "Assigning to rvalue" : "Parenthesized pattern");
  }
};
Xe.checkExpressionErrors = function(e, t) {
  if (!e) return false;
  var r = e.shorthandAssign, s = e.doubleProto;
  if (!t) return r >= 0 || s >= 0;
  r >= 0 && this.raise(r, "Shorthand property assignments are valid only in destructuring patterns"), s >= 0 && this.raiseRecoverable(s, "Redefinition of __proto__ property");
};
Xe.checkYieldAwaitInDefaultParams = function() {
  this.yieldPos && (!this.awaitPos || this.yieldPos < this.awaitPos) && this.raise(this.yieldPos, "Yield expression cannot be a default value"), this.awaitPos && this.raise(this.awaitPos, "Await expression cannot be a default value");
};
Xe.isSimpleAssignTarget = function(e) {
  return e.type === "ParenthesizedExpression" ? this.isSimpleAssignTarget(e.expression) : e.type === "Identifier" || e.type === "MemberExpression";
};
var oe = Be.prototype;
oe.parseTopLevel = function(e) {
  var t = /* @__PURE__ */ Object.create(null);
  for (e.body || (e.body = []); this.type !== d.eof; ) {
    var r = this.parseStatement(null, true, t);
    e.body.push(r);
  }
  if (this.inModule) for (var s = 0, o = Object.keys(this.undefinedExports); s < o.length; s += 1) {
    var p = o[s];
    this.raiseRecoverable(this.undefinedExports[p].start, "Export '" + p + "' is not defined");
  }
  return this.adaptDirectivePrologue(e.body), this.next(), e.sourceType = this.options.sourceType, this.finishNode(e, "Program");
};
var nu = { kind: "loop" }, vp = { kind: "switch" };
oe.isLet = function(e) {
  if (this.options.ecmaVersion < 6 || !this.isContextual("let")) return false;
  lt.lastIndex = this.pos;
  var t = lt.exec(this.input), r = this.pos + t[0].length, s = this.input.charCodeAt(r);
  if (s === 91 || s === 92) return true;
  if (e) return false;
  if (s === 123 || s > 55295 && s < 56320) return true;
  if (xt(s, true)) {
    for (var o = r + 1; Vt(s = this.input.charCodeAt(o), true); ) ++o;
    if (s === 92 || s > 55295 && s < 56320) return true;
    var p = this.input.slice(r, o);
    if (!op.test(p)) return true;
  }
  return false;
};
oe.isAsyncFunction = function() {
  if (this.options.ecmaVersion < 8 || !this.isContextual("async")) return false;
  lt.lastIndex = this.pos;
  var e = lt.exec(this.input), t = this.pos + e[0].length, r;
  return !at.test(this.input.slice(this.pos, t)) && this.input.slice(t, t + 8) === "function" && (t + 8 === this.input.length || !(Vt(r = this.input.charCodeAt(t + 8)) || r > 55295 && r < 56320));
};
oe.parseStatement = function(e, t, r) {
  var s = this.type, o = this.startNode(), p;
  switch (this.isLet(e) && (s = d._var, p = "let"), s) {
    case d._break:
    case d._continue:
      return this.parseBreakContinueStatement(o, s.keyword);
    case d._debugger:
      return this.parseDebuggerStatement(o);
    case d._do:
      return this.parseDoStatement(o);
    case d._for:
      return this.parseForStatement(o);
    case d._function:
      return e && (this.strict || e !== "if" && e !== "label") && this.options.ecmaVersion >= 6 && this.unexpected(), this.parseFunctionStatement(o, false, !e);
    case d._class:
      return e && this.unexpected(), this.parseClass(o, true);
    case d._if:
      return this.parseIfStatement(o);
    case d._return:
      return this.parseReturnStatement(o);
    case d._switch:
      return this.parseSwitchStatement(o);
    case d._throw:
      return this.parseThrowStatement(o);
    case d._try:
      return this.parseTryStatement(o);
    case d._const:
    case d._var:
      return p = p || this.value, e && p !== "var" && this.unexpected(), this.parseVarStatement(o, p);
    case d._while:
      return this.parseWhileStatement(o);
    case d._with:
      return this.parseWithStatement(o);
    case d.braceL:
      return this.parseBlock(true, o);
    case d.semi:
      return this.parseEmptyStatement(o);
    case d._export:
    case d._import:
      if (this.options.ecmaVersion > 10 && s === d._import) {
        lt.lastIndex = this.pos;
        var h = lt.exec(this.input), v = this.pos + h[0].length, m = this.input.charCodeAt(v);
        if (m === 40 || m === 46) return this.parseExpressionStatement(o, this.parseExpression());
      }
      return this.options.allowImportExportEverywhere || (t || this.raise(this.start, "'import' and 'export' may only appear at the top level"), this.inModule || this.raise(this.start, "'import' and 'export' may appear only with 'sourceType: module'")), s === d._import ? this.parseImport(o) : this.parseExport(o, r);
    default:
      if (this.isAsyncFunction()) return e && this.unexpected(), this.next(), this.parseFunctionStatement(o, true, !e);
      var l = this.value, w = this.parseExpression();
      return s === d.name && w.type === "Identifier" && this.eat(d.colon) ? this.parseLabeledStatement(o, l, w, e) : this.parseExpressionStatement(o, w);
  }
};
oe.parseBreakContinueStatement = function(e, t) {
  var r = t === "break";
  this.next(), this.eat(d.semi) || this.insertSemicolon() ? e.label = null : this.type !== d.name ? this.unexpected() : (e.label = this.parseIdent(), this.semicolon());
  for (var s = 0; s < this.labels.length; ++s) {
    var o = this.labels[s];
    if ((e.label == null || o.name === e.label.name) && (o.kind != null && (r || o.kind === "loop") || e.label && r)) break;
  }
  return s === this.labels.length && this.raise(e.start, "Unsyntactic " + t), this.finishNode(e, r ? "BreakStatement" : "ContinueStatement");
};
oe.parseDebuggerStatement = function(e) {
  return this.next(), this.semicolon(), this.finishNode(e, "DebuggerStatement");
};
oe.parseDoStatement = function(e) {
  return this.next(), this.labels.push(nu), e.body = this.parseStatement("do"), this.labels.pop(), this.expect(d._while), e.test = this.parseParenExpression(), this.options.ecmaVersion >= 6 ? this.eat(d.semi) : this.semicolon(), this.finishNode(e, "DoWhileStatement");
};
oe.parseForStatement = function(e) {
  this.next();
  var t = this.options.ecmaVersion >= 9 && this.canAwait && this.eatContextual("await") ? this.lastTokStart : -1;
  if (this.labels.push(nu), this.enterScope(0), this.expect(d.parenL), this.type === d.semi) return t > -1 && this.unexpected(t), this.parseFor(e, null);
  var r = this.isLet();
  if (this.type === d._var || this.type === d._const || r) {
    var s = this.startNode(), o = r ? "let" : this.value;
    return this.next(), this.parseVar(s, true, o), this.finishNode(s, "VariableDeclaration"), (this.type === d._in || this.options.ecmaVersion >= 6 && this.isContextual("of")) && s.declarations.length === 1 ? (this.options.ecmaVersion >= 9 && (this.type === d._in ? t > -1 && this.unexpected(t) : e.await = t > -1), this.parseForIn(e, s)) : (t > -1 && this.unexpected(t), this.parseFor(e, s));
  }
  var p = this.isContextual("let"), h = false, v = this.containsEsc, m = new Kr(), l = this.start, w = t > -1 ? this.parseExprSubscripts(m, "await") : this.parseExpression(true, m);
  return this.type === d._in || (h = this.options.ecmaVersion >= 6 && this.isContextual("of")) ? (t > -1 ? (this.type === d._in && this.unexpected(t), e.await = true) : h && this.options.ecmaVersion >= 8 && (w.start === l && !v && w.type === "Identifier" && w.name === "async" ? this.unexpected() : this.options.ecmaVersion >= 9 && (e.await = false)), p && h && this.raise(w.start, "The left-hand side of a for-of loop may not start with 'let'."), this.toAssignable(w, false, m), this.checkLValPattern(w), this.parseForIn(e, w)) : (this.checkExpressionErrors(m, true), t > -1 && this.unexpected(t), this.parseFor(e, w));
};
oe.parseFunctionStatement = function(e, t, r) {
  return this.next(), this.parseFunction(e, pr | (r ? 0 : Ml), false, t);
};
oe.parseIfStatement = function(e) {
  return this.next(), e.test = this.parseParenExpression(), e.consequent = this.parseStatement("if"), e.alternate = this.eat(d._else) ? this.parseStatement("if") : null, this.finishNode(e, "IfStatement");
};
oe.parseReturnStatement = function(e) {
  return !this.inFunction && !this.options.allowReturnOutsideFunction && this.raise(this.start, "'return' outside of function"), this.next(), this.eat(d.semi) || this.insertSemicolon() ? e.argument = null : (e.argument = this.parseExpression(), this.semicolon()), this.finishNode(e, "ReturnStatement");
};
oe.parseSwitchStatement = function(e) {
  this.next(), e.discriminant = this.parseParenExpression(), e.cases = [], this.expect(d.braceL), this.labels.push(vp), this.enterScope(0);
  for (var t, r = false; this.type !== d.braceR; ) if (this.type === d._case || this.type === d._default) {
    var s = this.type === d._case;
    t && this.finishNode(t, "SwitchCase"), e.cases.push(t = this.startNode()), t.consequent = [], this.next(), s ? t.test = this.parseExpression() : (r && this.raiseRecoverable(this.lastTokStart, "Multiple default clauses"), r = true, t.test = null), this.expect(d.colon);
  } else t || this.unexpected(), t.consequent.push(this.parseStatement(null));
  return this.exitScope(), t && this.finishNode(t, "SwitchCase"), this.next(), this.labels.pop(), this.finishNode(e, "SwitchStatement");
};
oe.parseThrowStatement = function(e) {
  return this.next(), at.test(this.input.slice(this.lastTokEnd, this.start)) && this.raise(this.lastTokEnd, "Illegal newline after throw"), e.argument = this.parseExpression(), this.semicolon(), this.finishNode(e, "ThrowStatement");
};
var bp = [];
oe.parseCatchClauseParam = function() {
  var e = this.parseBindingAtom(), t = e.type === "Identifier";
  return this.enterScope(t ? xc : 0), this.checkLValPattern(e, t ? Rc : kt), this.expect(d.parenR), e;
};
oe.parseTryStatement = function(e) {
  if (this.next(), e.block = this.parseBlock(), e.handler = null, this.type === d._catch) {
    var t = this.startNode();
    this.next(), this.eat(d.parenL) ? t.param = this.parseCatchClauseParam() : (this.options.ecmaVersion < 10 && this.unexpected(), t.param = null, this.enterScope(0)), t.body = this.parseBlock(false), this.exitScope(), e.handler = this.finishNode(t, "CatchClause");
  }
  return e.finalizer = this.eat(d._finally) ? this.parseBlock() : null, !e.handler && !e.finalizer && this.raise(e.start, "Missing catch or finally clause"), this.finishNode(e, "TryStatement");
};
oe.parseVarStatement = function(e, t, r) {
  return this.next(), this.parseVar(e, false, t, r), this.semicolon(), this.finishNode(e, "VariableDeclaration");
};
oe.parseWhileStatement = function(e) {
  return this.next(), e.test = this.parseParenExpression(), this.labels.push(nu), e.body = this.parseStatement("while"), this.labels.pop(), this.finishNode(e, "WhileStatement");
};
oe.parseWithStatement = function(e) {
  return this.strict && this.raise(this.start, "'with' in strict mode"), this.next(), e.object = this.parseParenExpression(), e.body = this.parseStatement("with"), this.finishNode(e, "WithStatement");
};
oe.parseEmptyStatement = function(e) {
  return this.next(), this.finishNode(e, "EmptyStatement");
};
oe.parseLabeledStatement = function(e, t, r, s) {
  for (var o = 0, p = this.labels; o < p.length; o += 1) {
    var h = p[o];
    h.name === t && this.raise(r.start, "Label '" + t + "' is already declared");
  }
  for (var v = this.type.isLoop ? "loop" : this.type === d._switch ? "switch" : null, m = this.labels.length - 1; m >= 0; m--) {
    var l = this.labels[m];
    if (l.statementStart === e.start) l.statementStart = this.start, l.kind = v;
    else break;
  }
  return this.labels.push({ name: t, kind: v, statementStart: this.start }), e.body = this.parseStatement(s ? s.indexOf("label") === -1 ? s + "label" : s : "label"), this.labels.pop(), e.label = r, this.finishNode(e, "LabeledStatement");
};
oe.parseExpressionStatement = function(e, t) {
  return e.expression = t, this.semicolon(), this.finishNode(e, "ExpressionStatement");
};
oe.parseBlock = function(e, t, r) {
  for (e === void 0 && (e = true), t === void 0 && (t = this.startNode()), t.body = [], this.expect(d.braceL), e && this.enterScope(0); this.type !== d.braceR; ) {
    var s = this.parseStatement(null);
    t.body.push(s);
  }
  return r && (this.strict = false), this.next(), e && this.exitScope(), this.finishNode(t, "BlockStatement");
};
oe.parseFor = function(e, t) {
  return e.init = t, this.expect(d.semi), e.test = this.type === d.semi ? null : this.parseExpression(), this.expect(d.semi), e.update = this.type === d.parenR ? null : this.parseExpression(), this.expect(d.parenR), e.body = this.parseStatement("for"), this.exitScope(), this.labels.pop(), this.finishNode(e, "ForStatement");
};
oe.parseForIn = function(e, t) {
  var r = this.type === d._in;
  return this.next(), t.type === "VariableDeclaration" && t.declarations[0].init != null && (!r || this.options.ecmaVersion < 8 || this.strict || t.kind !== "var" || t.declarations[0].id.type !== "Identifier") && this.raise(t.start, (r ? "for-in" : "for-of") + " loop variable declaration may not have an initializer"), e.left = t, e.right = r ? this.parseExpression() : this.parseMaybeAssign(), this.expect(d.parenR), e.body = this.parseStatement("for"), this.exitScope(), this.labels.pop(), this.finishNode(e, r ? "ForInStatement" : "ForOfStatement");
};
oe.parseVar = function(e, t, r, s) {
  for (e.declarations = [], e.kind = r; ; ) {
    var o = this.startNode();
    if (this.parseVarId(o, r), this.eat(d.eq) ? o.init = this.parseMaybeAssign(t) : !s && r === "const" && !(this.type === d._in || this.options.ecmaVersion >= 6 && this.isContextual("of")) ? this.unexpected() : !s && o.id.type !== "Identifier" && !(t && (this.type === d._in || this.isContextual("of"))) ? this.raise(this.lastTokEnd, "Complex binding patterns require an initialization value") : o.init = null, e.declarations.push(this.finishNode(o, "VariableDeclarator")), !this.eat(d.comma)) break;
  }
  return e;
};
oe.parseVarId = function(e, t) {
  e.id = this.parseBindingAtom(), this.checkLValPattern(e.id, t === "var" ? su : kt, false);
};
var pr = 1, Ml = 2, wc = 4;
oe.parseFunction = function(e, t, r, s, o) {
  this.initFunction(e), (this.options.ecmaVersion >= 9 || this.options.ecmaVersion >= 6 && !s) && (this.type === d.star && t & Ml && this.unexpected(), e.generator = this.eat(d.star)), this.options.ecmaVersion >= 8 && (e.async = !!s), t & pr && (e.id = t & wc && this.type !== d.name ? null : this.parseIdent(), e.id && !(t & Ml) && this.checkLValSimple(e.id, this.strict || e.generator || e.async ? this.treatFunctionsAsVar ? su : kt : Cc));
  var p = this.yieldPos, h = this.awaitPos, v = this.awaitIdentPos;
  return this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0, this.enterScope(iu(e.async, e.generator)), t & pr || (e.id = this.type === d.name ? this.parseIdent() : null), this.parseFunctionParams(e), this.parseFunctionBody(e, r, false, o), this.yieldPos = p, this.awaitPos = h, this.awaitIdentPos = v, this.finishNode(e, t & pr ? "FunctionDeclaration" : "FunctionExpression");
};
oe.parseFunctionParams = function(e) {
  this.expect(d.parenL), e.params = this.parseBindingList(d.parenR, false, this.options.ecmaVersion >= 8), this.checkYieldAwaitInDefaultParams();
};
oe.parseClass = function(e, t) {
  this.next();
  var r = this.strict;
  this.strict = true, this.parseClassId(e, t), this.parseClassSuper(e);
  var s = this.enterClassBody(), o = this.startNode(), p = false;
  for (o.body = [], this.expect(d.braceL); this.type !== d.braceR; ) {
    var h = this.parseClassElement(e.superClass !== null);
    h && (o.body.push(h), h.type === "MethodDefinition" && h.kind === "constructor" ? (p && this.raiseRecoverable(h.start, "Duplicate constructor in the same class"), p = true) : h.key && h.key.type === "PrivateIdentifier" && yp(s, h) && this.raiseRecoverable(h.key.start, "Identifier '#" + h.key.name + "' has already been declared"));
  }
  return this.strict = r, this.next(), e.body = this.finishNode(o, "ClassBody"), this.exitClassBody(), this.finishNode(e, t ? "ClassDeclaration" : "ClassExpression");
};
oe.parseClassElement = function(e) {
  if (this.eat(d.semi)) return null;
  var t = this.options.ecmaVersion, r = this.startNode(), s = "", o = false, p = false, h = "method", v = false;
  if (this.eatContextual("static")) {
    if (t >= 13 && this.eat(d.braceL)) return this.parseClassStaticBlock(r), r;
    this.isClassElementNameStart() || this.type === d.star ? v = true : s = "static";
  }
  if (r.static = v, !s && t >= 8 && this.eatContextual("async") && ((this.isClassElementNameStart() || this.type === d.star) && !this.canInsertSemicolon() ? p = true : s = "async"), !s && (t >= 9 || !p) && this.eat(d.star) && (o = true), !s && !p && !o) {
    var m = this.value;
    (this.eatContextual("get") || this.eatContextual("set")) && (this.isClassElementNameStart() ? h = m : s = m);
  }
  if (s ? (r.computed = false, r.key = this.startNodeAt(this.lastTokStart, this.lastTokStartLoc), r.key.name = s, this.finishNode(r.key, "Identifier")) : this.parseClassElementName(r), t < 13 || this.type === d.parenL || h !== "method" || o || p) {
    var l = !r.static && Hr(r, "constructor"), w = l && e;
    l && h !== "method" && this.raise(r.key.start, "Constructor can't have get/set modifier"), r.kind = l ? "constructor" : h, this.parseClassMethod(r, o, p, w);
  } else this.parseClassField(r);
  return r;
};
oe.isClassElementNameStart = function() {
  return this.type === d.name || this.type === d.privateId || this.type === d.num || this.type === d.string || this.type === d.bracketL || this.type.keyword;
};
oe.parseClassElementName = function(e) {
  this.type === d.privateId ? (this.value === "constructor" && this.raise(this.start, "Classes can't have an element named '#constructor'"), e.computed = false, e.key = this.parsePrivateIdent()) : this.parsePropertyName(e);
};
oe.parseClassMethod = function(e, t, r, s) {
  var o = e.key;
  e.kind === "constructor" ? (t && this.raise(o.start, "Constructor can't be a generator"), r && this.raise(o.start, "Constructor can't be an async method")) : e.static && Hr(e, "prototype") && this.raise(o.start, "Classes may not have a static property named prototype");
  var p = e.value = this.parseMethod(t, r, s);
  return e.kind === "get" && p.params.length !== 0 && this.raiseRecoverable(p.start, "getter should have no params"), e.kind === "set" && p.params.length !== 1 && this.raiseRecoverable(p.start, "setter should have exactly one param"), e.kind === "set" && p.params[0].type === "RestElement" && this.raiseRecoverable(p.params[0].start, "Setter cannot use rest params"), this.finishNode(e, "MethodDefinition");
};
oe.parseClassField = function(e) {
  if (Hr(e, "constructor") ? this.raise(e.key.start, "Classes can't have a field named 'constructor'") : e.static && Hr(e, "prototype") && this.raise(e.key.start, "Classes can't have a static field named 'prototype'"), this.eat(d.eq)) {
    var t = this.currentThisScope(), r = t.inClassFieldInit;
    t.inClassFieldInit = true, e.value = this.parseMaybeAssign(), t.inClassFieldInit = r;
  } else e.value = null;
  return this.semicolon(), this.finishNode(e, "PropertyDefinition");
};
oe.parseClassStaticBlock = function(e) {
  e.body = [];
  var t = this.labels;
  for (this.labels = [], this.enterScope(_r | ru); this.type !== d.braceR; ) {
    var r = this.parseStatement(null);
    e.body.push(r);
  }
  return this.next(), this.exitScope(), this.labels = t, this.finishNode(e, "StaticBlock");
};
oe.parseClassId = function(e, t) {
  this.type === d.name ? (e.id = this.parseIdent(), t && this.checkLValSimple(e.id, kt, false)) : (t === true && this.unexpected(), e.id = null);
};
oe.parseClassSuper = function(e) {
  e.superClass = this.eat(d._extends) ? this.parseExprSubscripts(null, false) : null;
};
oe.enterClassBody = function() {
  var e = { declared: /* @__PURE__ */ Object.create(null), used: [] };
  return this.privateNameStack.push(e), e.declared;
};
oe.exitClassBody = function() {
  var e = this.privateNameStack.pop(), t = e.declared, r = e.used;
  if (this.options.checkPrivateFields) for (var s = this.privateNameStack.length, o = s === 0 ? null : this.privateNameStack[s - 1], p = 0; p < r.length; ++p) {
    var h = r[p];
    ir(t, h.name) || (o ? o.used.push(h) : this.raiseRecoverable(h.start, "Private field '#" + h.name + "' must be declared in an enclosing class"));
  }
};
function yp(e, t) {
  var r = t.key.name, s = e[r], o = "true";
  return t.type === "MethodDefinition" && (t.kind === "get" || t.kind === "set") && (o = (t.static ? "s" : "i") + t.kind), s === "iget" && o === "iset" || s === "iset" && o === "iget" || s === "sget" && o === "sset" || s === "sset" && o === "sget" ? (e[r] = "true", false) : s ? true : (e[r] = o, false);
}
function Hr(e, t) {
  var r = e.computed, s = e.key;
  return !r && (s.type === "Identifier" && s.name === t || s.type === "Literal" && s.value === t);
}
oe.parseExportAllDeclaration = function(e, t) {
  return this.options.ecmaVersion >= 11 && (this.eatContextual("as") ? (e.exported = this.parseModuleExportName(), this.checkExport(t, e.exported, this.lastTokStart)) : e.exported = null), this.expectContextual("from"), this.type !== d.string && this.unexpected(), e.source = this.parseExprAtom(), this.options.ecmaVersion >= 16 && (e.attributes = this.parseWithClause()), this.semicolon(), this.finishNode(e, "ExportAllDeclaration");
};
oe.parseExport = function(e, t) {
  if (this.next(), this.eat(d.star)) return this.parseExportAllDeclaration(e, t);
  if (this.eat(d._default)) return this.checkExport(t, "default", this.lastTokStart), e.declaration = this.parseExportDefaultDeclaration(), this.finishNode(e, "ExportDefaultDeclaration");
  if (this.shouldParseExportStatement()) e.declaration = this.parseExportDeclaration(e), e.declaration.type === "VariableDeclaration" ? this.checkVariableExport(t, e.declaration.declarations) : this.checkExport(t, e.declaration.id, e.declaration.id.start), e.specifiers = [], e.source = null;
  else {
    if (e.declaration = null, e.specifiers = this.parseExportSpecifiers(t), this.eatContextual("from")) this.type !== d.string && this.unexpected(), e.source = this.parseExprAtom(), this.options.ecmaVersion >= 16 && (e.attributes = this.parseWithClause());
    else {
      for (var r = 0, s = e.specifiers; r < s.length; r += 1) {
        var o = s[r];
        this.checkUnreserved(o.local), this.checkLocalExport(o.local), o.local.type === "Literal" && this.raise(o.local.start, "A string literal cannot be used as an exported binding without `from`.");
      }
      e.source = null;
    }
    this.semicolon();
  }
  return this.finishNode(e, "ExportNamedDeclaration");
};
oe.parseExportDeclaration = function(e) {
  return this.parseStatement(null);
};
oe.parseExportDefaultDeclaration = function() {
  var e;
  if (this.type === d._function || (e = this.isAsyncFunction())) {
    var t = this.startNode();
    return this.next(), e && this.next(), this.parseFunction(t, pr | wc, false, e);
  } else if (this.type === d._class) {
    var r = this.startNode();
    return this.parseClass(r, "nullableID");
  } else {
    var s = this.parseMaybeAssign();
    return this.semicolon(), s;
  }
};
oe.checkExport = function(e, t, r) {
  e && (typeof t != "string" && (t = t.type === "Identifier" ? t.name : t.value), ir(e, t) && this.raiseRecoverable(r, "Duplicate export '" + t + "'"), e[t] = true);
};
oe.checkPatternExport = function(e, t) {
  var r = t.type;
  if (r === "Identifier") this.checkExport(e, t, t.start);
  else if (r === "ObjectPattern") for (var s = 0, o = t.properties; s < o.length; s += 1) {
    var p = o[s];
    this.checkPatternExport(e, p);
  }
  else if (r === "ArrayPattern") for (var h = 0, v = t.elements; h < v.length; h += 1) {
    var m = v[h];
    m && this.checkPatternExport(e, m);
  }
  else r === "Property" ? this.checkPatternExport(e, t.value) : r === "AssignmentPattern" ? this.checkPatternExport(e, t.left) : r === "RestElement" && this.checkPatternExport(e, t.argument);
};
oe.checkVariableExport = function(e, t) {
  if (e) for (var r = 0, s = t; r < s.length; r += 1) {
    var o = s[r];
    this.checkPatternExport(e, o.id);
  }
};
oe.shouldParseExportStatement = function() {
  return this.type.keyword === "var" || this.type.keyword === "const" || this.type.keyword === "class" || this.type.keyword === "function" || this.isLet() || this.isAsyncFunction();
};
oe.parseExportSpecifier = function(e) {
  var t = this.startNode();
  return t.local = this.parseModuleExportName(), t.exported = this.eatContextual("as") ? this.parseModuleExportName() : t.local, this.checkExport(e, t.exported, t.exported.start), this.finishNode(t, "ExportSpecifier");
};
oe.parseExportSpecifiers = function(e) {
  var t = [], r = true;
  for (this.expect(d.braceL); !this.eat(d.braceR); ) {
    if (r) r = false;
    else if (this.expect(d.comma), this.afterTrailingComma(d.braceR)) break;
    t.push(this.parseExportSpecifier(e));
  }
  return t;
};
oe.parseImport = function(e) {
  return this.next(), this.type === d.string ? (e.specifiers = bp, e.source = this.parseExprAtom()) : (e.specifiers = this.parseImportSpecifiers(), this.expectContextual("from"), e.source = this.type === d.string ? this.parseExprAtom() : this.unexpected()), this.options.ecmaVersion >= 16 && (e.attributes = this.parseWithClause()), this.semicolon(), this.finishNode(e, "ImportDeclaration");
};
oe.parseImportSpecifier = function() {
  var e = this.startNode();
  return e.imported = this.parseModuleExportName(), this.eatContextual("as") ? e.local = this.parseIdent() : (this.checkUnreserved(e.imported), e.local = e.imported), this.checkLValSimple(e.local, kt), this.finishNode(e, "ImportSpecifier");
};
oe.parseImportDefaultSpecifier = function() {
  var e = this.startNode();
  return e.local = this.parseIdent(), this.checkLValSimple(e.local, kt), this.finishNode(e, "ImportDefaultSpecifier");
};
oe.parseImportNamespaceSpecifier = function() {
  var e = this.startNode();
  return this.next(), this.expectContextual("as"), e.local = this.parseIdent(), this.checkLValSimple(e.local, kt), this.finishNode(e, "ImportNamespaceSpecifier");
};
oe.parseImportSpecifiers = function() {
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
oe.parseWithClause = function() {
  var e = [];
  if (!this.eat(d._with)) return e;
  this.expect(d.braceL);
  for (var t = {}, r = true; !this.eat(d.braceR); ) {
    if (r) r = false;
    else if (this.expect(d.comma), this.afterTrailingComma(d.braceR)) break;
    var s = this.parseImportAttribute(), o = s.key.type === "Identifier" ? s.key.name : s.key.value;
    ir(t, o) && this.raiseRecoverable(s.key.start, "Duplicate attribute key '" + o + "'"), t[o] = true, e.push(s);
  }
  return e;
};
oe.parseImportAttribute = function() {
  var e = this.startNode();
  return e.key = this.type === d.string ? this.parseExprAtom() : this.parseIdent(this.options.allowReserved !== "never"), this.expect(d.colon), this.type !== d.string && this.unexpected(), e.value = this.parseExprAtom(), this.finishNode(e, "ImportAttribute");
};
oe.parseModuleExportName = function() {
  if (this.options.ecmaVersion >= 13 && this.type === d.string) {
    var e = this.parseLiteral(this.value);
    return pp.test(e.value) && this.raise(e.start, "An export name cannot include a lone surrogate."), e;
  }
  return this.parseIdent(true);
};
oe.adaptDirectivePrologue = function(e) {
  for (var t = 0; t < e.length && this.isDirectiveCandidate(e[t]); ++t) e[t].directive = e[t].expression.raw.slice(1, -1);
};
oe.isDirectiveCandidate = function(e) {
  return this.options.ecmaVersion >= 5 && e.type === "ExpressionStatement" && e.expression.type === "Literal" && typeof e.expression.value == "string" && (this.input[e.start] === '"' || this.input[e.start] === "'");
};
var dt = Be.prototype;
dt.toAssignable = function(e, t, r) {
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
      e.type = "ObjectPattern", r && this.checkPatternErrors(r, true);
      for (var s = 0, o = e.properties; s < o.length; s += 1) {
        var p = o[s];
        this.toAssignable(p, t), p.type === "RestElement" && (p.argument.type === "ArrayPattern" || p.argument.type === "ObjectPattern") && this.raise(p.argument.start, "Unexpected token");
      }
      break;
    case "Property":
      e.kind !== "init" && this.raise(e.key.start, "Object pattern can't contain getter or setter"), this.toAssignable(e.value, t);
      break;
    case "ArrayExpression":
      e.type = "ArrayPattern", r && this.checkPatternErrors(r, true), this.toAssignableList(e.elements, t);
      break;
    case "SpreadElement":
      e.type = "RestElement", this.toAssignable(e.argument, t), e.argument.type === "AssignmentPattern" && this.raise(e.argument.start, "Rest elements cannot have a default value");
      break;
    case "AssignmentExpression":
      e.operator !== "=" && this.raise(e.left.end, "Only '=' operator can be used for specifying default value."), e.type = "AssignmentPattern", delete e.operator, this.toAssignable(e.left, t);
      break;
    case "ParenthesizedExpression":
      this.toAssignable(e.expression, t, r);
      break;
    case "ChainExpression":
      this.raiseRecoverable(e.start, "Optional chaining cannot appear in left-hand side");
      break;
    case "MemberExpression":
      if (!t) break;
    default:
      this.raise(e.start, "Assigning to rvalue");
  }
  else r && this.checkPatternErrors(r, true);
  return e;
};
dt.toAssignableList = function(e, t) {
  for (var r = e.length, s = 0; s < r; s++) {
    var o = e[s];
    o && this.toAssignable(o, t);
  }
  if (r) {
    var p = e[r - 1];
    this.options.ecmaVersion === 6 && t && p && p.type === "RestElement" && p.argument.type !== "Identifier" && this.unexpected(p.argument.start);
  }
  return e;
};
dt.parseSpread = function(e) {
  var t = this.startNode();
  return this.next(), t.argument = this.parseMaybeAssign(false, e), this.finishNode(t, "SpreadElement");
};
dt.parseRestBinding = function() {
  var e = this.startNode();
  return this.next(), this.options.ecmaVersion === 6 && this.type !== d.name && this.unexpected(), e.argument = this.parseBindingAtom(), this.finishNode(e, "RestElement");
};
dt.parseBindingAtom = function() {
  if (this.options.ecmaVersion >= 6) switch (this.type) {
    case d.bracketL:
      var e = this.startNode();
      return this.next(), e.elements = this.parseBindingList(d.bracketR, true, true), this.finishNode(e, "ArrayPattern");
    case d.braceL:
      return this.parseObj(true);
  }
  return this.parseIdent();
};
dt.parseBindingList = function(e, t, r, s) {
  for (var o = [], p = true; !this.eat(e); ) if (p ? p = false : this.expect(d.comma), t && this.type === d.comma) o.push(null);
  else {
    if (r && this.afterTrailingComma(e)) break;
    if (this.type === d.ellipsis) {
      var h = this.parseRestBinding();
      this.parseBindingListItem(h), o.push(h), this.type === d.comma && this.raiseRecoverable(this.start, "Comma is not permitted after the rest element"), this.expect(e);
      break;
    } else o.push(this.parseAssignableListItem(s));
  }
  return o;
};
dt.parseAssignableListItem = function(e) {
  var t = this.parseMaybeDefault(this.start, this.startLoc);
  return this.parseBindingListItem(t), t;
};
dt.parseBindingListItem = function(e) {
  return e;
};
dt.parseMaybeDefault = function(e, t, r) {
  if (r = r || this.parseBindingAtom(), this.options.ecmaVersion < 6 || !this.eat(d.eq)) return r;
  var s = this.startNodeAt(e, t);
  return s.left = r, s.right = this.parseMaybeAssign(), this.finishNode(s, "AssignmentPattern");
};
dt.checkLValSimple = function(e, t, r) {
  t === void 0 && (t = Vr);
  var s = t !== Vr;
  switch (e.type) {
    case "Identifier":
      this.strict && this.reservedWordsStrictBind.test(e.name) && this.raiseRecoverable(e.start, (s ? "Binding " : "Assigning to ") + e.name + " in strict mode"), s && (t === kt && e.name === "let" && this.raiseRecoverable(e.start, "let is disallowed as a lexically bound name"), r && (ir(r, e.name) && this.raiseRecoverable(e.start, "Argument name clash"), r[e.name] = true), t !== Pc && this.declareName(e.name, t, e.start));
      break;
    case "ChainExpression":
      this.raiseRecoverable(e.start, "Optional chaining cannot appear in left-hand side");
      break;
    case "MemberExpression":
      s && this.raiseRecoverable(e.start, "Binding member expression");
      break;
    case "ParenthesizedExpression":
      return s && this.raiseRecoverable(e.start, "Binding parenthesized expression"), this.checkLValSimple(e.expression, t, r);
    default:
      this.raise(e.start, (s ? "Binding" : "Assigning to") + " rvalue");
  }
};
dt.checkLValPattern = function(e, t, r) {
  switch (t === void 0 && (t = Vr), e.type) {
    case "ObjectPattern":
      for (var s = 0, o = e.properties; s < o.length; s += 1) {
        var p = o[s];
        this.checkLValInnerPattern(p, t, r);
      }
      break;
    case "ArrayPattern":
      for (var h = 0, v = e.elements; h < v.length; h += 1) {
        var m = v[h];
        m && this.checkLValInnerPattern(m, t, r);
      }
      break;
    default:
      this.checkLValSimple(e, t, r);
  }
};
dt.checkLValInnerPattern = function(e, t, r) {
  switch (t === void 0 && (t = Vr), e.type) {
    case "Property":
      this.checkLValInnerPattern(e.value, t, r);
      break;
    case "AssignmentPattern":
      this.checkLValPattern(e.left, t, r);
      break;
    case "RestElement":
      this.checkLValPattern(e.argument, t, r);
      break;
    default:
      this.checkLValPattern(e, t, r);
  }
};
var Ke = function(t, r, s, o, p) {
  this.token = t, this.isExpr = !!r, this.preserveSpace = !!s, this.override = o, this.generator = !!p;
}, De = { b_stat: new Ke("{", false), b_expr: new Ke("{", true), b_tmpl: new Ke("${", false), p_stat: new Ke("(", false), p_expr: new Ke("(", true), q_tmpl: new Ke("`", true, true, function(e) {
  return e.tryReadTemplateToken();
}), f_stat: new Ke("function", false), f_expr: new Ke("function", true), f_expr_gen: new Ke("function", true, false, null, true), f_gen: new Ke("function", false, false, null, true) }, nr = Be.prototype;
nr.initialContext = function() {
  return [De.b_stat];
};
nr.curContext = function() {
  return this.context[this.context.length - 1];
};
nr.braceIsBlock = function(e) {
  var t = this.curContext();
  return t === De.f_expr || t === De.f_stat ? true : e === d.colon && (t === De.b_stat || t === De.b_expr) ? !t.isExpr : e === d._return || e === d.name && this.exprAllowed ? at.test(this.input.slice(this.lastTokEnd, this.start)) : e === d._else || e === d.semi || e === d.eof || e === d.parenR || e === d.arrow ? true : e === d.braceL ? t === De.b_stat : e === d._var || e === d._const || e === d.name ? false : !this.exprAllowed;
};
nr.inGeneratorContext = function() {
  for (var e = this.context.length - 1; e >= 1; e--) {
    var t = this.context[e];
    if (t.token === "function") return t.generator;
  }
  return false;
};
nr.updateContext = function(e) {
  var t, r = this.type;
  r.keyword && e === d.dot ? this.exprAllowed = false : (t = r.updateContext) ? t.call(this, e) : this.exprAllowed = r.beforeExpr;
};
nr.overrideContext = function(e) {
  this.curContext() !== e && (this.context[this.context.length - 1] = e);
};
d.parenR.updateContext = d.braceR.updateContext = function() {
  if (this.context.length === 1) {
    this.exprAllowed = true;
    return;
  }
  var e = this.context.pop();
  e === De.b_stat && this.curContext().token === "function" && (e = this.context.pop()), this.exprAllowed = !e.isExpr;
};
d.braceL.updateContext = function(e) {
  this.context.push(this.braceIsBlock(e) ? De.b_stat : De.b_expr), this.exprAllowed = true;
};
d.dollarBraceL.updateContext = function() {
  this.context.push(De.b_tmpl), this.exprAllowed = true;
};
d.parenL.updateContext = function(e) {
  var t = e === d._if || e === d._for || e === d._with || e === d._while;
  this.context.push(t ? De.p_stat : De.p_expr), this.exprAllowed = true;
};
d.incDec.updateContext = function() {
};
d._function.updateContext = d._class.updateContext = function(e) {
  e.beforeExpr && e !== d._else && !(e === d.semi && this.curContext() !== De.p_stat) && !(e === d._return && at.test(this.input.slice(this.lastTokEnd, this.start))) && !((e === d.colon || e === d.braceL) && this.curContext() === De.b_stat) ? this.context.push(De.f_expr) : this.context.push(De.f_stat), this.exprAllowed = false;
};
d.colon.updateContext = function() {
  this.curContext().token === "function" && this.context.pop(), this.exprAllowed = true;
};
d.backQuote.updateContext = function() {
  this.curContext() === De.q_tmpl ? this.context.pop() : this.context.push(De.q_tmpl), this.exprAllowed = false;
};
d.star.updateContext = function(e) {
  if (e === d._function) {
    var t = this.context.length - 1;
    this.context[t] === De.f_expr ? this.context[t] = De.f_expr_gen : this.context[t] = De.f_gen;
  }
  this.exprAllowed = true;
};
d.name.updateContext = function(e) {
  var t = false;
  this.options.ecmaVersion >= 6 && e !== d.dot && (this.value === "of" && !this.exprAllowed || this.value === "yield" && this.inGeneratorContext()) && (t = true), this.exprAllowed = t;
};
var ve = Be.prototype;
ve.checkPropClash = function(e, t, r) {
  if (!(this.options.ecmaVersion >= 9 && e.type === "SpreadElement") && !(this.options.ecmaVersion >= 6 && (e.computed || e.method || e.shorthand))) {
    var s = e.key, o;
    switch (s.type) {
      case "Identifier":
        o = s.name;
        break;
      case "Literal":
        o = String(s.value);
        break;
      default:
        return;
    }
    var p = e.kind;
    if (this.options.ecmaVersion >= 6) {
      o === "__proto__" && p === "init" && (t.proto && (r ? r.doubleProto < 0 && (r.doubleProto = s.start) : this.raiseRecoverable(s.start, "Redefinition of __proto__ property")), t.proto = true);
      return;
    }
    o = "$" + o;
    var h = t[o];
    if (h) {
      var v;
      p === "init" ? v = this.strict && h.init || h.get || h.set : v = h.init || h[p], v && this.raiseRecoverable(s.start, "Redefinition of property");
    } else h = t[o] = { init: false, get: false, set: false };
    h[p] = true;
  }
};
ve.parseExpression = function(e, t) {
  var r = this.start, s = this.startLoc, o = this.parseMaybeAssign(e, t);
  if (this.type === d.comma) {
    var p = this.startNodeAt(r, s);
    for (p.expressions = [o]; this.eat(d.comma); ) p.expressions.push(this.parseMaybeAssign(e, t));
    return this.finishNode(p, "SequenceExpression");
  }
  return o;
};
ve.parseMaybeAssign = function(e, t, r) {
  if (this.isContextual("yield")) {
    if (this.inGenerator) return this.parseYield(e);
    this.exprAllowed = false;
  }
  var s = false, o = -1, p = -1, h = -1;
  t ? (o = t.parenthesizedAssign, p = t.trailingComma, h = t.doubleProto, t.parenthesizedAssign = t.trailingComma = -1) : (t = new Kr(), s = true);
  var v = this.start, m = this.startLoc;
  (this.type === d.parenL || this.type === d.name) && (this.potentialArrowAt = this.start, this.potentialArrowInForAwait = e === "await");
  var l = this.parseMaybeConditional(e, t);
  if (r && (l = r.call(this, l, v, m)), this.type.isAssign) {
    var w = this.startNodeAt(v, m);
    return w.operator = this.value, this.type === d.eq && (l = this.toAssignable(l, false, t)), s || (t.parenthesizedAssign = t.trailingComma = t.doubleProto = -1), t.shorthandAssign >= l.start && (t.shorthandAssign = -1), this.type === d.eq ? this.checkLValPattern(l) : this.checkLValSimple(l), w.left = l, this.next(), w.right = this.parseMaybeAssign(e), h > -1 && (t.doubleProto = h), this.finishNode(w, "AssignmentExpression");
  } else s && this.checkExpressionErrors(t, true);
  return o > -1 && (t.parenthesizedAssign = o), p > -1 && (t.trailingComma = p), l;
};
ve.parseMaybeConditional = function(e, t) {
  var r = this.start, s = this.startLoc, o = this.parseExprOps(e, t);
  if (this.checkExpressionErrors(t)) return o;
  if (this.eat(d.question)) {
    var p = this.startNodeAt(r, s);
    return p.test = o, p.consequent = this.parseMaybeAssign(), this.expect(d.colon), p.alternate = this.parseMaybeAssign(e), this.finishNode(p, "ConditionalExpression");
  }
  return o;
};
ve.parseExprOps = function(e, t) {
  var r = this.start, s = this.startLoc, o = this.parseMaybeUnary(t, false, false, e);
  return this.checkExpressionErrors(t) || o.start === r && o.type === "ArrowFunctionExpression" ? o : this.parseExprOp(o, r, s, -1, e);
};
ve.parseExprOp = function(e, t, r, s, o) {
  var p = this.type.binop;
  if (p != null && (!o || this.type !== d._in) && p > s) {
    var h = this.type === d.logicalOR || this.type === d.logicalAND, v = this.type === d.coalesce;
    v && (p = d.logicalAND.binop);
    var m = this.value;
    this.next();
    var l = this.start, w = this.startLoc, U = this.parseExprOp(this.parseMaybeUnary(null, false, false, o), l, w, p, o), J = this.buildBinary(t, r, e, U, m, h || v);
    return (h && this.type === d.coalesce || v && (this.type === d.logicalOR || this.type === d.logicalAND)) && this.raiseRecoverable(this.start, "Logical expressions and coalesce expressions cannot be mixed. Wrap either by parentheses"), this.parseExprOp(J, t, r, s, o);
  }
  return e;
};
ve.buildBinary = function(e, t, r, s, o, p) {
  s.type === "PrivateIdentifier" && this.raise(s.start, "Private identifier can only be left side of binary expression");
  var h = this.startNodeAt(e, t);
  return h.left = r, h.operator = o, h.right = s, this.finishNode(h, p ? "LogicalExpression" : "BinaryExpression");
};
ve.parseMaybeUnary = function(e, t, r, s) {
  var o = this.start, p = this.startLoc, h;
  if (this.isContextual("await") && this.canAwait) h = this.parseAwait(s), t = true;
  else if (this.type.prefix) {
    var v = this.startNode(), m = this.type === d.incDec;
    v.operator = this.value, v.prefix = true, this.next(), v.argument = this.parseMaybeUnary(null, true, m, s), this.checkExpressionErrors(e, true), m ? this.checkLValSimple(v.argument) : this.strict && v.operator === "delete" && Tc(v.argument) ? this.raiseRecoverable(v.start, "Deleting local variable in strict mode") : v.operator === "delete" && Ll(v.argument) ? this.raiseRecoverable(v.start, "Private fields can not be deleted") : t = true, h = this.finishNode(v, m ? "UpdateExpression" : "UnaryExpression");
  } else if (!t && this.type === d.privateId) (s || this.privateNameStack.length === 0) && this.options.checkPrivateFields && this.unexpected(), h = this.parsePrivateIdent(), this.type !== d._in && this.unexpected();
  else {
    if (h = this.parseExprSubscripts(e, s), this.checkExpressionErrors(e)) return h;
    for (; this.type.postfix && !this.canInsertSemicolon(); ) {
      var l = this.startNodeAt(o, p);
      l.operator = this.value, l.prefix = false, l.argument = h, this.checkLValSimple(h), this.next(), h = this.finishNode(l, "UpdateExpression");
    }
  }
  if (!r && this.eat(d.starstar)) if (t) this.unexpected(this.lastTokStart);
  else return this.buildBinary(o, p, h, this.parseMaybeUnary(null, false, false, s), "**", false);
  else return h;
};
function Tc(e) {
  return e.type === "Identifier" || e.type === "ParenthesizedExpression" && Tc(e.expression);
}
function Ll(e) {
  return e.type === "MemberExpression" && e.property.type === "PrivateIdentifier" || e.type === "ChainExpression" && Ll(e.expression) || e.type === "ParenthesizedExpression" && Ll(e.expression);
}
ve.parseExprSubscripts = function(e, t) {
  var r = this.start, s = this.startLoc, o = this.parseExprAtom(e, t);
  if (o.type === "ArrowFunctionExpression" && this.input.slice(this.lastTokStart, this.lastTokEnd) !== ")") return o;
  var p = this.parseSubscripts(o, r, s, false, t);
  return e && p.type === "MemberExpression" && (e.parenthesizedAssign >= p.start && (e.parenthesizedAssign = -1), e.parenthesizedBind >= p.start && (e.parenthesizedBind = -1), e.trailingComma >= p.start && (e.trailingComma = -1)), p;
};
ve.parseSubscripts = function(e, t, r, s, o) {
  for (var p = this.options.ecmaVersion >= 8 && e.type === "Identifier" && e.name === "async" && this.lastTokEnd === e.end && !this.canInsertSemicolon() && e.end - e.start === 5 && this.potentialArrowAt === e.start, h = false; ; ) {
    var v = this.parseSubscript(e, t, r, s, p, h, o);
    if (v.optional && (h = true), v === e || v.type === "ArrowFunctionExpression") {
      if (h) {
        var m = this.startNodeAt(t, r);
        m.expression = v, v = this.finishNode(m, "ChainExpression");
      }
      return v;
    }
    e = v;
  }
};
ve.shouldParseAsyncArrow = function() {
  return !this.canInsertSemicolon() && this.eat(d.arrow);
};
ve.parseSubscriptAsyncArrow = function(e, t, r, s) {
  return this.parseArrowExpression(this.startNodeAt(e, t), r, true, s);
};
ve.parseSubscript = function(e, t, r, s, o, p, h) {
  var v = this.options.ecmaVersion >= 11, m = v && this.eat(d.questionDot);
  s && m && this.raise(this.lastTokStart, "Optional chaining cannot appear in the callee of new expressions");
  var l = this.eat(d.bracketL);
  if (l || m && this.type !== d.parenL && this.type !== d.backQuote || this.eat(d.dot)) {
    var w = this.startNodeAt(t, r);
    w.object = e, l ? (w.property = this.parseExpression(), this.expect(d.bracketR)) : this.type === d.privateId && e.type !== "Super" ? w.property = this.parsePrivateIdent() : w.property = this.parseIdent(this.options.allowReserved !== "never"), w.computed = !!l, v && (w.optional = m), e = this.finishNode(w, "MemberExpression");
  } else if (!s && this.eat(d.parenL)) {
    var U = new Kr(), J = this.yieldPos, z = this.awaitPos, K = this.awaitIdentPos;
    this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0;
    var Z = this.parseExprList(d.parenR, this.options.ecmaVersion >= 8, false, U);
    if (o && !m && this.shouldParseAsyncArrow()) return this.checkPatternErrors(U, false), this.checkYieldAwaitInDefaultParams(), this.awaitIdentPos > 0 && this.raise(this.awaitIdentPos, "Cannot use 'await' as identifier inside an async function"), this.yieldPos = J, this.awaitPos = z, this.awaitIdentPos = K, this.parseSubscriptAsyncArrow(t, r, Z, h);
    this.checkExpressionErrors(U, true), this.yieldPos = J || this.yieldPos, this.awaitPos = z || this.awaitPos, this.awaitIdentPos = K || this.awaitIdentPos;
    var B = this.startNodeAt(t, r);
    B.callee = e, B.arguments = Z, v && (B.optional = m), e = this.finishNode(B, "CallExpression");
  } else if (this.type === d.backQuote) {
    (m || p) && this.raise(this.start, "Optional chaining cannot appear in the tag of tagged template expressions");
    var $ = this.startNodeAt(t, r);
    $.tag = e, $.quasi = this.parseTemplate({ isTagged: true }), e = this.finishNode($, "TaggedTemplateExpression");
  }
  return e;
};
ve.parseExprAtom = function(e, t, r) {
  this.type === d.slash && this.readRegexp();
  var s, o = this.potentialArrowAt === this.start;
  switch (this.type) {
    case d._super:
      return this.allowSuper || this.raise(this.start, "'super' keyword outside a method"), s = this.startNode(), this.next(), this.type === d.parenL && !this.allowDirectSuper && this.raise(s.start, "super() call outside constructor of a subclass"), this.type !== d.dot && this.type !== d.bracketL && this.type !== d.parenL && this.unexpected(), this.finishNode(s, "Super");
    case d._this:
      return s = this.startNode(), this.next(), this.finishNode(s, "ThisExpression");
    case d.name:
      var p = this.start, h = this.startLoc, v = this.containsEsc, m = this.parseIdent(false);
      if (this.options.ecmaVersion >= 8 && !v && m.name === "async" && !this.canInsertSemicolon() && this.eat(d._function)) return this.overrideContext(De.f_expr), this.parseFunction(this.startNodeAt(p, h), 0, false, true, t);
      if (o && !this.canInsertSemicolon()) {
        if (this.eat(d.arrow)) return this.parseArrowExpression(this.startNodeAt(p, h), [m], false, t);
        if (this.options.ecmaVersion >= 8 && m.name === "async" && this.type === d.name && !v && (!this.potentialArrowInForAwait || this.value !== "of" || this.containsEsc)) return m = this.parseIdent(false), (this.canInsertSemicolon() || !this.eat(d.arrow)) && this.unexpected(), this.parseArrowExpression(this.startNodeAt(p, h), [m], true, t);
      }
      return m;
    case d.regexp:
      var l = this.value;
      return s = this.parseLiteral(l.value), s.regex = { pattern: l.pattern, flags: l.flags }, s;
    case d.num:
    case d.string:
      return this.parseLiteral(this.value);
    case d._null:
    case d._true:
    case d._false:
      return s = this.startNode(), s.value = this.type === d._null ? null : this.type === d._true, s.raw = this.type.keyword, this.next(), this.finishNode(s, "Literal");
    case d.parenL:
      var w = this.start, U = this.parseParenAndDistinguishExpression(o, t);
      return e && (e.parenthesizedAssign < 0 && !this.isSimpleAssignTarget(U) && (e.parenthesizedAssign = w), e.parenthesizedBind < 0 && (e.parenthesizedBind = w)), U;
    case d.bracketL:
      return s = this.startNode(), this.next(), s.elements = this.parseExprList(d.bracketR, true, true, e), this.finishNode(s, "ArrayExpression");
    case d.braceL:
      return this.overrideContext(De.b_expr), this.parseObj(false, e);
    case d._function:
      return s = this.startNode(), this.next(), this.parseFunction(s, 0);
    case d._class:
      return this.parseClass(this.startNode(), false);
    case d._new:
      return this.parseNew();
    case d.backQuote:
      return this.parseTemplate();
    case d._import:
      return this.options.ecmaVersion >= 11 ? this.parseExprImport(r) : this.unexpected();
    default:
      return this.parseExprAtomDefault();
  }
};
ve.parseExprAtomDefault = function() {
  this.unexpected();
};
ve.parseExprImport = function(e) {
  var t = this.startNode();
  if (this.containsEsc && this.raiseRecoverable(this.start, "Escape sequence in keyword import"), this.next(), this.type === d.parenL && !e) return this.parseDynamicImport(t);
  if (this.type === d.dot) {
    var r = this.startNodeAt(t.start, t.loc && t.loc.start);
    return r.name = "import", t.meta = this.finishNode(r, "Identifier"), this.parseImportMeta(t);
  } else this.unexpected();
};
ve.parseDynamicImport = function(e) {
  if (this.next(), e.source = this.parseMaybeAssign(), this.options.ecmaVersion >= 16) this.eat(d.parenR) ? e.options = null : (this.expect(d.comma), this.afterTrailingComma(d.parenR) ? e.options = null : (e.options = this.parseMaybeAssign(), this.eat(d.parenR) || (this.expect(d.comma), this.afterTrailingComma(d.parenR) || this.unexpected())));
  else if (!this.eat(d.parenR)) {
    var t = this.start;
    this.eat(d.comma) && this.eat(d.parenR) ? this.raiseRecoverable(t, "Trailing comma is not allowed in import()") : this.unexpected(t);
  }
  return this.finishNode(e, "ImportExpression");
};
ve.parseImportMeta = function(e) {
  this.next();
  var t = this.containsEsc;
  return e.property = this.parseIdent(true), e.property.name !== "meta" && this.raiseRecoverable(e.property.start, "The only valid meta property for import is 'import.meta'"), t && this.raiseRecoverable(e.start, "'import.meta' must not contain escaped characters"), this.options.sourceType !== "module" && !this.options.allowImportExportEverywhere && this.raiseRecoverable(e.start, "Cannot use 'import.meta' outside a module"), this.finishNode(e, "MetaProperty");
};
ve.parseLiteral = function(e) {
  var t = this.startNode();
  return t.value = e, t.raw = this.input.slice(this.start, this.end), t.raw.charCodeAt(t.raw.length - 1) === 110 && (t.bigint = t.raw.slice(0, -1).replace(/_/g, "")), this.next(), this.finishNode(t, "Literal");
};
ve.parseParenExpression = function() {
  this.expect(d.parenL);
  var e = this.parseExpression();
  return this.expect(d.parenR), e;
};
ve.shouldParseArrow = function(e) {
  return !this.canInsertSemicolon();
};
ve.parseParenAndDistinguishExpression = function(e, t) {
  var r = this.start, s = this.startLoc, o, p = this.options.ecmaVersion >= 8;
  if (this.options.ecmaVersion >= 6) {
    this.next();
    var h = this.start, v = this.startLoc, m = [], l = true, w = false, U = new Kr(), J = this.yieldPos, z = this.awaitPos, K;
    for (this.yieldPos = 0, this.awaitPos = 0; this.type !== d.parenR; ) if (l ? l = false : this.expect(d.comma), p && this.afterTrailingComma(d.parenR, true)) {
      w = true;
      break;
    } else if (this.type === d.ellipsis) {
      K = this.start, m.push(this.parseParenItem(this.parseRestBinding())), this.type === d.comma && this.raiseRecoverable(this.start, "Comma is not permitted after the rest element");
      break;
    } else m.push(this.parseMaybeAssign(false, U, this.parseParenItem));
    var Z = this.lastTokEnd, B = this.lastTokEndLoc;
    if (this.expect(d.parenR), e && this.shouldParseArrow(m) && this.eat(d.arrow)) return this.checkPatternErrors(U, false), this.checkYieldAwaitInDefaultParams(), this.yieldPos = J, this.awaitPos = z, this.parseParenArrowList(r, s, m, t);
    (!m.length || w) && this.unexpected(this.lastTokStart), K && this.unexpected(K), this.checkExpressionErrors(U, true), this.yieldPos = J || this.yieldPos, this.awaitPos = z || this.awaitPos, m.length > 1 ? (o = this.startNodeAt(h, v), o.expressions = m, this.finishNodeAt(o, "SequenceExpression", Z, B)) : o = m[0];
  } else o = this.parseParenExpression();
  if (this.options.preserveParens) {
    var $ = this.startNodeAt(r, s);
    return $.expression = o, this.finishNode($, "ParenthesizedExpression");
  } else return o;
};
ve.parseParenItem = function(e) {
  return e;
};
ve.parseParenArrowList = function(e, t, r, s) {
  return this.parseArrowExpression(this.startNodeAt(e, t), r, false, s);
};
var gp = [];
ve.parseNew = function() {
  this.containsEsc && this.raiseRecoverable(this.start, "Escape sequence in keyword new");
  var e = this.startNode();
  if (this.next(), this.options.ecmaVersion >= 6 && this.type === d.dot) {
    var t = this.startNodeAt(e.start, e.loc && e.loc.start);
    t.name = "new", e.meta = this.finishNode(t, "Identifier"), this.next();
    var r = this.containsEsc;
    return e.property = this.parseIdent(true), e.property.name !== "target" && this.raiseRecoverable(e.property.start, "The only valid meta property for new is 'new.target'"), r && this.raiseRecoverable(e.start, "'new.target' must not contain escaped characters"), this.allowNewDotTarget || this.raiseRecoverable(e.start, "'new.target' can only be used in functions and class static block"), this.finishNode(e, "MetaProperty");
  }
  var s = this.start, o = this.startLoc;
  return e.callee = this.parseSubscripts(this.parseExprAtom(null, false, true), s, o, true, false), this.eat(d.parenL) ? e.arguments = this.parseExprList(d.parenR, this.options.ecmaVersion >= 8, false) : e.arguments = gp, this.finishNode(e, "NewExpression");
};
ve.parseTemplateElement = function(e) {
  var t = e.isTagged, r = this.startNode();
  return this.type === d.invalidTemplate ? (t || this.raiseRecoverable(this.start, "Bad escape sequence in untagged template literal"), r.value = { raw: this.value.replace(/\r\n?/g, `
`), cooked: null }) : r.value = { raw: this.input.slice(this.start, this.end).replace(/\r\n?/g, `
`), cooked: this.value }, this.next(), r.tail = this.type === d.backQuote, this.finishNode(r, "TemplateElement");
};
ve.parseTemplate = function(e) {
  e === void 0 && (e = {});
  var t = e.isTagged;
  t === void 0 && (t = false);
  var r = this.startNode();
  this.next(), r.expressions = [];
  var s = this.parseTemplateElement({ isTagged: t });
  for (r.quasis = [s]; !s.tail; ) this.type === d.eof && this.raise(this.pos, "Unterminated template literal"), this.expect(d.dollarBraceL), r.expressions.push(this.parseExpression()), this.expect(d.braceR), r.quasis.push(s = this.parseTemplateElement({ isTagged: t }));
  return this.next(), this.finishNode(r, "TemplateLiteral");
};
ve.isAsyncProp = function(e) {
  return !e.computed && e.key.type === "Identifier" && e.key.name === "async" && (this.type === d.name || this.type === d.num || this.type === d.string || this.type === d.bracketL || this.type.keyword || this.options.ecmaVersion >= 9 && this.type === d.star) && !at.test(this.input.slice(this.lastTokEnd, this.start));
};
ve.parseObj = function(e, t) {
  var r = this.startNode(), s = true, o = {};
  for (r.properties = [], this.next(); !this.eat(d.braceR); ) {
    if (s) s = false;
    else if (this.expect(d.comma), this.options.ecmaVersion >= 5 && this.afterTrailingComma(d.braceR)) break;
    var p = this.parseProperty(e, t);
    e || this.checkPropClash(p, o, t), r.properties.push(p);
  }
  return this.finishNode(r, e ? "ObjectPattern" : "ObjectExpression");
};
ve.parseProperty = function(e, t) {
  var r = this.startNode(), s, o, p, h;
  if (this.options.ecmaVersion >= 9 && this.eat(d.ellipsis)) return e ? (r.argument = this.parseIdent(false), this.type === d.comma && this.raiseRecoverable(this.start, "Comma is not permitted after the rest element"), this.finishNode(r, "RestElement")) : (r.argument = this.parseMaybeAssign(false, t), this.type === d.comma && t && t.trailingComma < 0 && (t.trailingComma = this.start), this.finishNode(r, "SpreadElement"));
  this.options.ecmaVersion >= 6 && (r.method = false, r.shorthand = false, (e || t) && (p = this.start, h = this.startLoc), e || (s = this.eat(d.star)));
  var v = this.containsEsc;
  return this.parsePropertyName(r), !e && !v && this.options.ecmaVersion >= 8 && !s && this.isAsyncProp(r) ? (o = true, s = this.options.ecmaVersion >= 9 && this.eat(d.star), this.parsePropertyName(r)) : o = false, this.parsePropertyValue(r, e, s, o, p, h, t, v), this.finishNode(r, "Property");
};
ve.parseGetterSetter = function(e) {
  e.kind = e.key.name, this.parsePropertyName(e), e.value = this.parseMethod(false);
  var t = e.kind === "get" ? 0 : 1;
  if (e.value.params.length !== t) {
    var r = e.value.start;
    e.kind === "get" ? this.raiseRecoverable(r, "getter should have no params") : this.raiseRecoverable(r, "setter should have exactly one param");
  } else e.kind === "set" && e.value.params[0].type === "RestElement" && this.raiseRecoverable(e.value.params[0].start, "Setter cannot use rest params");
};
ve.parsePropertyValue = function(e, t, r, s, o, p, h, v) {
  (r || s) && this.type === d.colon && this.unexpected(), this.eat(d.colon) ? (e.value = t ? this.parseMaybeDefault(this.start, this.startLoc) : this.parseMaybeAssign(false, h), e.kind = "init") : this.options.ecmaVersion >= 6 && this.type === d.parenL ? (t && this.unexpected(), e.kind = "init", e.method = true, e.value = this.parseMethod(r, s)) : !t && !v && this.options.ecmaVersion >= 5 && !e.computed && e.key.type === "Identifier" && (e.key.name === "get" || e.key.name === "set") && this.type !== d.comma && this.type !== d.braceR && this.type !== d.eq ? ((r || s) && this.unexpected(), this.parseGetterSetter(e)) : this.options.ecmaVersion >= 6 && !e.computed && e.key.type === "Identifier" ? ((r || s) && this.unexpected(), this.checkUnreserved(e.key), e.key.name === "await" && !this.awaitIdentPos && (this.awaitIdentPos = o), e.kind = "init", t ? e.value = this.parseMaybeDefault(o, p, this.copyNode(e.key)) : this.type === d.eq && h ? (h.shorthandAssign < 0 && (h.shorthandAssign = this.start), e.value = this.parseMaybeDefault(o, p, this.copyNode(e.key))) : e.value = this.copyNode(e.key), e.shorthand = true) : this.unexpected();
};
ve.parsePropertyName = function(e) {
  if (this.options.ecmaVersion >= 6) {
    if (this.eat(d.bracketL)) return e.computed = true, e.key = this.parseMaybeAssign(), this.expect(d.bracketR), e.key;
    e.computed = false;
  }
  return e.key = this.type === d.num || this.type === d.string ? this.parseExprAtom() : this.parseIdent(this.options.allowReserved !== "never");
};
ve.initFunction = function(e) {
  e.id = null, this.options.ecmaVersion >= 6 && (e.generator = e.expression = false), this.options.ecmaVersion >= 8 && (e.async = false);
};
ve.parseMethod = function(e, t, r) {
  var s = this.startNode(), o = this.yieldPos, p = this.awaitPos, h = this.awaitIdentPos;
  return this.initFunction(s), this.options.ecmaVersion >= 6 && (s.generator = e), this.options.ecmaVersion >= 8 && (s.async = !!t), this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0, this.enterScope(iu(t, s.generator) | ru | (r ? _c : 0)), this.expect(d.parenL), s.params = this.parseBindingList(d.parenR, false, this.options.ecmaVersion >= 8), this.checkYieldAwaitInDefaultParams(), this.parseFunctionBody(s, false, true, false), this.yieldPos = o, this.awaitPos = p, this.awaitIdentPos = h, this.finishNode(s, "FunctionExpression");
};
ve.parseArrowExpression = function(e, t, r, s) {
  var o = this.yieldPos, p = this.awaitPos, h = this.awaitIdentPos;
  return this.enterScope(iu(r, false) | gc), this.initFunction(e), this.options.ecmaVersion >= 8 && (e.async = !!r), this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0, e.params = this.toAssignableList(t, true), this.parseFunctionBody(e, true, false, s), this.yieldPos = o, this.awaitPos = p, this.awaitIdentPos = h, this.finishNode(e, "ArrowFunctionExpression");
};
ve.parseFunctionBody = function(e, t, r, s) {
  var o = t && this.type !== d.braceL, p = this.strict, h = false;
  if (o) e.body = this.parseMaybeAssign(s), e.expression = true, this.checkParams(e, false);
  else {
    var v = this.options.ecmaVersion >= 7 && !this.isSimpleParamList(e.params);
    (!p || v) && (h = this.strictDirective(this.end), h && v && this.raiseRecoverable(e.start, "Illegal 'use strict' directive in function with non-simple parameter list"));
    var m = this.labels;
    this.labels = [], h && (this.strict = true), this.checkParams(e, !p && !h && !t && !r && this.isSimpleParamList(e.params)), this.strict && e.id && this.checkLValSimple(e.id, Pc), e.body = this.parseBlock(false, void 0, h && !p), e.expression = false, this.adaptDirectivePrologue(e.body.body), this.labels = m;
  }
  this.exitScope();
};
ve.isSimpleParamList = function(e) {
  for (var t = 0, r = e; t < r.length; t += 1) {
    var s = r[t];
    if (s.type !== "Identifier") return false;
  }
  return true;
};
ve.checkParams = function(e, t) {
  for (var r = /* @__PURE__ */ Object.create(null), s = 0, o = e.params; s < o.length; s += 1) {
    var p = o[s];
    this.checkLValInnerPattern(p, su, t ? null : r);
  }
};
ve.parseExprList = function(e, t, r, s) {
  for (var o = [], p = true; !this.eat(e); ) {
    if (p) p = false;
    else if (this.expect(d.comma), t && this.afterTrailingComma(e)) break;
    var h = void 0;
    r && this.type === d.comma ? h = null : this.type === d.ellipsis ? (h = this.parseSpread(s), s && this.type === d.comma && s.trailingComma < 0 && (s.trailingComma = this.start)) : h = this.parseMaybeAssign(false, s), o.push(h);
  }
  return o;
};
ve.checkUnreserved = function(e) {
  var t = e.start, r = e.end, s = e.name;
  if (this.inGenerator && s === "yield" && this.raiseRecoverable(t, "Cannot use 'yield' as identifier inside a generator"), this.inAsync && s === "await" && this.raiseRecoverable(t, "Cannot use 'await' as identifier inside an async function"), this.currentThisScope().inClassFieldInit && s === "arguments" && this.raiseRecoverable(t, "Cannot use 'arguments' in class field initializer"), this.inClassStaticBlock && (s === "arguments" || s === "await") && this.raise(t, "Cannot use " + s + " in class static initialization block"), this.keywords.test(s) && this.raise(t, "Unexpected keyword '" + s + "'"), !(this.options.ecmaVersion < 6 && this.input.slice(t, r).indexOf("\\") !== -1)) {
    var o = this.strict ? this.reservedWordsStrict : this.reservedWords;
    o.test(s) && (!this.inAsync && s === "await" && this.raiseRecoverable(t, "Cannot use keyword 'await' outside an async function"), this.raiseRecoverable(t, "The keyword '" + s + "' is reserved"));
  }
};
ve.parseIdent = function(e) {
  var t = this.parseIdentNode();
  return this.next(!!e), this.finishNode(t, "Identifier"), e || (this.checkUnreserved(t), t.name === "await" && !this.awaitIdentPos && (this.awaitIdentPos = t.start)), t;
};
ve.parseIdentNode = function() {
  var e = this.startNode();
  return this.type === d.name ? e.name = this.value : this.type.keyword ? (e.name = this.type.keyword, (e.name === "class" || e.name === "function") && (this.lastTokEnd !== this.lastTokStart + 1 || this.input.charCodeAt(this.lastTokStart) !== 46) && this.context.pop(), this.type = d.name) : this.unexpected(), e;
};
ve.parsePrivateIdent = function() {
  var e = this.startNode();
  return this.type === d.privateId ? e.name = this.value : this.unexpected(), this.next(), this.finishNode(e, "PrivateIdentifier"), this.options.checkPrivateFields && (this.privateNameStack.length === 0 ? this.raise(e.start, "Private field '#" + e.name + "' must be declared in an enclosing class") : this.privateNameStack[this.privateNameStack.length - 1].used.push(e)), e;
};
ve.parseYield = function(e) {
  this.yieldPos || (this.yieldPos = this.start);
  var t = this.startNode();
  return this.next(), this.type === d.semi || this.canInsertSemicolon() || this.type !== d.star && !this.type.startsExpr ? (t.delegate = false, t.argument = null) : (t.delegate = this.eat(d.star), t.argument = this.parseMaybeAssign(e)), this.finishNode(t, "YieldExpression");
};
ve.parseAwait = function(e) {
  this.awaitPos || (this.awaitPos = this.start);
  var t = this.startNode();
  return this.next(), t.argument = this.parseMaybeUnary(null, true, false, e), this.finishNode(t, "AwaitExpression");
};
var Ur = Be.prototype;
Ur.raise = function(e, t) {
  var r = eu(this.input, e);
  t += " (" + r.line + ":" + r.column + ")";
  var s = new SyntaxError(t);
  throw s.pos = e, s.loc = r, s.raisedAt = this.pos, s;
};
Ur.raiseRecoverable = Ur.raise;
Ur.curPosition = function() {
  if (this.options.locations) return new er(this.curLine, this.pos - this.lineStart);
};
var Nt = Be.prototype, xp = function(t) {
  this.flags = t, this.var = [], this.lexical = [], this.functions = [], this.inClassFieldInit = false;
};
Nt.enterScope = function(e) {
  this.scopeStack.push(new xp(e));
};
Nt.exitScope = function() {
  this.scopeStack.pop();
};
Nt.treatFunctionsAsVarInScope = function(e) {
  return e.flags & sr || !this.inModule && e.flags & yr;
};
Nt.declareName = function(e, t, r) {
  var s = false;
  if (t === kt) {
    var o = this.currentScope();
    s = o.lexical.indexOf(e) > -1 || o.functions.indexOf(e) > -1 || o.var.indexOf(e) > -1, o.lexical.push(e), this.inModule && o.flags & yr && delete this.undefinedExports[e];
  } else if (t === Rc) {
    var p = this.currentScope();
    p.lexical.push(e);
  } else if (t === Cc) {
    var h = this.currentScope();
    this.treatFunctionsAsVar ? s = h.lexical.indexOf(e) > -1 : s = h.lexical.indexOf(e) > -1 || h.var.indexOf(e) > -1, h.functions.push(e);
  } else for (var v = this.scopeStack.length - 1; v >= 0; --v) {
    var m = this.scopeStack[v];
    if (m.lexical.indexOf(e) > -1 && !(m.flags & xc && m.lexical[0] === e) || !this.treatFunctionsAsVarInScope(m) && m.functions.indexOf(e) > -1) {
      s = true;
      break;
    }
    if (m.var.push(e), this.inModule && m.flags & yr && delete this.undefinedExports[e], m.flags & au) break;
  }
  s && this.raiseRecoverable(r, "Identifier '" + e + "' has already been declared");
};
Nt.checkLocalExport = function(e) {
  this.scopeStack[0].lexical.indexOf(e.name) === -1 && this.scopeStack[0].var.indexOf(e.name) === -1 && (this.undefinedExports[e.name] = e);
};
Nt.currentScope = function() {
  return this.scopeStack[this.scopeStack.length - 1];
};
Nt.currentVarScope = function() {
  for (var e = this.scopeStack.length - 1; ; e--) {
    var t = this.scopeStack[e];
    if (t.flags & au) return t;
  }
};
Nt.currentThisScope = function() {
  for (var e = this.scopeStack.length - 1; ; e--) {
    var t = this.scopeStack[e];
    if (t.flags & au && !(t.flags & gc)) return t;
  }
};
var Cr = function(t, r, s) {
  this.type = "", this.start = r, this.end = 0, t.options.locations && (this.loc = new xr(t, s)), t.options.directSourceFile && (this.sourceFile = t.options.directSourceFile), t.options.ranges && (this.range = [r, 0]);
}, Rr = Be.prototype;
Rr.startNode = function() {
  return new Cr(this, this.start, this.startLoc);
};
Rr.startNodeAt = function(e, t) {
  return new Cr(this, e, t);
};
function Sc(e, t, r, s) {
  return e.type = t, e.end = r, this.options.locations && (e.loc.end = s), this.options.ranges && (e.range[1] = r), e;
}
Rr.finishNode = function(e, t) {
  return Sc.call(this, e, t, this.lastTokEnd, this.lastTokEndLoc);
};
Rr.finishNodeAt = function(e, t, r, s) {
  return Sc.call(this, e, t, r, s);
};
Rr.copyNode = function(e) {
  var t = new Cr(this, e.start, this.startLoc);
  for (var r in e) t[r] = e[r];
  return t;
};
var _p = "Gara Garay Gukh Gurung_Khema Hrkt Katakana_Or_Hiragana Kawi Kirat_Rai Krai Nag_Mundari Nagm Ol_Onal Onao Sunu Sunuwar Todhri Todr Tulu_Tigalari Tutg Unknown Zzzz", Ac = "ASCII ASCII_Hex_Digit AHex Alphabetic Alpha Any Assigned Bidi_Control Bidi_C Bidi_Mirrored Bidi_M Case_Ignorable CI Cased Changes_When_Casefolded CWCF Changes_When_Casemapped CWCM Changes_When_Lowercased CWL Changes_When_NFKC_Casefolded CWKCF Changes_When_Titlecased CWT Changes_When_Uppercased CWU Dash Default_Ignorable_Code_Point DI Deprecated Dep Diacritic Dia Emoji Emoji_Component Emoji_Modifier Emoji_Modifier_Base Emoji_Presentation Extender Ext Grapheme_Base Gr_Base Grapheme_Extend Gr_Ext Hex_Digit Hex IDS_Binary_Operator IDSB IDS_Trinary_Operator IDST ID_Continue IDC ID_Start IDS Ideographic Ideo Join_Control Join_C Logical_Order_Exception LOE Lowercase Lower Math Noncharacter_Code_Point NChar Pattern_Syntax Pat_Syn Pattern_White_Space Pat_WS Quotation_Mark QMark Radical Regional_Indicator RI Sentence_Terminal STerm Soft_Dotted SD Terminal_Punctuation Term Unified_Ideograph UIdeo Uppercase Upper Variation_Selector VS White_Space space XID_Continue XIDC XID_Start XIDS", Ec = Ac + " Extended_Pictographic", kc = Ec, Ic = kc + " EBase EComp EMod EPres ExtPict", qc = Ic, Cp = qc, Rp = { 9: Ac, 10: Ec, 11: kc, 12: Ic, 13: qc, 14: Cp }, Pp = "Basic_Emoji Emoji_Keycap_Sequence RGI_Emoji_Modifier_Sequence RGI_Emoji_Flag_Sequence RGI_Emoji_Tag_Sequence RGI_Emoji_ZWJ_Sequence RGI_Emoji", wp = { 9: "", 10: "", 11: "", 12: "", 13: "", 14: Pp }, Pu = "Cased_Letter LC Close_Punctuation Pe Connector_Punctuation Pc Control Cc cntrl Currency_Symbol Sc Dash_Punctuation Pd Decimal_Number Nd digit Enclosing_Mark Me Final_Punctuation Pf Format Cf Initial_Punctuation Pi Letter L Letter_Number Nl Line_Separator Zl Lowercase_Letter Ll Mark M Combining_Mark Math_Symbol Sm Modifier_Letter Lm Modifier_Symbol Sk Nonspacing_Mark Mn Number N Open_Punctuation Ps Other C Other_Letter Lo Other_Number No Other_Punctuation Po Other_Symbol So Paragraph_Separator Zp Private_Use Co Punctuation P punct Separator Z Space_Separator Zs Spacing_Mark Mc Surrogate Cs Symbol S Titlecase_Letter Lt Unassigned Cn Uppercase_Letter Lu", Mc = "Adlam Adlm Ahom Anatolian_Hieroglyphs Hluw Arabic Arab Armenian Armn Avestan Avst Balinese Bali Bamum Bamu Bassa_Vah Bass Batak Batk Bengali Beng Bhaiksuki Bhks Bopomofo Bopo Brahmi Brah Braille Brai Buginese Bugi Buhid Buhd Canadian_Aboriginal Cans Carian Cari Caucasian_Albanian Aghb Chakma Cakm Cham Cham Cherokee Cher Common Zyyy Coptic Copt Qaac Cuneiform Xsux Cypriot Cprt Cyrillic Cyrl Deseret Dsrt Devanagari Deva Duployan Dupl Egyptian_Hieroglyphs Egyp Elbasan Elba Ethiopic Ethi Georgian Geor Glagolitic Glag Gothic Goth Grantha Gran Greek Grek Gujarati Gujr Gurmukhi Guru Han Hani Hangul Hang Hanunoo Hano Hatran Hatr Hebrew Hebr Hiragana Hira Imperial_Aramaic Armi Inherited Zinh Qaai Inscriptional_Pahlavi Phli Inscriptional_Parthian Prti Javanese Java Kaithi Kthi Kannada Knda Katakana Kana Kayah_Li Kali Kharoshthi Khar Khmer Khmr Khojki Khoj Khudawadi Sind Lao Laoo Latin Latn Lepcha Lepc Limbu Limb Linear_A Lina Linear_B Linb Lisu Lisu Lycian Lyci Lydian Lydi Mahajani Mahj Malayalam Mlym Mandaic Mand Manichaean Mani Marchen Marc Masaram_Gondi Gonm Meetei_Mayek Mtei Mende_Kikakui Mend Meroitic_Cursive Merc Meroitic_Hieroglyphs Mero Miao Plrd Modi Mongolian Mong Mro Mroo Multani Mult Myanmar Mymr Nabataean Nbat New_Tai_Lue Talu Newa Newa Nko Nkoo Nushu Nshu Ogham Ogam Ol_Chiki Olck Old_Hungarian Hung Old_Italic Ital Old_North_Arabian Narb Old_Permic Perm Old_Persian Xpeo Old_South_Arabian Sarb Old_Turkic Orkh Oriya Orya Osage Osge Osmanya Osma Pahawh_Hmong Hmng Palmyrene Palm Pau_Cin_Hau Pauc Phags_Pa Phag Phoenician Phnx Psalter_Pahlavi Phlp Rejang Rjng Runic Runr Samaritan Samr Saurashtra Saur Sharada Shrd Shavian Shaw Siddham Sidd SignWriting Sgnw Sinhala Sinh Sora_Sompeng Sora Soyombo Soyo Sundanese Sund Syloti_Nagri Sylo Syriac Syrc Tagalog Tglg Tagbanwa Tagb Tai_Le Tale Tai_Tham Lana Tai_Viet Tavt Takri Takr Tamil Taml Tangut Tang Telugu Telu Thaana Thaa Thai Thai Tibetan Tibt Tifinagh Tfng Tirhuta Tirh Ugaritic Ugar Vai Vaii Warang_Citi Wara Yi Yiii Zanabazar_Square Zanb", Lc = Mc + " Dogra Dogr Gunjala_Gondi Gong Hanifi_Rohingya Rohg Makasar Maka Medefaidrin Medf Old_Sogdian Sogo Sogdian Sogd", Oc = Lc + " Elymaic Elym Nandinagari Nand Nyiakeng_Puachue_Hmong Hmnp Wancho Wcho", Nc = Oc + " Chorasmian Chrs Diak Dives_Akuru Khitan_Small_Script Kits Yezi Yezidi", Dc = Nc + " Cypro_Minoan Cpmn Old_Uyghur Ougr Tangsa Tnsa Toto Vithkuqi Vith", Tp = Dc + " " + _p, Sp = { 9: Mc, 10: Lc, 11: Oc, 12: Nc, 13: Dc, 14: Tp }, jc = {};
function Ap(e) {
  var t = jc[e] = { binary: Mt(Rp[e] + " " + Pu), binaryOfStrings: Mt(wp[e]), nonBinary: { General_Category: Mt(Pu), Script: Mt(Sp[e]) } };
  t.nonBinary.Script_Extensions = t.nonBinary.Script, t.nonBinary.gc = t.nonBinary.General_Category, t.nonBinary.sc = t.nonBinary.Script, t.nonBinary.scx = t.nonBinary.Script_Extensions;
}
for (var hl = 0, wu = [9, 10, 11, 12, 13, 14]; hl < wu.length; hl += 1) {
  var Ep = wu[hl];
  Ap(Ep);
}
var re = Be.prototype, Gr = function(t, r) {
  this.parent = t, this.base = r || this;
};
Gr.prototype.separatedFrom = function(t) {
  for (var r = this; r; r = r.parent) for (var s = t; s; s = s.parent) if (r.base === s.base && r !== s) return true;
  return false;
};
Gr.prototype.sibling = function() {
  return new Gr(this.parent, this.base);
};
var Ct = function(t) {
  this.parser = t, this.validFlags = "gim" + (t.options.ecmaVersion >= 6 ? "uy" : "") + (t.options.ecmaVersion >= 9 ? "s" : "") + (t.options.ecmaVersion >= 13 ? "d" : "") + (t.options.ecmaVersion >= 15 ? "v" : ""), this.unicodeProperties = jc[t.options.ecmaVersion >= 14 ? 14 : t.options.ecmaVersion], this.source = "", this.flags = "", this.start = 0, this.switchU = false, this.switchV = false, this.switchN = false, this.pos = 0, this.lastIntValue = 0, this.lastStringValue = "", this.lastAssertionIsQuantifiable = false, this.numCapturingParens = 0, this.maxBackReference = 0, this.groupNames = /* @__PURE__ */ Object.create(null), this.backReferenceNames = [], this.branchID = null;
};
Ct.prototype.reset = function(t, r, s) {
  var o = s.indexOf("v") !== -1, p = s.indexOf("u") !== -1;
  this.start = t | 0, this.source = r + "", this.flags = s, o && this.parser.options.ecmaVersion >= 15 ? (this.switchU = true, this.switchV = true, this.switchN = true) : (this.switchU = p && this.parser.options.ecmaVersion >= 6, this.switchV = false, this.switchN = p && this.parser.options.ecmaVersion >= 9);
};
Ct.prototype.raise = function(t) {
  this.parser.raiseRecoverable(this.start, "Invalid regular expression: /" + this.source + "/: " + t);
};
Ct.prototype.at = function(t, r) {
  r === void 0 && (r = false);
  var s = this.source, o = s.length;
  if (t >= o) return -1;
  var p = s.charCodeAt(t);
  if (!(r || this.switchU) || p <= 55295 || p >= 57344 || t + 1 >= o) return p;
  var h = s.charCodeAt(t + 1);
  return h >= 56320 && h <= 57343 ? (p << 10) + h - 56613888 : p;
};
Ct.prototype.nextIndex = function(t, r) {
  r === void 0 && (r = false);
  var s = this.source, o = s.length;
  if (t >= o) return o;
  var p = s.charCodeAt(t), h;
  return !(r || this.switchU) || p <= 55295 || p >= 57344 || t + 1 >= o || (h = s.charCodeAt(t + 1)) < 56320 || h > 57343 ? t + 1 : t + 2;
};
Ct.prototype.current = function(t) {
  return t === void 0 && (t = false), this.at(this.pos, t);
};
Ct.prototype.lookahead = function(t) {
  return t === void 0 && (t = false), this.at(this.nextIndex(this.pos, t), t);
};
Ct.prototype.advance = function(t) {
  t === void 0 && (t = false), this.pos = this.nextIndex(this.pos, t);
};
Ct.prototype.eat = function(t, r) {
  return r === void 0 && (r = false), this.current(r) === t ? (this.advance(r), true) : false;
};
Ct.prototype.eatChars = function(t, r) {
  r === void 0 && (r = false);
  for (var s = this.pos, o = 0, p = t; o < p.length; o += 1) {
    var h = p[o], v = this.at(s, r);
    if (v === -1 || v !== h) return false;
    s = this.nextIndex(s, r);
  }
  return this.pos = s, true;
};
re.validateRegExpFlags = function(e) {
  for (var t = e.validFlags, r = e.flags, s = false, o = false, p = 0; p < r.length; p++) {
    var h = r.charAt(p);
    t.indexOf(h) === -1 && this.raise(e.start, "Invalid regular expression flag"), r.indexOf(h, p + 1) > -1 && this.raise(e.start, "Duplicate regular expression flag"), h === "u" && (s = true), h === "v" && (o = true);
  }
  this.options.ecmaVersion >= 15 && s && o && this.raise(e.start, "Invalid regular expression flag");
};
function kp(e) {
  for (var t in e) return true;
  return false;
}
re.validateRegExpPattern = function(e) {
  this.regexp_pattern(e), !e.switchN && this.options.ecmaVersion >= 9 && kp(e.groupNames) && (e.switchN = true, this.regexp_pattern(e));
};
re.regexp_pattern = function(e) {
  e.pos = 0, e.lastIntValue = 0, e.lastStringValue = "", e.lastAssertionIsQuantifiable = false, e.numCapturingParens = 0, e.maxBackReference = 0, e.groupNames = /* @__PURE__ */ Object.create(null), e.backReferenceNames.length = 0, e.branchID = null, this.regexp_disjunction(e), e.pos !== e.source.length && (e.eat(41) && e.raise("Unmatched ')'"), (e.eat(93) || e.eat(125)) && e.raise("Lone quantifier brackets")), e.maxBackReference > e.numCapturingParens && e.raise("Invalid escape");
  for (var t = 0, r = e.backReferenceNames; t < r.length; t += 1) {
    var s = r[t];
    e.groupNames[s] || e.raise("Invalid named capture referenced");
  }
};
re.regexp_disjunction = function(e) {
  var t = this.options.ecmaVersion >= 16;
  for (t && (e.branchID = new Gr(e.branchID, null)), this.regexp_alternative(e); e.eat(124); ) t && (e.branchID = e.branchID.sibling()), this.regexp_alternative(e);
  t && (e.branchID = e.branchID.parent), this.regexp_eatQuantifier(e, true) && e.raise("Nothing to repeat"), e.eat(123) && e.raise("Lone quantifier brackets");
};
re.regexp_alternative = function(e) {
  for (; e.pos < e.source.length && this.regexp_eatTerm(e); ) ;
};
re.regexp_eatTerm = function(e) {
  return this.regexp_eatAssertion(e) ? (e.lastAssertionIsQuantifiable && this.regexp_eatQuantifier(e) && e.switchU && e.raise("Invalid quantifier"), true) : (e.switchU ? this.regexp_eatAtom(e) : this.regexp_eatExtendedAtom(e)) ? (this.regexp_eatQuantifier(e), true) : false;
};
re.regexp_eatAssertion = function(e) {
  var t = e.pos;
  if (e.lastAssertionIsQuantifiable = false, e.eat(94) || e.eat(36)) return true;
  if (e.eat(92)) {
    if (e.eat(66) || e.eat(98)) return true;
    e.pos = t;
  }
  if (e.eat(40) && e.eat(63)) {
    var r = false;
    if (this.options.ecmaVersion >= 9 && (r = e.eat(60)), e.eat(61) || e.eat(33)) return this.regexp_disjunction(e), e.eat(41) || e.raise("Unterminated group"), e.lastAssertionIsQuantifiable = !r, true;
  }
  return e.pos = t, false;
};
re.regexp_eatQuantifier = function(e, t) {
  return t === void 0 && (t = false), this.regexp_eatQuantifierPrefix(e, t) ? (e.eat(63), true) : false;
};
re.regexp_eatQuantifierPrefix = function(e, t) {
  return e.eat(42) || e.eat(43) || e.eat(63) || this.regexp_eatBracedQuantifier(e, t);
};
re.regexp_eatBracedQuantifier = function(e, t) {
  var r = e.pos;
  if (e.eat(123)) {
    var s = 0, o = -1;
    if (this.regexp_eatDecimalDigits(e) && (s = e.lastIntValue, e.eat(44) && this.regexp_eatDecimalDigits(e) && (o = e.lastIntValue), e.eat(125))) return o !== -1 && o < s && !t && e.raise("numbers out of order in {} quantifier"), true;
    e.switchU && !t && e.raise("Incomplete quantifier"), e.pos = r;
  }
  return false;
};
re.regexp_eatAtom = function(e) {
  return this.regexp_eatPatternCharacters(e) || e.eat(46) || this.regexp_eatReverseSolidusAtomEscape(e) || this.regexp_eatCharacterClass(e) || this.regexp_eatUncapturingGroup(e) || this.regexp_eatCapturingGroup(e);
};
re.regexp_eatReverseSolidusAtomEscape = function(e) {
  var t = e.pos;
  if (e.eat(92)) {
    if (this.regexp_eatAtomEscape(e)) return true;
    e.pos = t;
  }
  return false;
};
re.regexp_eatUncapturingGroup = function(e) {
  var t = e.pos;
  if (e.eat(40)) {
    if (e.eat(63)) {
      if (this.options.ecmaVersion >= 16) {
        var r = this.regexp_eatModifiers(e), s = e.eat(45);
        if (r || s) {
          for (var o = 0; o < r.length; o++) {
            var p = r.charAt(o);
            r.indexOf(p, o + 1) > -1 && e.raise("Duplicate regular expression modifiers");
          }
          if (s) {
            var h = this.regexp_eatModifiers(e);
            !r && !h && e.current() === 58 && e.raise("Invalid regular expression modifiers");
            for (var v = 0; v < h.length; v++) {
              var m = h.charAt(v);
              (h.indexOf(m, v + 1) > -1 || r.indexOf(m) > -1) && e.raise("Duplicate regular expression modifiers");
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
re.regexp_eatCapturingGroup = function(e) {
  if (e.eat(40)) {
    if (this.options.ecmaVersion >= 9 ? this.regexp_groupSpecifier(e) : e.current() === 63 && e.raise("Invalid group"), this.regexp_disjunction(e), e.eat(41)) return e.numCapturingParens += 1, true;
    e.raise("Unterminated group");
  }
  return false;
};
re.regexp_eatModifiers = function(e) {
  for (var t = "", r = 0; (r = e.current()) !== -1 && Ip(r); ) t += Tt(r), e.advance();
  return t;
};
function Ip(e) {
  return e === 105 || e === 109 || e === 115;
}
re.regexp_eatExtendedAtom = function(e) {
  return e.eat(46) || this.regexp_eatReverseSolidusAtomEscape(e) || this.regexp_eatCharacterClass(e) || this.regexp_eatUncapturingGroup(e) || this.regexp_eatCapturingGroup(e) || this.regexp_eatInvalidBracedQuantifier(e) || this.regexp_eatExtendedPatternCharacter(e);
};
re.regexp_eatInvalidBracedQuantifier = function(e) {
  return this.regexp_eatBracedQuantifier(e, true) && e.raise("Nothing to repeat"), false;
};
re.regexp_eatSyntaxCharacter = function(e) {
  var t = e.current();
  return $c(t) ? (e.lastIntValue = t, e.advance(), true) : false;
};
function $c(e) {
  return e === 36 || e >= 40 && e <= 43 || e === 46 || e === 63 || e >= 91 && e <= 94 || e >= 123 && e <= 125;
}
re.regexp_eatPatternCharacters = function(e) {
  for (var t = e.pos, r = 0; (r = e.current()) !== -1 && !$c(r); ) e.advance();
  return e.pos !== t;
};
re.regexp_eatExtendedPatternCharacter = function(e) {
  var t = e.current();
  return t !== -1 && t !== 36 && !(t >= 40 && t <= 43) && t !== 46 && t !== 63 && t !== 91 && t !== 94 && t !== 124 ? (e.advance(), true) : false;
};
re.regexp_groupSpecifier = function(e) {
  if (e.eat(63)) {
    this.regexp_eatGroupName(e) || e.raise("Invalid group");
    var t = this.options.ecmaVersion >= 16, r = e.groupNames[e.lastStringValue];
    if (r) if (t) for (var s = 0, o = r; s < o.length; s += 1) {
      var p = o[s];
      p.separatedFrom(e.branchID) || e.raise("Duplicate capture group name");
    }
    else e.raise("Duplicate capture group name");
    t ? (r || (e.groupNames[e.lastStringValue] = [])).push(e.branchID) : e.groupNames[e.lastStringValue] = true;
  }
};
re.regexp_eatGroupName = function(e) {
  if (e.lastStringValue = "", e.eat(60)) {
    if (this.regexp_eatRegExpIdentifierName(e) && e.eat(62)) return true;
    e.raise("Invalid capture group name");
  }
  return false;
};
re.regexp_eatRegExpIdentifierName = function(e) {
  if (e.lastStringValue = "", this.regexp_eatRegExpIdentifierStart(e)) {
    for (e.lastStringValue += Tt(e.lastIntValue); this.regexp_eatRegExpIdentifierPart(e); ) e.lastStringValue += Tt(e.lastIntValue);
    return true;
  }
  return false;
};
re.regexp_eatRegExpIdentifierStart = function(e) {
  var t = e.pos, r = this.options.ecmaVersion >= 11, s = e.current(r);
  return e.advance(r), s === 92 && this.regexp_eatRegExpUnicodeEscapeSequence(e, r) && (s = e.lastIntValue), qp(s) ? (e.lastIntValue = s, true) : (e.pos = t, false);
};
function qp(e) {
  return xt(e, true) || e === 36 || e === 95;
}
re.regexp_eatRegExpIdentifierPart = function(e) {
  var t = e.pos, r = this.options.ecmaVersion >= 11, s = e.current(r);
  return e.advance(r), s === 92 && this.regexp_eatRegExpUnicodeEscapeSequence(e, r) && (s = e.lastIntValue), Mp(s) ? (e.lastIntValue = s, true) : (e.pos = t, false);
};
function Mp(e) {
  return Vt(e, true) || e === 36 || e === 95 || e === 8204 || e === 8205;
}
re.regexp_eatAtomEscape = function(e) {
  return this.regexp_eatBackReference(e) || this.regexp_eatCharacterClassEscape(e) || this.regexp_eatCharacterEscape(e) || e.switchN && this.regexp_eatKGroupName(e) ? true : (e.switchU && (e.current() === 99 && e.raise("Invalid unicode escape"), e.raise("Invalid escape")), false);
};
re.regexp_eatBackReference = function(e) {
  var t = e.pos;
  if (this.regexp_eatDecimalEscape(e)) {
    var r = e.lastIntValue;
    if (e.switchU) return r > e.maxBackReference && (e.maxBackReference = r), true;
    if (r <= e.numCapturingParens) return true;
    e.pos = t;
  }
  return false;
};
re.regexp_eatKGroupName = function(e) {
  if (e.eat(107)) {
    if (this.regexp_eatGroupName(e)) return e.backReferenceNames.push(e.lastStringValue), true;
    e.raise("Invalid named reference");
  }
  return false;
};
re.regexp_eatCharacterEscape = function(e) {
  return this.regexp_eatControlEscape(e) || this.regexp_eatCControlLetter(e) || this.regexp_eatZero(e) || this.regexp_eatHexEscapeSequence(e) || this.regexp_eatRegExpUnicodeEscapeSequence(e, false) || !e.switchU && this.regexp_eatLegacyOctalEscapeSequence(e) || this.regexp_eatIdentityEscape(e);
};
re.regexp_eatCControlLetter = function(e) {
  var t = e.pos;
  if (e.eat(99)) {
    if (this.regexp_eatControlLetter(e)) return true;
    e.pos = t;
  }
  return false;
};
re.regexp_eatZero = function(e) {
  return e.current() === 48 && !Xr(e.lookahead()) ? (e.lastIntValue = 0, e.advance(), true) : false;
};
re.regexp_eatControlEscape = function(e) {
  var t = e.current();
  return t === 116 ? (e.lastIntValue = 9, e.advance(), true) : t === 110 ? (e.lastIntValue = 10, e.advance(), true) : t === 118 ? (e.lastIntValue = 11, e.advance(), true) : t === 102 ? (e.lastIntValue = 12, e.advance(), true) : t === 114 ? (e.lastIntValue = 13, e.advance(), true) : false;
};
re.regexp_eatControlLetter = function(e) {
  var t = e.current();
  return Fc(t) ? (e.lastIntValue = t % 32, e.advance(), true) : false;
};
function Fc(e) {
  return e >= 65 && e <= 90 || e >= 97 && e <= 122;
}
re.regexp_eatRegExpUnicodeEscapeSequence = function(e, t) {
  t === void 0 && (t = false);
  var r = e.pos, s = t || e.switchU;
  if (e.eat(117)) {
    if (this.regexp_eatFixedHexDigits(e, 4)) {
      var o = e.lastIntValue;
      if (s && o >= 55296 && o <= 56319) {
        var p = e.pos;
        if (e.eat(92) && e.eat(117) && this.regexp_eatFixedHexDigits(e, 4)) {
          var h = e.lastIntValue;
          if (h >= 56320 && h <= 57343) return e.lastIntValue = (o - 55296) * 1024 + (h - 56320) + 65536, true;
        }
        e.pos = p, e.lastIntValue = o;
      }
      return true;
    }
    if (s && e.eat(123) && this.regexp_eatHexDigits(e) && e.eat(125) && Lp(e.lastIntValue)) return true;
    s && e.raise("Invalid unicode escape"), e.pos = r;
  }
  return false;
};
function Lp(e) {
  return e >= 0 && e <= 1114111;
}
re.regexp_eatIdentityEscape = function(e) {
  if (e.switchU) return this.regexp_eatSyntaxCharacter(e) ? true : e.eat(47) ? (e.lastIntValue = 47, true) : false;
  var t = e.current();
  return t !== 99 && (!e.switchN || t !== 107) ? (e.lastIntValue = t, e.advance(), true) : false;
};
re.regexp_eatDecimalEscape = function(e) {
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
var Bc = 0, St = 1, ct = 2;
re.regexp_eatCharacterClassEscape = function(e) {
  var t = e.current();
  if (Op(t)) return e.lastIntValue = -1, e.advance(), St;
  var r = false;
  if (e.switchU && this.options.ecmaVersion >= 9 && ((r = t === 80) || t === 112)) {
    e.lastIntValue = -1, e.advance();
    var s;
    if (e.eat(123) && (s = this.regexp_eatUnicodePropertyValueExpression(e)) && e.eat(125)) return r && s === ct && e.raise("Invalid property name"), s;
    e.raise("Invalid property name");
  }
  return Bc;
};
function Op(e) {
  return e === 100 || e === 68 || e === 115 || e === 83 || e === 119 || e === 87;
}
re.regexp_eatUnicodePropertyValueExpression = function(e) {
  var t = e.pos;
  if (this.regexp_eatUnicodePropertyName(e) && e.eat(61)) {
    var r = e.lastStringValue;
    if (this.regexp_eatUnicodePropertyValue(e)) {
      var s = e.lastStringValue;
      return this.regexp_validateUnicodePropertyNameAndValue(e, r, s), St;
    }
  }
  if (e.pos = t, this.regexp_eatLoneUnicodePropertyNameOrValue(e)) {
    var o = e.lastStringValue;
    return this.regexp_validateUnicodePropertyNameOrValue(e, o);
  }
  return Bc;
};
re.regexp_validateUnicodePropertyNameAndValue = function(e, t, r) {
  ir(e.unicodeProperties.nonBinary, t) || e.raise("Invalid property name"), e.unicodeProperties.nonBinary[t].test(r) || e.raise("Invalid property value");
};
re.regexp_validateUnicodePropertyNameOrValue = function(e, t) {
  if (e.unicodeProperties.binary.test(t)) return St;
  if (e.switchV && e.unicodeProperties.binaryOfStrings.test(t)) return ct;
  e.raise("Invalid property name");
};
re.regexp_eatUnicodePropertyName = function(e) {
  var t = 0;
  for (e.lastStringValue = ""; Vc(t = e.current()); ) e.lastStringValue += Tt(t), e.advance();
  return e.lastStringValue !== "";
};
function Vc(e) {
  return Fc(e) || e === 95;
}
re.regexp_eatUnicodePropertyValue = function(e) {
  var t = 0;
  for (e.lastStringValue = ""; Np(t = e.current()); ) e.lastStringValue += Tt(t), e.advance();
  return e.lastStringValue !== "";
};
function Np(e) {
  return Vc(e) || Xr(e);
}
re.regexp_eatLoneUnicodePropertyNameOrValue = function(e) {
  return this.regexp_eatUnicodePropertyValue(e);
};
re.regexp_eatCharacterClass = function(e) {
  if (e.eat(91)) {
    var t = e.eat(94), r = this.regexp_classContents(e);
    return e.eat(93) || e.raise("Unterminated character class"), t && r === ct && e.raise("Negated character class may contain strings"), true;
  }
  return false;
};
re.regexp_classContents = function(e) {
  return e.current() === 93 ? St : e.switchV ? this.regexp_classSetExpression(e) : (this.regexp_nonEmptyClassRanges(e), St);
};
re.regexp_nonEmptyClassRanges = function(e) {
  for (; this.regexp_eatClassAtom(e); ) {
    var t = e.lastIntValue;
    if (e.eat(45) && this.regexp_eatClassAtom(e)) {
      var r = e.lastIntValue;
      e.switchU && (t === -1 || r === -1) && e.raise("Invalid character class"), t !== -1 && r !== -1 && t > r && e.raise("Range out of order in character class");
    }
  }
};
re.regexp_eatClassAtom = function(e) {
  var t = e.pos;
  if (e.eat(92)) {
    if (this.regexp_eatClassEscape(e)) return true;
    if (e.switchU) {
      var r = e.current();
      (r === 99 || Gc(r)) && e.raise("Invalid class escape"), e.raise("Invalid escape");
    }
    e.pos = t;
  }
  var s = e.current();
  return s !== 93 ? (e.lastIntValue = s, e.advance(), true) : false;
};
re.regexp_eatClassEscape = function(e) {
  var t = e.pos;
  if (e.eat(98)) return e.lastIntValue = 8, true;
  if (e.switchU && e.eat(45)) return e.lastIntValue = 45, true;
  if (!e.switchU && e.eat(99)) {
    if (this.regexp_eatClassControlLetter(e)) return true;
    e.pos = t;
  }
  return this.regexp_eatCharacterClassEscape(e) || this.regexp_eatCharacterEscape(e);
};
re.regexp_classSetExpression = function(e) {
  var t = St, r;
  if (!this.regexp_eatClassSetRange(e)) if (r = this.regexp_eatClassSetOperand(e)) {
    r === ct && (t = ct);
    for (var s = e.pos; e.eatChars([38, 38]); ) {
      if (e.current() !== 38 && (r = this.regexp_eatClassSetOperand(e))) {
        r !== ct && (t = St);
        continue;
      }
      e.raise("Invalid character in character class");
    }
    if (s !== e.pos) return t;
    for (; e.eatChars([45, 45]); ) this.regexp_eatClassSetOperand(e) || e.raise("Invalid character in character class");
    if (s !== e.pos) return t;
  } else e.raise("Invalid character in character class");
  for (; ; ) if (!this.regexp_eatClassSetRange(e)) {
    if (r = this.regexp_eatClassSetOperand(e), !r) return t;
    r === ct && (t = ct);
  }
};
re.regexp_eatClassSetRange = function(e) {
  var t = e.pos;
  if (this.regexp_eatClassSetCharacter(e)) {
    var r = e.lastIntValue;
    if (e.eat(45) && this.regexp_eatClassSetCharacter(e)) {
      var s = e.lastIntValue;
      return r !== -1 && s !== -1 && r > s && e.raise("Range out of order in character class"), true;
    }
    e.pos = t;
  }
  return false;
};
re.regexp_eatClassSetOperand = function(e) {
  return this.regexp_eatClassSetCharacter(e) ? St : this.regexp_eatClassStringDisjunction(e) || this.regexp_eatNestedClass(e);
};
re.regexp_eatNestedClass = function(e) {
  var t = e.pos;
  if (e.eat(91)) {
    var r = e.eat(94), s = this.regexp_classContents(e);
    if (e.eat(93)) return r && s === ct && e.raise("Negated character class may contain strings"), s;
    e.pos = t;
  }
  if (e.eat(92)) {
    var o = this.regexp_eatCharacterClassEscape(e);
    if (o) return o;
    e.pos = t;
  }
  return null;
};
re.regexp_eatClassStringDisjunction = function(e) {
  var t = e.pos;
  if (e.eatChars([92, 113])) {
    if (e.eat(123)) {
      var r = this.regexp_classStringDisjunctionContents(e);
      if (e.eat(125)) return r;
    } else e.raise("Invalid escape");
    e.pos = t;
  }
  return null;
};
re.regexp_classStringDisjunctionContents = function(e) {
  for (var t = this.regexp_classString(e); e.eat(124); ) this.regexp_classString(e) === ct && (t = ct);
  return t;
};
re.regexp_classString = function(e) {
  for (var t = 0; this.regexp_eatClassSetCharacter(e); ) t++;
  return t === 1 ? St : ct;
};
re.regexp_eatClassSetCharacter = function(e) {
  var t = e.pos;
  if (e.eat(92)) return this.regexp_eatCharacterEscape(e) || this.regexp_eatClassSetReservedPunctuator(e) ? true : e.eat(98) ? (e.lastIntValue = 8, true) : (e.pos = t, false);
  var r = e.current();
  return r < 0 || r === e.lookahead() && Dp(r) || jp(r) ? false : (e.advance(), e.lastIntValue = r, true);
};
function Dp(e) {
  return e === 33 || e >= 35 && e <= 38 || e >= 42 && e <= 44 || e === 46 || e >= 58 && e <= 64 || e === 94 || e === 96 || e === 126;
}
function jp(e) {
  return e === 40 || e === 41 || e === 45 || e === 47 || e >= 91 && e <= 93 || e >= 123 && e <= 125;
}
re.regexp_eatClassSetReservedPunctuator = function(e) {
  var t = e.current();
  return $p(t) ? (e.lastIntValue = t, e.advance(), true) : false;
};
function $p(e) {
  return e === 33 || e === 35 || e === 37 || e === 38 || e === 44 || e === 45 || e >= 58 && e <= 62 || e === 64 || e === 96 || e === 126;
}
re.regexp_eatClassControlLetter = function(e) {
  var t = e.current();
  return Xr(t) || t === 95 ? (e.lastIntValue = t % 32, e.advance(), true) : false;
};
re.regexp_eatHexEscapeSequence = function(e) {
  var t = e.pos;
  if (e.eat(120)) {
    if (this.regexp_eatFixedHexDigits(e, 2)) return true;
    e.switchU && e.raise("Invalid escape"), e.pos = t;
  }
  return false;
};
re.regexp_eatDecimalDigits = function(e) {
  var t = e.pos, r = 0;
  for (e.lastIntValue = 0; Xr(r = e.current()); ) e.lastIntValue = 10 * e.lastIntValue + (r - 48), e.advance();
  return e.pos !== t;
};
function Xr(e) {
  return e >= 48 && e <= 57;
}
re.regexp_eatHexDigits = function(e) {
  var t = e.pos, r = 0;
  for (e.lastIntValue = 0; Hc(r = e.current()); ) e.lastIntValue = 16 * e.lastIntValue + Uc(r), e.advance();
  return e.pos !== t;
};
function Hc(e) {
  return e >= 48 && e <= 57 || e >= 65 && e <= 70 || e >= 97 && e <= 102;
}
function Uc(e) {
  return e >= 65 && e <= 70 ? 10 + (e - 65) : e >= 97 && e <= 102 ? 10 + (e - 97) : e - 48;
}
re.regexp_eatLegacyOctalEscapeSequence = function(e) {
  if (this.regexp_eatOctalDigit(e)) {
    var t = e.lastIntValue;
    if (this.regexp_eatOctalDigit(e)) {
      var r = e.lastIntValue;
      t <= 3 && this.regexp_eatOctalDigit(e) ? e.lastIntValue = t * 64 + r * 8 + e.lastIntValue : e.lastIntValue = t * 8 + r;
    } else e.lastIntValue = t;
    return true;
  }
  return false;
};
re.regexp_eatOctalDigit = function(e) {
  var t = e.current();
  return Gc(t) ? (e.lastIntValue = t - 48, e.advance(), true) : (e.lastIntValue = 0, false);
};
function Gc(e) {
  return e >= 48 && e <= 55;
}
re.regexp_eatFixedHexDigits = function(e, t) {
  var r = e.pos;
  e.lastIntValue = 0;
  for (var s = 0; s < t; ++s) {
    var o = e.current();
    if (!Hc(o)) return e.pos = r, false;
    e.lastIntValue = 16 * e.lastIntValue + Uc(o), e.advance();
  }
  return true;
};
var Jr = function(t) {
  this.type = t.type, this.value = t.value, this.start = t.start, this.end = t.end, t.options.locations && (this.loc = new xr(t, t.startLoc, t.endLoc)), t.options.ranges && (this.range = [t.start, t.end]);
}, we = Be.prototype;
we.next = function(e) {
  !e && this.type.keyword && this.containsEsc && this.raiseRecoverable(this.start, "Escape sequence in keyword " + this.type.keyword), this.options.onToken && this.options.onToken(new Jr(this)), this.lastTokEnd = this.end, this.lastTokStart = this.start, this.lastTokEndLoc = this.endLoc, this.lastTokStartLoc = this.startLoc, this.nextToken();
};
we.getToken = function() {
  return this.next(), new Jr(this);
};
typeof Symbol < "u" && (we[Symbol.iterator] = function() {
  var e = this;
  return { next: function() {
    var t = e.getToken();
    return { done: t.type === d.eof, value: t };
  } };
});
we.nextToken = function() {
  var e = this.curContext();
  if ((!e || !e.preserveSpace) && this.skipSpace(), this.start = this.pos, this.options.locations && (this.startLoc = this.curPosition()), this.pos >= this.input.length) return this.finishToken(d.eof);
  if (e.override) return e.override(this);
  this.readToken(this.fullCharCodeAtPos());
};
we.readToken = function(e) {
  return xt(e, this.options.ecmaVersion >= 6) || e === 92 ? this.readWord() : this.getTokenFromCode(e);
};
we.fullCharCodeAtPos = function() {
  var e = this.input.charCodeAt(this.pos);
  if (e <= 55295 || e >= 56320) return e;
  var t = this.input.charCodeAt(this.pos + 1);
  return t <= 56319 || t >= 57344 ? e : (e << 10) + t - 56613888;
};
we.skipBlockComment = function() {
  var e = this.options.onComment && this.curPosition(), t = this.pos, r = this.input.indexOf("*/", this.pos += 2);
  if (r === -1 && this.raise(this.pos - 2, "Unterminated comment"), this.pos = r + 2, this.options.locations) for (var s = void 0, o = t; (s = vc(this.input, o, this.pos)) > -1; ) ++this.curLine, o = this.lineStart = s;
  this.options.onComment && this.options.onComment(true, this.input.slice(t + 2, r), t, this.pos, e, this.curPosition());
};
we.skipLineComment = function(e) {
  for (var t = this.pos, r = this.options.onComment && this.curPosition(), s = this.input.charCodeAt(this.pos += e); this.pos < this.input.length && !Ut(s); ) s = this.input.charCodeAt(++this.pos);
  this.options.onComment && this.options.onComment(false, this.input.slice(t + e, this.pos), t, this.pos, r, this.curPosition());
};
we.skipSpace = function() {
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
        if (e > 8 && e < 14 || e >= 5760 && Zl.test(String.fromCharCode(e))) ++this.pos;
        else break e;
    }
  }
};
we.finishToken = function(e, t) {
  this.end = this.pos, this.options.locations && (this.endLoc = this.curPosition());
  var r = this.type;
  this.type = e, this.value = t, this.updateContext(r);
};
we.readToken_dot = function() {
  var e = this.input.charCodeAt(this.pos + 1);
  if (e >= 48 && e <= 57) return this.readNumber(true);
  var t = this.input.charCodeAt(this.pos + 2);
  return this.options.ecmaVersion >= 6 && e === 46 && t === 46 ? (this.pos += 3, this.finishToken(d.ellipsis)) : (++this.pos, this.finishToken(d.dot));
};
we.readToken_slash = function() {
  var e = this.input.charCodeAt(this.pos + 1);
  return this.exprAllowed ? (++this.pos, this.readRegexp()) : e === 61 ? this.finishOp(d.assign, 2) : this.finishOp(d.slash, 1);
};
we.readToken_mult_modulo_exp = function(e) {
  var t = this.input.charCodeAt(this.pos + 1), r = 1, s = e === 42 ? d.star : d.modulo;
  return this.options.ecmaVersion >= 7 && e === 42 && t === 42 && (++r, s = d.starstar, t = this.input.charCodeAt(this.pos + 2)), t === 61 ? this.finishOp(d.assign, r + 1) : this.finishOp(s, r);
};
we.readToken_pipe_amp = function(e) {
  var t = this.input.charCodeAt(this.pos + 1);
  if (t === e) {
    if (this.options.ecmaVersion >= 12) {
      var r = this.input.charCodeAt(this.pos + 2);
      if (r === 61) return this.finishOp(d.assign, 3);
    }
    return this.finishOp(e === 124 ? d.logicalOR : d.logicalAND, 2);
  }
  return t === 61 ? this.finishOp(d.assign, 2) : this.finishOp(e === 124 ? d.bitwiseOR : d.bitwiseAND, 1);
};
we.readToken_caret = function() {
  var e = this.input.charCodeAt(this.pos + 1);
  return e === 61 ? this.finishOp(d.assign, 2) : this.finishOp(d.bitwiseXOR, 1);
};
we.readToken_plus_min = function(e) {
  var t = this.input.charCodeAt(this.pos + 1);
  return t === e ? t === 45 && !this.inModule && this.input.charCodeAt(this.pos + 2) === 62 && (this.lastTokEnd === 0 || at.test(this.input.slice(this.lastTokEnd, this.pos))) ? (this.skipLineComment(3), this.skipSpace(), this.nextToken()) : this.finishOp(d.incDec, 2) : t === 61 ? this.finishOp(d.assign, 2) : this.finishOp(d.plusMin, 1);
};
we.readToken_lt_gt = function(e) {
  var t = this.input.charCodeAt(this.pos + 1), r = 1;
  return t === e ? (r = e === 62 && this.input.charCodeAt(this.pos + 2) === 62 ? 3 : 2, this.input.charCodeAt(this.pos + r) === 61 ? this.finishOp(d.assign, r + 1) : this.finishOp(d.bitShift, r)) : t === 33 && e === 60 && !this.inModule && this.input.charCodeAt(this.pos + 2) === 45 && this.input.charCodeAt(this.pos + 3) === 45 ? (this.skipLineComment(4), this.skipSpace(), this.nextToken()) : (t === 61 && (r = 2), this.finishOp(d.relational, r));
};
we.readToken_eq_excl = function(e) {
  var t = this.input.charCodeAt(this.pos + 1);
  return t === 61 ? this.finishOp(d.equality, this.input.charCodeAt(this.pos + 2) === 61 ? 3 : 2) : e === 61 && t === 62 && this.options.ecmaVersion >= 6 ? (this.pos += 2, this.finishToken(d.arrow)) : this.finishOp(e === 61 ? d.eq : d.prefix, 1);
};
we.readToken_question = function() {
  var e = this.options.ecmaVersion;
  if (e >= 11) {
    var t = this.input.charCodeAt(this.pos + 1);
    if (t === 46) {
      var r = this.input.charCodeAt(this.pos + 2);
      if (r < 48 || r > 57) return this.finishOp(d.questionDot, 2);
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
we.readToken_numberSign = function() {
  var e = this.options.ecmaVersion, t = 35;
  if (e >= 13 && (++this.pos, t = this.fullCharCodeAtPos(), xt(t, true) || t === 92)) return this.finishToken(d.privateId, this.readWord1());
  this.raise(this.pos, "Unexpected character '" + Tt(t) + "'");
};
we.getTokenFromCode = function(e) {
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
  this.raise(this.pos, "Unexpected character '" + Tt(e) + "'");
};
we.finishOp = function(e, t) {
  var r = this.input.slice(this.pos, this.pos + t);
  return this.pos += t, this.finishToken(e, r);
};
we.readRegexp = function() {
  for (var e, t, r = this.pos; ; ) {
    this.pos >= this.input.length && this.raise(r, "Unterminated regular expression");
    var s = this.input.charAt(this.pos);
    if (at.test(s) && this.raise(r, "Unterminated regular expression"), e) e = false;
    else {
      if (s === "[") t = true;
      else if (s === "]" && t) t = false;
      else if (s === "/" && !t) break;
      e = s === "\\";
    }
    ++this.pos;
  }
  var o = this.input.slice(r, this.pos);
  ++this.pos;
  var p = this.pos, h = this.readWord1();
  this.containsEsc && this.unexpected(p);
  var v = this.regexpState || (this.regexpState = new Ct(this));
  v.reset(r, o, h), this.validateRegExpFlags(v), this.validateRegExpPattern(v);
  var m = null;
  try {
    m = new RegExp(o, h);
  } catch {
  }
  return this.finishToken(d.regexp, { pattern: o, flags: h, value: m });
};
we.readInt = function(e, t, r) {
  for (var s = this.options.ecmaVersion >= 12 && t === void 0, o = r && this.input.charCodeAt(this.pos) === 48, p = this.pos, h = 0, v = 0, m = 0, l = t ?? 1 / 0; m < l; ++m, ++this.pos) {
    var w = this.input.charCodeAt(this.pos), U = void 0;
    if (s && w === 95) {
      o && this.raiseRecoverable(this.pos, "Numeric separator is not allowed in legacy octal numeric literals"), v === 95 && this.raiseRecoverable(this.pos, "Numeric separator must be exactly one underscore"), m === 0 && this.raiseRecoverable(this.pos, "Numeric separator is not allowed at the first of digits"), v = w;
      continue;
    }
    if (w >= 97 ? U = w - 97 + 10 : w >= 65 ? U = w - 65 + 10 : w >= 48 && w <= 57 ? U = w - 48 : U = 1 / 0, U >= e) break;
    v = w, h = h * e + U;
  }
  return s && v === 95 && this.raiseRecoverable(this.pos - 1, "Numeric separator is not allowed at the last of digits"), this.pos === p || t != null && this.pos - p !== t ? null : h;
};
function Fp(e, t) {
  return t ? parseInt(e, 8) : parseFloat(e.replace(/_/g, ""));
}
function zc(e) {
  return typeof BigInt != "function" ? null : BigInt(e.replace(/_/g, ""));
}
we.readRadixNumber = function(e) {
  var t = this.pos;
  this.pos += 2;
  var r = this.readInt(e);
  return r == null && this.raise(this.start + 2, "Expected number in radix " + e), this.options.ecmaVersion >= 11 && this.input.charCodeAt(this.pos) === 110 ? (r = zc(this.input.slice(t, this.pos)), ++this.pos) : xt(this.fullCharCodeAtPos()) && this.raise(this.pos, "Identifier directly after number"), this.finishToken(d.num, r);
};
we.readNumber = function(e) {
  var t = this.pos;
  !e && this.readInt(10, void 0, true) === null && this.raise(t, "Invalid number");
  var r = this.pos - t >= 2 && this.input.charCodeAt(t) === 48;
  r && this.strict && this.raise(t, "Invalid number");
  var s = this.input.charCodeAt(this.pos);
  if (!r && !e && this.options.ecmaVersion >= 11 && s === 110) {
    var o = zc(this.input.slice(t, this.pos));
    return ++this.pos, xt(this.fullCharCodeAtPos()) && this.raise(this.pos, "Identifier directly after number"), this.finishToken(d.num, o);
  }
  r && /[89]/.test(this.input.slice(t, this.pos)) && (r = false), s === 46 && !r && (++this.pos, this.readInt(10), s = this.input.charCodeAt(this.pos)), (s === 69 || s === 101) && !r && (s = this.input.charCodeAt(++this.pos), (s === 43 || s === 45) && ++this.pos, this.readInt(10) === null && this.raise(t, "Invalid number")), xt(this.fullCharCodeAtPos()) && this.raise(this.pos, "Identifier directly after number");
  var p = Fp(this.input.slice(t, this.pos), r);
  return this.finishToken(d.num, p);
};
we.readCodePoint = function() {
  var e = this.input.charCodeAt(this.pos), t;
  if (e === 123) {
    this.options.ecmaVersion < 6 && this.unexpected();
    var r = ++this.pos;
    t = this.readHexChar(this.input.indexOf("}", this.pos) - this.pos), ++this.pos, t > 1114111 && this.invalidStringToken(r, "Code point out of bounds");
  } else t = this.readHexChar(4);
  return t;
};
we.readString = function(e) {
  for (var t = "", r = ++this.pos; ; ) {
    this.pos >= this.input.length && this.raise(this.start, "Unterminated string constant");
    var s = this.input.charCodeAt(this.pos);
    if (s === e) break;
    s === 92 ? (t += this.input.slice(r, this.pos), t += this.readEscapedChar(false), r = this.pos) : s === 8232 || s === 8233 ? (this.options.ecmaVersion < 10 && this.raise(this.start, "Unterminated string constant"), ++this.pos, this.options.locations && (this.curLine++, this.lineStart = this.pos)) : (Ut(s) && this.raise(this.start, "Unterminated string constant"), ++this.pos);
  }
  return t += this.input.slice(r, this.pos++), this.finishToken(d.string, t);
};
var Wc = {};
we.tryReadTemplateToken = function() {
  this.inTemplateElement = true;
  try {
    this.readTmplToken();
  } catch (e) {
    if (e === Wc) this.readInvalidTemplateToken();
    else throw e;
  }
  this.inTemplateElement = false;
};
we.invalidStringToken = function(e, t) {
  if (this.inTemplateElement && this.options.ecmaVersion >= 9) throw Wc;
  this.raise(e, t);
};
we.readTmplToken = function() {
  for (var e = "", t = this.pos; ; ) {
    this.pos >= this.input.length && this.raise(this.start, "Unterminated template");
    var r = this.input.charCodeAt(this.pos);
    if (r === 96 || r === 36 && this.input.charCodeAt(this.pos + 1) === 123) return this.pos === this.start && (this.type === d.template || this.type === d.invalidTemplate) ? r === 36 ? (this.pos += 2, this.finishToken(d.dollarBraceL)) : (++this.pos, this.finishToken(d.backQuote)) : (e += this.input.slice(t, this.pos), this.finishToken(d.template, e));
    if (r === 92) e += this.input.slice(t, this.pos), e += this.readEscapedChar(true), t = this.pos;
    else if (Ut(r)) {
      switch (e += this.input.slice(t, this.pos), ++this.pos, r) {
        case 13:
          this.input.charCodeAt(this.pos) === 10 && ++this.pos;
        case 10:
          e += `
`;
          break;
        default:
          e += String.fromCharCode(r);
          break;
      }
      this.options.locations && (++this.curLine, this.lineStart = this.pos), t = this.pos;
    } else ++this.pos;
  }
};
we.readInvalidTemplateToken = function() {
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
we.readEscapedChar = function(e) {
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
      return Tt(this.readCodePoint());
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
        var r = this.pos - 1;
        this.invalidStringToken(r, "Invalid escape sequence in template string");
      }
    default:
      if (t >= 48 && t <= 55) {
        var s = this.input.substr(this.pos - 1, 3).match(/^[0-7]+/)[0], o = parseInt(s, 8);
        return o > 255 && (s = s.slice(0, -1), o = parseInt(s, 8)), this.pos += s.length - 1, t = this.input.charCodeAt(this.pos), (s !== "0" || t === 56 || t === 57) && (this.strict || e) && this.invalidStringToken(this.pos - 1 - s.length, e ? "Octal literal in template string" : "Octal literal in strict mode"), String.fromCharCode(o);
      }
      return Ut(t) ? (this.options.locations && (this.lineStart = this.pos, ++this.curLine), "") : String.fromCharCode(t);
  }
};
we.readHexChar = function(e) {
  var t = this.pos, r = this.readInt(16, e);
  return r === null && this.invalidStringToken(t, "Bad character escape sequence"), r;
};
we.readWord1 = function() {
  this.containsEsc = false;
  for (var e = "", t = true, r = this.pos, s = this.options.ecmaVersion >= 6; this.pos < this.input.length; ) {
    var o = this.fullCharCodeAtPos();
    if (Vt(o, s)) this.pos += o <= 65535 ? 1 : 2;
    else if (o === 92) {
      this.containsEsc = true, e += this.input.slice(r, this.pos);
      var p = this.pos;
      this.input.charCodeAt(++this.pos) !== 117 && this.invalidStringToken(this.pos, "Expecting Unicode escape sequence \\uXXXX"), ++this.pos;
      var h = this.readCodePoint();
      (t ? xt : Vt)(h, s) || this.invalidStringToken(p, "Invalid Unicode escape"), e += Tt(h), r = this.pos;
    } else break;
    t = false;
  }
  return e + this.input.slice(r, this.pos);
};
we.readWord = function() {
  var e = this.readWord1(), t = d.name;
  return this.keywords.test(e) && (t = Zt[e]), this.finishToken(t, e);
};
var Kc = "8.14.0";
Be.acorn = { Parser: Be, version: Kc, defaultOptions: Br, Position: er, SourceLocation: xr, getLineInfo: eu, Node: Cr, TokenType: Pe, tokTypes: d, keywordTypes: Zt, TokContext: Ke, tokContexts: De, isIdentifierChar: Vt, isIdentifierStart: xt, Token: Jr, isNewLine: Ut, lineBreak: at, lineBreakG: mc, nonASCIIwhitespace: Zl };
function Bp(e, t) {
  return Be.parse(e, t);
}
function Vp(e, t, r) {
  return Be.parseExpressionAt(e, t, r);
}
function Hp(e, t) {
  return Be.tokenizer(e, t);
}
const Up = Object.freeze(Object.defineProperty({ __proto__: null, Node: Cr, Parser: Be, Position: er, SourceLocation: xr, TokContext: Ke, Token: Jr, TokenType: Pe, defaultOptions: Br, getLineInfo: eu, isIdentifierChar: Vt, isIdentifierStart: xt, isNewLine: Ut, keywordTypes: Zt, lineBreak: at, lineBreakG: mc, nonASCIIwhitespace: Zl, parse: Bp, parseExpressionAt: Vp, tokContexts: De, tokTypes: d, tokenizer: Hp, version: Kc }, Symbol.toStringTag, { value: "Module" }));
function Tu(e, t) {
  for (var r = 0; r < t.length; r++) {
    var s = t[r];
    s.enumerable = s.enumerable || false, s.configurable = true, "value" in s && (s.writable = true), Object.defineProperty(e, typeof (o = function(p, h) {
      if (typeof p != "object" || p === null) return p;
      var v = p[Symbol.toPrimitive];
      if (v !== void 0) {
        var m = v.call(p, "string");
        if (typeof m != "object") return m;
        throw new TypeError("@@toPrimitive must return a primitive value.");
      }
      return String(p);
    }(s.key)) == "symbol" ? o : String(o), s);
  }
  var o;
}
function zr() {
  return zr = Object.assign ? Object.assign.bind() : function(e) {
    for (var t = 1; t < arguments.length; t++) {
      var r = arguments[t];
      for (var s in r) Object.prototype.hasOwnProperty.call(r, s) && (e[s] = r[s]);
    }
    return e;
  }, zr.apply(this, arguments);
}
function Sr(e, t) {
  e.prototype = Object.create(t.prototype), e.prototype.constructor = e, Ol(e, t);
}
function Ol(e, t) {
  return Ol = Object.setPrototypeOf ? Object.setPrototypeOf.bind() : function(r, s) {
    return r.__proto__ = s, r;
  }, Ol(e, t);
}
function Su(e, t) {
  (t == null || t > e.length) && (t = e.length);
  for (var r = 0, s = new Array(t); r < t; r++) s[r] = e[r];
  return s;
}
function Au(e, t) {
  var r = typeof Symbol < "u" && e[Symbol.iterator] || e["@@iterator"];
  if (r) return (r = r.call(e)).next.bind(r);
  if (Array.isArray(e) || (r = function(o, p) {
    if (o) {
      if (typeof o == "string") return Su(o, p);
      var h = Object.prototype.toString.call(o).slice(8, -1);
      return h === "Object" && o.constructor && (h = o.constructor.name), h === "Map" || h === "Set" ? Array.from(o) : h === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(h) ? Su(o, p) : void 0;
    }
  }(e)) || t) {
    r && (e = r);
    var s = 0;
    return function() {
      return s >= e.length ? { done: true } : { done: false, value: e[s++] };
    };
  }
  throw new TypeError(`Invalid attempt to iterate non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
}
var tt = true;
function rt(e, t) {
  return t === void 0 && (t = {}), new Pe("name", t);
}
var Gp = /* @__PURE__ */ new WeakMap();
function zp(e) {
  var t = Gp.get(e.Parser.acorn || e);
  if (!t) {
    var r = { assert: rt(0, { startsExpr: tt }), asserts: rt(0, { startsExpr: tt }), global: rt(0, { startsExpr: tt }), keyof: rt(0, { startsExpr: tt }), readonly: rt(0, { startsExpr: tt }), unique: rt(0, { startsExpr: tt }), abstract: rt(0, { startsExpr: tt }), declare: rt(0, { startsExpr: tt }), enum: rt(0, { startsExpr: tt }), module: rt(0, { startsExpr: tt }), namespace: rt(0, { startsExpr: tt }), interface: rt(0, { startsExpr: tt }), type: rt(0, { startsExpr: tt }) }, s = { at: new Pe("@"), jsxName: new Pe("jsxName"), jsxText: new Pe("jsxText", { beforeExpr: true }), jsxTagStart: new Pe("jsxTagStart", { startsExpr: true }), jsxTagEnd: new Pe("jsxTagEnd") }, o = { tc_oTag: new Ke("<tag", false, false), tc_cTag: new Ke("</tag", false, false), tc_expr: new Ke("<tag>...</tag>", true, true) }, p = new RegExp("^(?:" + Object.keys(r).join("|") + ")$");
    s.jsxTagStart.updateContext = function() {
      this.context.push(o.tc_expr), this.context.push(o.tc_oTag), this.exprAllowed = false;
    }, s.jsxTagEnd.updateContext = function(h) {
      var v = this.context.pop();
      v === o.tc_oTag && h === d.slash || v === o.tc_cTag ? (this.context.pop(), this.exprAllowed = this.curContext() === o.tc_expr) : this.exprAllowed = true;
    }, t = { tokTypes: zr({}, r, s), tokContexts: zr({}, o), keywordsRegExp: p, tokenIsLiteralPropertyName: function(h) {
      return [d.name, d.string, d.num].concat(Object.values(Zt), Object.values(r)).includes(h);
    }, tokenIsKeywordOrIdentifier: function(h) {
      return [d.name].concat(Object.values(Zt), Object.values(r)).includes(h);
    }, tokenIsIdentifier: function(h) {
      return [].concat(Object.values(r), [d.name]).includes(h);
    }, tokenIsTSDeclarationStart: function(h) {
      return [r.abstract, r.declare, r.enum, r.module, r.namespace, r.interface, r.type].includes(h);
    }, tokenIsTSTypeOperator: function(h) {
      return [r.keyof, r.readonly, r.unique].includes(h);
    }, tokenIsTemplate: function(h) {
      return h === d.invalidTemplate;
    } };
  }
  return t;
}
var cr = 1024, Wp = new RegExp("(?:[^\\S\\n\\r\\u2028\\u2029]|\\/\\/.*|\\/\\*.*?\\*\\/)*", "y"), Eu = new RegExp("(?=(" + Wp.source + "))\\1" + /(?=[\n\r\u2028\u2029]|\/\*(?!.*?\*\/)|$)/.source, "y"), dr = function() {
  this.shorthandAssign = void 0, this.trailingComma = void 0, this.parenthesizedAssign = void 0, this.parenthesizedBind = void 0, this.doubleProto = void 0, this.shorthandAssign = this.trailingComma = this.parenthesizedAssign = this.parenthesizedBind = this.doubleProto = -1;
};
function Kp(e, t) {
  var r = t.key.name, s = e[r], o = "true";
  return t.type !== "MethodDefinition" || t.kind !== "get" && t.kind !== "set" || (o = (t.static ? "s" : "i") + t.kind), s === "iget" && o === "iset" || s === "iset" && o === "iget" || s === "sget" && o === "sset" || s === "sset" && o === "sget" ? (e[r] = "true", false) : !!s || (e[r] = o, false);
}
function ku(e, t) {
  var r = e.key;
  return !e.computed && (r.type === "Identifier" && r.name === t || r.type === "Literal" && r.value === t);
}
var ne = { AbstractMethodHasImplementation: function(e) {
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
}, LetInLexicalBinding: "'let' is not allowed to be used as a name in 'let' or 'const' declarations." }, Xp = { quot: '"', amp: "&", apos: "'", lt: "<", gt: ">", nbsp: "\xA0", iexcl: "\xA1", cent: "\xA2", pound: "\xA3", curren: "\xA4", yen: "\xA5", brvbar: "\xA6", sect: "\xA7", uml: "\xA8", copy: "\xA9", ordf: "\xAA", laquo: "\xAB", not: "\xAC", shy: "\xAD", reg: "\xAE", macr: "\xAF", deg: "\xB0", plusmn: "\xB1", sup2: "\xB2", sup3: "\xB3", acute: "\xB4", micro: "\xB5", para: "\xB6", middot: "\xB7", cedil: "\xB8", sup1: "\xB9", ordm: "\xBA", raquo: "\xBB", frac14: "\xBC", frac12: "\xBD", frac34: "\xBE", iquest: "\xBF", Agrave: "\xC0", Aacute: "\xC1", Acirc: "\xC2", Atilde: "\xC3", Auml: "\xC4", Aring: "\xC5", AElig: "\xC6", Ccedil: "\xC7", Egrave: "\xC8", Eacute: "\xC9", Ecirc: "\xCA", Euml: "\xCB", Igrave: "\xCC", Iacute: "\xCD", Icirc: "\xCE", Iuml: "\xCF", ETH: "\xD0", Ntilde: "\xD1", Ograve: "\xD2", Oacute: "\xD3", Ocirc: "\xD4", Otilde: "\xD5", Ouml: "\xD6", times: "\xD7", Oslash: "\xD8", Ugrave: "\xD9", Uacute: "\xDA", Ucirc: "\xDB", Uuml: "\xDC", Yacute: "\xDD", THORN: "\xDE", szlig: "\xDF", agrave: "\xE0", aacute: "\xE1", acirc: "\xE2", atilde: "\xE3", auml: "\xE4", aring: "\xE5", aelig: "\xE6", ccedil: "\xE7", egrave: "\xE8", eacute: "\xE9", ecirc: "\xEA", euml: "\xEB", igrave: "\xEC", iacute: "\xED", icirc: "\xEE", iuml: "\xEF", eth: "\xF0", ntilde: "\xF1", ograve: "\xF2", oacute: "\xF3", ocirc: "\xF4", otilde: "\xF5", ouml: "\xF6", divide: "\xF7", oslash: "\xF8", ugrave: "\xF9", uacute: "\xFA", ucirc: "\xFB", uuml: "\xFC", yacute: "\xFD", thorn: "\xFE", yuml: "\xFF", OElig: "\u0152", oelig: "\u0153", Scaron: "\u0160", scaron: "\u0161", Yuml: "\u0178", fnof: "\u0192", circ: "\u02C6", tilde: "\u02DC", Alpha: "\u0391", Beta: "\u0392", Gamma: "\u0393", Delta: "\u0394", Epsilon: "\u0395", Zeta: "\u0396", Eta: "\u0397", Theta: "\u0398", Iota: "\u0399", Kappa: "\u039A", Lambda: "\u039B", Mu: "\u039C", Nu: "\u039D", Xi: "\u039E", Omicron: "\u039F", Pi: "\u03A0", Rho: "\u03A1", Sigma: "\u03A3", Tau: "\u03A4", Upsilon: "\u03A5", Phi: "\u03A6", Chi: "\u03A7", Psi: "\u03A8", Omega: "\u03A9", alpha: "\u03B1", beta: "\u03B2", gamma: "\u03B3", delta: "\u03B4", epsilon: "\u03B5", zeta: "\u03B6", eta: "\u03B7", theta: "\u03B8", iota: "\u03B9", kappa: "\u03BA", lambda: "\u03BB", mu: "\u03BC", nu: "\u03BD", xi: "\u03BE", omicron: "\u03BF", pi: "\u03C0", rho: "\u03C1", sigmaf: "\u03C2", sigma: "\u03C3", tau: "\u03C4", upsilon: "\u03C5", phi: "\u03C6", chi: "\u03C7", psi: "\u03C8", omega: "\u03C9", thetasym: "\u03D1", upsih: "\u03D2", piv: "\u03D6", ensp: "\u2002", emsp: "\u2003", thinsp: "\u2009", zwnj: "\u200C", zwj: "\u200D", lrm: "\u200E", rlm: "\u200F", ndash: "\u2013", mdash: "\u2014", lsquo: "\u2018", rsquo: "\u2019", sbquo: "\u201A", ldquo: "\u201C", rdquo: "\u201D", bdquo: "\u201E", dagger: "\u2020", Dagger: "\u2021", bull: "\u2022", hellip: "\u2026", permil: "\u2030", prime: "\u2032", Prime: "\u2033", lsaquo: "\u2039", rsaquo: "\u203A", oline: "\u203E", frasl: "\u2044", euro: "\u20AC", image: "\u2111", weierp: "\u2118", real: "\u211C", trade: "\u2122", alefsym: "\u2135", larr: "\u2190", uarr: "\u2191", rarr: "\u2192", darr: "\u2193", harr: "\u2194", crarr: "\u21B5", lArr: "\u21D0", uArr: "\u21D1", rArr: "\u21D2", dArr: "\u21D3", hArr: "\u21D4", forall: "\u2200", part: "\u2202", exist: "\u2203", empty: "\u2205", nabla: "\u2207", isin: "\u2208", notin: "\u2209", ni: "\u220B", prod: "\u220F", sum: "\u2211", minus: "\u2212", lowast: "\u2217", radic: "\u221A", prop: "\u221D", infin: "\u221E", ang: "\u2220", and: "\u2227", or: "\u2228", cap: "\u2229", cup: "\u222A", int: "\u222B", there4: "\u2234", sim: "\u223C", cong: "\u2245", asymp: "\u2248", ne: "\u2260", equiv: "\u2261", le: "\u2264", ge: "\u2265", sub: "\u2282", sup: "\u2283", nsub: "\u2284", sube: "\u2286", supe: "\u2287", oplus: "\u2295", otimes: "\u2297", perp: "\u22A5", sdot: "\u22C5", lceil: "\u2308", rceil: "\u2309", lfloor: "\u230A", rfloor: "\u230B", lang: "\u2329", rang: "\u232A", loz: "\u25CA", spades: "\u2660", clubs: "\u2663", hearts: "\u2665", diams: "\u2666" }, Jp = /^[\da-fA-F]+$/, Qp = /^\d+$/;
function hr(e) {
  return e && (e.type === "JSXIdentifier" ? e.name : e.type === "JSXNamespacedName" ? e.namespace.name + ":" + e.name.name : e.type === "JSXMemberExpression" ? hr(e.object) + "." + hr(e.property) : void 0);
}
var fl = /(?:\s|\/\/.*|\/\*[^]*?\*\/)*/g;
function Iu(e) {
  if (!e) throw new Error("Assert fail");
}
function Yp(e) {
  return e === "accessor";
}
function Zp(e) {
  return e === "in" || e === "out";
}
function ml(e, t) {
  return 2 | (e ? 4 : 0) | (t ? 8 : 0);
}
function eh(e) {
  if (e.type !== "MemberExpression") return false;
  var t = e.property;
  return (!e.computed || !(t.type !== "TemplateLiteral" || t.expressions.length > 0)) && Xc(e.object);
}
function Xc(e) {
  return e.type === "Identifier" || e.type === "MemberExpression" && !e.computed && Xc(e.object);
}
function qu(e) {
  return e === "private" || e === "public" || e === "protected";
}
function th(e) {
  var t = e || {}, r = t.dts, s = r !== void 0 && r, o = t.allowSatisfies, p = o !== void 0 && o;
  return function(h) {
    var v = h.acorn || Up, m = zp(v), l = v.tokTypes, w = v.keywordTypes, U = v.isIdentifierStart, J = v.lineBreak, z = v.isNewLine, K = v.tokContexts, Z = v.isIdentifierChar, B = m.tokTypes, $ = m.tokContexts, L = m.keywordsRegExp, k = m.tokenIsLiteralPropertyName, O = m.tokenIsTemplate, H = m.tokenIsTSDeclarationStart, D = m.tokenIsIdentifier, ie = m.tokenIsKeywordOrIdentifier, se = m.tokenIsTSTypeOperator;
    function fe(_, S, R) {
      R === void 0 && (R = _.length);
      for (var P = S; P < R; P++) {
        var A = _.charCodeAt(P);
        if (z(A)) return P < R - 1 && A === 13 && _.charCodeAt(P + 1) === 10 ? P + 2 : P + 1;
      }
      return -1;
    }
    h = function(_, S, R) {
      var P = R.tokTypes, A = S.tokTypes;
      return function(c) {
        function a() {
          return c.apply(this, arguments) || this;
        }
        Sr(a, c);
        var i = a.prototype;
        return i.takeDecorators = function(n) {
          var u = this.decoratorStack[this.decoratorStack.length - 1];
          u.length && (n.decorators = u, this.resetStartLocationFromNode(n, u[0]), this.decoratorStack[this.decoratorStack.length - 1] = []);
        }, i.parseDecorators = function(n) {
          for (var u = this.decoratorStack[this.decoratorStack.length - 1]; this.match(A.at); ) {
            var f = this.parseDecorator();
            u.push(f);
          }
          this.match(P._export) ? n || this.unexpected() : this.canHaveLeadingDecorator() || this.raise(this.start, "Leading decorators must be attached to a class declaration.");
        }, i.parseDecorator = function() {
          var n = this.startNode();
          this.next(), this.decoratorStack.push([]);
          var u, f = this.start, b = this.startLoc;
          if (this.match(P.parenL)) {
            var y = this.start, g = this.startLoc;
            if (this.next(), u = this.parseExpression(), this.expect(P.parenR), this.options.preserveParens) {
              var C = this.startNodeAt(y, g);
              C.expression = u, u = this.finishNode(C, "ParenthesizedExpression");
            }
          } else for (u = this.parseIdent(false); this.eat(P.dot); ) {
            var q = this.startNodeAt(f, b);
            q.object = u, q.property = this.parseIdent(true), q.computed = false, u = this.finishNode(q, "MemberExpression");
          }
          return n.expression = this.parseMaybeDecoratorArguments(u), this.decoratorStack.pop(), this.finishNode(n, "Decorator");
        }, i.parseMaybeDecoratorArguments = function(n) {
          if (this.eat(P.parenL)) {
            var u = this.startNodeAtNode(n);
            return u.callee = n, u.arguments = this.parseExprList(P.parenR, false), this.finishNode(u, "CallExpression");
          }
          return n;
        }, a;
      }(_);
    }(h, m, v), h = function(_, S, R, P) {
      var A = _.tokTypes, c = S.tokTypes, a = _.isNewLine, i = _.isIdentifierChar, n = Object.assign({ allowNamespaces: true, allowNamespacedObjects: true }, P || {});
      return function(u) {
        function f() {
          return u.apply(this, arguments) || this;
        }
        Sr(f, u);
        var b = f.prototype;
        return b.jsx_readToken = function() {
          for (var y = "", g = this.pos; ; ) {
            this.pos >= this.input.length && this.raise(this.start, "Unterminated JSX contents");
            var C = this.input.charCodeAt(this.pos);
            switch (C) {
              case 60:
              case 123:
                return this.pos === this.start ? C === 60 && this.exprAllowed ? (++this.pos, this.finishToken(c.jsxTagStart)) : this.getTokenFromCode(C) : (y += this.input.slice(g, this.pos), this.finishToken(c.jsxText, y));
              case 38:
                y += this.input.slice(g, this.pos), y += this.jsx_readEntity(), g = this.pos;
                break;
              case 62:
              case 125:
                this.raise(this.pos, "Unexpected token `" + this.input[this.pos] + "`. Did you mean `" + (C === 62 ? "&gt;" : "&rbrace;") + '` or `{"' + this.input[this.pos] + '"}`?');
              default:
                a(C) ? (y += this.input.slice(g, this.pos), y += this.jsx_readNewLine(true), g = this.pos) : ++this.pos;
            }
          }
        }, b.jsx_readNewLine = function(y) {
          var g, C = this.input.charCodeAt(this.pos);
          return ++this.pos, C === 13 && this.input.charCodeAt(this.pos) === 10 ? (++this.pos, g = y ? `
` : `\r
`) : g = String.fromCharCode(C), this.options.locations && (++this.curLine, this.lineStart = this.pos), g;
        }, b.jsx_readString = function(y) {
          for (var g = "", C = ++this.pos; ; ) {
            this.pos >= this.input.length && this.raise(this.start, "Unterminated string constant");
            var q = this.input.charCodeAt(this.pos);
            if (q === y) break;
            q === 38 ? (g += this.input.slice(C, this.pos), g += this.jsx_readEntity(), C = this.pos) : a(q) ? (g += this.input.slice(C, this.pos), g += this.jsx_readNewLine(false), C = this.pos) : ++this.pos;
          }
          return g += this.input.slice(C, this.pos++), this.finishToken(A.string, g);
        }, b.jsx_readEntity = function() {
          var y, g = "", C = 0, q = this.input[this.pos];
          q !== "&" && this.raise(this.pos, "Entity must start with an ampersand");
          for (var V = ++this.pos; this.pos < this.input.length && C++ < 10; ) {
            if ((q = this.input[this.pos++]) === ";") {
              g[0] === "#" ? g[1] === "x" ? (g = g.substr(2), Jp.test(g) && (y = String.fromCharCode(parseInt(g, 16)))) : (g = g.substr(1), Qp.test(g) && (y = String.fromCharCode(parseInt(g, 10)))) : y = Xp[g];
              break;
            }
            g += q;
          }
          return y || (this.pos = V, "&");
        }, b.jsx_readWord = function() {
          var y, g = this.pos;
          do
            y = this.input.charCodeAt(++this.pos);
          while (i(y) || y === 45);
          return this.finishToken(c.jsxName, this.input.slice(g, this.pos));
        }, b.jsx_parseIdentifier = function() {
          var y = this.startNode();
          return this.type === c.jsxName ? y.name = this.value : this.type.keyword ? y.name = this.type.keyword : this.unexpected(), this.next(), this.finishNode(y, "JSXIdentifier");
        }, b.jsx_parseNamespacedName = function() {
          var y = this.start, g = this.startLoc, C = this.jsx_parseIdentifier();
          if (!n.allowNamespaces || !this.eat(A.colon)) return C;
          var q = this.startNodeAt(y, g);
          return q.namespace = C, q.name = this.jsx_parseIdentifier(), this.finishNode(q, "JSXNamespacedName");
        }, b.jsx_parseElementName = function() {
          if (this.type === c.jsxTagEnd) return "";
          var y = this.start, g = this.startLoc, C = this.jsx_parseNamespacedName();
          for (this.type !== A.dot || C.type !== "JSXNamespacedName" || n.allowNamespacedObjects || this.unexpected(); this.eat(A.dot); ) {
            var q = this.startNodeAt(y, g);
            q.object = C, q.property = this.jsx_parseIdentifier(), C = this.finishNode(q, "JSXMemberExpression");
          }
          return C;
        }, b.jsx_parseAttributeValue = function() {
          switch (this.type) {
            case A.braceL:
              var y = this.jsx_parseExpressionContainer();
              return y.expression.type === "JSXEmptyExpression" && this.raise(y.start, "JSX attributes must only be assigned a non-empty expression"), y;
            case c.jsxTagStart:
            case A.string:
              return this.parseExprAtom();
            default:
              this.raise(this.start, "JSX value should be either an expression or a quoted JSX text");
          }
        }, b.jsx_parseEmptyExpression = function() {
          var y = this.startNodeAt(this.lastTokEnd, this.lastTokEndLoc);
          return this.finishNodeAt(y, "JSXEmptyExpression", this.start, this.startLoc);
        }, b.jsx_parseExpressionContainer = function() {
          var y = this.startNode();
          return this.next(), y.expression = this.type === A.braceR ? this.jsx_parseEmptyExpression() : this.parseExpression(), this.expect(A.braceR), this.finishNode(y, "JSXExpressionContainer");
        }, b.jsx_parseAttribute = function() {
          var y = this.startNode();
          return this.eat(A.braceL) ? (this.expect(A.ellipsis), y.argument = this.parseMaybeAssign(), this.expect(A.braceR), this.finishNode(y, "JSXSpreadAttribute")) : (y.name = this.jsx_parseNamespacedName(), y.value = this.eat(A.eq) ? this.jsx_parseAttributeValue() : null, this.finishNode(y, "JSXAttribute"));
        }, b.jsx_parseOpeningElementAt = function(y, g) {
          var C = this.startNodeAt(y, g);
          C.attributes = [];
          var q = this.jsx_parseElementName();
          for (q && (C.name = q); this.type !== A.slash && this.type !== c.jsxTagEnd; ) C.attributes.push(this.jsx_parseAttribute());
          return C.selfClosing = this.eat(A.slash), this.expect(c.jsxTagEnd), this.finishNode(C, q ? "JSXOpeningElement" : "JSXOpeningFragment");
        }, b.jsx_parseClosingElementAt = function(y, g) {
          var C = this.startNodeAt(y, g), q = this.jsx_parseElementName();
          return q && (C.name = q), this.expect(c.jsxTagEnd), this.finishNode(C, q ? "JSXClosingElement" : "JSXClosingFragment");
        }, b.jsx_parseElementAt = function(y, g) {
          var C = this.startNodeAt(y, g), q = [], V = this.jsx_parseOpeningElementAt(y, g), Q = null;
          if (!V.selfClosing) {
            e: for (; ; ) switch (this.type) {
              case c.jsxTagStart:
                if (y = this.start, g = this.startLoc, this.next(), this.eat(A.slash)) {
                  Q = this.jsx_parseClosingElementAt(y, g);
                  break e;
                }
                q.push(this.jsx_parseElementAt(y, g));
                break;
              case c.jsxText:
                q.push(this.parseExprAtom());
                break;
              case A.braceL:
                q.push(this.jsx_parseExpressionContainer());
                break;
              default:
                this.unexpected();
            }
            hr(Q.name) !== hr(V.name) && this.raise(Q.start, "Expected corresponding JSX closing tag for <" + hr(V.name) + ">");
          }
          var F = V.name ? "Element" : "Fragment";
          return C["opening" + F] = V, C["closing" + F] = Q, C.children = q, this.type === A.relational && this.value === "<" && this.raise(this.start, "Adjacent JSX elements must be wrapped in an enclosing tag"), this.finishNode(C, "JSX" + F);
        }, b.jsx_parseText = function() {
          var y = this.parseLiteral(this.value);
          return y.type = "JSXText", y;
        }, b.jsx_parseElement = function() {
          var y = this.start, g = this.startLoc;
          return this.next(), this.jsx_parseElementAt(y, g);
        }, f;
      }(R);
    }(v, m, h, e == null ? void 0 : e.jsx), h = function(_, S, R) {
      var P = S.tokTypes, A = R.tokTypes;
      return function(c) {
        function a() {
          return c.apply(this, arguments) || this;
        }
        Sr(a, c);
        var i = a.prototype;
        return i.parseMaybeImportAttributes = function(n) {
          if (this.type === A._with || this.type === P.assert) {
            this.next();
            var u = this.parseImportAttributes();
            u && (n.attributes = u);
          }
        }, i.parseImportAttributes = function() {
          this.expect(A.braceL);
          var n = this.parseWithEntries();
          return this.expect(A.braceR), n;
        }, i.parseWithEntries = function() {
          var n = [], u = /* @__PURE__ */ new Set();
          do {
            if (this.type === A.braceR) break;
            var f, b = this.startNode();
            f = this.type === A.string ? this.parseLiteral(this.value) : this.parseIdent(true), this.next(), b.key = f, u.has(b.key.name) && this.raise(this.pos, "Duplicated key in attributes"), u.add(b.key.name), this.type !== A.string && this.raise(this.pos, "Only string is supported as an attribute value"), b.value = this.parseLiteral(this.value), n.push(this.finishNode(b, "ImportAttribute"));
          } while (this.eat(A.comma));
          return n;
        }, a;
      }(_);
    }(h, m, v);
    var me = function(_) {
      function S(a, i, n) {
        var u;
        return (u = _.call(this, a, i, n) || this).preValue = null, u.preToken = null, u.isLookahead = false, u.isAmbientContext = false, u.inAbstractClass = false, u.inType = false, u.inDisallowConditionalTypesContext = false, u.maybeInArrowParameters = false, u.shouldParseArrowReturnType = void 0, u.shouldParseAsyncArrowReturnType = void 0, u.decoratorStack = [[]], u.importsStack = [[]], u.importOrExportOuterKind = void 0, u.tsParseConstModifier = u.tsParseModifiers.bind(function(f) {
          if (f === void 0) throw new ReferenceError("this hasn't been initialised - super() hasn't been called");
          return f;
        }(u), { allowedModifiers: ["const"], disallowedModifiers: ["in", "out"], errorTemplate: ne.InvalidModifierOnTypeParameterPositions }), u;
      }
      Sr(S, _);
      var R, P, A, c = S.prototype;
      return c.getTokenFromCodeInType = function(a) {
        return a === 62 || a === 60 ? this.finishOp(l.relational, 1) : _.prototype.getTokenFromCode.call(this, a);
      }, c.readToken = function(a) {
        if (!this.inType) {
          var i = this.curContext();
          if (i === $.tc_expr) return this.jsx_readToken();
          if (i === $.tc_oTag || i === $.tc_cTag) {
            if (U(a)) return this.jsx_readWord();
            if (a == 62) return ++this.pos, this.finishToken(B.jsxTagEnd);
            if ((a === 34 || a === 39) && i == $.tc_oTag) return this.jsx_readString(a);
          }
          if (a === 60 && this.exprAllowed && this.input.charCodeAt(this.pos + 1) !== 33) return ++this.pos, this.finishToken(B.jsxTagStart);
        }
        return _.prototype.readToken.call(this, a);
      }, c.getTokenFromCode = function(a) {
        return this.inType ? this.getTokenFromCodeInType(a) : a === 64 ? (++this.pos, this.finishToken(B.at)) : _.prototype.getTokenFromCode.call(this, a);
      }, c.isAbstractClass = function() {
        return this.ts_isContextual(B.abstract) && this.lookahead().type === l._class;
      }, c.finishNode = function(a, i) {
        return a.type !== "" && a.end !== 0 ? a : _.prototype.finishNode.call(this, a, i);
      }, c.tryParse = function(a, i) {
        i === void 0 && (i = this.cloneCurLookaheadState());
        var n = { node: null };
        try {
          return { node: a(function(f) {
            throw f === void 0 && (f = null), n.node = f, n;
          }), error: null, thrown: false, aborted: false, failState: null };
        } catch (f) {
          var u = this.getCurLookaheadState();
          if (this.setLookaheadState(i), f instanceof SyntaxError) return { node: null, error: f, thrown: true, aborted: false, failState: u };
          if (f === n) return { node: n.node, error: null, thrown: false, aborted: true, failState: u };
          throw f;
        }
      }, c.setOptionalParametersError = function(a, i) {
        var n;
        a.optionalParametersLoc = (n = i == null ? void 0 : i.loc) != null ? n : this.startLoc;
      }, c.reScan_lt_gt = function() {
        this.type === l.relational && (this.pos -= 1, this.readToken_lt_gt(this.fullCharCodeAtPos()));
      }, c.reScan_lt = function() {
        var a = this.type;
        return a === l.bitShift ? (this.pos -= 2, this.finishOp(l.relational, 1), l.relational) : a;
      }, c.resetEndLocation = function(a, i) {
        i === void 0 && (i = this.lastTokEndLoc), a.end = i.column, a.loc.end = i, this.options.ranges && (a.range[1] = i.column);
      }, c.startNodeAtNode = function(a) {
        return _.prototype.startNodeAt.call(this, a.start, a.loc.start);
      }, c.nextTokenStart = function() {
        return this.nextTokenStartSince(this.pos);
      }, c.tsHasSomeModifiers = function(a, i) {
        return i.some(function(n) {
          return qu(n) ? a.accessibility === n : !!a[n];
        });
      }, c.tsIsStartOfStaticBlocks = function() {
        return this.isContextual("static") && this.lookaheadCharCode() === 123;
      }, c.tsCheckForInvalidTypeCasts = function(a) {
        var i = this;
        a.forEach(function(n) {
          (n == null ? void 0 : n.type) === "TSTypeCastExpression" && i.raise(n.typeAnnotation.start, ne.UnexpectedTypeAnnotation);
        });
      }, c.atPossibleAsyncArrow = function(a) {
        return a.type === "Identifier" && a.name === "async" && this.lastTokEndLoc.column === a.end && !this.canInsertSemicolon() && a.end - a.start == 5 && a.start === this.potentialArrowAt;
      }, c.tsIsIdentifier = function() {
        return D(this.type);
      }, c.tsTryParseTypeOrTypePredicateAnnotation = function() {
        return this.match(l.colon) ? this.tsParseTypeOrTypePredicateAnnotation(l.colon) : void 0;
      }, c.tsTryParseGenericAsyncArrowFunction = function(a, i, n) {
        var u = this;
        if (this.tsMatchLeftRelational()) {
          var f = this.maybeInArrowParameters;
          this.maybeInArrowParameters = true;
          var b = this.tsTryParseAndCatch(function() {
            var y = u.startNodeAt(a, i);
            return y.typeParameters = u.tsParseTypeParameters(), _.prototype.parseFunctionParams.call(u, y), y.returnType = u.tsTryParseTypeOrTypePredicateAnnotation(), u.expect(l.arrow), y;
          });
          if (this.maybeInArrowParameters = f, b) return _.prototype.parseArrowExpression.call(this, b, null, true, n);
        }
      }, c.tsParseTypeArgumentsInExpression = function() {
        if (this.reScan_lt() === l.relational) return this.tsParseTypeArguments();
      }, c.tsInNoContext = function(a) {
        var i = this.context;
        this.context = [i[0]];
        try {
          return a();
        } finally {
          this.context = i;
        }
      }, c.tsTryParseTypeAnnotation = function() {
        return this.match(l.colon) ? this.tsParseTypeAnnotation() : void 0;
      }, c.isUnparsedContextual = function(a, i) {
        var n = a + i.length;
        if (this.input.slice(a, n) === i) {
          var u = this.input.charCodeAt(n);
          return !(Z(u) || (64512 & u) == 55296);
        }
        return false;
      }, c.isAbstractConstructorSignature = function() {
        return this.ts_isContextual(B.abstract) && this.lookahead().type === l._new;
      }, c.nextTokenStartSince = function(a) {
        return fl.lastIndex = a, fl.test(this.input) ? fl.lastIndex : a;
      }, c.lookaheadCharCode = function() {
        return this.input.charCodeAt(this.nextTokenStart());
      }, c.compareLookaheadState = function(a, i) {
        for (var n = 0, u = Object.keys(a); n < u.length; n++) {
          var f = u[n];
          if (a[f] !== i[f]) return false;
        }
        return true;
      }, c.createLookaheadState = function() {
        this.value = null, this.context = [this.curContext()];
      }, c.getCurLookaheadState = function() {
        return { endLoc: this.endLoc, lastTokEnd: this.lastTokEnd, lastTokStart: this.lastTokStart, lastTokStartLoc: this.lastTokStartLoc, pos: this.pos, value: this.value, type: this.type, start: this.start, end: this.end, context: this.context, startLoc: this.startLoc, lastTokEndLoc: this.lastTokEndLoc, curLine: this.curLine, lineStart: this.lineStart, curPosition: this.curPosition, containsEsc: this.containsEsc };
      }, c.cloneCurLookaheadState = function() {
        return { pos: this.pos, value: this.value, type: this.type, start: this.start, end: this.end, context: this.context && this.context.slice(), startLoc: this.startLoc, lastTokEndLoc: this.lastTokEndLoc, endLoc: this.endLoc, lastTokEnd: this.lastTokEnd, lastTokStart: this.lastTokStart, lastTokStartLoc: this.lastTokStartLoc, curLine: this.curLine, lineStart: this.lineStart, curPosition: this.curPosition, containsEsc: this.containsEsc };
      }, c.setLookaheadState = function(a) {
        this.pos = a.pos, this.value = a.value, this.endLoc = a.endLoc, this.lastTokEnd = a.lastTokEnd, this.lastTokStart = a.lastTokStart, this.lastTokStartLoc = a.lastTokStartLoc, this.type = a.type, this.start = a.start, this.end = a.end, this.context = a.context, this.startLoc = a.startLoc, this.lastTokEndLoc = a.lastTokEndLoc, this.curLine = a.curLine, this.lineStart = a.lineStart, this.curPosition = a.curPosition, this.containsEsc = a.containsEsc;
      }, c.tsLookAhead = function(a) {
        var i = this.getCurLookaheadState(), n = a();
        return this.setLookaheadState(i), n;
      }, c.lookahead = function(a) {
        var i = this.getCurLookaheadState();
        if (this.createLookaheadState(), this.isLookahead = true, a !== void 0) for (var n = 0; n < a; n++) this.nextToken();
        else this.nextToken();
        this.isLookahead = false;
        var u = this.getCurLookaheadState();
        return this.setLookaheadState(i), u;
      }, c.readWord = function() {
        var a = this.readWord1(), i = l.name;
        return this.keywords.test(a) ? i = w[a] : new RegExp(L).test(a) && (i = B[a]), this.finishToken(i, a);
      }, c.skipBlockComment = function() {
        var a;
        this.isLookahead || (a = this.options.onComment && this.curPosition());
        var i = this.pos, n = this.input.indexOf("*/", this.pos += 2);
        if (n === -1 && this.raise(this.pos - 2, "Unterminated comment"), this.pos = n + 2, this.options.locations) for (var u, f = i; (u = fe(this.input, f, this.pos)) > -1; ) ++this.curLine, f = this.lineStart = u;
        this.isLookahead || this.options.onComment && this.options.onComment(true, this.input.slice(i + 2, n), i, this.pos, a, this.curPosition());
      }, c.skipLineComment = function(a) {
        var i, n = this.pos;
        this.isLookahead || (i = this.options.onComment && this.curPosition());
        for (var u = this.input.charCodeAt(this.pos += a); this.pos < this.input.length && !z(u); ) u = this.input.charCodeAt(++this.pos);
        this.isLookahead || this.options.onComment && this.options.onComment(false, this.input.slice(n + a, this.pos), n, this.pos, i, this.curPosition());
      }, c.finishToken = function(a, i) {
        this.preValue = this.value, this.preToken = this.type, this.end = this.pos, this.options.locations && (this.endLoc = this.curPosition());
        var n = this.type;
        this.type = a, this.value = i, this.isLookahead || this.updateContext(n);
      }, c.resetStartLocation = function(a, i, n) {
        a.start = i, a.loc.start = n, this.options.ranges && (a.range[0] = i);
      }, c.isLineTerminator = function() {
        return this.eat(l.semi) || _.prototype.canInsertSemicolon.call(this);
      }, c.hasFollowingLineBreak = function() {
        return Eu.lastIndex = this.end, Eu.test(this.input);
      }, c.addExtra = function(a, i, n, u) {
        if (u === void 0 && (u = true), a) {
          var f = a.extra = a.extra || {};
          u ? f[i] = n : Object.defineProperty(f, i, { enumerable: u, value: n });
        }
      }, c.isLiteralPropertyName = function() {
        return k(this.type);
      }, c.hasPrecedingLineBreak = function() {
        return J.test(this.input.slice(this.lastTokEndLoc.index, this.start));
      }, c.createIdentifier = function(a, i) {
        return a.name = i, this.finishNode(a, "Identifier");
      }, c.resetStartLocationFromNode = function(a, i) {
        this.resetStartLocation(a, i.start, i.loc.start);
      }, c.isThisParam = function(a) {
        return a.type === "Identifier" && a.name === "this";
      }, c.isLookaheadContextual = function(a) {
        var i = this.nextTokenStart();
        return this.isUnparsedContextual(i, a);
      }, c.ts_type_isContextual = function(a, i) {
        return a === i && !this.containsEsc;
      }, c.ts_isContextual = function(a) {
        return this.type === a && !this.containsEsc;
      }, c.ts_isContextualWithState = function(a, i) {
        return a.type === i && !a.containsEsc;
      }, c.isContextualWithState = function(a, i) {
        return i.type === l.name && i.value === a && !i.containsEsc;
      }, c.tsIsStartOfMappedType = function() {
        return this.next(), this.eat(l.plusMin) ? this.ts_isContextual(B.readonly) : (this.ts_isContextual(B.readonly) && this.next(), !!this.match(l.bracketL) && (this.next(), !!this.tsIsIdentifier() && (this.next(), this.match(l._in))));
      }, c.tsInDisallowConditionalTypesContext = function(a) {
        var i = this.inDisallowConditionalTypesContext;
        this.inDisallowConditionalTypesContext = true;
        try {
          return a();
        } finally {
          this.inDisallowConditionalTypesContext = i;
        }
      }, c.tsTryParseType = function() {
        return this.tsEatThenParseType(l.colon);
      }, c.match = function(a) {
        return this.type === a;
      }, c.matchJsx = function(a) {
        return this.type === m.tokTypes[a];
      }, c.ts_eatWithState = function(a, i, n) {
        if (a === n.type) {
          for (var u = 0; u < i; u++) this.next();
          return true;
        }
        return false;
      }, c.ts_eatContextualWithState = function(a, i, n) {
        if (L.test(a)) {
          if (this.ts_isContextualWithState(n, B[a])) {
            for (var u = 0; u < i; u++) this.next();
            return true;
          }
          return false;
        }
        if (!this.isContextualWithState(a, n)) return false;
        for (var f = 0; f < i; f++) this.next();
        return true;
      }, c.canHaveLeadingDecorator = function() {
        return this.match(l._class);
      }, c.eatContextual = function(a) {
        return L.test(a) ? !!this.ts_isContextual(B[a]) && (this.next(), true) : _.prototype.eatContextual.call(this, a);
      }, c.tsIsExternalModuleReference = function() {
        return this.isContextual("require") && this.lookaheadCharCode() === 40;
      }, c.tsParseExternalModuleReference = function() {
        var a = this.startNode();
        return this.expectContextual("require"), this.expect(l.parenL), this.match(l.string) || this.unexpected(), a.expression = this.parseExprAtom(), this.expect(l.parenR), this.finishNode(a, "TSExternalModuleReference");
      }, c.tsParseEntityName = function(a) {
        a === void 0 && (a = true);
        for (var i = this.parseIdent(a); this.eat(l.dot); ) {
          var n = this.startNodeAtNode(i);
          n.left = i, n.right = this.parseIdent(a), i = this.finishNode(n, "TSQualifiedName");
        }
        return i;
      }, c.tsParseEnumMember = function() {
        var a = this.startNode();
        return a.id = this.match(l.string) ? this.parseLiteral(this.value) : this.parseIdent(true), this.eat(l.eq) && (a.initializer = this.parseMaybeAssign()), this.finishNode(a, "TSEnumMember");
      }, c.tsParseEnumDeclaration = function(a, i) {
        return i === void 0 && (i = {}), i.const && (a.const = true), i.declare && (a.declare = true), this.expectContextual("enum"), a.id = this.parseIdent(), this.checkLValSimple(a.id), this.expect(l.braceL), a.members = this.tsParseDelimitedList("EnumMembers", this.tsParseEnumMember.bind(this)), this.expect(l.braceR), this.finishNode(a, "TSEnumDeclaration");
      }, c.tsParseModuleBlock = function() {
        var a = this.startNode();
        for (_.prototype.enterScope.call(this, 512), this.expect(l.braceL), a.body = []; this.type !== l.braceR; ) {
          var i = this.parseStatement(null, true);
          a.body.push(i);
        }
        return this.next(), _.prototype.exitScope.call(this), this.finishNode(a, "TSModuleBlock");
      }, c.tsParseAmbientExternalModuleDeclaration = function(a) {
        return this.ts_isContextual(B.global) ? (a.global = true, a.id = this.parseIdent()) : this.match(l.string) ? a.id = this.parseLiteral(this.value) : this.unexpected(), this.match(l.braceL) ? (_.prototype.enterScope.call(this, cr), a.body = this.tsParseModuleBlock(), _.prototype.exitScope.call(this)) : _.prototype.semicolon.call(this), this.finishNode(a, "TSModuleDeclaration");
      }, c.tsTryParseDeclare = function(a) {
        var i = this;
        if (!this.isLineTerminator()) {
          var n, u = this.type;
          return this.isContextual("let") && (u = l._var, n = "let"), this.tsInAmbientContext(function() {
            if (u === l._function) return a.declare = true, i.parseFunctionStatement(a, false, true);
            if (u === l._class) return a.declare = true, i.parseClass(a, true);
            if (u === B.enum) return i.tsParseEnumDeclaration(a, { declare: true });
            if (u === B.global) return i.tsParseAmbientExternalModuleDeclaration(a);
            if (u === l._const || u === l._var) return i.match(l._const) && i.isLookaheadContextual("enum") ? (i.expect(l._const), i.tsParseEnumDeclaration(a, { const: true, declare: true })) : (a.declare = true, i.parseVarStatement(a, n || i.value, true));
            if (u === B.interface) {
              var f = i.tsParseInterfaceDeclaration(a, { declare: true });
              if (f) return f;
            }
            return D(u) ? i.tsParseDeclaration(a, i.value, true) : void 0;
          });
        }
      }, c.tsIsListTerminator = function(a) {
        switch (a) {
          case "EnumMembers":
          case "TypeMembers":
            return this.match(l.braceR);
          case "HeritageClauseElement":
            return this.match(l.braceL);
          case "TupleElementTypes":
            return this.match(l.bracketR);
          case "TypeParametersOrArguments":
            return this.tsMatchRightRelational();
        }
      }, c.tsParseDelimitedListWorker = function(a, i, n, u) {
        for (var f = [], b = -1; !this.tsIsListTerminator(a); ) {
          b = -1;
          var y = i();
          if (y == null) return;
          if (f.push(y), !this.eat(l.comma)) {
            if (this.tsIsListTerminator(a)) break;
            return void (n && this.expect(l.comma));
          }
          b = this.lastTokStart;
        }
        return u && (u.value = b), f;
      }, c.tsParseDelimitedList = function(a, i, n) {
        return function(u) {
          if (u == null) throw new Error("Unexpected " + u + " value.");
          return u;
        }(this.tsParseDelimitedListWorker(a, i, true, n));
      }, c.tsParseBracketedList = function(a, i, n, u, f) {
        u || this.expect(n ? l.bracketL : l.relational);
        var b = this.tsParseDelimitedList(a, i, f);
        return this.expect(n ? l.bracketR : l.relational), b;
      }, c.tsParseTypeParameterName = function() {
        return this.parseIdent().name;
      }, c.tsEatThenParseType = function(a) {
        return this.match(a) ? this.tsNextThenParseType() : void 0;
      }, c.tsExpectThenParseType = function(a) {
        var i = this;
        return this.tsDoThenParseType(function() {
          return i.expect(a);
        });
      }, c.tsNextThenParseType = function() {
        var a = this;
        return this.tsDoThenParseType(function() {
          return a.next();
        });
      }, c.tsDoThenParseType = function(a) {
        var i = this;
        return this.tsInType(function() {
          return a(), i.tsParseType();
        });
      }, c.tsSkipParameterStart = function() {
        if (D(this.type) || this.match(l._this)) return this.next(), true;
        if (this.match(l.braceL)) try {
          return this.parseObj(true), true;
        } catch {
          return false;
        }
        if (this.match(l.bracketL)) {
          this.next();
          try {
            return this.parseBindingList(l.bracketR, true, true), true;
          } catch {
            return false;
          }
        }
        return false;
      }, c.tsIsUnambiguouslyStartOfFunctionType = function() {
        return this.next(), !!(this.match(l.parenR) || this.match(l.ellipsis) || this.tsSkipParameterStart() && (this.match(l.colon) || this.match(l.comma) || this.match(l.question) || this.match(l.eq) || this.match(l.parenR) && (this.next(), this.match(l.arrow))));
      }, c.tsIsStartOfFunctionType = function() {
        return !!this.tsMatchLeftRelational() || this.match(l.parenL) && this.tsLookAhead(this.tsIsUnambiguouslyStartOfFunctionType.bind(this));
      }, c.tsInAllowConditionalTypesContext = function(a) {
        var i = this.inDisallowConditionalTypesContext;
        this.inDisallowConditionalTypesContext = false;
        try {
          return a();
        } finally {
          this.inDisallowConditionalTypesContext = i;
        }
      }, c.tsParseBindingListForSignature = function() {
        var a = this;
        return _.prototype.parseBindingList.call(this, l.parenR, true, true).map(function(i) {
          return i.type !== "Identifier" && i.type !== "RestElement" && i.type !== "ObjectPattern" && i.type !== "ArrayPattern" && a.raise(i.start, ne.UnsupportedSignatureParameterKind(i.type)), i;
        });
      }, c.tsParseTypePredicateAsserts = function() {
        if (this.type !== B.asserts) return false;
        var a = this.containsEsc;
        return this.next(), !(!D(this.type) && !this.match(l._this) || (a && this.raise(this.lastTokStart, "Escape sequence in keyword asserts"), 0));
      }, c.tsParseThisTypeNode = function() {
        var a = this.startNode();
        return this.next(), this.finishNode(a, "TSThisType");
      }, c.tsParseTypeAnnotation = function(a, i) {
        var n = this;
        return a === void 0 && (a = true), i === void 0 && (i = this.startNode()), this.tsInType(function() {
          a && n.expect(l.colon), i.typeAnnotation = n.tsParseType();
        }), this.finishNode(i, "TSTypeAnnotation");
      }, c.tsParseThisTypePredicate = function(a) {
        this.next();
        var i = this.startNodeAtNode(a);
        return i.parameterName = a, i.typeAnnotation = this.tsParseTypeAnnotation(false), i.asserts = false, this.finishNode(i, "TSTypePredicate");
      }, c.tsParseThisTypeOrThisTypePredicate = function() {
        var a = this.tsParseThisTypeNode();
        return this.isContextual("is") && !this.hasPrecedingLineBreak() ? this.tsParseThisTypePredicate(a) : a;
      }, c.tsParseTypePredicatePrefix = function() {
        var a = this.parseIdent();
        if (this.isContextual("is") && !this.hasPrecedingLineBreak()) return this.next(), a;
      }, c.tsParseTypeOrTypePredicateAnnotation = function(a) {
        var i = this;
        return this.tsInType(function() {
          var n = i.startNode();
          i.expect(a);
          var u = i.startNode(), f = !!i.tsTryParse(i.tsParseTypePredicateAsserts.bind(i));
          if (f && i.match(l._this)) {
            var b = i.tsParseThisTypeOrThisTypePredicate();
            return b.type === "TSThisType" ? (u.parameterName = b, u.asserts = true, u.typeAnnotation = null, b = i.finishNode(u, "TSTypePredicate")) : (i.resetStartLocationFromNode(b, u), b.asserts = true), n.typeAnnotation = b, i.finishNode(n, "TSTypeAnnotation");
          }
          var y = i.tsIsIdentifier() && i.tsTryParse(i.tsParseTypePredicatePrefix.bind(i));
          if (!y) return f ? (u.parameterName = i.parseIdent(), u.asserts = f, u.typeAnnotation = null, n.typeAnnotation = i.finishNode(u, "TSTypePredicate"), i.finishNode(n, "TSTypeAnnotation")) : i.tsParseTypeAnnotation(false, n);
          var g = i.tsParseTypeAnnotation(false);
          return u.parameterName = y, u.typeAnnotation = g, u.asserts = f, n.typeAnnotation = i.finishNode(u, "TSTypePredicate"), i.finishNode(n, "TSTypeAnnotation");
        });
      }, c.tsFillSignature = function(a, i) {
        var n = a === l.arrow;
        i.typeParameters = this.tsTryParseTypeParameters(), this.expect(l.parenL), i.parameters = this.tsParseBindingListForSignature(), (n || this.match(a)) && (i.typeAnnotation = this.tsParseTypeOrTypePredicateAnnotation(a));
      }, c.tsTryNextParseConstantContext = function() {
        if (this.lookahead().type !== l._const) return null;
        this.next();
        var a = this.tsParseTypeReference();
        return a.typeParameters && this.raise(a.typeName.start, ne.CannotFindName({ name: "const" })), a;
      }, c.tsParseFunctionOrConstructorType = function(a, i) {
        var n = this, u = this.startNode();
        return a === "TSConstructorType" && (u.abstract = !!i, i && this.next(), this.next()), this.tsInAllowConditionalTypesContext(function() {
          return n.tsFillSignature(l.arrow, u);
        }), this.finishNode(u, a);
      }, c.tsParseUnionOrIntersectionType = function(a, i, n) {
        var u = this.startNode(), f = this.eat(n), b = [];
        do
          b.push(i());
        while (this.eat(n));
        return b.length !== 1 || f ? (u.types = b, this.finishNode(u, a)) : b[0];
      }, c.tsCheckTypeAnnotationForReadOnly = function(a) {
        switch (a.typeAnnotation.type) {
          case "TSTupleType":
          case "TSArrayType":
            return;
          default:
            this.raise(a.start, ne.UnexpectedReadonly);
        }
      }, c.tsParseTypeOperator = function() {
        var a = this.startNode(), i = this.value;
        return this.next(), a.operator = i, a.typeAnnotation = this.tsParseTypeOperatorOrHigher(), i === "readonly" && this.tsCheckTypeAnnotationForReadOnly(a), this.finishNode(a, "TSTypeOperator");
      }, c.tsParseConstraintForInferType = function() {
        var a = this;
        if (this.eat(l._extends)) {
          var i = this.tsInDisallowConditionalTypesContext(function() {
            return a.tsParseType();
          });
          if (this.inDisallowConditionalTypesContext || !this.match(l.question)) return i;
        }
      }, c.tsParseInferType = function() {
        var a = this, i = this.startNode();
        this.expectContextual("infer");
        var n = this.startNode();
        return n.name = this.tsParseTypeParameterName(), n.constraint = this.tsTryParse(function() {
          return a.tsParseConstraintForInferType();
        }), i.typeParameter = this.finishNode(n, "TSTypeParameter"), this.finishNode(i, "TSInferType");
      }, c.tsParseLiteralTypeNode = function() {
        var a = this, i = this.startNode();
        return i.literal = function() {
          switch (a.type) {
            case l.num:
            case l.string:
            case l._true:
            case l._false:
              return a.parseExprAtom();
            default:
              a.unexpected();
          }
        }(), this.finishNode(i, "TSLiteralType");
      }, c.tsParseImportType = function() {
        var a = this.startNode();
        return this.expect(l._import), this.expect(l.parenL), this.match(l.string) || this.raise(this.start, ne.UnsupportedImportTypeArgument), a.argument = this.parseExprAtom(), this.expect(l.parenR), this.eat(l.dot) && (a.qualifier = this.tsParseEntityName()), this.tsMatchLeftRelational() && (a.typeParameters = this.tsParseTypeArguments()), this.finishNode(a, "TSImportType");
      }, c.tsParseTypeQuery = function() {
        var a = this.startNode();
        return this.expect(l._typeof), a.exprName = this.match(l._import) ? this.tsParseImportType() : this.tsParseEntityName(), !this.hasPrecedingLineBreak() && this.tsMatchLeftRelational() && (a.typeParameters = this.tsParseTypeArguments()), this.finishNode(a, "TSTypeQuery");
      }, c.tsParseMappedTypeParameter = function() {
        var a = this.startNode();
        return a.name = this.tsParseTypeParameterName(), a.constraint = this.tsExpectThenParseType(l._in), this.finishNode(a, "TSTypeParameter");
      }, c.tsParseMappedType = function() {
        var a = this.startNode();
        return this.expect(l.braceL), this.match(l.plusMin) ? (a.readonly = this.value, this.next(), this.expectContextual("readonly")) : this.eatContextual("readonly") && (a.readonly = true), this.expect(l.bracketL), a.typeParameter = this.tsParseMappedTypeParameter(), a.nameType = this.eatContextual("as") ? this.tsParseType() : null, this.expect(l.bracketR), this.match(l.plusMin) ? (a.optional = this.value, this.next(), this.expect(l.question)) : this.eat(l.question) && (a.optional = true), a.typeAnnotation = this.tsTryParseType(), this.semicolon(), this.expect(l.braceR), this.finishNode(a, "TSMappedType");
      }, c.tsParseTypeLiteral = function() {
        var a = this.startNode();
        return a.members = this.tsParseObjectTypeMembers(), this.finishNode(a, "TSTypeLiteral");
      }, c.tsParseTupleElementType = function() {
        var a = this.startLoc, i = this.start, n = this.eat(l.ellipsis), u = this.tsParseType(), f = this.eat(l.question);
        if (this.eat(l.colon)) {
          var b = this.startNodeAtNode(u);
          b.optional = f, u.type !== "TSTypeReference" || u.typeParameters || u.typeName.type !== "Identifier" ? (this.raise(u.start, ne.InvalidTupleMemberLabel), b.label = u) : b.label = u.typeName, b.elementType = this.tsParseType(), u = this.finishNode(b, "TSNamedTupleMember");
        } else if (f) {
          var y = this.startNodeAtNode(u);
          y.typeAnnotation = u, u = this.finishNode(y, "TSOptionalType");
        }
        if (n) {
          var g = this.startNodeAt(i, a);
          g.typeAnnotation = u, u = this.finishNode(g, "TSRestType");
        }
        return u;
      }, c.tsParseTupleType = function() {
        var a = this, i = this.startNode();
        i.elementTypes = this.tsParseBracketedList("TupleElementTypes", this.tsParseTupleElementType.bind(this), true, false);
        var n = false, u = null;
        return i.elementTypes.forEach(function(f) {
          var b = f.type;
          !n || b === "TSRestType" || b === "TSOptionalType" || b === "TSNamedTupleMember" && f.optional || a.raise(f.start, ne.OptionalTypeBeforeRequired), n || (n = b === "TSNamedTupleMember" && f.optional || b === "TSOptionalType");
          var y = b;
          b === "TSRestType" && (y = (f = f.typeAnnotation).type);
          var g = y === "TSNamedTupleMember";
          u != null || (u = g), u !== g && a.raise(f.start, ne.MixedLabeledAndUnlabeledElements);
        }), this.finishNode(i, "TSTupleType");
      }, c.tsParseTemplateLiteralType = function() {
        var a = this.startNode();
        return a.literal = this.parseTemplate({ isTagged: false }), this.finishNode(a, "TSLiteralType");
      }, c.tsParseTypeReference = function() {
        var a = this.startNode();
        return a.typeName = this.tsParseEntityName(), !this.hasPrecedingLineBreak() && this.tsMatchLeftRelational() && (a.typeParameters = this.tsParseTypeArguments()), this.finishNode(a, "TSTypeReference");
      }, c.tsMatchLeftRelational = function() {
        return this.match(l.relational) && this.value === "<";
      }, c.tsMatchRightRelational = function() {
        return this.match(l.relational) && this.value === ">";
      }, c.tsParseParenthesizedType = function() {
        var a = this.startNode();
        return this.expect(l.parenL), a.typeAnnotation = this.tsParseType(), this.expect(l.parenR), this.finishNode(a, "TSParenthesizedType");
      }, c.tsParseNonArrayType = function() {
        switch (this.type) {
          case l.string:
          case l.num:
          case l._true:
          case l._false:
            return this.tsParseLiteralTypeNode();
          case l.plusMin:
            if (this.value === "-") {
              var a = this.startNode();
              return this.lookahead().type !== l.num && this.unexpected(), a.literal = this.parseMaybeUnary(), this.finishNode(a, "TSLiteralType");
            }
            break;
          case l._this:
            return this.tsParseThisTypeOrThisTypePredicate();
          case l._typeof:
            return this.tsParseTypeQuery();
          case l._import:
            return this.tsParseImportType();
          case l.braceL:
            return this.tsLookAhead(this.tsIsStartOfMappedType.bind(this)) ? this.tsParseMappedType() : this.tsParseTypeLiteral();
          case l.bracketL:
            return this.tsParseTupleType();
          case l.parenL:
            return this.tsParseParenthesizedType();
          case l.backQuote:
          case l.dollarBraceL:
            return this.tsParseTemplateLiteralType();
          default:
            var i = this.type;
            if (D(i) || i === l._void || i === l._null) {
              var n = i === l._void ? "TSVoidKeyword" : i === l._null ? "TSNullKeyword" : function(f) {
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
                var u = this.startNode();
                return this.next(), this.finishNode(u, n);
              }
              return this.tsParseTypeReference();
            }
        }
        this.unexpected();
      }, c.tsParseArrayTypeOrHigher = function() {
        for (var a = this.tsParseNonArrayType(); !this.hasPrecedingLineBreak() && this.eat(l.bracketL); ) if (this.match(l.bracketR)) {
          var i = this.startNodeAtNode(a);
          i.elementType = a, this.expect(l.bracketR), a = this.finishNode(i, "TSArrayType");
        } else {
          var n = this.startNodeAtNode(a);
          n.objectType = a, n.indexType = this.tsParseType(), this.expect(l.bracketR), a = this.finishNode(n, "TSIndexedAccessType");
        }
        return a;
      }, c.tsParseTypeOperatorOrHigher = function() {
        var a = this;
        return se(this.type) && !this.containsEsc ? this.tsParseTypeOperator() : this.isContextual("infer") ? this.tsParseInferType() : this.tsInAllowConditionalTypesContext(function() {
          return a.tsParseArrayTypeOrHigher();
        });
      }, c.tsParseIntersectionTypeOrHigher = function() {
        return this.tsParseUnionOrIntersectionType("TSIntersectionType", this.tsParseTypeOperatorOrHigher.bind(this), l.bitwiseAND);
      }, c.tsParseUnionTypeOrHigher = function() {
        return this.tsParseUnionOrIntersectionType("TSUnionType", this.tsParseIntersectionTypeOrHigher.bind(this), l.bitwiseOR);
      }, c.tsParseNonConditionalType = function() {
        return this.tsIsStartOfFunctionType() ? this.tsParseFunctionOrConstructorType("TSFunctionType") : this.match(l._new) ? this.tsParseFunctionOrConstructorType("TSConstructorType") : this.isAbstractConstructorSignature() ? this.tsParseFunctionOrConstructorType("TSConstructorType", true) : this.tsParseUnionTypeOrHigher();
      }, c.tsParseType = function() {
        var a = this;
        Iu(this.inType);
        var i = this.tsParseNonConditionalType();
        if (this.inDisallowConditionalTypesContext || this.hasPrecedingLineBreak() || !this.eat(l._extends)) return i;
        var n = this.startNodeAtNode(i);
        return n.checkType = i, n.extendsType = this.tsInDisallowConditionalTypesContext(function() {
          return a.tsParseNonConditionalType();
        }), this.expect(l.question), n.trueType = this.tsInAllowConditionalTypesContext(function() {
          return a.tsParseType();
        }), this.expect(l.colon), n.falseType = this.tsInAllowConditionalTypesContext(function() {
          return a.tsParseType();
        }), this.finishNode(n, "TSConditionalType");
      }, c.tsIsUnambiguouslyIndexSignature = function() {
        return this.next(), !!D(this.type) && (this.next(), this.match(l.colon));
      }, c.tsInType = function(a) {
        var i = this.inType;
        this.inType = true;
        try {
          return a();
        } finally {
          this.inType = i;
        }
      }, c.tsTryParseIndexSignature = function(a) {
        if (this.match(l.bracketL) && this.tsLookAhead(this.tsIsUnambiguouslyIndexSignature.bind(this))) {
          this.expect(l.bracketL);
          var i = this.parseIdent();
          i.typeAnnotation = this.tsParseTypeAnnotation(), this.resetEndLocation(i), this.expect(l.bracketR), a.parameters = [i];
          var n = this.tsTryParseTypeAnnotation();
          return n && (a.typeAnnotation = n), this.tsParseTypeMemberSemicolon(), this.finishNode(a, "TSIndexSignature");
        }
      }, c.tsParseNoneModifiers = function(a) {
        this.tsParseModifiers({ modified: a, allowedModifiers: [], disallowedModifiers: ["in", "out"], errorTemplate: ne.InvalidModifierOnTypeParameterPositions });
      }, c.tsParseTypeParameter = function(a) {
        a === void 0 && (a = this.tsParseNoneModifiers.bind(this));
        var i = this.startNode();
        return a(i), i.name = this.tsParseTypeParameterName(), i.constraint = this.tsEatThenParseType(l._extends), i.default = this.tsEatThenParseType(l.eq), this.finishNode(i, "TSTypeParameter");
      }, c.tsParseTypeParameters = function(a) {
        var i = this.startNode();
        this.tsMatchLeftRelational() || this.matchJsx("jsxTagStart") ? this.next() : this.unexpected();
        var n = { value: -1 };
        return i.params = this.tsParseBracketedList("TypeParametersOrArguments", this.tsParseTypeParameter.bind(this, a), false, true, n), i.params.length === 0 && this.raise(this.start, ne.EmptyTypeParameters), n.value !== -1 && this.addExtra(i, "trailingComma", n.value), this.finishNode(i, "TSTypeParameterDeclaration");
      }, c.tsTryParseTypeParameters = function(a) {
        if (this.tsMatchLeftRelational()) return this.tsParseTypeParameters(a);
      }, c.tsTryParse = function(a) {
        var i = this.getCurLookaheadState(), n = a();
        return n !== void 0 && n !== false ? n : void this.setLookaheadState(i);
      }, c.tsTokenCanFollowModifier = function() {
        return (this.match(l.bracketL) || this.match(l.braceL) || this.match(l.star) || this.match(l.ellipsis) || this.match(l.privateId) || this.isLiteralPropertyName()) && !this.hasPrecedingLineBreak();
      }, c.tsNextTokenCanFollowModifier = function() {
        return this.next(true), this.tsTokenCanFollowModifier();
      }, c.tsParseModifier = function(a, i) {
        if (D(this.type) || this.type === l._in) {
          var n = this.value;
          if (a.indexOf(n) !== -1 && !this.containsEsc) {
            if (i && this.tsIsStartOfStaticBlocks()) return;
            if (this.tsTryParse(this.tsNextTokenCanFollowModifier.bind(this))) return n;
          }
        }
      }, c.tsParseModifiersByMap = function(a) {
        for (var i = a.modified, n = a.map, u = 0, f = Object.keys(n); u < f.length; u++) {
          var b = f[u];
          i[b] = n[b];
        }
      }, c.tsParseModifiers = function(a) {
        for (var i = this, n = a.modified, u = a.allowedModifiers, f = a.disallowedModifiers, b = a.stopOnStartOfClassStaticBlock, y = a.errorTemplate, g = y === void 0 ? ne.InvalidModifierOnTypeMember : y, C = {}, q = function(ce, ue, de, be) {
          ue === de && n[be] && i.raise(ce.column, ne.InvalidModifiersOrder({ orderedModifiers: [de, be] }));
        }, V = function(ce, ue, de, be) {
          (n[de] && ue === be || n[be] && ue === de) && i.raise(ce.column, ne.IncompatibleModifiers({ modifiers: [de, be] }));
        }; ; ) {
          var Q = this.startLoc, F = this.tsParseModifier(u.concat(f ?? []), b);
          if (!F) break;
          qu(F) ? n.accessibility ? this.raise(this.start, ne.DuplicateAccessibilityModifier()) : (q(Q, F, F, "override"), q(Q, F, F, "static"), q(Q, F, F, "readonly"), q(Q, F, F, "accessor"), C.accessibility = F, n.accessibility = F) : Zp(F) ? n[F] ? this.raise(this.start, ne.DuplicateModifier({ modifier: F })) : (q(Q, F, "in", "out"), C[F] = F, n[F] = true) : Yp(F) ? n[F] ? this.raise(this.start, ne.DuplicateModifier({ modifier: F })) : (V(Q, F, "accessor", "readonly"), V(Q, F, "accessor", "static"), V(Q, F, "accessor", "override"), C[F] = F, n[F] = true) : Object.hasOwnProperty.call(n, F) ? this.raise(this.start, ne.DuplicateModifier({ modifier: F })) : (q(Q, F, "static", "readonly"), q(Q, F, "static", "override"), q(Q, F, "override", "readonly"), q(Q, F, "abstract", "override"), V(Q, F, "declare", "override"), V(Q, F, "static", "abstract"), C[F] = F, n[F] = true), f != null && f.includes(F) && this.raise(this.start, g);
        }
        return C;
      }, c.tsParseInOutModifiers = function(a) {
        this.tsParseModifiers({ modified: a, allowedModifiers: ["in", "out"], disallowedModifiers: ["public", "private", "protected", "readonly", "declare", "abstract", "override"], errorTemplate: ne.InvalidModifierOnTypeParameter });
      }, c.tsParseTypeArguments = function() {
        var a = this, i = this.startNode();
        return i.params = this.tsInType(function() {
          return a.tsInNoContext(function() {
            return a.expect(l.relational), a.tsParseDelimitedList("TypeParametersOrArguments", a.tsParseType.bind(a));
          });
        }), i.params.length === 0 && this.raise(this.start, ne.EmptyTypeArguments), this.exprAllowed = false, this.expect(l.relational), this.finishNode(i, "TSTypeParameterInstantiation");
      }, c.tsParseHeritageClause = function(a) {
        var i = this, n = this.start, u = this.tsParseDelimitedList("HeritageClauseElement", function() {
          var f = i.startNode();
          return f.expression = i.tsParseEntityName(), i.tsMatchLeftRelational() && (f.typeParameters = i.tsParseTypeArguments()), i.finishNode(f, "TSExpressionWithTypeArguments");
        });
        return u.length || this.raise(n, ne.EmptyHeritageClauseType({ token: a })), u;
      }, c.tsParseTypeMemberSemicolon = function() {
        this.eat(l.comma) || this.isLineTerminator() || this.expect(l.semi);
      }, c.tsTryParseAndCatch = function(a) {
        var i = this.tryParse(function(n) {
          return a() || n();
        });
        if (!i.aborted && i.node) return i.error && this.setLookaheadState(i.failState), i.node;
      }, c.tsParseSignatureMember = function(a, i) {
        return this.tsFillSignature(l.colon, i), this.tsParseTypeMemberSemicolon(), this.finishNode(i, a);
      }, c.tsParsePropertyOrMethodSignature = function(a, i) {
        this.eat(l.question) && (a.optional = true);
        var n = a;
        if (this.match(l.parenL) || this.tsMatchLeftRelational()) {
          i && this.raise(a.start, ne.ReadonlyForMethodSignature);
          var u = n;
          u.kind && this.tsMatchLeftRelational() && this.raise(this.start, ne.AccesorCannotHaveTypeParameters), this.tsFillSignature(l.colon, u), this.tsParseTypeMemberSemicolon();
          var f = "parameters", b = "typeAnnotation";
          if (u.kind === "get") u[f].length > 0 && (this.raise(this.start, "A 'get' accesor must not have any formal parameters."), this.isThisParam(u[f][0]) && this.raise(this.start, ne.AccesorCannotDeclareThisParameter));
          else if (u.kind === "set") {
            if (u[f].length !== 1) this.raise(this.start, "A 'get' accesor must not have any formal parameters.");
            else {
              var y = u[f][0];
              this.isThisParam(y) && this.raise(this.start, ne.AccesorCannotDeclareThisParameter), y.type === "Identifier" && y.optional && this.raise(this.start, ne.SetAccesorCannotHaveOptionalParameter), y.type === "RestElement" && this.raise(this.start, ne.SetAccesorCannotHaveRestParameter);
            }
            u[b] && this.raise(u[b].start, ne.SetAccesorCannotHaveReturnType);
          } else u.kind = "method";
          return this.finishNode(u, "TSMethodSignature");
        }
        var g = n;
        i && (g.readonly = true);
        var C = this.tsTryParseTypeAnnotation();
        return C && (g.typeAnnotation = C), this.tsParseTypeMemberSemicolon(), this.finishNode(g, "TSPropertySignature");
      }, c.tsParseTypeMember = function() {
        var a = this.startNode();
        if (this.match(l.parenL) || this.tsMatchLeftRelational()) return this.tsParseSignatureMember("TSCallSignatureDeclaration", a);
        if (this.match(l._new)) {
          var i = this.startNode();
          return this.next(), this.match(l.parenL) || this.tsMatchLeftRelational() ? this.tsParseSignatureMember("TSConstructSignatureDeclaration", a) : (a.key = this.createIdentifier(i, "new"), this.tsParsePropertyOrMethodSignature(a, false));
        }
        return this.tsParseModifiers({ modified: a, allowedModifiers: ["readonly"], disallowedModifiers: ["declare", "abstract", "private", "protected", "public", "static", "override"] }), this.tsTryParseIndexSignature(a) || (this.parsePropertyName(a), a.computed || a.key.type !== "Identifier" || a.key.name !== "get" && a.key.name !== "set" || !this.tsTokenCanFollowModifier() || (a.kind = a.key.name, this.parsePropertyName(a)), this.tsParsePropertyOrMethodSignature(a, !!a.readonly));
      }, c.tsParseList = function(a, i) {
        for (var n = []; !this.tsIsListTerminator(a); ) n.push(i());
        return n;
      }, c.tsParseObjectTypeMembers = function() {
        this.expect(l.braceL);
        var a = this.tsParseList("TypeMembers", this.tsParseTypeMember.bind(this));
        return this.expect(l.braceR), a;
      }, c.tsParseInterfaceDeclaration = function(a, i) {
        if (i === void 0 && (i = {}), this.hasFollowingLineBreak()) return null;
        this.expectContextual("interface"), i.declare && (a.declare = true), D(this.type) ? (a.id = this.parseIdent(), this.checkLValSimple(a.id, 7)) : (a.id = null, this.raise(this.start, ne.MissingInterfaceName)), a.typeParameters = this.tsTryParseTypeParameters(this.tsParseInOutModifiers.bind(this)), this.eat(l._extends) && (a.extends = this.tsParseHeritageClause("extends"));
        var n = this.startNode();
        return n.body = this.tsInType(this.tsParseObjectTypeMembers.bind(this)), a.body = this.finishNode(n, "TSInterfaceBody"), this.finishNode(a, "TSInterfaceDeclaration");
      }, c.tsParseAbstractDeclaration = function(a) {
        if (this.match(l._class)) return a.abstract = true, this.parseClass(a, true);
        if (this.ts_isContextual(B.interface)) {
          if (!this.hasFollowingLineBreak()) return a.abstract = true, this.tsParseInterfaceDeclaration(a);
        } else this.unexpected(a.start);
      }, c.tsIsDeclarationStart = function() {
        return H(this.type);
      }, c.tsParseExpressionStatement = function(a, i) {
        switch (i.name) {
          case "declare":
            var n = this.tsTryParseDeclare(a);
            if (n) return n.declare = true, n;
            break;
          case "global":
            if (this.match(l.braceL)) {
              _.prototype.enterScope.call(this, cr);
              var u = a;
              return u.global = true, u.id = i, u.body = this.tsParseModuleBlock(), _.prototype.exitScope.call(this), this.finishNode(u, "TSModuleDeclaration");
            }
            break;
          default:
            return this.tsParseDeclaration(a, i.name, false);
        }
      }, c.tsParseModuleReference = function() {
        return this.tsIsExternalModuleReference() ? this.tsParseExternalModuleReference() : this.tsParseEntityName(false);
      }, c.tsIsExportDefaultSpecifier = function() {
        var a = this.type, i = this.isAsyncFunction(), n = this.isLet();
        if (D(a)) {
          if (i && !this.containsEsc || n) return false;
          if ((a === B.type || a === B.interface) && !this.containsEsc) {
            var u = this.lookahead();
            if (D(u.type) && !this.isContextualWithState("from", u) || u.type === l.braceL) return false;
          }
        } else if (!this.match(l._default)) return false;
        var f = this.nextTokenStart(), b = this.isUnparsedContextual(f, "from");
        if (this.input.charCodeAt(f) === 44 || D(this.type) && b) return true;
        if (this.match(l._default) && b) {
          var y = this.input.charCodeAt(this.nextTokenStartSince(f + 4));
          return y === 34 || y === 39;
        }
        return false;
      }, c.tsInAmbientContext = function(a) {
        var i = this.isAmbientContext;
        this.isAmbientContext = true;
        try {
          return a();
        } finally {
          this.isAmbientContext = i;
        }
      }, c.tsCheckLineTerminator = function(a) {
        return a ? !this.hasFollowingLineBreak() && (this.next(), true) : !this.isLineTerminator();
      }, c.tsParseModuleOrNamespaceDeclaration = function(a, i) {
        if (i === void 0 && (i = false), a.id = this.parseIdent(), i || this.checkLValSimple(a.id, 8), this.eat(l.dot)) {
          var n = this.startNode();
          this.tsParseModuleOrNamespaceDeclaration(n, true), a.body = n;
        } else _.prototype.enterScope.call(this, cr), a.body = this.tsParseModuleBlock(), _.prototype.exitScope.call(this);
        return this.finishNode(a, "TSModuleDeclaration");
      }, c.checkLValSimple = function(a, i, n) {
        return i === void 0 && (i = 0), _.prototype.checkLValSimple.call(this, a, i, n);
      }, c.tsParseTypeAliasDeclaration = function(a) {
        var i = this;
        return a.id = this.parseIdent(), this.checkLValSimple(a.id, 6), a.typeAnnotation = this.tsInType(function() {
          if (a.typeParameters = i.tsTryParseTypeParameters(i.tsParseInOutModifiers.bind(i)), i.expect(l.eq), i.ts_isContextual(B.interface) && i.lookahead().type !== l.dot) {
            var n = i.startNode();
            return i.next(), i.finishNode(n, "TSIntrinsicKeyword");
          }
          return i.tsParseType();
        }), this.semicolon(), this.finishNode(a, "TSTypeAliasDeclaration");
      }, c.tsParseDeclaration = function(a, i, n) {
        switch (i) {
          case "abstract":
            if (this.tsCheckLineTerminator(n) && (this.match(l._class) || D(this.type))) return this.tsParseAbstractDeclaration(a);
            break;
          case "module":
            if (this.tsCheckLineTerminator(n)) {
              if (this.match(l.string)) return this.tsParseAmbientExternalModuleDeclaration(a);
              if (D(this.type)) return this.tsParseModuleOrNamespaceDeclaration(a);
            }
            break;
          case "namespace":
            if (this.tsCheckLineTerminator(n) && D(this.type)) return this.tsParseModuleOrNamespaceDeclaration(a);
            break;
          case "type":
            if (this.tsCheckLineTerminator(n) && D(this.type)) return this.tsParseTypeAliasDeclaration(a);
        }
      }, c.tsTryParseExportDeclaration = function() {
        return this.tsParseDeclaration(this.startNode(), this.value, true);
      }, c.tsParseImportEqualsDeclaration = function(a, i) {
        a.isExport = i || false, a.id = this.parseIdent(), this.checkLValSimple(a.id, 2), _.prototype.expect.call(this, l.eq);
        var n = this.tsParseModuleReference();
        return a.importKind === "type" && n.type !== "TSExternalModuleReference" && this.raise(n.start, ne.ImportAliasHasImportType), a.moduleReference = n, _.prototype.semicolon.call(this), this.finishNode(a, "TSImportEqualsDeclaration");
      }, c.isExportDefaultSpecifier = function() {
        if (this.tsIsDeclarationStart()) return false;
        var a = this.type;
        if (D(a)) {
          if (this.isContextual("async") || this.isContextual("let")) return false;
          if ((a === B.type || a === B.interface) && !this.containsEsc) {
            var i = this.lookahead();
            if (D(i.type) && !this.isContextualWithState("from", i) || i.type === l.braceL) return false;
          }
        } else if (!this.match(l._default)) return false;
        var n = this.nextTokenStart(), u = this.isUnparsedContextual(n, "from");
        if (this.input.charCodeAt(n) === 44 || D(this.type) && u) return true;
        if (this.match(l._default) && u) {
          var f = this.input.charCodeAt(this.nextTokenStartSince(n + 4));
          return f === 34 || f === 39;
        }
        return false;
      }, c.parseTemplate = function(a) {
        var i = (a === void 0 ? {} : a).isTagged, n = i !== void 0 && i, u = this.startNode();
        this.next(), u.expressions = [];
        var f = this.parseTemplateElement({ isTagged: n });
        for (u.quasis = [f]; !f.tail; ) this.type === l.eof && this.raise(this.pos, "Unterminated template literal"), this.expect(l.dollarBraceL), u.expressions.push(this.inType ? this.tsParseType() : this.parseExpression()), this.expect(l.braceR), u.quasis.push(f = this.parseTemplateElement({ isTagged: n }));
        return this.next(), this.finishNode(u, "TemplateLiteral");
      }, c.parseFunction = function(a, i, n, u, f) {
        this.initFunction(a), (this.options.ecmaVersion >= 9 || this.options.ecmaVersion >= 6 && !u) && (this.type === l.star && 2 & i && this.unexpected(), a.generator = this.eat(l.star)), this.options.ecmaVersion >= 8 && (a.async = !!u), 1 & i && (a.id = 4 & i && this.type !== l.name ? null : this.parseIdent());
        var b = this.yieldPos, y = this.awaitPos, g = this.awaitIdentPos, C = this.maybeInArrowParameters;
        this.maybeInArrowParameters = false, this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0, this.enterScope(ml(a.async, a.generator)), 1 & i || (a.id = this.type === l.name ? this.parseIdent() : null), this.parseFunctionParams(a);
        var q = 1 & i;
        return this.parseFunctionBody(a, n, false, f, { isFunctionDeclaration: q }), this.yieldPos = b, this.awaitPos = y, this.awaitIdentPos = g, 1 & i && a.id && !(2 & i) && this.checkLValSimple(a.id, a.body ? this.strict || a.generator || a.async ? this.treatFunctionsAsVar ? 1 : 2 : 3 : 0), this.maybeInArrowParameters = C, this.finishNode(a, q ? "FunctionDeclaration" : "FunctionExpression");
      }, c.parseFunctionBody = function(a, i, n, u, f) {
        i === void 0 && (i = false), n === void 0 && (n = false), u === void 0 && (u = false), this.match(l.colon) && (a.returnType = this.tsParseTypeOrTypePredicateAnnotation(l.colon));
        var b = f != null && f.isFunctionDeclaration ? "TSDeclareFunction" : f != null && f.isClassMethod ? "TSDeclareMethod" : void 0;
        return b && !this.match(l.braceL) && this.isLineTerminator() ? this.finishNode(a, b) : b === "TSDeclareFunction" && this.isAmbientContext && (this.raise(a.start, ne.DeclareFunctionHasImplementation), a.declare) ? (_.prototype.parseFunctionBody.call(this, a, i, n, false), this.finishNode(a, b)) : (_.prototype.parseFunctionBody.call(this, a, i, n, u), a);
      }, c.parseNew = function() {
        var a;
        this.containsEsc && this.raiseRecoverable(this.start, "Escape sequence in keyword new");
        var i = this.startNode(), n = this.parseIdent(true);
        if (this.options.ecmaVersion >= 6 && this.eat(l.dot)) {
          i.meta = n;
          var u = this.containsEsc;
          return i.property = this.parseIdent(true), i.property.name !== "target" && this.raiseRecoverable(i.property.start, "The only valid meta property for new is 'new.target'"), u && this.raiseRecoverable(i.start, "'new.target' must not contain escaped characters"), this.allowNewDotTarget || this.raiseRecoverable(i.start, "'new.target' can only be used in functions and class static block"), this.finishNode(i, "MetaProperty");
        }
        var f = this.start, b = this.startLoc, y = this.type === l._import;
        i.callee = this.parseSubscripts(this.parseExprAtom(), f, b, true, false), y && i.callee.type === "ImportExpression" && this.raise(f, "Cannot use new with import()");
        var g = i.callee;
        return g.type !== "TSInstantiationExpression" || (a = g.extra) != null && a.parenthesized || (i.typeParameters = g.typeParameters, i.callee = g.expression), i.arguments = this.eat(l.parenL) ? this.parseExprList(l.parenR, this.options.ecmaVersion >= 8, false) : [], this.finishNode(i, "NewExpression");
      }, c.parseExprOp = function(a, i, n, u, f) {
        var b;
        if (l._in.binop > u && !this.hasPrecedingLineBreak() && (this.isContextual("as") && (b = "TSAsExpression"), p && this.isContextual("satisfies") && (b = "TSSatisfiesExpression"), b)) {
          var y = this.startNodeAt(i, n);
          y.expression = a;
          var g = this.tsTryNextParseConstantContext();
          return y.typeAnnotation = g || this.tsNextThenParseType(), this.finishNode(y, b), this.reScan_lt_gt(), this.parseExprOp(y, i, n, u, f);
        }
        return _.prototype.parseExprOp.call(this, a, i, n, u, f);
      }, c.parseImportSpecifiers = function() {
        var a = [], i = true;
        if (m.tokenIsIdentifier(this.type) && (a.push(this.parseImportDefaultSpecifier()), !this.eat(l.comma))) return a;
        if (this.type === l.star) return a.push(this.parseImportNamespaceSpecifier()), a;
        for (this.expect(l.braceL); !this.eat(l.braceR); ) {
          if (i) i = false;
          else if (this.expect(l.comma), this.afterTrailingComma(l.braceR)) break;
          a.push(this.parseImportSpecifier());
        }
        return a;
      }, c.parseImport = function(a) {
        var i = this.lookahead();
        if (a.importKind = "value", this.importOrExportOuterKind = "value", D(i.type) || this.match(l.star) || this.match(l.braceL)) {
          var n = this.lookahead(2);
          if (n.type !== l.comma && !this.isContextualWithState("from", n) && n.type !== l.eq && this.ts_eatContextualWithState("type", 1, i) && (this.importOrExportOuterKind = "type", a.importKind = "type", i = this.lookahead(), n = this.lookahead(2)), D(i.type) && n.type === l.eq) {
            this.next();
            var u = this.tsParseImportEqualsDeclaration(a);
            return this.importOrExportOuterKind = "value", u;
          }
        }
        return this.next(), this.type === l.string ? (a.specifiers = [], a.source = this.parseExprAtom()) : (a.specifiers = this.parseImportSpecifiers(), this.expectContextual("from"), a.source = this.type === l.string ? this.parseExprAtom() : this.unexpected()), this.parseMaybeImportAttributes(a), this.semicolon(), this.finishNode(a, "ImportDeclaration"), this.importOrExportOuterKind = "value", a.importKind === "type" && a.specifiers.length > 1 && a.specifiers[0].type === "ImportDefaultSpecifier" && this.raise(a.start, ne.TypeImportCannotSpecifyDefaultAndNamed), a;
      }, c.parseExportDefaultDeclaration = function() {
        if (this.isAbstractClass()) {
          var a = this.startNode();
          return this.next(), a.abstract = true, this.parseClass(a, true);
        }
        if (this.match(B.interface)) {
          var i = this.tsParseInterfaceDeclaration(this.startNode());
          if (i) return i;
        }
        return _.prototype.parseExportDefaultDeclaration.call(this);
      }, c.parseExportAllDeclaration = function(a, i) {
        return this.options.ecmaVersion >= 11 && (this.eatContextual("as") ? (a.exported = this.parseModuleExportName(), this.checkExport(i, a.exported, this.lastTokStart)) : a.exported = null), this.expectContextual("from"), this.type !== l.string && this.unexpected(), a.source = this.parseExprAtom(), this.parseMaybeImportAttributes(a), this.semicolon(), this.finishNode(a, "ExportAllDeclaration");
      }, c.parseDynamicImport = function(a) {
        if (this.next(), a.source = this.parseMaybeAssign(), this.eat(l.comma)) {
          var i = this.parseExpression();
          a.arguments = [i];
        }
        if (!this.eat(l.parenR)) {
          var n = this.start;
          this.eat(l.comma) && this.eat(l.parenR) ? this.raiseRecoverable(n, "Trailing comma is not allowed in import()") : this.unexpected(n);
        }
        return this.finishNode(a, "ImportExpression");
      }, c.parseExport = function(a, i) {
        var n = this.lookahead();
        if (this.ts_eatWithState(l._import, 2, n)) {
          this.ts_isContextual(B.type) && this.lookaheadCharCode() !== 61 ? (a.importKind = "type", this.importOrExportOuterKind = "type", this.next()) : (a.importKind = "value", this.importOrExportOuterKind = "value");
          var u = this.tsParseImportEqualsDeclaration(a, true);
          return this.importOrExportOuterKind = void 0, u;
        }
        if (this.ts_eatWithState(l.eq, 2, n)) {
          var f = a;
          return f.expression = this.parseExpression(), this.semicolon(), this.importOrExportOuterKind = void 0, this.finishNode(f, "TSExportAssignment");
        }
        if (this.ts_eatContextualWithState("as", 2, n)) {
          var b = a;
          return this.expectContextual("namespace"), b.id = this.parseIdent(), this.semicolon(), this.importOrExportOuterKind = void 0, this.finishNode(b, "TSNamespaceExportDeclaration");
        }
        if (this.ts_isContextualWithState(n, B.type) && this.lookahead(2).type === l.braceL ? (this.next(), this.importOrExportOuterKind = "type", a.exportKind = "type") : (this.importOrExportOuterKind = "value", a.exportKind = "value"), this.next(), this.eat(l.star)) return this.parseExportAllDeclaration(a, i);
        if (this.eat(l._default)) return this.checkExport(i, "default", this.lastTokStart), a.declaration = this.parseExportDefaultDeclaration(), this.finishNode(a, "ExportDefaultDeclaration");
        if (this.shouldParseExportStatement()) a.declaration = this.parseExportDeclaration(a), a.declaration.type === "VariableDeclaration" ? this.checkVariableExport(i, a.declaration.declarations) : this.checkExport(i, a.declaration.id, a.declaration.id.start), a.specifiers = [], a.source = null;
        else {
          if (a.declaration = null, a.specifiers = this.parseExportSpecifiers(i), this.eatContextual("from")) this.type !== l.string && this.unexpected(), a.source = this.parseExprAtom(), this.parseMaybeImportAttributes(a);
          else {
            for (var y, g = Au(a.specifiers); !(y = g()).done; ) {
              var C = y.value;
              this.checkUnreserved(C.local), this.checkLocalExport(C.local), C.local.type === "Literal" && this.raise(C.local.start, "A string literal cannot be used as an exported binding without `from`.");
            }
            a.source = null;
          }
          this.semicolon();
        }
        return this.finishNode(a, "ExportNamedDeclaration");
      }, c.checkExport = function(a, i, n) {
        a && (typeof i != "string" && (i = i.type === "Identifier" ? i.name : i.value), a[i] = true);
      }, c.parseMaybeDefault = function(a, i, n) {
        var u = _.prototype.parseMaybeDefault.call(this, a, i, n);
        return u.type === "AssignmentPattern" && u.typeAnnotation && u.right.start < u.typeAnnotation.start && this.raise(u.typeAnnotation.start, ne.TypeAnnotationAfterAssign), u;
      }, c.typeCastToParameter = function(a) {
        return a.expression.typeAnnotation = a.typeAnnotation, this.resetEndLocation(a.expression, a.typeAnnotation.end), a.expression;
      }, c.toAssignableList = function(a, i) {
        for (var n = 0; n < a.length; n++) {
          var u = a[n];
          (u == null ? void 0 : u.type) === "TSTypeCastExpression" && (a[n] = this.typeCastToParameter(u));
        }
        return _.prototype.toAssignableList.call(this, a, i);
      }, c.reportReservedArrowTypeParam = function(a) {
      }, c.parseExprAtom = function(a, i, n) {
        if (this.type === B.jsxText) return this.jsx_parseText();
        if (this.type === B.jsxTagStart) return this.jsx_parseElement();
        if (this.type === B.at) return this.parseDecorators(), this.parseExprAtom();
        if (D(this.type)) {
          var u = this.potentialArrowAt === this.start, f = this.start, b = this.startLoc, y = this.containsEsc, g = this.parseIdent(false);
          if (this.options.ecmaVersion >= 8 && !y && g.name === "async" && !this.canInsertSemicolon() && this.eat(l._function)) return this.overrideContext(K.f_expr), this.parseFunction(this.startNodeAt(f, b), 0, false, true, i);
          if (u && !this.canInsertSemicolon()) {
            if (this.eat(l.arrow)) return this.parseArrowExpression(this.startNodeAt(f, b), [g], false, i);
            if (this.options.ecmaVersion >= 8 && g.name === "async" && this.type === l.name && !y && (!this.potentialArrowInForAwait || this.value !== "of" || this.containsEsc)) return g = this.parseIdent(false), !this.canInsertSemicolon() && this.eat(l.arrow) || this.unexpected(), this.parseArrowExpression(this.startNodeAt(f, b), [g], true, i);
          }
          return g;
        }
        return _.prototype.parseExprAtom.call(this, a, i, n);
      }, c.parseExprAtomDefault = function() {
        if (D(this.type)) {
          var a = this.potentialArrowAt === this.start, i = this.containsEsc, n = this.parseIdent();
          if (!i && n.name === "async" && !this.canInsertSemicolon()) {
            var u = this.type;
            if (u === l._function) return this.next(), this.parseFunction(this.startNodeAtNode(n), void 0, true, true);
            if (D(u)) {
              if (this.lookaheadCharCode() === 61) {
                var f = this.parseIdent(false);
                return !this.canInsertSemicolon() && this.eat(l.arrow) || this.unexpected(), this.parseArrowExpression(this.startNodeAtNode(n), [f], true);
              }
              return n;
            }
          }
          return a && this.match(l.arrow) && !this.canInsertSemicolon() ? (this.next(), this.parseArrowExpression(this.startNodeAtNode(n), [n], false)) : n;
        }
        this.unexpected();
      }, c.parseIdentNode = function() {
        var a = this.startNode();
        return ie(this.type) ? (a.name = this.value, a) : _.prototype.parseIdentNode.call(this);
      }, c.parseVarStatement = function(a, i, n) {
        n === void 0 && (n = false);
        var u = this.isAmbientContext;
        this.next(), _.prototype.parseVar.call(this, a, false, i, n || u), this.semicolon();
        var f = this.finishNode(a, "VariableDeclaration");
        if (!u) return f;
        for (var b, y = Au(f.declarations); !(b = y()).done; ) {
          var g = b.value, C = g.init;
          C && (i !== "const" || g.id.typeAnnotation ? this.raise(C.start, ne.InitializerNotAllowedInAmbientContext) : C.type !== "StringLiteral" && C.type !== "BooleanLiteral" && C.type !== "NumericLiteral" && C.type !== "BigIntLiteral" && (C.type !== "TemplateLiteral" || C.expressions.length > 0) && !eh(C) && this.raise(C.start, ne.ConstInitiailizerMustBeStringOrNumericLiteralOrLiteralEnumReference));
        }
        return f;
      }, c.parseStatement = function(a, i, n) {
        if (this.match(B.at) && this.parseDecorators(true), this.match(l._const) && this.isLookaheadContextual("enum")) {
          var u = this.startNode();
          return this.expect(l._const), this.tsParseEnumDeclaration(u, { const: true });
        }
        if (this.ts_isContextual(B.enum)) return this.tsParseEnumDeclaration(this.startNode());
        if (this.ts_isContextual(B.interface)) {
          var f = this.tsParseInterfaceDeclaration(this.startNode());
          if (f) return f;
        }
        return _.prototype.parseStatement.call(this, a, i, n);
      }, c.parseAccessModifier = function() {
        return this.tsParseModifier(["public", "protected", "private"]);
      }, c.parsePostMemberNameModifiers = function(a) {
        this.eat(l.question) && (a.optional = true), a.readonly && this.match(l.parenL) && this.raise(a.start, ne.ClassMethodHasReadonly), a.declare && this.match(l.parenL) && this.raise(a.start, ne.ClassMethodHasDeclare);
      }, c.parseExpressionStatement = function(a, i) {
        return (i.type === "Identifier" ? this.tsParseExpressionStatement(a, i) : void 0) || _.prototype.parseExpressionStatement.call(this, a, i);
      }, c.shouldParseExportStatement = function() {
        return !!this.tsIsDeclarationStart() || !!this.match(B.at) || _.prototype.shouldParseExportStatement.call(this);
      }, c.parseConditional = function(a, i, n, u, f) {
        if (this.eat(l.question)) {
          var b = this.startNodeAt(i, n);
          return b.test = a, b.consequent = this.parseMaybeAssign(), this.expect(l.colon), b.alternate = this.parseMaybeAssign(u), this.finishNode(b, "ConditionalExpression");
        }
        return a;
      }, c.parseMaybeConditional = function(a, i) {
        var n = this, u = this.start, f = this.startLoc, b = this.parseExprOps(a, i);
        if (this.checkExpressionErrors(i)) return b;
        if (!this.maybeInArrowParameters || !this.match(l.question)) return this.parseConditional(b, u, f, a, i);
        var y = this.tryParse(function() {
          return n.parseConditional(b, u, f, a, i);
        });
        return y.node ? (y.error && this.setLookaheadState(y.failState), y.node) : (y.error && this.setOptionalParametersError(i, y.error), b);
      }, c.parseParenItem = function(a) {
        var i = this.start, n = this.startLoc;
        if (a = _.prototype.parseParenItem.call(this, a), this.eat(l.question) && (a.optional = true, this.resetEndLocation(a)), this.match(l.colon)) {
          var u = this.startNodeAt(i, n);
          return u.expression = a, u.typeAnnotation = this.tsParseTypeAnnotation(), this.finishNode(u, "TSTypeCastExpression");
        }
        return a;
      }, c.parseExportDeclaration = function(a) {
        var i = this;
        if (!this.isAmbientContext && this.ts_isContextual(B.declare)) return this.tsInAmbientContext(function() {
          return i.parseExportDeclaration(a);
        });
        var n = this.start, u = this.startLoc, f = this.eatContextual("declare");
        !f || !this.ts_isContextual(B.declare) && this.shouldParseExportStatement() || this.raise(this.start, ne.ExpectedAmbientAfterExportDeclare);
        var b = D(this.type) && this.tsTryParseExportDeclaration() || this.parseStatement(null);
        return b ? ((b.type === "TSInterfaceDeclaration" || b.type === "TSTypeAliasDeclaration" || f) && (a.exportKind = "type"), f && (this.resetStartLocation(b, n, u), b.declare = true), b) : null;
      }, c.parseClassId = function(a, i) {
        if (i || !this.isContextual("implements")) {
          _.prototype.parseClassId.call(this, a, i);
          var n = this.tsTryParseTypeParameters(this.tsParseInOutModifiers.bind(this));
          n && (a.typeParameters = n);
        }
      }, c.parseClassPropertyAnnotation = function(a) {
        a.optional || (this.value === "!" && this.eat(l.prefix) ? a.definite = true : this.eat(l.question) && (a.optional = true));
        var i = this.tsTryParseTypeAnnotation();
        i && (a.typeAnnotation = i);
      }, c.parseClassField = function(a) {
        if (a.key.type === "PrivateIdentifier") a.abstract && this.raise(a.start, ne.PrivateElementHasAbstract), a.accessibility && this.raise(a.start, ne.PrivateElementHasAccessibility({ modifier: a.accessibility })), this.parseClassPropertyAnnotation(a);
        else if (this.parseClassPropertyAnnotation(a), this.isAmbientContext && (!a.readonly || a.typeAnnotation) && this.match(l.eq) && this.raise(this.start, ne.DeclareClassFieldHasInitializer), a.abstract && this.match(l.eq)) {
          var i = a.key;
          this.raise(this.start, ne.AbstractPropertyHasInitializer({ propertyName: i.type !== "Identifier" || a.computed ? "[" + this.input.slice(i.start, i.end) + "]" : i.name }));
        }
        return _.prototype.parseClassField.call(this, a);
      }, c.parseClassMethod = function(a, i, n, u) {
        var f = a.kind === "constructor", b = a.key.type === "PrivateIdentifier", y = this.tsTryParseTypeParameters();
        b ? (y && (a.typeParameters = y), a.accessibility && this.raise(a.start, ne.PrivateMethodsHasAccessibility({ modifier: a.accessibility }))) : y && f && this.raise(y.start, ne.ConstructorHasTypeParameters);
        var g = a.declare, C = a.kind;
        !(g !== void 0 && g) || C !== "get" && C !== "set" || this.raise(a.start, ne.DeclareAccessor({ kind: C })), y && (a.typeParameters = y);
        var q = a.key;
        a.kind === "constructor" ? (i && this.raise(q.start, "Constructor can't be a generator"), n && this.raise(q.start, "Constructor can't be an async method")) : a.static && ku(a, "prototype") && this.raise(q.start, "Classes may not have a static property named prototype");
        var V = a.value = this.parseMethod(i, n, u, true, a);
        return a.kind === "get" && V.params.length !== 0 && this.raiseRecoverable(V.start, "getter should have no params"), a.kind === "set" && V.params.length !== 1 && this.raiseRecoverable(V.start, "setter should have exactly one param"), a.kind === "set" && V.params[0].type === "RestElement" && this.raiseRecoverable(V.params[0].start, "Setter cannot use rest params"), this.finishNode(a, "MethodDefinition");
      }, c.isClassMethod = function() {
        return this.match(l.relational);
      }, c.parseClassElement = function(a) {
        var i = this;
        if (this.eat(l.semi)) return null;
        var n, u = this.options.ecmaVersion, f = this.startNode(), b = "", y = false, g = false, C = "method", q = ["declare", "private", "public", "protected", "accessor", "override", "abstract", "readonly", "static"], V = this.tsParseModifiers({ modified: f, allowedModifiers: q, disallowedModifiers: ["in", "out"], stopOnStartOfClassStaticBlock: true, errorTemplate: ne.InvalidModifierOnTypeParameterPositions });
        n = !!V.static;
        var Q = function() {
          if (!i.tsIsStartOfStaticBlocks()) {
            var F = i.tsTryParseIndexSignature(f);
            if (F) return f.abstract && i.raise(f.start, ne.IndexSignatureHasAbstract), f.accessibility && i.raise(f.start, ne.IndexSignatureHasAccessibility({ modifier: f.accessibility })), f.declare && i.raise(f.start, ne.IndexSignatureHasDeclare), f.override && i.raise(f.start, ne.IndexSignatureHasOverride), F;
            if (!i.inAbstractClass && f.abstract && i.raise(f.start, ne.NonAbstractClassHasAbstractMethod), f.override && a && i.raise(f.start, ne.OverrideNotInSubClass), f.static = n, n && (i.isClassElementNameStart() || i.type === l.star || (b = "static")), !b && u >= 8 && i.eatContextual("async") && (!i.isClassElementNameStart() && i.type !== l.star || i.canInsertSemicolon() ? b = "async" : g = true), !b && (u >= 9 || !g) && i.eat(l.star) && (y = true), !b && !g && !y) {
              var ce = i.value;
              (i.eatContextual("get") || i.eatContextual("set")) && (i.isClassElementNameStart() ? C = ce : b = ce);
            }
            if (b ? (f.computed = false, f.key = i.startNodeAt(i.lastTokStart, i.lastTokStartLoc), f.key.name = b, i.finishNode(f.key, "Identifier")) : i.parseClassElementName(f), i.parsePostMemberNameModifiers(f), i.isClassMethod() || u < 13 || i.type === l.parenL || C !== "method" || y || g) {
              var ue = !f.static && ku(f, "constructor"), de = ue && a;
              ue && C !== "method" && i.raise(f.key.start, "Constructor can't have get/set modifier"), f.kind = ue ? "constructor" : C, i.parseClassMethod(f, y, g, de);
            } else i.parseClassField(f);
            return f;
          }
          if (i.next(), i.next(), i.tsHasSomeModifiers(f, q) && i.raise(i.start, ne.StaticBlockCannotHaveModifier), u >= 13) return _.prototype.parseClassStaticBlock.call(i, f), f;
        };
        return f.declare ? this.tsInAmbientContext(Q) : Q(), f;
      }, c.isClassElementNameStart = function() {
        return !!this.tsIsIdentifier() || _.prototype.isClassElementNameStart.call(this);
      }, c.parseClassSuper = function(a) {
        _.prototype.parseClassSuper.call(this, a), a.superClass && (this.tsMatchLeftRelational() || this.match(l.bitShift)) && (a.superTypeParameters = this.tsParseTypeArgumentsInExpression()), this.eatContextual("implements") && (a.implements = this.tsParseHeritageClause("implements"));
      }, c.parseFunctionParams = function(a) {
        var i = this.tsTryParseTypeParameters();
        i && (a.typeParameters = i), _.prototype.parseFunctionParams.call(this, a);
      }, c.parseVarId = function(a, i) {
        _.prototype.parseVarId.call(this, a, i), a.id.type === "Identifier" && !this.hasPrecedingLineBreak() && this.value === "!" && this.eat(l.prefix) && (a.definite = true);
        var n = this.tsTryParseTypeAnnotation();
        n && (a.id.typeAnnotation = n, this.resetEndLocation(a.id));
      }, c.parseArrowExpression = function(a, i, n, u) {
        this.match(l.colon) && (a.returnType = this.tsParseTypeAnnotation());
        var f = this.yieldPos, b = this.awaitPos, y = this.awaitIdentPos;
        this.enterScope(16 | ml(n, false)), this.initFunction(a);
        var g = this.maybeInArrowParameters;
        return this.options.ecmaVersion >= 8 && (a.async = !!n), this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0, this.maybeInArrowParameters = true, a.params = this.toAssignableList(i, true), this.maybeInArrowParameters = false, this.parseFunctionBody(a, true, false, u), this.yieldPos = f, this.awaitPos = b, this.awaitIdentPos = y, this.maybeInArrowParameters = g, this.finishNode(a, "ArrowFunctionExpression");
      }, c.parseMaybeAssignOrigin = function(a, i, n) {
        if (this.isContextual("yield")) {
          if (this.inGenerator) return this.parseYield(a);
          this.exprAllowed = false;
        }
        var u = false, f = -1, b = -1, y = -1;
        i ? (f = i.parenthesizedAssign, b = i.trailingComma, y = i.doubleProto, i.parenthesizedAssign = i.trailingComma = -1) : (i = new dr(), u = true);
        var g = this.start, C = this.startLoc;
        (this.type === l.parenL || D(this.type)) && (this.potentialArrowAt = this.start, this.potentialArrowInForAwait = a === "await");
        var q = this.parseMaybeConditional(a, i);
        if (n && (q = n.call(this, q, g, C)), this.type.isAssign) {
          var V = this.startNodeAt(g, C);
          return V.operator = this.value, this.type === l.eq && (q = this.toAssignable(q, true, i)), u || (i.parenthesizedAssign = i.trailingComma = i.doubleProto = -1), i.shorthandAssign >= q.start && (i.shorthandAssign = -1), this.type === l.eq ? this.checkLValPattern(q) : this.checkLValSimple(q), V.left = q, this.next(), V.right = this.parseMaybeAssign(a), y > -1 && (i.doubleProto = y), this.finishNode(V, "AssignmentExpression");
        }
        return u && this.checkExpressionErrors(i, true), f > -1 && (i.parenthesizedAssign = f), b > -1 && (i.trailingComma = b), q;
      }, c.parseMaybeAssign = function(a, i, n) {
        var u, f, b, y, g, C, q, V, Q, F, ce, ue = this;
        if (this.matchJsx("jsxTagStart") || this.tsMatchLeftRelational()) {
          if (V = this.cloneCurLookaheadState(), !(Q = this.tryParse(function() {
            return ue.parseMaybeAssignOrigin(a, i, n);
          }, V)).error) return Q.node;
          var de = this.context, be = de[de.length - 1];
          be === m.tokContexts.tc_oTag && de[de.length - 2] === m.tokContexts.tc_expr ? (de.pop(), de.pop()) : be !== m.tokContexts.tc_oTag && be !== m.tokContexts.tc_expr || de.pop();
        }
        if (!((u = Q) != null && u.error || this.tsMatchLeftRelational())) return this.parseMaybeAssignOrigin(a, i, n);
        V && !this.compareLookaheadState(V, this.getCurLookaheadState()) || (V = this.cloneCurLookaheadState());
        var ke = this.tryParse(function(Oe) {
          var qe, He;
          ce = ue.tsParseTypeParameters();
          var Ne = ue.parseMaybeAssignOrigin(a, i, n);
          return (Ne.type !== "ArrowFunctionExpression" || (qe = Ne.extra) != null && qe.parenthesized) && Oe(), ((He = ce) == null ? void 0 : He.params.length) !== 0 && ue.resetStartLocationFromNode(Ne, ce), Ne.typeParameters = ce, Ne;
        }, V);
        if (!ke.error && !ke.aborted) return ce && this.reportReservedArrowTypeParam(ce), ke.node;
        if (!Q && (Iu(true), !(F = this.tryParse(function() {
          return ue.parseMaybeAssignOrigin(a, i, n);
        }, V)).error)) return F.node;
        if ((f = Q) != null && f.node) return this.setLookaheadState(Q.failState), Q.node;
        if (ke.node) return this.setLookaheadState(ke.failState), ce && this.reportReservedArrowTypeParam(ce), ke.node;
        if ((b = F) != null && b.node) return this.setLookaheadState(F.failState), F.node;
        throw (y = Q) != null && y.thrown ? Q.error : ke.thrown ? ke.error : (g = F) != null && g.thrown ? F.error : ((C = Q) == null ? void 0 : C.error) || ke.error || ((q = F) == null ? void 0 : q.error);
      }, c.parseAssignableListItem = function(a) {
        for (var i = []; this.match(B.at); ) i.push(this.parseDecorator());
        var n, u = this.start, f = this.startLoc, b = false, y = false;
        if (a !== void 0) {
          var g = {};
          this.tsParseModifiers({ modified: g, allowedModifiers: ["public", "private", "protected", "override", "readonly"] }), n = g.accessibility, y = g.override, b = g.readonly, a === false && (n || b || y) && this.raise(f.start, ne.UnexpectedParameterModifier);
        }
        var C = this.parseMaybeDefault(u, f);
        this.parseBindingListItem(C);
        var q = this.parseMaybeDefault(C.start, C.loc, C);
        if (i.length && (q.decorators = i), n || b || y) {
          var V = this.startNodeAt(u, f);
          return n && (V.accessibility = n), b && (V.readonly = b), y && (V.override = y), q.type !== "Identifier" && q.type !== "AssignmentPattern" && this.raise(V.start, ne.UnsupportedParameterPropertyKind), V.parameter = q, this.finishNode(V, "TSParameterProperty");
        }
        return q;
      }, c.checkLValInnerPattern = function(a, i, n) {
        i === void 0 && (i = 0), a.type === "TSParameterProperty" ? this.checkLValInnerPattern(a.parameter, i, n) : _.prototype.checkLValInnerPattern.call(this, a, i, n);
      }, c.parseBindingListItem = function(a) {
        this.eat(l.question) && (a.type === "Identifier" || this.isAmbientContext || this.inType || this.raise(a.start, ne.PatternIsOptional), a.optional = true);
        var i = this.tsTryParseTypeAnnotation();
        return i && (a.typeAnnotation = i), this.resetEndLocation(a), a;
      }, c.isAssignable = function(a, i) {
        var n = this;
        switch (a.type) {
          case "TSTypeCastExpression":
            return this.isAssignable(a.expression, i);
          case "TSParameterProperty":
          case "Identifier":
          case "ObjectPattern":
          case "ArrayPattern":
          case "AssignmentPattern":
          case "RestElement":
            return true;
          case "ObjectExpression":
            var u = a.properties.length - 1;
            return a.properties.every(function(f, b) {
              return f.type !== "ObjectMethod" && (b === u || f.type !== "SpreadElement") && n.isAssignable(f);
            });
          case "Property":
          case "ObjectProperty":
            return this.isAssignable(a.value);
          case "SpreadElement":
            return this.isAssignable(a.argument);
          case "ArrayExpression":
            return a.elements.every(function(f) {
              return f === null || n.isAssignable(f);
            });
          case "AssignmentExpression":
            return a.operator === "=";
          case "ParenthesizedExpression":
            return this.isAssignable(a.expression);
          case "MemberExpression":
          case "OptionalMemberExpression":
            return !i;
          default:
            return false;
        }
      }, c.toAssignable = function(a, i, n) {
        switch (i === void 0 && (i = false), n === void 0 && (n = new dr()), a.type) {
          case "ParenthesizedExpression":
            return this.toAssignableParenthesizedExpression(a, i, n);
          case "TSAsExpression":
          case "TSSatisfiesExpression":
          case "TSNonNullExpression":
          case "TSTypeAssertion":
            return i || this.raise(a.start, ne.UnexpectedTypeCastInParameter), this.toAssignable(a.expression, i, n);
          case "MemberExpression":
            break;
          case "AssignmentExpression":
            return i || a.left.type !== "TSTypeCastExpression" || (a.left = this.typeCastToParameter(a.left)), _.prototype.toAssignable.call(this, a, i, n);
          case "TSTypeCastExpression":
            return this.typeCastToParameter(a);
          default:
            return _.prototype.toAssignable.call(this, a, i, n);
        }
        return a;
      }, c.toAssignableParenthesizedExpression = function(a, i, n) {
        switch (a.expression.type) {
          case "TSAsExpression":
          case "TSSatisfiesExpression":
          case "TSNonNullExpression":
          case "TSTypeAssertion":
          case "ParenthesizedExpression":
            return this.toAssignable(a.expression, i, n);
          default:
            return _.prototype.toAssignable.call(this, a, i, n);
        }
      }, c.curPosition = function() {
        if (this.options.locations) {
          var a = _.prototype.curPosition.call(this);
          return Object.defineProperty(a, "offset", { get: function() {
            return function(i) {
              var n = new v.Position(this.line, this.column + i);
              return n.index = this.index + i, n;
            };
          } }), a.index = this.pos, a;
        }
      }, c.parseBindingAtom = function() {
        return this.type === l._this ? this.parseIdent(true) : _.prototype.parseBindingAtom.call(this);
      }, c.shouldParseArrow = function(a) {
        var i, n = this;
        if (i = this.match(l.colon) ? a.every(function(f) {
          return n.isAssignable(f, true);
        }) : !this.canInsertSemicolon()) {
          if (this.match(l.colon)) {
            var u = this.tryParse(function(f) {
              var b = n.tsParseTypeOrTypePredicateAnnotation(l.colon);
              return !n.canInsertSemicolon() && n.match(l.arrow) || f(), b;
            });
            if (u.aborted) return this.shouldParseArrowReturnType = void 0, false;
            u.thrown || (u.error && this.setLookaheadState(u.failState), this.shouldParseArrowReturnType = u.node);
          }
          return !!this.match(l.arrow) || (this.shouldParseArrowReturnType = void 0, false);
        }
        return this.shouldParseArrowReturnType = void 0, i;
      }, c.parseParenArrowList = function(a, i, n, u) {
        var f = this.startNodeAt(a, i);
        return f.returnType = this.shouldParseArrowReturnType, this.shouldParseArrowReturnType = void 0, this.parseArrowExpression(f, n, false, u);
      }, c.parseParenAndDistinguishExpression = function(a, i) {
        var n, u = this.start, f = this.startLoc, b = this.options.ecmaVersion >= 8;
        if (this.options.ecmaVersion >= 6) {
          var y = this.maybeInArrowParameters;
          this.maybeInArrowParameters = true, this.next();
          var g, C = this.start, q = this.startLoc, V = [], Q = true, F = false, ce = new dr(), ue = this.yieldPos, de = this.awaitPos;
          for (this.yieldPos = 0, this.awaitPos = 0; this.type !== l.parenR; ) {
            if (Q ? Q = false : this.expect(l.comma), b && this.afterTrailingComma(l.parenR, true)) {
              F = true;
              break;
            }
            if (this.type === l.ellipsis) {
              g = this.start, V.push(this.parseParenItem(this.parseRestBinding())), this.type === l.comma && this.raise(this.start, "Comma is not permitted after the rest element");
              break;
            }
            V.push(this.parseMaybeAssign(i, ce, this.parseParenItem));
          }
          var be = this.lastTokEnd, ke = this.lastTokEndLoc;
          if (this.expect(l.parenR), this.maybeInArrowParameters = y, a && this.shouldParseArrow(V) && this.eat(l.arrow)) return this.checkPatternErrors(ce, false), this.checkYieldAwaitInDefaultParams(), this.yieldPos = ue, this.awaitPos = de, this.parseParenArrowList(u, f, V, i);
          V.length && !F || this.unexpected(this.lastTokStart), g && this.unexpected(g), this.checkExpressionErrors(ce, true), this.yieldPos = ue || this.yieldPos, this.awaitPos = de || this.awaitPos, V.length > 1 ? ((n = this.startNodeAt(C, q)).expressions = V, this.finishNodeAt(n, "SequenceExpression", be, ke)) : n = V[0];
        } else n = this.parseParenExpression();
        if (this.options.preserveParens) {
          var Oe = this.startNodeAt(u, f);
          return Oe.expression = n, this.finishNode(Oe, "ParenthesizedExpression");
        }
        return n;
      }, c.parseTaggedTemplateExpression = function(a, i, n, u) {
        var f = this.startNodeAt(i, n);
        return f.tag = a, f.quasi = this.parseTemplate({ isTagged: true }), u && this.raise(i, "Tagged Template Literals are not allowed in optionalChain."), this.finishNode(f, "TaggedTemplateExpression");
      }, c.shouldParseAsyncArrow = function() {
        var a = this;
        if (!this.match(l.colon)) return !this.canInsertSemicolon() && this.eat(l.arrow);
        var i = this.tryParse(function(n) {
          var u = a.tsParseTypeOrTypePredicateAnnotation(l.colon);
          return !a.canInsertSemicolon() && a.match(l.arrow) || n(), u;
        });
        return i.aborted ? (this.shouldParseAsyncArrowReturnType = void 0, false) : i.thrown ? void 0 : (i.error && this.setLookaheadState(i.failState), this.shouldParseAsyncArrowReturnType = i.node, !this.canInsertSemicolon() && this.eat(l.arrow));
      }, c.parseSubscriptAsyncArrow = function(a, i, n, u) {
        var f = this.startNodeAt(a, i);
        return f.returnType = this.shouldParseAsyncArrowReturnType, this.shouldParseAsyncArrowReturnType = void 0, this.parseArrowExpression(f, n, true, u);
      }, c.parseExprList = function(a, i, n, u) {
        for (var f = [], b = true; !this.eat(a); ) {
          if (b) b = false;
          else if (this.expect(l.comma), i && this.afterTrailingComma(a)) break;
          var y = void 0;
          n && this.type === l.comma ? y = null : this.type === l.ellipsis ? (y = this.parseSpread(u), u && this.type === l.comma && u.trailingComma < 0 && (u.trailingComma = this.start)) : y = this.parseMaybeAssign(false, u, this.parseParenItem), f.push(y);
        }
        return f;
      }, c.parseSubscript = function(a, i, n, u, f, b, y) {
        var g = this, C = b;
        if (!this.hasPrecedingLineBreak() && this.value === "!" && this.match(l.prefix)) {
          this.exprAllowed = false, this.next();
          var q = this.startNodeAt(i, n);
          return q.expression = a, a = this.finishNode(q, "TSNonNullExpression");
        }
        var V = false;
        if (this.match(l.questionDot) && this.lookaheadCharCode() === 60) {
          if (u) return a;
          a.optional = true, C = V = true, this.next();
        }
        if (this.tsMatchLeftRelational() || this.match(l.bitShift)) {
          var Q, F = this.tsTryParseAndCatch(function() {
            if (!u && g.atPossibleAsyncArrow(a)) {
              var jt = g.tsTryParseGenericAsyncArrowFunction(i, n, y);
              if (jt) return a = jt;
            }
            var T = g.tsParseTypeArgumentsInExpression();
            if (!T) return a;
            if (V && !g.match(l.parenL)) return Q = g.curPosition(), a;
            if (O(g.type) || g.type === l.backQuote) {
              var N = g.parseTaggedTemplateExpression(a, i, n, C);
              return N.typeParameters = T, N;
            }
            if (!u && g.eat(l.parenL)) {
              var j = new dr(), G = g.startNodeAt(i, n);
              return G.callee = a, G.arguments = g.parseExprList(l.parenR, g.options.ecmaVersion >= 8, false, j), g.tsCheckForInvalidTypeCasts(G.arguments), G.typeParameters = T, C && (G.optional = V), g.checkExpressionErrors(j, true), a = g.finishNode(G, "CallExpression");
            }
            var le = g.type;
            if (!(g.tsMatchRightRelational() || le === l.bitShift || le !== l.parenL && (ae = le, !!ae.startsExpr) && !g.hasPrecedingLineBreak())) {
              var ae, Re = g.startNodeAt(i, n);
              return Re.expression = a, Re.typeParameters = T, g.finishNode(Re, "TSInstantiationExpression");
            }
          });
          if (Q && this.unexpected(Q), F) return F.type === "TSInstantiationExpression" && (this.match(l.dot) || this.match(l.questionDot) && this.lookaheadCharCode() !== 40) && this.raise(this.start, ne.InvalidPropertyAccessAfterInstantiationExpression), a = F;
        }
        var ce = this.options.ecmaVersion >= 11, ue = ce && this.eat(l.questionDot);
        u && ue && this.raise(this.lastTokStart, "Optional chaining cannot appear in the callee of new expressions");
        var de = this.eat(l.bracketL);
        if (de || ue && this.type !== l.parenL && this.type !== l.backQuote || this.eat(l.dot)) {
          var be = this.startNodeAt(i, n);
          be.object = a, de ? (be.property = this.parseExpression(), this.expect(l.bracketR)) : be.property = this.type === l.privateId && a.type !== "Super" ? this.parsePrivateIdent() : this.parseIdent(this.options.allowReserved !== "never"), be.computed = !!de, ce && (be.optional = ue), a = this.finishNode(be, "MemberExpression");
        } else if (!u && this.eat(l.parenL)) {
          var ke = this.maybeInArrowParameters;
          this.maybeInArrowParameters = true;
          var Oe = new dr(), qe = this.yieldPos, He = this.awaitPos, Ne = this.awaitIdentPos;
          this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0;
          var it = this.parseExprList(l.parenR, this.options.ecmaVersion >= 8, false, Oe);
          if (f && !ue && this.shouldParseAsyncArrow()) this.checkPatternErrors(Oe, false), this.checkYieldAwaitInDefaultParams(), this.awaitIdentPos > 0 && this.raise(this.awaitIdentPos, "Cannot use 'await' as identifier inside an async function"), this.yieldPos = qe, this.awaitPos = He, this.awaitIdentPos = Ne, a = this.parseSubscriptAsyncArrow(i, n, it, y);
          else {
            this.checkExpressionErrors(Oe, true), this.yieldPos = qe || this.yieldPos, this.awaitPos = He || this.awaitPos, this.awaitIdentPos = Ne || this.awaitIdentPos;
            var Je = this.startNodeAt(i, n);
            Je.callee = a, Je.arguments = it, ce && (Je.optional = ue), a = this.finishNode(Je, "CallExpression");
          }
          this.maybeInArrowParameters = ke;
        } else if (this.type === l.backQuote) {
          (ue || C) && this.raise(this.start, "Optional chaining cannot appear in the tag of tagged template expressions");
          var Dt = this.startNodeAt(i, n);
          Dt.tag = a, Dt.quasi = this.parseTemplate({ isTagged: true }), a = this.finishNode(Dt, "TaggedTemplateExpression");
        }
        return a;
      }, c.parseGetterSetter = function(a) {
        a.kind = a.key.name, this.parsePropertyName(a), a.value = this.parseMethod(false);
        var i = a.kind === "get" ? 0 : 1, n = a.value.params[0], u = n && this.isThisParam(n);
        a.value.params.length !== (i = u ? i + 1 : i) ? this.raiseRecoverable(a.value.start, a.kind === "get" ? "getter should have no params" : "setter should have exactly one param") : a.kind === "set" && a.value.params[0].type === "RestElement" && this.raiseRecoverable(a.value.params[0].start, "Setter cannot use rest params");
      }, c.parseProperty = function(a, i) {
        if (!a) {
          var n = [];
          if (this.match(B.at)) for (; this.match(B.at); ) n.push(this.parseDecorator());
          var u = _.prototype.parseProperty.call(this, a, i);
          return u.type === "SpreadElement" && n.length && this.raise(u.start, "Decorators can't be used with SpreadElement"), n.length && (u.decorators = n, n = []), u;
        }
        return _.prototype.parseProperty.call(this, a, i);
      }, c.parseCatchClauseParam = function() {
        var a = this.parseBindingAtom(), i = a.type === "Identifier";
        this.enterScope(i ? 32 : 0), this.checkLValPattern(a, i ? 4 : 2);
        var n = this.tsTryParseTypeAnnotation();
        return n && (a.typeAnnotation = n, this.resetEndLocation(a)), this.expect(l.parenR), a;
      }, c.parseClass = function(a, i) {
        var n = this.inAbstractClass;
        this.inAbstractClass = !!a.abstract;
        try {
          this.next(), this.takeDecorators(a);
          var u = this.strict;
          this.strict = true, this.parseClassId(a, i), this.parseClassSuper(a);
          var f = this.enterClassBody(), b = this.startNode(), y = false;
          b.body = [];
          var g = [];
          for (this.expect(l.braceL); this.type !== l.braceR; ) if (this.match(B.at)) g.push(this.parseDecorator());
          else {
            var C = this.parseClassElement(a.superClass !== null);
            g.length && (C.decorators = g, this.resetStartLocationFromNode(C, g[0]), g = []), C && (b.body.push(C), C.type === "MethodDefinition" && C.kind === "constructor" && C.value.type === "FunctionExpression" ? (y && this.raiseRecoverable(C.start, "Duplicate constructor in the same class"), y = true, C.decorators && C.decorators.length > 0 && this.raise(C.start, "Decorators can't be used with a constructor. Did you mean '@dec class { ... }'?")) : C.key && C.key.type === "PrivateIdentifier" && Kp(f, C) && this.raiseRecoverable(C.key.start, "Identifier '#" + C.key.name + "' has already been declared"));
          }
          return this.strict = u, this.next(), g.length && this.raise(this.start, "Decorators must be attached to a class element."), a.body = this.finishNode(b, "ClassBody"), this.exitClassBody(), this.finishNode(a, i ? "ClassDeclaration" : "ClassExpression");
        } finally {
          this.inAbstractClass = n;
        }
      }, c.parseClassFunctionParams = function() {
        var a = this.tsTryParseTypeParameters(this.tsParseConstModifier), i = this.parseBindingList(l.parenR, false, this.options.ecmaVersion >= 8, true);
        return a && (i.typeParameters = a), i;
      }, c.parseMethod = function(a, i, n, u, f) {
        var b = this.startNode(), y = this.yieldPos, g = this.awaitPos, C = this.awaitIdentPos;
        if (this.initFunction(b), this.options.ecmaVersion >= 6 && (b.generator = a), this.options.ecmaVersion >= 8 && (b.async = !!i), this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0, this.enterScope(64 | ml(i, b.generator) | (n ? 128 : 0)), this.expect(l.parenL), b.params = this.parseClassFunctionParams(), this.checkYieldAwaitInDefaultParams(), this.parseFunctionBody(b, false, true, false, { isClassMethod: u }), this.yieldPos = y, this.awaitPos = g, this.awaitIdentPos = C, f && f.abstract && b.body) {
          var q = f.key;
          this.raise(f.start, ne.AbstractMethodHasImplementation({ methodName: q.type !== "Identifier" || f.computed ? "[" + this.input.slice(q.start, q.end) + "]" : q.name }));
        }
        return this.finishNode(b, "FunctionExpression");
      }, S.parse = function(a, i) {
        if (i.locations === false) throw new Error("You have to enable options.locations while using acorn-typescript");
        i.locations = true;
        var n = new this(i, a);
        return s && (n.isAmbientContext = true), n.parse();
      }, S.parseExpressionAt = function(a, i, n) {
        if (n.locations === false) throw new Error("You have to enable options.locations while using acorn-typescript");
        n.locations = true;
        var u = new this(n, a, i);
        return s && (u.isAmbientContext = true), u.nextToken(), u.parseExpression();
      }, c.parseImportSpecifier = function() {
        if (this.ts_isContextual(B.type)) {
          var a = this.startNode();
          return a.imported = this.parseModuleExportName(), this.parseTypeOnlyImportExportSpecifier(a, true, this.importOrExportOuterKind === "type"), this.finishNode(a, "ImportSpecifier");
        }
        var i = _.prototype.parseImportSpecifier.call(this);
        return i.importKind = "value", i;
      }, c.parseExportSpecifier = function(a) {
        var i = this.ts_isContextual(B.type);
        if (!this.match(l.string) && i) {
          var n = this.startNode();
          return n.local = this.parseModuleExportName(), this.parseTypeOnlyImportExportSpecifier(n, false, this.importOrExportOuterKind === "type"), this.finishNode(n, "ExportSpecifier"), this.checkExport(a, n.exported, n.exported.start), n;
        }
        var u = _.prototype.parseExportSpecifier.call(this, a);
        return u.exportKind = "value", u;
      }, c.parseTypeOnlyImportExportSpecifier = function(a, i, n) {
        var u, f = i ? "imported" : "local", b = i ? "local" : "exported", y = a[f], g = false, C = true, q = y.start;
        if (this.isContextual("as")) {
          var V = this.parseIdent();
          if (this.isContextual("as")) {
            var Q = this.parseIdent();
            ie(this.type) ? (g = true, y = V, u = i ? this.parseIdent() : this.parseModuleExportName(), C = false) : (u = Q, C = false);
          } else ie(this.type) ? (C = false, u = i ? this.parseIdent() : this.parseModuleExportName()) : (g = true, y = V);
        } else ie(this.type) && (g = true, i ? (y = _.prototype.parseIdent.call(this, true), this.isContextual("as") || this.checkUnreserved(y)) : y = this.parseModuleExportName());
        g && n && this.raise(q, i ? ne.TypeModifierIsUsedInTypeImports : ne.TypeModifierIsUsedInTypeExports), a[f] = y, a[b] = u, a[i ? "importKind" : "exportKind"] = g ? "type" : "value", C && this.eatContextual("as") && (a[b] = i ? this.parseIdent() : this.parseModuleExportName()), a[b] || (a[b] = this.copyNode(a[f])), i && this.checkLValSimple(a[b], 2);
      }, c.raiseCommonCheck = function(a, i, n) {
        return i === "Comma is not permitted after the rest element" ? this.isAmbientContext && this.match(l.comma) && this.lookaheadCharCode() === 41 ? void this.next() : _.prototype.raise.call(this, a, i) : n ? _.prototype.raiseRecoverable.call(this, a, i) : _.prototype.raise.call(this, a, i);
      }, c.raiseRecoverable = function(a, i) {
        return this.raiseCommonCheck(a, i, true);
      }, c.raise = function(a, i) {
        return this.raiseCommonCheck(a, i, true);
      }, c.updateContext = function(a) {
        var i = this.type;
        if (i == l.braceL) {
          var n = this.curContext();
          n == $.tc_oTag ? this.context.push(K.b_expr) : n == $.tc_expr ? this.context.push(K.b_tmpl) : _.prototype.updateContext.call(this, a), this.exprAllowed = true;
        } else {
          if (i !== l.slash || a !== B.jsxTagStart) return _.prototype.updateContext.call(this, a);
          this.context.length -= 2, this.context.push($.tc_cTag), this.exprAllowed = false;
        }
      }, c.jsx_parseOpeningElementAt = function(a, i) {
        var n = this, u = this.startNodeAt(a, i), f = this.jsx_parseElementName();
        if (f && (u.name = f), this.match(l.relational) || this.match(l.bitShift)) {
          var b = this.tsTryParseAndCatch(function() {
            return n.tsParseTypeArgumentsInExpression();
          });
          b && (u.typeParameters = b);
        }
        for (u.attributes = []; this.type !== l.slash && this.type !== B.jsxTagEnd; ) u.attributes.push(this.jsx_parseAttribute());
        return u.selfClosing = this.eat(l.slash), this.expect(B.jsxTagEnd), this.finishNode(u, f ? "JSXOpeningElement" : "JSXOpeningFragment");
      }, c.enterScope = function(a) {
        a === cr && this.importsStack.push([]), _.prototype.enterScope.call(this, a);
        var i = _.prototype.currentScope.call(this);
        i.types = [], i.enums = [], i.constEnums = [], i.classes = [], i.exportOnlyBindings = [];
      }, c.exitScope = function() {
        _.prototype.currentScope.call(this).flags === cr && this.importsStack.pop(), _.prototype.exitScope.call(this);
      }, c.hasImport = function(a, i) {
        var n = this.importsStack.length;
        if (this.importsStack[n - 1].indexOf(a) > -1) return true;
        if (!i && n > 1) {
          for (var u = 0; u < n - 1; u++) if (this.importsStack[u].indexOf(a) > -1) return true;
        }
        return false;
      }, c.maybeExportDefined = function(a, i) {
        this.inModule && 1 & a.flags && this.undefinedExports.delete(i);
      }, c.isRedeclaredInScope = function(a, i, n) {
        return !!(0 & n) && (2 & n ? a.lexical.indexOf(i) > -1 || a.functions.indexOf(i) > -1 || a.var.indexOf(i) > -1 : 3 & n ? a.lexical.indexOf(i) > -1 || !_.prototype.treatFunctionsAsVarInScope.call(this, a) && a.var.indexOf(i) > -1 : a.lexical.indexOf(i) > -1 && !(32 & a.flags && a.lexical[0] === i) || !this.treatFunctionsAsVarInScope(a) && a.functions.indexOf(i) > -1);
      }, c.checkRedeclarationInScope = function(a, i, n, u) {
        this.isRedeclaredInScope(a, i, n) && this.raise(u, "Identifier '" + i + "' has already been declared.");
      }, c.declareName = function(a, i, n) {
        if (4096 & i) return this.hasImport(a, true) && this.raise(n, "Identifier '" + a + "' has already been declared."), void this.importsStack[this.importsStack.length - 1].push(a);
        var u = this.currentScope();
        if (1024 & i) return this.maybeExportDefined(u, a), void u.exportOnlyBindings.push(a);
        _.prototype.declareName.call(this, a, i, n), 0 & i && (0 & i || (this.checkRedeclarationInScope(u, a, i, n), this.maybeExportDefined(u, a)), u.types.push(a)), 256 & i && u.enums.push(a), 512 & i && u.constEnums.push(a), 128 & i && u.classes.push(a);
      }, c.checkLocalExport = function(a) {
        var i = a.name;
        if (!this.hasImport(i)) {
          for (var n = this.scopeStack.length - 1; n >= 0; n--) {
            var u = this.scopeStack[n];
            if (u.types.indexOf(i) > -1 || u.exportOnlyBindings.indexOf(i) > -1) return;
          }
          _.prototype.checkLocalExport.call(this, a);
        }
      }, R = S, A = [{ key: "acornTypeScript", get: function() {
        return m;
      } }], (P = [{ key: "acornTypeScript", get: function() {
        return m;
      } }]) && Tu(R.prototype, P), A && Tu(R, A), Object.defineProperty(R, "prototype", { writable: false }), S;
    }(h);
    return me;
  };
}
Be.extend(th({ allowSatisfies: true }));
class rh extends Error {
  constructor(t, r, s) {
    super(r);
    __publicField(this, "message", "");
    __privateAdd(this, _e2);
    this.stack = "", __privateSet(this, _e2, new pc(t, r, s)), Object.assign(this, __privateGet(this, _e2)), this.name = "CompileError";
  }
  toString() {
    return __privateGet(this, _e2).toString();
  }
  toJSON() {
    return __privateGet(this, _e2).toJSON();
  }
}
_e2 = new WeakMap();
function ou(e, t, r) {
  throw new rh(t, r, void 0);
}
function ah(e, t) {
  ou(e, "options_invalid_value", `Invalid compiler option: ${t}
https://svelte.dev/e/options_invalid_value`);
}
function ih(e, t) {
  ou(e, "options_removed", `Invalid compiler option: ${t}
https://svelte.dev/e/options_removed`);
}
function sh(e, t) {
  ou(e, "options_unrecognised", `Unrecognised compiler option ${t}
https://svelte.dev/e/options_unrecognised`);
}
const nh = { "CounterClockwiseContourIntegral;": 8755, "ClockwiseContourIntegral;": 8754, "DoubleLongLeftRightArrow;": 10234, "NotNestedGreaterGreater;": 10914, "DiacriticalDoubleAcute;": 733, "NotSquareSupersetEqual;": 8931, "CloseCurlyDoubleQuote;": 8221, "DoubleContourIntegral;": 8751, "FilledVerySmallSquare;": 9642, "NegativeVeryThinSpace;": 8203, "NotPrecedesSlantEqual;": 8928, "NotRightTriangleEqual;": 8941, "NotSucceedsSlantEqual;": 8929, "CapitalDifferentialD;": 8517, "DoubleLeftRightArrow;": 8660, "DoubleLongRightArrow;": 10233, "EmptyVerySmallSquare;": 9643, "NestedGreaterGreater;": 8811, "NotDoubleVerticalBar;": 8742, "NotGreaterSlantEqual;": 10878, "NotLeftTriangleEqual;": 8940, "NotSquareSubsetEqual;": 8930, "OpenCurlyDoubleQuote;": 8220, "ReverseUpEquilibrium;": 10607, "DoubleLongLeftArrow;": 10232, "DownLeftRightVector;": 10576, "LeftArrowRightArrow;": 8646, "NegativeMediumSpace;": 8203, "NotGreaterFullEqual;": 8807, "NotRightTriangleBar;": 10704, "RightArrowLeftArrow;": 8644, "SquareSupersetEqual;": 8850, "leftrightsquigarrow;": 8621, "DownRightTeeVector;": 10591, "DownRightVectorBar;": 10583, "LongLeftRightArrow;": 10231, "Longleftrightarrow;": 10234, "NegativeThickSpace;": 8203, "NotLeftTriangleBar;": 10703, "PrecedesSlantEqual;": 8828, "ReverseEquilibrium;": 8651, "RightDoubleBracket;": 10215, "RightDownTeeVector;": 10589, "RightDownVectorBar;": 10581, "RightTriangleEqual;": 8885, "SquareIntersection;": 8851, "SucceedsSlantEqual;": 8829, "blacktriangleright;": 9656, "longleftrightarrow;": 10231, "DoubleUpDownArrow;": 8661, "DoubleVerticalBar;": 8741, "DownLeftTeeVector;": 10590, "DownLeftVectorBar;": 10582, "FilledSmallSquare;": 9724, "GreaterSlantEqual;": 10878, "LeftDoubleBracket;": 10214, "LeftDownTeeVector;": 10593, "LeftDownVectorBar;": 10585, "LeftTriangleEqual;": 8884, "NegativeThinSpace;": 8203, "NotGreaterGreater;": 8811, "NotLessSlantEqual;": 10877, "NotNestedLessLess;": 10913, "NotReverseElement;": 8716, "NotSquareSuperset;": 8848, "NotTildeFullEqual;": 8775, "RightAngleBracket;": 10217, "RightUpDownVector;": 10575, "SquareSubsetEqual;": 8849, "VerticalSeparator;": 10072, "blacktriangledown;": 9662, "blacktriangleleft;": 9666, "leftrightharpoons;": 8651, "rightleftharpoons;": 8652, "twoheadrightarrow;": 8608, "DiacriticalAcute;": 180, "DiacriticalGrave;": 96, "DiacriticalTilde;": 732, "DoubleRightArrow;": 8658, "DownArrowUpArrow;": 8693, "EmptySmallSquare;": 9723, "GreaterEqualLess;": 8923, "GreaterFullEqual;": 8807, "LeftAngleBracket;": 10216, "LeftUpDownVector;": 10577, "LessEqualGreater;": 8922, "NonBreakingSpace;": 160, "NotPrecedesEqual;": 10927, "NotRightTriangle;": 8939, "NotSucceedsEqual;": 10928, "NotSucceedsTilde;": 8831, "NotSupersetEqual;": 8841, "RightTriangleBar;": 10704, "RightUpTeeVector;": 10588, "RightUpVectorBar;": 10580, "UnderParenthesis;": 9181, "UpArrowDownArrow;": 8645, "circlearrowright;": 8635, "downharpoonright;": 8642, "ntrianglerighteq;": 8941, "rightharpoondown;": 8641, "rightrightarrows;": 8649, "twoheadleftarrow;": 8606, "vartriangleright;": 8883, "CloseCurlyQuote;": 8217, "ContourIntegral;": 8750, "DoubleDownArrow;": 8659, "DoubleLeftArrow;": 8656, "DownRightVector;": 8641, "LeftRightVector;": 10574, "LeftTriangleBar;": 10703, "LeftUpTeeVector;": 10592, "LeftUpVectorBar;": 10584, "LowerRightArrow;": 8600, "NotGreaterEqual;": 8817, "NotGreaterTilde;": 8821, "NotHumpDownHump;": 8782, "NotLeftTriangle;": 8938, "NotSquareSubset;": 8847, "OverParenthesis;": 9180, "RightDownVector;": 8642, "ShortRightArrow;": 8594, "UpperRightArrow;": 8599, "bigtriangledown;": 9661, "circlearrowleft;": 8634, "curvearrowright;": 8631, "downharpoonleft;": 8643, "leftharpoondown;": 8637, "leftrightarrows;": 8646, "nLeftrightarrow;": 8654, "nleftrightarrow;": 8622, "ntrianglelefteq;": 8940, "rightleftarrows;": 8644, "rightsquigarrow;": 8605, "rightthreetimes;": 8908, "straightepsilon;": 1013, "trianglerighteq;": 8885, "vartriangleleft;": 8882, "DiacriticalDot;": 729, "DoubleRightTee;": 8872, "DownLeftVector;": 8637, "GreaterGreater;": 10914, "HorizontalLine;": 9472, "InvisibleComma;": 8291, "InvisibleTimes;": 8290, "LeftDownVector;": 8643, "LeftRightArrow;": 8596, "Leftrightarrow;": 8660, "LessSlantEqual;": 10877, "LongRightArrow;": 10230, "Longrightarrow;": 10233, "LowerLeftArrow;": 8601, "NestedLessLess;": 8810, "NotGreaterLess;": 8825, "NotLessGreater;": 8824, "NotSubsetEqual;": 8840, "NotVerticalBar;": 8740, "OpenCurlyQuote;": 8216, "ReverseElement;": 8715, "RightTeeVector;": 10587, "RightVectorBar;": 10579, "ShortDownArrow;": 8595, "ShortLeftArrow;": 8592, "SquareSuperset;": 8848, "TildeFullEqual;": 8773, "UpperLeftArrow;": 8598, "ZeroWidthSpace;": 8203, "curvearrowleft;": 8630, "doublebarwedge;": 8966, "downdownarrows;": 8650, "hookrightarrow;": 8618, "leftleftarrows;": 8647, "leftrightarrow;": 8596, "leftthreetimes;": 8907, "longrightarrow;": 10230, "looparrowright;": 8620, "nshortparallel;": 8742, "ntriangleright;": 8939, "rightarrowtail;": 8611, "rightharpoonup;": 8640, "trianglelefteq;": 8884, "upharpoonright;": 8638, "ApplyFunction;": 8289, "DifferentialD;": 8518, "DoubleLeftTee;": 10980, "DoubleUpArrow;": 8657, "LeftTeeVector;": 10586, "LeftVectorBar;": 10578, "LessFullEqual;": 8806, "LongLeftArrow;": 10229, "Longleftarrow;": 10232, "NotEqualTilde;": 8770, "NotTildeEqual;": 8772, "NotTildeTilde;": 8777, "Poincareplane;": 8460, "PrecedesEqual;": 10927, "PrecedesTilde;": 8830, "RightArrowBar;": 8677, "RightTeeArrow;": 8614, "RightTriangle;": 8883, "RightUpVector;": 8638, "SucceedsEqual;": 10928, "SucceedsTilde;": 8831, "SupersetEqual;": 8839, "UpEquilibrium;": 10606, "VerticalTilde;": 8768, "VeryThinSpace;": 8202, "bigtriangleup;": 9651, "blacktriangle;": 9652, "divideontimes;": 8903, "fallingdotseq;": 8786, "hookleftarrow;": 8617, "leftarrowtail;": 8610, "leftharpoonup;": 8636, "longleftarrow;": 10229, "looparrowleft;": 8619, "measuredangle;": 8737, "ntriangleleft;": 8938, "shortparallel;": 8741, "smallsetminus;": 8726, "triangleright;": 9657, "upharpoonleft;": 8639, "varsubsetneqq;": 10955, "varsupsetneqq;": 10956, "DownArrowBar;": 10515, "DownTeeArrow;": 8615, "ExponentialE;": 8519, "GreaterEqual;": 8805, "GreaterTilde;": 8819, "HilbertSpace;": 8459, "HumpDownHump;": 8782, "Intersection;": 8898, "LeftArrowBar;": 8676, "LeftTeeArrow;": 8612, "LeftTriangle;": 8882, "LeftUpVector;": 8639, "NotCongruent;": 8802, "NotHumpEqual;": 8783, "NotLessEqual;": 8816, "NotLessTilde;": 8820, "Proportional;": 8733, "RightCeiling;": 8969, "RoundImplies;": 10608, "ShortUpArrow;": 8593, "SquareSubset;": 8847, "UnderBracket;": 9141, "VerticalLine;": 124, "blacklozenge;": 10731, "exponentiale;": 8519, "risingdotseq;": 8787, "triangledown;": 9663, "triangleleft;": 9667, "varsubsetneq;": 8842, "varsupsetneq;": 8843, "CircleMinus;": 8854, "CircleTimes;": 8855, "Equilibrium;": 8652, "GreaterLess;": 8823, "LeftCeiling;": 8968, "LessGreater;": 8822, "MediumSpace;": 8287, "NotLessLess;": 8810, "NotPrecedes;": 8832, "NotSucceeds;": 8833, "NotSuperset;": 8835, "OverBracket;": 9140, "RightVector;": 8640, "Rrightarrow;": 8667, "RuleDelayed;": 10740, "SmallCircle;": 8728, "SquareUnion;": 8852, "SubsetEqual;": 8838, "UpDownArrow;": 8597, "Updownarrow;": 8661, "VerticalBar;": 8739, "backepsilon;": 1014, "blacksquare;": 9642, "circledcirc;": 8858, "circleddash;": 8861, "curlyeqprec;": 8926, "curlyeqsucc;": 8927, "diamondsuit;": 9830, "eqslantless;": 10901, "expectation;": 8496, "nRightarrow;": 8655, "nrightarrow;": 8603, "preccurlyeq;": 8828, "precnapprox;": 10937, "quaternions;": 8461, "straightphi;": 981, "succcurlyeq;": 8829, "succnapprox;": 10938, "thickapprox;": 8776, "updownarrow;": 8597, "Bernoullis;": 8492, "CirclePlus;": 8853, "EqualTilde;": 8770, "Fouriertrf;": 8497, "ImaginaryI;": 8520, "Laplacetrf;": 8466, "LeftVector;": 8636, "Lleftarrow;": 8666, "NotElement;": 8713, "NotGreater;": 8815, "Proportion;": 8759, "RightArrow;": 8594, "RightFloor;": 8971, "Rightarrow;": 8658, "ThickSpace;": 8287, "TildeEqual;": 8771, "TildeTilde;": 8776, "UnderBrace;": 9183, "UpArrowBar;": 10514, "UpTeeArrow;": 8613, "circledast;": 8859, "complement;": 8705, "curlywedge;": 8911, "eqslantgtr;": 10902, "gtreqqless;": 10892, "lessapprox;": 10885, "lesseqqgtr;": 10891, "lmoustache;": 9136, "longmapsto;": 10236, "mapstodown;": 8615, "mapstoleft;": 8612, "nLeftarrow;": 8653, "nleftarrow;": 8602, "nsubseteqq;": 10949, "nsupseteqq;": 10950, "precapprox;": 10935, "rightarrow;": 8594, "rmoustache;": 9137, "sqsubseteq;": 8849, "sqsupseteq;": 8850, "subsetneqq;": 10955, "succapprox;": 10936, "supsetneqq;": 10956, "upuparrows;": 8648, "varepsilon;": 1013, "varnothing;": 8709, "Backslash;": 8726, "CenterDot;": 183, "CircleDot;": 8857, "Congruent;": 8801, "Coproduct;": 8720, "DoubleDot;": 168, "DownArrow;": 8595, "DownBreve;": 785, "Downarrow;": 8659, "HumpEqual;": 8783, "LeftArrow;": 8592, "LeftFloor;": 8970, "Leftarrow;": 8656, "LessTilde;": 8818, "Mellintrf;": 8499, "MinusPlus;": 8723, "NotCupCap;": 8813, "NotExists;": 8708, "NotSubset;": 8834, "OverBrace;": 9182, "PlusMinus;": 177, "Therefore;": 8756, "ThinSpace;": 8201, "TripleDot;": 8411, "UnionPlus;": 8846, "backprime;": 8245, "backsimeq;": 8909, "bigotimes;": 10754, "centerdot;": 183, "checkmark;": 10003, "complexes;": 8450, "dotsquare;": 8865, "downarrow;": 8595, "gtrapprox;": 10886, "gtreqless;": 8923, "gvertneqq;": 8809, "heartsuit;": 9829, "leftarrow;": 8592, "lesseqgtr;": 8922, "lvertneqq;": 8808, "ngeqslant;": 10878, "nleqslant;": 10877, "nparallel;": 8742, "nshortmid;": 8740, "nsubseteq;": 8840, "nsupseteq;": 8841, "pitchfork;": 8916, "rationals;": 8474, "spadesuit;": 9824, "subseteqq;": 10949, "subsetneq;": 8842, "supseteqq;": 10950, "supsetneq;": 8843, "therefore;": 8756, "triangleq;": 8796, "varpropto;": 8733, "DDotrahd;": 10513, "DotEqual;": 8784, "Integral;": 8747, "LessLess;": 10913, "NotEqual;": 8800, "NotTilde;": 8769, "PartialD;": 8706, "Precedes;": 8826, "RightTee;": 8866, "Succeeds;": 8827, "SuchThat;": 8715, "Superset;": 8835, "Uarrocir;": 10569, "UnderBar;": 95, "andslope;": 10840, "angmsdaa;": 10664, "angmsdab;": 10665, "angmsdac;": 10666, "angmsdad;": 10667, "angmsdae;": 10668, "angmsdaf;": 10669, "angmsdag;": 10670, "angmsdah;": 10671, "angrtvbd;": 10653, "approxeq;": 8778, "awconint;": 8755, "backcong;": 8780, "barwedge;": 8965, "bbrktbrk;": 9142, "bigoplus;": 10753, "bigsqcup;": 10758, "biguplus;": 10756, "bigwedge;": 8896, "boxminus;": 8863, "boxtimes;": 8864, "bsolhsub;": 10184, "capbrcup;": 10825, "circledR;": 174, "circledS;": 9416, "cirfnint;": 10768, "clubsuit;": 9827, "cupbrcap;": 10824, "curlyvee;": 8910, "cwconint;": 8754, "doteqdot;": 8785, "dotminus;": 8760, "drbkarow;": 10512, "dzigrarr;": 10239, "elinters;": 9191, "emptyset;": 8709, "eqvparsl;": 10725, "fpartint;": 10765, "geqslant;": 10878, "gesdotol;": 10884, "gnapprox;": 10890, "hksearow;": 10533, "hkswarow;": 10534, "imagline;": 8464, "imagpart;": 8465, "infintie;": 10717, "integers;": 8484, "intercal;": 8890, "intlarhk;": 10775, "laemptyv;": 10676, "ldrushar;": 10571, "leqslant;": 10877, "lesdotor;": 10883, "llcorner;": 8990, "lnapprox;": 10889, "lrcorner;": 8991, "lurdshar;": 10570, "mapstoup;": 8613, "multimap;": 8888, "naturals;": 8469, "ncongdot;": 10861, "notindot;": 8949, "otimesas;": 10806, "parallel;": 8741, "plusacir;": 10787, "pointint;": 10773, "precneqq;": 10933, "precnsim;": 8936, "profalar;": 9006, "profline;": 8978, "profsurf;": 8979, "raemptyv;": 10675, "realpart;": 8476, "rppolint;": 10770, "rtriltri;": 10702, "scpolint;": 10771, "setminus;": 8726, "shortmid;": 8739, "smeparsl;": 10724, "sqsubset;": 8847, "sqsupset;": 8848, "subseteq;": 8838, "succneqq;": 10934, "succnsim;": 8937, "supseteq;": 8839, "thetasym;": 977, "thicksim;": 8764, "timesbar;": 10801, "triangle;": 9653, "triminus;": 10810, "trpezium;": 9186, "ulcorner;": 8988, "urcorner;": 8989, "varkappa;": 1008, "varsigma;": 962, "vartheta;": 977, "Because;": 8757, "Cayleys;": 8493, "Cconint;": 8752, "Cedilla;": 184, "Diamond;": 8900, "DownTee;": 8868, "Element;": 8712, "Epsilon;": 917, "Implies;": 8658, "LeftTee;": 8867, "NewLine;": 10, "NoBreak;": 8288, "NotLess;": 8814, "Omicron;": 927, "OverBar;": 8254, "Product;": 8719, "UpArrow;": 8593, "Uparrow;": 8657, "Upsilon;": 933, "alefsym;": 8501, "angrtvb;": 8894, "angzarr;": 9084, "asympeq;": 8781, "backsim;": 8765, "because;": 8757, "bemptyv;": 10672, "between;": 8812, "bigcirc;": 9711, "bigodot;": 10752, "bigstar;": 9733, "bnequiv;": 8801, "boxplus;": 8862, "ccupssm;": 10832, "cemptyv;": 10674, "cirscir;": 10690, "coloneq;": 8788, "congdot;": 10861, "cudarrl;": 10552, "cudarrr;": 10549, "cularrp;": 10557, "curarrm;": 10556, "dbkarow;": 10511, "ddagger;": 8225, "ddotseq;": 10871, "demptyv;": 10673, "diamond;": 8900, "digamma;": 989, "dotplus;": 8724, "dwangle;": 10662, "epsilon;": 949, "eqcolon;": 8789, "equivDD;": 10872, "gesdoto;": 10882, "gtquest;": 10876, "gtrless;": 8823, "harrcir;": 10568, "intprod;": 10812, "isindot;": 8949, "larrbfs;": 10527, "larrsim;": 10611, "lbrksld;": 10639, "lbrkslu;": 10637, "ldrdhar;": 10599, "lesdoto;": 10881, "lessdot;": 8918, "lessgtr;": 8822, "lesssim;": 8818, "lotimes;": 10804, "lozenge;": 9674, "ltquest;": 10875, "luruhar;": 10598, "maltese;": 10016, "minusdu;": 10794, "napprox;": 8777, "natural;": 9838, "nearrow;": 8599, "nexists;": 8708, "notinva;": 8713, "notinvb;": 8951, "notinvc;": 8950, "notniva;": 8716, "notnivb;": 8958, "notnivc;": 8957, "npolint;": 10772, "npreceq;": 10927, "nsqsube;": 8930, "nsqsupe;": 8931, "nsubset;": 8834, "nsucceq;": 10928, "nsupset;": 8835, "nvinfin;": 10718, "nvltrie;": 8884, "nvrtrie;": 8885, "nwarrow;": 8598, "olcross;": 10683, "omicron;": 959, "orderof;": 8500, "orslope;": 10839, "pertenk;": 8241, "planckh;": 8462, "pluscir;": 10786, "plussim;": 10790, "plustwo;": 10791, "precsim;": 8830, "quatint;": 10774, "questeq;": 8799, "rarrbfs;": 10528, "rarrsim;": 10612, "rbrksld;": 10638, "rbrkslu;": 10640, "rdldhar;": 10601, "realine;": 8475, "rotimes;": 10805, "ruluhar;": 10600, "searrow;": 8600, "simplus;": 10788, "simrarr;": 10610, "subedot;": 10947, "submult;": 10945, "subplus;": 10943, "subrarr;": 10617, "succsim;": 8831, "supdsub;": 10968, "supedot;": 10948, "suphsol;": 10185, "suphsub;": 10967, "suplarr;": 10619, "supmult;": 10946, "supplus;": 10944, "swarrow;": 8601, "topfork;": 10970, "triplus;": 10809, "tritime;": 10811, "uparrow;": 8593, "upsilon;": 965, "uwangle;": 10663, "vzigzag;": 10650, "zigrarr;": 8669, "Aacute;": 193, "Abreve;": 258, "Agrave;": 192, "Assign;": 8788, "Atilde;": 195, "Barwed;": 8966, "Bumpeq;": 8782, "Cacute;": 262, "Ccaron;": 268, "Ccedil;": 199, "Colone;": 10868, "Conint;": 8751, "CupCap;": 8781, "Dagger;": 8225, "Dcaron;": 270, "DotDot;": 8412, "Dstrok;": 272, "Eacute;": 201, "Ecaron;": 282, "Egrave;": 200, "Exists;": 8707, "ForAll;": 8704, "Gammad;": 988, "Gbreve;": 286, "Gcedil;": 290, "HARDcy;": 1066, "Hstrok;": 294, "Iacute;": 205, "Igrave;": 204, "Itilde;": 296, "Jsercy;": 1032, "Kcedil;": 310, "Lacute;": 313, "Lambda;": 923, "Lcaron;": 317, "Lcedil;": 315, "Lmidot;": 319, "Lstrok;": 321, "Nacute;": 323, "Ncaron;": 327, "Ncedil;": 325, "Ntilde;": 209, "Oacute;": 211, "Odblac;": 336, "Ograve;": 210, "Oslash;": 216, "Otilde;": 213, "Otimes;": 10807, "Racute;": 340, "Rarrtl;": 10518, "Rcaron;": 344, "Rcedil;": 342, "SHCHcy;": 1065, "SOFTcy;": 1068, "Sacute;": 346, "Scaron;": 352, "Scedil;": 350, "Square;": 9633, "Subset;": 8912, "Supset;": 8913, "Tcaron;": 356, "Tcedil;": 354, "Tstrok;": 358, "Uacute;": 218, "Ubreve;": 364, "Udblac;": 368, "Ugrave;": 217, "Utilde;": 360, "Vdashl;": 10982, "Verbar;": 8214, "Vvdash;": 8874, "Yacute;": 221, "Zacute;": 377, "Zcaron;": 381, "aacute;": 225, "abreve;": 259, "agrave;": 224, "andand;": 10837, "angmsd;": 8737, "angsph;": 8738, "apacir;": 10863, "approx;": 8776, "atilde;": 227, "barvee;": 8893, "barwed;": 8965, "becaus;": 8757, "bernou;": 8492, "bigcap;": 8898, "bigcup;": 8899, "bigvee;": 8897, "bkarow;": 10509, "bottom;": 8869, "bowtie;": 8904, "boxbox;": 10697, "bprime;": 8245, "brvbar;": 166, "bullet;": 8226, "bumpeq;": 8783, "cacute;": 263, "capand;": 10820, "capcap;": 10827, "capcup;": 10823, "capdot;": 10816, "ccaron;": 269, "ccedil;": 231, "circeq;": 8791, "cirmid;": 10991, "colone;": 8788, "commat;": 64, "compfn;": 8728, "conint;": 8750, "coprod;": 8720, "copysr;": 8471, "cularr;": 8630, "cupcap;": 10822, "cupcup;": 10826, "cupdot;": 8845, "curarr;": 8631, "curren;": 164, "cylcty;": 9005, "dagger;": 8224, "daleth;": 8504, "dcaron;": 271, "dfisht;": 10623, "divide;": 247, "divonx;": 8903, "dlcorn;": 8990, "dlcrop;": 8973, "dollar;": 36, "drcorn;": 8991, "drcrop;": 8972, "dstrok;": 273, "eacute;": 233, "easter;": 10862, "ecaron;": 283, "ecolon;": 8789, "egrave;": 232, "egsdot;": 10904, "elsdot;": 10903, "emptyv;": 8709, "emsp13;": 8196, "emsp14;": 8197, "eparsl;": 10723, "eqcirc;": 8790, "equals;": 61, "equest;": 8799, "female;": 9792, "ffilig;": 64259, "ffllig;": 64260, "forall;": 8704, "frac12;": 189, "frac13;": 8531, "frac14;": 188, "frac15;": 8533, "frac16;": 8537, "frac18;": 8539, "frac23;": 8532, "frac25;": 8534, "frac34;": 190, "frac35;": 8535, "frac38;": 8540, "frac45;": 8536, "frac56;": 8538, "frac58;": 8541, "frac78;": 8542, "gacute;": 501, "gammad;": 989, "gbreve;": 287, "gesdot;": 10880, "gesles;": 10900, "gtlPar;": 10645, "gtrarr;": 10616, "gtrdot;": 8919, "gtrsim;": 8819, "hairsp;": 8202, "hamilt;": 8459, "hardcy;": 1098, "hearts;": 9829, "hellip;": 8230, "hercon;": 8889, "homtht;": 8763, "horbar;": 8213, "hslash;": 8463, "hstrok;": 295, "hybull;": 8259, "hyphen;": 8208, "iacute;": 237, "igrave;": 236, "iiiint;": 10764, "iinfin;": 10716, "incare;": 8453, "inodot;": 305, "intcal;": 8890, "iquest;": 191, "isinsv;": 8947, "itilde;": 297, "jsercy;": 1112, "kappav;": 1008, "kcedil;": 311, "kgreen;": 312, "lAtail;": 10523, "lacute;": 314, "lagran;": 8466, "lambda;": 955, "langle;": 10216, "larrfs;": 10525, "larrhk;": 8617, "larrlp;": 8619, "larrpl;": 10553, "larrtl;": 8610, "latail;": 10521, "lbrace;": 123, "lbrack;": 91, "lcaron;": 318, "lcedil;": 316, "ldquor;": 8222, "lesdot;": 10879, "lesges;": 10899, "lfisht;": 10620, "lfloor;": 8970, "lharul;": 10602, "llhard;": 10603, "lmidot;": 320, "lmoust;": 9136, "loplus;": 10797, "lowast;": 8727, "lowbar;": 95, "lparlt;": 10643, "lrhard;": 10605, "lsaquo;": 8249, "lsquor;": 8218, "lstrok;": 322, "lthree;": 8907, "ltimes;": 8905, "ltlarr;": 10614, "ltrPar;": 10646, "mapsto;": 8614, "marker;": 9646, "mcomma;": 10793, "midast;": 42, "midcir;": 10992, "middot;": 183, "minusb;": 8863, "minusd;": 8760, "mnplus;": 8723, "models;": 8871, "mstpos;": 8766, "nVDash;": 8879, "nVdash;": 8878, "nacute;": 324, "nbumpe;": 8783, "ncaron;": 328, "ncedil;": 326, "nearhk;": 10532, "nequiv;": 8802, "nesear;": 10536, "nexist;": 8708, "nltrie;": 8940, "notinE;": 8953, "nparsl;": 11005, "nprcue;": 8928, "nrarrc;": 10547, "nrarrw;": 8605, "nrtrie;": 8941, "nsccue;": 8929, "nsimeq;": 8772, "ntilde;": 241, "numero;": 8470, "nvDash;": 8877, "nvHarr;": 10500, "nvdash;": 8876, "nvlArr;": 10498, "nvrArr;": 10499, "nwarhk;": 10531, "nwnear;": 10535, "oacute;": 243, "odblac;": 337, "odsold;": 10684, "ograve;": 242, "ominus;": 8854, "origof;": 8886, "oslash;": 248, "otilde;": 245, "otimes;": 8855, "parsim;": 10995, "percnt;": 37, "period;": 46, "permil;": 8240, "phmmat;": 8499, "planck;": 8463, "plankv;": 8463, "plusdo;": 8724, "plusdu;": 10789, "plusmn;": 177, "preceq;": 10927, "primes;": 8473, "prnsim;": 8936, "propto;": 8733, "prurel;": 8880, "puncsp;": 8200, "qprime;": 8279, "rAtail;": 10524, "racute;": 341, "rangle;": 10217, "rarrap;": 10613, "rarrfs;": 10526, "rarrhk;": 8618, "rarrlp;": 8620, "rarrpl;": 10565, "rarrtl;": 8611, "ratail;": 10522, "rbrace;": 125, "rbrack;": 93, "rcaron;": 345, "rcedil;": 343, "rdquor;": 8221, "rfisht;": 10621, "rfloor;": 8971, "rharul;": 10604, "rmoust;": 9137, "roplus;": 10798, "rpargt;": 10644, "rsaquo;": 8250, "rsquor;": 8217, "rthree;": 8908, "rtimes;": 8906, "sacute;": 347, "scaron;": 353, "scedil;": 351, "scnsim;": 8937, "searhk;": 10533, "seswar;": 10537, "sfrown;": 8994, "shchcy;": 1097, "sigmaf;": 962, "sigmav;": 962, "simdot;": 10858, "smashp;": 10803, "softcy;": 1100, "solbar;": 9023, "spades;": 9824, "sqcaps;": 8851, "sqcups;": 8852, "sqsube;": 8849, "sqsupe;": 8850, "square;": 9633, "squarf;": 9642, "ssetmn;": 8726, "ssmile;": 8995, "sstarf;": 8902, "subdot;": 10941, "subset;": 8834, "subsim;": 10951, "subsub;": 10965, "subsup;": 10963, "succeq;": 10928, "supdot;": 10942, "supset;": 8835, "supsim;": 10952, "supsub;": 10964, "supsup;": 10966, "swarhk;": 10534, "swnwar;": 10538, "target;": 8982, "tcaron;": 357, "tcedil;": 355, "telrec;": 8981, "there4;": 8756, "thetav;": 977, "thinsp;": 8201, "thksim;": 8764, "timesb;": 8864, "timesd;": 10800, "topbot;": 9014, "topcir;": 10993, "tprime;": 8244, "tridot;": 9708, "tstrok;": 359, "uacute;": 250, "ubreve;": 365, "udblac;": 369, "ufisht;": 10622, "ugrave;": 249, "ulcorn;": 8988, "ulcrop;": 8975, "urcorn;": 8989, "urcrop;": 8974, "utilde;": 361, "vangrt;": 10652, "varphi;": 981, "varrho;": 1009, "veebar;": 8891, "vellip;": 8942, "verbar;": 124, "vsubnE;": 10955, "vsubne;": 8842, "vsupnE;": 10956, "vsupne;": 8843, "wedbar;": 10847, "wedgeq;": 8793, "weierp;": 8472, "wreath;": 8768, "xoplus;": 10753, "xotime;": 10754, "xsqcup;": 10758, "xuplus;": 10756, "xwedge;": 8896, "yacute;": 253, "zacute;": 378, "zcaron;": 382, "zeetrf;": 8488, "AElig;": 198, Aacute: 193, "Acirc;": 194, Agrave: 192, "Alpha;": 913, "Amacr;": 256, "Aogon;": 260, "Aring;": 197, Atilde: 195, "Breve;": 728, Ccedil: 199, "Ccirc;": 264, "Colon;": 8759, "Cross;": 10799, "Dashv;": 10980, "Delta;": 916, Eacute: 201, "Ecirc;": 202, Egrave: 200, "Emacr;": 274, "Eogon;": 280, "Equal;": 10869, "Gamma;": 915, "Gcirc;": 284, "Hacek;": 711, "Hcirc;": 292, "IJlig;": 306, Iacute: 205, "Icirc;": 206, Igrave: 204, "Imacr;": 298, "Iogon;": 302, "Iukcy;": 1030, "Jcirc;": 308, "Jukcy;": 1028, "Kappa;": 922, Ntilde: 209, "OElig;": 338, Oacute: 211, "Ocirc;": 212, Ograve: 210, "Omacr;": 332, "Omega;": 937, Oslash: 216, Otilde: 213, "Prime;": 8243, "RBarr;": 10512, "Scirc;": 348, "Sigma;": 931, "THORN;": 222, "TRADE;": 8482, "TSHcy;": 1035, "Theta;": 920, "Tilde;": 8764, Uacute: 218, "Ubrcy;": 1038, "Ucirc;": 219, Ugrave: 217, "Umacr;": 362, "Union;": 8899, "Uogon;": 370, "UpTee;": 8869, "Uring;": 366, "VDash;": 8875, "Vdash;": 8873, "Wcirc;": 372, "Wedge;": 8896, Yacute: 221, "Ycirc;": 374, aacute: 225, "acirc;": 226, "acute;": 180, "aelig;": 230, agrave: 224, "aleph;": 8501, "alpha;": 945, "amacr;": 257, "amalg;": 10815, "angle;": 8736, "angrt;": 8735, "angst;": 197, "aogon;": 261, "aring;": 229, "asymp;": 8776, atilde: 227, "awint;": 10769, "bcong;": 8780, "bdquo;": 8222, "bepsi;": 1014, "blank;": 9251, "blk12;": 9618, "blk14;": 9617, "blk34;": 9619, "block;": 9608, "boxDL;": 9559, "boxDR;": 9556, "boxDl;": 9558, "boxDr;": 9555, "boxHD;": 9574, "boxHU;": 9577, "boxHd;": 9572, "boxHu;": 9575, "boxUL;": 9565, "boxUR;": 9562, "boxUl;": 9564, "boxUr;": 9561, "boxVH;": 9580, "boxVL;": 9571, "boxVR;": 9568, "boxVh;": 9579, "boxVl;": 9570, "boxVr;": 9567, "boxdL;": 9557, "boxdR;": 9554, "boxdl;": 9488, "boxdr;": 9484, "boxhD;": 9573, "boxhU;": 9576, "boxhd;": 9516, "boxhu;": 9524, "boxuL;": 9563, "boxuR;": 9560, "boxul;": 9496, "boxur;": 9492, "boxvH;": 9578, "boxvL;": 9569, "boxvR;": 9566, "boxvh;": 9532, "boxvl;": 9508, "boxvr;": 9500, "breve;": 728, brvbar: 166, "bsemi;": 8271, "bsime;": 8909, "bsolb;": 10693, "bumpE;": 10926, "bumpe;": 8783, "caret;": 8257, "caron;": 711, "ccaps;": 10829, ccedil: 231, "ccirc;": 265, "ccups;": 10828, "cedil;": 184, "check;": 10003, "clubs;": 9827, "colon;": 58, "comma;": 44, "crarr;": 8629, "cross;": 10007, "csube;": 10961, "csupe;": 10962, "ctdot;": 8943, "cuepr;": 8926, "cuesc;": 8927, "cupor;": 10821, curren: 164, "cuvee;": 8910, "cuwed;": 8911, "cwint;": 8753, "dashv;": 8867, "dblac;": 733, "ddarr;": 8650, "delta;": 948, "dharl;": 8643, "dharr;": 8642, "diams;": 9830, "disin;": 8946, divide: 247, "doteq;": 8784, "dtdot;": 8945, "dtrif;": 9662, "duarr;": 8693, "duhar;": 10607, "eDDot;": 10871, eacute: 233, "ecirc;": 234, "efDot;": 8786, egrave: 232, "emacr;": 275, "empty;": 8709, "eogon;": 281, "eplus;": 10865, "epsiv;": 1013, "eqsim;": 8770, "equiv;": 8801, "erDot;": 8787, "erarr;": 10609, "esdot;": 8784, "exist;": 8707, "fflig;": 64256, "filig;": 64257, "fjlig;": 102, "fllig;": 64258, "fltns;": 9649, "forkv;": 10969, frac12: 189, frac14: 188, frac34: 190, "frasl;": 8260, "frown;": 8994, "gamma;": 947, "gcirc;": 285, "gescc;": 10921, "gimel;": 8503, "gneqq;": 8809, "gnsim;": 8935, "grave;": 96, "gsime;": 10894, "gsiml;": 10896, "gtcir;": 10874, "gtdot;": 8919, "harrw;": 8621, "hcirc;": 293, "hoarr;": 8703, iacute: 237, "icirc;": 238, "iexcl;": 161, igrave: 236, "iiint;": 8749, "iiota;": 8489, "ijlig;": 307, "imacr;": 299, "image;": 8465, "imath;": 305, "imped;": 437, "infin;": 8734, "iogon;": 303, "iprod;": 10812, iquest: 191, "isinE;": 8953, "isins;": 8948, "isinv;": 8712, "iukcy;": 1110, "jcirc;": 309, "jmath;": 567, "jukcy;": 1108, "kappa;": 954, "lAarr;": 8666, "lBarr;": 10510, "langd;": 10641, "laquo;": 171, "larrb;": 8676, "lates;": 10925, "lbarr;": 10508, "lbbrk;": 10098, "lbrke;": 10635, "lceil;": 8968, "ldquo;": 8220, "lescc;": 10920, "lhard;": 8637, "lharu;": 8636, "lhblk;": 9604, "llarr;": 8647, "lltri;": 9722, "lneqq;": 8808, "lnsim;": 8934, "loang;": 10220, "loarr;": 8701, "lobrk;": 10214, "lopar;": 10629, "lrarr;": 8646, "lrhar;": 8651, "lrtri;": 8895, "lsime;": 10893, "lsimg;": 10895, "lsquo;": 8216, "ltcir;": 10873, "ltdot;": 8918, "ltrie;": 8884, "ltrif;": 9666, "mDDot;": 8762, "mdash;": 8212, "micro;": 181, middot: 183, "minus;": 8722, "mumap;": 8888, "nabla;": 8711, "napid;": 8779, "napos;": 329, "natur;": 9838, "nbump;": 8782, "ncong;": 8775, "ndash;": 8211, "neArr;": 8663, "nearr;": 8599, "nedot;": 8784, "nesim;": 8770, "ngeqq;": 8807, "ngsim;": 8821, "nhArr;": 8654, "nharr;": 8622, "nhpar;": 10994, "nlArr;": 8653, "nlarr;": 8602, "nleqq;": 8806, "nless;": 8814, "nlsim;": 8820, "nltri;": 8938, "notin;": 8713, "notni;": 8716, "npart;": 8706, "nprec;": 8832, "nrArr;": 8655, "nrarr;": 8603, "nrtri;": 8939, "nsime;": 8772, "nsmid;": 8740, "nspar;": 8742, "nsubE;": 10949, "nsube;": 8840, "nsucc;": 8833, "nsupE;": 10950, "nsupe;": 8841, ntilde: 241, "numsp;": 8199, "nvsim;": 8764, "nwArr;": 8662, "nwarr;": 8598, oacute: 243, "ocirc;": 244, "odash;": 8861, "oelig;": 339, "ofcir;": 10687, ograve: 242, "ohbar;": 10677, "olarr;": 8634, "olcir;": 10686, "oline;": 8254, "omacr;": 333, "omega;": 969, "operp;": 10681, "oplus;": 8853, "orarr;": 8635, "order;": 8500, oslash: 248, otilde: 245, "ovbar;": 9021, "parsl;": 11005, "phone;": 9742, "plusb;": 8862, "pluse;": 10866, plusmn: 177, "pound;": 163, "prcue;": 8828, "prime;": 8242, "prnap;": 10937, "prsim;": 8830, "quest;": 63, "rAarr;": 8667, "rBarr;": 10511, "radic;": 8730, "rangd;": 10642, "range;": 10661, "raquo;": 187, "rarrb;": 8677, "rarrc;": 10547, "rarrw;": 8605, "ratio;": 8758, "rbarr;": 10509, "rbbrk;": 10099, "rbrke;": 10636, "rceil;": 8969, "rdquo;": 8221, "reals;": 8477, "rhard;": 8641, "rharu;": 8640, "rlarr;": 8644, "rlhar;": 8652, "rnmid;": 10990, "roang;": 10221, "roarr;": 8702, "robrk;": 10215, "ropar;": 10630, "rrarr;": 8649, "rsquo;": 8217, "rtrie;": 8885, "rtrif;": 9656, "sbquo;": 8218, "sccue;": 8829, "scirc;": 349, "scnap;": 10938, "scsim;": 8831, "sdotb;": 8865, "sdote;": 10854, "seArr;": 8664, "searr;": 8600, "setmn;": 8726, "sharp;": 9839, "sigma;": 963, "simeq;": 8771, "simgE;": 10912, "simlE;": 10911, "simne;": 8774, "slarr;": 8592, "smile;": 8995, "smtes;": 10924, "sqcap;": 8851, "sqcup;": 8852, "sqsub;": 8847, "sqsup;": 8848, "srarr;": 8594, "starf;": 9733, "strns;": 175, "subnE;": 10955, "subne;": 8842, "supnE;": 10956, "supne;": 8843, "swArr;": 8665, "swarr;": 8601, "szlig;": 223, "theta;": 952, "thkap;": 8776, "thorn;": 254, "tilde;": 732, "times;": 215, "trade;": 8482, "trisb;": 10701, "tshcy;": 1115, "twixt;": 8812, uacute: 250, "ubrcy;": 1118, "ucirc;": 251, "udarr;": 8645, "udhar;": 10606, ugrave: 249, "uharl;": 8639, "uharr;": 8638, "uhblk;": 9600, "ultri;": 9720, "umacr;": 363, "uogon;": 371, "uplus;": 8846, "upsih;": 978, "uring;": 367, "urtri;": 9721, "utdot;": 8944, "utrif;": 9652, "uuarr;": 8648, "vBarv;": 10985, "vDash;": 8872, "varpi;": 982, "vdash;": 8866, "veeeq;": 8794, "vltri;": 8882, "vnsub;": 8834, "vnsup;": 8835, "vprop;": 8733, "vrtri;": 8883, "wcirc;": 373, "wedge;": 8743, "xcirc;": 9711, "xdtri;": 9661, "xhArr;": 10234, "xharr;": 10231, "xlArr;": 10232, "xlarr;": 10229, "xodot;": 10752, "xrArr;": 10233, "xrarr;": 10230, "xutri;": 9651, yacute: 253, "ycirc;": 375, AElig: 198, Acirc: 194, "Aopf;": 120120, Aring: 197, "Ascr;": 119964, "Auml;": 196, "Barv;": 10983, "Beta;": 914, "Bopf;": 120121, "Bscr;": 8492, "CHcy;": 1063, "COPY;": 169, "Cdot;": 266, "Copf;": 8450, "Cscr;": 119966, "DJcy;": 1026, "DScy;": 1029, "DZcy;": 1039, "Darr;": 8609, "Dopf;": 120123, "Dscr;": 119967, Ecirc: 202, "Edot;": 278, "Eopf;": 120124, "Escr;": 8496, "Esim;": 10867, "Euml;": 203, "Fopf;": 120125, "Fscr;": 8497, "GJcy;": 1027, "Gdot;": 288, "Gopf;": 120126, "Gscr;": 119970, "Hopf;": 8461, "Hscr;": 8459, "IEcy;": 1045, "IOcy;": 1025, Icirc: 206, "Idot;": 304, "Iopf;": 120128, "Iota;": 921, "Iscr;": 8464, "Iuml;": 207, "Jopf;": 120129, "Jscr;": 119973, "KHcy;": 1061, "KJcy;": 1036, "Kopf;": 120130, "Kscr;": 119974, "LJcy;": 1033, "Lang;": 10218, "Larr;": 8606, "Lopf;": 120131, "Lscr;": 8466, "Mopf;": 120132, "Mscr;": 8499, "NJcy;": 1034, "Nopf;": 8469, "Nscr;": 119977, Ocirc: 212, "Oopf;": 120134, "Oscr;": 119978, "Ouml;": 214, "Popf;": 8473, "Pscr;": 119979, "QUOT;": 34, "Qopf;": 8474, "Qscr;": 119980, "Rang;": 10219, "Rarr;": 8608, "Ropf;": 8477, "Rscr;": 8475, "SHcy;": 1064, "Sopf;": 120138, "Sqrt;": 8730, "Sscr;": 119982, "Star;": 8902, THORN: 222, "TScy;": 1062, "Topf;": 120139, "Tscr;": 119983, "Uarr;": 8607, Ucirc: 219, "Uopf;": 120140, "Upsi;": 978, "Uscr;": 119984, "Uuml;": 220, "Vbar;": 10987, "Vert;": 8214, "Vopf;": 120141, "Vscr;": 119985, "Wopf;": 120142, "Wscr;": 119986, "Xopf;": 120143, "Xscr;": 119987, "YAcy;": 1071, "YIcy;": 1031, "YUcy;": 1070, "Yopf;": 120144, "Yscr;": 119988, "Yuml;": 376, "ZHcy;": 1046, "Zdot;": 379, "Zeta;": 918, "Zopf;": 8484, "Zscr;": 119989, acirc: 226, acute: 180, aelig: 230, "andd;": 10844, "andv;": 10842, "ange;": 10660, "aopf;": 120146, "apid;": 8779, "apos;": 39, aring: 229, "ascr;": 119990, "auml;": 228, "bNot;": 10989, "bbrk;": 9141, "beta;": 946, "beth;": 8502, "bnot;": 8976, "bopf;": 120147, "boxH;": 9552, "boxV;": 9553, "boxh;": 9472, "boxv;": 9474, "bscr;": 119991, "bsim;": 8765, "bsol;": 92, "bull;": 8226, "bump;": 8782, "caps;": 8745, "cdot;": 267, cedil: 184, "cent;": 162, "chcy;": 1095, "cirE;": 10691, "circ;": 710, "cire;": 8791, "comp;": 8705, "cong;": 8773, "copf;": 120148, "copy;": 169, "cscr;": 119992, "csub;": 10959, "csup;": 10960, "cups;": 8746, "dArr;": 8659, "dHar;": 10597, "darr;": 8595, "dash;": 8208, "diam;": 8900, "djcy;": 1106, "dopf;": 120149, "dscr;": 119993, "dscy;": 1109, "dsol;": 10742, "dtri;": 9663, "dzcy;": 1119, "eDot;": 8785, "ecir;": 8790, ecirc: 234, "edot;": 279, "emsp;": 8195, "ensp;": 8194, "eopf;": 120150, "epar;": 8917, "epsi;": 949, "escr;": 8495, "esim;": 8770, "euml;": 235, "euro;": 8364, "excl;": 33, "flat;": 9837, "fnof;": 402, "fopf;": 120151, "fork;": 8916, "fscr;": 119995, "gdot;": 289, "geqq;": 8807, "gesl;": 8923, "gjcy;": 1107, "gnap;": 10890, "gneq;": 10888, "gopf;": 120152, "gscr;": 8458, "gsim;": 8819, "gtcc;": 10919, "gvnE;": 8809, "hArr;": 8660, "half;": 189, "harr;": 8596, "hbar;": 8463, "hopf;": 120153, "hscr;": 119997, icirc: 238, "iecy;": 1077, iexcl: 161, "imof;": 8887, "iocy;": 1105, "iopf;": 120154, "iota;": 953, "iscr;": 119998, "isin;": 8712, "iuml;": 239, "jopf;": 120155, "jscr;": 119999, "khcy;": 1093, "kjcy;": 1116, "kopf;": 120156, "kscr;": 12e4, "lArr;": 8656, "lHar;": 10594, "lang;": 10216, laquo: 171, "larr;": 8592, "late;": 10925, "lcub;": 123, "ldca;": 10550, "ldsh;": 8626, "leqq;": 8806, "lesg;": 8922, "ljcy;": 1113, "lnap;": 10889, "lneq;": 10887, "lopf;": 120157, "lozf;": 10731, "lpar;": 40, "lscr;": 120001, "lsim;": 8818, "lsqb;": 91, "ltcc;": 10918, "ltri;": 9667, "lvnE;": 8808, "macr;": 175, "male;": 9794, "malt;": 10016, micro: 181, "mlcp;": 10971, "mldr;": 8230, "mopf;": 120158, "mscr;": 120002, "nGtv;": 8811, "nLtv;": 8810, "nang;": 8736, "napE;": 10864, "nbsp;": 160, "ncap;": 10819, "ncup;": 10818, "ngeq;": 8817, "nges;": 10878, "ngtr;": 8815, "nisd;": 8954, "njcy;": 1114, "nldr;": 8229, "nleq;": 8816, "nles;": 10877, "nmid;": 8740, "nopf;": 120159, "npar;": 8742, "npre;": 10927, "nsce;": 10928, "nscr;": 120003, "nsim;": 8769, "nsub;": 8836, "nsup;": 8837, "ntgl;": 8825, "ntlg;": 8824, "nvap;": 8781, "nvge;": 8805, "nvgt;": 62, "nvle;": 8804, "nvlt;": 60, "oast;": 8859, "ocir;": 8858, ocirc: 244, "odiv;": 10808, "odot;": 8857, "ogon;": 731, "oint;": 8750, "omid;": 10678, "oopf;": 120160, "opar;": 10679, "ordf;": 170, "ordm;": 186, "oror;": 10838, "oscr;": 8500, "osol;": 8856, "ouml;": 246, "para;": 182, "part;": 8706, "perp;": 8869, "phiv;": 981, "plus;": 43, "popf;": 120161, pound: 163, "prap;": 10935, "prec;": 8826, "prnE;": 10933, "prod;": 8719, "prop;": 8733, "pscr;": 120005, "qint;": 10764, "qopf;": 120162, "qscr;": 120006, "quot;": 34, "rArr;": 8658, "rHar;": 10596, "race;": 8765, "rang;": 10217, raquo: 187, "rarr;": 8594, "rcub;": 125, "rdca;": 10551, "rdsh;": 8627, "real;": 8476, "rect;": 9645, "rhov;": 1009, "ring;": 730, "ropf;": 120163, "rpar;": 41, "rscr;": 120007, "rsqb;": 93, "rtri;": 9657, "scap;": 10936, "scnE;": 10934, "sdot;": 8901, "sect;": 167, "semi;": 59, "sext;": 10038, "shcy;": 1096, "sime;": 8771, "simg;": 10910, "siml;": 10909, "smid;": 8739, "smte;": 10924, "solb;": 10692, "sopf;": 120164, "spar;": 8741, "squf;": 9642, "sscr;": 120008, "star;": 9734, "subE;": 10949, "sube;": 8838, "succ;": 8827, "sung;": 9834, "sup1;": 185, "sup2;": 178, "sup3;": 179, "supE;": 10950, "supe;": 8839, szlig: 223, "tbrk;": 9140, "tdot;": 8411, thorn: 254, times: 215, "tint;": 8749, "toea;": 10536, "topf;": 120165, "tosa;": 10537, "trie;": 8796, "tscr;": 120009, "tscy;": 1094, "uArr;": 8657, "uHar;": 10595, "uarr;": 8593, ucirc: 251, "uopf;": 120166, "upsi;": 965, "uscr;": 120010, "utri;": 9653, "uuml;": 252, "vArr;": 8661, "vBar;": 10984, "varr;": 8597, "vert;": 124, "vopf;": 120167, "vscr;": 120011, "wopf;": 120168, "wscr;": 120012, "xcap;": 8898, "xcup;": 8899, "xmap;": 10236, "xnis;": 8955, "xopf;": 120169, "xscr;": 120013, "xvee;": 8897, "yacy;": 1103, "yicy;": 1111, "yopf;": 120170, "yscr;": 120014, "yucy;": 1102, "yuml;": 255, "zdot;": 380, "zeta;": 950, "zhcy;": 1078, "zopf;": 120171, "zscr;": 120015, "zwnj;": 8204, "AMP;": 38, "Acy;": 1040, "Afr;": 120068, "And;": 10835, Auml: 196, "Bcy;": 1041, "Bfr;": 120069, COPY: 169, "Cap;": 8914, "Cfr;": 8493, "Chi;": 935, "Cup;": 8915, "Dcy;": 1044, "Del;": 8711, "Dfr;": 120071, "Dot;": 168, "ENG;": 330, "ETH;": 208, "Ecy;": 1069, "Efr;": 120072, "Eta;": 919, Euml: 203, "Fcy;": 1060, "Ffr;": 120073, "Gcy;": 1043, "Gfr;": 120074, "Hat;": 94, "Hfr;": 8460, "Icy;": 1048, "Ifr;": 8465, "Int;": 8748, Iuml: 207, "Jcy;": 1049, "Jfr;": 120077, "Kcy;": 1050, "Kfr;": 120078, "Lcy;": 1051, "Lfr;": 120079, "Lsh;": 8624, "Map;": 10501, "Mcy;": 1052, "Mfr;": 120080, "Ncy;": 1053, "Nfr;": 120081, "Not;": 10988, "Ocy;": 1054, "Ofr;": 120082, Ouml: 214, "Pcy;": 1055, "Pfr;": 120083, "Phi;": 934, "Psi;": 936, QUOT: 34, "Qfr;": 120084, "REG;": 174, "Rcy;": 1056, "Rfr;": 8476, "Rho;": 929, "Rsh;": 8625, "Scy;": 1057, "Sfr;": 120086, "Sub;": 8912, "Sum;": 8721, "Sup;": 8913, "Tab;": 9, "Tau;": 932, "Tcy;": 1058, "Tfr;": 120087, "Ucy;": 1059, "Ufr;": 120088, Uuml: 220, "Vcy;": 1042, "Vee;": 8897, "Vfr;": 120089, "Wfr;": 120090, "Xfr;": 120091, "Ycy;": 1067, "Yfr;": 120092, "Zcy;": 1047, "Zfr;": 8488, "acE;": 8766, "acd;": 8767, "acy;": 1072, "afr;": 120094, "amp;": 38, "and;": 8743, "ang;": 8736, "apE;": 10864, "ape;": 8778, "ast;": 42, auml: 228, "bcy;": 1073, "bfr;": 120095, "bne;": 61, "bot;": 8869, "cap;": 8745, cent: 162, "cfr;": 120096, "chi;": 967, "cir;": 9675, copy: 169, "cup;": 8746, "dcy;": 1076, "deg;": 176, "dfr;": 120097, "die;": 168, "div;": 247, "dot;": 729, "ecy;": 1101, "efr;": 120098, "egs;": 10902, "ell;": 8467, "els;": 10901, "eng;": 331, "eta;": 951, "eth;": 240, euml: 235, "fcy;": 1092, "ffr;": 120099, "gEl;": 10892, "gap;": 10886, "gcy;": 1075, "gel;": 8923, "geq;": 8805, "ges;": 10878, "gfr;": 120100, "ggg;": 8921, "glE;": 10898, "gla;": 10917, "glj;": 10916, "gnE;": 8809, "gne;": 10888, "hfr;": 120101, "icy;": 1080, "iff;": 8660, "ifr;": 120102, "int;": 8747, iuml: 239, "jcy;": 1081, "jfr;": 120103, "kcy;": 1082, "kfr;": 120104, "lEg;": 10891, "lap;": 10885, "lat;": 10923, "lcy;": 1083, "leg;": 8922, "leq;": 8804, "les;": 10877, "lfr;": 120105, "lgE;": 10897, "lnE;": 8808, "lne;": 10887, "loz;": 9674, "lrm;": 8206, "lsh;": 8624, macr: 175, "map;": 8614, "mcy;": 1084, "mfr;": 120106, "mho;": 8487, "mid;": 8739, "nGg;": 8921, "nGt;": 8811, "nLl;": 8920, "nLt;": 8810, "nap;": 8777, nbsp: 160, "ncy;": 1085, "nfr;": 120107, "ngE;": 8807, "nge;": 8817, "ngt;": 8815, "nis;": 8956, "niv;": 8715, "nlE;": 8806, "nle;": 8816, "nlt;": 8814, "not;": 172, "npr;": 8832, "nsc;": 8833, "num;": 35, "ocy;": 1086, "ofr;": 120108, "ogt;": 10689, "ohm;": 937, "olt;": 10688, "ord;": 10845, ordf: 170, ordm: 186, "orv;": 10843, ouml: 246, "par;": 8741, para: 182, "pcy;": 1087, "pfr;": 120109, "phi;": 966, "piv;": 982, "prE;": 10931, "pre;": 10927, "psi;": 968, "qfr;": 120110, quot: 34, "rcy;": 1088, "reg;": 174, "rfr;": 120111, "rho;": 961, "rlm;": 8207, "rsh;": 8625, "scE;": 10932, "sce;": 10928, "scy;": 1089, sect: 167, "sfr;": 120112, "shy;": 173, "sim;": 8764, "smt;": 10922, "sol;": 47, "squ;": 9633, "sub;": 8834, "sum;": 8721, sup1: 185, sup2: 178, sup3: 179, "sup;": 8835, "tau;": 964, "tcy;": 1090, "tfr;": 120113, "top;": 8868, "ucy;": 1091, "ufr;": 120114, "uml;": 168, uuml: 252, "vcy;": 1074, "vee;": 8744, "vfr;": 120115, "wfr;": 120116, "xfr;": 120117, "ycy;": 1099, "yen;": 165, "yfr;": 120118, yuml: 255, "zcy;": 1079, "zfr;": 120119, "zwj;": 8205, AMP: 38, "DD;": 8517, ETH: 208, "GT;": 62, "Gg;": 8921, "Gt;": 8811, "Im;": 8465, "LT;": 60, "Ll;": 8920, "Lt;": 8810, "Mu;": 924, "Nu;": 925, "Or;": 10836, "Pi;": 928, "Pr;": 10939, REG: 174, "Re;": 8476, "Sc;": 10940, "Xi;": 926, "ac;": 8766, "af;": 8289, amp: 38, "ap;": 8776, "dd;": 8518, deg: 176, "ee;": 8519, "eg;": 10906, "el;": 10905, eth: 240, "gE;": 8807, "ge;": 8805, "gg;": 8811, "gl;": 8823, "gt;": 62, "ic;": 8291, "ii;": 8520, "in;": 8712, "it;": 8290, "lE;": 8806, "le;": 8804, "lg;": 8822, "ll;": 8810, "lt;": 60, "mp;": 8723, "mu;": 956, "ne;": 8800, "ni;": 8715, not: 172, "nu;": 957, "oS;": 9416, "or;": 8744, "pi;": 960, "pm;": 177, "pr;": 8826, reg: 174, "rx;": 8478, "sc;": 8827, shy: 173, uml: 168, "wp;": 8472, "wr;": 8768, "xi;": 958, yen: 165, GT: 62, LT: 60, gt: 62, lt: 60 };
function oh(e, t) {
  return t && !e.endsWith(";") ? `${e}\\b(?!=)` : e;
}
function Jc(e) {
  const t = "#(?:x[a-fA-F\\d]+|\\d+)(?:;)?", r = Object.keys(nh).map((o) => oh(o, e));
  return new RegExp(`&(${t}|${r.join("|")})`, "g");
}
Jc(false);
Jc(true);
const lh = /* @__PURE__ */ new Map([["svelte:head", "SvelteHead"], ["svelte:options", "SvelteOptions"], ["svelte:window", "SvelteWindow"], ["svelte:document", "SvelteDocument"], ["svelte:body", "SvelteBody"]]);
new Map([...lh, ["svelte:element", "SvelteElement"], ["svelte:component", "SvelteComponent"], ["svelte:self", "SvelteSelf"], ["svelte:fragment", "SvelteFragment"], ["svelte:boundary", "SvelteBoundary"]]);
var ft = {}, Qr = {}, Gt = {}, Yr = {};
Object.defineProperty(Yr, "__esModule", { value: true });
Yr.default = void 0;
function uh() {
  var e = this, t = 0, r = { "@@iterator": function() {
    return r;
  }, next: function() {
    if (t < e.length) {
      var o = e[t];
      return t = t + 1, { done: false, value: o };
    } else return { done: true };
  } };
  return r;
}
Yr.default = uh;
Object.defineProperty(Gt, "__esModule", { value: true });
Gt.default = ph;
var ch = dh(Yr);
function dh(e) {
  return e && e.__esModule ? e : { default: e };
}
function Nl(e) {
  "@babel/helpers - typeof";
  return Nl = typeof Symbol == "function" && typeof Symbol.iterator == "symbol" ? function(t) {
    return typeof t;
  } : function(t) {
    return t && typeof Symbol == "function" && t.constructor === Symbol && t !== Symbol.prototype ? "symbol" : typeof t;
  }, Nl(e);
}
function ph(e, t) {
  return typeof Symbol == "function" && Nl(Symbol.iterator) === "symbol" && Object.defineProperty(e, Symbol.iterator, { value: ch.default.bind(t) }), e;
}
Object.defineProperty(Qr, "__esModule", { value: true });
Qr.default = void 0;
var hh = fh(Gt);
function fh(e) {
  return e && e.__esModule ? e : { default: e };
}
function vl(e, t) {
  return yh(e) || bh(e, t) || vh(e, t) || mh();
}
function mh() {
  throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
}
function vh(e, t) {
  if (e) {
    if (typeof e == "string") return Mu(e, t);
    var r = {}.toString.call(e).slice(8, -1);
    return r === "Object" && e.constructor && (r = e.constructor.name), r === "Map" || r === "Set" ? Array.from(e) : r === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(r) ? Mu(e, t) : void 0;
  }
}
function Mu(e, t) {
  (t == null || t > e.length) && (t = e.length);
  for (var r = 0, s = Array(t); r < t; r++) s[r] = e[r];
  return s;
}
function bh(e, t) {
  var r = e == null ? null : typeof Symbol < "u" && e[Symbol.iterator] || e["@@iterator"];
  if (r != null) {
    var s, o, p, h, v = [], m = true, l = false;
    try {
      if (p = (r = r.call(e)).next, t === 0) {
        if (Object(r) !== r) return;
        m = false;
      } else for (; !(m = (s = p.call(r)).done) && (v.push(s.value), v.length !== t); m = true) ;
    } catch (w) {
      l = true, o = w;
    } finally {
      try {
        if (!m && r.return != null && (h = r.return(), Object(h) !== h)) return;
      } finally {
        if (l) throw o;
      }
    }
    return v;
  }
}
function yh(e) {
  if (Array.isArray(e)) return e;
}
var Wt = [["aria-activedescendant", { type: "id" }], ["aria-atomic", { type: "boolean" }], ["aria-autocomplete", { type: "token", values: ["inline", "list", "both", "none"] }], ["aria-braillelabel", { type: "string" }], ["aria-brailleroledescription", { type: "string" }], ["aria-busy", { type: "boolean" }], ["aria-checked", { type: "tristate" }], ["aria-colcount", { type: "integer" }], ["aria-colindex", { type: "integer" }], ["aria-colspan", { type: "integer" }], ["aria-controls", { type: "idlist" }], ["aria-current", { type: "token", values: ["page", "step", "location", "date", "time", true, false] }], ["aria-describedby", { type: "idlist" }], ["aria-description", { type: "string" }], ["aria-details", { type: "id" }], ["aria-disabled", { type: "boolean" }], ["aria-dropeffect", { type: "tokenlist", values: ["copy", "execute", "link", "move", "none", "popup"] }], ["aria-errormessage", { type: "id" }], ["aria-expanded", { type: "boolean", allowundefined: true }], ["aria-flowto", { type: "idlist" }], ["aria-grabbed", { type: "boolean", allowundefined: true }], ["aria-haspopup", { type: "token", values: [false, true, "menu", "listbox", "tree", "grid", "dialog"] }], ["aria-hidden", { type: "boolean", allowundefined: true }], ["aria-invalid", { type: "token", values: ["grammar", false, "spelling", true] }], ["aria-keyshortcuts", { type: "string" }], ["aria-label", { type: "string" }], ["aria-labelledby", { type: "idlist" }], ["aria-level", { type: "integer" }], ["aria-live", { type: "token", values: ["assertive", "off", "polite"] }], ["aria-modal", { type: "boolean" }], ["aria-multiline", { type: "boolean" }], ["aria-multiselectable", { type: "boolean" }], ["aria-orientation", { type: "token", values: ["vertical", "undefined", "horizontal"] }], ["aria-owns", { type: "idlist" }], ["aria-placeholder", { type: "string" }], ["aria-posinset", { type: "integer" }], ["aria-pressed", { type: "tristate" }], ["aria-readonly", { type: "boolean" }], ["aria-relevant", { type: "tokenlist", values: ["additions", "all", "removals", "text"] }], ["aria-required", { type: "boolean" }], ["aria-roledescription", { type: "string" }], ["aria-rowcount", { type: "integer" }], ["aria-rowindex", { type: "integer" }], ["aria-rowspan", { type: "integer" }], ["aria-selected", { type: "boolean", allowundefined: true }], ["aria-setsize", { type: "integer" }], ["aria-sort", { type: "token", values: ["ascending", "descending", "none", "other"] }], ["aria-valuemax", { type: "number" }], ["aria-valuemin", { type: "number" }], ["aria-valuenow", { type: "number" }], ["aria-valuetext", { type: "string" }]], Dl = { entries: function() {
  return Wt;
}, forEach: function(t) {
  for (var r = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, s = 0, o = Wt; s < o.length; s++) {
    var p = vl(o[s], 2), h = p[0], v = p[1];
    t.call(r, v, h, Wt);
  }
}, get: function(t) {
  var r = Wt.filter(function(s) {
    return s[0] === t;
  })[0];
  return r && r[1];
}, has: function(t) {
  return !!Dl.get(t);
}, keys: function() {
  return Wt.map(function(t) {
    var r = vl(t, 1), s = r[0];
    return s;
  });
}, values: function() {
  return Wt.map(function(t) {
    var r = vl(t, 2), s = r[1];
    return s;
  });
} };
Qr.default = (0, hh.default)(Dl, Dl.entries());
var Zr = {};
Object.defineProperty(Zr, "__esModule", { value: true });
Zr.default = void 0;
var gh = xh(Gt);
function xh(e) {
  return e && e.__esModule ? e : { default: e };
}
function bl(e, t) {
  return Ph(e) || Rh(e, t) || Ch(e, t) || _h();
}
function _h() {
  throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
}
function Ch(e, t) {
  if (e) {
    if (typeof e == "string") return Lu(e, t);
    var r = {}.toString.call(e).slice(8, -1);
    return r === "Object" && e.constructor && (r = e.constructor.name), r === "Map" || r === "Set" ? Array.from(e) : r === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(r) ? Lu(e, t) : void 0;
  }
}
function Lu(e, t) {
  (t == null || t > e.length) && (t = e.length);
  for (var r = 0, s = Array(t); r < t; r++) s[r] = e[r];
  return s;
}
function Rh(e, t) {
  var r = e == null ? null : typeof Symbol < "u" && e[Symbol.iterator] || e["@@iterator"];
  if (r != null) {
    var s, o, p, h, v = [], m = true, l = false;
    try {
      if (p = (r = r.call(e)).next, t === 0) {
        if (Object(r) !== r) return;
        m = false;
      } else for (; !(m = (s = p.call(r)).done) && (v.push(s.value), v.length !== t); m = true) ;
    } catch (w) {
      l = true, o = w;
    } finally {
      try {
        if (!m && r.return != null && (h = r.return(), Object(h) !== h)) return;
      } finally {
        if (l) throw o;
      }
    }
    return v;
  }
}
function Ph(e) {
  if (Array.isArray(e)) return e;
}
var Kt = [["a", { reserved: false }], ["abbr", { reserved: false }], ["acronym", { reserved: false }], ["address", { reserved: false }], ["applet", { reserved: false }], ["area", { reserved: false }], ["article", { reserved: false }], ["aside", { reserved: false }], ["audio", { reserved: false }], ["b", { reserved: false }], ["base", { reserved: true }], ["bdi", { reserved: false }], ["bdo", { reserved: false }], ["big", { reserved: false }], ["blink", { reserved: false }], ["blockquote", { reserved: false }], ["body", { reserved: false }], ["br", { reserved: false }], ["button", { reserved: false }], ["canvas", { reserved: false }], ["caption", { reserved: false }], ["center", { reserved: false }], ["cite", { reserved: false }], ["code", { reserved: false }], ["col", { reserved: true }], ["colgroup", { reserved: true }], ["content", { reserved: false }], ["data", { reserved: false }], ["datalist", { reserved: false }], ["dd", { reserved: false }], ["del", { reserved: false }], ["details", { reserved: false }], ["dfn", { reserved: false }], ["dialog", { reserved: false }], ["dir", { reserved: false }], ["div", { reserved: false }], ["dl", { reserved: false }], ["dt", { reserved: false }], ["em", { reserved: false }], ["embed", { reserved: false }], ["fieldset", { reserved: false }], ["figcaption", { reserved: false }], ["figure", { reserved: false }], ["font", { reserved: false }], ["footer", { reserved: false }], ["form", { reserved: false }], ["frame", { reserved: false }], ["frameset", { reserved: false }], ["h1", { reserved: false }], ["h2", { reserved: false }], ["h3", { reserved: false }], ["h4", { reserved: false }], ["h5", { reserved: false }], ["h6", { reserved: false }], ["head", { reserved: true }], ["header", { reserved: false }], ["hgroup", { reserved: false }], ["hr", { reserved: false }], ["html", { reserved: true }], ["i", { reserved: false }], ["iframe", { reserved: false }], ["img", { reserved: false }], ["input", { reserved: false }], ["ins", { reserved: false }], ["kbd", { reserved: false }], ["keygen", { reserved: false }], ["label", { reserved: false }], ["legend", { reserved: false }], ["li", { reserved: false }], ["link", { reserved: true }], ["main", { reserved: false }], ["map", { reserved: false }], ["mark", { reserved: false }], ["marquee", { reserved: false }], ["menu", { reserved: false }], ["menuitem", { reserved: false }], ["meta", { reserved: true }], ["meter", { reserved: false }], ["nav", { reserved: false }], ["noembed", { reserved: true }], ["noscript", { reserved: true }], ["object", { reserved: false }], ["ol", { reserved: false }], ["optgroup", { reserved: false }], ["option", { reserved: false }], ["output", { reserved: false }], ["p", { reserved: false }], ["param", { reserved: true }], ["picture", { reserved: true }], ["pre", { reserved: false }], ["progress", { reserved: false }], ["q", { reserved: false }], ["rp", { reserved: false }], ["rt", { reserved: false }], ["rtc", { reserved: false }], ["ruby", { reserved: false }], ["s", { reserved: false }], ["samp", { reserved: false }], ["script", { reserved: true }], ["section", { reserved: false }], ["select", { reserved: false }], ["small", { reserved: false }], ["source", { reserved: true }], ["spacer", { reserved: false }], ["span", { reserved: false }], ["strike", { reserved: false }], ["strong", { reserved: false }], ["style", { reserved: true }], ["sub", { reserved: false }], ["summary", { reserved: false }], ["sup", { reserved: false }], ["table", { reserved: false }], ["tbody", { reserved: false }], ["td", { reserved: false }], ["textarea", { reserved: false }], ["tfoot", { reserved: false }], ["th", { reserved: false }], ["thead", { reserved: false }], ["time", { reserved: false }], ["title", { reserved: true }], ["tr", { reserved: false }], ["track", { reserved: true }], ["tt", { reserved: false }], ["u", { reserved: false }], ["ul", { reserved: false }], ["var", { reserved: false }], ["video", { reserved: false }], ["wbr", { reserved: false }], ["xmp", { reserved: false }]], jl = { entries: function() {
  return Kt;
}, forEach: function(t) {
  for (var r = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, s = 0, o = Kt; s < o.length; s++) {
    var p = bl(o[s], 2), h = p[0], v = p[1];
    t.call(r, v, h, Kt);
  }
}, get: function(t) {
  var r = Kt.filter(function(s) {
    return s[0] === t;
  })[0];
  return r && r[1];
}, has: function(t) {
  return !!jl.get(t);
}, keys: function() {
  return Kt.map(function(t) {
    var r = bl(t, 1), s = r[0];
    return s;
  });
}, values: function() {
  return Kt.map(function(t) {
    var r = bl(t, 2), s = r[1];
    return s;
  });
} };
Zr.default = (0, gh.default)(jl, jl.entries());
var or = {}, ea = {}, ta = {};
Object.defineProperty(ta, "__esModule", { value: true });
ta.default = void 0;
var wh = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget"]] };
ta.default = wh;
var ra = {};
Object.defineProperty(ra, "__esModule", { value: true });
ra.default = void 0;
var Th = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-activedescendant": null, "aria-disabled": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget"]] };
ra.default = Th;
var aa = {};
Object.defineProperty(aa, "__esModule", { value: true });
aa.default = void 0;
var Sh = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null }, relatedConcepts: [{ concept: { name: "input" }, module: "XForms" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget"]] };
aa.default = Sh;
var ia = {};
Object.defineProperty(ia, "__esModule", { value: true });
ia.default = void 0;
var Ah = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
ia.default = Ah;
var sa = {};
Object.defineProperty(sa, "__esModule", { value: true });
sa.default = void 0;
var Eh = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-valuemax": null, "aria-valuemin": null, "aria-valuenow": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
sa.default = Eh;
var na = {};
Object.defineProperty(na, "__esModule", { value: true });
na.default = void 0;
var kh = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: [], prohibitedProps: [], props: { "aria-atomic": null, "aria-busy": null, "aria-controls": null, "aria-current": null, "aria-describedby": null, "aria-details": null, "aria-dropeffect": null, "aria-flowto": null, "aria-grabbed": null, "aria-hidden": null, "aria-keyshortcuts": null, "aria-label": null, "aria-labelledby": null, "aria-live": null, "aria-owns": null, "aria-relevant": null, "aria-roledescription": null }, relatedConcepts: [{ concept: { name: "role" }, module: "XHTML" }, { concept: { name: "type" }, module: "Dublin Core" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [] };
na.default = kh;
var oa = {};
Object.defineProperty(oa, "__esModule", { value: true });
oa.default = void 0;
var Ih = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: [], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "frontmatter" }, module: "DTB" }, { concept: { name: "level" }, module: "DTB" }, { concept: { name: "level" }, module: "SMIL" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
oa.default = Ih;
var la = {};
Object.defineProperty(la, "__esModule", { value: true });
la.default = void 0;
var qh = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
la.default = qh;
var ua = {};
Object.defineProperty(ua, "__esModule", { value: true });
ua.default = void 0;
var Mh = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-orientation": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "composite"], ["roletype", "structure", "section", "group"]] };
ua.default = Mh;
var ca = {};
Object.defineProperty(ca, "__esModule", { value: true });
ca.default = void 0;
var Lh = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: [], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype"]] };
ca.default = Lh;
var da = {};
Object.defineProperty(da, "__esModule", { value: true });
da.default = void 0;
var Oh = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: [], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype"]] };
da.default = Oh;
var pa = {};
Object.defineProperty(pa, "__esModule", { value: true });
pa.default = void 0;
var Nh = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-modal": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype"]] };
pa.default = Nh;
Object.defineProperty(ea, "__esModule", { value: true });
ea.default = void 0;
var Dh = pt(ta), jh = pt(ra), $h = pt(aa), Fh = pt(ia), Bh = pt(sa), Vh = pt(na), Hh = pt(oa), Uh = pt(la), Gh = pt(ua), zh = pt(ca), Wh = pt(da), Kh = pt(pa);
function pt(e) {
  return e && e.__esModule ? e : { default: e };
}
var Xh = [["command", Dh.default], ["composite", jh.default], ["input", $h.default], ["landmark", Fh.default], ["range", Bh.default], ["roletype", Vh.default], ["section", Hh.default], ["sectionhead", Uh.default], ["select", Gh.default], ["structure", zh.default], ["widget", Wh.default], ["window", Kh.default]];
ea.default = Xh;
var ha = {}, fa = {};
Object.defineProperty(fa, "__esModule", { value: true });
fa.default = void 0;
var Jh = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-atomic": "true", "aria-live": "assertive" }, relatedConcepts: [{ concept: { name: "alert" }, module: "XForms" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
fa.default = Jh;
var ma = {};
Object.defineProperty(ma, "__esModule", { value: true });
ma.default = void 0;
var Qh = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "alert" }, module: "XForms" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "alert"], ["roletype", "window", "dialog"]] };
ma.default = Qh;
var va = {};
Object.defineProperty(va, "__esModule", { value: true });
va.default = void 0;
var Yh = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-activedescendant": null, "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "Device Independence Delivery Unit" } }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
va.default = Yh;
var ba = {};
Object.defineProperty(ba, "__esModule", { value: true });
ba.default = void 0;
var Zh = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-posinset": null, "aria-setsize": null }, relatedConcepts: [{ concept: { name: "article" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "document"]] };
ba.default = Zh;
var ya = {};
Object.defineProperty(ya, "__esModule", { value: true });
ya.default = void 0;
var ef = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { constraints: ["scoped to the body element"], name: "header" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
ya.default = ef;
var ga = {};
Object.defineProperty(ga, "__esModule", { value: true });
ga.default = void 0;
var tf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "blockquote" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
ga.default = tf;
var xa = {};
Object.defineProperty(xa, "__esModule", { value: true });
xa.default = void 0;
var rf = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-disabled": null, "aria-expanded": null, "aria-haspopup": null, "aria-pressed": null }, relatedConcepts: [{ concept: { attributes: [{ name: "type", value: "button" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ name: "type", value: "image" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ name: "type", value: "reset" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ name: "type", value: "submit" }], name: "input" }, module: "HTML" }, { concept: { name: "button" }, module: "HTML" }, { concept: { name: "trigger" }, module: "XForms" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command"]] };
xa.default = rf;
var _a = {};
Object.defineProperty(_a, "__esModule", { value: true });
_a.default = void 0;
var af = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "caption" }, module: "HTML" }], requireContextRole: ["figure", "grid", "table"], requiredContextRole: ["figure", "grid", "table"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
_a.default = af;
var Ca = {};
Object.defineProperty(Ca, "__esModule", { value: true });
Ca.default = void 0;
var sf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-colindex": null, "aria-colspan": null, "aria-rowindex": null, "aria-rowspan": null }, relatedConcepts: [{ concept: { constraints: ["ancestor table element has table role"], name: "td" }, module: "HTML" }], requireContextRole: ["row"], requiredContextRole: ["row"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
Ca.default = sf;
var Ra = {};
Object.defineProperty(Ra, "__esModule", { value: true });
Ra.default = void 0;
var nf = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-checked": null, "aria-errormessage": null, "aria-expanded": null, "aria-invalid": null, "aria-readonly": null, "aria-required": null }, relatedConcepts: [{ concept: { attributes: [{ name: "type", value: "checkbox" }], name: "input" }, module: "HTML" }, { concept: { name: "option" }, module: "ARIA" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-checked": null }, superClass: [["roletype", "widget", "input"]] };
Ra.default = nf;
var Pa = {};
Object.defineProperty(Pa, "__esModule", { value: true });
Pa.default = void 0;
var of = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "code" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
Pa.default = of;
var wa = {};
Object.defineProperty(wa, "__esModule", { value: true });
wa.default = void 0;
var lf = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-sort": null }, relatedConcepts: [{ concept: { name: "th" }, module: "HTML" }, { concept: { attributes: [{ name: "scope", value: "col" }], name: "th" }, module: "HTML" }, { concept: { attributes: [{ name: "scope", value: "colgroup" }], name: "th" }, module: "HTML" }], requireContextRole: ["row"], requiredContextRole: ["row"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "cell"], ["roletype", "structure", "section", "cell", "gridcell"], ["roletype", "widget", "gridcell"], ["roletype", "structure", "sectionhead"]] };
wa.default = lf;
var Ta = {};
Object.defineProperty(Ta, "__esModule", { value: true });
Ta.default = void 0;
var uf = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-activedescendant": null, "aria-autocomplete": null, "aria-errormessage": null, "aria-invalid": null, "aria-readonly": null, "aria-required": null, "aria-expanded": "false", "aria-haspopup": "listbox" }, relatedConcepts: [{ concept: { attributes: [{ constraints: ["set"], name: "list" }, { name: "type", value: "email" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "list" }, { name: "type", value: "search" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "list" }, { name: "type", value: "tel" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "list" }, { name: "type", value: "text" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "list" }, { name: "type", value: "url" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "list" }, { name: "type", value: "url" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["undefined"], name: "multiple" }, { constraints: ["undefined"], name: "size" }], constraints: ["the multiple attribute is not set and the size attribute does not have a value greater than 1"], name: "select" }, module: "HTML" }, { concept: { name: "select" }, module: "XForms" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-controls": null, "aria-expanded": "false" }, superClass: [["roletype", "widget", "input"]] };
Ta.default = uf;
var Sa = {};
Object.defineProperty(Sa, "__esModule", { value: true });
Sa.default = void 0;
var cf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { constraints: ["scoped to the body element", "scoped to the main element"], name: "aside" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "aria-label" }], constraints: ["scoped to a sectioning content element", "scoped to a sectioning root element other than body"], name: "aside" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "aria-labelledby" }], constraints: ["scoped to a sectioning content element", "scoped to a sectioning root element other than body"], name: "aside" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
Sa.default = cf;
var Aa = {};
Object.defineProperty(Aa, "__esModule", { value: true });
Aa.default = void 0;
var df = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { constraints: ["scoped to the body element"], name: "footer" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
Aa.default = df;
var Ea = {};
Object.defineProperty(Ea, "__esModule", { value: true });
Ea.default = void 0;
var pf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "dd" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
Ea.default = pf;
var ka = {};
Object.defineProperty(ka, "__esModule", { value: true });
ka.default = void 0;
var hf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "del" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
ka.default = hf;
var Ia = {};
Object.defineProperty(Ia, "__esModule", { value: true });
Ia.default = void 0;
var ff = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "dialog" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "window"]] };
Ia.default = ff;
var qa = {};
Object.defineProperty(qa, "__esModule", { value: true });
qa.default = void 0;
var mf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ module: "DAISY Guide" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "list"]] };
qa.default = mf;
var Ma = {};
Object.defineProperty(Ma, "__esModule", { value: true });
Ma.default = void 0;
var vf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "Device Independence Delivery Unit" } }, { concept: { name: "html" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
Ma.default = vf;
var La = {};
Object.defineProperty(La, "__esModule", { value: true });
La.default = void 0;
var bf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "em" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
La.default = bf;
var Oa = {};
Object.defineProperty(Oa, "__esModule", { value: true });
Oa.default = void 0;
var yf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["article"]], requiredProps: {}, superClass: [["roletype", "structure", "section", "list"]] };
Oa.default = yf;
var Na = {};
Object.defineProperty(Na, "__esModule", { value: true });
Na.default = void 0;
var gf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "figure" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
Na.default = gf;
var Da = {};
Object.defineProperty(Da, "__esModule", { value: true });
Da.default = void 0;
var xf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { attributes: [{ constraints: ["set"], name: "aria-label" }], name: "form" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "aria-labelledby" }], name: "form" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "name" }], name: "form" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
Da.default = xf;
var ja = {};
Object.defineProperty(ja, "__esModule", { value: true });
ja.default = void 0;
var _f = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "a" }, module: "HTML" }, { concept: { name: "area" }, module: "HTML" }, { concept: { name: "aside" }, module: "HTML" }, { concept: { name: "b" }, module: "HTML" }, { concept: { name: "bdo" }, module: "HTML" }, { concept: { name: "body" }, module: "HTML" }, { concept: { name: "data" }, module: "HTML" }, { concept: { name: "div" }, module: "HTML" }, { concept: { constraints: ["scoped to the main element", "scoped to a sectioning content element", "scoped to a sectioning root element other than body"], name: "footer" }, module: "HTML" }, { concept: { constraints: ["scoped to the main element", "scoped to a sectioning content element", "scoped to a sectioning root element other than body"], name: "header" }, module: "HTML" }, { concept: { name: "hgroup" }, module: "HTML" }, { concept: { name: "i" }, module: "HTML" }, { concept: { name: "pre" }, module: "HTML" }, { concept: { name: "q" }, module: "HTML" }, { concept: { name: "samp" }, module: "HTML" }, { concept: { name: "section" }, module: "HTML" }, { concept: { name: "small" }, module: "HTML" }, { concept: { name: "span" }, module: "HTML" }, { concept: { name: "u" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
ja.default = _f;
var $a = {};
Object.defineProperty($a, "__esModule", { value: true });
$a.default = void 0;
var Cf = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-multiselectable": null, "aria-readonly": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["row"], ["row", "rowgroup"]], requiredProps: {}, superClass: [["roletype", "widget", "composite"], ["roletype", "structure", "section", "table"]] };
$a.default = Cf;
var Fa = {};
Object.defineProperty(Fa, "__esModule", { value: true });
Fa.default = void 0;
var Rf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null, "aria-readonly": null, "aria-required": null, "aria-selected": null }, relatedConcepts: [{ concept: { constraints: ["ancestor table element has grid role", "ancestor table element has treegrid role"], name: "td" }, module: "HTML" }], requireContextRole: ["row"], requiredContextRole: ["row"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "cell"], ["roletype", "widget"]] };
Fa.default = Rf;
var Ba = {};
Object.defineProperty(Ba, "__esModule", { value: true });
Ba.default = void 0;
var Pf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-activedescendant": null, "aria-disabled": null }, relatedConcepts: [{ concept: { name: "details" }, module: "HTML" }, { concept: { name: "fieldset" }, module: "HTML" }, { concept: { name: "optgroup" }, module: "HTML" }, { concept: { name: "address" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
Ba.default = Pf;
var Va = {};
Object.defineProperty(Va, "__esModule", { value: true });
Va.default = void 0;
var wf = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-level": "2" }, relatedConcepts: [{ concept: { name: "h1" }, module: "HTML" }, { concept: { name: "h2" }, module: "HTML" }, { concept: { name: "h3" }, module: "HTML" }, { concept: { name: "h4" }, module: "HTML" }, { concept: { name: "h5" }, module: "HTML" }, { concept: { name: "h6" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-level": "2" }, superClass: [["roletype", "structure", "sectionhead"]] };
Va.default = wf;
var Ha = {};
Object.defineProperty(Ha, "__esModule", { value: true });
Ha.default = void 0;
var Tf = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { attributes: [{ constraints: ["set"], name: "alt" }], name: "img" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["undefined"], name: "alt" }], name: "img" }, module: "HTML" }, { concept: { name: "imggroup" }, module: "DTB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
Ha.default = Tf;
var Ua = {};
Object.defineProperty(Ua, "__esModule", { value: true });
Ua.default = void 0;
var Sf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "ins" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
Ua.default = Sf;
var Ga = {};
Object.defineProperty(Ga, "__esModule", { value: true });
Ga.default = void 0;
var Af = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-disabled": null, "aria-expanded": null, "aria-haspopup": null }, relatedConcepts: [{ concept: { attributes: [{ constraints: ["set"], name: "href" }], name: "a" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "href" }], name: "area" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command"]] };
Ga.default = Af;
var za = {};
Object.defineProperty(za, "__esModule", { value: true });
za.default = void 0;
var Ef = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "menu" }, module: "HTML" }, { concept: { name: "ol" }, module: "HTML" }, { concept: { name: "ul" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["listitem"]], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
za.default = Ef;
var Wa = {};
Object.defineProperty(Wa, "__esModule", { value: true });
Wa.default = void 0;
var kf = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-expanded": null, "aria-invalid": null, "aria-multiselectable": null, "aria-readonly": null, "aria-required": null, "aria-orientation": "vertical" }, relatedConcepts: [{ concept: { attributes: [{ constraints: [">1"], name: "size" }], constraints: ["the size attribute value is greater than 1"], name: "select" }, module: "HTML" }, { concept: { attributes: [{ name: "multiple" }], name: "select" }, module: "HTML" }, { concept: { name: "datalist" }, module: "HTML" }, { concept: { name: "list" }, module: "ARIA" }, { concept: { name: "select" }, module: "XForms" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["option", "group"], ["option"]], requiredProps: {}, superClass: [["roletype", "widget", "composite", "select"], ["roletype", "structure", "section", "group", "select"]] };
Wa.default = kf;
var Ka = {};
Object.defineProperty(Ka, "__esModule", { value: true });
Ka.default = void 0;
var If = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-level": null, "aria-posinset": null, "aria-setsize": null }, relatedConcepts: [{ concept: { constraints: ["direct descendant of ol", "direct descendant of ul", "direct descendant of menu"], name: "li" }, module: "HTML" }, { concept: { name: "item" }, module: "XForms" }], requireContextRole: ["directory", "list"], requiredContextRole: ["directory", "list"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
Ka.default = If;
var Xa = {};
Object.defineProperty(Xa, "__esModule", { value: true });
Xa.default = void 0;
var qf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-live": "polite" }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
Xa.default = qf;
var Ja = {};
Object.defineProperty(Ja, "__esModule", { value: true });
Ja.default = void 0;
var Mf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "main" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
Ja.default = Mf;
var Qa = {};
Object.defineProperty(Qa, "__esModule", { value: true });
Qa.default = void 0;
var Lf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: [], props: { "aria-braillelabel": null, "aria-brailleroledescription": null, "aria-description": null }, relatedConcepts: [{ concept: { name: "mark" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
Qa.default = Lf;
var Ya = {};
Object.defineProperty(Ya, "__esModule", { value: true });
Ya.default = void 0;
var Of = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
Ya.default = Of;
var Za = {};
Object.defineProperty(Za, "__esModule", { value: true });
Za.default = void 0;
var Nf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "math" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
Za.default = Nf;
var ei = {};
Object.defineProperty(ei, "__esModule", { value: true });
ei.default = void 0;
var Df = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-orientation": "vertical" }, relatedConcepts: [{ concept: { name: "MENU" }, module: "JAPI" }, { concept: { name: "list" }, module: "ARIA" }, { concept: { name: "select" }, module: "XForms" }, { concept: { name: "sidebar" }, module: "DTB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["menuitem", "group"], ["menuitemradio", "group"], ["menuitemcheckbox", "group"], ["menuitem"], ["menuitemcheckbox"], ["menuitemradio"]], requiredProps: {}, superClass: [["roletype", "widget", "composite", "select"], ["roletype", "structure", "section", "group", "select"]] };
ei.default = Df;
var ti = {};
Object.defineProperty(ti, "__esModule", { value: true });
ti.default = void 0;
var jf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-orientation": "horizontal" }, relatedConcepts: [{ concept: { name: "toolbar" }, module: "ARIA" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["menuitem", "group"], ["menuitemradio", "group"], ["menuitemcheckbox", "group"], ["menuitem"], ["menuitemcheckbox"], ["menuitemradio"]], requiredProps: {}, superClass: [["roletype", "widget", "composite", "select", "menu"], ["roletype", "structure", "section", "group", "select", "menu"]] };
ti.default = jf;
var ri = {};
Object.defineProperty(ri, "__esModule", { value: true });
ri.default = void 0;
var $f = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-disabled": null, "aria-expanded": null, "aria-haspopup": null, "aria-posinset": null, "aria-setsize": null }, relatedConcepts: [{ concept: { name: "MENU_ITEM" }, module: "JAPI" }, { concept: { name: "listitem" }, module: "ARIA" }, { concept: { name: "option" }, module: "ARIA" }], requireContextRole: ["group", "menu", "menubar"], requiredContextRole: ["group", "menu", "menubar"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command"]] };
ri.default = $f;
var ai = {};
Object.defineProperty(ai, "__esModule", { value: true });
ai.default = void 0;
var Ff = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "menuitem" }, module: "ARIA" }], requireContextRole: ["group", "menu", "menubar"], requiredContextRole: ["group", "menu", "menubar"], requiredOwnedElements: [], requiredProps: { "aria-checked": null }, superClass: [["roletype", "widget", "input", "checkbox"], ["roletype", "widget", "command", "menuitem"]] };
ai.default = Ff;
var ii = {};
Object.defineProperty(ii, "__esModule", { value: true });
ii.default = void 0;
var Bf = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "menuitem" }, module: "ARIA" }], requireContextRole: ["group", "menu", "menubar"], requiredContextRole: ["group", "menu", "menubar"], requiredOwnedElements: [], requiredProps: { "aria-checked": null }, superClass: [["roletype", "widget", "input", "checkbox", "menuitemcheckbox"], ["roletype", "widget", "command", "menuitem", "menuitemcheckbox"], ["roletype", "widget", "input", "radio"]] };
ii.default = Bf;
var si = {};
Object.defineProperty(si, "__esModule", { value: true });
si.default = void 0;
var Vf = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-valuetext": null, "aria-valuemax": "100", "aria-valuemin": "0" }, relatedConcepts: [{ concept: { name: "meter" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-valuenow": null }, superClass: [["roletype", "structure", "range"]] };
si.default = Vf;
var ni = {};
Object.defineProperty(ni, "__esModule", { value: true });
ni.default = void 0;
var Hf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "nav" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
ni.default = Hf;
var oi = {};
Object.defineProperty(oi, "__esModule", { value: true });
oi.default = void 0;
var Uf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: [], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [] };
oi.default = Uf;
var li = {};
Object.defineProperty(li, "__esModule", { value: true });
li.default = void 0;
var Gf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
li.default = Gf;
var ui = {};
Object.defineProperty(ui, "__esModule", { value: true });
ui.default = void 0;
var zf = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-checked": null, "aria-posinset": null, "aria-setsize": null, "aria-selected": "false" }, relatedConcepts: [{ concept: { name: "item" }, module: "XForms" }, { concept: { name: "listitem" }, module: "ARIA" }, { concept: { name: "option" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-selected": "false" }, superClass: [["roletype", "widget", "input"]] };
ui.default = zf;
var ci = {};
Object.defineProperty(ci, "__esModule", { value: true });
ci.default = void 0;
var Wf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "p" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
ci.default = Wf;
var di = {};
Object.defineProperty(di, "__esModule", { value: true });
di.default = void 0;
var Kf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { attributes: [{ name: "alt", value: "" }], name: "img" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
di.default = Kf;
var pi = {};
Object.defineProperty(pi, "__esModule", { value: true });
pi.default = void 0;
var Xf = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-valuetext": null }, relatedConcepts: [{ concept: { name: "progress" }, module: "HTML" }, { concept: { name: "status" }, module: "ARIA" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "range"], ["roletype", "widget"]] };
pi.default = Xf;
var hi = {};
Object.defineProperty(hi, "__esModule", { value: true });
hi.default = void 0;
var Jf = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-checked": null, "aria-posinset": null, "aria-setsize": null }, relatedConcepts: [{ concept: { attributes: [{ name: "type", value: "radio" }], name: "input" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-checked": null }, superClass: [["roletype", "widget", "input"]] };
hi.default = Jf;
var fi = {};
Object.defineProperty(fi, "__esModule", { value: true });
fi.default = void 0;
var Qf = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null, "aria-readonly": null, "aria-required": null }, relatedConcepts: [{ concept: { name: "list" }, module: "ARIA" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["radio"]], requiredProps: {}, superClass: [["roletype", "widget", "composite", "select"], ["roletype", "structure", "section", "group", "select"]] };
fi.default = Qf;
var mi = {};
Object.defineProperty(mi, "__esModule", { value: true });
mi.default = void 0;
var Yf = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { attributes: [{ constraints: ["set"], name: "aria-label" }], name: "section" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "aria-labelledby" }], name: "section" }, module: "HTML" }, { concept: { name: "Device Independence Glossart perceivable unit" } }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
mi.default = Yf;
var vi = {};
Object.defineProperty(vi, "__esModule", { value: true });
vi.default = void 0;
var Zf = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-colindex": null, "aria-expanded": null, "aria-level": null, "aria-posinset": null, "aria-rowindex": null, "aria-selected": null, "aria-setsize": null }, relatedConcepts: [{ concept: { name: "tr" }, module: "HTML" }], requireContextRole: ["grid", "rowgroup", "table", "treegrid"], requiredContextRole: ["grid", "rowgroup", "table", "treegrid"], requiredOwnedElements: [["cell"], ["columnheader"], ["gridcell"], ["rowheader"]], requiredProps: {}, superClass: [["roletype", "structure", "section", "group"], ["roletype", "widget"]] };
vi.default = Zf;
var bi = {};
Object.defineProperty(bi, "__esModule", { value: true });
bi.default = void 0;
var em = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "tbody" }, module: "HTML" }, { concept: { name: "tfoot" }, module: "HTML" }, { concept: { name: "thead" }, module: "HTML" }], requireContextRole: ["grid", "table", "treegrid"], requiredContextRole: ["grid", "table", "treegrid"], requiredOwnedElements: [["row"]], requiredProps: {}, superClass: [["roletype", "structure"]] };
bi.default = em;
var yi = {};
Object.defineProperty(yi, "__esModule", { value: true });
yi.default = void 0;
var tm = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-sort": null }, relatedConcepts: [{ concept: { attributes: [{ name: "scope", value: "row" }], name: "th" }, module: "HTML" }, { concept: { attributes: [{ name: "scope", value: "rowgroup" }], name: "th" }, module: "HTML" }], requireContextRole: ["row", "rowgroup"], requiredContextRole: ["row", "rowgroup"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "cell"], ["roletype", "structure", "section", "cell", "gridcell"], ["roletype", "widget", "gridcell"], ["roletype", "structure", "sectionhead"]] };
yi.default = tm;
var gi = {};
Object.defineProperty(gi, "__esModule", { value: true });
gi.default = void 0;
var rm = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-valuetext": null, "aria-orientation": "vertical", "aria-valuemax": "100", "aria-valuemin": "0" }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-controls": null, "aria-valuenow": null }, superClass: [["roletype", "structure", "range"], ["roletype", "widget"]] };
gi.default = rm;
var xi = {};
Object.defineProperty(xi, "__esModule", { value: true });
xi.default = void 0;
var am = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
xi.default = am;
var _i = {};
Object.defineProperty(_i, "__esModule", { value: true });
_i.default = void 0;
var im = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { attributes: [{ constraints: ["undefined"], name: "list" }, { name: "type", value: "search" }], constraints: ["the list attribute is not set"], name: "input" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "input", "textbox"]] };
_i.default = im;
var Ci = {};
Object.defineProperty(Ci, "__esModule", { value: true });
Ci.default = void 0;
var sm = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-orientation": "horizontal", "aria-valuemax": "100", "aria-valuemin": "0", "aria-valuenow": null, "aria-valuetext": null }, relatedConcepts: [{ concept: { name: "hr" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
Ci.default = sm;
var Ri = {};
Object.defineProperty(Ri, "__esModule", { value: true });
Ri.default = void 0;
var nm = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-haspopup": null, "aria-invalid": null, "aria-readonly": null, "aria-valuetext": null, "aria-orientation": "horizontal", "aria-valuemax": "100", "aria-valuemin": "0" }, relatedConcepts: [{ concept: { attributes: [{ name: "type", value: "range" }], name: "input" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-valuenow": null }, superClass: [["roletype", "widget", "input"], ["roletype", "structure", "range"]] };
Ri.default = nm;
var Pi = {};
Object.defineProperty(Pi, "__esModule", { value: true });
Pi.default = void 0;
var om = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null, "aria-readonly": null, "aria-required": null, "aria-valuetext": null, "aria-valuenow": "0" }, relatedConcepts: [{ concept: { attributes: [{ name: "type", value: "number" }], name: "input" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "composite"], ["roletype", "widget", "input"], ["roletype", "structure", "range"]] };
Pi.default = om;
var wi = {};
Object.defineProperty(wi, "__esModule", { value: true });
wi.default = void 0;
var lm = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-atomic": "true", "aria-live": "polite" }, relatedConcepts: [{ concept: { name: "output" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
wi.default = lm;
var Ti = {};
Object.defineProperty(Ti, "__esModule", { value: true });
Ti.default = void 0;
var um = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "strong" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
Ti.default = um;
var Si = {};
Object.defineProperty(Si, "__esModule", { value: true });
Si.default = void 0;
var cm = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "sub" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
Si.default = cm;
var Ai = {};
Object.defineProperty(Ai, "__esModule", { value: true });
Ai.default = void 0;
var dm = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "sup" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
Ai.default = dm;
var Ei = {};
Object.defineProperty(Ei, "__esModule", { value: true });
Ei.default = void 0;
var pm = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "button" }, module: "ARIA" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-checked": null }, superClass: [["roletype", "widget", "input", "checkbox"]] };
Ei.default = pm;
var ki = {};
Object.defineProperty(ki, "__esModule", { value: true });
ki.default = void 0;
var hm = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-disabled": null, "aria-expanded": null, "aria-haspopup": null, "aria-posinset": null, "aria-setsize": null, "aria-selected": "false" }, relatedConcepts: [], requireContextRole: ["tablist"], requiredContextRole: ["tablist"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "sectionhead"], ["roletype", "widget"]] };
ki.default = hm;
var Ii = {};
Object.defineProperty(Ii, "__esModule", { value: true });
Ii.default = void 0;
var fm = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-colcount": null, "aria-rowcount": null }, relatedConcepts: [{ concept: { name: "table" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["row"], ["row", "rowgroup"]], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
Ii.default = fm;
var qi = {};
Object.defineProperty(qi, "__esModule", { value: true });
qi.default = void 0;
var mm = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-level": null, "aria-multiselectable": null, "aria-orientation": "horizontal" }, relatedConcepts: [{ module: "DAISY", concept: { name: "guide" } }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["tab"]], requiredProps: {}, superClass: [["roletype", "widget", "composite"]] };
qi.default = mm;
var Mi = {};
Object.defineProperty(Mi, "__esModule", { value: true });
Mi.default = void 0;
var vm = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
Mi.default = vm;
var Li = {};
Object.defineProperty(Li, "__esModule", { value: true });
Li.default = void 0;
var bm = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "dfn" }, module: "HTML" }, { concept: { name: "dt" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
Li.default = bm;
var Oi = {};
Object.defineProperty(Oi, "__esModule", { value: true });
Oi.default = void 0;
var ym = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-activedescendant": null, "aria-autocomplete": null, "aria-errormessage": null, "aria-haspopup": null, "aria-invalid": null, "aria-multiline": null, "aria-placeholder": null, "aria-readonly": null, "aria-required": null }, relatedConcepts: [{ concept: { attributes: [{ constraints: ["undefined"], name: "type" }, { constraints: ["undefined"], name: "list" }], constraints: ["the list attribute is not set"], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["undefined"], name: "list" }, { name: "type", value: "email" }], constraints: ["the list attribute is not set"], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["undefined"], name: "list" }, { name: "type", value: "tel" }], constraints: ["the list attribute is not set"], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["undefined"], name: "list" }, { name: "type", value: "text" }], constraints: ["the list attribute is not set"], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["undefined"], name: "list" }, { name: "type", value: "url" }], constraints: ["the list attribute is not set"], name: "input" }, module: "HTML" }, { concept: { name: "input" }, module: "XForms" }, { concept: { name: "textarea" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "input"]] };
Oi.default = ym;
var Ni = {};
Object.defineProperty(Ni, "__esModule", { value: true });
Ni.default = void 0;
var gm = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "time" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
Ni.default = gm;
var Di = {};
Object.defineProperty(Di, "__esModule", { value: true });
Di.default = void 0;
var xm = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "status"]] };
Di.default = xm;
var ji = {};
Object.defineProperty(ji, "__esModule", { value: true });
ji.default = void 0;
var _m = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-orientation": "horizontal" }, relatedConcepts: [{ concept: { name: "menubar" }, module: "ARIA" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "group"]] };
ji.default = _m;
var $i = {};
Object.defineProperty($i, "__esModule", { value: true });
$i.default = void 0;
var Cm = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
$i.default = Cm;
var Fi = {};
Object.defineProperty(Fi, "__esModule", { value: true });
Fi.default = void 0;
var Rm = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null, "aria-multiselectable": null, "aria-required": null, "aria-orientation": "vertical" }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["treeitem", "group"], ["treeitem"]], requiredProps: {}, superClass: [["roletype", "widget", "composite", "select"], ["roletype", "structure", "section", "group", "select"]] };
Fi.default = Rm;
var Bi = {};
Object.defineProperty(Bi, "__esModule", { value: true });
Bi.default = void 0;
var Pm = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["row"], ["row", "rowgroup"]], requiredProps: {}, superClass: [["roletype", "widget", "composite", "grid"], ["roletype", "structure", "section", "table", "grid"], ["roletype", "widget", "composite", "select", "tree"], ["roletype", "structure", "section", "group", "select", "tree"]] };
Bi.default = Pm;
var Vi = {};
Object.defineProperty(Vi, "__esModule", { value: true });
Vi.default = void 0;
var wm = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-expanded": null, "aria-haspopup": null }, relatedConcepts: [], requireContextRole: ["group", "tree"], requiredContextRole: ["group", "tree"], requiredOwnedElements: [], requiredProps: { "aria-selected": null }, superClass: [["roletype", "structure", "section", "listitem"], ["roletype", "widget", "input", "option"]] };
Vi.default = wm;
Object.defineProperty(ha, "__esModule", { value: true });
ha.default = void 0;
var Tm = W(fa), Sm = W(ma), Am = W(va), Em = W(ba), km = W(ya), Im = W(ga), qm = W(xa), Mm = W(_a), Lm = W(Ca), Om = W(Ra), Nm = W(Pa), Dm = W(wa), jm = W(Ta), $m = W(Sa), Fm = W(Aa), Bm = W(Ea), Vm = W(ka), Hm = W(Ia), Um = W(qa), Gm = W(Ma), zm = W(La), Wm = W(Oa), Km = W(Na), Xm = W(Da), Jm = W(ja), Qm = W($a), Ym = W(Fa), Zm = W(Ba), ev = W(Va), tv = W(Ha), rv = W(Ua), av = W(Ga), iv = W(za), sv = W(Wa), nv = W(Ka), ov = W(Xa), lv = W(Ja), uv = W(Qa), cv = W(Ya), dv = W(Za), pv = W(ei), hv = W(ti), fv = W(ri), mv = W(ai), vv = W(ii), bv = W(si), yv = W(ni), gv = W(oi), xv = W(li), _v = W(ui), Cv = W(ci), Rv = W(di), Pv = W(pi), wv = W(hi), Tv = W(fi), Sv = W(mi), Av = W(vi), Ev = W(bi), kv = W(yi), Iv = W(gi), qv = W(xi), Mv = W(_i), Lv = W(Ci), Ov = W(Ri), Nv = W(Pi), Dv = W(wi), jv = W(Ti), $v = W(Si), Fv = W(Ai), Bv = W(Ei), Vv = W(ki), Hv = W(Ii), Uv = W(qi), Gv = W(Mi), zv = W(Li), Wv = W(Oi), Kv = W(Ni), Xv = W(Di), Jv = W(ji), Qv = W($i), Yv = W(Fi), Zv = W(Bi), e0 = W(Vi);
function W(e) {
  return e && e.__esModule ? e : { default: e };
}
var t0 = [["alert", Tm.default], ["alertdialog", Sm.default], ["application", Am.default], ["article", Em.default], ["banner", km.default], ["blockquote", Im.default], ["button", qm.default], ["caption", Mm.default], ["cell", Lm.default], ["checkbox", Om.default], ["code", Nm.default], ["columnheader", Dm.default], ["combobox", jm.default], ["complementary", $m.default], ["contentinfo", Fm.default], ["definition", Bm.default], ["deletion", Vm.default], ["dialog", Hm.default], ["directory", Um.default], ["document", Gm.default], ["emphasis", zm.default], ["feed", Wm.default], ["figure", Km.default], ["form", Xm.default], ["generic", Jm.default], ["grid", Qm.default], ["gridcell", Ym.default], ["group", Zm.default], ["heading", ev.default], ["img", tv.default], ["insertion", rv.default], ["link", av.default], ["list", iv.default], ["listbox", sv.default], ["listitem", nv.default], ["log", ov.default], ["main", lv.default], ["mark", uv.default], ["marquee", cv.default], ["math", dv.default], ["menu", pv.default], ["menubar", hv.default], ["menuitem", fv.default], ["menuitemcheckbox", mv.default], ["menuitemradio", vv.default], ["meter", bv.default], ["navigation", yv.default], ["none", gv.default], ["note", xv.default], ["option", _v.default], ["paragraph", Cv.default], ["presentation", Rv.default], ["progressbar", Pv.default], ["radio", wv.default], ["radiogroup", Tv.default], ["region", Sv.default], ["row", Av.default], ["rowgroup", Ev.default], ["rowheader", kv.default], ["scrollbar", Iv.default], ["search", qv.default], ["searchbox", Mv.default], ["separator", Lv.default], ["slider", Ov.default], ["spinbutton", Nv.default], ["status", Dv.default], ["strong", jv.default], ["subscript", $v.default], ["superscript", Fv.default], ["switch", Bv.default], ["tab", Vv.default], ["table", Hv.default], ["tablist", Uv.default], ["tabpanel", Gv.default], ["term", zv.default], ["textbox", Wv.default], ["time", Kv.default], ["timer", Xv.default], ["toolbar", Jv.default], ["tooltip", Qv.default], ["tree", Yv.default], ["treegrid", Zv.default], ["treeitem", e0.default]];
ha.default = t0;
var Hi = {}, Ui = {};
Object.defineProperty(Ui, "__esModule", { value: true });
Ui.default = void 0;
var r0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "abstract [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
Ui.default = r0;
var Gi = {};
Object.defineProperty(Gi, "__esModule", { value: true });
Gi.default = void 0;
var a0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "acknowledgments [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
Gi.default = a0;
var zi = {};
Object.defineProperty(zi, "__esModule", { value: true });
zi.default = void 0;
var i0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "afterword [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
zi.default = i0;
var Wi = {};
Object.defineProperty(Wi, "__esModule", { value: true });
Wi.default = void 0;
var s0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "appendix [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
Wi.default = s0;
var Ki = {};
Object.defineProperty(Ki, "__esModule", { value: true });
Ki.default = void 0;
var n0 = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "referrer [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command", "link"]] };
Ki.default = n0;
var Xi = {};
Object.defineProperty(Xi, "__esModule", { value: true });
Xi.default = void 0;
var o0 = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "EPUB biblioentry [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: ["doc-bibliography"], requiredContextRole: ["doc-bibliography"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "listitem"]] };
Xi.default = o0;
var Ji = {};
Object.defineProperty(Ji, "__esModule", { value: true });
Ji.default = void 0;
var l0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "bibliography [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["doc-biblioentry"]], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
Ji.default = l0;
var Qi = {};
Object.defineProperty(Qi, "__esModule", { value: true });
Qi.default = void 0;
var u0 = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "biblioref [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command", "link"]] };
Qi.default = u0;
var Yi = {};
Object.defineProperty(Yi, "__esModule", { value: true });
Yi.default = void 0;
var c0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "chapter [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
Yi.default = c0;
var Zi = {};
Object.defineProperty(Zi, "__esModule", { value: true });
Zi.default = void 0;
var d0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "colophon [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
Zi.default = d0;
var es = {};
Object.defineProperty(es, "__esModule", { value: true });
es.default = void 0;
var p0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "conclusion [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
es.default = p0;
var ts = {};
Object.defineProperty(ts, "__esModule", { value: true });
ts.default = void 0;
var h0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "cover [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "img"]] };
ts.default = h0;
var rs = {};
Object.defineProperty(rs, "__esModule", { value: true });
rs.default = void 0;
var f0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "credit [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
rs.default = f0;
var as = {};
Object.defineProperty(as, "__esModule", { value: true });
as.default = void 0;
var m0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "credits [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
as.default = m0;
var is = {};
Object.defineProperty(is, "__esModule", { value: true });
is.default = void 0;
var v0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "dedication [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
is.default = v0;
var ss = {};
Object.defineProperty(ss, "__esModule", { value: true });
ss.default = void 0;
var b0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "rearnote [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: ["doc-endnotes"], requiredContextRole: ["doc-endnotes"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "listitem"]] };
ss.default = b0;
var ns = {};
Object.defineProperty(ns, "__esModule", { value: true });
ns.default = void 0;
var y0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "rearnotes [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["doc-endnote"]], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
ns.default = y0;
var os = {};
Object.defineProperty(os, "__esModule", { value: true });
os.default = void 0;
var g0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "epigraph [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
os.default = g0;
var ls = {};
Object.defineProperty(ls, "__esModule", { value: true });
ls.default = void 0;
var x0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "epilogue [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
ls.default = x0;
var us = {};
Object.defineProperty(us, "__esModule", { value: true });
us.default = void 0;
var _0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "errata [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
us.default = _0;
var cs = {};
Object.defineProperty(cs, "__esModule", { value: true });
cs.default = void 0;
var C0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
cs.default = C0;
var ds = {};
Object.defineProperty(ds, "__esModule", { value: true });
ds.default = void 0;
var R0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "footnote [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
ds.default = R0;
var ps = {};
Object.defineProperty(ps, "__esModule", { value: true });
ps.default = void 0;
var P0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "foreword [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
ps.default = P0;
var hs = {};
Object.defineProperty(hs, "__esModule", { value: true });
hs.default = void 0;
var w0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "glossary [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["definition"], ["term"]], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
hs.default = w0;
var fs = {};
Object.defineProperty(fs, "__esModule", { value: true });
fs.default = void 0;
var T0 = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "glossref [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command", "link"]] };
fs.default = T0;
var ms = {};
Object.defineProperty(ms, "__esModule", { value: true });
ms.default = void 0;
var S0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "index [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark", "navigation"]] };
ms.default = S0;
var vs = {};
Object.defineProperty(vs, "__esModule", { value: true });
vs.default = void 0;
var A0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "introduction [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
vs.default = A0;
var bs = {};
Object.defineProperty(bs, "__esModule", { value: true });
bs.default = void 0;
var E0 = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "noteref [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command", "link"]] };
bs.default = E0;
var ys = {};
Object.defineProperty(ys, "__esModule", { value: true });
ys.default = void 0;
var k0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "notice [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "note"]] };
ys.default = k0;
var gs = {};
Object.defineProperty(gs, "__esModule", { value: true });
gs.default = void 0;
var I0 = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "pagebreak [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "separator"]] };
gs.default = I0;
var xs = {};
Object.defineProperty(xs, "__esModule", { value: true });
xs.default = void 0;
var q0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: [], props: { "aria-braillelabel": null, "aria-brailleroledescription": null, "aria-description": null, "aria-disabled": null, "aria-errormessage": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
xs.default = q0;
var _s = {};
Object.defineProperty(_s, "__esModule", { value: true });
_s.default = void 0;
var M0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: [], props: { "aria-braillelabel": null, "aria-brailleroledescription": null, "aria-description": null, "aria-disabled": null, "aria-errormessage": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
_s.default = M0;
var Cs = {};
Object.defineProperty(Cs, "__esModule", { value: true });
Cs.default = void 0;
var L0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "page-list [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark", "navigation"]] };
Cs.default = L0;
var Rs = {};
Object.defineProperty(Rs, "__esModule", { value: true });
Rs.default = void 0;
var O0 = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "part [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
Rs.default = O0;
var Ps = {};
Object.defineProperty(Ps, "__esModule", { value: true });
Ps.default = void 0;
var N0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "preface [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
Ps.default = N0;
var ws = {};
Object.defineProperty(ws, "__esModule", { value: true });
ws.default = void 0;
var D0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "prologue [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
ws.default = D0;
var Ts = {};
Object.defineProperty(Ts, "__esModule", { value: true });
Ts.default = void 0;
var j0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "pullquote [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["none"]] };
Ts.default = j0;
var Ss = {};
Object.defineProperty(Ss, "__esModule", { value: true });
Ss.default = void 0;
var $0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "qna [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
Ss.default = $0;
var As = {};
Object.defineProperty(As, "__esModule", { value: true });
As.default = void 0;
var F0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "subtitle [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "sectionhead"]] };
As.default = F0;
var Es = {};
Object.defineProperty(Es, "__esModule", { value: true });
Es.default = void 0;
var B0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "help [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "note"]] };
Es.default = B0;
var ks = {};
Object.defineProperty(ks, "__esModule", { value: true });
ks.default = void 0;
var V0 = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "toc [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark", "navigation"]] };
ks.default = V0;
Object.defineProperty(Hi, "__esModule", { value: true });
Hi.default = void 0;
var H0 = Ce(Ui), U0 = Ce(Gi), G0 = Ce(zi), z0 = Ce(Wi), W0 = Ce(Ki), K0 = Ce(Xi), X0 = Ce(Ji), J0 = Ce(Qi), Q0 = Ce(Yi), Y0 = Ce(Zi), Z0 = Ce(es), eb = Ce(ts), tb = Ce(rs), rb = Ce(as), ab = Ce(is), ib = Ce(ss), sb = Ce(ns), nb = Ce(os), ob = Ce(ls), lb = Ce(us), ub = Ce(cs), cb = Ce(ds), db = Ce(ps), pb = Ce(hs), hb = Ce(fs), fb = Ce(ms), mb = Ce(vs), vb = Ce(bs), bb = Ce(ys), yb = Ce(gs), gb = Ce(xs), xb = Ce(_s), _b = Ce(Cs), Cb = Ce(Rs), Rb = Ce(Ps), Pb = Ce(ws), wb = Ce(Ts), Tb = Ce(Ss), Sb = Ce(As), Ab = Ce(Es), Eb = Ce(ks);
function Ce(e) {
  return e && e.__esModule ? e : { default: e };
}
var kb = [["doc-abstract", H0.default], ["doc-acknowledgments", U0.default], ["doc-afterword", G0.default], ["doc-appendix", z0.default], ["doc-backlink", W0.default], ["doc-biblioentry", K0.default], ["doc-bibliography", X0.default], ["doc-biblioref", J0.default], ["doc-chapter", Q0.default], ["doc-colophon", Y0.default], ["doc-conclusion", Z0.default], ["doc-cover", eb.default], ["doc-credit", tb.default], ["doc-credits", rb.default], ["doc-dedication", ab.default], ["doc-endnote", ib.default], ["doc-endnotes", sb.default], ["doc-epigraph", nb.default], ["doc-epilogue", ob.default], ["doc-errata", lb.default], ["doc-example", ub.default], ["doc-footnote", cb.default], ["doc-foreword", db.default], ["doc-glossary", pb.default], ["doc-glossref", hb.default], ["doc-index", fb.default], ["doc-introduction", mb.default], ["doc-noteref", vb.default], ["doc-notice", bb.default], ["doc-pagebreak", yb.default], ["doc-pagefooter", gb.default], ["doc-pageheader", xb.default], ["doc-pagelist", _b.default], ["doc-part", Cb.default], ["doc-preface", Rb.default], ["doc-prologue", Pb.default], ["doc-pullquote", wb.default], ["doc-qna", Tb.default], ["doc-subtitle", Sb.default], ["doc-tip", Ab.default], ["doc-toc", Eb.default]];
Hi.default = kb;
var Is = {}, qs = {};
Object.defineProperty(qs, "__esModule", { value: true });
qs.default = void 0;
var Ib = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ module: "GRAPHICS", concept: { name: "graphics-object" } }, { module: "ARIA", concept: { name: "img" } }, { module: "ARIA", concept: { name: "article" } }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "document"]] };
qs.default = Ib;
var Ms = {};
Object.defineProperty(Ms, "__esModule", { value: true });
Ms.default = void 0;
var qb = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ module: "GRAPHICS", concept: { name: "graphics-document" } }, { module: "ARIA", concept: { name: "group" } }, { module: "ARIA", concept: { name: "img" } }, { module: "GRAPHICS", concept: { name: "graphics-symbol" } }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "group"]] };
Ms.default = qb;
var Ls = {};
Object.defineProperty(Ls, "__esModule", { value: true });
Ls.default = void 0;
var Mb = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "img"]] };
Ls.default = Mb;
Object.defineProperty(Is, "__esModule", { value: true });
Is.default = void 0;
var Lb = lu(qs), Ob = lu(Ms), Nb = lu(Ls);
function lu(e) {
  return e && e.__esModule ? e : { default: e };
}
var Db = [["graphics-document", Lb.default], ["graphics-object", Ob.default], ["graphics-symbol", Nb.default]];
Is.default = Db;
Object.defineProperty(or, "__esModule", { value: true });
or.default = void 0;
var jb = Pr(ea), $b = Pr(ha), Fb = Pr(Hi), Bb = Pr(Is), Vb = Pr(Gt);
function Pr(e) {
  return e && e.__esModule ? e : { default: e };
}
function $l(e, t) {
  var r = typeof Symbol < "u" && e[Symbol.iterator] || e["@@iterator"];
  if (!r) {
    if (Array.isArray(e) || (r = Qc(e)) || t) {
      r && (e = r);
      var s = 0, o = function() {
      };
      return { s: o, n: function() {
        return s >= e.length ? { done: true } : { done: false, value: e[s++] };
      }, e: function(l) {
        throw l;
      }, f: o };
    }
    throw new TypeError(`Invalid attempt to iterate non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  var p, h = true, v = false;
  return { s: function() {
    r = r.call(e);
  }, n: function() {
    var l = r.next();
    return h = l.done, l;
  }, e: function(l) {
    v = true, p = l;
  }, f: function() {
    try {
      h || r.return == null || r.return();
    } finally {
      if (v) throw p;
    }
  } };
}
function fr(e, t) {
  return Gb(e) || Ub(e, t) || Qc(e, t) || Hb();
}
function Hb() {
  throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
}
function Qc(e, t) {
  if (e) {
    if (typeof e == "string") return Ou(e, t);
    var r = {}.toString.call(e).slice(8, -1);
    return r === "Object" && e.constructor && (r = e.constructor.name), r === "Map" || r === "Set" ? Array.from(e) : r === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(r) ? Ou(e, t) : void 0;
  }
}
function Ou(e, t) {
  (t == null || t > e.length) && (t = e.length);
  for (var r = 0, s = Array(t); r < t; r++) s[r] = e[r];
  return s;
}
function Ub(e, t) {
  var r = e == null ? null : typeof Symbol < "u" && e[Symbol.iterator] || e["@@iterator"];
  if (r != null) {
    var s, o, p, h, v = [], m = true, l = false;
    try {
      if (p = (r = r.call(e)).next, t === 0) {
        if (Object(r) !== r) return;
        m = false;
      } else for (; !(m = (s = p.call(r)).done) && (v.push(s.value), v.length !== t); m = true) ;
    } catch (w) {
      l = true, o = w;
    } finally {
      try {
        if (!m && r.return != null && (h = r.return(), Object(h) !== h)) return;
      } finally {
        if (l) throw o;
      }
    }
    return v;
  }
}
function Gb(e) {
  if (Array.isArray(e)) return e;
}
var It = [].concat(jb.default, $b.default, Fb.default, Bb.default);
It.forEach(function(e) {
  var t = fr(e, 2), r = t[1], s = $l(r.superClass), o;
  try {
    for (s.s(); !(o = s.n()).done; ) {
      var p = o.value, h = $l(p), v;
      try {
        var m = function() {
          var w = v.value, U = It.filter(function(B) {
            var $ = fr(B, 1), L = $[0];
            return L === w;
          })[0];
          if (U) for (var J = U[1], z = 0, K = Object.keys(J.props); z < K.length; z++) {
            var Z = K[z];
            Object.prototype.hasOwnProperty.call(r.props, Z) || (r.props[Z] = J.props[Z]);
          }
        };
        for (h.s(); !(v = h.n()).done; ) m();
      } catch (l) {
        h.e(l);
      } finally {
        h.f();
      }
    }
  } catch (l) {
    s.e(l);
  } finally {
    s.f();
  }
});
var Fl = { entries: function() {
  return It;
}, forEach: function(t) {
  var r = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, s = $l(It), o;
  try {
    for (s.s(); !(o = s.n()).done; ) {
      var p = fr(o.value, 2), h = p[0], v = p[1];
      t.call(r, v, h, It);
    }
  } catch (m) {
    s.e(m);
  } finally {
    s.f();
  }
}, get: function(t) {
  var r = It.filter(function(s) {
    return s[0] === t;
  })[0];
  return r && r[1];
}, has: function(t) {
  return !!Fl.get(t);
}, keys: function() {
  return It.map(function(t) {
    var r = fr(t, 1), s = r[0];
    return s;
  });
}, values: function() {
  return It.map(function(t) {
    var r = fr(t, 2), s = r[1];
    return s;
  });
} };
or.default = (0, Vb.default)(Fl, Fl.entries());
var Os = {};
Object.defineProperty(Os, "__esModule", { value: true });
Os.default = void 0;
var zb = Zc(Gt), Yc = Zc(or);
function Zc(e) {
  return e && e.__esModule ? e : { default: e };
}
function yl(e, t) {
  return Jb(e) || Xb(e, t) || Kb(e, t) || Wb();
}
function Wb() {
  throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
}
function Kb(e, t) {
  if (e) {
    if (typeof e == "string") return Nu(e, t);
    var r = {}.toString.call(e).slice(8, -1);
    return r === "Object" && e.constructor && (r = e.constructor.name), r === "Map" || r === "Set" ? Array.from(e) : r === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(r) ? Nu(e, t) : void 0;
  }
}
function Nu(e, t) {
  (t == null || t > e.length) && (t = e.length);
  for (var r = 0, s = Array(t); r < t; r++) s[r] = e[r];
  return s;
}
function Xb(e, t) {
  var r = e == null ? null : typeof Symbol < "u" && e[Symbol.iterator] || e["@@iterator"];
  if (r != null) {
    var s, o, p, h, v = [], m = true, l = false;
    try {
      if (p = (r = r.call(e)).next, t === 0) {
        if (Object(r) !== r) return;
        m = false;
      } else for (; !(m = (s = p.call(r)).done) && (v.push(s.value), v.length !== t); m = true) ;
    } catch (w) {
      l = true, o = w;
    } finally {
      try {
        if (!m && r.return != null && (h = r.return(), Object(h) !== h)) return;
      } finally {
        if (l) throw o;
      }
    }
    return v;
  }
}
function Jb(e) {
  if (Array.isArray(e)) return e;
}
var qt = [], Du = Yc.default.keys();
for (var gl = 0; gl < Du.length; gl++) {
  var xl = Du[gl], _l = Yc.default.get(xl);
  if (_l) for (var ju = [].concat(_l.baseConcepts, _l.relatedConcepts), Qb = function() {
    var t = ju[Cl];
    if (t.module === "HTML") {
      var r = t.concept;
      if (r) {
        var s = qt.filter(function(v) {
          return Yb(v[0], r);
        })[0], o;
        s ? o = s[1] : o = [];
        for (var p = true, h = 0; h < o.length; h++) if (o[h] === xl) {
          p = false;
          break;
        }
        p && o.push(xl), s || qt.push([r, o]);
      }
    }
  }, Cl = 0; Cl < ju.length; Cl++) Qb();
}
var Bl = { entries: function() {
  return qt;
}, forEach: function(t) {
  for (var r = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, s = 0, o = qt; s < o.length; s++) {
    var p = yl(o[s], 2), h = p[0], v = p[1];
    t.call(r, v, h, qt);
  }
}, get: function(t) {
  var r = qt.filter(function(s) {
    return t.name === s[0].name && ed(t.attributes, s[0].attributes);
  })[0];
  return r && r[1];
}, has: function(t) {
  return !!Bl.get(t);
}, keys: function() {
  return qt.map(function(t) {
    var r = yl(t, 1), s = r[0];
    return s;
  });
}, values: function() {
  return qt.map(function(t) {
    var r = yl(t, 2), s = r[1];
    return s;
  });
} };
function Yb(e, t) {
  return e.name === t.name && Zb(e.constraints, t.constraints) && ed(e.attributes, t.attributes);
}
function Zb(e, t) {
  if (e === void 0 && t !== void 0 || e !== void 0 && t === void 0) return false;
  if (e !== void 0 && t !== void 0) {
    if (e.length !== t.length) return false;
    for (var r = 0; r < e.length; r++) if (e[r] !== t[r]) return false;
  }
  return true;
}
function ed(e, t) {
  if (e === void 0 && t !== void 0 || e !== void 0 && t === void 0) return false;
  if (e !== void 0 && t !== void 0) {
    if (e.length !== t.length) return false;
    for (var r = 0; r < e.length; r++) {
      if (e[r].name !== t[r].name || e[r].value !== t[r].value || e[r].constraints === void 0 && t[r].constraints !== void 0 || e[r].constraints !== void 0 && t[r].constraints === void 0) return false;
      if (e[r].constraints !== void 0 && t[r].constraints !== void 0) {
        if (e[r].constraints.length !== t[r].constraints.length) return false;
        for (var s = 0; s < e[r].constraints.length; s++) if (e[r].constraints[s] !== t[r].constraints[s]) return false;
      }
    }
  }
  return true;
}
Os.default = (0, zb.default)(Bl, Bl.entries());
var Ns = {};
Object.defineProperty(Ns, "__esModule", { value: true });
Ns.default = void 0;
var ey = rd(Gt), td = rd(or);
function rd(e) {
  return e && e.__esModule ? e : { default: e };
}
function Rl(e, t) {
  return iy(e) || ay(e, t) || ry(e, t) || ty();
}
function ty() {
  throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
}
function ry(e, t) {
  if (e) {
    if (typeof e == "string") return $u(e, t);
    var r = {}.toString.call(e).slice(8, -1);
    return r === "Object" && e.constructor && (r = e.constructor.name), r === "Map" || r === "Set" ? Array.from(e) : r === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(r) ? $u(e, t) : void 0;
  }
}
function $u(e, t) {
  (t == null || t > e.length) && (t = e.length);
  for (var r = 0, s = Array(t); r < t; r++) s[r] = e[r];
  return s;
}
function ay(e, t) {
  var r = e == null ? null : typeof Symbol < "u" && e[Symbol.iterator] || e["@@iterator"];
  if (r != null) {
    var s, o, p, h, v = [], m = true, l = false;
    try {
      if (p = (r = r.call(e)).next, t === 0) {
        if (Object(r) !== r) return;
        m = false;
      } else for (; !(m = (s = p.call(r)).done) && (v.push(s.value), v.length !== t); m = true) ;
    } catch (w) {
      l = true, o = w;
    } finally {
      try {
        if (!m && r.return != null && (h = r.return(), Object(h) !== h)) return;
      } finally {
        if (l) throw o;
      }
    }
    return v;
  }
}
function iy(e) {
  if (Array.isArray(e)) return e;
}
var Bt = [], Fu = td.default.keys();
for (var Pl = 0; Pl < Fu.length; Pl++) {
  var Bu = Fu[Pl], wl = td.default.get(Bu), Tl = [];
  if (wl) {
    for (var Vu = [].concat(wl.baseConcepts, wl.relatedConcepts), Sl = 0; Sl < Vu.length; Sl++) {
      var Hu = Vu[Sl];
      if (Hu.module === "HTML") {
        var Uu = Hu.concept;
        Uu != null && Tl.push(Uu);
      }
    }
    Tl.length > 0 && Bt.push([Bu, Tl]);
  }
}
var Vl = { entries: function() {
  return Bt;
}, forEach: function(t) {
  for (var r = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, s = 0, o = Bt; s < o.length; s++) {
    var p = Rl(o[s], 2), h = p[0], v = p[1];
    t.call(r, v, h, Bt);
  }
}, get: function(t) {
  var r = Bt.filter(function(s) {
    return s[0] === t;
  })[0];
  return r && r[1];
}, has: function(t) {
  return !!Vl.get(t);
}, keys: function() {
  return Bt.map(function(t) {
    var r = Rl(t, 1), s = r[0];
    return s;
  });
}, values: function() {
  return Bt.map(function(t) {
    var r = Rl(t, 2), s = r[1];
    return s;
  });
} };
Ns.default = (0, ey.default)(Vl, Vl.entries());
var Ds;
Object.defineProperty(ft, "__esModule", { value: true });
var js = ft.roles = ft.roleElements = Ds = ft.elementRoles = ft.dom = ft.aria = void 0, sy = wr(Qr), ny = wr(Zr), oy = wr(or), ly = wr(Os), uy = wr(Ns);
function wr(e) {
  return e && e.__esModule ? e : { default: e };
}
ft.aria = sy.default;
ft.dom = ny.default;
js = ft.roles = oy.default;
Ds = ft.elementRoles = ly.default;
ft.roleElements = uy.default;
var Pt = {}, $s = {}, lr = {}, Fs = {};
Object.defineProperty(Fs, "__esModule", { value: true });
Fs.default = void 0;
function cy() {
  var e = this, t = 0, r = { "@@iterator": function() {
    return r;
  }, next: function() {
    if (t < e.length) {
      var o = e[t];
      return t = t + 1, { done: false, value: o };
    } else return { done: true };
  } };
  return r;
}
var dy = cy;
Fs.default = dy;
Object.defineProperty(lr, "__esModule", { value: true });
lr.default = fy;
var py = hy(Fs);
function hy(e) {
  return e && e.__esModule ? e : { default: e };
}
function Hl(e) {
  "@babel/helpers - typeof";
  return Hl = typeof Symbol == "function" && typeof Symbol.iterator == "symbol" ? function(t) {
    return typeof t;
  } : function(t) {
    return t && typeof Symbol == "function" && t.constructor === Symbol && t !== Symbol.prototype ? "symbol" : typeof t;
  }, Hl(e);
}
function fy(e, t) {
  return typeof Symbol == "function" && Hl(Symbol.iterator) === "symbol" && Object.defineProperty(e, Symbol.iterator, { value: py.default.bind(t) }), e;
}
var zt = {}, Bs = {};
Object.defineProperty(Bs, "__esModule", { value: true });
Bs.default = void 0;
var my = { relatedConcepts: [{ module: "HTML", concept: { name: "abbr" } }], type: "structure" }, vy = my;
Bs.default = vy;
var Vs = {};
Object.defineProperty(Vs, "__esModule", { value: true });
Vs.default = void 0;
var by = { relatedConcepts: [{ module: "ARIA", concept: { name: "alertdialog" } }], type: "window" }, yy = by;
Vs.default = yy;
var Hs = {};
Object.defineProperty(Hs, "__esModule", { value: true });
Hs.default = void 0;
var gy = { relatedConcepts: [{ module: "ARIA", concept: { name: "alert" } }], type: "structure" }, xy = gy;
Hs.default = xy;
var Us = {};
Object.defineProperty(Us, "__esModule", { value: true });
Us.default = void 0;
var _y = { relatedConcepts: [], type: "structure" }, Cy = _y;
Us.default = Cy;
var Gs = {};
Object.defineProperty(Gs, "__esModule", { value: true });
Gs.default = void 0;
var Ry = { relatedConcepts: [{ module: "ARIA", concept: { name: "application" } }], type: "window" }, Py = Ry;
Gs.default = Py;
var zs = {};
Object.defineProperty(zs, "__esModule", { value: true });
zs.default = void 0;
var wy = { relatedConcepts: [{ module: "ARIA", concept: { name: "article" } }, { module: "HTML", concept: { name: "article" } }], type: "structure" }, Ty = wy;
zs.default = Ty;
var Ws = {};
Object.defineProperty(Ws, "__esModule", { value: true });
Ws.default = void 0;
var Sy = { relatedConcepts: [{ module: "HTML", concept: { name: "audio" } }], type: "widget" }, Ay = Sy;
Ws.default = Ay;
var Ks = {};
Object.defineProperty(Ks, "__esModule", { value: true });
Ks.default = void 0;
var Ey = { relatedConcepts: [{ module: "ARIA", concept: { name: "banner" } }], type: "structure" }, ky = Ey;
Ks.default = ky;
var Xs = {};
Object.defineProperty(Xs, "__esModule", { value: true });
Xs.default = void 0;
var Iy = { relatedConcepts: [{ module: "HTML", concept: { name: "blockquote" } }], type: "structure" }, qy = Iy;
Xs.default = qy;
var Js = {};
Object.defineProperty(Js, "__esModule", { value: true });
Js.default = void 0;
var My = { relatedConcepts: [{ module: "ARIA", concept: { attributes: [{ name: "aria-busy", value: "true" }] } }], type: "widget" }, Ly = My;
Js.default = Ly;
var Qs = {};
Object.defineProperty(Qs, "__esModule", { value: true });
Qs.default = void 0;
var Oy = { relatedConcepts: [{ module: "ARIA", concept: { name: "button" } }, { module: "HTML", concept: { name: "button" } }], type: "widget" }, Ny = Oy;
Qs.default = Ny;
var Ys = {};
Object.defineProperty(Ys, "__esModule", { value: true });
Ys.default = void 0;
var Dy = { relatedConcepts: [{ module: "HTML", concept: { name: "canvas" } }], type: "widget" }, jy = Dy;
Ys.default = jy;
var Zs = {};
Object.defineProperty(Zs, "__esModule", { value: true });
Zs.default = void 0;
var $y = { relatedConcepts: [{ module: "HTML", concept: { name: "caption" } }], type: "structure" }, Fy = $y;
Zs.default = Fy;
var en = {};
Object.defineProperty(en, "__esModule", { value: true });
en.default = void 0;
var By = { relatedConcepts: [{ module: "ARIA", concept: { name: "cell" } }, { module: "ARIA", concept: { name: "gridcell" } }, { module: "HTML", concept: { name: "td" } }], type: "widget" }, Vy = By;
en.default = Vy;
var tn = {};
Object.defineProperty(tn, "__esModule", { value: true });
tn.default = void 0;
var Hy = { relatedConcepts: [{ module: "ARIA", concept: { name: "checkbox" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "checkbox" }] } }], type: "widget" }, Uy = Hy;
tn.default = Uy;
var rn = {};
Object.defineProperty(rn, "__esModule", { value: true });
rn.default = void 0;
var Gy = { relatedConcepts: [{ module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "color" }] } }], type: "widget" }, zy = Gy;
rn.default = zy;
var an = {};
Object.defineProperty(an, "__esModule", { value: true });
an.default = void 0;
var Wy = { relatedConcepts: [{ module: "ARIA", concept: { name: "columnheader" } }, { module: "HTML", concept: { name: "th" } }], type: "widget" }, Ky = Wy;
an.default = Ky;
var sn = {};
Object.defineProperty(sn, "__esModule", { value: true });
sn.default = void 0;
var Xy = { relatedConcepts: [], type: "structure" }, Jy = Xy;
sn.default = Jy;
var nn = {};
Object.defineProperty(nn, "__esModule", { value: true });
nn.default = void 0;
var Qy = { relatedConcepts: [{ module: "ARIA", concept: { name: "combobox" } }, { module: "HTML", concept: { name: "select" } }], type: "widget" }, Yy = Qy;
nn.default = Yy;
var on = {};
Object.defineProperty(on, "__esModule", { value: true });
on.default = void 0;
var Zy = { relatedConcepts: [{ module: "ARIA", concept: { name: "complementary" } }], type: "structure" }, e1 = Zy;
on.default = e1;
var ln = {};
Object.defineProperty(ln, "__esModule", { value: true });
ln.default = void 0;
var t1 = { relatedConcepts: [{ module: "ARIA", concept: { name: "structureinfo" } }], type: "structure" }, r1 = t1;
ln.default = r1;
var un = {};
Object.defineProperty(un, "__esModule", { value: true });
un.default = void 0;
var a1 = { relatedConcepts: [{ module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "date" }] } }], type: "widget" }, i1 = a1;
un.default = i1;
var cn = {};
Object.defineProperty(cn, "__esModule", { value: true });
cn.default = void 0;
var s1 = { relatedConcepts: [{ module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "datetime" }] } }], type: "widget" }, n1 = s1;
cn.default = n1;
var dn = {};
Object.defineProperty(dn, "__esModule", { value: true });
dn.default = void 0;
var o1 = { relatedConcepts: [{ module: "HTML", concept: { name: "dfn" } }], type: "structure" }, l1 = o1;
dn.default = l1;
var pn = {};
Object.defineProperty(pn, "__esModule", { value: true });
pn.default = void 0;
var u1 = { relatedConcepts: [{ module: "HTML", concept: { name: "dd" } }], type: "structure" }, c1 = u1;
pn.default = c1;
var hn = {};
Object.defineProperty(hn, "__esModule", { value: true });
hn.default = void 0;
var d1 = { relatedConcepts: [{ module: "HTML", concept: { name: "dl" } }], type: "structure" }, p1 = d1;
hn.default = p1;
var fn = {};
Object.defineProperty(fn, "__esModule", { value: true });
fn.default = void 0;
var h1 = { relatedConcepts: [{ module: "HTML", concept: { name: "dt" } }], type: "structure" }, f1 = h1;
fn.default = f1;
var mn = {};
Object.defineProperty(mn, "__esModule", { value: true });
mn.default = void 0;
var m1 = { relatedConcepts: [{ module: "HTML", concept: { name: "details" } }], type: "structure" }, v1 = m1;
mn.default = v1;
var vn = {};
Object.defineProperty(vn, "__esModule", { value: true });
vn.default = void 0;
var b1 = { relatedConcepts: [{ module: "ARIA", concept: { name: "dialog" } }, { module: "HTML", concept: { name: "dialog" } }], type: "window" }, y1 = b1;
vn.default = y1;
var bn = {};
Object.defineProperty(bn, "__esModule", { value: true });
bn.default = void 0;
var g1 = { relatedConcepts: [{ module: "ARIA", concept: { name: "directory" } }, { module: "HTML", concept: { name: "dir" } }], type: "structure" }, x1 = g1;
bn.default = x1;
var yn = {};
Object.defineProperty(yn, "__esModule", { value: true });
yn.default = void 0;
var _1 = { relatedConcepts: [{ module: "HTML", concept: { constraints: ["scoped to a details element"], name: "summary" } }], type: "widget" }, C1 = _1;
yn.default = C1;
var gn = {};
Object.defineProperty(gn, "__esModule", { value: true });
gn.default = void 0;
var R1 = { relatedConcepts: [{ module: "HTML", concept: { name: "div" } }], type: "generic" }, P1 = R1;
gn.default = P1;
var xn = {};
Object.defineProperty(xn, "__esModule", { value: true });
xn.default = void 0;
var w1 = { relatedConcepts: [{ module: "ARIA", concept: { name: "document" } }], type: "structure" }, T1 = w1;
xn.default = T1;
var _n = {};
Object.defineProperty(_n, "__esModule", { value: true });
_n.default = void 0;
var S1 = { relatedConcepts: [{ module: "HTML", concept: { name: "embed" } }], type: "widget" }, A1 = S1;
_n.default = A1;
var Cn = {};
Object.defineProperty(Cn, "__esModule", { value: true });
Cn.default = void 0;
var E1 = { relatedConcepts: [{ module: "ARIA", concept: { name: "feed" } }], type: "structure" }, k1 = E1;
Cn.default = k1;
var Rn = {};
Object.defineProperty(Rn, "__esModule", { value: true });
Rn.default = void 0;
var I1 = { relatedConcepts: [{ module: "HTML", concept: { name: "figcaption" } }], type: "structure" }, q1 = I1;
Rn.default = q1;
var Pn = {};
Object.defineProperty(Pn, "__esModule", { value: true });
Pn.default = void 0;
var M1 = { relatedConcepts: [{ module: "ARIA", concept: { name: "figure" } }, { module: "HTML", concept: { name: "figure" } }], type: "structure" }, L1 = M1;
Pn.default = L1;
var wn = {};
Object.defineProperty(wn, "__esModule", { value: true });
wn.default = void 0;
var O1 = { relatedConcepts: [{ module: "HTML", concept: { name: "footer" } }], type: "structure" }, N1 = O1;
wn.default = N1;
var Tn = {};
Object.defineProperty(Tn, "__esModule", { value: true });
Tn.default = void 0;
var D1 = { relatedConcepts: [{ module: "ARIA", concept: { name: "form" } }, { module: "HTML", concept: { name: "form" } }], type: "structure" }, j1 = D1;
Tn.default = j1;
var Sn = {};
Object.defineProperty(Sn, "__esModule", { value: true });
Sn.default = void 0;
var $1 = { relatedConcepts: [{ module: "ARIA", concept: { name: "grid" } }], type: "widget" }, F1 = $1;
Sn.default = F1;
var An = {};
Object.defineProperty(An, "__esModule", { value: true });
An.default = void 0;
var B1 = { relatedConcepts: [{ module: "ARIA", concept: { name: "group" } }], type: "structure" }, V1 = B1;
An.default = V1;
var En = {};
Object.defineProperty(En, "__esModule", { value: true });
En.default = void 0;
var H1 = { relatedConcepts: [{ module: "ARIA", concept: { name: "heading" } }, { module: "HTML", concept: { name: "h1" } }, { module: "HTML", concept: { name: "h2" } }, { module: "HTML", concept: { name: "h3" } }, { module: "HTML", concept: { name: "h4" } }, { module: "HTML", concept: { name: "h5" } }, { module: "HTML", concept: { name: "h6" } }], type: "structure" }, U1 = H1;
En.default = U1;
var kn = {};
Object.defineProperty(kn, "__esModule", { value: true });
kn.default = void 0;
var G1 = { relatedConcepts: [], type: "window" }, z1 = G1;
kn.default = z1;
var In = {};
Object.defineProperty(In, "__esModule", { value: true });
In.default = void 0;
var W1 = { relatedConcepts: [{ module: "HTML", concept: { name: "iframe" } }], type: "window" }, K1 = W1;
In.default = K1;
var qn = {};
Object.defineProperty(qn, "__esModule", { value: true });
qn.default = void 0;
var X1 = { relatedConcepts: [], type: "structure" }, J1 = X1;
qn.default = J1;
var Mn = {};
Object.defineProperty(Mn, "__esModule", { value: true });
Mn.default = void 0;
var Q1 = { relatedConcepts: [], type: "widget" }, Y1 = Q1;
Mn.default = Y1;
var Ln = {};
Object.defineProperty(Ln, "__esModule", { value: true });
Ln.default = void 0;
var Z1 = { relatedConcepts: [{ module: "HTML", concept: { name: "img", attributes: [{ name: "usemap" }] } }], type: "structure" }, eg = Z1;
Ln.default = eg;
var On = {};
Object.defineProperty(On, "__esModule", { value: true });
On.default = void 0;
var tg = { relatedConcepts: [{ module: "ARIA", concept: { name: "img" } }, { module: "HTML", concept: { name: "img" } }], type: "structure" }, rg = tg;
On.default = rg;
var Nn = {};
Object.defineProperty(Nn, "__esModule", { value: true });
Nn.default = void 0;
var ag = { relatedConcepts: [{ module: "HTML", concept: { name: "input" } }], type: "widget" }, ig = ag;
Nn.default = ig;
var Dn = {};
Object.defineProperty(Dn, "__esModule", { value: true });
Dn.default = void 0;
var sg = { relatedConcepts: [{ module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "time" }] } }], type: "widget" }, ng = sg;
Dn.default = ng;
var jn = {};
Object.defineProperty(jn, "__esModule", { value: true });
jn.default = void 0;
var og = { relatedConcepts: [{ module: "HTML", concept: { name: "label" } }], type: "structure" }, lg = og;
jn.default = lg;
var $n = {};
Object.defineProperty($n, "__esModule", { value: true });
$n.default = void 0;
var ug = { relatedConcepts: [{ module: "HTML", concept: { name: "legend" } }], type: "structure" }, cg = ug;
$n.default = cg;
var Fn = {};
Object.defineProperty(Fn, "__esModule", { value: true });
Fn.default = void 0;
var dg = { relatedConcepts: [{ module: "HTML", concept: { name: "br" } }], type: "structure" }, pg = dg;
Fn.default = pg;
var Bn = {};
Object.defineProperty(Bn, "__esModule", { value: true });
Bn.default = void 0;
var hg = { relatedConcepts: [{ module: "ARIA", concept: { name: "link" } }, { module: "HTML", concept: { name: "a", attributes: [{ name: "href" }] } }], type: "widget" }, fg = hg;
Bn.default = fg;
var Vn = {};
Object.defineProperty(Vn, "__esModule", { value: true });
Vn.default = void 0;
var mg = { relatedConcepts: [{ module: "ARIA", concept: { name: "option" } }, { module: "HTML", concept: { name: "option" } }], type: "widget" }, vg = mg;
Vn.default = vg;
var Hn = {};
Object.defineProperty(Hn, "__esModule", { value: true });
Hn.default = void 0;
var bg = { relatedConcepts: [{ module: "ARIA", concept: { name: "listbox" } }, { module: "HTML", concept: { name: "datalist" } }, { module: "HTML", concept: { name: "select" } }], type: "widget" }, yg = bg;
Hn.default = yg;
var Un = {};
Object.defineProperty(Un, "__esModule", { value: true });
Un.default = void 0;
var gg = { relatedConcepts: [{ module: "ARIA", concept: { name: "listitem" } }, { module: "HTML", concept: { name: "li" } }], type: "structure" }, xg = gg;
Un.default = xg;
var Gn = {};
Object.defineProperty(Gn, "__esModule", { value: true });
Gn.default = void 0;
var _g = { relatedConcepts: [], type: "structure" }, Cg = _g;
Gn.default = Cg;
var zn = {};
Object.defineProperty(zn, "__esModule", { value: true });
zn.default = void 0;
var Rg = { relatedConcepts: [{ module: "ARIA", concept: { name: "list" } }, { module: "HTML", concept: { name: "ul" } }, { module: "HTML", concept: { name: "ol" } }], type: "structure" }, Pg = Rg;
zn.default = Pg;
var Wn = {};
Object.defineProperty(Wn, "__esModule", { value: true });
Wn.default = void 0;
var wg = { relatedConcepts: [{ module: "ARIA", concept: { name: "log" } }], type: "structure" }, Tg = wg;
Wn.default = Tg;
var Kn = {};
Object.defineProperty(Kn, "__esModule", { value: true });
Kn.default = void 0;
var Sg = { relatedConcepts: [{ module: "ARIA", concept: { name: "main" } }, { module: "HTML", concept: { name: "main" } }], type: "structure" }, Ag = Sg;
Kn.default = Ag;
var Xn = {};
Object.defineProperty(Xn, "__esModule", { value: true });
Xn.default = void 0;
var Eg = { relatedConcepts: [{ module: "HTML", concept: { name: "mark" } }], type: "structure" }, kg = Eg;
Xn.default = kg;
var Jn = {};
Object.defineProperty(Jn, "__esModule", { value: true });
Jn.default = void 0;
var Ig = { relatedConcepts: [{ module: "ARIA", concept: { name: "marquee" } }, { module: "HTML", concept: { name: "marquee" } }], type: "structure" }, qg = Ig;
Jn.default = qg;
var Qn = {};
Object.defineProperty(Qn, "__esModule", { value: true });
Qn.default = void 0;
var Mg = { relatedConcepts: [{ module: "ARIA", concept: { name: "math" } }], type: "structure" }, Lg = Mg;
Qn.default = Lg;
var Yn = {};
Object.defineProperty(Yn, "__esModule", { value: true });
Yn.default = void 0;
var Og = { relatedConcepts: [{ module: "ARIA", concept: { name: "menubar" } }], type: "structure" }, Ng = Og;
Yn.default = Ng;
var Zn = {};
Object.defineProperty(Zn, "__esModule", { value: true });
Zn.default = void 0;
var Dg = { relatedConcepts: [], type: "widget" }, jg = Dg;
Zn.default = jg;
var eo = {};
Object.defineProperty(eo, "__esModule", { value: true });
eo.default = void 0;
var $g = { relatedConcepts: [{ module: "ARIA", concept: { name: "menuitem" } }, { module: "HTML", concept: { name: "menuitem" } }], type: "widget" }, Fg = $g;
eo.default = Fg;
var to = {};
Object.defineProperty(to, "__esModule", { value: true });
to.default = void 0;
var Bg = { relatedConcepts: [{ module: "ARIA", concept: { name: "menuitemcheckbox" } }], type: "widget" }, Vg = Bg;
to.default = Vg;
var ro = {};
Object.defineProperty(ro, "__esModule", { value: true });
ro.default = void 0;
var Hg = { relatedConcepts: [{ module: "ARIA", concept: { name: "menuitemradio" } }], type: "widget" }, Ug = Hg;
ro.default = Ug;
var ao = {};
Object.defineProperty(ao, "__esModule", { value: true });
ao.default = void 0;
var Gg = { relatedConcepts: [], type: "widget" }, zg = Gg;
ao.default = zg;
var io = {};
Object.defineProperty(io, "__esModule", { value: true });
io.default = void 0;
var Wg = { relatedConcepts: [], type: "widget" }, Kg = Wg;
io.default = Kg;
var so = {};
Object.defineProperty(so, "__esModule", { value: true });
so.default = void 0;
var Xg = { relatedConcepts: [{ module: "ARIA", concept: { name: "menu" } }, { module: "HTML", concept: { name: "menu" } }], type: "structure" }, Jg = Xg;
so.default = Jg;
var no = {};
Object.defineProperty(no, "__esModule", { value: true });
no.default = void 0;
var Qg = { relatedConcepts: [{ module: "HTML", concept: { name: "meter" } }], type: "structure" }, Yg = Qg;
no.default = Yg;
var oo = {};
Object.defineProperty(oo, "__esModule", { value: true });
oo.default = void 0;
var Zg = { relatedConcepts: [{ module: "ARIA", concept: { name: "navigation" } }, { module: "HTML", concept: { name: "nav" } }], type: "structure" }, ex = Zg;
oo.default = ex;
var lo = {};
Object.defineProperty(lo, "__esModule", { value: true });
lo.default = void 0;
var tx = { relatedConcepts: [{ module: "ARIA", concept: { name: "none" } }], type: "structure" }, rx = tx;
lo.default = rx;
var uo = {};
Object.defineProperty(uo, "__esModule", { value: true });
uo.default = void 0;
var ax = { relatedConcepts: [{ module: "ARIA", concept: { name: "note" } }], type: "structure" }, ix = ax;
uo.default = ix;
var co = {};
Object.defineProperty(co, "__esModule", { value: true });
co.default = void 0;
var sx = { relatedConcepts: [], type: "structure" }, nx = sx;
co.default = nx;
var po = {};
Object.defineProperty(po, "__esModule", { value: true });
po.default = void 0;
var ox = { relatedConcepts: [{ module: "HTML", concept: { name: "p" } }], type: "structure" }, lx = ox;
po.default = lx;
var ho = {};
Object.defineProperty(ho, "__esModule", { value: true });
ho.default = void 0;
var ux = { relatedConcepts: [], type: "widget" }, cx = ux;
ho.default = cx;
var fo = {};
Object.defineProperty(fo, "__esModule", { value: true });
fo.default = void 0;
var dx = { relatedConcepts: [{ module: "HTML", concept: { name: "pre" } }], type: "structure" }, px = dx;
fo.default = px;
var mo = {};
Object.defineProperty(mo, "__esModule", { value: true });
mo.default = void 0;
var hx = { relatedConcepts: [{ module: "ARIA", concept: { name: "presentation" } }], type: "structure" }, fx = hx;
mo.default = fx;
var vo = {};
Object.defineProperty(vo, "__esModule", { value: true });
vo.default = void 0;
var mx = { relatedConcepts: [{ module: "ARIA", concept: { name: "progressbar" } }, { module: "HTML", concept: { name: "progress" } }], type: "structure" }, vx = mx;
vo.default = vx;
var bo = {};
Object.defineProperty(bo, "__esModule", { value: true });
bo.default = void 0;
var bx = { relatedConcepts: [{ module: "ARIA", concept: { name: "radio" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "radio" }] } }], type: "widget" }, yx = bx;
bo.default = yx;
var yo = {};
Object.defineProperty(yo, "__esModule", { value: true });
yo.default = void 0;
var gx = { relatedConcepts: [{ module: "ARIA", concept: { name: "radiogroup" } }], type: "structure" }, xx = gx;
yo.default = xx;
var go = {};
Object.defineProperty(go, "__esModule", { value: true });
go.default = void 0;
var _x = { relatedConcepts: [{ module: "ARIA", concept: { name: "region" } }], type: "structure" }, Cx = _x;
go.default = Cx;
var xo = {};
Object.defineProperty(xo, "__esModule", { value: true });
xo.default = void 0;
var Rx = { relatedConcepts: [], type: "structure" }, Px = Rx;
xo.default = Px;
var _o = {};
Object.defineProperty(_o, "__esModule", { value: true });
_o.default = void 0;
var wx = { relatedConcepts: [{ module: "ARIA", concept: { name: "rowheader" } }, { module: "HTML", concept: { name: "th", attributes: [{ name: "scope", value: "row" }] } }], type: "widget" }, Tx = wx;
_o.default = Tx;
var Co = {};
Object.defineProperty(Co, "__esModule", { value: true });
Co.default = void 0;
var Sx = { relatedConcepts: [{ module: "ARIA", concept: { name: "row" } }, { module: "HTML", concept: { name: "tr" } }], type: "structure" }, Ax = Sx;
Co.default = Ax;
var Ro = {};
Object.defineProperty(Ro, "__esModule", { value: true });
Ro.default = void 0;
var Ex = { relatedConcepts: [{ module: "HTML", concept: { name: "ruby" } }], type: "structure" }, kx = Ex;
Ro.default = kx;
var Po = {};
Object.defineProperty(Po, "__esModule", { value: true });
Po.default = void 0;
var Ix = { relatedConcepts: [], type: "structure" }, qx = Ix;
Po.default = qx;
var wo = {};
Object.defineProperty(wo, "__esModule", { value: true });
wo.default = void 0;
var Mx = { relatedConcepts: [], type: "structure" }, Lx = Mx;
wo.default = Lx;
var To = {};
Object.defineProperty(To, "__esModule", { value: true });
To.default = void 0;
var Ox = { relatedConcepts: [{ module: "ARIA", concept: { name: "scrollbar" } }], type: "widget" }, Nx = Ox;
To.default = Nx;
var So = {};
Object.defineProperty(So, "__esModule", { value: true });
So.default = void 0;
var Dx = { relatedConcepts: [], type: "structure" }, jx = Dx;
So.default = jx;
var Ao = {};
Object.defineProperty(Ao, "__esModule", { value: true });
Ao.default = void 0;
var $x = { relatedConcepts: [{ module: "ARIA", concept: { name: "search" } }], type: "structure" }, Fx = $x;
Ao.default = Fx;
var Eo = {};
Object.defineProperty(Eo, "__esModule", { value: true });
Eo.default = void 0;
var Bx = { relatedConcepts: [{ module: "ARIA", concept: { name: "searchbox" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "search" }] } }], type: "widget" }, Vx = Bx;
Eo.default = Vx;
var ko = {};
Object.defineProperty(ko, "__esModule", { value: true });
ko.default = void 0;
var Hx = { relatedConcepts: [{ module: "ARIA", concept: { name: "slider" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "range" }] } }], type: "widget" }, Ux = Hx;
ko.default = Ux;
var Io = {};
Object.defineProperty(Io, "__esModule", { value: true });
Io.default = void 0;
var Gx = { relatedConcepts: [], type: "structure" }, zx = Gx;
Io.default = zx;
var qo = {};
Object.defineProperty(qo, "__esModule", { value: true });
qo.default = void 0;
var Wx = { relatedConcepts: [{ module: "ARIA", concept: { name: "spinbutton" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "number" }] } }], type: "widget" }, Kx = Wx;
qo.default = Kx;
var Mo = {};
Object.defineProperty(Mo, "__esModule", { value: true });
Mo.default = void 0;
var Xx = { relatedConcepts: [], type: "structure" }, Jx = Xx;
Mo.default = Jx;
var Lo = {};
Object.defineProperty(Lo, "__esModule", { value: true });
Lo.default = void 0;
var Qx = { relatedConcepts: [{ module: "ARIA", concept: { name: "separator" } }], type: "widget" }, Yx = Qx;
Lo.default = Yx;
var Oo = {};
Object.defineProperty(Oo, "__esModule", { value: true });
Oo.default = void 0;
var Zx = { relatedConcepts: [], type: "structure" }, e_ = Zx;
Oo.default = e_;
var No = {};
Object.defineProperty(No, "__esModule", { value: true });
No.default = void 0;
var t_ = { relatedConcepts: [{ module: "ARIA", concept: { name: "status" } }], type: "structure" }, r_ = t_;
No.default = r_;
var Do = {};
Object.defineProperty(Do, "__esModule", { value: true });
Do.default = void 0;
var a_ = { relatedConcepts: [], type: "structure" }, i_ = a_;
Do.default = i_;
var jo = {};
Object.defineProperty(jo, "__esModule", { value: true });
jo.default = void 0;
var s_ = { relatedConcepts: [{ module: "ARIA", concept: { name: "switch" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "checkbox" }] } }], type: "widget" }, n_ = s_;
jo.default = n_;
var $o = {};
Object.defineProperty($o, "__esModule", { value: true });
$o.default = void 0;
var o_ = { relatedConcepts: [{ module: "ARIA", concept: { name: "tablist" } }], type: "structure" }, l_ = o_;
$o.default = l_;
var Fo = {};
Object.defineProperty(Fo, "__esModule", { value: true });
Fo.default = void 0;
var u_ = { relatedConcepts: [{ module: "ARIA", concept: { name: "tab" } }], type: "widget" }, c_ = u_;
Fo.default = c_;
var Bo = {};
Object.defineProperty(Bo, "__esModule", { value: true });
Bo.default = void 0;
var d_ = { relatedConcepts: [], type: "structure" }, p_ = d_;
Bo.default = p_;
var Vo = {};
Object.defineProperty(Vo, "__esModule", { value: true });
Vo.default = void 0;
var h_ = { relatedConcepts: [{ module: "ARIA", concept: { name: "table" } }, { module: "HTML", concept: { name: "table" } }], type: "structure" }, f_ = h_;
Vo.default = f_;
var Ho = {};
Object.defineProperty(Ho, "__esModule", { value: true });
Ho.default = void 0;
var m_ = { relatedConcepts: [{ module: "ARIA", concept: { name: "tablist" } }], type: "structure" }, v_ = m_;
Ho.default = v_;
var Uo = {};
Object.defineProperty(Uo, "__esModule", { value: true });
Uo.default = void 0;
var b_ = { relatedConcepts: [{ module: "ARIA", concept: { name: "tabpanel" } }], type: "structure" }, y_ = b_;
Uo.default = y_;
var Go = {};
Object.defineProperty(Go, "__esModule", { value: true });
Go.default = void 0;
var g_ = { relatedConcepts: [{ module: "ARIA", concept: { name: "term" } }], type: "structure" }, x_ = g_;
Go.default = x_;
var zo = {};
Object.defineProperty(zo, "__esModule", { value: true });
zo.default = void 0;
var __ = { relatedConcepts: [{ module: "ARIA", concept: { attributes: [{ name: "aria-multiline", value: "true" }], name: "textbox" } }, { module: "HTML", concept: { name: "textarea" } }], type: "widget" }, C_ = __;
zo.default = C_;
var Wo = {};
Object.defineProperty(Wo, "__esModule", { value: true });
Wo.default = void 0;
var R_ = { relatedConcepts: [{ module: "ARIA", concept: { name: "textbox" } }, { module: "HTML", concept: { name: "input" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "text" }] } }], type: "widget" }, P_ = R_;
Wo.default = P_;
var Ko = {};
Object.defineProperty(Ko, "__esModule", { value: true });
Ko.default = void 0;
var w_ = { relatedConcepts: [{ module: "HTML", concept: { name: "time" } }], type: "structure" }, T_ = w_;
Ko.default = T_;
var Xo = {};
Object.defineProperty(Xo, "__esModule", { value: true });
Xo.default = void 0;
var S_ = { relatedConcepts: [{ module: "ARIA", concept: { name: "timer" } }], type: "structure" }, A_ = S_;
Xo.default = A_;
var Jo = {};
Object.defineProperty(Jo, "__esModule", { value: true });
Jo.default = void 0;
var E_ = { relatedConcepts: [{ module: "ARIA", concept: { attributes: [{ name: "aria-pressed" }] } }], type: "widget" }, k_ = E_;
Jo.default = k_;
var Qo = {};
Object.defineProperty(Qo, "__esModule", { value: true });
Qo.default = void 0;
var I_ = { relatedConcepts: [{ module: "ARIA", concept: { name: "toolbar" } }], type: "structure" }, q_ = I_;
Qo.default = q_;
var Yo = {};
Object.defineProperty(Yo, "__esModule", { value: true });
Yo.default = void 0;
var M_ = { relatedConcepts: [{ module: "ARIA", concept: { name: "tree" } }], type: "widget" }, L_ = M_;
Yo.default = L_;
var Zo = {};
Object.defineProperty(Zo, "__esModule", { value: true });
Zo.default = void 0;
var O_ = { relatedConcepts: [{ module: "ARIA", concept: { name: "treegrid" } }], type: "widget" }, N_ = O_;
Zo.default = N_;
var el = {};
Object.defineProperty(el, "__esModule", { value: true });
el.default = void 0;
var D_ = { relatedConcepts: [{ module: "ARIA", concept: { name: "treeitem" } }], type: "widget" }, j_ = D_;
el.default = j_;
var tl = {};
Object.defineProperty(tl, "__esModule", { value: true });
tl.default = void 0;
var $_ = { relatedConcepts: [{ module: "ARIA", concept: { name: "tooltip" } }], type: "structure" }, F_ = $_;
tl.default = F_;
var rl = {};
Object.defineProperty(rl, "__esModule", { value: true });
rl.default = void 0;
var B_ = { relatedConcepts: [{ module: "HTML", concept: { name: "video" } }], type: "widget" }, V_ = B_;
rl.default = V_;
var al = {};
Object.defineProperty(al, "__esModule", { value: true });
al.default = void 0;
var H_ = { relatedConcepts: [], type: "structure" }, U_ = H_;
al.default = U_;
var il = {};
Object.defineProperty(il, "__esModule", { value: true });
il.default = void 0;
var G_ = { relatedConcepts: [], type: "window" }, z_ = G_;
il.default = z_;
Object.defineProperty(zt, "__esModule", { value: true });
zt.default = void 0;
var W_ = I(lr), K_ = I(Bs), X_ = I(Vs), J_ = I(Hs), Q_ = I(Us), Y_ = I(Gs), Z_ = I(zs), e8 = I(Ws), t8 = I(Ks), r8 = I(Xs), a8 = I(Js), i8 = I(Qs), s8 = I(Ys), n8 = I(Zs), o8 = I(en), l8 = I(tn), u8 = I(rn), c8 = I(an), d8 = I(sn), p8 = I(nn), h8 = I(on), f8 = I(ln), m8 = I(un), v8 = I(cn), b8 = I(dn), y8 = I(pn), g8 = I(hn), x8 = I(fn), _8 = I(mn), C8 = I(vn), R8 = I(bn), P8 = I(yn), w8 = I(gn), T8 = I(xn), S8 = I(_n), A8 = I(Cn), E8 = I(Rn), k8 = I(Pn), I8 = I(wn), q8 = I(Tn), M8 = I(Sn), L8 = I(An), O8 = I(En), N8 = I(kn), D8 = I(In), j8 = I(qn), $8 = I(Mn), F8 = I(Ln), B8 = I(On), V8 = I(Nn), H8 = I(Dn), U8 = I(jn), G8 = I($n), z8 = I(Fn), W8 = I(Bn), K8 = I(Vn), X8 = I(Hn), J8 = I(Un), Q8 = I(Gn), Y8 = I(zn), Z8 = I(Wn), eC = I(Kn), tC = I(Xn), rC = I(Jn), aC = I(Qn), iC = I(Yn), sC = I(Zn), nC = I(eo), oC = I(to), lC = I(ro), uC = I(ao), cC = I(io), dC = I(so), pC = I(no), hC = I(oo), fC = I(lo), mC = I(uo), vC = I(co), bC = I(po), yC = I(ho), gC = I(fo), xC = I(mo), _C = I(vo), CC = I(bo), RC = I(yo), PC = I(go), wC = I(xo), TC = I(_o), SC = I(Co), AC = I(Ro), EC = I(Po), kC = I(wo), IC = I(To), qC = I(So), MC = I(Ao), LC = I(Eo), OC = I(ko), NC = I(Io), DC = I(qo), jC = I(Mo), $C = I(Lo), FC = I(Oo), BC = I(No), VC = I(Do), HC = I(jo), UC = I($o), GC = I(Fo), zC = I(Bo), WC = I(Vo), KC = I(Ho), XC = I(Uo), JC = I(Go), QC = I(zo), YC = I(Wo), ZC = I(Ko), eR = I(Xo), tR = I(Jo), rR = I(Qo), aR = I(Yo), iR = I(Zo), sR = I(el), nR = I(tl), oR = I(rl), lR = I(al), uR = I(il);
function I(e) {
  return e && e.__esModule ? e : { default: e };
}
function Al(e, t) {
  return hR(e) || pR(e, t) || dR(e, t) || cR();
}
function cR() {
  throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
}
function dR(e, t) {
  if (e) {
    if (typeof e == "string") return Gu(e, t);
    var r = Object.prototype.toString.call(e).slice(8, -1);
    if (r === "Object" && e.constructor && (r = e.constructor.name), r === "Map" || r === "Set") return Array.from(e);
    if (r === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(r)) return Gu(e, t);
  }
}
function Gu(e, t) {
  (t == null || t > e.length) && (t = e.length);
  for (var r = 0, s = new Array(t); r < t; r++) s[r] = e[r];
  return s;
}
function pR(e, t) {
  var r = e == null ? null : typeof Symbol < "u" && e[Symbol.iterator] || e["@@iterator"];
  if (r != null) {
    var s = [], o = true, p = false, h, v;
    try {
      for (r = r.call(e); !(o = (h = r.next()).done) && (s.push(h.value), !(t && s.length === t)); o = true) ;
    } catch (m) {
      p = true, v = m;
    } finally {
      try {
        !o && r.return != null && r.return();
      } finally {
        if (p) throw v;
      }
    }
    return s;
  }
}
function hR(e) {
  if (Array.isArray(e)) return e;
}
var Xt = [["AbbrRole", K_.default], ["AlertDialogRole", X_.default], ["AlertRole", J_.default], ["AnnotationRole", Q_.default], ["ApplicationRole", Y_.default], ["ArticleRole", Z_.default], ["AudioRole", e8.default], ["BannerRole", t8.default], ["BlockquoteRole", r8.default], ["BusyIndicatorRole", a8.default], ["ButtonRole", i8.default], ["CanvasRole", s8.default], ["CaptionRole", n8.default], ["CellRole", o8.default], ["CheckBoxRole", l8.default], ["ColorWellRole", u8.default], ["ColumnHeaderRole", c8.default], ["ColumnRole", d8.default], ["ComboBoxRole", p8.default], ["ComplementaryRole", h8.default], ["ContentInfoRole", f8.default], ["DateRole", m8.default], ["DateTimeRole", v8.default], ["DefinitionRole", b8.default], ["DescriptionListDetailRole", y8.default], ["DescriptionListRole", g8.default], ["DescriptionListTermRole", x8.default], ["DetailsRole", _8.default], ["DialogRole", C8.default], ["DirectoryRole", R8.default], ["DisclosureTriangleRole", P8.default], ["DivRole", w8.default], ["DocumentRole", T8.default], ["EmbeddedObjectRole", S8.default], ["FeedRole", A8.default], ["FigcaptionRole", E8.default], ["FigureRole", k8.default], ["FooterRole", I8.default], ["FormRole", q8.default], ["GridRole", M8.default], ["GroupRole", L8.default], ["HeadingRole", O8.default], ["IframePresentationalRole", N8.default], ["IframeRole", D8.default], ["IgnoredRole", j8.default], ["ImageMapLinkRole", $8.default], ["ImageMapRole", F8.default], ["ImageRole", B8.default], ["InlineTextBoxRole", V8.default], ["InputTimeRole", H8.default], ["LabelRole", U8.default], ["LegendRole", G8.default], ["LineBreakRole", z8.default], ["LinkRole", W8.default], ["ListBoxOptionRole", K8.default], ["ListBoxRole", X8.default], ["ListItemRole", J8.default], ["ListMarkerRole", Q8.default], ["ListRole", Y8.default], ["LogRole", Z8.default], ["MainRole", eC.default], ["MarkRole", tC.default], ["MarqueeRole", rC.default], ["MathRole", aC.default], ["MenuBarRole", iC.default], ["MenuButtonRole", sC.default], ["MenuItemRole", nC.default], ["MenuItemCheckBoxRole", oC.default], ["MenuItemRadioRole", lC.default], ["MenuListOptionRole", uC.default], ["MenuListPopupRole", cC.default], ["MenuRole", dC.default], ["MeterRole", pC.default], ["NavigationRole", hC.default], ["NoneRole", fC.default], ["NoteRole", mC.default], ["OutlineRole", vC.default], ["ParagraphRole", bC.default], ["PopUpButtonRole", yC.default], ["PreRole", gC.default], ["PresentationalRole", xC.default], ["ProgressIndicatorRole", _C.default], ["RadioButtonRole", CC.default], ["RadioGroupRole", RC.default], ["RegionRole", PC.default], ["RootWebAreaRole", wC.default], ["RowHeaderRole", TC.default], ["RowRole", SC.default], ["RubyRole", AC.default], ["RulerRole", EC.default], ["ScrollAreaRole", kC.default], ["ScrollBarRole", IC.default], ["SeamlessWebAreaRole", qC.default], ["SearchRole", MC.default], ["SearchBoxRole", LC.default], ["SliderRole", OC.default], ["SliderThumbRole", NC.default], ["SpinButtonRole", DC.default], ["SpinButtonPartRole", jC.default], ["SplitterRole", $C.default], ["StaticTextRole", FC.default], ["StatusRole", BC.default], ["SVGRootRole", VC.default], ["SwitchRole", HC.default], ["TabGroupRole", UC.default], ["TabRole", GC.default], ["TableHeaderContainerRole", zC.default], ["TableRole", WC.default], ["TabListRole", KC.default], ["TabPanelRole", XC.default], ["TermRole", JC.default], ["TextAreaRole", QC.default], ["TextFieldRole", YC.default], ["TimeRole", ZC.default], ["TimerRole", eR.default], ["ToggleButtonRole", tR.default], ["ToolbarRole", rR.default], ["TreeRole", aR.default], ["TreeGridRole", iR.default], ["TreeItemRole", sR.default], ["UserInterfaceTooltipRole", nR.default], ["VideoRole", oR.default], ["WebAreaRole", lR.default], ["WindowRole", uR.default]], Ul = { entries: function() {
  return Xt;
}, forEach: function(t) {
  for (var r = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, s = 0, o = Xt; s < o.length; s++) {
    var p = Al(o[s], 2), h = p[0], v = p[1];
    t.call(r, v, h, Xt);
  }
}, get: function(t) {
  var r = Xt.find(function(s) {
    return s[0] === t;
  });
  return r && r[1];
}, has: function(t) {
  return !!Ul.get(t);
}, keys: function() {
  return Xt.map(function(t) {
    var r = Al(t, 1), s = r[0];
    return s;
  });
}, values: function() {
  return Xt.map(function(t) {
    var r = Al(t, 2), s = r[1];
    return s;
  });
} }, fR = (0, W_.default)(Ul, Ul.entries());
zt.default = fR;
Object.defineProperty($s, "__esModule", { value: true });
$s.default = void 0;
var mR = ad(lr), vR = ad(zt);
function ad(e) {
  return e && e.__esModule ? e : { default: e };
}
function mr(e, t) {
  return gR(e) || yR(e, t) || id(e, t) || bR();
}
function bR() {
  throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
}
function yR(e, t) {
  var r = e == null ? null : typeof Symbol < "u" && e[Symbol.iterator] || e["@@iterator"];
  if (r != null) {
    var s = [], o = true, p = false, h, v;
    try {
      for (r = r.call(e); !(o = (h = r.next()).done) && (s.push(h.value), !(t && s.length === t)); o = true) ;
    } catch (m) {
      p = true, v = m;
    } finally {
      try {
        !o && r.return != null && r.return();
      } finally {
        if (p) throw v;
      }
    }
    return s;
  }
}
function gR(e) {
  if (Array.isArray(e)) return e;
}
function xR(e, t) {
  var r = typeof Symbol < "u" && e[Symbol.iterator] || e["@@iterator"];
  if (!r) {
    if (Array.isArray(e) || (r = id(e)) || t) {
      r && (e = r);
      var s = 0, o = function() {
      };
      return { s: o, n: function() {
        return s >= e.length ? { done: true } : { done: false, value: e[s++] };
      }, e: function(l) {
        throw l;
      }, f: o };
    }
    throw new TypeError(`Invalid attempt to iterate non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  var p = true, h = false, v;
  return { s: function() {
    r = r.call(e);
  }, n: function() {
    var l = r.next();
    return p = l.done, l;
  }, e: function(l) {
    h = true, v = l;
  }, f: function() {
    try {
      !p && r.return != null && r.return();
    } finally {
      if (h) throw v;
    }
  } };
}
function id(e, t) {
  if (e) {
    if (typeof e == "string") return zu(e, t);
    var r = Object.prototype.toString.call(e).slice(8, -1);
    if (r === "Object" && e.constructor && (r = e.constructor.name), r === "Map" || r === "Set") return Array.from(e);
    if (r === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(r)) return zu(e, t);
  }
}
function zu(e, t) {
  (t == null || t > e.length) && (t = e.length);
  for (var r = 0, s = new Array(t); r < t; r++) s[r] = e[r];
  return s;
}
var vt = [], Ar = xR(vR.default.entries()), Wu;
try {
  var _R = function() {
    var t = mr(Wu.value, 2), r = t[0], s = t[1], o = s.relatedConcepts;
    Array.isArray(o) && o.forEach(function(p) {
      if (p.module === "HTML") {
        var h = p.concept;
        if (h) {
          var v = vt.findIndex(function(m) {
            var l = mr(m, 1), w = l[0];
            return w === r;
          });
          v === -1 && (vt.push([r, []]), v = vt.length - 1), vt[v][1].push(h);
        }
      }
    });
  };
  for (Ar.s(); !(Wu = Ar.n()).done; ) _R();
} catch (e) {
  Ar.e(e);
} finally {
  Ar.f();
}
var Gl = { entries: function() {
  return vt;
}, forEach: function(t) {
  for (var r = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, s = 0, o = vt; s < o.length; s++) {
    var p = mr(o[s], 2), h = p[0], v = p[1];
    t.call(r, v, h, vt);
  }
}, get: function(t) {
  var r = vt.find(function(s) {
    return s[0] === t;
  });
  return r && r[1];
}, has: function(t) {
  return !!Gl.get(t);
}, keys: function() {
  return vt.map(function(t) {
    var r = mr(t, 1), s = r[0];
    return s;
  });
}, values: function() {
  return vt.map(function(t) {
    var r = mr(t, 2), s = r[1];
    return s;
  });
} }, CR = (0, mR.default)(Gl, Gl.entries());
$s.default = CR;
var sl = {};
Object.defineProperty(sl, "__esModule", { value: true });
sl.default = void 0;
var RR = sd(lr), PR = sd(zt);
function sd(e) {
  return e && e.__esModule ? e : { default: e };
}
function vr(e, t) {
  return SR(e) || TR(e, t) || nd(e, t) || wR();
}
function wR() {
  throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
}
function TR(e, t) {
  var r = e == null ? null : typeof Symbol < "u" && e[Symbol.iterator] || e["@@iterator"];
  if (r != null) {
    var s = [], o = true, p = false, h, v;
    try {
      for (r = r.call(e); !(o = (h = r.next()).done) && (s.push(h.value), !(t && s.length === t)); o = true) ;
    } catch (m) {
      p = true, v = m;
    } finally {
      try {
        !o && r.return != null && r.return();
      } finally {
        if (p) throw v;
      }
    }
    return s;
  }
}
function SR(e) {
  if (Array.isArray(e)) return e;
}
function AR(e, t) {
  var r = typeof Symbol < "u" && e[Symbol.iterator] || e["@@iterator"];
  if (!r) {
    if (Array.isArray(e) || (r = nd(e)) || t) {
      r && (e = r);
      var s = 0, o = function() {
      };
      return { s: o, n: function() {
        return s >= e.length ? { done: true } : { done: false, value: e[s++] };
      }, e: function(l) {
        throw l;
      }, f: o };
    }
    throw new TypeError(`Invalid attempt to iterate non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  var p = true, h = false, v;
  return { s: function() {
    r = r.call(e);
  }, n: function() {
    var l = r.next();
    return p = l.done, l;
  }, e: function(l) {
    h = true, v = l;
  }, f: function() {
    try {
      !p && r.return != null && r.return();
    } finally {
      if (h) throw v;
    }
  } };
}
function nd(e, t) {
  if (e) {
    if (typeof e == "string") return Ku(e, t);
    var r = Object.prototype.toString.call(e).slice(8, -1);
    if (r === "Object" && e.constructor && (r = e.constructor.name), r === "Map" || r === "Set") return Array.from(e);
    if (r === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(r)) return Ku(e, t);
  }
}
function Ku(e, t) {
  (t == null || t > e.length) && (t = e.length);
  for (var r = 0, s = new Array(t); r < t; r++) s[r] = e[r];
  return s;
}
var bt = [], Er = AR(PR.default.entries()), Xu;
try {
  var ER = function() {
    var t = vr(Xu.value, 2), r = t[0], s = t[1], o = s.relatedConcepts;
    Array.isArray(o) && o.forEach(function(p) {
      if (p.module === "ARIA") {
        var h = p.concept;
        if (h) {
          var v = bt.findIndex(function(m) {
            var l = vr(m, 1), w = l[0];
            return w === r;
          });
          v === -1 && (bt.push([r, []]), v = bt.length - 1), bt[v][1].push(h);
        }
      }
    });
  };
  for (Er.s(); !(Xu = Er.n()).done; ) ER();
} catch (e) {
  Er.e(e);
} finally {
  Er.f();
}
var zl = { entries: function() {
  return bt;
}, forEach: function(t) {
  for (var r = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, s = 0, o = bt; s < o.length; s++) {
    var p = vr(o[s], 2), h = p[0], v = p[1];
    t.call(r, v, h, bt);
  }
}, get: function(t) {
  var r = bt.find(function(s) {
    return s[0] === t;
  });
  return r && r[1];
}, has: function(t) {
  return !!zl.get(t);
}, keys: function() {
  return bt.map(function(t) {
    var r = vr(t, 1), s = r[0];
    return s;
  });
}, values: function() {
  return bt.map(function(t) {
    var r = vr(t, 2), s = r[1];
    return s;
  });
} }, kR = (0, RR.default)(zl, zl.entries());
sl.default = kR;
var nl = {};
Object.defineProperty(nl, "__esModule", { value: true });
nl.default = void 0;
var IR = od(zt), qR = od(lr);
function od(e) {
  return e && e.__esModule ? e : { default: e };
}
function Nr(e, t) {
  return OR(e) || LR(e, t) || ld(e, t) || MR();
}
function MR() {
  throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
}
function LR(e, t) {
  var r = e == null ? null : typeof Symbol < "u" && e[Symbol.iterator] || e["@@iterator"];
  if (r != null) {
    var s = [], o = true, p = false, h, v;
    try {
      for (r = r.call(e); !(o = (h = r.next()).done) && (s.push(h.value), !(t && s.length === t)); o = true) ;
    } catch (m) {
      p = true, v = m;
    } finally {
      try {
        !o && r.return != null && r.return();
      } finally {
        if (p) throw v;
      }
    }
    return s;
  }
}
function OR(e) {
  if (Array.isArray(e)) return e;
}
function NR(e, t) {
  var r = typeof Symbol < "u" && e[Symbol.iterator] || e["@@iterator"];
  if (!r) {
    if (Array.isArray(e) || (r = ld(e)) || t) {
      r && (e = r);
      var s = 0, o = function() {
      };
      return { s: o, n: function() {
        return s >= e.length ? { done: true } : { done: false, value: e[s++] };
      }, e: function(l) {
        throw l;
      }, f: o };
    }
    throw new TypeError(`Invalid attempt to iterate non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  var p = true, h = false, v;
  return { s: function() {
    r = r.call(e);
  }, n: function() {
    var l = r.next();
    return p = l.done, l;
  }, e: function(l) {
    h = true, v = l;
  }, f: function() {
    try {
      !p && r.return != null && r.return();
    } finally {
      if (h) throw v;
    }
  } };
}
function ld(e, t) {
  if (e) {
    if (typeof e == "string") return Ju(e, t);
    var r = Object.prototype.toString.call(e).slice(8, -1);
    if (r === "Object" && e.constructor && (r = e.constructor.name), r === "Map" || r === "Set") return Array.from(e);
    if (r === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(r)) return Ju(e, t);
  }
}
function Ju(e, t) {
  (t == null || t > e.length) && (t = e.length);
  for (var r = 0, s = new Array(t); r < t; r++) s[r] = e[r];
  return s;
}
var ut = [], kr = NR(IR.default.entries()), Qu;
try {
  var DR = function() {
    var t = Nr(Qu.value, 2), r = t[0], s = t[1], o = s.relatedConcepts;
    Array.isArray(o) && o.forEach(function(p) {
      if (p.module === "HTML") {
        var h = p.concept;
        if (h != null) {
          for (var v = JSON.stringify(h), m, l = 0; l < ut.length; l++) {
            var w = ut[l][0];
            if (JSON.stringify(w) === v) {
              m = ut[l][1];
              break;
            }
          }
          Array.isArray(m) || (m = []);
          var U = m.findIndex(function(J) {
            return J === r;
          });
          U === -1 && m.push(r), l < ut.length ? ut.splice(l, 1, [h, m]) : ut.push([h, m]);
        }
      }
    });
  };
  for (kr.s(); !(Qu = kr.n()).done; ) DR();
} catch (e) {
  kr.e(e);
} finally {
  kr.f();
}
function jR(e, t) {
  if (e === void 0 && t !== void 0 || e !== void 0 && t === void 0) return false;
  if (e !== void 0 && t !== void 0) {
    if (e.length != t.length) return false;
    for (var r = 0; r < e.length; r++) if (t[r].name !== e[r].name || t[r].value !== e[r].value) return false;
  }
  return true;
}
var Wl = { entries: function() {
  return ut;
}, forEach: function(t) {
  for (var r = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, s = 0, o = ut; s < o.length; s++) {
    var p = Nr(o[s], 2), h = p[0], v = p[1];
    t.call(r, v, h, ut);
  }
}, get: function(t) {
  var r = ut.find(function(s) {
    return t.name === s[0].name && jR(t.attributes, s[0].attributes);
  });
  return r && r[1];
}, has: function(t) {
  return !!Wl.get(t);
}, keys: function() {
  return ut.map(function(t) {
    var r = Nr(t, 1), s = r[0];
    return s;
  });
}, values: function() {
  return ut.map(function(t) {
    var r = Nr(t, 2), s = r[1];
    return s;
  });
} }, $R = (0, qR.default)(Wl, Wl.entries());
nl.default = $R;
Object.defineProperty(Pt, "__esModule", { value: true });
var uu = Pt.elementAXObjects = gr = Pt.AXObjects = Pt.AXObjectRoles = Pt.AXObjectElements = void 0, FR = ol($s), BR = ol(sl), VR = ol(zt), HR = ol(nl);
function ol(e) {
  return e && e.__esModule ? e : { default: e };
}
var UR = FR.default;
Pt.AXObjectElements = UR;
var GR = BR.default;
Pt.AXObjectRoles = GR;
var zR = VR.default, gr = Pt.AXObjects = zR, WR = HR.default;
uu = Pt.elementAXObjects = WR;
const ud = js.keys(), KR = ud.filter((e) => {
  var _a3;
  return (_a3 = js.get(e)) == null ? void 0 : _a3.abstract;
}), cd = ud.filter((e) => !KR.includes(e)), dd = cd.filter((e) => {
  const t = js.get(e);
  return !["toolbar", "tabpanel", "generic", "cell"].includes(e) && !(t == null ? void 0 : t.superClass.some((r) => r.includes("widget")));
}).concat("progressbar"), XR = cd.filter((e) => !dd.includes(e) && e !== "generic");
Ds.entries().forEach(([e, t]) => {
  [...t].every((r) => r !== "generic" && dd.includes(r));
});
Ds.entries().forEach(([e, t]) => {
  [...t].every((r) => XR.includes(r));
});
const JR = [...gr.keys()].filter((e) => gr.get(e).type === "widget"), QR = [...gr.keys()].filter((e) => ["windows", "structure"].includes(gr.get(e).type));
uu.entries().forEach(([e, t]) => {
  [...t].every((r) => JR.includes(r));
});
uu.entries().forEach(([e, t]) => {
  [...t].every((r) => QR.includes(r));
});
var Kl = { exports: {} };
(function(e, t) {
  (function(r, s) {
    s(t);
  })(rr, function(r) {
    const p = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/", h = new Uint8Array(64), v = new Uint8Array(128);
    for (let _ = 0; _ < p.length; _++) {
      const S = p.charCodeAt(_);
      h[_] = S, v[S] = _;
    }
    function m(_, S) {
      let R = 0, P = 0, A = 0;
      do {
        const a = _.next();
        A = v[a], R |= (A & 31) << P, P += 5;
      } while (A & 32);
      const c = R & 1;
      return R >>>= 1, c && (R = -2147483648 | -R), S + R;
    }
    function l(_, S, R) {
      let P = S - R;
      P = P < 0 ? -P << 1 | 1 : P << 1;
      do {
        let A = P & 31;
        P >>>= 5, P > 0 && (A |= 32), _.write(h[A]);
      } while (P > 0);
      return S;
    }
    function w(_, S) {
      return _.pos >= S ? false : _.peek() !== 44;
    }
    const U = 1024 * 16, J = typeof TextDecoder < "u" ? new TextDecoder() : typeof Buffer < "u" ? { decode(_) {
      return Buffer.from(_.buffer, _.byteOffset, _.byteLength).toString();
    } } : { decode(_) {
      let S = "";
      for (let R = 0; R < _.length; R++) S += String.fromCharCode(_[R]);
      return S;
    } };
    class z {
      constructor() {
        this.pos = 0, this.out = "", this.buffer = new Uint8Array(U);
      }
      write(S) {
        const { buffer: R } = this;
        R[this.pos++] = S, this.pos === U && (this.out += J.decode(R), this.pos = 0);
      }
      flush() {
        const { buffer: S, out: R, pos: P } = this;
        return P > 0 ? R + J.decode(S.subarray(0, P)) : R;
      }
    }
    class K {
      constructor(S) {
        this.pos = 0, this.buffer = S;
      }
      next() {
        return this.buffer.charCodeAt(this.pos++);
      }
      peek() {
        return this.buffer.charCodeAt(this.pos);
      }
      indexOf(S) {
        const { buffer: R, pos: P } = this, A = R.indexOf(S, P);
        return A === -1 ? R.length : A;
      }
    }
    const Z = [];
    function B(_) {
      const { length: S } = _, R = new K(_), P = [], A = [];
      let c = 0;
      for (; R.pos < S; R.pos++) {
        c = m(R, c);
        const a = m(R, 0);
        if (!w(R, S)) {
          const y = A.pop();
          y[2] = c, y[3] = a;
          continue;
        }
        const i = m(R, 0), f = m(R, 0) & 1 ? [c, a, 0, 0, i, m(R, 0)] : [c, a, 0, 0, i];
        let b = Z;
        if (w(R, S)) {
          b = [];
          do {
            const y = m(R, 0);
            b.push(y);
          } while (w(R, S));
        }
        f.vars = b, P.push(f), A.push(f);
      }
      return P;
    }
    function $(_) {
      const S = new z();
      for (let R = 0; R < _.length; ) R = L(_, R, S, [0]);
      return S.flush();
    }
    function L(_, S, R, P) {
      const A = _[S], { 0: c, 1: a, 2: i, 3: n, 4: u, vars: f } = A;
      S > 0 && R.write(44), P[0] = l(R, c, P[0]), l(R, a, 0), l(R, u, 0);
      const b = A.length === 6 ? 1 : 0;
      l(R, b, 0), A.length === 6 && l(R, A[5], 0);
      for (const y of f) l(R, y, 0);
      for (S++; S < _.length; ) {
        const y = _[S], { 0: g, 1: C } = y;
        if (g > i || g === i && C >= n) break;
        S = L(_, S, R, P);
      }
      return R.write(44), P[0] = l(R, i, P[0]), l(R, n, 0), S;
    }
    function k(_) {
      const { length: S } = _, R = new K(_), P = [], A = [];
      let c = 0, a = 0, i = 0, n = 0, u = 0, f = 0, b = 0, y = 0;
      do {
        const g = R.indexOf(";");
        let C = 0;
        for (; R.pos < g; R.pos++) {
          if (C = m(R, C), !w(R, g)) {
            const be = A.pop();
            be[2] = c, be[3] = C;
            continue;
          }
          const q = m(R, 0), V = q & 1, Q = q & 2, F = q & 4;
          let ce = null, ue = Z, de;
          if (V) {
            const be = m(R, a);
            i = m(R, a === be ? i : 0), a = be, de = [c, C, 0, 0, be, i];
          } else de = [c, C, 0, 0];
          if (de.isScope = !!F, Q) {
            const be = n, ke = u;
            n = m(R, n);
            const Oe = be === n;
            u = m(R, Oe ? u : 0), f = m(R, Oe && ke === u ? f : 0), ce = [n, u, f];
          }
          if (de.callsite = ce, w(R, g)) {
            ue = [];
            do {
              b = c, y = C;
              const be = m(R, 0);
              let ke;
              if (be < -1) {
                ke = [[m(R, 0)]];
                for (let Oe = -1; Oe > be; Oe--) {
                  const qe = b;
                  b = m(R, b), y = m(R, b === qe ? y : 0);
                  const He = m(R, 0);
                  ke.push([He, b, y]);
                }
              } else ke = [[be]];
              ue.push(ke);
            } while (w(R, g));
          }
          de.bindings = ue, P.push(de), A.push(de);
        }
        c++, R.pos = g + 1;
      } while (R.pos < S);
      return P;
    }
    function O(_) {
      if (_.length === 0) return "";
      const S = new z();
      for (let R = 0; R < _.length; ) R = H(_, R, S, [0, 0, 0, 0, 0, 0, 0]);
      return S.flush();
    }
    function H(_, S, R, P) {
      const A = _[S], { 0: c, 1: a, 2: i, 3: n, isScope: u, callsite: f, bindings: b } = A;
      P[0] < c ? (D(R, P[0], c), P[0] = c, P[1] = 0) : S > 0 && R.write(44), P[1] = l(R, A[1], P[1]);
      const y = (A.length === 6 ? 1 : 0) | (f ? 2 : 0) | (u ? 4 : 0);
      if (l(R, y, 0), A.length === 6) {
        const { 4: g, 5: C } = A;
        g !== P[2] && (P[3] = 0), P[2] = l(R, g, P[2]), P[3] = l(R, C, P[3]);
      }
      if (f) {
        const { 0: g, 1: C, 2: q } = A.callsite;
        g !== P[4] ? (P[5] = 0, P[6] = 0) : C !== P[5] && (P[6] = 0), P[4] = l(R, g, P[4]), P[5] = l(R, C, P[5]), P[6] = l(R, q, P[6]);
      }
      if (b) for (const g of b) {
        g.length > 1 && l(R, -g.length, 0);
        const C = g[0][0];
        l(R, C, 0);
        let q = c, V = a;
        for (let Q = 1; Q < g.length; Q++) {
          const F = g[Q];
          q = l(R, F[1], q), V = l(R, F[2], V), l(R, F[0], 0);
        }
      }
      for (S++; S < _.length; ) {
        const g = _[S], { 0: C, 1: q } = g;
        if (C > i || C === i && q >= n) break;
        S = H(_, S, R, P);
      }
      return P[0] < i ? (D(R, P[0], i), P[0] = i, P[1] = 0) : R.write(44), P[1] = l(R, n, P[1]), S;
    }
    function D(_, S, R) {
      do
        _.write(59);
      while (++S < R);
    }
    function ie(_) {
      const { length: S } = _, R = new K(_), P = [];
      let A = 0, c = 0, a = 0, i = 0, n = 0;
      do {
        const u = R.indexOf(";"), f = [];
        let b = true, y = 0;
        for (A = 0; R.pos < u; ) {
          let g;
          A = m(R, A), A < y && (b = false), y = A, w(R, u) ? (c = m(R, c), a = m(R, a), i = m(R, i), w(R, u) ? (n = m(R, n), g = [A, c, a, i, n]) : g = [A, c, a, i]) : g = [A], f.push(g), R.pos++;
        }
        b || se(f), P.push(f), R.pos = u + 1;
      } while (R.pos <= S);
      return P;
    }
    function se(_) {
      _.sort(fe);
    }
    function fe(_, S) {
      return _[0] - S[0];
    }
    function me(_) {
      const S = new z();
      let R = 0, P = 0, A = 0, c = 0;
      for (let a = 0; a < _.length; a++) {
        const i = _[a];
        if (a > 0 && S.write(59), i.length === 0) continue;
        let n = 0;
        for (let u = 0; u < i.length; u++) {
          const f = i[u];
          u > 0 && S.write(44), n = l(S, f[0], n), f.length !== 1 && (R = l(S, f[1], R), P = l(S, f[2], P), A = l(S, f[3], A), f.length !== 4 && (c = l(S, f[4], c)));
        }
      }
      return S.flush();
    }
    r.decode = ie, r.decodeGeneratedRanges = k, r.decodeOriginalScopes = B, r.encode = me, r.encodeGeneratedRanges = O, r.encodeOriginalScopes = $, Object.defineProperty(r, "__esModule", { value: true });
  });
})(Kl, Kl.exports);
var pd = Kl.exports, YR = { exports: {} }, Ir = { exports: {} }, El = { exports: {} }, Yu;
function ZR() {
  return Yu || (Yu = 1, function(e, t) {
    (function(r, s) {
      e.exports = s();
    })(rr, function() {
      const r = /^[\w+.-]+:\/\//, s = /^([\w+.-]+:)\/\/([^@/#?]*@)?([^:/#?]*)(:\d+)?(\/[^#?]*)?(\?[^#]*)?(#.*)?/, o = /^file:(?:\/\/((?![a-z]:)[^/#?]*)?)?(\/?[^#?]*)(\?[^#]*)?(#.*)?/i;
      function p(L) {
        return r.test(L);
      }
      function h(L) {
        return L.startsWith("//");
      }
      function v(L) {
        return L.startsWith("/");
      }
      function m(L) {
        return L.startsWith("file:");
      }
      function l(L) {
        return /^[.?#]/.test(L);
      }
      function w(L) {
        const k = s.exec(L);
        return J(k[1], k[2] || "", k[3], k[4] || "", k[5] || "/", k[6] || "", k[7] || "");
      }
      function U(L) {
        const k = o.exec(L), O = k[2];
        return J("file:", "", k[1] || "", "", v(O) ? O : "/" + O, k[3] || "", k[4] || "");
      }
      function J(L, k, O, H, D, ie, se) {
        return { scheme: L, user: k, host: O, port: H, path: D, query: ie, hash: se, type: 7 };
      }
      function z(L) {
        if (h(L)) {
          const O = w("http:" + L);
          return O.scheme = "", O.type = 6, O;
        }
        if (v(L)) {
          const O = w("http://foo.com" + L);
          return O.scheme = "", O.host = "", O.type = 5, O;
        }
        if (m(L)) return U(L);
        if (p(L)) return w(L);
        const k = w("http://foo.com/" + L);
        return k.scheme = "", k.host = "", k.type = L ? L.startsWith("?") ? 3 : L.startsWith("#") ? 2 : 4 : 1, k;
      }
      function K(L) {
        if (L.endsWith("/..")) return L;
        const k = L.lastIndexOf("/");
        return L.slice(0, k + 1);
      }
      function Z(L, k) {
        B(k, k.type), L.path === "/" ? L.path = k.path : L.path = K(k.path) + L.path;
      }
      function B(L, k) {
        const O = k <= 4, H = L.path.split("/");
        let D = 1, ie = 0, se = false;
        for (let me = 1; me < H.length; me++) {
          const _ = H[me];
          if (!_) {
            se = true;
            continue;
          }
          if (se = false, _ !== ".") {
            if (_ === "..") {
              ie ? (se = true, ie--, D--) : O && (H[D++] = _);
              continue;
            }
            H[D++] = _, ie++;
          }
        }
        let fe = "";
        for (let me = 1; me < D; me++) fe += "/" + H[me];
        (!fe || se && !fe.endsWith("/..")) && (fe += "/"), L.path = fe;
      }
      function $(L, k) {
        if (!L && !k) return "";
        const O = z(L);
        let H = O.type;
        if (k && H !== 7) {
          const ie = z(k), se = ie.type;
          switch (H) {
            case 1:
              O.hash = ie.hash;
            case 2:
              O.query = ie.query;
            case 3:
            case 4:
              Z(O, ie);
            case 5:
              O.user = ie.user, O.host = ie.host, O.port = ie.port;
            case 6:
              O.scheme = ie.scheme;
          }
          se > H && (H = se);
        }
        B(O, H);
        const D = O.query + O.hash;
        switch (H) {
          case 2:
          case 3:
            return D;
          case 4: {
            const ie = O.path.slice(1);
            return ie ? l(k || L) && !l(ie) ? "./" + ie + D : ie + D : D || ".";
          }
          case 5:
            return O.path + D;
          default:
            return O.scheme + "//" + O.user + O.host + O.port + O.path + D;
        }
      }
      return $;
    });
  }(El)), El.exports;
}
var Zu;
function hd() {
  return Zu || (Zu = 1, function(e, t) {
    (function(r, s) {
      s(t, pd, ZR());
    })(rr, function(r, s, o) {
      function p(T, N) {
        return N && !N.endsWith("/") && (N += "/"), o(T, N);
      }
      function h(T) {
        if (!T) return "";
        const N = T.lastIndexOf("/");
        return T.slice(0, N + 1);
      }
      const v = 0, m = 1, l = 2, w = 3, U = 4, J = 1, z = 2;
      function K(T, N) {
        const j = Z(T, 0);
        if (j === T.length) return T;
        N || (T = T.slice());
        for (let G = j; G < T.length; G = Z(T, G + 1)) T[G] = $(T[G], N);
        return T;
      }
      function Z(T, N) {
        for (let j = N; j < T.length; j++) if (!B(T[j])) return j;
        return T.length;
      }
      function B(T) {
        for (let N = 1; N < T.length; N++) if (T[N][v] < T[N - 1][v]) return false;
        return true;
      }
      function $(T, N) {
        return N || (T = T.slice()), T.sort(L);
      }
      function L(T, N) {
        return T[v] - N[v];
      }
      let k = false;
      function O(T, N, j, G) {
        for (; j <= G; ) {
          const le = j + (G - j >> 1), ae = T[le][v] - N;
          if (ae === 0) return k = true, le;
          ae < 0 ? j = le + 1 : G = le - 1;
        }
        return k = false, j - 1;
      }
      function H(T, N, j) {
        for (let G = j + 1; G < T.length && T[G][v] === N; j = G++) ;
        return j;
      }
      function D(T, N, j) {
        for (let G = j - 1; G >= 0 && T[G][v] === N; j = G--) ;
        return j;
      }
      function ie() {
        return { lastKey: -1, lastNeedle: -1, lastIndex: -1 };
      }
      function se(T, N, j, G) {
        const { lastKey: le, lastNeedle: ae, lastIndex: Re } = j;
        let Me = 0, xe = T.length - 1;
        if (G === le) {
          if (N === ae) return k = Re !== -1 && T[Re][v] === N, Re;
          N >= ae ? Me = Re === -1 ? 0 : Re : xe = Re;
        }
        return j.lastKey = G, j.lastNeedle = N, j.lastIndex = O(T, N, Me, xe);
      }
      function fe(T, N) {
        const j = N.map(_);
        for (let G = 0; G < T.length; G++) {
          const le = T[G];
          for (let ae = 0; ae < le.length; ae++) {
            const Re = le[ae];
            if (Re.length === 1) continue;
            const Me = Re[m], xe = Re[l], Ie = Re[w], ze = j[Me], Fe = ze[xe] || (ze[xe] = []), je = N[Me];
            let Ue = H(Fe, Ie, se(Fe, Ie, je, xe));
            je.lastIndex = ++Ue, me(Fe, Ue, [Ie, G, Re[v]]);
          }
        }
        return j;
      }
      function me(T, N, j) {
        for (let G = T.length; G > N; G--) T[G] = T[G - 1];
        T[N] = j;
      }
      function _() {
        return { __proto__: null };
      }
      const S = function(T, N) {
        const j = R(T);
        if (!("sections" in j)) return new b(j, N);
        const G = [], le = [], ae = [], Re = [], Me = [];
        P(j, N, G, le, ae, Re, Me, 0, 0, 1 / 0, 1 / 0);
        const xe = { version: 3, file: j.file, names: Re, sources: le, sourcesContent: ae, mappings: G, ignoreList: Me };
        return ke(xe);
      };
      function R(T) {
        return typeof T == "string" ? JSON.parse(T) : T;
      }
      function P(T, N, j, G, le, ae, Re, Me, xe, Ie, ze) {
        const { sections: Fe } = T;
        for (let je = 0; je < Fe.length; je++) {
          const { map: Ue, offset: X } = Fe[je];
          let Le = Ie, ge = ze;
          if (je + 1 < Fe.length) {
            const Qe = Fe[je + 1].offset;
            Le = Math.min(Ie, Me + Qe.line), Le === Ie ? ge = Math.min(ze, xe + Qe.column) : Le < Ie && (ge = xe + Qe.column);
          }
          A(Ue, N, j, G, le, ae, Re, Me + X.line, xe + X.column, Le, ge);
        }
      }
      function A(T, N, j, G, le, ae, Re, Me, xe, Ie, ze) {
        const Fe = R(T);
        if ("sections" in Fe) return P(...arguments);
        const je = new b(Fe, N), Ue = G.length, X = ae.length, Le = C(je), { resolvedSources: ge, sourcesContent: Qe, ignoreList: ht } = je;
        if (c(G, ge), c(ae, je.names), Qe) c(le, Qe);
        else for (let We = 0; We < ge.length; We++) le.push(null);
        if (ht) for (let We = 0; We < ht.length; We++) Re.push(ht[We] + Ue);
        for (let We = 0; We < Le.length; We++) {
          const Ge = Me + We;
          if (Ge > Ie) return;
          const Ye = a(j, Ge), ur = We === 0 ? xe : 0, du = Le[We];
          for (let ll = 0; ll < du.length; ll++) {
            const $t = du[ll], Tr = ur + $t[v];
            if (Ge === Ie && Tr >= ze) return;
            if ($t.length === 1) {
              Ye.push([Tr]);
              continue;
            }
            const pu = Ue + $t[m], hu = $t[l], fu = $t[w];
            Ye.push($t.length === 4 ? [Tr, pu, hu, fu] : [Tr, pu, hu, fu, X + $t[U]]);
          }
        }
      }
      function c(T, N) {
        for (let j = 0; j < N.length; j++) T.push(N[j]);
      }
      function a(T, N) {
        for (let j = T.length; j <= N; j++) T[j] = [];
        return T[N];
      }
      const i = "`line` must be greater than 0 (lines start at line 1)", n = "`column` must be greater than or equal to 0 (columns start at column 0)", u = -1, f = 1;
      class b {
        constructor(N, j) {
          const G = typeof N == "string";
          if (!G && N._decodedMemo) return N;
          const le = G ? JSON.parse(N) : N, { version: ae, file: Re, names: Me, sourceRoot: xe, sources: Ie, sourcesContent: ze } = le;
          this.version = ae, this.file = Re, this.names = Me || [], this.sourceRoot = xe, this.sources = Ie, this.sourcesContent = ze, this.ignoreList = le.ignoreList || le.x_google_ignoreList || void 0;
          const Fe = p(xe || "", h(j));
          this.resolvedSources = Ie.map((Ue) => p(Ue || "", Fe));
          const { mappings: je } = le;
          typeof je == "string" ? (this._encoded = je, this._decoded = void 0) : (this._encoded = void 0, this._decoded = K(je, G)), this._decodedMemo = ie(), this._bySources = void 0, this._bySourceMemos = void 0;
        }
      }
      function y(T) {
        return T;
      }
      function g(T) {
        var N, j;
        return (N = (j = T)._encoded) !== null && N !== void 0 ? N : j._encoded = s.encode(T._decoded);
      }
      function C(T) {
        var N;
        return (N = T)._decoded || (N._decoded = s.decode(T._encoded));
      }
      function q(T, N, j) {
        const G = C(T);
        if (N >= G.length) return null;
        const le = G[N], ae = Je(le, T._decodedMemo, N, j, f);
        return ae === -1 ? null : le[ae];
      }
      function V(T, N) {
        let { line: j, column: G, bias: le } = N;
        if (j--, j < 0) throw new Error(i);
        if (G < 0) throw new Error(n);
        const ae = C(T);
        if (j >= ae.length) return Ne(null, null, null, null);
        const Re = ae[j], Me = Je(Re, T._decodedMemo, j, G, le || f);
        if (Me === -1) return Ne(null, null, null, null);
        const xe = Re[Me];
        if (xe.length === 1) return Ne(null, null, null, null);
        const { names: Ie, resolvedSources: ze } = T;
        return Ne(ze[xe[m]], xe[l] + 1, xe[w], xe.length === 5 ? Ie[xe[U]] : null);
      }
      function Q(T, N) {
        const { source: j, line: G, column: le, bias: ae } = N;
        return jt(T, j, G, le, ae || f, false);
      }
      function F(T, N) {
        const { source: j, line: G, column: le, bias: ae } = N;
        return jt(T, j, G, le, ae || u, true);
      }
      function ce(T, N) {
        const j = C(T), { names: G, resolvedSources: le } = T;
        for (let ae = 0; ae < j.length; ae++) {
          const Re = j[ae];
          for (let Me = 0; Me < Re.length; Me++) {
            const xe = Re[Me], Ie = ae + 1, ze = xe[0];
            let Fe = null, je = null, Ue = null, X = null;
            xe.length !== 1 && (Fe = le[xe[1]], je = xe[2] + 1, Ue = xe[3]), xe.length === 5 && (X = G[xe[4]]), N({ generatedLine: Ie, generatedColumn: ze, source: Fe, originalLine: je, originalColumn: Ue, name: X });
          }
        }
      }
      function ue(T, N) {
        const { sources: j, resolvedSources: G } = T;
        let le = j.indexOf(N);
        return le === -1 && (le = G.indexOf(N)), le;
      }
      function de(T, N) {
        const { sourcesContent: j } = T;
        if (j == null) return null;
        const G = ue(T, N);
        return G === -1 ? null : j[G];
      }
      function be(T, N) {
        const { ignoreList: j } = T;
        if (j == null) return false;
        const G = ue(T, N);
        return G === -1 ? false : j.includes(G);
      }
      function ke(T, N) {
        const j = new b(He(T, []), N);
        return j._decoded = T.mappings, j;
      }
      function Oe(T) {
        return He(T, C(T));
      }
      function qe(T) {
        return He(T, g(T));
      }
      function He(T, N) {
        return { version: T.version, file: T.file, names: T.names, sourceRoot: T.sourceRoot, sources: T.sources, sourcesContent: T.sourcesContent, mappings: N, ignoreList: T.ignoreList || T.x_google_ignoreList };
      }
      function Ne(T, N, j, G) {
        return { source: T, line: N, column: j, name: G };
      }
      function it(T, N) {
        return { line: T, column: N };
      }
      function Je(T, N, j, G, le) {
        let ae = se(T, G, N, j);
        return k ? ae = (le === u ? H : D)(T, G, ae) : le === u && ae++, ae === -1 || ae === T.length ? -1 : ae;
      }
      function Dt(T, N, j, G, le) {
        let ae = Je(T, N, j, G, f);
        if (!k && le === u && ae++, ae === -1 || ae === T.length) return [];
        const Re = k ? G : T[ae][v];
        k || (ae = D(T, Re, ae));
        const Me = H(T, Re, ae), xe = [];
        for (; ae <= Me; ae++) {
          const Ie = T[ae];
          xe.push(it(Ie[J] + 1, Ie[z]));
        }
        return xe;
      }
      function jt(T, N, j, G, le, ae) {
        var Re;
        if (j--, j < 0) throw new Error(i);
        if (G < 0) throw new Error(n);
        const { sources: Me, resolvedSources: xe } = T;
        let Ie = Me.indexOf(N);
        if (Ie === -1 && (Ie = xe.indexOf(N)), Ie === -1) return ae ? [] : it(null, null);
        const Fe = ((Re = T)._bySources || (Re._bySources = fe(C(T), T._bySourceMemos = Me.map(ie))))[Ie][j];
        if (Fe == null) return ae ? [] : it(null, null);
        const je = T._bySourceMemos[Ie];
        if (ae) return Dt(Fe, je, j, G, le);
        const Ue = Je(Fe, je, j, G, le);
        if (Ue === -1) return it(null, null);
        const X = Fe[Ue];
        return it(X[J] + 1, X[z]);
      }
      r.AnyMap = S, r.GREATEST_LOWER_BOUND = f, r.LEAST_UPPER_BOUND = u, r.TraceMap = b, r.allGeneratedPositionsFor = F, r.decodedMap = Oe, r.decodedMappings = C, r.eachMapping = ce, r.encodedMap = qe, r.encodedMappings = g, r.generatedPositionFor = Q, r.isIgnored = be, r.originalPositionFor = V, r.presortedDecodedMap = ke, r.sourceContentFor = de, r.traceSegment = q;
    });
  }(Ir, Ir.exports)), Ir.exports;
}
var qr = { exports: {} }, Mr = { exports: {} }, ec;
function eP() {
  return ec || (ec = 1, function(e, t) {
    (function(r, s) {
      s(t);
    })(rr, function(r) {
      class s {
        constructor() {
          this._indexes = { __proto__: null }, this.array = [];
        }
      }
      function o(l) {
        return l;
      }
      function p(l, w) {
        return l._indexes[w];
      }
      function h(l, w) {
        const U = p(l, w);
        if (U !== void 0) return U;
        const { array: J, _indexes: z } = l, K = J.push(w);
        return z[w] = K - 1;
      }
      function v(l) {
        const { array: w, _indexes: U } = l;
        if (w.length === 0) return;
        const J = w.pop();
        U[J] = void 0;
      }
      function m(l, w) {
        const U = p(l, w);
        if (U === void 0) return;
        const { array: J, _indexes: z } = l;
        for (let K = U + 1; K < J.length; K++) {
          const Z = J[K];
          J[K - 1] = Z, z[Z]--;
        }
        z[w] = void 0, J.pop();
      }
      r.SetArray = s, r.get = p, r.pop = v, r.put = h, r.remove = m, Object.defineProperty(r, "__esModule", { value: true });
    });
  }(Mr, Mr.exports)), Mr.exports;
}
var tc;
function tP() {
  return tc || (tc = 1, function(e, t) {
    (function(r, s) {
      s(t, eP(), pd, hd());
    })(rr, function(r, s, o, p) {
      class J {
        constructor({ file: i, sourceRoot: n } = {}) {
          this._names = new s.SetArray(), this._sources = new s.SetArray(), this._sourcesContent = [], this._mappings = [], this.file = i, this.sourceRoot = n, this._ignoreList = new s.SetArray();
        }
      }
      function z(a) {
        return a;
      }
      function K(a, i, n, u, f, b, y, g) {
        return se(false, a, i, n, u, f, b, y, g);
      }
      function Z(a, i) {
        return c(false, a, i);
      }
      const B = (a, i, n, u, f, b, y, g) => se(true, a, i, n, u, f, b, y, g), $ = (a, i) => c(true, a, i);
      function L(a, i, n) {
        const { _sources: u, _sourcesContent: f } = a, b = s.put(u, i);
        f[b] = n;
      }
      function k(a, i, n = true) {
        const { _sources: u, _sourcesContent: f, _ignoreList: b } = a, y = s.put(u, i);
        y === f.length && (f[y] = null), n ? s.put(b, y) : s.remove(b, y);
      }
      function O(a) {
        const { _mappings: i, _sources: n, _sourcesContent: u, _names: f, _ignoreList: b } = a;
        return S(i), { version: 3, file: a.file || void 0, names: f.array, sourceRoot: a.sourceRoot || void 0, sources: n.array, sourcesContent: u, mappings: i, ignoreList: b.array };
      }
      function H(a) {
        const i = O(a);
        return Object.assign(Object.assign({}, i), { mappings: o.encode(i.mappings) });
      }
      function D(a) {
        const i = new p.TraceMap(a), n = new J({ file: i.file, sourceRoot: i.sourceRoot });
        return R(n._names, i.names), R(n._sources, i.sources), n._sourcesContent = i.sourcesContent || i.sources.map(() => null), n._mappings = p.decodedMappings(i), i.ignoreList && R(n._ignoreList, i.ignoreList), n;
      }
      function ie(a) {
        const i = [], { _mappings: n, _sources: u, _names: f } = a;
        for (let b = 0; b < n.length; b++) {
          const y = n[b];
          for (let g = 0; g < y.length; g++) {
            const C = y[g], q = { line: b + 1, column: C[0] };
            let V, Q, F;
            C.length !== 1 && (V = u.array[C[1]], Q = { line: C[2] + 1, column: C[3] }, C.length === 5 && (F = f.array[C[4]])), i.push({ generated: q, source: V, original: Q, name: F });
          }
        }
        return i;
      }
      function se(a, i, n, u, f, b, y, g, C) {
        const { _mappings: q, _sources: V, _sourcesContent: Q, _names: F } = i, ce = fe(q, n), ue = me(ce, u);
        if (!f) return a && P(ce, ue) ? void 0 : _(ce, ue, [u]);
        const de = s.put(V, f), be = g ? s.put(F, g) : -1;
        if (de === Q.length && (Q[de] = C ?? null), !(a && A(ce, ue, de, b, y, be))) return _(ce, ue, g ? [u, de, b, y, be] : [u, de, b, y]);
      }
      function fe(a, i) {
        for (let n = a.length; n <= i; n++) a[n] = [];
        return a[i];
      }
      function me(a, i) {
        let n = a.length;
        for (let u = n - 1; u >= 0; n = u--) {
          const f = a[u];
          if (i >= f[0]) break;
        }
        return n;
      }
      function _(a, i, n) {
        for (let u = a.length; u > i; u--) a[u] = a[u - 1];
        a[i] = n;
      }
      function S(a) {
        const { length: i } = a;
        let n = i;
        for (let u = n - 1; u >= 0 && !(a[u].length > 0); n = u, u--) ;
        n < i && (a.length = n);
      }
      function R(a, i) {
        for (let n = 0; n < i.length; n++) s.put(a, i[n]);
      }
      function P(a, i) {
        return i === 0 ? true : a[i - 1].length === 1;
      }
      function A(a, i, n, u, f, b) {
        if (i === 0) return false;
        const y = a[i - 1];
        return y.length === 1 ? false : n === y[1] && u === y[2] && f === y[3] && b === (y.length === 5 ? y[4] : -1);
      }
      function c(a, i, n) {
        const { generated: u, source: f, original: b, name: y, content: g } = n;
        return f ? se(a, i, u.line - 1, u.column, f, b.line - 1, b.column, y, g) : se(a, i, u.line - 1, u.column, null, null, null, null, null);
      }
      r.GenMapping = J, r.addMapping = Z, r.addSegment = K, r.allMappings = ie, r.fromMap = D, r.maybeAddMapping = $, r.maybeAddSegment = B, r.setIgnore = k, r.setSourceContent = L, r.toDecodedMap = O, r.toEncodedMap = H, Object.defineProperty(r, "__esModule", { value: true });
    });
  }(qr, qr.exports)), qr.exports;
}
(function(e, t) {
  (function(r, s) {
    e.exports = s(hd(), tP());
  })(rr, function(r, s) {
    const o = h("", -1, -1, "", null, false), p = [];
    function h($, L, k, O, H, D) {
      return { source: $, line: L, column: k, name: O, content: H, ignore: D };
    }
    function v($, L, k, O, H) {
      return { map: $, sources: L, source: k, content: O, ignore: H };
    }
    function m($, L) {
      return v($, L, "", null, false);
    }
    function l($, L, k) {
      return v(null, p, $, L, k);
    }
    function w($) {
      const L = new s.GenMapping({ file: $.map.file }), { sources: k, map: O } = $, H = O.names, D = r.decodedMappings(O);
      for (let ie = 0; ie < D.length; ie++) {
        const se = D[ie];
        for (let fe = 0; fe < se.length; fe++) {
          const me = se[fe], _ = me[0];
          let S = o;
          if (me.length !== 1) {
            const n = k[me[1]];
            if (S = U(n, me[2], me[3], me.length === 5 ? H[me[4]] : ""), S == null) continue;
          }
          const { column: R, line: P, name: A, content: c, source: a, ignore: i } = S;
          s.maybeAddSegment(L, ie, _, a, P, R, A), a && c != null && s.setSourceContent(L, a, c), i && s.setIgnore(L, a, true);
        }
      }
      return L;
    }
    function U($, L, k, O) {
      if (!$.map) return h($.source, L, k, O, $.content, $.ignore);
      const H = r.traceSegment($.map, L, k);
      return H == null ? null : H.length === 1 ? o : U($.sources[H[1]], H[2], H[3], H.length === 5 ? $.map.names[H[4]] : O);
    }
    function J($) {
      return Array.isArray($) ? $ : [$];
    }
    function z($, L) {
      const k = J($).map((D) => new r.TraceMap(D, "")), O = k.pop();
      for (let D = 0; D < k.length; D++) if (k[D].sources.length > 1) throw new Error(`Transformation map ${D} must have exactly one source file.
Did you specify these with the most recent transformation maps first?`);
      let H = K(O, L, "", 0);
      for (let D = k.length - 1; D >= 0; D--) H = m(k[D], [H]);
      return H;
    }
    function K($, L, k, O) {
      const { resolvedSources: H, sourcesContent: D, ignoreList: ie } = $, se = O + 1, fe = H.map((me, _) => {
        const S = { importer: k, depth: se, source: me || "", content: void 0, ignore: void 0 }, R = L(S.source, S), { source: P, content: A, ignore: c } = S;
        if (R) return K(new r.TraceMap(R, P), L, P, se);
        const a = A !== void 0 ? A : D ? D[_] : null, i = c !== void 0 ? c : ie ? ie.includes(_) : false;
        return l(P, a, i);
      });
      return m($, fe);
    }
    class Z {
      constructor(L, k) {
        const O = k.decodedMappings ? s.toDecodedMap(L) : s.toEncodedMap(L);
        this.version = O.version, this.file = O.file, this.mappings = O.mappings, this.names = O.names, this.ignoreList = O.ignoreList, this.sourceRoot = O.sourceRoot, this.sources = O.sources, k.excludeContent || (this.sourcesContent = O.sourcesContent);
      }
      toString() {
        return JSON.stringify(this);
      }
    }
    function B($, L, k) {
      const O = typeof k == "object" ? k : { excludeContent: !!k, decodedMappings: false }, H = z($, L);
      return new Z(w(H), O);
    }
    return B;
  });
})(YR);
const rP = { filename: br("(unknown)"), rootDir: br(typeof process < "u" ? (_a2 = process.cwd) == null ? void 0 : _a2.call(process) : typeof Deno < "u" ? Deno.cwd() : void 0), dev: mt(false), generate: Ht("client", (e, t) => e === "dom" || e === "ssr" ? (cu(ap), e === "dom" ? "client" : "server") : (e !== "client" && e !== "server" && e !== false && gt(`${t} must be "client", "server" or false`), e)), warningFilter: fd(() => true) };
ic({ ...rP, accessors: ac(Yd, mt(false)), css: Ht("external", (e) => ((e === true || e === false) && gt('The boolean options have been removed from the css option. Use "external" instead of false and "injected" instead of true'), e === "none" && gt('css: "none" is no longer a valid option. If this was crucial for you, please open an issue on GitHub with your use case.'), e !== "external" && e !== "injected" && gt('css should be either "external" (default, recommended) or "injected"'), e)), cssHash: fd(({ css: e, hash: t }) => `svelte-${t(e)}`), cssOutputFilename: br(void 0), customElement: mt(false), discloseVersion: mt(true), immutable: ac(Zd, mt(false)), legacy: Jt("The legacy option has been removed. If you are using this because of legacy.componentApi, use compatibility.componentApi instead"), compatibility: ic({ componentApi: sc([4, 5], 5) }), loopGuardTimeout: kl(rp), name: br(void 0), namespace: sc(["html", "mathml", "svg"]), modernAst: mt(false), outputFilename: br(void 0), preserveComments: mt(false), preserveWhitespace: mt(false), runes: mt(void 0), hmr: mt(false), sourcemap: Ht(void 0, (e) => e), enableSourcemap: kl(ep), hydratable: kl(tp), format: Jt('The format option has been removed in Svelte 4, the compiler only outputs ESM now. Remove "format" from your compiler options. If you did not set this yourself, bump the version of your bundler plugin (vite-plugin-svelte/rollup-plugin-svelte/svelte-loader)'), tag: Jt('The tag option has been removed in Svelte 5. Use `<svelte:options customElement="tag-name" />` inside the component instead. If that does not solve your use case, please open an issue on GitHub with details.'), sveltePath: Jt("The sveltePath option has been removed in Svelte 5. If this option was crucial for you, please open an issue on GitHub with your use case."), errorMode: Jt("The errorMode option has been removed. If you are using this through svelte-preprocess with TypeScript, use the https://www.typescriptlang.org/tsconfig#verbatimModuleSyntax setting instead"), varsReport: Jt("The vars option has been removed. If you are using this through svelte-preprocess with TypeScript, use the https://www.typescriptlang.org/tsconfig#verbatimModuleSyntax setting instead") });
function Jt(e) {
  return (t) => {
    t !== void 0 && ih(null, e);
  };
}
const rc = /* @__PURE__ */ new Set();
function cu(e) {
  rc.has(e) || (rc.add(e), e(null));
}
function kl(e) {
  return (t) => {
    t !== void 0 && cu(e);
  };
}
function ac(e, t) {
  return (r, s) => (r !== void 0 && cu(e), t(r, s));
}
function ic(e, t = false) {
  return (r, s) => {
    const o = {};
    (r && typeof r != "object" || Array.isArray(r)) && gt(`${s} should be an object`);
    for (const p in r) p in e || (t ? o[p] = r[p] : sh(null, `${s ? `${s}.${p}` : p}`));
    for (const p in e) {
      const h = e[p];
      o[p] = h(r && r[p], s ? `${s}.${p}` : p);
    }
    return o;
  };
}
function Ht(e, t) {
  return (r, s) => r === void 0 ? e : t(r, s);
}
function br(e, t = true) {
  return Ht(e, (r, s) => (typeof r != "string" && gt(`${s} should be a string, if specified`), !t && r === "" && gt(`${s} cannot be empty`), r));
}
function mt(e) {
  return Ht(e, (t, r) => (typeof t != "boolean" && gt(`${r} should be true or false, if specified`), t));
}
function sc(e, t = e[0]) {
  return Ht(t, (r, s) => {
    if (!e.includes(r)) {
      const o = e.length > 2 ? `${s} should be one of ${e.slice(0, -1).map((p) => `"${p}"`).join(", ")} or "${e[e.length - 1]}"` : `${s} should be either "${e[0]}" or "${e[1]}"`;
      gt(o);
    }
    return r;
  });
}
function fd(e) {
  return Ht(e, (t, r) => (typeof t != "function" && gt(`${r} should be a function, if specified`), t));
}
function gt(e) {
  ah(null, e);
}
var aP = Ae("<p> </p>"), iP = Ae('<p> </p> <!> <div class="flex gap-05"><!> <!></div>', 1), sP = Ae('<div class="err"> </div>'), nP = Ae('<div class="container svelte-b4h84n"><!></div> <!>', 1);
function oP(e, t) {
  At(t, true);
  let r = tr(), s = Ot(), o = ee(""), p = ee(false), h = ee(M([])), v = ee(M([])), m = ee(M([]));
  Xl(() => {
    l();
  }), wt(() => {
    w();
  }), wt(() => {
    E(m, M(x(h).map((k) => {
      var _a3;
      return { name: k.name, desc: k.desc, value: ((_a3 = x(v).find((H) => H.key === k.name)) == null ? void 0 : _a3.value) || "" };
    })));
  });
  async function l() {
    var _a3;
    let k = await Lt("/auth/v1/users/attr");
    k.body ? E(h, M(k.body.values.toSorted((O, H) => O.name.localeCompare(H.name)))) : E(o, M(((_a3 = k.error) == null ? void 0 : _a3.message) || "Error fetching attrs"));
  }
  async function w() {
    var _a3;
    let k = await Lt(`/auth/v1/users/${t.user.id}/attr`);
    k.body ? E(v, M(k.body.values)) : E(o, M(((_a3 = k.error) == null ? void 0 : _a3.message) || "Error fetching users attrs"));
  }
  async function U() {
    E(o, "");
    let k = x(v).map((D) => D.key), O = { values: x(m).filter((D) => D.value.trim() || k.includes(D.name)).map((D) => ({ key: D.name, value: D.value.trim() })) }, H = await Jl(`/auth/v1/users/${t.user.id}/attr`, O);
    H.error ? E(o, M(H.error.message)) : (E(p, true), setTimeout(() => {
      E(p, false);
    }, 3e3)), t.onSave();
  }
  var J = nP(), z = Ee(J), K = he(z);
  {
    var Z = (k) => {
      var O = aP(), H = he(O, true);
      pe(O), _e(() => ye(H, s.common.noEntries)), Y(k, O);
    }, B = (k) => {
      var O = iP(), H = Ee(O), D = he(H, true);
      pe(H);
      var ie = te(H, 2);
      jr(ie, 17, () => x(m), md, (S, R, P) => {
        const A = yt(() => x(R).desc || "JSON Value");
        ot(S, { autocomplete: "off", get label() {
          return x(R).name;
        }, get placeholder() {
          return x(A);
        }, get value() {
          return x(R).value;
        }, set value(c) {
          x(R).value = c;
        } });
      });
      var se = te(ie, 2), fe = he(se);
      Rt(fe, { onclick: U, children: (S, R) => {
        Ve();
        var P = $e();
        _e(() => ye(P, r.common.save)), Y(S, P);
      }, $$slots: { default: true } });
      var me = te(fe, 2);
      {
        var _ = (S) => {
          Wr(S, {});
        };
        Se(me, (S) => {
          x(p) && S(_);
        });
      }
      pe(se), _e(() => ye(D, s.users.descAttr)), Y(k, O);
    };
    Se(K, (k) => {
      x(m).length === 0 ? k(Z) : k(B, false);
    });
  }
  pe(z);
  var $ = te(z, 2);
  {
    var L = (k) => {
      var O = sP(), H = he(O, true);
      pe(O), _e(() => ye(H, x(o))), Y(k, O);
    };
    Se($, (k) => {
      x(o) && k(L);
    });
  }
  Y(e, J), Et();
}
var lP = Ae("<p><b> </b></p> <p> </p> <!>", 1), uP = Ae('<div class="desc"><p><b> </b></p> <p> </p> <p> </p></div>'), cP = Ae('<!> <!> <!> <div></div> <p> </p> <div class="flex gap-05"><!> <!> <!></div>', 1), dP = Ae("<div></div> <!> <!>", 1), pP = Ae('<div class="err"> </div>'), hP = Ae("<!> <!>", 1);
function fP(e, t) {
  At(t, true);
  const r = "min(20rem, calc(100dvw - .5rem))";
  let s = tr(), o = Ot(), p = ee(false), h = ee(""), v = ee(false), m = ee(void 0), l = ee(false), w = ee(""), U = ee(""), J = yt(() => {
    var _a3;
    return x(w).length > (((_a3 = x(m)) == null ? void 0 : _a3.length_min) || 8) && x(w) === x(U);
  }), z = ee(void 0), K = ee(void 0);
  Xl(async () => {
    var _a3;
    let se = await Lt("/auth/v1/password_policy");
    se.body ? E(m, M(se.body)) : E(h, M(((_a3 = se.error) == null ? void 0 : _a3.message) || "Error"));
  }), wt(() => {
    t.user.id && (E(w, ""), E(U, ""));
  });
  async function Z() {
    E(h, ""), E(p, true);
    let se = { email: t.user.email }, fe = await lc("/auth/v1/users/request_reset", se);
    fe.error ? E(h, M(fe.error.message)) : (E(v, true), setTimeout(() => {
      E(v, false);
    }, 3e3)), E(p, false);
  }
  async function B(se, fe) {
    if (!x(l)) {
      E(h, M(s.account.passwordPolicyFollow));
      return;
    }
    if (x(w) !== x(U)) {
      E(h, M(s.account.passwordNoMatch));
      return;
    }
    let me = { email: t.user.email, given_name: t.user.given_name, family_name: t.user.family_name, language: t.user.language, password: x(w), roles: t.user.roles, groups: t.user.groups, enabled: t.user.enabled, email_verified: t.user.email_verified, user_expires: t.user.user_expires };
    E(h, ""), E(p, true);
    let _ = await Jl(se.action, me);
    _.error ? E(h, M(_.error.message)) : (E(v, true), setTimeout(() => {
      E(v, false), t.onSave();
    }, 3e3)), E(p, false);
  }
  function $() {
    if (E(h, ""), !x(m)) return;
    let se = Pd(x(m));
    E(w, M(se)), E(U, M(se)), requestAnimationFrame(() => {
      var _a3, _b2;
      (_a3 = x(z)) == null ? void 0 : _a3(), (_b2 = x(K)) == null ? void 0 : _b2();
    });
  }
  var L = hP(), k = Ee(L);
  {
    var O = (se) => {
      var fe = lP(), me = Ee(fe), _ = he(me), S = he(_, true);
      pe(_), pe(me);
      var R = te(me, 2), P = he(R, true);
      pe(R);
      var A = te(R, 2);
      Rt(A, { onclick: Z, get isLoading() {
        return x(p);
      }, children: (c, a) => {
        Ve();
        var i = $e();
        _e(() => ye(i, o.users.pwdSendEmailBtn)), Y(c, i);
      }, $$slots: { default: true } }), _e(() => {
        ye(S, o.users.pwdNoInit), ye(P, o.users.pwdSendEmailDesc);
      }), Y(se, fe);
    }, H = (se) => {
      var fe = et(), me = Ee(fe);
      {
        var _ = (R) => {
          var P = uP(), A = he(P), c = he(A), a = he(c, true);
          pe(c), pe(A);
          var i = te(A, 2), n = he(i, true);
          pe(i);
          var u = te(i, 2), f = he(u, true);
          pe(u), pe(P), _e(() => {
            ye(a, o.users.pkOnly1), ye(n, o.users.pkOnly2), ye(f, o.users.pkOnly3);
          }), Y(R, P);
        }, S = (R) => {
          var P = dP(), A = Ee(P);
          Yt(A, "margin-top", ".5rem");
          var c = te(A, 2);
          {
            var a = (u) => {
              Ad(u, { get password() {
                return x(w);
              }, get policy() {
                return x(m);
              }, get accepted() {
                return x(l);
              }, set accepted(f) {
                E(l, M(f));
              } });
            };
            Se(c, (u) => {
              x(m) && u(a);
            });
          }
          var i = te(c, 2);
          const n = yt(() => `/auth/v1/users/${t.user.id}`);
          Yl(i, { get action() {
            return x(n);
          }, onSubmit: B, children: (u, f) => {
            var b = cP(), y = Ee(b);
            const g = yt(() => {
              var _a3;
              return ((_a3 = x(m)) == null ? void 0 : _a3.length_max) || 256;
            });
            yu(y, { autocomplete: "off", get label() {
              return s.account.passwordNew;
            }, get placeholder() {
              return s.account.passwordNew;
            }, get showCopy() {
              return x(J);
            }, required: true, get maxLength() {
              return x(g);
            }, width: r, get value() {
              return x(w);
            }, set value(qe) {
              E(w, M(qe));
            }, get reportValidity() {
              return x(z);
            }, set reportValidity(qe) {
              E(z, M(qe));
            } });
            var C = te(y, 2);
            const q = yt(() => {
              var _a3;
              return ((_a3 = x(m)) == null ? void 0 : _a3.length_max) || 256;
            });
            yu(C, { autocomplete: "off", get label() {
              return s.account.passwordConfirm;
            }, get placeholder() {
              return s.account.passwordConfirm;
            }, required: true, get maxLength() {
              return x(q);
            }, width: r, get value() {
              return x(U);
            }, set value(qe) {
              E(U, M(qe));
            }, get reportValidity() {
              return x(K);
            }, set reportValidity(qe) {
              E(K, M(qe));
            } });
            var V = te(C, 2);
            Rt(V, { level: 2, onclick: $, children: (qe, He) => {
              Ve();
              var Ne = $e();
              _e(() => ye(Ne, s.account.generateRandom)), Y(qe, Ne);
            }, $$slots: { default: true } });
            var Q = te(V, 2);
            Yt(Q, "margin-top", "1rem");
            var F = te(Q, 2), ce = he(F, true);
            pe(F);
            var ue = te(F, 2), de = he(ue);
            Rt(de, { onclick: Z, get isLoading() {
              return x(p);
            }, children: (qe, He) => {
              Ve();
              var Ne = $e();
              _e(() => ye(Ne, o.users.sendResetEmail)), Y(qe, Ne);
            }, $$slots: { default: true } });
            var be = te(de, 2);
            Rt(be, { type: "submit", level: 2, get isLoading() {
              return x(p);
            }, children: (qe, He) => {
              Ve();
              var Ne = $e();
              _e(() => ye(Ne, o.users.savePassword)), Y(qe, Ne);
            }, $$slots: { default: true } });
            var ke = te(be, 2);
            {
              var Oe = (qe) => {
                Wr(qe, {});
              };
              Se(ke, (qe) => {
                x(v) && qe(Oe);
              });
            }
            pe(ue), _e(() => ye(ce, o.users.selfServiceDesc)), Y(u, b);
          }, $$slots: { default: true } }), Y(R, P);
        };
        Se(me, (R) => {
          t.user.account_type === "passkey" || t.user.account_type === "federated_passkey" ? R(_) : R(S, false);
        }, true);
      }
      Y(se, fe);
    };
    Se(k, (se) => {
      t.user.account_type === "new" ? se(O) : se(H, false);
    });
  }
  var D = te(k, 2);
  {
    var ie = (se) => {
      var fe = pP(), me = he(fe, true);
      pe(fe), _e(() => ye(me, x(h))), Y(se, fe);
    };
    Se(D, (se) => {
      x(h) && se(ie);
    });
  }
  Y(e, L), Et();
}
var mP = Ae("<p> </p>"), vP = Ae('<p> </p> <p><!></p> <div class="keysContainer svelte-1teaezc"></div>', 1), bP = Ae('<div class="err"> </div>'), yP = Ae("<!> <!>", 1);
function gP(e, t) {
  At(t, true);
  let r = Ot(), s = ee(""), o = ee(M([]));
  wt(() => {
    p();
  });
  async function p() {
    var _a3;
    let z = await Lt(`/auth/v1/users/${t.user.id}/webauthn`);
    z.body ? E(o, M(z.body)) : E(s, M(((_a3 = z.error) == null ? void 0 : _a3.message) || "Error"));
  }
  async function h(z) {
    var _a3;
    let K = x(o).length === 1, Z = await Ql(`/auth/v1/users/${t.user.id}/webauthn/delete/${z}`);
    Z.status === 200 ? (await p(), K && t.onSave()) : E(s, M(((_a3 = Z.error) == null ? void 0 : _a3.message) || "Error"));
  }
  var v = yP(), m = Ee(v);
  {
    var l = (z) => {
      var K = mP(), Z = he(K, true);
      pe(K), _e(() => ye(Z, r.users.noMfaKeys)), Y(z, K);
    }, w = (z) => {
      var K = vP(), Z = Ee(K), B = he(Z, true);
      pe(Z);
      var $ = te(Z, 2), L = he($);
      Ed(L, () => r.users.mfaDelete2), pe($);
      var k = te($, 2);
      jr(k, 21, () => x(o), (O) => O.name, (O, H) => {
        kd(O, { get passkey() {
          return x(H);
        }, showDelete: true, onDelete: h });
      }), pe(k), _e(() => ye(B, r.users.mfaDelete1)), Y(z, K);
    };
    Se(m, (z) => {
      x(o).length === 0 ? z(l) : z(w, false);
    });
  }
  var U = te(m, 2);
  {
    var J = (z) => {
      var K = bP(), Z = he(K, true);
      pe(K), _e(() => ye(Z, x(s))), Y(z, K);
    };
    Se(U, (z) => {
      x(s) && z(J);
    });
  }
  Y(e, v), Et();
}
var xP = Ae('<div class="err"> </div>'), _P = Ae('<p> </p> <div class="flex gap-05"><!> <!></div> <!>', 1);
function CP(e, t) {
  At(t, true);
  let r = Ot(), s = ee(""), o = ee(false);
  async function p() {
    E(s, "");
    let Z = await Ql(`/auth/v1/sessions/${t.userId}`);
    Z.error ? E(s, M(Z.error.message)) : (E(o, true), setTimeout(() => {
      E(o, false);
    }, 3e3));
  }
  var h = _P(), v = Ee(h), m = he(v, true);
  pe(v);
  var l = te(v, 2), w = he(l);
  Rt(w, { level: -1, onclick: p, children: (Z, B) => {
    Ve();
    var $ = $e("Logout");
    Y(Z, $);
  }, $$slots: { default: true } });
  var U = te(w, 2);
  {
    var J = (Z) => {
      Wr(Z, {});
    };
    Se(U, (Z) => {
      x(o) && Z(J);
    });
  }
  pe(l);
  var z = te(l, 2);
  {
    var K = (Z) => {
      var B = xP(), $ = he(B, true);
      pe(B), _e(() => ye($, x(s))), Y(Z, B);
    };
    Se(z, (Z) => {
      x(s) && Z(K);
    });
  }
  _e(() => ye(m, r.users.forceLogout)), Y(e, h), Et();
}
var RP = Ae('<div class="err"> </div>'), PP = Ae("<p> </p> <!> <!>", 1);
function wP(e, t) {
  At(t, true);
  let r = tr(), s = Ot(), o = ee("");
  async function p() {
    E(o, "");
    let J = await Ql(`/auth/v1/users/${t.userId}`);
    J.error ? E(o, M(J.error.message)) : t.onSave();
  }
  var h = PP(), v = Ee(h), m = he(v, true);
  pe(v);
  var l = te(v, 2);
  Rt(l, { level: -1, onclick: p, children: (J, z) => {
    Ve();
    var K = $e();
    _e(() => ye(K, r.common.delete)), Y(J, K);
  }, $$slots: { default: true } });
  var w = te(l, 2);
  {
    var U = (J) => {
      var z = RP(), K = he(z, true);
      pe(z), _e(() => ye(K, x(o))), Y(J, z);
    };
    Se(w, (J) => {
      x(o) && J(U);
    });
  }
  _e(() => ye(m, s.users.deleteUser)), Y(e, h), Et();
}
var TP = Ae('<div class="err"> </div>'), SP = Ae('<!> <div class="flex"><!></div> <!>', 1);
function AP(e, t) {
  At(t, true);
  let r = tr(), s = Ot();
  const o = [r.account.navInfo, s.users.attributes, r.common.password, r.account.navMfa, r.account.devices, r.account.navLogout, r.common.delete];
  let p = ee(M(o[0])), h = ee(void 0), v = ee(""), m = ee(void 0);
  wt(() => {
    l();
  });
  async function l() {
    var _a3;
    let $ = await Lt(`/auth/v1/users/${t.userId}`);
    $.body ? (E(m, M($.body)), requestAnimationFrame(() => {
      var _a4;
      (_a4 = x(h)) == null ? void 0 : _a4();
    })) : E(v, M(((_a3 = $.error) == null ? void 0 : _a3.message) || "Error fetching user"));
  }
  var w = SP(), U = Ee(w);
  {
    var J = ($) => {
      var L = TP(), k = he(L, true);
      pe(L), _e(() => ye(k, x(v))), Y($, L);
    };
    Se(U, ($) => {
      x(v) && $(J);
    });
  }
  var z = te(U, 2), K = he(z);
  wd(K, { tabs: o, get selected() {
    return x(p);
  }, set selected($) {
    E(p, M($));
  }, get focusFirst() {
    return x(h);
  }, set focusFirst($) {
    E(h, M($));
  } }), pe(z);
  var Z = te(z, 2);
  {
    var B = ($) => {
      var L = et(), k = Ee(L);
      {
        var O = (D) => {
          Hd(D, { get roles() {
            return t.roles;
          }, get groups() {
            return t.groups;
          }, get onSave() {
            return t.onSave;
          }, get user() {
            return x(m);
          }, set user(ie) {
            E(m, M(ie));
          } });
        }, H = (D) => {
          var ie = et(), se = Ee(ie);
          {
            var fe = (_) => {
              oP(_, { get user() {
                return x(m);
              }, get onSave() {
                return t.onSave;
              } });
            }, me = (_) => {
              var S = et(), R = Ee(S);
              {
                var P = (c) => {
                  fP(c, { get user() {
                    return x(m);
                  }, get onSave() {
                    return t.onSave;
                  } });
                }, A = (c) => {
                  var a = et(), i = Ee(a);
                  {
                    var n = (f) => {
                      gP(f, { get user() {
                        return x(m);
                      }, get onSave() {
                        return t.onSave;
                      } });
                    }, u = (f) => {
                      var b = et(), y = Ee(b);
                      {
                        var g = (q) => {
                          Id(q, { viewMode: "admin", get userId() {
                            return t.userId;
                          } });
                        }, C = (q) => {
                          var V = et(), Q = Ee(V);
                          {
                            var F = (ue) => {
                              CP(ue, { get userId() {
                                return t.userId;
                              } });
                            }, ce = (ue) => {
                              var de = et(), be = Ee(de);
                              {
                                var ke = (Oe) => {
                                  wP(Oe, { get userId() {
                                    return t.userId;
                                  }, get onSave() {
                                    return t.onSave;
                                  } });
                                };
                                Se(be, (Oe) => {
                                  x(p) === o[6] && Oe(ke);
                                }, true);
                              }
                              Y(ue, de);
                            };
                            Se(Q, (ue) => {
                              x(p) === o[5] ? ue(F) : ue(ce, false);
                            }, true);
                          }
                          Y(q, V);
                        };
                        Se(y, (q) => {
                          x(p) === o[4] ? q(g) : q(C, false);
                        }, true);
                      }
                      Y(f, b);
                    };
                    Se(i, (f) => {
                      x(p) === o[3] ? f(n) : f(u, false);
                    }, true);
                  }
                  Y(c, a);
                };
                Se(R, (c) => {
                  x(p) === o[2] ? c(P) : c(A, false);
                }, true);
              }
              Y(_, S);
            };
            Se(se, (_) => {
              x(p) === o[1] ? _(fe) : _(me, false);
            }, true);
          }
          Y(D, ie);
        };
        Se(k, (D) => {
          x(p) === o[0] ? D(O) : D(H, false);
        });
      }
      Y($, L);
    };
    Se(Z, ($) => {
      x(m) && $(B);
    });
  }
  Y(e, w), Et();
}
var EP = Ae("<div></div> <!> <!>", 1), kP = Ae("<!> <!>", 1), IP = Ae('<div class="err"> </div>'), qP = Ae('<!> <div id="users"><!></div>', 1), MP = Ae("<!> <!>", 1);
function v2(e, t) {
  At(t, true);
  const r = (P, A = mu, c = mu) => {
    const a = yt(() => m.get() === A());
    xd(P, { onclick: () => m.set(A()), get selected() {
      return x(a);
    }, children: (i, n) => {
      Ve();
      var u = $e();
      _e(() => ye(u, c())), Y(i, u);
    } });
  };
  let s = ee(void 0), o = ee(""), p = ee(M([])), h = ee(M([])), v = ee(M([])), m = gd("uid"), l = ee(void 0), w = ee(M([])), U = ee(M([])), J = ee(false), z = ee(void 0), K = ee(M(qd)), Z = ee(false), B = M(["E-Mail", "ID"]), $ = ee(M(B[0])), L = ee(""), k = M(["E-Mail", "ID", "Created", "Last Login"]);
  Xl(() => {
    H("page_size=" + x(K)), D(), ie();
  }), wt(() => {
    var _a3;
    E(l, M((_a3 = x(p).find((P) => P.id === m.get())) == null ? void 0 : _a3.id));
  }), wt(() => {
    let P = x(L).toLowerCase();
    x(J) ? P.length < 3 ? x(Z) && (H("page_size=" + x(K)), E(Z, false)) : O(P) : P ? x($) === B[0] ? E(h, M(x(p).filter((A) => {
      var _a3;
      return (_a3 = A.email) == null ? void 0 : _a3.toLowerCase().includes(P);
    }))) : x($) === B[1] && E(h, M(x(p).filter((A) => A.id.toLowerCase().includes(P)))) : E(h, M(x(p)));
  });
  async function O(P) {
    E(z, void 0), E(Z, true);
    let A;
    x($) === B[0] ? A = "email" : A = "id";
    let c = await Md({ ty: "user", idx: A, q: P });
    c.body ? E(p, M(c.body)) : console.error(c.error);
  }
  async function H(P) {
    let A = "/auth/v1/users";
    P && (A += `?${P}`);
    let c = await Lt(A);
    return c.error ? E(o, "Error fetching users: " + c.error.message) : c.body && (c.status === 206 ? (E(J, true), E(z, M(c.headers))) : (E(J, false), E(z, void 0)), E(p, M(c.body))), [c.status, c.headers];
  }
  async function D() {
    var _a3;
    let P = await Lt("/auth/v1/roles");
    P.body ? E(U, M(P.body.toSorted((A, c) => A.name.localeCompare(c.name)))) : E(o, M(((_a3 = P.error) == null ? void 0 : _a3.message) || "Error"));
  }
  async function ie() {
    var _a3;
    let P = await Lt("/auth/v1/groups");
    P.body ? E(w, M(P.body.toSorted((A, c) => A.name.localeCompare(c.name)))) : E(o, M(((_a3 = P.error) == null ? void 0 : _a3.message) || "Error"));
  }
  function se(P, A) {
    let c = A === "up";
    P === k[0] ? x(p).sort((a, i) => c ? a.email.localeCompare(i.email) : i.email.localeCompare(a.email)) : P === k[1] ? x(p).sort((a, i) => c ? a.id.localeCompare(i.id) : i.id.localeCompare(a.id)) : P === k[2] ? x(p).sort((a, i) => c ? a.created_at - i.created_at : i.created_at - a.created_at) : P === k[3] && x(p).sort((a, i) => {
      let n = a.last_login || 9999999999, u = a.last_login || 9999999999;
      return c ? n - u : u - n;
    });
  }
  async function fe(P) {
    var _a3;
    (_a3 = x(s)) == null ? void 0 : _a3(), await H(), m.set(P);
  }
  function me() {
    H(), D(), ie(), E(L, "");
  }
  var _ = MP(), S = Ee(_);
  bd(S, { paddingTop: "2.1rem", buttonTilesAriaControls: "users", width: "min(23rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (A) => {
    var c = EP(), a = Ee(c);
    Yt(a, "height", ".5rem");
    var i = te(a, 2);
    {
      var n = (g) => {
        var C = et(), q = Ee(C);
        jr(q, 17, () => x(p), (V) => V.id, (V, Q) => {
          r(V, () => x(Q).id, () => x(Q).email);
        }), Y(g, C);
      }, u = (g) => {
        var C = et(), q = Ee(C);
        jr(q, 17, () => x(v), (V) => V.id, (V, Q) => {
          r(V, () => x(Q).id, () => x(Q).email);
        }), Y(g, C);
      };
      Se(i, (g) => {
        x(z) ? g(n) : g(u, false);
      });
    }
    var f = te(i, 2);
    {
      var b = (g) => {
        Ld(g, { sspFetch: H, idxTotalCount: "x-user-count", get itemsLength() {
          return x(p).length;
        }, get firstFetchHeaders() {
          return x(z);
        }, get pageSize() {
          return x(K);
        }, set pageSize(C) {
          E(K, M(C));
        } });
      }, y = (g) => {
        var C = et(), q = Ee(C);
        {
          var V = (F) => {
            vu(F, { get items() {
              return x(p);
            }, set items(ce) {
              E(p, M(ce));
            }, get itemsPaginated() {
              return x(v);
            }, set itemsPaginated(ce) {
              E(v, M(ce));
            } });
          }, Q = (F) => {
            vu(F, { get items() {
              return x(h);
            }, set items(ce) {
              E(h, M(ce));
            }, get itemsPaginated() {
              return x(v);
            }, set itemsPaginated(ce) {
              E(v, M(ce));
            } });
          };
          Se(q, (F) => {
            x(J) ? F(V) : F(Q, false);
          }, true);
        }
        Y(g, C);
      };
      Se(f, (g) => {
        x(z) ? g(b) : g(y, false);
      });
    }
    Y(A, c);
  }, children: (A, c) => {
    var a = kP(), i = Ee(a);
    const n = yt(() => x(U).length === 0 ? 1 : 2);
    yd(i, { get level() {
      return x(n);
    }, alignRight: true, get closeModal() {
      return x(s);
    }, set closeModal(f) {
      E(s, M(f));
    }, children: (f, b) => {
      $d(f, { onSave: fe, get roles() {
        return x(U);
      }, get groups() {
        return x(w);
      } });
    }, $$slots: { default: true } });
    var u = te(i, 2);
    vd(u, { get searchOptions() {
      return B;
    }, get orderOptions() {
      return k;
    }, onChangeOrder: se, searchWidth: "min(23rem, calc(100dvw - .5rem))", get value() {
      return x(L);
    }, set value(f) {
      E(L, M(f));
    }, get searchOption() {
      return x($);
    }, set searchOption(f) {
      E($, M(f));
    } }), Y(A, a);
  }, $$slots: { buttonTiles: true, default: true } });
  var R = te(S, 2);
  _d(R, { children: (P, A) => {
    var c = qP(), a = Ee(c);
    {
      var i = (b) => {
        var y = IP(), g = he(y, true);
        pe(y), _e(() => ye(g, x(o))), Y(b, y);
      };
      Se(a, (b) => {
        x(o) && b(i);
      });
    }
    var n = te(a, 2), u = he(n);
    {
      var f = (b) => {
        AP(b, { get userId() {
          return x(l);
        }, get roles() {
          return x(U);
        }, get groups() {
          return x(w);
        }, onSave: me });
      };
      Se(u, (b) => {
        x(l) && b(f);
      });
    }
    pe(n), Y(P, c);
  } }), Y(e, _), Et();
}
export {
  v2 as U
};
