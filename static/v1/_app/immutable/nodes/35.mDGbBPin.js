import { t as _, a, d as Pe, e as k } from "../chunks/DLBGyKVC.js";
import { p as dr, aa as le, f as I, a as pr, l as r, k as p, j as e, s as o, a7 as Re, c, a8 as T, a9 as se, r as v, t as y } from "../chunks/CmQi0fbH.js";
import { h as ur, s as u } from "../chunks/BjaYyaa_.js";
import { i as f } from "../chunks/C6bK2EJJ.js";
import { B as D, t as Ve, s as Fe } from "../chunks/DPLO-ozG.js";
import { p as l } from "../chunks/B_ggA-0N.js";
import { y as cr, z as vr } from "../chunks/B21bTIl7.js";
import { P as fr } from "../chunks/C37IM9Kl.js";
import { I as _r } from "../chunks/DwPr_s7h.js";
import { W as mr } from "../chunks/0sZfVpOr.js";
import { L as wr } from "../chunks/DJEyp40I.js";
import { u as gr } from "../chunks/DGTOa5g8.js";
import { M as hr } from "../chunks/Cf3VYlYr.js";
import { C as $r } from "../chunks/CyYIiz5-.js";
import { T as yr } from "../chunks/CLEj1OQz.js";
import { u as xr } from "../chunks/BctQMYGV.js";
import { T as br } from "../chunks/DB8eOlLu.js";
import { I as Me } from "../chunks/Clnxc71_.js";
import { A as Ue } from "../chunks/D3vXntHU.js";
import { u as Pr } from "../chunks/Dd1y69dN.js";
import { w as Rr } from "../chunks/CcWMq3Bz.js";
import { F as ze } from "../chunks/CNMQp9ma.js";
import { b as kr } from "../chunks/gfDO7tLr.js";
var Tr = _('<!> <!> <!> <div class="generate svelte-1w50bpj"><!></div> <!>', 1), Ar = _('<div class="dev svelte-1w50bpj"><p>This window shows up during local dev,<br> only to be able to switch modes easily.<br></p> <!> <!></div>'), Lr = _("<p> <br> <br> <br> <!></p>"), Nr = _("<div><!></div>"), Sr = _("<!> <!>", 1), Cr = _('<div class="success"><p> </p> <p> </p> <!></div>'), Er = _("<div><!> <!></div>"), Ir = _('<h1> </h1> <p> </p> <p> <!></p> <div class="typeChoice svelte-1w50bpj"><!> <!></div> <!>', 1), Wr = _("<h1>Password Reset</h1> <!>", 1), jr = _('<div class="err svelte-1w50bpj"> </div>'), qr = _('<div class="container"><!> <!></div>'), Dr = _("<!> <!> <!>", 1), Vr = _("<!> <!> <!> <!>", 1);
function dt(Oe, Be) {
  dr(Be, true);
  const ke = (s) => {
    var m = Pe(), w = I(m);
    {
      var W = (g) => {
        ze(g, { action: "", onSubmit: He, children: (h, L) => {
          var F = Tr(), B = I(F);
          fr(B, { get policy() {
            return e(i).password_policy;
          }, get password() {
            return e(A);
          }, get accepted() {
            return e(ce);
          }, set accepted(n) {
            r(ce, l(n));
          } });
          var J = o(B, 2);
          const N = se(() => e(A).length >= e(i).password_policy.length_min);
          Me(J, { autocomplete: "new-password", get label() {
            return t.account.passwordNew;
          }, get placeholder() {
            return t.account.passwordNew;
          }, get maxLength() {
            return e(i).password_policy.length_max;
          }, required: true, get showCopy() {
            return e(N);
          }, width: de, get ref() {
            return e(ue);
          }, set ref(n) {
            r(ue, l(n));
          }, get value() {
            return e(A);
          }, set value(n) {
            r(A, l(n));
          }, get reportValidity() {
            return e(ve);
          }, set reportValidity(n) {
            r(ve, l(n));
          } });
          var d = o(J, 2);
          Me(d, { autocomplete: "new-password", get label() {
            return t.account.passwordConfirm;
          }, get placeholder() {
            return t.account.passwordConfirm;
          }, get maxLength() {
            return e(i).password_policy.length_max;
          }, required: true, width: de, get value() {
            return e(Z);
          }, set value(n) {
            r(Z, l(n));
          }, get reportValidity() {
            return e(fe);
          }, set reportValidity(n) {
            r(fe, l(n));
          } });
          var b = o(d, 2), S = c(b);
          D(S, { level: 2, onclick: Ke, children: (n, K) => {
            T();
            var C = k();
            y(() => u(C, t.passwordReset.generate)), a(n, C);
          }, $$slots: { default: true } }), v(b);
          var j = o(b, 2);
          D(j, { type: "submit", get isLoading() {
            return e(Q);
          }, children: (n, K) => {
            T();
            var C = k();
            y(() => u(C, t.common.save)), a(n, C);
          }, $$slots: { default: true } }), a(h, F);
        }, $$slots: { default: true } });
      };
      f(w, (g) => {
        e(i) && g(W);
      });
    }
    a(s, m);
  }, de = "20rem";
  let t = gr(), Je = Pr().get(), i = p(void 0), pe = p(void 0), ue = p(void 0), Q = p(false), x = p(""), z = xr("type", "password_reset"), V = p(""), ae = p(""), O = p(false), ce = p(false), X = p(void 0), Y = p(""), A = p(""), Z = p(""), ve = p(void 0), fe = p(void 0);
  le(() => {
    e(V) && _e();
  }), le(() => {
    var _a;
    (_a = e(pe)) == null ? void 0 : _a.focus();
  }), le(() => {
    var _a;
    (_a = e(ue)) == null ? void 0 : _a.focus();
  }), le(() => {
    e(O) && setTimeout(() => {
      e(ae) ? window.location.replace(e(ae)) : Te();
    }, 5e3);
  });
  function Te() {
    window.location.replace("/auth/v1/account");
  }
  function Ke() {
    if (e(i)) {
      let s = vr(e(i).password_policy);
      r(A, l(s)), r(Z, l(s)), requestAnimationFrame(() => {
        var _a, _b;
        (_a = e(ve)) == null ? void 0 : _a(), (_b = e(fe)) == null ? void 0 : _b();
      });
    }
  }
  async function Ge() {
    if (!e(i)) {
      console.error("template data is undefined");
      return;
    }
    if (r(x, ""), e(Y).length < 1) {
      r(x, l(t.mfa.passkeyNameErr));
      return;
    }
    let s = await Rr(e(i).user_id, e(Y), t.authorize.invalidKeyUsed, t.authorize.requestExpired, e(i).magic_link_id, e(i).csrf_token);
    s.error ? r(x, `${t.mfa.errorReg} - ${s.error}`) : (_e(), r(O, true));
  }
  async function He() {
    var _a;
    if (r(x, ""), !!e(ce)) {
      if (e(A) !== e(Z)) {
        r(x, l(t.passwordReset.passwordNoMatch));
        return;
      }
      if (e(A).length > 256) {
        r(x, "max 256");
        return;
      }
      ((_a = e(i)) == null ? void 0 : _a.needs_mfa) ? r(X, "PasswordReset") : await Ae();
    }
  }
  async function Ae(s) {
    var _a;
    if (!e(i)) return;
    r(Q, true);
    const m = { password: e(A), magic_link_id: e(i).magic_link_id, mfa_code: s };
    let w = await fetch(`/auth/v1/users/${e(i).user_id}/reset`, { method: "PUT", headers: { "content-type": "application/json", "x-pwd-csrf-token": (_a = e(i)) == null ? void 0 : _a.csrf_token }, body: JSON.stringify(m) });
    if (w.ok) _e(), r(ae, l(w.headers.get("location") || "/auth/v1/account")), r(O, true);
    else {
      const W = await w.json();
      r(x, l(W.message));
    }
    r(Q, false);
  }
  function Qe(s) {
    r(x, l(s)), r(X, void 0);
  }
  function Xe(s) {
    r(X, void 0), s && "code" in s ? Ae(s.code) : console.error("invalid webauthn response", s);
  }
  function _e() {
    r(x, ""), r(Y, ""), r(A, ""), r(Z, "");
  }
  var Le = Vr();
  ur((s) => {
    var m = Pe(), w = I(m);
    {
      var W = (h) => {
        var L = Pe(), F = I(L);
        {
          var B = (N) => {
            y(() => Re.title = t.passwordReset.newAccount);
          }, J = (N, d) => {
            {
              var b = (S) => {
                y(() => Re.title = t.passwordReset.passwordReset);
              };
              f(N, (S) => {
                z.get() === "password_reset" && S(b);
              }, d);
            }
          };
          f(F, (N) => {
            var _a;
            ((_a = z.get()) == null ? void 0 : _a.startsWith("new_user")) ? N(B) : N(J, false);
          });
        }
        a(h, L);
      }, g = (h) => {
        Re.title = "Password";
      };
      f(w, (h) => {
        t ? h(W) : h(g, false);
      });
    }
    a(s, m);
  });
  var Ne = I(Le);
  yr(Ne, { id: cr, get value() {
    return e(i);
  }, set value(s) {
    r(i, l(s));
  } });
  var Se = o(Ne, 2);
  {
    var Ye = (s) => {
      var m = Ar(), w = o(c(m), 2);
      D(w, { level: 2, onclick: () => z.set("new_user"), children: (g, h) => {
        T();
        var L = k("new_user");
        a(g, L);
      }, $$slots: { default: true } });
      var W = o(w, 2);
      D(W, { level: 2, onclick: () => z.set("password_reset"), children: (g, h) => {
        T();
        var L = k("password_reset");
        a(g, L);
      }, $$slots: { default: true } }), v(m), a(s, m);
    };
    f(Se, (s) => {
      Je && s(Ye);
    });
  }
  var Ce = o(Se, 2);
  hr(Ce, { children: (s, m) => {
    $r(s, { children: (w, W) => {
      var g = Dr(), h = I(g);
      {
        var L = (d) => {
          mr(d, { get userId() {
            return e(i).user_id;
          }, get purpose() {
            return e(X);
          }, onSuccess: Xe, onError: Qe });
        };
        f(h, (d) => {
          e(X) && e(i) && d(L);
        });
      }
      var F = o(h, 2);
      {
        var B = (d) => {
          var b = Lr(), S = c(b), j = o(S, 2), n = o(j, 4), K = o(n);
          const C = se(() => e(ae) || "/auth/v1/account");
          Ue(K, { get href() {
            return e(C);
          }, children: (me, Ee) => {
            T();
            var we = k("Account");
            a(me, we);
          }, $$slots: { default: true } }), v(b), y(() => {
            u(S, `${t.passwordReset.success1 ?? ""} `), u(j, ` ${t.passwordReset.success2 ?? ""} `), u(n, ` ${t.passwordReset.success3 ?? ""} `);
          }), a(d, b);
        }, J = (d, b) => {
          {
            var S = (j) => {
              var n = qr(), K = c(n);
              {
                var C = (R) => {
                  var q = Ir(), M = I(q), ee = c(M, true);
                  v(M);
                  var G = o(M, 2), ge = c(G, true);
                  v(G);
                  var he = o(G, 2), Ie = c(he), er = o(Ie);
                  Ue(er, { get href() {
                    return t.passwordReset.fidoLink;
                  }, target: "_blank", children: ($, E) => {
                    T();
                    var P = k("FIDO Alliance");
                    a($, P);
                  }, $$slots: { default: true } }), v(he);
                  var $e = o(he, 2), We = c($e);
                  const rr = se(() => e(V) ? 3 : 1);
                  D(We, { get level() {
                    return e(rr);
                  }, onclick: () => r(V, "passkey"), get isLoading() {
                    return e(Q);
                  }, children: ($, E) => {
                    T();
                    var P = k();
                    y(() => u(P, t.passwordReset.passwordless)), a($, P);
                  }, $$slots: { default: true } });
                  var tr = o(We, 2);
                  const sr = se(() => e(V) ? 3 : 2);
                  D(tr, { get level() {
                    return e(sr);
                  }, onclick: () => r(V, "password"), get isLoading() {
                    return e(Q);
                  }, children: ($, E) => {
                    T();
                    var P = k();
                    y(() => u(P, t.passwordReset.password)), a($, P);
                  }, $$slots: { default: true } }), v($e);
                  var ar = o($e, 2);
                  {
                    var or = ($) => {
                      var E = Nr(), P = c(E);
                      ke(P), v(E), Ve(3, E, () => Fe), a($, E);
                    }, nr = ($, E) => {
                      {
                        var P = (ye) => {
                          var oe = Er(), je = c(oe);
                          ze(je, { action: "", onSubmit: Ge, children: (re, ne) => {
                            var H = Sr(), ie = I(H);
                            _r(ie, { autocomplete: "off", get label() {
                              return t.mfa.passkeyName;
                            }, get placeholder() {
                              return t.mfa.passkeyName;
                            }, width: de, maxLength: 32, pattern: kr, required: true, get ref() {
                              return e(pe);
                            }, set ref(U) {
                              r(pe, l(U));
                            }, get value() {
                              return e(Y);
                            }, set value(U) {
                              r(Y, l(U));
                            } });
                            var te = o(ie, 2);
                            const xe = se(() => e(O) ? 2 : 1);
                            D(te, { type: "submit", get level() {
                              return e(xe);
                            }, children: (U, qe) => {
                              T();
                              var be = k();
                              y(() => u(be, t.mfa.register)), a(U, be);
                            }, $$slots: { default: true } }), a(re, H);
                          }, $$slots: { default: true } });
                          var ir = o(je, 2);
                          {
                            var lr = (re) => {
                              var ne = Cr(), H = c(ne), ie = c(H, true);
                              v(H);
                              var te = o(H, 2), xe = c(te, true);
                              v(te);
                              var U = o(te, 2);
                              D(U, { onclick: Te, children: (qe, be) => {
                                T();
                                var De = k();
                                y(() => u(De, t.passwordReset.accountLogin)), a(qe, De);
                              }, $$slots: { default: true } }), v(ne), y(() => {
                                u(ie, t.passwordReset.successPasskey1), u(xe, t.passwordReset.successPasskey2);
                              }), a(re, ne);
                            };
                            f(ir, (re) => {
                              e(O) && re(lr);
                            });
                          }
                          v(oe), Ve(3, oe, () => Fe), a(ye, oe);
                        };
                        f($, (ye) => {
                          e(V) === "passkey" && ye(P);
                        }, E);
                      }
                    };
                    f(ar, ($) => {
                      e(V) === "password" ? $(or) : $(nr, false);
                    });
                  }
                  y(() => {
                    u(ee, t.passwordReset.newAccount), u(ge, t.passwordReset.newAccDesc1), u(Ie, `${t.passwordReset.newAccDesc2 ?? ""} `);
                  }), a(R, q);
                }, me = (R, q) => {
                  {
                    var M = (ee) => {
                      var G = Wr(), ge = o(I(G), 2);
                      ke(ge), a(ee, G);
                    };
                    f(R, (ee) => {
                      var _a;
                      ((_a = z.get()) == null ? void 0 : _a.startsWith("password_reset")) && ee(M);
                    }, q);
                  }
                };
                f(K, (R) => {
                  var _a;
                  ((_a = z.get()) == null ? void 0 : _a.startsWith("new_user")) ? R(C) : R(me, false);
                });
              }
              var Ee = o(K, 2);
              {
                var we = (R) => {
                  var q = jr(), M = c(q, true);
                  v(q), y(() => u(M, e(x))), a(R, q);
                };
                f(Ee, (R) => {
                  e(x) && R(we);
                });
              }
              v(n), a(j, n);
            };
            f(d, (j) => {
              e(i) && j(S);
            }, b);
          }
        };
        f(F, (d) => {
          e(O) ? d(B) : d(J, false);
        });
      }
      var N = o(F, 2);
      br(N, { absolute: true }), a(w, g);
    } });
  } });
  var Ze = o(Ce, 2);
  wr(Ze, { absolute: true }), a(Oe, Le), pr();
}
export {
  dt as component
};
