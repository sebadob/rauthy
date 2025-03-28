import { a1 as s } from "./w0HvPX0p.js";
import { g as f } from "./B3MKgqW_.js";
let o = s({}), r = typeof window < "u" ? new URLSearchParams(window.location.search) : void 0;
const c = (t, n) => {
  if (!r && typeof window < "u") {
    r = new URLSearchParams(window.location.search);
    for (let [e, a] of o.entries()) r.set(e, a);
  }
  if (!o[t]) {
    let e = r == null ? void 0 : r.get(t);
    e ? (o[t] = e, r && r.set(t, e)) : n && (o[t] = n, r && (r.set(t, n.toString()), i()));
  }
  return { get() {
    return o[t];
  }, getNum() {
    let e = Number.parseInt(o[t]);
    if (!isNaN(e)) return e;
  }, true() {
    return o[t] === true || o[t] === "true";
  }, set(e) {
    e ? (o[t] = e.toString(), r && r.set(t, e.toString())) : (o[t] = void 0, r && r.delete(t)), i();
  } };
};
function i() {
  if (typeof window < "u" && r) {
    let t = r.toString().replace(" ", "+");
    f(`?${t}`, {}).catch((n) => console.error(n));
  }
}
export {
  c as u
};
