import { t as D, a as O, d as Y } from "./D8nUqfqE.js";
import { K as B, E as Z, aD as $, aE as tt, x as at, O as rt, aF as et, aG as it, P as st, a8 as A, au as z, av as M, aw as ot, aH as nt, aI as lt, c as N, s as W, r as S, t as H, p as ct, k as dt, aa as vt, a as ft, l as j, j as E, a9 as U, f as ut } from "./D-nwkJyM.js";
import { i as G } from "./C3XMhfdI.js";
import { s as _t } from "./ivlJJTAR.js";
import { a as bt, s as pt, d as gt, C as ht } from "./D_HYGYLR.js";
import { o as mt, w as wt } from "./DmLjbmU6.js";
import { b as yt } from "./CwQ5lQAV.js";
import { p as g, r as kt } from "./2rFDT0Lm.js";
function q(a, t, r, i) {
  var e = a.__styles ?? (a.__styles = {});
  e[t] !== r && (e[t] = r, r == null ? a.style.removeProperty(t) : a.style.setProperty(t, r, ""));
}
const xt = () => performance.now(), x = { tick: (a) => requestAnimationFrame(a), now: () => xt(), tasks: /* @__PURE__ */ new Set() };
function J() {
  const a = x.now();
  x.tasks.forEach((t) => {
    t.c(a) || (x.tasks.delete(t), t.f());
  }), x.tasks.size !== 0 && x.tick(J);
}
function Ct(a) {
  let t;
  return x.tasks.size === 0 && x.tick(J), { promise: new Promise((r) => {
    x.tasks.add(t = { c: a, f: r });
  }), abort() {
    x.tasks.delete(t);
  } };
}
function I(a, t) {
  wt(() => {
    a.dispatchEvent(new CustomEvent(t));
  });
}
function Ft(a) {
  if (a === "float") return "cssFloat";
  if (a === "offset") return "cssOffset";
  if (a.startsWith("--")) return a;
  const t = a.split("-");
  return t.length === 1 ? t[0] : t[0] + t.slice(1).map((r) => r[0].toUpperCase() + r.slice(1)).join("");
}
function K(a) {
  const t = {}, r = a.split(";");
  for (const i of r) {
    const [e, s] = i.split(":");
    if (!e || s === void 0) break;
    const c = Ft(e.trim());
    t[c] = s.trim();
  }
  return t;
}
const Tt = (a) => a;
function Q(a, t, r, i) {
  var e = (a & nt) !== 0, s = (a & lt) !== 0, c = e && s, d = (a & et) !== 0, h = c ? "both" : e ? "in" : "out", n, l = t.inert, w = t.style.overflow, u, p;
  function m() {
    var k = ot, F = B;
    z(null), M(null);
    try {
      return n ?? (n = r()(t, (i == null ? void 0 : i()) ?? {}, { direction: h }));
    } finally {
      z(k), M(F);
    }
  }
  var f = { is_global: d, in() {
    var _a;
    if (t.inert = l, !e) {
      p == null ? void 0 : p.abort(), (_a = p == null ? void 0 : p.reset) == null ? void 0 : _a.call(p);
      return;
    }
    s || (u == null ? void 0 : u.abort()), I(t, "introstart"), u = P(t, m(), p, 1, () => {
      I(t, "introend"), u == null ? void 0 : u.abort(), u = n = void 0, t.style.overflow = w;
    });
  }, out(k) {
    if (!s) {
      k == null ? void 0 : k(), n = void 0;
      return;
    }
    t.inert = true, I(t, "outrostart"), p = P(t, m(), u, 0, () => {
      I(t, "outroend"), k == null ? void 0 : k();
    });
  }, stop: () => {
    u == null ? void 0 : u.abort(), p == null ? void 0 : p.abort();
  } }, v = B;
  if ((v.transitions ?? (v.transitions = [])).push(f), e && mt) {
    var o = d;
    if (!o) {
      for (var _ = v.parent; _ && _.f & Z; ) for (; (_ = _.parent) && !(_.f & $); ) ;
      o = !_ || (_.f & tt) !== 0;
    }
    o && at(() => {
      rt(() => f.in());
    });
  }
}
function P(a, t, r, i, e) {
  var s = i === 1;
  if (it(t)) {
    var c, d = false;
    return st(() => {
      if (!d) {
        var v = t({ direction: s ? "in" : "out" });
        c = P(a, v, r, i, e);
      }
    }), { abort: () => {
      d = true, c == null ? void 0 : c.abort();
    }, deactivate: () => c.deactivate(), reset: () => c.reset(), t: () => c.t() };
  }
  if (r == null ? void 0 : r.deactivate(), !(t == null ? void 0 : t.duration)) return e(), { abort: A, deactivate: A, reset: A, t: () => i };
  const { delay: h = 0, css: n, tick: l, easing: w = Tt } = t;
  var u = [];
  if (s && r === void 0 && (l && l(0, 1), n)) {
    var p = K(n(0, 1));
    u.push(p, p);
  }
  var m = () => 1 - i, f = a.animate(u, { duration: h });
  return f.onfinish = () => {
    var v = (r == null ? void 0 : r.t()) ?? 1 - i;
    r == null ? void 0 : r.abort();
    var o = i - v, _ = t.duration * Math.abs(o), k = [];
    if (_ > 0) {
      var F = false;
      if (n) for (var b = Math.ceil(_ / 16.666666666666668), y = 0; y <= b; y += 1) {
        var T = v + o * w(y / b), L = K(n(T, 1 - T));
        k.push(L), F || (F = L.overflow === "hidden");
      }
      F && (a.style.overflow = "hidden"), m = () => {
        var C = f.currentTime;
        return v + o * w(C / _);
      }, l && Ct(() => {
        if (f.playState !== "running") return false;
        var C = m();
        return l(C, 1 - C), true;
      });
    }
    f = a.animate(k, { duration: _, fill: "forwards" }), f.onfinish = () => {
      m = () => i, l == null ? void 0 : l(i, 1 - i), e();
    };
  }, { abort: () => {
    f && (f.cancel(), f.effect = null, f.onfinish = A);
  }, deactivate: () => {
    e = A;
  }, reset: () => {
    i === 0 && (l == null ? void 0 : l(1, 0));
  }, t: () => m() };
}
const Lt = (a) => a;
function At(a) {
  const t = a - 1;
  return t * t * t + 1;
}
function V(a, { delay: t = 0, duration: r = 400, easing: i = Lt } = {}) {
  const e = +getComputedStyle(a).opacity;
  return { delay: t, duration: r, easing: i, css: (s) => `opacity: ${s * e}` };
}
function Wt(a, { delay: t = 0, duration: r = 400, easing: i = At, axis: e = "y" } = {}) {
  const s = getComputedStyle(a), c = +s.opacity, d = e === "y" ? "height" : "width", h = parseFloat(s[d]), n = e === "y" ? ["top", "bottom"] : ["left", "right"], l = n.map((o) => `${o[0].toUpperCase()}${o.slice(1)}`), w = parseFloat(s[`padding${l[0]}`]), u = parseFloat(s[`padding${l[1]}`]), p = parseFloat(s[`margin${l[0]}`]), m = parseFloat(s[`margin${l[1]}`]), f = parseFloat(s[`border${l[0]}Width`]), v = parseFloat(s[`border${l[1]}Width`]);
  return { delay: t, duration: r, easing: i, css: (o) => `overflow: hidden;opacity: ${Math.min(o * 20, 1) * c};${d}: ${o * h}px;padding-${n[0]}: ${o * w}px;padding-${n[1]}: ${o * u}px;margin-${n[0]}: ${o * p}px;margin-${n[1]}: ${o * m}px;border-${n[0]}-width: ${o * f}px;border-${n[1]}-width: ${o * v}px;min-${d}: 0` };
}
var Et = D('<div><div class="loading svelte-1yqkxw6"><div class="loading-1 svelte-1yqkxw6"></div> <div class="loading-2 svelte-1yqkxw6"></div> <div class="loading-3 svelte-1yqkxw6"></div></div></div>');
function Ot(a, t) {
  let r = g(t, "background", 3, false), i = g(t, "color", 3, "hsl(var(--text))"), e = g(t, "global", 3, false), s = g(t, "offset", 3, 0);
  var c = Et();
  let d;
  var h = N(c), n = N(h), l = W(n, 2), w = W(l, 2);
  S(h), S(c), H(() => {
    d = bt(c, 1, "container svelte-1yqkxw6", null, d, { global: e(), local: !e(), background: r() }), pt(h, "style", `margin-top: ${s() ?? ""}px;`), q(n, "background", i()), q(l, "background", i()), q(w, "background", i());
  }), Q(3, c, () => V, () => ({ duration: 100 })), O(a, c);
}
var Nt = D('<div class="load svelte-148xxdi"><!></div>'), St = D('<div class="font-label svelte-148xxdi"><!></div>'), Rt = D("<button><!></button>");
function jt(a, t) {
  ct(t, true);
  let r = g(t, "type", 3, "button"), i = g(t, "role", 3, "button"), e = g(t, "ref", 15), s = g(t, "level", 3, 1), c = g(t, "isDisabled", 3, false), d = g(t, "isLoading", 3, false), h = g(t, "invisible", 3, false), n = g(t, "invisibleOutline", 3, false), l = kt(t, ["$$slots", "$$events", "$$legacy", "type", "role", "ref", "id", "ariaLabel", "ariaControls", "ariaCurrent", "level", "width", "isDisabled", "isLoading", "invisible", "invisibleOutline", "popovertarget", "popovertargetaction", "onclick", "onLeft", "onRight", "onUp", "onDown", "children"]), w = U(() => {
    if (h()) return "invisible";
    switch (s()) {
      case -1:
        return "l1d";
      case -2:
        return "l2d";
      case -3:
        return "l3d";
      case 2:
        return "l2";
      case 3:
        return "l3";
      default:
        return "l1";
    }
  }), u = dt(!d()), p = U(() => c() || d());
  vt(() => {
    d() ? setTimeout(() => {
      j(u, false);
    }, 120) : setTimeout(() => {
      j(u, true);
    }, 120);
  });
  function m() {
    switch (s()) {
      case -1:
        return "var(--btn-text)";
      case -2:
        return "hsl(var(--error))";
      case -3:
        return "hsl(var(--error))";
      case 2:
        return "hsl(var(--action))";
      case 3:
        return "hsl(var(--action))";
      default:
        return "var(--btn-text)";
    }
  }
  function f(b) {
    var _a, _b, _c, _d, _e;
    switch (b.code) {
      case "Enter":
        (_a = t.onclick) == null ? void 0 : _a.call(t, b);
        break;
      case "ArrowLeft":
        (_b = t.onLeft) == null ? void 0 : _b.call(t);
        break;
      case "ArrowRight":
        (_c = t.onRight) == null ? void 0 : _c.call(t);
        break;
      case "ArrowUp":
        (_d = t.onUp) == null ? void 0 : _d.call(t);
        break;
      case "ArrowDown":
        (_e = t.onDown) == null ? void 0 : _e.call(t);
        break;
    }
  }
  var v = Rt();
  let o;
  var _ = N(v);
  {
    var k = (b) => {
      var y = Nt(), T = N(y);
      const L = U(m);
      Ot(T, { background: false, get color() {
        return E(L);
      } }), S(y), O(b, y);
    }, F = (b) => {
      var y = Y(), T = ut(y);
      {
        var L = (C) => {
          var R = St(), X = N(R);
          _t(X, () => t.children), S(R), Q(1, R, () => V), O(C, R);
        };
        G(T, (C) => {
          E(u) && C(L);
        }, true);
      }
      O(b, y);
    };
    G(_, (b) => {
      d() ? b(k) : b(F, false);
    });
  }
  S(v), yt(v, (b) => e(b), () => e()), H(() => {
    o = gt(v, o, { role: i(), type: r(), id: t.id, "aria-label": t.ariaLabel, "aria-controls": t.ariaControls, "aria-current": t.ariaCurrent, class: E(w), "data-isloading": d(), onclick: t.onclick, onkeydown: f, disabled: E(p), "aria-disabled": E(p), popovertarget: t.popovertarget, popovertargetaction: t.popovertargetaction, ...l, [ht]: { invisibleOutline: n() } }, "svelte-148xxdi"), q(v, "width", t.width);
  }), O(a, v), ft();
}
export {
  jt as B,
  Ot as L,
  q as a,
  V as f,
  Ct as l,
  x as r,
  Wt as s,
  Q as t
};
