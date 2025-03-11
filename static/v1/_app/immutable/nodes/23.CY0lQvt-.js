import { t as h, a as l, e as H } from "../chunks/CWf9OOFK.js";
import { p as xe, a6 as $e, f as M, a as ze, t as n, l as a, k as f, j as e, s as i, a7 as Le, c as o, r as s, aa as ie, a8 as V } from "../chunks/nlANaGLT.js";
import { h as Pe, s as d } from "../chunks/BmMHVVX3.js";
import { i as N } from "../chunks/DOjUa9u5.js";
import { e as Te, i as be } from "../chunks/B6azywu7.js";
import { p as c } from "../chunks/5u5qd9TD.js";
import { m as we, r as de } from "../chunks/B21bTIl7.js";
import { L as Ce } from "../chunks/xD9JfGV5.js";
import { I as Ee, f as Se } from "../chunks/6f3yten3.js";
import { B as F } from "../chunks/BEQMYyDu.js";
import { M as ye } from "../chunks/BDnQQV_p.js";
import { C as Ae } from "../chunks/DA40Z_in.js";
import { u as ke } from "../chunks/DwvS5LQk.js";
import { T as De } from "../chunks/C4fNG2bJ.js";
import { u as Ie } from "../chunks/DAXezkVN.js";
import { T as Me } from "../chunks/Afb0E4F7.js";
import { f as Ne, b as Re } from "../chunks/BO1A6s0c.js";
import { P as Be } from "../chunks/BRCxk8by.js";
var Ge = h('<div class="desc svelte-hzde4x"> </div> <!> <div><!></div>', 1), Oe = h('<div class="desc svelte-hzde4x"><p> </p> <p> </p></div>'), Ue = h('<div class="desc svelte-hzde4x"><p class="declined svelte-hzde4x"> </p> <p> </p></div>'), We = h('<li class="svelte-hzde4x"> </li>'), je = h('<div class="desc svelte-hzde4x"> <ul class="svelte-hzde4x"></ul></div> <div class="inline svelte-hzde4x"><!> <!></div>', 1), qe = h('<div class="container svelte-hzde4x"><div class="name svelte-hzde4x"><h2> </h2></div> <!> <div class="err svelte-hzde4x"> </div></div>'), He = h("<!> <!> <!>", 1), Ve = h("<!> <!>", 1);
function ct(ve, le) {
  xe(le, true);
  let t = ke(), J = f(void 0), g = f(""), R = f(false), S = f(false), x = f(8), B = f(void 0), K = f(false), Q = f(false), b = f("");
  $e(async () => {
    let z = Ie("code").get() || "";
    a(b, c(z));
    let w = await Ne("/auth/v1/oidc/sessioninfo");
    w.body ? a(J, c(w.body)) : z ? de(`device?code=${z}`) : de("device");
  });
  async function G($) {
    var _a, _b, _c;
    if (a(g, ""), e(R)) return;
    if (e(b).length < e(x)) {
      a(g, c(t.common.errTooShort));
      return;
    }
    if (e(b).length > e(x)) {
      a(g, c(t.common.errTooLong));
      return;
    }
    a(S, true);
    let z = await Se();
    if (!z) {
      a(g, "PoW error - please contact your administrator");
      return;
    }
    let w = { user_code: e(b), pow: z, device_accepted: $ }, m = await Re("/auth/v1/oidc/device/verify", w);
    m.status === 200 ? a(B, c(((_b = (_a = m.body) == null ? void 0 : _a.scopes) == null ? void 0 : _b.split(" ")) || ["openid"])) : m.status === 202 ? (a(K, true), setTimeout(() => {
      window.location.replace("/auth/v1/account?v=devices");
    }, 2e3)) : m.status === 204 ? a(Q, true) : m.status === 404 ? a(g, c(t.device.wrongOrExpired)) : (console.error(m), a(g, c((_c = m.error) == null ? void 0 : _c.message))), a(S, false);
  }
  var X = Ve();
  Pe(($) => {
    n(() => Le.title = (t == null ? void 0 : t.device.title) || "Device Authorization");
  });
  var Y = M(X);
  De(Y, { id: we, get value() {
    return e(x);
  }, set value($) {
    a(x, c($));
  } });
  var ne = i(Y, 2);
  ye(ne, { children: ($, z) => {
    Ae($, { children: (w, m) => {
      var Z = He(), ee = M(Z);
      {
        var ce = (O) => {
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
                a(b, c(r));
              }, get isError() {
                return e(R);
              }, set isError(r) {
                a(R, c(r));
              } });
              var E = i(u, 2), v = o(E);
              F(v, { onclick: () => G("pending"), get isLoading() {
                return e(S);
              }, children: (r, _) => {
                V();
                var T = H();
                n(() => d(T, t.device.submit)), l(r, T);
              }, $$slots: { default: true } }), s(E), n((r) => d(j, r), [() => t.device.desc.replace("{{count}}", e(x).toString())]), l(C, D);
            }, me = (C, D) => {
              {
                var y = (u) => {
                  var L = Oe(), P = o(L), E = o(P, true);
                  s(P);
                  var v = i(P, 2), r = o(v, true);
                  s(v), s(L), n(() => {
                    d(E, t.device.isAccepted), d(r, t.device.autoRedirectAccount);
                  }), l(u, L);
                }, j = (u, L) => {
                  {
                    var P = (v) => {
                      var r = Ue(), _ = o(r), T = o(_, true);
                      s(_);
                      var A = i(_, 2), I = o(A, true);
                      s(A), s(r), n(() => {
                        d(T, t.device.isDeclined), d(I, t.device.closeWindow);
                      }), l(v, r);
                    }, E = (v) => {
                      var r = je(), _ = M(r), T = o(_), A = i(T);
                      Te(A, 21, () => e(B), be, (k, q) => {
                        var p = We(), ge = o(p, true);
                        s(p), n(() => d(ge, e(q))), l(k, p);
                      }), s(A), s(_);
                      var I = i(_, 2), oe = o(I);
                      F(oe, { onclick: () => G("accept"), get isLoading() {
                        return e(S);
                      }, children: (k, q) => {
                        V();
                        var p = H();
                        n(() => d(p, t.device.accept)), l(k, p);
                      }, $$slots: { default: true } });
                      var he = i(oe, 2);
                      F(he, { level: -1, onclick: () => G("decline"), get isLoading() {
                        return e(S);
                      }, children: (k, q) => {
                        V();
                        var p = H();
                        n(() => d(p, t.device.decline)), l(k, p);
                      }, $$slots: { default: true } }), s(I), n(() => d(T, `${t.device.descScopes ?? ""} `)), l(v, r);
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
          s(se), s(U), n(() => {
            d(pe, t.device.title), d(_e, e(g));
          }), l(O, U);
        };
        N(ee, (O) => {
          e(J) && O(ce);
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
  ct as component
};
