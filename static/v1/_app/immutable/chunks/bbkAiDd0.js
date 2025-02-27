import { C as f } from "./CGXT-546.js";
function u(e, n) {
  let t;
  return n === "json" ? t = { "Content-Type": "application/json", Accept: "application/json" } : t = { "Content-type": "application/x-www-form-urlencoded", Accept: "application/json" }, e !== "GET" && (t["csrf-token"] = localStorage.getItem(f)), t;
}
async function p(e, n = "json", t = "default") {
  return r("GET", e, n, t);
}
async function h(e, n, t = "json") {
  return n ? c("POST", e, t, n) : r("POST", e, t);
}
async function w(e, n) {
  return await fetch(e.action, { method: e.method, headers: { "content-type": "application/x-www-form-urlencoded" }, body: n });
}
async function y(e, n, t = "json") {
  return n ? c("PUT", e, t, n) : r("PUT", e, t);
}
async function m(e, n, t = "json") {
  return n ? await c("DELETE", e, t, n) : await r("DELETE", e, t);
}
async function r(e, n, t, a) {
  let o = await fetch(n, { method: e, headers: u(e, t), redirect: "manual", cache: a });
  return await i(o);
}
async function c(e, n, t, a) {
  let o;
  t === "json" ? o = JSON.stringify(a) : t === "form" && (o = l(a));
  let s = await fetch(n, { method: e, headers: u(e, t), redirect: "manual", body: o });
  return i(s);
}
function l(e) {
  let n = new FormData();
  for (let t of Object.keys(e)) {
    let a = e[t];
    typeof a == "object" ? n.append(t, JSON.stringify(a)) : n.append(t, e[t]);
  }
  return n;
}
async function i(e, n) {
  e.status === 401 && window.location.reload();
  let t = { body: void 0, text: void 0, error: void 0, status: e.status, headers: e.headers };
  return e.ok ? e.headers.get("content-type") === "application/json" ? t.body = await e.json() : t.text = await e.text() : e.status !== 405 && (t.error = await e.json()), t;
}
async function j(e, n) {
  return await e.json();
}
async function T(e, n, t, a) {
  const o = new FormData();
  o.append(a, t);
  const s = await fetch(n, { method: e, headers: { "csrf-token": localStorage.getItem(f) || "" }, body: o });
  return i(s);
}
export {
  w as a,
  h as b,
  m as c,
  y as d,
  j as e,
  p as f,
  T as u
};
