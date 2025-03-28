import { d as S, a as f, t as w } from "./BxmJRzoY.js";
import { p as T, j as U, a1 as V, a5 as W, l as X, f as Y, a as Z, c as r, r as o, s as g, t as h, k as x } from "./w0HvPX0p.js";
import { s as _ } from "./BzP2S3Z_.js";
import { i as m } from "./iO9_dPNE.js";
import { b as p } from "./BdbQ6g_y.js";
import { p as $ } from "./C6GSeq7M.js";
import { u as ll } from "./0cG6LBdy.js";
var el = w("<li> </li>"), il = w("<li> </li>"), al = w("<li> </li>"), tl = w("<li> </li>"), cl = w('<li class="li svelte-u5h3xg"> </li>'), sl = w('<div class="policyContainer svelte-u5h3xg"><b class="svelte-u5h3xg"> </b> <ul class="svelte-u5h3xg"><li> </li> <li> </li> <!> <!> <!> <!> <!></ul></div>');
function yl(q, l) {
  T(l, true);
  let z = $(l, "accepted", 15, false), u = ll(), d = U(V([false, false, false, false, false, false]));
  W(() => {
    B();
  });
  function B() {
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
    l.policy.include_lower_case && l.policy.include_lower_case !== -1 && l.policy.include_lower_case > a[0] && (c[2] = true, s = true), l.policy.include_upper_case && l.policy.include_upper_case !== -1 && l.policy.include_upper_case > a[1] && (c[3] = true, s = true), l.policy.include_digits && l.policy.include_digits !== -1 && l.policy.include_digits > a[2] && (c[4] = true, s = true), l.policy.include_special && l.policy.include_special !== -1 && l.policy.include_special > a[3] && (c[5] = true, s = true), X(d, c, true), z(!s);
  }
  var M = S(), D = Y(M);
  {
    var F = (c) => {
      var s = sl(), a = r(s), P = r(a, true);
      o(a);
      var n = g(a, 2), E = r(n);
      let k;
      var G = r(E);
      o(E);
      var b = g(E, 2);
      let C;
      var H = r(b);
      o(b);
      var j = g(b, 2);
      {
        var J = (i) => {
          var e = el();
          let t;
          var v = r(e);
          o(e), h((y) => {
            var _a;
            t = p(e, 1, "li svelte-u5h3xg", null, t, y), _(v, `${u.passwordPolicy.lowercaseMin ?? ""}
                    ${((_a = l.policy) == null ? void 0 : _a.include_lower_case) || 0}`);
          }, [() => ({ policyErr: !!x(d)[2] })]), f(i, e);
        };
        m(j, (i) => {
          l.policy.include_lower_case !== -1 && i(J);
        });
      }
      var A = g(j, 2);
      {
        var K = (i) => {
          var e = il();
          let t;
          var v = r(e);
          o(e), h((y) => {
            var _a;
            t = p(e, 1, "li svelte-u5h3xg", null, t, y), _(v, `${u.passwordPolicy.uppercaseMin ?? ""}
                    ${((_a = l.policy) == null ? void 0 : _a.include_upper_case) || 0}`);
          }, [() => ({ policyErr: !!x(d)[3] })]), f(i, e);
        };
        m(A, (i) => {
          l.policy.include_upper_case !== -1 && i(K);
        });
      }
      var I = g(A, 2);
      {
        var L = (i) => {
          var e = al();
          let t;
          var v = r(e);
          o(e), h((y) => {
            var _a;
            t = p(e, 1, "li svelte-u5h3xg", null, t, y), _(v, `${u.passwordPolicy.digitsMin ?? ""}
                    ${((_a = l.policy) == null ? void 0 : _a.include_digits) || 0}`);
          }, [() => ({ policyErr: !!x(d)[4] })]), f(i, e);
        };
        m(I, (i) => {
          l.policy.include_digits !== -1 && i(L);
        });
      }
      var R = g(I, 2);
      {
        var N = (i) => {
          var e = tl();
          let t;
          var v = r(e);
          o(e), h((y) => {
            var _a;
            t = p(e, 1, "li svelte-u5h3xg", null, t, y), _(v, `${u.passwordPolicy.specialMin ?? ""}
                    ${((_a = l.policy) == null ? void 0 : _a.include_special) || 0}`);
          }, [() => ({ policyErr: !!x(d)[5] })]), f(i, e);
        };
        m(R, (i) => {
          l.policy.include_special !== -1 && i(N);
        });
      }
      var O = g(R, 2);
      {
        var Q = (i) => {
          var e = cl(), t = r(e);
          o(e), h(() => {
            var _a;
            return _(t, `${u.passwordPolicy.notRecent ?? ""}
                    ${((_a = l.policy) == null ? void 0 : _a.not_recently_used) || 0}`);
          }), f(i, e);
        };
        m(O, (i) => {
          l.policy.not_recently_used !== -1 && i(Q);
        });
      }
      o(n), o(s), h((i, e) => {
        var _a;
        _(P, u.passwordPolicy.passwordPolicy), k = p(E, 1, "li svelte-u5h3xg", null, k, i), _(G, `${u.passwordPolicy.lengthMin ?? ""}
                ${l.policy.length_min ?? ""}`), C = p(b, 1, "li svelte-u5h3xg", null, C, e), _(H, `${u.passwordPolicy.lengthMax ?? ""}
                ${((_a = l.policy) == null ? void 0 : _a.length_max) || 0}`);
      }, [() => ({ policyErr: !!x(d)[0] }), () => ({ policyErr: !!x(d)[1] })]), f(c, s);
    };
    m(D, (c) => {
      l.policy && c(F);
    });
  }
  f(q, M), Z();
}
export {
  yl as P
};
