import"../chunks/disclose-version.BDr9Qe-U.js";import{p as be,o as ye,a3 as k,a as Te,j as a,a5 as l,$ as Le,g as e,a4 as o,a6 as f,a7 as t,a9 as s,a8 as Q,h as ce}from"../chunks/index-client.C1uYfxHd.js";import{s as d}from"../chunks/render.DlGhCWX2.js";import{i as V}from"../chunks/if.DL6pUnI4.js";import{f as Se,T as ke,B as X,a as Ae,e as Pe,i as De}from"../chunks/ThemeSwitch.B8t8VQsu.js";import{a as i,t as z,b as Y,c as le}from"../chunks/template.BehheIQn.js";import{h as Ee}from"../chunks/svelte-head.76d9LV1R.js";import{p as n}from"../chunks/proxy.bZKuG6wn.js";import{r as ne,f as Ce}from"../chunks/helpers.EoWe9H8J.js";import{L as Re}from"../chunks/LangSelector.Bntt9KZe.js";import{I as je}from"../chunks/Input.Dq3XA-_Z.js";import{f as qe}from"../chunks/pow.BpM6kMYr.js";import{M as Be,C as Me}from"../chunks/ContentCenter.BU-nX73m.js";import{u as Ne}from"../chunks/i18n.svelte.C5UT6hSp.js";import{T as Oe}from"../chunks/Template.DxkxrmTY.js";import{u as Ve}from"../chunks/param.svelte.vEd3x5XR.js";import{P as We}from"../chunks/patterns.DmsbdEr_.js";var _e=z('<div class="desc svelte-hzde4x"> </div> <!> <div><!></div>',1),Fe=z('<div class="desc svelte-hzde4x"><p> </p> <p> </p></div>'),Ge=z('<div class="desc svelte-hzde4x"><p class="declined svelte-hzde4x"> </p> <p> </p></div>'),He=z('<li class="svelte-hzde4x"> </li>'),Ie=z('<div class="desc svelte-hzde4x"> <ul class="svelte-hzde4x"></ul></div> <div class="inline svelte-hzde4x"><!> <!></div>',1),Je=z('<div class="container svelte-hzde4x"><div class="name svelte-hzde4x"><h2> </h2></div> <!> <div class="err svelte-hzde4x"> </div></div>'),Ke=z("<!> <!> <!>",1),Ue=z("<!> <!>",1);function Qe(ue,pe){be(pe,!0);let r=Ne(),Z=f(void 0),w=f(""),W=f(!1),A=f(!1),b=f(8),_=f(void 0),ee=f(!1),re=f(!1),T=f("");ye(async()=>{let v=Ve("code").get()||"";a(T,n(v));let L=await Se("/auth/v1/oidc/sessioninfo");L.body?a(Z,n(L.body)):v?ne(`device?code=${v}`):ne("device")});async function F(v){var P,D,B;if(a(w,""),e(W))return;if(e(T).length<e(b)){a(w,n(r.common.errTooShort));return}if(e(T).length>e(b)){a(w,n(r.common.errTooLong));return}a(A,!0);let L=await qe();if(!L){a(w,"PoW error - please contact your administrator");return}let G={user_code:e(T),pow:L,device_accepted:v},h=await Ae("/auth/v1/oidc/device/verify",G);h.status===200?a(_,n(((D=(P=h.body)==null?void 0:P.scopes)==null?void 0:D.split(" "))||["openid"])):h.status===202?(a(ee,!0),setTimeout(()=>{window.location.replace("/auth/v1/account?v=devices")},2e3)):h.status===204?a(re,!0):h.status===404?a(w,n(r.device.wrongOrExpired)):(console.error(h),a(w,n((B=h.error)==null?void 0:B.message))),a(A,!1)}var ae=Ue();Ee(v=>{l(()=>Le.title=(r==null?void 0:r.device.title)||"Device Authorization")});var se=k(ae);Oe(se,{id:Ce,get value(){return e(b)},set value(v){a(b,n(v))}});var me=o(se,2);Be(me,{children:(v,L)=>{Me(v,{children:(G,h)=>{var P=Ke(),D=k(P);{var B=H=>{var I=Je(),J=t(I),ie=t(J),he=t(ie,!0);s(ie),s(J);var oe=o(J,2);{var ge=S=>{var E=_e(),C=k(E),K=t(C,!0);l(()=>d(K,r.device.desc.replace("{{count}}",e(b).toString()))),s(C);var M=o(C,2),g=ce(()=>e(b).toString()),x=ce(()=>e(b).toString());je(M,{name:"userCode",autocomplete:"off",get label(){return r.device.userCode},get placeholder(){return r.device.userCode},required:!0,get min(){return e(g)},get max(){return e(x)},pattern:We,get value(){return e(T)},set value(c){a(T,n(c))},get isError(){return e(W)},set isError(c){a(W,n(c))}});var $=o(M,2),R=t($);X(R,{onclick:()=>F("pending"),get isLoading(){return e(A)},children:(c,u)=>{Q();var p=Y();l(()=>d(p,r.device.submit)),i(c,p)},$$slots:{default:!0}}),s($),i(S,E)},xe=S=>{var E=le(),C=k(E);{var K=g=>{var x=Fe(),$=t(x),R=t($,!0);s($);var c=o($,2),u=t(c,!0);s(c),s(x),l(()=>{d(R,r.device.isAccepted),d(u,r.device.autoRedirectAccount)}),i(g,x)},M=g=>{var x=le(),$=k(x);{var R=u=>{var p=Ge(),y=t(p),N=t(y,!0);s(y);var j=o(y,2),O=t(j,!0);s(j),s(p),l(()=>{d(N,r.device.isDeclined),d(O,r.device.closeWindow)}),i(u,p)},c=u=>{var p=Ie(),y=k(p),N=t(y),j=o(N);Pe(j,21,()=>e(_),De,(q,U)=>{var m=He(),we=t(m,!0);s(m),l(()=>d(we,e(U))),i(q,m)}),s(j),s(y);var O=o(y,2),ve=t(O);X(ve,{onclick:()=>F("accept"),get isLoading(){return e(A)},children:(q,U)=>{Q();var m=Y();l(()=>d(m,r.device.accept)),i(q,m)},$$slots:{default:!0}});var ze=o(ve,2);X(ze,{level:-1,onclick:()=>F("decline"),get isLoading(){return e(A)},children:(q,U)=>{Q();var m=Y();l(()=>d(m,r.device.decline)),i(q,m)},$$slots:{default:!0}}),s(O),l(()=>d(N,`${r.device.descScopes??""} `)),i(u,p)};V($,u=>{e(re)?u(R):u(c,!1)},!0)}i(g,x)};V(C,g=>{e(ee)?g(K):g(M,!1)},!0)}i(S,E)};V(oe,S=>{e(_)===void 0?S(ge):S(xe,!1)})}var de=o(oe,2),$e=t(de,!0);s(de),s(I),l(()=>{d(he,r.device.title),d($e,e(w))}),i(H,I)};V(D,H=>{e(Z)&&H(B)})}var te=o(D,2);ke(te,{absolute:!0});var fe=o(te,2);Re(fe,{absolute:!0}),i(G,P)},$$slots:{default:!0}})},$$slots:{default:!0}}),i(ue,ae),Te()}export{Qe as component};
