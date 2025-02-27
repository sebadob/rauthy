import { O as n, w as d, h as o } from "./D-nwkJyM.js";
import { a as s } from "./DmLjbmU6.js";
function m(e, a, c = a) {
  s(e, "input", (l) => {
    var r = l ? e.defaultValue : e.value;
    if (r = f(e) ? v(r) : r, c(r), r !== (r = a())) {
      var _ = e.selectionStart, h = e.selectionEnd;
      e.value = r ?? "", h !== null && (e.selectionStart = _, e.selectionEnd = Math.min(h, e.value.length));
    }
  }), (o && e.defaultValue !== e.value || n(a) == null && e.value) && c(f(e) ? v(e.value) : e.value), d(() => {
    var l = a();
    f(e) && l === v(e.value) || e.type === "date" && !l && !e.value || l !== e.value && (e.value = l ?? "");
  });
}
function b(e, a, c = a) {
  s(e, "change", (l) => {
    var r = l ? e.defaultChecked : e.checked;
    c(r);
  }), (o && e.defaultChecked !== e.checked || n(a) == null) && c(e.checked), d(() => {
    var l = a();
    e.checked = !!l;
  });
}
function f(e) {
  var a = e.type;
  return a === "number" || a === "range";
}
function v(e) {
  return e === "" ? null : +e;
}
function y(e, a, c = a) {
  s(e, "change", () => {
    c(e.files);
  }), o && e.files && c(e.files), d(() => {
    e.files = a();
  });
}
export {
  b as a,
  m as b,
  y as c
};
