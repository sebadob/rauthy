import { t as S, a as h } from "./YHhP1LbZ.js";
import { p as ee, P as C, k as _, a6 as te, l as d, aa as ae, c as n, r as o, t as x, j as a, a9 as E, s as m, a as ie } from "./Ck6jKiur.js";
import { s as F } from "./tDAaDMC_.js";
import { i as se } from "./7JDmqCCW.js";
import { s as b } from "./BTaFr7HN.js";
import { p as u } from "./ho0YXExL.js";
import { p as re } from "./DZP54pO_.js";
import { I as A } from "./qjmJ_WV1.js";
import { u as ne } from "./mN05BXqA.js";
import { B as G } from "./Bd2Rvcxs.js";
import { O as oe } from "./By82gADu.js";
import { f as le } from "./BO1A6s0c.js";
const O = 20;
var de = S('<div class="iconLeft svelte-1eyqa5j"><!></div>'), ve = S('<div class="iconRight svelte-1eyqa5j"><!></div>'), ue = S('<div class="font-label total svelte-1eyqa5j"> </div>'), ce = S('<div class="container svelte-1eyqa5j"><div class="flex"><!> <ul class="svelte-1eyqa5j"><li class="svelte-1eyqa5j"> </li></ul> <!></div> <div class="flex gap-10"><div class="flex gap-05 chunkSize noselect"><div> </div> <div><!></div></div> <!></div></div>');
function ke(v, i) {
  ee(i, true);
  const M = [5, 10, 20, 30, 50, 100];
  let l = re(i, "pageSize", 31, () => u(O)), f = ne();
  const T = "1rem";
  l(Number.parseInt(i.firstFetchHeaders.get("x-page-size") || O.toString()));
  let g = _(void 0), j = C(() => l()), P = _(u(Number.parseInt(i.firstFetchHeaders.get("x-page-count") || "1"))), p = _(u(i.firstFetchHeaders.get("x-continuation-token"))), r = _(1), N = E(() => a(r) >= a(P));
  te(() => {
    if (i.idxTotalCount) {
      let e = i.firstFetchHeaders.get(i.idxTotalCount);
      e && d(g, u(Number.parseInt(e)));
    }
  }), ae(() => {
    U();
  });
  async function U() {
    if (l() !== j) {
      j = l(), d(p, void 0);
      let [e, t] = await i.sspFetch(`page_size=${l()}`);
      y(e, t), d(r, 1);
    }
  }
  function L(e) {
    let t = `page_size=${C(() => l())}`;
    if (e) {
      if (a(r) === 2) return t;
      t += `&backwards=${e}&offset=${i.itemsLength - 1}`;
    }
    return a(p) && (t += `&continuation_token=${C(() => a(p))}`), t;
  }
  async function Z() {
    if (a(r) > 1) {
      let [e, t] = await i.sspFetch(L(true));
      y(e, t), d(r, a(r) - 1);
    }
  }
  async function J() {
    if (a(r) < a(P)) {
      let [e, t] = await i.sspFetch(L(false));
      y(e, t), d(r, a(r) + 1);
    }
  }
  function y(e, t) {
    if (e === 206) {
      let s = t.get("x-page-size");
      if (!s) {
        console.error("Did not receive x-page-size with SSP");
        return;
      }
      l(Number.parseInt(s)), d(P, u(Number.parseInt(t.get("x-page-count") || "1"))), d(p, u(t.get("x-continuation-token")));
      let c = i.idxTotalCount ? t.get(i.idxTotalCount) : void 0;
      c ? d(g, u(Number.parseInt(c))) : d(g, void 0);
    } else console.error("Received non 206 status with SSP");
  }
  var z = ce(), k = n(z), H = n(k);
  const K = E(() => a(r) < 2);
  G(H, { onclick: Z, invisible: true, get isDisabled() {
    return a(K);
  }, children: (e, t) => {
    var s = de(), c = n(s);
    A(c, { width: T }), o(s), x(() => {
      b(s, "aria-label", f.pagination.gotoPagePrev), b(s, "data-disabled", a(r) === 1);
    }), h(e, s);
  }, $$slots: { default: true } });
  var I = m(H, 2), D = n(I), Q = n(D, true);
  o(D), o(I);
  var V = m(I, 2);
  G(V, { onclick: J, invisible: true, get isDisabled() {
    return a(N);
  }, children: (e, t) => {
    var s = ve(), c = n(s);
    A(c, { width: T }), o(s), x(() => {
      b(s, "aria-label", f.pagination.gotoPageNext), b(s, "data-disabled", a(N));
    }), h(e, s);
  }, $$slots: { default: true } }), o(k);
  var R = m(k, 2), w = n(R), q = n(w), W = n(q, true);
  o(q);
  var B = m(q, 2), X = n(B);
  oe(X, { get ariaLabel() {
    return f.pagination.showCount;
  }, options: M, offsetTop: "-14rem", borderless: true, get value() {
    return l();
  }, set value(e) {
    l(e);
  } }), o(B), o(w);
  var Y = m(w, 2);
  {
    var $ = (e) => {
      var t = ue(), s = n(t);
      o(t), x(() => F(s, `${f.pagination.total ?? ""}: ${a(g) ?? ""}`)), h(e, t);
    };
    se(Y, (e) => {
      a(g) && e($);
    });
  }
  o(R), o(z), x(() => {
    F(Q, a(r)), F(W, f.pagination.entries);
  }), h(v, z), ie();
}
async function Ie(v) {
  let i = `/auth/v1/search?ty=${v.ty}&idx=${v.idx}&q=${v.q}`;
  return v.limit && (i += `$limit=${v.limit}`), await le(i);
}
export {
  O as P,
  ke as a,
  Ie as f
};
