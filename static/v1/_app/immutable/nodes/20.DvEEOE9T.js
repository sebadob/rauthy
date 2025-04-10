import { t as b, e as W, a as i, d as fe } from "../chunks/BxmJRzoY.js";
import { p as Q, j as x, a4 as te, a1 as H, a5 as J, l as a, f as j, a3 as z, t as B, s as w, k as e, c as E, r as L, a as X, a0 as pe } from "../chunks/w0HvPX0p.js";
import { s as N } from "../chunks/BzP2S3Z_.js";
import { i as F } from "../chunks/iO9_dPNE.js";
import { e as _e } from "../chunks/S81raU5Y.js";
import { a as ge } from "../chunks/BdbQ6g_y.js";
import { u as Y } from "../chunks/Bt_9UXew.js";
import { O as he } from "../chunks/HVQD_1Ej.js";
import { d as xe, c as be, b as Se, f as ne } from "../chunks/UPFlzoow.js";
import { C as ye } from "../chunks/C6jTHtu1.js";
import { N as Pe } from "../chunks/D9abP6hj.js";
import { B as we } from "../chunks/CyiD59-2.js";
import { N as Ce } from "../chunks/CzQKNE2_.js";
import { u as Ae } from "../chunks/Bz_VUaol.js";
import { O as K } from "../chunks/B21bTIl7.js";
import { B as se } from "../chunks/C8YTstTD.js";
import { I as le } from "../chunks/DVXwAhn3.js";
import { I as Te } from "../chunks/CTshzOVc.js";
import { u as re } from "../chunks/0cG6LBdy.js";
import { F as ce } from "../chunks/BEbxeS8S.js";
import { L as Ie } from "../chunks/DQwTjYxX.js";
import { S as ie } from "../chunks/ueDoZEuj.js";
import { f as de } from "../chunks/gfDO7tLr.js";
import { T as Oe } from "../chunks/BdAKL3gn.js";
var De = b("<p> </p>"), Ne = b("<p> </p> <p> </p> <!> <!>", 1), Fe = b('<div class="err"> </div>'), Me = b('<div class="flex gap-05"><!> <!></div> <!>', 1), ke = b("<!> <!> <!> <!>", 1);
function Be(R, r) {
  Q(r, true);
  let G = re(), _ = Y(), c = x(""), m = x(false), S = te(() => K(r.scope.name)), l = x(H(r.scope.name)), d = x(void 0), C = x(void 0);
  J(() => {
    r.scope.id && a(l, r.scope.name, true);
  }), J(() => {
    K(r.scope.name) ? (a(d, void 0), a(C, void 0)) : (a(d, r.attrs.map((s) => {
      var _a;
      return { name: s.name, selected: ((_a = r.scope.attr_include_access) == null ? void 0 : _a.includes(s.name)) || false };
    }).toSorted((s, o) => s.name.localeCompare(o.name)), true), a(C, r.attrs.map((s) => {
      var _a;
      return { name: s.name, selected: ((_a = r.scope.attr_include_id) == null ? void 0 : _a.includes(s.name)) || false };
    }).toSorted((s, o) => s.name.localeCompare(o.name)), true));
  });
  async function u(s, o) {
    if (a(c, ""), K(e(l)) || r.scope.name !== e(l) && r.scopes.find((g) => g.name === e(l))) {
      a(c, _.common.nameExistsAlready, true);
      return;
    }
    let y = { scope: e(l) };
    if (e(d)) {
      let g = e(d).filter((n) => n.selected).map((n) => n.name);
      g.length > 0 && (y.attr_include_access = g);
    }
    if (e(C)) {
      let g = e(C).filter((n) => n.selected).map((n) => n.name);
      g.length > 0 && (y.attr_include_id = g);
    }
    let I = await xe(s.action, y);
    I.error ? a(c, I.error.message, true) : (a(m, true), r.onSave(), setTimeout(() => {
      a(m, false);
    }, 2e3));
  }
  const A = te(() => `/auth/v1/scopes/${r.scope.id}`);
  ce(R, { get action() {
    return e(A);
  }, onSubmit: u, children: (s, o) => {
    var y = ke(), I = j(y);
    Ie(I, { label: "ID", mono: true, children: (t, f) => {
      z();
      var v = W();
      B(() => N(v, r.scope.id)), i(t, v);
    }, $$slots: { default: true } });
    var g = w(I, 2);
    le(g, { autocomplete: "off", get label() {
      return _.scopes.name;
    }, get placeholder() {
      return _.scopes.name;
    }, get disabled() {
      return e(S);
    }, width: "14.5rem", required: true, pattern: de, get value() {
      return e(l);
    }, set value(t) {
      a(l, t, true);
    } });
    var n = w(g, 2);
    {
      var M = (t) => {
        var f = De(), v = E(f, true);
        L(f), B(() => N(v, _.scopes.defaultNoMod)), i(t, f);
      }, O = (t) => {
        var f = Ne(), v = j(f), h = E(v, true);
        L(v);
        var P = w(v, 2), q = E(P, true);
        L(P);
        var k = w(P, 2);
        {
          var D = (T) => {
            ie(T, { get items() {
              return e(d);
            }, set items(V) {
              a(d, V, true);
            }, children: (V, ee) => {
              z();
              var U = W("Access Token Mappings");
              i(V, U);
            }, $$slots: { default: true } });
          };
          F(k, (T) => {
            e(d) && T(D);
          });
        }
        var p = w(k, 2);
        {
          var $ = (T) => {
            ie(T, { get items() {
              return e(C);
            }, set items(V) {
              a(C, V, true);
            }, children: (V, ee) => {
              z();
              var U = W("Id Token Mappings");
              i(V, U);
            }, $$slots: { default: true } });
          };
          F(p, (T) => {
            e(C) && T($);
          });
        }
        B(() => {
          N(h, _.scopes.mapping1), N(q, _.scopes.mapping2);
        }), i(t, f);
      };
      F(n, (t) => {
        e(S) ? t(M) : t(O, false);
      });
    }
    var Z = w(n, 2);
    {
      var ae = (t) => {
        var f = Me(), v = j(f), h = E(v);
        se(h, { type: "submit", children: (p, $) => {
          z();
          var T = W();
          B(() => N(T, G.common.save)), i(p, T);
        }, $$slots: { default: true } });
        var P = w(h, 2);
        {
          var q = (p) => {
            Te(p, {});
          };
          F(P, (p) => {
            e(m) && p(q);
          });
        }
        L(v);
        var k = w(v, 2);
        {
          var D = (p) => {
            var $ = Fe(), T = E($, true);
            L($), B(() => N(T, e(c))), i(p, $);
          };
          F(k, (p) => {
            e(c) && p(D);
          });
        }
        i(t, f);
      };
      F(Z, (t) => {
        e(S) || t(ae);
      });
    }
    i(s, y);
  }, $$slots: { default: true } }), X();
}
var Ee = b("<p> </p>"), Le = b('<div class="err"> </div>'), qe = b("<p> </p> <!> <!>", 1);
function $e(R, r) {
  Q(r, true);
  let G = re(), _ = Y(), c = x("");
  async function m() {
    a(c, "");
    let u = await be(`/auth/v1/scopes/${r.scope.id}`);
    u.error ? a(c, u.error.message, true) : r.onSave();
  }
  var S = fe(), l = j(S);
  {
    var d = (u) => {
      var A = Ee(), s = E(A, true);
      L(A), B(() => N(s, _.scopes.deleteDefault)), i(u, A);
    }, C = (u) => {
      var A = qe(), s = j(A), o = E(s, true);
      L(s);
      var y = w(s, 2);
      se(y, { level: -1, onclick: m, children: (n, M) => {
        z();
        var O = W();
        B(() => N(O, G.common.delete)), i(n, O);
      }, $$slots: { default: true } });
      var I = w(y, 2);
      {
        var g = (n) => {
          var M = Le(), O = E(M, true);
          L(M), B(() => N(O, e(c))), i(n, M);
        };
        F(I, (n) => {
          e(c) && n(g);
        });
      }
      B(() => N(o, _.scopes.delete1)), i(u, A);
    };
    F(l, (u) => {
      K(r.scope.name) ? u(d) : u(C, false);
    });
  }
  i(R, S), X();
}
var je = b('<div class="flex"><!></div> <!>', 1);
function Re(R, r) {
  Q(r, true);
  let G = re(), _ = Y();
  const c = [_.nav.config, G.common.delete];
  let m = x(H(c[0])), S = x(void 0);
  J(() => {
    r.scope.id && requestAnimationFrame(() => {
      var _a;
      (_a = e(S)) == null ? void 0 : _a();
    });
  });
  var l = je(), d = j(l), C = E(d);
  Oe(C, { tabs: c, get selected() {
    return e(m);
  }, set selected(o) {
    a(m, o, true);
  }, get focusFirst() {
    return e(S);
  }, set focusFirst(o) {
    a(S, o, true);
  } }), L(d);
  var u = w(d, 2);
  {
    var A = (o) => {
      Be(o, { get attrs() {
        return r.attrs;
      }, get scope() {
        return r.scope;
      }, get scopes() {
        return r.scopes;
      }, get onSave() {
        return r.onSave;
      } });
    }, s = (o, y) => {
      {
        var I = (g) => {
          $e(g, { get scope() {
            return r.scope;
          }, get onSave() {
            return r.onSave;
          } });
        };
        F(o, (g) => {
          e(m) === G.common.delete && g(I);
        }, y);
      }
    };
    F(u, (o) => {
      e(m) === _.nav.config ? o(A) : o(s, false);
    });
  }
  i(R, l), X();
}
var Ge = b('<div class="err"> </div>'), Ve = b("<!> <!> <!>", 1), ze = b('<div class="container svelte-s1196z"><!></div>');
function Ue(R, r) {
  Q(r, true);
  let G = re(), _ = Y(), c = x(void 0), m = x(""), S = x("");
  J(() => {
    requestAnimationFrame(() => {
      var _a;
      (_a = e(c)) == null ? void 0 : _a.focus();
    });
  });
  async function l(u, A) {
    var _a;
    if (r.scopes.find((y) => y.name === e(S))) {
      a(m, _.common.nameExistsAlready, true);
      return;
    }
    a(m, "");
    let s = { scope: e(S) }, o = await Se(u.action, s);
    o.body ? r.onSave(o.body.id) : a(m, ((_a = o.error) == null ? void 0 : _a.message) || "Error", true);
  }
  var d = ze(), C = E(d);
  ce(C, { action: "/auth/v1/scopes", onSubmit: l, children: (u, A) => {
    var s = Ve(), o = j(s);
    le(o, { autocomplete: "off", get label() {
      return _.scopes.name;
    }, get placeholder() {
      return _.scopes.name;
    }, required: true, pattern: de, get ref() {
      return e(c);
    }, set ref(n) {
      a(c, n, true);
    }, get value() {
      return e(S);
    }, set value(n) {
      a(S, n, true);
    } });
    var y = w(o, 2);
    se(y, { type: "submit", children: (n, M) => {
      z();
      var O = W();
      B(() => N(O, G.common.save)), i(n, O);
    }, $$slots: { default: true } });
    var I = w(y, 2);
    {
      var g = (n) => {
        var M = Ge(), O = E(M, true);
        L(M), B(() => N(O, e(m))), i(n, M);
      };
      F(I, (n) => {
        e(m) && n(g);
      });
    }
    i(u, s);
  }, $$slots: { default: true } }), L(d), i(R, d), X();
}
var We = b('<span class="default svelte-1bjjcss"><i>default</i></span>'), He = b(" <!>", 1), Je = b("<div></div> <!>", 1), Ke = b("<!> <!>", 1), Qe = b('<div class="err"> </div>'), Xe = b('<!> <div id="scopes"><!></div>', 1), Ye = b("<!> <!>", 1);
function Ze(R, r) {
  Q(r, true);
  let G = Y(), _ = x(void 0), c = x(""), m = Ae("sid"), S = x(H([])), l = x(H([])), d = x(H([])), C = x(void 0);
  const u = [G.common.name, "ID"];
  let A = x(H(u[0])), s = x("");
  const o = [G.common.name, "ID"];
  pe(() => {
    I(), y();
  }), J(() => {
    a(C, e(l).find((t) => t.id === m.get()), true);
  }), J(() => {
    let t = e(s).toLowerCase();
    t ? e(A) === u[0] ? a(d, e(l).filter((f) => {
      var _a;
      return (_a = f.name) == null ? void 0 : _a.toLowerCase().includes(t);
    }), true) : e(A) === u[1] && a(d, e(l).filter((f) => f.id.toLowerCase().includes(t)), true) : a(d, e(l), true);
  });
  async function y() {
    var _a;
    let t = await ne("/auth/v1/users/attr");
    t.body ? a(S, t.body.values, true) : a(c, ((_a = t.error) == null ? void 0 : _a.message) || "Error", true);
  }
  async function I() {
    var _a;
    let t = await ne("/auth/v1/scopes");
    t.body ? a(l, t.body, true) : a(c, ((_a = t.error) == null ? void 0 : _a.message) || "Error", true);
  }
  function g(t, f) {
    let v = f === "up";
    t === o[0] ? e(l).sort((h, P) => v ? h.name.localeCompare(P.name) : P.name.localeCompare(h.name)) : t === o[1] && e(l).sort((h, P) => v ? h.id.localeCompare(P.id) : P.id.localeCompare(h.id));
  }
  function n() {
    I();
  }
  async function M(t) {
    var _a;
    (_a = e(_)) == null ? void 0 : _a(), await I(), m.set(t);
  }
  var O = Ye(), Z = j(O);
  Ce(Z, { paddingTop: "2.1rem", buttonTilesAriaControls: "scopes", width: "min(20rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (f) => {
    var v = Je(), h = j(v);
    ge(h, "", {}, { height: ".5rem" });
    var P = w(h, 2);
    _e(P, 17, () => e(d), (q) => q.id, (q, k, D, p) => {
      const $ = te(() => m.get() === e(k).id);
      Pe(q, { onclick: () => m.set(e(k).id), get selected() {
        return e($);
      }, children: (T, V) => {
        z();
        var ee = He(), U = j(ee), ue = w(U);
        {
          var ve = (oe) => {
            var me = We();
            i(oe, me);
          };
          F(ue, (oe) => {
            K(e(k).name) && oe(ve);
          });
        }
        B(() => N(U, `${e(k).name ?? ""} `)), i(T, ee);
      } });
    }), i(f, v);
  }, children: (f, v) => {
    var h = Ke(), P = j(h);
    const q = te(() => e(l).length === 0 ? 1 : 2);
    we(P, { get level() {
      return e(q);
    }, alignRight: true, get closeModal() {
      return e(_);
    }, set closeModal(D) {
      a(_, D, true);
    }, children: (D, p) => {
      Ue(D, { onSave: M, get scopes() {
        return e(l);
      } });
    }, $$slots: { default: true } });
    var k = w(P, 2);
    he(k, { searchOptions: u, orderOptions: o, onChangeOrder: g, searchWidth: "min(19.5rem, 100dvw - .5rem)", get searchOption() {
      return e(A);
    }, set searchOption(D) {
      a(A, D, true);
    }, get value() {
      return e(s);
    }, set value(D) {
      a(s, D, true);
    } }), i(f, h);
  }, $$slots: { buttonTiles: true, default: true } });
  var ae = w(Z, 2);
  ye(ae, { children: (t, f) => {
    var v = Xe(), h = j(v);
    {
      var P = (p) => {
        var $ = Qe(), T = E($, true);
        L($), B(() => N(T, e(c))), i(p, $);
      };
      F(h, (p) => {
        e(c) && p(P);
      });
    }
    var q = w(h, 2), k = E(q);
    {
      var D = (p) => {
        Re(p, { get attrs() {
          return e(S);
        }, get scope() {
          return e(C);
        }, get scopes() {
          return e(l);
        }, onSave: n });
      };
      F(k, (p) => {
        e(C) && p(D);
      });
    }
    L(q), i(t, v);
  } }), i(R, O), X();
}
function wt(R) {
  Ze(R, {});
}
export {
  wt as component
};
