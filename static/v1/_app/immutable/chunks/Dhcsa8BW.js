import "./BxmJRzoY.js";
import { p as a, a0 as i, a as f } from "./w0HvPX0p.js";
import { p as n } from "./C6GSeq7M.js";
import { u as s } from "./F_Qf1tHt.js";
import { f as l } from "./UPFlzoow.js";
function g(m, r) {
  a(r, true);
  let t = n(r, "value", 15);
  i(async () => {
    if (s().get()) {
      let e = await l(`/auth/v1/template/${r.id}`);
      e.error ? console.error(e.error) : e.text && o(e.text);
    } else {
      let e = document.getElementById(r.id);
      e && o(e.innerHTML);
    }
  });
  function o(e) {
    typeof t() == "boolean" ? t(e === "true") : typeof t() == "string" ? t(e) : typeof t() == "number" ? t(Number.parseInt(e)) : t(JSON.parse(e));
  }
  f();
}
export {
  g as T
};
