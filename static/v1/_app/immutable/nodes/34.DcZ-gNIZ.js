import"../chunks/disclose-version.BDr9Qe-U.js";import{p as S,f as q,a as U,t as c,ap as V,u as i,v as z,s as a,w as B,c as t,r,aq as W}from"../chunks/index-client.ChFmlJin.js";import{s}from"../chunks/render.D821es-0.js";import{a as g,t as E,d as X}from"../chunks/template.DOaMAhd6.js";import{h as Y}from"../chunks/svelte-head.BHag5tza.js";import{p as j}from"../chunks/proxy.DVTQVrWW.js";import{T as Z,L as _}from"../chunks/LangSelector.6yM5iW85.js";import{B as F}from"../chunks/Button.DAsYf-wl.js";import{M as N}from"../chunks/Main.CZkA2qJN.js";import{C as O}from"../chunks/ContentCenter.Cb3YBr_D.js";import{u as aa}from"../chunks/i18n.svelte.CKzx-KOG.js";import{T as k}from"../chunks/Template.CmI5wdc1.js";import{w as ta,x as ra}from"../chunks/helpers.BweJi6s8.js";var sa=E('<div class="container svelte-1g19pwu"><h1> </h1> <p class="svelte-1g19pwu"> <br> <b> </b> <b> </b></p> <p class="svelte-1g19pwu"> </p> <div class="btn svelte-1g19pwu"><!></div></div> <!> <!>',1),ea=E("<!> <!> <!>",1);function oa(y,A){S(A,!0);let e=aa(),l=B("old@mail.org"),m=B("new@mail.org");var d=ea();Y(o=>{c(()=>V.title=e.emailChange.title||"E-Mail Change Confirm")});var h=q(d);k(h,{id:ta,get value(){return i(l)},set value(o){z(l,j(o))}});var b=a(h,2);k(b,{id:ra,get value(){return i(m)},set value(o){z(m,j(o))}});var D=a(b,2);N(D,{children:(o,ia)=>{O(o,{children:(G,la)=>{var w=sa(),n=q(w),p=t(n),H=t(p,!0);r(p);var v=a(p,2),C=t(v),u=a(C,3),I=t(u,!0);r(u);var $=a(u),x=a($),J=t(x,!0);r(x),r(v);var f=a(v,2),K=t(f,!0);r(f);var L=a(f,2),P=t(L);F(P,{onclick:()=>window.location.replace("/auth/v1/account"),children:(R,ma)=>{W();var T=X();c(()=>s(T,e.authorize.login)),g(R,T)},$$slots:{default:!0}}),r(L),r(n);var M=a(n,2);Z(M,{absolute:!0});var Q=a(M,2);_(Q,{absolute:!0}),c(()=>{s(H,e.emailChange.title),s(C,`${e.emailChange.textChanged??""}:`),s(I,i(l)),s($,` ${e.emailChange.to??""} `),s(J,i(m)),s(K,e.emailChange.textLogin)}),g(G,w)},$$slots:{default:!0}})},$$slots:{default:!0}}),g(y,d),U()}export{oa as component};
