import"./disclose-version.BDr9Qe-U.js";import{t as z,p as W,a as Y,h as n,g as e,s as B,c as m,i as R,r as u}from"./runtime.BsghBUX9.js";import{a as S,n as K,t as j}from"./template.jENAUvUX.js";import{i as F}from"./if.6Ov_EDLa.js";import{p as k}from"./proxy.YH42NwLs.js";import{p as v}from"./props.CslUVmW_.js";import{d as Q,e as H,s as G}from"./render.CTp9lacC.js";import{e as V,i as X}from"./each.URXtkKAp.js";import{e as ee,f as re,O as oe}from"./OptionSelect.Cn2sqgid.js";import{o as te}from"./index-client.ClP8y9aT.js";import{r as q}from"./legacy-client.DY3VEcT0.js";import{I as ne,a as ve}from"./IconBarsArrowUp.CwQnwfB4.js";import{T as se,I as ie}from"./Tooltip.D8Tbt6Nb.js";import{s as T,d as ce,t as de}from"./index.G5lyZbma.js";import{b as fe}from"./input.D13BG9DA.js";import{g as pe}from"./Button.CHwcFOJ-.js";import{U as ue}from"./helpers.CBl6QeM5.js";var he=K('<svg fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z"></path></svg>');function me(I,s){let f=v(s,"opacity",3,.9),g=v(s,"width",3,24);var o=he();T(o,"stroke-width",2),z(()=>{T(o,"width",g()),T(o,"opacity",f())}),S(I,o)}var be=K('<svg fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" d="M12 9.75L14.25 12m0 0l2.25 2.25M14.25 12l2.25-2.25M14.25 12L12 14.25m-2.58 4.92l-6.375-6.375a1.125 1.125 0 010-1.59L9.42 4.83c.211-.211.498-.33.796-.33H19.5a2.25 2.25 0 012.25 2.25v10.5a2.25 2.25 0 01-2.25 2.25h-9.284c-.298 0-.585-.119-.796-.33z"></path></svg>');function ke(I,s){let f=v(s,"opacity",3,.9),g=v(s,"width",3,24);var o=be();T(o,"stroke-width",2),z(()=>{T(o,"width",g()),T(o,"opacity",f())}),S(I,o)}var ge=j("<option> </option>"),ye=j('<select class="opts font-label svelte-9iykj"></select>'),we=j('<div class="container svelte-9iykj"><!> <div class="inputBar svelte-9iykj"><input class="input svelte-9iykj" type="text" placeholder="Search" autocomplete="off"> <div class="magnify svelte-9iykj"><!></div></div> <div class="backWrap svelte-9iykj"><div role="button" tabindex="0" class="back svelte-9iykj"><!></div></div></div>');function ae(I,s){W(s,!0);let f=v(s,"items",27,()=>k([])),g=v(s,"resItems",15),o=v(s,"options",19,()=>[]),C=v(s,"useServerSideIdx",11,""),c=v(s,"isSearchFiltered",15,!1),a=v(s,"search",15,""),p=R(""),y;te(()=>{o().length>0&&(n(p,k(o()[0].label)),b())});function b(){for(let t of o())if(t.label===e(p)){y=t.callback;break}y||console.error("Could not find a valid callback function in search options for label "+e(p))}function w(){a("")}function _(){if(a().length<2){g(f()),c(!1);return}g([...f().filter(t=>o().length>0?y(t,a()):t.toLowerCase().includes(a())||t===a())]),c(!0)}async function D(){if(a().length<ue){g(f()),c(!1);return}c(!0);const t=e(p).replaceAll("-","").replaceAll(" ","").toLowerCase();let i=await re(C(),t,a());i.ok?g(await i.json()):console.error(i)}q(()=>{e(p)&&b()}),q(()=>{a()?C()?D():_():(g(f()),c(!1))});var O=we(),$=m(O);{var A=t=>{se(t,{text:"Search by",yOffset:-30,children:(i,Z)=>{var P=ye();V(P,21,o,X,(N,U)=>{var E=ge(),J={},le=m(E,!0);u(E),z(()=>{J!==(J=e(U).label)&&(E.value=(E.__value=e(U).label)==null?"":e(U).label),G(le,e(U).label)}),S(N,E)}),u(P),ee(P,()=>e(p),N=>n(p,N)),S(i,P)},$$slots:{default:!0}})};F($,t=>{o().length>1&&t(A)})}var d=B($,2),l=m(d);ce(l),z(()=>T(l,"name",pe()));var x=B(l,2),M=m(x);me(M,{width:20}),u(x),u(d);var L=B(d,2),h=m(L);h.__click=w;var r=m(h);ke(r,{}),u(h),u(L),u(O),fe(l,a),H("keypress",h,w),S(I,O),Y()}Q(["click"]);var Se=j("<option> </option>"),Ie=j('<select class="opts font-label svelte-t7wy0q"></select>'),xe=j('<div role="button" tabindex="0" class="icon svelte-t7wy0q"><!></div>'),je=j('<div class="container svelte-t7wy0q"><!> <!></div>');function _e(I,s){W(s,!0);let f=v(s,"items",19,()=>[]),g=v(s,"resItems",15),o=v(s,"options",19,()=>[]),C=v(s,"firstDirReverse",3,!1),c=R(""),a=R(1),p;te(()=>{o().length>0&&(n(c,k(o()[0].label)),y()),C()&&w(),b()});function y(){for(let d of o())if(d.label===e(c)){p=d.callback;break}p||console.error("Could not find a valid callback function in order options for label "+e(c))}function b(){if(p){let d=[...f()];d.sort((l,x)=>p(l,x)*e(a)),g([...d])}}function w(){n(a,e(a)*-1),b()}q(()=>{f()&&b()}),q(()=>{e(c)&&(y(),b())});var _=je(),D=m(_);{var O=d=>{se(d,{text:"Order by",yOffset:-30,children:(l,x)=>{var M=Ie();V(M,21,o,X,(L,h)=>{var r=Se(),t={},i=m(r,!0);u(r),z(()=>{t!==(t=e(h).label)&&(r.value=(r.__value=e(h).label)==null?"":e(h).label),G(i,e(h).label)}),S(L,r)}),u(M),ee(M,()=>e(c),L=>n(c,L)),S(l,M)},$$slots:{default:!0}})};F(D,d=>{o().length>1&&d(O)})}var $=B(D,2);{var A=d=>{var l=xe();l.__click=w;var x=m(l);{var M=h=>{ne(h,{})},L=h=>{ve(h,{})};F(x,h=>{e(a)===1?h(M):h(L,!1)})}u(l),H("keypress",l,w),S(d,l)};F($,d=>{o().length>0&&d(A)})}u(_),S(I,_),Y()}Q(["click"]);var Oe=j('<div class="divider svelte-16rtgja"></div>'),Le=j('<div class="container svelte-16rtgja"><div><!></div> <!> <div><!></div></div>');function Be(I,s){W(s,!0);let f=v(s,"items",31,()=>k([])),g=v(s,"resItems",15),o=v(s,"searchOptions",27,()=>k([])),C=v(s,"orderOptions",27,()=>k([])),c=v(s,"firstDirReverse",3,!1),a=v(s,"useServerSideIdx",15,""),p=v(s,"isSearchFiltered",15,!1),y=v(s,"search",15,""),b=R(k([]));var w=Le(),_=m(w),D=m(_);_e(D,{get items(){return e(b)},get options(){return C()},get firstDirReverse(){return c()},get resItems(){return g()},set resItems(l){g(l)}}),u(_);var O=B(_,2);{var $=l=>{var x=Oe();S(l,x)};F(O,l=>{C().length>0&&l($)})}var A=B(O,2),d=m(A);ae(d,{get options(){return o()},get items(){return f()},set items(l){f(l)},get resItems(){return e(b)},set resItems(l){n(b,k(l))},get useServerSideIdx(){return a()},set useServerSideIdx(l){a(l)},get isSearchFiltered(){return p()},set isSearchFiltered(l){p(l)},get search(){return y()},set search(l){y(l)}}),u(A),u(w),S(I,w),Y()}var Ce=(I,s)=>n(s,e(s)-1),De=j('<div role="button" tabindex="0" class="icon iconLeft svelte-9iw34a"><!></div>'),Ae=(I,s,f)=>n(s,k(e(f))),Me=j('<div role="button" tabindex="0" class="link noselect svelte-9iw34a"> </div>'),Re=(I,s)=>n(s,e(s)+1),$e=j('<div role="button" tabindex="0" class="icon iconRight svelte-9iw34a"><!></div>'),ze=j('<div class="container svelte-9iw34a"><!> <div class="links svelte-9iw34a"></div> <!> <div class="chunkSize noselect svelte-9iw34a">Entries <!></div> <div class="font-label total svelte-9iw34a"> </div></div>');function Fe(I,s){W(s,!0);let f=v(s,"items",27,()=>k([])),g=v(s,"resItems",31,()=>k([]));const o=[5,7,10,15,20,30,50,100],C=16;let c=R(k([])),a=R(1),p=5,y=R(15),b=R(!1),w=R(!1),_=R(k([]));q(()=>{e(y)&&n(a,1)}),q(()=>{if(e(y)&&f().length>0){let r=[];for(let t=0;t<f().length;t+=e(y)){const i=f().slice(t,t+e(y));r.push(i)}n(c,k(r)),g(e(c)[e(a)-1])}}),q(()=>{if(e(a)){let r=[],t=Math.floor(p/2);if(e(c).length<=p){for(let i=1;i<=e(c).length;i++)r.push(i);n(b,!1),n(w,!1)}else if(e(a)<=t){for(let i=1;i<=p;i++)r.push(i);n(b,!1),n(w,!0)}else if(e(a)>e(c).length-t-1){for(let i=e(c).length-p;i<=e(c).length-1;i++)r.push(i+1);n(b,!0),n(w,!1)}else{for(let i=e(a)-t;i<e(a)-t+p;i++)r.push(i);n(b,!0),n(w,!0)}n(_,k(r))}});var D=ze(),O=m(D);{var $=r=>{var t=De();t.__click=[Ce,a];var i=m(t);ie(i,{width:C}),u(t),H("keypress",t,()=>n(a,e(a)-1)),S(r,t)};F(O,r=>{e(b)&&r($)})}var A=B(O,2);V(A,21,()=>e(_),X,(r,t)=>{var i=Me();i.__click=[Ae,a,t];var Z=m(i,!0);u(i),z(()=>{de(i,"selected",e(a)===e(t)),G(Z,e(t))}),H("keypress",i,()=>n(a,k(e(t)))),S(r,i)}),u(A);var d=B(A,2);{var l=r=>{var t=$e();t.__click=[Re,a];var i=m(t);ie(i,{width:C}),u(t),H("keypress",t,()=>n(a,e(a)+1)),S(r,t)};F(d,r=>{e(w)&&r(l)})}var x=B(d,2),M=B(m(x));oe(M,{options:o,width:"50px",get value(){return e(y)},set value(r){n(y,k(r))}}),u(x);var L=B(x,2),h=m(L);u(L),u(D),z(()=>G(h,`Total: ${f().length??""}`)),S(I,D),Y()}Q(["click"]);export{Be as O,Fe as P,ae as S};
