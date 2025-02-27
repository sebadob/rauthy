let G, K, $, H, Q, V, P, L, X, z, A, Y, I, B, ee, D, N, J, q, F, Z, ue;
let __tla = (async () => {
  const v = "" + new URL("../assets/spow-wasm_bg.PRnxRq7g.wasm", import.meta.url).href, M = async (e = {}, n) => {
    let _;
    if (n.startsWith("data:")) {
      const r = n.replace(/^data:.*?base64,/, "");
      let t;
      if (typeof Buffer == "function" && typeof Buffer.from == "function") t = Buffer.from(r, "base64");
      else if (typeof atob == "function") {
        const a = atob(r);
        t = new Uint8Array(a.length);
        for (let o = 0; o < a.length; o++) t[o] = a.charCodeAt(o);
      } else throw new Error("Cannot decode base64-encoded data URL");
      _ = await WebAssembly.instantiate(t, e);
    } else {
      const r = await fetch(n), t = r.headers.get("Content-Type") || "";
      if ("instantiateStreaming" in WebAssembly && t.startsWith("application/wasm")) _ = await WebAssembly.instantiateStreaming(r, e);
      else {
        const a = await r.arrayBuffer();
        _ = await WebAssembly.instantiate(a, e);
      }
    }
    return _.instance.exports;
  };
  let u;
  A = function(e) {
    u = e;
  };
  const f = new Array(128).fill(void 0);
  f.push(void 0, null, true, false);
  function s(e) {
    return f[e];
  }
  let w = f.length;
  function R(e) {
    e < 132 || (f[e] = w, w = e);
  }
  function y(e) {
    const n = s(e);
    return R(e), n;
  }
  const O = typeof TextDecoder > "u" ? (0, module.require)("util").TextDecoder : TextDecoder;
  let x = new O("utf-8", {
    ignoreBOM: true,
    fatal: true
  });
  x.decode();
  let g = null;
  function m() {
    return (g === null || g.byteLength === 0) && (g = new Uint8Array(u.memory.buffer)), g;
  }
  function T(e, n) {
    return e = e >>> 0, x.decode(m().subarray(e, e + n));
  }
  function c(e) {
    w === f.length && f.push(f.length + 1);
    const n = w;
    return w = f[n], f[n] = e, n;
  }
  const k = typeof FinalizationRegistry > "u" ? {
    register: () => {
    },
    unregister: () => {
    }
  } : new FinalizationRegistry((e) => {
    u.__wbindgen_export_0.get(e.dtor)(e.a, e.b);
  });
  function W(e, n, _, r) {
    const t = {
      a: e,
      b: n,
      cnt: 1,
      dtor: _
    }, a = (...o) => {
      t.cnt++;
      const i = t.a;
      t.a = 0;
      try {
        return r(i, t.b, ...o);
      } finally {
        --t.cnt === 0 ? (u.__wbindgen_export_0.get(t.dtor)(i, t.b), k.unregister(t)) : t.a = i;
      }
    };
    return a.original = t, k.register(a, t, t), a;
  }
  function j(e, n, _) {
    u._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h7eff1fc92a11aeed(e, n, c(_));
  }
  let h = 0;
  const C = typeof TextEncoder > "u" ? (0, module.require)("util").TextEncoder : TextEncoder;
  let p = new C("utf-8");
  const E = typeof p.encodeInto == "function" ? function(e, n) {
    return p.encodeInto(e, n);
  } : function(e, n) {
    const _ = p.encode(e);
    return n.set(_), {
      read: e.length,
      written: _.length
    };
  };
  function S(e, n, _) {
    if (_ === void 0) {
      const i = p.encode(e), l = n(i.length, 1) >>> 0;
      return m().subarray(l, l + i.length).set(i), h = i.length, l;
    }
    let r = e.length, t = n(r, 1) >>> 0;
    const a = m();
    let o = 0;
    for (; o < r; o++) {
      const i = e.charCodeAt(o);
      if (i > 127) break;
      a[t + o] = i;
    }
    if (o !== r) {
      o !== 0 && (e = e.slice(o)), t = _(t, r, r = o + e.length * 3, 1) >>> 0;
      const i = m().subarray(t + o, t + r), l = E(e, i);
      o += l.written, t = _(t, r, o, 1) >>> 0;
    }
    return h = o, t;
  }
  ue = function(e) {
    const n = S(e, u.__wbindgen_malloc, u.__wbindgen_realloc), _ = h, r = u.pow_work_wasm(n, _);
    return y(r);
  };
  function d(e, n) {
    try {
      return e.apply(this, n);
    } catch (_) {
      u.__wbindgen_exn_store(c(_));
    }
  }
  function U(e, n, _, r) {
    u.wasm_bindgen__convert__closures__invoke2_mut__h8d1a7fb8f096da29(e, n, c(_), c(r));
  }
  q = function(e) {
    y(e);
  };
  F = function(e, n) {
    const _ = T(e, n);
    return c(_);
  };
  L = function(e) {
    const n = s(e).queueMicrotask;
    return c(n);
  };
  D = function(e) {
    return typeof s(e) == "function";
  };
  B = function(e) {
    const n = y(e).original;
    return n.cnt-- == 1 ? (n.a = 0, true) : false;
  };
  P = function(e) {
    queueMicrotask(s(e));
  };
  z = function() {
    return d(function() {
      const e = self.self;
      return c(e);
    }, arguments);
  };
  I = function() {
    return d(function() {
      const e = window.window;
      return c(e);
    }, arguments);
  };
  $ = function() {
    return d(function() {
      const e = globalThis.globalThis;
      return c(e);
    }, arguments);
  };
  H = function() {
    return d(function() {
      const e = global.global;
      return c(e);
    }, arguments);
  };
  N = function(e) {
    return s(e) === void 0;
  };
  V = function(e, n) {
    const _ = new Function(T(e, n));
    return c(_);
  };
  G = function() {
    return d(function(e, n) {
      const _ = s(e).call(s(n));
      return c(_);
    }, arguments);
  };
  J = function(e) {
    const n = s(e);
    return c(n);
  };
  K = function() {
    return d(function(e, n, _) {
      const r = s(e).call(s(n), s(_));
      return c(r);
    }, arguments);
  };
  Q = function(e, n) {
    try {
      var _ = {
        a: e,
        b: n
      }, r = (a, o) => {
        const i = _.a;
        _.a = 0;
        try {
          return U(i, _.b, a, o);
        } finally {
          _.a = i;
        }
      };
      const t = new Promise(r);
      return c(t);
    } finally {
      _.a = _.b = 0;
    }
  };
  X = function(e) {
    const n = Promise.resolve(s(e));
    return c(n);
  };
  Y = function(e, n) {
    const _ = s(e).then(s(n));
    return c(_);
  };
  Z = function(e, n) {
    throw new Error(T(e, n));
  };
  ee = function(e, n, _) {
    const r = W(e, n, 7, j);
    return c(r);
  };
  URL = globalThis.URL;
  const b = await M({
    "./spow-wasm_bg.js": {
      __wbindgen_object_drop_ref: q,
      __wbindgen_string_new: F,
      __wbg_queueMicrotask_48421b3cc9052b68: L,
      __wbindgen_is_function: D,
      __wbindgen_cb_drop: B,
      __wbg_queueMicrotask_12a30234db4045d3: P,
      __wbg_self_3093d5d1f7bcb682: z,
      __wbg_window_3bcfc4d31bc012f8: I,
      __wbg_globalThis_86b222e13bdf32ed: $,
      __wbg_global_e5a3fe56f8be9485: H,
      __wbindgen_is_undefined: N,
      __wbg_newnoargs_76313bd6ff35d0f2: V,
      __wbg_call_1084a111329e68ce: G,
      __wbindgen_object_clone_ref: J,
      __wbg_call_89af060b4e1523f2: K,
      __wbg_new_b85e72ed1bfd57f9: Q,
      __wbg_resolve_570458cb99d56a43: X,
      __wbg_then_95e6edc0f89b73b1: Y,
      __wbindgen_throw: Z,
      __wbindgen_closure_wrapper36: ee
    }
  }, v), ne = b.memory, _e = b.pow_work_wasm, te = b.__wbindgen_export_0, re = b._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h7eff1fc92a11aeed, oe = b.__wbindgen_malloc, ce = b.__wbindgen_realloc, ie = b.__wbindgen_exn_store, se = b.wasm_bindgen__convert__closures__invoke2_mut__h8d1a7fb8f096da29, ae = Object.freeze(Object.defineProperty({
    __proto__: null,
    __wbindgen_exn_store: ie,
    __wbindgen_export_0: te,
    __wbindgen_malloc: oe,
    __wbindgen_realloc: ce,
    _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h7eff1fc92a11aeed: re,
    memory: ne,
    pow_work_wasm: _e,
    wasm_bindgen__convert__closures__invoke2_mut__h8d1a7fb8f096da29: se
  }, Symbol.toStringTag, {
    value: "Module"
  }));
  A(ae);
})();
export {
  __tla,
  G as __wbg_call_1084a111329e68ce,
  K as __wbg_call_89af060b4e1523f2,
  $ as __wbg_globalThis_86b222e13bdf32ed,
  H as __wbg_global_e5a3fe56f8be9485,
  Q as __wbg_new_b85e72ed1bfd57f9,
  V as __wbg_newnoargs_76313bd6ff35d0f2,
  P as __wbg_queueMicrotask_12a30234db4045d3,
  L as __wbg_queueMicrotask_48421b3cc9052b68,
  X as __wbg_resolve_570458cb99d56a43,
  z as __wbg_self_3093d5d1f7bcb682,
  A as __wbg_set_wasm,
  Y as __wbg_then_95e6edc0f89b73b1,
  I as __wbg_window_3bcfc4d31bc012f8,
  B as __wbindgen_cb_drop,
  ee as __wbindgen_closure_wrapper36,
  D as __wbindgen_is_function,
  N as __wbindgen_is_undefined,
  J as __wbindgen_object_clone_ref,
  q as __wbindgen_object_drop_ref,
  F as __wbindgen_string_new,
  Z as __wbindgen_throw,
  ue as pow_work_wasm
};
