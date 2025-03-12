import { t as f, a as n, n as Et, d as at, e as pe } from "../chunks/DKC5GJ29.js";
import { c as v, j as e, k as i, r as c, t as p, a9 as V, l as t, p as Mt, a6 as St, aa as de, f as H, a as Ut, s as l, a7 as Ct, a8 as be } from "../chunks/BveBAmlr.js";
import { e as Dt, b as At, h as Nt, s as S } from "../chunks/CYCba2oX.js";
import { i as h } from "../chunks/D-uYoVwt.js";
import { e as qt } from "../chunks/BLE4FKaJ.js";
import { s as K, b as Ft } from "../chunks/Dql74IOz.js";
import { p as s } from "../chunks/VbPNpVtZ.js";
import { D as Wt, F as Ot, G as Bt, H as Ht, o as Gt, I as Vt, j as Kt, e as ot, l as Jt, J as Qt, P as Xt, K as Yt } from "../chunks/B21bTIl7.js";
import { B as we } from "../chunks/DlLEcmNg.js";
import { W as Zt } from "../chunks/C3pTUZii.js";
import { I as er } from "../chunks/CCQyB9gY.js";
import { L as tr } from "../chunks/DNYXBAk_.js";
import { p as Ue } from "../chunks/Db0ChEdV.js";
import { M as rr } from "../chunks/DaM_1q3c.js";
import { C as ar } from "../chunks/Ci6f1shh.js";
import { u as or } from "../chunks/8R5My_LO.js";
import { T as G } from "../chunks/CDuWpfYL.js";
import { u as q } from "../chunks/BdFU-gNS.js";
import { T as sr } from "../chunks/BFgWaswG.js";
import { I as ir } from "../chunks/CUO8Plwh.js";
import { b as ue } from "../chunks/BO1A6s0c.js";
import { u as st } from "../chunks/NJ3e7ymm.js";
import { F as nr } from "../chunks/BguaNybM.js";
import { B as lr } from "../chunks/DjyBHFkv.js";
import { R as dr } from "../chunks/CE9vlfyw.js";
var ur = f('<img alt="Client Logo" width="100%" height="100%">'), vr = f('<div class="logo svelte-1gwy16z"><!></div>');
function cr(re, x) {
  let F = i(false), u = V(() => x.updated ? `/auth/v1/clients/${x.clientId}/logo?updated=${x.updated}` : `/auth/v1/clients/${x.clientId}/logo`);
  var R = vr(), $ = v(R);
  {
    var b = (I) => {
      dr(I, { width: "100%" });
    }, J = (I) => {
      var w = ur();
      p(() => K(w, "src", e(u))), Dt("error", w, () => t(F, true)), At(w), n(I, w);
    };
    h($, (I) => {
      e(F) || x.updated === void 0 ? I(b) : I(J, false);
    });
  }
  c(R), n(re, R);
}
var fr = Et(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125
            1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504
            1.125-1.125V9.75M8.25 21h8.25"></path></svg>`);
function _r(re, x) {
  let F = Ue(x, "opacity", 3, 0.9), u = Ue(x, "width", 3, "1.5rem"), R = Ue(x, "color", 3, "currentColor");
  var $ = fr();
  K($, "stroke-width", 2), p(() => {
    K($, "stroke", R()), K($, "width", u()), K($, "opacity", F());
  }), n(re, $);
}
var mr = f('<a class="home svelte-ftjjbu"><!></a>'), hr = f('<div class="forgotten svelte-ftjjbu"><!></div>'), gr = f("<!> <!>", 1), pr = f('<div class="btn flex-col svelte-ftjjbu"><!></div>'), br = f('<div class="btn flex-col svelte-ftjjbu"><!></div>'), wr = f("<div><!></div> <!> <!>", 1), jr = f('<a class="reg svelte-ftjjbu" target="_blank"> </a>'), Lr = f('<a class="reg svelte-ftjjbu" href="/auth/v1/users/register" target="_blank"> </a>'), Pr = f("<!> <!>", 1), xr = f('<div class="errMsg svelte-ftjjbu"> </div>'), Ir = f('<div class="success svelte-ftjjbu"> </div>'), Tr = f('<div class="btn flex-col svelte-ftjjbu"><!></div>'), yr = f('<div class="providers flex-col svelte-ftjjbu"><div class="providersSeparator svelte-ftjjbu"><div class="separator svelte-ftjjbu"></div> <div class="loginWith svelte-ftjjbu"><div class="svelte-ftjjbu"> </div></div></div> <!></div>'), Rr = f('<div class="container svelte-ftjjbu"><div class="head svelte-ftjjbu"><!> <!></div> <div class="name svelte-ftjjbu"><h2> </h2></div> <!> <!> <!> <!> <!> <!></div> <!> <!>', 1), $r = f("<!> <!> <!> <!> <!> <!> <!> <!>", 1);
function ra(re, x) {
  var _a;
  Mt(x, true);
  const F = "18rem";
  let u = or(), R = st().get(), $ = V(() => R ? "/auth/v1/dev/authorize" : "/auth/v1/oidc/authorize"), b = q("client_id").get(), J = i(""), I = i(-1), w = i(s(R ? "/auth/v1" : "")), Q = q("redirect_uri").get(), je = q("nonce").get(), Le = ((_a = q("scope").get()) == null ? void 0 : _a.split(" ")) || [], Pe = i(void 0), xe = i(void 0), Ie = q("state").get(), ae = q("code_challenge").get(), k = q("code_challenge_method").get(), Ce = false, Te = i(void 0), ve = i(s([])), ce = i(void 0), X = i(false), _ = i(""), oe = i(""), fe = i(""), U = i(false), W = i(false), _e = i(false), me = i(false), he = i(false), O = i(false), De = i(""), ye = i(false), T = i(s(q("login_hint").get() || "")), C = i(""), Re = i(""), Ae = V(() => e(U) && e(Te) !== e(T) && !e(_e));
  St(() => {
    var _a2;
    e(U) || ((_a2 = e(Pe)) == null ? void 0 : _a2.focus());
  }), de(() => {
    Ce && (b == null ? void 0 : b.length) && lt();
  }), de(() => {
    e(he) && setTimeout(() => {
      t(he, false), t(_e, false), t(me, false);
    }, 3e3);
  }), de(() => {
    var _a2;
    (_a2 = e(xe)) == null ? void 0 : _a2.focus();
  }), de(() => {
    var _a2;
    if (e(oe) === "Refresh") Ce = true;
    else if ((_a2 = e(oe)) == null ? void 0 : _a2.startsWith("MfaLogin ")) {
      let r = e(oe).replace("MfaLogin ", "");
      t(T, s(r)), t(Te, s(r));
    }
  }), de(() => {
    R ? it() : e(fe) && ot(e(fe));
  });
  async function it() {
    var _a2;
    let r = await ue("/auth/v1/oidc/session");
    ((_a2 = r.body) == null ? void 0 : _a2.csrf_token) ? ot(r.body.csrf_token) : console.error(r.error);
  }
  function nt() {
    t(_, ""), t(_e, true), t(C, "");
  }
  async function lt() {
    if (!b) {
      console.error("clientId is undefined");
      return;
    }
    if (!Q) {
      console.error("redirectUri is undefined");
      return;
    }
    t(X, true);
    const r = { client_id: b, redirect_uri: Q, state: Ie, nonce: je, scopes: Le };
    ae && k && (k === "plain" || k === "S256") && (r.code_challenge = ae, r.code_challenge_method = k);
    let d = await ue("/auth/v1/oidc/authorize/refresh", r);
    await Ne(d);
  }
  async function dt(r, d) {
    if (t(_, ""), !b) {
      console.error("clientId is undefined");
      return;
    }
    if (!Q) {
      console.error("redirectUri is undefined");
      return;
    }
    const j = { email: e(T), client_id: b, redirect_uri: Q, state: Ie, nonce: je, scopes: Le };
    if (ae && k && (k === "plain" || k === "S256") && (j.code_challenge = ae, j.code_challenge_method = k), e(U) && e(T) !== e(Te)) {
      if (!e(C)) {
        t(_, s(u.authorize.passwordRequired));
        return;
      }
      if (e(C).length > 256) {
        t(_, "max 256");
        return;
      }
      j.password = e(C);
    }
    t(X, true);
    let P = "/auth/v1/oidc/authorize";
    st().get() && (P = "/auth/v1/dev/authorize");
    let z = await ue(P, j, "json", "noRedirect");
    await Ne(z);
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
      t(_, "");
      let d = r.body;
      d && "user_id" in d && "code" in d ? (t(Re, s(d.user_id)), t(ce, s({ Login: d.code }))) : console.error("did not receive a proper WebauthnLoginResponse after HTTP200");
    } else if (r.status === 406) t(_, s(u.authorize.clientForceMfa)), t(W, true);
    else if (r.status === 429) {
      let d = r.headers.get("x-retry-not-before");
      if (!d) {
        console.error("x-retry-not-before header missing");
        return;
      }
      let j = Number.parseInt(d), P = Jt(j), z = j * 1e3 - (/* @__PURE__ */ new Date()).getTime();
      t(O, true), t(_, `${u.authorize.http429} ${P}`), t(T, ""), t(C, ""), t(U, false), setTimeout(() => {
        t(O, false), t(_, "");
      }, z);
    } else e(U) ? (t(_, s(u.authorize.invalidCredentials)), t(me, true)) : (t(U, true), t(De, s(e(T))));
    t(X, false);
  }
  function ut() {
    e(U) && e(De) !== e(T) && (t(U, false), t(C, ""), t(_, ""));
  }
  function vt(r) {
    Qt(64, (d, { challenge: j, verifier: P }) => {
      d || (localStorage.setItem(Xt, P), ct(r, j));
    });
  }
  async function ct(r, d) {
    var _a2;
    if (!b) {
      console.error("clientId is undefined");
      return;
    }
    if (!Q) {
      console.error("redirectUri is undefined");
      return;
    }
    let j = { email: e(T) || void 0, client_id: b, redirect_uri: Q, scopes: Le, state: Ie, nonce: je, code_challenge: ae, code_challenge_method: k, provider_id: r, pkce_challenge: d }, P = await ue("/auth/v1/providers/login", j);
    if (P.text) {
      Yt(P.text);
      let z = P.headers.get("location");
      if (!z) {
        console.error("no location header set for provider login");
        return;
      }
      window.location.href = z;
    } else t(_, s(((_a2 = P.error) == null ? void 0 : _a2.message) || "Error"));
  }
  function ft(r) {
    t(ce, void 0), t(_, s(r));
  }
  function _t(r) {
    r && "loc" in r && window.location.replace(r.loc);
  }
  async function mt() {
    let r = { email: e(T) };
    e(w) && (r.redirect_uri = encodeURI(e(w))), t(X, true);
    let d = await ue("/auth/v1/users/request_reset", r);
    d.error ? t(_, s(d.error.message)) : t(he, true), t(X, false);
  }
  var qe = $r();
  Nt((r) => {
    p(() => Ct.title = `Login ${(e(J) || b) ?? ""}`);
  });
  var Fe = H(qe);
  G(Fe, { id: Wt, get value() {
    return e(ve);
  }, set value(r) {
    t(ve, s(r));
  } });
  var We = l(Fe, 2);
  G(We, { id: Ot, get value() {
    return e(J);
  }, set value(r) {
    t(J, s(r));
  } });
  var Oe = l(We, 2);
  G(Oe, { id: Bt, get value() {
    return e(w);
  }, set value(r) {
    t(w, s(r));
  } });
  var Be = l(Oe, 2);
  G(Be, { id: Ht, get value() {
    return e(I);
  }, set value(r) {
    t(I, s(r));
  } });
  var He = l(Be, 2);
  G(He, { id: Gt, get value() {
    return e(fe);
  }, set value(r) {
    t(fe, s(r));
  } });
  var Ge = l(He, 2);
  G(Ge, { id: Vt, get value() {
    return e(oe);
  }, set value(r) {
    t(oe, s(r));
  } });
  var Ve = l(Ge, 2);
  G(Ve, { id: Kt, get value() {
    return e(ye);
  }, set value(r) {
    t(ye, s(r));
  } });
  var ht = l(Ve, 2);
  rr(ht, { children: (r, d) => {
    ar(r, { children: (j, P) => {
      var z = Rr(), $e = H(z), ke = v($e), Ke = v(ke);
      {
        var gt = (a) => {
          const o = V(() => e(I) > -1 ? e(I) : void 0);
          cr(a, { clientId: b, get updated() {
            return e(o);
          } });
        };
        h(Ke, (a) => {
          b && a(gt);
        });
      }
      var pt = l(Ke, 2);
      {
        var bt = (a) => {
          var o = mr(), g = v(o);
          _r(g, { color: "hsla(var(--text) / .4)" }), c(o), p(() => K(o, "href", e(w))), n(a, o);
        };
        h(pt, (a) => {
          e(w) && a(bt);
        });
      }
      c(ke);
      var ze = l(ke, 2), Je = v(ze), wt = v(Je, true);
      c(Je), c(ze);
      var Qe = l(ze, 2);
      {
        var jt = (a) => {
          Zt(a, { get userId() {
            return e(Re);
          }, get purpose() {
            return e(ce);
          }, onSuccess: _t, onError: ft });
        };
        h(Qe, (a) => {
          e(ce) && e(Re) && a(jt);
        });
      }
      var Xe = l(Qe, 2);
      {
        var Lt = (a) => {
          var o = Pr(), g = H(o);
          nr(g, { get action() {
            return e($);
          }, onSubmit: dt, children: (E, ie) => {
            var D = wr(), M = H(D);
            let Z;
            var A = v(M);
            const y = V(() => e(O) || e(W));
            er(A, { typ: "email", name: "email", autocomplete: "email", get label() {
              return u.common.email;
            }, get placeholder() {
              return u.common.email;
            }, get errMsg() {
              return u.authorize.validEmail;
            }, get disabled() {
              return e(y);
            }, onInput: ut, width: F, required: true, get ref() {
              return e(Pe);
            }, set ref(L) {
              t(Pe, s(L));
            }, get value() {
              return e(T);
            }, set value(L) {
              t(T, s(L));
            } }), c(M);
            var ee = l(M, 2);
            {
              var $t = (L) => {
                var ne = gr(), ge = H(ne);
                const Ee = V(() => e(O) || e(W));
                ir(ge, { name: "password", autocomplete: "current-password", get label() {
                  return u.common.password;
                }, get placeholder() {
                  return u.common.password;
                }, maxLength: 256, get disabled() {
                  return e(Ee);
                }, width: F, required: true, get ref() {
                  return e(xe);
                }, set ref(m) {
                  t(xe, s(m));
                }, get value() {
                  return e(C);
                }, set value(m) {
                  t(C, s(m));
                } });
                var Me = l(ge, 2);
                {
                  var N = (m) => {
                    var B = hr(), le = v(B);
                    we(le, { invisible: true, onclick: nt, children: (Se, te) => {
                      be();
                      var rt = pe();
                      p(() => S(rt, u.authorize.passwordForgotten)), n(Se, rt);
                    }, $$slots: { default: true } }), c(B), n(m, B);
                  };
                  h(Me, (m) => {
                    e(me) && !e(O) && m(N);
                  });
                }
                n(L, ne);
              };
              h(ee, (L) => {
                e(Ae) && L($t);
              });
            }
            var kt = l(ee, 2);
            {
              var zt = (L) => {
                var ne = at(), ge = H(ne);
                {
                  var Ee = (N) => {
                    var m = pr(), B = v(m);
                    we(B, { onclick: mt, children: (le, Se) => {
                      be();
                      var te = pe();
                      p(() => S(te, u.authorize.passwordRequest)), n(le, te);
                    }, $$slots: { default: true } }), c(m), n(N, m);
                  }, Me = (N) => {
                    var m = br(), B = v(m);
                    we(B, { type: "submit", get isLoading() {
                      return e(X);
                    }, children: (le, Se) => {
                      be();
                      var te = pe();
                      p(() => S(te, u.authorize.login)), n(le, te);
                    }, $$slots: { default: true } }), c(m), n(N, m);
                  };
                  h(ge, (N) => {
                    e(_e) ? N(Ee) : N(Me, false);
                  });
                }
                n(L, ne);
              };
              h(kt, (L) => {
                !e(O) && !e(W) && L(zt);
              });
            }
            p((L) => Z = Ft(M, 1, "svelte-ftjjbu", null, Z, L), [() => ({ emailMinHeight: !e(Ae) })]), n(E, D);
          }, $$slots: { default: true } });
          var Y = l(g, 2);
          {
            var se = (E) => {
              var ie = at(), D = H(ie);
              {
                var M = (A) => {
                  var y = jr(), ee = v(y, true);
                  c(y), p(() => {
                    K(y, "href", `/auth/v1/users/register?redirect_uri=${e(w) ?? ""}`), S(ee, u.authorize.signUp);
                  }), n(A, y);
                }, Z = (A) => {
                  var y = Lr(), ee = v(y, true);
                  c(y), p(() => S(ee, u.authorize.signUp)), n(A, y);
                };
                h(D, (A) => {
                  e(w) ? A(M) : A(Z, false);
                });
              }
              n(E, ie);
            };
            h(Y, (E) => {
              e(ye) && !e(me) && !e(O) && E(se);
            });
          }
          n(a, o);
        };
        h(Xe, (a) => {
          e(W) || a(Lt);
        });
      }
      var Ye = l(Xe, 2);
      {
        var Pt = (a) => {
          var o = xr(), g = v(o, true);
          c(o), p(() => S(g, e(_))), n(a, o);
        };
        h(Ye, (a) => {
          e(_) && a(Pt);
        });
      }
      var Ze = l(Ye, 2);
      {
        var xt = (a) => {
          var o = Ir(), g = v(o, true);
          c(o), p(() => S(g, u.authorize.emailSentMsg)), n(a, o);
        };
        h(Ze, (a) => {
          e(he) && a(xt);
        });
      }
      var et = l(Ze, 2);
      {
        var It = (a) => {
          var o = Tr(), g = v(o);
          we(g, { onclick: () => window.location.href = "/auth/v1/account", children: (Y, se) => {
            be();
            var E = pe("Account");
            n(Y, E);
          }, $$slots: { default: true } }), c(o), n(a, o);
        };
        h(et, (a) => {
          e(W) && a(It);
        });
      }
      var Tt = l(et, 2);
      {
        var yt = (a) => {
          var o = yr(), g = v(o), Y = l(v(g), 2), se = v(Y), E = v(se, true);
          c(se), c(Y), c(g);
          var ie = l(g, 2);
          qt(ie, 17, () => e(ve), (D) => D.id, (D, M) => {
            const Z = V(() => `Login: ${e(M).name}`);
            lr(D, { get ariaLabel() {
              return e(Z);
            }, get provider() {
              return e(M);
            }, onclick: vt });
          }), c(o), p(() => S(E, u.authorize.orLoginWith)), n(a, o);
        };
        h(Tt, (a) => {
          !e(W) && e(ve).length > 0 && a(yt);
        });
      }
      c($e);
      var tt = l($e, 2);
      sr(tt, { absolute: true });
      var Rt = l(tt, 2);
      tr(Rt, { absolute: true }), p(() => S(wt, e(J) || b)), n(j, z);
    } });
  } }), n(re, qe), Ut();
}
export {
  ra as component
};
