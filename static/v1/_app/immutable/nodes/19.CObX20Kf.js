import { t as y, e as j, a as d, d as oe } from "../chunks/DLBGyKVC.js";
import { p as H, aa as z, f as q, l, k as x, j as e, s as T, t as D, c as M, a as J, a8 as U, r as B, a9 as W, a6 as se } from "../chunks/CmQi0fbH.js";
import { s as k } from "../chunks/BjaYyaa_.js";
import { i as E } from "../chunks/C6bK2EJJ.js";
import { e as le } from "../chunks/YQCw2eEa.js";
import { a as ne } from "../chunks/DOlUUCkJ.js";
import { p as s } from "../chunks/B_ggA-0N.js";
import { u as K } from "../chunks/CZf6fJph.js";
import { B as ie } from "../chunks/BLvNyGE2.js";
import { C as de } from "../chunks/Bfi4YUpT.js";
import { N as ve } from "../chunks/DpqPmJAJ.js";
import { N as me } from "../chunks/ClbMy9q0.js";
import { O as ue } from "../chunks/BBZ-aIjz.js";
import { b as fe, d as ce, c as _e, f as ge } from "../chunks/BO1A6s0c.js";
import { u as he } from "../chunks/BctQMYGV.js";
import { B as Y } from "../chunks/DPLO-ozG.js";
import { I as ee } from "../chunks/DwPr_s7h.js";
import { f as re } from "../chunks/gfDO7tLr.js";
import { F as te } from "../chunks/CNMQp9ma.js";
import { u as Q } from "../chunks/DGTOa5g8.js";
import { T as pe } from "../chunks/MKf726CY.js";
import { h as ae } from "../chunks/UlQQkuaW.js";
import { I as be } from "../chunks/CAzQQhB1.js";
import { L as xe } from "../chunks/gwKtuDud.js";
var ye = y('<div class="err"> </div>'), Pe = y("<!> <!> <!>", 1), Se = y('<div class="container svelte-s1196z"><!></div>');
function we(I, a) {
  H(a, true);
  let O = Q(), f = K(), v = x(void 0), o = x(""), c = x("");
  z(() => {
    requestAnimationFrame(() => {
      var _a;
      (_a = e(v)) == null ? void 0 : _a.focus();
    });
  });
  async function _(h, m) {
    var _a;
    if (a.roles.find((P) => P.name === e(c))) {
      l(o, s(f.common.nameExistsAlready));
      return;
    }
    l(o, "");
    let i = { role: e(c) }, t = await fe(h.action, i);
    t.body ? a.onSave(t.body.id) : l(o, s(((_a = t.error) == null ? void 0 : _a.message) || "Error"));
  }
  var g = Se(), N = M(g);
  te(N, { action: "/auth/v1/roles", onSubmit: _, children: (h, m) => {
    var i = Pe(), t = q(i);
    ee(t, { autocomplete: "off", get label() {
      return f.roles.name;
    }, get placeholder() {
      return f.roles.name;
    }, required: true, pattern: re, get ref() {
      return e(v);
    }, set ref(p) {
      l(v, s(p));
    }, get value() {
      return e(c);
    }, set value(p) {
      l(c, s(p));
    } });
    var P = T(t, 2);
    Y(P, { type: "submit", children: (p, S) => {
      U();
      var C = j();
      D(() => k(C, O.common.save)), d(p, C);
    }, $$slots: { default: true } });
    var $ = T(P, 2);
    {
      var F = (p) => {
        var S = ye(), C = M(S, true);
        B(S), D(() => k(C, e(o))), d(p, S);
      };
      E($, (p) => {
        e(o) && p(F);
      });
    }
    d(h, i);
  }, $$slots: { default: true } }), B(g), d(I, g), J();
}
var Ae = y("<p><!></p>"), Ce = y('<div class="flex gap-05"><!> <!></div>'), Re = y('<div class="err"> </div>'), Te = y("<!> <!> <!> <!>", 1);
function Ne(I, a) {
  H(a, true);
  let O = Q(), f = K(), v = x(""), o = x(false), c = W(() => a.role.name === "rauthy_admin"), _ = x(s(a.role.name));
  z(() => {
    a.role.id && l(_, s(a.role.name));
  });
  async function g(h, m) {
    if (l(v, ""), e(_) !== a.role.name && a.roles.find((P) => P.name === e(_))) {
      l(v, s(f.common.nameExistsAlready));
      return;
    }
    let i = { role: e(_) }, t = await ce(h.action, i);
    t.error ? l(v, s(t.error.message)) : (l(o, true), a.onSave(), setTimeout(() => {
      l(o, false);
    }, 2e3));
  }
  const N = W(() => `/auth/v1/roles/${a.role.id}`);
  te(I, { get action() {
    return e(N);
  }, onSubmit: g, children: (h, m) => {
    var i = Te(), t = q(i);
    xe(t, { label: "ID", mono: true, children: (r, n) => {
      U();
      var u = j();
      D(() => k(u, a.role.id)), d(r, u);
    }, $$slots: { default: true } });
    var P = T(t, 2);
    ee(P, { autocomplete: "off", get label() {
      return f.roles.name;
    }, get placeholder() {
      return f.roles.name;
    }, get disabled() {
      return e(c);
    }, width: "14.5rem", required: true, pattern: re, get value() {
      return e(_);
    }, set value(r) {
      l(_, s(r));
    } });
    var $ = T(P, 2);
    {
      var F = (r) => {
        var n = Ae(), u = M(n);
        ae(u, () => f.roles.adminNoMod), B(n), d(r, n);
      }, p = (r) => {
        var n = Ce(), u = M(n);
        Y(u, { type: "submit", children: (A, G) => {
          U();
          var R = j();
          D(() => k(R, O.common.save)), d(A, R);
        }, $$slots: { default: true } });
        var b = T(u, 2);
        {
          var w = (A) => {
            be(A, {});
          };
          E(b, (A) => {
            e(o) && A(w);
          });
        }
        B(n), d(r, n);
      };
      E($, (r) => {
        e(c) ? r(F) : r(p, false);
      });
    }
    var S = T($, 2);
    {
      var C = (r) => {
        var n = Re(), u = M(n, true);
        B(n), D(() => k(u, e(v))), d(r, n);
      };
      E(S, (r) => {
        e(v) && r(C);
      });
    }
    d(h, i);
  }, $$slots: { default: true } }), J();
}
var Ie = y("<p><!></p>"), Oe = y('<div class="err"> </div>'), $e = y("<p> </p> <!> <!>", 1);
function Fe(I, a) {
  H(a, true);
  let O = Q(), f = K(), v = x(""), o = W(() => a.role.name === "rauthy_admin");
  async function c() {
    l(v, "");
    let m = await _e(`/auth/v1/roles/${a.role.id}`);
    m.error ? l(v, s(m.error.message)) : a.onSave();
  }
  var _ = oe(), g = q(_);
  {
    var N = (m) => {
      var i = Ie(), t = M(i);
      ae(t, () => f.roles.adminNoMod), B(i), d(m, i);
    }, h = (m) => {
      var i = $e(), t = q(i), P = M(t, true);
      B(t);
      var $ = T(t, 2);
      Y($, { level: -1, onclick: c, children: (S, C) => {
        U();
        var r = j();
        D(() => k(r, O.common.delete)), d(S, r);
      }, $$slots: { default: true } });
      var F = T($, 2);
      {
        var p = (S) => {
          var C = Oe(), r = M(C, true);
          B(C), D(() => k(r, e(v))), d(S, C);
        };
        E(F, (S) => {
          e(v) && S(p);
        });
      }
      D(() => k(P, f.roles.delete1)), d(m, i);
    };
    E(g, (m) => {
      e(o) ? m(N) : m(h, false);
    });
  }
  d(I, _), J();
}
var Me = y('<div class="flex"><!></div> <!>', 1);
function Be(I, a) {
  H(a, true);
  let O = Q(), f = K();
  const v = [f.nav.config, O.common.delete];
  let o = x(s(v[0])), c = x(void 0);
  z(() => {
    a.role.id && requestAnimationFrame(() => {
      var _a;
      (_a = e(c)) == null ? void 0 : _a();
    });
  });
  var _ = Me(), g = q(_), N = M(g);
  pe(N, { tabs: v, get selected() {
    return e(o);
  }, set selected(t) {
    l(o, s(t));
  }, get focusFirst() {
    return e(c);
  }, set focusFirst(t) {
    l(c, s(t));
  } }), B(g);
  var h = T(g, 2);
  {
    var m = (t) => {
      Ne(t, { get role() {
        return a.role;
      }, get roles() {
        return a.roles;
      }, get onSave() {
        return a.onSave;
      } });
    }, i = (t, P) => {
      {
        var $ = (F) => {
          Fe(F, { get role() {
            return a.role;
          }, get onSave() {
            return a.onSave;
          } });
        };
        E(t, (F) => {
          e(o) === O.common.delete && F($);
        }, P);
      }
    };
    E(h, (t) => {
      e(o) === f.nav.config ? t(m) : t(i, false);
    });
  }
  d(I, _), J();
}
var De = y("<div></div> <!>", 1), ke = y("<!> <!>", 1), Ee = y('<div class="err"> </div>'), Le = y('<!> <div id="groups"><!></div>', 1), qe = y("<!> <!>", 1);
function Ge(I, a) {
  H(a, true);
  let O = K(), f = x(void 0), v = x(""), o = x(s([])), c = x(s([])), _ = x(void 0), g = he("rid");
  const N = [O.common.name, "ID"];
  let h = x(s(N[0])), m = x("");
  const i = [O.common.name, "ID"];
  se(() => {
    t();
  }), z(() => {
    l(_, s(e(o).find((r) => r.id === g.get())));
  }), z(() => {
    let r = e(m).toLowerCase();
    r ? e(h) === N[0] ? l(c, s(e(o).filter((n) => {
      var _a;
      return (_a = n.name) == null ? void 0 : _a.toLowerCase().includes(r);
    }))) : e(h) === N[1] && l(c, s(e(o).filter((n) => n.id.toLowerCase().includes(r)))) : l(c, s(e(o)));
  });
  async function t() {
    var _a;
    let r = await ge("/auth/v1/roles");
    r.body ? l(o, s(r.body)) : l(v, s(((_a = r.error) == null ? void 0 : _a.message) || "Error"));
  }
  function P(r, n) {
    let u = n === "up";
    r === i[0] ? e(o).sort((b, w) => u ? b.name.localeCompare(w.name) : w.name.localeCompare(b.name)) : r === i[1] && e(o).sort((b, w) => u ? b.id.localeCompare(w.id) : w.id.localeCompare(b.id));
  }
  function $() {
    t();
  }
  async function F(r) {
    var _a;
    (_a = e(f)) == null ? void 0 : _a(), await t(), g.set(r);
  }
  var p = qe(), S = q(p);
  me(S, { paddingTop: "2.1rem", buttonTilesAriaControls: "groups", width: "min(20rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (n) => {
    var u = De(), b = q(u);
    ne(b, "", {}, { height: ".5rem" });
    var w = T(b, 2);
    le(w, 17, () => e(c), (A) => A.id, (A, G, R, L) => {
      const V = W(() => g.get() === e(G).id);
      ve(A, { onclick: () => g.set(e(G).id), get selected() {
        return e(V);
      }, children: (X, Ve) => {
        U();
        var Z = j();
        D(() => k(Z, e(G).name)), d(X, Z);
      } });
    }), d(n, u);
  }, children: (n, u) => {
    var b = ke(), w = q(b);
    const A = W(() => e(o).length === 0 ? 1 : 2);
    ie(w, { get level() {
      return e(A);
    }, alignRight: true, get closeModal() {
      return e(f);
    }, set closeModal(R) {
      l(f, s(R));
    }, children: (R, L) => {
      we(R, { onSave: F, get roles() {
        return e(o);
      } });
    }, $$slots: { default: true } });
    var G = T(w, 2);
    ue(G, { searchOptions: N, orderOptions: i, onChangeOrder: P, searchWidth: "min(19.5rem, 100dvw - .5rem)", get searchOption() {
      return e(h);
    }, set searchOption(R) {
      l(h, s(R));
    }, get value() {
      return e(m);
    }, set value(R) {
      l(m, s(R));
    } }), d(n, b);
  }, $$slots: { buttonTiles: true, default: true } });
  var C = T(S, 2);
  de(C, { children: (r, n) => {
    var u = Le(), b = q(u);
    {
      var w = (L) => {
        var V = Ee(), X = M(V, true);
        B(V), D(() => k(X, e(v))), d(L, V);
      };
      E(b, (L) => {
        e(v) && L(w);
      });
    }
    var A = T(b, 2), G = M(A);
    {
      var R = (L) => {
        Be(L, { get role() {
          return e(_);
        }, get roles() {
          return e(o);
        }, onSave: $ });
      };
      E(G, (L) => {
        e(_) && L(R);
      });
    }
    B(A), d(r, u);
  } }), d(I, p), J();
}
function fr(I) {
  Ge(I, {});
}
export {
  fr as component
};
