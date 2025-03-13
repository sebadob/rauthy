import { t as k, a as i, d as pe, n as $e, e as J } from "./YHhP1LbZ.js";
import { j as e, k as O, l as p, p as ve, aa as ie, a9 as fe, c as r, r as a, a as de, f as le, s as b, t as h, a8 as Q, P as Ue } from "./Ck6jKiur.js";
import { e as me, s as w, b as De } from "./tDAaDMC_.js";
import { i as K } from "./7JDmqCCW.js";
import { s as E, a as re, b as Te } from "./BTaFr7HN.js";
import { a as Ce } from "./0HgaTnX3.js";
import { p as W } from "./ho0YXExL.js";
import { p as Y } from "./DZP54pO_.js";
import { I as Ee } from "./BuPqb6Wv.js";
import { u as ce } from "./mN05BXqA.js";
import { i as Ne, g as Fe, C as ze, l as ae } from "./B21bTIl7.js";
import { I as he } from "./DREqFFqR.js";
import { B as ne } from "./Bd2Rvcxs.js";
import { f as be, c as ge, d as Ae } from "./BO1A6s0c.js";
import { T as Be } from "./Ct8z9Mup.js";
import { e as Se } from "./Cj5zIR7o.js";
import { I as Le } from "./D75ao6x5.js";
import { E as Me } from "./CKFrqqP0.js";
import { g as Re } from "./BRCxk8by.js";
import { L as te } from "./Sykf3ifF.js";
let _e = O(void 0);
function je() {
  return !e(_e) && Ne() && be("/auth/v1/users/picture_config").then((S) => {
    p(_e, W(S.body));
  }), { get() {
    return e(_e);
  } };
}
var Ke = k("<span> </span>"), Ve = k('<img loading="lazy" class="absolute svelte-1e810u7" aria-label="Avatar" alt="" width="100%">'), qe = k('<div class="errLimit svelte-1e810u7"> </div>'), Ge = k("<div><!></div>"), He = k('<div class="relative"><div class="delete svelte-1e810u7"><!></div></div>'), Oe = k('<form class="avatar svelte-1e810u7" aria-dropeffect="move" aria-label="Upload"><label class="svelte-1e810u7"><!></label> <input type="file" aria-hidden="true" class="svelte-1e810u7"> <!> <!></form> <!>', 1), Je = k('<span class="avatar svelte-1e810u7"><!></span>'), Qe = k('<div class="container svelte-1e810u7"><!></div>');
function Tt(S, t) {
  ve(t, true);
  const o = (c) => {
    var u = pe(), n = le(u);
    {
      var A = (B) => {
        var M = Ke(), G = r(M, true);
        a(M), h((se) => {
          Te(M, 1, `absolute font-mono font-${y()} noselect`, "svelte-1e810u7"), w(G, se);
        }, [() => t.fallbackCharacters.toUpperCase()]), i(B, M);
      }, D = (B) => {
        var M = Ve();
        h(() => E(M, "src", `/auth/v1/users/${t.userId}/picture/${s()}`)), me("error", M, q), De(M), i(B, M);
      };
      K(n, (B) => {
        e(x) || !s() ? B(A) : B(D, false);
      });
    }
    i(c, u);
  };
  let s = Y(t, "pictureId", 15), y = Y(t, "size", 3, "medium");
  const f = Fe(), L = "image/png, image/jpeg, image/webp";
  let F = ce(), g = fe(() => je().get()), x = O(""), m = O(""), _, I = O(false), P = O(false), T = O(void 0), V = fe(R), C = fe(() => {
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
    e(T) && e(T).length > 0 && l(e(T));
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
      let A = c.charCodeAt(n);
      if (A) u[n] = A;
      else break;
    }
    return (u[0] * (64 + u[1]) + 4 * u[0] + (u[2] * u[3] - u[4])) % 360;
  }
  async function N() {
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
        p(m, `${F.common.maxFileSize}: ${e(g).content_len_limit / 1024 / 1024} MB`);
        break;
      }
      let A = new FormData();
      A.append(n.name, n);
      let D = await fetch(u, { method: "PUT", headers: { "x-csrf-token": localStorage.getItem(ze) || "" }, body: A });
      if (D.ok) s(await D.text());
      else {
        let B = await D.json();
        D.status === 406 ? alert("max size" + B.message) : p(x, W(B.message || "Upload Error"));
      }
    }
    p(T, void 0), p(P, false);
  }
  var d = Qe(), $ = r(d);
  {
    var U = (c) => {
      var u = Oe(), n = le(u);
      let A;
      var D = r(n);
      E(D, "for", f), E(D, "aria-controls", f);
      let B;
      var M = r(D);
      Ee(M, { get width() {
        return e(C);
      } }), a(D);
      var G = b(D, 2);
      E(G, "id", f), E(G, "accept", L);
      var se = b(G, 2);
      {
        var we = (j) => {
          var H = qe();
          let ee;
          var ue = r(H, true);
          a(H), h(() => {
            ee = re(H, "", ee, { width: e(C) }), w(ue, e(m));
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
          ne(ue, { invisible: true, onclick: N, children: (Ie, nt) => {
            var oe = Ge(), Pe = r(oe);
            he(Pe, {}), a(oe), h(() => E(oe, "title", F.common.delete)), i(Ie, oe);
          }, $$slots: { default: true } }), a(ee), a(H), i(j, H);
        };
        K(xe, (j) => {
          y() === "large" && s() && j(ke);
        });
      }
      h(() => {
        A = re(n, "", A, { "background-color": e(V), width: e(C) }), E(D, "aria-disabled", e(P)), E(D, "data-show", !e(P) && e(I)), B = re(D, "", B, { width: e(C) }), G.disabled = e(P), E(G, "aria-disabled", e(P));
      }), me("mouseenter", n, Z), me("mouseleave", n, X), Ce(G, () => e(T), (j) => p(T, j)), i(c, u);
    }, z = (c) => {
      var u = Je();
      let n;
      var A = r(u);
      o(A), a(u), h(() => n = re(u, "", n, { "background-color": e(V), width: e(C), height: e(C) })), i(c, u);
    };
    K($, (c) => {
      var _a;
      ((_a = e(g)) == null ? void 0 : _a.upload_allowed) && !t.disableUpload ? c(U) : c(z, false);
    });
  }
  a(d), i(S, d), de();
}
var We = $e('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M7.864 4.243A7.5 7.5 0 0119.5 10.5c0 2.92-.556 5.709-1.568 8.268M5.742 6.364A7.465 7.465 0 004.5 10.5a7.464 7.464 0 01-1.15 3.993m1.989 3.559A11.209 11.209 0 008.25 10.5a3.75 3.75 0 117.5 0c0 .527-.021 1.049-.064 1.565M12 10.5a14.94 14.94 0 01-3.6 9.75m6.633-4.596a18.666 18.666 0 01-2.485 5.33"></path></svg>');
function Xe(S, t) {
  let o = Y(t, "opacity", 3, 0.9), s = Y(t, "width", 3, "1.5rem"), y = Y(t, "color", 3, "currentColor");
  var f = We();
  E(f, "stroke-width", 2), h(() => {
    E(f, "stroke", y()), E(f, "width", s()), E(f, "opacity", o());
  }), i(S, f);
}
var Ye = k("<div><!></div>"), Ze = k('<div class="row svelte-165b9or"><div></div> <div class="deleteBtn svelte-165b9or"><!></div></div>'), et = k('<div class="keyContainer svelte-165b9or"><div class="row svelte-165b9or"><span class="label svelte-165b9or"> </span> <div class="nameUv svelte-165b9or"><b> </b> <!></div></div> <div class="row svelte-165b9or"><span class="label svelte-165b9or"> </span> <span class="font-mono"> </span></div> <div class="row svelte-165b9or"><span class="label svelte-165b9or"> </span> <span class="font-mono"> </span></div> <!></div>');
function Ct(S, t) {
  ve(t, true);
  let o = ce();
  var s = et(), y = r(s), f = r(y), L = r(f, true);
  a(f);
  var F = b(f, 2), g = r(F), x = r(g, true);
  a(g);
  var m = b(g, 2);
  {
    var _ = (d) => {
      Be(d, { get text() {
        return o.account.userVerifiedTooltip;
      }, children: ($, U) => {
        var z = Ye();
        re(z, "", {}, { "margin-bottom": "-.25rem" });
        var c = r(z);
        Xe(c, { width: 18, color: "var(--col-acnt)" }), a(z), i($, z);
      }, $$slots: { default: true } });
    };
    K(m, (d) => {
      t.passkey.user_verified && d(_);
    });
  }
  a(F), a(y);
  var I = b(y, 2), P = r(I), T = r(P, true);
  a(P);
  var V = b(P, 2), C = r(V, true);
  a(V), a(I);
  var R = b(I, 2), q = r(R), Z = r(q, true);
  a(q);
  var X = b(q, 2), v = r(X, true);
  a(X), a(R);
  var N = b(R, 2);
  {
    var l = (d) => {
      var $ = Ze(), U = b(r($), 2), z = r(U);
      ne(z, { level: -3, onclick: () => t.onDelete(t.passkey.name), children: (c, u) => {
        Q();
        var n = J();
        h(() => w(n, o.common.delete)), i(c, n);
      }, $$slots: { default: true } }), a(U), a($), i(d, $);
    };
    K(N, (d) => {
      t.showDelete && d(l);
    });
  }
  a(s), h((d, $) => {
    w(L, o.mfa.passkeyName), w(x, t.passkey.name), w(T, o.mfa.registerd), w(C, d), w(Z, o.mfa.lastUsed), w(v, $);
  }, [() => ae(t.passkey.registered), () => ae(t.passkey.last_used)]), i(S, s), de();
}
var tt = k('<div class="device-header svelte-9n911e"><div class="device-head font-mono svelte-9n911e"> </div></div>'), rt = k('<div class="saveButton svelte-9n911e"><!></div>'), at = k("<div><!></div>"), st = k("<div> </div>"), ot = k('<div class="device svelte-9n911e"><!> <div class="row svelte-9n911e"><!> <!></div> <!> <!> <!> <!></div>');
function it(S, t) {
  ve(t, true);
  let o = Y(t, "device", 15), s = ce(), y = O(W(Ue(() => o().name)));
  async function f() {
    let F = { device_id: o().id, name: e(y) }, g = await Ae(`/auth/v1/users/${t.userId}/devices`, F);
    g.error ? console.error(g.error) : o(o().name = e(y), true);
  }
  async function L() {
    let F = { device_id: o().id }, g = await ge(`/auth/v1/users/${t.userId}/devices`, F);
    g.error ? console.error(g.error) : o(o().refresh_exp = void 0, true);
  }
  Me(S, { summary: (x) => {
    var m = tt(), _ = r(m), I = r(_, true);
    a(_), a(m), h(() => w(I, o().name)), i(x, m);
  }, details: (x) => {
    var m = ot(), _ = r(m);
    te(_, { get label() {
      return s.account.deviceId;
    }, mono: true, children: (v, N) => {
      Q();
      var l = J();
      h(() => w(l, o().id)), i(v, l);
    }, $$slots: { default: true } });
    var I = b(_, 2), P = r(I);
    Le(P, { autocomplete: "off", width: "17rem", get label() {
      return s.account.deviceName;
    }, get placeholder() {
      return s.account.deviceName;
    }, onEnter: f, pattern: Re, required: true, get value() {
      return e(y);
    }, set value(v) {
      p(y, W(v));
    }, children: (v, N) => {
      Q();
      var l = J();
      h(() => w(l, s.account.deviceName)), i(v, l);
    }, $$slots: { default: true } });
    var T = b(P, 2);
    {
      var V = (v) => {
        var N = rt(), l = r(N);
        ne(l, { onclick: f, children: (d, $) => {
          Q();
          var U = J();
          h(() => w(U, s.common.save)), i(d, U);
        }, $$slots: { default: true } }), a(N), i(v, N);
      };
      K(T, (v) => {
        e(y) !== o().name && v(V);
      });
    }
    a(I);
    var C = b(I, 2);
    te(C, { get label() {
      return s.account.regDate;
    }, children: (v, N) => {
      Q();
      var l = J();
      h((d) => w(l, d), [() => ae(o().created)]), i(v, l);
    }, $$slots: { default: true } });
    var R = b(C, 2);
    te(R, { get label() {
      return s.account.accessExp;
    }, children: (v, N) => {
      Q();
      var l = J();
      h((d) => w(l, d), [() => ae(o().access_exp)]), i(v, l);
    }, $$slots: { default: true } });
    var q = b(R, 2);
    {
      var Z = (v) => {
        te(v, { get label() {
          return s.account.accessRenew;
        }, button: (l) => {
          ne(l, { get ariaLabel() {
            return s.common.delete;
          }, invisible: true, onclick: L, children: (d, $) => {
            var U = at(), z = r(U);
            he(z, {}), a(U), h(() => E(U, "title", s.common.delete)), i(d, U);
          }, $$slots: { default: true } });
        }, children: (l, d) => {
          var $ = st(), U = r($, true);
          a($), h((z) => w(U, z), [() => ae(o().refresh_exp)]), i(l, $);
        }, $$slots: { button: true, default: true } });
      };
      K(q, (v) => {
        o().refresh_exp && v(Z);
      });
    }
    var X = b(q, 2);
    te(X, { get label() {
      return s.account.regIp;
    }, children: (v, N) => {
      Q();
      var l = J();
      h(() => w(l, o().peer_ip)), i(v, l);
    }, $$slots: { default: true } }), a(m), i(x, m);
  }, $$slots: { summary: true, details: true } }), de();
}
var lt = k('<div class="head svelte-w0g6eh"> </div> <div class="devices svelte-w0g6eh"><!></div>', 1);
function Et(S, t) {
  ve(t, true);
  let o = ce(), s = O(W([]));
  ie(() => {
    y();
  });
  async function y() {
    let _ = await be(`/auth/v1/users/${t.userId}/devices`);
    _.body ? p(s, W(_.body)) : console.error("error fetching devices: " + _.error);
  }
  var f = lt(), L = le(f), F = r(L, true);
  a(L);
  var g = b(L, 2), x = r(g);
  {
    var m = (_) => {
      var I = pe(), P = le(I);
      Se(P, 19, () => e(s), (T) => T.id, (T, V, C) => {
        it(T, { get userId() {
          return t.userId;
        }, get device() {
          return e(s)[e(C)];
        }, set device(R) {
          e(s)[e(C)] = R;
        } });
      }), i(_, I);
    };
    K(x, (_) => {
      t.userId && _(m);
    });
  }
  a(g), h(() => w(F, (o == null ? void 0 : o.account.devicesDesc) || "Devices linked to this account")), i(S, f), de();
}
export {
  Et as D,
  Tt as U,
  Ct as a
};
