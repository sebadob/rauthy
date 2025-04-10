import { t as b, a as h } from "./BxmJRzoY.js";
import { p as ee, a1 as F, j as p, F as q, a4 as E, k as a, a0 as te, l as d, a5 as ae, c as n, r as o, t as _, s as g, a as ie } from "./w0HvPX0p.js";
import { s as C } from "./BzP2S3Z_.js";
import { i as se } from "./iO9_dPNE.js";
import { s as x } from "./BdbQ6g_y.js";
import { p as re } from "./C6GSeq7M.js";
import { I as A } from "./Ck-A6HWf.js";
import { u as ne } from "./0cG6LBdy.js";
import { B as G } from "./C8YTstTD.js";
import { O as oe } from "./BBiSUZQU.js";
import { f as le } from "./UPFlzoow.js";
const O = 20;
var de = b('<div class="iconLeft svelte-1eyqa5j"><!></div>'), ue = b('<div class="iconRight svelte-1eyqa5j"><!></div>'), ve = b('<div class="font-label total svelte-1eyqa5j"> </div>'), ce = b('<div class="container svelte-1eyqa5j"><div class="flex"><!> <ul class="svelte-1eyqa5j"><li class="svelte-1eyqa5j"> </li></ul> <!></div> <div class="flex gap-10"><div class="flex gap-05 chunkSize noselect"><div> </div> <div><!></div></div> <!></div></div>');
function ze(u, i) {
  ee(i, true);
  const M = [5, 10, 20, 30, 50, 100];
  let l = re(i, "pageSize", 31, () => F(O)), c = ne();
  const T = "1rem";
  l(Number.parseInt(i.firstFetchHeaders.get("x-page-size") || O.toString()));
  let f = p(void 0), j = q(() => l()), S = p(F(Number.parseInt(i.firstFetchHeaders.get("x-page-count") || "1"))), m = p(F(i.firstFetchHeaders.get("x-continuation-token"))), r = p(1), N = E(() => a(r) >= a(S));
  te(() => {
    if (i.idxTotalCount) {
      let e = i.firstFetchHeaders.get(i.idxTotalCount);
      e && d(f, Number.parseInt(e), true);
    }
  }), ae(() => {
    U();
  });
  async function U() {
    if (l() !== j) {
      j = l(), d(m, void 0);
      let [e, t] = await i.sspFetch(`page_size=${l()}`);
      P(e, t), d(r, 1);
    }
  }
  function L(e) {
    let t = `page_size=${q(() => l())}`;
    if (e) {
      if (a(r) === 2) return t;
      t += `&backwards=${e}&offset=${i.itemsLength - 1}`;
    }
    return a(m) && (t += `&continuation_token=${q(() => a(m))}`), t;
  }
  async function Z() {
    if (a(r) > 1) {
      let [e, t] = await i.sspFetch(L(true));
      P(e, t), d(r, a(r) - 1);
    }
  }
  async function J() {
    if (a(r) < a(S)) {
      let [e, t] = await i.sspFetch(L(false));
      P(e, t), d(r, a(r) + 1);
    }
  }
  function P(e, t) {
    if (e === 206) {
      let s = t.get("x-page-size");
      if (!s) {
        console.error("Did not receive x-page-size with SSP");
        return;
      }
      l(Number.parseInt(s)), d(S, Number.parseInt(t.get("x-page-count") || "1"), true), d(m, t.get("x-continuation-token"), true);
      let v = i.idxTotalCount ? t.get(i.idxTotalCount) : void 0;
      v ? d(f, Number.parseInt(v), true) : d(f, void 0);
    } else console.error("Received non 206 status with SSP");
  }
  var y = ce(), z = n(y), H = n(z);
  const K = E(() => a(r) < 2);
  G(H, { onclick: Z, invisible: true, get isDisabled() {
    return a(K);
  }, children: (e, t) => {
    var s = de(), v = n(s);
    A(v, { width: T }), o(s), _(() => {
      x(s, "aria-label", c.pagination.gotoPagePrev), x(s, "data-disabled", a(r) === 1);
    }), h(e, s);
  }, $$slots: { default: true } });
  var k = g(H, 2), D = n(k), Q = n(D, true);
  o(D), o(k);
  var V = g(k, 2);
  G(V, { onclick: J, invisible: true, get isDisabled() {
    return a(N);
  }, children: (e, t) => {
    var s = ue(), v = n(s);
    A(v, { width: T }), o(s), _(() => {
      x(s, "aria-label", c.pagination.gotoPageNext), x(s, "data-disabled", a(N));
    }), h(e, s);
  }, $$slots: { default: true } }), o(z);
  var R = g(z, 2), I = n(R), w = n(I), W = n(w, true);
  o(w);
  var B = g(w, 2), X = n(B);
  oe(X, { get ariaLabel() {
    return c.pagination.showCount;
  }, options: M, offsetTop: "-14rem", borderless: true, get value() {
    return l();
  }, set value(e) {
    l(e);
  } }), o(B), o(I);
  var Y = g(I, 2);
  {
    var $ = (e) => {
      var t = ve(), s = n(t);
      o(t), _(() => C(s, `${c.pagination.total ?? ""}: ${a(f) ?? ""}`)), h(e, t);
    };
    se(Y, (e) => {
      a(f) && e($);
    });
  }
  o(R), o(y), _(() => {
    C(Q, a(r)), C(W, c.pagination.entries);
  }), h(u, y), ie();
}
async function ke(u) {
  let i = `/auth/v1/search?ty=${u.ty}&idx=${u.idx}&q=${u.q}`;
  return u.limit && (i += `$limit=${u.limit}`), await le(i);
}
export {
  O as P,
  ze as a,
  ke as f
};
