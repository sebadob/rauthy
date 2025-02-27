import { t as w, e as j, a as d, d as ae } from "../chunks/BH6NCLk-.js";
import { p as W, aa as z, f as N, l as i, k as _, j as e, s as S, t as I, c as B, a as H, a7 as U, r as D, a9 as X, a5 as oe } from "../chunks/CvlvO1XB.js";
import { s as O } from "../chunks/CTI4QPiR.js";
import { i as M } from "../chunks/BUO_AUgz.js";
import { e as se } from "../chunks/BpWRzPRQ.js";
import { B as Y, a as ne } from "../chunks/DMkkW5Nn.js";
import { p as n } from "../chunks/Wh68IIk2.js";
import { u as J } from "../chunks/D8mHI_K9.js";
import { B as ie } from "../chunks/BIsFWZ5z.js";
import { C as le } from "../chunks/BnPoFdx3.js";
import { N as ue } from "../chunks/D-L0o8jR.js";
import { N as de } from "../chunks/D3G2nKoD.js";
import { O as me } from "../chunks/DxrdFxfV.js";
import { b as ve, d as fe, c as ce, f as ge } from "../chunks/bbkAiDd0.js";
import { u as pe } from "../chunks/D45liQ4S.js";
import { I as ee } from "../chunks/DmeAqnkr.js";
import { f as re } from "../chunks/BRCxk8by.js";
import { F as te } from "../chunks/CqS1e6KT.js";
import { u as K } from "../chunks/DOl8_ubJ.js";
import { T as _e } from "../chunks/_OE2Cq0B.js";
import { I as he } from "../chunks/Nks81rMs.js";
import { L as xe } from "../chunks/CE2_6siz.js";
var be = w('<div class="err"> </div>'), ye = w("<!> <!> <!>", 1), Pe = w('<div class="container svelte-s1196z"><!></div>');
function Se(A, a) {
  W(a, true);
  let T = K(), c = J(), u = _(void 0), o = _(""), l = _("");
  z(() => {
    requestAnimationFrame(() => {
      var _a;
      (_a = e(u)) == null ? void 0 : _a.focus();
    });
  });
  async function x(b, y) {
    var _a;
    if (a.groups.find((m) => m.name === e(l))) {
      i(o, n(c.common.nameExistsAlready));
      return;
    }
    i(o, "");
    let s = { group: e(l) }, r = await ve(b.action, s);
    r.body ? a.onSave(r.body.id) : i(o, n(((_a = r.error) == null ? void 0 : _a.message) || "Error"));
  }
  var g = Pe(), h = B(g);
  te(h, { action: "/auth/v1/groups", onSubmit: x, children: (b, y) => {
    var s = ye(), r = N(s);
    ee(r, { autocomplete: "off", get label() {
      return c.groups.name;
    }, get placeholder() {
      return c.groups.name;
    }, required: true, pattern: re, get ref() {
      return e(u);
    }, set ref(v) {
      i(u, n(v));
    }, get value() {
      return e(l);
    }, set value(v) {
      i(l, n(v));
    } });
    var m = S(r, 2);
    Y(m, { type: "submit", children: (v, F) => {
      U();
      var E = j();
      I(() => O(E, T.common.save)), d(v, E);
    }, $$slots: { default: true } });
    var k = S(m, 2);
    {
      var q = (v) => {
        var F = be(), E = B(F, true);
        D(F), I(() => O(E, e(o))), d(v, F);
      };
      M(k, (v) => {
        e(o) && v(q);
      });
    }
    d(b, s);
  }, $$slots: { default: true } }), D(g), d(A, g), H();
}
var we = w('<div class="err"> </div>'), Ce = w('<!> <!> <div class="flex gap-05"><!> <!></div> <!>', 1);
function Ae(A, a) {
  W(a, true);
  let T = K(), c = J(), u = _(""), o = _(false), l = _(n(a.group.name));
  z(() => {
    a.group.id && i(l, n(a.group.name));
  });
  async function x(h, b) {
    if (i(u, ""), e(l) !== a.group.name && a.groups.find((r) => r.name === e(l))) {
      i(u, n(c.common.nameExistsAlready));
      return;
    }
    let y = { group: e(l) }, s = await fe(h.action, y);
    s.error ? i(u, n(s.error.message)) : (i(o, true), a.onSave(), setTimeout(() => {
      i(o, false);
    }, 2e3));
  }
  const g = X(() => `/auth/v1/groups/${a.group.id}`);
  te(A, { get action() {
    return e(g);
  }, onSubmit: x, children: (h, b) => {
    var y = Ce(), s = N(y);
    xe(s, { label: "ID", mono: true, children: (t, p) => {
      U();
      var f = j();
      I(() => O(f, a.group.id)), d(t, f);
    } });
    var r = S(s, 2);
    ee(r, { autocomplete: "off", get label() {
      return c.groups.name;
    }, get placeholder() {
      return c.groups.name;
    }, width: "14.5rem", required: true, pattern: re, get value() {
      return e(l);
    }, set value(t) {
      i(l, n(t));
    } });
    var m = S(r, 2), k = B(m);
    Y(k, { type: "submit", children: (t, p) => {
      U();
      var f = j();
      I(() => O(f, T.common.save)), d(t, f);
    }, $$slots: { default: true } });
    var q = S(k, 2);
    {
      var v = (t) => {
        he(t, {});
      };
      M(q, (t) => {
        e(o) && t(v);
      });
    }
    D(m);
    var F = S(m, 2);
    {
      var E = (t) => {
        var p = we(), f = B(p, true);
        D(p), I(() => O(f, e(u))), d(t, p);
      };
      M(F, (t) => {
        e(u) && t(E);
      });
    }
    d(h, y);
  }, $$slots: { default: true } }), H();
}
var Te = w('<div class="err"> </div>'), $e = w("<p> </p> <!> <!>", 1);
function Ie(A, a) {
  W(a, true);
  let T = K(), c = J(), u = _("");
  async function o() {
    i(u, "");
    let s = await ce(`/auth/v1/groups/${a.group.id}`);
    s.error ? i(u, n(s.error.message)) : a.onSave();
  }
  var l = $e(), x = N(l), g = B(x, true);
  D(x);
  var h = S(x, 2);
  Y(h, { level: -1, onclick: o, children: (s, r) => {
    U();
    var m = j();
    I(() => O(m, T.common.delete)), d(s, m);
  }, $$slots: { default: true } });
  var b = S(h, 2);
  {
    var y = (s) => {
      var r = Te(), m = B(r, true);
      D(r), I(() => O(m, e(u))), d(s, r);
    };
    M(b, (s) => {
      e(u) && s(y);
    });
  }
  I(() => O(g, c.groups.delete1)), d(A, l), H();
}
var Oe = w('<div class="flex"><!></div> <!>', 1);
function Fe(A, a) {
  W(a, true);
  let T = K(), c = J();
  const u = [c.nav.config, T.common.delete];
  let o = _(n(u[0])), l = _(void 0);
  z(() => {
    a.group.id && requestAnimationFrame(() => {
      var _a;
      (_a = e(l)) == null ? void 0 : _a();
    });
  });
  var x = Oe(), g = N(x), h = B(g);
  _e(h, { tabs: u, get selected() {
    return e(o);
  }, set selected(r) {
    i(o, n(r));
  }, get focusFirst() {
    return e(l);
  }, set focusFirst(r) {
    i(l, n(r));
  } }), D(g);
  var b = S(g, 2);
  {
    var y = (r) => {
      Ae(r, { get group() {
        return a.group;
      }, get groups() {
        return a.groups;
      }, get onSave() {
        return a.onSave;
      } });
    }, s = (r) => {
      var m = ae(), k = N(m);
      {
        var q = (v) => {
          Ie(v, { get group() {
            return a.group;
          }, get onSave() {
            return a.onSave;
          } });
        };
        M(k, (v) => {
          e(o) === T.common.delete && v(q);
        }, true);
      }
      d(r, m);
    };
    M(b, (r) => {
      e(o) === c.nav.config ? r(y) : r(s, false);
    });
  }
  d(A, x), H();
}
var Ge = w("<div></div> <!>", 1), Ne = w("<!> <!>", 1), Be = w('<div class="err"> </div>'), De = w('<!> <div id="groups"><!></div>', 1), ke = w("<!> <!>", 1);
function Ee(A, a) {
  W(a, true);
  let T = J(), c = _(void 0), u = _(""), o = _(n([])), l = _(n([])), x = _(void 0), g = pe("gid");
  const h = [T.common.name, "ID"];
  let b = _(n(h[0])), y = _("");
  const s = [T.common.name, "ID"];
  oe(() => {
    r();
  }), z(() => {
    i(x, n(e(o).find((t) => t.id === g.get())));
  }), z(() => {
    let t = e(y).toLowerCase();
    t ? e(b) === h[0] ? i(l, n(e(o).filter((p) => {
      var _a;
      return (_a = p.name) == null ? void 0 : _a.toLowerCase().includes(t);
    }))) : e(b) === h[1] && i(l, n(e(o).filter((p) => p.id.toLowerCase().includes(t)))) : i(l, n(e(o)));
  });
  async function r() {
    var _a;
    let t = await ge("/auth/v1/groups");
    t.body ? i(o, n(t.body)) : i(u, n(((_a = t.error) == null ? void 0 : _a.message) || "Error"));
  }
  function m(t, p) {
    let f = p === "up";
    t === s[0] ? e(o).sort((P, C) => f ? P.name.localeCompare(C.name) : C.name.localeCompare(P.name)) : t === s[1] && e(o).sort((P, C) => f ? P.id.localeCompare(C.id) : C.id.localeCompare(P.id));
  }
  function k() {
    r();
  }
  async function q(t) {
    var _a;
    (_a = e(c)) == null ? void 0 : _a(), await r(), g.set(t);
  }
  var v = ke(), F = N(v);
  de(F, { paddingTop: "2.1rem", buttonTilesAriaControls: "groups", width: "min(20rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (p) => {
    var f = Ge(), P = N(f);
    ne(P, "height", ".5rem");
    var C = S(P, 2);
    se(C, 17, () => e(l), (L) => L.id, (L, R, $, G) => {
      const V = X(() => g.get() === e(R).id);
      ue(L, { onclick: () => g.set(e(R).id), get selected() {
        return e(V);
      }, children: (Q, Le) => {
        U();
        var Z = j();
        I(() => O(Z, e(R).name)), d(Q, Z);
      } });
    }), d(p, f);
  }, children: (p, f) => {
    var P = Ne(), C = N(P);
    const L = X(() => e(o).length === 0 ? 1 : 2);
    ie(C, { get level() {
      return e(L);
    }, alignRight: true, get closeModal() {
      return e(c);
    }, set closeModal($) {
      i(c, n($));
    }, children: ($, G) => {
      Se($, { onSave: q, get groups() {
        return e(o);
      } });
    }, $$slots: { default: true } });
    var R = S(C, 2);
    me(R, { searchOptions: h, orderOptions: s, onChangeOrder: m, searchWidth: "min(19.5rem, 100dvw - .5rem)", get searchOption() {
      return e(b);
    }, set searchOption($) {
      i(b, n($));
    }, get value() {
      return e(y);
    }, set value($) {
      i(y, n($));
    } }), d(p, P);
  }, $$slots: { buttonTiles: true, default: true } });
  var E = S(F, 2);
  le(E, { children: (t, p) => {
    var f = De(), P = N(f);
    {
      var C = (G) => {
        var V = Be(), Q = B(V, true);
        D(V), I(() => O(Q, e(u))), d(G, V);
      };
      M(P, (G) => {
        e(u) && G(C);
      });
    }
    var L = S(P, 2), R = B(L);
    {
      var $ = (G) => {
        Fe(G, { get group() {
          return e(x);
        }, get groups() {
          return e(o);
        }, onSave: k });
      };
      M(R, (G) => {
        e(x) && G($);
      });
    }
    D(L), d(t, f);
  } }), d(A, v), H();
}
function ir(A) {
  Ee(A, {});
}
export {
  ir as component
};
