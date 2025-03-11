import { t as L, a as p } from "./CWf9OOFK.js";
import { p as W, c as m, s as E, t as I, a as z, r as f } from "./nlANaGLT.js";
import { d as C, e as h, s as A } from "./BmMHVVX3.js";
import { i as F } from "./DOjUa9u5.js";
import { r as G, a as H, s as a, b as J } from "./CZv_AhHu.js";
import { t as N, s as Q } from "./BEQMYyDu.js";
import { b as X } from "./DHRD3bu9.js";
import { b as Y } from "./CzqnnvDH.js";
import { p as n } from "./uWmgYd3Z.js";
import { u as Z } from "./DwvS5LQk.js";
import { g as $ } from "./B21bTIl7.js";
import { _ as ee } from "./CmsKOCeN.js";
import { b as te } from "./BO1A6s0c.js";
let xe, ye;
let __tla = (async () => {
  function ae(l, e, r) {
    var _a;
    e(), (_a = r.onInput) == null ? void 0 : _a.call(r);
  }
  function ie(l, e, r) {
    var _a, _b, _c, _d, _e;
    switch (e(), l.code) {
      case "Enter":
        e() && ((_a = r.onEnter) == null ? void 0 : _a.call(r));
        break;
      case "ArrowUp":
        (_b = r.onUp) == null ? void 0 : _b.call(r);
        break;
      case "ArrowDown":
        (_c = r.onDown) == null ? void 0 : _c.call(r);
        break;
      case "ArrowLeft":
        (_d = r.onLeft) == null ? void 0 : _d.call(r);
        break;
      case "ArrowRight":
        (_e = r.onRight) == null ? void 0 : _e.call(r);
        break;
    }
  }
  var re = L("<div> </div>"), ne = L('<div><input> <div class="label svelte-1q8j01x"><label class="font-label noselect svelte-1q8j01x"> </label> <!></div></div>');
  xe = function(l, e) {
    W(e, true);
    let r = n(e, "ref", 15), v = n(e, "typ", 3, "text"), x = n(e, "id", 19, $), D = n(e, "value", 15), c = n(e, "label", 3, ""), P = n(e, "autocomplete", 3, "on"), _ = n(e, "placeholder", 3, ""), y = n(e, "disabled", 3, false), j = n(e, "step", 3, 1), b = n(e, "required", 3, false), o = n(e, "isError", 15, false), R = n(e, "width", 3, "inherit"), M = Z();
    function S(i) {
      var _a;
      w(), (_a = e.onBlur) == null ? void 0 : _a.call(e);
    }
    function U(i) {
      i.preventDefault(), o(true);
    }
    function w() {
      var _a;
      let i = (_a = r()) == null ? void 0 : _a.validity;
      return i ? (o(!i.valid), i.valid) : (o(false), true);
    }
    var u = ne();
    let k;
    var t = m(u);
    G(t), t.__input = [
      ae,
      w,
      e
    ], t.__keydown = [
      ie,
      w,
      e
    ], Y(t, (i) => r(i), () => r());
    var g = E(t, 2), d = m(g), B = m(d, true);
    f(d);
    var K = E(d, 2);
    {
      var O = (i) => {
        var s = re();
        let q;
        var T = m(s, true);
        f(s), I((V) => {
          q = J(s, 1, "error svelte-1q8j01x", null, q, V), A(T, e.errMsg || M.common.invalidInput);
        }, [
          () => ({
            errWithLabel: !!c()
          })
        ]), N(3, s, () => Q), p(i, s);
      };
      F(K, (i) => {
        o() && i(O);
      });
    }
    f(g), f(u), I(() => {
      k = H(u, "", k, {
        width: R()
      }), a(t, "type", v()), a(t, "id", x()), a(t, "name", e.name), a(t, "title", e.errMsg), a(t, "aria-label", c() || _()), a(t, "autocomplete", P()), a(t, "placeholder", _()), a(t, "aria-placeholder", _()), t.disabled = y(), a(t, "aria-disabled", y()), a(t, "maxlength", e.maxLength || void 0), a(t, "min", e.min || void 0), a(t, "max", e.max || void 0), a(t, "step", j()), t.required = b() || void 0, a(t, "aria-required", b() || false), a(t, "aria-invalid", o()), a(t, "pattern", e.pattern || void 0), a(d, "for", x()), a(d, "data-required", b()), A(B, c());
    }), h("invalid", t, U), h("blur", t, S), h("submit", t, function(...i) {
      var _a;
      (_a = e.onSubmit) == null ? void 0 : _a.apply(this, i);
    }), X(t, D), p(l, u), z();
  };
  C([
    "input",
    "keydown"
  ]);
  ye = async function() {
    let l = await te("/auth/v1/pow");
    if (l.text) {
      let e = l.text, r = (await ee(async () => {
        const { pow_work_wasm: v } = await import("./UQjNzBcP.js").then(async (m2) => {
          await m2.__tla;
          return m2;
        });
        return {
          pow_work_wasm: v
        };
      }, [], import.meta.url)).pow_work_wasm;
      return await r(e) || "";
    } else console.error(l.error);
  };
})();
export {
  xe as I,
  __tla,
  ye as f
};
