import{t as D,a as x,b as F}from"../chunks/disclose-version.C0dpEWi_.js";import{p as G,o as H,a as I,j as y,a1 as v,$ as J,_ as K,a0 as a,Z as B,a2 as s,a4 as t,a3 as P,g as E}from"../chunks/index-client.LkyNpeLM.js";import{h as Q,s as e}from"../chunks/render.D21ACvZ4.js";import{p as M}from"../chunks/proxy.bVSoDcUp.js";import{L as R}from"../chunks/LangSelector.PrFnGVsI.js";import{B as S}from"../chunks/Button.BDI_bIrm.js";import{M as T,C as U}from"../chunks/ContentCenter.BsVUI9il.js";import{u as V}from"../chunks/i18n.svelte.DPkpATkG.js";var W=F('<div class="container svelte-1g19pwu"><h1> </h1> <p class="svelte-1g19pwu"> <br> <b> </b> <b> </b></p> <p class="svelte-1g19pwu"> </p> <div class="btn svelte-1g19pwu"><!></div></div> <!>',1);function X(j,k){G(k,!0);let o=V(),g=B("old@mail.org"),d=B("new@mail.org");H(async()=>{const l=document.getElementsByName("rauthy-data")[0].id,i=[];l.split(",").forEach(r=>i.push(r)),y(g,M(i[0])),y(d,M(i[1]))}),Q(l=>{v(()=>J.title=o.emailChange.title||"E-Mail Change Confirm")}),T(j,{children:(l,i)=>{U(l,{children:(r,O)=>{var u=W(),n=K(u),m=s(n),z=s(m,!0);t(m);var p=a(m,2),f=s(p),c=a(f,3),L=s(c,!0);t(c);var $=a(c),C=a($),N=s(C,!0);t(C),t(p);var h=a(p,2),Z=s(h,!0);t(h);var b=a(h,2),_=s(b);S(_,{$$events:{click:()=>window.location.replace("/auth/v1/account")},children:(A,Y)=>{P();var w=D();v(()=>e(w,o.authorize.login)),x(A,w)},$$slots:{default:!0}}),t(b),t(n);var q=a(n,2);R(q,{absolute:!0}),v(()=>{e(z,o.emailChange.title),e(f,`${o.emailChange.textChanged??""}:`),e(L,E(g)),e($,` ${o.emailChange.to??""} `),e(N,E(d)),e(Z,o.emailChange.textLogin)}),x(r,u)},$$slots:{default:!0}})},$$slots:{default:!0}}),I()}export{X as component};
