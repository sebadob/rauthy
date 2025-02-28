import { n as xe, a as m, t as L, e as z, d as Y } from "../chunks/BH6NCLk-.js";
import { t as _, p as le, k as w, c as C, r as I, a as ne, s as p, j as t, a9 as we, f as W, a7 as U, l as a, a5 as Pe, aa as Se } from "../chunks/CvlvO1XB.js";
import { s as P } from "../chunks/CTI4QPiR.js";
import { i as J } from "../chunks/BUO_AUgz.js";
import { e as oe } from "../chunks/BpWRzPRQ.js";
import { p as d } from "../chunks/Wh68IIk2.js";
import { l as ee, r as ye } from "../chunks/BaE7H8ny.js";
import { a as Ce, B as Q } from "../chunks/DMkkW5Nn.js";
import { P as ie } from "../chunks/yrKs3w9k.js";
import { s as K, a as Ie } from "../chunks/BMbqVy6X.js";
import { C as De } from "../chunks/CS_Msctd.js";
import { E as Te } from "../chunks/CgRaCfAG.js";
import { T as N } from "../chunks/BmpEzKyJ.js";
import { u as $e } from "../chunks/BQ1-pLIs.js";
import { u as de } from "../chunks/D8mHI_K9.js";
import { L as G } from "../chunks/CE2_6siz.js";
import { p as te } from "../chunks/C6SR4G2t.js";
import { c as ve, f as be } from "../chunks/CBGoQiUs.js";
import { C as Le } from "../chunks/BnPoFdx3.js";
import { O as ke } from "../chunks/CT2gdhvj.js";
import { u as Oe } from "../chunks/BhccdcsY.js";
import { P as Fe, f as Ae, a as Ee } from "../chunks/BWj5qgFT.js";
var ze = xe(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25
            2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0
            0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5
            0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5
            0a48.667 48.667 0 0 0-7.5 0"></path></svg>`);
function Ue(j, l) {
  let S = te(l, "opacity", 3, 0.9), x = te(l, "width", 3, "1.5rem"), n = te(l, "color", 3, "hsl(var(--error))");
  var u = ze();
  K(u, "stroke-width", 2), _(() => {
    K(u, "stroke", n()), K(u, "width", x()), K(u, "opacity", S());
  }), m(j, u);
}
var je = L('<div class="uid font-mono svelte-79owtg"> </div>'), Be = L('<div class="date svelte-79owtg"> </div>'), He = L('<div class="ip font-mono svelte-79owtg"> </div>'), qe = L('<div class="trash svelte-79owtg"><!></div>'), Ge = L('<div class="header svelte-79owtg"><!> <div class="headerRight svelte-79owtg"><!> <!> <!></div></div>'), Me = L("<!> <!> <!> <!> <!> <!> <!> <!>", 1), Re = L("<div><!></div>");
function ae(j, l) {
  le(l, true);
  let S = $e(), x = de(), n = w(d(S.common.copyToClip));
  function u() {
    l.session.remote_ip && (navigator.clipboard.writeText(l.session.remote_ip), a(n, d(x.common.copiedToClip)), setTimeout(() => {
      a(n, d(S.common.copyToClip));
    }, 3e3));
  }
  async function k() {
    let A = await ve(`/auth/v1/sessions/id/${l.session.id}`);
    A.error ? console.error(A.error) : l.onDeleted(l.session.id);
  }
  var O = Re();
  let B;
  var F = C(O);
  Te(F, { summary: (y) => {
    var f = Ge(), T = C(f);
    N(T, { text: "User ID", children: (g, E) => {
      var s = je(), r = C(s, true);
      I(s), _(() => {
        Ce(s, "margin-left", l.session.user_id ? "" : ".5rem"), P(r, l.session.user_id);
      }), m(g, s);
    }, $$slots: { default: true } });
    var h = p(T, 2), H = C(h);
    N(H, { get text() {
      return x.options.lastSeen;
    }, children: (g, E) => {
      var s = Be(), r = C(s, true);
      I(s), _((o) => P(r, o), [() => ee(l.session.last_seen)]), m(g, s);
    }, $$slots: { default: true } });
    var $ = p(H, 2);
    Q($, { invisible: true, onclick: u, children: (g, E) => {
      const s = we(() => `IP - ${t(n)}`);
      N(g, { get text() {
        return t(s);
      }, children: (r, o) => {
        var e = He(), i = C(e, true);
        I(e), _(() => P(i, l.session.remote_ip)), m(r, e);
      }, $$slots: { default: true } });
    }, $$slots: { default: true } });
    var M = p($, 2);
    Q(M, { invisible: true, onclick: k, children: (g, E) => {
      N(g, { get text() {
        return S.common.delete;
      }, children: (s, r) => {
        var o = qe(), e = C(o);
        Ue(e, { width: "1.2rem" }), I(o), m(s, o);
      }, $$slots: { default: true } });
    }, $$slots: { default: true } }), I(h), I(f), m(y, f);
  }, details: (y) => {
    var f = Me(), T = W(f);
    G(T, { label: "Session ID", children: (r, o) => {
      U();
      var e = z();
      _(() => P(e, l.session.id)), m(r, e);
    } });
    var h = p(T, 2);
    G(h, { label: "User ID", children: (r, o) => {
      U();
      var e = z();
      _(() => P(e, l.session.user_id)), m(r, e);
    } });
    var H = p(h, 2);
    G(H, { get label() {
      return x.options.expires;
    }, children: (r, o) => {
      U();
      var e = z();
      _((i) => P(e, i), [() => ee(l.session.exp)]), m(r, e);
    } });
    var $ = p(H, 2);
    G($, { get label() {
      return x.options.lastSeen;
    }, children: (r, o) => {
      U();
      var e = z();
      _((i) => P(e, i), [() => ee(l.session.last_seen)]), m(r, e);
    } });
    var M = p($, 2);
    G(M, { get label() {
      return x.options.state;
    }, children: (r, o) => {
      U();
      var e = z();
      _(() => P(e, l.session.state)), m(r, e);
    } });
    var g = p(M, 2);
    G(g, { label: "IP", children: (r, o) => {
      U();
      var e = z();
      _(() => P(e, l.session.remote_ip)), m(r, e);
    } });
    var E = p(g, 2);
    G(E, { label: "MFA", children: (r, o) => {
      De(r, {});
    } });
    var s = p(E, 2);
    Q(s, { level: -1, onclick: k, children: (r, o) => {
      U();
      var e = z();
      _(() => P(e, S.common.delete)), m(r, e);
    }, $$slots: { default: true } }), m(y, f);
  }, $$slots: { summary: true, details: true } }), I(O), _(() => B = Ie(O, 1, "container svelte-79owtg", null, B, { expired: l.session.exp < l.now })), m(j, O), ne();
}
var Ve = L('<div class="err"> </div>'), We = L('<div class="top svelte-1sljqh8"><!> <div class="btn svelte-1sljqh8"><!></div></div> <!> <div id="sessions"><!></div> <!>', 1);
function Ze(j, l) {
  le(l, true);
  let S = de(), x = w(""), n = w(d([])), u = w(d([])), k = w(d([])), O = w(Date.now() / 1e3), B = w(false), F = w(void 0), A = w(d(Fe)), Z = w(false), y = ["User ID", "Session ID", "IP"], f = w(d(y[0])), T = w(""), h = [S.options.expires, S.options.lastSeen, "Session ID", "User ID", S.options.state, "IP"];
  Pe(() => {
    $("page_size=" + t(A));
  }), Se(() => {
    let s = t(T).toLowerCase();
    t(B) ? s.length < 3 ? t(Z) && ($("page_size=" + t(A)), a(Z, false)) : H(s) : s ? t(f) === y[0] ? a(u, d(t(n).filter((r) => {
      var _a;
      return (_a = r.user_id) == null ? void 0 : _a.toLowerCase().includes(s);
    }))) : t(f) === y[1] ? a(u, d(t(n).filter((r) => r.id.toLowerCase().includes(s)))) : t(f) === y[2] && a(u, d(t(n).filter((r) => {
      var _a;
      return (_a = r.remote_ip) == null ? void 0 : _a.toLowerCase().includes(s);
    }))) : a(u, d(t(n)));
  });
  async function H(s) {
    a(F, void 0), a(Z, true);
    let r;
    t(f) === y[0] ? r = "userid" : t(f) === y[1] ? r = "sessionid" : r = "ip";
    let o = await Ae({ ty: "session", idx: r, q: s });
    o.body ? a(n, d(o.body)) : console.error(o.error);
  }
  async function $(s) {
    let r = "/auth/v1/sessions";
    s && (r += `?${s}`);
    let o = await be(r);
    return o.error ? a(x, "Error fetching sessions: " + o.error.message) : o.body && (o.status === 206 ? (a(B, true), a(F, d(o.headers))) : (a(B, false), a(F, void 0)), a(n, d(o.body)), a(O, Date.now() / 1e3)), [o.status, o.headers];
  }
  function M(s, r) {
    let o = r === "up";
    s === h[0] ? t(n).sort((e, i) => o ? e.exp - i.exp : i.exp - e.exp) : s === h[1] ? t(n).sort((e, i) => o ? e.last_seen - i.last_seen : i.last_seen - e.last_seen) : s === h[2] ? t(n).sort((e, i) => o ? e.id.localeCompare(i.id) : i.id.localeCompare(e.id)) : s === h[3] ? t(n).sort((e, i) => e.user_id && i.user_id ? o ? e.user_id.localeCompare(i.user_id) : i.user_id.localeCompare(e.user_id) : e.user_id ? o ? -1 : 1 : o ? 1 : -1) : s === h[4] ? t(n).sort((e, i) => o ? e.state.localeCompare(i.state) : i.state.localeCompare(e.state)) : s === h[5] && t(n).sort((e, i) => e.remote_ip && i.remote_ip ? o ? e.remote_ip.localeCompare(i.remote_ip) : i.remote_ip.localeCompare(e.remote_ip) : e.remote_ip ? o ? -1 : 1 : o ? 1 : -1);
  }
  function g(s) {
    var _a;
    a(n, d(t(n).filter((r) => r.id !== s))), ((_a = Oe("admin").get()) == null ? void 0 : _a.id) === s && window.location.reload();
  }
  async function E() {
    let s = await ve("/auth/v1/sessions");
    s.error ? a(x, d(s.error.message)) : ye();
  }
  Le(j, { children: (s, r) => {
    var o = We(), e = W(o), i = C(e);
    ke(i, { searchOptions: y, orderOptions: h, onChangeOrder: M, searchWidth: "min(25rem, calc(100dvw - 1rem))", get value() {
      return t(T);
    }, set value(v) {
      a(T, d(v));
    }, get searchOption() {
      return t(f);
    }, set searchOption(v) {
      a(f, d(v));
    } });
    var se = p(i, 2), me = C(se);
    Q(me, { level: -1, onclick: E, children: (v, c) => {
      U();
      var D = z();
      _(() => P(D, S.sessions.invalidateAll)), m(v, D);
    }, $$slots: { default: true } }), I(se), I(e);
    var re = p(e, 2);
    {
      var ce = (v) => {
        var c = Ve(), D = C(c, true);
        I(c), _(() => P(D, t(x))), m(v, c);
      };
      J(re, (v) => {
        t(x) && v(ce);
      });
    }
    var X = p(re, 2), ue = C(X);
    {
      var fe = (v) => {
        var c = Y(), D = W(c);
        oe(D, 17, () => t(n), (b) => b.id, (b, V) => {
          ae(b, { get session() {
            return t(V);
          }, get now() {
            return t(O);
          }, onDeleted: g });
        }), m(v, c);
      }, _e = (v) => {
        var c = Y(), D = W(c);
        oe(D, 17, () => t(k), (b) => b.id, (b, V) => {
          ae(b, { get session() {
            return t(V);
          }, get now() {
            return t(O);
          }, onDeleted: g });
        }), m(v, c);
      };
      J(ue, (v) => {
        t(F) ? v(fe) : v(_e, false);
      });
    }
    I(X);
    var pe = p(X, 2);
    {
      var he = (v) => {
        Ee(v, { sspFetch: $, get itemsLength() {
          return t(n).length;
        }, get firstFetchHeaders() {
          return t(F);
        }, get pageSize() {
          return t(A);
        }, set pageSize(c) {
          a(A, d(c));
        } });
      }, ge = (v) => {
        var c = Y(), D = W(c);
        {
          var b = (R) => {
            ie(R, { get items() {
              return t(n);
            }, set items(q) {
              a(n, d(q));
            }, get itemsPaginated() {
              return t(k);
            }, set itemsPaginated(q) {
              a(k, d(q));
            } });
          }, V = (R) => {
            ie(R, { get items() {
              return t(u);
            }, set items(q) {
              a(u, d(q));
            }, get itemsPaginated() {
              return t(k);
            }, set itemsPaginated(q) {
              a(k, d(q));
            } });
          };
          J(D, (R) => {
            t(B) ? R(b) : R(V, false);
          }, true);
        }
        m(v, c);
      };
      J(pe, (v) => {
        t(F) ? v(he) : v(ge, false);
      });
    }
    m(s, o);
  } }), ne();
}
function pt(j) {
  Ze(j, {});
}
export {
  pt as component
};
