import { t as P, e as j, a as i, d as ee } from "../chunks/BH6NCLk-.js";
import { p as H, aa as z, f as B, l as s, k as y, j as e, s as T, t as D, c as M, a as J, a7 as U, r as $, a9 as W, a5 as ne } from "../chunks/CvlvO1XB.js";
import { s as k } from "../chunks/CTI4QPiR.js";
import { i as E } from "../chunks/BUO_AUgz.js";
import { e as se } from "../chunks/BpWRzPRQ.js";
import { B as Y, a as le } from "../chunks/DMkkW5Nn.js";
import { p as n } from "../chunks/Wh68IIk2.js";
import { u as K } from "../chunks/D8mHI_K9.js";
import { B as ie } from "../chunks/ClvgHNP_.js";
import { C as de } from "../chunks/BnPoFdx3.js";
import { N as ve } from "../chunks/D-L0o8jR.js";
import { N as me } from "../chunks/DZ-K8XgE.js";
import { O as ue } from "../chunks/CONsTX1L.js";
import { b as fe, d as ce, c as _e, f as ge } from "../chunks/BO1A6s0c.js";
import { u as he } from "../chunks/BYZsu9rB.js";
import { I as re } from "../chunks/BZu_mjh1.js";
import { f as te } from "../chunks/BRCxk8by.js";
import { F as ae } from "../chunks/CsaAZyUr.js";
import { u as Q } from "../chunks/CUqQZdNU.js";
import { T as pe } from "../chunks/_OE2Cq0B.js";
import { h as oe } from "../chunks/i8Xqpu09.js";
import { I as be } from "../chunks/Nks81rMs.js";
import { L as xe } from "../chunks/nnFaiMsH.js";
var ye = P('<div class="err"> </div>'), Pe = P("<!> <!> <!>", 1), Se = P('<div class="container svelte-s1196z"><!></div>');
function we(I, a) {
  H(a, true);
  let O = Q(), c = K(), v = y(void 0), o = y(""), _ = y("");
  z(() => {
    requestAnimationFrame(() => {
      var _a;
      (_a = e(v)) == null ? void 0 : _a.focus();
    });
  });
  async function g(p, u) {
    var _a;
    if (a.roles.find((b) => b.name === e(_))) {
      s(o, n(c.common.nameExistsAlready));
      return;
    }
    s(o, "");
    let d = { role: e(_) }, t = await fe(p.action, d);
    t.body ? a.onSave(t.body.id) : s(o, n(((_a = t.error) == null ? void 0 : _a.message) || "Error"));
  }
  var h = Se(), N = M(h);
  ae(N, { action: "/auth/v1/roles", onSubmit: g, children: (p, u) => {
    var d = Pe(), t = B(d);
    re(t, { autocomplete: "off", get label() {
      return c.roles.name;
    }, get placeholder() {
      return c.roles.name;
    }, required: true, pattern: te, get ref() {
      return e(v);
    }, set ref(m) {
      s(v, n(m));
    }, get value() {
      return e(_);
    }, set value(m) {
      s(_, n(m));
    } });
    var b = T(t, 2);
    Y(b, { type: "submit", children: (m, S) => {
      U();
      var C = j();
      D(() => k(C, O.common.save)), i(m, C);
    }, $$slots: { default: true } });
    var F = T(b, 2);
    {
      var L = (m) => {
        var S = ye(), C = M(S, true);
        $(S), D(() => k(C, e(o))), i(m, S);
      };
      E(F, (m) => {
        e(o) && m(L);
      });
    }
    i(p, d);
  }, $$slots: { default: true } }), $(h), i(I, h), J();
}
var Ae = P("<p><!></p>"), Ce = P('<div class="flex gap-05"><!> <!></div>'), Re = P('<div class="err"> </div>'), Te = P("<!> <!> <!> <!>", 1);
function Ne(I, a) {
  H(a, true);
  let O = Q(), c = K(), v = y(""), o = y(false), _ = W(() => a.role.name === "rauthy_admin"), g = y(n(a.role.name));
  z(() => {
    a.role.id && s(g, n(a.role.name));
  });
  async function h(p, u) {
    if (s(v, ""), e(g) !== a.role.name && a.roles.find((b) => b.name === e(g))) {
      s(v, n(c.common.nameExistsAlready));
      return;
    }
    let d = { role: e(g) }, t = await ce(p.action, d);
    t.error ? s(v, n(t.error.message)) : (s(o, true), a.onSave(), setTimeout(() => {
      s(o, false);
    }, 2e3));
  }
  const N = W(() => `/auth/v1/roles/${a.role.id}`);
  ae(I, { get action() {
    return e(N);
  }, onSubmit: h, children: (p, u) => {
    var d = Te(), t = B(d);
    xe(t, { label: "ID", mono: true, children: (r, l) => {
      U();
      var f = j();
      D(() => k(f, a.role.id)), i(r, f);
    } });
    var b = T(t, 2);
    re(b, { autocomplete: "off", get label() {
      return c.scopes.name;
    }, get placeholder() {
      return c.scopes.name;
    }, get disabled() {
      return e(_);
    }, width: "14.5rem", required: true, pattern: te, get value() {
      return e(g);
    }, set value(r) {
      s(g, n(r));
    } });
    var F = T(b, 2);
    {
      var L = (r) => {
        var l = Ae(), f = M(l);
        oe(f, () => c.roles.adminNoMod), $(l), i(r, l);
      }, m = (r) => {
        var l = Ce(), f = M(l);
        Y(f, { type: "submit", children: (A, G) => {
          U();
          var R = j();
          D(() => k(R, O.common.save)), i(A, R);
        }, $$slots: { default: true } });
        var x = T(f, 2);
        {
          var w = (A) => {
            be(A, {});
          };
          E(x, (A) => {
            e(o) && A(w);
          });
        }
        $(l), i(r, l);
      };
      E(F, (r) => {
        e(_) ? r(L) : r(m, false);
      });
    }
    var S = T(F, 2);
    {
      var C = (r) => {
        var l = Re(), f = M(l, true);
        $(l), D(() => k(f, e(v))), i(r, l);
      };
      E(S, (r) => {
        e(v) && r(C);
      });
    }
    i(p, d);
  }, $$slots: { default: true } }), J();
}
var Ie = P("<p><!></p>"), Oe = P('<div class="err"> </div>'), Fe = P("<p> </p> <!> <!>", 1);
function Me(I, a) {
  H(a, true);
  let O = Q(), c = K(), v = y(""), o = W(() => a.role.name === "rauthy_admin");
  async function _() {
    s(v, "");
    let u = await _e(`/auth/v1/roles/${a.role.id}`);
    u.error ? s(v, n(u.error.message)) : a.onSave();
  }
  var g = ee(), h = B(g);
  {
    var N = (u) => {
      var d = Ie(), t = M(d);
      oe(t, () => c.roles.adminNoMod), $(d), i(u, d);
    }, p = (u) => {
      var d = Fe(), t = B(d), b = M(t, true);
      $(t);
      var F = T(t, 2);
      Y(F, { level: -1, onclick: _, children: (S, C) => {
        U();
        var r = j();
        D(() => k(r, O.common.delete)), i(S, r);
      }, $$slots: { default: true } });
      var L = T(F, 2);
      {
        var m = (S) => {
          var C = Oe(), r = M(C, true);
          $(C), D(() => k(r, e(v))), i(S, C);
        };
        E(L, (S) => {
          e(v) && S(m);
        });
      }
      D(() => k(b, c.roles.delete1)), i(u, d);
    };
    E(h, (u) => {
      e(o) ? u(N) : u(p, false);
    });
  }
  i(I, g), J();
}
var $e = P('<div class="flex"><!></div> <!>', 1);
function Be(I, a) {
  H(a, true);
  let O = Q(), c = K();
  const v = [c.nav.config, O.common.delete];
  let o = y(n(v[0])), _ = y(void 0);
  z(() => {
    a.role.id && requestAnimationFrame(() => {
      var _a;
      (_a = e(_)) == null ? void 0 : _a();
    });
  });
  var g = $e(), h = B(g), N = M(h);
  pe(N, { tabs: v, get selected() {
    return e(o);
  }, set selected(t) {
    s(o, n(t));
  }, get focusFirst() {
    return e(_);
  }, set focusFirst(t) {
    s(_, n(t));
  } }), $(h);
  var p = T(h, 2);
  {
    var u = (t) => {
      Ne(t, { get role() {
        return a.role;
      }, get roles() {
        return a.roles;
      }, get onSave() {
        return a.onSave;
      } });
    }, d = (t) => {
      var b = ee(), F = B(b);
      {
        var L = (m) => {
          Me(m, { get role() {
            return a.role;
          }, get onSave() {
            return a.onSave;
          } });
        };
        E(F, (m) => {
          e(o) === O.common.delete && m(L);
        }, true);
      }
      i(t, b);
    };
    E(p, (t) => {
      e(o) === c.nav.config ? t(u) : t(d, false);
    });
  }
  i(I, g), J();
}
var De = P("<div></div> <!>", 1), ke = P("<!> <!>", 1), Ee = P('<div class="err"> </div>'), Le = P('<!> <div id="groups"><!></div>', 1), qe = P("<!> <!>", 1);
function Ge(I, a) {
  H(a, true);
  let O = K(), c = y(void 0), v = y(""), o = y(n([])), _ = y(n([])), g = y(void 0), h = he("rid");
  const N = [O.common.name, "ID"];
  let p = y(n(N[0])), u = y("");
  const d = [O.common.name, "ID"];
  ne(() => {
    t();
  }), z(() => {
    s(g, n(e(o).find((r) => r.id === h.get())));
  }), z(() => {
    let r = e(u).toLowerCase();
    r ? e(p) === N[0] ? s(_, n(e(o).filter((l) => {
      var _a;
      return (_a = l.name) == null ? void 0 : _a.toLowerCase().includes(r);
    }))) : e(p) === N[1] && s(_, n(e(o).filter((l) => l.id.toLowerCase().includes(r)))) : s(_, n(e(o)));
  });
  async function t() {
    var _a;
    let r = await ge("/auth/v1/roles");
    r.body ? s(o, n(r.body)) : s(v, n(((_a = r.error) == null ? void 0 : _a.message) || "Error"));
  }
  function b(r, l) {
    let f = l === "up";
    r === d[0] ? e(o).sort((x, w) => f ? x.name.localeCompare(w.name) : w.name.localeCompare(x.name)) : r === d[1] && e(o).sort((x, w) => f ? x.id.localeCompare(w.id) : w.id.localeCompare(x.id));
  }
  function F() {
    t();
  }
  async function L(r) {
    var _a;
    (_a = e(c)) == null ? void 0 : _a(), await t(), h.set(r);
  }
  var m = qe(), S = B(m);
  me(S, { paddingTop: "2.1rem", buttonTilesAriaControls: "groups", width: "min(20rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (l) => {
    var f = De(), x = B(f);
    le(x, "height", ".5rem");
    var w = T(x, 2);
    se(w, 17, () => e(_), (A) => A.id, (A, G, R, q) => {
      const V = W(() => h.get() === e(G).id);
      ve(A, { onclick: () => h.set(e(G).id), get selected() {
        return e(V);
      }, children: (X, Ve) => {
        U();
        var Z = j();
        D(() => k(Z, e(G).name)), i(X, Z);
      } });
    }), i(l, f);
  }, children: (l, f) => {
    var x = ke(), w = B(x);
    const A = W(() => e(o).length === 0 ? 1 : 2);
    ie(w, { get level() {
      return e(A);
    }, alignRight: true, get closeModal() {
      return e(c);
    }, set closeModal(R) {
      s(c, n(R));
    }, children: (R, q) => {
      we(R, { onSave: L, get roles() {
        return e(o);
      } });
    }, $$slots: { default: true } });
    var G = T(w, 2);
    ue(G, { searchOptions: N, orderOptions: d, onChangeOrder: b, searchWidth: "min(19.5rem, 100dvw - .5rem)", get searchOption() {
      return e(p);
    }, set searchOption(R) {
      s(p, n(R));
    }, get value() {
      return e(u);
    }, set value(R) {
      s(u, n(R));
    } }), i(l, x);
  }, $$slots: { buttonTiles: true, default: true } });
  var C = T(S, 2);
  de(C, { children: (r, l) => {
    var f = Le(), x = B(f);
    {
      var w = (q) => {
        var V = Ee(), X = M(V, true);
        $(V), D(() => k(X, e(v))), i(q, V);
      };
      E(x, (q) => {
        e(v) && q(w);
      });
    }
    var A = T(x, 2), G = M(A);
    {
      var R = (q) => {
        Be(q, { get role() {
          return e(g);
        }, get roles() {
          return e(o);
        }, onSave: F });
      };
      E(G, (q) => {
        e(g) && q(R);
      });
    }
    $(A), i(r, f);
  } }), i(I, m), J();
}
function ur(I) {
  Ge(I, {});
}
export {
  ur as component
};
