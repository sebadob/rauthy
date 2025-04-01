import { t as L, a as k, e as U, d as Se } from "../chunks/BxmJRzoY.js";
import { p as Z, a1 as Y, a5 as Q, s as u, c as v, a6 as me, a as ee, a4 as F, a3 as B, t as D, k as e, r as c, j as A, l as s, f as W, a0 as ge } from "../chunks/w0HvPX0p.js";
import { s as E, r as Ae } from "../chunks/BzP2S3Z_.js";
import { i as R } from "../chunks/iO9_dPNE.js";
import { e as he, i as Te } from "../chunks/S81raU5Y.js";
import { s as ne, a as ke, e as ze, S as Ce } from "../chunks/BdbQ6g_y.js";
import { d as ye, c as Le, b as Ke, f as De } from "../chunks/BO1A6s0c.js";
import { C as Ne } from "../chunks/C6jTHtu1.js";
import { N as Ee } from "../chunks/D9abP6hj.js";
import { N as Ve } from "../chunks/CzQKNE2_.js";
import { B as Me } from "../chunks/DoeALvoe.js";
import { u as Ge } from "../chunks/Cy3hLAXJ.js";
import { u as se } from "../chunks/DHOKTGcE.js";
import { O as Oe } from "../chunks/85nGGue2.js";
import { p as J, r as Be } from "../chunks/C6GSeq7M.js";
import { B as X, t as te, s as re } from "../chunks/C8YTstTD.js";
import { A as be } from "../chunks/B21bTIl7.js";
import { I as Re } from "../chunks/CTshzOVc.js";
import { L as Fe } from "../chunks/DOFJTuej.js";
import { I as ae } from "../chunks/ScYc5fRW.js";
import { I as xe } from "../chunks/cZptBaPP.js";
import { u as le } from "../chunks/0cG6LBdy.js";
import { f as oe, a as fe } from "../chunks/DswDW5U8.js";
import { k as pe } from "../chunks/DDNkWuIk.js";
import { h as ue } from "../chunks/C2ZdIFW_.js";
import { b as Ue } from "../chunks/Cxw7xmE1.js";
import { I as qe } from "../chunks/BR_xb8zP.js";
import { a as je, b as He, I as Ye } from "../chunks/iedauS3r.js";
import { T as We } from "../chunks/BdAKL3gn.js";
import { I as Je } from "../chunks/Q4PIg3iI.js";
import { h as Qe } from "../chunks/gfDO7tLr.js";
import { F as Xe } from "../chunks/CDe-qvZi.js";
var Ze = L('<div class="center svelte-opxw4z"><!></div>'), et = L("<div><!></div>"), tt = L('<div class="row svelte-opxw4z"><!> <div class="center svelte-opxw4z"><!></div> <div class="center svelte-opxw4z"><!></div> <div class="center svelte-opxw4z"><!></div> <div class="center svelte-opxw4z"><!></div></div>'), rt = L('<div class="matrix svelte-opxw4z"><div class="row svelte-opxw4z"><div></div> <!> <!> <!> <!></div> <!></div>');
function we(V, i) {
  Z(i, true), J(i, "finalize", 15)(C);
  let o = Y([["Blacklist", false, false, false, false], ["Clients", false, false, false, false], ["Events", false, false, false, false], ["Generic", false, false, false, false], ["Groups", false, false, false, false], ["Roles", false, false, false, false], ["Secrets", false, false, false, false], ["Sessions", false, false, false, false], ["Scopes", false, false, false, false], ["UserAttributes", false, false, false, false], ["Users", false, false, false, false]]);
  Q(() => {
    if (n(), i.key) for (let f of i.key.access) {
      let t;
      switch (f.group) {
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
      f.access_rights.includes("create") && (o[t][1] = true), f.access_rights.includes("read") && (o[t][2] = true), f.access_rights.includes("update") && (o[t][3] = true), f.access_rights.includes("delete") && (o[t][4] = true);
    }
  });
  function C() {
    let f = [];
    for (let t of o) {
      let l = [];
      t[1] && l.push("create"), t[2] && l.push("read"), t[3] && l.push("update"), t[4] && l.push("delete"), l.length > 0 && f.push({ group: t[0], access_rights: l });
    }
    return f;
  }
  function n() {
    for (let f of o) f[1] = false, f[2] = false, f[3] = false, f[4] = false;
  }
  function b(f) {
    for (let t of o) if (t[0] === f) {
      t[1] && t[2] && t[3] && t[4] ? (t[1] = false, t[2] = false, t[3] = false, t[4] = false) : (t[1] = true, t[2] = true, t[3] = true, t[4] = true);
      break;
    }
  }
  function y(f) {
    let t;
    switch (f) {
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
    for (let m of o) if (!m[t]) {
      l = false;
      break;
    }
    for (let m of o) m[t] = !l;
  }
  var T = rt();
  {
    const f = (l, m = me) => {
      var p = Ze(), h = v(p);
      const g = F(() => `Toggle: ${m()}`);
      X(h, { get ariaLabel() {
        return e(g);
      }, invisible: true, onclick: () => y(m()), children: (I, _) => {
        B();
        var a = U();
        D(() => E(a, m())), k(I, a);
      }, $$slots: { default: true } }), c(p), k(l, p);
    }, t = (l, m = me) => {
      var p = et(), h = v(p);
      const g = F(() => `Toggle: ${m()}`);
      X(h, { get ariaLabel() {
        return e(g);
      }, invisible: true, onclick: () => b(m()), children: (I, _) => {
        B();
        var a = U();
        D(() => E(a, m())), k(I, a);
      }, $$slots: { default: true } }), c(p), k(l, p);
    };
    var x = v(T), P = u(v(x), 2);
    f(P, () => "create");
    var K = u(P, 2);
    f(K, () => "read");
    var w = u(K, 2);
    f(w, () => "update");
    var $ = u(w, 2);
    f($, () => "delete"), c(x);
    var S = u(x, 2);
    he(S, 17, () => o, Te, (l, m, p) => {
      var h = tt(), g = v(h);
      t(g, () => e(m)[0]);
      var I = u(g, 2), _ = v(I);
      const a = F(() => `${e(m)[0]}: create`);
      ae(_, { get ariaLabel() {
        return e(a);
      }, get checked() {
        return o[p][1];
      }, set checked(H) {
        o[p][1] = H;
      } }), c(I);
      var r = u(I, 2), d = v(r);
      const z = F(() => `${e(m)[0]}: read`);
      ae(d, { get ariaLabel() {
        return e(z);
      }, get checked() {
        return o[p][2];
      }, set checked(H) {
        o[p][2] = H;
      } }), c(r);
      var G = u(r, 2), O = v(G);
      const M = F(() => `${e(m)[0]}: update`);
      ae(O, { get ariaLabel() {
        return e(M);
      }, get checked() {
        return o[p][3];
      }, set checked(H) {
        o[p][3] = H;
      } }), c(G);
      var q = u(G, 2), j = v(q);
      const ie = F(() => `${e(m)[0]}: delete`);
      ae(j, { get ariaLabel() {
        return e(ie);
      }, get checked() {
        return o[p][4];
      }, set checked(H) {
        o[p][4] = H;
      } }), c(q), c(h), k(l, h);
    }), c(T);
  }
  k(V, T), ee();
}
var at = L("<div><!></div>"), st = L('<div class="err"> </div>'), it = L('<!> <!> <!> <!> <div class="btn svelte-1j50zcx"><!> <!> <!></div>', 1);
function ot(V, i) {
  Z(i, true);
  let N = le(), o = se();
  const C = oe();
  let n = A(""), b = A(false), y = A(!!i.key.expires), T = A(Y(oe())), x = A(Y(fe())), P = A(void 0);
  Q(() => {
    i.key.name && s(y, !!i.key.expires);
  }), Q(() => {
    if (e(y)) {
      let a;
      i.key.expires ? a = new Date(i.key.expires * 1e3) : a = /* @__PURE__ */ new Date(), s(T, oe(a), true), s(x, fe(a), true);
    }
  });
  async function K() {
    if (s(n, ""), !e(P)) {
      console.error("finalizeMatrix is undefined");
      return;
    }
    let a = { name: i.key.name, access: e(P)() };
    if (a.access.length === 0) {
      s(n, "Grant the API Key at least one permission");
      return;
    }
    if (e(y)) {
      let d = be(e(T), e(x));
      if (!d) {
        s(n, "Invalid Date Input: User Expires");
        return;
      }
      a.exp = d;
    }
    let r = await ye(`/auth/v1/api_keys/${i.key.name}`, a);
    r.error ? s(n, r.error.message, true) : (i.onSave(), s(b, true), setTimeout(() => {
      s(b, false), i.onSave();
    }, 2e3));
  }
  var w = it(), $ = W(w);
  Fe($, { get label() {
    return o.api_key.keyName;
  }, children: (a, r) => {
    B();
    var d = U();
    D(() => E(d, i.key.name)), k(a, d);
  }, $$slots: { default: true } });
  var S = u($, 2);
  ae(S, { get ariaLabel() {
    return o.api_key.limitedValidity;
  }, get checked() {
    return e(y);
  }, set checked(a) {
    s(y, a, true);
  }, children: (a, r) => {
    B();
    var d = U();
    D(() => E(d, o.api_key.limitedValidity)), k(a, d);
  }, $$slots: { default: true } });
  var f = u(S, 2);
  {
    var t = (a) => {
      var r = at(), d = v(r);
      xe(d, { get label() {
        return o.api_key.expires;
      }, withTime: true, min: C, required: true, get value() {
        return e(T);
      }, set value(z) {
        s(T, z, true);
      }, get timeValue() {
        return e(x);
      }, set timeValue(z) {
        s(x, z, true);
      } }), c(r), te(3, r, () => re, () => ({ duration: 150 })), k(a, r);
    };
    R(f, (a) => {
      e(y) && a(t);
    });
  }
  var l = u(f, 2);
  we(l, { get key() {
    return i.key;
  }, get finalize() {
    return e(P);
  }, set finalize(a) {
    s(P, a, true);
  } });
  var m = u(l, 2), p = v(m);
  X(p, { onclick: K, children: (a, r) => {
    B();
    var d = U();
    D(() => E(d, N.common.save)), k(a, d);
  }, $$slots: { default: true } });
  var h = u(p, 2);
  {
    var g = (a) => {
      Re(a, {});
    };
    R(h, (a) => {
      e(b) && a(g);
    });
  }
  var I = u(h, 2);
  {
    var _ = (a) => {
      var r = st(), d = v(r, true);
      c(r), D(() => E(d, e(n))), k(a, r);
    };
    R(I, (a) => {
      e(n) && a(_);
    });
  }
  c(m), k(V, w), ee();
}
var lt = L("<div><!></div>"), nt = L("<div><!></div>"), dt = L("<div><!></div>"), vt = L('<div><div class="iconsOuter svelte-1pc4fk7"><div class="iconsInner svelte-1pc4fk7"><!> <!></div></div> <textarea></textarea></div>');
function _e(V, i) {
  Z(i, true);
  let N = J(i, "ariaLabel", 3, ""), o = J(i, "value", 3, ""), C = J(i, "rows", 3, 10), n = J(i, "cols", 3, 60), b = J(i, "show", 15, false), y = J(i, "width", 3, "min(25rem, calc(100dvw - .5rem))"), T = Be(i, ["$$slots", "$$events", "$$legacy", "ariaLabel", "value", "rows", "cols", "show", "width"]), x = le(), P = A(""), K = A("");
  Q(() => {
    b() ? s(K, o()) : s(K, e(P), true);
  }), ge(() => {
    for (let _ = 0; _ < o().length; _++) s(P, e(P) + "*");
    s(K, e(P), true), b(false);
  });
  function w() {
    navigator.clipboard.writeText(o());
  }
  function $() {
    b(!b());
  }
  var S = vt();
  let f;
  var t = v(S), l = v(t), m = v(l);
  const p = F(() => b() ? x.common.hide : x.common.show);
  X(m, { get ariaLabel() {
    return e(p);
  }, invisible: true, onclick: $, children: (_, a) => {
    var r = Se(), d = W(r);
    {
      var z = (O) => {
        var M = lt(), q = v(M);
        je(q, { width: 22 }), c(M), D(() => ne(M, "title", x.common.hide)), k(O, M);
      }, G = (O) => {
        var M = nt(), q = v(M);
        He(q, { width: 22 }), c(M), D(() => ne(M, "title", x.common.show)), k(O, M);
      };
      R(d, (O) => {
        b() ? O(z) : O(G, false);
      });
    }
    k(_, r);
  }, $$slots: { default: true } });
  var h = u(m, 2);
  X(h, { get ariaLabel() {
    return x.common.copyToClip;
  }, invisible: true, onclick: w, children: (_, a) => {
    var r = dt(), d = v(r);
    qe(d, {}), c(r), D(() => ne(r, "title", x.common.copyToClip)), k(_, r);
  }, $$slots: { default: true } }), c(l), c(t);
  var g = u(t, 2);
  Ae(g);
  let I;
  c(S), D(() => {
    f = ke(S, "", f, { width: `${y()}` }), I = ze(g, I, { "aria-label": N(), disabled: true, rows: C(), cols: n(), ...T, [Ce]: { width: y(), "padding-right": "2.75rem" } }, "svelte-1pc4fk7");
  }), Ue(g, () => e(K), (_) => s(K, _)), k(V, S), ee();
}
var ct = L('<div class="secret font-mono svelte-1ii2aob"><!></div> <div><p><!></p> <p><span class="headerCode"><b><code></code></b></span></p></div> <div><p><!></p> <!></div> <div><p><!></p> <!></div>', 1), ut = L('<p> </p> <p> </p> <div class="btn svelte-1ii2aob"><!></div> <!> <div class="err"> </div>', 1);
function ft(V, i) {
  Z(i, true);
  let N = le(), o = se(), C = A(""), n = A(""), b = A(""), y = A("");
  Q(() => {
    i.key.name && s(n, "");
  });
  async function T() {
    var _a;
    let g = await ye(`/auth/v1/api_keys/${i.key.name}/secret`);
    g.text ? (s(n, g.text, true), s(b, `curl -s -H 'Authorization: API-Key ${g.text}' ${window.location.origin}/auth/v1/api_keys/${i.key.name}/test`), s(y, `${e(b)} | jq`)) : s(C, ((_a = g.error) == null ? void 0 : _a.message) || "Error", true);
  }
  var x = ut(), P = W(x), K = v(P, true);
  c(P);
  var w = u(P, 2), $ = v(w, true);
  c(w);
  var S = u(w, 2), f = v(S);
  const t = F(() => e(n) ? 3 : 1);
  X(f, { get level() {
    return e(t);
  }, onclick: T, children: (g, I) => {
    B();
    var _ = U();
    D(() => E(_, N.passwordReset.generate)), k(g, _);
  }, $$slots: { default: true } }), c(S);
  var l = u(S, 2);
  {
    var m = (g) => {
      var I = ct(), _ = W(I), a = v(_);
      Ye(a, { autocomplete: "off", get value() {
        return e(n);
      }, label: "API Key", placeholder: "API Key", disabled: true, showCopy: true, width: "min(25rem, calc(100dvw - .5rem))" }), c(_);
      var r = u(_, 2), d = v(r), z = v(d);
      ue(z, () => o.api_key.generate3), c(d);
      var G = u(d, 2), O = v(G), M = v(O), q = v(M);
      q.textContent = "API-Key <api_key>", c(M), c(O), c(G), c(r);
      var j = u(r, 2), ie = v(j), H = v(ie);
      ue(H, () => o.api_key.generate4), c(ie);
      var $e = u(ie, 2);
      pe($e, () => e(y), (ce) => {
        _e(ce, { ariaLabel: "curl text command with jq", rows: 5, get value() {
          return e(y);
        } });
      }), c(j);
      var de = u(j, 2), ve = v(de), Ie = v(ve);
      ue(Ie, () => o.api_key.generate5), c(ve);
      var Pe = u(ve, 2);
      pe(Pe, () => e(y), (ce) => {
        _e(ce, { ariaLabel: "curl text command simple", rows: 5, get value() {
          return e(b);
        } });
      }), c(de), te(3, _, () => re, () => ({ duration: 150 })), te(3, r, () => re, () => ({ duration: 150 })), te(3, j, () => re, () => ({ duration: 150 })), te(3, de, () => re, () => ({ duration: 150 })), k(g, I);
    };
    R(l, (g) => {
      e(n) && g(m);
    });
  }
  var p = u(l, 2), h = v(p, true);
  c(p), D(() => {
    E(K, o.api_key.generate1), E($, o.api_key.generate2), E(h, e(C));
  }), k(V, x), ee();
}
var mt = L('<div class="err"> </div>'), pt = L("<p> </p> <!> <!>", 1);
function _t(V, i) {
  Z(i, true);
  let N = le(), o = se(), C = A("");
  async function n() {
    s(C, "");
    let w = await Le(`/auth/v1/api_keys/${i.key.name}`);
    w.error ? s(C, w.error.message, true) : i.onSave();
  }
  var b = pt(), y = W(b), T = v(y, true);
  c(y);
  var x = u(y, 2);
  X(x, { level: -1, onclick: n, children: (w, $) => {
    B();
    var S = U();
    D(() => E(S, N.common.delete)), k(w, S);
  }, $$slots: { default: true } });
  var P = u(x, 2);
  {
    var K = (w) => {
      var $ = mt(), S = v($, true);
      c($), D(() => E(S, e(C))), k(w, $);
    };
    R(P, (w) => {
      e(C) && w(K);
    });
  }
  D(() => E(T, o.api_key.delete1)), k(V, b), ee();
}
var gt = L('<div><div class="flex"><!></div> <!></div>');
function ht(V, i) {
  Z(i, true);
  let N = J(i, "key", 15), o = se();
  const C = [o.tabs.config, "Secret", o.tabs.delete];
  let n = A(Y(C[0])), b = A(void 0);
  Q(() => {
    var _a;
    N().name && ((_a = e(b)) == null ? void 0 : _a());
  });
  var y = gt(), T = v(y), x = v(T);
  We(x, { tabs: C, get selected() {
    return e(n);
  }, set selected($) {
    s(n, $, true);
  }, get focusFirst() {
    return e(b);
  }, set focusFirst($) {
    s(b, $, true);
  } }), c(T);
  var P = u(T, 2);
  {
    var K = ($) => {
      ot($, { get onSave() {
        return i.onSave;
      }, get key() {
        return N();
      }, set key(S) {
        N(S);
      } });
    }, w = ($, S) => {
      {
        var f = (l) => {
          ft(l, { get key() {
            return N();
          } });
        }, t = (l, m) => {
          {
            var p = (h) => {
              _t(h, { get key() {
                return N();
              }, get onSave() {
                return i.onSave;
              } });
            };
            R(l, (h) => {
              e(n) === o.tabs.delete && h(p);
            }, m);
          }
        };
        R($, (l) => {
          e(n) === "Secret" ? l(f) : l(t, false);
        }, S);
      }
    };
    R(P, ($) => {
      e(n) === o.tabs.config ? $(K) : $(w, false);
    });
  }
  c(y), k(V, y), ee();
}
var kt = L("<div><!></div>"), yt = L('<!> <!> <!> <!> <!> <div class="err"> </div>', 1), bt = L('<div class="container svelte-1dweyfr"><!></div>');
function xt(V, i) {
  Z(i, true);
  const N = oe();
  let o = le(), C = se(), n = A(""), b = A(false), y = A(void 0), T = A(""), x = A(Y(oe())), P = A(Y(fe()));
  async function K(S, f) {
    var _a;
    s(n, "");
    for (let p of i.keys) if (p.name === e(T)) {
      s(n, "Name already exists");
      return;
    }
    let t = (_a = e(y)) == null ? void 0 : _a();
    if (!t) {
      console.error("access rights is undefined");
      return;
    }
    let l = { name: e(T), access: t };
    if (l.access.length === 0) {
      s(n, "Grant the new API Key at least one permission");
      return;
    }
    if (e(b)) {
      if (console.log(), !e(x) || !e(P)) {
        s(n, "Disable expiry or provide an valid date and time");
        return;
      }
      let p = be(e(x), e(P));
      if (!p) {
        s(n, "Invalid Date Input: User Expires");
        return;
      }
      l.exp = p;
    }
    let m = await Ke(S.action, l);
    m.error ? s(n, m.error.message, true) : i.onSave();
  }
  var w = bt(), $ = v(w);
  Xe($, { action: "/auth/v1/api_keys", onSubmit: K, children: (S, f) => {
    var t = yt(), l = W(t);
    Je(l, { get label() {
      return C.api_key.keyName;
    }, get placeholder() {
      return C.api_key.keyName;
    }, autocomplete: "off", required: true, min: "2", max: "24", pattern: Qe, get value() {
      return e(T);
    }, set value(r) {
      s(T, r, true);
    } });
    var m = u(l, 2);
    ae(m, { get ariaLabel() {
      return C.api_key.limitedValidity;
    }, get checked() {
      return e(b);
    }, set checked(r) {
      s(b, r, true);
    }, children: (r, d) => {
      B();
      var z = U();
      D(() => E(z, C.api_key.limitedValidity)), k(r, z);
    }, $$slots: { default: true } });
    var p = u(m, 2);
    {
      var h = (r) => {
        var d = kt(), z = v(d);
        xe(z, { get label() {
          return C.api_key.expires;
        }, withTime: true, min: N, required: true, get value() {
          return e(x);
        }, set value(G) {
          s(x, G, true);
        }, get timeValue() {
          return e(P);
        }, set timeValue(G) {
          s(P, G, true);
        } }), c(d), te(3, d, () => re, () => ({ duration: 150 })), k(r, d);
      };
      R(p, (r) => {
        e(b) && r(h);
      });
    }
    var g = u(p, 2);
    we(g, { get finalize() {
      return e(y);
    }, set finalize(r) {
      s(y, r, true);
    } });
    var I = u(g, 2);
    X(I, { type: "submit", children: (r, d) => {
      B();
      var z = U();
      D(() => E(z, o.common.save)), k(r, z);
    }, $$slots: { default: true } });
    var _ = u(I, 2), a = v(_, true);
    c(_), D(() => E(a, e(n))), k(S, t);
  }, $$slots: { default: true } }), c(w), k(V, w), ee();
}
var wt = L("<div></div> <!>", 1), $t = L("<!> <!>", 1), It = L('<div id="keys"><!></div>'), Pt = L(" <!> <!>", 1);
function St(V, i) {
  Z(i, true);
  let N = se(), o = A(void 0), C = A(""), n = A(Y([])), b = A(Y([])), y = A(void 0), T = Ge("kn");
  const x = ["Name"], P = ["Name"];
  let K = A(Y(P[0])), w = A("");
  ge(() => {
    $();
  }), Q(() => {
    s(y, e(n).find((h) => h.name === T.get()), true);
  }), Q(() => {
    let h = e(w).toLowerCase();
    h ? e(K) === P[0] && s(b, e(n).filter((g) => {
      var _a;
      return (_a = g.name) == null ? void 0 : _a.includes(h);
    }), true) : s(b, e(n), true);
  });
  async function $() {
    var _a;
    let h = await De("/auth/v1/api_keys");
    h.body ? s(n, h.body.keys, true) : s(C, ((_a = h.error) == null ? void 0 : _a.message) || "Error", true);
  }
  function S(h, g) {
    let I = g === "up";
    h === x[0] && e(n).sort((_, a) => I ? _.name.localeCompare(a.name) : a.name.localeCompare(_.name));
  }
  function f() {
    var _a;
    (_a = e(o)) == null ? void 0 : _a(), $();
  }
  B();
  var t = Pt(), l = W(t), m = u(l);
  Ve(m, { paddingTop: "2.1rem", buttonTilesAriaControls: "keys", width: "min(20rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (g) => {
    var I = wt(), _ = W(I);
    ke(_, "", {}, { height: ".5rem" });
    var a = u(_, 2);
    he(a, 17, () => e(b), (r) => r.name, (r, d, z, G) => {
      const O = F(() => T.get() === e(d).name);
      Ee(r, { onclick: () => T.set(e(d).name), get selected() {
        return e(O);
      }, children: (M, q) => {
        B();
        var j = U();
        D(() => E(j, e(d).name)), k(M, j);
      } });
    }), k(g, I);
  }, children: (g, I) => {
    var _ = $t(), a = W(_);
    const r = F(() => e(n).length === 0 ? 1 : 2);
    Me(a, { get level() {
      return e(r);
    }, alignRight: true, get closeModal() {
      return e(o);
    }, set closeModal(z) {
      s(o, z, true);
    }, children: (z, G) => {
      xt(z, { get keys() {
        return e(n);
      }, onSave: f });
    }, $$slots: { default: true } });
    var d = u(a, 2);
    Oe(d, { searchOptions: P, orderOptions: x, onChangeOrder: S, searchWidth: "min(19.5rem, 100dvw - .5rem)", get searchOption() {
      return e(K);
    }, set searchOption(z) {
      s(K, z, true);
    }, get value() {
      return e(w);
    }, set value(z) {
      s(w, z, true);
    } }), k(g, _);
  }, $$slots: { buttonTiles: true, default: true } });
  var p = u(m, 2);
  Ne(p, { children: (h, g) => {
    var I = It(), _ = v(I);
    {
      var a = (r) => {
        ht(r, { onSave: $, get key() {
          return e(y);
        }, set key(d) {
          s(y, d, true);
        } });
      };
      R(_, (r) => {
        e(y) && r(a);
      });
    }
    c(I), D(() => ne(I, "aria-label", N.common.details)), k(h, I);
  } }), D(() => E(l, `${e(C) ?? ""} `)), k(V, t), ee();
}
function or(V) {
  St(V, {});
}
export {
  or as component
};
