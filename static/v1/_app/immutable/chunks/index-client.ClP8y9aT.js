import{K as t,ar as m,F as p,G as a,a6 as v}from"./runtime.BsghBUX9.js";function r(n){throw new Error("https://svelte.dev/e/lifecycle_outside_component")}function f(n){t===null&&r(),m&&t.l!==null?x(t).m.push(n):p(()=>{const e=a(n);if(typeof e=="function")return e})}function d(n){t===null&&r(),f(()=>()=>a(n))}function h(n,e,{bubbles:l=!1,cancelable:u=!1}={}){return new CustomEvent(n,{detail:e,bubbles:l,cancelable:u})}function w(){const n=t;return n===null&&r(),(e,l,u)=>{var s;const o=(s=n.s.$$events)==null?void 0:s[e];if(o){const i=v(o)?o.slice():[o],c=h(e,l,u);for(const b of i)b.call(n.x,c);return!c.defaultPrevented}return!0}}function x(n){var e=n.l;return e.u??(e.u={a:[],b:[],m:[]})}export{d as a,w as c,f as o};
