import { n as rt, a as p, t as A } from "./BH6NCLk-.js";
import { t as V, p as He, k as S, O as ie, j as i, a9 as L, aa as re, l as x, c as h, s as B, r as u, a as Oe } from "./CvlvO1XB.js";
import { i as _e } from "./BUO_AUgz.js";
import { s as d, r as at } from "./BMbqVy6X.js";
import { p as g } from "./Wh68IIk2.js";
import { p as y } from "./C6SR4G2t.js";
import { t as nt, B as it, s as ot, a as Et } from "./DMkkW5Nn.js";
import { d as lt, s as oe, e as ve } from "./CTI4QPiR.js";
import { e as Le, i as Ye } from "./BpWRzPRQ.js";
import { b as st } from "./BhIBACXG.js";
import { b as Pe } from "./zosqiMUL.js";
import { $ as dt, c as ct, d as Tt, f as Ne, g as St, e as Qe, h as At, a as Ue } from "./DswDW5U8.js";
import { u as Fe } from "./DOl8_ubJ.js";
import { P as ut, O as ke } from "./CGniv4pe.js";
import { I as Ct } from "./Vi3uK7uO.js";
function qe(t, e) {
  return t - e * Math.floor(t / e);
}
const ft = 1721426;
function we(t, e, r, a) {
  e = je(t, e);
  let n = e - 1, o = -2;
  return r <= 2 ? o = 0 : xe(e) && (o = -1), ft - 1 + 365 * n + Math.floor(n / 4) - Math.floor(n / 100) + Math.floor(n / 400) + Math.floor((367 * r - 362) / 12 + o + a);
}
function xe(t) {
  return t % 4 === 0 && (t % 100 !== 0 || t % 400 === 0);
}
function je(t, e) {
  return t === "BC" ? 1 - e : e;
}
function Lt(t) {
  let e = "AD";
  return t <= 0 && (e = "BC", t = 1 - t), [e, t];
}
const Yt = { standard: [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31], leapyear: [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31] };
class $e {
  fromJulianDay(e) {
    let r = e, a = r - ft, n = Math.floor(a / 146097), o = qe(a, 146097), s = Math.floor(o / 36524), c = qe(o, 36524), v = Math.floor(c / 1461), $ = qe(c, 1461), E = Math.floor($ / 365), P = n * 400 + s * 100 + v * 4 + E + (s !== 4 && E !== 4 ? 1 : 0), [I, D] = Lt(P), Y = r - we(I, D, 1, 1), C = 2;
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
  t = We(t, new $e());
  let e = je(t.era, t.year);
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
    a = We(a, new $e());
    let v = /* @__PURE__ */ new Date(), $ = je(a.era, a.year);
    return v.setFullYear($, a.month - 1, a.day), v.setHours(a.hour, a.minute, a.second, a.millisecond), v.getTime();
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
function We(t, e) {
  if (t.calendar.identifier === e.identifier) return t;
  let r = e.fromJulianDay(t.calendar.toJulianDay(t)), a = t.copy();
  return a.calendar = e, a.era = r.era, a.year = r.year, a.month = r.month, a.day = r.day, le(a), a;
}
function Je(t, e) {
  let r = t.copy(), a = "hour" in r ? Ot(r, e) : 0;
  Re(r, e.years || 0), r.calendar.balanceYearMonth && r.calendar.balanceYearMonth(r, t), r.month += e.months || 0, Ve(r), yt(r), r.day += (e.weeks || 0) * 7, r.day += e.days || 0, r.day += a, Nt(r), r.calendar.balanceDate && r.calendar.balanceDate(r), r.year < 1 && (r.year = 1, r.month = 1, r.day = 1);
  let n = r.calendar.getYearsInEra(r);
  if (r.year > n) {
    var o, s;
    let v = (o = (s = r.calendar).isInverseEra) === null || o === void 0 ? void 0 : o.call(s, r);
    r.year = n, r.month = v ? 1 : r.calendar.getMonthsInYear(r), r.day = v ? 1 : r.calendar.getDaysInMonth(r);
  }
  r.month < 1 && (r.month = 1, r.day = 1);
  let c = r.calendar.getMonthsInYear(r);
  return r.month > c && (r.month = c, r.day = r.calendar.getDaysInMonth(r)), r.day = Math.max(1, Math.min(r.calendar.getDaysInMonth(r), r.day)), r;
}
function Re(t, e) {
  var r, a;
  !((r = (a = t.calendar).isInverseEra) === null || r === void 0) && r.call(a, t) && (e = -e), t.year += e;
}
function Ve(t) {
  for (; t.month < 1; ) Re(t, -1), t.month += t.calendar.getMonthsInYear(t);
  let e = 0;
  for (; t.month > (e = t.calendar.getMonthsInYear(t)); ) t.month -= e, Re(t, 1);
}
function Nt(t) {
  for (; t.day < 1; ) t.month--, Ve(t), t.day += t.calendar.getDaysInMonth(t);
  for (; t.day > t.calendar.getDaysInMonth(t); ) t.day -= t.calendar.getDaysInMonth(t), t.month++, Ve(t);
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
  return Je(t, Ut(e));
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
function Ot(t, e) {
  return t.hour += e.hours || 0, t.minute += e.minutes || 0, t.second += e.seconds || 0, t.millisecond += e.milliseconds || 0, Vt(t);
}
function Mt(t, e, r, a) {
  let n = t.copy();
  switch (e) {
    case "era": {
      let c = t.calendar.getEras(), v = c.indexOf(t.era);
      if (v < 0) throw new Error("Invalid era: " + t.era);
      v = Z(v, r, 0, c.length - 1, a == null ? void 0 : a.round), n.era = c[v], le(n);
      break;
    }
    case "year":
      var o, s;
      !((o = (s = n.calendar).isInverseEra) === null || o === void 0) && o.call(s, n) && (r = -r), n.year = Z(t.year, r, -1 / 0, 9999, a == null ? void 0 : a.round), n.year === -1 / 0 && (n.year = 1), n.calendar.balanceYearMonth && n.calendar.balanceYearMonth(n, t);
      break;
    case "month":
      n.month = Z(t.month, r, 1, t.calendar.getMonthsInYear(t), a == null ? void 0 : a.round);
      break;
    case "day":
      n.day = Z(t.day, r, 1, t.calendar.getDaysInMonth(t), a == null ? void 0 : a.round);
      break;
    default:
      throw new Error("Unsupported field " + e);
  }
  return t.calendar.balanceDate && t.calendar.balanceDate(n), le(n), n;
}
function Ft(t, e, r, a) {
  let n = t.copy();
  switch (e) {
    case "hour": {
      let o = t.hour, s = 0, c = 23;
      if ((a == null ? void 0 : a.hourCycle) === 12) {
        let v = o >= 12;
        s = v ? 12 : 0, c = v ? 23 : 11;
      }
      n.hour = Z(o, r, s, c, a == null ? void 0 : a.round);
      break;
    }
    case "minute":
      n.minute = Z(t.minute, r, 0, 59, a == null ? void 0 : a.round);
      break;
    case "second":
      n.second = Z(t.second, r, 0, 59, a == null ? void 0 : a.round);
      break;
    case "millisecond":
      n.millisecond = Z(t.millisecond, r, 0, 999, a == null ? void 0 : a.round);
      break;
    default:
      throw new Error("Unsupported field " + e);
  }
  return n;
}
function Z(t, e, r, a, n = false) {
  if (n) {
    t += Math.sign(e), t < r && (t = a);
    let o = Math.abs(e);
    e > 0 ? t = Math.ceil(t / o) * o : t = Math.floor(t / o) * o, t > a && (t = r);
  } else t += e, t < r ? t = a - (r - t - 1) : t > a && (t = r + (t - a - 1));
  return t;
}
const jt = /^([+-]\d{6}|\d{4})-(\d{2})-(\d{2})$/;
function X(t) {
  let e = t.match(jt);
  if (!e) throw new Error("Invalid ISO 8601 date string: " + t);
  let r = new se(Be(e[1], 0, 9999), Be(e[2], 1, 12), 1);
  return r.day = Be(e[3], 0, r.calendar.getDaysInMonth(r)), r;
}
function Be(t, e, r) {
  let a = Number(t);
  if (a < e || a > r) throw new RangeError(`Value out of range: ${e} <= ${a} <= ${r}`);
  return a;
}
function Wt(t) {
  return `${String(t.hour).padStart(2, "0")}:${String(t.minute).padStart(2, "0")}:${String(t.second).padStart(2, "0")}${t.millisecond ? String(t.millisecond / 1e3).slice(1) : ""}`;
}
function xt(t) {
  let e = We(t, new $e()), r;
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
    return Je(this, e);
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
    return Je(this, e);
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
        return Ft(this, e, r, a);
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
  He(e, true);
  let r = y(e, "value", 31, () => g(Ne())), a = y(e, "label", 3, ""), n = y(e, "errMsg", 3, ""), o = y(e, "min", 3, "1900-01-01"), s = y(e, "max", 3, "2100-01-01");
  const c = X(Ne()), v = c.toString();
  let $ = Fe(), E = S(false), P = S(void 0), I, D = S(void 0), Y = S(void 0), C = S(void 0), R = L(() => r() ? X(r()) : c), T = L(() => X(o())), H = L(() => X(s())), N = S(g(ie(() => $.common.months[i(R).month - 1]))), U = L(() => $.common.months.findIndex((f) => f === i(N)) + 1), O = S(g(ie(() => i(R).year))), F = S(g([ie(() => i(O))])), de = L(() => St(i(R), $.lang));
  re(() => {
    let f = Number.parseInt(o().slice(0, 4)), m = Number.parseInt(s().slice(0, 4)), M = [];
    for (let k = f; k <= m; k++) M.push(k);
    x(F, g(M));
  }), re(() => {
    if (r()) {
      let f = ie(() => X(r()));
      f = f.set({ year: i(O), month: i(U) }), r(f.toString());
    }
  }), re(() => {
    i(C) && r() && requestAnimationFrame(() => {
      var _a, _b, _c;
      let f = ((_a = i(C)) == null ? void 0 : _a.getElementsByTagName("time")) || [];
      for (let m of f) if (m.getAttribute("datetime") === r()) {
        (_c = (_b = m.parentElement) == null ? void 0 : _b.parentElement) == null ? void 0 : _c.focus();
        break;
      }
    });
  });
  function Ie(f) {
    let m = X(Qe(f));
    return m.compare(i(T)) < 0 || m.compare(i(H)) > 0 ? false : i(U) === f.month;
  }
  function me() {
    r(new se(i(O), i(U), 1).toString());
  }
  function pe(f) {
    f.preventDefault(), x(E, true);
  }
  function be(f, m) {
    var _a;
    f.preventDefault(), r(m), (_a = i(P)) == null ? void 0 : _a();
  }
  function j(f, m) {
    var _a, _b;
    let M = X(m), k = X(m);
    switch (f) {
      case "left":
        k = M.subtract({ days: 1 });
        break;
      case "right":
        k = M.add({ days: 1 });
        break;
      case "up":
        k = M.subtract({ weeks: 1 });
        break;
      case "down":
        k = M.add({ weeks: 1 });
        break;
    }
    k.month !== M.month ? At(k, $.lang) < 4 ? (_a = i(D)) == null ? void 0 : _a.focus() : (_b = i(Y)) == null ? void 0 : _b.focus() : r(k.toString());
  }
  function ce(f) {
    switch (f) {
      case "open":
        r() || (r(v), requestAnimationFrame(() => w()));
        break;
      case "closed":
        x(C, void 0);
        break;
    }
  }
  function w() {
    x(E, !(I == null ? void 0 : I.reportValidity()));
  }
  var ae = lr(), ne = h(ae), _ = h(ne);
  at(_), _.__keydown = [Zt, e], Pe(_, (f) => I = f, () => I);
  var ue = B(_, 2), z = B(h(ue), 2), Ee = h(z);
  ut(Ee, { ariaLabel: "Show Popover Example", offsetLeft: "-7.5rem", offsetTop: "-.2rem", onToggle: ce, btnInvisible: true, lazy: true, get close() {
    return i(P);
  }, set close(m) {
    x(P, g(m));
  }, button: (m) => {
    var M = er(), k = h(M);
    Xt(k, { color: "hsl(var(--text)", width: "1.2rem" }), u(M), V(() => d(M, "title", a())), p(m, M);
  }, children: (m, M) => {
    var k = ir(), te = h(k), fe = h(te), J = h(fe);
    ke(J, { get ariaLabel() {
      return $.common.month;
    }, get options() {
      return $.common.months;
    }, maxHeight: "13rem", borderless: true, onRight: () => {
      var _a;
      return (_a = i(Y)) == null ? void 0 : _a.focus();
    }, onDown: me, get ref() {
      return i(D);
    }, set ref(q) {
      x(D, g(q));
    }, get value() {
      return i(N);
    }, set value(q) {
      x(N, g(q));
    } }), u(fe);
    var ze = B(fe, 2), Dt = h(ze);
    ke(Dt, { get ariaLabel() {
      return $.common.year;
    }, get options() {
      return i(F);
    }, maxHeight: "13rem", borderless: true, onLeft: () => {
      var _a;
      return (_a = i(D)) == null ? void 0 : _a.focus();
    }, onDown: me, get ref() {
      return i(Y);
    }, set ref(q) {
      x(Y, g(q));
    }, get value() {
      return i(O);
    }, set value(q) {
      x(O, g(q));
    } }), u(ze), u(te);
    var Te = B(te, 2);
    Le(Te, 21, () => $.common.weekDaysShort, Ye, (q, Ae, ge, Ke) => {
      var K = tr(), he = h(K, true);
      u(K), V(() => oe(he, i(Ae))), p(q, K);
    }), u(Te);
    var Se = B(Te, 2);
    Le(Se, 21, () => i(de), Ye, (q, Ae) => {
      var ge = nr();
      Le(ge, 21, () => i(Ae).days, Ye, (Ke, K) => {
        var he = ar();
        const G = L(() => Qe(i(K))), Ge = L(() => Ie(i(K)));
        var $t = h(he);
        const It = L(() => !i(Ge));
        it($t, { onclick: (Ce) => be(Ce, i(G)), get isDisabled() {
          return i(It);
        }, invisible: true, onLeft: () => j("left", i(G)), onRight: () => j("right", i(G)), onUp: () => j("up", i(G)), onDown: () => j("down", i(G)), children: (Ce, Mr) => {
          var Q = rr(), pt = h(Q, true);
          u(Q), V(() => {
            d(Q, "data-today", i(G) === v), d(Q, "data-active", i(Ge)), d(Q, "data-selected", i(G) === r()), d(Q, "data-weekend", i(K).isWeekend), d(Q, "datetime", i(G)), oe(pt, i(K).day);
          }), p(Ce, Q);
        }, $$slots: { default: true } }), u(he), p(Ke, he);
      }), u(ge), p(q, ge);
    }), u(Se), Pe(Se, (q) => x(C, q), () => i(C)), u(k), p(m, k);
  }, $$slots: { button: true, default: true } }), u(z), u(ue), u(ne);
  var ye = B(ne, 2), ee = h(ye), b = h(ee, true);
  u(ee);
  var l = B(ee, 2);
  {
    var W = (f) => {
      var m = or(), M = h(m, true);
      u(m), V(() => oe(M, n())), nt(3, m, () => ot), p(f, m);
    };
    _e(l, (f) => {
      i(E) && f(W);
    });
  }
  u(ye), u(ae), V(() => {
    d(_, "id", e.id), d(_, "name", e.name), d(_, "title", a()), d(_, "aria-label", a()), _.disabled = e.disabled, d(_, "aria-disabled", e.disabled), _.required = e.required, d(_, "aria-required", e.required || false), d(_, "aria-invalid", i(E)), d(_, "min", o()), d(_, "max", s()), d(ee, "for", e.id), d(ee, "data-required", e.required), oe(b, a());
  }), ve("invalid", _, pe), ve("blur", _, () => w()), st(_, r), p(t, ae), Oe();
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
  He(e, true);
  let r = y(e, "name", 3, "time"), a = y(e, "value", 31, () => g(Ue())), n = y(e, "label", 3, ""), o = y(e, "ariaLabel", 3, ""), s = y(e, "errMsg", 3, ""), c = y(e, "min", 3, "00:00"), v = y(e, "max", 3, "23:59"), $ = Fe(), E = S(false), P = S(void 0), I, D = S(void 0), Y = S(void 0), C = L(() => Number.parseInt(c().slice(0, 2))), R = L(() => Number.parseInt(v().slice(0, 2))), T = L(() => Number.parseInt(c().slice(3, 5))), H = L(() => Number.parseInt(v().slice(3, 5))), N = S(g(ie(() => a().slice(0, 2)))), U = S(g(ie(() => a().slice(3, 5)))), O = S(false), F = L(() => {
    let b = [];
    for (let l = i(C); l <= i(R); l++) l < 10 ? b.push("0" + l) : b.push("" + l);
    return b;
  }), de = L(() => {
    let b = [];
    for (let l = i(T); l <= i(H); l++) l < 10 ? b.push("0" + l) : b.push("" + l);
    return b;
  });
  re(() => {
    a().length === 5 && (x(N, g(a().slice(0, 2))), x(U, g(a().slice(3, 5))));
  }), re(() => {
    i(O) && a(`${i(N)}:${i(U)}`);
  }), re(() => {
    var _a;
    i(U) && ((_a = i(Y)) == null ? void 0 : _a.focus());
  }), re(() => {
    var _a;
    i(N) && ((_a = i(D)) == null ? void 0 : _a.focus());
  });
  function Ie(b) {
    let l = b.currentTarget.value;
    l.length === 1 && l.charAt(0) === ":" && (l = "00:"), l.length === 2 && (l.charAt(1) === ":" ? l = `0${l}:` : l += ":"), l = l.replace(/\D/g, ""), l.length >= 3 && (l = `${l.slice(0, 2)}:${l.slice(2)}`), a(l);
  }
  function me(b) {
    b.preventDefault(), x(E, true);
  }
  function pe(b) {
    var _a;
    switch (b) {
      case "open":
        a().length !== 5 && a(Ue()), x(O, true), (_a = i(D)) == null ? void 0 : _a.focus(), a() || requestAnimationFrame(() => be());
        break;
      default:
        x(O, false);
        break;
    }
  }
  function be() {
    let b = Number.parseInt(i(N)), l = Number.parseInt(i(U));
    (b < i(C) || b > i(R) || l < i(T) || l > i(H)) && (I == null ? void 0 : I.setCustomValidity(s())), x(E, !(I == null ? void 0 : I.reportValidity()));
  }
  var j = mr(), ce = h(j), w = h(ce);
  at(w), w.__input = Ie, w.__keydown = [ur, e], Pe(w, (b) => I = b, () => I);
  var ae = B(w, 2), ne = h(ae), _ = h(ne);
  ut(_, { ariaLabel: "Show Popover Example", offsetLeft: "-3.8rem", offsetTop: "-.2rem", onToggle: pe, btnInvisible: true, get close() {
    return i(P);
  }, set close(l) {
    x(P, g(l));
  }, button: (l) => {
    var W = fr(), f = h(W);
    cr(f, { color: "hsl(var(--text)", width: "1.2rem" }), u(W), V(() => d(W, "title", n())), p(l, W);
  }, children: (l, W) => {
    var f = hr(), m = h(f), M = h(m), k = h(M);
    ke(k, { get ariaLabel() {
      return $.common.hours;
    }, get options() {
      return i(F);
    }, maxHeight: "13rem", borderless: true, onRight: () => {
      var _a;
      return (_a = i(Y)) == null ? void 0 : _a.focus();
    }, get ref() {
      return i(D);
    }, set ref(J) {
      x(D, g(J));
    }, get value() {
      return i(N);
    }, set value(J) {
      x(N, g(J));
    } }), u(M);
    var te = B(M, 4), fe = h(te);
    ke(fe, { get ariaLabel() {
      return $.common.minutes;
    }, get options() {
      return i(de);
    }, maxHeight: "13rem", borderless: true, onLeft: () => {
      var _a;
      return (_a = i(D)) == null ? void 0 : _a.focus();
    }, get ref() {
      return i(Y);
    }, set ref(J) {
      x(Y, g(J));
    }, get value() {
      return i(U);
    }, set value(J) {
      x(U, g(J));
    } }), u(te), u(m), u(f), p(l, f);
  }, $$slots: { button: true, default: true } }), u(ne), u(ae), u(ce);
  var ue = B(ce, 2), z = h(ue), Ee = h(z, true);
  u(z);
  var ye = B(z, 2);
  {
    var ee = (b) => {
      var l = vr(), W = h(l, true);
      u(l), V(() => oe(W, s())), nt(3, l, () => ot), p(b, l);
    };
    _e(ye, (b) => {
      i(E) && b(ee);
    });
  }
  u(ue), u(j), V(() => {
    Et(j, "width", e.width), d(w, "id", e.id), d(w, "name", r()), d(w, "title", s()), d(w, "aria-label", o() || n()), w.disabled = e.disabled, d(w, "aria-disabled", e.disabled), w.required = e.required, d(w, "aria-required", e.required || false), d(w, "aria-invalid", i(E)), d(z, "for", e.id), d(z, "data-required", e.required), oe(Ee, n());
  }), ve("invalid", w, me), ve("focus", w, () => I == null ? void 0 : I.select()), ve("blur", w, () => be()), st(w, a), p(t, j), Oe();
}
lt(["input", "keydown"]);
var yr = A("<span><!></span>"), gr = A('<div class="delete svelte-obgb62"><!></div>'), wr = A('<div class="container svelte-obgb62"><div class="inputs svelte-obgb62"><!> <!></div> <!></div>');
function Br(t, e) {
  He(e, true);
  let r = y(e, "value", 31, () => g(Ne())), a = y(e, "label", 3, ""), n = y(e, "errMsg", 3, ""), o = y(e, "min", 3, "1900-01-01"), s = y(e, "max", 3, "2100-01-01"), c = y(e, "timeValue", 31, () => g(Ue())), v = Fe();
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
  u(P);
  var C = B(P, 2);
  {
    var R = (T) => {
      var H = gr(), N = h(H);
      it(N, { get ariaLabel() {
        return v.common.delete;
      }, invisible: true, onclick: $, children: (U, O) => {
        var F = yr(), de = h(F);
        Ct(de, { color: "hsla(var(--error) / .8)", width: "1.2rem" }), u(F), V(() => d(F, "title", v.common.delete)), p(U, F);
      }, $$slots: { default: true } }), u(H), p(T, H);
    };
    _e(C, (T) => {
      e.withDelete && T(R);
    });
  }
  u(E), p(t, E), Oe();
}
export {
  Br as I
};
