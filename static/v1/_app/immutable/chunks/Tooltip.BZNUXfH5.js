import"./disclose-version.BDr9Qe-U.js";import{p as P,v as u,c as b,Q as R,s as S,u as c,r as g,a as U,w as O,t as H,f as Q}from"./runtime.BONl2e88.js";import{d as V,e as W,s as X}from"./render.AqJWcVDb.js";import{i as d}from"./if.DS90EFHs.js";import{h as Z}from"./html.B6hxHEfi.js";import{s as A}from"./snippet.D6O6IDbh.js";import{a as m,t as Y,d as C,e as j}from"./template.B7ldVCvc.js";import{s as E,a as G,f as oo}from"./index.BJiFQ8Di.js";import{p as so}from"./proxy.0BECmVET.js";import{s as to,p as e,b as ao,a as ro}from"./props.BlbrNjWf.js";import{o as io,a as mo}from"./index-client.Bjp_k6xO.js";import{s as eo}from"./spring.DIfzGqN5.js";var fo=Y('<div class="tooltip svelte-op12yu"><!></div>'),no=Y('<div role="none"><!> <!></div>');function po(k,s){P(s,!0);const l=to(),x=()=>ro(c(n),"$coords",l);let q=e(s,"xOffset",3,10),z=e(s,"yOffset",3,10),y=e(s,"text",3,""),w=e(s,"html",3,""),f=O(!1),n=O(void 0),p;io(()=>{ao(u(n,so(eo({x:(window==null?void 0:window.innerWidth)/2,y:window.innerHeight/2},{stiffness:.1,damping:.6}))),"$coords",l)}),mo(()=>{clearTimeout(p)});function _(){clearTimeout(p),u(f,!0)}function $(){p=setTimeout(()=>{u(f,!1)},100)}function B(t){c(n).set({x:t.clientX+q(),y:t.clientY+z()})}var o=no();o.__mouseover=_,o.__mouseout=$,o.__mousemove=B;var h=b(o);A(h,()=>s.children??R);var D=S(h,2);{var F=t=>{var r=fo(),I=b(r);{var J=a=>{var i=C();H(()=>X(i,y())),m(a,i)},K=a=>{var i=j(),L=Q(i);{var M=v=>{var T=j(),N=Q(T);Z(N,w),m(v,T)};d(L,v=>{w()&&v(M)},!0)}m(a,i)};d(I,a=>{y()?a(J):a(K,!1)})}g(r),H(()=>E(r,"style",`top: ${`${x().y}px`??""}; left: ${`${x().x}px`??""}`)),G(7,r,()=>oo,()=>({delay:400,duration:200})),m(t,r)};d(D,t=>{c(f)&&t(F)})}g(o),W("focus",o,_),W("blur",o,$),m(k,o),U()}V(["mouseover","mouseout","mousemove"]);export{po as T};
