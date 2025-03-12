import { b as s, a as o } from "./C3pTUZii.js";
import { C as h, p as w } from "./B21bTIl7.js";
async function T(n, l, c, f, u, p) {
  var _a;
  let b = { passkey_name: l, magic_link_id: u }, i = { "Content-Type": "application/json", Accept: "application/json" };
  p ? i["x-pwd-csrf-token"] = p : i["x-csrf-token"] = localStorage.getItem(h) || "";
  let a = await (await fetch(`/auth/v1/users/${n}/webauthn/register/start`, { method: "POST", headers: i, body: JSON.stringify(b) })).json();
  if ("error" in a) return { error: a.error.message || "did not receive any registration data" };
  let e = a;
  if (!e.publicKey) {
    let t = "no publicKey in options from the backend";
    return console.error(t, e), { error: t };
  }
  if (e.publicKey.challenge = s(e.publicKey.challenge), e.publicKey.user.id = s(e.publicKey.user.id), e.publicKey.excludeCredentials) for (let t of e.publicKey.excludeCredentials) t.id = s(t.id);
  let d = (e.publicKey.timeout || 6e4) - 1e3;
  const m = (/* @__PURE__ */ new Date()).getTime() + d;
  let r;
  try {
    const t = await w(navigator.credentials.create(e), d);
    if (t) r = t;
    else return { error: c };
  } catch {
    return { error: (/* @__PURE__ */ new Date()).getTime() >= m ? f : c };
  }
  let g = { passkey_name: l, data: { id: r.id, rawId: o(r.rawId), response: { attestationObject: o(r.response.attestationObject), clientDataJSON: o(r.response.clientDataJSON) }, extensions: r.getClientExtensionResults(), type: r.type }, magic_link_id: u }, y = await fetch(`/auth/v1/users/${n}/webauthn/register/finish`, { method: "POST", headers: i, body: JSON.stringify(g) });
  return y.status === 201 ? {} : { error: ((_a = (await y.json()).error) == null ? void 0 : _a.message) || "Registration failed" };
}
export {
  T as w
};
