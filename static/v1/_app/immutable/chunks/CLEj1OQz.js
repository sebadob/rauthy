import "./DLBGyKVC.js";
import { p as a, a6 as i, a as f } from "./CmQi0fbH.js";
import { p as n } from "./DNJm3-SG.js";
import { u as s } from "./Dd1y69dN.js";
import { f as l } from "./BO1A6s0c.js";
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
