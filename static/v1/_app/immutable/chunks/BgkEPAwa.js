import { t as T, a as b, d as Y, n as Z } from "./BxmJRzoY.js";
import { p as $, j as z, a5 as q, s as F, c as x, t as j, a as ee, f as K, k as t, r as w, l as m, a4 as re, a1 as ue } from "./w0HvPX0p.js";
import { e as ae, d as ne, s as oe } from "./BzP2S3Z_.js";
import { i as V } from "./iO9_dPNE.js";
import { e as ie, i as le } from "./S81raU5Y.js";
import { s as n, r as se, a as de } from "./BdbQ6g_y.js";
import { b as ce } from "./Cxw7xmE1.js";
import { b as te } from "./Dgjj26O5.js";
import { p as s } from "./C6GSeq7M.js";
import { s as W } from "./DM69BKKN.js";
import { B as G } from "./C4AV2CoD.js";
import { g as J } from "./B21bTIl7.js";
import { u as fe } from "./D12OFlGX.js";
import { u as he } from "./N6FgGI8m.js";
var ge = T('<div><!> <div class="popover svelte-15eoy2j" popover="auto"><div class="inner fade-in svelte-15eoy2j"><!></div></div></div>');
function me(h, e) {
  $(e, true);
  let a = s(e, "ref", 15), d = s(e, "roleButton", 3, "button"), v = s(e, "offsetLeft", 3, "0px"), o = s(e, "offsetTop", 3, "0px"), C = s(e, "close", 15);
  const L = J(), S = J();
  let i = z(void 0), p = z(false);
  q(() => {
    C(P);
  });
  function B(l) {
    if (l.stopPropagation(), a() && t(i)) if (e.absolute) t(i).style.top = o(), t(i).style.left = v();
    else {
      let c = a().getBoundingClientRect();
      t(i).style.top = `calc(${c.bottom + window.scrollY}px + ${o()})`, t(i).style.left = `calc(${c.left + window.scrollX}px + ${v()})`;
    }
    else console.warn("button and popover ref missing");
  }
  function P() {
    var _a;
    (_a = t(i)) == null ? void 0 : _a.hidePopover();
  }
  function H(l) {
    var _a;
    let c = l.newState;
    m(p, c === "open"), (_a = e.onToggle) == null ? void 0 : _a.call(e, c);
  }
  var A = ge(), O = x(A);
  G(O, { get role() {
    return d();
  }, id: L, get ariaLabel() {
    return e.ariaLabel;
  }, ariaControls: S, popovertarget: S, onclick: B, get invisible() {
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
  }, set ref(l) {
    a(l);
  }, children: (l, c) => {
    var k = Y(), M = K(k);
    W(M, () => e.button), b(l, k);
  }, $$slots: { default: true } });
  var y = F(O, 2);
  n(y, "id", S), n(y, "aria-labelledby", L);
  var E = x(y), _ = x(E);
  {
    var r = (l) => {
      var c = Y(), k = K(c);
      {
        var M = (N) => {
          var R = Y(), u = K(R);
          W(u, () => e.children), b(N, R);
        };
        V(k, (N) => {
          t(p) && N(M);
        });
      }
      b(l, c);
    }, I = (l) => {
      var c = Y(), k = K(c);
      W(k, () => e.children), b(l, c);
    };
    V(_, (l) => {
      e.lazy ? l(r) : l(I, false);
    });
  }
  w(E), w(y), te(y, (l) => m(i, l), () => t(i)), w(A), j(() => {
    n(y, "aria-label", e.ariaLabel), n(y, "aria-expanded", t(p));
  }), ae("toggle", y, H), b(h, A), ee();
}
var be = Z('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z"></path></svg>');
function _e(h, e) {
  let a = s(e, "opacity", 3, 0.9), d = s(e, "width", 3, "1.5rem"), v = s(e, "color", 3, "currentColor");
  var o = be();
  n(o, "stroke-width", 2), j(() => {
    n(o, "stroke", v()), n(o, "width", d()), n(o, "opacity", a());
  }), b(h, o);
}
var we = Z('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M12 9.75L14.25 12m0 0l2.25 2.25M14.25 12l2.25-2.25M14.25 12L12 14.25m-2.58 4.92l-6.375-6.375a1.125 1.125 0 010-1.59L9.42 4.83c.211-.211.498-.33.796-.33H19.5a2.25 2.25 0 012.25 2.25v10.5a2.25 2.25 0 01-2.25 2.25h-9.284c-.298 0-.585-.119-.796-.33z"></path></svg>');
function ke(h, e) {
  let a = s(e, "opacity", 3, 0.9), d = s(e, "width", 3, "1.5rem"), v = s(e, "color", 3, "currentColor");
  var o = we();
  n(o, "stroke-width", 2), j(() => {
    n(o, "stroke", v()), n(o, "width", d()), n(o, "opacity", a());
  }), b(h, o);
}
function ye(h, e, a, d) {
  var _a, _b, _c;
  switch (h.code) {
    case "Enter":
      e();
      break;
    case "Tab":
      (_a = a.onTab) == null ? void 0 : _a.call(a, d());
      break;
    case "ArrowUp":
      (_b = a.onUp) == null ? void 0 : _b.call(a, d());
      break;
    case "ArrowDown":
      (_c = a.onDown) == null ? void 0 : _c.call(a, d());
      break;
  }
}
var xe = T('<div class="options svelte-1wttsdc"><!></div>'), Le = (h) => h.stopPropagation(), pe = T("<option></option>"), Be = T('<datalist class="absolute svelte-1wttsdc"></datalist>'), De = T('<span class="backspace svelte-1wttsdc"><!></span>'), Pe = T('<div class="btnSearch svelte-1wttsdc"><!></div>'), Ie = T('<search class="flex container svelte-1wttsdc"><!> <input type="search" autocomplete="off" class="svelte-1wttsdc"> <!> <div class="relative"><div class="absolute btnDelete svelte-1wttsdc"><!></div></div> <!></search>');
function Te(h, e) {
  $(e, true);
  let a = he(), d = fe(), v = s(e, "id", 19, J), o = s(e, "value", 15, ""), C = s(e, "placeholder", 19, () => d.common.search), L = s(e, "option", 15), S = s(e, "focus", 15), i = s(e, "width", 3, "100%");
  const p = J();
  let B = z(void 0), P = re(() => e.datalist && e.datalist.length > 0 ? p : void 0);
  q(() => {
    S(A);
  });
  function H() {
    var _a;
    (_a = e.onSearch) == null ? void 0 : _a.call(e, o(), L());
  }
  function A() {
    var _a;
    (_a = t(B)) == null ? void 0 : _a.focus();
  }
  var O = Ie();
  let y;
  var E = x(O);
  {
    var _ = (u) => {
      var f = xe(), g = x(f);
      Ee(g, { get ariaLabel() {
        return d.common.searchOptions;
      }, get options() {
        return e.options;
      }, borderless: true, offsetLeft: "-.25rem", get value() {
        return L();
      }, set value(D) {
        L(D);
      } }), w(f), b(u, f);
    };
    V(E, (u) => {
      e.options && u(_);
    });
  }
  var r = F(E, 2);
  se(r), r.__click = [Le], r.__keydown = [ye, H, e, o], te(r, (u) => m(B, u), () => t(B));
  var I = F(r, 2);
  {
    var l = (u) => {
      var f = Be();
      n(f, "id", p), ie(f, 21, () => e.datalist, le, (g, D, Q, ve) => {
        var U = pe(), X = {};
        j(() => {
          X !== (X = t(D)) && (U.value = (U.__value = t(D)) == null ? "" : t(D));
        }), b(g, U);
      }), w(f), b(u, f);
    };
    V(I, (u) => {
      e.datalist && u(l);
    });
  }
  var c = F(I, 2), k = x(c), M = x(k);
  G(M, { get ariaLabel() {
    return a.common.delete;
  }, invisible: true, onclick: () => o(""), children: (u, f) => {
    var g = De(), D = x(g);
    ke(D, { color: "hsl(var(--bg-high))" }), w(g), b(u, g);
  }, $$slots: { default: true } }), w(k), w(c);
  var N = F(c, 2);
  {
    var R = (u) => {
      var f = Pe(), g = x(f);
      G(g, { get ariaLabel() {
        return d.common.search;
      }, invisible: true, onclick: H, children: (D, Q) => {
        _e(D, {});
      }, $$slots: { default: true } }), w(f), b(u, f);
    };
    V(N, (u) => {
      e.onSearch && u(R);
    });
  }
  w(O), j(() => {
    y = de(O, "", y, { border: e.borderless ? void 0 : "1px solid hsl(var(--bg-high))", width: i() }), n(r, "id", v()), n(r, "list", t(P)), n(r, "aria-label", e.ariaLabel || d.common.search), n(r, "pattern", e.pattern), n(r, "placeholder", C());
  }), ae("focus", r, () => {
    var _a;
    return (_a = e.onFocus) == null ? void 0 : _a.call(e);
  }), ce(r, o), b(h, O), ee();
}
ne(["click", "keydown"]);
var Ce = Z('<svg fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5"></path></svg>');
function Se(h, e) {
  let a = s(e, "opacity", 3, 0.9), d = s(e, "width", 3, "1.5rem"), v = s(e, "color", 3, "currentColor");
  var o = Ce();
  n(o, "stroke-width", 2), j(() => {
    n(o, "width", d()), n(o, "color", v()), n(o, "opacity", a());
  }), b(h, o);
}
function Oe(h, e, a, d, v) {
  let o = h.code;
  o === "ArrowDown" ? (h.preventDefault(), e() && m(a, t(a) + 1)) : o === "ArrowUp" ? (h.preventDefault(), e() && m(a, t(a) - 1)) : o === "Enter" && t(a) > -1 ? d(t(v)[t(a)]) : o === "Enter" && t(a) === -1 && t(v).length === 1 && d(t(v)[0]);
}
var Ue = T('<div class="btn svelte-5ktp6l"><div class="btnText svelte-5ktp6l"> </div> <!></div>'), je = T('<div class="optPopover svelte-5ktp6l"> </div>'), Ae = T('<div role="listbox" tabindex="0" class="popover svelte-5ktp6l"><!> <div class="popoverOptions svelte-5ktp6l"></div></div> <input type="hidden" aria-hidden="true">', 1);
function Ee(h, e) {
  $(e, true);
  let a = s(e, "ref", 15), d = s(e, "options", 19, () => []), v = s(e, "value", 15), o = s(e, "borderless", 3, false), C = s(e, "withSearch", 3, false), L = z(void 0), S = z(void 0), i = z(ue(C() ? -1 : 0)), p = z(void 0), B = z(""), P = re(() => {
    if (!C()) return d();
    if (typeof v() == "string") return d().filter((r) => r.toLowerCase().includes(t(B).toLowerCase()));
    let _ = Number.parseInt(t(B)) || v();
    return d().filter((r) => r === _);
  }), H = false;
  q(() => {
    var _a;
    let _ = v();
    H ? (_a = e.onChange) == null ? void 0 : _a.call(e, _) : H = true;
  }), q(() => {
    var _a, _b;
    if (t(i) === -1 && ((_a = t(L)) == null ? void 0 : _a.scrollTo({ top: 0, behavior: "smooth" })), C()) {
      if (t(i) < 0 || t(i) > t(P).length - 1) {
        m(i, -1), (_b = t(p)) == null ? void 0 : _b();
        return;
      }
    } else t(i) < 0 ? m(i, t(P).length - 1) : t(i) > t(P).length - 1 && m(i, 0), A();
  });
  function A() {
    if (t(L)) {
      let _ = t(L).getElementsByTagName("button")[t(i)];
      _ == null ? void 0 : _.scrollIntoView({ behavior: "smooth", block: "center" }), _ == null ? void 0 : _.focus();
    } else console.error("refOptions is undefined");
  }
  function O(_) {
    var _a;
    _ === "open" && (C() ? (m(i, -1), (_a = t(p)) == null ? void 0 : _a()) : (m(i, d().findIndex((r) => r === v()) || 0, true), A()));
  }
  function y() {
    return t(P).length > 0 ? true : (m(i, -1), false);
  }
  function E(_) {
    v(_), m(B, ""), setTimeout(() => {
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
    m(S, r, true);
  }, button: (r) => {
    var I = Ue(), l = x(I), c = x(l, true);
    w(l);
    var k = F(l, 2);
    Se(k, { width: ".8rem" }), w(I), j(() => {
      n(I, "data-border", !o()), n(l, "title", e.ariaLabel), oe(c, v());
    }), b(r, I);
  }, children: (r, I) => {
    var l = Ae(), c = K(l);
    c.__keydown = [Oe, y, i, E, P];
    let k;
    var M = x(c);
    {
      var N = (f) => {
        Te(f, { onFocus: () => m(i, -1), get value() {
          return t(B);
        }, set value(g) {
          m(B, g, true);
        }, get focus() {
          return t(p);
        }, set focus(g) {
          m(p, g, true);
        } });
      };
      V(M, (f) => {
        C() && f(N);
      });
    }
    var R = F(M, 2);
    ie(R, 21, () => t(P), le, (f, g, D) => {
      G(f, { invisible: true, invisibleOutline: true, onclick: () => E(t(g)), children: (Q, ve) => {
        var U = je(), X = x(U, true);
        w(U), j(() => {
          n(U, "aria-selected", v() === t(g)), n(U, "data-focus", t(i) === D), oe(X, t(g));
        }), b(Q, U);
      }, $$slots: { default: true } });
    }), w(R), te(R, (f) => m(L, f), () => t(L)), w(c);
    var u = F(c, 2);
    se(u), j(() => {
      k = de(c, "", k, { "max-height": e.maxHeight }), n(u, "name", e.name);
    }), ce(u, v), b(r, l);
  }, $$slots: { button: true, default: true } }), ee();
}
ne(["keydown"]);
export {
  Ee as O,
  me as P,
  Te as S
};
