import { C as f } from "./BaE7H8ny.js";
function l(e, n) {
  let t;
  return n === "json" ? t = { "Content-Type": "application/json", Accept: "application/json" } : t = { "Content-type": "application/x-www-form-urlencoded", Accept: "application/json" }, e !== "GET" && (t["csrf-token"] = localStorage.getItem(f)), t;
}
async function p(e, n = "json", t = "handle401") {
  return s("GET", e, n, t);
}
async function w(e, n, t = "json", a = "handle401") {
  return n ? c("POST", e, t, a, n) : s("POST", e, t, a);
}
async function y(e, n) {
  return await fetch(e.action, { method: e.method, headers: { "content-type": "application/x-www-form-urlencoded" }, body: n });
}
async function m(e, n, t = "json", a = "handle401") {
  return n ? c("PUT", e, t, a, n) : s("PUT", e, t, a);
}
async function j(e, n, t = "json", a = "handle401") {
  return n ? await c("DELETE", e, t, a, n) : await s("DELETE", e, t, a);
}
async function s(e, n, t, a) {
  let o = await fetch(n, { method: e, headers: l(e, t), redirect: "manual" });
  return await i(o, a);
}
async function c(e, n, t, a, o) {
  let r;
  t === "json" ? r = JSON.stringify(o) : t === "form" && (r = d(o));
  let u = await fetch(n, { method: e, headers: l(e, t), redirect: "manual", body: r });
  return i(u, a);
}
function d(e) {
  let n = new FormData();
  for (let t of Object.keys(e)) {
    let a = e[t];
    typeof a == "object" ? n.append(t, JSON.stringify(a)) : n.append(t, e[t]);
  }
  return n;
}
async function i(e, n) {
  n === "handle401" && e.status === 401 && window.location.reload();
  let t = { body: void 0, text: void 0, error: void 0, status: e.status, headers: e.headers };
  return e.ok ? e.headers.get("content-type") === "application/json" ? t.body = await e.json() : t.text = await e.text() : e.status !== 405 && (t.error = await e.json()), t;
}
async function T(e, n) {
  return await e.json();
}
async function E(e, n, t, a) {
  const o = new FormData();
  o.append(a, t);
  const r = await fetch(n, { method: e, headers: { "csrf-token": localStorage.getItem(f) || "" }, body: o });
  return i(r, "handle401");
}
export {
  y as a,
  w as b,
  j as c,
  m as d,
  T as e,
  p as f,
  E as u
};
