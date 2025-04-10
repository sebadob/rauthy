var __typeError = (msg) => {
  throw TypeError(msg);
};
var __accessCheck = (obj, member, msg) => member.has(obj) || __typeError("Cannot " + msg);
var __privateGet = (obj, member, getter) => (__accessCheck(obj, member, "read from private field"), getter ? getter.call(obj) : member.get(obj));
var __privateAdd = (obj, member, value) => member.has(obj) ? __typeError("Cannot add the same private member more than once") : member instanceof WeakSet ? member.add(obj) : member.set(obj, value);
var _a, _b, _e2, _t2, _n2, _r, _a2, _o, _s, _i, _c, _e3, _d, _e4, _e5;
import { a6 as _e, aI as _t, a0 as $e, j as T, k as x, l as P, at as mt } from "./w0HvPX0p.js";
const F = [];
function Se(e, t = _e) {
  let n = null;
  const r = /* @__PURE__ */ new Set();
  function a(o) {
    if (_t(e, o) && (e = o, n)) {
      const c = !F.length;
      for (const l of r) l[1](), F.push(l, e);
      if (c) {
        for (let l = 0; l < F.length; l += 2) F[l][0](F[l + 1]);
        F.length = 0;
      }
    }
  }
  function s(o) {
    a(o(e));
  }
  function i(o, c = _e) {
    const l = [o, c];
    return r.add(l), r.size === 1 && (n = t(a, s) || _e), o(e), () => {
      r.delete(l), r.size === 0 && n && (n(), n = null);
    };
  }
  return { set: a, update: s, subscribe: i };
}
new URL("sveltekit-internal://");
function wt(e, t) {
  return e === "/" || t === "ignore" ? e : t === "never" ? e.endsWith("/") ? e.slice(0, -1) : e : t === "always" && !e.endsWith("/") ? e + "/" : e;
}
function yt(e) {
  return e.split("%25").map(decodeURI).join("%25");
}
function vt(e) {
  for (const t in e) e[t] = decodeURIComponent(e[t]);
  return e;
}
function me({ href: e }) {
  return e.split("#")[0];
}
function bt(e, t, n, r = false) {
  const a = new URL(e);
  Object.defineProperty(a, "searchParams", { value: new Proxy(a.searchParams, { get(i, o) {
    if (o === "get" || o === "getAll" || o === "has") return (l) => (n(l), i[o](l));
    t();
    const c = Reflect.get(i, o);
    return typeof c == "function" ? c.bind(i) : c;
  } }), enumerable: true, configurable: true });
  const s = ["href", "pathname", "search", "toString", "toJSON"];
  r && s.push("hash");
  for (const i of s) Object.defineProperty(a, i, { get() {
    return t(), e[i];
  }, enumerable: true, configurable: true });
  return a;
}
function kt(...e) {
  let t = 5381;
  for (const n of e) if (typeof n == "string") {
    let r = n.length;
    for (; r; ) t = t * 33 ^ n.charCodeAt(--r);
  } else if (ArrayBuffer.isView(n)) {
    const r = new Uint8Array(n.buffer, n.byteOffset, n.byteLength);
    let a = r.length;
    for (; a; ) t = t * 33 ^ r[--a];
  } else throw new TypeError("value must be a string or TypedArray");
  return (t >>> 0).toString(36);
}
function At(e) {
  const t = atob(e), n = new Uint8Array(t.length);
  for (let r = 0; r < t.length; r++) n[r] = t.charCodeAt(r);
  return n.buffer;
}
const St = window.fetch;
window.fetch = (e, t) => ((e instanceof Request ? e.method : (t == null ? void 0 : t.method) || "GET") !== "GET" && H.delete(Ee(e)), St(e, t));
const H = /* @__PURE__ */ new Map();
function Et(e, t) {
  const n = Ee(e, t), r = document.querySelector(n);
  if (r == null ? void 0 : r.textContent) {
    let { body: a, ...s } = JSON.parse(r.textContent);
    const i = r.getAttribute("data-ttl");
    return i && H.set(n, { body: a, init: s, ttl: 1e3 * Number(i) }), r.getAttribute("data-b64") !== null && (a = At(a)), Promise.resolve(new Response(a, s));
  }
  return window.fetch(e, t);
}
function Rt(e, t, n) {
  if (H.size > 0) {
    const r = Ee(e, n), a = H.get(r);
    if (a) {
      if (performance.now() < a.ttl && ["default", "force-cache", "only-if-cached", void 0].includes(n == null ? void 0 : n.cache)) return new Response(a.body, a.init);
      H.delete(r);
    }
  }
  return window.fetch(t, n);
}
function Ee(e, t) {
  let r = `script[data-sveltekit-fetched][data-url=${JSON.stringify(e instanceof Request ? e.url : e)}]`;
  if ((t == null ? void 0 : t.headers) || (t == null ? void 0 : t.body)) {
    const a = [];
    t.headers && a.push([...new Headers(t.headers)].join(",")), t.body && (typeof t.body == "string" || ArrayBuffer.isView(t.body)) && a.push(t.body), r += `[data-hash="${kt(...a)}"]`;
  }
  return r;
}
const It = /^(\[)?(\.\.\.)?(\w+)(?:=(\w+))?(\])?$/;
function Ut(e) {
  const t = [];
  return { pattern: e === "/" ? /^\/$/ : new RegExp(`^${Tt(e).map((r) => {
    const a = /^\[\.\.\.(\w+)(?:=(\w+))?\]$/.exec(r);
    if (a) return t.push({ name: a[1], matcher: a[2], optional: false, rest: true, chained: true }), "(?:/(.*))?";
    const s = /^\[\[(\w+)(?:=(\w+))?\]\]$/.exec(r);
    if (s) return t.push({ name: s[1], matcher: s[2], optional: true, rest: false, chained: true }), "(?:/([^/]+))?";
    if (!r) return;
    const i = r.split(/\[(.+?)\](?!\])/);
    return "/" + i.map((c, l) => {
      if (l % 2) {
        if (c.startsWith("x+")) return we(String.fromCharCode(parseInt(c.slice(2), 16)));
        if (c.startsWith("u+")) return we(String.fromCharCode(...c.slice(2).split("-").map((m) => parseInt(m, 16))));
        const u = It.exec(c), [, h, d, f, p] = u;
        return t.push({ name: f, matcher: p, optional: !!h, rest: !!d, chained: d ? l === 1 && i[0] === "" : false }), d ? "(.*?)" : h ? "([^/]*)?" : "([^/]+?)";
      }
      return we(c);
    }).join("");
  }).join("")}/?$`), params: t };
}
function Lt(e) {
  return !/^\([^)]+\)$/.test(e);
}
function Tt(e) {
  return e.slice(1).split("/").filter(Lt);
}
function xt(e, t, n) {
  const r = {}, a = e.slice(1), s = a.filter((o) => o !== void 0);
  let i = 0;
  for (let o = 0; o < t.length; o += 1) {
    const c = t[o];
    let l = a[o - i];
    if (c.chained && c.rest && i && (l = a.slice(o - i, o + 1).filter((u) => u).join("/"), i = 0), l === void 0) {
      c.rest && (r[c.name] = "");
      continue;
    }
    if (!c.matcher || n[c.matcher](l)) {
      r[c.name] = l;
      const u = t[o + 1], h = a[o + 1];
      u && !u.rest && u.optional && h && c.chained && (i = 0), !u && !h && Object.keys(r).length === s.length && (i = 0);
      continue;
    }
    if (c.optional && c.chained) {
      i++;
      continue;
    }
    return;
  }
  if (!i) return r;
}
function we(e) {
  return e.normalize().replace(/[[\]]/g, "\\$&").replace(/%/g, "%25").replace(/\//g, "%2[Ff]").replace(/\?/g, "%3[Ff]").replace(/#/g, "%23").replace(/[.*+?^${}()|\\]/g, "\\$&");
}
function Pt({ nodes: e, server_loads: t, dictionary: n, matchers: r }) {
  const a = new Set(t);
  return Object.entries(n).map(([o, [c, l, u]]) => {
    const { pattern: h, params: d } = Ut(o), f = { id: o, exec: (p) => {
      const m = h.exec(p);
      if (m) return xt(m, d, r);
    }, errors: [1, ...u || []].map((p) => e[p]), layouts: [0, ...l || []].map(i), leaf: s(c) };
    return f.errors.length = f.layouts.length = Math.max(f.errors.length, f.layouts.length), f;
  });
  function s(o) {
    const c = o < 0;
    return c && (o = ~o), [c, e[o]];
  }
  function i(o) {
    return o === void 0 ? o : [a.has(o), e[o]];
  }
}
function We(e, t = JSON.parse) {
  try {
    return t(sessionStorage[e]);
  } catch {
  }
}
function De(e, t, n = JSON.stringify) {
  const r = n(t);
  try {
    sessionStorage[e] = r;
  } catch {
  }
}
const U = ((_a = globalThis.__sveltekit_26h21r) == null ? void 0 : _a.base) ?? "/auth/v1", Ct = ((_b = globalThis.__sveltekit_26h21r) == null ? void 0 : _b.assets) ?? U, Ot = "1744269846218", Ye = "sveltekit:snapshot", ze = "sveltekit:scroll", Je = "sveltekit:states", Nt = "sveltekit:pageurl", B = "sveltekit:history", Y = "sveltekit:navigation", j = { tap: 1, hover: 2, viewport: 3, eager: 4, off: -1, false: -1 }, Z = location.origin;
function Xe(e) {
  if (e instanceof URL) return e;
  let t = document.baseURI;
  if (!t) {
    const n = document.getElementsByTagName("base");
    t = n.length ? n[0].href : document.URL;
  }
  return new URL(e, t);
}
function Re() {
  return { x: pageXOffset, y: pageYOffset };
}
function V(e, t) {
  return e.getAttribute(`data-sveltekit-${t}`);
}
const Fe = { ...j, "": j.hover };
function Ze(e) {
  let t = e.assignedSlot ?? e.parentNode;
  return (t == null ? void 0 : t.nodeType) === 11 && (t = t.host), t;
}
function Qe(e, t) {
  for (; e && e !== t; ) {
    if (e.nodeName.toUpperCase() === "A" && e.hasAttribute("href")) return e;
    e = Ze(e);
  }
}
function be(e, t, n) {
  let r;
  try {
    if (r = new URL(e instanceof SVGAElement ? e.href.baseVal : e.href, document.baseURI), n && r.hash.match(/^#[^/]/)) {
      const o = location.hash.split("#")[1] || "/";
      r.hash = `#${o}${r.hash}`;
    }
  } catch {
  }
  const a = e instanceof SVGAElement ? e.target.baseVal : e.target, s = !r || !!a || fe(r, t, n) || (e.getAttribute("rel") || "").split(/\s+/).includes("external"), i = (r == null ? void 0 : r.origin) === Z && e.hasAttribute("download");
  return { url: r, external: s, target: a, download: i };
}
function ne(e) {
  let t = null, n = null, r = null, a = null, s = null, i = null, o = e;
  for (; o && o !== document.documentElement; ) r === null && (r = V(o, "preload-code")), a === null && (a = V(o, "preload-data")), t === null && (t = V(o, "keepfocus")), n === null && (n = V(o, "noscroll")), s === null && (s = V(o, "reload")), i === null && (i = V(o, "replacestate")), o = Ze(o);
  function c(l) {
    switch (l) {
      case "":
      case "true":
        return true;
      case "off":
      case "false":
        return false;
      default:
        return;
    }
  }
  return { preload_code: Fe[r ?? "off"], preload_data: Fe[a ?? "off"], keepfocus: c(t), noscroll: c(n), reload: c(s), replace_state: c(i) };
}
function Ve(e) {
  const t = Se(e);
  let n = true;
  function r() {
    n = true, t.update((i) => i);
  }
  function a(i) {
    n = false, t.set(i);
  }
  function s(i) {
    let o;
    return t.subscribe((c) => {
      (o === void 0 || n && c !== o) && i(o = c);
    });
  }
  return { notify: r, set: a, subscribe: s };
}
const et = { v: () => {
} };
function jt() {
  const { set: e, subscribe: t } = Se(false);
  let n;
  async function r() {
    clearTimeout(n);
    try {
      const a = await fetch(`${Ct}/_app/version.json`, { headers: { pragma: "no-cache", "cache-control": "no-cache" } });
      if (!a.ok) return false;
      const i = (await a.json()).version !== Ot;
      return i && (e(true), et.v(), clearTimeout(n)), i;
    } catch {
      return false;
    }
  }
  return { subscribe: t, check: r };
}
function fe(e, t, n) {
  return e.origin !== Z || !e.pathname.startsWith(t) ? true : n ? !(e.pathname === t + "/" || e.pathname === t + "/index.html" || e.protocol === "file:" && e.pathname.replace(/\/[^/]+\.html?$/, "") === t) : false;
}
function vn(e) {
}
function Be(e) {
  const t = Dt(e), n = new ArrayBuffer(t.length), r = new DataView(n);
  for (let a = 0; a < n.byteLength; a++) r.setUint8(a, t.charCodeAt(a));
  return n;
}
const $t = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
function Dt(e) {
  e.length % 4 === 0 && (e = e.replace(/==?$/, ""));
  let t = "", n = 0, r = 0;
  for (let a = 0; a < e.length; a++) n <<= 6, n |= $t.indexOf(e[a]), r += 6, r === 24 && (t += String.fromCharCode((n & 16711680) >> 16), t += String.fromCharCode((n & 65280) >> 8), t += String.fromCharCode(n & 255), n = r = 0);
  return r === 12 ? (n >>= 4, t += String.fromCharCode(n)) : r === 18 && (n >>= 2, t += String.fromCharCode((n & 65280) >> 8), t += String.fromCharCode(n & 255)), t;
}
const Ft = -1, Vt = -2, Bt = -3, qt = -4, Mt = -5, Gt = -6;
function Ht(e, t) {
  if (typeof e == "number") return a(e, true);
  if (!Array.isArray(e) || e.length === 0) throw new Error("Invalid input");
  const n = e, r = Array(n.length);
  function a(s, i = false) {
    if (s === Ft) return;
    if (s === Bt) return NaN;
    if (s === qt) return 1 / 0;
    if (s === Mt) return -1 / 0;
    if (s === Gt) return -0;
    if (i) throw new Error("Invalid input");
    if (s in r) return r[s];
    const o = n[s];
    if (!o || typeof o != "object") r[s] = o;
    else if (Array.isArray(o)) if (typeof o[0] == "string") {
      const c = o[0], l = t == null ? void 0 : t[c];
      if (l) return r[s] = l(a(o[1]));
      switch (c) {
        case "Date":
          r[s] = new Date(o[1]);
          break;
        case "Set":
          const u = /* @__PURE__ */ new Set();
          r[s] = u;
          for (let f = 1; f < o.length; f += 1) u.add(a(o[f]));
          break;
        case "Map":
          const h = /* @__PURE__ */ new Map();
          r[s] = h;
          for (let f = 1; f < o.length; f += 2) h.set(a(o[f]), a(o[f + 1]));
          break;
        case "RegExp":
          r[s] = new RegExp(o[1], o[2]);
          break;
        case "Object":
          r[s] = Object(o[1]);
          break;
        case "BigInt":
          r[s] = BigInt(o[1]);
          break;
        case "null":
          const d = /* @__PURE__ */ Object.create(null);
          r[s] = d;
          for (let f = 1; f < o.length; f += 2) d[o[f]] = a(o[f + 1]);
          break;
        case "Int8Array":
        case "Uint8Array":
        case "Uint8ClampedArray":
        case "Int16Array":
        case "Uint16Array":
        case "Int32Array":
        case "Uint32Array":
        case "Float32Array":
        case "Float64Array":
        case "BigInt64Array":
        case "BigUint64Array": {
          const f = globalThis[c], p = o[1], m = Be(p), _ = new f(m);
          r[s] = _;
          break;
        }
        case "ArrayBuffer": {
          const f = o[1], p = Be(f);
          r[s] = p;
          break;
        }
        default:
          throw new Error(`Unknown type ${c}`);
      }
    } else {
      const c = new Array(o.length);
      r[s] = c;
      for (let l = 0; l < o.length; l += 1) {
        const u = o[l];
        u !== Vt && (c[l] = a(u));
      }
    }
    else {
      const c = {};
      r[s] = c;
      for (const l in o) {
        const u = o[l];
        c[l] = a(u);
      }
    }
    return r[s];
  }
  return a(0);
}
const tt = /* @__PURE__ */ new Set(["load", "prerender", "csr", "ssr", "trailingSlash", "config"]);
[...tt];
const Kt = /* @__PURE__ */ new Set([...tt]);
[...Kt];
function Wt(e) {
  return e.filter((t) => t != null);
}
class ue {
  constructor(t, n) {
    this.status = t, typeof n == "string" ? this.body = { message: n } : n ? this.body = n : this.body = { message: `Error: ${t}` };
  }
  toString() {
    return JSON.stringify(this.body);
  }
}
class Ie {
  constructor(t, n) {
    this.status = t, this.location = n;
  }
}
class Ue extends Error {
  constructor(t, n, r) {
    super(r), this.status = t, this.text = n;
  }
}
const Yt = "x-sveltekit-invalidated", zt = "x-sveltekit-trailing-slash";
function re(e) {
  return e instanceof ue || e instanceof Ue ? e.status : 500;
}
function Jt(e) {
  return e instanceof Ue ? e.text : "Internal Error";
}
let S, z, ye;
const Xt = $e.toString().includes("$$") || /function \w+\(\) \{\}/.test($e.toString());
Xt ? (S = { data: {}, form: null, error: null, params: {}, route: { id: null }, state: {}, status: -1, url: new URL("https://example.com") }, z = { current: null }, ye = { current: false }) : (S = new (_c = class {
  constructor() {
    __privateAdd(this, _e2, T({}));
    __privateAdd(this, _t2, T(null));
    __privateAdd(this, _n2, T(null));
    __privateAdd(this, _r, T({}));
    __privateAdd(this, _a2, T({ id: null }));
    __privateAdd(this, _o, T({}));
    __privateAdd(this, _s, T(-1));
    __privateAdd(this, _i, T(new URL("https://example.com")));
  }
  get data() {
    return x(__privateGet(this, _e2));
  }
  set data(t) {
    P(__privateGet(this, _e2), t);
  }
  get form() {
    return x(__privateGet(this, _t2));
  }
  set form(t) {
    P(__privateGet(this, _t2), t);
  }
  get error() {
    return x(__privateGet(this, _n2));
  }
  set error(t) {
    P(__privateGet(this, _n2), t);
  }
  get params() {
    return x(__privateGet(this, _r));
  }
  set params(t) {
    P(__privateGet(this, _r), t);
  }
  get route() {
    return x(__privateGet(this, _a2));
  }
  set route(t) {
    P(__privateGet(this, _a2), t);
  }
  get state() {
    return x(__privateGet(this, _o));
  }
  set state(t) {
    P(__privateGet(this, _o), t);
  }
  get status() {
    return x(__privateGet(this, _s));
  }
  set status(t) {
    P(__privateGet(this, _s), t);
  }
  get url() {
    return x(__privateGet(this, _i));
  }
  set url(t) {
    P(__privateGet(this, _i), t);
  }
}, _e2 = new WeakMap(), _t2 = new WeakMap(), _n2 = new WeakMap(), _r = new WeakMap(), _a2 = new WeakMap(), _o = new WeakMap(), _s = new WeakMap(), _i = new WeakMap(), _c)(), z = new (_d = class {
  constructor() {
    __privateAdd(this, _e3, T(null));
  }
  get current() {
    return x(__privateGet(this, _e3));
  }
  set current(t) {
    P(__privateGet(this, _e3), t);
  }
}, _e3 = new WeakMap(), _d)(), ye = new (_e5 = class {
  constructor() {
    __privateAdd(this, _e4, T(false));
  }
  get current() {
    return x(__privateGet(this, _e4));
  }
  set current(t) {
    P(__privateGet(this, _e4), t);
  }
}, _e4 = new WeakMap(), _e5)(), et.v = () => ye.current = true);
function Zt(e) {
  Object.assign(S, e);
}
const Qt = "/__data.json", en = ".html__data.json";
function tn(e) {
  return e.endsWith(".html") ? e.replace(/\.html$/, en) : e.replace(/\/$/, "") + Qt;
}
const nn = /* @__PURE__ */ new Set(["icon", "shortcut icon", "apple-touch-icon"]), D = We(ze) ?? {}, J = We(Ye) ?? {}, N = { url: Ve({}), page: Ve({}), navigating: Se(null), updated: jt() };
function Le(e) {
  D[e] = Re();
}
function rn(e, t) {
  let n = e + 1;
  for (; D[n]; ) delete D[n], n += 1;
  for (n = t + 1; J[n]; ) delete J[n], n += 1;
}
function M(e) {
  return location.href = e.href, new Promise(() => {
  });
}
async function nt() {
  if ("serviceWorker" in navigator) {
    const e = await navigator.serviceWorker.getRegistration(U || "/");
    e && await e.update();
  }
}
function qe() {
}
let Te, ke, ae, C, Ae, v;
const oe = [], se = [];
let O = null;
const ee = /* @__PURE__ */ new Map(), rt = /* @__PURE__ */ new Set(), an = /* @__PURE__ */ new Set(), K = /* @__PURE__ */ new Set();
let y = { branch: [], error: null, url: null }, xe = false, ie = false, Me = true, X = false, G = false, at = false, Pe = false, ot, k, I, $;
const W = /* @__PURE__ */ new Set();
async function Sn(e, t, n) {
  var _a3, _b2, _c2, _d2;
  document.URL !== location.href && (location.href = location.href), v = e, await ((_b2 = (_a3 = e.hooks).init) == null ? void 0 : _b2.call(_a3)), Te = Pt(e), C = document.documentElement, Ae = t, ke = e.nodes[0], ae = e.nodes[1], ke(), ae(), k = (_c2 = history.state) == null ? void 0 : _c2[B], I = (_d2 = history.state) == null ? void 0 : _d2[Y], k || (k = I = Date.now(), history.replaceState({ ...history.state, [B]: k, [Y]: I }, ""));
  const r = D[k];
  r && (history.scrollRestoration = "manual", scrollTo(r.x, r.y)), n ? await _n(Ae, n) : await hn(v.hash ? wn(new URL(location.href)) : location.href, { replaceState: true }), gn();
}
function on() {
  oe.length = 0, Pe = false;
}
function st(e) {
  se.some((t) => t == null ? void 0 : t.snapshot) && (J[e] = se.map((t) => {
    var _a3;
    return (_a3 = t == null ? void 0 : t.snapshot) == null ? void 0 : _a3.capture();
  }));
}
function it(e) {
  var _a3;
  (_a3 = J[e]) == null ? void 0 : _a3.forEach((t, n) => {
    var _a4, _b2;
    (_b2 = (_a4 = se[n]) == null ? void 0 : _a4.snapshot) == null ? void 0 : _b2.restore(t);
  });
}
function Ge() {
  Le(k), De(ze, D), st(I), De(Ye, J);
}
async function Ce(e, t, n, r) {
  return te({ type: "goto", url: Xe(e), keepfocus: t.keepFocus, noscroll: t.noScroll, replace_state: t.replaceState, state: t.state, redirect_count: n, nav_token: r, accept: () => {
    t.invalidateAll && (Pe = true), t.invalidate && t.invalidate.forEach(pn);
  } });
}
async function sn(e) {
  if (e.id !== (O == null ? void 0 : O.id)) {
    const t = {};
    W.add(t), O = { id: e.id, token: t, promise: ft({ ...e, preload: t }).then((n) => (W.delete(t), n.type === "loaded" && n.state.error && (O = null), n)) };
  }
  return O.promise;
}
async function ve(e) {
  var _a3;
  const t = (_a3 = await he(e, false)) == null ? void 0 : _a3.route;
  t && await Promise.all([...t.layouts, t.leaf].map((n) => n == null ? void 0 : n[1]()));
}
function ct(e, t, n) {
  var _a3;
  y = e.state;
  const r = document.querySelector("style[data-sveltekit]");
  r && r.remove(), Object.assign(S, e.props.page), ot = new v.root({ target: t, props: { ...e.props, stores: N, components: se }, hydrate: n, sync: false }), it(I);
  const a = { from: null, to: { params: y.params, route: { id: ((_a3 = y.route) == null ? void 0 : _a3.id) ?? null }, url: new URL(location.href) }, willUnload: false, type: "enter", complete: Promise.resolve() };
  K.forEach((s) => s(a)), ie = true;
}
function ce({ url: e, params: t, branch: n, status: r, error: a, route: s, form: i }) {
  let o = "never";
  if (U && (e.pathname === U || e.pathname === U + "/")) o = "always";
  else for (const f of n) (f == null ? void 0 : f.slash) !== void 0 && (o = f.slash);
  e.pathname = wt(e.pathname, o), e.search = e.search;
  const c = { type: "loaded", state: { url: e, params: t, branch: n, error: a, route: s }, props: { constructors: Wt(n).map((f) => f.node.component), page: je(S) } };
  i !== void 0 && (c.props.form = i);
  let l = {}, u = !S, h = 0;
  for (let f = 0; f < Math.max(n.length, y.branch.length); f += 1) {
    const p = n[f], m = y.branch[f];
    (p == null ? void 0 : p.data) !== (m == null ? void 0 : m.data) && (u = true), p && (l = { ...l, ...p.data }, u && (c.props[`data_${h}`] = l), h += 1);
  }
  return (!y.url || e.href !== y.url.href || y.error !== a || i !== void 0 && i !== S.form || u) && (c.props.page = { error: a, params: t, route: { id: (s == null ? void 0 : s.id) ?? null }, state: {}, status: r, url: new URL(e), form: i ?? null, data: u ? l : S.data }), c;
}
async function Oe({ loader: e, parent: t, url: n, params: r, route: a, server_data_node: s }) {
  var _a3, _b2, _c2;
  let i = null, o = true;
  const c = { dependencies: /* @__PURE__ */ new Set(), params: /* @__PURE__ */ new Set(), parent: false, route: false, url: false, search_params: /* @__PURE__ */ new Set() }, l = await e();
  if ((_a3 = l.universal) == null ? void 0 : _a3.load) {
    let u = function(...d) {
      for (const f of d) {
        const { href: p } = new URL(f, n);
        c.dependencies.add(p);
      }
    };
    const h = { route: new Proxy(a, { get: (d, f) => (o && (c.route = true), d[f]) }), params: new Proxy(r, { get: (d, f) => (o && c.params.add(f), d[f]) }), data: (s == null ? void 0 : s.data) ?? null, url: bt(n, () => {
      o && (c.url = true);
    }, (d) => {
      o && c.search_params.add(d);
    }, v.hash), async fetch(d, f) {
      d instanceof Request && (f = { body: d.method === "GET" || d.method === "HEAD" ? void 0 : await d.blob(), cache: d.cache, credentials: d.credentials, headers: [...d.headers].length ? d.headers : void 0, integrity: d.integrity, keepalive: d.keepalive, method: d.method, mode: d.mode, redirect: d.redirect, referrer: d.referrer, referrerPolicy: d.referrerPolicy, signal: d.signal, ...f });
      const { resolved: p, promise: m } = lt(d, f, n);
      return o && u(p.href), m;
    }, setHeaders: () => {
    }, depends: u, parent() {
      return o && (c.parent = true), t();
    }, untrack(d) {
      o = false;
      try {
        return d();
      } finally {
        o = true;
      }
    } };
    i = await l.universal.load.call(null, h) ?? null;
  }
  return { node: l, loader: e, server: s, universal: ((_b2 = l.universal) == null ? void 0 : _b2.load) ? { type: "data", data: i, uses: c } : null, data: i ?? (s == null ? void 0 : s.data) ?? null, slash: ((_c2 = l.universal) == null ? void 0 : _c2.trailingSlash) ?? (s == null ? void 0 : s.slash) };
}
function lt(e, t, n) {
  let r = e instanceof Request ? e.url : e;
  const a = new URL(r, n);
  a.origin === n.origin && (r = a.href.slice(n.origin.length));
  const s = ie ? Rt(r, a.href, t) : Et(r, t);
  return { resolved: a, promise: s };
}
function He(e, t, n, r, a, s) {
  if (Pe) return true;
  if (!a) return false;
  if (a.parent && e || a.route && t || a.url && n) return true;
  for (const i of a.search_params) if (r.has(i)) return true;
  for (const i of a.params) if (s[i] !== y.params[i]) return true;
  for (const i of a.dependencies) if (oe.some((o) => o(new URL(i)))) return true;
  return false;
}
function Ne(e, t) {
  return (e == null ? void 0 : e.type) === "data" ? e : (e == null ? void 0 : e.type) === "skip" ? t ?? null : null;
}
function cn(e, t) {
  if (!e) return new Set(t.searchParams.keys());
  const n = /* @__PURE__ */ new Set([...e.searchParams.keys(), ...t.searchParams.keys()]);
  for (const r of n) {
    const a = e.searchParams.getAll(r), s = t.searchParams.getAll(r);
    a.every((i) => s.includes(i)) && s.every((i) => a.includes(i)) && n.delete(r);
  }
  return n;
}
function Ke({ error: e, url: t, route: n, params: r }) {
  return { type: "loaded", state: { error: e, url: t, route: n, params: r, branch: [] }, props: { page: je(S), constructors: [] } };
}
async function ft({ id: e, invalidating: t, url: n, params: r, route: a, preload: s }) {
  if ((O == null ? void 0 : O.id) === e) return W.delete(O.token), O.promise;
  const { errors: i, layouts: o, leaf: c } = a, l = [...o, c];
  i.forEach((g) => g == null ? void 0 : g().catch(() => {
  })), l.forEach((g) => g == null ? void 0 : g[1]().catch(() => {
  }));
  let u = null;
  const h = y.url ? e !== le(y.url) : false, d = y.route ? a.id !== y.route.id : false, f = cn(y.url, n);
  let p = false;
  const m = l.map((g, w) => {
    var _a3;
    const b = y.branch[w], A = !!(g == null ? void 0 : g[0]) && ((b == null ? void 0 : b.loader) !== g[1] || He(p, d, h, f, (_a3 = b.server) == null ? void 0 : _a3.uses, r));
    return A && (p = true), A;
  });
  if (m.some(Boolean)) {
    try {
      u = await ht(n, m);
    } catch (g) {
      const w = await q(g, { url: n, params: r, route: { id: e } });
      return W.has(s) ? Ke({ error: w, url: n, params: r, route: a }) : de({ status: re(g), error: w, url: n, route: a });
    }
    if (u.type === "redirect") return u;
  }
  const _ = u == null ? void 0 : u.nodes;
  let R = false;
  const E = l.map(async (g, w) => {
    var _a3;
    if (!g) return;
    const b = y.branch[w], A = _ == null ? void 0 : _[w];
    if ((!A || A.type === "skip") && g[1] === (b == null ? void 0 : b.loader) && !He(R, d, h, f, (_a3 = b.universal) == null ? void 0 : _a3.uses, r)) return b;
    if (R = true, (A == null ? void 0 : A.type) === "error") throw A;
    return Oe({ loader: g[1], url: n, params: r, route: a, parent: async () => {
      var _a4;
      const pe = {};
      for (let ge = 0; ge < w; ge += 1) Object.assign(pe, (_a4 = await E[ge]) == null ? void 0 : _a4.data);
      return pe;
    }, server_data_node: Ne(A === void 0 && g[0] ? { type: "skip" } : A ?? null, g[0] ? b == null ? void 0 : b.server : void 0) });
  });
  for (const g of E) g.catch(() => {
  });
  const L = [];
  for (let g = 0; g < l.length; g += 1) if (l[g]) try {
    L.push(await E[g]);
  } catch (w) {
    if (w instanceof Ie) return { type: "redirect", location: w.location };
    if (W.has(s)) return Ke({ error: await q(w, { params: r, url: n, route: { id: a.id } }), url: n, params: r, route: a });
    let b = re(w), A;
    if (_ == null ? void 0 : _.includes(w)) b = w.status ?? b, A = w.error;
    else if (w instanceof ue) A = w.body;
    else {
      if (await N.updated.check()) return await nt(), await M(n);
      A = await q(w, { params: r, url: n, route: { id: a.id } });
    }
    const Q = await ln(g, L, i);
    return Q ? ce({ url: n, params: r, branch: L.slice(0, Q.idx).concat(Q.node), status: b, error: A, route: a }) : await dt(n, { id: a.id }, A, b);
  }
  else L.push(void 0);
  return ce({ url: n, params: r, branch: L, status: 200, error: null, route: a, form: t ? void 0 : null });
}
async function ln(e, t, n) {
  for (; e--; ) if (n[e]) {
    let r = e;
    for (; !t[r]; ) r -= 1;
    try {
      return { idx: r + 1, node: { node: await n[e](), loader: n[e], data: {}, server: null, universal: null } };
    } catch {
      continue;
    }
  }
}
async function de({ status: e, error: t, url: n, route: r }) {
  const a = {};
  let s = null;
  if (v.server_loads[0] === 0) try {
    const o = await ht(n, [true]);
    if (o.type !== "data" || o.nodes[0] && o.nodes[0].type !== "data") throw 0;
    s = o.nodes[0] ?? null;
  } catch {
    (n.origin !== Z || n.pathname !== location.pathname || xe) && await M(n);
  }
  try {
    const o = await Oe({ loader: ke, url: n, params: a, route: r, parent: () => Promise.resolve({}), server_data_node: Ne(s) }), c = { node: await ae(), loader: ae, universal: null, server: null, data: null };
    return ce({ url: n, params: a, branch: [o, c], status: e, error: t, route: null });
  } catch (o) {
    if (o instanceof Ie) return Ce(new URL(o.location, location.href), {}, 0);
    throw o;
  }
}
async function fn(e) {
  const t = e.href;
  if (ee.has(t)) return ee.get(t);
  let n;
  try {
    const r = (async () => {
      let a = await v.hooks.reroute({ url: new URL(e), fetch: async (s, i) => lt(s, i, e).promise }) ?? e;
      if (typeof a == "string") {
        const s = new URL(e);
        v.hash ? s.hash = a : s.pathname = a, a = s;
      }
      return a;
    })();
    ee.set(t, r), n = await r;
  } catch {
    ee.delete(t);
    return;
  }
  return n;
}
async function he(e, t) {
  if (e && !fe(e, U, v.hash)) {
    const n = await fn(e);
    if (!n) return;
    const r = un(n);
    for (const a of Te) {
      const s = a.exec(r);
      if (s) return { id: le(e), invalidating: t, route: a, params: vt(s), url: e };
    }
  }
}
function un(e) {
  return yt(v.hash ? e.hash.replace(/^#/, "").replace(/[?#].+/, "") : e.pathname.slice(U.length)) || "/";
}
function le(e) {
  return (v.hash ? e.hash.replace(/^#/, "") : e.pathname) + e.search;
}
function ut({ url: e, type: t, intent: n, delta: r }) {
  let a = false;
  const s = gt(y, n, e, t);
  r !== void 0 && (s.navigation.delta = r);
  const i = { ...s.navigation, cancel: () => {
    a = true, s.reject(new Error("navigation cancelled"));
  } };
  return X || rt.forEach((o) => o(i)), a ? null : s;
}
async function te({ type: e, url: t, popped: n, keepfocus: r, noscroll: a, replace_state: s, state: i = {}, redirect_count: o = 0, nav_token: c = {}, accept: l = qe, block: u = qe }) {
  const h = $;
  $ = c;
  const d = await he(t, false), f = ut({ url: t, type: e, delta: n == null ? void 0 : n.delta, intent: d });
  if (!f) {
    u(), $ === c && ($ = h);
    return;
  }
  const p = k, m = I;
  l(), X = true, ie && N.navigating.set(z.current = f.navigation);
  let _ = d && await ft(d);
  if (!_) {
    if (fe(t, U, v.hash)) return await M(t);
    _ = await dt(t, { id: null }, await q(new Ue(404, "Not Found", `Not found: ${t.pathname}`), { url: t, params: {}, route: { id: null } }), 404);
  }
  if (t = (d == null ? void 0 : d.url) || t, $ !== c) return f.reject(new Error("navigation aborted")), false;
  if (_.type === "redirect") if (o >= 20) _ = await de({ status: 500, error: await q(new Error("Redirect loop"), { url: t, params: {}, route: { id: null } }), url: t, route: { id: null } });
  else return await Ce(new URL(_.location, t).href, {}, o + 1, c), false;
  else _.props.page.status >= 400 && await N.updated.check() && (await nt(), await M(t));
  if (on(), Le(p), st(m), _.props.page.url.pathname !== t.pathname && (t.pathname = _.props.page.url.pathname), i = n ? n.state : i, !n) {
    const g = s ? 0 : 1, w = { [B]: k += g, [Y]: I += g, [Je]: i };
    (s ? history.replaceState : history.pushState).call(history, w, "", t), s || rn(k, I);
  }
  if (O = null, _.props.page.state = i, ie) {
    y = _.state, _.props.page && (_.props.page.url = t);
    const g = (await Promise.all(Array.from(an, (w) => w(f.navigation)))).filter((w) => typeof w == "function");
    if (g.length > 0) {
      let w = function() {
        g.forEach((b) => {
          K.delete(b);
        });
      };
      g.push(w), g.forEach((b) => {
        K.add(b);
      });
    }
    ot.$set(_.props), Zt(_.props.page), at = true;
  } else ct(_, Ae, false);
  const { activeElement: R } = document;
  await mt();
  const E = n ? n.scroll : a ? Re() : null;
  if (Me) {
    const g = t.hash && document.getElementById(decodeURIComponent(v.hash ? t.hash.split("#")[2] ?? "" : t.hash.slice(1)));
    E ? scrollTo(E.x, E.y) : g ? g.scrollIntoView() : scrollTo(0, 0);
  }
  const L = document.activeElement !== R && document.activeElement !== document.body;
  !r && !L && mn(), Me = true, _.props.page && Object.assign(S, _.props.page), X = false, e === "popstate" && it(I), f.fulfil(void 0), K.forEach((g) => g(f.navigation)), N.navigating.set(z.current = null);
}
async function dt(e, t, n, r) {
  return e.origin === Z && e.pathname === location.pathname && !xe ? await de({ status: r, error: n, url: e, route: t }) : await M(e);
}
function dn() {
  let e, t, n;
  C.addEventListener("mousemove", (o) => {
    const c = o.target;
    clearTimeout(e), e = setTimeout(() => {
      s(c, j.hover);
    }, 20);
  });
  function r(o) {
    o.defaultPrevented || s(o.composedPath()[0], j.tap);
  }
  C.addEventListener("mousedown", r), C.addEventListener("touchstart", r, { passive: true });
  const a = new IntersectionObserver((o) => {
    for (const c of o) c.isIntersecting && (ve(new URL(c.target.href)), a.unobserve(c.target));
  }, { threshold: 0 });
  async function s(o, c) {
    const l = Qe(o, C), u = l === t && c >= n;
    if (!l || u) return;
    const { url: h, external: d, download: f } = be(l, U, v.hash);
    if (d || f) return;
    const p = ne(l), m = h && le(y.url) === le(h);
    if (!(p.reload || m)) if (c <= p.preload_data) {
      t = l, n = j.tap;
      const _ = await he(h, false);
      if (!_) return;
      sn(_);
    } else c <= p.preload_code && (t = l, n = c, ve(h));
  }
  function i() {
    a.disconnect();
    for (const o of C.querySelectorAll("a")) {
      const { url: c, external: l, download: u } = be(o, U, v.hash);
      if (l || u) continue;
      const h = ne(o);
      h.reload || (h.preload_code === j.viewport && a.observe(o), h.preload_code === j.eager && ve(c));
    }
  }
  K.add(i), i();
}
function q(e, t) {
  if (e instanceof ue) return e.body;
  const n = re(e), r = Jt(e);
  return v.hooks.handleError({ error: e, event: t, status: n, message: r }) ?? { message: r };
}
function hn(e, t = {}) {
  return e = new URL(Xe(e)), e.origin !== Z ? Promise.reject(new Error("goto: invalid URL")) : Ce(e, t, 0);
}
function pn(e) {
  if (typeof e == "function") oe.push(e);
  else {
    const { href: t } = new URL(e, location.href);
    oe.push((n) => n.href === t);
  }
}
function gn() {
  var _a3;
  history.scrollRestoration = "manual", addEventListener("beforeunload", (t) => {
    let n = false;
    if (Ge(), !X) {
      const r = gt(y, void 0, null, "leave"), a = { ...r.navigation, cancel: () => {
        n = true, r.reject(new Error("navigation cancelled"));
      } };
      rt.forEach((s) => s(a));
    }
    n ? (t.preventDefault(), t.returnValue = "") : history.scrollRestoration = "auto";
  }), addEventListener("visibilitychange", () => {
    document.visibilityState === "hidden" && Ge();
  }), ((_a3 = navigator.connection) == null ? void 0 : _a3.saveData) || dn(), C.addEventListener("click", async (t) => {
    if (t.button || t.which !== 1 || t.metaKey || t.ctrlKey || t.shiftKey || t.altKey || t.defaultPrevented) return;
    const n = Qe(t.composedPath()[0], C);
    if (!n) return;
    const { url: r, external: a, target: s, download: i } = be(n, U, v.hash);
    if (!r) return;
    if (s === "_parent" || s === "_top") {
      if (window.parent !== window) return;
    } else if (s && s !== "_self") return;
    const o = ne(n);
    if (!(n instanceof SVGAElement) && r.protocol !== location.protocol && !(r.protocol === "https:" || r.protocol === "http:") || i) return;
    const [l, u] = (v.hash ? r.hash.replace(/^#/, "") : r.href).split("#"), h = l === me(location);
    if (a || o.reload && (!h || !u)) {
      ut({ url: r, type: "link" }) ? X = true : t.preventDefault();
      return;
    }
    if (u !== void 0 && h) {
      const [, d] = y.url.href.split("#");
      if (d === u) {
        if (t.preventDefault(), u === "" || u === "top" && n.ownerDocument.getElementById("top") === null) window.scrollTo({ top: 0 });
        else {
          const f = n.ownerDocument.getElementById(decodeURIComponent(u));
          f && (f.scrollIntoView(), f.focus());
        }
        return;
      }
      if (G = true, Le(k), e(r), !o.replace_state) return;
      G = false;
    }
    t.preventDefault(), await new Promise((d) => {
      requestAnimationFrame(() => {
        setTimeout(d, 0);
      }), setTimeout(d, 100);
    }), await te({ type: "link", url: r, keepfocus: o.keepfocus, noscroll: o.noscroll, replace_state: o.replace_state ?? r.href === location.href });
  }), C.addEventListener("submit", (t) => {
    if (t.defaultPrevented) return;
    const n = HTMLFormElement.prototype.cloneNode.call(t.target), r = t.submitter;
    if (((r == null ? void 0 : r.formTarget) || n.target) === "_blank" || ((r == null ? void 0 : r.formMethod) || n.method) !== "get") return;
    const i = new URL((r == null ? void 0 : r.hasAttribute("formaction")) && (r == null ? void 0 : r.formAction) || n.action);
    if (fe(i, U, false)) return;
    const o = t.target, c = ne(o);
    if (c.reload) return;
    t.preventDefault(), t.stopPropagation();
    const l = new FormData(o), u = r == null ? void 0 : r.getAttribute("name");
    u && l.append(u, (r == null ? void 0 : r.getAttribute("value")) ?? ""), i.search = new URLSearchParams(l).toString(), te({ type: "form", url: i, keepfocus: c.keepfocus, noscroll: c.noscroll, replace_state: c.replace_state ?? i.href === location.href });
  }), addEventListener("popstate", async (t) => {
    var _a4;
    if ((_a4 = t.state) == null ? void 0 : _a4[B]) {
      const n = t.state[B];
      if ($ = {}, n === k) return;
      const r = D[n], a = t.state[Je] ?? {}, s = new URL(t.state[Nt] ?? location.href), i = t.state[Y], o = y.url ? me(location) === me(y.url) : false;
      if (i === I && (at || o)) {
        a !== S.state && (S.state = a), e(s), D[k] = Re(), r && scrollTo(r.x, r.y), k = n;
        return;
      }
      const l = n - k;
      await te({ type: "popstate", url: s, popped: { state: a, scroll: r, delta: l }, accept: () => {
        k = n, I = i;
      }, block: () => {
        history.go(-l);
      }, nav_token: $ });
    } else if (!G) {
      const n = new URL(location.href);
      e(n), v.hash && location.reload();
    }
  }), addEventListener("hashchange", () => {
    G && (G = false, history.replaceState({ ...history.state, [B]: ++k, [Y]: I }, "", location.href));
  });
  for (const t of document.querySelectorAll("link")) nn.has(t.rel) && (t.href = t.href);
  addEventListener("pageshow", (t) => {
    t.persisted && N.navigating.set(z.current = null);
  });
  function e(t) {
    y.url = S.url = t, N.page.set(je(S)), N.page.notify();
  }
}
async function _n(e, { status: t = 200, error: n, node_ids: r, params: a, route: s, server_route: i, data: o, form: c }) {
  xe = true;
  const l = new URL(location.href);
  let u;
  ({ params: a = {}, route: s = { id: null } } = await he(l, false) || {}), u = Te.find(({ id: f }) => f === s.id);
  let h, d = true;
  try {
    const f = r.map(async (m, _) => {
      const R = o[_];
      return (R == null ? void 0 : R.uses) && (R.uses = pt(R.uses)), Oe({ loader: v.nodes[m], url: l, params: a, route: s, parent: async () => {
        const E = {};
        for (let L = 0; L < _; L += 1) Object.assign(E, (await f[L]).data);
        return E;
      }, server_data_node: Ne(R) });
    }), p = await Promise.all(f);
    if (u) {
      const m = u.layouts;
      for (let _ = 0; _ < m.length; _++) m[_] || p.splice(_, 0, void 0);
    }
    h = ce({ url: l, params: a, branch: p, status: t, error: n, form: c, route: u ?? null });
  } catch (f) {
    if (f instanceof Ie) {
      await M(new URL(f.location, location.href));
      return;
    }
    h = await de({ status: re(f), error: await q(f, { url: l, params: a, route: s }), url: l, route: s }), e.textContent = "", d = false;
  }
  h.props.page && (h.props.page.state = {}), ct(h, e, d);
}
async function ht(e, t) {
  var _a3;
  const n = new URL(e);
  n.pathname = tn(e.pathname), e.pathname.endsWith("/") && n.searchParams.append(zt, "1"), n.searchParams.append(Yt, t.map((s) => s ? "1" : "0").join(""));
  const r = window.fetch, a = await r(n.href, {});
  if (!a.ok) {
    let s;
    throw ((_a3 = a.headers.get("content-type")) == null ? void 0 : _a3.includes("application/json")) ? s = await a.json() : a.status === 404 ? s = "Not Found" : a.status === 500 && (s = "Internal Error"), new ue(a.status, s);
  }
  return new Promise(async (s) => {
    var _a4;
    const i = /* @__PURE__ */ new Map(), o = a.body.getReader(), c = new TextDecoder();
    function l(h) {
      return Ht(h, { ...v.decoders, Promise: (d) => new Promise((f, p) => {
        i.set(d, { fulfil: f, reject: p });
      }) });
    }
    let u = "";
    for (; ; ) {
      const { done: h, value: d } = await o.read();
      if (h && !u) break;
      for (u += !d && u ? `
` : c.decode(d, { stream: true }); ; ) {
        const f = u.indexOf(`
`);
        if (f === -1) break;
        const p = JSON.parse(u.slice(0, f));
        if (u = u.slice(f + 1), p.type === "redirect") return s(p);
        if (p.type === "data") (_a4 = p.nodes) == null ? void 0 : _a4.forEach((m) => {
          (m == null ? void 0 : m.type) === "data" && (m.uses = pt(m.uses), m.data = l(m.data));
        }), s(p);
        else if (p.type === "chunk") {
          const { id: m, data: _, error: R } = p, E = i.get(m);
          i.delete(m), R ? E.reject(l(R)) : E.fulfil(l(_));
        }
      }
    }
  });
}
function pt(e) {
  return { dependencies: new Set((e == null ? void 0 : e.dependencies) ?? []), params: new Set((e == null ? void 0 : e.params) ?? []), parent: !!(e == null ? void 0 : e.parent), route: !!(e == null ? void 0 : e.route), url: !!(e == null ? void 0 : e.url), search_params: new Set((e == null ? void 0 : e.search_params) ?? []) };
}
function mn() {
  const e = document.querySelector("[autofocus]");
  if (e) e.focus();
  else {
    const t = document.body, n = t.getAttribute("tabindex");
    t.tabIndex = -1, t.focus({ preventScroll: true, focusVisible: false }), n !== null ? t.setAttribute("tabindex", n) : t.removeAttribute("tabindex");
    const r = getSelection();
    if (r && r.type !== "None") {
      const a = [];
      for (let s = 0; s < r.rangeCount; s += 1) a.push(r.getRangeAt(s));
      setTimeout(() => {
        if (r.rangeCount === a.length) {
          for (let s = 0; s < r.rangeCount; s += 1) {
            const i = a[s], o = r.getRangeAt(s);
            if (i.commonAncestorContainer !== o.commonAncestorContainer || i.startContainer !== o.startContainer || i.endContainer !== o.endContainer || i.startOffset !== o.startOffset || i.endOffset !== o.endOffset) return;
          }
          r.removeAllRanges();
        }
      });
    }
  }
}
function gt(e, t, n, r) {
  var _a3, _b2;
  let a, s;
  const i = new Promise((c, l) => {
    a = c, s = l;
  });
  return i.catch(() => {
  }), { navigation: { from: { params: e.params, route: { id: ((_a3 = e.route) == null ? void 0 : _a3.id) ?? null }, url: e.url }, to: n && { params: (t == null ? void 0 : t.params) ?? null, route: { id: ((_b2 = t == null ? void 0 : t.route) == null ? void 0 : _b2.id) ?? null }, url: n }, willUnload: !t, type: r, complete: i }, fulfil: a, reject: s };
}
function je(e) {
  return { data: e.data, error: e.error, form: e.form, params: e.params, route: e.route, state: e.state, status: e.status, url: e.url };
}
function wn(e) {
  const t = new URL(e);
  return t.hash = decodeURIComponent(e.hash), t;
}
export {
  Sn as a,
  hn as g,
  vn as l,
  S as p,
  N as s
};
