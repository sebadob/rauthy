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
import { t as Me, a as ae, e as $e, d as st } from "./BH6NCLk-.js";
import { p as It, k as ie, O as $o, aa as At, a as Mt, f as Le, l as $, j as k, s as ne, t as _e, c as ve, a7 as Ke, a9 as _t, r as me, a5 as gl, a8 as Nl } from "./CvlvO1XB.js";
import { s as xe } from "./CTI4QPiR.js";
import { i as Se } from "./BUO_AUgz.js";
import { e as Ho, i as ov } from "./BpWRzPRQ.js";
import { a as Xt, B as wt, t as Ch, s as Ph } from "./DMkkW5Nn.js";
import { p as W } from "./Wh68IIk2.js";
import { b as qh, d as Rl, f as Nt, c as xl } from "./bbkAiDd0.js";
import { O as lv } from "./DxrdFxfV.js";
import { P as Dl } from "./CLPs45Pj.js";
import { N as uv } from "./D3G2nKoD.js";
import { B as cv } from "./BIsFWZ5z.js";
import { u as dv } from "./D45liQ4S.js";
import { N as pv } from "./D-L0o8jR.js";
import { C as fv } from "./BnPoFdx3.js";
import { I as pt } from "./DmeAqnkr.js";
import { b as Uo, k as hv, l as jl, m as mv } from "./BRCxk8by.js";
import { F as _l } from "./CqS1e6KT.js";
import { u as Yt } from "./DOl8_ubJ.js";
import { u as Dt } from "./D8mHI_K9.js";
import { O as Th } from "./CGniv4pe.js";
import { n as Sh, l as nl, A as vv } from "./CGXT-546.js";
import { I as jo } from "./DTR8xafZ.js";
import { I as hl } from "./C3mvM0t4.js";
import { f as zt, a as Bo, b as wh } from "./DswDW5U8.js";
import { S as Go } from "./CwWWGNuX.js";
import { L as Bt } from "./CE2_6siz.js";
import { T as bv } from "./_OE2Cq0B.js";
import { p as yv } from "./C6SR4G2t.js";
import { C as gv } from "./CS_Msctd.js";
import { I as Yo } from "./Nks81rMs.js";
import { P as Rv } from "./B6Ygp7Nf.js";
import { I as Bl } from "./CLUUhKug.js";
import { h as xv } from "./i8Xqpu09.js";
import { U as _v, D as Cv } from "./CHyRPnih.js";
import { P as Pv, f as qv, a as Tv } from "./BnmgLJ5E.js";
var Sv = Me("<div><!></div>"), wv = Me('<div class="err"> </div>'), Av = Me("<!> <!> <!> <!> <!> <!> <!> <!> <div></div> <!> <!>", 1), Ev = Me('<div class="container svelte-x7a3kj"><!></div>');
function kv(e, t) {
  It(t, true);
  let i = Yt(), s = Dt(), l = ie(void 0), p = ie(""), m = ie(""), w = ie(""), _ = ie(""), o = ie("en"), b = ie(W($o(() => t.roles.map((h) => ({ name: h.name, selected: false }))))), L = ie(W($o(() => t.groups.map((h) => ({ name: h.name, selected: false }))))), O = ie(false), B = ie(W(zt())), G = ie(W(Bo()));
  At(() => {
    requestAnimationFrame(() => {
      var _a3;
      (_a3 = k(l)) == null ? void 0 : _a3.focus();
    });
  });
  async function F(h, g) {
    var _a3, _b2;
    $(p, "");
    let S = { email: k(m), given_name: k(w), family_name: k(_) || void 0, language: k(o), groups: k(L).filter((x) => x.selected).map((x) => x.name), roles: k(b).filter((x) => x.selected).map((x) => x.name), user_expires: wh(k(B), k(G)) };
    ((_a3 = S.groups) == null ? void 0 : _a3.length) === 0 && (S.groups = void 0);
    let C = await qh(h.action, S);
    C.body ? t.onSave(C.body.id) : $(p, W(((_b2 = C.error) == null ? void 0 : _b2.message) || "Error"));
  }
  var R = Ev(), v = ve(R);
  _l(v, { action: "/auth/v1/users", onSubmit: F, children: (h, g) => {
    var S = Av(), C = Le(S);
    pt(C, { typ: "email", autocomplete: "off", label: "E-Mail", placeholder: "E-Mail", required: true, get ref() {
      return k(l);
    }, set ref(a) {
      $(l, W(a));
    }, get value() {
      return k(m);
    }, set value(a) {
      $(m, W(a));
    } });
    var x = ne(C, 2);
    pt(x, { autocomplete: "off", get label() {
      return i.account.givenName;
    }, get placeholder() {
      return i.account.givenName;
    }, required: true, pattern: Uo, get value() {
      return k(w);
    }, set value(a) {
      $(w, W(a));
    } });
    var E = ne(x, 2);
    pt(E, { autocomplete: "off", get label() {
      return i.account.familyName;
    }, get placeholder() {
      return i.account.familyName;
    }, pattern: Uo, get value() {
      return k(_);
    }, set value(a) {
      $(_, W(a));
    } });
    var D = ne(E, 2);
    Bt(D, { get label() {
      return s.common.language;
    }, children: (a, n) => {
      Th(a, { get ariaLabel() {
        return i.common.selectI18n;
      }, options: Sh, borderless: true, get value() {
        return k(o);
      }, set value(u) {
        $(o, W(u));
      } });
    } });
    var z = ne(D, 2);
    Go(z, { get items() {
      return k(b);
    }, set items(a) {
      $(b, W(a));
    }, children: (a, n) => {
      Ke();
      var u = $e();
      _e(() => xe(u, i.account.roles)), ae(a, u);
    }, $$slots: { default: true } });
    var Z = ne(z, 2);
    Go(Z, { get items() {
      return k(L);
    }, set items(a) {
      $(L, W(a));
    }, children: (a, n) => {
      Ke();
      var u = $e();
      _e(() => xe(u, i.account.groups)), ae(a, u);
    }, $$slots: { default: true } });
    var A = ne(Z, 2);
    jo(A, { get ariaLabel() {
      return i.account.accessExp;
    }, get checked() {
      return k(O);
    }, set checked(a) {
      $(O, W(a));
    }, children: (a, n) => {
      Ke();
      var u = $e();
      _e(() => xe(u, i.account.accessExp)), ae(a, u);
    }, $$slots: { default: true } });
    var N = ne(A, 2);
    {
      var I = (a) => {
        var n = Sv(), u = ve(n);
        const f = _t(zt);
        hl(u, { get label() {
          return i.account.accessExp;
        }, withTime: true, get min() {
          return k(f);
        }, required: true, get value() {
          return k(B);
        }, set value(y) {
          $(B, W(y));
        }, get timeValue() {
          return k(G);
        }, set timeValue(y) {
          $(G, W(y));
        } }), me(n), Ch(3, n, () => Ph, () => ({ duration: 150 })), ae(a, n);
      };
      Se(N, (a) => {
        k(O) && a(I);
      });
    }
    var M = ne(N, 2);
    Xt(M, "height", ".66rem");
    var V = ne(M, 2);
    wt(V, { type: "submit", children: (a, n) => {
      Ke();
      var u = $e();
      _e(() => xe(u, i.common.save)), ae(a, u);
    }, $$slots: { default: true } });
    var c = ne(V, 2);
    {
      var r = (a) => {
        var n = wv(), u = ve(n, true);
        me(n), _e(() => xe(u, k(p))), ae(a, n);
      };
      Se(c, (a) => {
        k(p) && a(r);
      });
    }
    ae(h, S);
  }, $$slots: { default: true } }), me(R), ae(e, R), Mt();
}
var Iv = Me('<div class="svelte-k6dexs"><!></div>'), Mv = Me('<div class="err"> </div>'), Lv = Me('<!> <div class="values svelte-k6dexs"><div class="svelte-k6dexs"><!></div> <div class="svelte-k6dexs"><!></div></div> <div class="values svelte-k6dexs"><div class="svelte-k6dexs"><!> <!> <!> <!> <div><!></div></div> <div class="svelte-k6dexs"><!> <!> <!> <!> <!></div></div> <!> <!> <div class="values svelte-k6dexs"><div class="svelte-k6dexs"><!></div> <!></div> <div class="values svelte-k6dexs"><div class="svelte-k6dexs"><!> <!></div> <div class="svelte-k6dexs"><!> <!></div></div> <div class="flex gap-05"><!> <!></div> <!>', 1);
function Ov(e, t) {
  It(t, true);
  let i = yv(t, "user", 15), s = Yt(), l = Dt(), p = ie(""), m = ie(false), w = ie(""), _ = ie(""), o = ie(""), b = ie("en"), L = ie(""), O = ie(""), B = ie(""), G = ie(""), F = ie(""), R = ie(""), v = ie(false), h = ie(false), g = ie(false), S = ie(W(zt())), C = ie(W(Bo())), x = ie(W($o(() => t.roles.map((N) => ({ name: N.name, selected: false }))))), E = ie(W($o(() => t.groups.map((N) => ({ name: N.name, selected: false })))));
  At(() => {
    var _a3, _b2, _c2, _d2, _e3, _f2, _g2;
    if (i()) {
      if ($(w, W(i().email)), $(_, W(i().given_name)), $(o, W(i().family_name || "")), $(b, W(i().language)), $(L, W(((_a3 = i().user_values) == null ? void 0 : _a3.birthdate) || "")), $(O, W(((_b2 = i().user_values) == null ? void 0 : _b2.phone) || "")), $(B, W(((_c2 = i().user_values) == null ? void 0 : _c2.street) || "")), $(G, W(((_e3 = (_d2 = i().user_values) == null ? void 0 : _d2.zip) == null ? void 0 : _e3.toString()) || "")), $(F, W(((_f2 = i().user_values) == null ? void 0 : _f2.city) || "")), $(R, W(((_g2 = i().user_values) == null ? void 0 : _g2.country) || "")), $(v, W(i().enabled)), $(h, W(i().email_verified)), i().user_expires) {
        let N = new Date(i().user_expires * 1e3);
        $(g, true), $(S, W(zt(N))), $(C, W(Bo(N)));
      } else $(g, false), $(S, W(zt())), $(C, W(Bo()));
      $(x, W(t.roles.map((N) => {
        var _a4;
        return { name: N.name, selected: ((_a4 = i()) == null ? void 0 : _a4.roles.includes(N.name)) || false };
      }))), $(E, W(t.roles.map((N) => {
        var _a4, _b3;
        return { name: N.name, selected: ((_b3 = (_a4 = i()) == null ? void 0 : _a4.groups) == null ? void 0 : _b3.includes(N.name)) || false };
      })));
    }
  });
  async function D(N, I) {
    var _a3, _b2;
    $(p, "");
    const M = { email: k(w), given_name: k(_), family_name: k(o) || void 0, language: k(b), roles: k(x).filter((c) => c.selected).map((c) => c.name), groups: k(E).filter((c) => c.selected).map((c) => c.name), enabled: k(v), email_verified: k(h), user_expires: k(g) ? wh(k(S), k(C)) : void 0 };
    (k(L) || k(O) || k(B) || k(G) || k(F) || k(R)) && (console.log(k(L)), M.user_values = { birthdate: k(L) || void 0, phone: ((_a3 = k(O)) == null ? void 0 : _a3.replaceAll(" ", "")) || void 0, street: k(B) || void 0, zip: k(G) ? Number.parseInt(k(G)) : void 0, city: k(F) || void 0, country: k(R) || void 0 });
    let V = await Rl(N.action, M);
    V.body ? ($(m, true), i(V.body), setTimeout(() => {
      $(m, false);
    }, 3e3)) : $(p, W(((_b2 = V.error) == null ? void 0 : _b2.message) || "Error"));
  }
  var z = st(), Z = Le(z);
  {
    var A = (N) => {
      const I = _t(() => `/auth/v1/users/${i().id}`);
      _l(N, { get action() {
        return k(I);
      }, onSubmit: D, children: (M, V) => {
        var c = Lv(), r = Le(c);
        Bt(r, { label: "ID", mono: true, children: (te, Ie) => {
          Ke();
          var be = $e();
          _e(() => xe(be, i().id)), ae(te, be);
        } });
        var a = ne(r, 2), n = ve(a), u = ve(n);
        jo(u, { get ariaLabel() {
          return l.common.enabled;
        }, get checked() {
          return k(v);
        }, set checked(te) {
          $(v, W(te));
        }, children: (te, Ie) => {
          Ke();
          var be = $e();
          _e(() => xe(be, l.common.enabled)), ae(te, be);
        }, $$slots: { default: true } }), me(n);
        var f = ne(n, 2), y = ve(f);
        jo(y, { get ariaLabel() {
          return s.account.emailVerified;
        }, get checked() {
          return k(h);
        }, set checked(te) {
          $(h, W(te));
        }, children: (te, Ie) => {
          Ke();
          var be = $e();
          _e(() => xe(be, s.account.emailVerified)), ae(te, be);
        }, $$slots: { default: true } }), me(f), me(a);
        var P = ne(a, 2), T = ve(P), q = ve(T);
        pt(q, { typ: "email", autocomplete: "off", label: "E-Mail", placeholder: "E-Mail", required: true, get value() {
          return k(w);
        }, set value(te) {
          $(w, W(te));
        } });
        var H = ne(q, 2);
        pt(H, { autocomplete: "off", get label() {
          return s.account.givenName;
        }, get placeholder() {
          return s.account.givenName;
        }, required: true, pattern: Uo, get value() {
          return k(_);
        }, set value(te) {
          $(_, W(te));
        } });
        var J = ne(H, 2);
        pt(J, { autocomplete: "off", get label() {
          return s.account.familyName;
        }, get placeholder() {
          return s.account.familyName;
        }, pattern: Uo, get value() {
          return k(o);
        }, set value(te) {
          $(o, W(te));
        } });
        var re = ne(J, 2);
        hl(re, { get label() {
          return s.account.birthdate;
        }, withDelete: true, get value() {
          return k(L);
        }, set value(te) {
          $(L, W(te));
        } });
        var Q = ne(re, 2);
        Xt(Q, "padding", ".25rem");
        var pe = ve(Q);
        Bt(pe, { get label() {
          return l.common.language;
        }, children: (te, Ie) => {
          Th(te, { get ariaLabel() {
            return s.common.selectI18n;
          }, options: Sh, borderless: true, get value() {
            return k(b);
          }, set value(be) {
            $(b, W(be));
          } });
        } }), me(Q), me(T);
        var de = ne(T, 2), fe = ve(de);
        pt(fe, { autocomplete: "off", get label() {
          return s.account.street;
        }, get placeholder() {
          return s.account.street;
        }, pattern: hv, get value() {
          return k(B);
        }, set value(te) {
          $(B, W(te));
        } });
        var he = ne(fe, 2);
        pt(he, { typ: "number", autocomplete: "off", get label() {
          return s.account.zip;
        }, get placeholder() {
          return s.account.zip;
        }, min: "1000", max: "9999999", get value() {
          return k(G);
        }, set value(te) {
          $(G, W(te));
        } });
        var we = ne(he, 2);
        pt(we, { autocomplete: "off", get label() {
          return s.account.city;
        }, get placeholder() {
          return s.account.city;
        }, pattern: jl, get value() {
          return k(F);
        }, set value(te) {
          $(F, W(te));
        } });
        var Oe = ne(we, 2);
        pt(Oe, { autocomplete: "off", get label() {
          return s.account.country;
        }, get placeholder() {
          return s.account.country;
        }, pattern: jl, get value() {
          return k(R);
        }, set value(te) {
          $(R, W(te));
        } });
        var Ee = ne(Oe, 2);
        pt(Ee, { autocomplete: "off", get label() {
          return s.account.phone;
        }, get placeholder() {
          return s.account.phone;
        }, pattern: mv, get value() {
          return k(O);
        }, set value(te) {
          $(O, W(te));
        } }), me(de), me(P);
        var Ue = ne(P, 2);
        Go(Ue, { get items() {
          return k(x);
        }, set items(te) {
          $(x, W(te));
        }, children: (te, Ie) => {
          Ke();
          var be = $e();
          _e(() => xe(be, s.account.roles)), ae(te, be);
        }, $$slots: { default: true } });
        var Ne = ne(Ue, 2);
        Go(Ne, { get items() {
          return k(E);
        }, set items(te) {
          $(E, W(te));
        }, children: (te, Ie) => {
          Ke();
          var be = $e();
          _e(() => xe(be, s.account.groups)), ae(te, be);
        }, $$slots: { default: true } });
        var Ze = ne(Ne, 2), Je = ve(Ze);
        Xt(Je, "margin-top", ".5rem");
        var vt = ve(Je);
        jo(vt, { get ariaLabel() {
          return s.account.accessExp;
        }, get checked() {
          return k(g);
        }, set checked(te) {
          $(g, W(te));
        }, children: (te, Ie) => {
          Ke();
          var be = $e();
          _e(() => xe(be, s.account.accessExp)), ae(te, be);
        }, $$slots: { default: true } }), me(Je);
        var bt = ne(Je, 2);
        {
          var j = (te) => {
            var Ie = Iv(), be = ve(Ie);
            const Qe = _t(zt);
            hl(be, { get label() {
              return s.account.accessExp;
            }, withTime: true, get min() {
              return k(Qe);
            }, required: true, get value() {
              return k(S);
            }, set value(tt) {
              $(S, W(tt));
            }, get timeValue() {
              return k(C);
            }, set timeValue(tt) {
              $(C, W(tt));
            } }), me(Ie), Ch(3, Ie, () => Ph, () => ({ duration: 150 })), ae(te, Ie);
          };
          Se(bt, (te) => {
            k(g) && te(j);
          });
        }
        me(Ze);
        var X = ne(Ze, 2), K = ve(X), ee = ve(K);
        Bt(ee, { get label() {
          return s.account.userCreated;
        }, children: (te, Ie) => {
          Ke();
          var be = $e();
          _e((Qe) => xe(be, Qe), [() => nl(i().created_at)]), ae(te, be);
        } });
        var ce = ne(ee, 2);
        Bt(ce, { get label() {
          return l.users.lastLogin;
        }, children: (te, Ie) => {
          var be = st(), Qe = Le(be);
          {
            var tt = (We) => {
              var Ye = $e();
              _e((St) => xe(Ye, St), [() => nl(i().last_login)]), ae(We, Ye);
            }, Xe = (We) => {
              var Ye = $e();
              _e(() => xe(Ye, s.common.never)), ae(We, Ye);
            };
            Se(Qe, (We) => {
              i().last_login ? We(tt) : We(Xe, false);
            });
          }
          ae(te, be);
        } }), me(K);
        var se = ne(K, 2), Re = ve(se);
        Bt(Re, { get label() {
          return s.account.passwordExpiry;
        }, children: (te, Ie) => {
          var be = st(), Qe = Le(be);
          {
            var tt = (We) => {
              var Ye = $e();
              _e((St) => xe(Ye, St), [() => nl(i().password_expires)]), ae(We, Ye);
            }, Xe = (We) => {
              var Ye = $e();
              _e(() => xe(Ye, s.common.never)), ae(We, Ye);
            };
            Se(Qe, (We) => {
              i().password_expires ? We(tt) : We(Xe, false);
            });
          }
          ae(te, be);
        } });
        var ke = ne(Re, 2);
        Bt(ke, { get label() {
          return s.account.mfaActivated;
        }, children: (te, Ie) => {
          gv(te, {});
        } }), me(se), me(X);
        var ge = ne(X, 2), Ae = ve(ge);
        wt(Ae, { type: "submit", children: (te, Ie) => {
          Ke();
          var be = $e();
          _e(() => xe(be, s.common.save)), ae(te, be);
        }, $$slots: { default: true } });
        var ze = ne(Ae, 2);
        {
          var je = (te) => {
            Yo(te, {});
          };
          Se(ze, (te) => {
            k(m) && te(je);
          });
        }
        me(ge);
        var De = ne(ge, 2);
        {
          var Ge = (te) => {
            var Ie = Mv(), be = ve(Ie, true);
            me(Ie), _e(() => xe(be, k(p))), ae(te, Ie);
          };
          Se(De, (te) => {
            k(p) && te(Ge);
          });
        }
        ae(M, c);
      }, $$slots: { default: true } });
    };
    Se(Z, (N) => {
      i() && N(A);
    });
  }
  ae(e, z), Mt();
}
function Nv(e, t) {
  return e.start <= t && t < e.end;
}
function Dv(e, t = {}) {
  const { offsetLine: i = 0, offsetColumn: s = 0 } = t;
  let l = 0;
  const p = e.split(`
`).map((_, o) => {
    const b = l + _.length + 1, L = { start: l, end: b, line: o };
    return l = b, L;
  });
  let m = 0;
  function w(_, o) {
    if (typeof _ == "string" && (_ = e.indexOf(_, o ?? 0)), _ === -1) return;
    let b = p[m];
    const L = _ >= b.end ? 1 : -1;
    for (; b; ) {
      if (Nv(b, _)) return { line: i + b.line, column: s + _ - b.start, character: _ };
      m += L, b = p[m];
    }
  }
  return w;
}
let jv, Fl = Dv("", { offsetLine: 1 }), Bv, Vl = [], Fv = /* @__PURE__ */ new Map();
const Vv = /^\t+/;
function ol(e) {
  return e.replace(Vv, (t) => t.split("	").join("  "));
}
function $v(e, t, i) {
  const s = e.split(`
`), l = Math.max(0, t - 2), p = Math.min(t + 3, s.length), m = String(p + 1).length;
  return s.slice(l, p).map((w, _) => {
    const o = l + _ === t, b = String(_ + l + 1).padStart(m, " ");
    if (o) {
      const L = " ".repeat(m + 2 + ol(w.slice(0, i)).length) + "^";
      return `${b}: ${ol(w)}
${L}`;
    }
    return `${b}: ${ol(w)}`;
  }).join(`
`);
}
class Ah {
  constructor(t, i, s) {
    __publicField(this, "name", "CompileDiagnostic");
    this.code = t, this.message = i, s && (this.position = s, this.start = Fl(s[0]), this.end = Fl(s[1]), this.start && this.end && (this.frame = $v(jv, this.start.line - 1, this.end.column)));
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
class Hv extends Ah {
  constructor(t, i, s) {
    super(t, i, s);
    __publicField(this, "name", "CompileWarning");
  }
}
function Zt(e, t, i) {
  var _a3;
  let s = Vl;
  if (e && (s = Fv.get(e) ?? Vl), s && ((_a3 = s.at(-1)) == null ? void 0 : _a3.has(t))) return;
  const l = new Hv(t, i, e && e.start !== void 0 ? [e.start, e.end ?? e.start] : void 0);
  Bv(l);
}
function Uv(e) {
  Zt(e, "options_deprecated_accessors", "The `accessors` option has been deprecated. It will have no effect in runes mode\nhttps://svelte.dev/e/options_deprecated_accessors");
}
function Gv(e) {
  Zt(e, "options_deprecated_immutable", "The `immutable` option has been deprecated. It will have no effect in runes mode\nhttps://svelte.dev/e/options_deprecated_immutable");
}
function Wv(e) {
  Zt(e, "options_removed_enable_sourcemap", "The `enableSourcemap` option has been removed. Source maps are always generated now, and tooling can choose to ignore them\nhttps://svelte.dev/e/options_removed_enable_sourcemap");
}
function zv(e) {
  Zt(e, "options_removed_hydratable", "The `hydratable` option has been removed. Svelte components are always hydratable now\nhttps://svelte.dev/e/options_removed_hydratable");
}
function Xv(e) {
  Zt(e, "options_removed_loop_guard_timeout", "The `loopGuardTimeout` option has been removed\nhttps://svelte.dev/e/options_removed_loop_guard_timeout");
}
function Kv(e) {
  Zt(e, "options_renamed_ssr_dom", '`generate: "dom"` and `generate: "ssr"` options have been renamed to "client" and "server" respectively\nhttps://svelte.dev/e/options_renamed_ssr_dom');
}
var Jv = [509, 0, 227, 0, 150, 4, 294, 9, 1368, 2, 2, 1, 6, 3, 41, 2, 5, 0, 166, 1, 574, 3, 9, 9, 7, 9, 32, 4, 318, 1, 80, 3, 71, 10, 50, 3, 123, 2, 54, 14, 32, 10, 3, 1, 11, 3, 46, 10, 8, 0, 46, 9, 7, 2, 37, 13, 2, 9, 6, 1, 45, 0, 13, 2, 49, 13, 9, 3, 2, 11, 83, 11, 7, 0, 3, 0, 158, 11, 6, 9, 7, 3, 56, 1, 2, 6, 3, 1, 3, 2, 10, 0, 11, 1, 3, 6, 4, 4, 68, 8, 2, 0, 3, 0, 2, 3, 2, 4, 2, 0, 15, 1, 83, 17, 10, 9, 5, 0, 82, 19, 13, 9, 214, 6, 3, 8, 28, 1, 83, 16, 16, 9, 82, 12, 9, 9, 7, 19, 58, 14, 5, 9, 243, 14, 166, 9, 71, 5, 2, 1, 3, 3, 2, 0, 2, 1, 13, 9, 120, 6, 3, 6, 4, 0, 29, 9, 41, 6, 2, 3, 9, 0, 10, 10, 47, 15, 343, 9, 54, 7, 2, 7, 17, 9, 57, 21, 2, 13, 123, 5, 4, 0, 2, 1, 2, 6, 2, 0, 9, 9, 49, 4, 2, 1, 2, 4, 9, 9, 330, 3, 10, 1, 2, 0, 49, 6, 4, 4, 14, 10, 5350, 0, 7, 14, 11465, 27, 2343, 9, 87, 9, 39, 4, 60, 6, 26, 9, 535, 9, 470, 0, 2, 54, 8, 3, 82, 0, 12, 1, 19628, 1, 4178, 9, 519, 45, 3, 22, 543, 4, 4, 5, 9, 7, 3, 6, 31, 3, 149, 2, 1418, 49, 513, 54, 5, 49, 9, 0, 15, 0, 23, 4, 2, 14, 1361, 6, 2, 16, 3, 6, 2, 1, 2, 4, 101, 0, 161, 6, 10, 9, 357, 0, 62, 13, 499, 13, 245, 1, 2, 9, 726, 6, 110, 6, 6, 9, 4759, 9, 787719, 239], Eh = [0, 11, 2, 25, 2, 18, 2, 1, 2, 14, 3, 13, 35, 122, 70, 52, 268, 28, 4, 48, 48, 31, 14, 29, 6, 37, 11, 29, 3, 35, 5, 7, 2, 4, 43, 157, 19, 35, 5, 35, 5, 39, 9, 51, 13, 10, 2, 14, 2, 6, 2, 1, 2, 10, 2, 14, 2, 6, 2, 1, 4, 51, 13, 310, 10, 21, 11, 7, 25, 5, 2, 41, 2, 8, 70, 5, 3, 0, 2, 43, 2, 1, 4, 0, 3, 22, 11, 22, 10, 30, 66, 18, 2, 1, 11, 21, 11, 25, 71, 55, 7, 1, 65, 0, 16, 3, 2, 2, 2, 28, 43, 28, 4, 28, 36, 7, 2, 27, 28, 53, 11, 21, 11, 18, 14, 17, 111, 72, 56, 50, 14, 50, 14, 35, 39, 27, 10, 22, 251, 41, 7, 1, 17, 2, 60, 28, 11, 0, 9, 21, 43, 17, 47, 20, 28, 22, 13, 52, 58, 1, 3, 0, 14, 44, 33, 24, 27, 35, 30, 0, 3, 0, 9, 34, 4, 0, 13, 47, 15, 3, 22, 0, 2, 0, 36, 17, 2, 24, 20, 1, 64, 6, 2, 0, 2, 3, 2, 14, 2, 9, 8, 46, 39, 7, 3, 1, 3, 21, 2, 6, 2, 1, 2, 4, 4, 0, 19, 0, 13, 4, 31, 9, 2, 0, 3, 0, 2, 37, 2, 0, 26, 0, 2, 0, 45, 52, 19, 3, 21, 2, 31, 47, 21, 1, 2, 0, 185, 46, 42, 3, 37, 47, 21, 0, 60, 42, 14, 0, 72, 26, 38, 6, 186, 43, 117, 63, 32, 7, 3, 0, 3, 7, 2, 1, 2, 23, 16, 0, 2, 0, 95, 7, 3, 38, 17, 0, 2, 0, 29, 0, 11, 39, 8, 0, 22, 0, 12, 45, 20, 0, 19, 72, 200, 32, 32, 8, 2, 36, 18, 0, 50, 29, 113, 6, 2, 1, 2, 37, 22, 0, 26, 5, 2, 1, 2, 31, 15, 0, 328, 18, 16, 0, 2, 12, 2, 33, 125, 0, 80, 921, 103, 110, 18, 195, 2637, 96, 16, 1071, 18, 5, 26, 3994, 6, 582, 6842, 29, 1763, 568, 8, 30, 18, 78, 18, 29, 19, 47, 17, 3, 32, 20, 6, 18, 433, 44, 212, 63, 129, 74, 6, 0, 67, 12, 65, 1, 2, 0, 29, 6135, 9, 1237, 42, 9, 8936, 3, 2, 6, 2, 1, 2, 290, 16, 0, 30, 2, 3, 0, 15, 3, 9, 395, 2309, 106, 6, 12, 4, 8, 8, 9, 5991, 84, 2, 70, 2, 1, 3, 0, 3, 1, 3, 3, 2, 11, 2, 0, 2, 6, 2, 64, 2, 3, 3, 7, 2, 6, 2, 27, 2, 3, 2, 4, 2, 0, 4, 6, 2, 339, 3, 24, 2, 24, 2, 30, 2, 24, 2, 30, 2, 24, 2, 30, 2, 24, 2, 30, 2, 24, 2, 7, 1845, 30, 7, 5, 262, 61, 147, 44, 11, 6, 17, 0, 322, 29, 19, 43, 485, 27, 229, 29, 3, 0, 496, 6, 2, 3, 2, 1, 2, 14, 2, 196, 60, 67, 8, 0, 1205, 3, 2, 26, 2, 1, 2, 0, 3, 0, 2, 9, 2, 3, 2, 0, 2, 0, 7, 0, 5, 0, 2, 0, 2, 0, 2, 2, 2, 1, 2, 0, 3, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 1, 2, 0, 3, 3, 2, 6, 2, 3, 2, 3, 2, 0, 2, 9, 2, 16, 6, 2, 2, 4, 2, 16, 4421, 42719, 33, 4153, 7, 221, 3, 5761, 15, 7472, 16, 621, 2467, 541, 1507, 4938, 6, 4191], Qv = "\u200C\u200D\xB7\u0300-\u036F\u0387\u0483-\u0487\u0591-\u05BD\u05BF\u05C1\u05C2\u05C4\u05C5\u05C7\u0610-\u061A\u064B-\u0669\u0670\u06D6-\u06DC\u06DF-\u06E4\u06E7\u06E8\u06EA-\u06ED\u06F0-\u06F9\u0711\u0730-\u074A\u07A6-\u07B0\u07C0-\u07C9\u07EB-\u07F3\u07FD\u0816-\u0819\u081B-\u0823\u0825-\u0827\u0829-\u082D\u0859-\u085B\u0897-\u089F\u08CA-\u08E1\u08E3-\u0903\u093A-\u093C\u093E-\u094F\u0951-\u0957\u0962\u0963\u0966-\u096F\u0981-\u0983\u09BC\u09BE-\u09C4\u09C7\u09C8\u09CB-\u09CD\u09D7\u09E2\u09E3\u09E6-\u09EF\u09FE\u0A01-\u0A03\u0A3C\u0A3E-\u0A42\u0A47\u0A48\u0A4B-\u0A4D\u0A51\u0A66-\u0A71\u0A75\u0A81-\u0A83\u0ABC\u0ABE-\u0AC5\u0AC7-\u0AC9\u0ACB-\u0ACD\u0AE2\u0AE3\u0AE6-\u0AEF\u0AFA-\u0AFF\u0B01-\u0B03\u0B3C\u0B3E-\u0B44\u0B47\u0B48\u0B4B-\u0B4D\u0B55-\u0B57\u0B62\u0B63\u0B66-\u0B6F\u0B82\u0BBE-\u0BC2\u0BC6-\u0BC8\u0BCA-\u0BCD\u0BD7\u0BE6-\u0BEF\u0C00-\u0C04\u0C3C\u0C3E-\u0C44\u0C46-\u0C48\u0C4A-\u0C4D\u0C55\u0C56\u0C62\u0C63\u0C66-\u0C6F\u0C81-\u0C83\u0CBC\u0CBE-\u0CC4\u0CC6-\u0CC8\u0CCA-\u0CCD\u0CD5\u0CD6\u0CE2\u0CE3\u0CE6-\u0CEF\u0CF3\u0D00-\u0D03\u0D3B\u0D3C\u0D3E-\u0D44\u0D46-\u0D48\u0D4A-\u0D4D\u0D57\u0D62\u0D63\u0D66-\u0D6F\u0D81-\u0D83\u0DCA\u0DCF-\u0DD4\u0DD6\u0DD8-\u0DDF\u0DE6-\u0DEF\u0DF2\u0DF3\u0E31\u0E34-\u0E3A\u0E47-\u0E4E\u0E50-\u0E59\u0EB1\u0EB4-\u0EBC\u0EC8-\u0ECE\u0ED0-\u0ED9\u0F18\u0F19\u0F20-\u0F29\u0F35\u0F37\u0F39\u0F3E\u0F3F\u0F71-\u0F84\u0F86\u0F87\u0F8D-\u0F97\u0F99-\u0FBC\u0FC6\u102B-\u103E\u1040-\u1049\u1056-\u1059\u105E-\u1060\u1062-\u1064\u1067-\u106D\u1071-\u1074\u1082-\u108D\u108F-\u109D\u135D-\u135F\u1369-\u1371\u1712-\u1715\u1732-\u1734\u1752\u1753\u1772\u1773\u17B4-\u17D3\u17DD\u17E0-\u17E9\u180B-\u180D\u180F-\u1819\u18A9\u1920-\u192B\u1930-\u193B\u1946-\u194F\u19D0-\u19DA\u1A17-\u1A1B\u1A55-\u1A5E\u1A60-\u1A7C\u1A7F-\u1A89\u1A90-\u1A99\u1AB0-\u1ABD\u1ABF-\u1ACE\u1B00-\u1B04\u1B34-\u1B44\u1B50-\u1B59\u1B6B-\u1B73\u1B80-\u1B82\u1BA1-\u1BAD\u1BB0-\u1BB9\u1BE6-\u1BF3\u1C24-\u1C37\u1C40-\u1C49\u1C50-\u1C59\u1CD0-\u1CD2\u1CD4-\u1CE8\u1CED\u1CF4\u1CF7-\u1CF9\u1DC0-\u1DFF\u200C\u200D\u203F\u2040\u2054\u20D0-\u20DC\u20E1\u20E5-\u20F0\u2CEF-\u2CF1\u2D7F\u2DE0-\u2DFF\u302A-\u302F\u3099\u309A\u30FB\uA620-\uA629\uA66F\uA674-\uA67D\uA69E\uA69F\uA6F0\uA6F1\uA802\uA806\uA80B\uA823-\uA827\uA82C\uA880\uA881\uA8B4-\uA8C5\uA8D0-\uA8D9\uA8E0-\uA8F1\uA8FF-\uA909\uA926-\uA92D\uA947-\uA953\uA980-\uA983\uA9B3-\uA9C0\uA9D0-\uA9D9\uA9E5\uA9F0-\uA9F9\uAA29-\uAA36\uAA43\uAA4C\uAA4D\uAA50-\uAA59\uAA7B-\uAA7D\uAAB0\uAAB2-\uAAB4\uAAB7\uAAB8\uAABE\uAABF\uAAC1\uAAEB-\uAAEF\uAAF5\uAAF6\uABE3-\uABEA\uABEC\uABED\uABF0-\uABF9\uFB1E\uFE00-\uFE0F\uFE20-\uFE2F\uFE33\uFE34\uFE4D-\uFE4F\uFF10-\uFF19\uFF3F\uFF65", kh = "\xAA\xB5\xBA\xC0-\xD6\xD8-\xF6\xF8-\u02C1\u02C6-\u02D1\u02E0-\u02E4\u02EC\u02EE\u0370-\u0374\u0376\u0377\u037A-\u037D\u037F\u0386\u0388-\u038A\u038C\u038E-\u03A1\u03A3-\u03F5\u03F7-\u0481\u048A-\u052F\u0531-\u0556\u0559\u0560-\u0588\u05D0-\u05EA\u05EF-\u05F2\u0620-\u064A\u066E\u066F\u0671-\u06D3\u06D5\u06E5\u06E6\u06EE\u06EF\u06FA-\u06FC\u06FF\u0710\u0712-\u072F\u074D-\u07A5\u07B1\u07CA-\u07EA\u07F4\u07F5\u07FA\u0800-\u0815\u081A\u0824\u0828\u0840-\u0858\u0860-\u086A\u0870-\u0887\u0889-\u088E\u08A0-\u08C9\u0904-\u0939\u093D\u0950\u0958-\u0961\u0971-\u0980\u0985-\u098C\u098F\u0990\u0993-\u09A8\u09AA-\u09B0\u09B2\u09B6-\u09B9\u09BD\u09CE\u09DC\u09DD\u09DF-\u09E1\u09F0\u09F1\u09FC\u0A05-\u0A0A\u0A0F\u0A10\u0A13-\u0A28\u0A2A-\u0A30\u0A32\u0A33\u0A35\u0A36\u0A38\u0A39\u0A59-\u0A5C\u0A5E\u0A72-\u0A74\u0A85-\u0A8D\u0A8F-\u0A91\u0A93-\u0AA8\u0AAA-\u0AB0\u0AB2\u0AB3\u0AB5-\u0AB9\u0ABD\u0AD0\u0AE0\u0AE1\u0AF9\u0B05-\u0B0C\u0B0F\u0B10\u0B13-\u0B28\u0B2A-\u0B30\u0B32\u0B33\u0B35-\u0B39\u0B3D\u0B5C\u0B5D\u0B5F-\u0B61\u0B71\u0B83\u0B85-\u0B8A\u0B8E-\u0B90\u0B92-\u0B95\u0B99\u0B9A\u0B9C\u0B9E\u0B9F\u0BA3\u0BA4\u0BA8-\u0BAA\u0BAE-\u0BB9\u0BD0\u0C05-\u0C0C\u0C0E-\u0C10\u0C12-\u0C28\u0C2A-\u0C39\u0C3D\u0C58-\u0C5A\u0C5D\u0C60\u0C61\u0C80\u0C85-\u0C8C\u0C8E-\u0C90\u0C92-\u0CA8\u0CAA-\u0CB3\u0CB5-\u0CB9\u0CBD\u0CDD\u0CDE\u0CE0\u0CE1\u0CF1\u0CF2\u0D04-\u0D0C\u0D0E-\u0D10\u0D12-\u0D3A\u0D3D\u0D4E\u0D54-\u0D56\u0D5F-\u0D61\u0D7A-\u0D7F\u0D85-\u0D96\u0D9A-\u0DB1\u0DB3-\u0DBB\u0DBD\u0DC0-\u0DC6\u0E01-\u0E30\u0E32\u0E33\u0E40-\u0E46\u0E81\u0E82\u0E84\u0E86-\u0E8A\u0E8C-\u0EA3\u0EA5\u0EA7-\u0EB0\u0EB2\u0EB3\u0EBD\u0EC0-\u0EC4\u0EC6\u0EDC-\u0EDF\u0F00\u0F40-\u0F47\u0F49-\u0F6C\u0F88-\u0F8C\u1000-\u102A\u103F\u1050-\u1055\u105A-\u105D\u1061\u1065\u1066\u106E-\u1070\u1075-\u1081\u108E\u10A0-\u10C5\u10C7\u10CD\u10D0-\u10FA\u10FC-\u1248\u124A-\u124D\u1250-\u1256\u1258\u125A-\u125D\u1260-\u1288\u128A-\u128D\u1290-\u12B0\u12B2-\u12B5\u12B8-\u12BE\u12C0\u12C2-\u12C5\u12C8-\u12D6\u12D8-\u1310\u1312-\u1315\u1318-\u135A\u1380-\u138F\u13A0-\u13F5\u13F8-\u13FD\u1401-\u166C\u166F-\u167F\u1681-\u169A\u16A0-\u16EA\u16EE-\u16F8\u1700-\u1711\u171F-\u1731\u1740-\u1751\u1760-\u176C\u176E-\u1770\u1780-\u17B3\u17D7\u17DC\u1820-\u1878\u1880-\u18A8\u18AA\u18B0-\u18F5\u1900-\u191E\u1950-\u196D\u1970-\u1974\u1980-\u19AB\u19B0-\u19C9\u1A00-\u1A16\u1A20-\u1A54\u1AA7\u1B05-\u1B33\u1B45-\u1B4C\u1B83-\u1BA0\u1BAE\u1BAF\u1BBA-\u1BE5\u1C00-\u1C23\u1C4D-\u1C4F\u1C5A-\u1C7D\u1C80-\u1C8A\u1C90-\u1CBA\u1CBD-\u1CBF\u1CE9-\u1CEC\u1CEE-\u1CF3\u1CF5\u1CF6\u1CFA\u1D00-\u1DBF\u1E00-\u1F15\u1F18-\u1F1D\u1F20-\u1F45\u1F48-\u1F4D\u1F50-\u1F57\u1F59\u1F5B\u1F5D\u1F5F-\u1F7D\u1F80-\u1FB4\u1FB6-\u1FBC\u1FBE\u1FC2-\u1FC4\u1FC6-\u1FCC\u1FD0-\u1FD3\u1FD6-\u1FDB\u1FE0-\u1FEC\u1FF2-\u1FF4\u1FF6-\u1FFC\u2071\u207F\u2090-\u209C\u2102\u2107\u210A-\u2113\u2115\u2118-\u211D\u2124\u2126\u2128\u212A-\u2139\u213C-\u213F\u2145-\u2149\u214E\u2160-\u2188\u2C00-\u2CE4\u2CEB-\u2CEE\u2CF2\u2CF3\u2D00-\u2D25\u2D27\u2D2D\u2D30-\u2D67\u2D6F\u2D80-\u2D96\u2DA0-\u2DA6\u2DA8-\u2DAE\u2DB0-\u2DB6\u2DB8-\u2DBE\u2DC0-\u2DC6\u2DC8-\u2DCE\u2DD0-\u2DD6\u2DD8-\u2DDE\u3005-\u3007\u3021-\u3029\u3031-\u3035\u3038-\u303C\u3041-\u3096\u309B-\u309F\u30A1-\u30FA\u30FC-\u30FF\u3105-\u312F\u3131-\u318E\u31A0-\u31BF\u31F0-\u31FF\u3400-\u4DBF\u4E00-\uA48C\uA4D0-\uA4FD\uA500-\uA60C\uA610-\uA61F\uA62A\uA62B\uA640-\uA66E\uA67F-\uA69D\uA6A0-\uA6EF\uA717-\uA71F\uA722-\uA788\uA78B-\uA7CD\uA7D0\uA7D1\uA7D3\uA7D5-\uA7DC\uA7F2-\uA801\uA803-\uA805\uA807-\uA80A\uA80C-\uA822\uA840-\uA873\uA882-\uA8B3\uA8F2-\uA8F7\uA8FB\uA8FD\uA8FE\uA90A-\uA925\uA930-\uA946\uA960-\uA97C\uA984-\uA9B2\uA9CF\uA9E0-\uA9E4\uA9E6-\uA9EF\uA9FA-\uA9FE\uAA00-\uAA28\uAA40-\uAA42\uAA44-\uAA4B\uAA60-\uAA76\uAA7A\uAA7E-\uAAAF\uAAB1\uAAB5\uAAB6\uAAB9-\uAABD\uAAC0\uAAC2\uAADB-\uAADD\uAAE0-\uAAEA\uAAF2-\uAAF4\uAB01-\uAB06\uAB09-\uAB0E\uAB11-\uAB16\uAB20-\uAB26\uAB28-\uAB2E\uAB30-\uAB5A\uAB5C-\uAB69\uAB70-\uABE2\uAC00-\uD7A3\uD7B0-\uD7C6\uD7CB-\uD7FB\uF900-\uFA6D\uFA70-\uFAD9\uFB00-\uFB06\uFB13-\uFB17\uFB1D\uFB1F-\uFB28\uFB2A-\uFB36\uFB38-\uFB3C\uFB3E\uFB40\uFB41\uFB43\uFB44\uFB46-\uFBB1\uFBD3-\uFD3D\uFD50-\uFD8F\uFD92-\uFDC7\uFDF0-\uFDFB\uFE70-\uFE74\uFE76-\uFEFC\uFF21-\uFF3A\uFF41-\uFF5A\uFF66-\uFFBE\uFFC2-\uFFC7\uFFCA-\uFFCF\uFFD2-\uFFD7\uFFDA-\uFFDC", ll = { 3: "abstract boolean byte char class double enum export extends final float goto implements import int interface long native package private protected public short static super synchronized throws transient volatile", 5: "class enum extends super const export import", 6: "enum", strict: "implements interface let package private protected public static yield", strictBind: "eval arguments" }, ul = "break case catch continue debugger default do else finally for function if return switch throw try var while with null true false instanceof typeof void delete new in this", Yv = { 5: ul, "5module": ul + " export import", 6: ul + " const class extends export import super" }, Zv = /^in(stanceof)?$/, e1 = new RegExp("[" + kh + "]"), t1 = new RegExp("[" + kh + Qv + "]");
function ml(e, t) {
  for (var i = 65536, s = 0; s < t.length; s += 2) {
    if (i += t[s], i > e) return false;
    if (i += t[s + 1], i >= e) return true;
  }
  return false;
}
function Pt(e, t) {
  return e < 65 ? e === 36 : e < 91 ? true : e < 97 ? e === 95 : e < 123 ? true : e <= 65535 ? e >= 170 && e1.test(String.fromCharCode(e)) : t === false ? false : ml(e, Eh);
}
function Ft(e, t) {
  return e < 48 ? e === 36 : e < 58 ? true : e < 65 ? false : e < 91 ? true : e < 97 ? e === 95 : e < 123 ? true : e <= 65535 ? e >= 170 && t1.test(String.fromCharCode(e)) : t === false ? false : ml(e, Eh) || ml(e, Jv);
}
var Ce = function(t, i) {
  i === void 0 && (i = {}), this.label = t, this.keyword = i.keyword, this.beforeExpr = !!i.beforeExpr, this.startsExpr = !!i.startsExpr, this.isLoop = !!i.isLoop, this.isAssign = !!i.isAssign, this.prefix = !!i.prefix, this.postfix = !!i.postfix, this.binop = i.binop || null, this.updateContext = null;
};
function ct(e, t) {
  return new Ce(e, { beforeExpr: true, binop: t });
}
var dt = { beforeExpr: true }, it = { startsExpr: true }, Kt = {};
function Te(e, t) {
  return t === void 0 && (t = {}), t.keyword = e, Kt[e] = new Ce(e, t);
}
var d = { num: new Ce("num", it), regexp: new Ce("regexp", it), string: new Ce("string", it), name: new Ce("name", it), privateId: new Ce("privateId", it), eof: new Ce("eof"), bracketL: new Ce("[", { beforeExpr: true, startsExpr: true }), bracketR: new Ce("]"), braceL: new Ce("{", { beforeExpr: true, startsExpr: true }), braceR: new Ce("}"), parenL: new Ce("(", { beforeExpr: true, startsExpr: true }), parenR: new Ce(")"), comma: new Ce(",", dt), semi: new Ce(";", dt), colon: new Ce(":", dt), dot: new Ce("."), question: new Ce("?", dt), questionDot: new Ce("?."), arrow: new Ce("=>", dt), template: new Ce("template"), invalidTemplate: new Ce("invalidTemplate"), ellipsis: new Ce("...", dt), backQuote: new Ce("`", it), dollarBraceL: new Ce("${", { beforeExpr: true, startsExpr: true }), eq: new Ce("=", { beforeExpr: true, isAssign: true }), assign: new Ce("_=", { beforeExpr: true, isAssign: true }), incDec: new Ce("++/--", { prefix: true, postfix: true, startsExpr: true }), prefix: new Ce("!/~", { beforeExpr: true, prefix: true, startsExpr: true }), logicalOR: ct("||", 1), logicalAND: ct("&&", 2), bitwiseOR: ct("|", 3), bitwiseXOR: ct("^", 4), bitwiseAND: ct("&", 5), equality: ct("==/!=/===/!==", 6), relational: ct("</>/<=/>=", 7), bitShift: ct("<</>>/>>>", 8), plusMin: new Ce("+/-", { beforeExpr: true, binop: 9, prefix: true, startsExpr: true }), modulo: ct("%", 10), star: ct("*", 10), slash: ct("/", 10), starstar: new Ce("**", { beforeExpr: true }), coalesce: ct("??", 1), _break: Te("break"), _case: Te("case", dt), _catch: Te("catch"), _continue: Te("continue"), _debugger: Te("debugger"), _default: Te("default", dt), _do: Te("do", { isLoop: true, beforeExpr: true }), _else: Te("else", dt), _finally: Te("finally"), _for: Te("for", { isLoop: true }), _function: Te("function", it), _if: Te("if"), _return: Te("return", dt), _switch: Te("switch"), _throw: Te("throw", dt), _try: Te("try"), _var: Te("var"), _const: Te("const"), _while: Te("while", { isLoop: true }), _with: Te("with"), _new: Te("new", { beforeExpr: true, startsExpr: true }), _this: Te("this", it), _super: Te("super", it), _class: Te("class", it), _extends: Te("extends", dt), _export: Te("export"), _import: Te("import", it), _null: Te("null", it), _true: Te("true", it), _false: Te("false", it), _in: Te("in", { beforeExpr: true, binop: 7 }), _instanceof: Te("instanceof", { beforeExpr: true, binop: 7 }), _typeof: Te("typeof", { beforeExpr: true, prefix: true, startsExpr: true }), _void: Te("void", { beforeExpr: true, prefix: true, startsExpr: true }), _delete: Te("delete", { beforeExpr: true, prefix: true, startsExpr: true }) }, ut = /\r\n?|\n|\u2028|\u2029/, Ih = new RegExp(ut.source, "g");
function $t(e) {
  return e === 10 || e === 13 || e === 8232 || e === 8233;
}
function Mh(e, t, i) {
  i === void 0 && (i = e.length);
  for (var s = t; s < i; s++) {
    var l = e.charCodeAt(s);
    if ($t(l)) return s < i - 1 && l === 13 && e.charCodeAt(s + 1) === 10 ? s + 2 : s + 1;
  }
  return -1;
}
var Cl = /[\u1680\u2000-\u200a\u202f\u205f\u3000\ufeff]/, ft = /(?:\s|\/\/.*|\/\*[^]*?\*\/)*/g, Lh = Object.prototype, r1 = Lh.hasOwnProperty, a1 = Lh.toString, er = Object.hasOwn || function(e, t) {
  return r1.call(e, t);
}, $l = Array.isArray || function(e) {
  return a1.call(e) === "[object Array]";
}, Hl = /* @__PURE__ */ Object.create(null);
function Ot(e) {
  return Hl[e] || (Hl[e] = new RegExp("^(?:" + e.replace(/ /g, "|") + ")$"));
}
function Et(e) {
  return e <= 65535 ? String.fromCharCode(e) : (e -= 65536, String.fromCharCode((e >> 10) + 55296, (e & 1023) + 56320));
}
var i1 = /(?:[\uD800-\uDBFF](?![\uDC00-\uDFFF])|(?:[^\uD800-\uDBFF]|^)[\uDC00-\uDFFF])/, Jt = function(t, i) {
  this.line = t, this.column = i;
};
Jt.prototype.offset = function(t) {
  return new Jt(this.line, this.column + t);
};
var Po = function(t, i, s) {
  this.start = i, this.end = s, t.sourceFile !== null && (this.source = t.sourceFile);
};
function Pl(e, t) {
  for (var i = 1, s = 0; ; ) {
    var l = Mh(e, s, t);
    if (l < 0) return new Jt(i, t - s);
    ++i, s = l;
  }
}
var Wo = { ecmaVersion: null, sourceType: "script", onInsertedSemicolon: null, onTrailingComma: null, allowReserved: null, allowReturnOutsideFunction: false, allowImportExportEverywhere: false, allowAwaitOutsideFunction: null, allowSuperOutsideMethod: null, allowHashBang: false, checkPrivateFields: true, locations: false, onToken: null, onComment: null, ranges: false, program: null, sourceFile: null, directSourceFile: null, preserveParens: false }, Ul = false;
function s1(e) {
  var t = {};
  for (var i in Wo) t[i] = e && er(e, i) ? e[i] : Wo[i];
  if (t.ecmaVersion === "latest" ? t.ecmaVersion = 1e8 : t.ecmaVersion == null ? (!Ul && typeof console == "object" && console.warn && (Ul = true, console.warn(`Since Acorn 8.0.0, options.ecmaVersion is required.
Defaulting to 2020, but this will stop working in the future.`)), t.ecmaVersion = 11) : t.ecmaVersion >= 2015 && (t.ecmaVersion -= 2009), t.allowReserved == null && (t.allowReserved = t.ecmaVersion < 5), (!e || e.allowHashBang == null) && (t.allowHashBang = t.ecmaVersion >= 14), $l(t.onToken)) {
    var s = t.onToken;
    t.onToken = function(l) {
      return s.push(l);
    };
  }
  return $l(t.onComment) && (t.onComment = n1(t, t.onComment)), t;
}
function n1(e, t) {
  return function(i, s, l, p, m, w) {
    var _ = { type: i ? "Block" : "Line", value: s, start: l, end: p };
    e.locations && (_.loc = new Po(this, m, w)), e.ranges && (_.range = [l, p]), t.push(_);
  };
}
var Co = 1, tr = 2, ql = 4, Oh = 8, Nh = 16, Dh = 32, Tl = 64, jh = 128, qo = 256, Sl = Co | tr | qo;
function wl(e, t) {
  return tr | (e ? ql : 0) | (t ? Oh : 0);
}
var zo = 0, Al = 1, Lt = 2, Bh = 3, Fh = 4, Vh = 5, He = function(t, i, s) {
  this.options = t = s1(t), this.sourceFile = t.sourceFile, this.keywords = Ot(Yv[t.ecmaVersion >= 6 ? 6 : t.sourceType === "module" ? "5module" : 5]);
  var l = "";
  t.allowReserved !== true && (l = ll[t.ecmaVersion >= 6 ? 6 : t.ecmaVersion === 5 ? 5 : 3], t.sourceType === "module" && (l += " await")), this.reservedWords = Ot(l);
  var p = (l ? l + " " : "") + ll.strict;
  this.reservedWordsStrict = Ot(p), this.reservedWordsStrictBind = Ot(p + " " + ll.strictBind), this.input = String(i), this.containsEsc = false, s ? (this.pos = s, this.lineStart = this.input.lastIndexOf(`
`, s - 1) + 1, this.curLine = this.input.slice(0, this.lineStart).split(ut).length) : (this.pos = this.lineStart = 0, this.curLine = 1), this.type = d.eof, this.value = null, this.start = this.end = this.pos, this.startLoc = this.endLoc = this.curPosition(), this.lastTokEndLoc = this.lastTokStartLoc = null, this.lastTokStart = this.lastTokEnd = this.pos, this.context = this.initialContext(), this.exprAllowed = true, this.inModule = t.sourceType === "module", this.strict = this.inModule || this.strictDirective(this.pos), this.potentialArrowAt = -1, this.potentialArrowInForAwait = false, this.yieldPos = this.awaitPos = this.awaitIdentPos = 0, this.labels = [], this.undefinedExports = /* @__PURE__ */ Object.create(null), this.pos === 0 && t.allowHashBang && this.input.slice(0, 2) === "#!" && this.skipLineComment(2), this.scopeStack = [], this.enterScope(Co), this.regexpState = null, this.privateNameStack = [];
}, qt = { inFunction: { configurable: true }, inGenerator: { configurable: true }, inAsync: { configurable: true }, canAwait: { configurable: true }, allowSuper: { configurable: true }, allowDirectSuper: { configurable: true }, treatFunctionsAsVar: { configurable: true }, allowNewDotTarget: { configurable: true }, inClassStaticBlock: { configurable: true } };
He.prototype.parse = function() {
  var t = this.options.program || this.startNode();
  return this.nextToken(), this.parseTopLevel(t);
};
qt.inFunction.get = function() {
  return (this.currentVarScope().flags & tr) > 0;
};
qt.inGenerator.get = function() {
  return (this.currentVarScope().flags & Oh) > 0 && !this.currentVarScope().inClassFieldInit;
};
qt.inAsync.get = function() {
  return (this.currentVarScope().flags & ql) > 0 && !this.currentVarScope().inClassFieldInit;
};
qt.canAwait.get = function() {
  for (var e = this.scopeStack.length - 1; e >= 0; e--) {
    var t = this.scopeStack[e];
    if (t.inClassFieldInit || t.flags & qo) return false;
    if (t.flags & tr) return (t.flags & ql) > 0;
  }
  return this.inModule && this.options.ecmaVersion >= 13 || this.options.allowAwaitOutsideFunction;
};
qt.allowSuper.get = function() {
  var e = this.currentThisScope(), t = e.flags, i = e.inClassFieldInit;
  return (t & Tl) > 0 || i || this.options.allowSuperOutsideMethod;
};
qt.allowDirectSuper.get = function() {
  return (this.currentThisScope().flags & jh) > 0;
};
qt.treatFunctionsAsVar.get = function() {
  return this.treatFunctionsAsVarInScope(this.currentScope());
};
qt.allowNewDotTarget.get = function() {
  var e = this.currentThisScope(), t = e.flags, i = e.inClassFieldInit;
  return (t & (tr | qo)) > 0 || i;
};
qt.inClassStaticBlock.get = function() {
  return (this.currentVarScope().flags & qo) > 0;
};
He.extend = function() {
  for (var t = [], i = arguments.length; i--; ) t[i] = arguments[i];
  for (var s = this, l = 0; l < t.length; l++) s = t[l](s);
  return s;
};
He.parse = function(t, i) {
  return new this(i, t).parse();
};
He.parseExpressionAt = function(t, i, s) {
  var l = new this(s, t, i);
  return l.nextToken(), l.parseExpression();
};
He.tokenizer = function(t, i) {
  return new this(i, t);
};
Object.defineProperties(He.prototype, qt);
var at = He.prototype, o1 = /^(?:'((?:\\[^]|[^'\\])*?)'|"((?:\\[^]|[^"\\])*?)")/;
at.strictDirective = function(e) {
  if (this.options.ecmaVersion < 5) return false;
  for (; ; ) {
    ft.lastIndex = e, e += ft.exec(this.input)[0].length;
    var t = o1.exec(this.input.slice(e));
    if (!t) return false;
    if ((t[1] || t[2]) === "use strict") {
      ft.lastIndex = e + t[0].length;
      var i = ft.exec(this.input), s = i.index + i[0].length, l = this.input.charAt(s);
      return l === ";" || l === "}" || ut.test(i[0]) && !(/[(`.[+\-/*%<>=,?^&]/.test(l) || l === "!" && this.input.charAt(s + 1) === "=");
    }
    e += t[0].length, ft.lastIndex = e, e += ft.exec(this.input)[0].length, this.input[e] === ";" && e++;
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
  return this.type === d.eof || this.type === d.braceR || ut.test(this.input.slice(this.lastTokEnd, this.start));
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
    var i = t ? e.parenthesizedAssign : e.parenthesizedBind;
    i > -1 && this.raiseRecoverable(i, t ? "Assigning to rvalue" : "Parenthesized pattern");
  }
};
at.checkExpressionErrors = function(e, t) {
  if (!e) return false;
  var i = e.shorthandAssign, s = e.doubleProto;
  if (!t) return i >= 0 || s >= 0;
  i >= 0 && this.raise(i, "Shorthand property assignments are valid only in destructuring patterns"), s >= 0 && this.raiseRecoverable(s, "Redefinition of __proto__ property");
};
at.checkYieldAwaitInDefaultParams = function() {
  this.yieldPos && (!this.awaitPos || this.yieldPos < this.awaitPos) && this.raise(this.yieldPos, "Yield expression cannot be a default value"), this.awaitPos && this.raise(this.awaitPos, "Await expression cannot be a default value");
};
at.isSimpleAssignTarget = function(e) {
  return e.type === "ParenthesizedExpression" ? this.isSimpleAssignTarget(e.expression) : e.type === "Identifier" || e.type === "MemberExpression";
};
var ue = He.prototype;
ue.parseTopLevel = function(e) {
  var t = /* @__PURE__ */ Object.create(null);
  for (e.body || (e.body = []); this.type !== d.eof; ) {
    var i = this.parseStatement(null, true, t);
    e.body.push(i);
  }
  if (this.inModule) for (var s = 0, l = Object.keys(this.undefinedExports); s < l.length; s += 1) {
    var p = l[s];
    this.raiseRecoverable(this.undefinedExports[p].start, "Export '" + p + "' is not defined");
  }
  return this.adaptDirectivePrologue(e.body), this.next(), e.sourceType = this.options.sourceType, this.finishNode(e, "Program");
};
var El = { kind: "loop" }, l1 = { kind: "switch" };
ue.isLet = function(e) {
  if (this.options.ecmaVersion < 6 || !this.isContextual("let")) return false;
  ft.lastIndex = this.pos;
  var t = ft.exec(this.input), i = this.pos + t[0].length, s = this.input.charCodeAt(i);
  if (s === 91 || s === 92) return true;
  if (e) return false;
  if (s === 123 || s > 55295 && s < 56320) return true;
  if (Pt(s, true)) {
    for (var l = i + 1; Ft(s = this.input.charCodeAt(l), true); ) ++l;
    if (s === 92 || s > 55295 && s < 56320) return true;
    var p = this.input.slice(i, l);
    if (!Zv.test(p)) return true;
  }
  return false;
};
ue.isAsyncFunction = function() {
  if (this.options.ecmaVersion < 8 || !this.isContextual("async")) return false;
  ft.lastIndex = this.pos;
  var e = ft.exec(this.input), t = this.pos + e[0].length, i;
  return !ut.test(this.input.slice(this.pos, t)) && this.input.slice(t, t + 8) === "function" && (t + 8 === this.input.length || !(Ft(i = this.input.charCodeAt(t + 8)) || i > 55295 && i < 56320));
};
ue.parseStatement = function(e, t, i) {
  var s = this.type, l = this.startNode(), p;
  switch (this.isLet(e) && (s = d._var, p = "let"), s) {
    case d._break:
    case d._continue:
      return this.parseBreakContinueStatement(l, s.keyword);
    case d._debugger:
      return this.parseDebuggerStatement(l);
    case d._do:
      return this.parseDoStatement(l);
    case d._for:
      return this.parseForStatement(l);
    case d._function:
      return e && (this.strict || e !== "if" && e !== "label") && this.options.ecmaVersion >= 6 && this.unexpected(), this.parseFunctionStatement(l, false, !e);
    case d._class:
      return e && this.unexpected(), this.parseClass(l, true);
    case d._if:
      return this.parseIfStatement(l);
    case d._return:
      return this.parseReturnStatement(l);
    case d._switch:
      return this.parseSwitchStatement(l);
    case d._throw:
      return this.parseThrowStatement(l);
    case d._try:
      return this.parseTryStatement(l);
    case d._const:
    case d._var:
      return p = p || this.value, e && p !== "var" && this.unexpected(), this.parseVarStatement(l, p);
    case d._while:
      return this.parseWhileStatement(l);
    case d._with:
      return this.parseWithStatement(l);
    case d.braceL:
      return this.parseBlock(true, l);
    case d.semi:
      return this.parseEmptyStatement(l);
    case d._export:
    case d._import:
      if (this.options.ecmaVersion > 10 && s === d._import) {
        ft.lastIndex = this.pos;
        var m = ft.exec(this.input), w = this.pos + m[0].length, _ = this.input.charCodeAt(w);
        if (_ === 40 || _ === 46) return this.parseExpressionStatement(l, this.parseExpression());
      }
      return this.options.allowImportExportEverywhere || (t || this.raise(this.start, "'import' and 'export' may only appear at the top level"), this.inModule || this.raise(this.start, "'import' and 'export' may appear only with 'sourceType: module'")), s === d._import ? this.parseImport(l) : this.parseExport(l, i);
    default:
      if (this.isAsyncFunction()) return e && this.unexpected(), this.next(), this.parseFunctionStatement(l, true, !e);
      var o = this.value, b = this.parseExpression();
      return s === d.name && b.type === "Identifier" && this.eat(d.colon) ? this.parseLabeledStatement(l, o, b, e) : this.parseExpressionStatement(l, b);
  }
};
ue.parseBreakContinueStatement = function(e, t) {
  var i = t === "break";
  this.next(), this.eat(d.semi) || this.insertSemicolon() ? e.label = null : this.type !== d.name ? this.unexpected() : (e.label = this.parseIdent(), this.semicolon());
  for (var s = 0; s < this.labels.length; ++s) {
    var l = this.labels[s];
    if ((e.label == null || l.name === e.label.name) && (l.kind != null && (i || l.kind === "loop") || e.label && i)) break;
  }
  return s === this.labels.length && this.raise(e.start, "Unsyntactic " + t), this.finishNode(e, i ? "BreakStatement" : "ContinueStatement");
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
  var i = this.isLet();
  if (this.type === d._var || this.type === d._const || i) {
    var s = this.startNode(), l = i ? "let" : this.value;
    return this.next(), this.parseVar(s, true, l), this.finishNode(s, "VariableDeclaration"), (this.type === d._in || this.options.ecmaVersion >= 6 && this.isContextual("of")) && s.declarations.length === 1 ? (this.options.ecmaVersion >= 9 && (this.type === d._in ? t > -1 && this.unexpected(t) : e.await = t > -1), this.parseForIn(e, s)) : (t > -1 && this.unexpected(t), this.parseFor(e, s));
  }
  var p = this.isContextual("let"), m = false, w = this.containsEsc, _ = new Zo(), o = this.start, b = t > -1 ? this.parseExprSubscripts(_, "await") : this.parseExpression(true, _);
  return this.type === d._in || (m = this.options.ecmaVersion >= 6 && this.isContextual("of")) ? (t > -1 ? (this.type === d._in && this.unexpected(t), e.await = true) : m && this.options.ecmaVersion >= 8 && (b.start === o && !w && b.type === "Identifier" && b.name === "async" ? this.unexpected() : this.options.ecmaVersion >= 9 && (e.await = false)), p && m && this.raise(b.start, "The left-hand side of a for-of loop may not start with 'let'."), this.toAssignable(b, false, _), this.checkLValPattern(b), this.parseForIn(e, b)) : (this.checkExpressionErrors(_, true), t > -1 && this.unexpected(t), this.parseFor(e, b));
};
ue.parseFunctionStatement = function(e, t, i) {
  return this.next(), this.parseFunction(e, Ro | (i ? 0 : vl), false, t);
};
ue.parseIfStatement = function(e) {
  return this.next(), e.test = this.parseParenExpression(), e.consequent = this.parseStatement("if"), e.alternate = this.eat(d._else) ? this.parseStatement("if") : null, this.finishNode(e, "IfStatement");
};
ue.parseReturnStatement = function(e) {
  return !this.inFunction && !this.options.allowReturnOutsideFunction && this.raise(this.start, "'return' outside of function"), this.next(), this.eat(d.semi) || this.insertSemicolon() ? e.argument = null : (e.argument = this.parseExpression(), this.semicolon()), this.finishNode(e, "ReturnStatement");
};
ue.parseSwitchStatement = function(e) {
  this.next(), e.discriminant = this.parseParenExpression(), e.cases = [], this.expect(d.braceL), this.labels.push(l1), this.enterScope(0);
  for (var t, i = false; this.type !== d.braceR; ) if (this.type === d._case || this.type === d._default) {
    var s = this.type === d._case;
    t && this.finishNode(t, "SwitchCase"), e.cases.push(t = this.startNode()), t.consequent = [], this.next(), s ? t.test = this.parseExpression() : (i && this.raiseRecoverable(this.lastTokStart, "Multiple default clauses"), i = true, t.test = null), this.expect(d.colon);
  } else t || this.unexpected(), t.consequent.push(this.parseStatement(null));
  return this.exitScope(), t && this.finishNode(t, "SwitchCase"), this.next(), this.labels.pop(), this.finishNode(e, "SwitchStatement");
};
ue.parseThrowStatement = function(e) {
  return this.next(), ut.test(this.input.slice(this.lastTokEnd, this.start)) && this.raise(this.lastTokEnd, "Illegal newline after throw"), e.argument = this.parseExpression(), this.semicolon(), this.finishNode(e, "ThrowStatement");
};
var u1 = [];
ue.parseCatchClauseParam = function() {
  var e = this.parseBindingAtom(), t = e.type === "Identifier";
  return this.enterScope(t ? Dh : 0), this.checkLValPattern(e, t ? Fh : Lt), this.expect(d.parenR), e;
};
ue.parseTryStatement = function(e) {
  if (this.next(), e.block = this.parseBlock(), e.handler = null, this.type === d._catch) {
    var t = this.startNode();
    this.next(), this.eat(d.parenL) ? t.param = this.parseCatchClauseParam() : (this.options.ecmaVersion < 10 && this.unexpected(), t.param = null, this.enterScope(0)), t.body = this.parseBlock(false), this.exitScope(), e.handler = this.finishNode(t, "CatchClause");
  }
  return e.finalizer = this.eat(d._finally) ? this.parseBlock() : null, !e.handler && !e.finalizer && this.raise(e.start, "Missing catch or finally clause"), this.finishNode(e, "TryStatement");
};
ue.parseVarStatement = function(e, t, i) {
  return this.next(), this.parseVar(e, false, t, i), this.semicolon(), this.finishNode(e, "VariableDeclaration");
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
ue.parseLabeledStatement = function(e, t, i, s) {
  for (var l = 0, p = this.labels; l < p.length; l += 1) {
    var m = p[l];
    m.name === t && this.raise(i.start, "Label '" + t + "' is already declared");
  }
  for (var w = this.type.isLoop ? "loop" : this.type === d._switch ? "switch" : null, _ = this.labels.length - 1; _ >= 0; _--) {
    var o = this.labels[_];
    if (o.statementStart === e.start) o.statementStart = this.start, o.kind = w;
    else break;
  }
  return this.labels.push({ name: t, kind: w, statementStart: this.start }), e.body = this.parseStatement(s ? s.indexOf("label") === -1 ? s + "label" : s : "label"), this.labels.pop(), e.label = i, this.finishNode(e, "LabeledStatement");
};
ue.parseExpressionStatement = function(e, t) {
  return e.expression = t, this.semicolon(), this.finishNode(e, "ExpressionStatement");
};
ue.parseBlock = function(e, t, i) {
  for (e === void 0 && (e = true), t === void 0 && (t = this.startNode()), t.body = [], this.expect(d.braceL), e && this.enterScope(0); this.type !== d.braceR; ) {
    var s = this.parseStatement(null);
    t.body.push(s);
  }
  return i && (this.strict = false), this.next(), e && this.exitScope(), this.finishNode(t, "BlockStatement");
};
ue.parseFor = function(e, t) {
  return e.init = t, this.expect(d.semi), e.test = this.type === d.semi ? null : this.parseExpression(), this.expect(d.semi), e.update = this.type === d.parenR ? null : this.parseExpression(), this.expect(d.parenR), e.body = this.parseStatement("for"), this.exitScope(), this.labels.pop(), this.finishNode(e, "ForStatement");
};
ue.parseForIn = function(e, t) {
  var i = this.type === d._in;
  return this.next(), t.type === "VariableDeclaration" && t.declarations[0].init != null && (!i || this.options.ecmaVersion < 8 || this.strict || t.kind !== "var" || t.declarations[0].id.type !== "Identifier") && this.raise(t.start, (i ? "for-in" : "for-of") + " loop variable declaration may not have an initializer"), e.left = t, e.right = i ? this.parseExpression() : this.parseMaybeAssign(), this.expect(d.parenR), e.body = this.parseStatement("for"), this.exitScope(), this.labels.pop(), this.finishNode(e, i ? "ForInStatement" : "ForOfStatement");
};
ue.parseVar = function(e, t, i, s) {
  for (e.declarations = [], e.kind = i; ; ) {
    var l = this.startNode();
    if (this.parseVarId(l, i), this.eat(d.eq) ? l.init = this.parseMaybeAssign(t) : !s && i === "const" && !(this.type === d._in || this.options.ecmaVersion >= 6 && this.isContextual("of")) ? this.unexpected() : !s && l.id.type !== "Identifier" && !(t && (this.type === d._in || this.isContextual("of"))) ? this.raise(this.lastTokEnd, "Complex binding patterns require an initialization value") : l.init = null, e.declarations.push(this.finishNode(l, "VariableDeclarator")), !this.eat(d.comma)) break;
  }
  return e;
};
ue.parseVarId = function(e, t) {
  e.id = this.parseBindingAtom(), this.checkLValPattern(e.id, t === "var" ? Al : Lt, false);
};
var Ro = 1, vl = 2, $h = 4;
ue.parseFunction = function(e, t, i, s, l) {
  this.initFunction(e), (this.options.ecmaVersion >= 9 || this.options.ecmaVersion >= 6 && !s) && (this.type === d.star && t & vl && this.unexpected(), e.generator = this.eat(d.star)), this.options.ecmaVersion >= 8 && (e.async = !!s), t & Ro && (e.id = t & $h && this.type !== d.name ? null : this.parseIdent(), e.id && !(t & vl) && this.checkLValSimple(e.id, this.strict || e.generator || e.async ? this.treatFunctionsAsVar ? Al : Lt : Bh));
  var p = this.yieldPos, m = this.awaitPos, w = this.awaitIdentPos;
  return this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0, this.enterScope(wl(e.async, e.generator)), t & Ro || (e.id = this.type === d.name ? this.parseIdent() : null), this.parseFunctionParams(e), this.parseFunctionBody(e, i, false, l), this.yieldPos = p, this.awaitPos = m, this.awaitIdentPos = w, this.finishNode(e, t & Ro ? "FunctionDeclaration" : "FunctionExpression");
};
ue.parseFunctionParams = function(e) {
  this.expect(d.parenL), e.params = this.parseBindingList(d.parenR, false, this.options.ecmaVersion >= 8), this.checkYieldAwaitInDefaultParams();
};
ue.parseClass = function(e, t) {
  this.next();
  var i = this.strict;
  this.strict = true, this.parseClassId(e, t), this.parseClassSuper(e);
  var s = this.enterClassBody(), l = this.startNode(), p = false;
  for (l.body = [], this.expect(d.braceL); this.type !== d.braceR; ) {
    var m = this.parseClassElement(e.superClass !== null);
    m && (l.body.push(m), m.type === "MethodDefinition" && m.kind === "constructor" ? (p && this.raiseRecoverable(m.start, "Duplicate constructor in the same class"), p = true) : m.key && m.key.type === "PrivateIdentifier" && c1(s, m) && this.raiseRecoverable(m.key.start, "Identifier '#" + m.key.name + "' has already been declared"));
  }
  return this.strict = i, this.next(), e.body = this.finishNode(l, "ClassBody"), this.exitClassBody(), this.finishNode(e, t ? "ClassDeclaration" : "ClassExpression");
};
ue.parseClassElement = function(e) {
  if (this.eat(d.semi)) return null;
  var t = this.options.ecmaVersion, i = this.startNode(), s = "", l = false, p = false, m = "method", w = false;
  if (this.eatContextual("static")) {
    if (t >= 13 && this.eat(d.braceL)) return this.parseClassStaticBlock(i), i;
    this.isClassElementNameStart() || this.type === d.star ? w = true : s = "static";
  }
  if (i.static = w, !s && t >= 8 && this.eatContextual("async") && ((this.isClassElementNameStart() || this.type === d.star) && !this.canInsertSemicolon() ? p = true : s = "async"), !s && (t >= 9 || !p) && this.eat(d.star) && (l = true), !s && !p && !l) {
    var _ = this.value;
    (this.eatContextual("get") || this.eatContextual("set")) && (this.isClassElementNameStart() ? m = _ : s = _);
  }
  if (s ? (i.computed = false, i.key = this.startNodeAt(this.lastTokStart, this.lastTokStartLoc), i.key.name = s, this.finishNode(i.key, "Identifier")) : this.parseClassElementName(i), t < 13 || this.type === d.parenL || m !== "method" || l || p) {
    var o = !i.static && Xo(i, "constructor"), b = o && e;
    o && m !== "method" && this.raise(i.key.start, "Constructor can't have get/set modifier"), i.kind = o ? "constructor" : m, this.parseClassMethod(i, l, p, b);
  } else this.parseClassField(i);
  return i;
};
ue.isClassElementNameStart = function() {
  return this.type === d.name || this.type === d.privateId || this.type === d.num || this.type === d.string || this.type === d.bracketL || this.type.keyword;
};
ue.parseClassElementName = function(e) {
  this.type === d.privateId ? (this.value === "constructor" && this.raise(this.start, "Classes can't have an element named '#constructor'"), e.computed = false, e.key = this.parsePrivateIdent()) : this.parsePropertyName(e);
};
ue.parseClassMethod = function(e, t, i, s) {
  var l = e.key;
  e.kind === "constructor" ? (t && this.raise(l.start, "Constructor can't be a generator"), i && this.raise(l.start, "Constructor can't be an async method")) : e.static && Xo(e, "prototype") && this.raise(l.start, "Classes may not have a static property named prototype");
  var p = e.value = this.parseMethod(t, i, s);
  return e.kind === "get" && p.params.length !== 0 && this.raiseRecoverable(p.start, "getter should have no params"), e.kind === "set" && p.params.length !== 1 && this.raiseRecoverable(p.start, "setter should have exactly one param"), e.kind === "set" && p.params[0].type === "RestElement" && this.raiseRecoverable(p.params[0].start, "Setter cannot use rest params"), this.finishNode(e, "MethodDefinition");
};
ue.parseClassField = function(e) {
  if (Xo(e, "constructor") ? this.raise(e.key.start, "Classes can't have a field named 'constructor'") : e.static && Xo(e, "prototype") && this.raise(e.key.start, "Classes can't have a static field named 'prototype'"), this.eat(d.eq)) {
    var t = this.currentThisScope(), i = t.inClassFieldInit;
    t.inClassFieldInit = true, e.value = this.parseMaybeAssign(), t.inClassFieldInit = i;
  } else e.value = null;
  return this.semicolon(), this.finishNode(e, "PropertyDefinition");
};
ue.parseClassStaticBlock = function(e) {
  e.body = [];
  var t = this.labels;
  for (this.labels = [], this.enterScope(qo | Tl); this.type !== d.braceR; ) {
    var i = this.parseStatement(null);
    e.body.push(i);
  }
  return this.next(), this.exitScope(), this.labels = t, this.finishNode(e, "StaticBlock");
};
ue.parseClassId = function(e, t) {
  this.type === d.name ? (e.id = this.parseIdent(), t && this.checkLValSimple(e.id, Lt, false)) : (t === true && this.unexpected(), e.id = null);
};
ue.parseClassSuper = function(e) {
  e.superClass = this.eat(d._extends) ? this.parseExprSubscripts(null, false) : null;
};
ue.enterClassBody = function() {
  var e = { declared: /* @__PURE__ */ Object.create(null), used: [] };
  return this.privateNameStack.push(e), e.declared;
};
ue.exitClassBody = function() {
  var e = this.privateNameStack.pop(), t = e.declared, i = e.used;
  if (this.options.checkPrivateFields) for (var s = this.privateNameStack.length, l = s === 0 ? null : this.privateNameStack[s - 1], p = 0; p < i.length; ++p) {
    var m = i[p];
    er(t, m.name) || (l ? l.used.push(m) : this.raiseRecoverable(m.start, "Private field '#" + m.name + "' must be declared in an enclosing class"));
  }
};
function c1(e, t) {
  var i = t.key.name, s = e[i], l = "true";
  return t.type === "MethodDefinition" && (t.kind === "get" || t.kind === "set") && (l = (t.static ? "s" : "i") + t.kind), s === "iget" && l === "iset" || s === "iset" && l === "iget" || s === "sget" && l === "sset" || s === "sset" && l === "sget" ? (e[i] = "true", false) : s ? true : (e[i] = l, false);
}
function Xo(e, t) {
  var i = e.computed, s = e.key;
  return !i && (s.type === "Identifier" && s.name === t || s.type === "Literal" && s.value === t);
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
      for (var i = 0, s = e.specifiers; i < s.length; i += 1) {
        var l = s[i];
        this.checkUnreserved(l.local), this.checkLocalExport(l.local), l.local.type === "Literal" && this.raise(l.local.start, "A string literal cannot be used as an exported binding without `from`.");
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
    var i = this.startNode();
    return this.parseClass(i, "nullableID");
  } else {
    var s = this.parseMaybeAssign();
    return this.semicolon(), s;
  }
};
ue.checkExport = function(e, t, i) {
  e && (typeof t != "string" && (t = t.type === "Identifier" ? t.name : t.value), er(e, t) && this.raiseRecoverable(i, "Duplicate export '" + t + "'"), e[t] = true);
};
ue.checkPatternExport = function(e, t) {
  var i = t.type;
  if (i === "Identifier") this.checkExport(e, t, t.start);
  else if (i === "ObjectPattern") for (var s = 0, l = t.properties; s < l.length; s += 1) {
    var p = l[s];
    this.checkPatternExport(e, p);
  }
  else if (i === "ArrayPattern") for (var m = 0, w = t.elements; m < w.length; m += 1) {
    var _ = w[m];
    _ && this.checkPatternExport(e, _);
  }
  else i === "Property" ? this.checkPatternExport(e, t.value) : i === "AssignmentPattern" ? this.checkPatternExport(e, t.left) : i === "RestElement" && this.checkPatternExport(e, t.argument);
};
ue.checkVariableExport = function(e, t) {
  if (e) for (var i = 0, s = t; i < s.length; i += 1) {
    var l = s[i];
    this.checkPatternExport(e, l.id);
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
  var t = [], i = true;
  for (this.expect(d.braceL); !this.eat(d.braceR); ) {
    if (i) i = false;
    else if (this.expect(d.comma), this.afterTrailingComma(d.braceR)) break;
    t.push(this.parseExportSpecifier(e));
  }
  return t;
};
ue.parseImport = function(e) {
  return this.next(), this.type === d.string ? (e.specifiers = u1, e.source = this.parseExprAtom()) : (e.specifiers = this.parseImportSpecifiers(), this.expectContextual("from"), e.source = this.type === d.string ? this.parseExprAtom() : this.unexpected()), this.options.ecmaVersion >= 16 && (e.attributes = this.parseWithClause()), this.semicolon(), this.finishNode(e, "ImportDeclaration");
};
ue.parseImportSpecifier = function() {
  var e = this.startNode();
  return e.imported = this.parseModuleExportName(), this.eatContextual("as") ? e.local = this.parseIdent() : (this.checkUnreserved(e.imported), e.local = e.imported), this.checkLValSimple(e.local, Lt), this.finishNode(e, "ImportSpecifier");
};
ue.parseImportDefaultSpecifier = function() {
  var e = this.startNode();
  return e.local = this.parseIdent(), this.checkLValSimple(e.local, Lt), this.finishNode(e, "ImportDefaultSpecifier");
};
ue.parseImportNamespaceSpecifier = function() {
  var e = this.startNode();
  return this.next(), this.expectContextual("as"), e.local = this.parseIdent(), this.checkLValSimple(e.local, Lt), this.finishNode(e, "ImportNamespaceSpecifier");
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
  for (var t = {}, i = true; !this.eat(d.braceR); ) {
    if (i) i = false;
    else if (this.expect(d.comma), this.afterTrailingComma(d.braceR)) break;
    var s = this.parseImportAttribute(), l = s.key.type === "Identifier" ? s.key.name : s.key.value;
    er(t, l) && this.raiseRecoverable(s.key.start, "Duplicate attribute key '" + l + "'"), t[l] = true, e.push(s);
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
    return i1.test(e.value) && this.raise(e.start, "An export name cannot include a lone surrogate."), e;
  }
  return this.parseIdent(true);
};
ue.adaptDirectivePrologue = function(e) {
  for (var t = 0; t < e.length && this.isDirectiveCandidate(e[t]); ++t) e[t].directive = e[t].expression.raw.slice(1, -1);
};
ue.isDirectiveCandidate = function(e) {
  return this.options.ecmaVersion >= 5 && e.type === "ExpressionStatement" && e.expression.type === "Literal" && typeof e.expression.value == "string" && (this.input[e.start] === '"' || this.input[e.start] === "'");
};
var mt = He.prototype;
mt.toAssignable = function(e, t, i) {
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
      e.type = "ObjectPattern", i && this.checkPatternErrors(i, true);
      for (var s = 0, l = e.properties; s < l.length; s += 1) {
        var p = l[s];
        this.toAssignable(p, t), p.type === "RestElement" && (p.argument.type === "ArrayPattern" || p.argument.type === "ObjectPattern") && this.raise(p.argument.start, "Unexpected token");
      }
      break;
    case "Property":
      e.kind !== "init" && this.raise(e.key.start, "Object pattern can't contain getter or setter"), this.toAssignable(e.value, t);
      break;
    case "ArrayExpression":
      e.type = "ArrayPattern", i && this.checkPatternErrors(i, true), this.toAssignableList(e.elements, t);
      break;
    case "SpreadElement":
      e.type = "RestElement", this.toAssignable(e.argument, t), e.argument.type === "AssignmentPattern" && this.raise(e.argument.start, "Rest elements cannot have a default value");
      break;
    case "AssignmentExpression":
      e.operator !== "=" && this.raise(e.left.end, "Only '=' operator can be used for specifying default value."), e.type = "AssignmentPattern", delete e.operator, this.toAssignable(e.left, t);
      break;
    case "ParenthesizedExpression":
      this.toAssignable(e.expression, t, i);
      break;
    case "ChainExpression":
      this.raiseRecoverable(e.start, "Optional chaining cannot appear in left-hand side");
      break;
    case "MemberExpression":
      if (!t) break;
    default:
      this.raise(e.start, "Assigning to rvalue");
  }
  else i && this.checkPatternErrors(i, true);
  return e;
};
mt.toAssignableList = function(e, t) {
  for (var i = e.length, s = 0; s < i; s++) {
    var l = e[s];
    l && this.toAssignable(l, t);
  }
  if (i) {
    var p = e[i - 1];
    this.options.ecmaVersion === 6 && t && p && p.type === "RestElement" && p.argument.type !== "Identifier" && this.unexpected(p.argument.start);
  }
  return e;
};
mt.parseSpread = function(e) {
  var t = this.startNode();
  return this.next(), t.argument = this.parseMaybeAssign(false, e), this.finishNode(t, "SpreadElement");
};
mt.parseRestBinding = function() {
  var e = this.startNode();
  return this.next(), this.options.ecmaVersion === 6 && this.type !== d.name && this.unexpected(), e.argument = this.parseBindingAtom(), this.finishNode(e, "RestElement");
};
mt.parseBindingAtom = function() {
  if (this.options.ecmaVersion >= 6) switch (this.type) {
    case d.bracketL:
      var e = this.startNode();
      return this.next(), e.elements = this.parseBindingList(d.bracketR, true, true), this.finishNode(e, "ArrayPattern");
    case d.braceL:
      return this.parseObj(true);
  }
  return this.parseIdent();
};
mt.parseBindingList = function(e, t, i, s) {
  for (var l = [], p = true; !this.eat(e); ) if (p ? p = false : this.expect(d.comma), t && this.type === d.comma) l.push(null);
  else {
    if (i && this.afterTrailingComma(e)) break;
    if (this.type === d.ellipsis) {
      var m = this.parseRestBinding();
      this.parseBindingListItem(m), l.push(m), this.type === d.comma && this.raiseRecoverable(this.start, "Comma is not permitted after the rest element"), this.expect(e);
      break;
    } else l.push(this.parseAssignableListItem(s));
  }
  return l;
};
mt.parseAssignableListItem = function(e) {
  var t = this.parseMaybeDefault(this.start, this.startLoc);
  return this.parseBindingListItem(t), t;
};
mt.parseBindingListItem = function(e) {
  return e;
};
mt.parseMaybeDefault = function(e, t, i) {
  if (i = i || this.parseBindingAtom(), this.options.ecmaVersion < 6 || !this.eat(d.eq)) return i;
  var s = this.startNodeAt(e, t);
  return s.left = i, s.right = this.parseMaybeAssign(), this.finishNode(s, "AssignmentPattern");
};
mt.checkLValSimple = function(e, t, i) {
  t === void 0 && (t = zo);
  var s = t !== zo;
  switch (e.type) {
    case "Identifier":
      this.strict && this.reservedWordsStrictBind.test(e.name) && this.raiseRecoverable(e.start, (s ? "Binding " : "Assigning to ") + e.name + " in strict mode"), s && (t === Lt && e.name === "let" && this.raiseRecoverable(e.start, "let is disallowed as a lexically bound name"), i && (er(i, e.name) && this.raiseRecoverable(e.start, "Argument name clash"), i[e.name] = true), t !== Vh && this.declareName(e.name, t, e.start));
      break;
    case "ChainExpression":
      this.raiseRecoverable(e.start, "Optional chaining cannot appear in left-hand side");
      break;
    case "MemberExpression":
      s && this.raiseRecoverable(e.start, "Binding member expression");
      break;
    case "ParenthesizedExpression":
      return s && this.raiseRecoverable(e.start, "Binding parenthesized expression"), this.checkLValSimple(e.expression, t, i);
    default:
      this.raise(e.start, (s ? "Binding" : "Assigning to") + " rvalue");
  }
};
mt.checkLValPattern = function(e, t, i) {
  switch (t === void 0 && (t = zo), e.type) {
    case "ObjectPattern":
      for (var s = 0, l = e.properties; s < l.length; s += 1) {
        var p = l[s];
        this.checkLValInnerPattern(p, t, i);
      }
      break;
    case "ArrayPattern":
      for (var m = 0, w = e.elements; m < w.length; m += 1) {
        var _ = w[m];
        _ && this.checkLValInnerPattern(_, t, i);
      }
      break;
    default:
      this.checkLValSimple(e, t, i);
  }
};
mt.checkLValInnerPattern = function(e, t, i) {
  switch (t === void 0 && (t = zo), e.type) {
    case "Property":
      this.checkLValInnerPattern(e.value, t, i);
      break;
    case "AssignmentPattern":
      this.checkLValPattern(e.left, t, i);
      break;
    case "RestElement":
      this.checkLValPattern(e.argument, t, i);
      break;
    default:
      this.checkLValPattern(e, t, i);
  }
};
var rt = function(t, i, s, l, p) {
  this.token = t, this.isExpr = !!i, this.preserveSpace = !!s, this.override = l, this.generator = !!p;
}, Fe = { b_stat: new rt("{", false), b_expr: new rt("{", true), b_tmpl: new rt("${", false), p_stat: new rt("(", false), p_expr: new rt("(", true), q_tmpl: new rt("`", true, true, function(e) {
  return e.tryReadTemplateToken();
}), f_stat: new rt("function", false), f_expr: new rt("function", true), f_expr_gen: new rt("function", true, false, null, true), f_gen: new rt("function", false, false, null, true) }, rr = He.prototype;
rr.initialContext = function() {
  return [Fe.b_stat];
};
rr.curContext = function() {
  return this.context[this.context.length - 1];
};
rr.braceIsBlock = function(e) {
  var t = this.curContext();
  return t === Fe.f_expr || t === Fe.f_stat ? true : e === d.colon && (t === Fe.b_stat || t === Fe.b_expr) ? !t.isExpr : e === d._return || e === d.name && this.exprAllowed ? ut.test(this.input.slice(this.lastTokEnd, this.start)) : e === d._else || e === d.semi || e === d.eof || e === d.parenR || e === d.arrow ? true : e === d.braceL ? t === Fe.b_stat : e === d._var || e === d._const || e === d.name ? false : !this.exprAllowed;
};
rr.inGeneratorContext = function() {
  for (var e = this.context.length - 1; e >= 1; e--) {
    var t = this.context[e];
    if (t.token === "function") return t.generator;
  }
  return false;
};
rr.updateContext = function(e) {
  var t, i = this.type;
  i.keyword && e === d.dot ? this.exprAllowed = false : (t = i.updateContext) ? t.call(this, e) : this.exprAllowed = i.beforeExpr;
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
  e === Fe.b_stat && this.curContext().token === "function" && (e = this.context.pop()), this.exprAllowed = !e.isExpr;
};
d.braceL.updateContext = function(e) {
  this.context.push(this.braceIsBlock(e) ? Fe.b_stat : Fe.b_expr), this.exprAllowed = true;
};
d.dollarBraceL.updateContext = function() {
  this.context.push(Fe.b_tmpl), this.exprAllowed = true;
};
d.parenL.updateContext = function(e) {
  var t = e === d._if || e === d._for || e === d._with || e === d._while;
  this.context.push(t ? Fe.p_stat : Fe.p_expr), this.exprAllowed = true;
};
d.incDec.updateContext = function() {
};
d._function.updateContext = d._class.updateContext = function(e) {
  e.beforeExpr && e !== d._else && !(e === d.semi && this.curContext() !== Fe.p_stat) && !(e === d._return && ut.test(this.input.slice(this.lastTokEnd, this.start))) && !((e === d.colon || e === d.braceL) && this.curContext() === Fe.b_stat) ? this.context.push(Fe.f_expr) : this.context.push(Fe.f_stat), this.exprAllowed = false;
};
d.colon.updateContext = function() {
  this.curContext().token === "function" && this.context.pop(), this.exprAllowed = true;
};
d.backQuote.updateContext = function() {
  this.curContext() === Fe.q_tmpl ? this.context.pop() : this.context.push(Fe.q_tmpl), this.exprAllowed = false;
};
d.star.updateContext = function(e) {
  if (e === d._function) {
    var t = this.context.length - 1;
    this.context[t] === Fe.f_expr ? this.context[t] = Fe.f_expr_gen : this.context[t] = Fe.f_gen;
  }
  this.exprAllowed = true;
};
d.name.updateContext = function(e) {
  var t = false;
  this.options.ecmaVersion >= 6 && e !== d.dot && (this.value === "of" && !this.exprAllowed || this.value === "yield" && this.inGeneratorContext()) && (t = true), this.exprAllowed = t;
};
var ye = He.prototype;
ye.checkPropClash = function(e, t, i) {
  if (!(this.options.ecmaVersion >= 9 && e.type === "SpreadElement") && !(this.options.ecmaVersion >= 6 && (e.computed || e.method || e.shorthand))) {
    var s = e.key, l;
    switch (s.type) {
      case "Identifier":
        l = s.name;
        break;
      case "Literal":
        l = String(s.value);
        break;
      default:
        return;
    }
    var p = e.kind;
    if (this.options.ecmaVersion >= 6) {
      l === "__proto__" && p === "init" && (t.proto && (i ? i.doubleProto < 0 && (i.doubleProto = s.start) : this.raiseRecoverable(s.start, "Redefinition of __proto__ property")), t.proto = true);
      return;
    }
    l = "$" + l;
    var m = t[l];
    if (m) {
      var w;
      p === "init" ? w = this.strict && m.init || m.get || m.set : w = m.init || m[p], w && this.raiseRecoverable(s.start, "Redefinition of property");
    } else m = t[l] = { init: false, get: false, set: false };
    m[p] = true;
  }
};
ye.parseExpression = function(e, t) {
  var i = this.start, s = this.startLoc, l = this.parseMaybeAssign(e, t);
  if (this.type === d.comma) {
    var p = this.startNodeAt(i, s);
    for (p.expressions = [l]; this.eat(d.comma); ) p.expressions.push(this.parseMaybeAssign(e, t));
    return this.finishNode(p, "SequenceExpression");
  }
  return l;
};
ye.parseMaybeAssign = function(e, t, i) {
  if (this.isContextual("yield")) {
    if (this.inGenerator) return this.parseYield(e);
    this.exprAllowed = false;
  }
  var s = false, l = -1, p = -1, m = -1;
  t ? (l = t.parenthesizedAssign, p = t.trailingComma, m = t.doubleProto, t.parenthesizedAssign = t.trailingComma = -1) : (t = new Zo(), s = true);
  var w = this.start, _ = this.startLoc;
  (this.type === d.parenL || this.type === d.name) && (this.potentialArrowAt = this.start, this.potentialArrowInForAwait = e === "await");
  var o = this.parseMaybeConditional(e, t);
  if (i && (o = i.call(this, o, w, _)), this.type.isAssign) {
    var b = this.startNodeAt(w, _);
    return b.operator = this.value, this.type === d.eq && (o = this.toAssignable(o, false, t)), s || (t.parenthesizedAssign = t.trailingComma = t.doubleProto = -1), t.shorthandAssign >= o.start && (t.shorthandAssign = -1), this.type === d.eq ? this.checkLValPattern(o) : this.checkLValSimple(o), b.left = o, this.next(), b.right = this.parseMaybeAssign(e), m > -1 && (t.doubleProto = m), this.finishNode(b, "AssignmentExpression");
  } else s && this.checkExpressionErrors(t, true);
  return l > -1 && (t.parenthesizedAssign = l), p > -1 && (t.trailingComma = p), o;
};
ye.parseMaybeConditional = function(e, t) {
  var i = this.start, s = this.startLoc, l = this.parseExprOps(e, t);
  if (this.checkExpressionErrors(t)) return l;
  if (this.eat(d.question)) {
    var p = this.startNodeAt(i, s);
    return p.test = l, p.consequent = this.parseMaybeAssign(), this.expect(d.colon), p.alternate = this.parseMaybeAssign(e), this.finishNode(p, "ConditionalExpression");
  }
  return l;
};
ye.parseExprOps = function(e, t) {
  var i = this.start, s = this.startLoc, l = this.parseMaybeUnary(t, false, false, e);
  return this.checkExpressionErrors(t) || l.start === i && l.type === "ArrowFunctionExpression" ? l : this.parseExprOp(l, i, s, -1, e);
};
ye.parseExprOp = function(e, t, i, s, l) {
  var p = this.type.binop;
  if (p != null && (!l || this.type !== d._in) && p > s) {
    var m = this.type === d.logicalOR || this.type === d.logicalAND, w = this.type === d.coalesce;
    w && (p = d.logicalAND.binop);
    var _ = this.value;
    this.next();
    var o = this.start, b = this.startLoc, L = this.parseExprOp(this.parseMaybeUnary(null, false, false, l), o, b, p, l), O = this.buildBinary(t, i, e, L, _, m || w);
    return (m && this.type === d.coalesce || w && (this.type === d.logicalOR || this.type === d.logicalAND)) && this.raiseRecoverable(this.start, "Logical expressions and coalesce expressions cannot be mixed. Wrap either by parentheses"), this.parseExprOp(O, t, i, s, l);
  }
  return e;
};
ye.buildBinary = function(e, t, i, s, l, p) {
  s.type === "PrivateIdentifier" && this.raise(s.start, "Private identifier can only be left side of binary expression");
  var m = this.startNodeAt(e, t);
  return m.left = i, m.operator = l, m.right = s, this.finishNode(m, p ? "LogicalExpression" : "BinaryExpression");
};
ye.parseMaybeUnary = function(e, t, i, s) {
  var l = this.start, p = this.startLoc, m;
  if (this.isContextual("await") && this.canAwait) m = this.parseAwait(s), t = true;
  else if (this.type.prefix) {
    var w = this.startNode(), _ = this.type === d.incDec;
    w.operator = this.value, w.prefix = true, this.next(), w.argument = this.parseMaybeUnary(null, true, _, s), this.checkExpressionErrors(e, true), _ ? this.checkLValSimple(w.argument) : this.strict && w.operator === "delete" && Hh(w.argument) ? this.raiseRecoverable(w.start, "Deleting local variable in strict mode") : w.operator === "delete" && bl(w.argument) ? this.raiseRecoverable(w.start, "Private fields can not be deleted") : t = true, m = this.finishNode(w, _ ? "UpdateExpression" : "UnaryExpression");
  } else if (!t && this.type === d.privateId) (s || this.privateNameStack.length === 0) && this.options.checkPrivateFields && this.unexpected(), m = this.parsePrivateIdent(), this.type !== d._in && this.unexpected();
  else {
    if (m = this.parseExprSubscripts(e, s), this.checkExpressionErrors(e)) return m;
    for (; this.type.postfix && !this.canInsertSemicolon(); ) {
      var o = this.startNodeAt(l, p);
      o.operator = this.value, o.prefix = false, o.argument = m, this.checkLValSimple(m), this.next(), m = this.finishNode(o, "UpdateExpression");
    }
  }
  if (!i && this.eat(d.starstar)) if (t) this.unexpected(this.lastTokStart);
  else return this.buildBinary(l, p, m, this.parseMaybeUnary(null, false, false, s), "**", false);
  else return m;
};
function Hh(e) {
  return e.type === "Identifier" || e.type === "ParenthesizedExpression" && Hh(e.expression);
}
function bl(e) {
  return e.type === "MemberExpression" && e.property.type === "PrivateIdentifier" || e.type === "ChainExpression" && bl(e.expression) || e.type === "ParenthesizedExpression" && bl(e.expression);
}
ye.parseExprSubscripts = function(e, t) {
  var i = this.start, s = this.startLoc, l = this.parseExprAtom(e, t);
  if (l.type === "ArrowFunctionExpression" && this.input.slice(this.lastTokStart, this.lastTokEnd) !== ")") return l;
  var p = this.parseSubscripts(l, i, s, false, t);
  return e && p.type === "MemberExpression" && (e.parenthesizedAssign >= p.start && (e.parenthesizedAssign = -1), e.parenthesizedBind >= p.start && (e.parenthesizedBind = -1), e.trailingComma >= p.start && (e.trailingComma = -1)), p;
};
ye.parseSubscripts = function(e, t, i, s, l) {
  for (var p = this.options.ecmaVersion >= 8 && e.type === "Identifier" && e.name === "async" && this.lastTokEnd === e.end && !this.canInsertSemicolon() && e.end - e.start === 5 && this.potentialArrowAt === e.start, m = false; ; ) {
    var w = this.parseSubscript(e, t, i, s, p, m, l);
    if (w.optional && (m = true), w === e || w.type === "ArrowFunctionExpression") {
      if (m) {
        var _ = this.startNodeAt(t, i);
        _.expression = w, w = this.finishNode(_, "ChainExpression");
      }
      return w;
    }
    e = w;
  }
};
ye.shouldParseAsyncArrow = function() {
  return !this.canInsertSemicolon() && this.eat(d.arrow);
};
ye.parseSubscriptAsyncArrow = function(e, t, i, s) {
  return this.parseArrowExpression(this.startNodeAt(e, t), i, true, s);
};
ye.parseSubscript = function(e, t, i, s, l, p, m) {
  var w = this.options.ecmaVersion >= 11, _ = w && this.eat(d.questionDot);
  s && _ && this.raise(this.lastTokStart, "Optional chaining cannot appear in the callee of new expressions");
  var o = this.eat(d.bracketL);
  if (o || _ && this.type !== d.parenL && this.type !== d.backQuote || this.eat(d.dot)) {
    var b = this.startNodeAt(t, i);
    b.object = e, o ? (b.property = this.parseExpression(), this.expect(d.bracketR)) : this.type === d.privateId && e.type !== "Super" ? b.property = this.parsePrivateIdent() : b.property = this.parseIdent(this.options.allowReserved !== "never"), b.computed = !!o, w && (b.optional = _), e = this.finishNode(b, "MemberExpression");
  } else if (!s && this.eat(d.parenL)) {
    var L = new Zo(), O = this.yieldPos, B = this.awaitPos, G = this.awaitIdentPos;
    this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0;
    var F = this.parseExprList(d.parenR, this.options.ecmaVersion >= 8, false, L);
    if (l && !_ && this.shouldParseAsyncArrow()) return this.checkPatternErrors(L, false), this.checkYieldAwaitInDefaultParams(), this.awaitIdentPos > 0 && this.raise(this.awaitIdentPos, "Cannot use 'await' as identifier inside an async function"), this.yieldPos = O, this.awaitPos = B, this.awaitIdentPos = G, this.parseSubscriptAsyncArrow(t, i, F, m);
    this.checkExpressionErrors(L, true), this.yieldPos = O || this.yieldPos, this.awaitPos = B || this.awaitPos, this.awaitIdentPos = G || this.awaitIdentPos;
    var R = this.startNodeAt(t, i);
    R.callee = e, R.arguments = F, w && (R.optional = _), e = this.finishNode(R, "CallExpression");
  } else if (this.type === d.backQuote) {
    (_ || p) && this.raise(this.start, "Optional chaining cannot appear in the tag of tagged template expressions");
    var v = this.startNodeAt(t, i);
    v.tag = e, v.quasi = this.parseTemplate({ isTagged: true }), e = this.finishNode(v, "TaggedTemplateExpression");
  }
  return e;
};
ye.parseExprAtom = function(e, t, i) {
  this.type === d.slash && this.readRegexp();
  var s, l = this.potentialArrowAt === this.start;
  switch (this.type) {
    case d._super:
      return this.allowSuper || this.raise(this.start, "'super' keyword outside a method"), s = this.startNode(), this.next(), this.type === d.parenL && !this.allowDirectSuper && this.raise(s.start, "super() call outside constructor of a subclass"), this.type !== d.dot && this.type !== d.bracketL && this.type !== d.parenL && this.unexpected(), this.finishNode(s, "Super");
    case d._this:
      return s = this.startNode(), this.next(), this.finishNode(s, "ThisExpression");
    case d.name:
      var p = this.start, m = this.startLoc, w = this.containsEsc, _ = this.parseIdent(false);
      if (this.options.ecmaVersion >= 8 && !w && _.name === "async" && !this.canInsertSemicolon() && this.eat(d._function)) return this.overrideContext(Fe.f_expr), this.parseFunction(this.startNodeAt(p, m), 0, false, true, t);
      if (l && !this.canInsertSemicolon()) {
        if (this.eat(d.arrow)) return this.parseArrowExpression(this.startNodeAt(p, m), [_], false, t);
        if (this.options.ecmaVersion >= 8 && _.name === "async" && this.type === d.name && !w && (!this.potentialArrowInForAwait || this.value !== "of" || this.containsEsc)) return _ = this.parseIdent(false), (this.canInsertSemicolon() || !this.eat(d.arrow)) && this.unexpected(), this.parseArrowExpression(this.startNodeAt(p, m), [_], true, t);
      }
      return _;
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
      var b = this.start, L = this.parseParenAndDistinguishExpression(l, t);
      return e && (e.parenthesizedAssign < 0 && !this.isSimpleAssignTarget(L) && (e.parenthesizedAssign = b), e.parenthesizedBind < 0 && (e.parenthesizedBind = b)), L;
    case d.bracketL:
      return s = this.startNode(), this.next(), s.elements = this.parseExprList(d.bracketR, true, true, e), this.finishNode(s, "ArrayExpression");
    case d.braceL:
      return this.overrideContext(Fe.b_expr), this.parseObj(false, e);
    case d._function:
      return s = this.startNode(), this.next(), this.parseFunction(s, 0);
    case d._class:
      return this.parseClass(this.startNode(), false);
    case d._new:
      return this.parseNew();
    case d.backQuote:
      return this.parseTemplate();
    case d._import:
      return this.options.ecmaVersion >= 11 ? this.parseExprImport(i) : this.unexpected();
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
    var i = this.startNodeAt(t.start, t.loc && t.loc.start);
    return i.name = "import", t.meta = this.finishNode(i, "Identifier"), this.parseImportMeta(t);
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
  var i = this.start, s = this.startLoc, l, p = this.options.ecmaVersion >= 8;
  if (this.options.ecmaVersion >= 6) {
    this.next();
    var m = this.start, w = this.startLoc, _ = [], o = true, b = false, L = new Zo(), O = this.yieldPos, B = this.awaitPos, G;
    for (this.yieldPos = 0, this.awaitPos = 0; this.type !== d.parenR; ) if (o ? o = false : this.expect(d.comma), p && this.afterTrailingComma(d.parenR, true)) {
      b = true;
      break;
    } else if (this.type === d.ellipsis) {
      G = this.start, _.push(this.parseParenItem(this.parseRestBinding())), this.type === d.comma && this.raiseRecoverable(this.start, "Comma is not permitted after the rest element");
      break;
    } else _.push(this.parseMaybeAssign(false, L, this.parseParenItem));
    var F = this.lastTokEnd, R = this.lastTokEndLoc;
    if (this.expect(d.parenR), e && this.shouldParseArrow(_) && this.eat(d.arrow)) return this.checkPatternErrors(L, false), this.checkYieldAwaitInDefaultParams(), this.yieldPos = O, this.awaitPos = B, this.parseParenArrowList(i, s, _, t);
    (!_.length || b) && this.unexpected(this.lastTokStart), G && this.unexpected(G), this.checkExpressionErrors(L, true), this.yieldPos = O || this.yieldPos, this.awaitPos = B || this.awaitPos, _.length > 1 ? (l = this.startNodeAt(m, w), l.expressions = _, this.finishNodeAt(l, "SequenceExpression", F, R)) : l = _[0];
  } else l = this.parseParenExpression();
  if (this.options.preserveParens) {
    var v = this.startNodeAt(i, s);
    return v.expression = l, this.finishNode(v, "ParenthesizedExpression");
  } else return l;
};
ye.parseParenItem = function(e) {
  return e;
};
ye.parseParenArrowList = function(e, t, i, s) {
  return this.parseArrowExpression(this.startNodeAt(e, t), i, false, s);
};
var d1 = [];
ye.parseNew = function() {
  this.containsEsc && this.raiseRecoverable(this.start, "Escape sequence in keyword new");
  var e = this.startNode();
  if (this.next(), this.options.ecmaVersion >= 6 && this.type === d.dot) {
    var t = this.startNodeAt(e.start, e.loc && e.loc.start);
    t.name = "new", e.meta = this.finishNode(t, "Identifier"), this.next();
    var i = this.containsEsc;
    return e.property = this.parseIdent(true), e.property.name !== "target" && this.raiseRecoverable(e.property.start, "The only valid meta property for new is 'new.target'"), i && this.raiseRecoverable(e.start, "'new.target' must not contain escaped characters"), this.allowNewDotTarget || this.raiseRecoverable(e.start, "'new.target' can only be used in functions and class static block"), this.finishNode(e, "MetaProperty");
  }
  var s = this.start, l = this.startLoc;
  return e.callee = this.parseSubscripts(this.parseExprAtom(null, false, true), s, l, true, false), this.eat(d.parenL) ? e.arguments = this.parseExprList(d.parenR, this.options.ecmaVersion >= 8, false) : e.arguments = d1, this.finishNode(e, "NewExpression");
};
ye.parseTemplateElement = function(e) {
  var t = e.isTagged, i = this.startNode();
  return this.type === d.invalidTemplate ? (t || this.raiseRecoverable(this.start, "Bad escape sequence in untagged template literal"), i.value = { raw: this.value.replace(/\r\n?/g, `
`), cooked: null }) : i.value = { raw: this.input.slice(this.start, this.end).replace(/\r\n?/g, `
`), cooked: this.value }, this.next(), i.tail = this.type === d.backQuote, this.finishNode(i, "TemplateElement");
};
ye.parseTemplate = function(e) {
  e === void 0 && (e = {});
  var t = e.isTagged;
  t === void 0 && (t = false);
  var i = this.startNode();
  this.next(), i.expressions = [];
  var s = this.parseTemplateElement({ isTagged: t });
  for (i.quasis = [s]; !s.tail; ) this.type === d.eof && this.raise(this.pos, "Unterminated template literal"), this.expect(d.dollarBraceL), i.expressions.push(this.parseExpression()), this.expect(d.braceR), i.quasis.push(s = this.parseTemplateElement({ isTagged: t }));
  return this.next(), this.finishNode(i, "TemplateLiteral");
};
ye.isAsyncProp = function(e) {
  return !e.computed && e.key.type === "Identifier" && e.key.name === "async" && (this.type === d.name || this.type === d.num || this.type === d.string || this.type === d.bracketL || this.type.keyword || this.options.ecmaVersion >= 9 && this.type === d.star) && !ut.test(this.input.slice(this.lastTokEnd, this.start));
};
ye.parseObj = function(e, t) {
  var i = this.startNode(), s = true, l = {};
  for (i.properties = [], this.next(); !this.eat(d.braceR); ) {
    if (s) s = false;
    else if (this.expect(d.comma), this.options.ecmaVersion >= 5 && this.afterTrailingComma(d.braceR)) break;
    var p = this.parseProperty(e, t);
    e || this.checkPropClash(p, l, t), i.properties.push(p);
  }
  return this.finishNode(i, e ? "ObjectPattern" : "ObjectExpression");
};
ye.parseProperty = function(e, t) {
  var i = this.startNode(), s, l, p, m;
  if (this.options.ecmaVersion >= 9 && this.eat(d.ellipsis)) return e ? (i.argument = this.parseIdent(false), this.type === d.comma && this.raiseRecoverable(this.start, "Comma is not permitted after the rest element"), this.finishNode(i, "RestElement")) : (i.argument = this.parseMaybeAssign(false, t), this.type === d.comma && t && t.trailingComma < 0 && (t.trailingComma = this.start), this.finishNode(i, "SpreadElement"));
  this.options.ecmaVersion >= 6 && (i.method = false, i.shorthand = false, (e || t) && (p = this.start, m = this.startLoc), e || (s = this.eat(d.star)));
  var w = this.containsEsc;
  return this.parsePropertyName(i), !e && !w && this.options.ecmaVersion >= 8 && !s && this.isAsyncProp(i) ? (l = true, s = this.options.ecmaVersion >= 9 && this.eat(d.star), this.parsePropertyName(i)) : l = false, this.parsePropertyValue(i, e, s, l, p, m, t, w), this.finishNode(i, "Property");
};
ye.parseGetterSetter = function(e) {
  e.kind = e.key.name, this.parsePropertyName(e), e.value = this.parseMethod(false);
  var t = e.kind === "get" ? 0 : 1;
  if (e.value.params.length !== t) {
    var i = e.value.start;
    e.kind === "get" ? this.raiseRecoverable(i, "getter should have no params") : this.raiseRecoverable(i, "setter should have exactly one param");
  } else e.kind === "set" && e.value.params[0].type === "RestElement" && this.raiseRecoverable(e.value.params[0].start, "Setter cannot use rest params");
};
ye.parsePropertyValue = function(e, t, i, s, l, p, m, w) {
  (i || s) && this.type === d.colon && this.unexpected(), this.eat(d.colon) ? (e.value = t ? this.parseMaybeDefault(this.start, this.startLoc) : this.parseMaybeAssign(false, m), e.kind = "init") : this.options.ecmaVersion >= 6 && this.type === d.parenL ? (t && this.unexpected(), e.kind = "init", e.method = true, e.value = this.parseMethod(i, s)) : !t && !w && this.options.ecmaVersion >= 5 && !e.computed && e.key.type === "Identifier" && (e.key.name === "get" || e.key.name === "set") && this.type !== d.comma && this.type !== d.braceR && this.type !== d.eq ? ((i || s) && this.unexpected(), this.parseGetterSetter(e)) : this.options.ecmaVersion >= 6 && !e.computed && e.key.type === "Identifier" ? ((i || s) && this.unexpected(), this.checkUnreserved(e.key), e.key.name === "await" && !this.awaitIdentPos && (this.awaitIdentPos = l), e.kind = "init", t ? e.value = this.parseMaybeDefault(l, p, this.copyNode(e.key)) : this.type === d.eq && m ? (m.shorthandAssign < 0 && (m.shorthandAssign = this.start), e.value = this.parseMaybeDefault(l, p, this.copyNode(e.key))) : e.value = this.copyNode(e.key), e.shorthand = true) : this.unexpected();
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
ye.parseMethod = function(e, t, i) {
  var s = this.startNode(), l = this.yieldPos, p = this.awaitPos, m = this.awaitIdentPos;
  return this.initFunction(s), this.options.ecmaVersion >= 6 && (s.generator = e), this.options.ecmaVersion >= 8 && (s.async = !!t), this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0, this.enterScope(wl(t, s.generator) | Tl | (i ? jh : 0)), this.expect(d.parenL), s.params = this.parseBindingList(d.parenR, false, this.options.ecmaVersion >= 8), this.checkYieldAwaitInDefaultParams(), this.parseFunctionBody(s, false, true, false), this.yieldPos = l, this.awaitPos = p, this.awaitIdentPos = m, this.finishNode(s, "FunctionExpression");
};
ye.parseArrowExpression = function(e, t, i, s) {
  var l = this.yieldPos, p = this.awaitPos, m = this.awaitIdentPos;
  return this.enterScope(wl(i, false) | Nh), this.initFunction(e), this.options.ecmaVersion >= 8 && (e.async = !!i), this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0, e.params = this.toAssignableList(t, true), this.parseFunctionBody(e, true, false, s), this.yieldPos = l, this.awaitPos = p, this.awaitIdentPos = m, this.finishNode(e, "ArrowFunctionExpression");
};
ye.parseFunctionBody = function(e, t, i, s) {
  var l = t && this.type !== d.braceL, p = this.strict, m = false;
  if (l) e.body = this.parseMaybeAssign(s), e.expression = true, this.checkParams(e, false);
  else {
    var w = this.options.ecmaVersion >= 7 && !this.isSimpleParamList(e.params);
    (!p || w) && (m = this.strictDirective(this.end), m && w && this.raiseRecoverable(e.start, "Illegal 'use strict' directive in function with non-simple parameter list"));
    var _ = this.labels;
    this.labels = [], m && (this.strict = true), this.checkParams(e, !p && !m && !t && !i && this.isSimpleParamList(e.params)), this.strict && e.id && this.checkLValSimple(e.id, Vh), e.body = this.parseBlock(false, void 0, m && !p), e.expression = false, this.adaptDirectivePrologue(e.body.body), this.labels = _;
  }
  this.exitScope();
};
ye.isSimpleParamList = function(e) {
  for (var t = 0, i = e; t < i.length; t += 1) {
    var s = i[t];
    if (s.type !== "Identifier") return false;
  }
  return true;
};
ye.checkParams = function(e, t) {
  for (var i = /* @__PURE__ */ Object.create(null), s = 0, l = e.params; s < l.length; s += 1) {
    var p = l[s];
    this.checkLValInnerPattern(p, Al, t ? null : i);
  }
};
ye.parseExprList = function(e, t, i, s) {
  for (var l = [], p = true; !this.eat(e); ) {
    if (p) p = false;
    else if (this.expect(d.comma), t && this.afterTrailingComma(e)) break;
    var m = void 0;
    i && this.type === d.comma ? m = null : this.type === d.ellipsis ? (m = this.parseSpread(s), s && this.type === d.comma && s.trailingComma < 0 && (s.trailingComma = this.start)) : m = this.parseMaybeAssign(false, s), l.push(m);
  }
  return l;
};
ye.checkUnreserved = function(e) {
  var t = e.start, i = e.end, s = e.name;
  if (this.inGenerator && s === "yield" && this.raiseRecoverable(t, "Cannot use 'yield' as identifier inside a generator"), this.inAsync && s === "await" && this.raiseRecoverable(t, "Cannot use 'await' as identifier inside an async function"), this.currentThisScope().inClassFieldInit && s === "arguments" && this.raiseRecoverable(t, "Cannot use 'arguments' in class field initializer"), this.inClassStaticBlock && (s === "arguments" || s === "await") && this.raise(t, "Cannot use " + s + " in class static initialization block"), this.keywords.test(s) && this.raise(t, "Unexpected keyword '" + s + "'"), !(this.options.ecmaVersion < 6 && this.input.slice(t, i).indexOf("\\") !== -1)) {
    var l = this.strict ? this.reservedWordsStrict : this.reservedWords;
    l.test(s) && (!this.inAsync && s === "await" && this.raiseRecoverable(t, "Cannot use keyword 'await' outside an async function"), this.raiseRecoverable(t, "The keyword '" + s + "' is reserved"));
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
var Ko = He.prototype;
Ko.raise = function(e, t) {
  var i = Pl(this.input, e);
  t += " (" + i.line + ":" + i.column + ")";
  var s = new SyntaxError(t);
  throw s.pos = e, s.loc = i, s.raisedAt = this.pos, s;
};
Ko.raiseRecoverable = Ko.raise;
Ko.curPosition = function() {
  if (this.options.locations) return new Jt(this.curLine, this.pos - this.lineStart);
};
var jt = He.prototype, p1 = function(t) {
  this.flags = t, this.var = [], this.lexical = [], this.functions = [], this.inClassFieldInit = false;
};
jt.enterScope = function(e) {
  this.scopeStack.push(new p1(e));
};
jt.exitScope = function() {
  this.scopeStack.pop();
};
jt.treatFunctionsAsVarInScope = function(e) {
  return e.flags & tr || !this.inModule && e.flags & Co;
};
jt.declareName = function(e, t, i) {
  var s = false;
  if (t === Lt) {
    var l = this.currentScope();
    s = l.lexical.indexOf(e) > -1 || l.functions.indexOf(e) > -1 || l.var.indexOf(e) > -1, l.lexical.push(e), this.inModule && l.flags & Co && delete this.undefinedExports[e];
  } else if (t === Fh) {
    var p = this.currentScope();
    p.lexical.push(e);
  } else if (t === Bh) {
    var m = this.currentScope();
    this.treatFunctionsAsVar ? s = m.lexical.indexOf(e) > -1 : s = m.lexical.indexOf(e) > -1 || m.var.indexOf(e) > -1, m.functions.push(e);
  } else for (var w = this.scopeStack.length - 1; w >= 0; --w) {
    var _ = this.scopeStack[w];
    if (_.lexical.indexOf(e) > -1 && !(_.flags & Dh && _.lexical[0] === e) || !this.treatFunctionsAsVarInScope(_) && _.functions.indexOf(e) > -1) {
      s = true;
      break;
    }
    if (_.var.push(e), this.inModule && _.flags & Co && delete this.undefinedExports[e], _.flags & Sl) break;
  }
  s && this.raiseRecoverable(i, "Identifier '" + e + "' has already been declared");
};
jt.checkLocalExport = function(e) {
  this.scopeStack[0].lexical.indexOf(e.name) === -1 && this.scopeStack[0].var.indexOf(e.name) === -1 && (this.undefinedExports[e.name] = e);
};
jt.currentScope = function() {
  return this.scopeStack[this.scopeStack.length - 1];
};
jt.currentVarScope = function() {
  for (var e = this.scopeStack.length - 1; ; e--) {
    var t = this.scopeStack[e];
    if (t.flags & Sl) return t;
  }
};
jt.currentThisScope = function() {
  for (var e = this.scopeStack.length - 1; ; e--) {
    var t = this.scopeStack[e];
    if (t.flags & Sl && !(t.flags & Nh)) return t;
  }
};
var To = function(t, i, s) {
  this.type = "", this.start = i, this.end = 0, t.options.locations && (this.loc = new Po(t, s)), t.options.directSourceFile && (this.sourceFile = t.options.directSourceFile), t.options.ranges && (this.range = [i, 0]);
}, So = He.prototype;
So.startNode = function() {
  return new To(this, this.start, this.startLoc);
};
So.startNodeAt = function(e, t) {
  return new To(this, e, t);
};
function Uh(e, t, i, s) {
  return e.type = t, e.end = i, this.options.locations && (e.loc.end = s), this.options.ranges && (e.range[1] = i), e;
}
So.finishNode = function(e, t) {
  return Uh.call(this, e, t, this.lastTokEnd, this.lastTokEndLoc);
};
So.finishNodeAt = function(e, t, i, s) {
  return Uh.call(this, e, t, i, s);
};
So.copyNode = function(e) {
  var t = new To(this, e.start, this.startLoc);
  for (var i in e) t[i] = e[i];
  return t;
};
var f1 = "Gara Garay Gukh Gurung_Khema Hrkt Katakana_Or_Hiragana Kawi Kirat_Rai Krai Nag_Mundari Nagm Ol_Onal Onao Sunu Sunuwar Todhri Todr Tulu_Tigalari Tutg Unknown Zzzz", Gh = "ASCII ASCII_Hex_Digit AHex Alphabetic Alpha Any Assigned Bidi_Control Bidi_C Bidi_Mirrored Bidi_M Case_Ignorable CI Cased Changes_When_Casefolded CWCF Changes_When_Casemapped CWCM Changes_When_Lowercased CWL Changes_When_NFKC_Casefolded CWKCF Changes_When_Titlecased CWT Changes_When_Uppercased CWU Dash Default_Ignorable_Code_Point DI Deprecated Dep Diacritic Dia Emoji Emoji_Component Emoji_Modifier Emoji_Modifier_Base Emoji_Presentation Extender Ext Grapheme_Base Gr_Base Grapheme_Extend Gr_Ext Hex_Digit Hex IDS_Binary_Operator IDSB IDS_Trinary_Operator IDST ID_Continue IDC ID_Start IDS Ideographic Ideo Join_Control Join_C Logical_Order_Exception LOE Lowercase Lower Math Noncharacter_Code_Point NChar Pattern_Syntax Pat_Syn Pattern_White_Space Pat_WS Quotation_Mark QMark Radical Regional_Indicator RI Sentence_Terminal STerm Soft_Dotted SD Terminal_Punctuation Term Unified_Ideograph UIdeo Uppercase Upper Variation_Selector VS White_Space space XID_Continue XIDC XID_Start XIDS", Wh = Gh + " Extended_Pictographic", zh = Wh, Xh = zh + " EBase EComp EMod EPres ExtPict", Kh = Xh, h1 = Kh, m1 = { 9: Gh, 10: Wh, 11: zh, 12: Xh, 13: Kh, 14: h1 }, v1 = "Basic_Emoji Emoji_Keycap_Sequence RGI_Emoji_Modifier_Sequence RGI_Emoji_Flag_Sequence RGI_Emoji_Tag_Sequence RGI_Emoji_ZWJ_Sequence RGI_Emoji", b1 = { 9: "", 10: "", 11: "", 12: "", 13: "", 14: v1 }, Gl = "Cased_Letter LC Close_Punctuation Pe Connector_Punctuation Pc Control Cc cntrl Currency_Symbol Sc Dash_Punctuation Pd Decimal_Number Nd digit Enclosing_Mark Me Final_Punctuation Pf Format Cf Initial_Punctuation Pi Letter L Letter_Number Nl Line_Separator Zl Lowercase_Letter Ll Mark M Combining_Mark Math_Symbol Sm Modifier_Letter Lm Modifier_Symbol Sk Nonspacing_Mark Mn Number N Open_Punctuation Ps Other C Other_Letter Lo Other_Number No Other_Punctuation Po Other_Symbol So Paragraph_Separator Zp Private_Use Co Punctuation P punct Separator Z Space_Separator Zs Spacing_Mark Mc Surrogate Cs Symbol S Titlecase_Letter Lt Unassigned Cn Uppercase_Letter Lu", Jh = "Adlam Adlm Ahom Anatolian_Hieroglyphs Hluw Arabic Arab Armenian Armn Avestan Avst Balinese Bali Bamum Bamu Bassa_Vah Bass Batak Batk Bengali Beng Bhaiksuki Bhks Bopomofo Bopo Brahmi Brah Braille Brai Buginese Bugi Buhid Buhd Canadian_Aboriginal Cans Carian Cari Caucasian_Albanian Aghb Chakma Cakm Cham Cham Cherokee Cher Common Zyyy Coptic Copt Qaac Cuneiform Xsux Cypriot Cprt Cyrillic Cyrl Deseret Dsrt Devanagari Deva Duployan Dupl Egyptian_Hieroglyphs Egyp Elbasan Elba Ethiopic Ethi Georgian Geor Glagolitic Glag Gothic Goth Grantha Gran Greek Grek Gujarati Gujr Gurmukhi Guru Han Hani Hangul Hang Hanunoo Hano Hatran Hatr Hebrew Hebr Hiragana Hira Imperial_Aramaic Armi Inherited Zinh Qaai Inscriptional_Pahlavi Phli Inscriptional_Parthian Prti Javanese Java Kaithi Kthi Kannada Knda Katakana Kana Kayah_Li Kali Kharoshthi Khar Khmer Khmr Khojki Khoj Khudawadi Sind Lao Laoo Latin Latn Lepcha Lepc Limbu Limb Linear_A Lina Linear_B Linb Lisu Lisu Lycian Lyci Lydian Lydi Mahajani Mahj Malayalam Mlym Mandaic Mand Manichaean Mani Marchen Marc Masaram_Gondi Gonm Meetei_Mayek Mtei Mende_Kikakui Mend Meroitic_Cursive Merc Meroitic_Hieroglyphs Mero Miao Plrd Modi Mongolian Mong Mro Mroo Multani Mult Myanmar Mymr Nabataean Nbat New_Tai_Lue Talu Newa Newa Nko Nkoo Nushu Nshu Ogham Ogam Ol_Chiki Olck Old_Hungarian Hung Old_Italic Ital Old_North_Arabian Narb Old_Permic Perm Old_Persian Xpeo Old_South_Arabian Sarb Old_Turkic Orkh Oriya Orya Osage Osge Osmanya Osma Pahawh_Hmong Hmng Palmyrene Palm Pau_Cin_Hau Pauc Phags_Pa Phag Phoenician Phnx Psalter_Pahlavi Phlp Rejang Rjng Runic Runr Samaritan Samr Saurashtra Saur Sharada Shrd Shavian Shaw Siddham Sidd SignWriting Sgnw Sinhala Sinh Sora_Sompeng Sora Soyombo Soyo Sundanese Sund Syloti_Nagri Sylo Syriac Syrc Tagalog Tglg Tagbanwa Tagb Tai_Le Tale Tai_Tham Lana Tai_Viet Tavt Takri Takr Tamil Taml Tangut Tang Telugu Telu Thaana Thaa Thai Thai Tibetan Tibt Tifinagh Tfng Tirhuta Tirh Ugaritic Ugar Vai Vaii Warang_Citi Wara Yi Yiii Zanabazar_Square Zanb", Qh = Jh + " Dogra Dogr Gunjala_Gondi Gong Hanifi_Rohingya Rohg Makasar Maka Medefaidrin Medf Old_Sogdian Sogo Sogdian Sogd", Yh = Qh + " Elymaic Elym Nandinagari Nand Nyiakeng_Puachue_Hmong Hmnp Wancho Wcho", Zh = Yh + " Chorasmian Chrs Diak Dives_Akuru Khitan_Small_Script Kits Yezi Yezidi", em = Zh + " Cypro_Minoan Cpmn Old_Uyghur Ougr Tangsa Tnsa Toto Vithkuqi Vith", y1 = em + " " + f1, g1 = { 9: Jh, 10: Qh, 11: Yh, 12: Zh, 13: em, 14: y1 }, tm = {};
function R1(e) {
  var t = tm[e] = { binary: Ot(m1[e] + " " + Gl), binaryOfStrings: Ot(b1[e]), nonBinary: { General_Category: Ot(Gl), Script: Ot(g1[e]) } };
  t.nonBinary.Script_Extensions = t.nonBinary.Script, t.nonBinary.gc = t.nonBinary.General_Category, t.nonBinary.sc = t.nonBinary.Script, t.nonBinary.scx = t.nonBinary.Script_Extensions;
}
for (var cl = 0, Wl = [9, 10, 11, 12, 13, 14]; cl < Wl.length; cl += 1) {
  var x1 = Wl[cl];
  R1(x1);
}
var oe = He.prototype, Jo = function(t, i) {
  this.parent = t, this.base = i || this;
};
Jo.prototype.separatedFrom = function(t) {
  for (var i = this; i; i = i.parent) for (var s = t; s; s = s.parent) if (i.base === s.base && i !== s) return true;
  return false;
};
Jo.prototype.sibling = function() {
  return new Jo(this.parent, this.base);
};
var Tt = function(t) {
  this.parser = t, this.validFlags = "gim" + (t.options.ecmaVersion >= 6 ? "uy" : "") + (t.options.ecmaVersion >= 9 ? "s" : "") + (t.options.ecmaVersion >= 13 ? "d" : "") + (t.options.ecmaVersion >= 15 ? "v" : ""), this.unicodeProperties = tm[t.options.ecmaVersion >= 14 ? 14 : t.options.ecmaVersion], this.source = "", this.flags = "", this.start = 0, this.switchU = false, this.switchV = false, this.switchN = false, this.pos = 0, this.lastIntValue = 0, this.lastStringValue = "", this.lastAssertionIsQuantifiable = false, this.numCapturingParens = 0, this.maxBackReference = 0, this.groupNames = /* @__PURE__ */ Object.create(null), this.backReferenceNames = [], this.branchID = null;
};
Tt.prototype.reset = function(t, i, s) {
  var l = s.indexOf("v") !== -1, p = s.indexOf("u") !== -1;
  this.start = t | 0, this.source = i + "", this.flags = s, l && this.parser.options.ecmaVersion >= 15 ? (this.switchU = true, this.switchV = true, this.switchN = true) : (this.switchU = p && this.parser.options.ecmaVersion >= 6, this.switchV = false, this.switchN = p && this.parser.options.ecmaVersion >= 9);
};
Tt.prototype.raise = function(t) {
  this.parser.raiseRecoverable(this.start, "Invalid regular expression: /" + this.source + "/: " + t);
};
Tt.prototype.at = function(t, i) {
  i === void 0 && (i = false);
  var s = this.source, l = s.length;
  if (t >= l) return -1;
  var p = s.charCodeAt(t);
  if (!(i || this.switchU) || p <= 55295 || p >= 57344 || t + 1 >= l) return p;
  var m = s.charCodeAt(t + 1);
  return m >= 56320 && m <= 57343 ? (p << 10) + m - 56613888 : p;
};
Tt.prototype.nextIndex = function(t, i) {
  i === void 0 && (i = false);
  var s = this.source, l = s.length;
  if (t >= l) return l;
  var p = s.charCodeAt(t), m;
  return !(i || this.switchU) || p <= 55295 || p >= 57344 || t + 1 >= l || (m = s.charCodeAt(t + 1)) < 56320 || m > 57343 ? t + 1 : t + 2;
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
Tt.prototype.eat = function(t, i) {
  return i === void 0 && (i = false), this.current(i) === t ? (this.advance(i), true) : false;
};
Tt.prototype.eatChars = function(t, i) {
  i === void 0 && (i = false);
  for (var s = this.pos, l = 0, p = t; l < p.length; l += 1) {
    var m = p[l], w = this.at(s, i);
    if (w === -1 || w !== m) return false;
    s = this.nextIndex(s, i);
  }
  return this.pos = s, true;
};
oe.validateRegExpFlags = function(e) {
  for (var t = e.validFlags, i = e.flags, s = false, l = false, p = 0; p < i.length; p++) {
    var m = i.charAt(p);
    t.indexOf(m) === -1 && this.raise(e.start, "Invalid regular expression flag"), i.indexOf(m, p + 1) > -1 && this.raise(e.start, "Duplicate regular expression flag"), m === "u" && (s = true), m === "v" && (l = true);
  }
  this.options.ecmaVersion >= 15 && s && l && this.raise(e.start, "Invalid regular expression flag");
};
function _1(e) {
  for (var t in e) return true;
  return false;
}
oe.validateRegExpPattern = function(e) {
  this.regexp_pattern(e), !e.switchN && this.options.ecmaVersion >= 9 && _1(e.groupNames) && (e.switchN = true, this.regexp_pattern(e));
};
oe.regexp_pattern = function(e) {
  e.pos = 0, e.lastIntValue = 0, e.lastStringValue = "", e.lastAssertionIsQuantifiable = false, e.numCapturingParens = 0, e.maxBackReference = 0, e.groupNames = /* @__PURE__ */ Object.create(null), e.backReferenceNames.length = 0, e.branchID = null, this.regexp_disjunction(e), e.pos !== e.source.length && (e.eat(41) && e.raise("Unmatched ')'"), (e.eat(93) || e.eat(125)) && e.raise("Lone quantifier brackets")), e.maxBackReference > e.numCapturingParens && e.raise("Invalid escape");
  for (var t = 0, i = e.backReferenceNames; t < i.length; t += 1) {
    var s = i[t];
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
    var i = false;
    if (this.options.ecmaVersion >= 9 && (i = e.eat(60)), e.eat(61) || e.eat(33)) return this.regexp_disjunction(e), e.eat(41) || e.raise("Unterminated group"), e.lastAssertionIsQuantifiable = !i, true;
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
  var i = e.pos;
  if (e.eat(123)) {
    var s = 0, l = -1;
    if (this.regexp_eatDecimalDigits(e) && (s = e.lastIntValue, e.eat(44) && this.regexp_eatDecimalDigits(e) && (l = e.lastIntValue), e.eat(125))) return l !== -1 && l < s && !t && e.raise("numbers out of order in {} quantifier"), true;
    e.switchU && !t && e.raise("Incomplete quantifier"), e.pos = i;
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
        var i = this.regexp_eatModifiers(e), s = e.eat(45);
        if (i || s) {
          for (var l = 0; l < i.length; l++) {
            var p = i.charAt(l);
            i.indexOf(p, l + 1) > -1 && e.raise("Duplicate regular expression modifiers");
          }
          if (s) {
            var m = this.regexp_eatModifiers(e);
            !i && !m && e.current() === 58 && e.raise("Invalid regular expression modifiers");
            for (var w = 0; w < m.length; w++) {
              var _ = m.charAt(w);
              (m.indexOf(_, w + 1) > -1 || i.indexOf(_) > -1) && e.raise("Duplicate regular expression modifiers");
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
  for (var t = "", i = 0; (i = e.current()) !== -1 && C1(i); ) t += Et(i), e.advance();
  return t;
};
function C1(e) {
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
  for (var t = e.pos, i = 0; (i = e.current()) !== -1 && !rm(i); ) e.advance();
  return e.pos !== t;
};
oe.regexp_eatExtendedPatternCharacter = function(e) {
  var t = e.current();
  return t !== -1 && t !== 36 && !(t >= 40 && t <= 43) && t !== 46 && t !== 63 && t !== 91 && t !== 94 && t !== 124 ? (e.advance(), true) : false;
};
oe.regexp_groupSpecifier = function(e) {
  if (e.eat(63)) {
    this.regexp_eatGroupName(e) || e.raise("Invalid group");
    var t = this.options.ecmaVersion >= 16, i = e.groupNames[e.lastStringValue];
    if (i) if (t) for (var s = 0, l = i; s < l.length; s += 1) {
      var p = l[s];
      p.separatedFrom(e.branchID) || e.raise("Duplicate capture group name");
    }
    else e.raise("Duplicate capture group name");
    t ? (i || (e.groupNames[e.lastStringValue] = [])).push(e.branchID) : e.groupNames[e.lastStringValue] = true;
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
    for (e.lastStringValue += Et(e.lastIntValue); this.regexp_eatRegExpIdentifierPart(e); ) e.lastStringValue += Et(e.lastIntValue);
    return true;
  }
  return false;
};
oe.regexp_eatRegExpIdentifierStart = function(e) {
  var t = e.pos, i = this.options.ecmaVersion >= 11, s = e.current(i);
  return e.advance(i), s === 92 && this.regexp_eatRegExpUnicodeEscapeSequence(e, i) && (s = e.lastIntValue), P1(s) ? (e.lastIntValue = s, true) : (e.pos = t, false);
};
function P1(e) {
  return Pt(e, true) || e === 36 || e === 95;
}
oe.regexp_eatRegExpIdentifierPart = function(e) {
  var t = e.pos, i = this.options.ecmaVersion >= 11, s = e.current(i);
  return e.advance(i), s === 92 && this.regexp_eatRegExpUnicodeEscapeSequence(e, i) && (s = e.lastIntValue), q1(s) ? (e.lastIntValue = s, true) : (e.pos = t, false);
};
function q1(e) {
  return Ft(e, true) || e === 36 || e === 95 || e === 8204 || e === 8205;
}
oe.regexp_eatAtomEscape = function(e) {
  return this.regexp_eatBackReference(e) || this.regexp_eatCharacterClassEscape(e) || this.regexp_eatCharacterEscape(e) || e.switchN && this.regexp_eatKGroupName(e) ? true : (e.switchU && (e.current() === 99 && e.raise("Invalid unicode escape"), e.raise("Invalid escape")), false);
};
oe.regexp_eatBackReference = function(e) {
  var t = e.pos;
  if (this.regexp_eatDecimalEscape(e)) {
    var i = e.lastIntValue;
    if (e.switchU) return i > e.maxBackReference && (e.maxBackReference = i), true;
    if (i <= e.numCapturingParens) return true;
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
  var i = e.pos, s = t || e.switchU;
  if (e.eat(117)) {
    if (this.regexp_eatFixedHexDigits(e, 4)) {
      var l = e.lastIntValue;
      if (s && l >= 55296 && l <= 56319) {
        var p = e.pos;
        if (e.eat(92) && e.eat(117) && this.regexp_eatFixedHexDigits(e, 4)) {
          var m = e.lastIntValue;
          if (m >= 56320 && m <= 57343) return e.lastIntValue = (l - 55296) * 1024 + (m - 56320) + 65536, true;
        }
        e.pos = p, e.lastIntValue = l;
      }
      return true;
    }
    if (s && e.eat(123) && this.regexp_eatHexDigits(e) && e.eat(125) && T1(e.lastIntValue)) return true;
    s && e.raise("Invalid unicode escape"), e.pos = i;
  }
  return false;
};
function T1(e) {
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
var im = 0, kt = 1, ht = 2;
oe.regexp_eatCharacterClassEscape = function(e) {
  var t = e.current();
  if (S1(t)) return e.lastIntValue = -1, e.advance(), kt;
  var i = false;
  if (e.switchU && this.options.ecmaVersion >= 9 && ((i = t === 80) || t === 112)) {
    e.lastIntValue = -1, e.advance();
    var s;
    if (e.eat(123) && (s = this.regexp_eatUnicodePropertyValueExpression(e)) && e.eat(125)) return i && s === ht && e.raise("Invalid property name"), s;
    e.raise("Invalid property name");
  }
  return im;
};
function S1(e) {
  return e === 100 || e === 68 || e === 115 || e === 83 || e === 119 || e === 87;
}
oe.regexp_eatUnicodePropertyValueExpression = function(e) {
  var t = e.pos;
  if (this.regexp_eatUnicodePropertyName(e) && e.eat(61)) {
    var i = e.lastStringValue;
    if (this.regexp_eatUnicodePropertyValue(e)) {
      var s = e.lastStringValue;
      return this.regexp_validateUnicodePropertyNameAndValue(e, i, s), kt;
    }
  }
  if (e.pos = t, this.regexp_eatLoneUnicodePropertyNameOrValue(e)) {
    var l = e.lastStringValue;
    return this.regexp_validateUnicodePropertyNameOrValue(e, l);
  }
  return im;
};
oe.regexp_validateUnicodePropertyNameAndValue = function(e, t, i) {
  er(e.unicodeProperties.nonBinary, t) || e.raise("Invalid property name"), e.unicodeProperties.nonBinary[t].test(i) || e.raise("Invalid property value");
};
oe.regexp_validateUnicodePropertyNameOrValue = function(e, t) {
  if (e.unicodeProperties.binary.test(t)) return kt;
  if (e.switchV && e.unicodeProperties.binaryOfStrings.test(t)) return ht;
  e.raise("Invalid property name");
};
oe.regexp_eatUnicodePropertyName = function(e) {
  var t = 0;
  for (e.lastStringValue = ""; sm(t = e.current()); ) e.lastStringValue += Et(t), e.advance();
  return e.lastStringValue !== "";
};
function sm(e) {
  return am(e) || e === 95;
}
oe.regexp_eatUnicodePropertyValue = function(e) {
  var t = 0;
  for (e.lastStringValue = ""; w1(t = e.current()); ) e.lastStringValue += Et(t), e.advance();
  return e.lastStringValue !== "";
};
function w1(e) {
  return sm(e) || el(e);
}
oe.regexp_eatLoneUnicodePropertyNameOrValue = function(e) {
  return this.regexp_eatUnicodePropertyValue(e);
};
oe.regexp_eatCharacterClass = function(e) {
  if (e.eat(91)) {
    var t = e.eat(94), i = this.regexp_classContents(e);
    return e.eat(93) || e.raise("Unterminated character class"), t && i === ht && e.raise("Negated character class may contain strings"), true;
  }
  return false;
};
oe.regexp_classContents = function(e) {
  return e.current() === 93 ? kt : e.switchV ? this.regexp_classSetExpression(e) : (this.regexp_nonEmptyClassRanges(e), kt);
};
oe.regexp_nonEmptyClassRanges = function(e) {
  for (; this.regexp_eatClassAtom(e); ) {
    var t = e.lastIntValue;
    if (e.eat(45) && this.regexp_eatClassAtom(e)) {
      var i = e.lastIntValue;
      e.switchU && (t === -1 || i === -1) && e.raise("Invalid character class"), t !== -1 && i !== -1 && t > i && e.raise("Range out of order in character class");
    }
  }
};
oe.regexp_eatClassAtom = function(e) {
  var t = e.pos;
  if (e.eat(92)) {
    if (this.regexp_eatClassEscape(e)) return true;
    if (e.switchU) {
      var i = e.current();
      (i === 99 || lm(i)) && e.raise("Invalid class escape"), e.raise("Invalid escape");
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
  var t = kt, i;
  if (!this.regexp_eatClassSetRange(e)) if (i = this.regexp_eatClassSetOperand(e)) {
    i === ht && (t = ht);
    for (var s = e.pos; e.eatChars([38, 38]); ) {
      if (e.current() !== 38 && (i = this.regexp_eatClassSetOperand(e))) {
        i !== ht && (t = kt);
        continue;
      }
      e.raise("Invalid character in character class");
    }
    if (s !== e.pos) return t;
    for (; e.eatChars([45, 45]); ) this.regexp_eatClassSetOperand(e) || e.raise("Invalid character in character class");
    if (s !== e.pos) return t;
  } else e.raise("Invalid character in character class");
  for (; ; ) if (!this.regexp_eatClassSetRange(e)) {
    if (i = this.regexp_eatClassSetOperand(e), !i) return t;
    i === ht && (t = ht);
  }
};
oe.regexp_eatClassSetRange = function(e) {
  var t = e.pos;
  if (this.regexp_eatClassSetCharacter(e)) {
    var i = e.lastIntValue;
    if (e.eat(45) && this.regexp_eatClassSetCharacter(e)) {
      var s = e.lastIntValue;
      return i !== -1 && s !== -1 && i > s && e.raise("Range out of order in character class"), true;
    }
    e.pos = t;
  }
  return false;
};
oe.regexp_eatClassSetOperand = function(e) {
  return this.regexp_eatClassSetCharacter(e) ? kt : this.regexp_eatClassStringDisjunction(e) || this.regexp_eatNestedClass(e);
};
oe.regexp_eatNestedClass = function(e) {
  var t = e.pos;
  if (e.eat(91)) {
    var i = e.eat(94), s = this.regexp_classContents(e);
    if (e.eat(93)) return i && s === ht && e.raise("Negated character class may contain strings"), s;
    e.pos = t;
  }
  if (e.eat(92)) {
    var l = this.regexp_eatCharacterClassEscape(e);
    if (l) return l;
    e.pos = t;
  }
  return null;
};
oe.regexp_eatClassStringDisjunction = function(e) {
  var t = e.pos;
  if (e.eatChars([92, 113])) {
    if (e.eat(123)) {
      var i = this.regexp_classStringDisjunctionContents(e);
      if (e.eat(125)) return i;
    } else e.raise("Invalid escape");
    e.pos = t;
  }
  return null;
};
oe.regexp_classStringDisjunctionContents = function(e) {
  for (var t = this.regexp_classString(e); e.eat(124); ) this.regexp_classString(e) === ht && (t = ht);
  return t;
};
oe.regexp_classString = function(e) {
  for (var t = 0; this.regexp_eatClassSetCharacter(e); ) t++;
  return t === 1 ? kt : ht;
};
oe.regexp_eatClassSetCharacter = function(e) {
  var t = e.pos;
  if (e.eat(92)) return this.regexp_eatCharacterEscape(e) || this.regexp_eatClassSetReservedPunctuator(e) ? true : e.eat(98) ? (e.lastIntValue = 8, true) : (e.pos = t, false);
  var i = e.current();
  return i < 0 || i === e.lookahead() && A1(i) || E1(i) ? false : (e.advance(), e.lastIntValue = i, true);
};
function A1(e) {
  return e === 33 || e >= 35 && e <= 38 || e >= 42 && e <= 44 || e === 46 || e >= 58 && e <= 64 || e === 94 || e === 96 || e === 126;
}
function E1(e) {
  return e === 40 || e === 41 || e === 45 || e === 47 || e >= 91 && e <= 93 || e >= 123 && e <= 125;
}
oe.regexp_eatClassSetReservedPunctuator = function(e) {
  var t = e.current();
  return k1(t) ? (e.lastIntValue = t, e.advance(), true) : false;
};
function k1(e) {
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
  var t = e.pos, i = 0;
  for (e.lastIntValue = 0; el(i = e.current()); ) e.lastIntValue = 10 * e.lastIntValue + (i - 48), e.advance();
  return e.pos !== t;
};
function el(e) {
  return e >= 48 && e <= 57;
}
oe.regexp_eatHexDigits = function(e) {
  var t = e.pos, i = 0;
  for (e.lastIntValue = 0; nm(i = e.current()); ) e.lastIntValue = 16 * e.lastIntValue + om(i), e.advance();
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
      var i = e.lastIntValue;
      t <= 3 && this.regexp_eatOctalDigit(e) ? e.lastIntValue = t * 64 + i * 8 + e.lastIntValue : e.lastIntValue = t * 8 + i;
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
  var i = e.pos;
  e.lastIntValue = 0;
  for (var s = 0; s < t; ++s) {
    var l = e.current();
    if (!nm(l)) return e.pos = i, false;
    e.lastIntValue = 16 * e.lastIntValue + om(l), e.advance();
  }
  return true;
};
var tl = function(t) {
  this.type = t.type, this.value = t.value, this.start = t.start, this.end = t.end, t.options.locations && (this.loc = new Po(t, t.startLoc, t.endLoc)), t.options.ranges && (this.range = [t.start, t.end]);
}, Pe = He.prototype;
Pe.next = function(e) {
  !e && this.type.keyword && this.containsEsc && this.raiseRecoverable(this.start, "Escape sequence in keyword " + this.type.keyword), this.options.onToken && this.options.onToken(new tl(this)), this.lastTokEnd = this.end, this.lastTokStart = this.start, this.lastTokEndLoc = this.endLoc, this.lastTokStartLoc = this.startLoc, this.nextToken();
};
Pe.getToken = function() {
  return this.next(), new tl(this);
};
typeof Symbol < "u" && (Pe[Symbol.iterator] = function() {
  var e = this;
  return { next: function() {
    var t = e.getToken();
    return { done: t.type === d.eof, value: t };
  } };
});
Pe.nextToken = function() {
  var e = this.curContext();
  if ((!e || !e.preserveSpace) && this.skipSpace(), this.start = this.pos, this.options.locations && (this.startLoc = this.curPosition()), this.pos >= this.input.length) return this.finishToken(d.eof);
  if (e.override) return e.override(this);
  this.readToken(this.fullCharCodeAtPos());
};
Pe.readToken = function(e) {
  return Pt(e, this.options.ecmaVersion >= 6) || e === 92 ? this.readWord() : this.getTokenFromCode(e);
};
Pe.fullCharCodeAtPos = function() {
  var e = this.input.charCodeAt(this.pos);
  if (e <= 55295 || e >= 56320) return e;
  var t = this.input.charCodeAt(this.pos + 1);
  return t <= 56319 || t >= 57344 ? e : (e << 10) + t - 56613888;
};
Pe.skipBlockComment = function() {
  var e = this.options.onComment && this.curPosition(), t = this.pos, i = this.input.indexOf("*/", this.pos += 2);
  if (i === -1 && this.raise(this.pos - 2, "Unterminated comment"), this.pos = i + 2, this.options.locations) for (var s = void 0, l = t; (s = Mh(this.input, l, this.pos)) > -1; ) ++this.curLine, l = this.lineStart = s;
  this.options.onComment && this.options.onComment(true, this.input.slice(t + 2, i), t, this.pos, e, this.curPosition());
};
Pe.skipLineComment = function(e) {
  for (var t = this.pos, i = this.options.onComment && this.curPosition(), s = this.input.charCodeAt(this.pos += e); this.pos < this.input.length && !$t(s); ) s = this.input.charCodeAt(++this.pos);
  this.options.onComment && this.options.onComment(false, this.input.slice(t + e, this.pos), t, this.pos, i, this.curPosition());
};
Pe.skipSpace = function() {
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
Pe.finishToken = function(e, t) {
  this.end = this.pos, this.options.locations && (this.endLoc = this.curPosition());
  var i = this.type;
  this.type = e, this.value = t, this.updateContext(i);
};
Pe.readToken_dot = function() {
  var e = this.input.charCodeAt(this.pos + 1);
  if (e >= 48 && e <= 57) return this.readNumber(true);
  var t = this.input.charCodeAt(this.pos + 2);
  return this.options.ecmaVersion >= 6 && e === 46 && t === 46 ? (this.pos += 3, this.finishToken(d.ellipsis)) : (++this.pos, this.finishToken(d.dot));
};
Pe.readToken_slash = function() {
  var e = this.input.charCodeAt(this.pos + 1);
  return this.exprAllowed ? (++this.pos, this.readRegexp()) : e === 61 ? this.finishOp(d.assign, 2) : this.finishOp(d.slash, 1);
};
Pe.readToken_mult_modulo_exp = function(e) {
  var t = this.input.charCodeAt(this.pos + 1), i = 1, s = e === 42 ? d.star : d.modulo;
  return this.options.ecmaVersion >= 7 && e === 42 && t === 42 && (++i, s = d.starstar, t = this.input.charCodeAt(this.pos + 2)), t === 61 ? this.finishOp(d.assign, i + 1) : this.finishOp(s, i);
};
Pe.readToken_pipe_amp = function(e) {
  var t = this.input.charCodeAt(this.pos + 1);
  if (t === e) {
    if (this.options.ecmaVersion >= 12) {
      var i = this.input.charCodeAt(this.pos + 2);
      if (i === 61) return this.finishOp(d.assign, 3);
    }
    return this.finishOp(e === 124 ? d.logicalOR : d.logicalAND, 2);
  }
  return t === 61 ? this.finishOp(d.assign, 2) : this.finishOp(e === 124 ? d.bitwiseOR : d.bitwiseAND, 1);
};
Pe.readToken_caret = function() {
  var e = this.input.charCodeAt(this.pos + 1);
  return e === 61 ? this.finishOp(d.assign, 2) : this.finishOp(d.bitwiseXOR, 1);
};
Pe.readToken_plus_min = function(e) {
  var t = this.input.charCodeAt(this.pos + 1);
  return t === e ? t === 45 && !this.inModule && this.input.charCodeAt(this.pos + 2) === 62 && (this.lastTokEnd === 0 || ut.test(this.input.slice(this.lastTokEnd, this.pos))) ? (this.skipLineComment(3), this.skipSpace(), this.nextToken()) : this.finishOp(d.incDec, 2) : t === 61 ? this.finishOp(d.assign, 2) : this.finishOp(d.plusMin, 1);
};
Pe.readToken_lt_gt = function(e) {
  var t = this.input.charCodeAt(this.pos + 1), i = 1;
  return t === e ? (i = e === 62 && this.input.charCodeAt(this.pos + 2) === 62 ? 3 : 2, this.input.charCodeAt(this.pos + i) === 61 ? this.finishOp(d.assign, i + 1) : this.finishOp(d.bitShift, i)) : t === 33 && e === 60 && !this.inModule && this.input.charCodeAt(this.pos + 2) === 45 && this.input.charCodeAt(this.pos + 3) === 45 ? (this.skipLineComment(4), this.skipSpace(), this.nextToken()) : (t === 61 && (i = 2), this.finishOp(d.relational, i));
};
Pe.readToken_eq_excl = function(e) {
  var t = this.input.charCodeAt(this.pos + 1);
  return t === 61 ? this.finishOp(d.equality, this.input.charCodeAt(this.pos + 2) === 61 ? 3 : 2) : e === 61 && t === 62 && this.options.ecmaVersion >= 6 ? (this.pos += 2, this.finishToken(d.arrow)) : this.finishOp(e === 61 ? d.eq : d.prefix, 1);
};
Pe.readToken_question = function() {
  var e = this.options.ecmaVersion;
  if (e >= 11) {
    var t = this.input.charCodeAt(this.pos + 1);
    if (t === 46) {
      var i = this.input.charCodeAt(this.pos + 2);
      if (i < 48 || i > 57) return this.finishOp(d.questionDot, 2);
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
Pe.readToken_numberSign = function() {
  var e = this.options.ecmaVersion, t = 35;
  if (e >= 13 && (++this.pos, t = this.fullCharCodeAtPos(), Pt(t, true) || t === 92)) return this.finishToken(d.privateId, this.readWord1());
  this.raise(this.pos, "Unexpected character '" + Et(t) + "'");
};
Pe.getTokenFromCode = function(e) {
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
  this.raise(this.pos, "Unexpected character '" + Et(e) + "'");
};
Pe.finishOp = function(e, t) {
  var i = this.input.slice(this.pos, this.pos + t);
  return this.pos += t, this.finishToken(e, i);
};
Pe.readRegexp = function() {
  for (var e, t, i = this.pos; ; ) {
    this.pos >= this.input.length && this.raise(i, "Unterminated regular expression");
    var s = this.input.charAt(this.pos);
    if (ut.test(s) && this.raise(i, "Unterminated regular expression"), e) e = false;
    else {
      if (s === "[") t = true;
      else if (s === "]" && t) t = false;
      else if (s === "/" && !t) break;
      e = s === "\\";
    }
    ++this.pos;
  }
  var l = this.input.slice(i, this.pos);
  ++this.pos;
  var p = this.pos, m = this.readWord1();
  this.containsEsc && this.unexpected(p);
  var w = this.regexpState || (this.regexpState = new Tt(this));
  w.reset(i, l, m), this.validateRegExpFlags(w), this.validateRegExpPattern(w);
  var _ = null;
  try {
    _ = new RegExp(l, m);
  } catch {
  }
  return this.finishToken(d.regexp, { pattern: l, flags: m, value: _ });
};
Pe.readInt = function(e, t, i) {
  for (var s = this.options.ecmaVersion >= 12 && t === void 0, l = i && this.input.charCodeAt(this.pos) === 48, p = this.pos, m = 0, w = 0, _ = 0, o = t ?? 1 / 0; _ < o; ++_, ++this.pos) {
    var b = this.input.charCodeAt(this.pos), L = void 0;
    if (s && b === 95) {
      l && this.raiseRecoverable(this.pos, "Numeric separator is not allowed in legacy octal numeric literals"), w === 95 && this.raiseRecoverable(this.pos, "Numeric separator must be exactly one underscore"), _ === 0 && this.raiseRecoverable(this.pos, "Numeric separator is not allowed at the first of digits"), w = b;
      continue;
    }
    if (b >= 97 ? L = b - 97 + 10 : b >= 65 ? L = b - 65 + 10 : b >= 48 && b <= 57 ? L = b - 48 : L = 1 / 0, L >= e) break;
    w = b, m = m * e + L;
  }
  return s && w === 95 && this.raiseRecoverable(this.pos - 1, "Numeric separator is not allowed at the last of digits"), this.pos === p || t != null && this.pos - p !== t ? null : m;
};
function I1(e, t) {
  return t ? parseInt(e, 8) : parseFloat(e.replace(/_/g, ""));
}
function um(e) {
  return typeof BigInt != "function" ? null : BigInt(e.replace(/_/g, ""));
}
Pe.readRadixNumber = function(e) {
  var t = this.pos;
  this.pos += 2;
  var i = this.readInt(e);
  return i == null && this.raise(this.start + 2, "Expected number in radix " + e), this.options.ecmaVersion >= 11 && this.input.charCodeAt(this.pos) === 110 ? (i = um(this.input.slice(t, this.pos)), ++this.pos) : Pt(this.fullCharCodeAtPos()) && this.raise(this.pos, "Identifier directly after number"), this.finishToken(d.num, i);
};
Pe.readNumber = function(e) {
  var t = this.pos;
  !e && this.readInt(10, void 0, true) === null && this.raise(t, "Invalid number");
  var i = this.pos - t >= 2 && this.input.charCodeAt(t) === 48;
  i && this.strict && this.raise(t, "Invalid number");
  var s = this.input.charCodeAt(this.pos);
  if (!i && !e && this.options.ecmaVersion >= 11 && s === 110) {
    var l = um(this.input.slice(t, this.pos));
    return ++this.pos, Pt(this.fullCharCodeAtPos()) && this.raise(this.pos, "Identifier directly after number"), this.finishToken(d.num, l);
  }
  i && /[89]/.test(this.input.slice(t, this.pos)) && (i = false), s === 46 && !i && (++this.pos, this.readInt(10), s = this.input.charCodeAt(this.pos)), (s === 69 || s === 101) && !i && (s = this.input.charCodeAt(++this.pos), (s === 43 || s === 45) && ++this.pos, this.readInt(10) === null && this.raise(t, "Invalid number")), Pt(this.fullCharCodeAtPos()) && this.raise(this.pos, "Identifier directly after number");
  var p = I1(this.input.slice(t, this.pos), i);
  return this.finishToken(d.num, p);
};
Pe.readCodePoint = function() {
  var e = this.input.charCodeAt(this.pos), t;
  if (e === 123) {
    this.options.ecmaVersion < 6 && this.unexpected();
    var i = ++this.pos;
    t = this.readHexChar(this.input.indexOf("}", this.pos) - this.pos), ++this.pos, t > 1114111 && this.invalidStringToken(i, "Code point out of bounds");
  } else t = this.readHexChar(4);
  return t;
};
Pe.readString = function(e) {
  for (var t = "", i = ++this.pos; ; ) {
    this.pos >= this.input.length && this.raise(this.start, "Unterminated string constant");
    var s = this.input.charCodeAt(this.pos);
    if (s === e) break;
    s === 92 ? (t += this.input.slice(i, this.pos), t += this.readEscapedChar(false), i = this.pos) : s === 8232 || s === 8233 ? (this.options.ecmaVersion < 10 && this.raise(this.start, "Unterminated string constant"), ++this.pos, this.options.locations && (this.curLine++, this.lineStart = this.pos)) : ($t(s) && this.raise(this.start, "Unterminated string constant"), ++this.pos);
  }
  return t += this.input.slice(i, this.pos++), this.finishToken(d.string, t);
};
var cm = {};
Pe.tryReadTemplateToken = function() {
  this.inTemplateElement = true;
  try {
    this.readTmplToken();
  } catch (e) {
    if (e === cm) this.readInvalidTemplateToken();
    else throw e;
  }
  this.inTemplateElement = false;
};
Pe.invalidStringToken = function(e, t) {
  if (this.inTemplateElement && this.options.ecmaVersion >= 9) throw cm;
  this.raise(e, t);
};
Pe.readTmplToken = function() {
  for (var e = "", t = this.pos; ; ) {
    this.pos >= this.input.length && this.raise(this.start, "Unterminated template");
    var i = this.input.charCodeAt(this.pos);
    if (i === 96 || i === 36 && this.input.charCodeAt(this.pos + 1) === 123) return this.pos === this.start && (this.type === d.template || this.type === d.invalidTemplate) ? i === 36 ? (this.pos += 2, this.finishToken(d.dollarBraceL)) : (++this.pos, this.finishToken(d.backQuote)) : (e += this.input.slice(t, this.pos), this.finishToken(d.template, e));
    if (i === 92) e += this.input.slice(t, this.pos), e += this.readEscapedChar(true), t = this.pos;
    else if ($t(i)) {
      switch (e += this.input.slice(t, this.pos), ++this.pos, i) {
        case 13:
          this.input.charCodeAt(this.pos) === 10 && ++this.pos;
        case 10:
          e += `
`;
          break;
        default:
          e += String.fromCharCode(i);
          break;
      }
      this.options.locations && (++this.curLine, this.lineStart = this.pos), t = this.pos;
    } else ++this.pos;
  }
};
Pe.readInvalidTemplateToken = function() {
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
Pe.readEscapedChar = function(e) {
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
      return Et(this.readCodePoint());
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
        var i = this.pos - 1;
        this.invalidStringToken(i, "Invalid escape sequence in template string");
      }
    default:
      if (t >= 48 && t <= 55) {
        var s = this.input.substr(this.pos - 1, 3).match(/^[0-7]+/)[0], l = parseInt(s, 8);
        return l > 255 && (s = s.slice(0, -1), l = parseInt(s, 8)), this.pos += s.length - 1, t = this.input.charCodeAt(this.pos), (s !== "0" || t === 56 || t === 57) && (this.strict || e) && this.invalidStringToken(this.pos - 1 - s.length, e ? "Octal literal in template string" : "Octal literal in strict mode"), String.fromCharCode(l);
      }
      return $t(t) ? (this.options.locations && (this.lineStart = this.pos, ++this.curLine), "") : String.fromCharCode(t);
  }
};
Pe.readHexChar = function(e) {
  var t = this.pos, i = this.readInt(16, e);
  return i === null && this.invalidStringToken(t, "Bad character escape sequence"), i;
};
Pe.readWord1 = function() {
  this.containsEsc = false;
  for (var e = "", t = true, i = this.pos, s = this.options.ecmaVersion >= 6; this.pos < this.input.length; ) {
    var l = this.fullCharCodeAtPos();
    if (Ft(l, s)) this.pos += l <= 65535 ? 1 : 2;
    else if (l === 92) {
      this.containsEsc = true, e += this.input.slice(i, this.pos);
      var p = this.pos;
      this.input.charCodeAt(++this.pos) !== 117 && this.invalidStringToken(this.pos, "Expecting Unicode escape sequence \\uXXXX"), ++this.pos;
      var m = this.readCodePoint();
      (t ? Pt : Ft)(m, s) || this.invalidStringToken(p, "Invalid Unicode escape"), e += Et(m), i = this.pos;
    } else break;
    t = false;
  }
  return e + this.input.slice(i, this.pos);
};
Pe.readWord = function() {
  var e = this.readWord1(), t = d.name;
  return this.keywords.test(e) && (t = Kt[e]), this.finishToken(t, e);
};
var dm = "8.14.0";
He.acorn = { Parser: He, version: dm, defaultOptions: Wo, Position: Jt, SourceLocation: Po, getLineInfo: Pl, Node: To, TokenType: Ce, tokTypes: d, keywordTypes: Kt, TokContext: rt, tokContexts: Fe, isIdentifierChar: Ft, isIdentifierStart: Pt, Token: tl, isNewLine: $t, lineBreak: ut, lineBreakG: Ih, nonASCIIwhitespace: Cl };
function M1(e, t) {
  return He.parse(e, t);
}
function L1(e, t, i) {
  return He.parseExpressionAt(e, t, i);
}
function O1(e, t) {
  return He.tokenizer(e, t);
}
const N1 = Object.freeze(Object.defineProperty({ __proto__: null, Node: To, Parser: He, Position: Jt, SourceLocation: Po, TokContext: rt, Token: tl, TokenType: Ce, defaultOptions: Wo, getLineInfo: Pl, isIdentifierChar: Ft, isIdentifierStart: Pt, isNewLine: $t, keywordTypes: Kt, lineBreak: ut, lineBreakG: Ih, nonASCIIwhitespace: Cl, parse: M1, parseExpressionAt: L1, tokContexts: Fe, tokTypes: d, tokenizer: O1, version: dm }, Symbol.toStringTag, { value: "Module" }));
function zl(e, t) {
  for (var i = 0; i < t.length; i++) {
    var s = t[i];
    s.enumerable = s.enumerable || false, s.configurable = true, "value" in s && (s.writable = true), Object.defineProperty(e, typeof (l = function(p, m) {
      if (typeof p != "object" || p === null) return p;
      var w = p[Symbol.toPrimitive];
      if (w !== void 0) {
        var _ = w.call(p, "string");
        if (typeof _ != "object") return _;
        throw new TypeError("@@toPrimitive must return a primitive value.");
      }
      return String(p);
    }(s.key)) == "symbol" ? l : String(l), s);
  }
  var l;
}
function Qo() {
  return Qo = Object.assign ? Object.assign.bind() : function(e) {
    for (var t = 1; t < arguments.length; t++) {
      var i = arguments[t];
      for (var s in i) Object.prototype.hasOwnProperty.call(i, s) && (e[s] = i[s]);
    }
    return e;
  }, Qo.apply(this, arguments);
}
function Oo(e, t) {
  e.prototype = Object.create(t.prototype), e.prototype.constructor = e, yl(e, t);
}
function yl(e, t) {
  return yl = Object.setPrototypeOf ? Object.setPrototypeOf.bind() : function(i, s) {
    return i.__proto__ = s, i;
  }, yl(e, t);
}
function Xl(e, t) {
  (t == null || t > e.length) && (t = e.length);
  for (var i = 0, s = new Array(t); i < t; i++) s[i] = e[i];
  return s;
}
function Kl(e, t) {
  var i = typeof Symbol < "u" && e[Symbol.iterator] || e["@@iterator"];
  if (i) return (i = i.call(e)).next.bind(i);
  if (Array.isArray(e) || (i = function(l, p) {
    if (l) {
      if (typeof l == "string") return Xl(l, p);
      var m = Object.prototype.toString.call(l).slice(8, -1);
      return m === "Object" && l.constructor && (m = l.constructor.name), m === "Map" || m === "Set" ? Array.from(l) : m === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(m) ? Xl(l, p) : void 0;
    }
  }(e)) || t) {
    i && (e = i);
    var s = 0;
    return function() {
      return s >= e.length ? { done: true } : { done: false, value: e[s++] };
    };
  }
  throw new TypeError(`Invalid attempt to iterate non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
}
var nt = true;
function ot(e, t) {
  return t === void 0 && (t = {}), new Ce("name", t);
}
var D1 = /* @__PURE__ */ new WeakMap();
function j1(e) {
  var t = D1.get(e.Parser.acorn || e);
  if (!t) {
    var i = { assert: ot(0, { startsExpr: nt }), asserts: ot(0, { startsExpr: nt }), global: ot(0, { startsExpr: nt }), keyof: ot(0, { startsExpr: nt }), readonly: ot(0, { startsExpr: nt }), unique: ot(0, { startsExpr: nt }), abstract: ot(0, { startsExpr: nt }), declare: ot(0, { startsExpr: nt }), enum: ot(0, { startsExpr: nt }), module: ot(0, { startsExpr: nt }), namespace: ot(0, { startsExpr: nt }), interface: ot(0, { startsExpr: nt }), type: ot(0, { startsExpr: nt }) }, s = { at: new Ce("@"), jsxName: new Ce("jsxName"), jsxText: new Ce("jsxText", { beforeExpr: true }), jsxTagStart: new Ce("jsxTagStart", { startsExpr: true }), jsxTagEnd: new Ce("jsxTagEnd") }, l = { tc_oTag: new rt("<tag", false, false), tc_cTag: new rt("</tag", false, false), tc_expr: new rt("<tag>...</tag>", true, true) }, p = new RegExp("^(?:" + Object.keys(i).join("|") + ")$");
    s.jsxTagStart.updateContext = function() {
      this.context.push(l.tc_expr), this.context.push(l.tc_oTag), this.exprAllowed = false;
    }, s.jsxTagEnd.updateContext = function(m) {
      var w = this.context.pop();
      w === l.tc_oTag && m === d.slash || w === l.tc_cTag ? (this.context.pop(), this.exprAllowed = this.curContext() === l.tc_expr) : this.exprAllowed = true;
    }, t = { tokTypes: Qo({}, i, s), tokContexts: Qo({}, l), keywordsRegExp: p, tokenIsLiteralPropertyName: function(m) {
      return [d.name, d.string, d.num].concat(Object.values(Kt), Object.values(i)).includes(m);
    }, tokenIsKeywordOrIdentifier: function(m) {
      return [d.name].concat(Object.values(Kt), Object.values(i)).includes(m);
    }, tokenIsIdentifier: function(m) {
      return [].concat(Object.values(i), [d.name]).includes(m);
    }, tokenIsTSDeclarationStart: function(m) {
      return [i.abstract, i.declare, i.enum, i.module, i.namespace, i.interface, i.type].includes(m);
    }, tokenIsTSTypeOperator: function(m) {
      return [i.keyof, i.readonly, i.unique].includes(m);
    }, tokenIsTemplate: function(m) {
      return m === d.invalidTemplate;
    } };
  }
  return t;
}
var ir = 1024, B1 = new RegExp("(?:[^\\S\\n\\r\\u2028\\u2029]|\\/\\/.*|\\/\\*.*?\\*\\/)*", "y"), Jl = new RegExp("(?=(" + B1.source + "))\\1" + /(?=[\n\r\u2028\u2029]|\/\*(?!.*?\*\/)|$)/.source, "y"), sr = function() {
  this.shorthandAssign = void 0, this.trailingComma = void 0, this.parenthesizedAssign = void 0, this.parenthesizedBind = void 0, this.doubleProto = void 0, this.shorthandAssign = this.trailingComma = this.parenthesizedAssign = this.parenthesizedBind = this.doubleProto = -1;
};
function F1(e, t) {
  var i = t.key.name, s = e[i], l = "true";
  return t.type !== "MethodDefinition" || t.kind !== "get" && t.kind !== "set" || (l = (t.static ? "s" : "i") + t.kind), s === "iget" && l === "iset" || s === "iset" && l === "iget" || s === "sget" && l === "sset" || s === "sset" && l === "sget" ? (e[i] = "true", false) : !!s || (e[i] = l, false);
}
function Ql(e, t) {
  var i = e.key;
  return !e.computed && (i.type === "Identifier" && i.name === t || i.type === "Literal" && i.value === t);
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
}, LetInLexicalBinding: "'let' is not allowed to be used as a name in 'let' or 'const' declarations." }, V1 = { quot: '"', amp: "&", apos: "'", lt: "<", gt: ">", nbsp: "\xA0", iexcl: "\xA1", cent: "\xA2", pound: "\xA3", curren: "\xA4", yen: "\xA5", brvbar: "\xA6", sect: "\xA7", uml: "\xA8", copy: "\xA9", ordf: "\xAA", laquo: "\xAB", not: "\xAC", shy: "\xAD", reg: "\xAE", macr: "\xAF", deg: "\xB0", plusmn: "\xB1", sup2: "\xB2", sup3: "\xB3", acute: "\xB4", micro: "\xB5", para: "\xB6", middot: "\xB7", cedil: "\xB8", sup1: "\xB9", ordm: "\xBA", raquo: "\xBB", frac14: "\xBC", frac12: "\xBD", frac34: "\xBE", iquest: "\xBF", Agrave: "\xC0", Aacute: "\xC1", Acirc: "\xC2", Atilde: "\xC3", Auml: "\xC4", Aring: "\xC5", AElig: "\xC6", Ccedil: "\xC7", Egrave: "\xC8", Eacute: "\xC9", Ecirc: "\xCA", Euml: "\xCB", Igrave: "\xCC", Iacute: "\xCD", Icirc: "\xCE", Iuml: "\xCF", ETH: "\xD0", Ntilde: "\xD1", Ograve: "\xD2", Oacute: "\xD3", Ocirc: "\xD4", Otilde: "\xD5", Ouml: "\xD6", times: "\xD7", Oslash: "\xD8", Ugrave: "\xD9", Uacute: "\xDA", Ucirc: "\xDB", Uuml: "\xDC", Yacute: "\xDD", THORN: "\xDE", szlig: "\xDF", agrave: "\xE0", aacute: "\xE1", acirc: "\xE2", atilde: "\xE3", auml: "\xE4", aring: "\xE5", aelig: "\xE6", ccedil: "\xE7", egrave: "\xE8", eacute: "\xE9", ecirc: "\xEA", euml: "\xEB", igrave: "\xEC", iacute: "\xED", icirc: "\xEE", iuml: "\xEF", eth: "\xF0", ntilde: "\xF1", ograve: "\xF2", oacute: "\xF3", ocirc: "\xF4", otilde: "\xF5", ouml: "\xF6", divide: "\xF7", oslash: "\xF8", ugrave: "\xF9", uacute: "\xFA", ucirc: "\xFB", uuml: "\xFC", yacute: "\xFD", thorn: "\xFE", yuml: "\xFF", OElig: "\u0152", oelig: "\u0153", Scaron: "\u0160", scaron: "\u0161", Yuml: "\u0178", fnof: "\u0192", circ: "\u02C6", tilde: "\u02DC", Alpha: "\u0391", Beta: "\u0392", Gamma: "\u0393", Delta: "\u0394", Epsilon: "\u0395", Zeta: "\u0396", Eta: "\u0397", Theta: "\u0398", Iota: "\u0399", Kappa: "\u039A", Lambda: "\u039B", Mu: "\u039C", Nu: "\u039D", Xi: "\u039E", Omicron: "\u039F", Pi: "\u03A0", Rho: "\u03A1", Sigma: "\u03A3", Tau: "\u03A4", Upsilon: "\u03A5", Phi: "\u03A6", Chi: "\u03A7", Psi: "\u03A8", Omega: "\u03A9", alpha: "\u03B1", beta: "\u03B2", gamma: "\u03B3", delta: "\u03B4", epsilon: "\u03B5", zeta: "\u03B6", eta: "\u03B7", theta: "\u03B8", iota: "\u03B9", kappa: "\u03BA", lambda: "\u03BB", mu: "\u03BC", nu: "\u03BD", xi: "\u03BE", omicron: "\u03BF", pi: "\u03C0", rho: "\u03C1", sigmaf: "\u03C2", sigma: "\u03C3", tau: "\u03C4", upsilon: "\u03C5", phi: "\u03C6", chi: "\u03C7", psi: "\u03C8", omega: "\u03C9", thetasym: "\u03D1", upsih: "\u03D2", piv: "\u03D6", ensp: "\u2002", emsp: "\u2003", thinsp: "\u2009", zwnj: "\u200C", zwj: "\u200D", lrm: "\u200E", rlm: "\u200F", ndash: "\u2013", mdash: "\u2014", lsquo: "\u2018", rsquo: "\u2019", sbquo: "\u201A", ldquo: "\u201C", rdquo: "\u201D", bdquo: "\u201E", dagger: "\u2020", Dagger: "\u2021", bull: "\u2022", hellip: "\u2026", permil: "\u2030", prime: "\u2032", Prime: "\u2033", lsaquo: "\u2039", rsaquo: "\u203A", oline: "\u203E", frasl: "\u2044", euro: "\u20AC", image: "\u2111", weierp: "\u2118", real: "\u211C", trade: "\u2122", alefsym: "\u2135", larr: "\u2190", uarr: "\u2191", rarr: "\u2192", darr: "\u2193", harr: "\u2194", crarr: "\u21B5", lArr: "\u21D0", uArr: "\u21D1", rArr: "\u21D2", dArr: "\u21D3", hArr: "\u21D4", forall: "\u2200", part: "\u2202", exist: "\u2203", empty: "\u2205", nabla: "\u2207", isin: "\u2208", notin: "\u2209", ni: "\u220B", prod: "\u220F", sum: "\u2211", minus: "\u2212", lowast: "\u2217", radic: "\u221A", prop: "\u221D", infin: "\u221E", ang: "\u2220", and: "\u2227", or: "\u2228", cap: "\u2229", cup: "\u222A", int: "\u222B", there4: "\u2234", sim: "\u223C", cong: "\u2245", asymp: "\u2248", ne: "\u2260", equiv: "\u2261", le: "\u2264", ge: "\u2265", sub: "\u2282", sup: "\u2283", nsub: "\u2284", sube: "\u2286", supe: "\u2287", oplus: "\u2295", otimes: "\u2297", perp: "\u22A5", sdot: "\u22C5", lceil: "\u2308", rceil: "\u2309", lfloor: "\u230A", rfloor: "\u230B", lang: "\u2329", rang: "\u232A", loz: "\u25CA", spades: "\u2660", clubs: "\u2663", hearts: "\u2665", diams: "\u2666" }, $1 = /^[\da-fA-F]+$/, H1 = /^\d+$/;
function xo(e) {
  return e && (e.type === "JSXIdentifier" ? e.name : e.type === "JSXNamespacedName" ? e.namespace.name + ":" + e.name.name : e.type === "JSXMemberExpression" ? xo(e.object) + "." + xo(e.property) : void 0);
}
var dl = /(?:\s|\/\/.*|\/\*[^]*?\*\/)*/g;
function Yl(e) {
  if (!e) throw new Error("Assert fail");
}
function U1(e) {
  return e === "accessor";
}
function G1(e) {
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
function z1(e) {
  var t = e || {}, i = t.dts, s = i !== void 0 && i, l = t.allowSatisfies, p = l !== void 0 && l;
  return function(m) {
    var w = m.acorn || N1, _ = j1(w), o = w.tokTypes, b = w.keywordTypes, L = w.isIdentifierStart, O = w.lineBreak, B = w.isNewLine, G = w.tokContexts, F = w.isIdentifierChar, R = _.tokTypes, v = _.tokContexts, h = _.keywordsRegExp, g = _.tokenIsLiteralPropertyName, S = _.tokenIsTemplate, C = _.tokenIsTSDeclarationStart, x = _.tokenIsIdentifier, E = _.tokenIsKeywordOrIdentifier, D = _.tokenIsTSTypeOperator;
    function z(A, N, I) {
      I === void 0 && (I = A.length);
      for (var M = N; M < I; M++) {
        var V = A.charCodeAt(M);
        if (B(V)) return M < I - 1 && V === 13 && A.charCodeAt(M + 1) === 10 ? M + 2 : M + 1;
      }
      return -1;
    }
    m = function(A, N, I) {
      var M = I.tokTypes, V = N.tokTypes;
      return function(c) {
        function r() {
          return c.apply(this, arguments) || this;
        }
        Oo(r, c);
        var a = r.prototype;
        return a.takeDecorators = function(n) {
          var u = this.decoratorStack[this.decoratorStack.length - 1];
          u.length && (n.decorators = u, this.resetStartLocationFromNode(n, u[0]), this.decoratorStack[this.decoratorStack.length - 1] = []);
        }, a.parseDecorators = function(n) {
          for (var u = this.decoratorStack[this.decoratorStack.length - 1]; this.match(V.at); ) {
            var f = this.parseDecorator();
            u.push(f);
          }
          this.match(M._export) ? n || this.unexpected() : this.canHaveLeadingDecorator() || this.raise(this.start, "Leading decorators must be attached to a class declaration.");
        }, a.parseDecorator = function() {
          var n = this.startNode();
          this.next(), this.decoratorStack.push([]);
          var u, f = this.start, y = this.startLoc;
          if (this.match(M.parenL)) {
            var P = this.start, T = this.startLoc;
            if (this.next(), u = this.parseExpression(), this.expect(M.parenR), this.options.preserveParens) {
              var q = this.startNodeAt(P, T);
              q.expression = u, u = this.finishNode(q, "ParenthesizedExpression");
            }
          } else for (u = this.parseIdent(false); this.eat(M.dot); ) {
            var H = this.startNodeAt(f, y);
            H.object = u, H.property = this.parseIdent(true), H.computed = false, u = this.finishNode(H, "MemberExpression");
          }
          return n.expression = this.parseMaybeDecoratorArguments(u), this.decoratorStack.pop(), this.finishNode(n, "Decorator");
        }, a.parseMaybeDecoratorArguments = function(n) {
          if (this.eat(M.parenL)) {
            var u = this.startNodeAtNode(n);
            return u.callee = n, u.arguments = this.parseExprList(M.parenR, false), this.finishNode(u, "CallExpression");
          }
          return n;
        }, r;
      }(A);
    }(m, _, w), m = function(A, N, I, M) {
      var V = A.tokTypes, c = N.tokTypes, r = A.isNewLine, a = A.isIdentifierChar, n = Object.assign({ allowNamespaces: true, allowNamespacedObjects: true }, M || {});
      return function(u) {
        function f() {
          return u.apply(this, arguments) || this;
        }
        Oo(f, u);
        var y = f.prototype;
        return y.jsx_readToken = function() {
          for (var P = "", T = this.pos; ; ) {
            this.pos >= this.input.length && this.raise(this.start, "Unterminated JSX contents");
            var q = this.input.charCodeAt(this.pos);
            switch (q) {
              case 60:
              case 123:
                return this.pos === this.start ? q === 60 && this.exprAllowed ? (++this.pos, this.finishToken(c.jsxTagStart)) : this.getTokenFromCode(q) : (P += this.input.slice(T, this.pos), this.finishToken(c.jsxText, P));
              case 38:
                P += this.input.slice(T, this.pos), P += this.jsx_readEntity(), T = this.pos;
                break;
              case 62:
              case 125:
                this.raise(this.pos, "Unexpected token `" + this.input[this.pos] + "`. Did you mean `" + (q === 62 ? "&gt;" : "&rbrace;") + '` or `{"' + this.input[this.pos] + '"}`?');
              default:
                r(q) ? (P += this.input.slice(T, this.pos), P += this.jsx_readNewLine(true), T = this.pos) : ++this.pos;
            }
          }
        }, y.jsx_readNewLine = function(P) {
          var T, q = this.input.charCodeAt(this.pos);
          return ++this.pos, q === 13 && this.input.charCodeAt(this.pos) === 10 ? (++this.pos, T = P ? `
` : `\r
`) : T = String.fromCharCode(q), this.options.locations && (++this.curLine, this.lineStart = this.pos), T;
        }, y.jsx_readString = function(P) {
          for (var T = "", q = ++this.pos; ; ) {
            this.pos >= this.input.length && this.raise(this.start, "Unterminated string constant");
            var H = this.input.charCodeAt(this.pos);
            if (H === P) break;
            H === 38 ? (T += this.input.slice(q, this.pos), T += this.jsx_readEntity(), q = this.pos) : r(H) ? (T += this.input.slice(q, this.pos), T += this.jsx_readNewLine(false), q = this.pos) : ++this.pos;
          }
          return T += this.input.slice(q, this.pos++), this.finishToken(V.string, T);
        }, y.jsx_readEntity = function() {
          var P, T = "", q = 0, H = this.input[this.pos];
          H !== "&" && this.raise(this.pos, "Entity must start with an ampersand");
          for (var J = ++this.pos; this.pos < this.input.length && q++ < 10; ) {
            if ((H = this.input[this.pos++]) === ";") {
              T[0] === "#" ? T[1] === "x" ? (T = T.substr(2), $1.test(T) && (P = String.fromCharCode(parseInt(T, 16)))) : (T = T.substr(1), H1.test(T) && (P = String.fromCharCode(parseInt(T, 10)))) : P = V1[T];
              break;
            }
            T += H;
          }
          return P || (this.pos = J, "&");
        }, y.jsx_readWord = function() {
          var P, T = this.pos;
          do
            P = this.input.charCodeAt(++this.pos);
          while (a(P) || P === 45);
          return this.finishToken(c.jsxName, this.input.slice(T, this.pos));
        }, y.jsx_parseIdentifier = function() {
          var P = this.startNode();
          return this.type === c.jsxName ? P.name = this.value : this.type.keyword ? P.name = this.type.keyword : this.unexpected(), this.next(), this.finishNode(P, "JSXIdentifier");
        }, y.jsx_parseNamespacedName = function() {
          var P = this.start, T = this.startLoc, q = this.jsx_parseIdentifier();
          if (!n.allowNamespaces || !this.eat(V.colon)) return q;
          var H = this.startNodeAt(P, T);
          return H.namespace = q, H.name = this.jsx_parseIdentifier(), this.finishNode(H, "JSXNamespacedName");
        }, y.jsx_parseElementName = function() {
          if (this.type === c.jsxTagEnd) return "";
          var P = this.start, T = this.startLoc, q = this.jsx_parseNamespacedName();
          for (this.type !== V.dot || q.type !== "JSXNamespacedName" || n.allowNamespacedObjects || this.unexpected(); this.eat(V.dot); ) {
            var H = this.startNodeAt(P, T);
            H.object = q, H.property = this.jsx_parseIdentifier(), q = this.finishNode(H, "JSXMemberExpression");
          }
          return q;
        }, y.jsx_parseAttributeValue = function() {
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
        }, y.jsx_parseEmptyExpression = function() {
          var P = this.startNodeAt(this.lastTokEnd, this.lastTokEndLoc);
          return this.finishNodeAt(P, "JSXEmptyExpression", this.start, this.startLoc);
        }, y.jsx_parseExpressionContainer = function() {
          var P = this.startNode();
          return this.next(), P.expression = this.type === V.braceR ? this.jsx_parseEmptyExpression() : this.parseExpression(), this.expect(V.braceR), this.finishNode(P, "JSXExpressionContainer");
        }, y.jsx_parseAttribute = function() {
          var P = this.startNode();
          return this.eat(V.braceL) ? (this.expect(V.ellipsis), P.argument = this.parseMaybeAssign(), this.expect(V.braceR), this.finishNode(P, "JSXSpreadAttribute")) : (P.name = this.jsx_parseNamespacedName(), P.value = this.eat(V.eq) ? this.jsx_parseAttributeValue() : null, this.finishNode(P, "JSXAttribute"));
        }, y.jsx_parseOpeningElementAt = function(P, T) {
          var q = this.startNodeAt(P, T);
          q.attributes = [];
          var H = this.jsx_parseElementName();
          for (H && (q.name = H); this.type !== V.slash && this.type !== c.jsxTagEnd; ) q.attributes.push(this.jsx_parseAttribute());
          return q.selfClosing = this.eat(V.slash), this.expect(c.jsxTagEnd), this.finishNode(q, H ? "JSXOpeningElement" : "JSXOpeningFragment");
        }, y.jsx_parseClosingElementAt = function(P, T) {
          var q = this.startNodeAt(P, T), H = this.jsx_parseElementName();
          return H && (q.name = H), this.expect(c.jsxTagEnd), this.finishNode(q, H ? "JSXClosingElement" : "JSXClosingFragment");
        }, y.jsx_parseElementAt = function(P, T) {
          var q = this.startNodeAt(P, T), H = [], J = this.jsx_parseOpeningElementAt(P, T), re = null;
          if (!J.selfClosing) {
            e: for (; ; ) switch (this.type) {
              case c.jsxTagStart:
                if (P = this.start, T = this.startLoc, this.next(), this.eat(V.slash)) {
                  re = this.jsx_parseClosingElementAt(P, T);
                  break e;
                }
                H.push(this.jsx_parseElementAt(P, T));
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
          return q["opening" + Q] = J, q["closing" + Q] = re, q.children = H, this.type === V.relational && this.value === "<" && this.raise(this.start, "Adjacent JSX elements must be wrapped in an enclosing tag"), this.finishNode(q, "JSX" + Q);
        }, y.jsx_parseText = function() {
          var P = this.parseLiteral(this.value);
          return P.type = "JSXText", P;
        }, y.jsx_parseElement = function() {
          var P = this.start, T = this.startLoc;
          return this.next(), this.jsx_parseElementAt(P, T);
        }, f;
      }(I);
    }(w, _, m, e == null ? void 0 : e.jsx), m = function(A, N, I) {
      var M = N.tokTypes, V = I.tokTypes;
      return function(c) {
        function r() {
          return c.apply(this, arguments) || this;
        }
        Oo(r, c);
        var a = r.prototype;
        return a.parseMaybeImportAttributes = function(n) {
          if (this.type === V._with || this.type === M.assert) {
            this.next();
            var u = this.parseImportAttributes();
            u && (n.attributes = u);
          }
        }, a.parseImportAttributes = function() {
          this.expect(V.braceL);
          var n = this.parseWithEntries();
          return this.expect(V.braceR), n;
        }, a.parseWithEntries = function() {
          var n = [], u = /* @__PURE__ */ new Set();
          do {
            if (this.type === V.braceR) break;
            var f, y = this.startNode();
            f = this.type === V.string ? this.parseLiteral(this.value) : this.parseIdent(true), this.next(), y.key = f, u.has(y.key.name) && this.raise(this.pos, "Duplicated key in attributes"), u.add(y.key.name), this.type !== V.string && this.raise(this.pos, "Only string is supported as an attribute value"), y.value = this.parseLiteral(this.value), n.push(this.finishNode(y, "ImportAttribute"));
          } while (this.eat(V.comma));
          return n;
        }, r;
      }(A);
    }(m, _, w);
    var Z = function(A) {
      function N(r, a, n) {
        var u;
        return (u = A.call(this, r, a, n) || this).preValue = null, u.preToken = null, u.isLookahead = false, u.isAmbientContext = false, u.inAbstractClass = false, u.inType = false, u.inDisallowConditionalTypesContext = false, u.maybeInArrowParameters = false, u.shouldParseArrowReturnType = void 0, u.shouldParseAsyncArrowReturnType = void 0, u.decoratorStack = [[]], u.importsStack = [[]], u.importOrExportOuterKind = void 0, u.tsParseConstModifier = u.tsParseModifiers.bind(function(f) {
          if (f === void 0) throw new ReferenceError("this hasn't been initialised - super() hasn't been called");
          return f;
        }(u), { allowedModifiers: ["const"], disallowedModifiers: ["in", "out"], errorTemplate: le.InvalidModifierOnTypeParameterPositions }), u;
      }
      Oo(N, A);
      var I, M, V, c = N.prototype;
      return c.getTokenFromCodeInType = function(r) {
        return r === 62 || r === 60 ? this.finishOp(o.relational, 1) : A.prototype.getTokenFromCode.call(this, r);
      }, c.readToken = function(r) {
        if (!this.inType) {
          var a = this.curContext();
          if (a === v.tc_expr) return this.jsx_readToken();
          if (a === v.tc_oTag || a === v.tc_cTag) {
            if (L(r)) return this.jsx_readWord();
            if (r == 62) return ++this.pos, this.finishToken(R.jsxTagEnd);
            if ((r === 34 || r === 39) && a == v.tc_oTag) return this.jsx_readString(r);
          }
          if (r === 60 && this.exprAllowed && this.input.charCodeAt(this.pos + 1) !== 33) return ++this.pos, this.finishToken(R.jsxTagStart);
        }
        return A.prototype.readToken.call(this, r);
      }, c.getTokenFromCode = function(r) {
        return this.inType ? this.getTokenFromCodeInType(r) : r === 64 ? (++this.pos, this.finishToken(R.at)) : A.prototype.getTokenFromCode.call(this, r);
      }, c.isAbstractClass = function() {
        return this.ts_isContextual(R.abstract) && this.lookahead().type === o._class;
      }, c.finishNode = function(r, a) {
        return r.type !== "" && r.end !== 0 ? r : A.prototype.finishNode.call(this, r, a);
      }, c.tryParse = function(r, a) {
        a === void 0 && (a = this.cloneCurLookaheadState());
        var n = { node: null };
        try {
          return { node: r(function(f) {
            throw f === void 0 && (f = null), n.node = f, n;
          }), error: null, thrown: false, aborted: false, failState: null };
        } catch (f) {
          var u = this.getCurLookaheadState();
          if (this.setLookaheadState(a), f instanceof SyntaxError) return { node: null, error: f, thrown: true, aborted: false, failState: u };
          if (f === n) return { node: n.node, error: null, thrown: false, aborted: true, failState: u };
          throw f;
        }
      }, c.setOptionalParametersError = function(r, a) {
        var n;
        r.optionalParametersLoc = (n = a == null ? void 0 : a.loc) != null ? n : this.startLoc;
      }, c.reScan_lt_gt = function() {
        this.type === o.relational && (this.pos -= 1, this.readToken_lt_gt(this.fullCharCodeAtPos()));
      }, c.reScan_lt = function() {
        var r = this.type;
        return r === o.bitShift ? (this.pos -= 2, this.finishOp(o.relational, 1), o.relational) : r;
      }, c.resetEndLocation = function(r, a) {
        a === void 0 && (a = this.lastTokEndLoc), r.end = a.column, r.loc.end = a, this.options.ranges && (r.range[1] = a.column);
      }, c.startNodeAtNode = function(r) {
        return A.prototype.startNodeAt.call(this, r.start, r.loc.start);
      }, c.nextTokenStart = function() {
        return this.nextTokenStartSince(this.pos);
      }, c.tsHasSomeModifiers = function(r, a) {
        return a.some(function(n) {
          return Zl(n) ? r.accessibility === n : !!r[n];
        });
      }, c.tsIsStartOfStaticBlocks = function() {
        return this.isContextual("static") && this.lookaheadCharCode() === 123;
      }, c.tsCheckForInvalidTypeCasts = function(r) {
        var a = this;
        r.forEach(function(n) {
          (n == null ? void 0 : n.type) === "TSTypeCastExpression" && a.raise(n.typeAnnotation.start, le.UnexpectedTypeAnnotation);
        });
      }, c.atPossibleAsyncArrow = function(r) {
        return r.type === "Identifier" && r.name === "async" && this.lastTokEndLoc.column === r.end && !this.canInsertSemicolon() && r.end - r.start == 5 && r.start === this.potentialArrowAt;
      }, c.tsIsIdentifier = function() {
        return x(this.type);
      }, c.tsTryParseTypeOrTypePredicateAnnotation = function() {
        return this.match(o.colon) ? this.tsParseTypeOrTypePredicateAnnotation(o.colon) : void 0;
      }, c.tsTryParseGenericAsyncArrowFunction = function(r, a, n) {
        var u = this;
        if (this.tsMatchLeftRelational()) {
          var f = this.maybeInArrowParameters;
          this.maybeInArrowParameters = true;
          var y = this.tsTryParseAndCatch(function() {
            var P = u.startNodeAt(r, a);
            return P.typeParameters = u.tsParseTypeParameters(), A.prototype.parseFunctionParams.call(u, P), P.returnType = u.tsTryParseTypeOrTypePredicateAnnotation(), u.expect(o.arrow), P;
          });
          if (this.maybeInArrowParameters = f, y) return A.prototype.parseArrowExpression.call(this, y, null, true, n);
        }
      }, c.tsParseTypeArgumentsInExpression = function() {
        if (this.reScan_lt() === o.relational) return this.tsParseTypeArguments();
      }, c.tsInNoContext = function(r) {
        var a = this.context;
        this.context = [a[0]];
        try {
          return r();
        } finally {
          this.context = a;
        }
      }, c.tsTryParseTypeAnnotation = function() {
        return this.match(o.colon) ? this.tsParseTypeAnnotation() : void 0;
      }, c.isUnparsedContextual = function(r, a) {
        var n = r + a.length;
        if (this.input.slice(r, n) === a) {
          var u = this.input.charCodeAt(n);
          return !(F(u) || (64512 & u) == 55296);
        }
        return false;
      }, c.isAbstractConstructorSignature = function() {
        return this.ts_isContextual(R.abstract) && this.lookahead().type === o._new;
      }, c.nextTokenStartSince = function(r) {
        return dl.lastIndex = r, dl.test(this.input) ? dl.lastIndex : r;
      }, c.lookaheadCharCode = function() {
        return this.input.charCodeAt(this.nextTokenStart());
      }, c.compareLookaheadState = function(r, a) {
        for (var n = 0, u = Object.keys(r); n < u.length; n++) {
          var f = u[n];
          if (r[f] !== a[f]) return false;
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
        var a = this.getCurLookaheadState(), n = r();
        return this.setLookaheadState(a), n;
      }, c.lookahead = function(r) {
        var a = this.getCurLookaheadState();
        if (this.createLookaheadState(), this.isLookahead = true, r !== void 0) for (var n = 0; n < r; n++) this.nextToken();
        else this.nextToken();
        this.isLookahead = false;
        var u = this.getCurLookaheadState();
        return this.setLookaheadState(a), u;
      }, c.readWord = function() {
        var r = this.readWord1(), a = o.name;
        return this.keywords.test(r) ? a = b[r] : new RegExp(h).test(r) && (a = R[r]), this.finishToken(a, r);
      }, c.skipBlockComment = function() {
        var r;
        this.isLookahead || (r = this.options.onComment && this.curPosition());
        var a = this.pos, n = this.input.indexOf("*/", this.pos += 2);
        if (n === -1 && this.raise(this.pos - 2, "Unterminated comment"), this.pos = n + 2, this.options.locations) for (var u, f = a; (u = z(this.input, f, this.pos)) > -1; ) ++this.curLine, f = this.lineStart = u;
        this.isLookahead || this.options.onComment && this.options.onComment(true, this.input.slice(a + 2, n), a, this.pos, r, this.curPosition());
      }, c.skipLineComment = function(r) {
        var a, n = this.pos;
        this.isLookahead || (a = this.options.onComment && this.curPosition());
        for (var u = this.input.charCodeAt(this.pos += r); this.pos < this.input.length && !B(u); ) u = this.input.charCodeAt(++this.pos);
        this.isLookahead || this.options.onComment && this.options.onComment(false, this.input.slice(n + r, this.pos), n, this.pos, a, this.curPosition());
      }, c.finishToken = function(r, a) {
        this.preValue = this.value, this.preToken = this.type, this.end = this.pos, this.options.locations && (this.endLoc = this.curPosition());
        var n = this.type;
        this.type = r, this.value = a, this.isLookahead || this.updateContext(n);
      }, c.resetStartLocation = function(r, a, n) {
        r.start = a, r.loc.start = n, this.options.ranges && (r.range[0] = a);
      }, c.isLineTerminator = function() {
        return this.eat(o.semi) || A.prototype.canInsertSemicolon.call(this);
      }, c.hasFollowingLineBreak = function() {
        return Jl.lastIndex = this.end, Jl.test(this.input);
      }, c.addExtra = function(r, a, n, u) {
        if (u === void 0 && (u = true), r) {
          var f = r.extra = r.extra || {};
          u ? f[a] = n : Object.defineProperty(f, a, { enumerable: u, value: n });
        }
      }, c.isLiteralPropertyName = function() {
        return g(this.type);
      }, c.hasPrecedingLineBreak = function() {
        return O.test(this.input.slice(this.lastTokEndLoc.index, this.start));
      }, c.createIdentifier = function(r, a) {
        return r.name = a, this.finishNode(r, "Identifier");
      }, c.resetStartLocationFromNode = function(r, a) {
        this.resetStartLocation(r, a.start, a.loc.start);
      }, c.isThisParam = function(r) {
        return r.type === "Identifier" && r.name === "this";
      }, c.isLookaheadContextual = function(r) {
        var a = this.nextTokenStart();
        return this.isUnparsedContextual(a, r);
      }, c.ts_type_isContextual = function(r, a) {
        return r === a && !this.containsEsc;
      }, c.ts_isContextual = function(r) {
        return this.type === r && !this.containsEsc;
      }, c.ts_isContextualWithState = function(r, a) {
        return r.type === a && !r.containsEsc;
      }, c.isContextualWithState = function(r, a) {
        return a.type === o.name && a.value === r && !a.containsEsc;
      }, c.tsIsStartOfMappedType = function() {
        return this.next(), this.eat(o.plusMin) ? this.ts_isContextual(R.readonly) : (this.ts_isContextual(R.readonly) && this.next(), !!this.match(o.bracketL) && (this.next(), !!this.tsIsIdentifier() && (this.next(), this.match(o._in))));
      }, c.tsInDisallowConditionalTypesContext = function(r) {
        var a = this.inDisallowConditionalTypesContext;
        this.inDisallowConditionalTypesContext = true;
        try {
          return r();
        } finally {
          this.inDisallowConditionalTypesContext = a;
        }
      }, c.tsTryParseType = function() {
        return this.tsEatThenParseType(o.colon);
      }, c.match = function(r) {
        return this.type === r;
      }, c.matchJsx = function(r) {
        return this.type === _.tokTypes[r];
      }, c.ts_eatWithState = function(r, a, n) {
        if (r === n.type) {
          for (var u = 0; u < a; u++) this.next();
          return true;
        }
        return false;
      }, c.ts_eatContextualWithState = function(r, a, n) {
        if (h.test(r)) {
          if (this.ts_isContextualWithState(n, R[r])) {
            for (var u = 0; u < a; u++) this.next();
            return true;
          }
          return false;
        }
        if (!this.isContextualWithState(r, n)) return false;
        for (var f = 0; f < a; f++) this.next();
        return true;
      }, c.canHaveLeadingDecorator = function() {
        return this.match(o._class);
      }, c.eatContextual = function(r) {
        return h.test(r) ? !!this.ts_isContextual(R[r]) && (this.next(), true) : A.prototype.eatContextual.call(this, r);
      }, c.tsIsExternalModuleReference = function() {
        return this.isContextual("require") && this.lookaheadCharCode() === 40;
      }, c.tsParseExternalModuleReference = function() {
        var r = this.startNode();
        return this.expectContextual("require"), this.expect(o.parenL), this.match(o.string) || this.unexpected(), r.expression = this.parseExprAtom(), this.expect(o.parenR), this.finishNode(r, "TSExternalModuleReference");
      }, c.tsParseEntityName = function(r) {
        r === void 0 && (r = true);
        for (var a = this.parseIdent(r); this.eat(o.dot); ) {
          var n = this.startNodeAtNode(a);
          n.left = a, n.right = this.parseIdent(r), a = this.finishNode(n, "TSQualifiedName");
        }
        return a;
      }, c.tsParseEnumMember = function() {
        var r = this.startNode();
        return r.id = this.match(o.string) ? this.parseLiteral(this.value) : this.parseIdent(true), this.eat(o.eq) && (r.initializer = this.parseMaybeAssign()), this.finishNode(r, "TSEnumMember");
      }, c.tsParseEnumDeclaration = function(r, a) {
        return a === void 0 && (a = {}), a.const && (r.const = true), a.declare && (r.declare = true), this.expectContextual("enum"), r.id = this.parseIdent(), this.checkLValSimple(r.id), this.expect(o.braceL), r.members = this.tsParseDelimitedList("EnumMembers", this.tsParseEnumMember.bind(this)), this.expect(o.braceR), this.finishNode(r, "TSEnumDeclaration");
      }, c.tsParseModuleBlock = function() {
        var r = this.startNode();
        for (A.prototype.enterScope.call(this, 512), this.expect(o.braceL), r.body = []; this.type !== o.braceR; ) {
          var a = this.parseStatement(null, true);
          r.body.push(a);
        }
        return this.next(), A.prototype.exitScope.call(this), this.finishNode(r, "TSModuleBlock");
      }, c.tsParseAmbientExternalModuleDeclaration = function(r) {
        return this.ts_isContextual(R.global) ? (r.global = true, r.id = this.parseIdent()) : this.match(o.string) ? r.id = this.parseLiteral(this.value) : this.unexpected(), this.match(o.braceL) ? (A.prototype.enterScope.call(this, ir), r.body = this.tsParseModuleBlock(), A.prototype.exitScope.call(this)) : A.prototype.semicolon.call(this), this.finishNode(r, "TSModuleDeclaration");
      }, c.tsTryParseDeclare = function(r) {
        var a = this;
        if (!this.isLineTerminator()) {
          var n, u = this.type;
          return this.isContextual("let") && (u = o._var, n = "let"), this.tsInAmbientContext(function() {
            if (u === o._function) return r.declare = true, a.parseFunctionStatement(r, false, true);
            if (u === o._class) return r.declare = true, a.parseClass(r, true);
            if (u === R.enum) return a.tsParseEnumDeclaration(r, { declare: true });
            if (u === R.global) return a.tsParseAmbientExternalModuleDeclaration(r);
            if (u === o._const || u === o._var) return a.match(o._const) && a.isLookaheadContextual("enum") ? (a.expect(o._const), a.tsParseEnumDeclaration(r, { const: true, declare: true })) : (r.declare = true, a.parseVarStatement(r, n || a.value, true));
            if (u === R.interface) {
              var f = a.tsParseInterfaceDeclaration(r, { declare: true });
              if (f) return f;
            }
            return x(u) ? a.tsParseDeclaration(r, a.value, true) : void 0;
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
      }, c.tsParseDelimitedListWorker = function(r, a, n, u) {
        for (var f = [], y = -1; !this.tsIsListTerminator(r); ) {
          y = -1;
          var P = a();
          if (P == null) return;
          if (f.push(P), !this.eat(o.comma)) {
            if (this.tsIsListTerminator(r)) break;
            return void (n && this.expect(o.comma));
          }
          y = this.lastTokStart;
        }
        return u && (u.value = y), f;
      }, c.tsParseDelimitedList = function(r, a, n) {
        return function(u) {
          if (u == null) throw new Error("Unexpected " + u + " value.");
          return u;
        }(this.tsParseDelimitedListWorker(r, a, true, n));
      }, c.tsParseBracketedList = function(r, a, n, u, f) {
        u || this.expect(n ? o.bracketL : o.relational);
        var y = this.tsParseDelimitedList(r, a, f);
        return this.expect(n ? o.bracketR : o.relational), y;
      }, c.tsParseTypeParameterName = function() {
        return this.parseIdent().name;
      }, c.tsEatThenParseType = function(r) {
        return this.match(r) ? this.tsNextThenParseType() : void 0;
      }, c.tsExpectThenParseType = function(r) {
        var a = this;
        return this.tsDoThenParseType(function() {
          return a.expect(r);
        });
      }, c.tsNextThenParseType = function() {
        var r = this;
        return this.tsDoThenParseType(function() {
          return r.next();
        });
      }, c.tsDoThenParseType = function(r) {
        var a = this;
        return this.tsInType(function() {
          return r(), a.tsParseType();
        });
      }, c.tsSkipParameterStart = function() {
        if (x(this.type) || this.match(o._this)) return this.next(), true;
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
        var a = this.inDisallowConditionalTypesContext;
        this.inDisallowConditionalTypesContext = false;
        try {
          return r();
        } finally {
          this.inDisallowConditionalTypesContext = a;
        }
      }, c.tsParseBindingListForSignature = function() {
        var r = this;
        return A.prototype.parseBindingList.call(this, o.parenR, true, true).map(function(a) {
          return a.type !== "Identifier" && a.type !== "RestElement" && a.type !== "ObjectPattern" && a.type !== "ArrayPattern" && r.raise(a.start, le.UnsupportedSignatureParameterKind(a.type)), a;
        });
      }, c.tsParseTypePredicateAsserts = function() {
        if (this.type !== R.asserts) return false;
        var r = this.containsEsc;
        return this.next(), !(!x(this.type) && !this.match(o._this) || (r && this.raise(this.lastTokStart, "Escape sequence in keyword asserts"), 0));
      }, c.tsParseThisTypeNode = function() {
        var r = this.startNode();
        return this.next(), this.finishNode(r, "TSThisType");
      }, c.tsParseTypeAnnotation = function(r, a) {
        var n = this;
        return r === void 0 && (r = true), a === void 0 && (a = this.startNode()), this.tsInType(function() {
          r && n.expect(o.colon), a.typeAnnotation = n.tsParseType();
        }), this.finishNode(a, "TSTypeAnnotation");
      }, c.tsParseThisTypePredicate = function(r) {
        this.next();
        var a = this.startNodeAtNode(r);
        return a.parameterName = r, a.typeAnnotation = this.tsParseTypeAnnotation(false), a.asserts = false, this.finishNode(a, "TSTypePredicate");
      }, c.tsParseThisTypeOrThisTypePredicate = function() {
        var r = this.tsParseThisTypeNode();
        return this.isContextual("is") && !this.hasPrecedingLineBreak() ? this.tsParseThisTypePredicate(r) : r;
      }, c.tsParseTypePredicatePrefix = function() {
        var r = this.parseIdent();
        if (this.isContextual("is") && !this.hasPrecedingLineBreak()) return this.next(), r;
      }, c.tsParseTypeOrTypePredicateAnnotation = function(r) {
        var a = this;
        return this.tsInType(function() {
          var n = a.startNode();
          a.expect(r);
          var u = a.startNode(), f = !!a.tsTryParse(a.tsParseTypePredicateAsserts.bind(a));
          if (f && a.match(o._this)) {
            var y = a.tsParseThisTypeOrThisTypePredicate();
            return y.type === "TSThisType" ? (u.parameterName = y, u.asserts = true, u.typeAnnotation = null, y = a.finishNode(u, "TSTypePredicate")) : (a.resetStartLocationFromNode(y, u), y.asserts = true), n.typeAnnotation = y, a.finishNode(n, "TSTypeAnnotation");
          }
          var P = a.tsIsIdentifier() && a.tsTryParse(a.tsParseTypePredicatePrefix.bind(a));
          if (!P) return f ? (u.parameterName = a.parseIdent(), u.asserts = f, u.typeAnnotation = null, n.typeAnnotation = a.finishNode(u, "TSTypePredicate"), a.finishNode(n, "TSTypeAnnotation")) : a.tsParseTypeAnnotation(false, n);
          var T = a.tsParseTypeAnnotation(false);
          return u.parameterName = P, u.typeAnnotation = T, u.asserts = f, n.typeAnnotation = a.finishNode(u, "TSTypePredicate"), a.finishNode(n, "TSTypeAnnotation");
        });
      }, c.tsFillSignature = function(r, a) {
        var n = r === o.arrow;
        a.typeParameters = this.tsTryParseTypeParameters(), this.expect(o.parenL), a.parameters = this.tsParseBindingListForSignature(), (n || this.match(r)) && (a.typeAnnotation = this.tsParseTypeOrTypePredicateAnnotation(r));
      }, c.tsTryNextParseConstantContext = function() {
        if (this.lookahead().type !== o._const) return null;
        this.next();
        var r = this.tsParseTypeReference();
        return r.typeParameters && this.raise(r.typeName.start, le.CannotFindName({ name: "const" })), r;
      }, c.tsParseFunctionOrConstructorType = function(r, a) {
        var n = this, u = this.startNode();
        return r === "TSConstructorType" && (u.abstract = !!a, a && this.next(), this.next()), this.tsInAllowConditionalTypesContext(function() {
          return n.tsFillSignature(o.arrow, u);
        }), this.finishNode(u, r);
      }, c.tsParseUnionOrIntersectionType = function(r, a, n) {
        var u = this.startNode(), f = this.eat(n), y = [];
        do
          y.push(a());
        while (this.eat(n));
        return y.length !== 1 || f ? (u.types = y, this.finishNode(u, r)) : y[0];
      }, c.tsCheckTypeAnnotationForReadOnly = function(r) {
        switch (r.typeAnnotation.type) {
          case "TSTupleType":
          case "TSArrayType":
            return;
          default:
            this.raise(r.start, le.UnexpectedReadonly);
        }
      }, c.tsParseTypeOperator = function() {
        var r = this.startNode(), a = this.value;
        return this.next(), r.operator = a, r.typeAnnotation = this.tsParseTypeOperatorOrHigher(), a === "readonly" && this.tsCheckTypeAnnotationForReadOnly(r), this.finishNode(r, "TSTypeOperator");
      }, c.tsParseConstraintForInferType = function() {
        var r = this;
        if (this.eat(o._extends)) {
          var a = this.tsInDisallowConditionalTypesContext(function() {
            return r.tsParseType();
          });
          if (this.inDisallowConditionalTypesContext || !this.match(o.question)) return a;
        }
      }, c.tsParseInferType = function() {
        var r = this, a = this.startNode();
        this.expectContextual("infer");
        var n = this.startNode();
        return n.name = this.tsParseTypeParameterName(), n.constraint = this.tsTryParse(function() {
          return r.tsParseConstraintForInferType();
        }), a.typeParameter = this.finishNode(n, "TSTypeParameter"), this.finishNode(a, "TSInferType");
      }, c.tsParseLiteralTypeNode = function() {
        var r = this, a = this.startNode();
        return a.literal = function() {
          switch (r.type) {
            case o.num:
            case o.string:
            case o._true:
            case o._false:
              return r.parseExprAtom();
            default:
              r.unexpected();
          }
        }(), this.finishNode(a, "TSLiteralType");
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
        var r = this.startLoc, a = this.start, n = this.eat(o.ellipsis), u = this.tsParseType(), f = this.eat(o.question);
        if (this.eat(o.colon)) {
          var y = this.startNodeAtNode(u);
          y.optional = f, u.type !== "TSTypeReference" || u.typeParameters || u.typeName.type !== "Identifier" ? (this.raise(u.start, le.InvalidTupleMemberLabel), y.label = u) : y.label = u.typeName, y.elementType = this.tsParseType(), u = this.finishNode(y, "TSNamedTupleMember");
        } else if (f) {
          var P = this.startNodeAtNode(u);
          P.typeAnnotation = u, u = this.finishNode(P, "TSOptionalType");
        }
        if (n) {
          var T = this.startNodeAt(a, r);
          T.typeAnnotation = u, u = this.finishNode(T, "TSRestType");
        }
        return u;
      }, c.tsParseTupleType = function() {
        var r = this, a = this.startNode();
        a.elementTypes = this.tsParseBracketedList("TupleElementTypes", this.tsParseTupleElementType.bind(this), true, false);
        var n = false, u = null;
        return a.elementTypes.forEach(function(f) {
          var y = f.type;
          !n || y === "TSRestType" || y === "TSOptionalType" || y === "TSNamedTupleMember" && f.optional || r.raise(f.start, le.OptionalTypeBeforeRequired), n || (n = y === "TSNamedTupleMember" && f.optional || y === "TSOptionalType");
          var P = y;
          y === "TSRestType" && (P = (f = f.typeAnnotation).type);
          var T = P === "TSNamedTupleMember";
          u != null || (u = T), u !== T && r.raise(f.start, le.MixedLabeledAndUnlabeledElements);
        }), this.finishNode(a, "TSTupleType");
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
            var a = this.type;
            if (x(a) || a === o._void || a === o._null) {
              var n = a === o._void ? "TSVoidKeyword" : a === o._null ? "TSNullKeyword" : function(f) {
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
        for (var r = this.tsParseNonArrayType(); !this.hasPrecedingLineBreak() && this.eat(o.bracketL); ) if (this.match(o.bracketR)) {
          var a = this.startNodeAtNode(r);
          a.elementType = r, this.expect(o.bracketR), r = this.finishNode(a, "TSArrayType");
        } else {
          var n = this.startNodeAtNode(r);
          n.objectType = r, n.indexType = this.tsParseType(), this.expect(o.bracketR), r = this.finishNode(n, "TSIndexedAccessType");
        }
        return r;
      }, c.tsParseTypeOperatorOrHigher = function() {
        var r = this;
        return D(this.type) && !this.containsEsc ? this.tsParseTypeOperator() : this.isContextual("infer") ? this.tsParseInferType() : this.tsInAllowConditionalTypesContext(function() {
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
        var a = this.tsParseNonConditionalType();
        if (this.inDisallowConditionalTypesContext || this.hasPrecedingLineBreak() || !this.eat(o._extends)) return a;
        var n = this.startNodeAtNode(a);
        return n.checkType = a, n.extendsType = this.tsInDisallowConditionalTypesContext(function() {
          return r.tsParseNonConditionalType();
        }), this.expect(o.question), n.trueType = this.tsInAllowConditionalTypesContext(function() {
          return r.tsParseType();
        }), this.expect(o.colon), n.falseType = this.tsInAllowConditionalTypesContext(function() {
          return r.tsParseType();
        }), this.finishNode(n, "TSConditionalType");
      }, c.tsIsUnambiguouslyIndexSignature = function() {
        return this.next(), !!x(this.type) && (this.next(), this.match(o.colon));
      }, c.tsInType = function(r) {
        var a = this.inType;
        this.inType = true;
        try {
          return r();
        } finally {
          this.inType = a;
        }
      }, c.tsTryParseIndexSignature = function(r) {
        if (this.match(o.bracketL) && this.tsLookAhead(this.tsIsUnambiguouslyIndexSignature.bind(this))) {
          this.expect(o.bracketL);
          var a = this.parseIdent();
          a.typeAnnotation = this.tsParseTypeAnnotation(), this.resetEndLocation(a), this.expect(o.bracketR), r.parameters = [a];
          var n = this.tsTryParseTypeAnnotation();
          return n && (r.typeAnnotation = n), this.tsParseTypeMemberSemicolon(), this.finishNode(r, "TSIndexSignature");
        }
      }, c.tsParseNoneModifiers = function(r) {
        this.tsParseModifiers({ modified: r, allowedModifiers: [], disallowedModifiers: ["in", "out"], errorTemplate: le.InvalidModifierOnTypeParameterPositions });
      }, c.tsParseTypeParameter = function(r) {
        r === void 0 && (r = this.tsParseNoneModifiers.bind(this));
        var a = this.startNode();
        return r(a), a.name = this.tsParseTypeParameterName(), a.constraint = this.tsEatThenParseType(o._extends), a.default = this.tsEatThenParseType(o.eq), this.finishNode(a, "TSTypeParameter");
      }, c.tsParseTypeParameters = function(r) {
        var a = this.startNode();
        this.tsMatchLeftRelational() || this.matchJsx("jsxTagStart") ? this.next() : this.unexpected();
        var n = { value: -1 };
        return a.params = this.tsParseBracketedList("TypeParametersOrArguments", this.tsParseTypeParameter.bind(this, r), false, true, n), a.params.length === 0 && this.raise(this.start, le.EmptyTypeParameters), n.value !== -1 && this.addExtra(a, "trailingComma", n.value), this.finishNode(a, "TSTypeParameterDeclaration");
      }, c.tsTryParseTypeParameters = function(r) {
        if (this.tsMatchLeftRelational()) return this.tsParseTypeParameters(r);
      }, c.tsTryParse = function(r) {
        var a = this.getCurLookaheadState(), n = r();
        return n !== void 0 && n !== false ? n : void this.setLookaheadState(a);
      }, c.tsTokenCanFollowModifier = function() {
        return (this.match(o.bracketL) || this.match(o.braceL) || this.match(o.star) || this.match(o.ellipsis) || this.match(o.privateId) || this.isLiteralPropertyName()) && !this.hasPrecedingLineBreak();
      }, c.tsNextTokenCanFollowModifier = function() {
        return this.next(true), this.tsTokenCanFollowModifier();
      }, c.tsParseModifier = function(r, a) {
        if (x(this.type) || this.type === o._in) {
          var n = this.value;
          if (r.indexOf(n) !== -1 && !this.containsEsc) {
            if (a && this.tsIsStartOfStaticBlocks()) return;
            if (this.tsTryParse(this.tsNextTokenCanFollowModifier.bind(this))) return n;
          }
        }
      }, c.tsParseModifiersByMap = function(r) {
        for (var a = r.modified, n = r.map, u = 0, f = Object.keys(n); u < f.length; u++) {
          var y = f[u];
          a[y] = n[y];
        }
      }, c.tsParseModifiers = function(r) {
        for (var a = this, n = r.modified, u = r.allowedModifiers, f = r.disallowedModifiers, y = r.stopOnStartOfClassStaticBlock, P = r.errorTemplate, T = P === void 0 ? le.InvalidModifierOnTypeMember : P, q = {}, H = function(pe, de, fe, he) {
          de === fe && n[he] && a.raise(pe.column, le.InvalidModifiersOrder({ orderedModifiers: [fe, he] }));
        }, J = function(pe, de, fe, he) {
          (n[fe] && de === he || n[he] && de === fe) && a.raise(pe.column, le.IncompatibleModifiers({ modifiers: [fe, he] }));
        }; ; ) {
          var re = this.startLoc, Q = this.tsParseModifier(u.concat(f ?? []), y);
          if (!Q) break;
          Zl(Q) ? n.accessibility ? this.raise(this.start, le.DuplicateAccessibilityModifier()) : (H(re, Q, Q, "override"), H(re, Q, Q, "static"), H(re, Q, Q, "readonly"), H(re, Q, Q, "accessor"), q.accessibility = Q, n.accessibility = Q) : G1(Q) ? n[Q] ? this.raise(this.start, le.DuplicateModifier({ modifier: Q })) : (H(re, Q, "in", "out"), q[Q] = Q, n[Q] = true) : U1(Q) ? n[Q] ? this.raise(this.start, le.DuplicateModifier({ modifier: Q })) : (J(re, Q, "accessor", "readonly"), J(re, Q, "accessor", "static"), J(re, Q, "accessor", "override"), q[Q] = Q, n[Q] = true) : Object.hasOwnProperty.call(n, Q) ? this.raise(this.start, le.DuplicateModifier({ modifier: Q })) : (H(re, Q, "static", "readonly"), H(re, Q, "static", "override"), H(re, Q, "override", "readonly"), H(re, Q, "abstract", "override"), J(re, Q, "declare", "override"), J(re, Q, "static", "abstract"), q[Q] = Q, n[Q] = true), f != null && f.includes(Q) && this.raise(this.start, T);
        }
        return q;
      }, c.tsParseInOutModifiers = function(r) {
        this.tsParseModifiers({ modified: r, allowedModifiers: ["in", "out"], disallowedModifiers: ["public", "private", "protected", "readonly", "declare", "abstract", "override"], errorTemplate: le.InvalidModifierOnTypeParameter });
      }, c.tsParseTypeArguments = function() {
        var r = this, a = this.startNode();
        return a.params = this.tsInType(function() {
          return r.tsInNoContext(function() {
            return r.expect(o.relational), r.tsParseDelimitedList("TypeParametersOrArguments", r.tsParseType.bind(r));
          });
        }), a.params.length === 0 && this.raise(this.start, le.EmptyTypeArguments), this.exprAllowed = false, this.expect(o.relational), this.finishNode(a, "TSTypeParameterInstantiation");
      }, c.tsParseHeritageClause = function(r) {
        var a = this, n = this.start, u = this.tsParseDelimitedList("HeritageClauseElement", function() {
          var f = a.startNode();
          return f.expression = a.tsParseEntityName(), a.tsMatchLeftRelational() && (f.typeParameters = a.tsParseTypeArguments()), a.finishNode(f, "TSExpressionWithTypeArguments");
        });
        return u.length || this.raise(n, le.EmptyHeritageClauseType({ token: r })), u;
      }, c.tsParseTypeMemberSemicolon = function() {
        this.eat(o.comma) || this.isLineTerminator() || this.expect(o.semi);
      }, c.tsTryParseAndCatch = function(r) {
        var a = this.tryParse(function(n) {
          return r() || n();
        });
        if (!a.aborted && a.node) return a.error && this.setLookaheadState(a.failState), a.node;
      }, c.tsParseSignatureMember = function(r, a) {
        return this.tsFillSignature(o.colon, a), this.tsParseTypeMemberSemicolon(), this.finishNode(a, r);
      }, c.tsParsePropertyOrMethodSignature = function(r, a) {
        this.eat(o.question) && (r.optional = true);
        var n = r;
        if (this.match(o.parenL) || this.tsMatchLeftRelational()) {
          a && this.raise(r.start, le.ReadonlyForMethodSignature);
          var u = n;
          u.kind && this.tsMatchLeftRelational() && this.raise(this.start, le.AccesorCannotHaveTypeParameters), this.tsFillSignature(o.colon, u), this.tsParseTypeMemberSemicolon();
          var f = "parameters", y = "typeAnnotation";
          if (u.kind === "get") u[f].length > 0 && (this.raise(this.start, "A 'get' accesor must not have any formal parameters."), this.isThisParam(u[f][0]) && this.raise(this.start, le.AccesorCannotDeclareThisParameter));
          else if (u.kind === "set") {
            if (u[f].length !== 1) this.raise(this.start, "A 'get' accesor must not have any formal parameters.");
            else {
              var P = u[f][0];
              this.isThisParam(P) && this.raise(this.start, le.AccesorCannotDeclareThisParameter), P.type === "Identifier" && P.optional && this.raise(this.start, le.SetAccesorCannotHaveOptionalParameter), P.type === "RestElement" && this.raise(this.start, le.SetAccesorCannotHaveRestParameter);
            }
            u[y] && this.raise(u[y].start, le.SetAccesorCannotHaveReturnType);
          } else u.kind = "method";
          return this.finishNode(u, "TSMethodSignature");
        }
        var T = n;
        a && (T.readonly = true);
        var q = this.tsTryParseTypeAnnotation();
        return q && (T.typeAnnotation = q), this.tsParseTypeMemberSemicolon(), this.finishNode(T, "TSPropertySignature");
      }, c.tsParseTypeMember = function() {
        var r = this.startNode();
        if (this.match(o.parenL) || this.tsMatchLeftRelational()) return this.tsParseSignatureMember("TSCallSignatureDeclaration", r);
        if (this.match(o._new)) {
          var a = this.startNode();
          return this.next(), this.match(o.parenL) || this.tsMatchLeftRelational() ? this.tsParseSignatureMember("TSConstructSignatureDeclaration", r) : (r.key = this.createIdentifier(a, "new"), this.tsParsePropertyOrMethodSignature(r, false));
        }
        return this.tsParseModifiers({ modified: r, allowedModifiers: ["readonly"], disallowedModifiers: ["declare", "abstract", "private", "protected", "public", "static", "override"] }), this.tsTryParseIndexSignature(r) || (this.parsePropertyName(r), r.computed || r.key.type !== "Identifier" || r.key.name !== "get" && r.key.name !== "set" || !this.tsTokenCanFollowModifier() || (r.kind = r.key.name, this.parsePropertyName(r)), this.tsParsePropertyOrMethodSignature(r, !!r.readonly));
      }, c.tsParseList = function(r, a) {
        for (var n = []; !this.tsIsListTerminator(r); ) n.push(a());
        return n;
      }, c.tsParseObjectTypeMembers = function() {
        this.expect(o.braceL);
        var r = this.tsParseList("TypeMembers", this.tsParseTypeMember.bind(this));
        return this.expect(o.braceR), r;
      }, c.tsParseInterfaceDeclaration = function(r, a) {
        if (a === void 0 && (a = {}), this.hasFollowingLineBreak()) return null;
        this.expectContextual("interface"), a.declare && (r.declare = true), x(this.type) ? (r.id = this.parseIdent(), this.checkLValSimple(r.id, 7)) : (r.id = null, this.raise(this.start, le.MissingInterfaceName)), r.typeParameters = this.tsTryParseTypeParameters(this.tsParseInOutModifiers.bind(this)), this.eat(o._extends) && (r.extends = this.tsParseHeritageClause("extends"));
        var n = this.startNode();
        return n.body = this.tsInType(this.tsParseObjectTypeMembers.bind(this)), r.body = this.finishNode(n, "TSInterfaceBody"), this.finishNode(r, "TSInterfaceDeclaration");
      }, c.tsParseAbstractDeclaration = function(r) {
        if (this.match(o._class)) return r.abstract = true, this.parseClass(r, true);
        if (this.ts_isContextual(R.interface)) {
          if (!this.hasFollowingLineBreak()) return r.abstract = true, this.tsParseInterfaceDeclaration(r);
        } else this.unexpected(r.start);
      }, c.tsIsDeclarationStart = function() {
        return C(this.type);
      }, c.tsParseExpressionStatement = function(r, a) {
        switch (a.name) {
          case "declare":
            var n = this.tsTryParseDeclare(r);
            if (n) return n.declare = true, n;
            break;
          case "global":
            if (this.match(o.braceL)) {
              A.prototype.enterScope.call(this, ir);
              var u = r;
              return u.global = true, u.id = a, u.body = this.tsParseModuleBlock(), A.prototype.exitScope.call(this), this.finishNode(u, "TSModuleDeclaration");
            }
            break;
          default:
            return this.tsParseDeclaration(r, a.name, false);
        }
      }, c.tsParseModuleReference = function() {
        return this.tsIsExternalModuleReference() ? this.tsParseExternalModuleReference() : this.tsParseEntityName(false);
      }, c.tsIsExportDefaultSpecifier = function() {
        var r = this.type, a = this.isAsyncFunction(), n = this.isLet();
        if (x(r)) {
          if (a && !this.containsEsc || n) return false;
          if ((r === R.type || r === R.interface) && !this.containsEsc) {
            var u = this.lookahead();
            if (x(u.type) && !this.isContextualWithState("from", u) || u.type === o.braceL) return false;
          }
        } else if (!this.match(o._default)) return false;
        var f = this.nextTokenStart(), y = this.isUnparsedContextual(f, "from");
        if (this.input.charCodeAt(f) === 44 || x(this.type) && y) return true;
        if (this.match(o._default) && y) {
          var P = this.input.charCodeAt(this.nextTokenStartSince(f + 4));
          return P === 34 || P === 39;
        }
        return false;
      }, c.tsInAmbientContext = function(r) {
        var a = this.isAmbientContext;
        this.isAmbientContext = true;
        try {
          return r();
        } finally {
          this.isAmbientContext = a;
        }
      }, c.tsCheckLineTerminator = function(r) {
        return r ? !this.hasFollowingLineBreak() && (this.next(), true) : !this.isLineTerminator();
      }, c.tsParseModuleOrNamespaceDeclaration = function(r, a) {
        if (a === void 0 && (a = false), r.id = this.parseIdent(), a || this.checkLValSimple(r.id, 8), this.eat(o.dot)) {
          var n = this.startNode();
          this.tsParseModuleOrNamespaceDeclaration(n, true), r.body = n;
        } else A.prototype.enterScope.call(this, ir), r.body = this.tsParseModuleBlock(), A.prototype.exitScope.call(this);
        return this.finishNode(r, "TSModuleDeclaration");
      }, c.checkLValSimple = function(r, a, n) {
        return a === void 0 && (a = 0), A.prototype.checkLValSimple.call(this, r, a, n);
      }, c.tsParseTypeAliasDeclaration = function(r) {
        var a = this;
        return r.id = this.parseIdent(), this.checkLValSimple(r.id, 6), r.typeAnnotation = this.tsInType(function() {
          if (r.typeParameters = a.tsTryParseTypeParameters(a.tsParseInOutModifiers.bind(a)), a.expect(o.eq), a.ts_isContextual(R.interface) && a.lookahead().type !== o.dot) {
            var n = a.startNode();
            return a.next(), a.finishNode(n, "TSIntrinsicKeyword");
          }
          return a.tsParseType();
        }), this.semicolon(), this.finishNode(r, "TSTypeAliasDeclaration");
      }, c.tsParseDeclaration = function(r, a, n) {
        switch (a) {
          case "abstract":
            if (this.tsCheckLineTerminator(n) && (this.match(o._class) || x(this.type))) return this.tsParseAbstractDeclaration(r);
            break;
          case "module":
            if (this.tsCheckLineTerminator(n)) {
              if (this.match(o.string)) return this.tsParseAmbientExternalModuleDeclaration(r);
              if (x(this.type)) return this.tsParseModuleOrNamespaceDeclaration(r);
            }
            break;
          case "namespace":
            if (this.tsCheckLineTerminator(n) && x(this.type)) return this.tsParseModuleOrNamespaceDeclaration(r);
            break;
          case "type":
            if (this.tsCheckLineTerminator(n) && x(this.type)) return this.tsParseTypeAliasDeclaration(r);
        }
      }, c.tsTryParseExportDeclaration = function() {
        return this.tsParseDeclaration(this.startNode(), this.value, true);
      }, c.tsParseImportEqualsDeclaration = function(r, a) {
        r.isExport = a || false, r.id = this.parseIdent(), this.checkLValSimple(r.id, 2), A.prototype.expect.call(this, o.eq);
        var n = this.tsParseModuleReference();
        return r.importKind === "type" && n.type !== "TSExternalModuleReference" && this.raise(n.start, le.ImportAliasHasImportType), r.moduleReference = n, A.prototype.semicolon.call(this), this.finishNode(r, "TSImportEqualsDeclaration");
      }, c.isExportDefaultSpecifier = function() {
        if (this.tsIsDeclarationStart()) return false;
        var r = this.type;
        if (x(r)) {
          if (this.isContextual("async") || this.isContextual("let")) return false;
          if ((r === R.type || r === R.interface) && !this.containsEsc) {
            var a = this.lookahead();
            if (x(a.type) && !this.isContextualWithState("from", a) || a.type === o.braceL) return false;
          }
        } else if (!this.match(o._default)) return false;
        var n = this.nextTokenStart(), u = this.isUnparsedContextual(n, "from");
        if (this.input.charCodeAt(n) === 44 || x(this.type) && u) return true;
        if (this.match(o._default) && u) {
          var f = this.input.charCodeAt(this.nextTokenStartSince(n + 4));
          return f === 34 || f === 39;
        }
        return false;
      }, c.parseTemplate = function(r) {
        var a = (r === void 0 ? {} : r).isTagged, n = a !== void 0 && a, u = this.startNode();
        this.next(), u.expressions = [];
        var f = this.parseTemplateElement({ isTagged: n });
        for (u.quasis = [f]; !f.tail; ) this.type === o.eof && this.raise(this.pos, "Unterminated template literal"), this.expect(o.dollarBraceL), u.expressions.push(this.inType ? this.tsParseType() : this.parseExpression()), this.expect(o.braceR), u.quasis.push(f = this.parseTemplateElement({ isTagged: n }));
        return this.next(), this.finishNode(u, "TemplateLiteral");
      }, c.parseFunction = function(r, a, n, u, f) {
        this.initFunction(r), (this.options.ecmaVersion >= 9 || this.options.ecmaVersion >= 6 && !u) && (this.type === o.star && 2 & a && this.unexpected(), r.generator = this.eat(o.star)), this.options.ecmaVersion >= 8 && (r.async = !!u), 1 & a && (r.id = 4 & a && this.type !== o.name ? null : this.parseIdent());
        var y = this.yieldPos, P = this.awaitPos, T = this.awaitIdentPos, q = this.maybeInArrowParameters;
        this.maybeInArrowParameters = false, this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0, this.enterScope(pl(r.async, r.generator)), 1 & a || (r.id = this.type === o.name ? this.parseIdent() : null), this.parseFunctionParams(r);
        var H = 1 & a;
        return this.parseFunctionBody(r, n, false, f, { isFunctionDeclaration: H }), this.yieldPos = y, this.awaitPos = P, this.awaitIdentPos = T, 1 & a && r.id && !(2 & a) && this.checkLValSimple(r.id, r.body ? this.strict || r.generator || r.async ? this.treatFunctionsAsVar ? 1 : 2 : 3 : 0), this.maybeInArrowParameters = q, this.finishNode(r, H ? "FunctionDeclaration" : "FunctionExpression");
      }, c.parseFunctionBody = function(r, a, n, u, f) {
        a === void 0 && (a = false), n === void 0 && (n = false), u === void 0 && (u = false), this.match(o.colon) && (r.returnType = this.tsParseTypeOrTypePredicateAnnotation(o.colon));
        var y = f != null && f.isFunctionDeclaration ? "TSDeclareFunction" : f != null && f.isClassMethod ? "TSDeclareMethod" : void 0;
        return y && !this.match(o.braceL) && this.isLineTerminator() ? this.finishNode(r, y) : y === "TSDeclareFunction" && this.isAmbientContext && (this.raise(r.start, le.DeclareFunctionHasImplementation), r.declare) ? (A.prototype.parseFunctionBody.call(this, r, a, n, false), this.finishNode(r, y)) : (A.prototype.parseFunctionBody.call(this, r, a, n, u), r);
      }, c.parseNew = function() {
        var r;
        this.containsEsc && this.raiseRecoverable(this.start, "Escape sequence in keyword new");
        var a = this.startNode(), n = this.parseIdent(true);
        if (this.options.ecmaVersion >= 6 && this.eat(o.dot)) {
          a.meta = n;
          var u = this.containsEsc;
          return a.property = this.parseIdent(true), a.property.name !== "target" && this.raiseRecoverable(a.property.start, "The only valid meta property for new is 'new.target'"), u && this.raiseRecoverable(a.start, "'new.target' must not contain escaped characters"), this.allowNewDotTarget || this.raiseRecoverable(a.start, "'new.target' can only be used in functions and class static block"), this.finishNode(a, "MetaProperty");
        }
        var f = this.start, y = this.startLoc, P = this.type === o._import;
        a.callee = this.parseSubscripts(this.parseExprAtom(), f, y, true, false), P && a.callee.type === "ImportExpression" && this.raise(f, "Cannot use new with import()");
        var T = a.callee;
        return T.type !== "TSInstantiationExpression" || (r = T.extra) != null && r.parenthesized || (a.typeParameters = T.typeParameters, a.callee = T.expression), a.arguments = this.eat(o.parenL) ? this.parseExprList(o.parenR, this.options.ecmaVersion >= 8, false) : [], this.finishNode(a, "NewExpression");
      }, c.parseExprOp = function(r, a, n, u, f) {
        var y;
        if (o._in.binop > u && !this.hasPrecedingLineBreak() && (this.isContextual("as") && (y = "TSAsExpression"), p && this.isContextual("satisfies") && (y = "TSSatisfiesExpression"), y)) {
          var P = this.startNodeAt(a, n);
          P.expression = r;
          var T = this.tsTryNextParseConstantContext();
          return P.typeAnnotation = T || this.tsNextThenParseType(), this.finishNode(P, y), this.reScan_lt_gt(), this.parseExprOp(P, a, n, u, f);
        }
        return A.prototype.parseExprOp.call(this, r, a, n, u, f);
      }, c.parseImportSpecifiers = function() {
        var r = [], a = true;
        if (_.tokenIsIdentifier(this.type) && (r.push(this.parseImportDefaultSpecifier()), !this.eat(o.comma))) return r;
        if (this.type === o.star) return r.push(this.parseImportNamespaceSpecifier()), r;
        for (this.expect(o.braceL); !this.eat(o.braceR); ) {
          if (a) a = false;
          else if (this.expect(o.comma), this.afterTrailingComma(o.braceR)) break;
          r.push(this.parseImportSpecifier());
        }
        return r;
      }, c.parseImport = function(r) {
        var a = this.lookahead();
        if (r.importKind = "value", this.importOrExportOuterKind = "value", x(a.type) || this.match(o.star) || this.match(o.braceL)) {
          var n = this.lookahead(2);
          if (n.type !== o.comma && !this.isContextualWithState("from", n) && n.type !== o.eq && this.ts_eatContextualWithState("type", 1, a) && (this.importOrExportOuterKind = "type", r.importKind = "type", a = this.lookahead(), n = this.lookahead(2)), x(a.type) && n.type === o.eq) {
            this.next();
            var u = this.tsParseImportEqualsDeclaration(r);
            return this.importOrExportOuterKind = "value", u;
          }
        }
        return this.next(), this.type === o.string ? (r.specifiers = [], r.source = this.parseExprAtom()) : (r.specifiers = this.parseImportSpecifiers(), this.expectContextual("from"), r.source = this.type === o.string ? this.parseExprAtom() : this.unexpected()), this.parseMaybeImportAttributes(r), this.semicolon(), this.finishNode(r, "ImportDeclaration"), this.importOrExportOuterKind = "value", r.importKind === "type" && r.specifiers.length > 1 && r.specifiers[0].type === "ImportDefaultSpecifier" && this.raise(r.start, le.TypeImportCannotSpecifyDefaultAndNamed), r;
      }, c.parseExportDefaultDeclaration = function() {
        if (this.isAbstractClass()) {
          var r = this.startNode();
          return this.next(), r.abstract = true, this.parseClass(r, true);
        }
        if (this.match(R.interface)) {
          var a = this.tsParseInterfaceDeclaration(this.startNode());
          if (a) return a;
        }
        return A.prototype.parseExportDefaultDeclaration.call(this);
      }, c.parseExportAllDeclaration = function(r, a) {
        return this.options.ecmaVersion >= 11 && (this.eatContextual("as") ? (r.exported = this.parseModuleExportName(), this.checkExport(a, r.exported, this.lastTokStart)) : r.exported = null), this.expectContextual("from"), this.type !== o.string && this.unexpected(), r.source = this.parseExprAtom(), this.parseMaybeImportAttributes(r), this.semicolon(), this.finishNode(r, "ExportAllDeclaration");
      }, c.parseDynamicImport = function(r) {
        if (this.next(), r.source = this.parseMaybeAssign(), this.eat(o.comma)) {
          var a = this.parseExpression();
          r.arguments = [a];
        }
        if (!this.eat(o.parenR)) {
          var n = this.start;
          this.eat(o.comma) && this.eat(o.parenR) ? this.raiseRecoverable(n, "Trailing comma is not allowed in import()") : this.unexpected(n);
        }
        return this.finishNode(r, "ImportExpression");
      }, c.parseExport = function(r, a) {
        var n = this.lookahead();
        if (this.ts_eatWithState(o._import, 2, n)) {
          this.ts_isContextual(R.type) && this.lookaheadCharCode() !== 61 ? (r.importKind = "type", this.importOrExportOuterKind = "type", this.next()) : (r.importKind = "value", this.importOrExportOuterKind = "value");
          var u = this.tsParseImportEqualsDeclaration(r, true);
          return this.importOrExportOuterKind = void 0, u;
        }
        if (this.ts_eatWithState(o.eq, 2, n)) {
          var f = r;
          return f.expression = this.parseExpression(), this.semicolon(), this.importOrExportOuterKind = void 0, this.finishNode(f, "TSExportAssignment");
        }
        if (this.ts_eatContextualWithState("as", 2, n)) {
          var y = r;
          return this.expectContextual("namespace"), y.id = this.parseIdent(), this.semicolon(), this.importOrExportOuterKind = void 0, this.finishNode(y, "TSNamespaceExportDeclaration");
        }
        if (this.ts_isContextualWithState(n, R.type) && this.lookahead(2).type === o.braceL ? (this.next(), this.importOrExportOuterKind = "type", r.exportKind = "type") : (this.importOrExportOuterKind = "value", r.exportKind = "value"), this.next(), this.eat(o.star)) return this.parseExportAllDeclaration(r, a);
        if (this.eat(o._default)) return this.checkExport(a, "default", this.lastTokStart), r.declaration = this.parseExportDefaultDeclaration(), this.finishNode(r, "ExportDefaultDeclaration");
        if (this.shouldParseExportStatement()) r.declaration = this.parseExportDeclaration(r), r.declaration.type === "VariableDeclaration" ? this.checkVariableExport(a, r.declaration.declarations) : this.checkExport(a, r.declaration.id, r.declaration.id.start), r.specifiers = [], r.source = null;
        else {
          if (r.declaration = null, r.specifiers = this.parseExportSpecifiers(a), this.eatContextual("from")) this.type !== o.string && this.unexpected(), r.source = this.parseExprAtom(), this.parseMaybeImportAttributes(r);
          else {
            for (var P, T = Kl(r.specifiers); !(P = T()).done; ) {
              var q = P.value;
              this.checkUnreserved(q.local), this.checkLocalExport(q.local), q.local.type === "Literal" && this.raise(q.local.start, "A string literal cannot be used as an exported binding without `from`.");
            }
            r.source = null;
          }
          this.semicolon();
        }
        return this.finishNode(r, "ExportNamedDeclaration");
      }, c.checkExport = function(r, a, n) {
        r && (typeof a != "string" && (a = a.type === "Identifier" ? a.name : a.value), r[a] = true);
      }, c.parseMaybeDefault = function(r, a, n) {
        var u = A.prototype.parseMaybeDefault.call(this, r, a, n);
        return u.type === "AssignmentPattern" && u.typeAnnotation && u.right.start < u.typeAnnotation.start && this.raise(u.typeAnnotation.start, le.TypeAnnotationAfterAssign), u;
      }, c.typeCastToParameter = function(r) {
        return r.expression.typeAnnotation = r.typeAnnotation, this.resetEndLocation(r.expression, r.typeAnnotation.end), r.expression;
      }, c.toAssignableList = function(r, a) {
        for (var n = 0; n < r.length; n++) {
          var u = r[n];
          (u == null ? void 0 : u.type) === "TSTypeCastExpression" && (r[n] = this.typeCastToParameter(u));
        }
        return A.prototype.toAssignableList.call(this, r, a);
      }, c.reportReservedArrowTypeParam = function(r) {
      }, c.parseExprAtom = function(r, a, n) {
        if (this.type === R.jsxText) return this.jsx_parseText();
        if (this.type === R.jsxTagStart) return this.jsx_parseElement();
        if (this.type === R.at) return this.parseDecorators(), this.parseExprAtom();
        if (x(this.type)) {
          var u = this.potentialArrowAt === this.start, f = this.start, y = this.startLoc, P = this.containsEsc, T = this.parseIdent(false);
          if (this.options.ecmaVersion >= 8 && !P && T.name === "async" && !this.canInsertSemicolon() && this.eat(o._function)) return this.overrideContext(G.f_expr), this.parseFunction(this.startNodeAt(f, y), 0, false, true, a);
          if (u && !this.canInsertSemicolon()) {
            if (this.eat(o.arrow)) return this.parseArrowExpression(this.startNodeAt(f, y), [T], false, a);
            if (this.options.ecmaVersion >= 8 && T.name === "async" && this.type === o.name && !P && (!this.potentialArrowInForAwait || this.value !== "of" || this.containsEsc)) return T = this.parseIdent(false), !this.canInsertSemicolon() && this.eat(o.arrow) || this.unexpected(), this.parseArrowExpression(this.startNodeAt(f, y), [T], true, a);
          }
          return T;
        }
        return A.prototype.parseExprAtom.call(this, r, a, n);
      }, c.parseExprAtomDefault = function() {
        if (x(this.type)) {
          var r = this.potentialArrowAt === this.start, a = this.containsEsc, n = this.parseIdent();
          if (!a && n.name === "async" && !this.canInsertSemicolon()) {
            var u = this.type;
            if (u === o._function) return this.next(), this.parseFunction(this.startNodeAtNode(n), void 0, true, true);
            if (x(u)) {
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
        return E(this.type) ? (r.name = this.value, r) : A.prototype.parseIdentNode.call(this);
      }, c.parseVarStatement = function(r, a, n) {
        n === void 0 && (n = false);
        var u = this.isAmbientContext;
        this.next(), A.prototype.parseVar.call(this, r, false, a, n || u), this.semicolon();
        var f = this.finishNode(r, "VariableDeclaration");
        if (!u) return f;
        for (var y, P = Kl(f.declarations); !(y = P()).done; ) {
          var T = y.value, q = T.init;
          q && (a !== "const" || T.id.typeAnnotation ? this.raise(q.start, le.InitializerNotAllowedInAmbientContext) : q.type !== "StringLiteral" && q.type !== "BooleanLiteral" && q.type !== "NumericLiteral" && q.type !== "BigIntLiteral" && (q.type !== "TemplateLiteral" || q.expressions.length > 0) && !W1(q) && this.raise(q.start, le.ConstInitiailizerMustBeStringOrNumericLiteralOrLiteralEnumReference));
        }
        return f;
      }, c.parseStatement = function(r, a, n) {
        if (this.match(R.at) && this.parseDecorators(true), this.match(o._const) && this.isLookaheadContextual("enum")) {
          var u = this.startNode();
          return this.expect(o._const), this.tsParseEnumDeclaration(u, { const: true });
        }
        if (this.ts_isContextual(R.enum)) return this.tsParseEnumDeclaration(this.startNode());
        if (this.ts_isContextual(R.interface)) {
          var f = this.tsParseInterfaceDeclaration(this.startNode());
          if (f) return f;
        }
        return A.prototype.parseStatement.call(this, r, a, n);
      }, c.parseAccessModifier = function() {
        return this.tsParseModifier(["public", "protected", "private"]);
      }, c.parsePostMemberNameModifiers = function(r) {
        this.eat(o.question) && (r.optional = true), r.readonly && this.match(o.parenL) && this.raise(r.start, le.ClassMethodHasReadonly), r.declare && this.match(o.parenL) && this.raise(r.start, le.ClassMethodHasDeclare);
      }, c.parseExpressionStatement = function(r, a) {
        return (a.type === "Identifier" ? this.tsParseExpressionStatement(r, a) : void 0) || A.prototype.parseExpressionStatement.call(this, r, a);
      }, c.shouldParseExportStatement = function() {
        return !!this.tsIsDeclarationStart() || !!this.match(R.at) || A.prototype.shouldParseExportStatement.call(this);
      }, c.parseConditional = function(r, a, n, u, f) {
        if (this.eat(o.question)) {
          var y = this.startNodeAt(a, n);
          return y.test = r, y.consequent = this.parseMaybeAssign(), this.expect(o.colon), y.alternate = this.parseMaybeAssign(u), this.finishNode(y, "ConditionalExpression");
        }
        return r;
      }, c.parseMaybeConditional = function(r, a) {
        var n = this, u = this.start, f = this.startLoc, y = this.parseExprOps(r, a);
        if (this.checkExpressionErrors(a)) return y;
        if (!this.maybeInArrowParameters || !this.match(o.question)) return this.parseConditional(y, u, f, r, a);
        var P = this.tryParse(function() {
          return n.parseConditional(y, u, f, r, a);
        });
        return P.node ? (P.error && this.setLookaheadState(P.failState), P.node) : (P.error && this.setOptionalParametersError(a, P.error), y);
      }, c.parseParenItem = function(r) {
        var a = this.start, n = this.startLoc;
        if (r = A.prototype.parseParenItem.call(this, r), this.eat(o.question) && (r.optional = true, this.resetEndLocation(r)), this.match(o.colon)) {
          var u = this.startNodeAt(a, n);
          return u.expression = r, u.typeAnnotation = this.tsParseTypeAnnotation(), this.finishNode(u, "TSTypeCastExpression");
        }
        return r;
      }, c.parseExportDeclaration = function(r) {
        var a = this;
        if (!this.isAmbientContext && this.ts_isContextual(R.declare)) return this.tsInAmbientContext(function() {
          return a.parseExportDeclaration(r);
        });
        var n = this.start, u = this.startLoc, f = this.eatContextual("declare");
        !f || !this.ts_isContextual(R.declare) && this.shouldParseExportStatement() || this.raise(this.start, le.ExpectedAmbientAfterExportDeclare);
        var y = x(this.type) && this.tsTryParseExportDeclaration() || this.parseStatement(null);
        return y ? ((y.type === "TSInterfaceDeclaration" || y.type === "TSTypeAliasDeclaration" || f) && (r.exportKind = "type"), f && (this.resetStartLocation(y, n, u), y.declare = true), y) : null;
      }, c.parseClassId = function(r, a) {
        if (a || !this.isContextual("implements")) {
          A.prototype.parseClassId.call(this, r, a);
          var n = this.tsTryParseTypeParameters(this.tsParseInOutModifiers.bind(this));
          n && (r.typeParameters = n);
        }
      }, c.parseClassPropertyAnnotation = function(r) {
        r.optional || (this.value === "!" && this.eat(o.prefix) ? r.definite = true : this.eat(o.question) && (r.optional = true));
        var a = this.tsTryParseTypeAnnotation();
        a && (r.typeAnnotation = a);
      }, c.parseClassField = function(r) {
        if (r.key.type === "PrivateIdentifier") r.abstract && this.raise(r.start, le.PrivateElementHasAbstract), r.accessibility && this.raise(r.start, le.PrivateElementHasAccessibility({ modifier: r.accessibility })), this.parseClassPropertyAnnotation(r);
        else if (this.parseClassPropertyAnnotation(r), this.isAmbientContext && (!r.readonly || r.typeAnnotation) && this.match(o.eq) && this.raise(this.start, le.DeclareClassFieldHasInitializer), r.abstract && this.match(o.eq)) {
          var a = r.key;
          this.raise(this.start, le.AbstractPropertyHasInitializer({ propertyName: a.type !== "Identifier" || r.computed ? "[" + this.input.slice(a.start, a.end) + "]" : a.name }));
        }
        return A.prototype.parseClassField.call(this, r);
      }, c.parseClassMethod = function(r, a, n, u) {
        var f = r.kind === "constructor", y = r.key.type === "PrivateIdentifier", P = this.tsTryParseTypeParameters();
        y ? (P && (r.typeParameters = P), r.accessibility && this.raise(r.start, le.PrivateMethodsHasAccessibility({ modifier: r.accessibility }))) : P && f && this.raise(P.start, le.ConstructorHasTypeParameters);
        var T = r.declare, q = r.kind;
        !(T !== void 0 && T) || q !== "get" && q !== "set" || this.raise(r.start, le.DeclareAccessor({ kind: q })), P && (r.typeParameters = P);
        var H = r.key;
        r.kind === "constructor" ? (a && this.raise(H.start, "Constructor can't be a generator"), n && this.raise(H.start, "Constructor can't be an async method")) : r.static && Ql(r, "prototype") && this.raise(H.start, "Classes may not have a static property named prototype");
        var J = r.value = this.parseMethod(a, n, u, true, r);
        return r.kind === "get" && J.params.length !== 0 && this.raiseRecoverable(J.start, "getter should have no params"), r.kind === "set" && J.params.length !== 1 && this.raiseRecoverable(J.start, "setter should have exactly one param"), r.kind === "set" && J.params[0].type === "RestElement" && this.raiseRecoverable(J.params[0].start, "Setter cannot use rest params"), this.finishNode(r, "MethodDefinition");
      }, c.isClassMethod = function() {
        return this.match(o.relational);
      }, c.parseClassElement = function(r) {
        var a = this;
        if (this.eat(o.semi)) return null;
        var n, u = this.options.ecmaVersion, f = this.startNode(), y = "", P = false, T = false, q = "method", H = ["declare", "private", "public", "protected", "accessor", "override", "abstract", "readonly", "static"], J = this.tsParseModifiers({ modified: f, allowedModifiers: H, disallowedModifiers: ["in", "out"], stopOnStartOfClassStaticBlock: true, errorTemplate: le.InvalidModifierOnTypeParameterPositions });
        n = !!J.static;
        var re = function() {
          if (!a.tsIsStartOfStaticBlocks()) {
            var Q = a.tsTryParseIndexSignature(f);
            if (Q) return f.abstract && a.raise(f.start, le.IndexSignatureHasAbstract), f.accessibility && a.raise(f.start, le.IndexSignatureHasAccessibility({ modifier: f.accessibility })), f.declare && a.raise(f.start, le.IndexSignatureHasDeclare), f.override && a.raise(f.start, le.IndexSignatureHasOverride), Q;
            if (!a.inAbstractClass && f.abstract && a.raise(f.start, le.NonAbstractClassHasAbstractMethod), f.override && r && a.raise(f.start, le.OverrideNotInSubClass), f.static = n, n && (a.isClassElementNameStart() || a.type === o.star || (y = "static")), !y && u >= 8 && a.eatContextual("async") && (!a.isClassElementNameStart() && a.type !== o.star || a.canInsertSemicolon() ? y = "async" : T = true), !y && (u >= 9 || !T) && a.eat(o.star) && (P = true), !y && !T && !P) {
              var pe = a.value;
              (a.eatContextual("get") || a.eatContextual("set")) && (a.isClassElementNameStart() ? q = pe : y = pe);
            }
            if (y ? (f.computed = false, f.key = a.startNodeAt(a.lastTokStart, a.lastTokStartLoc), f.key.name = y, a.finishNode(f.key, "Identifier")) : a.parseClassElementName(f), a.parsePostMemberNameModifiers(f), a.isClassMethod() || u < 13 || a.type === o.parenL || q !== "method" || P || T) {
              var de = !f.static && Ql(f, "constructor"), fe = de && r;
              de && q !== "method" && a.raise(f.key.start, "Constructor can't have get/set modifier"), f.kind = de ? "constructor" : q, a.parseClassMethod(f, P, T, fe);
            } else a.parseClassField(f);
            return f;
          }
          if (a.next(), a.next(), a.tsHasSomeModifiers(f, H) && a.raise(a.start, le.StaticBlockCannotHaveModifier), u >= 13) return A.prototype.parseClassStaticBlock.call(a, f), f;
        };
        return f.declare ? this.tsInAmbientContext(re) : re(), f;
      }, c.isClassElementNameStart = function() {
        return !!this.tsIsIdentifier() || A.prototype.isClassElementNameStart.call(this);
      }, c.parseClassSuper = function(r) {
        A.prototype.parseClassSuper.call(this, r), r.superClass && (this.tsMatchLeftRelational() || this.match(o.bitShift)) && (r.superTypeParameters = this.tsParseTypeArgumentsInExpression()), this.eatContextual("implements") && (r.implements = this.tsParseHeritageClause("implements"));
      }, c.parseFunctionParams = function(r) {
        var a = this.tsTryParseTypeParameters();
        a && (r.typeParameters = a), A.prototype.parseFunctionParams.call(this, r);
      }, c.parseVarId = function(r, a) {
        A.prototype.parseVarId.call(this, r, a), r.id.type === "Identifier" && !this.hasPrecedingLineBreak() && this.value === "!" && this.eat(o.prefix) && (r.definite = true);
        var n = this.tsTryParseTypeAnnotation();
        n && (r.id.typeAnnotation = n, this.resetEndLocation(r.id));
      }, c.parseArrowExpression = function(r, a, n, u) {
        this.match(o.colon) && (r.returnType = this.tsParseTypeAnnotation());
        var f = this.yieldPos, y = this.awaitPos, P = this.awaitIdentPos;
        this.enterScope(16 | pl(n, false)), this.initFunction(r);
        var T = this.maybeInArrowParameters;
        return this.options.ecmaVersion >= 8 && (r.async = !!n), this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0, this.maybeInArrowParameters = true, r.params = this.toAssignableList(a, true), this.maybeInArrowParameters = false, this.parseFunctionBody(r, true, false, u), this.yieldPos = f, this.awaitPos = y, this.awaitIdentPos = P, this.maybeInArrowParameters = T, this.finishNode(r, "ArrowFunctionExpression");
      }, c.parseMaybeAssignOrigin = function(r, a, n) {
        if (this.isContextual("yield")) {
          if (this.inGenerator) return this.parseYield(r);
          this.exprAllowed = false;
        }
        var u = false, f = -1, y = -1, P = -1;
        a ? (f = a.parenthesizedAssign, y = a.trailingComma, P = a.doubleProto, a.parenthesizedAssign = a.trailingComma = -1) : (a = new sr(), u = true);
        var T = this.start, q = this.startLoc;
        (this.type === o.parenL || x(this.type)) && (this.potentialArrowAt = this.start, this.potentialArrowInForAwait = r === "await");
        var H = this.parseMaybeConditional(r, a);
        if (n && (H = n.call(this, H, T, q)), this.type.isAssign) {
          var J = this.startNodeAt(T, q);
          return J.operator = this.value, this.type === o.eq && (H = this.toAssignable(H, true, a)), u || (a.parenthesizedAssign = a.trailingComma = a.doubleProto = -1), a.shorthandAssign >= H.start && (a.shorthandAssign = -1), this.type === o.eq ? this.checkLValPattern(H) : this.checkLValSimple(H), J.left = H, this.next(), J.right = this.parseMaybeAssign(r), P > -1 && (a.doubleProto = P), this.finishNode(J, "AssignmentExpression");
        }
        return u && this.checkExpressionErrors(a, true), f > -1 && (a.parenthesizedAssign = f), y > -1 && (a.trailingComma = y), H;
      }, c.parseMaybeAssign = function(r, a, n) {
        var u, f, y, P, T, q, H, J, re, Q, pe, de = this;
        if (this.matchJsx("jsxTagStart") || this.tsMatchLeftRelational()) {
          if (J = this.cloneCurLookaheadState(), !(re = this.tryParse(function() {
            return de.parseMaybeAssignOrigin(r, a, n);
          }, J)).error) return re.node;
          var fe = this.context, he = fe[fe.length - 1];
          he === _.tokContexts.tc_oTag && fe[fe.length - 2] === _.tokContexts.tc_expr ? (fe.pop(), fe.pop()) : he !== _.tokContexts.tc_oTag && he !== _.tokContexts.tc_expr || fe.pop();
        }
        if (!((u = re) != null && u.error || this.tsMatchLeftRelational())) return this.parseMaybeAssignOrigin(r, a, n);
        J && !this.compareLookaheadState(J, this.getCurLookaheadState()) || (J = this.cloneCurLookaheadState());
        var we = this.tryParse(function(Oe) {
          var Ee, Ue;
          pe = de.tsParseTypeParameters();
          var Ne = de.parseMaybeAssignOrigin(r, a, n);
          return (Ne.type !== "ArrowFunctionExpression" || (Ee = Ne.extra) != null && Ee.parenthesized) && Oe(), ((Ue = pe) == null ? void 0 : Ue.params.length) !== 0 && de.resetStartLocationFromNode(Ne, pe), Ne.typeParameters = pe, Ne;
        }, J);
        if (!we.error && !we.aborted) return pe && this.reportReservedArrowTypeParam(pe), we.node;
        if (!re && (Yl(true), !(Q = this.tryParse(function() {
          return de.parseMaybeAssignOrigin(r, a, n);
        }, J)).error)) return Q.node;
        if ((f = re) != null && f.node) return this.setLookaheadState(re.failState), re.node;
        if (we.node) return this.setLookaheadState(we.failState), pe && this.reportReservedArrowTypeParam(pe), we.node;
        if ((y = Q) != null && y.node) return this.setLookaheadState(Q.failState), Q.node;
        throw (P = re) != null && P.thrown ? re.error : we.thrown ? we.error : (T = Q) != null && T.thrown ? Q.error : ((q = re) == null ? void 0 : q.error) || we.error || ((H = Q) == null ? void 0 : H.error);
      }, c.parseAssignableListItem = function(r) {
        for (var a = []; this.match(R.at); ) a.push(this.parseDecorator());
        var n, u = this.start, f = this.startLoc, y = false, P = false;
        if (r !== void 0) {
          var T = {};
          this.tsParseModifiers({ modified: T, allowedModifiers: ["public", "private", "protected", "override", "readonly"] }), n = T.accessibility, P = T.override, y = T.readonly, r === false && (n || y || P) && this.raise(f.start, le.UnexpectedParameterModifier);
        }
        var q = this.parseMaybeDefault(u, f);
        this.parseBindingListItem(q);
        var H = this.parseMaybeDefault(q.start, q.loc, q);
        if (a.length && (H.decorators = a), n || y || P) {
          var J = this.startNodeAt(u, f);
          return n && (J.accessibility = n), y && (J.readonly = y), P && (J.override = P), H.type !== "Identifier" && H.type !== "AssignmentPattern" && this.raise(J.start, le.UnsupportedParameterPropertyKind), J.parameter = H, this.finishNode(J, "TSParameterProperty");
        }
        return H;
      }, c.checkLValInnerPattern = function(r, a, n) {
        a === void 0 && (a = 0), r.type === "TSParameterProperty" ? this.checkLValInnerPattern(r.parameter, a, n) : A.prototype.checkLValInnerPattern.call(this, r, a, n);
      }, c.parseBindingListItem = function(r) {
        this.eat(o.question) && (r.type === "Identifier" || this.isAmbientContext || this.inType || this.raise(r.start, le.PatternIsOptional), r.optional = true);
        var a = this.tsTryParseTypeAnnotation();
        return a && (r.typeAnnotation = a), this.resetEndLocation(r), r;
      }, c.isAssignable = function(r, a) {
        var n = this;
        switch (r.type) {
          case "TSTypeCastExpression":
            return this.isAssignable(r.expression, a);
          case "TSParameterProperty":
          case "Identifier":
          case "ObjectPattern":
          case "ArrayPattern":
          case "AssignmentPattern":
          case "RestElement":
            return true;
          case "ObjectExpression":
            var u = r.properties.length - 1;
            return r.properties.every(function(f, y) {
              return f.type !== "ObjectMethod" && (y === u || f.type !== "SpreadElement") && n.isAssignable(f);
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
            return !a;
          default:
            return false;
        }
      }, c.toAssignable = function(r, a, n) {
        switch (a === void 0 && (a = false), n === void 0 && (n = new sr()), r.type) {
          case "ParenthesizedExpression":
            return this.toAssignableParenthesizedExpression(r, a, n);
          case "TSAsExpression":
          case "TSSatisfiesExpression":
          case "TSNonNullExpression":
          case "TSTypeAssertion":
            return a || this.raise(r.start, le.UnexpectedTypeCastInParameter), this.toAssignable(r.expression, a, n);
          case "MemberExpression":
            break;
          case "AssignmentExpression":
            return a || r.left.type !== "TSTypeCastExpression" || (r.left = this.typeCastToParameter(r.left)), A.prototype.toAssignable.call(this, r, a, n);
          case "TSTypeCastExpression":
            return this.typeCastToParameter(r);
          default:
            return A.prototype.toAssignable.call(this, r, a, n);
        }
        return r;
      }, c.toAssignableParenthesizedExpression = function(r, a, n) {
        switch (r.expression.type) {
          case "TSAsExpression":
          case "TSSatisfiesExpression":
          case "TSNonNullExpression":
          case "TSTypeAssertion":
          case "ParenthesizedExpression":
            return this.toAssignable(r.expression, a, n);
          default:
            return A.prototype.toAssignable.call(this, r, a, n);
        }
      }, c.curPosition = function() {
        if (this.options.locations) {
          var r = A.prototype.curPosition.call(this);
          return Object.defineProperty(r, "offset", { get: function() {
            return function(a) {
              var n = new w.Position(this.line, this.column + a);
              return n.index = this.index + a, n;
            };
          } }), r.index = this.pos, r;
        }
      }, c.parseBindingAtom = function() {
        return this.type === o._this ? this.parseIdent(true) : A.prototype.parseBindingAtom.call(this);
      }, c.shouldParseArrow = function(r) {
        var a, n = this;
        if (a = this.match(o.colon) ? r.every(function(f) {
          return n.isAssignable(f, true);
        }) : !this.canInsertSemicolon()) {
          if (this.match(o.colon)) {
            var u = this.tryParse(function(f) {
              var y = n.tsParseTypeOrTypePredicateAnnotation(o.colon);
              return !n.canInsertSemicolon() && n.match(o.arrow) || f(), y;
            });
            if (u.aborted) return this.shouldParseArrowReturnType = void 0, false;
            u.thrown || (u.error && this.setLookaheadState(u.failState), this.shouldParseArrowReturnType = u.node);
          }
          return !!this.match(o.arrow) || (this.shouldParseArrowReturnType = void 0, false);
        }
        return this.shouldParseArrowReturnType = void 0, a;
      }, c.parseParenArrowList = function(r, a, n, u) {
        var f = this.startNodeAt(r, a);
        return f.returnType = this.shouldParseArrowReturnType, this.shouldParseArrowReturnType = void 0, this.parseArrowExpression(f, n, false, u);
      }, c.parseParenAndDistinguishExpression = function(r, a) {
        var n, u = this.start, f = this.startLoc, y = this.options.ecmaVersion >= 8;
        if (this.options.ecmaVersion >= 6) {
          var P = this.maybeInArrowParameters;
          this.maybeInArrowParameters = true, this.next();
          var T, q = this.start, H = this.startLoc, J = [], re = true, Q = false, pe = new sr(), de = this.yieldPos, fe = this.awaitPos;
          for (this.yieldPos = 0, this.awaitPos = 0; this.type !== o.parenR; ) {
            if (re ? re = false : this.expect(o.comma), y && this.afterTrailingComma(o.parenR, true)) {
              Q = true;
              break;
            }
            if (this.type === o.ellipsis) {
              T = this.start, J.push(this.parseParenItem(this.parseRestBinding())), this.type === o.comma && this.raise(this.start, "Comma is not permitted after the rest element");
              break;
            }
            J.push(this.parseMaybeAssign(a, pe, this.parseParenItem));
          }
          var he = this.lastTokEnd, we = this.lastTokEndLoc;
          if (this.expect(o.parenR), this.maybeInArrowParameters = P, r && this.shouldParseArrow(J) && this.eat(o.arrow)) return this.checkPatternErrors(pe, false), this.checkYieldAwaitInDefaultParams(), this.yieldPos = de, this.awaitPos = fe, this.parseParenArrowList(u, f, J, a);
          J.length && !Q || this.unexpected(this.lastTokStart), T && this.unexpected(T), this.checkExpressionErrors(pe, true), this.yieldPos = de || this.yieldPos, this.awaitPos = fe || this.awaitPos, J.length > 1 ? ((n = this.startNodeAt(q, H)).expressions = J, this.finishNodeAt(n, "SequenceExpression", he, we)) : n = J[0];
        } else n = this.parseParenExpression();
        if (this.options.preserveParens) {
          var Oe = this.startNodeAt(u, f);
          return Oe.expression = n, this.finishNode(Oe, "ParenthesizedExpression");
        }
        return n;
      }, c.parseTaggedTemplateExpression = function(r, a, n, u) {
        var f = this.startNodeAt(a, n);
        return f.tag = r, f.quasi = this.parseTemplate({ isTagged: true }), u && this.raise(a, "Tagged Template Literals are not allowed in optionalChain."), this.finishNode(f, "TaggedTemplateExpression");
      }, c.shouldParseAsyncArrow = function() {
        var r = this;
        if (!this.match(o.colon)) return !this.canInsertSemicolon() && this.eat(o.arrow);
        var a = this.tryParse(function(n) {
          var u = r.tsParseTypeOrTypePredicateAnnotation(o.colon);
          return !r.canInsertSemicolon() && r.match(o.arrow) || n(), u;
        });
        return a.aborted ? (this.shouldParseAsyncArrowReturnType = void 0, false) : a.thrown ? void 0 : (a.error && this.setLookaheadState(a.failState), this.shouldParseAsyncArrowReturnType = a.node, !this.canInsertSemicolon() && this.eat(o.arrow));
      }, c.parseSubscriptAsyncArrow = function(r, a, n, u) {
        var f = this.startNodeAt(r, a);
        return f.returnType = this.shouldParseAsyncArrowReturnType, this.shouldParseAsyncArrowReturnType = void 0, this.parseArrowExpression(f, n, true, u);
      }, c.parseExprList = function(r, a, n, u) {
        for (var f = [], y = true; !this.eat(r); ) {
          if (y) y = false;
          else if (this.expect(o.comma), a && this.afterTrailingComma(r)) break;
          var P = void 0;
          n && this.type === o.comma ? P = null : this.type === o.ellipsis ? (P = this.parseSpread(u), u && this.type === o.comma && u.trailingComma < 0 && (u.trailingComma = this.start)) : P = this.parseMaybeAssign(false, u, this.parseParenItem), f.push(P);
        }
        return f;
      }, c.parseSubscript = function(r, a, n, u, f, y, P) {
        var T = this, q = y;
        if (!this.hasPrecedingLineBreak() && this.value === "!" && this.match(o.prefix)) {
          this.exprAllowed = false, this.next();
          var H = this.startNodeAt(a, n);
          return H.expression = r, r = this.finishNode(H, "TSNonNullExpression");
        }
        var J = false;
        if (this.match(o.questionDot) && this.lookaheadCharCode() === 60) {
          if (u) return r;
          r.optional = true, q = J = true, this.next();
        }
        if (this.tsMatchLeftRelational() || this.match(o.bitShift)) {
          var re, Q = this.tsTryParseAndCatch(function() {
            if (!u && T.atPossibleAsyncArrow(r)) {
              var bt = T.tsTryParseGenericAsyncArrowFunction(a, n, P);
              if (bt) return r = bt;
            }
            var j = T.tsParseTypeArgumentsInExpression();
            if (!j) return r;
            if (J && !T.match(o.parenL)) return re = T.curPosition(), r;
            if (S(T.type) || T.type === o.backQuote) {
              var X = T.parseTaggedTemplateExpression(r, a, n, q);
              return X.typeParameters = j, X;
            }
            if (!u && T.eat(o.parenL)) {
              var K = new sr(), ee = T.startNodeAt(a, n);
              return ee.callee = r, ee.arguments = T.parseExprList(o.parenR, T.options.ecmaVersion >= 8, false, K), T.tsCheckForInvalidTypeCasts(ee.arguments), ee.typeParameters = j, q && (ee.optional = J), T.checkExpressionErrors(K, true), r = T.finishNode(ee, "CallExpression");
            }
            var ce = T.type;
            if (!(T.tsMatchRightRelational() || ce === o.bitShift || ce !== o.parenL && (se = ce, !!se.startsExpr) && !T.hasPrecedingLineBreak())) {
              var se, Re = T.startNodeAt(a, n);
              return Re.expression = r, Re.typeParameters = j, T.finishNode(Re, "TSInstantiationExpression");
            }
          });
          if (re && this.unexpected(re), Q) return Q.type === "TSInstantiationExpression" && (this.match(o.dot) || this.match(o.questionDot) && this.lookaheadCharCode() !== 40) && this.raise(this.start, le.InvalidPropertyAccessAfterInstantiationExpression), r = Q;
        }
        var pe = this.options.ecmaVersion >= 11, de = pe && this.eat(o.questionDot);
        u && de && this.raise(this.lastTokStart, "Optional chaining cannot appear in the callee of new expressions");
        var fe = this.eat(o.bracketL);
        if (fe || de && this.type !== o.parenL && this.type !== o.backQuote || this.eat(o.dot)) {
          var he = this.startNodeAt(a, n);
          he.object = r, fe ? (he.property = this.parseExpression(), this.expect(o.bracketR)) : he.property = this.type === o.privateId && r.type !== "Super" ? this.parsePrivateIdent() : this.parseIdent(this.options.allowReserved !== "never"), he.computed = !!fe, pe && (he.optional = de), r = this.finishNode(he, "MemberExpression");
        } else if (!u && this.eat(o.parenL)) {
          var we = this.maybeInArrowParameters;
          this.maybeInArrowParameters = true;
          var Oe = new sr(), Ee = this.yieldPos, Ue = this.awaitPos, Ne = this.awaitIdentPos;
          this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0;
          var Ze = this.parseExprList(o.parenR, this.options.ecmaVersion >= 8, false, Oe);
          if (f && !de && this.shouldParseAsyncArrow()) this.checkPatternErrors(Oe, false), this.checkYieldAwaitInDefaultParams(), this.awaitIdentPos > 0 && this.raise(this.awaitIdentPos, "Cannot use 'await' as identifier inside an async function"), this.yieldPos = Ee, this.awaitPos = Ue, this.awaitIdentPos = Ne, r = this.parseSubscriptAsyncArrow(a, n, Ze, P);
          else {
            this.checkExpressionErrors(Oe, true), this.yieldPos = Ee || this.yieldPos, this.awaitPos = Ue || this.awaitPos, this.awaitIdentPos = Ne || this.awaitIdentPos;
            var Je = this.startNodeAt(a, n);
            Je.callee = r, Je.arguments = Ze, pe && (Je.optional = de), r = this.finishNode(Je, "CallExpression");
          }
          this.maybeInArrowParameters = we;
        } else if (this.type === o.backQuote) {
          (de || q) && this.raise(this.start, "Optional chaining cannot appear in the tag of tagged template expressions");
          var vt = this.startNodeAt(a, n);
          vt.tag = r, vt.quasi = this.parseTemplate({ isTagged: true }), r = this.finishNode(vt, "TaggedTemplateExpression");
        }
        return r;
      }, c.parseGetterSetter = function(r) {
        r.kind = r.key.name, this.parsePropertyName(r), r.value = this.parseMethod(false);
        var a = r.kind === "get" ? 0 : 1, n = r.value.params[0], u = n && this.isThisParam(n);
        r.value.params.length !== (a = u ? a + 1 : a) ? this.raiseRecoverable(r.value.start, r.kind === "get" ? "getter should have no params" : "setter should have exactly one param") : r.kind === "set" && r.value.params[0].type === "RestElement" && this.raiseRecoverable(r.value.params[0].start, "Setter cannot use rest params");
      }, c.parseProperty = function(r, a) {
        if (!r) {
          var n = [];
          if (this.match(R.at)) for (; this.match(R.at); ) n.push(this.parseDecorator());
          var u = A.prototype.parseProperty.call(this, r, a);
          return u.type === "SpreadElement" && n.length && this.raise(u.start, "Decorators can't be used with SpreadElement"), n.length && (u.decorators = n, n = []), u;
        }
        return A.prototype.parseProperty.call(this, r, a);
      }, c.parseCatchClauseParam = function() {
        var r = this.parseBindingAtom(), a = r.type === "Identifier";
        this.enterScope(a ? 32 : 0), this.checkLValPattern(r, a ? 4 : 2);
        var n = this.tsTryParseTypeAnnotation();
        return n && (r.typeAnnotation = n, this.resetEndLocation(r)), this.expect(o.parenR), r;
      }, c.parseClass = function(r, a) {
        var n = this.inAbstractClass;
        this.inAbstractClass = !!r.abstract;
        try {
          this.next(), this.takeDecorators(r);
          var u = this.strict;
          this.strict = true, this.parseClassId(r, a), this.parseClassSuper(r);
          var f = this.enterClassBody(), y = this.startNode(), P = false;
          y.body = [];
          var T = [];
          for (this.expect(o.braceL); this.type !== o.braceR; ) if (this.match(R.at)) T.push(this.parseDecorator());
          else {
            var q = this.parseClassElement(r.superClass !== null);
            T.length && (q.decorators = T, this.resetStartLocationFromNode(q, T[0]), T = []), q && (y.body.push(q), q.type === "MethodDefinition" && q.kind === "constructor" && q.value.type === "FunctionExpression" ? (P && this.raiseRecoverable(q.start, "Duplicate constructor in the same class"), P = true, q.decorators && q.decorators.length > 0 && this.raise(q.start, "Decorators can't be used with a constructor. Did you mean '@dec class { ... }'?")) : q.key && q.key.type === "PrivateIdentifier" && F1(f, q) && this.raiseRecoverable(q.key.start, "Identifier '#" + q.key.name + "' has already been declared"));
          }
          return this.strict = u, this.next(), T.length && this.raise(this.start, "Decorators must be attached to a class element."), r.body = this.finishNode(y, "ClassBody"), this.exitClassBody(), this.finishNode(r, a ? "ClassDeclaration" : "ClassExpression");
        } finally {
          this.inAbstractClass = n;
        }
      }, c.parseClassFunctionParams = function() {
        var r = this.tsTryParseTypeParameters(this.tsParseConstModifier), a = this.parseBindingList(o.parenR, false, this.options.ecmaVersion >= 8, true);
        return r && (a.typeParameters = r), a;
      }, c.parseMethod = function(r, a, n, u, f) {
        var y = this.startNode(), P = this.yieldPos, T = this.awaitPos, q = this.awaitIdentPos;
        if (this.initFunction(y), this.options.ecmaVersion >= 6 && (y.generator = r), this.options.ecmaVersion >= 8 && (y.async = !!a), this.yieldPos = 0, this.awaitPos = 0, this.awaitIdentPos = 0, this.enterScope(64 | pl(a, y.generator) | (n ? 128 : 0)), this.expect(o.parenL), y.params = this.parseClassFunctionParams(), this.checkYieldAwaitInDefaultParams(), this.parseFunctionBody(y, false, true, false, { isClassMethod: u }), this.yieldPos = P, this.awaitPos = T, this.awaitIdentPos = q, f && f.abstract && y.body) {
          var H = f.key;
          this.raise(f.start, le.AbstractMethodHasImplementation({ methodName: H.type !== "Identifier" || f.computed ? "[" + this.input.slice(H.start, H.end) + "]" : H.name }));
        }
        return this.finishNode(y, "FunctionExpression");
      }, N.parse = function(r, a) {
        if (a.locations === false) throw new Error("You have to enable options.locations while using acorn-typescript");
        a.locations = true;
        var n = new this(a, r);
        return s && (n.isAmbientContext = true), n.parse();
      }, N.parseExpressionAt = function(r, a, n) {
        if (n.locations === false) throw new Error("You have to enable options.locations while using acorn-typescript");
        n.locations = true;
        var u = new this(n, r, a);
        return s && (u.isAmbientContext = true), u.nextToken(), u.parseExpression();
      }, c.parseImportSpecifier = function() {
        if (this.ts_isContextual(R.type)) {
          var r = this.startNode();
          return r.imported = this.parseModuleExportName(), this.parseTypeOnlyImportExportSpecifier(r, true, this.importOrExportOuterKind === "type"), this.finishNode(r, "ImportSpecifier");
        }
        var a = A.prototype.parseImportSpecifier.call(this);
        return a.importKind = "value", a;
      }, c.parseExportSpecifier = function(r) {
        var a = this.ts_isContextual(R.type);
        if (!this.match(o.string) && a) {
          var n = this.startNode();
          return n.local = this.parseModuleExportName(), this.parseTypeOnlyImportExportSpecifier(n, false, this.importOrExportOuterKind === "type"), this.finishNode(n, "ExportSpecifier"), this.checkExport(r, n.exported, n.exported.start), n;
        }
        var u = A.prototype.parseExportSpecifier.call(this, r);
        return u.exportKind = "value", u;
      }, c.parseTypeOnlyImportExportSpecifier = function(r, a, n) {
        var u, f = a ? "imported" : "local", y = a ? "local" : "exported", P = r[f], T = false, q = true, H = P.start;
        if (this.isContextual("as")) {
          var J = this.parseIdent();
          if (this.isContextual("as")) {
            var re = this.parseIdent();
            E(this.type) ? (T = true, P = J, u = a ? this.parseIdent() : this.parseModuleExportName(), q = false) : (u = re, q = false);
          } else E(this.type) ? (q = false, u = a ? this.parseIdent() : this.parseModuleExportName()) : (T = true, P = J);
        } else E(this.type) && (T = true, a ? (P = A.prototype.parseIdent.call(this, true), this.isContextual("as") || this.checkUnreserved(P)) : P = this.parseModuleExportName());
        T && n && this.raise(H, a ? le.TypeModifierIsUsedInTypeImports : le.TypeModifierIsUsedInTypeExports), r[f] = P, r[y] = u, r[a ? "importKind" : "exportKind"] = T ? "type" : "value", q && this.eatContextual("as") && (r[y] = a ? this.parseIdent() : this.parseModuleExportName()), r[y] || (r[y] = this.copyNode(r[f])), a && this.checkLValSimple(r[y], 2);
      }, c.raiseCommonCheck = function(r, a, n) {
        return a === "Comma is not permitted after the rest element" ? this.isAmbientContext && this.match(o.comma) && this.lookaheadCharCode() === 41 ? void this.next() : A.prototype.raise.call(this, r, a) : n ? A.prototype.raiseRecoverable.call(this, r, a) : A.prototype.raise.call(this, r, a);
      }, c.raiseRecoverable = function(r, a) {
        return this.raiseCommonCheck(r, a, true);
      }, c.raise = function(r, a) {
        return this.raiseCommonCheck(r, a, true);
      }, c.updateContext = function(r) {
        var a = this.type;
        if (a == o.braceL) {
          var n = this.curContext();
          n == v.tc_oTag ? this.context.push(G.b_expr) : n == v.tc_expr ? this.context.push(G.b_tmpl) : A.prototype.updateContext.call(this, r), this.exprAllowed = true;
        } else {
          if (a !== o.slash || r !== R.jsxTagStart) return A.prototype.updateContext.call(this, r);
          this.context.length -= 2, this.context.push(v.tc_cTag), this.exprAllowed = false;
        }
      }, c.jsx_parseOpeningElementAt = function(r, a) {
        var n = this, u = this.startNodeAt(r, a), f = this.jsx_parseElementName();
        if (f && (u.name = f), this.match(o.relational) || this.match(o.bitShift)) {
          var y = this.tsTryParseAndCatch(function() {
            return n.tsParseTypeArgumentsInExpression();
          });
          y && (u.typeParameters = y);
        }
        for (u.attributes = []; this.type !== o.slash && this.type !== R.jsxTagEnd; ) u.attributes.push(this.jsx_parseAttribute());
        return u.selfClosing = this.eat(o.slash), this.expect(R.jsxTagEnd), this.finishNode(u, f ? "JSXOpeningElement" : "JSXOpeningFragment");
      }, c.enterScope = function(r) {
        r === ir && this.importsStack.push([]), A.prototype.enterScope.call(this, r);
        var a = A.prototype.currentScope.call(this);
        a.types = [], a.enums = [], a.constEnums = [], a.classes = [], a.exportOnlyBindings = [];
      }, c.exitScope = function() {
        A.prototype.currentScope.call(this).flags === ir && this.importsStack.pop(), A.prototype.exitScope.call(this);
      }, c.hasImport = function(r, a) {
        var n = this.importsStack.length;
        if (this.importsStack[n - 1].indexOf(r) > -1) return true;
        if (!a && n > 1) {
          for (var u = 0; u < n - 1; u++) if (this.importsStack[u].indexOf(r) > -1) return true;
        }
        return false;
      }, c.maybeExportDefined = function(r, a) {
        this.inModule && 1 & r.flags && this.undefinedExports.delete(a);
      }, c.isRedeclaredInScope = function(r, a, n) {
        return !!(0 & n) && (2 & n ? r.lexical.indexOf(a) > -1 || r.functions.indexOf(a) > -1 || r.var.indexOf(a) > -1 : 3 & n ? r.lexical.indexOf(a) > -1 || !A.prototype.treatFunctionsAsVarInScope.call(this, r) && r.var.indexOf(a) > -1 : r.lexical.indexOf(a) > -1 && !(32 & r.flags && r.lexical[0] === a) || !this.treatFunctionsAsVarInScope(r) && r.functions.indexOf(a) > -1);
      }, c.checkRedeclarationInScope = function(r, a, n, u) {
        this.isRedeclaredInScope(r, a, n) && this.raise(u, "Identifier '" + a + "' has already been declared.");
      }, c.declareName = function(r, a, n) {
        if (4096 & a) return this.hasImport(r, true) && this.raise(n, "Identifier '" + r + "' has already been declared."), void this.importsStack[this.importsStack.length - 1].push(r);
        var u = this.currentScope();
        if (1024 & a) return this.maybeExportDefined(u, r), void u.exportOnlyBindings.push(r);
        A.prototype.declareName.call(this, r, a, n), 0 & a && (0 & a || (this.checkRedeclarationInScope(u, r, a, n), this.maybeExportDefined(u, r)), u.types.push(r)), 256 & a && u.enums.push(r), 512 & a && u.constEnums.push(r), 128 & a && u.classes.push(r);
      }, c.checkLocalExport = function(r) {
        var a = r.name;
        if (!this.hasImport(a)) {
          for (var n = this.scopeStack.length - 1; n >= 0; n--) {
            var u = this.scopeStack[n];
            if (u.types.indexOf(a) > -1 || u.exportOnlyBindings.indexOf(a) > -1) return;
          }
          A.prototype.checkLocalExport.call(this, r);
        }
      }, I = N, V = [{ key: "acornTypeScript", get: function() {
        return _;
      } }], (M = [{ key: "acornTypeScript", get: function() {
        return _;
      } }]) && zl(I.prototype, M), V && zl(I, V), Object.defineProperty(I, "prototype", { writable: false }), N;
    }(m);
    return Z;
  };
}
He.extend(z1({ allowSatisfies: true }));
class X1 extends Error {
  constructor(t, i, s) {
    super(i);
    __publicField(this, "message", "");
    __privateAdd(this, _e2);
    this.stack = "", __privateSet(this, _e2, new Ah(t, i, s)), Object.assign(this, __privateGet(this, _e2)), this.name = "CompileError";
  }
  toString() {
    return __privateGet(this, _e2).toString();
  }
  toJSON() {
    return __privateGet(this, _e2).toJSON();
  }
}
_e2 = new WeakMap();
function kl(e, t, i) {
  throw new X1(t, i, void 0);
}
function K1(e, t) {
  kl(e, "options_invalid_value", `Invalid compiler option: ${t}
https://svelte.dev/e/options_invalid_value`);
}
function J1(e, t) {
  kl(e, "options_removed", `Invalid compiler option: ${t}
https://svelte.dev/e/options_removed`);
}
function Q1(e, t) {
  kl(e, "options_unrecognised", `Unrecognised compiler option ${t}
https://svelte.dev/e/options_unrecognised`);
}
const Y1 = { "CounterClockwiseContourIntegral;": 8755, "ClockwiseContourIntegral;": 8754, "DoubleLongLeftRightArrow;": 10234, "NotNestedGreaterGreater;": 10914, "DiacriticalDoubleAcute;": 733, "NotSquareSupersetEqual;": 8931, "CloseCurlyDoubleQuote;": 8221, "DoubleContourIntegral;": 8751, "FilledVerySmallSquare;": 9642, "NegativeVeryThinSpace;": 8203, "NotPrecedesSlantEqual;": 8928, "NotRightTriangleEqual;": 8941, "NotSucceedsSlantEqual;": 8929, "CapitalDifferentialD;": 8517, "DoubleLeftRightArrow;": 8660, "DoubleLongRightArrow;": 10233, "EmptyVerySmallSquare;": 9643, "NestedGreaterGreater;": 8811, "NotDoubleVerticalBar;": 8742, "NotGreaterSlantEqual;": 10878, "NotLeftTriangleEqual;": 8940, "NotSquareSubsetEqual;": 8930, "OpenCurlyDoubleQuote;": 8220, "ReverseUpEquilibrium;": 10607, "DoubleLongLeftArrow;": 10232, "DownLeftRightVector;": 10576, "LeftArrowRightArrow;": 8646, "NegativeMediumSpace;": 8203, "NotGreaterFullEqual;": 8807, "NotRightTriangleBar;": 10704, "RightArrowLeftArrow;": 8644, "SquareSupersetEqual;": 8850, "leftrightsquigarrow;": 8621, "DownRightTeeVector;": 10591, "DownRightVectorBar;": 10583, "LongLeftRightArrow;": 10231, "Longleftrightarrow;": 10234, "NegativeThickSpace;": 8203, "NotLeftTriangleBar;": 10703, "PrecedesSlantEqual;": 8828, "ReverseEquilibrium;": 8651, "RightDoubleBracket;": 10215, "RightDownTeeVector;": 10589, "RightDownVectorBar;": 10581, "RightTriangleEqual;": 8885, "SquareIntersection;": 8851, "SucceedsSlantEqual;": 8829, "blacktriangleright;": 9656, "longleftrightarrow;": 10231, "DoubleUpDownArrow;": 8661, "DoubleVerticalBar;": 8741, "DownLeftTeeVector;": 10590, "DownLeftVectorBar;": 10582, "FilledSmallSquare;": 9724, "GreaterSlantEqual;": 10878, "LeftDoubleBracket;": 10214, "LeftDownTeeVector;": 10593, "LeftDownVectorBar;": 10585, "LeftTriangleEqual;": 8884, "NegativeThinSpace;": 8203, "NotGreaterGreater;": 8811, "NotLessSlantEqual;": 10877, "NotNestedLessLess;": 10913, "NotReverseElement;": 8716, "NotSquareSuperset;": 8848, "NotTildeFullEqual;": 8775, "RightAngleBracket;": 10217, "RightUpDownVector;": 10575, "SquareSubsetEqual;": 8849, "VerticalSeparator;": 10072, "blacktriangledown;": 9662, "blacktriangleleft;": 9666, "leftrightharpoons;": 8651, "rightleftharpoons;": 8652, "twoheadrightarrow;": 8608, "DiacriticalAcute;": 180, "DiacriticalGrave;": 96, "DiacriticalTilde;": 732, "DoubleRightArrow;": 8658, "DownArrowUpArrow;": 8693, "EmptySmallSquare;": 9723, "GreaterEqualLess;": 8923, "GreaterFullEqual;": 8807, "LeftAngleBracket;": 10216, "LeftUpDownVector;": 10577, "LessEqualGreater;": 8922, "NonBreakingSpace;": 160, "NotPrecedesEqual;": 10927, "NotRightTriangle;": 8939, "NotSucceedsEqual;": 10928, "NotSucceedsTilde;": 8831, "NotSupersetEqual;": 8841, "RightTriangleBar;": 10704, "RightUpTeeVector;": 10588, "RightUpVectorBar;": 10580, "UnderParenthesis;": 9181, "UpArrowDownArrow;": 8645, "circlearrowright;": 8635, "downharpoonright;": 8642, "ntrianglerighteq;": 8941, "rightharpoondown;": 8641, "rightrightarrows;": 8649, "twoheadleftarrow;": 8606, "vartriangleright;": 8883, "CloseCurlyQuote;": 8217, "ContourIntegral;": 8750, "DoubleDownArrow;": 8659, "DoubleLeftArrow;": 8656, "DownRightVector;": 8641, "LeftRightVector;": 10574, "LeftTriangleBar;": 10703, "LeftUpTeeVector;": 10592, "LeftUpVectorBar;": 10584, "LowerRightArrow;": 8600, "NotGreaterEqual;": 8817, "NotGreaterTilde;": 8821, "NotHumpDownHump;": 8782, "NotLeftTriangle;": 8938, "NotSquareSubset;": 8847, "OverParenthesis;": 9180, "RightDownVector;": 8642, "ShortRightArrow;": 8594, "UpperRightArrow;": 8599, "bigtriangledown;": 9661, "circlearrowleft;": 8634, "curvearrowright;": 8631, "downharpoonleft;": 8643, "leftharpoondown;": 8637, "leftrightarrows;": 8646, "nLeftrightarrow;": 8654, "nleftrightarrow;": 8622, "ntrianglelefteq;": 8940, "rightleftarrows;": 8644, "rightsquigarrow;": 8605, "rightthreetimes;": 8908, "straightepsilon;": 1013, "trianglerighteq;": 8885, "vartriangleleft;": 8882, "DiacriticalDot;": 729, "DoubleRightTee;": 8872, "DownLeftVector;": 8637, "GreaterGreater;": 10914, "HorizontalLine;": 9472, "InvisibleComma;": 8291, "InvisibleTimes;": 8290, "LeftDownVector;": 8643, "LeftRightArrow;": 8596, "Leftrightarrow;": 8660, "LessSlantEqual;": 10877, "LongRightArrow;": 10230, "Longrightarrow;": 10233, "LowerLeftArrow;": 8601, "NestedLessLess;": 8810, "NotGreaterLess;": 8825, "NotLessGreater;": 8824, "NotSubsetEqual;": 8840, "NotVerticalBar;": 8740, "OpenCurlyQuote;": 8216, "ReverseElement;": 8715, "RightTeeVector;": 10587, "RightVectorBar;": 10579, "ShortDownArrow;": 8595, "ShortLeftArrow;": 8592, "SquareSuperset;": 8848, "TildeFullEqual;": 8773, "UpperLeftArrow;": 8598, "ZeroWidthSpace;": 8203, "curvearrowleft;": 8630, "doublebarwedge;": 8966, "downdownarrows;": 8650, "hookrightarrow;": 8618, "leftleftarrows;": 8647, "leftrightarrow;": 8596, "leftthreetimes;": 8907, "longrightarrow;": 10230, "looparrowright;": 8620, "nshortparallel;": 8742, "ntriangleright;": 8939, "rightarrowtail;": 8611, "rightharpoonup;": 8640, "trianglelefteq;": 8884, "upharpoonright;": 8638, "ApplyFunction;": 8289, "DifferentialD;": 8518, "DoubleLeftTee;": 10980, "DoubleUpArrow;": 8657, "LeftTeeVector;": 10586, "LeftVectorBar;": 10578, "LessFullEqual;": 8806, "LongLeftArrow;": 10229, "Longleftarrow;": 10232, "NotEqualTilde;": 8770, "NotTildeEqual;": 8772, "NotTildeTilde;": 8777, "Poincareplane;": 8460, "PrecedesEqual;": 10927, "PrecedesTilde;": 8830, "RightArrowBar;": 8677, "RightTeeArrow;": 8614, "RightTriangle;": 8883, "RightUpVector;": 8638, "SucceedsEqual;": 10928, "SucceedsTilde;": 8831, "SupersetEqual;": 8839, "UpEquilibrium;": 10606, "VerticalTilde;": 8768, "VeryThinSpace;": 8202, "bigtriangleup;": 9651, "blacktriangle;": 9652, "divideontimes;": 8903, "fallingdotseq;": 8786, "hookleftarrow;": 8617, "leftarrowtail;": 8610, "leftharpoonup;": 8636, "longleftarrow;": 10229, "looparrowleft;": 8619, "measuredangle;": 8737, "ntriangleleft;": 8938, "shortparallel;": 8741, "smallsetminus;": 8726, "triangleright;": 9657, "upharpoonleft;": 8639, "varsubsetneqq;": 10955, "varsupsetneqq;": 10956, "DownArrowBar;": 10515, "DownTeeArrow;": 8615, "ExponentialE;": 8519, "GreaterEqual;": 8805, "GreaterTilde;": 8819, "HilbertSpace;": 8459, "HumpDownHump;": 8782, "Intersection;": 8898, "LeftArrowBar;": 8676, "LeftTeeArrow;": 8612, "LeftTriangle;": 8882, "LeftUpVector;": 8639, "NotCongruent;": 8802, "NotHumpEqual;": 8783, "NotLessEqual;": 8816, "NotLessTilde;": 8820, "Proportional;": 8733, "RightCeiling;": 8969, "RoundImplies;": 10608, "ShortUpArrow;": 8593, "SquareSubset;": 8847, "UnderBracket;": 9141, "VerticalLine;": 124, "blacklozenge;": 10731, "exponentiale;": 8519, "risingdotseq;": 8787, "triangledown;": 9663, "triangleleft;": 9667, "varsubsetneq;": 8842, "varsupsetneq;": 8843, "CircleMinus;": 8854, "CircleTimes;": 8855, "Equilibrium;": 8652, "GreaterLess;": 8823, "LeftCeiling;": 8968, "LessGreater;": 8822, "MediumSpace;": 8287, "NotLessLess;": 8810, "NotPrecedes;": 8832, "NotSucceeds;": 8833, "NotSuperset;": 8835, "OverBracket;": 9140, "RightVector;": 8640, "Rrightarrow;": 8667, "RuleDelayed;": 10740, "SmallCircle;": 8728, "SquareUnion;": 8852, "SubsetEqual;": 8838, "UpDownArrow;": 8597, "Updownarrow;": 8661, "VerticalBar;": 8739, "backepsilon;": 1014, "blacksquare;": 9642, "circledcirc;": 8858, "circleddash;": 8861, "curlyeqprec;": 8926, "curlyeqsucc;": 8927, "diamondsuit;": 9830, "eqslantless;": 10901, "expectation;": 8496, "nRightarrow;": 8655, "nrightarrow;": 8603, "preccurlyeq;": 8828, "precnapprox;": 10937, "quaternions;": 8461, "straightphi;": 981, "succcurlyeq;": 8829, "succnapprox;": 10938, "thickapprox;": 8776, "updownarrow;": 8597, "Bernoullis;": 8492, "CirclePlus;": 8853, "EqualTilde;": 8770, "Fouriertrf;": 8497, "ImaginaryI;": 8520, "Laplacetrf;": 8466, "LeftVector;": 8636, "Lleftarrow;": 8666, "NotElement;": 8713, "NotGreater;": 8815, "Proportion;": 8759, "RightArrow;": 8594, "RightFloor;": 8971, "Rightarrow;": 8658, "ThickSpace;": 8287, "TildeEqual;": 8771, "TildeTilde;": 8776, "UnderBrace;": 9183, "UpArrowBar;": 10514, "UpTeeArrow;": 8613, "circledast;": 8859, "complement;": 8705, "curlywedge;": 8911, "eqslantgtr;": 10902, "gtreqqless;": 10892, "lessapprox;": 10885, "lesseqqgtr;": 10891, "lmoustache;": 9136, "longmapsto;": 10236, "mapstodown;": 8615, "mapstoleft;": 8612, "nLeftarrow;": 8653, "nleftarrow;": 8602, "nsubseteqq;": 10949, "nsupseteqq;": 10950, "precapprox;": 10935, "rightarrow;": 8594, "rmoustache;": 9137, "sqsubseteq;": 8849, "sqsupseteq;": 8850, "subsetneqq;": 10955, "succapprox;": 10936, "supsetneqq;": 10956, "upuparrows;": 8648, "varepsilon;": 1013, "varnothing;": 8709, "Backslash;": 8726, "CenterDot;": 183, "CircleDot;": 8857, "Congruent;": 8801, "Coproduct;": 8720, "DoubleDot;": 168, "DownArrow;": 8595, "DownBreve;": 785, "Downarrow;": 8659, "HumpEqual;": 8783, "LeftArrow;": 8592, "LeftFloor;": 8970, "Leftarrow;": 8656, "LessTilde;": 8818, "Mellintrf;": 8499, "MinusPlus;": 8723, "NotCupCap;": 8813, "NotExists;": 8708, "NotSubset;": 8834, "OverBrace;": 9182, "PlusMinus;": 177, "Therefore;": 8756, "ThinSpace;": 8201, "TripleDot;": 8411, "UnionPlus;": 8846, "backprime;": 8245, "backsimeq;": 8909, "bigotimes;": 10754, "centerdot;": 183, "checkmark;": 10003, "complexes;": 8450, "dotsquare;": 8865, "downarrow;": 8595, "gtrapprox;": 10886, "gtreqless;": 8923, "gvertneqq;": 8809, "heartsuit;": 9829, "leftarrow;": 8592, "lesseqgtr;": 8922, "lvertneqq;": 8808, "ngeqslant;": 10878, "nleqslant;": 10877, "nparallel;": 8742, "nshortmid;": 8740, "nsubseteq;": 8840, "nsupseteq;": 8841, "pitchfork;": 8916, "rationals;": 8474, "spadesuit;": 9824, "subseteqq;": 10949, "subsetneq;": 8842, "supseteqq;": 10950, "supsetneq;": 8843, "therefore;": 8756, "triangleq;": 8796, "varpropto;": 8733, "DDotrahd;": 10513, "DotEqual;": 8784, "Integral;": 8747, "LessLess;": 10913, "NotEqual;": 8800, "NotTilde;": 8769, "PartialD;": 8706, "Precedes;": 8826, "RightTee;": 8866, "Succeeds;": 8827, "SuchThat;": 8715, "Superset;": 8835, "Uarrocir;": 10569, "UnderBar;": 95, "andslope;": 10840, "angmsdaa;": 10664, "angmsdab;": 10665, "angmsdac;": 10666, "angmsdad;": 10667, "angmsdae;": 10668, "angmsdaf;": 10669, "angmsdag;": 10670, "angmsdah;": 10671, "angrtvbd;": 10653, "approxeq;": 8778, "awconint;": 8755, "backcong;": 8780, "barwedge;": 8965, "bbrktbrk;": 9142, "bigoplus;": 10753, "bigsqcup;": 10758, "biguplus;": 10756, "bigwedge;": 8896, "boxminus;": 8863, "boxtimes;": 8864, "bsolhsub;": 10184, "capbrcup;": 10825, "circledR;": 174, "circledS;": 9416, "cirfnint;": 10768, "clubsuit;": 9827, "cupbrcap;": 10824, "curlyvee;": 8910, "cwconint;": 8754, "doteqdot;": 8785, "dotminus;": 8760, "drbkarow;": 10512, "dzigrarr;": 10239, "elinters;": 9191, "emptyset;": 8709, "eqvparsl;": 10725, "fpartint;": 10765, "geqslant;": 10878, "gesdotol;": 10884, "gnapprox;": 10890, "hksearow;": 10533, "hkswarow;": 10534, "imagline;": 8464, "imagpart;": 8465, "infintie;": 10717, "integers;": 8484, "intercal;": 8890, "intlarhk;": 10775, "laemptyv;": 10676, "ldrushar;": 10571, "leqslant;": 10877, "lesdotor;": 10883, "llcorner;": 8990, "lnapprox;": 10889, "lrcorner;": 8991, "lurdshar;": 10570, "mapstoup;": 8613, "multimap;": 8888, "naturals;": 8469, "ncongdot;": 10861, "notindot;": 8949, "otimesas;": 10806, "parallel;": 8741, "plusacir;": 10787, "pointint;": 10773, "precneqq;": 10933, "precnsim;": 8936, "profalar;": 9006, "profline;": 8978, "profsurf;": 8979, "raemptyv;": 10675, "realpart;": 8476, "rppolint;": 10770, "rtriltri;": 10702, "scpolint;": 10771, "setminus;": 8726, "shortmid;": 8739, "smeparsl;": 10724, "sqsubset;": 8847, "sqsupset;": 8848, "subseteq;": 8838, "succneqq;": 10934, "succnsim;": 8937, "supseteq;": 8839, "thetasym;": 977, "thicksim;": 8764, "timesbar;": 10801, "triangle;": 9653, "triminus;": 10810, "trpezium;": 9186, "ulcorner;": 8988, "urcorner;": 8989, "varkappa;": 1008, "varsigma;": 962, "vartheta;": 977, "Because;": 8757, "Cayleys;": 8493, "Cconint;": 8752, "Cedilla;": 184, "Diamond;": 8900, "DownTee;": 8868, "Element;": 8712, "Epsilon;": 917, "Implies;": 8658, "LeftTee;": 8867, "NewLine;": 10, "NoBreak;": 8288, "NotLess;": 8814, "Omicron;": 927, "OverBar;": 8254, "Product;": 8719, "UpArrow;": 8593, "Uparrow;": 8657, "Upsilon;": 933, "alefsym;": 8501, "angrtvb;": 8894, "angzarr;": 9084, "asympeq;": 8781, "backsim;": 8765, "because;": 8757, "bemptyv;": 10672, "between;": 8812, "bigcirc;": 9711, "bigodot;": 10752, "bigstar;": 9733, "bnequiv;": 8801, "boxplus;": 8862, "ccupssm;": 10832, "cemptyv;": 10674, "cirscir;": 10690, "coloneq;": 8788, "congdot;": 10861, "cudarrl;": 10552, "cudarrr;": 10549, "cularrp;": 10557, "curarrm;": 10556, "dbkarow;": 10511, "ddagger;": 8225, "ddotseq;": 10871, "demptyv;": 10673, "diamond;": 8900, "digamma;": 989, "dotplus;": 8724, "dwangle;": 10662, "epsilon;": 949, "eqcolon;": 8789, "equivDD;": 10872, "gesdoto;": 10882, "gtquest;": 10876, "gtrless;": 8823, "harrcir;": 10568, "intprod;": 10812, "isindot;": 8949, "larrbfs;": 10527, "larrsim;": 10611, "lbrksld;": 10639, "lbrkslu;": 10637, "ldrdhar;": 10599, "lesdoto;": 10881, "lessdot;": 8918, "lessgtr;": 8822, "lesssim;": 8818, "lotimes;": 10804, "lozenge;": 9674, "ltquest;": 10875, "luruhar;": 10598, "maltese;": 10016, "minusdu;": 10794, "napprox;": 8777, "natural;": 9838, "nearrow;": 8599, "nexists;": 8708, "notinva;": 8713, "notinvb;": 8951, "notinvc;": 8950, "notniva;": 8716, "notnivb;": 8958, "notnivc;": 8957, "npolint;": 10772, "npreceq;": 10927, "nsqsube;": 8930, "nsqsupe;": 8931, "nsubset;": 8834, "nsucceq;": 10928, "nsupset;": 8835, "nvinfin;": 10718, "nvltrie;": 8884, "nvrtrie;": 8885, "nwarrow;": 8598, "olcross;": 10683, "omicron;": 959, "orderof;": 8500, "orslope;": 10839, "pertenk;": 8241, "planckh;": 8462, "pluscir;": 10786, "plussim;": 10790, "plustwo;": 10791, "precsim;": 8830, "quatint;": 10774, "questeq;": 8799, "rarrbfs;": 10528, "rarrsim;": 10612, "rbrksld;": 10638, "rbrkslu;": 10640, "rdldhar;": 10601, "realine;": 8475, "rotimes;": 10805, "ruluhar;": 10600, "searrow;": 8600, "simplus;": 10788, "simrarr;": 10610, "subedot;": 10947, "submult;": 10945, "subplus;": 10943, "subrarr;": 10617, "succsim;": 8831, "supdsub;": 10968, "supedot;": 10948, "suphsol;": 10185, "suphsub;": 10967, "suplarr;": 10619, "supmult;": 10946, "supplus;": 10944, "swarrow;": 8601, "topfork;": 10970, "triplus;": 10809, "tritime;": 10811, "uparrow;": 8593, "upsilon;": 965, "uwangle;": 10663, "vzigzag;": 10650, "zigrarr;": 8669, "Aacute;": 193, "Abreve;": 258, "Agrave;": 192, "Assign;": 8788, "Atilde;": 195, "Barwed;": 8966, "Bumpeq;": 8782, "Cacute;": 262, "Ccaron;": 268, "Ccedil;": 199, "Colone;": 10868, "Conint;": 8751, "CupCap;": 8781, "Dagger;": 8225, "Dcaron;": 270, "DotDot;": 8412, "Dstrok;": 272, "Eacute;": 201, "Ecaron;": 282, "Egrave;": 200, "Exists;": 8707, "ForAll;": 8704, "Gammad;": 988, "Gbreve;": 286, "Gcedil;": 290, "HARDcy;": 1066, "Hstrok;": 294, "Iacute;": 205, "Igrave;": 204, "Itilde;": 296, "Jsercy;": 1032, "Kcedil;": 310, "Lacute;": 313, "Lambda;": 923, "Lcaron;": 317, "Lcedil;": 315, "Lmidot;": 319, "Lstrok;": 321, "Nacute;": 323, "Ncaron;": 327, "Ncedil;": 325, "Ntilde;": 209, "Oacute;": 211, "Odblac;": 336, "Ograve;": 210, "Oslash;": 216, "Otilde;": 213, "Otimes;": 10807, "Racute;": 340, "Rarrtl;": 10518, "Rcaron;": 344, "Rcedil;": 342, "SHCHcy;": 1065, "SOFTcy;": 1068, "Sacute;": 346, "Scaron;": 352, "Scedil;": 350, "Square;": 9633, "Subset;": 8912, "Supset;": 8913, "Tcaron;": 356, "Tcedil;": 354, "Tstrok;": 358, "Uacute;": 218, "Ubreve;": 364, "Udblac;": 368, "Ugrave;": 217, "Utilde;": 360, "Vdashl;": 10982, "Verbar;": 8214, "Vvdash;": 8874, "Yacute;": 221, "Zacute;": 377, "Zcaron;": 381, "aacute;": 225, "abreve;": 259, "agrave;": 224, "andand;": 10837, "angmsd;": 8737, "angsph;": 8738, "apacir;": 10863, "approx;": 8776, "atilde;": 227, "barvee;": 8893, "barwed;": 8965, "becaus;": 8757, "bernou;": 8492, "bigcap;": 8898, "bigcup;": 8899, "bigvee;": 8897, "bkarow;": 10509, "bottom;": 8869, "bowtie;": 8904, "boxbox;": 10697, "bprime;": 8245, "brvbar;": 166, "bullet;": 8226, "bumpeq;": 8783, "cacute;": 263, "capand;": 10820, "capcap;": 10827, "capcup;": 10823, "capdot;": 10816, "ccaron;": 269, "ccedil;": 231, "circeq;": 8791, "cirmid;": 10991, "colone;": 8788, "commat;": 64, "compfn;": 8728, "conint;": 8750, "coprod;": 8720, "copysr;": 8471, "cularr;": 8630, "cupcap;": 10822, "cupcup;": 10826, "cupdot;": 8845, "curarr;": 8631, "curren;": 164, "cylcty;": 9005, "dagger;": 8224, "daleth;": 8504, "dcaron;": 271, "dfisht;": 10623, "divide;": 247, "divonx;": 8903, "dlcorn;": 8990, "dlcrop;": 8973, "dollar;": 36, "drcorn;": 8991, "drcrop;": 8972, "dstrok;": 273, "eacute;": 233, "easter;": 10862, "ecaron;": 283, "ecolon;": 8789, "egrave;": 232, "egsdot;": 10904, "elsdot;": 10903, "emptyv;": 8709, "emsp13;": 8196, "emsp14;": 8197, "eparsl;": 10723, "eqcirc;": 8790, "equals;": 61, "equest;": 8799, "female;": 9792, "ffilig;": 64259, "ffllig;": 64260, "forall;": 8704, "frac12;": 189, "frac13;": 8531, "frac14;": 188, "frac15;": 8533, "frac16;": 8537, "frac18;": 8539, "frac23;": 8532, "frac25;": 8534, "frac34;": 190, "frac35;": 8535, "frac38;": 8540, "frac45;": 8536, "frac56;": 8538, "frac58;": 8541, "frac78;": 8542, "gacute;": 501, "gammad;": 989, "gbreve;": 287, "gesdot;": 10880, "gesles;": 10900, "gtlPar;": 10645, "gtrarr;": 10616, "gtrdot;": 8919, "gtrsim;": 8819, "hairsp;": 8202, "hamilt;": 8459, "hardcy;": 1098, "hearts;": 9829, "hellip;": 8230, "hercon;": 8889, "homtht;": 8763, "horbar;": 8213, "hslash;": 8463, "hstrok;": 295, "hybull;": 8259, "hyphen;": 8208, "iacute;": 237, "igrave;": 236, "iiiint;": 10764, "iinfin;": 10716, "incare;": 8453, "inodot;": 305, "intcal;": 8890, "iquest;": 191, "isinsv;": 8947, "itilde;": 297, "jsercy;": 1112, "kappav;": 1008, "kcedil;": 311, "kgreen;": 312, "lAtail;": 10523, "lacute;": 314, "lagran;": 8466, "lambda;": 955, "langle;": 10216, "larrfs;": 10525, "larrhk;": 8617, "larrlp;": 8619, "larrpl;": 10553, "larrtl;": 8610, "latail;": 10521, "lbrace;": 123, "lbrack;": 91, "lcaron;": 318, "lcedil;": 316, "ldquor;": 8222, "lesdot;": 10879, "lesges;": 10899, "lfisht;": 10620, "lfloor;": 8970, "lharul;": 10602, "llhard;": 10603, "lmidot;": 320, "lmoust;": 9136, "loplus;": 10797, "lowast;": 8727, "lowbar;": 95, "lparlt;": 10643, "lrhard;": 10605, "lsaquo;": 8249, "lsquor;": 8218, "lstrok;": 322, "lthree;": 8907, "ltimes;": 8905, "ltlarr;": 10614, "ltrPar;": 10646, "mapsto;": 8614, "marker;": 9646, "mcomma;": 10793, "midast;": 42, "midcir;": 10992, "middot;": 183, "minusb;": 8863, "minusd;": 8760, "mnplus;": 8723, "models;": 8871, "mstpos;": 8766, "nVDash;": 8879, "nVdash;": 8878, "nacute;": 324, "nbumpe;": 8783, "ncaron;": 328, "ncedil;": 326, "nearhk;": 10532, "nequiv;": 8802, "nesear;": 10536, "nexist;": 8708, "nltrie;": 8940, "notinE;": 8953, "nparsl;": 11005, "nprcue;": 8928, "nrarrc;": 10547, "nrarrw;": 8605, "nrtrie;": 8941, "nsccue;": 8929, "nsimeq;": 8772, "ntilde;": 241, "numero;": 8470, "nvDash;": 8877, "nvHarr;": 10500, "nvdash;": 8876, "nvlArr;": 10498, "nvrArr;": 10499, "nwarhk;": 10531, "nwnear;": 10535, "oacute;": 243, "odblac;": 337, "odsold;": 10684, "ograve;": 242, "ominus;": 8854, "origof;": 8886, "oslash;": 248, "otilde;": 245, "otimes;": 8855, "parsim;": 10995, "percnt;": 37, "period;": 46, "permil;": 8240, "phmmat;": 8499, "planck;": 8463, "plankv;": 8463, "plusdo;": 8724, "plusdu;": 10789, "plusmn;": 177, "preceq;": 10927, "primes;": 8473, "prnsim;": 8936, "propto;": 8733, "prurel;": 8880, "puncsp;": 8200, "qprime;": 8279, "rAtail;": 10524, "racute;": 341, "rangle;": 10217, "rarrap;": 10613, "rarrfs;": 10526, "rarrhk;": 8618, "rarrlp;": 8620, "rarrpl;": 10565, "rarrtl;": 8611, "ratail;": 10522, "rbrace;": 125, "rbrack;": 93, "rcaron;": 345, "rcedil;": 343, "rdquor;": 8221, "rfisht;": 10621, "rfloor;": 8971, "rharul;": 10604, "rmoust;": 9137, "roplus;": 10798, "rpargt;": 10644, "rsaquo;": 8250, "rsquor;": 8217, "rthree;": 8908, "rtimes;": 8906, "sacute;": 347, "scaron;": 353, "scedil;": 351, "scnsim;": 8937, "searhk;": 10533, "seswar;": 10537, "sfrown;": 8994, "shchcy;": 1097, "sigmaf;": 962, "sigmav;": 962, "simdot;": 10858, "smashp;": 10803, "softcy;": 1100, "solbar;": 9023, "spades;": 9824, "sqcaps;": 8851, "sqcups;": 8852, "sqsube;": 8849, "sqsupe;": 8850, "square;": 9633, "squarf;": 9642, "ssetmn;": 8726, "ssmile;": 8995, "sstarf;": 8902, "subdot;": 10941, "subset;": 8834, "subsim;": 10951, "subsub;": 10965, "subsup;": 10963, "succeq;": 10928, "supdot;": 10942, "supset;": 8835, "supsim;": 10952, "supsub;": 10964, "supsup;": 10966, "swarhk;": 10534, "swnwar;": 10538, "target;": 8982, "tcaron;": 357, "tcedil;": 355, "telrec;": 8981, "there4;": 8756, "thetav;": 977, "thinsp;": 8201, "thksim;": 8764, "timesb;": 8864, "timesd;": 10800, "topbot;": 9014, "topcir;": 10993, "tprime;": 8244, "tridot;": 9708, "tstrok;": 359, "uacute;": 250, "ubreve;": 365, "udblac;": 369, "ufisht;": 10622, "ugrave;": 249, "ulcorn;": 8988, "ulcrop;": 8975, "urcorn;": 8989, "urcrop;": 8974, "utilde;": 361, "vangrt;": 10652, "varphi;": 981, "varrho;": 1009, "veebar;": 8891, "vellip;": 8942, "verbar;": 124, "vsubnE;": 10955, "vsubne;": 8842, "vsupnE;": 10956, "vsupne;": 8843, "wedbar;": 10847, "wedgeq;": 8793, "weierp;": 8472, "wreath;": 8768, "xoplus;": 10753, "xotime;": 10754, "xsqcup;": 10758, "xuplus;": 10756, "xwedge;": 8896, "yacute;": 253, "zacute;": 378, "zcaron;": 382, "zeetrf;": 8488, "AElig;": 198, Aacute: 193, "Acirc;": 194, Agrave: 192, "Alpha;": 913, "Amacr;": 256, "Aogon;": 260, "Aring;": 197, Atilde: 195, "Breve;": 728, Ccedil: 199, "Ccirc;": 264, "Colon;": 8759, "Cross;": 10799, "Dashv;": 10980, "Delta;": 916, Eacute: 201, "Ecirc;": 202, Egrave: 200, "Emacr;": 274, "Eogon;": 280, "Equal;": 10869, "Gamma;": 915, "Gcirc;": 284, "Hacek;": 711, "Hcirc;": 292, "IJlig;": 306, Iacute: 205, "Icirc;": 206, Igrave: 204, "Imacr;": 298, "Iogon;": 302, "Iukcy;": 1030, "Jcirc;": 308, "Jukcy;": 1028, "Kappa;": 922, Ntilde: 209, "OElig;": 338, Oacute: 211, "Ocirc;": 212, Ograve: 210, "Omacr;": 332, "Omega;": 937, Oslash: 216, Otilde: 213, "Prime;": 8243, "RBarr;": 10512, "Scirc;": 348, "Sigma;": 931, "THORN;": 222, "TRADE;": 8482, "TSHcy;": 1035, "Theta;": 920, "Tilde;": 8764, Uacute: 218, "Ubrcy;": 1038, "Ucirc;": 219, Ugrave: 217, "Umacr;": 362, "Union;": 8899, "Uogon;": 370, "UpTee;": 8869, "Uring;": 366, "VDash;": 8875, "Vdash;": 8873, "Wcirc;": 372, "Wedge;": 8896, Yacute: 221, "Ycirc;": 374, aacute: 225, "acirc;": 226, "acute;": 180, "aelig;": 230, agrave: 224, "aleph;": 8501, "alpha;": 945, "amacr;": 257, "amalg;": 10815, "angle;": 8736, "angrt;": 8735, "angst;": 197, "aogon;": 261, "aring;": 229, "asymp;": 8776, atilde: 227, "awint;": 10769, "bcong;": 8780, "bdquo;": 8222, "bepsi;": 1014, "blank;": 9251, "blk12;": 9618, "blk14;": 9617, "blk34;": 9619, "block;": 9608, "boxDL;": 9559, "boxDR;": 9556, "boxDl;": 9558, "boxDr;": 9555, "boxHD;": 9574, "boxHU;": 9577, "boxHd;": 9572, "boxHu;": 9575, "boxUL;": 9565, "boxUR;": 9562, "boxUl;": 9564, "boxUr;": 9561, "boxVH;": 9580, "boxVL;": 9571, "boxVR;": 9568, "boxVh;": 9579, "boxVl;": 9570, "boxVr;": 9567, "boxdL;": 9557, "boxdR;": 9554, "boxdl;": 9488, "boxdr;": 9484, "boxhD;": 9573, "boxhU;": 9576, "boxhd;": 9516, "boxhu;": 9524, "boxuL;": 9563, "boxuR;": 9560, "boxul;": 9496, "boxur;": 9492, "boxvH;": 9578, "boxvL;": 9569, "boxvR;": 9566, "boxvh;": 9532, "boxvl;": 9508, "boxvr;": 9500, "breve;": 728, brvbar: 166, "bsemi;": 8271, "bsime;": 8909, "bsolb;": 10693, "bumpE;": 10926, "bumpe;": 8783, "caret;": 8257, "caron;": 711, "ccaps;": 10829, ccedil: 231, "ccirc;": 265, "ccups;": 10828, "cedil;": 184, "check;": 10003, "clubs;": 9827, "colon;": 58, "comma;": 44, "crarr;": 8629, "cross;": 10007, "csube;": 10961, "csupe;": 10962, "ctdot;": 8943, "cuepr;": 8926, "cuesc;": 8927, "cupor;": 10821, curren: 164, "cuvee;": 8910, "cuwed;": 8911, "cwint;": 8753, "dashv;": 8867, "dblac;": 733, "ddarr;": 8650, "delta;": 948, "dharl;": 8643, "dharr;": 8642, "diams;": 9830, "disin;": 8946, divide: 247, "doteq;": 8784, "dtdot;": 8945, "dtrif;": 9662, "duarr;": 8693, "duhar;": 10607, "eDDot;": 10871, eacute: 233, "ecirc;": 234, "efDot;": 8786, egrave: 232, "emacr;": 275, "empty;": 8709, "eogon;": 281, "eplus;": 10865, "epsiv;": 1013, "eqsim;": 8770, "equiv;": 8801, "erDot;": 8787, "erarr;": 10609, "esdot;": 8784, "exist;": 8707, "fflig;": 64256, "filig;": 64257, "fjlig;": 102, "fllig;": 64258, "fltns;": 9649, "forkv;": 10969, frac12: 189, frac14: 188, frac34: 190, "frasl;": 8260, "frown;": 8994, "gamma;": 947, "gcirc;": 285, "gescc;": 10921, "gimel;": 8503, "gneqq;": 8809, "gnsim;": 8935, "grave;": 96, "gsime;": 10894, "gsiml;": 10896, "gtcir;": 10874, "gtdot;": 8919, "harrw;": 8621, "hcirc;": 293, "hoarr;": 8703, iacute: 237, "icirc;": 238, "iexcl;": 161, igrave: 236, "iiint;": 8749, "iiota;": 8489, "ijlig;": 307, "imacr;": 299, "image;": 8465, "imath;": 305, "imped;": 437, "infin;": 8734, "iogon;": 303, "iprod;": 10812, iquest: 191, "isinE;": 8953, "isins;": 8948, "isinv;": 8712, "iukcy;": 1110, "jcirc;": 309, "jmath;": 567, "jukcy;": 1108, "kappa;": 954, "lAarr;": 8666, "lBarr;": 10510, "langd;": 10641, "laquo;": 171, "larrb;": 8676, "lates;": 10925, "lbarr;": 10508, "lbbrk;": 10098, "lbrke;": 10635, "lceil;": 8968, "ldquo;": 8220, "lescc;": 10920, "lhard;": 8637, "lharu;": 8636, "lhblk;": 9604, "llarr;": 8647, "lltri;": 9722, "lneqq;": 8808, "lnsim;": 8934, "loang;": 10220, "loarr;": 8701, "lobrk;": 10214, "lopar;": 10629, "lrarr;": 8646, "lrhar;": 8651, "lrtri;": 8895, "lsime;": 10893, "lsimg;": 10895, "lsquo;": 8216, "ltcir;": 10873, "ltdot;": 8918, "ltrie;": 8884, "ltrif;": 9666, "mDDot;": 8762, "mdash;": 8212, "micro;": 181, middot: 183, "minus;": 8722, "mumap;": 8888, "nabla;": 8711, "napid;": 8779, "napos;": 329, "natur;": 9838, "nbump;": 8782, "ncong;": 8775, "ndash;": 8211, "neArr;": 8663, "nearr;": 8599, "nedot;": 8784, "nesim;": 8770, "ngeqq;": 8807, "ngsim;": 8821, "nhArr;": 8654, "nharr;": 8622, "nhpar;": 10994, "nlArr;": 8653, "nlarr;": 8602, "nleqq;": 8806, "nless;": 8814, "nlsim;": 8820, "nltri;": 8938, "notin;": 8713, "notni;": 8716, "npart;": 8706, "nprec;": 8832, "nrArr;": 8655, "nrarr;": 8603, "nrtri;": 8939, "nsime;": 8772, "nsmid;": 8740, "nspar;": 8742, "nsubE;": 10949, "nsube;": 8840, "nsucc;": 8833, "nsupE;": 10950, "nsupe;": 8841, ntilde: 241, "numsp;": 8199, "nvsim;": 8764, "nwArr;": 8662, "nwarr;": 8598, oacute: 243, "ocirc;": 244, "odash;": 8861, "oelig;": 339, "ofcir;": 10687, ograve: 242, "ohbar;": 10677, "olarr;": 8634, "olcir;": 10686, "oline;": 8254, "omacr;": 333, "omega;": 969, "operp;": 10681, "oplus;": 8853, "orarr;": 8635, "order;": 8500, oslash: 248, otilde: 245, "ovbar;": 9021, "parsl;": 11005, "phone;": 9742, "plusb;": 8862, "pluse;": 10866, plusmn: 177, "pound;": 163, "prcue;": 8828, "prime;": 8242, "prnap;": 10937, "prsim;": 8830, "quest;": 63, "rAarr;": 8667, "rBarr;": 10511, "radic;": 8730, "rangd;": 10642, "range;": 10661, "raquo;": 187, "rarrb;": 8677, "rarrc;": 10547, "rarrw;": 8605, "ratio;": 8758, "rbarr;": 10509, "rbbrk;": 10099, "rbrke;": 10636, "rceil;": 8969, "rdquo;": 8221, "reals;": 8477, "rhard;": 8641, "rharu;": 8640, "rlarr;": 8644, "rlhar;": 8652, "rnmid;": 10990, "roang;": 10221, "roarr;": 8702, "robrk;": 10215, "ropar;": 10630, "rrarr;": 8649, "rsquo;": 8217, "rtrie;": 8885, "rtrif;": 9656, "sbquo;": 8218, "sccue;": 8829, "scirc;": 349, "scnap;": 10938, "scsim;": 8831, "sdotb;": 8865, "sdote;": 10854, "seArr;": 8664, "searr;": 8600, "setmn;": 8726, "sharp;": 9839, "sigma;": 963, "simeq;": 8771, "simgE;": 10912, "simlE;": 10911, "simne;": 8774, "slarr;": 8592, "smile;": 8995, "smtes;": 10924, "sqcap;": 8851, "sqcup;": 8852, "sqsub;": 8847, "sqsup;": 8848, "srarr;": 8594, "starf;": 9733, "strns;": 175, "subnE;": 10955, "subne;": 8842, "supnE;": 10956, "supne;": 8843, "swArr;": 8665, "swarr;": 8601, "szlig;": 223, "theta;": 952, "thkap;": 8776, "thorn;": 254, "tilde;": 732, "times;": 215, "trade;": 8482, "trisb;": 10701, "tshcy;": 1115, "twixt;": 8812, uacute: 250, "ubrcy;": 1118, "ucirc;": 251, "udarr;": 8645, "udhar;": 10606, ugrave: 249, "uharl;": 8639, "uharr;": 8638, "uhblk;": 9600, "ultri;": 9720, "umacr;": 363, "uogon;": 371, "uplus;": 8846, "upsih;": 978, "uring;": 367, "urtri;": 9721, "utdot;": 8944, "utrif;": 9652, "uuarr;": 8648, "vBarv;": 10985, "vDash;": 8872, "varpi;": 982, "vdash;": 8866, "veeeq;": 8794, "vltri;": 8882, "vnsub;": 8834, "vnsup;": 8835, "vprop;": 8733, "vrtri;": 8883, "wcirc;": 373, "wedge;": 8743, "xcirc;": 9711, "xdtri;": 9661, "xhArr;": 10234, "xharr;": 10231, "xlArr;": 10232, "xlarr;": 10229, "xodot;": 10752, "xrArr;": 10233, "xrarr;": 10230, "xutri;": 9651, yacute: 253, "ycirc;": 375, AElig: 198, Acirc: 194, "Aopf;": 120120, Aring: 197, "Ascr;": 119964, "Auml;": 196, "Barv;": 10983, "Beta;": 914, "Bopf;": 120121, "Bscr;": 8492, "CHcy;": 1063, "COPY;": 169, "Cdot;": 266, "Copf;": 8450, "Cscr;": 119966, "DJcy;": 1026, "DScy;": 1029, "DZcy;": 1039, "Darr;": 8609, "Dopf;": 120123, "Dscr;": 119967, Ecirc: 202, "Edot;": 278, "Eopf;": 120124, "Escr;": 8496, "Esim;": 10867, "Euml;": 203, "Fopf;": 120125, "Fscr;": 8497, "GJcy;": 1027, "Gdot;": 288, "Gopf;": 120126, "Gscr;": 119970, "Hopf;": 8461, "Hscr;": 8459, "IEcy;": 1045, "IOcy;": 1025, Icirc: 206, "Idot;": 304, "Iopf;": 120128, "Iota;": 921, "Iscr;": 8464, "Iuml;": 207, "Jopf;": 120129, "Jscr;": 119973, "KHcy;": 1061, "KJcy;": 1036, "Kopf;": 120130, "Kscr;": 119974, "LJcy;": 1033, "Lang;": 10218, "Larr;": 8606, "Lopf;": 120131, "Lscr;": 8466, "Mopf;": 120132, "Mscr;": 8499, "NJcy;": 1034, "Nopf;": 8469, "Nscr;": 119977, Ocirc: 212, "Oopf;": 120134, "Oscr;": 119978, "Ouml;": 214, "Popf;": 8473, "Pscr;": 119979, "QUOT;": 34, "Qopf;": 8474, "Qscr;": 119980, "Rang;": 10219, "Rarr;": 8608, "Ropf;": 8477, "Rscr;": 8475, "SHcy;": 1064, "Sopf;": 120138, "Sqrt;": 8730, "Sscr;": 119982, "Star;": 8902, THORN: 222, "TScy;": 1062, "Topf;": 120139, "Tscr;": 119983, "Uarr;": 8607, Ucirc: 219, "Uopf;": 120140, "Upsi;": 978, "Uscr;": 119984, "Uuml;": 220, "Vbar;": 10987, "Vert;": 8214, "Vopf;": 120141, "Vscr;": 119985, "Wopf;": 120142, "Wscr;": 119986, "Xopf;": 120143, "Xscr;": 119987, "YAcy;": 1071, "YIcy;": 1031, "YUcy;": 1070, "Yopf;": 120144, "Yscr;": 119988, "Yuml;": 376, "ZHcy;": 1046, "Zdot;": 379, "Zeta;": 918, "Zopf;": 8484, "Zscr;": 119989, acirc: 226, acute: 180, aelig: 230, "andd;": 10844, "andv;": 10842, "ange;": 10660, "aopf;": 120146, "apid;": 8779, "apos;": 39, aring: 229, "ascr;": 119990, "auml;": 228, "bNot;": 10989, "bbrk;": 9141, "beta;": 946, "beth;": 8502, "bnot;": 8976, "bopf;": 120147, "boxH;": 9552, "boxV;": 9553, "boxh;": 9472, "boxv;": 9474, "bscr;": 119991, "bsim;": 8765, "bsol;": 92, "bull;": 8226, "bump;": 8782, "caps;": 8745, "cdot;": 267, cedil: 184, "cent;": 162, "chcy;": 1095, "cirE;": 10691, "circ;": 710, "cire;": 8791, "comp;": 8705, "cong;": 8773, "copf;": 120148, "copy;": 169, "cscr;": 119992, "csub;": 10959, "csup;": 10960, "cups;": 8746, "dArr;": 8659, "dHar;": 10597, "darr;": 8595, "dash;": 8208, "diam;": 8900, "djcy;": 1106, "dopf;": 120149, "dscr;": 119993, "dscy;": 1109, "dsol;": 10742, "dtri;": 9663, "dzcy;": 1119, "eDot;": 8785, "ecir;": 8790, ecirc: 234, "edot;": 279, "emsp;": 8195, "ensp;": 8194, "eopf;": 120150, "epar;": 8917, "epsi;": 949, "escr;": 8495, "esim;": 8770, "euml;": 235, "euro;": 8364, "excl;": 33, "flat;": 9837, "fnof;": 402, "fopf;": 120151, "fork;": 8916, "fscr;": 119995, "gdot;": 289, "geqq;": 8807, "gesl;": 8923, "gjcy;": 1107, "gnap;": 10890, "gneq;": 10888, "gopf;": 120152, "gscr;": 8458, "gsim;": 8819, "gtcc;": 10919, "gvnE;": 8809, "hArr;": 8660, "half;": 189, "harr;": 8596, "hbar;": 8463, "hopf;": 120153, "hscr;": 119997, icirc: 238, "iecy;": 1077, iexcl: 161, "imof;": 8887, "iocy;": 1105, "iopf;": 120154, "iota;": 953, "iscr;": 119998, "isin;": 8712, "iuml;": 239, "jopf;": 120155, "jscr;": 119999, "khcy;": 1093, "kjcy;": 1116, "kopf;": 120156, "kscr;": 12e4, "lArr;": 8656, "lHar;": 10594, "lang;": 10216, laquo: 171, "larr;": 8592, "late;": 10925, "lcub;": 123, "ldca;": 10550, "ldsh;": 8626, "leqq;": 8806, "lesg;": 8922, "ljcy;": 1113, "lnap;": 10889, "lneq;": 10887, "lopf;": 120157, "lozf;": 10731, "lpar;": 40, "lscr;": 120001, "lsim;": 8818, "lsqb;": 91, "ltcc;": 10918, "ltri;": 9667, "lvnE;": 8808, "macr;": 175, "male;": 9794, "malt;": 10016, micro: 181, "mlcp;": 10971, "mldr;": 8230, "mopf;": 120158, "mscr;": 120002, "nGtv;": 8811, "nLtv;": 8810, "nang;": 8736, "napE;": 10864, "nbsp;": 160, "ncap;": 10819, "ncup;": 10818, "ngeq;": 8817, "nges;": 10878, "ngtr;": 8815, "nisd;": 8954, "njcy;": 1114, "nldr;": 8229, "nleq;": 8816, "nles;": 10877, "nmid;": 8740, "nopf;": 120159, "npar;": 8742, "npre;": 10927, "nsce;": 10928, "nscr;": 120003, "nsim;": 8769, "nsub;": 8836, "nsup;": 8837, "ntgl;": 8825, "ntlg;": 8824, "nvap;": 8781, "nvge;": 8805, "nvgt;": 62, "nvle;": 8804, "nvlt;": 60, "oast;": 8859, "ocir;": 8858, ocirc: 244, "odiv;": 10808, "odot;": 8857, "ogon;": 731, "oint;": 8750, "omid;": 10678, "oopf;": 120160, "opar;": 10679, "ordf;": 170, "ordm;": 186, "oror;": 10838, "oscr;": 8500, "osol;": 8856, "ouml;": 246, "para;": 182, "part;": 8706, "perp;": 8869, "phiv;": 981, "plus;": 43, "popf;": 120161, pound: 163, "prap;": 10935, "prec;": 8826, "prnE;": 10933, "prod;": 8719, "prop;": 8733, "pscr;": 120005, "qint;": 10764, "qopf;": 120162, "qscr;": 120006, "quot;": 34, "rArr;": 8658, "rHar;": 10596, "race;": 8765, "rang;": 10217, raquo: 187, "rarr;": 8594, "rcub;": 125, "rdca;": 10551, "rdsh;": 8627, "real;": 8476, "rect;": 9645, "rhov;": 1009, "ring;": 730, "ropf;": 120163, "rpar;": 41, "rscr;": 120007, "rsqb;": 93, "rtri;": 9657, "scap;": 10936, "scnE;": 10934, "sdot;": 8901, "sect;": 167, "semi;": 59, "sext;": 10038, "shcy;": 1096, "sime;": 8771, "simg;": 10910, "siml;": 10909, "smid;": 8739, "smte;": 10924, "solb;": 10692, "sopf;": 120164, "spar;": 8741, "squf;": 9642, "sscr;": 120008, "star;": 9734, "subE;": 10949, "sube;": 8838, "succ;": 8827, "sung;": 9834, "sup1;": 185, "sup2;": 178, "sup3;": 179, "supE;": 10950, "supe;": 8839, szlig: 223, "tbrk;": 9140, "tdot;": 8411, thorn: 254, times: 215, "tint;": 8749, "toea;": 10536, "topf;": 120165, "tosa;": 10537, "trie;": 8796, "tscr;": 120009, "tscy;": 1094, "uArr;": 8657, "uHar;": 10595, "uarr;": 8593, ucirc: 251, "uopf;": 120166, "upsi;": 965, "uscr;": 120010, "utri;": 9653, "uuml;": 252, "vArr;": 8661, "vBar;": 10984, "varr;": 8597, "vert;": 124, "vopf;": 120167, "vscr;": 120011, "wopf;": 120168, "wscr;": 120012, "xcap;": 8898, "xcup;": 8899, "xmap;": 10236, "xnis;": 8955, "xopf;": 120169, "xscr;": 120013, "xvee;": 8897, "yacy;": 1103, "yicy;": 1111, "yopf;": 120170, "yscr;": 120014, "yucy;": 1102, "yuml;": 255, "zdot;": 380, "zeta;": 950, "zhcy;": 1078, "zopf;": 120171, "zscr;": 120015, "zwnj;": 8204, "AMP;": 38, "Acy;": 1040, "Afr;": 120068, "And;": 10835, Auml: 196, "Bcy;": 1041, "Bfr;": 120069, COPY: 169, "Cap;": 8914, "Cfr;": 8493, "Chi;": 935, "Cup;": 8915, "Dcy;": 1044, "Del;": 8711, "Dfr;": 120071, "Dot;": 168, "ENG;": 330, "ETH;": 208, "Ecy;": 1069, "Efr;": 120072, "Eta;": 919, Euml: 203, "Fcy;": 1060, "Ffr;": 120073, "Gcy;": 1043, "Gfr;": 120074, "Hat;": 94, "Hfr;": 8460, "Icy;": 1048, "Ifr;": 8465, "Int;": 8748, Iuml: 207, "Jcy;": 1049, "Jfr;": 120077, "Kcy;": 1050, "Kfr;": 120078, "Lcy;": 1051, "Lfr;": 120079, "Lsh;": 8624, "Map;": 10501, "Mcy;": 1052, "Mfr;": 120080, "Ncy;": 1053, "Nfr;": 120081, "Not;": 10988, "Ocy;": 1054, "Ofr;": 120082, Ouml: 214, "Pcy;": 1055, "Pfr;": 120083, "Phi;": 934, "Psi;": 936, QUOT: 34, "Qfr;": 120084, "REG;": 174, "Rcy;": 1056, "Rfr;": 8476, "Rho;": 929, "Rsh;": 8625, "Scy;": 1057, "Sfr;": 120086, "Sub;": 8912, "Sum;": 8721, "Sup;": 8913, "Tab;": 9, "Tau;": 932, "Tcy;": 1058, "Tfr;": 120087, "Ucy;": 1059, "Ufr;": 120088, Uuml: 220, "Vcy;": 1042, "Vee;": 8897, "Vfr;": 120089, "Wfr;": 120090, "Xfr;": 120091, "Ycy;": 1067, "Yfr;": 120092, "Zcy;": 1047, "Zfr;": 8488, "acE;": 8766, "acd;": 8767, "acy;": 1072, "afr;": 120094, "amp;": 38, "and;": 8743, "ang;": 8736, "apE;": 10864, "ape;": 8778, "ast;": 42, auml: 228, "bcy;": 1073, "bfr;": 120095, "bne;": 61, "bot;": 8869, "cap;": 8745, cent: 162, "cfr;": 120096, "chi;": 967, "cir;": 9675, copy: 169, "cup;": 8746, "dcy;": 1076, "deg;": 176, "dfr;": 120097, "die;": 168, "div;": 247, "dot;": 729, "ecy;": 1101, "efr;": 120098, "egs;": 10902, "ell;": 8467, "els;": 10901, "eng;": 331, "eta;": 951, "eth;": 240, euml: 235, "fcy;": 1092, "ffr;": 120099, "gEl;": 10892, "gap;": 10886, "gcy;": 1075, "gel;": 8923, "geq;": 8805, "ges;": 10878, "gfr;": 120100, "ggg;": 8921, "glE;": 10898, "gla;": 10917, "glj;": 10916, "gnE;": 8809, "gne;": 10888, "hfr;": 120101, "icy;": 1080, "iff;": 8660, "ifr;": 120102, "int;": 8747, iuml: 239, "jcy;": 1081, "jfr;": 120103, "kcy;": 1082, "kfr;": 120104, "lEg;": 10891, "lap;": 10885, "lat;": 10923, "lcy;": 1083, "leg;": 8922, "leq;": 8804, "les;": 10877, "lfr;": 120105, "lgE;": 10897, "lnE;": 8808, "lne;": 10887, "loz;": 9674, "lrm;": 8206, "lsh;": 8624, macr: 175, "map;": 8614, "mcy;": 1084, "mfr;": 120106, "mho;": 8487, "mid;": 8739, "nGg;": 8921, "nGt;": 8811, "nLl;": 8920, "nLt;": 8810, "nap;": 8777, nbsp: 160, "ncy;": 1085, "nfr;": 120107, "ngE;": 8807, "nge;": 8817, "ngt;": 8815, "nis;": 8956, "niv;": 8715, "nlE;": 8806, "nle;": 8816, "nlt;": 8814, "not;": 172, "npr;": 8832, "nsc;": 8833, "num;": 35, "ocy;": 1086, "ofr;": 120108, "ogt;": 10689, "ohm;": 937, "olt;": 10688, "ord;": 10845, ordf: 170, ordm: 186, "orv;": 10843, ouml: 246, "par;": 8741, para: 182, "pcy;": 1087, "pfr;": 120109, "phi;": 966, "piv;": 982, "prE;": 10931, "pre;": 10927, "psi;": 968, "qfr;": 120110, quot: 34, "rcy;": 1088, "reg;": 174, "rfr;": 120111, "rho;": 961, "rlm;": 8207, "rsh;": 8625, "scE;": 10932, "sce;": 10928, "scy;": 1089, sect: 167, "sfr;": 120112, "shy;": 173, "sim;": 8764, "smt;": 10922, "sol;": 47, "squ;": 9633, "sub;": 8834, "sum;": 8721, sup1: 185, sup2: 178, sup3: 179, "sup;": 8835, "tau;": 964, "tcy;": 1090, "tfr;": 120113, "top;": 8868, "ucy;": 1091, "ufr;": 120114, "uml;": 168, uuml: 252, "vcy;": 1074, "vee;": 8744, "vfr;": 120115, "wfr;": 120116, "xfr;": 120117, "ycy;": 1099, "yen;": 165, "yfr;": 120118, yuml: 255, "zcy;": 1079, "zfr;": 120119, "zwj;": 8205, AMP: 38, "DD;": 8517, ETH: 208, "GT;": 62, "Gg;": 8921, "Gt;": 8811, "Im;": 8465, "LT;": 60, "Ll;": 8920, "Lt;": 8810, "Mu;": 924, "Nu;": 925, "Or;": 10836, "Pi;": 928, "Pr;": 10939, REG: 174, "Re;": 8476, "Sc;": 10940, "Xi;": 926, "ac;": 8766, "af;": 8289, amp: 38, "ap;": 8776, "dd;": 8518, deg: 176, "ee;": 8519, "eg;": 10906, "el;": 10905, eth: 240, "gE;": 8807, "ge;": 8805, "gg;": 8811, "gl;": 8823, "gt;": 62, "ic;": 8291, "ii;": 8520, "in;": 8712, "it;": 8290, "lE;": 8806, "le;": 8804, "lg;": 8822, "ll;": 8810, "lt;": 60, "mp;": 8723, "mu;": 956, "ne;": 8800, "ni;": 8715, not: 172, "nu;": 957, "oS;": 9416, "or;": 8744, "pi;": 960, "pm;": 177, "pr;": 8826, reg: 174, "rx;": 8478, "sc;": 8827, shy: 173, uml: 168, "wp;": 8472, "wr;": 8768, "xi;": 958, yen: 165, GT: 62, LT: 60, gt: 62, lt: 60 };
function Z1(e, t) {
  return t && !e.endsWith(";") ? `${e}\\b(?!=)` : e;
}
function fm(e) {
  const t = "#(?:x[a-fA-F\\d]+|\\d+)(?:;)?", i = Object.keys(Y1).map((l) => Z1(l, e));
  return new RegExp(`&(${t}|${i.join("|")})`, "g");
}
fm(false);
fm(true);
const e0 = /* @__PURE__ */ new Map([["svelte:head", "SvelteHead"], ["svelte:options", "SvelteOptions"], ["svelte:window", "SvelteWindow"], ["svelte:document", "SvelteDocument"], ["svelte:body", "SvelteBody"]]);
new Map([...e0, ["svelte:element", "SvelteElement"], ["svelte:component", "SvelteComponent"], ["svelte:self", "SvelteSelf"], ["svelte:fragment", "SvelteFragment"], ["svelte:boundary", "SvelteBoundary"]]);
var lt = {}, nr = {}, No = {}, or = {}, eu;
function t0() {
  if (eu) return or;
  eu = 1, Object.defineProperty(or, "__esModule", { value: true }), or.default = void 0;
  function e() {
    var t = this, i = 0, s = { "@@iterator": function() {
      return s;
    }, next: function() {
      if (i < t.length) {
        var p = t[i];
        return i = i + 1, { done: false, value: p };
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
  var e = t(t0());
  function t(l) {
    return l && l.__esModule ? l : { default: l };
  }
  function i(l) {
    "@babel/helpers - typeof";
    return i = typeof Symbol == "function" && typeof Symbol.iterator == "symbol" ? function(p) {
      return typeof p;
    } : function(p) {
      return p && typeof Symbol == "function" && p.constructor === Symbol && p !== Symbol.prototype ? "symbol" : typeof p;
    }, i(l);
  }
  function s(l, p) {
    return typeof Symbol == "function" && i(Symbol.iterator) === "symbol" && Object.defineProperty(l, Symbol.iterator, { value: e.default.bind(p) }), l;
  }
  return No;
}
var ru;
function r0() {
  if (ru) return nr;
  ru = 1, Object.defineProperty(nr, "__esModule", { value: true }), nr.default = void 0;
  var e = t(wo());
  function t(b) {
    return b && b.__esModule ? b : { default: b };
  }
  function i(b, L) {
    return w(b) || m(b, L) || l(b, L) || s();
  }
  function s() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function l(b, L) {
    if (b) {
      if (typeof b == "string") return p(b, L);
      var O = {}.toString.call(b).slice(8, -1);
      return O === "Object" && b.constructor && (O = b.constructor.name), O === "Map" || O === "Set" ? Array.from(b) : O === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(O) ? p(b, L) : void 0;
    }
  }
  function p(b, L) {
    (L == null || L > b.length) && (L = b.length);
    for (var O = 0, B = Array(L); O < L; O++) B[O] = b[O];
    return B;
  }
  function m(b, L) {
    var O = b == null ? null : typeof Symbol < "u" && b[Symbol.iterator] || b["@@iterator"];
    if (O != null) {
      var B, G, F, R, v = [], h = true, g = false;
      try {
        if (F = (O = O.call(b)).next, L === 0) {
          if (Object(O) !== O) return;
          h = false;
        } else for (; !(h = (B = F.call(O)).done) && (v.push(B.value), v.length !== L); h = true) ;
      } catch (S) {
        g = true, G = S;
      } finally {
        try {
          if (!h && O.return != null && (R = O.return(), Object(R) !== R)) return;
        } finally {
          if (g) throw G;
        }
      }
      return v;
    }
  }
  function w(b) {
    if (Array.isArray(b)) return b;
  }
  var _ = [["aria-activedescendant", { type: "id" }], ["aria-atomic", { type: "boolean" }], ["aria-autocomplete", { type: "token", values: ["inline", "list", "both", "none"] }], ["aria-braillelabel", { type: "string" }], ["aria-brailleroledescription", { type: "string" }], ["aria-busy", { type: "boolean" }], ["aria-checked", { type: "tristate" }], ["aria-colcount", { type: "integer" }], ["aria-colindex", { type: "integer" }], ["aria-colspan", { type: "integer" }], ["aria-controls", { type: "idlist" }], ["aria-current", { type: "token", values: ["page", "step", "location", "date", "time", true, false] }], ["aria-describedby", { type: "idlist" }], ["aria-description", { type: "string" }], ["aria-details", { type: "id" }], ["aria-disabled", { type: "boolean" }], ["aria-dropeffect", { type: "tokenlist", values: ["copy", "execute", "link", "move", "none", "popup"] }], ["aria-errormessage", { type: "id" }], ["aria-expanded", { type: "boolean", allowundefined: true }], ["aria-flowto", { type: "idlist" }], ["aria-grabbed", { type: "boolean", allowundefined: true }], ["aria-haspopup", { type: "token", values: [false, true, "menu", "listbox", "tree", "grid", "dialog"] }], ["aria-hidden", { type: "boolean", allowundefined: true }], ["aria-invalid", { type: "token", values: ["grammar", false, "spelling", true] }], ["aria-keyshortcuts", { type: "string" }], ["aria-label", { type: "string" }], ["aria-labelledby", { type: "idlist" }], ["aria-level", { type: "integer" }], ["aria-live", { type: "token", values: ["assertive", "off", "polite"] }], ["aria-modal", { type: "boolean" }], ["aria-multiline", { type: "boolean" }], ["aria-multiselectable", { type: "boolean" }], ["aria-orientation", { type: "token", values: ["vertical", "undefined", "horizontal"] }], ["aria-owns", { type: "idlist" }], ["aria-placeholder", { type: "string" }], ["aria-posinset", { type: "integer" }], ["aria-pressed", { type: "tristate" }], ["aria-readonly", { type: "boolean" }], ["aria-relevant", { type: "tokenlist", values: ["additions", "all", "removals", "text"] }], ["aria-required", { type: "boolean" }], ["aria-roledescription", { type: "string" }], ["aria-rowcount", { type: "integer" }], ["aria-rowindex", { type: "integer" }], ["aria-rowspan", { type: "integer" }], ["aria-selected", { type: "boolean", allowundefined: true }], ["aria-setsize", { type: "integer" }], ["aria-sort", { type: "token", values: ["ascending", "descending", "none", "other"] }], ["aria-valuemax", { type: "number" }], ["aria-valuemin", { type: "number" }], ["aria-valuenow", { type: "number" }], ["aria-valuetext", { type: "string" }]], o = { entries: function() {
    return _;
  }, forEach: function(L) {
    for (var O = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, B = 0, G = _; B < G.length; B++) {
      var F = i(G[B], 2), R = F[0], v = F[1];
      L.call(O, v, R, _);
    }
  }, get: function(L) {
    var O = _.filter(function(B) {
      return B[0] === L;
    })[0];
    return O && O[1];
  }, has: function(L) {
    return !!o.get(L);
  }, keys: function() {
    return _.map(function(L) {
      var O = i(L, 1), B = O[0];
      return B;
    });
  }, values: function() {
    return _.map(function(L) {
      var O = i(L, 2), B = O[1];
      return B;
    });
  } };
  return nr.default = (0, e.default)(o, o.entries()), nr;
}
var lr = {}, au;
function a0() {
  if (au) return lr;
  au = 1, Object.defineProperty(lr, "__esModule", { value: true }), lr.default = void 0;
  var e = t(wo());
  function t(b) {
    return b && b.__esModule ? b : { default: b };
  }
  function i(b, L) {
    return w(b) || m(b, L) || l(b, L) || s();
  }
  function s() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function l(b, L) {
    if (b) {
      if (typeof b == "string") return p(b, L);
      var O = {}.toString.call(b).slice(8, -1);
      return O === "Object" && b.constructor && (O = b.constructor.name), O === "Map" || O === "Set" ? Array.from(b) : O === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(O) ? p(b, L) : void 0;
    }
  }
  function p(b, L) {
    (L == null || L > b.length) && (L = b.length);
    for (var O = 0, B = Array(L); O < L; O++) B[O] = b[O];
    return B;
  }
  function m(b, L) {
    var O = b == null ? null : typeof Symbol < "u" && b[Symbol.iterator] || b["@@iterator"];
    if (O != null) {
      var B, G, F, R, v = [], h = true, g = false;
      try {
        if (F = (O = O.call(b)).next, L === 0) {
          if (Object(O) !== O) return;
          h = false;
        } else for (; !(h = (B = F.call(O)).done) && (v.push(B.value), v.length !== L); h = true) ;
      } catch (S) {
        g = true, G = S;
      } finally {
        try {
          if (!h && O.return != null && (R = O.return(), Object(R) !== R)) return;
        } finally {
          if (g) throw G;
        }
      }
      return v;
    }
  }
  function w(b) {
    if (Array.isArray(b)) return b;
  }
  var _ = [["a", { reserved: false }], ["abbr", { reserved: false }], ["acronym", { reserved: false }], ["address", { reserved: false }], ["applet", { reserved: false }], ["area", { reserved: false }], ["article", { reserved: false }], ["aside", { reserved: false }], ["audio", { reserved: false }], ["b", { reserved: false }], ["base", { reserved: true }], ["bdi", { reserved: false }], ["bdo", { reserved: false }], ["big", { reserved: false }], ["blink", { reserved: false }], ["blockquote", { reserved: false }], ["body", { reserved: false }], ["br", { reserved: false }], ["button", { reserved: false }], ["canvas", { reserved: false }], ["caption", { reserved: false }], ["center", { reserved: false }], ["cite", { reserved: false }], ["code", { reserved: false }], ["col", { reserved: true }], ["colgroup", { reserved: true }], ["content", { reserved: false }], ["data", { reserved: false }], ["datalist", { reserved: false }], ["dd", { reserved: false }], ["del", { reserved: false }], ["details", { reserved: false }], ["dfn", { reserved: false }], ["dialog", { reserved: false }], ["dir", { reserved: false }], ["div", { reserved: false }], ["dl", { reserved: false }], ["dt", { reserved: false }], ["em", { reserved: false }], ["embed", { reserved: false }], ["fieldset", { reserved: false }], ["figcaption", { reserved: false }], ["figure", { reserved: false }], ["font", { reserved: false }], ["footer", { reserved: false }], ["form", { reserved: false }], ["frame", { reserved: false }], ["frameset", { reserved: false }], ["h1", { reserved: false }], ["h2", { reserved: false }], ["h3", { reserved: false }], ["h4", { reserved: false }], ["h5", { reserved: false }], ["h6", { reserved: false }], ["head", { reserved: true }], ["header", { reserved: false }], ["hgroup", { reserved: false }], ["hr", { reserved: false }], ["html", { reserved: true }], ["i", { reserved: false }], ["iframe", { reserved: false }], ["img", { reserved: false }], ["input", { reserved: false }], ["ins", { reserved: false }], ["kbd", { reserved: false }], ["keygen", { reserved: false }], ["label", { reserved: false }], ["legend", { reserved: false }], ["li", { reserved: false }], ["link", { reserved: true }], ["main", { reserved: false }], ["map", { reserved: false }], ["mark", { reserved: false }], ["marquee", { reserved: false }], ["menu", { reserved: false }], ["menuitem", { reserved: false }], ["meta", { reserved: true }], ["meter", { reserved: false }], ["nav", { reserved: false }], ["noembed", { reserved: true }], ["noscript", { reserved: true }], ["object", { reserved: false }], ["ol", { reserved: false }], ["optgroup", { reserved: false }], ["option", { reserved: false }], ["output", { reserved: false }], ["p", { reserved: false }], ["param", { reserved: true }], ["picture", { reserved: true }], ["pre", { reserved: false }], ["progress", { reserved: false }], ["q", { reserved: false }], ["rp", { reserved: false }], ["rt", { reserved: false }], ["rtc", { reserved: false }], ["ruby", { reserved: false }], ["s", { reserved: false }], ["samp", { reserved: false }], ["script", { reserved: true }], ["section", { reserved: false }], ["select", { reserved: false }], ["small", { reserved: false }], ["source", { reserved: true }], ["spacer", { reserved: false }], ["span", { reserved: false }], ["strike", { reserved: false }], ["strong", { reserved: false }], ["style", { reserved: true }], ["sub", { reserved: false }], ["summary", { reserved: false }], ["sup", { reserved: false }], ["table", { reserved: false }], ["tbody", { reserved: false }], ["td", { reserved: false }], ["textarea", { reserved: false }], ["tfoot", { reserved: false }], ["th", { reserved: false }], ["thead", { reserved: false }], ["time", { reserved: false }], ["title", { reserved: true }], ["tr", { reserved: false }], ["track", { reserved: true }], ["tt", { reserved: false }], ["u", { reserved: false }], ["ul", { reserved: false }], ["var", { reserved: false }], ["video", { reserved: false }], ["wbr", { reserved: false }], ["xmp", { reserved: false }]], o = { entries: function() {
    return _;
  }, forEach: function(L) {
    for (var O = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, B = 0, G = _; B < G.length; B++) {
      var F = i(G[B], 2), R = F[0], v = F[1];
      L.call(O, v, R, _);
    }
  }, get: function(L) {
    var O = _.filter(function(B) {
      return B[0] === L;
    })[0];
    return O && O[1];
  }, has: function(L) {
    return !!o.get(L);
  }, keys: function() {
    return _.map(function(L) {
      var O = i(L, 1), B = O[0];
      return B;
    });
  }, values: function() {
    return _.map(function(L) {
      var O = i(L, 2), B = O[1];
      return B;
    });
  } };
  return lr.default = (0, e.default)(o, o.entries()), lr;
}
var ur = {}, cr = {}, dr = {}, iu;
function i0() {
  if (iu) return dr;
  iu = 1, Object.defineProperty(dr, "__esModule", { value: true }), dr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget"]] };
  return dr.default = e, dr;
}
var pr = {}, su;
function s0() {
  if (su) return pr;
  su = 1, Object.defineProperty(pr, "__esModule", { value: true }), pr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-activedescendant": null, "aria-disabled": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget"]] };
  return pr.default = e, pr;
}
var fr = {}, nu;
function n0() {
  if (nu) return fr;
  nu = 1, Object.defineProperty(fr, "__esModule", { value: true }), fr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null }, relatedConcepts: [{ concept: { name: "input" }, module: "XForms" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget"]] };
  return fr.default = e, fr;
}
var hr = {}, ou;
function o0() {
  if (ou) return hr;
  ou = 1, Object.defineProperty(hr, "__esModule", { value: true }), hr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return hr.default = e, hr;
}
var mr = {}, lu;
function l0() {
  if (lu) return mr;
  lu = 1, Object.defineProperty(mr, "__esModule", { value: true }), mr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-valuemax": null, "aria-valuemin": null, "aria-valuenow": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return mr.default = e, mr;
}
var vr = {}, uu;
function u0() {
  if (uu) return vr;
  uu = 1, Object.defineProperty(vr, "__esModule", { value: true }), vr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: [], prohibitedProps: [], props: { "aria-atomic": null, "aria-busy": null, "aria-controls": null, "aria-current": null, "aria-describedby": null, "aria-details": null, "aria-dropeffect": null, "aria-flowto": null, "aria-grabbed": null, "aria-hidden": null, "aria-keyshortcuts": null, "aria-label": null, "aria-labelledby": null, "aria-live": null, "aria-owns": null, "aria-relevant": null, "aria-roledescription": null }, relatedConcepts: [{ concept: { name: "role" }, module: "XHTML" }, { concept: { name: "type" }, module: "Dublin Core" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [] };
  return vr.default = e, vr;
}
var br = {}, cu;
function c0() {
  if (cu) return br;
  cu = 1, Object.defineProperty(br, "__esModule", { value: true }), br.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: [], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "frontmatter" }, module: "DTB" }, { concept: { name: "level" }, module: "DTB" }, { concept: { name: "level" }, module: "SMIL" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return br.default = e, br;
}
var yr = {}, du;
function d0() {
  if (du) return yr;
  du = 1, Object.defineProperty(yr, "__esModule", { value: true }), yr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return yr.default = e, yr;
}
var gr = {}, pu;
function p0() {
  if (pu) return gr;
  pu = 1, Object.defineProperty(gr, "__esModule", { value: true }), gr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-orientation": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "composite"], ["roletype", "structure", "section", "group"]] };
  return gr.default = e, gr;
}
var Rr = {}, fu;
function f0() {
  if (fu) return Rr;
  fu = 1, Object.defineProperty(Rr, "__esModule", { value: true }), Rr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: [], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype"]] };
  return Rr.default = e, Rr;
}
var xr = {}, hu;
function h0() {
  if (hu) return xr;
  hu = 1, Object.defineProperty(xr, "__esModule", { value: true }), xr.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: [], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype"]] };
  return xr.default = e, xr;
}
var _r = {}, mu;
function m0() {
  if (mu) return _r;
  mu = 1, Object.defineProperty(_r, "__esModule", { value: true }), _r.default = void 0;
  var e = { abstract: true, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-modal": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype"]] };
  return _r.default = e, _r;
}
var vu;
function v0() {
  if (vu) return cr;
  vu = 1, Object.defineProperty(cr, "__esModule", { value: true }), cr.default = void 0;
  var e = O(i0()), t = O(s0()), i = O(n0()), s = O(o0()), l = O(l0()), p = O(u0()), m = O(c0()), w = O(d0()), _ = O(p0()), o = O(f0()), b = O(h0()), L = O(m0());
  function O(G) {
    return G && G.__esModule ? G : { default: G };
  }
  var B = [["command", e.default], ["composite", t.default], ["input", i.default], ["landmark", s.default], ["range", l.default], ["roletype", p.default], ["section", m.default], ["sectionhead", w.default], ["select", _.default], ["structure", o.default], ["widget", b.default], ["window", L.default]];
  return cr.default = B, cr;
}
var Cr = {}, Pr = {}, bu;
function b0() {
  if (bu) return Pr;
  bu = 1, Object.defineProperty(Pr, "__esModule", { value: true }), Pr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-atomic": "true", "aria-live": "assertive" }, relatedConcepts: [{ concept: { name: "alert" }, module: "XForms" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Pr.default = e, Pr;
}
var qr = {}, yu;
function y0() {
  if (yu) return qr;
  yu = 1, Object.defineProperty(qr, "__esModule", { value: true }), qr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "alert" }, module: "XForms" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "alert"], ["roletype", "window", "dialog"]] };
  return qr.default = e, qr;
}
var Tr = {}, gu;
function g0() {
  if (gu) return Tr;
  gu = 1, Object.defineProperty(Tr, "__esModule", { value: true }), Tr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-activedescendant": null, "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "Device Independence Delivery Unit" } }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return Tr.default = e, Tr;
}
var Sr = {}, Ru;
function R0() {
  if (Ru) return Sr;
  Ru = 1, Object.defineProperty(Sr, "__esModule", { value: true }), Sr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-posinset": null, "aria-setsize": null }, relatedConcepts: [{ concept: { name: "article" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "document"]] };
  return Sr.default = e, Sr;
}
var wr = {}, xu;
function x0() {
  if (xu) return wr;
  xu = 1, Object.defineProperty(wr, "__esModule", { value: true }), wr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { constraints: ["scoped to the body element"], name: "header" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return wr.default = e, wr;
}
var Ar = {}, _u;
function _0() {
  if (_u) return Ar;
  _u = 1, Object.defineProperty(Ar, "__esModule", { value: true }), Ar.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "blockquote" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Ar.default = e, Ar;
}
var Er = {}, Cu;
function C0() {
  if (Cu) return Er;
  Cu = 1, Object.defineProperty(Er, "__esModule", { value: true }), Er.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-disabled": null, "aria-expanded": null, "aria-haspopup": null, "aria-pressed": null }, relatedConcepts: [{ concept: { attributes: [{ name: "type", value: "button" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ name: "type", value: "image" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ name: "type", value: "reset" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ name: "type", value: "submit" }], name: "input" }, module: "HTML" }, { concept: { name: "button" }, module: "HTML" }, { concept: { name: "trigger" }, module: "XForms" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command"]] };
  return Er.default = e, Er;
}
var kr = {}, Pu;
function P0() {
  if (Pu) return kr;
  Pu = 1, Object.defineProperty(kr, "__esModule", { value: true }), kr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "caption" }, module: "HTML" }], requireContextRole: ["figure", "grid", "table"], requiredContextRole: ["figure", "grid", "table"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return kr.default = e, kr;
}
var Ir = {}, qu;
function q0() {
  if (qu) return Ir;
  qu = 1, Object.defineProperty(Ir, "__esModule", { value: true }), Ir.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-colindex": null, "aria-colspan": null, "aria-rowindex": null, "aria-rowspan": null }, relatedConcepts: [{ concept: { constraints: ["ancestor table element has table role"], name: "td" }, module: "HTML" }], requireContextRole: ["row"], requiredContextRole: ["row"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Ir.default = e, Ir;
}
var Mr = {}, Tu;
function T0() {
  if (Tu) return Mr;
  Tu = 1, Object.defineProperty(Mr, "__esModule", { value: true }), Mr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-checked": null, "aria-errormessage": null, "aria-expanded": null, "aria-invalid": null, "aria-readonly": null, "aria-required": null }, relatedConcepts: [{ concept: { attributes: [{ name: "type", value: "checkbox" }], name: "input" }, module: "HTML" }, { concept: { name: "option" }, module: "ARIA" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-checked": null }, superClass: [["roletype", "widget", "input"]] };
  return Mr.default = e, Mr;
}
var Lr = {}, Su;
function S0() {
  if (Su) return Lr;
  Su = 1, Object.defineProperty(Lr, "__esModule", { value: true }), Lr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "code" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Lr.default = e, Lr;
}
var Or = {}, wu;
function w0() {
  if (wu) return Or;
  wu = 1, Object.defineProperty(Or, "__esModule", { value: true }), Or.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-sort": null }, relatedConcepts: [{ concept: { name: "th" }, module: "HTML" }, { concept: { attributes: [{ name: "scope", value: "col" }], name: "th" }, module: "HTML" }, { concept: { attributes: [{ name: "scope", value: "colgroup" }], name: "th" }, module: "HTML" }], requireContextRole: ["row"], requiredContextRole: ["row"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "cell"], ["roletype", "structure", "section", "cell", "gridcell"], ["roletype", "widget", "gridcell"], ["roletype", "structure", "sectionhead"]] };
  return Or.default = e, Or;
}
var Nr = {}, Au;
function A0() {
  if (Au) return Nr;
  Au = 1, Object.defineProperty(Nr, "__esModule", { value: true }), Nr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-activedescendant": null, "aria-autocomplete": null, "aria-errormessage": null, "aria-invalid": null, "aria-readonly": null, "aria-required": null, "aria-expanded": "false", "aria-haspopup": "listbox" }, relatedConcepts: [{ concept: { attributes: [{ constraints: ["set"], name: "list" }, { name: "type", value: "email" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "list" }, { name: "type", value: "search" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "list" }, { name: "type", value: "tel" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "list" }, { name: "type", value: "text" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "list" }, { name: "type", value: "url" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "list" }, { name: "type", value: "url" }], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["undefined"], name: "multiple" }, { constraints: ["undefined"], name: "size" }], constraints: ["the multiple attribute is not set and the size attribute does not have a value greater than 1"], name: "select" }, module: "HTML" }, { concept: { name: "select" }, module: "XForms" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-controls": null, "aria-expanded": "false" }, superClass: [["roletype", "widget", "input"]] };
  return Nr.default = e, Nr;
}
var Dr = {}, Eu;
function E0() {
  if (Eu) return Dr;
  Eu = 1, Object.defineProperty(Dr, "__esModule", { value: true }), Dr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { constraints: ["scoped to the body element", "scoped to the main element"], name: "aside" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "aria-label" }], constraints: ["scoped to a sectioning content element", "scoped to a sectioning root element other than body"], name: "aside" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "aria-labelledby" }], constraints: ["scoped to a sectioning content element", "scoped to a sectioning root element other than body"], name: "aside" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Dr.default = e, Dr;
}
var jr = {}, ku;
function k0() {
  if (ku) return jr;
  ku = 1, Object.defineProperty(jr, "__esModule", { value: true }), jr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { constraints: ["scoped to the body element"], name: "footer" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return jr.default = e, jr;
}
var Br = {}, Iu;
function I0() {
  if (Iu) return Br;
  Iu = 1, Object.defineProperty(Br, "__esModule", { value: true }), Br.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "dd" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Br.default = e, Br;
}
var Fr = {}, Mu;
function M0() {
  if (Mu) return Fr;
  Mu = 1, Object.defineProperty(Fr, "__esModule", { value: true }), Fr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "del" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Fr.default = e, Fr;
}
var Vr = {}, Lu;
function L0() {
  if (Lu) return Vr;
  Lu = 1, Object.defineProperty(Vr, "__esModule", { value: true }), Vr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "dialog" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "window"]] };
  return Vr.default = e, Vr;
}
var $r = {}, Ou;
function O0() {
  if (Ou) return $r;
  Ou = 1, Object.defineProperty($r, "__esModule", { value: true }), $r.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ module: "DAISY Guide" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "list"]] };
  return $r.default = e, $r;
}
var Hr = {}, Nu;
function N0() {
  if (Nu) return Hr;
  Nu = 1, Object.defineProperty(Hr, "__esModule", { value: true }), Hr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "Device Independence Delivery Unit" } }, { concept: { name: "html" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return Hr.default = e, Hr;
}
var Ur = {}, Du;
function D0() {
  if (Du) return Ur;
  Du = 1, Object.defineProperty(Ur, "__esModule", { value: true }), Ur.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "em" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Ur.default = e, Ur;
}
var Gr = {}, ju;
function j0() {
  if (ju) return Gr;
  ju = 1, Object.defineProperty(Gr, "__esModule", { value: true }), Gr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["article"]], requiredProps: {}, superClass: [["roletype", "structure", "section", "list"]] };
  return Gr.default = e, Gr;
}
var Wr = {}, Bu;
function B0() {
  if (Bu) return Wr;
  Bu = 1, Object.defineProperty(Wr, "__esModule", { value: true }), Wr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "figure" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Wr.default = e, Wr;
}
var zr = {}, Fu;
function F0() {
  if (Fu) return zr;
  Fu = 1, Object.defineProperty(zr, "__esModule", { value: true }), zr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { attributes: [{ constraints: ["set"], name: "aria-label" }], name: "form" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "aria-labelledby" }], name: "form" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "name" }], name: "form" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return zr.default = e, zr;
}
var Xr = {}, Vu;
function V0() {
  if (Vu) return Xr;
  Vu = 1, Object.defineProperty(Xr, "__esModule", { value: true }), Xr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "a" }, module: "HTML" }, { concept: { name: "area" }, module: "HTML" }, { concept: { name: "aside" }, module: "HTML" }, { concept: { name: "b" }, module: "HTML" }, { concept: { name: "bdo" }, module: "HTML" }, { concept: { name: "body" }, module: "HTML" }, { concept: { name: "data" }, module: "HTML" }, { concept: { name: "div" }, module: "HTML" }, { concept: { constraints: ["scoped to the main element", "scoped to a sectioning content element", "scoped to a sectioning root element other than body"], name: "footer" }, module: "HTML" }, { concept: { constraints: ["scoped to the main element", "scoped to a sectioning content element", "scoped to a sectioning root element other than body"], name: "header" }, module: "HTML" }, { concept: { name: "hgroup" }, module: "HTML" }, { concept: { name: "i" }, module: "HTML" }, { concept: { name: "pre" }, module: "HTML" }, { concept: { name: "q" }, module: "HTML" }, { concept: { name: "samp" }, module: "HTML" }, { concept: { name: "section" }, module: "HTML" }, { concept: { name: "small" }, module: "HTML" }, { concept: { name: "span" }, module: "HTML" }, { concept: { name: "u" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return Xr.default = e, Xr;
}
var Kr = {}, $u;
function $0() {
  if ($u) return Kr;
  $u = 1, Object.defineProperty(Kr, "__esModule", { value: true }), Kr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-multiselectable": null, "aria-readonly": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["row"], ["row", "rowgroup"]], requiredProps: {}, superClass: [["roletype", "widget", "composite"], ["roletype", "structure", "section", "table"]] };
  return Kr.default = e, Kr;
}
var Jr = {}, Hu;
function H0() {
  if (Hu) return Jr;
  Hu = 1, Object.defineProperty(Jr, "__esModule", { value: true }), Jr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null, "aria-readonly": null, "aria-required": null, "aria-selected": null }, relatedConcepts: [{ concept: { constraints: ["ancestor table element has grid role", "ancestor table element has treegrid role"], name: "td" }, module: "HTML" }], requireContextRole: ["row"], requiredContextRole: ["row"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "cell"], ["roletype", "widget"]] };
  return Jr.default = e, Jr;
}
var Qr = {}, Uu;
function U0() {
  if (Uu) return Qr;
  Uu = 1, Object.defineProperty(Qr, "__esModule", { value: true }), Qr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-activedescendant": null, "aria-disabled": null }, relatedConcepts: [{ concept: { name: "details" }, module: "HTML" }, { concept: { name: "fieldset" }, module: "HTML" }, { concept: { name: "optgroup" }, module: "HTML" }, { concept: { name: "address" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Qr.default = e, Qr;
}
var Yr = {}, Gu;
function G0() {
  if (Gu) return Yr;
  Gu = 1, Object.defineProperty(Yr, "__esModule", { value: true }), Yr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-level": "2" }, relatedConcepts: [{ concept: { name: "h1" }, module: "HTML" }, { concept: { name: "h2" }, module: "HTML" }, { concept: { name: "h3" }, module: "HTML" }, { concept: { name: "h4" }, module: "HTML" }, { concept: { name: "h5" }, module: "HTML" }, { concept: { name: "h6" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-level": "2" }, superClass: [["roletype", "structure", "sectionhead"]] };
  return Yr.default = e, Yr;
}
var Zr = {}, Wu;
function W0() {
  if (Wu) return Zr;
  Wu = 1, Object.defineProperty(Zr, "__esModule", { value: true }), Zr.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { attributes: [{ constraints: ["set"], name: "alt" }], name: "img" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["undefined"], name: "alt" }], name: "img" }, module: "HTML" }, { concept: { name: "imggroup" }, module: "DTB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Zr.default = e, Zr;
}
var ea = {}, zu;
function z0() {
  if (zu) return ea;
  zu = 1, Object.defineProperty(ea, "__esModule", { value: true }), ea.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "ins" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ea.default = e, ea;
}
var ta = {}, Xu;
function X0() {
  if (Xu) return ta;
  Xu = 1, Object.defineProperty(ta, "__esModule", { value: true }), ta.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-disabled": null, "aria-expanded": null, "aria-haspopup": null }, relatedConcepts: [{ concept: { attributes: [{ constraints: ["set"], name: "href" }], name: "a" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "href" }], name: "area" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command"]] };
  return ta.default = e, ta;
}
var ra = {}, Ku;
function K0() {
  if (Ku) return ra;
  Ku = 1, Object.defineProperty(ra, "__esModule", { value: true }), ra.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "menu" }, module: "HTML" }, { concept: { name: "ol" }, module: "HTML" }, { concept: { name: "ul" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["listitem"]], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ra.default = e, ra;
}
var aa = {}, Ju;
function J0() {
  if (Ju) return aa;
  Ju = 1, Object.defineProperty(aa, "__esModule", { value: true }), aa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-expanded": null, "aria-invalid": null, "aria-multiselectable": null, "aria-readonly": null, "aria-required": null, "aria-orientation": "vertical" }, relatedConcepts: [{ concept: { attributes: [{ constraints: [">1"], name: "size" }], constraints: ["the size attribute value is greater than 1"], name: "select" }, module: "HTML" }, { concept: { attributes: [{ name: "multiple" }], name: "select" }, module: "HTML" }, { concept: { name: "datalist" }, module: "HTML" }, { concept: { name: "list" }, module: "ARIA" }, { concept: { name: "select" }, module: "XForms" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["option", "group"], ["option"]], requiredProps: {}, superClass: [["roletype", "widget", "composite", "select"], ["roletype", "structure", "section", "group", "select"]] };
  return aa.default = e, aa;
}
var ia = {}, Qu;
function Q0() {
  if (Qu) return ia;
  Qu = 1, Object.defineProperty(ia, "__esModule", { value: true }), ia.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-level": null, "aria-posinset": null, "aria-setsize": null }, relatedConcepts: [{ concept: { constraints: ["direct descendant of ol", "direct descendant of ul", "direct descendant of menu"], name: "li" }, module: "HTML" }, { concept: { name: "item" }, module: "XForms" }], requireContextRole: ["directory", "list"], requiredContextRole: ["directory", "list"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ia.default = e, ia;
}
var sa = {}, Yu;
function Y0() {
  if (Yu) return sa;
  Yu = 1, Object.defineProperty(sa, "__esModule", { value: true }), sa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-live": "polite" }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return sa.default = e, sa;
}
var na = {}, Zu;
function Z0() {
  if (Zu) return na;
  Zu = 1, Object.defineProperty(na, "__esModule", { value: true }), na.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "main" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return na.default = e, na;
}
var oa = {}, ec;
function eb() {
  if (ec) return oa;
  ec = 1, Object.defineProperty(oa, "__esModule", { value: true }), oa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: [], props: { "aria-braillelabel": null, "aria-brailleroledescription": null, "aria-description": null }, relatedConcepts: [{ concept: { name: "mark" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return oa.default = e, oa;
}
var la = {}, tc;
function tb() {
  if (tc) return la;
  tc = 1, Object.defineProperty(la, "__esModule", { value: true }), la.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return la.default = e, la;
}
var ua = {}, rc;
function rb() {
  if (rc) return ua;
  rc = 1, Object.defineProperty(ua, "__esModule", { value: true }), ua.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "math" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ua.default = e, ua;
}
var ca = {}, ac;
function ab() {
  if (ac) return ca;
  ac = 1, Object.defineProperty(ca, "__esModule", { value: true }), ca.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-orientation": "vertical" }, relatedConcepts: [{ concept: { name: "MENU" }, module: "JAPI" }, { concept: { name: "list" }, module: "ARIA" }, { concept: { name: "select" }, module: "XForms" }, { concept: { name: "sidebar" }, module: "DTB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["menuitem", "group"], ["menuitemradio", "group"], ["menuitemcheckbox", "group"], ["menuitem"], ["menuitemcheckbox"], ["menuitemradio"]], requiredProps: {}, superClass: [["roletype", "widget", "composite", "select"], ["roletype", "structure", "section", "group", "select"]] };
  return ca.default = e, ca;
}
var da = {}, ic;
function ib() {
  if (ic) return da;
  ic = 1, Object.defineProperty(da, "__esModule", { value: true }), da.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-orientation": "horizontal" }, relatedConcepts: [{ concept: { name: "toolbar" }, module: "ARIA" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["menuitem", "group"], ["menuitemradio", "group"], ["menuitemcheckbox", "group"], ["menuitem"], ["menuitemcheckbox"], ["menuitemradio"]], requiredProps: {}, superClass: [["roletype", "widget", "composite", "select", "menu"], ["roletype", "structure", "section", "group", "select", "menu"]] };
  return da.default = e, da;
}
var pa = {}, sc;
function sb() {
  if (sc) return pa;
  sc = 1, Object.defineProperty(pa, "__esModule", { value: true }), pa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-disabled": null, "aria-expanded": null, "aria-haspopup": null, "aria-posinset": null, "aria-setsize": null }, relatedConcepts: [{ concept: { name: "MENU_ITEM" }, module: "JAPI" }, { concept: { name: "listitem" }, module: "ARIA" }, { concept: { name: "option" }, module: "ARIA" }], requireContextRole: ["group", "menu", "menubar"], requiredContextRole: ["group", "menu", "menubar"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command"]] };
  return pa.default = e, pa;
}
var fa = {}, nc;
function nb() {
  if (nc) return fa;
  nc = 1, Object.defineProperty(fa, "__esModule", { value: true }), fa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "menuitem" }, module: "ARIA" }], requireContextRole: ["group", "menu", "menubar"], requiredContextRole: ["group", "menu", "menubar"], requiredOwnedElements: [], requiredProps: { "aria-checked": null }, superClass: [["roletype", "widget", "input", "checkbox"], ["roletype", "widget", "command", "menuitem"]] };
  return fa.default = e, fa;
}
var ha = {}, oc;
function ob() {
  if (oc) return ha;
  oc = 1, Object.defineProperty(ha, "__esModule", { value: true }), ha.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "menuitem" }, module: "ARIA" }], requireContextRole: ["group", "menu", "menubar"], requiredContextRole: ["group", "menu", "menubar"], requiredOwnedElements: [], requiredProps: { "aria-checked": null }, superClass: [["roletype", "widget", "input", "checkbox", "menuitemcheckbox"], ["roletype", "widget", "command", "menuitem", "menuitemcheckbox"], ["roletype", "widget", "input", "radio"]] };
  return ha.default = e, ha;
}
var ma = {}, lc;
function lb() {
  if (lc) return ma;
  lc = 1, Object.defineProperty(ma, "__esModule", { value: true }), ma.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-valuetext": null, "aria-valuemax": "100", "aria-valuemin": "0" }, relatedConcepts: [{ concept: { name: "meter" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-valuenow": null }, superClass: [["roletype", "structure", "range"]] };
  return ma.default = e, ma;
}
var va = {}, uc;
function ub() {
  if (uc) return va;
  uc = 1, Object.defineProperty(va, "__esModule", { value: true }), va.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "nav" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return va.default = e, va;
}
var ba = {}, cc;
function cb() {
  if (cc) return ba;
  cc = 1, Object.defineProperty(ba, "__esModule", { value: true }), ba.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: [], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [] };
  return ba.default = e, ba;
}
var ya = {}, dc;
function db() {
  if (dc) return ya;
  dc = 1, Object.defineProperty(ya, "__esModule", { value: true }), ya.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ya.default = e, ya;
}
var ga = {}, pc;
function pb() {
  if (pc) return ga;
  pc = 1, Object.defineProperty(ga, "__esModule", { value: true }), ga.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-checked": null, "aria-posinset": null, "aria-setsize": null, "aria-selected": "false" }, relatedConcepts: [{ concept: { name: "item" }, module: "XForms" }, { concept: { name: "listitem" }, module: "ARIA" }, { concept: { name: "option" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-selected": "false" }, superClass: [["roletype", "widget", "input"]] };
  return ga.default = e, ga;
}
var Ra = {}, fc;
function fb() {
  if (fc) return Ra;
  fc = 1, Object.defineProperty(Ra, "__esModule", { value: true }), Ra.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "p" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Ra.default = e, Ra;
}
var xa = {}, hc;
function hb() {
  if (hc) return xa;
  hc = 1, Object.defineProperty(xa, "__esModule", { value: true }), xa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { attributes: [{ name: "alt", value: "" }], name: "img" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return xa.default = e, xa;
}
var _a = {}, mc;
function mb() {
  if (mc) return _a;
  mc = 1, Object.defineProperty(_a, "__esModule", { value: true }), _a.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-valuetext": null }, relatedConcepts: [{ concept: { name: "progress" }, module: "HTML" }, { concept: { name: "status" }, module: "ARIA" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "range"], ["roletype", "widget"]] };
  return _a.default = e, _a;
}
var Ca = {}, vc;
function vb() {
  if (vc) return Ca;
  vc = 1, Object.defineProperty(Ca, "__esModule", { value: true }), Ca.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-checked": null, "aria-posinset": null, "aria-setsize": null }, relatedConcepts: [{ concept: { attributes: [{ name: "type", value: "radio" }], name: "input" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-checked": null }, superClass: [["roletype", "widget", "input"]] };
  return Ca.default = e, Ca;
}
var Pa = {}, bc;
function bb() {
  if (bc) return Pa;
  bc = 1, Object.defineProperty(Pa, "__esModule", { value: true }), Pa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null, "aria-readonly": null, "aria-required": null }, relatedConcepts: [{ concept: { name: "list" }, module: "ARIA" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["radio"]], requiredProps: {}, superClass: [["roletype", "widget", "composite", "select"], ["roletype", "structure", "section", "group", "select"]] };
  return Pa.default = e, Pa;
}
var qa = {}, yc;
function yb() {
  if (yc) return qa;
  yc = 1, Object.defineProperty(qa, "__esModule", { value: true }), qa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { attributes: [{ constraints: ["set"], name: "aria-label" }], name: "section" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["set"], name: "aria-labelledby" }], name: "section" }, module: "HTML" }, { concept: { name: "Device Independence Glossart perceivable unit" } }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return qa.default = e, qa;
}
var Ta = {}, gc;
function gb() {
  if (gc) return Ta;
  gc = 1, Object.defineProperty(Ta, "__esModule", { value: true }), Ta.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-colindex": null, "aria-expanded": null, "aria-level": null, "aria-posinset": null, "aria-rowindex": null, "aria-selected": null, "aria-setsize": null }, relatedConcepts: [{ concept: { name: "tr" }, module: "HTML" }], requireContextRole: ["grid", "rowgroup", "table", "treegrid"], requiredContextRole: ["grid", "rowgroup", "table", "treegrid"], requiredOwnedElements: [["cell"], ["columnheader"], ["gridcell"], ["rowheader"]], requiredProps: {}, superClass: [["roletype", "structure", "section", "group"], ["roletype", "widget"]] };
  return Ta.default = e, Ta;
}
var Sa = {}, Rc;
function Rb() {
  if (Rc) return Sa;
  Rc = 1, Object.defineProperty(Sa, "__esModule", { value: true }), Sa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "tbody" }, module: "HTML" }, { concept: { name: "tfoot" }, module: "HTML" }, { concept: { name: "thead" }, module: "HTML" }], requireContextRole: ["grid", "table", "treegrid"], requiredContextRole: ["grid", "table", "treegrid"], requiredOwnedElements: [["row"]], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return Sa.default = e, Sa;
}
var wa = {}, xc;
function xb() {
  if (xc) return wa;
  xc = 1, Object.defineProperty(wa, "__esModule", { value: true }), wa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-sort": null }, relatedConcepts: [{ concept: { attributes: [{ name: "scope", value: "row" }], name: "th" }, module: "HTML" }, { concept: { attributes: [{ name: "scope", value: "rowgroup" }], name: "th" }, module: "HTML" }], requireContextRole: ["row", "rowgroup"], requiredContextRole: ["row", "rowgroup"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "cell"], ["roletype", "structure", "section", "cell", "gridcell"], ["roletype", "widget", "gridcell"], ["roletype", "structure", "sectionhead"]] };
  return wa.default = e, wa;
}
var Aa = {}, _c;
function _b() {
  if (_c) return Aa;
  _c = 1, Object.defineProperty(Aa, "__esModule", { value: true }), Aa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-valuetext": null, "aria-orientation": "vertical", "aria-valuemax": "100", "aria-valuemin": "0" }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-controls": null, "aria-valuenow": null }, superClass: [["roletype", "structure", "range"], ["roletype", "widget"]] };
  return Aa.default = e, Aa;
}
var Ea = {}, Cc;
function Cb() {
  if (Cc) return Ea;
  Cc = 1, Object.defineProperty(Ea, "__esModule", { value: true }), Ea.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Ea.default = e, Ea;
}
var ka = {}, Pc;
function Pb() {
  if (Pc) return ka;
  Pc = 1, Object.defineProperty(ka, "__esModule", { value: true }), ka.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { attributes: [{ constraints: ["undefined"], name: "list" }, { name: "type", value: "search" }], constraints: ["the list attribute is not set"], name: "input" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "input", "textbox"]] };
  return ka.default = e, ka;
}
var Ia = {}, qc;
function qb() {
  if (qc) return Ia;
  qc = 1, Object.defineProperty(Ia, "__esModule", { value: true }), Ia.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-orientation": "horizontal", "aria-valuemax": "100", "aria-valuemin": "0", "aria-valuenow": null, "aria-valuetext": null }, relatedConcepts: [{ concept: { name: "hr" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure"]] };
  return Ia.default = e, Ia;
}
var Ma = {}, Tc;
function Tb() {
  if (Tc) return Ma;
  Tc = 1, Object.defineProperty(Ma, "__esModule", { value: true }), Ma.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-haspopup": null, "aria-invalid": null, "aria-readonly": null, "aria-valuetext": null, "aria-orientation": "horizontal", "aria-valuemax": "100", "aria-valuemin": "0" }, relatedConcepts: [{ concept: { attributes: [{ name: "type", value: "range" }], name: "input" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-valuenow": null }, superClass: [["roletype", "widget", "input"], ["roletype", "structure", "range"]] };
  return Ma.default = e, Ma;
}
var La = {}, Sc;
function Sb() {
  if (Sc) return La;
  Sc = 1, Object.defineProperty(La, "__esModule", { value: true }), La.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null, "aria-readonly": null, "aria-required": null, "aria-valuetext": null, "aria-valuenow": "0" }, relatedConcepts: [{ concept: { attributes: [{ name: "type", value: "number" }], name: "input" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "composite"], ["roletype", "widget", "input"], ["roletype", "structure", "range"]] };
  return La.default = e, La;
}
var Oa = {}, wc;
function wb() {
  if (wc) return Oa;
  wc = 1, Object.defineProperty(Oa, "__esModule", { value: true }), Oa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-atomic": "true", "aria-live": "polite" }, relatedConcepts: [{ concept: { name: "output" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Oa.default = e, Oa;
}
var Na = {}, Ac;
function Ab() {
  if (Ac) return Na;
  Ac = 1, Object.defineProperty(Na, "__esModule", { value: true }), Na.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "strong" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Na.default = e, Na;
}
var Da = {}, Ec;
function Eb() {
  if (Ec) return Da;
  Ec = 1, Object.defineProperty(Da, "__esModule", { value: true }), Da.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "sub" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Da.default = e, Da;
}
var ja = {}, kc;
function kb() {
  if (kc) return ja;
  kc = 1, Object.defineProperty(ja, "__esModule", { value: true }), ja.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: ["aria-label", "aria-labelledby"], props: {}, relatedConcepts: [{ concept: { name: "sup" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ja.default = e, ja;
}
var Ba = {}, Ic;
function Ib() {
  if (Ic) return Ba;
  Ic = 1, Object.defineProperty(Ba, "__esModule", { value: true }), Ba.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "button" }, module: "ARIA" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: { "aria-checked": null }, superClass: [["roletype", "widget", "input", "checkbox"]] };
  return Ba.default = e, Ba;
}
var Fa = {}, Mc;
function Mb() {
  if (Mc) return Fa;
  Mc = 1, Object.defineProperty(Fa, "__esModule", { value: true }), Fa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: true, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-disabled": null, "aria-expanded": null, "aria-haspopup": null, "aria-posinset": null, "aria-setsize": null, "aria-selected": "false" }, relatedConcepts: [], requireContextRole: ["tablist"], requiredContextRole: ["tablist"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "sectionhead"], ["roletype", "widget"]] };
  return Fa.default = e, Fa;
}
var Va = {}, Lc;
function Lb() {
  if (Lc) return Va;
  Lc = 1, Object.defineProperty(Va, "__esModule", { value: true }), Va.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-colcount": null, "aria-rowcount": null }, relatedConcepts: [{ concept: { name: "table" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["row"], ["row", "rowgroup"]], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Va.default = e, Va;
}
var $a = {}, Oc;
function Ob() {
  if (Oc) return $a;
  Oc = 1, Object.defineProperty($a, "__esModule", { value: true }), $a.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-level": null, "aria-multiselectable": null, "aria-orientation": "horizontal" }, relatedConcepts: [{ module: "DAISY", concept: { name: "guide" } }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["tab"]], requiredProps: {}, superClass: [["roletype", "widget", "composite"]] };
  return $a.default = e, $a;
}
var Ha = {}, Nc;
function Nb() {
  if (Nc) return Ha;
  Nc = 1, Object.defineProperty(Ha, "__esModule", { value: true }), Ha.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Ha.default = e, Ha;
}
var Ua = {}, Dc;
function Db() {
  if (Dc) return Ua;
  Dc = 1, Object.defineProperty(Ua, "__esModule", { value: true }), Ua.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "dfn" }, module: "HTML" }, { concept: { name: "dt" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Ua.default = e, Ua;
}
var Ga = {}, jc;
function jb() {
  if (jc) return Ga;
  jc = 1, Object.defineProperty(Ga, "__esModule", { value: true }), Ga.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-activedescendant": null, "aria-autocomplete": null, "aria-errormessage": null, "aria-haspopup": null, "aria-invalid": null, "aria-multiline": null, "aria-placeholder": null, "aria-readonly": null, "aria-required": null }, relatedConcepts: [{ concept: { attributes: [{ constraints: ["undefined"], name: "type" }, { constraints: ["undefined"], name: "list" }], constraints: ["the list attribute is not set"], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["undefined"], name: "list" }, { name: "type", value: "email" }], constraints: ["the list attribute is not set"], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["undefined"], name: "list" }, { name: "type", value: "tel" }], constraints: ["the list attribute is not set"], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["undefined"], name: "list" }, { name: "type", value: "text" }], constraints: ["the list attribute is not set"], name: "input" }, module: "HTML" }, { concept: { attributes: [{ constraints: ["undefined"], name: "list" }, { name: "type", value: "url" }], constraints: ["the list attribute is not set"], name: "input" }, module: "HTML" }, { concept: { name: "input" }, module: "XForms" }, { concept: { name: "textarea" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "input"]] };
  return Ga.default = e, Ga;
}
var Wa = {}, Bc;
function Bb() {
  if (Bc) return Wa;
  Bc = 1, Object.defineProperty(Wa, "__esModule", { value: true }), Wa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "time" }, module: "HTML" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Wa.default = e, Wa;
}
var za = {}, Fc;
function Fb() {
  if (Fc) return za;
  Fc = 1, Object.defineProperty(za, "__esModule", { value: true }), za.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "status"]] };
  return za.default = e, za;
}
var Xa = {}, Vc;
function Vb() {
  if (Vc) return Xa;
  Vc = 1, Object.defineProperty(Xa, "__esModule", { value: true }), Xa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-orientation": "horizontal" }, relatedConcepts: [{ concept: { name: "menubar" }, module: "ARIA" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "group"]] };
  return Xa.default = e, Xa;
}
var Ka = {}, $c;
function $b() {
  if ($c) return Ka;
  $c = 1, Object.defineProperty(Ka, "__esModule", { value: true }), Ka.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Ka.default = e, Ka;
}
var Ja = {}, Hc;
function Hb() {
  if (Hc) return Ja;
  Hc = 1, Object.defineProperty(Ja, "__esModule", { value: true }), Ja.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null, "aria-multiselectable": null, "aria-required": null, "aria-orientation": "vertical" }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["treeitem", "group"], ["treeitem"]], requiredProps: {}, superClass: [["roletype", "widget", "composite", "select"], ["roletype", "structure", "section", "group", "select"]] };
  return Ja.default = e, Ja;
}
var Qa = {}, Uc;
function Ub() {
  if (Uc) return Qa;
  Uc = 1, Object.defineProperty(Qa, "__esModule", { value: true }), Qa.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["row"], ["row", "rowgroup"]], requiredProps: {}, superClass: [["roletype", "widget", "composite", "grid"], ["roletype", "structure", "section", "table", "grid"], ["roletype", "widget", "composite", "select", "tree"], ["roletype", "structure", "section", "group", "select", "tree"]] };
  return Qa.default = e, Qa;
}
var Ya = {}, Gc;
function Gb() {
  if (Gc) return Ya;
  Gc = 1, Object.defineProperty(Ya, "__esModule", { value: true }), Ya.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-expanded": null, "aria-haspopup": null }, relatedConcepts: [], requireContextRole: ["group", "tree"], requiredContextRole: ["group", "tree"], requiredOwnedElements: [], requiredProps: { "aria-selected": null }, superClass: [["roletype", "structure", "section", "listitem"], ["roletype", "widget", "input", "option"]] };
  return Ya.default = e, Ya;
}
var Wc;
function Wb() {
  if (Wc) return Cr;
  Wc = 1, Object.defineProperty(Cr, "__esModule", { value: true }), Cr.default = void 0;
  var e = Y(b0()), t = Y(y0()), i = Y(g0()), s = Y(R0()), l = Y(x0()), p = Y(_0()), m = Y(C0()), w = Y(P0()), _ = Y(q0()), o = Y(T0()), b = Y(S0()), L = Y(w0()), O = Y(A0()), B = Y(E0()), G = Y(k0()), F = Y(I0()), R = Y(M0()), v = Y(L0()), h = Y(O0()), g = Y(N0()), S = Y(D0()), C = Y(j0()), x = Y(B0()), E = Y(F0()), D = Y(V0()), z = Y($0()), Z = Y(H0()), A = Y(U0()), N = Y(G0()), I = Y(W0()), M = Y(z0()), V = Y(X0()), c = Y(K0()), r = Y(J0()), a = Y(Q0()), n = Y(Y0()), u = Y(Z0()), f = Y(eb()), y = Y(tb()), P = Y(rb()), T = Y(ab()), q = Y(ib()), H = Y(sb()), J = Y(nb()), re = Y(ob()), Q = Y(lb()), pe = Y(ub()), de = Y(cb()), fe = Y(db()), he = Y(pb()), we = Y(fb()), Oe = Y(hb()), Ee = Y(mb()), Ue = Y(vb()), Ne = Y(bb()), Ze = Y(yb()), Je = Y(gb()), vt = Y(Rb()), bt = Y(xb()), j = Y(_b()), X = Y(Cb()), K = Y(Pb()), ee = Y(qb()), ce = Y(Tb()), se = Y(Sb()), Re = Y(wb()), ke = Y(Ab()), ge = Y(Eb()), Ae = Y(kb()), ze = Y(Ib()), je = Y(Mb()), De = Y(Lb()), Ge = Y(Ob()), te = Y(Nb()), Ie = Y(Db()), be = Y(jb()), Qe = Y(Bb()), tt = Y(Fb()), Xe = Y(Vb()), We = Y($b()), Ye = Y(Hb()), St = Y(Ub()), Ht = Y(Gb());
  function Y(gt) {
    return gt && gt.__esModule ? gt : { default: gt };
  }
  var yt = [["alert", e.default], ["alertdialog", t.default], ["application", i.default], ["article", s.default], ["banner", l.default], ["blockquote", p.default], ["button", m.default], ["caption", w.default], ["cell", _.default], ["checkbox", o.default], ["code", b.default], ["columnheader", L.default], ["combobox", O.default], ["complementary", B.default], ["contentinfo", G.default], ["definition", F.default], ["deletion", R.default], ["dialog", v.default], ["directory", h.default], ["document", g.default], ["emphasis", S.default], ["feed", C.default], ["figure", x.default], ["form", E.default], ["generic", D.default], ["grid", z.default], ["gridcell", Z.default], ["group", A.default], ["heading", N.default], ["img", I.default], ["insertion", M.default], ["link", V.default], ["list", c.default], ["listbox", r.default], ["listitem", a.default], ["log", n.default], ["main", u.default], ["mark", f.default], ["marquee", y.default], ["math", P.default], ["menu", T.default], ["menubar", q.default], ["menuitem", H.default], ["menuitemcheckbox", J.default], ["menuitemradio", re.default], ["meter", Q.default], ["navigation", pe.default], ["none", de.default], ["note", fe.default], ["option", he.default], ["paragraph", we.default], ["presentation", Oe.default], ["progressbar", Ee.default], ["radio", Ue.default], ["radiogroup", Ne.default], ["region", Ze.default], ["row", Je.default], ["rowgroup", vt.default], ["rowheader", bt.default], ["scrollbar", j.default], ["search", X.default], ["searchbox", K.default], ["separator", ee.default], ["slider", ce.default], ["spinbutton", se.default], ["status", Re.default], ["strong", ke.default], ["subscript", ge.default], ["superscript", Ae.default], ["switch", ze.default], ["tab", je.default], ["table", De.default], ["tablist", Ge.default], ["tabpanel", te.default], ["term", Ie.default], ["textbox", be.default], ["time", Qe.default], ["timer", tt.default], ["toolbar", Xe.default], ["tooltip", We.default], ["tree", Ye.default], ["treegrid", St.default], ["treeitem", Ht.default]];
  return Cr.default = yt, Cr;
}
var Za = {}, ei = {}, zc;
function zb() {
  if (zc) return ei;
  zc = 1, Object.defineProperty(ei, "__esModule", { value: true }), ei.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "abstract [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ei.default = e, ei;
}
var ti = {}, Xc;
function Xb() {
  if (Xc) return ti;
  Xc = 1, Object.defineProperty(ti, "__esModule", { value: true }), ti.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "acknowledgments [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return ti.default = e, ti;
}
var ri = {}, Kc;
function Kb() {
  if (Kc) return ri;
  Kc = 1, Object.defineProperty(ri, "__esModule", { value: true }), ri.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "afterword [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return ri.default = e, ri;
}
var ai = {}, Jc;
function Jb() {
  if (Jc) return ai;
  Jc = 1, Object.defineProperty(ai, "__esModule", { value: true }), ai.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "appendix [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return ai.default = e, ai;
}
var ii = {}, Qc;
function Qb() {
  if (Qc) return ii;
  Qc = 1, Object.defineProperty(ii, "__esModule", { value: true }), ii.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "referrer [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command", "link"]] };
  return ii.default = e, ii;
}
var si = {}, Yc;
function Yb() {
  if (Yc) return si;
  Yc = 1, Object.defineProperty(si, "__esModule", { value: true }), si.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "EPUB biblioentry [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: ["doc-bibliography"], requiredContextRole: ["doc-bibliography"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "listitem"]] };
  return si.default = e, si;
}
var ni = {}, Zc;
function Zb() {
  if (Zc) return ni;
  Zc = 1, Object.defineProperty(ni, "__esModule", { value: true }), ni.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "bibliography [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["doc-biblioentry"]], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return ni.default = e, ni;
}
var oi = {}, ed;
function ey() {
  if (ed) return oi;
  ed = 1, Object.defineProperty(oi, "__esModule", { value: true }), oi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "biblioref [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command", "link"]] };
  return oi.default = e, oi;
}
var li = {}, td;
function ty() {
  if (td) return li;
  td = 1, Object.defineProperty(li, "__esModule", { value: true }), li.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "chapter [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return li.default = e, li;
}
var ui = {}, rd;
function ry() {
  if (rd) return ui;
  rd = 1, Object.defineProperty(ui, "__esModule", { value: true }), ui.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "colophon [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ui.default = e, ui;
}
var ci = {}, ad;
function ay() {
  if (ad) return ci;
  ad = 1, Object.defineProperty(ci, "__esModule", { value: true }), ci.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "conclusion [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return ci.default = e, ci;
}
var di = {}, id;
function iy() {
  if (id) return di;
  id = 1, Object.defineProperty(di, "__esModule", { value: true }), di.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "cover [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "img"]] };
  return di.default = e, di;
}
var pi = {}, sd;
function sy() {
  if (sd) return pi;
  sd = 1, Object.defineProperty(pi, "__esModule", { value: true }), pi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "credit [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return pi.default = e, pi;
}
var fi = {}, nd;
function ny() {
  if (nd) return fi;
  nd = 1, Object.defineProperty(fi, "__esModule", { value: true }), fi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "credits [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return fi.default = e, fi;
}
var hi = {}, od;
function oy() {
  if (od) return hi;
  od = 1, Object.defineProperty(hi, "__esModule", { value: true }), hi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "dedication [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return hi.default = e, hi;
}
var mi = {}, ld;
function ly() {
  if (ld) return mi;
  ld = 1, Object.defineProperty(mi, "__esModule", { value: true }), mi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "rearnote [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: ["doc-endnotes"], requiredContextRole: ["doc-endnotes"], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "listitem"]] };
  return mi.default = e, mi;
}
var vi = {}, ud;
function uy() {
  if (ud) return vi;
  ud = 1, Object.defineProperty(vi, "__esModule", { value: true }), vi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "rearnotes [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["doc-endnote"]], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return vi.default = e, vi;
}
var bi = {}, cd;
function cy() {
  if (cd) return bi;
  cd = 1, Object.defineProperty(bi, "__esModule", { value: true }), bi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "epigraph [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return bi.default = e, bi;
}
var yi = {}, dd;
function dy() {
  if (dd) return yi;
  dd = 1, Object.defineProperty(yi, "__esModule", { value: true }), yi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "epilogue [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return yi.default = e, yi;
}
var gi = {}, pd;
function py() {
  if (pd) return gi;
  pd = 1, Object.defineProperty(gi, "__esModule", { value: true }), gi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "errata [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return gi.default = e, gi;
}
var Ri = {}, fd;
function fy() {
  if (fd) return Ri;
  fd = 1, Object.defineProperty(Ri, "__esModule", { value: true }), Ri.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Ri.default = e, Ri;
}
var xi = {}, hd;
function hy() {
  if (hd) return xi;
  hd = 1, Object.defineProperty(xi, "__esModule", { value: true }), xi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "footnote [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return xi.default = e, xi;
}
var _i = {}, md;
function my() {
  if (md) return _i;
  md = 1, Object.defineProperty(_i, "__esModule", { value: true }), _i.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "foreword [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return _i.default = e, _i;
}
var Ci = {}, vd;
function vy() {
  if (vd) return Ci;
  vd = 1, Object.defineProperty(Ci, "__esModule", { value: true }), Ci.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "glossary [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [["definition"], ["term"]], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Ci.default = e, Ci;
}
var Pi = {}, bd;
function by() {
  if (bd) return Pi;
  bd = 1, Object.defineProperty(Pi, "__esModule", { value: true }), Pi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "glossref [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command", "link"]] };
  return Pi.default = e, Pi;
}
var qi = {}, yd;
function yy() {
  if (yd) return qi;
  yd = 1, Object.defineProperty(qi, "__esModule", { value: true }), qi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "index [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark", "navigation"]] };
  return qi.default = e, qi;
}
var Ti = {}, gd;
function gy() {
  if (gd) return Ti;
  gd = 1, Object.defineProperty(Ti, "__esModule", { value: true }), Ti.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "introduction [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Ti.default = e, Ti;
}
var Si = {}, Rd;
function Ry() {
  if (Rd) return Si;
  Rd = 1, Object.defineProperty(Si, "__esModule", { value: true }), Si.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "noteref [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "widget", "command", "link"]] };
  return Si.default = e, Si;
}
var wi = {}, xd;
function xy() {
  if (xd) return wi;
  xd = 1, Object.defineProperty(wi, "__esModule", { value: true }), wi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "notice [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "note"]] };
  return wi.default = e, wi;
}
var Ai = {}, _d;
function _y() {
  if (_d) return Ai;
  _d = 1, Object.defineProperty(Ai, "__esModule", { value: true }), Ai.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "pagebreak [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "separator"]] };
  return Ai.default = e, Ai;
}
var Ei = {}, Cd;
function Cy() {
  if (Cd) return Ei;
  Cd = 1, Object.defineProperty(Ei, "__esModule", { value: true }), Ei.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: [], props: { "aria-braillelabel": null, "aria-brailleroledescription": null, "aria-description": null, "aria-disabled": null, "aria-errormessage": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Ei.default = e, Ei;
}
var ki = {}, Pd;
function Py() {
  if (Pd) return ki;
  Pd = 1, Object.defineProperty(ki, "__esModule", { value: true }), ki.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["prohibited"], prohibitedProps: [], props: { "aria-braillelabel": null, "aria-brailleroledescription": null, "aria-description": null, "aria-disabled": null, "aria-errormessage": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return ki.default = e, ki;
}
var Ii = {}, qd;
function qy() {
  if (qd) return Ii;
  qd = 1, Object.defineProperty(Ii, "__esModule", { value: true }), Ii.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "page-list [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark", "navigation"]] };
  return Ii.default = e, Ii;
}
var Mi = {}, Td;
function Ty() {
  if (Td) return Mi;
  Td = 1, Object.defineProperty(Mi, "__esModule", { value: true }), Mi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "part [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Mi.default = e, Mi;
}
var Li = {}, Sd;
function Sy() {
  if (Sd) return Li;
  Sd = 1, Object.defineProperty(Li, "__esModule", { value: true }), Li.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "preface [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Li.default = e, Li;
}
var Oi = {}, wd;
function wy() {
  if (wd) return Oi;
  wd = 1, Object.defineProperty(Oi, "__esModule", { value: true }), Oi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "prologue [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark"]] };
  return Oi.default = e, Oi;
}
var Ni = {}, Ad;
function Ay() {
  if (Ad) return Ni;
  Ad = 1, Object.defineProperty(Ni, "__esModule", { value: true }), Ni.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: {}, relatedConcepts: [{ concept: { name: "pullquote [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["none"]] };
  return Ni.default = e, Ni;
}
var Di = {}, Ed;
function Ey() {
  if (Ed) return Di;
  Ed = 1, Object.defineProperty(Di, "__esModule", { value: true }), Di.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "qna [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section"]] };
  return Di.default = e, Di;
}
var ji = {}, kd;
function ky() {
  if (kd) return ji;
  kd = 1, Object.defineProperty(ji, "__esModule", { value: true }), ji.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "subtitle [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "sectionhead"]] };
  return ji.default = e, ji;
}
var Bi = {}, Id;
function Iy() {
  if (Id) return Bi;
  Id = 1, Object.defineProperty(Bi, "__esModule", { value: true }), Bi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "help [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "note"]] };
  return Bi.default = e, Bi;
}
var Fi = {}, Md;
function My() {
  if (Md) return Fi;
  Md = 1, Object.defineProperty(Fi, "__esModule", { value: true }), Fi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ concept: { name: "toc [EPUB-SSV]" }, module: "EPUB" }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "landmark", "navigation"]] };
  return Fi.default = e, Fi;
}
var Ld;
function Ly() {
  if (Ld) return Za;
  Ld = 1, Object.defineProperty(Za, "__esModule", { value: true }), Za.default = void 0;
  var e = q(zb()), t = q(Xb()), i = q(Kb()), s = q(Jb()), l = q(Qb()), p = q(Yb()), m = q(Zb()), w = q(ey()), _ = q(ty()), o = q(ry()), b = q(ay()), L = q(iy()), O = q(sy()), B = q(ny()), G = q(oy()), F = q(ly()), R = q(uy()), v = q(cy()), h = q(dy()), g = q(py()), S = q(fy()), C = q(hy()), x = q(my()), E = q(vy()), D = q(by()), z = q(yy()), Z = q(gy()), A = q(Ry()), N = q(xy()), I = q(_y()), M = q(Cy()), V = q(Py()), c = q(qy()), r = q(Ty()), a = q(Sy()), n = q(wy()), u = q(Ay()), f = q(Ey()), y = q(ky()), P = q(Iy()), T = q(My());
  function q(J) {
    return J && J.__esModule ? J : { default: J };
  }
  var H = [["doc-abstract", e.default], ["doc-acknowledgments", t.default], ["doc-afterword", i.default], ["doc-appendix", s.default], ["doc-backlink", l.default], ["doc-biblioentry", p.default], ["doc-bibliography", m.default], ["doc-biblioref", w.default], ["doc-chapter", _.default], ["doc-colophon", o.default], ["doc-conclusion", b.default], ["doc-cover", L.default], ["doc-credit", O.default], ["doc-credits", B.default], ["doc-dedication", G.default], ["doc-endnote", F.default], ["doc-endnotes", R.default], ["doc-epigraph", v.default], ["doc-epilogue", h.default], ["doc-errata", g.default], ["doc-example", S.default], ["doc-footnote", C.default], ["doc-foreword", x.default], ["doc-glossary", E.default], ["doc-glossref", D.default], ["doc-index", z.default], ["doc-introduction", Z.default], ["doc-noteref", A.default], ["doc-notice", N.default], ["doc-pagebreak", I.default], ["doc-pagefooter", M.default], ["doc-pageheader", V.default], ["doc-pagelist", c.default], ["doc-part", r.default], ["doc-preface", a.default], ["doc-prologue", n.default], ["doc-pullquote", u.default], ["doc-qna", f.default], ["doc-subtitle", y.default], ["doc-tip", P.default], ["doc-toc", T.default]];
  return Za.default = H, Za;
}
var Vi = {}, $i = {}, Od;
function Oy() {
  if (Od) return $i;
  Od = 1, Object.defineProperty($i, "__esModule", { value: true }), $i.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: false, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ module: "GRAPHICS", concept: { name: "graphics-object" } }, { module: "ARIA", concept: { name: "img" } }, { module: "ARIA", concept: { name: "article" } }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "document"]] };
  return $i.default = e, $i;
}
var Hi = {}, Nd;
function Ny() {
  if (Nd) return Hi;
  Nd = 1, Object.defineProperty(Hi, "__esModule", { value: true }), Hi.default = void 0;
  var e = { abstract: false, accessibleNameRequired: false, baseConcepts: [], childrenPresentational: false, nameFrom: ["author", "contents"], prohibitedProps: [], props: { "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [{ module: "GRAPHICS", concept: { name: "graphics-document" } }, { module: "ARIA", concept: { name: "group" } }, { module: "ARIA", concept: { name: "img" } }, { module: "GRAPHICS", concept: { name: "graphics-symbol" } }], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "group"]] };
  return Hi.default = e, Hi;
}
var Ui = {}, Dd;
function Dy() {
  if (Dd) return Ui;
  Dd = 1, Object.defineProperty(Ui, "__esModule", { value: true }), Ui.default = void 0;
  var e = { abstract: false, accessibleNameRequired: true, baseConcepts: [], childrenPresentational: true, nameFrom: ["author"], prohibitedProps: [], props: { "aria-disabled": null, "aria-errormessage": null, "aria-expanded": null, "aria-haspopup": null, "aria-invalid": null }, relatedConcepts: [], requireContextRole: [], requiredContextRole: [], requiredOwnedElements: [], requiredProps: {}, superClass: [["roletype", "structure", "section", "img"]] };
  return Ui.default = e, Ui;
}
var jd;
function jy() {
  if (jd) return Vi;
  jd = 1, Object.defineProperty(Vi, "__esModule", { value: true }), Vi.default = void 0;
  var e = s(Oy()), t = s(Ny()), i = s(Dy());
  function s(p) {
    return p && p.__esModule ? p : { default: p };
  }
  var l = [["graphics-document", e.default], ["graphics-object", t.default], ["graphics-symbol", i.default]];
  return Vi.default = l, Vi;
}
var Bd;
function Il() {
  if (Bd) return ur;
  Bd = 1, Object.defineProperty(ur, "__esModule", { value: true }), ur.default = void 0;
  var e = p(v0()), t = p(Wb()), i = p(Ly()), s = p(jy()), l = p(wo());
  function p(F) {
    return F && F.__esModule ? F : { default: F };
  }
  function m(F, R) {
    var v = typeof Symbol < "u" && F[Symbol.iterator] || F["@@iterator"];
    if (!v) {
      if (Array.isArray(F) || (v = o(F)) || R) {
        v && (F = v);
        var h = 0, g = function() {
        };
        return { s: g, n: function() {
          return h >= F.length ? { done: true } : { done: false, value: F[h++] };
        }, e: function(D) {
          throw D;
        }, f: g };
      }
      throw new TypeError(`Invalid attempt to iterate non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
    }
    var S, C = true, x = false;
    return { s: function() {
      v = v.call(F);
    }, n: function() {
      var D = v.next();
      return C = D.done, D;
    }, e: function(D) {
      x = true, S = D;
    }, f: function() {
      try {
        C || v.return == null || v.return();
      } finally {
        if (x) throw S;
      }
    } };
  }
  function w(F, R) {
    return O(F) || L(F, R) || o(F, R) || _();
  }
  function _() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function o(F, R) {
    if (F) {
      if (typeof F == "string") return b(F, R);
      var v = {}.toString.call(F).slice(8, -1);
      return v === "Object" && F.constructor && (v = F.constructor.name), v === "Map" || v === "Set" ? Array.from(F) : v === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(v) ? b(F, R) : void 0;
    }
  }
  function b(F, R) {
    (R == null || R > F.length) && (R = F.length);
    for (var v = 0, h = Array(R); v < R; v++) h[v] = F[v];
    return h;
  }
  function L(F, R) {
    var v = F == null ? null : typeof Symbol < "u" && F[Symbol.iterator] || F["@@iterator"];
    if (v != null) {
      var h, g, S, C, x = [], E = true, D = false;
      try {
        if (S = (v = v.call(F)).next, R === 0) {
          if (Object(v) !== v) return;
          E = false;
        } else for (; !(E = (h = S.call(v)).done) && (x.push(h.value), x.length !== R); E = true) ;
      } catch (z) {
        D = true, g = z;
      } finally {
        try {
          if (!E && v.return != null && (C = v.return(), Object(C) !== C)) return;
        } finally {
          if (D) throw g;
        }
      }
      return x;
    }
  }
  function O(F) {
    if (Array.isArray(F)) return F;
  }
  var B = [].concat(e.default, t.default, i.default, s.default);
  B.forEach(function(F) {
    var R = w(F, 2), v = R[1], h = m(v.superClass), g;
    try {
      for (h.s(); !(g = h.n()).done; ) {
        var S = g.value, C = m(S), x;
        try {
          var E = function() {
            var z = x.value, Z = B.filter(function(V) {
              var c = w(V, 1), r = c[0];
              return r === z;
            })[0];
            if (Z) for (var A = Z[1], N = 0, I = Object.keys(A.props); N < I.length; N++) {
              var M = I[N];
              Object.prototype.hasOwnProperty.call(v.props, M) || (v.props[M] = A.props[M]);
            }
          };
          for (C.s(); !(x = C.n()).done; ) E();
        } catch (D) {
          C.e(D);
        } finally {
          C.f();
        }
      }
    } catch (D) {
      h.e(D);
    } finally {
      h.f();
    }
  });
  var G = { entries: function() {
    return B;
  }, forEach: function(R) {
    var v = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, h = m(B), g;
    try {
      for (h.s(); !(g = h.n()).done; ) {
        var S = w(g.value, 2), C = S[0], x = S[1];
        R.call(v, x, C, B);
      }
    } catch (E) {
      h.e(E);
    } finally {
      h.f();
    }
  }, get: function(R) {
    var v = B.filter(function(h) {
      return h[0] === R;
    })[0];
    return v && v[1];
  }, has: function(R) {
    return !!G.get(R);
  }, keys: function() {
    return B.map(function(R) {
      var v = w(R, 1), h = v[0];
      return h;
    });
  }, values: function() {
    return B.map(function(R) {
      var v = w(R, 2), h = v[1];
      return h;
    });
  } };
  return ur.default = (0, l.default)(G, G.entries()), ur;
}
var Gi = {}, Fd;
function By() {
  if (Fd) return Gi;
  Fd = 1, Object.defineProperty(Gi, "__esModule", { value: true }), Gi.default = void 0;
  var e = i(wo()), t = i(Il());
  function i(C) {
    return C && C.__esModule ? C : { default: C };
  }
  function s(C, x) {
    return _(C) || w(C, x) || p(C, x) || l();
  }
  function l() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function p(C, x) {
    if (C) {
      if (typeof C == "string") return m(C, x);
      var E = {}.toString.call(C).slice(8, -1);
      return E === "Object" && C.constructor && (E = C.constructor.name), E === "Map" || E === "Set" ? Array.from(C) : E === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(E) ? m(C, x) : void 0;
    }
  }
  function m(C, x) {
    (x == null || x > C.length) && (x = C.length);
    for (var E = 0, D = Array(x); E < x; E++) D[E] = C[E];
    return D;
  }
  function w(C, x) {
    var E = C == null ? null : typeof Symbol < "u" && C[Symbol.iterator] || C["@@iterator"];
    if (E != null) {
      var D, z, Z, A, N = [], I = true, M = false;
      try {
        if (Z = (E = E.call(C)).next, x === 0) {
          if (Object(E) !== E) return;
          I = false;
        } else for (; !(I = (D = Z.call(E)).done) && (N.push(D.value), N.length !== x); I = true) ;
      } catch (V) {
        M = true, z = V;
      } finally {
        try {
          if (!I && E.return != null && (A = E.return(), Object(A) !== A)) return;
        } finally {
          if (M) throw z;
        }
      }
      return N;
    }
  }
  function _(C) {
    if (Array.isArray(C)) return C;
  }
  for (var o = [], b = t.default.keys(), L = 0; L < b.length; L++) {
    var O = b[L], B = t.default.get(O);
    if (B) for (var G = [].concat(B.baseConcepts, B.relatedConcepts), F = function() {
      var x = G[R];
      if (x.module === "HTML") {
        var E = x.concept;
        if (E) {
          var D = o.filter(function(N) {
            return h(N[0], E);
          })[0], z;
          D ? z = D[1] : z = [];
          for (var Z = true, A = 0; A < z.length; A++) if (z[A] === O) {
            Z = false;
            break;
          }
          Z && z.push(O), D || o.push([E, z]);
        }
      }
    }, R = 0; R < G.length; R++) F();
  }
  var v = { entries: function() {
    return o;
  }, forEach: function(x) {
    for (var E = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, D = 0, z = o; D < z.length; D++) {
      var Z = s(z[D], 2), A = Z[0], N = Z[1];
      x.call(E, N, A, o);
    }
  }, get: function(x) {
    var E = o.filter(function(D) {
      return x.name === D[0].name && S(x.attributes, D[0].attributes);
    })[0];
    return E && E[1];
  }, has: function(x) {
    return !!v.get(x);
  }, keys: function() {
    return o.map(function(x) {
      var E = s(x, 1), D = E[0];
      return D;
    });
  }, values: function() {
    return o.map(function(x) {
      var E = s(x, 2), D = E[1];
      return D;
    });
  } };
  function h(C, x) {
    return C.name === x.name && g(C.constraints, x.constraints) && S(C.attributes, x.attributes);
  }
  function g(C, x) {
    if (C === void 0 && x !== void 0 || C !== void 0 && x === void 0) return false;
    if (C !== void 0 && x !== void 0) {
      if (C.length !== x.length) return false;
      for (var E = 0; E < C.length; E++) if (C[E] !== x[E]) return false;
    }
    return true;
  }
  function S(C, x) {
    if (C === void 0 && x !== void 0 || C !== void 0 && x === void 0) return false;
    if (C !== void 0 && x !== void 0) {
      if (C.length !== x.length) return false;
      for (var E = 0; E < C.length; E++) {
        if (C[E].name !== x[E].name || C[E].value !== x[E].value || C[E].constraints === void 0 && x[E].constraints !== void 0 || C[E].constraints !== void 0 && x[E].constraints === void 0) return false;
        if (C[E].constraints !== void 0 && x[E].constraints !== void 0) {
          if (C[E].constraints.length !== x[E].constraints.length) return false;
          for (var D = 0; D < C[E].constraints.length; D++) if (C[E].constraints[D] !== x[E].constraints[D]) return false;
        }
      }
    }
    return true;
  }
  return Gi.default = (0, e.default)(v, v.entries()), Gi;
}
var Wi = {}, Vd;
function Fy() {
  if (Vd) return Wi;
  Vd = 1, Object.defineProperty(Wi, "__esModule", { value: true }), Wi.default = void 0;
  var e = i(wo()), t = i(Il());
  function i(S) {
    return S && S.__esModule ? S : { default: S };
  }
  function s(S, C) {
    return _(S) || w(S, C) || p(S, C) || l();
  }
  function l() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function p(S, C) {
    if (S) {
      if (typeof S == "string") return m(S, C);
      var x = {}.toString.call(S).slice(8, -1);
      return x === "Object" && S.constructor && (x = S.constructor.name), x === "Map" || x === "Set" ? Array.from(S) : x === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(x) ? m(S, C) : void 0;
    }
  }
  function m(S, C) {
    (C == null || C > S.length) && (C = S.length);
    for (var x = 0, E = Array(C); x < C; x++) E[x] = S[x];
    return E;
  }
  function w(S, C) {
    var x = S == null ? null : typeof Symbol < "u" && S[Symbol.iterator] || S["@@iterator"];
    if (x != null) {
      var E, D, z, Z, A = [], N = true, I = false;
      try {
        if (z = (x = x.call(S)).next, C === 0) {
          if (Object(x) !== x) return;
          N = false;
        } else for (; !(N = (E = z.call(x)).done) && (A.push(E.value), A.length !== C); N = true) ;
      } catch (M) {
        I = true, D = M;
      } finally {
        try {
          if (!N && x.return != null && (Z = x.return(), Object(Z) !== Z)) return;
        } finally {
          if (I) throw D;
        }
      }
      return A;
    }
  }
  function _(S) {
    if (Array.isArray(S)) return S;
  }
  for (var o = [], b = t.default.keys(), L = 0; L < b.length; L++) {
    var O = b[L], B = t.default.get(O), G = [];
    if (B) {
      for (var F = [].concat(B.baseConcepts, B.relatedConcepts), R = 0; R < F.length; R++) {
        var v = F[R];
        if (v.module === "HTML") {
          var h = v.concept;
          h != null && G.push(h);
        }
      }
      G.length > 0 && o.push([O, G]);
    }
  }
  var g = { entries: function() {
    return o;
  }, forEach: function(C) {
    for (var x = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, E = 0, D = o; E < D.length; E++) {
      var z = s(D[E], 2), Z = z[0], A = z[1];
      C.call(x, A, Z, o);
    }
  }, get: function(C) {
    var x = o.filter(function(E) {
      return E[0] === C;
    })[0];
    return x && x[1];
  }, has: function(C) {
    return !!g.get(C);
  }, keys: function() {
    return o.map(function(C) {
      var x = s(C, 1), E = x[0];
      return E;
    });
  }, values: function() {
    return o.map(function(C) {
      var x = s(C, 2), E = x[1];
      return E;
    });
  } };
  return Wi.default = (0, e.default)(g, g.entries()), Wi;
}
var $d;
function Vy() {
  if ($d) return lt;
  $d = 1, Object.defineProperty(lt, "__esModule", { value: true }), lt.roles = lt.roleElements = lt.elementRoles = lt.dom = lt.aria = void 0;
  var e = p(r0()), t = p(a0()), i = p(Il()), s = p(By()), l = p(Fy());
  function p(m) {
    return m && m.__esModule ? m : { default: m };
  }
  return lt.aria = e.default, lt.dom = t.default, lt.roles = i.default, lt.elementRoles = s.default, lt.roleElements = l.default, lt;
}
var Ao = Vy(), Rt = {}, zi = {}, Do = {}, Xi = {}, Hd;
function $y() {
  if (Hd) return Xi;
  Hd = 1, Object.defineProperty(Xi, "__esModule", { value: true }), Xi.default = void 0;
  function e() {
    var i = this, s = 0, l = { "@@iterator": function() {
      return l;
    }, next: function() {
      if (s < i.length) {
        var m = i[s];
        return s = s + 1, { done: false, value: m };
      } else return { done: true };
    } };
    return l;
  }
  var t = e;
  return Xi.default = t, Xi;
}
var Ud;
function rl() {
  if (Ud) return Do;
  Ud = 1, Object.defineProperty(Do, "__esModule", { value: true }), Do.default = s;
  var e = t($y());
  function t(l) {
    return l && l.__esModule ? l : { default: l };
  }
  function i(l) {
    "@babel/helpers - typeof";
    return i = typeof Symbol == "function" && typeof Symbol.iterator == "symbol" ? function(p) {
      return typeof p;
    } : function(p) {
      return p && typeof Symbol == "function" && p.constructor === Symbol && p !== Symbol.prototype ? "symbol" : typeof p;
    }, i(l);
  }
  function s(l, p) {
    return typeof Symbol == "function" && i(Symbol.iterator) === "symbol" && Object.defineProperty(l, Symbol.iterator, { value: e.default.bind(p) }), l;
  }
  return Do;
}
var Ki = {}, Ji = {}, Gd;
function Hy() {
  if (Gd) return Ji;
  Gd = 1, Object.defineProperty(Ji, "__esModule", { value: true }), Ji.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "abbr" } }], type: "structure" }, t = e;
  return Ji.default = t, Ji;
}
var Qi = {}, Wd;
function Uy() {
  if (Wd) return Qi;
  Wd = 1, Object.defineProperty(Qi, "__esModule", { value: true }), Qi.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "alertdialog" } }], type: "window" }, t = e;
  return Qi.default = t, Qi;
}
var Yi = {}, zd;
function Gy() {
  if (zd) return Yi;
  zd = 1, Object.defineProperty(Yi, "__esModule", { value: true }), Yi.default = void 0;
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
function zy() {
  if (Kd) return es;
  Kd = 1, Object.defineProperty(es, "__esModule", { value: true }), es.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "application" } }], type: "window" }, t = e;
  return es.default = t, es;
}
var ts = {}, Jd;
function Xy() {
  if (Jd) return ts;
  Jd = 1, Object.defineProperty(ts, "__esModule", { value: true }), ts.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "article" } }, { module: "HTML", concept: { name: "article" } }], type: "structure" }, t = e;
  return ts.default = t, ts;
}
var rs = {}, Qd;
function Ky() {
  if (Qd) return rs;
  Qd = 1, Object.defineProperty(rs, "__esModule", { value: true }), rs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "audio" } }], type: "widget" }, t = e;
  return rs.default = t, rs;
}
var as = {}, Yd;
function Jy() {
  if (Yd) return as;
  Yd = 1, Object.defineProperty(as, "__esModule", { value: true }), as.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "banner" } }], type: "structure" }, t = e;
  return as.default = t, as;
}
var is = {}, Zd;
function Qy() {
  if (Zd) return is;
  Zd = 1, Object.defineProperty(is, "__esModule", { value: true }), is.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "blockquote" } }], type: "structure" }, t = e;
  return is.default = t, is;
}
var ss = {}, ep;
function Yy() {
  if (ep) return ss;
  ep = 1, Object.defineProperty(ss, "__esModule", { value: true }), ss.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { attributes: [{ name: "aria-busy", value: "true" }] } }], type: "widget" }, t = e;
  return ss.default = t, ss;
}
var ns = {}, tp;
function Zy() {
  if (tp) return ns;
  tp = 1, Object.defineProperty(ns, "__esModule", { value: true }), ns.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "button" } }, { module: "HTML", concept: { name: "button" } }], type: "widget" }, t = e;
  return ns.default = t, ns;
}
var os = {}, rp;
function eg() {
  if (rp) return os;
  rp = 1, Object.defineProperty(os, "__esModule", { value: true }), os.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "canvas" } }], type: "widget" }, t = e;
  return os.default = t, os;
}
var ls = {}, ap;
function tg() {
  if (ap) return ls;
  ap = 1, Object.defineProperty(ls, "__esModule", { value: true }), ls.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "caption" } }], type: "structure" }, t = e;
  return ls.default = t, ls;
}
var us = {}, ip;
function rg() {
  if (ip) return us;
  ip = 1, Object.defineProperty(us, "__esModule", { value: true }), us.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "cell" } }, { module: "ARIA", concept: { name: "gridcell" } }, { module: "HTML", concept: { name: "td" } }], type: "widget" }, t = e;
  return us.default = t, us;
}
var cs = {}, sp;
function ag() {
  if (sp) return cs;
  sp = 1, Object.defineProperty(cs, "__esModule", { value: true }), cs.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "checkbox" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "checkbox" }] } }], type: "widget" }, t = e;
  return cs.default = t, cs;
}
var ds = {}, np;
function ig() {
  if (np) return ds;
  np = 1, Object.defineProperty(ds, "__esModule", { value: true }), ds.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "color" }] } }], type: "widget" }, t = e;
  return ds.default = t, ds;
}
var ps = {}, op;
function sg() {
  if (op) return ps;
  op = 1, Object.defineProperty(ps, "__esModule", { value: true }), ps.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "columnheader" } }, { module: "HTML", concept: { name: "th" } }], type: "widget" }, t = e;
  return ps.default = t, ps;
}
var fs = {}, lp;
function ng() {
  if (lp) return fs;
  lp = 1, Object.defineProperty(fs, "__esModule", { value: true }), fs.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return fs.default = t, fs;
}
var hs = {}, up;
function og() {
  if (up) return hs;
  up = 1, Object.defineProperty(hs, "__esModule", { value: true }), hs.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "combobox" } }, { module: "HTML", concept: { name: "select" } }], type: "widget" }, t = e;
  return hs.default = t, hs;
}
var ms = {}, cp;
function lg() {
  if (cp) return ms;
  cp = 1, Object.defineProperty(ms, "__esModule", { value: true }), ms.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "complementary" } }], type: "structure" }, t = e;
  return ms.default = t, ms;
}
var vs = {}, dp;
function ug() {
  if (dp) return vs;
  dp = 1, Object.defineProperty(vs, "__esModule", { value: true }), vs.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "structureinfo" } }], type: "structure" }, t = e;
  return vs.default = t, vs;
}
var bs = {}, pp;
function cg() {
  if (pp) return bs;
  pp = 1, Object.defineProperty(bs, "__esModule", { value: true }), bs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "date" }] } }], type: "widget" }, t = e;
  return bs.default = t, bs;
}
var ys = {}, fp;
function dg() {
  if (fp) return ys;
  fp = 1, Object.defineProperty(ys, "__esModule", { value: true }), ys.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "datetime" }] } }], type: "widget" }, t = e;
  return ys.default = t, ys;
}
var gs = {}, hp;
function pg() {
  if (hp) return gs;
  hp = 1, Object.defineProperty(gs, "__esModule", { value: true }), gs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "dfn" } }], type: "structure" }, t = e;
  return gs.default = t, gs;
}
var Rs = {}, mp;
function fg() {
  if (mp) return Rs;
  mp = 1, Object.defineProperty(Rs, "__esModule", { value: true }), Rs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "dd" } }], type: "structure" }, t = e;
  return Rs.default = t, Rs;
}
var xs = {}, vp;
function hg() {
  if (vp) return xs;
  vp = 1, Object.defineProperty(xs, "__esModule", { value: true }), xs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "dl" } }], type: "structure" }, t = e;
  return xs.default = t, xs;
}
var _s = {}, bp;
function mg() {
  if (bp) return _s;
  bp = 1, Object.defineProperty(_s, "__esModule", { value: true }), _s.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "dt" } }], type: "structure" }, t = e;
  return _s.default = t, _s;
}
var Cs = {}, yp;
function vg() {
  if (yp) return Cs;
  yp = 1, Object.defineProperty(Cs, "__esModule", { value: true }), Cs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "details" } }], type: "structure" }, t = e;
  return Cs.default = t, Cs;
}
var Ps = {}, gp;
function bg() {
  if (gp) return Ps;
  gp = 1, Object.defineProperty(Ps, "__esModule", { value: true }), Ps.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "dialog" } }, { module: "HTML", concept: { name: "dialog" } }], type: "window" }, t = e;
  return Ps.default = t, Ps;
}
var qs = {}, Rp;
function yg() {
  if (Rp) return qs;
  Rp = 1, Object.defineProperty(qs, "__esModule", { value: true }), qs.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "directory" } }, { module: "HTML", concept: { name: "dir" } }], type: "structure" }, t = e;
  return qs.default = t, qs;
}
var Ts = {}, xp;
function gg() {
  if (xp) return Ts;
  xp = 1, Object.defineProperty(Ts, "__esModule", { value: true }), Ts.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { constraints: ["scoped to a details element"], name: "summary" } }], type: "widget" }, t = e;
  return Ts.default = t, Ts;
}
var Ss = {}, _p;
function Rg() {
  if (_p) return Ss;
  _p = 1, Object.defineProperty(Ss, "__esModule", { value: true }), Ss.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "div" } }], type: "generic" }, t = e;
  return Ss.default = t, Ss;
}
var ws = {}, Cp;
function xg() {
  if (Cp) return ws;
  Cp = 1, Object.defineProperty(ws, "__esModule", { value: true }), ws.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "document" } }], type: "structure" }, t = e;
  return ws.default = t, ws;
}
var As = {}, Pp;
function _g() {
  if (Pp) return As;
  Pp = 1, Object.defineProperty(As, "__esModule", { value: true }), As.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "embed" } }], type: "widget" }, t = e;
  return As.default = t, As;
}
var Es = {}, qp;
function Cg() {
  if (qp) return Es;
  qp = 1, Object.defineProperty(Es, "__esModule", { value: true }), Es.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "feed" } }], type: "structure" }, t = e;
  return Es.default = t, Es;
}
var ks = {}, Tp;
function Pg() {
  if (Tp) return ks;
  Tp = 1, Object.defineProperty(ks, "__esModule", { value: true }), ks.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "figcaption" } }], type: "structure" }, t = e;
  return ks.default = t, ks;
}
var Is = {}, Sp;
function qg() {
  if (Sp) return Is;
  Sp = 1, Object.defineProperty(Is, "__esModule", { value: true }), Is.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "figure" } }, { module: "HTML", concept: { name: "figure" } }], type: "structure" }, t = e;
  return Is.default = t, Is;
}
var Ms = {}, wp;
function Tg() {
  if (wp) return Ms;
  wp = 1, Object.defineProperty(Ms, "__esModule", { value: true }), Ms.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "footer" } }], type: "structure" }, t = e;
  return Ms.default = t, Ms;
}
var Ls = {}, Ap;
function Sg() {
  if (Ap) return Ls;
  Ap = 1, Object.defineProperty(Ls, "__esModule", { value: true }), Ls.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "form" } }, { module: "HTML", concept: { name: "form" } }], type: "structure" }, t = e;
  return Ls.default = t, Ls;
}
var Os = {}, Ep;
function wg() {
  if (Ep) return Os;
  Ep = 1, Object.defineProperty(Os, "__esModule", { value: true }), Os.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "grid" } }], type: "widget" }, t = e;
  return Os.default = t, Os;
}
var Ns = {}, kp;
function Ag() {
  if (kp) return Ns;
  kp = 1, Object.defineProperty(Ns, "__esModule", { value: true }), Ns.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "group" } }], type: "structure" }, t = e;
  return Ns.default = t, Ns;
}
var Ds = {}, Ip;
function Eg() {
  if (Ip) return Ds;
  Ip = 1, Object.defineProperty(Ds, "__esModule", { value: true }), Ds.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "heading" } }, { module: "HTML", concept: { name: "h1" } }, { module: "HTML", concept: { name: "h2" } }, { module: "HTML", concept: { name: "h3" } }, { module: "HTML", concept: { name: "h4" } }, { module: "HTML", concept: { name: "h5" } }, { module: "HTML", concept: { name: "h6" } }], type: "structure" }, t = e;
  return Ds.default = t, Ds;
}
var js = {}, Mp;
function kg() {
  if (Mp) return js;
  Mp = 1, Object.defineProperty(js, "__esModule", { value: true }), js.default = void 0;
  var e = { relatedConcepts: [], type: "window" }, t = e;
  return js.default = t, js;
}
var Bs = {}, Lp;
function Ig() {
  if (Lp) return Bs;
  Lp = 1, Object.defineProperty(Bs, "__esModule", { value: true }), Bs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "iframe" } }], type: "window" }, t = e;
  return Bs.default = t, Bs;
}
var Fs = {}, Op;
function Mg() {
  if (Op) return Fs;
  Op = 1, Object.defineProperty(Fs, "__esModule", { value: true }), Fs.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Fs.default = t, Fs;
}
var Vs = {}, Np;
function Lg() {
  if (Np) return Vs;
  Np = 1, Object.defineProperty(Vs, "__esModule", { value: true }), Vs.default = void 0;
  var e = { relatedConcepts: [], type: "widget" }, t = e;
  return Vs.default = t, Vs;
}
var $s = {}, Dp;
function Og() {
  if (Dp) return $s;
  Dp = 1, Object.defineProperty($s, "__esModule", { value: true }), $s.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "img", attributes: [{ name: "usemap" }] } }], type: "structure" }, t = e;
  return $s.default = t, $s;
}
var Hs = {}, jp;
function Ng() {
  if (jp) return Hs;
  jp = 1, Object.defineProperty(Hs, "__esModule", { value: true }), Hs.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "img" } }, { module: "HTML", concept: { name: "img" } }], type: "structure" }, t = e;
  return Hs.default = t, Hs;
}
var Us = {}, Bp;
function Dg() {
  if (Bp) return Us;
  Bp = 1, Object.defineProperty(Us, "__esModule", { value: true }), Us.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "input" } }], type: "widget" }, t = e;
  return Us.default = t, Us;
}
var Gs = {}, Fp;
function jg() {
  if (Fp) return Gs;
  Fp = 1, Object.defineProperty(Gs, "__esModule", { value: true }), Gs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "time" }] } }], type: "widget" }, t = e;
  return Gs.default = t, Gs;
}
var Ws = {}, Vp;
function Bg() {
  if (Vp) return Ws;
  Vp = 1, Object.defineProperty(Ws, "__esModule", { value: true }), Ws.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "label" } }], type: "structure" }, t = e;
  return Ws.default = t, Ws;
}
var zs = {}, $p;
function Fg() {
  if ($p) return zs;
  $p = 1, Object.defineProperty(zs, "__esModule", { value: true }), zs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "legend" } }], type: "structure" }, t = e;
  return zs.default = t, zs;
}
var Xs = {}, Hp;
function Vg() {
  if (Hp) return Xs;
  Hp = 1, Object.defineProperty(Xs, "__esModule", { value: true }), Xs.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "br" } }], type: "structure" }, t = e;
  return Xs.default = t, Xs;
}
var Ks = {}, Up;
function $g() {
  if (Up) return Ks;
  Up = 1, Object.defineProperty(Ks, "__esModule", { value: true }), Ks.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "link" } }, { module: "HTML", concept: { name: "a", attributes: [{ name: "href" }] } }], type: "widget" }, t = e;
  return Ks.default = t, Ks;
}
var Js = {}, Gp;
function Hg() {
  if (Gp) return Js;
  Gp = 1, Object.defineProperty(Js, "__esModule", { value: true }), Js.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "option" } }, { module: "HTML", concept: { name: "option" } }], type: "widget" }, t = e;
  return Js.default = t, Js;
}
var Qs = {}, Wp;
function Ug() {
  if (Wp) return Qs;
  Wp = 1, Object.defineProperty(Qs, "__esModule", { value: true }), Qs.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "listbox" } }, { module: "HTML", concept: { name: "datalist" } }, { module: "HTML", concept: { name: "select" } }], type: "widget" }, t = e;
  return Qs.default = t, Qs;
}
var Ys = {}, zp;
function Gg() {
  if (zp) return Ys;
  zp = 1, Object.defineProperty(Ys, "__esModule", { value: true }), Ys.default = void 0;
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
function zg() {
  if (Kp) return en;
  Kp = 1, Object.defineProperty(en, "__esModule", { value: true }), en.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "list" } }, { module: "HTML", concept: { name: "ul" } }, { module: "HTML", concept: { name: "ol" } }], type: "structure" }, t = e;
  return en.default = t, en;
}
var tn = {}, Jp;
function Xg() {
  if (Jp) return tn;
  Jp = 1, Object.defineProperty(tn, "__esModule", { value: true }), tn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "log" } }], type: "structure" }, t = e;
  return tn.default = t, tn;
}
var rn = {}, Qp;
function Kg() {
  if (Qp) return rn;
  Qp = 1, Object.defineProperty(rn, "__esModule", { value: true }), rn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "main" } }, { module: "HTML", concept: { name: "main" } }], type: "structure" }, t = e;
  return rn.default = t, rn;
}
var an = {}, Yp;
function Jg() {
  if (Yp) return an;
  Yp = 1, Object.defineProperty(an, "__esModule", { value: true }), an.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "mark" } }], type: "structure" }, t = e;
  return an.default = t, an;
}
var sn = {}, Zp;
function Qg() {
  if (Zp) return sn;
  Zp = 1, Object.defineProperty(sn, "__esModule", { value: true }), sn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "marquee" } }, { module: "HTML", concept: { name: "marquee" } }], type: "structure" }, t = e;
  return sn.default = t, sn;
}
var nn = {}, ef;
function Yg() {
  if (ef) return nn;
  ef = 1, Object.defineProperty(nn, "__esModule", { value: true }), nn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "math" } }], type: "structure" }, t = e;
  return nn.default = t, nn;
}
var on = {}, tf;
function Zg() {
  if (tf) return on;
  tf = 1, Object.defineProperty(on, "__esModule", { value: true }), on.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "menubar" } }], type: "structure" }, t = e;
  return on.default = t, on;
}
var ln = {}, rf;
function eR() {
  if (rf) return ln;
  rf = 1, Object.defineProperty(ln, "__esModule", { value: true }), ln.default = void 0;
  var e = { relatedConcepts: [], type: "widget" }, t = e;
  return ln.default = t, ln;
}
var un = {}, af;
function tR() {
  if (af) return un;
  af = 1, Object.defineProperty(un, "__esModule", { value: true }), un.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "menuitem" } }, { module: "HTML", concept: { name: "menuitem" } }], type: "widget" }, t = e;
  return un.default = t, un;
}
var cn = {}, sf;
function rR() {
  if (sf) return cn;
  sf = 1, Object.defineProperty(cn, "__esModule", { value: true }), cn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "menuitemcheckbox" } }], type: "widget" }, t = e;
  return cn.default = t, cn;
}
var dn = {}, nf;
function aR() {
  if (nf) return dn;
  nf = 1, Object.defineProperty(dn, "__esModule", { value: true }), dn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "menuitemradio" } }], type: "widget" }, t = e;
  return dn.default = t, dn;
}
var pn = {}, of;
function iR() {
  if (of) return pn;
  of = 1, Object.defineProperty(pn, "__esModule", { value: true }), pn.default = void 0;
  var e = { relatedConcepts: [], type: "widget" }, t = e;
  return pn.default = t, pn;
}
var fn = {}, lf;
function sR() {
  if (lf) return fn;
  lf = 1, Object.defineProperty(fn, "__esModule", { value: true }), fn.default = void 0;
  var e = { relatedConcepts: [], type: "widget" }, t = e;
  return fn.default = t, fn;
}
var hn = {}, uf;
function nR() {
  if (uf) return hn;
  uf = 1, Object.defineProperty(hn, "__esModule", { value: true }), hn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "menu" } }, { module: "HTML", concept: { name: "menu" } }], type: "structure" }, t = e;
  return hn.default = t, hn;
}
var mn = {}, cf;
function oR() {
  if (cf) return mn;
  cf = 1, Object.defineProperty(mn, "__esModule", { value: true }), mn.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "meter" } }], type: "structure" }, t = e;
  return mn.default = t, mn;
}
var vn = {}, df;
function lR() {
  if (df) return vn;
  df = 1, Object.defineProperty(vn, "__esModule", { value: true }), vn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "navigation" } }, { module: "HTML", concept: { name: "nav" } }], type: "structure" }, t = e;
  return vn.default = t, vn;
}
var bn = {}, pf;
function uR() {
  if (pf) return bn;
  pf = 1, Object.defineProperty(bn, "__esModule", { value: true }), bn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "none" } }], type: "structure" }, t = e;
  return bn.default = t, bn;
}
var yn = {}, ff;
function cR() {
  if (ff) return yn;
  ff = 1, Object.defineProperty(yn, "__esModule", { value: true }), yn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "note" } }], type: "structure" }, t = e;
  return yn.default = t, yn;
}
var gn = {}, hf;
function dR() {
  if (hf) return gn;
  hf = 1, Object.defineProperty(gn, "__esModule", { value: true }), gn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return gn.default = t, gn;
}
var Rn = {}, mf;
function pR() {
  if (mf) return Rn;
  mf = 1, Object.defineProperty(Rn, "__esModule", { value: true }), Rn.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "p" } }], type: "structure" }, t = e;
  return Rn.default = t, Rn;
}
var xn = {}, vf;
function fR() {
  if (vf) return xn;
  vf = 1, Object.defineProperty(xn, "__esModule", { value: true }), xn.default = void 0;
  var e = { relatedConcepts: [], type: "widget" }, t = e;
  return xn.default = t, xn;
}
var _n = {}, bf;
function hR() {
  if (bf) return _n;
  bf = 1, Object.defineProperty(_n, "__esModule", { value: true }), _n.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "pre" } }], type: "structure" }, t = e;
  return _n.default = t, _n;
}
var Cn = {}, yf;
function mR() {
  if (yf) return Cn;
  yf = 1, Object.defineProperty(Cn, "__esModule", { value: true }), Cn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "presentation" } }], type: "structure" }, t = e;
  return Cn.default = t, Cn;
}
var Pn = {}, gf;
function vR() {
  if (gf) return Pn;
  gf = 1, Object.defineProperty(Pn, "__esModule", { value: true }), Pn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "progressbar" } }, { module: "HTML", concept: { name: "progress" } }], type: "structure" }, t = e;
  return Pn.default = t, Pn;
}
var qn = {}, Rf;
function bR() {
  if (Rf) return qn;
  Rf = 1, Object.defineProperty(qn, "__esModule", { value: true }), qn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "radio" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "radio" }] } }], type: "widget" }, t = e;
  return qn.default = t, qn;
}
var Tn = {}, xf;
function yR() {
  if (xf) return Tn;
  xf = 1, Object.defineProperty(Tn, "__esModule", { value: true }), Tn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "radiogroup" } }], type: "structure" }, t = e;
  return Tn.default = t, Tn;
}
var Sn = {}, _f;
function gR() {
  if (_f) return Sn;
  _f = 1, Object.defineProperty(Sn, "__esModule", { value: true }), Sn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "region" } }], type: "structure" }, t = e;
  return Sn.default = t, Sn;
}
var wn = {}, Cf;
function RR() {
  if (Cf) return wn;
  Cf = 1, Object.defineProperty(wn, "__esModule", { value: true }), wn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return wn.default = t, wn;
}
var An = {}, Pf;
function xR() {
  if (Pf) return An;
  Pf = 1, Object.defineProperty(An, "__esModule", { value: true }), An.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "rowheader" } }, { module: "HTML", concept: { name: "th", attributes: [{ name: "scope", value: "row" }] } }], type: "widget" }, t = e;
  return An.default = t, An;
}
var En = {}, qf;
function _R() {
  if (qf) return En;
  qf = 1, Object.defineProperty(En, "__esModule", { value: true }), En.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "row" } }, { module: "HTML", concept: { name: "tr" } }], type: "structure" }, t = e;
  return En.default = t, En;
}
var kn = {}, Tf;
function CR() {
  if (Tf) return kn;
  Tf = 1, Object.defineProperty(kn, "__esModule", { value: true }), kn.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "ruby" } }], type: "structure" }, t = e;
  return kn.default = t, kn;
}
var In = {}, Sf;
function PR() {
  if (Sf) return In;
  Sf = 1, Object.defineProperty(In, "__esModule", { value: true }), In.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return In.default = t, In;
}
var Mn = {}, wf;
function qR() {
  if (wf) return Mn;
  wf = 1, Object.defineProperty(Mn, "__esModule", { value: true }), Mn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Mn.default = t, Mn;
}
var Ln = {}, Af;
function TR() {
  if (Af) return Ln;
  Af = 1, Object.defineProperty(Ln, "__esModule", { value: true }), Ln.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "scrollbar" } }], type: "widget" }, t = e;
  return Ln.default = t, Ln;
}
var On = {}, Ef;
function SR() {
  if (Ef) return On;
  Ef = 1, Object.defineProperty(On, "__esModule", { value: true }), On.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return On.default = t, On;
}
var Nn = {}, kf;
function wR() {
  if (kf) return Nn;
  kf = 1, Object.defineProperty(Nn, "__esModule", { value: true }), Nn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "search" } }], type: "structure" }, t = e;
  return Nn.default = t, Nn;
}
var Dn = {}, If;
function AR() {
  if (If) return Dn;
  If = 1, Object.defineProperty(Dn, "__esModule", { value: true }), Dn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "searchbox" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "search" }] } }], type: "widget" }, t = e;
  return Dn.default = t, Dn;
}
var jn = {}, Mf;
function ER() {
  if (Mf) return jn;
  Mf = 1, Object.defineProperty(jn, "__esModule", { value: true }), jn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "slider" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "range" }] } }], type: "widget" }, t = e;
  return jn.default = t, jn;
}
var Bn = {}, Lf;
function kR() {
  if (Lf) return Bn;
  Lf = 1, Object.defineProperty(Bn, "__esModule", { value: true }), Bn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Bn.default = t, Bn;
}
var Fn = {}, Of;
function IR() {
  if (Of) return Fn;
  Of = 1, Object.defineProperty(Fn, "__esModule", { value: true }), Fn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "spinbutton" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "number" }] } }], type: "widget" }, t = e;
  return Fn.default = t, Fn;
}
var Vn = {}, Nf;
function MR() {
  if (Nf) return Vn;
  Nf = 1, Object.defineProperty(Vn, "__esModule", { value: true }), Vn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Vn.default = t, Vn;
}
var $n = {}, Df;
function LR() {
  if (Df) return $n;
  Df = 1, Object.defineProperty($n, "__esModule", { value: true }), $n.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "separator" } }], type: "widget" }, t = e;
  return $n.default = t, $n;
}
var Hn = {}, jf;
function OR() {
  if (jf) return Hn;
  jf = 1, Object.defineProperty(Hn, "__esModule", { value: true }), Hn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Hn.default = t, Hn;
}
var Un = {}, Bf;
function NR() {
  if (Bf) return Un;
  Bf = 1, Object.defineProperty(Un, "__esModule", { value: true }), Un.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "status" } }], type: "structure" }, t = e;
  return Un.default = t, Un;
}
var Gn = {}, Ff;
function DR() {
  if (Ff) return Gn;
  Ff = 1, Object.defineProperty(Gn, "__esModule", { value: true }), Gn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Gn.default = t, Gn;
}
var Wn = {}, Vf;
function jR() {
  if (Vf) return Wn;
  Vf = 1, Object.defineProperty(Wn, "__esModule", { value: true }), Wn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "switch" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "checkbox" }] } }], type: "widget" }, t = e;
  return Wn.default = t, Wn;
}
var zn = {}, $f;
function BR() {
  if ($f) return zn;
  $f = 1, Object.defineProperty(zn, "__esModule", { value: true }), zn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "tablist" } }], type: "structure" }, t = e;
  return zn.default = t, zn;
}
var Xn = {}, Hf;
function FR() {
  if (Hf) return Xn;
  Hf = 1, Object.defineProperty(Xn, "__esModule", { value: true }), Xn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "tab" } }], type: "widget" }, t = e;
  return Xn.default = t, Xn;
}
var Kn = {}, Uf;
function VR() {
  if (Uf) return Kn;
  Uf = 1, Object.defineProperty(Kn, "__esModule", { value: true }), Kn.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return Kn.default = t, Kn;
}
var Jn = {}, Gf;
function $R() {
  if (Gf) return Jn;
  Gf = 1, Object.defineProperty(Jn, "__esModule", { value: true }), Jn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "table" } }, { module: "HTML", concept: { name: "table" } }], type: "structure" }, t = e;
  return Jn.default = t, Jn;
}
var Qn = {}, Wf;
function HR() {
  if (Wf) return Qn;
  Wf = 1, Object.defineProperty(Qn, "__esModule", { value: true }), Qn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "tablist" } }], type: "structure" }, t = e;
  return Qn.default = t, Qn;
}
var Yn = {}, zf;
function UR() {
  if (zf) return Yn;
  zf = 1, Object.defineProperty(Yn, "__esModule", { value: true }), Yn.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "tabpanel" } }], type: "structure" }, t = e;
  return Yn.default = t, Yn;
}
var Zn = {}, Xf;
function GR() {
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
function zR() {
  if (Jf) return to;
  Jf = 1, Object.defineProperty(to, "__esModule", { value: true }), to.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "textbox" } }, { module: "HTML", concept: { name: "input" } }, { module: "HTML", concept: { name: "input", attributes: [{ name: "type", value: "text" }] } }], type: "widget" }, t = e;
  return to.default = t, to;
}
var ro = {}, Qf;
function XR() {
  if (Qf) return ro;
  Qf = 1, Object.defineProperty(ro, "__esModule", { value: true }), ro.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "time" } }], type: "structure" }, t = e;
  return ro.default = t, ro;
}
var ao = {}, Yf;
function KR() {
  if (Yf) return ao;
  Yf = 1, Object.defineProperty(ao, "__esModule", { value: true }), ao.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "timer" } }], type: "structure" }, t = e;
  return ao.default = t, ao;
}
var io = {}, Zf;
function JR() {
  if (Zf) return io;
  Zf = 1, Object.defineProperty(io, "__esModule", { value: true }), io.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { attributes: [{ name: "aria-pressed" }] } }], type: "widget" }, t = e;
  return io.default = t, io;
}
var so = {}, eh;
function QR() {
  if (eh) return so;
  eh = 1, Object.defineProperty(so, "__esModule", { value: true }), so.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "toolbar" } }], type: "structure" }, t = e;
  return so.default = t, so;
}
var no = {}, th;
function YR() {
  if (th) return no;
  th = 1, Object.defineProperty(no, "__esModule", { value: true }), no.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "tree" } }], type: "widget" }, t = e;
  return no.default = t, no;
}
var oo = {}, rh;
function ZR() {
  if (rh) return oo;
  rh = 1, Object.defineProperty(oo, "__esModule", { value: true }), oo.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "treegrid" } }], type: "widget" }, t = e;
  return oo.default = t, oo;
}
var lo = {}, ah;
function ex() {
  if (ah) return lo;
  ah = 1, Object.defineProperty(lo, "__esModule", { value: true }), lo.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "treeitem" } }], type: "widget" }, t = e;
  return lo.default = t, lo;
}
var uo = {}, ih;
function tx() {
  if (ih) return uo;
  ih = 1, Object.defineProperty(uo, "__esModule", { value: true }), uo.default = void 0;
  var e = { relatedConcepts: [{ module: "ARIA", concept: { name: "tooltip" } }], type: "structure" }, t = e;
  return uo.default = t, uo;
}
var co = {}, sh;
function rx() {
  if (sh) return co;
  sh = 1, Object.defineProperty(co, "__esModule", { value: true }), co.default = void 0;
  var e = { relatedConcepts: [{ module: "HTML", concept: { name: "video" } }], type: "widget" }, t = e;
  return co.default = t, co;
}
var po = {}, nh;
function ax() {
  if (nh) return po;
  nh = 1, Object.defineProperty(po, "__esModule", { value: true }), po.default = void 0;
  var e = { relatedConcepts: [], type: "structure" }, t = e;
  return po.default = t, po;
}
var fo = {}, oh;
function ix() {
  if (oh) return fo;
  oh = 1, Object.defineProperty(fo, "__esModule", { value: true }), fo.default = void 0;
  var e = { relatedConcepts: [], type: "window" }, t = e;
  return fo.default = t, fo;
}
var lh;
function al() {
  if (lh) return Ki;
  lh = 1, Object.defineProperty(Ki, "__esModule", { value: true }), Ki.default = void 0;
  var e = U(rl()), t = U(Hy()), i = U(Uy()), s = U(Gy()), l = U(Wy()), p = U(zy()), m = U(Xy()), w = U(Ky()), _ = U(Jy()), o = U(Qy()), b = U(Yy()), L = U(Zy()), O = U(eg()), B = U(tg()), G = U(rg()), F = U(ag()), R = U(ig()), v = U(sg()), h = U(ng()), g = U(og()), S = U(lg()), C = U(ug()), x = U(cg()), E = U(dg()), D = U(pg()), z = U(fg()), Z = U(hg()), A = U(mg()), N = U(vg()), I = U(bg()), M = U(yg()), V = U(gg()), c = U(Rg()), r = U(xg()), a = U(_g()), n = U(Cg()), u = U(Pg()), f = U(qg()), y = U(Tg()), P = U(Sg()), T = U(wg()), q = U(Ag()), H = U(Eg()), J = U(kg()), re = U(Ig()), Q = U(Mg()), pe = U(Lg()), de = U(Og()), fe = U(Ng()), he = U(Dg()), we = U(jg()), Oe = U(Bg()), Ee = U(Fg()), Ue = U(Vg()), Ne = U($g()), Ze = U(Hg()), Je = U(Ug()), vt = U(Gg()), bt = U(Wg()), j = U(zg()), X = U(Xg()), K = U(Kg()), ee = U(Jg()), ce = U(Qg()), se = U(Yg()), Re = U(Zg()), ke = U(eR()), ge = U(tR()), Ae = U(rR()), ze = U(aR()), je = U(iR()), De = U(sR()), Ge = U(nR()), te = U(oR()), Ie = U(lR()), be = U(uR()), Qe = U(cR()), tt = U(dR()), Xe = U(pR()), We = U(fR()), Ye = U(hR()), St = U(mR()), Ht = U(vR()), Y = U(bR()), yt = U(yR()), gt = U(gR()), Eo = U(RR()), ko = U(xR()), Io = U(_R()), gm = U(CR()), Rm = U(PR()), xm = U(qR()), _m = U(TR()), Cm = U(SR()), Pm = U(wR()), qm = U(AR()), Tm = U(ER()), Sm = U(kR()), wm = U(IR()), Am = U(MR()), Em = U(LR()), km = U(OR()), Im = U(NR()), Mm = U(DR()), Lm = U(jR()), Om = U(BR()), Nm = U(FR()), Dm = U(VR()), jm = U($R()), Bm = U(HR()), Fm = U(UR()), Vm = U(GR()), $m = U(WR()), Hm = U(zR()), Um = U(XR()), Gm = U(KR()), Wm = U(JR()), zm = U(QR()), Xm = U(YR()), Km = U(ZR()), Jm = U(ex()), Qm = U(tx()), Ym = U(rx()), Zm = U(ax()), ev = U(ix());
  function U(qe) {
    return qe && qe.__esModule ? qe : { default: qe };
  }
  function il(qe, Ve) {
    return iv(qe) || av(qe, Ve) || rv(qe, Ve) || tv();
  }
  function tv() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function rv(qe, Ve) {
    if (qe) {
      if (typeof qe == "string") return Ol(qe, Ve);
      var Be = Object.prototype.toString.call(qe).slice(8, -1);
      if (Be === "Object" && qe.constructor && (Be = qe.constructor.name), Be === "Map" || Be === "Set") return Array.from(qe);
      if (Be === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(Be)) return Ol(qe, Ve);
    }
  }
  function Ol(qe, Ve) {
    (Ve == null || Ve > qe.length) && (Ve = qe.length);
    for (var Be = 0, et = new Array(Ve); Be < Ve; Be++) et[Be] = qe[Be];
    return et;
  }
  function av(qe, Ve) {
    var Be = qe == null ? null : typeof Symbol < "u" && qe[Symbol.iterator] || qe["@@iterator"];
    if (Be != null) {
      var et = [], Gt = true, ar = false, Mo, Lo;
      try {
        for (Be = Be.call(qe); !(Gt = (Mo = Be.next()).done) && (et.push(Mo.value), !(Ve && et.length === Ve)); Gt = true) ;
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
  function iv(qe) {
    if (Array.isArray(qe)) return qe;
  }
  var Ut = [["AbbrRole", t.default], ["AlertDialogRole", i.default], ["AlertRole", s.default], ["AnnotationRole", l.default], ["ApplicationRole", p.default], ["ArticleRole", m.default], ["AudioRole", w.default], ["BannerRole", _.default], ["BlockquoteRole", o.default], ["BusyIndicatorRole", b.default], ["ButtonRole", L.default], ["CanvasRole", O.default], ["CaptionRole", B.default], ["CellRole", G.default], ["CheckBoxRole", F.default], ["ColorWellRole", R.default], ["ColumnHeaderRole", v.default], ["ColumnRole", h.default], ["ComboBoxRole", g.default], ["ComplementaryRole", S.default], ["ContentInfoRole", C.default], ["DateRole", x.default], ["DateTimeRole", E.default], ["DefinitionRole", D.default], ["DescriptionListDetailRole", z.default], ["DescriptionListRole", Z.default], ["DescriptionListTermRole", A.default], ["DetailsRole", N.default], ["DialogRole", I.default], ["DirectoryRole", M.default], ["DisclosureTriangleRole", V.default], ["DivRole", c.default], ["DocumentRole", r.default], ["EmbeddedObjectRole", a.default], ["FeedRole", n.default], ["FigcaptionRole", u.default], ["FigureRole", f.default], ["FooterRole", y.default], ["FormRole", P.default], ["GridRole", T.default], ["GroupRole", q.default], ["HeadingRole", H.default], ["IframePresentationalRole", J.default], ["IframeRole", re.default], ["IgnoredRole", Q.default], ["ImageMapLinkRole", pe.default], ["ImageMapRole", de.default], ["ImageRole", fe.default], ["InlineTextBoxRole", he.default], ["InputTimeRole", we.default], ["LabelRole", Oe.default], ["LegendRole", Ee.default], ["LineBreakRole", Ue.default], ["LinkRole", Ne.default], ["ListBoxOptionRole", Ze.default], ["ListBoxRole", Je.default], ["ListItemRole", vt.default], ["ListMarkerRole", bt.default], ["ListRole", j.default], ["LogRole", X.default], ["MainRole", K.default], ["MarkRole", ee.default], ["MarqueeRole", ce.default], ["MathRole", se.default], ["MenuBarRole", Re.default], ["MenuButtonRole", ke.default], ["MenuItemRole", ge.default], ["MenuItemCheckBoxRole", Ae.default], ["MenuItemRadioRole", ze.default], ["MenuListOptionRole", je.default], ["MenuListPopupRole", De.default], ["MenuRole", Ge.default], ["MeterRole", te.default], ["NavigationRole", Ie.default], ["NoneRole", be.default], ["NoteRole", Qe.default], ["OutlineRole", tt.default], ["ParagraphRole", Xe.default], ["PopUpButtonRole", We.default], ["PreRole", Ye.default], ["PresentationalRole", St.default], ["ProgressIndicatorRole", Ht.default], ["RadioButtonRole", Y.default], ["RadioGroupRole", yt.default], ["RegionRole", gt.default], ["RootWebAreaRole", Eo.default], ["RowHeaderRole", ko.default], ["RowRole", Io.default], ["RubyRole", gm.default], ["RulerRole", Rm.default], ["ScrollAreaRole", xm.default], ["ScrollBarRole", _m.default], ["SeamlessWebAreaRole", Cm.default], ["SearchRole", Pm.default], ["SearchBoxRole", qm.default], ["SliderRole", Tm.default], ["SliderThumbRole", Sm.default], ["SpinButtonRole", wm.default], ["SpinButtonPartRole", Am.default], ["SplitterRole", Em.default], ["StaticTextRole", km.default], ["StatusRole", Im.default], ["SVGRootRole", Mm.default], ["SwitchRole", Lm.default], ["TabGroupRole", Om.default], ["TabRole", Nm.default], ["TableHeaderContainerRole", Dm.default], ["TableRole", jm.default], ["TabListRole", Bm.default], ["TabPanelRole", Fm.default], ["TermRole", Vm.default], ["TextAreaRole", $m.default], ["TextFieldRole", Hm.default], ["TimeRole", Um.default], ["TimerRole", Gm.default], ["ToggleButtonRole", Wm.default], ["ToolbarRole", zm.default], ["TreeRole", Xm.default], ["TreeGridRole", Km.default], ["TreeItemRole", Jm.default], ["UserInterfaceTooltipRole", Qm.default], ["VideoRole", Ym.default], ["WebAreaRole", Zm.default], ["WindowRole", ev.default]], sl = { entries: function() {
    return Ut;
  }, forEach: function(Ve) {
    for (var Be = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, et = 0, Gt = Ut; et < Gt.length; et++) {
      var ar = il(Gt[et], 2), Mo = ar[0], Lo = ar[1];
      Ve.call(Be, Lo, Mo, Ut);
    }
  }, get: function(Ve) {
    var Be = Ut.find(function(et) {
      return et[0] === Ve;
    });
    return Be && Be[1];
  }, has: function(Ve) {
    return !!sl.get(Ve);
  }, keys: function() {
    return Ut.map(function(Ve) {
      var Be = il(Ve, 1), et = Be[0];
      return et;
    });
  }, values: function() {
    return Ut.map(function(Ve) {
      var Be = il(Ve, 2), et = Be[1];
      return et;
    });
  } }, sv = (0, e.default)(sl, sl.entries());
  return Ki.default = sv, Ki;
}
var uh;
function sx() {
  if (uh) return zi;
  uh = 1, Object.defineProperty(zi, "__esModule", { value: true }), zi.default = void 0;
  var e = i(rl()), t = i(al());
  function i(R) {
    return R && R.__esModule ? R : { default: R };
  }
  function s(R, v) {
    return m(R) || p(R, v) || _(R, v) || l();
  }
  function l() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function p(R, v) {
    var h = R == null ? null : typeof Symbol < "u" && R[Symbol.iterator] || R["@@iterator"];
    if (h != null) {
      var g = [], S = true, C = false, x, E;
      try {
        for (h = h.call(R); !(S = (x = h.next()).done) && (g.push(x.value), !(v && g.length === v)); S = true) ;
      } catch (D) {
        C = true, E = D;
      } finally {
        try {
          !S && h.return != null && h.return();
        } finally {
          if (C) throw E;
        }
      }
      return g;
    }
  }
  function m(R) {
    if (Array.isArray(R)) return R;
  }
  function w(R, v) {
    var h = typeof Symbol < "u" && R[Symbol.iterator] || R["@@iterator"];
    if (!h) {
      if (Array.isArray(R) || (h = _(R)) || v) {
        h && (R = h);
        var g = 0, S = function() {
        };
        return { s: S, n: function() {
          return g >= R.length ? { done: true } : { done: false, value: R[g++] };
        }, e: function(z) {
          throw z;
        }, f: S };
      }
      throw new TypeError(`Invalid attempt to iterate non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
    }
    var C = true, x = false, E;
    return { s: function() {
      h = h.call(R);
    }, n: function() {
      var z = h.next();
      return C = z.done, z;
    }, e: function(z) {
      x = true, E = z;
    }, f: function() {
      try {
        !C && h.return != null && h.return();
      } finally {
        if (x) throw E;
      }
    } };
  }
  function _(R, v) {
    if (R) {
      if (typeof R == "string") return o(R, v);
      var h = Object.prototype.toString.call(R).slice(8, -1);
      if (h === "Object" && R.constructor && (h = R.constructor.name), h === "Map" || h === "Set") return Array.from(R);
      if (h === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(h)) return o(R, v);
    }
  }
  function o(R, v) {
    (v == null || v > R.length) && (v = R.length);
    for (var h = 0, g = new Array(v); h < v; h++) g[h] = R[h];
    return g;
  }
  var b = [], L = w(t.default.entries()), O;
  try {
    var B = function() {
      var v = s(O.value, 2), h = v[0], g = v[1], S = g.relatedConcepts;
      Array.isArray(S) && S.forEach(function(C) {
        if (C.module === "HTML") {
          var x = C.concept;
          if (x) {
            var E = b.findIndex(function(D) {
              var z = s(D, 1), Z = z[0];
              return Z === h;
            });
            E === -1 && (b.push([h, []]), E = b.length - 1), b[E][1].push(x);
          }
        }
      });
    };
    for (L.s(); !(O = L.n()).done; ) B();
  } catch (R) {
    L.e(R);
  } finally {
    L.f();
  }
  var G = { entries: function() {
    return b;
  }, forEach: function(v) {
    for (var h = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, g = 0, S = b; g < S.length; g++) {
      var C = s(S[g], 2), x = C[0], E = C[1];
      v.call(h, E, x, b);
    }
  }, get: function(v) {
    var h = b.find(function(g) {
      return g[0] === v;
    });
    return h && h[1];
  }, has: function(v) {
    return !!G.get(v);
  }, keys: function() {
    return b.map(function(v) {
      var h = s(v, 1), g = h[0];
      return g;
    });
  }, values: function() {
    return b.map(function(v) {
      var h = s(v, 2), g = h[1];
      return g;
    });
  } }, F = (0, e.default)(G, G.entries());
  return zi.default = F, zi;
}
var ho = {}, ch;
function nx() {
  if (ch) return ho;
  ch = 1, Object.defineProperty(ho, "__esModule", { value: true }), ho.default = void 0;
  var e = i(rl()), t = i(al());
  function i(R) {
    return R && R.__esModule ? R : { default: R };
  }
  function s(R, v) {
    return m(R) || p(R, v) || _(R, v) || l();
  }
  function l() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function p(R, v) {
    var h = R == null ? null : typeof Symbol < "u" && R[Symbol.iterator] || R["@@iterator"];
    if (h != null) {
      var g = [], S = true, C = false, x, E;
      try {
        for (h = h.call(R); !(S = (x = h.next()).done) && (g.push(x.value), !(v && g.length === v)); S = true) ;
      } catch (D) {
        C = true, E = D;
      } finally {
        try {
          !S && h.return != null && h.return();
        } finally {
          if (C) throw E;
        }
      }
      return g;
    }
  }
  function m(R) {
    if (Array.isArray(R)) return R;
  }
  function w(R, v) {
    var h = typeof Symbol < "u" && R[Symbol.iterator] || R["@@iterator"];
    if (!h) {
      if (Array.isArray(R) || (h = _(R)) || v) {
        h && (R = h);
        var g = 0, S = function() {
        };
        return { s: S, n: function() {
          return g >= R.length ? { done: true } : { done: false, value: R[g++] };
        }, e: function(z) {
          throw z;
        }, f: S };
      }
      throw new TypeError(`Invalid attempt to iterate non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
    }
    var C = true, x = false, E;
    return { s: function() {
      h = h.call(R);
    }, n: function() {
      var z = h.next();
      return C = z.done, z;
    }, e: function(z) {
      x = true, E = z;
    }, f: function() {
      try {
        !C && h.return != null && h.return();
      } finally {
        if (x) throw E;
      }
    } };
  }
  function _(R, v) {
    if (R) {
      if (typeof R == "string") return o(R, v);
      var h = Object.prototype.toString.call(R).slice(8, -1);
      if (h === "Object" && R.constructor && (h = R.constructor.name), h === "Map" || h === "Set") return Array.from(R);
      if (h === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(h)) return o(R, v);
    }
  }
  function o(R, v) {
    (v == null || v > R.length) && (v = R.length);
    for (var h = 0, g = new Array(v); h < v; h++) g[h] = R[h];
    return g;
  }
  var b = [], L = w(t.default.entries()), O;
  try {
    var B = function() {
      var v = s(O.value, 2), h = v[0], g = v[1], S = g.relatedConcepts;
      Array.isArray(S) && S.forEach(function(C) {
        if (C.module === "ARIA") {
          var x = C.concept;
          if (x) {
            var E = b.findIndex(function(D) {
              var z = s(D, 1), Z = z[0];
              return Z === h;
            });
            E === -1 && (b.push([h, []]), E = b.length - 1), b[E][1].push(x);
          }
        }
      });
    };
    for (L.s(); !(O = L.n()).done; ) B();
  } catch (R) {
    L.e(R);
  } finally {
    L.f();
  }
  var G = { entries: function() {
    return b;
  }, forEach: function(v) {
    for (var h = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, g = 0, S = b; g < S.length; g++) {
      var C = s(S[g], 2), x = C[0], E = C[1];
      v.call(h, E, x, b);
    }
  }, get: function(v) {
    var h = b.find(function(g) {
      return g[0] === v;
    });
    return h && h[1];
  }, has: function(v) {
    return !!G.get(v);
  }, keys: function() {
    return b.map(function(v) {
      var h = s(v, 1), g = h[0];
      return g;
    });
  }, values: function() {
    return b.map(function(v) {
      var h = s(v, 2), g = h[1];
      return g;
    });
  } }, F = (0, e.default)(G, G.entries());
  return ho.default = F, ho;
}
var mo = {}, dh;
function ox() {
  if (dh) return mo;
  dh = 1, Object.defineProperty(mo, "__esModule", { value: true }), mo.default = void 0;
  var e = i(al()), t = i(rl());
  function i(v) {
    return v && v.__esModule ? v : { default: v };
  }
  function s(v, h) {
    return m(v) || p(v, h) || _(v, h) || l();
  }
  function l() {
    throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
  }
  function p(v, h) {
    var g = v == null ? null : typeof Symbol < "u" && v[Symbol.iterator] || v["@@iterator"];
    if (g != null) {
      var S = [], C = true, x = false, E, D;
      try {
        for (g = g.call(v); !(C = (E = g.next()).done) && (S.push(E.value), !(h && S.length === h)); C = true) ;
      } catch (z) {
        x = true, D = z;
      } finally {
        try {
          !C && g.return != null && g.return();
        } finally {
          if (x) throw D;
        }
      }
      return S;
    }
  }
  function m(v) {
    if (Array.isArray(v)) return v;
  }
  function w(v, h) {
    var g = typeof Symbol < "u" && v[Symbol.iterator] || v["@@iterator"];
    if (!g) {
      if (Array.isArray(v) || (g = _(v)) || h) {
        g && (v = g);
        var S = 0, C = function() {
        };
        return { s: C, n: function() {
          return S >= v.length ? { done: true } : { done: false, value: v[S++] };
        }, e: function(Z) {
          throw Z;
        }, f: C };
      }
      throw new TypeError(`Invalid attempt to iterate non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`);
    }
    var x = true, E = false, D;
    return { s: function() {
      g = g.call(v);
    }, n: function() {
      var Z = g.next();
      return x = Z.done, Z;
    }, e: function(Z) {
      E = true, D = Z;
    }, f: function() {
      try {
        !x && g.return != null && g.return();
      } finally {
        if (E) throw D;
      }
    } };
  }
  function _(v, h) {
    if (v) {
      if (typeof v == "string") return o(v, h);
      var g = Object.prototype.toString.call(v).slice(8, -1);
      if (g === "Object" && v.constructor && (g = v.constructor.name), g === "Map" || g === "Set") return Array.from(v);
      if (g === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(g)) return o(v, h);
    }
  }
  function o(v, h) {
    (h == null || h > v.length) && (h = v.length);
    for (var g = 0, S = new Array(h); g < h; g++) S[g] = v[g];
    return S;
  }
  var b = [], L = w(e.default.entries()), O;
  try {
    var B = function() {
      var h = s(O.value, 2), g = h[0], S = h[1], C = S.relatedConcepts;
      Array.isArray(C) && C.forEach(function(x) {
        if (x.module === "HTML") {
          var E = x.concept;
          if (E != null) {
            for (var D = JSON.stringify(E), z, Z = 0; Z < b.length; Z++) {
              var A = b[Z][0];
              if (JSON.stringify(A) === D) {
                z = b[Z][1];
                break;
              }
            }
            Array.isArray(z) || (z = []);
            var N = z.findIndex(function(I) {
              return I === g;
            });
            N === -1 && z.push(g), Z < b.length ? b.splice(Z, 1, [E, z]) : b.push([E, z]);
          }
        }
      });
    };
    for (L.s(); !(O = L.n()).done; ) B();
  } catch (v) {
    L.e(v);
  } finally {
    L.f();
  }
  function G(v, h) {
    if (v === void 0 && h !== void 0 || v !== void 0 && h === void 0) return false;
    if (v !== void 0 && h !== void 0) {
      if (v.length != h.length) return false;
      for (var g = 0; g < v.length; g++) if (h[g].name !== v[g].name || h[g].value !== v[g].value) return false;
    }
    return true;
  }
  var F = { entries: function() {
    return b;
  }, forEach: function(h) {
    for (var g = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : null, S = 0, C = b; S < C.length; S++) {
      var x = s(C[S], 2), E = x[0], D = x[1];
      h.call(g, D, E, b);
    }
  }, get: function(h) {
    var g = b.find(function(S) {
      return h.name === S[0].name && G(h.attributes, S[0].attributes);
    });
    return g && g[1];
  }, has: function(h) {
    return !!F.get(h);
  }, keys: function() {
    return b.map(function(h) {
      var g = s(h, 1), S = g[0];
      return S;
    });
  }, values: function() {
    return b.map(function(h) {
      var g = s(h, 2), S = g[1];
      return S;
    });
  } }, R = (0, t.default)(F, F.entries());
  return mo.default = R, mo;
}
var ph;
function lx() {
  if (ph) return Rt;
  ph = 1, Object.defineProperty(Rt, "__esModule", { value: true }), Rt.elementAXObjects = Rt.AXObjects = Rt.AXObjectRoles = Rt.AXObjectElements = void 0;
  var e = l(sx()), t = l(nx()), i = l(al()), s = l(ox());
  function l(o) {
    return o && o.__esModule ? o : { default: o };
  }
  var p = e.default;
  Rt.AXObjectElements = p;
  var m = t.default;
  Rt.AXObjectRoles = m;
  var w = i.default;
  Rt.AXObjects = w;
  var _ = s.default;
  return Rt.elementAXObjects = _, Rt;
}
var Qt = lx();
const hm = Ao.roles.keys(), ux = hm.filter((e) => {
  var _a3;
  return (_a3 = Ao.roles.get(e)) == null ? void 0 : _a3.abstract;
}), mm = hm.filter((e) => !ux.includes(e)), vm = mm.filter((e) => {
  const t = Ao.roles.get(e);
  return !["toolbar", "tabpanel", "generic", "cell"].includes(e) && !(t == null ? void 0 : t.superClass.some((i) => i.includes("widget")));
}).concat("progressbar"), cx = mm.filter((e) => !vm.includes(e) && e !== "generic");
Ao.elementRoles.entries().forEach(([e, t]) => {
  [...t].every((i) => i !== "generic" && vm.includes(i));
});
Ao.elementRoles.entries().forEach(([e, t]) => {
  [...t].every((i) => cx.includes(i));
});
const dx = [...Qt.AXObjects.keys()].filter((e) => Qt.AXObjects.get(e).type === "widget"), px = [...Qt.AXObjects.keys()].filter((e) => ["windows", "structure"].includes(Qt.AXObjects.get(e).type));
Qt.elementAXObjects.entries().forEach(([e, t]) => {
  [...t].every((i) => dx.includes(i));
});
Qt.elementAXObjects.entries().forEach(([e, t]) => {
  [...t].every((i) => px.includes(i));
});
var vo = { exports: {} }, fx = vo.exports, fh;
function Ml() {
  return fh || (fh = 1, function(e, t) {
    (function(i, s) {
      s(t);
    })(fx, function(i) {
      const p = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/", m = new Uint8Array(64), w = new Uint8Array(128);
      for (let A = 0; A < p.length; A++) {
        const N = p.charCodeAt(A);
        m[A] = N, w[N] = A;
      }
      function _(A, N) {
        let I = 0, M = 0, V = 0;
        do {
          const r = A.next();
          V = w[r], I |= (V & 31) << M, M += 5;
        } while (V & 32);
        const c = I & 1;
        return I >>>= 1, c && (I = -2147483648 | -I), N + I;
      }
      function o(A, N, I) {
        let M = N - I;
        M = M < 0 ? -M << 1 | 1 : M << 1;
        do {
          let V = M & 31;
          M >>>= 5, M > 0 && (V |= 32), A.write(m[V]);
        } while (M > 0);
        return N;
      }
      function b(A, N) {
        return A.pos >= N ? false : A.peek() !== 44;
      }
      const L = 1024 * 16, O = typeof TextDecoder < "u" ? new TextDecoder() : typeof Buffer < "u" ? { decode(A) {
        return Buffer.from(A.buffer, A.byteOffset, A.byteLength).toString();
      } } : { decode(A) {
        let N = "";
        for (let I = 0; I < A.length; I++) N += String.fromCharCode(A[I]);
        return N;
      } };
      class B {
        constructor() {
          this.pos = 0, this.out = "", this.buffer = new Uint8Array(L);
        }
        write(N) {
          const { buffer: I } = this;
          I[this.pos++] = N, this.pos === L && (this.out += O.decode(I), this.pos = 0);
        }
        flush() {
          const { buffer: N, out: I, pos: M } = this;
          return M > 0 ? I + O.decode(N.subarray(0, M)) : I;
        }
      }
      class G {
        constructor(N) {
          this.pos = 0, this.buffer = N;
        }
        next() {
          return this.buffer.charCodeAt(this.pos++);
        }
        peek() {
          return this.buffer.charCodeAt(this.pos);
        }
        indexOf(N) {
          const { buffer: I, pos: M } = this, V = I.indexOf(N, M);
          return V === -1 ? I.length : V;
        }
      }
      const F = [];
      function R(A) {
        const { length: N } = A, I = new G(A), M = [], V = [];
        let c = 0;
        for (; I.pos < N; I.pos++) {
          c = _(I, c);
          const r = _(I, 0);
          if (!b(I, N)) {
            const P = V.pop();
            P[2] = c, P[3] = r;
            continue;
          }
          const a = _(I, 0), f = _(I, 0) & 1 ? [c, r, 0, 0, a, _(I, 0)] : [c, r, 0, 0, a];
          let y = F;
          if (b(I, N)) {
            y = [];
            do {
              const P = _(I, 0);
              y.push(P);
            } while (b(I, N));
          }
          f.vars = y, M.push(f), V.push(f);
        }
        return M;
      }
      function v(A) {
        const N = new B();
        for (let I = 0; I < A.length; ) I = h(A, I, N, [0]);
        return N.flush();
      }
      function h(A, N, I, M) {
        const V = A[N], { 0: c, 1: r, 2: a, 3: n, 4: u, vars: f } = V;
        N > 0 && I.write(44), M[0] = o(I, c, M[0]), o(I, r, 0), o(I, u, 0);
        const y = V.length === 6 ? 1 : 0;
        o(I, y, 0), V.length === 6 && o(I, V[5], 0);
        for (const P of f) o(I, P, 0);
        for (N++; N < A.length; ) {
          const P = A[N], { 0: T, 1: q } = P;
          if (T > a || T === a && q >= n) break;
          N = h(A, N, I, M);
        }
        return I.write(44), M[0] = o(I, a, M[0]), o(I, n, 0), N;
      }
      function g(A) {
        const { length: N } = A, I = new G(A), M = [], V = [];
        let c = 0, r = 0, a = 0, n = 0, u = 0, f = 0, y = 0, P = 0;
        do {
          const T = I.indexOf(";");
          let q = 0;
          for (; I.pos < T; I.pos++) {
            if (q = _(I, q), !b(I, T)) {
              const he = V.pop();
              he[2] = c, he[3] = q;
              continue;
            }
            const H = _(I, 0), J = H & 1, re = H & 2, Q = H & 4;
            let pe = null, de = F, fe;
            if (J) {
              const he = _(I, r);
              a = _(I, r === he ? a : 0), r = he, fe = [c, q, 0, 0, he, a];
            } else fe = [c, q, 0, 0];
            if (fe.isScope = !!Q, re) {
              const he = n, we = u;
              n = _(I, n);
              const Oe = he === n;
              u = _(I, Oe ? u : 0), f = _(I, Oe && we === u ? f : 0), pe = [n, u, f];
            }
            if (fe.callsite = pe, b(I, T)) {
              de = [];
              do {
                y = c, P = q;
                const he = _(I, 0);
                let we;
                if (he < -1) {
                  we = [[_(I, 0)]];
                  for (let Oe = -1; Oe > he; Oe--) {
                    const Ee = y;
                    y = _(I, y), P = _(I, y === Ee ? P : 0);
                    const Ue = _(I, 0);
                    we.push([Ue, y, P]);
                  }
                } else we = [[he]];
                de.push(we);
              } while (b(I, T));
            }
            fe.bindings = de, M.push(fe), V.push(fe);
          }
          c++, I.pos = T + 1;
        } while (I.pos < N);
        return M;
      }
      function S(A) {
        if (A.length === 0) return "";
        const N = new B();
        for (let I = 0; I < A.length; ) I = C(A, I, N, [0, 0, 0, 0, 0, 0, 0]);
        return N.flush();
      }
      function C(A, N, I, M) {
        const V = A[N], { 0: c, 1: r, 2: a, 3: n, isScope: u, callsite: f, bindings: y } = V;
        M[0] < c ? (x(I, M[0], c), M[0] = c, M[1] = 0) : N > 0 && I.write(44), M[1] = o(I, V[1], M[1]);
        const P = (V.length === 6 ? 1 : 0) | (f ? 2 : 0) | (u ? 4 : 0);
        if (o(I, P, 0), V.length === 6) {
          const { 4: T, 5: q } = V;
          T !== M[2] && (M[3] = 0), M[2] = o(I, T, M[2]), M[3] = o(I, q, M[3]);
        }
        if (f) {
          const { 0: T, 1: q, 2: H } = V.callsite;
          T !== M[4] ? (M[5] = 0, M[6] = 0) : q !== M[5] && (M[6] = 0), M[4] = o(I, T, M[4]), M[5] = o(I, q, M[5]), M[6] = o(I, H, M[6]);
        }
        if (y) for (const T of y) {
          T.length > 1 && o(I, -T.length, 0);
          const q = T[0][0];
          o(I, q, 0);
          let H = c, J = r;
          for (let re = 1; re < T.length; re++) {
            const Q = T[re];
            H = o(I, Q[1], H), J = o(I, Q[2], J), o(I, Q[0], 0);
          }
        }
        for (N++; N < A.length; ) {
          const T = A[N], { 0: q, 1: H } = T;
          if (q > a || q === a && H >= n) break;
          N = C(A, N, I, M);
        }
        return M[0] < a ? (x(I, M[0], a), M[0] = a, M[1] = 0) : I.write(44), M[1] = o(I, n, M[1]), N;
      }
      function x(A, N, I) {
        do
          A.write(59);
        while (++N < I);
      }
      function E(A) {
        const { length: N } = A, I = new G(A), M = [];
        let V = 0, c = 0, r = 0, a = 0, n = 0;
        do {
          const u = I.indexOf(";"), f = [];
          let y = true, P = 0;
          for (V = 0; I.pos < u; ) {
            let T;
            V = _(I, V), V < P && (y = false), P = V, b(I, u) ? (c = _(I, c), r = _(I, r), a = _(I, a), b(I, u) ? (n = _(I, n), T = [V, c, r, a, n]) : T = [V, c, r, a]) : T = [V], f.push(T), I.pos++;
          }
          y || D(f), M.push(f), I.pos = u + 1;
        } while (I.pos <= N);
        return M;
      }
      function D(A) {
        A.sort(z);
      }
      function z(A, N) {
        return A[0] - N[0];
      }
      function Z(A) {
        const N = new B();
        let I = 0, M = 0, V = 0, c = 0;
        for (let r = 0; r < A.length; r++) {
          const a = A[r];
          if (r > 0 && N.write(59), a.length === 0) continue;
          let n = 0;
          for (let u = 0; u < a.length; u++) {
            const f = a[u];
            u > 0 && N.write(44), n = o(N, f[0], n), f.length !== 1 && (I = o(N, f[1], I), M = o(N, f[2], M), V = o(N, f[3], V), f.length !== 4 && (c = o(N, f[4], c)));
          }
        }
        return N.flush();
      }
      i.decode = E, i.decodeGeneratedRanges = g, i.decodeOriginalScopes = R, i.encode = Z, i.encodeGeneratedRanges = S, i.encodeOriginalScopes = v, Object.defineProperty(i, "__esModule", { value: true });
    });
  }(vo, vo.exports)), vo.exports;
}
Ml();
var Fo = { exports: {} }, bo = { exports: {} }, Vo = { exports: {} }, hx = Vo.exports, hh;
function mx() {
  return hh || (hh = 1, function(e, t) {
    (function(i, s) {
      e.exports = s();
    })(hx, function() {
      const i = /^[\w+.-]+:\/\//, s = /^([\w+.-]+:)\/\/([^@/#?]*@)?([^:/#?]*)(:\d+)?(\/[^#?]*)?(\?[^#]*)?(#.*)?/, l = /^file:(?:\/\/((?![a-z]:)[^/#?]*)?)?(\/?[^#?]*)(\?[^#]*)?(#.*)?/i;
      function p(h) {
        return i.test(h);
      }
      function m(h) {
        return h.startsWith("//");
      }
      function w(h) {
        return h.startsWith("/");
      }
      function _(h) {
        return h.startsWith("file:");
      }
      function o(h) {
        return /^[.?#]/.test(h);
      }
      function b(h) {
        const g = s.exec(h);
        return O(g[1], g[2] || "", g[3], g[4] || "", g[5] || "/", g[6] || "", g[7] || "");
      }
      function L(h) {
        const g = l.exec(h), S = g[2];
        return O("file:", "", g[1] || "", "", w(S) ? S : "/" + S, g[3] || "", g[4] || "");
      }
      function O(h, g, S, C, x, E, D) {
        return { scheme: h, user: g, host: S, port: C, path: x, query: E, hash: D, type: 7 };
      }
      function B(h) {
        if (m(h)) {
          const S = b("http:" + h);
          return S.scheme = "", S.type = 6, S;
        }
        if (w(h)) {
          const S = b("http://foo.com" + h);
          return S.scheme = "", S.host = "", S.type = 5, S;
        }
        if (_(h)) return L(h);
        if (p(h)) return b(h);
        const g = b("http://foo.com/" + h);
        return g.scheme = "", g.host = "", g.type = h ? h.startsWith("?") ? 3 : h.startsWith("#") ? 2 : 4 : 1, g;
      }
      function G(h) {
        if (h.endsWith("/..")) return h;
        const g = h.lastIndexOf("/");
        return h.slice(0, g + 1);
      }
      function F(h, g) {
        R(g, g.type), h.path === "/" ? h.path = g.path : h.path = G(g.path) + h.path;
      }
      function R(h, g) {
        const S = g <= 4, C = h.path.split("/");
        let x = 1, E = 0, D = false;
        for (let Z = 1; Z < C.length; Z++) {
          const A = C[Z];
          if (!A) {
            D = true;
            continue;
          }
          if (D = false, A !== ".") {
            if (A === "..") {
              E ? (D = true, E--, x--) : S && (C[x++] = A);
              continue;
            }
            C[x++] = A, E++;
          }
        }
        let z = "";
        for (let Z = 1; Z < x; Z++) z += "/" + C[Z];
        (!z || D && !z.endsWith("/..")) && (z += "/"), h.path = z;
      }
      function v(h, g) {
        if (!h && !g) return "";
        const S = B(h);
        let C = S.type;
        if (g && C !== 7) {
          const E = B(g), D = E.type;
          switch (C) {
            case 1:
              S.hash = E.hash;
            case 2:
              S.query = E.query;
            case 3:
            case 4:
              F(S, E);
            case 5:
              S.user = E.user, S.host = E.host, S.port = E.port;
            case 6:
              S.scheme = E.scheme;
          }
          D > C && (C = D);
        }
        R(S, C);
        const x = S.query + S.hash;
        switch (C) {
          case 2:
          case 3:
            return x;
          case 4: {
            const E = S.path.slice(1);
            return E ? o(g || h) && !o(E) ? "./" + E + x : E + x : x || ".";
          }
          case 5:
            return S.path + x;
          default:
            return S.scheme + "//" + S.user + S.host + S.port + S.path + x;
        }
      }
      return v;
    });
  }(Vo)), Vo.exports;
}
var vx = bo.exports, mh;
function bm() {
  return mh || (mh = 1, function(e, t) {
    (function(i, s) {
      s(t, Ml(), mx());
    })(vx, function(i, s, l) {
      function p(j, X) {
        return X && !X.endsWith("/") && (X += "/"), l(j, X);
      }
      function m(j) {
        if (!j) return "";
        const X = j.lastIndexOf("/");
        return j.slice(0, X + 1);
      }
      const w = 0, _ = 1, o = 2, b = 3, L = 4, O = 1, B = 2;
      function G(j, X) {
        const K = F(j, 0);
        if (K === j.length) return j;
        X || (j = j.slice());
        for (let ee = K; ee < j.length; ee = F(j, ee + 1)) j[ee] = v(j[ee], X);
        return j;
      }
      function F(j, X) {
        for (let K = X; K < j.length; K++) if (!R(j[K])) return K;
        return j.length;
      }
      function R(j) {
        for (let X = 1; X < j.length; X++) if (j[X][w] < j[X - 1][w]) return false;
        return true;
      }
      function v(j, X) {
        return X || (j = j.slice()), j.sort(h);
      }
      function h(j, X) {
        return j[w] - X[w];
      }
      let g = false;
      function S(j, X, K, ee) {
        for (; K <= ee; ) {
          const ce = K + (ee - K >> 1), se = j[ce][w] - X;
          if (se === 0) return g = true, ce;
          se < 0 ? K = ce + 1 : ee = ce - 1;
        }
        return g = false, K - 1;
      }
      function C(j, X, K) {
        for (let ee = K + 1; ee < j.length && j[ee][w] === X; K = ee++) ;
        return K;
      }
      function x(j, X, K) {
        for (let ee = K - 1; ee >= 0 && j[ee][w] === X; K = ee--) ;
        return K;
      }
      function E() {
        return { lastKey: -1, lastNeedle: -1, lastIndex: -1 };
      }
      function D(j, X, K, ee) {
        const { lastKey: ce, lastNeedle: se, lastIndex: Re } = K;
        let ke = 0, ge = j.length - 1;
        if (ee === ce) {
          if (X === se) return g = Re !== -1 && j[Re][w] === X, Re;
          X >= se ? ke = Re === -1 ? 0 : Re : ge = Re;
        }
        return K.lastKey = ee, K.lastNeedle = X, K.lastIndex = S(j, X, ke, ge);
      }
      function z(j, X) {
        const K = X.map(A);
        for (let ee = 0; ee < j.length; ee++) {
          const ce = j[ee];
          for (let se = 0; se < ce.length; se++) {
            const Re = ce[se];
            if (Re.length === 1) continue;
            const ke = Re[_], ge = Re[o], Ae = Re[b], ze = K[ke], je = ze[ge] || (ze[ge] = []), De = X[ke];
            let Ge = C(je, Ae, D(je, Ae, De, ge));
            De.lastIndex = ++Ge, Z(je, Ge, [Ae, ee, Re[w]]);
          }
        }
        return K;
      }
      function Z(j, X, K) {
        for (let ee = j.length; ee > X; ee--) j[ee] = j[ee - 1];
        j[X] = K;
      }
      function A() {
        return { __proto__: null };
      }
      const N = function(j, X) {
        const K = I(j);
        if (!("sections" in K)) return new y(K, X);
        const ee = [], ce = [], se = [], Re = [], ke = [];
        M(K, X, ee, ce, se, Re, ke, 0, 0, 1 / 0, 1 / 0);
        const ge = { version: 3, file: K.file, names: Re, sources: ce, sourcesContent: se, mappings: ee, ignoreList: ke };
        return we(ge);
      };
      function I(j) {
        return typeof j == "string" ? JSON.parse(j) : j;
      }
      function M(j, X, K, ee, ce, se, Re, ke, ge, Ae, ze) {
        const { sections: je } = j;
        for (let De = 0; De < je.length; De++) {
          const { map: Ge, offset: te } = je[De];
          let Ie = Ae, be = ze;
          if (De + 1 < je.length) {
            const Qe = je[De + 1].offset;
            Ie = Math.min(Ae, ke + Qe.line), Ie === Ae ? be = Math.min(ze, ge + Qe.column) : Ie < Ae && (be = ge + Qe.column);
          }
          V(Ge, X, K, ee, ce, se, Re, ke + te.line, ge + te.column, Ie, be);
        }
      }
      function V(j, X, K, ee, ce, se, Re, ke, ge, Ae, ze) {
        const je = I(j);
        if ("sections" in je) return M(...arguments);
        const De = new y(je, X), Ge = ee.length, te = se.length, Ie = q(De), { resolvedSources: be, sourcesContent: Qe, ignoreList: tt } = De;
        if (c(ee, be), c(se, De.names), Qe) c(ce, Qe);
        else for (let Xe = 0; Xe < be.length; Xe++) ce.push(null);
        if (tt) for (let Xe = 0; Xe < tt.length; Xe++) Re.push(tt[Xe] + Ge);
        for (let Xe = 0; Xe < Ie.length; Xe++) {
          const We = ke + Xe;
          if (We > Ae) return;
          const Ye = r(K, We), St = Xe === 0 ? ge : 0, Ht = Ie[Xe];
          for (let Y = 0; Y < Ht.length; Y++) {
            const yt = Ht[Y], gt = St + yt[w];
            if (We === Ae && gt >= ze) return;
            if (yt.length === 1) {
              Ye.push([gt]);
              continue;
            }
            const Eo = Ge + yt[_], ko = yt[o], Io = yt[b];
            Ye.push(yt.length === 4 ? [gt, Eo, ko, Io] : [gt, Eo, ko, Io, te + yt[L]]);
          }
        }
      }
      function c(j, X) {
        for (let K = 0; K < X.length; K++) j.push(X[K]);
      }
      function r(j, X) {
        for (let K = j.length; K <= X; K++) j[K] = [];
        return j[X];
      }
      const a = "`line` must be greater than 0 (lines start at line 1)", n = "`column` must be greater than or equal to 0 (columns start at column 0)", u = -1, f = 1;
      class y {
        constructor(X, K) {
          const ee = typeof X == "string";
          if (!ee && X._decodedMemo) return X;
          const ce = ee ? JSON.parse(X) : X, { version: se, file: Re, names: ke, sourceRoot: ge, sources: Ae, sourcesContent: ze } = ce;
          this.version = se, this.file = Re, this.names = ke || [], this.sourceRoot = ge, this.sources = Ae, this.sourcesContent = ze, this.ignoreList = ce.ignoreList || ce.x_google_ignoreList || void 0;
          const je = p(ge || "", m(K));
          this.resolvedSources = Ae.map((Ge) => p(Ge || "", je));
          const { mappings: De } = ce;
          typeof De == "string" ? (this._encoded = De, this._decoded = void 0) : (this._encoded = void 0, this._decoded = G(De, ee)), this._decodedMemo = E(), this._bySources = void 0, this._bySourceMemos = void 0;
        }
      }
      function P(j) {
        return j;
      }
      function T(j) {
        var X, K;
        return (X = (K = j)._encoded) !== null && X !== void 0 ? X : K._encoded = s.encode(j._decoded);
      }
      function q(j) {
        var X;
        return (X = j)._decoded || (X._decoded = s.decode(j._encoded));
      }
      function H(j, X, K) {
        const ee = q(j);
        if (X >= ee.length) return null;
        const ce = ee[X], se = Je(ce, j._decodedMemo, X, K, f);
        return se === -1 ? null : ce[se];
      }
      function J(j, X) {
        let { line: K, column: ee, bias: ce } = X;
        if (K--, K < 0) throw new Error(a);
        if (ee < 0) throw new Error(n);
        const se = q(j);
        if (K >= se.length) return Ne(null, null, null, null);
        const Re = se[K], ke = Je(Re, j._decodedMemo, K, ee, ce || f);
        if (ke === -1) return Ne(null, null, null, null);
        const ge = Re[ke];
        if (ge.length === 1) return Ne(null, null, null, null);
        const { names: Ae, resolvedSources: ze } = j;
        return Ne(ze[ge[_]], ge[o] + 1, ge[b], ge.length === 5 ? Ae[ge[L]] : null);
      }
      function re(j, X) {
        const { source: K, line: ee, column: ce, bias: se } = X;
        return bt(j, K, ee, ce, se || f, false);
      }
      function Q(j, X) {
        const { source: K, line: ee, column: ce, bias: se } = X;
        return bt(j, K, ee, ce, se || u, true);
      }
      function pe(j, X) {
        const K = q(j), { names: ee, resolvedSources: ce } = j;
        for (let se = 0; se < K.length; se++) {
          const Re = K[se];
          for (let ke = 0; ke < Re.length; ke++) {
            const ge = Re[ke], Ae = se + 1, ze = ge[0];
            let je = null, De = null, Ge = null, te = null;
            ge.length !== 1 && (je = ce[ge[1]], De = ge[2] + 1, Ge = ge[3]), ge.length === 5 && (te = ee[ge[4]]), X({ generatedLine: Ae, generatedColumn: ze, source: je, originalLine: De, originalColumn: Ge, name: te });
          }
        }
      }
      function de(j, X) {
        const { sources: K, resolvedSources: ee } = j;
        let ce = K.indexOf(X);
        return ce === -1 && (ce = ee.indexOf(X)), ce;
      }
      function fe(j, X) {
        const { sourcesContent: K } = j;
        if (K == null) return null;
        const ee = de(j, X);
        return ee === -1 ? null : K[ee];
      }
      function he(j, X) {
        const { ignoreList: K } = j;
        if (K == null) return false;
        const ee = de(j, X);
        return ee === -1 ? false : K.includes(ee);
      }
      function we(j, X) {
        const K = new y(Ue(j, []), X);
        return K._decoded = j.mappings, K;
      }
      function Oe(j) {
        return Ue(j, q(j));
      }
      function Ee(j) {
        return Ue(j, T(j));
      }
      function Ue(j, X) {
        return { version: j.version, file: j.file, names: j.names, sourceRoot: j.sourceRoot, sources: j.sources, sourcesContent: j.sourcesContent, mappings: X, ignoreList: j.ignoreList || j.x_google_ignoreList };
      }
      function Ne(j, X, K, ee) {
        return { source: j, line: X, column: K, name: ee };
      }
      function Ze(j, X) {
        return { line: j, column: X };
      }
      function Je(j, X, K, ee, ce) {
        let se = D(j, ee, X, K);
        return g ? se = (ce === u ? C : x)(j, ee, se) : ce === u && se++, se === -1 || se === j.length ? -1 : se;
      }
      function vt(j, X, K, ee, ce) {
        let se = Je(j, X, K, ee, f);
        if (!g && ce === u && se++, se === -1 || se === j.length) return [];
        const Re = g ? ee : j[se][w];
        g || (se = x(j, Re, se));
        const ke = C(j, Re, se), ge = [];
        for (; se <= ke; se++) {
          const Ae = j[se];
          ge.push(Ze(Ae[O] + 1, Ae[B]));
        }
        return ge;
      }
      function bt(j, X, K, ee, ce, se) {
        var Re;
        if (K--, K < 0) throw new Error(a);
        if (ee < 0) throw new Error(n);
        const { sources: ke, resolvedSources: ge } = j;
        let Ae = ke.indexOf(X);
        if (Ae === -1 && (Ae = ge.indexOf(X)), Ae === -1) return se ? [] : Ze(null, null);
        const je = ((Re = j)._bySources || (Re._bySources = z(q(j), j._bySourceMemos = ke.map(E))))[Ae][K];
        if (je == null) return se ? [] : Ze(null, null);
        const De = j._bySourceMemos[Ae];
        if (se) return vt(je, De, K, ee, ce);
        const Ge = Je(je, De, K, ee, ce);
        if (Ge === -1) return Ze(null, null);
        const te = je[Ge];
        return Ze(te[O] + 1, te[B]);
      }
      i.AnyMap = N, i.GREATEST_LOWER_BOUND = f, i.LEAST_UPPER_BOUND = u, i.TraceMap = y, i.allGeneratedPositionsFor = Q, i.decodedMap = Oe, i.decodedMappings = q, i.eachMapping = pe, i.encodedMap = Ee, i.encodedMappings = T, i.generatedPositionFor = re, i.isIgnored = he, i.originalPositionFor = J, i.presortedDecodedMap = we, i.sourceContentFor = fe, i.traceSegment = H;
    });
  }(bo, bo.exports)), bo.exports;
}
var yo = { exports: {} }, go = { exports: {} }, bx = go.exports, vh;
function yx() {
  return vh || (vh = 1, function(e, t) {
    (function(i, s) {
      s(t);
    })(bx, function(i) {
      class s {
        constructor() {
          this._indexes = { __proto__: null }, this.array = [];
        }
      }
      function l(o) {
        return o;
      }
      function p(o, b) {
        return o._indexes[b];
      }
      function m(o, b) {
        const L = p(o, b);
        if (L !== void 0) return L;
        const { array: O, _indexes: B } = o, G = O.push(b);
        return B[b] = G - 1;
      }
      function w(o) {
        const { array: b, _indexes: L } = o;
        if (b.length === 0) return;
        const O = b.pop();
        L[O] = void 0;
      }
      function _(o, b) {
        const L = p(o, b);
        if (L === void 0) return;
        const { array: O, _indexes: B } = o;
        for (let G = L + 1; G < O.length; G++) {
          const F = O[G];
          O[G - 1] = F, B[F]--;
        }
        B[b] = void 0, O.pop();
      }
      i.SetArray = s, i.get = p, i.pop = w, i.put = m, i.remove = _, Object.defineProperty(i, "__esModule", { value: true });
    });
  }(go, go.exports)), go.exports;
}
var gx = yo.exports, bh;
function Rx() {
  return bh || (bh = 1, function(e, t) {
    (function(i, s) {
      s(t, yx(), Ml(), bm());
    })(gx, function(i, s, l, p) {
      class O {
        constructor({ file: a, sourceRoot: n } = {}) {
          this._names = new s.SetArray(), this._sources = new s.SetArray(), this._sourcesContent = [], this._mappings = [], this.file = a, this.sourceRoot = n, this._ignoreList = new s.SetArray();
        }
      }
      function B(r) {
        return r;
      }
      function G(r, a, n, u, f, y, P, T) {
        return D(false, r, a, n, u, f, y, P, T);
      }
      function F(r, a) {
        return c(false, r, a);
      }
      const R = (r, a, n, u, f, y, P, T) => D(true, r, a, n, u, f, y, P, T), v = (r, a) => c(true, r, a);
      function h(r, a, n) {
        const { _sources: u, _sourcesContent: f } = r, y = s.put(u, a);
        f[y] = n;
      }
      function g(r, a, n = true) {
        const { _sources: u, _sourcesContent: f, _ignoreList: y } = r, P = s.put(u, a);
        P === f.length && (f[P] = null), n ? s.put(y, P) : s.remove(y, P);
      }
      function S(r) {
        const { _mappings: a, _sources: n, _sourcesContent: u, _names: f, _ignoreList: y } = r;
        return N(a), { version: 3, file: r.file || void 0, names: f.array, sourceRoot: r.sourceRoot || void 0, sources: n.array, sourcesContent: u, mappings: a, ignoreList: y.array };
      }
      function C(r) {
        const a = S(r);
        return Object.assign(Object.assign({}, a), { mappings: l.encode(a.mappings) });
      }
      function x(r) {
        const a = new p.TraceMap(r), n = new O({ file: a.file, sourceRoot: a.sourceRoot });
        return I(n._names, a.names), I(n._sources, a.sources), n._sourcesContent = a.sourcesContent || a.sources.map(() => null), n._mappings = p.decodedMappings(a), a.ignoreList && I(n._ignoreList, a.ignoreList), n;
      }
      function E(r) {
        const a = [], { _mappings: n, _sources: u, _names: f } = r;
        for (let y = 0; y < n.length; y++) {
          const P = n[y];
          for (let T = 0; T < P.length; T++) {
            const q = P[T], H = { line: y + 1, column: q[0] };
            let J, re, Q;
            q.length !== 1 && (J = u.array[q[1]], re = { line: q[2] + 1, column: q[3] }, q.length === 5 && (Q = f.array[q[4]])), a.push({ generated: H, source: J, original: re, name: Q });
          }
        }
        return a;
      }
      function D(r, a, n, u, f, y, P, T, q) {
        const { _mappings: H, _sources: J, _sourcesContent: re, _names: Q } = a, pe = z(H, n), de = Z(pe, u);
        if (!f) return r && M(pe, de) ? void 0 : A(pe, de, [u]);
        const fe = s.put(J, f), he = T ? s.put(Q, T) : -1;
        if (fe === re.length && (re[fe] = q ?? null), !(r && V(pe, de, fe, y, P, he))) return A(pe, de, T ? [u, fe, y, P, he] : [u, fe, y, P]);
      }
      function z(r, a) {
        for (let n = r.length; n <= a; n++) r[n] = [];
        return r[a];
      }
      function Z(r, a) {
        let n = r.length;
        for (let u = n - 1; u >= 0; n = u--) {
          const f = r[u];
          if (a >= f[0]) break;
        }
        return n;
      }
      function A(r, a, n) {
        for (let u = r.length; u > a; u--) r[u] = r[u - 1];
        r[a] = n;
      }
      function N(r) {
        const { length: a } = r;
        let n = a;
        for (let u = n - 1; u >= 0 && !(r[u].length > 0); n = u, u--) ;
        n < a && (r.length = n);
      }
      function I(r, a) {
        for (let n = 0; n < a.length; n++) s.put(r, a[n]);
      }
      function M(r, a) {
        return a === 0 ? true : r[a - 1].length === 1;
      }
      function V(r, a, n, u, f, y) {
        if (a === 0) return false;
        const P = r[a - 1];
        return P.length === 1 ? false : n === P[1] && u === P[2] && f === P[3] && y === (P.length === 5 ? P[4] : -1);
      }
      function c(r, a, n) {
        const { generated: u, source: f, original: y, name: P, content: T } = n;
        return f ? D(r, a, u.line - 1, u.column, f, y.line - 1, y.column, P, T) : D(r, a, u.line - 1, u.column, null, null, null, null, null);
      }
      i.GenMapping = O, i.addMapping = F, i.addSegment = G, i.allMappings = E, i.fromMap = x, i.maybeAddMapping = v, i.maybeAddSegment = R, i.setIgnore = g, i.setSourceContent = h, i.toDecodedMap = S, i.toEncodedMap = C, Object.defineProperty(i, "__esModule", { value: true });
    });
  }(yo, yo.exports)), yo.exports;
}
var xx = Fo.exports, yh;
function _x() {
  return yh || (yh = 1, function(e, t) {
    (function(i, s) {
      e.exports = s(bm(), Rx());
    })(xx, function(i, s) {
      const l = m("", -1, -1, "", null, false), p = [];
      function m(v, h, g, S, C, x) {
        return { source: v, line: h, column: g, name: S, content: C, ignore: x };
      }
      function w(v, h, g, S, C) {
        return { map: v, sources: h, source: g, content: S, ignore: C };
      }
      function _(v, h) {
        return w(v, h, "", null, false);
      }
      function o(v, h, g) {
        return w(null, p, v, h, g);
      }
      function b(v) {
        const h = new s.GenMapping({ file: v.map.file }), { sources: g, map: S } = v, C = S.names, x = i.decodedMappings(S);
        for (let E = 0; E < x.length; E++) {
          const D = x[E];
          for (let z = 0; z < D.length; z++) {
            const Z = D[z], A = Z[0];
            let N = l;
            if (Z.length !== 1) {
              const n = g[Z[1]];
              if (N = L(n, Z[2], Z[3], Z.length === 5 ? C[Z[4]] : ""), N == null) continue;
            }
            const { column: I, line: M, name: V, content: c, source: r, ignore: a } = N;
            s.maybeAddSegment(h, E, A, r, M, I, V), r && c != null && s.setSourceContent(h, r, c), a && s.setIgnore(h, r, true);
          }
        }
        return h;
      }
      function L(v, h, g, S) {
        if (!v.map) return m(v.source, h, g, S, v.content, v.ignore);
        const C = i.traceSegment(v.map, h, g);
        return C == null ? null : C.length === 1 ? l : L(v.sources[C[1]], C[2], C[3], C.length === 5 ? v.map.names[C[4]] : S);
      }
      function O(v) {
        return Array.isArray(v) ? v : [v];
      }
      function B(v, h) {
        const g = O(v).map((x) => new i.TraceMap(x, "")), S = g.pop();
        for (let x = 0; x < g.length; x++) if (g[x].sources.length > 1) throw new Error(`Transformation map ${x} must have exactly one source file.
Did you specify these with the most recent transformation maps first?`);
        let C = G(S, h, "", 0);
        for (let x = g.length - 1; x >= 0; x--) C = _(g[x], [C]);
        return C;
      }
      function G(v, h, g, S) {
        const { resolvedSources: C, sourcesContent: x, ignoreList: E } = v, D = S + 1, z = C.map((Z, A) => {
          const N = { importer: g, depth: D, source: Z || "", content: void 0, ignore: void 0 }, I = h(N.source, N), { source: M, content: V, ignore: c } = N;
          if (I) return G(new i.TraceMap(I, M), h, M, D);
          const r = V !== void 0 ? V : x ? x[A] : null, a = c !== void 0 ? c : E ? E.includes(A) : false;
          return o(M, r, a);
        });
        return _(v, z);
      }
      class F {
        constructor(h, g) {
          const S = g.decodedMappings ? s.toDecodedMap(h) : s.toEncodedMap(h);
          this.version = S.version, this.file = S.file, this.mappings = S.mappings, this.names = S.names, this.ignoreList = S.ignoreList, this.sourceRoot = S.sourceRoot, this.sources = S.sources, g.excludeContent || (this.sourcesContent = S.sourcesContent);
        }
        toString() {
          return JSON.stringify(this);
        }
      }
      function R(v, h, g) {
        const S = typeof g == "object" ? g : { excludeContent: !!g, decodedMappings: false }, C = B(v, h);
        return new F(b(C), S);
      }
      return R;
    });
  }(Fo)), Fo.exports;
}
_x();
const Cx = { filename: _o("(unknown)"), rootDir: _o(typeof process < "u" ? (_a2 = process.cwd) == null ? void 0 : _a2.call(process) : typeof Deno < "u" ? Deno.cwd() : void 0), dev: xt(false), generate: Vt("client", (e, t) => e === "dom" || e === "ssr" ? (Ll(Kv), e === "dom" ? "client" : "server") : (e !== "client" && e !== "server" && e !== false && Ct(`${t} must be "client", "server" or false`), e)), warningFilter: ym(() => true) };
xh({ ...Cx, accessors: Rh(Uv, xt(false)), css: Vt("external", (e) => ((e === true || e === false) && Ct('The boolean options have been removed from the css option. Use "external" instead of false and "injected" instead of true'), e === "none" && Ct('css: "none" is no longer a valid option. If this was crucial for you, please open an issue on GitHub with your use case.'), e !== "external" && e !== "injected" && Ct('css should be either "external" (default, recommended) or "injected"'), e)), cssHash: ym(({ css: e, hash: t }) => `svelte-${t(e)}`), cssOutputFilename: _o(void 0), customElement: xt(false), discloseVersion: xt(true), immutable: Rh(Gv, xt(false)), legacy: Wt("The legacy option has been removed. If you are using this because of legacy.componentApi, use compatibility.componentApi instead"), compatibility: xh({ componentApi: _h([4, 5], 5) }), loopGuardTimeout: fl(Xv), name: _o(void 0), namespace: _h(["html", "mathml", "svg"]), modernAst: xt(false), outputFilename: _o(void 0), preserveComments: xt(false), preserveWhitespace: xt(false), runes: xt(void 0), hmr: xt(false), sourcemap: Vt(void 0, (e) => e), enableSourcemap: fl(Wv), hydratable: fl(zv), format: Wt('The format option has been removed in Svelte 4, the compiler only outputs ESM now. Remove "format" from your compiler options. If you did not set this yourself, bump the version of your bundler plugin (vite-plugin-svelte/rollup-plugin-svelte/svelte-loader)'), tag: Wt('The tag option has been removed in Svelte 5. Use `<svelte:options customElement="tag-name" />` inside the component instead. If that does not solve your use case, please open an issue on GitHub with details.'), sveltePath: Wt("The sveltePath option has been removed in Svelte 5. If this option was crucial for you, please open an issue on GitHub with your use case."), errorMode: Wt("The errorMode option has been removed. If you are using this through svelte-preprocess with TypeScript, use the https://www.typescriptlang.org/tsconfig#verbatimModuleSyntax setting instead"), varsReport: Wt("The vars option has been removed. If you are using this through svelte-preprocess with TypeScript, use the https://www.typescriptlang.org/tsconfig#verbatimModuleSyntax setting instead") });
function Wt(e) {
  return (t) => {
    t !== void 0 && J1(null, e);
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
  return (i, s) => (i !== void 0 && Ll(e), t(i, s));
}
function xh(e, t = false) {
  return (i, s) => {
    const l = {};
    (i && typeof i != "object" || Array.isArray(i)) && Ct(`${s} should be an object`);
    for (const p in i) p in e || (t ? l[p] = i[p] : Q1(null, `${s ? `${s}.${p}` : p}`));
    for (const p in e) {
      const m = e[p];
      l[p] = m(i && i[p], s ? `${s}.${p}` : p);
    }
    return l;
  };
}
function Vt(e, t) {
  return (i, s) => i === void 0 ? e : t(i, s);
}
function _o(e, t = true) {
  return Vt(e, (i, s) => (typeof i != "string" && Ct(`${s} should be a string, if specified`), !t && i === "" && Ct(`${s} cannot be empty`), i));
}
function xt(e) {
  return Vt(e, (t, i) => (typeof t != "boolean" && Ct(`${i} should be true or false, if specified`), t));
}
function _h(e, t = e[0]) {
  return Vt(t, (i, s) => {
    if (!e.includes(i)) {
      const l = e.length > 2 ? `${s} should be one of ${e.slice(0, -1).map((p) => `"${p}"`).join(", ")} or "${e[e.length - 1]}"` : `${s} should be either "${e[0]}" or "${e[1]}"`;
      Ct(l);
    }
    return i;
  });
}
function ym(e) {
  return Vt(e, (t, i) => (typeof t != "function" && Ct(`${i} should be a function, if specified`), t));
}
function Ct(e) {
  K1(null, e);
}
var Px = Me("<p> </p>"), qx = Me('<p> </p> <!> <div class="flex gap-05"><!> <!></div>', 1), Tx = Me('<div class="err"> </div>'), Sx = Me('<div class="container svelte-b4h84n"><!></div> <!>', 1);
function wx(e, t) {
  It(t, true);
  let i = Yt(), s = Dt(), l = ie(""), p = ie(false), m = ie(W([])), w = ie(W([])), _ = ie(W([]));
  gl(() => {
    o();
  }), At(() => {
    b();
  }), At(() => {
    $(_, W(k(m).map((g) => {
      var _a3;
      return { name: g.name, desc: g.desc, value: ((_a3 = k(w).find((C) => C.key === g.name)) == null ? void 0 : _a3.value) || "" };
    })));
  });
  async function o() {
    var _a3;
    let g = await Nt("/auth/v1/users/attr");
    g.body ? $(m, W(g.body.values.toSorted((S, C) => S.name.localeCompare(C.name)))) : $(l, W(((_a3 = g.error) == null ? void 0 : _a3.message) || "Error fetching attrs"));
  }
  async function b() {
    var _a3;
    let g = await Nt(`/auth/v1/users/${t.user.id}/attr`);
    g.body ? $(w, W(g.body.values)) : $(l, W(((_a3 = g.error) == null ? void 0 : _a3.message) || "Error fetching users attrs"));
  }
  async function L() {
    $(l, "");
    let g = k(w).map((x) => x.key), S = { values: k(_).filter((x) => x.value.trim() || g.includes(x.name)).map((x) => ({ key: x.name, value: x.value.trim() })) }, C = await Rl(`/auth/v1/users/${t.user.id}/attr`, S);
    C.error ? $(l, W(C.error.message)) : ($(p, true), setTimeout(() => {
      $(p, false);
    }, 3e3)), t.onSave();
  }
  var O = Sx(), B = Le(O), G = ve(B);
  {
    var F = (g) => {
      var S = Px(), C = ve(S, true);
      me(S), _e(() => xe(C, s.common.noEntries)), ae(g, S);
    }, R = (g) => {
      var S = qx(), C = Le(S), x = ve(C, true);
      me(C);
      var E = ne(C, 2);
      Ho(E, 17, () => k(_), ov, (N, I, M) => {
        const V = _t(() => k(I).desc || "JSON Value");
        pt(N, { autocomplete: "off", get label() {
          return k(I).name;
        }, get placeholder() {
          return k(V);
        }, get value() {
          return k(I).value;
        }, set value(c) {
          k(I).value = c;
        } });
      });
      var D = ne(E, 2), z = ve(D);
      wt(z, { onclick: L, children: (N, I) => {
        Ke();
        var M = $e();
        _e(() => xe(M, i.common.save)), ae(N, M);
      }, $$slots: { default: true } });
      var Z = ne(z, 2);
      {
        var A = (N) => {
          Yo(N, {});
        };
        Se(Z, (N) => {
          k(p) && N(A);
        });
      }
      me(D), _e(() => xe(x, s.users.descAttr)), ae(g, S);
    };
    Se(G, (g) => {
      k(_).length === 0 ? g(F) : g(R, false);
    });
  }
  me(B);
  var v = ne(B, 2);
  {
    var h = (g) => {
      var S = Tx(), C = ve(S, true);
      me(S), _e(() => xe(C, k(l))), ae(g, S);
    };
    Se(v, (g) => {
      k(l) && g(h);
    });
  }
  ae(e, O), Mt();
}
var Ax = Me("<p><b> </b></p> <p> </p> <!>", 1), Ex = Me('<div class="desc"><p><b> </b></p> <p> </p> <p> </p></div>'), kx = Me('<!> <!> <!> <div></div> <p> </p> <div class="flex gap-05"><!> <!> <!></div>', 1), Ix = Me("<div></div> <!> <!>", 1), Mx = Me('<div class="err"> </div>'), Lx = Me("<!> <!>", 1);
function Ox(e, t) {
  It(t, true);
  const i = "min(20rem, calc(100dvw - .5rem))";
  let s = Yt(), l = Dt(), p = ie(false), m = ie(""), w = ie(false), _ = ie(void 0), o = ie(false), b = ie(""), L = ie(""), O = _t(() => {
    var _a3;
    return k(b).length > (((_a3 = k(_)) == null ? void 0 : _a3.length_min) || 8) && k(b) === k(L);
  }), B = ie(void 0), G = ie(void 0);
  gl(async () => {
    var _a3;
    let D = await Nt("/auth/v1/password_policy");
    D.body ? $(_, W(D.body)) : $(m, W(((_a3 = D.error) == null ? void 0 : _a3.message) || "Error"));
  }), At(() => {
    t.user.id && ($(b, ""), $(L, ""));
  });
  async function F() {
    $(m, ""), $(p, true);
    let D = { email: t.user.email }, z = await qh("/auth/v1/users/request_reset", D);
    z.error ? $(m, W(z.error.message)) : ($(w, true), setTimeout(() => {
      $(w, false);
    }, 3e3)), $(p, false);
  }
  async function R(D, z) {
    if (!k(o)) {
      $(m, W(s.account.passwordPolicyFollow));
      return;
    }
    if (k(b) !== k(L)) {
      $(m, W(s.account.passwordNoMatch));
      return;
    }
    let Z = { email: t.user.email, given_name: t.user.given_name, family_name: t.user.family_name, language: t.user.language, password: k(b), roles: t.user.roles, groups: t.user.groups, enabled: t.user.enabled, email_verified: t.user.email_verified, user_expires: t.user.user_expires };
    $(m, ""), $(p, true);
    let A = await Rl(D.action, Z);
    A.error ? $(m, W(A.error.message)) : ($(w, true), setTimeout(() => {
      $(w, false), t.onSave();
    }, 3e3)), $(p, false);
  }
  function v() {
    if ($(m, ""), !k(_)) return;
    let D = vv(k(_));
    $(b, W(D)), $(L, W(D)), requestAnimationFrame(() => {
      var _a3, _b2;
      (_a3 = k(B)) == null ? void 0 : _a3(), (_b2 = k(G)) == null ? void 0 : _b2();
    });
  }
  var h = Lx(), g = Le(h);
  {
    var S = (D) => {
      var z = Ax(), Z = Le(z), A = ve(Z), N = ve(A, true);
      me(A), me(Z);
      var I = ne(Z, 2), M = ve(I, true);
      me(I);
      var V = ne(I, 2);
      wt(V, { onclick: F, get isLoading() {
        return k(p);
      }, children: (c, r) => {
        Ke();
        var a = $e();
        _e(() => xe(a, l.users.pwdSendEmailBtn)), ae(c, a);
      }, $$slots: { default: true } }), _e(() => {
        xe(N, l.users.pwdNoInit), xe(M, l.users.pwdSendEmailDesc);
      }), ae(D, z);
    }, C = (D) => {
      var z = st(), Z = Le(z);
      {
        var A = (I) => {
          var M = Ex(), V = ve(M), c = ve(V), r = ve(c, true);
          me(c), me(V);
          var a = ne(V, 2), n = ve(a, true);
          me(a);
          var u = ne(a, 2), f = ve(u, true);
          me(u), me(M), _e(() => {
            xe(r, l.users.pkOnly1), xe(n, l.users.pkOnly2), xe(f, l.users.pkOnly3);
          }), ae(I, M);
        }, N = (I) => {
          var M = Ix(), V = Le(M);
          Xt(V, "margin-top", ".5rem");
          var c = ne(V, 2);
          {
            var r = (u) => {
              Rv(u, { get password() {
                return k(b);
              }, get policy() {
                return k(_);
              }, get accepted() {
                return k(o);
              }, set accepted(f) {
                $(o, W(f));
              } });
            };
            Se(c, (u) => {
              k(_) && u(r);
            });
          }
          var a = ne(c, 2);
          const n = _t(() => `/auth/v1/users/${t.user.id}`);
          _l(a, { get action() {
            return k(n);
          }, onSubmit: R, children: (u, f) => {
            var y = kx(), P = Le(y);
            const T = _t(() => {
              var _a3;
              return ((_a3 = k(_)) == null ? void 0 : _a3.length_max) || 256;
            });
            Bl(P, { autocomplete: "off", get label() {
              return s.account.passwordNew;
            }, get placeholder() {
              return s.account.passwordNew;
            }, get showCopy() {
              return k(O);
            }, required: true, get maxLength() {
              return k(T);
            }, width: i, get value() {
              return k(b);
            }, set value(Ee) {
              $(b, W(Ee));
            }, get reportValidity() {
              return k(B);
            }, set reportValidity(Ee) {
              $(B, W(Ee));
            } });
            var q = ne(P, 2);
            const H = _t(() => {
              var _a3;
              return ((_a3 = k(_)) == null ? void 0 : _a3.length_max) || 256;
            });
            Bl(q, { autocomplete: "off", get label() {
              return s.account.passwordConfirm;
            }, get placeholder() {
              return s.account.passwordConfirm;
            }, required: true, get maxLength() {
              return k(H);
            }, width: i, get value() {
              return k(L);
            }, set value(Ee) {
              $(L, W(Ee));
            }, get reportValidity() {
              return k(G);
            }, set reportValidity(Ee) {
              $(G, W(Ee));
            } });
            var J = ne(q, 2);
            wt(J, { level: 2, onclick: v, children: (Ee, Ue) => {
              Ke();
              var Ne = $e();
              _e(() => xe(Ne, s.account.generateRandom)), ae(Ee, Ne);
            }, $$slots: { default: true } });
            var re = ne(J, 2);
            Xt(re, "margin-top", "1rem");
            var Q = ne(re, 2), pe = ve(Q, true);
            me(Q);
            var de = ne(Q, 2), fe = ve(de);
            wt(fe, { onclick: F, get isLoading() {
              return k(p);
            }, children: (Ee, Ue) => {
              Ke();
              var Ne = $e();
              _e(() => xe(Ne, l.users.sendResetEmail)), ae(Ee, Ne);
            }, $$slots: { default: true } });
            var he = ne(fe, 2);
            wt(he, { type: "submit", level: 2, get isLoading() {
              return k(p);
            }, children: (Ee, Ue) => {
              Ke();
              var Ne = $e();
              _e(() => xe(Ne, l.users.savePassword)), ae(Ee, Ne);
            }, $$slots: { default: true } });
            var we = ne(he, 2);
            {
              var Oe = (Ee) => {
                Yo(Ee, {});
              };
              Se(we, (Ee) => {
                k(w) && Ee(Oe);
              });
            }
            me(de), _e(() => xe(pe, l.users.selfServiceDesc)), ae(u, y);
          }, $$slots: { default: true } }), ae(I, M);
        };
        Se(Z, (I) => {
          t.user.account_type === "passkey" || t.user.account_type === "federated_passkey" ? I(A) : I(N, false);
        }, true);
      }
      ae(D, z);
    };
    Se(g, (D) => {
      t.user.account_type === "new" ? D(S) : D(C, false);
    });
  }
  var x = ne(g, 2);
  {
    var E = (D) => {
      var z = Mx(), Z = ve(z, true);
      me(z), _e(() => xe(Z, k(m))), ae(D, z);
    };
    Se(x, (D) => {
      k(m) && D(E);
    });
  }
  ae(e, h), Mt();
}
var Nx = Me("<p> </p>"), Dx = Me('<p> </p> <p><!></p> <div class="keysContainer svelte-1teaezc"></div>', 1), jx = Me('<div class="err"> </div>'), Bx = Me("<!> <!>", 1);
function Fx(e, t) {
  It(t, true);
  let i = Dt(), s = ie(""), l = ie(W([]));
  At(() => {
    p();
  });
  async function p() {
    var _a3;
    let B = await Nt(`/auth/v1/users/${t.user.id}/webauthn`);
    B.body ? $(l, W(B.body)) : $(s, W(((_a3 = B.error) == null ? void 0 : _a3.message) || "Error"));
  }
  async function m(B) {
    var _a3;
    let G = k(l).length === 1, F = await xl(`/auth/v1/users/${t.user.id}/webauthn/delete/${B}`);
    F.status === 200 ? (await p(), G && t.onSave()) : $(s, W(((_a3 = F.error) == null ? void 0 : _a3.message) || "Error"));
  }
  var w = Bx(), _ = Le(w);
  {
    var o = (B) => {
      var G = Nx(), F = ve(G, true);
      me(G), _e(() => xe(F, i.users.noMfaKeys)), ae(B, G);
    }, b = (B) => {
      var G = Dx(), F = Le(G), R = ve(F, true);
      me(F);
      var v = ne(F, 2), h = ve(v);
      xv(h, () => i.users.mfaDelete2), me(v);
      var g = ne(v, 2);
      Ho(g, 21, () => k(l), (S) => S.name, (S, C) => {
        _v(S, { get passkey() {
          return k(C);
        }, showDelete: true, onDelete: m });
      }), me(g), _e(() => xe(R, i.users.mfaDelete1)), ae(B, G);
    };
    Se(_, (B) => {
      k(l).length === 0 ? B(o) : B(b, false);
    });
  }
  var L = ne(_, 2);
  {
    var O = (B) => {
      var G = jx(), F = ve(G, true);
      me(G), _e(() => xe(F, k(s))), ae(B, G);
    };
    Se(L, (B) => {
      k(s) && B(O);
    });
  }
  ae(e, w), Mt();
}
var Vx = Me('<div class="err"> </div>'), $x = Me('<p> </p> <div class="flex gap-05"><!> <!></div> <!>', 1);
function Hx(e, t) {
  It(t, true);
  let i = Dt(), s = ie(""), l = ie(false);
  async function p() {
    $(s, "");
    let F = await xl(`/auth/v1/sessions/${t.userId}`);
    F.error ? $(s, W(F.error.message)) : ($(l, true), setTimeout(() => {
      $(l, false);
    }, 3e3));
  }
  var m = $x(), w = Le(m), _ = ve(w, true);
  me(w);
  var o = ne(w, 2), b = ve(o);
  wt(b, { level: -1, onclick: p, children: (F, R) => {
    Ke();
    var v = $e("Logout");
    ae(F, v);
  }, $$slots: { default: true } });
  var L = ne(b, 2);
  {
    var O = (F) => {
      Yo(F, {});
    };
    Se(L, (F) => {
      k(l) && F(O);
    });
  }
  me(o);
  var B = ne(o, 2);
  {
    var G = (F) => {
      var R = Vx(), v = ve(R, true);
      me(R), _e(() => xe(v, k(s))), ae(F, R);
    };
    Se(B, (F) => {
      k(s) && F(G);
    });
  }
  _e(() => xe(_, i.users.forceLogout)), ae(e, m), Mt();
}
var Ux = Me('<div class="err"> </div>'), Gx = Me("<p> </p> <!> <!>", 1);
function Wx(e, t) {
  It(t, true);
  let i = Yt(), s = Dt(), l = ie("");
  async function p() {
    $(l, "");
    let O = await xl(`/auth/v1/users/${t.userId}`);
    O.error ? $(l, W(O.error.message)) : t.onSave();
  }
  var m = Gx(), w = Le(m), _ = ve(w, true);
  me(w);
  var o = ne(w, 2);
  wt(o, { level: -1, onclick: p, children: (O, B) => {
    Ke();
    var G = $e();
    _e(() => xe(G, i.common.delete)), ae(O, G);
  }, $$slots: { default: true } });
  var b = ne(o, 2);
  {
    var L = (O) => {
      var B = Ux(), G = ve(B, true);
      me(B), _e(() => xe(G, k(l))), ae(O, B);
    };
    Se(b, (O) => {
      k(l) && O(L);
    });
  }
  _e(() => xe(_, s.users.deleteUser)), ae(e, m), Mt();
}
var zx = Me('<div class="err"> </div>'), Xx = Me('<!> <div class="flex"><!></div> <!>', 1);
function Kx(e, t) {
  It(t, true);
  let i = Yt(), s = Dt();
  const l = [i.account.navInfo, s.users.attributes, i.common.password, i.account.navMfa, i.account.devices, i.account.navLogout, i.common.delete];
  let p = ie(W(l[0])), m = ie(void 0), w = ie(""), _ = ie(void 0);
  At(() => {
    o();
  });
  async function o() {
    var _a3;
    let v = await Nt(`/auth/v1/users/${t.userId}`);
    v.body ? ($(_, W(v.body)), requestAnimationFrame(() => {
      var _a4;
      (_a4 = k(m)) == null ? void 0 : _a4();
    })) : $(w, W(((_a3 = v.error) == null ? void 0 : _a3.message) || "Error fetching user"));
  }
  var b = Xx(), L = Le(b);
  {
    var O = (v) => {
      var h = zx(), g = ve(h, true);
      me(h), _e(() => xe(g, k(w))), ae(v, h);
    };
    Se(L, (v) => {
      k(w) && v(O);
    });
  }
  var B = ne(L, 2), G = ve(B);
  bv(G, { tabs: l, get selected() {
    return k(p);
  }, set selected(v) {
    $(p, W(v));
  }, get focusFirst() {
    return k(m);
  }, set focusFirst(v) {
    $(m, W(v));
  } }), me(B);
  var F = ne(B, 2);
  {
    var R = (v) => {
      var h = st(), g = Le(h);
      {
        var S = (x) => {
          Ov(x, { get roles() {
            return t.roles;
          }, get groups() {
            return t.groups;
          }, get onSave() {
            return t.onSave;
          }, get user() {
            return k(_);
          }, set user(E) {
            $(_, W(E));
          } });
        }, C = (x) => {
          var E = st(), D = Le(E);
          {
            var z = (A) => {
              wx(A, { get user() {
                return k(_);
              }, get onSave() {
                return t.onSave;
              } });
            }, Z = (A) => {
              var N = st(), I = Le(N);
              {
                var M = (c) => {
                  Ox(c, { get user() {
                    return k(_);
                  }, get onSave() {
                    return t.onSave;
                  } });
                }, V = (c) => {
                  var r = st(), a = Le(r);
                  {
                    var n = (f) => {
                      Fx(f, { get user() {
                        return k(_);
                      }, get onSave() {
                        return t.onSave;
                      } });
                    }, u = (f) => {
                      var y = st(), P = Le(y);
                      {
                        var T = (H) => {
                          Cv(H, { viewMode: "admin", get userId() {
                            return t.userId;
                          } });
                        }, q = (H) => {
                          var J = st(), re = Le(J);
                          {
                            var Q = (de) => {
                              Hx(de, { get userId() {
                                return t.userId;
                              } });
                            }, pe = (de) => {
                              var fe = st(), he = Le(fe);
                              {
                                var we = (Oe) => {
                                  Wx(Oe, { get userId() {
                                    return t.userId;
                                  }, get onSave() {
                                    return t.onSave;
                                  } });
                                };
                                Se(he, (Oe) => {
                                  k(p) === l[6] && Oe(we);
                                }, true);
                              }
                              ae(de, fe);
                            };
                            Se(re, (de) => {
                              k(p) === l[5] ? de(Q) : de(pe, false);
                            }, true);
                          }
                          ae(H, J);
                        };
                        Se(P, (H) => {
                          k(p) === l[4] ? H(T) : H(q, false);
                        }, true);
                      }
                      ae(f, y);
                    };
                    Se(a, (f) => {
                      k(p) === l[3] ? f(n) : f(u, false);
                    }, true);
                  }
                  ae(c, r);
                };
                Se(I, (c) => {
                  k(p) === l[2] ? c(M) : c(V, false);
                }, true);
              }
              ae(A, N);
            };
            Se(D, (A) => {
              k(p) === l[1] ? A(z) : A(Z, false);
            }, true);
          }
          ae(x, E);
        };
        Se(g, (x) => {
          k(p) === l[0] ? x(S) : x(C, false);
        });
      }
      ae(v, h);
    };
    Se(F, (v) => {
      k(_) && v(R);
    });
  }
  ae(e, b), Mt();
}
var Jx = Me("<div></div> <!> <!>", 1), Qx = Me("<!> <!>", 1), Yx = Me('<div class="err"> </div>'), Zx = Me('<!> <div id="users"><!></div>', 1), e_ = Me("<!> <!>", 1);
function D_(e, t) {
  It(t, true);
  const i = (M, V = Nl, c = Nl) => {
    const r = _t(() => _.get() === V());
    pv(M, { onclick: () => _.set(V()), get selected() {
      return k(r);
    }, children: (a, n) => {
      Ke();
      var u = $e();
      _e(() => xe(u, c())), ae(a, u);
    } });
  };
  let s = ie(void 0), l = ie(""), p = ie(W([])), m = ie(W([])), w = ie(W([])), _ = dv("uid"), o = ie(void 0), b = ie(W([])), L = ie(W([])), O = ie(false), B = ie(void 0), G = ie(W(Pv)), F = ie(false), R = W(["E-Mail", "ID"]), v = ie(W(R[0])), h = ie(""), g = W(["E-Mail", "ID", "Created", "Last Login"]);
  gl(() => {
    C("page_size=" + k(G)), x(), E();
  }), At(() => {
    var _a3;
    $(o, W((_a3 = k(p).find((M) => M.id === _.get())) == null ? void 0 : _a3.id));
  }), At(() => {
    let M = k(h).toLowerCase();
    k(O) ? M.length < 3 ? k(F) && (C("page_size=" + k(G)), $(F, false)) : S(M) : M ? k(v) === R[0] ? $(m, W(k(p).filter((V) => {
      var _a3;
      return (_a3 = V.email) == null ? void 0 : _a3.toLowerCase().includes(M);
    }))) : k(v) === R[1] && $(m, W(k(p).filter((V) => V.id.toLowerCase().includes(M)))) : $(m, W(k(p)));
  });
  async function S(M) {
    $(B, void 0), $(F, true);
    let V;
    k(v) === R[0] ? V = "email" : V = "id";
    let c = await qv({ ty: "user", idx: V, q: M });
    c.body ? $(p, W(c.body)) : console.error(c.error);
  }
  async function C(M) {
    let V = "/auth/v1/users";
    M && (V += `?${M}`);
    let c = await Nt(V);
    return c.error ? $(l, "Error fetching users: " + c.error.message) : c.body && (c.status === 206 ? ($(O, true), $(B, W(c.headers))) : ($(O, false), $(B, void 0)), $(p, W(c.body))), [c.status, c.headers];
  }
  async function x() {
    var _a3;
    let M = await Nt("/auth/v1/roles");
    M.body ? $(L, W(M.body.toSorted((V, c) => V.name.localeCompare(c.name)))) : $(l, W(((_a3 = M.error) == null ? void 0 : _a3.message) || "Error"));
  }
  async function E() {
    var _a3;
    let M = await Nt("/auth/v1/groups");
    M.body ? $(b, W(M.body.toSorted((V, c) => V.name.localeCompare(c.name)))) : $(l, W(((_a3 = M.error) == null ? void 0 : _a3.message) || "Error"));
  }
  function D(M, V) {
    let c = V === "up";
    M === g[0] ? k(p).sort((r, a) => c ? r.email.localeCompare(a.email) : a.email.localeCompare(r.email)) : M === g[1] ? k(p).sort((r, a) => c ? r.id.localeCompare(a.id) : a.id.localeCompare(r.id)) : M === g[2] ? k(p).sort((r, a) => c ? r.created_at - a.created_at : a.created_at - r.created_at) : M === g[3] && k(p).sort((r, a) => {
      let n = r.last_login || 9999999999, u = r.last_login || 9999999999;
      return c ? n - u : u - n;
    });
  }
  async function z(M) {
    var _a3;
    (_a3 = k(s)) == null ? void 0 : _a3(), await C(), _.set(M);
  }
  function Z() {
    C(), x(), E(), $(h, "");
  }
  var A = e_(), N = Le(A);
  uv(N, { paddingTop: "2.1rem", buttonTilesAriaControls: "users", width: "min(23rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (V) => {
    var c = Jx(), r = Le(c);
    Xt(r, "height", ".5rem");
    var a = ne(r, 2);
    {
      var n = (T) => {
        var q = st(), H = Le(q);
        Ho(H, 17, () => k(p), (J) => J.id, (J, re) => {
          i(J, () => k(re).id, () => k(re).email);
        }), ae(T, q);
      }, u = (T) => {
        var q = st(), H = Le(q);
        Ho(H, 17, () => k(w), (J) => J.id, (J, re) => {
          i(J, () => k(re).id, () => k(re).email);
        }), ae(T, q);
      };
      Se(a, (T) => {
        k(B) ? T(n) : T(u, false);
      });
    }
    var f = ne(a, 2);
    {
      var y = (T) => {
        Tv(T, { sspFetch: C, idxTotalCount: "x-user-count", get itemsLength() {
          return k(p).length;
        }, get firstFetchHeaders() {
          return k(B);
        }, get pageSize() {
          return k(G);
        }, set pageSize(q) {
          $(G, W(q));
        } });
      }, P = (T) => {
        var q = st(), H = Le(q);
        {
          var J = (Q) => {
            Dl(Q, { get items() {
              return k(p);
            }, set items(pe) {
              $(p, W(pe));
            }, get itemsPaginated() {
              return k(w);
            }, set itemsPaginated(pe) {
              $(w, W(pe));
            } });
          }, re = (Q) => {
            Dl(Q, { get items() {
              return k(m);
            }, set items(pe) {
              $(m, W(pe));
            }, get itemsPaginated() {
              return k(w);
            }, set itemsPaginated(pe) {
              $(w, W(pe));
            } });
          };
          Se(H, (Q) => {
            k(O) ? Q(J) : Q(re, false);
          }, true);
        }
        ae(T, q);
      };
      Se(f, (T) => {
        k(B) ? T(y) : T(P, false);
      });
    }
    ae(V, c);
  }, children: (V, c) => {
    var r = Qx(), a = Le(r);
    const n = _t(() => k(L).length === 0 ? 1 : 2);
    cv(a, { get level() {
      return k(n);
    }, alignRight: true, get closeModal() {
      return k(s);
    }, set closeModal(f) {
      $(s, W(f));
    }, children: (f, y) => {
      kv(f, { onSave: z, get roles() {
        return k(L);
      }, get groups() {
        return k(b);
      } });
    }, $$slots: { default: true } });
    var u = ne(a, 2);
    lv(u, { get searchOptions() {
      return R;
    }, get orderOptions() {
      return g;
    }, onChangeOrder: D, searchWidth: "min(23rem, calc(100dvw - .5rem))", get value() {
      return k(h);
    }, set value(f) {
      $(h, W(f));
    }, get searchOption() {
      return k(v);
    }, set searchOption(f) {
      $(v, W(f));
    } }), ae(V, r);
  }, $$slots: { buttonTiles: true, default: true } });
  var I = ne(N, 2);
  fv(I, { children: (M, V) => {
    var c = Zx(), r = Le(c);
    {
      var a = (y) => {
        var P = Yx(), T = ve(P, true);
        me(P), _e(() => xe(T, k(l))), ae(y, P);
      };
      Se(r, (y) => {
        k(l) && y(a);
      });
    }
    var n = ne(r, 2), u = ve(n);
    {
      var f = (y) => {
        Kx(y, { get userId() {
          return k(o);
        }, get roles() {
          return k(L);
        }, get groups() {
          return k(b);
        }, onSave: Z });
      };
      Se(u, (y) => {
        k(o) && y(f);
      });
    }
    me(n), ae(M, c);
  } }), ae(e, A), Mt();
}
export {
  D_ as U
};
