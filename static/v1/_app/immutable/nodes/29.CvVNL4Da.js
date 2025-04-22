import { t as c, a as i, n as Dt, d as nt, e as fe } from "../chunks/BxmJRzoY.js";
import { j as s, a4 as O, c as v, k as e, r as u, t as P, l as t, p as At, a1 as Se, a0 as Nt, a5 as ae, f as F, a as qt, s as l, a2 as Ft, a3 as me } from "../chunks/w0HvPX0p.js";
import { e as Wt, b as Ot, h as Ht, s as k } from "../chunks/BzP2S3Z_.js";
import { i as h } from "../chunks/iO9_dPNE.js";
import { e as Bt } from "../chunks/S81raU5Y.js";
import { s as H, b as Gt } from "../chunks/BdbQ6g_y.js";
import { D as Vt, F as Kt, G as Jt, H as Qt, o as Xt, I as Yt, j as Zt, e as dt, l as er, J as tr, P as rr, K as ar } from "../chunks/B21bTIl7.js";
import { B as _e } from "../chunks/C4AV2CoD.js";
import { W as or } from "../chunks/D40x3DMA.js";
import { I as sr } from "../chunks/Bk5EVqw2.js";
import { L as ir } from "../chunks/Cmb3Gob_.js";
import { p as Ue } from "../chunks/C6GSeq7M.js";
import { M as lr } from "../chunks/DKM0QPz5.js";
import { C as nr } from "../chunks/QNragXLc.js";
import { u as dr } from "../chunks/N6FgGI8m.js";
import { T as W } from "../chunks/Dhcsa8BW.js";
import { u as C } from "../chunks/DvTygCXn.js";
import { T as vr } from "../chunks/B808p9S3.js";
import { I as ur } from "../chunks/CfKSJy9E.js";
import { b as oe } from "../chunks/UPFlzoow.js";
import { u as vt } from "../chunks/F_Qf1tHt.js";
import { F as cr } from "../chunks/CfHEvq46.js";
import { B as fr } from "../chunks/D3-OgAkl.js";
import { R as mr } from "../chunks/BsfuR8cO.js";
var _r = c('<img alt="Client Logo" width="100%" height="100%">'), hr = c('<div class="logo svelte-1gwy16z"><!></div>');
function gr(Y, x) {
  let D = s(false), d = O(() => x.updated ? `/auth/v1/clients/${x.clientId}/logo?updated=${x.updated}` : `/auth/v1/clients/${x.clientId}/logo`);
  var R = hr(), j = v(R);
  {
    var g = (I) => {
      mr(I, { width: "100%" });
    }, B = (I) => {
      var p = _r();
      P(() => H(p, "src", e(d))), Wt("error", p, () => t(D, true)), Ot(p), i(I, p);
    };
    h(j, (I) => {
      e(D) || x.updated === void 0 ? I(g) : I(B, false);
    });
  }
  u(R), i(Y, R);
}
var pr = Dt(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125
            1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504
            1.125-1.125V9.75M8.25 21h8.25"></path></svg>`);
function wr(Y, x) {
  let D = Ue(x, "opacity", 3, 0.9), d = Ue(x, "width", 3, "1.5rem"), R = Ue(x, "color", 3, "currentColor");
  var j = pr();
  H(j, "stroke-width", 2), P(() => {
    H(j, "stroke", R()), H(j, "width", d()), H(j, "opacity", D());
  }), i(Y, j);
}
var br = c('<a class="home svelte-j0vmae" aria-label="Client Home Page"><!></a>'), Pr = c('<div class="forgotten svelte-j0vmae"><!></div>'), Lr = c("<!> <!>", 1), xr = c('<div class="btn flex-col svelte-j0vmae"><!></div>'), Ir = c('<div class="btn flex-col svelte-j0vmae"><!></div>'), Tr = c("<div><!></div> <!> <!>", 1), yr = c('<a class="reg svelte-j0vmae" target="_blank"> </a>'), Rr = c('<a class="reg svelte-j0vmae" href="/auth/v1/users/register" target="_blank"> </a>'), jr = c("<!> <!>", 1), $r = c('<div class="errMsg svelte-j0vmae"> </div>'), kr = c('<div class="success svelte-j0vmae"> </div>'), zr = c('<div class="btn flex-col svelte-j0vmae"><!></div>'), Er = c('<div class="container svelte-j0vmae"><div class="head svelte-j0vmae"><!> <!></div> <div class="name svelte-j0vmae"><h2> </h2></div> <!> <!> <!> <!> <!> <div class="providers flex-col svelte-j0vmae"><div class="providersSeparator svelte-j0vmae"><div class="separator svelte-j0vmae"></div> <div class="loginWith svelte-j0vmae"><div class="svelte-j0vmae"> </div></div></div> <!></div></div> <!> <!>', 1), Mr = c('<div class="outer svelte-j0vmae"><!></div>'), Sr = c("<!> <!> <!> <!> <!> <!> <!> <!>", 1);
function la(Y, x) {
  var _a;
  At(x, true);
  const D = "18rem";
  let d = dr(), R = vt().get(), j = O(() => R ? "/auth/v1/dev/authorize" : "/auth/v1/oidc/authorize"), g = C("client_id").get(), B = s(""), I = s(-1), p = s(Se(R ? "/auth/v1" : "")), G = C("redirect_uri").get(), he = C("nonce").get(), ge = ((_a = C("scope").get()) == null ? void 0 : _a.split(" ")) || [], pe = s(void 0), we = s(void 0), be = C("state").get(), Z = C("code_challenge").get(), $ = C("code_challenge_method").get(), Ce = false, Pe = s(void 0), Le = s(Se([])), se = s(void 0), V = s(false), f = s(""), ee = s(""), ie = s(""), z = s(false), K = s(false), le = s(false), ne = s(false), de = s(false), A = s(false), De = s(""), xe = s(false), T = s(Se(C("login_hint").get() || "")), E = s(""), Ie = s(""), Ae = O(() => e(z) && e(Pe) !== e(T) && !e(le));
  Nt(() => {
    var _a2;
    e(z) || ((_a2 = e(pe)) == null ? void 0 : _a2.focus());
  }), ae(() => {
    Ce && (g == null ? void 0 : g.length) && ft();
  }), ae(() => {
    e(de) && setTimeout(() => {
      t(de, false), t(le, false), t(ne, false);
    }, 3e3);
  }), ae(() => {
    var _a2;
    (_a2 = e(we)) == null ? void 0 : _a2.focus();
  }), ae(() => {
    var _a2;
    if (e(ee) === "Refresh") Ce = true;
    else if ((_a2 = e(ee)) == null ? void 0 : _a2.startsWith("MfaLogin ")) {
      let r = e(ee).replace("MfaLogin ", "");
      t(T, r, true), t(Pe, r, true);
    }
  }), ae(() => {
    R ? ut() : e(ie) && dt(e(ie));
  });
  async function ut() {
    var _a2;
    let r = await oe("/auth/v1/oidc/session");
    ((_a2 = r.body) == null ? void 0 : _a2.csrf_token) ? dt(r.body.csrf_token) : console.error(r.error);
  }
  function ct() {
    t(f, ""), t(le, true), t(E, "");
  }
  async function ft() {
    if (!g) {
      console.error("clientId is undefined");
      return;
    }
    if (!G) {
      console.error("redirectUri is undefined");
      return;
    }
    t(V, true);
    const r = { client_id: g, redirect_uri: G, state: be, nonce: he, scopes: ge };
    Z && $ && ($ === "plain" || $ === "S256") && (r.code_challenge = Z, r.code_challenge_method = $);
    let n = await oe("/auth/v1/oidc/authorize/refresh", r);
    await Ne(n);
  }
  async function mt(r, n) {
    if (t(f, ""), !g) {
      console.error("clientId is undefined");
      return;
    }
    if (!G) {
      console.error("redirectUri is undefined");
      return;
    }
    const m = { email: e(T), client_id: g, redirect_uri: G, state: be, nonce: he, scopes: ge };
    if (Z && $ && ($ === "plain" || $ === "S256") && (m.code_challenge = Z, m.code_challenge_method = $), e(z) && e(T) !== e(Pe)) {
      if (!e(E)) {
        t(f, d.authorize.passwordRequired, true);
        return;
      }
      if (e(E).length > 256) {
        t(f, "max 256");
        return;
      }
      m.password = e(E);
    }
    t(V, true);
    let w = "/auth/v1/oidc/authorize";
    vt().get() && (w = "/auth/v1/dev/authorize");
    let M = await oe(w, m, "json", "noRedirect");
    await Ne(M);
  }
  async function Ne(r) {
    if (r.status === 202) {
      let n = r.headers.get("location");
      if (!n) {
        console.error("location header missing");
        return;
      }
      window.location.replace(n);
    } else if (r.status === 200) {
      t(f, "");
      let n = r.body;
      n && "user_id" in n && "code" in n ? (t(Ie, n.user_id, true), t(se, { Login: n.code }, true)) : console.error("did not receive a proper WebauthnLoginResponse after HTTP200");
    } else if (r.status === 406) t(f, d.authorize.clientForceMfa, true), t(K, true);
    else if (r.status === 429) {
      let n = r.headers.get("x-retry-not-before");
      if (!n) {
        console.error("x-retry-not-before header missing");
        return;
      }
      let m = Number.parseInt(n), w = er(m), M = m * 1e3 - (/* @__PURE__ */ new Date()).getTime();
      t(A, true), t(f, `${d.authorize.http429} ${w}`), t(T, ""), t(E, ""), t(z, false), setTimeout(() => {
        t(A, false), t(f, "");
      }, M);
    } else e(z) ? (t(f, d.authorize.invalidCredentials, true), t(ne, true)) : (t(z, true), t(De, e(T), true));
    t(V, false);
  }
  function _t() {
    e(z) && e(De) !== e(T) && (t(z, false), t(E, ""), t(f, ""));
  }
  function ht(r) {
    tr(64, (n, { challenge: m, verifier: w }) => {
      n || (localStorage.setItem(rr, w), gt(r, m));
    });
  }
  async function gt(r, n) {
    var _a2;
    if (!g) {
      console.error("clientId is undefined");
      return;
    }
    if (!G) {
      console.error("redirectUri is undefined");
      return;
    }
    let m = { email: e(T) || void 0, client_id: g, redirect_uri: G, scopes: ge, state: be, nonce: he, code_challenge: Z, code_challenge_method: $, provider_id: r, pkce_challenge: n }, w = await oe("/auth/v1/providers/login", m);
    if (w.text) {
      ar(w.text);
      let M = w.headers.get("location");
      if (!M) {
        console.error("no location header set for provider login");
        return;
      }
      window.location.href = M;
    } else t(f, ((_a2 = w.error) == null ? void 0 : _a2.message) || "Error", true);
  }
  function pt(r) {
    t(se, void 0), t(f, r, true);
  }
  function wt(r) {
    r && "loc" in r && window.location.replace(r.loc);
  }
  async function bt() {
    let r = { email: e(T) };
    e(p) && (r.redirect_uri = encodeURI(e(p))), t(V, true);
    let n = await oe("/auth/v1/users/request_reset", r);
    n.error ? t(f, n.error.message, true) : t(de, true), t(V, false);
  }
  var qe = Sr();
  Ht((r) => {
    P(() => Ft.title = `Login ${(e(B) || g) ?? ""}`);
  });
  var Fe = F(qe);
  W(Fe, { id: Vt, get value() {
    return e(Le);
  }, set value(r) {
    t(Le, r, true);
  } });
  var We = l(Fe, 2);
  W(We, { id: Kt, get value() {
    return e(B);
  }, set value(r) {
    t(B, r, true);
  } });
  var Oe = l(We, 2);
  W(Oe, { id: Jt, get value() {
    return e(p);
  }, set value(r) {
    t(p, r, true);
  } });
  var He = l(Oe, 2);
  W(He, { id: Qt, get value() {
    return e(I);
  }, set value(r) {
    t(I, r, true);
  } });
  var Be = l(He, 2);
  W(Be, { id: Xt, get value() {
    return e(ie);
  }, set value(r) {
    t(ie, r, true);
  } });
  var Ge = l(Be, 2);
  W(Ge, { id: Yt, get value() {
    return e(ee);
  }, set value(r) {
    t(ee, r, true);
  } });
  var Ve = l(Ge, 2);
  W(Ve, { id: Zt, get value() {
    return e(xe);
  }, set value(r) {
    t(xe, r, true);
  } });
  var Pt = l(Ve, 2);
  lr(Pt, { children: (r, n) => {
    var m = Mr(), w = v(m);
    nr(w, { children: (M, Ur) => {
      var Ke = Er(), Te = F(Ke), ye = v(Te), Je = v(ye);
      {
        var Lt = (a) => {
          const o = O(() => e(I) > -1 ? e(I) : void 0);
          gr(a, { clientId: g, get updated() {
            return e(o);
          } });
        };
        h(Je, (a) => {
          g && a(Lt);
        });
      }
      var xt = l(Je, 2);
      {
        var It = (a) => {
          var o = br(), L = v(o);
          wr(L, { color: "hsla(var(--text) / .4)" }), u(o), P(() => H(o, "href", e(p))), i(a, o);
        };
        h(xt, (a) => {
          e(p) && a(It);
        });
      }
      u(ye);
      var Re = l(ye, 2), Qe = v(Re), Tt = v(Qe, true);
      u(Qe), u(Re);
      var Xe = l(Re, 2);
      {
        var yt = (a) => {
          or(a, { get userId() {
            return e(Ie);
          }, get purpose() {
            return e(se);
          }, onSuccess: wt, onError: pt });
        };
        h(Xe, (a) => {
          e(se) && e(Ie) && a(yt);
        });
      }
      var Ye = l(Xe, 2);
      {
        var Rt = (a) => {
          var o = jr(), L = F(o);
          cr(L, { get action() {
            return e(j);
          }, onSubmit: mt, children: (N, ke) => {
            var ve = Tr(), J = F(ve);
            let ue;
            var S = v(J);
            const y = O(() => e(A) || e(K));
            sr(S, { typ: "email", name: "email", autocomplete: "email", get label() {
              return d.common.email;
            }, get placeholder() {
              return d.common.email;
            }, get errMsg() {
              return d.authorize.validEmail;
            }, get disabled() {
              return e(y);
            }, onInput: _t, width: D, required: true, get ref() {
              return e(pe);
            }, set ref(b) {
              t(pe, b, true);
            }, get value() {
              return e(T);
            }, set value(b) {
              t(T, b, true);
            } }), u(J);
            var Q = l(J, 2);
            {
              var St = (b) => {
                var te = Lr(), ce = F(te);
                const ze = O(() => e(A) || e(K));
                ur(ce, { name: "password", autocomplete: "current-password", get label() {
                  return d.common.password;
                }, get placeholder() {
                  return d.common.password;
                }, maxLength: 256, get disabled() {
                  return e(ze);
                }, width: D, required: true, get ref() {
                  return e(we);
                }, set ref(_) {
                  t(we, _, true);
                }, get value() {
                  return e(E);
                }, set value(_) {
                  t(E, _, true);
                } });
                var Ee = l(ce, 2);
                {
                  var U = (_) => {
                    var q = Pr(), re = v(q);
                    _e(re, { invisible: true, onclick: ct, children: (Me, X) => {
                      me();
                      var lt = fe();
                      P(() => k(lt, d.authorize.passwordForgotten)), i(Me, lt);
                    }, $$slots: { default: true } }), u(q), i(_, q);
                  };
                  h(Ee, (_) => {
                    e(ne) && !e(A) && _(U);
                  });
                }
                i(b, te);
              };
              h(Q, (b) => {
                e(Ae) && b(St);
              });
            }
            var Ut = l(Q, 2);
            {
              var Ct = (b) => {
                var te = nt(), ce = F(te);
                {
                  var ze = (U) => {
                    var _ = xr(), q = v(_);
                    _e(q, { onclick: bt, children: (re, Me) => {
                      me();
                      var X = fe();
                      P(() => k(X, d.authorize.passwordRequest)), i(re, X);
                    }, $$slots: { default: true } }), u(_), i(U, _);
                  }, Ee = (U) => {
                    var _ = Ir(), q = v(_);
                    _e(q, { type: "submit", get isLoading() {
                      return e(V);
                    }, children: (re, Me) => {
                      me();
                      var X = fe();
                      P(() => k(X, d.authorize.login)), i(re, X);
                    }, $$slots: { default: true } }), u(_), i(U, _);
                  };
                  h(ce, (U) => {
                    e(le) ? U(ze) : U(Ee, false);
                  });
                }
                i(b, te);
              };
              h(Ut, (b) => {
                !e(A) && !e(K) && b(Ct);
              });
            }
            P((b) => ue = Gt(J, 1, "svelte-j0vmae", null, ue, b), [() => ({ emailMinHeight: !e(Ae) })]), i(N, ve);
          }, $$slots: { default: true } });
          var $e = l(L, 2);
          {
            var it = (N) => {
              var ke = nt(), ve = F(ke);
              {
                var J = (S) => {
                  var y = yr(), Q = v(y, true);
                  u(y), P(() => {
                    H(y, "href", `/auth/v1/users/register?redirect_uri=${e(p) ?? ""}`), k(Q, d.authorize.signUp);
                  }), i(S, y);
                }, ue = (S) => {
                  var y = Rr(), Q = v(y, true);
                  u(y), P(() => k(Q, d.authorize.signUp)), i(S, y);
                };
                h(ve, (S) => {
                  e(p) ? S(J) : S(ue, false);
                });
              }
              i(N, ke);
            };
            h($e, (N) => {
              e(xe) && !e(ne) && !e(A) && N(it);
            });
          }
          i(a, o);
        };
        h(Ye, (a) => {
          e(K) || a(Rt);
        });
      }
      var Ze = l(Ye, 2);
      {
        var jt = (a) => {
          var o = $r(), L = v(o, true);
          u(o), P(() => k(L, e(f))), i(a, o);
        };
        h(Ze, (a) => {
          e(f) && a(jt);
        });
      }
      var et = l(Ze, 2);
      {
        var $t = (a) => {
          var o = kr(), L = v(o, true);
          u(o), P(() => k(L, d.authorize.emailSentMsg)), i(a, o);
        };
        h(et, (a) => {
          e(de) && a($t);
        });
      }
      var tt = l(et, 2);
      {
        var kt = (a) => {
          var o = zr(), L = v(o);
          _e(L, { onclick: () => window.location.href = "/auth/v1/account", children: ($e, it) => {
            me();
            var N = fe("Account");
            i($e, N);
          }, $$slots: { default: true } }), u(o), i(a, o);
        };
        h(tt, (a) => {
          e(K) && a(kt);
        });
      }
      var rt = l(tt, 2), je = v(rt), at = l(v(je), 2), ot = v(at), zt = v(ot, true);
      u(ot), u(at), u(je);
      var Et = l(je, 2);
      Bt(Et, 17, () => e(Le), (a) => a.id, (a, o) => {
        const L = O(() => `Login: ${e(o).name}`);
        fr(a, { get ariaLabel() {
          return e(L);
        }, get provider() {
          return e(o);
        }, onclick: ht });
      }), u(rt), u(Te);
      var st = l(Te, 2);
      vr(st, { absolute: true });
      var Mt = l(st, 2);
      ir(Mt, { absolute: true }), P(() => {
        k(Tt, e(B) || g), k(zt, d.authorize.orLoginWith);
      }), i(M, Ke);
    } }), u(m), i(r, m);
  } }), i(Y, qe), qt();
}
export {
  la as component
};
