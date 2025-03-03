import { t as v, a as T } from "./BH6NCLk-.js";
import { p as b, t as D, a as F, c as g, r as P } from "./CvlvO1XB.js";
import { e as R } from "./CTI4QPiR.js";
import { s as k } from "./-T201g_q.js";
import { s as l } from "./BMbqVy6X.js";
import { p as n } from "./C6SR4G2t.js";
import { f as y } from "./CeeD0cV_.js";
import { u as E } from "./DswDW5U8.js";
import { a as V, e as _ } from "./DYtiVhoA.js";
var x = v("<form><!></form>");
function H(u, t) {
  b(t, true);
  let d = n(t, "method", 3, "POST"), f = n(t, "isError", 15), h = n(t, "convertDateToTs", 19, () => []);
  async function w(c) {
    var _a;
    c.preventDefault();
    const a = c.currentTarget;
    if (a.reportValidity()) f(false);
    else {
      f(true);
      return;
    }
    const i = new FormData(a);
    for (let r of h()) {
      let m = ((_a = i.get(r)) == null ? void 0 : _a.toString()) || "", S = E(m) || "";
      i.set(r, (S == null ? void 0 : S.toString()) || "");
    }
    if (t.withPowAs) {
      let r = await y();
      i.set(t.withPowAs, r || "");
    }
    let o, s = new URLSearchParams();
    if (i.forEach((r, m) => {
      s.append(m, r.toString());
    }), t.onSubmit) {
      t.onSubmit(a, s);
      return;
    }
    o = await V(a, s), o && (o.ok ? t.onResponse && (t.onResponse(o), t.resetOnSubmit && o.ok && a.reset()) : await _(o));
  }
  var e = x(), p = g(e);
  k(p, () => t.children), P(e), D(() => {
    l(e, "action", t.action), l(e, "method", d());
  }), R("submit", e, w), T(u, e), F();
}
export {
  H as F
};
