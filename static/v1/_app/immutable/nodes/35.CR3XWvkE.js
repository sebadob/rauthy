import { t as f, a, d as be, e as R } from "../chunks/BxmJRzoY.js";
import { p as lr, j as d, a5 as ie, f as E, a as dr, l as r, k as e, s as o, a2 as Pe, c as p, a4 as te, a3 as k, r as c, t as $ } from "../chunks/w0HvPX0p.js";
import { h as ur, s as u } from "../chunks/BzP2S3Z_.js";
import { i as v } from "../chunks/iO9_dPNE.js";
import { B as q, t as De, s as Ve } from "../chunks/C8YTstTD.js";
import { y as pr, z as cr } from "../chunks/B21bTIl7.js";
import { P as vr } from "../chunks/DbyBiYcN.js";
import { I as fr } from "../chunks/Q4PIg3iI.js";
import { W as _r } from "../chunks/Be9Mus3q.js";
import { L as mr } from "../chunks/D9_8lD-s.js";
import { u as wr } from "../chunks/0cG6LBdy.js";
import { M as gr } from "../chunks/DKM0QPz5.js";
import { C as hr } from "../chunks/QNragXLc.js";
import { T as $r } from "../chunks/BdDmaO9S.js";
import { u as yr } from "../chunks/Cy3hLAXJ.js";
import { T as xr } from "../chunks/B1f0afjj.js";
import { I as Fe } from "../chunks/iedauS3r.js";
import { A as Me } from "../chunks/D7WltQR7.js";
import { u as br } from "../chunks/F_Qf1tHt.js";
import { w as Pr } from "../chunks/lIqoVohd.js";
import { F as Ue } from "../chunks/CDe-qvZi.js";
import { b as Rr } from "../chunks/gfDO7tLr.js";
var kr = f('<!> <!> <!> <div class="generate svelte-1w50bpj"><!></div> <!>', 1), Tr = f('<div class="dev svelte-1w50bpj"><p>This window shows up during local dev,<br> only to be able to switch modes easily.<br></p> <!> <!></div>'), Ar = f("<p> <br> <br> <br> <!></p>"), Lr = f("<div><!></div>"), Nr = f("<!> <!>", 1), Sr = f('<div class="success"><p> </p> <p> </p> <!></div>'), Cr = f("<div><!> <!></div>"), Er = f('<h1> </h1> <p> </p> <p> <!></p> <div class="typeChoice svelte-1w50bpj"><!> <!></div> <!>', 1), Ir = f("<h1>Password Reset</h1> <!>", 1), Wr = f('<div class="err svelte-1w50bpj"> </div>'), jr = f('<div class="container"><!> <!></div>'), qr = f("<!> <!> <!>", 1), Dr = f("<!> <!> <!> <!>", 1);
function it(ze, Oe) {
  lr(Oe, true);
  const Re = (s) => {
    var _ = be(), m = E(_);
    {
      var I = (w) => {
        Ue(w, { action: "", onSubmit: Ge, children: (g, A) => {
          var V = kr(), O = E(V);
          vr(O, { get policy() {
            return e(i).password_policy;
          }, get password() {
            return e(T);
          }, get accepted() {
            return e(pe);
          }, set accepted(n) {
            r(pe, n, true);
          } });
          var B = o(O, 2);
          const L = te(() => e(T).length >= e(i).password_policy.length_min);
          Fe(B, { autocomplete: "new-password", get label() {
            return t.account.passwordNew;
          }, get placeholder() {
            return t.account.passwordNew;
          }, get maxLength() {
            return e(i).password_policy.length_max;
          }, required: true, get showCopy() {
            return e(L);
          }, width: le, get ref() {
            return e(ue);
          }, set ref(n) {
            r(ue, n, true);
          }, get value() {
            return e(T);
          }, set value(n) {
            r(T, n, true);
          }, get reportValidity() {
            return e(ce);
          }, set reportValidity(n) {
            r(ce, n, true);
          } });
          var l = o(B, 2);
          Fe(l, { autocomplete: "new-password", get label() {
            return t.account.passwordConfirm;
          }, get placeholder() {
            return t.account.passwordConfirm;
          }, get maxLength() {
            return e(i).password_policy.length_max;
          }, required: true, width: le, get value() {
            return e(Y);
          }, set value(n) {
            r(Y, n, true);
          }, get reportValidity() {
            return e(ve);
          }, set reportValidity(n) {
            r(ve, n, true);
          } });
          var x = o(l, 2), N = p(x);
          q(N, { level: 2, onclick: Je, children: (n, J) => {
            k();
            var S = R();
            $(() => u(S, t.passwordReset.generate)), a(n, S);
          }, $$slots: { default: true } }), c(x);
          var W = o(x, 2);
          q(W, { type: "submit", get isLoading() {
            return e(H);
          }, children: (n, J) => {
            k();
            var S = R();
            $(() => u(S, t.common.save)), a(n, S);
          }, $$slots: { default: true } }), a(g, V);
        }, $$slots: { default: true } });
      };
      v(m, (w) => {
        e(i) && w(I);
      });
    }
    a(s, _);
  }, le = "20rem";
  let t = wr(), Be = br().get(), i = d(void 0), de = d(void 0), ue = d(void 0), H = d(false), y = d(""), U = yr("type", "password_reset"), D = d(""), se = d(""), z = d(false), pe = d(false), Q = d(void 0), X = d(""), T = d(""), Y = d(""), ce = d(void 0), ve = d(void 0);
  ie(() => {
    e(D) && fe();
  }), ie(() => {
    var _a;
    (_a = e(de)) == null ? void 0 : _a.focus();
  }), ie(() => {
    var _a;
    (_a = e(ue)) == null ? void 0 : _a.focus();
  }), ie(() => {
    e(z) && setTimeout(() => {
      e(se) ? window.location.replace(e(se)) : ke();
    }, 5e3);
  });
  function ke() {
    window.location.replace("/auth/v1/account");
  }
  function Je() {
    if (e(i)) {
      let s = cr(e(i).password_policy);
      r(T, s, true), r(Y, s, true), requestAnimationFrame(() => {
        var _a, _b;
        (_a = e(ce)) == null ? void 0 : _a(), (_b = e(ve)) == null ? void 0 : _b();
      });
    }
  }
  async function Ke() {
    if (!e(i)) {
      console.error("template data is undefined");
      return;
    }
    if (r(y, ""), e(X).length < 1) {
      r(y, t.mfa.passkeyNameErr, true);
      return;
    }
    let s = await Pr(e(i).user_id, e(X), t.authorize.invalidKeyUsed, t.authorize.requestExpired, e(i).magic_link_id, e(i).csrf_token);
    s.error ? r(y, `${t.mfa.errorReg} - ${s.error}`) : (fe(), r(z, true));
  }
  async function Ge() {
    var _a;
    if (r(y, ""), !!e(pe)) {
      if (e(T) !== e(Y)) {
        r(y, t.passwordReset.passwordNoMatch, true);
        return;
      }
      if (e(T).length > 256) {
        r(y, "max 256");
        return;
      }
      ((_a = e(i)) == null ? void 0 : _a.needs_mfa) ? r(Q, "PasswordReset") : await Te();
    }
  }
  async function Te(s) {
    var _a;
    if (!e(i)) return;
    r(H, true);
    const _ = { password: e(T), magic_link_id: e(i).magic_link_id, mfa_code: s };
    let m = await fetch(`/auth/v1/users/${e(i).user_id}/reset`, { method: "PUT", headers: { "content-type": "application/json", "x-pwd-csrf-token": (_a = e(i)) == null ? void 0 : _a.csrf_token }, body: JSON.stringify(_) });
    if (m.ok) fe(), r(se, m.headers.get("location") || "/auth/v1/account", true), r(z, true);
    else {
      const I = await m.json();
      r(y, I.message, true);
    }
    r(H, false);
  }
  function He(s) {
    r(y, s, true), r(Q, void 0);
  }
  function Qe(s) {
    r(Q, void 0), s && "code" in s ? Te(s.code) : console.error("invalid webauthn response", s);
  }
  function fe() {
    r(y, ""), r(X, ""), r(T, ""), r(Y, "");
  }
  var Ae = Dr();
  ur((s) => {
    var _ = be(), m = E(_);
    {
      var I = (g) => {
        var A = be(), V = E(A);
        {
          var O = (L) => {
            $(() => Pe.title = t.passwordReset.newAccount);
          }, B = (L, l) => {
            {
              var x = (N) => {
                $(() => Pe.title = t.passwordReset.passwordReset);
              };
              v(L, (N) => {
                U.get() === "password_reset" && N(x);
              }, l);
            }
          };
          v(V, (L) => {
            var _a;
            ((_a = U.get()) == null ? void 0 : _a.startsWith("new_user")) ? L(O) : L(B, false);
          });
        }
        a(g, A);
      }, w = (g) => {
        Pe.title = "Password";
      };
      v(m, (g) => {
        t ? g(I) : g(w, false);
      });
    }
    a(s, _);
  });
  var Le = E(Ae);
  $r(Le, { id: pr, get value() {
    return e(i);
  }, set value(s) {
    r(i, s, true);
  } });
  var Ne = o(Le, 2);
  {
    var Xe = (s) => {
      var _ = Tr(), m = o(p(_), 2);
      q(m, { level: 2, onclick: () => U.set("new_user"), children: (w, g) => {
        k();
        var A = R("new_user");
        a(w, A);
      }, $$slots: { default: true } });
      var I = o(m, 2);
      q(I, { level: 2, onclick: () => U.set("password_reset"), children: (w, g) => {
        k();
        var A = R("password_reset");
        a(w, A);
      }, $$slots: { default: true } }), c(_), a(s, _);
    };
    v(Ne, (s) => {
      Be && s(Xe);
    });
  }
  var Se = o(Ne, 2);
  gr(Se, { children: (s, _) => {
    hr(s, { children: (m, I) => {
      var w = qr(), g = E(w);
      {
        var A = (l) => {
          _r(l, { get userId() {
            return e(i).user_id;
          }, get purpose() {
            return e(Q);
          }, onSuccess: Qe, onError: He });
        };
        v(g, (l) => {
          e(Q) && e(i) && l(A);
        });
      }
      var V = o(g, 2);
      {
        var O = (l) => {
          var x = Ar(), N = p(x), W = o(N, 2), n = o(W, 4), J = o(n);
          const S = te(() => e(se) || "/auth/v1/account");
          Me(J, { get href() {
            return e(S);
          }, children: (_e, Ce) => {
            k();
            var me = R("Account");
            a(_e, me);
          }, $$slots: { default: true } }), c(x), $(() => {
            u(N, `${t.passwordReset.success1 ?? ""} `), u(W, ` ${t.passwordReset.success2 ?? ""} `), u(n, ` ${t.passwordReset.success3 ?? ""} `);
          }), a(l, x);
        }, B = (l, x) => {
          {
            var N = (W) => {
              var n = jr(), J = p(n);
              {
                var S = (P) => {
                  var j = Er(), F = E(j), Z = p(F, true);
                  c(F);
                  var K = o(F, 2), we = p(K, true);
                  c(K);
                  var ge = o(K, 2), Ee = p(ge), Ze = o(Ee);
                  Me(Ze, { get href() {
                    return t.passwordReset.fidoLink;
                  }, target: "_blank", children: (h, C) => {
                    k();
                    var b = R("FIDO Alliance");
                    a(h, b);
                  }, $$slots: { default: true } }), c(ge);
                  var he = o(ge, 2), Ie = p(he);
                  const er = te(() => e(D) ? 3 : 1);
                  q(Ie, { get level() {
                    return e(er);
                  }, onclick: () => r(D, "passkey"), get isLoading() {
                    return e(H);
                  }, children: (h, C) => {
                    k();
                    var b = R();
                    $(() => u(b, t.passwordReset.passwordless)), a(h, b);
                  }, $$slots: { default: true } });
                  var rr = o(Ie, 2);
                  const tr = te(() => e(D) ? 3 : 2);
                  q(rr, { get level() {
                    return e(tr);
                  }, onclick: () => r(D, "password"), get isLoading() {
                    return e(H);
                  }, children: (h, C) => {
                    k();
                    var b = R();
                    $(() => u(b, t.passwordReset.password)), a(h, b);
                  }, $$slots: { default: true } }), c(he);
                  var sr = o(he, 2);
                  {
                    var ar = (h) => {
                      var C = Lr(), b = p(C);
                      Re(b), c(C), De(3, C, () => Ve), a(h, C);
                    }, or = (h, C) => {
                      {
                        var b = ($e) => {
                          var ae = Cr(), We = p(ae);
                          Ue(We, { action: "", onSubmit: Ke, children: (ee, oe) => {
                            var G = Nr(), ne = E(G);
                            fr(ne, { autocomplete: "off", get label() {
                              return t.mfa.passkeyName;
                            }, get placeholder() {
                              return t.mfa.passkeyName;
                            }, width: le, maxLength: 32, pattern: Rr, required: true, get ref() {
                              return e(de);
                            }, set ref(M) {
                              r(de, M, true);
                            }, get value() {
                              return e(X);
                            }, set value(M) {
                              r(X, M, true);
                            } });
                            var re = o(ne, 2);
                            const ye = te(() => e(z) ? 2 : 1);
                            q(re, { type: "submit", get level() {
                              return e(ye);
                            }, children: (M, je) => {
                              k();
                              var xe = R();
                              $(() => u(xe, t.mfa.register)), a(M, xe);
                            }, $$slots: { default: true } }), a(ee, G);
                          }, $$slots: { default: true } });
                          var nr = o(We, 2);
                          {
                            var ir = (ee) => {
                              var oe = Sr(), G = p(oe), ne = p(G, true);
                              c(G);
                              var re = o(G, 2), ye = p(re, true);
                              c(re);
                              var M = o(re, 2);
                              q(M, { onclick: ke, children: (je, xe) => {
                                k();
                                var qe = R();
                                $(() => u(qe, t.passwordReset.accountLogin)), a(je, qe);
                              }, $$slots: { default: true } }), c(oe), $(() => {
                                u(ne, t.passwordReset.successPasskey1), u(ye, t.passwordReset.successPasskey2);
                              }), a(ee, oe);
                            };
                            v(nr, (ee) => {
                              e(z) && ee(ir);
                            });
                          }
                          c(ae), De(3, ae, () => Ve), a($e, ae);
                        };
                        v(h, ($e) => {
                          e(D) === "passkey" && $e(b);
                        }, C);
                      }
                    };
                    v(sr, (h) => {
                      e(D) === "password" ? h(ar) : h(or, false);
                    });
                  }
                  $(() => {
                    u(Z, t.passwordReset.newAccount), u(we, t.passwordReset.newAccDesc1), u(Ee, `${t.passwordReset.newAccDesc2 ?? ""} `);
                  }), a(P, j);
                }, _e = (P, j) => {
                  {
                    var F = (Z) => {
                      var K = Ir(), we = o(E(K), 2);
                      Re(we), a(Z, K);
                    };
                    v(P, (Z) => {
                      var _a;
                      ((_a = U.get()) == null ? void 0 : _a.startsWith("password_reset")) && Z(F);
                    }, j);
                  }
                };
                v(J, (P) => {
                  var _a;
                  ((_a = U.get()) == null ? void 0 : _a.startsWith("new_user")) ? P(S) : P(_e, false);
                });
              }
              var Ce = o(J, 2);
              {
                var me = (P) => {
                  var j = Wr(), F = p(j, true);
                  c(j), $(() => u(F, e(y))), a(P, j);
                };
                v(Ce, (P) => {
                  e(y) && P(me);
                });
              }
              c(n), a(W, n);
            };
            v(l, (W) => {
              e(i) && W(N);
            }, x);
          }
        };
        v(V, (l) => {
          e(z) ? l(O) : l(B, false);
        });
      }
      var L = o(V, 2);
      xr(L, { absolute: true }), a(m, w);
    } });
  } });
  var Ye = o(Se, 2);
  mr(Ye, { absolute: true }), a(ze, Ae), dr();
}
export {
  it as component
};
