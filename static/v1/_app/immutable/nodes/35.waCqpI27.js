import { t as m, a as t, d as B, e as L } from "../chunks/BH6NCLk-.js";
import { p as pr, aa as ue, f as _, a as vr, l as r, k as u, j as e, s as o, a6 as Re, c as v, a7 as N, a9 as ae, r as c, t as b } from "../chunks/CvlvO1XB.js";
import { h as cr, s as p } from "../chunks/CTI4QPiR.js";
import { i as f } from "../chunks/BUO_AUgz.js";
import { B as D, t as Ve, s as Fe } from "../chunks/DMkkW5Nn.js";
import { p as l } from "../chunks/Wh68IIk2.js";
import { z as fr, A as _r } from "../chunks/BaE7H8ny.js";
import { P as mr } from "../chunks/h7VULPdI.js";
import { I as wr } from "../chunks/BtKnbFDH.js";
import { W as gr } from "../chunks/8x1xiTUd.js";
import { L as hr } from "../chunks/DUtP9z1z.js";
import { u as $r } from "../chunks/BQ1-pLIs.js";
import { M as yr } from "../chunks/Ds6bXi0i.js";
import { C as xr } from "../chunks/GWgockW8.js";
import { T as br } from "../chunks/DcJo-CQ6.js";
import { u as Pr } from "../chunks/Beqa1JKV.js";
import { T as Rr } from "../chunks/CVHA2fPQ.js";
import { I as Me } from "../chunks/D6E4lNPv.js";
import { A as Ue } from "../chunks/gtHtP0A2.js";
import { u as kr } from "../chunks/BMFqJ6Jy.js";
import { w as Tr } from "../chunks/Co1XQfiq.js";
import { F as ze } from "../chunks/BS0DIDHc.js";
import { b as Ar } from "../chunks/BRCxk8by.js";
var Lr = m('<!> <!> <!> <div class="generate svelte-1w50bpj"><!></div> <!>', 1), Nr = m('<div class="dev svelte-1w50bpj"><p>This window shows up during local dev,<br> only to be able to switch modes easily.<br></p> <!> <!></div>'), Sr = m("<p> <br> <br> <br> <!></p>"), Cr = m("<div><!></div>"), Er = m("<!> <!>", 1), Ir = m('<div class="success"><p> </p> <p> </p> <!></div>'), Wr = m("<div><!> <!></div>"), jr = m('<h1> </h1> <p> </p> <p> <!></p> <div class="typeChoice svelte-1w50bpj"><!> <!></div> <!>', 1), qr = m("<h1>Password Reset</h1> <!>", 1), Dr = m('<div class="err svelte-1w50bpj"> </div>'), Vr = m('<div class="container"><!> <!></div>'), Fr = m("<!> <!> <!>", 1), Mr = m("<!> <!> <!> <!>", 1);
function pt(Oe, Be) {
  pr(Be, true);
  const ke = (a) => {
    var w = B(), g = _(w);
    {
      var j = (h) => {
        ze(h, { action: "", onSubmit: He, children: ($, C) => {
          var F = Lr(), G = _(F);
          mr(G, { get policy() {
            return e(i).password_policy;
          }, get password() {
            return e(S);
          }, get accepted() {
            return e(fe);
          }, set accepted(n) {
            r(fe, l(n));
          } });
          var H = o(G, 2);
          const E = ae(() => e(S).length >= e(i).password_policy.length_min);
          Me(H, { autocomplete: "new-password", get label() {
            return s.account.passwordNew;
          }, get placeholder() {
            return s.account.passwordNew;
          }, get maxLength() {
            return e(i).password_policy.length_max;
          }, required: true, get showCopy() {
            return e(E);
          }, width: pe, get ref() {
            return e(ce);
          }, set ref(n) {
            r(ce, l(n));
          }, get value() {
            return e(S);
          }, set value(n) {
            r(S, l(n));
          }, get reportValidity() {
            return e(_e);
          }, set reportValidity(n) {
            r(_e, l(n));
          } });
          var d = o(H, 2);
          Me(d, { autocomplete: "new-password", get label() {
            return s.account.passwordConfirm;
          }, get placeholder() {
            return s.account.passwordConfirm;
          }, get maxLength() {
            return e(i).password_policy.length_max;
          }, required: true, width: pe, get value() {
            return e(ee);
          }, set value(n) {
            r(ee, l(n));
          }, get reportValidity() {
            return e(me);
          }, set reportValidity(n) {
            r(me, l(n));
          } });
          var y = o(d, 2), q = v(y);
          D(q, { level: 2, onclick: Ke, children: (n, M) => {
            N();
            var k = L();
            b(() => p(k, s.passwordReset.generate)), t(n, k);
          }, $$slots: { default: true } }), c(y);
          var I = o(y, 2);
          D(I, { type: "submit", get isLoading() {
            return e(X);
          }, children: (n, M) => {
            N();
            var k = L();
            b(() => p(k, s.common.save)), t(n, k);
          }, $$slots: { default: true } }), t($, F);
        }, $$slots: { default: true } });
      };
      f(g, (h) => {
        e(i) && h(j);
      });
    }
    t(a, w);
  }, pe = "20rem";
  let s = $r(), Je = kr().get(), i = u(void 0), ve = u(void 0), ce = u(void 0), X = u(false), P = u(""), J = Pr("type", "password_reset"), V = u(""), oe = u(""), K = u(false), fe = u(false), Y = u(void 0), Z = u(""), S = u(""), ee = u(""), _e = u(void 0), me = u(void 0);
  ue(() => {
    e(V) && we();
  }), ue(() => {
    var _a;
    (_a = e(ve)) == null ? void 0 : _a.focus();
  }), ue(() => {
    var _a;
    (_a = e(ce)) == null ? void 0 : _a.focus();
  }), ue(() => {
    e(K) && setTimeout(() => {
      e(oe) ? window.location.replace(e(oe)) : Te();
    }, 5e3);
  });
  function Te() {
    window.location.replace("/auth/v1/account");
  }
  function Ke() {
    if (e(i)) {
      let a = _r(e(i).password_policy);
      r(S, l(a)), r(ee, l(a)), requestAnimationFrame(() => {
        var _a, _b;
        (_a = e(_e)) == null ? void 0 : _a(), (_b = e(me)) == null ? void 0 : _b();
      });
    }
  }
  async function Ge() {
    if (!e(i)) {
      console.error("template data is undefined");
      return;
    }
    if (r(P, ""), e(Z).length < 1) {
      r(P, l(s.mfa.passkeyNameErr));
      return;
    }
    let a = await Tr(e(i).user_id, e(Z), s.authorize.invalidKeyUsed, s.authorize.requestExpired);
    a.error ? r(P, `${s.mfa.errorReg} - ${a.error}`) : (we(), r(K, true));
  }
  async function He() {
    var _a;
    if (r(P, ""), !!e(fe)) {
      if (e(S) !== e(ee)) {
        r(P, l(s.passwordReset.passwordNoMatch));
        return;
      }
      if (e(S).length > 256) {
        r(P, "max 256");
        return;
      }
      ((_a = e(i)) == null ? void 0 : _a.needs_mfa) ? r(Y, "PasswordReset") : await Ae();
    }
  }
  async function Ae(a) {
    var _a;
    if (!e(i)) return;
    r(X, true);
    const w = { password: e(S), magic_link_id: e(i).magic_link_id, mfa_code: a };
    let g = await fetch(`/auth/v1/users/${e(i).user_id}/reset`, { method: "PUT", headers: { "content-type": "application/json", "pwd-csrf-token": (_a = e(i)) == null ? void 0 : _a.csrf_token }, body: JSON.stringify(w) });
    if (g.ok) we(), r(oe, l(g.headers.get("location") || "/auth/v1/account")), r(K, true);
    else {
      const j = await g.json();
      r(P, l(j.message));
    }
    r(X, false);
  }
  function Qe(a) {
    r(P, l(a)), r(Y, void 0);
  }
  function Xe(a) {
    r(Y, void 0), a && "code" in a ? Ae(a.code) : console.error("invalid webauthn response", a);
  }
  function we() {
    r(P, ""), r(Z, ""), r(S, ""), r(ee, "");
  }
  var Le = Mr();
  cr((a) => {
    var w = B(), g = _(w);
    {
      var j = ($) => {
        var C = B(), F = _(C);
        {
          var G = (E) => {
            b(() => Re.title = s.passwordReset.newAccount);
          }, H = (E) => {
            var d = B(), y = _(d);
            {
              var q = (I) => {
                b(() => Re.title = s.passwordReset.passwordReset);
              };
              f(y, (I) => {
                J.get() === "password_reset" && I(q);
              }, true);
            }
            t(E, d);
          };
          f(F, (E) => {
            var _a;
            ((_a = J.get()) == null ? void 0 : _a.startsWith("new_user")) ? E(G) : E(H, false);
          });
        }
        t($, C);
      }, h = ($) => {
        Re.title = "Password";
      };
      f(g, ($) => {
        s ? $(j) : $(h, false);
      });
    }
    t(a, w);
  });
  var Ne = _(Le);
  br(Ne, { id: fr, get value() {
    return e(i);
  }, set value(a) {
    r(i, l(a));
  } });
  var Se = o(Ne, 2);
  {
    var Ye = (a) => {
      var w = Nr(), g = o(v(w), 2);
      D(g, { level: 2, onclick: () => J.set("new_user"), children: (h, $) => {
        N();
        var C = L("new_user");
        t(h, C);
      }, $$slots: { default: true } });
      var j = o(g, 2);
      D(j, { level: 2, onclick: () => J.set("password_reset"), children: (h, $) => {
        N();
        var C = L("password_reset");
        t(h, C);
      }, $$slots: { default: true } }), c(w), t(a, w);
    };
    f(Se, (a) => {
      Je && a(Ye);
    });
  }
  var Ce = o(Se, 2);
  yr(Ce, { children: (a, w) => {
    xr(a, { children: (g, j) => {
      var h = Fr(), $ = _(h);
      {
        var C = (d) => {
          gr(d, { get userId() {
            return e(i).user_id;
          }, get purpose() {
            return e(Y);
          }, onSuccess: Xe, onError: Qe });
        };
        f($, (d) => {
          e(Y) && e(i) && d(C);
        });
      }
      var F = o($, 2);
      {
        var G = (d) => {
          var y = Sr(), q = v(y), I = o(q, 2), n = o(I, 4), M = o(n);
          const k = ae(() => e(oe) || "/auth/v1/account");
          Ue(M, { get href() {
            return e(k);
          }, children: (ge, Ee) => {
            N();
            var he = L("Account");
            t(ge, he);
          }, $$slots: { default: true } }), c(y), b(() => {
            p(q, `${s.passwordReset.success1 ?? ""} `), p(I, ` ${s.passwordReset.success2 ?? ""} `), p(n, ` ${s.passwordReset.success3 ?? ""} `);
          }), t(d, y);
        }, H = (d) => {
          var y = B(), q = _(y);
          {
            var I = (n) => {
              var M = Vr(), k = v(M);
              {
                var ge = (T) => {
                  var W = jr(), U = _(W), $e = v(U, true);
                  c(U);
                  var z = o(U, 2), ne = v(z, true);
                  c(z);
                  var re = o(z, 2), Ie = v(re), rr = o(Ie);
                  Ue(rr, { get href() {
                    return s.passwordReset.fidoLink;
                  }, target: "_blank", children: (x, A) => {
                    N();
                    var R = L("FIDO Alliance");
                    t(x, R);
                  }, $$slots: { default: true } }), c(re);
                  var ye = o(re, 2), We = v(ye);
                  const tr = ae(() => e(V) ? 3 : 1);
                  D(We, { get level() {
                    return e(tr);
                  }, onclick: () => r(V, "passkey"), get isLoading() {
                    return e(X);
                  }, children: (x, A) => {
                    N();
                    var R = L();
                    b(() => p(R, s.passwordReset.passwordless)), t(x, R);
                  }, $$slots: { default: true } });
                  var sr = o(We, 2);
                  const ar = ae(() => e(V) ? 3 : 2);
                  D(sr, { get level() {
                    return e(ar);
                  }, onclick: () => r(V, "password"), get isLoading() {
                    return e(X);
                  }, children: (x, A) => {
                    N();
                    var R = L();
                    b(() => p(R, s.passwordReset.password)), t(x, R);
                  }, $$slots: { default: true } }), c(ye);
                  var or = o(ye, 2);
                  {
                    var nr = (x) => {
                      var A = Cr(), R = v(A);
                      ke(R), c(A), Ve(3, A, () => Fe), t(x, A);
                    }, ir = (x) => {
                      var A = B(), R = _(A);
                      {
                        var dr = (xe) => {
                          var ie = Wr(), je = v(ie);
                          ze(je, { action: "", onSubmit: Ge, children: (te, de) => {
                            var Q = Er(), le = _(Q);
                            wr(le, { autocomplete: "off", get label() {
                              return s.mfa.passkeyName;
                            }, get placeholder() {
                              return s.mfa.passkeyName;
                            }, width: pe, maxLength: 32, pattern: Ar, required: true, get ref() {
                              return e(ve);
                            }, set ref(O) {
                              r(ve, l(O));
                            }, get value() {
                              return e(Z);
                            }, set value(O) {
                              r(Z, l(O));
                            } });
                            var se = o(le, 2);
                            const be = ae(() => e(K) ? 2 : 1);
                            D(se, { type: "submit", get level() {
                              return e(be);
                            }, children: (O, qe) => {
                              N();
                              var Pe = L();
                              b(() => p(Pe, s.mfa.register)), t(O, Pe);
                            }, $$slots: { default: true } }), t(te, Q);
                          }, $$slots: { default: true } });
                          var lr = o(je, 2);
                          {
                            var ur = (te) => {
                              var de = Ir(), Q = v(de), le = v(Q, true);
                              c(Q);
                              var se = o(Q, 2), be = v(se, true);
                              c(se);
                              var O = o(se, 2);
                              D(O, { onclick: Te, children: (qe, Pe) => {
                                N();
                                var De = L();
                                b(() => p(De, s.passwordReset.accountLogin)), t(qe, De);
                              }, $$slots: { default: true } }), c(de), b(() => {
                                p(le, s.passwordReset.successPasskey1), p(be, s.passwordReset.successPasskey2);
                              }), t(te, de);
                            };
                            f(lr, (te) => {
                              e(K) && te(ur);
                            });
                          }
                          c(ie), Ve(3, ie, () => Fe), t(xe, ie);
                        };
                        f(R, (xe) => {
                          e(V) === "passkey" && xe(dr);
                        }, true);
                      }
                      t(x, A);
                    };
                    f(or, (x) => {
                      e(V) === "password" ? x(nr) : x(ir, false);
                    });
                  }
                  b(() => {
                    p($e, s.passwordReset.newAccount), p(ne, s.passwordReset.newAccDesc1), p(Ie, `${s.passwordReset.newAccDesc2 ?? ""} `);
                  }), t(T, W);
                }, Ee = (T) => {
                  var W = B(), U = _(W);
                  {
                    var $e = (z) => {
                      var ne = qr(), re = o(_(ne), 2);
                      ke(re), t(z, ne);
                    };
                    f(U, (z) => {
                      var _a;
                      ((_a = J.get()) == null ? void 0 : _a.startsWith("password_reset")) && z($e);
                    }, true);
                  }
                  t(T, W);
                };
                f(k, (T) => {
                  var _a;
                  ((_a = J.get()) == null ? void 0 : _a.startsWith("new_user")) ? T(ge) : T(Ee, false);
                });
              }
              var he = o(k, 2);
              {
                var er = (T) => {
                  var W = Dr(), U = v(W, true);
                  c(W), b(() => p(U, e(P))), t(T, W);
                };
                f(he, (T) => {
                  e(P) && T(er);
                });
              }
              c(M), t(n, M);
            };
            f(q, (n) => {
              e(i) && n(I);
            }, true);
          }
          t(d, y);
        };
        f(F, (d) => {
          e(K) ? d(G) : d(H, false);
        });
      }
      var E = o(F, 2);
      Rr(E, { absolute: true }), t(g, h);
    } });
  } });
  var Ze = o(Ce, 2);
  hr(Ze, { absolute: true }), t(Oe, Le), vr();
}
export {
  pt as component
};
