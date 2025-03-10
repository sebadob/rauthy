import { t as f, a as n, n as Mt, d as at, e as pe } from "../chunks/BH6NCLk-.js";
import { c, j as e, k as i, r as u, t as p, a9 as V, l as t, p as St, a5 as Ut, aa as de, f as j, a as Ct, s as l, a6 as qt, a7 as xe } from "../chunks/CvlvO1XB.js";
import { e as At, b as Dt, h as Nt, s as U } from "../chunks/CTI4QPiR.js";
import { i as m } from "../chunks/BUO_AUgz.js";
import { e as Ft } from "../chunks/BpWRzPRQ.js";
import { s as K, a as Wt } from "../chunks/BMbqVy6X.js";
import { p as s } from "../chunks/Wh68IIk2.js";
import { F as Ot, G as Bt, H as Ht, I as jt, q as Gt, J as Vt, j as Kt, e as ot, l as Jt, K as Qt, P as Xt, M as Yt } from "../chunks/Brp0G0eV.js";
import { B as we } from "../chunks/DMkkW5Nn.js";
import { W as Zt } from "../chunks/DEx36zkG.js";
import { I as er } from "../chunks/CJP-ccgO.js";
import { L as tr } from "../chunks/BCG8E-Sv.js";
import { p as Ce } from "../chunks/C6SR4G2t.js";
import { M as rr } from "../chunks/BbRujPvb.js";
import { C as ar } from "../chunks/GWgockW8.js";
import { u as or } from "../chunks/CUqQZdNU.js";
import { T as G } from "../chunks/Bb2Y1h4J.js";
import { u as F } from "../chunks/BNumlRaG.js";
import { T as sr } from "../chunks/Cbuxv9a6.js";
import { I as ir } from "../chunks/B-xX0s4n.js";
import { b as ve } from "../chunks/CRjU5SuJ.js";
import { u as st } from "../chunks/BMFqJ6Jy.js";
import { F as nr } from "../chunks/CW8abPUe.js";
import { B as lr } from "../chunks/DQDimnsq.js";
import { R as dr } from "../chunks/BfJ5aiUK.js";
var vr = f('<img alt="Client Logo" width="100%" height="100%">'), cr = f('<div class="logo svelte-1gwy16z"><!></div>');
function ur(re, L) {
  let W = i(false), v = V(() => L.updated ? `/auth/v1/clients/${L.clientId}/logo?updated=${L.updated}` : `/auth/v1/clients/${L.clientId}/logo`);
  var $ = cr(), k = c($);
  {
    var x = (P) => {
      dr(P, { width: "100%" });
    }, J = (P) => {
      var w = vr();
      p(() => K(w, "src", e(v))), At("error", w, () => t(W, true)), Dt(w), n(P, w);
    };
    m(k, (P) => {
      e(W) || L.updated === void 0 ? P(x) : P(J, false);
    });
  }
  u($), n(re, $);
}
var fr = Mt(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125
            1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504
            1.125-1.125V9.75M8.25 21h8.25"></path></svg>`);
function hr(re, L) {
  let W = Ce(L, "opacity", 3, 0.9), v = Ce(L, "width", 3, "1.5rem"), $ = Ce(L, "color", 3, "currentColor");
  var k = fr();
  K(k, "stroke-width", 2), p(() => {
    K(k, "stroke", $()), K(k, "width", v()), K(k, "opacity", W());
  }), n(re, k);
}
var _r = f('<a class="home svelte-1hxn0yc"><!></a>'), mr = f('<div class="forgotten svelte-1hxn0yc"><!></div>'), gr = f("<!> <!>", 1), pr = f('<div class="btn flex-col svelte-1hxn0yc"><!></div>'), xr = f('<div class="btn flex-col svelte-1hxn0yc"><!></div>'), wr = f("<div><!></div> <!> <!>", 1), yr = f('<a class="reg svelte-1hxn0yc" target="_blank"> </a>'), br = f('<a class="reg svelte-1hxn0yc" href="/auth/v1/users/register" target="_blank"> </a>'), Lr = f("<!> <!>", 1), Pr = f('<div class="errMsg svelte-1hxn0yc"> </div>'), Ir = f('<div class="success svelte-1hxn0yc"> </div>'), Tr = f('<div class="btn flex-col svelte-1hxn0yc"><!></div>'), Rr = f('<div class="providers flex-col svelte-1hxn0yc"><div class="providersSeparator svelte-1hxn0yc"><div class="separator svelte-1hxn0yc"></div> <div class="loginWith svelte-1hxn0yc"><div class="svelte-1hxn0yc"> </div></div></div> <!></div>'), $r = f('<div class="container svelte-1hxn0yc"><div class="head svelte-1hxn0yc"><!> <!></div> <div class="name svelte-1hxn0yc"><h2> </h2></div> <!> <!> <!> <!> <!> <!></div> <!> <!>', 1), kr = f("<!> <!> <!> <!> <!> <!> <!> <!>", 1);
function ra(re, L) {
  var _a;
  St(L, true);
  const W = "18rem";
  let v = or(), $ = st().get(), k = V(() => $ ? "/auth/v1/dev/authorize" : "/auth/v1/oidc/authorize"), x = F("client_id").get(), J = i(""), P = i(-1), w = i(s($ ? "/auth/v1" : "")), Q = F("redirect_uri").get(), ye = F("nonce").get(), be = ((_a = F("scope").get()) == null ? void 0 : _a.split(" ")) || [], Le = i(void 0), Pe = i(void 0), Ie = F("state").get(), ae = F("code_challenge").get(), z = F("code_challenge_method").get(), qe = false, Te = i(void 0), ce = i(s([])), ue = i(void 0), X = i(false), h = i(""), oe = i(""), fe = i(""), C = i(false), O = i(false), he = i(false), _e = i(false), me = i(false), B = i(false), Ae = i(""), Re = i(false), I = i(s(F("login_hint").get() || "")), q = i(""), $e = i(""), De = V(() => e(C) && e(Te) !== e(I) && !e(he));
  Ut(() => {
    var _a2;
    e(C) || ((_a2 = e(Le)) == null ? void 0 : _a2.focus());
  }), de(() => {
    qe && (x == null ? void 0 : x.length) && lt();
  }), de(() => {
    e(me) && setTimeout(() => {
      t(me, false), t(he, false), t(_e, false);
    }, 3e3);
  }), de(() => {
    var _a2;
    (_a2 = e(Pe)) == null ? void 0 : _a2.focus();
  }), de(() => {
    var _a2;
    if (e(oe) === "Refresh") qe = true;
    else if ((_a2 = e(oe)) == null ? void 0 : _a2.startsWith("MfaLogin ")) {
      let r = e(oe).replace("MfaLogin ", "");
      t(I, s(r)), t(Te, s(r));
    }
  }), de(() => {
    $ ? it() : e(fe) && ot(e(fe));
  });
  async function it() {
    var _a2;
    let r = await ve("/auth/v1/oidc/session");
    ((_a2 = r.body) == null ? void 0 : _a2.csrf_token) ? ot(r.body.csrf_token) : console.error(r.error);
  }
  function nt() {
    t(h, ""), t(he, true), t(q, "");
  }
  async function lt() {
    if (!x) {
      console.error("clientId is undefined");
      return;
    }
    if (!Q) {
      console.error("redirectUri is undefined");
      return;
    }
    t(X, true);
    const r = { client_id: x, redirect_uri: Q, state: Ie, nonce: ye, scopes: be };
    ae && z && (z === "plain" || z === "S256") && (r.code_challenge = ae, r.code_challenge_method = z);
    let d = await ve("/auth/v1/oidc/authorize/refresh", r);
    await Ne(d);
  }
  async function dt(r, d) {
    if (t(h, ""), !x) {
      console.error("clientId is undefined");
      return;
    }
    if (!Q) {
      console.error("redirectUri is undefined");
      return;
    }
    const y = { email: e(I), client_id: x, redirect_uri: Q, state: Ie, nonce: ye, scopes: be };
    if (ae && z && (z === "plain" || z === "S256") && (y.code_challenge = ae, y.code_challenge_method = z), e(C) && e(I) !== e(Te)) {
      if (!e(q)) {
        t(h, s(v.authorize.passwordRequired));
        return;
      }
      if (e(q).length > 256) {
        t(h, "max 256");
        return;
      }
      y.password = e(q);
    }
    t(X, true);
    let b = "/auth/v1/oidc/authorize";
    st().get() && (b = "/auth/v1/dev/authorize");
    let E = await ve(b, y, "json", "noRedirect");
    await Ne(E);
  }
  async function Ne(r) {
    if (r.status === 202) {
      let d = r.headers.get("location");
      if (!d) {
        console.error("location header missing");
        return;
      }
      window.location.replace(d);
    } else if (r.status === 200) {
      t(h, "");
      let d = r.body;
      d && "user_id" in d && "code" in d ? (t($e, s(d.user_id)), t(ue, s({ Login: d.code }))) : console.error("did not receive a proper WebauthnLoginResponse after HTTP200");
    } else if (r.status === 406) t(h, s(v.authorize.clientForceMfa)), t(O, true);
    else if (r.status === 429) {
      let d = r.headers.get("x-retry-not-before");
      if (!d) {
        console.error("x-retry-not-before header missing");
        return;
      }
      let y = Number.parseInt(d), b = Jt(y), E = y * 1e3 - (/* @__PURE__ */ new Date()).getTime();
      t(B, true), t(h, `${v.authorize.http429} ${b}`), t(I, ""), t(q, ""), t(C, false), setTimeout(() => {
        t(B, false), t(h, "");
      }, E);
    } else e(C) ? (t(h, s(v.authorize.invalidCredentials)), t(_e, true)) : (t(C, true), t(Ae, s(e(I))));
    t(X, false);
  }
  function vt() {
    e(C) && e(Ae) !== e(I) && (t(C, false), t(q, ""), t(h, ""));
  }
  function ct(r) {
    Qt(64, (d, { challenge: y, verifier: b }) => {
      d || (localStorage.setItem(Xt, b), ut(r, y));
    });
  }
  async function ut(r, d) {
    var _a2;
    if (!x) {
      console.error("clientId is undefined");
      return;
    }
    if (!Q) {
      console.error("redirectUri is undefined");
      return;
    }
    let y = { email: e(I) || void 0, client_id: x, redirect_uri: Q, scopes: be, state: Ie, nonce: ye, code_challenge: ae, code_challenge_method: z, provider_id: r, pkce_challenge: d }, b = await ve("/auth/v1/providers/login", y);
    if (b.text) {
      Yt(b.text);
      let E = b.headers.get("location");
      if (!E) {
        console.error("no location header set for provider login");
        return;
      }
      window.location.href = E;
    } else t(h, s(((_a2 = b.error) == null ? void 0 : _a2.message) || "Error"));
  }
  function ft(r) {
    t(ue, void 0), t(h, s(r));
  }
  function ht(r) {
    r && "loc" in r && window.location.replace(r.loc);
  }
  async function _t() {
    let r = { email: e(I) };
    e(w) && (r.redirect_uri = encodeURI(e(w))), t(X, true);
    let d = await ve("/auth/v1/users/request_reset", r);
    d.error ? t(h, s(d.error.message)) : t(me, true), t(X, false);
  }
  var Fe = kr();
  Nt((r) => {
    p(() => qt.title = `Login ${(e(J) || x) ?? ""}`);
  });
  var We = j(Fe);
  G(We, { id: Ot, get value() {
    return e(ce);
  }, set value(r) {
    t(ce, s(r));
  } });
  var Oe = l(We, 2);
  G(Oe, { id: Bt, get value() {
    return e(J);
  }, set value(r) {
    t(J, s(r));
  } });
  var Be = l(Oe, 2);
  G(Be, { id: Ht, get value() {
    return e(w);
  }, set value(r) {
    t(w, s(r));
  } });
  var He = l(Be, 2);
  G(He, { id: jt, get value() {
    return e(P);
  }, set value(r) {
    t(P, s(r));
  } });
  var je = l(He, 2);
  G(je, { id: Gt, get value() {
    return e(fe);
  }, set value(r) {
    t(fe, s(r));
  } });
  var Ge = l(je, 2);
  G(Ge, { id: Vt, get value() {
    return e(oe);
  }, set value(r) {
    t(oe, s(r));
  } });
  var Ve = l(Ge, 2);
  G(Ve, { id: Kt, get value() {
    return e(Re);
  }, set value(r) {
    t(Re, s(r));
  } });
  var mt = l(Ve, 2);
  rr(mt, { children: (r, d) => {
    ar(r, { children: (y, b) => {
      var E = $r(), ke = j(E), ze = c(ke), Ke = c(ze);
      {
        var gt = (a) => {
          const o = V(() => e(P) > -1 ? e(P) : void 0);
          ur(a, { clientId: x, get updated() {
            return e(o);
          } });
        };
        m(Ke, (a) => {
          x && a(gt);
        });
      }
      var pt = l(Ke, 2);
      {
        var xt = (a) => {
          var o = _r(), g = c(o);
          hr(g, { color: "hsla(var(--text) / .4)" }), u(o), p(() => K(o, "href", e(w))), n(a, o);
        };
        m(pt, (a) => {
          e(w) && a(xt);
        });
      }
      u(ze);
      var Ee = l(ze, 2), Je = c(Ee), wt = c(Je, true);
      u(Je), u(Ee);
      var Qe = l(Ee, 2);
      {
        var yt = (a) => {
          Zt(a, { get userId() {
            return e($e);
          }, get purpose() {
            return e(ue);
          }, onSuccess: ht, onError: ft });
        };
        m(Qe, (a) => {
          e(ue) && e($e) && a(yt);
        });
      }
      var Xe = l(Qe, 2);
      {
        var bt = (a) => {
          var o = Lr(), g = j(o);
          nr(g, { get action() {
            return e(k);
          }, onSubmit: dt, children: (M, ie) => {
            var A = wr(), S = j(A);
            let Z;
            var D = c(S);
            const R = V(() => e(B) || e(O));
            er(D, { typ: "email", name: "email", autocomplete: "email", get label() {
              return v.common.email;
            }, get placeholder() {
              return v.common.email;
            }, get errMsg() {
              return v.authorize.validEmail;
            }, get disabled() {
              return e(R);
            }, onInput: vt, width: W, required: true, get ref() {
              return e(Le);
            }, set ref(T) {
              t(Le, s(T));
            }, get value() {
              return e(I);
            }, set value(T) {
              t(I, s(T));
            } }), u(S);
            var ee = l(S, 2);
            {
              var kt = (T) => {
                var ne = gr(), ge = j(ne);
                const Me = V(() => e(B) || e(O));
                ir(ge, { name: "password", autocomplete: "current-password", get label() {
                  return v.common.password;
                }, get placeholder() {
                  return v.common.password;
                }, maxLength: 256, get disabled() {
                  return e(Me);
                }, width: W, required: true, get ref() {
                  return e(Pe);
                }, set ref(_) {
                  t(Pe, s(_));
                }, get value() {
                  return e(q);
                }, set value(_) {
                  t(q, s(_));
                } });
                var Se = l(ge, 2);
                {
                  var N = (_) => {
                    var H = mr(), le = c(H);
                    we(le, { invisible: true, onclick: nt, children: (Ue, te) => {
                      xe();
                      var rt = pe();
                      p(() => U(rt, v.authorize.passwordForgotten)), n(Ue, rt);
                    }, $$slots: { default: true } }), u(H), n(_, H);
                  };
                  m(Se, (_) => {
                    e(_e) && !e(B) && _(N);
                  });
                }
                n(T, ne);
              };
              m(ee, (T) => {
                e(De) && T(kt);
              });
            }
            var zt = l(ee, 2);
            {
              var Et = (T) => {
                var ne = at(), ge = j(ne);
                {
                  var Me = (N) => {
                    var _ = pr(), H = c(_);
                    we(H, { onclick: _t, children: (le, Ue) => {
                      xe();
                      var te = pe();
                      p(() => U(te, v.authorize.passwordRequest)), n(le, te);
                    }, $$slots: { default: true } }), u(_), n(N, _);
                  }, Se = (N) => {
                    var _ = xr(), H = c(_);
                    we(H, { type: "submit", get isLoading() {
                      return e(X);
                    }, children: (le, Ue) => {
                      xe();
                      var te = pe();
                      p(() => U(te, v.authorize.login)), n(le, te);
                    }, $$slots: { default: true } }), u(_), n(N, _);
                  };
                  m(ge, (N) => {
                    e(he) ? N(Me) : N(Se, false);
                  });
                }
                n(T, ne);
              };
              m(zt, (T) => {
                !e(B) && !e(O) && T(Et);
              });
            }
            p(() => Z = Wt(S, 1, "svelte-1hxn0yc", null, Z, { emailMinHeight: !e(De) })), n(M, A);
          }, $$slots: { default: true } });
          var Y = l(g, 2);
          {
            var se = (M) => {
              var ie = at(), A = j(ie);
              {
                var S = (D) => {
                  var R = yr(), ee = c(R, true);
                  u(R), p(() => {
                    K(R, "href", `/auth/v1/users/register?redirect_uri=${e(w) ?? ""}`), U(ee, v.authorize.signUp);
                  }), n(D, R);
                }, Z = (D) => {
                  var R = br(), ee = c(R, true);
                  u(R), p(() => U(ee, v.authorize.signUp)), n(D, R);
                };
                m(A, (D) => {
                  e(w) ? D(S) : D(Z, false);
                });
              }
              n(M, ie);
            };
            m(Y, (M) => {
              e(Re) && !e(_e) && !e(B) && M(se);
            });
          }
          n(a, o);
        };
        m(Xe, (a) => {
          e(O) || a(bt);
        });
      }
      var Ye = l(Xe, 2);
      {
        var Lt = (a) => {
          var o = Pr(), g = c(o, true);
          u(o), p(() => U(g, e(h))), n(a, o);
        };
        m(Ye, (a) => {
          e(h) && a(Lt);
        });
      }
      var Ze = l(Ye, 2);
      {
        var Pt = (a) => {
          var o = Ir(), g = c(o, true);
          u(o), p(() => U(g, v.authorize.emailSentMsg)), n(a, o);
        };
        m(Ze, (a) => {
          e(me) && a(Pt);
        });
      }
      var et = l(Ze, 2);
      {
        var It = (a) => {
          var o = Tr(), g = c(o);
          we(g, { onclick: () => window.location.href = "/auth/v1/account", children: (Y, se) => {
            xe();
            var M = pe("Account");
            n(Y, M);
          }, $$slots: { default: true } }), u(o), n(a, o);
        };
        m(et, (a) => {
          e(O) && a(It);
        });
      }
      var Tt = l(et, 2);
      {
        var Rt = (a) => {
          var o = Rr(), g = c(o), Y = l(c(g), 2), se = c(Y), M = c(se, true);
          u(se), u(Y), u(g);
          var ie = l(g, 2);
          Ft(ie, 17, () => e(ce), (A) => A.id, (A, S) => {
            const Z = V(() => `Login: ${e(S).name}`);
            lr(A, { get ariaLabel() {
              return e(Z);
            }, get provider() {
              return e(S);
            }, onclick: ct });
          }), u(o), p(() => U(M, v.authorize.orLoginWith)), n(a, o);
        };
        m(Tt, (a) => {
          !e(O) && e(ce).length > 0 && a(Rt);
        });
      }
      u(ke);
      var tt = l(ke, 2);
      sr(tt, { absolute: true });
      var $t = l(tt, 2);
      tr($t, { absolute: true }), p(() => U(wt, e(J) || x)), n(y, E);
    } });
  } }), n(re, Fe), Ct();
}
export {
  ra as component
};
