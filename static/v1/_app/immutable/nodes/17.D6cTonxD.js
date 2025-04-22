import { t as S, e as j, a as l } from "../chunks/BxmJRzoY.js";
import { p as W, j as g, a5 as z, f as L, l as n, k as e, s as P, t as T, c as N, a as H, a3 as U, r as B, a1 as V, a4 as X, a0 as oe } from "../chunks/w0HvPX0p.js";
import { s as I } from "../chunks/BzP2S3Z_.js";
import { i as M } from "../chunks/iO9_dPNE.js";
import { e as ae } from "../chunks/S81raU5Y.js";
import { a as se } from "../chunks/BdbQ6g_y.js";
import { u as J } from "../chunks/D12OFlGX.js";
import { B as ne } from "../chunks/B5QJ0QFB.js";
import { C as ie } from "../chunks/C6jTHtu1.js";
import { N as ue } from "../chunks/UWnXu0Z3.js";
import { N as le } from "../chunks/Bmyn6g0M.js";
import { O as de } from "../chunks/Ssae7OHr.js";
import { b as ve, d as me, c as fe, f as ce } from "../chunks/UPFlzoow.js";
import { u as ge } from "../chunks/DvTygCXn.js";
import { B as Y } from "../chunks/C4AV2CoD.js";
import { I as ee } from "../chunks/Bk5EVqw2.js";
import { f as re } from "../chunks/gfDO7tLr.js";
import { F as te } from "../chunks/CfHEvq46.js";
import { u as K } from "../chunks/N6FgGI8m.js";
import { T as pe } from "../chunks/Dgt5DyOi.js";
import { I as _e } from "../chunks/CTshzOVc.js";
import { L as he } from "../chunks/D8MSWRQv.js";
var xe = S('<div class="err"> </div>'), be = S("<!> <!> <!>", 1), ye = S('<div class="container svelte-s1196z"><!></div>');
function Pe(C, o) {
  W(o, true);
  let $ = K(), m = J(), u = g(void 0), a = g(""), i = g("");
  z(() => {
    requestAnimationFrame(() => {
      var _a;
      (_a = e(u)) == null ? void 0 : _a.focus();
    });
  });
  async function _(h, x) {
    var _a;
    if (o.groups.find((d) => d.name === e(i))) {
      n(a, m.common.nameExistsAlready, true);
      return;
    }
    n(a, "");
    let s = { group: e(i) }, r = await ve(h.action, s);
    r.body ? o.onSave(r.body.id) : n(a, ((_a = r.error) == null ? void 0 : _a.message) || "Error", true);
  }
  var f = ye(), p = N(f);
  te(p, { action: "/auth/v1/groups", onSubmit: _, children: (h, x) => {
    var s = be(), r = L(s);
    ee(r, { autocomplete: "off", get label() {
      return m.groups.name;
    }, get placeholder() {
      return m.groups.name;
    }, required: true, pattern: re, get ref() {
      return e(u);
    }, set ref(b) {
      n(u, b, true);
    }, get value() {
      return e(i);
    }, set value(b) {
      n(i, b, true);
    } });
    var d = P(r, 2);
    Y(d, { type: "submit", children: (b, F) => {
      U();
      var k = j();
      T(() => I(k, $.common.save)), l(b, k);
    }, $$slots: { default: true } });
    var D = P(d, 2);
    {
      var O = (b) => {
        var F = xe(), k = N(F, true);
        B(F), T(() => I(k, e(a))), l(b, F);
      };
      M(D, (b) => {
        e(a) && b(O);
      });
    }
    l(h, s);
  }, $$slots: { default: true } }), B(f), l(C, f), H();
}
var Se = S('<div class="err"> </div>'), we = S('<!> <!> <div class="flex gap-05"><!> <!></div> <!>', 1);
function Ce(C, o) {
  W(o, true);
  let $ = K(), m = J(), u = g(""), a = g(false), i = g(V(o.group.name));
  z(() => {
    o.group.id && n(i, o.group.name, true);
  });
  async function _(p, h) {
    if (n(u, ""), e(i) !== o.group.name && o.groups.find((r) => r.name === e(i))) {
      n(u, m.common.nameExistsAlready, true);
      return;
    }
    let x = { group: e(i) }, s = await me(p.action, x);
    s.error ? n(u, s.error.message, true) : (n(a, true), o.onSave(), setTimeout(() => {
      n(a, false);
    }, 2e3));
  }
  const f = X(() => `/auth/v1/groups/${o.group.id}`);
  te(C, { get action() {
    return e(f);
  }, onSubmit: _, children: (p, h) => {
    var x = we(), s = L(x);
    he(s, { label: "ID", mono: true, children: (t, c) => {
      U();
      var v = j();
      T(() => I(v, o.group.id)), l(t, v);
    }, $$slots: { default: true } });
    var r = P(s, 2);
    ee(r, { autocomplete: "off", get label() {
      return m.groups.name;
    }, get placeholder() {
      return m.groups.name;
    }, width: "14.5rem", required: true, pattern: re, get value() {
      return e(i);
    }, set value(t) {
      n(i, t, true);
    } });
    var d = P(r, 2), D = N(d);
    Y(D, { type: "submit", children: (t, c) => {
      U();
      var v = j();
      T(() => I(v, $.common.save)), l(t, v);
    }, $$slots: { default: true } });
    var O = P(D, 2);
    {
      var b = (t) => {
        _e(t, {});
      };
      M(O, (t) => {
        e(a) && t(b);
      });
    }
    B(d);
    var F = P(d, 2);
    {
      var k = (t) => {
        var c = Se(), v = N(c, true);
        B(c), T(() => I(v, e(u))), l(t, c);
      };
      M(F, (t) => {
        e(u) && t(k);
      });
    }
    l(p, x);
  }, $$slots: { default: true } }), H();
}
var $e = S('<div class="err"> </div>'), Ae = S("<p> </p> <!> <!>", 1);
function Te(C, o) {
  W(o, true);
  let $ = K(), m = J(), u = g("");
  async function a() {
    n(u, "");
    let s = await fe(`/auth/v1/groups/${o.group.id}`);
    s.error ? n(u, s.error.message, true) : o.onSave();
  }
  var i = Ae(), _ = L(i), f = N(_, true);
  B(_);
  var p = P(_, 2);
  Y(p, { level: -1, onclick: a, children: (s, r) => {
    U();
    var d = j();
    T(() => I(d, $.common.delete)), l(s, d);
  }, $$slots: { default: true } });
  var h = P(p, 2);
  {
    var x = (s) => {
      var r = $e(), d = N(r, true);
      B(r), T(() => I(d, e(u))), l(s, r);
    };
    M(h, (s) => {
      e(u) && s(x);
    });
  }
  T(() => I(f, m.groups.delete1)), l(C, i), H();
}
var Ie = S('<div class="flex"><!></div> <!>', 1);
function Oe(C, o) {
  W(o, true);
  let $ = K(), m = J();
  const u = [m.nav.config, $.common.delete];
  let a = g(V(u[0])), i = g(void 0);
  z(() => {
    o.group.id && requestAnimationFrame(() => {
      var _a;
      (_a = e(i)) == null ? void 0 : _a();
    });
  });
  var _ = Ie(), f = L(_), p = N(f);
  pe(p, { tabs: u, get selected() {
    return e(a);
  }, set selected(r) {
    n(a, r, true);
  }, get focusFirst() {
    return e(i);
  }, set focusFirst(r) {
    n(i, r, true);
  } }), B(f);
  var h = P(f, 2);
  {
    var x = (r) => {
      Ce(r, { get group() {
        return o.group;
      }, get groups() {
        return o.groups;
      }, get onSave() {
        return o.onSave;
      } });
    }, s = (r, d) => {
      {
        var D = (O) => {
          Te(O, { get group() {
            return o.group;
          }, get onSave() {
            return o.onSave;
          } });
        };
        M(r, (O) => {
          e(a) === $.common.delete && O(D);
        }, d);
      }
    };
    M(h, (r) => {
      e(a) === m.nav.config ? r(x) : r(s, false);
    });
  }
  l(C, _), H();
}
var Fe = S("<div></div> <!>", 1), Ge = S("<!> <!>", 1), Ne = S('<div class="err"> </div>'), Be = S('<!> <div id="groups"><!></div>', 1), De = S("<!> <!>", 1);
function ke(C, o) {
  W(o, true);
  let $ = J(), m = g(void 0), u = g(""), a = g(V([])), i = g(V([])), _ = g(void 0), f = ge("gid");
  const p = [$.common.name, "ID"];
  let h = g(V(p[0])), x = g("");
  const s = [$.common.name, "ID"];
  oe(() => {
    r();
  }), z(() => {
    n(_, e(a).find((t) => t.id === f.get()), true);
  }), z(() => {
    let t = e(x).toLowerCase();
    t ? e(h) === p[0] ? n(i, e(a).filter((c) => {
      var _a;
      return (_a = c.name) == null ? void 0 : _a.toLowerCase().includes(t);
    }), true) : e(h) === p[1] && n(i, e(a).filter((c) => c.id.toLowerCase().includes(t)), true) : n(i, e(a), true);
  });
  async function r() {
    var _a;
    let t = await ce("/auth/v1/groups");
    t.body ? n(a, t.body, true) : n(u, ((_a = t.error) == null ? void 0 : _a.message) || "Error", true);
  }
  function d(t, c) {
    let v = c === "up";
    t === s[0] ? e(a).sort((y, w) => v ? y.name.localeCompare(w.name) : w.name.localeCompare(y.name)) : t === s[1] && e(a).sort((y, w) => v ? y.id.localeCompare(w.id) : w.id.localeCompare(y.id));
  }
  function D() {
    r();
  }
  async function O(t) {
    var _a;
    (_a = e(m)) == null ? void 0 : _a(), await r(), f.set(t);
  }
  var b = De(), F = L(b);
  le(F, { paddingTop: "2.1rem", buttonTilesAriaControls: "groups", width: "min(20rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (c) => {
    var v = Fe(), y = L(v);
    se(y, "", {}, { height: ".5rem" });
    var w = P(y, 2);
    ae(w, 17, () => e(i), (E) => E.id, (E, q, A, G) => {
      const R = X(() => f.get() === e(q).id);
      ue(E, { onclick: () => f.set(e(q).id), get selected() {
        return e(R);
      }, children: (Q, Ee) => {
        U();
        var Z = j();
        T(() => I(Z, e(q).name)), l(Q, Z);
      } });
    }), l(c, v);
  }, children: (c, v) => {
    var y = Ge(), w = L(y);
    const E = X(() => e(a).length === 0 ? 1 : 2);
    ne(w, { get level() {
      return e(E);
    }, alignRight: true, get closeModal() {
      return e(m);
    }, set closeModal(A) {
      n(m, A, true);
    }, children: (A, G) => {
      Pe(A, { onSave: O, get groups() {
        return e(a);
      } });
    }, $$slots: { default: true } });
    var q = P(w, 2);
    de(q, { searchOptions: p, orderOptions: s, onChangeOrder: d, searchWidth: "min(19.5rem, 100dvw - .5rem)", get searchOption() {
      return e(h);
    }, set searchOption(A) {
      n(h, A, true);
    }, get value() {
      return e(x);
    }, set value(A) {
      n(x, A, true);
    } }), l(c, y);
  }, $$slots: { buttonTiles: true, default: true } });
  var k = P(F, 2);
  ie(k, { children: (t, c) => {
    var v = Be(), y = L(v);
    {
      var w = (G) => {
        var R = Ne(), Q = N(R, true);
        B(R), T(() => I(Q, e(u))), l(G, R);
      };
      M(y, (G) => {
        e(u) && G(w);
      });
    }
    var E = P(y, 2), q = N(E);
    {
      var A = (G) => {
        Oe(G, { get group() {
          return e(_);
        }, get groups() {
          return e(a);
        }, onSave: D });
      };
      M(q, (G) => {
        e(_) && G(A);
      });
    }
    B(E), l(t, v);
  } }), l(C, b), H();
}
function nr(C) {
  ke(C, {});
}
export {
  nr as component
};
