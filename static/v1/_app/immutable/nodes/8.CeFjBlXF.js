import { t as T, a as f, e as K } from "../chunks/BxmJRzoY.js";
import { p as V, j as c, a5 as G, a as W, f as M, l as o, k as e, s as b, t as $, c as B, a3 as Q, r as I, a1 as j, a4 as Y, a0 as oe } from "../chunks/w0HvPX0p.js";
import { s as E } from "../chunks/BzP2S3Z_.js";
import { i as R } from "../chunks/iO9_dPNE.js";
import { e as se } from "../chunks/S81raU5Y.js";
import { a as ne } from "../chunks/BdbQ6g_y.js";
import { u as H } from "../chunks/DHOKTGcE.js";
import { B as ie } from "../chunks/DoeALvoe.js";
import { C as le } from "../chunks/C6jTHtu1.js";
import { N as ue } from "../chunks/D9abP6hj.js";
import { N as de } from "../chunks/CzQKNE2_.js";
import { O as me } from "../chunks/85nGGue2.js";
import { b as ve, d as ce, c as fe, f as ge } from "../chunks/BO1A6s0c.js";
import { u as _e } from "../chunks/Di69SM1b.js";
import { B as Z } from "../chunks/C8YTstTD.js";
import { I as J } from "../chunks/Q4PIg3iI.js";
import { d as te, e as re } from "../chunks/gfDO7tLr.js";
import { F as ae } from "../chunks/CDe-qvZi.js";
import { u as U } from "../chunks/0cG6LBdy.js";
import { T as he } from "../chunks/BdAKL3gn.js";
import { I as pe } from "../chunks/CTshzOVc.js";
var be = T('<div class="err"> </div>'), xe = T("<!> <!> <!> <!>", 1), Ae = T('<div class="container svelte-s1196z"><!></div>');
function ye(w, r) {
  V(r, true);
  let P = U(), u = H(), l = c(void 0), s = c(""), n = c(""), v = c("");
  G(() => {
    requestAnimationFrame(() => {
      var _a;
      (_a = e(l)) == null ? void 0 : _a.focus();
    });
  });
  async function h(A, d) {
    var _a;
    if (r.attrs.find((S) => S.name === e(n))) {
      o(s, u.common.nameExistsAlready, true);
      return;
    }
    o(s, "");
    let t = { name: e(n), desc: e(v) || void 0 }, m = await ve(A.action, t);
    m.body ? r.onSave(m.body.name) : o(s, ((_a = m.error) == null ? void 0 : _a.message) || "Error", true);
  }
  var p = Ae(), x = B(p);
  ae(x, { action: "/auth/v1/users/attr", onSubmit: h, children: (A, d) => {
    var t = xe(), m = M(t);
    J(m, { autocomplete: "off", get label() {
      return u.attrs.name;
    }, get placeholder() {
      return u.attrs.name;
    }, required: true, pattern: te, get ref() {
      return e(l);
    }, set ref(_) {
      o(l, _, true);
    }, get value() {
      return e(n);
    }, set value(_) {
      o(n, _, true);
    } });
    var S = b(m, 2);
    J(S, { autocomplete: "off", get label() {
      return u.attrs.desc;
    }, get placeholder() {
      return u.attrs.desc;
    }, pattern: re, get value() {
      return e(v);
    }, set value(_) {
      o(v, _, true);
    } });
    var C = b(S, 2);
    Z(C, { type: "submit", children: (_, i) => {
      Q();
      var a = K();
      $(() => E(a, P.common.save)), f(_, a);
    }, $$slots: { default: true } });
    var D = b(C, 2);
    {
      var L = (_) => {
        var i = be(), a = B(i, true);
        I(i), $(() => E(a, e(s))), f(_, i);
      };
      R(D, (_) => {
        e(s) && _(L);
      });
    }
    f(A, t);
  }, $$slots: { default: true } }), I(p), f(w, p), W();
}
var Te = T('<div class="err"> </div>'), Se = T('<!> <!> <div class="flex gap-05"><!> <!></div> <!>', 1);
function we(w, r) {
  V(r, true);
  let P = U(), u = H(), l = c(""), s = c(false), n = c(j(r.attr.name)), v = c(j(r.attr.desc));
  G(() => {
    r.attr.name && (o(n, r.attr.name, true), o(v, r.attr.desc, true));
  });
  async function h(x, A) {
    if (o(l, ""), e(n) !== r.attr.name && r.attrs.find((m) => m.name === e(n))) {
      o(l, u.common.nameExistsAlready, true);
      return;
    }
    let d = { name: e(n), desc: e(v) || void 0 }, t = await ce(x.action, d);
    t.error ? o(l, t.error.message, true) : (o(s, true), r.onSave(), setTimeout(() => {
      o(s, false);
    }, 2e3));
  }
  const p = Y(() => `/auth/v1/users/attr/${r.attr.name}`);
  ae(w, { get action() {
    return e(p);
  }, onSubmit: h, children: (x, A) => {
    var d = Se(), t = M(d);
    J(t, { autocomplete: "off", get label() {
      return u.attrs.name;
    }, get placeholder() {
      return u.attrs.name;
    }, width: "14.5rem", required: true, pattern: te, get value() {
      return e(n);
    }, set value(a) {
      o(n, a, true);
    } });
    var m = b(t, 2);
    J(m, { autocomplete: "off", get label() {
      return u.attrs.desc;
    }, get placeholder() {
      return u.attrs.desc;
    }, width: "14.5rem", pattern: re, get value() {
      return e(v);
    }, set value(a) {
      o(v, a, true);
    } });
    var S = b(m, 2), C = B(S);
    Z(C, { type: "submit", children: (a, y) => {
      Q();
      var g = K();
      $(() => E(g, P.common.save)), f(a, g);
    }, $$slots: { default: true } });
    var D = b(C, 2);
    {
      var L = (a) => {
        pe(a, {});
      };
      R(D, (a) => {
        e(s) && a(L);
      });
    }
    I(S);
    var _ = b(S, 2);
    {
      var i = (a) => {
        var y = Te(), g = B(y, true);
        I(y), $(() => E(g, e(l))), f(a, y);
      };
      R(_, (a) => {
        e(l) && a(i);
      });
    }
    f(x, d);
  }, $$slots: { default: true } }), W();
}
var Pe = T('<div class="err"> </div>'), Ce = T("<p> </p> <!> <!>", 1);
function Ne(w, r) {
  V(r, true);
  let P = U(), u = H(), l = c("");
  async function s() {
    o(l, "");
    let d = await fe(`/auth/v1/users/attr/${r.attr.name}`);
    d.error ? o(l, d.error.message, true) : r.onSave();
  }
  var n = Ce(), v = M(n), h = B(v, true);
  I(v);
  var p = b(v, 2);
  Z(p, { level: -1, onclick: s, children: (d, t) => {
    Q();
    var m = K();
    $(() => E(m, P.common.delete)), f(d, m);
  }, $$slots: { default: true } });
  var x = b(p, 2);
  {
    var A = (d) => {
      var t = Pe(), m = B(t, true);
      I(t), $(() => E(m, e(l))), f(d, t);
    };
    R(x, (d) => {
      e(l) && d(A);
    });
  }
  $(() => E(h, u.attrs.delete1)), f(w, n), W();
}
var Fe = T('<div class="flex"><!></div> <!>', 1);
function Oe(w, r) {
  V(r, true);
  let P = U(), u = H();
  const l = [u.nav.config, P.common.delete];
  let s = c(j(l[0])), n = c(void 0);
  G(() => {
    r.attr.name && requestAnimationFrame(() => {
      var _a;
      (_a = e(n)) == null ? void 0 : _a();
    });
  });
  var v = Fe(), h = M(v), p = B(h);
  he(p, { tabs: l, get selected() {
    return e(s);
  }, set selected(t) {
    o(s, t, true);
  }, get focusFirst() {
    return e(n);
  }, set focusFirst(t) {
    o(n, t, true);
  } }), I(h);
  var x = b(h, 2);
  {
    var A = (t) => {
      we(t, { get attr() {
        return r.attr;
      }, get attrs() {
        return r.attrs;
      }, get onSave() {
        return r.onSave;
      } });
    }, d = (t, m) => {
      {
        var S = (C) => {
          Ne(C, { get attr() {
            return r.attr;
          }, get onSave() {
            return r.onSave;
          } });
        };
        R(t, (C) => {
          e(s) === P.common.delete && C(S);
        }, m);
      }
    };
    R(x, (t) => {
      e(s) === u.nav.config ? t(A) : t(d, false);
    });
  }
  f(w, v), W();
}
var $e = T("<div></div> <!>", 1), Ee = T("<!> <!>", 1), Be = T('<div class="err"> </div>'), Ie = T('<!> <div id="groups"><!></div>', 1), ke = T("<!> <!>", 1);
function Me(w, r) {
  V(r, true);
  let P = H(), u = c(void 0), l = c(""), s = c(j([])), n = c(j([])), v = c(void 0), h = _e("an");
  const p = [P.common.name];
  let x = c(j(p[0])), A = c("");
  const d = [P.common.name];
  oe(() => {
    t();
  }), G(() => {
    o(v, e(s).find((i) => i.name === h.get()), true);
  }), G(() => {
    let i = e(A).toLowerCase();
    i ? e(x) === p[0] && o(n, e(s).filter((a) => {
      var _a;
      return (_a = a.name) == null ? void 0 : _a.toLowerCase().includes(i);
    }), true) : o(n, e(s), true);
  });
  async function t() {
    var _a;
    let i = await ge("/auth/v1/users/attr");
    i.body ? o(s, i.body.values, true) : o(l, ((_a = i.error) == null ? void 0 : _a.message) || "Error", true);
  }
  function m(i, a) {
    let y = a === "up";
    i === d[0] && e(s).sort((g, F) => y ? g.name.localeCompare(F.name) : F.name.localeCompare(g.name));
  }
  function S() {
    t();
  }
  async function C(i) {
    var _a;
    (_a = e(u)) == null ? void 0 : _a(), await t(), h.set(i);
  }
  var D = ke(), L = M(D);
  de(L, { paddingTop: "2.1rem", buttonTilesAriaControls: "groups", width: "min(20rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (a) => {
    var y = $e(), g = M(y);
    ne(g, "", {}, { height: ".5rem" });
    var F = b(g, 2);
    se(F, 17, () => e(n), (k) => k.name, (k, q, N, O) => {
      const z = Y(() => h.get() === e(q).name);
      ue(k, { onclick: () => h.set(e(q).name), get selected() {
        return e(z);
      }, children: (X, Re) => {
        Q();
        var ee = K();
        $(() => E(ee, e(q).name)), f(X, ee);
      } });
    }), f(a, y);
  }, children: (a, y) => {
    var g = Ee(), F = M(g);
    const k = Y(() => e(s).length === 0 ? 1 : 2);
    ie(F, { get level() {
      return e(k);
    }, alignRight: true, get closeModal() {
      return e(u);
    }, set closeModal(N) {
      o(u, N, true);
    }, children: (N, O) => {
      ye(N, { onSave: C, get attrs() {
        return e(s);
      } });
    }, $$slots: { default: true } });
    var q = b(F, 2);
    me(q, { searchOptions: p, orderOptions: d, onChangeOrder: m, searchWidth: "min(19.5rem, 100dvw - .5rem)", get searchOption() {
      return e(x);
    }, set searchOption(N) {
      o(x, N, true);
    }, get value() {
      return e(A);
    }, set value(N) {
      o(A, N, true);
    } }), f(a, g);
  }, $$slots: { buttonTiles: true, default: true } });
  var _ = b(L, 2);
  le(_, { children: (i, a) => {
    var y = Ie(), g = M(y);
    {
      var F = (O) => {
        var z = Be(), X = B(z, true);
        I(z), $(() => E(X, e(l))), f(O, z);
      };
      R(g, (O) => {
        e(l) && O(F);
      });
    }
    var k = b(g, 2), q = B(k);
    {
      var N = (O) => {
        Oe(O, { get attr() {
          return e(v);
        }, get attrs() {
          return e(s);
        }, onSave: S });
      };
      R(q, (O) => {
        e(v) && O(N);
      });
    }
    I(k), f(i, y);
  } }), f(w, D), W();
}
function st(w) {
  Me(w, {});
}
export {
  st as component
};
