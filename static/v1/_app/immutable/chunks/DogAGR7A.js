var __typeError = (msg) => {
  throw TypeError(msg);
};
var __accessCheck = (obj, member, msg) => member.has(obj) || __typeError("Cannot " + msg);
var __privateGet = (obj, member, getter) => (__accessCheck(obj, member, "read from private field"), getter ? getter.call(obj) : member.get(obj));
var __privateAdd = (obj, member, value) => member.has(obj) ? __typeError("Cannot add the same private member more than once") : member instanceof WeakSet ? member.add(obj) : member.set(obj, value);
var __privateSet = (obj, member, value, setter) => (__accessCheck(obj, member, "write to private field"), setter ? setter.call(obj, value) : member.set(obj, value), value);
var __privateMethod = (obj, member, method) => (__accessCheck(obj, member, "access private method"), method);
var _i, _r, _n, _t, _e, _o, _f, _a, _m, _s, _h, _v_instances, d_fn;
import { t as D, a as _ } from "./BxmJRzoY.js";
import { V as p, w as z, l as o, ag as C, a6 as P, k as f, p as S, j as y, a1 as V, a5 as X, ah as Y, s as B, c as x, t as T, a as F, r as j } from "./w0HvPX0p.js";
import { d as G, e as k, s as I } from "./BzP2S3Z_.js";
import { i as J } from "./iO9_dPNE.js";
import { s as K } from "./DM69BKKN.js";
import { s as L, a as N } from "./BdbQ6g_y.js";
import { r as Q, l as R, t as U, f as W } from "./C4AV2CoD.js";
import { p as O } from "./C6GSeq7M.js";
function A(e) {
  return Object.prototype.toString.call(e) === "[object Date]";
}
function b(e, s, t, i) {
  if (typeof t == "number" || A(t)) {
    const n = i - t, r = (t - s) / (e.dt || 1 / 60), h = e.opts.stiffness * n, l = e.opts.damping * r, c = (h - l) * e.inv_mass, m = (r + c) * e.dt;
    return Math.abs(m) < e.opts.precision && Math.abs(n) < e.opts.precision ? i : (e.settled = false, A(t) ? new Date(t.getTime() + m) : t + m);
  } else {
    if (Array.isArray(t)) return t.map((n, r) => b(e, s[r], t[r], i[r]));
    if (typeof t == "object") {
      const n = {};
      for (const r in t) n[r] = b(e, s[r], t[r], i[r]);
      return n;
    } else throw new Error(`Cannot spring ${typeof t} values`);
  }
}
const _v = class _v {
  constructor(s, t = {}) {
    __privateAdd(this, _v_instances);
    __privateAdd(this, _i, p(0.15));
    __privateAdd(this, _r, p(0.8));
    __privateAdd(this, _n, p(0.01));
    __privateAdd(this, _t, p(void 0));
    __privateAdd(this, _e, p(void 0));
    __privateAdd(this, _o);
    __privateAdd(this, _f, 0);
    __privateAdd(this, _a, 1);
    __privateAdd(this, _m, 0);
    __privateAdd(this, _s, null);
    __privateAdd(this, _h, null);
    __privateGet(this, _t).v = __privateGet(this, _e).v = s, typeof t.stiffness == "number" && (__privateGet(this, _i).v = u(t.stiffness, 0, 1)), typeof t.damping == "number" && (__privateGet(this, _r).v = u(t.damping, 0, 1)), typeof t.precision == "number" && (__privateGet(this, _n).v = t.precision);
  }
  static of(s, t) {
    const i = new _v(s(), t);
    return z(() => {
      i.set(s());
    }), i;
  }
  set(s, t) {
    var _a2, _b;
    if ((_a2 = __privateGet(this, _h)) == null ? void 0 : _a2.reject(new Error("Aborted")), (t == null ? void 0 : t.instant) || __privateGet(this, _t).v === void 0) return (_b = __privateGet(this, _s)) == null ? void 0 : _b.abort(), __privateSet(this, _s, null), o(__privateGet(this, _t), o(__privateGet(this, _e), s)), __privateSet(this, _o, s), Promise.resolve();
    (t == null ? void 0 : t.preserveMomentum) && (__privateSet(this, _a, 0), __privateSet(this, _m, t.preserveMomentum));
    var i = __privateSet(this, _h, C());
    return i.promise.catch(P), __privateMethod(this, _v_instances, d_fn).call(this, s).then(() => {
      i === __privateGet(this, _h) && i.resolve(void 0);
    }), i.promise;
  }
  get current() {
    return f(__privateGet(this, _t));
  }
  get damping() {
    return f(__privateGet(this, _r));
  }
  set damping(s) {
    o(__privateGet(this, _r), u(s, 0, 1));
  }
  get precision() {
    return f(__privateGet(this, _n));
  }
  set precision(s) {
    o(__privateGet(this, _n), s);
  }
  get stiffness() {
    return f(__privateGet(this, _i));
  }
  set stiffness(s) {
    o(__privateGet(this, _i), u(s, 0, 1));
  }
  get target() {
    return f(__privateGet(this, _e));
  }
  set target(s) {
    this.set(s);
  }
};
_i = new WeakMap();
_r = new WeakMap();
_n = new WeakMap();
_t = new WeakMap();
_e = new WeakMap();
_o = new WeakMap();
_f = new WeakMap();
_a = new WeakMap();
_m = new WeakMap();
_s = new WeakMap();
_h = new WeakMap();
_v_instances = new WeakSet();
d_fn = function(s) {
  var _a2;
  if (o(__privateGet(this, _e), s), (_a2 = __privateGet(this, _t)).v ?? (_a2.v = s), __privateGet(this, _o) ?? __privateSet(this, _o, __privateGet(this, _t).v), !__privateGet(this, _s)) {
    __privateSet(this, _f, Q.now());
    var t = 1e3 / (__privateGet(this, _m) * 60);
    __privateGet(this, _s) ?? __privateSet(this, _s, R((i) => {
      __privateSet(this, _a, Math.min(__privateGet(this, _a) + t, 1));
      const n = Math.min(i - __privateGet(this, _f), 1e3 / 30), r = { inv_mass: __privateGet(this, _a), opts: { stiffness: __privateGet(this, _i).v, damping: __privateGet(this, _r).v, precision: __privateGet(this, _n).v }, settled: true, dt: n * 60 / 1e3 };
      var h = b(r, __privateGet(this, _o), __privateGet(this, _t).v, __privateGet(this, _e).v);
      return __privateSet(this, _o, __privateGet(this, _t).v), __privateSet(this, _f, i), o(__privateGet(this, _t), h), r.settled && __privateSet(this, _s, null), !r.settled;
    }));
  }
  return __privateGet(this, _s).promise;
};
let v = _v;
function u(e, s, t) {
  return Math.max(s, Math.min(t, e));
}
function Z(e, s, t, i, n) {
  s.set(e.clientY + t()), i.set(e.clientX + n());
}
var $ = D('<div class="tooltip svelte-exzuuq"> </div>'), tt = D('<div role="tooltip"><!> <!></div>');
function ht(e, s) {
  S(s, true);
  let t = O(s, "xOffset", 3, 10), i = O(s, "yOffset", 3, 10), n = y(V(s.text)), r = y(false), h = y(void 0);
  const l = new v(0, { stiffness: 0.1, damping: 0.6 }), c = new v(0, { stiffness: 0.1, damping: 0.6 });
  X(() => {
    o(n, "");
  }), Y(() => {
    clearTimeout(f(h));
  });
  function m() {
    clearTimeout(f(h)), o(r, true);
  }
  function w() {
    o(h, setTimeout(() => {
      o(r, false);
    }, 100), true);
  }
  var a = tt();
  a.__mouseover = m, a.__mouseout = w, a.__mousemove = [Z, l, i, c, t];
  var M = x(a);
  K(M, () => s.children);
  var E = B(M, 2);
  {
    var H = (g) => {
      var d = $(), q = x(d, true);
      j(d), T(() => {
        N(d, `top: ${`${l.current}px` ?? ""}; left: ${`${c.current}px` ?? ""}`), I(q, s.text);
      }), U(3, d, () => W, () => ({ delay: 400, duration: 200 })), _(g, d);
    };
    J(E, (g) => {
      f(r) && g(H);
    });
  }
  j(a), T(() => L(a, "title", f(n))), k("focus", a, m), k("blur", a, w), _(e, a), F();
}
G(["mouseover", "mouseout", "mousemove"]);
export {
  ht as T
};
