import { t as T, a as b, d as q, n as $ } from "./DLBGyKVC.js";
import { p as ee, aa as G, s as F, c as y, t as j, a as te, f as X, j as t, k as z, r as w, l as m, a9 as ae } from "./CmQi0fbH.js";
import { e as ne, d as ie, s as re } from "./BjaYyaa_.js";
import { i as V } from "./C6bK2EJJ.js";
import { e as le, i as se } from "./YQCw2eEa.js";
import { s as l, r as ce, a as de } from "./DOlUUCkJ.js";
import { b as ve } from "./B3JAWFL5.js";
import { p as K } from "./B_ggA-0N.js";
import { b as oe } from "./BLvsVIZg.js";
import { p as s } from "./DNJm3-SG.js";
import { s as Z } from "./71gCR-8F.js";
import { B as J } from "./DPLO-ozG.js";
import { g as Q } from "./B21bTIl7.js";
import { u as ue } from "./CZf6fJph.js";
import { u as he } from "./DGTOa5g8.js";
var ge = T('<div><!> <div class="popover svelte-15eoy2j" popover="auto"><div class="inner fade-in svelte-15eoy2j"><!></div></div></div>');
function me(h, e) {
  ee(e, true);
  let a = s(e, "ref", 15), c = s(e, "roleButton", 3, "button"), v = s(e, "offsetLeft", 3, "0px"), o = s(e, "offsetTop", 3, "0px"), C = s(e, "close", 15);
  const x = Q(), S = Q();
  let n = z(void 0), D = z(false);
  G(() => {
    C(P);
  });
  function p(i) {
    if (i.stopPropagation(), a() && t(n)) if (e.absolute) t(n).style.top = o(), t(n).style.left = v();
    else {
      let d = a().getBoundingClientRect();
      t(n).style.top = `calc(${d.bottom + window.scrollY}px + ${o()})`, t(n).style.left = `calc(${d.left + window.scrollX}px + ${v()})`;
    }
    else console.warn("button and popover ref missing");
  }
  function P() {
    var _a;
    (_a = t(n)) == null ? void 0 : _a.hidePopover();
  }
  function H(i) {
    var _a;
    let d = i.newState;
    m(D, d === "open"), (_a = e.onToggle) == null ? void 0 : _a.call(e, d);
  }
  var A = ge(), O = y(A);
  J(O, { get role() {
    return c();
  }, id: x, ariaControls: S, popovertarget: S, onclick: p, get invisible() {
    return e.btnInvisible;
  }, get isDisabled() {
    return e.btnDisabled;
  }, get onLeft() {
    return e.onLeft;
  }, get onRight() {
    return e.onRight;
  }, get onUp() {
    return e.onUp;
  }, get onDown() {
    return e.onDown;
  }, get ref() {
    return a();
  }, set ref(i) {
    a(i);
  }, children: (i, d) => {
    var k = q(), M = X(k);
    Z(M, () => e.button), b(i, k);
  }, $$slots: { default: true } });
  var L = F(O, 2);
  l(L, "id", S), l(L, "aria-labelledby", x);
  var E = y(L), _ = y(E);
  {
    var r = (i) => {
      var d = q(), k = X(d);
      {
        var M = (N) => {
          var R = q(), f = X(R);
          Z(f, () => e.children), b(N, R);
        };
        V(k, (N) => {
          t(D) && N(M);
        });
      }
      b(i, d);
    }, I = (i) => {
      var d = q(), k = X(d);
      Z(k, () => e.children), b(i, d);
    };
    V(_, (i) => {
      e.lazy ? i(r) : i(I, false);
    });
  }
  w(E), w(L), oe(L, (i) => m(n, i), () => t(n)), w(A), j(() => l(L, "aria-label", e.ariaLabel)), ne("toggle", L, H), b(h, A), te();
}
var be = $('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z"></path></svg>');
function _e(h, e) {
  let a = s(e, "opacity", 3, 0.9), c = s(e, "width", 3, "1.5rem"), v = s(e, "color", 3, "currentColor");
  var o = be();
  l(o, "stroke-width", 2), j(() => {
    l(o, "stroke", v()), l(o, "width", c()), l(o, "opacity", a());
  }), b(h, o);
}
var we = $('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M12 9.75L14.25 12m0 0l2.25 2.25M14.25 12l2.25-2.25M14.25 12L12 14.25m-2.58 4.92l-6.375-6.375a1.125 1.125 0 010-1.59L9.42 4.83c.211-.211.498-.33.796-.33H19.5a2.25 2.25 0 012.25 2.25v10.5a2.25 2.25 0 01-2.25 2.25h-9.284c-.298 0-.585-.119-.796-.33z"></path></svg>');
function ke(h, e) {
  let a = s(e, "opacity", 3, 0.9), c = s(e, "width", 3, "1.5rem"), v = s(e, "color", 3, "currentColor");
  var o = we();
  l(o, "stroke-width", 2), j(() => {
    l(o, "stroke", v()), l(o, "width", c()), l(o, "opacity", a());
  }), b(h, o);
}
function ye(h, e, a, c) {
  var _a, _b, _c;
  switch (h.code) {
    case "Enter":
      e();
      break;
    case "Tab":
      (_a = a.onTab) == null ? void 0 : _a.call(a, c());
      break;
    case "ArrowUp":
      (_b = a.onUp) == null ? void 0 : _b.call(a, c());
      break;
    case "ArrowDown":
      (_c = a.onDown) == null ? void 0 : _c.call(a, c());
      break;
  }
}
var xe = T('<div class="options svelte-1wttsdc"><!></div>'), Le = (h) => h.stopPropagation(), pe = T("<option></option>"), Be = T('<datalist class="absolute svelte-1wttsdc"></datalist>'), De = T('<span class="backspace svelte-1wttsdc"><!></span>'), Pe = T('<div class="btnSearch svelte-1wttsdc"><!></div>'), Ie = T('<search class="flex container svelte-1wttsdc"><!> <input type="search" autocomplete="off" class="svelte-1wttsdc"> <!> <div class="relative"><div class="absolute btnDelete svelte-1wttsdc"><!></div></div> <!></search>');
function Te(h, e) {
  ee(e, true);
  let a = he(), c = ue(), v = s(e, "id", 19, Q), o = s(e, "value", 15, ""), C = s(e, "placeholder", 19, () => c.common.search), x = s(e, "option", 15), S = s(e, "focus", 15), n = s(e, "width", 3, "100%");
  const D = Q();
  let p = z(void 0), P = ae(() => e.datalist && e.datalist.length > 0 ? D : void 0);
  G(() => {
    S(A);
  });
  function H() {
    var _a;
    (_a = e.onSearch) == null ? void 0 : _a.call(e, o(), x());
  }
  function A() {
    var _a;
    (_a = t(p)) == null ? void 0 : _a.focus();
  }
  var O = Ie();
  let L;
  var E = y(O);
  {
    var _ = (f) => {
      var u = xe(), g = y(u);
      Ee(g, { get ariaLabel() {
        return c.common.searchOptions;
      }, get options() {
        return e.options;
      }, borderless: true, offsetLeft: "-.25rem", get value() {
        return x();
      }, set value(B) {
        x(B);
      } }), w(u), b(f, u);
    };
    V(E, (f) => {
      e.options && f(_);
    });
  }
  var r = F(E, 2);
  ce(r), r.__click = [Le], r.__keydown = [ye, H, e, o], oe(r, (f) => m(p, f), () => t(p));
  var I = F(r, 2);
  {
    var i = (f) => {
      var u = Be();
      l(u, "id", D), le(u, 21, () => e.datalist, se, (g, B, W, fe) => {
        var U = pe(), Y = {};
        j(() => {
          Y !== (Y = t(B)) && (U.value = (U.__value = t(B)) == null ? "" : t(B));
        }), b(g, U);
      }), w(u), b(f, u);
    };
    V(I, (f) => {
      e.datalist && f(i);
    });
  }
  var d = F(I, 2), k = y(d), M = y(k);
  J(M, { get ariaLabel() {
    return a.common.delete;
  }, invisible: true, onclick: () => o(""), children: (f, u) => {
    var g = De(), B = y(g);
    ke(B, { color: "hsl(var(--bg-high))" }), w(g), b(f, g);
  }, $$slots: { default: true } }), w(k), w(d);
  var N = F(d, 2);
  {
    var R = (f) => {
      var u = Pe(), g = y(u);
      J(g, { get ariaLabel() {
        return c.common.search;
      }, invisible: true, onclick: H, children: (B, W) => {
        _e(B, {});
      }, $$slots: { default: true } }), w(u), b(f, u);
    };
    V(N, (f) => {
      e.onSearch && f(R);
    });
  }
  w(O), j(() => {
    L = de(O, "", L, { border: e.borderless ? void 0 : "1px solid hsl(var(--bg-high))", width: n() }), l(r, "id", v()), l(r, "list", t(P)), l(r, "aria-label", e.ariaLabel || c.common.search), l(r, "pattern", e.pattern), l(r, "placeholder", C());
  }), ne("focus", r, () => {
    var _a;
    return (_a = e.onFocus) == null ? void 0 : _a.call(e);
  }), ve(r, o), b(h, O), te();
}
ie(["click", "keydown"]);
var Ce = $('<svg fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5"></path></svg>');
function Se(h, e) {
  let a = s(e, "opacity", 3, 0.9), c = s(e, "width", 3, "1.5rem"), v = s(e, "color", 3, "currentColor");
  var o = Ce();
  l(o, "stroke-width", 2), j(() => {
    l(o, "width", c()), l(o, "color", v()), l(o, "opacity", a());
  }), b(h, o);
}
function Oe(h, e, a, c, v) {
  let o = h.code;
  o === "ArrowDown" ? (h.preventDefault(), e() && m(a, t(a) + 1)) : o === "ArrowUp" ? (h.preventDefault(), e() && m(a, t(a) - 1)) : o === "Enter" && t(a) > -1 ? c(t(v)[t(a)]) : o === "Enter" && t(a) === -1 && t(v).length === 1 && c(t(v)[0]);
}
var Ue = T('<div class="btn svelte-5ktp6l"><div class="btnText svelte-5ktp6l"> </div> <!></div>'), je = T('<div class="optPopover svelte-5ktp6l"> </div>'), Ae = T('<div role="listbox" tabindex="0" class="popover svelte-5ktp6l"><!> <div class="popoverOptions svelte-5ktp6l"></div></div> <input type="hidden" aria-hidden="true">', 1);
function Ee(h, e) {
  ee(e, true);
  let a = s(e, "ref", 15), c = s(e, "options", 19, () => []), v = s(e, "value", 15), o = s(e, "borderless", 3, false), C = s(e, "withSearch", 3, false), x = z(void 0), S = z(void 0), n = z(K(C() ? -1 : 0)), D = z(void 0), p = z(""), P = ae(() => {
    if (!C()) return c();
    if (typeof v() == "string") return c().filter((r) => r.toLowerCase().includes(t(p).toLowerCase()));
    let _ = Number.parseInt(t(p)) || v();
    return c().filter((r) => r === _);
  }), H = false;
  G(() => {
    var _a;
    let _ = v();
    H ? (_a = e.onChange) == null ? void 0 : _a.call(e, _) : H = true;
  }), G(() => {
    var _a, _b;
    if (t(n) === -1 && ((_a = t(x)) == null ? void 0 : _a.scrollTo({ top: 0, behavior: "smooth" })), C()) {
      if (t(n) < 0 || t(n) > t(P).length - 1) {
        m(n, -1), (_b = t(D)) == null ? void 0 : _b();
        return;
      }
    } else t(n) < 0 ? m(n, t(P).length - 1) : t(n) > t(P).length - 1 && m(n, 0), A();
  });
  function A() {
    if (t(x)) {
      let _ = t(x).getElementsByTagName("button")[t(n)];
      _ == null ? void 0 : _.scrollIntoView({ behavior: "smooth", block: "center" }), _ == null ? void 0 : _.focus();
    } else console.error("refOptions is undefined");
  }
  function O(_) {
    var _a;
    _ === "open" && (C() ? (m(n, -1), (_a = t(D)) == null ? void 0 : _a()) : (m(n, K(c().findIndex((r) => r === v()) || 0)), A()));
  }
  function L() {
    return t(P).length > 0 ? true : (m(n, -1), false);
  }
  function E(_) {
    v(_), m(p, ""), setTimeout(() => {
      var _a;
      (_a = t(S)) == null ? void 0 : _a();
    }, 20);
  }
  me(h, { get ariaLabel() {
    return e.ariaLabel;
  }, roleButton: "combobox", btnInvisible: true, get offsetTop() {
    return e.offsetTop;
  }, get offsetLeft() {
    return e.offsetLeft;
  }, onToggle: O, get onLeft() {
    return e.onLeft;
  }, get onRight() {
    return e.onRight;
  }, get onUp() {
    return e.onUp;
  }, get onDown() {
    return e.onDown;
  }, get ref() {
    return a();
  }, set ref(r) {
    a(r);
  }, get close() {
    return t(S);
  }, set close(r) {
    m(S, K(r));
  }, button: (r) => {
    var I = Ue(), i = y(I), d = y(i, true);
    w(i);
    var k = F(i, 2);
    Se(k, { width: ".8rem" }), w(I), j(() => {
      l(I, "data-border", !o()), l(i, "title", e.ariaLabel), re(d, v());
    }), b(r, I);
  }, children: (r, I) => {
    var i = Ae(), d = X(i);
    d.__keydown = [Oe, L, n, E, P];
    let k;
    var M = y(d);
    {
      var N = (u) => {
        Te(u, { onFocus: () => m(n, -1), get value() {
          return t(p);
        }, set value(g) {
          m(p, K(g));
        }, get focus() {
          return t(D);
        }, set focus(g) {
          m(D, K(g));
        } });
      };
      V(M, (u) => {
        C() && u(N);
      });
    }
    var R = F(M, 2);
    le(R, 21, () => t(P), se, (u, g, B) => {
      J(u, { invisible: true, invisibleOutline: true, onclick: () => E(t(g)), children: (W, fe) => {
        var U = je(), Y = y(U, true);
        w(U), j(() => {
          l(U, "aria-selected", v() === t(g)), l(U, "data-focus", t(n) === B), re(Y, t(g));
        }), b(W, U);
      }, $$slots: { default: true } });
    }), w(R), oe(R, (u) => m(x, u), () => t(x)), w(d);
    var f = F(d, 2);
    ce(f), j(() => {
      k = de(d, "", k, { "max-height": e.maxHeight }), l(f, "name", e.name);
    }), ve(f, v), b(r, i);
  }, $$slots: { button: true, default: true } }), te();
}
ie(["keydown"]);
export {
  Ee as O,
  me as P,
  Te as S
};
