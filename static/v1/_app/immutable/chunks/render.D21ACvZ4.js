import{X as V,x as p,ar as x,ay as W,aI as L,aj as E,aB as A,v as M,ac as q,l as R,T as U,aq as D,w as Z,aJ as Y,aK as F,aH as j,B as w,A as k,I as m,aL as C,aM as N,y as ee,aA as te,aN as ne,aO as oe,au as ae,aP as re,F as se,p as ue,a as ce,c as ie}from"./index-client.LkyNpeLM.js";import{d as le}from"./disclose-version.C0dpEWi_.js";function de(e){return e.endsWith("capture")&&e!=="gotpointercapture"&&e!=="lostpointercapture"}const fe=["beforeinput","click","change","dblclick","contextmenu","focusin","focusout","input","keydown","keyup","mousedown","mousemove","mouseout","mouseover","mouseup","pointerdown","pointermove","pointerout","pointerover","pointerup","touchend","touchmove","touchstart"];function ve(e){return fe.includes(e)}const pe={formnovalidate:"formNoValidate",ismap:"isMap",nomodule:"noModule",playsinline:"playsInline",readonly:"readOnly",defaultvalue:"defaultValue",defaultchecked:"defaultChecked",srcobject:"srcObject"};function me(e){return e=e.toLowerCase(),pe[e]??e}const he=["touchstart","touchmove"];function _e(e){return he.includes(e)}function we(e,t){if(t){const n=document.body;e.autofocus=!0,V(()=>{document.activeElement===n&&e.focus()})}}function ye(e){p&&x(e)!==null&&W(e)}let H=!1;function J(){H||(H=!0,document.addEventListener("reset",e=>{Promise.resolve().then(()=>{var t;if(!e.defaultPrevented)for(const n of e.target.elements)(t=n.__on_r)==null||t.call(n)})},{capture:!0}))}function ge(e,t,n,a=!0){a&&n();for(var r of t)e.addEventListener(r,n);q(()=>{for(var o of t)e.removeEventListener(o,n)})}function O(e){var t=A,n=M;L(null),E(null);try{return e()}finally{L(t),E(n)}}function be(e,t,n,a=n){e.addEventListener(t,()=>O(n));const r=e.__on_r;r?e.__on_r=()=>{r(),a(!0)}:e.__on_r=()=>a(!0),J()}const K=new Set,P=new Set;function X(e,t,n,a){function r(o){if(a.capture||g.call(t,o),!o.cancelBubble)return O(()=>n.call(this,o))}return e.startsWith("pointer")||e.startsWith("touch")||e==="wheel"?V(()=>{t.addEventListener(e,r,a)}):t.addEventListener(e,r,a),r}function Le(e,t,n,a,r){var o={capture:a,passive:r},i=X(e,t,n,o);(t===document.body||t===window||t===document)&&q(()=>{t.removeEventListener(e,i,o)})}function Ee(e){for(var t=0;t<e.length;t++)K.add(e[t]);for(var n of P)n(e)}function g(e){var I;var t=this,n=t.ownerDocument,a=e.type,r=((I=e.composedPath)==null?void 0:I.call(e))||[],o=r[0]||e.target,i=0,h=e.__root;if(h){var f=r.indexOf(h);if(f!==-1&&(t===document||t===window)){e.__root=t;return}var _=r.indexOf(t);if(_===-1)return;f<=_&&(i=f)}if(o=r[i]||e.target,o!==t){R(e,"currentTarget",{configurable:!0,get(){return o||n}});var T=A,l=M;L(null),E(null);try{for(var s,u=[];o!==null;){var d=o.assignedSlot||o.parentNode||o.host||null;try{var v=o["__"+a];if(v!==void 0&&!o.disabled)if(U(v)){var[G,...Q]=v;G.apply(o,[e,...Q])}else v.call(o,e)}catch(b){s?u.push(b):s=b}if(e.cancelBubble||d===t||d===null)break;o=d}if(s){for(let b of u)queueMicrotask(()=>{throw b});throw s}}finally{e.__root=t,delete e.currentTarget,L(T),E(l)}}}let c;function ke(){c=void 0}function Te(e){let t=null,n=p;var a;if(p){for(t=m,c===void 0&&(c=x(document.head));c!==null&&(c.nodeType!==8||c.data!==F);)c=j(c);c===null?w(!1):c=k(j(c))}p||(a=document.head.appendChild(D()));try{Z(()=>e(a),Y)}finally{n&&(w(!0),c=m,k(t))}}let S=!0;function xe(e,t){var n=t==null?"":typeof t=="object"?t+"":t;n!==(e.__t??(e.__t=e.nodeValue))&&(e.__t=n,e.nodeValue=n==null?"":n+"")}function $(e,t){return z(e,t)}function Me(e,t){C(),t.intro=t.intro??!1;const n=t.target,a=p,r=m;try{for(var o=x(n);o&&(o.nodeType!==8||o.data!==F);)o=j(o);if(!o)throw N;w(!0),k(o),ee();const i=z(e,{...t,anchor:o});if(m===null||m.nodeType!==8||m.data!==te)throw ne(),N;return w(!1),i}catch(i){if(i===N)return t.recover===!1&&oe(),C(),W(n),w(!1),$(e,t);throw i}finally{w(a),k(r),ke()}}const y=new Map;function z(e,{target:t,anchor:n,props:a={},events:r,context:o,intro:i=!0}){C();var h=new Set,f=l=>{for(var s=0;s<l.length;s++){var u=l[s];if(!h.has(u)){h.add(u);var d=_e(u);t.addEventListener(u,g,{passive:d});var v=y.get(u);v===void 0?(document.addEventListener(u,g,{passive:d}),y.set(u,1)):y.set(u,v+1)}}};f(ae(K)),P.add(f);var _=void 0,T=re(()=>{var l=n??t.appendChild(D());return se(()=>{if(o){ue({});var s=ie;s.c=o}r&&(a.$$events=r),p&&le(l,null),S=i,_=e(l,a)||{},S=!0,p&&(M.nodes_end=m),o&&ce()}),()=>{var d;for(var s of h){t.removeEventListener(s,g);var u=y.get(s);--u===0?(document.removeEventListener(s,g),y.delete(s)):y.set(s,u)}P.delete(f),l!==n&&((d=l.parentNode)==null||d.removeChild(l))}});return B.set(_,T),_}let B=new WeakMap;function je(e,t){const n=B.get(e);return n?(B.delete(e),n(t)):Promise.resolve()}export{Me as a,J as b,X as c,Ee as d,Le as e,we as f,ve as g,Te as h,de as i,S as j,ge as k,be as l,$ as m,me as n,ye as r,xe as s,je as u,O as w};