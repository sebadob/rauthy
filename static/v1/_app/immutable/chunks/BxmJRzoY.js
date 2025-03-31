var _a;
import { u, ai as p, I as l, aj as h, ak as E, O as v, h as f, i as s, q as g, b as T } from "./w0HvPX0p.js";
function m(r) {
  var t = document.createElement("template");
  return t.innerHTML = r, t.content;
}
function a(r, t) {
  var e = v;
  e.nodes_start === null && (e.nodes_start = r, e.nodes_end = t);
}
function N(r, t) {
  var e = (t & h) !== 0, _ = (t & E) !== 0, n, i = !r.startsWith("<!>");
  return () => {
    if (f) return a(s, null), s;
    n === void 0 && (n = m(i ? r : "<!>" + r), e || (n = u(n)));
    var o = _ || p ? document.importNode(n, true) : n.cloneNode(true);
    if (e) {
      var c = u(o), d = o.lastChild;
      a(c, d);
    } else a(o, o);
    return o;
  };
}
function x(r, t, e = "svg") {
  var _ = !r.startsWith("<!>"), n = `<${e}>${_ ? r : "<!>" + r}</${e}>`, i;
  return () => {
    if (f) return a(s, null), s;
    if (!i) {
      var o = m(n), c = u(o);
      i = u(c);
    }
    var d = i.cloneNode(true);
    return a(d, d), d;
  };
}
function M(r = "") {
  if (!f) {
    var t = l(r + "");
    return a(t, t), t;
  }
  var e = s;
  return e.nodeType !== 3 && (e.before(e = l()), g(e)), a(e, e), e;
}
function b() {
  if (f) return a(s, null), s;
  var r = document.createDocumentFragment(), t = document.createComment(""), e = l();
  return r.append(t, e), a(t, e), r;
}
function I(r, t) {
  if (f) {
    v.nodes_end = s, T();
    return;
  }
  r !== null && r.before(t);
}
const w = "5";
typeof window < "u" && ((_a = window.__svelte ?? (window.__svelte = {})).v ?? (_a.v = /* @__PURE__ */ new Set())).add(w);
export {
  I as a,
  a as b,
  m as c,
  b as d,
  M as e,
  x as n,
  N as t
};
