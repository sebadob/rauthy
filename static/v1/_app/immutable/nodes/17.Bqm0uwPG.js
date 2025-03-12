import { t as w, e as j, a as d } from "../chunks/DKC5GJ29.js";
import { p as W, aa as z, f as M, l as i, k as p, j as e, s as S, t as I, c as B, a as H, a8 as U, r as D, a9 as X, a6 as ae } from "../chunks/BveBAmlr.js";
import { s as O } from "../chunks/CYCba2oX.js";
import { i as q } from "../chunks/D-uYoVwt.js";
import { e as oe } from "../chunks/BLE4FKaJ.js";
import { a as se } from "../chunks/Dql74IOz.js";
import { p as n } from "../chunks/VbPNpVtZ.js";
import { u as J } from "../chunks/DtT3Jahq.js";
import { B as ne } from "../chunks/B1rQzxl_.js";
import { C as ie } from "../chunks/CqiL-eHy.js";
import { N as le } from "../chunks/pmj7Ha_a.js";
import { N as ue } from "../chunks/DLHOTAgm.js";
import { O as de } from "../chunks/CPgfHyhH.js";
import { b as me, d as ve, c as fe, f as ce } from "../chunks/BO1A6s0c.js";
import { u as ge } from "../chunks/BdFU-gNS.js";
import { B as Y } from "../chunks/DlLEcmNg.js";
import { I as ee } from "../chunks/CCQyB9gY.js";
import { f as re } from "../chunks/BRCxk8by.js";
import { F as te } from "../chunks/BguaNybM.js";
import { u as K } from "../chunks/8R5My_LO.js";
import { T as pe } from "../chunks/BGiCnx1L.js";
import { I as _e } from "../chunks/Cxysd5TC.js";
import { L as he } from "../chunks/FI26onRO.js";
var xe = w('<div class="err"> </div>'), be = w("<!> <!> <!>", 1), ye = w('<div class="container svelte-s1196z"><!></div>');
function Pe($, a) {
  W(a, true);
  let A = K(), f = J(), u = p(void 0), o = p(""), l = p("");
  z(() => {
    requestAnimationFrame(() => {
      var _a;
      (_a = e(u)) == null ? void 0 : _a.focus();
    });
  });
  async function h(x, b) {
    var _a;
    if (a.groups.find((m) => m.name === e(l))) {
      i(o, n(f.common.nameExistsAlready));
      return;
    }
    i(o, "");
    let s = { group: e(l) }, r = await me(x.action, s);
    r.body ? a.onSave(r.body.id) : i(o, n(((_a = r.error) == null ? void 0 : _a.message) || "Error"));
  }
  var c = ye(), _ = B(c);
  te(_, { action: "/auth/v1/groups", onSubmit: h, children: (x, b) => {
    var s = be(), r = M(s);
    ee(r, { autocomplete: "off", get label() {
      return f.groups.name;
    }, get placeholder() {
      return f.groups.name;
    }, required: true, pattern: re, get ref() {
      return e(u);
    }, set ref(y) {
      i(u, n(y));
    }, get value() {
      return e(l);
    }, set value(y) {
      i(l, n(y));
    } });
    var m = S(r, 2);
    Y(m, { type: "submit", children: (y, G) => {
      U();
      var E = j();
      I(() => O(E, A.common.save)), d(y, E);
    }, $$slots: { default: true } });
    var k = S(m, 2);
    {
      var F = (y) => {
        var G = xe(), E = B(G, true);
        D(G), I(() => O(E, e(o))), d(y, G);
      };
      q(k, (y) => {
        e(o) && y(F);
      });
    }
    d(x, s);
  }, $$slots: { default: true } }), D(c), d($, c), H();
}
var Se = w('<div class="err"> </div>'), we = w('<!> <!> <div class="flex gap-05"><!> <!></div> <!>', 1);
function Ce($, a) {
  W(a, true);
  let A = K(), f = J(), u = p(""), o = p(false), l = p(n(a.group.name));
  z(() => {
    a.group.id && i(l, n(a.group.name));
  });
  async function h(_, x) {
    if (i(u, ""), e(l) !== a.group.name && a.groups.find((r) => r.name === e(l))) {
      i(u, n(f.common.nameExistsAlready));
      return;
    }
    let b = { group: e(l) }, s = await ve(_.action, b);
    s.error ? i(u, n(s.error.message)) : (i(o, true), a.onSave(), setTimeout(() => {
      i(o, false);
    }, 2e3));
  }
  const c = X(() => `/auth/v1/groups/${a.group.id}`);
  te($, { get action() {
    return e(c);
  }, onSubmit: h, children: (_, x) => {
    var b = we(), s = M(b);
    he(s, { label: "ID", mono: true, children: (t, g) => {
      U();
      var v = j();
      I(() => O(v, a.group.id)), d(t, v);
    }, $$slots: { default: true } });
    var r = S(s, 2);
    ee(r, { autocomplete: "off", get label() {
      return f.groups.name;
    }, get placeholder() {
      return f.groups.name;
    }, width: "14.5rem", required: true, pattern: re, get value() {
      return e(l);
    }, set value(t) {
      i(l, n(t));
    } });
    var m = S(r, 2), k = B(m);
    Y(k, { type: "submit", children: (t, g) => {
      U();
      var v = j();
      I(() => O(v, A.common.save)), d(t, v);
    }, $$slots: { default: true } });
    var F = S(k, 2);
    {
      var y = (t) => {
        _e(t, {});
      };
      q(F, (t) => {
        e(o) && t(y);
      });
    }
    D(m);
    var G = S(m, 2);
    {
      var E = (t) => {
        var g = Se(), v = B(g, true);
        D(g), I(() => O(v, e(u))), d(t, g);
      };
      q(G, (t) => {
        e(u) && t(E);
      });
    }
    d(_, b);
  }, $$slots: { default: true } }), H();
}
var $e = w('<div class="err"> </div>'), Ae = w("<p> </p> <!> <!>", 1);
function Te($, a) {
  W(a, true);
  let A = K(), f = J(), u = p("");
  async function o() {
    i(u, "");
    let s = await fe(`/auth/v1/groups/${a.group.id}`);
    s.error ? i(u, n(s.error.message)) : a.onSave();
  }
  var l = Ae(), h = M(l), c = B(h, true);
  D(h);
  var _ = S(h, 2);
  Y(_, { level: -1, onclick: o, children: (s, r) => {
    U();
    var m = j();
    I(() => O(m, A.common.delete)), d(s, m);
  }, $$slots: { default: true } });
  var x = S(_, 2);
  {
    var b = (s) => {
      var r = $e(), m = B(r, true);
      D(r), I(() => O(m, e(u))), d(s, r);
    };
    q(x, (s) => {
      e(u) && s(b);
    });
  }
  I(() => O(c, f.groups.delete1)), d($, l), H();
}
var Ie = w('<div class="flex"><!></div> <!>', 1);
function Oe($, a) {
  W(a, true);
  let A = K(), f = J();
  const u = [f.nav.config, A.common.delete];
  let o = p(n(u[0])), l = p(void 0);
  z(() => {
    a.group.id && requestAnimationFrame(() => {
      var _a;
      (_a = e(l)) == null ? void 0 : _a();
    });
  });
  var h = Ie(), c = M(h), _ = B(c);
  pe(_, { tabs: u, get selected() {
    return e(o);
  }, set selected(r) {
    i(o, n(r));
  }, get focusFirst() {
    return e(l);
  }, set focusFirst(r) {
    i(l, n(r));
  } }), D(c);
  var x = S(c, 2);
  {
    var b = (r) => {
      Ce(r, { get group() {
        return a.group;
      }, get groups() {
        return a.groups;
      }, get onSave() {
        return a.onSave;
      } });
    }, s = (r, m) => {
      {
        var k = (F) => {
          Te(F, { get group() {
            return a.group;
          }, get onSave() {
            return a.onSave;
          } });
        };
        q(r, (F) => {
          e(o) === A.common.delete && F(k);
        }, m);
      }
    };
    q(x, (r) => {
      e(o) === f.nav.config ? r(b) : r(s, false);
    });
  }
  d($, h), H();
}
var Fe = w("<div></div> <!>", 1), Ge = w("<!> <!>", 1), Ne = w('<div class="err"> </div>'), Be = w('<!> <div id="groups"><!></div>', 1), De = w("<!> <!>", 1);
function ke($, a) {
  W(a, true);
  let A = J(), f = p(void 0), u = p(""), o = p(n([])), l = p(n([])), h = p(void 0), c = ge("gid");
  const _ = [A.common.name, "ID"];
  let x = p(n(_[0])), b = p("");
  const s = [A.common.name, "ID"];
  ae(() => {
    r();
  }), z(() => {
    i(h, n(e(o).find((t) => t.id === c.get())));
  }), z(() => {
    let t = e(b).toLowerCase();
    t ? e(x) === _[0] ? i(l, n(e(o).filter((g) => {
      var _a;
      return (_a = g.name) == null ? void 0 : _a.toLowerCase().includes(t);
    }))) : e(x) === _[1] && i(l, n(e(o).filter((g) => g.id.toLowerCase().includes(t)))) : i(l, n(e(o)));
  });
  async function r() {
    var _a;
    let t = await ce("/auth/v1/groups");
    t.body ? i(o, n(t.body)) : i(u, n(((_a = t.error) == null ? void 0 : _a.message) || "Error"));
  }
  function m(t, g) {
    let v = g === "up";
    t === s[0] ? e(o).sort((P, C) => v ? P.name.localeCompare(C.name) : C.name.localeCompare(P.name)) : t === s[1] && e(o).sort((P, C) => v ? P.id.localeCompare(C.id) : C.id.localeCompare(P.id));
  }
  function k() {
    r();
  }
  async function F(t) {
    var _a;
    (_a = e(f)) == null ? void 0 : _a(), await r(), c.set(t);
  }
  var y = De(), G = M(y);
  ue(G, { paddingTop: "2.1rem", buttonTilesAriaControls: "groups", width: "min(20rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (g) => {
    var v = Fe(), P = M(v);
    se(P, "", {}, { height: ".5rem" });
    var C = S(P, 2);
    oe(C, 17, () => e(l), (L) => L.id, (L, R, T, N) => {
      const V = X(() => c.get() === e(R).id);
      le(L, { onclick: () => c.set(e(R).id), get selected() {
        return e(V);
      }, children: (Q, Ee) => {
        U();
        var Z = j();
        I(() => O(Z, e(R).name)), d(Q, Z);
      } });
    }), d(g, v);
  }, children: (g, v) => {
    var P = Ge(), C = M(P);
    const L = X(() => e(o).length === 0 ? 1 : 2);
    ne(C, { get level() {
      return e(L);
    }, alignRight: true, get closeModal() {
      return e(f);
    }, set closeModal(T) {
      i(f, n(T));
    }, children: (T, N) => {
      Pe(T, { onSave: F, get groups() {
        return e(o);
      } });
    }, $$slots: { default: true } });
    var R = S(C, 2);
    de(R, { searchOptions: _, orderOptions: s, onChangeOrder: m, searchWidth: "min(19.5rem, 100dvw - .5rem)", get searchOption() {
      return e(x);
    }, set searchOption(T) {
      i(x, n(T));
    }, get value() {
      return e(b);
    }, set value(T) {
      i(b, n(T));
    } }), d(g, P);
  }, $$slots: { buttonTiles: true, default: true } });
  var E = S(G, 2);
  ie(E, { children: (t, g) => {
    var v = Be(), P = M(v);
    {
      var C = (N) => {
        var V = Ne(), Q = B(V, true);
        D(V), I(() => O(Q, e(u))), d(N, V);
      };
      q(P, (N) => {
        e(u) && N(C);
      });
    }
    var L = S(P, 2), R = B(L);
    {
      var T = (N) => {
        Oe(N, { get group() {
          return e(h);
        }, get groups() {
          return e(o);
        }, onSave: k });
      };
      q(R, (N) => {
        e(h) && N(T);
      });
    }
    D(L), d(t, v);
  } }), d($, y), H();
}
function ir($) {
  ke($, {});
}
export {
  ir as component
};
