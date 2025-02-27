const D = { "001": 1, AD: 1, AE: 6, AF: 6, AI: 1, AL: 1, AM: 1, AN: 1, AR: 1, AT: 1, AU: 1, AX: 1, AZ: 1, BA: 1, BE: 1, BG: 1, BH: 6, BM: 1, BN: 1, BY: 1, CH: 1, CL: 1, CM: 1, CN: 1, CR: 1, CY: 1, CZ: 1, DE: 1, DJ: 6, DK: 1, DZ: 6, EC: 1, EE: 1, EG: 6, ES: 1, FI: 1, FJ: 1, FO: 1, FR: 1, GB: 1, GE: 1, GF: 1, GP: 1, GR: 1, HR: 1, HU: 1, IE: 1, IQ: 6, IR: 6, IS: 1, IT: 1, JO: 6, KG: 1, KW: 6, KZ: 1, LB: 1, LI: 1, LK: 1, LT: 1, LU: 1, LV: 1, LY: 6, MC: 1, MD: 1, ME: 1, MK: 1, MN: 1, MQ: 1, MV: 5, MY: 1, NL: 1, NO: 1, NZ: 1, OM: 6, PL: 1, QA: 6, RE: 1, RO: 1, RS: 1, RU: 1, SD: 6, SE: 1, SI: 1, SK: 1, SM: 1, SY: 6, TJ: 1, TM: 1, TR: 1, UA: 1, UY: 1, UZ: 1, VA: 1, VN: 1, XK: 1 };
function u(e, t, a) {
  let n = e.calendar.toJulianDay(e), f = M(t), r = Math.ceil(n + 1 - f) % 7;
  return r < 0 && (r += 7), r;
}
function g(e, t) {
  return e.calendar.toJulianDay(e) - t.calendar.toJulianDay(t);
}
function E(e, t) {
  return l(e) - l(t);
}
function l(e) {
  return e.hour * 36e5 + e.minute * 6e4 + e.second * 1e3 + e.millisecond;
}
let o = null;
function T() {
  return o == null && (o = new Intl.DateTimeFormat().resolvedOptions().timeZone), o;
}
function m(e) {
  return e.subtract({ days: e.day - 1 });
}
function h(e, t, a) {
  let n = u(e, t);
  return e.subtract({ days: n });
}
const y = /* @__PURE__ */ new Map();
function d(e) {
  if (Intl.Locale) {
    let a = y.get(e);
    return a || (a = new Intl.Locale(e).maximize().region, a && y.set(e, a)), a;
  }
  let t = e.split("-")[1];
  return t === "u" ? void 0 : t;
}
function M(e) {
  let t = d(e);
  return t && D[t] || 0;
}
const p = { AF: [4, 5], AE: [5, 6], BH: [5, 6], DZ: [5, 6], EG: [5, 6], IL: [5, 6], IQ: [5, 6], IR: [5, 5], JO: [5, 6], KW: [5, 6], LY: [5, 6], OM: [5, 6], QA: [5, 6], SA: [5, 6], SD: [5, 6], SY: [5, 6], YE: [5, 6] };
function A(e, t) {
  let a = e.calendar.toJulianDay(e), n = Math.ceil(a + 1) % 7;
  n < 0 && (n += 7);
  let f = d(t), [r, i] = p[f] || [6, 0];
  return n === r || n === i;
}
function I(e) {
  let t = Date.parse(e.split("T")[0]);
  if (!isNaN(t)) return t / 1e3;
}
function L(e, t) {
  let a = Date.parse(`${e}T${t}`);
  if (!isNaN(a)) return a / 1e3 - (/* @__PURE__ */ new Date()).getTimezoneOffset();
}
function k(e) {
  return `${e.year}-${e.month > 9 ? e.month : "0" + e.month}-${e.day > 9 ? e.day : "0" + e.day}`;
}
function O(e, t) {
  const a = e.month;
  e = m(e), e = h(e, t);
  let n = [];
  const f = (c) => {
    c.push({ day: e.day, month: e.month, year: e.year, isWeekend: A(e, t) }), e = e.add({ days: 1 });
  }, r = () => {
    let c = { days: [] };
    for (let $ = 0; $ < 7; $++) f(c.days);
    n.push(c);
  };
  for (r(); e.month === a; ) r();
  let i = n[n.length - 1], s = u(e, t);
  for (; s > 0; ) f(i.days), s = u(e, t);
  return n[n.length - 1] = i, n;
}
function S(e) {
  const t = e || /* @__PURE__ */ new Date();
  let a = t.getDate();
  a < 10 && (a = "0" + a);
  let n = t.getMonth() + 1;
  return n < 10 && (n = "0" + n), `${t.getUTCFullYear()}-${n}-${a}`;
}
function W(e) {
  const t = e || /* @__PURE__ */ new Date();
  let a = t.getHours();
  a < 10 && (a = "0" + a);
  let n = t.getMinutes();
  return n < 10 && (n = "0" + n), `${a}:${n}`;
}
export {
  T as $,
  W as a,
  L as b,
  g as c,
  E as d,
  k as e,
  S as f,
  O as g,
  u as h,
  I as u
};
