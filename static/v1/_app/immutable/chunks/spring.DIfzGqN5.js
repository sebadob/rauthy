var L=t=>{throw TypeError(t)};var B=(t,i,s)=>i.has(t)||L("Cannot "+s);var e=(t,i,s)=>(B(t,i,"read from private field"),s?s.call(t):i.get(t)),h=(t,i,s)=>i.has(t)?L("Cannot add the same private member more than once"):i instanceof WeakSet?i.add(t):i.set(t,s),a=(t,i,s,r)=>(B(t,i,"write to private field"),r?r.call(t,s):i.set(t,s),s),N=(t,i,s)=>(B(t,i,"access private method"),s);var d,g,b,f,c,y,j,w,E,p,A,G,U;import{w as X}from"./index.Di-WNyKL.js";import{r as F,l as R}from"./index.BJiFQ8Di.js";import{a1 as P,b as Y,v,aq as Z,Q as ss,u as D}from"./runtime.BONl2e88.js";function S(t){return Object.prototype.toString.call(t)==="[object Date]"}function $(t,i,s,r){if(typeof s=="number"||S(s)){const o=r-s,n=(s-i)/(t.dt||1/60),u=t.opts.stiffness*o,m=t.opts.damping*n,q=(u-m)*t.inv_mass,l=(n+q)*t.dt;return Math.abs(l)<t.opts.precision&&Math.abs(o)<t.opts.precision?r:(t.settled=!1,S(s)?new Date(s.getTime()+l):s+l)}else{if(Array.isArray(s))return s.map((o,n)=>$(t,i[n],s[n],r[n]));if(typeof s=="object"){const o={};for(const n in s)o[n]=$(t,i[n],s[n],r[n]);return o}else throw new Error(`Cannot spring ${typeof s} values`)}}function ts(t,i={}){const s=X(t),{stiffness:r=.15,damping:o=.8,precision:n=.01}=i;let u,m,q,l=t,x=t,C=1,J=0,O=!1;function K(T,M={}){x=T;const V=q={};return t==null||M.hard||Q.stiffness>=1&&Q.damping>=1?(O=!0,u=F.now(),l=T,s.set(t=x),Promise.resolve()):(M.soft&&(J=1/((M.soft===!0?.5:+M.soft)*60),C=0),m||(u=F.now(),O=!1,m=R(_=>{if(O)return O=!1,m=null,!1;C=Math.min(C+J,1);const z={inv_mass:C,opts:Q,settled:!0,dt:(_-u)*60/1e3},W=$(z,l,t,x);return u=_,l=t,s.set(t=W),z.settled&&(m=null),!z.settled})),new Promise(_=>{m.promise.then(()=>{V===q&&_()})}))}const Q={set:K,update:(T,M)=>K(T(x,t),M),subscribe:s.subscribe,stiffness:r,damping:o,precision:n};return Q}const I=class I{constructor(i,s={}){h(this,G);h(this,d,P(.15));h(this,g,P(.8));h(this,b,P(.01));h(this,f,P(void 0));h(this,c,P(void 0));h(this,y);h(this,j,0);h(this,w,1);h(this,E,0);h(this,p,null);h(this,A,null);e(this,f).v=e(this,c).v=i,typeof s.stiffness=="number"&&(e(this,d).v=k(s.stiffness,0,1)),typeof s.damping=="number"&&(e(this,g).v=k(s.damping,0,1)),typeof s.precision=="number"&&(e(this,b).v=s.precision)}static of(i,s){const r=new I(i(),s);return Y(()=>{r.set(i())}),r}set(i,s){var o,n;if((o=e(this,A))==null||o.reject(new Error("Aborted")),(s==null?void 0:s.instant)||e(this,f).v===void 0)return(n=e(this,p))==null||n.abort(),a(this,p,null),v(e(this,f),v(e(this,c),i)),a(this,y,i),Promise.resolve();s!=null&&s.preserveMomentum&&(a(this,w,0),a(this,E,s.preserveMomentum));var r=a(this,A,Z());return r.promise.catch(ss),N(this,G,U).call(this,i).then(()=>{r===e(this,A)&&r.resolve(void 0)}),r.promise}get current(){return D(e(this,f))}get damping(){return D(e(this,g))}set damping(i){v(e(this,g),k(i,0,1))}get precision(){return D(e(this,b))}set precision(i){v(e(this,b),i)}get stiffness(){return D(e(this,d))}set stiffness(i){v(e(this,d),k(i,0,1))}get target(){return D(e(this,c))}set target(i){this.set(i)}};d=new WeakMap,g=new WeakMap,b=new WeakMap,f=new WeakMap,c=new WeakMap,y=new WeakMap,j=new WeakMap,w=new WeakMap,E=new WeakMap,p=new WeakMap,A=new WeakMap,G=new WeakSet,U=function(i){var r;if(v(e(this,c),i),(r=e(this,f)).v??(r.v=i),e(this,y)??a(this,y,e(this,f).v),!e(this,p)){a(this,j,F.now());var s=1e3/(e(this,E)*60);e(this,p)??a(this,p,R(o=>{a(this,w,Math.min(e(this,w)+s,1));const n={inv_mass:e(this,w),opts:{stiffness:e(this,d).v,damping:e(this,g).v,precision:e(this,b).v},settled:!0,dt:(o-e(this,j))*60/1e3};var u=$(n,e(this,y),e(this,f).v,e(this,c).v);return a(this,y,e(this,f).v),a(this,j,o),v(e(this,f),u),n.settled&&a(this,p,null),!n.settled}))}return e(this,p).promise};let H=I;function k(t,i,s){return Math.max(i,Math.min(s,t))}export{H as S,ts as s};
