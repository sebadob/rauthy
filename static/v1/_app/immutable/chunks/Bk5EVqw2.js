import { t as D, a as I } from "./BxmJRzoY.js";
import { p as W, a5 as z, c as f, s as q, t as A, a as C, r as m } from "./w0HvPX0p.js";
import { d as F, e as h, s as L } from "./BzP2S3Z_.js";
import { i as G } from "./iO9_dPNE.js";
import { r as H, a as J, s as a, b as N } from "./BdbQ6g_y.js";
import { t as Q, s as X } from "./C4AV2CoD.js";
import { b as Y } from "./Cxw7xmE1.js";
import { b as Z } from "./Dgjj26O5.js";
import { p as r } from "./C6GSeq7M.js";
import { u as $ } from "./N6FgGI8m.js";
import { g as ee } from "./B21bTIl7.js";
import { _ as te } from "./CmsKOCeN.js";
import { b as ae } from "./UPFlzoow.js";
let ke, ge;
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
  var ne = D("<div> </div>"), le = D('<div><input> <div class="label svelte-1yjio1c"><label class="font-label noselect svelte-1yjio1c"> </label> <!></div></div>');
  ke = function(l, e) {
    W(e, true);
    let n = r(e, "ref", 15), c = r(e, "typ", 3, "text"), y = r(e, "id", 19, ee), k = r(e, "value", 15), v = r(e, "label", 3, ""), P = r(e, "autocomplete", 3, "on"), _ = r(e, "placeholder", 3, ""), g = r(e, "disabled", 3, false), j = r(e, "step", 3, 1), b = r(e, "required", 3, false), o = r(e, "isError", 15, false), R = r(e, "width", 3, "inherit"), M = $();
    z(() => {
      k(), w();
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
    let x;
    var t = f(u);
    H(t), t.__input = [
      ie,
      e
    ], t.__keydown = [
      re,
      w,
      e
    ], Z(t, (i) => n(i), () => n());
    var p = q(t, 2), s = f(p), B = f(s, true);
    m(s);
    var K = q(s, 2);
    {
      var O = (i) => {
        var d = ne();
        let E;
        var T = f(d, true);
        m(d), A((V) => {
          E = N(d, 1, "error svelte-1yjio1c", null, E, V), L(T, e.errMsg || M.common.invalidInput);
        }, [
          () => ({
            errWithLabel: !!v()
          })
        ]), Q(3, d, () => X, () => ({
          duration: 150
        })), I(i, d);
      };
      G(K, (i) => {
        o() && i(O);
      });
    }
    m(p), m(u), A(() => {
      x = J(u, "", x, {
        width: R()
      }), a(t, "type", c()), a(t, "id", y()), a(t, "name", e.name), a(t, "title", e.errMsg), a(t, "aria-label", v() || _()), a(t, "autocomplete", P()), a(t, "placeholder", _()), a(t, "aria-placeholder", _()), t.disabled = g(), a(t, "aria-disabled", g()), a(t, "maxlength", e.maxLength || void 0), a(t, "min", e.min || void 0), a(t, "max", e.max || void 0), a(t, "step", j()), t.required = b() || void 0, a(t, "aria-required", b() || false), a(t, "aria-invalid", o()), a(t, "pattern", e.pattern || void 0), a(s, "for", y()), a(s, "data-required", b()), L(B, v());
    }), h("invalid", t, U), h("blur", t, S), h("submit", t, function(...i) {
      var _a;
      (_a = e.onSubmit) == null ? void 0 : _a.apply(this, i);
    }), Y(t, k), I(l, u), C();
  };
  F([
    "input",
    "keydown"
  ]);
  ge = async function() {
    let l = await ae("/auth/v1/pow");
    if (l.text) {
      let e = l.text, n = (await te(async () => {
        const { pow_work_wasm: c } = await import("./UQjNzBcP.js").then(async (m2) => {
          await m2.__tla;
          return m2;
        });
        return {
          pow_work_wasm: c
        };
      }, [], import.meta.url)).pow_work_wasm;
      return await n(e) || "";
    } else console.error(l.error);
  };
})();
export {
  ke as I,
  __tla,
  ge as f
};
