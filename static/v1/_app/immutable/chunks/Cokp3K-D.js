import { n as C, a as g, t as M } from "./BH6NCLk-.js";
import { t as _, p as P, k as y, aa as S, c as k, s as x, a as j, l as p, j as v, r as B } from "./CvlvO1XB.js";
import { p as o } from "./C6SR4G2t.js";
import { i as I } from "./BUO_AUgz.js";
import { p as w } from "./Wh68IIk2.js";
import { s as i } from "./BMbqVy6X.js";
import { B as O } from "./DMkkW5Nn.js";
import { T as b } from "./BmpEzKyJ.js";
import { O as R, S as W } from "./Cn6D5SC8.js";
import { u as U } from "./D8mHI_K9.js";
var q = C('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M3 4.5h14.25M3 9h9.75M3 13.5h9.75m4.5-4.5v12m0 0l-3.75-3.75M17.25 21L21 17.25"></path></svg>');
function z(l, e) {
  let a = o(e, "opacity", 3, 0.9), r = o(e, "width", 3, "1.5rem"), s = o(e, "color", 3, "currentColor");
  var t = q();
  i(t, "stroke-width", 2), _(() => {
    i(t, "stroke", s()), i(t, "width", r()), i(t, "opacity", a());
  }), g(l, t);
}
var E = C('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M3 4.5h14.25M3 9h9.75M3 13.5h5.25m5.25-.75L17.25 9m0 0L21 12.75M17.25 9v12"></path></svg>');
function F(l, e) {
  let a = o(e, "opacity", 3, 0.9), r = o(e, "width", 3, "1.5rem"), s = o(e, "color", 3, "currentColor");
  var t = E();
  i(t, "stroke-width", 2), _(() => {
    i(t, "stroke", s()), i(t, "width", r()), i(t, "opacity", a());
  }), g(l, t);
}
var G = M('<div class="container svelte-5a6d30"><!> <div class="btn svelte-5a6d30"><!></div></div>');
function H(l, e) {
  P(e, true);
  let a = o(e, "options", 19, () => []), r = U(), s = y(w(a()[0])), t = y(w(e.firstDirReverse ? "down" : "up"));
  S(() => {
    e.onChange(v(s), v(t));
  });
  var d = G(), u = k(d);
  b(u, { get text() {
    return r.search.orderBy;
  }, children: (n, h) => {
    R(n, { get ariaLabel() {
      return r.search.orderBy;
    }, get options() {
      return a();
    }, get borderless() {
      return e.borderless;
    }, get value() {
      return v(s);
    }, set value(m) {
      p(s, w(m));
    } });
  }, $$slots: { default: true } });
  var c = x(u, 2), D = k(c);
  {
    var L = (n) => {
      O(n, { get ariaLabel() {
        return r.search.orderChangeToDesc;
      }, invisible: true, onclick: () => p(t, "down"), children: (h, m) => {
        b(h, { get text() {
          return r.search.orderChangeToDesc;
        }, children: (f, A) => {
          F(f, {});
        }, $$slots: { default: true } });
      }, $$slots: { default: true } });
    }, T = (n) => {
      O(n, { get ariaLabel() {
        return r.search.orderChangeToAsc;
      }, invisible: true, onclick: () => p(t, "up"), children: (h, m) => {
        b(h, { get text() {
          return r.search.orderChangeToAsc;
        }, children: (f, A) => {
          z(f, {});
        }, $$slots: { default: true } });
      }, $$slots: { default: true } });
    };
    I(D, (n) => {
      v(t) === "up" ? n(L) : n(T, false);
    });
  }
  B(c), B(d), g(l, d), j();
}
var J = M('<div class="container svelte-1labfm1"><!> <!></div>');
function re(l, e) {
  let a = o(e, "searchOption", 15), r = o(e, "value", 15), s = o(e, "searchWidth", 3, "min(25rem, calc(100dvw - 1rem))");
  var t = J(), d = k(t);
  H(d, { get options() {
    return e.orderOptions;
  }, get onChange() {
    return e.onChangeOrder;
  }, get borderless() {
    return e.borderless;
  }, get firstDirReverse() {
    return e.firstDirReverse;
  } });
  var u = x(d, 2);
  W(u, { get datalist() {
    return e.datalist;
  }, get options() {
    return e.searchOptions;
  }, get pattern() {
    return e.pattern;
  }, get width() {
    return s();
  }, get borderless() {
    return e.borderless;
  }, get onSearch() {
    return e.onSearch;
  }, get value() {
    return r();
  }, set value(c) {
    r(c);
  }, get option() {
    return a();
  }, set option(c) {
    a(c);
  } }), B(t), g(l, t);
}
export {
  re as O
};
