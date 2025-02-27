import { b as p } from "./bbkAiDd0.js";
import { b as i, a as s } from "./CY7Th9O-.js";
import { p as y } from "./CGXT-546.js";
async function x(o, n, l, d) {
  var _a, _b;
  let b = { passkey_name: n }, a = await p(`/auth/v1/users/${o}/webauthn/register/start`, b);
  if (!a.body) return { error: ((_a = a.error) == null ? void 0 : _a.message) || "did not receive any registration data" };
  let e = a.body;
  if (!e.publicKey) {
    let t = "no publicKey in options from the backend";
    return console.error(t), { error: t };
  }
  if (e.publicKey.challenge = i(e.publicKey.challenge), e.publicKey.user.id = i(e.publicKey.user.id), e.publicKey.excludeCredentials) for (let t of e.publicKey.excludeCredentials) t.id = i(t.id);
  let u = (e.publicKey.timeout || 6e4) - 1e3;
  const f = (/* @__PURE__ */ new Date()).getTime() + u;
  let r;
  try {
    const t = await y(navigator.credentials.create(e), u);
    if (t) r = t;
    else return { error: l };
  } catch {
    return { error: (/* @__PURE__ */ new Date()).getTime() >= f ? d : l };
  }
  let m = { passkey_name: n, data: { id: r.id, rawId: s(r.rawId), response: { attestationObject: s(r.response.attestationObject), clientDataJSON: s(r.response.clientDataJSON) }, extensions: r.getClientExtensionResults(), type: r.type } }, c = await p(`/auth/v1/users/${o}/webauthn/register/finish`, m);
  return c.status === 201 ? {} : { error: ((_b = c.error) == null ? void 0 : _b.message) || "Registration failed" };
}
export {
  x as w
};
