import { t as j, a as m, d as se, e as oe } from "../chunks/DLBGyKVC.js";
import { p as ne, a6 as de, f as R, t as h, a as ue, s as i, l as c, k as S, c as s, r as o, j as e, a8 as ce } from "../chunks/CmQi0fbH.js";
import { s as _ } from "../chunks/BjaYyaa_.js";
import { i as T } from "../chunks/C6bK2EJJ.js";
import { p as J } from "../chunks/B_ggA-0N.js";
import { B as pe } from "../chunks/DPLO-ozG.js";
import { I as d } from "../chunks/DwPr_s7h.js";
import { f as _e, d as ve } from "../chunks/BO1A6s0c.js";
import { F as me } from "../chunks/CNMQp9ma.js";
import { u as fe } from "../chunks/CZf6fJph.js";
import { u as ge } from "../chunks/DGTOa5g8.js";
import { I as ye } from "../chunks/CAzQQhB1.js";
var he = j('<div class="err"> </div>'), we = j('<div class="row svelte-11d4h1"><!> <!></div> <div class="row svelte-11d4h1"><!> <!></div> <div class="row svelte-11d4h1"><!> <!></div> <p> <br> </p> <div class="row svelte-11d4h1"><!> <!></div> <div class="flex gap-05"><!> <!></div> <!>', 1), xe = j("<h2> </h2> <p> </p> <!>", 1);
function Pe(w, K) {
  ne(K, true);
  const n = "160px";
  let r = ge(), v = fe(), p = S(""), x = S(false), t = S(void 0);
  de(async () => {
    let u = await _e("/auth/v1/password_policy");
    if (u.body) {
      let l = u.body;
      l.include_lower_case || (l.include_lower_case = 0), l.include_upper_case || (l.include_upper_case = 0), l.include_digits || (l.include_digits = 0), l.include_special || (l.include_special = 0), l.not_recently_used || (l.not_recently_used = 0), l.valid_days || (l.valid_days = 0), c(t, J(l));
    } else console.error(u.error);
  });
  async function O(u, l) {
    if (c(p, ""), !e(t)) return;
    if (e(t).length_max < e(t).length_min) return c(p, "Max Length cannot be lower than Min Length"), false;
    if ((e(t).include_digits || 0) + (e(t).include_lower_case || 0) + (e(t).include_upper_case || 0) + (e(t).include_special || 0) > e(t).length_max) return c(p, "The sum of all includes does not fit into Max Length"), false;
    let $ = { length_min: e(t).length_min, length_max: e(t).length_max, not_recently_used: e(t).not_recently_used ? e(t).not_recently_used : void 0, valid_days: e(t).valid_days ? e(t).valid_days : void 0, include_digits: e(t).include_digits ? e(t).include_digits : void 0, include_lower_case: e(t).include_lower_case ? e(t).include_lower_case : void 0, include_upper_case: e(t).include_upper_case ? e(t).include_upper_case : void 0, include_special: e(t).include_special ? e(t).include_special : void 0 }, f = await ve(u.action, $);
    f.error ? c(p, J(f.error.message)) : (c(x, true), setTimeout(() => {
      c(x, false);
    }, 3e3));
  }
  var A = xe(), P = R(A), Q = s(P, true);
  o(P);
  var b = i(P, 2), U = s(b, true);
  o(b);
  var V = i(b, 2);
  me(V, { action: "/auth/v1/password_policy", onSubmit: O, children: (u, l) => {
    var M = se(), $ = R(M);
    {
      var f = (I) => {
        var C = we(), F = R(C), G = s(F);
        d(G, { typ: "number", get label() {
          return r.passwordPolicy.lengthMin;
        }, get placeholder() {
          return r.passwordPolicy.lengthMin;
        }, autocomplete: "off", min: "8", max: "128", required: true, width: n, get value() {
          return e(t).length_min;
        }, set value(a) {
          e(t).length_min = a;
        } });
        var X = i(G, 2);
        d(X, { typ: "number", get label() {
          return r.passwordPolicy.lengthMax;
        }, get placeholder() {
          return r.passwordPolicy.lengthMax;
        }, autocomplete: "off", min: "8", max: "128", required: true, width: n, get value() {
          return e(t).length_max;
        }, set value(a) {
          e(t).length_max = a;
        } }), o(F);
        var k = i(F, 2), N = s(k);
        d(N, { typ: "number", get label() {
          return r.passwordPolicy.lowercaseMin;
        }, get placeholder() {
          return r.passwordPolicy.lowercaseMin;
        }, autocomplete: "off", min: "0", max: "32", width: n, get value() {
          return e(t).include_lower_case;
        }, set value(a) {
          e(t).include_lower_case = a;
        } });
        var Y = i(N, 2);
        d(Y, { typ: "number", get label() {
          return r.passwordPolicy.uppercaseMin;
        }, get placeholder() {
          return r.passwordPolicy.uppercaseMin;
        }, autocomplete: "off", min: "0", max: "32", width: n, get value() {
          return e(t).include_upper_case;
        }, set value(a) {
          e(t).include_upper_case = a;
        } }), o(k);
        var D = i(k, 2), W = s(D);
        d(W, { typ: "number", get label() {
          return r.passwordPolicy.digitsMin;
        }, get placeholder() {
          return r.passwordPolicy.digitsMin;
        }, autocomplete: "off", min: "0", max: "32", width: n, get value() {
          return e(t).include_digits;
        }, set value(a) {
          e(t).include_digits = a;
        } });
        var Z = i(W, 2);
        d(Z, { typ: "number", get label() {
          return r.passwordPolicy.specialMin;
        }, get placeholder() {
          return r.passwordPolicy.specialMin;
        }, autocomplete: "off", min: "0", max: "32", width: n, get value() {
          return e(t).include_special;
        }, set value(a) {
          e(t).include_special = a;
        } }), o(D);
        var L = i(D, 2), z = s(L, true), ee = i(z, 2);
        o(L);
        var q = i(L, 2), E = s(q);
        d(E, { typ: "number", get label() {
          return r.passwordPolicy.notRecent;
        }, get placeholder() {
          return r.passwordPolicy.notRecent;
        }, autocomplete: "off", min: "0", max: "32", width: n, get value() {
          return e(t).not_recently_used;
        }, set value(a) {
          e(t).not_recently_used = a;
        } });
        var te = i(E, 2);
        d(te, { typ: "number", get label() {
          return v.passwordPolicy.validForDays;
        }, get placeholder() {
          return v.passwordPolicy.validForDays;
        }, autocomplete: "off", min: "0", max: "3650", width: n, get value() {
          return e(t).valid_days;
        }, set value(a) {
          e(t).valid_days = a;
        } }), o(q);
        var B = i(q, 2), H = s(B);
        pe(H, { type: "submit", children: (a, g) => {
          ce();
          var y = oe();
          h(() => _(y, r.common.save)), m(a, y);
        }, $$slots: { default: true } });
        var ae = i(H, 2);
        {
          var re = (a) => {
            ye(a, {});
          };
          T(ae, (a) => {
            e(x) && a(re);
          });
        }
        o(B);
        var le = i(B, 2);
        {
          var ie = (a) => {
            var g = he(), y = s(g, true);
            o(g), h(() => _(y, e(p))), m(a, g);
          };
          T(le, (a) => {
            e(p) && a(ie);
          });
        }
        h(() => {
          _(z, v.passwordPolicy.validityNew), _(ee, ` ${v.passwordPolicy.resetSet0 ?? ""}`);
        }), m(I, C);
      };
      T($, (I) => {
        e(t) && I(f);
      });
    }
    m(u, M);
  }, $$slots: { default: true } }), h(() => {
    _(Q, r.passwordPolicy.passwordPolicy), _(U, v.passwordPolicy.configDesc);
  }), m(w, A), ue();
}
function Te(w) {
  Pe(w, {});
}
export {
  Te as component
};
