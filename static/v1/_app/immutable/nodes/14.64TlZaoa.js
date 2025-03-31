import { t as j, a as m, d as ie, e as se } from "../chunks/BxmJRzoY.js";
import { p as ne, j as R, a0 as oe, f as S, t as h, a as de, s as i, l as c, c as s, r as n, k as e, a3 as ue } from "../chunks/w0HvPX0p.js";
import { s as _ } from "../chunks/BzP2S3Z_.js";
import { i as T } from "../chunks/iO9_dPNE.js";
import { B as ce } from "../chunks/C8YTstTD.js";
import { I as d } from "../chunks/Q4PIg3iI.js";
import { f as pe, d as _e } from "../chunks/BO1A6s0c.js";
import { F as ve } from "../chunks/CDe-qvZi.js";
import { u as me } from "../chunks/DHOKTGcE.js";
import { u as fe } from "../chunks/0cG6LBdy.js";
import { I as ge } from "../chunks/CTshzOVc.js";
var ye = j('<div class="err"> </div>'), he = j('<div class="row svelte-11d4h1"><!> <!></div> <div class="row svelte-11d4h1"><!> <!></div> <div class="row svelte-11d4h1"><!> <!></div> <p> <br> </p> <div class="row svelte-11d4h1"><!> <!></div> <div class="flex gap-05"><!> <!></div> <!>', 1), we = j("<h2> </h2> <p> </p> <!>", 1);
function xe(w, J) {
  ne(J, true);
  const o = "160px";
  let r = fe(), v = me(), p = R(""), x = R(false), t = R(void 0);
  oe(async () => {
    let u = await pe("/auth/v1/password_policy");
    if (u.body) {
      let l = u.body;
      l.include_lower_case || (l.include_lower_case = 0), l.include_upper_case || (l.include_upper_case = 0), l.include_digits || (l.include_digits = 0), l.include_special || (l.include_special = 0), l.not_recently_used || (l.not_recently_used = 0), l.valid_days || (l.valid_days = 0), c(t, l, true);
    } else console.error(u.error);
  });
  async function K(u, l) {
    if (c(p, ""), !e(t)) return;
    if (e(t).length_max < e(t).length_min) return c(p, "Max Length cannot be lower than Min Length"), false;
    if ((e(t).include_digits || 0) + (e(t).include_lower_case || 0) + (e(t).include_upper_case || 0) + (e(t).include_special || 0) > e(t).length_max) return c(p, "The sum of all includes does not fit into Max Length"), false;
    let $ = { length_min: e(t).length_min, length_max: e(t).length_max, not_recently_used: e(t).not_recently_used ? e(t).not_recently_used : void 0, valid_days: e(t).valid_days ? e(t).valid_days : void 0, include_digits: e(t).include_digits ? e(t).include_digits : void 0, include_lower_case: e(t).include_lower_case ? e(t).include_lower_case : void 0, include_upper_case: e(t).include_upper_case ? e(t).include_upper_case : void 0, include_special: e(t).include_special ? e(t).include_special : void 0 }, f = await _e(u.action, $);
    f.error ? c(p, f.error.message, true) : (c(x, true), setTimeout(() => {
      c(x, false);
    }, 3e3));
  }
  var A = we(), P = S(A), O = s(P, true);
  n(P);
  var b = i(P, 2), Q = s(b, true);
  n(b);
  var U = i(b, 2);
  ve(U, { action: "/auth/v1/password_policy", onSubmit: K, children: (u, l) => {
    var M = ie(), $ = S(M);
    {
      var f = (I) => {
        var C = he(), F = S(C), G = s(F);
        d(G, { typ: "number", get label() {
          return r.passwordPolicy.lengthMin;
        }, get placeholder() {
          return r.passwordPolicy.lengthMin;
        }, autocomplete: "off", min: "8", max: "128", required: true, width: o, get value() {
          return e(t).length_min;
        }, set value(a) {
          e(t).length_min = a;
        } });
        var V = i(G, 2);
        d(V, { typ: "number", get label() {
          return r.passwordPolicy.lengthMax;
        }, get placeholder() {
          return r.passwordPolicy.lengthMax;
        }, autocomplete: "off", min: "8", max: "128", required: true, width: o, get value() {
          return e(t).length_max;
        }, set value(a) {
          e(t).length_max = a;
        } }), n(F);
        var k = i(F, 2), N = s(k);
        d(N, { typ: "number", get label() {
          return r.passwordPolicy.lowercaseMin;
        }, get placeholder() {
          return r.passwordPolicy.lowercaseMin;
        }, autocomplete: "off", min: "0", max: "32", width: o, get value() {
          return e(t).include_lower_case;
        }, set value(a) {
          e(t).include_lower_case = a;
        } });
        var X = i(N, 2);
        d(X, { typ: "number", get label() {
          return r.passwordPolicy.uppercaseMin;
        }, get placeholder() {
          return r.passwordPolicy.uppercaseMin;
        }, autocomplete: "off", min: "0", max: "32", width: o, get value() {
          return e(t).include_upper_case;
        }, set value(a) {
          e(t).include_upper_case = a;
        } }), n(k);
        var D = i(k, 2), W = s(D);
        d(W, { typ: "number", get label() {
          return r.passwordPolicy.digitsMin;
        }, get placeholder() {
          return r.passwordPolicy.digitsMin;
        }, autocomplete: "off", min: "0", max: "32", width: o, get value() {
          return e(t).include_digits;
        }, set value(a) {
          e(t).include_digits = a;
        } });
        var Y = i(W, 2);
        d(Y, { typ: "number", get label() {
          return r.passwordPolicy.specialMin;
        }, get placeholder() {
          return r.passwordPolicy.specialMin;
        }, autocomplete: "off", min: "0", max: "32", width: o, get value() {
          return e(t).include_special;
        }, set value(a) {
          e(t).include_special = a;
        } }), n(D);
        var L = i(D, 2), z = s(L, true), Z = i(z, 2);
        n(L);
        var q = i(L, 2), E = s(q);
        d(E, { typ: "number", get label() {
          return r.passwordPolicy.notRecent;
        }, get placeholder() {
          return r.passwordPolicy.notRecent;
        }, autocomplete: "off", min: "0", max: "32", width: o, get value() {
          return e(t).not_recently_used;
        }, set value(a) {
          e(t).not_recently_used = a;
        } });
        var ee = i(E, 2);
        d(ee, { typ: "number", get label() {
          return v.passwordPolicy.validForDays;
        }, get placeholder() {
          return v.passwordPolicy.validForDays;
        }, autocomplete: "off", min: "0", max: "3650", width: o, get value() {
          return e(t).valid_days;
        }, set value(a) {
          e(t).valid_days = a;
        } }), n(q);
        var B = i(q, 2), H = s(B);
        ce(H, { type: "submit", children: (a, g) => {
          ue();
          var y = se();
          h(() => _(y, r.common.save)), m(a, y);
        }, $$slots: { default: true } });
        var te = i(H, 2);
        {
          var ae = (a) => {
            ge(a, {});
          };
          T(te, (a) => {
            e(x) && a(ae);
          });
        }
        n(B);
        var re = i(B, 2);
        {
          var le = (a) => {
            var g = ye(), y = s(g, true);
            n(g), h(() => _(y, e(p))), m(a, g);
          };
          T(re, (a) => {
            e(p) && a(le);
          });
        }
        h(() => {
          _(z, v.passwordPolicy.validityNew), _(Z, ` ${v.passwordPolicy.resetSet0 ?? ""}`);
        }), m(I, C);
      };
      T($, (I) => {
        e(t) && I(f);
      });
    }
    m(u, M);
  }, $$slots: { default: true } }), h(() => {
    _(O, r.passwordPolicy.passwordPolicy), _(Q, v.passwordPolicy.configDesc);
  }), m(w, A), de();
}
function Re(w) {
  xe(w, {});
}
export {
  Re as component
};
