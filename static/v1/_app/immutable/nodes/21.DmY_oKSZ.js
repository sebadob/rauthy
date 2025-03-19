import { n as ge, a as u, t as O, e as U, d as se } from "../chunks/DLBGyKVC.js";
import { t as _, p as ae, k as w, c as y, r as C, a as le, s as p, j as s, a9 as xe, f as K, a8 as j, l as a, a6 as we, aa as Pe } from "../chunks/CmQi0fbH.js";
import { s as P } from "../chunks/BjaYyaa_.js";
import { i as W } from "../chunks/C6bK2EJJ.js";
import { e as re } from "../chunks/YQCw2eEa.js";
import { p as d } from "../chunks/B_ggA-0N.js";
import { l as X, r as Se } from "../chunks/B21bTIl7.js";
import { B as N } from "../chunks/DPLO-ozG.js";
import { P as oe } from "../chunks/Z8_Ab8EB.js";
import { s as Z, b as $e, a as ye } from "../chunks/DOlUUCkJ.js";
import { C as Ce } from "../chunks/B7t3YI0Q.js";
import { E as Ie } from "../chunks/BWK4fHb9.js";
import { T as J } from "../chunks/ibGTLHQD.js";
import { u as De } from "../chunks/DGTOa5g8.js";
import { u as ne } from "../chunks/CZf6fJph.js";
import { L as M } from "../chunks/gwKtuDud.js";
import { p as Y } from "../chunks/DNJm3-SG.js";
import { c as de, f as be } from "../chunks/BO1A6s0c.js";
import { C as Te } from "../chunks/Bfi4YUpT.js";
import { O as ke } from "../chunks/BBZ-aIjz.js";
import { u as Le } from "../chunks/CMm1ZRdW.js";
import { P as Oe, f as Fe, a as Ae } from "../chunks/Cvau9dZM.js";
var Ee = ge(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25
            2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0
            0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5
            0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5
            0a48.667 48.667 0 0 0-7.5 0"></path></svg>`);
function ze(B, l) {
  let S = Y(l, "opacity", 3, 0.9), x = Y(l, "width", 3, "1.5rem"), n = Y(l, "color", 3, "hsl(var(--error))");
  var c = Ee();
  Z(c, "stroke-width", 2), _(() => {
    Z(c, "stroke", n()), Z(c, "width", x()), Z(c, "opacity", S());
  }), u(B, c);
}
var Ue = O('<div class="uid font-mono svelte-79owtg"> </div>'), je = O('<div class="date svelte-79owtg"> </div>'), Be = O('<div class="ip font-mono svelte-79owtg"> </div>'), He = O('<div class="trash svelte-79owtg"><!></div>'), qe = O('<div class="header svelte-79owtg"><!> <div class="headerRight svelte-79owtg"><!> <!> <!></div></div>'), Ge = O("<!> <!> <!> <!> <!> <!> <!> <!>", 1), Me = O("<div><!></div>");
function ie(B, l) {
  ae(l, true);
  let S = De(), x = ne(), n = w(d(S.common.copyToClip));
  function c() {
    l.session.remote_ip && (navigator.clipboard.writeText(l.session.remote_ip), a(n, d(x.common.copiedToClip)), setTimeout(() => {
      a(n, d(S.common.copyToClip));
    }, 3e3));
  }
  async function F() {
    let I = await de(`/auth/v1/sessions/id/${l.session.id}`);
    I.error ? console.error(I.error) : l.onDeleted(l.session.id);
  }
  var A = Me();
  let H;
  var E = y(A);
  Ie(E, { summary: ($) => {
    var m = qe(), T = y(m);
    J(T, { text: "User ID", children: (g, z) => {
      var o = Ue();
      let r;
      var t = y(o, true);
      C(o), _(() => {
        r = ye(o, "", r, { "margin-left": l.session.user_id ? "" : ".5rem" }), P(t, l.session.user_id);
      }), u(g, o);
    }, $$slots: { default: true } });
    var h = p(T, 2), q = y(h);
    J(q, { get text() {
      return x.options.lastSeen;
    }, children: (g, z) => {
      var o = je(), r = y(o, true);
      C(o), _((t) => P(r, t), [() => X(l.session.last_seen)]), u(g, o);
    }, $$slots: { default: true } });
    var k = p(q, 2);
    N(k, { invisible: true, onclick: c, children: (g, z) => {
      const o = xe(() => `IP - ${s(n)}`);
      J(g, { get text() {
        return s(o);
      }, children: (r, t) => {
        var e = Be(), i = y(e, true);
        C(e), _(() => P(i, l.session.remote_ip)), u(r, e);
      }, $$slots: { default: true } });
    }, $$slots: { default: true } });
    var R = p(k, 2);
    N(R, { invisible: true, onclick: F, children: (g, z) => {
      J(g, { get text() {
        return S.common.delete;
      }, children: (o, r) => {
        var t = He(), e = y(t);
        ze(e, { width: "1.2rem" }), C(t), u(o, t);
      }, $$slots: { default: true } });
    }, $$slots: { default: true } }), C(h), C(m), u($, m);
  }, details: ($) => {
    var m = Ge(), T = K(m);
    M(T, { label: "Session ID", children: (r, t) => {
      j();
      var e = U();
      _(() => P(e, l.session.id)), u(r, e);
    }, $$slots: { default: true } });
    var h = p(T, 2);
    M(h, { label: "User ID", children: (r, t) => {
      j();
      var e = U();
      _(() => P(e, l.session.user_id)), u(r, e);
    }, $$slots: { default: true } });
    var q = p(h, 2);
    M(q, { get label() {
      return x.options.expires;
    }, children: (r, t) => {
      j();
      var e = U();
      _((i) => P(e, i), [() => X(l.session.exp)]), u(r, e);
    }, $$slots: { default: true } });
    var k = p(q, 2);
    M(k, { get label() {
      return x.options.lastSeen;
    }, children: (r, t) => {
      j();
      var e = U();
      _((i) => P(e, i), [() => X(l.session.last_seen)]), u(r, e);
    }, $$slots: { default: true } });
    var R = p(k, 2);
    M(R, { get label() {
      return x.options.state;
    }, children: (r, t) => {
      j();
      var e = U();
      _(() => P(e, l.session.state)), u(r, e);
    }, $$slots: { default: true } });
    var g = p(R, 2);
    M(g, { label: "IP", children: (r, t) => {
      j();
      var e = U();
      _(() => P(e, l.session.remote_ip)), u(r, e);
    }, $$slots: { default: true } });
    var z = p(g, 2);
    M(z, { label: "MFA", children: (r, t) => {
      Ce(r, { get check() {
        return l.session.is_mfa;
      } });
    }, $$slots: { default: true } });
    var o = p(z, 2);
    N(o, { level: -1, onclick: F, children: (r, t) => {
      j();
      var e = U();
      _(() => P(e, S.common.delete)), u(r, e);
    }, $$slots: { default: true } }), u($, m);
  }, $$slots: { summary: true, details: true } }), C(A), _((I) => H = $e(A, 1, "container svelte-79owtg", null, H, I), [() => ({ expired: l.session.exp < l.now })]), u(B, A), le();
}
var Re = O('<div class="err"> </div>'), Ve = O('<div class="top svelte-1sljqh8"><!> <div class="btn svelte-1sljqh8"><!></div></div> <!> <div id="sessions"><!></div> <!>', 1);
function We(B, l) {
  ae(l, true);
  let S = ne(), x = w(""), n = w(d([])), c = w(d([])), F = w(d([])), A = w(Date.now() / 1e3), H = w(false), E = w(void 0), I = w(d(Oe)), V = w(false), $ = ["User ID", "Session ID", "IP"], m = w(d($[0])), T = w(""), h = [S.options.expires, S.options.lastSeen, "Session ID", "User ID", S.options.state, "IP"];
  we(() => {
    k("page_size=" + s(I));
  }), Pe(() => {
    let o = s(T).toLowerCase();
    s(H) ? o.length < 3 ? s(V) && (k("page_size=" + s(I)), a(V, false)) : q(o) : o ? s(m) === $[0] ? a(c, d(s(n).filter((r) => {
      var _a;
      return (_a = r.user_id) == null ? void 0 : _a.toLowerCase().includes(o);
    }))) : s(m) === $[1] ? a(c, d(s(n).filter((r) => r.id.toLowerCase().includes(o)))) : s(m) === $[2] && a(c, d(s(n).filter((r) => {
      var _a;
      return (_a = r.remote_ip) == null ? void 0 : _a.toLowerCase().includes(o);
    }))) : a(c, d(s(n)));
  });
  async function q(o) {
    a(E, void 0), a(V, true);
    let r;
    s(m) === $[0] ? r = "userid" : s(m) === $[1] ? r = "sessionid" : r = "ip";
    let t = await Fe({ ty: "session", idx: r, q: o });
    t.body ? a(n, d(t.body)) : console.error(t.error);
  }
  async function k(o) {
    let r = "/auth/v1/sessions";
    o && (r += `?${o}`);
    let t = await be(r);
    return t.error ? a(x, "Error fetching sessions: " + t.error.message) : t.body && (t.status === 206 ? (a(H, true), a(E, d(t.headers))) : (a(H, false), a(E, void 0)), a(n, d(t.body)), a(A, Date.now() / 1e3)), [t.status, t.headers];
  }
  function R(o, r) {
    let t = r === "up";
    o === h[0] ? s(n).sort((e, i) => t ? e.exp - i.exp : i.exp - e.exp) : o === h[1] ? s(n).sort((e, i) => t ? e.last_seen - i.last_seen : i.last_seen - e.last_seen) : o === h[2] ? s(n).sort((e, i) => t ? e.id.localeCompare(i.id) : i.id.localeCompare(e.id)) : o === h[3] ? s(n).sort((e, i) => e.user_id && i.user_id ? t ? e.user_id.localeCompare(i.user_id) : i.user_id.localeCompare(e.user_id) : e.user_id ? t ? -1 : 1 : t ? 1 : -1) : o === h[4] ? s(n).sort((e, i) => t ? e.state.localeCompare(i.state) : i.state.localeCompare(e.state)) : o === h[5] && s(n).sort((e, i) => e.remote_ip && i.remote_ip ? t ? e.remote_ip.localeCompare(i.remote_ip) : i.remote_ip.localeCompare(e.remote_ip) : e.remote_ip ? t ? -1 : 1 : t ? 1 : -1);
  }
  function g(o) {
    var _a;
    a(n, d(s(n).filter((r) => r.id !== o))), ((_a = Le("admin").get()) == null ? void 0 : _a.id) === o && window.location.reload();
  }
  async function z() {
    let o = await de("/auth/v1/sessions");
    o.error ? a(x, d(o.error.message)) : Se();
  }
  Te(B, { children: (o, r) => {
    var t = Ve(), e = K(t), i = y(e);
    ke(i, { searchOptions: $, orderOptions: h, onChangeOrder: R, searchWidth: "min(25rem, calc(100dvw - 1rem))", get value() {
      return s(T);
    }, set value(v) {
      a(T, d(v));
    }, get searchOption() {
      return s(m);
    }, set searchOption(v) {
      a(m, d(v));
    } });
    var ee = p(i, 2), ve = y(ee);
    N(ve, { level: -1, onclick: z, children: (v, f) => {
      j();
      var D = U();
      _(() => P(D, S.sessions.invalidateAll)), u(v, D);
    }, $$slots: { default: true } }), C(ee), C(e);
    var te = p(e, 2);
    {
      var ue = (v) => {
        var f = Re(), D = y(f, true);
        C(f), _(() => P(D, s(x))), u(v, f);
      };
      W(te, (v) => {
        s(x) && v(ue);
      });
    }
    var Q = p(te, 2), ce = y(Q);
    {
      var me = (v) => {
        var f = se(), D = K(f);
        re(D, 17, () => s(n), (L) => L.id, (L, b) => {
          ie(L, { get session() {
            return s(b);
          }, get now() {
            return s(A);
          }, onDeleted: g });
        }), u(v, f);
      }, fe = (v) => {
        var f = se(), D = K(f);
        re(D, 17, () => s(F), (L) => L.id, (L, b) => {
          ie(L, { get session() {
            return s(b);
          }, get now() {
            return s(A);
          }, onDeleted: g });
        }), u(v, f);
      };
      W(ce, (v) => {
        s(E) ? v(me) : v(fe, false);
      });
    }
    C(Q);
    var _e = p(Q, 2);
    {
      var pe = (v) => {
        Ae(v, { sspFetch: k, get itemsLength() {
          return s(n).length;
        }, get firstFetchHeaders() {
          return s(E);
        }, get pageSize() {
          return s(I);
        }, set pageSize(f) {
          a(I, d(f));
        } });
      }, he = (v, f) => {
        {
          var D = (b) => {
            oe(b, { get items() {
              return s(n);
            }, set items(G) {
              a(n, d(G));
            }, get itemsPaginated() {
              return s(F);
            }, set itemsPaginated(G) {
              a(F, d(G));
            } });
          }, L = (b) => {
            oe(b, { get items() {
              return s(c);
            }, set items(G) {
              a(c, d(G));
            }, get itemsPaginated() {
              return s(F);
            }, set itemsPaginated(G) {
              a(F, d(G));
            } });
          };
          W(v, (b) => {
            s(H) ? b(D) : b(L, false);
          }, f);
        }
      };
      W(_e, (v) => {
        s(E) ? v(pe) : v(he, false);
      });
    }
    u(o, t);
  } }), le();
}
function _t(B) {
  We(B, {});
}
export {
  _t as component
};
