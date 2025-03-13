import { t as R, a as M } from "./YHhP1LbZ.js";
import { p as J, c as v, s as D, r as c, t as E, a as K } from "./Ck6jKiur.js";
import { d as N, r as O, s as L, e as _ } from "./tDAaDMC_.js";
import { i as P } from "./7JDmqCCW.js";
import { a as Q, s as t, b as S } from "./BTaFr7HN.js";
import { t as T, s as W } from "./Bd2Rvcxs.js";
import { b as X } from "./0HgaTnX3.js";
import { b as Y } from "./BNlp6PFo.js";
import { p as r } from "./DZP54pO_.js";
function Z(o, e) {
  e();
}
function p(o, e, n) {
  var _a, _b, _c, _d;
  switch (e(), o.code) {
    case "ArrowUp":
      (_a = n.onUp) == null ? void 0 : _a.call(n);
      break;
    case "ArrowDown":
      (_b = n.onDown) == null ? void 0 : _b.call(n);
      break;
    case "ArrowLeft":
      (_c = n.onLeft) == null ? void 0 : _c.call(n);
      break;
    case "ArrowRight":
      (_d = n.onRight) == null ? void 0 : _d.call(n);
      break;
  }
}
var $ = R('<div class="error svelte-y71ny6"> </div>'), ee = R(`<div class="container svelte-y71ny6"><textarea autocomplete="off">
    </textarea> <div class="label svelte-y71ny6"><label class="font-label noselect svelte-y71ny6"> </label> <!></div></div>`);
function fe(o, e) {
  J(e, true);
  let n = r(e, "ref", 15), w = r(e, "value", 15, ""), y = r(e, "label", 3, ""), V = r(e, "rows", 3, 3), m = r(e, "placeholder", 3, ""), b = r(e, "errMsg", 3, ""), g = r(e, "disabled", 3, false), C = r(e, "fontMono", 3, false), h = r(e, "required", 3, false), l = r(e, "isError", 15, false), I = r(e, "width", 3, "inherit");
  const x = e.pattern ? new RegExp(e.pattern, "gm") : void 0;
  function U(i) {
    var _a;
    d(), (_a = e.onBlur) == null ? void 0 : _a.call(e);
  }
  function B(i) {
    i.preventDefault(), l(true);
  }
  function j(i) {
    d();
  }
  function d() {
    var _a, _b;
    return x && (w().match(x).length > 2 ? ((_a = n()) == null ? void 0 : _a.setCustomValidity(b()), l(true)) : ((_b = n()) == null ? void 0 : _b.setCustomValidity(""), l(false))), true;
  }
  var f = ee();
  let k;
  var a = v(f);
  O(a), a.__input = [Z, d], a.__keydown = [p, d, e];
  let q;
  Y(a, (i) => n(i), () => n());
  var A = D(a, 2), s = v(A), z = v(s, true);
  c(s);
  var F = D(s, 2);
  {
    var G = (i) => {
      var u = $(), H = v(u, true);
      c(u), E(() => L(H, b())), T(3, u, () => W), M(i, u);
    };
    P(F, (i) => {
      l() && i(G);
    });
  }
  c(A), c(f), E((i) => {
    k = Q(f, "", k, { width: I() }), t(a, "id", e.id), t(a, "name", e.name), t(a, "title", b()), t(a, "aria-label", y() || m()), t(a, "rows", V()), t(a, "placeholder", m()), t(a, "aria-placeholder", m()), a.disabled = g(), t(a, "aria-disabled", g()), t(a, "maxlength", e.maxLength || void 0), a.required = h() || void 0, t(a, "aria-required", h() || false), t(a, "aria-invalid", l()), q = S(a, 1, "svelte-y71ny6", null, q, i), t(s, "for", e.id), t(s, "data-required", h()), L(z, y());
  }, [() => ({ "font-mono": C() })]), _("submit", a, j), _("invalid", a, B), _("blur", a, U), X(a, w), M(o, f), K();
}
N(["input", "keydown"]);
export {
  fe as I
};
