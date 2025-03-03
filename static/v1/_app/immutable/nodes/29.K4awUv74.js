import { t as f, a as s, n as kr, d as rr, e as me } from "../chunks/BH6NCLk-.js";
import { c as u, j as e, k as n, r as c, t as g, l as r, p as zr, a5 as Mr, aa as ne, f as O, a as Sr, s as v, a6 as Er, a9 as le, a7 as ge } from "../chunks/CvlvO1XB.js";
import { e as Ur, b as Cr, h as qr, s as M } from "../chunks/CTI4QPiR.js";
import { i as _ } from "../chunks/BUO_AUgz.js";
import { e as Ar } from "../chunks/BpWRzPRQ.js";
import { s as V, a as Fr } from "../chunks/BMbqVy6X.js";
import { p as o } from "../chunks/Wh68IIk2.js";
import { F as Nr, G as Wr, H as Dr, q as Br, I as Hr, j as Or, e as tr, l as Vr, J as Gr, P as Kr, K as Jr } from "../chunks/B8wC3kJv.js";
import { B as we } from "../chunks/DMkkW5Nn.js";
import { W as Qr } from "../chunks/BRsMuvuu.js";
import { I as Xr } from "../chunks/CeeD0cV_.js";
import { L as Yr } from "../chunks/BWe1X9CS.js";
import { p as Ee } from "../chunks/C6SR4G2t.js";
import { M as Zr } from "../chunks/BbRujPvb.js";
import { C as et } from "../chunks/GWgockW8.js";
import { u as rt } from "../chunks/BQ1-pLIs.js";
import { T as Z } from "../chunks/C8BaH7A7.js";
import { u as A } from "../chunks/BpfzNsA6.js";
import { T as tt } from "../chunks/C6vpM94V.js";
import { I as at } from "../chunks/D6E4lNPv.js";
import { b as de } from "../chunks/DYtiVhoA.js";
import { u as ar } from "../chunks/BMFqJ6Jy.js";
import { F as ot } from "../chunks/Gcs0S4Fz.js";
import { B as st } from "../chunks/BV-QGn6o.js";
import { R as it } from "../chunks/2DYY54Zs.js";
var nt = f('<img alt="Client Logo" width="100%" height="100%">'), lt = f('<div class="logo svelte-1gwy16z"><!></div>');
function dt(ee, F) {
  let N = n(false);
  var l = lt(), W = u(l);
  {
    var $ = (I) => {
      it(I, { width: "100%" });
    }, w = (I) => {
      var b = nt();
      g(() => V(b, "src", `/auth/v1/clients/${F.clientId}/logo`)), Ur("error", b, () => r(N, true)), Cr(b), s(I, b);
    };
    _(W, (I) => {
      e(N) ? I($) : I(w, false);
    });
  }
  c(l), s(ee, l);
}
var vt = kr(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125
            1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504
            1.125-1.125V9.75M8.25 21h8.25"></path></svg>`);
function ut(ee, F) {
  let N = Ee(F, "opacity", 3, 0.9), l = Ee(F, "width", 3, "1.5rem"), W = Ee(F, "color", 3, "currentColor");
  var $ = vt();
  V($, "stroke-width", 2), g(() => {
    V($, "stroke", W()), V($, "width", l()), V($, "opacity", N());
  }), s(ee, $);
}
var ct = f('<a class="home svelte-1pnhhpj"><!></a>'), ft = f('<div class="forgotten svelte-1pnhhpj"><!></div>'), ht = f("<!> <!>", 1), pt = f('<div class="btn flex-col svelte-1pnhhpj"><!></div>'), _t = f('<div class="btn flex-col svelte-1pnhhpj"><!></div>'), mt = f("<div><!></div> <!> <!>", 1), gt = f('<a class="reg svelte-1pnhhpj" target="_blank"> </a>'), wt = f('<a class="reg svelte-1pnhhpj" href="/auth/v1/users/register" target="_blank"> </a>'), bt = f("<!> <!>", 1), xt = f('<div class="errMsg svelte-1pnhhpj"> </div>'), Pt = f('<div class="success svelte-1pnhhpj"> </div>'), It = f('<div class="btn flex-col svelte-1pnhhpj"><!></div>'), Lt = f('<div class="providers flex-col svelte-1pnhhpj"><div class="providersSeparator svelte-1pnhhpj"><div class="separator svelte-1pnhhpj"></div> <div class="loginWith svelte-1pnhhpj"><div class="svelte-1pnhhpj"> </div></div></div> <!></div>'), yt = f('<div class="container svelte-1pnhhpj"><div class="head svelte-1pnhhpj"><!> <!></div> <div class="name svelte-1pnhhpj"><h2> </h2></div> <!> <!> <!> <!> <!> <!></div> <!> <!>', 1), Rt = f("<!> <!> <!> <!> <!> <!> <!>", 1);
function Zt(ee, F) {
  var _a;
  zr(F, true);
  const N = "18rem";
  let l = rt(), W = ar().get(), $ = le(() => W ? "/auth/v1/dev/authorize" : "/auth/v1/oidc/authorize"), w = A("client_id").get(), I = n(""), b = n(o(W ? "/auth/v1" : "")), G = A("redirect_uri").get(), be = A("nonce").get(), xe = ((_a = A("scope").get()) == null ? void 0 : _a.split(" ")) || [], Pe = n(void 0), Ie = n(void 0), Le = A("state").get(), re = A("code_challenge").get(), T = A("code_challenge_method").get(), Ue = false, ye = n(void 0), ve = n(o([])), ue = n(void 0), K = n(false), h = n(""), te = n(""), ce = n(""), S = n(false), D = n(false), fe = n(false), he = n(false), pe = n(false), B = n(false), Ce = n(""), Re = n(false), L = n(o(A("login_hint").get() || "")), E = n(""), $e = n(""), qe = le(() => e(S) && e(ye) !== e(L) && !e(fe));
  Mr(() => {
    var _a2;
    e(S) || ((_a2 = e(Pe)) == null ? void 0 : _a2.focus());
  }), ne(() => {
    Ue && (w == null ? void 0 : w.length) && ir();
  }), ne(() => {
    e(pe) && setTimeout(() => {
      r(pe, false), r(fe, false), r(he, false);
    }, 3e3);
  }), ne(() => {
    var _a2;
    (_a2 = e(Ie)) == null ? void 0 : _a2.focus();
  }), ne(() => {
    var _a2;
    if (e(te) === "Refresh") Ue = true;
    else if ((_a2 = e(te)) == null ? void 0 : _a2.startsWith("MfaLogin ")) {
      let t = e(te).replace("MfaLogin ", "");
      r(L, o(t)), r(ye, o(t));
    }
  }), ne(() => {
    W ? or() : e(ce) && tr(e(ce));
  });
  async function or() {
    var _a2;
    let t = await de("/auth/v1/oidc/session");
    ((_a2 = t.body) == null ? void 0 : _a2.csrf_token) ? tr(t.body.csrf_token) : console.error(t.error);
  }
  function sr() {
    r(h, ""), r(fe, true), r(E, "");
  }
  async function ir() {
    if (!w) {
      console.error("clientId is undefined");
      return;
    }
    if (!G) {
      console.error("redirectUri is undefined");
      return;
    }
    r(K, true);
    const t = { client_id: w, redirect_uri: G, state: Le, nonce: be, scopes: xe };
    re && T && (T === "plain" || T === "S256") && (t.code_challenge = re, t.code_challenge_method = T);
    let d = await de("/auth/v1/oidc/authorize/refresh", t);
    await Ae(d);
  }
  async function nr(t, d) {
    if (r(h, ""), !w) {
      console.error("clientId is undefined");
      return;
    }
    if (!G) {
      console.error("redirectUri is undefined");
      return;
    }
    const x = { email: e(L), client_id: w, redirect_uri: G, state: Le, nonce: be, scopes: xe };
    if (re && T && (T === "plain" || T === "S256") && (x.code_challenge = re, x.code_challenge_method = T), e(S) && e(L) !== e(ye)) {
      if (!e(E)) {
        r(h, o(l.authorize.passwordRequired));
        return;
      }
      if (e(E).length > 256) {
        r(h, "max 256");
        return;
      }
      x.password = e(E);
    }
    r(K, true);
    let P = "/auth/v1/oidc/authorize";
    ar().get() && (P = "/auth/v1/dev/authorize");
    let j = await de(P, x, "json", "noRedirect");
    await Ae(j);
  }
  async function Ae(t) {
    if (t.status === 202) {
      let d = t.headers.get("location");
      if (!d) {
        console.error("location header missing");
        return;
      }
      window.location.replace(d);
    } else if (t.status === 200) {
      r(h, "");
      let d = t.body;
      d && "user_id" in d && "code" in d ? (r($e, o(d.user_id)), r(ue, o({ Login: d.code }))) : console.error("did not receive a proper WebauthnLoginResponse after HTTP200");
    } else if (t.status === 406) r(h, o(l.authorize.clientForceMfa)), r(D, true);
    else if (t.status === 429) {
      let d = t.headers.get("x-retry-not-before");
      if (!d) {
        console.error("x-retry-not-before header missing");
        return;
      }
      let x = Number.parseInt(d), P = Vr(x), j = x * 1e3 - (/* @__PURE__ */ new Date()).getTime();
      r(B, true), r(h, `${l.authorize.http429} ${P}`), r(L, ""), r(E, ""), r(S, false), setTimeout(() => {
        r(B, false), r(h, "");
      }, j);
    } else e(S) ? (r(h, o(l.authorize.invalidCredentials)), r(he, true)) : (r(S, true), r(Ce, o(e(L))));
    r(K, false);
  }
  function lr() {
    e(S) && e(Ce) !== e(L) && (r(S, false), r(E, ""), r(h, ""));
  }
  function dr(t) {
    Gr(64, (d, { challenge: x, verifier: P }) => {
      d || (localStorage.setItem(Kr, P), vr(t, x));
    });
  }
  async function vr(t, d) {
    var _a2;
    if (!w) {
      console.error("clientId is undefined");
      return;
    }
    if (!G) {
      console.error("redirectUri is undefined");
      return;
    }
    let x = { email: e(L) || void 0, client_id: w, redirect_uri: G, scopes: xe, state: Le, nonce: be, code_challenge: re, code_challenge_method: T, provider_id: t, pkce_challenge: d }, P = await de("/auth/v1/providers/login", x);
    if (P.text) {
      Jr(P.text);
      let j = P.headers.get("location");
      if (!j) {
        console.error("no location header set for provider login");
        return;
      }
      window.location.href = j;
    } else r(h, o(((_a2 = P.error) == null ? void 0 : _a2.message) || "Error"));
  }
  function ur(t) {
    r(ue, void 0), r(h, o(t));
  }
  function cr(t) {
    t && "loc" in t && window.location.replace(t.loc);
  }
  async function fr() {
    let t = { email: e(L) };
    e(b) && (t.redirect_uri = encodeURI(e(b))), r(K, true);
    let d = await de("/auth/v1/users/request_reset", t);
    d.error ? r(h, o(d.error.message)) : r(pe, true), r(K, false);
  }
  var Fe = Rt();
  qr((t) => {
    g(() => Er.title = `Login ${(e(I) || w) ?? ""}`);
  });
  var Ne = O(Fe);
  Z(Ne, { id: Nr, get value() {
    return e(ve);
  }, set value(t) {
    r(ve, o(t));
  } });
  var We = v(Ne, 2);
  Z(We, { id: Wr, get value() {
    return e(I);
  }, set value(t) {
    r(I, o(t));
  } });
  var De = v(We, 2);
  Z(De, { id: Dr, get value() {
    return e(b);
  }, set value(t) {
    r(b, o(t));
  } });
  var Be = v(De, 2);
  Z(Be, { id: Br, get value() {
    return e(ce);
  }, set value(t) {
    r(ce, o(t));
  } });
  var He = v(Be, 2);
  Z(He, { id: Hr, get value() {
    return e(te);
  }, set value(t) {
    r(te, o(t));
  } });
  var Oe = v(He, 2);
  Z(Oe, { id: Or, get value() {
    return e(Re);
  }, set value(t) {
    r(Re, o(t));
  } });
  var hr = v(Oe, 2);
  Zr(hr, { children: (t, d) => {
    et(t, { children: (x, P) => {
      var j = yt(), Te = O(j), je = u(Te), Ve = u(je);
      {
        var pr = (a) => {
          dt(a, { clientId: w });
        };
        _(Ve, (a) => {
          w && a(pr);
        });
      }
      var _r = v(Ve, 2);
      {
        var mr = (a) => {
          var i = ct(), m = u(i);
          ut(m, { color: "hsla(var(--text) / .4)" }), c(i), g(() => V(i, "href", e(b))), s(a, i);
        };
        _(_r, (a) => {
          e(b) && a(mr);
        });
      }
      c(je);
      var ke = v(je, 2), Ge = u(ke), gr = u(Ge, true);
      c(Ge), c(ke);
      var Ke = v(ke, 2);
      {
        var wr = (a) => {
          Qr(a, { get userId() {
            return e($e);
          }, get purpose() {
            return e(ue);
          }, onSuccess: cr, onError: ur });
        };
        _(Ke, (a) => {
          e(ue) && e($e) && a(wr);
        });
      }
      var Je = v(Ke, 2);
      {
        var br = (a) => {
          var i = bt(), m = O(i);
          ot(m, { get action() {
            return e($);
          }, onSubmit: nr, children: (k, oe) => {
            var U = mt(), z = O(U);
            let Q;
            var C = u(z);
            const R = le(() => e(B) || e(D));
            Xr(C, { typ: "email", name: "email", autocomplete: "email", get label() {
              return l.common.email;
            }, get placeholder() {
              return l.common.email;
            }, get errMsg() {
              return l.authorize.validEmail;
            }, get disabled() {
              return e(R);
            }, onInput: lr, width: N, required: true, get ref() {
              return e(Pe);
            }, set ref(y) {
              r(Pe, o(y));
            }, get value() {
              return e(L);
            }, set value(y) {
              r(L, o(y));
            } }), c(z);
            var X = v(z, 2);
            {
              var $r = (y) => {
                var se = ht(), _e = O(se);
                const ze = le(() => e(B) || e(D));
                at(_e, { name: "password", autocomplete: "current-password", get label() {
                  return l.common.password;
                }, get placeholder() {
                  return l.common.password;
                }, maxLength: 256, get disabled() {
                  return e(ze);
                }, width: N, required: true, get ref() {
                  return e(Ie);
                }, set ref(p) {
                  r(Ie, o(p));
                }, get value() {
                  return e(E);
                }, set value(p) {
                  r(E, o(p));
                } });
                var Me = v(_e, 2);
                {
                  var q = (p) => {
                    var H = ft(), ie = u(H);
                    we(ie, { invisible: true, onclick: sr, children: (Se, Y) => {
                      ge();
                      var er = me();
                      g(() => M(er, l.authorize.passwordForgotten)), s(Se, er);
                    }, $$slots: { default: true } }), c(H), s(p, H);
                  };
                  _(Me, (p) => {
                    e(he) && !e(B) && p(q);
                  });
                }
                s(y, se);
              };
              _(X, (y) => {
                e(qe) && y($r);
              });
            }
            var Tr = v(X, 2);
            {
              var jr = (y) => {
                var se = rr(), _e = O(se);
                {
                  var ze = (q) => {
                    var p = pt(), H = u(p);
                    we(H, { onclick: fr, children: (ie, Se) => {
                      ge();
                      var Y = me();
                      g(() => M(Y, l.authorize.passwordRequest)), s(ie, Y);
                    }, $$slots: { default: true } }), c(p), s(q, p);
                  }, Me = (q) => {
                    var p = _t(), H = u(p);
                    we(H, { type: "submit", get isLoading() {
                      return e(K);
                    }, children: (ie, Se) => {
                      ge();
                      var Y = me();
                      g(() => M(Y, l.authorize.login)), s(ie, Y);
                    }, $$slots: { default: true } }), c(p), s(q, p);
                  };
                  _(_e, (q) => {
                    e(fe) ? q(ze) : q(Me, false);
                  });
                }
                s(y, se);
              };
              _(Tr, (y) => {
                !e(B) && !e(D) && y(jr);
              });
            }
            g(() => Q = Fr(z, 1, "svelte-1pnhhpj", null, Q, { emailMinHeight: !e(qe) })), s(k, U);
          }, $$slots: { default: true } });
          var J = v(m, 2);
          {
            var ae = (k) => {
              var oe = rr(), U = O(oe);
              {
                var z = (C) => {
                  var R = gt(), X = u(R, true);
                  c(R), g(() => {
                    V(R, "href", `/auth/v1/users/register?redirect_uri=${e(b) ?? ""}`), M(X, l.authorize.signUp);
                  }), s(C, R);
                }, Q = (C) => {
                  var R = wt(), X = u(R, true);
                  c(R), g(() => M(X, l.authorize.signUp)), s(C, R);
                };
                _(U, (C) => {
                  e(b) ? C(z) : C(Q, false);
                });
              }
              s(k, oe);
            };
            _(J, (k) => {
              e(Re) && !e(he) && !e(B) && k(ae);
            });
          }
          s(a, i);
        };
        _(Je, (a) => {
          e(D) || a(br);
        });
      }
      var Qe = v(Je, 2);
      {
        var xr = (a) => {
          var i = xt(), m = u(i, true);
          c(i), g(() => M(m, e(h))), s(a, i);
        };
        _(Qe, (a) => {
          e(h) && a(xr);
        });
      }
      var Xe = v(Qe, 2);
      {
        var Pr = (a) => {
          var i = Pt(), m = u(i, true);
          c(i), g(() => M(m, l.authorize.emailSentMsg)), s(a, i);
        };
        _(Xe, (a) => {
          e(pe) && a(Pr);
        });
      }
      var Ye = v(Xe, 2);
      {
        var Ir = (a) => {
          var i = It(), m = u(i);
          we(m, { onclick: () => window.location.href = "/auth/v1/account", children: (J, ae) => {
            ge();
            var k = me("Account");
            s(J, k);
          }, $$slots: { default: true } }), c(i), s(a, i);
        };
        _(Ye, (a) => {
          e(D) && a(Ir);
        });
      }
      var Lr = v(Ye, 2);
      {
        var yr = (a) => {
          var i = Lt(), m = u(i), J = v(u(m), 2), ae = u(J), k = u(ae, true);
          c(ae), c(J), c(m);
          var oe = v(m, 2);
          Ar(oe, 17, () => e(ve), (U) => U.id, (U, z) => {
            const Q = le(() => `Login: ${e(z).name}`);
            st(U, { get ariaLabel() {
              return e(Q);
            }, get provider() {
              return e(z);
            }, onclick: dr });
          }), c(i), g(() => M(k, l.authorize.orLoginWith)), s(a, i);
        };
        _(Lr, (a) => {
          !e(D) && e(ve).length > 0 && a(yr);
        });
      }
      c(Te);
      var Ze = v(Te, 2);
      tt(Ze, { absolute: true });
      var Rr = v(Ze, 2);
      Yr(Rr, { absolute: true }), g(() => M(gr, e(I) || w)), s(x, j);
    } });
  } }), s(ee, Fe), Sr();
}
export {
  Zt as component
};
