import { t as K, a as y, e as q, d as me } from "../chunks/BH6NCLk-.js";
import { p as X, aa as J, s as u, c as v, a8 as _e, a as Z, a7 as O, t as N, j as e, a9 as j, r as c, k as T, l as s, f as R, a5 as ke } from "../chunks/CvlvO1XB.js";
import { s as M, r as Ae } from "../chunks/CTI4QPiR.js";
import { i as F } from "../chunks/BUO_AUgz.js";
import { e as ye, i as Te } from "../chunks/BpWRzPRQ.js";
import { s as ne, d as ze } from "../chunks/BMbqVy6X.js";
import { B as Q, t as ee, s as te, a as de } from "../chunks/DMkkW5Nn.js";
import { p as m } from "../chunks/Wh68IIk2.js";
import { d as be, c as Ce, b as Le, f as Ke } from "../chunks/CRjU5SuJ.js";
import { C as De } from "../chunks/BnPoFdx3.js";
import { N as Ne } from "../chunks/D-L0o8jR.js";
import { N as Ee } from "../chunks/G5KuhWXq.js";
import { B as Ve } from "../chunks/ClvgHNP_.js";
import { u as Me } from "../chunks/BNumlRaG.js";
import { u as re } from "../chunks/D8mHI_K9.js";
import { O as Be } from "../chunks/Cokp3K-D.js";
import { p as Y, r as Ge } from "../chunks/C6SR4G2t.js";
import { B as xe } from "../chunks/Brp0G0eV.js";
import { I as Oe } from "../chunks/Nks81rMs.js";
import { L as Re } from "../chunks/CE2_6siz.js";
import { I as ae } from "../chunks/D9bDr0Ir.js";
import { I as we } from "../chunks/oGfQq50S.js";
import { u as le } from "../chunks/CUqQZdNU.js";
import { f as oe, a as pe } from "../chunks/DswDW5U8.js";
import { k as ge } from "../chunks/CaD2yKt4.js";
import { h as fe } from "../chunks/i8Xqpu09.js";
import { b as Fe } from "../chunks/dU6E9WaN.js";
import { a as Ue, b as je, c as qe, I as He } from "../chunks/B-xX0s4n.js";
import { T as We } from "../chunks/_OE2Cq0B.js";
import { I as Ye } from "../chunks/CJP-ccgO.js";
import { j as Je } from "../chunks/BRCxk8by.js";
import { F as Qe } from "../chunks/CW8abPUe.js";
var Xe = K('<div class="center svelte-opxw4z"><!></div>'), Ze = K("<div><!></div>"), et = K('<div class="row svelte-opxw4z"><!> <div class="center svelte-opxw4z"><!></div> <div class="center svelte-opxw4z"><!></div> <div class="center svelte-opxw4z"><!></div> <div class="center svelte-opxw4z"><!></div></div>'), tt = K('<div class="matrix svelte-opxw4z"><div class="row svelte-opxw4z"><div></div> <!> <!> <!> <!></div> <!></div>');
function $e(B, i) {
  X(i, true), Y(i, "finalize", 15)(L);
  let o = m([["Blacklist", false, false, false, false], ["Clients", false, false, false, false], ["Events", false, false, false, false], ["Generic", false, false, false, false], ["Groups", false, false, false, false], ["Roles", false, false, false, false], ["Secrets", false, false, false, false], ["Sessions", false, false, false, false], ["Scopes", false, false, false, false], ["UserAttributes", false, false, false, false], ["Users", false, false, false, false]]);
  J(() => {
    if (l(), i.key) for (let p of i.key.access) {
      let t;
      switch (p.group) {
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
      p.access_rights.includes("create") && (o[t][1] = true), p.access_rights.includes("read") && (o[t][2] = true), p.access_rights.includes("update") && (o[t][3] = true), p.access_rights.includes("delete") && (o[t][4] = true);
    }
  });
  function L() {
    let p = [];
    for (let t of o) {
      let g = [];
      t[1] && g.push("create"), t[2] && g.push("read"), t[3] && g.push("update"), t[4] && g.push("delete"), g.length > 0 && p.push({ group: t[0], access_rights: g });
    }
    return p;
  }
  function l() {
    for (let p of o) p[1] = false, p[2] = false, p[3] = false, p[4] = false;
  }
  function x(p) {
    for (let t of o) if (t[0] === p) {
      t[1] && t[2] && t[3] && t[4] ? (t[1] = false, t[2] = false, t[3] = false, t[4] = false) : (t[1] = true, t[2] = true, t[3] = true, t[4] = true);
      break;
    }
  }
  function b(p) {
    let t;
    switch (p) {
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
    let g = true;
    for (let n of o) if (!n[t]) {
      g = false;
      break;
    }
    for (let n of o) n[t] = !g;
  }
  var z = tt();
  {
    const p = (g, n = _e) => {
      var _ = Xe(), h = v(_);
      const k = j(() => `Toggle: ${n()}`);
      Q(h, { get ariaLabel() {
        return e(k);
      }, invisible: true, onclick: () => b(n()), children: (d, A) => {
        O();
        var a = q();
        N(() => M(a, n())), y(d, a);
      }, $$slots: { default: true } }), c(_), y(g, _);
    }, t = (g, n = _e) => {
      var _ = Ze(), h = v(_);
      const k = j(() => `Toggle: ${n()}`);
      Q(h, { get ariaLabel() {
        return e(k);
      }, invisible: true, onclick: () => x(n()), children: (d, A) => {
        O();
        var a = q();
        N(() => M(a, n())), y(d, a);
      }, $$slots: { default: true } }), c(_), y(g, _);
    };
    var w = v(z), P = u(v(w), 2);
    p(P, () => "create");
    var D = u(P, 2);
    p(D, () => "read");
    var $ = u(D, 2);
    p($, () => "update");
    var I = u($, 2);
    p(I, () => "delete"), c(w);
    var S = u(w, 2);
    ye(S, 17, () => o, Te, (g, n, _) => {
      var h = et(), k = v(h);
      t(k, () => e(n)[0]);
      var d = u(k, 2), A = v(d);
      const a = j(() => `${e(n)[0]}: create`);
      ae(A, { get ariaLabel() {
        return e(a);
      }, get checked() {
        return o[_][1];
      }, set checked(W) {
        o[_][1] = W;
      } }), c(d);
      var r = u(d, 2), f = v(r);
      const C = j(() => `${e(n)[0]}: read`);
      ae(f, { get ariaLabel() {
        return e(C);
      }, get checked() {
        return o[_][2];
      }, set checked(W) {
        o[_][2] = W;
      } }), c(r);
      var E = u(r, 2), G = v(E);
      const U = j(() => `${e(n)[0]}: update`);
      ae(G, { get ariaLabel() {
        return e(U);
      }, get checked() {
        return o[_][3];
      }, set checked(W) {
        o[_][3] = W;
      } }), c(E);
      var se = u(E, 2), H = v(se);
      const ie = j(() => `${e(n)[0]}: delete`);
      ae(H, { get ariaLabel() {
        return e(ie);
      }, get checked() {
        return o[_][4];
      }, set checked(W) {
        o[_][4] = W;
      } }), c(se), c(h), y(g, h);
    }), c(z);
  }
  y(B, z), Z();
}
var at = K("<div><!></div>"), rt = K('<div class="err"> </div>'), st = K('<!> <!> <!> <!> <div class="btn svelte-1j50zcx"><!> <!> <!></div>', 1);
function it(B, i) {
  X(i, true);
  let V = le(), o = re();
  const L = oe();
  let l = T(""), x = T(false), b = T(!!i.key.expires), z = T(m(oe())), w = T(m(pe())), P = T(void 0);
  J(() => {
    i.key.name && s(b, !!i.key.expires);
  }), J(() => {
    if (e(b)) {
      let a;
      i.key.expires ? a = new Date(i.key.expires * 1e3) : a = /* @__PURE__ */ new Date(), s(z, m(oe(a))), s(w, m(pe(a)));
    }
  });
  async function D() {
    if (s(l, ""), !e(P)) {
      console.error("finalizeMatrix is undefined");
      return;
    }
    let a = { name: i.key.name, access: e(P)() };
    if (a.access.length === 0) {
      s(l, "Grant the API Key at least one permission");
      return;
    }
    if (e(b)) {
      let f = xe(e(z), e(w));
      if (!f) {
        s(l, "Invalid Date Input: User Expires");
        return;
      }
      a.exp = f;
    }
    let r = await be(`/auth/v1/api_keys/${i.key.name}`, a);
    r.error ? s(l, m(r.error.message)) : (i.onSave(), s(x, true), setTimeout(() => {
      s(x, false), i.onSave();
    }, 2e3));
  }
  var $ = st(), I = R($);
  Re(I, { get label() {
    return o.api_key.keyName;
  }, children: (a, r) => {
    O();
    var f = q();
    N(() => M(f, i.key.name)), y(a, f);
  } });
  var S = u(I, 2);
  ae(S, { get ariaLabel() {
    return o.api_key.limitedValidity;
  }, get checked() {
    return e(b);
  }, set checked(a) {
    s(b, m(a));
  }, children: (a, r) => {
    O();
    var f = q();
    N(() => M(f, o.api_key.limitedValidity)), y(a, f);
  }, $$slots: { default: true } });
  var p = u(S, 2);
  {
    var t = (a) => {
      var r = at(), f = v(r);
      we(f, { get label() {
        return o.api_key.expires;
      }, withTime: true, min: L, required: true, get value() {
        return e(z);
      }, set value(C) {
        s(z, m(C));
      }, get timeValue() {
        return e(w);
      }, set timeValue(C) {
        s(w, m(C));
      } }), c(r), ee(3, r, () => te), y(a, r);
    };
    F(p, (a) => {
      e(b) && a(t);
    });
  }
  var g = u(p, 2);
  $e(g, { get key() {
    return i.key;
  }, get finalize() {
    return e(P);
  }, set finalize(a) {
    s(P, m(a));
  } });
  var n = u(g, 2), _ = v(n);
  Q(_, { onclick: D, children: (a, r) => {
    O();
    var f = q();
    N(() => M(f, V.common.save)), y(a, f);
  }, $$slots: { default: true } });
  var h = u(_, 2);
  {
    var k = (a) => {
      Oe(a, {});
    };
    F(h, (a) => {
      e(x) && a(k);
    });
  }
  var d = u(h, 2);
  {
    var A = (a) => {
      var r = rt(), f = v(r, true);
      c(r), N(() => M(f, e(l))), y(a, r);
    };
    F(d, (a) => {
      e(l) && a(A);
    });
  }
  c(n), y(B, $), Z();
}
var ot = K("<div><!></div>"), lt = K("<div><!></div>"), nt = K("<div><!></div>"), dt = K('<div><div class="iconsOuter svelte-1pc4fk7"><div class="iconsInner svelte-1pc4fk7"><!> <!></div></div> <textarea></textarea></div>');
function he(B, i) {
  X(i, true);
  let V = Y(i, "ariaLabel", 3, ""), o = Y(i, "value", 3, ""), L = Y(i, "rows", 3, 10), l = Y(i, "cols", 3, 60), x = Y(i, "show", 15, false), b = Y(i, "width", 3, "min(25rem, calc(100dvw - .5rem))"), z = Ge(i, ["$$slots", "$$events", "$$legacy", "ariaLabel", "value", "rows", "cols", "show", "width"]), w = le(), P = T(""), D = T("");
  J(() => {
    x() ? s(D, o()) : s(D, m(e(P)));
  }), ke(() => {
    for (let d = 0; d < o().length; d++) s(P, e(P) + "*");
    s(D, m(e(P))), x(false);
  });
  function $() {
    navigator.clipboard.writeText(o());
  }
  function I() {
    x(!x());
  }
  var S = dt(), p = v(S), t = v(p), g = v(t);
  const n = j(() => x() ? w.common.hide : w.common.show);
  Q(g, { get ariaLabel() {
    return e(n);
  }, invisible: true, onclick: I, children: (d, A) => {
    var a = me(), r = R(a);
    {
      var f = (E) => {
        var G = ot(), U = v(G);
        Ue(U, { width: 22 }), c(G), N(() => ne(G, "title", w.common.hide)), y(E, G);
      }, C = (E) => {
        var G = lt(), U = v(G);
        je(U, { width: 22 }), c(G), N(() => ne(G, "title", w.common.show)), y(E, G);
      };
      F(r, (E) => {
        x() ? E(f) : E(C, false);
      });
    }
    y(d, a);
  }, $$slots: { default: true } });
  var _ = u(g, 2);
  Q(_, { get ariaLabel() {
    return w.common.copyToClip;
  }, invisible: true, onclick: $, children: (d, A) => {
    var a = nt(), r = v(a);
    qe(r, {}), c(a), N(() => ne(a, "title", w.common.copyToClip)), y(d, a);
  }, $$slots: { default: true } }), c(t), c(p);
  var h = u(p, 2);
  Ae(h);
  let k;
  c(S), N(() => {
    de(S, "width", `${b()}`), k = ze(h, k, { "aria-label": V(), disabled: true, rows: L(), cols: l(), ...z }, "svelte-1pc4fk7"), de(h, "width", b()), de(h, "padding-right", "2.75rem");
  }), Fe(h, () => e(D), (d) => s(D, d)), y(B, S), Z();
}
var vt = K('<div class="secret font-mono svelte-1ii2aob"><!></div> <div><p><!></p> <p><span class="headerCode"><b><code></code></b></span></p></div> <div><p><!></p> <!></div> <div><p><!></p> <!></div>', 1), ct = K('<p> </p> <p> </p> <div class="btn svelte-1ii2aob"><!></div> <!> <div class="err"> </div>', 1);
function ut(B, i) {
  X(i, true);
  let V = le(), o = re(), L = T(""), l = T(""), x = T(""), b = T("");
  J(() => {
    i.key.name && s(l, "");
  });
  async function z() {
    var _a;
    let k = await be(`/auth/v1/api_keys/${i.key.name}/secret`);
    k.text ? (s(l, m(k.text)), s(x, `curl -s -H 'Authorization: API-Key ${k.text}' ${window.location.origin}/auth/v1/api_keys/${i.key.name}/test`), s(b, `${e(x)} | jq`)) : s(L, m(((_a = k.error) == null ? void 0 : _a.message) || "Error"));
  }
  var w = ct(), P = R(w), D = v(P, true);
  c(P);
  var $ = u(P, 2), I = v($, true);
  c($);
  var S = u($, 2), p = v(S);
  const t = j(() => e(l) ? 3 : 1);
  Q(p, { get level() {
    return e(t);
  }, onclick: z, children: (k, d) => {
    O();
    var A = q();
    N(() => M(A, V.passwordReset.generate)), y(k, A);
  }, $$slots: { default: true } }), c(S);
  var g = u(S, 2);
  {
    var n = (k) => {
      var d = vt(), A = R(d), a = v(A);
      He(a, { autocomplete: "off", get value() {
        return e(l);
      }, label: "API Key", placeholder: "API Key", disabled: true, showCopy: true, width: "min(25rem, calc(100dvw - .5rem))" }), c(A);
      var r = u(A, 2), f = v(r), C = v(f);
      fe(C, () => o.api_key.generate3), c(f);
      var E = u(f, 2), G = v(E), U = v(G), se = v(U);
      se.textContent = "API-Key <api_key>", c(U), c(G), c(E), c(r);
      var H = u(r, 2), ie = v(H), W = v(ie);
      fe(W, () => o.api_key.generate4), c(ie);
      var Ie = u(ie, 2);
      ge(Ie, () => e(b), (ue) => {
        he(ue, { ariaLabel: "curl text command with jq", rows: 5, get value() {
          return e(b);
        } });
      }), c(H);
      var ve = u(H, 2), ce = v(ve), Pe = v(ce);
      fe(Pe, () => o.api_key.generate5), c(ce);
      var Se = u(ce, 2);
      ge(Se, () => e(b), (ue) => {
        he(ue, { ariaLabel: "curl text command simple", rows: 5, get value() {
          return e(x);
        } });
      }), c(ve), ee(3, A, () => te, () => ({ duration: 150 })), ee(3, r, () => te, () => ({ duration: 150 })), ee(3, H, () => te, () => ({ duration: 150 })), ee(3, ve, () => te, () => ({ duration: 150 })), y(k, d);
    };
    F(g, (k) => {
      e(l) && k(n);
    });
  }
  var _ = u(g, 2), h = v(_, true);
  c(_), N(() => {
    M(D, o.api_key.generate1), M(I, o.api_key.generate2), M(h, e(L));
  }), y(B, w), Z();
}
var ft = K('<div class="err"> </div>'), mt = K("<p> </p> <!> <!>", 1);
function pt(B, i) {
  X(i, true);
  let V = le(), o = re(), L = T("");
  async function l() {
    s(L, "");
    let $ = await Ce(`/auth/v1/api_keys/${i.key.name}`);
    $.error ? s(L, m($.error.message)) : i.onSave();
  }
  var x = mt(), b = R(x), z = v(b, true);
  c(b);
  var w = u(b, 2);
  Q(w, { level: -1, onclick: l, children: ($, I) => {
    O();
    var S = q();
    N(() => M(S, V.common.delete)), y($, S);
  }, $$slots: { default: true } });
  var P = u(w, 2);
  {
    var D = ($) => {
      var I = ft(), S = v(I, true);
      c(I), N(() => M(S, e(L))), y($, I);
    };
    F(P, ($) => {
      e(L) && $(D);
    });
  }
  N(() => M(z, o.api_key.delete1)), y(B, x), Z();
}
var _t = K('<div><div class="flex"><!></div> <!></div>');
function gt(B, i) {
  X(i, true);
  let V = Y(i, "key", 15), o = re();
  const L = [o.tabs.config, "Secret", o.tabs.delete];
  let l = T(m(L[0])), x = T(void 0);
  J(() => {
    var _a;
    V().name && ((_a = e(x)) == null ? void 0 : _a());
  });
  var b = _t(), z = v(b), w = v(z);
  We(w, { tabs: L, get selected() {
    return e(l);
  }, set selected(I) {
    s(l, m(I));
  }, get focusFirst() {
    return e(x);
  }, set focusFirst(I) {
    s(x, m(I));
  } }), c(z);
  var P = u(z, 2);
  {
    var D = (I) => {
      it(I, { get onSave() {
        return i.onSave;
      }, get key() {
        return V();
      }, set key(S) {
        V(S);
      } });
    }, $ = (I) => {
      var S = me(), p = R(S);
      {
        var t = (n) => {
          ut(n, { get key() {
            return V();
          } });
        }, g = (n) => {
          var _ = me(), h = R(_);
          {
            var k = (d) => {
              pt(d, { get key() {
                return V();
              }, get onSave() {
                return i.onSave;
              } });
            };
            F(h, (d) => {
              e(l) === o.tabs.delete && d(k);
            }, true);
          }
          y(n, _);
        };
        F(p, (n) => {
          e(l) === "Secret" ? n(t) : n(g, false);
        }, true);
      }
      y(I, S);
    };
    F(P, (I) => {
      e(l) === o.tabs.config ? I(D) : I($, false);
    });
  }
  c(b), y(B, b), Z();
}
var ht = K("<div><!></div>"), kt = K('<!> <!> <!> <!> <!> <div class="err"> </div>', 1), yt = K('<div class="container svelte-1dweyfr"><!></div>');
function bt(B, i) {
  X(i, true);
  const V = oe();
  let o = le(), L = re(), l = T(""), x = T(false), b = T(void 0), z = T(""), w = T(m(oe())), P = T(m(pe()));
  async function D(S, p) {
    var _a;
    s(l, "");
    for (let _ of i.keys) if (_.name === e(z)) {
      s(l, "Name already exists");
      return;
    }
    let t = (_a = e(b)) == null ? void 0 : _a();
    if (!t) {
      console.error("access rights is undefined");
      return;
    }
    let g = { name: e(z), access: t };
    if (g.access.length === 0) {
      s(l, "Grant the new API Key at least one permission");
      return;
    }
    if (e(x)) {
      if (console.log(), !e(w) || !e(P)) {
        s(l, "Disable expiry or provide an valid date and time");
        return;
      }
      let _ = xe(e(w), e(P));
      if (!_) {
        s(l, "Invalid Date Input: User Expires");
        return;
      }
      g.exp = _;
    }
    let n = await Le(S.action, g);
    n.error ? s(l, m(n.error.message)) : i.onSave();
  }
  var $ = yt(), I = v($);
  Qe(I, { action: "/auth/v1/api_keys", onSubmit: D, children: (S, p) => {
    var t = kt(), g = R(t);
    Ye(g, { get label() {
      return L.api_key.keyName;
    }, get placeholder() {
      return L.api_key.keyName;
    }, autocomplete: "off", required: true, min: "2", max: "24", pattern: Je, get value() {
      return e(z);
    }, set value(r) {
      s(z, m(r));
    } });
    var n = u(g, 2);
    ae(n, { get ariaLabel() {
      return L.api_key.limitedValidity;
    }, get checked() {
      return e(x);
    }, set checked(r) {
      s(x, m(r));
    }, children: (r, f) => {
      O();
      var C = q();
      N(() => M(C, L.api_key.limitedValidity)), y(r, C);
    }, $$slots: { default: true } });
    var _ = u(n, 2);
    {
      var h = (r) => {
        var f = ht(), C = v(f);
        we(C, { get label() {
          return L.api_key.expires;
        }, withTime: true, min: V, required: true, get value() {
          return e(w);
        }, set value(E) {
          s(w, m(E));
        }, get timeValue() {
          return e(P);
        }, set timeValue(E) {
          s(P, m(E));
        } }), c(f), ee(3, f, () => te), y(r, f);
      };
      F(_, (r) => {
        e(x) && r(h);
      });
    }
    var k = u(_, 2);
    $e(k, { get finalize() {
      return e(b);
    }, set finalize(r) {
      s(b, m(r));
    } });
    var d = u(k, 2);
    Q(d, { type: "submit", children: (r, f) => {
      O();
      var C = q();
      N(() => M(C, o.common.save)), y(r, C);
    }, $$slots: { default: true } });
    var A = u(d, 2), a = v(A, true);
    c(A), N(() => M(a, e(l))), y(S, t);
  }, $$slots: { default: true } }), c($), y(B, $), Z();
}
var xt = K("<div></div> <!>", 1), wt = K("<!> <!>", 1), $t = K('<div id="keys"><!></div>'), It = K(" <!> <!>", 1);
function Pt(B, i) {
  X(i, true);
  let V = re(), o = T(void 0), L = T(""), l = T(m([])), x = T(m([])), b = T(void 0), z = Me("kn");
  const w = ["Name"], P = ["Name"];
  let D = T(m(P[0])), $ = T("");
  ke(() => {
    I();
  }), J(() => {
    s(b, m(e(l).find((h) => h.name === z.get())));
  }), J(() => {
    let h = e($).toLowerCase();
    h ? e(D) === P[0] && s(x, m(e(l).filter((k) => {
      var _a;
      return (_a = k.name) == null ? void 0 : _a.includes(h);
    }))) : s(x, m(e(l)));
  });
  async function I() {
    var _a;
    let h = await Ke("/auth/v1/api_keys");
    h.body ? s(l, m(h.body.keys)) : s(L, m(((_a = h.error) == null ? void 0 : _a.message) || "Error"));
  }
  function S(h, k) {
    let d = k === "up";
    h === w[0] && e(l).sort((A, a) => d ? A.name.localeCompare(a.name) : a.name.localeCompare(A.name));
  }
  function p() {
    var _a;
    (_a = e(o)) == null ? void 0 : _a(), I();
  }
  O();
  var t = It(), g = R(t), n = u(g);
  Ee(n, { paddingTop: "2.1rem", buttonTilesAriaControls: "keys", width: "min(20rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (k) => {
    var d = xt(), A = R(d);
    de(A, "height", ".5rem");
    var a = u(A, 2);
    ye(a, 17, () => e(x), (r) => r.name, (r, f, C, E) => {
      const G = j(() => z.get() === e(f).name);
      Ne(r, { onclick: () => z.set(e(f).name), get selected() {
        return e(G);
      }, children: (U, se) => {
        O();
        var H = q();
        N(() => M(H, e(f).name)), y(U, H);
      } });
    }), y(k, d);
  }, children: (k, d) => {
    var A = wt(), a = R(A);
    const r = j(() => e(l).length === 0 ? 1 : 2);
    Ve(a, { get level() {
      return e(r);
    }, alignRight: true, get closeModal() {
      return e(o);
    }, set closeModal(C) {
      s(o, m(C));
    }, children: (C, E) => {
      bt(C, { get keys() {
        return e(l);
      }, onSave: p });
    }, $$slots: { default: true } });
    var f = u(a, 2);
    Be(f, { searchOptions: P, orderOptions: w, onChangeOrder: S, searchWidth: "min(19.5rem, 100dvw - .5rem)", get searchOption() {
      return e(D);
    }, set searchOption(C) {
      s(D, m(C));
    }, get value() {
      return e($);
    }, set value(C) {
      s($, m(C));
    } }), y(k, A);
  }, $$slots: { buttonTiles: true, default: true } });
  var _ = u(n, 2);
  De(_, { children: (h, k) => {
    var d = $t(), A = v(d);
    {
      var a = (r) => {
        gt(r, { onSave: I, get key() {
          return e(b);
        }, set key(f) {
          s(b, m(f));
        } });
      };
      F(A, (r) => {
        e(b) && r(a);
      });
    }
    c(d), N(() => ne(d, "aria-label", V.common.details)), y(h, d);
  } }), N(() => M(g, `${e(L) ?? ""} `)), y(B, t), Z();
}
function ia(B) {
  Pt(B, {});
}
export {
  ia as component
};
