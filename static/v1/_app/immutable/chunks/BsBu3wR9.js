import { n as Z, a as d, t as I, e as B, d as ee } from "./BH6NCLk-.js";
import { t as h, p as H, c as t, r, s as u, a as J, a7 as L, k as X, O as te, l as Y, j as w, aa as re, f as W } from "./CvlvO1XB.js";
import { s as n } from "./CTI4QPiR.js";
import { i as F } from "./BUO_AUgz.js";
import { a as ae, B as z } from "./DMkkW5Nn.js";
import { l as j } from "./B8wC3kJv.js";
import { s as C } from "./BMbqVy6X.js";
import { p as V } from "./C6SR4G2t.js";
import { T as se } from "./BmpEzKyJ.js";
import { u as K } from "./BQ1-pLIs.js";
import { e as oe } from "./BpWRzPRQ.js";
import { p as q } from "./Wh68IIk2.js";
import { d as ve, c as ie, f as de } from "./DYtiVhoA.js";
import { I as ne } from "./Vi3uK7uO.js";
import { I as le } from "./CeeD0cV_.js";
import { E as ce } from "./CcOY4mXK.js";
import { g as ue } from "./BRCxk8by.js";
import { L as U } from "./CE2_6siz.js";
var me = Z('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M7.864 4.243A7.5 7.5 0 0119.5 10.5c0 2.92-.556 5.709-1.568 8.268M5.742 6.364A7.465 7.465 0 004.5 10.5a7.464 7.464 0 01-1.15 3.993m1.989 3.559A11.209 11.209 0 008.25 10.5a3.75 3.75 0 117.5 0c0 .527-.021 1.049-.064 1.565M12 10.5a14.94 14.94 0 01-3.6 9.75m6.633-4.596a18.666 18.666 0 01-2.485 5.33"></path></svg>');
function fe($, a) {
  let e = V(a, "opacity", 3, 0.9), s = V(a, "width", 3, "1.5rem"), m = V(a, "color", 3, "currentColor");
  var l = me();
  C(l, "stroke-width", 2), h(() => {
    C(l, "stroke", m()), C(l, "width", s()), C(l, "opacity", e());
  }), d($, l);
}
var _e = I("<div><!></div>"), pe = I('<div class="row svelte-165b9or"><div></div> <div class="deleteBtn svelte-165b9or"><!></div></div>'), he = I('<div class="keyContainer svelte-165b9or"><div class="row svelte-165b9or"><span class="label svelte-165b9or"> </span> <div class="nameUv svelte-165b9or"><b> </b> <!></div></div> <div class="row svelte-165b9or"><span class="label svelte-165b9or"> </span> <span class="font-mono"> </span></div> <div class="row svelte-165b9or"><span class="label svelte-165b9or"> </span> <span class="font-mono"> </span></div> <!></div>');
function Oe($, a) {
  H(a, true);
  let e = K();
  var s = he(), m = t(s), l = t(m), N = t(l, true);
  r(l);
  var k = u(l, 2), f = t(k), E = t(f, true);
  r(f);
  var g = u(f, 2);
  {
    var c = (i) => {
      se(i, { get text() {
        return e.account.userVerifiedTooltip;
      }, children: (_, p) => {
        var y = _e();
        ae(y, "margin-bottom", "-.25rem");
        var O = t(y);
        fe(O, { width: 18, color: "var(--col-acnt)" }), r(y), d(_, y);
      }, $$slots: { default: true } });
    };
    F(g, (i) => {
      a.passkey.user_verified && i(c);
    });
  }
  r(k), r(m);
  var b = u(m, 2), P = t(b), T = t(P, true);
  r(P);
  var R = u(P, 2), A = t(R, true);
  r(R), r(b);
  var D = u(b, 2), M = t(D), G = t(M, true);
  r(M);
  var S = u(M, 2), v = t(S, true);
  r(S), r(D);
  var x = u(D, 2);
  {
    var o = (i) => {
      var _ = pe(), p = u(t(_), 2), y = t(p);
      z(y, { level: -3, onclick: () => a.onDelete(a.passkey.name), children: (O, Pe) => {
        L();
        var Q = B();
        h(() => n(Q, e.common.delete)), d(O, Q);
      }, $$slots: { default: true } }), r(p), r(_), d(i, _);
    };
    F(x, (i) => {
      a.showDelete && i(o);
    });
  }
  r(s), h((i, _) => {
    n(N, e.mfa.passkeyName), n(E, a.passkey.name), n(T, e.mfa.registerd), n(A, i), n(G, e.mfa.lastUsed), n(v, _);
  }, [() => j(a.passkey.registered), () => j(a.passkey.last_used)]), d($, s), J();
}
var be = I('<div class="device-header svelte-9n911e"><div class="device-head font-mono svelte-9n911e"> </div></div>'), ge = I('<div class="saveButton svelte-9n911e"><!></div>'), xe = I("<div><!></div>"), ye = I("<div> </div>"), ke = I('<div class="device svelte-9n911e"><!> <div class="row svelte-9n911e"><!> <!></div> <!> <!> <!> <!></div>');
function we($, a) {
  H(a, true);
  let e = V(a, "device", 15), s = K(), m = X(q(te(() => e().name)));
  async function l() {
    let k = { device_id: e().id, name: w(m) }, f = await ve(`/auth/v1/users/${a.userId}/devices`, k);
    f.error ? console.error(f.error) : e(e().name = w(m), true);
  }
  async function N() {
    let k = { device_id: e().id }, f = await ie(`/auth/v1/users/${a.userId}/devices`, k);
    f.error ? console.error(f.error) : e(e().refresh_exp = void 0, true);
  }
  ce($, { summary: (E) => {
    var g = be(), c = t(g), b = t(c, true);
    r(c), r(g), h(() => n(b, e().name)), d(E, g);
  }, details: (E) => {
    var g = ke(), c = t(g);
    U(c, { get label() {
      return s.account.deviceId;
    }, mono: true, children: (v, x) => {
      L();
      var o = B();
      h(() => n(o, e().id)), d(v, o);
    } });
    var b = u(c, 2), P = t(b);
    le(P, { autocomplete: "off", width: "17rem", get label() {
      return s.account.deviceName;
    }, get placeholder() {
      return s.account.deviceName;
    }, onEnter: l, pattern: ue, required: true, get value() {
      return w(m);
    }, set value(v) {
      Y(m, q(v));
    }, children: (v, x) => {
      L();
      var o = B();
      h(() => n(o, s.account.deviceName)), d(v, o);
    }, $$slots: { default: true } });
    var T = u(P, 2);
    {
      var R = (v) => {
        var x = ge(), o = t(x);
        z(o, { onclick: l, children: (i, _) => {
          L();
          var p = B();
          h(() => n(p, s.common.save)), d(i, p);
        }, $$slots: { default: true } }), r(x), d(v, x);
      };
      F(T, (v) => {
        w(m) !== e().name && v(R);
      });
    }
    r(b);
    var A = u(b, 2);
    U(A, { get label() {
      return s.account.regDate;
    }, children: (v, x) => {
      L();
      var o = B();
      h((i) => n(o, i), [() => j(e().created)]), d(v, o);
    } });
    var D = u(A, 2);
    U(D, { get label() {
      return s.account.accessExp;
    }, children: (v, x) => {
      L();
      var o = B();
      h((i) => n(o, i), [() => j(e().access_exp)]), d(v, o);
    } });
    var M = u(D, 2);
    {
      var G = (v) => {
        U(v, { get label() {
          return s.account.accessRenew;
        }, button: (o) => {
          z(o, { get ariaLabel() {
            return s.common.delete;
          }, invisible: true, onclick: N, children: (i, _) => {
            var p = xe(), y = t(p);
            ne(y, {}), r(p), h(() => C(p, "title", s.common.delete)), d(i, p);
          }, $$slots: { default: true } });
        }, children: (o, i) => {
          var _ = ye(), p = t(_, true);
          r(_), h((y) => n(p, y), [() => j(e().refresh_exp)]), d(o, _);
        } });
      };
      F(M, (v) => {
        e().refresh_exp && v(G);
      });
    }
    var S = u(M, 2);
    U(S, { get label() {
      return s.account.regIp;
    }, children: (v, x) => {
      L();
      var o = B();
      h(() => n(o, e().peer_ip)), d(v, o);
    } }), r(g), d(E, g);
  }, $$slots: { summary: true, details: true } }), J();
}
var Ie = I('<div class="head svelte-w0g6eh"> </div> <div class="devices svelte-w0g6eh"><!></div>', 1);
function ze($, a) {
  H(a, true);
  let e = K(), s = X(q([]));
  re(() => {
    m();
  });
  async function m() {
    let c = await de(`/auth/v1/users/${a.userId}/devices`);
    c.body ? Y(s, q(c.body)) : console.error("error fetching devices: " + c.error);
  }
  var l = Ie(), N = W(l), k = t(N, true);
  r(N);
  var f = u(N, 2), E = t(f);
  {
    var g = (c) => {
      var b = ee(), P = W(b);
      oe(P, 19, () => w(s), (T) => T.id, (T, R, A) => {
        we(T, { get userId() {
          return a.userId;
        }, get device() {
          return w(s)[w(A)];
        }, set device(D) {
          w(s)[w(A)] = D;
        } });
      }), d(c, b);
    };
    F(E, (c) => {
      a.userId && c(g);
    });
  }
  r(f), h(() => n(k, (e == null ? void 0 : e.account.devicesDesc) || "Devices linked to this account")), d($, l), J();
}
export {
  ze as D,
  Oe as U
};
