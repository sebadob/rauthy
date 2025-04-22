import { t as _, a as l, e as q } from "../chunks/BxmJRzoY.js";
import { p as ge, j as p, a0 as xe, f as I, a as $e, t as c, l as a, k as e, s as i, a2 as ze, c as o, r as s, a4 as oe, a3 as H } from "../chunks/w0HvPX0p.js";
import { h as Le, s as d } from "../chunks/BzP2S3Z_.js";
import { i as M } from "../chunks/iO9_dPNE.js";
import { e as Pe, i as Te } from "../chunks/S81raU5Y.js";
import { m as be, r as ie } from "../chunks/B21bTIl7.js";
import { L as we } from "../chunks/Cmb3Gob_.js";
import { I as Ce, f as Ee } from "../chunks/Bk5EVqw2.js";
import { B as V } from "../chunks/C4AV2CoD.js";
import { M as Se } from "../chunks/DKM0QPz5.js";
import { C as ye } from "../chunks/QNragXLc.js";
import { u as Ae } from "../chunks/N6FgGI8m.js";
import { T as ke } from "../chunks/Dhcsa8BW.js";
import { u as De } from "../chunks/DvTygCXn.js";
import { T as Ie } from "../chunks/B808p9S3.js";
import { f as Me, b as Ne } from "../chunks/UPFlzoow.js";
import { P as Re } from "../chunks/gfDO7tLr.js";
var Be = _('<div class="desc svelte-hzde4x"> </div> <!> <div><!></div>', 1), Ge = _('<div class="desc svelte-hzde4x"><p> </p> <p> </p></div>'), Oe = _('<div class="desc svelte-hzde4x"><p class="declined svelte-hzde4x"> </p> <p> </p></div>'), Ue = _('<li class="svelte-hzde4x"> </li>'), We = _('<div class="desc svelte-hzde4x"> <ul class="svelte-hzde4x"></ul></div> <div class="inline svelte-hzde4x"><!> <!></div>', 1), je = _('<div class="container svelte-hzde4x"><div class="name svelte-hzde4x"><h2> </h2></div> <!> <div class="err svelte-hzde4x"> </div></div>'), qe = _("<!> <!> <!>", 1), He = _("<!> <!>", 1);
function lt(de, ve) {
  ge(ve, true);
  let t = Ae(), F = p(void 0), h = p(""), N = p(false), E = p(false), g = p(8), R = p(void 0), J = p(false), K = p(false), T = p("");
  xe(async () => {
    let $ = De("code").get() || "";
    a(T, $, true);
    let b = await Me("/auth/v1/oidc/sessioninfo");
    b.body ? a(F, b.body, true) : $ ? ie(`device?code=${$}`) : ie("device");
  });
  async function B(x) {
    var _a, _b, _c;
    if (a(h, ""), e(N)) return;
    if (e(T).length < e(g)) {
      a(h, t.common.errTooShort, true);
      return;
    }
    if (e(T).length > e(g)) {
      a(h, t.common.errTooLong, true);
      return;
    }
    a(E, true);
    let $ = await Ee();
    if (!$) {
      a(h, "PoW error - please contact your administrator");
      return;
    }
    let b = { user_code: e(T), pow: $, device_accepted: x }, f = await Ne("/auth/v1/oidc/device/verify", b);
    f.status === 200 ? a(R, ((_b = (_a = f.body) == null ? void 0 : _a.scopes) == null ? void 0 : _b.split(" ")) || ["openid"], true) : f.status === 202 ? (a(J, true), setTimeout(() => {
      window.location.replace("/auth/v1/account?v=devices");
    }, 2e3)) : f.status === 204 ? a(K, true) : f.status === 404 ? a(h, t.device.wrongOrExpired, true) : (console.error(f), a(h, (_c = f.error) == null ? void 0 : _c.message, true)), a(E, false);
  }
  var Q = He();
  Le((x) => {
    c(() => ze.title = (t == null ? void 0 : t.device.title) || "Device Authorization");
  });
  var X = I(Q);
  ke(X, { id: be, get value() {
    return e(g);
  }, set value(x) {
    a(g, x, true);
  } });
  var le = i(X, 2);
  Se(le, { children: (x, $) => {
    ye(x, { children: (b, f) => {
      var Y = qe(), Z = I(Y);
      {
        var ce = (G) => {
          var O = je(), U = o(O), te = o(U), ue = o(te, true);
          s(te), s(U);
          var re = i(U, 2);
          {
            var pe = (w) => {
              var k = Be(), S = I(k), W = o(S, true);
              s(S);
              var n = i(S, 2);
              const z = oe(() => e(g).toString()), L = oe(() => e(g).toString());
              Ce(n, { name: "userCode", autocomplete: "off", get label() {
                return t.device.userCode;
              }, get placeholder() {
                return t.device.userCode;
              }, required: true, get min() {
                return e(z);
              }, get max() {
                return e(L);
              }, pattern: Re, get value() {
                return e(T);
              }, set value(r) {
                a(T, r, true);
              }, get isError() {
                return e(N);
              }, set isError(r) {
                a(N, r, true);
              } });
              var C = i(n, 2), v = o(C);
              V(v, { onclick: () => B("pending"), get isLoading() {
                return e(E);
              }, children: (r, m) => {
                H();
                var P = q();
                c(() => d(P, t.device.submit)), l(r, P);
              }, $$slots: { default: true } }), s(C), c((r) => d(W, r), [() => t.device.desc.replace("{{count}}", e(g).toString())]), l(w, k);
            }, fe = (w, k) => {
              {
                var S = (n) => {
                  var z = Ge(), L = o(z), C = o(L, true);
                  s(L);
                  var v = i(L, 2), r = o(v, true);
                  s(v), s(z), c(() => {
                    d(C, t.device.isAccepted), d(r, t.device.autoRedirectAccount);
                  }), l(n, z);
                }, W = (n, z) => {
                  {
                    var L = (v) => {
                      var r = Oe(), m = o(r), P = o(m, true);
                      s(m);
                      var y = i(m, 2), D = o(y, true);
                      s(y), s(r), c(() => {
                        d(P, t.device.isDeclined), d(D, t.device.closeWindow);
                      }), l(v, r);
                    }, C = (v) => {
                      var r = We(), m = I(r), P = o(m), y = i(P);
                      Pe(y, 21, () => e(R), Te, (A, j) => {
                        var u = Ue(), he = o(u, true);
                        s(u), c(() => d(he, e(j))), l(A, u);
                      }), s(y), s(m);
                      var D = i(m, 2), se = o(D);
                      V(se, { onclick: () => B("accept"), get isLoading() {
                        return e(E);
                      }, children: (A, j) => {
                        H();
                        var u = q();
                        c(() => d(u, t.device.accept)), l(A, u);
                      }, $$slots: { default: true } });
                      var _e = i(se, 2);
                      V(_e, { level: -1, onclick: () => B("decline"), get isLoading() {
                        return e(E);
                      }, children: (A, j) => {
                        H();
                        var u = q();
                        c(() => d(u, t.device.decline)), l(A, u);
                      }, $$slots: { default: true } }), s(D), c(() => d(P, `${t.device.descScopes ?? ""} `)), l(v, r);
                    };
                    M(n, (v) => {
                      e(K) ? v(L) : v(C, false);
                    }, z);
                  }
                };
                M(w, (n) => {
                  e(J) ? n(S) : n(W, false);
                }, k);
              }
            };
            M(re, (w) => {
              e(R) === void 0 ? w(pe) : w(fe, false);
            });
          }
          var ae = i(re, 2), me = o(ae, true);
          s(ae), s(O), c(() => {
            d(ue, t.device.title), d(me, e(h));
          }), l(G, O);
        };
        M(Z, (G) => {
          e(F) && G(ce);
        });
      }
      var ee = i(Z, 2);
      Ie(ee, { absolute: true });
      var ne = i(ee, 2);
      we(ne, { absolute: true }), l(b, Y);
    } });
  } }), l(de, Q), $e();
}
export {
  lt as component
};
