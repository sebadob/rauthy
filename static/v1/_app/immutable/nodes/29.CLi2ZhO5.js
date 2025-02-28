import { n as Tr, a as s, t as f, d as er, e as me } from "../chunks/BH6NCLk-.js";
import { t as w, c as u, r as c, p as yr, k as l, a5 as Rr, aa as se, f as B, a as kr, l as r, j as e, s as v, a6 as Mr, a9 as ge, a7 as pe } from "../chunks/CvlvO1XB.js";
import { h as Sr, s as T } from "../chunks/CTI4QPiR.js";
import { i as g } from "../chunks/BUO_AUgz.js";
import { e as Er } from "../chunks/BpWRzPRQ.js";
import { s as j } from "../chunks/BMbqVy6X.js";
import { p as o } from "../chunks/Wh68IIk2.js";
import { I as Ur, J as Cr, K as Nr, q as Ar, M as Wr, j as Fr, e as rr, l as Dr, N as Or, P as Br, O as jr } from "../chunks/BaE7H8ny.js";
import { B as we } from "../chunks/DMkkW5Nn.js";
import { W as Hr } from "../chunks/8x1xiTUd.js";
import { I as Vr } from "../chunks/BtKnbFDH.js";
import { L as Kr } from "../chunks/DUtP9z1z.js";
import { p as Ee } from "../chunks/C6SR4G2t.js";
import { M as Gr } from "../chunks/Ds6bXi0i.js";
import { C as Jr } from "../chunks/GWgockW8.js";
import { u as Qr } from "../chunks/BQ1-pLIs.js";
import { T as J } from "../chunks/DcJo-CQ6.js";
import { u as N } from "../chunks/Beqa1JKV.js";
import { T as Xr } from "../chunks/CVHA2fPQ.js";
import { I as Yr } from "../chunks/D6E4lNPv.js";
import { b as ne } from "../chunks/CBGoQiUs.js";
import { u as tr } from "../chunks/BMFqJ6Jy.js";
import { F as Zr } from "../chunks/BS0DIDHc.js";
import { B as et } from "../chunks/BV-QGn6o.js";
var rt = Tr(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125
            1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504
            1.125-1.125V9.75M8.25 21h8.25"></path></svg>`);
function tt(Q, A) {
  let y = Ee(A, "opacity", 3, 0.9), d = Ee(A, "width", 3, "1.5rem"), X = Ee(A, "color", 3, "currentColor");
  var W = rt();
  j(W, "stroke-width", 2), w(() => {
    j(W, "stroke", X()), j(W, "width", d()), j(W, "opacity", y());
  }), s(Q, W);
}
var at = f('<div class="logo svelte-1gwy16z"><img alt="No Logo Available"></div>');
function ot(Q, A) {
  var y = at(), d = u(y);
  c(y), w(() => j(d, "src", `/auth/v1/clients/${A.clientId}/logo`)), s(Q, y);
}
var it = f('<a class="home svelte-5hzq8v"><!></a>'), st = f('<div class="forgotten svelte-5hzq8v"><!></div>'), nt = f("<!> <!>", 1), lt = f('<div class="btn flex-col svelte-5hzq8v"><!></div>'), vt = f('<div class="btn flex-col svelte-5hzq8v"><!></div>'), dt = f('<div class="email svelte-5hzq8v"><!></div> <!> <!>', 1), ut = f('<a class="reg svelte-5hzq8v" target="_blank"> </a>'), ct = f('<a class="reg svelte-5hzq8v" href="/auth/v1/users/register" target="_blank"> </a>'), ft = f("<!> <!>", 1), ht = f('<div class="errMsg svelte-5hzq8v"> </div>'), _t = f('<div class="success svelte-5hzq8v"> </div>'), mt = f('<div class="btn flex-col svelte-5hzq8v"><!></div>'), gt = f('<div class="providers flex-col svelte-5hzq8v"><div class="providersSeparator svelte-5hzq8v"><div class="separator svelte-5hzq8v"></div> <div class="loginWith svelte-5hzq8v"><div class="svelte-5hzq8v"> </div></div></div> <!></div>'), pt = f('<div class="container svelte-5hzq8v"><div class="head svelte-5hzq8v"><!> <!></div> <div class="name svelte-5hzq8v"><h2> </h2></div> <!> <!> <!> <!> <!> <!></div> <!> <!>', 1), wt = f("<!> <!> <!> <!> <!> <!> <!>", 1);
function jt(Q, A) {
  var _a;
  yr(A, true);
  const y = "18rem";
  let d = Qr(), X = tr().get(), W = ge(() => X ? "/auth/v1/dev/authorize" : "/auth/v1/oidc/authorize"), b = N("client_id").get(), le = l(""), R = l(o(X ? "/auth/v1" : "")), H = N("redirect_uri").get(), ze = N("nonce").get(), be = ((_a = N("scope").get()) == null ? void 0 : _a.split(" ")) || [], xe = l(void 0), Pe = l(void 0), qe = N("state").get(), Y = N("code_challenge").get(), I = N("code_challenge_method").get(), Ue = false, Ie = l(void 0), ve = l(o([])), de = l(void 0), V = l(false), h = l(""), Z = l(""), ue = l(""), k = l(false), F = l(false), ce = l(false), fe = l(false), he = l(false), D = l(false), Ce = l(""), Le = l(false), x = l(o(N("login_hint").get() || "")), M = l(""), $e = l("");
  Rr(() => {
    var _a2;
    e(k) || ((_a2 = e(xe)) == null ? void 0 : _a2.focus());
  }), se(() => {
    Ue && (b == null ? void 0 : b.length) && ir();
  }), se(() => {
    e(he) && setTimeout(() => {
      r(he, false), r(ce, false), r(fe, false);
    }, 3e3);
  }), se(() => {
    var _a2;
    (_a2 = e(Pe)) == null ? void 0 : _a2.focus();
  }), se(() => {
    var _a2;
    if (e(Z) === "Refresh") Ue = true;
    else if ((_a2 = e(Z)) == null ? void 0 : _a2.startsWith("MfaLogin ")) {
      let t = e(Z).replace("MfaLogin ", "");
      r(x, o(t)), r(Ie, o(t));
    }
  }), se(() => {
    X ? ar() : e(ue) && rr(e(ue));
  });
  async function ar() {
    var _a2;
    let t = await ne("/auth/v1/oidc/session");
    ((_a2 = t.body) == null ? void 0 : _a2.csrf_token) ? rr(t.body.csrf_token) : console.error(t.error);
  }
  function or() {
    r(h, ""), r(ce, true), r(M, "");
  }
  async function ir() {
    if (!b) {
      console.error("clientId is undefined");
      return;
    }
    if (!H) {
      console.error("redirectUri is undefined");
      return;
    }
    r(V, true);
    const t = { client_id: b, redirect_uri: H, state: qe, nonce: ze, scopes: be };
    Y && I && (I === "plain" || I === "S256") && (t.code_challenge = Y, t.code_challenge_method = I);
    let n = await ne("/auth/v1/oidc/authorize/refresh", t);
    await Ne(n);
  }
  async function sr(t, n) {
    if (r(h, ""), !b) {
      console.error("clientId is undefined");
      return;
    }
    if (!H) {
      console.error("redirectUri is undefined");
      return;
    }
    const p = { email: e(x), client_id: b, redirect_uri: H, state: qe, nonce: ze, scopes: be };
    if (Y && I && (I === "plain" || I === "S256") && (p.code_challenge = Y, p.code_challenge_method = I), e(k) && e(x) !== e(Ie)) {
      if (!e(M)) {
        r(h, o(d.authorize.passwordRequired));
        return;
      }
      if (e(M).length > 256) {
        r(h, "max 256");
        return;
      }
      p.password = e(M);
    }
    r(V, true);
    let z = "/auth/v1/oidc/authorize";
    tr().get() && (z = "/auth/v1/dev/authorize");
    let L = await ne(z, p, "json", "noRedirect");
    await Ne(L);
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
      n && "user_id" in n && "code" in n ? (r($e, o(n.user_id)), r(de, o({ Login: n.code }))) : console.error("did not receive a proper WebauthnLoginResponse after HTTP200");
    } else if (t.status === 406) r(h, o(d.authorize.clientForceMfa)), r(F, true);
    else if (t.status === 429) {
      let n = t.headers.get("x-retry-not-before");
      if (!n) {
        console.error("x-retry-not-before header missing");
        return;
      }
      let p = Number.parseInt(n), z = Dr(p), L = p * 1e3 - (/* @__PURE__ */ new Date()).getTime();
      r(D, true), r(h, `${d.authorize.http429} ${z}`), r(x, ""), r(M, ""), r(k, false), setTimeout(() => {
        r(D, false), r(h, "");
      }, L);
    } else e(k) ? (r(h, o(d.authorize.invalidCredentials)), r(fe, true)) : (r(k, true), r(Ce, o(e(x))));
    r(V, false);
  }
  function nr() {
    e(k) && e(Ce) !== e(x) && (r(k, false), r(M, ""), r(h, ""));
  }
  function lr(t) {
    Or(64, (n, { challenge: p, verifier: z }) => {
      n || (localStorage.setItem(Br, z), vr(t, p));
    });
  }
  async function vr(t, n) {
    var _a2;
    if (!b) {
      console.error("clientId is undefined");
      return;
    }
    if (!H) {
      console.error("redirectUri is undefined");
      return;
    }
    let p = { email: e(x) || void 0, client_id: b, redirect_uri: H, scopes: be, state: qe, nonce: ze, code_challenge: Y, code_challenge_method: I, provider_id: t, pkce_challenge: n }, z = await ne("/auth/v1/providers/login", p);
    if (z.text) {
      jr(z.text);
      let L = z.headers.get("location");
      if (!L) {
        console.error("no location header set for provider login");
        return;
      }
      window.location.href = L;
    } else r(h, o(((_a2 = z.error) == null ? void 0 : _a2.message) || "Error"));
  }
  function dr(t) {
    r(de, void 0), r(h, o(t));
  }
  function ur(t) {
    t && "loc" in t && window.location.replace(t.loc);
  }
  async function cr() {
    let t = { email: e(x) };
    e(R) && (t.redirect_uri = encodeURI(e(R))), r(V, true);
    let n = await ne("/auth/v1/users/request_reset", t);
    n.error ? r(h, o(n.error.message)) : r(he, true), r(V, false);
  }
  var Ae = wt();
  Sr((t) => {
    w(() => Mr.title = `Login ${(e(le) || b) ?? ""}`);
  });
  var We = B(Ae);
  J(We, { id: Ur, get value() {
    return e(ve);
  }, set value(t) {
    r(ve, o(t));
  } });
  var Fe = v(We, 2);
  J(Fe, { id: Cr, get value() {
    return e(le);
  }, set value(t) {
    r(le, o(t));
  } });
  var De = v(Fe, 2);
  J(De, { id: Nr, get value() {
    return e(R);
  }, set value(t) {
    r(R, o(t));
  } });
  var Oe = v(De, 2);
  J(Oe, { id: Ar, get value() {
    return e(ue);
  }, set value(t) {
    r(ue, o(t));
  } });
  var Be = v(Oe, 2);
  J(Be, { id: Wr, get value() {
    return e(Z);
  }, set value(t) {
    r(Z, o(t));
  } });
  var je = v(Be, 2);
  J(je, { id: Fr, get value() {
    return e(Le);
  }, set value(t) {
    r(Le, o(t));
  } });
  var fr = v(je, 2);
  Gr(fr, { children: (t, n) => {
    Jr(t, { children: (p, z) => {
      var L = pt(), Te = B(L), ye = u(Te), He = u(ye);
      {
        var hr = (a) => {
          ot(a, { clientId: b });
        };
        g(He, (a) => {
          b && a(hr);
        });
      }
      var _r = v(He, 2);
      {
        var mr = (a) => {
          var i = it(), m = u(i);
          tt(m, { color: "hsla(var(--text) / .4)" }), c(i), w(() => j(i, "href", e(R))), s(a, i);
        };
        g(_r, (a) => {
          e(R) && a(mr);
        });
      }
      c(ye);
      var Re = v(ye, 2), Ve = u(Re), gr = u(Ve, true);
      c(Ve), c(Re);
      var Ke = v(Re, 2);
      {
        var pr = (a) => {
          Hr(a, { get userId() {
            return e($e);
          }, get purpose() {
            return e(de);
          }, onSuccess: ur, onError: dr });
        };
        g(Ke, (a) => {
          e(de) && e($e) && a(pr);
        });
      }
      var Ge = v(Ke, 2);
      {
        var wr = (a) => {
          var i = ft(), m = B(i);
          Zr(m, { get action() {
            return e(W);
          }, onSubmit: sr, children: ($, re) => {
            var S = dt(), E = B(S), te = u(E);
            const U = ge(() => e(D) || e(F));
            Vr(te, { typ: "email", name: "email", autocomplete: "email", get label() {
              return d.common.email;
            }, get placeholder() {
              return d.common.email;
            }, get errMsg() {
              return d.authorize.validEmail;
            }, get disabled() {
              return e(U);
            }, onInput: nr, width: y, required: true, get ref() {
              return e(xe);
            }, set ref(q) {
              r(xe, o(q));
            }, get value() {
              return e(x);
            }, set value(q) {
              r(x, o(q));
            } }), c(E);
            var P = v(E, 2);
            {
              var ae = (q) => {
                var oe = nt(), _e = B(oe);
                const ke = ge(() => e(D) || e(F));
                Yr(_e, { name: "password", autocomplete: "current-password", get label() {
                  return d.common.password;
                }, get placeholder() {
                  return d.common.password;
                }, maxLength: 256, get disabled() {
                  return e(ke);
                }, width: y, required: true, get ref() {
                  return e(Pe);
                }, set ref(_) {
                  r(Pe, o(_));
                }, get value() {
                  return e(M);
                }, set value(_) {
                  r(M, o(_));
                } });
                var Me = v(_e, 2);
                {
                  var C = (_) => {
                    var O = st(), ie = u(O);
                    we(ie, { invisible: true, onclick: or, children: (Se, G) => {
                      pe();
                      var Ze = me();
                      w(() => T(Ze, d.authorize.passwordForgotten)), s(Se, Ze);
                    }, $$slots: { default: true } }), c(O), s(_, O);
                  };
                  g(Me, (_) => {
                    e(fe) && !e(D) && _(C);
                  });
                }
                s(q, oe);
              };
              g(P, (q) => {
                e(k) && e(Ie) !== e(x) && !e(ce) && q(ae);
              });
            }
            var Lr = v(P, 2);
            {
              var $r = (q) => {
                var oe = er(), _e = B(oe);
                {
                  var ke = (C) => {
                    var _ = lt(), O = u(_);
                    we(O, { onclick: cr, children: (ie, Se) => {
                      pe();
                      var G = me();
                      w(() => T(G, d.authorize.passwordRequest)), s(ie, G);
                    }, $$slots: { default: true } }), c(_), s(C, _);
                  }, Me = (C) => {
                    var _ = vt(), O = u(_);
                    we(O, { type: "submit", get isLoading() {
                      return e(V);
                    }, children: (ie, Se) => {
                      pe();
                      var G = me();
                      w(() => T(G, d.authorize.login)), s(ie, G);
                    }, $$slots: { default: true } }), c(_), s(C, _);
                  };
                  g(_e, (C) => {
                    e(ce) ? C(ke) : C(Me, false);
                  });
                }
                s(q, oe);
              };
              g(Lr, (q) => {
                !e(D) && !e(F) && q($r);
              });
            }
            s($, S);
          }, $$slots: { default: true } });
          var K = v(m, 2);
          {
            var ee = ($) => {
              var re = er(), S = B(re);
              {
                var E = (U) => {
                  var P = ut(), ae = u(P, true);
                  c(P), w(() => {
                    j(P, "href", `/auth/v1/users/register?redirect_uri=${e(R) ?? ""}`), T(ae, d.authorize.signUp);
                  }), s(U, P);
                }, te = (U) => {
                  var P = ct(), ae = u(P, true);
                  c(P), w(() => T(ae, d.authorize.signUp)), s(U, P);
                };
                g(S, (U) => {
                  e(R) ? U(E) : U(te, false);
                });
              }
              s($, re);
            };
            g(K, ($) => {
              e(Le) && !e(fe) && !e(D) && $(ee);
            });
          }
          s(a, i);
        };
        g(Ge, (a) => {
          e(F) || a(wr);
        });
      }
      var Je = v(Ge, 2);
      {
        var zr = (a) => {
          var i = ht(), m = u(i, true);
          c(i), w(() => T(m, e(h))), s(a, i);
        };
        g(Je, (a) => {
          e(h) && a(zr);
        });
      }
      var Qe = v(Je, 2);
      {
        var br = (a) => {
          var i = _t(), m = u(i, true);
          c(i), w(() => T(m, d.authorize.emailSentMsg)), s(a, i);
        };
        g(Qe, (a) => {
          e(he) && a(br);
        });
      }
      var Xe = v(Qe, 2);
      {
        var xr = (a) => {
          var i = mt(), m = u(i);
          we(m, { onclick: () => window.location.href = "/auth/v1/account", children: (K, ee) => {
            pe();
            var $ = me("Account");
            s(K, $);
          }, $$slots: { default: true } }), c(i), s(a, i);
        };
        g(Xe, (a) => {
          e(F) && a(xr);
        });
      }
      var Pr = v(Xe, 2);
      {
        var qr = (a) => {
          var i = gt(), m = u(i), K = v(u(m), 2), ee = u(K), $ = u(ee, true);
          c(ee), c(K), c(m);
          var re = v(m, 2);
          Er(re, 17, () => e(ve), (S) => S.id, (S, E) => {
            const te = ge(() => `Login: ${e(E).name}`);
            et(S, { get ariaLabel() {
              return e(te);
            }, get provider() {
              return e(E);
            }, onclick: lr });
          }), c(i), w(() => T($, d.authorize.orLoginWith)), s(a, i);
        };
        g(Pr, (a) => {
          !e(F) && e(ve).length > 0 && a(qr);
        });
      }
      c(Te);
      var Ye = v(Te, 2);
      Xr(Ye, { absolute: true });
      var Ir = v(Ye, 2);
      Kr(Ir, { absolute: true }), w(() => T(gr, e(le) || b)), s(p, L);
    } });
  } }), s(Q, Ae), kr();
}
export {
  jt as component
};
