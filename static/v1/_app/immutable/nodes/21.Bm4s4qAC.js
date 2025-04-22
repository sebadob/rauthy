import { n as ge, a as u, t as L, e as z, d as se } from "../chunks/BxmJRzoY.js";
import { t as f, p as ae, j as x, a1 as R, c as $, r as y, a as le, s as _, a4 as xe, k as s, f as K, a3 as U, l as a, a0 as we, a5 as Pe } from "../chunks/w0HvPX0p.js";
import { s as w } from "../chunks/BzP2S3Z_.js";
import { i as W } from "../chunks/iO9_dPNE.js";
import { e as re } from "../chunks/S81raU5Y.js";
import { l as X, r as Se } from "../chunks/B21bTIl7.js";
import { B as N } from "../chunks/C4AV2CoD.js";
import { P as oe } from "../chunks/BXV49XFJ.js";
import { s as Z, b as $e, a as ye } from "../chunks/BdbQ6g_y.js";
import { C as Ce } from "../chunks/DwsgkunL.js";
import { E as Ie } from "../chunks/CvlP2s8z.js";
import { T as J } from "../chunks/DogAGR7A.js";
import { u as De } from "../chunks/N6FgGI8m.js";
import { u as ne } from "../chunks/D12OFlGX.js";
import { L as G } from "../chunks/D8MSWRQv.js";
import { p as Y } from "../chunks/C6GSeq7M.js";
import { c as de, f as be } from "../chunks/UPFlzoow.js";
import { C as Te } from "../chunks/C6jTHtu1.js";
import { O as ke } from "../chunks/Ssae7OHr.js";
import { u as Le } from "../chunks/CxfVL694.js";
import { P as Oe, f as Fe, a as Ae } from "../chunks/BoJgjqfh.js";
var Ee = ge(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25
            2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0
            0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5
            0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5
            0a48.667 48.667 0 0 0-7.5 0"></path></svg>`);
function ze(j, l) {
  let P = Y(l, "opacity", 3, 0.9), g = Y(l, "width", 3, "1.5rem"), n = Y(l, "color", 3, "hsl(var(--error))");
  var v = Ee();
  Z(v, "stroke-width", 2), f(() => {
    Z(v, "stroke", n()), Z(v, "width", g()), Z(v, "opacity", P());
  }), u(j, v);
}
var Ue = L('<div class="uid font-mono svelte-79owtg"> </div>'), je = L('<div class="date svelte-79owtg"> </div>'), Be = L('<div class="ip font-mono svelte-79owtg"> </div>'), He = L('<div class="trash svelte-79owtg"><!></div>'), qe = L('<div class="header svelte-79owtg"><!> <div class="headerRight svelte-79owtg"><!> <!> <!></div></div>'), Ge = L("<!> <!> <!> <!> <!> <!> <!> <!>", 1), Me = L("<div><!></div>");
function ie(j, l) {
  ae(l, true);
  let P = De(), g = ne(), n = x(R(P.common.copyToClip));
  function v() {
    l.session.remote_ip && (navigator.clipboard.writeText(l.session.remote_ip), a(n, g.common.copiedToClip, true), setTimeout(() => {
      a(n, P.common.copyToClip, true);
    }, 3e3));
  }
  async function O() {
    let C = await de(`/auth/v1/sessions/id/${l.session.id}`);
    C.error ? console.error(C.error) : l.onDeleted(l.session.id);
  }
  var F = Me();
  let B;
  var A = $(F);
  Ie(A, { summary: (S) => {
    var c = qe(), b = $(c);
    J(b, { text: "User ID", children: (h, E) => {
      var o = Ue();
      let r;
      var t = $(o, true);
      y(o), f(() => {
        r = ye(o, "", r, { "margin-left": l.session.user_id ? "" : ".5rem" }), w(t, l.session.user_id);
      }), u(h, o);
    }, $$slots: { default: true } });
    var p = _(b, 2), H = $(p);
    J(H, { get text() {
      return g.options.lastSeen;
    }, children: (h, E) => {
      var o = je(), r = $(o, true);
      y(o), f((t) => w(r, t), [() => X(l.session.last_seen)]), u(h, o);
    }, $$slots: { default: true } });
    var T = _(H, 2);
    N(T, { invisible: true, onclick: v, children: (h, E) => {
      const o = xe(() => `IP - ${s(n)}`);
      J(h, { get text() {
        return s(o);
      }, children: (r, t) => {
        var e = Be(), i = $(e, true);
        y(e), f(() => w(i, l.session.remote_ip)), u(r, e);
      }, $$slots: { default: true } });
    }, $$slots: { default: true } });
    var M = _(T, 2);
    N(M, { invisible: true, onclick: O, children: (h, E) => {
      J(h, { get text() {
        return P.common.delete;
      }, children: (o, r) => {
        var t = He(), e = $(t);
        ze(e, { width: "1.2rem" }), y(t), u(o, t);
      }, $$slots: { default: true } });
    }, $$slots: { default: true } }), y(p), y(c), u(S, c);
  }, details: (S) => {
    var c = Ge(), b = K(c);
    G(b, { label: "Session ID", children: (r, t) => {
      U();
      var e = z();
      f(() => w(e, l.session.id)), u(r, e);
    }, $$slots: { default: true } });
    var p = _(b, 2);
    G(p, { label: "User ID", children: (r, t) => {
      U();
      var e = z();
      f(() => w(e, l.session.user_id)), u(r, e);
    }, $$slots: { default: true } });
    var H = _(p, 2);
    G(H, { get label() {
      return g.options.expires;
    }, children: (r, t) => {
      U();
      var e = z();
      f((i) => w(e, i), [() => X(l.session.exp)]), u(r, e);
    }, $$slots: { default: true } });
    var T = _(H, 2);
    G(T, { get label() {
      return g.options.lastSeen;
    }, children: (r, t) => {
      U();
      var e = z();
      f((i) => w(e, i), [() => X(l.session.last_seen)]), u(r, e);
    }, $$slots: { default: true } });
    var M = _(T, 2);
    G(M, { get label() {
      return g.options.state;
    }, children: (r, t) => {
      U();
      var e = z();
      f(() => w(e, l.session.state)), u(r, e);
    }, $$slots: { default: true } });
    var h = _(M, 2);
    G(h, { label: "IP", children: (r, t) => {
      U();
      var e = z();
      f(() => w(e, l.session.remote_ip)), u(r, e);
    }, $$slots: { default: true } });
    var E = _(h, 2);
    G(E, { label: "MFA", children: (r, t) => {
      Ce(r, { get check() {
        return l.session.is_mfa;
      } });
    }, $$slots: { default: true } });
    var o = _(E, 2);
    N(o, { level: -1, onclick: O, children: (r, t) => {
      U();
      var e = z();
      f(() => w(e, P.common.delete)), u(r, e);
    }, $$slots: { default: true } }), u(S, c);
  }, $$slots: { summary: true, details: true } }), y(F), f((C) => B = $e(F, 1, "container svelte-79owtg", null, B, C), [() => ({ expired: l.session.exp < l.now })]), u(j, F), le();
}
var Re = L('<div class="err"> </div>'), Ve = L('<div class="top svelte-1sljqh8"><!> <div class="btn svelte-1sljqh8"><!></div></div> <!> <div id="sessions"><!></div> <!>', 1);
function We(j, l) {
  ae(l, true);
  let P = ne(), g = x(""), n = x(R([])), v = x(R([])), O = x(R([])), F = x(Date.now() / 1e3), B = x(false), A = x(void 0), C = x(R(Oe)), V = x(false), S = ["User ID", "Session ID", "IP"], c = x(R(S[0])), b = x(""), p = [P.options.expires, P.options.lastSeen, "Session ID", "User ID", P.options.state, "IP"];
  we(() => {
    T("page_size=" + s(C));
  }), Pe(() => {
    let o = s(b).toLowerCase();
    s(B) ? o.length < 3 ? s(V) && (T("page_size=" + s(C)), a(V, false)) : H(o) : o ? s(c) === S[0] ? a(v, s(n).filter((r) => {
      var _a;
      return (_a = r.user_id) == null ? void 0 : _a.toLowerCase().includes(o);
    }), true) : s(c) === S[1] ? a(v, s(n).filter((r) => r.id.toLowerCase().includes(o)), true) : s(c) === S[2] && a(v, s(n).filter((r) => {
      var _a;
      return (_a = r.remote_ip) == null ? void 0 : _a.toLowerCase().includes(o);
    }), true) : a(v, s(n), true);
  });
  async function H(o) {
    a(A, void 0), a(V, true);
    let r;
    s(c) === S[0] ? r = "userid" : s(c) === S[1] ? r = "sessionid" : r = "ip";
    let t = await Fe({ ty: "session", idx: r, q: o });
    t.body ? a(n, t.body, true) : console.error(t.error);
  }
  async function T(o) {
    let r = "/auth/v1/sessions";
    o && (r += `?${o}`);
    let t = await be(r);
    return t.error ? a(g, "Error fetching sessions: " + t.error.message) : t.body && (t.status === 206 ? (a(B, true), a(A, t.headers, true)) : (a(B, false), a(A, void 0)), a(n, t.body, true), a(F, Date.now() / 1e3)), [t.status, t.headers];
  }
  function M(o, r) {
    let t = r === "up";
    o === p[0] ? s(n).sort((e, i) => t ? e.exp - i.exp : i.exp - e.exp) : o === p[1] ? s(n).sort((e, i) => t ? e.last_seen - i.last_seen : i.last_seen - e.last_seen) : o === p[2] ? s(n).sort((e, i) => t ? e.id.localeCompare(i.id) : i.id.localeCompare(e.id)) : o === p[3] ? s(n).sort((e, i) => e.user_id && i.user_id ? t ? e.user_id.localeCompare(i.user_id) : i.user_id.localeCompare(e.user_id) : e.user_id ? t ? -1 : 1 : t ? 1 : -1) : o === p[4] ? s(n).sort((e, i) => t ? e.state.localeCompare(i.state) : i.state.localeCompare(e.state)) : o === p[5] && s(n).sort((e, i) => e.remote_ip && i.remote_ip ? t ? e.remote_ip.localeCompare(i.remote_ip) : i.remote_ip.localeCompare(e.remote_ip) : e.remote_ip ? t ? -1 : 1 : t ? 1 : -1);
  }
  function h(o) {
    var _a;
    a(n, s(n).filter((r) => r.id !== o), true), ((_a = Le("admin").get()) == null ? void 0 : _a.id) === o && window.location.reload();
  }
  async function E() {
    let o = await de("/auth/v1/sessions");
    o.error ? a(g, o.error.message, true) : Se();
  }
  Te(j, { children: (o, r) => {
    var t = Ve(), e = K(t), i = $(e);
    ke(i, { searchOptions: S, orderOptions: p, onChangeOrder: M, searchWidth: "min(25rem, calc(100dvw - 1rem))", get value() {
      return s(b);
    }, set value(d) {
      a(b, d, true);
    }, get searchOption() {
      return s(c);
    }, set searchOption(d) {
      a(c, d, true);
    } });
    var ee = _(i, 2), ue = $(ee);
    N(ue, { level: -1, onclick: E, children: (d, m) => {
      U();
      var I = z();
      f(() => w(I, P.sessions.invalidateAll)), u(d, I);
    }, $$slots: { default: true } }), y(ee), y(e);
    var te = _(e, 2);
    {
      var ve = (d) => {
        var m = Re(), I = $(m, true);
        y(m), f(() => w(I, s(g))), u(d, m);
      };
      W(te, (d) => {
        s(g) && d(ve);
      });
    }
    var Q = _(te, 2), ce = $(Q);
    {
      var me = (d) => {
        var m = se(), I = K(m);
        re(I, 17, () => s(n), (k) => k.id, (k, D) => {
          ie(k, { get session() {
            return s(D);
          }, get now() {
            return s(F);
          }, onDeleted: h });
        }), u(d, m);
      }, fe = (d) => {
        var m = se(), I = K(m);
        re(I, 17, () => s(O), (k) => k.id, (k, D) => {
          ie(k, { get session() {
            return s(D);
          }, get now() {
            return s(F);
          }, onDeleted: h });
        }), u(d, m);
      };
      W(ce, (d) => {
        s(A) ? d(me) : d(fe, false);
      });
    }
    y(Q);
    var _e = _(Q, 2);
    {
      var pe = (d) => {
        Ae(d, { sspFetch: T, get itemsLength() {
          return s(n).length;
        }, get firstFetchHeaders() {
          return s(A);
        }, get pageSize() {
          return s(C);
        }, set pageSize(m) {
          a(C, m, true);
        } });
      }, he = (d, m) => {
        {
          var I = (D) => {
            oe(D, { get items() {
              return s(n);
            }, set items(q) {
              a(n, q, true);
            }, get itemsPaginated() {
              return s(O);
            }, set itemsPaginated(q) {
              a(O, q, true);
            } });
          }, k = (D) => {
            oe(D, { get items() {
              return s(v);
            }, set items(q) {
              a(v, q, true);
            }, get itemsPaginated() {
              return s(O);
            }, set itemsPaginated(q) {
              a(O, q, true);
            } });
          };
          W(d, (D) => {
            s(B) ? D(I) : D(k, false);
          }, m);
        }
      };
      W(_e, (d) => {
        s(A) ? d(pe) : d(he, false);
      });
    }
    u(o, t);
  } }), le();
}
function ft(j) {
  We(j, {});
}
export {
  ft as component
};
