import{a8 as m,o as d,E as p,at as _,au as h,h as c,k as o,n as b,i as N}from"./runtime.BONl2e88.js";function v(e){var t=document.createElement("template");return t.innerHTML=e,t.content}function r(e,t){var n=p;n.nodes_start===null&&(n.nodes_start=e,n.nodes_end=t)}function g(e,t){var n=(t&_)!==0,f=(t&h)!==0,a,s=!e.startsWith("<!>");return()=>{if(c)return r(o,null),o;a===void 0&&(a=v(s?e:"<!>"+e),n||(a=d(a)));var u=f?document.importNode(a,!0):a.cloneNode(!0);if(n){var l=d(u),i=u.lastChild;r(l,i)}else r(u,u);return u}}function E(e,t,n="svg"){var f=!e.startsWith("<!>"),a=`<${n}>${f?e:"<!>"+e}</${n}>`,s;return()=>{if(c)return r(o,null),o;if(!s){var u=v(a),l=d(u);s=d(l)}var i=s.cloneNode(!0);return r(i,i),i}}function $(e=""){if(!c){var t=m(e+"");return r(t,t),t}var n=o;return n.nodeType!==3&&(n.before(n=m()),b(n)),r(n,n),n}function x(){if(c)return r(o,null),o;var e=document.createDocumentFragment(),t=document.createComment(""),n=m();return e.append(t,n),r(t,n),e}function y(e,t){if(c){p.nodes_end=o,N();return}e!==null&&e.before(t)}export{y as a,r as b,v as c,$ as d,x as e,E as n,g as t};
