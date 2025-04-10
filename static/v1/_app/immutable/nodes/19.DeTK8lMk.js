import { t as x, e as j, a as i, d as oe } from "../chunks/BxmJRzoY.js";
import { p as H, j as b, a5 as z, f as L, l as s, k as e, s as R, t as B, c as F, a as J, a3 as U, r as M, a4 as W, a1 as V, a0 as se } from "../chunks/w0HvPX0p.js";
import { s as D } from "../chunks/BzP2S3Z_.js";
import { i as k } from "../chunks/iO9_dPNE.js";
import { e as le } from "../chunks/S81raU5Y.js";
import { a as ne } from "../chunks/BdbQ6g_y.js";
import { u as K } from "../chunks/Bt_9UXew.js";
import { B as ie } from "../chunks/CyiD59-2.js";
import { C as de } from "../chunks/C6jTHtu1.js";
import { N as ue } from "../chunks/D9abP6hj.js";
import { N as ve } from "../chunks/CzQKNE2_.js";
import { O as me } from "../chunks/HVQD_1Ej.js";
import { b as fe, d as ce, c as _e, f as ge } from "../chunks/UPFlzoow.js";
import { u as he } from "../chunks/Bz_VUaol.js";
import { B as Y } from "../chunks/C8YTstTD.js";
import { I as ee } from "../chunks/DVXwAhn3.js";
import { f as re } from "../chunks/gfDO7tLr.js";
import { F as te } from "../chunks/BEbxeS8S.js";
import { u as Q } from "../chunks/0cG6LBdy.js";
import { T as pe } from "../chunks/BdAKL3gn.js";
import { h as ae } from "../chunks/C2ZdIFW_.js";
import { I as be } from "../chunks/CTshzOVc.js";
import { L as xe } from "../chunks/DQwTjYxX.js";
var ye = x('<div class="err"> </div>'), Pe = x("<!> <!> <!>", 1), Se = x('<div class="container svelte-s1196z"><!></div>');
function we(N, a) {
  H(a, true);
  let I = Q(), m = K(), d = b(void 0), o = b(""), f = b("");
  z(() => {
    requestAnimationFrame(() => {
      var _a;
      (_a = e(d)) == null ? void 0 : _a.focus();
    });
  });
  async function c(g, u) {
    var _a;
    if (a.roles.find((y) => y.name === e(f))) {
      s(o, m.common.nameExistsAlready, true);
      return;
    }
    s(o, "");
    let n = { role: e(f) }, t = await fe(g.action, n);
    t.body ? a.onSave(t.body.id) : s(o, ((_a = t.error) == null ? void 0 : _a.message) || "Error", true);
  }
  var _ = Se(), T = F(_);
  te(T, { action: "/auth/v1/roles", onSubmit: c, children: (g, u) => {
    var n = Pe(), t = L(n);
    ee(t, { autocomplete: "off", get label() {
      return m.roles.name;
    }, get placeholder() {
      return m.roles.name;
    }, required: true, pattern: re, get ref() {
      return e(d);
    }, set ref(h) {
      s(d, h, true);
    }, get value() {
      return e(f);
    }, set value(h) {
      s(f, h, true);
    } });
    var y = R(t, 2);
    Y(y, { type: "submit", children: (h, P) => {
      U();
      var A = j();
      B(() => D(A, I.common.save)), i(h, A);
    }, $$slots: { default: true } });
    var O = R(y, 2);
    {
      var $ = (h) => {
        var P = ye(), A = F(P, true);
        M(P), B(() => D(A, e(o))), i(h, P);
      };
      k(O, (h) => {
        e(o) && h($);
      });
    }
    i(g, n);
  }, $$slots: { default: true } }), M(_), i(N, _), J();
}
var Ae = x("<p><!></p>"), Ce = x('<div class="flex gap-05"><!> <!></div>'), Re = x('<div class="err"> </div>'), Te = x("<!> <!> <!> <!>", 1);
function Ne(N, a) {
  H(a, true);
  let I = Q(), m = K(), d = b(""), o = b(false), f = W(() => a.role.name === "rauthy_admin"), c = b(V(a.role.name));
  z(() => {
    a.role.id && s(c, a.role.name, true);
  });
  async function _(g, u) {
    if (s(d, ""), e(c) !== a.role.name && a.roles.find((y) => y.name === e(c))) {
      s(d, m.common.nameExistsAlready, true);
      return;
    }
    let n = { role: e(c) }, t = await ce(g.action, n);
    t.error ? s(d, t.error.message, true) : (s(o, true), a.onSave(), setTimeout(() => {
      s(o, false);
    }, 2e3));
  }
  const T = W(() => `/auth/v1/roles/${a.role.id}`);
  te(N, { get action() {
    return e(T);
  }, onSubmit: _, children: (g, u) => {
    var n = Te(), t = L(n);
    xe(t, { label: "ID", mono: true, children: (r, l) => {
      U();
      var v = j();
      B(() => D(v, a.role.id)), i(r, v);
    }, $$slots: { default: true } });
    var y = R(t, 2);
    ee(y, { autocomplete: "off", get label() {
      return m.roles.name;
    }, get placeholder() {
      return m.roles.name;
    }, get disabled() {
      return e(f);
    }, width: "14.5rem", required: true, pattern: re, get value() {
      return e(c);
    }, set value(r) {
      s(c, r, true);
    } });
    var O = R(y, 2);
    {
      var $ = (r) => {
        var l = Ae(), v = F(l);
        ae(v, () => m.roles.adminNoMod), M(l), i(r, l);
      }, h = (r) => {
        var l = Ce(), v = F(l);
        Y(v, { type: "submit", children: (w, q) => {
          U();
          var C = j();
          B(() => D(C, I.common.save)), i(w, C);
        }, $$slots: { default: true } });
        var p = R(v, 2);
        {
          var S = (w) => {
            be(w, {});
          };
          k(p, (w) => {
            e(o) && w(S);
          });
        }
        M(l), i(r, l);
      };
      k(O, (r) => {
        e(f) ? r($) : r(h, false);
      });
    }
    var P = R(O, 2);
    {
      var A = (r) => {
        var l = Re(), v = F(l, true);
        M(l), B(() => D(v, e(d))), i(r, l);
      };
      k(P, (r) => {
        e(d) && r(A);
      });
    }
    i(g, n);
  }, $$slots: { default: true } }), J();
}
var Ie = x("<p><!></p>"), Oe = x('<div class="err"> </div>'), $e = x("<p> </p> <!> <!>", 1);
function Fe(N, a) {
  H(a, true);
  let I = Q(), m = K(), d = b(""), o = W(() => a.role.name === "rauthy_admin");
  async function f() {
    s(d, "");
    let u = await _e(`/auth/v1/roles/${a.role.id}`);
    u.error ? s(d, u.error.message, true) : a.onSave();
  }
  var c = oe(), _ = L(c);
  {
    var T = (u) => {
      var n = Ie(), t = F(n);
      ae(t, () => m.roles.adminNoMod), M(n), i(u, n);
    }, g = (u) => {
      var n = $e(), t = L(n), y = F(t, true);
      M(t);
      var O = R(t, 2);
      Y(O, { level: -1, onclick: f, children: (P, A) => {
        U();
        var r = j();
        B(() => D(r, I.common.delete)), i(P, r);
      }, $$slots: { default: true } });
      var $ = R(O, 2);
      {
        var h = (P) => {
          var A = Oe(), r = F(A, true);
          M(A), B(() => D(r, e(d))), i(P, A);
        };
        k($, (P) => {
          e(d) && P(h);
        });
      }
      B(() => D(y, m.roles.delete1)), i(u, n);
    };
    k(_, (u) => {
      e(o) ? u(T) : u(g, false);
    });
  }
  i(N, c), J();
}
var Me = x('<div class="flex"><!></div> <!>', 1);
function Be(N, a) {
  H(a, true);
  let I = Q(), m = K();
  const d = [m.nav.config, I.common.delete];
  let o = b(V(d[0])), f = b(void 0);
  z(() => {
    a.role.id && requestAnimationFrame(() => {
      var _a;
      (_a = e(f)) == null ? void 0 : _a();
    });
  });
  var c = Me(), _ = L(c), T = F(_);
  pe(T, { tabs: d, get selected() {
    return e(o);
  }, set selected(t) {
    s(o, t, true);
  }, get focusFirst() {
    return e(f);
  }, set focusFirst(t) {
    s(f, t, true);
  } }), M(_);
  var g = R(_, 2);
  {
    var u = (t) => {
      Ne(t, { get role() {
        return a.role;
      }, get roles() {
        return a.roles;
      }, get onSave() {
        return a.onSave;
      } });
    }, n = (t, y) => {
      {
        var O = ($) => {
          Fe($, { get role() {
            return a.role;
          }, get onSave() {
            return a.onSave;
          } });
        };
        k(t, ($) => {
          e(o) === I.common.delete && $(O);
        }, y);
      }
    };
    k(g, (t) => {
      e(o) === m.nav.config ? t(u) : t(n, false);
    });
  }
  i(N, c), J();
}
var De = x("<div></div> <!>", 1), ke = x("<!> <!>", 1), Ee = x('<div class="err"> </div>'), Le = x('<!> <div id="groups"><!></div>', 1), qe = x("<!> <!>", 1);
function Ge(N, a) {
  H(a, true);
  let I = K(), m = b(void 0), d = b(""), o = b(V([])), f = b(V([])), c = b(void 0), _ = he("rid");
  const T = [I.common.name, "ID"];
  let g = b(V(T[0])), u = b("");
  const n = [I.common.name, "ID"];
  se(() => {
    t();
  }), z(() => {
    s(c, e(o).find((r) => r.id === _.get()), true);
  }), z(() => {
    let r = e(u).toLowerCase();
    r ? e(g) === T[0] ? s(f, e(o).filter((l) => {
      var _a;
      return (_a = l.name) == null ? void 0 : _a.toLowerCase().includes(r);
    }), true) : e(g) === T[1] && s(f, e(o).filter((l) => l.id.toLowerCase().includes(r)), true) : s(f, e(o), true);
  });
  async function t() {
    var _a;
    let r = await ge("/auth/v1/roles");
    r.body ? s(o, r.body, true) : s(d, ((_a = r.error) == null ? void 0 : _a.message) || "Error", true);
  }
  function y(r, l) {
    let v = l === "up";
    r === n[0] ? e(o).sort((p, S) => v ? p.name.localeCompare(S.name) : S.name.localeCompare(p.name)) : r === n[1] && e(o).sort((p, S) => v ? p.id.localeCompare(S.id) : S.id.localeCompare(p.id));
  }
  function O() {
    t();
  }
  async function $(r) {
    var _a;
    (_a = e(m)) == null ? void 0 : _a(), await t(), _.set(r);
  }
  var h = qe(), P = L(h);
  ve(P, { paddingTop: "2.1rem", buttonTilesAriaControls: "groups", width: "min(20rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (l) => {
    var v = De(), p = L(v);
    ne(p, "", {}, { height: ".5rem" });
    var S = R(p, 2);
    le(S, 17, () => e(f), (w) => w.id, (w, q, C, E) => {
      const G = W(() => _.get() === e(q).id);
      ue(w, { onclick: () => _.set(e(q).id), get selected() {
        return e(G);
      }, children: (X, Ve) => {
        U();
        var Z = j();
        B(() => D(Z, e(q).name)), i(X, Z);
      } });
    }), i(l, v);
  }, children: (l, v) => {
    var p = ke(), S = L(p);
    const w = W(() => e(o).length === 0 ? 1 : 2);
    ie(S, { get level() {
      return e(w);
    }, alignRight: true, get closeModal() {
      return e(m);
    }, set closeModal(C) {
      s(m, C, true);
    }, children: (C, E) => {
      we(C, { onSave: $, get roles() {
        return e(o);
      } });
    }, $$slots: { default: true } });
    var q = R(S, 2);
    me(q, { searchOptions: T, orderOptions: n, onChangeOrder: y, searchWidth: "min(19.5rem, 100dvw - .5rem)", get searchOption() {
      return e(g);
    }, set searchOption(C) {
      s(g, C, true);
    }, get value() {
      return e(u);
    }, set value(C) {
      s(u, C, true);
    } }), i(l, p);
  }, $$slots: { buttonTiles: true, default: true } });
  var A = R(P, 2);
  de(A, { children: (r, l) => {
    var v = Le(), p = L(v);
    {
      var S = (E) => {
        var G = Ee(), X = F(G, true);
        M(G), B(() => D(X, e(d))), i(E, G);
      };
      k(p, (E) => {
        e(d) && E(S);
      });
    }
    var w = R(p, 2), q = F(w);
    {
      var C = (E) => {
        Be(E, { get role() {
          return e(c);
        }, get roles() {
          return e(o);
        }, onSave: O });
      };
      k(q, (E) => {
        e(c) && E(C);
      });
    }
    M(w), i(r, v);
  } }), i(N, h), J();
}
function mr(N) {
  Ge(N, {});
}
export {
  mr as component
};
