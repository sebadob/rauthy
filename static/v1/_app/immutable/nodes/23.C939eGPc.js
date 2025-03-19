import { t as h, a as l, e as H } from "../chunks/DLBGyKVC.js";
import { p as xe, a6 as $e, f as M, a as ze, t as c, l as a, k as f, j as e, s as i, a7 as Le, c as o, r as s, a9 as ie, a8 as V } from "../chunks/CmQi0fbH.js";
import { h as Pe, s as d } from "../chunks/BjaYyaa_.js";
import { i as N } from "../chunks/C6bK2EJJ.js";
import { e as Te, i as be } from "../chunks/YQCw2eEa.js";
import { p as n } from "../chunks/B_ggA-0N.js";
import { m as we, r as de } from "../chunks/B21bTIl7.js";
import { L as Ce } from "../chunks/DJEyp40I.js";
import { I as Ee, f as Se } from "../chunks/DwPr_s7h.js";
import { B as F } from "../chunks/DPLO-ozG.js";
import { M as ye } from "../chunks/Cf3VYlYr.js";
import { C as Ae } from "../chunks/CyYIiz5-.js";
import { u as ke } from "../chunks/DGTOa5g8.js";
import { T as De } from "../chunks/CLEj1OQz.js";
import { u as Ie } from "../chunks/BctQMYGV.js";
import { T as Me } from "../chunks/DB8eOlLu.js";
import { f as Ne, b as Re } from "../chunks/BO1A6s0c.js";
import { P as Be } from "../chunks/gfDO7tLr.js";
var Ge = h('<div class="desc svelte-hzde4x"> </div> <!> <div><!></div>', 1), Oe = h('<div class="desc svelte-hzde4x"><p> </p> <p> </p></div>'), Ue = h('<div class="desc svelte-hzde4x"><p class="declined svelte-hzde4x"> </p> <p> </p></div>'), We = h('<li class="svelte-hzde4x"> </li>'), je = h('<div class="desc svelte-hzde4x"> <ul class="svelte-hzde4x"></ul></div> <div class="inline svelte-hzde4x"><!> <!></div>', 1), qe = h('<div class="container svelte-hzde4x"><div class="name svelte-hzde4x"><h2> </h2></div> <!> <div class="err svelte-hzde4x"> </div></div>'), He = h("<!> <!> <!>", 1), Ve = h("<!> <!>", 1);
function nt(ve, le) {
  xe(le, true);
  let t = ke(), J = f(void 0), g = f(""), R = f(false), S = f(false), x = f(8), B = f(void 0), K = f(false), Q = f(false), b = f("");
  $e(async () => {
    let z = Ie("code").get() || "";
    a(b, n(z));
    let w = await Ne("/auth/v1/oidc/sessioninfo");
    w.body ? a(J, n(w.body)) : z ? de(`device?code=${z}`) : de("device");
  });
  async function G($) {
    var _a, _b, _c;
    if (a(g, ""), e(R)) return;
    if (e(b).length < e(x)) {
      a(g, n(t.common.errTooShort));
      return;
    }
    if (e(b).length > e(x)) {
      a(g, n(t.common.errTooLong));
      return;
    }
    a(S, true);
    let z = await Se();
    if (!z) {
      a(g, "PoW error - please contact your administrator");
      return;
    }
    let w = { user_code: e(b), pow: z, device_accepted: $ }, m = await Re("/auth/v1/oidc/device/verify", w);
    m.status === 200 ? a(B, n(((_b = (_a = m.body) == null ? void 0 : _a.scopes) == null ? void 0 : _b.split(" ")) || ["openid"])) : m.status === 202 ? (a(K, true), setTimeout(() => {
      window.location.replace("/auth/v1/account?v=devices");
    }, 2e3)) : m.status === 204 ? a(Q, true) : m.status === 404 ? a(g, n(t.device.wrongOrExpired)) : (console.error(m), a(g, n((_c = m.error) == null ? void 0 : _c.message))), a(S, false);
  }
  var X = Ve();
  Pe(($) => {
    c(() => Le.title = (t == null ? void 0 : t.device.title) || "Device Authorization");
  });
  var Y = M(X);
  De(Y, { id: we, get value() {
    return e(x);
  }, set value($) {
    a(x, n($));
  } });
  var ce = i(Y, 2);
  ye(ce, { children: ($, z) => {
    Ae($, { children: (w, m) => {
      var Z = He(), ee = M(Z);
      {
        var ne = (O) => {
          var U = qe(), W = o(U), re = o(W), pe = o(re, true);
          s(re), s(W);
          var ae = i(W, 2);
          {
            var fe = (C) => {
              var D = Ge(), y = M(D), j = o(y, true);
              s(y);
              var u = i(y, 2);
              const L = ie(() => e(x).toString()), P = ie(() => e(x).toString());
              Ee(u, { name: "userCode", autocomplete: "off", get label() {
                return t.device.userCode;
              }, get placeholder() {
                return t.device.userCode;
              }, required: true, get min() {
                return e(L);
              }, get max() {
                return e(P);
              }, pattern: Be, get value() {
                return e(b);
              }, set value(r) {
                a(b, n(r));
              }, get isError() {
                return e(R);
              }, set isError(r) {
                a(R, n(r));
              } });
              var E = i(u, 2), v = o(E);
              F(v, { onclick: () => G("pending"), get isLoading() {
                return e(S);
              }, children: (r, _) => {
                V();
                var T = H();
                c(() => d(T, t.device.submit)), l(r, T);
              }, $$slots: { default: true } }), s(E), c((r) => d(j, r), [() => t.device.desc.replace("{{count}}", e(x).toString())]), l(C, D);
            }, me = (C, D) => {
              {
                var y = (u) => {
                  var L = Oe(), P = o(L), E = o(P, true);
                  s(P);
                  var v = i(P, 2), r = o(v, true);
                  s(v), s(L), c(() => {
                    d(E, t.device.isAccepted), d(r, t.device.autoRedirectAccount);
                  }), l(u, L);
                }, j = (u, L) => {
                  {
                    var P = (v) => {
                      var r = Ue(), _ = o(r), T = o(_, true);
                      s(_);
                      var A = i(_, 2), I = o(A, true);
                      s(A), s(r), c(() => {
                        d(T, t.device.isDeclined), d(I, t.device.closeWindow);
                      }), l(v, r);
                    }, E = (v) => {
                      var r = je(), _ = M(r), T = o(_), A = i(T);
                      Te(A, 21, () => e(B), be, (k, q) => {
                        var p = We(), ge = o(p, true);
                        s(p), c(() => d(ge, e(q))), l(k, p);
                      }), s(A), s(_);
                      var I = i(_, 2), oe = o(I);
                      F(oe, { onclick: () => G("accept"), get isLoading() {
                        return e(S);
                      }, children: (k, q) => {
                        V();
                        var p = H();
                        c(() => d(p, t.device.accept)), l(k, p);
                      }, $$slots: { default: true } });
                      var he = i(oe, 2);
                      F(he, { level: -1, onclick: () => G("decline"), get isLoading() {
                        return e(S);
                      }, children: (k, q) => {
                        V();
                        var p = H();
                        c(() => d(p, t.device.decline)), l(k, p);
                      }, $$slots: { default: true } }), s(I), c(() => d(T, `${t.device.descScopes ?? ""} `)), l(v, r);
                    };
                    N(u, (v) => {
                      e(Q) ? v(P) : v(E, false);
                    }, L);
                  }
                };
                N(C, (u) => {
                  e(K) ? u(y) : u(j, false);
                }, D);
              }
            };
            N(ae, (C) => {
              e(B) === void 0 ? C(fe) : C(me, false);
            });
          }
          var se = i(ae, 2), _e = o(se, true);
          s(se), s(U), c(() => {
            d(pe, t.device.title), d(_e, e(g));
          }), l(O, U);
        };
        N(ee, (O) => {
          e(J) && O(ne);
        });
      }
      var te = i(ee, 2);
      Me(te, { absolute: true });
      var ue = i(te, 2);
      Ce(ue, { absolute: true }), l(w, Z);
    } });
  } }), l(ve, X), ze();
}
export {
  nt as component
};
