import { t as K, a as y, e as j, d as Se } from "../chunks/YHhP1LbZ.js";
import { p as Z, aa as Q, s as u, c as v, ab as me, a as ee, a8 as R, t as N, j as e, a9 as U, r as c, k as T, l as s, f as W, a6 as ge } from "../chunks/Ck6jKiur.js";
import { s as V, r as Ae } from "../chunks/tDAaDMC_.js";
import { i as F } from "../chunks/7JDmqCCW.js";
import { e as he, i as Te } from "../chunks/Cj5zIR7o.js";
import { s as ne, a as ke, e as ze, S as Ce } from "../chunks/BTaFr7HN.js";
import { p as f } from "../chunks/ho0YXExL.js";
import { d as ye, c as Le, b as Ke, f as De } from "../chunks/BO1A6s0c.js";
import { C as Ne } from "../chunks/CovtrSYj.js";
import { N as Ee } from "../chunks/8C8fAzgF.js";
import { N as Ve } from "../chunks/BrquzDs9.js";
import { B as Me } from "../chunks/Dxa4r_PX.js";
import { u as Ge } from "../chunks/4NKlJeDw.js";
import { u as se } from "../chunks/BM7IgWpA.js";
import { O as Oe } from "../chunks/CiLAX3c1.js";
import { p as J, r as Be } from "../chunks/DZP54pO_.js";
import { B as X, t as te, s as ae } from "../chunks/Bd2Rvcxs.js";
import { A as be } from "../chunks/B21bTIl7.js";
import { I as Re } from "../chunks/CWz_piBP.js";
import { L as Fe } from "../chunks/Sykf3ifF.js";
import { I as re } from "../chunks/ZgUUKPWu.js";
import { I as xe } from "../chunks/xKzkh2rf.js";
import { u as le } from "../chunks/mN05BXqA.js";
import { f as oe, a as fe } from "../chunks/DswDW5U8.js";
import { k as pe } from "../chunks/C99p8-79.js";
import { h as ue } from "../chunks/DZNCtpeX.js";
import { b as Ue } from "../chunks/0HgaTnX3.js";
import { I as je } from "../chunks/DkYC4qk-.js";
import { a as qe, b as He, I as Ye } from "../chunks/C1PH3bBF.js";
import { T as We } from "../chunks/Doh1wgOS.js";
import { I as Je } from "../chunks/D75ao6x5.js";
import { j as Qe } from "../chunks/BRCxk8by.js";
import { F as Xe } from "../chunks/iKqYlEgo.js";
var Ze = K('<div class="center svelte-opxw4z"><!></div>'), et = K("<div><!></div>"), tt = K('<div class="row svelte-opxw4z"><!> <div class="center svelte-opxw4z"><!></div> <div class="center svelte-opxw4z"><!></div> <div class="center svelte-opxw4z"><!></div> <div class="center svelte-opxw4z"><!></div></div>'), at = K('<div class="matrix svelte-opxw4z"><div class="row svelte-opxw4z"><div></div> <!> <!> <!> <!></div> <!></div>');
function we(M, i) {
  Z(i, true), J(i, "finalize", 15)(L);
  let o = f([["Blacklist", false, false, false, false], ["Clients", false, false, false, false], ["Events", false, false, false, false], ["Generic", false, false, false, false], ["Groups", false, false, false, false], ["Roles", false, false, false, false], ["Secrets", false, false, false, false], ["Sessions", false, false, false, false], ["Scopes", false, false, false, false], ["UserAttributes", false, false, false, false], ["Users", false, false, false, false]]);
  Q(() => {
    if (n(), i.key) for (let m of i.key.access) {
      let t;
      switch (m.group) {
        case "Blacklist":
          t = 0;
          break;
        case "Clients":
          t = 1;
          break;
        case "Events":
          t = 2;
          break;
        case "Generic":
          t = 3;
          break;
        case "Groups":
          t = 4;
          break;
        case "Roles":
          t = 5;
          break;
        case "Secrets":
          t = 6;
          break;
        case "Sessions":
          t = 7;
          break;
        case "Scopes":
          t = 8;
          break;
        case "UserAttributes":
          t = 9;
          break;
        case "Users":
          t = 10;
          break;
      }
      if (t === void 0) {
        console.error("invalid idx during access key buildup");
        return;
      }
      m.access_rights.includes("create") && (o[t][1] = true), m.access_rights.includes("read") && (o[t][2] = true), m.access_rights.includes("update") && (o[t][3] = true), m.access_rights.includes("delete") && (o[t][4] = true);
    }
  });
  function L() {
    let m = [];
    for (let t of o) {
      let l = [];
      t[1] && l.push("create"), t[2] && l.push("read"), t[3] && l.push("update"), t[4] && l.push("delete"), l.length > 0 && m.push({ group: t[0], access_rights: l });
    }
    return m;
  }
  function n() {
    for (let m of o) m[1] = false, m[2] = false, m[3] = false, m[4] = false;
  }
  function x(m) {
    for (let t of o) if (t[0] === m) {
      t[1] && t[2] && t[3] && t[4] ? (t[1] = false, t[2] = false, t[3] = false, t[4] = false) : (t[1] = true, t[2] = true, t[3] = true, t[4] = true);
      break;
    }
  }
  function b(m) {
    let t;
    switch (m) {
      case "create":
        t = 1;
        break;
      case "read":
        t = 2;
        break;
      case "update":
        t = 3;
        break;
      case "delete":
        t = 4;
        break;
    }
    if (!t) {
      console.error("logic error in toggleRight, idx is undefined");
      return;
    }
    let l = true;
    for (let p of o) if (!p[t]) {
      l = false;
      break;
    }
    for (let p of o) p[t] = !l;
  }
  var z = at();
  {
    const m = (l, p = me) => {
      var _ = Ze(), k = v(_);
      const h = U(() => `Toggle: ${p()}`);
      X(k, { get ariaLabel() {
        return e(h);
      }, invisible: true, onclick: () => b(p()), children: (P, g) => {
        R();
        var r = j();
        N(() => V(r, p())), y(P, r);
      }, $$slots: { default: true } }), c(_), y(l, _);
    }, t = (l, p = me) => {
      var _ = et(), k = v(_);
      const h = U(() => `Toggle: ${p()}`);
      X(k, { get ariaLabel() {
        return e(h);
      }, invisible: true, onclick: () => x(p()), children: (P, g) => {
        R();
        var r = j();
        N(() => V(r, p())), y(P, r);
      }, $$slots: { default: true } }), c(_), y(l, _);
    };
    var w = v(z), S = u(v(w), 2);
    m(S, () => "create");
    var D = u(S, 2);
    m(D, () => "read");
    var $ = u(D, 2);
    m($, () => "update");
    var I = u($, 2);
    m(I, () => "delete"), c(w);
    var A = u(w, 2);
    he(A, 17, () => o, Te, (l, p, _) => {
      var k = tt(), h = v(k);
      t(h, () => e(p)[0]);
      var P = u(h, 2), g = v(P);
      const r = U(() => `${e(p)[0]}: create`);
      re(g, { get ariaLabel() {
        return e(r);
      }, get checked() {
        return o[_][1];
      }, set checked(Y) {
        o[_][1] = Y;
      } }), c(P);
      var a = u(P, 2), d = v(a);
      const C = U(() => `${e(p)[0]}: read`);
      re(d, { get ariaLabel() {
        return e(C);
      }, get checked() {
        return o[_][2];
      }, set checked(Y) {
        o[_][2] = Y;
      } }), c(a);
      var O = u(a, 2), B = v(O);
      const G = U(() => `${e(p)[0]}: update`);
      re(B, { get ariaLabel() {
        return e(G);
      }, get checked() {
        return o[_][3];
      }, set checked(Y) {
        o[_][3] = Y;
      } }), c(O);
      var q = u(O, 2), H = v(q);
      const ie = U(() => `${e(p)[0]}: delete`);
      re(H, { get ariaLabel() {
        return e(ie);
      }, get checked() {
        return o[_][4];
      }, set checked(Y) {
        o[_][4] = Y;
      } }), c(q), c(k), y(l, k);
    }), c(z);
  }
  y(M, z), ee();
}
var rt = K("<div><!></div>"), st = K('<div class="err"> </div>'), it = K('<!> <!> <!> <!> <div class="btn svelte-1j50zcx"><!> <!> <!></div>', 1);
function ot(M, i) {
  Z(i, true);
  let E = le(), o = se();
  const L = oe();
  let n = T(""), x = T(false), b = T(!!i.key.expires), z = T(f(oe())), w = T(f(fe())), S = T(void 0);
  Q(() => {
    i.key.name && s(b, !!i.key.expires);
  }), Q(() => {
    if (e(b)) {
      let r;
      i.key.expires ? r = new Date(i.key.expires * 1e3) : r = /* @__PURE__ */ new Date(), s(z, f(oe(r))), s(w, f(fe(r)));
    }
  });
  async function D() {
    if (s(n, ""), !e(S)) {
      console.error("finalizeMatrix is undefined");
      return;
    }
    let r = { name: i.key.name, access: e(S)() };
    if (r.access.length === 0) {
      s(n, "Grant the API Key at least one permission");
      return;
    }
    if (e(b)) {
      let d = be(e(z), e(w));
      if (!d) {
        s(n, "Invalid Date Input: User Expires");
        return;
      }
      r.exp = d;
    }
    let a = await ye(`/auth/v1/api_keys/${i.key.name}`, r);
    a.error ? s(n, f(a.error.message)) : (i.onSave(), s(x, true), setTimeout(() => {
      s(x, false), i.onSave();
    }, 2e3));
  }
  var $ = it(), I = W($);
  Fe(I, { get label() {
    return o.api_key.keyName;
  }, children: (r, a) => {
    R();
    var d = j();
    N(() => V(d, i.key.name)), y(r, d);
  }, $$slots: { default: true } });
  var A = u(I, 2);
  re(A, { get ariaLabel() {
    return o.api_key.limitedValidity;
  }, get checked() {
    return e(b);
  }, set checked(r) {
    s(b, f(r));
  }, children: (r, a) => {
    R();
    var d = j();
    N(() => V(d, o.api_key.limitedValidity)), y(r, d);
  }, $$slots: { default: true } });
  var m = u(A, 2);
  {
    var t = (r) => {
      var a = rt(), d = v(a);
      xe(d, { get label() {
        return o.api_key.expires;
      }, withTime: true, min: L, required: true, get value() {
        return e(z);
      }, set value(C) {
        s(z, f(C));
      }, get timeValue() {
        return e(w);
      }, set timeValue(C) {
        s(w, f(C));
      } }), c(a), te(3, a, () => ae), y(r, a);
    };
    F(m, (r) => {
      e(b) && r(t);
    });
  }
  var l = u(m, 2);
  we(l, { get key() {
    return i.key;
  }, get finalize() {
    return e(S);
  }, set finalize(r) {
    s(S, f(r));
  } });
  var p = u(l, 2), _ = v(p);
  X(_, { onclick: D, children: (r, a) => {
    R();
    var d = j();
    N(() => V(d, E.common.save)), y(r, d);
  }, $$slots: { default: true } });
  var k = u(_, 2);
  {
    var h = (r) => {
      Re(r, {});
    };
    F(k, (r) => {
      e(x) && r(h);
    });
  }
  var P = u(k, 2);
  {
    var g = (r) => {
      var a = st(), d = v(a, true);
      c(a), N(() => V(d, e(n))), y(r, a);
    };
    F(P, (r) => {
      e(n) && r(g);
    });
  }
  c(p), y(M, $), ee();
}
var lt = K("<div><!></div>"), nt = K("<div><!></div>"), dt = K("<div><!></div>"), vt = K('<div><div class="iconsOuter svelte-1pc4fk7"><div class="iconsInner svelte-1pc4fk7"><!> <!></div></div> <textarea></textarea></div>');
function _e(M, i) {
  Z(i, true);
  let E = J(i, "ariaLabel", 3, ""), o = J(i, "value", 3, ""), L = J(i, "rows", 3, 10), n = J(i, "cols", 3, 60), x = J(i, "show", 15, false), b = J(i, "width", 3, "min(25rem, calc(100dvw - .5rem))"), z = Be(i, ["$$slots", "$$events", "$$legacy", "ariaLabel", "value", "rows", "cols", "show", "width"]), w = le(), S = T(""), D = T("");
  Q(() => {
    x() ? s(D, o()) : s(D, f(e(S)));
  }), ge(() => {
    for (let g = 0; g < o().length; g++) s(S, e(S) + "*");
    s(D, f(e(S))), x(false);
  });
  function $() {
    navigator.clipboard.writeText(o());
  }
  function I() {
    x(!x());
  }
  var A = vt();
  let m;
  var t = v(A), l = v(t), p = v(l);
  const _ = U(() => x() ? w.common.hide : w.common.show);
  X(p, { get ariaLabel() {
    return e(_);
  }, invisible: true, onclick: I, children: (g, r) => {
    var a = Se(), d = W(a);
    {
      var C = (B) => {
        var G = lt(), q = v(G);
        qe(q, { width: 22 }), c(G), N(() => ne(G, "title", w.common.hide)), y(B, G);
      }, O = (B) => {
        var G = nt(), q = v(G);
        He(q, { width: 22 }), c(G), N(() => ne(G, "title", w.common.show)), y(B, G);
      };
      F(d, (B) => {
        x() ? B(C) : B(O, false);
      });
    }
    y(g, a);
  }, $$slots: { default: true } });
  var k = u(p, 2);
  X(k, { get ariaLabel() {
    return w.common.copyToClip;
  }, invisible: true, onclick: $, children: (g, r) => {
    var a = dt(), d = v(a);
    je(d, {}), c(a), N(() => ne(a, "title", w.common.copyToClip)), y(g, a);
  }, $$slots: { default: true } }), c(l), c(t);
  var h = u(t, 2);
  Ae(h);
  let P;
  c(A), N(() => {
    m = ke(A, "", m, { width: `${b()}` }), P = ze(h, P, { "aria-label": E(), disabled: true, rows: L(), cols: n(), ...z, [Ce]: { width: b(), "padding-right": "2.75rem" } }, "svelte-1pc4fk7");
  }), Ue(h, () => e(D), (g) => s(D, g)), y(M, A), ee();
}
var ct = K('<div class="secret font-mono svelte-1ii2aob"><!></div> <div><p><!></p> <p><span class="headerCode"><b><code></code></b></span></p></div> <div><p><!></p> <!></div> <div><p><!></p> <!></div>', 1), ut = K('<p> </p> <p> </p> <div class="btn svelte-1ii2aob"><!></div> <!> <div class="err"> </div>', 1);
function ft(M, i) {
  Z(i, true);
  let E = le(), o = se(), L = T(""), n = T(""), x = T(""), b = T("");
  Q(() => {
    i.key.name && s(n, "");
  });
  async function z() {
    var _a;
    let h = await ye(`/auth/v1/api_keys/${i.key.name}/secret`);
    h.text ? (s(n, f(h.text)), s(x, `curl -s -H 'Authorization: API-Key ${h.text}' ${window.location.origin}/auth/v1/api_keys/${i.key.name}/test`), s(b, `${e(x)} | jq`)) : s(L, f(((_a = h.error) == null ? void 0 : _a.message) || "Error"));
  }
  var w = ut(), S = W(w), D = v(S, true);
  c(S);
  var $ = u(S, 2), I = v($, true);
  c($);
  var A = u($, 2), m = v(A);
  const t = U(() => e(n) ? 3 : 1);
  X(m, { get level() {
    return e(t);
  }, onclick: z, children: (h, P) => {
    R();
    var g = j();
    N(() => V(g, E.passwordReset.generate)), y(h, g);
  }, $$slots: { default: true } }), c(A);
  var l = u(A, 2);
  {
    var p = (h) => {
      var P = ct(), g = W(P), r = v(g);
      Ye(r, { autocomplete: "off", get value() {
        return e(n);
      }, label: "API Key", placeholder: "API Key", disabled: true, showCopy: true, width: "min(25rem, calc(100dvw - .5rem))" }), c(g);
      var a = u(g, 2), d = v(a), C = v(d);
      ue(C, () => o.api_key.generate3), c(d);
      var O = u(d, 2), B = v(O), G = v(B), q = v(G);
      q.textContent = "API-Key <api_key>", c(G), c(B), c(O), c(a);
      var H = u(a, 2), ie = v(H), Y = v(ie);
      ue(Y, () => o.api_key.generate4), c(ie);
      var $e = u(ie, 2);
      pe($e, () => e(b), (ce) => {
        _e(ce, { ariaLabel: "curl text command with jq", rows: 5, get value() {
          return e(b);
        } });
      }), c(H);
      var de = u(H, 2), ve = v(de), Ie = v(ve);
      ue(Ie, () => o.api_key.generate5), c(ve);
      var Pe = u(ve, 2);
      pe(Pe, () => e(b), (ce) => {
        _e(ce, { ariaLabel: "curl text command simple", rows: 5, get value() {
          return e(x);
        } });
      }), c(de), te(3, g, () => ae, () => ({ duration: 150 })), te(3, a, () => ae, () => ({ duration: 150 })), te(3, H, () => ae, () => ({ duration: 150 })), te(3, de, () => ae, () => ({ duration: 150 })), y(h, P);
    };
    F(l, (h) => {
      e(n) && h(p);
    });
  }
  var _ = u(l, 2), k = v(_, true);
  c(_), N(() => {
    V(D, o.api_key.generate1), V(I, o.api_key.generate2), V(k, e(L));
  }), y(M, w), ee();
}
var mt = K('<div class="err"> </div>'), pt = K("<p> </p> <!> <!>", 1);
function _t(M, i) {
  Z(i, true);
  let E = le(), o = se(), L = T("");
  async function n() {
    s(L, "");
    let $ = await Le(`/auth/v1/api_keys/${i.key.name}`);
    $.error ? s(L, f($.error.message)) : i.onSave();
  }
  var x = pt(), b = W(x), z = v(b, true);
  c(b);
  var w = u(b, 2);
  X(w, { level: -1, onclick: n, children: ($, I) => {
    R();
    var A = j();
    N(() => V(A, E.common.delete)), y($, A);
  }, $$slots: { default: true } });
  var S = u(w, 2);
  {
    var D = ($) => {
      var I = mt(), A = v(I, true);
      c(I), N(() => V(A, e(L))), y($, I);
    };
    F(S, ($) => {
      e(L) && $(D);
    });
  }
  N(() => V(z, o.api_key.delete1)), y(M, x), ee();
}
var gt = K('<div><div class="flex"><!></div> <!></div>');
function ht(M, i) {
  Z(i, true);
  let E = J(i, "key", 15), o = se();
  const L = [o.tabs.config, "Secret", o.tabs.delete];
  let n = T(f(L[0])), x = T(void 0);
  Q(() => {
    var _a;
    E().name && ((_a = e(x)) == null ? void 0 : _a());
  });
  var b = gt(), z = v(b), w = v(z);
  We(w, { tabs: L, get selected() {
    return e(n);
  }, set selected(I) {
    s(n, f(I));
  }, get focusFirst() {
    return e(x);
  }, set focusFirst(I) {
    s(x, f(I));
  } }), c(z);
  var S = u(z, 2);
  {
    var D = (I) => {
      ot(I, { get onSave() {
        return i.onSave;
      }, get key() {
        return E();
      }, set key(A) {
        E(A);
      } });
    }, $ = (I, A) => {
      {
        var m = (l) => {
          ft(l, { get key() {
            return E();
          } });
        }, t = (l, p) => {
          {
            var _ = (k) => {
              _t(k, { get key() {
                return E();
              }, get onSave() {
                return i.onSave;
              } });
            };
            F(l, (k) => {
              e(n) === o.tabs.delete && k(_);
            }, p);
          }
        };
        F(I, (l) => {
          e(n) === "Secret" ? l(m) : l(t, false);
        }, A);
      }
    };
    F(S, (I) => {
      e(n) === o.tabs.config ? I(D) : I($, false);
    });
  }
  c(b), y(M, b), ee();
}
var kt = K("<div><!></div>"), yt = K('<!> <!> <!> <!> <!> <div class="err"> </div>', 1), bt = K('<div class="container svelte-1dweyfr"><!></div>');
function xt(M, i) {
  Z(i, true);
  const E = oe();
  let o = le(), L = se(), n = T(""), x = T(false), b = T(void 0), z = T(""), w = T(f(oe())), S = T(f(fe()));
  async function D(A, m) {
    var _a;
    s(n, "");
    for (let _ of i.keys) if (_.name === e(z)) {
      s(n, "Name already exists");
      return;
    }
    let t = (_a = e(b)) == null ? void 0 : _a();
    if (!t) {
      console.error("access rights is undefined");
      return;
    }
    let l = { name: e(z), access: t };
    if (l.access.length === 0) {
      s(n, "Grant the new API Key at least one permission");
      return;
    }
    if (e(x)) {
      if (console.log(), !e(w) || !e(S)) {
        s(n, "Disable expiry or provide an valid date and time");
        return;
      }
      let _ = be(e(w), e(S));
      if (!_) {
        s(n, "Invalid Date Input: User Expires");
        return;
      }
      l.exp = _;
    }
    let p = await Ke(A.action, l);
    p.error ? s(n, f(p.error.message)) : i.onSave();
  }
  var $ = bt(), I = v($);
  Xe(I, { action: "/auth/v1/api_keys", onSubmit: D, children: (A, m) => {
    var t = yt(), l = W(t);
    Je(l, { get label() {
      return L.api_key.keyName;
    }, get placeholder() {
      return L.api_key.keyName;
    }, autocomplete: "off", required: true, min: "2", max: "24", pattern: Qe, get value() {
      return e(z);
    }, set value(a) {
      s(z, f(a));
    } });
    var p = u(l, 2);
    re(p, { get ariaLabel() {
      return L.api_key.limitedValidity;
    }, get checked() {
      return e(x);
    }, set checked(a) {
      s(x, f(a));
    }, children: (a, d) => {
      R();
      var C = j();
      N(() => V(C, L.api_key.limitedValidity)), y(a, C);
    }, $$slots: { default: true } });
    var _ = u(p, 2);
    {
      var k = (a) => {
        var d = kt(), C = v(d);
        xe(C, { get label() {
          return L.api_key.expires;
        }, withTime: true, min: E, required: true, get value() {
          return e(w);
        }, set value(O) {
          s(w, f(O));
        }, get timeValue() {
          return e(S);
        }, set timeValue(O) {
          s(S, f(O));
        } }), c(d), te(3, d, () => ae), y(a, d);
      };
      F(_, (a) => {
        e(x) && a(k);
      });
    }
    var h = u(_, 2);
    we(h, { get finalize() {
      return e(b);
    }, set finalize(a) {
      s(b, f(a));
    } });
    var P = u(h, 2);
    X(P, { type: "submit", children: (a, d) => {
      R();
      var C = j();
      N(() => V(C, o.common.save)), y(a, C);
    }, $$slots: { default: true } });
    var g = u(P, 2), r = v(g, true);
    c(g), N(() => V(r, e(n))), y(A, t);
  }, $$slots: { default: true } }), c($), y(M, $), ee();
}
var wt = K("<div></div> <!>", 1), $t = K("<!> <!>", 1), It = K('<div id="keys"><!></div>'), Pt = K(" <!> <!>", 1);
function St(M, i) {
  Z(i, true);
  let E = se(), o = T(void 0), L = T(""), n = T(f([])), x = T(f([])), b = T(void 0), z = Ge("kn");
  const w = ["Name"], S = ["Name"];
  let D = T(f(S[0])), $ = T("");
  ge(() => {
    I();
  }), Q(() => {
    s(b, f(e(n).find((k) => k.name === z.get())));
  }), Q(() => {
    let k = e($).toLowerCase();
    k ? e(D) === S[0] && s(x, f(e(n).filter((h) => {
      var _a;
      return (_a = h.name) == null ? void 0 : _a.includes(k);
    }))) : s(x, f(e(n)));
  });
  async function I() {
    var _a;
    let k = await De("/auth/v1/api_keys");
    k.body ? s(n, f(k.body.keys)) : s(L, f(((_a = k.error) == null ? void 0 : _a.message) || "Error"));
  }
  function A(k, h) {
    let P = h === "up";
    k === w[0] && e(n).sort((g, r) => P ? g.name.localeCompare(r.name) : r.name.localeCompare(g.name));
  }
  function m() {
    var _a;
    (_a = e(o)) == null ? void 0 : _a(), I();
  }
  R();
  var t = Pt(), l = W(t), p = u(l);
  Ve(p, { paddingTop: "2.1rem", buttonTilesAriaControls: "keys", width: "min(20rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (h) => {
    var P = wt(), g = W(P);
    ke(g, "", {}, { height: ".5rem" });
    var r = u(g, 2);
    he(r, 17, () => e(x), (a) => a.name, (a, d, C, O) => {
      const B = U(() => z.get() === e(d).name);
      Ee(a, { onclick: () => z.set(e(d).name), get selected() {
        return e(B);
      }, children: (G, q) => {
        R();
        var H = j();
        N(() => V(H, e(d).name)), y(G, H);
      } });
    }), y(h, P);
  }, children: (h, P) => {
    var g = $t(), r = W(g);
    const a = U(() => e(n).length === 0 ? 1 : 2);
    Me(r, { get level() {
      return e(a);
    }, alignRight: true, get closeModal() {
      return e(o);
    }, set closeModal(C) {
      s(o, f(C));
    }, children: (C, O) => {
      xt(C, { get keys() {
        return e(n);
      }, onSave: m });
    }, $$slots: { default: true } });
    var d = u(r, 2);
    Oe(d, { searchOptions: S, orderOptions: w, onChangeOrder: A, searchWidth: "min(19.5rem, 100dvw - .5rem)", get searchOption() {
      return e(D);
    }, set searchOption(C) {
      s(D, f(C));
    }, get value() {
      return e($);
    }, set value(C) {
      s($, f(C));
    } }), y(h, g);
  }, $$slots: { buttonTiles: true, default: true } });
  var _ = u(p, 2);
  Ne(_, { children: (k, h) => {
    var P = It(), g = v(P);
    {
      var r = (a) => {
        ht(a, { onSave: I, get key() {
          return e(b);
        }, set key(d) {
          s(b, f(d));
        } });
      };
      F(g, (a) => {
        e(b) && a(r);
      });
    }
    c(P), N(() => ne(P, "aria-label", E.common.details)), y(k, P);
  } }), N(() => V(l, `${e(L) ?? ""} `)), y(M, t), ee();
}
function la(M) {
  St(M, {});
}
export {
  la as component
};
