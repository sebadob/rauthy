import { t as S, e as H, a as l, d as fe } from "../chunks/DLBGyKVC.js";
import { p as Q, k as b, aa as J, l as a, f as R, a8 as U, t as E, s as C, j as e, a9 as te, c as L, r as q, a as X, a6 as pe } from "../chunks/CmQi0fbH.js";
import { s as F } from "../chunks/BjaYyaa_.js";
import { i as M } from "../chunks/C6bK2EJJ.js";
import { e as _e } from "../chunks/YQCw2eEa.js";
import { a as ge } from "../chunks/DOlUUCkJ.js";
import { p as o } from "../chunks/B_ggA-0N.js";
import { u as Y } from "../chunks/CZf6fJph.js";
import { O as he } from "../chunks/BBZ-aIjz.js";
import { d as xe, c as be, b as Se, f as ne } from "../chunks/BO1A6s0c.js";
import { C as ye } from "../chunks/Bfi4YUpT.js";
import { N as Pe } from "../chunks/DpqPmJAJ.js";
import { B as we } from "../chunks/BLvNyGE2.js";
import { N as Ce } from "../chunks/ClbMy9q0.js";
import { u as Ae } from "../chunks/BctQMYGV.js";
import { O as K } from "../chunks/B21bTIl7.js";
import { B as se } from "../chunks/DPLO-ozG.js";
import { I as le } from "../chunks/DwPr_s7h.js";
import { I as Te } from "../chunks/CAzQQhB1.js";
import { u as re } from "../chunks/DGTOa5g8.js";
import { F as ce } from "../chunks/CNMQp9ma.js";
import { L as Ie } from "../chunks/gwKtuDud.js";
import { S as ie } from "../chunks/1UVa_F2Q.js";
import { f as de } from "../chunks/gfDO7tLr.js";
import { T as Oe } from "../chunks/MKf726CY.js";
var De = S("<p> </p>"), Ne = S("<p> </p> <p> </p> <!> <!>", 1), Fe = S('<div class="err"> </div>'), Me = S('<div class="flex gap-05"><!> <!></div> <!>', 1), ke = S("<!> <!> <!> <!>", 1);
function Be(G, r) {
  Q(r, true);
  let V = re(), g = Y(), d = b(""), f = b(false), y = te(() => K(r.scope.name)), c = b(o(r.scope.name)), v = b(void 0), A = b(void 0);
  J(() => {
    r.scope.id && a(c, o(r.scope.name));
  }), J(() => {
    K(r.scope.name) ? (a(v, void 0), a(A, void 0)) : (a(v, o(r.attrs.map((n) => {
      var _a;
      return { name: n.name, selected: ((_a = r.scope.attr_include_access) == null ? void 0 : _a.includes(n.name)) || false };
    }).toSorted((n, s) => n.name.localeCompare(s.name)))), a(A, o(r.attrs.map((n) => {
      var _a;
      return { name: n.name, selected: ((_a = r.scope.attr_include_id) == null ? void 0 : _a.includes(n.name)) || false };
    }).toSorted((n, s) => n.name.localeCompare(s.name)))));
  });
  async function m(n, s) {
    if (a(d, ""), K(e(c)) || r.scope.name !== e(c) && r.scopes.find((h) => h.name === e(c))) {
      a(d, o(g.common.nameExistsAlready));
      return;
    }
    let P = { scope: e(c) };
    if (e(v)) {
      let h = e(v).filter((i) => i.selected).map((i) => i.name);
      h.length > 0 && (P.attr_include_access = h);
    }
    if (e(A)) {
      let h = e(A).filter((i) => i.selected).map((i) => i.name);
      h.length > 0 && (P.attr_include_id = h);
    }
    let O = await xe(n.action, P);
    O.error ? a(d, o(O.error.message)) : (a(f, true), r.onSave(), setTimeout(() => {
      a(f, false);
    }, 2e3));
  }
  const T = te(() => `/auth/v1/scopes/${r.scope.id}`);
  ce(G, { get action() {
    return e(T);
  }, onSubmit: m, children: (n, s) => {
    var P = ke(), O = R(P);
    Ie(O, { label: "ID", mono: true, children: (t, p) => {
      U();
      var u = H();
      E(() => F(u, r.scope.id)), l(t, u);
    }, $$slots: { default: true } });
    var h = C(O, 2);
    le(h, { autocomplete: "off", get label() {
      return g.scopes.name;
    }, get placeholder() {
      return g.scopes.name;
    }, get disabled() {
      return e(y);
    }, width: "14.5rem", required: true, pattern: de, get value() {
      return e(c);
    }, set value(t) {
      a(c, o(t));
    } });
    var i = C(h, 2);
    {
      var k = (t) => {
        var p = De(), u = L(p, true);
        q(p), E(() => F(u, g.scopes.defaultNoMod)), l(t, p);
      }, D = (t) => {
        var p = Ne(), u = R(p), x = L(u, true);
        q(u);
        var w = C(u, 2), $ = L(w, true);
        q(w);
        var B = C(w, 2);
        {
          var N = (I) => {
            ie(I, { get items() {
              return e(v);
            }, set items(z) {
              a(v, o(z));
            }, children: (z, ee) => {
              U();
              var W = H("Access Token Mappings");
              l(z, W);
            }, $$slots: { default: true } });
          };
          M(B, (I) => {
            e(v) && I(N);
          });
        }
        var _ = C(B, 2);
        {
          var j = (I) => {
            ie(I, { get items() {
              return e(A);
            }, set items(z) {
              a(A, o(z));
            }, children: (z, ee) => {
              U();
              var W = H("Id Token Mappings");
              l(z, W);
            }, $$slots: { default: true } });
          };
          M(_, (I) => {
            e(A) && I(j);
          });
        }
        E(() => {
          F(x, g.scopes.mapping1), F($, g.scopes.mapping2);
        }), l(t, p);
      };
      M(i, (t) => {
        e(y) ? t(k) : t(D, false);
      });
    }
    var Z = C(i, 2);
    {
      var ae = (t) => {
        var p = Me(), u = R(p), x = L(u);
        se(x, { type: "submit", children: (_, j) => {
          U();
          var I = H();
          E(() => F(I, V.common.save)), l(_, I);
        }, $$slots: { default: true } });
        var w = C(x, 2);
        {
          var $ = (_) => {
            Te(_, {});
          };
          M(w, (_) => {
            e(f) && _($);
          });
        }
        q(u);
        var B = C(u, 2);
        {
          var N = (_) => {
            var j = Fe(), I = L(j, true);
            q(j), E(() => F(I, e(d))), l(_, j);
          };
          M(B, (_) => {
            e(d) && _(N);
          });
        }
        l(t, p);
      };
      M(Z, (t) => {
        e(y) || t(ae);
      });
    }
    l(n, P);
  }, $$slots: { default: true } }), X();
}
var Ee = S("<p> </p>"), Le = S('<div class="err"> </div>'), qe = S("<p> </p> <!> <!>", 1);
function $e(G, r) {
  Q(r, true);
  let V = re(), g = Y(), d = b("");
  async function f() {
    a(d, "");
    let m = await be(`/auth/v1/scopes/${r.scope.id}`);
    m.error ? a(d, o(m.error.message)) : r.onSave();
  }
  var y = fe(), c = R(y);
  {
    var v = (m) => {
      var T = Ee(), n = L(T, true);
      q(T), E(() => F(n, g.scopes.deleteDefault)), l(m, T);
    }, A = (m) => {
      var T = qe(), n = R(T), s = L(n, true);
      q(n);
      var P = C(n, 2);
      se(P, { level: -1, onclick: f, children: (i, k) => {
        U();
        var D = H();
        E(() => F(D, V.common.delete)), l(i, D);
      }, $$slots: { default: true } });
      var O = C(P, 2);
      {
        var h = (i) => {
          var k = Le(), D = L(k, true);
          q(k), E(() => F(D, e(d))), l(i, k);
        };
        M(O, (i) => {
          e(d) && i(h);
        });
      }
      E(() => F(s, g.scopes.delete1)), l(m, T);
    };
    M(c, (m) => {
      K(r.scope.name) ? m(v) : m(A, false);
    });
  }
  l(G, y), X();
}
var je = S('<div class="flex"><!></div> <!>', 1);
function Re(G, r) {
  Q(r, true);
  let V = re(), g = Y();
  const d = [g.nav.config, V.common.delete];
  let f = b(o(d[0])), y = b(void 0);
  J(() => {
    r.scope.id && requestAnimationFrame(() => {
      var _a;
      (_a = e(y)) == null ? void 0 : _a();
    });
  });
  var c = je(), v = R(c), A = L(v);
  Oe(A, { tabs: d, get selected() {
    return e(f);
  }, set selected(s) {
    a(f, o(s));
  }, get focusFirst() {
    return e(y);
  }, set focusFirst(s) {
    a(y, o(s));
  } }), q(v);
  var m = C(v, 2);
  {
    var T = (s) => {
      Be(s, { get attrs() {
        return r.attrs;
      }, get scope() {
        return r.scope;
      }, get scopes() {
        return r.scopes;
      }, get onSave() {
        return r.onSave;
      } });
    }, n = (s, P) => {
      {
        var O = (h) => {
          $e(h, { get scope() {
            return r.scope;
          }, get onSave() {
            return r.onSave;
          } });
        };
        M(s, (h) => {
          e(f) === V.common.delete && h(O);
        }, P);
      }
    };
    M(m, (s) => {
      e(f) === g.nav.config ? s(T) : s(n, false);
    });
  }
  l(G, c), X();
}
var Ge = S('<div class="err"> </div>'), Ve = S("<!> <!> <!>", 1), ze = S('<div class="container svelte-s1196z"><!></div>');
function Ue(G, r) {
  Q(r, true);
  let V = re(), g = Y(), d = b(void 0), f = b(""), y = b("");
  J(() => {
    requestAnimationFrame(() => {
      var _a;
      (_a = e(d)) == null ? void 0 : _a.focus();
    });
  });
  async function c(m, T) {
    var _a;
    if (r.scopes.find((P) => P.name === e(y))) {
      a(f, o(g.common.nameExistsAlready));
      return;
    }
    a(f, "");
    let n = { scope: e(y) }, s = await Se(m.action, n);
    s.body ? r.onSave(s.body.id) : a(f, o(((_a = s.error) == null ? void 0 : _a.message) || "Error"));
  }
  var v = ze(), A = L(v);
  ce(A, { action: "/auth/v1/scopes", onSubmit: c, children: (m, T) => {
    var n = Ve(), s = R(n);
    le(s, { autocomplete: "off", get label() {
      return g.scopes.name;
    }, get placeholder() {
      return g.scopes.name;
    }, required: true, pattern: de, get ref() {
      return e(d);
    }, set ref(i) {
      a(d, o(i));
    }, get value() {
      return e(y);
    }, set value(i) {
      a(y, o(i));
    } });
    var P = C(s, 2);
    se(P, { type: "submit", children: (i, k) => {
      U();
      var D = H();
      E(() => F(D, V.common.save)), l(i, D);
    }, $$slots: { default: true } });
    var O = C(P, 2);
    {
      var h = (i) => {
        var k = Ge(), D = L(k, true);
        q(k), E(() => F(D, e(f))), l(i, k);
      };
      M(O, (i) => {
        e(f) && i(h);
      });
    }
    l(m, n);
  }, $$slots: { default: true } }), q(v), l(G, v), X();
}
var We = S('<span class="default svelte-1bjjcss"><i>default</i></span>'), He = S(" <!>", 1), Je = S("<div></div> <!>", 1), Ke = S("<!> <!>", 1), Qe = S('<div class="err"> </div>'), Xe = S('<!> <div id="scopes"><!></div>', 1), Ye = S("<!> <!>", 1);
function Ze(G, r) {
  Q(r, true);
  let V = Y(), g = b(void 0), d = b(""), f = Ae("sid"), y = b(o([])), c = b(o([])), v = b(o([])), A = b(void 0);
  const m = [V.common.name, "ID"];
  let T = b(o(m[0])), n = b("");
  const s = [V.common.name, "ID"];
  pe(() => {
    O(), P();
  }), J(() => {
    a(A, o(e(c).find((t) => t.id === f.get())));
  }), J(() => {
    let t = e(n).toLowerCase();
    t ? e(T) === m[0] ? a(v, o(e(c).filter((p) => {
      var _a;
      return (_a = p.name) == null ? void 0 : _a.toLowerCase().includes(t);
    }))) : e(T) === m[1] && a(v, o(e(c).filter((p) => p.id.toLowerCase().includes(t)))) : a(v, o(e(c)));
  });
  async function P() {
    var _a;
    let t = await ne("/auth/v1/users/attr");
    t.body ? a(y, o(t.body.values)) : a(d, o(((_a = t.error) == null ? void 0 : _a.message) || "Error"));
  }
  async function O() {
    var _a;
    let t = await ne("/auth/v1/scopes");
    t.body ? a(c, o(t.body)) : a(d, o(((_a = t.error) == null ? void 0 : _a.message) || "Error"));
  }
  function h(t, p) {
    let u = p === "up";
    t === s[0] ? e(c).sort((x, w) => u ? x.name.localeCompare(w.name) : w.name.localeCompare(x.name)) : t === s[1] && e(c).sort((x, w) => u ? x.id.localeCompare(w.id) : w.id.localeCompare(x.id));
  }
  function i() {
    O();
  }
  async function k(t) {
    var _a;
    (_a = e(g)) == null ? void 0 : _a(), await O(), f.set(t);
  }
  var D = Ye(), Z = R(D);
  Ce(Z, { paddingTop: "2.1rem", buttonTilesAriaControls: "scopes", width: "min(20rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (p) => {
    var u = Je(), x = R(u);
    ge(x, "", {}, { height: ".5rem" });
    var w = C(x, 2);
    _e(w, 17, () => e(v), ($) => $.id, ($, B, N, _) => {
      const j = te(() => f.get() === e(B).id);
      Pe($, { onclick: () => f.set(e(B).id), get selected() {
        return e(j);
      }, children: (I, z) => {
        U();
        var ee = He(), W = R(ee), ve = C(W);
        {
          var me = (oe) => {
            var ue = We();
            l(oe, ue);
          };
          M(ve, (oe) => {
            K(e(B).name) && oe(me);
          });
        }
        E(() => F(W, `${e(B).name ?? ""} `)), l(I, ee);
      } });
    }), l(p, u);
  }, children: (p, u) => {
    var x = Ke(), w = R(x);
    const $ = te(() => e(c).length === 0 ? 1 : 2);
    we(w, { get level() {
      return e($);
    }, alignRight: true, get closeModal() {
      return e(g);
    }, set closeModal(N) {
      a(g, o(N));
    }, children: (N, _) => {
      Ue(N, { onSave: k, get scopes() {
        return e(c);
      } });
    }, $$slots: { default: true } });
    var B = C(w, 2);
    he(B, { searchOptions: m, orderOptions: s, onChangeOrder: h, searchWidth: "min(19.5rem, 100dvw - .5rem)", get searchOption() {
      return e(T);
    }, set searchOption(N) {
      a(T, o(N));
    }, get value() {
      return e(n);
    }, set value(N) {
      a(n, o(N));
    } }), l(p, x);
  }, $$slots: { buttonTiles: true, default: true } });
  var ae = C(Z, 2);
  ye(ae, { children: (t, p) => {
    var u = Xe(), x = R(u);
    {
      var w = (_) => {
        var j = Qe(), I = L(j, true);
        q(j), E(() => F(I, e(d))), l(_, j);
      };
      M(x, (_) => {
        e(d) && _(w);
      });
    }
    var $ = C(x, 2), B = L($);
    {
      var N = (_) => {
        Re(_, { get attrs() {
          return e(y);
        }, get scope() {
          return e(A);
        }, get scopes() {
          return e(c);
        }, onSave: i });
      };
      M(B, (_) => {
        e(A) && _(N);
      });
    }
    q($), l(t, u);
  } }), l(G, D), X();
}
function Ct(G) {
  Ze(G, {});
}
export {
  Ct as component
};
