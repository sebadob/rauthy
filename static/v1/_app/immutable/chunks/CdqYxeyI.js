import { t as T, a as _, d as Y, n as $ } from "./BH6NCLk-.js";
import { p as ee, aa as q, t as A, a as te, s as R, c as y, j as t, k as M, f as K, r as w, l as b, a9 as ae } from "./CvlvO1XB.js";
import { e as ne, d as ie, s as re } from "./CTI4QPiR.js";
import { i as N } from "./BUO_AUgz.js";
import { e as le, i as se } from "./BpWRzPRQ.js";
import { s as l, r as de } from "./BMbqVy6X.js";
import { B as G, a as Z } from "./CqH8LXO-.js";
import { b as ce } from "./dU6E9WaN.js";
import { p as V } from "./Wh68IIk2.js";
import { b as oe } from "./zosqiMUL.js";
import { p as s } from "./C6SR4G2t.js";
import { s as W } from "./-T201g_q.js";
import { g as J } from "./B21bTIl7.js";
import { u as fe } from "./D8mHI_K9.js";
import { u as ue } from "./CUqQZdNU.js";
var he = T('<div><!> <div class="popover svelte-15eoy2j" popover="auto"><div class="inner fade-in svelte-15eoy2j"><!></div></div></div>');
function ge(h, e) {
  ee(e, true);
  let a = s(e, "ref", 15), d = s(e, "roleButton", 3, "button"), c = s(e, "offsetLeft", 3, "0px"), o = s(e, "offsetTop", 3, "0px"), C = s(e, "close", 15);
  const x = J(), S = J();
  let n = M(void 0), D = M(false);
  q(() => {
    C(P);
  });
  function p(r) {
    if (r.stopPropagation(), a() && t(n)) if (e.absolute) t(n).style.top = o(), t(n).style.left = c();
    else {
      let v = a().getBoundingClientRect();
      t(n).style.top = `calc(${v.bottom + window.scrollY}px + ${o()})`, t(n).style.left = `calc(${v.left + window.scrollX}px + ${c()})`;
    }
    else console.warn("button and popover ref missing");
  }
  function P() {
    var _a;
    (_a = t(n)) == null ? void 0 : _a.hidePopover();
  }
  function z(r) {
    var _a;
    let v = r.newState;
    b(D, v === "open"), (_a = e.onToggle) == null ? void 0 : _a.call(e, v);
  }
  var E = he(), I = y(E);
  G(I, { get role() {
    return d();
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
  }, set ref(r) {
    a(r);
  }, children: (r, v) => {
    var k = Y(), H = K(k);
    W(H, () => e.button), _(r, k);
  }, $$slots: { default: true } });
  var L = R(I, 2);
  l(L, "id", S), l(L, "aria-labelledby", x);
  var F = y(L), i = y(F);
  {
    var g = (r) => {
      var v = Y(), k = K(v);
      {
        var H = (U) => {
          var u = Y(), f = K(u);
          W(f, () => e.children), _(U, u);
        };
        N(k, (U) => {
          t(D) && U(H);
        });
      }
      _(r, v);
    }, O = (r) => {
      var v = Y(), k = K(v);
      W(k, () => e.children), _(r, v);
    };
    N(i, (r) => {
      e.lazy ? r(g) : r(O, false);
    });
  }
  w(F), w(L), oe(L, (r) => b(n, r), () => t(n)), w(E), A(() => l(L, "aria-label", e.ariaLabel)), ne("toggle", L, z), _(h, E), te();
}
var me = $('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z"></path></svg>');
function be(h, e) {
  let a = s(e, "opacity", 3, 0.9), d = s(e, "width", 3, "1.5rem"), c = s(e, "color", 3, "currentColor");
  var o = me();
  l(o, "stroke-width", 2), A(() => {
    l(o, "stroke", c()), l(o, "width", d()), l(o, "opacity", a());
  }), _(h, o);
}
var _e = $('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M12 9.75L14.25 12m0 0l2.25 2.25M14.25 12l2.25-2.25M14.25 12L12 14.25m-2.58 4.92l-6.375-6.375a1.125 1.125 0 010-1.59L9.42 4.83c.211-.211.498-.33.796-.33H19.5a2.25 2.25 0 012.25 2.25v10.5a2.25 2.25 0 01-2.25 2.25h-9.284c-.298 0-.585-.119-.796-.33z"></path></svg>');
function we(h, e) {
  let a = s(e, "opacity", 3, 0.9), d = s(e, "width", 3, "1.5rem"), c = s(e, "color", 3, "currentColor");
  var o = _e();
  l(o, "stroke-width", 2), A(() => {
    l(o, "stroke", c()), l(o, "width", d()), l(o, "opacity", a());
  }), _(h, o);
}
function ke(h, e, a, d) {
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
var ye = T('<div class="options svelte-1wttsdc"><!></div>'), xe = (h) => h.stopPropagation(), Le = T("<option></option>"), pe = T('<datalist class="absolute svelte-1wttsdc"></datalist>'), Be = T('<span class="backspace svelte-1wttsdc"><!></span>'), De = T('<div class="btnSearch svelte-1wttsdc"><!></div>'), Pe = T('<search class="flex container svelte-1wttsdc"><!> <input type="search" autocomplete="off" class="svelte-1wttsdc"> <!> <div class="relative"><div class="absolute btnDelete svelte-1wttsdc"><!></div></div> <!></search>');
function Ie(h, e) {
  ee(e, true);
  let a = ue(), d = fe(), c = s(e, "id", 19, J), o = s(e, "value", 15, ""), C = s(e, "placeholder", 19, () => d.common.search), x = s(e, "option", 15), S = s(e, "focus", 15), n = s(e, "width", 3, "100%");
  const D = J();
  let p = M(void 0), P = ae(() => e.datalist && e.datalist.length > 0 ? D : void 0);
  q(() => {
    S(E);
  });
  function z() {
    var _a;
    (_a = e.onSearch) == null ? void 0 : _a.call(e, o(), x());
  }
  function E() {
    var _a;
    (_a = t(p)) == null ? void 0 : _a.focus();
  }
  var I = Pe(), L = y(I);
  {
    var F = (u) => {
      var f = ye(), m = y(f);
      Ae(m, { get ariaLabel() {
        return d.common.searchOptions;
      }, get options() {
        return e.options;
      }, borderless: true, offsetLeft: "-.25rem", get value() {
        return x();
      }, set value(B) {
        x(B);
      } }), w(f), _(u, f);
    };
    N(L, (u) => {
      e.options && u(F);
    });
  }
  var i = R(L, 2);
  de(i), i.__click = [xe], i.__keydown = [ke, z, e, o], oe(i, (u) => b(p, u), () => t(p));
  var g = R(i, 2);
  {
    var O = (u) => {
      var f = pe();
      l(f, "id", D), le(f, 21, () => e.datalist, se, (m, B, Q, ve) => {
        var j = Le(), X = {};
        A(() => {
          X !== (X = t(B)) && (j.value = (j.__value = t(B)) == null ? "" : t(B));
        }), _(m, j);
      }), w(f), _(u, f);
    };
    N(g, (u) => {
      e.datalist && u(O);
    });
  }
  var r = R(g, 2), v = y(r), k = y(v);
  G(k, { get ariaLabel() {
    return a.common.delete;
  }, invisible: true, onclick: () => o(""), children: (u, f) => {
    var m = Be(), B = y(m);
    we(B, { color: "hsl(var(--bg-high))" }), w(m), _(u, m);
  }, $$slots: { default: true } }), w(v), w(r);
  var H = R(r, 2);
  {
    var U = (u) => {
      var f = De(), m = y(f);
      G(m, { get ariaLabel() {
        return d.common.search;
      }, invisible: true, onclick: z, children: (B, Q) => {
        be(B, {});
      }, $$slots: { default: true } }), w(f), _(u, f);
    };
    N(H, (u) => {
      e.onSearch && u(U);
    });
  }
  w(I), A(() => {
    Z(I, "border", e.borderless ? void 0 : "1px solid hsl(var(--bg-high))"), Z(I, "width", n()), l(i, "id", c()), l(i, "list", t(P)), l(i, "aria-label", e.ariaLabel || d.common.search), l(i, "pattern", e.pattern), l(i, "placeholder", C());
  }), ne("focus", i, () => {
    var _a;
    return (_a = e.onFocus) == null ? void 0 : _a.call(e);
  }), ce(i, o), _(h, I), te();
}
ie(["click", "keydown"]);
var Te = $('<svg fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5"></path></svg>');
function Ce(h, e) {
  let a = s(e, "opacity", 3, 0.9), d = s(e, "width", 3, "1.5rem"), c = s(e, "color", 3, "currentColor");
  var o = Te();
  l(o, "stroke-width", 2), A(() => {
    l(o, "width", d()), l(o, "color", c()), l(o, "opacity", a());
  }), _(h, o);
}
function Se(h, e, a, d, c) {
  let o = h.code;
  o === "ArrowDown" ? (h.preventDefault(), e() && b(a, t(a) + 1)) : o === "ArrowUp" ? (h.preventDefault(), e() && b(a, t(a) - 1)) : o === "Enter" && t(a) > -1 ? d(t(c)[t(a)]) : o === "Enter" && t(a) === -1 && t(c).length === 1 && d(t(c)[0]);
}
var Oe = T('<div class="btn svelte-5ktp6l"><div class="btnText svelte-5ktp6l"> </div> <!></div>'), Ue = T('<div class="optPopover svelte-5ktp6l"> </div>'), je = T('<div role="listbox" tabindex="0" class="popover svelte-5ktp6l"><!> <div class="popoverOptions svelte-5ktp6l"></div></div> <input type="hidden" aria-hidden="true">', 1);
function Ae(h, e) {
  ee(e, true);
  let a = s(e, "ref", 15), d = s(e, "options", 19, () => []), c = s(e, "value", 15), o = s(e, "borderless", 3, false), C = s(e, "withSearch", 3, false), x = M(void 0), S = M(void 0), n = M(V(C() ? -1 : 0)), D = M(void 0), p = M(""), P = ae(() => {
    if (!C()) return d();
    if (typeof c() == "string") return d().filter((g) => g.toLowerCase().includes(t(p).toLowerCase()));
    let i = Number.parseInt(t(p)) || c();
    return d().filter((g) => g === i);
  }), z = false;
  q(() => {
    var _a;
    let i = c();
    z ? (_a = e.onChange) == null ? void 0 : _a.call(e, i) : z = true;
  }), q(() => {
    var _a, _b;
    if (t(n) === -1 && ((_a = t(x)) == null ? void 0 : _a.scrollTo({ top: 0, behavior: "smooth" })), C()) {
      if (t(n) < 0 || t(n) > t(P).length - 1) {
        b(n, -1), (_b = t(D)) == null ? void 0 : _b();
        return;
      }
    } else t(n) < 0 ? b(n, t(P).length - 1) : t(n) > t(P).length - 1 && b(n, 0), E();
  });
  function E() {
    if (t(x)) {
      let i = t(x).getElementsByTagName("button")[t(n)];
      i == null ? void 0 : i.scrollIntoView({ behavior: "smooth", block: "center" }), i == null ? void 0 : i.focus();
    } else console.error("refOptions is undefined");
  }
  function I(i) {
    var _a;
    i === "open" && (C() ? (b(n, -1), (_a = t(D)) == null ? void 0 : _a()) : (b(n, V(d().findIndex((g) => g === c()) || 0)), E()));
  }
  function L() {
    return t(P).length > 0 ? true : (b(n, -1), false);
  }
  function F(i) {
    c(i), b(p, ""), setTimeout(() => {
      var _a;
      (_a = t(S)) == null ? void 0 : _a();
    }, 20);
  }
  ge(h, { get ariaLabel() {
    return e.ariaLabel;
  }, roleButton: "combobox", btnInvisible: true, get offsetTop() {
    return e.offsetTop;
  }, get offsetLeft() {
    return e.offsetLeft;
  }, onToggle: I, get onLeft() {
    return e.onLeft;
  }, get onRight() {
    return e.onRight;
  }, get onUp() {
    return e.onUp;
  }, get onDown() {
    return e.onDown;
  }, get ref() {
    return a();
  }, set ref(g) {
    a(g);
  }, get close() {
    return t(S);
  }, set close(g) {
    b(S, V(g));
  }, button: (g) => {
    var O = Oe(), r = y(O), v = y(r, true);
    w(r);
    var k = R(r, 2);
    Ce(k, { width: ".8rem" }), w(O), A(() => {
      l(O, "data-border", !o()), l(r, "title", e.ariaLabel), re(v, c());
    }), _(g, O);
  }, children: (g, O) => {
    var r = je(), v = K(r);
    v.__keydown = [Se, L, n, F, P];
    var k = y(v);
    {
      var H = (f) => {
        Ie(f, { onFocus: () => b(n, -1), get value() {
          return t(p);
        }, set value(m) {
          b(p, V(m));
        }, get focus() {
          return t(D);
        }, set focus(m) {
          b(D, V(m));
        } });
      };
      N(k, (f) => {
        C() && f(H);
      });
    }
    var U = R(k, 2);
    le(U, 21, () => t(P), se, (f, m, B) => {
      G(f, { invisible: true, invisibleOutline: true, onclick: () => F(t(m)), children: (Q, ve) => {
        var j = Ue(), X = y(j, true);
        w(j), A(() => {
          l(j, "aria-selected", c() === t(m)), l(j, "data-focus", t(n) === B), re(X, t(m));
        }), _(Q, j);
      }, $$slots: { default: true } });
    }), w(U), oe(U, (f) => b(x, f), () => t(x)), w(v);
    var u = R(v, 2);
    de(u), A(() => {
      Z(v, "max-height", e.maxHeight), l(u, "name", e.name);
    }), ce(u, c), _(g, r);
  }, $$slots: { button: true, default: true } }), te();
}
ie(["keydown"]);
export {
  Ae as O,
  ge as P,
  Ie as S
};
