import { n as yr, a as i, t as f, d as er, e as me } from "../chunks/BH6NCLk-.js";
import { t as w, c as u, r as c, p as Tr, k as l, a5 as Rr, aa as ie, f as B, a as kr, l as r, j as e, s as d, a6 as Sr, a9 as ge, a7 as pe } from "../chunks/CvlvO1XB.js";
import { h as zr, s as R } from "../chunks/CTI4QPiR.js";
import { i as g } from "../chunks/BUO_AUgz.js";
import { e as Mr } from "../chunks/BpWRzPRQ.js";
import { s as H } from "../chunks/BMbqVy6X.js";
import { p as o } from "../chunks/Wh68IIk2.js";
import { I as Er, J as Ur, K as Cr, q as Nr, M as Ar, j as Wr, e as rr, l as Fr, N as Dr, P as Or, O as Br } from "../chunks/CGXT-546.js";
import { B as we } from "../chunks/DMkkW5Nn.js";
import { W as Hr } from "../chunks/CY7Th9O-.js";
import { I as Vr } from "../chunks/DmeAqnkr.js";
import { L as jr } from "../chunks/KFFqpuZ0.js";
import { p as Ee } from "../chunks/C6SR4G2t.js";
import { M as Kr } from "../chunks/Ds6bXi0i.js";
import { C as Gr } from "../chunks/GWgockW8.js";
import { u as Jr } from "../chunks/DOl8_ubJ.js";
import { T as J } from "../chunks/DSw3WKCO.js";
import { u as C } from "../chunks/D45liQ4S.js";
import { T as Qr } from "../chunks/DNdifVes.js";
import { I as Xr } from "../chunks/CLUUhKug.js";
import { b as ne } from "../chunks/bbkAiDd0.js";
import { u as tr } from "../chunks/BMFqJ6Jy.js";
import { F as Yr } from "../chunks/CqS1e6KT.js";
import { B as Zr } from "../chunks/BV-QGn6o.js";
var et = yr(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125
            1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504
            1.125-1.125V9.75M8.25 21h8.25"></path></svg>`);
function rt(Q, N) {
  let k = Ee(N, "opacity", 3, 0.9), v = Ee(N, "width", 3, "1.5rem"), X = Ee(N, "color", 3, "currentColor");
  var A = et();
  H(A, "stroke-width", 2), w(() => {
    H(A, "stroke", X()), H(A, "width", v()), H(A, "opacity", k());
  }), i(Q, A);
}
var tt = f('<div class="logo svelte-1gwy16z"><img alt="No Logo Available"></div>');
function at(Q, N) {
  console.log("rendering clientLogo");
  var k = tt(), v = u(k);
  c(k), w(() => H(v, "src", `/auth/v1/clients/${N.clientId}/logo`)), i(Q, k);
}
var ot = f('<a class="home svelte-1hqs85f"><!></a>'), st = f('<div class="forgotten svelte-1hqs85f"><!></div>'), it = f("<!> <!>", 1), nt = f('<div class="btn flex-col svelte-1hqs85f"><!></div>'), lt = f('<div class="btn flex-col svelte-1hqs85f"><!></div>'), dt = f("<!> <!> <!>", 1), vt = f('<a class="reg svelte-1hqs85f" target="_blank"> </a>'), ut = f('<a class="reg svelte-1hqs85f" href="/auth/v1/users/register" target="_blank"> </a>'), ct = f("<!> <!>", 1), ft = f('<div class="errMsg svelte-1hqs85f"> </div>'), ht = f('<div class="success svelte-1hqs85f"> </div>'), _t = f('<div class="btn flex-col svelte-1hqs85f"><!></div>'), mt = f('<div class="providers flex-col svelte-1hqs85f"><div class="providersSeparator svelte-1hqs85f"><div class="separator svelte-1hqs85f"></div> <div class="loginWith svelte-1hqs85f"><div class="svelte-1hqs85f"> </div></div></div> <!></div>'), gt = f('<div class="container svelte-1hqs85f"><div class="head svelte-1hqs85f"><!> <!></div> <div class="name svelte-1hqs85f"><h2> </h2></div> <!> <!> <!> <!> <!> <!></div> <!> <!>', 1), pt = f("<!> <!> <!> <!> <!> <!> <!>", 1);
function Bt(Q, N) {
  var _a;
  Tr(N, true);
  const k = "18rem";
  let v = Jr(), X = tr().get(), A = ge(() => X ? "/auth/v1/dev/authorize" : "/auth/v1/oidc/authorize"), x = C("client_id").get(), le = l(""), S = l(o(X ? "/auth/v1" : "")), V = C("redirect_uri").get(), be = C("nonce").get(), xe = ((_a = C("scope").get()) == null ? void 0 : _a.split(" ")) || [], Pe = l(void 0), Le = l(void 0), Ie = C("state").get(), Y = C("code_challenge").get(), $ = C("code_challenge_method").get(), Ue = false, $e = l(void 0), de = l(o([])), ve = l(void 0), j = l(false), h = l(""), Z = l(""), ue = l(""), z = l(false), W = l(false), ce = l(false), fe = l(false), he = l(false), F = l(false), Ce = l(""), qe = l(false), P = l(o(C("login_hint").get() || "")), M = l(""), ye = l("");
  Rr(() => {
    var _a2;
    e(z) || ((_a2 = e(Pe)) == null ? void 0 : _a2.focus());
  }), ie(() => {
    Ue && (x == null ? void 0 : x.length) && sr();
  }), ie(() => {
    e(he) && setTimeout(() => {
      r(he, false), r(ce, false), r(fe, false);
    }, 3e3);
  }), ie(() => {
    var _a2;
    (_a2 = e(Le)) == null ? void 0 : _a2.focus();
  }), ie(() => {
    var _a2;
    if (e(Z) === "Refresh") Ue = true;
    else if ((_a2 = e(Z)) == null ? void 0 : _a2.startsWith("MfaLogin ")) {
      let t = e(Z).replace("MfaLogin ", "");
      r(P, o(t)), r($e, o(t));
    }
  }), ie(() => {
    X ? ar() : e(ue) && rr(e(ue));
  });
  async function ar() {
    var _a2;
    let t = await ne("/auth/v1/oidc/session");
    console.log("manual session", t.body), ((_a2 = t.body) == null ? void 0 : _a2.csrf_token) ? rr(t.body.csrf_token) : console.error(t.error);
  }
  function or() {
    r(h, ""), r(ce, true), r(M, "");
  }
  async function sr() {
    if (!x) {
      console.error("clientId is undefined");
      return;
    }
    if (!V) {
      console.error("redirectUri is undefined");
      return;
    }
    r(j, true);
    const t = { client_id: x, redirect_uri: V, state: Ie, nonce: be, scopes: xe };
    Y && $ && ($ === "plain" || $ === "S256") && (t.code_challenge = Y, t.code_challenge_method = $);
    let n = await ne("/auth/v1/oidc/authorize/refresh", t);
    await Ne(n);
  }
  async function ir(t, n) {
    if (r(h, ""), !x) {
      console.error("clientId is undefined");
      return;
    }
    if (!V) {
      console.error("redirectUri is undefined");
      return;
    }
    const p = { email: e(P), client_id: x, redirect_uri: V, state: Ie, nonce: be, scopes: xe };
    if (Y && $ && ($ === "plain" || $ === "S256") && (p.code_challenge = Y, p.code_challenge_method = $), e(z) && e(P) !== e($e)) {
      if (!e(M)) {
        r(h, o(v.authorize.passwordRequired));
        return;
      }
      if (e(M).length > 256) {
        r(h, "max 256");
        return;
      }
      p.password = e(M);
    }
    r(j, true);
    let b = "/auth/v1/oidc/authorize";
    tr().get() && (b = "/auth/v1/dev/authorize");
    let q = await ne(b, p);
    await Ne(q);
  }
  async function Ne(t) {
    if (t.status === 202) {
      let n = t.headers.get("location");
      if (!n) {
        console.error("location header missing");
        return;
      }
      window.location.replace(n);
    } else if (t.status === 200) {
      r(h, "");
      let n = t.body;
      n && "user_id" in n && "code" in n ? (r(ye, o(n.user_id)), r(ve, o({ Login: n.code }))) : console.error("did not receive a proper WebauthnLoginResponse after HTTP200");
    } else if (t.status === 406) r(h, o(v.authorize.clientForceMfa)), r(W, true);
    else if (t.status === 429) {
      let n = t.headers.get("x-retry-not-before");
      if (!n) {
        console.error("x-retry-not-before header missing");
        return;
      }
      let p = Number.parseInt(n), b = Fr(p), q = p * 1e3 - (/* @__PURE__ */ new Date()).getTime();
      r(F, true), r(h, `${v.authorize.http429} ${b}`), r(P, ""), r(M, ""), r(z, false), setTimeout(() => {
        r(F, false), r(h, "");
      }, q);
    } else e(z) ? (r(h, o(v.authorize.invalidCredentials)), r(fe, true)) : (r(z, true), r(Ce, o(e(P))));
    r(j, false);
  }
  function nr() {
    e(z) && e(Ce) !== e(P) && (r(z, false), r(M, ""), r(h, ""));
  }
  function lr(t) {
    Dr(64, (n, { challenge: p, verifier: b }) => {
      n || (localStorage.setItem(Or, b), dr(t, p));
    });
  }
  async function dr(t, n) {
    var _a2;
    if (!x) {
      console.error("clientId is undefined");
      return;
    }
    if (!V) {
      console.error("redirectUri is undefined");
      return;
    }
    let p = { email: e(P) || void 0, client_id: x, redirect_uri: V, scopes: xe, state: Ie, nonce: be, code_challenge: Y, code_challenge_method: $, provider_id: t, pkce_challenge: n }, b = await ne("/auth/v1/providers/login", p);
    if (b.text) {
      Br(b.text);
      let q = b.headers.get("location");
      if (!q) {
        console.error("no location header set for provider login");
        return;
      }
      window.location.href = q;
    } else r(h, o(((_a2 = b.error) == null ? void 0 : _a2.message) || "Error"));
  }
  function vr(t) {
    r(ve, void 0), r(h, o(t));
  }
  function ur(t) {
    console.log("onWebauthnSuccess", t), t && "loc" in t && window.location.replace(t.loc);
  }
  async function cr() {
    let t = { email: e(P) };
    e(S) && (t.redirect_uri = encodeURI(e(S))), r(j, true);
    let n = await ne("/auth/v1/users/request_reset", t);
    n.error ? r(h, o(n.error.message)) : r(he, true), r(j, false);
  }
  var Ae = pt();
  zr((t) => {
    w(() => Sr.title = `Login ${(e(le) || x) ?? ""}`);
  });
  var We = B(Ae);
  J(We, { id: Er, get value() {
    return e(de);
  }, set value(t) {
    r(de, o(t));
  } });
  var Fe = d(We, 2);
  J(Fe, { id: Ur, get value() {
    return e(le);
  }, set value(t) {
    r(le, o(t));
  } });
  var De = d(Fe, 2);
  J(De, { id: Cr, get value() {
    return e(S);
  }, set value(t) {
    r(S, o(t));
  } });
  var Oe = d(De, 2);
  J(Oe, { id: Nr, get value() {
    return e(ue);
  }, set value(t) {
    r(ue, o(t));
  } });
  var Be = d(Oe, 2);
  J(Be, { id: Ar, get value() {
    return e(Z);
  }, set value(t) {
    r(Z, o(t));
  } });
  var He = d(Be, 2);
  J(He, { id: Wr, get value() {
    return e(qe);
  }, set value(t) {
    r(qe, o(t));
  } });
  var fr = d(He, 2);
  Kr(fr, { children: (t, n) => {
    Gr(t, { children: (p, b) => {
      var q = gt(), Te = B(q), Re = u(Te), Ve = u(Re);
      {
        var hr = (a) => {
          at(a, { clientId: x });
        };
        g(Ve, (a) => {
          x && a(hr);
        });
      }
      var _r = d(Ve, 2);
      {
        var mr = (a) => {
          var s = ot(), m = u(s);
          rt(m, { color: "hsla(var(--text) / .4)" }), c(s), w(() => H(s, "href", e(S))), i(a, s);
        };
        g(_r, (a) => {
          e(S) && a(mr);
        });
      }
      c(Re);
      var ke = d(Re, 2), je = u(ke), gr = u(je, true);
      c(je), c(ke);
      var Ke = d(ke, 2);
      {
        var pr = (a) => {
          Hr(a, { get userId() {
            return e(ye);
          }, get purpose() {
            return e(ve);
          }, onSuccess: ur, onError: vr });
        };
        g(Ke, (a) => {
          e(ve) && e(ye) && a(pr);
        });
      }
      var Ge = d(Ke, 2);
      {
        var wr = (a) => {
          var s = ct(), m = B(s);
          Yr(m, { get action() {
            return e(A);
          }, onSubmit: ir, children: (y, re) => {
            var E = dt(), D = B(E);
            const te = ge(() => e(F) || e(W));
            Vr(D, { typ: "email", name: "email", autocomplete: "email", get label() {
              return v.common.email;
            }, get placeholder() {
              return v.common.email;
            }, get disabled() {
              return e(te);
            }, onInput: nr, width: k, required: true, get ref() {
              return e(Pe);
            }, set ref(L) {
              r(Pe, o(L));
            }, get value() {
              return e(P);
            }, set value(L) {
              r(P, o(L));
            } });
            var T = d(D, 2);
            {
              var I = (L) => {
                var oe = it(), _e = B(oe);
                const Se = ge(() => e(F) || e(W));
                Xr(_e, { name: "password", autocomplete: "current-password", get label() {
                  return v.common.password;
                }, get placeholder() {
                  return v.common.password;
                }, maxLength: 256, get disabled() {
                  return e(Se);
                }, width: k, required: true, get ref() {
                  return e(Le);
                }, set ref(_) {
                  r(Le, o(_));
                }, get value() {
                  return e(M);
                }, set value(_) {
                  r(M, o(_));
                } });
                var ze = d(_e, 2);
                {
                  var U = (_) => {
                    var O = st(), se = u(O);
                    we(se, { invisible: true, onclick: or, children: (Me, G) => {
                      pe();
                      var Ze = me();
                      w(() => R(Ze, v.authorize.passwordForgotten)), i(Me, Ze);
                    }, $$slots: { default: true } }), c(O), i(_, O);
                  };
                  g(ze, (_) => {
                    e(fe) && !e(F) && _(U);
                  });
                }
                i(L, oe);
              };
              g(T, (L) => {
                e(z) && e($e) !== e(P) && !e(ce) && L(I);
              });
            }
            var ae = d(T, 2);
            {
              var qr = (L) => {
                var oe = er(), _e = B(oe);
                {
                  var Se = (U) => {
                    var _ = nt(), O = u(_);
                    we(O, { onclick: cr, children: (se, Me) => {
                      pe();
                      var G = me();
                      w(() => R(G, v.authorize.passwordRequest)), i(se, G);
                    }, $$slots: { default: true } }), c(_), i(U, _);
                  }, ze = (U) => {
                    var _ = lt(), O = u(_);
                    we(O, { type: "submit", get isLoading() {
                      return e(j);
                    }, children: (se, Me) => {
                      pe();
                      var G = me();
                      w(() => R(G, v.authorize.login)), i(se, G);
                    }, $$slots: { default: true } }), c(_), i(U, _);
                  };
                  g(_e, (U) => {
                    e(ce) ? U(Se) : U(ze, false);
                  });
                }
                i(L, oe);
              };
              g(ae, (L) => {
                !e(F) && !e(W) && L(qr);
              });
            }
            i(y, E);
          }, $$slots: { default: true } });
          var K = d(m, 2);
          {
            var ee = (y) => {
              var re = er(), E = B(re);
              {
                var D = (T) => {
                  var I = vt(), ae = u(I, true);
                  c(I), w(() => {
                    H(I, "href", `/auth/v1/users/register?redirect_uri=${e(S) ?? ""}`), R(ae, v.authorize.signUp);
                  }), i(T, I);
                }, te = (T) => {
                  var I = ut(), ae = u(I, true);
                  c(I), w(() => R(ae, v.authorize.signUp)), i(T, I);
                };
                g(E, (T) => {
                  e(S) ? T(D) : T(te, false);
                });
              }
              i(y, re);
            };
            g(K, (y) => {
              e(qe) && !e(fe) && !e(F) && y(ee);
            });
          }
          i(a, s);
        };
        g(Ge, (a) => {
          e(W) || a(wr);
        });
      }
      var Je = d(Ge, 2);
      {
        var br = (a) => {
          var s = ft(), m = u(s, true);
          c(s), w(() => R(m, e(h))), i(a, s);
        };
        g(Je, (a) => {
          e(h) && a(br);
        });
      }
      var Qe = d(Je, 2);
      {
        var xr = (a) => {
          var s = ht(), m = u(s, true);
          c(s), w(() => R(m, v.authorize.emailSentMsg)), i(a, s);
        };
        g(Qe, (a) => {
          e(he) && a(xr);
        });
      }
      var Xe = d(Qe, 2);
      {
        var Pr = (a) => {
          var s = _t(), m = u(s);
          we(m, { onclick: () => window.location.href = "/auth/v1/account", children: (K, ee) => {
            pe();
            var y = me("Account");
            i(K, y);
          }, $$slots: { default: true } }), c(s), i(a, s);
        };
        g(Xe, (a) => {
          e(W) && a(Pr);
        });
      }
      var Lr = d(Xe, 2);
      {
        var Ir = (a) => {
          var s = mt(), m = u(s), K = d(u(m), 2), ee = u(K), y = u(ee, true);
          c(ee), c(K), c(m);
          var re = d(m, 2);
          Mr(re, 17, () => e(de), (E) => E.id, (E, D) => {
            const te = ge(() => `Login: ${e(D).name}`);
            Zr(E, { get ariaLabel() {
              return e(te);
            }, get provider() {
              return e(D);
            }, onclick: lr });
          }), c(s), w(() => R(y, v.authorize.orLoginWith)), i(a, s);
        };
        g(Lr, (a) => {
          !e(W) && e(de) && a(Ir);
        });
      }
      c(Te);
      var Ye = d(Te, 2);
      Qr(Ye, { absolute: true });
      var $r = d(Ye, 2);
      jr($r, { absolute: true }), w(() => R(gr, e(le) || x)), i(p, q);
    } });
  } }), i(Q, Ae), kr();
}
export {
  Bt as component
};
