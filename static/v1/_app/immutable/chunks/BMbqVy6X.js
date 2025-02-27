import { h as p, an as E, M, ao as V, ap as G, B as k } from "./CvlvO1XB.js";
import { c as P, i as q, f as z, d as D, g as K, n as U, j as Y } from "./CTI4QPiR.js";
function S(s) {
  var r, t, i = "";
  if (typeof s == "string" || typeof s == "number") i += s;
  else if (typeof s == "object") if (Array.isArray(s)) {
    var e = s.length;
    for (r = 0; r < e; r++) s[r] && (t = S(s[r])) && (i && (i += " "), i += t);
  } else for (t in s) s[t] && (i && (i += " "), i += t);
  return i;
}
function F() {
  for (var s, r, t = 0, i = "", e = arguments.length; t < e; t++) (s = arguments[t]) && (r = S(s)) && (i && (i += " "), i += r);
  return i;
}
function H(s) {
  return typeof s == "object" ? F(s) : s ?? "";
}
const L = [...` 	
\r\f\xA0\v\uFEFF`];
function J(s, r, t) {
  var i = s == null ? "" : "" + s;
  if (r && (i = i ? i + " " + r : r), t) {
    for (var e in t) if (t[e]) i = i ? i + " " + e : e;
    else if (i.length) for (var l = e.length, _ = 0; (_ = i.indexOf(e, _)) >= 0; ) {
      var n = _ + l;
      (_ === 0 || L.includes(i[_ - 1])) && (n === i.length || L.includes(i[n])) ? i = (_ === 0 ? "" : i.substring(0, _)) + i.substring(n + 1) : _ = n;
    }
  }
  return i === "" ? null : i;
}
function Q(s, r, t, i, e, l) {
  var _ = s.__className;
  if (p || _ !== t) {
    var n = J(t, i, l);
    (!p || n !== s.getAttribute("class")) && (n == null ? s.removeAttribute("class") : r ? s.className = n : s.setAttribute("class", n)), s.__className = t;
  } else if (l) for (var o in l) {
    var g = !!l[o];
    (e == null || g !== !!e[o]) && s.classList.toggle(o, g);
  }
  return l;
}
const h = Symbol("class");
function x(s) {
  if (p) {
    var r = false, t = () => {
      if (!r) {
        if (r = true, s.hasAttribute("value")) {
          var i = s.value;
          y(s, "value", null), s.value = i;
        }
        if (s.hasAttribute("checked")) {
          var e = s.checked;
          y(s, "checked", null), s.checked = e;
        }
      }
    };
    s.__on_r = t, V(t), P();
  }
}
function m(s, r) {
  var t = s.__attributes ?? (s.__attributes = {});
  t.value === (t.value = r ?? void 0) || s.value === r && (r !== 0 || s.nodeName !== "PROGRESS") || (s.value = r ?? "");
}
function W(s, r) {
  r ? s.hasAttribute("selected") || s.setAttribute("selected", "") : s.removeAttribute("selected");
}
function y(s, r, t, i) {
  var e = s.__attributes ?? (s.__attributes = {});
  p && (e[r] = s.getAttribute(r), r === "src" || r === "srcset" || r === "href" && s.nodeName === "LINK") || e[r] !== (e[r] = t) && (r === "style" && "__styles" in s && (s.__styles = {}), r === "loading" && (s[E] = t), t == null ? s.removeAttribute(r) : typeof t != "string" && C(s).includes(r) ? s[r] = t : s.setAttribute(r, t));
}
function ss(s, r, t, i, e = false, l = false, _ = false) {
  let n = p && l;
  n && k(false);
  var o = r || {}, g = s.tagName === "OPTION";
  for (var b in r) b in t || (t[b] = null);
  t.class ? t.class = H(t.class) : (i || t[h]) && (t.class = null);
  var I = C(s), R = s.__attributes ?? (s.__attributes = {});
  for (const f in t) {
    let a = t[f];
    if (g && f === "value" && a == null) {
      s.value = s.__value = "", o[f] = a;
      continue;
    }
    if (f === "class") {
      var T = s.namespaceURI === "http://www.w3.org/1999/xhtml";
      Q(s, T, a, i, r == null ? void 0 : r[h], t[h]), o[f] = a, o[h] = t[h];
      continue;
    }
    var N = o[f];
    if (a !== N) {
      o[f] = a;
      var w = f[0] + f[1];
      if (w !== "$$") {
        if (w === "on") {
          const c = {}, d = "$$" + f;
          let u = f.slice(2);
          var A = Y(u);
          if (q(u) && (u = u.slice(0, -7), c.capture = true), !A && N) {
            if (a != null) continue;
            s.removeEventListener(u, o[d], c), o[d] = null;
          }
          if (a != null) if (A) s[`__${u}`] = a, D([u]);
          else {
            let j = function(B) {
              o[f].call(this, B);
            };
            o[d] = z(u, s, j, c);
          }
          else A && (s[`__${u}`] = void 0);
        } else if (f === "style" && a != null) s.style.cssText = a + "";
        else if (f === "autofocus") K(s, !!a);
        else if (!l && (f === "__value" || f === "value" && a != null)) s.value = s.__value = a;
        else if (f === "selected" && g) W(s, a);
        else {
          var v = f;
          e || (v = U(v));
          var $ = v === "defaultValue" || v === "defaultChecked";
          if (a == null && !l && !$) if (R[f] = null, v === "value" || v === "checked") {
            let c = s;
            const d = r === void 0;
            if (v === "value") {
              let u = c.defaultValue;
              c.removeAttribute(v), c.defaultValue = u, c.value = c.__value = d ? u : null;
            } else {
              let u = c.defaultChecked;
              c.removeAttribute(v), c.defaultChecked = u, c.checked = d ? u : false;
            }
          } else s.removeAttribute(f);
          else $ || I.includes(v) && (l || typeof a != "string") ? s[v] = a : typeof a != "function" && y(s, v, a);
        }
        f === "style" && "__styles" in s && (s.__styles = {});
      }
    }
  }
  return n && k(true), o;
}
var O = /* @__PURE__ */ new Map();
function C(s) {
  var r = O.get(s.nodeName);
  if (r) return r;
  O.set(s.nodeName, r = []);
  for (var t, i = s, e = Element.prototype; e !== i; ) {
    t = G(i);
    for (var l in t) t[l].set && r.push(l);
    i = M(i);
  }
  return r;
}
export {
  h as C,
  Q as a,
  m as b,
  H as c,
  ss as d,
  x as r,
  y as s
};
