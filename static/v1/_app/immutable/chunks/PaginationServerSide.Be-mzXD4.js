import"./disclose-version.BDr9Qe-U.js";import{p as ee,o as f,$ as j,l as e,ap as te,m as r,a6 as ae,c as n,r as l,t as b,s as g,a as ie,ab as R}from"./index-client.Ds4XgiuL.js";import{s as q}from"./render.Bq2-eTRf.js";import{i as se}from"./if.Du7BDhAX.js";import{a as h,t as x}from"./template.DXIK6jcH.js";import{s as $}from"./class.Ik0ma2JO.js";import{p as c}from"./proxy.Bsf9UJbk.js";import{I as B}from"./IconChevronRight.VeFLbKE9.js";import{u as oe}from"./i18n.svelte.CB3BpIrt.js";import{B as A}from"./Button.C_IpvWJY.js";import{O as re}from"./Options.DccTdWl1.js";const E=20;var ne=x('<div class="iconLeft svelte-1eyqa5j"><!></div>'),le=x('<div class="iconRight svelte-1eyqa5j"><!></div>'),ce=x('<div class="font-label total svelte-1eyqa5j"> </div>'),de=x('<div class="container svelte-1eyqa5j"><div class="flex"><!> <ul class="svelte-1eyqa5j"><li class="svelte-1eyqa5j"> </li></ul> <!></div> <div class="flex gap-10"><div class="flex gap-05 chunkSize noselect"><div> </div> <div><!></div></div> <!></div></div>');function ve(G,s){ee(s,!0);const J=[5,10,20,30,50,100];let p=oe();const z="1rem";let u=f(void 0),d=f(c(Number.parseInt(s.firstFetchHeaders.get("x-page-size")||E.toString()))),P=j(()=>e(d)),w=f(c(Number.parseInt(s.firstFetchHeaders.get("x-page-count")||"1"))),m=f(c(s.firstFetchHeaders.get("x-continuation-token"))),o=f(1),C=R(()=>e(o)>=e(w));te(()=>{if(s.idxTotalCount){let t=s.firstFetchHeaders.get(s.idxTotalCount);t&&r(u,c(Number.parseInt(t)))}}),ae(()=>{K()});async function K(){if(e(d)!==P){P=e(d),r(m,void 0);let[t,a]=await s.sspFetch(`page_size=${e(d)}`);y(t,a),r(o,1)}}function T(t){let a=`page_size=${j(()=>e(d))}`;if(t){if(e(o)===2)return a;a+=`&backwards=${t}&offset=${s.itemsLength-1}`}return e(m)&&(a+=`&continuation_token=${j(()=>e(m))}`),a}async function M(){if(e(o)>1){let[t,a]=await s.sspFetch(T(!0));y(t,a),r(o,e(o)-1)}}async function O(){if(e(o)<e(w)){let[t,a]=await s.sspFetch(T(!1));y(t,a),r(o,e(o)+1)}}function y(t,a){if(t===206){let i=a.get("x-page-size");if(!i){console.error("Did not receive x-page-size with SSP");return}r(d,c(Number.parseInt(i))),r(w,c(Number.parseInt(a.get("x-page-count")||"1"))),r(m,c(a.get("x-continuation-token")));let v=s.idxTotalCount?a.get(s.idxTotalCount):void 0;v?r(u,c(Number.parseInt(v))):r(u,void 0)}else console.error("Received non 206 status with SSP")}var k=de(),F=n(k),H=n(F),Q=R(()=>e(o)<2);A(H,{onclick:M,invisible:!0,get isDisabled(){return e(Q)},children:(t,a)=>{var i=ne(),v=n(i);B(v,{width:z}),l(i),b(()=>{$(i,"aria-label",p.pagination.gotoPagePrev),$(i,"data-disabled",e(o)===1)}),h(t,i)},$$slots:{default:!0}});var I=g(H,2),D=n(I),U=n(D,!0);l(D),l(I);var V=g(I,2);A(V,{onclick:O,invisible:!0,get isDisabled(){return e(C)},children:(t,a)=>{var i=le(),v=n(i);B(v,{width:z}),l(i),b(()=>{$(i,"aria-label",p.pagination.gotoPageNext),$(i,"data-disabled",e(C))}),h(t,i)},$$slots:{default:!0}}),l(F);var L=g(F,2),N=n(L),S=n(N),W=n(S,!0);l(S);var _=g(S,2),X=n(_);re(X,{get ariaLabel(){return p.pagination.showCount},options:J,offsetTop:"-14rem",borderless:!0,get value(){return e(d)},set value(t){r(d,c(t))}}),l(_),l(N);var Z=g(N,2);{var Y=t=>{var a=ce(),i=n(a);l(a),b(()=>q(i,`${p.pagination.total??""}: ${e(u)??""}`)),h(t,a)};se(Z,t=>{e(u)&&t(Y)})}l(L),l(k),b(()=>{q(U,e(o)),q(W,p.pagination.entries)}),h(G,k),ie()}export{ve as P,E as a};
