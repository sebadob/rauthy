import { t as y, e as j, a as d, d as oe } from "../chunks/CWf9OOFK.js";
import { p as H, ab as z, f as q, l as n, k as x, j as e, s as T, t as D, c as $, a as J, a8 as U, r as B, aa as W, a6 as se } from "../chunks/nlANaGLT.js";
import { s as k } from "../chunks/BmMHVVX3.js";
import { i as E } from "../chunks/DOjUa9u5.js";
import { e as ne } from "../chunks/B6azywu7.js";
import { a as le } from "../chunks/CZv_AhHu.js";
import { p as s } from "../chunks/5u5qd9TD.js";
import { u as K } from "../chunks/Bb8ybDgy.js";
import { B as ie } from "../chunks/B-Kclwyq.js";
import { C as de } from "../chunks/BSE9YYho.js";
import { N as me } from "../chunks/B_NGI7Kx.js";
import { N as ve } from "../chunks/hmJwmp7r.js";
import { O as ue } from "../chunks/Np7i_reI.js";
import { b as fe, d as ce, c as _e, f as ge } from "../chunks/BO1A6s0c.js";
import { u as he } from "../chunks/BTUJVHfP.js";
import { B as Y } from "../chunks/BEQMYyDu.js";
import { I as ee } from "../chunks/6f3yten3.js";
import { f as re } from "../chunks/BRCxk8by.js";
import { F as te } from "../chunks/CmL1R98Y.js";
import { u as Q } from "../chunks/DwvS5LQk.js";
import { T as pe } from "../chunks/D-oL4W7r.js";
import { h as ae } from "../chunks/CSmZOR6t.js";
import { I as be } from "../chunks/DYRiKtW9.js";
import { L as xe } from "../chunks/x5MgfX0R.js";
var ye = y('<div class="err"> </div>'), Pe = y("<!> <!> <!>", 1), Se = y('<div class="container svelte-s1196z"><!></div>');
function we(I, a) {
  H(a, true);
  let O = Q(), f = K(), m = x(void 0), o = x(""), c = x("");
  z(() => {
    requestAnimationFrame(() => {
      var _a;
      (_a = e(m)) == null ? void 0 : _a.focus();
    });
  });
  async function _(h, v) {
    var _a;
    if (a.roles.find((P) => P.name === e(c))) {
      n(o, s(f.common.nameExistsAlready));
      return;
    }
    n(o, "");
    let i = { role: e(c) }, t = await fe(h.action, i);
    t.body ? a.onSave(t.body.id) : n(o, s(((_a = t.error) == null ? void 0 : _a.message) || "Error"));
  }
  var g = Se(), N = $(g);
  te(N, { action: "/auth/v1/roles", onSubmit: _, children: (h, v) => {
    var i = Pe(), t = q(i);
    ee(t, { autocomplete: "off", get label() {
      return f.roles.name;
    }, get placeholder() {
      return f.roles.name;
    }, required: true, pattern: re, get ref() {
      return e(m);
    }, set ref(p) {
      n(m, s(p));
    }, get value() {
      return e(c);
    }, set value(p) {
      n(c, s(p));
    } });
    var P = T(t, 2);
    Y(P, { type: "submit", children: (p, S) => {
      U();
      var C = j();
      D(() => k(C, O.common.save)), d(p, C);
    }, $$slots: { default: true } });
    var F = T(P, 2);
    {
      var M = (p) => {
        var S = ye(), C = $(S, true);
        B(S), D(() => k(C, e(o))), d(p, S);
      };
      E(F, (p) => {
        e(o) && p(M);
      });
    }
    d(h, i);
  }, $$slots: { default: true } }), B(g), d(I, g), J();
}
var Ae = y("<p><!></p>"), Ce = y('<div class="flex gap-05"><!> <!></div>'), Re = y('<div class="err"> </div>'), Te = y("<!> <!> <!> <!>", 1);
function Ne(I, a) {
  H(a, true);
  let O = Q(), f = K(), m = x(""), o = x(false), c = W(() => a.role.name === "rauthy_admin"), _ = x(s(a.role.name));
  z(() => {
    a.role.id && n(_, s(a.role.name));
  });
  async function g(h, v) {
    if (n(m, ""), e(_) !== a.role.name && a.roles.find((P) => P.name === e(_))) {
      n(m, s(f.common.nameExistsAlready));
      return;
    }
    let i = { role: e(_) }, t = await ce(h.action, i);
    t.error ? n(m, s(t.error.message)) : (n(o, true), a.onSave(), setTimeout(() => {
      n(o, false);
    }, 2e3));
  }
  const N = W(() => `/auth/v1/roles/${a.role.id}`);
  te(I, { get action() {
    return e(N);
  }, onSubmit: g, children: (h, v) => {
    var i = Te(), t = q(i);
    xe(t, { label: "ID", mono: true, children: (r, l) => {
      U();
      var u = j();
      D(() => k(u, a.role.id)), d(r, u);
    } });
    var P = T(t, 2);
    ee(P, { autocomplete: "off", get label() {
      return f.scopes.name;
    }, get placeholder() {
      return f.scopes.name;
    }, get disabled() {
      return e(c);
    }, width: "14.5rem", required: true, pattern: re, get value() {
      return e(_);
    }, set value(r) {
      n(_, s(r));
    } });
    var F = T(P, 2);
    {
      var M = (r) => {
        var l = Ae(), u = $(l);
        ae(u, () => f.roles.adminNoMod), B(l), d(r, l);
      }, p = (r) => {
        var l = Ce(), u = $(l);
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
        B(l), d(r, l);
      };
      E(F, (r) => {
        e(c) ? r(M) : r(p, false);
      });
    }
    var S = T(F, 2);
    {
      var C = (r) => {
        var l = Re(), u = $(l, true);
        B(l), D(() => k(u, e(m))), d(r, l);
      };
      E(S, (r) => {
        e(m) && r(C);
      });
    }
    d(h, i);
  }, $$slots: { default: true } }), J();
}
var Ie = y("<p><!></p>"), Oe = y('<div class="err"> </div>'), Fe = y("<p> </p> <!> <!>", 1);
function Me(I, a) {
  H(a, true);
  let O = Q(), f = K(), m = x(""), o = W(() => a.role.name === "rauthy_admin");
  async function c() {
    n(m, "");
    let v = await _e(`/auth/v1/roles/${a.role.id}`);
    v.error ? n(m, s(v.error.message)) : a.onSave();
  }
  var _ = oe(), g = q(_);
  {
    var N = (v) => {
      var i = Ie(), t = $(i);
      ae(t, () => f.roles.adminNoMod), B(i), d(v, i);
    }, h = (v) => {
      var i = Fe(), t = q(i), P = $(t, true);
      B(t);
      var F = T(t, 2);
      Y(F, { level: -1, onclick: c, children: (S, C) => {
        U();
        var r = j();
        D(() => k(r, O.common.delete)), d(S, r);
      }, $$slots: { default: true } });
      var M = T(F, 2);
      {
        var p = (S) => {
          var C = Oe(), r = $(C, true);
          B(C), D(() => k(r, e(m))), d(S, C);
        };
        E(M, (S) => {
          e(m) && S(p);
        });
      }
      D(() => k(P, f.roles.delete1)), d(v, i);
    };
    E(g, (v) => {
      e(o) ? v(N) : v(h, false);
    });
  }
  d(I, _), J();
}
var $e = y('<div class="flex"><!></div> <!>', 1);
function Be(I, a) {
  H(a, true);
  let O = Q(), f = K();
  const m = [f.nav.config, O.common.delete];
  let o = x(s(m[0])), c = x(void 0);
  z(() => {
    a.role.id && requestAnimationFrame(() => {
      var _a;
      (_a = e(c)) == null ? void 0 : _a();
    });
  });
  var _ = $e(), g = q(_), N = $(g);
  pe(N, { tabs: m, get selected() {
    return e(o);
  }, set selected(t) {
    n(o, s(t));
  }, get focusFirst() {
    return e(c);
  }, set focusFirst(t) {
    n(c, s(t));
  } }), B(g);
  var h = T(g, 2);
  {
    var v = (t) => {
      Ne(t, { get role() {
        return a.role;
      }, get roles() {
        return a.roles;
      }, get onSave() {
        return a.onSave;
      } });
    }, i = (t, P) => {
      {
        var F = (M) => {
          Me(M, { get role() {
            return a.role;
          }, get onSave() {
            return a.onSave;
          } });
        };
        E(t, (M) => {
          e(o) === O.common.delete && M(F);
        }, P);
      }
    };
    E(h, (t) => {
      e(o) === f.nav.config ? t(v) : t(i, false);
    });
  }
  d(I, _), J();
}
var De = y("<div></div> <!>", 1), ke = y("<!> <!>", 1), Ee = y('<div class="err"> </div>'), Le = y('<!> <div id="groups"><!></div>', 1), qe = y("<!> <!>", 1);
function Ge(I, a) {
  H(a, true);
  let O = K(), f = x(void 0), m = x(""), o = x(s([])), c = x(s([])), _ = x(void 0), g = he("rid");
  const N = [O.common.name, "ID"];
  let h = x(s(N[0])), v = x("");
  const i = [O.common.name, "ID"];
  se(() => {
    t();
  }), z(() => {
    n(_, s(e(o).find((r) => r.id === g.get())));
  }), z(() => {
    let r = e(v).toLowerCase();
    r ? e(h) === N[0] ? n(c, s(e(o).filter((l) => {
      var _a;
      return (_a = l.name) == null ? void 0 : _a.toLowerCase().includes(r);
    }))) : e(h) === N[1] && n(c, s(e(o).filter((l) => l.id.toLowerCase().includes(r)))) : n(c, s(e(o)));
  });
  async function t() {
    var _a;
    let r = await ge("/auth/v1/roles");
    r.body ? n(o, s(r.body)) : n(m, s(((_a = r.error) == null ? void 0 : _a.message) || "Error"));
  }
  function P(r, l) {
    let u = l === "up";
    r === i[0] ? e(o).sort((b, w) => u ? b.name.localeCompare(w.name) : w.name.localeCompare(b.name)) : r === i[1] && e(o).sort((b, w) => u ? b.id.localeCompare(w.id) : w.id.localeCompare(b.id));
  }
  function F() {
    t();
  }
  async function M(r) {
    var _a;
    (_a = e(f)) == null ? void 0 : _a(), await t(), g.set(r);
  }
  var p = qe(), S = q(p);
  ve(S, { paddingTop: "2.1rem", buttonTilesAriaControls: "groups", width: "min(20rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (l) => {
    var u = De(), b = q(u);
    le(b, "", {}, { height: ".5rem" });
    var w = T(b, 2);
    ne(w, 17, () => e(c), (A) => A.id, (A, G, R, L) => {
      const V = W(() => g.get() === e(G).id);
      me(A, { onclick: () => g.set(e(G).id), get selected() {
        return e(V);
      }, children: (X, Ve) => {
        U();
        var Z = j();
        D(() => k(Z, e(G).name)), d(X, Z);
      } });
    }), d(l, u);
  }, children: (l, u) => {
    var b = ke(), w = q(b);
    const A = W(() => e(o).length === 0 ? 1 : 2);
    ie(w, { get level() {
      return e(A);
    }, alignRight: true, get closeModal() {
      return e(f);
    }, set closeModal(R) {
      n(f, s(R));
    }, children: (R, L) => {
      we(R, { onSave: M, get roles() {
        return e(o);
      } });
    }, $$slots: { default: true } });
    var G = T(w, 2);
    ue(G, { searchOptions: N, orderOptions: i, onChangeOrder: P, searchWidth: "min(19.5rem, 100dvw - .5rem)", get searchOption() {
      return e(h);
    }, set searchOption(R) {
      n(h, s(R));
    }, get value() {
      return e(v);
    }, set value(R) {
      n(v, s(R));
    } }), d(l, b);
  }, $$slots: { buttonTiles: true, default: true } });
  var C = T(S, 2);
  de(C, { children: (r, l) => {
    var u = Le(), b = q(u);
    {
      var w = (L) => {
        var V = Ee(), X = $(V, true);
        B(V), D(() => k(X, e(m))), d(L, V);
      };
      E(b, (L) => {
        e(m) && L(w);
      });
    }
    var A = T(b, 2), G = $(A);
    {
      var R = (L) => {
        Be(L, { get role() {
          return e(_);
        }, get roles() {
          return e(o);
        }, onSave: F });
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
