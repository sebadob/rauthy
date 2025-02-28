import { t as S, a as g, e as K, d as oe } from "../chunks/BH6NCLk-.js";
import { p as V, aa as G, a as W, f as B, l as n, k as f, j as e, s as x, t as I, c as M, a7 as Q, r as R, a9 as Y, a5 as ne } from "../chunks/CvlvO1XB.js";
import { s as k } from "../chunks/CTI4QPiR.js";
import { i as D } from "../chunks/BUO_AUgz.js";
import { e as se } from "../chunks/BpWRzPRQ.js";
import { B as Z, a as ie } from "../chunks/DMkkW5Nn.js";
import { p as a } from "../chunks/Wh68IIk2.js";
import { u as H } from "../chunks/D8mHI_K9.js";
import { B as le } from "../chunks/dWvWuq1E.js";
import { C as de } from "../chunks/BnPoFdx3.js";
import { N as ue } from "../chunks/D-L0o8jR.js";
import { N as me } from "../chunks/DpcgKHU5.js";
import { O as ve } from "../chunks/CT2gdhvj.js";
import { b as ce, d as fe, c as ge, f as _e } from "../chunks/CBGoQiUs.js";
import { u as he } from "../chunks/DxHSykar.js";
import { I as J } from "../chunks/BtKnbFDH.js";
import { d as te, e as re } from "../chunks/BRCxk8by.js";
import { F as ae } from "../chunks/BS0DIDHc.js";
import { u as U } from "../chunks/BQ1-pLIs.js";
import { T as pe } from "../chunks/_OE2Cq0B.js";
import { I as be } from "../chunks/Nks81rMs.js";
var xe = S('<div class="err"> </div>'), Ae = S("<!> <!> <!> <!>", 1), ye = S('<div class="container svelte-s1196z"><!></div>');
function Te(P, r) {
  V(r, true);
  let C = U(), m = H(), d = f(void 0), s = f(""), i = f(""), c = f("");
  G(() => {
    requestAnimationFrame(() => {
      var _a;
      (_a = e(d)) == null ? void 0 : _a.focus();
    });
  });
  async function p(y, v) {
    var _a;
    if (r.attrs.find((w) => w.name === e(i))) {
      n(s, a(m.common.nameExistsAlready));
      return;
    }
    n(s, "");
    let t = { name: e(i), desc: e(c) || void 0 }, u = await ce(y.action, t);
    u.body ? r.onSave(u.body.name) : n(s, a(((_a = u.error) == null ? void 0 : _a.message) || "Error"));
  }
  var b = ye(), A = M(b);
  ae(A, { action: "/auth/v1/users/attr", onSubmit: p, children: (y, v) => {
    var t = Ae(), u = B(t);
    J(u, { autocomplete: "off", get label() {
      return m.attrs.name;
    }, get placeholder() {
      return m.attrs.name;
    }, required: true, pattern: te, get ref() {
      return e(d);
    }, set ref(h) {
      n(d, a(h));
    }, get value() {
      return e(i);
    }, set value(h) {
      n(i, a(h));
    } });
    var w = x(u, 2);
    J(w, { autocomplete: "off", get label() {
      return m.attrs.desc;
    }, get placeholder() {
      return m.attrs.desc;
    }, pattern: re, get value() {
      return e(c);
    }, set value(h) {
      n(c, a(h));
    } });
    var O = x(w, 2);
    Z(O, { type: "submit", children: (h, l) => {
      Q();
      var o = K();
      I(() => k(o, C.common.save)), g(h, o);
    }, $$slots: { default: true } });
    var N = x(O, 2);
    {
      var j = (h) => {
        var l = xe(), o = M(l, true);
        R(l), I(() => k(o, e(s))), g(h, l);
      };
      D(N, (h) => {
        e(s) && h(j);
      });
    }
    g(y, t);
  }, $$slots: { default: true } }), R(b), g(P, b), W();
}
var Se = S('<div class="err"> </div>'), we = S('<!> <!> <div class="flex gap-05"><!> <!></div> <!>', 1);
function Pe(P, r) {
  V(r, true);
  let C = U(), m = H(), d = f(""), s = f(false), i = f(a(r.attr.name)), c = f(a(r.attr.desc));
  G(() => {
    r.attr.name && (n(i, a(r.attr.name)), n(c, a(r.attr.desc)));
  });
  async function p(A, y) {
    if (n(d, ""), e(i) !== r.attr.name && r.attrs.find((u) => u.name === e(i))) {
      n(d, a(m.common.nameExistsAlready));
      return;
    }
    let v = { name: e(i), desc: e(c) || void 0 }, t = await fe(A.action, v);
    t.error ? n(d, a(t.error.message)) : (n(s, true), r.onSave(), setTimeout(() => {
      n(s, false);
    }, 2e3));
  }
  const b = Y(() => `/auth/v1/users/attr/${r.attr.name}`);
  ae(P, { get action() {
    return e(b);
  }, onSubmit: p, children: (A, y) => {
    var v = we(), t = B(v);
    J(t, { autocomplete: "off", get label() {
      return m.attrs.name;
    }, get placeholder() {
      return m.attrs.name;
    }, width: "14.5rem", required: true, pattern: te, get value() {
      return e(i);
    }, set value(o) {
      n(i, a(o));
    } });
    var u = x(t, 2);
    J(u, { autocomplete: "off", get label() {
      return m.attrs.desc;
    }, get placeholder() {
      return m.attrs.desc;
    }, width: "14.5rem", pattern: re, get value() {
      return e(c);
    }, set value(o) {
      n(c, a(o));
    } });
    var w = x(u, 2), O = M(w);
    Z(O, { type: "submit", children: (o, T) => {
      Q();
      var _ = K();
      I(() => k(_, C.common.save)), g(o, _);
    }, $$slots: { default: true } });
    var N = x(O, 2);
    {
      var j = (o) => {
        be(o, {});
      };
      D(N, (o) => {
        e(s) && o(j);
      });
    }
    R(w);
    var h = x(w, 2);
    {
      var l = (o) => {
        var T = Se(), _ = M(T, true);
        R(T), I(() => k(_, e(d))), g(o, T);
      };
      D(h, (o) => {
        e(d) && o(l);
      });
    }
    g(A, v);
  }, $$slots: { default: true } }), W();
}
var Ce = S('<div class="err"> </div>'), Ne = S("<p> </p> <!> <!>", 1);
function Fe(P, r) {
  V(r, true);
  let C = U(), m = H(), d = f("");
  async function s() {
    n(d, "");
    let v = await ge(`/auth/v1/users/attr/${r.attr.name}`);
    v.error ? n(d, a(v.error.message)) : r.onSave();
  }
  var i = Ne(), c = B(i), p = M(c, true);
  R(c);
  var b = x(c, 2);
  Z(b, { level: -1, onclick: s, children: (v, t) => {
    Q();
    var u = K();
    I(() => k(u, C.common.delete)), g(v, u);
  }, $$slots: { default: true } });
  var A = x(b, 2);
  {
    var y = (v) => {
      var t = Ce(), u = M(t, true);
      R(t), I(() => k(u, e(d))), g(v, t);
    };
    D(A, (v) => {
      e(d) && v(y);
    });
  }
  I(() => k(p, m.attrs.delete1)), g(P, i), W();
}
var Oe = S('<div class="flex"><!></div> <!>', 1);
function $e(P, r) {
  V(r, true);
  let C = U(), m = H();
  const d = [m.nav.config, C.common.delete];
  let s = f(a(d[0])), i = f(void 0);
  G(() => {
    r.attr.name && requestAnimationFrame(() => {
      var _a;
      (_a = e(i)) == null ? void 0 : _a();
    });
  });
  var c = Oe(), p = B(c), b = M(p);
  pe(b, { tabs: d, get selected() {
    return e(s);
  }, set selected(t) {
    n(s, a(t));
  }, get focusFirst() {
    return e(i);
  }, set focusFirst(t) {
    n(i, a(t));
  } }), R(p);
  var A = x(p, 2);
  {
    var y = (t) => {
      Pe(t, { get attr() {
        return r.attr;
      }, get attrs() {
        return r.attrs;
      }, get onSave() {
        return r.onSave;
      } });
    }, v = (t) => {
      var u = oe(), w = B(u);
      {
        var O = (N) => {
          Fe(N, { get attr() {
            return r.attr;
          }, get onSave() {
            return r.onSave;
          } });
        };
        D(w, (N) => {
          e(s) === C.common.delete && N(O);
        }, true);
      }
      g(t, u);
    };
    D(A, (t) => {
      e(s) === m.nav.config ? t(y) : t(v, false);
    });
  }
  g(P, c), W();
}
var Ee = S("<div></div> <!>", 1), Be = S("<!> <!>", 1), Ie = S('<div class="err"> </div>'), ke = S('<!> <div id="groups"><!></div>', 1), Me = S("<!> <!>", 1);
function Re(P, r) {
  V(r, true);
  let C = H(), m = f(void 0), d = f(""), s = f(a([])), i = f(a([])), c = f(void 0), p = he("an");
  const b = [C.common.name];
  let A = f(a(b[0])), y = f("");
  const v = [C.common.name];
  ne(() => {
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
    let l = await _e("/auth/v1/users/attr");
    l.body ? n(s, a(l.body.values)) : n(d, a(((_a = l.error) == null ? void 0 : _a.message) || "Error"));
  }
  function u(l, o) {
    let T = o === "up";
    l === v[0] && e(s).sort((_, $) => T ? _.name.localeCompare($.name) : $.name.localeCompare(_.name));
  }
  function w() {
    t();
  }
  async function O(l) {
    var _a;
    (_a = e(m)) == null ? void 0 : _a(), await t(), p.set(l);
  }
  var N = Me(), j = B(N);
  me(j, { paddingTop: "2.1rem", buttonTilesAriaControls: "groups", width: "min(20rem, 100dvw)", thresholdNavSub: 700, buttonTiles: (o) => {
    var T = Ee(), _ = B(T);
    ie(_, "height", ".5rem");
    var $ = x(_, 2);
    se($, 17, () => e(i), (q) => q.name, (q, L, F, E) => {
      const z = Y(() => p.get() === e(L).name);
      ue(q, { onclick: () => p.set(e(L).name), get selected() {
        return e(z);
      }, children: (X, qe) => {
        Q();
        var ee = K();
        I(() => k(ee, e(L).name)), g(X, ee);
      } });
    }), g(o, T);
  }, children: (o, T) => {
    var _ = Be(), $ = B(_);
    const q = Y(() => e(s).length === 0 ? 1 : 2);
    le($, { get level() {
      return e(q);
    }, alignRight: true, get closeModal() {
      return e(m);
    }, set closeModal(F) {
      n(m, a(F));
    }, children: (F, E) => {
      Te(F, { onSave: O, get attrs() {
        return e(s);
      } });
    }, $$slots: { default: true } });
    var L = x($, 2);
    ve(L, { searchOptions: b, orderOptions: v, onChangeOrder: u, searchWidth: "min(19.5rem, 100dvw - .5rem)", get searchOption() {
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
  de(h, { children: (l, o) => {
    var T = ke(), _ = B(T);
    {
      var $ = (E) => {
        var z = Ie(), X = M(z, true);
        R(z), I(() => k(X, e(d))), g(E, z);
      };
      D(_, (E) => {
        e(d) && E($);
      });
    }
    var q = x(_, 2), L = M(q);
    {
      var F = (E) => {
        $e(E, { get attr() {
          return e(c);
        }, get attrs() {
          return e(s);
        }, onSave: w });
      };
      D(L, (E) => {
        e(c) && E(F);
      });
    }
    R(q), g(l, T);
  } }), g(P, N), W();
}
function st(P) {
  Re(P, {});
}
export {
  st as component
};
