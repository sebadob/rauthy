import { h as p, ao as Y, N as y, ap as z, aq as D, C as w, ar as K } from "./BveBAmlr.js";
import { i as F, c as J, d as Q, f as W, n as X, g as Z, j as m } from "./CYCba2oX.js";
function U(r) {
  var i, f, s = "";
  if (typeof r == "string" || typeof r == "number") s += r;
  else if (typeof r == "object") if (Array.isArray(r)) {
    var t = r.length;
    for (i = 0; i < t; i++) r[i] && (f = U(r[i])) && (s && (s += " "), s += f);
  } else for (f in r) r[f] && (s && (s += " "), s += f);
  return s;
}
function x() {
  for (var r, i, f = 0, s = "", t = arguments.length; f < t; f++) (r = arguments[f]) && (i = U(r)) && (s && (s += " "), s += i);
  return s;
}
function rr(r) {
  return typeof r == "object" ? x(r) : r ?? "";
}
const P = [...` 	
\r\f\xA0\v\uFEFF`];
function fr(r, i, f) {
  var s = r == null ? "" : "" + r;
  if (i && (s = s ? s + " " + i : i), f) {
    for (var t in f) if (f[t]) s = s ? s + " " + t : t;
    else if (s.length) for (var u = t.length, l = 0; (l = s.indexOf(t, l)) >= 0; ) {
      var c = l + u;
      (l === 0 || P.includes(s[l - 1])) && (c === s.length || P.includes(s[c])) ? s = (l === 0 ? "" : s.substring(0, l)) + s.substring(c + 1) : l = c;
    }
  }
  return s === "" ? null : s;
}
function R(r, i = false) {
  var f = i ? " !important;" : ";", s = "";
  for (var t in r) {
    var u = r[t];
    u != null && u !== "" && (s += " " + t + ": " + u + f);
  }
  return s;
}
function I(r) {
  return r[0] !== "-" || r[1] !== "-" ? r.toLowerCase() : r;
}
function ir(r, i) {
  if (i) {
    var f = "", s, t;
    if (Array.isArray(i) ? (s = i[0], t = i[1]) : s = i, r) {
      r = String(r).replaceAll(/\s*\/\*.*?\*\/\s*/g, "").trim();
      var u = false, l = 0, c = false, A = [];
      s && A.push(...Object.keys(s).map(I)), t && A.push(...Object.keys(t).map(I));
      var o = 0, b = -1;
      const S = r.length;
      for (var n = 0; n < S; n++) {
        var g = r[n];
        if (c ? g === "/" && r[n - 1] === "*" && (c = false) : u ? u === g && (u = false) : g === "/" && r[n + 1] === "*" ? c = true : g === '"' || g === "'" ? u = g : g === "(" ? l++ : g === ")" && l--, !c && u === false && l === 0) {
          if (g === ":" && b === -1) b = n;
          else if (g === ";" || n === S - 1) {
            if (b !== -1) {
              var O = I(r.substring(o, b).trim());
              if (!A.includes(O)) {
                g !== ";" && n++;
                var C = r.substring(o, n).trim();
                f += " " + C + ";";
              }
            }
            o = n + 1, b = -1;
          }
        }
      }
    }
    return s && (f += R(s)), t && (f += R(t, true)), f = f.trim(), f === "" ? null : f;
  }
  return r == null ? null : String(r);
}
function sr(r, i, f, s, t, u) {
  var l = r.__className;
  if (p || l !== f) {
    var c = fr(f, s, u);
    (!p || c !== r.getAttribute("class")) && (c == null ? r.removeAttribute("class") : i ? r.className = c : r.setAttribute("class", c)), r.__className = f;
  } else if (u && t !== u) for (var A in u) {
    var o = !!u[A];
    (t == null || o !== !!t[A]) && r.classList.toggle(A, o);
  }
  return u;
}
function M(r, i = {}, f, s) {
  for (var t in f) {
    var u = f[t];
    i[t] !== u && (f[t] == null ? r.style.removeProperty(t) : r.style.setProperty(t, u, s));
  }
}
function tr(r, i, f, s) {
  var t = r.__style;
  if (p || t !== i) {
    var u = ir(i, s);
    (!p || u !== r.getAttribute("style")) && (u == null ? r.removeAttribute("style") : r.style.cssText = u), r.__style = i;
  } else s && (Array.isArray(s) ? (M(r, f == null ? void 0 : f[0], s[0]), M(r, f == null ? void 0 : f[1], s[1], "important")) : M(r, f, s));
  return s;
}
const N = Symbol("class"), L = Symbol("style"), V = Symbol("is custom element"), q = Symbol("is html");
function lr(r) {
  if (p) {
    var i = false, f = () => {
      if (!i) {
        if (i = true, r.hasAttribute("value")) {
          var s = r.value;
          E(r, "value", null), r.value = s;
        }
        if (r.hasAttribute("checked")) {
          var t = r.checked;
          E(r, "checked", null), r.checked = t;
        }
      }
    };
    r.__on_r = f, z(f), Z();
  }
}
function or(r, i) {
  var f = $(r);
  f.value === (f.value = i ?? void 0) || r.value === i && (i !== 0 || r.nodeName !== "PROGRESS") || (r.value = i ?? "");
}
function ur(r, i) {
  i ? r.hasAttribute("selected") || r.setAttribute("selected", "") : r.removeAttribute("selected");
}
function E(r, i, f, s) {
  var t = $(r);
  p && (t[i] = r.getAttribute(i), i === "src" || i === "srcset" || i === "href" && r.nodeName === "LINK") || t[i] !== (t[i] = f) && (i === "loading" && (r[D] = f), f == null ? r.removeAttribute(i) : typeof f != "string" && B(r).includes(i) ? r[i] = f : r.setAttribute(i, f));
}
function cr(r, i, f, s, t = false) {
  var u = $(r), l = u[V], c = !u[q];
  let A = p && l;
  A && w(false);
  var o = i || {}, b = r.tagName === "OPTION";
  for (var n in i) n in f || (f[n] = null);
  f.class ? f.class = rr(f.class) : (s || f[N]) && (f.class = null), f[L] && (f.style ?? (f.style = null));
  var g = B(r);
  for (const a in f) {
    let e = f[a];
    if (b && a === "value" && e == null) {
      r.value = r.__value = "", o[a] = e;
      continue;
    }
    if (a === "class") {
      var O = r.namespaceURI === "http://www.w3.org/1999/xhtml";
      sr(r, O, e, s, i == null ? void 0 : i[N], f[N]), o[a] = e, o[N] = f[N];
      continue;
    }
    if (a === "style") {
      tr(r, e, i == null ? void 0 : i[L], f[L]), o[a] = e, o[L] = f[L];
      continue;
    }
    var C = o[a];
    if (e !== C) {
      o[a] = e;
      var S = a[0] + a[1];
      if (S !== "$$") if (S === "on") {
        const d = {}, _ = "$$" + a;
        let v = a.slice(2);
        var T = m(v);
        if (F(v) && (v = v.slice(0, -7), d.capture = true), !T && C) {
          if (e != null) continue;
          r.removeEventListener(v, o[_], d), o[_] = null;
        }
        if (e != null) if (T) r[`__${v}`] = e, Q([v]);
        else {
          let G = function(H) {
            o[a].call(this, H);
          };
          o[_] = J(v, r, G, d);
        }
        else T && (r[`__${v}`] = void 0);
      } else if (a === "style") E(r, a, e);
      else if (a === "autofocus") W(r, !!e);
      else if (!l && (a === "__value" || a === "value" && e != null)) r.value = r.__value = e;
      else if (a === "selected" && b) ur(r, e);
      else {
        var h = a;
        c || (h = X(h));
        var j = h === "defaultValue" || h === "defaultChecked";
        if (e == null && !l && !j) if (u[a] = null, h === "value" || h === "checked") {
          let d = r;
          const _ = i === void 0;
          if (h === "value") {
            let v = d.defaultValue;
            d.removeAttribute(h), d.defaultValue = v, d.value = d.__value = _ ? v : null;
          } else {
            let v = d.defaultChecked;
            d.removeAttribute(h), d.defaultChecked = v, d.checked = _ ? v : false;
          }
        } else r.removeAttribute(a);
        else j || g.includes(h) && (l || typeof e != "string") ? r[h] = e : typeof e != "function" && E(r, h, e);
      }
    }
  }
  return A && w(true), o;
}
function $(r) {
  return r.__attributes ?? (r.__attributes = { [V]: r.nodeName.includes("-"), [q]: r.namespaceURI === Y });
}
var k = /* @__PURE__ */ new Map();
function B(r) {
  var i = k.get(r.nodeName);
  if (i) return i;
  k.set(r.nodeName, i = []);
  for (var f, s = r, t = Element.prototype; t !== s; ) {
    f = K(s);
    for (var u in f) f[u].set && i.push(u);
    s = y(s);
  }
  return i;
}
export {
  N as C,
  L as S,
  tr as a,
  sr as b,
  rr as c,
  or as d,
  cr as e,
  lr as r,
  E as s
};
