import { d as T, a as y, t as w } from "./YHhP1LbZ.js";
import { p as U, k as V, aa as W, l as X, f as Y, a as Z, c as r, r as o, s as g, t as m, j as p } from "./Ck6jKiur.js";
import { s as u } from "./tDAaDMC_.js";
import { i as h } from "./7JDmqCCW.js";
import { b as x } from "./BTaFr7HN.js";
import { p as q } from "./ho0YXExL.js";
import { p as $ } from "./DZP54pO_.js";
import { u as ll } from "./mN05BXqA.js";
var el = w("<li> </li>"), il = w("<li> </li>"), al = w("<li> </li>"), tl = w("<li> </li>"), cl = w('<li class="li svelte-u5h3xg"> </li>'), sl = w('<div class="policyContainer svelte-u5h3xg"><b class="svelte-u5h3xg"> </b> <ul class="svelte-u5h3xg"><li> </li> <li> </li> <!> <!> <!> <!> <!></ul></div>');
function yl(z, l) {
  U(l, true);
  let B = $(l, "accepted", 15, false), _ = ll(), d = V(q([false, false, false, false, false, false]));
  W(() => {
    D();
  });
  function D() {
    if (!l.policy) return false;
    let c = [false, false, false, false, false, false], s = false;
    l.password.length < l.policy.length_min && (c[0] = true, s = true), l.password.length > l.policy.length_max && (c[1] = true, s = true);
    let a = [0, 0, 0, 0];
    for (let P = 0; P < l.password.length; P++) {
      let n = l.password.charCodeAt(P);
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
    l.policy.include_lower_case && l.policy.include_lower_case !== -1 && l.policy.include_lower_case > a[0] && (c[2] = true, s = true), l.policy.include_upper_case && l.policy.include_upper_case !== -1 && l.policy.include_upper_case > a[1] && (c[3] = true, s = true), l.policy.include_digits && l.policy.include_digits !== -1 && l.policy.include_digits > a[2] && (c[4] = true, s = true), l.policy.include_special && l.policy.include_special !== -1 && l.policy.include_special > a[3] && (c[5] = true, s = true), X(d, q(c)), B(!s);
  }
  var M = T(), F = Y(M);
  {
    var G = (c) => {
      var s = sl(), a = r(s), P = r(a, true);
      o(a);
      var n = g(a, 2), E = r(n);
      let k;
      var H = r(E);
      o(E);
      var b = g(E, 2);
      let C;
      var J = r(b);
      o(b);
      var j = g(b, 2);
      {
        var K = (i) => {
          var e = el();
          let t;
          var f = r(e);
          o(e), m((v) => {
            var _a;
            t = x(e, 1, "li svelte-u5h3xg", null, t, v), u(f, `${_.passwordPolicy.lowercaseMin ?? ""}
                    ${((_a = l.policy) == null ? void 0 : _a.include_lower_case) || 0}`);
          }, [() => ({ policyErr: !!p(d)[2] })]), y(i, e);
        };
        h(j, (i) => {
          l.policy.include_lower_case !== -1 && i(K);
        });
      }
      var A = g(j, 2);
      {
        var L = (i) => {
          var e = il();
          let t;
          var f = r(e);
          o(e), m((v) => {
            var _a;
            t = x(e, 1, "li svelte-u5h3xg", null, t, v), u(f, `${_.passwordPolicy.uppercaseMin ?? ""}
                    ${((_a = l.policy) == null ? void 0 : _a.include_upper_case) || 0}`);
          }, [() => ({ policyErr: !!p(d)[3] })]), y(i, e);
        };
        h(A, (i) => {
          l.policy.include_upper_case !== -1 && i(L);
        });
      }
      var I = g(A, 2);
      {
        var N = (i) => {
          var e = al();
          let t;
          var f = r(e);
          o(e), m((v) => {
            var _a;
            t = x(e, 1, "li svelte-u5h3xg", null, t, v), u(f, `${_.passwordPolicy.digitsMin ?? ""}
                    ${((_a = l.policy) == null ? void 0 : _a.include_digits) || 0}`);
          }, [() => ({ policyErr: !!p(d)[4] })]), y(i, e);
        };
        h(I, (i) => {
          l.policy.include_digits !== -1 && i(N);
        });
      }
      var R = g(I, 2);
      {
        var O = (i) => {
          var e = tl();
          let t;
          var f = r(e);
          o(e), m((v) => {
            var _a;
            t = x(e, 1, "li svelte-u5h3xg", null, t, v), u(f, `${_.passwordPolicy.specialMin ?? ""}
                    ${((_a = l.policy) == null ? void 0 : _a.include_special) || 0}`);
          }, [() => ({ policyErr: !!p(d)[5] })]), y(i, e);
        };
        h(R, (i) => {
          l.policy.include_special !== -1 && i(O);
        });
      }
      var Q = g(R, 2);
      {
        var S = (i) => {
          var e = cl(), t = r(e);
          o(e), m(() => {
            var _a;
            return u(t, `${_.passwordPolicy.notRecent ?? ""}
                    ${((_a = l.policy) == null ? void 0 : _a.not_recently_used) || 0}`);
          }), y(i, e);
        };
        h(Q, (i) => {
          l.policy.not_recently_used !== -1 && i(S);
        });
      }
      o(n), o(s), m((i, e) => {
        var _a;
        u(P, _.passwordPolicy.passwordPolicy), k = x(E, 1, "li svelte-u5h3xg", null, k, i), u(H, `${_.passwordPolicy.lengthMin ?? ""}
                ${l.policy.length_min ?? ""}`), C = x(b, 1, "li svelte-u5h3xg", null, C, e), u(J, `${_.passwordPolicy.lengthMax ?? ""}
                ${((_a = l.policy) == null ? void 0 : _a.length_max) || 0}`);
      }, [() => ({ policyErr: !!p(d)[0] }), () => ({ policyErr: !!p(d)[1] })]), y(c, s);
    };
    h(F, (c) => {
      l.policy && c(G);
    });
  }
  y(z, M), Z();
}
export {
  yl as P
};
