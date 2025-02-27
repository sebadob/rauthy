import { t as L, a as A } from "./BH6NCLk-.js";
import { p as H, c as v, s as M, r as c, t as D, a as J } from "./CvlvO1XB.js";
import { d as K, r as N, s as E, e as _ } from "./CTI4QPiR.js";
import { i as O } from "./BUO_AUgz.js";
import { s as t, a as P } from "./BMbqVy6X.js";
import { a as Q, t as S, s as T } from "./DMkkW5Nn.js";
import { b as W } from "./BhIBACXG.js";
import { b as X } from "./zosqiMUL.js";
import { p as i } from "./C6SR4G2t.js";
function Y(o, e) {
  e();
}
function Z(o, e, r) {
  var _a, _b, _c, _d;
  switch (e(), o.code) {
    case "ArrowUp":
      (_a = r.onUp) == null ? void 0 : _a.call(r);
      break;
    case "ArrowDown":
      (_b = r.onDown) == null ? void 0 : _b.call(r);
      break;
    case "ArrowLeft":
      (_c = r.onLeft) == null ? void 0 : _c.call(r);
      break;
    case "ArrowRight":
      (_d = r.onRight) == null ? void 0 : _d.call(r);
      break;
  }
}
var p = L('<div class="error svelte-y71ny6"> </div>'), $ = L(`<div class="container svelte-y71ny6"><textarea autocomplete="off">
    </textarea> <div class="label svelte-y71ny6"><label class="font-label noselect svelte-y71ny6"> </label> <!></div></div>`);
function de(o, e) {
  H(e, true);
  let r = i(e, "ref", 15), w = i(e, "value", 15, ""), y = i(e, "label", 3, ""), R = i(e, "rows", 3, 3), m = i(e, "placeholder", 3, ""), b = i(e, "errMsg", 3, ""), g = i(e, "disabled", 3, false), V = i(e, "fontMono", 3, false), h = i(e, "required", 3, false), l = i(e, "isError", 15, false), C = i(e, "width", 3, "inherit");
  const x = e.pattern ? new RegExp(e.pattern, "gm") : void 0;
  function I(n) {
    var _a;
    d(), (_a = e.onBlur) == null ? void 0 : _a.call(e);
  }
  function U(n) {
    n.preventDefault(), l(true);
  }
  function B(n) {
    d();
  }
  function d() {
    var _a, _b;
    return x && (w().match(x).length > 2 ? ((_a = r()) == null ? void 0 : _a.setCustomValidity(b()), l(true)) : ((_b = r()) == null ? void 0 : _b.setCustomValidity(""), l(false))), true;
  }
  var f = $(), a = v(f);
  N(a), a.__input = [Y, d], a.__keydown = [Z, d, e];
  let k;
  X(a, (n) => r(n), () => r());
  var q = M(a, 2), s = v(q), j = v(s, true);
  c(s);
  var z = M(s, 2);
  {
    var F = (n) => {
      var u = p(), G = v(u, true);
      c(u), D(() => E(G, b())), S(3, u, () => T), A(n, u);
    };
    O(z, (n) => {
      l() && n(F);
    });
  }
  c(q), c(f), D(() => {
    Q(f, "width", C()), t(a, "id", e.id), t(a, "name", e.name), t(a, "title", b()), t(a, "aria-label", y() || m()), t(a, "rows", R()), t(a, "placeholder", m()), t(a, "aria-placeholder", m()), a.disabled = g(), t(a, "aria-disabled", g()), t(a, "maxlength", e.maxLength || void 0), a.required = h() || void 0, t(a, "aria-required", h() || false), t(a, "aria-invalid", l()), k = P(a, 1, "svelte-y71ny6", null, k, { "font-mono": V() }), t(s, "for", e.id), t(s, "data-required", h()), E(j, y());
  }), _("submit", a, B), _("invalid", a, U), _("blur", a, I), W(a, w), A(o, f), J();
}
K(["input", "keydown"]);
export {
  de as I
};
