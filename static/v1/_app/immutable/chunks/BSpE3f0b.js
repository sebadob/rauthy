import { d as s, a as t } from "./D8nUqfqE.js";
import { f as e } from "./D-nwkJyM.js";
import { i as n } from "./C3XMhfdI.js";
import { I as p } from "./C0_UZAx3.js";
import { I as l } from "./Dq3vuPfn.js";
function C(a, c) {
  var r = s(), m = e(r);
  {
    var f = (o) => {
      p(o, { color: "hsl(var(--action))" });
    }, i = (o) => {
      l(o, { color: "hsl(var(--error))" });
    };
    n(m, (o) => {
      c.checked ? o(f) : o(i, false);
    });
  }
  t(a, r);
}
export {
  C
};
