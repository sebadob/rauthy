import { d as S, a as v, t as x } from "./BH6NCLk-.js";
import { p as T, k as U, aa as V, l as W, f as X, a as Y, c as r, r as o, s as y, t as g, j as m } from "./CvlvO1XB.js";
import { s as u } from "./CTI4QPiR.js";
import { i as p } from "./BUO_AUgz.js";
import { a as h } from "./BMbqVy6X.js";
import { p as R } from "./Wh68IIk2.js";
import { p as Z } from "./C6SR4G2t.js";
import { u as $ } from "./BQ1-pLIs.js";
var ll = x("<li> </li>"), el = x("<li> </li>"), il = x("<li> </li>"), al = x("<li> </li>"), tl = x('<li class="li svelte-u5h3xg"> </li>'), cl = x('<div class="policyContainer svelte-u5h3xg"><b class="svelte-u5h3xg"> </b> <ul class="svelte-u5h3xg"><li> </li> <li> </li> <!> <!> <!> <!> <!></ul></div>');
function vl(q, l) {
  T(l, true);
  let z = Z(l, "accepted", 15, false), _ = $(), d = U(R([false, false, false, false, false, false]));
  V(() => {
    B();
  });
  function B() {
    if (!l.policy) return false;
    let c = [false, false, false, false, false, false], s = false;
    l.password.length < l.policy.length_min && (c[0] = true, s = true), l.password.length > l.policy.length_max && (c[1] = true, s = true);
    let a = [0, 0, 0, 0];
    for (let w = 0; w < l.password.length; w++) {
      let n = l.password.charCodeAt(w);
      if (n >= 97 && n <= 122) {
        a[0] = a[0] + 1;
        continue;
      }
      if (n >= 65 && n <= 90) {
        a[1] = a[1] + 1;
        continue;
      }
      if (n >= 48 && n <= 57) {
        a[2] = a[2] + 1;
        continue;
      }
      a[3] = a[3] + 1;
    }
    l.policy.include_lower_case && l.policy.include_lower_case !== -1 && l.policy.include_lower_case > a[0] && (c[2] = true, s = true), l.policy.include_upper_case && l.policy.include_upper_case !== -1 && l.policy.include_upper_case > a[1] && (c[3] = true, s = true), l.policy.include_digits && l.policy.include_digits !== -1 && l.policy.include_digits > a[2] && (c[4] = true, s = true), l.policy.include_special && l.policy.include_special !== -1 && l.policy.include_special > a[3] && (c[5] = true, s = true), W(d, R(c)), z(!s);
  }
  var M = S(), D = X(M);
  {
    var F = (c) => {
      var s = cl(), a = r(s), w = r(a, true);
      o(a);
      var n = y(a, 2), P = r(n);
      let b;
      var G = r(P);
      o(P);
      var E = y(P, 2);
      let k;
      var H = r(E);
      o(E);
      var C = y(E, 2);
      {
        var J = (i) => {
          var e = ll();
          let t;
          var f = r(e);
          o(e), g(() => {
            var _a;
            t = h(e, 1, "li svelte-u5h3xg", null, t, { policyErr: !!m(d)[2] }), u(f, `${_.passwordPolicy.lowercaseMin ?? ""}
                    ${((_a = l.policy) == null ? void 0 : _a.include_lower_case) || 0}`);
          }), v(i, e);
        };
        p(C, (i) => {
          l.policy.include_lower_case !== -1 && i(J);
        });
      }
      var j = y(C, 2);
      {
        var K = (i) => {
          var e = el();
          let t;
          var f = r(e);
          o(e), g(() => {
            var _a;
            t = h(e, 1, "li svelte-u5h3xg", null, t, { policyErr: !!m(d)[3] }), u(f, `${_.passwordPolicy.uppercaseMin ?? ""}
                    ${((_a = l.policy) == null ? void 0 : _a.include_upper_case) || 0}`);
          }), v(i, e);
        };
        p(j, (i) => {
          l.policy.include_upper_case !== -1 && i(K);
        });
      }
      var A = y(j, 2);
      {
        var L = (i) => {
          var e = il();
          let t;
          var f = r(e);
          o(e), g(() => {
            var _a;
            t = h(e, 1, "li svelte-u5h3xg", null, t, { policyErr: !!m(d)[4] }), u(f, `${_.passwordPolicy.digitsMin ?? ""}
                    ${((_a = l.policy) == null ? void 0 : _a.include_digits) || 0}`);
          }), v(i, e);
        };
        p(A, (i) => {
          l.policy.include_digits !== -1 && i(L);
        });
      }
      var I = y(A, 2);
      {
        var N = (i) => {
          var e = al();
          let t;
          var f = r(e);
          o(e), g(() => {
            var _a;
            t = h(e, 1, "li svelte-u5h3xg", null, t, { policyErr: !!m(d)[5] }), u(f, `${_.passwordPolicy.specialMin ?? ""}
                    ${((_a = l.policy) == null ? void 0 : _a.include_special) || 0}`);
          }), v(i, e);
        };
        p(I, (i) => {
          l.policy.include_special !== -1 && i(N);
        });
      }
      var O = y(I, 2);
      {
        var Q = (i) => {
          var e = tl(), t = r(e);
          o(e), g(() => {
            var _a;
            return u(t, `${_.passwordPolicy.notRecent ?? ""}
                    ${((_a = l.policy) == null ? void 0 : _a.not_recently_used) || 0}`);
          }), v(i, e);
        };
        p(O, (i) => {
          l.policy.not_recently_used !== -1 && i(Q);
        });
      }
      o(n), o(s), g(() => {
        var _a;
        u(w, _.passwordPolicy.passwordPolicy), b = h(P, 1, "li svelte-u5h3xg", null, b, { policyErr: !!m(d)[0] }), u(G, `${_.passwordPolicy.lengthMin ?? ""}
                ${l.policy.length_min ?? ""}`), k = h(E, 1, "li svelte-u5h3xg", null, k, { policyErr: !!m(d)[1] }), u(H, `${_.passwordPolicy.lengthMax ?? ""}
                ${((_a = l.policy) == null ? void 0 : _a.length_max) || 0}`);
      }), v(c, s);
    };
    p(D, (c) => {
      l.policy && c(F);
    });
  }
  v(q, M), Y();
}
export {
  vl as P
};
