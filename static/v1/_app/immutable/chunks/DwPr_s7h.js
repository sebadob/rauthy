import { t as D, a as E } from "./DLBGyKVC.js";
import { p as W, aa as z, c as f, s as I, t as A, a as C, r as m } from "./CmQi0fbH.js";
import { d as F, e as h, s as L } from "./BjaYyaa_.js";
import { i as G } from "./C6bK2EJJ.js";
import { r as H, a as J, s as a, b as N } from "./DOlUUCkJ.js";
import { t as Q, s as X } from "./DPLO-ozG.js";
import { b as Y } from "./B3JAWFL5.js";
import { b as Z } from "./BLvsVIZg.js";
import { p as r } from "./DNJm3-SG.js";
import { u as $ } from "./DGTOa5g8.js";
import { g as ee } from "./B21bTIl7.js";
import { _ as te } from "./CmsKOCeN.js";
import { b as ae } from "./BO1A6s0c.js";
let ye, ke;
let __tla = (async () => {
  function ie(l, e) {
    var _a;
    (_a = e.onInput) == null ? void 0 : _a.call(e);
  }
  function re(l, e, n) {
    var _a, _b, _c, _d, _e;
    switch (e(), l.code) {
      case "Enter":
        e() && ((_a = n.onEnter) == null ? void 0 : _a.call(n));
        break;
      case "ArrowUp":
        (_b = n.onUp) == null ? void 0 : _b.call(n);
        break;
      case "ArrowDown":
        (_c = n.onDown) == null ? void 0 : _c.call(n);
        break;
      case "ArrowLeft":
        (_d = n.onLeft) == null ? void 0 : _d.call(n);
        break;
      case "ArrowRight":
        (_e = n.onRight) == null ? void 0 : _e.call(n);
        break;
    }
  }
  var ne = D("<div> </div>"), le = D('<div><input> <div class="label svelte-1q8j01x"><label class="font-label noselect svelte-1q8j01x"> </label> <!></div></div>');
  ye = function(l, e) {
    W(e, true);
    let n = r(e, "ref", 15), v = r(e, "typ", 3, "text"), x = r(e, "id", 19, ee), y = r(e, "value", 15), c = r(e, "label", 3, ""), P = r(e, "autocomplete", 3, "on"), _ = r(e, "placeholder", 3, ""), k = r(e, "disabled", 3, false), j = r(e, "step", 3, 1), b = r(e, "required", 3, false), o = r(e, "isError", 15, false), R = r(e, "width", 3, "inherit"), M = $();
    z(() => {
      y(), w();
    });
    function S(i) {
      var _a;
      w(), (_a = e.onBlur) == null ? void 0 : _a.call(e);
    }
    function U(i) {
      i.preventDefault(), o(true);
    }
    function w() {
      var _a;
      let i = (_a = n()) == null ? void 0 : _a.validity;
      return i ? (o(!i.valid), i.valid) : (o(false), true);
    }
    var u = le();
    let g;
    var t = f(u);
    H(t), t.__input = [
      ie,
      e
    ], t.__keydown = [
      re,
      w,
      e
    ], Z(t, (i) => n(i), () => n());
    var q = I(t, 2), s = f(q), B = f(s, true);
    m(s);
    var K = I(s, 2);
    {
      var O = (i) => {
        var d = ne();
        let p;
        var T = f(d, true);
        m(d), A((V) => {
          p = N(d, 1, "error svelte-1q8j01x", null, p, V), L(T, e.errMsg || M.common.invalidInput);
        }, [
          () => ({
            errWithLabel: !!c()
          })
        ]), Q(3, d, () => X, () => ({
          duration: 150
        })), E(i, d);
      };
      G(K, (i) => {
        o() && i(O);
      });
    }
    m(q), m(u), A(() => {
      g = J(u, "", g, {
        width: R()
      }), a(t, "type", v()), a(t, "id", x()), a(t, "name", e.name), a(t, "title", e.errMsg), a(t, "aria-label", c() || _()), a(t, "autocomplete", P()), a(t, "placeholder", _()), a(t, "aria-placeholder", _()), t.disabled = k(), a(t, "aria-disabled", k()), a(t, "maxlength", e.maxLength || void 0), a(t, "min", e.min || void 0), a(t, "max", e.max || void 0), a(t, "step", j()), t.required = b() || void 0, a(t, "aria-required", b() || false), a(t, "aria-invalid", o()), a(t, "pattern", e.pattern || void 0), a(s, "for", x()), a(s, "data-required", b()), L(B, c());
    }), h("invalid", t, U), h("blur", t, S), h("submit", t, function(...i) {
      var _a;
      (_a = e.onSubmit) == null ? void 0 : _a.apply(this, i);
    }), Y(t, y), E(l, u), C();
  };
  F([
    "input",
    "keydown"
  ]);
  ke = async function() {
    let l = await ae("/auth/v1/pow");
    if (l.text) {
      let e = l.text, n = (await te(async () => {
        const { pow_work_wasm: v } = await import("./UQjNzBcP.js").then(async (m2) => {
          await m2.__tla;
          return m2;
        });
        return {
          pow_work_wasm: v
        };
      }, [], import.meta.url)).pow_work_wasm;
      return await n(e) || "";
    } else console.error(l.error);
  };
})();
export {
  ye as I,
  __tla,
  ke as f
};
