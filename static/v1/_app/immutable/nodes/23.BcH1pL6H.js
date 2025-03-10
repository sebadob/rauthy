import { t as x, a as o, e as F, d as ve } from "../chunks/BH6NCLk-.js";
import { p as Le, a5 as Pe, f as E, a as Te, t as l, l as r, k as f, j as e, s as d, a6 as be, c as s, r as a, a9 as le, a7 as J } from "../chunks/CvlvO1XB.js";
import { h as we, s as v } from "../chunks/CTI4QPiR.js";
import { i as B } from "../chunks/BUO_AUgz.js";
import { e as Ce, i as Ee } from "../chunks/BpWRzPRQ.js";
import { p as n } from "../chunks/Wh68IIk2.js";
import { m as Se, r as ne } from "../chunks/Brp0G0eV.js";
import { L as ye } from "../chunks/CE6qm9-J.js";
import { I as Ae, f as ke } from "../chunks/CJP-ccgO.js";
import { B as K } from "../chunks/DMkkW5Nn.js";
import { M as De } from "../chunks/BbRujPvb.js";
import { C as Ie } from "../chunks/GWgockW8.js";
import { u as Me } from "../chunks/CUqQZdNU.js";
import { T as Ne } from "../chunks/Bb2Y1h4J.js";
import { u as Re } from "../chunks/D6tYT786.js";
import { T as Be } from "../chunks/Cbuxv9a6.js";
import { f as Ge, b as Oe } from "../chunks/CRjU5SuJ.js";
import { P as Ue } from "../chunks/BRCxk8by.js";
var We = x('<div class="desc svelte-hzde4x"> </div> <!> <div><!></div>', 1), je = x('<div class="desc svelte-hzde4x"><p> </p> <p> </p></div>'), qe = x('<div class="desc svelte-hzde4x"><p class="declined svelte-hzde4x"> </p> <p> </p></div>'), He = x('<li class="svelte-hzde4x"> </li>'), Ve = x('<div class="desc svelte-hzde4x"> <ul class="svelte-hzde4x"></ul></div> <div class="inline svelte-hzde4x"><!> <!></div>', 1), Fe = x('<div class="container svelte-hzde4x"><div class="name svelte-hzde4x"><h2> </h2></div> <!> <div class="err svelte-hzde4x"> </div></div>'), Je = x("<!> <!> <!>", 1), Ke = x("<!> <!>", 1);
function ft(ce, ue) {
  Le(ue, true);
  let t = Me(), Q = f(void 0), $ = f(""), G = f(false), S = f(false), z = f(8), O = f(void 0), X = f(false), Y = f(false), b = f("");
  Pe(async () => {
    let P = Re("code").get() || "";
    r(b, n(P));
    let w = await Ge("/auth/v1/oidc/sessioninfo");
    w.body ? r(Q, n(w.body)) : P ? ne(`device?code=${P}`) : ne("device");
  });
  async function U(L) {
    var _a, _b, _c;
    if (r($, ""), e(G)) return;
    if (e(b).length < e(z)) {
      r($, n(t.common.errTooShort));
      return;
    }
    if (e(b).length > e(z)) {
      r($, n(t.common.errTooLong));
      return;
    }
    r(S, true);
    let P = await ke();
    if (!P) {
      r($, "PoW error - please contact your administrator");
      return;
    }
    let w = { user_code: e(b), pow: P, device_accepted: L }, m = await Oe("/auth/v1/oidc/device/verify", w);
    m.status === 200 ? r(O, n(((_b = (_a = m.body) == null ? void 0 : _a.scopes) == null ? void 0 : _b.split(" ")) || ["openid"])) : m.status === 202 ? (r(X, true), setTimeout(() => {
      window.location.replace("/auth/v1/account?v=devices");
    }, 2e3)) : m.status === 204 ? r(Y, true) : m.status === 404 ? r($, n(t.device.wrongOrExpired)) : (console.error(m), r($, n((_c = m.error) == null ? void 0 : _c.message))), r(S, false);
  }
  var Z = Ke();
  we((L) => {
    l(() => be.title = (t == null ? void 0 : t.device.title) || "Device Authorization");
  });
  var ee = E(Z);
  Ne(ee, { id: Se, get value() {
    return e(z);
  }, set value(L) {
    r(z, n(L));
  } });
  var pe = d(ee, 2);
  De(pe, { children: (L, P) => {
    Ie(L, { children: (w, m) => {
      var te = Je(), re = E(te);
      {
        var fe = (W) => {
          var j = Fe(), q = s(j), se = s(q), _e = s(se, true);
          a(se), a(q);
          var oe = d(q, 2);
          {
            var he = (C) => {
              var y = We(), A = E(y), H = s(A, true);
              a(A);
              var M = d(A, 2);
              const _ = le(() => e(z).toString()), h = le(() => e(z).toString());
              Ae(M, { name: "userCode", autocomplete: "off", get label() {
                return t.device.userCode;
              }, get placeholder() {
                return t.device.userCode;
              }, required: true, get min() {
                return e(_);
              }, get max() {
                return e(h);
              }, pattern: Ue, get value() {
                return e(b);
              }, set value(i) {
                r(b, n(i));
              }, get isError() {
                return e(G);
              }, set isError(i) {
                r(G, n(i));
              } });
              var g = d(M, 2), k = s(g);
              K(k, { onclick: () => U("pending"), get isLoading() {
                return e(S);
              }, children: (i, c) => {
                J();
                var u = F();
                l(() => v(u, t.device.submit)), o(i, u);
              }, $$slots: { default: true } }), a(g), l((i) => v(H, i), [() => t.device.desc.replace("{{count}}", e(z).toString())]), o(C, y);
            }, ge = (C) => {
              var y = ve(), A = E(y);
              {
                var H = (_) => {
                  var h = je(), g = s(h), k = s(g, true);
                  a(g);
                  var i = d(g, 2), c = s(i, true);
                  a(i), a(h), l(() => {
                    v(k, t.device.isAccepted), v(c, t.device.autoRedirectAccount);
                  }), o(_, h);
                }, M = (_) => {
                  var h = ve(), g = E(h);
                  {
                    var k = (c) => {
                      var u = qe(), T = s(u), N = s(T, true);
                      a(T);
                      var D = d(T, 2), R = s(D, true);
                      a(D), a(u), l(() => {
                        v(N, t.device.isDeclined), v(R, t.device.closeWindow);
                      }), o(c, u);
                    }, i = (c) => {
                      var u = Ve(), T = E(u), N = s(T), D = d(N);
                      Ce(D, 21, () => e(O), Ee, (I, V) => {
                        var p = He(), ze = s(p, true);
                        a(p), l(() => v(ze, e(V))), o(I, p);
                      }), a(D), a(T);
                      var R = d(T, 2), de = s(R);
                      K(de, { onclick: () => U("accept"), get isLoading() {
                        return e(S);
                      }, children: (I, V) => {
                        J();
                        var p = F();
                        l(() => v(p, t.device.accept)), o(I, p);
                      }, $$slots: { default: true } });
                      var $e = d(de, 2);
                      K($e, { level: -1, onclick: () => U("decline"), get isLoading() {
                        return e(S);
                      }, children: (I, V) => {
                        J();
                        var p = F();
                        l(() => v(p, t.device.decline)), o(I, p);
                      }, $$slots: { default: true } }), a(R), l(() => v(N, `${t.device.descScopes ?? ""} `)), o(c, u);
                    };
                    B(g, (c) => {
                      e(Y) ? c(k) : c(i, false);
                    }, true);
                  }
                  o(_, h);
                };
                B(A, (_) => {
                  e(X) ? _(H) : _(M, false);
                }, true);
              }
              o(C, y);
            };
            B(oe, (C) => {
              e(O) === void 0 ? C(he) : C(ge, false);
            });
          }
          var ie = d(oe, 2), xe = s(ie, true);
          a(ie), a(j), l(() => {
            v(_e, t.device.title), v(xe, e($));
          }), o(W, j);
        };
        B(re, (W) => {
          e(Q) && W(fe);
        });
      }
      var ae = d(re, 2);
      Be(ae, { absolute: true });
      var me = d(ae, 2);
      ye(me, { absolute: true }), o(w, te);
    } });
  } }), o(ce, Z), Te();
}
export {
  ft as component
};
