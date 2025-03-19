var _a;
import { u, al as p, R as c, am as h, an as E, L as m, h as f, i as s, q as g, b as T } from "./CmQi0fbH.js";
function v(r) {
  var t = document.createElement("template");
  return t.innerHTML = r, t.content;
}
function a(r, t) {
  var e = m;
  e.nodes_start === null && (e.nodes_start = r, e.nodes_end = t);
}
function N(r, t) {
  var e = (t & h) !== 0, _ = (t & E) !== 0, n, i = !r.startsWith("<!>");
  return () => {
    if (f) return a(s, null), s;
    n === void 0 && (n = v(i ? r : "<!>" + r), e || (n = u(n)));
    var o = _ || p ? document.importNode(n, true) : n.cloneNode(true);
    if (e) {
      var l = u(o), d = o.lastChild;
      a(l, d);
    } else a(o, o);
    return o;
  };
}
function x(r, t, e = "svg") {
  var _ = !r.startsWith("<!>"), n = `<${e}>${_ ? r : "<!>" + r}</${e}>`, i;
  return () => {
    if (f) return a(s, null), s;
    if (!i) {
      var o = v(n), l = u(o);
      i = u(l);
    }
    var d = i.cloneNode(true);
    return a(d, d), d;
  };
}
function L(r = "") {
  if (!f) {
    var t = c(r + "");
    return a(t, t), t;
  }
  var e = s;
  return e.nodeType !== 3 && (e.before(e = c()), g(e)), a(e, e), e;
}
function M() {
  if (f) return a(s, null), s;
  var r = document.createDocumentFragment(), t = document.createComment(""), e = c();
  return r.append(t, e), a(t, e), r;
}
function b(r, t) {
  if (f) {
    m.nodes_end = s, T();
    return;
  }
  r !== null && r.before(t);
}
const w = "5";
typeof window < "u" && ((_a = window.__svelte ?? (window.__svelte = {})).v ?? (_a.v = /* @__PURE__ */ new Set())).add(w);
export {
  b as a,
  a as b,
  v as c,
  M as d,
  L as e,
  x as n,
  N as t
};
