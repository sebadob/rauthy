import { t as c, a as i, n as Et, d as at, e as ge } from "../chunks/BxmJRzoY.js";
import { j as s, a4 as G, c as d, k as e, r as v, t as g, l as t, p as Mt, a1 as Se, a0 as St, a5 as le, f as B, a as Ut, s as n, a2 as Ct, a3 as pe } from "../chunks/w0HvPX0p.js";
import { e as Dt, b as At, h as Nt, s as M } from "../chunks/BzP2S3Z_.js";
import { i as m } from "../chunks/iO9_dPNE.js";
import { e as qt } from "../chunks/S81raU5Y.js";
import { s as V, b as Ft } from "../chunks/BdbQ6g_y.js";
import { D as Wt, F as Ot, G as Bt, H as Ht, o as Gt, I as Vt, j as Kt, e as ot, l as Jt, J as Qt, P as Xt, K as Yt } from "../chunks/B21bTIl7.js";
import { B as be } from "../chunks/C8YTstTD.js";
import { W as Zt } from "../chunks/Be9Mus3q.js";
import { I as er } from "../chunks/Q4PIg3iI.js";
import { L as tr } from "../chunks/DAiQTto5.js";
import { p as Ue } from "../chunks/C6GSeq7M.js";
import { M as rr } from "../chunks/DKM0QPz5.js";
import { C as ar } from "../chunks/QNragXLc.js";
import { u as or } from "../chunks/0cG6LBdy.js";
import { T as H } from "../chunks/BdDmaO9S.js";
import { u as N } from "../chunks/Di69SM1b.js";
import { T as sr } from "../chunks/B1f0afjj.js";
import { I as ir } from "../chunks/iedauS3r.js";
import { b as ue } from "../chunks/BO1A6s0c.js";
import { u as st } from "../chunks/F_Qf1tHt.js";
import { F as nr } from "../chunks/CDe-qvZi.js";
import { B as lr } from "../chunks/CybldwDO.js";
import { R as ur } from "../chunks/B-AEc_6C.js";
var dr = c('<img alt="Client Logo" width="100%" height="100%">'), vr = c('<div class="logo svelte-1gwy16z"><!></div>');
function cr(te, P) {
  let q = s(false), u = G(() => P.updated ? `/auth/v1/clients/${P.clientId}/logo?updated=${P.updated}` : `/auth/v1/clients/${P.clientId}/logo`);
  var y = vr(), R = d(y);
  {
    var p = (x) => {
      ur(x, { width: "100%" });
    }, K = (x) => {
      var b = dr();
      g(() => V(b, "src", e(u))), Dt("error", b, () => t(q, true)), At(b), i(x, b);
    };
    m(R, (x) => {
      e(q) || P.updated === void 0 ? x(p) : x(K, false);
    });
  }
  v(y), i(te, y);
}
var fr = Et(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125
            1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504
            1.125-1.125V9.75M8.25 21h8.25"></path></svg>`);
function _r(te, P) {
  let q = Ue(P, "opacity", 3, 0.9), u = Ue(P, "width", 3, "1.5rem"), y = Ue(P, "color", 3, "currentColor");
  var R = fr();
  V(R, "stroke-width", 2), g(() => {
    V(R, "stroke", y()), V(R, "width", u()), V(R, "opacity", q());
  }), i(te, R);
}
var mr = c('<a class="home svelte-ftjjbu"><!></a>'), hr = c('<div class="forgotten svelte-ftjjbu"><!></div>'), gr = c("<!> <!>", 1), pr = c('<div class="btn flex-col svelte-ftjjbu"><!></div>'), br = c('<div class="btn flex-col svelte-ftjjbu"><!></div>'), wr = c("<div><!></div> <!> <!>", 1), jr = c('<a class="reg svelte-ftjjbu" target="_blank"> </a>'), Lr = c('<a class="reg svelte-ftjjbu" href="/auth/v1/users/register" target="_blank"> </a>'), Pr = c("<!> <!>", 1), xr = c('<div class="errMsg svelte-ftjjbu"> </div>'), Ir = c('<div class="success svelte-ftjjbu"> </div>'), Tr = c('<div class="btn flex-col svelte-ftjjbu"><!></div>'), yr = c('<div class="providers flex-col svelte-ftjjbu"><div class="providersSeparator svelte-ftjjbu"><div class="separator svelte-ftjjbu"></div> <div class="loginWith svelte-ftjjbu"><div class="svelte-ftjjbu"> </div></div></div> <!></div>'), Rr = c('<div class="container svelte-ftjjbu"><div class="head svelte-ftjjbu"><!> <!></div> <div class="name svelte-ftjjbu"><h2> </h2></div> <!> <!> <!> <!> <!> <!></div> <!> <!>', 1), $r = c("<!> <!> <!> <!> <!> <!> <!> <!>", 1);
function ta(te, P) {
  var _a;
  Mt(P, true);
  const q = "18rem";
  let u = or(), y = st().get(), R = G(() => y ? "/auth/v1/dev/authorize" : "/auth/v1/oidc/authorize"), p = N("client_id").get(), K = s(""), x = s(-1), b = s(Se(y ? "/auth/v1" : "")), J = N("redirect_uri").get(), we = N("nonce").get(), je = ((_a = N("scope").get()) == null ? void 0 : _a.split(" ")) || [], Le = s(void 0), Pe = s(void 0), xe = N("state").get(), re = N("code_challenge").get(), $ = N("code_challenge_method").get(), Ce = false, Ie = s(void 0), de = s(Se([])), ve = s(void 0), Q = s(false), f = s(""), ae = s(""), ce = s(""), S = s(false), F = s(false), fe = s(false), _e = s(false), me = s(false), W = s(false), De = s(""), Te = s(false), I = s(Se(N("login_hint").get() || "")), U = s(""), ye = s(""), Ae = G(() => e(S) && e(Ie) !== e(I) && !e(fe));
  St(() => {
    var _a2;
    e(S) || ((_a2 = e(Le)) == null ? void 0 : _a2.focus());
  }), le(() => {
    Ce && (p == null ? void 0 : p.length) && lt();
  }), le(() => {
    e(me) && setTimeout(() => {
      t(me, false), t(fe, false), t(_e, false);
    }, 3e3);
  }), le(() => {
    var _a2;
    (_a2 = e(Pe)) == null ? void 0 : _a2.focus();
  }), le(() => {
    var _a2;
    if (e(ae) === "Refresh") Ce = true;
    else if ((_a2 = e(ae)) == null ? void 0 : _a2.startsWith("MfaLogin ")) {
      let r = e(ae).replace("MfaLogin ", "");
      t(I, r, true), t(Ie, r, true);
    }
  }), le(() => {
    y ? it() : e(ce) && ot(e(ce));
  });
  async function it() {
    var _a2;
    let r = await ue("/auth/v1/oidc/session");
    ((_a2 = r.body) == null ? void 0 : _a2.csrf_token) ? ot(r.body.csrf_token) : console.error(r.error);
  }
  function nt() {
    t(f, ""), t(fe, true), t(U, "");
  }
  async function lt() {
    if (!p) {
      console.error("clientId is undefined");
      return;
    }
    if (!J) {
      console.error("redirectUri is undefined");
      return;
    }
    t(Q, true);
    const r = { client_id: p, redirect_uri: J, state: xe, nonce: we, scopes: je };
    re && $ && ($ === "plain" || $ === "S256") && (r.code_challenge = re, r.code_challenge_method = $);
    let l = await ue("/auth/v1/oidc/authorize/refresh", r);
    await Ne(l);
  }
  async function ut(r, l) {
    if (t(f, ""), !p) {
      console.error("clientId is undefined");
      return;
    }
    if (!J) {
      console.error("redirectUri is undefined");
      return;
    }
    const w = { email: e(I), client_id: p, redirect_uri: J, state: xe, nonce: we, scopes: je };
    if (re && $ && ($ === "plain" || $ === "S256") && (w.code_challenge = re, w.code_challenge_method = $), e(S) && e(I) !== e(Ie)) {
      if (!e(U)) {
        t(f, u.authorize.passwordRequired, true);
        return;
      }
      if (e(U).length > 256) {
        t(f, "max 256");
        return;
      }
      w.password = e(U);
    }
    t(Q, true);
    let L = "/auth/v1/oidc/authorize";
    st().get() && (L = "/auth/v1/dev/authorize");
    let k = await ue(L, w, "json", "noRedirect");
    await Ne(k);
  }
  async function Ne(r) {
    if (r.status === 202) {
      let l = r.headers.get("location");
      if (!l) {
        console.error("location header missing");
        return;
      }
      window.location.replace(l);
    } else if (r.status === 200) {
      t(f, "");
      let l = r.body;
      l && "user_id" in l && "code" in l ? (t(ye, l.user_id, true), t(ve, { Login: l.code }, true)) : console.error("did not receive a proper WebauthnLoginResponse after HTTP200");
    } else if (r.status === 406) t(f, u.authorize.clientForceMfa, true), t(F, true);
    else if (r.status === 429) {
      let l = r.headers.get("x-retry-not-before");
      if (!l) {
        console.error("x-retry-not-before header missing");
        return;
      }
      let w = Number.parseInt(l), L = Jt(w), k = w * 1e3 - (/* @__PURE__ */ new Date()).getTime();
      t(W, true), t(f, `${u.authorize.http429} ${L}`), t(I, ""), t(U, ""), t(S, false), setTimeout(() => {
        t(W, false), t(f, "");
      }, k);
    } else e(S) ? (t(f, u.authorize.invalidCredentials, true), t(_e, true)) : (t(S, true), t(De, e(I), true));
    t(Q, false);
  }
  function dt() {
    e(S) && e(De) !== e(I) && (t(S, false), t(U, ""), t(f, ""));
  }
  function vt(r) {
    Qt(64, (l, { challenge: w, verifier: L }) => {
      l || (localStorage.setItem(Xt, L), ct(r, w));
    });
  }
  async function ct(r, l) {
    var _a2;
    if (!p) {
      console.error("clientId is undefined");
      return;
    }
    if (!J) {
      console.error("redirectUri is undefined");
      return;
    }
    let w = { email: e(I) || void 0, client_id: p, redirect_uri: J, scopes: je, state: xe, nonce: we, code_challenge: re, code_challenge_method: $, provider_id: r, pkce_challenge: l }, L = await ue("/auth/v1/providers/login", w);
    if (L.text) {
      Yt(L.text);
      let k = L.headers.get("location");
      if (!k) {
        console.error("no location header set for provider login");
        return;
      }
      window.location.href = k;
    } else t(f, ((_a2 = L.error) == null ? void 0 : _a2.message) || "Error", true);
  }
  function ft(r) {
    t(ve, void 0), t(f, r, true);
  }
  function _t(r) {
    r && "loc" in r && window.location.replace(r.loc);
  }
  async function mt() {
    let r = { email: e(I) };
    e(b) && (r.redirect_uri = encodeURI(e(b))), t(Q, true);
    let l = await ue("/auth/v1/users/request_reset", r);
    l.error ? t(f, l.error.message, true) : t(me, true), t(Q, false);
  }
  var qe = $r();
  Nt((r) => {
    g(() => Ct.title = `Login ${(e(K) || p) ?? ""}`);
  });
  var Fe = B(qe);
  H(Fe, { id: Wt, get value() {
    return e(de);
  }, set value(r) {
    t(de, r, true);
  } });
  var We = n(Fe, 2);
  H(We, { id: Ot, get value() {
    return e(K);
  }, set value(r) {
    t(K, r, true);
  } });
  var Oe = n(We, 2);
  H(Oe, { id: Bt, get value() {
    return e(b);
  }, set value(r) {
    t(b, r, true);
  } });
  var Be = n(Oe, 2);
  H(Be, { id: Ht, get value() {
    return e(x);
  }, set value(r) {
    t(x, r, true);
  } });
  var He = n(Be, 2);
  H(He, { id: Gt, get value() {
    return e(ce);
  }, set value(r) {
    t(ce, r, true);
  } });
  var Ge = n(He, 2);
  H(Ge, { id: Vt, get value() {
    return e(ae);
  }, set value(r) {
    t(ae, r, true);
  } });
  var Ve = n(Ge, 2);
  H(Ve, { id: Kt, get value() {
    return e(Te);
  }, set value(r) {
    t(Te, r, true);
  } });
  var ht = n(Ve, 2);
  rr(ht, { children: (r, l) => {
    ar(r, { children: (w, L) => {
      var k = Rr(), Re = B(k), $e = d(Re), Ke = d($e);
      {
        var gt = (a) => {
          const o = G(() => e(x) > -1 ? e(x) : void 0);
          cr(a, { clientId: p, get updated() {
            return e(o);
          } });
        };
        m(Ke, (a) => {
          p && a(gt);
        });
      }
      var pt = n(Ke, 2);
      {
        var bt = (a) => {
          var o = mr(), h = d(o);
          _r(h, { color: "hsla(var(--text) / .4)" }), v(o), g(() => V(o, "href", e(b))), i(a, o);
        };
        m(pt, (a) => {
          e(b) && a(bt);
        });
      }
      v($e);
      var ke = n($e, 2), Je = d(ke), wt = d(Je, true);
      v(Je), v(ke);
      var Qe = n(ke, 2);
      {
        var jt = (a) => {
          Zt(a, { get userId() {
            return e(ye);
          }, get purpose() {
            return e(ve);
          }, onSuccess: _t, onError: ft });
        };
        m(Qe, (a) => {
          e(ve) && e(ye) && a(jt);
        });
      }
      var Xe = n(Qe, 2);
      {
        var Lt = (a) => {
          var o = Pr(), h = B(o);
          nr(h, { get action() {
            return e(R);
          }, onSubmit: ut, children: (z, se) => {
            var C = wr(), E = B(C);
            let Y;
            var D = d(E);
            const T = G(() => e(W) || e(F));
            er(D, { typ: "email", name: "email", autocomplete: "email", get label() {
              return u.common.email;
            }, get placeholder() {
              return u.common.email;
            }, get errMsg() {
              return u.authorize.validEmail;
            }, get disabled() {
              return e(T);
            }, onInput: dt, width: q, required: true, get ref() {
              return e(Le);
            }, set ref(j) {
              t(Le, j, true);
            }, get value() {
              return e(I);
            }, set value(j) {
              t(I, j, true);
            } }), v(E);
            var Z = n(E, 2);
            {
              var $t = (j) => {
                var ie = gr(), he = B(ie);
                const ze = G(() => e(W) || e(F));
                ir(he, { name: "password", autocomplete: "current-password", get label() {
                  return u.common.password;
                }, get placeholder() {
                  return u.common.password;
                }, maxLength: 256, get disabled() {
                  return e(ze);
                }, width: q, required: true, get ref() {
                  return e(Pe);
                }, set ref(_) {
                  t(Pe, _, true);
                }, get value() {
                  return e(U);
                }, set value(_) {
                  t(U, _, true);
                } });
                var Ee = n(he, 2);
                {
                  var A = (_) => {
                    var O = hr(), ne = d(O);
                    be(ne, { invisible: true, onclick: nt, children: (Me, ee) => {
                      pe();
                      var rt = ge();
                      g(() => M(rt, u.authorize.passwordForgotten)), i(Me, rt);
                    }, $$slots: { default: true } }), v(O), i(_, O);
                  };
                  m(Ee, (_) => {
                    e(_e) && !e(W) && _(A);
                  });
                }
                i(j, ie);
              };
              m(Z, (j) => {
                e(Ae) && j($t);
              });
            }
            var kt = n(Z, 2);
            {
              var zt = (j) => {
                var ie = at(), he = B(ie);
                {
                  var ze = (A) => {
                    var _ = pr(), O = d(_);
                    be(O, { onclick: mt, children: (ne, Me) => {
                      pe();
                      var ee = ge();
                      g(() => M(ee, u.authorize.passwordRequest)), i(ne, ee);
                    }, $$slots: { default: true } }), v(_), i(A, _);
                  }, Ee = (A) => {
                    var _ = br(), O = d(_);
                    be(O, { type: "submit", get isLoading() {
                      return e(Q);
                    }, children: (ne, Me) => {
                      pe();
                      var ee = ge();
                      g(() => M(ee, u.authorize.login)), i(ne, ee);
                    }, $$slots: { default: true } }), v(_), i(A, _);
                  };
                  m(he, (A) => {
                    e(fe) ? A(ze) : A(Ee, false);
                  });
                }
                i(j, ie);
              };
              m(kt, (j) => {
                !e(W) && !e(F) && j(zt);
              });
            }
            g((j) => Y = Ft(E, 1, "svelte-ftjjbu", null, Y, j), [() => ({ emailMinHeight: !e(Ae) })]), i(z, C);
          }, $$slots: { default: true } });
          var X = n(h, 2);
          {
            var oe = (z) => {
              var se = at(), C = B(se);
              {
                var E = (D) => {
                  var T = jr(), Z = d(T, true);
                  v(T), g(() => {
                    V(T, "href", `/auth/v1/users/register?redirect_uri=${e(b) ?? ""}`), M(Z, u.authorize.signUp);
                  }), i(D, T);
                }, Y = (D) => {
                  var T = Lr(), Z = d(T, true);
                  v(T), g(() => M(Z, u.authorize.signUp)), i(D, T);
                };
                m(C, (D) => {
                  e(b) ? D(E) : D(Y, false);
                });
              }
              i(z, se);
            };
            m(X, (z) => {
              e(Te) && !e(_e) && !e(W) && z(oe);
            });
          }
          i(a, o);
        };
        m(Xe, (a) => {
          e(F) || a(Lt);
        });
      }
      var Ye = n(Xe, 2);
      {
        var Pt = (a) => {
          var o = xr(), h = d(o, true);
          v(o), g(() => M(h, e(f))), i(a, o);
        };
        m(Ye, (a) => {
          e(f) && a(Pt);
        });
      }
      var Ze = n(Ye, 2);
      {
        var xt = (a) => {
          var o = Ir(), h = d(o, true);
          v(o), g(() => M(h, u.authorize.emailSentMsg)), i(a, o);
        };
        m(Ze, (a) => {
          e(me) && a(xt);
        });
      }
      var et = n(Ze, 2);
      {
        var It = (a) => {
          var o = Tr(), h = d(o);
          be(h, { onclick: () => window.location.href = "/auth/v1/account", children: (X, oe) => {
            pe();
            var z = ge("Account");
            i(X, z);
          }, $$slots: { default: true } }), v(o), i(a, o);
        };
        m(et, (a) => {
          e(F) && a(It);
        });
      }
      var Tt = n(et, 2);
      {
        var yt = (a) => {
          var o = yr(), h = d(o), X = n(d(h), 2), oe = d(X), z = d(oe, true);
          v(oe), v(X), v(h);
          var se = n(h, 2);
          qt(se, 17, () => e(de), (C) => C.id, (C, E) => {
            const Y = G(() => `Login: ${e(E).name}`);
            lr(C, { get ariaLabel() {
              return e(Y);
            }, get provider() {
              return e(E);
            }, onclick: vt });
          }), v(o), g(() => M(z, u.authorize.orLoginWith)), i(a, o);
        };
        m(Tt, (a) => {
          !e(F) && e(de).length > 0 && a(yt);
        });
      }
      v(Re);
      var tt = n(Re, 2);
      sr(tt, { absolute: true });
      var Rt = n(tt, 2);
      tr(Rt, { absolute: true }), g(() => M(wt, e(K) || p)), i(w, k);
    } });
  } }), i(te, qe), Ut();
}
export {
  ta as component
};
