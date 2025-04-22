import { t as k, a as i, d as _e, n as $e, e as J } from "./BxmJRzoY.js";
import { j as O, k as e, l as p, p as ve, a4 as ue, a5 as oe, c as r, r as a, a as ne, f as ie, s as b, t as h, a3 as Q, a1 as pe, F as Ue } from "./w0HvPX0p.js";
import { e as fe, s as w, b as De } from "./BzP2S3Z_.js";
import { i as K } from "./iO9_dPNE.js";
import { s as E, a as te, b as Te } from "./BdbQ6g_y.js";
import { a as Ce } from "./Cxw7xmE1.js";
import { p as X } from "./C6GSeq7M.js";
import { I as Ee } from "./85fl2JjN.js";
import { u as de } from "./N6FgGI8m.js";
import { i as Fe, g as Ne, C as ze, l as re } from "./B21bTIl7.js";
import { I as he } from "./QCVRj9pj.js";
import { B as le } from "./C4AV2CoD.js";
import { f as be, c as ge, d as Ae } from "./UPFlzoow.js";
import { T as Be } from "./DogAGR7A.js";
import { e as Se } from "./S81raU5Y.js";
import { I as Le } from "./Bk5EVqw2.js";
import { E as Me } from "./CvlP2s8z.js";
import { g as Re } from "./gfDO7tLr.js";
import { L as ee } from "./D8MSWRQv.js";
let me = O(void 0);
function je() {
  return !e(me) && Fe() && be("/auth/v1/users/picture_config").then((S) => {
    p(me, S.body, true);
  }), { get() {
    return e(me);
  } };
}
var Ke = k("<span> </span>"), Ve = k('<img loading="lazy" class="absolute svelte-1e810u7" aria-label="Avatar" alt="" width="100%">'), qe = k('<div class="errLimit svelte-1e810u7"> </div>'), Ge = k("<div><!></div>"), He = k('<div class="relative"><div class="delete svelte-1e810u7"><!></div></div>'), Oe = k('<form class="avatar svelte-1e810u7" aria-dropeffect="move" aria-label="Upload"><label class="svelte-1e810u7"><!></label> <input type="file" aria-hidden="true" class="svelte-1e810u7"> <!> <!></form> <!>', 1), Je = k('<span class="avatar svelte-1e810u7"><!></span>'), Qe = k('<div class="container svelte-1e810u7"><!></div>');
function Dt(S, t) {
  ve(t, true);
  const o = (c) => {
    var u = _e(), v = ie(u);
    {
      var A = (B) => {
        var M = Ke(), G = r(M, true);
        a(M), h((ae) => {
          Te(M, 1, `absolute font-mono font-${y()} noselect`, "svelte-1e810u7"), w(G, ae);
        }, [() => t.fallbackCharacters.toUpperCase()]), i(B, M);
      }, D = (B) => {
        var M = Ve();
        h(() => E(M, "src", `/auth/v1/users/${t.userId}/picture/${s()}`)), fe("error", M, q), De(M), i(B, M);
      };
      K(v, (B) => {
        e(x) || !s() ? B(A) : B(D, false);
      });
    }
    i(c, u);
  };
  let s = X(t, "pictureId", 15), y = X(t, "size", 3, "medium");
  const f = Ne(), L = "image/png, image/jpeg, image/webp";
  let N = de(), g = ue(() => je().get()), x = O(""), m = O(""), _, I = O(false), P = O(false), T = O(void 0), V = ue(R), C = ue(() => {
    switch (y()) {
      case "small":
        return "2.25rem";
      case "medium":
        return "3rem";
      default:
        return "192px";
    }
  });
  oe(() => {
    e(x) && console.error(e(x));
  }), oe(() => {
    e(m) && (_ && clearTimeout(_), _ = setTimeout(() => {
      p(m, "");
    }, 5e3));
  }), oe(() => {
    e(T) && e(T).length > 0 && l(e(T));
  });
  function R() {
    return `hsl(${n(t.fallbackCharacters)}, 50%, 50%)`;
  }
  function q(c) {
    c.type === "error" && p(x, "Input Error");
  }
  function Y() {
    p(I, true);
  }
  function W() {
    p(I, false);
  }
  function n(c) {
    let u = [1, 255, 3, 13, 19];
    for (let v = 0; v < 5; v++) {
      let A = c.charCodeAt(v);
      if (A) u[v] = A;
      else break;
    }
    return (u[0] * (64 + u[1]) + 4 * u[0] + (u[2] * u[3] - u[4])) % 360;
  }
  async function F() {
    if (p(x, ""), p(m, ""), !s()) return;
    let c = await ge(`/auth/v1/users/${t.userId}/picture/${s()}`);
    c.error ? p(x, c.error.message, true) : s(void 0);
  }
  async function l(c) {
    if (p(x, ""), p(m, ""), !e(g)) return;
    p(P, true);
    let u = `/auth/v1/users/${t.userId}/picture`;
    for (let v of c) {
      if (!L.includes(v.type)) {
        p(x, "Invalid File Format, allowed: " + L);
        break;
      }
      if (v.size > e(g).content_len_limit) {
        p(m, `${N.common.maxFileSize}: ${e(g).content_len_limit / 1024 / 1024} MB`);
        break;
      }
      let A = new FormData();
      A.append(v.name, v);
      let D = await fetch(u, { method: "PUT", headers: { "x-csrf-token": localStorage.getItem(ze) || "" }, body: A });
      if (D.ok) s(await D.text());
      else {
        let B = await D.json();
        D.status === 406 ? alert("max size" + B.message) : p(x, B.message || "Upload Error", true);
      }
    }
    p(T, void 0), p(P, false);
  }
  var d = Qe(), $ = r(d);
  {
    var U = (c) => {
      var u = Oe(), v = ie(u);
      let A;
      var D = r(v);
      E(D, "for", f), E(D, "aria-controls", f);
      let B;
      var M = r(D);
      Ee(M, { get width() {
        return e(C);
      } }), a(D);
      var G = b(D, 2);
      E(G, "id", f), E(G, "accept", L);
      var ae = b(G, 2);
      {
        var we = (j) => {
          var H = qe();
          let Z;
          var ce = r(H, true);
          a(H), h(() => {
            Z = te(H, "", Z, { width: e(C) }), w(ce, e(m));
          }), i(j, H);
        };
        K(ae, (j) => {
          e(m) && j(we);
        });
      }
      var ye = b(ae, 2);
      o(ye), a(v);
      var xe = b(v, 2);
      {
        var ke = (j) => {
          var H = He(), Z = r(H), ce = r(Z);
          le(ce, { invisible: true, onclick: F, children: (Ie, vt) => {
            var se = Ge(), Pe = r(se);
            he(Pe, {}), a(se), h(() => E(se, "title", N.common.delete)), i(Ie, se);
          }, $$slots: { default: true } }), a(Z), a(H), i(j, H);
        };
        K(xe, (j) => {
          y() === "large" && s() && j(ke);
        });
      }
      h(() => {
        A = te(v, "", A, { "background-color": e(V), width: e(C) }), E(D, "aria-disabled", e(P)), E(D, "data-show", !e(P) && e(I)), B = te(D, "", B, { width: e(C) }), G.disabled = e(P), E(G, "aria-disabled", e(P));
      }), fe("mouseenter", v, Y), fe("mouseleave", v, W), Ce(G, () => e(T), (j) => p(T, j)), i(c, u);
    }, z = (c) => {
      var u = Je();
      let v;
      var A = r(u);
      o(A), a(u), h(() => v = te(u, "", v, { "background-color": e(V), width: e(C), height: e(C) })), i(c, u);
    };
    K($, (c) => {
      var _a;
      ((_a = e(g)) == null ? void 0 : _a.upload_allowed) && !t.disableUpload ? c(U) : c(z, false);
    });
  }
  a(d), i(S, d), ne();
}
var We = $e('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M7.864 4.243A7.5 7.5 0 0119.5 10.5c0 2.92-.556 5.709-1.568 8.268M5.742 6.364A7.465 7.465 0 004.5 10.5a7.464 7.464 0 01-1.15 3.993m1.989 3.559A11.209 11.209 0 008.25 10.5a3.75 3.75 0 117.5 0c0 .527-.021 1.049-.064 1.565M12 10.5a14.94 14.94 0 01-3.6 9.75m6.633-4.596a18.666 18.666 0 01-2.485 5.33"></path></svg>');
function Xe(S, t) {
  let o = X(t, "opacity", 3, 0.9), s = X(t, "width", 3, "1.5rem"), y = X(t, "color", 3, "currentColor");
  var f = We();
  E(f, "stroke-width", 2), h(() => {
    E(f, "stroke", y()), E(f, "width", s()), E(f, "opacity", o());
  }), i(S, f);
}
var Ye = k("<div><!></div>"), Ze = k('<div class="row svelte-165b9or"><div></div> <div class="deleteBtn svelte-165b9or"><!></div></div>'), et = k('<div class="keyContainer svelte-165b9or"><div class="row svelte-165b9or"><span class="label svelte-165b9or"> </span> <div class="nameUv svelte-165b9or"><b> </b> <!></div></div> <div class="row svelte-165b9or"><span class="label svelte-165b9or"> </span> <span class="font-mono"> </span></div> <div class="row svelte-165b9or"><span class="label svelte-165b9or"> </span> <span class="font-mono"> </span></div> <!></div>');
function Tt(S, t) {
  ve(t, true);
  let o = de();
  var s = et(), y = r(s), f = r(y), L = r(f, true);
  a(f);
  var N = b(f, 2), g = r(N), x = r(g, true);
  a(g);
  var m = b(g, 2);
  {
    var _ = (d) => {
      Be(d, { get text() {
        return o.account.userVerifiedTooltip;
      }, children: ($, U) => {
        var z = Ye();
        te(z, "", {}, { "margin-bottom": "-.25rem" });
        var c = r(z);
        Xe(c, { width: 18, color: "var(--col-acnt)" }), a(z), i($, z);
      }, $$slots: { default: true } });
    };
    K(m, (d) => {
      t.passkey.user_verified && d(_);
    });
  }
  a(N), a(y);
  var I = b(y, 2), P = r(I), T = r(P, true);
  a(P);
  var V = b(P, 2), C = r(V, true);
  a(V), a(I);
  var R = b(I, 2), q = r(R), Y = r(q, true);
  a(q);
  var W = b(q, 2), n = r(W, true);
  a(W), a(R);
  var F = b(R, 2);
  {
    var l = (d) => {
      var $ = Ze(), U = b(r($), 2), z = r(U);
      le(z, { level: -3, onclick: () => t.onDelete(t.passkey.name), children: (c, u) => {
        Q();
        var v = J();
        h(() => w(v, o.common.delete)), i(c, v);
      }, $$slots: { default: true } }), a(U), a($), i(d, $);
    };
    K(F, (d) => {
      t.showDelete && d(l);
    });
  }
  a(s), h((d, $) => {
    w(L, o.mfa.passkeyName), w(x, t.passkey.name), w(T, o.mfa.registerd), w(C, d), w(Y, o.mfa.lastUsed), w(n, $);
  }, [() => re(t.passkey.registered), () => re(t.passkey.last_used)]), i(S, s), ne();
}
var tt = k('<div class="device-header svelte-9n911e"><div class="device-head font-mono svelte-9n911e"> </div></div>'), rt = k('<div class="saveButton svelte-9n911e"><!></div>'), at = k("<div><!></div>"), st = k("<div> </div>"), ot = k('<div class="device svelte-9n911e"><!> <div class="row svelte-9n911e"><!> <!></div> <!> <!> <!> <!></div>');
function it(S, t) {
  ve(t, true);
  let o = X(t, "device", 15), s = de(), y = O(pe(Ue(() => o().name)));
  async function f() {
    let N = { device_id: o().id, name: e(y) }, g = await Ae(`/auth/v1/users/${t.userId}/devices`, N);
    g.error ? console.error(g.error) : o(o().name = e(y), true);
  }
  async function L() {
    let N = { device_id: o().id }, g = await ge(`/auth/v1/users/${t.userId}/devices`, N);
    g.error ? console.error(g.error) : o(o().refresh_exp = void 0, true);
  }
  Me(S, { summary: (x) => {
    var m = tt(), _ = r(m), I = r(_, true);
    a(_), a(m), h(() => w(I, o().name)), i(x, m);
  }, details: (x) => {
    var m = ot(), _ = r(m);
    ee(_, { get label() {
      return s.account.deviceId;
    }, mono: true, children: (n, F) => {
      Q();
      var l = J();
      h(() => w(l, o().id)), i(n, l);
    }, $$slots: { default: true } });
    var I = b(_, 2), P = r(I);
    Le(P, { autocomplete: "off", width: "17rem", get label() {
      return s.account.deviceName;
    }, get placeholder() {
      return s.account.deviceName;
    }, onEnter: f, pattern: Re, required: true, get value() {
      return e(y);
    }, set value(n) {
      p(y, n, true);
    }, children: (n, F) => {
      Q();
      var l = J();
      h(() => w(l, s.account.deviceName)), i(n, l);
    }, $$slots: { default: true } });
    var T = b(P, 2);
    {
      var V = (n) => {
        var F = rt(), l = r(F);
        le(l, { onclick: f, children: (d, $) => {
          Q();
          var U = J();
          h(() => w(U, s.common.save)), i(d, U);
        }, $$slots: { default: true } }), a(F), i(n, F);
      };
      K(T, (n) => {
        e(y) !== o().name && n(V);
      });
    }
    a(I);
    var C = b(I, 2);
    ee(C, { get label() {
      return s.account.regDate;
    }, children: (n, F) => {
      Q();
      var l = J();
      h((d) => w(l, d), [() => re(o().created)]), i(n, l);
    }, $$slots: { default: true } });
    var R = b(C, 2);
    ee(R, { get label() {
      return s.account.accessExp;
    }, children: (n, F) => {
      Q();
      var l = J();
      h((d) => w(l, d), [() => re(o().access_exp)]), i(n, l);
    }, $$slots: { default: true } });
    var q = b(R, 2);
    {
      var Y = (n) => {
        ee(n, { get label() {
          return s.account.accessRenew;
        }, button: (l) => {
          le(l, { get ariaLabel() {
            return s.common.delete;
          }, invisible: true, onclick: L, children: (d, $) => {
            var U = at(), z = r(U);
            he(z, {}), a(U), h(() => E(U, "title", s.common.delete)), i(d, U);
          }, $$slots: { default: true } });
        }, children: (l, d) => {
          var $ = st(), U = r($, true);
          a($), h((z) => w(U, z), [() => re(o().refresh_exp)]), i(l, $);
        }, $$slots: { button: true, default: true } });
      };
      K(q, (n) => {
        o().refresh_exp && n(Y);
      });
    }
    var W = b(q, 2);
    ee(W, { get label() {
      return s.account.regIp;
    }, children: (n, F) => {
      Q();
      var l = J();
      h(() => w(l, o().peer_ip)), i(n, l);
    }, $$slots: { default: true } }), a(m), i(x, m);
  }, $$slots: { summary: true, details: true } }), ne();
}
var lt = k('<div class="head svelte-w0g6eh"> </div> <div class="devices svelte-w0g6eh"><!></div>', 1);
function Ct(S, t) {
  ve(t, true);
  let o = de(), s = O(pe([]));
  oe(() => {
    y();
  });
  async function y() {
    let _ = await be(`/auth/v1/users/${t.userId}/devices`);
    _.body ? p(s, _.body, true) : console.error("error fetching devices: " + _.error);
  }
  var f = lt(), L = ie(f), N = r(L, true);
  a(L);
  var g = b(L, 2), x = r(g);
  {
    var m = (_) => {
      var I = _e(), P = ie(I);
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
  a(g), h(() => w(N, (o == null ? void 0 : o.account.devicesDesc) || "Devices linked to this account")), i(S, f), ne();
}
export {
  Ct as D,
  Dt as U,
  Tt as a
};
