import { t as S, a as g, e as K } from "../chunks/CWf9OOFK.js";
import { p as V, ab as G, a as W, f as R, l as n, k as f, j as e, s as x, t as E, c as I, a8 as Q, r as k, aa as Y, a6 as oe } from "../chunks/nlANaGLT.js";
import { s as B } from "../chunks/BmMHVVX3.js";
import { i as q } from "../chunks/DOjUa9u5.js";
import { e as ne } from "../chunks/B6azywu7.js";
import { a as se } from "../chunks/CZv_AhHu.js";
import { p as a } from "../chunks/5u5qd9TD.js";
import { u as H } from "../chunks/Bb8ybDgy.js";
import { B as ie } from "../chunks/B-Kclwyq.js";
import { C as le } from "../chunks/BSE9YYho.js";
import { N as de } from "../chunks/B_NGI7Kx.js";
import { N as ue } from "../chunks/hmJwmp7r.js";
import { O as me } from "../chunks/Np7i_reI.js";
import { b as ve, d as ce, c as fe, f as ge } from "../chunks/BO1A6s0c.js";
import { u as _e } from "../chunks/DAXezkVN.js";
import { B as Z } from "../chunks/BEQMYyDu.js";
import { I as J } from "../chunks/6f3yten3.js";
import { d as te, e as re } from "../chunks/BRCxk8by.js";
import { F as ae } from "../chunks/CmL1R98Y.js";
import { u as U } from "../chunks/DwvS5LQk.js";
import { T as he } from "../chunks/D-oL4W7r.js";
import { I as pe } from "../chunks/DYRiKtW9.js";
var be = S('<div class="err"> </div>'), xe = S("<!> <!> <!> <!>", 1), Ae = S('<div class="container svelte-s1196z"><!></div>');
function ye(P, r) {
  V(r, true);
  let C = U(), u = H(), d = f(void 0), s = f(""), i = f(""), c = f("");
  G(() => {
    requestAnimationFrame(() => {
      var _a;
      (_a = e(d)) == null ? void 0 : _a.focus();
    });
  });
  async function p(y, m) {
    var _a;
    if (r.attrs.find((w) => w.name === e(i))) {
      n(s, a(u.common.nameExistsAlready));
      return;
    }
    n(s, "");
    let t = { name: e(i), desc: e(c) || void 0 }, v = await ve(y.action, t);
    v.body ? r.onSave(v.body.name) : n(s, a(((_a = v.error) == null ? void 0 : _a.message) || "Error"));
  }
  var b = Ae(), A = I(b);
  ae(A, { action: "/auth/v1/users/attr", onSubmit: p, children: (y, m) => {
    var t = xe(), v = R(t);
    J(v, { autocomplete: "off", get label() {
      return u.attrs.name;
    }, get placeholder() {
      return u.attrs.name;
    }, required: true, pattern: te, get ref() {
      return e(d);
    }, set ref(h) {
      n(d, a(h));
    }, get value() {
      return e(i);
    }, set value(h) {
      n(i, a(h));
    } });
    var w = x(v, 2);
    J(w, { autocomplete: "off", get label() {
      return u.attrs.desc;
    }, get placeholder() {
      return u.attrs.desc;
    }, pattern: re, get value() {
      return e(c);
    }, set value(h) {
      n(c, a(h));
    } });
    var N = x(w, 2);
    Z(N, { type: "submit", children: (h, l) => {
      Q();
      var o = K();
      E(() => B(o, C.common.save)), g(h, o);
    }, $$slots: { default: true } });
    var L = x(N, 2);
    {
      var j = (h) => {
        var l = be(), o = I(l, true);
        k(l), E(() => B(o, e(s))), g(h, l);
      };
      q(L, (h) => {
        e(s) && h(j);
      });
    }
    g(y, t);
  }, $$slots: { default: true } }), k(b), g(P, b), W();
}
var Te = S('<div class="err"> </div>'), Se = S('<!> <!> <div class="flex gap-05"><!> <!></div> <!>', 1);
function we(P, r) {
  V(r, true);
  let C = U(), u = H(), d = f(""), s = f(false), i = f(a(r.attr.name)), c = f(a(r.attr.desc));
  G(() => {
    r.attr.name && (n(i, a(r.attr.name)), n(c, a(r.attr.desc)));
  });
  async function p(A, y) {
    if (n(d, ""), e(i) !== r.attr.name && r.attrs.find((v) => v.name === e(i))) {
      n(d, a(u.common.nameExistsAlready));
      return;
    }
    let m = { name: e(i), desc: e(c) || void 0 }, t = await ce(A.action, m);
    t.error ? n(d, a(t.error.message)) : (n(s, true), r.onSave(), setTimeout(() => {
      n(s, false);
    }, 2e3));
  }
  const b = Y(() => `/auth/v1/users/attr/${r.attr.name}`);
  ae(P, { get action() {
    return e(b);
  }, onSubmit: p, children: (A, y) => {
    var m = Se(), t = R(m);
    J(t, { autocomplete: "off", get label() {
      return u.attrs.name;
    }, get placeholder() {
      return u.attrs.name;
    }, width: "14.5rem", required: true, pattern: te, get value() {
      return e(i);
    }, set value(o) {
      n(i, a(o));
    } });
    var v = x(t, 2);
    J(v, { autocomplete: "off", get label() {
      return u.attrs.desc;
    }, get placeholder() {
      return u.attrs.desc;
    }, width: "14.5rem", pattern: re, get value() {
      return e(c);
    }, set value(o) {
      n(c, a(o));
    } });
    var w = x(v, 2), N = I(w);
    Z(N, { type: "submit", children: (o, T) => {
      Q();
      var _ = K();
      E(() => B(_, C.common.save)), g(o, _);
    }, $$slots: { default: true } });
    var L = x(N, 2);
    {
      var j = (o) => {
        pe(o, {});
      };
      q(L, (o) => {
        e(s) && o(j);
      });
    }
    k(w);
    var h = x(w, 2);
    {
      var l = (o) => {
        var T = Te(), _ = I(T, true);
        k(T), E(() => B(_, e(d))), g(o, T);
      };
      q(h, (o) => {
        e(d) && o(l);
      });
    }
    g(A, m);
  }, $$slots: { default: true } }), W();
}
var Pe = S('<div class="err"> </div>'), Ce = S("<p> </p> <!> <!>", 1);
function Ne(P, r) {
  V(r, true);
  let C = U(), u = H(), d = f("");
  async function s() {
    n(d, "");
    let m = await fe(`/auth/v1/users/attr/${r.attr.name}`);
    m.error ? n(d, a(m.error.message)) : r.onSave();
  }
  var i = Ce(), c = R(i), p = I(c, true);
  k(c);
  var b = x(c, 2);
  Z(b, { level: -1, onclick: s, children: (m, t) => {
    Q();
    var v = K();
    E(() => B(v, C.common.delete)), g(m, v);
  }, $$slots: { default: true } });
  var A = x(b, 2);
  {
    var y = (m) => {
      var t = Pe(), v = I(t, true);
      k(t), E(() => B(v, e(d))), g(m, t);
    };
    q(A, (m) => {
      e(d) && m(y);
    });
  }
  E(() => B(p, u.attrs.delete1)), g(P, i), W();
}
var Fe = S('<div class="flex"><!></div> <!>', 1);
function Oe(P, r) {
  V(r, true);
  let C = U(), u = H();
  const d = [u.nav.config, C.common.delete];
  let s = f(a(d[0])), i = f(void 0);
  G(() => {
    r.attr.name && requestAnimationFrame(() => {
      var _a;
      (_a = e(i)) == null ? void 0 : _a();
    });
  });
  var c = Fe(), p = R(c), b = I(p);
  he(b, { tabs: d, get selected() {
    return e(s);
  }, set selected(t) {
    n(s, a(t));
  }, get focusFirst() {
    return e(i);
  }, set focusFirst(t) {
    n(i, a(t));
  } }), k(p);
  var A = x(p, 2);
  {
    var y = (t) => {
      we(t, { get attr() {
        return r.attr;
      }, get attrs() {
        return r.attrs;
      }, get onSave() {
        return r.onSave;
      } });
    }, m = (t, v) => {
      {
        var w = (N) => {
          Ne(N, { get attr() {
            return r.attr;
          }, get onSave() {
            return r.onSave;
          } });
        };
        q(t, (N) => {
          e(s) === C.common.delete && N(w);
        }, v);
      }
    };
    q(A, (t) => {
      e(s) === u.nav.config ? t(y) : t(m, false);
    });
  }
  g(P, c), W();
}
var $e = S("<div></div> <!>", 1), Ee = S("<!> <!>", 1), Be = S('<div class="err"> </div>'), Ie = S('<!> <div id="groups"><!></div>', 1), ke = S("<!> <!>", 1);
function Me(P, r) {
  V(r, true);
  let C = H(), u = f(void 0), d = f(""), s = f(a([])), i = f(a([])), c = f(void 0), p = _e("an");
  const b = [C.common.name];
  let A = f(a(b[0])), y = f("");
  const m = [C.common.name];
  oe(() => {
    t();
  }), G(() => {
    n(c, a(e(s).find((l) => l.name === p.get())));
  }), G(() => {
    let l = e(y).toLowerCase();
    l ? e(A) === b[0] && n(i, a(e(s).filter((o) => {
      var _a;
      return (_a = o.name) == null ? void 0 : _a.toLowerCase().includes(l);
    }))) : n(i, a(e(s)));
  });
  async function t() {
    var _a;
    let l = await ge("/auth/v1/users/attr");
    l.body ? n(s, a(l.body.values)) : n(d, a(((_a = l.error) == null ? void 0 : _a.message) || "Error"));
  }
  function v(l, o) {
    let T = o === "up";
    l === m[0] && e(s).sort((_, O) => T ? _.name.localeCompare(O.name) : O.name.localeCompare(_.name));
  }
  function w() {
    t();
  }
  async function N(l) {
    var _a;
    (_a = e(u)) == null ? void 0 : _a(), await t(), p.set(l);
  }
  var L = ke(), j = R(L);
  ue(j, { paddingTop: "2.1rem", buttonTilesAriaControls: "groups", width: "min(20rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (o) => {
    var T = $e(), _ = R(T);
    se(_, "", {}, { height: ".5rem" });
    var O = x(_, 2);
    ne(O, 17, () => e(i), (M) => M.name, (M, D, F, $) => {
      const z = Y(() => p.get() === e(D).name);
      de(M, { onclick: () => p.set(e(D).name), get selected() {
        return e(z);
      }, children: (X, Re) => {
        Q();
        var ee = K();
        E(() => B(ee, e(D).name)), g(X, ee);
      } });
    }), g(o, T);
  }, children: (o, T) => {
    var _ = Ee(), O = R(_);
    const M = Y(() => e(s).length === 0 ? 1 : 2);
    ie(O, { get level() {
      return e(M);
    }, alignRight: true, get closeModal() {
      return e(u);
    }, set closeModal(F) {
      n(u, a(F));
    }, children: (F, $) => {
      ye(F, { onSave: N, get attrs() {
        return e(s);
      } });
    }, $$slots: { default: true } });
    var D = x(O, 2);
    me(D, { searchOptions: b, orderOptions: m, onChangeOrder: v, searchWidth: "min(19.5rem, 100dvw - .5rem)", get searchOption() {
      return e(A);
    }, set searchOption(F) {
      n(A, a(F));
    }, get value() {
      return e(y);
    }, set value(F) {
      n(y, a(F));
    } }), g(o, _);
  }, $$slots: { buttonTiles: true, default: true } });
  var h = x(j, 2);
  le(h, { children: (l, o) => {
    var T = Ie(), _ = R(T);
    {
      var O = ($) => {
        var z = Be(), X = I(z, true);
        k(z), E(() => B(X, e(d))), g($, z);
      };
      q(_, ($) => {
        e(d) && $(O);
      });
    }
    var M = x(_, 2), D = I(M);
    {
      var F = ($) => {
        Oe($, { get attr() {
          return e(c);
        }, get attrs() {
          return e(s);
        }, onSave: w });
      };
      q(D, ($) => {
        e(c) && $(F);
      });
    }
    k(M), g(l, T);
  } }), g(P, L), W();
}
function st(P) {
  Me(P, {});
}
export {
  st as component
};
