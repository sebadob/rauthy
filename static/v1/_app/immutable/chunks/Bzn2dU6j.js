import "./D8nUqfqE.js";
import { p as a, a5 as i, a as f } from "./D-nwkJyM.js";
import { p as n } from "./2rFDT0Lm.js";
import { u as s } from "./BO-Hjil0.js";
import { f as l } from "./emZtalxW.js";
function h(m, o) {
  a(o, true);
  let t = n(o, "value", 15);
  i(async () => {
    if (s().get()) {
      let e = await l(`/auth/v1/template/${o.id}`);
      e.error ? console.error(e.error) : e.text && r(e.text);
    } else {
      let e = document.getElementById(o.id);
      e && r(e.innerHTML);
    }
  });
  function r(e) {
    typeof t() == "boolean" ? t(e === "true") : typeof t() == "string" ? t(e) : t(JSON.parse(e));
  }
  f();
}
export {
  h as T
};
