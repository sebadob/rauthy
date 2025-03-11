import { t as k, a as i, d as pe, n as Ue, e as J } from "./CWf9OOFK.js";
import { j as e, k as O, l as p, p as ve, ab as ie, aa as fe, c as r, r as a, a as de, f as le, s as b, t as h, a8 as Q, P as De } from "./nlANaGLT.js";
import { e as me, s as w, b as Te } from "./BmMHVVX3.js";
import { i as K } from "./DOjUa9u5.js";
import { s as N, a as re, b as Ce } from "./CZv_AhHu.js";
import { a as Ee } from "./DHRD3bu9.js";
import { p as W } from "./5u5qd9TD.js";
import { p as Y } from "./uWmgYd3Z.js";
import { I as Ne } from "./lUFuVbaC.js";
import { u as ce } from "./DwvS5LQk.js";
import { i as Fe, g as ze, C as Ae, l as ae } from "./B21bTIl7.js";
import { I as he } from "./DJ6IvX5N.js";
import { B as ne } from "./BEQMYyDu.js";
import { f as be, c as ge, d as Be } from "./BO1A6s0c.js";
import { T as Se } from "./C8iV2OFO.js";
import { e as $e } from "./B6azywu7.js";
import { I as Le } from "./6f3yten3.js";
import { E as Me } from "./D8CbPIAy.js";
import { g as Re } from "./BRCxk8by.js";
import { L as te } from "./x5MgfX0R.js";
let _e = O(void 0);
function je() {
  return !e(_e) && Fe() && be("/auth/v1/users/picture_config").then(($) => {
    p(_e, W($.body));
  }), { get() {
    return e(_e);
  } };
}
var Ke = k("<span> </span>"), Ve = k('<img loading="lazy" class="absolute svelte-1e810u7" aria-label="Avatar" alt="" width="100%">'), qe = k('<div class="errLimit svelte-1e810u7"> </div>'), Ge = k("<div><!></div>"), He = k('<div class="relative"><div class="delete svelte-1e810u7"><!></div></div>'), Oe = k('<form class="avatar svelte-1e810u7" aria-dropeffect="move" aria-label="Upload"><label class="svelte-1e810u7"><!></label> <input type="file" aria-hidden="true" class="svelte-1e810u7"> <!> <!></form> <!>', 1), Je = k('<span class="avatar svelte-1e810u7"><!></span>'), Qe = k('<div class="container svelte-1e810u7"><!></div>');
function Ct($, t) {
  ve(t, true);
  const o = (c) => {
    var u = pe(), n = le(u);
    {
      var B = (S) => {
        var M = Ke(), G = r(M, true);
        a(M), h((se) => {
          Ce(M, 1, `absolute font-mono font-${y()} noselect`, "svelte-1e810u7"), w(G, se);
        }, [() => t.fallbackCharacters.toUpperCase()]), i(S, M);
      }, T = (S) => {
        var M = Ve();
        h(() => N(M, "src", `/auth/v1/users/${t.userId}/picture/${s()}`)), me("error", M, q), Te(M), i(S, M);
      };
      K(n, (S) => {
        e(x) || !s() ? S(B) : S(T, false);
      });
    }
    i(c, u);
  };
  let s = Y(t, "pictureId", 15), y = Y(t, "size", 3, "medium");
  const f = ze(), L = "image/png, image/jpeg, image/webp";
  let z = ce(), g = fe(() => je().get()), x = O(""), m = O(""), _, I = O(false), P = O(false), C = O(void 0), V = fe(R), E = fe(() => {
    switch (y()) {
      case "small":
        return "2.25rem";
      case "medium":
        return "3rem";
      default:
        return "192px";
    }
  });
  ie(() => {
    e(x) && console.error(e(x));
  }), ie(() => {
    e(m) && (_ && clearTimeout(_), _ = setTimeout(() => {
      p(m, "");
    }, 5e3));
  }), ie(() => {
    e(C) && e(C).length > 0 && l(e(C));
  });
  function R() {
    return `hsl(${v(t.fallbackCharacters)}, 50%, 50%)`;
  }
  function q(c) {
    c.type === "error" && p(x, "Input Error");
  }
  function Z() {
    p(I, true);
  }
  function X() {
    p(I, false);
  }
  function v(c) {
    let u = [1, 255, 3, 13, 19];
    for (let n = 0; n < 5; n++) {
      let B = c.charCodeAt(n);
      if (B) u[n] = B;
      else break;
    }
    return (u[0] * (64 + u[1]) + 4 * u[0] + (u[2] * u[3] - u[4])) % 360;
  }
  async function F() {
    if (p(x, ""), p(m, ""), !s()) return;
    let c = await ge(`/auth/v1/users/${t.userId}/picture/${s()}`);
    c.error ? p(x, W(c.error.message)) : s(void 0);
  }
  async function l(c) {
    if (p(x, ""), p(m, ""), !e(g)) return;
    p(P, true);
    let u = `/auth/v1/users/${t.userId}/picture`;
    for (let n of c) {
      if (!L.includes(n.type)) {
        p(x, "Invalid File Format, allowed: " + L);
        break;
      }
      if (n.size > e(g).content_len_limit) {
        p(m, `${z.common.maxFileSize}: ${e(g).content_len_limit / 1024 / 1024} MB`);
        break;
      }
      let B = new FormData();
      B.append(n.name, n);
      let T = await fetch(u, { method: "PUT", headers: { "x-csrf-token": localStorage.getItem(Ae) || "" }, body: B });
      if (T.ok) s(await T.text());
      else {
        let S = await T.json();
        T.status === 406 ? alert("max size" + S.message) : p(x, W(S.message || "Upload Error"));
      }
    }
    p(C, void 0), p(P, false);
  }
  var d = Qe(), U = r(d);
  {
    var D = (c) => {
      var u = Oe(), n = le(u);
      let B;
      var T = r(n);
      N(T, "for", f), N(T, "aria-controls", f);
      let S;
      var M = r(T);
      Ne(M, { get width() {
        return e(E);
      } }), a(T);
      var G = b(T, 2);
      N(G, "id", f), N(G, "accept", L);
      var se = b(G, 2);
      {
        var we = (j) => {
          var H = qe();
          let ee;
          var ue = r(H, true);
          a(H), h(() => {
            ee = re(H, "", ee, { width: e(E) }), w(ue, e(m));
          }), i(j, H);
        };
        K(se, (j) => {
          e(m) && j(we);
        });
      }
      var ye = b(se, 2);
      o(ye), a(n);
      var xe = b(n, 2);
      {
        var ke = (j) => {
          var H = He(), ee = r(H), ue = r(ee);
          ne(ue, { invisible: true, onclick: F, children: (Ie, nt) => {
            var oe = Ge(), Pe = r(oe);
            he(Pe, {}), a(oe), h(() => N(oe, "title", z.common.delete)), i(Ie, oe);
          }, $$slots: { default: true } }), a(ee), a(H), i(j, H);
        };
        K(xe, (j) => {
          y() === "large" && s() && j(ke);
        });
      }
      h(() => {
        B = re(n, "", B, { "background-color": e(V), width: e(E) }), N(T, "aria-disabled", e(P)), N(T, "data-show", !e(P) && e(I)), S = re(T, "", S, { width: e(E) }), G.disabled = e(P), N(G, "aria-disabled", e(P));
      }), me("mouseenter", n, Z), me("mouseleave", n, X), Ee(G, () => e(C), (j) => p(C, j)), i(c, u);
    }, A = (c) => {
      var u = Je();
      let n;
      var B = r(u);
      o(B), a(u), h(() => n = re(u, "", n, { "background-color": e(V), width: e(E), height: e(E) })), i(c, u);
    };
    K(U, (c) => {
      var _a;
      ((_a = e(g)) == null ? void 0 : _a.upload_allowed) && !t.disableUpload ? c(D) : c(A, false);
    });
  }
  a(d), i($, d), de();
}
var We = Ue('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M7.864 4.243A7.5 7.5 0 0119.5 10.5c0 2.92-.556 5.709-1.568 8.268M5.742 6.364A7.465 7.465 0 004.5 10.5a7.464 7.464 0 01-1.15 3.993m1.989 3.559A11.209 11.209 0 008.25 10.5a3.75 3.75 0 117.5 0c0 .527-.021 1.049-.064 1.565M12 10.5a14.94 14.94 0 01-3.6 9.75m6.633-4.596a18.666 18.666 0 01-2.485 5.33"></path></svg>');
function Xe($, t) {
  let o = Y(t, "opacity", 3, 0.9), s = Y(t, "width", 3, "1.5rem"), y = Y(t, "color", 3, "currentColor");
  var f = We();
  N(f, "stroke-width", 2), h(() => {
    N(f, "stroke", y()), N(f, "width", s()), N(f, "opacity", o());
  }), i($, f);
}
var Ye = k("<div><!></div>"), Ze = k('<div class="row svelte-165b9or"><div></div> <div class="deleteBtn svelte-165b9or"><!></div></div>'), et = k('<div class="keyContainer svelte-165b9or"><div class="row svelte-165b9or"><span class="label svelte-165b9or"> </span> <div class="nameUv svelte-165b9or"><b> </b> <!></div></div> <div class="row svelte-165b9or"><span class="label svelte-165b9or"> </span> <span class="font-mono"> </span></div> <div class="row svelte-165b9or"><span class="label svelte-165b9or"> </span> <span class="font-mono"> </span></div> <!></div>');
function Et($, t) {
  ve(t, true);
  let o = ce();
  var s = et(), y = r(s), f = r(y), L = r(f, true);
  a(f);
  var z = b(f, 2), g = r(z), x = r(g, true);
  a(g);
  var m = b(g, 2);
  {
    var _ = (d) => {
      Se(d, { get text() {
        return o.account.userVerifiedTooltip;
      }, children: (U, D) => {
        var A = Ye();
        re(A, "", {}, { "margin-bottom": "-.25rem" });
        var c = r(A);
        Xe(c, { width: 18, color: "var(--col-acnt)" }), a(A), i(U, A);
      }, $$slots: { default: true } });
    };
    K(m, (d) => {
      t.passkey.user_verified && d(_);
    });
  }
  a(z), a(y);
  var I = b(y, 2), P = r(I), C = r(P, true);
  a(P);
  var V = b(P, 2), E = r(V, true);
  a(V), a(I);
  var R = b(I, 2), q = r(R), Z = r(q, true);
  a(q);
  var X = b(q, 2), v = r(X, true);
  a(X), a(R);
  var F = b(R, 2);
  {
    var l = (d) => {
      var U = Ze(), D = b(r(U), 2), A = r(D);
      ne(A, { level: -3, onclick: () => t.onDelete(t.passkey.name), children: (c, u) => {
        Q();
        var n = J();
        h(() => w(n, o.common.delete)), i(c, n);
      }, $$slots: { default: true } }), a(D), a(U), i(d, U);
    };
    K(F, (d) => {
      t.showDelete && d(l);
    });
  }
  a(s), h((d, U) => {
    w(L, o.mfa.passkeyName), w(x, t.passkey.name), w(C, o.mfa.registerd), w(E, d), w(Z, o.mfa.lastUsed), w(v, U);
  }, [() => ae(t.passkey.registered), () => ae(t.passkey.last_used)]), i($, s), de();
}
var tt = k('<div class="device-header svelte-9n911e"><div class="device-head font-mono svelte-9n911e"> </div></div>'), rt = k('<div class="saveButton svelte-9n911e"><!></div>'), at = k("<div><!></div>"), st = k("<div> </div>"), ot = k('<div class="device svelte-9n911e"><!> <div class="row svelte-9n911e"><!> <!></div> <!> <!> <!> <!></div>');
function it($, t) {
  ve(t, true);
  let o = Y(t, "device", 15), s = ce(), y = O(W(De(() => o().name)));
  async function f() {
    let z = { device_id: o().id, name: e(y) }, g = await Be(`/auth/v1/users/${t.userId}/devices`, z);
    g.error ? console.error(g.error) : o(o().name = e(y), true);
  }
  async function L() {
    let z = { device_id: o().id }, g = await ge(`/auth/v1/users/${t.userId}/devices`, z);
    g.error ? console.error(g.error) : o(o().refresh_exp = void 0, true);
  }
  Me($, { summary: (x) => {
    var m = tt(), _ = r(m), I = r(_, true);
    a(_), a(m), h(() => w(I, o().name)), i(x, m);
  }, details: (x) => {
    var m = ot(), _ = r(m);
    te(_, { get label() {
      return s.account.deviceId;
    }, mono: true, children: (v, F) => {
      Q();
      var l = J();
      h(() => w(l, o().id)), i(v, l);
    } });
    var I = b(_, 2), P = r(I);
    Le(P, { autocomplete: "off", width: "17rem", get label() {
      return s.account.deviceName;
    }, get placeholder() {
      return s.account.deviceName;
    }, onEnter: f, pattern: Re, required: true, get value() {
      return e(y);
    }, set value(v) {
      p(y, W(v));
    }, children: (v, F) => {
      Q();
      var l = J();
      h(() => w(l, s.account.deviceName)), i(v, l);
    }, $$slots: { default: true } });
    var C = b(P, 2);
    {
      var V = (v) => {
        var F = rt(), l = r(F);
        ne(l, { onclick: f, children: (d, U) => {
          Q();
          var D = J();
          h(() => w(D, s.common.save)), i(d, D);
        }, $$slots: { default: true } }), a(F), i(v, F);
      };
      K(C, (v) => {
        e(y) !== o().name && v(V);
      });
    }
    a(I);
    var E = b(I, 2);
    te(E, { get label() {
      return s.account.regDate;
    }, children: (v, F) => {
      Q();
      var l = J();
      h((d) => w(l, d), [() => ae(o().created)]), i(v, l);
    } });
    var R = b(E, 2);
    te(R, { get label() {
      return s.account.accessExp;
    }, children: (v, F) => {
      Q();
      var l = J();
      h((d) => w(l, d), [() => ae(o().access_exp)]), i(v, l);
    } });
    var q = b(R, 2);
    {
      var Z = (v) => {
        te(v, { get label() {
          return s.account.accessRenew;
        }, button: (l) => {
          ne(l, { get ariaLabel() {
            return s.common.delete;
          }, invisible: true, onclick: L, children: (d, U) => {
            var D = at(), A = r(D);
            he(A, {}), a(D), h(() => N(D, "title", s.common.delete)), i(d, D);
          }, $$slots: { default: true } });
        }, children: (l, d) => {
          var U = st(), D = r(U, true);
          a(U), h((A) => w(D, A), [() => ae(o().refresh_exp)]), i(l, U);
        } });
      };
      K(q, (v) => {
        o().refresh_exp && v(Z);
      });
    }
    var X = b(q, 2);
    te(X, { get label() {
      return s.account.regIp;
    }, children: (v, F) => {
      Q();
      var l = J();
      h(() => w(l, o().peer_ip)), i(v, l);
    } }), a(m), i(x, m);
  }, $$slots: { summary: true, details: true } }), de();
}
var lt = k('<div class="head svelte-w0g6eh"> </div> <div class="devices svelte-w0g6eh"><!></div>', 1);
function Nt($, t) {
  ve(t, true);
  let o = ce(), s = O(W([]));
  ie(() => {
    y();
  });
  async function y() {
    let _ = await be(`/auth/v1/users/${t.userId}/devices`);
    _.body ? p(s, W(_.body)) : console.error("error fetching devices: " + _.error);
  }
  var f = lt(), L = le(f), z = r(L, true);
  a(L);
  var g = b(L, 2), x = r(g);
  {
    var m = (_) => {
      var I = pe(), P = le(I);
      $e(P, 19, () => e(s), (C) => C.id, (C, V, E) => {
        it(C, { get userId() {
          return t.userId;
        }, get device() {
          return e(s)[e(E)];
        }, set device(R) {
          e(s)[e(E)] = R;
        } });
      }), i(_, I);
    };
    K(x, (_) => {
      t.userId && _(m);
    });
  }
  a(g), h(() => w(z, (o == null ? void 0 : o.account.devicesDesc) || "Devices linked to this account")), i($, f), de();
}
export {
  Nt as D,
  Ct as U,
  Et as a
};
