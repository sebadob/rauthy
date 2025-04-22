import { n as N, a as u, t as b } from "./BxmJRzoY.js";
import { t as _, p as se, j as de, l as P, f as ve, c as d, s as k, r as v, k as ce, a as ue } from "./w0HvPX0p.js";
import { d as me, s as H, e as J } from "./BzP2S3Z_.js";
import { i as I } from "./iO9_dPNE.js";
import { s as a, r as fe, a as K } from "./BdbQ6g_y.js";
import { B as q, t as he, s as _e } from "./C4AV2CoD.js";
import { b as be } from "./Cxw7xmE1.js";
import { b as we } from "./Dgjj26O5.js";
import { p as r } from "./C6GSeq7M.js";
import { I as pe } from "./BR_xb8zP.js";
import { u as ye } from "./N6FgGI8m.js";
var ge = N('<svg fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" d="M3.98 8.223A10.477 10.477 0 001.934 12C3.226 16.338 7.244 19.5 12 19.5c.993 0 1.953-.138 2.863-.395M6.228 6.228A10.45 10.45 0 0112 4.5c4.756 0 8.773 3.162 10.065 7.498a10.523 10.523 0 01-4.293 5.774M6.228 6.228L3 3m3.228 3.228l3.65 3.65m7.894 7.894L21 21m-3.228-3.228l-3.65-3.65m0 0a3 3 0 10-4.243-4.243m4.242 4.242L9.88 9.88"></path></svg>');
function ke(m, e) {
  let s = r(e, "opacity", 3, 0.9), c = r(e, "width", 3, "1.5rem"), y = r(e, "color", 3, "hsl(var(--error))");
  var o = ge();
  a(o, "stroke-width", 2), _(() => {
    a(o, "width", c()), a(o, "color", y()), a(o, "opacity", s());
  }), u(m, o);
}
var xe = N('<svg fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" d="M2.036 12.322a1.012 1.012 0 010-.639C3.423 7.51 7.36 4.5 12 4.5c4.638 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.638 0-8.573-3.007-9.963-7.178z"></path><path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path></svg>');
function Ce(m, e) {
  let s = r(e, "opacity", 3, 0.9), c = r(e, "width", 3, "1.5rem"), y = r(e, "color", 3, "currentColor");
  var o = xe();
  a(o, "stroke-width", 2), _(() => {
    a(o, "width", c()), a(o, "color", y()), a(o, "opacity", s());
  }), u(m, o);
}
function Le(m, e, s) {
  var _a;
  e(), (_a = s.onInput) == null ? void 0 : _a.call(s);
}
function Ie(m, e, s) {
  var _a;
  m.code === "Enter" && e() && ((_a = s.onEnter) == null ? void 0 : _a.call(s));
}
var Me = b("<div><!></div>"), Pe = b('<div class="btn clip svelte-11t06s4"><!></div>'), qe = b("<div><!></div>"), Be = b("<div><!></div>"), Ee = b('<div class="nolabel svelte-11t06s4"></div>'), je = b('<div class="error svelte-11t06s4"><!> </div>'), Ve = b('<div><div class="input-row svelte-11t06s4"><input> <div class="rel svelte-11t06s4"><!> <div class="btn show svelte-11t06s4"><!></div></div></div></div> <div class="label svelte-11t06s4"><label class="font-label noselect svelte-11t06s4"> </label> <!></div>', 1);
function Oe(m, e) {
  se(e, true);
  let s = r(e, "ref", 15), c = r(e, "type", 7, "password"), y = r(e, "name", 3, "password"), o = r(e, "value", 15, ""), B = r(e, "label", 3, "Password"), O = r(e, "ariaLabel", 3, "Password"), Q = r(e, "autocomplete", 3, "current-password"), R = r(e, "placeholder", 3, "Password"), U = r(e, "disabled", 3, false), W = r(e, "maxLength", 3, 128), X = r(e, "width", 3, "inherit"), E = r(e, "showCopy", 3, false), Y = r(e, "reportValidity", 15), h = ye(), x = de(false);
  Y(C);
  function Z() {
    navigator.clipboard ? navigator.clipboard.writeText(o()) : console.error("Copy to clipboard is only available in secure contexts");
  }
  function j() {
    c() === "password" ? c("text") : c("password");
  }
  function $(t) {
    var _a;
    C(), (_a = e.onBlur) == null ? void 0 : _a.call(e);
  }
  function ee(t) {
    t.preventDefault(), P(x, true);
  }
  function C() {
    var _a;
    let t = (_a = s()) == null ? void 0 : _a.validity;
    return t ? (P(x, !t.valid), t.valid) : (P(x, false), true);
  }
  var V = Ve(), L = ve(V);
  let T;
  var z = d(L), i = d(z);
  fe(i), i.__input = [Le, C, e], i.__keydown = [Ie, C, e];
  let A;
  we(i, (t) => s(t), () => s());
  var D = k(i, 2), S = d(D);
  {
    var te = (t) => {
      var n = Pe(), w = d(n);
      q(w, { get ariaLabel() {
        return h.common.copyToClip;
      }, invisible: true, onclick: Z, children: (l, p) => {
        var f = Me(), M = d(f);
        pe(M, {}), v(f), _(() => a(f, "title", h.common.copyToClip)), u(l, f);
      }, $$slots: { default: true } }), v(n), u(t, n);
    };
    I(S, (t) => {
      E() && t(te);
    });
  }
  var F = k(S, 2), ae = d(F);
  {
    var ie = (t) => {
      q(t, { get ariaLabel() {
        return h.common.show;
      }, invisible: true, onclick: j, children: (n, w) => {
        var l = qe(), p = d(l);
        ke(p, {}), v(l), _(() => a(l, "title", h.common.show)), u(n, l);
      }, $$slots: { default: true } });
    }, re = (t) => {
      q(t, { get ariaLabel() {
        return h.common.hide;
      }, invisible: true, onclick: j, children: (n, w) => {
        var l = Be(), p = d(l);
        Ce(p, {}), v(l), _(() => a(l, "title", h.common.hide)), u(n, l);
      }, $$slots: { default: true } });
    };
    I(ae, (t) => {
      c() === "password" ? t(ie) : t(re, false);
    });
  }
  v(F), v(D), v(z), v(L);
  var G = k(L, 2), g = d(G), oe = d(g, true);
  v(g);
  var le = k(g, 2);
  {
    var ne = (t) => {
      var n = je(), w = d(n);
      {
        var l = (f) => {
          var M = Ee();
          u(f, M);
        };
        I(w, (f) => {
          B() || f(l);
        });
      }
      var p = k(w);
      v(n), _(() => H(p, ` ${(e.errMsg || h.common.invalidInput) ?? ""}`)), he(3, n, () => _e, () => ({ duration: 150 })), u(t, n);
    };
    I(le, (t) => {
      ce(x) && t(ne);
    });
  }
  v(G), _(() => {
    T = K(L, "", T, { width: X() }), a(i, "type", c()), a(i, "id", e.id), a(i, "name", y()), a(i, "title", e.errMsg), a(i, "aria-label", O()), a(i, "autocomplete", Q()), a(i, "placeholder", R()), i.disabled = U(), i.required = e.required || void 0, a(i, "aria-required", e.required || false), a(i, "maxlength", W() || void 0), a(i, "pattern", e.pattern || void 0), A = K(i, "", A, { "padding-right": E() ? "3.8rem" : "2.2rem" }), a(g, "for", e.id), a(g, "data-required", e.required), H(oe, B());
  }), J("invalid", i, ee), J("blur", i, $), be(i, o), u(m, V), ue();
}
me(["input", "keydown"]);
export {
  Oe as I,
  Ce as a,
  ke as b
};
