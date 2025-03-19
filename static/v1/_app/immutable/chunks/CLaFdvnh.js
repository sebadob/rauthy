import { n as rt, a as p, t as A } from "./DLBGyKVC.js";
import { t as V, p as Fe, k as S, P as ie, j as i, a9 as L, aa as ee, l as _, c as h, s as B, r as f, a as Oe } from "./CmQi0fbH.js";
import { i as _e } from "./C6bK2EJJ.js";
import { s as d, r as at, a as Et } from "./DOlUUCkJ.js";
import { p as w } from "./B_ggA-0N.js";
import { p as y } from "./DNJm3-SG.js";
import { t as nt, B as it, s as ot } from "./DPLO-ozG.js";
import { d as lt, s as oe, e as fe } from "./BjaYyaa_.js";
import { e as Ye, i as qe } from "./YQCw2eEa.js";
import { b as st } from "./B3JAWFL5.js";
import { b as Ne } from "./BLvsVIZg.js";
import { $ as dt, c as ct, d as Tt, f as Ue, g as St, e as Qe, h as At, a as Re } from "./DswDW5U8.js";
import { u as je } from "./DGTOa5g8.js";
import { P as ut, O as ke } from "./DqhsozSK.js";
import { I as Ct } from "./Dth7WIu-.js";
function Be(t, e) {
  return t - e * Math.floor(t / e);
}
const ft = 1721426;
function we(t, e, r, a) {
  e = We(t, e);
  let n = e - 1, o = -2;
  return r <= 2 ? o = 0 : xe(e) && (o = -1), ft - 1 + 365 * n + Math.floor(n / 4) - Math.floor(n / 100) + Math.floor(n / 400) + Math.floor((367 * r - 362) / 12 + o + a);
}
function xe(t) {
  return t % 4 === 0 && (t % 100 !== 0 || t % 400 === 0);
}
function We(t, e) {
  return t === "BC" ? 1 - e : e;
}
function Lt(t) {
  let e = "AD";
  return t <= 0 && (e = "BC", t = 1 - t), [e, t];
}
const Yt = { standard: [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31], leapyear: [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31] };
class $e {
  fromJulianDay(e) {
    let r = e, a = r - ft, n = Math.floor(a / 146097), o = Be(a, 146097), s = Math.floor(o / 36524), c = Be(o, 36524), m = Math.floor(c / 1461), $ = Be(c, 1461), E = Math.floor($ / 365), P = n * 400 + s * 100 + m * 4 + E + (s !== 4 && E !== 4 ? 1 : 0), [I, D] = Lt(P), Y = r - we(I, D, 1, 1), C = 2;
    r < we(I, D, 3, 1) ? C = 0 : xe(D) && (C = 1);
    let R = Math.floor(((Y + C) * 12 + 373) / 367), T = r - we(I, D, R, 1) + 1;
    return new se(I, D, R, T);
  }
  toJulianDay(e) {
    return we(e.era, e.year, e.month, e.day);
  }
  getDaysInMonth(e) {
    return Yt[xe(e.year) ? "leapyear" : "standard"][e.month - 1];
  }
  getMonthsInYear(e) {
    return 12;
  }
  getDaysInYear(e) {
    return xe(e.year) ? 366 : 365;
  }
  getYearsInEra(e) {
    return 9999;
  }
  getEras() {
    return ["BC", "AD"];
  }
  isInverseEra(e) {
    return e.era === "BC";
  }
  balanceDate(e) {
    e.year <= 0 && (e.era = e.era === "BC" ? "AD" : "BC", e.year = 1 - e.year);
  }
  constructor() {
    this.identifier = "gregory";
  }
}
function Xe(t) {
  t = Je(t, new $e());
  let e = We(t.era, t.year);
  return ht(e, t.month, t.day, t.hour, t.minute, t.second, t.millisecond);
}
function ht(t, e, r, a, n, o, s) {
  let c = /* @__PURE__ */ new Date();
  return c.setUTCHours(a, n, o, s), c.setUTCFullYear(t, e - 1, r), c.getTime();
}
function Ze(t, e) {
  if (e === "UTC") return 0;
  if (t > 0 && e === dt()) return new Date(t).getTimezoneOffset() * -6e4;
  let { year: r, month: a, day: n, hour: o, minute: s, second: c } = vt(t, e);
  return ht(r, a, n, o, s, c, 0) - Math.floor(t / 1e3) * 1e3;
}
const et = /* @__PURE__ */ new Map();
function vt(t, e) {
  let r = et.get(e);
  r || (r = new Intl.DateTimeFormat("en-US", { timeZone: e, hour12: false, era: "short", year: "numeric", month: "numeric", day: "numeric", hour: "numeric", minute: "numeric", second: "numeric" }), et.set(e, r));
  let a = r.formatToParts(new Date(t)), n = {};
  for (let o of a) o.type !== "literal" && (n[o.type] = o.value);
  return { year: n.era === "BC" || n.era === "B" ? -n.year + 1 : +n.year, month: +n.month, day: +n.day, hour: n.hour === "24" ? 0 : +n.hour, minute: +n.minute, second: +n.second };
}
const tt = 864e5;
function qt(t, e, r, a) {
  return (r === a ? [r] : [r, a]).filter((o) => Bt(t, e, o));
}
function Bt(t, e, r) {
  let a = vt(r, e);
  return t.year === a.year && t.month === a.month && t.day === a.day && t.hour === a.hour && t.minute === a.minute && t.second === a.second;
}
function Pt(t, e, r = "compatible") {
  let a = bt(t);
  if (e === "UTC") return Xe(a);
  if (e === dt() && r === "compatible") {
    a = Je(a, new $e());
    let m = /* @__PURE__ */ new Date(), $ = We(a.era, a.year);
    return m.setFullYear($, a.month - 1, a.day), m.setHours(a.hour, a.minute, a.second, a.millisecond), m.getTime();
  }
  let n = Xe(a), o = Ze(n - tt, e), s = Ze(n + tt, e), c = qt(a, e, n - o, n - s);
  if (c.length === 1) return c[0];
  if (c.length > 1) switch (r) {
    case "compatible":
    case "earlier":
      return c[0];
    case "later":
      return c[c.length - 1];
    case "reject":
      throw new RangeError("Multiple possible absolute times found");
  }
  switch (r) {
    case "earlier":
      return Math.min(n - o, n - s);
    case "compatible":
    case "later":
      return Math.max(n - o, n - s);
    case "reject":
      throw new RangeError("No such absolute time found");
  }
}
function mt(t, e, r = "compatible") {
  return new Date(Pt(t, e, r));
}
function bt(t, e) {
  let r = 0, a = 0, n = 0, o = 0;
  if ("timeZone" in t) ({ hour: r, minute: a, second: n, millisecond: o } = t);
  else if ("hour" in t && !e) return t;
  return e && ({ hour: r, minute: a, second: n, millisecond: o } = e), new De(t.calendar, t.era, t.year, t.month, t.day, r, a, n, o);
}
function Je(t, e) {
  if (t.calendar.identifier === e.identifier) return t;
  let r = e.fromJulianDay(t.calendar.toJulianDay(t)), a = t.copy();
  return a.calendar = e, a.era = r.era, a.year = r.year, a.month = r.month, a.day = r.day, le(a), a;
}
function ze(t, e) {
  let r = t.copy(), a = "hour" in r ? Ft(r, e) : 0;
  Ve(r, e.years || 0), r.calendar.balanceYearMonth && r.calendar.balanceYearMonth(r, t), r.month += e.months || 0, He(r), yt(r), r.day += (e.weeks || 0) * 7, r.day += e.days || 0, r.day += a, Nt(r), r.calendar.balanceDate && r.calendar.balanceDate(r), r.year < 1 && (r.year = 1, r.month = 1, r.day = 1);
  let n = r.calendar.getYearsInEra(r);
  if (r.year > n) {
    var o, s;
    let m = (o = (s = r.calendar).isInverseEra) === null || o === void 0 ? void 0 : o.call(s, r);
    r.year = n, r.month = m ? 1 : r.calendar.getMonthsInYear(r), r.day = m ? 1 : r.calendar.getDaysInMonth(r);
  }
  r.month < 1 && (r.month = 1, r.day = 1);
  let c = r.calendar.getMonthsInYear(r);
  return r.month > c && (r.month = c, r.day = r.calendar.getDaysInMonth(r)), r.day = Math.max(1, Math.min(r.calendar.getDaysInMonth(r), r.day)), r;
}
function Ve(t, e) {
  var r, a;
  !((r = (a = t.calendar).isInverseEra) === null || r === void 0) && r.call(a, t) && (e = -e), t.year += e;
}
function He(t) {
  for (; t.month < 1; ) Ve(t, -1), t.month += t.calendar.getMonthsInYear(t);
  let e = 0;
  for (; t.month > (e = t.calendar.getMonthsInYear(t)); ) t.month -= e, Ve(t, 1);
}
function Nt(t) {
  for (; t.day < 1; ) t.month--, He(t), t.day += t.calendar.getDaysInMonth(t);
  for (; t.day > t.calendar.getDaysInMonth(t); ) t.day -= t.calendar.getDaysInMonth(t), t.month++, He(t);
}
function yt(t) {
  t.month = Math.max(1, Math.min(t.calendar.getMonthsInYear(t), t.month)), t.day = Math.max(1, Math.min(t.calendar.getDaysInMonth(t), t.day));
}
function le(t) {
  t.calendar.constrainDate && t.calendar.constrainDate(t), t.year = Math.max(1, Math.min(t.calendar.getYearsInEra(t), t.year)), yt(t);
}
function Ut(t) {
  let e = {};
  for (let r in t) typeof t[r] == "number" && (e[r] = -t[r]);
  return e;
}
function gt(t, e) {
  return ze(t, Ut(e));
}
function wt(t, e) {
  let r = t.copy();
  return e.era != null && (r.era = e.era), e.year != null && (r.year = e.year), e.month != null && (r.month = e.month), e.day != null && (r.day = e.day), le(r), r;
}
function Rt(t, e) {
  let r = t.copy();
  return e.hour != null && (r.hour = e.hour), e.minute != null && (r.minute = e.minute), e.second != null && (r.second = e.second), e.millisecond != null && (r.millisecond = e.millisecond), Ht(r), r;
}
function Vt(t) {
  t.second += Math.floor(t.millisecond / 1e3), t.millisecond = Me(t.millisecond, 1e3), t.minute += Math.floor(t.second / 60), t.second = Me(t.second, 60), t.hour += Math.floor(t.minute / 60), t.minute = Me(t.minute, 60);
  let e = Math.floor(t.hour / 24);
  return t.hour = Me(t.hour, 24), e;
}
function Ht(t) {
  t.millisecond = Math.max(0, Math.min(t.millisecond, 1e3)), t.second = Math.max(0, Math.min(t.second, 59)), t.minute = Math.max(0, Math.min(t.minute, 59)), t.hour = Math.max(0, Math.min(t.hour, 23));
}
function Me(t, e) {
  let r = t % e;
  return r < 0 && (r += e), r;
}
function Ft(t, e) {
  return t.hour += e.hours || 0, t.minute += e.minutes || 0, t.second += e.seconds || 0, t.millisecond += e.milliseconds || 0, Vt(t);
}
function Mt(t, e, r, a) {
  let n = t.copy();
  switch (e) {
    case "era": {
      let c = t.calendar.getEras(), m = c.indexOf(t.era);
      if (m < 0) throw new Error("Invalid era: " + t.era);
      m = Q(m, r, 0, c.length - 1, a == null ? void 0 : a.round), n.era = c[m], le(n);
      break;
    }
    case "year":
      var o, s;
      !((o = (s = n.calendar).isInverseEra) === null || o === void 0) && o.call(s, n) && (r = -r), n.year = Q(t.year, r, -1 / 0, 9999, a == null ? void 0 : a.round), n.year === -1 / 0 && (n.year = 1), n.calendar.balanceYearMonth && n.calendar.balanceYearMonth(n, t);
      break;
    case "month":
      n.month = Q(t.month, r, 1, t.calendar.getMonthsInYear(t), a == null ? void 0 : a.round);
      break;
    case "day":
      n.day = Q(t.day, r, 1, t.calendar.getDaysInMonth(t), a == null ? void 0 : a.round);
      break;
    default:
      throw new Error("Unsupported field " + e);
  }
  return t.calendar.balanceDate && t.calendar.balanceDate(n), le(n), n;
}
function Ot(t, e, r, a) {
  let n = t.copy();
  switch (e) {
    case "hour": {
      let o = t.hour, s = 0, c = 23;
      if ((a == null ? void 0 : a.hourCycle) === 12) {
        let m = o >= 12;
        s = m ? 12 : 0, c = m ? 23 : 11;
      }
      n.hour = Q(o, r, s, c, a == null ? void 0 : a.round);
      break;
    }
    case "minute":
      n.minute = Q(t.minute, r, 0, 59, a == null ? void 0 : a.round);
      break;
    case "second":
      n.second = Q(t.second, r, 0, 59, a == null ? void 0 : a.round);
      break;
    case "millisecond":
      n.millisecond = Q(t.millisecond, r, 0, 999, a == null ? void 0 : a.round);
      break;
    default:
      throw new Error("Unsupported field " + e);
  }
  return n;
}
function Q(t, e, r, a, n = false) {
  if (n) {
    t += Math.sign(e), t < r && (t = a);
    let o = Math.abs(e);
    e > 0 ? t = Math.ceil(t / o) * o : t = Math.floor(t / o) * o, t > a && (t = r);
  } else t += e, t < r ? t = a - (r - t - 1) : t > a && (t = r + (t - a - 1));
  return t;
}
const jt = /^([+-]\d{6}|\d{4})-(\d{2})-(\d{2})$/;
function G(t) {
  let e = t.match(jt);
  if (!e) throw new Error("Invalid ISO 8601 date string: " + t);
  let r = new se(Pe(e[1], 0, 9999), Pe(e[2], 1, 12), 1);
  return r.day = Pe(e[3], 0, r.calendar.getDaysInMonth(r)), r;
}
function Pe(t, e, r) {
  let a = Number(t);
  if (a < e || a > r) throw new RangeError(`Value out of range: ${e} <= ${a} <= ${r}`);
  return a;
}
function Wt(t) {
  return `${String(t.hour).padStart(2, "0")}:${String(t.minute).padStart(2, "0")}:${String(t.second).padStart(2, "0")}${t.millisecond ? String(t.millisecond / 1e3).slice(1) : ""}`;
}
function xt(t) {
  let e = Je(t, new $e()), r;
  return e.era === "BC" ? r = e.year === 1 ? "0000" : "-" + String(Math.abs(1 - e.year)).padStart(6, "00") : r = String(e.year).padStart(4, "0"), `${r}-${String(e.month).padStart(2, "0")}-${String(e.day).padStart(2, "0")}`;
}
function Jt(t) {
  return `${xt(t)}T${Wt(t)}`;
}
function zt(t, e) {
  if (e.has(t)) throw new TypeError("Cannot initialize the same private elements twice on an object");
}
function _t(t, e, r) {
  zt(t, e), e.set(t, r);
}
function kt(t) {
  let e = typeof t[0] == "object" ? t.shift() : new $e(), r;
  if (typeof t[0] == "string") r = t.shift();
  else {
    let s = e.getEras();
    r = s[s.length - 1];
  }
  let a = t.shift(), n = t.shift(), o = t.shift();
  return [e, r, a, n, o];
}
var Kt = /* @__PURE__ */ new WeakMap();
class se {
  copy() {
    return this.era ? new se(this.calendar, this.era, this.year, this.month, this.day) : new se(this.calendar, this.year, this.month, this.day);
  }
  add(e) {
    return ze(this, e);
  }
  subtract(e) {
    return gt(this, e);
  }
  set(e) {
    return wt(this, e);
  }
  cycle(e, r, a) {
    return Mt(this, e, r, a);
  }
  toDate(e) {
    return mt(this, e);
  }
  toString() {
    return xt(this);
  }
  compare(e) {
    return ct(this, e);
  }
  constructor(...e) {
    _t(this, Kt, { writable: true, value: void 0 });
    let [r, a, n, o, s] = kt(e);
    this.calendar = r, this.era = a, this.year = n, this.month = o, this.day = s, le(this);
  }
}
var Gt = /* @__PURE__ */ new WeakMap();
class De {
  copy() {
    return this.era ? new De(this.calendar, this.era, this.year, this.month, this.day, this.hour, this.minute, this.second, this.millisecond) : new De(this.calendar, this.year, this.month, this.day, this.hour, this.minute, this.second, this.millisecond);
  }
  add(e) {
    return ze(this, e);
  }
  subtract(e) {
    return gt(this, e);
  }
  set(e) {
    return wt(Rt(this, e), e);
  }
  cycle(e, r, a) {
    switch (e) {
      case "era":
      case "year":
      case "month":
      case "day":
        return Mt(this, e, r, a);
      default:
        return Ot(this, e, r, a);
    }
  }
  toDate(e, r) {
    return mt(this, e, r);
  }
  toString() {
    return Jt(this);
  }
  compare(e) {
    let r = ct(this, e);
    return r === 0 ? Tt(this, bt(e)) : r;
  }
  constructor(...e) {
    _t(this, Gt, { writable: true, value: void 0 });
    let [r, a, n, o, s] = kt(e);
    this.calendar = r, this.era = a, this.year = n, this.month = o, this.day = s, this.hour = e.shift() || 0, this.minute = e.shift() || 0, this.second = e.shift() || 0, this.millisecond = e.shift() || 0, le(this);
  }
}
var Qt = rt(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 0 1 2.25-2.25h13.5A2.25 2.25 0 0 1 21
            7.5v11.25m-18 0A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75m-18 0v-7.5A2.25 2.25
            0 0 1 5.25 9h13.5A2.25 2.25 0 0 1 21 11.25v7.5"></path></svg>`);
function Xt(t, e) {
  let r = y(e, "opacity", 3, 0.9), a = y(e, "width", 3, "1.5rem"), n = y(e, "color", 3, "currentColor");
  var o = Qt();
  d(o, "stroke-width", 2), V(() => {
    d(o, "stroke", n()), d(o, "width", a()), d(o, "opacity", r());
  }), p(t, o);
}
function Zt(t, e) {
  var _a, _b, _c, _d, _e2;
  switch (t.code) {
    case "Enter":
      (_a = e.onEnter) == null ? void 0 : _a.call(e);
      break;
    case "ArrowUp":
      (_b = e.onUp) == null ? void 0 : _b.call(e);
      break;
    case "ArrowDown":
      (_c = e.onDown) == null ? void 0 : _c.call(e);
      break;
    case "ArrowLeft":
      (_d = e.onLeft) == null ? void 0 : _d.call(e);
      break;
    case "ArrowRight":
      (_e2 = e.onRight) == null ? void 0 : _e2.call(e);
      break;
  }
}
var er = A('<div class="pointer"><!></div>'), tr = A('<div class="day font-label weekLabel svelte-1x73n1"> </div>'), rr = A('<time class="svelte-1x73n1"> </time>'), ar = A('<div class="day svelte-1x73n1"><!></div>'), nr = A('<div class="week svelte-1x73n1"></div>'), ir = A('<div class="popup svelte-1x73n1"><div class="flex space-between"><div><!></div> <div><!></div></div> <div class="week svelte-1x73n1"></div> <div class="month svelte-1x73n1"></div></div>'), or = A('<div class="error svelte-1x73n1"> </div>'), lr = A('<div><div class="flex"><input type="date" step="any" class="svelte-1x73n1"> <div class="relative"><div class="absolute patch svelte-1x73n1"></div> <div class="absolute indicator svelte-1x73n1"><!></div></div></div> <div class="label svelte-1x73n1"><label class="font-label noselect svelte-1x73n1"> </label> <!></div></div>');
function sr(t, e) {
  Fe(e, true);
  let r = y(e, "value", 31, () => w(Ue())), a = y(e, "label", 3, ""), n = y(e, "errMsg", 3, ""), o = y(e, "min", 3, "1900-01-01"), s = y(e, "max", 3, "2100-01-01");
  const c = G(Ue()), m = c.toString();
  let $ = je(), E = S(false), P = S(void 0), I, D = S(void 0), Y = S(void 0), C = S(void 0), R = L(() => r() ? G(r()) : c), T = L(() => G(o())), H = L(() => G(s())), N = S(w(ie(() => $.common.months[i(R).month - 1]))), U = L(() => $.common.months.findIndex((u) => u === i(N)) + 1), F = S(w(ie(() => i(R).year))), j = S(w([ie(() => i(F))])), de = L(() => St(i(R), $.lang));
  ee(() => {
    let u = Number.parseInt(o().slice(0, 4)), v = Number.parseInt(s().slice(0, 4)), k = [];
    for (let x = u; x <= v; x++) k.push(x);
    _(j, w(k));
  }), ee(() => {
    if (r()) {
      let u = ie(() => G(r()));
      u = u.set({ year: i(F), month: i(U) }), r(u.toString());
    }
  }), ee(() => {
    i(C) && r() && requestAnimationFrame(() => {
      var _a, _b, _c;
      let u = ((_a = i(C)) == null ? void 0 : _a.getElementsByTagName("time")) || [];
      for (let v of u) if (v.getAttribute("datetime") === r()) {
        (_c = (_b = v.parentElement) == null ? void 0 : _b.parentElement) == null ? void 0 : _c.focus();
        break;
      }
    });
  });
  function Ie(u) {
    let v = G(Qe(u));
    return v.compare(i(T)) < 0 || v.compare(i(H)) > 0 ? false : i(U) === u.month;
  }
  function he() {
    r(new se(i(F), i(U), 1).toString());
  }
  function pe(u) {
    u.preventDefault(), _(E, true);
  }
  function ve(u, v) {
    var _a;
    u.preventDefault(), r(v), (_a = i(P)) == null ? void 0 : _a();
  }
  function W(u, v) {
    var _a, _b;
    let k = G(v), x = G(v);
    switch (u) {
      case "left":
        x = k.subtract({ days: 1 });
        break;
      case "right":
        x = k.add({ days: 1 });
        break;
      case "up":
        x = k.subtract({ weeks: 1 });
        break;
      case "down":
        x = k.add({ weeks: 1 });
        break;
    }
    x.month !== k.month ? At(x, $.lang) < 4 ? (_a = i(D)) == null ? void 0 : _a.focus() : (_b = i(Y)) == null ? void 0 : _b.focus() : r(x.toString());
  }
  function me(u) {
    switch (u) {
      case "open":
        r() || (r(m), requestAnimationFrame(() => te()));
        break;
      case "closed":
        _(C, void 0);
        break;
    }
  }
  function te() {
    _(E, !(I == null ? void 0 : I.reportValidity()));
  }
  var g = lr(), re = h(g), M = h(re);
  at(M), M.__keydown = [Zt, e], Ne(M, (u) => I = u, () => I);
  var be = B(M, 2), ce = B(h(be), 2), X = h(ce);
  ut(X, { ariaLabel: "Show Popover Example", offsetLeft: "-7.5rem", offsetTop: "-.2rem", onToggle: me, btnInvisible: true, lazy: true, get close() {
    return i(P);
  }, set close(v) {
    _(P, w(v));
  }, button: (v) => {
    var k = er(), x = h(k);
    Xt(x, { color: "hsl(var(--text)", width: "1.2rem" }), f(k), V(() => d(k, "title", a())), p(v, k);
  }, children: (v, k) => {
    var x = ir(), ae = h(x), ne = h(ae), Te = h(ne);
    ke(Te, { get ariaLabel() {
      return $.common.month;
    }, get options() {
      return $.common.months;
    }, maxHeight: "13rem", borderless: true, onRight: () => {
      var _a;
      return (_a = i(Y)) == null ? void 0 : _a.focus();
    }, onDown: he, get ref() {
      return i(D);
    }, set ref(q) {
      _(D, w(q));
    }, get value() {
      return i(N);
    }, set value(q) {
      _(N, w(q));
    } }), f(ne);
    var O = B(ne, 2), Dt = h(O);
    ke(Dt, { get ariaLabel() {
      return $.common.year;
    }, get options() {
      return i(j);
    }, maxHeight: "13rem", borderless: true, onLeft: () => {
      var _a;
      return (_a = i(D)) == null ? void 0 : _a.focus();
    }, onDown: he, get ref() {
      return i(Y);
    }, set ref(q) {
      _(Y, w(q));
    }, get value() {
      return i(F);
    }, set value(q) {
      _(F, w(q));
    } }), f(O), f(ae);
    var Se = B(ae, 2);
    Ye(Se, 21, () => $.common.weekDaysShort, qe, (q, Ce, ge, Ke) => {
      var J = tr(), ue = h(J, true);
      f(J), V(() => oe(ue, i(Ce))), p(q, J);
    }), f(Se);
    var Ae = B(Se, 2);
    Ye(Ae, 21, () => i(de), qe, (q, Ce) => {
      var ge = nr();
      Ye(ge, 21, () => i(Ce).days, qe, (Ke, J) => {
        var ue = ar();
        const z = L(() => Qe(i(J))), Ge = L(() => Ie(i(J)));
        var $t = h(ue);
        const It = L(() => !i(Ge));
        it($t, { onclick: (Le) => ve(Le, i(z)), get isDisabled() {
          return i(It);
        }, invisible: true, onLeft: () => W("left", i(z)), onRight: () => W("right", i(z)), onUp: () => W("up", i(z)), onDown: () => W("down", i(z)), children: (Le, Mr) => {
          var K = rr(), pt = h(K, true);
          f(K), V(() => {
            d(K, "data-today", i(z) === m), d(K, "data-active", i(Ge)), d(K, "data-selected", i(z) === r()), d(K, "data-weekend", i(J).isWeekend), d(K, "datetime", i(z)), oe(pt, i(J).day);
          }), p(Le, K);
        }, $$slots: { default: true } }), f(ue), p(Ke, ue);
      }), f(ge), p(q, ge);
    }), f(Ae), Ne(Ae, (q) => _(C, q), () => i(C)), f(x), p(v, x);
  }, $$slots: { button: true, default: true } }), f(ce), f(be), f(re);
  var ye = B(re, 2), Z = h(ye), Ee = h(Z, true);
  f(Z);
  var b = B(Z, 2);
  {
    var l = (u) => {
      var v = or(), k = h(v, true);
      f(v), V(() => oe(k, n())), nt(3, v, () => ot, () => ({ duration: 150 })), p(u, v);
    };
    _e(b, (u) => {
      i(E) && u(l);
    });
  }
  f(ye), f(g), V(() => {
    d(M, "id", e.id), d(M, "name", e.name), d(M, "title", a()), d(M, "aria-label", a()), M.disabled = e.disabled, d(M, "aria-disabled", e.disabled), M.required = e.required, d(M, "aria-required", e.required || false), d(M, "aria-invalid", i(E)), d(M, "min", o()), d(M, "max", s()), d(Z, "for", e.id), d(Z, "data-required", e.required), oe(Ee, a());
  }), fe("invalid", M, pe), fe("blur", M, () => te()), st(M, r), p(t, g), Oe();
}
lt(["keydown"]);
var dr = rt('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M12 6v6h4.5m4.5 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"></path></svg>');
function cr(t, e) {
  let r = y(e, "opacity", 3, 0.9), a = y(e, "width", 3, "1.5rem"), n = y(e, "color", 3, "currentColor");
  var o = dr();
  d(o, "stroke-width", 2), V(() => {
    d(o, "stroke", n()), d(o, "width", a()), d(o, "opacity", r());
  }), p(t, o);
}
function ur(t, e) {
  var _a, _b, _c, _d, _e2;
  switch (t.code) {
    case "Enter":
      (_a = e.onEnter) == null ? void 0 : _a.call(e);
      break;
    case "ArrowUp":
      (_b = e.onUp) == null ? void 0 : _b.call(e);
      break;
    case "ArrowDown":
      (_c = e.onDown) == null ? void 0 : _c.call(e);
      break;
    case "ArrowLeft":
      (_d = e.onLeft) == null ? void 0 : _d.call(e);
      break;
    case "ArrowRight":
      (_e2 = e.onRight) == null ? void 0 : _e2.call(e);
      break;
  }
}
var fr = A('<div class="pointer"><!></div>'), hr = A('<div class="popup svelte-79bts9"><div class="flex space-between"><div><!></div> <div class="colon svelte-79bts9">:</div> <div><!></div></div></div>'), vr = A('<div class="error svelte-79bts9"> </div>'), mr = A('<div><div class="flex"><input type="text" minlength="5" maxlength="5" class="svelte-79bts9"> <div class="relative"><div class="absolute indicator svelte-79bts9"><!></div></div></div> <div class="label svelte-79bts9"><label class="font-label noselect svelte-79bts9"> </label> <!></div></div>');
function br(t, e) {
  Fe(e, true);
  let r = y(e, "name", 3, "time"), a = y(e, "value", 31, () => w(Re())), n = y(e, "label", 3, ""), o = y(e, "ariaLabel", 3, ""), s = y(e, "errMsg", 3, ""), c = y(e, "min", 3, "00:00"), m = y(e, "max", 3, "23:59"), $ = je(), E = S(false), P = S(void 0), I, D = S(void 0), Y = S(void 0), C = L(() => Number.parseInt(c().slice(0, 2))), R = L(() => Number.parseInt(m().slice(0, 2))), T = L(() => Number.parseInt(c().slice(3, 5))), H = L(() => Number.parseInt(m().slice(3, 5))), N = S(w(ie(() => a().slice(0, 2)))), U = S(w(ie(() => a().slice(3, 5)))), F = S(false), j = L(() => {
    let b = [];
    for (let l = i(C); l <= i(R); l++) l < 10 ? b.push("0" + l) : b.push("" + l);
    return b;
  }), de = L(() => {
    let b = [];
    for (let l = i(T); l <= i(H); l++) l < 10 ? b.push("0" + l) : b.push("" + l);
    return b;
  });
  ee(() => {
    a().length === 5 && (_(N, w(a().slice(0, 2))), _(U, w(a().slice(3, 5))));
  }), ee(() => {
    i(F) && a(`${i(N)}:${i(U)}`);
  }), ee(() => {
    var _a;
    i(U) && ((_a = i(Y)) == null ? void 0 : _a.focus());
  }), ee(() => {
    var _a;
    i(N) && ((_a = i(D)) == null ? void 0 : _a.focus());
  });
  function Ie(b) {
    let l = b.currentTarget.value;
    l.length === 1 && l.charAt(0) === ":" && (l = "00:"), l.length === 2 && (l.charAt(1) === ":" ? l = `0${l}:` : l += ":"), l = l.replace(/\D/g, ""), l.length >= 3 && (l = `${l.slice(0, 2)}:${l.slice(2)}`), a(l);
  }
  function he(b) {
    b.preventDefault(), _(E, true);
  }
  function pe(b) {
    var _a;
    switch (b) {
      case "open":
        a().length !== 5 && a(Re()), _(F, true), (_a = i(D)) == null ? void 0 : _a.focus(), a() || requestAnimationFrame(() => ve());
        break;
      default:
        _(F, false);
        break;
    }
  }
  function ve() {
    let b = Number.parseInt(i(N)), l = Number.parseInt(i(U));
    (b < i(C) || b > i(R) || l < i(T) || l > i(H)) && (I == null ? void 0 : I.setCustomValidity(s())), _(E, !(I == null ? void 0 : I.reportValidity()));
  }
  var W = mr();
  let me;
  var te = h(W), g = h(te);
  at(g), g.__input = Ie, g.__keydown = [ur, e], Ne(g, (b) => I = b, () => I);
  var re = B(g, 2), M = h(re), be = h(M);
  ut(be, { ariaLabel: "Show Popover Example", offsetLeft: "-3.8rem", offsetTop: "-.2rem", onToggle: pe, btnInvisible: true, get close() {
    return i(P);
  }, set close(l) {
    _(P, w(l));
  }, button: (l) => {
    var u = fr(), v = h(u);
    cr(v, { color: "hsl(var(--text)", width: "1.2rem" }), f(u), V(() => d(u, "title", n())), p(l, u);
  }, children: (l, u) => {
    var v = hr(), k = h(v), x = h(k), ae = h(x);
    ke(ae, { get ariaLabel() {
      return $.common.hours;
    }, get options() {
      return i(j);
    }, maxHeight: "13rem", borderless: true, onRight: () => {
      var _a;
      return (_a = i(Y)) == null ? void 0 : _a.focus();
    }, get ref() {
      return i(D);
    }, set ref(O) {
      _(D, w(O));
    }, get value() {
      return i(N);
    }, set value(O) {
      _(N, w(O));
    } }), f(x);
    var ne = B(x, 4), Te = h(ne);
    ke(Te, { get ariaLabel() {
      return $.common.minutes;
    }, get options() {
      return i(de);
    }, maxHeight: "13rem", borderless: true, onLeft: () => {
      var _a;
      return (_a = i(D)) == null ? void 0 : _a.focus();
    }, get ref() {
      return i(Y);
    }, set ref(O) {
      _(Y, w(O));
    }, get value() {
      return i(U);
    }, set value(O) {
      _(U, w(O));
    } }), f(ne), f(k), f(v), p(l, v);
  }, $$slots: { button: true, default: true } }), f(M), f(re), f(te);
  var ce = B(te, 2), X = h(ce), ye = h(X, true);
  f(X);
  var Z = B(X, 2);
  {
    var Ee = (b) => {
      var l = vr(), u = h(l, true);
      f(l), V(() => oe(u, s())), nt(3, l, () => ot, () => ({ duration: 150 })), p(b, l);
    };
    _e(Z, (b) => {
      i(E) && b(Ee);
    });
  }
  f(ce), f(W), V(() => {
    me = Et(W, "", me, { width: e.width }), d(g, "id", e.id), d(g, "name", r()), d(g, "title", s()), d(g, "aria-label", o() || n()), g.disabled = e.disabled, d(g, "aria-disabled", e.disabled), g.required = e.required, d(g, "aria-required", e.required || false), d(g, "aria-invalid", i(E)), d(X, "for", e.id), d(X, "data-required", e.required), oe(ye, n());
  }), fe("invalid", g, he), fe("focus", g, () => I == null ? void 0 : I.select()), fe("blur", g, () => ve()), st(g, a), p(t, W), Oe();
}
lt(["input", "keydown"]);
var yr = A("<span><!></span>"), gr = A('<div class="delete svelte-obgb62"><!></div>'), wr = A('<div class="container svelte-obgb62"><div class="inputs svelte-obgb62"><!> <!></div> <!></div>');
function Br(t, e) {
  Fe(e, true);
  let r = y(e, "value", 31, () => w(Ue())), a = y(e, "label", 3, ""), n = y(e, "errMsg", 3, ""), o = y(e, "min", 3, "1900-01-01"), s = y(e, "max", 3, "2100-01-01"), c = y(e, "timeValue", 31, () => w(Re())), m = je();
  function $() {
    r(""), c("--:--");
  }
  var E = wr(), P = h(E), I = h(P);
  sr(I, { get id() {
    return e.id;
  }, get name() {
    return e.name;
  }, get label() {
    return a();
  }, get errMsg() {
    return n();
  }, get disabled() {
    return e.disabled;
  }, get min() {
    return o();
  }, get max() {
    return s();
  }, get required() {
    return e.required;
  }, get onEnter() {
    return e.onEnter;
  }, get onLeft() {
    return e.onLeft;
  }, get onRight() {
    return e.onRight;
  }, get onUp() {
    return e.onUp;
  }, get onDown() {
    return e.onDown;
  }, get value() {
    return r();
  }, set value(T) {
    r(T);
  } });
  var D = B(I, 2);
  {
    var Y = (T) => {
      br(T, { get name() {
        return e.timeName;
      }, get ariaLabel() {
        return a();
      }, get errMsg() {
        return e.timeErrMsg;
      }, get min() {
        return e.timeMin;
      }, get max() {
        return e.timeMax;
      }, get disabled() {
        return e.disabled;
      }, get required() {
        return e.required;
      }, get value() {
        return c();
      }, set value(H) {
        c(H);
      } });
    };
    _e(D, (T) => {
      e.withTime && T(Y);
    });
  }
  f(P);
  var C = B(P, 2);
  {
    var R = (T) => {
      var H = gr(), N = h(H);
      it(N, { get ariaLabel() {
        return m.common.delete;
      }, invisible: true, onclick: $, children: (U, F) => {
        var j = yr(), de = h(j);
        Ct(de, { color: "hsla(var(--error) / .8)", width: "1.2rem" }), f(j), V(() => d(j, "title", m.common.delete)), p(U, j);
      }, $$slots: { default: true } }), f(H), p(T, H);
    };
    _e(C, (T) => {
      e.withDelete && T(R);
    });
  }
  f(E), p(t, E), Oe();
}
export {
  Br as I
};
