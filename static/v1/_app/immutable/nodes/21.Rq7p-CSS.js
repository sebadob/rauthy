import { n as ge, a as c, t as O, e as U, d as se } from "../chunks/CWf9OOFK.js";
import { t as _, p as ae, k as w, c as C, r as I, a as le, s as p, j as s, aa as xe, f as K, a8 as j, l as a, a6 as we, ab as Pe } from "../chunks/nlANaGLT.js";
import { s as P } from "../chunks/BmMHVVX3.js";
import { i as W } from "../chunks/DOjUa9u5.js";
import { e as re } from "../chunks/B6azywu7.js";
import { p as d } from "../chunks/5u5qd9TD.js";
import { l as X, r as Se } from "../chunks/B21bTIl7.js";
import { B as N } from "../chunks/BEQMYyDu.js";
import { P as oe } from "../chunks/mxwYGKSe.js";
import { s as Z, b as ye, a as Ce } from "../chunks/CZv_AhHu.js";
import { C as Ie } from "../chunks/gvhP3O7i.js";
import { E as be } from "../chunks/D8CbPIAy.js";
import { T as J } from "../chunks/C8iV2OFO.js";
import { u as De } from "../chunks/DwvS5LQk.js";
import { u as ne } from "../chunks/Bb8ybDgy.js";
import { L as M } from "../chunks/x5MgfX0R.js";
import { p as Y } from "../chunks/uWmgYd3Z.js";
import { c as de, f as Te } from "../chunks/BO1A6s0c.js";
import { C as $e } from "../chunks/BSE9YYho.js";
import { O as Le } from "../chunks/Np7i_reI.js";
import { u as ke } from "../chunks/Be0yYLp7.js";
import { P as Oe, f as Fe, a as Ae } from "../chunks/DRk-dg9U.js";
var Ee = ge(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25
            2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0
            0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5
            0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5
            0a48.667 48.667 0 0 0-7.5 0"></path></svg>`);
function ze(B, l) {
  let S = Y(l, "opacity", 3, 0.9), x = Y(l, "width", 3, "1.5rem"), n = Y(l, "color", 3, "hsl(var(--error))");
  var m = Ee();
  Z(m, "stroke-width", 2), _(() => {
    Z(m, "stroke", n()), Z(m, "width", x()), Z(m, "opacity", S());
  }), c(B, m);
}
var Ue = O('<div class="uid font-mono svelte-79owtg"> </div>'), je = O('<div class="date svelte-79owtg"> </div>'), Be = O('<div class="ip font-mono svelte-79owtg"> </div>'), He = O('<div class="trash svelte-79owtg"><!></div>'), qe = O('<div class="header svelte-79owtg"><!> <div class="headerRight svelte-79owtg"><!> <!> <!></div></div>'), Ge = O("<!> <!> <!> <!> <!> <!> <!> <!>", 1), Me = O("<div><!></div>");
function ie(B, l) {
  ae(l, true);
  let S = De(), x = ne(), n = w(d(S.common.copyToClip));
  function m() {
    l.session.remote_ip && (navigator.clipboard.writeText(l.session.remote_ip), a(n, d(x.common.copiedToClip)), setTimeout(() => {
      a(n, d(S.common.copyToClip));
    }, 3e3));
  }
  async function F() {
    let b = await de(`/auth/v1/sessions/id/${l.session.id}`);
    b.error ? console.error(b.error) : l.onDeleted(l.session.id);
  }
  var A = Me();
  let H;
  var E = C(A);
  be(E, { summary: (y) => {
    var u = qe(), $ = C(u);
    J($, { text: "User ID", children: (g, z) => {
      var o = Ue();
      let r;
      var t = C(o, true);
      I(o), _(() => {
        r = Ce(o, "", r, { "margin-left": l.session.user_id ? "" : ".5rem" }), P(t, l.session.user_id);
      }), c(g, o);
    }, $$slots: { default: true } });
    var h = p($, 2), q = C(h);
    J(q, { get text() {
      return x.options.lastSeen;
    }, children: (g, z) => {
      var o = je(), r = C(o, true);
      I(o), _((t) => P(r, t), [() => X(l.session.last_seen)]), c(g, o);
    }, $$slots: { default: true } });
    var L = p(q, 2);
    N(L, { invisible: true, onclick: m, children: (g, z) => {
      const o = xe(() => `IP - ${s(n)}`);
      J(g, { get text() {
        return s(o);
      }, children: (r, t) => {
        var e = Be(), i = C(e, true);
        I(e), _(() => P(i, l.session.remote_ip)), c(r, e);
      }, $$slots: { default: true } });
    }, $$slots: { default: true } });
    var R = p(L, 2);
    N(R, { invisible: true, onclick: F, children: (g, z) => {
      J(g, { get text() {
        return S.common.delete;
      }, children: (o, r) => {
        var t = He(), e = C(t);
        ze(e, { width: "1.2rem" }), I(t), c(o, t);
      }, $$slots: { default: true } });
    }, $$slots: { default: true } }), I(h), I(u), c(y, u);
  }, details: (y) => {
    var u = Ge(), $ = K(u);
    M($, { label: "Session ID", children: (r, t) => {
      j();
      var e = U();
      _(() => P(e, l.session.id)), c(r, e);
    } });
    var h = p($, 2);
    M(h, { label: "User ID", children: (r, t) => {
      j();
      var e = U();
      _(() => P(e, l.session.user_id)), c(r, e);
    } });
    var q = p(h, 2);
    M(q, { get label() {
      return x.options.expires;
    }, children: (r, t) => {
      j();
      var e = U();
      _((i) => P(e, i), [() => X(l.session.exp)]), c(r, e);
    } });
    var L = p(q, 2);
    M(L, { get label() {
      return x.options.lastSeen;
    }, children: (r, t) => {
      j();
      var e = U();
      _((i) => P(e, i), [() => X(l.session.last_seen)]), c(r, e);
    } });
    var R = p(L, 2);
    M(R, { get label() {
      return x.options.state;
    }, children: (r, t) => {
      j();
      var e = U();
      _(() => P(e, l.session.state)), c(r, e);
    } });
    var g = p(R, 2);
    M(g, { label: "IP", children: (r, t) => {
      j();
      var e = U();
      _(() => P(e, l.session.remote_ip)), c(r, e);
    } });
    var z = p(g, 2);
    M(z, { label: "MFA", children: (r, t) => {
      Ie(r, {});
    } });
    var o = p(z, 2);
    N(o, { level: -1, onclick: F, children: (r, t) => {
      j();
      var e = U();
      _(() => P(e, S.common.delete)), c(r, e);
    }, $$slots: { default: true } }), c(y, u);
  }, $$slots: { summary: true, details: true } }), I(A), _((b) => H = ye(A, 1, "container svelte-79owtg", null, H, b), [() => ({ expired: l.session.exp < l.now })]), c(B, A), le();
}
var Re = O('<div class="err"> </div>'), Ve = O('<div class="top svelte-1sljqh8"><!> <div class="btn svelte-1sljqh8"><!></div></div> <!> <div id="sessions"><!></div> <!>', 1);
function We(B, l) {
  ae(l, true);
  let S = ne(), x = w(""), n = w(d([])), m = w(d([])), F = w(d([])), A = w(Date.now() / 1e3), H = w(false), E = w(void 0), b = w(d(Oe)), V = w(false), y = ["User ID", "Session ID", "IP"], u = w(d(y[0])), $ = w(""), h = [S.options.expires, S.options.lastSeen, "Session ID", "User ID", S.options.state, "IP"];
  we(() => {
    L("page_size=" + s(b));
  }), Pe(() => {
    let o = s($).toLowerCase();
    s(H) ? o.length < 3 ? s(V) && (L("page_size=" + s(b)), a(V, false)) : q(o) : o ? s(u) === y[0] ? a(m, d(s(n).filter((r) => {
      var _a;
      return (_a = r.user_id) == null ? void 0 : _a.toLowerCase().includes(o);
    }))) : s(u) === y[1] ? a(m, d(s(n).filter((r) => r.id.toLowerCase().includes(o)))) : s(u) === y[2] && a(m, d(s(n).filter((r) => {
      var _a;
      return (_a = r.remote_ip) == null ? void 0 : _a.toLowerCase().includes(o);
    }))) : a(m, d(s(n)));
  });
  async function q(o) {
    a(E, void 0), a(V, true);
    let r;
    s(u) === y[0] ? r = "userid" : s(u) === y[1] ? r = "sessionid" : r = "ip";
    let t = await Fe({ ty: "session", idx: r, q: o });
    t.body ? a(n, d(t.body)) : console.error(t.error);
  }
  async function L(o) {
    let r = "/auth/v1/sessions";
    o && (r += `?${o}`);
    let t = await Te(r);
    return t.error ? a(x, "Error fetching sessions: " + t.error.message) : t.body && (t.status === 206 ? (a(H, true), a(E, d(t.headers))) : (a(H, false), a(E, void 0)), a(n, d(t.body)), a(A, Date.now() / 1e3)), [t.status, t.headers];
  }
  function R(o, r) {
    let t = r === "up";
    o === h[0] ? s(n).sort((e, i) => t ? e.exp - i.exp : i.exp - e.exp) : o === h[1] ? s(n).sort((e, i) => t ? e.last_seen - i.last_seen : i.last_seen - e.last_seen) : o === h[2] ? s(n).sort((e, i) => t ? e.id.localeCompare(i.id) : i.id.localeCompare(e.id)) : o === h[3] ? s(n).sort((e, i) => e.user_id && i.user_id ? t ? e.user_id.localeCompare(i.user_id) : i.user_id.localeCompare(e.user_id) : e.user_id ? t ? -1 : 1 : t ? 1 : -1) : o === h[4] ? s(n).sort((e, i) => t ? e.state.localeCompare(i.state) : i.state.localeCompare(e.state)) : o === h[5] && s(n).sort((e, i) => e.remote_ip && i.remote_ip ? t ? e.remote_ip.localeCompare(i.remote_ip) : i.remote_ip.localeCompare(e.remote_ip) : e.remote_ip ? t ? -1 : 1 : t ? 1 : -1);
  }
  function g(o) {
    var _a;
    a(n, d(s(n).filter((r) => r.id !== o))), ((_a = ke("admin").get()) == null ? void 0 : _a.id) === o && window.location.reload();
  }
  async function z() {
    let o = await de("/auth/v1/sessions");
    o.error ? a(x, d(o.error.message)) : Se();
  }
  $e(B, { children: (o, r) => {
    var t = Ve(), e = K(t), i = C(e);
    Le(i, { searchOptions: y, orderOptions: h, onChangeOrder: R, searchWidth: "min(25rem, calc(100dvw - 1rem))", get value() {
      return s($);
    }, set value(v) {
      a($, d(v));
    }, get searchOption() {
      return s(u);
    }, set searchOption(v) {
      a(u, d(v));
    } });
    var ee = p(i, 2), ve = C(ee);
    N(ve, { level: -1, onclick: z, children: (v, f) => {
      j();
      var D = U();
      _(() => P(D, S.sessions.invalidateAll)), c(v, D);
    }, $$slots: { default: true } }), I(ee), I(e);
    var te = p(e, 2);
    {
      var ce = (v) => {
        var f = Re(), D = C(f, true);
        I(f), _(() => P(D, s(x))), c(v, f);
      };
      W(te, (v) => {
        s(x) && v(ce);
      });
    }
    var Q = p(te, 2), me = C(Q);
    {
      var ue = (v) => {
        var f = se(), D = K(f);
        re(D, 17, () => s(n), (k) => k.id, (k, T) => {
          ie(k, { get session() {
            return s(T);
          }, get now() {
            return s(A);
          }, onDeleted: g });
        }), c(v, f);
      }, fe = (v) => {
        var f = se(), D = K(f);
        re(D, 17, () => s(F), (k) => k.id, (k, T) => {
          ie(k, { get session() {
            return s(T);
          }, get now() {
            return s(A);
          }, onDeleted: g });
        }), c(v, f);
      };
      W(me, (v) => {
        s(E) ? v(ue) : v(fe, false);
      });
    }
    I(Q);
    var _e = p(Q, 2);
    {
      var pe = (v) => {
        Ae(v, { sspFetch: L, get itemsLength() {
          return s(n).length;
        }, get firstFetchHeaders() {
          return s(E);
        }, get pageSize() {
          return s(b);
        }, set pageSize(f) {
          a(b, d(f));
        } });
      }, he = (v, f) => {
        {
          var D = (T) => {
            oe(T, { get items() {
              return s(n);
            }, set items(G) {
              a(n, d(G));
            }, get itemsPaginated() {
              return s(F);
            }, set itemsPaginated(G) {
              a(F, d(G));
            } });
          }, k = (T) => {
            oe(T, { get items() {
              return s(m);
            }, set items(G) {
              a(m, d(G));
            }, get itemsPaginated() {
              return s(F);
            }, set itemsPaginated(G) {
              a(F, d(G));
            } });
          };
          W(v, (T) => {
            s(H) ? T(D) : T(k, false);
          }, f);
        }
      };
      W(_e, (v) => {
        s(E) ? v(pe) : v(he, false);
      });
    }
    c(o, t);
  } }), le();
}
function _t(B) {
  We(B, {});
}
export {
  _t as component
};
