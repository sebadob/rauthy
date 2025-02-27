import { t as A, a as q } from "./D8nUqfqE.js";
import { p as T, c as m, s as p, t as E, a as V, r as f } from "./D-nwkJyM.js";
import { d as W, e as h, s as I } from "./DmLjbmU6.js";
import { i as z } from "./C3XMhfdI.js";
import { r as C, s as a, a as F } from "./D_HYGYLR.js";
import { a as G, t as H, s as J } from "./BUAPoI3e.js";
import { b as N } from "./DAX5MQt8.js";
import { b as Q } from "./CwQ5lQAV.js";
import { p as n } from "./2rFDT0Lm.js";
import { u as X } from "./CMjKUQkH.js";
import { g as Y } from "./DppGgfa0.js";
import { _ as Z } from "./CmsKOCeN.js";
import { b as $ } from "./emZtalxW.js";
let we, he;
let __tla = (async () => {
  function ee(l, e, r) {
    var _a;
    e(), (_a = r.onInput) == null ? void 0 : _a.call(r);
  }
  function te(l, e, r) {
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
  var ae = A("<div> </div>"), ie = A('<div><input> <div class="label svelte-1q8j01x"><label class="font-label noselect svelte-1q8j01x"> </label> <!></div></div>');
  we = function(l, e) {
    T(e, true);
    let r = n(e, "ref", 15), v = n(e, "typ", 3, "text"), x = n(e, "id", 19, Y), L = n(e, "value", 15), c = n(e, "label", 3, ""), D = n(e, "autocomplete", 3, "on"), _ = n(e, "placeholder", 3, ""), y = n(e, "disabled", 3, false), P = n(e, "step", 3, 1), b = n(e, "required", 3, false), o = n(e, "isError", 15, false), j = n(e, "width", 3, "inherit"), R = X();
    function M(i) {
      var _a;
      w(), (_a = e.onBlur) == null ? void 0 : _a.call(e);
    }
    function S(i) {
      i.preventDefault(), o(true);
    }
    function w() {
      var _a;
      let i = (_a = r()) == null ? void 0 : _a.validity;
      return i ? (o(!i.valid), i.valid) : (o(false), true);
    }
    var u = ie(), t = m(u);
    C(t), t.__input = [
      ee,
      w,
      e
    ], t.__keydown = [
      te,
      w,
      e
    ], Q(t, (i) => r(i), () => r());
    var k = p(t, 2), d = m(k), U = m(d, true);
    f(d);
    var B = p(d, 2);
    {
      var K = (i) => {
        var s = ae();
        let g;
        var O = m(s, true);
        f(s), E(() => {
          g = F(s, 1, "error svelte-1q8j01x", null, g, {
            errWithLabel: !!c()
          }), I(O, e.errMsg || R.common.invalidInput);
        }), H(3, s, () => J), q(i, s);
      };
      z(B, (i) => {
        o() && i(K);
      });
    }
    f(k), f(u), E(() => {
      G(u, "width", j()), a(t, "type", v()), a(t, "id", x()), a(t, "name", e.name), a(t, "title", e.errMsg), a(t, "aria-label", c() || _()), a(t, "autocomplete", D()), a(t, "placeholder", _()), a(t, "aria-placeholder", _()), t.disabled = y(), a(t, "aria-disabled", y()), a(t, "maxlength", e.maxLength || void 0), a(t, "min", e.min || void 0), a(t, "max", e.max || void 0), a(t, "step", P()), t.required = b() || void 0, a(t, "aria-required", b() || false), a(t, "aria-invalid", o()), a(t, "pattern", e.pattern || void 0), a(d, "for", x()), a(d, "data-required", b()), I(U, c());
    }), h("invalid", t, S), h("blur", t, M), h("submit", t, function(...i) {
      var _a;
      (_a = e.onSubmit) == null ? void 0 : _a.apply(this, i);
    }), N(t, L), q(l, u), V();
  };
  W([
    "input",
    "keydown"
  ]);
  he = async function() {
    let l = await $("/auth/v1/pow");
    if (l.text) {
      let e = l.text, r = (await Z(async () => {
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
  we as I,
  __tla,
  he as f
};
