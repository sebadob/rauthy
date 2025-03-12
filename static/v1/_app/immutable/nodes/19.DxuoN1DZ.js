import { t as y, e as j, a as d, d as oe } from "../chunks/DKC5GJ29.js";
import { p as H, aa as z, f as q, l as n, k as x, j as e, s as T, t as D, c as M, a as J, a8 as U, r as B, a9 as W, a6 as se } from "../chunks/BveBAmlr.js";
import { s as k } from "../chunks/CYCba2oX.js";
import { i as E } from "../chunks/D-uYoVwt.js";
import { e as ne } from "../chunks/BLE4FKaJ.js";
import { a as le } from "../chunks/Dql74IOz.js";
import { p as s } from "../chunks/VbPNpVtZ.js";
import { u as K } from "../chunks/DtT3Jahq.js";
import { B as ie } from "../chunks/B1rQzxl_.js";
import { C as de } from "../chunks/CqiL-eHy.js";
import { N as me } from "../chunks/pmj7Ha_a.js";
import { N as ve } from "../chunks/DLHOTAgm.js";
import { O as ue } from "../chunks/CPgfHyhH.js";
import { b as fe, d as ce, c as _e, f as ge } from "../chunks/BO1A6s0c.js";
import { u as he } from "../chunks/BdFU-gNS.js";
import { B as Y } from "../chunks/DlLEcmNg.js";
import { I as ee } from "../chunks/CCQyB9gY.js";
import { f as re } from "../chunks/BRCxk8by.js";
import { F as te } from "../chunks/BguaNybM.js";
import { u as Q } from "../chunks/8R5My_LO.js";
import { T as pe } from "../chunks/BGiCnx1L.js";
import { h as ae } from "../chunks/CJj8kriY.js";
import { I as be } from "../chunks/Cxysd5TC.js";
import { L as xe } from "../chunks/FI26onRO.js";
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
  var g = Se(), N = M(g);
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
    }, $$slots: { default: true } });
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
    var $ = T(P, 2);
    {
      var F = (r) => {
        var l = Ae(), u = M(l);
        ae(u, () => f.roles.adminNoMod), B(l), d(r, l);
      }, p = (r) => {
        var l = Ce(), u = M(l);
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
      E($, (r) => {
        e(c) ? r(F) : r(p, false);
      });
    }
    var S = T($, 2);
    {
      var C = (r) => {
        var l = Re(), u = M(l, true);
        B(l), D(() => k(u, e(m))), d(r, l);
      };
      E(S, (r) => {
        e(m) && r(C);
      });
    }
    d(h, i);
  }, $$slots: { default: true } }), J();
}
var Ie = y("<p><!></p>"), Oe = y('<div class="err"> </div>'), $e = y("<p> </p> <!> <!>", 1);
function Fe(I, a) {
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
      var i = Ie(), t = M(i);
      ae(t, () => f.roles.adminNoMod), B(i), d(v, i);
    }, h = (v) => {
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
          B(C), D(() => k(r, e(m))), d(S, C);
        };
        E(F, (S) => {
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
var Me = y('<div class="flex"><!></div> <!>', 1);
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
  var _ = Me(), g = q(_), N = M(g);
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
  function $() {
    t();
  }
  async function F(r) {
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
      we(R, { onSave: F, get roles() {
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
        var V = Ee(), X = M(V, true);
        B(V), D(() => k(X, e(m))), d(L, V);
      };
      E(b, (L) => {
        e(m) && L(w);
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
