import { t as k, a as i, d as _e, n as Ie, e as J } from "./BH6NCLk-.js";
import { j as e, k as H, l as p, p as le, aa as se, a9 as ue, c as r, r as a, a as ne, f as oe, s as g, t as h, a7 as Q, O as Pe } from "./CvlvO1XB.js";
import { e as fe, s as w, b as Ue } from "./CTI4QPiR.js";
import { i as j } from "./BUO_AUgz.js";
import { s as F, a as De } from "./BMbqVy6X.js";
import { a as G, B as ie } from "./CqH8LXO-.js";
import { a as Te } from "./dU6E9WaN.js";
import { p as W } from "./Wh68IIk2.js";
import { p as Y } from "./C6SR4G2t.js";
import { I as Ce } from "./BWnKfvFA.js";
import { u as ve } from "./CUqQZdNU.js";
import { i as Ee, g as Ne, C as Fe, l as te } from "./B21bTIl7.js";
import { I as pe } from "./Vi3uK7uO.js";
import { f as he, c as ge, d as ze } from "./BO1A6s0c.js";
import { T as Ae } from "./ChMSzdNf.js";
import { e as Be } from "./BpWRzPRQ.js";
import { I as Se } from "./KD8vSltG.js";
import { E as $e } from "./4p_0EoSa.js";
import { g as Le } from "./BRCxk8by.js";
import { L as ee } from "./nnFaiMsH.js";
let me = H(void 0);
function Me() {
  return !e(me) && Ee() && he("/auth/v1/users/picture_config").then(($) => {
    p(me, W($.body));
  }), { get() {
    return e(me);
  } };
}
var Re = k("<span> </span>"), je = k('<img loading="lazy" class="absolute svelte-1e810u7" aria-label="Avatar" alt="" width="100%">'), Ke = k('<div class="errLimit svelte-1e810u7"> </div>'), Oe = k("<div><!></div>"), Ve = k('<div class="relative"><div class="delete svelte-1e810u7"><!></div></div>'), qe = k('<form class="avatar svelte-1e810u7" aria-dropeffect="move" aria-label="Upload"><label class="svelte-1e810u7"><!></label> <input type="file" aria-hidden="true" class="svelte-1e810u7"> <!> <!></form> <!>', 1), Ge = k('<span class="avatar svelte-1e810u7"><!></span>'), He = k('<div class="container svelte-1e810u7"><!></div>');
function Dt($, t) {
  le(t, true);
  const o = (u) => {
    var d = _e(), n = oe(d);
    {
      var P = (U) => {
        var S = Re(), de = r(S, true);
        a(S), h((ce) => {
          De(S, 1, `absolute font-mono font-${x()} noselect`, "svelte-1e810u7"), w(de, ce);
        }, [() => t.fallbackCharacters.toUpperCase()]), i(U, S);
      }, V = (U) => {
        var S = je();
        h(() => F(S, "src", `/auth/v1/users/${t.userId}/picture/${s()}`)), fe("error", S, O), Ue(S), i(U, S);
      };
      j(n, (U) => {
        e(y) || !s() ? U(P) : U(V, false);
      });
    }
    i(u, d);
  };
  let s = Y(t, "pictureId", 15), x = Y(t, "size", 3, "medium");
  const f = Ne(), L = "image/png, image/jpeg, image/webp";
  let A = ve(), b = ue(() => Me().get()), y = H(""), m = H(""), _, I = H(false), D = H(false), E = H(void 0), K = ue(M), N = ue(() => {
    switch (x()) {
      case "small":
        return "2.25rem";
      case "medium":
        return "3rem";
      default:
        return "192px";
    }
  });
  se(() => {
    e(y) && console.error(e(y));
  }), se(() => {
    e(m) && (_ && clearTimeout(_), _ = setTimeout(() => {
      p(m, "");
    }, 5e3));
  }), se(() => {
    e(E) && e(E).length > 0 && l(e(E));
  });
  function M() {
    return `hsl(${v(t.fallbackCharacters)}, 50%, 50%)`;
  }
  function O(u) {
    u.type === "error" && p(y, "Input Error");
  }
  function Z() {
    p(I, true);
  }
  function X() {
    p(I, false);
  }
  function v(u) {
    let d = [1, 255, 3, 13, 19];
    for (let n = 0; n < 5; n++) {
      let P = u.charCodeAt(n);
      if (P) d[n] = P;
      else break;
    }
    return (d[0] * (64 + d[1]) + 4 * d[0] + (d[2] * d[3] - d[4])) % 360;
  }
  async function z() {
    if (p(y, ""), p(m, ""), !s()) return;
    let u = await ge(`/auth/v1/users/${t.userId}/picture/${s()}`);
    u.error ? p(y, W(u.error.message)) : s(void 0);
  }
  async function l(u) {
    if (p(y, ""), p(m, ""), !e(b)) return;
    p(D, true);
    let d = `/auth/v1/users/${t.userId}/picture`;
    for (let n of u) {
      if (!L.includes(n.type)) {
        p(y, "Invalid File Format, allowed: " + L);
        break;
      }
      if (n.size > e(b).content_len_limit) {
        p(m, `${A.common.maxFileSize}: ${e(b).content_len_limit / 1024 / 1024} MB`);
        break;
      }
      let P = new FormData();
      P.append(n.name, n);
      let V = await fetch(d, { method: "PUT", headers: { "x-csrf-token": localStorage.getItem(Fe) || "" }, body: P });
      if (V.ok) s(await V.text());
      else {
        let U = await V.json();
        V.status === 406 ? alert("max size" + U.message) : p(y, W(U.message || "Upload Error"));
      }
    }
    p(E, void 0), p(D, false);
  }
  var c = He(), T = r(c);
  {
    var C = (u) => {
      var d = qe(), n = oe(d), P = r(n);
      F(P, "for", f), F(P, "aria-controls", f);
      var V = r(P);
      Ce(V, { get width() {
        return e(N);
      } }), a(P);
      var U = g(P, 2);
      F(U, "id", f), F(U, "accept", L);
      var S = g(U, 2);
      {
        var de = (R) => {
          var q = Ke(), re = r(q, true);
          a(q), h(() => {
            G(q, "width", e(N)), w(re, e(m));
          }), i(R, q);
        };
        j(S, (R) => {
          e(m) && R(de);
        });
      }
      var ce = g(S, 2);
      o(ce), a(n);
      var be = g(n, 2);
      {
        var we = (R) => {
          var q = Ve(), re = r(q), xe = r(re);
          ie(xe, { invisible: true, onclick: z, children: (ye, it) => {
            var ae = Oe(), ke = r(ae);
            pe(ke, {}), a(ae), h(() => F(ae, "title", A.common.delete)), i(ye, ae);
          }, $$slots: { default: true } }), a(re), a(q), i(R, q);
        };
        j(be, (R) => {
          x() === "large" && s() && R(we);
        });
      }
      h(() => {
        G(n, "background-color", e(K)), G(n, "width", e(N)), F(P, "aria-disabled", e(D)), F(P, "data-show", !e(D) && e(I)), G(P, "width", e(N)), U.disabled = e(D), F(U, "aria-disabled", e(D));
      }), fe("mouseenter", n, Z), fe("mouseleave", n, X), Te(U, () => e(E), (R) => p(E, R)), i(u, d);
    }, B = (u) => {
      var d = Ge(), n = r(d);
      o(n), a(d), h(() => {
        G(d, "background-color", e(K)), G(d, "width", e(N)), G(d, "height", e(N));
      }), i(u, d);
    };
    j(T, (u) => {
      var _a;
      ((_a = e(b)) == null ? void 0 : _a.upload_allowed) && !t.disableUpload ? u(C) : u(B, false);
    });
  }
  a(c), i($, c), ne();
}
var Je = Ie('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M7.864 4.243A7.5 7.5 0 0119.5 10.5c0 2.92-.556 5.709-1.568 8.268M5.742 6.364A7.465 7.465 0 004.5 10.5a7.464 7.464 0 01-1.15 3.993m1.989 3.559A11.209 11.209 0 008.25 10.5a3.75 3.75 0 117.5 0c0 .527-.021 1.049-.064 1.565M12 10.5a14.94 14.94 0 01-3.6 9.75m6.633-4.596a18.666 18.666 0 01-2.485 5.33"></path></svg>');
function Qe($, t) {
  let o = Y(t, "opacity", 3, 0.9), s = Y(t, "width", 3, "1.5rem"), x = Y(t, "color", 3, "currentColor");
  var f = Je();
  F(f, "stroke-width", 2), h(() => {
    F(f, "stroke", x()), F(f, "width", s()), F(f, "opacity", o());
  }), i($, f);
}
var We = k("<div><!></div>"), Xe = k('<div class="row svelte-165b9or"><div></div> <div class="deleteBtn svelte-165b9or"><!></div></div>'), Ye = k('<div class="keyContainer svelte-165b9or"><div class="row svelte-165b9or"><span class="label svelte-165b9or"> </span> <div class="nameUv svelte-165b9or"><b> </b> <!></div></div> <div class="row svelte-165b9or"><span class="label svelte-165b9or"> </span> <span class="font-mono"> </span></div> <div class="row svelte-165b9or"><span class="label svelte-165b9or"> </span> <span class="font-mono"> </span></div> <!></div>');
function Tt($, t) {
  le(t, true);
  let o = ve();
  var s = Ye(), x = r(s), f = r(x), L = r(f, true);
  a(f);
  var A = g(f, 2), b = r(A), y = r(b, true);
  a(b);
  var m = g(b, 2);
  {
    var _ = (c) => {
      Ae(c, { get text() {
        return o.account.userVerifiedTooltip;
      }, children: (T, C) => {
        var B = We();
        G(B, "margin-bottom", "-.25rem");
        var u = r(B);
        Qe(u, { width: 18, color: "var(--col-acnt)" }), a(B), i(T, B);
      }, $$slots: { default: true } });
    };
    j(m, (c) => {
      t.passkey.user_verified && c(_);
    });
  }
  a(A), a(x);
  var I = g(x, 2), D = r(I), E = r(D, true);
  a(D);
  var K = g(D, 2), N = r(K, true);
  a(K), a(I);
  var M = g(I, 2), O = r(M), Z = r(O, true);
  a(O);
  var X = g(O, 2), v = r(X, true);
  a(X), a(M);
  var z = g(M, 2);
  {
    var l = (c) => {
      var T = Xe(), C = g(r(T), 2), B = r(C);
      ie(B, { level: -3, onclick: () => t.onDelete(t.passkey.name), children: (u, d) => {
        Q();
        var n = J();
        h(() => w(n, o.common.delete)), i(u, n);
      }, $$slots: { default: true } }), a(C), a(T), i(c, T);
    };
    j(z, (c) => {
      t.showDelete && c(l);
    });
  }
  a(s), h((c, T) => {
    w(L, o.mfa.passkeyName), w(y, t.passkey.name), w(E, o.mfa.registerd), w(N, c), w(Z, o.mfa.lastUsed), w(v, T);
  }, [() => te(t.passkey.registered), () => te(t.passkey.last_used)]), i($, s), ne();
}
var Ze = k('<div class="device-header svelte-9n911e"><div class="device-head font-mono svelte-9n911e"> </div></div>'), et = k('<div class="saveButton svelte-9n911e"><!></div>'), tt = k("<div><!></div>"), rt = k("<div> </div>"), at = k('<div class="device svelte-9n911e"><!> <div class="row svelte-9n911e"><!> <!></div> <!> <!> <!> <!></div>');
function st($, t) {
  le(t, true);
  let o = Y(t, "device", 15), s = ve(), x = H(W(Pe(() => o().name)));
  async function f() {
    let A = { device_id: o().id, name: e(x) }, b = await ze(`/auth/v1/users/${t.userId}/devices`, A);
    b.error ? console.error(b.error) : o(o().name = e(x), true);
  }
  async function L() {
    let A = { device_id: o().id }, b = await ge(`/auth/v1/users/${t.userId}/devices`, A);
    b.error ? console.error(b.error) : o(o().refresh_exp = void 0, true);
  }
  $e($, { summary: (y) => {
    var m = Ze(), _ = r(m), I = r(_, true);
    a(_), a(m), h(() => w(I, o().name)), i(y, m);
  }, details: (y) => {
    var m = at(), _ = r(m);
    ee(_, { get label() {
      return s.account.deviceId;
    }, mono: true, children: (v, z) => {
      Q();
      var l = J();
      h(() => w(l, o().id)), i(v, l);
    } });
    var I = g(_, 2), D = r(I);
    Se(D, { autocomplete: "off", width: "17rem", get label() {
      return s.account.deviceName;
    }, get placeholder() {
      return s.account.deviceName;
    }, onEnter: f, pattern: Le, required: true, get value() {
      return e(x);
    }, set value(v) {
      p(x, W(v));
    }, children: (v, z) => {
      Q();
      var l = J();
      h(() => w(l, s.account.deviceName)), i(v, l);
    }, $$slots: { default: true } });
    var E = g(D, 2);
    {
      var K = (v) => {
        var z = et(), l = r(z);
        ie(l, { onclick: f, children: (c, T) => {
          Q();
          var C = J();
          h(() => w(C, s.common.save)), i(c, C);
        }, $$slots: { default: true } }), a(z), i(v, z);
      };
      j(E, (v) => {
        e(x) !== o().name && v(K);
      });
    }
    a(I);
    var N = g(I, 2);
    ee(N, { get label() {
      return s.account.regDate;
    }, children: (v, z) => {
      Q();
      var l = J();
      h((c) => w(l, c), [() => te(o().created)]), i(v, l);
    } });
    var M = g(N, 2);
    ee(M, { get label() {
      return s.account.accessExp;
    }, children: (v, z) => {
      Q();
      var l = J();
      h((c) => w(l, c), [() => te(o().access_exp)]), i(v, l);
    } });
    var O = g(M, 2);
    {
      var Z = (v) => {
        ee(v, { get label() {
          return s.account.accessRenew;
        }, button: (l) => {
          ie(l, { get ariaLabel() {
            return s.common.delete;
          }, invisible: true, onclick: L, children: (c, T) => {
            var C = tt(), B = r(C);
            pe(B, {}), a(C), h(() => F(C, "title", s.common.delete)), i(c, C);
          }, $$slots: { default: true } });
        }, children: (l, c) => {
          var T = rt(), C = r(T, true);
          a(T), h((B) => w(C, B), [() => te(o().refresh_exp)]), i(l, T);
        } });
      };
      j(O, (v) => {
        o().refresh_exp && v(Z);
      });
    }
    var X = g(O, 2);
    ee(X, { get label() {
      return s.account.regIp;
    }, children: (v, z) => {
      Q();
      var l = J();
      h(() => w(l, o().peer_ip)), i(v, l);
    } }), a(m), i(y, m);
  }, $$slots: { summary: true, details: true } }), ne();
}
var ot = k('<div class="head svelte-w0g6eh"> </div> <div class="devices svelte-w0g6eh"><!></div>', 1);
function Ct($, t) {
  le(t, true);
  let o = ve(), s = H(W([]));
  se(() => {
    x();
  });
  async function x() {
    let _ = await he(`/auth/v1/users/${t.userId}/devices`);
    _.body ? p(s, W(_.body)) : console.error("error fetching devices: " + _.error);
  }
  var f = ot(), L = oe(f), A = r(L, true);
  a(L);
  var b = g(L, 2), y = r(b);
  {
    var m = (_) => {
      var I = _e(), D = oe(I);
      Be(D, 19, () => e(s), (E) => E.id, (E, K, N) => {
        st(E, { get userId() {
          return t.userId;
        }, get device() {
          return e(s)[e(N)];
        }, set device(M) {
          e(s)[e(N)] = M;
        } });
      }), i(_, I);
    };
    j(y, (_) => {
      t.userId && _(m);
    });
  }
  a(b), h(() => w(A, (o == null ? void 0 : o.account.devicesDesc) || "Devices linked to this account")), i($, f), ne();
}
export {
  Ct as D,
  Dt as U,
  Tt as a
};
