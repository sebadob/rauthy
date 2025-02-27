import { t as h, a as m, d as se, e as ie } from "../chunks/BH6NCLk-.js";
import { p as ne, a5 as oe, f as q, t as w, a as de, s, l as c, k as B, c as n, r as o, j as e, a7 as ue } from "../chunks/CvlvO1XB.js";
import { s as v } from "../chunks/CTI4QPiR.js";
import { i as R } from "../chunks/BUO_AUgz.js";
import { p as J } from "../chunks/Wh68IIk2.js";
import { B as ce } from "../chunks/DMkkW5Nn.js";
import { I as d } from "../chunks/DmeAqnkr.js";
import { f as pe, d as _e } from "../chunks/bbkAiDd0.js";
import { F as me } from "../chunks/CqS1e6KT.js";
import { u as ve } from "../chunks/D8mHI_K9.js";
import { u as fe } from "../chunks/DOl8_ubJ.js";
var ge = h('<div class="success svelte-maj8s4">Success</div>'), ye = h('<div class="err"> </div>'), we = h('<div class="row svelte-maj8s4"><!> <!></div> <div class="row svelte-maj8s4"><!> <!></div> <div class="row svelte-maj8s4"><!> <!></div> <p> <br> </p> <div class="row svelte-maj8s4"><!> <!></div> <!> <!> <!>', 1), he = h("<h2> </h2> <p> </p> <!>", 1);
function xe(x, K) {
  ne(K, true);
  const i = "160px";
  let r = fe(), f = ve(), p = B(""), P = B(false), t = B(void 0);
  oe(async () => {
    let u = await pe("/auth/v1/password_policy");
    if (u.body) {
      let l = u.body;
      l.include_lower_case || (l.include_lower_case = 0), l.include_upper_case || (l.include_upper_case = 0), l.include_digits || (l.include_digits = 0), l.include_special || (l.include_special = 0), l.not_recently_used || (l.not_recently_used = 0), l.valid_days || (l.valid_days = 0), c(t, J(l));
    } else console.error(u.error);
  });
  async function O(u, l) {
    if (c(p, ""), !e(t)) return;
    if (e(t).length_max < e(t).length_min) return c(p, "Max Length cannot be lower than Min Length"), false;
    if ((e(t).include_digits || 0) + (e(t).include_lower_case || 0) + (e(t).include_upper_case || 0) + (e(t).include_special || 0) > e(t).length_max) return c(p, "The sum of all includes does not fit into Max Length"), false;
    let j = { length_min: e(t).length_min, length_max: e(t).length_max, not_recently_used: e(t).not_recently_used ? e(t).not_recently_used : void 0, valid_days: e(t).valid_days ? e(t).valid_days : void 0, include_digits: e(t).include_digits ? e(t).include_digits : void 0, include_lower_case: e(t).include_lower_case ? e(t).include_lower_case : void 0, include_upper_case: e(t).include_upper_case ? e(t).include_upper_case : void 0, include_special: e(t).include_special ? e(t).include_special : void 0 }, g = await _e(u.action, j);
    g.error ? c(p, J(g.error.message)) : (c(P, true), setTimeout(() => {
      c(P, false);
    }, 3e3));
  }
  var T = he(), b = q(T), Q = n(b, true);
  o(b);
  var M = s(b, 2), U = n(M, true);
  o(M);
  var V = s(M, 2);
  me(V, { action: "/auth/v1/password_policy", onSubmit: O, children: (u, l) => {
    var $ = se(), j = q($);
    {
      var g = (F) => {
        var A = we(), I = q(A), G = n(I);
        d(G, { typ: "number", get label() {
          return r.passwordPolicy.lengthMin;
        }, get placeholder() {
          return r.passwordPolicy.lengthMin;
        }, autocomplete: "off", min: "8", max: "128", required: true, width: i, get value() {
          return e(t).length_min;
        }, set value(a) {
          e(t).length_min = a;
        } });
        var X = s(G, 2);
        d(X, { typ: "number", get label() {
          return r.passwordPolicy.lengthMax;
        }, get placeholder() {
          return r.passwordPolicy.lengthMax;
        }, autocomplete: "off", min: "8", max: "128", required: true, width: i, get value() {
          return e(t).length_max;
        }, set value(a) {
          e(t).length_max = a;
        } }), o(I);
        var D = s(I, 2), N = n(D);
        d(N, { typ: "number", get label() {
          return r.passwordPolicy.lowercaseMin;
        }, get placeholder() {
          return r.passwordPolicy.lowercaseMin;
        }, autocomplete: "off", min: "0", max: "32", width: i, get value() {
          return e(t).include_lower_case;
        }, set value(a) {
          e(t).include_lower_case = a;
        } });
        var Y = s(N, 2);
        d(Y, { typ: "number", get label() {
          return r.passwordPolicy.uppercaseMin;
        }, get placeholder() {
          return r.passwordPolicy.uppercaseMin;
        }, autocomplete: "off", min: "0", max: "32", width: i, get value() {
          return e(t).include_upper_case;
        }, set value(a) {
          e(t).include_upper_case = a;
        } }), o(D);
        var L = s(D, 2), W = n(L);
        d(W, { typ: "number", get label() {
          return r.passwordPolicy.digitsMin;
        }, get placeholder() {
          return r.passwordPolicy.digitsMin;
        }, autocomplete: "off", min: "0", max: "32", width: i, get value() {
          return e(t).include_digits;
        }, set value(a) {
          e(t).include_digits = a;
        } });
        var Z = s(W, 2);
        d(Z, { typ: "number", get label() {
          return r.passwordPolicy.specialMin;
        }, get placeholder() {
          return r.passwordPolicy.specialMin;
        }, autocomplete: "off", min: "0", max: "32", width: i, get value() {
          return e(t).include_special;
        }, set value(a) {
          e(t).include_special = a;
        } }), o(L);
        var S = s(L, 2), z = n(S, true), ee = s(z, 2);
        o(S);
        var k = s(S, 2), C = n(k);
        d(C, { typ: "number", get label() {
          return r.passwordPolicy.notRecent;
        }, get placeholder() {
          return r.passwordPolicy.notRecent;
        }, autocomplete: "off", min: "0", max: "32", width: i, get value() {
          return e(t).not_recently_used;
        }, set value(a) {
          e(t).not_recently_used = a;
        } });
        var te = s(C, 2);
        d(te, { typ: "number", get label() {
          return f.passwordPolicy.validForDays;
        }, get placeholder() {
          return f.passwordPolicy.validForDays;
        }, autocomplete: "off", min: "0", max: "3650", width: i, get value() {
          return e(t).valid_days;
        }, set value(a) {
          e(t).valid_days = a;
        } }), o(k);
        var E = s(k, 2);
        ce(E, { type: "submit", children: (a, _) => {
          ue();
          var y = ie();
          w(() => v(y, r.common.save)), m(a, y);
        }, $$slots: { default: true } });
        var H = s(E, 2);
        {
          var ae = (a) => {
            var _ = ge();
            m(a, _);
          };
          R(H, (a) => {
            e(P) && a(ae);
          });
        }
        var re = s(H, 2);
        {
          var le = (a) => {
            var _ = ye(), y = n(_, true);
            o(_), w(() => v(y, e(p))), m(a, _);
          };
          R(re, (a) => {
            e(p) && a(le);
          });
        }
        w(() => {
          v(z, f.passwordPolicy.validityNew), v(ee, ` ${f.passwordPolicy.resetSet0 ?? ""}`);
        }), m(F, A);
      };
      R(j, (F) => {
        e(t) && F(g);
      });
    }
    m(u, $);
  }, $$slots: { default: true } }), w(() => {
    v(Q, r.passwordPolicy.passwordPolicy), v(U, f.passwordPolicy.configDesc);
  }), m(x, T), de();
}
function qe(x) {
  xe(x, {});
}
export {
  qe as component
};
