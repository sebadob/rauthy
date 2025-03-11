import { t as w, e as j, a as d } from "../chunks/CWf9OOFK.js";
import { p as W, ab as z, f as M, l as i, k as p, j as e, s as S, t as I, c as B, a as H, a8 as U, r as D, aa as X, a6 as ae } from "../chunks/nlANaGLT.js";
import { s as O } from "../chunks/BmMHVVX3.js";
import { i as q } from "../chunks/DOjUa9u5.js";
import { e as oe } from "../chunks/B6azywu7.js";
import { a as se } from "../chunks/CZv_AhHu.js";
import { p as n } from "../chunks/5u5qd9TD.js";
import { u as J } from "../chunks/Bb8ybDgy.js";
import { B as ne } from "../chunks/B-Kclwyq.js";
import { C as ie } from "../chunks/BSE9YYho.js";
import { N as le } from "../chunks/B_NGI7Kx.js";
import { N as ue } from "../chunks/hmJwmp7r.js";
import { O as de } from "../chunks/Np7i_reI.js";
import { b as me, d as ve, c as fe, f as ce } from "../chunks/BO1A6s0c.js";
import { u as ge } from "../chunks/DAXezkVN.js";
import { B as Y } from "../chunks/BEQMYyDu.js";
import { I as ee } from "../chunks/6f3yten3.js";
import { f as re } from "../chunks/BRCxk8by.js";
import { F as te } from "../chunks/CmL1R98Y.js";
import { u as K } from "../chunks/DwvS5LQk.js";
import { T as pe } from "../chunks/D-oL4W7r.js";
import { I as _e } from "../chunks/DYRiKtW9.js";
import { L as he } from "../chunks/x5MgfX0R.js";
var be = w('<div class="err"> </div>'), xe = w("<!> <!> <!>", 1), ye = w('<div class="container svelte-s1196z"><!></div>');
function Pe(A, a) {
  W(a, true);
  let T = K(), f = J(), u = p(void 0), o = p(""), l = p("");
  z(() => {
    requestAnimationFrame(() => {
      var _a;
      (_a = e(u)) == null ? void 0 : _a.focus();
    });
  });
  async function h(b, x) {
    var _a;
    if (a.groups.find((m) => m.name === e(l))) {
      i(o, n(f.common.nameExistsAlready));
      return;
    }
    i(o, "");
    let s = { group: e(l) }, r = await me(b.action, s);
    r.body ? a.onSave(r.body.id) : i(o, n(((_a = r.error) == null ? void 0 : _a.message) || "Error"));
  }
  var c = ye(), _ = B(c);
  te(_, { action: "/auth/v1/groups", onSubmit: h, children: (b, x) => {
    var s = xe(), r = M(s);
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
      I(() => O(E, T.common.save)), d(y, E);
    }, $$slots: { default: true } });
    var k = S(m, 2);
    {
      var F = (y) => {
        var G = be(), E = B(G, true);
        D(G), I(() => O(E, e(o))), d(y, G);
      };
      q(k, (y) => {
        e(o) && y(F);
      });
    }
    d(b, s);
  }, $$slots: { default: true } }), D(c), d(A, c), H();
}
var Se = w('<div class="err"> </div>'), we = w('<!> <!> <div class="flex gap-05"><!> <!></div> <!>', 1);
function Ce(A, a) {
  W(a, true);
  let T = K(), f = J(), u = p(""), o = p(false), l = p(n(a.group.name));
  z(() => {
    a.group.id && i(l, n(a.group.name));
  });
  async function h(_, b) {
    if (i(u, ""), e(l) !== a.group.name && a.groups.find((r) => r.name === e(l))) {
      i(u, n(f.common.nameExistsAlready));
      return;
    }
    let x = { group: e(l) }, s = await ve(_.action, x);
    s.error ? i(u, n(s.error.message)) : (i(o, true), a.onSave(), setTimeout(() => {
      i(o, false);
    }, 2e3));
  }
  const c = X(() => `/auth/v1/groups/${a.group.id}`);
  te(A, { get action() {
    return e(c);
  }, onSubmit: h, children: (_, b) => {
    var x = we(), s = M(x);
    he(s, { label: "ID", mono: true, children: (t, g) => {
      U();
      var v = j();
      I(() => O(v, a.group.id)), d(t, v);
    } });
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
      I(() => O(v, T.common.save)), d(t, v);
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
    d(_, x);
  }, $$slots: { default: true } }), H();
}
var Ae = w('<div class="err"> </div>'), Te = w("<p> </p> <!> <!>", 1);
function $e(A, a) {
  W(a, true);
  let T = K(), f = J(), u = p("");
  async function o() {
    i(u, "");
    let s = await fe(`/auth/v1/groups/${a.group.id}`);
    s.error ? i(u, n(s.error.message)) : a.onSave();
  }
  var l = Te(), h = M(l), c = B(h, true);
  D(h);
  var _ = S(h, 2);
  Y(_, { level: -1, onclick: o, children: (s, r) => {
    U();
    var m = j();
    I(() => O(m, T.common.delete)), d(s, m);
  }, $$slots: { default: true } });
  var b = S(_, 2);
  {
    var x = (s) => {
      var r = Ae(), m = B(r, true);
      D(r), I(() => O(m, e(u))), d(s, r);
    };
    q(b, (s) => {
      e(u) && s(x);
    });
  }
  I(() => O(c, f.groups.delete1)), d(A, l), H();
}
var Ie = w('<div class="flex"><!></div> <!>', 1);
function Oe(A, a) {
  W(a, true);
  let T = K(), f = J();
  const u = [f.nav.config, T.common.delete];
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
  var b = S(c, 2);
  {
    var x = (r) => {
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
          $e(F, { get group() {
            return a.group;
          }, get onSave() {
            return a.onSave;
          } });
        };
        q(r, (F) => {
          e(o) === T.common.delete && F(k);
        }, m);
      }
    };
    q(b, (r) => {
      e(o) === f.nav.config ? r(x) : r(s, false);
    });
  }
  d(A, h), H();
}
var Fe = w("<div></div> <!>", 1), Ge = w("<!> <!>", 1), Ne = w('<div class="err"> </div>'), Be = w('<!> <div id="groups"><!></div>', 1), De = w("<!> <!>", 1);
function ke(A, a) {
  W(a, true);
  let T = J(), f = p(void 0), u = p(""), o = p(n([])), l = p(n([])), h = p(void 0), c = ge("gid");
  const _ = [T.common.name, "ID"];
  let b = p(n(_[0])), x = p("");
  const s = [T.common.name, "ID"];
  ae(() => {
    r();
  }), z(() => {
    i(h, n(e(o).find((t) => t.id === c.get())));
  }), z(() => {
    let t = e(x).toLowerCase();
    t ? e(b) === _[0] ? i(l, n(e(o).filter((g) => {
      var _a;
      return (_a = g.name) == null ? void 0 : _a.toLowerCase().includes(t);
    }))) : e(b) === _[1] && i(l, n(e(o).filter((g) => g.id.toLowerCase().includes(t)))) : i(l, n(e(o)));
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
    oe(C, 17, () => e(l), (L) => L.id, (L, R, $, N) => {
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
    }, set closeModal($) {
      i(f, n($));
    }, children: ($, N) => {
      Pe($, { onSave: F, get groups() {
        return e(o);
      } });
    }, $$slots: { default: true } });
    var R = S(C, 2);
    de(R, { searchOptions: _, orderOptions: s, onChangeOrder: m, searchWidth: "min(19.5rem, 100dvw - .5rem)", get searchOption() {
      return e(b);
    }, set searchOption($) {
      i(b, n($));
    }, get value() {
      return e(x);
    }, set value($) {
      i(x, n($));
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
      var $ = (N) => {
        Oe(N, { get group() {
          return e(h);
        }, get groups() {
          return e(o);
        }, onSave: k });
      };
      q(R, (N) => {
        e(h) && N($);
      });
    }
    D(L), d(t, v);
  } }), d(A, y), H();
}
function ir(A) {
  ke(A, {});
}
export {
  ir as component
};
